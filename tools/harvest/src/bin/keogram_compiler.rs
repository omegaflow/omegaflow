use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::keogram::{brightness_columns, parse_bin, write_bin};
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const HOST: &str = "space.fmi.fi";
const BASE: &str = "https://space.fmi.fi/MIRACLE/ASC/ASC_keograms";
const DEFAULT_STATION: &str = "KIL";

const STATIONS: &[(&str, f64, f64)] = &[
    ("ABK", 68.36, 18.82),
    ("KEV", 69.76, 27.01),
    ("KIL", 69.05, 20.78),
    ("MUO", 68.02, 23.53),
    ("NAL", 78.92, 11.93),
    ("SOD", 67.42, 26.39),
    ("HAN", 62.30, 26.65),
    ("NYR", 62.34, 25.51),
    ("LYR", 78.15, 16.04),
];

const DEG: f64 = std::f64::consts::PI / 180.0;
const JD_UNIX_EPOCH: f64 = 2440587.5;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_date(s: &str) -> Option<i64> {
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let (cy, cm, cd) = civil_from_days(days)?;
    if cy as i64 != year || cm as i64 != month || cd as i64 != day {
        return None;
    }
    Some(days)
}

fn is_jpeg(bytes: &[u8]) -> bool {
    bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xD8
}

fn looks_like_html(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(64)];
    let text = String::from_utf8_lossy(head).to_ascii_lowercase();
    text.contains("<!doctype") || text.contains("<html") || text.contains("404")
}

fn decode_image_body(body: &[u8]) -> Result<omegaflow::keogram::KeogramColumns, String> {
    if !is_jpeg(body) {
        let kind = if looks_like_html(body) {
            "HTML/404 body"
        } else {
            "JPEG magic FF D8 absent"
        };
        return Err(format!("no image — {kind} ({} B)", body.len()));
    }
    brightness_columns(body)
        .ok_or_else(|| format!("no image — JPEG stream does not decode ({} B)", body.len()))
}

fn station_coords(station: &str) -> Option<(f64, f64)> {
    STATIONS
        .iter()
        .find(|(code, _, _)| *code == station)
        .map(|(_, lat, lon)| (*lat, *lon))
}

fn minutes_axis(width: usize, night_minutes: f64) -> Option<Vec<f64>> {
    if width == 0 || !night_minutes.is_finite() || night_minutes <= 0.0 {
        return None;
    }
    let cadence_min = night_minutes / width as f64;
    Some((0..width).map(|i| i as f64 * cadence_min).collect())
}

fn unix_from_jd(jd: f64) -> f64 {
    (jd - JD_UNIX_EPOCH) * 86400.0
}

fn solar_rise_set(day: i64, lat_deg: f64, lon_deg: f64) -> Option<(f64, f64)> {
    let n = (day - days_from_civil(2000, 1, 1)?) as f64;
    let j_star = n + 0.0008 - lon_deg / 360.0;
    let m = (357.5291 + 0.98560028 * j_star).rem_euclid(360.0);
    let c =
        1.9148 * (m * DEG).sin() + 0.0200 * (2.0 * m * DEG).sin() + 0.0003 * (3.0 * m * DEG).sin();
    let lambda = (m + c + 180.0 + 102.9372).rem_euclid(360.0);
    let j_transit =
        2451545.0 + j_star + 0.0053 * (m * DEG).sin() - 0.0069 * (2.0 * lambda * DEG).sin();
    let decl = ((lambda * DEG).sin() * (23.4397 * DEG).sin()).asin();
    let lat = lat_deg * DEG;
    let cos_omega = ((-0.833 * DEG).sin() - lat.sin() * decl.sin()) / (lat.cos() * decl.cos());
    if !cos_omega.is_finite() || !(-1.0..=1.0).contains(&cos_omega) {
        return None;
    }
    let omega = cos_omega.acos() / DEG;
    let rise = unix_from_jd(j_transit - omega / 360.0);
    let set = unix_from_jd(j_transit + omega / 360.0);
    Some((rise, set))
}

fn night_window(day: i64, station: &str) -> Option<(f64, f64)> {
    let (lat, lon) = station_coords(station)?;
    let (_, sunset) = solar_rise_set(day, lat, lon)?;
    let (sunrise, _) = solar_rise_set(day + 1, lat, lon)?;
    if !sunrise.is_finite() || !sunset.is_finite() || sunrise <= sunset {
        return None;
    }
    Some((sunset, sunrise))
}

fn night_rows(day: i64, station: &str) -> Vec<(f64, u32, f64)> {
    let Some((year, month, mday)) = civil_from_days(day) else {
        return Vec::new();
    };
    let yy = year % 100;
    let dir = format!("{station}.{yy:02}{month:02}");
    let file = format!("{station}_{yy:02}{month:02}{mday:02}.jpg");
    let url = format!("{BASE}/{dir}/{file}");
    let Some(jpeg) = fetch_raw_bytes(&url) else {
        eprintln!("{file} fetch void — night skipped (0 honored)");
        return Vec::new();
    };
    let cols = match decode_image_body(&jpeg) {
        Ok(cols) => cols,
        Err(msg) => {
            eprintln!("{file}: {msg} — night skipped (0 honored)");
            return Vec::new();
        }
    };
    let window = night_window(day, station);
    let axis = window.and_then(|(start, end)| {
        let night_minutes = (end - start) / 60.0;
        minutes_axis(cols.width, night_minutes).map(|mins| (start, mins, night_minutes))
    });
    let t_unix = day as f64 * 86400.0;
    let mut rows = Vec::new();
    for (i, &m) in cols.mean.iter().enumerate() {
        if !m.is_finite() {
            continue;
        }
        let t = match &axis {
            Some((start, mins, _)) => start + mins[i] * 60.0,
            None => t_unix,
        };
        rows.push((t, i as u32, m));
    }
    eprintln!(
        "{file}: {}x{} -> {} columns",
        cols.width,
        cols.height,
        rows.len()
    );
    match axis {
        Some((_, _, night_minutes)) => eprintln!(
            "{file}: time axis {:.3} min/column over {:.1} min (sunset→sunrise)",
            night_minutes / cols.width as f64,
            night_minutes
        ),
        None => eprintln!(
            "{file}: time axis absent — no sunset/sunrise for {station} on {year:04}-{month:02}-{mday:02}; columns carry the day start only"
        ),
    }
    rows
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = match arg_value(&args, "--station") {
        Some(s) => s.to_uppercase(),
        None => {
            eprintln!("--station absent — defaulting to {DEFAULT_STATION} (2026-active FMI ASC)");
            DEFAULT_STATION.to_string()
        }
    };
    let (start, stop) = match (arg_value(&args, "--start"), arg_value(&args, "--stop")) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!("keogram_compiler needs --start <YYYY-MM-DD> and --stop <YYYY-MM-DD>");
            std::process::exit(2);
        }
    };
    let (start_day, stop_day) = match (parse_date(&start), parse_date(&stop)) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!("--start/--stop must be YYYY-MM-DD");
            std::process::exit(2);
        }
    };
    if stop_day < start_day {
        eprintln!("--stop must not lie before --start");
        std::process::exit(2);
    }
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => format!("keogram_{station}.bin"),
    };
    let lookback_days: i64 = arg_value(&args, "--lookback-days")
        .and_then(|v| v.parse().ok())
        .unwrap_or(400);

    let mut rows: Vec<(f64, u32, f64)> = Vec::new();
    for day in start_day..=stop_day {
        rows.extend(night_rows(day, &station));
    }
    if rows.is_empty() && lookback_days > 0 {
        let floor = stop_day.saturating_sub(lookback_days);
        let mut day = stop_day - 1;
        while day >= floor {
            let night = night_rows(day, &station);
            if !night.is_empty() {
                eprintln!("no column in {start}..{stop}; the newest available night is {day}");
                rows = night;
                break;
            }
            day -= 1;
        }
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    if rows.is_empty() {
        eprintln!("no columns across {start}..{stop} — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&rows);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} columns, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(HOST, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn htmlish_404_fixture() -> Vec<u8> {
        let mut body = b"<!DOCTYPE HTML PUBLIC \"-//IETF//DTD HTML 2.0//EN\">\n\
<html><head><title>404 Not Found</title></head>\n\
<body><h1>Not Found</h1><p>The requested URL was not found on this server.</p></body></html>"
            .to_vec();
        body.resize(273, b' ');
        body
    }

    #[test]
    fn a_200_html_body_is_named_and_rejected() {
        let body = htmlish_404_fixture();
        assert_eq!(body.len(), 273);
        assert!(!is_jpeg(&body));
        let err = decode_image_body(&body)
            .err()
            .expect("HTML is not an image");
        assert!(err.contains("no image"), "{err}");
        assert!(err.contains("HTML/404 body"), "{err}");
    }

    #[test]
    fn the_jpeg_magic_is_the_gate() {
        assert!(is_jpeg(&[0xFF, 0xD8, 0xFF, 0xE0]));
        assert!(!is_jpeg(&[0xFF, 0xD9]));
        assert!(!is_jpeg(&[]));
        let err = decode_image_body(&[0x00, 0x01, 0x02, 0x03])
            .err()
            .expect("a non-JPEG body is not an image");
        assert!(err.contains("JPEG magic FF D8 absent"), "{err}");
    }

    #[test]
    fn minutes_axis_uses_the_fixed_width_and_night_length() {
        let axis = minutes_axis(4, 480.0).expect("a fixed night has an axis");
        assert_eq!(axis, vec![0.0, 120.0, 240.0, 360.0]);
        assert!(minutes_axis(0, 480.0).is_none());
        assert!(minutes_axis(4, 0.0).is_none());
        assert!(minutes_axis(4, f64::NAN).is_none());
    }

    #[test]
    fn the_kil_night_window_runs_from_evening_to_the_next_morning() {
        let day = days_from_civil(2026, 4, 19).expect("valid date");
        let (sunset, sunrise) = night_window(day, "KIL").expect("KIL has a night");
        let hours = (sunrise - sunset) / 3600.0;
        assert!(hours > 4.0 && hours < 16.0, "night length {hours} h");
        let axis_minutes = (sunrise - sunset) / 60.0;
        let axis = minutes_axis(800, axis_minutes).expect("axis exists");
        assert_eq!(axis.len(), 800);
        assert_eq!(axis[0], 0.0);
        assert!((axis[799] - axis_minutes * 799.0 / 800.0).abs() < 1e-9);
    }

    #[test]
    fn an_unknown_station_has_no_time_axis() {
        let day = days_from_civil(2026, 4, 19).expect("valid date");
        assert!(night_window(day, "ZZZ").is_none());
    }
}
