use omegaflow::archivar::bsp_reader::spk::SpkFile;

const AU_KM: f64 = 1.495978707e8;

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let spk_path = match arg_token(&args, "--spk") {
        Some(p) => p,
        None => {
            eprintln!("spk_type1_check: --spk <path> absent");
            std::process::exit(2);
        }
    };
    let target: i32 = match arg_token(&args, "--target").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => -32,
    };
    let center: i32 = match arg_token(&args, "--center").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => 10,
    };
    let epochs: Vec<f64> = match arg_token(&args, "--epoch") {
        Some(e) => match e.parse() {
            Ok(v) => vec![v],
            Err(_) => {
                eprintln!("spk_type1_check: --epoch {e} is no JD");
                std::process::exit(2);
            }
        },
        None => vec![2447761.5, 2451545.0, 2455197.5, 2458850.0, 2461041.5],
    };

    let spk = match SpkFile::open(&spk_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("spk_type1_check: {spk_path} opens void — {e:?}");
            return;
        }
    };

    let mut t1 = 0usize;
    for seg in spk.segments() {
        if seg.data_type == 1 {
            t1 += 1;
            eprintln!(
                "  type-1: target {} wrt {} frame {} ({})",
                seg.target, seg.center, seg.frame, seg.name
            );
        }
    }
    eprintln!(
        "spk_type1_check: {} segments, {} type-1",
        spk.segments().len(),
        t1
    );

    let day = 86400.0;
    for jd in epochs {
        let et = (jd - 2451545.0) * day;
        match spk.state(target, center, et) {
            Ok(s) => {
                let r = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt();
                let v = (s[3] * s[3] + s[4] * s[4] + s[5] * s[5]).sqrt();
                println!("  jd {jd:.1}: r {:.4} AU, v {:.4} km/s", r / AU_KM, v);
            }
            Err(e) => eprintln!("  jd {jd:.1}: state void — {e}"),
        }
    }
}
