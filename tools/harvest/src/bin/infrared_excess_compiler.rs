use omegaflow::archivar::ir::{IR_EXCESS_THRESHOLD_MAG, IrSource, write_bin};
use omegaflow::cdn::upload_asset;
use std::process::Command;

const TAP_ROOT: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const OUT_DEFAULT: &str = "/tmp/opencode/ir.bin";
const W1_MAX: f64 = 9.0;
const W1W2_MAX: f64 = 0.2;
const SNR_MIN: f64 = 10.0;

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
            if v.is_finite() { Some(v) } else { None }
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
}
