use omegaflow::archivar::bsp_reader::spk::SpkFile;

fn arg_values(args: &[String], key: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == key {
            out.push(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

fn run(path: &str) -> bool {
    let Some(spk) = SpkFile::open(path).ok() else {
        eprintln!("{path}: unreadable as SPK");
        return false;
    };
    let segs = spk.segments();
    let mut by_type: std::collections::BTreeMap<i32, usize> = std::collections::BTreeMap::new();
    for s in segs {
        *by_type.entry(s.data_type).or_default() += 1;
    }
    let census: Vec<String> = by_type.iter().map(|(t, n)| format!("T{t}x{n}")).collect();
    println!("{path}: {} segments [{}]", segs.len(), census.join(" "));

    let mut checked = 0usize;
    let mut worst: f64 = 0.0;
    let mut worst_at = String::new();
    let mut per_type: std::collections::BTreeMap<i32, (usize, f64, String)> =
        std::collections::BTreeMap::new();
    for s in segs {
        if s.data_type != 2 && s.data_type != 3 {
            continue;
        }
        let span = s.end_et - s.start_et;
        if !span.is_finite() || span <= 0.0 {
            println!("  T{} {} -> {}: empty span", s.data_type, s.target, s.center);
            continue;
        }
        for k in 0..5 {
            let et = s.start_et + span * (k as f64 + 0.5) / 5.0;
            let h = 0.1;
            let Ok(left) = spk.state(s.target, s.center, et - h) else {
                continue;
            };
            let Ok(right) = spk.state(s.target, s.center, et + h) else {
                continue;
            };
            let Ok(mid) = spk.state(s.target, s.center, et) else {
                continue;
            };
            if !mid.iter().all(|v| v.is_finite()) {
                println!("  T{} {} -> {} at {et:.1}: non-finite state", s.data_type, s.target, s.center);
                continue;
            }
            let mut num = [0.0f64; 3];
            for a in 0..3 {
                num[a] = (right[a] - left[a]) / (2.0 * h);
            }
            let scale = mid[3..6].iter().map(|v| v.abs()).fold(0.0f64, f64::max).max(1.0);
            let mut d = 0.0f64;
            for a in 0..3 {
                d = d.max((num[a] - mid[3 + a]).abs());
            }
            let rel = d / scale;
            checked += 1;
            if rel > worst {
                worst = rel;
                worst_at = format!(
                    "T{} target {} center {} et {et:.1}",
                    s.data_type, s.target, s.center
                );
            }
            let entry = per_type
                .entry(s.data_type)
                .or_insert((0, 0.0, String::new()));
            entry.0 += 1;
            if rel > entry.1 {
                entry.1 = rel;
                entry.2 = format!("target {} center {} et {et:.1}", s.target, s.center);
            }
        }
    }
    if checked == 0 {
        println!("  no Type 2/3 segment to check");
        return true;
    }
    println!(
        "  Type 2/3 checks {checked}, worst |v_fd - v| / |v| = {worst:.3e} ({worst_at})"
    );
    for (t, (n, w, at)) in &per_type {
        println!("  T{t}: {n} checks, worst {w:.3e} ({at})");
    }
    worst < 1e-3
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let kernels = arg_values(&args, "--kernel");
    if kernels.is_empty() {
        eprintln!("usage: spk_structure_probe --kernel <file.bsp> [--kernel <file.bsp> ...]");
        std::process::exit(2);
    }
    let mut ok = true;
    for k in &kernels {
        ok &= run(k);
    }
    std::process::exit(if ok { 0 } else { 1 });
}
