use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use omegaflow_measure::idl_save;

const DEFAULT_IN: &str = "zustand/river/themis/kapu_20240909_skymap.sav";
const DEG: f64 = 180.0 / std::f64::consts::PI;
const DEG2RAD: f64 = std::f64::consts::PI / 180.0;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    let mut i = 1;
    while i < args.len() {
        if args[i] == key {
            return args.get(i + 1).cloned();
        }
        i += 1;
    }
    None
}

fn solve3(a: [[f64; 3]; 3], rhs: [f64; 3]) -> Option<[f64; 3]> {
    let mut m = [[0.0f64; 4]; 3];
    for i in 0..3 {
        for j in 0..3 {
            m[i][j] = a[i][j];
        }
        m[i][3] = rhs[i];
    }
    for col in 0..3 {
        let mut piv = col;
        let mut best = m[col][col].abs();
        for row in (col + 1)..3 {
            let v = m[row][col].abs();
            if v > best {
                best = v;
                piv = row;
            }
        }
        if best <= 1e-300 {
            return None;
        }
        if piv != col {
            m.swap(piv, col);
        }
        let d = m[col][col];
        for j in col..4 {
            m[col][j] /= d;
        }
        for row in 0..3 {
            if row != col {
                let f = m[row][col];
                for j in col..4 {
                    m[row][j] -= f * m[col][j];
                }
            }
        }
    }
    Some([m[0][3], m[1][3], m[2][3]])
}

fn rms(pairs: &[(f64, f64)]) -> f64 {
    let mut s = 0.0;
    for &(p, t) in pairs {
        let d = p - t;
        s += d * d;
    }
    (s / pairs.len() as f64).sqrt()
}

fn stereo_sse(samples: &[(f64, f64)], f: f64) -> f64 {
    let mut s = 0.0;
    for &(r, t) in samples {
        let p = 2.0 * (r / (2.0 * f)).atan();
        let d = p - t;
        s += d * d;
    }
    s
}

fn fit_stereo(samples: &[(f64, f64)]) -> f64 {
    let mut f0 = 0.0;
    for &(r, t) in samples {
        if t > 1e-9 {
            f0 += r / t;
        }
    }
    f0 /= samples.len() as f64;
    let mut a = (f0 / 8.0).max(1e-9);
    let mut b = (f0 * 8.0).max(a * 2.0);
    let gr = 0.618_033_988_749_894_9;
    let mut c = b - gr * (b - a);
    let mut d = a + gr * (b - a);
    let mut fc = stereo_sse(samples, c);
    let mut fd = stereo_sse(samples, d);
    for _ in 0..200 {
        if fc < fd {
            b = d;
            d = c;
            fd = fc;
            c = b - gr * (b - a);
            fc = stereo_sse(samples, c);
        } else {
            a = c;
            c = d;
            fc = fd;
            d = a + gr * (b - a);
            fd = stereo_sse(samples, d);
        }
    }
    (a + b) / 2.0
}

fn contains_value(values: &[f64], p: f64) -> bool {
    if p.abs() <= 1e-2 {
        return false;
    }
    let tol = 1e-3 * p.abs();
    values
        .iter()
        .any(|&v| v.abs() > 1e-2 && (v - p).abs() <= tol)
}

fn run(input: &str, out: Option<&str>) -> Result<(), String> {
    let skymap = idl_save::load_skymap(Path::new(input))?;
    let w = skymap.full_w;
    let h = skymap.full_h;
    let n = w * h;
    if skymap.full_elevation.len() != n {
        return Err(format!(
            "FULL_ELEVATION {} differs from {}x{} at {}",
            skymap.full_elevation.len(),
            w,
            h,
            input
        ));
    }
    if skymap.full_azimuth.len() != n {
        return Err(format!(
            "FULL_AZIMUTH {} differs from {}x{} at {}",
            skymap.full_azimuth.len(),
            w,
            h,
            input
        ));
    }
    let aspect = skymap.pixel_aspect_ratio;
    let mut count = 0usize;
    let mut yc = 0usize;
    let mut xc = 0usize;
    let mut elev_max = f64::NEG_INFINITY;
    for row in 0..h {
        for col in 0..w {
            let e = skymap.full_elevation[row * w + col] as f64;
            if !e.is_finite() {
                continue;
            }
            count += 1;
            if e > elev_max {
                elev_max = e;
                yc = row;
                xc = col;
            }
        }
    }
    if count == 0 {
        return Err(format!("no finite FULL_ELEVATION pixel at {}", input));
    }
    let mut samples: Vec<(f64, f64)> = Vec::with_capacity(count);
    let mut theta_min = f64::INFINITY;
    let mut theta_max = f64::NEG_INFINITY;
    for row in 0..h {
        for col in 0..w {
            let e = skymap.full_elevation[row * w + col] as f64;
            if !e.is_finite() {
                continue;
            }
            let dx = col as f64 - xc as f64;
            let dy = aspect * (row as f64 - yc as f64);
            let r = (dx * dx + dy * dy).sqrt();
            let theta = std::f64::consts::FRAC_PI_2 - e * DEG2RAD;
            if theta < theta_min {
                theta_min = theta;
            }
            if theta > theta_max {
                theta_max = theta;
            }
            samples.push((r, theta));
        }
    }

    println!("skymap_fit: {}", input);
    println!("grid: {} x {}", w, h);
    println!("pixel_aspect_ratio: {:.6}", aspect);
    print!("ccd_center:");
    for v in &skymap.ccd_center {
        print!(" {:.4}", v);
    }
    println!();
    println!(
        "center (zenith, max elevation): row {} col {} (elev {:.6} deg)",
        yc, xc, elev_max
    );
    println!("FULL_ELEVATION unit: degrees (measured)");
    println!("finite pixels: {}", count);
    println!(
        "theta min = {:.6} rad = {:.4} deg",
        theta_min,
        theta_min * DEG
    );
    println!(
        "theta max = {:.6} rad = {:.4} deg",
        theta_max,
        theta_max * DEG
    );
    println!("FOV = 2*theta_max = {:.4} deg", 2.0 * theta_max * DEG);

    let mut srr = 0.0;
    let mut srt = 0.0;
    for &(r, t) in &samples {
        srr += r * r;
        srt += r * t;
    }
    let a_lin = srt / srr;
    let mut pairs_lin: Vec<(f64, f64)> = Vec::with_capacity(count);
    for &(r, t) in &samples {
        pairs_lin.push((a_lin * r, t));
    }
    let rms_lin = rms(&pairs_lin);
    println!();
    println!("linear/equidistant: theta = a*r");
    println!("  a = {:.10} rad/px", a_lin);
    println!("  RMS = {:.8} rad = {:.5} deg", rms_lin, rms_lin * DEG);

    let f_stereo = fit_stereo(&samples);
    let mut pairs_stereo: Vec<(f64, f64)> = Vec::with_capacity(count);
    for &(r, t) in &samples {
        pairs_stereo.push((2.0 * (r / (2.0 * f_stereo)).atan(), t));
    }
    let rms_stereo = rms(&pairs_stereo);
    println!();
    println!("stereographic: theta = 2*atan(r/(2*f))");
    println!("  f = {:.10} px", f_stereo);
    println!(
        "  RMS = {:.8} rad = {:.5} deg",
        rms_stereo,
        rms_stereo * DEG
    );

    let mut norm = [[0.0f64; 3]; 3];
    let mut rhs = [0.0f64; 3];
    for &(r, t) in &samples {
        let p = [r, r * r, r * r * r];
        for i in 0..3 {
            for j in 0..3 {
                norm[i][j] += p[i] * p[j];
            }
            rhs[i] += p[i] * t;
        }
    }
    println!();
    println!("cubic: theta = a*r + b*r^2 + c*r^3");
    match solve3(norm, rhs) {
        Some(cubic) => {
            let mut pairs_cubic: Vec<(f64, f64)> = Vec::with_capacity(count);
            for &(r, t) in &samples {
                let p = cubic[0] * r + cubic[1] * r * r + cubic[2] * r * r * r;
                pairs_cubic.push((p, t));
            }
            let rms_cubic = rms(&pairs_cubic);
            println!(
                "  a = {:.10}  b = {:.10}  c = {:.12}",
                cubic[0], cubic[1], cubic[2]
            );
            println!("  RMS = {:.8} rad = {:.5} deg", rms_cubic, rms_cubic * DEG);
            report_projection(
                &skymap.optical_projection,
                &[a_lin],
                &[f_stereo],
                &[cubic[0], cubic[1], cubic[2]],
            );
        }
        None => {
            println!("  normal equations singular — no cubic coefficients measured");
            report_projection(&skymap.optical_projection, &[a_lin], &[f_stereo], &[]);
        }
    }

    if let Some(path) = out {
        let mut file = File::create(path).map_err(|e| format!("{} write void: {}", path, e))?;
        file.write_all(b"r,theta,elevation\n")
            .map_err(|e| format!("{} write void: {}", path, e))?;
        for row in 0..h {
            for col in 0..w {
                let e = skymap.full_elevation[row * w + col] as f64;
                if !e.is_finite() {
                    continue;
                }
                let dx = col as f64 - xc as f64;
                let dy = aspect * (row as f64 - yc as f64);
                let r = (dx * dx + dy * dy).sqrt();
                let theta = std::f64::consts::FRAC_PI_2 - e * DEG2RAD;
                writeln!(file, "{:.6},{:.8},{:.8}", r, theta, e)
                    .map_err(|e| format!("{} write void: {}", path, e))?;
            }
        }
        println!();
        println!("wrote {} r,theta,elevation rows -> {}", count, path);
    }
    Ok(())
}

fn report_projection(proj: &[f64], linear: &[f64], stereo: &[f64], cubic: &[f64]) {
    println!();
    println!("OPTICAL_PROJECTION ({} values):", proj.len());
    for (i, v) in proj.iter().enumerate() {
        println!("  [{}] = {:.10}", i, v);
    }
    let mut matched = false;
    let lin_hit = linear.iter().any(|&p| contains_value(proj, p));
    if lin_hit {
        println!("  linear: coefficient present in OPTICAL_PROJECTION");
        matched = true;
    }
    let stereo_hit = stereo.iter().any(|&p| contains_value(proj, p));
    if stereo_hit {
        println!("  stereographic: f present in OPTICAL_PROJECTION");
        matched = true;
    }
    let cubic_hit = cubic.iter().any(|&p| contains_value(proj, p));
    if cubic_hit {
        println!("  cubic: coefficient present in OPTICAL_PROJECTION");
        matched = true;
    }
    if !matched {
        println!("  kein Modell-Zuordnung gemessen");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match arg_value(&args, "--in") {
        Some(v) => v,
        None => DEFAULT_IN.to_string(),
    };
    let out = arg_value(&args, "--out");
    if let Err(detail) = run(&input, out.as_deref()) {
        eprintln!("skymap_fit: {}", detail);
        exit(2);
    }
}
