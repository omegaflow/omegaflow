use omegaflow::archivar::geo::{
    parse_bin, write_bin, GeoRec, COMP_HINET_E, COMP_HINET_N, COMP_HINET_U, MAGIC_HINET,
};
use omegaflow::archivar::win32::{parse_win32, station_of};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{parse as parse_lsk, LeapSeconds};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const HINET_BASE: &str = "https://hinetwww11.bosai.go.jp";
const HINET_NETLOC: &str = "hinetwww11.bosai.go.jp";
const HINET_USER: &str = "omegaflow";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn comp_of_letter(c: &str) -> Option<u32> {
    match c {
        "U" => Some(COMP_HINET_U),
        "E" => Some(COMP_HINET_E),
        "N" => Some(COMP_HINET_N),
        _ => None,
    }
}

struct ChannelAnchor {
    lat: f64,
    lon: f64,
    alt: f64,
    comp: u32,
}

fn parse_finite(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn load_channels(path: &str) -> (HashMap<u16, ChannelAnchor>, usize, usize) {
    let mut map = HashMap::new();
    let mut rows = 0usize;
    let mut rejected = 0usize;
    let body = match std::fs::read_to_string(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("hinet: channel table {path} stayed unread");
            return (map, 0, 0);
        }
    };
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = t.split('|').collect();
        if cols.len() < 5 {
            rejected += 1;
            continue;
        }
        let Some(chan) = u16::from_str_radix(cols[0].trim(), 16).ok() else {
            rejected += 1;
            continue;
        };
        let (Some(lat), Some(lon), Some(alt), Some(comp)) = (
            parse_finite(cols[1]),
            parse_finite(cols[2]),
            parse_finite(cols[3]),
            comp_of_letter(cols[4].trim()),
        ) else {
            rejected += 1;
            continue;
        };
        rows += 1;
        map.insert(
            chan,
            ChannelAnchor {
                lat,
                lon,
                alt,
                comp,
            },
        );
    }
    (map, rows, rejected)
}

fn walk_local(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_local(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("cnt") {
            out.push(path);
        }
    }
}

fn curl_get(url: &str, jar: &Option<PathBuf>) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-L")
        .arg("-m")
        .arg("300")
        .arg("--connect-timeout")
        .arg("20");
    if let Some(j) = jar {
        cmd.arg("-b").arg(j).arg("-c").arg(j);
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!("hinet: {url} returned void");
        None
    }
}

fn curl_form(url: &str, jar: &Path, form: &[(String, String)]) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-L")
        .arg("-m")
        .arg("120")
        .arg("-b")
        .arg(jar)
        .arg("-c")
        .arg(jar);
    for (k, v) in form {
        cmd.arg("--data-urlencode").arg(format!("{k}={v}"));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!("hinet: {url} returned void");
        None
    }
}

fn auth(jar: &Path) -> bool {
    let Some(pass) = secret("HINET_PASS") else {
        eprintln!("hinet: HINET_PASS absent — no credential, the session stays closed");
        return false;
    };
    let url = format!("{HINET_BASE}/auth/?LANG=en");
    let form = vec![
        ("auth_un".to_string(), HINET_USER.to_string()),
        ("auth_pw".to_string(), pass),
    ];
    match curl_form(&url, jar, &form) {
        Some(_) => {
            eprintln!("hinet: auth session opened at {url}");
            true
        }
        None => {
            eprintln!("hinet: auth session stayed closed at {url}");
            false
        }
    }
}

fn select_stations(jar: &Path, stations: &[String]) -> bool {
    let check = format!("{HINET_BASE}/select_check.cgi?LANG=en");
    let form: Vec<(String, String)> = stations
        .iter()
        .map(|s| ("station".to_string(), s.clone()))
        .collect();
    if curl_form(&check, jar, &form).is_none() {
        eprintln!("hinet: station selection stayed unread at {check}");
        return false;
    }
    let confirm = format!("{HINET_BASE}/select_confirm.php?LANG=en");
    if curl_get(&confirm, &Some(jar.to_path_buf())).is_none() {
        eprintln!("hinet: station confirm stayed unread at {confirm}");
        return false;
    }
    eprintln!(
        "hinet: {} stations registered through the select flow",
        stations.len()
    );
    true
}

struct Window {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
}

fn parse_window(s: &str) -> Option<Window> {
    let t = s.trim();
    let date = t.split('T').next()?;
    let time = match t.split('T').nth(1) {
        Some(x) => x,
        None => "00:00",
    };
    let dp: Vec<&str> = date.split('-').collect();
    let tp: Vec<&str> = time.split(':').collect();
    if dp.len() < 3 || tp.len() < 2 {
        return None;
    }
    let year: i32 = dp[0].parse().ok()?;
    let month: u32 = dp[1].parse().ok()?;
    let day: u32 = dp[2].parse().ok()?;
    let hour: u32 = tp[0].parse().ok()?;
    let min: u32 = tp[1].parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || min > 59 {
        return None;
    }
    Some(Window {
        year,
        month,
        day,
        hour,
        min,
    })
}

fn request_waveform(jar: &Path, w: &Window, span_min: u32) -> Option<Vec<u8>> {
    let url = format!(
        "{HINET_BASE}/cont_request.php?org1=&org2=&year={}&month={}&day={}&hour={}&min={}&span={}&arc=&size=&LANG=en&volc=&rn=",
        w.year, w.month, w.day, w.hour, w.min, span_min
    );
    let body = curl_get(&url, &Some(jar.to_path_buf()))?;
    eprintln!("hinet: cont request placed at {url}");
    Some(body)
}

fn poll_status(jar: &Path, rn: &str) -> Option<String> {
    let url = format!("{HINET_BASE}/cont_status.php?LANG=en&rn={rn}");
    let body = curl_get(&url, &Some(jar.to_path_buf()))?;
    Some(String::from_utf8_lossy(&body).to_string())
}

fn compile_file(
    path: &Path,
    channels: &HashMap<u16, ChannelAnchor>,
    lsk: &LeapSeconds,
    tz_offset: f64,
    records: &mut Vec<GeoRec>,
) -> usize {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("hinet: {} stayed unread", path.display());
            return 0;
        }
    };
    let Some(samples) = parse_win32(&bytes) else {
        eprintln!(
            "hinet: {} parses void — not a WIN32 .cnt block stream",
            path.display()
        );
        return 0;
    };
    let mut n = 0usize;
    for s in samples {
        let Some(anchor) = channels.get(&s.chan) else {
            continue;
        };
        let Some(t) = lsk.unix_to_tdb(s.t - tz_offset) else {
            continue;
        };
        records.push(GeoRec {
            t,
            lat: anchor.lat,
            lon: anchor.lon,
            alt: anchor.alt,
            freq: 0.0,
            bin_width: 0.0,
            val: s.val as f64,
            comp: anchor.comp,
            station: station_of(s.chan),
        });
        n += 1;
    }
    n
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_bin = arg_value(&args, "--out-bin");
    let channels_path = arg_value(&args, "--channels");
    let tz_offset: f64 = match arg_value(&args, "--tz-offset") {
        None => 0.0,
        Some(v) => match v.parse::<f64>() {
            Ok(x) if x.is_finite() => x,
            _ => {
                eprintln!("hinet: --tz-offset {v} carries no measured shift");
                std::process::exit(1);
            }
        },
    };

    let mut sources: Vec<PathBuf> = Vec::new();
    if let Some(dir) = arg_value(&args, "--in") {
        walk_local(Path::new(&dir), &mut sources);
        sources.sort();
    } else {
        let Some(_pass) = secret("HINET_PASS") else {
            eprintln!("hinet: HINET_PASS absent — no credential, the .cnt tree stays unfetched");
            std::process::exit(1);
        };
        let cache = match arg_value(&args, "--cache") {
            Some(v) => v,
            None => "data/hinetwww11.bosai.go.jp".to_string(),
        };
        let cache_path = PathBuf::from(&cache);
        let _ = std::fs::create_dir_all(&cache_path);
        let jar = cache_path.join("cookies.txt");
        if !auth(&jar) {
            std::process::exit(1);
        }
        let station_csv = match arg_value(&args, "--stations") {
            Some(v) => v,
            None => String::new(),
        };
        let stations: Vec<String> = station_csv
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        if !stations.is_empty() && !select_stations(&jar, &stations) {
            std::process::exit(1);
        }
        let span_min: u32 = match arg_value(&args, "--span") {
            Some(v) => match v.parse() {
                Ok(x) => x,
                Err(_) => {
                    eprintln!("hinet: --span {v} carries no measured span");
                    std::process::exit(1);
                }
            },
            None => 60,
        };
        let Some(w) = arg_value(&args, "--start").and_then(|s| parse_window(&s)) else {
            eprintln!("hinet: --start YYYY-MM-DDTHH:MM names the request window");
            std::process::exit(1);
        };
        let rn = match arg_value(&args, "--rn") {
            Some(v) => v,
            None => "1".to_string(),
        };
        let Some(body) = request_waveform(&jar, &w, span_min) else {
            eprintln!("hinet: cont request returned void — no .cnt to fetch");
            std::process::exit(1);
        };
        let cnt_path = cache_path.join(format!("{rn}.cnt"));
        if std::fs::write(&cnt_path, &body).is_err() {
            eprintln!("hinet: write {} returned void", cnt_path.display());
            std::process::exit(1);
        }
        sources.push(cnt_path);
        let status = match poll_status(&jar, &rn) {
            Some(s) => s,
            None => String::new(),
        };
        eprintln!(
            "hinet: request {rn} window {}-{:02}-{:02}T{:02}:{:02} span {span_min} min — status {} bytes",
            w.year, w.month, w.day, w.hour, w.min, status.len()
        );
    }

    if sources.is_empty() {
        eprintln!("hinet: no .cnt files flow — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }

    let (Some(out_bin), Some(channels_path)) = (out_bin, channels_path) else {
        eprintln!("hinet: --out-bin <path> and --channels <file> are the required compile pair");
        std::process::exit(1);
    };
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("hinet: --lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("hinet: --lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let (channels, ch_rows, ch_rejected) = load_channels(&channels_path);
    if channels.is_empty() {
        eprintln!(
            "hinet: channel table {channels_path} carries no anchors — the join stays unread"
        );
        std::process::exit(1);
    }

    let mut records: Vec<GeoRec> = Vec::new();
    let mut total = 0usize;
    for src in &sources {
        let n = compile_file(src, &channels, &lsk, tz_offset, &mut records);
        if n == 0 {
            eprintln!(
                "hinet: {} carries no matched channel samples — file skipped",
                src.display()
            );
        }
        total += n;
    }

    if records.is_empty() {
        eprintln!("hinet: no matched channel samples — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));

    let bytes = write_bin(MAGIC_HINET, &records);
    if std::fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {out_bin} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_HINET, &bytes) {
        Some(parsed) => eprintln!(
            "hinet: {}: {} geo records ({} samples, {} channel rows, {} rejected), {} B, roundtrip parses",
            out_bin,
            parsed.len(),
            total,
            ch_rows,
            ch_rejected,
            bytes.len()
        ),
        None => {
            eprintln!("hinet: {out_bin} roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(HINET_NETLOC, &out_bin) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::win32::chan_of;

    #[test]
    fn channel_table_parses_hex_anchor_and_component() {
        let dir = std::env::temp_dir().join(format!("hinet_chan_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join("ch.txt");
        let text = "# chanid|lat|lon|elev|comp\n0101|35.0|139.0|800.0|U\n0102|35.1|139.1|800.0|N\n0103|35.2|139.2|800.0|E\nzzzz|1|2|3|U\n";
        std::fs::write(&p, text).unwrap();
        let (map, rows, rejected) = load_channels(p.to_str().unwrap());
        assert_eq!(rows, 3);
        assert_eq!(rejected, 1);
        assert_eq!(map[&0x0101u16].comp, COMP_HINET_U);
        assert_eq!(map[&0x0102u16].comp, COMP_HINET_N);
        assert_eq!(map[&0x0103u16].comp, COMP_HINET_E);
        assert!(!map.contains_key(&0x0104u16));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn comp_letters_map_to_hinet_components() {
        assert_eq!(comp_of_letter("U"), Some(COMP_HINET_U));
        assert_eq!(comp_of_letter("E"), Some(COMP_HINET_E));
        assert_eq!(comp_of_letter("N"), Some(COMP_HINET_N));
        assert_eq!(comp_of_letter("H"), None);
        assert_eq!(comp_of_letter(""), None);
    }

    #[test]
    fn window_parses_iso_fragment() {
        let w = parse_window("2024-05-04T13:05").expect("window parses");
        assert_eq!((w.year, w.month, w.day, w.hour, w.min), (2024, 5, 4, 13, 5));
        assert!(parse_window("2024-13-04T13:05").is_none());
        assert!(parse_window("2024-05-04T25:00").is_none());
        assert!(parse_window("garbage").is_none());
    }

    #[test]
    fn compile_maps_win32_samples_into_georecords() {
        let dir = std::env::temp_dir().join(format!("hinet_cmp_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join("f.cnt");
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0, 0, 0, 0]);
        let mut header = [0u8; 16];
        header[0] = 0x20;
        header[1] = 0x20;
        header[2] = 0x01;
        header[3] = 0x01;
        header[4] = 0x00;
        header[5] = 0x00;
        header[6] = 0x00;
        let packet = [
            0x00u8, 0x01, 0x01, 0x01, 0x10, 0x05, 0x00, 0x00, 0x03, 0xE8, 0x01, 0xFE, 0x03, 0xFC,
        ];
        let sz = packet.len() as u32;
        header[12] = (sz >> 24) as u8;
        header[13] = (sz >> 16) as u8;
        header[14] = (sz >> 8) as u8;
        header[15] = sz as u8;
        buf.extend_from_slice(&header);
        buf.extend_from_slice(&packet);
        std::fs::write(&p, &buf).unwrap();

        let mut channels = HashMap::new();
        channels.insert(
            0x0101u16,
            ChannelAnchor {
                lat: 35.0,
                lon: 139.0,
                alt: 800.0,
                comp: COMP_HINET_U,
            },
        );
        let lsk_text = "KPL/LSK\n\
[2]       DELTA_AT  =  TAI - UTC\n\
\\begindata\n\n\
DELTET/DELTA_T_A       =   32.184\n\
DELTET/DELTA_AT        = ( 10,   @1972-JAN-1,\n 37,   @2017-JAN-1 )\n";
        let lsk = parse_lsk(lsk_text).expect("lsk parses");
        let mut records = Vec::new();
        let n = compile_file(&p, &channels, &lsk, 0.0, &mut records);
        assert_eq!(n, 5);
        assert_eq!(records.len(), 5);
        assert_eq!(records[0].val, 1000.0);
        assert_eq!(records[0].comp, COMP_HINET_U);
        assert_eq!(chan_of(records[0].station), 0x0101);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
