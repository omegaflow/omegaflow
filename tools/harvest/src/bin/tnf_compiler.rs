use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::odf::{scan_tnf_sfdus, tnf_dt0, TnfDt0, TnfSfdu};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-smallbodies.astro.umd.edu";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let bytes = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url, 3600) {
                Some(b) => b,
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--input <path.tnf> or --url <url> required");
                std::process::exit(1);
            }
        },
    };
    let frames = match scan_tnf_sfdus(&bytes) {
        Some(f) => f,
        None => {
            eprintln!("TNF SFDU scan void — the series stays unwritten");
            std::process::exit(1);
        }
    };
    let mut decoded: Vec<(TnfSfdu, TnfDt0)> = Vec::new();
    for f in &frames {
        if let Some(d) = tnf_dt0(f, &bytes[f.offset..]) {
            decoded.push((*f, d));
        }
    }
    if decoded.is_empty() {
        eprintln!("no DT0 carrier-phase records decoded — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let mut csv = String::from(
        "year,doy,sec,upl_rec_seq_num,ul_hi_phs_cycles,ul_lo_phs_cycles,ul_frac_phs_cycles,ramp_freq\n",
    );
    for (f, d) in &decoded {
        csv.push_str(&format!(
            "{},{},{:.3},{},{},{},{},{:.6e}\n",
            f.year,
            f.doy,
            f.sec,
            d.upl_rec_seq_num,
            d.ul_hi_phs_cycles,
            d.ul_lo_phs_cycles,
            d.ul_frac_phs_cycles,
            d.ramp_freq
        ));
    }
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &csv).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let rows = match std::fs::read_to_string(&out) {
        Ok(t) => t.lines().filter(|l| !l.starts_with("year,")).count(),
        Err(_) => 0,
    };
    if rows != decoded.len() {
        eprintln!(
            "{out}: roundtrip reads {rows} row(s) against {} decoded record(s) — the series stays unverified",
            decoded.len()
        );
        std::process::exit(1);
    }
    eprintln!(
        "{} frame(s), {} DT0 carrier-phase record(s) — roundtrip reads {rows}",
        frames.len(),
        decoded.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
