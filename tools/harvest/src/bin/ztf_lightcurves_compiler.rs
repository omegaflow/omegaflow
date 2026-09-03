use omegaflow::archivar::{STAR_RECORD_BYTES, embedded_lsk, parse_star_record, star_stride};
use omegaflow::cdn::upload_release;
use omegaflow::ztf::{
    ZtfCurve, band_freq_width, flux_from_mag, hjd_to_unix, parse_csv, write_ztf_bin,
};
use std::collections::HashMap;
use std::process::Command;

const IRSA_BASE: &str = "https://irsa.ipac.caltech.edu/cgi-bin/ZTF/nph_light_curves";
const CDN_RELEASE: &str = "irsa.ipac.caltech.edu";

fn curl_csv(ra: f64, dec: f64, radius: f64) -> Option<String> {
    let url = format!("{IRSA_BASE}?POS=CIRCLE+{ra}+{dec}+{radius}&FORMAT=csv");
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("--max-time")
        .arg("300")
        .arg("--retry")
        .arg("3")
        .arg("--retry-delay")
        .arg("2")
        .arg("-b")
        .arg("/tmp/opencode/ztf_cookies.txt")
        .arg("-c")
        .arg("/tmp/opencode/ztf_cookies.txt")
        .arg(&url)
        .output()
        .ok()?;
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

struct CurveBuilder {
    ra: f64,
    dec: f64,
    freq: f64,
    bin_width: f64,
    seen: std::collections::HashSet<u64>,
    samples: Vec<(f64, f32)>,
}

fn catalog_positions(path: &str, limit: usize) -> Vec<(f64, f64)> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return Vec::new();
        }
    };
    let Some(stride) = star_stride(&bytes) else {
        eprintln!(
            "{}: no {}-byte records — stars stay unqueried",
            path, STAR_RECORD_BYTES
        );
        return Vec::new();
    };
    let mut out = Vec::new();
    for chunk in bytes.chunks_exact(stride) {
        if out.len() >= limit {
            break;
        }
        if let Some(rec) = parse_star_record(chunk) {
            out.push((rec.ra_deg, rec.dec_deg));
        }
    }
    out
}

fn probe_csv(path: &str) {
    let body = match std::fs::read_to_string(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return;
        }
    };
    let rows = parse_csv(&body);
    let mut bands: HashMap<String, usize> = HashMap::new();
    let mut min_mag = f64::INFINITY;
    let mut max_mag = f64::NEG_INFINITY;
    for r in &rows {
        *bands.entry(r.filtercode.clone()).or_insert(0) += 1;
        if r.mag < min_mag {
            min_mag = r.mag;
        }
        if r.mag > max_mag {
            max_mag = r.mag;
        }
    }
    eprintln!("rows: {}", rows.len());
    eprintln!("bands: {:?}", bands);
    if !rows.is_empty() {
        eprintln!("mag range: {} .. {}", min_mag, max_mag);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut limit = usize::MAX;
    let mut radius = 0.02f64;
    let mut catalog: Option<String> = None;
    let mut cone: Option<(f64, f64)> = None;
    let mut probe: Option<String> = None;
    let mut sleep_ms: u64 = 250;
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
            "--radius" => {
                radius = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0.02);
                i += 1;
            }
            "--catalog" => {
                catalog = args.get(i + 1).cloned();
                i += 1;
            }
            "--cone" => {
                let ra = args.get(i + 1).and_then(|s| s.parse().ok());
                let dec = args.get(i + 2).and_then(|s| s.parse().ok());
                cone = match (ra, dec) {
                    (Some(ra), Some(dec)) => Some((ra, dec)),
                    _ => None,
                };
                i += 2;
            }
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            "--sleep-ms" => {
                sleep_ms = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(250);
                i += 1;
            }
            _ => {
                eprintln!(
                    "usage: ztf_lightcurves_compiler --out <ztf_lightcurves.bin> [--ci-mode] [--limit N] [--radius deg] [--catalog <dr3_stars.bin> | --cone <ra> <dec>] [--probe <file.csv>] [--sleep-ms N]"
                );
                return;
            }
        }
        i += 1;
    }
    if let Some(p) = probe {
        probe_csv(&p);
        return;
    }
    let Some(out_path) = out else {
        eprintln!("--out absent");
        return;
    };
    let targets = if let Some((ra, dec)) = cone {
        vec![(ra, dec)]
    } else if let Some(cat) = catalog {
        catalog_positions(&cat, limit)
    } else {
        eprintln!("--catalog or --cone absent");
        return;
    };
    if targets.is_empty() {
        eprintln!("targets empty — the asset stays unwritten (0 honored)");
        return;
    }
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the TDB epoch stays void (no fabricated epoch)");
        return;
    };
    let _ = std::fs::create_dir_all("/tmp/opencode");
    let mut curves: HashMap<(String, String), CurveBuilder> = HashMap::new();
    for (n, (ra, dec)) in targets.iter().enumerate() {
        let Some(body) = curl_csv(*ra, *dec, radius) else {
            continue;
        };
        for row in parse_csv(&body) {
            let Some((freq, bin_width)) = band_freq_width(&row.filtercode) else {
                continue;
            };
            let Some(tdb) = lsk.unix_to_tdb(hjd_to_unix(row.hjd)) else {
                continue;
            };
            let flux = flux_from_mag(row.mag) as f32;
            let key = (row.oid.clone(), row.filtercode.clone());
            let entry = curves.entry(key).or_insert_with(|| CurveBuilder {
                ra: row.ra,
                dec: row.dec,
                freq,
                bin_width,
                seen: std::collections::HashSet::new(),
                samples: Vec::new(),
            });
            let bits = row.hjd.to_bits();
            if entry.seen.insert(bits) {
                entry.samples.push((tdb, flux));
            }
        }
        eprintln!(
            "\r\x1b[K[{}] ra {} dec {}: {} curves",
            n,
            ra,
            dec,
            curves.len()
        );
        if sleep_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
        }
    }
    let mut out_curves: Vec<ZtfCurve> = Vec::with_capacity(curves.len());
    let mut total_samples = 0usize;
    for (_key, mut b) in curves {
        b.samples
            .sort_by(|a, c| a.0.partial_cmp(&c.0).unwrap_or(std::cmp::Ordering::Equal));
        total_samples += b.samples.len();
        out_curves.push(ZtfCurve {
            ra_deg: b.ra,
            dec_deg: b.dec,
            plx_mas: 0.0,
            freq: b.freq,
            bin_width: b.bin_width,
            samples: b.samples,
        });
    }
    out_curves.sort_by(|a, b| {
        a.ra_deg
            .partial_cmp(&b.ra_deg)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(
                a.dec_deg
                    .partial_cmp(&b.dec_deg)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
            .then(
                a.freq
                    .partial_cmp(&b.freq)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
    });
    eprintln!(
        "\nztf curves: {} ({} samples), all on the celestial sphere — {} awaiting z (plx 0.0, TNS crossmatch)",
        out_curves.len(),
        total_samples,
        out_curves.len()
    );
    let Some(bytes) = write_ztf_bin(&out_curves) else {
        eprintln!(
            "write {}: a curve is non-finite — the asset stays unwritten (0 honored)",
            out_path
        );
        return;
    };
    if std::fs::write(&out_path, &bytes).is_err() {
        eprintln!("write {}: void", out_path);
        return;
    }
    if ci_mode && !upload_release(CDN_RELEASE, &out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
