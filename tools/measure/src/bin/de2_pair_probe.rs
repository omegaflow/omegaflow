use std::env;
use std::process::exit;

use omegaflow::archivar::SECONDS_PER_DAY;

const WATS_CADENCE_MS: f64 = 2000.0;
const WATS_FILL: f32 = 9999.0;

fn read_lines(path: &str) -> Option<Vec<String>> {
    let text = std::fs::read_to_string(path).ok()?;
    Some(text.lines().map(|l| l.to_string()).collect())
}

fn absolute_ms(date: &str, ut_ms: &str) -> Option<f64> {
    let day: f64 = date.parse().ok()?;
    let ms: f64 = ut_ms.parse().ok()?;
    Some(day * SECONDS_PER_DAY * 1000.0 + ms)
}

fn wats_series(paths: &[String]) -> Vec<(f64, f32)> {
    let mut out = Vec::new();
    for path in paths {
        let Some(lines) = read_lines(path) else {
            continue;
        };
        let mut line = 0;
        while line + 1 < lines.len() {
            let head: Vec<&str> = lines[line].split_whitespace().collect();
            let is_date = head
                .first()
                .is_some_and(|t| t.len() == 5 && t.bytes().all(|b| b.is_ascii_digit()));
            if !is_date {
                line += 1;
                continue;
            }
            let data: Vec<&str> = lines[line + 1].split_whitespace().collect();
            line += 2;
            if head.len() < 3 || data.len() < 2 {
                continue;
            }
            if !matches!(head[2].chars().next(), Some('3') | Some('4')) {
                continue;
            }
            let (Some(time), Ok(v_geo_cor)) =
                (absolute_ms(head[0], head[1]), data[1].parse::<f32>())
            else {
                continue;
            };
            if v_geo_cor.abs() >= WATS_FILL {
                continue;
            }
            out.push((time, v_geo_cor));
        }
    }
    out.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn idm_velocity(token: &str) -> Option<(f32, u32, u32)> {
    let (int_part, frac) = token.split_once('.')?;
    let mut digits = frac.chars();
    let quality = digits.next()?.to_digit(10)?;
    let direction = digits.next()?.to_digit(10)?;
    let value = int_part.parse::<f32>().ok()?;
    Some((value, quality, direction))
}

fn idm_series(paths: &[String]) -> Vec<(f64, f32)> {
    let mut out = Vec::new();
    for path in paths {
        let Some(lines) = read_lines(path) else {
            continue;
        };
        let mut line = 0;
        while line < lines.len() {
            let head: Vec<&str> = lines[line].split_whitespace().collect();
            if head.len() < 9 {
                line += 1;
                continue;
            }
            let Some(count) = head[8].parse::<usize>().ok() else {
                line += 1;
                continue;
            };
            let date = head[0].to_string();
            let mut read = 0usize;
            let mut cursor = line + 1;
            while read < count && cursor < lines.len() {
                let toks: Vec<&str> = lines[cursor].split_whitespace().collect();
                let mut k = 0;
                while k + 1 < toks.len() && read < count {
                    let (time_token, vel_token) = (toks[k], toks[k + 1]);
                    k += 2;
                    read += 1;
                    let (Some((value, quality, direction)), Some(time)) =
                        (idm_velocity(vel_token), absolute_ms(&date, time_token))
                    else {
                        continue;
                    };
                    if direction % 2 == 0 && quality <= 1 {
                        out.push((time, value));
                    }
                }
                cursor += 1;
            }
            line = cursor;
        }
    }
    out.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let collect = |flag: &str| -> Vec<String> {
        let mut out = Vec::new();
        let mut i = 0;
        while i < args.len() {
            if args[i] == flag {
                let mut j = i + 1;
                while j < args.len() && !args[j].starts_with("--") {
                    out.push(args[j].clone());
                    j += 1;
                }
                i = j;
            } else {
                i += 1;
            }
        }
        out
    };
    let find = |flag: &str| -> Option<String> {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let wats = collect("--wats");
    let idm = collect("--idm");
    if wats.is_empty() || idm.is_empty() {
        eprintln!("usage: de2_pair_probe --wats <file>... --idm <file>... [--out-a <path>] [--out-b <path>]");
        exit(2);
    }

    let w = wats_series(&wats);
    let v = idm_series(&idm);
    if w.is_empty() || v.is_empty() {
        println!(
            "de2_pair_probe: wats {} samples | idm {} samples -> no pairing (no fabrication)",
            w.len(),
            v.len()
        );
        return;
    }
    let tolerance = WATS_CADENCE_MS / 2.0;
    let mut pairs: Vec<(f32, f32)> = Vec::new();
    let mut cursor = 0usize;
    for (time, nve) in &w {
        while cursor + 1 < v.len()
            && (v[cursor + 1].0 - time).abs() <= (v[cursor].0 - time).abs()
        {
            cursor += 1;
        }
        if let Some((ti, ive)) = v.get(cursor) {
            if (ti - time).abs() <= tolerance {
                pairs.push((*nve, *ive));
            }
        }
    }
    let n = pairs.len();
    println!(
        "de2_pair_probe: wats {} | idm {} | paired {} | tolerance {} ms (wats cadence / 2)",
        w.len(),
        v.len(),
        n,
        tolerance
    );
    println!(
        "frames: wats v_geo_cor = geographic east (corotating); idm = s/c z-axis (direction flag even), quality 0/1"
    );
    if n >= 2 {
        let nf = n as f64;
        let mean_a = pairs.iter().map(|p| p.0 as f64).sum::<f64>() / nf;
        let mean_b = pairs.iter().map(|p| p.1 as f64).sum::<f64>() / nf;
        let (mut sab, mut sa, mut sb) = (0.0f64, 0.0f64, 0.0f64);
        for (a, b) in &pairs {
            let da = *a as f64 - mean_a;
            let db = *b as f64 - mean_b;
            sab += da * db;
            sa += da * da;
            sb += db * db;
        }
        if sa > 0.0 && sb > 0.0 {
            println!(
                "means: nve {:.1} | ive {:.1} | r = {:.4}",
                mean_a,
                mean_b,
                sab / (sa.sqrt() * sb.sqrt())
            );
        }
    }
    if let Some(path) = find("--out-a") {
        let body: String = pairs.iter().map(|p| format!("{}\n", p.0)).collect();
        match std::fs::write(&path, body) {
            Ok(()) => println!("wrote nve -> {}", path),
            Err(e) => eprintln!("{}: {}", path, e),
        }
    }
    if let Some(path) = find("--out-b") {
        let body: String = pairs.iter().map(|p| format!("{}\n", p.1)).collect();
        match std::fs::write(&path, body) {
            Ok(()) => println!("wrote ive -> {}", path),
            Err(e) => eprintln!("{}: {}", path, e),
        }
    }
}
