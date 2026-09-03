use std::process::Command;

const MAGIC: [u8; 2] = [0xCF, 0x86];
const SRD62_VERSION: u8 = 0x05;
const ANGSTROM_M: f64 = 1.0e-10;
const LAMBDA_MIN_ANGSTROM: f64 = 50.0;
const LAMBDA_MAX_ANGSTROM: f64 = 500000.0;

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "curl {} http {}: {}",
            url,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn strip_tags(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&sigma;", "sigma")
        .replace("&amp;", "&")
        .replace("&#39;", "'")
        .replace("&delta;", "d")
        .trim()
        .to_string()
}

fn search_citations(html: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let needle = "href=\"/CeramicDataPortal/Hts/";
    let mut rest = html;
    while let Some(i) = rest.find(needle) {
        let after = &rest[i + needle.len()..];
        let id: String = after.chars().take_while(|c| *c != '"').collect();
        let id_len = id.len();
        if !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric()) && !ids.contains(&id) {
            ids.push(id);
        }
        rest = &after[id_len..];
    }
    ids
}

fn table_rows(table: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut rest = table;
    while let Some(ts) = rest.find("<tr") {
        let Some(te_rel) = rest[ts..].find("</tr>") else {
            break;
        };
        let te = ts + te_rel + 5;
        let row = &rest[ts..te];
        let mut cells = Vec::new();
        let mut cre = row;
        loop {
            let td = cre.find("<td");
            let th = cre.find("<th");
            let (tag, close) = match (td, th) {
                (Some(a), Some(b)) if a < b => ("<td", "</td>"),
                (Some(_), None) => ("<td", "</td>"),
                (None, Some(_)) => ("<th", "</th>"),
                _ => break,
            };
            let Some(cs) = cre.find(tag) else { break };
            let Some(ce_rel) = cre[cs..].find(close) else {
                break;
            };
            let ce = cs + ce_rel + close.len();
            cells.push(strip_tags(&cre[cs..ce]));
            cre = &cre[ce..];
        }
        if !cells.is_empty() {
            rows.push(cells);
        }
        rest = &rest[te..];
    }
    rows
}

fn parse_penetration(html: &str) -> Vec<(f64, f64, f64)> {
    let mut points = Vec::new();
    let mut rest = html;
    while let Some(ts) = rest.find("<table") {
        let Some(te_rel) = rest[ts..].find("</table>") else {
            break;
        };
        let te = ts + te_rel + 8;
        let table = &rest[ts..te];
        if !table.to_lowercase().contains("penetration depth") {
            rest = &rest[te..];
            continue;
        }
        let rows = table_rows(table);
        let mut col = None;
        for r in &rows {
            for (i, c) in r.iter().enumerate() {
                if c.to_lowercase().contains("penetration") {
                    col = Some(i);
                    break;
                }
            }
            if col.is_some() {
                break;
            }
        }
        let Some(ci) = col else {
            rest = &rest[te..];
            continue;
        };
        for r in &rows {
            if let Some(cell) = r.get(ci) {
                if let Ok(v) = cell.parse::<f64>() {
                    if v.is_finite() && v >= LAMBDA_MIN_ANGSTROM && v <= LAMBDA_MAX_ANGSTROM {
                        let indep = r
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i != ci)
                            .find_map(|(_, c)| c.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        points.push((indep, v, 0.0));
                    }
                }
            }
        }
        rest = &rest[te..];
    }
    points
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out_path = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "srd62_suprastrom.bin".to_string());

    let search_url = "https://srdata.nist.gov/CeramicDataPortal/Hts/DoSearch?Properties=44";
    let Some(search_html) = fetch(search_url) else {
        eprintln!("srd62_compiler: search carries void");
        std::process::exit(1);
    };
    let ids = search_citations(&search_html);
    eprintln!(
        "srd62_compiler: {} citations with Penetration Depth",
        ids.len()
    );

    let mut points: Vec<(f64, f64, f64)> = Vec::new();
    for id in &ids {
        let url = format!("https://srdata.nist.gov/CeramicDataPortal/Hts/{}", id);
        let Some(html) = fetch(&url) else {
            continue;
        };
        let ps = parse_penetration(&html);
        if !ps.is_empty() {
            eprintln!("  {} -> {} points", id, ps.len());
            points.extend(ps);
        }
    }
    points.retain(|(_, l, _)| *l > 0.0 && l.is_finite());
    eprintln!(
        "srd62_compiler: {} penetration-depth points total",
        points.len()
    );

    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.push(SRD62_VERSION);
    out.extend_from_slice(&(points.len() as u32).to_le_bytes());
    for (t, l, s) in &points {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&(l * ANGSTROM_M).to_le_bytes());
        out.extend_from_slice(&(s * ANGSTROM_M).to_le_bytes());
    }
    if let Err(e) = std::fs::write(&out_path, &out) {
        eprintln!("srd62_compiler: write {} returned {}", out_path, e);
        std::process::exit(1);
    }
    eprintln!(
        "srd62_compiler: wrote {} ({} points)",
        out_path,
        points.len()
    );
}
