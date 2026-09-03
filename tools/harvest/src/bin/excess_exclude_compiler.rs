use omegaflow::archivar::exclude::{ExcludeKind, ExcludeRow, NAME_LEN, write_bin};
use omegaflow::archivar::ir::parse_bin as parse_ir;
use omegaflow::cdn::upload_asset;
use std::process::Command;

const TAP_ROOT: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const NASA_TAP: &str = "https://exoplanetarchive.ipac.caltech.edu/TAP/sync";
const OUT_DEFAULT: &str = "/tmp/opencode/exclude.bin";
const CONE_RADIUS_DEG: f64 = 0.02;
const IR_EXCESS_THRESHOLD_MAG: f64 = -0.5;

fn tap_csv(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("120")
        .arg("--compressed")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!("tap {} http {}", root, out.status);
        None
    }
}

fn fixname(name: [u8; NAME_LEN]) -> [u8; NAME_LEN] {
    name
}

fn vsx_cone(ra: f64, dec: f64, r: f64) -> Vec<ExcludeRow> {
    let adql = format!(
        "SELECT RAJ2000,DEJ2000,Type,Name FROM \"B/vsx/vsx\" WHERE CONTAINS(POINT('ICRS',RAJ2000,DEJ2000),CIRCLE('ICRS',{},{},{}))=1",
        ra, dec, r
    );
    let Some(body) = tap_csv(TAP_ROOT, &adql) else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for (i, line) in body.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 4 {
            continue;
        }
        let (Ok(ra_), Ok(dec_)) = (cols[0].trim().parse::<f64>(), cols[1].trim().parse::<f64>())
        else {
            continue;
        };
        let mut nb = [0u8; NAME_LEN];
        let typ = cols[2].trim().trim_matches('"');
        let name = cols[3].trim().trim_matches('"');
        let label = if !name.is_empty() { name } else { typ };
        let bytes = label.as_bytes();
        let n = bytes.len().min(NAME_LEN);
        nb[..n].copy_from_slice(&bytes[..n]);
        rows.push(ExcludeRow {
            name: fixname(nb),
            ra_deg: ra_,
            dec_deg: dec_,
            kind: ExcludeKind::Variable,
        });
    }
    rows
}

fn gcvs_cone(ra: f64, dec: f64, r: f64) -> Vec<ExcludeRow> {
    let adql = format!(
        "SELECT RAJ2000,DEJ2000,VarType,GCVS FROM \"B/gcvs/gcvs_cat\" WHERE CONTAINS(POINT('ICRS',RAJ2000,DEJ2000),CIRCLE('ICRS',{},{},{}))=1",
        ra, dec, r
    );
    let Some(body) = tap_csv(TAP_ROOT, &adql) else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for (i, line) in body.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 4 {
            continue;
        }
        let (Ok(ra_), Ok(dec_)) = (cols[0].trim().parse::<f64>(), cols[1].trim().parse::<f64>())
        else {
            continue;
        };
        let mut nb = [0u8; NAME_LEN];
        let var = cols[2].trim().trim_matches('"');
        let gcvs = cols[3].trim().trim_matches('"');
        let label = if !gcvs.is_empty() { gcvs } else { var };
        let bytes = label.as_bytes();
        let n = bytes.len().min(NAME_LEN);
        nb[..n].copy_from_slice(&bytes[..n]);
        rows.push(ExcludeRow {
            name: nb,
            ra_deg: ra_,
            dec_deg: dec_,
            kind: ExcludeKind::Variable,
        });
    }
    rows
}

fn exoplanet_cone(ra: f64, dec: f64, r: f64) -> Vec<ExcludeRow> {
    let adql = format!(
        "SELECT ra,dec,hostname FROM ps WHERE CONTAINS(POINT('ICRS',ra,dec),CIRCLE('ICRS',{},{},{}))=1",
        ra, dec, r
    );
    let Some(body) = tap_csv(NASA_TAP, &adql) else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for (i, line) in body.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 3 {
            continue;
        }
        let (Ok(ra_), Ok(dec_)) = (cols[0].trim().parse::<f64>(), cols[1].trim().parse::<f64>())
        else {
            continue;
        };
        let mut nb = [0u8; NAME_LEN];
        let host = cols[2].trim().trim_matches('"');
        let bytes = host.as_bytes();
        let n = bytes.len().min(NAME_LEN);
        nb[..n].copy_from_slice(&bytes[..n]);
        rows.push(ExcludeRow {
            name: nb,
            ra_deg: ra_,
            dec_deg: dec_,
            kind: ExcludeKind::Exoplanet,
        });
    }
    rows
}

fn run(
    ir_path: &str,
    radius: f64,
    only_excess: bool,
    out_path: &str,
    ci: bool,
) -> Result<(), String> {
    let ib = std::fs::read(ir_path).map_err(|e| format!("ir {ir_path}: {e}"))?;
    let ir = parse_ir(&ib).ok_or("ir.bin: no IR1X contract")?;
    let mut rows: Vec<ExcludeRow> = Vec::new();
    let mut candidates = 0usize;
    for src in &ir {
        let is_excess = src.excess.is_finite() && src.excess < IR_EXCESS_THRESHOLD_MAG;
        if only_excess && !is_excess {
            continue;
        }
        if only_excess {
            candidates += 1;
        }
        let vsx = vsx_cone(src.ra_deg, src.dec_deg, radius);
        let gcvs = gcvs_cone(src.ra_deg, src.dec_deg, radius);
        let exo = exoplanet_cone(src.ra_deg, src.dec_deg, radius);
        eprintln!(
            "cand ra {:.4} dec {:.4} exzess {}: vsx {} gcvs {} exoplanet {}",
            src.ra_deg,
            src.dec_deg,
            is_excess,
            vsx.len(),
            gcvs.len(),
            exo.len()
        );
        for r in vsx
            .into_iter()
            .chain(gcvs.into_iter())
            .chain(exo.into_iter())
        {
            rows.push(r);
        }
    }
    if rows.is_empty() {
        return Err(
            "no exclusion rows harvested — the output would be an empty bin (refused)".into(),
        );
    }
    let bytes = write_bin(&rows).ok_or("write_bin: non-finite value refused")?;
    std::fs::write(out_path, &bytes).map_err(|e| format!("{out_path}: {e}"))?;
    println!(
        "exclude.bin: {} rows, {} B ({} candidates checked, radius {}°)",
        rows.len(),
        bytes.len(),
        candidates,
        radius
    );
    if ci {
        if !upload_asset(out_path) {
            return Err(format!("{out_path}: CDN upload returned void"));
        }
        println!("exclude.bin: uploaded to the CDN");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut ir = "/tmp/opencode/ir.bin".to_string();
    let mut out = OUT_DEFAULT.to_string();
    let mut radius = CONE_RADIUS_DEG;
    let mut ci = false;
    let mut only_excess = true;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--ir" => {
                i += 1;
                ir = args.get(i).cloned().unwrap_or(ir);
            }
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(out);
            }
            "--radius" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse::<f64>().ok()) {
                    radius = v;
                }
            }
            "--ci-mode" => ci = true,
            "--all" => only_excess = false,
            other => {
                eprintln!("excess_exclude_compiler: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if let Err(msg) = run(&ir, radius, only_excess, &out, ci) {
        eprintln!("excess_exclude_compiler: {msg}");
        std::process::exit(1);
    }
}
