use omegaflow::bison_basu::{BAND_HIGH, BAND_LOW, BAND_MID, parse_bin, write_bin};

const X0: f64 = 278.596;
const X_PER_YEAR: f64 = 61.0849;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

enum Tok {
    Color(f64, f64, f64),
    Move(f64, f64),
    Line(f64, f64),
    Stroke,
    Ignore,
}

fn token(line: &str) -> Tok {
    let line = line.trim();
    if line == "S" {
        return Tok::Stroke;
    }
    if line.contains("concat T GR") || line.contains("ISOFONT") || line.contains("setdash") {
        return Tok::Ignore;
    }
    let f: Vec<&str> = line.split_whitespace().collect();
    if f.len() >= 4 && f[3] == "R" {
        if let (Ok(r), Ok(g), Ok(b)) = (f[0].parse(), f[1].parse(), f[2].parse()) {
            return Tok::Color(r, g, b);
        }
    }
    if f.len() == 3 && f[2] == "M" {
        if let (Ok(x), Ok(y)) = (f[0].parse(), f[1].parse()) {
            return Tok::Move(x, y);
        }
    }
    if f.len() == 3 && f[2] == "L" {
        if let (Ok(x), Ok(y)) = (f[0].parse(), f[1].parse()) {
            return Tok::Line(x, y);
        }
    }
    Tok::Ignore
}

fn band_of(y: f64) -> Option<u8> {
    if y > 1700.0 {
        Some(BAND_HIGH)
    } else if y > 980.0 {
        Some(BAND_MID)
    } else {
        Some(BAND_LOW)
    }
}

fn shift_hz(band: u8, y: f64) -> f64 {
    let u = match band {
        BAND_HIGH => (-0.6, 1755.34, 870.585),
        BAND_MID => (-0.3, 1052.53, 1451.4),
        _ => (-0.3, 243.594, 1703.34),
    };
    (u.0 + (y - u.1) / u.2) * 1e-6
}

fn units_per_hz(band: u8) -> f64 {
    match band {
        BAND_HIGH => 870.585,
        BAND_MID => 1451.4,
        _ => 1703.34,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = arg_value(&args, "--out").unwrap_or_else(|| "bison_basu.bin".to_string());
    let path = arg_value(&args, "--file").unwrap_or_else(|| "f2.ps".to_string());
    let Some(body_bytes) = std::fs::read(&path).ok() else {
        eprintln!("{path}: read void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let body = String::from_utf8_lossy(&body_bytes);

    let mut color = (1.0f64, 1.0f64, 1.0f64);
    let mut polylines: Vec<((f64, f64, f64), Vec<(f64, f64)>)> = Vec::new();
    let mut cur: Vec<(f64, f64)> = Vec::new();
    for line in body.lines() {
        match token(line) {
            Tok::Color(r, g, b) => {
                if !cur.is_empty() {
                    polylines.push((color, std::mem::take(&mut cur)));
                }
                color = (r, g, b);
            }
            Tok::Move(x, y) => {
                if !cur.is_empty() {
                    polylines.push((color, std::mem::take(&mut cur)));
                }
                cur.push((x, y));
            }
            Tok::Line(x, y) => cur.push((x, y)),
            Tok::Stroke => {
                if !cur.is_empty() {
                    polylines.push((color, std::mem::take(&mut cur)));
                }
            }
            Tok::Ignore => {}
        }
    }
    if !cur.is_empty() {
        polylines.push((color, cur));
    }

    let black = |c: &(f64, f64, f64)| c.0 == 0.0 && c.1 == 0.0 && c.2 == 0.0;
    let mut curves: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut vbars: Vec<(f64, f64, f64)> = Vec::new();
    for (c, pts) in &polylines {
        if !black(c) {
            continue;
        }
        if pts.len() > 3 {
            curves.push(pts.clone());
        } else if pts.len() == 2 {
            let (x1, y1) = pts[0];
            let (x2, y2) = pts[1];
            if (x1 - x2).abs() < 0.5 && (y1 - y2).abs() > 5.0 {
                vbars.push((x1, y1.min(y2), y1.max(y2)));
            }
        }
    }

    let mut records: Vec<(u8, i64, f64, f64)> = Vec::new();
    for curve in &curves {
        let ys: Vec<f64> = curve.iter().map(|&(_, y)| y).collect();
        let ymid = (ys.iter().cloned().fold(f64::MIN, f64::max)
            + ys.iter().cloned().fold(f64::MAX, f64::min))
            / 2.0;
        let Some(band) = band_of(ymid) else {
            continue;
        };
        for &(x, y) in curve {
            let year = 1975.0 + (x - X0) / X_PER_YEAR;
            let days = ((year - 1970.0) * 365.2425).round() as i64;
            let shift = shift_hz(band, y);
            let err = vbars
                .iter()
                .filter(|&&(bx, ..)| (bx - x).abs() < 0.6)
                .map(|&(_, ylo, yhi)| (yhi - ylo) / 2.0 / units_per_hz(band))
                .fold(f64::NAN, |a, b| {
                    if b.is_finite() && (a.is_nan() || b < a) {
                        b
                    } else {
                        a
                    }
                });
            if shift.is_finite() && err.is_finite() && err > 0.0 {
                records.push((band, days, shift, err));
            }
        }
    }
    records.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    records.dedup_by(|a, b| (a.0, a.1) == (b.0, b.1));

    let (n_low, n_mid, n_high) =
        records
            .iter()
            .fold((0usize, 0usize, 0usize), |acc, r| match r.0 {
                BAND_LOW => (acc.0 + 1, acc.1, acc.2),
                BAND_MID => (acc.0, acc.1 + 1, acc.2),
                _ => (acc.0, acc.1, acc.2 + 1),
            });
    eprintln!(
        "BiSON-Basu-2012: {} records (low {} mid {} high {}), Fig.-2 digitization",
        records.len(),
        n_low,
        n_mid,
        n_high
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
}
