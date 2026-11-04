use omegaflow::archivar::hrv::{TONE_ABSENT, TONE_CALM, TONE_STRESSED};
use omegaflow::sha256::sha256_hex;

fn quantile(sorted: &[f32], q: f64) -> f32 {
    let n = sorted.len();
    if n == 0 {
        return f32::NAN;
    }
    if n == 1 {
        return sorted[0];
    }
    let pos = q * (n - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = (pos.ceil() as usize).min(n - 1);
    let frac = (pos - lo as f64) as f32;
    sorted[lo] + (sorted[hi] - sorted[lo]) * frac
}

fn print_quantity(name: &str, values: &[f32]) {
    let mut s = values.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    println!(
        " {:<20} | {:>6} | {:>8.4} | {:>8.4} {:>8.4} {:>8.4} | {:>8.4}",
        name,
        s.len(),
        s[0],
        quantile(&s, 0.10),
        quantile(&s, 0.50),
        quantile(&s, 0.90),
        s[s.len() - 1],
    );
}

fn mean_of(values: &[f32]) -> Option<f32> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum::<f32>() / values.len() as f32)
}

fn mean_label(values: &[f32]) -> String {
    match mean_of(values) {
        Some(m) => format!("{m:.4}"),
        None => "absent".to_string(),
    }
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            println!("live dump absent");
            std::process::exit(2);
        }
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(_) => {
            println!("live dump absent");
            std::process::exit(2);
        }
    };
    let text = String::from_utf8_lossy(&bytes);

    let mut tone_codes: Vec<u8> = Vec::new();
    let mut scales: Vec<f32> = Vec::new();
    let mut apertures: Vec<f32> = Vec::new();
    let mut perms: Vec<f32> = Vec::new();
    let mut scale_calm: Vec<f32> = Vec::new();
    let mut scale_stressed: Vec<f32> = Vec::new();
    let mut aperture_calm: Vec<f32> = Vec::new();
    let mut aperture_stressed: Vec<f32> = Vec::new();
    let mut perm_calm: Vec<f32> = Vec::new();
    let mut perm_stressed: Vec<f32> = Vec::new();

    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = t.split(',').collect();
        if cols.len() < 10 {
            continue;
        }
        if cols[0].trim().parse::<u64>().is_err() {
            continue;
        }
        let (Ok(tone_code), Ok(tone_scale), Ok(aperture), Ok(fp)) = (
            cols[6].trim().parse::<u8>(),
            cols[7].trim().parse::<f32>(),
            cols[8].trim().parse::<f32>(),
            cols[9].trim().parse::<f32>(),
        ) else {
            continue;
        };
        if !tone_scale.is_finite() || !aperture.is_finite() || !fp.is_finite() {
            continue;
        }
        tone_codes.push(tone_code);
        scales.push(tone_scale);
        apertures.push(aperture);
        perms.push(fp);
        if tone_code == TONE_CALM {
            scale_calm.push(tone_scale);
            aperture_calm.push(aperture);
            perm_calm.push(fp);
        } else if tone_code == TONE_STRESSED {
            scale_stressed.push(tone_scale);
            aperture_stressed.push(aperture);
            perm_stressed.push(fp);
        }
    }

    if tone_codes.is_empty() {
        println!("live dump absent");
        std::process::exit(2);
    }

    println!(
        "=== perm_tone_probe --live: the HRV/pulse -> tone -> aperture -> radiation chain, measured ==="
    );
    println!("sha256={} n={}", sha256_hex(&bytes), tone_codes.len());
    println!();

    let n_absent = tone_codes.iter().filter(|&&c| c == TONE_ABSENT).count();
    let n_calm = tone_codes.iter().filter(|&&c| c == TONE_CALM).count();
    let n_stressed = tone_codes.iter().filter(|&&c| c == TONE_STRESSED).count();
    let n_other = tone_codes.len() - n_absent - n_calm - n_stressed;
    println!(
        "tone_code: absent={n_absent} calm={n_calm} stressed={n_stressed} other={n_other} (0=absent 1=calm 2=stressed)"
    );
    println!();

    println!(
        " {:<20} | {:>6} | {:>8} | {:>8} {:>8} {:>8} | {:>8}",
        "quantity", "n", "min", "q10", "q50", "q90", "max"
    );
    print_quantity("tone_scale", &scales);
    print_quantity("aperture", &apertures);
    print_quantity("field_permeability", &perms);
    println!();

    println!("conditioned on tone_code:");
    println!(
        " {:<20} | {:>8} | {:>8} | {:>8} | {:>8}",
        "quantity", "calm", "stressed", "calm n", "stressed n"
    );
    println!(
        " {:<20} | {:>8} | {:>8} | {:>8} | {:>8}",
        "tone_scale",
        mean_label(&scale_calm),
        mean_label(&scale_stressed),
        scale_calm.len(),
        scale_stressed.len()
    );
    println!(
        " {:<20} | {:>8} | {:>8} | {:>8} | {:>8}",
        "aperture",
        mean_label(&aperture_calm),
        mean_label(&aperture_stressed),
        aperture_calm.len(),
        aperture_stressed.len()
    );
    println!(
        " {:<20} | {:>8} | {:>8} | {:>8} | {:>8}",
        "field_permeability",
        mean_label(&perm_calm),
        mean_label(&perm_stressed),
        perm_calm.len(),
        perm_stressed.len()
    );
}
