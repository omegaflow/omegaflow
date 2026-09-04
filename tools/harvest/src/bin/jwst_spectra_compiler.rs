use omegaflow::cdn::upload_asset;
use omegaflow::fits::{FitsHeader, FitsTable};
use omegaflow::json::{jnum, jpath_val, jstr, parse_json, JsonVal};
use omegaflow::jwst::{
    bins_from_jwst_rows, finalize_workdir, ledger_append, ledger_done, mjd_to_unix, parse_jwst_bin,
    write_sidecar, JwstSpectrum,
};
use omegaflow::lsk::parse as parse_lsk;
use std::io::Write;
use std::process::Command;
use std::time::{Duration, Instant};

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from(".")
}

fn mast_token() -> Option<String> {
    if let Ok(t) = std::env::var("MAST_TOKEN") {
        if !t.is_empty() {
            return Some(t);
        }
    }
    let body = std::fs::read_to_string(state_dir().join(".secrets.local")).ok()?;
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
        .arg("--retry")
        .arg("3")
        .arg("--retry-delay")
        .arg("2")
        .arg("--retry-all-errors")
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
        .arg("--retry")
        .arg("3")
        .arg("--retry-delay")
        .arg("2")
        .arg("--retry-all-errors")
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
    host: String,
    ra_deg: f64,
    dec_deg: f64,
    plx_mas: f64,
}

fn tap_targets(token: &str, limit: usize) -> Vec<Target> {
    let adql = "SELECT pl_name,hostname,ra,dec,sy_dist FROM pscomppars WHERE pl_tranmid IS NOT NULL AND sy_dist IS NOT NULL";
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
    let rows: &[JsonVal] = match jpath_val(&root, "data") {
        Some(JsonVal::Arr(a)) => a,
        _ => match &root {
            JsonVal::Arr(a) => a,
            _ => {
                eprintln!("tap targets: data array absent");
                return Vec::new();
            }
        },
    };
    let mut seen = std::collections::HashSet::new();
    let mut targets = Vec::new();
    for row in rows {
        if targets.len() >= limit {
            break;
        }
        let Some(host) = jstr(row, "hostname") else {
            continue;
        };
        if !seen.insert(host.clone()) {
            continue;
        }
        let (Some(ra), Some(dec)) = (jnum(row, "ra"), jnum(row, "dec")) else {
            continue;
        };
        let plx = match jnum(row, "sy_dist") {
            Some(d) if d > 0.0 => 1000.0 / d,
            _ => 0.0,
        };
        targets.push(Target {
            host,
            ra_deg: ra,
            dec_deg: dec,
            plx_mas: plx,
        });
    }
    targets
}

fn tap_curated_targets(token: &str) -> Vec<Target> {
    let adql = "SELECT DISTINCT p.hostname, p.ra, p.dec FROM spectra s, ps p WHERE s.pl_name = p.pl_name AND p.default_flag = 1 AND p.ra IS NOT NULL AND p.dec IS NOT NULL AND s.facility LIKE '%James Webb Space Telescope%' AND s.spec_type = 'Transmission' AND (s.instrument LIKE '%NIRSpec%' OR s.instrument LIKE '%NIRISS%' OR s.instrument LIKE '%MIRI%')";
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
        eprintln!("tap curated: json absent");
        return Vec::new();
    };
    let rows: &[JsonVal] = match jpath_val(&root, "data") {
        Some(JsonVal::Arr(a)) => a,
        _ => match &root {
            JsonVal::Arr(a) => a,
            _ => {
                eprintln!("tap curated: data array absent");
                return Vec::new();
            }
        },
    };
    let mut seen = std::collections::HashSet::new();
    let mut targets = Vec::new();
    for row in rows {
        let Some(host) = jstr(row, "hostname") else {
            continue;
        };
        if !seen.insert(host.clone()) {
            continue;
        }
        let (Some(ra), Some(dec)) = (jnum(row, "ra"), jnum(row, "dec")) else {
            continue;
        };
        targets.push(Target {
            host,
            ra_deg: ra,
            dec_deg: dec,
            plx_mas: 0.0,
        });
    }
    targets
}

struct CaomRow {
    obs_id: String,
    s_ra: f64,
    s_dec: f64,
    t_min: f64,
    t_max: f64,
}

fn caom_jwst_timeseries(token: &str) -> Vec<CaomRow> {
    let request = r#"{"service":"Mast.Caom.Filtered","params":{"columns":"*","filters":[{"paramName":"obs_collection","values":["JWST"]},{"paramName":"dataproduct_type","values":["timeseries"]},{"paramName":"calib_level","min":3}]},"format":"json","pagesize":5000,"removenullcolumns":true}"#;
    let body = match curl_json(
        "https://mast.stsci.edu/api/v0/invoke",
        &[("request", request.to_string())],
        token,
    ) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let Some(root) = parse_json(&body) else {
        eprintln!("caom: json absent");
        return Vec::new();
    };
    let mut rows = Vec::new();
    let mut seen = std::collections::HashSet::new();
    if let Some(JsonVal::Arr(arr)) = jpath_val(&root, "data") {
        for row in arr {
            let Some(obs_id) = jstr(row, "obs_id") else {
                continue;
            };
            if !seen.insert(obs_id.clone()) {
                continue;
            }
            let (Some(s_ra), Some(s_dec)) = (jnum(row, "s_ra"), jnum(row, "s_dec")) else {
                continue;
            };
            let (Some(t_min), Some(t_max)) = (jnum(row, "t_min"), jnum(row, "t_max")) else {
                continue;
            };
            let instrument = jstr(row, "instrument_name").unwrap_or_default();
            if !(instrument.starts_with("NIRISS")
                || instrument.starts_with("NIRSPEC")
                || instrument.starts_with("MIRI"))
            {
                continue;
            }
            let rights = jstr(row, "dataRights").unwrap_or_default();
            if rights == "EXCLUSIVE_ACCESS" || rights == "PROPRIETARY" {
                continue;
            }
            rows.push(CaomRow {
                obs_id,
                s_ra,
                s_dec,
                t_min,
                t_max,
            });
        }
    }
    rows
}

fn angular_distance_deg(a_ra: f64, a_dec: f64, b_ra: f64, b_dec: f64) -> f64 {
    let d_ra = (a_ra - b_ra).to_radians();
    let (s_a, c_a) = a_dec.to_radians().sin_cos();
    let (s_b, c_b) = b_dec.to_radians().sin_cos();
    let cos_ang = s_a * s_b + c_a * c_b * d_ra.cos();
    cos_ang.acos().to_degrees()
}

struct TablePickup {
    axis: Vec<f64>,
    flux_rows: Vec<Vec<f64>>,
    dq_rows: Vec<Option<Vec<i64>>>,
}

fn collect_table(t: &FitsTable, bytes: &[u8]) -> Option<TablePickup> {
    let wl_col = t.column("WAVELENGTH")?;
    let flux_col = t.column("FLUX")?;
    let dq_col = t.column("DQ");
    if wl_col.repeat <= 1 {
        let mut axis = Vec::with_capacity(t.n_rows);
        let mut flux = Vec::with_capacity(t.n_rows);
        let mut dq_row: Option<Vec<i64>> = None;
        for r in 0..t.n_rows {
            axis.push(t.cell_f64(bytes, r, wl_col)?);
            flux.push(t.cell_f64(bytes, r, flux_col)?);
            if let Some(c) = dq_col {
                dq_row
                    .get_or_insert_with(|| Vec::with_capacity(t.n_rows))
                    .push(t.cell_i64(bytes, r, c)?);
            }
        }
        return Some(TablePickup {
            axis,
            flux_rows: vec![flux],
            dq_rows: vec![dq_row],
        });
    }
    let mut axis: Option<Vec<f64>> = None;
    let mut flux_rows = Vec::with_capacity(t.n_rows);
    let mut dq_rows = Vec::with_capacity(t.n_rows);
    for r in 0..t.n_rows {
        let wl = t.cell_array_f64(bytes, r, wl_col)?;
        let flux = t.cell_array_f64(bytes, r, flux_col)?;
        if wl.len() != flux.len() || wl.is_empty() {
            return None;
        }
        let dq = match dq_col {
            Some(c) => {
                let d = t.cell_array_i64(bytes, r, c)?;
                if d.len() != wl.len() {
                    return None;
                }
                Some(d)
            }
            None => None,
        };
        match &axis {
            None => axis = Some(wl),
            Some(a) => {
                if a.len() != wl.len() {
                    return None;
                }
                for (x, y) in a.iter().zip(wl.iter()) {
                    if (x - y).abs() / x.max(1e-30) > 1e-9 {
                        return None;
                    }
                }
            }
        }
        flux_rows.push(flux);
        dq_rows.push(dq);
    }
    Some(TablePickup {
        axis: axis?,
        flux_rows,
        dq_rows,
    })
}

fn median(vals: &mut Vec<f64>) -> f64 {
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = vals.len();
    if n % 2 == 1 {
        vals[n / 2]
    } else {
        (vals[n / 2 - 1] + vals[n / 2]) * 0.5
    }
}

fn reduce_table(pickup: TablePickup, name: &str) -> Option<Vec<(f64, f64)>> {
    let n = pickup.axis.len();
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let wl = pickup.axis[i];
        if !wl.is_finite() || wl <= 0.0 {
            continue;
        }
        let mut vals = Vec::with_capacity(pickup.flux_rows.len());
        for (r, f) in pickup.flux_rows.iter().enumerate() {
            if let Some(Some(dq)) = pickup.dq_rows.get(r) {
                if dq[i] & 1 != 0 {
                    continue;
                }
            }
            let v = f[i];
            if v.is_finite() && v > 0.0 {
                vals.push(v);
            }
        }
        if vals.is_empty() {
            continue;
        }
        out.push((wl, median(&mut vals)));
    }
    if out.is_empty() {
        eprintln!("{}: no valid bins — the spectrum stays void", name);
        return None;
    }
    Some(out)
}

fn fits_spectrum_rows(bytes: &[u8], name: &str) -> Option<Vec<(f64, f64)>> {
    let mut off = 0usize;
    if let Some((_, data_end)) = FitsHeader::parse(bytes, 0) {
        off = data_end;
    }
    let mut all: Vec<(f64, f64)> = Vec::new();
    while off + 80 <= bytes.len() {
        let Some((t, next)) = FitsTable::parse(bytes, off) else {
            break;
        };
        if let Some(pickup) = collect_table(&t, bytes) {
            if let Some(reduced) = reduce_table(pickup, name) {
                all.extend(reduced);
            }
        }
        off = next;
    }
    if all.is_empty() {
        None
    } else {
        all.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        Some(all)
    }
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
        let rows = collect_table(&t, &bytes);
        let n_bins = rows.as_ref().map(|r| r.axis.len()).unwrap_or(0);
        eprintln!(
            "ext {}: rows {} row_bytes {} heap {} bins {} cols {:?}",
            ext, t.n_rows, t.row_bytes, t.heap_bytes, n_bins, names
        );
        if let Some(r) = &rows {
            if !r.axis.is_empty() {
                let dq0 = r.dq_rows.first().and_then(|d| d.as_ref()).map(|d| d[0]);
                eprintln!(
                    "  first bin: wl {} flux {:?} dq {:?}, first row bins {}",
                    r.axis[0],
                    r.flux_rows.first().map(|f| f[0]),
                    dq0,
                    r.flux_rows.first().map(|f| f.len()).unwrap_or(0)
                );
                for &i in &[500usize, 1000, 1500, 2000] {
                    if i >= r.axis.len() {
                        continue;
                    }
                    let dq = r.dq_rows.first().and_then(|d| d.as_ref()).map(|d| d[i]);
                    eprintln!(
                        "  bin {}: wl {} flux {:?} dq {:?}",
                        i,
                        r.axis[i],
                        r.flux_rows.first().map(|f| f[i]),
                        dq
                    );
                }
                if let Some(dq) = r.dq_rows.first().and_then(|d| d.as_ref()) {
                    let bit0 = dq.iter().filter(|v| *v & 1 != 0).count();
                    let bit31 = dq.iter().filter(|v| *v & (1 << 31) != 0).count();
                    let finite = r.flux_rows[0]
                        .iter()
                        .filter(|v| v.is_finite() && **v > 0.0)
                        .count();
                    eprintln!(
                        "  row0 flags: bit0 {}, bit31 {}, finite-positive flux {}",
                        bit0, bit31, finite
                    );
                }
            }
        }
        off = next;
        if ext >= 3 {
            break;
        }
    }
    match fits_spectrum_rows(&bytes, path) {
        Some(spec) => {
            eprintln!(
                "reduced: {} bins, first (wl {}, flux {:.6e}), last (wl {}, flux {:.6e})",
                spec.len(),
                spec[0].0,
                spec[0].1,
                spec[spec.len() - 1].0,
                spec[spec.len() - 1].1
            );
        }
        None => eprintln!("reduced: void"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut limit = usize::MAX;
    let mut probe: Option<String> = None;
    let mut lsk_path: Option<String> = None;
    let mut hosts_filter: Option<String> = None;
    let mut workdir = std::path::PathBuf::from("phi/jwst_harvest");
    let mut budget: Option<u64> = None;
    let mut curated = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--curated" => curated = true,
            "--limit" => {
                limit = args
                    .get(i + 1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(usize::MAX);
                i += 1;
            }
            "--lsk" => {
                lsk_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--hosts" => {
                hosts_filter = args.get(i + 1).cloned();
                i += 1;
            }
            "--workdir" => {
                workdir = std::path::PathBuf::from(args.get(i + 1).cloned().unwrap_or_default());
                i += 1;
            }
            "--budget" => {
                budget = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {
                eprintln!(
                    "usage: jwst_spectra_compiler --out <jwst_spectra.bin> [--ci-mode] [--limit N] [--hosts <host,...>] [--workdir <dir>] [--budget <minutes>] --lsk <naif0012.tls> [--probe <x1d.fits>]"
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
    let lsk = match lsk_path
        .as_deref()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| parse_lsk(&t))
    {
        Some(l) => l,
        None => {
            eprintln!(
                "--lsk absent or parses void — the TDB epoch stays void (no fabricated epoch)"
            );
            return;
        }
    };
    let token = mast_token().unwrap_or_default();
    if token.is_empty() {
        eprintln!("MAST_TOKEN absent (.secrets.local or env)");
        return;
    }
    let targets = if curated {
        tap_curated_targets(&token)
    } else {
        tap_targets(
            &token,
            if hosts_filter.is_some() {
                usize::MAX
            } else {
                limit
            },
        )
    };
    let host_set: Option<std::collections::HashSet<String>> =
        hosts_filter.map(|s| s.split(',').map(|t| t.trim().to_string()).collect());
    let targets: Vec<Target> = match &host_set {
        Some(set) => targets
            .into_iter()
            .filter(|t| set.contains(&t.host))
            .collect(),
        None => targets,
    };
    eprintln!("tap targets: {}", targets.len());
    let caom_rows = caom_jwst_timeseries(&token);
    eprintln!("caom jwst timeseries rows: {}", caom_rows.len());
    if std::fs::create_dir_all(&workdir).is_err() {
        eprintln!(
            "workdir {}: create void — the harvest stays unwritten",
            workdir.display()
        );
        return;
    }

    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(workdir.join(omegaflow::jwst::JWST_LEDGER));
    let done = ledger_done(&workdir);
    eprintln!(
        "workdir {}: {} completed obs in the ledger",
        workdir.display(),
        done.len()
    );
    let mut harvested = 0usize;
    let mut resumed = 0usize;
    let mut named_skips = 0usize;
    let start = Instant::now();
    let deadline = budget.map(|m| start + Duration::from_secs(m.saturating_mul(60)));
    let mut budget_stopped = false;
    'harvest: for (n, target) in targets.iter().enumerate() {
        let matches: Vec<&CaomRow> = caom_rows
            .iter()
            .filter(|r| {
                angular_distance_deg(target.ra_deg, target.dec_deg, r.s_ra, r.s_dec) <= 0.01
            })
            .collect();
        if matches.is_empty() {
            continue;
        }
        for row in matches {
            if done.contains(&row.obs_id) {
                resumed += 1;
                continue;
            }
            if let Some(d) = deadline {
                if Instant::now() >= d {
                    budget_stopped = true;
                    break 'harvest;
                }
            }
            if row.obs_id.len() > omegaflow::jwst::JWST_OBSID_BYTES
                || target.host.len() > omegaflow::jwst::JWST_HOST_BYTES
            {
                eprintln!(
                    "\r\x1b[K[{}] {} {}: name exceeds the bin buffer — the record is skipped",
                    n, target.host, row.obs_id
                );
                named_skips += 1;
                continue;
            }
            let mjd_mid = (row.t_min + row.t_max) * 0.5;
            let epoch_tdb = match lsk.unix_to_tdb(mjd_to_unix(mjd_mid)) {
                Some(t) => t,
                None => {
                    eprintln!(
                        "{} {}: epoch stays void — the record is skipped",
                        target.host, row.obs_id
                    );
                    named_skips += 1;
                    continue;
                }
            };
            let mut spec_rows: Option<Vec<(f64, f64)>> = None;
            for suffix in ["_x1d.fits", "_x1dints.fits"] {
                let tmp = format!(
                    "/tmp/opencode/jwst_{}_{}.fits",
                    row.obs_id,
                    if suffix.starts_with("_x1dints") {
                        "ints"
                    } else {
                        "x1d"
                    }
                );
                let uri = format!("mast:JWST/product/{}{}", row.obs_id, suffix);
                if !curl_bytes(
                    "https://mast.stsci.edu/api/v0.1/Download/file",
                    &[("uri", uri)],
                    &token,
                    &tmp,
                ) {
                    continue;
                }
                match std::fs::read(&tmp)
                    .ok()
                    .and_then(|b| fits_spectrum_rows(&b, &row.obs_id))
                {
                    Some(rows) => {
                        spec_rows = Some(rows);
                    }
                    None => {
                        eprintln!(
                            "{} {}: {} parses void — the spectrum stays unread",
                            target.host, row.obs_id, suffix
                        );
                    }
                }
                let _ = std::fs::remove_file(&tmp);
                if spec_rows.is_some() {
                    break;
                }
            }
            let Some(rows) = spec_rows else {
                eprintln!(
                    "\r\x1b[K[{}] {} {}: no readable spectrum product",
                    n, target.host, row.obs_id
                );
                named_skips += 1;
                continue;
            };
            let bins = bins_from_jwst_rows(&rows);
            if bins.is_empty() {
                named_skips += 1;
                continue;
            }
            let spec = JwstSpectrum {
                ra_deg: target.ra_deg,
                dec_deg: target.dec_deg,
                plx_mas: target.plx_mas,
                epoch_tdb,
                host: target.host.clone(),
                obs_id: row.obs_id.clone(),
                bins,
            };
            if !write_sidecar(&workdir, &spec)
                || !ledger_append(
                    &workdir,
                    &spec.obs_id,
                    &spec.host,
                    spec.bins.len(),
                    spec.epoch_tdb,
                )
            {
                eprintln!(
                    "\r\x1b[K[{}] {} {}: sidecar/ledger write void — the record is lost (named)",
                    n, target.host, row.obs_id
                );
                named_skips += 1;
                continue;
            }
            harvested += 1;
            eprintln!(
                "\r\x1b[K[{}] {} {}: {} bins, epoch_tdb {}",
                n,
                target.host,
                row.obs_id,
                spec.bins.len(),
                spec.epoch_tdb
            );
            let _ = std::io::stdout().flush();
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    }
    eprintln!(
        "\nharvest: {} new sidecars, {} resumed from the ledger, {} named skips",
        harvested, resumed, named_skips
    );
    if budget_stopped {
        eprintln!(
            "budget reached — partial harvest; the partial bin is still written and uploaded; resume continues from the ledger"
        );
    }
    let Some(bytes) = finalize_workdir(&workdir) else {
        eprintln!("finalize void — no sidecars, the bin stays unwritten (0 honored)");
        return;
    };
    match std::fs::write(&out_path, &bytes) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("write {}: {}", out_path, e);
            return;
        }
    }
    match parse_jwst_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{}: {} records, {} B — roundtrip parses",
                out_path,
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!(
                "{}: roundtrip parse void — the bin stays unverified",
                out_path
            );
            return;
        }
    }
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
