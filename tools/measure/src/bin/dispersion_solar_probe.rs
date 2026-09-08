use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::te::{
    benjamini_hochberg, conditional_te_stats_lagged, permutation_entropy,
    transfer_entropy_conditional_h,
};

const MAGIC: [u8; 4] = *b"AIA1";
const DT: f64 = 24.0;
const WINDOW: usize = 100;
const REFRACTORY: usize = 75;
const FLARE_THRESH: f64 = 5e-6;
const FILL: f64 = -9999.0;
const LAGS: [usize; 3] = [0, 4, 8];
const PE_ORDER: usize = 3;
const PE_DELAY: usize = 1;
const LIGHT_M_S: f64 = 299792458.0;
const AU_M: f64 = 1.496e11;
const SHELF_FORCE: u8 = 0;
const DISPERSION_S: f64 = 4.15e-3;
const DM_SPALTE_PCCM3: f64 = 3.24;
const FDR_LEVEL: f64 = 0.05;

const AIA_BANDS: [(u32, &str, f64); 7] = [
    (0, "94A", 9.4e-9),
    (1, "131A", 13.1e-9),
    (2, "171A", 17.1e-9),
    (3, "193A", 19.3e-9),
    (4, "211A", 21.1e-9),
    (5, "304A", 30.4e-9),
    (6, "335A", 33.5e-9),
];

const MEASURED: [(&str, f64); 8] = [
    ("XRSA", 0.225e-9),
    ("94A", 9.4e-9),
    ("131A", 13.1e-9),
    ("171A", 17.1e-9),
    ("193A", 19.3e-9),
    ("211A", 21.1e-9),
    ("304A", 30.4e-9),
    ("335A", 33.5e-9),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_aia_lines(path: &str) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{} reads void", path);
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        eprintln!("{} carries no AIA1 contract", path);
        return Vec::new();
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 20;
        let Some(t) = bytes
            .get(o..o + 8)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(v) = bytes
            .get(o + 8..o + 16)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(idx) = bytes
            .get(o + 16..o + 20)
            .and_then(|b| b.try_into().ok())
            .map(u32::from_le_bytes)
        else {
            continue;
        };
        out.push((t, v, idx));
    }
    out
}

fn goes_flux(path: &str, ds: &str, flags: &str) -> Vec<(f64, f64)> {
    let Ok(bytes) = std::fs::read(path) else {
        return Vec::new();
    };
    let Ok(file) = Hdf5File::parse(&bytes) else {
        return Vec::new();
    };
    let (Ok(t_raw), Ok(v_raw), Ok(f_raw)) = (
        file.read_dataset("time"),
        file.read_dataset(ds),
        file.read_dataset(flags),
    ) else {
        return Vec::new();
    };
    let n = t_raw.len() / 8;
    if v_raw.len() != n * 4 || f_raw.len() != n * 2 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let Some(t) = decode_f64(&t_raw, i * 8, Endian::Le) else {
            continue;
        };
        let flag = u16::from_le_bytes([f_raw[i * 2], f_raw[i * 2 + 1]]);
        if flag != 0 {
            continue;
        }
        let Some(v) = decode_f32(&v_raw, i * 4, Endian::Le) else {
            continue;
        };
        let v = v as f64;
        if !v.is_finite() || v == FILL || v <= 0.0 {
            continue;
        }
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn bin_median(series: &[(f64, f64)], t0: f64, bins: usize) -> Vec<Option<f32>> {
    let mut acc: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(t, v) in series {
        let idx = ((t - t0) / DT).floor();
        if idx < 0.0 || idx >= bins as f64 {
            continue;
        }
        acc[idx as usize].push(v);
    }
    acc.into_iter()
        .map(|mut v| {
            if v.is_empty() {
                return None;
            }
            v.sort_by(|a, b| a.total_cmp(b));
            let m = if v.len() % 2 == 0 {
                (v[v.len() / 2 - 1] + v[v.len() / 2]) * 0.5
            } else {
                v[v.len() / 2]
            };
            Some(m as f32)
        })
        .collect()
}

struct Event {
    lines: Vec<Vec<f32>>,
    epoch: f64,
}

fn cut_events(
    trig: &[Option<f32>],
    threshold: f32,
    grid: &[Vec<Option<f32>>],
    t0: f64,
) -> Vec<Event> {
    let n = trig.len();
    let n_series = grid.len();
    let mut events: Vec<Event> = Vec::new();
    let mut i = 0usize;
    while i < n {
        let Some(tv) = trig[i] else {
            i += 1;
            continue;
        };
        if tv < threshold {
            i += 1;
            continue;
        }
        let mut j = i;
        let mut peak = i;
        while j < n && j - i < REFRACTORY {
            if let Some(v) = trig[j] {
                if v < threshold && j > i + 3 {
                    break;
                }
                if let Some(pv) = trig[peak] {
                    if v > pv {
                        peak = j;
                    }
                }
            }
            j += 1;
        }
        let lo = peak.saturating_sub(WINDOW);
        let hi = (peak + WINDOW).min(n);
        let mut best: Vec<Vec<f32>> = Vec::new();
        let mut run: Vec<Vec<f32>> = vec![Vec::new(); n_series];
        for k in lo..hi {
            let complete = grid.iter().all(|g| g[k].is_some());
            if complete {
                for (li, g) in grid.iter().enumerate() {
                    run[li].push(g[k].unwrap());
                }
                continue;
            }
            if run[0].len() > best.first().map_or(0, Vec::len) {
                best = run;
            }
            run = vec![Vec::new(); n_series];
        }
        if run[0].len() > best.first().map_or(0, Vec::len) {
            best = run;
        }
        if best.first().map_or(0, Vec::len) >= 100 {
            events.push(Event {
                lines: best,
                epoch: t0 + peak as f64 * DT,
            });
        }
        i = j.max(i + REFRACTORY);
    }
    events
}

fn sha256_hex(data: &[u8]) -> String {
    let mut digest = [0u32; 8];
    digest[0] = 0x6a09e667;
    digest[1] = 0xbb67ae85;
    digest[2] = 0x3c6ef372;
    digest[3] = 0xa54ff53a;
    digest[4] = 0x510e527f;
    digest[5] = 0x9b05688c;
    digest[6] = 0x1f83d9ab;
    digest[7] = 0x5be0cd19;
    let mut msg: Vec<u8> = Vec::with_capacity(data.len() + 64);
    msg.extend_from_slice(data);
    let bitlen = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    for i in 0..8 {
        let shift = (7 - i) * 8;
        msg.push(((bitlen >> shift) & 0xff) as u8);
    }
    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = ((chunk[i * 4] as u32) << 24)
                | ((chunk[i * 4 + 1] as u32) << 16)
                | ((chunk[i * 4 + 2] as u32) << 8)
                | (chunk[i * 4 + 3] as u32);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut a = digest[0];
        let mut b = digest[1];
        let mut c = digest[2];
        let mut d = digest[3];
        let mut e = digest[4];
        let mut f = digest[5];
        let mut g = digest[6];
        let mut h = digest[7];
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        digest[0] = digest[0].wrapping_add(a);
        digest[1] = digest[1].wrapping_add(b);
        digest[2] = digest[2].wrapping_add(c);
        digest[3] = digest[3].wrapping_add(d);
        digest[4] = digest[4].wrapping_add(e);
        digest[5] = digest[5].wrapping_add(f);
        digest[6] = digest[6].wrapping_add(g);
        digest[7] = digest[7].wrapping_add(h);
    }
    let mut out = String::with_capacity(64);
    for v in digest {
        out.push_str(&format!("{:08x}", v));
    }
    out
}

fn mean_sd(vals: &[f64]) -> (f64, f64) {
    let n = vals.len() as f64;
    if n == 0.0 {
        return (0.0, 0.0);
    }
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n;
    (mean, var.sqrt())
}

fn mode_mean_sd(vals: &[f64]) -> (Option<f64>, Option<f64>, Option<f64>) {
    if vals.is_empty() {
        return (None, None, None);
    }
    let (mean, sd) = mean_sd(vals);
    let mut counts: Vec<(f64, usize)> = Vec::new();
    for &v in vals {
        match counts.iter_mut().find(|(x, _)| *x == v) {
            Some((_, c)) => *c += 1,
            None => counts.push((v, 1)),
        }
    }
    let mode = counts.into_iter().max_by_key(|(_, c)| *c).map(|(v, _)| v);
    (mode, Some(mean), Some(sd))
}

fn binomial_p_two_sided(k: usize, n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let scale = 2.0f64.powi(-(n as i32));
    let mut c = 1.0f64;
    let mut p_le = 0.0;
    let mut p_ge = 0.0;
    for j in 0..=n {
        let pm = c * scale;
        if j <= k {
            p_le += pm;
        }
        if j >= k {
            p_ge += pm;
        }
        if j < n {
            c *= (n - j) as f64 / (j + 1) as f64;
        }
    }
    (2.0 * p_le.min(p_ge)).min(1.0)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let lsk = omegaflow::archivar::embedded_lsk();
    let mut positions: Vec<usize> = Vec::new();
    let mut idx = 0usize;
    while idx < args.len() {
        if args[idx] == "--year" {
            positions.push(idx);
            idx += 1;
        } else {
            idx += 1;
        }
    }
    if positions.is_empty() {
        eprintln!("--year <aia_lines.bin> <goes-dir> absent (repeat per year)");
        return;
    }
    let max_lag: usize = match arg_value(&args, "--max-lag").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => 8,
    };
    let n_surr: usize = match arg_value(&args, "--n-surr").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => 10,
    };
    let confound_name = match arg_value(&args, "--confound") {
        Some(n) => n,
        None => "goes".to_string(),
    };
    if confound_name != "goes" {
        eprintln!(
            "--confound {} carries no band (the dispersion probe conditions on GOES XRSB)",
            confound_name
        );
        return;
    }
    let max_events: usize = match arg_value(&args, "--max-events").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => 0,
    };
    let shelf_path = arg_value(&args, "--write-shelf");

    let mut all_events: Vec<Event> = Vec::new();
    for (yi, &pos) in positions.iter().enumerate() {
        let path = match args.get(pos + 1) {
            Some(p) => p.clone(),
            None => {
                eprintln!("--year {}: bin path absent", yi);
                return;
            }
        };
        let goes_dir = match args.get(pos + 2) {
            Some(d) => d.clone(),
            None => {
                eprintln!("--year {}: goes-dir absent", yi);
                return;
            }
        };
        let records = read_aia_lines(&path);
        if records.is_empty() {
            eprintln!("year {}: {} reads void", yi, path);
            return;
        }
        let mut t0 = f64::INFINITY;
        let mut t1 = f64::NEG_INFINITY;
        for (bidx, _name, _lam) in AIA_BANDS {
            let s: Vec<(f64, f64)> = records
                .iter()
                .filter(|(_, _, i)| *i == bidx)
                .map(|&(t, v, _)| (t, v))
                .collect();
            if s.is_empty() {
                eprintln!("year {} band {} absent in the bin", yi, bidx);
                return;
            }
            for &(t, _) in &s {
                t0 = t0.min(t);
                t1 = t1.max(t);
            }
        }
        let bins = ((t1 - t0) / DT).floor() as usize;
        let mut grid: Vec<Vec<Option<f32>>> = Vec::new();
        for (bidx, _name, _lam) in AIA_BANDS {
            let s: Vec<(f64, f64)> = records
                .iter()
                .filter(|(_, _, i)| *i == bidx)
                .map(|&(t, v, _)| (t, v))
                .collect();
            grid.push(bin_median(&s, t0, bins));
        }
        let mut a_flux: Vec<(f64, f64)> = Vec::new();
        let mut b_flux: Vec<(f64, f64)> = Vec::new();
        let Ok(entries) = std::fs::read_dir(&goes_dir) else {
            eprintln!("{} reads void", goes_dir);
            return;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("xr_") && name.ends_with(".nc") {
                let pa = entry.path().to_string_lossy().to_string();
                let ra = goes_flux(&pa, "a_flux", "a_flags");
                let rb = goes_flux(&pa, "b_flux", "b_flags");
                if let Some(lsk) = &lsk {
                    a_flux.extend(
                        ra.into_iter()
                            .filter_map(|(t, v)| lsk.unix_to_tdb(t).map(|t2| (t2, v))),
                    );
                    b_flux.extend(
                        rb.into_iter()
                            .filter_map(|(t, v)| lsk.unix_to_tdb(t).map(|t2| (t2, v))),
                    );
                } else {
                    a_flux.extend(ra);
                    b_flux.extend(rb);
                }
            }
        }
        a_flux.sort_by(|a, b| a.0.total_cmp(&b.0));
        b_flux.sort_by(|a, b| a.0.total_cmp(&b.0));
        grid.push(bin_median(&a_flux, t0, bins));
        let confound = bin_median(&b_flux, t0, bins);
        grid.push(confound.clone());
        let ev = cut_events(&confound, FLARE_THRESH as f32, &grid, t0);
        let before = all_events.len();
        all_events.extend(ev);
        println!(
            "year {}: {} events (GOES b_flux > {:.0e} W/m2), window +/-40 min, 24-s cells",
            yi,
            all_events.len() - before,
            FLARE_THRESH
        );
    }
    let mut events = all_events;
    if max_events > 0 && events.len() > max_events {
        events.truncate(max_events);
    }
    println!(
        "stack: {} events; confounder C = GOES XRSB (b_flux); null max_lag {} surrogates {}",
        events.len(),
        max_lag,
        n_surr
    );
    println!(
        "D_freq(l) = TE(f_hi -> f_lo | C) - TE(f_lo -> f_hi | C) per band pair and lag; lagged residual-surrogate null (mean+2sd)"
    );
    println!();

    let n_bands = MEASURED.len();
    let n_pairs = n_bands * (n_bands - 1) / 2;
    let mut d_sum = vec![vec![0.0f64; LAGS.len()]; n_pairs];
    let mut fwd_sum = vec![vec![0.0f64; LAGS.len()]; n_pairs];
    let mut rev_sum = vec![vec![0.0f64; LAGS.len()]; n_pairs];
    let mut d_cnt = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut fwd_arrow = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut rev_arrow = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut fwd_gated = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut rev_gated = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut deltas: Vec<Vec<[Option<f64>; LAGS.len()]>> = vec![Vec::new(); n_pairs];
    let mut pe_hi: Vec<Vec<Vec<f64>>> = vec![vec![Vec::new(); LAGS.len()]; n_pairs];
    let mut pe_lo: Vec<Vec<Vec<f64>>> = vec![vec![Vec::new(); LAGS.len()]; n_pairs];

    for (ei, ev) in events.iter().enumerate() {
        if ei % 25 == 0 {
            println!("progress {} / {}", ei, events.len());
        }
        let mut p = 0usize;
        for hi in 0..n_bands {
            for lo in hi + 1..n_bands {
                let x = &ev.lines[hi];
                let y = &ev.lines[lo];
                let c = &ev.lines[n_bands];
                let pe_x = permutation_entropy(
                    &x.iter().map(|&v| v as f64).collect::<Vec<f64>>(),
                    PE_ORDER,
                    PE_DELAY,
                );
                let pe_y = permutation_entropy(
                    &y.iter().map(|&v| v as f64).collect::<Vec<f64>>(),
                    PE_ORDER,
                    PE_DELAY,
                );
                let mut del: [Option<f64>; LAGS.len()] = [None; LAGS.len()];
                for (lagi, &lag) in LAGS.iter().enumerate() {
                    let seed = 0x9E37_79B9_7F4A_7C15
                        ^ (p as u64 * 0x9E37_79B9)
                        ^ (lag as u64 * 0x85EB_CA6B);
                    let (Some(te_fwd), Some(te_rev)) = (
                        transfer_entropy_conditional_h(x, y, c, lag, 1.0),
                        transfer_entropy_conditional_h(y, x, c, lag, 1.0),
                    ) else {
                        continue;
                    };
                    del[lagi] = Some(te_fwd - te_rev);
                    let n_fwd = conditional_te_stats_lagged(x, y, c, lag, max_lag, seed, n_surr);
                    let n_rev = conditional_te_stats_lagged(y, x, c, lag, max_lag, seed, n_surr);
                    d_sum[p][lagi] += te_fwd - te_rev;
                    fwd_sum[p][lagi] += te_fwd;
                    rev_sum[p][lagi] += te_rev;
                    d_cnt[p][lagi] += 1;
                    if let Some((_, _, thr)) = n_fwd {
                        if te_fwd > thr {
                            fwd_arrow[p][lagi] += 1;
                        }
                    }
                    if let Some((_, _, thr)) = n_rev {
                        if te_rev > thr {
                            rev_arrow[p][lagi] += 1;
                        }
                    }
                    if let Some(pe) = pe_x {
                        pe_hi[p][lagi].push(pe);
                    }
                    if let Some(pe) = pe_y {
                        pe_lo[p][lagi].push(pe);
                    }
                }
                deltas[p].push(del);
                p += 1;
            }
        }
    }
    for p in 0..n_pairs {
        for lagi in 0..LAGS.len() {
            let (m_hi, s_hi) = mean_sd(&pe_hi[p][lagi]);
            let (m_lo, s_lo) = mean_sd(&pe_lo[p][lagi]);
            for &pe in &pe_hi[p][lagi] {
                if (pe - m_hi).abs() > 2.0 * s_hi {
                    fwd_gated[p][lagi] += 1;
                }
            }
            for &pe in &pe_lo[p][lagi] {
                if (pe - m_lo).abs() > 2.0 * s_lo {
                    rev_gated[p][lagi] += 1;
                }
            }
        }
    }

    println!();
    println!("pair      | lag | mean D_freq | TE f_hi->f_lo | TE f_lo->f_hi | fwd/events | rev/events | fwd_gated | rev_gated");
    let mut p = 0usize;
    for hi in 0..n_bands {
        for lo in hi + 1..n_bands {
            let pair = format!("{}->{}", MEASURED[hi].0, MEASURED[lo].0);
            for (lagi, &lag) in LAGS.iter().enumerate() {
                if d_cnt[p][lagi] == 0 {
                    println!("{:>9} | {:>3}s | absent", pair, lag * 24);
                    continue;
                }
                let mean = d_sum[p][lagi] / d_cnt[p][lagi] as f64;
                let mf = fwd_sum[p][lagi] / d_cnt[p][lagi] as f64;
                let mr = rev_sum[p][lagi] / d_cnt[p][lagi] as f64;
                println!(
                    "{:>9} | {:>3}s | {:>12.3e} | {:>12.3e} | {:>12.3e} | {:>3}/{:<4} | {:>3}/{:<4} | {:>3} | {:>3}",
                    pair,
                    lag * 24,
                    mean,
                    mf,
                    mr,
                    fwd_arrow[p][lagi],
                    d_cnt[p][lagi],
                    rev_arrow[p][lagi],
                    d_cnt[p][lagi],
                    fwd_gated[p][lagi],
                    rev_gated[p][lagi]
                );
            }
            p += 1;
        }
    }

    let tau0 = AU_M / LIGHT_M_S;
    let dv = LIGHT_M_S * DT / tau0;
    println!();
    println!(
        "sensitivity: cell {} s -> dtau_min = {} s (smallest resolvable band latency)",
        DT, DT
    );
    println!(
        "  at 1 AU (d = {:.3e} m, tau = {:.1} s): dv = C_LIGHT * dtau_min / tau = {:.2e} m/s ({:.1} % of c)",
        AU_M,
        tau0,
        dv,
        dv / LIGHT_M_S * 100.0
    );
    println!(
        "  latency range {} s .. {} min (window)",
        DT,
        WINDOW as f64 * DT / 60.0
    );

    let mut p_dir = vec![vec![1.0f64; LAGS.len()]; n_pairs];
    let mut tau_star: Vec<Vec<f64>> = vec![Vec::new(); n_pairs];
    for p in 0..n_pairs {
        for (lagi, _) in LAGS.iter().enumerate() {
            let valid: Vec<f64> = deltas[p].iter().filter_map(|d| d[lagi]).collect();
            let k = valid.iter().filter(|&&d| d > 0.0).count();
            p_dir[p][lagi] = binomial_p_two_sided(k, valid.len());
        }
        for d in &deltas[p] {
            let mut best: Option<(usize, f64)> = None;
            for (lagi, _) in LAGS.iter().enumerate() {
                if let Some(v) = d[lagi] {
                    if best.map_or(true, |(_, bv)| v.abs() > bv) {
                        best = Some((lagi, v.abs()));
                    }
                }
            }
            if let Some((lagi, _)) = best {
                tau_star[p].push(LAGS[lagi] as f64 * DT);
            }
        }
    }
    let mut fdr_vals: Vec<f64> = Vec::with_capacity(n_pairs * LAGS.len());
    for p in 0..n_pairs {
        for lagi in 0..LAGS.len() {
            fdr_vals.push(p_dir[p][lagi]);
        }
    }
    let cutoff = benjamini_hochberg(&fdr_vals, FDR_LEVEL);

    let mut medium_pairs: Vec<(usize, usize)> = Vec::new();
    let mut source_pairs: Vec<usize> = Vec::new();
    println!();
    let cutoff_str = match cutoff {
        Some(c) => format!("{:.3e}", c),
        None => "void".to_string(),
    };
    println!("direction gate: per-event asymmetry fwd-rev, two-sided exact binomial, FDR level {} over {} tests, cutoff {}", FDR_LEVEL, fdr_vals.len(), cutoff_str);
    println!(
        "medium ceiling: dtau_medium(pair) = {} s * {} pc/cm3 * (nu_lo_GHz^-2 - nu_hi_GHz^-2)",
        DISPERSION_S, DM_SPALTE_PCCM3
    );
    println!();
    println!("band_pair | f_hi_hz | f_lo_hz | p_dir_min | dtau_medium_s | tau*_mode_s | tau*_mean_s | tau*_sd_s | state");
    let mut p = 0usize;
    for hi in 0..n_bands {
        for lo in hi + 1..n_bands {
            let f_hi = LIGHT_M_S / MEASURED[hi].1;
            let f_lo = LIGHT_M_S / MEASURED[lo].1;
            let pair = format!("{}->{}", MEASURED[hi].0, MEASURED[lo].0);
            let nu_hi = f_hi / 1e9;
            let nu_lo = f_lo / 1e9;
            let dtau_med =
                DISPERSION_S * DM_SPALTE_PCCM3 * (1.0 / (nu_lo * nu_lo) - 1.0 / (nu_hi * nu_hi));
            let p_min = (0..LAGS.len())
                .map(|li| p_dir[p][li])
                .fold(1.0f64, f64::min);
            let (mode_s, mean_s, sd_s) = mode_mean_sd(&tau_star[p]);
            let cleared = cutoff.map_or(false, |c| p_min <= c);
            let state = if cleared {
                if dtau_med >= DT {
                    medium_pairs.push((hi, lo));
                    "medium"
                } else {
                    source_pairs.push(p);
                    "quell-seitig"
                }
            } else {
                "flat"
            };
            let mode_str = match mode_s {
                Some(v) => format!("{:.0}", v),
                None => "-".to_string(),
            };
            let mean_str = match mean_s {
                Some(v) => format!("{:.1}", v),
                None => "-".to_string(),
            };
            let sd_str = match sd_s {
                Some(v) => format!("{:.1}", v),
                None => "-".to_string(),
            };
            println!(
                "{:>9} | {:.3e} | {:.3e} | {:>9.3e} | {:>13.3e} | {:>11} | {:>10} | {:>9} | {}",
                pair, f_hi, f_lo, p_min, dtau_med, mode_str, mean_str, sd_str, state
            );
            p += 1;
        }
    }

    let mut shelf_lines: Vec<String> = Vec::new();
    let mid_epoch = if events.is_empty() {
        0.0
    } else {
        events.iter().map(|e| e.epoch).sum::<f64>() / events.len() as f64
    };
    if !medium_pairs.is_empty() {
        println!();
        println!(
            "verdict: medium — {} band pair(s) carry a directional arrow (binomial + FDR) at dtau_medium >= dtau_min = {} s: v(f) becomes a measurement duty",
            medium_pairs.len(),
            DT
        );
        for &(hi, lo) in &medium_pairs {
            let freq = LIGHT_M_S / MEASURED[hi].1;
            let nu_hi = freq / 1e9;
            let nu_lo = LIGHT_M_S / MEASURED[lo].1 / 1e9;
            let dtau =
                DISPERSION_S * DM_SPALTE_PCCM3 * (1.0 / (nu_lo * nu_lo) - 1.0 / (nu_hi * nu_hi));
            let v = AU_M / (tau0 + dtau);
            shelf_lines.push(format!(
                "{} {:.6e} 0.0 {:.6e} {:.6e} {:.6e}",
                SHELF_FORCE, freq, v, dv, mid_epoch
            ));
        }
    } else if !source_pairs.is_empty() {
        println!();
        println!(
            "verdict: quell-seitig — {} band pair(s) carry a directional arrow above the binomial-FDR gate, but dtau_medium ~ 1e-16 s is null-echt against dtau_min = {} s: the arrow is the source response order (Neupert), not a v(f) duty",
            source_pairs.len(),
            DT
        );
        for (_name, lam) in MEASURED {
            let freq = LIGHT_M_S / lam;
            shelf_lines.push(format!(
                "{} {:.6e} 0.0 {:.6e} {:.6e} {:.6e}",
                SHELF_FORCE, freq, LIGHT_M_S, dv, mid_epoch
            ));
        }
    } else {
        println!();
        println!(
            "verdict: flat — no band pair carries a directional arrow (binomial + FDR {} over {} tests)",
            FDR_LEVEL,
            fdr_vals.len()
        );
        for (_name, lam) in MEASURED {
            let freq = LIGHT_M_S / lam;
            shelf_lines.push(format!(
                "{} {:.6e} 0.0 {:.6e} {:.6e} {:.6e}",
                SHELF_FORCE, freq, LIGHT_M_S, dv, mid_epoch
            ));
        }
    }
    if let Some(path) = shelf_path {
        let mut body = String::new();
        for line in &shelf_lines {
            body.push_str(line);
            body.push('\n');
        }
        let hex = sha256_hex(body.as_bytes());
        let mut out = String::from("# v_freq_shelf v1 sha256:");
        out.push_str(&hex);
        out.push_str("\n# columns: force_type freq_hz bin_width_hz v_m_s v_unc_m_s epoch_tdb\n");
        out.push_str(&body);
        if std::fs::write(&path, out).is_ok() {
            println!("shelf written: {} rows, sha256 {}", shelf_lines.len(), hex);
        } else {
            eprintln!("{} unwritable", path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::binomial_p_two_sided;

    #[test]
    fn a_symmetric_split_carries_no_direction() {
        let p = binomial_p_two_sided(500, 1000);
        assert!(
            p > 0.9,
            "a balanced split must not clear any gate, got {}",
            p
        );
    }

    #[test]
    fn a_strong_majority_clears_the_gate() {
        let p = binomial_p_two_sided(900, 1000);
        assert!(p < 1e-6, "a 900/1000 split must clear the gate, got {}", p);
    }

    #[test]
    fn the_side_with_more_positive_deltas_is_symmetric() {
        let p_hi = binomial_p_two_sided(700, 1000);
        let p_lo = binomial_p_two_sided(300, 1000);
        assert!((p_hi - p_lo).abs() < 1e-12);
    }
}
