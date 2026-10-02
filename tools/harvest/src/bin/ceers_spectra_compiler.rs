use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage, FitsTable};
use omegaflow::jwst::{
    JWST_REDSHIFT_ABSENT, JWST_Z_ABSENT, JWST_Z_SPEC, JwstSpectrum, bins_from_jwst_rows,
    collect_table, finalize_workdir, ledger_append, ledger_done, mjd_to_unix, parse_jwst_bin,
    reduce_table, write_sidecar,
};
use omegaflow::lsk::parse as parse_lsk;
use std::collections::HashSet;
use std::process::Command;
use std::time::{Duration, Instant};

const CEERS_BASE: &str = "https://web.corral.tacc.utexas.edu/ceersdata/DR07/NIRSpec";
const CEERS_ORIGIN: &str = "web.corral.tacc.utexas.edu";
const DAWN_Z_API: &str = "https://grizli-cutout.herokuapp.com/nirspec_extractions";
const DAWN_Z_RADIUS_ARCSEC: f64 = 10.0;
const DAWN_Z_MATCH_ARCSEC: f64 = 1.0;

fn ceers_url(base: &str, nirspec: u32, disperser: &str, msa: u32) -> String {
    format!(
        "{base}/nirspec{nirspec}/{disperser}/hlsp_ceers_jwst_nirspec_nirspec{nirspec}-{msa:06}_{disperser}_v0.7_x1d.fits"
    )
}

fn product_stem(label: &str) -> String {
    let name = label.rsplit('/').next().unwrap_or(label);
    name.strip_suffix(".fits").unwrap_or(name).to_string()
}

fn msa_from_stem(stem: &str) -> Option<u32> {
    let bytes = stem.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] != b'-' || i + 7 > bytes.len() {
            continue;
        }
        let digits = &bytes[i + 1..i + 7];
        if !digits.iter().all(|b| b.is_ascii_digit()) {
            continue;
        }
        if i + 7 < bytes.len() && bytes[i + 7] != b'_' {
            continue;
        }
        let text = std::str::from_utf8(digits).ok()?;
        return text.parse::<u32>().ok();
    }
    None
}

struct DawnRow {
    msa: Option<u32>,
    srcid: Option<i64>,
    ra: f64,
    dec: f64,
    z: f64,
    grade: Option<i64>,
    sn50: Option<f64>,
    exptime: Option<f64>,
}

fn dawn_file_msa(file: &str) -> Option<u32> {
    let core = file.strip_suffix(".spec.fits")?;
    core.rsplit('_').next()?.parse::<u32>().ok()
}

fn curl_text(url: &str, timeout_secs: u64) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("--retry")
        .arg("2")
        .arg("--retry-delay")
        .arg("2")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg(timeout_secs.to_string())
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!("dawn z {}: exit {} — z stays absent", url, out.status);
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

fn parse_dawn_z(text: &str) -> Vec<DawnRow> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let cols: Vec<&str> = header.split(',').map(|c| c.trim()).collect();
    let idx = |name: &str| cols.iter().position(|c| *c == name);
    let (Some(i_src), Some(i_ra), Some(i_dec), Some(i_z)) =
        (idx("srcid"), idx("ra"), idx("dec"), idx("z"))
    else {
        return Vec::new();
    };
    let i_grade = idx("grade");
    let i_sn = idx("sn50");
    let i_exp = idx("exptime");
    let i_file = idx("file");
    let mut out = Vec::new();
    for line in lines {
        let cells: Vec<&str> = line.split(',').collect();
        let get = |i: usize| cells.get(i).map(|s| s.trim());
        let Some(ra) = get(i_ra)
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite())
        else {
            continue;
        };
        let Some(dec) = get(i_dec)
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite())
        else {
            continue;
        };
        let Some(z) = get(i_z)
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite() && *v > 0.0)
        else {
            continue;
        };
        let srcid = get(i_src).and_then(|s| s.parse::<i64>().ok());
        let msa = i_file.and_then(|i| get(i)).and_then(dawn_file_msa);
        let grade = i_grade
            .and_then(|i| get(i))
            .and_then(|s| s.parse::<i64>().ok());
        let sn50 = i_sn
            .and_then(|i| get(i))
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite());
        let exptime = i_exp
            .and_then(|i| get(i))
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite());
        out.push(DawnRow {
            msa,
            srcid,
            ra,
            dec,
            z,
            grade,
            sn50,
            exptime,
        });
    }
    out
}

fn dawn_stronger(a: &DawnRow, b: &DawnRow) -> bool {
    match (a.grade, b.grade) {
        (Some(x), Some(y)) if x != y => x > y,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        _ => match (a.sn50, b.sn50) {
            (Some(x), Some(y)) if x != y => x > y,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            _ => match (a.exptime, b.exptime) {
                (Some(x), Some(y)) => x > y,
                (Some(_), None) => true,
                _ => false,
            },
        },
    }
}

fn angular_sep_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let d2r = std::f64::consts::PI / 180.0;
    let ddec = (dec2 - dec1) * d2r;
    let dra = (ra2 - ra1) * d2r * dec1.to_radians().cos().abs();
    (dra * dra + ddec * ddec).sqrt() / d2r * 3600.0
}

fn nearest_dawn_row<'a>(rows: &'a [DawnRow], ra: f64, dec: f64) -> Option<&'a DawnRow> {
    let mut best: Option<(&DawnRow, f64)> = None;
    for row in rows {
        let sep = angular_sep_arcsec(ra, dec, row.ra, row.dec);
        if !sep.is_finite() {
            continue;
        }
        let take = match best {
            Some((_, s)) => sep < s,
            None => true,
        };
        if take {
            best = Some((row, sep));
        }
    }
    best.and_then(|(row, sep)| {
        if sep <= DAWN_Z_MATCH_ARCSEC {
            Some(row)
        } else {
            None
        }
    })
}

fn dawn_z(ra: f64, dec: f64, msa: Option<u32>) -> Option<(f64, u8)> {
    let url = format!("{DAWN_Z_API}?coords={ra},{dec}&size={DAWN_Z_RADIUS_ARCSEC}&output=csv");
    let text = curl_text(&url, 30)?;
    let rows = parse_dawn_z(&text);
    if rows.is_empty() {
        return None;
    }
    let by_id = msa.and_then(|m| {
        let target = m as i64;
        let mut best: Option<&DawnRow> = None;
        for row in &rows {
            if row.msa == Some(m) || row.srcid == Some(target) {
                best = match best {
                    Some(b) if !dawn_stronger(row, b) => Some(b),
                    _ => Some(row),
                };
            }
        }
        best
    });
    let pick = by_id.or_else(|| nearest_dawn_row(&rows, ra, dec));
    pick.map(|row| (row.z, JWST_Z_SPEC))
}

fn curl_get(url: &str, dest: &str) -> bool {
    match Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("--retry")
        .arg("3")
        .arg("--retry-delay")
        .arg("2")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg("600")
        .arg("-o")
        .arg(dest)
        .arg(url)
        .status()
    {
        Ok(s) if s.success() => true,
        Ok(s) => {
            eprintln!("curl {}: exit {}", url, s);
            false
        }
        Err(e) => {
            eprintln!("curl {}: {}", url, e);
            false
        }
    }
}

struct X1dMeta {
    rows: Vec<(f64, f64)>,
    ra_deg: f64,
    dec_deg: f64,
    epoch_mjd: f64,
    host: String,
    obs_id_primary: Option<String>,
}

fn x1d_meta(bytes: &[u8], label: &str) -> Option<X1dMeta> {
    let (primary, mut off) = FitsHeader::parse(bytes, 0)?;
    let mut ra = primary.f64("TARG_RA");
    let mut dec = primary.f64("TARG_DEC");
    let mut rows: Option<Vec<(f64, f64)>> = None;
    while off + 80 <= bytes.len() {
        let Some((hdr, _)) = FitsHeader::parse(bytes, off) else {
            break;
        };
        if hdr.value("XTENSION") == Some("'BINTABLE'") {
            let Some((t, next)) = FitsTable::parse(bytes, off) else {
                break;
            };
            if t.column("WAVELENGTH").is_some() {
                if let Some(v) = hdr.f64("SRCRA") {
                    ra = Some(v);
                }
                if let Some(v) = hdr.f64("SRCDEC") {
                    dec = Some(v);
                }
                if rows.is_none() {
                    rows = collect_table(&t, bytes).and_then(|p| reduce_table(p, label));
                }
            }
            off = next;
        } else {
            let Some((_img, next)) = FitsImage::parse(bytes, off) else {
                break;
            };
            off = next;
        }
    }
    let ra_deg = ra?;
    let dec_deg = dec?;
    let epoch_mjd = primary
        .f64("MJD-AVG")
        .or_else(|| primary.f64("EXPMID"))
        .or_else(|| primary.f64("MJD-OBS"))
        .or_else(|| primary.f64("EXPSTART"))?;
    let host = primary
        .str_unescaped("HLSPTARG")
        .or_else(|| primary.str_unescaped("TARGPROP"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())?;
    let obs_id_primary = primary
        .str_unescaped("OBS_ID")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    Some(X1dMeta {
        rows: rows?,
        ra_deg,
        dec_deg,
        epoch_mjd,
        host,
        obs_id_primary,
    })
}

fn probe_fits(path: &str) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return;
        }
    };
    let (h, mut off) = match FitsHeader::parse(&bytes, 0) {
        Some(v) => v,
        None => {
            eprintln!("primary header: absent");
            return;
        }
    };
    eprintln!(
        "primary: FILENAME={:?} OBS_ID={:?} HLSPTARG={:?} TARGPROP={:?} TARG_RA={:?} TARG_DEC={:?} EXPMID={:?} MJD-AVG={:?}",
        h.str_unescaped("FILENAME"),
        h.str_unescaped("OBS_ID"),
        h.str_unescaped("HLSPTARG"),
        h.str_unescaped("TARGPROP"),
        h.f64("TARG_RA"),
        h.f64("TARG_DEC"),
        h.f64("EXPMID"),
        h.f64("MJD-AVG")
    );
    let mut ext = 0;
    while off + 80 <= bytes.len() {
        let Some((hdr, _)) = FitsHeader::parse(&bytes, off) else {
            break;
        };
        if hdr.value("XTENSION") == Some("'BINTABLE'") {
            let Some((t, next)) = FitsTable::parse(&bytes, off) else {
                break;
            };
            ext += 1;
            let names: Vec<String> = t.columns.iter().map(|c| c.name.clone()).collect();
            eprintln!(
                "ext {}: rows {} row_bytes {} cols {:?}",
                ext, t.n_rows, t.row_bytes, names
            );
            if let Some(p) = collect_table(&t, &bytes) {
                eprintln!(
                    "  first bin: wl {:?} flux {:?}",
                    p.axis.first(),
                    p.flux_rows.first().and_then(|f| f.first())
                );
            }
            off = next;
        } else {
            let Some((_img, next)) = FitsImage::parse(&bytes, off) else {
                break;
            };
            off = next;
        }
    }
    match x1d_meta(&bytes, path) {
        Some(m) => {
            let msa = msa_from_stem(&product_stem(path));
            let (z, kind) = match dawn_z(m.ra_deg, m.dec_deg, msa) {
                Some(v) => v,
                None => (JWST_REDSHIFT_ABSENT, JWST_Z_ABSENT),
            };
            eprintln!(
                "reduced: {} bins, ra {:.6} dec {:.6} epoch_mjd {:.6} host {} obs_id {:?} msa {:?} z {} kind {}",
                m.rows.len(),
                m.ra_deg,
                m.dec_deg,
                m.epoch_mjd,
                m.host,
                m.obs_id_primary,
                msa,
                z,
                kind
            );
        }
        None => eprintln!("reduced: void"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out: Option<String> = None;
    let mut lsk_path: Option<String> = None;
    let mut workdir = std::path::PathBuf::from("phi/ceers_harvest");
    let mut ci_mode = false;
    let mut limit = usize::MAX;
    let mut budget: Option<u64> = None;
    let mut probe: Option<String> = None;
    let mut urls: Vec<String> = Vec::new();
    let mut msas: Vec<u32> = Vec::new();
    let mut msa_csv: Option<String> = None;
    let mut base = CEERS_BASE.to_string();
    let mut nirspec: u32 = 4;
    let mut disperser = "prism".to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--lsk" => {
                lsk_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--workdir" => {
                if let Some(v) = args.get(i + 1) {
                    workdir = std::path::PathBuf::from(v);
                }
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
            "--budget" => {
                budget = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            "--url" => {
                if let Some(v) = args.get(i + 1) {
                    urls.push(v.clone());
                }
                i += 1;
            }
            "--msa" => {
                if let Some(v) = args.get(i + 1).and_then(|s| s.parse().ok()) {
                    msas.push(v);
                }
                i += 1;
            }
            "--msa-csv" => {
                msa_csv = args.get(i + 1).cloned();
                i += 1;
            }
            "--base" => {
                if let Some(v) = args.get(i + 1) {
                    base = v.clone();
                }
                i += 1;
            }
            "--nirspec" => {
                if let Some(v) = args.get(i + 1).and_then(|s| s.parse().ok()) {
                    nirspec = v;
                }
                i += 1;
            }
            "--disperser" => {
                if let Some(v) = args.get(i + 1) {
                    disperser = v.clone();
                }
                i += 1;
            }
            _ => {
                eprintln!(
                    "usage: ceers_spectra_compiler --out <ceers_spectra.bin> [--url <x1d.fits>]... [--msa <N>]... [--msa-csv <master_yield.csv>] [--base <url>] [--nirspec <P>] [--disperser <prism|g140m|g235m|g395m|comb-mgrat>] [--lsk <naif0012.tls>] [--workdir <dir>] [--budget <minutes>] [--limit N] [--ci-mode] [--probe <x1d.fits>]"
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
    if let Some(csv) = &msa_csv {
        match std::fs::read_to_string(csv) {
            Ok(body) => {
                for line in body.lines() {
                    let field = line.split(',').next().unwrap_or("").trim();
                    if field.is_empty() || field.starts_with('#') {
                        continue;
                    }
                    if let Ok(id) = field.parse::<u32>() {
                        msas.push(id);
                    }
                }
            }
            Err(e) => eprintln!("msa-csv {}: {}", csv, e),
        }
    }
    for m in &msas {
        urls.push(ceers_url(&base, nirspec, &disperser, *m));
    }
    let mut seen = HashSet::new();
    let urls: Vec<String> = urls
        .into_iter()
        .filter(|u| seen.insert(u.clone()))
        .collect();
    if urls.is_empty() {
        eprintln!("no CEERS product named — nothing to fetch");
        return;
    }

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
        "workdir {}: {} completed products in the ledger",
        workdir.display(),
        done.len()
    );
    let tmp_dir = workdir.join("tmp");
    let _ = std::fs::create_dir_all(&tmp_dir);

    let start = Instant::now();
    let deadline = budget.map(|m| start + Duration::from_secs(m.saturating_mul(60)));
    let mut harvested = 0usize;
    let mut resumed = 0usize;
    let mut named_skips = 0usize;
    let mut budget_stopped = false;

    for (n, url) in urls.iter().enumerate() {
        if harvested >= limit {
            break;
        }
        if let Some(d) = deadline {
            if Instant::now() >= d {
                budget_stopped = true;
                break;
            }
        }
        let stem = product_stem(url);
        if done.contains(&stem) {
            resumed += 1;
            continue;
        }
        let tmp = tmp_dir
            .join(format!("{}.fits", stem))
            .to_string_lossy()
            .into_owned();
        if !curl_get(url, &tmp) {
            named_skips += 1;
            continue;
        }
        let bytes = match std::fs::read(&tmp) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {}: {}", tmp, e);
                named_skips += 1;
                let _ = std::fs::remove_file(&tmp);
                continue;
            }
        };
        let (meta, n_bytes) = match x1d_meta(&bytes, url) {
            Some(m) => (m, bytes.len()),
            None => {
                eprintln!(
                    "[{}] {}: no readable EXTRACT1D spectrum — the product stays unread",
                    n, stem
                );
                named_skips += 1;
                let _ = std::fs::remove_file(&tmp);
                continue;
            }
        };
        let _ = std::fs::remove_file(&tmp);
        eprintln!("[{}] {} bytes, {} bins", n, n_bytes, meta.rows.len());
        if meta.host.len() > omegaflow::jwst::JWST_HOST_BYTES
            || stem.len() > omegaflow::jwst::JWST_OBSID_BYTES
        {
            eprintln!(
                "[{}] {} {}: name exceeds the bin buffer — the record is skipped",
                n, meta.host, stem
            );
            named_skips += 1;
            continue;
        }
        let epoch_tdb = match lsk.unix_to_tdb(mjd_to_unix(meta.epoch_mjd)) {
            Some(t) => t,
            None => {
                eprintln!(
                    "[{}] {} {}: epoch stays void — the record is skipped",
                    n, meta.host, stem
                );
                named_skips += 1;
                continue;
            }
        };
        let bins = bins_from_jwst_rows(&meta.rows);
        if bins.is_empty() {
            eprintln!(
                "[{}] {} {}: no valid frequency bins — the record is skipped",
                n, meta.host, stem
            );
            named_skips += 1;
            continue;
        }
        let msa = msa_from_stem(&stem);
        let (redshift, z_kind) = match dawn_z(meta.ra_deg, meta.dec_deg, msa) {
            Some(v) => v,
            None => (JWST_REDSHIFT_ABSENT, JWST_Z_ABSENT),
        };
        let spec = JwstSpectrum {
            ra_deg: meta.ra_deg,
            dec_deg: meta.dec_deg,
            plx_mas: 0.0,
            epoch_tdb,
            host: meta.host.clone(),
            obs_id: stem.clone(),
            redshift,
            z_kind,
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
                "[{}] {} {}: sidecar/ledger write void — the record is lost (named)",
                n, meta.host, stem
            );
            named_skips += 1;
            continue;
        }
        harvested += 1;
        eprintln!(
            "[{}] {} {}: {} bins, epoch_tdb {}, z {} kind {}",
            n,
            meta.host,
            stem,
            spec.bins.len(),
            spec.epoch_tdb,
            spec.redshift,
            spec.z_kind
        );
    }

    eprintln!(
        "\nharvest: {} new products, {} resumed from the ledger, {} named skips",
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
            let spec_z = parsed
                .iter()
                .filter(|s| s.z_kind == JWST_Z_SPEC && s.redshift.is_finite() && s.redshift > 0.0)
                .count();
            eprintln!(
                "{}: {} records, {} B, {} carrying spec-z — roundtrip parses",
                out_path,
                parsed.len(),
                bytes.len(),
                spec_z
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
    if ci_mode && !upload_release(CEERS_ORIGIN, &out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
