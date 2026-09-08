use std::collections::HashMap;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use omegaflow::archivar::{
    body_barycenter_position, body_fixed_to_icrs, body_fixed_to_icrs_smooth, embedded_lsk,
    fetch_raw_bytes, parse_ephemeris_binary, BodyEphemeris, C_LIGHT,
};
use omegaflow::cdn::CDN_BASE;

const OBS_LAT_DEG: f64 = -22.534444444;
const OBS_LON_DEG: f64 = -45.5825;
const OBS_ALT_M: f64 = 1810.7;
const UNIX_JD_OFFSET: f64 = 2440587.5;
const MAS_PER_RAD: f64 = 206_264_806.247_096_36;
const BIN_TTL_S: u64 = 604800;
const VEL_DT_S: f64 = 10.0;

const SATELLITES: [(&str, i32); 5] = [
    ("ariel_j", 701),
    ("umbri_j", 702),
    ("titan_j", 703),
    ("obero_j", 704),
    ("miran_j", 705),
];

#[derive(Clone)]
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

struct SatSpk {
    id: i32,
}

#[derive(Clone)]
struct EpochRow {
    jd_utc: f64,
    dra: f64,
    ddec: f64,
    par_ra: f64,
    par_dec: f64,
    aber_ra: f64,
    aber_dec: f64,
}

struct Group {
    name: String,
    rows: Vec<EpochRow>,
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

fn inv4(a_in: [[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
    let mut a = a_in;
    let mut inv = [[0.0; 4]; 4];
    for i in 0..4 {
        inv[i][i] = 1.0;
    }
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
            inv.swap(piv, col);
        }
        let d = a[col][col];
        for c in 0..4 {
            a[col][c] /= d;
            inv[col][c] /= d;
        }
        for r in 0..4 {
            if r == col {
                continue;
            }
            let f = a[r][col];
            for c in 0..4 {
                a[r][c] -= f * a[col][c];
                inv[r][c] -= f * inv[col][c];
            }
        }
    }
    Some(inv)
}

fn fit_group(group: &Group) -> Option<([f64; 4], [f64; 4])> {
    let mut fit = Fit::new();
    for row in &group.rows {
        fit.row([1.0, 0.0, row.par_ra, row.aber_ra], row.dra);
        fit.row([0.0, 1.0, row.par_dec, row.aber_dec], row.ddec);
    }
    if fit.n < 8 {
        return None;
    }
    let x = solve4(fit.ata, fit.atb)?;
    let inv = inv4(fit.ata)?;
    let mut rss = 0.0;
    for row in &group.rows {
        let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra);
        let rdec = row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec);
        rss += rra * rra + rdec * rdec;
    }
    let dof = (fit.n - 4) as f64;
    if !(dof.is_finite() && dof > 0.0) {
        return None;
    }
    let var = rss / dof;
    let mut sigma = [0.0; 4];
    for i in 0..4 {
        let v = inv[i][i] * var;
        sigma[i] = if v.is_finite() && v > 0.0 {
            v.sqrt()
        } else {
            f64::NAN
        };
    }
    Some((x, sigma))
}

fn rms_of(group: &Group) -> Option<f64> {
    if group.rows.is_empty() {
        return None;
    }
    let mut sum = 0.0;
    for row in &group.rows {
        sum += row.dra * row.dra + row.ddec * row.ddec;
    }
    let r = (sum / group.rows.len() as f64).sqrt();
    if r.is_finite() {
        Some(r)
    } else {
        None
    }
}

fn mean_vec(group: &Group) -> Option<(f64, f64)> {
    if group.rows.is_empty() {
        return None;
    }
    let mut dra = 0.0;
    let mut ddec = 0.0;
    for row in &group.rows {
        dra += row.dra;
        ddec += row.ddec;
    }
    let n = group.rows.len() as f64;
    Some((dra / n, ddec / n))
}

fn rms_reduced(group: &Group, x: &[f64; 4]) -> Option<f64> {
    if group.rows.is_empty() {
        return None;
    }
    let mut sum = 0.0;
    for row in &group.rows {
        let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra);
        let rdec = row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec);
        sum += rra * rra + rdec * rdec;
    }
    let r = (sum / group.rows.len() as f64).sqrt();
    if r.is_finite() {
        Some(r)
    } else {
        None
    }
}

fn swing_of(rows: &[EpochRow], f: &dyn Fn(&EpochRow) -> (f64, f64)) -> Option<(f64, f64)> {
    if rows.is_empty() {
        return None;
    }
    let mut sum = 0.0;
    let mut max = 0.0;
    for row in rows {
        let (a, b) = f(row);
        let m = (a * a + b * b).sqrt();
        if !m.is_finite() {
            return None;
        }
        sum += m;
        if m > max {
            max = m;
        }
    }
    let mean = sum / rows.len() as f64;
    Some((mean, max))
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

fn roemer_fold_state(
    station: [f64; 3],
    tdb: f64,
    worldline: &dyn Fn(f64) -> Option<[f64; 3]>,
) -> Option<[f64; 3]> {
    let mut emitted = tdb;
    for _ in 0..12 {
        let apparent = worldline(emitted)?;
        let d = vec_len(vec_sub(apparent, station))?;
        let next = tdb - d / C_LIGHT;
        if !next.is_finite() {
            return None;
        }
        if (next - emitted).abs() < 1e-9 {
            emitted = next;
            break;
        }
        emitted = next;
    }
    worldline(emitted)
}

fn sat_worldline(
    sat: &SatSpk,
    spk: &SpkFile,
    planet_map: &HashMap<String, BodyEphemeris>,
    t: f64,
) -> Option<[f64; 3]> {
    let bary = body_barycenter_position("uranus", t, planet_map)?;
    let rel = spk.state(sat.id, 7, t).ok()?;
    Some([
        bary[0] + rel[0] * 1000.0,
        bary[1] + rel[1] * 1000.0,
        bary[2] + rel[2] * 1000.0,
    ])
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
            println!("uranus-diurnal {word}: {path} bin void — absent on disk and the CDN fetch returned non-200");
            return Line { word, map: None };
        };
        let Some(eph) = parse_ephemeris_binary(&bytes) else {
            println!("uranus-diurnal {word}: {path} reads but does not parse to a BodyEphemeris");
            return Line { word, map: None };
        };
        map.insert(name.to_string(), eph);
    }
    Line {
        word,
        map: Some(map),
    }
}

fn load_sat(spk: &SpkFile, name: &str, id: i32) -> Option<SatSpk> {
    let covered = spk.segments().iter().any(|seg| seg.target == id);
    if covered {
        Some(SatSpk { id })
    } else {
        println!("uranus-diurnal {name} ({id}): no segment covers it");
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

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn carries_word(c: f64, sigma: f64) -> &'static str {
    if !c.is_finite() || !sigma.is_finite() {
        return "unmeasurable (absent rows or a singular normal matrix)";
    }
    if (c - 1.0).abs() <= 3.0 * sigma || (c - 1.0).abs() < 1e-9 {
        "carries it (c ≈ +1)"
    } else if c.abs() <= 3.0 * sigma || c.abs() < 1e-9 {
        "carries none of it (c ≈ 0)"
    } else {
        "carries a partial share (the number stands)"
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Uranus diurnal decomposition — parallax vs diurnal aberration inside the absolute satellite residual.");

    let tsv_dir =
        arg_token(&args, "--tsv-dir").unwrap_or("data/vizier.cfa.harvard.edu".to_string());
    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let spk_dir = arg_token(&args, "--spk-dir").unwrap_or("data/naif.jpl.nasa.gov".to_string());
    let spk_name = arg_token(&args, "--spk").unwrap_or("ura111.bsp".to_string());
    let report_dir = arg_token(&args, "--report-dir").unwrap_or("state/reports".to_string());
    let cal_topo = args.iter().any(|a| a == "--calibrate-topo");
    let cal_aber = args.iter().any(|a| a == "--calibrate-aber");
    let calibrating = cal_topo || cal_aber;

    let Some(lsk) = embedded_lsk() else {
        eprintln!("uranus-diurnal: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
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

    let spk_path = format!("{spk_dir}/{spk_name}");
    let spk = match SpkFile::open(&spk_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("uranus-diurnal: {spk_path} opens void — {e:?}");
            return;
        }
    };

    let mut sats: Vec<(String, SatSpk, Vec<Obs>)> = Vec::new();
    for (name, id) in SATELLITES {
        let Some(sat) = load_sat(&spk, name, id) else {
            continue;
        };
        let tsv_path = format!("{tsv_dir}/camargo_{name}.tsv");
        let text = match std::fs::read_to_string(&tsv_path) {
            Ok(t) => t,
            Err(e) => {
                println!("uranus-diurnal {name}: {tsv_path} reads void — {e}");
                continue;
            }
        };
        let obs = parse_obs(&text);
        println!("uranus-diurnal {name}: {} rows", obs.len());
        sats.push((name.to_string(), sat, obs));
    }
    if sats.is_empty() {
        eprintln!(
            "uranus-diurnal: no satellite carried both an SPK and observation rows (0 honored)"
        );
        return;
    }

    let mut all_sigma: Vec<f64> = Vec::new();
    for (_, _, obs) in &sats {
        for o in obs {
            all_sigma.push((o.e_ra_mas * o.e_ra_mas + o.e_dec_mas * o.e_dec_mas).sqrt());
        }
    }
    let sigma_mean = all_sigma.iter().sum::<f64>() / all_sigma.len() as f64;
    let x_mas = 2.0 * sigma_mean;

    let de_map = lines
        .iter()
        .find(|l| l.word == "de441")
        .and_then(|l| l.map.as_ref());

    let mut groups: Vec<Group> = Vec::new();
    let mut census: Vec<(String, usize, usize, usize, usize)> = Vec::new();
    for line in lines.iter() {
        let Some(map) = line.map.as_ref() else {
            continue;
        };
        let mut line_rows: Vec<EpochRow> = Vec::new();
        let mut obs_from_own = 0usize;
        let mut obs_from_de = 0usize;
        let mut skip_geo = 0usize;
        let mut skip_fold = 0usize;
        for (name, sat, obs) in &sats {
            let mut sat_rows: Vec<EpochRow> = Vec::new();
            for o in obs {
                let unix = (o.jd_utc - UNIX_JD_OFFSET) * 86400.0;
                let Some(tdb) = lsk.unix_to_tdb(unix) else {
                    continue;
                };
                let Some(geocenter) = body_barycenter_position("earth", tdb, map) else {
                    skip_geo += 1;
                    continue;
                };
                let Some(s_em) =
                    roemer_fold_state(geocenter, tdb, &|t| sat_worldline(sat, &spk, map, t))
                else {
                    skip_fold += 1;
                    continue;
                };
                let Some(u_geo) = toward_unit(geocenter, s_em) else {
                    skip_fold += 1;
                    continue;
                };
                let (sta, from_de) = match body_fixed_to_icrs(
                    "earth",
                    OBS_LAT_DEG,
                    OBS_LON_DEG,
                    OBS_ALT_M,
                    tdb,
                    map,
                ) {
                    Some(s) => (s, false),
                    None => match de_map.and_then(|m| {
                        body_fixed_to_icrs("earth", OBS_LAT_DEG, OBS_LON_DEG, OBS_ALT_M, tdb, m)
                    }) {
                        Some(s) => (s, true),
                        None => {
                            skip_geo += 1;
                            continue;
                        }
                    },
                };
                if from_de {
                    obs_from_de += 1;
                } else {
                    obs_from_own += 1;
                }
                let Some(u_top) = toward_unit(sta, s_em) else {
                    skip_geo += 1;
                    continue;
                };
                let s_par = vec_sub(u_top, u_geo);
                let (sta_p, sta_m) = if from_de {
                    match (
                        de_map.and_then(|m| {
                            body_fixed_to_icrs_smooth(
                                "earth",
                                OBS_LAT_DEG,
                                OBS_LON_DEG,
                                OBS_ALT_M,
                                tdb + VEL_DT_S,
                                m,
                            )
                        }),
                        de_map.and_then(|m| {
                            body_fixed_to_icrs_smooth(
                                "earth",
                                OBS_LAT_DEG,
                                OBS_LON_DEG,
                                OBS_ALT_M,
                                tdb - VEL_DT_S,
                                m,
                            )
                        }),
                    ) {
                        (Some(p), Some(n)) => (p, n),
                        _ => {
                            skip_geo += 1;
                            continue;
                        }
                    }
                } else {
                    match (
                        body_fixed_to_icrs_smooth(
                            "earth",
                            OBS_LAT_DEG,
                            OBS_LON_DEG,
                            OBS_ALT_M,
                            tdb + VEL_DT_S,
                            map,
                        ),
                        body_fixed_to_icrs_smooth(
                            "earth",
                            OBS_LAT_DEG,
                            OBS_LON_DEG,
                            OBS_ALT_M,
                            tdb - VEL_DT_S,
                            map,
                        ),
                    ) {
                        (Some(p), Some(n)) => (p, n),
                        _ => {
                            skip_geo += 1;
                            continue;
                        }
                    }
                };
                let Some(geo_p) = body_barycenter_position("earth", tdb + VEL_DT_S, map) else {
                    continue;
                };
                let Some(geo_m) = body_barycenter_position("earth", tdb - VEL_DT_S, map) else {
                    continue;
                };
                let dt2 = 2.0 * VEL_DT_S;
                let v_rot = [
                    (sta_p[0] - sta_m[0]) / dt2 - (geo_p[0] - geo_m[0]) / dt2,
                    (sta_p[1] - sta_m[1]) / dt2 - (geo_p[1] - geo_m[1]) / dt2,
                    (sta_p[2] - sta_m[2]) / dt2 - (geo_p[2] - geo_m[2]) / dt2,
                ];
                let udotv = u_geo[0] * v_rot[0] + u_geo[1] * v_rot[1] + u_geo[2] * v_rot[2];
                let s_aber = [
                    (v_rot[0] - u_geo[0] * udotv) / C_LIGHT,
                    (v_rot[1] - u_geo[1] * udotv) / C_LIGHT,
                    (v_rot[2] - u_geo[2] * udotv) / C_LIGHT,
                ];
                let u_obs = if calibrating {
                    let mut synth = u_geo;
                    if cal_topo {
                        synth = [
                            synth[0] + s_par[0],
                            synth[1] + s_par[1],
                            synth[2] + s_par[2],
                        ];
                    }
                    if cal_aber {
                        synth = [
                            synth[0] + s_aber[0],
                            synth[1] + s_aber[1],
                            synth[2] + s_aber[2],
                        ];
                    }
                    let Some(u_syn) = unit3(synth) else {
                        skip_fold += 1;
                        continue;
                    };
                    u_syn
                } else {
                    icrs_unit(o.ra_deg, o.dec_deg)
                };
                let (e_ra, e_dec) = tangent_basis(o.ra_deg, o.dec_deg);
                let r = vec_sub(u_obs, u_geo);
                let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
                let row = EpochRow {
                    jd_utc: o.jd_utc,
                    dra: dot(r, e_ra) * MAS_PER_RAD,
                    ddec: dot(r, e_dec) * MAS_PER_RAD,
                    par_ra: dot(s_par, e_ra) * MAS_PER_RAD,
                    par_dec: dot(s_par, e_dec) * MAS_PER_RAD,
                    aber_ra: dot(s_aber, e_ra) * MAS_PER_RAD,
                    aber_dec: dot(s_aber, e_dec) * MAS_PER_RAD,
                };
                sat_rows.push(row);
            }
            groups.push(Group {
                name: format!("{}-{}", line.word, name),
                rows: sat_rows,
            });
            if let Some(g) = groups.last() {
                line_rows.extend(g.rows.clone());
            }
        }
        groups.push(Group {
            name: line.word.to_string(),
            rows: line_rows,
        });
        census.push((
            line.word.to_string(),
            obs_from_own,
            obs_from_de,
            skip_geo,
            skip_fold,
        ));
    }

    let mut report = String::new();
    report.push_str("Uranus diurnal decomposition — parallax vs diurnal aberration\n\n");
    report.push_str(&format!(
        "Observatory: Pico dos Dias (MPC 874), λ = {OBS_LON_DEG}°, φ = {OBS_LAT_DEG}°, h = {OBS_ALT_M} m\n"
    ));
    report.push_str(&format!(
        "Tables: five Camargo+ 2015 satellite tables (A&A 582 A8), moon orbits {spk_name}, {sigma_mean:.1} mas mean per-row sigma, X = 2·⟨σ⟩ = {x_mas:.1} mas\n"
    ));
    report.push_str("Model: geocentric astrometric (ICRS, light-time, no aberration/deflection) — the root-probe convention\n");
    report.push_str("Signatures: parallax = topocentric − geocentric direction (computed with body_fixed_to_icrs at the MPC-874 geodetic); diurnal aberration = observer rotational velocity projected on the sky\n");
    report.push_str("Fit per line and satellite: ΔRA·cosδ, ΔDec = c0 + c_par·parallax + c_aber·diurnal_aberration\n\n");

    if calibrating {
        report.push_str(&format!(
            "Calibration mode: the observed direction is synthesized as the geocentric model + injected parallax ({cal_topo}) + injected diurnal aberration ({cal_aber}) — the fit must recover the injected coefficients\n\n"
        ));
    }

    report.push_str("Row census per line (the observer side is one physical observer — the de441 Earth model serves the station when the line's own Earth bin carries no body-fixed orientation):\n");
    for (word, own, de, sgeo, sfold) in &census {
        report.push_str(&format!(
            "  {word:9} : {own} rows station from own Earth bin, {de} from de441, {sgeo} skipped (geocenter/station void), {sfold} skipped (light-time fold void)\n"
        ));
    }

    report.push_str("\nSignature swing over the series (mas):\n");
    for line in lines.iter() {
        let Some(g) = groups.iter().find(|g| g.name == line.word) else {
            continue;
        };
        match (
            swing_of(&g.rows, &|r| (r.par_ra, r.par_dec)),
            swing_of(&g.rows, &|r| (r.aber_ra, r.aber_dec)),
        ) {
            (Some((pm, px)), Some((am, ax))) => {
                report.push_str(&format!(
                    "  {} : parallax mean {pm:.1} max {px:.1} | diurnal aberration mean {am:.1} max {ax:.1} ({n} epochs)\n",
                    line.word,
                    n = g.rows.len()
                ));
            }
            _ => {
                report.push_str(&format!(
                    "  {} : absent — no rows carried both signatures\n",
                    line.word
                ));
            }
        }
    }

    report.push_str("\nDecomposition coefficients (per satellite, all epochs):\n");
    let mut line_pars: Vec<Option<([f64; 4], [f64; 4])>> = Vec::new();
    for line in lines.iter() {
        for (name, _, _) in &sats {
            let key = format!("{}-{}", line.word, name);
            let Some(g) = groups.iter().find(|g| g.name == key) else {
                continue;
            };
            match fit_group(g) {
                Some((x, sigma)) => {
                    let rms0 = rms_of(g)
                        .map(|v| format!("{v:.1}"))
                        .unwrap_or("na".to_string());
                    let rms1 = rms_reduced(g, &x)
                        .map(|v| format!("{v:.1}"))
                        .unwrap_or("na".to_string());
                    report.push_str(&format!(
                        "  {} {name:10} : c_par {:.2} ± {:.2}, c_aber {:.2} ± {:.2} | RMS {rms0} → {rms1} mas ({n} epochs)\n",
                        line.word,
                        x[2],
                        sigma[2],
                        x[3],
                        sigma[3],
                        n = g.rows.len()
                    ));
                }
                None => {
                    report.push_str(&format!(
                        "  {} {name:10} : fit void — fewer than 8 rows or a singular normal matrix\n",
                        line.word
                    ));
                }
            }
        }
        let pooled_fit = groups
            .iter()
            .find(|g| g.name == line.word)
            .and_then(fit_group);
        line_pars.push(pooled_fit);
    }

    report.push_str("\nPooled decomposition (all satellites per line):\n");
    for (i, line) in lines.iter().enumerate() {
        let Some(g) = groups.iter().find(|g| g.name == line.word) else {
            continue;
        };
        match line_pars.get(i).and_then(|p| *p) {
            Some((x, sigma)) => {
                let rms0 = rms_of(g)
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or("na".to_string());
                let rms1 = rms_reduced(g, &x)
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or("na".to_string());
                report.push_str(&format!(
                    "  {} : c_par {:.2} ± {:.2} → the tables {} | c_aber {:.2} ± {:.2} → the tables {} | RMS {rms0} → {rms1} mas ({n} epochs)\n",
                    line.word,
                    x[2],
                    sigma[2],
                    carries_word(x[2], sigma[2]),
                    x[3],
                    sigma[3],
                    carries_word(x[3], sigma[3]),
                    n = g.rows.len()
                ));
            }
            None => {
                report.push_str(&format!(
                    "  {} : fit void — fewer than 8 rows or a singular normal matrix\n",
                    line.word
                ));
            }
        }
    }

    report.push_str("\nRaw residual (the root-probe numbers — the Riss anchor):\n");
    let mut raw_means: Vec<(&str, f64, f64)> = Vec::new();
    for line in lines.iter() {
        let Some(g) = groups.iter().find(|g| g.name == line.word) else {
            continue;
        };
        match (rms_of(g), mean_vec(g)) {
            (Some(r), Some((mra, mdec))) => {
                raw_means.push((line.word, mra, mdec));
                report.push_str(&format!(
                    "  {} : RMS {r:.1} mas, mean ΔRA·cosδ {mra:+.1} mas, ΔDec {mdec:+.1} mas\n",
                    line.word
                ));
            }
            _ => {
                report.push_str(&format!("  {} : absent — no residual rows\n", line.word));
            }
        }
    }
    report.push_str("Pairwise mean-residual differences (mas):\n");
    for a in 0..raw_means.len() {
        for b in (a + 1)..raw_means.len() {
            let (wa, mra, mdec) = raw_means[a];
            let (wb, brra, brdec) = raw_means[b];
            let dra = mra - brra;
            let ddec = mdec - brdec;
            let mag = (dra * dra + ddec * ddec).sqrt();
            report.push_str(&format!(
                "  {wa} − {wb} : ΔRA·cosδ {dra:+.1} mas, ΔDec {ddec:+.1} mas, magnitude {mag:.1} mas\n"
            ));
        }
    }

    report.push_str(
        "\nReduced residual (after the per-satellite fits — the diurnal terms removed):\n",
    );
    let mut red_rms: Vec<(&str, f64)> = Vec::new();
    for line in lines.iter() {
        let mut sum2 = 0.0;
        let mut n = 0usize;
        for (name, _, _) in &sats {
            let key = format!("{}-{}", line.word, name);
            let Some(g) = groups.iter().find(|g| g.name == key) else {
                continue;
            };
            let Some((x, _)) = fit_group(g) else {
                continue;
            };
            for row in &g.rows {
                let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra);
                let rdec = row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec);
                sum2 += rra * rra + rdec * rdec;
                n += 1;
            }
        }
        if n == 0 {
            report.push_str(&format!("  {} : absent — no reduced rows\n", line.word));
            continue;
        }
        let rms = (sum2 / n as f64).sqrt();
        red_rms.push((line.word, rms));
        report.push_str(&format!(
            "  {} : RMS {rms:.1} mas ({n} epochs)\n",
            line.word
        ));
    }

    let mut riss_set: Vec<String> = Vec::new();
    for a in 0..raw_means.len() {
        for b in (a + 1)..raw_means.len() {
            let (wa, mra, mdec) = raw_means[a];
            let (wb, brra, brdec) = raw_means[b];
            let mag = ((mra - brra).powi(2) + (mdec - brdec).powi(2)).sqrt();
            riss_set.push(format!("{wa}−{wb} {mag:.1}"));
        }
    }
    report.push_str(&format!(
        "\nAnchor check: the fits carry free constants, so the fitted model reproduces the raw mean vectors exactly (LSQ identity) — the Riss under the reduction IS the raw pairwise set above ({}) and stands unchanged.\n",
        riss_set.join(" / ")
    ));

    report.push('\n');
    let mut sorted = red_rms.clone();
    sorted.sort_by(|a, b| a.1.total_cmp(&b.1));
    match sorted.as_slice() {
        [best, second, third] => {
            let delta = second.1 - best.1;
            let worst = third.1;
            report.push_str(&format!(
                "Verdict on the reduced residual: X = {x_mas:.1} mas, RMS(best) = {:.1} ({}) mas, RMS(second) = {:.1} ({}) mas, ΔRMS = {:.1} mas\n",
                best.1, best.0, second.1, second.0, delta
            ));
            if delta > x_mas {
                report.push_str(&format!(
                    "(i) the observations arbitrate — ephemeris {} carries the satellite observations closer (unweighted RMS {:.1} mas against {:.1} mas)\n",
                    best.0, best.1, second.1
                ));
            } else if worst < x_mas {
                report.push_str(&format!(
                    "(ii) the observations do not arbitrate — all three ephemerides lie within the observation uncertainty (worst RMS {worst:.1} < X)\n"
                ));
            } else if best.1 >= x_mas {
                report.push_str(&format!(
                    "(iii) the observations contradict all three — best RMS {:.1} ≥ X and no ephemeris carries measurably better (ΔRMS {delta:.1} ≤ X)\n",
                    best.1
                ));
            } else {
                report.push_str(&format!(
                    "none of the three pre-nailed outcomes holds — best RMS {:.1} < X ≤ worst {worst:.1}, ΔRMS {delta:.1} ≤ X\n",
                    best.1
                ));
            }
        }
        _ => {
            report.push_str(
                "Verdict: absent — fewer than three ephemeris lines carried a reduced RMS\n",
            );
        }
    }

    print!("{report}");
    if let Err(e) = std::fs::create_dir_all(&report_dir) {
        eprintln!("uranus-diurnal: report dir {report_dir} stays unbuilt — {e}");
    }
    let report_path = format!("{report_dir}/uranus_diurnal_decomposition.txt");
    let series_path = format!("{report_dir}/uranus_diurnal_decomposition_series.tsv");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("uranus-diurnal: report write void — {e}");
    }
    let mut series = String::new();
    series.push_str(
        "line\tsat\tjd_utc\tdra_mas\tddec_mas\tpar_ra_mas\tpar_dec_mas\taber_ra_mas\taber_dec_mas\n",
    );
    for g in &groups {
        for row in &g.rows {
            let mut parts: Vec<String> = g.name.splitn(2, '-').map(|s| s.to_string()).collect();
            if parts.len() == 1 {
                parts.push("all".to_string());
            }
            series.push_str(&format!(
                "{}\t{}\t{:.8}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\n",
                parts[0],
                parts[1],
                row.jd_utc,
                row.dra,
                row.ddec,
                row.par_ra,
                row.par_dec,
                row.aber_ra,
                row.aber_dec,
            ));
        }
    }
    if let Err(e) = std::fs::write(&series_path, &series) {
        eprintln!("uranus-diurnal: series write void — {e}");
    }
    println!("series: {series_path}");
    println!("report: {report_path}");
}
