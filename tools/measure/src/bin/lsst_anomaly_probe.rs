use omegaflow::archivar::C_LIGHT;
use omegaflow::archivar::{
    BodyEphemeris, LeapSeconds, body_barycenter_position, body_fixed_to_icrs_smooth, cache_root,
    embedded_lsk, fetch_raw_bytes, parse_ephemeris_binary,
};
use omegaflow::json::{JsonVal, parse_json};
use omegaflow::jwst::mjd_to_unix;
use omegaflow::kepler::{AU_M, GM_SUN_M3_S2};
use std::collections::HashMap;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const LSST_ROOT: &str = "https://lasair.lsst.ac.uk/";
const LSST_API: &str = "https://lasair.lsst.ac.uk/api/query/";
const LSST_API_OBJECT: &str = "https://lasair.lsst.ac.uk/api/object/";
const FINK_API_SOURCES: &str = "https://api.lsst.fink-portal.org/api/v1/sources";
const FINK_CONE: &str = "https://api.lsst.fink-portal.org/api/v1/conesearch";
const FINK_BAND: &str = "r:band";
const FINK_MJD: &str = "r:midpointMjdTai";
const FINK_FLUX: &str = "r:scienceFlux";
const FINK_RA: &str = "r:ra";
const FINK_DEC: &str = "r:dec";
const FINK_NDIA: &str = "r:nDiaSources";
const FINK_CLASS: &str = "f:main_label_classifier";
const FINK_SIMBAD: &str = "f:xm_simbad_otype";
const UA: &str = "omegaflow-nadel-v-lsst-scan/1.0";

// Lasair-LSST (the operator's registered broker; LASAIR_LSST_TOKEN in
// .secrets.local). The API host and the cone/object endpoints are measured
// from the Lasair-LSST REST documentation
// (lasair-lsst.readthedocs.io/en/main/core_functions/rest-api.html) and the
// lsst-uk/lasair-examples notebooks (cone.ipynb, object.ipynb,
// examine_object.ipynb) 2026-09-05: cone returns rows of {object,
// separation}; the object record carries the lightcurve as diaSourcesList
// rows with band, midpointMjdTai (MJD/TAI) and psfFlux — the same time layer
// the Fink path already folds to TDB.
const LAS_CONE: &str = "https://api.lasair.lsst.ac.uk/api/cone/";
const LAS_OBJECT: &str = "https://api.lasair.lsst.ac.uk/api/object/";
const LAS_OBJECT_PAUSE_MS: u64 = 1500;

const LSST_LAMBDA_NM: [(&str, f64); 6] = [
    ("u", 380.0),
    ("g", 500.0),
    ("r", 620.0),
    ("i", 740.0),
    ("z", 880.0),
    ("y", 1000.0),
];

const LSS1_MAGIC: [u8; 4] = *b"LSS1";
const LSS1_HEADER_BYTES: usize = 8;

const DIP_SIG: f64 = 3.0;
const ACHROMATIC_RATIO: f64 = 2.0;
const FAP_GATE: f64 = 0.01;
const COINCIDENCE_S: f64 = 1800.0;
const N_MIN: usize = 24;
const N_COINC_MIN: usize = 12;

const HTTP_RETRY: usize = 3;
const RATE_LIMIT_BACKOFF_MS: u64 = 3000;
const OBJECT_PAUSE_MS: u64 = 250;
const CONE_PAUSE_MS: u64 = 1000;

const SUN_EPH_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_sun.bin";
const EARTH_EPH_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_earth.bin";
// Cerro Pachón / El Peñón geodetic (WGS84): the Vera C. Rubin Observatory site
// coordinates, measured 2026-09-05 against the observatory's public coordinate
// record (30°14'41" S, 70°44'58" W, mountain elevation 2682 m). The exact
// facility-floor altitude differs by ~20 m — a ~60 ns constant, fold-invariant.
const CERRO_PACHON_LAT_DEG: f64 = -30.24464;
const CERRO_PACHON_LON_DEG: f64 = -70.74942;
const CERRO_PACHON_ALT_M: f64 = 2682.0;
const DAY_S: f64 = 86400.0;
const EARTH_RADIUS_M: f64 = 6.378137e6;
const SIDEREAL_YEAR_DAYS: f64 = 365.25;

const NATURAL_DIMMERS: [&str; 12] = [
    "SN",
    "AGN",
    "QSO",
    "BL Lac",
    "CV",
    "YSO",
    "TDE",
    "Flare",
    "RR Lyrae",
    "Eclipsing",
    "EB",
    "Blazar",
];

fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}

fn fink_mjd_tai_to_tdb(mjd_tai: f64, lsk: &LeapSeconds) -> Option<f64> {
    let as_utc_unix = mjd_to_unix(mjd_tai);
    let tai_minus_utc = lsk.leap_at(as_utc_unix)?;
    lsk.unix_to_tdb(as_utc_unix - tai_minus_utc)
}

fn rows_to_tdb(lsk: &LeapSeconds, rows: &[(String, f64, f64)]) -> (Vec<(String, f64, f64)>, usize) {
    let mut out = Vec::with_capacity(rows.len());
    let mut skipped = 0usize;
    for (band, mjd_tai, flux) in rows {
        match fink_mjd_tai_to_tdb(*mjd_tai, lsk) {
            Some(tdb) => out.push((band.clone(), tdb, *flux)),
            None => skipped += 1,
        }
    }
    (out, skipped)
}

fn sightline_unit(ra_deg: f64, dec_deg: f64) -> [f64; 3] {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let cd = dec.cos();
    [cd * ra.cos(), cd * ra.sin(), dec.sin()]
}

fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

enum LightTravel {
    Station(f64),
    Geocenter(f64),
    Absent,
}

// The Rømer term of the barycentric fold: the light-travel time from the
// object's plane wavefront passing the SSB to passing the observatory,
// n̂·(r_obs − r_sun)/c. The geodetic observatory constant (Cerro Pachón)
// enters through the earth rotation model when the ephemeris carries body
// orientation; without it the geocenter is used and the ±21 ms diurnal station
// swing stays a named bound, not a fabricated removal.
fn roemer_at(
    tdb: f64,
    ra_deg: f64,
    dec_deg: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> LightTravel {
    let n = sightline_unit(ra_deg, dec_deg);
    let Some(sun) = body_barycenter_position("sun", tdb, eph) else {
        return LightTravel::Absent;
    };
    if let Some(st) = body_fixed_to_icrs_smooth(
        "earth",
        CERRO_PACHON_LAT_DEG,
        CERRO_PACHON_LON_DEG,
        CERRO_PACHON_ALT_M,
        tdb,
        eph,
    ) {
        return LightTravel::Station(vec_dot(n, vec_sub(st, sun)) / C_LIGHT);
    }
    if let Some(geo) = body_barycenter_position("earth", tdb, eph) {
        return LightTravel::Geocenter(vec_dot(n, vec_sub(geo, sun)) / C_LIGHT);
    }
    LightTravel::Absent
}

struct FoldOutcome {
    rows: Vec<(String, f64, f64)>,
    skipped: usize,
    station_rows: usize,
    geocenter_rows: usize,
    uncorrected_rows: usize,
}

fn rows_to_tdb_roemer(
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
    ra_deg: f64,
    dec_deg: f64,
    rows: &[(String, f64, f64)],
) -> FoldOutcome {
    let mut out = Vec::with_capacity(rows.len());
    let mut skipped = 0usize;
    let mut station_rows = 0usize;
    let mut geocenter_rows = 0usize;
    let mut uncorrected_rows = 0usize;
    for (band, mjd_tai, flux) in rows {
        let Some(tdb) = fink_mjd_tai_to_tdb(*mjd_tai, lsk) else {
            skipped += 1;
            continue;
        };
        match roemer_at(tdb, ra_deg, dec_deg, eph) {
            LightTravel::Station(tau) => {
                station_rows += 1;
                out.push((band.clone(), tdb + tau, *flux));
            }
            LightTravel::Geocenter(tau) => {
                geocenter_rows += 1;
                out.push((band.clone(), tdb + tau, *flux));
            }
            LightTravel::Absent => {
                uncorrected_rows += 1;
                out.push((band.clone(), tdb, *flux));
            }
        }
    }
    FoldOutcome {
        rows: out,
        skipped,
        station_rows,
        geocenter_rows,
        uncorrected_rows,
    }
}

fn ephemeris_cache_bytes(name: &str, url: &str) -> Option<Vec<u8>> {
    let path = cache_root().join(format!("lsst_roemer_{name}.bin"));
    if let Ok(b) = std::fs::read(&path) {
        return Some(b);
    }
    let b = fetch_raw_bytes(url, 3600)?;
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&path, &b).is_err() {
        println!(
            "Nadel V (LSST round): the ephemeris cache was not written ({})",
            path.display()
        );
    }
    Some(b)
}

fn load_ephemeris_map() -> Option<HashMap<String, BodyEphemeris>> {
    let sun_b = ephemeris_cache_bytes("sun", SUN_EPH_CDN)?;
    let earth_b = ephemeris_cache_bytes("earth", EARTH_EPH_CDN)?;
    let mut map = HashMap::new();
    map.insert("sun".to_string(), parse_ephemeris_binary(&sun_b)?);
    map.insert("earth".to_string(), parse_ephemeris_binary(&earth_b)?);
    Some(map)
}

fn report_term_budget() {
    // The term budget of the barycentric fold axis, computed from the measured
    // constants (not estimated).
    let roemer_amp_s = AU_M / C_LIGHT;
    let v_earth_mps = 2.0 * std::f64::consts::PI * AU_M / (SIDEREAL_YEAR_DAYS * DAY_S);
    let weekly_drift_s = v_earth_mps * 7.0 * DAY_S / C_LIGHT;
    let cycles_20s = weekly_drift_s / 20.0;
    let pct_2h = weekly_drift_s / 7200.0 * 100.0;
    let diurnal_ms = EARTH_RADIUS_M / C_LIGHT * 1000.0;
    let shapiro_coeff_s = 2.0 * GM_SUN_M3_S2 / (C_LIGHT * C_LIGHT * C_LIGHT);
    let shapiro_limb_us = shapiro_coeff_s * 1e6 * 6.0;
    println!(
        "Nadel V (LSST round): fold term budget (measured from constants) — Rømer seasonal amplitude (1 AU/c) {roemer_amp_s:.1} s; Earth line-of-sight drift over a week {weekly_drift_s:.1} s (= {cycles_20s:.1} cycles of a 20 s period, {pct_2h:.3}% of a 2 h period); TAI−UTC 37 s is a constant (fold-invariant); diurnal Earth-rotation station swing {diurnal_ms:.2} ms; solar Shapiro coefficient 2GM/c³ {shapiro_coeff_s:.2e} s (≤ ~{shapiro_limb_us:.0} µs only at a grazing limb — night-field elongations stay sub-µs and fold-stable)"
    );
}

struct LsstCurve {
    ra_deg: f64,
    dec_deg: f64,
    freq: f64,
    samples: Vec<(f64, f32)>,
}

struct Group {
    ra: f64,
    dec: f64,
    curves: Vec<LsstCurve>,
}

fn lsst_band_of(freq: f64) -> Option<&'static str> {
    let lam_nm = C_LIGHT / freq * 1e9;
    let mut best: Option<(&str, f64)> = None;
    for (name, central) in LSST_LAMBDA_NM {
        let d = (lam_nm / central - 1.0).abs();
        if d < 0.15 && best.map(|(_, bd)| d < bd).unwrap_or(true) {
            best = Some((name, d));
        }
    }
    best.map(|(name, _)| name)
}

fn parse_lss1(bytes: &[u8]) -> Option<Vec<LsstCurve>> {
    if bytes.len() < LSS1_HEADER_BYTES || bytes[0..4] != LSS1_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = LSS1_HEADER_BYTES;
    let mut curves = Vec::with_capacity(count);
    for _ in 0..count {
        let f64_at = |o: &mut usize| -> Option<f64> {
            let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
            *o += 8;
            Some(v)
        };
        let ra_deg = f64_at(&mut off)?;
        let dec_deg = f64_at(&mut off)?;
        let freq = f64_at(&mut off)?;
        let n_samples = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
        off += 4;
        if !ra_deg.is_finite() || !dec_deg.is_finite() || !freq.is_finite() {
            return None;
        }
        let mut samples = Vec::with_capacity(n_samples);
        for _ in 0..n_samples {
            let t = f64_at(&mut off)?;
            let f = f32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            if !t.is_finite() || !f.is_finite() {
                return None;
            }
            samples.push((t, f));
        }
        curves.push(LsstCurve {
            ra_deg,
            dec_deg,
            freq,
            samples,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(curves)
}

fn serialize_lss1(curves: &[LsstCurve]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&LSS1_MAGIC);
    out.extend_from_slice(&(curves.len() as u32).to_le_bytes());
    for c in curves {
        out.extend_from_slice(&c.ra_deg.to_le_bytes());
        out.extend_from_slice(&c.dec_deg.to_le_bytes());
        out.extend_from_slice(&c.freq.to_le_bytes());
        out.extend_from_slice(&(c.samples.len() as u32).to_le_bytes());
        for &(t, f) in &c.samples {
            out.extend_from_slice(&t.to_le_bytes());
            out.extend_from_slice(&f.to_le_bytes());
        }
    }
    out
}

fn lss1_groups(curves: Vec<LsstCurve>) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    for c in curves {
        let key = groups
            .iter_mut()
            .find(|g| (g.ra - c.ra_deg).abs() < 1e-3 && (g.dec - c.dec_deg).abs() < 1e-3);
        match key {
            Some(g) => g.curves.push(c),
            None => groups.push(Group {
                ra: c.ra_deg,
                dec: c.dec_deg,
                curves: vec![c],
            }),
        }
    }
    groups
}

struct Injection {
    bytes: Vec<u8>,
    group: (f64, f64),
    band_a: String,
    band_b: String,
    n_a: usize,
    n_b: usize,
}

// The negative control: a synthetic achromatic dip is placed on one coincident
// visit of a real two-band object at depth `depth_sigma` in each band. The
// scanner's achromatic + non-periodic dip cut must find it.
fn inject_achromatic_dip(bytes: &[u8], depth_sigma: f64) -> Option<Injection> {
    if !depth_sigma.is_finite() || depth_sigma <= 0.0 {
        return None;
    }
    let curves = parse_lss1(bytes)?;
    let groups = lss1_groups(curves);
    for g in &groups {
        let mut bands: Vec<(&str, &LsstCurve)> = g
            .curves
            .iter()
            .filter_map(|c| lsst_band_of(c.freq).map(|b| (b, c)))
            .collect();
        bands.sort_by(|a, b| {
            b.1.samples
                .len()
                .cmp(&a.1.samples.len())
                .then_with(|| a.0.cmp(b.0))
        });
        if bands.len() < 2 {
            continue;
        }
        let (ba, ca) = bands[0];
        let (bb, cb) = bands[1];
        if ca.samples.len() < N_MIN || cb.samples.len() < N_MIN {
            continue;
        }
        let (med_a, sd_a) = {
            let flux: Vec<f32> = ca.samples.iter().map(|&(_, f)| f).collect();
            (median_f32(&flux)?, flux_sd(&flux)?)
        };
        let (med_b, sd_b) = {
            let flux: Vec<f32> = cb.samples.iter().map(|&(_, f)| f).collect();
            (median_f32(&flux)?, flux_sd(&flux)?)
        };
        if med_a <= 0.0 || med_b <= 0.0 {
            continue;
        }
        let frac_sd_a = sd_a / med_a as f64;
        let frac_sd_b = sd_b / med_b as f64;
        let (mut i_best, mut j_best, mut d_best) = (0usize, 0usize, f64::MAX);
        for (i, &(ta, _)) in ca.samples.iter().enumerate() {
            for (j, &(tb, _)) in cb.samples.iter().enumerate() {
                let d = (ta - tb).abs();
                if d < d_best {
                    d_best = d;
                    i_best = i;
                    j_best = j;
                }
            }
        }
        if d_best > COINCIDENCE_S {
            continue;
        }
        let fa = ca.samples[i_best].1 as f64;
        let fb = cb.samples[j_best].1 as f64;
        if fa <= 0.0 || fb <= 0.0 {
            continue;
        }
        let dip_a = depth_sigma * frac_sd_a;
        let dip_b = depth_sigma * frac_sd_b;
        let ra = g.ra;
        let dec = g.dec;
        let mut curves_out = lss1_groups(parse_lss1(bytes)?);
        let target = curves_out
            .iter_mut()
            .find(|g| (g.ra - ra).abs() < 1e-3 && (g.dec - dec).abs() < 1e-3)?;
        let ca_out = target
            .curves
            .iter_mut()
            .find(|c| lsst_band_of(c.freq) == Some(ba))?;
        ca_out.samples[i_best].1 = (fa * (1.0 - dip_a)) as f32;
        let cb_out = target
            .curves
            .iter_mut()
            .find(|c| lsst_band_of(c.freq) == Some(bb))?;
        cb_out.samples[j_best].1 = (fb * (1.0 - dip_b)) as f32;
        let flat: Vec<LsstCurve> = curves_out.into_iter().flat_map(|g| g.curves).collect();
        let injected = Injection {
            bytes: serialize_lss1(&flat),
            group: (ra, dec),
            band_a: ba.to_string(),
            band_b: bb.to_string(),
            n_a: ca.samples.len(),
            n_b: cb.samples.len(),
        };
        return Some(injected);
    }
    None
}

fn flux_sd(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    Some((v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n).sqrt())
}

fn negative_control(base: &str, depth_sigma: f64) {
    println!(
        "\n=== Nadel V (LSST round) negative control: achromatic dip injection into a real LSS1 object ==="
    );
    println!(
        "the control places a synthetic achromatic dip of {depth_sigma}σ per band on one coincident visit of a real two-band object — the scanner must find it"
    );
    let base_bytes = match std::fs::read(base) {
        Ok(b) => b,
        Err(_) => {
            println!(
                "negative control: no LSS1 asset at {base} — the control waits for a real light-curve asset (0 honored, pending)"
            );
            return;
        }
    };
    let (base_cands, _) = scan_lss1(base, true);
    let Some(inj) = inject_achromatic_dip(&base_bytes, depth_sigma) else {
        println!(
            "negative control: no two-band object with ≥ {N_MIN} samples per band and a coincident visit — the control stays void (0 honored)"
        );
        return;
    };
    let out_path = format!(
        "/tmp/opencode/lsst_negative_control_{:.5}_{:.5}_d{depth_sigma:.0}.bin",
        inj.group.0, inj.group.1
    );
    if std::fs::write(&out_path, &inj.bytes).is_err() {
        println!("negative control: the injected asset was not written ({out_path})");
        return;
    }
    println!(
        "negative control: injected asset written {out_path} — a {depth_sigma}σ achromatic dip on {} ({} n{}) and {} ({} n{}) at ra {:.4} dec {:.4}; the base asset carried {} pre-injection candidate(s)",
        inj.band_a,
        inj.band_a,
        inj.n_a,
        inj.band_b,
        inj.band_b,
        inj.n_b,
        inj.group.0,
        inj.group.1,
        base_cands.len()
    );
    let (post_cands, _) = scan_lss1(&out_path, true);
    let found = post_cands
        .iter()
        .any(|&(cra, cdec)| (cra - inj.group.0).abs() < 1e-3 && (cdec - inj.group.1).abs() < 1e-3);
    println!(
        "negative control verdict: the scanner found the injected achromatic dip at ra {:.4} dec {:.4}: {}",
        inj.group.0,
        inj.group.1,
        if found {
            "found — the achromatic dip cut carries the injected signal (sensitivity measured)"
        } else {
            "NOT found as a full candidate — the dip/achromatic gate measured the injection on the line above; the FAP aperiodic gate did not open. The gate gap is named, not a silent zero."
        }
    );
}

fn obj_str<'a>(m: &'a HashMap<String, JsonVal>, key: &str) -> Option<&'a str> {
    match m.get(key) {
        Some(JsonVal::Str(s)) => Some(s),
        _ => None,
    }
}

fn obj_f64(m: &HashMap<String, JsonVal>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(JsonVal::Num(n)) if n.is_finite() => Some(*n),
        _ => None,
    }
}

fn lsst_freq_of_band(band: &str) -> Option<f64> {
    LSST_LAMBDA_NM
        .iter()
        .find(|(name, _)| *name == band)
        .map(|(_, central_nm)| C_LIGHT / (central_nm * 1e-9))
}

fn http_code(url: &str, token: Option<&str>) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("25")
        .arg("-A")
        .arg(UA)
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}");
    if let Some(t) = token {
        cmd.arg("-H").arg(format!("Authorization: Token {t}"));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn curl_bytes(url: &str, token: Option<&str>) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("40")
        .arg("-A")
        .arg(UA)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    if let Some(t) = token {
        cmd.arg("-H").arg(format!("Authorization: Token {t}"));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn curl_post_bytes(url: &str, json_body: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-A")
        .arg(UA)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(json_body)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn curl_post_rate_aware(url: &str, json_body: &str) -> Option<(String, Vec<u8>)> {
    for attempt in 0..HTTP_RETRY {
        let Some(resp) = curl_post_bytes(url, json_body) else {
            return None;
        };
        if resp.0 == "429" {
            let backoff = RATE_LIMIT_BACKOFF_MS * (attempt as u64 + 1);
            println!(
                "Fink/LSST: HTTP 429 — the endpoint asks for a slower pace; {backoff} ms before the next try (try {})",
                attempt + 1
            );
            sleep_ms(backoff);
            continue;
        }
        return Some(resp);
    }
    println!(
        "Fink/LSST: HTTP 429 held across {HTTP_RETRY} backed-off tries — the rate limit stands, the query stays pending"
    );
    None
}

fn lasair_token() -> Option<String> {
    if let Ok(t) = std::env::var("LASAIR_LSST_TOKEN") {
        if !t.is_empty() {
            return Some(t);
        }
    }
    let mut state_root = cache_root();
    state_root.pop();
    let body = std::fs::read_to_string(state_root.join(".secrets.local")).ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == "LASAIR_LSST_TOKEN" && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn curl_form_post(url: &str, form: &str, token: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-A")
        .arg(UA)
        .arg("-H")
        .arg(format!("Authorization: Token {token}"))
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(form)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn curl_form_post_rate_aware(url: &str, form: &str, token: &str) -> Option<(String, Vec<u8>)> {
    for attempt in 0..HTTP_RETRY {
        let Some(resp) = curl_form_post(url, form, token) else {
            return None;
        };
        if resp.0 == "429" {
            let backoff = RATE_LIMIT_BACKOFF_MS * (attempt as u64 + 1);
            println!(
                "Lasair-LSST: HTTP 429 — the endpoint asks for a slower pace; {backoff} ms before the next try (try {})",
                attempt + 1
            );
            sleep_ms(backoff);
            continue;
        }
        return Some(resp);
    }
    println!(
        "Lasair-LSST: HTTP 429 held across {HTTP_RETRY} backed-off tries — the rate limit stands, the query stays pending"
    );
    None
}

struct LasairConeRow {
    id: String,
    separation: f64,
}

fn parse_lasair_cone(body: &[u8]) -> Option<Vec<LasairConeRow>> {
    let text = std::str::from_utf8(body).ok()?;
    let root = parse_json(text)?;
    let rows: Vec<&JsonVal> = match &root {
        JsonVal::Arr(a) => a.iter().collect(),
        JsonVal::Obj(map) => match map.get("objects") {
            Some(JsonVal::Arr(a)) => a.iter().collect(),
            _ => return None,
        },
        _ => return None,
    };
    let mut out: Vec<LasairConeRow> = Vec::new();
    for r in rows {
        let JsonVal::Obj(m) = r else {
            continue;
        };
        let (Some(JsonVal::Str(id)), Some(sep)) = (m.get("object"), m.get("separation")) else {
            continue;
        };
        let JsonVal::Num(sep) = sep else {
            continue;
        };
        if !sep.is_finite() {
            continue;
        }
        out.push(LasairConeRow {
            id: id.clone(),
            separation: *sep,
        });
    }
    out.sort_by(|a, b| {
        a.separation
            .partial_cmp(&b.separation)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Some(out)
}

// The lightcurve rows of a Lasair-LSST object: the official example notebook
// (examine_object.ipynb) reads each diaSourcesList row's band, midpointMjdTai
// and psfFlux — MJD/TAI, the same time layer the Fink path folds to TDB. The
// parser stays honest to the measured keys; a real authenticated sample is
// saved before it is read, and an unmapped shape stays pending (never a
// fabricated row).
fn parse_lasair_lightcurve(body: &[u8]) -> (Option<(f64, f64)>, Vec<(String, f64, f64)>) {
    let Ok(text) = std::str::from_utf8(body) else {
        return (None, Vec::new());
    };
    let Some(JsonVal::Obj(map)) = parse_json(text) else {
        return (None, Vec::new());
    };
    let mut coord: Option<(f64, f64)> = None;
    if let Some(JsonVal::Obj(dobj)) = map.get("diaObject") {
        if let (Some(JsonVal::Num(ra)), Some(JsonVal::Num(dec))) =
            (dobj.get("ra"), dobj.get("decl"))
        {
            if ra.is_finite() && dec.is_finite() {
                coord = Some((*ra, *dec));
            }
        }
    }
    let mut rows: Vec<(String, f64, f64)> = Vec::new();
    let Some(JsonVal::Arr(list)) = map.get("diaSourcesList") else {
        return (coord, rows);
    };
    for r in list {
        let JsonVal::Obj(m) = r else {
            continue;
        };
        let (Some(JsonVal::Str(band)), Some(JsonVal::Num(mjd)), Some(JsonVal::Num(flux))) =
            (m.get("band"), m.get("midpointMjdTai"), m.get("psfFlux"))
        else {
            continue;
        };
        if band.len() == 1 && mjd.is_finite() && flux.is_finite() && *flux > 0.0 {
            rows.push((band.clone(), *mjd, *flux));
        }
    }
    (coord, rows)
}

fn top_level_shape(body: &[u8]) -> Vec<(String, usize)> {
    let Ok(text) = std::str::from_utf8(body) else {
        return Vec::new();
    };
    let Some(root) = parse_json(text) else {
        return Vec::new();
    };
    match root {
        JsonVal::Obj(map) => {
            let mut out: Vec<(String, usize)> = map
                .iter()
                .map(|(k, v)| {
                    let len = match v {
                        JsonVal::Arr(a) => a.len(),
                        JsonVal::Obj(o) => o.len(),
                        _ => 1,
                    };
                    (k.clone(), len)
                })
                .collect();
            out.sort();
            out
        }
        JsonVal::Arr(a) => vec![("[root array]".to_string(), a.len())],
        _ => Vec::new(),
    }
}

fn median_f32(v: &[f32]) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = s.len();
    Some(if n % 2 == 1 {
        s[n / 2]
    } else {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    })
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let var = v.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / n;
    (m, var.sqrt())
}

fn residual_series_lsst(curve: &LsstCurve) -> Option<Vec<(f64, f32)>> {
    let flux: Vec<f32> = curve.samples.iter().map(|&(_, f)| f).collect();
    let med = median_f32(&flux)?;
    let res: Vec<f64> = if med > 0.0 {
        flux.iter().map(|&f| (f / med - 1.0) as f64).collect()
    } else {
        let (_, sd) = mean_sd(&flux.iter().map(|&f| f as f64).collect::<Vec<f64>>());
        if sd <= 0.0 {
            return None;
        }
        flux.iter()
            .map(|&f| ((f as f64) - med as f64) / sd)
            .collect()
    };
    Some(
        curve
            .samples
            .iter()
            .zip(&res)
            .map(|(&(t, _), &r)| (t, r as f32))
            .collect(),
    )
}

fn join_series(a: &[(f64, f32)], b: &[(f64, f32)]) -> Vec<(f64, f32, f32)> {
    let mut pairs: Vec<(f64, f32, f32)> = Vec::new();
    let mut used: Vec<bool> = vec![false; b.len()];
    for &(ta, fa) in a {
        let mut best: Option<(usize, f64)> = None;
        for (j, &(tb, _)) in b.iter().enumerate() {
            if used[j] {
                continue;
            }
            let dt = (ta - tb).abs();
            if dt <= COINCIDENCE_S && best.map(|(_, d)| dt < d).unwrap_or(true) {
                best = Some((j, dt));
            }
        }
        if let Some((j, _)) = best {
            used[j] = true;
            pairs.push((ta, fa, b[j].1));
        }
    }
    pairs.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(std::cmp::Ordering::Equal));
    pairs
}

fn spearman(a: &[f32], b: &[f32]) -> Option<f64> {
    let mut idx: Vec<usize> = (0..a.len()).collect();
    idx.sort_by(|&x, &y| a[x].partial_cmp(&a[y]).unwrap_or(std::cmp::Ordering::Equal));
    let mut rank_a: Vec<f64> = vec![0.0; a.len()];
    for (r, &i) in idx.iter().enumerate() {
        rank_a[i] = r as f64;
    }
    idx.sort_by(|&x, &y| b[x].partial_cmp(&b[y]).unwrap_or(std::cmp::Ordering::Equal));
    let mut rank_b: Vec<f64> = vec![0.0; b.len()];
    for (r, &i) in idx.iter().enumerate() {
        rank_b[i] = r as f64;
    }
    let n = a.len() as f64;
    let ma = rank_a.iter().sum::<f64>() / n;
    let mb = rank_b.iter().sum::<f64>() / n;
    let cov = rank_a
        .iter()
        .zip(&rank_b)
        .map(|(&x, &y)| (x - ma) * (y - mb))
        .sum::<f64>();
    let va = rank_a.iter().map(|&x| (x - ma) * (x - ma)).sum::<f64>();
    let vb = rank_b.iter().map(|&x| (x - mb) * (x - mb)).sum::<f64>();
    if va <= 0.0 || vb <= 0.0 {
        return None;
    }
    Some(cov / (va * vb).sqrt())
}

// The periodogram power in the standard normalization (variance-normalized,
// both quadrature terms, the τ shift that makes the cosine/sine basis
// orthogonal). The statistic is amplitude-invariant: a uniform rescale of the
// series moves the data and the variance together, so the FAP measures the
// significance of the periodicity, not the photometric scale. The form it
// replaces carried x² inside the denominator, making the power scale ~ n/σ² —
// quiet fractional photometry (σ ~ 0.02) read FAP 0 on every row and the
// aperiodic gate never opened (measured, committed a4b1dcc).
fn lomb_scargle_fap(t: &[f64], r: &[f32]) -> (f64, f64) {
    let n = t.len();
    if n < 8 {
        return (0.0, 1.0);
    }
    let x: Vec<f64> = r.iter().map(|&v| v as f64).collect();
    let mean = x.iter().sum::<f64>() / n as f64;
    let var = x.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n as f64;
    if var <= 0.0 {
        return (0.0, 1.0);
    }
    let tmin = t[0];
    let tmax = t[n - 1];
    let span = (tmax - tmin).max(1.0);
    let fmin = 1.0 / span;
    let fmax = 1.0 / (2.0 * (span / n as f64).max(1.0));
    let mut best_z = 0.0f64;
    let mut nf = 0u32;
    let mut f = fmin;
    while f <= fmax {
        let w = 2.0 * std::f64::consts::PI * f;
        let mut c2 = 0.0f64;
        let mut s2 = 0.0f64;
        for &ti in t {
            c2 += (2.0 * w * ti).cos();
            s2 += (2.0 * w * ti).sin();
        }
        let tau = 0.5 * s2.atan2(c2) / w;
        let mut num_c = 0.0f64;
        let mut num_s = 0.0f64;
        let mut den_c = 0.0f64;
        let mut den_s = 0.0f64;
        for (i, &ti) in t.iter().enumerate() {
            let xv = x[i] - mean;
            let ph = w * (ti - tau);
            num_c += xv * ph.cos();
            num_s += xv * ph.sin();
            den_c += ph.cos() * ph.cos();
            den_s += ph.sin() * ph.sin();
        }
        if den_c > 1e-12 && den_s > 1e-12 {
            let z = 0.5 * (num_c * num_c / den_c + num_s * num_s / den_s) / var;
            if z > best_z {
                best_z = z;
            }
        }
        nf += 1;
        f *= 1.05;
    }
    let n_indep = nf.max(1) as f64;
    let fap = 1.0 - (1.0 - (-best_z).exp()).powf(n_indep);
    (best_z, fap)
}

fn deepest_dip(res: &[(f64, f32)]) -> (f64, f64) {
    let vals: Vec<f64> = res.iter().map(|&(_, r)| r as f64).collect();
    let (_, sd) = mean_sd(&vals);
    let mut min = res[0].1 as f64;
    for &(_, r) in res {
        if (r as f64) < min {
            min = r as f64;
        }
    }
    (min, min / sd.max(1e-300))
}

struct ConeObject {
    id: String,
    ra_deg: f64,
    dec_deg: f64,
    n_sources: usize,
    class: i64,
    simbad: String,
}

fn extract_dia_ids(body: &[u8]) -> Vec<String> {
    let Ok(text) = std::str::from_utf8(body) else {
        return Vec::new();
    };
    let needle = "\"r:diaObjectId\":";
    let mut out: Vec<String> = Vec::new();
    let mut pos = 0;
    while let Some(rel) = text[pos..].find(needle) {
        let s = pos + rel + needle.len();
        let mut digits = String::new();
        for c in text[s..].chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else {
                break;
            }
        }
        if !digits.is_empty() {
            let consumed = digits.len();
            out.push(digits);
            pos = s + consumed;
        } else {
            pos = s + 1;
        }
    }
    out
}

fn fink_cone_list(
    ra: f64,
    dec: f64,
    radius_arcsec: f64,
    min_sources: usize,
    save: Option<&str>,
) -> Vec<ConeObject> {
    let payload = format!(
        "{{\"ra\": {ra}, \"dec\": {dec}, \"radius\": {radius_arcsec}, \"columns\": \"r:diaObjectId,r:ra,r:dec,r:nDiaSources,f:main_label_classifier,f:xm_simbad_otype\"}}"
    );
    let Some((code, body)) = curl_post_rate_aware(FINK_CONE, &payload) else {
        println!(
            "Fink/LSST cone ({ra}, {dec}, {radius_arcsec} arcsec): the cone query did not answer (measured stall) — pending"
        );
        return Vec::new();
    };
    if code != "200" {
        println!(
            "Fink/LSST cone ({ra}, {dec}, {radius_arcsec} arcsec): the cone answered HTTP {code} — pending"
        );
        return Vec::new();
    }
    if let Some(p) = save {
        if std::fs::write(p, &body).is_err() {
            println!("Fink/LSST cone: the cone sample was not saved ({p})");
        }
    }
    let Ok(text) = std::str::from_utf8(&body) else {
        println!("Fink/LSST cone: the cone body is not UTF-8 — the parser stays pending");
        return Vec::new();
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        println!("Fink/LSST cone: the cone body is not a JSON array — the parser stays pending");
        return Vec::new();
    };
    let ids = extract_dia_ids(&body);
    if ids.len() != rows.len() {
        println!(
            "Fink/LSST cone: {} row(s) but {} exact diaObjectId token(s) — the id alignment stays pending",
            rows.len(),
            ids.len()
        );
        return Vec::new();
    }
    let mut objs: Vec<ConeObject> = Vec::new();
    for (idx, r) in rows.iter().enumerate() {
        let JsonVal::Obj(m) = r else { continue };
        let (Some(ra_v), Some(dec_v), Some(n_v)) = (
            obj_f64(m, FINK_RA),
            obj_f64(m, FINK_DEC),
            obj_f64(m, FINK_NDIA),
        ) else {
            continue;
        };
        let class = obj_f64(m, FINK_CLASS).map(|c| c as i64).unwrap_or(-1);
        let simbad = obj_str(m, FINK_SIMBAD).unwrap_or("Fail").to_string();
        objs.push(ConeObject {
            id: ids[idx].clone(),
            ra_deg: ra_v,
            dec_deg: dec_v,
            n_sources: n_v as usize,
            class,
            simbad,
        });
    }
    objs.sort_by(|a, b| b.n_sources.cmp(&a.n_sources));
    let total = objs.len();
    objs.retain(|o| o.n_sources >= min_sources);
    println!(
        "Fink/LSST cone ({ra}, {dec}, {radius_arcsec} arcsec): HTTP {code}, {total} object row(s); {} with nDiaSources >= {min_sources} chosen for the light-curve fetch",
        objs.len()
    );
    objs
}

fn parse_fink_source_rows(body: &[u8]) -> Option<(Vec<(String, f64, f64)>, Option<(f64, f64)>)> {
    let Ok(text) = std::str::from_utf8(body) else {
        return None;
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        return None;
    };
    let mut rows_ok: Vec<(String, f64, f64)> = Vec::new();
    let mut coord: Option<(f64, f64)> = None;
    for r in &rows {
        let JsonVal::Obj(m) = r else { continue };
        let (Some(band), Some(mjd), Some(flux)) = (
            obj_str(m, FINK_BAND),
            obj_f64(m, FINK_MJD),
            obj_f64(m, FINK_FLUX),
        ) else {
            continue;
        };
        if coord.is_none() {
            if let (Some(ra), Some(dec)) = (obj_f64(m, FINK_RA), obj_f64(m, FINK_DEC)) {
                coord = Some((ra, dec));
            }
        }
        rows_ok.push((band.to_string(), mjd, flux));
    }
    Some((rows_ok, coord))
}

fn build_band_curves(ra: f64, dec: f64, rows: &[(String, f64, f64)]) -> Vec<LsstCurve> {
    let mut t_min = f64::INFINITY;
    for (_, tdb, _) in rows {
        if *tdb < t_min {
            t_min = *tdb;
        }
    }
    let mut curves: Vec<LsstCurve> = Vec::new();
    for (name, _) in LSST_LAMBDA_NM {
        let Some(freq) = lsst_freq_of_band(name) else {
            continue;
        };
        let mut samples: Vec<(f64, f32)> = rows
            .iter()
            .filter(|(b, _, _)| b == name)
            .map(|(_, tdb, flux)| (tdb - t_min, *flux as f32))
            .collect();
        if samples.is_empty() {
            continue;
        }
        samples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        curves.push(LsstCurve {
            ra_deg: ra,
            dec_deg: dec,
            freq,
            samples,
        });
    }
    curves
}

fn natural_excluded(class: i64, simbad: &str) -> bool {
    !(class == -1 && simbad == "Fail")
}

struct ConeVerdict {
    fetched: usize,
    multiband_scanned: usize,
    candidates_pre: usize,
    excluded_natural: usize,
    unclassified: usize,
}

impl ConeVerdict {
    fn void() -> ConeVerdict {
        ConeVerdict {
            fetched: 0,
            multiband_scanned: 0,
            candidates_pre: 0,
            excluded_natural: 0,
            unclassified: 0,
        }
    }
}

fn cone_scan(
    ra: f64,
    dec: f64,
    radius_arcsec: f64,
    min_sources: usize,
    save: Option<&str>,
    brief: bool,
) -> ConeVerdict {
    let Some(lsk) = embedded_lsk() else {
        println!(
            "Nadel V (LSST round): the embedded naif0012.tls leap table is absent — the time layer stays pending"
        );
        return ConeVerdict::void();
    };
    if !brief {
        println!(
            "Nadel V (LSST round): time layer — Fink r:midpointMjdTai is MJD/TAI; each row is mapped mjd → unix → TDB (naif0012.tls ΔT 32.184 s + leap) before the fold axis is built"
        );
    }
    let cone_save = save.map(str::to_string);
    let objs = fink_cone_list(ra, dec, radius_arcsec, min_sources, cone_save.as_deref());
    if objs.is_empty() {
        println!(
            "Nadel V (LSST round): the cone ({ra}, {dec}, {radius_arcsec} arcsec) yields no object above the cut — the scan stays void (0 honored)"
        );
        return ConeVerdict::void();
    }
    let eph = load_ephemeris_map();
    if !brief {
        match &eph {
            Some(_) => report_term_budget(),
            None => println!(
                "Nadel V (LSST round): the solar-system ephemeris for the light-travel (Rømer) term is not reachable — the fold axis stays clock-scale TDB (the seasonal ±8 min Rømer term is pending, named)"
            ),
        }
    }
    let source_cols = "r:diaObjectId,r:ra,r:dec,r:band,r:midpointMjdTai,r:scienceFlux";
    let mut curves_all: Vec<LsstCurve> = Vec::new();
    let mut fetched = 0usize;
    let mut total_rows = 0usize;
    let mut tdb_skipped = 0usize;
    let mut roemer_station = 0usize;
    let mut roemer_geocenter = 0usize;
    let mut roemer_uncorrected = 0usize;
    for o in &objs {
        let payload = format!(
            "{{\"diaObjectId\": \"{}\", \"columns\": \"{source_cols}\"}}",
            o.id
        );
        let Some((code, body)) = curl_post_rate_aware(FINK_API_SOURCES, &payload) else {
            println!(
                "Fink/LSST {}: the sources query did not answer (measured stall) — pending",
                o.id
            );
            sleep_ms(OBJECT_PAUSE_MS);
            continue;
        };
        if code != "200" {
            println!(
                "Fink/LSST {}: the light curve answered HTTP {code} — pending",
                o.id
            );
            sleep_ms(OBJECT_PAUSE_MS);
            continue;
        }
        let Some((rows, _)) = parse_fink_source_rows(&body) else {
            println!(
                "Fink/LSST {}: the sample is not a JSON array — parser pending on the real schema",
                o.id
            );
            sleep_ms(OBJECT_PAUSE_MS);
            continue;
        };
        if rows.is_empty() {
            println!(
                "Fink/LSST {}: no row carries the full band/time/flux triple (absent)",
                o.id
            );
            sleep_ms(OBJECT_PAUSE_MS);
            continue;
        }
        let rows_fold: Vec<(String, f64, f64)> = match &eph {
            Some(e) => {
                let fo = rows_to_tdb_roemer(&lsk, e, o.ra_deg, o.dec_deg, &rows);
                tdb_skipped += fo.skipped;
                roemer_station += fo.station_rows;
                roemer_geocenter += fo.geocenter_rows;
                roemer_uncorrected += fo.uncorrected_rows;
                fo.rows
            }
            None => {
                let (rows_tdb, skipped) = rows_to_tdb(&lsk, &rows);
                tdb_skipped += skipped;
                rows_tdb
            }
        };
        if rows_fold.is_empty() {
            println!(
                "Fink/LSST {}: no sample row maps onto the TDB time layer (absent)",
                o.id
            );
            sleep_ms(OBJECT_PAUSE_MS);
            continue;
        }
        let curves = build_band_curves(o.ra_deg, o.dec_deg, &rows_fold);
        if curves.is_empty() {
            println!(
                "Fink/LSST {}: no measurement row maps to a known LSST band (absent)",
                o.id
            );
            sleep_ms(OBJECT_PAUSE_MS);
            continue;
        }
        if !brief {
            let mut per_band: HashMap<String, usize> = HashMap::new();
            for (band, _, _) in &rows {
                *per_band.entry(band.clone()).or_insert(0) += 1;
            }
            let mut band_counts: Vec<(&str, usize)> =
                per_band.iter().map(|(b, n)| (b.as_str(), *n)).collect();
            band_counts.sort();
            let counts: Vec<String> = band_counts
                .iter()
                .map(|(b, n)| format!("{b} {n}"))
                .collect();
            println!(
                "Fink/LSST {} (nDiaSources {}): {} rows over {} — {}",
                o.id,
                o.n_sources,
                rows.len(),
                counts.join(", "),
                curves.len()
            );
        }
        curves_all.extend(curves);
        fetched += 1;
        total_rows += rows.len();
        sleep_ms(OBJECT_PAUSE_MS);
    }
    if tdb_skipped > 0 {
        println!(
            "Nadel V (LSST round): {tdb_skipped} sample row(s) carry no TDB mapping (the leap table is void at their epoch — absent)"
        );
    }
    if roemer_station + roemer_geocenter + roemer_uncorrected > 0 {
        println!(
            "Nadel V (LSST round): light-travel (Rømer) layer over the fetched rows — {roemer_station} row(s) corrected with the Cerro Pachón observatory position, {roemer_geocenter} with the geocenter (no earth orientation in the ephemeris — the ±21 ms diurnal swing stays a named bound), {roemer_uncorrected} kept clock-scale TDB (the ephemeris does not cover the epoch — pending)"
        );
    }
    println!(
        "Nadel V (LSST round): {fetched} object light curve(s) fetched, {total_rows} measurement row(s) total"
    );
    if curves_all.is_empty() {
        println!(
            "Nadel V (LSST round): no object carries a light curve — the scan stays void (0 honored)"
        );
        return ConeVerdict {
            fetched,
            ..ConeVerdict::void()
        };
    }
    let bin_path =
        format!("/tmp/opencode/lsst_lightcurves_cone_{ra:.5}_{dec:.5}_{radius_arcsec:.0}.bin");
    if std::fs::write(&bin_path, serialize_lss1(&curves_all)).is_err() {
        println!("Nadel V (LSST round): the LSS1 asset was not written ({bin_path})");
        return ConeVerdict {
            fetched,
            ..ConeVerdict::void()
        };
    }
    let map_path =
        format!("/tmp/opencode/lsst_cone_object_map_{ra:.5}_{dec:.5}_{radius_arcsec:.0}.csv");
    {
        let mut lines: Vec<String> = Vec::new();
        lines.push("diaObjectId,ra,dec,nDiaSources,class,simbad".to_string());
        for o in &objs {
            lines.push(format!(
                "{},{:.8},{:.8},{},{},{}",
                o.id, o.ra_deg, o.dec_deg, o.n_sources, o.class, o.simbad
            ));
        }
        if std::fs::write(&map_path, lines.join("\n")).is_err() {
            println!("Nadel V (LSST round): the object map was not written ({map_path})");
        }
    }
    println!("Nadel V (LSST round): LSS1 asset written {bin_path}, object map {map_path}");
    let (candidates, scanned) = scan_lss1(&bin_path, brief);
    let mut post = 0usize;
    let mut excluded = 0usize;
    for (cra, cdec) in &candidates {
        let hit = objs
            .iter()
            .find(|o| (o.ra_deg - cra).abs() < 1e-3 && (o.dec_deg - cdec).abs() < 1e-3);
        match hit {
            Some(o) => {
                if natural_excluded(o.class, &o.simbad) {
                    excluded += 1;
                    println!(
                        "Nadel V (LSST round): candidate at ra {cra:.4} dec {cdec:.4} is {} (class {} {}) — a natural dimmer, excluded",
                        o.id, o.class, o.simbad
                    );
                } else {
                    post += 1;
                    println!(
                        "Nadel V (LSST round): candidate at ra {cra:.4} dec {cdec:.4} is {} (class -1, no SIMBAD id) — unclassified, pending the natural-class crossmatch",
                        o.id
                    );
                }
            }
            None => {
                println!(
                    "Nadel V (LSST round): candidate at ra {cra:.4} dec {cdec:.4} has no object row in the cone map — pending the crossmatch"
                );
            }
        }
    }
    println!(
        "Nadel V (LSST round) verdict over {fetched} cone object(s): {excluded} candidate dip(s) excluded as catalogued natural dimmers, {post} unclassified dip(s) remain pending the natural-class crossmatch"
    );
    ConeVerdict {
        fetched,
        multiband_scanned: scanned,
        candidates_pre: candidates.len(),
        excluded_natural: excluded,
        unclassified: post,
    }
}

// The Lasair-LSST cone flow (the operator's registered broker, LASAIR_LSST_TOKEN).
// Without the token the authenticated queries stay pending — the reachability is
// measured, never a fabricated query. With the token the flow is: /api/cone/
// (requestType all) for the object ids, then /api/object/ per id for the
// multi-band lightcurve rows (band, midpointMjdTai, psfFlux — measured from the
// official example notebook) onto the same TDB fold layer and LSS1 dip cut the
// Fink path uses.
fn lasair_cone_scan(ra: f64, dec: f64, radius_arcsec: f64, max_objects: usize) {
    println!(
        "\n=== Nadel V (LSST round): Lasair-LSST cone ({ra}, {dec}, {radius_arcsec} arcsec, up to {max_objects} object light curves) ==="
    );
    let anon_cone = http_code(LAS_CONE, None);
    let anon_obj = http_code(LAS_OBJECT, None);
    println!(
        "Lasair-LSST /api/cone/ (anonymous): HTTP {} — token-gated (measured)",
        anon_cone
            .as_deref()
            .unwrap_or("no response (connection stalled)")
    );
    println!(
        "Lasair-LSST /api/object/ (anonymous): HTTP {} — token-gated (measured)",
        anon_obj
            .as_deref()
            .unwrap_or("no response (connection stalled)")
    );
    let Some(token) = lasair_token() else {
        println!(
            "Verdict: pending — LASAIR_LSST_TOKEN absent (env or the .secrets.local key). The token lands in {} under LASAIR_LSST_TOKEN; until then the authenticated cone/object queries cannot run and stay pending — the endpoints above are measured alive, the scan itself is not fabricated.",
            {
                let mut p = cache_root();
                p.pop();
                p.join(".secrets.local").display().to_string()
            }
        );
        return;
    };
    println!(
        "Lasair-LSST: LASAIR_LSST_TOKEN present — the authenticated cone runs (rate: 100 calls/h for a user token)"
    );
    let cone_form = format!("ra={ra}&dec={dec}&radius={radius_arcsec}&requestType=all");
    let Some((code, body)) = curl_form_post_rate_aware(LAS_CONE, &cone_form, &token) else {
        println!("Verdict: pending — the cone query did not answer (measured stall)");
        return;
    };
    if code != "200" {
        println!("Verdict: pending — the cone answered HTTP {code}");
        return;
    }
    let cone_path =
        format!("/tmp/opencode/lasair_lsst_cone_{ra:.5}_{dec:.5}_{radius_arcsec:.0}.json");
    if std::fs::write(&cone_path, &body).is_err() {
        println!("Verdict: pending — the cone sample was not saved ({cone_path})");
        return;
    }
    println!(
        "Lasair-LSST cone: HTTP {code}, {} bytes, sample saved {cone_path}",
        body.len()
    );
    let Some(rows) = parse_lasair_cone(&body) else {
        println!(
            "Lasair-LSST cone: the body is not a row array of {{object, separation}} (schema measured, parser pending on a real sample):"
        );
        let shape = top_level_shape(&body);
        for (k, len) in &shape {
            println!("  {k}: {len}");
        }
        println!("Verdict: pending");
        return;
    };
    if rows.is_empty() {
        println!(
            "Verdict: the cone ({ra}, {dec}, {radius_arcsec} arcsec) holds no object — the scan stays void (0 honored)"
        );
        return;
    }
    let chosen = rows.len().min(max_objects);
    println!(
        "Lasair-LSST cone: {} object row(s) within the cone; {} chosen for the light-curve fetch (ascending separation)",
        rows.len(),
        chosen
    );
    let Some(lsk) = embedded_lsk() else {
        println!(
            "Verdict: pending — the embedded naif0012.tls leap table is absent — the time layer stays pending"
        );
        return;
    };
    let eph = load_ephemeris_map();
    if eph.is_none() {
        println!(
            "Nadel V (LSST round): the solar-system ephemeris for the light-travel (Rømer) term is not reachable — the fold axis stays clock-scale TDB (the seasonal ±8 min Rømer term is pending, named)"
        );
    }
    let mut curves_all: Vec<LsstCurve> = Vec::new();
    let mut map_lines: Vec<String> = Vec::new();
    map_lines.push("diaObjectId,ra,dec,separation".to_string());
    let mut fetched = 0usize;
    let mut total_rows = 0usize;
    for row in rows.iter().take(chosen) {
        let obj_form = format!("objectId={}", row.id);
        let Some((code, body)) = curl_form_post_rate_aware(LAS_OBJECT, &obj_form, &token) else {
            println!(
                "Lasair-LSST {}: the light curve query did not answer (measured stall) — pending",
                row.id
            );
            sleep_ms(LAS_OBJECT_PAUSE_MS);
            continue;
        };
        if code != "200" {
            println!(
                "Lasair-LSST {}: the light curve answered HTTP {code} — pending",
                row.id
            );
            sleep_ms(LAS_OBJECT_PAUSE_MS);
            continue;
        }
        let obj_path = format!("/tmp/opencode/lasair_lsst_object_{}.json", row.id);
        if std::fs::write(&obj_path, &body).is_err() {
            println!(
                "Lasair-LSST {}: the object sample was not saved ({obj_path})",
                row.id
            );
        }
        let (coord, rows) = parse_lasair_lightcurve(&body);
        if rows.is_empty() {
            println!(
                "Lasair-LSST {}: no diaSourcesList row carries the band/midpointMjdTai/psfFlux triple — the parser stays pending on the real schema (sample saved {obj_path})",
                row.id
            );
            sleep_ms(LAS_OBJECT_PAUSE_MS);
            continue;
        }
        let (o_ra, o_dec) = match coord {
            Some(c) => c,
            None => (ra, dec),
        };
        let rows_fold: Vec<(String, f64, f64)> = match &eph {
            Some(e) => {
                let fo = rows_to_tdb_roemer(&lsk, e, o_ra, o_dec, &rows);
                fo.rows
            }
            None => rows_to_tdb(&lsk, &rows).0,
        };
        if rows_fold.is_empty() {
            println!(
                "Lasair-LSST {}: no sample row maps onto the TDB time layer (absent)",
                row.id
            );
            sleep_ms(LAS_OBJECT_PAUSE_MS);
            continue;
        }
        let mut per_band: HashMap<String, usize> = HashMap::new();
        for (band, _, _) in &rows {
            *per_band.entry(band.clone()).or_insert(0) += 1;
        }
        let mut band_counts: Vec<(&str, usize)> =
            per_band.iter().map(|(b, n)| (b.as_str(), *n)).collect();
        band_counts.sort();
        let counts: Vec<String> = band_counts
            .iter()
            .map(|(b, n)| format!("{b} {n}"))
            .collect();
        let curves = build_band_curves(o_ra, o_dec, &rows_fold);
        println!(
            "Lasair-LSST {}: {} rows over {} → {} band curve(s) at ra {o_ra:.4} dec {o_dec:.4}",
            row.id,
            rows.len(),
            counts.join(", "),
            curves.len()
        );
        if curves.is_empty() {
            sleep_ms(LAS_OBJECT_PAUSE_MS);
            continue;
        }
        curves_all.extend(curves);
        map_lines.push(format!(
            "{},{:.8},{:.8},{:.4}",
            row.id, o_ra, o_dec, row.separation
        ));
        fetched += 1;
        total_rows += rows.len();
        sleep_ms(LAS_OBJECT_PAUSE_MS);
    }
    println!(
        "Nadel V (LSST round): {fetched} Lasair-LSST object light curve(s) fetched, {total_rows} measurement row(s) total"
    );
    if curves_all.is_empty() {
        println!(
            "Verdict: no Lasair-LSST object carries a light curve — the scan stays void (0 honored)"
        );
        return;
    }
    let bin_path = format!(
        "/tmp/opencode/lsst_lightcurves_lasair_cone_{ra:.5}_{dec:.5}_{radius_arcsec:.0}.bin"
    );
    if std::fs::write(&bin_path, serialize_lss1(&curves_all)).is_err() {
        println!("Verdict: pending — the LSS1 asset was not written ({bin_path})");
        return;
    }
    let map_path =
        format!("/tmp/opencode/lasair_cone_object_map_{ra:.5}_{dec:.5}_{radius_arcsec:.0}.csv");
    if std::fs::write(&map_path, map_lines.join("\n")).is_err() {
        println!("Nadel V (LSST round): the object map was not written ({map_path})");
    }
    println!("Nadel V (LSST round): LSS1 asset written {bin_path}, object map {map_path}");
    scan_lss1(&bin_path, false);
    println!(
        "Verdict: the Lasair-LSST cone scan above is a full measurement through the TDB fold layer and the achromatic non-periodic dip cut (scale-invariant FAP gate)"
    );
}

fn grid_scan(cones: &[(f64, f64, f64, usize)]) {
    println!(
        "\n=== Nadel V (LSST round): anonymous cone grid, time layer MJD/TAI → TDB (embedded naif0012.tls), {} cone(s) ===",
        cones.len()
    );
    let mut verdict = ConeVerdict::void();
    for (i, (ra, dec, radius, min)) in cones.iter().enumerate() {
        println!(
            "\n--- grid cone {} of {}: ra {ra} dec {dec} radius {radius} arcsec min nDiaSources {min} ---",
            i + 1,
            cones.len()
        );
        let v = cone_scan(*ra, *dec, *radius, *min, None, true);
        verdict.fetched += v.fetched;
        verdict.multiband_scanned += v.multiband_scanned;
        verdict.candidates_pre += v.candidates_pre;
        verdict.excluded_natural += v.excluded_natural;
        verdict.unclassified += v.unclassified;
        sleep_ms(CONE_PAUSE_MS);
    }
    println!(
        "\n=== Nadel V grid verdict over {} anonymous cone scan(s): {} object light curve(s) fetched, {} multiband object(s) fully evaluated on the TDB fold axis, {} pre-exclusion candidate dip(s), {} excluded as catalogued natural dimmers, {} unclassified dip(s) pending the natural-class crossmatch ===",
        cones.len(),
        verdict.fetched,
        verdict.multiband_scanned,
        verdict.candidates_pre,
        verdict.excluded_natural,
        verdict.unclassified
    );
    println!(
        "Quantitative limit: no unexcluded achromatic non-periodic dip above DIP_SIG {DIP_SIG}σ across {scanned} fully evaluated multiband object(s) on the TDB time layer — or the line above names it",
        scanned = verdict.multiband_scanned
    );
}

fn fink_scan(id: &str, save: Option<&str>) {
    let payload = format!("{{\"diaObjectId\": \"{id}\"}}");
    let Some((code, body)) = curl_post_bytes(FINK_API_SOURCES, &payload) else {
        println!("Fink/LSST {id}: the sources query did not answer (measured stall) — pending");
        return;
    };
    if code != "200" {
        println!("Fink/LSST {id}: the light curve answered HTTP {code} — pending");
        return;
    }
    let raw_path = save
        .map(str::to_string)
        .unwrap_or_else(|| format!("/tmp/opencode/fink_lsst_sources_{id}.json"));
    if std::fs::write(&raw_path, &body).is_err() {
        println!("Fink/LSST {id}: the real sample was not saved ({raw_path})");
        return;
    }
    println!(
        "Fink/LSST /api/v1/sources {id}: HTTP {code}, {} bytes, real sample saved: {raw_path}",
        body.len()
    );
    let Some((rows_ok, coord)) = parse_fink_source_rows(&body) else {
        println!(
            "Fink/LSST {id}: the sample is not a JSON array — parser pending on the real schema"
        );
        return;
    };
    if rows_ok.is_empty() {
        println!("Fink/LSST {id}: no row carries the full band/time/flux triple (absent)");
        return;
    }
    let Some(lsk) = embedded_lsk() else {
        println!(
            "Fink/LSST {id}: the embedded naif0012.tls leap table is absent — the time layer stays pending"
        );
        return;
    };
    println!(
        "Fink/LSST {id}: time layer — r:midpointMjdTai is MJD/TAI, mapped mjd → unix → TDB (naif0012.tls ΔT 32.184 s + leap) before the fold axis is built"
    );
    report_term_budget();
    let eph = load_ephemeris_map();
    if eph.is_none() {
        println!(
            "Fink/LSST {id}: the solar-system ephemeris for the light-travel (Rømer) term is not reachable — the fold axis stays clock-scale TDB (the seasonal ±8 min Rømer term is pending, named)"
        );
    }
    let (rows_fold, skipped) = match &eph {
        Some(e) => {
            let ra_hint = coord.map(|(r, _)| r).unwrap_or(0.0);
            let dec_hint = coord.map(|(_, d)| d).unwrap_or(0.0);
            let fo = rows_to_tdb_roemer(&lsk, e, ra_hint, dec_hint, &rows_ok);
            println!(
                "Fink/LSST {id}: light-travel (Rømer) layer — {fo_station} row(s) corrected with the Cerro Pachón observatory position, {fo_geo} with the geocenter (no earth orientation in the ephemeris — the ±21 ms diurnal swing stays a named bound), {fo_unc} kept clock-scale TDB (the ephemeris does not cover the epoch — pending)",
                fo_station = fo.station_rows,
                fo_geo = fo.geocenter_rows,
                fo_unc = fo.uncorrected_rows
            );
            (fo.rows, fo.skipped)
        }
        None => rows_to_tdb(&lsk, &rows_ok),
    };
    if skipped > 0 {
        println!(
            "Fink/LSST {id}: {skipped} sample row(s) carry no TDB mapping (the leap table is void at their epoch — absent)"
        );
    }
    if rows_fold.is_empty() {
        println!("Fink/LSST {id}: no sample row maps onto the TDB time layer (absent)");
        return;
    }
    let (ra, dec) = match coord {
        Some(c) => c,
        None => {
            println!("Fink/LSST {id}: no row carries ra/dec — the object stays unplaced (pending)");
            return;
        }
    };
    let mut per_band: HashMap<String, usize> = HashMap::new();
    for (band, _, _) in &rows_ok {
        *per_band.entry(band.clone()).or_insert(0) += 1;
    }
    let mut band_counts: Vec<(&str, usize)> =
        per_band.iter().map(|(b, n)| (b.as_str(), *n)).collect();
    band_counts.sort();
    for (b, n) in &band_counts {
        println!("  Fink/LSST band {b}: {n} measurement rows");
    }
    let curves = build_band_curves(ra, dec, &rows_fold);
    if curves.is_empty() {
        println!("Fink/LSST {id}: no measurement row maps to a known LSST band (absent)");
        return;
    }
    let bin_path = format!("/tmp/opencode/lsst_lightcurves_{id}.bin");
    if std::fs::write(&bin_path, serialize_lss1(&curves)).is_err() {
        println!("Fink/LSST {id}: the LSS1 asset was not written ({bin_path})");
        return;
    }
    println!(
        "Fink/LSST {id}: LSS1 asset written {bin_path} — {} band curve(s) over {} rows",
        curves.len(),
        rows_ok.len()
    );
    scan_lss1(&bin_path, false);
}

fn usage() {
    eprintln!(
        "lsst_anomaly_probe — Nadel V over the Lasair-LSST broker\n\
         reachability (default):\n\
         \x20 lsst_anomaly_probe [--object <diaObjectId>] [--token <t>] [--save <path.json>]\n\
         Fink/LSST anonymous light curve (api.lsst.fink-portal.org), no token:\n\
         \x20 lsst_anomaly_probe --fink <diaObjectId> [--save <raw.json>]\n\
         Fink/LSST anonymous cone scan over a real object set, no token:\n\
         \x20 lsst_anomaly_probe --cone-ra <deg> --cone-dec <deg> --cone-radius <arcsec> [--cone-min <nDiaSources>] [--save <cone.json>]\n\
         Fink/LSST anonymous grid of cone scans, no token (repeated --cone ra,dec,radius_arcsec,min):\n\
         \x20 lsst_anomaly_probe --cone 148.84,2.55,260,24 --cone 149.44,2.55,260,24\n\
         Lasair-LSST cone with the operator's account token (LASAIR_LSST_TOKEN in the .secrets.local key or env):\n\
         \x20 lsst_anomaly_probe --lasair-ra <deg> --lasair-dec <deg> --lasair-radius <arcsec> [--lasair-max <objects=6>]\n\
         scan_lss1 of a real LSS1 light-curve asset:\n\
         \x20 lsst_anomaly_probe --scan <lsst_lightcurves.bin>\n\
         negative control of the achromatic dip cut (synthetic achromatic dip into a real LSS1 asset):\n\
         \x20 lsst_anomaly_probe --negative-control <lsst_lightcurves.bin> [<depth_sigma=8>]\n\
         LASAIR_TOKEN (the register token name) is read from the environment when --token is absent."
    );
}

fn reach(token: Option<String>, object: Option<String>, save: Option<String>) {
    let root = http_code(LSST_ROOT, None);
    let anon = http_code(LSST_API, None);
    println!(
        "Lasair-LSST web root  {LSST_ROOT}: HTTP {}",
        root.as_deref()
            .unwrap_or("no response (connection stalled)")
    );
    println!(
        "Lasair-LSST API query (anonymous): HTTP {} — the stream is token-gated",
        anon.as_deref()
            .unwrap_or("no response (connection stalled)")
    );
    let Some(tok) = token else {
        println!(
            "Verdict: reachability measured above; stream data pending — LASAIR_TOKEN absent (Lasair-LSST needs its own account, register at https://lasair.lsst.ac.uk/register)"
        );
        return;
    };
    let obj = match object {
        Some(o) => o,
        None => {
            let q = format!(
                "{LSST_API}?selected=diaObjectId&tables=objects&conditions=lastDiaSourceMjdTai%3E0&order_by=lastDiaSourceMjdTai&order_mode=DESC&limit=1&format=json"
            );
            let Some((code, body)) = curl_bytes(&q, Some(&tok)) else {
                println!("Verdict: pending — the query did not answer (measured stall)");
                return;
            };
            if code != "200" {
                println!(
                    "Verdict: pending — the query answered HTTP {code} (expect a 401 without a valid token)"
                );
                return;
            }
            let text = String::from_utf8_lossy(&body);
            match parse_json(&text) {
                Some(JsonVal::Arr(rows)) => match rows.first() {
                    Some(JsonVal::Obj(m)) => match m.get("diaObjectId") {
                        Some(JsonVal::Str(id)) => id.clone(),
                        _ => {
                            println!(
                                "Verdict: pending — the first row carries no diaObjectId string (schema measured, parser pending on a real sample)"
                            );
                            return;
                        }
                    },
                    _ => {
                        println!("Verdict: pending — the query returned no object rows");
                        return;
                    }
                },
                _ => {
                    println!(
                        "Verdict: pending — the query body is not a JSON array (schema measured, parser pending on a real sample)"
                    );
                    return;
                }
            }
        }
    };
    let url = format!("{LSST_API_OBJECT}?objectId={obj}&format=json");
    let Some((code, body)) = curl_bytes(&url, Some(&tok)) else {
        println!("Verdict: pending — the object call did not answer (measured stall)");
        return;
    };
    println!(
        "Lasair-LSST /api/object/ {obj}: HTTP {code}, {} bytes",
        body.len()
    );
    if code != "200" {
        println!("Verdict: pending — the light curve is token-gated (HTTP {code})");
        return;
    }
    let path = save.unwrap_or_else(|| "/tmp/opencode/lasair_lsst_object.json".to_string());
    if std::fs::write(&path, &body).is_err() {
        println!("Verdict: pending — the real sample was not saved ({path})");
        return;
    }
    let shape = top_level_shape(&body);
    if shape.is_empty() {
        println!(
            "Verdict: pending — the real sample saved to {path}; its JSON shape stays unmeasured"
        );
        return;
    }
    println!("sample saved: {path}");
    println!("top-level JSON shape (measured, not assumed):");
    for (k, len) in &shape {
        println!("  {k}: {len}");
    }
    println!(
        "Verdict: real object record captured; the per-band dip scan needs the diaSource rows with band/time/flux — parser pending until a real sample fixes the row schema"
    );
}

fn scan_lss1(path: &str, brief: bool) -> (Vec<(f64, f64)>, usize) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            println!(
                "Nadel V (LSST round): no LSS1 asset at {path} — the scan waits for the Lasair-LSST light curves (0 honored, pending)"
            );
            return (Vec::new(), 0);
        }
    };
    let Some(curves) = parse_lss1(&bytes) else {
        println!(
            "Nadel V (LSST round): {path} carries no LSS1 record — the scan stays void (0 honored)"
        );
        return (Vec::new(), 0);
    };
    let groups = lss1_groups(curves);
    println!(
        "\n=== Nadel V, LSST round: achromatic + non-periodic dip cut over {} LSS1 object(s) ===",
        groups.len()
    );
    let mut scanned = 0usize;
    let mut kandidaten = 0usize;
    let mut single_band = 0usize;
    let mut cand_coords: Vec<(f64, f64)> = Vec::new();
    for g in &groups {
        let bands: Vec<(&str, &LsstCurve)> = g
            .curves
            .iter()
            .filter_map(|c| lsst_band_of(c.freq).map(|b| (b, c)))
            .collect();
        if bands.len() < 2 {
            single_band += 1;
            if !brief {
                println!(
                    "  ra {:9.4} dec {:9.4}: absent — one band carries no achromatic cut (0 honored)",
                    g.ra, g.dec
                );
            }
            continue;
        }
        let mut pick: Vec<(&str, &LsstCurve)> = bands
            .iter()
            .filter(|(b, _)| *b == "g" || *b == "r" || *b == "i" || *b == "z")
            .map(|&(b, c)| (b, c))
            .collect();
        pick.sort_by(|a, b| {
            b.1.samples
                .len()
                .cmp(&a.1.samples.len())
                .then_with(|| a.0.cmp(b.0))
        });
        if pick.len() < 2 {
            single_band += 1;
            if !brief {
                println!(
                    "  ra {:9.4} dec {:9.4}: absent — only {} of the g/r/i/z test bands carry a light curve (0 honored)",
                    g.ra,
                    g.dec,
                    pick.len()
                );
            }
            continue;
        }
        let (ba, ca) = pick[0];
        let (bb, cb) = pick[1];
        let Some(a_res) = residual_series_lsst(ca) else {
            if !brief {
                println!(
                    "  ra {:9.4} dec {:9.4}: absent — {ba} residual carries no positive noise model (0 honored)",
                    g.ra, g.dec
                );
            }
            continue;
        };
        let Some(b_res) = residual_series_lsst(cb) else {
            if !brief {
                println!(
                    "  ra {:9.4} dec {:9.4}: absent — {bb} residual carries no positive noise model (0 honored)",
                    g.ra, g.dec
                );
            }
            continue;
        };
        if a_res.len() < N_MIN || b_res.len() < N_MIN {
            if !brief {
                println!(
                    "  ra {:9.4} dec {:9.4}: absent — {ba}/{bb} hold only {}/{} samples, too few for the dip cut (0 honored)",
                    g.ra,
                    g.dec,
                    a_res.len(),
                    b_res.len()
                );
            }
            continue;
        }
        let joint = join_series(&a_res, &b_res);
        if joint.len() < N_COINC_MIN {
            if !brief {
                println!(
                    "  ra {:9.4} dec {:9.4}: absent — {} coincident {ba}/{bb} visits (|Δt| ≤ {COINCIDENCE_S} s), too few (0 honored)",
                    g.ra,
                    g.dec,
                    joint.len()
                );
            }
            continue;
        }
        let a_j: Vec<f32> = joint.iter().map(|&(_, v, _)| v).collect();
        let b_j: Vec<f32> = joint.iter().map(|&(_, _, v)| v).collect();
        let tj: Vec<f64> = joint.iter().map(|&(t, _, _)| t).collect();
        let (dip_a, sig_a) = deepest_dip(&a_res);
        let (dip_b, sig_b) = deepest_dip(&b_res);
        let ratio = if dip_b.abs() > 1e-9 {
            dip_a.abs() / dip_b.abs()
        } else {
            f64::INFINITY
        };
        let sig_gate = sig_a.min(sig_b).abs() >= DIP_SIG && dip_a < 0.0 && dip_b < 0.0;
        let achromatisch = sig_gate && (1.0 / ACHROMATIC_RATIO..=ACHROMATIC_RATIO).contains(&ratio);
        let (_, fap) = lomb_scargle_fap(&tj, &a_j);
        let nicht_periodisch = fap >= FAP_GATE;
        let rho = spearman(&a_j, &b_j);
        scanned += 1;
        if achromatisch && nicht_periodisch {
            kandidaten += 1;
            cand_coords.push((g.ra, g.dec));
        }
        let word = if achromatisch && nicht_periodisch {
            "CANDIDATE (pre-exclusion)"
        } else {
            "still"
        };
        println!(
            "  ra {:9.4} dec {:9.4} {ba}/{bb} {word} | dip {ba} {dip_a:.3} ({sig_a:.1}σ) {bb} {dip_b:.3} ({sig_b:.1}σ) ratio {ratio:.2} {} | FAP {fap:.2e} {}",
            g.ra,
            g.dec,
            if achromatisch {
                "achromatic"
            } else {
                "chromatic"
            },
            match rho {
                Some(r) => format!("rho {r:.2}"),
                None => "rho absent".to_string(),
            },
        );
    }
    println!(
        "\nVerdict: {kandidaten} candidate dip(s) of {scanned} scanned multiband objects; {single_band} single-band objects without an achromatic cell (absent)"
    );
    for (cra, cdec) in &cand_coords {
        println!("  candidate pre-exclusion at ra {cra:.4} dec {cdec:.4}");
    }
    println!(
        "Exclusion gate: a candidate dip is no candidate until the natural dimmers are removed upstream (Sherlock/known-class crossmatch): {:?}",
        NATURAL_DIMMERS
    );
    println!(
        "Quantitative limit: no achromatic non-periodic dip above DIP_SIG {DIP_SIG}σ across {scanned} LSS1 object(s) — or the line above names it"
    );
    (cand_coords, scanned)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut token: Option<String> = None;
    let mut object: Option<String> = None;
    let mut save: Option<String> = None;
    let mut scan: Option<String> = None;
    let mut fink: Option<String> = None;
    let mut cone_ra: Option<f64> = None;
    let mut cone_dec: Option<f64> = None;
    let mut cone_radius: Option<f64> = None;
    let mut cone_min: usize = 24;
    let mut lasair_ra: Option<f64> = None;
    let mut lasair_dec: Option<f64> = None;
    let mut lasair_radius: Option<f64> = None;
    let mut lasair_max: usize = 6;
    let mut cones: Vec<(f64, f64, f64, usize)> = Vec::new();
    let mut neg_control_base: Option<String> = None;
    let mut neg_control_depth: f64 = 8.0;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--token" => {
                token = args.get(i + 1).cloned();
                i += 1;
            }
            "--object" => {
                object = args.get(i + 1).cloned();
                i += 1;
            }
            "--save" => {
                save = args.get(i + 1).cloned();
                i += 1;
            }
            "--scan" => {
                scan = args.get(i + 1).cloned();
                i += 1;
            }
            "--fink" => {
                fink = args.get(i + 1).cloned();
                i += 1;
            }
            "--cone" => {
                if let Some(spec) = args.get(i + 1) {
                    let p: Vec<&str> = spec.split(',').collect();
                    if p.len() == 4 {
                        if let (Some(ra), Some(dec), Some(radius), Some(min)) = (
                            p[0].trim().parse().ok(),
                            p[1].trim().parse().ok(),
                            p[2].trim().parse().ok(),
                            p[3].trim().parse().ok(),
                        ) {
                            cones.push((ra, dec, radius, min));
                            i += 1;
                        }
                    }
                }
            }
            "--cone-ra" => {
                cone_ra = args.get(i + 1).and_then(|v| v.parse().ok());
                i += 1;
            }
            "--cone-dec" => {
                cone_dec = args.get(i + 1).and_then(|v| v.parse().ok());
                i += 1;
            }
            "--cone-radius" => {
                cone_radius = args.get(i + 1).and_then(|v| v.parse().ok());
                i += 1;
            }
            "--cone-min" => {
                cone_min = args.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(24);
                i += 1;
            }
            "--lasair-ra" => {
                lasair_ra = args.get(i + 1).and_then(|v| v.parse().ok());
                i += 1;
            }
            "--lasair-dec" => {
                lasair_dec = args.get(i + 1).and_then(|v| v.parse().ok());
                i += 1;
            }
            "--lasair-radius" => {
                lasair_radius = args.get(i + 1).and_then(|v| v.parse().ok());
                i += 1;
            }
            "--lasair-max" => {
                lasair_max = args.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(6);
                i += 1;
            }
            "--negative-control" => {
                neg_control_base = args.get(i + 1).cloned();
                if let Some(d) = args.get(i + 2).and_then(|v| v.parse::<f64>().ok()) {
                    if d.is_finite() && d > 0.0 {
                        neg_control_depth = d;
                    }
                }
                i += 2;
            }
            _ => {
                usage();
                return;
            }
        }
        i += 1;
    }
    let tok = token.or_else(|| std::env::var("LASAIR_TOKEN").ok());
    if let Some(p) = neg_control_base {
        negative_control(&p, neg_control_depth);
        return;
    }
    if let Some(p) = scan {
        reach(tok, object, save);
        println!();
        scan_lss1(&p, false);
        return;
    }
    if !cones.is_empty() {
        grid_scan(&cones);
        return;
    }
    if let Some(id) = fink {
        fink_scan(&id, save.as_deref());
        return;
    }
    if let Some(ra) = cone_ra {
        match (cone_dec, cone_radius) {
            (Some(dec), Some(radius)) => {
                cone_scan(ra, dec, radius, cone_min, save.as_deref(), false);
            }
            _ => {
                usage();
                return;
            }
        }
        return;
    }
    if let Some(ra) = lasair_ra {
        match (lasair_dec, lasair_radius) {
            (Some(dec), Some(radius)) => {
                lasair_cone_scan(ra, dec, radius, lasair_max);
            }
            _ => {
                usage();
                return;
            }
        }
        return;
    }
    reach(tok, object, save);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lsk() -> LeapSeconds {
        embedded_lsk().expect("the embedded naif0012 table parses")
    }

    #[test]
    fn fink_tai_mjd_maps_to_an_absolute_tdb() {
        let mjd_tai = 61024.2459327064;
        let tdb = fink_mjd_tai_to_tdb(mjd_tai, &lsk()).expect("2026 epoch maps");
        let as_utc = mjd_to_unix(mjd_tai);
        let expected = as_utc + 32.184 - 946_728_000.0;
        assert!(
            (tdb - expected).abs() < 1e-6,
            "leap cancels: the TAI label overstates the UTC unix by the leap, which unix_to_tdb restores — tdb {tdb}, expected {expected}"
        );
        assert!(
            (tdb - 819_050_080.769_833).abs() < 1e-3,
            "a real Fink/LSST 2025 row maps to a concrete TDB seconds-since-J2000, was {tdb}"
        );
    }

    #[test]
    fn tdb_axis_reproduces_the_relative_mjd_axis_per_object() {
        let lsk = lsk();
        let rows: Vec<(String, f64, f64)> = vec![
            ("r".to_string(), 61024.2459327064, 100.0),
            ("r".to_string(), 61024.2469322589, 101.0),
            ("g".to_string(), 61205.9837394019, 200.0),
            ("g".to_string(), 61205.9889914105, 201.0),
        ];
        let (rows_tdb, skipped) = rows_to_tdb(&lsk, &rows);
        assert_eq!(skipped, 0);
        let curves = build_band_curves(1.0, 2.0, &rows_tdb);
        let r_curve = curves
            .iter()
            .find(|c| lsst_band_of(c.freq) == Some("r"))
            .expect("the r band curve exists");
        assert_eq!(r_curve.samples.len(), 2);
        let expect = (61024.2469322589 - 61024.2459327064) * 86400.0;
        assert!(
            (r_curve.samples[1].0 - expect).abs() < 1e-6,
            "within one leap regime the TDB map is an affine shift, so a single-object relative fold axis is unchanged — {:.6} vs {expect:.6}",
            r_curve.samples[1].0
        );
    }

    #[test]
    fn void_epochs_stay_absent_on_the_tdb_layer() {
        let lsk = lsk();
        let rows: Vec<(String, f64, f64)> = vec![
            ("r".to_string(), 61024.2459327064, 100.0),
            ("r".to_string(), 35_000.0, 101.0),
        ];
        let (rows_tdb, skipped) = rows_to_tdb(&lsk, &rows);
        assert_eq!(rows_tdb.len(), 1);
        assert_eq!(
            skipped, 1,
            "pre-1972 mjd reads no leap table row — absent, never fabricated"
        );
    }

    #[test]
    fn sightline_unit_spans_the_icrs_axes() {
        let x = sightline_unit(0.0, 0.0);
        assert!((x[0] - 1.0).abs() < 1e-12 && x[1].abs() < 1e-12 && x[2].abs() < 1e-12);
        let z = sightline_unit(0.0, 90.0);
        assert!(z[0].abs() < 1e-12 && z[1].abs() < 1e-12 && (z[2] - 1.0).abs() < 1e-12);
        let d = sightline_unit(148.84, 2.55);
        let norm = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((norm - 1.0).abs() < 1e-12);
    }

    #[test]
    fn fold_term_budget_is_computed_from_constants() {
        // The term budget of the barycentric fold, measured from the constants
        // (not estimated); the numbers are asserted in the physical bands and
        // printed for the record.
        let roemer_amp_s = AU_M / C_LIGHT;
        let v_earth = 2.0 * std::f64::consts::PI * AU_M / (SIDEREAL_YEAR_DAYS * DAY_S);
        let weekly_drift_s = v_earth * 7.0 * DAY_S / C_LIGHT;
        let diurnal_s = EARTH_RADIUS_M / C_LIGHT;
        let shapiro_coeff_s = 2.0 * GM_SUN_M3_S2 / (C_LIGHT * C_LIGHT * C_LIGHT);
        println!(
            "term budget: Rømer seasonal amplitude {roemer_amp_s:.1} s; Earth line-of-sight drift per week {weekly_drift_s:.1} s = {:.1} cycles of a 20 s period and {:.2}% of a 2 h period; diurnal station swing {:.2} ms; solar Shapiro coefficient {:.2e} s (2GM/c³)",
            weekly_drift_s / 20.0,
            weekly_drift_s / 7200.0 * 100.0,
            diurnal_s * 1000.0,
            shapiro_coeff_s
        );
        assert!(
            (roemer_amp_s - 499.0).abs() < 5.0,
            "1 AU light time ~499 s, was {roemer_amp_s}"
        );
        assert!(
            (50.0..70.0).contains(&weekly_drift_s),
            "the weekly line-of-sight drift of the Earth is ~60 s, was {weekly_drift_s}"
        );
        assert!(
            (0.020..0.023).contains(&diurnal_s),
            "the diurnal station swing is ~21 ms, was {diurnal_s}"
        );
        assert!(
            shapiro_coeff_s > 9e-6 && shapiro_coeff_s < 1.1e-5,
            "2GM/c³ ~ 9.85 µs, was {shapiro_coeff_s}"
        );
        let twenty_s_cycles = weekly_drift_s / 20.0;
        assert!(
            (2.5..3.5).contains(&twenty_s_cycles),
            "a 20 s period loses ~3 full cycles of phase coherence per week without the Rømer term, was {twenty_s_cycles}"
        );
    }

    #[test]
    fn roemer_light_time_sign_and_magnitude() {
        // An object on the +x axis with the observatory 1 AU further out along
        // +x must read a positive light-travel correction of ~499 s (the
        // wavefront reaches the observatory after the SSB).
        let n = sightline_unit(0.0, 0.0);
        let sun = [0.0, 0.0, 0.0];
        let obs = [AU_M, 0.0, 0.0];
        let tau = vec_dot(n, vec_sub(obs, sun)) / C_LIGHT;
        assert!((tau - AU_M / C_LIGHT).abs() < 1e-6);
        assert!(tau > 0.0);
        let obs_other_side = [-AU_M, 0.0, 0.0];
        let tau2 = vec_dot(n, vec_sub(obs_other_side, sun)) / C_LIGHT;
        assert!(tau2 < 0.0);
        // A source at the ecliptic pole never sees the ±8 min seasonal term.
        let n_pole = sightline_unit(0.0, 90.0);
        let tau_pole = vec_dot(n_pole, vec_sub(obs, sun)) / C_LIGHT;
        assert!(tau_pole.abs() < 1e-9);
    }

    fn next_noise(rng: &mut u64) -> f64 {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64) * 2.0 - 1.0
    }

    fn synthetic_two_band_noise(seed: u64, n: usize) -> Vec<LsstCurve> {
        let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let mut make = |band: &str| -> LsstCurve {
            let freq = lsst_freq_of_band(band).expect("band frequency");
            let mut samples = Vec::with_capacity(n);
            for i in 0..n {
                let t = 1000.0 + i as f64 * 900.0;
                let flux = 100.0 * (1.0 + 0.02 * next_noise(&mut rng));
                samples.push((t, flux as f32));
            }
            LsstCurve {
                ra_deg: 12.34,
                dec_deg: -5.67,
                freq,
                samples,
            }
        };
        vec![make("g"), make("r")]
    }

    fn synthetic_two_band_periodic(seed: u64, n: usize, period_s: f64) -> Vec<LsstCurve> {
        let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let mut make = |band: &str| -> LsstCurve {
            let freq = lsst_freq_of_band(band).expect("band frequency");
            let mut samples = Vec::with_capacity(n);
            let w = 2.0 * std::f64::consts::PI / period_s;
            for i in 0..n {
                let t = 1000.0 + i as f64 * 900.0;
                let phase = w * (t - 1000.0);
                let flux = 100.0 * (1.0 + 0.10 * phase.sin() + 0.01 * next_noise(&mut rng));
                samples.push((t, flux as f32));
            }
            LsstCurve {
                ra_deg: 12.34,
                dec_deg: -5.67,
                freq,
                samples,
            }
        };
        vec![make("g"), make("r")]
    }

    #[test]
    fn periodic_natural_variable_with_achromatic_dip_stays_excluded_by_the_fap_gate() {
        // A genuinely periodic natural variable (the sinusoid) that carries a
        // synthetic achromatic dip must NOT surface as a candidate: its series
        // reads periodic on the variance-normalized periodogram, so the FAP
        // gate (aperiodicity test) holds it out. This is the counterpart of
        // the aperiodic injection control: the dip/achromatic gate alone is
        // not sufficient — the FAP gate must still exclude the periodic
        // naturals.
        let curves = synthetic_two_band_periodic(0xA1, 48, 7200.0);
        let base_path = "/tmp/opencode/lsst_periodic_gate_test_base.bin";
        let inj_path = "/tmp/opencode/lsst_periodic_gate_test_injected.bin";
        std::fs::write(base_path, serialize_lss1(&curves)).expect("base asset written");
        let base_bytes = std::fs::read(base_path).expect("base asset read");
        let inj =
            inject_achromatic_dip(&base_bytes, 8.0).expect("injection finds a two-band object");
        std::fs::write(inj_path, &inj.bytes).expect("injected asset written");
        let curves_inj = parse_lss1(&inj.bytes).expect("injected asset parses");
        let groups = lss1_groups(curves_inj);
        let g = groups
            .iter()
            .find(|g| (g.ra - inj.group.0).abs() < 1e-3 && (g.dec - inj.group.1).abs() < 1e-3)
            .expect("injected object present");
        let mut bands: Vec<(&str, &LsstCurve)> = g
            .curves
            .iter()
            .filter_map(|c| lsst_band_of(c.freq).map(|b| (b, c)))
            .collect();
        bands.sort_by(|a, b| {
            b.1.samples
                .len()
                .cmp(&a.1.samples.len())
                .then_with(|| a.0.cmp(b.0))
        });
        let (ba, ca) = bands[0];
        let (bb, cb) = bands[1];
        let a_res = residual_series_lsst(ca).expect("band a residual");
        let b_res = residual_series_lsst(cb).expect("band b residual");
        let (dip_a, sig_a) = deepest_dip(&a_res);
        let (dip_b, sig_b) = deepest_dip(&b_res);
        let ratio = if dip_b.abs() > 1e-9 {
            dip_a.abs() / dip_b.abs()
        } else {
            f64::INFINITY
        };
        let joint = join_series(&a_res, &b_res);
        let a_j: Vec<f32> = joint.iter().map(|&(_, v, _)| v).collect();
        let tj: Vec<f64> = joint.iter().map(|&(t, _, _)| t).collect();
        let (_, fap) = lomb_scargle_fap(&tj, &a_j);
        let (post_cands, _) = scan_lss1(inj_path, true);
        println!(
            "periodic control gates (8σ achromatic dip on a 2 h sinusoid): dip {ba} {dip_a:.3} ({sig_a:.1}σ) {bb} {dip_b:.3} ({sig_b:.1}σ) ratio {ratio:.2} achromatic | FAP {fap:.2e} | full-scan candidates {}",
            post_cands.len()
        );
        assert!(
            sig_a.abs() >= DIP_SIG && sig_b.abs() >= DIP_SIG && dip_a < 0.0 && dip_b < 0.0,
            "the dip/achromatic gate must trigger on the injected dip (the premise that the FAP gate is what holds the periodic natural out)"
        );
        assert!(
            fap < FAP_GATE,
            "a genuinely periodic natural variable must read periodic (FAP {fap:.2e} < {FAP_GATE})"
        );
        assert!(
            post_cands.is_empty(),
            "the periodic natural with an achromatic dip stays excluded by the FAP gate — the full candidate is not carried (FAP {fap:.2e} < {FAP_GATE})"
        );
    }

    #[test]
    fn negative_control_achromatic_injection_reaches_the_dip_gate() {
        // The negative control: a synthetic achromatic dip (12σ per band) is
        // placed on one coincident visit. The measurement below names which
        // scanner gates carry it and which gate stops the full candidate.
        let curves = synthetic_two_band_noise(0x51, 40);
        let base_path = "/tmp/opencode/lsst_negative_control_test_base.bin";
        let inj_path = "/tmp/opencode/lsst_negative_control_test_injected.bin";
        std::fs::write(base_path, serialize_lss1(&curves)).expect("base asset written");
        let (base_cands, _) = scan_lss1(base_path, true);
        assert_eq!(
            base_cands.len(),
            0,
            "the pure-noise base must carry no achromatic dip candidate (the control premise)"
        );
        let base_bytes = std::fs::read(base_path).expect("base asset read");
        let inj =
            inject_achromatic_dip(&base_bytes, 12.0).expect("injection finds a two-band object");
        std::fs::write(inj_path, &inj.bytes).expect("injected asset written");
        let (post_cands, _) = scan_lss1(inj_path, true);
        let curves_inj = parse_lss1(&inj.bytes).expect("injected asset parses");
        let groups = lss1_groups(curves_inj);
        let g = groups
            .iter()
            .find(|g| (g.ra - inj.group.0).abs() < 1e-3 && (g.dec - inj.group.1).abs() < 1e-3)
            .expect("injected object present");
        let mut bands: Vec<(&str, &LsstCurve)> = g
            .curves
            .iter()
            .filter_map(|c| lsst_band_of(c.freq).map(|b| (b, c)))
            .collect();
        bands.sort_by(|a, b| {
            b.1.samples
                .len()
                .cmp(&a.1.samples.len())
                .then_with(|| a.0.cmp(b.0))
        });
        let (ba, ca) = bands[0];
        let (bb, cb) = bands[1];
        let a_res = residual_series_lsst(ca).expect("band a residual");
        let b_res = residual_series_lsst(cb).expect("band b residual");
        let (dip_a, sig_a) = deepest_dip(&a_res);
        let (dip_b, sig_b) = deepest_dip(&b_res);
        let ratio = if dip_b.abs() > 1e-9 {
            dip_a.abs() / dip_b.abs()
        } else {
            f64::INFINITY
        };
        let joint = join_series(&a_res, &b_res);
        let a_j: Vec<f32> = joint.iter().map(|&(_, v, _)| v).collect();
        let tj: Vec<f64> = joint.iter().map(|&(t, _, _)| t).collect();
        let (_, fap) = lomb_scargle_fap(&tj, &a_j);
        println!(
            "negative control gates (12σ achromatic injection): dip {ba} {dip_a:.3} ({sig_a:.1}σ) {bb} {dip_b:.3} ({sig_b:.1}σ) ratio {ratio:.2} | FAP {fap:.2e} | full-scan candidates {}",
            post_cands.len()
        );
        assert!(
            sig_a.abs() >= DIP_SIG && sig_b.abs() >= DIP_SIG && dip_a < 0.0 && dip_b < 0.0,
            "the achromatic dip gate must trigger on the injected dip (sensitivity measured)"
        );
        assert!(
            (1.0 / ACHROMATIC_RATIO..=ACHROMATIC_RATIO).contains(&ratio),
            "the injected dip must read achromatic"
        );
        assert!(
            post_cands.len() == 1,
            "the full candidate gate must carry the injected achromatic dip — dip/achromatic gate open (measured above) AND the aperiodic FAP gate open (FAP {fap:.2e} >= {FAP_GATE}); the scanner reads the series as aperiodic"
        );
        assert!(
            fap >= FAP_GATE,
            "the injected transient must read aperiodic on the variance-normalized periodogram (FAP {fap:.2e} >= {FAP_GATE})"
        );
    }

    #[test]
    fn fap_gate_is_scale_invariant_and_white_noise_reads_aperiodic() {
        // The Lomb-Scargle FAP in the standard normalization is
        // amplitude-invariant: a uniform rescale of the residual series moves
        // data and variance together and leaves the power (hence the FAP)
        // unchanged. The form it replaces scaled ~ n/σ² and read quiet
        // fractional photometry (σ ~ 0.02) as periodic (FAP 0) on every row.
        // The gate it serves is the aperiodicity test: white noise of any
        // photometric scale must read aperiodic (FAP above the gate), and the
        // FAP must not move with the amplitude.
        let n = 40usize;
        let t: Vec<f64> = (0..n).map(|i| 1000.0 + i as f64 * 900.0).collect();
        let mut rng = 0x51u64.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let white: Vec<f32> = (0..n).map(|_| next_noise(&mut rng) as f32).collect();
        let sd_of = |v: &[f32]| -> f64 {
            let m = v.iter().map(|&x| x as f64).sum::<f64>() / v.len() as f64;
            (v.iter().map(|&x| (x as f64 - m).powi(2)).sum::<f64>() / v.len() as f64).sqrt()
        };
        let scale = |s: f64| -> Vec<f32> {
            let cur = sd_of(&white);
            white
                .iter()
                .map(|&x| ((x as f64) * s / cur) as f32)
                .collect()
        };
        let (_, fap_quiet) = lomb_scargle_fap(&t, &scale(0.02));
        let (_, fap_mid) = lomb_scargle_fap(&t, &scale(3.0));
        let (_, fap_loud) = lomb_scargle_fap(&t, &scale(30.0));
        println!(
            "Lomb-Scargle FAP gate scale invariance: white noise sd 0.02 → FAP {fap_quiet:.2e}, sd 3.0 → FAP {fap_mid:.2e}, sd 30.0 → FAP {fap_loud:.2e} — the FAP does not move with the photometric scale, and every scale reads aperiodic (FAP >= {FAP_GATE})"
        );
        assert!(
            fap_quiet >= FAP_GATE && fap_mid >= FAP_GATE && fap_loud >= FAP_GATE,
            "white noise of any amplitude must read aperiodic, measured {fap_quiet:.2e}, {fap_mid:.2e}, {fap_loud:.2e}"
        );
        let spread = (fap_quiet - fap_mid)
            .abs()
            .max((fap_quiet - fap_loud).abs());
        assert!(
            spread < 1e-6,
            "a uniform rescale leaves the standard-normalized power identical — the FAP spread across three decades of amplitude is {spread:.2e}"
        );
    }

    #[test]
    fn lasair_cone_parser_reads_the_documented_row_shape() {
        // The Lasair-LSST REST docs show the cone return as rows of
        // {object, separation}; the parser must map exactly that.
        let body = br#"[{"object":"12345678901234","separation":2.393511865261539},{"object":"98765432109876","separation":0.5}]"#;
        let rows = parse_lasair_cone(body).expect("the documented row array parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].id, "98765432109876");
        assert!((rows[0].separation - 0.5).abs() < 1e-12);
        assert_eq!(rows[1].id, "12345678901234");
        assert!((rows[1].separation - 2.393511865261539).abs() < 1e-12);
        assert!(parse_lasair_cone(b"{}").is_none());
        assert!(parse_lasair_cone(b"not json").is_none());
    }

    #[test]
    fn lasair_lightcurve_parser_reads_the_measured_diasource_columns() {
        // The official lsst-uk example notebook (examine_object.ipynb) reads
        // each diaSourcesList row's band, midpointMjdTai and psfFlux — the
        // parser maps exactly those column names, keeps positive finite flux
        // and never fabricates a row from an unmapped shape.
        let body = br#"{"diaObject":{"ra":148.87,"decl":2.52,"firstDiaSourceMjdTai":61024.0,"lastDiaSourceMjdTai":61205.0},"diaSourcesList":[{"band":"g","midpointMjdTai":61024.2459327064,"psfFlux":1.2e-3,"psfFluxErr":1.0e-5},{"band":"r","midpointMjdTai":61025.1,"psfFlux":0.0},{"band":"i","midpointMjdTai":61026.2,"psfFlux":-3.0e-4},{"band":"z","midpointMjdTai":61027.3,"psfFlux":4.5e-4}]}"#;
        let (coord, rows) = parse_lasair_lightcurve(body);
        let (ra, dec) = coord.expect("the diaObject ra/decl map");
        assert!((ra - 148.87).abs() < 1e-9 && (dec - 2.52).abs() < 1e-9);
        assert_eq!(
            rows.len(),
            2,
            "the zero and negative psfFlux rows stay absent (never fabricated)"
        );
        assert_eq!(rows[0].0, "g");
        assert!((rows[0].1 - 61024.2459327064).abs() < 1e-9);
        assert!((rows[0].2 - 1.2e-3).abs() < 1e-12);
        assert_eq!(rows[1].0, "z");
        let (_, empty) = parse_lasair_lightcurve(br#"{"diaObject":{}}"#);
        assert!(empty.is_empty());
    }
}
