use omegaflow::cdn::upload_asset;
use omegaflow::fits::{FitsHeader, FitsTable};
use omegaflow::json::{jnum, jpath_val, jstr, parse_json, JsonVal};
use std::io::Write;
use std::process::Command;

fn mast_token() -> Option<String> {
    if let Ok(t) = std::env::var("MAST_TOKEN") {
        if !t.is_empty() {
            return Some(t);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == "MAST_TOKEN" && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn curl_json(url: &str, data: &[(&str, String)], token: &str) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-L")
        .arg("--max-time")
        .arg("300")
        .arg("-G");
    for (k, v) in data {
        cmd.arg("--data-urlencode").arg(format!("{}={}", k, v));
    }
    if !token.is_empty() {
        cmd.arg("-H")
            .arg(format!("Authorization: Bearer {}", token));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "curl {}: {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn curl_bytes(url: &str, data: &[(&str, String)], token: &str, dest: &str) -> bool {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-L")
        .arg("--max-time")
        .arg("600")
        .arg("-G")
        .arg("-o")
        .arg(dest);
    for (k, v) in data {
        cmd.arg("--data-urlencode").arg(format!("{}={}", k, v));
    }
    if !token.is_empty() {
        cmd.arg("-H")
            .arg(format!("Authorization: Bearer {}", token));
    }
    cmd.arg(url);
    match cmd.output() {
        Ok(o) if o.status.success() => true,
        Ok(o) => {
            eprintln!(
                "curl {}: {}",
                url,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("curl {}: {}", url, e);
            false
        }
    }
}

struct Target {
    tic_id: String,
    ra_deg: f64,
    dec_deg: f64,
    plx_mas: f64,
}

struct CurveStar {
    ra_deg: f64,
    dec_deg: f64,
    plx_mas: f64,
    samples: Vec<(f64, f32)>,
}

fn tap_targets(token: &str, limit: usize) -> Vec<Target> {
    let adql = "SELECT pl_name,hostname,tic_id,ra,dec,sy_dist FROM pscomppars WHERE pl_tranmid IS NOT NULL AND tic_id IS NOT NULL";
    let body = match curl_json(
        "https://exoplanetarchive.ipac.caltech.edu/TAP/sync",
        &[
            ("REQUEST", "doQuery".to_string()),
            ("LANG", "ADQL".to_string()),
            ("FORMAT", "json".to_string()),
            ("QUERY", adql.to_string()),
        ],
        token,
    ) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let Some(root) = parse_json(&body) else {
        eprintln!("tap targets: json absent");
        return Vec::new();
    };
    let mut seen = std::collections::HashSet::new();
    let mut targets = Vec::new();
    if let Some(JsonVal::Arr(rows)) = jpath_val(&root, "data") {
        for row in rows {
            if targets.len() >= limit {
                break;
            }
            let Some(tic) = jstr(row, "tic_id") else {
                continue;
            };
            let Some(ra) = jnum(row, "ra") else {
                continue;
            };
            let Some(dec) = jnum(row, "dec") else {
                continue;
            };
            if !seen.insert(tic.clone()) {
                continue;
            }
            let plx = match jnum(row, "sy_dist") {
                Some(d) if d > 0.0 => 1000.0 / d,
                _ => 0.0,
            };
            targets.push(Target {
                tic_id: tic,
                ra_deg: ra,
                dec_deg: dec,
                plx_mas: plx,
            });
        }
    }
    targets
}

fn mast_lc_obs_ids(target: &Target, token: &str) -> Vec<String> {
    let request = format!(
        r#"{{"service":"Mast.Caom.Cone","params":{{"ra":{},"dec":{},"radius":0.002,"obs_collection":"TESS","dataproduct_type":"timeseries","provenance_name":"SPOC"}},"format":"json","pagesize":500,"removenullcolumns":true}}"#,
        target.ra_deg, target.dec_deg
    );
    let body = match curl_json(
        "https://mast.stsci.edu/api/v0/invoke",
        &[("request", request)],
        token,
    ) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let Some(root) = parse_json(&body) else {
        return Vec::new();
    };
    let mut ids = Vec::new();
    if let Some(JsonVal::Arr(rows)) = jpath_val(&root, "data") {
        for row in rows {
            let Some(obs_id) = jstr(row, "obs_id") else {
                continue;
            };
            if obs_id.ends_with("-0120-s") {
                ids.push(obs_id);
            }
        }
    }
    ids
}

fn fits_points_from(bytes: &[u8]) -> Vec<(f64, f32)> {
    let mut off = 0usize;
    if let Some((_, data_end)) = FitsHeader::parse(bytes, 0) {
        off = data_end;
    }
    let mut points = Vec::new();
    while off + 80 <= bytes.len() {
        let Some((t, next)) = FitsTable::parse(bytes, off) else {
            break;
        };
        let time = match t.column("TIME") {
            Some(c) => c,
            None => {
                off = next;
                continue;
            }
        };
        let flux_col = t.column("PDCSAP_FLUX").or_else(|| t.column("SAP_FLUX"));
        let quality = t.column("QUALITY");
        let Some(flux_c) = flux_col else {
            off = next;
            continue;
        };
        for row in 0..t.n_rows {
            let btjd = match t.cell_f64(bytes, row, time) {
                Some(v) if v.is_finite() => v,
                _ => continue,
            };
            if let Some(q) = quality {
                match t.cell_i64(bytes, row, q) {
                    Some(v) if v != 0 => continue,
                    _ => {}
                }
            }
            let f = match t.cell_f64(bytes, row, flux_c) {
                Some(v) if v.is_finite() && v >= 0.0 => v as f32,
                _ => continue,
            };
            points.push(((btjd + 5455.0) * 86400.0, f));
        }
        off = next;
    }
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    points
}

fn fits_points(path: &str) -> Vec<(f64, f32)> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return Vec::new();
        }
    };
    fits_points_from(&bytes)
}

fn probe_fits(path: &str) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return;
        }
    };
    let (h, _) = match FitsHeader::parse(&bytes, 0) {
        Some(v) => v,
        None => {
            eprintln!("primary header: absent");
            return;
        }
    };
    eprintln!(
        "primary: SIMPLE={:?} NAXIS={:?}",
        h.value("SIMPLE"),
        h.value("NAXIS")
    );
    let mut off = 2880usize;
    if let Some((_, data_end)) = FitsHeader::parse(&bytes, 0) {
        off = data_end;
    }
    let mut ext = 0;
    while off + 80 <= bytes.len() {
        let Some((t, next)) = FitsTable::parse(&bytes, off) else {
            break;
        };
        ext += 1;
        let names: Vec<String> = t.columns.iter().map(|c| c.name.clone()).collect();
        let pts = fits_points_from(&bytes);
        eprintln!("ext {}: rows {} cols {:?}", ext, t.n_rows, names);
        eprintln!("points: {}", pts.len());
        off = next;
        if ext >= 2 {
            break;
        }
    }
}

fn write_asset(curves: &[CurveStar], out: &str) -> bool {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"TSS1");
    buf.extend_from_slice(&(curves.len() as u32).to_le_bytes());
    for c in curves {
        buf.extend_from_slice(&c.ra_deg.to_le_bytes());
        buf.extend_from_slice(&c.dec_deg.to_le_bytes());
        buf.extend_from_slice(&c.plx_mas.to_le_bytes());
        buf.extend_from_slice(&(c.samples.len() as u32).to_le_bytes());
        for (t, f) in &c.samples {
            buf.extend_from_slice(&t.to_le_bytes());
            buf.extend_from_slice(&f.to_le_bytes());
        }
    }
    match std::fs::write(out, &buf) {
        Ok(()) => true,
        Err(e) => {
            eprintln!("write {}: {}", out, e);
            false
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut limit = usize::MAX;
    let mut probe: Option<String> = None;
    let mut tic_filter: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--limit" => {
                limit = args
                    .get(i + 1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(usize::MAX);
                i += 1;
            }
            "--tic" => {
                tic_filter = args.get(i + 1).cloned();
                i += 1;
            }
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {
                eprintln!(
                    "usage: tess_compiler --out <tess_lightcurves.bin> [--ci-mode] [--limit N] [--tic <tic_id,...>] [--probe <lc.fits>]"
                );
                return;
            }
        }
        i += 1;
    }
    if let Some(p) = probe {
        probe_fits(&p);
        return;
    }
    let Some(out_path) = out else {
        eprintln!("--out absent");
        return;
    };
    let token = mast_token().unwrap_or_default();
    if token.is_empty() {
        eprintln!("MAST_TOKEN absent (.secrets.local or env)");
        return;
    }
    let tic_set: Option<std::collections::HashSet<String>> =
        tic_filter.map(|s| s.split(',').map(|t| t.trim().to_string()).collect());
    let targets = tap_targets(&token, limit);
    eprintln!("tess targets: {}", targets.len());
    let mut curves = Vec::new();
    for (n, target) in targets.iter().enumerate() {
        if let Some(set) = &tic_set {
            if !set.contains(&target.tic_id) {
                continue;
            }
        }
        let ids = mast_lc_obs_ids(target, &token);
        if ids.is_empty() {
            eprintln!(
                "\r\x1b[K[{}] {} {}: no SPOC 2-min sectors",
                n, target.tic_id, target.ra_deg
            );
            continue;
        }
        let mut samples = Vec::new();
        for obs_id in &ids {
            let tmp = format!("/tmp/opencode/tess_lc_{}.fits", obs_id);
            if !curl_bytes(
                "https://mast.stsci.edu/api/v0/Download/file",
                &[("uri", format!("mast:TESS/product/{}_lc.fits", obs_id))],
                &token,
                &tmp,
            ) {
                continue;
            }
            samples.extend(fits_points(&tmp));
            let _ = std::fs::remove_file(&tmp);
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
        if samples.is_empty() {
            continue;
        }
        samples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        eprintln!(
            "\r\x1b[K[{}] {} {}: {} sektoren, {} samples",
            n,
            target.tic_id,
            target.ra_deg,
            ids.len(),
            samples.len()
        );
        let _ = std::io::stdout().flush();
        curves.push(CurveStar {
            ra_deg: target.ra_deg,
            dec_deg: target.dec_deg,
            plx_mas: target.plx_mas,
            samples,
        });
    }
    eprintln!("\ntess stars with curves: {}", curves.len());
    if write_asset(&curves, &out_path) && ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
