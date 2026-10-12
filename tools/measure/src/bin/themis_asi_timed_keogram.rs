use omegaflow::lsk::days_from_civil;
use omegaflow_measure::idl_save;
use std::path::{Path, PathBuf};

const FRAME_W: usize = 256;
const FRAME_H: usize = 256;
const FRAME_BYTES: usize = FRAME_W * FRAME_H * 2;

const BAND_COL0: usize = 120;
const BAND_COL1: usize = 135;
const BAND_COLS: usize = BAND_COL1 - BAND_COL0 + 1;

const MAGIC: &[u8; 4] = b"TKDG";
const DAY_S: f64 = 86400.0;
const HOUR_S: f64 = 3600.0;
const PGM_MAXVAL: u32 = 65535;

const ORIENTATION: &str = "raw stream0 row0=north (measured 2026-10-11: ucalgary-srs/pyUCalgarySRS _themis.py and mshumko/asilib themis.py both flip N-S to render north-up, so the stored PGM row0 is north); poleward = decreasing row";

#[derive(Debug)]
struct Frame {
    epoch: f64,
    profile: Vec<u16>,
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
    let y = if m <= 2 { y + 1 } else { y };
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

fn profile_of(block: &[u8]) -> Vec<u16> {
    let mut profile = Vec::with_capacity(FRAME_H);
    for row in 0..FRAME_H {
        let mut sum: u32 = 0;
        for col in BAND_COL0..=BAND_COL1 {
            let i = (row * FRAME_W + col) * 2;
            let value = ((block[i] as u16) << 8) | block[i + 1] as u16;
            sum += value as u32;
        }
        profile.push(((sum + BAND_COLS as u32 / 2) / BAND_COLS as u32) as u16);
    }
    profile
}

fn meridian_distance(azimuth_deg: f64) -> f64 {
    let period = 180.0;
    let folded = azimuth_deg.rem_euclid(period);
    folded.min(period - folded)
}

fn meridian_column(azimuth_row: &[f32]) -> Option<usize> {
    let mut best_col = 0usize;
    let mut best_distance = f64::INFINITY;
    for (col, &az) in azimuth_row.iter().enumerate() {
        let distance = meridian_distance(az as f64);
        if distance.is_finite() && distance < best_distance {
            best_distance = distance;
            best_col = col;
        }
    }
    if best_distance.is_finite() {
        Some(best_col)
    } else {
        None
    }
}

fn meridian_columns(azimuth: &[f32], w: usize, h: usize) -> Vec<Option<usize>> {
    let mut cols = Vec::with_capacity(h);
    for row in 0..h {
        cols.push(meridian_column(&azimuth[row * w..row * w + w]));
    }
    cols
}

fn profile_of_meridian(block: &[u8], cols: &[Option<usize>]) -> Vec<u16> {
    let mut profile = Vec::with_capacity(FRAME_H);
    for (row, &col) in cols.iter().enumerate() {
        match col {
            Some(c) => {
                let i = (row * FRAME_W + c) * 2;
                profile.push(((block[i] as u16) << 8) | block[i + 1] as u16);
            }
            None => profile.push(0),
        }
    }
    profile
}

fn parse_frames(data: &[u8], name: &str) -> Result<Vec<Frame>, String> {
    parse_frames_with(data, name, None)
}

fn parse_frames_with(
    data: &[u8],
    name: &str,
    meridian_cols: Option<&[Option<usize>]>,
) -> Result<Vec<Frame>, String> {
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
        let profile = match meridian_cols {
            Some(cols) => profile_of_meridian(&data[pos..pos + FRAME_BYTES], cols),
            None => profile_of(&data[pos..pos + FRAME_BYTES]),
        };
        pos += FRAME_BYTES;
        frames.push(Frame { epoch, profile });
    }
    Ok(frames)
}

fn profile_stats(profile: &[u16]) -> (f64, usize) {
    let sum: u64 = profile.iter().map(|&v| v as u64).sum();
    let mean = sum as f64 / profile.len() as f64;
    let mut argmax = 0usize;
    let mut best = 0u16;
    for (row, &v) in profile.iter().enumerate() {
        if v > best {
            best = v;
            argmax = row;
        }
    }
    (mean, argmax)
}

fn write_tkdg(frames: &[Frame]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + frames.len() * (8 + FRAME_H * 2));
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(frames.len() as u32).to_le_bytes());
    for f in frames {
        out.extend_from_slice(&f.epoch.to_le_bytes());
        for v in &f.profile {
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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out_path = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!("themis_asi_timed_keogram needs --out <file.bin>");
            std::process::exit(2);
        }
    };
    let inputs = match collect_inputs(&args) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("themis_asi_timed_keogram: {msg}");
            std::process::exit(2);
        }
    };

    let elevation_out = arg_value(&args, "--elevation-out");
    let skymap_path = arg_value(&args, "--skymap");
    if elevation_out.is_some() && skymap_path.is_none() {
        eprintln!("themis_asi_timed_keogram: --elevation-out needs --skymap <path.sav>");
        std::process::exit(2);
    }
    let meridian_cols = match &skymap_path {
        Some(p) => {
            let skymap = match idl_save::load_skymap(Path::new(p)) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("themis_asi_timed_keogram: skymap {p} unreadable: {e}");
                    std::process::exit(2);
                }
            };
            if skymap.full_w != FRAME_W || skymap.full_h != FRAME_H {
                eprintln!(
                    "themis_asi_timed_keogram: skymap {p} geometry {}x{} differs from {FRAME_W}x{FRAME_H}",
                    skymap.full_w, skymap.full_h
                );
                std::process::exit(2);
            }
            let cols = meridian_columns(&skymap.full_azimuth, skymap.full_w, skymap.full_h);
            let off_map: Vec<usize> = cols
                .iter()
                .enumerate()
                .filter(|(_, col)| col.is_none())
                .map(|(row, _)| row)
                .collect();
            if let Some(out) = &elevation_out {
                let mut text = String::new();
                for (row, col) in cols.iter().enumerate() {
                    let elevation_deg = match col {
                        Some(c) => skymap.full_elevation[row * skymap.full_w + c] as f64,
                        None => f64::NAN,
                    };
                    text.push_str(&format!("{elevation_deg:.6}\n"));
                }
                if let Err(e) = std::fs::write(out, text) {
                    eprintln!("themis_asi_timed_keogram: {out} write void: {e}");
                    std::process::exit(1);
                }
                println!("wrote {FRAME_H} meridian elevation row(s) -> {out}");
            }
            let rows_list: Vec<String> = off_map.iter().map(|r| r.to_string()).collect();
            println!(
                "off-map rows (no meridian pixel): {} [{}]",
                off_map.len(),
                rows_list.join(" ")
            );
            Some(cols)
        }
        None => None,
    };

    let mut frames: Vec<Frame> = Vec::new();
    for path in &inputs {
        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!(
                    "themis_asi_timed_keogram: {} unreadable: {e}",
                    path.display()
                );
                std::process::exit(1);
            }
        };
        let name = path.display().to_string();
        let parsed = match meridian_cols.as_deref() {
            Some(cols) => parse_frames_with(&data, &name, Some(cols)),
            None => parse_frames(&data, &name),
        };
        match parsed {
            Ok(mut f) => frames.append(&mut f),
            Err(msg) => {
                eprintln!("themis_asi_timed_keogram: {msg}");
                std::process::exit(2);
            }
        }
    }

    if frames.is_empty() {
        eprintln!(
            "themis_asi_timed_keogram: no usable sub-frame in {} input file(s)",
            inputs.len()
        );
        std::process::exit(2);
    }
    frames.sort_by(|a, b| a.epoch.total_cmp(&b.epoch));

    let (Some(first), Some(last)) = (frames.first(), frames.last()) else {
        eprintln!("themis_asi_timed_keogram: no usable sub-frame");
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
        println!("report UT band_mean argmax_row");
        println!("# orientation: {ORIENTATION}");
        for f in &frames {
            let (mean, argmax) = profile_stats(&f.profile);
            println!("{} {:.1} {}", iso(f.epoch), mean, argmax);
        }
    }

    match std::fs::write(&out_path, write_tkdg(&frames)) {
        Ok(()) => println!("wrote {} frame(s) -> {}", frames.len(), out_path),
        Err(e) => {
            eprintln!("themis_asi_timed_keogram: {out_path} write void: {e}");
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
    fn a_foreign_comment_is_not_a_stamp() {
        assert_eq!(parse_utc(r#"#"Ccd background" 2900"#), None);
    }

    #[test]
    fn the_profile_is_the_band_mean_in_big_endian() {
        let mut block = vec![0u8; FRAME_BYTES];
        for col in BAND_COL0..=BAND_COL1 {
            let i = col * 2;
            block[i] = 0x03;
            block[i + 1] = 0xE8;
        }
        block[(FRAME_W + BAND_COL0) * 2 + 1] = 0xC8;
        let profile = profile_of(&block);
        assert_eq!(profile.len(), FRAME_H);
        assert_eq!(profile[0], 1000);
        assert_eq!(profile[1], 200);
        assert_eq!(profile[2], 0);
    }

    #[test]
    fn a_sub_frame_with_a_foreign_geometry_aborts() {
        let data =
            b"P5\n#\"Image request start\" 2024-05-10 06:00:00.0 UTC\n128 128\n65535\n".to_vec();
        let err = parse_frames(&data, "synthetic").expect_err("geometry differs");
        assert!(err.contains("128x128"));
    }

    #[test]
    fn the_tkdg_header_carries_magic_and_count() {
        let frames = vec![Frame {
            epoch: 1715320800.0,
            profile: vec![7u16; FRAME_H],
        }];
        let bytes = write_tkdg(&frames);
        assert_eq!(&bytes[0..4], MAGIC);
        assert_eq!(
            u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            1
        );
        assert_eq!(bytes.len(), 8 + 8 + FRAME_H * 2);
    }

    #[test]
    fn the_meridian_distance_folds_the_0_360_seam() {
        assert_eq!(meridian_distance(0.0), 0.0);
        assert_eq!(meridian_distance(180.0), 0.0);
        assert_eq!(meridian_distance(360.0), 0.0);
        assert_eq!(meridian_distance(350.0), 10.0);
        assert_eq!(meridian_distance(190.0), 10.0);
        assert_eq!(meridian_distance(90.0), 90.0);
        assert_eq!(meridian_distance(270.0), 90.0);
    }

    #[test]
    fn meridian_columns_take_the_smallest_angular_distance() {
        let mut az = vec![90.0f32; FRAME_W * FRAME_H];
        for row in 0..FRAME_H {
            let target = (row * 7) % FRAME_W;
            az[row * FRAME_W + target] = 0.0;
        }
        let nan_row = 5usize;
        for col in 0..FRAME_W {
            az[nan_row * FRAME_W + col] = f32::NAN;
        }
        let cols = meridian_columns(&az, FRAME_W, FRAME_H);
        assert_eq!(cols.len(), FRAME_H);
        for row in 0..FRAME_H {
            let target = (row * 7) % FRAME_W;
            if row == nan_row {
                assert_eq!(cols[row], None);
                continue;
            }
            assert_eq!(cols[row], Some(target));
            let chosen = meridian_distance(az[row * FRAME_W + cols[row].unwrap()] as f64);
            let least = (0..FRAME_W)
                .map(|c| meridian_distance(az[row * FRAME_W + c] as f64))
                .fold(f64::INFINITY, f64::min);
            assert_eq!(chosen, least);
        }
    }

    #[test]
    fn meridian_columns_break_ties_to_the_smaller_column() {
        let mut az = vec![90.0f32; FRAME_W * FRAME_H];
        for row in 0..FRAME_H {
            az[row * FRAME_W + 100] = 0.0;
            az[row * FRAME_W + 200] = 0.0;
        }
        let cols = meridian_columns(&az, FRAME_W, FRAME_H);
        for &col in &cols {
            assert_eq!(col, Some(100));
        }
    }
}
