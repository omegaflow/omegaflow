use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mcon_event::{
    HEADER_LEN, MconEvent, PRES_AMPLITUDE, PRES_BAY_DECREASE, PRES_BAY_DURATION, PRES_BAY_WIDTH,
    PRES_SML, PRES_TIME, REC_BYTES, encode_rec, jd_from_unix, parse_bin, write_header,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::zip_members;
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "zenodo.org";
const FORMAT: &str = "mcon_event";
const URL: &str = "https://zenodo.org/api/records/22711679/files/CorrOnsFiles.zip/content";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn cols(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

fn parse_num(cell: Option<&&str>) -> Option<f64> {
    let t = cell?.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_unix(c: &[&str]) -> Option<f64> {
    if c.len() < 6 {
        return None;
    }
    let year: i64 = c[0].parse().ok()?;
    let month: i64 = c[1].parse().ok()?;
    let day: i64 = c[2].parse().ok()?;
    let hour: i64 = c[3].parse().ok()?;
    let minute: i64 = c[4].parse().ok()?;
    let second: i64 = c[5].parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let unix = (days * 86400 + hour * 3600 + minute * 60 + second) as f64;
    if unix.is_finite() { Some(unix) } else { None }
}

fn parse_rows(ftims: &str, fvals: &str, fadd: &str) -> Vec<MconEvent> {
    let vlines: Vec<&str> = fvals.lines().collect();
    let alines: Vec<&str> = fadd.lines().collect();
    let mut out = Vec::new();
    for (i, line) in ftims.lines().enumerate() {
        let tc = cols(line);
        let Some(unix) = parse_unix(&tc) else {
            continue;
        };
        let Some(jd_utc) = jd_from_unix(unix) else {
            continue;
        };
        let mut present = PRES_TIME;
        let (mut sml, mut amplitude) = (0.0f32, 0.0f32);
        let (mut bay_decrease, mut bay_duration, mut bay_width) = (0.0f32, 0.0f32, 0.0f32);
        if let Some(vl) = vlines.get(i) {
            let vc = cols(vl);
            if let Some(v) = parse_num(vc.get(9)) {
                sml = v as f32;
                present |= PRES_SML;
            }
        }
        if let Some(al) = alines.get(i) {
            let ac = cols(al);
            if let Some(v) = parse_num(ac.get(6)) {
                amplitude = v as f32;
                present |= PRES_AMPLITUDE;
            }
            if let Some(v) = parse_num(ac.get(7)) {
                bay_decrease = v as f32;
                present |= PRES_BAY_DECREASE;
            }
            if let Some(v) = parse_num(ac.get(8)) {
                bay_duration = v as f32;
                present |= PRES_BAY_DURATION;
            }
            if let Some(v) = parse_num(ac.get(9)) {
                bay_width = v as f32;
                present |= PRES_BAY_WIDTH;
            }
        }
        if present == PRES_TIME {
            continue;
        }
        out.push(MconEvent {
            present,
            jd_utc,
            sml,
            amplitude,
            bay_decrease,
            bay_duration,
            bay_width,
        });
    }
    out
}

fn write_bin(rows: &[MconEvent]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + rows.len() * REC_BYTES);
    write_header(&mut out, rows.len() as u64);
    for r in rows {
        let mut rec = [0u8; REC_BYTES];
        encode_rec(&mut rec, r);
        out.extend_from_slice(&rec);
    }
    out
}

fn member_text(zip: &[u8], needle: &str) -> Option<String> {
    let mut text: Option<String> = None;
    zip_members(zip, |name, data| {
        if text.is_none() && name.ends_with(needle) {
            text = Some(String::from_utf8_lossy(data).into_owned());
        }
    });
    text
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out-bin") {
        Some(v) => v,
        None => "mcpherron_onsets.bin".to_string(),
    };

    let bytes = match arg_value(args, "--in") {
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?
        }
        None => fetch_raw_bytes(URL).ok_or_else(|| format!("{URL}: fetch void"))?,
    };
    let ftims =
        member_text(&bytes, "ftims.txt").ok_or_else(|| format!("{URL}: no ftims.txt member"))?;
    let fvals =
        member_text(&bytes, "fvals.txt").ok_or_else(|| format!("{URL}: no fvals.txt member"))?;
    let fadd =
        member_text(&bytes, "fadd.txt").ok_or_else(|| format!("{URL}: no fadd.txt member"))?;

    let rows = parse_rows(&ftims, &fvals, &fadd);
    if rows.is_empty() {
        return Err("no onsets parsed — the bin stays unwritten (0 honored)".to_string());
    }
    let bin = write_bin(&rows);
    if parse_bin(&bin).is_none() {
        return Err(format!("{out}: roundtrip parse void"));
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    eprintln!(
        "{out}: {} onsets ({} B), format {FORMAT}, sha256 {}",
        rows.len(),
        bin.len(),
        sha256_hex(&bin)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: mcpherron_compiler [--in <CorrOnsFiles.zip>] [--out-bin <name>] [--ci-mode]"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("mcpherron_compiler: {msg}");
        std::process::exit(1);
    }
}
