// Der Temperaturleiter-Probe — die Alfvén-Frage auf der vollen Leiter.
// Liest eve_lines.bin (EVL1, 7 Linien), ordnet sie nach Formationstemperatur
// (LOGT), erkennt Flare-Ereignisse über die heißeste Linie (94 Å, Fe XVIII)
// und misst je benachbartem Paar (kühl, heiß) der Leiter die gerichtete TE
// über einen Lag-Sweep, ereignisgestapelt gegen die Phasen-Null:
//   D(lag) = TE(kühl→heiß, lag) − TE(heiß→kühl, lag)
// Ein konsistent positiver D bei lag ≈ 10 Zellen (≈ 100 s) bedeutet
// Energiefluss die Leiter HOCH (Alfvén-vereinbar); D ohne Lag und ohne
// Richtung ist die gemeinsame Nanoflare-Erwärmung. src/te.rs bleibt
// unberührt (öffentliche API transfer_entropy_lag/phase_randomized_surrogate).

use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};

const MAGIC: [u8; 4] = *b"EVL1";
const DT: f64 = 10.0;
const WINDOW: usize = 120;
const N_SURR: usize = 10;
const THRESH_FACTOR: f64 = 1.5;
const REFRACTORY: usize = 180;

const LINES: [(u32, &str, f64); 7] = [
    (11, "304A", 4.70),
    (1, "131A", 5.57),
    (3, "171A", 5.81),
    (6, "195A", 6.13),
    (8, "211A", 6.27),
    (10, "284A", 6.30),
    (0, "94A", 6.81),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_eve_lines(path: &str) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{} reads void", path);
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        eprintln!("{} trägt kein EVL1-Kontrakt", path);
        return Vec::new();
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 20;
        let Some(t) = bytes
            .get(o..o + 8)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(v) = bytes
            .get(o + 8..o + 16)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(idx) = bytes
            .get(o + 16..o + 20)
            .and_then(|b| b.try_into().ok())
            .map(u32::from_le_bytes)
        else {
            continue;
        };
        out.push((t, v, idx));
    }
    out
}

fn bin_median(series: &[(f64, f64)], t0: f64, bins: usize) -> Vec<Option<f32>> {
    let mut acc: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(t, v) in series {
        let idx = ((t - t0) / DT).floor();
        if idx < 0.0 || idx >= bins as f64 {
            continue;
        }
        acc[idx as usize].push(v);
    }
    acc.into_iter()
        .map(|mut v| {
            if v.is_empty() {
                return None;
            }
            v.sort_by(|a, b| a.total_cmp(b));
            let m = if v.len() % 2 == 0 {
                (v[v.len() / 2 - 1] + v[v.len() / 2]) * 0.5
            } else {
                v[v.len() / 2]
            };
            Some(m as f32)
        })
        .collect()
}

fn median_f32(v: &[f32]) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    let m = if s.len() % 2 == 0 {
        (s[s.len() / 2 - 1] + s[s.len() / 2]) * 0.5
    } else {
        s[s.len() / 2]
    };
    Some(m)
}

struct Event {
    lines: Vec<Vec<f32>>,
}

fn stack_pair(
    events: &[Event],
    ci: usize,
    hi: usize,
    lag: usize,
    shuffle: bool,
    seed: u64,
) -> (f64, usize, usize) {
    let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut sum = 0.0;
    let mut pos = 0usize;
    let mut tot = 0usize;
    for ev in events {
        let cool_use: Vec<f32>;
        let cool = if shuffle {
            cool_use = phase_randomized_surrogate(&ev.lines[ci], &mut rng);
            &cool_use
        } else {
            &ev.lines[ci]
        };
        let hot = &ev.lines[hi];
        let (Some(f), Some(r)) = (
            transfer_entropy_lag(hot, cool, lag),
            transfer_entropy_lag(cool, hot, lag),
        ) else {
            continue;
        };
        let d = f - r;
        let scale = (f.abs() + r.abs()).max(1e-12);
        sum += d / scale;
        tot += 1;
        if d > 0.0 {
            pos += 1;
        }
    }
    (sum / tot.max(1) as f64, pos, tot)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = arg_value(&args, "--eve") else {
        eprintln!("--eve <eve_lines.bin> fehlt");
        return;
    };
    let records = read_eve_lines(&path);
    let mut series: Vec<(f64, f64)> = Vec::new();
    for (idx, _name, _logt) in LINES {
        let s: Vec<(f64, f64)> = records
            .iter()
            .filter(|(_, _, i)| *i == idx)
            .map(|&(t, v, _)| (t, v))
            .collect();
        if s.is_empty() {
            eprintln!("Linie {} fehlt im Bin", idx);
            return;
        }
        series.extend(s);
    }
    let t0 = series.iter().map(|&(t, _)| t).fold(f64::INFINITY, f64::min);
    let t1 = series
        .iter()
        .map(|&(t, _)| t)
        .fold(f64::NEG_INFINITY, f64::max);
    let bins = ((t1 - t0) / DT).floor() as usize;

    let mut grid: Vec<Vec<Option<f32>>> = Vec::new();
    for (idx, _name, _logt) in LINES {
        let s: Vec<(f64, f64)> = records
            .iter()
            .filter(|(_, _, i)| *i == idx)
            .map(|&(t, v, _)| (t, v))
            .collect();
        grid.push(bin_median(&s, t0, bins));
    }
    let n = grid[0].len();
    let trig = &grid[6];
    let mut trig_vals: Vec<f32> = trig.iter().filter_map(|&o| o).collect();
    let Some(bg) = median_f32(&trig_vals) else {
        eprintln!("94-Å-Linie trägt keine Zellen");
        return;
    };
    trig_vals.clear();
    let threshold = bg * THRESH_FACTOR as f32;

    let mut events: Vec<Event> = Vec::new();
    let mut i = 0usize;
    while i < n {
        let Some(tv) = trig[i] else {
            i += 1;
            continue;
        };
        if tv < threshold {
            i += 1;
            continue;
        }
        let mut j = i;
        let mut peak = i;
        while j < n && j - i < REFRACTORY {
            if let Some(v) = trig[j] {
                if v < threshold && j > i + 3 {
                    break;
                }
                if let Some(pv) = trig[peak] {
                    if v > pv {
                        peak = j;
                    }
                }
            }
            j += 1;
        }
        let lo = peak.saturating_sub(WINDOW);
        let hi = (peak + WINDOW).min(n);
        let mut lines = vec![Vec::new(); 7];
        for k in lo..hi {
            let mut all = true;
            for (li, g) in grid.iter().enumerate() {
                match g[k] {
                    Some(v) => lines[li].push(v),
                    None => {
                        all = false;
                        break;
                    }
                }
            }
            if !all {
                for l in lines.iter_mut() {
                    l.clear();
                }
                break;
            }
        }
        if lines[0].len() >= 100 {
            events.push(Event { lines });
        }
        i = j.max(i + REFRACTORY);
    }

    println!(
        "{} Ereignisse (94 Å > {:.1}× Median = {:.2e}), Fenster ±20 min, 10-s-Zellen",
        events.len(),
        THRESH_FACTOR,
        threshold
    );
    println!();
    println!("Die Leiter (kühl → heiß): 304 → 131 → 171 → 195 → 211 → 284 → 94 Å");
    println!("D(lag) = TE(kühl→heiß) − TE(heiß→kühl), positiv = Fluss die Leiter hoch.");
    println!();
    println!(
        "{:>12} | {:>9} | {:>8} | {:>6} | {:>9}",
        "Paar", "D(lag10)", "Anteil>0", "n", "Null-max"
    );
    for p in 0..6 {
        let (d10, pos, tot) = stack_pair(&events, p, p + 1, 10, false, 0);
        let mut null_max = f64::NEG_INFINITY;
        for s in 1..=N_SURR {
            let (d_null, _, _) = stack_pair(
                &events,
                p,
                p + 1,
                10,
                true,
                s as u64 * 0x9E37_79B9_7F4A_7C15,
            );
            if d_null > null_max {
                null_max = d_null;
            }
        }
        let pair = format!("{}→{}", LINES[p].1, LINES[p + 1].1);
        let sig = if d10 > null_max {
            "  <-- über Null"
        } else {
            ""
        };
        println!(
            "{:>12} | {:>9.4e} | {:>7.1}% | {:>6} | {:>9.4e}{}",
            pair,
            d10,
            pos as f64 / tot.max(1) as f64 * 100.0,
            tot,
            null_max,
            sig
        );
    }
    println!();
    println!("lag 10 ≈ 100 s. Die Spalte zeigt D bei der Alfvén-Skala; Null-max = stärkster D der phasenrandomisierten Runden (kühle Linie je Fenster umgeordnet).");
}
