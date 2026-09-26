use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::hamqsl::{
    COMP_MAGFIELD, COMP_SOLARFLUX, COMP_SOLARWIND, parse_bin, write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "www.hamqsl.com";
const URL: &str = "https://www.hamqsl.com/solarxml.php";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn tag(body: &str, name: &str) -> Option<String> {
    let open = format!("<{name}>");
    let close = format!("</{name}>");
    let start = body.find(&open)? + open.len();
    let end = start + body[start..].find(&close)?;
    Some(body[start..end].trim().to_string())
}

fn month_of(name: &str) -> Option<i64> {
    Some(match name {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => return None,
    })
}

fn unix_of_updated(s: &str) -> Option<f64> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 4 {
        return None;
    }
    let day: i64 = parts[0].parse().ok()?;
    let month = month_of(parts[1])?;
    let year: i64 = parts[2].parse().ok()?;
    let hhmm = parts[3].as_bytes();
    if hhmm.len() != 4 {
        return None;
    }
    let hour: i64 = parts[3][0..2].parse().ok()?;
    let minute: i64 = parts[3][2..4].parse().ok()?;
    Some(
        days_from_civil(year, month, day)? as f64 * 86400.0
            + hour as f64 * 3600.0
            + minute as f64 * 60.0,
    )
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/hamqsl_solar.bin"),
    };

    let body = fetch_raw(URL, None, &[]).ok_or_else(|| format!("{URL}: fetch void"))?;
    let updated =
        tag(&body, "updated").ok_or_else(|| "the <updated> tag stays unread".to_string())?;
    let tdb = unix_of_updated(&updated)
        .and_then(|unix| lsk.unix_to_tdb(unix))
        .ok_or_else(|| format!("updated {updated}: date reads void — the bin stays unwritten"))?;

    let mut records = Vec::new();
    if let Some(v) = tag(&body, "solarflux").and_then(|s| s.parse::<f64>().ok()) {
        if v.is_finite() && v > 0.0 {
            records.push((tdb, v, COMP_SOLARFLUX));
        }
    }
    if let Some(v) = tag(&body, "solarwind").and_then(|s| s.parse::<f64>().ok()) {
        if v.is_finite() && v > 0.0 {
            records.push((tdb, v, COMP_SOLARWIND));
        }
    }
    if let Some(v) = tag(&body, "magneticfield").and_then(|s| s.parse::<f64>().ok()) {
        if v.is_finite() {
            records.push((tdb, v, COMP_MAGFIELD));
        }
    }

    if records.is_empty() {
        return Err(format!(
            "{URL}: no physical field left the harvest — the bin stays unwritten (0 honored)"
        ));
    }
    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_bin(&bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} solar records at {updated}, roundtrip parses",
                parsed.len()
            );
            Ok(())
        }
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }?;
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("hamqsl_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_reads_the_scalar() {
        let body = "<solarflux>105</solarflux><solarwind>469.3</solarwind>";
        assert_eq!(tag(body, "solarflux").as_deref(), Some("105"));
        assert_eq!(tag(body, "solarwind").as_deref(), Some("469.3"));
        assert!(tag(body, "magneticfield").is_none());
    }

    #[test]
    fn updated_parses_the_stamp() {
        let unix = unix_of_updated("26 Sep 2026 0700 GMT").unwrap();
        let expected = days_from_civil(2026, 9, 26).unwrap() as f64 * 86400.0 + 7.0 * 3600.0;
        assert!((unix - expected).abs() < 1e-6);
        assert!(unix_of_updated("No Report").is_none());
        assert!(unix_of_updated("26 Foo 2026 0700 GMT").is_none());
    }

    #[test]
    fn month_of_knows_all_months() {
        assert_eq!(month_of("Jan"), Some(1));
        assert_eq!(month_of("Sep"), Some(9));
        assert_eq!(month_of("Dec"), Some(12));
        assert_eq!(month_of("Foo"), None);
    }
}
