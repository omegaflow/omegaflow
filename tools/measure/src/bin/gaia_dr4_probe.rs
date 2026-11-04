use omegaflow::archivar::{JsonVal, jnum, votable_to_json};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

const TAP_SYNC: &str = "https://gea.esac.esa.int/tap-server/tap/sync";
const USER_AGENT: &str = "omegaflow-gaia-dr4-probe/1.0";
const ANCHOR_CHUNK: usize = 100;
const DEFAULT_RUN_PREFIX: &str = "state/gaia_tap/dr4_epoch";
const DEFAULT_ADQL_PREFIX: &str = "state/gaia_tap/adql_probe";

fn dr4_adql(ids: &str) -> String {
    format!(
        "SELECT source_id, transit_id, ra0, dec0, obs_time_tcb, scan_pos_angle, centroid_pos_al, parallax_factor_al, used_by_agis_al FROM gaiadr4.epoch_astrometry WHERE source_id IN ({ids})"
    )
}

fn snippet(text: &str, max: usize) -> String {
    let mut out: String = text.chars().take(max).collect();
    if text.chars().count() > max {
        out.push_str("...");
    }
    out
}

fn votable_error(text: &str) -> Option<String> {
    let key = "QUERY_STATUS=\"ERROR\"";
    let at = text.find(key)?;
    let tail = &text[at + key.len()..];
    match tail.find('>') {
        Some(gt) => {
            let after = &tail[gt + 1..];
            match after.find("</INFO>") {
                Some(end) => {
                    let inner = after[..end].trim();
                    if inner.is_empty() {
                        Some("QUERY_STATUS=ERROR without message".to_string())
                    } else {
                        Some(inner.to_string())
                    }
                }
                None => Some("QUERY_STATUS=ERROR without message".to_string()),
            }
        }
        None => Some("QUERY_STATUS=ERROR without message".to_string()),
    }
}

fn tap_post(adql: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("300")
        .arg("-A")
        .arg(USER_AGENT)
        .arg("-X")
        .arg("POST")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=votable")
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(TAP_SYNC)
        .output()
        .map_err(|e| format!("curl: {e}"))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("curl: {}", stderr.trim()));
    }
    let stdout = out.stdout;
    let idx = stdout
        .iter()
        .rposition(|&b| b == b'\n')
        .ok_or_else(|| "TAP response carries no HTTP status".to_string())?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    let body = &stdout[..idx];
    if body.iter().all(|b| b.is_ascii_whitespace()) {
        return Err(format!("empty response at HTTP {code}"));
    }
    if !code.starts_with('2') {
        let text = String::from_utf8_lossy(body);
        return Err(format!("HTTP {code}: {}", snippet(text.trim(), 240)));
    }
    let text = String::from_utf8_lossy(body);
    if let Some(msg) = votable_error(&text) {
        return Err(format!("TAP QUERY_STATUS=ERROR: {msg}"));
    }
    if !text.contains("<VOTABLE") {
        return Err(format!("response is not a VOTable ({} bytes)", body.len()));
    }
    Ok(body.to_vec())
}

fn votable_rows(body: &[u8]) -> Option<usize> {
    let text = std::str::from_utf8(body).ok()?;
    match votable_to_json(text) {
        Some(JsonVal::Arr(rows)) => Some(rows.len()),
        _ => None,
    }
}

fn write_vot(path: &Path, body: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
    }
    std::fs::write(path, body).map_err(|e| format!("{}: {e}", path.display()))
}

fn tap_run(csv: &str, prefix: &str) -> Result<(), String> {
    let anchors = read_anchors(csv)?;
    let chunks: Vec<&[Anchor]> = anchors.chunks(ANCHOR_CHUNK).collect();
    println!("anchors {} chunks {}", anchors.len(), chunks.len());
    let mut written = 0usize;
    let mut not_written: Vec<usize> = Vec::new();
    for (i, chunk) in chunks.iter().enumerate() {
        let number = i + 1;
        let ids = chunk
            .iter()
            .map(|a| a.id.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let path = Path::new(&format!("{prefix}_{number:02}.vot")).to_path_buf();
        let result = tap_post(&dr4_adql(&ids)).and_then(|body| {
            write_vot(&path, &body)?;
            Ok(body)
        });
        match result {
            Ok(body) => {
                written += 1;
                match votable_rows(&body) {
                    Some(n) => println!(
                        "chunk {number}/{} rows {n} -> {}",
                        chunks.len(),
                        path.display()
                    ),
                    None => println!(
                        "chunk {number}/{} rows unparsed -> {}",
                        chunks.len(),
                        path.display()
                    ),
                }
            }
            Err(e) => {
                eprintln!("chunk {number}/{}: {e}", chunks.len());
                not_written.push(number);
            }
        }
    }
    println!("written {written}/{}", chunks.len());
    if not_written.is_empty() {
        Ok(())
    } else {
        Err(format!("chunks not written: {not_written:?}"))
    }
}

fn tap_adql(adql: &str, prefix: &str) -> Result<(), String> {
    let path = Path::new(&format!("{prefix}.vot")).to_path_buf();
    let body = tap_post(adql)?;
    write_vot(&path, &body)?;
    match votable_rows(&body) {
        Some(n) => println!("rows {n} -> {}", path.display()),
        None => println!("rows unparsed -> {}", path.display()),
    }
    Ok(())
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn field<'a>(row: &'a JsonVal, name: &str) -> Option<&'a JsonVal> {
    match row {
        JsonVal::Obj(map) => map.get(name),
        _ => None,
    }
}

fn nums(value: Option<&JsonVal>) -> Vec<f64> {
    match value {
        Some(JsonVal::Arr(items)) => items
            .iter()
            .filter_map(|v| match v {
                JsonVal::Num(n) if n.is_finite() => Some(*n),
                _ => None,
            })
            .collect(),
        Some(JsonVal::Num(n)) if n.is_finite() => vec![*n],
        Some(JsonVal::Str(s)) => s
            .split_whitespace()
            .filter_map(|x| x.parse::<f64>().ok())
            .filter(|x| x.is_finite())
            .collect(),
        _ => Vec::new(),
    }
}

fn bools(value: Option<&JsonVal>) -> Vec<bool> {
    match value {
        Some(JsonVal::Arr(items)) => items
            .iter()
            .filter_map(|v| match v {
                JsonVal::Bool(b) => Some(*b),
                JsonVal::Num(n) => Some(*n != 0.0),
                _ => None,
            })
            .collect(),
        Some(JsonVal::Bool(b)) => vec![*b],
        _ => Vec::new(),
    }
}

fn solve3(mut a: [[f64; 3]; 3], mut b: [f64; 3]) -> Option<[f64; 3]> {
    for col in 0..3 {
        let mut pivot = col;
        for row in col + 1..3 {
            if a[row][col].abs() > a[pivot][col].abs() {
                pivot = row;
            }
        }
        if a[pivot][col].abs() < 1e-18 {
            return None;
        }
        a.swap(col, pivot);
        b.swap(col, pivot);
        let d = a[col][col];
        for k in col..3 {
            a[col][k] /= d;
        }
        b[col] /= d;
        for row in 0..3 {
            if row == col {
                continue;
            }
            let f = a[row][col];
            for k in col..3 {
                a[row][k] -= f * a[col][k];
            }
            b[row] -= f * b[col];
        }
    }
    Some(b)
}

fn fit_pm(ts: &[f64], al: &[f64], pa: &[f64]) -> Option<(f64, f64, f64)> {
    let n = ts.len().min(al.len()).min(pa.len());
    if n < 3 {
        return None;
    }
    let mt = ts[..n].iter().sum::<f64>() / n as f64;
    let mut ata = [[0.0f64; 3]; 3];
    let mut atb = [0.0f64; 3];
    for i in 0..n {
        let dt = ts[i] - mt;
        let p = pa[i].to_radians();
        let row = [dt * p.sin(), dt * p.cos(), 1.0];
        for r in 0..3 {
            atb[r] += row[r] * al[i];
            for c in 0..3 {
                ata[r][c] += row[r] * row[c];
            }
        }
    }
    let [pm_ra, pm_dec, c] = solve3(ata, atb)?;
    let rms = (0..n)
        .map(|i| {
            let dt = ts[i] - mt;
            let p = pa[i].to_radians();
            let model = pm_ra * dt * p.sin() + pm_dec * dt * p.cos() + c;
            let r = al[i] - model;
            r * r
        })
        .sum::<f64>()
        / n as f64;
    Some((pm_ra, pm_dec, rms.sqrt()))
}

fn solve4(mut a: [[f64; 4]; 4], mut b: [f64; 4]) -> Option<[f64; 4]> {
    for col in 0..4 {
        let mut pivot = col;
        for row in col + 1..4 {
            if a[row][col].abs() > a[pivot][col].abs() {
                pivot = row;
            }
        }
        if a[pivot][col].abs() < 1e-18 {
            return None;
        }
        a.swap(col, pivot);
        b.swap(col, pivot);
        let d = a[col][col];
        for k in col..4 {
            a[col][k] /= d;
        }
        b[col] /= d;
        for row in 0..4 {
            if row == col {
                continue;
            }
            let f = a[row][col];
            for k in col..4 {
                a[row][k] -= f * a[col][k];
            }
            b[row] -= f * b[col];
        }
    }
    Some(b)
}

fn fit_pm4(ts: &[f64], al: &[f64], pa: &[f64], pf: &[f64]) -> Option<(f64, f64, f64, f64)> {
    let n = ts.len().min(al.len()).min(pa.len()).min(pf.len());
    if n < 4 {
        return None;
    }
    let mt = ts[..n].iter().sum::<f64>() / n as f64;
    let mut ata = [[0.0f64; 4]; 4];
    let mut atb = [0.0f64; 4];
    for i in 0..n {
        let dt = ts[i] - mt;
        let p = pa[i].to_radians();
        let row = [dt * p.sin(), dt * p.cos(), pf[i], 1.0];
        for r in 0..4 {
            atb[r] += row[r] * al[i];
            for c in 0..4 {
                ata[r][c] += row[r] * row[c];
            }
        }
    }
    let [pm_ra, pm_dec, plx, c] = solve4(ata, atb)?;
    let rms = (0..n)
        .map(|i| {
            let dt = ts[i] - mt;
            let p = pa[i].to_radians();
            let model = pm_ra * dt * p.sin() + pm_dec * dt * p.cos() + plx * pf[i] + c;
            let r = al[i] - model;
            r * r
        })
        .sum::<f64>()
        / n as f64;
    Some((pm_ra, pm_dec, plx, rms.sqrt()))
}

struct Anchor {
    id: String,
    ra: f64,
    dec: f64,
}

fn read_anchors(path: &str) -> Result<Vec<Anchor>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 3 {
            continue;
        }
        let id = cols[0].trim().trim_matches('"').to_string();
        let (Ok(ra), Ok(dec)) = (cols[1].trim().parse::<f64>(), cols[2].trim().parse::<f64>())
        else {
            continue;
        };
        if id.is_empty() {
            continue;
        }
        out.push(Anchor { id, ra, dec });
    }
    if out.is_empty() {
        return Err(format!("{path}: no anchors parsed"));
    }
    Ok(out)
}

fn sep_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let (ra1, dec1, ra2, dec2) = (
        ra1.to_radians(),
        dec1.to_radians(),
        ra2.to_radians(),
        dec2.to_radians(),
    );
    let a = ((dec1 - dec2) / 2.0).sin().powi(2)
        + dec1.cos() * dec2.cos() * ((ra1 - ra2) / 2.0).sin().powi(2);
    2.0 * a.sqrt().asin().to_degrees() * 3600.0
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let anchors = match arg_value(&args, "--anchors") {
        Some(path) => match read_anchors(&path) {
            Ok(anchors) => anchors,
            Err(e) => {
                eprintln!("{e}");
                Vec::new()
            }
        },
        None => Vec::new(),
    };
    let summary_out = arg_value(&args, "--summary-out");
    if let Some(path) = arg_value(&args, "--tap-query") {
        let query_anchors = match read_anchors(&path) {
            Ok(anchors) => anchors,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        };
        println!("-- anchors {}", query_anchors.len());
        for chunk in query_anchors.chunks(ANCHOR_CHUNK) {
            let ids = chunk
                .iter()
                .map(|a| a.id.as_str())
                .collect::<Vec<_>>()
                .join(",");
            println!("{}", dr4_adql(&ids));
        }
        return;
    }
    if let Some(csv) = arg_value(&args, "--tap-run") {
        let prefix = match arg_value(&args, "--out-prefix") {
            Some(prefix) => prefix,
            None => DEFAULT_RUN_PREFIX.to_string(),
        };
        match tap_run(&csv, &prefix) {
            Ok(()) => return,
            Err(e) => {
                eprintln!("tap-run: {e}");
                std::process::exit(1);
            }
        }
    }
    if let Some(adql) = arg_value(&args, "--tap-adql") {
        let prefix = match arg_value(&args, "--out-prefix") {
            Some(prefix) => prefix,
            None => DEFAULT_ADQL_PREFIX.to_string(),
        };
        match tap_adql(&adql, &prefix) {
            Ok(()) => return,
            Err(e) => {
                eprintln!("tap-adql: {e}");
                std::process::exit(1);
            }
        }
    }
    let Some(path) = arg_value(&args, "--xml") else {
        eprintln!(
            "usage: gaia_dr4_probe --xml <GAIA_DR4_PRERELEASE_EPOCH_ASTROMETRY_RAW.xml> [--anchors <csv>]"
        );
        eprintln!("       gaia_dr4_probe --tap-query <anchors.csv>");
        eprintln!("       gaia_dr4_probe --tap-run <anchors.csv> [--out-prefix <path>]");
        eprintln!("       gaia_dr4_probe --tap-adql <ADQL> [--out-prefix <path>]");
        std::process::exit(2);
    };
    let body = match std::fs::read_to_string(&path) {
        Ok(body) => body,
        Err(e) => {
            eprintln!("{path}: {e}");
            std::process::exit(1);
        }
    };
    let Some(JsonVal::Arr(rows)) = votable_to_json(&body) else {
        eprintln!("{path}: VOTable did not parse into rows");
        std::process::exit(1);
    };
    println!("file {path}");
    println!("rows {}", rows.len());

    let mut groups: BTreeMap<(i64, i64), Vec<(f64, f64, f64, f64)>> = BTreeMap::new();
    let mut ids: BTreeMap<(i64, i64), f64> = BTreeMap::new();
    for row in &rows {
        let ra0 = jnum(row, "ra0");
        let dec0 = jnum(row, "dec0");
        let (Some(ra0), Some(dec0)) = (ra0, dec0) else {
            continue;
        };
        let key = ((ra0 * 1e6).round() as i64, (dec0 * 1e6).round() as i64);
        if let Some(id) = jnum(row, "source_id") {
            ids.entry(key).or_insert(id);
        }
        let times = nums(field(row, "obs_time_tcb"));
        let al = nums(field(row, "centroid_pos_al"));
        let pa = nums(field(row, "scan_pos_angle"));
        let Some(pf) = jnum(row, "parallax_factor_al") else {
            continue;
        };
        let used = bools(field(row, "used_by_agis_al"));
        let entry = groups.entry(key).or_default();
        for i in 0..times.len().min(al.len()).min(pa.len()) {
            if !used.is_empty() && i < used.len() && !used[i] {
                continue;
            }
            let years = times[i] / 1e9 / 86400.0 / 365.25;
            entry.push((years, al[i], pa[i], pf));
        }
    }

    println!("sources {}", groups.len());
    let mut matched = 0usize;
    let mut summary =
        String::from("anchor,source_id,ra0,dec0,epochs,span_a,pm_ra,pm_dec,plx,rms\n");
    for (key, epochs) in &groups {
        let ts: Vec<f64> = epochs.iter().map(|(t, _, _, _)| *t).collect();
        let al: Vec<f64> = epochs.iter().map(|(_, a, _, _)| *a).collect();
        let pa: Vec<f64> = epochs.iter().map(|(_, _, p, _)| *p).collect();
        let pf: Vec<f64> = epochs.iter().map(|(_, _, _, f)| *f).collect();
        let span = ts
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), t| {
                (lo.min(*t), hi.max(*t))
            });
        let use_plx = pf.iter().any(|p| p.abs() > 1e-9);
        let fit: Option<(f64, f64, Option<f64>, f64)> = if use_plx {
            fit_pm4(&ts, &al, &pa, &pf).map(|(a, d, p, r)| (a, d, Some(p), r))
        } else {
            fit_pm(&ts, &al, &pa).map(|(a, d, r)| (a, d, None, r))
        };
        let id_text = match ids.get(key) {
            Some(id) => format!("{id:.0}"),
            None => "n/a".to_string(),
        };
        let anchor_id = if anchors.is_empty() {
            None
        } else {
            let ra = key.0 as f64 / 1e6;
            let dec = key.1 as f64 / 1e6;
            let mut best: Option<(f64, &Anchor)> = None;
            for anchor in &anchors {
                let s = sep_arcsec(ra, dec, anchor.ra, anchor.dec);
                if s <= 2.0 && best.map(|(b, _)| s < b).unwrap_or(true) {
                    best = Some((s, anchor));
                }
            }
            best.map(|(_, a)| a.id.clone())
        };
        if anchor_id.is_some() {
            matched += 1;
        }
        let anchor_text = match &anchor_id {
            Some(id) => format!("anchor {id}"),
            None => "anchor -".to_string(),
        };
        let Some((pm_ra, pm_dec, plx, rms)) = fit else {
            println!(
                "source_id {id_text} ra0 {:.6} dec0 {:.6} | fit absent",
                key.0 as f64 / 1e6,
                key.1 as f64 / 1e6
            );
            continue;
        };
        let plx_text = match plx {
            Some(p) => format!("{p:+.4} mas"),
            None => "n/a".to_string(),
        };
        let plx_field = match plx {
            Some(p) => format!("{p:+.4}"),
            None => "n/a".to_string(),
        };
        println!(
            "source_id {id_text} ra0 {:.6} dec0 {:.6} | epochs {} | span {:.2} a | pm_ra {:+.4} mas/a | pm_dec {:+.4} mas/a | plx {} | rms {:.3} mas | {anchor_text}",
            key.0 as f64 / 1e6,
            key.1 as f64 / 1e6,
            epochs.len(),
            span.1 - span.0,
            pm_ra,
            pm_dec,
            plx_text,
            rms
        );
        let anchor_field = match &anchor_id {
            Some(id) => id.as_str(),
            None => "",
        };
        summary.push_str(&format!(
            "{},{},{:.6},{:.6},{},{:.2},{:+.4},{:+.4},{},{:.3}\n",
            anchor_field,
            id_text,
            key.0 as f64 / 1e6,
            key.1 as f64 / 1e6,
            epochs.len(),
            span.1 - span.0,
            pm_ra,
            pm_dec,
            plx_field,
            rms
        ));
    }
    println!("anchor matches {matched}/{}", groups.len());
    if let Some(path) = summary_out {
        match std::fs::write(&path, &summary) {
            Ok(()) => println!("summary {} rows -> {path}", groups.len()),
            Err(e) => eprintln!("summary {path}: {e}"),
        }
    }
}
