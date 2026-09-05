use omegaflow::archivar::C_LIGHT;
use omegaflow::json::{parse_json, JsonVal};
use std::collections::HashMap;
use std::process::Command;

const LSST_ROOT: &str = "https://lasair.lsst.ac.uk/";
const LSST_API: &str = "https://lasair.lsst.ac.uk/api/query/";
const LSST_API_OBJECT: &str = "https://lasair.lsst.ac.uk/api/object/";
const FINK_API_SOURCES: &str = "https://api.lsst.fink-portal.org/api/v1/sources";
const FINK_BAND: &str = "r:band";
const FINK_MJD: &str = "r:midpointMjdTai";
const FINK_FLUX: &str = "r:scienceFlux";
const FINK_RA: &str = "r:ra";
const FINK_DEC: &str = "r:dec";
const UA: &str = "omegaflow-nadel-v-lsst-scan/1.0";

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

struct LsstCurve {
    ra_deg: f64,
    dec_deg: f64,
    freq: f64,
    samples: Vec<(f64, f32)>,
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

fn lomb_scargle_fap(t: &[f64], r: &[f32]) -> (f64, f64) {
    let n = t.len();
    if n < 8 {
        return (0.0, 1.0);
    }
    let (mean, sd) = mean_sd(&r.iter().map(|&x| x as f64).collect::<Vec<f64>>());
    if sd <= 0.0 {
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
        let mut c = 0.0f64;
        let mut s = 0.0f64;
        for &ti in t {
            c += (w * ti).cos();
            s += (w * ti).sin();
        }
        let tau = 0.5 * (2.0 * s / c).atan();
        let mut num = 0.0f64;
        let mut den = 0.0f64;
        for (i, &ti) in t.iter().enumerate() {
            let x = r[i] as f64 - mean;
            let ph = w * (ti - tau);
            num += x * ph.cos();
            den += x * x * ph.sin() * ph.sin();
        }
        if den > 0.0 {
            let z = 0.5 * n as f64 * num * num / (sd * sd * den);
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
    let Ok(text) = std::str::from_utf8(&body) else {
        println!(
            "Fink/LSST {id}: the sample is not UTF-8 text — parser pending on the real schema"
        );
        return;
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        println!(
            "Fink/LSST {id}: the sample is not a JSON array — parser pending on the real schema"
        );
        return;
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
    if rows_ok.is_empty() {
        println!("Fink/LSST {id}: no row carries the full band/time/flux triple (absent)");
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
    let mut t_min = f64::INFINITY;
    for (band, mjd, _) in &rows_ok {
        *per_band.entry(band.clone()).or_insert(0) += 1;
        if *mjd < t_min {
            t_min = *mjd;
        }
    }
    let mut band_counts: Vec<(&str, usize)> =
        per_band.iter().map(|(b, n)| (b.as_str(), *n)).collect();
    band_counts.sort();
    for (b, n) in &band_counts {
        println!("  Fink/LSST band {b}: {n} measurement rows");
    }
    let mut curves: Vec<LsstCurve> = Vec::new();
    for (name, _) in LSST_LAMBDA_NM {
        let Some(freq) = lsst_freq_of_band(name) else {
            continue;
        };
        let mut samples: Vec<(f64, f32)> = rows_ok
            .iter()
            .filter(|(b, _, _)| b == name)
            .map(|(_, mjd, flux)| ((mjd - t_min) * 86400.0, *flux as f32))
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
    scan_lss1(&bin_path);
}

fn usage() {
    eprintln!(
        "lsst_anomaly_probe — Nadel V over the Lasair-LSST broker\n\
         reachability (default):\n\
         \x20 lsst_anomaly_probe [--object <diaObjectId>] [--token <t>] [--save <path.json>]\n\
         Fink/LSST anonymous light curve (api.lsst.fink-portal.org), no token:\n\
         \x20 lsst_anomaly_probe --fink <diaObjectId> [--save <raw.json>]\n\
         scan_lss1 of a real LSS1 light-curve asset:\n\
         \x20 lsst_anomaly_probe --scan <lsst_lightcurves.bin>\n\
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
                println!("Verdict: pending — the query answered HTTP {code} (expect a 401 without a valid token)");
                return;
            }
            let text = String::from_utf8_lossy(&body);
            match parse_json(&text) {
                Some(JsonVal::Arr(rows)) => match rows.first() {
                    Some(JsonVal::Obj(m)) => match m.get("diaObjectId") {
                        Some(JsonVal::Str(id)) => id.clone(),
                        _ => {
                            println!("Verdict: pending — the first row carries no diaObjectId string (schema measured, parser pending on a real sample)");
                            return;
                        }
                    },
                    _ => {
                        println!("Verdict: pending — the query returned no object rows");
                        return;
                    }
                },
                _ => {
                    println!("Verdict: pending — the query body is not a JSON array (schema measured, parser pending on a real sample)");
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

fn scan_lss1(path: &str) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            println!(
                "Nadel V (LSST round): no LSS1 asset at {path} — the scan waits for the Lasair-LSST light curves (0 honored, pending)"
            );
            return;
        }
    };
    let Some(curves) = parse_lss1(&bytes) else {
        println!(
            "Nadel V (LSST round): {path} carries no LSS1 record — the scan stays void (0 honored)"
        );
        return;
    };
    struct Group {
        ra: f64,
        dec: f64,
        curves: Vec<LsstCurve>,
    }
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
    println!(
        "\n=== Nadel V, LSST round: achromatic + non-periodic dip cut over {} LSS1 object(s) ===",
        groups.len()
    );
    let mut scanned = 0usize;
    let mut kandidaten = 0usize;
    let mut single_band = 0usize;
    for g in &groups {
        let bands: Vec<(&str, &LsstCurve)> = g
            .curves
            .iter()
            .filter_map(|c| lsst_band_of(c.freq).map(|b| (b, c)))
            .collect();
        if bands.len() < 2 {
            single_band += 1;
            println!(
                "  ra {:9.4} dec {:9.4}: absent — one band carries no achromatic cut (0 honored)",
                g.ra, g.dec
            );
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
            println!(
                "  ra {:9.4} dec {:9.4}: absent — only {} of the g/r/i/z test bands carry a light curve (0 honored)",
                g.ra,
                g.dec,
                pick.len()
            );
            continue;
        }
        let (ba, ca) = pick[0];
        let (bb, cb) = pick[1];
        let Some(a_res) = residual_series_lsst(ca) else {
            println!(
                "  ra {:9.4} dec {:9.4}: absent — {ba} residual carries no positive noise model (0 honored)",
                g.ra, g.dec
            );
            continue;
        };
        let Some(b_res) = residual_series_lsst(cb) else {
            println!(
                "  ra {:9.4} dec {:9.4}: absent — {bb} residual carries no positive noise model (0 honored)",
                g.ra, g.dec
            );
            continue;
        };
        if a_res.len() < N_MIN || b_res.len() < N_MIN {
            println!(
                "  ra {:9.4} dec {:9.4}: absent — {ba}/{bb} hold only {}/{} samples, too few for the dip cut (0 honored)",
                g.ra,
                g.dec,
                a_res.len(),
                b_res.len()
            );
            continue;
        }
        let joint = join_series(&a_res, &b_res);
        if joint.len() < N_COINC_MIN {
            println!(
                "  ra {:9.4} dec {:9.4}: absent — {} coincident {ba}/{bb} visits (|Δt| ≤ {COINCIDENCE_S} s), too few (0 honored)",
                g.ra,
                g.dec,
                joint.len()
            );
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
    println!("\nVerdict: {kandidaten} candidate dip(s) of {scanned} scanned multiband objects; {single_band} single-band objects without an achromatic cell (absent)");
    println!(
        "Exclusion gate: a candidate dip is no candidate until the natural dimmers are removed upstream (Sherlock/known-class crossmatch): {:?}",
        NATURAL_DIMMERS
    );
    println!(
        "Quantitative limit: no achromatic non-periodic dip above DIP_SIG {DIP_SIG}σ across {scanned} LSS1 object(s) — or the line above names it"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut token: Option<String> = None;
    let mut object: Option<String> = None;
    let mut save: Option<String> = None;
    let mut scan: Option<String> = None;
    let mut fink: Option<String> = None;
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
            _ => {
                usage();
                return;
            }
        }
        i += 1;
    }
    let tok = token.or_else(|| std::env::var("LASAIR_TOKEN").ok());
    if let Some(p) = scan {
        reach(tok, object, save);
        println!();
        scan_lss1(&p);
        return;
    }
    if let Some(id) = fink {
        fink_scan(&id, save.as_deref());
        return;
    }
    reach(tok, object, save);
}
