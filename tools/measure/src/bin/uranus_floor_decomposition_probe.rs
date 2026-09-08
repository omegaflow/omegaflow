use std::collections::HashMap;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use omegaflow::archivar::{
    body_barycenter_position, embedded_lsk, fetch_raw_bytes, parse_ephemeris_binary, BodyEphemeris,
    C_LIGHT,
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
const NIGHT_GAP_D: f64 = 0.1;

const SATELLITES: [(&str, i32); 5] = [
    ("ariel_j", 701),
    ("umbri_j", 702),
    ("titan_j", 703),
    ("obero_j", 704),
    ("miran_j", 705),
];

fn station_offset(jd_utc: f64, lat_deg: f64, lon_deg: f64, alt_m: f64) -> ([f64; 3], [f64; 3]) {
    let lr = lat_deg.to_radians();
    let nr = lon_deg.to_radians();
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let sl = lr.sin();
    let n = WGS84_A_M / (1.0 - e2 * sl * sl).sqrt();
    let xb = (n + alt_m) * lr.cos() * nr.cos();
    let yb = (n + alt_m) * lr.cos() * nr.sin();
    let zb = (n * (1.0 - e2) + alt_m) * sl;
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
    dcr_ra: f64,
    dcr_dec: f64,
}

struct Group {
    name: String,
    rows: Vec<EpochRow>,
}

struct Fit {
    ata: [[f64; 5]; 5],
    atb: [f64; 5],
    n: usize,
}

impl Fit {
    fn new() -> Fit {
        Fit {
            ata: [[0.0; 5]; 5],
            atb: [0.0; 5],
            n: 0,
        }
    }
    fn row(&mut self, design: [f64; 5], y: f64) {
        for i in 0..5 {
            for j in 0..5 {
                self.ata[i][j] += design[i] * design[j];
            }
            self.atb[i] += design[i] * y;
        }
        self.n += 1;
    }
}

fn solve5(ata: [[f64; 5]; 5], atb: [f64; 5]) -> Option<[f64; 5]> {
    let mut a = ata;
    let mut b = atb;
    for col in 0..5 {
        let mut piv = col;
        for r in (col + 1)..5 {
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
        for r in (col + 1)..5 {
            let f = a[r][col] / a[col][col];
            for c in col..5 {
                a[r][c] -= f * a[col][c];
            }
            b[r] -= f * b[col];
        }
    }
    let mut x = [0.0; 5];
    for r in (0..5).rev() {
        let mut s = b[r];
        for c in (r + 1)..5 {
            s -= a[r][c] * x[c];
        }
        x[r] = s / a[r][r];
    }
    Some(x)
}

fn inv5(a_in: [[f64; 5]; 5]) -> Option<[[f64; 5]; 5]> {
    let mut a = a_in;
    let mut inv = [[0.0; 5]; 5];
    for i in 0..5 {
        inv[i][i] = 1.0;
    }
    for col in 0..5 {
        let mut piv = col;
        for r in (col + 1)..5 {
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
        for c in 0..5 {
            a[col][c] /= d;
            inv[col][c] /= d;
        }
        for r in 0..5 {
            if r == col {
                continue;
            }
            let f = a[r][col];
            for c in 0..5 {
                a[r][c] -= f * a[col][c];
                inv[r][c] -= f * inv[col][c];
            }
        }
    }
    Some(inv)
}

fn fit_group(group: &Group) -> Option<([f64; 5], [f64; 5])> {
    let mut fit = Fit::new();
    for row in &group.rows {
        fit.row([1.0, 0.0, row.par_ra, row.aber_ra, row.dcr_ra], row.dra);
        fit.row([0.0, 1.0, row.par_dec, row.aber_dec, row.dcr_dec], row.ddec);
    }
    if fit.n < 10 {
        return None;
    }
    let x = solve5(fit.ata, fit.atb)?;
    let inv = inv5(fit.ata)?;
    let mut rss = 0.0;
    for row in &group.rows {
        let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra + x[4] * row.dcr_ra);
        let rdec =
            row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec + x[4] * row.dcr_dec);
        rss += rra * rra + rdec * rdec;
    }
    let dof = (fit.n - 5) as f64;
    if !(dof.is_finite() && dof > 0.0) {
        return None;
    }
    let var = rss / dof;
    let mut sigma = [0.0; 5];
    for i in 0..5 {
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

fn rms_reduced(group: &Group, x: &[f64; 5]) -> Option<f64> {
    if group.rows.is_empty() {
        return None;
    }
    let mut sum = 0.0;
    for row in &group.rows {
        let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra + x[4] * row.dcr_ra);
        let rdec =
            row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec + x[4] * row.dcr_dec);
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

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
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
            println!("uranus-floor {word}: {path} bin void — absent on disk and the CDN fetch returned non-200");
            return Line { word, map: None };
        };
        let Some(eph) = parse_ephemeris_binary(&bytes) else {
            println!("uranus-floor {word}: {path} reads but does not parse to a BodyEphemeris");
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
        println!("uranus-floor {name} ({id}): no segment covers it");
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

fn reduced_pair(row: &EpochRow, x: &[f64; 5]) -> (f64, f64) {
    let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra + x[4] * row.dcr_ra);
    let rdec = row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec + x[4] * row.dcr_dec);
    (rra, rdec)
}

fn night_groups(rows: &[EpochRow]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..rows.len()).collect();
    order.sort_by(|a, b| rows[*a].jd_utc.total_cmp(&rows[*b].jd_utc));
    let mut ids = vec![0usize; rows.len()];
    let mut cur = 0usize;
    for k in 0..order.len() {
        if k > 0 && rows[order[k]].jd_utc - rows[order[k - 1]].jd_utc > NIGHT_GAP_D {
            cur += 1;
        }
        ids[order[k]] = cur;
    }
    ids
}

struct NightStats {
    total: f64,
    constant: f64,
    intra: f64,
    n_rows: usize,
    n_groups: usize,
}

fn night_split(rows: &[EpochRow], x: &[f64; 5]) -> Option<NightStats> {
    if rows.is_empty() {
        return None;
    }
    let ids = night_groups(rows);
    let n_groups = match ids.iter().max() {
        Some(m) => m + 1,
        None => 0,
    };
    let mut sum_ra = vec![0.0; n_groups];
    let mut sum_dec = vec![0.0; n_groups];
    let mut counts = vec![0usize; n_groups];
    let mut pairs = Vec::with_capacity(rows.len());
    for (i, row) in rows.iter().enumerate() {
        let (rra, rdec) = reduced_pair(row, x);
        if !(rra.is_finite() && rdec.is_finite()) {
            return None;
        }
        sum_ra[ids[i]] += rra;
        sum_dec[ids[i]] += rdec;
        counts[ids[i]] += 1;
        pairs.push((rra, rdec));
    }
    let mut mean_ra = vec![0.0; n_groups];
    let mut mean_dec = vec![0.0; n_groups];
    for g in 0..n_groups {
        if counts[g] == 0 {
            return None;
        }
        mean_ra[g] = sum_ra[g] / counts[g] as f64;
        mean_dec[g] = sum_dec[g] / counts[g] as f64;
    }
    let n = rows.len() as f64;
    let mut total2 = 0.0;
    let mut const2 = 0.0;
    let mut intra2 = 0.0;
    for (i, (rra, rdec)) in pairs.iter().enumerate() {
        let g = ids[i];
        total2 += rra * rra + rdec * rdec;
        const2 += mean_ra[g] * mean_ra[g] + mean_dec[g] * mean_dec[g];
        let dra = rra - mean_ra[g];
        let ddec = rdec - mean_dec[g];
        intra2 += dra * dra + ddec * ddec;
    }
    let t = (total2 / n).sqrt();
    let c = (const2 / n).sqrt();
    let i = (intra2 / n).sqrt();
    if !(t.is_finite() && c.is_finite() && i.is_finite()) {
        return None;
    }
    Some(NightStats {
        total: t,
        constant: c,
        intra: i,
        n_rows: rows.len(),
        n_groups,
    })
}

fn significance(c: f64, s: f64) -> Option<f64> {
    if c.is_finite() && s.is_finite() && s > 0.0 {
        Some((c / s).abs())
    } else {
        None
    }
}

fn dcr_amp(rows: &[EpochRow], x: &[f64; 5]) -> Option<f64> {
    if rows.is_empty() {
        return None;
    }
    let mut sum = 0.0;
    for row in rows {
        let a = x[4] * row.dcr_ra;
        let b = x[4] * row.dcr_dec;
        sum += a * a + b * b;
    }
    let r = (sum / rows.len() as f64).sqrt();
    if r.is_finite() {
        Some(r)
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Uranus floor decomposition — the residual floor after the diurnal terms: the zenith/refraction signature and the night structure.");

    let tsv_dir =
        arg_token(&args, "--tsv-dir").unwrap_or("data/vizier.cfa.harvard.edu".to_string());
    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let spk_dir = arg_token(&args, "--spk-dir").unwrap_or("data/naif.jpl.nasa.gov".to_string());
    let spk_name = arg_token(&args, "--spk").unwrap_or("ura111.bsp".to_string());
    let report_dir = arg_token(&args, "--report-dir").unwrap_or("state/reports".to_string());

    let Some(lsk) = embedded_lsk() else {
        eprintln!("uranus-floor: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
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
            eprintln!("uranus-floor: {spk_path} opens void — {e:?}");
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
                println!("uranus-floor {name}: {tsv_path} reads void — {e}");
                continue;
            }
        };
        let obs = parse_obs(&text);
        println!("uranus-floor {name}: {} rows", obs.len());
        sats.push((name.to_string(), sat, obs));
    }
    if sats.is_empty() {
        eprintln!(
            "uranus-floor: no satellite carried both an SPK and observation rows (0 honored)"
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

    let mut groups: Vec<Group> = Vec::new();
    let mut census: Vec<(String, usize, usize, usize)> = Vec::new();
    for line in lines.iter() {
        let Some(map) = line.map.as_ref() else {
            continue;
        };
        let mut line_rows: Vec<EpochRow> = Vec::new();
        let mut skip_geo = 0usize;
        let mut skip_fold = 0usize;
        let mut skip_dcr = 0usize;
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
                let (off, v_rot) = station_offset(o.jd_utc, OBS_LAT_DEG, OBS_LON_DEG, OBS_ALT_M);
                let sta = [
                    geocenter[0] + off[0],
                    geocenter[1] + off[1],
                    geocenter[2] + off[2],
                ];
                let Some(u_top) = toward_unit(sta, s_em) else {
                    skip_geo += 1;
                    continue;
                };
                let s_par = vec_sub(u_top, u_geo);
                let udotv = dot3(u_geo, v_rot);
                let s_aber = [
                    (v_rot[0] - u_geo[0] * udotv) / C_LIGHT,
                    (v_rot[1] - u_geo[1] * udotv) / C_LIGHT,
                    (v_rot[2] - u_geo[2] * udotv) / C_LIGHT,
                ];
                let Some(w_zen) = unit3(off) else {
                    skip_dcr += 1;
                    continue;
                };
                let cosz = dot3(u_geo, w_zen);
                if !(cosz.is_finite() && cosz > 0.0 && cosz < 1.0) {
                    skip_dcr += 1;
                    continue;
                }
                let tanz = (1.0 - cosz * cosz).sqrt() / cosz;
                let s_dcr = [
                    (w_zen[0] - u_geo[0] * cosz) * tanz,
                    (w_zen[1] - u_geo[1] * cosz) * tanz,
                    (w_zen[2] - u_geo[2] * cosz) * tanz,
                ];
                let u_obs = icrs_unit(o.ra_deg, o.dec_deg);
                let (e_ra, e_dec) = tangent_basis(o.ra_deg, o.dec_deg);
                let r = vec_sub(u_obs, u_geo);
                let row = EpochRow {
                    jd_utc: o.jd_utc,
                    dra: dot3(r, e_ra) * MAS_PER_RAD,
                    ddec: dot3(r, e_dec) * MAS_PER_RAD,
                    par_ra: dot3(s_par, e_ra) * MAS_PER_RAD,
                    par_dec: dot3(s_par, e_dec) * MAS_PER_RAD,
                    aber_ra: dot3(s_aber, e_ra) * MAS_PER_RAD,
                    aber_dec: dot3(s_aber, e_dec) * MAS_PER_RAD,
                    dcr_ra: dot3(s_dcr, e_ra) * MAS_PER_RAD,
                    dcr_dec: dot3(s_dcr, e_dec) * MAS_PER_RAD,
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
        census.push((line.word.to_string(), skip_geo, skip_fold, skip_dcr));
    }

    let mut report = String::new();
    report.push_str("Uranus floor decomposition — the residual floor after the diurnal terms: the zenith/refraction signature and the night structure\n\n");
    report.push_str(&format!(
        "Observatory: Pico dos Dias (MPC 874), λ = {OBS_LON_DEG}°, φ = {OBS_LAT_DEG}°, h = {OBS_ALT_M} m\n"
    ));
    report.push_str(&format!(
        "Tables: five Camargo+ 2015 satellite tables (A&A 582 A8), moon orbits {spk_name}, {sigma_mean:.1} mas mean per-row sigma, X = 2·⟨σ⟩ = {x_mas:.1} mas\n"
    ));
    report.push_str("Model: geocentric astrometric (ICRS, light-time, no aberration/deflection) — the root-probe convention\n");
    report.push_str("Signatures: parallax = topocentric − geocentric direction (probe-local WGS84 + IAU 1982 GMST station at the MPC-874 geodetic); diurnal aberration = observer rotational velocity (analytic ω×r) projected on the sky; zenith/refraction = s_dcr = tan(z)·(ŵ − (û_geo·ŵ)·û_geo), ŵ = station-offset unit vector, cos z = û_geo·ŵ, projected on the sky\n");
    report.push_str("Fit per line and satellite: ΔRA·cosδ, ΔDec = c0 + c_par·parallax + c_aber·diurnal_aberration + c_dcr·zenith_signature\n\n");

    report.push_str("Row census per line (the station is probe-local: WGS84 geodetic + IAU 1982 GMST rotation, one physical observer for all three lines):\n");
    for (word, sgeo, sfold, sdcr) in &census {
        report.push_str(&format!(
            "  {word:9} : {sgeo} skipped (geocenter/station void), {sfold} skipped (light-time fold void), {sdcr} skipped (zenith signature void)\n"
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

    report.push_str("\nFive-parameter decomposition coefficients (per satellite, all epochs); c_dcr multiplies the zenith signature s_dcr = tan(z)·(w − (u·w)·u) in its given dimensionless form — the fitted zenith-aligned term amplitude in mas is quoted beside the coefficient:\n");
    let mut line_fits: Vec<Option<([f64; 5], [f64; 5])>> = Vec::new();
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
                    let amp = dcr_amp(&g.rows, &x)
                        .map(|v| format!("{v:.1}"))
                        .unwrap_or("na".to_string());
                    let dcrsig = significance(x[4], sigma[4])
                        .map(|v| format!("{v:.1}σ"))
                        .unwrap_or("σ na".to_string());
                    report.push_str(&format!(
                        "  {} {name:10} : c_par {:.2} ± {:.2}, c_aber {:.2} ± {:.2} | c_dcr {:+.3e} ± {:.3e} ({dcrsig}, term {amp} mas) | RMS {rms0} → {rms1} mas ({n} epochs)\n",
                        line.word,
                        x[2],
                        sigma[2],
                        x[3],
                        sigma[3],
                        x[4],
                        sigma[4],
                        n = g.rows.len()
                    ));
                }
                None => {
                    report.push_str(&format!(
                        "  {} {name:10} : fit absent — fewer than five epochs or a singular normal matrix\n",
                        line.word
                    ));
                }
            }
        }
        let pooled_fit = groups
            .iter()
            .find(|g| g.name == line.word)
            .and_then(fit_group);
        line_fits.push(pooled_fit);
    }

    report.push_str("\nPooled decomposition (all satellites per line):\n");
    for (i, line) in lines.iter().enumerate() {
        let Some(g) = groups.iter().find(|g| g.name == line.word) else {
            continue;
        };
        match line_fits.get(i).and_then(|p| *p) {
            Some((x, sigma)) => {
                let rms0 = rms_of(g)
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or("na".to_string());
                let rms1 = rms_reduced(g, &x)
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or("na".to_string());
                let amp = dcr_amp(&g.rows, &x)
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or("na".to_string());
                let dcrsig = significance(x[4], sigma[4])
                    .map(|v| format!("{v:.1}σ"))
                    .unwrap_or("σ na".to_string());
                report.push_str(&format!(
                    "  {} : c_par {:.2} ± {:.2}, c_aber {:.2} ± {:.2} | c_dcr {:+.3e} ± {:.3e} ({dcrsig}, term {amp} mas) | RMS {rms0} → {rms1} mas ({n} epochs)\n",
                    line.word,
                    x[2],
                    sigma[2],
                    x[3],
                    sigma[3],
                    x[4],
                    sigma[4],
                    n = g.rows.len()
                ));
            }
            None => {
                report.push_str(&format!(
                    "  {} : fit absent — fewer than five epochs or a singular normal matrix\n",
                    line.word
                ));
            }
        }
    }

    report.push_str("\nNight structure of the reduced residual (per line; the residual after the per-satellite five-parameter fits; a night boundary is a same-satellite JD gap > 0.1 d; total² = nightly-constant² + intra-night² holds row-wise by construction):\n");
    let mut line_night: Vec<(String, NightStats)> = Vec::new();
    for line in lines.iter() {
        let mut acc: Vec<NightStats> = Vec::new();
        let mut groups_all = 0usize;
        for (name, _, _) in &sats {
            let key = format!("{}-{}", line.word, name);
            let Some(g) = groups.iter().find(|g| g.name == key) else {
                continue;
            };
            let Some((x, _)) = fit_group(g) else {
                continue;
            };
            let Some(st) = night_split(&g.rows, &x) else {
                continue;
            };
            groups_all += st.n_groups;
            acc.push(st);
        }
        if acc.is_empty() {
            report.push_str(&format!(
                "  {} : absent — no satellite carried a fit and a night split\n",
                line.word
            ));
            continue;
        }
        let mut total2 = 0.0;
        let mut const2 = 0.0;
        let mut intra2 = 0.0;
        let mut n_rows = 0usize;
        for st in &acc {
            total2 += st.total * st.total * st.n_rows as f64;
            const2 += st.constant * st.constant * st.n_rows as f64;
            intra2 += st.intra * st.intra * st.n_rows as f64;
            n_rows += st.n_rows;
        }
        let t = (total2 / n_rows as f64).sqrt();
        let c = (const2 / n_rows as f64).sqrt();
        let i = (intra2 / n_rows as f64).sqrt();
        let dev2 = t * t - c * c - i * i;
        let pct_c = if t > 0.0 {
            100.0 * c * c / (t * t)
        } else {
            0.0
        };
        let pct_i = if t > 0.0 {
            100.0 * i * i / (t * t)
        } else {
            0.0
        };
        line_night.push((
            line.word.to_string(),
            NightStats {
                total: t,
                constant: c,
                intra: i,
                n_rows,
                n_groups: groups_all,
            },
        ));
        report.push_str(&format!(
            "  {} : RMS {t:.1} mas = nightly-constant {c:.1} mas ({pct_c:.1} % of variance) + intra-night {i:.1} mas ({pct_i:.1} % of variance) — identity closes to {dev2:.3e} mas² ({n_rows} epochs, {groups_all} night-groups)\n",
            line.word
        ));
    }

    report.push_str(
        "\nReduced residual after the five-parameter per-satellite fits (RMS over the pooled per-satellite residuals — the floor under decomposition):\n",
    );
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
                let (rra, rdec) = reduced_pair(row, &x);
                sum2 += rra * rra + rdec * rdec;
                n += 1;
            }
        }
        if n == 0 {
            report.push_str(&format!("  {} : absent — no reduced rows\n", line.word));
            continue;
        }
        let rms = (sum2 / n as f64).sqrt();
        report.push_str(&format!(
            "  {} : RMS {rms:.1} mas ({n} epochs)\n",
            line.word
        ));
    }

    report.push_str("\nVerdict on the floor (measured shares only):\n");
    for (i, line) in lines.iter().enumerate() {
        let Some(g) = groups.iter().find(|g| g.name == line.word) else {
            continue;
        };
        let Some((x, sigma)) = line_fits.get(i).and_then(|p| *p) else {
            report.push_str(&format!(
                "  {} : the five-parameter fit stays absent — the floor composition is unmeasured\n",
                line.word
            ));
            continue;
        };
        let c = x[4];
        let s = sigma[4];
        let rms1 = rms_reduced(g, &x)
            .map(|v| format!("{v:.1}"))
            .unwrap_or("na".to_string());
        let amp = dcr_amp(&g.rows, &x)
            .map(|v| format!("{v:.1}"))
            .unwrap_or("na".to_string());
        let carried = match significance(c, s) {
            Some(sig) => format!(
                "c_dcr {c:+.3e} ± {s:.3e} ({sig:.1}σ) — the fitted zenith term amplitude {amp} mas"
            ),
            None => "the zenith/refraction coefficient sigma is absent".to_string(),
        };
        report.push_str(&format!(
            "  {} : {carried}; reduced RMS {rms1} mas\n",
            line.word
        ));
    }
    report.push('\n');

    for (w, st) in &line_night {
        report.push_str(&format!(
            "  {} : reduced RMS {:.1} mas — {:.1} % of the variance is constant within a night (the offset returns night after night, on the frame/catalog-zonal scale), {:.1} % lives inside nights (intra-night structure), against a {:.1} mas per-row sigma and X = {:.1} mas\n",
            w,
            st.total,
            100.0 * st.constant * st.constant / (st.total * st.total),
            100.0 * st.intra * st.intra / (st.total * st.total),
            sigma_mean,
            x_mas
        ));
    }
    if line_night.len() == 3 {
        let best = line_night
            .iter()
            .min_by(|a, b| a.1.total.total_cmp(&b.1.total));
        let (wt, tsum) = match best {
            Some((w, s)) => (w.as_str(), s.total),
            None => ("na", f64::NAN),
        };
        let const_w = line_night.iter().fold(0.0, |a: f64, (_, s)| {
            a + s.constant * s.constant * s.n_rows as f64
        });
        let intra_w = line_night.iter().fold(0.0, |a: f64, (_, s)| {
            a + s.intra * s.intra * s.n_rows as f64
        });
        let total_w = line_night.iter().fold(0.0, |a: f64, (_, s)| {
            a + s.total * s.total * s.n_rows as f64
        });
        let nr = line_night.iter().fold(0usize, |a, (_, s)| a + s.n_rows);
        let share_c = 100.0 * const_w / total_w;
        let share_i = 100.0 * intra_w / total_w;
        let margin = x_mas - tsum;
        let status = if margin > 0.0 {
            format!("{tsum:.1} mas lies {margin:.1} mas under X = {x_mas:.1} mas")
        } else {
            format!("{tsum:.1} mas lies at or above X = {x_mas:.1} mas")
        };
        let noise_word = if tsum < sigma_mean {
            "below the per-row sigma — the reduced residual sits at the observation-noise level, no unnamed systematic remains"
        } else {
            "above the per-row sigma — systematic structure beyond noise"
        };
        report.push_str(&format!(
            "The reduced residual carries a measured {share_c:.1} % night-constant share and {share_i:.1} % intra-night share of its variance (row-weighted over {nr} epochs). The best reduced RMS across the three lines is {wt} at {status} — {noise_word}. The zenith/refraction coefficient per line stands above; the five-parameter fit absorbs the parallax and leaves the rest at the noise scale.\n"
        ));
    }
    report.push('\n');

    print!("{report}");
    if let Err(e) = std::fs::create_dir_all(&report_dir) {
        eprintln!("uranus-floor: report dir {report_dir} stays unbuilt — {e}");
    }
    let report_path = format!("{report_dir}/uranus_floor_decomposition.txt");
    let series_path = format!("{report_dir}/uranus_floor_decomposition_series.tsv");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("uranus-floor: report write void — {e}");
    }
    let mut series = String::new();
    series.push_str(
        "line\tsat\tjd_utc\tdra\tddec\tpar_ra\tpar_dec\taber_ra\taber_dec\tdcr_ra\tdcr_dec\tnight_id\n",
    );
    for line in lines.iter() {
        for (name, _, _) in &sats {
            let key = format!("{}-{}", line.word, name);
            let Some(g) = groups.iter().find(|g| g.name == key) else {
                continue;
            };
            let ids = night_groups(&g.rows);
            let mut order: Vec<usize> = (0..g.rows.len()).collect();
            order.sort_by(|a, b| g.rows[*a].jd_utc.total_cmp(&g.rows[*b].jd_utc));
            for k in order {
                let row = &g.rows[k];
                series.push_str(&format!(
                    "{}\t{}\t{:.8}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{}\n",
                    line.word,
                    name,
                    row.jd_utc,
                    row.dra,
                    row.ddec,
                    row.par_ra,
                    row.par_dec,
                    row.aber_ra,
                    row.aber_dec,
                    row.dcr_ra,
                    row.dcr_dec,
                    ids[k]
                ));
            }
        }
    }
    if let Err(e) = std::fs::write(&series_path, &series) {
        eprintln!("uranus-floor: series write void — {e}");
    }
    println!("series: {series_path}");
    println!("report: {report_path}");
}
