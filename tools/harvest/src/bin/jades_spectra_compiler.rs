use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage, FitsTable};
use omegaflow::jwst::{
    JWST_HOST_BYTES, JWST_LEDGER, JWST_OBSID_BYTES, JWST_REDSHIFT_ABSENT, JWST_Z_ABSENT,
    JWST_Z_PAPER, JWST_Z_PHOT, JWST_Z_SPEC, JwstSpectrum, finalize_workdir, ledger_append,
    ledger_done, parse_jwst_bin, write_sidecar,
};
use omegaflow::lsk::parse as parse_lsk;
use std::collections::{HashMap, HashSet};
use std::process::Command;
use std::time::{Duration, Instant};

const JADES_URL: &str = "https://jades.herts.ac.uk/DR4/Combined_DR4_external_v1.2.1.fits";
const JADES_ORIGIN: &str = "jades.herts.ac.uk";
const META_EXTNAME: &str = "Obs_info";
const LINE_EXTNAME: &str = "R100_5pix";
const LINE_EXTNAME_FALLBACK: &str = "R1000_5pix";
const Z_SPEC_COLUMN: &str = "z_Spec";
const Z_PHOT_COLUMN: &str = "z_phot";
const Z_PAPER_COLUMN: &str = "z_paper";

const C_LIGHT: f64 = 299_792_458.0;
const ANGSTROM_M: f64 = 1e-10;
const ERG_CM2_TO_W_M2: f64 = 1e-23;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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
        .arg("1800")
        .arg("-o")
        .arg(dest)
        .arg(url)
        .status()
    {
        Ok(s) if s.success() => true,
        Ok(s) => {
            eprintln!("curl {url}: exit {s}");
            false
        }
        Err(e) => {
            eprintln!("curl {url}: {e}");
            false
        }
    }
}

fn load_product(url: &str, input: Option<&str>, scratch: &std::path::Path) -> Option<Vec<u8>> {
    if let Some(path) = input {
        return match std::fs::read(path) {
            Ok(b) => Some(b),
            Err(e) => {
                eprintln!("read {path}: {e}");
                None
            }
        };
    }
    let dest = scratch.join("product.fits");
    let dest_s = dest.to_string_lossy().into_owned();
    if !curl_get(url, &dest_s) {
        return None;
    }
    std::fs::read(&dest).ok()
}

fn hdu_tables(bytes: &[u8]) -> Vec<(String, FitsTable)> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + 80 <= bytes.len() {
        let Some((header, _)) = FitsHeader::parse(bytes, off) else {
            break;
        };
        if header.value("XTENSION") == Some("'BINTABLE'") {
            let Some((table, next)) = FitsTable::parse(bytes, off) else {
                break;
            };
            let name = match header.str_unescaped("EXTNAME") {
                Some(v) => v,
                None => String::new(),
            };
            out.push((name.trim().to_string(), table));
            if next <= off {
                break;
            }
            off = next;
        } else {
            let Some((_image, next)) = FitsImage::parse(bytes, off) else {
                break;
            };
            if next <= off {
                break;
            }
            off = next;
        }
    }
    out
}

fn probe_fits(path: &str) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {path}: {e}");
            return;
        }
    };
    let mut off = 0usize;
    let mut hdu = 0usize;
    while off + 80 <= bytes.len() {
        let Some((header, _)) = FitsHeader::parse(&bytes, off) else {
            break;
        };
        if header.value("XTENSION") == Some("'BINTABLE'") {
            let Some((table, next)) = FitsTable::parse_lenient(&bytes, off) else {
                break;
            };
            let name = match header.str_unescaped("EXTNAME") {
                Some(v) => v,
                None => String::new(),
            };
            eprintln!(
                "hdu {hdu}: BINTABLE {} rows {} row_bytes {} cols {}",
                name.trim(),
                table.n_rows,
                table.row_bytes,
                table.columns.len()
            );
            if name.trim() == META_EXTNAME || name.trim() == LINE_EXTNAME {
                for c in &table.columns {
                    eprintln!("  col {} {} {}", c.name, c.code, c.repeat);
                }
            }
            if name.trim() == META_EXTNAME {
                for col_name in [Z_SPEC_COLUMN, Z_PHOT_COLUMN, Z_PAPER_COLUMN] {
                    let Some(c) = table.column(col_name) else {
                        eprintln!("  z {col_name}: absent");
                        continue;
                    };
                    let mut n = 0usize;
                    let mut lo = f64::INFINITY;
                    let mut hi = f64::NEG_INFINITY;
                    for r in 0..table.n_rows {
                        if let Some(v) = table.cell_f64(&bytes, r, c)
                            && v.is_finite()
                            && v > 0.0
                        {
                            n += 1;
                            lo = lo.min(v);
                            hi = hi.max(v);
                        }
                    }
                    if n == 0 {
                        eprintln!("  z {col_name}: no positive value");
                    } else {
                        eprintln!("  z {col_name}: {n} positive, min {lo}, max {hi}");
                    }
                }
            }
            if next <= off {
                break;
            }
            off = next;
        } else {
            let Some((_image, next)) = FitsImage::parse(&bytes, off) else {
                break;
            };
            eprintln!("hdu {hdu}: primary/image");
            if next <= off {
                break;
            }
            off = next;
        }
        hdu += 1;
    }
}

fn clean_id(raw: &str) -> String {
    raw.trim_end_matches('\0').trim().to_string()
}

fn parse_ymd(text: &str) -> Option<(i64, u32, u32)> {
    let mut parts = text.split('-');
    let y = parts.next()?.parse::<i64>().ok()?;
    let m = parts.next()?.parse::<u32>().ok()?;
    let d = parts.next()?.parse::<u32>().ok()?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some((y, m, d))
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp as i64 + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn line_angstrom(name: &str) -> Option<f64> {
    let last = name.rsplit('_').next()?;
    let digits = last.trim_end_matches(|c: char| c.is_ascii_alphabetic());
    digits.parse::<f64>().ok().filter(|v| *v > 0.0)
}

struct Target {
    unique_id: String,
    ra_deg: f64,
    dec_deg: f64,
    epoch_tdb: f64,
    line_row: usize,
    redshift: f64,
    z_kind: u8,
}

fn targets(
    bytes: &[u8],
    meta: &FitsTable,
    line: &FitsTable,
    lsk: &omegaflow::lsk::LeapSeconds,
    limit: usize,
) -> Vec<Target> {
    let Some(uid_c) = meta.column("Unique_ID") else {
        eprintln!("{META_EXTNAME}: Unique_ID column absent — the targets stay unread");
        return Vec::new();
    };
    let (Some(ra_c), Some(dec_c), Some(date_c)) = (
        meta.column("RA_TARG"),
        meta.column("Dec_TARG"),
        meta.column("ObsDate"),
    ) else {
        eprintln!("{META_EXTNAME}: RA_TARG/Dec_TARG/ObsDate absent — the targets stay unread");
        return Vec::new();
    };
    let Some(line_uid_c) = line.column("Unique_ID") else {
        eprintln!("{LINE_EXTNAME}: Unique_ID column absent — the line table stays unjoined");
        return Vec::new();
    };
    let z_arms = [
        (meta.column(Z_SPEC_COLUMN), JWST_Z_SPEC),
        (meta.column(Z_PHOT_COLUMN), JWST_Z_PHOT),
        (meta.column(Z_PAPER_COLUMN), JWST_Z_PAPER),
    ];
    let mut line_row_of: HashMap<String, usize> = HashMap::new();
    for r in 0..line.n_rows {
        if let Some(uid) = line.cell_str(bytes, r, line_uid_c) {
            line_row_of.entry(clean_id(uid)).or_insert(r);
        }
    }
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for r in 0..meta.n_rows {
        if out.len() >= limit {
            break;
        }
        let Some(uid) = meta.cell_str(bytes, r, uid_c).map(clean_id) else {
            continue;
        };
        if uid.is_empty() || !seen.insert(uid.clone()) {
            continue;
        }
        let (Some(ra), Some(dec)) = (
            meta.cell_f64(bytes, r, ra_c),
            meta.cell_f64(bytes, r, dec_c),
        ) else {
            continue;
        };
        if !(ra.is_finite() && dec.is_finite())
            || !(0.0..360.0).contains(&ra)
            || !(-90.0..=90.0).contains(&dec)
        {
            continue;
        }
        let Some(date) = meta.cell_str(bytes, r, date_c) else {
            continue;
        };
        let Some((y, m, d)) = parse_ymd(date) else {
            continue;
        };
        let Some(epoch_tdb) = lsk.unix_to_tdb(days_from_civil(y, m, d) as f64 * 86400.0) else {
            continue;
        };
        let Some(line_row) = line_row_of.get(&uid).copied() else {
            continue;
        };
        let (redshift, z_kind) = z_arms
            .iter()
            .find_map(|(col, kind)| {
                col.and_then(|c| meta.cell_f64(bytes, r, c))
                    .filter(|v| v.is_finite() && *v > 0.0)
                    .map(|v| (v, *kind))
            })
            .unwrap_or((JWST_REDSHIFT_ABSENT, JWST_Z_ABSENT));
        out.push(Target {
            unique_id: uid,
            ra_deg: ra,
            dec_deg: dec,
            epoch_tdb,
            line_row,
            redshift,
            z_kind,
        });
    }
    out
}

fn line_bins(bytes: &[u8], line: &FitsTable, row: usize) -> Vec<(f64, f64, f64)> {
    let mut bins = Vec::new();
    for c in &line.columns {
        let Some(token) = c.name.strip_suffix("_flux") else {
            continue;
        };
        let Some(lam_ang) = line_angstrom(token) else {
            continue;
        };
        let Some(flux) = line.cell_f64(bytes, row, c) else {
            continue;
        };
        if !flux.is_finite() || flux <= 0.0 {
            continue;
        }
        let freq = C_LIGHT / (lam_ang * ANGSTROM_M);
        bins.push((freq, 0.0, flux * ERG_CM2_TO_W_M2));
    }
    bins
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(p) = arg_value(&args, "--probe") {
        probe_fits(&p);
        return;
    }
    let mut out: Option<String> = None;
    let mut input: Option<String> = None;
    let mut url = JADES_URL.to_string();
    let mut lsk_path: Option<String> = None;
    let mut workdir = std::path::PathBuf::from("phi/jades_harvest");
    let mut ci_mode = false;
    let mut limit = usize::MAX;
    let mut budget: Option<u64> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--url" => {
                if let Some(v) = args.get(i + 1) {
                    url = v.clone();
                }
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
            "--ci-mode" => ci_mode = true,
            _ => {
                eprintln!(
                    "usage: jades_spectra_compiler --out <jades_spectra.bin> --lsk <naif0012.tls> [--input <local.fits>] [--url <route>] [--workdir <dir>] [--budget <minutes>] [--limit N] [--ci-mode] [--probe <fits>]"
                );
                return;
            }
        }
        i += 1;
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
    if std::fs::create_dir_all(&workdir).is_err() {
        eprintln!("workdir {}: create void", workdir.display());
        return;
    }
    let Some(bytes) = load_product(&url, input.as_deref(), &workdir) else {
        eprintln!("product fetch void — the harvest stays unwritten");
        return;
    };
    let tables = hdu_tables(&bytes);
    let meta = match tables.iter().find(|(n, _)| n == META_EXTNAME) {
        Some((_, t)) => t,
        None => {
            eprintln!("{META_EXTNAME}: BINTABLE absent — the product stays unread");
            return;
        }
    };
    let line = match tables
        .iter()
        .find(|(n, _)| n == LINE_EXTNAME)
        .or_else(|| tables.iter().find(|(n, _)| n == LINE_EXTNAME_FALLBACK))
    {
        Some((_, t)) => t,
        None => {
            eprintln!("{LINE_EXTNAME}: BINTABLE absent — the line fluxes stay unread");
            return;
        }
    };
    eprintln!(
        "product {} B: {} rows targets, {} rows line table",
        bytes.len(),
        meta.n_rows,
        line.n_rows
    );
    let targets = targets(&bytes, meta, line, &lsk, limit);
    eprintln!("targets joined: {}", targets.len());
    if targets.is_empty() {
        eprintln!("no joined target — the harvest stays unwritten (0 honored)");
        return;
    }
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(workdir.join(JWST_LEDGER));
    let done = ledger_done(&workdir);
    let start = Instant::now();
    let deadline = budget.map(|m| start + Duration::from_secs(m.saturating_mul(60)));
    let mut harvested = 0usize;
    let mut resumed = 0usize;
    let mut named_skips = 0usize;
    for t in &targets {
        if let Some(d) = deadline {
            if Instant::now() >= d {
                eprintln!("budget reached — partial harvest; resume continues from the ledger");
                break;
            }
        }
        if done.contains(&t.unique_id) {
            resumed += 1;
            continue;
        }
        if t.unique_id.len() > JWST_HOST_BYTES
            || t.unique_id.len() > JWST_OBSID_BYTES
            || t.unique_id.is_empty()
        {
            eprintln!(
                "{}: name exceeds the bin buffer — the record is skipped",
                t.unique_id
            );
            named_skips += 1;
            continue;
        }
        let bins = line_bins(&bytes, line, t.line_row);
        if bins.is_empty() {
            named_skips += 1;
            continue;
        }
        let spec = JwstSpectrum {
            ra_deg: t.ra_deg,
            dec_deg: t.dec_deg,
            plx_mas: 0.0,
            epoch_tdb: t.epoch_tdb,
            host: t.unique_id.clone(),
            obs_id: t.unique_id.clone(),
            redshift: t.redshift,
            z_kind: t.z_kind,
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
                "{}: sidecar/ledger write void — the record is lost (named)",
                t.unique_id
            );
            named_skips += 1;
            continue;
        }
        harvested += 1;
    }
    eprintln!(
        "\nharvest: {} new spectra, {} resumed from the ledger, {} named skips",
        harvested, resumed, named_skips
    );
    let Some(bin) = finalize_workdir(&workdir) else {
        eprintln!("finalize void — no sidecars, the bin stays unwritten (0 honored)");
        return;
    };
    match std::fs::write(&out_path, &bin) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("write {out_path}: {e}");
            return;
        }
    }
    match parse_jwst_bin(&bin) {
        Some(parsed) => {
            eprintln!(
                "{out_path}: {} records, {} B — roundtrip parses",
                parsed.len(),
                bin.len()
            );
        }
        None => {
            eprintln!("{out_path}: roundtrip parse void — the bin stays unverified");
            return;
        }
    }
    if ci_mode && !upload_release(JADES_ORIGIN, &out_path) {
        eprintln!("upload: {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}
