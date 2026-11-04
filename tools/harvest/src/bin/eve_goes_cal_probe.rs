use std::collections::BTreeMap;
use std::env;
use std::fs;

const GOES_MAGIC: [u8; 4] = *b"GEUV";
const EVE_MAGIC: [u8; 4] = *b"EV20";
const MERGED_MAGIC: [u8; 4] = *b"MERG";
const INSTRUMENT_GOES15: u32 = 1;
const INSTRUMENT_EVE_SCALED: u32 = 3;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn minute_bins(records: &[(f64, f64)]) -> BTreeMap<i64, (f64, usize)> {
    let mut bins: BTreeMap<i64, (f64, usize)> = BTreeMap::new();
    for (t, v) in records {
        let key = (t / 60.0).floor() as i64;
        let e = bins.entry(key).or_insert((0.0, 0));
        e.0 += *v;
        e.1 += 1;
    }
    bins
}

fn read_goes(path: &str) -> Option<Vec<(f64, f64)>> {
    let b = fs::read(path).ok()?;
    if b.len() < 8 || b[0..4] != GOES_MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(b[4..8].try_into().ok()?) as usize;
    if n > (b.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let off = 8 + i * 20;
        let t = f64::from_le_bytes(b[off..off + 8].try_into().ok()?);
        let v = f64::from_le_bytes(b[off + 8..off + 16].try_into().ok()?);
        let comp = u32::from_le_bytes(b[off + 16..off + 20].try_into().ok()?);
        if comp == 2 && v.is_finite() && v > 0.0 {
            out.push((t, v));
        }
    }
    Some(out)
}

fn read_eve(path: &str) -> Option<Vec<(f64, f64, u32)>> {
    let b = fs::read(path).ok()?;
    if b.len() < 8 || b[0..4] != EVE_MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(b[4..8].try_into().ok()?) as usize;
    if n > (b.len() - 8) / 32 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let off = 8 + i * 32;
        let t = f64::from_le_bytes(b[off..off + 8].try_into().ok()?);
        let v = f64::from_le_bytes(b[off + 8..off + 16].try_into().ok()?);
        let flags = u32::from_le_bytes(b[off + 16..off + 20].try_into().ok()?);
        let inst = u32::from_le_bytes(b[off + 20..off + 24].try_into().ok()?);
        if inst == 2 && v.is_finite() && v > 0.0 {
            out.push((t, v, flags));
        }
    }
    Some(out)
}

fn fit(pairs: &[(f64, f64)]) -> Option<(usize, f64, f64, f64, f64, f64, f64)> {
    let n = pairs.len();
    if n < 3 {
        return None;
    }
    let nf = n as f64;
    let mx = pairs.iter().map(|p| p.0).sum::<f64>() / nf;
    let my = pairs.iter().map(|p| p.1).sum::<f64>() / nf;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    let mut sxy = 0.0;
    for (x, y) in pairs {
        let dx = x - mx;
        let dy = y - my;
        sxx += dx * dx;
        syy += dy * dy;
        sxy += dx * dy;
    }
    let a = sxy / sxx;
    let b = my - a * mx;
    let r = sxy / (sxx * syy).sqrt();
    let mut ss = 0.0;
    let mut ratios: Vec<f64> = Vec::with_capacity(n);
    for (x, y) in pairs {
        let res = y - (a * x + b);
        ss += res * res;
        ratios.push(y / x);
    }
    ratios.sort_by(|p, q| p.total_cmp(q));
    let median_ratio = ratios[n / 2];
    Some((n, a, b, r, (ss / nf).sqrt(), median_ratio, my))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let goes_path = arg_value(&args, "--goes").expect("--goes absent");
    let eve_paths: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| a.as_str() == "--eve" && i + 1 < args.len())
        .map(|(i, _)| args[i + 1].clone())
        .collect();
    let merged_path = arg_value(&args, "--out-merged");
    let stats_path = arg_value(&args, "--stats");
    let goes = read_goes(&goes_path).expect("goes asset void");
    let goes_bins = minute_bins(&goes);
    let mut all_pairs: Vec<(f64, f64)> = Vec::new();
    let mut report =
        String::from("set\teve_n\tgoes_n\tpaired\ta\tb\tr\tresid_sd\tmedian_ratio\tmean_goes\n");
    let mut eve_scaled: Vec<(f64, f64, u32)> = Vec::new();
    let mut combined_ratio = 1.0;
    for path in &eve_paths {
        let eve = read_eve(path).expect("eve asset void");
        let flat: Vec<(f64, f64)> = eve
            .iter()
            .filter(|(_, _, flags)| *flags == 0)
            .map(|(t, v, _)| (*t, *v))
            .collect();
        let bins = minute_bins(&flat);
        let mut pairs: Vec<(f64, f64)> = Vec::new();
        for (minute, (sum, count)) in &bins {
            if let Some((gsum, gcount)) = goes_bins.get(minute) {
                let x = gsum / *gcount as f64;
                let y = sum / *count as f64;
                if x > 0.0 && y > 0.0 {
                    pairs.push((x, y));
                    all_pairs.push((x, y));
                }
            }
        }
        if let Some((n, a, b, r, sd, mr, my)) = fit(&pairs) {
            let label = path.rsplit('/').next().unwrap_or(path);
            report.push_str(&format!(
                "{}\t{}\t{}\t{}\t{:.9}\t{:.9e}\t{:.6}\t{:.6e}\t{:.6}\t{:.6e}\n",
                label,
                eve.len(),
                goes.len(),
                n,
                a,
                b,
                r,
                sd,
                mr,
                my
            ));
            eprintln!(
                "{}: paired {} a {:.6} b {:.3e} r {:.6} resid {:.3e}",
                label, n, a, b, r, sd
            );
        } else {
            eprintln!("{}: not enough pairs", path);
        }
    }
    if let Some((n, a, b, r, sd, mr, my)) = fit(&all_pairs) {
        report.push_str(&format!(
            "combined\t{}\t{}\t{}\t{:.9}\t{:.9e}\t{:.6}\t{:.6e}\t{:.6}\t{:.6e}\n",
            all_pairs.len(),
            goes.len(),
            n,
            a,
            b,
            r,
            sd,
            mr,
            my
        ));
        eprintln!(
            "combined: paired {} a {:.6} b {:.3e} r {:.6} resid {:.3e} ratio {:.6}",
            n, a, b, r, sd, mr
        );
        combined_ratio = mr;
    } else {
        eprintln!("combined: not enough pairs");
    }
    report.push_str(&format!(
        "scale_used_for_merge\tGOES=EVE/{:.9}\n",
        combined_ratio
    ));
    if let Some(p) = stats_path {
        let _ = fs::write(&p, report);
    }
    if let Some(out) = merged_path {
        for path in &eve_paths {
            if !path.contains("2020_04_12") {
                continue;
            }
            let eve = read_eve(path).expect("eve asset void");
            for (t, v, flags) in eve {
                eve_scaled.push((t, v / combined_ratio, flags));
            }
        }
        let mut rows: Vec<(f64, f64, u32, u32)> = Vec::new();
        for (t, v) in &goes {
            rows.push((*t, *v, 0, INSTRUMENT_GOES15));
        }
        for (t, v, flags) in &eve_scaled {
            rows.push((*t, *v, *flags, INSTRUMENT_EVE_SCALED));
        }
        rows.sort_by(|p, q| p.0.total_cmp(&q.0).then(p.3.cmp(&q.3)));
        rows.dedup_by(|p, q| p.0 == q.0 && p.3 == q.3);
        let mut buf = Vec::with_capacity(8 + rows.len() * 32);
        buf.extend_from_slice(&MERGED_MAGIC);
        buf.extend_from_slice(&(rows.len() as u32).to_le_bytes());
        for (t, v, flags, inst) in &rows {
            buf.extend_from_slice(&t.to_le_bytes());
            buf.extend_from_slice(&v.to_le_bytes());
            buf.extend_from_slice(&flags.to_le_bytes());
            buf.extend_from_slice(&inst.to_le_bytes());
            buf.extend_from_slice(&0f32.to_le_bytes());
            buf.extend_from_slice(&0f32.to_le_bytes());
        }
        let _ = fs::write(&out, &buf);
        eprintln!("{}: {} merged records", out, rows.len());
    }
}
