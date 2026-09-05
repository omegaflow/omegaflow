use omegaflow::archivar::spatial::{parse_star_record, star_stride, STAR_RECORD_BYTES};
use omegaflow::healpix::{ang2pix_nest, galactic_to_icrs, icrs_to_galactic};

const NSIDE_DUST: i64 = 512;
const BIN_DEG: f64 = 0.25;
const SCAN_B_MAX: f64 = 75.0;
const SCAN_B_MIN: f64 = 10.0;
const SCAN_B_STEP: f64 = 1.5;
const SCAN_L_STEP: f64 = 3.0;
const AV_LO: f64 = 0.3;
const AV_HI: f64 = 1.5;
const AV_SPREAD_MIN: f64 = 0.15;
const RADIUS_DEFAULT_DEG: f64 = 4.0;
const SLICE_LO_DEFAULT: f64 = 3.0;
const SLICE_HI_DEFAULT: f64 = 6.0;
const MAG_CLEAN_MAX: f64 = 18.0;
const COLOR_BAND_HALF: f64 = 0.15;
const COLOR_MIN: f64 = -0.5;
const COLOR_MAX: f64 = 4.5;
const K_OLS_MIN: usize = 150;
const GC_EXCLUDE_DEG: f64 = 45.0;
const MIN_SLICE: usize = 200;
const SHOW_TOP: usize = 6;
const N_EXACT_MIN: usize = 200;

const WANG_A_GBP: f64 = 2.429;
const WANG_A_G: f64 = 1.890;

struct DustMap {
    av: Vec<f64>,
    screen_pc: Vec<f64>,
    valid: Vec<bool>,
}

struct Star {
    ra_deg: f64,
    dec_deg: f64,
    d_pc: f64,
    g: f64,
    color: f64,
    m_abs: f64,
}

struct BinAgg {
    nl: usize,
    nb: usize,
    cnt: Vec<u32>,
    scnt: Vec<u32>,
    scol: Vec<f64>,
    sav: Vec<f64>,
    vcnt: Vec<u32>,
    vsum: Vec<f64>,
    vsum2: Vec<f64>,
}

struct ConeStats {
    n: usize,
    sn: usize,
    mean: f64,
    sd: f64,
    slope: f64,
}

struct Reg {
    n: usize,
    slope: f64,
    intercept: f64,
    rms: f64,
    pearson: f64,
    x_mean: f64,
    x_sd: f64,
}

struct Member {
    av: f64,
    g: f64,
    color: f64,
    m_abs: f64,
}

struct ConeSample {
    cone_stars: usize,
    cone_no_pixel: usize,
    behind_av_refused: usize,
    color_refused: usize,
    members: Vec<Member>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn pair_of(args: &[String], name: &str) -> Option<(f64, f64)> {
    let i = args.iter().position(|a| a == name)?;
    let a = args.get(i + 1)?.parse::<f64>().ok()?;
    let b = args.get(i + 2)?.parse::<f64>().ok()?;
    if a.is_finite() && b.is_finite() {
        Some((a, b))
    } else {
        None
    }
}

fn f64_of(args: &[String], name: &str) -> Option<f64> {
    arg_value(args, name).and_then(|v| v.parse().ok())
}

fn read_bytes(path: &str, what: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {path} ({what}) returned void: {e}");
            std::process::exit(1);
        }
    }
}

fn token_char(c: u8) -> bool {
    c.is_ascii_digit() || c == b'-' || c == b'+' || c == b'.' || c == b'e' || c == b'E'
}

fn skip_ws(bytes: &[u8], p: &mut usize) {
    while *p < bytes.len() && (bytes[*p] as char).is_whitespace() {
        *p += 1;
    }
}

fn load_dust(bytes: &[u8]) -> (DustMap, usize, usize, usize, usize, f64, f64) {
    let npix = (12 * NSIDE_DUST * NSIDE_DUST) as usize;
    let mut av = vec![0.0f64; npix];
    let mut screen_pc = vec![0.0f64; npix];
    let mut valid = vec![false; npix];
    let mut p = 0usize;
    skip_ws(bytes, &mut p);
    let mut rows = 0usize;
    let mut rows_unread = 0usize;
    let mut aliases = 0usize;
    let mut screen_lo = f64::INFINITY;
    let mut screen_hi = f64::NEG_INFINITY;
    if p >= bytes.len() || bytes[p] != b'[' {
        eprintln!("dust json: the top value is not an array — the map stays unread");
        std::process::exit(1);
    }
    p += 1;
    loop {
        skip_ws(bytes, &mut p);
        if p >= bytes.len() {
            break;
        }
        if bytes[p] == b']' {
            break;
        }
        if bytes[p] == b',' {
            p += 1;
            continue;
        }
        if bytes[p] != b'{' {
            eprintln!("dust json at byte {p}: expected an object — the map stays unread");
            std::process::exit(1);
        }
        p += 1;
        let mut ra = None;
        let mut dec = None;
        let mut dist = None;
        let mut avrq = None;
        loop {
            skip_ws(bytes, &mut p);
            if p >= bytes.len() || bytes[p] != b'"' {
                break;
            }
            p += 1;
            let ks = p;
            while p < bytes.len() && bytes[p] != b'"' {
                p += 1;
            }
            let key = std::str::from_utf8(&bytes[ks..p]);
            p += 1;
            skip_ws(bytes, &mut p);
            if p >= bytes.len() || bytes[p] != b':' {
                break;
            }
            p += 1;
            skip_ws(bytes, &mut p);
            let ns = p;
            while p < bytes.len() && token_char(bytes[p]) {
                p += 1;
            }
            let num = std::str::from_utf8(&bytes[ns..p])
                .ok()
                .and_then(|t| t.parse::<f64>().ok());
            match key {
                Ok("ra") => ra = num,
                Ok("dec") => dec = num,
                Ok("dist") => dist = num,
                Ok("av_rq") => avrq = num,
                _ => {}
            }
            skip_ws(bytes, &mut p);
            if p < bytes.len() && bytes[p] == b'}' {
                break;
            }
            if p < bytes.len() && bytes[p] == b',' {
                p += 1;
            }
        }
        skip_ws(bytes, &mut p);
        if p < bytes.len() && bytes[p] == b'}' {
            p += 1;
        }
        rows += 1;
        match (ra, dec, dist, avrq) {
            (Some(ra), Some(dec), Some(dist), Some(avrq))
                if ra.is_finite()
                    && dec.is_finite()
                    && dist.is_finite()
                    && dist > 0.0
                    && avrq.is_finite()
                    && (0.0..=360.0).contains(&ra)
                    && (-90.0..=90.0).contains(&dec) =>
            {
                let (theta, phi) = icrs_to_galactic(ra, dec);
                match ang2pix_nest(NSIDE_DUST, theta, phi) {
                    Some(pix) => {
                        let c = pix as usize;
                        if valid[c] {
                            aliases += 1;
                        }
                        av[c] = avrq;
                        screen_pc[c] = dist;
                        valid[c] = true;
                        if dist < screen_lo {
                            screen_lo = dist;
                        }
                        if dist > screen_hi {
                            screen_hi = dist;
                        }
                    }
                    None => rows_unread += 1,
                }
            }
            _ => rows_unread += 1,
        }
    }
    let absent = valid.iter().filter(|v| !**v).count();
    (
        DustMap {
            av,
            screen_pc,
            valid,
        },
        rows,
        rows_unread,
        aliases,
        absent,
        screen_lo,
        screen_hi,
    )
}

fn pixel_of(dust: &DustMap, ra_deg: f64, dec_deg: f64) -> Option<(f64, f64)> {
    let (theta, phi) = icrs_to_galactic(ra_deg, dec_deg);
    let pix = ang2pix_nest(NSIDE_DUST, theta, phi)? as usize;
    if dust.valid[pix] {
        Some((dust.av[pix], dust.screen_pc[pix]))
    } else {
        None
    }
}

fn load_stars(bytes: &[u8]) -> Vec<Star> {
    let stride = match star_stride(bytes) {
        Some(s) => s,
        None => {
            eprintln!(
                "star bin {} bytes: no {}-byte records — the probe stays dark",
                bytes.len(),
                STAR_RECORD_BYTES
            );
            std::process::exit(1);
        }
    };
    let mut stars = Vec::new();
    let mut refused = 0usize;
    for chunk in bytes.chunks_exact(stride) {
        match parse_star_record(chunk) {
            Some(rec) => {
                if rec.mag.is_finite() && rec.mag > 0.0 && rec.mag < 30.0 {
                    let plx = rec.plx_mas;
                    stars.push(Star {
                        ra_deg: rec.ra_deg,
                        dec_deg: rec.dec_deg,
                        d_pc: 1000.0 / plx,
                        g: rec.mag,
                        color: rec.color_index,
                        m_abs: rec.mag + 5.0 * plx.log10() - 10.0,
                    });
                } else {
                    refused += 1;
                }
            }
            None => refused += 1,
        }
    }
    eprintln!(
        "stars: {} records kept, {refused} records refused",
        stars.len()
    );
    stars
}

fn gal_of(ra_deg: f64, dec_deg: f64) -> (f64, f64) {
    let (theta, phi) = icrs_to_galactic(ra_deg, dec_deg);
    (phi.to_degrees(), 90.0 - theta.to_degrees())
}

fn build_bins(stars: &[Star], dust: &DustMap, slice_lo: f64, slice_hi: f64) -> BinAgg {
    let nl = (360.0 / BIN_DEG) as usize;
    let nb = (180.0 / BIN_DEG) as usize;
    let size = nl * nb;
    let mut agg = BinAgg {
        nl,
        nb,
        cnt: vec![0u32; size],
        scnt: vec![0u32; size],
        scol: vec![0.0f64; size],
        sav: vec![0.0f64; size],
        vcnt: vec![0u32; size],
        vsum: vec![0.0f64; size],
        vsum2: vec![0.0f64; size],
    };
    let mut front_or_no_pixel = 0usize;
    let mut neg_av = 0usize;
    for s in stars {
        let (l, b) = gal_of(s.ra_deg, s.dec_deg);
        match pixel_of(dust, s.ra_deg, s.dec_deg) {
            Some((av, screen_pc)) if s.d_pc > screen_pc => {
                let il = ((l / BIN_DEG) as usize).rem_euclid(nl);
                let ib = (((b + 90.0) / BIN_DEG) as usize).clamp(0, nb - 1);
                let c = ib * nl + il;
                agg.cnt[c] += 1;
                if av.is_finite() && av >= 0.0 {
                    agg.vcnt[c] += 1;
                    agg.vsum[c] += av;
                    agg.vsum2[c] += av * av;
                    if s.m_abs >= slice_lo
                        && s.m_abs <= slice_hi
                        && s.g <= MAG_CLEAN_MAX
                        && s.color >= COLOR_MIN
                        && s.color <= COLOR_MAX
                    {
                        agg.scnt[c] += 1;
                        agg.scol[c] += s.color;
                        agg.sav[c] += av;
                    }
                } else {
                    neg_av += 1;
                }
            }
            _ => front_or_no_pixel += 1,
        }
    }
    eprintln!(
        "behind-screen aggregation: {} stars in front of the screen or without a dust pixel, {} behind stars with a refused A_V (non-finite or negative)",
        front_or_no_pixel, neg_av
    );
    agg
}

fn sep_deg(lon1: f64, lat1: f64, lon2: f64, lat2: f64) -> f64 {
    let (lo1, la1) = (lon1.to_radians(), lat1.to_radians());
    let (lo2, la2) = (lon2.to_radians(), lat2.to_radians());
    let dlat = (la1 - la2) * 0.5;
    let dlon = (lo1 - lo2) * 0.5;
    let d = dlat.sin().powi(2) + la1.cos() * la2.cos() * dlon.sin().powi(2);
    2.0 * d.sqrt().asin().to_degrees()
}

fn cone_from_bins(agg: &BinAgg, l0: f64, b0: f64, radius_deg: f64) -> Option<ConeStats> {
    let nl = agg.nl;
    let nb = agg.nb;
    let cosb = b0.to_radians().cos();
    let l_half = (radius_deg / cosb.abs().max(0.2) + 1.0) / BIN_DEG;
    let b_half = (radius_deg + 1.0) / BIN_DEG;
    let il0 = ((l0 / BIN_DEG) as usize).rem_euclid(nl);
    let ib0 = (((b0 + 90.0) / BIN_DEG) as usize).clamp(0, nb - 1);
    let lw = l_half as usize + 1;
    let bw = b_half as usize + 1;
    let mut n = 0u64;
    let mut sn = 0u64;
    let mut vn = 0u64;
    let mut vs = 0.0f64;
    let mut v2 = 0.0f64;
    let mut wsum = 0.0f64;
    let mut wx = 0.0f64;
    let mut wy = 0.0f64;
    let mut wxx = 0.0f64;
    let mut wxy = 0.0f64;
    let il_lo = il0 as isize - lw as isize;
    let il_hi = il0 as isize + lw as isize;
    let ib_lo = (ib0 as isize - bw as isize).max(0);
    let ib_hi = (ib0 as isize + bw as isize).min(nb as isize - 1);
    for iib in ib_lo..=ib_hi {
        for iil in il_lo..=il_hi {
            let il = iil.rem_euclid(nl as isize) as usize;
            let c = iib as usize * nl + il;
            if agg.cnt[c] == 0 && agg.vcnt[c] == 0 && agg.scnt[c] == 0 {
                continue;
            }
            let lc = il as f64 * BIN_DEG;
            let bc = iib as f64 * BIN_DEG - 90.0;
            if sep_deg(lc, bc, l0, b0) > radius_deg {
                continue;
            }
            n += agg.cnt[c] as u64;
            sn += agg.scnt[c] as u64;
            vn += agg.vcnt[c] as u64;
            vs += agg.vsum[c];
            v2 += agg.vsum2[c];
            let w = agg.scnt[c] as f64;
            if w > 0.0 {
                let xm = agg.sav[c] / w;
                let ym = agg.scol[c] / w;
                wsum += w;
                wx += w * xm;
                wy += w * ym;
                wxx += w * xm * xm;
                wxy += w * xm * ym;
            }
        }
    }
    if vn == 0 {
        return None;
    }
    let mean = vs / vn as f64;
    let var = (v2 / vn as f64 - mean * mean).max(0.0);
    let slope = if wsum > 0.0 {
        let denom = (wxx - wx * wx / wsum).max(0.0);
        if denom > 0.0 {
            (wxy - wx * wy / wsum) / denom
        } else {
            0.0
        }
    } else {
        0.0
    };
    Some(ConeStats {
        n: n as usize,
        sn: sn as usize,
        mean,
        sd: var.sqrt(),
        slope,
    })
}

struct ScanChoice {
    ra_deg: f64,
    dec_deg: f64,
    l: f64,
    b: f64,
    center_av: f64,
    agg_n: usize,
    agg_slice: usize,
    agg_mean: f64,
    agg_sd: f64,
    tier: usize,
    evaluated: usize,
}

struct ScanCand {
    l: f64,
    b: f64,
    n: usize,
    sn: usize,
    mean: f64,
    sd: f64,
    slope: f64,
    cav: f64,
}

fn exact_cone_regress(
    dust: &DustMap,
    stars: &[Star],
    ra_deg: f64,
    dec_deg: f64,
    radius_deg: f64,
    slice_lo: f64,
    slice_hi: f64,
) -> Option<Reg> {
    let mut x = Vec::new();
    let mut y = Vec::new();
    for s in stars {
        if sep_deg(s.ra_deg, s.dec_deg, ra_deg, dec_deg) > radius_deg {
            continue;
        }
        let Some((av, screen_pc)) = pixel_of(dust, s.ra_deg, s.dec_deg) else {
            continue;
        };
        if s.d_pc <= screen_pc || !(av.is_finite() && av >= 0.0) {
            continue;
        }
        if s.m_abs < slice_lo
            || s.m_abs > slice_hi
            || s.g > MAG_CLEAN_MAX
            || s.color < COLOR_MIN
            || s.color > COLOR_MAX
        {
            continue;
        }
        x.push(av);
        y.push(s.color);
    }
    ols(&x, &y)
}

fn scan_cone(
    dust: &DustMap,
    agg: &BinAgg,
    stars: &[Star],
    radius_deg: f64,
    slice_lo: f64,
    slice_hi: f64,
) -> Option<ScanChoice> {
    let mut passed = Vec::new();
    let mut b = -SCAN_B_MAX;
    while b <= SCAN_B_MAX {
        if b.abs() < SCAN_B_MIN {
            b += SCAN_B_STEP;
            continue;
        }
        let mut l: f64 = 0.0;
        while l < 360.0 {
            let (ra, dec) = galactic_to_icrs((90.0 - b).to_radians(), l.to_radians());
            if let Some((av, _)) = pixel_of(dust, ra, dec) {
                if av.is_finite() && av >= AV_LO && av <= AV_HI {
                    passed.push((l, b, av));
                }
            }
            l += SCAN_L_STEP;
        }
        b += SCAN_B_STEP;
    }
    eprintln!(
        "cone scan: {} candidate centers carry A_V in [{AV_LO}, {AV_HI}] at |b| in [{SCAN_B_MIN}, {SCAN_B_MAX}]",
        passed.len()
    );
    if passed.is_empty() {
        return None;
    }
    let mut qualified = Vec::new();
    let mut evaluated = 0usize;
    for (l, b, cav) in &passed {
        if sep_deg(*l, *b, 0.0, 0.0) <= GC_EXCLUDE_DEG {
            continue;
        }
        let Some(stats) = cone_from_bins(agg, *l, *b, radius_deg) else {
            continue;
        };
        evaluated += 1;
        if stats.mean >= AV_LO && stats.mean <= AV_HI {
            qualified.push(ScanCand {
                l: *l,
                b: *b,
                n: stats.n,
                sn: stats.sn,
                mean: stats.mean,
                sd: stats.sd,
                slope: stats.slope,
                cav: *cav,
            });
        }
    }
    qualified.sort_by(|a, b| b.sn.cmp(&a.sn));
    let tier1: Vec<&ScanCand> = qualified
        .iter()
        .filter(|q| q.sn >= MIN_SLICE && q.sd >= AV_SPREAD_MIN)
        .collect();
    let mut pool: Vec<&ScanCand> = if tier1.is_empty() {
        qualified.iter().collect()
    } else {
        tier1
    };
    pool.sort_by(|a, b| {
        b.slope
            .partial_cmp(&a.slope)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let eval_n = pool.len().min(SHOW_TOP);
    if eval_n == 0 {
        return None;
    }
    println!("\ncandidate pool ranked by binned (0.25 deg) slice color~A_V slope:");
    println!("  (l,b) deg | behind n | slice n | A_V mean/sd | binned slope | center A_V");
    for (i, q) in pool.iter().take(eval_n).enumerate() {
        println!(
            "  {} | ({:.2}, {:.2}) | {} | {} | {:.3} +/- {:.3} | {:.3} | {:.3}",
            i + 1,
            q.l,
            q.b,
            q.n,
            q.sn,
            q.mean,
            q.sd,
            q.slope,
            q.cav
        );
    }
    println!("\nexact candidate survey (color ~ A_V over the abs-G slice window):");
    println!(
        "  # | (l,b) deg | (ra,dec) deg | slice n (exact) | slope d(BP-RP)/dA_V | slope se | A_V sd"
    );
    let mut usable: Vec<(usize, f64, f64, usize)> = Vec::new();
    for (i, q) in pool.iter().take(eval_n).enumerate() {
        let (ra, dec) = galactic_to_icrs((90.0 - q.b).to_radians(), q.l.to_radians());
        match exact_cone_regress(dust, stars, ra, dec, radius_deg, slice_lo, slice_hi) {
            Some(reg) if reg.n >= N_EXACT_MIN && reg.x_sd >= AV_SPREAD_MIN => {
                let se = slope_se(&reg);
                println!(
                    "  {} | ({:.2}, {:.2}) | ({:.3}, {:.3}) | {} | {:.4} | {:.4} | {:.4}",
                    i + 1,
                    q.l,
                    q.b,
                    ra,
                    dec,
                    reg.n,
                    reg.slope,
                    se,
                    reg.x_sd
                );
                usable.push((i, reg.slope, se, reg.n));
            }
            Some(_) => {
                println!(
                    "  {} | ({:.2}, {:.2}) | ({:.3}, {:.3}) | n below {} or A_V sd thin — not evaluated",
                    i + 1,
                    q.l,
                    q.b,
                    ra,
                    dec,
                    N_EXACT_MIN
                );
            }
            None => {
                println!(
                    "  {} | ({:.2}, {:.2}) | ({:.3}, {:.3}) | regression void",
                    i + 1,
                    q.l,
                    q.b,
                    ra,
                    dec
                );
            }
        }
    }
    let detected: Vec<&(usize, f64, f64, usize)> = usable
        .iter()
        .filter(|u| u.1 > 0.0 && u.1 >= 2.0 * u.2)
        .collect();
    let best = if !detected.is_empty() {
        detected.iter().max_by_key(|u| u.3).copied()
    } else if !usable.is_empty() {
        usable.iter().max_by_key(|u| u.3)
    } else {
        None
    };
    let (q, tier) = match best {
        Some(u) => (pool[u.0], if detected.is_empty() { 2 } else { 1 }),
        None if !qualified.is_empty() => (&qualified[0], 3),
        None => return None,
    };
    let (ra, dec) = galactic_to_icrs((90.0 - q.b).to_radians(), q.l.to_radians());
    Some(ScanChoice {
        ra_deg: ra,
        dec_deg: dec,
        l: q.l,
        b: q.b,
        center_av: q.cav,
        agg_n: q.n,
        agg_slice: q.sn,
        agg_mean: q.mean,
        agg_sd: q.sd,
        tier,
        evaluated,
    })
}

fn collect_cone(
    dust: &DustMap,
    stars: &[Star],
    ra_deg: f64,
    dec_deg: f64,
    radius_deg: f64,
) -> ConeSample {
    let mut cone_stars = 0usize;
    let mut cone_no_pixel = 0usize;
    let mut behind_av_refused = 0usize;
    let mut color_refused = 0usize;
    let mut members = Vec::new();
    for s in stars {
        if sep_deg(s.ra_deg, s.dec_deg, ra_deg, dec_deg) > radius_deg {
            continue;
        }
        cone_stars += 1;
        let Some((av, screen_pc)) = pixel_of(dust, s.ra_deg, s.dec_deg) else {
            cone_no_pixel += 1;
            continue;
        };
        if s.d_pc <= screen_pc {
            continue;
        }
        if !(av.is_finite() && av >= 0.0) {
            behind_av_refused += 1;
            continue;
        }
        if !(s.color >= COLOR_MIN && s.color <= COLOR_MAX) {
            color_refused += 1;
            continue;
        }
        members.push(Member {
            av,
            g: s.g,
            color: s.color,
            m_abs: s.m_abs,
        });
    }
    if members.is_empty() {
        eprintln!("no behind-screen star carries a plausible A_V and color — the cone stays unmeasured (0 honored)");
        std::process::exit(1);
    }
    ConeSample {
        cone_stars,
        cone_no_pixel,
        behind_av_refused,
        color_refused,
        members,
    }
}

fn ols(x: &[f64], y: &[f64]) -> Option<Reg> {
    let n = x.len();
    if n < 3 {
        return None;
    }
    let nf = n as f64;
    let mut sx = 0.0f64;
    let mut sy = 0.0f64;
    let mut sxx = 0.0f64;
    let mut syy = 0.0f64;
    let mut sxy = 0.0f64;
    for i in 0..n {
        sx += x[i];
        sy += y[i];
        sxx += x[i] * x[i];
        syy += y[i] * y[i];
        sxy += x[i] * y[i];
    }
    let x_mean = sx / nf;
    let denom = (sxx - sx * sx / nf).max(0.0);
    if denom <= 0.0 {
        return None;
    }
    let num = sxy - sx * sy / nf;
    let slope = num / denom;
    let intercept = (sy - slope * sx) / nf;
    let ydenom = (syy - sy * sy / nf).max(0.0);
    let resid2 = (ydenom - num * num / denom).max(0.0);
    let rms = (resid2 / nf).sqrt();
    let pearson = if ydenom > 0.0 {
        num / (denom * ydenom).sqrt()
    } else {
        0.0
    };
    Some(Reg {
        n,
        slope,
        intercept,
        rms,
        pearson,
        x_mean,
        x_sd: (denom / nf).sqrt(),
    })
}

fn slope_se(reg: &Reg) -> f64 {
    reg.rms / ((reg.n as f64 - 2.0).sqrt() * reg.x_sd)
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let var = v.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / n;
    (m, var.sqrt())
}

fn median(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    let n = s.len();
    if n % 2 == 1 {
        Some(s[n / 2])
    } else {
        Some(0.5 * (s[n / 2 - 1] + s[n / 2]))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(dust_path) = arg_value(&args, "--dust") else {
        eprintln!("usage: dust_cleaning_probe --dust <planck_dust_av_rq_n512.json> --stars <dr3_stars.bin> [--cone <ra> <dec> | --cone-gal <l> <b>] [--radius <deg>] [--slice <m_lo> <m_hi>]");
        std::process::exit(1);
    };
    let Some(stars_path) = arg_value(&args, "--stars") else {
        eprintln!("--stars <dr3_stars.bin>: the star sample is never silent");
        std::process::exit(1);
    };
    let radius_deg = match f64_of(&args, "--radius") {
        Some(r) if r.is_finite() && r > 0.0 => r,
        Some(_) => {
            eprintln!("--radius carries no plausible degree count");
            std::process::exit(1);
        }
        None => RADIUS_DEFAULT_DEG,
    };
    let (slice_lo, slice_hi) = match pair_of(&args, "--slice") {
        Some((lo, hi)) if hi > lo => (lo, hi),
        Some(_) => {
            eprintln!("--slice <m_lo> <m_hi> carries no plausible magnitude window");
            std::process::exit(1);
        }
        None => (SLICE_LO_DEFAULT, SLICE_HI_DEFAULT),
    };

    println!("=== dust_cleaning_probe — Planck DL07 AV_RQ screen subtraction of Gaia DR3 ===");
    println!(
        "dust map {dust_path} | stars {stars_path} | cone radius {radius_deg} deg | abs-G slice [{slice_lo}, {slice_hi}]"
    );

    let dust_bytes = read_bytes(&dust_path, "dust map");
    let (dust, rows, rows_unread, aliases, absent, screen_lo, screen_hi) = load_dust(&dust_bytes);
    eprintln!(
        "dust: {rows} rows read ({rows_unread} unread, {aliases} alias pixels, {absent} absent of {}), screen distance {:.3}..{:.3} pc",
        12 * NSIDE_DUST * NSIDE_DUST,
        screen_lo,
        screen_hi
    );

    let star_bytes = read_bytes(&stars_path, "dr3 stars");
    let stars = load_stars(&star_bytes);

    let (sra, sdec, _) = match pair_of(&args, "--cone") {
        Some((ra, dec)) => (ra, dec, true),
        None => match pair_of(&args, "--cone-gal") {
            Some((l, b)) => {
                let (ra, dec) = galactic_to_icrs((90.0 - b).to_radians(), l.to_radians());
                (ra, dec, true)
            }
            None => {
                let agg = build_bins(&stars, &dust, slice_lo, slice_hi);
                match scan_cone(&dust, &agg, &stars, radius_deg, slice_lo, slice_hi) {
                    Some(c) => {
                        println!(
                            "\ncone scan chose (l,b) = ({:.3}, {:.3}) deg over {} evaluated cones — selection {} ({})",
                            c.l,
                            c.b,
                            c.evaluated,
                            c.tier,
                            match c.tier {
                                1 => "exact reddening slope detected at >= 2 se",
                                2 => "usable field, slope below the 2-se detection line",
                                _ => "binned fallback (no usable exact field)",
                            }
                        );
                        println!(
                            "  binned field: center A_V {:.4}, behind stars n ~ {}, abs-G-slice n ~ {}, A_V mean {:.4}, sd {:.4}",
                            c.center_av, c.agg_n, c.agg_slice, c.agg_mean, c.agg_sd
                        );
                        (c.ra_deg, c.dec_deg, false)
                    }
                    None => {
                        eprintln!(
                            "no cone meets the scan criteria — the field stays unmeasured (0 honored)"
                        );
                        std::process::exit(1);
                    }
                }
            }
        },
    };

    let cs = collect_cone(&dust, &stars, sra, sdec, radius_deg);
    let (gal_l, gal_b) = gal_of(sra, sdec);
    println!("\n=== chosen cone ===");
    println!(
        "center ICRS (ra, dec) = ({:.4}, {:.4}) deg, radius {radius_deg} deg",
        sra, sdec
    );
    println!(
        "center galactic (l, b) = ({gal_l:.3}, {gal_b:.3}) deg, |b| = {:.3} deg",
        gal_b.abs()
    );
    println!(
        "stars in cone: {} total, {} without a dust pixel; {} behind-screen members analyzed, {} behind stars with refused A_V, {} with refused color",
        cs.cone_stars,
        cs.cone_no_pixel,
        cs.members.len(),
        cs.behind_av_refused,
        cs.color_refused
    );

    let n_behind = cs.members.len();
    let avs: Vec<f64> = cs.members.iter().map(|m| m.av).collect();
    let (av_mean, av_sd) = mean_sd(&avs);
    let mut av_lo = avs[0];
    let mut av_hi = avs[0];
    for &a in &avs {
        if a < av_lo {
            av_lo = a;
        }
        if a > av_hi {
            av_hi = a;
        }
    }
    println!(
        "behind-screen A_V over {n_behind} stars: mean {av_mean:.4}, sd {av_sd:.4}, min {av_lo:.4}, max {av_hi:.4} mag"
    );

    let slice_members: Vec<&Member> = cs
        .members
        .iter()
        .filter(|m| m.m_abs >= slice_lo && m.m_abs <= slice_hi && m.g <= MAG_CLEAN_MAX)
        .collect();
    if slice_members.is_empty() {
        eprintln!("no behind-screen star in the abs-G slice — the regression stays unmeasured (0 honored)");
        std::process::exit(1);
    }

    let sx: Vec<f64> = slice_members.iter().map(|m| m.av).collect();
    let sy: Vec<f64> = slice_members.iter().map(|m| m.color).collect();
    let Some(sreg) = ols(&sx, &sy) else {
        eprintln!("the color regression carries no variance — the slope stays unmeasured");
        std::process::exit(1);
    };
    println!("\n=== measured reddening slope (step 4) ===");
    println!(
        "regression d(BP-RP)/dA_V over {} behind-screen stars in abs-G [{slice_lo}, {slice_hi}] with apparent G <= {MAG_CLEAN_MAX}:",
        sreg.n
    );
    println!(
        "slope = {:.4} mag/mag, intercept = {:.4} mag, residual rms = {:.4} mag, Pearson r = {:.4}",
        sreg.slope, sreg.intercept, sreg.rms, sreg.pearson
    );
    println!(
        "A_V over the regression sample: mean {:.4}, sd {:.4}",
        sreg.x_mean, sreg.x_sd
    );
    let se_slope = slope_se(&sreg);
    println!("slope standard error = {:.4} mag/mag", se_slope);
    println!(
        "measured slope {:.4} vs cited expectation {:.4}: difference {:.4} mag/mag ({:.1} slope-se)",
        sreg.slope,
        1.0 / WANG_A_GBP,
        sreg.slope - 1.0 / WANG_A_GBP,
        (sreg.slope - 1.0 / WANG_A_GBP) / se_slope
    );
    println!(
        "cited comparison — Wang & Chen 2019, ApJ 877, 116 (red clump stars, Gaia DR2): \"A_GBP = (2.429 +/- 0.015) E(GBP-GRP)\" and \"The extinctions in GBP, G, and GRP bands are close to those in V, r, and i bands, respectively\"; with A_GBP ~ A_V the expectation is E(BP-RP)/A_V = 1/2.429 = {:.4} mag/mag",
        1.0 / WANG_A_GBP
    );
    println!("their total-to-selective ratio: \"R_V = A_V/(A_B-A_V) = ... = 3.16 +/- 0.15\"");

    let cc_aligned: Vec<f64> = slice_members
        .iter()
        .map(|m| m.color - sreg.slope * m.av)
        .collect();
    let med = match median(&cc_aligned) {
        Some(m) => m,
        None => {
            eprintln!("corrected colors carry no median");
            std::process::exit(1);
        }
    };
    let k_wc = WANG_A_G * sreg.slope;
    let kreg = ols_band(&slice_members, &cc_aligned, med);
    println!("\n=== G-band ratio k = A_G/A_V (step 5) ===");
    let (k_apply, k_source) = match kreg {
        Some(reg) => {
            println!(
                "measured k = A_G/A_V over {} stars in a corrected-color band of +/- {COLOR_BAND_HALF} mag around {med:.4} (the median corrected color of the slice):",
                reg.n
            );
            println!(
                "k = {:.4}, residual rms {:.4} mag, Pearson r = {:.4}, A_V sd = {:.4}",
                reg.slope, reg.rms, reg.pearson, reg.x_sd
            );
            let se_k = slope_se(&reg);
            println!("k standard error = {:.4}", se_k);
            println!(
                "cross-check from the cited law (\"A_G = (1.890 +/- 0.015) E(GBP-GRP)\", Wang & Chen 2019) with the measured slope: k = 1.890 * d(BP-RP)/dA_V = {:.4}",
                k_wc
            );
            (reg.slope, "measured")
        }
        None => {
            println!(
                "the k regression carries no usable variance; the probe applies k = 1.890 * d(BP-RP)/dA_V = {k_wc:.4} from Wang & Chen 2019 (\"A_G = (1.890 +/- 0.015) E(GBP-GRP)\")"
            );
            (k_wc, "Wang & Chen 2019 law (1.890 * measured slope)")
        }
    };

    println!("\n=== color-magnitude outcome (step 6) ===");
    println!(
        "behind-screen members (n = {n_behind}) corrected as G - {k_apply:.4}*A_V, BP-RP - {:.4}*A_V:",
        sreg.slope
    );
    let bbp: Vec<f64> = cs.members.iter().map(|m| m.color).collect();
    let abp: Vec<f64> = cs
        .members
        .iter()
        .map(|m| m.color - sreg.slope * m.av)
        .collect();
    let bmg: Vec<f64> = cs.members.iter().map(|m| m.m_abs).collect();
    let amg: Vec<f64> = cs
        .members
        .iter()
        .map(|m| m.m_abs - k_apply * m.av)
        .collect();
    let (bbp_m, bbp_s) = mean_sd(&bbp);
    let (abp_m, abp_s) = mean_sd(&abp);
    let (bmg_m, bmg_s) = mean_sd(&bmg);
    let (amg_m, amg_s) = mean_sd(&amg);
    println!(
        "  BP-RP  before: mean {bbp_m:.4}  sd {bbp_s:.4} | after: mean {abp_m:.4}  sd {abp_s:.4}"
    );
    println!(
        "  abs-G  before: mean {bmg_m:.4}  sd {bmg_s:.4} | after: mean {amg_m:.4}  sd {amg_s:.4}"
    );

    let n_s = slice_members.len();
    let sbp: Vec<f64> = slice_members.iter().map(|m| m.color).collect();
    let sap: Vec<f64> = slice_members
        .iter()
        .map(|m| m.color - sreg.slope * m.av)
        .collect();
    let smg: Vec<f64> = slice_members.iter().map(|m| m.m_abs).collect();
    let sam: Vec<f64> = slice_members
        .iter()
        .map(|m| m.m_abs - k_apply * m.av)
        .collect();
    let (sbp_m, sbp_s) = mean_sd(&sbp);
    let (sap_m, sap_s) = mean_sd(&sap);
    let (smg_m, smg_s) = mean_sd(&smg);
    let (sam_m, sam_s) = mean_sd(&sam);
    println!(
        "abs-G slice population (n = {n_s}, the regression set; apparent G <= {MAG_CLEAN_MAX}):"
    );
    println!(
        "  BP-RP  before: mean {sbp_m:.4}  sd {sbp_s:.4} | after: mean {sap_m:.4}  sd {sap_s:.4}"
    );
    println!(
        "  abs-G  before: mean {smg_m:.4}  sd {smg_s:.4} | after: mean {sam_m:.4}  sd {sam_s:.4}"
    );

    println!("  abs-G bins (corrected abs-G): n | BP-RP sd before -> after");
    let mut bin_start = slice_lo;
    while bin_start < slice_hi {
        let bin_end = (bin_start + 1.0).min(slice_hi);
        let bin: Vec<&&Member> = slice_members
            .iter()
            .filter(|m| {
                let c = m.m_abs - k_apply * m.av;
                c >= bin_start && c < bin_end
            })
            .collect();
        if !bin.is_empty() {
            let bv: Vec<f64> = bin.iter().map(|m| m.color).collect();
            let avv: Vec<f64> = bin.iter().map(|m| m.color - sreg.slope * m.av).collect();
            let (_, bs) = mean_sd(&bv);
            let (_, asd) = mean_sd(&avv);
            println!(
                "    [{bin_start:.2}, {bin_end:.2}): n = {}  sd {bs:.4} -> {asd:.4}",
                bin.len()
            );
        }
        bin_start = bin_end;
    }

    println!(
        "\ncorrection summary: slope {:.4} mag/mag (measured), k {k_apply:.4} ({k_source})",
        sreg.slope
    );
}

fn ols_band(members: &[&Member], cc: &[f64], med: f64) -> Option<Reg> {
    let idx: Vec<usize> = (0..cc.len())
        .filter(|&i| (cc[i] - med).abs() <= COLOR_BAND_HALF)
        .collect();
    let xk: Vec<f64> = idx.iter().map(|&i| members[i].av).collect();
    let yk: Vec<f64> = idx.iter().map(|&i| members[i].m_abs).collect();
    let reg = ols(&xk, &yk)?;
    if reg.n < K_OLS_MIN || reg.x_sd < AV_SPREAD_MIN {
        None
    } else {
        Some(reg)
    }
}
