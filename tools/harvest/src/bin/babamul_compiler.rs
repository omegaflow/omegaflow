use omegaflow::archivar::{babamul, fetch_raw_bytes_headers, load_env};
use omegaflow::cdn::upload_release;
use omegaflow::skymap::{
    HEADER_LEN, REC_BYTES, SkymapRecord, decode_rec, encode_rec, parse_header, write_header,
};

const NETLOC: &str = "babamul.caltech.edu";
const BASE: &str = "https://babamul.caltech.edu/api/babamul";
const DEFAULT_SURVEY: &str = "ZTF";
const DEFAULT_OUT: &str = "data/babamul.caltech.edu/babamul_alerts.bin";
const JD_WINDOW: f64 = 0.999;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn jd_now() -> f64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs_f64() / 86400.0 + 2440587.5,
        Err(_) => 0.0,
    }
}

fn write_sky1(records: &[SkymapRecord]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + records.len() * REC_BYTES);
    write_header(&mut out, records.len() as u64);
    let mut rec = [0u8; REC_BYTES];
    for r in records {
        encode_rec(&mut rec, r);
        out.extend_from_slice(&rec);
    }
    out
}

fn read_sky1(data: &[u8]) -> Option<Vec<SkymapRecord>> {
    let n = parse_header(data)? as usize;
    if data.len() != HEADER_LEN + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_LEN;
    for _ in 0..n {
        let rec = decode_rec(data.get(off..off + REC_BYTES)?)?;
        off += REC_BYTES;
        out.push(rec);
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let survey = match arg_value(&args, "--survey") {
        Some(v) => v,
        None => DEFAULT_SURVEY.to_string(),
    };
    let env = load_env();
    let Some(token) = env.get("BABAMUL_API_TOKEN") else {
        eprintln!("BABAMUL_API_TOKEN absent in .secrets.local — the alerts route stays unread");
        std::process::exit(1);
    };
    let start_jd = match arg_value(&args, "--start-jd") {
        Some(v) => match v.parse::<f64>() {
            Ok(x) if x.is_finite() && x > 0.0 => x,
            _ => {
                eprintln!("--start-jd {v} carries no finite Julian Date");
                std::process::exit(1);
            }
        },
        None => jd_now() - JD_WINDOW,
    };
    let url = format!(
        "{BASE}/surveys/{survey}/alerts?start_jd={start_jd}&end_jd={}",
        start_jd + JD_WINDOW
    );
    let headers = [("Authorization".to_string(), format!("Bearer {token}"))];
    let bytes = match fetch_raw_bytes_headers(&url, &headers) {
        Some(b) => b,
        None => {
            eprintln!("{url}: fetch void");
            std::process::exit(1);
        }
    };
    let body = String::from_utf8_lossy(&bytes);
    let alerts = match babamul::parse_alerts(&body) {
        babamul::BabamulParse::Alerts(a) => a,
        babamul::BabamulParse::Empty => {
            eprintln!(
                "{survey} {start_jd}: the alert envelope carries zero candidate rows in {} B — the bin stays unwritten (0 honored)",
                bytes.len()
            );
            return;
        }
        babamul::BabamulParse::NotJson => {
            eprintln!("{url}: the body is not JSON ({} B)", bytes.len());
            std::process::exit(2);
        }
        babamul::BabamulParse::NoData => {
            eprintln!(
                "{url}: the JSON carries no alert data array ({} B)",
                bytes.len()
            );
            std::process::exit(2);
        }
        babamul::BabamulParse::Unplaced => {
            eprintln!(
                "{url}: candidate rows arrived, none placeable ({} B)",
                bytes.len()
            );
            std::process::exit(2);
        }
    };
    let records = babamul::to_skymap(&alerts);
    if records.is_empty() {
        eprintln!(
            "{survey} {start_jd}: no placeable candidate — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let bin = write_sky1(&records);
    if let Some(parent) = std::path::Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match read_sky1(&bin) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} alerts from {survey} JD {start_jd}, {} B — roundtrip parses",
                records.len(),
                bin.len()
            );
        }
        _ => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
