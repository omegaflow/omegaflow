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

struct Acc {
    n: usize,
    sum_sep2: f64,
}

struct VecAcc {
    n: usize,
    dra: f64,
    ddec: f64,
}

impl VecAcc {
    fn new() -> VecAcc {
        VecAcc {
            n: 0,
            dra: 0.0,
            ddec: 0.0,
        }
    }
    fn mean(&self) -> Option<(f64, f64)> {
        if self.n == 0 {
            None
        } else {
            Some((self.dra / self.n as f64, self.ddec / self.n as f64))
        }
    }
}

impl Acc {
    fn new() -> Acc {
        Acc {
            n: 0,
            sum_sep2: 0.0,
        }
    }
    fn rms(&self) -> Option<f64> {
        if self.n == 0 {
            None
        } else {
            Some((self.sum_sep2 / self.n as f64).sqrt())
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

fn wrap_delta_deg(d: f64) -> f64 {
    let mut r = d.rem_euclid(360.0);
    if r > 180.0 {
        r -= 360.0;
    }
    r
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

fn roemer_fold(
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
    let apparent = worldline(emitted)?;
    toward_unit(station, apparent)
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
            println!("uranus-sat {word}: {path} bin void — absent on disk and the CDN fetch returned non-200");
            return Line { word, map: None };
        };
        let Some(eph) = parse_ephemeris_binary(&bytes) else {
            println!("uranus-sat {word}: {path} reads but does not parse to a BodyEphemeris");
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
        println!("uranus-sat {name} ({id}): no segment covers it");
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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Uranus-satellite Schiedsspruch — the five observed moons against DE441/INPOP19a/EPM2021 (moon-orbit SPK, geocentric astrometric).");

    let tsv_dir =
        arg_token(&args, "--tsv-dir").unwrap_or("data/vizier.cfa.harvard.edu".to_string());
    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let spk_dir = arg_token(&args, "--spk-dir").unwrap_or("data/naif.jpl.nasa.gov".to_string());
    let spk_name = arg_token(&args, "--spk").unwrap_or("ura111.bsp".to_string());
    let report_dir = arg_token(&args, "--report-dir").unwrap_or("state/reports".to_string());

    let Some(lsk) = embedded_lsk() else {
        eprintln!("uranus-sat: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
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
            eprintln!("uranus-sat: {spk_path} opens void — {e:?}");
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
                println!("uranus-sat {name}: {tsv_path} reads void — {e}");
                continue;
            }
        };
        let obs = parse_obs(&text);
        println!("uranus-sat {name}: {} rows", obs.len());
        sats.push((name.to_string(), sat, obs));
    }
    if sats.is_empty() {
        eprintln!("uranus-sat: no satellite carried both an SPK and observation rows (0 honored)");
        return;
    }

    let mut all_obs: Vec<f64> = Vec::new();
    for (_, _, obs) in &sats {
        for o in obs {
            all_obs.push((o.e_ra_mas * o.e_ra_mas + o.e_dec_mas * o.e_dec_mas).sqrt());
        }
    }
    let sigma_mean = all_obs.iter().sum::<f64>() / all_obs.len() as f64;
    let x_mas = 2.0 * sigma_mean;

    let mut accs = [Acc::new(), Acc::new(), Acc::new()];
    let mut vaccs = [VecAcc::new(), VecAcc::new(), VecAcc::new()];
    let mut per_sat: Vec<(String, [Acc; 3])> = Vec::new();
    let mut series = String::new();
    series.push_str(
        "sat\tjd_utc\ttdb_secs\tra_deg\tdec_deg\te_ra_mas\te_dec_mas\tsep_de441_mas\tsep_inpop19a_mas\tsep_epm2021_mas\n",
    );

    for (name, sat, obs) in &sats {
        let mut pacc = [Acc::new(), Acc::new(), Acc::new()];
        for o in obs {
            let unix = (o.jd_utc - UNIX_JD_OFFSET) * 86400.0;
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            let obs_unit = icrs_unit(o.ra_deg, o.dec_deg);
            let cosdec = o.dec_deg.to_radians().cos();
            let mut seps = [None::<f64>; 3];
            let mut dra: [Option<f64>; 3] = [None, None, None];
            let mut ddec: [Option<f64>; 3] = [None, None, None];
            for (i, line) in lines.iter().enumerate() {
                let Some(map) = line.map.as_ref() else {
                    continue;
                };
                let Some(station) = body_barycenter_position("earth", tdb, map) else {
                    continue;
                };
                let Some(unit) = roemer_fold(station, tdb, &|t| sat_worldline(sat, &spk, map, t))
                else {
                    continue;
                };
                seps[i] = separation_rad(obs_unit, unit).map(|r| r * MAS_PER_RAD);
                if let Some((pra, pdec)) = unit_to_icrs_deg(unit) {
                    dra[i] = Some(wrap_delta_deg(pra - o.ra_deg) * cosdec * 3600.0 * 1000.0);
                    ddec[i] = Some((pdec - o.dec_deg) * 3600.0 * 1000.0);
                }
            }
            for i in 0..3 {
                if let Some(s) = seps[i] {
                    accs[i].n += 1;
                    accs[i].sum_sep2 += s * s;
                    pacc[i].n += 1;
                    pacc[i].sum_sep2 += s * s;
                }
                if let (Some(dra_v), Some(ddec_v)) = (dra[i], ddec[i]) {
                    vaccs[i].n += 1;
                    vaccs[i].dra += dra_v;
                    vaccs[i].ddec += ddec_v;
                }
            }
            let f = |v: Option<f64>| match v {
                Some(x) => format!("{x:.4}"),
                None => "na".to_string(),
            };
            series.push_str(&format!(
                "{name}\t{:.8}\t{:.3}\t{:.7}\t{:.7}\t{:.1}\t{:.1}\t{}\t{}\t{}\n",
                o.jd_utc,
                tdb,
                o.ra_deg,
                o.dec_deg,
                o.e_ra_mas,
                o.e_dec_mas,
                f(seps[0]),
                f(seps[1]),
                f(seps[2]),
            ));
        }
        per_sat.push((name.clone(), pacc));
    }

    let mut report = String::new();
    report.push_str("Uranus-satellite Schiedsspruch — residual probe\n\n");
    report.push_str(&format!(
        "Satellites: {} tables (Camargo+ 2015, A&A 582 A8, ariel_j/umbri_j/titan_j/obero_j/miran_j), {spk_name} SPK orbits\n",
        sats.len()
    ));
    report.push_str(&format!(
        "Dispute-site threshold X = 2·⟨σ⟩ = {x_mas:.1} mas (⟨σ⟩ = {sigma_mean:.1} mas from the per-row errors)\n"
    ));
    report.push_str(&format!(
        "Observatory: Pico dos Dias (MPC 874), λ = {OBS_LON_DEG}°, φ = {OBS_LAT_DEG}°, h = {OBS_ALT_M} m — geocentric astrometric (ICRS, light-time, no aberration/deflection)\n"
    ));
    report.push_str("Reduction: satellite = planet barycenter (DE441/INPOP19a/EPM2021) + (satellite − barycenter) (the SPK moon model, common to all three lines)\n\n");

    report.push_str("RMS per ephemeris (unweighted, mas, all satellites):\n");
    let mut rms_list: Vec<(&str, f64)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        match accs[i].rms() {
            Some(r) => {
                rms_list.push((line.word, r));
                report.push_str(&format!(
                    "  {} : sep {:.1} ({} epochs)\n",
                    line.word, r, accs[i].n
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

    report.push_str("\nRMS per satellite (unweighted, mas):\n");
    for (name, pacc) in &per_sat {
        let mut parts: Vec<String> = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            match pacc[i].rms() {
                Some(r) => parts.push(format!("{} {:.1}", line.word, r)),
                None => parts.push(format!("{} na", line.word)),
            }
        }
        report.push_str(&format!("  {name:10} : {}\n", parts.join(" | ")));
    }

    report.push_str("\nMean residual vector per ephemeris (mas, all satellites — the moon-model error and the frame offset are common to all three, so they cancel in the differences):\n");
    let mut means: Vec<(&str, f64, f64)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        match vaccs[i].mean() {
            Some((mra, mdec)) => {
                means.push((line.word, mra, mdec));
                report.push_str(&format!(
                    "  {} : ΔRA·cosδ {mra:+.1} mas, ΔDec {mdec:+.1} mas ({n} epochs)\n",
                    line.word,
                    n = vaccs[i].n
                ));
            }
            None => {
                report.push_str(&format!("  {} : absent — no residual vector\n", line.word));
            }
        }
    }
    report.push_str("Pairwise mean-residual differences (mas — the observed Riss, the moon-model error cancels):\n");
    for a in 0..means.len() {
        for b in (a + 1)..means.len() {
            let (wa, mra, mdec) = means[a];
            let (wb, brra, brdec) = means[b];
            let dra = mra - brra;
            let ddec = mdec - brdec;
            let mag = (dra * dra + ddec * ddec).sqrt();
            report.push_str(&format!(
                "  {wa} − {wb} : ΔRA·cosδ {dra:+.1} mas, ΔDec {ddec:+.1} mas, magnitude {mag:.1} mas\n"
            ));
        }
    }

    report.push('\n');
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
            report.push_str("Verdict: absent — fewer than three ephemeris lines carried an RMS\n");
        }
    }

    print!("{report}");
    if let Err(e) = std::fs::create_dir_all(&report_dir) {
        eprintln!("uranus-sat: report dir {report_dir} stays unbuilt — {e}");
    }
    let report_path = format!("{report_dir}/uranus_satellite_schiedsspruch.txt");
    let series_path = format!("{report_dir}/uranus_satellite_schiedsspruch_series.tsv");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("uranus-sat: report write void — {e}");
    }
    if let Err(e) = std::fs::write(&series_path, &series) {
        eprintln!("uranus-sat: series write void — {e}");
    }
    println!("series: {series_path}");
    println!("report: {report_path}");
}
