use std::fs;

use omegaflow::atdf::parse_resid_bin;

const REC: usize = 2666;
const HDR: usize = 166;
const FS: f64 = 1250.0;
const SEG: usize = 8192;
const SEG_MS: f64 = SEG as f64 / FS * 1000.0;
const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: f64 = -2560.0;
const LOUD_HZ: f64 = 1.0;
const BIN_S: f64 = 300.0;
const MIN_FLOOR: usize = 8;

fn unix_day_float(tdb: f64) -> f64 {
    tdb / DAY_S + 10957.5
}

fn day_key(tdb: f64) -> i64 {
    unix_day_float(tdb).round() as i64
}

fn hour_of_day(tdb: f64) -> f64 {
    unix_day_float(tdb).rem_euclid(1.0) * 24.0
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.len() % 6 != 0 {
        eprintln!("galileo ODR same-day phase probe: <odr> <label> <YYYY-MM-DD> <station> <h0> <h1> [...] (6 tokens per window)");
        return;
    }
    let Ok(bytes) = fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("resid parse void");
        return;
    };
    drop(bytes);

    let mut resid_floor: Vec<(i64, i64, f64, f64)> = Vec::new();
    for r in &recs {
        let tdb = r[0];
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        if r[7] != FLOOR_AGC {
            continue;
        }
        let st = r[2] as i64;
        if st != 14 && st != 43 && st != 63 {
            continue;
        }
        resid_floor.push((st, day_key(tdb), hour_of_day(tdb), resid));
    }

    println!("galileo GO-J/GO-JS ODR same-day phase split: 300-s sub-windows of the ODR stream are tagged by the coincident closed-loop floor loudness (floor-class resid RMS of the same station and civil day, >= 8 samples in the sub-window); tone = AD1 carrier-line segsnr median over the 8192-sample segments inside each 300-s sub-window");
    let mut i = 0;
    while i < args.len() {
        let path = &args[i];
        let label = &args[i + 1];
        let ymd: Vec<i64> = args[i + 2].split('-').filter_map(|x| x.parse().ok()).collect();
        if ymd.len() != 3 {
            println!("{label}: date parse void");
            i += 6;
            continue;
        }
        let day = days_from_civil(ymd[0], ymd[1], ymd[2]);
        let Ok(station) = args[i + 3].parse::<i64>() else {
            println!("{label}: station parse void");
            i += 6;
            continue;
        };
        let Ok(h0) = args[i + 4].parse::<f64>() else {
            println!("{label}: window start not numeric");
            i += 6;
            continue;
        };
        let Ok(h1) = args[i + 5].parse::<f64>() else {
            println!("{label}: window stop not numeric");
            i += 6;
            continue;
        };
        measure(path, label, day, station, h0, h1, &resid_floor);
        i += 6;
    }
}

fn measure(path: &str, label: &str, day: i64, station: i64, h0: f64, h1: f64, resid_floor: &[(i64, i64, f64, f64)]) {
    let Ok(bytes) = fs::read(path) else {
        println!("{label}: read void");
        return;
    };
    let nrec = bytes.len() / REC;
    if nrec == 0 {
        println!("{label}: no records");
        return;
    }
    let t0 = (h0 * 3.6e6) as u32;
    let t1 = (h1 * 3.6e6) as u32;

    let mut segs: Vec<(f64, Vec<f64>)> = Vec::new();
    let mut buf: Vec<f64> = Vec::with_capacity(SEG);
    let mut buf_start_ms: f64 = f64::NAN;
    let mut n_in = 0usize;
    for k in 0..nrec {
        let r = &bytes[k * REC..(k + 1) * REC];
        let ms = f64::from(u32::from_be_bytes([r[12], r[13], r[14], r[15]]) & 0x07FF_FFFF);
        if ms < t0 as f64 {
            continue;
        }
        if ms >= t1 as f64 {
            break;
        }
        n_in += 1;
        let s = &r[HDR..REC];
        if buf.is_empty() {
            buf_start_ms = ms;
        }
        for g in 0..625 {
            buf.push(s[g * 4] as f64);
        }
        while buf.len() >= SEG {
            let seg_start_ms = buf_start_ms;
            let seg: Vec<f64> = buf.drain(..SEG).collect();
            segs.push((seg_start_ms, seg));
            buf_start_ms += SEG_MS;
        }
    }
    if n_in == 0 {
        println!("{label}: no records in [{h0:.1},{h1:.1}]h");
        return;
    }
    if segs.is_empty() {
        println!("{label}: {n_in} records, no full 8192-sample segment");
        return;
    }

    let resid_day: Vec<&(i64, i64, f64, f64)> = resid_floor
        .iter()
        .filter(|(s, d, _, _)| *s == station && *d == day)
        .collect();

    let first_ms = segs[0].0;
    let bin_count = ((segs.last().unwrap().0 + SEG_MS - first_ms) / (BIN_S * 1000.0)).ceil() as usize;
    let mut loud_tone: Vec<f64> = Vec::new();
    let mut quiet_tone: Vec<f64> = Vec::new();
    let mut no_floor_bins = 0usize;
    for b in 0..bin_count {
        let b0_ms = first_ms + b as f64 * BIN_S * 1000.0;
        let b1_ms = b0_ms + BIN_S * 1000.0;
        let b0_h = b0_ms / 3.6e6;
        let b1_h = b1_ms / 3.6e6;
        if b0_h > h1 || b1_h < h0 {
            continue;
        }
        let floor: Vec<f64> = resid_day
            .iter()
            .filter(|e| e.2 >= b0_h && e.2 < b1_h)
            .map(|e| e.3)
            .collect();
        if floor.len() < MIN_FLOOR {
            no_floor_bins += 1;
            continue;
        }
        let m = floor.iter().sum::<f64>() / floor.len() as f64;
        let var = floor.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / floor.len() as f64;
        let rms = var.sqrt();
        let snrs: Vec<f64> = segs
            .iter()
            .filter(|(ms, _)| *ms >= b0_ms && *ms < b1_ms)
            .map(|(_, seg)| segment_snr(seg))
            .filter(|s| *s > 0.0)
            .collect();
        if snrs.is_empty() {
            continue;
        }
        let (_, med, _, _) = stat(&snrs);
        if rms >= LOUD_HZ {
            loud_tone.push(med);
        } else {
            quiet_tone.push(med);
        }
    }

    let (ln, lmed, l10, l90) = stat(&loud_tone);
    let (qn, qmed, q10, q90) = stat(&quiet_tone);
    println!(
        "{label}: {n_in} records in-window, {} full segments, {bin_count} 300-s bins over the file span; loud-phase bins n {} (bin tone med {:.1}, p10-p90 {:.1}-{:.1}); quiet-phase bins n {} (bin tone med {:.1}, p10-p90 {:.1}-{:.1}); bins without >={} floor samples {no_floor_bins}",
        segs.len(),
        ln, lmed, l10, l90,
        qn, qmed, q10, q90,
        MIN_FLOOR
    );
    if ln > 0 && qn > 0 {
        println!(
            "{label}: SAME-DAY SPLIT — loud-vs-quiet 300-s bin tone med {:.1} vs {:.1} (n {}/{})",
            lmed, qmed, ln, qn
        );
    } else if ln > 0 {
        println!("{label}: only loud phase in window (quiet side n 0, 0 honored)");
    } else if qn > 0 {
        println!("{label}: only quiet phase in window (loud side n 0, 0 honored)");
    } else {
        println!("{label}: no phase classified in window (0 honored)");
    }
}

fn segment_snr(x: &[f64]) -> f64 {
    let m = seg_spec(x);
    let nf = median(&m);
    let pk = m.iter().cloned().fold(f64::MIN, f64::max);
    if nf > 0.0 {
        pk / nf
    } else {
        0.0
    }
}

fn seg_spec(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let mut re = vec![0.0f64; n];
    let mut im = vec![0.0f64; n];
    for (j, v) in x.iter().enumerate() {
        let w = 0.5 - 0.5 * ((std::f64::consts::TAU * j as f64) / (n - 1) as f64).cos();
        re[j] = v * w;
    }
    fft(&mut re, &mut im, false);
    let mut out = Vec::new();
    for b in 1..=n / 2 {
        let hz = b as f64 * FS / n as f64;
        if hz > 2.0 {
            out.push(re[b] * re[b] + im[b] * im[b]);
        }
    }
    out
}

fn median(v: &[f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    s[s.len() / 2]
}

fn stat(vals: &[f64]) -> (usize, f64, f64, f64) {
    if vals.is_empty() {
        return (0, 0.0, 0.0, 0.0);
    }
    let mut s = vals.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    (
        s.len(),
        s[s.len() / 2],
        s[(s.len() as f64 * 0.10) as usize],
        s[((s.len() as f64 * 0.90) as usize).min(s.len() - 1)],
    )
}

fn fft(re: &mut [f64], im: &mut [f64], inverse: bool) {
    let n = re.len();
    if n <= 1 {
        return;
    }
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2usize;
    while len <= n {
        let ang = if inverse {
            std::f64::consts::TAU / len as f64
        } else {
            -std::f64::consts::TAU / len as f64
        };
        let wr = ang.cos();
        let wi = ang.sin();
        let mut i = 0usize;
        while i < n {
            let mut cr = 1.0f64;
            let mut ci = 0.0f64;
            for k in 0..len / 2 {
                let ur = re[i + k];
                let ui = im[i + k];
                let vr = re[i + k + len / 2] * cr - im[i + k + len / 2] * ci;
                let vi = re[i + k + len / 2] * ci + im[i + k + len / 2] * cr;
                re[i + k] = ur + vr;
                im[i + k] = ui + vi;
                re[i + k + len / 2] = ur - vr;
                im[i + k + len / 2] = ui - vi;
                let ncr = cr * wr - ci * wi;
                ci = cr * wi + ci * wr;
                cr = ncr;
            }
            i += len;
        }
        len <<= 1;
    }
    if inverse {
        for v in re.iter_mut() {
            *v /= n as f64;
        }
        for v in im.iter_mut() {
            *v /= n as f64;
        }
    }
}
