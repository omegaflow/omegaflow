use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ASU_URL: &str = "https://vizier.cds.unistra.fr/viz-bin/asu-tsv";
const SOURCE: &str = "VII/289/dr16q";
const DEFAULT_INPUT: &str = "/tmp/opencode/gaia/missing88.csv";

#[derive(Clone, Debug)]
struct Anchor {
    iers: String,
    ra_deg: f64,
    dec_deg: f64,
}

#[derive(Clone, Debug)]
struct Candidate {
    recno: String,
    sdss: String,
    ra_deg: f64,
    dec_deg: f64,
    z: String,
    zqn: String,
    sep_arcsec: f64,
}

#[derive(Debug)]
struct Match {
    anchor: Anchor,
    n_cand: usize,
    best: Option<Candidate>,
    footprint_n: Option<usize>,
    footprint_nearest: Option<f64>,
}

fn parse_tsv_line(line: &str) -> Vec<String> {
    line.split('\t').map(|f| f.trim().to_string()).collect()
}

fn parse_anchor(line: &str, lineno: usize) -> Result<Anchor, String> {
    let f: Vec<String> = line.split(',').map(|v| v.trim().to_string()).collect();
    if f.len() < 3 {
        return Err(format!("line {lineno}: expected iers,ra_deg,dec_deg"));
    }
    let ra_deg: f64 = f[1]
        .parse()
        .map_err(|e| format!("line {lineno}: ra_deg {:?}: {e}", f[1]))?;
    let dec_deg: f64 = f[2]
        .parse()
        .map_err(|e| format!("line {lineno}: dec_deg {:?}: {e}", f[2]))?;
    if !ra_deg.is_finite() || !dec_deg.is_finite() {
        return Err(format!("line {lineno}: non-finite coordinate"));
    }
    if !(-90.0..=90.0).contains(&dec_deg) {
        return Err(format!("line {lineno}: declination out of range"));
    }
    Ok(Anchor {
        iers: f[0].clone(),
        ra_deg,
        dec_deg,
    })
}

fn load_anchors(path: &Path) -> Result<Vec<Anchor>, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut out = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }
        if i == 0 && line.starts_with("iers") {
            continue;
        }
        out.push(parse_anchor(line, i + 1)?);
    }
    Ok(out)
}

fn separation_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let d2r = std::f64::consts::PI / 180.0;
    let (r1, d1, r2, d2) = (ra1 * d2r, dec1 * d2r, ra2 * d2r, dec2 * d2r);
    let sd = ((d2 - d1) / 2.0).sin();
    let sa = ((r2 - r1) / 2.0).sin();
    let a = sa * sa * d1.cos() * d2.cos() + sd * sd;
    let c = a.sqrt().min(1.0);
    2.0 * c.asin() / d2r * 3600.0
}

fn asu_url(anchor: &Anchor, radius_arcsec: f64) -> String {
    let dec_token = if anchor.dec_deg < 0.0 {
        format!("{:.9}", anchor.dec_deg)
    } else {
        format!("%2B{:.9}", anchor.dec_deg)
    };
    format!(
        "{ASU_URL}?-source={SOURCE}&-c={:.9}{dec_token}&-c.rs={:.3}&-out=recno,SDSS,RAJ2000,DEJ2000,z,zQN",
        anchor.ra_deg, radius_arcsec
    )
}

fn raw_name(index: usize, anchor: &Anchor, suffix: &str) -> String {
    let safe: String = anchor
        .iers
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '+' || c == '.' { c } else { '_' })
        .collect();
    format!("{:03}_{safe}{suffix}.tsv", index + 1)
}

fn fetch_url(url: &str) -> Result<String, String> {
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-m")
        .arg("90")
        .arg(url)
        .output()
        .map_err(|e| format!("curl: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "curl status {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    String::from_utf8(out.stdout).map_err(|e| format!("response is not utf-8: {e}"))
}

fn read_response(
    index: usize,
    anchor: &Anchor,
    url: &str,
    raw_dir: Option<&Path>,
    suffix: &str,
) -> Result<String, String> {
    if let Some(dir) = raw_dir {
        let path = dir.join(raw_name(index, anchor, suffix));
        if path.is_file() {
            return fs::read_to_string(&path)
                .map_err(|e| format!("read {}: {e}", path.display()));
        }
        let text = fetch_url(url)?;
        fs::write(&path, text.as_bytes())
            .map_err(|e| format!("write {}: {e}", path.display()))?;
        return Ok(text);
    }
    fetch_url(url)
}

fn data_lines(text: &str) -> Vec<(&str, usize)> {
    let mut out = Vec::new();
    let mut seen_header = false;
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim_end_matches('\r');
        let trimmed = line.trim_start();
        if line.trim().is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with("recno") {
            seen_header = true;
            continue;
        }
        if !seen_header {
            continue;
        }
        if trimmed.starts_with('-') {
            continue;
        }
        if trimmed.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            out.push((line, i));
        }
    }
    out
}

fn parse_candidates(text: &str, anchor: &Anchor) -> Result<Vec<Candidate>, String> {
    let mut out = Vec::new();
    for (line, lineno) in data_lines(text) {
        let f = parse_tsv_line(line);
        if f.len() < 6 {
            return Err(format!("candidate row {lineno}: expected 6 columns, got {}", f.len()));
        }
        let ra_deg: f64 = f[2]
            .parse()
            .map_err(|e| format!("candidate row {lineno}: RAJ2000 {:?}: {e}", f[2]))?;
        let dec_deg: f64 = f[3]
            .parse()
            .map_err(|e| format!("candidate row {lineno}: DEJ2000 {:?}: {e}", f[3]))?;
        out.push(Candidate {
            recno: f[0].clone(),
            sdss: f[1].clone(),
            ra_deg,
            dec_deg,
            z: f[4].clone(),
            zqn: f[5].clone(),
            sep_arcsec: separation_arcsec(anchor.ra_deg, anchor.dec_deg, ra_deg, dec_deg),
        });
    }
    Ok(out)
}

fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    if n % 2 == 1 {
        Some(sorted[n / 2])
    } else {
        Some((sorted[n / 2 - 1] + sorted[n / 2]) / 2.0)
    }
}

fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum::<f64>() / values.len() as f64)
}

fn sample_std(values: &[f64]) -> Option<f64> {
    if values.len() < 2 {
        return None;
    }
    let m = mean(values)?;
    let ss: f64 = values.iter().map(|v| (v - m) * (v - m)).sum();
    Some((ss / (values.len() - 1) as f64).sqrt())
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let input = PathBuf::from(match arg_value(&args, "--input") {
        Some(value) => value,
        None => DEFAULT_INPUT.to_string(),
    });
    let radius_arcsec: f64 = match arg_value(&args, "--radius-arcsec") {
        Some(v) => v.parse().map_err(|e| format!("--radius-arcsec: {e}"))?,
        None => 2.0,
    };
    if !(radius_arcsec.is_finite() && radius_arcsec > 0.0) {
        return Err("--radius-arcsec must be positive and finite".to_string());
    }
    let out_path = arg_value(&args, "--out").map(PathBuf::from);
    let raw_dir = arg_value(&args, "--raw-dir").map(PathBuf::from);
    if let Some(dir) = raw_dir.as_deref() {
        fs::create_dir_all(dir)
            .map_err(|e| format!("create {}: {e}", dir.display()))?;
    }
    let footprint_arcmin: Option<f64> = match arg_value(&args, "--footprint-arcmin") {
        Some(v) => Some(v.parse().map_err(|e| format!("--footprint-arcmin: {e}"))?),
        None => None,
    };
    if let Some(v) = footprint_arcmin {
        if !(v.is_finite() && v > 0.0) {
            return Err("--footprint-arcmin must be positive and finite".to_string());
        }
    }

    let anchors = load_anchors(&input)?;
    if anchors.is_empty() {
        return Err(format!("no anchors in {}", input.display()));
    }

    println!("sdss_88_probe");
    println!("source: {ASU_URL}");
    println!("table: {SOURCE} (VizieR VII/289, SDSS DR16Q, Lyke+ 2020)");
    println!("input: {}", input.display());
    println!("anchors: {}", anchors.len());
    println!("radius_arcsec: {radius_arcsec:.3}");
    match footprint_arcmin {
        Some(v) => println!("footprint_arcmin: {v:.3}"),
        None => println!("footprint_arcmin: n/a"),
    }

    let mut matches: Vec<Match> = Vec::new();
    for (i, anchor) in anchors.iter().enumerate() {
        let url = asu_url(anchor, radius_arcsec);
        let text = read_response(i, anchor, &url, raw_dir.as_deref(), "")?;
        let cands = parse_candidates(&text, anchor)?;
        let best = cands
            .iter()
            .min_by(|a, b| a.sep_arcsec.partial_cmp(&b.sep_arcsec).unwrap_or(std::cmp::Ordering::Equal))
            .cloned();
        let (footprint_n, footprint_nearest) = match footprint_arcmin {
            Some(arcmin) => {
                let furl = asu_url(anchor, arcmin * 60.0);
                let ft = read_response(i, anchor, &furl, raw_dir.as_deref(), ".footprint")?;
                let fcands = parse_candidates(&ft, anchor)?;
                (
                    Some(fcands.len()),
                    fcands.iter().map(|c| c.sep_arcsec).reduce(f64::min),
                )
            }
            None => (None, None),
        };
        println!(
            "{:>3} {:<12} {:>11.6} {:>11.6} cand={:<2} fp={:<5} {}",
            i + 1,
            anchor.iers,
            anchor.ra_deg,
            anchor.dec_deg,
            cands.len(),
            match footprint_n {
                Some(v) => v.to_string(),
                None => "n/a".to_string(),
            },
            match &best {
                Some(c) => format!("sep={:.4}\" sdss={} z={}", c.sep_arcsec, c.sdss, c.z),
                None => match footprint_nearest {
                    Some(v) => format!("no_sdss nearest={v:.1}\""),
                    None => "no_sdss".to_string(),
                },
            }
        );
        matches.push(Match {
            anchor: anchor.clone(),
            n_cand: cands.len(),
            best,
            footprint_n,
            footprint_nearest,
        });
    }

    let seps: Vec<f64> = matches
        .iter()
        .filter_map(|m| m.best.as_ref().map(|c| c.sep_arcsec))
        .collect();
    let unmatched: Vec<&Match> = matches.iter().filter(|m| m.best.is_none()).collect();
    let n = matches.len();
    let n_match = seps.len();

    println!();
    println!("summary");
    println!("matched: {n_match}/{n}");
    println!("unmatched: {}", unmatched.len());
    match median(&seps) {
        Some(v) => println!("median_sep_arcsec: {v:.4}"),
        None => println!("median_sep_arcsec: n/a"),
    }
    match mean(&seps) {
        Some(v) => println!("mean_sep_arcsec: {v:.4}"),
        None => println!("mean_sep_arcsec: n/a"),
    }
    match sample_std(&seps) {
        Some(v) => println!("std_sep_arcsec: {v:.4}"),
        None => println!("std_sep_arcsec: n/a"),
    }
    match seps.iter().copied().reduce(f64::min) {
        Some(v) => println!("min_sep_arcsec: {v:.4}"),
        None => println!("min_sep_arcsec: n/a"),
    }
    match seps.iter().copied().reduce(f64::max) {
        Some(v) => println!("max_sep_arcsec: {v:.4}"),
        None => println!("max_sep_arcsec: n/a"),
    }
    println!("unmatched_iers: {}", unmatched.iter().map(|m| m.anchor.iers.as_str()).collect::<Vec<_>>().join(" "));

    if let Some(arcmin) = footprint_arcmin {
        let covered: Vec<&Match> = matches
            .iter()
            .filter(|m| m.footprint_n.map(|v| v > 0).unwrap_or(false))
            .collect();
        let covered_match = covered.iter().filter(|m| m.best.is_some()).count();
        println!("footprint_arcmin_used: {arcmin:.3}");
        println!("footprint_covered: {}/{}", covered.len(), n);
        println!("footprint_uncovered: {}", n - covered.len());
        println!("matched_in_footprint: {covered_match}/{}", covered.len());
        let mut nearest_list = Vec::new();
        for m in &matches {
            if m.best.is_none() {
                if let Some(v) = m.footprint_nearest {
                    nearest_list.push(format!("{}:{:.1}", m.anchor.iers, v));
                }
            }
        }
        println!("nearest_unmatched_arcsec: {}", nearest_list.join(" "));
    }

    if let Some(path) = out_path {
        let mut text = String::from(
            "iers,ra_deg,dec_deg,n_cand,footprint_n,footprint_nearest_arcsec,recno,sdss,ra_sdss,dec_sdss,sep_arcsec,z,zQN\n",
        );
        for m in &matches {
            let footprint = match m.footprint_n {
                Some(v) => v.to_string(),
                None => String::new(),
            };
            let nearest = match m.footprint_nearest {
                Some(v) => format!("{v:.3}"),
                None => String::new(),
            };
            match &m.best {
                Some(c) => text.push_str(&format!(
                    "{},{:.9},{:.9},{},{},{},{},{},{:.6},{:.6},{:.4},{},{}\n",
                    m.anchor.iers,
                    m.anchor.ra_deg,
                    m.anchor.dec_deg,
                    m.n_cand,
                    footprint,
                    nearest,
                    c.recno,
                    c.sdss,
                    c.ra_deg,
                    c.dec_deg,
                    c.sep_arcsec,
                    c.z,
                    c.zqn
                )),
                None => text.push_str(&format!(
                    "{},{:.9},{:.9},{},{},{},,,,,,,\n",
                    m.anchor.iers,
                    m.anchor.ra_deg,
                    m.anchor.dec_deg,
                    m.n_cand,
                    footprint,
                    nearest
                )),
            }
        }
        fs::write(&path, text.as_bytes())
            .map_err(|e| format!("write {}: {e}", path.display()))?;
        println!("out: {}", path.display());
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("sdss_88_probe: {e}");
        std::process::exit(1);
    }
}
