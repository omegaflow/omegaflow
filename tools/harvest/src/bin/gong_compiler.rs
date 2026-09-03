use omegaflow::cdn::upload_asset;
use omegaflow::fits::{FitsHeader, FitsImage};
use omegaflow::gong::{parse_bin, write_bin};
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const TSERIES: &str = "https://gong2.nso.edu/ftp/TSERIES";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--retry-delay")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg("240")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn parse_ts(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let h: i64 = s.get(11..13)?.parse().ok()?;
    let mi: i64 = s.get(14..16)?.parse().ok()?;
    let sec: i64 = s.get(17..19)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + sec as f64)
}

fn href_entries(text: &str, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(start) = line.find("href=\"") else {
            continue;
        };
        let rest = &line[start + 6..];
        let Some(end) = rest.find('"') else {
            continue;
        };
        let entry = &rest[..end];
        if entry.starts_with(prefix) {
            out.push(entry.to_string());
        }
    }
    out
}

fn discover_latest_day() -> Option<String> {
    let months = fetch(&format!("{TSERIES}/vmt/"))?;
    let months = href_entries(&String::from_utf8_lossy(&months), "20");
    let month = months
        .iter()
        .filter(|m| m.len() == 7 && m.ends_with('/') && m[..6].bytes().all(|b| b.is_ascii_digit()))
        .max()?
        .clone();
    let days = fetch(&format!("{TSERIES}/vmt/{month}"))?;
    let days = href_entries(&String::from_utf8_lossy(&days), "mrvmt");
    let day = days
        .iter()
        .filter(|d| d.len() == 12 && d.ends_with('/'))
        .max()?
        .clone();
    Some(format!(
        "{}/{}",
        month.trim_end_matches('/'),
        day.trim_end_matches('/')
    ))
}

fn day_dirs(month: &str) -> Option<Vec<String>> {
    let text = fetch(&format!("{TSERIES}/vmt/{month}"))?;
    let mut days: Vec<String> = href_entries(&String::from_utf8_lossy(&text), "mrvmt")
        .into_iter()
        .filter(|d| d.len() == 12 && d.ends_with('/'))
        .map(|d| d.trim_end_matches('/').to_string())
        .collect();
    days.sort();
    Some(days)
}

fn compile_day(
    day_path: &str,
    lsk: &Arc<omegaflow::lsk::LeapSeconds>,
    lmax: u32,
) -> Vec<(u32, i32, f64, f64)> {
    let url = format!("{TSERIES}/vmt/{day_path}/");
    let Some(listing) = fetch(&url) else {
        eprintln!("{day_path}: listing void — the day stays unharvested");
        return Vec::new();
    };
    let files: Vec<String> = href_entries(&String::from_utf8_lossy(&listing), "mrvmt")
        .into_iter()
        .filter(|f| {
            if !f.ends_with(".fits") {
                return false;
            }
            let idx: u32 = f
                .rsplit('d')
                .next()
                .and_then(|s| s.strip_suffix(".fits"))
                .and_then(|s| s.parse().ok())
                .unwrap_or(u32::MAX);
            idx <= lmax
        })
        .collect();
    let out: Arc<Mutex<Vec<(u32, i32, f64, f64)>>> = Arc::new(Mutex::new(Vec::new()));
    let next: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let n_files = files.len();
    let mut workers = Vec::new();
    for _ in 0..2 {
        let out = Arc::clone(&out);
        let next = Arc::clone(&next);
        let files = files.clone();
        let day_path = day_path.to_string();
        let url = url.clone();
        let lsk = Arc::clone(lsk);
        workers.push(std::thread::spawn(move || {
            loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= files.len() {
                    break;
                }
                let f = &files[i];
                std::thread::sleep(std::time::Duration::from_millis(300));
                let Some(buf) = fetch(&format!("{url}{f}")) else {
                    eprintln!("{day_path}/{f}: fetch void");
                    continue;
                };
                let Some(header) = FitsHeader::parse(&buf, 0).map(|(h, _)| h) else {
                    eprintln!("{day_path}/{f}: header parses void");
                    continue;
                };
                let Some(l_value) = header.int("L_VALUE") else {
                    eprintln!("{day_path}/{f}: L_VALUE absent");
                    continue;
                };
                let l = l_value as u32;
                let Some(ts_start) = header.str_unescaped("TS_START").and_then(|s| parse_ts(&s))
                else {
                    eprintln!("{day_path}/{f}: TS_START absent");
                    continue;
                };
                let Some(ts_end) = header.str_unescaped("TS_END").and_then(|s| parse_ts(&s)) else {
                    eprintln!("{day_path}/{f}: TS_END absent");
                    continue;
                };
                let epoch_tdb = match lsk.unix_to_tdb((ts_start + ts_end) * 0.5) {
                    Some(t) => t,
                    None => continue,
                };
                let Some((img, _)) = FitsImage::parse(&buf, 0) else {
                    eprintln!("{day_path}/{f}: image parses void");
                    continue;
                };
                let n_t = img.dims[0];
                let n_slots = img.dims[1];
                let mask_slot = if n_slots > l as usize + 1 {
                    Some(l as usize + 1)
                } else {
                    None
                };
                let mut file_modes: Vec<(u32, i32, f64, f64)> = Vec::new();
                for m in 0..=l {
                    let mut sum_sq = 0.0f64;
                    let mut n = 0usize;
                    for t in 0..n_t {
                        if let Some(ms) = mask_slot {
                            if let Some(mask) = img.value_f64(&buf, [t, ms, 0]) {
                                if mask < 0.5 {
                                    continue;
                                }
                            }
                        }
                        let Some(re) = img.value_f64(&buf, [t, m as usize, 0]) else {
                            continue;
                        };
                        let Some(im) = img.value_f64(&buf, [t, m as usize, 1]) else {
                            continue;
                        };
                        if re.is_finite() && im.is_finite() {
                            sum_sq += re * re + im * im;
                            n += 1;
                        }
                    }
                    if n == 0 {
                        continue;
                    }
                    let rms = (sum_sq / n as f64).sqrt();
                    if rms.is_finite() && rms > 0.0 {
                        file_modes.push((l, m as i32, epoch_tdb, rms));
                    }
                }
                let parsed_total = out.lock().map(|mut g| {
                    g.extend(file_modes);
                    g.len()
                });
                if i % 20 == 0 {
                    eprintln!(
                        "{day_path}: {} of {} FITS parsed, {} modes",
                        i + 1,
                        n_files,
                        parsed_total.unwrap_or(0)
                    );
                }
            }
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let out = Arc::try_unwrap(out)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    eprintln!("{day_path}: {} FITS parsed, {} modes", n_files, out.len());
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "gong_modes.bin".to_string());
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => Arc::new(l),
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let paths: Vec<String> = if let Some(day) = arg_value(&args, "--day") {
        let day = day.trim_end_matches('/').to_string();
        let digits = day.strip_prefix("mrvmt").unwrap_or(&day).to_string();
        let yy: u32 = digits.get(0..2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let century = if yy >= 50 { "19" } else { "20" };
        let month = format!("{}{}", century, digits.get(0..4).unwrap_or_default());
        vec![format!("{month}/mrvmt{digits}")]
    } else if let Some(month) = arg_value(&args, "--month") {
        match day_dirs(&month) {
            Some(days) => days.into_iter().map(|d| format!("{month}/{d}")).collect(),
            None => {
                eprintln!("--month {month}: listing void");
                std::process::exit(1);
            }
        }
    } else {
        match discover_latest_day() {
            Some(path) => vec![path],
            None => {
                eprintln!("vmt index void — no month/day directory found (--day to declare one)");
                std::process::exit(1);
            }
        }
    };
    let mut latest: HashMap<(u32, i32), (f64, f64)> = HashMap::new();
    let lmax: u32 = arg_value(&args, "--lmax")
        .and_then(|v| v.parse().ok())
        .unwrap_or(200);
    for path in &paths {
        for (l, m, epoch, rms) in compile_day(path, &lsk, lmax) {
            match latest.get(&(l, m)) {
                Some((old_epoch, _)) if *old_epoch >= epoch => {}
                _ => {
                    latest.insert((l, m), (epoch, rms));
                }
            }
        }
    }
    let mut modes: Vec<(u32, i32, f64, f64)> = latest
        .into_iter()
        .map(|((l, m), (t, rms))| (l, m, t, rms))
        .collect();
    modes.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    eprintln!(
        "vmt {} day-set(s): {} modes (L 0-200, |m|<=L, rms over the 36-day FITS window)",
        paths.len(),
        modes.len()
    );
    if modes.is_empty() {
        eprintln!("no modes — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&modes);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{}: {} modes, roundtrip parses ({} B)",
                out,
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
