use omegaflow::archivar::kbo::{KboRec, family_name, name_of, rec_from_row, state_at};
use omegaflow::json::{JsonVal, parse_json};
use std::f64::consts::TAU;

const NLON: usize = 72;
const NLAT: usize = 36;
const CELL: f64 = 5.0;
const BAND_FRACTION: f64 = 0.90;
const N_NULL: usize = 2000;
const SEED: u64 = 0x0D1E_9A17_3C6B_0020;
const OBLIQ_DEG: f64 = 23.439_291_111;
const NGP_RA: f64 = 192.859_48;
const NGP_DEC: f64 = 27.128_25;

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn angle(&mut self) -> f64 {
        self.unit() * TAU
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next_u64() >> 33) as usize % (i + 1);
            v.swap(i, j);
        }
    }
}

fn lon_bin(lon_deg: f64) -> usize {
    ((lon_deg.rem_euclid(360.0) / CELL).floor() as usize).min(NLON - 1)
}

fn lat_bin(lat_deg: f64) -> usize {
    ((((lat_deg + 90.0) / CELL).floor() as isize).clamp(0, NLAT as isize - 1)) as usize
}

fn cell(lon_deg: f64, lat_deg: f64) -> usize {
    lon_bin(lon_deg) * NLAT + lat_bin(lat_deg)
}

fn icrs_to_ecliptic(v: [f64; 3]) -> (f64, f64) {
    let (se, ce) = OBLIQ_DEG.to_radians().sin_cos();
    let x = v[0];
    let y = v[1] * ce + v[2] * se;
    let z = -v[1] * se + v[2] * ce;
    let lon = y.atan2(x).to_degrees().rem_euclid(360.0);
    let lat = (z / (x * x + y * y + z * z).sqrt())
        .clamp(-1.0, 1.0)
        .asin()
        .to_degrees();
    (lon, lat)
}

fn radec_to_ecliptic(ra_deg: f64, dec_deg: f64) -> (f64, f64) {
    let (sr, cr) = ra_deg.to_radians().sin_cos();
    let (sd, cd) = dec_deg.to_radians().sin_cos();
    icrs_to_ecliptic([cd * cr, cd * sr, sd])
}

fn galactic_lat(ra_deg: f64, dec_deg: f64) -> f64 {
    let (_sr, cr) = ra_deg.to_radians().sin_cos();
    let (sd, cd) = dec_deg.to_radians().sin_cos();
    let (sn, cn) = NGP_DEC.to_radians().sin_cos();
    let c = cr * cd * cn * (ra_deg - NGP_RA).to_radians().cos() + sd * sn;
    c.clamp(-1.0, 1.0).asin().to_degrees()
}

fn galactic_lat_icrs(v: [f64; 3]) -> f64 {
    let (sn, cn) = NGP_DEC.to_radians().sin_cos();
    let (sr, cr) = NGP_RA.to_radians().sin_cos();
    let n = [cn * cr, cn * sr, sn];
    let norm = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if norm <= 0.0 {
        return 0.0;
    }
    ((v[0] * n[0] + v[1] * n[1] + v[2] * n[2]) / norm)
        .clamp(-1.0, 1.0)
        .asin()
        .to_degrees()
}

fn circ_mean_r(vals: &[f64]) -> (f64, f64) {
    if vals.is_empty() {
        return (0.0, 0.0);
    }
    let (mut sx, mut sy) = (0.0, 0.0);
    for &v in vals {
        let a = v.to_radians();
        sx += a.cos();
        sy += a.sin();
    }
    let n = vals.len() as f64;
    (
        (sx * sx + sy * sy).sqrt() / n,
        sy.atan2(sx).to_degrees().rem_euclid(360.0),
    )
}

fn circ_corr(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 3 {
        return f64::NAN;
    }
    let (_, mx) = circ_mean_r(x);
    let (_, my) = circ_mean_r(y);
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    for i in 0..n {
        let a = (x[i] - mx).to_radians().sin();
        let b = (y[i] - my).to_radians().sin();
        sxy += a * b;
        sxx += a * a;
        syy += b * b;
    }
    if sxx <= 0.0 || syy <= 0.0 {
        return f64::NAN;
    }
    sxy / (sxx * syy).sqrt()
}

fn pct(part: usize, total: usize) -> f64 {
    if total == 0 {
        f64::NAN
    } else {
        100.0 * part as f64 / total as f64
    }
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.iter().sum::<f64>() / v.len() as f64
}

fn sd(v: &[f64]) -> f64 {
    if v.len() < 2 {
        return f64::NAN;
    }
    let m = mean(v);
    (v.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt()
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let idx = ((sorted.len() - 1) as f64 * q).round() as usize;
    sorted[idx]
}

struct Pnrp {
    n: usize,
    lon: Vec<f64>,
    lat: Vec<f64>,
    gal_lat: Vec<f64>,
    any_det: Vec<bool>,
    ztf: usize,
    des: usize,
    ps1: usize,
}

fn parse_pnrp(path: &str) -> Option<Pnrp> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut p = Pnrp {
        n: 0,
        lon: Vec::new(),
        lat: Vec::new(),
        gal_lat: Vec::new(),
        any_det: Vec::new(),
        ztf: 0,
        des: 0,
        ps1: 0,
    };
    let mut header_seen = false;
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if !header_seen {
            header_seen = true;
            continue;
        }
        let f: Vec<&str> = t.split(',').map(|s| s.trim()).collect();
        if f.len() < 17 {
            continue;
        }
        let (Ok(ra), Ok(dec)) = (f[8].parse::<f64>(), f[9].parse::<f64>()) else {
            continue;
        };
        let (lon, lat) = radec_to_ecliptic(ra, dec);
        let ztf = matches!(f[14].parse::<i32>(), Ok(v) if v != 0);
        let des = matches!(f[15].parse::<i32>(), Ok(v) if v != 0);
        let ps1 = matches!(f[16].parse::<i32>(), Ok(v) if v != 0);
        p.lon.push(lon);
        p.lat.push(lat);
        p.gal_lat.push(galactic_lat(ra, dec));
        p.any_det.push(ztf || des || ps1);
        p.ztf += ztf as usize;
        p.des += des as usize;
        p.ps1 += ps1 as usize;
        p.n += 1;
    }
    if p.n == 0 { None } else { Some(p) }
}

struct Band {
    cells: Vec<bool>,
    area_frac: f64,
    p90_cells: usize,
}

fn build_band(p: &Pnrp) -> Band {
    let mut counts = vec![0.0f64; NLON * NLAT];
    for i in 0..p.n {
        counts[cell(p.lon[i], p.lat[i])] += 1.0;
    }
    let mut sm = vec![0.0f64; NLON * NLAT];
    for il in 0..NLON {
        for ib in 0..NLAT {
            let mut s = 0.0;
            for dl in -1i32..=1 {
                let jl = ((il as i32 + dl).rem_euclid(NLON as i32)) as usize;
                for db in -1i32..=1 {
                    let jb = ib as i32 + db;
                    if !(0..NLAT as i32).contains(&jb) {
                        continue;
                    }
                    let w = if dl == 0 { 2.0 } else { 1.0 } * if db == 0 { 2.0 } else { 1.0 };
                    s += w * counts[jl * NLAT + jb as usize];
                }
            }
            sm[il * NLAT + ib] = s;
        }
    }
    let mut order: Vec<usize> = (0..NLON * NLAT).collect();
    order.sort_by(|&a, &b| {
        sm[b]
            .partial_cmp(&sm[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let total: f64 = sm.iter().sum();
    let mut acc = 0.0;
    let mut cells = vec![false; NLON * NLAT];
    let mut p90_cells = 0;
    for &idx in &order {
        if acc / total >= BAND_FRACTION {
            break;
        }
        cells[idx] = true;
        acc += sm[idx];
        p90_cells += 1;
    }
    let mut band_area = 0.0;
    let mut all_area = 0.0;
    for il in 0..NLON {
        for ib in 0..NLAT {
            let b = -90.0 + (ib as f64 + 0.5) * CELL;
            let w = b.to_radians().cos().max(0.0);
            all_area += w;
            if cells[il * NLAT + ib] {
                band_area += w;
            }
        }
    }
    Band {
        cells,
        area_frac: band_area / all_area,
        p90_cells,
    }
}

fn in_band(band: &Band, lon: f64, lat: f64) -> bool {
    band.cells[cell(lon, lat)]
}

fn load_kbo(path: &str) -> Vec<KboRec> {
    let Ok(text) = std::fs::read_to_string(path) else {
        eprintln!("kbo: {path} reads void");
        return Vec::new();
    };
    let Some(root) = parse_json(&text) else {
        eprintln!("kbo: {path} json void");
        return Vec::new();
    };
    let JsonVal::Obj(m) = &root else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(rows)) = m.get("data") else {
        return Vec::new();
    };
    rows.iter().filter_map(rec_from_row).collect()
}

struct Obj {
    name: String,
    lon: f64,
    lat: f64,
    gal_lat: f64,
    varpi: f64,
    omega: f64,
    inc: f64,
    loga: f64,
    a: f64,
    e: f64,
    p9: bool,
    family: u8,
}

fn is_p9(a: f64, e: f64) -> bool {
    a >= 250.0 && a * (1.0 - e) >= 30.0
}

struct Vals {
    band: [f64; 5],
    glob: [f64; 2],
    n_p9: usize,
}

fn eval(
    objs: &[Obj],
    band: &Band,
    delta_deg: f64,
    perm: Option<&[usize]>,
    uniform_glob: bool,
    rng: &mut Rng,
) -> Vals {
    let n = objs.len();
    let mut n_p9 = 0usize;
    let mut n_in = 0usize;
    let mut n_p9_in = 0usize;
    let mut varpi_in = Vec::new();
    let mut omega_in = Vec::new();
    let mut inc_in = Vec::new();
    let mut inc_out = Vec::new();
    let mut loga_in = Vec::new();
    let mut loga_out = Vec::new();
    let mut varpi_p9 = Vec::new();
    let mut omega_p9 = Vec::new();

    for i in 0..n {
        let o = &objs[i];
        let j = perm.map(|p| p[i]).unwrap_or(i);
        let e = &objs[j];
        let p9 = is_p9(e.a, e.e);
        let inside = in_band(band, o.lon + delta_deg, o.lat);
        if inside {
            n_in += 1;
        }
        if p9 {
            n_p9 += 1;
            varpi_p9.push(e.varpi);
            omega_p9.push(e.omega);
            if inside {
                n_p9_in += 1;
                varpi_in.push(e.varpi);
                omega_in.push(e.omega);
                inc_in.push(e.inc);
                loga_in.push(e.loga);
            } else {
                inc_out.push(e.inc);
                loga_out.push(e.loga);
            }
        }
    }

    let r_varpi_in = if varpi_in.len() >= 3 {
        circ_mean_r(&varpi_in).0
    } else {
        f64::NAN
    };
    let r_omega_in = if omega_in.len() >= 3 {
        circ_mean_r(&omega_in).0
    } else {
        f64::NAN
    };
    let inc_shift = if !inc_in.is_empty() && !inc_out.is_empty() {
        mean(&inc_in) - mean(&inc_out)
    } else {
        f64::NAN
    };
    let loga_shift = if !loga_in.is_empty() && !loga_out.is_empty() {
        mean(&loga_in) - mean(&loga_out)
    } else {
        f64::NAN
    };
    let enrich = if n_p9 > 0 && n_in > 0 && n_p9_in > 0 {
        ((n_p9_in as f64 / n_p9 as f64) / (n_in as f64 / n as f64)).ln()
    } else {
        f64::NAN
    };

    let (r_varpi_all, r_omega_all) = if uniform_glob {
        let mut a: Vec<f64> = (0..n_p9).map(|_| rng.angle().to_degrees()).collect();
        let r1 = circ_mean_r(&a).0;
        a.clear();
        a.extend((0..n_p9).map(|_| rng.angle().to_degrees()));
        let r2 = circ_mean_r(&a).0;
        (r1, r2)
    } else {
        (
            if varpi_p9.len() >= 3 {
                circ_mean_r(&varpi_p9).0
            } else {
                f64::NAN
            },
            if omega_p9.len() >= 3 {
                circ_mean_r(&omega_p9).0
            } else {
                f64::NAN
            },
        )
    };

    Vals {
        band: [r_varpi_in, r_omega_in, inc_shift, loga_shift, enrich],
        glob: [r_varpi_all, r_omega_all],
        n_p9,
    }
}

const THREAD_NAMES: [&str; 7] = [
    "ϖ Rayleigh R in band",
    "Ω Rayleigh R in band",
    "i mean shift in−out",
    "log10 a mean shift in−out",
    "P9 enrichment in band",
    "ϖ Rayleigh R (P9 selection, global)",
    "Ω Rayleigh R (P9 selection, global)",
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut kbo_path = "kbo_elements.json".to_string();
    let mut pnrp_path = "reference_population.csv".to_string();
    let mut draws = N_NULL;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--kbo" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    kbo_path = v.clone();
                }
            }
            "--pnrp" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    pnrp_path = v.clone();
                }
            }
            "--draws" => {
                i += 1;
                draws = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(draws);
            }
            _ => {
                eprintln!(
                    "usage: planet_nine_netz_probe [--kbo <json>] [--pnrp <csv>] [--draws N]"
                );
                return;
            }
        }
        i += 1;
    }

    let Some(pnrp) = parse_pnrp(&pnrp_path) else {
        println!("PNRP: {pnrp_path} absent — no band, no verdict");
        return;
    };
    let band = build_band(&pnrp);
    let recs = load_kbo(&kbo_path);
    if recs.is_empty() {
        println!("KBO: {kbo_path} absent — no verdict");
        return;
    }

    let mut objs: Vec<Obj> = Vec::with_capacity(recs.len());
    let mut gaps = 0usize;
    for r in &recs {
        let Some((pos, _)) = state_at(r, r.epoch_jd) else {
            gaps += 1;
            continue;
        };
        let (lon, lat) = icrs_to_ecliptic(pos);
        objs.push(Obj {
            name: name_of(r).to_string(),
            lon,
            lat,
            gal_lat: galactic_lat_icrs(pos),
            varpi: (r.node_deg + r.peri_deg).rem_euclid(360.0),
            omega: r.node_deg.rem_euclid(360.0),
            inc: r.incl_deg,
            loga: r.a_au.log10(),
            a: r.a_au,
            e: r.e,
            p9: is_p9(r.a_au, r.e),
            family: r.family,
        });
    }
    let n = objs.len();
    let n_p9 = objs.iter().filter(|o| o.p9).count();
    let n_in = objs.iter().filter(|o| in_band(&band, o.lon, o.lat)).count();
    let n_p9_in = objs
        .iter()
        .filter(|o| o.p9 && in_band(&band, o.lon, o.lat))
        .count();

    let (p_r, p_mean) = circ_mean_r(&pnrp.lon);
    let p_lat_mean = mean(&pnrp.lat);
    let gal10 = pnrp.gal_lat.iter().filter(|b| b.abs() < 10.0).count();
    let gal20 = pnrp.gal_lat.iter().filter(|b| b.abs() < 20.0).count();
    let det_any = pnrp.any_det.iter().filter(|&&b| b).count();
    let mut band_mass = 0.0;
    for i in 0..pnrp.n {
        if in_band(&band, pnrp.lon[i], pnrp.lat[i]) {
            band_mass += 1.0;
        }
    }

    println!("=== Planet Nine Net — PNRP 3.0 band vs harvested KBO catalogue ===");
    println!(
        "seed 0x{SEED:016X}, null draws {draws}, band = smoothed 5°×5° {:.0}% contour",
        BAND_FRACTION * 100.0
    );
    println!("PNRP source: {pnrp_path} (CaltechDATA 8fjad-x7y61, Version 3.0, epoch JD 2458270.0)");
    println!(
        "PNRP n {} | ecliptic lon mean {:.1}° R {:.3} | lat mean {:.1}° | |b_gal|<10° {:.1}%, <20° {:.1}% | band cells {} ({:.1}% of sky) | band holds {:.1}% of PNRP",
        pnrp.n,
        p_mean,
        p_r,
        p_lat_mean,
        100.0 * gal10 as f64 / pnrp.n as f64,
        100.0 * gal20 as f64 / pnrp.n as f64,
        band.p90_cells,
        band.area_frac * 100.0,
        100.0 * band_mass / pnrp.n as f64
    );
    println!(
        "PNRP detection flags: ZTF {} ({:.1}%), DES {} ({:.1}%), PS1 {} ({:.1}%), any {:.1}% | LSST/MPC absent (no flag in v3.0 CSV)",
        pnrp.ztf,
        100.0 * pnrp.ztf as f64 / pnrp.n as f64,
        pnrp.des,
        100.0 * pnrp.des as f64 / pnrp.n as f64,
        pnrp.ps1,
        100.0 * pnrp.ps1 as f64 / pnrp.n as f64,
        100.0 * det_any as f64 / pnrp.n as f64
    );
    println!(
        "KBO n {n} ({} series gaps) | P9 selection n {n_p9} | all in band {} ({:.1}%) | P9 in band {n_p9_in} ({:.1}%)",
        gaps,
        n_in,
        pct(n_in, n),
        pct(n_p9_in, n_p9)
    );

    let mut rng = Rng::new(SEED);
    let obs = eval(&objs, &band, 0.0, None, false, &mut rng);
    let (obs_varpi_all, obs_varpi_mean) = {
        let v: Vec<f64> = objs.iter().filter(|o| o.p9).map(|o| o.varpi).collect();
        circ_mean_r(&v)
    };
    let (obs_omega_all, obs_omega_mean) = {
        let v: Vec<f64> = objs.iter().filter(|o| o.p9).map(|o| o.omega).collect();
        circ_mean_r(&v)
    };
    println!(
        "global P9: ϖ R {:.4} mean {:.1}° | Ω R {:.4} mean {:.1}°",
        obs_varpi_all, obs_varpi_mean, obs_omega_all, obs_omega_mean
    );
    let p9_in_obs: Vec<&Obj> = objs
        .iter()
        .filter(|o| o.p9 && in_band(&band, o.lon, o.lat))
        .collect();
    let (r_varpi_in, m_varpi_in) =
        circ_mean_r(&p9_in_obs.iter().map(|o| o.varpi).collect::<Vec<_>>());
    let (r_omega_in, m_omega_in) =
        circ_mean_r(&p9_in_obs.iter().map(|o| o.omega).collect::<Vec<_>>());
    println!(
        "P9 in band: ϖ R {:.4} mean {:.1}° | Ω R {:.4} mean {:.1}°",
        r_varpi_in, m_varpi_in, r_omega_in, m_omega_in
    );
    let p9_omega: Vec<f64> = objs.iter().filter(|o| o.p9).map(|o| o.omega).collect();
    let p9_varpi: Vec<f64> = objs.iter().filter(|o| o.p9).map(|o| o.varpi).collect();
    let p9_lon: Vec<f64> = objs.iter().filter(|o| o.p9).map(|o| o.lon).collect();
    println!(
        "P9 circular correlation with ecliptic longitude λ: Ω–λ {:.3}, ϖ–λ {:.3}",
        circ_corr(&p9_omega, &p9_lon),
        circ_corr(&p9_varpi, &p9_lon)
    );

    let mut nulls: Vec<Vec<Vec<f64>>> = vec![vec![Vec::with_capacity(draws); 7]; 3];
    let mut n_p9_aux: Vec<Vec<usize>> = vec![Vec::with_capacity(draws); 3];
    let mut rng = Rng::new(SEED ^ 0xA5A5_5A5A_1234_5678);
    let mut perm: Vec<usize> = (0..n).collect();
    for mode in 0..3 {
        for _ in 0..draws {
            let delta = if mode == 0 { 0.0 } else { rng.unit() * 360.0 };
            let use_perm = mode != 1;
            let p_ref = if use_perm {
                rng.shuffle(&mut perm);
                Some(perm.as_slice())
            } else {
                None
            };
            let v = eval(&objs, &band, delta, p_ref, true, &mut rng);
            for t in 0..5 {
                nulls[mode][t].push(v.band[t]);
            }
            for t in 0..2 {
                nulls[mode][5 + t].push(v.glob[t]);
            }
            n_p9_aux[mode].push(v.n_p9);
        }
    }

    println!();
    println!("=== Thread rank list (observed vs null; z = (obs − null mean)/null sd) ===");
    println!(
        "{:<36} {:>10} {:>9} {:>9} {:>8} {:>8} {:>8} {:>8}",
        "thread", "observed", "null mean", "null sd", "z_shuf", "z_rot", "z_joint", "p2_joint"
    );
    let obs_all = [
        obs.band[0],
        obs.band[1],
        obs.band[2],
        obs.band[3],
        obs.band[4],
        obs.glob[0],
        obs.glob[1],
    ];
    let stats_by_mode: Vec<Vec<(f64, f64)>> = (0..3)
        .map(|mode| {
            (0..7)
                .map(|t| (mean(&nulls[mode][t]), sd(&nulls[mode][t])))
                .collect()
        })
        .collect();
    let stats = stats_by_mode[2].clone(); // joint null is the rank-list reference
    for t in 0..7 {
        let (m, s) = stats[t];
        let z_at = |mode: usize| {
            let (mm, ss) = stats_by_mode[mode][t];
            if ss.is_finite() && ss > 0.0 {
                (obs_all[t] - mm) / ss
            } else {
                f64::NAN
            }
        };
        let p2 = if s.is_finite() && s > 0.0 {
            let vals = &nulls[2][t];
            let ao = (obs_all[t] - m).abs();
            let c = vals.iter().filter(|&&v| (v - m).abs() >= ao).count();
            (c as f64 + 1.0) / (vals.len() as f64 + 1.0)
        } else {
            f64::NAN
        };
        println!(
            "{:<36} {:>10.4} {:>9.4} {:>9.4} {:>8.2} {:>8.2} {:>8.2} {:>8.3}",
            THREAD_NAMES[t],
            obs_all[t],
            m,
            s,
            z_at(0),
            z_at(1),
            z_at(2),
            p2
        );
    }

    let mut q95_by_mode = [0.0f64; 3];
    let mut q99_by_mode = [0.0f64; 3];
    let mut obs_max_by_mode = [0.0f64; 3];
    let mut obs_max_thread = [0usize; 3];
    let mut p_look_by_mode = [1.0f64; 3];
    for mode in 0..3 {
        let stats_mode: Vec<(f64, f64)> = (0..7)
            .map(|t| (mean(&nulls[mode][t]), sd(&nulls[mode][t])))
            .collect();
        let mut maxima = Vec::with_capacity(draws);
        for r in 0..draws {
            let mut mx = 0.0f64;
            for t in 0..7 {
                let (m, s) = stats_mode[t];
                if s.is_finite() && s > 0.0 {
                    let z = ((nulls[mode][t][r] - m) / s).abs();
                    if z.is_finite() && z > mx {
                        mx = z;
                    }
                }
            }
            maxima.push(mx);
        }
        let mut om = 0.0f64;
        for t in 0..7 {
            let (m, s) = stats_mode[t];
            if s.is_finite() && s > 0.0 {
                let z = ((obs_all[t] - m) / s).abs();
                if z.is_finite() && z > om {
                    om = z;
                    obs_max_thread[mode] = t;
                }
            }
        }
        obs_max_by_mode[mode] = om;
        p_look_by_mode[mode] = (maxima.iter().filter(|&&m| m >= om).count() as f64 + 1.0)
            / (maxima.len() as f64 + 1.0);
        maxima.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        q95_by_mode[mode] = quantile(&maxima, 0.95);
        q99_by_mode[mode] = quantile(&maxima, 0.99);
    }

    println!();
    println!("=== Max-null over all 7 threads (look-elsewhere) ===");
    println!(
        "{:<28} {:>10} {:>10} {:>10} {:>8} {:<28} {:>8}",
        "null family", "observed max", "q95", "q99", "p_look", "observed max thread", "verdict"
    );
    for (mode, name) in [
        (0, "shuffle only"),
        (1, "band rotation only"),
        (2, "joint shuffle+rotation"),
    ] {
        let verdict = if obs_max_by_mode[mode] > q99_by_mode[mode] {
            "above q99 — exploratory"
        } else if obs_max_by_mode[mode] > q95_by_mode[mode] {
            "above q95 — exploratory"
        } else {
            "below q95 — no excess"
        };
        println!(
            "{:<28} {:>10.2} {:>10.2} {:>10.2} {:>8.4} {:<28} {:>8}",
            name,
            obs_max_by_mode[mode],
            q95_by_mode[mode],
            q99_by_mode[mode],
            p_look_by_mode[mode],
            THREAD_NAMES[obs_max_thread[mode]],
            verdict
        );
    }

    println!();
    println!("=== Band composition (bias rifts, observed) ===");
    let mut fam_in: Vec<(u8, usize, usize)> = Vec::new();
    for f in 0u8..=9 {
        let tot = objs.iter().filter(|o| o.family == f).count();
        let inn = objs
            .iter()
            .filter(|o| o.family == f && in_band(&band, o.lon, o.lat))
            .count();
        if tot > 0 {
            fam_in.push((f, tot, inn));
        }
    }
    for (f, tot, inn) in fam_in {
        println!(
            "{:<10} n {:>5} in band {:>5} ({:.1}%)",
            family_name(f),
            tot,
            inn,
            100.0 * inn as f64 / tot as f64
        );
    }
    let p9_in: Vec<&Obj> = objs
        .iter()
        .filter(|o| o.p9 && in_band(&band, o.lon, o.lat))
        .collect();
    println!(
        "P9-in-band n {} | mean i {:.1}° | mean log10 a {:.3} | mean a {:.1} AU",
        p9_in.len(),
        mean(&p9_in.iter().map(|o| o.inc).collect::<Vec<_>>()),
        mean(&p9_in.iter().map(|o| o.loga).collect::<Vec<_>>()),
        mean(&p9_in.iter().map(|o| o.a).collect::<Vec<_>>())
    );
    let p9_out: Vec<&Obj> = objs
        .iter()
        .filter(|o| o.p9 && !in_band(&band, o.lon, o.lat))
        .collect();
    println!(
        "P9-out-band n {} | mean i {:.1}° | mean log10 a {:.3} | mean a {:.1} AU",
        p9_out.len(),
        mean(&p9_out.iter().map(|o| o.inc).collect::<Vec<_>>()),
        mean(&p9_out.iter().map(|o| o.loga).collect::<Vec<_>>()),
        mean(&p9_out.iter().map(|o| o.a).collect::<Vec<_>>())
    );
    let mut p9_in_sorted: Vec<&Obj> = p9_in.clone();
    p9_in_sorted.sort_by(|a, b| {
        a.omega
            .partial_cmp(&b.omega)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    println!("P9-in-band detail (sorted by Ω):");
    for o in p9_in_sorted {
        println!(
            "  {:<28} Ω {:6.1}° ϖ {:6.1}° i {:5.1}° a {:7.1} AU | ecl lon {:6.1}° lat {:5.1}° | b_gal {:6.1}°",
            o.name, o.omega, o.varpi, o.inc, o.a, o.lon, o.lat, o.gal_lat
        );
    }
    let all_gal10 = objs.iter().filter(|o| o.gal_lat.abs() < 10.0).count();
    let all_gal20 = objs.iter().filter(|o| o.gal_lat.abs() < 20.0).count();
    let p9_gal10 = objs
        .iter()
        .filter(|o| o.p9 && o.gal_lat.abs() < 10.0)
        .count();
    let p9_gal20 = objs
        .iter()
        .filter(|o| o.p9 && o.gal_lat.abs() < 20.0)
        .count();
    println!(
        "galactic rift | all KBO |b_gal|<10° {:.1}%, <20° {:.1}% | P9 |b_gal|<10° {:.1}%, <20° {:.1}%",
        pct(all_gal10, n),
        pct(all_gal20, n),
        pct(p9_gal10, n_p9),
        pct(p9_gal20, n_p9)
    );

    println!();
    println!("=== Pre-fix paper row (not comparable to the post-fix surrogate machine) ===");
    println!(
        "paper n 44: ϖ R 5.83e-2 vs uniform null mean 1.35e-1, threshold 2.770e-1 → still; fam 7.2822e-1 pre-fix, no family above. This row is carried, not re-measured, and is excluded from the max-null."
    );
    println!(
        "today's probe measures the angular threads and the band geometry; it does not rerun the ±400 yr N-body residue/TE (the paper's fam-machine state is pre-fix)."
    );
}
