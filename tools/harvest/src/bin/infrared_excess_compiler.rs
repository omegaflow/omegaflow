use omegaflow::archivar::ir::{write_bin, IrSource, IR_EXCESS_THRESHOLD_MAG};
use omegaflow::cdn::upload_asset;
use std::process::Command;

const TAP_ROOT: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const IRSA_TAP: &str = "https://irsa.ipac.caltech.edu/TAP/sync";
const OUT_DEFAULT: &str = "tmp/ir.bin";
const W1_MAX: f64 = 9.0;
const W1W2_MAX: f64 = 0.2;
const SNR_MIN: f64 = 10.0;
const MAGIC_IRAS: [u8; 4] = *b"IRAS";
const MAGIC_MSX: [u8; 4] = *b"IRMS";
const MAGIC_AKARI: [u8; 4] = *b"IRAK";
const FAR_IR_RECORD_BYTES: usize = 48;

fn tap_query_csv(adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("300")
        .arg("--compressed")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(TAP_ROOT)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap_query http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn harvest(limit: usize) -> Vec<IrSource> {
    let adql = format!(
        "SELECT TOP {limit} RAJ2000,DEJ2000,W1mag,W2mag,W3mag,W4mag \
         FROM \"II/328/allwise\" \
         WHERE W1mag > 0 AND W1mag < {W1_MAX} \
         AND abs(W1mag-W2mag) < {W1W2_MAX} \
         AND W3mag IS NOT NULL AND W4mag IS NOT NULL \
         AND snr3 > {SNR_MIN} AND snr4 > {SNR_MIN}"
    );
    let Some(body) = tap_query_csv(&adql) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (i, line) in body.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 6 {
            continue;
        }
        let parse = |s: &str| -> Option<f64> {
            let v: f64 = s.trim().parse().ok()?;
            if v.is_finite() {
                Some(v)
            } else {
                None
            }
        };
        let (Some(ra), Some(dec), Some(w1), Some(w2), Some(w3), Some(w4)) = (
            parse(cols[0]),
            parse(cols[1]),
            parse(cols[2]),
            parse(cols[3]),
            parse(cols[4]),
            parse(cols[5]),
        ) else {
            continue;
        };
        if w1 <= 0.0 || w1 >= W1_MAX || (w1 - w2).abs() >= W1W2_MAX {
            continue;
        }
        if w3 <= 0.0 || w4 <= 0.0 {
            continue;
        }
        out.push(IrSource {
            ra_deg: ra,
            dec_deg: dec,
            plx_mas: 0.0,
            w3mag: w3,
            w4mag: w4,
            excess: w3 - w4,
        });
    }
    out
}

struct FarIrRec {
    ra_deg: f64,
    dec_deg: f64,
    plx_mas: f64,
    flux_short: f64,
    flux_long: f64,
    ratio: f64,
}

fn write_far_bin(magic: [u8; 4], sources: &[FarIrRec]) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(8 + sources.len() * FAR_IR_RECORD_BYTES);
    buf.extend_from_slice(&magic);
    buf.extend_from_slice(&(sources.len() as u32).to_le_bytes());
    for s in sources {
        if !s.ra_deg.is_finite()
            || !s.dec_deg.is_finite()
            || !s.plx_mas.is_finite()
            || !s.flux_short.is_finite()
            || !s.flux_long.is_finite()
            || !s.ratio.is_finite()
        {
            return None;
        }
        buf.extend_from_slice(&s.ra_deg.to_le_bytes());
        buf.extend_from_slice(&s.dec_deg.to_le_bytes());
        buf.extend_from_slice(&s.plx_mas.to_le_bytes());
        buf.extend_from_slice(&s.flux_short.to_le_bytes());
        buf.extend_from_slice(&s.flux_long.to_le_bytes());
        buf.extend_from_slice(&s.ratio.to_le_bytes());
    }
    Some(buf)
}

fn irsa_tap_csv(adql: &str, limit: usize) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("300")
        .arg("--compressed")
        .arg("-A")
        .arg("omegaflow-nadel-v-lsst-scan/1.0")
        .arg("-G")
        .arg(IRSA_TAP)
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("MAXREC={limit}"))
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "irsa tap http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn parse_far_rows(body: &str) -> Vec<FarIrRec> {
    let mut out = Vec::new();
    for (i, line) in body.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 5 {
            continue;
        }
        let parse = |s: &str| -> Option<f64> {
            let v: f64 = s.trim().parse().ok()?;
            if v.is_finite() {
                Some(v)
            } else {
                None
            }
        };
        let (Some(ra), Some(dec), Some(short), Some(long)) = (
            parse(cols[0]),
            parse(cols[1]),
            parse(cols[2]),
            parse(cols[3]),
        ) else {
            continue;
        };
        if short <= 0.0 || long <= 0.0 {
            continue;
        }
        let ratio = long / short;
        if !ratio.is_finite() || ratio <= 0.0 {
            continue;
        }
        out.push(FarIrRec {
            ra_deg: ra,
            dec_deg: dec,
            plx_mas: 0.0,
            flux_short: short,
            flux_long: long,
            ratio,
        });
    }
    out
}

fn harvest_iras(limit: usize) -> Vec<FarIrRec> {
    let adql = format!(
        "SELECT TOP {limit} ra, dec, fnu_12, fnu_25 FROM irasfsc WHERE fqual_12 >= 2 AND fqual_25 >= 2 AND fnu_12 > 0 AND fnu_25 > 0"
    );
    let Some(body) = irsa_tap_csv(&adql, limit) else {
        return Vec::new();
    };
    parse_far_rows(&body)
}

fn harvest_msx(limit: usize) -> Vec<FarIrRec> {
    let adql = format!(
        "SELECT TOP {limit} ra, dec, c, e FROM msxc6 WHERE q_c >= 1 AND q_e >= 1 AND c > 0 AND e > 0"
    );
    let Some(body) = irsa_tap_csv(&adql, limit) else {
        return Vec::new();
    };
    parse_far_rows(&body)
}

fn harvest_akari(limit: usize) -> Vec<FarIrRec> {
    let adql = format!(
        "SELECT TOP {limit} ra, dec, flux09, flux18 FROM akari_irc WHERE flux09 IS NOT NULL AND flux18 IS NOT NULL AND flux09 > 0 AND flux18 > 0"
    );
    let Some(body) = irsa_tap_csv(&adql, limit) else {
        return Vec::new();
    };
    parse_far_rows(&body)
}

fn far_ir_run(limit: usize, ci: bool) -> Result<(), String> {
    let catalogs = [
        (MAGIC_IRAS, "tmp/ir_iras.bin", harvest_iras(limit)),
        (MAGIC_MSX, "tmp/ir_msx.bin", harvest_msx(limit)),
        (MAGIC_AKARI, "tmp/ir_akari.bin", harvest_akari(limit)),
    ];
    for (magic, path, sources) in catalogs {
        if sources.is_empty() {
            return Err(format!(
                "{path}: no far-IR sources harvested — refused to write an empty bin"
            ));
        }
        let bytes =
            write_far_bin(magic, &sources).ok_or(format!("{path}: non-finite value refused"))?;
        std::fs::write(path, &bytes).map_err(|e| format!("{path}: {e}"))?;
        println!(
            "{path}: {} sources (ratio long/short), {} B",
            sources.len(),
            bytes.len()
        );
        if ci {
            if !upload_asset(path) {
                return Err(format!("{path}: CDN upload returned void"));
            }
            println!("{path}: uploaded to the CDN");
        }
    }
    Ok(())
}

fn run(out_path: &str, limit: usize, ci: bool) -> Result<(), String> {
    let sources = harvest(limit);
    if sources.is_empty() {
        return Err("no IR sources harvested — refused to write an empty bin".into());
    }
    let n_excess = sources.iter().filter(|s| s.is_excess()).count();
    println!(
        "ir.bin: {} sources, {} excess (< {} mag), {} B",
        sources.len(),
        n_excess,
        IR_EXCESS_THRESHOLD_MAG,
        sources.len() * 8
    );
    let bytes = write_bin(&sources).ok_or("write_bin: non-finite value refused")?;
    std::fs::write(out_path, &bytes).map_err(|e| format!("{out_path}: {e}"))?;
    println!("ir.bin: written to {out_path}");

    if ci {
        if !upload_asset(out_path) {
            return Err(format!("{out_path}: CDN upload returned void"));
        }
        println!("ir.bin: uploaded to the CDN");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out = OUT_DEFAULT.to_string();
    let mut limit = 30000usize;
    let mut ci = false;
    let mut far_ir = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(OUT_DEFAULT.to_string());
            }
            "--limit" => {
                i += 1;
                limit = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(30000);
            }
            "--ci-mode" => ci = true,
            "--far-ir" => far_ir = true,
            other => {
                eprintln!("infrared_excess_compiler: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if let Err(msg) = run(&out, limit, ci) {
        eprintln!("infrared_excess_compiler: {msg}");
        std::process::exit(1);
    }
    if far_ir {
        if let Err(msg) = far_ir_run(limit, ci) {
            eprintln!("infrared_excess_compiler: {msg}");
            std::process::exit(1);
        }
    }
}
