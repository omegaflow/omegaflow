use omegaflow::lsk::days_from_civil;
use omegaflow_measure::idl_save;
use std::path::{Path, PathBuf};

const FRAME_W: usize = 256;
const FRAME_H: usize = 256;
const FRAME_BYTES: usize = FRAME_W * FRAME_H * 2;

const MAGIC: &[u8; 4] = b"FODG";
const DAY_S: f64 = 86400.0;
const HOUR_S: f64 = 3600.0;
const PGM_MAXVAL: u32 = 65535;

const DEFAULT_AZ_BINS: usize = 36;
const DEFAULT_EL_BINS: usize = 9;
const DEFAULT_THRESHOLD_K: f64 = 8.0;
const DEFAULT_PERSIST: usize = 3;
const DEFAULT_WINDOW: usize = 120;

const ORIENTATION: &str = "raw stream0 row0=north, col0=east, west-winding columns; the per-station skymap gives each pixel its azimuth (deg, 0=N clockwise) and elevation (deg above horizon); a grid cell is the mean brightness of its pixels";

#[derive(Debug)]
struct Binned {
    epoch: f64,
    cells: Vec<f32>,
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (y, m, d)
}

fn iso(epoch: f64) -> String {
    let days = (epoch / DAY_S).floor() as i64;
    let secs = epoch - days as f64 * DAY_S;
    let (y, m, d) = civil_from_days(days);
    let hh = (secs / HOUR_S) as i64;
    let mm = ((secs - hh as f64 * HOUR_S) / 60.0) as i64;
    let ss = secs - hh as f64 * HOUR_S - mm as f64 * 60.0;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:06.3}Z")
}

fn parse_utc(line: &str) -> Option<f64> {
    if !line.contains("Image request start") {
        return None;
    }
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks.len() < 3 || toks[toks.len() - 1] != "UTC" {
        return None;
    }
    let date = toks[toks.len() - 3];
    let time = toks[toks.len() - 2];
    let mut d = date.split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let ss: f64 = t.next()?.parse().ok()?;
    if !(1..=12).contains(&mo) || !(1..=31).contains(&da) {
        return None;
    }
    let days = days_from_civil(y, mo, da)?;
    Some(days as f64 * DAY_S + hh as f64 * HOUR_S + mm as f64 * 60.0 + ss)
}

fn parse_iso(s: &str) -> Option<f64> {
    let body = s.trim().trim_end_matches('Z');
    let (date, time) = body.split_once(['T', ' '])?;
    let mut d = date.split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let ss: f64 = t.next().unwrap_or("0").parse().ok()?;
    if !(1..=12).contains(&mo) || !(1..=31).contains(&da) {
        return None;
    }
    let days = days_from_civil(y, mo, da)?;
    Some(days as f64 * DAY_S + hh as f64 * HOUR_S + mm as f64 * 60.0 + ss)
}

fn read_line(data: &[u8], pos: &mut usize, name: &str, index: usize) -> Result<String, String> {
    let start = *pos;
    let mut end = start;
    while end < data.len() && data[end] != b'\n' {
        end += 1;
    }
    if end >= data.len() {
        return Err(format!(
            "{name}: sub-frame {index} header line unterminated at byte {start}"
        ));
    }
    let line = &data[start..end];
    *pos = end + 1;
    String::from_utf8(line.to_vec())
        .map_err(|_| format!("{name}: sub-frame {index} header line is not UTF-8 at byte {start}"))
}

fn bin_map(skymap: &idl_save::Skymap, az_bins: usize, el_bins: usize) -> Vec<Option<u32>> {
    let mut map = vec![None; skymap.full_w * skymap.full_h];
    for (i, slot) in map.iter_mut().enumerate() {
        let az = skymap.full_azimuth[i] as f64;
        let el = skymap.full_elevation[i] as f64;
        if !az.is_finite() || !el.is_finite() || el < 0.0 {
            continue;
        }
        let ab =
            ((az.rem_euclid(360.0) / 360.0 * az_bins as f64).floor() as usize).min(az_bins - 1);
        let eb = ((el / 90.0 * el_bins as f64).floor() as usize).min(el_bins - 1);
        *slot = Some((ab * el_bins + eb) as u32);
    }
    map
}

fn frame_cells(block: &[u8], map: &[Option<u32>], cells: usize) -> Vec<f32> {
    let mut sums = vec![0u64; cells];
    let mut counts = vec![0u32; cells];
    for (i, slot) in map.iter().enumerate() {
        if let Some(c) = *slot {
            let v = ((block[i * 2] as u16) << 8) | block[i * 2 + 1] as u16;
            sums[c as usize] += v as u64;
            counts[c as usize] += 1;
        }
    }
    (0..cells)
        .map(|c| {
            if counts[c] > 0 {
                sums[c] as f64 / counts[c] as f64
            } else {
                f64::NAN
            }
        })
        .map(|v| v as f32)
        .collect()
}

fn parse_frames(
    data: &[u8],
    name: &str,
    map: &[Option<u32>],
    cells: usize,
) -> Result<Vec<Binned>, String> {
    let mut frames = Vec::new();
    let mut pos = 0usize;
    let mut index = 0usize;
    while pos < data.len() {
        index += 1;
        if data.len() - pos < 2 || &data[pos..pos + 2] != b"P5" {
            return Err(format!(
                "{name}: sub-frame {index} does not begin with P5 at byte {pos}"
            ));
        }
        pos += 2;
        if pos < data.len() && data[pos] == b'\n' {
            pos += 1;
        }
        let mut dims: Option<(usize, usize)> = None;
        let mut maxval: Option<u32> = None;
        let mut stamp: Option<f64> = None;
        while maxval.is_none() {
            let line = read_line(data, &mut pos, name, index)?;
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                if let Some(t) = parse_utc(trimmed) {
                    stamp = Some(t);
                }
                continue;
            }
            if trimmed.is_empty() {
                continue;
            }
            if dims.is_none() {
                let mut it = trimmed.split_whitespace();
                let w: usize = it.next().and_then(|v| v.parse().ok()).ok_or_else(|| {
                    format!("{name}: sub-frame {index} dimensions line unreadable: {trimmed}")
                })?;
                let h: usize = it.next().and_then(|v| v.parse().ok()).ok_or_else(|| {
                    format!("{name}: sub-frame {index} dimensions line unreadable: {trimmed}")
                })?;
                dims = Some((w, h));
            } else {
                let v: u32 = trimmed.parse().map_err(|_| {
                    format!("{name}: sub-frame {index} maxval unreadable: {trimmed}")
                })?;
                maxval = Some(v);
            }
        }
        let (w, h) =
            dims.ok_or_else(|| format!("{name}: sub-frame {index} carries no dimensions"))?;
        let mv = maxval.ok_or_else(|| format!("{name}: sub-frame {index} carries no maxval"))?;
        if w != FRAME_W || h != FRAME_H || mv != PGM_MAXVAL {
            return Err(format!(
                "{name}: sub-frame {index} geometry {w}x{h} maxval {mv} differs from {FRAME_W}x{FRAME_H} maxval {PGM_MAXVAL}"
            ));
        }
        let epoch = stamp.ok_or_else(|| {
            format!("{name}: sub-frame {index} carries no `Image request start` UTC stamp")
        })?;
        if data.len() - pos < FRAME_BYTES {
            return Err(format!(
                "{name}: sub-frame {index} pixel block truncated: {} of {FRAME_BYTES} bytes",
                data.len() - pos
            ));
        }
        let cells_vec = frame_cells(&data[pos..pos + FRAME_BYTES], map, cells);
        pos += FRAME_BYTES;
        frames.push(Binned {
            epoch,
            cells: cells_vec,
        });
    }
    Ok(frames)
}

fn write_fodg(frames: &[Binned], az_bins: usize, el_bins: usize) -> Vec<u8> {
    let cells = az_bins * el_bins;
    let mut out = Vec::with_capacity(16 + frames.len() * (8 + cells * 4));
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(az_bins as u32).to_le_bytes());
    out.extend_from_slice(&(el_bins as u32).to_le_bytes());
    out.extend_from_slice(&(frames.len() as u32).to_le_bytes());
    for f in frames {
        out.extend_from_slice(&f.epoch.to_le_bytes());
        for v in &f.cells {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn collect_inputs(args: &[String]) -> Result<Vec<PathBuf>, String> {
    let mut inputs: Vec<PathBuf> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--in" {
            match args.get(i + 1) {
                Some(v) => {
                    inputs.push(PathBuf::from(v));
                    i += 2;
                    continue;
                }
                None => return Err("--in needs a path".to_string()),
            }
        }
        i += 1;
    }
    if inputs.is_empty() {
        if let Some(dir) = arg_value(args, "--dir") {
            let entries =
                std::fs::read_dir(&dir).map_err(|e| format!("--dir {dir} unreadable: {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("--dir {dir} entry unreadable: {e}"))?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("pgm") {
                    inputs.push(path);
                }
            }
            inputs.sort();
        }
    }
    if inputs.is_empty() {
        return Err("needs --in <file.pgm> (repeatable) or --dir <dir> holding *.pgm".to_string());
    }
    Ok(inputs)
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

struct Onset {
    epoch: f64,
    value: f64,
    baseline: f64,
    threshold: f64,
}

fn running_onset(
    series: &[(f64, f64)],
    window: usize,
    k: f64,
    persist: usize,
    search_from: f64,
) -> Option<Onset> {
    if series.len() <= window {
        return None;
    }
    let mut run = 0usize;
    for i in window..series.len() {
        let (epoch, value) = series[i];
        if epoch < search_from || !value.is_finite() {
            run = 0;
            continue;
        }
        let mut base: Vec<f64> = series[i - window..i]
            .iter()
            .map(|(_, v)| *v)
            .filter(|v| v.is_finite())
            .collect();
        if base.len() < 5 {
            run = 0;
            continue;
        }
        let med = median(&mut base);
        let mut dev: Vec<f64> = base.iter().map(|v| (v - med).abs()).collect();
        let mad = median(&mut dev);
        let sigma = 1.4826 * mad;
        let threshold = med + k * sigma;
        if value > threshold {
            run += 1;
            if run >= persist {
                let start = i + 1 - run;
                let (se, sv) = series[start];
                return Some(Onset {
                    epoch: se,
                    value: sv,
                    baseline: med,
                    threshold,
                });
            }
        } else {
            run = 0;
        }
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out_path = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!("themis_asi_frame_onset needs --out <file.bin>");
            std::process::exit(2);
        }
    };
    let skymap_path = match arg_value(&args, "--skymap") {
        Some(p) => p,
        None => {
            eprintln!("themis_asi_frame_onset needs --skymap <path.sav> (the az/el surface)");
            std::process::exit(2);
        }
    };
    let az_bins = arg_value(&args, "--az-bins")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_AZ_BINS);
    let el_bins = arg_value(&args, "--el-bins")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_EL_BINS);
    if az_bins == 0 || el_bins == 0 {
        eprintln!("themis_asi_frame_onset: --az-bins/--el-bins must be positive");
        std::process::exit(2);
    }
    let threshold_k = arg_value(&args, "--threshold-k")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_THRESHOLD_K);
    let persist = arg_value(&args, "--persist")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_PERSIST);
    if persist == 0 {
        eprintln!("themis_asi_frame_onset: --persist must be positive");
        std::process::exit(2);
    }
    let window = arg_value(&args, "--window")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_WINDOW);
    if window < 5 {
        eprintln!("themis_asi_frame_onset: --window must be at least 5 frames");
        std::process::exit(2);
    }
    let search_from = match arg_value(&args, "--search-from") {
        Some(s) => match parse_iso(&s) {
            Some(t) => t,
            None => {
                eprintln!("themis_asi_frame_onset: --search-from {s} is not an ISO UTC stamp");
                std::process::exit(2);
            }
        },
        None => f64::NEG_INFINITY,
    };

    let skymap = match idl_save::load_skymap(Path::new(&skymap_path)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("themis_asi_frame_onset: skymap {skymap_path} unreadable: {e}");
            std::process::exit(2);
        }
    };
    if skymap.full_w != FRAME_W || skymap.full_h != FRAME_H {
        eprintln!(
            "themis_asi_frame_onset: skymap {skymap_path} geometry {}x{} differs from {FRAME_W}x{FRAME_H}",
            skymap.full_w, skymap.full_h
        );
        std::process::exit(2);
    }
    let cells = az_bins * el_bins;
    let map = bin_map(&skymap, az_bins, el_bins);
    let mapped = map.iter().filter(|m| m.is_some()).count();
    println!(
        "grid {az_bins} az x {el_bins} el = {cells} cells; {mapped} of {} skymap pixels carry an (az,el) bin",
        map.len()
    );

    let inputs = match collect_inputs(&args) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("themis_asi_frame_onset: {msg}");
            std::process::exit(2);
        }
    };

    let mut frames: Vec<Binned> = Vec::new();
    for path in &inputs {
        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("themis_asi_frame_onset: {} unreadable: {e}", path.display());
                std::process::exit(1);
            }
        };
        let name = path.display().to_string();
        match parse_frames(&data, &name, &map, cells) {
            Ok(mut f) => frames.append(&mut f),
            Err(msg) => {
                eprintln!("themis_asi_frame_onset: {msg}");
                std::process::exit(2);
            }
        }
    }

    if frames.is_empty() {
        eprintln!(
            "themis_asi_frame_onset: no usable sub-frame in {} input file(s)",
            inputs.len()
        );
        std::process::exit(2);
    }
    frames.sort_by(|a, b| a.epoch.total_cmp(&b.epoch));

    let (Some(first), Some(last)) = (frames.first(), frames.last()) else {
        eprintln!("themis_asi_frame_onset: no usable sub-frame");
        std::process::exit(2);
    };
    println!(
        "frames {} UT {} .. {} (span {:.3} s)",
        frames.len(),
        iso(first.epoch),
        iso(last.epoch),
        last.epoch - first.epoch
    );

    if args.iter().any(|a| a == "--report") {
        println!("report UT frame_mean");
        println!("# orientation: {ORIENTATION}");
        for f in &frames {
            let mut sum = 0.0f64;
            let mut n = 0usize;
            for v in &f.cells {
                if v.is_finite() {
                    sum += *v as f64;
                    n += 1;
                }
            }
            let mean = if n > 0 { sum / n as f64 } else { f64::NAN };
            println!("{} {:.1}", iso(f.epoch), mean);
        }
    }

    if args.iter().any(|a| a == "--onset-report") {
        let bg: Vec<f64> = frames
            .iter()
            .map(|f| {
                let mut vals: Vec<f64> = f
                    .cells
                    .iter()
                    .map(|v| *v as f64)
                    .filter(|v| v.is_finite())
                    .collect();
                median(&mut vals)
            })
            .collect();
        let contrast_of = |cell: usize| -> Vec<(f64, f64)> {
            frames
                .iter()
                .enumerate()
                .map(|(t, f)| (f.epoch, f.cells[cell] as f64 - bg[t]))
                .collect()
        };
        println!(
            "# onset detection: causal window {window} frames ; threshold = median_prev + {threshold_k:.1} * (1.4826*MAD) ; persist {persist} ; search-from {} ; contrast = cell - frame median",
            if search_from.is_finite() {
                iso(search_from)
            } else {
                "start".to_string()
            }
        );
        println!("# az_center el_center onset_ut contrast baseline threshold");
        let mut fired: Vec<(f64, usize, usize, f64, f64, f64)> = Vec::new();
        for eb in 0..el_bins {
            for ab in 0..az_bins {
                let cell = ab * el_bins + eb;
                let series = contrast_of(cell);
                if let Some(o) = running_onset(&series, window, threshold_k, persist, search_from) {
                    let az_center = (ab as f64 + 0.5) * 360.0 / az_bins as f64;
                    let el_center = (eb as f64 + 0.5) * 90.0 / el_bins as f64;
                    fired.push((o.epoch, ab, eb, o.value, o.baseline, o.threshold));
                    println!(
                        "{az_center:6.1} {el_center:6.1} {} {:.1} {:.1} {:.1}",
                        iso(o.epoch),
                        o.value,
                        o.baseline,
                        o.threshold
                    );
                }
            }
        }
        fired.sort_by(|a, b| a.0.total_cmp(&b.0));
        println!("# {} of {cells} cells fire", fired.len());
        if let Some(&(e, ab, eb, v, _, thr)) = fired.first() {
            println!(
                "# earliest cell onset {} az_center {:.1} el_center {:.1} contrast {:.1} threshold {:.1}",
                iso(e),
                (ab as f64 + 0.5) * 360.0 / az_bins as f64,
                (eb as f64 + 0.5) * 90.0 / el_bins as f64,
                v,
                thr
            );
        }
        println!("# earliest onset per az sector (az_lo..az_hi -> onset_ut el_center contrast)");
        for ab in 0..az_bins {
            let mut best: Option<(f64, usize, f64)> = None;
            for eb in 0..el_bins {
                let cell = ab * el_bins + eb;
                let series = contrast_of(cell);
                if let Some(o) = running_onset(&series, window, threshold_k, persist, search_from) {
                    if best.map(|b| o.epoch < b.0).unwrap_or(true) {
                        best = Some((o.epoch, eb, o.value));
                    }
                }
            }
            if let Some((e, eb, v)) = best {
                println!(
                    "{:5.1}..{:5.1} {} el_center {:.1} contrast {:.1}",
                    ab as f64 * 360.0 / az_bins as f64,
                    (ab + 1) as f64 * 360.0 / az_bins as f64,
                    iso(e),
                    (eb as f64 + 0.5) * 90.0 / el_bins as f64,
                    v
                );
            }
        }
    }

    match std::fs::write(&out_path, write_fodg(&frames, az_bins, el_bins)) {
        Ok(()) => println!("wrote {} frame(s) -> {}", frames.len(), out_path),
        Err(e) => {
            eprintln!("themis_asi_frame_onset: {out_path} write void: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_request_start_line_yields_unix_utc() {
        let line = r#"#"Image request start" 2024-05-10 06:00:00.033644 UTC"#;
        let t = parse_utc(line).expect("a stamp");
        assert!((t - 1715320800.033644).abs() < 1e-6);
    }

    #[test]
    fn iso_baseline_parses_to_the_same_epoch() {
        let a = parse_iso("2024-05-10T07:41:00Z").unwrap();
        let b = parse_utc(r#"#"Image request start" 2024-05-10 07:41:00.0 UTC"#).unwrap();
        assert!((a - b).abs() < 1e-6);
    }

    #[test]
    fn a_pixel_outside_the_disk_carries_no_bin() {
        let mut sm = idl_save::Skymap {
            site_uid: String::new(),
            imager_uid: String::new(),
            project_uid: String::new(),
            site_map_latitude: 0.0,
            site_map_longitude: 0.0,
            site_map_altitude: 0.0,
            pixel_aspect_ratio: 0.0,
            bin_map_altitude: 0.0,
            code_used: String::new(),
            full_w: 2,
            full_h: 2,
            full_elevation: vec![10.0, f32::NAN, -1.0, 45.0],
            full_azimuth: vec![0.0, 90.0, 180.0, 359.9],
            optical_projection: Vec::new(),
            ccd_center: Vec::new(),
            optical_orientation: Vec::new(),
            img_flip: Vec::new(),
        };
        sm.full_w = 2;
        sm.full_h = 2;
        let map = bin_map(&sm, 36, 9);
        assert!(map[0].is_some());
        assert_eq!(map[1], None);
        assert_eq!(map[2], None);
        assert!(map[3].is_some());
    }

    #[test]
    fn cells_are_the_mean_of_their_pixels() {
        let mut block = vec![0u8; FRAME_BYTES];
        for i in 0..4 {
            block[i * 2] = 0x00;
            block[i * 2 + 1] = 100;
        }
        let map = vec![Some(0u32), Some(0u32), Some(1u32), Some(1u32)];
        let cells = frame_cells(&block, &map, 2);
        assert_eq!(cells.len(), 2);
        assert!((cells[0] - 100.0).abs() < 1e-6);
        assert!((cells[1] - 100.0).abs() < 1e-6);
    }

    #[test]
    fn an_empty_cell_is_nan_not_zero() {
        let block = vec![7u8; 8];
        let map = vec![None, Some(0u32), None, None];
        let cells = frame_cells(&block, &map, 2);
        assert!(cells[0].is_finite());
        assert!(cells[1].is_nan());
    }

    #[test]
    fn a_sustained_rise_is_an_onset_and_a_spike_is_not() {
        let mut series: Vec<(f64, f64)> = (0..40)
            .map(|i| (1000.0 + i as f64, 10.0 + (i % 3) as f64))
            .collect();
        series[35].1 = 500.0;
        assert!(running_onset(&series, 20, 8.0, 3, f64::NEG_INFINITY).is_none());
        for v in series.iter_mut().take(37).skip(33) {
            v.1 = 500.0;
        }
        let o = running_onset(&series, 20, 8.0, 3, f64::NEG_INFINITY).expect("onset");
        assert_eq!(o.epoch, 1033.0);
    }

    #[test]
    fn the_fodg_header_carries_magic_and_grid() {
        let frames = vec![Binned {
            epoch: 1715320800.0,
            cells: vec![1.0; 4],
        }];
        let bytes = write_fodg(&frames, 2, 2);
        assert_eq!(&bytes[0..4], MAGIC);
        assert_eq!(
            u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            2
        );
        assert_eq!(
            u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            2
        );
        assert_eq!(
            u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            1
        );
        assert_eq!(bytes.len(), 16 + 8 + 4 * 4);
    }
}
