use omegaflow::cdn::upload_release;
use std::process::Command;

const PIPE_URL: &str = "https://exofop.ipac.caltech.edu/tess/download_toi.php?sort=toi&output=pipe";
const CDN_TAG: &str = "exofop.ipac.caltech.edu";

const MAGIC: [u8; 4] = *b"EXF1";
const VERSION: u8 = 1;
const HEADER_LEN: usize = 13;
const REC_BYTES: usize = 44;

const COL_TIC_ID: usize = 0;
const COL_TOI: usize = 1;
const COL_RA: usize = 25;
const COL_DEC: usize = 26;
const COL_EPOCH_BJD: usize = 31;
const COL_DEPTH_PPM: usize = 39;

fn cell<'a>(cells: &'a [&str], idx: usize) -> Option<&'a str> {
    cells.get(idx).map(|s| s.trim()).filter(|s| !s.is_empty())
}

fn cell_f64(cells: &[&str], idx: usize) -> Option<f64> {
    let v: f64 = cell(cells, idx)?.parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn sexagesimal_deg(s: &str, hours: bool) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let (neg, rest) = match s.as_bytes().first()? {
        b'-' => (true, &s[1..]),
        b'+' => (false, &s[1..]),
        _ => (false, s),
    };
    let mut parts = rest.split(':');
    let d: f64 = parts.next()?.trim().parse().ok()?;
    let m: f64 = parts.next()?.trim().parse().ok()?;
    let sec: f64 = parts.next()?.trim().parse().ok()?;
    let mut deg = d + m / 60.0 + sec / 3600.0;
    if hours {
        deg *= 15.0;
    }
    if neg {
        deg = -deg;
    }
    if deg.is_finite() { Some(deg) } else { None }
}

fn record_bytes(line: &str) -> Option<Vec<u8>> {
    let cells: Vec<&str> = line.split('|').collect();
    let tic_id: u64 = cell(&cells, COL_TIC_ID)?.parse().ok()?;
    let toi = cell_f64(&cells, COL_TOI)?;
    if toi <= 0.0 {
        return None;
    }
    let ra = sexagesimal_deg(cell(&cells, COL_RA)?, true)?;
    let dec = sexagesimal_deg(cell(&cells, COL_DEC)?, false)?;
    if ra < 0.0 || ra >= 360.0 || dec.abs() > 90.0 {
        return None;
    }
    let bjd = cell_f64(&cells, COL_EPOCH_BJD)?;
    if bjd <= 2400000.0 {
        return None;
    }
    let depth = cell_f64(&cells, COL_DEPTH_PPM)?;
    if depth <= 0.0 {
        return None;
    }
    let mut out = Vec::with_capacity(REC_BYTES);
    out.extend_from_slice(&tic_id.to_le_bytes());
    out.extend_from_slice(&toi.to_le_bytes());
    out.extend_from_slice(&ra.to_le_bytes());
    out.extend_from_slice(&dec.to_le_bytes());
    out.extend_from_slice(&bjd.to_le_bytes());
    out.extend_from_slice(&(depth as f32).to_le_bytes());
    Some(out)
}

fn fetch_pipe(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("--max-time")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "exofop fetch {}: {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn compile(body: &str, out_path: &str) -> usize {
    let mut records = Vec::new();
    let mut written = 0usize;
    let mut skipped = 0usize;
    let mut header_done = false;
    for line in body.split('\n') {
        let line = line.trim_end_matches('\r').trim();
        if line.is_empty() {
            continue;
        }
        if !header_done {
            header_done = true;
            continue;
        }
        match record_bytes(line) {
            Some(rec) => {
                records.extend_from_slice(&rec);
                written += 1;
            }
            None => {
                skipped += 1;
            }
        }
    }
    let mut out = Vec::with_capacity(HEADER_LEN + records.len());
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&(written as u64).to_le_bytes());
    out.extend_from_slice(&records);
    if let Err(e) = std::fs::write(out_path, &out) {
        panic!("write {}: {}", out_path, e);
    }
    eprintln!(
        "exofop: {} records, {} skipped, {} B -> {}",
        written,
        skipped,
        out.len(),
        out_path
    );
    written
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    let Some(out_path) = out else {
        eprintln!("usage: exofop_compiler --out <exofop_toi.bin> [--ci-mode]");
        std::process::exit(1);
    };
    let Some(body) = fetch_pipe(PIPE_URL) else {
        eprintln!("exofop pipe fetch: no content");
        std::process::exit(2);
    };
    compile(&body, &out_path);
    if ci_mode && !upload_release(CDN_TAG, &out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line_with(cols: &[(usize, &str)]) -> String {
        let mut cells = vec![""; 63];
        for &(i, v) in cols {
            cells[i] = v;
        }
        cells.join("|")
    }

    #[test]
    fn sexagesimal_ra_and_dec_to_degrees() {
        let ra = sexagesimal_deg("21:14:56.88", true).unwrap();
        assert!((ra - 318.737).abs() < 1e-3);
        let dec = sexagesimal_deg("-55:52:18.71", false).unwrap();
        assert!((dec + 55.871864).abs() < 1e-5);
        assert!(sexagesimal_deg("", true).is_none());
        assert!(sexagesimal_deg("21:14", true).is_none());
    }

    #[test]
    fn record_parses_real_pipe_row() {
        let line = "231663901|101.01||5|5|5|5|5|5|5|86.8|209.9|115.19|63.3|0|1|3|KP|KP|12.4069|0.006||1|spoc-s01-s69-b0A-KP|SPOC|21:14:56.88|-55:52:18.71|12.641|0.044|-16.011|0.041|2458326.009117|0.00013154597|1.43036994965074|0.00000077233494|1.61659940399439|0.019208942|20.784|0.200653|18960.7122943629|184.79134|13.1874503075824|.6576402|1281.2408253875|1525.90480889146|151.72173|375.31|4.411|5600||4.48851||.890774011611938|.0438467|||1.05|.129454|1,27,67|2018-09-05|2024-09-06|2025-07-31 12:59:22|WASP-46 b";
        let rec = record_bytes(line).unwrap();
        assert_eq!(rec.len(), REC_BYTES);
        let tic_id = u64::from_le_bytes(rec[0..8].try_into().unwrap());
        assert_eq!(tic_id, 231663901);
        let ra = f64::from_le_bytes(rec[16..24].try_into().unwrap());
        assert!((ra - 318.737).abs() < 1e-3);
        let dec = f64::from_le_bytes(rec[24..32].try_into().unwrap());
        assert!((dec + 55.871864).abs() < 1e-5);
        let depth = f32::from_le_bytes(rec[40..44].try_into().unwrap());
        assert!((depth - 18960.71).abs() < 1.0);
    }

    #[test]
    fn record_void_on_bjd_below_2400000() {
        let line = line_with(&[
            (COL_TIC_ID, "42"),
            (COL_TOI, "42.01"),
            (COL_RA, "21:14:56.88"),
            (COL_DEC, "-55:52:18.71"),
            (COL_EPOCH_BJD, "2450000.0"),
            (COL_DEPTH_PPM, "10.0"),
        ]);
        assert!(record_bytes(&line).is_none());
    }

    #[test]
    fn record_void_on_empty_ra() {
        let line = line_with(&[
            (COL_TIC_ID, "42"),
            (COL_TOI, "42.01"),
            (COL_DEC, "-55:52:18.71"),
            (COL_EPOCH_BJD, "2458326.0"),
            (COL_DEPTH_PPM, "10.0"),
        ]);
        assert!(record_bytes(&line).is_none());
    }
}
