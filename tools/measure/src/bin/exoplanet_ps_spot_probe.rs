use std::collections::HashMap;
use std::process::Command;

const UA: &str = "omegaflow-measure-exoplanet-ps-spot/1.0";
const TAP_SYNC: &str = "https://exoplanetarchive.ipac.caltech.edu/TAP/sync";
const TAP_QUERY: &str =
    "SELECT pl_name,discoverymethod,pl_rade,pl_bmasse,pl_orbper,pl_orbeccen,pl_dens,pl_eqt,st_met,st_teff FROM ps WHERE default_flag=1 AND pl_name IS NOT NULL ORDER BY pl_name";
const NAME_COL: &str = "pl_name";
const METHOD_COL: &str = "discoverymethod";
const NUMERIC: [(&str, &str); 8] = [
    ("pl_rade", "R"),
    ("pl_bmasse", "M"),
    ("pl_orbper", "P"),
    ("pl_orbeccen", "e"),
    ("pl_dens", "dens"),
    ("pl_eqt", "Teq"),
    ("st_met", "met"),
    ("st_teff", "T*"),
];
const DEFAULT_NAMES: [&str; 10] = [
    "51 Peg b",
    "Epsilon Eridani b",
    "Proxima Centauri b",
    "HR 8799 b",
    "Kepler-452 b",
    "HD 20782 b",
    "K2-18 b",
    "PSR B1257+12 b",
    "GJ 1214 b",
    "HD 189733 b",
];

struct PlanetSpot {
    name: String,
    method: Option<String>,
    numeric: [Option<f64>; 8],
}

fn fetch_csv() -> Result<String, String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("300")
        .arg("-A")
        .arg(UA)
        .arg("-G")
        .arg(TAP_SYNC)
        .arg("--data-urlencode")
        .arg(format!("query={TAP_QUERY}"))
        .arg("--data-urlencode")
        .arg("format=csv")
        .arg("-w")
        .arg("\n%{http_code}");
    let out = cmd.output().map_err(|e| format!("curl: {e}"))?;
    let stdout = out.stdout;
    if stdout.is_empty() {
        return Err("curl without bytes".to_string());
    }
    let idx = stdout
        .iter()
        .rposition(|&b| b == b'\n')
        .ok_or_else(|| "curl reply without newline".to_string())?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    let body = String::from_utf8_lossy(&stdout[..idx]).to_string();
    if code != "200" {
        return Err(format!("TAP HTTP {code}"));
    }
    Ok(body)
}

fn split_csv(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    current.push('"');
                } else {
                    quoted = false;
                }
            } else {
                current.push(c);
            }
        } else if c == '"' {
            quoted = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut current));
        } else {
            current.push(c);
        }
    }
    fields.push(current);
    fields
}

fn parse_cell(cell: &str) -> Option<f64> {
    let trimmed = cell.trim();
    if trimmed.is_empty() {
        return None;
    }
    match trimmed.parse::<f64>() {
        Ok(v) if v.is_finite() => Some(v),
        _ => None,
    }
}

fn col_pos(header: &[String], name: &str) -> Option<usize> {
    header.iter().position(|c| c.trim() == name)
}

fn parse_rows(text: &str) -> Result<(Vec<PlanetSpot>, usize), String> {
    let mut lines = text.lines();
    let header = lines
        .next()
        .ok_or_else(|| "TAP reply without header line".to_string())?;
    let header_cells = split_csv(header);
    let name_pos = col_pos(&header_cells, NAME_COL)
        .ok_or_else(|| "TAP header without pl_name column".to_string())?;
    let method_pos = col_pos(&header_cells, METHOD_COL)
        .ok_or_else(|| "TAP header without discoverymethod column".to_string())?;
    let mut numeric_pos = [None; 8];
    for (i, (col, _)) in NUMERIC.iter().enumerate() {
        numeric_pos[i] = col_pos(&header_cells, col);
    }
    let mut planets: Vec<PlanetSpot> = Vec::new();
    let mut skipped = 0usize;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let cells = split_csv(trimmed);
        let name = match cells.get(name_pos) {
            Some(c) => c.trim(),
            None => {
                skipped += 1;
                continue;
            }
        };
        if name.is_empty() {
            skipped += 1;
            continue;
        }
        let method = match cells.get(method_pos) {
            Some(c) => {
                let m = c.trim();
                if m.is_empty() {
                    None
                } else {
                    Some(m.to_string())
                }
            }
            None => None,
        };
        let mut numeric = [None; 8];
        for (i, pos) in numeric_pos.iter().enumerate() {
            let value = pos.and_then(|p| cells.get(p)).and_then(|c| parse_cell(c));
            numeric[i] = value;
        }
        planets.push(PlanetSpot {
            name: name.to_string(),
            method,
            numeric,
        });
    }
    Ok((planets, skipped))
}

fn fmt_value(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x}"),
        None => "absent".to_string(),
    }
}

fn count_text(counts: &[(String, usize)]) -> String {
    let mut out = String::new();
    for (method, n) in counts {
        out.push_str(&format!("{method:<32} {n}\n"));
    }
    out
}

fn spot_text(planet: &PlanetSpot) -> String {
    let mut parts: Vec<String> = Vec::new();
    match &planet.method {
        Some(m) => parts.push(format!("disc={m}")),
        None => parts.push("disc=absent".to_string()),
    }
    for (i, (_, label)) in NUMERIC.iter().enumerate() {
        parts.push(format!("{label}={}", fmt_value(planet.numeric[i])));
    }
    parts.join(" | ")
}

fn run(csv_path: &Option<String>, names: &[String]) -> Result<String, String> {
    let text = match csv_path {
        Some(path) => std::fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?,
        None => fetch_csv()?,
    };
    let (planets, skipped) = parse_rows(&text)?;
    if planets.is_empty() {
        return Err("no planet rows parsed from the reply".to_string());
    }
    let mut report = String::new();
    report.push_str(&format!(
        "total rows (confirmed planets, ps composite default_flag=1): {}\n",
        planets.len()
    ));
    if skipped > 0 {
        report.push_str(&format!("rows skipped (nameless): {skipped}\n"));
    }
    report.push_str(
        "empty cells are read as absent (Option/None); no cell is filled with a fabricated value\n",
    );
    let mut counts: HashMap<String, usize> = HashMap::new();
    for planet in &planets {
        if let Some(m) = &planet.method {
            *counts.entry(m.clone()).or_insert(0) += 1;
        }
    }
    let mut counts_sorted: Vec<(String, usize)> = counts.into_iter().collect();
    counts_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    report.push_str("discoverymethod counts:\n");
    report.push_str(&count_text(&counts_sorted));
    for name in names {
        let found = planets.iter().find(|p| p.name == *name);
        match found {
            Some(planet) => {
                report.push_str(&format!("{} | {}\n", planet.name, spot_text(planet)));
            }
            None => {
                report.push_str(&format!("{name} NOT FOUND\n"));
            }
        }
    }
    Ok(report)
}

fn usage() {
    eprintln!(
        "exoplanet_ps_spot_probe — spot queries of the NASA Exoplanet Archive\n\
         ps-composite (Planetary Systems, default_flag=1): discoverymethod distribution +\n\
         a parameter row per named planet (pl_rade/pl_bmasse/pl_orbper/pl_orbeccen/\n\
         pl_dens/pl_eqt/st_met/st_teff), empty cells read absent.\n\
         exoplanet_ps_spot_probe [--csv <snapshot.csv>] [--name <pl_name> ...]\n\
         without --csv the ps table is loaded live over TAP; without --name the ten\n\
         named spot samples apply."
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut csv_path: Option<String> = None;
    let mut names: Vec<String> = Vec::new();
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--csv" => {
                i += 1;
                csv_path = args.get(i).cloned();
            }
            "--name" => {
                i += 1;
                if let Some(n) = args.get(i).cloned() {
                    names.push(n);
                }
            }
            other => {
                eprintln!("exoplanet_ps_spot_probe: unknown argument {other}");
                usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if names.is_empty() {
        names = DEFAULT_NAMES.iter().map(|s| s.to_string()).collect();
    }
    let report = match run(&csv_path, &names) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("exoplanet_ps_spot_probe: {msg}");
            std::process::exit(1);
        }
    };
    print!("{report}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "pl_name,discoverymethod,pl_rade,pl_bmasse,pl_orbper,pl_orbeccen,pl_dens,pl_eqt,st_met,st_teff\n\
        \"51 Peg b\",\"Radial Velocity\",1.9,151.4,4.23,0.052,0.5,1278.0,0.33,5793\n\
        \"GJ 1214 b\",\"Transit\",2.85,6.55,1.58,0.27,,555.0,,3026\n\
        \"unknown row\",,3.0,,9.0,,,,\n";

    #[test]
    fn parser_reads_present_and_absent_fields() {
        let (planets, skipped) = parse_rows(SAMPLE).unwrap();
        assert_eq!(planets.len(), 3);
        assert_eq!(skipped, 0);
        assert_eq!(planets[0].name, "51 Peg b");
        assert_eq!(planets[0].method.as_deref(), Some("Radial Velocity"));
        assert_eq!(planets[0].numeric[0], Some(1.9));
        assert_eq!(planets[0].numeric[4], Some(0.5));
        assert_eq!(planets[1].name, "GJ 1214 b");
        assert_eq!(
            planets[1].numeric[4], None,
            "an empty density cell is absent"
        );
        assert_eq!(
            planets[1].numeric[6], None,
            "an empty st_met cell is absent"
        );
        assert_eq!(planets[2].method, None, "an empty method cell is absent");
        assert_eq!(planets[2].numeric[1], None);
    }

    #[test]
    fn count_text_ranks_the_measured_methods() {
        let (planets, _) = parse_rows(SAMPLE).unwrap();
        let mut counts: HashMap<String, usize> = HashMap::new();
        for planet in &planets {
            if let Some(m) = &planet.method {
                *counts.entry(m.clone()).or_insert(0) += 1;
            }
        }
        let mut counts_sorted: Vec<(String, usize)> = counts.into_iter().collect();
        counts_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        assert_eq!(counts_sorted.len(), 2);
        assert_eq!(counts_sorted[0].1, 1);
        assert_eq!(counts_sorted[1].0, "Transit");
    }

    #[test]
    fn spot_line_carries_absent_fields_as_words_not_zeros() {
        let (planets, _) = parse_rows(SAMPLE).unwrap();
        let line = spot_text(&planets[1]);
        assert!(line.contains("dens=absent"), "line: {line}");
        assert!(line.contains("met=absent"), "line: {line}");
        assert!(
            !line.contains("dens=0") && !line.contains("met=0"),
            "line: {line}"
        );
    }

    #[test]
    fn name_lookup_finds_the_row_and_reports_missing() {
        let (planets, _) = parse_rows(SAMPLE).unwrap();
        let found = planets.iter().find(|p| p.name == "51 Peg b");
        assert!(found.is_some());
        let missing = planets.iter().find(|p| p.name == "PSR B1257+12 b");
        assert!(missing.is_none());
    }
}
