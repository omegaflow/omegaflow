use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use omegaflow::archivar::gic::{GIC_FAMILY_NAMES as FAMILY_NAMES, family_of};

const EPOCH_YEAR: f64 = 2025.0;
const HEIGHT_KM: f64 = 0.0;
const CGM_URL: &str = "https://omniweb.gsfc.nasa.gov/cgi/vitmo/cgm_model.cgi";
const SUPERMAG_URL: &str =
    "https://raw.githubusercontent.com/spacecataz/supermag/master/station_info.txt";
const DEFAULT_OUT: &str = "state/river/gic-cgm-lat.tsv";
const DEFAULT_PAUSE_MS: u64 = 150;
const FAMILY_CHANNEL_FIELD: &str = "intermagnet_xyz_x_nt";

struct Station {
    code: String,
    lat: f64,
    lon: f64,
    elev: Option<f64>,
}

fn repo_root() -> String {
    match env::var("OMEGAFLOW_REPO") {
        Ok(v) => v,
        Err(_) => ".".to_string(),
    }
}

fn parse_stations(text: &str) -> BTreeMap<String, Station> {
    let mut out = BTreeMap::new();
    for block in text.split("\n\n") {
        if !block.contains("GIN_V1") {
            continue;
        }
        let mut code = None;
        let mut earth = None;
        for line in block.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("station ") {
                code = Some(rest.trim().to_string());
            } else if let Some(rest) = line.strip_prefix("on earth ") {
                let f: Vec<f64> = rest
                    .split_whitespace()
                    .filter_map(|t| t.parse().ok())
                    .collect();
                if f.len() >= 2 && f[0].is_finite() && f[1].is_finite() {
                    earth = Some((f[0], f[1], f.get(2).copied()));
                }
            }
        }
        if let (Some(code), Some((lat, lon, elev))) = (code, earth) {
            out.insert(
                code.clone(),
                Station {
                    code,
                    lat,
                    lon,
                    elev,
                },
            );
        }
    }
    out
}

fn curl_get(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-A")
        .arg("omegaflow-cgm-lat/1.0")
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url)
        .output()
        .ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    if code != "200" {
        eprintln!("curl_get: HTTP {code} for {url}");
        return None;
    }
    Some(String::from_utf8_lossy(&stdout[..idx]).to_string())
}

fn cgm_of(lat: f64, lon: f64) -> Option<(f64, f64)> {
    let url = format!(
        "{CGM_URL}?model=cgm&year={EPOCH_YEAR}&height={HEIGHT_KM}&geo_flag=1&latitude={lat}&longitude={lon}&profile=1&start=0.&stop=0.&step=1.&format=0&vars=04&vars=05"
    );
    for attempt in 0..3 {
        if let Some((la, lo)) = parse_cgm(&url) {
            return Some((la, lo));
        }
        sleep(Duration::from_millis(800 * (attempt + 1)));
    }
    None
}

fn parse_cgm(url: &str) -> Option<(f64, f64)> {
    let body = curl_get(url)?;
    let mut last: Option<(f64, f64)> = None;
    for line in body.lines() {
        let t = line.trim();
        let f: Vec<f64> = t
            .split_whitespace()
            .filter_map(|x| x.parse::<f64>().ok())
            .collect();
        if f.len() == 2 && f[0].is_finite() && f[1].is_finite() {
            last = Some((f[0], f[1]));
        }
    }
    last
}

fn supermag_mlat(text: &str) -> BTreeMap<String, (f64, f64)> {
    let mut m = BTreeMap::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() >= 5 {
            let code = f[0].trim();
            if code.len() == 3 && code.chars().all(|c| c.is_ascii_uppercase()) {
                if let (Ok(la), Ok(lo)) = (f[4].trim().parse::<f64>(), f[3].trim().parse::<f64>()) {
                    m.insert(code.to_string(), (la, lo));
                }
            }
        }
    }
    m
}

fn bgs_qd(lat: f64, lon: f64) -> Option<(f64, f64)> {
    let url = format!(
        "https://gifs-api.bgs.ac.uk/geomagnetic-coordinates/point?latitude={lat}&longitude={lon}&altitude=0&decimal_year={EPOCH_YEAR}"
    );
    let body = curl_get(&url)?;
    let key = "\"quasi-dipole\":{\"latitude\":";
    let rest = &body[body.find(key)? + key.len()..];
    let la: f64 = rest[..rest.find(',')?].trim().parse().ok()?;
    let lkey = "\"longitude\":";
    let lrest = &rest[rest.find(lkey)? + lkey.len()..];
    let lo: f64 = lrest[..lrest.find('}')?].trim().parse().ok()?;
    Some((la, lo))
}

fn resolve(lat: f64, lon: f64, smag: Option<(f64, f64)>) -> Option<(f64, f64, &'static str)> {
    if lat.abs() > 20.0 {
        if let Some((la, lo)) = cgm_of(lat, lon) {
            return Some((la, lo, "omniweb-cgm"));
        }
    }
    if let Some((la, lo)) = smag {
        return Some((la, lo, "supermag-aacgm"));
    }
    bgs_qd(lat, lon).map(|(la, lo)| (la, lo, "bgs-quasi-dipole"))
}

fn read_partition(tsv_path: &str) -> BTreeMap<String, String> {
    let text = fs::read_to_string(tsv_path).expect("the partition TSV reads");
    let mut out = BTreeMap::new();
    for (i, line) in text.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() >= 4 && !f[0].trim().is_empty() {
            out.insert(f[0].trim().to_string(), f[3].trim().to_string());
        }
    }
    out
}

fn family_channels(partition: &BTreeMap<String, String>, family: &str) -> Vec<String> {
    let mut out: Vec<String> = partition
        .iter()
        .filter(|(_, fam)| fam.as_str() == family)
        .map(|(code, _)| format!("{FAMILY_CHANNEL_FIELD}_{}", code.to_ascii_lowercase()))
        .collect();
    out.sort();
    out
}

fn emit_families(tsv_path: &str, dir: &str) {
    let partition = read_partition(tsv_path);
    fs::create_dir_all(dir).ok();
    let mut seen: BTreeMap<String, &'static str> = BTreeMap::new();
    let mut total = 0usize;
    for family in FAMILY_NAMES {
        let channels = family_channels(&partition, family);
        total += channels.len();
        for c in &channels {
            if let Some(other) = seen.insert(c.clone(), family) {
                eprintln!("coverage: channel {c} in {other} and {family} — not pairwise disjoint");
                std::process::exit(2);
            }
        }
        let path = format!("{dir}/gic-family-{family}.txt");
        fs::write(&path, format!("{}\n", channels.join("\n"))).expect("the channel list writes");
        println!("family {family}: {} channels -> {path}", channels.len());
    }
    let unassigned: Vec<&String> = partition
        .iter()
        .filter(|(_, fam)| !FAMILY_NAMES.contains(&fam.as_str()))
        .map(|(code, _)| code)
        .collect();
    println!(
        "coverage: union {total} = partition {} · distinct {} · unassigned {}",
        partition.len(),
        seen.len(),
        unassigned.len()
    );
    if total != partition.len() || seen.len() != partition.len() || !unassigned.is_empty() {
        eprintln!(
            "coverage: union {total} != partition {} — not complete",
            partition.len()
        );
        std::process::exit(2);
    }
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1).cloned())
}

fn fmt_deg(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.2}"),
        None => "pending".to_string(),
    }
}

fn fmt_int(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.0}"),
        None => "pending".to_string(),
    }
}

fn fmt_drift(primary: Option<f64>, smag: Option<(f64, f64)>, source: &str) -> String {
    match (primary, smag) {
        (Some(a), Some((s, _))) if source == "omniweb-cgm" => format!("{:.2}", (a - s.abs()).abs()),
        _ => "pending".to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let pause_ms = match arg_value(&args, "--pause-ms").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => DEFAULT_PAUSE_MS,
    };
    let supermag_path = arg_value(&args, "--supermag");

    if let Some(tsv) = arg_value(&args, "--from-tsv") {
        let dir = match arg_value(&args, "--emit-dir") {
            Some(v) => v,
            None => "state/river".to_string(),
        };
        emit_families(&tsv, &dir);
        return;
    }

    let src = format!("{}/phi/sources.φ", repo_root());
    let text = fs::read_to_string(&src).expect("phi/sources.φ reads");
    let stations = parse_stations(&text);
    println!(
        "cgm_lat_partition: {} GIN stations · epoch {EPOCH_YEAR} · height {HEIGHT_KM} km · model cgm",
        stations.len()
    );

    let supermag_text = match supermag_path {
        Some(p) => fs::read_to_string(p).ok(),
        None => curl_get(SUPERMAG_URL),
    };
    let smag: BTreeMap<String, (f64, f64)> = match supermag_text.as_deref() {
        Some(t) => supermag_mlat(t),
        None => BTreeMap::new(),
    };

    let mut rows = Vec::new();
    let mut fam_counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut missing = 0usize;
    let mut drift_sum = 0.0f64;
    let mut drift_n = 0usize;
    for st in stations.values() {
        let sm = smag.get(&st.code).copied();
        let resolved = resolve(st.lat, st.lon, sm);
        let (cgm_lat, cgm_lon, source, abs_cgm, family) = match resolved {
            Some((la, lo, src)) => {
                let a = la.abs();
                (
                    format!("{la:.2}"),
                    format!("{lo:.2}"),
                    src,
                    Some(a),
                    family_of(a),
                )
            }
            None => {
                missing += 1;
                (
                    "pending".to_string(),
                    "pending".to_string(),
                    "pending",
                    None,
                    "pending",
                )
            }
        };
        if source == "omniweb-cgm" {
            if let (Some(a), Some((s, _))) = (abs_cgm, sm) {
                drift_sum += (a - s.abs()).abs();
                drift_n += 1;
            }
        }
        *fam_counts.entry(family).or_insert(0) += 1;
        let drift = fmt_drift(abs_cgm, sm, source);
        rows.push(format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            st.code,
            cgm_lat,
            cgm_lon,
            family,
            fmt_deg(abs_cgm),
            fmt_int(st.elev),
            fmt_deg(sm.map(|(s, _)| s)),
            drift,
            source
        ));
        if pause_ms > 0 {
            sleep(Duration::from_millis(pause_ms));
        }
    }

    let header =
        "station\tcgm_lat\tcgm_lon\tfamily\tabs_cgm\telev_m\tsupermag_aacgmlat\tdrift_deg\tsource";
    let body = format!("{header}\n{}\n", rows.join("\n"));
    if let Some(dir) = std::path::Path::new(&out_path).parent() {
        if !dir.as_os_str().is_empty() {
            fs::create_dir_all(dir).ok();
        }
    }
    fs::write(&out_path, &body).expect("the partition TSV writes");
    println!("partition written: {out_path} ({} rows)", rows.len());

    let mut auroral = 0usize;
    let mut subau = 0usize;
    let mut midlat = 0usize;
    for (fam, n) in &fam_counts {
        match *fam {
            "auroral" => auroral = *n,
            "sub-auroral" => subau = *n,
            "mid-latitude" => midlat = *n,
            _ => {}
        }
    }
    println!(
        "families: auroral {auroral} · sub-auroral {subau} · mid-latitude {midlat} · pending {missing}"
    );
    if drift_n > 0 {
        println!(
            "drift (omniweb-cgm vs SuperMAG) over {drift_n}: mean {:.2}°",
            drift_sum / drift_n as f64
        );
    }
    let union = auroral + subau + midlat;
    if missing == 0 && union == stations.len() {
        println!(
            "coverage: disjoint by construction · union = {union} = {}",
            stations.len()
        );
    } else {
        println!(
            "coverage: measured {union} / {} · pending {missing} — the union omits the pending",
            stations.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partition(rows: &[(&str, &str)]) -> BTreeMap<String, String> {
        rows.iter()
            .map(|(c, f)| (c.to_string(), f.to_string()))
            .collect()
    }

    #[test]
    fn family_channels_are_pairwise_disjoint_and_cover_the_pool() {
        let p = partition(&[
            ("AAA", "auroral"),
            ("BBB", "auroral"),
            ("CCC", "sub-auroral"),
            ("DDD", "mid-latitude"),
        ]);
        let auroral = family_channels(&p, "auroral");
        let sub = family_channels(&p, "sub-auroral");
        let mid = family_channels(&p, "mid-latitude");
        assert_eq!(
            auroral,
            vec!["intermagnet_xyz_x_nt_aaa", "intermagnet_xyz_x_nt_bbb"]
        );
        assert_eq!(sub, vec!["intermagnet_xyz_x_nt_ccc"]);
        assert_eq!(mid, vec!["intermagnet_xyz_x_nt_ddd"]);
        let mut all: Vec<String> = auroral.into_iter().chain(sub).chain(mid).collect();
        all.sort();
        let union = all.len();
        all.dedup();
        assert_eq!(union, p.len());
        assert_eq!(all.len(), p.len());
    }

    #[test]
    fn a_pending_family_is_a_named_riss_not_a_zero() {
        let p = partition(&[("AAA", "auroral"), ("BBB", "pending")]);
        let unassigned = p
            .values()
            .filter(|fam| !FAMILY_NAMES.contains(&fam.as_str()))
            .count();
        assert_eq!(unassigned, 1);
    }
}
