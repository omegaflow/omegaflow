use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::lsk::days_from_civil;

trait AbsentExt<T> {
    fn or_absent(self, marker: &str) -> String;
}

impl<T: ToString> AbsentExt<T> for Option<T> {
    fn or_absent(self, marker: &str) -> String {
        match self {
            Some(v) => v.to_string(),
            None => marker.to_string(),
        }
    }
}

pub const COMP_HYDROPHONE_PSD_DB: u32 = 1;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_HYDROPHONE_PSD_DB => Some("pangaea_palaoa_hydrophone_psd_db"),
        _ => None,
    }
}

pub const DEFAULT_URL: &str =
    "https://github.com/omegaflow/sources/releases/download/doi.pangaea.de/pangaea_967167.json";
pub const ORIGIN: &str = "https://doi.pangaea.de/10.1594/PANGAEA.967167?format=textfile";

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub date: String,
    pub t: Option<f64>,
    pub depth_m: Option<f64>,
    pub audio: String,
    pub size_text: String,
    pub size_bytes: Option<f64>,
    pub cells: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PangaeaDoc {
    pub citation: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
}

fn number_after(line: &str, key: &str) -> Option<f64> {
    let pos = line.find(key)? + key.len();
    line[pos..]
        .trim()
        .split_whitespace()
        .next()?
        .trim_end_matches(['*', ','])
        .parse::<f64>()
        .ok()
}

fn date_unix(s: &str) -> Option<f64> {
    let mut p = s.trim().split('-');
    let y = p.next()?.parse::<i64>().ok()?;
    let m = p.next()?.parse::<i64>().ok()?;
    let d = p.next()?.parse::<i64>().ok()?;
    Some(days_from_civil(y, m, d)? as f64 * 86400.0)
}

fn parse_size(text: &str) -> Option<f64> {
    let mut p = text.split_whitespace();
    let value = p.next()?.parse::<f64>().ok()?;
    let unit = p.next().unwrap_or("").to_ascii_lowercase();
    let factor = if unit.starts_with("tb") {
        1e12
    } else if unit.starts_with("gb") {
        1e9
    } else if unit.starts_with("mb") {
        1e6
    } else if unit.starts_with("kb") {
        1e3
    } else {
        1.0
    };
    let bytes = value * factor;
    bytes.is_finite().then_some(bytes)
}

fn split_tab(line: &str) -> Vec<String> {
    line.trim_end_matches('\r')
        .split('\t')
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn parse_text(bytes: &[u8]) -> Option<PangaeaDoc> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut citation = None;
    let mut lat = None;
    let mut lon = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("Citation:") {
            citation = Some(rest.trim().to_string());
        }
        if line.contains("LATITUDE:") {
            lat = number_after(line, "LATITUDE:");
        }
        if line.contains("LONGITUDE:") {
            lon = number_after(line, "LONGITUDE:");
        }
    }

    let mut after = false;
    let mut columns: Vec<String> = Vec::new();
    let mut rows = Vec::new();
    for line in text.lines() {
        let t = line.trim_end();
        if t == "*/" {
            after = true;
            continue;
        }
        if !after || t.trim().is_empty() {
            continue;
        }
        let cells = split_tab(line);
        if cells.len() < 2 {
            continue;
        }
        if columns.is_empty() {
            columns = cells;
            continue;
        }
        let get = |needle: &str| {
            columns
                .iter()
                .position(|c| c.to_ascii_lowercase().contains(needle))
                .and_then(|i| cells.get(i))
                .cloned()
                .or_absent("")
        };
        let date = get("date");
        let size_text = get("size");
        rows.push(Row {
            t: date_unix(&date),
            depth_m: get("depth").parse::<f64>().ok().filter(|v| v.is_finite()),
            audio: get("audio"),
            size_text: size_text.clone(),
            size_bytes: parse_size(&size_text),
            date,
            cells,
        });
    }
    if columns.is_empty() || rows.is_empty() {
        return None;
    }
    Some(PangaeaDoc {
        citation,
        lat,
        lon,
        columns,
        rows,
    })
}

pub fn psd_column(doc: &PangaeaDoc) -> Option<usize> {
    doc.columns.iter().position(|c| {
        let l = c.to_ascii_lowercase();
        l.contains("psd") || l.contains("sound pressure") || l.contains("spl")
    })
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let doc = parse_text(bytes)?;
    let col = psd_column(&doc)?;
    let mut out = Vec::new();
    for row in &doc.rows {
        let t = row.t?;
        let raw = row.cells.get(col)?.parse::<f64>().ok()?;
        if raw.is_finite() {
            out.push((t, raw, COMP_HYDROPHONE_PSD_DB));
        }
    }
    (!out.is_empty()).then_some(out)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load(args: &[String]) -> Option<(String, Vec<u8>)> {
    if let Some(path) = arg_value(args, "--file") {
        std::fs::read(&path).ok().map(|b| (path, b))
    } else {
        let url = arg_value(&args, "--url").or_absent(DEFAULT_URL);
        fetch_raw_bytes(&url).map(|b| (url, b))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((src, bytes)) = load(&args) else {
        eprintln!("pangaea: {ORIGIN} stays unfetched");
        std::process::exit(1);
    };
    let Some(doc) = parse_text(&bytes) else {
        eprintln!(
            "{src}: {} B are not a PANGAEA textfile (the stored .json release is not the origin)",
            bytes.len()
        );
        std::process::exit(1);
    };
    eprintln!(
        "{src}: {} row(s), lat {}, lon {}",
        doc.rows.len(),
        doc.lat.map(|v| format!("{v:.3}")).or_absent("absent"),
        doc.lon.map(|v| format!("{v:.3}")).or_absent("absent")
    );
    eprintln!("  columns: {}", doc.columns.join(" | "));
    for row in doc.rows.iter().take(3) {
        eprintln!(
            "  date={} depth={} audio={} size={}",
            row.date,
            row.depth_m.map(|v| format!("{v:.0}")).or_absent("absent"),
            row.audio,
            row.size_text
        );
    }
    match psd_column(&doc) {
        Some(i) => eprintln!(
            "  {}: column {}",
            component_name(COMP_HYDROPHONE_PSD_DB).unwrap(),
            doc.columns[i]
        ),
        None => eprintln!(
            "  {}: absent (the textfile carries archive names/sizes, no dB column)",
            component_name(COMP_HYDROPHONE_PSD_DB).unwrap()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &[u8] = b"/* DATA DESCRIPTION:\n\
Citation:\tThomisch, Karolin; Spiesecke, Stefanie; Boebel, Olaf (2024): Passive acoustic monitoring data recorded by the Perennial Acoustic Observatory in the Antarctic Ocean (PALAOA), Antarctica, in 2021 [dataset]. PANGAEA, https://doi.org/10.1594/PANGAEA.967167\n\
Coverage:\tLATITUDE: -70.523000 * LONGITUDE: -8.230000\n\
*/\n\
Date/Time\tDepth water [m]\tAUDIO\tAUDIO (Size) [Bytes]\n\
2021-01-01\t155\t20210101.tar\t23 GBytes\n\
2021-01-02\t155\t20210102.tar\t23.2 GBytes\n";

    #[test]
    fn reads_real_pangaea_head() {
        let doc = parse_text(HEAD).expect("textfile head parses");
        assert_eq!(doc.rows.len(), 2);
        assert!((doc.lat.unwrap() - -70.523).abs() < 1e-9);
        assert!((doc.lon.unwrap() - -8.230).abs() < 1e-9);
        assert_eq!(doc.rows[0].date, "2021-01-01");
        assert_eq!(doc.rows[0].depth_m, Some(155.0));
        assert_eq!(doc.rows[0].audio, "20210101.tar");
        assert!((doc.rows[0].size_bytes.unwrap() - 23e9).abs() < 1.0);
    }

    #[test]
    fn psd_is_absent_in_this_dataset() {
        let doc = parse_text(HEAD).unwrap();
        assert!(psd_column(&doc).is_none());
        assert!(parse_series(HEAD).is_none());
    }

    #[test]
    fn reads_a_psd_column_when_one_exists() {
        let text = b"/* DATA DESCRIPTION:\n\
Coverage:\tLATITUDE: -70.523000 * LONGITUDE: -8.230000\n\
*/\n\
Date/Time\tDepth water [m]\tPSD [dB]\n\
2021-01-01\t155\t42.5\n";
        let doc = parse_text(text).expect("PSD table parses");
        assert_eq!(psd_column(&doc), Some(2));
        let rows = parse_series(text).expect("PSD value reads");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].2, COMP_HYDROPHONE_PSD_DB);
        assert!((rows[0].1 - 42.5).abs() < 1e-9);
    }

    #[test]
    fn absent_for_non_textfile_bytes() {
        assert!(parse_text(b"{not a pangaea textfile}").is_none());
    }
}
