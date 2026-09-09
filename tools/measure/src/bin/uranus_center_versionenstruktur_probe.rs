use std::collections::HashMap;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use omegaflow::archivar::{
    body_barycenter_position, embedded_lsk, fetch_raw_bytes, light_time_worldline,
    parse_ephemeris_binary, BodyEphemeris, C_LIGHT,
};
use omegaflow::cdn::CDN_BASE;

const OBS_LAT_DEG: f64 = -22.534444444;
const OBS_LON_DEG: f64 = -45.5825;
const OBS_ALT_M: f64 = 1810.7;
const UNIX_JD_OFFSET: f64 = 2440587.5;
const MAS_PER_RAD: f64 = 206_264_806.247_096_36;
const BIN_TTL_S: u64 = 604800;
const JD_J2000: f64 = 2451545.0;
const WGS84_A_M: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const EARTH_ROT_RAD_S: f64 = 7.2921150e-5;
const MIN_BIN_ROWS: usize = 8;
const NULL_TRIALS: usize = 400;
const RNG_SEED: u64 = 0x9E37_79B9_7F4A_7C15;

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
}

struct Row {
    u_top: [f64; 3],
    u_top_plain: Option<[f64; 3]>,
    dra: f64,
    ddec: f64,
    par_ra: f64,
    par_dec: f64,
    aber_ra: f64,
    aber_dec: f64,
}

struct Epoch {
    jd_utc: f64,
    e_ra: [f64; 3],
    e_dec: [f64; 3],
    rows: Vec<Option<Row>>,
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

fn reduced_rms(rows: &[&Row], x: &[f64; 4]) -> Option<f64> {
    if rows.is_empty() {
        return None;
    }
    let mut sum = 0.0;
    for row in rows {
        let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra);
        let rdec = row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec);
        sum += rra * rra + rdec * rdec;
    }
    let r = (sum / rows.len() as f64).sqrt();
    if r.is_finite() {
        Some(r)
    } else {
        None
    }
}

fn residual_mag(row: &Row, x: &[f64; 4]) -> Option<f64> {
    let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra);
    let rdec = row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec);
    let m = (rra * rra + rdec * rdec).sqrt();
    if m.is_finite() {
        Some(m)
    } else {
        None
    }
}

fn station_offset(jd_utc: f64) -> ([f64; 3], [f64; 3]) {
    let lr = OBS_LAT_DEG.to_radians();
    let nr = OBS_LON_DEG.to_radians();
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let sl = lr.sin();
    let n = WGS84_A_M / (1.0 - e2 * sl * sl).sqrt();
    let xb = (n + OBS_ALT_M) * lr.cos() * nr.cos();
    let yb = (n + OBS_ALT_M) * lr.cos() * nr.sin();
    let zb = (n * (1.0 - e2) + OBS_ALT_M) * sl;
    let d = jd_utc - JD_J2000;
    let t = d / 36525.0;
    let gmst_deg =
        280.46061837 + 360.98564736629 * d + 0.000387933 * t * t - t * t * t / 38710000.0;
    let g = gmst_deg.to_radians();
    let (sg, cg) = g.sin_cos();
    let pos = [xb * cg - yb * sg, xb * sg + yb * cg, zb];
    let vel = [-pos[1] * EARTH_ROT_RAD_S, pos[0] * EARTH_ROT_RAD_S, 0.0];
    (pos, vel)
}

fn ensure_bin(path: &str, netloc: &str, asset: &str, ttl: u64) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("{}/{}/{}", CDN_BASE, netloc, asset);
    let bytes = fetch_raw_bytes(&url, ttl)?;
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

fn unit3(v: [f64; 3]) -> Option<[f64; 3]> {
    let n = vec_len(v)?;
    Some([v[0] / n, v[1] / n, v[2] / n])
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

fn angle_mas(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    if !dot.is_finite() {
        return None;
    }
    let c = dot.clamp(-1.0, 1.0);
    let ang = c.acos();
    if ang.is_finite() {
        Some(ang * MAS_PER_RAD)
    } else {
        None
    }
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
    let mut map: HashMap<String, BodyEphemeris> = HashMap::new();
    for (name, asset) in [("earth", earth_asset), ("uranus", uranus_asset)] {
        let path = format!("{eph_dir}/{netloc}/{asset}");
        let Some(bytes) = ensure_bin(&path, netloc, asset, BIN_TTL_S) else {
            println!("uranus-center-versions {word}: {path} bin void — absent on disk and the CDN fetch returned non-200");
            return Line { word, map: None };
        };
        let Some(eph) = parse_ephemeris_binary(&bytes) else {
            println!(
                "uranus-center-versions {word}: {path} reads but does not parse to a BodyEphemeris"
            );
            return Line { word, map: None };
        };
        map.insert(name.to_string(), eph);
    }
    Line {
        word,
        map: Some(map),
    }
}

fn load_center(netloc: &str, asset: &str, eph_dir: &str) -> Option<HashMap<String, BodyEphemeris>> {
    let path = format!("{eph_dir}/{netloc}/{asset}");
    let bytes = ensure_bin(&path, netloc, asset, BIN_TTL_S)?;
    let eph = parse_ephemeris_binary(&bytes)?;
    let mut map = HashMap::new();
    map.insert("uranus_c".to_string(), eph);
    Some(map)
}

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn year_bin(jd_utc: f64) -> i64 {
    (2000.0 + (jd_utc - JD_J2000) / 365.25).floor() as i64
}

fn rng_next(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *state = x;
    x.wrapping_mul(0x2545_F491_4F6C_DD1D)
}

fn shuffle<T>(v: &mut [T], state: &mut u64) {
    let n = v.len();
    for i in (1..n).rev() {
        let j = (rng_next(state) % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
}

struct Inject {
    ra_mas: f64,
    dec_mas: f64,
    jd0: f64,
    jd1: f64,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Uranus center version-structure — the per-point version difference between the three ephemeris lines at the planet center.");

    let tsv_dir =
        arg_token(&args, "--tsv-dir").unwrap_or("data/vizier.cfa.harvard.edu".to_string());
    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let spk_dir = arg_token(&args, "--spk-dir").unwrap_or("data/naif.jpl.nasa.gov".to_string());
    let spk_name = arg_token(&args, "--spk").unwrap_or("ura111.bsp".to_string());
    let report_dir = arg_token(&args, "--report-dir").unwrap_or("state/reports".to_string());

    let inject: Option<Inject> = {
        let pos = args.iter().position(|a| a == "--calibrate-inject");
        let values: Vec<f64> = match pos {
            Some(p) => {
                let mut v = Vec::new();
                for i in (p + 1)..(p + 5) {
                    match args.get(i).and_then(|s| s.parse::<f64>().ok()) {
                        Some(x) if x.is_finite() => v.push(x),
                        _ => break,
                    }
                }
                v
            }
            None => Vec::new(),
        };
        if values.len() == 4 && values[0].abs() < 1000.0 && values[1].abs() < 1000.0 {
            Some(Inject {
                ra_mas: values[0],
                dec_mas: values[1],
                jd0: values[2],
                jd1: values[3],
            })
        } else if !values.is_empty() {
            println!(
                "uranus-center-versions: --calibrate-inject wants ra_mas dec_mas jd0 jd1 (finite) — received {} values, injection stays off",
                values.len()
            );
            None
        } else {
            None
        }
    };

    let Some(lsk) = embedded_lsk() else {
        eprintln!("uranus-center-versions: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
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

    let center_map = match load_center("ssd.jpl.nasa.gov", "ephemeris_uranus_c.bin", &eph_dir) {
        Some(m) => Some(m),
        None => {
            eprintln!("uranus-center-versions: ephemeris_uranus_c.bin absent — the DE441 planet line composes from the barycenter + (799−7) offset");
            None
        }
    };

    let spk_path = format!("{spk_dir}/{spk_name}");
    let spk = match SpkFile::open(&spk_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("uranus-center-versions: {spk_path} opens void — {e:?}");
            return;
        }
    };

    let tsv_path = format!("{tsv_dir}/camargo_uranu_j.tsv");
    let text = match std::fs::read_to_string(&tsv_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("uranus-center-versions: {tsv_path} reads void — {e}");
            return;
        }
    };
    let obs = parse_obs(&text);
    println!(
        "uranus-center-versions: uranu_j (the planet) — {} rows",
        obs.len()
    );

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

    let mut epochs: Vec<Epoch> = Vec::new();
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
                if use_center {
                    if let Some(cm) = &center_map {
                        if let Some(c) = body_barycenter_position("uranus_c", t, cm) {
                            return Some(c);
                        }
                    }
                }
                let bary = body_barycenter_position("uranus", t, map)?;
                let rel = spk.state(799, 7, t).ok()?;
                Some([
                    bary[0] + rel[0] * 1000.0,
                    bary[1] + rel[1] * 1000.0,
                    bary[2] + rel[2] * 1000.0,
                ])
            };
            let Some(geocenter) = body_barycenter_position("earth", tdb, map) else {
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
            let (off, v_rot) = station_offset(o.jd_utc);
            let sta = [
                geocenter[0] + off[0],
                geocenter[1] + off[1],
                geocenter[2] + off[2],
            ];
            let Some(u_top) = toward_unit(sta, s_em) else {
                rows.push(None);
                continue;
            };
            let (u_top, u_top_plain) = match &inject {
                Some(ij) if line.word == "de441" && o.jd_utc >= ij.jd0 && o.jd_utc <= ij.jd1 => {
                    let sx = ij.ra_mas / MAS_PER_RAD;
                    let sy = ij.dec_mas / MAS_PER_RAD;
                    let offv = [
                        e_ra[0] * sx + e_dec[0] * sy,
                        e_ra[1] * sx + e_dec[1] * sy,
                        e_ra[2] * sx + e_dec[2] * sy,
                    ];
                    match unit3([u_top[0] + offv[0], u_top[1] + offv[1], u_top[2] + offv[2]]) {
                        Some(nt) => (nt, Some(u_top)),
                        None => {
                            rows.push(None);
                            continue;
                        }
                    }
                }
                _ => (u_top, None),
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
                u_top,
                u_top_plain,
                dra: dot(r, e_ra) * MAS_PER_RAD,
                ddec: dot(r, e_dec) * MAS_PER_RAD,
                par_ra: dot(s_par, e_ra) * MAS_PER_RAD,
                par_dec: dot(s_par, e_dec) * MAS_PER_RAD,
                aber_ra: dot(s_aber, e_ra) * MAS_PER_RAD,
                aber_dec: dot(s_aber, e_dec) * MAS_PER_RAD,
            }));
        }
        epochs.push(Epoch {
            jd_utc: o.jd_utc,
            e_ra,
            e_dec,
            rows,
        });
    }

    let complete: Vec<&Epoch> = epochs
        .iter()
        .filter(|e| e.rows.iter().all(|r| r.is_some()))
        .collect();
    println!(
        "uranus-center-versions: {} complete epochs (all three lines carried a center direction)",
        complete.len()
    );

    let mut report = String::new();
    report.push_str("Uranus center version-structure — per-point version difference\n\n");

    match sigma_mean {
        Some(s) => report.push_str(&format!(
            "Observation uncertainty ⟨σ⟩ = {s:.1} mas ({} rows) — the noise the structure must exceed\n\n",
            sigmas.len()
        )),
        None => report.push_str("Observation uncertainty absent — no finite positive error rows\n\n"),
    }

    let names = ["de441", "inpop19a", "epm2021"];
    let mut per_line: Vec<Vec<&Row>> = vec![Vec::new(); 3];
    for e in &complete {
        for (i, r) in e.rows.iter().enumerate() {
            if let Some(row) = r {
                per_line[i].push(row);
            }
        }
    }

    report.push_str(
        "Per-line center-anchored fit (c0, c_par, c_aber — the reduced residual floor):\n",
    );
    let mut fits: Vec<Option<[f64; 4]>> = Vec::new();
    for i in 0..3 {
        match fit_rows(&per_line[i]) {
            Some(x) => {
                let rms = match reduced_rms(&per_line[i], &x) {
                    Some(v) => format!("{v:.1}"),
                    None => "na".to_string(),
                };
                report.push_str(&format!(
                    "  {:<9} : c_par {:.2}, c_aber {:.2}, c0 {:+.1}/{:+.1} mas, reduced RMS {} mas ({} epochs)\n",
                    names[i], x[2], x[3], x[0], x[1], rms, per_line[i].len()
                ));
                fits.push(Some(x));
            }
            None => {
                report.push_str(&format!(
                    "  {:<9} : fit void — fewer than 8 rows or a singular normal matrix ({} epochs)\n",
                    names[i], per_line[i].len()
                ));
                fits.push(None);
            }
        }
    }

    let mut pair_vec: [[Vec<(f64, f64)>; 3]; 3] =
        std::array::from_fn(|_| [Vec::new(), Vec::new(), Vec::new()]);
    let mut pair_max: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut pair_max_jd: [[f64; 3]; 3] = [[0.0; 3]; 3];
    for e in &complete {
        for a in 0..3 {
            for b in (a + 1)..3 {
                let Some(ra) = e.rows[a].as_ref() else {
                    continue;
                };
                let Some(rb) = e.rows[b].as_ref() else {
                    continue;
                };
                let d = vec_sub(ra.u_top, rb.u_top);
                let dot = |x: [f64; 3], y: [f64; 3]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
                let dra = dot(d, e.e_ra) * MAS_PER_RAD;
                let ddec = dot(d, e.e_dec) * MAS_PER_RAD;
                let mag = (dra * dra + ddec * ddec).sqrt();
                if mag > pair_max[a][b] {
                    pair_max[a][b] = mag;
                    pair_max_jd[a][b] = e.jd_utc;
                }
                pair_vec[a][b].push((dra, ddec));
            }
        }
    }
    report.push_str(
        "\nPer-point center version difference (line a − line b, topocentric tangent plane, mas):\n",
    );
    for a in 0..3 {
        for b in (a + 1)..3 {
            let n = pair_vec[a][b].len();
            if n == 0 {
                report.push_str(&format!(
                    "  {} − {} : absent (no complete epochs)\n",
                    names[a], names[b]
                ));
                continue;
            }
            let (sra, sdec): (f64, f64) = pair_vec[a][b]
                .iter()
                .fold((0.0, 0.0), |(ax, ay), &(x, y)| (ax + x, ay + y));
            let mean_ra = sra / n as f64;
            let mean_dec = sdec / n as f64;
            let mean_mag = pair_vec[a][b]
                .iter()
                .map(|&(x, y)| (x * x + y * y).sqrt())
                .sum::<f64>()
                / n as f64;
            report.push_str(&format!(
                "  {} − {} : ΔRA·cosδ {:+.1} mas, ΔDec {:+.1} mas, |Δ| mean {:.1} mas, max {:.1} mas at JD {:.2} ({} epochs)\n",
                names[a], names[b], mean_ra, mean_dec, mean_mag, pair_max[a][b], pair_max_jd[a][b], n
            ));
        }
    }

    report.push_str("\nYear-bin structure (per-bin mean |Δ| and the best-carrying line):\n");
    let mut bins: Vec<(i64, Vec<usize>)> = Vec::new();
    for (idx, e) in complete.iter().enumerate() {
        let y = year_bin(e.jd_utc);
        match bins.iter_mut().find(|(by, _)| *by == y) {
            Some(b) => b.1.push(idx),
            None => bins.push((y, vec![idx])),
        }
    }
    bins.sort_by(|a, b| a.0.cmp(&b.0));

    let mut resid: [Vec<Option<f64>>; 3] = std::array::from_fn(|_| Vec::new());
    for e in &complete {
        for i in 0..3 {
            let v = match (e.rows[i].as_ref(), fits[i].as_ref()) {
                (Some(row), Some(x)) => residual_mag(row, x),
                _ => None,
            };
            resid[i].push(v);
        }
    }

    let mut observed_wins: [usize; 3] = [0; 3];
    let mut year_winner: Vec<(i64, String)> = Vec::new();
    for (y, idxs) in &bins {
        if idxs.len() < MIN_BIN_ROWS {
            continue;
        }
        let mut means: [f64; 3] = [0.0; 3];
        let mut counts: [usize; 3] = [0; 3];
        for &k in idxs {
            for i in 0..3 {
                if let Some(v) = resid[i][k] {
                    means[i] += v;
                    counts[i] += 1;
                }
            }
        }
        let mut best: Option<usize> = None;
        let mut best_m = f64::INFINITY;
        for i in 0..3 {
            if counts[i] == 0 {
                continue;
            }
            means[i] /= counts[i] as f64;
            if means[i] < best_m {
                best_m = means[i];
                best = Some(i);
            }
        }
        if let Some(i) = best {
            observed_wins[i] += 1;
            year_winner.push((*y, names[i].to_string()));
        }
    }

    for (y, idxs) in &bins {
        if idxs.len() < MIN_BIN_ROWS {
            continue;
        }
        let mut line = format!("  {y}: {:<4} epochs |", idxs.len());
        for a in 0..3 {
            for b in (a + 1)..3 {
                let mut sum = 0.0;
                let mut cnt = 0;
                for &k in idxs {
                    if let Some(&(x, yv)) = pair_vec[a][b].get(k) {
                        sum += (x * x + yv * yv).sqrt();
                        cnt += 1;
                    }
                }
                if cnt > 0 {
                    line.push_str(&format!(
                        " {}−{} {:5.1} |",
                        names[a],
                        names[b],
                        sum / cnt as f64
                    ));
                }
            }
        }
        report.push_str(&format!("{}\n", line));
    }

    report.push_str("\nBest-carrying line per year bin (reduced residual mean):\n");
    for (y, w) in &year_winner {
        report.push_str(&format!("  {y}: {w}\n"));
    }
    report.push_str(&format!(
        "Observed bin wins: de441 {}, inpop19a {}, epm2021 {}\n",
        observed_wins[0], observed_wins[1], observed_wins[2]
    ));

    report.push_str(
        "\nNull (epoch-order shuffle): is the temporal best-carrier structure above chance?\n",
    );
    let sample_idx: Vec<usize> = (0..complete.len()).collect();
    let mut state = RNG_SEED;
    let mut null_mean: [f64; 3] = [0.0; 3];
    let mut null_sumsq: [f64; 3] = [0.0; 3];
    let n_trials = NULL_TRIALS as f64;
    for _ in 0..NULL_TRIALS {
        let mut idx = sample_idx.clone();
        shuffle(&mut idx, &mut state);
        let mut wins: [usize; 3] = [0; 3];
        for (_, idxs) in &bins {
            if idxs.len() < MIN_BIN_ROWS {
                continue;
            }
            let mut means: [f64; 3] = [0.0; 3];
            let mut counts: [usize; 3] = [0; 3];
            for &k in idxs {
                let j = idx[k];
                for i in 0..3 {
                    if let Some(v) = resid[i][j] {
                        means[i] += v;
                        counts[i] += 1;
                    }
                }
            }
            let mut best: Option<usize> = None;
            let mut best_m = f64::INFINITY;
            for i in 0..3 {
                if counts[i] == 0 {
                    continue;
                }
                means[i] /= counts[i] as f64;
                if means[i] < best_m {
                    best_m = means[i];
                    best = Some(i);
                }
            }
            if let Some(i) = best {
                wins[i] += 1;
            }
        }
        for i in 0..3 {
            let w = wins[i] as f64;
            null_mean[i] += w;
            null_sumsq[i] += w * w;
        }
    }
    for i in 0..3 {
        let m = null_mean[i] / n_trials;
        let var = null_sumsq[i] / n_trials - m * m;
        let sd = if var > 0.0 { var.sqrt() } else { 0.0 };
        let obs = observed_wins[i] as f64;
        let z = if sd > 0.0 { (obs - m) / sd } else { 0.0 };
        report.push_str(&format!(
            "  {:<9} : observed {:.0} bins vs null {:.1} ± {:.1} (z = {:+.1})\n",
            names[i], obs, m, sd, z
        ));
    }

    if let Some(ij) = &inject {
        report.push_str(&format!(
            "\nCalibration injection: {:.1} mas RA, {:.1} mas Dec into de441 over JD {:.2}..{:.2}:\n",
            ij.ra_mas, ij.dec_mas, ij.jd0, ij.jd1
        ));
        let mut rec_ra = 0.0;
        let mut rec_dec = 0.0;
        let mut rec_mag = 0.0;
        let mut rec_n = 0usize;
        for e in &complete {
            let Some(row) = e.rows[0].as_ref() else {
                continue;
            };
            let Some(plain) = row.u_top_plain else {
                continue;
            };
            let d = vec_sub(row.u_top, plain);
            let dot = |x: [f64; 3], y: [f64; 3]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
            rec_ra += dot(d, e.e_ra) * MAS_PER_RAD;
            rec_dec += dot(d, e.e_dec) * MAS_PER_RAD;
            if let Some(m) = angle_mas(plain, row.u_top) {
                rec_mag += m;
            }
            rec_n += 1;
        }
        if rec_n == 0 {
            report.push_str("  recovered nothing — the injection window carried no de441 epochs\n");
        } else {
            report.push_str(&format!(
                "  recovered vector ΔRA·cosδ {:+.2} mas, ΔDec {:+.2} mas, magnitude {:.2} mas over {} epochs — injected {:+.1}/{:+.1} mas, magnitude {:.1} mas\n",
                rec_ra / rec_n as f64,
                rec_dec / rec_n as f64,
                rec_mag / rec_n as f64,
                rec_n,
                ij.ra_mas,
                ij.dec_mas,
                (ij.ra_mas * ij.ra_mas + ij.dec_mas * ij.dec_mas).sqrt()
            ));
        }
    }

    print!("{report}");
    if let Err(e) = std::fs::create_dir_all(&report_dir) {
        eprintln!("uranus-center-versions: report dir {report_dir} stays unbuilt — {e}");
    }
    let report_path = format!("{report_dir}/uranus_center_versionenstruktur.txt");
    let series_path = format!("{report_dir}/uranus_center_versionenstruktur_series.tsv");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("uranus-center-versions: report write void — {e}");
    }
    let mut series = String::new();
    series.push_str("jd_utc\tyear\tdra_de441_inpop_mas\tddec_de441_inpop_mas\tdra_de441_epm_mas\tddec_de441_epm_mas\tdra_inpop_epm_mas\tddec_inpop_epm_mas\tresid_de441_mas\tresid_inpop_mas\tresid_epm_mas\n");
    for (idx, e) in complete.iter().enumerate() {
        let (p01x, p01y) = pair_vec[0][1][idx];
        let (p02x, p02y) = pair_vec[0][2][idx];
        let (p12x, p12y) = pair_vec[1][2][idx];
        let r0 = match resid[0][idx] {
            Some(v) => format!("{v:.3}"),
            None => "na".to_string(),
        };
        let r1 = match resid[1][idx] {
            Some(v) => format!("{v:.3}"),
            None => "na".to_string(),
        };
        let r2 = match resid[2][idx] {
            Some(v) => format!("{v:.3}"),
            None => "na".to_string(),
        };
        series.push_str(&format!(
            "{:.8}\t{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{}\t{}\t{}\n",
            e.jd_utc,
            year_bin(e.jd_utc),
            p01x,
            p01y,
            p02x,
            p02y,
            p12x,
            p12y,
            r0,
            r1,
            r2
        ));
    }
    if let Err(e) = std::fs::write(&series_path, &series) {
        eprintln!("uranus-center-versions: series write void — {e}");
    }
    println!("series: {series_path}");
    println!("report: {report_path}");
}
