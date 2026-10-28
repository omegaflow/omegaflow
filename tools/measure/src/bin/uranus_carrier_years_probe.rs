use std::collections::HashMap;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::ephemeris::body_of_ephemeris_asset;
use omegaflow::archivar::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use omegaflow::archivar::{
    BodyEphemeris, C_LIGHT, body_barycenter_position, body_fixed_to_icrs,
    body_fixed_to_icrs_smooth, embedded_lsk, fetch_raw_bytes, light_time_worldline,
    parse_ephemeris_binary,
};
use omegaflow::cdn::CDN_BASE;
use omegaflow::odp::dsn_host;

const OBS_LAT_DEG: f64 = -22.534444444;
const OBS_LON_DEG: f64 = -45.5825;
const OBS_ALT_M: f64 = 1810.7;
const UNIX_JD_OFFSET: f64 = 2440587.5;
const MAS_PER_RAD: f64 = 206_264_806.247_096_36;
const JD_J2000: f64 = 2451545.0;

struct Obs {
    jd_utc: f64,
    ra_deg: f64,
    dec_deg: f64,
    e_ra_mas: f64,
    e_dec_mas: f64,
}

struct Line {
    word: &'static str,
    map: Option<HashMap<String, BodyEphemeris>>,
    host: String,
    target: String,
}

struct Row {
    dra: f64,
    ddec: f64,
    par_ra: f64,
    par_dec: f64,
    aber_ra: f64,
    aber_dec: f64,
}

struct Fit {
    ata: [[f64; 4]; 4],
    atb: [f64; 4],
    n: usize,
}

impl Fit {
    fn new() -> Fit {
        Fit {
            ata: [[0.0; 4]; 4],
            atb: [0.0; 4],
            n: 0,
        }
    }
    fn row(&mut self, design: [f64; 4], y: f64) {
        for i in 0..4 {
            for j in 0..4 {
                self.ata[i][j] += design[i] * design[j];
            }
            self.atb[i] += design[i] * y;
        }
        self.n += 1;
    }
}

fn solve4(ata: [[f64; 4]; 4], atb: [f64; 4]) -> Option<[f64; 4]> {
    let mut a = ata;
    let mut b = atb;
    for col in 0..4 {
        let mut piv = col;
        for r in (col + 1)..4 {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        if a[piv][col].abs() < 1e-300 {
            return None;
        }
        if piv != col {
            a.swap(piv, col);
            b.swap(piv, col);
        }
        for r in (col + 1)..4 {
            let f = a[r][col] / a[col][col];
            for c in col..4 {
                a[r][c] -= f * a[col][c];
            }
            b[r] -= f * b[col];
        }
    }
    let mut x = [0.0; 4];
    for r in (0..4).rev() {
        let mut s = b[r];
        for c in (r + 1)..4 {
            s -= a[r][c] * x[c];
        }
        x[r] = s / a[r][r];
    }
    Some(x)
}

fn fit_rows(rows: &[&Row]) -> Option<[f64; 4]> {
    let mut fit = Fit::new();
    for row in rows {
        fit.row([1.0, 0.0, row.par_ra, row.aber_ra], row.dra);
        fit.row([0.0, 1.0, row.par_dec, row.aber_dec], row.ddec);
    }
    if fit.n < 8 {
        return None;
    }
    solve4(fit.ata, fit.atb)
}

fn station_offset(
    map: &HashMap<String, BodyEphemeris>,
    host: &str,
    jd_utc: f64,
) -> Option<([f64; 3], [f64; 3])> {
    let tdb = (jd_utc - JD_J2000) * 86400.0;
    let dt = 1.0;
    let at = |t: f64| -> Option<[f64; 3]> {
        let p = body_fixed_to_icrs(host, OBS_LAT_DEG, OBS_LON_DEG, OBS_ALT_M, t, map)?;
        let b = body_barycenter_position(host, t, map)?;
        Some([p[0] - b[0], p[1] - b[1], p[2] - b[2]])
    };
    let p0 = at(tdb)?;
    let pp = body_fixed_to_icrs_smooth(host, OBS_LAT_DEG, OBS_LON_DEG, OBS_ALT_M, tdb + dt, map)?;
    let bp = body_barycenter_position(host, tdb + dt, map)?;
    let pm = body_fixed_to_icrs_smooth(host, OBS_LAT_DEG, OBS_LON_DEG, OBS_ALT_M, tdb - dt, map)?;
    let bm = body_barycenter_position(host, tdb - dt, map)?;
    let vel = [
        (pp[0] - bp[0] - pm[0] + bm[0]) / (2.0 * dt),
        (pp[1] - bp[1] - pm[1] + bm[1]) / (2.0 * dt),
        (pp[2] - bp[2] - pm[2] + bm[2]) / (2.0 * dt),
    ];
    Some((p0, vel))
}

fn ensure_bin(path: &str, netloc: &str, asset: &str) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("{}/{}/{}", CDN_BASE, netloc, asset);
    let bytes = fetch_raw_bytes(&url)?;
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(path, &bytes).is_err() {
        return None;
    }
    Some(bytes)
}

fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec_len(v: [f64; 3]) -> Option<f64> {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if n.is_finite() && n > 0.0 {
        Some(n)
    } else {
        None
    }
}

fn toward_unit(from: [f64; 3], to: [f64; 3]) -> Option<[f64; 3]> {
    let d = vec_sub(to, from);
    let n = vec_len(d)?;
    Some([d[0] / n, d[1] / n, d[2] / n])
}

fn icrs_unit(ra_deg: f64, dec_deg: f64) -> [f64; 3] {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let cd = dec.cos();
    [cd * ra.cos(), cd * ra.sin(), dec.sin()]
}

fn tangent_basis(ra_deg: f64, dec_deg: f64) -> ([f64; 3], [f64; 3]) {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    (
        [-ra.sin(), ra.cos(), 0.0],
        [-ra.cos() * dec.sin(), -ra.sin() * dec.sin(), dec.cos()],
    )
}

fn parse_obs(text: &str) -> Vec<Obs> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with('-') {
            continue;
        }
        let fields: Vec<&str> = t.split('\t').map(|f| f.trim()).collect();
        if fields.len() < 5 {
            continue;
        }
        let jd_utc: f64 = match fields[0].parse::<f64>() {
            Ok(v) if v.is_finite() => v,
            _ => continue,
        };
        let Some(ra_deg) = sexagesimal_ra_to_deg(fields[1]) else {
            continue;
        };
        let Some(dec_deg) = sexagesimal_dec_to_deg(fields[3]) else {
            continue;
        };
        let e_ra_mas: f64 = match fields[2].parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => continue,
        };
        let e_dec_mas: f64 = match fields[4].parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => continue,
        };
        out.push(Obs {
            jd_utc,
            ra_deg,
            dec_deg,
            e_ra_mas,
            e_dec_mas,
        });
    }
    out
}

fn load_line(
    word: &'static str,
    netloc: &str,
    earth_asset: &str,
    uranus_asset: &str,
    eph_dir: &str,
) -> Line {
    let (Some(host), Some(target)) = (dsn_host(), body_of_ephemeris_asset(uranus_asset)) else {
        return Line {
            word,
            map: None,
            host: String::new(),
            target: String::new(),
        };
    };
    let mut map: HashMap<String, BodyEphemeris> = HashMap::new();
    for (name, asset) in [(host, earth_asset), (target.as_str(), uranus_asset)] {
        let path = format!("{eph_dir}/{netloc}/{asset}");
        let Some(bytes) = ensure_bin(&path, netloc, asset) else {
            return Line {
                word,
                map: None,
                host: String::new(),
                target: String::new(),
            };
        };
        let Some(eph) = parse_ephemeris_binary(&bytes) else {
            return Line {
                word,
                map: None,
                host: String::new(),
                target: String::new(),
            };
        };
        map.insert(name.to_string(), eph);
    }
    Line {
        word,
        map: Some(map),
        host: host.to_string(),
        target,
    }
}

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn year_bin(jd_utc: f64) -> i64 {
    (2000.0 + (jd_utc - JD_J2000) / 365.25).floor() as i64
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!(
        "Uranus carrier years — the per-year residual vector of de441 against the other two houses."
    );

    let tsv_dir =
        arg_token(&args, "--tsv-dir").unwrap_or("data/vizier.cfa.harvard.edu".to_string());
    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let spk_dir = arg_token(&args, "--spk-dir").unwrap_or("data/naif.jpl.nasa.gov".to_string());
    let spk_name = arg_token(&args, "--spk").unwrap_or("ura111.bsp".to_string());

    let Some(lsk) = embedded_lsk() else {
        eprintln!("uranus-carrier: the embedded LSK carries no naif0012 table");
        return;
    };

    let de = load_line(
        "de441",
        "ssd.jpl.nasa.gov",
        "ephemeris_earth.bin",
        "ephemeris_uranus.bin",
        &eph_dir,
    );
    let inpop = load_line(
        "inpop19a",
        "ftp.imcce.fr",
        "ephemeris_inpop_earth.bin",
        "ephemeris_inpop_uranus.bin",
        &eph_dir,
    );
    let epm = load_line(
        "epm2021",
        "ftp.iaaras.ru",
        "ephemeris_epm_earth.bin",
        "ephemeris_epm_uranus.bin",
        &eph_dir,
    );
    let lines = [de, inpop, epm];

    let center_map = {
        let path = format!("{eph_dir}/ssd.jpl.nasa.gov/ephemeris_uranus_c.bin");
        match ensure_bin(&path, "ssd.jpl.nasa.gov", "ephemeris_uranus_c.bin") {
            Some(bytes) => {
                let mut m = HashMap::new();
                if let Some(e) = parse_ephemeris_binary(&bytes) {
                    m.insert("uranus_c".to_string(), e);
                    Some(m)
                } else {
                    None
                }
            }
            None => None,
        }
    };

    let spk = match SpkFile::open(format!("{spk_dir}/{spk_name}")) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("uranus-carrier: {spk_dir}/{spk_name} opens void — {e:?}");
            return;
        }
    };

    let text = match std::fs::read_to_string(format!("{tsv_dir}/camargo_uranu_j.tsv")) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("uranus-carrier: camargo_uranu_j.tsv reads void — {e}");
            return;
        }
    };
    let obs = parse_obs(&text);

    let mut sigmas: Vec<f64> = Vec::new();
    for o in &obs {
        let s = (o.e_ra_mas * o.e_ra_mas + o.e_dec_mas * o.e_dec_mas).sqrt();
        if s.is_finite() && s > 0.0 {
            sigmas.push(s);
        }
    }
    let sigma_mean = if sigmas.is_empty() {
        None
    } else {
        Some(sigmas.iter().sum::<f64>() / sigmas.len() as f64)
    };

    let mut epochs: Vec<(f64, Vec<Option<Row>>)> = Vec::new();
    for o in &obs {
        let unix = (o.jd_utc - UNIX_JD_OFFSET) * 86400.0;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let (e_ra, e_dec) = tangent_basis(o.ra_deg, o.dec_deg);
        let mut rows: Vec<Option<Row>> = Vec::new();
        for line in &lines {
            let Some(map) = line.map.as_ref() else {
                rows.push(None);
                continue;
            };
            let use_center = line.word == "de441";
            let worldline = |t: f64| -> Option<[f64; 3]> {
                if use_center
                    && let Some(cm) = &center_map
                    && let Some(c) = body_barycenter_position("uranus_c", t, cm)
                {
                    return Some(c);
                }
                let bary = body_barycenter_position(&line.target, t, map)?;
                let rel = spk.state(799, 7, t).ok()?;
                Some([
                    bary[0] + rel[0] * 1000.0,
                    bary[1] + rel[1] * 1000.0,
                    bary[2] + rel[2] * 1000.0,
                ])
            };
            let Some(geocenter) = body_barycenter_position(&line.host, tdb, map) else {
                rows.push(None);
                continue;
            };
            let Some((s_em, _)) = light_time_worldline(geocenter, tdb, &worldline) else {
                rows.push(None);
                continue;
            };
            let Some(u_geo) = toward_unit(geocenter, s_em) else {
                rows.push(None);
                continue;
            };
            let Some((off, v_rot)) = station_offset(map, &line.host, o.jd_utc) else {
                rows.push(None);
                continue;
            };
            let sta = [
                geocenter[0] + off[0],
                geocenter[1] + off[1],
                geocenter[2] + off[2],
            ];
            let Some(u_top) = toward_unit(sta, s_em) else {
                rows.push(None);
                continue;
            };
            let u_obs = icrs_unit(o.ra_deg, o.dec_deg);
            let r = vec_sub(u_obs, u_geo);
            let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
            let s_par = vec_sub(u_top, u_geo);
            let udotv = u_geo[0] * v_rot[0] + u_geo[1] * v_rot[1] + u_geo[2] * v_rot[2];
            let s_aber = [
                (v_rot[0] - u_geo[0] * udotv) / C_LIGHT,
                (v_rot[1] - u_geo[1] * udotv) / C_LIGHT,
                (v_rot[2] - u_geo[2] * udotv) / C_LIGHT,
            ];
            rows.push(Some(Row {
                dra: dot(r, e_ra) * MAS_PER_RAD,
                ddec: dot(r, e_dec) * MAS_PER_RAD,
                par_ra: dot(s_par, e_ra) * MAS_PER_RAD,
                par_dec: dot(s_par, e_dec) * MAS_PER_RAD,
                aber_ra: dot(s_aber, e_ra) * MAS_PER_RAD,
                aber_dec: dot(s_aber, e_dec) * MAS_PER_RAD,
            }));
        }
        epochs.push((o.jd_utc, rows));
    }

    let complete: Vec<&(f64, Vec<Option<Row>>)> = epochs
        .iter()
        .filter(|(_, r)| r.iter().all(|x| x.is_some()))
        .collect();

    let names = ["de441", "inpop19a", "epm2021"];
    let mut per_line: Vec<Vec<&Row>> = vec![Vec::new(); 3];
    for (_, r) in &complete {
        for (i, x) in r.iter().enumerate() {
            if let Some(row) = x {
                per_line[i].push(row);
            }
        }
    }
    let mut fits: Vec<Option<[f64; 4]>> = Vec::new();
    for i in 0..3 {
        fits.push(fit_rows(&per_line[i]));
    }

    let mut year_rows: HashMap<i64, Vec<usize>> = HashMap::new();
    for (idx, (jd, _)) in complete.iter().enumerate() {
        year_rows.entry(year_bin(*jd)).or_default().push(idx);
    }
    let mut years: Vec<i64> = year_rows.keys().cloned().collect();
    years.sort();

    let mut report = String::new();
    report.push_str(
        "Uranus carrier years — per-year reduced residual (mas) and de441 residual vector\n\n",
    );
    match sigma_mean {
        Some(s) => report.push_str(&format!(
            "Observation uncertainty ⟨σ⟩ = {s:.1} mas ({} rows) — the noise scale against the win margins\n\n",
            sigmas.len()
        )),
        None => report.push_str("Observation uncertainty absent\n\n"),
    }
    report.push_str(
        "  year  n    de441 |d|  inpop |d|  epm |d|   winner   de441 ΔRA·cosδ  de441 ΔDec\n",
    );
    let mut carriers: Vec<(i64, f64, f64, f64, f64)> = Vec::new();
    for y in &years {
        let idxs = &year_rows[y];
        if idxs.len() < 8 {
            continue;
        }
        let mut means: [f64; 3] = [0.0; 3];
        let mut dra_sum = 0.0;
        let mut ddec_sum = 0.0;
        for &k in idxs {
            let r = &complete[k].1;
            for i in 0..3 {
                if let (Some(row), Some(x)) = (r[i].as_ref(), fits[i].as_ref()) {
                    let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra);
                    let rdec = row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec);
                    means[i] += (rra * rra + rdec * rdec).sqrt();
                }
            }
            if let (Some(row), Some(x)) = (r[0].as_ref(), fits[0].as_ref()) {
                let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra);
                let rdec = row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec);
                dra_sum += rra;
                ddec_sum += rdec;
            }
        }
        let n = idxs.len() as f64;
        let mut best = 0;
        let mut best_m = f64::INFINITY;
        for i in 0..3 {
            means[i] /= n;
            if means[i] < best_m {
                best_m = means[i];
                best = i;
            }
        }
        let d_dra = dra_sum / n;
        let d_ddec = ddec_sum / n;
        let nrow = idxs.len();
        report.push_str(&format!(
            "  {y:>4} {nrow:>4}  {:6.1}   {:6.1}   {:6.1}   {:<8} {:+8.1}       {:+8.1}\n",
            means[0], means[1], means[2], names[best], d_dra, d_ddec
        ));
        if names[best] == "de441" {
            carriers.push((*y, means[0], means[1], means[2], best_m));
        }
    }

    report.push_str("\nCarrier years (de441 carries best):\n");
    for (y, d0, d1, d2, _) in &carriers {
        report.push_str(&format!(
            "  {y}: de441 {:.1} mas vs inpop {:.1} / epm {:.1} mas\n",
            d0, d1, d2
        ));
    }
    if carriers.is_empty() {
        report.push_str("  none\n");
    }

    print!("{report}");
    if let Err(e) = std::fs::create_dir_all("zustand/reports") {
        eprintln!("uranus-carrier: report dir stays unbuilt — {e}");
    }
    let path = "zustand/reports/uranus_carrier_years.txt";
    if let Err(e) = std::fs::write(path, &report) {
        eprintln!("uranus-carrier: report write void — {e}");
    }
    println!("report: {path}");
}
