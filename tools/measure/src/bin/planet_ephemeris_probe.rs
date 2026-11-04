use omegaflow::archivar::bsp_reader::spk::SpkFile;

fn main() {
    let mut kernels: Vec<String> = Vec::new();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    let mut et = (2_461_041.5 - 2_451_545.0) * 86_400.0;
    while i < args.len() {
        match args[i].as_str() {
            "--kernel" => {
                if let Some(p) = args.get(i + 1) {
                    kernels.push(p.clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--et" => {
                if let Some(v) = args.get(i + 1).and_then(|v| v.parse::<f64>().ok()) {
                    et = v;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    if kernels.is_empty() {
        eprintln!(
            "usage: planet_ephemeris_probe --kernel <file.bsp> [--kernel ...] [--et <seconds>]"
        );
        return;
    }
    println!("# et {et}");
    for path in &kernels {
        match SpkFile::open(path) {
            Ok(spk) => {
                let segs = spk.segments();
                let types: Vec<String> = segs
                    .iter()
                    .map(|s| {
                        format!(
                            "T{}:{}/{}/{:.0}..{:.0}",
                            s.data_type, s.target, s.center, s.start_et, s.end_et
                        )
                    })
                    .collect();
                println!("# {path}: {} segments", segs.len());
                println!("# {}", types.join(" "));
                for target in 1..=9 {
                    match spk.state(target, 0, et) {
                        Ok(s) => println!(
                            "{path}\t{target}\t{:.6}\t{:.6}\t{:.6}\t{:.9}\t{:.9}\t{:.9}",
                            s[0], s[1], s[2], s[3], s[4], s[5]
                        ),
                        Err(e) => println!("{path}\t{target}\tabsent\t{e:?}"),
                    }
                }
            }
            Err(e) => println!("# {path}: {e:?}"),
        }
    }
}
