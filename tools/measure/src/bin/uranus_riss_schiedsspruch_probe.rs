use std::collections::HashMap;

use omegaflow::archivar::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use omegaflow::archivar::{
    body_barycenter_position, body_barycenter_velocity, embedded_lsk, fetch_raw_bytes,
    parse_ephemeris_binary, BodyEphemeris, C_LIGHT,
};
use omegaflow::cdn::CDN_BASE;

const OBS_LAT_DEG: f64 = -22.534444444;
const OBS_LON_DEG: f64 = -45.5825;
const OBS_ALT_M: f64 = 1810.7;
const UNIX_JD_OFFSET: f64 = 2440587.5;
const MAS_PER_RAD: f64 = 206_264_806.247_096_36;
const BIN_TTL_S: u64 = 604800;

#[derive(Clone)]
struct Obs {
    jd_utc: f64,
    ra_deg: f64,
    dec_deg: f64,
    e_ra_mas: f64,
    e_dec_mas: f64,
    sat: String,
}

struct Line {
    word: &'static str,
    map: Option<HashMap<String, BodyEphemeris>>,
}

struct Predict {
    unit: [f64; 3],
    vel: [f64; 3],
}

struct Acc {
    n: usize,
    absent: usize,
    sum_sep2: f64,
    sum_dra2: f64,
    sum_ddec2: f64,
    sum_along2: f64,
    sum_cross2: f64,
}

impl Acc {
    fn new() -> Acc {
        Acc {
            n: 0,
            absent: 0,
            sum_sep2: 0.0,
            sum_dra2: 0.0,
            sum_ddec2: 0.0,
            sum_along2: 0.0,
            sum_cross2: 0.0,
        }
    }
    fn rms(&self, s: f64) -> Option<f64> {
        if self.n == 0 {
            None
        } else {
            Some((s / self.n as f64).sqrt())
        }
    }
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

fn vec_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn vec_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
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

fn unit_to_icrs_deg(u: [f64; 3]) -> Option<(f64, f64)> {
    if !(u[0].is_finite() && u[1].is_finite() && u[2].is_finite()) {
        return None;
    }
    let p = (u[0] * u[0] + u[1] * u[1]).sqrt();
    if !p.is_finite() {
        return None;
    }
    let ra = u[1].atan2(u[0]).to_degrees().rem_euclid(360.0);
    let dec = u[2].atan2(p).to_degrees();
    Some((ra, dec))
}

fn separation_rad(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let na = vec_len(a)?;
    let nb = vec_len(b)?;
    let c = (a[0] * b[0] + a[1] * b[1] + a[2] * b[2]) / (na * nb);
    let s = c.clamp(-1.0, 1.0).acos();
    if s.is_finite() {
        Some(s)
    } else {
        None
    }
}

struct Fold {
    unit: [f64; 3],
}

fn roemer_fold(
    station: [f64; 3],
    tdb: f64,
    worldline: &dyn Fn(f64) -> Option<[f64; 3]>,
) -> Option<Fold> {
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
    let apparent = worldline(emitted)?;
    let unit = toward_unit(station, apparent)?;
    Some(Fold { unit })
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
            println!("uranus-riss {word}: {path} bin void — absent on disk and the CDN fetch returned non-200");
            return Line { word, map: None };
        };
        let Some(eph) = parse_ephemeris_binary(&bytes) else {
            println!("uranus-riss {word}: {path} reads but does not parse to a BodyEphemeris");
            return Line { word, map: None };
        };
        map.insert(name.to_string(), eph);
    }
    Line {
        word,
        map: Some(map),
    }
}

fn predict(line: &Line, tdb: f64) -> Option<Predict> {
    let map = line.map.as_ref()?;
    let station = body_barycenter_position("earth", tdb, map)?;
    let fold = roemer_fold(station, tdb, &|t| {
        body_barycenter_position("uranus", t, map)
    })?;
    let vel = body_barycenter_velocity("uranus", tdb, map)?;
    Some(Predict {
        unit: fold.unit,
        vel,
    })
}

fn wrap_delta_deg(d: f64) -> f64 {
    let mut r = d.rem_euclid(360.0);
    if r > 180.0 {
        r -= 360.0;
    }
    r
}

fn parse_obs(text: &str) -> Vec<Obs> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with('-') {
            continue;
        }
        let fields: Vec<&str> = t.split('\t').map(|f| f.trim()).collect();
        if fields.len() < 6 {
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
        let sat = fields.get(5).copied().unwrap_or("").to_string();
        out.push(Obs {
            jd_utc,
            ra_deg,
            dec_deg,
            e_ra_mas,
            e_dec_mas,
            sat,
        });
    }
    out
}

fn sky_basis(n: [f64; 3], vel: [f64; 3]) -> Option<([f64; 3], [f64; 3])> {
    let vn = vec_dot(vel, n);
    let vt = [vel[0] - vn * n[0], vel[1] - vn * n[1], vel[2] - vn * n[2]];
    let along = {
        let m = vec_len(vt)?;
        [vt[0] / m, vt[1] / m, vt[2] / m]
    };
    let cross = vec_cross(n, along);
    Some((along, cross))
}

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn usage() {
    println!(
        "usage: uranus_riss_schiedsspruch_probe [--tsv <camargo_uranu_j.tsv>] [--eph-dir <data-root>] [--report-dir <dir>]"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Uranus-Riss-Schiedsspruch — DE441/INPOP19a/EPM2021 against the Camargo+2015 Uranus astrometry (geocentric, ICRS, TDB).");

    let tsv_path = arg_token(&args, "--tsv")
        .unwrap_or("data/vizier.cfa.harvard.edu/camargo_uranu_j.tsv".to_string());
    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let report_dir = arg_token(&args, "--report-dir").unwrap_or("state/reports".to_string());

    let Some(lsk) = embedded_lsk() else {
        eprintln!("uranus-riss: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
        return;
    };

    let tsv_text = match std::fs::read_to_string(&tsv_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("uranus-riss: {tsv_path} reads void — {e}");
            usage();
            return;
        }
    };
    let obs = parse_obs(&tsv_text);
    if obs.is_empty() {
        eprintln!("uranus-riss: {tsv_path} carries no observation rows (0 honored)");
        return;
    }

    let sigma_sum: f64 = obs
        .iter()
        .map(|o| (o.e_ra_mas * o.e_ra_mas + o.e_dec_mas * o.e_dec_mas).sqrt())
        .sum();
    let sigma_mean = sigma_sum / obs.len() as f64;
    let x_mas = 2.0 * sigma_mean;

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

    let mut accs = [Acc::new(), Acc::new(), Acc::new()];
    let mut div_sum = 0.0;
    let mut div_count = 0usize;
    let mut div_peak = 0.0f64;
    let mut div_peak_jd = 0.0f64;
    let mut series = String::new();
    series.push_str(
        "jd_utc\ttdb_secs\tra_deg\tdec_deg\te_ra_mas\te_dec_mas\tsat\tsep_de441_mas\tsep_inpop19a_mas\tsep_epm2021_mas\tdiv_max_mas\tdra_de441_mas\tddec_de441_mas\talong_de441_mas\tcross_de441_mas\n",
    );

    for o in &obs {
        let unix = (o.jd_utc - UNIX_JD_OFFSET) * 86400.0;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let mut preds: [Option<Predict>; 3] = [None, None, None];
        for (i, line) in lines.iter().enumerate() {
            preds[i] = predict(line, tdb);
        }
        let obs_unit = icrs_unit(o.ra_deg, o.dec_deg);

        let mut seps = [None::<f64>; 3];
        for i in 0..3 {
            if let Some(p) = &preds[i] {
                seps[i] = separation_rad(obs_unit, p.unit).map(|r| r * MAS_PER_RAD);
            }
        }

        let mut div_max: Option<f64> = None;
        for a in 0..3 {
            for b in (a + 1)..3 {
                if let (Some(pa), Some(pb)) = (&preds[a], &preds[b]) {
                    if let Some(d) = separation_rad(pa.unit, pb.unit) {
                        let m = d * MAS_PER_RAD;
                        div_max = Some(match div_max {
                            Some(cur) => cur.max(m),
                            None => m,
                        });
                    }
                }
            }
        }
        if let Some(m) = div_max {
            div_sum += m;
            div_count += 1;
            if m > div_peak {
                div_peak = m;
                div_peak_jd = o.jd_utc;
            }
        }

        let mut dra: [Option<f64>; 3] = [None, None, None];
        let mut ddec: [Option<f64>; 3] = [None, None, None];
        let mut along: [Option<f64>; 3] = [None, None, None];
        let mut cross: [Option<f64>; 3] = [None, None, None];
        for i in 0..3 {
            if let Some(p) = &preds[i] {
                if let Some((pra, pdec)) = unit_to_icrs_deg(p.unit) {
                    let cosdec = o.dec_deg.to_radians().cos();
                    dra[i] = Some(wrap_delta_deg(pra - o.ra_deg) * cosdec * 3600.0 * 1000.0);
                    ddec[i] = Some((pdec - o.dec_deg) * 3600.0 * 1000.0);
                }
                if let Some((a, c)) = sky_basis(p.unit, p.vel) {
                    along[i] = Some(vec_dot(obs_unit, a) * MAS_PER_RAD);
                    cross[i] = Some(vec_dot(obs_unit, c) * MAS_PER_RAD);
                }
            }
        }

        for i in 0..3 {
            match seps[i] {
                Some(s) => {
                    let a = &mut accs[i];
                    a.n += 1;
                    a.sum_sep2 += s * s;
                    if let Some(v) = dra[i] {
                        a.sum_dra2 += v * v;
                    }
                    if let Some(v) = ddec[i] {
                        a.sum_ddec2 += v * v;
                    }
                    if let Some(v) = along[i] {
                        a.sum_along2 += v * v;
                    }
                    if let Some(v) = cross[i] {
                        a.sum_cross2 += v * v;
                    }
                }
                None => {
                    accs[i].absent += 1;
                }
            }
        }

        let f = |v: Option<f64>| match v {
            Some(x) => format!("{x:.4}"),
            None => "na".to_string(),
        };
        series.push_str(&format!(
            "{:.8}\t{:.3}\t{:.7}\t{:.7}\t{:.1}\t{:.1}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            o.jd_utc,
            tdb,
            o.ra_deg,
            o.dec_deg,
            o.e_ra_mas,
            o.e_dec_mas,
            o.sat,
            f(seps[0]),
            f(seps[1]),
            f(seps[2]),
            f(div_max),
            f(dra[0]),
            f(ddec[0]),
            f(along[0]),
            f(cross[0]),
        ));
    }

    let mut report = String::new();
    report.push_str("Uranus-Riss-Schiedsspruch — residual probe\n\n");
    report.push_str(&format!(
        "Observations: {} rows from {tsv_path} (Camargo+ 2015, A&A 582 A8, uranu_j)\n",
        obs.len()
    ));
    report.push_str(&format!(
        "Dispute-site threshold X = 2·⟨σ⟩ = {x_mas:.1} mas (⟨σ⟩ = {sigma_mean:.1} mas, derived from the per-row errors before the residuals)\n"
    ));
    report.push_str(&format!(
        "Observatory (provenance): Pico dos Dias (MPC 874), λ = {OBS_LON_DEG}°, φ = {OBS_LAT_DEG}°, h = {OBS_ALT_M} m\n"
    ));
    report.push_str("Convention: geocentric astrometric (ICRS, light-time, no aberration/deflection), unweighted RMS\n");
    report.push_str("Convention measurement: the uranu_j position is derived-geocentric, not topocentric — against Horizons (geocentric, DE441) the topocentric approach lies ~0.45″ (parallax) off; the geocentric one holds\n");
    report.push_str("Time: the bins are TDB-bound; unix_to_tdb (LSK/naif0012) returns TT — the TDB−TT wobble (sub-ms, pending) carries ~40 m = ~3 mas at Uranus, below the data errors\n");
    report.push_str("Barycenter confound: the bins carry the Uranus-system barycenter (SPK 7), uranu_j the planet center (via ura111) — the offset (time-varying, ~518 km = ~37 mas at 2011) sits common-mode in all three lines\n");
    report.push_str("Confound: the Uranus position is not observed — derived from satellite astrometry on a DE432+ura111 carrier (mean satellite offset ~±30-60 mas); the observed root are the satellite positions, not Uranus\n\n");

    report.push_str("RMS per ephemeris (unweighted, mas):\n");
    let mut rms_list: Vec<(&str, f64)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let a = &accs[i];
        let rms = a.rms(a.sum_sep2);
        match rms {
            Some(r) => {
                rms_list.push((line.word, r));
                let r_dra = a.rms(a.sum_dra2);
                let r_ddec = a.rms(a.sum_ddec2);
                let r_along = a.rms(a.sum_along2);
                let r_cross = a.rms(a.sum_cross2);
                let d = |v: Option<f64>| match v {
                    Some(x) => format!("{x:.1}"),
                    None => "na".to_string(),
                };
                report.push_str(&format!(
                    "  {} : sep {:.1} | ΔRA·cosδ {} | ΔDec {} | along {} | cross {} ({} epochs, {} absent)\n",
                    line.word,
                    r,
                    d(r_dra),
                    d(r_ddec),
                    d(r_along),
                    d(r_cross),
                    a.n,
                    a.absent
                ));
            }
            None => {
                report.push_str(&format!(
                    "  {} : absent — no epoch carried a prediction\n",
                    line.word
                ));
            }
        }
    }

    report.push('\n');
    report.push_str(&format!(
        "Dispute site (pairwise DE-INPOP-EPM divergence at the observation site): peak {div_peak:.1} mas at JD {div_peak_jd:.4}, mean {:.1} mas over {div_count} epochs\n",
        div_sum / div_count as f64
    ));
    let mut sorted = rms_list.clone();
    sorted.sort_by(|a, b| a.1.total_cmp(&b.1));
    match sorted.as_slice() {
        [best, second, third] => {
            let delta = second.1 - best.1;
            let worst = third.1;
            report.push_str(&format!(
                "Verdict: X = {x_mas:.1} mas, RMS(best) = {:.1} ({}) mas, RMS(second) = {:.1} ({}) mas, ΔRMS = {:.1} mas\n",
                best.1, best.0, second.1, second.0, delta
            ));
            if delta > x_mas {
                report.push_str(&format!(
                    "(i) the observations arbitrate — ephemeris {} carries the observations closer (unweighted RMS {:.1} mas against {:.1} mas)\n",
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
            report.push_str("Verdict: absent — fewer than three ephemeris lines carried an RMS\n");
        }
    }

    print!("{report}");
    if let Err(e) = std::fs::create_dir_all(&report_dir) {
        eprintln!("uranus-riss: report dir {report_dir} stays unbuilt — {e}");
    }
    let report_path = format!("{report_dir}/uranus_riss_schiedsspruch.txt");
    let series_path = format!("{report_dir}/uranus_riss_schiedsspruch_series.tsv");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("uranus-riss: report write void — {e}");
    }
    if let Err(e) = std::fs::write(&series_path, &series) {
        eprintln!("uranus-riss: series write void — {e}");
    }
    println!("series: {series_path}");
    println!("report: {report_path}");
}
