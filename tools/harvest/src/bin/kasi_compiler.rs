use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::json::{JsonVal, jnum, jstr, parse_json};
use omegaflow::cdn::upload_release;

const CDN_TAG: &str = "data.kasi.re.kr";

struct Facility {
    name: &'static str,
    url: &'static str,
}

const FACILITIES: [Facility; 3] = [
    Facility {
        name: "miris",
        url: "https://data.kasi.re.kr/api/MIRIS/search?ra=84.0&dec=-70.0",
    },
    Facility {
        name: "kmtnet",
        url: "https://data.kasi.re.kr/api/KMTNet/search?ra=270.694&dec=-30.066&rad=60&unit=arcmin&limit=100000",
    },
    Facility {
        name: "kvn",
        url: "https://data.kasi.re.kr/api/KVN/search?ra=267.075&dec=-28.127&rad=1.5&unit=degree",
    },
];

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn key_num(row: &JsonVal, keys: &[&str]) -> Option<f64> {
    keys.iter().find_map(|k| jnum(row, k))
}

fn key_str(row: &JsonVal, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| jstr(row, k))
}

const UNIX_JD_OFFSET: f64 = 2440587.5;

fn iso8601_to_jd(s: &str) -> Option<f64> {
    let year: f64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let hour: i64 = s.get(11..13)?.parse().ok()?;
    let minute: i64 = s.get(14..16)?.parse().ok()?;
    let second: f64 = s.get(17..19)?.parse().ok()?;
    let days = omegaflow::lsk::days_from_civil(year as i64, month, day)?;
    let unix = days as f64 * 86400.0 + (hour * 3600 + minute * 60) as f64 + second;
    let jd = unix / 86400.0 + UNIX_JD_OFFSET;
    if jd.is_finite() { Some(jd) } else { None }
}

fn jd_of(row: &JsonVal) -> Option<f64> {
    if let Some(jd) = key_num(row, &["MIDJD", "jd"]) {
        return Some(jd);
    }
    if let Some(s) = key_str(row, &["MIDJD"]) {
        if let Ok(jd) = s.trim().parse::<f64>() {
            return Some(jd);
        }
    }
    key_str(row, &["start_date_ut"])
        .or_else(|| key_str(row, &["DATE-OBS"]))
        .and_then(|s| iso8601_to_jd(&s))
}

fn one_record(row: &JsonVal) -> Option<String> {
    let ra = key_num(row, &["RA-CNT", "RA", "ra_deg", "ra"])?;
    let dec = key_num(row, &["DEC-CNT", "DEC", "dec_deg", "dec"])?;
    if !(ra.is_finite() && (0.0..360.0).contains(&ra)) {
        return None;
    }
    if !(dec.is_finite() && (-90.0..=90.0).contains(&dec)) {
        return None;
    }
    let jd = jd_of(row)?;
    if !jd.is_finite() {
        return None;
    }
    let name = match key_str(row, &["OBS-ID", "DATAID", "exp_code", "name"]) {
        Some(s) => json_str(&s),
        None => "null".to_string(),
    };
    let dataurl = match key_str(row, &["DATAURL", "view_url", "dataurl"]) {
        Some(s) => json_str(&s),
        None => "null".to_string(),
    };
    Some(format!(
        "{{\"name\":{},\"ra\":{},\"dec\":{},\"jd\":{},\"dataurl\":{}}}",
        name, ra, dec, jd, dataurl
    ))
}

fn parse_rows(text: &str) -> Vec<String> {
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        return Vec::new();
    };
    rows.iter().filter_map(one_record).collect()
}

const FIXTURE: &str = "[{\"OBS-ID\":\"MS1412446136\",\"RA-CNT\":84.08379234866028,\"DEC-CNT\":-70.45057767297078,\"MIDJD\":2456935.2589814817,\"DATAURL\":\"https://archive.kasi.re.kr/miris/201410/MS1412446136.fits\"},{\"OBS-ID\":\"void\",\"RA-CNT\":400.0,\"DEC-CNT\":-70.0,\"MIDJD\":2456935.0}]";

fn selftest() {
    let records = parse_rows(FIXTURE);
    if records.len() != 1 {
        eprintln!(
            "selftest: {} of 2 rows survived, 1 expected (the 400-degree RA stays absent)",
            records.len()
        );
        std::process::exit(1);
    }
    if !records[0].contains("\"ra\":84.08379234866028") {
        eprintln!("selftest: the RA does not carry the measured value");
        std::process::exit(1);
    }
    if parse_rows("not json") != Vec::<String>::new() {
        eprintln!("selftest: a non-JSON body is read as records");
        std::process::exit(1);
    }
    eprintln!("kasi_compiler: selftest passes (JSON fold + plausibility gate)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_dir = match args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
    {
        Some(v) => v.clone(),
        None => omegaflow::archivar::cache_root()
            .to_string_lossy()
            .into_owned(),
    };

    let mut written = 0usize;
    for facility in &FACILITIES {
        let Some(bytes) = fetch_raw_bytes(facility.url) else {
            eprintln!(
                "{}: fetch void — the facility stays absent (0 honored)",
                facility.name
            );
            continue;
        };
        let Ok(text) = std::str::from_utf8(&bytes) else {
            eprintln!(
                "{}: response not utf8 — the facility stays absent (0 honored)",
                facility.name
            );
            continue;
        };
        let records = parse_rows(text);
        if records.is_empty() {
            eprintln!(
                "{}: no observation survived the plausibility gate — absent (0 honored)",
                facility.name
            );
            continue;
        }
        let json = format!("[{}]\n", records.join(","));
        let asset = format!("kasi_{}.json", facility.name);
        let out_path = format!("{}/{}", out_dir.trim_end_matches('/'), asset);
        if let Some(parent) = std::path::Path::new(&out_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if std::fs::write(&out_path, &json).is_err() {
            eprintln!("{}: write {} returned void", facility.name, out_path);
            continue;
        }
        if parse_rows(&json).len() != records.len() {
            eprintln!(
                "{}: roundtrip lost rows — the asset stays unverified (0 honored)",
                facility.name
            );
            continue;
        }
        eprintln!(
            "{}: {} observation(s), {} B",
            out_path,
            records.len(),
            json.len()
        );
        if ci_mode && !upload_release(CDN_TAG, &out_path) {
            eprintln!("{asset}: CDN upload returned void");
            continue;
        }
        written += 1;
    }
    if written == 0 {
        eprintln!("no KASI facility written (0 honored)");
        std::process::exit(1);
    }
}
