use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::geo::{
    COMP_OGM_DEWP, COMP_OGM_SLP, COMP_OGM_TEMP, COMP_OGM_WSPD, MAGIC_OGM, parse_bin, write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "www.ogimet.com";
const BASE: &str = "https://www.ogimet.com/cgi-bin/gsynres";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
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
    out
}

fn cells(row: &str) -> Vec<String> {
    let lower = row.to_ascii_lowercase();
    let mut out = Vec::new();
    let mut idx = 0usize;
    while let Some(rel) = lower[idx..].find("<td") {
        let start = idx + rel;
        let Some(tag_end) = lower[start..].find('>').map(|p| start + p + 1) else {
            break;
        };
        let Some(close) = lower[tag_end..].find("</td").map(|p| tag_end + p) else {
            break;
        };
        out.push(strip_tags(&row[tag_end..close]));
        idx = close + 5;
    }
    out
}

fn after(text: &str, marker: &str) -> Option<String> {
    let pos = text.find(marker)? + marker.len();
    let rest = text[pos..].trim_start();
    let token: String = rest.chars().take_while(|c| !c.is_whitespace()).collect();
    if token.is_empty() { None } else { Some(token) }
}

fn parse_dms(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.len() < 2 {
        return None;
    }
    let (num, hemi) = s.split_at(s.len() - 1);
    let parts: Vec<&str> = num.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let d: f64 = parts[0].parse().ok()?;
    let m: f64 = parts[1].parse().ok()?;
    let sec: f64 = parts[2].parse().ok()?;
    let mut deg = d + m / 60.0 + sec / 3600.0;
    if hemi == "S" || hemi == "W" {
        deg = -deg;
    }
    Some(deg)
}

fn parse_mdy(s: &str) -> Option<(i64, i64, i64)> {
    let parts: Vec<&str> = s.trim().split('/').collect();
    if parts.len() != 3 {
        return None;
    }
    Some((
        parts[2].parse().ok()?,
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
    ))
}

fn parse_hm(s: &str) -> Option<(i64, i64)> {
    let parts: Vec<&str> = s.trim().split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
}

fn station_anchor(body: &str) -> Option<(f64, f64, f64)> {
    let h4_start = body.find("<h4>")? + 4;
    let h4_end = body[h4_start..].find("</h4>")? + h4_start;
    let text = strip_tags(&body[h4_start..h4_end]);
    let lat = after(&text, "Latitude:").and_then(|t| parse_dms(&t))?;
    let lon = after(&text, "Longitude:").and_then(|t| parse_dms(&t))?;
    let alt = after(&text, "Altitude:")
        .and_then(|t| t.parse::<f64>().ok())?;
    Some((lat, lon, alt))
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = arg_value(args, "--station")
        .ok_or_else(|| "--station (WMO index, e.g. 72594) required".to_string())?;
    let year = arg_value(args, "--year")
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| "--year (e.g. 2026) required".to_string())?;
    let month = arg_value(args, "--month")
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| "--month (1-12) required".to_string())?;
    let day = arg_value(args, "--day")
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| "--day (1-31) required".to_string())?;
    let ndays = arg_value(args, "--ndays")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1);
    let out = arg_value(args, "--out")
        .ok_or_else(|| "--out (path) required".to_string())?;

    let url = format!(
        "{BASE}?ind={station}&lang=en&decoded=yes&ndays={ndays}&ano={year}&mes={month:02}&day={day:02}&hora=12"
    );
    let body = fetch_raw(&url, None, &[]).ok_or_else(|| format!("{url}: fetch void"))?;
    if body.contains("Unknown station") || body.contains("closed at requested dates") {
        return Err(format!(
            "{station}: unknown or closed at the requested dates — the bin stays unwritten"
        ));
    }
    let (lat, lon, alt) = station_anchor(&body).ok_or_else(|| {
        format!("{station}: no latitude/longitude in the header — the bin stays unwritten")
    })?;

    let mut records = Vec::new();
    let mut rows = 0usize;
    for row in body.split("<tr").skip(1) {
        if !row.contains("decomet") {
            continue;
        }
        let c = cells(row);
        if c.len() < 12 {
            continue;
        }
        let Some((y, m, d)) = parse_mdy(&c[0]) else {
            continue;
        };
        let Some((h, min)) = parse_hm(&c[1]) else {
            continue;
        };
        let Some(unix) = days_from_civil(y, m, d)
            .map(|days| days as f64 * 86400.0 + h as f64 * 3600.0 + min as f64 * 60.0)
        else {
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        rows += 1;
        if let Some(v) = c[2].trim().parse::<f64>().ok() {
            if v.is_finite() {
                records.push(geo_rec(tdb, lat, lon, alt, v, COMP_OGM_TEMP));
            }
        }
        if let Some(v) = c[3].trim().parse::<f64>().ok() {
            if v.is_finite() {
                records.push(geo_rec(tdb, lat, lon, alt, v, COMP_OGM_DEWP));
            }
        }
        if let Some(v) = c[9].trim().parse::<f64>().ok() {
            if v.is_finite() && v >= 0.0 {
                records.push(geo_rec(tdb, lat, lon, alt, v / 3.6, COMP_OGM_WSPD));
            }
        }
        if let Some(v) = c[11].trim().parse::<f64>().ok() {
            if v.is_finite() && v > 0.0 {
                records.push(geo_rec(tdb, lat, lon, alt, v, COMP_OGM_SLP));
            }
        }
    }
    if records.is_empty() {
        return Err(format!(
            "{station}: no measured record left the harvest ({rows} rows) — the bin stays unwritten (0 honored)"
        ));
    }
    let bytes = write_bin(MAGIC_OGM, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_bin(MAGIC_OGM, &bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} synop records ({rows} rows), roundtrip parses",
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

fn geo_rec(
    tdb: f64,
    lat: f64,
    lon: f64,
    alt: f64,
    val: f64,
    comp: u32,
) -> omegaflow::archivar::geo::GeoRec {
    omegaflow::archivar::geo::GeoRec {
        t: tdb,
        lat,
        lon,
        alt,
        freq: 0.0,
        bin_width: 0.0,
        val,
        comp,
        station: 0,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ogimet_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_tags_drops_markup() {
        assert_eq!(
            strip_tags("<TD><font color=\"#000\">13.9</font></TD>"),
            "13.9"
        );
        assert_eq!(strip_tags("<td><i>15.1</i></td>"), "15.1");
    }

    #[test]
    fn dms_reads_hemispheres() {
        assert!((parse_dms("40-47-59N").unwrap() - 40.799722).abs() < 1e-4);
        assert!((parse_dms("124-10-00W").unwrap() - (-124.166667)).abs() < 1e-4);
        assert!(parse_dms("CAL").is_none());
    }

    #[test]
    fn date_and_time_parse() {
        assert_eq!(parse_mdy("09/25/2026"), Some((2026, 9, 25)));
        assert_eq!(parse_hm("11:53"), Some((11, 53)));
        assert!(parse_mdy("----").is_none());
        assert!(parse_hm("CAL").is_none());
    }
}
