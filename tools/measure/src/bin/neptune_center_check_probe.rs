use std::collections::HashMap;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};

fn load(path: &str, name: &str) -> Option<HashMap<String, BodyEphemeris>> {
    let bytes = std::fs::read(path).ok()?;
    let eph = parse_ephemeris_binary(&bytes)?;
    let mut map = HashMap::new();
    map.insert(name.to_string(), eph);
    Some(map)
}

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let spk_path =
        arg_token(&args, "--spk").unwrap_or("data/naif.jpl.nasa.gov/nep097xl-899.bsp".to_string());

    let c_path = format!("{eph_dir}/ssd.jpl.nasa.gov/ephemeris_neptune_c.bin");
    let b_path = format!("{eph_dir}/ssd.jpl.nasa.gov/ephemeris_neptune.bin");
    let (Some(cm), Some(bm)) = (load(&c_path, "neptune_c"), load(&b_path, "neptune")) else {
        eprintln!("neptune-center-check: {c_path} or {b_path} reads void");
        return;
    };
    let spk = match SpkFile::open(&spk_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("neptune-center-check: {spk_path} opens void — {e:?}");
            return;
        }
    };

    let start_jd = 2451545.0;
    let stop_jd = 2451545.0 + 365.0;
    let step = 0.5;
    let mut wobble_sum = 0.0;
    let mut wobble_max = 0.0;
    let mut repro_sum = 0.0;
    let mut repro_max = 0.0;
    let mut repro_sq = 0.0;
    let mut n = 0usize;
    let mut jd = start_jd;
    while jd <= stop_jd {
        let tdb = (jd - 2451545.0) * 86400.0;
        let (Some(c), Some(b)) = (
            body_barycenter_position("neptune_c", tdb, &cm),
            body_barycenter_position("neptune", tdb, &bm),
        ) else {
            jd += step;
            continue;
        };
        let w = [c[0] - b[0], c[1] - b[1], c[2] - b[2]];
        let wm = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
        wobble_sum += wm;
        if wm > wobble_max {
            wobble_max = wm;
        }
        if let Ok(rel) = spk.state(899, 8, tdb) {
            let e = [
                c[0] - (b[0] + rel[0] * 1000.0),
                c[1] - (b[1] + rel[1] * 1000.0),
                c[2] - (b[2] + rel[2] * 1000.0),
            ];
            let em = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
            repro_sum += em;
            repro_sq += em * em;
            if em > repro_max {
                repro_max = em;
            }
        }
        n += 1;
        jd += step;
    }

    if n == 0 {
        eprintln!("neptune-center-check: no samples carried both lines");
        return;
    }
    let mean_w = wobble_sum / n as f64;
    let mean_r = repro_sum / n as f64;
    let rms_r = (repro_sq / n as f64).sqrt();
    println!(
        "neptune-center-check ({} samples over JD {:.2}..{:.2}):",
        n, start_jd, stop_jd
    );
    println!(
        "  wobble (center − barycenter): mean {:.0} m, max {:.0} m",
        mean_w, wobble_max
    );
    println!("  composition reproduction (composed − DE441+899−8): mean {:.2} m, RMS {:.2} m, max {:.2} m", mean_r, rms_r, repro_max);
}
