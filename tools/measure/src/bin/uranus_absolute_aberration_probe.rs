use std::collections::HashMap;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use omegaflow::archivar::{
    body_barycenter_position, body_barycenter_velocity, embedded_lsk, fetch_raw_bytes,
    light_time_worldline, parse_ephemeris_binary, BodyEphemeris, C_LIGHT,
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
    dra: f64,
    ddec: f64,
    par_ra: f64,
    par_dec: f64,
    aber_ra: f64,
    aber_dec: f64,
    ann_ra: f64,
    ann_dec: f64,
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
        fit.row([1.0, 0.0, row.par_ra, row.aber_ra, row.ann_ra], row.dra);
        fit.row([0.0, 1.0, row.par_dec, row.aber_dec, row.ann_dec], row.ddec);
    }
    if fit.n < 10 {
        return None;
    }
    let x = solve5(fit.ata, fit.atb)?;
    let inv = inv5(fit.ata)?;
    let mut rss = 0.0;
    for row in &group.rows {
        let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra + x[4] * row.ann_ra);
        let rdec =
            row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec + x[4] * row.ann_dec);
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
        let rra = row.dra - (x[0] + x[2] * row.par_ra + x[3] * row.aber_ra + x[4] * row.ann_ra);
        let rdec =
            row.ddec - (x[1] + x[2] * row.par_dec + x[3] * row.aber_dec + x[4] * row.ann_dec);
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
            println!("uranus-absolute {word}: {path} bin void — absent on disk and the CDN fetch returned non-200");
            return Line { word, map: None };
        };
        let Some(eph) = parse_ephemeris_binary(&bytes) else {
            println!("uranus-absolute {word}: {path} reads but does not parse to a BodyEphemeris");
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
        println!("uranus-absolute {name} ({id}): no segment covers it");
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
    println!("Uranus absolute-offset/aberration decomposition — the annual aberration (~20\") separated from the diurnal parallax (~0.45\") in the absolute satellite residual.");

    let tsv_dir =
        arg_token(&args, "--tsv-dir").unwrap_or("data/vizier.cfa.harvard.edu".to_string());
    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let spk_dir = arg_token(&args, "--spk-dir").unwrap_or("data/naif.jpl.nasa.gov".to_string());
    let spk_name = arg_token(&args, "--spk").unwrap_or("ura111.bsp".to_string());
    let report_dir = arg_token(&args, "--report-dir").unwrap_or("state/reports".to_string());

    let Some(lsk) = embedded_lsk() else {
        eprintln!("uranus-absolute: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
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
            eprintln!("uranus-absolute: {spk_path} opens void — {e:?}");
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
                println!("uranus-absolute {name}: {tsv_path} reads void — {e}");
                continue;
            }
        };
        let obs = parse_obs(&text);
        println!("uranus-absolute {name}: {} rows", obs.len());
        sats.push((name.to_string(), sat, obs));
    }
    if sats.is_empty() {
        eprintln!(
            "uranus-absolute: no satellite carried both an SPK and observation rows (0 honored)"
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
    for line in lines.iter() {
        let Some(map) = line.map.as_ref() else {
            continue;
        };
        let mut line_rows: Vec<EpochRow> = Vec::new();
        for (name, sat, obs) in &sats {
            let mut sat_rows: Vec<EpochRow> = Vec::new();
            for o in obs {
                let unix = (o.jd_utc - UNIX_JD_OFFSET) * 86400.0;
                let Some(tdb) = lsk.unix_to_tdb(unix) else {
                    continue;
                };
                let Some(geocenter) = body_barycenter_position("earth", tdb, map) else {
                    continue;
                };
                let Some((s_em, _)) =
                    light_time_worldline(geocenter, tdb, &|t| sat_worldline(sat, &spk, map, t))
                else {
                    continue;
                };
                let Some(u_geo) = toward_unit(geocenter, s_em) else {
                    continue;
                };
                let (off, v_rot) = station_offset(o.jd_utc, OBS_LAT_DEG, OBS_LON_DEG, OBS_ALT_M);
                let sta = [
                    geocenter[0] + off[0],
                    geocenter[1] + off[1],
                    geocenter[2] + off[2],
                ];
                let Some(u_top) = toward_unit(sta, s_em) else {
                    continue;
                };
                let s_par = vec_sub(u_top, u_geo);
                let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
                let udotv = dot(u_geo, v_rot);
                let s_aber = [
                    (v_rot[0] - u_geo[0] * udotv) / C_LIGHT,
                    (v_rot[1] - u_geo[1] * udotv) / C_LIGHT,
                    (v_rot[2] - u_geo[2] * udotv) / C_LIGHT,
                ];
                let Some(v_earth) = body_barycenter_velocity("earth", tdb, map) else {
                    continue;
                };
                let udotve = dot(u_geo, v_earth);
                let s_ann = [
                    (v_earth[0] - u_geo[0] * udotve) / C_LIGHT,
                    (v_earth[1] - u_geo[1] * udotve) / C_LIGHT,
                    (v_earth[2] - u_geo[2] * udotve) / C_LIGHT,
                ];
                let u_obs = icrs_unit(o.ra_deg, o.dec_deg);
                let (e_ra, e_dec) = tangent_basis(o.ra_deg, o.dec_deg);
                let r = vec_sub(u_obs, u_geo);
                let row = EpochRow {
                    dra: dot(r, e_ra) * MAS_PER_RAD,
                    ddec: dot(r, e_dec) * MAS_PER_RAD,
                    par_ra: dot(s_par, e_ra) * MAS_PER_RAD,
                    par_dec: dot(s_par, e_dec) * MAS_PER_RAD,
                    aber_ra: dot(s_aber, e_ra) * MAS_PER_RAD,
                    aber_dec: dot(s_aber, e_dec) * MAS_PER_RAD,
                    ann_ra: dot(s_ann, e_ra) * MAS_PER_RAD,
                    ann_dec: dot(s_ann, e_dec) * MAS_PER_RAD,
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
    }

    let mut report = String::new();
    report.push_str("Uranus absolute-offset/aberration decomposition\n\n");
    report.push_str(&format!(
        "Observatory: Pico dos Dias (MPC 874), λ = {OBS_LON_DEG}°, φ = {OBS_LAT_DEG}°, h = {OBS_ALT_M} m\n"
    ));
    report.push_str(&format!(
        "Tables: five Camargo+ 2015 satellite tables (A&A 582 A8), moon orbits {spk_name}, {sigma_mean:.1} mas mean per-row sigma, X = 2·⟨σ⟩ = {x_mas:.1} mas\n"
    ));
    report.push_str("Model: geocentric astrometric (ICRS, light-time, no aberration/deflection) — the root-probe convention\n");
    report.push_str("Signatures: parallax = topocentric − geocentric (diurnal, ~0.45\"), diurnal aberration = ω×r observer rotation, annual aberration = Earth barycentric velocity (barycenter_velocity(\"earth\")) projected on the sky, /c (~20\")\n");
    report.push_str("Fit per line and satellite: ΔRA·cosδ, ΔDec = c0 + c_par·parallax + c_aber·diurnal_aberration + c_ann·annual_aberration\n\n");

    report.push_str("Signature swing over the series (mas):\n");
    for line in lines.iter() {
        let Some(g) = groups.iter().find(|g| g.name == line.word) else {
            continue;
        };
        match (
            swing_of(&g.rows, &|r| (r.par_ra, r.par_dec)),
            swing_of(&g.rows, &|r| (r.aber_ra, r.aber_dec)),
            swing_of(&g.rows, &|r| (r.ann_ra, r.ann_dec)),
        ) {
            (Some((pm, px)), Some((am, ax)), Some((nm, nx))) => {
                report.push_str(&format!(
                    "  {} : parallax mean {pm:.1} max {px:.1} ({px:.2}\") | diurnal aberration mean {am:.1} max {ax:.1} | annual aberration mean {nm:.1} max {nx:.1} ({nx:.2}\") ({n} epochs)\n",
                    line.word,
                    n = g.rows.len()
                ));
            }
            _ => {
                report.push_str(&format!(
                    "  {} : absent — no rows carried the signatures\n",
                    line.word
                ));
            }
        }
    }

    report.push_str("\nPooled decomposition (all satellites per line, the absolute residual c0 after all three terms removed):\n");
    let mut c0s: Vec<(&str, f64, f64)> = Vec::new();
    let mut line_fits: Vec<Option<([f64; 5], [f64; 5])>> = Vec::new();
    for line in lines.iter() {
        let Some(g) = groups.iter().find(|g| g.name == line.word) else {
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
                let par_txt = match swing_of(&g.rows, &|r| (r.par_ra, r.par_dec)) {
                    Some((pm, _)) => format!(
                        "share {:.1} mas over a {:.1} mas parallax swing",
                        x[2] * pm,
                        pm
                    ),
                    None => "parallax swing absent".to_string(),
                };
                let ann_txt = match swing_of(&g.rows, &|r| (r.ann_ra, r.ann_dec)) {
                    Some((nm, _)) => format!(
                        "share {:.1} mas over a {:.1} mas = {:.2}\" annual swing",
                        x[4] * nm,
                        nm,
                        nm
                    ),
                    None => "annual swing absent".to_string(),
                };
                let c0mag = (x[0] * x[0] + x[1] * x[1]).sqrt();
                c0s.push((line.word, x[0], x[1]));
                report.push_str(&format!(
                    "  {} : c0 {:+.1}/{:+.1} mas (magnitude {c0mag:.1} mas) | c_par {:.3} ± {:.3} → {} ({par_txt}) | c_aber {:.3} ± {:.3} → {} | c_ann {:.4} ± {:.4} → {} ({ann_txt}) | RMS {rms0} → {rms1} mas ({n} epochs)\n",
                    line.word,
                    x[0],
                    x[1],
                    x[2],
                    sigma[2],
                    carries_word(x[2], sigma[2]),
                    x[3],
                    sigma[3],
                    carries_word(x[3], sigma[3]),
                    x[4],
                    sigma[4],
                    carries_word(x[4], sigma[4]),
                    n = g.rows.len()
                ));
                line_fits.push(Some((x, sigma)));
            }
            None => {
                report.push_str(&format!(
                    "  {} : fit void — fewer than 10 rows or a singular normal matrix\n",
                    line.word
                ));
                line_fits.push(None);
            }
        }
    }

    report.push_str("\nAbsolute residual vector per line (c0 — the answer to which line lies closest to zero, once the diurnal term AND the annual aberration are gone):\n");
    let mut abs_vecs: Vec<(&str, f64, f64, f64)> = Vec::new();
    for (word, mra, mdec) in &c0s {
        let mag = (mra * mra + mdec * mdec).sqrt();
        abs_vecs.push((word, *mra, *mdec, mag));
        report.push_str(&format!(
            "  {word:9} : ΔRA·cosδ {mra:+.1} mas, ΔDec {mdec:+.1} mas, magnitude {mag:.1} mas\n"
        ));
    }
    let mut sorted = abs_vecs.clone();
    sorted.sort_by(|a, b| a.3.total_cmp(&b.3));
    match sorted.as_slice() {
        [best, second, third] => {
            report.push_str(&format!(
                "Closest to zero: {} at {:.1} mas | then {} at {:.1} mas | then {} at {:.1} mas\n",
                best.0, best.3, second.0, second.3, third.0, third.3
            ));
        }
        _ => {
            report.push_str(
                "Closest to zero: absent — fewer than three lines carried an absolute vector\n",
            );
        }
    }

    report.push_str(
        "Pairwise differences of the absolute vectors (mas — the Riss after the diurnal AND annual reduction):\n",
    );
    for a in 0..abs_vecs.len() {
        for b in (a + 1)..abs_vecs.len() {
            let (wa, mra, mdec, _) = abs_vecs[a];
            let (wb, brra, brdec, _) = abs_vecs[b];
            let dra = mra - brra;
            let ddec = mdec - brdec;
            let mag = (dra * dra + ddec * ddec).sqrt();
            report.push_str(&format!(
                "  {wa} − {wb} : ΔRA·cosδ {dra:+.1} mas, ΔDec {ddec:+.1} mas, magnitude {mag:.1} mas\n"
            ));
        }
    }

    report.push('\n');
    print!("{report}");
    if let Err(e) = std::fs::create_dir_all(&report_dir) {
        eprintln!("uranus-absolute: report dir {report_dir} stays unbuilt — {e}");
    }
    let report_path = format!("{report_dir}/uranus_absolute_aberration.txt");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("uranus-absolute: report write void — {e}");
    }
    let mut series = String::new();
    series.push_str(
        "line\tsat\tdra_mas\tddec_mas\tpar_ra_mas\tpar_dec_mas\taber_ra_mas\taber_dec_mas\tann_ra_mas\tann_dec_mas\n",
    );
    for g in &groups {
        for row in &g.rows {
            let mut parts: Vec<String> = g.name.splitn(2, '-').map(|s| s.to_string()).collect();
            if parts.len() == 1 {
                parts.push("all".to_string());
            }
            series.push_str(&format!(
                "{}\t{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\n",
                parts[0],
                parts[1],
                row.dra,
                row.ddec,
                row.par_ra,
                row.par_dec,
                row.aber_ra,
                row.aber_dec,
                row.ann_ra,
                row.ann_dec,
            ));
        }
    }
    if let Err(e) = std::fs::write(
        format!("{report_dir}/uranus_absolute_aberration_series.tsv"),
        &series,
    ) {
        eprintln!("uranus-absolute: series write void — {e}");
    }
    println!("report: {report_path}");
}
