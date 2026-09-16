use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::odf::{TnfDt0, TnfSfdu, scan_tnf_sfdus, tnf_dt0};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const NETLOC: &str = "pds-smallbodies.astro.umd.edu";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn epoch_iso(frame: &TnfSfdu) -> Option<String> {
    let day0 = days_from_civil(frame.year as i64, 1, 1)?;
    let ms = (frame.sec * 1000.0).round() as i64;
    let days = day0 + frame.doy as i64 - 1 + ms.div_euclid(86_400_000);
    let rem = ms.rem_euclid(86_400_000);
    let (year, month, day) = civil_from_days(days)?;
    let hour = rem / 3_600_000;
    let minute = (rem / 60_000) % 60;
    let second = (rem / 1000) % 60;
    let milli = rem % 1000;
    Some(format!(
        "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{milli:03}Z"
    ))
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
        "epoch,year,doy,sec,upl_rec_seq_num,ul_hi_phs_cycles,ul_lo_phs_cycles,ul_frac_phs_cycles,ramp_freq,ul_phase_cycles\n",
    );
    for (f, d) in &decoded {
        let Some(epoch) = epoch_iso(f) else {
            eprintln!(
                "year {} doy {} does not resolve to a civil date — the series stays unwritten",
                f.year, f.doy
            );
            std::process::exit(1);
        };
        let phase_cycles = d.ul_hi_phs_cycles as f64 * 4_294_967_296.0
            + d.ul_lo_phs_cycles as f64
            + d.ul_frac_phs_cycles as f64 / 4_294_967_296.0;
        csv.push_str(&format!(
            "{epoch},{},{},{:.3},{},{},{},{},{:.6e},{:.6e}\n",
            f.year,
            f.doy,
            f.sec,
            d.upl_rec_seq_num,
            d.ul_hi_phs_cycles,
            d.ul_lo_phs_cycles,
            d.ul_frac_phs_cycles,
            d.ramp_freq,
            phase_cycles
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
        Ok(t) => t.lines().filter(|l| !l.starts_with("epoch,")).count(),
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
