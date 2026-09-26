use std::process::exit;

use omegaflow::te::{surrogate_threshold_lag, transfer_entropy_lag};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const ALIGN_WINDOW_S: i64 = 3600;

fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn unix_of_datetime(dt: &str) -> Option<i64> {
    let parts: Vec<&str> = dt.split(['-', 'T', ':']).collect();
    if parts.len() < 5 {
        return None;
    }
    let y: i64 = parts[0].parse().ok()?;
    let mo: i64 = parts[1].parse().ok()?;
    let d: i64 = parts[2].parse().ok()?;
    let h: i64 = parts[3].parse().ok()?;
    let mi: i64 = parts[4].parse().ok()?;
    if !(1960..=2200).contains(&y) || !(1..=12).contains(&mo) || h > 23 || mi > 59 {
        return None;
    }
    let mdays = [
        31,
        if is_leap(y) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    if d < 1 || d > mdays[(mo - 1) as usize] {
        return None;
    }
    let mut days = 0i64;
    for yy in 1970..y {
        days += if is_leap(yy) { 366 } else { 365 };
    }
    for m in 0..(mo - 1) as usize {
        days += mdays[m];
    }
    days += d - 1;
    Some(days * 86400 + h * 3600 + mi * 60)
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    (y + if m <= 2 { 1 } else { 0 }, m, d)
}

fn civil_of_unix(t: i64) -> String {
    let days = t / 86400;
    let rem = t % 86400;
    let h = rem / 3600;
    let mi = (rem % 3600) / 60;
    let se = rem % 60;
    let (y, mo, d) = civil_from_days(days);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{se:02}")
}

fn read_stage_csv(path: &str) -> Option<(Vec<(i64, f32)>, usize)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut out = Vec::new();
    let mut skipped = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((dt, v)) = line.split_once(' ') else {
            skipped += 1;
            continue;
        };
        let t = match unix_of_datetime(dt.trim()) {
            Some(t) => t,
            None => {
                skipped += 1;
                continue;
            }
        };
        match v.trim().parse::<f32>() {
            Ok(x) if x.is_finite() => out.push((t, x)),
            _ => skipped += 1,
        }
    }
    Some((out, skipped))
}

fn read_precip_csv(path: &str) -> Option<(Vec<(i64, f32)>, usize)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut out = Vec::new();
    let mut skipped = 0usize;
    let mut in_data = false;
    for line in text.lines() {
        let line = line.trim();
        if !in_data {
            if line.starts_with("time,") {
                in_data = true;
            }
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let Some((dt, v)) = line.split_once(',') else {
            skipped += 1;
            continue;
        };
        let t = match unix_of_datetime(dt.trim()) {
            Some(t) => t,
            None => {
                skipped += 1;
                continue;
            }
        };
        match v.trim().parse::<f32>() {
            Ok(x) if x.is_finite() => out.push((t, x)),
            _ => skipped += 1,
        }
    }
    if !in_data {
        return None;
    }
    Some((out, skipped))
}

fn align_hourly(stage: &[(i64, f32)], precip: &[(i64, f32)]) -> (Vec<f32>, Vec<f32>) {
    let mut out_p = Vec::new();
    let mut out_s = Vec::new();
    let mut j = 0usize;
    for &(tp, pv) in precip {
        while j + 1 < stage.len() && stage[j + 1].0 <= tp {
            j += 1;
        }
        if j < stage.len() && stage[j].0 <= tp && tp - stage[j].0 <= ALIGN_WINDOW_S {
            out_p.push(pv);
            out_s.push(stage[j].1);
        }
    }
    (out_p, out_s)
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let stage_path = flag(&args, "--stage");
    let precip_path = flag(&args, "--precip-csv");
    let station_id = match flag(&args, "--station-id") {
        Some(v) => v,
        None => "4913".to_string(),
    };
    let station_name = match flag(&args, "--station-name") {
        Some(v) => v,
        None => "Bhotekoshi at Rasuwagadhi".to_string(),
    };
    let precip_name = match flag(&args, "--precip-name") {
        Some(v) => v,
        None => "rasuwa".to_string(),
    };
    let precip_coords = match flag(&args, "--precip-coords") {
        Some(v) => v,
        None => "28.25, 85.10".to_string(),
    };
    let lags: Vec<usize> = match flag(&args, "--lags") {
        Some(v) => v.split(',').filter_map(|s| s.parse().ok()).collect(),
        None => vec![1, 3, 6, 12, 24],
    };
    let (Some(sp), Some(pp)) = (stage_path, precip_path) else {
        eprintln!(
            "--stage <dhm_{station_id}_stage.csv> --precip-csv <open-meteo.csv> [--station-id N] [--station-name NAME] [--precip-name NAME] [--precip-coords LAT,LON] required"
        );
        exit(2);
    };

    println!("=== Trishuli precipitation -> stage TE lag sweep ===");
    println!(
        "precipitation route : Open-Meteo archive-api, {precip_name} ({precip_coords}), hourly, keyless"
    );
    println!(
        "stage route         : DHM Nepal river-watch, {station_name} (id {station_id}), 10-min, keyless"
    );
    println!(
        "stage source note   : the live rolling buffer covers only ~4.6 d and ages the flood window out;"
    );
    println!("                      the flood-window series is carried by a wayback page snapshot");
    println!(
        "                      (livefeed_gate --dhm {station_id} --dhm-page <snapshot.html>);"
    );
    println!(
        "                      measured 2026-09-26: snapshot 20260901142220 -> 2026-08-25 14:25 .."
    );
    println!(
        "                      2026-09-01 14:15 UTC (station 113); station 4913 ends 08-26 02:55 UTC."
    );
    println!("precip file         : {pp}");
    println!("stage file          : {sp}");
    println!();

    let stage = read_stage_csv(&sp);
    let Some((stage_rows, stage_skipped)) = stage else {
        println!(
            "stage series: absent — no dhm_{station_id}_stage.csv was written by livefeed_gate --dhm"
        );
        println!(
            "(the live DHM river-watch rolling buffer covers only ~4.6 d and ages the flood window out;"
        );
        println!("the flood-window series is recoverable via a wayback snapshot of the page —");
        println!(
            "livefeed_gate --dhm {station_id} --dhm-page <snapshot.html>; measured 2026-09-26: snapshot"
        );
        println!(
            "20260901142220 carries the flood window (station 113 full, 4913 to 08-26 02:55 UTC).)"
        );
        println!();
        println!(
            "verdict: no sweep — the stage series is absent on the fetched source (0 honored)"
        );
        return;
    };
    let precip = read_precip_csv(&pp);
    let Some((precip_rows, precip_skipped)) = precip else {
        println!(
            "precip series: absent — the file carries no time,value header row (archive-api csv shape)"
        );
        println!();
        println!("verdict: no sweep — the precipitation series is absent (0 honored)");
        return;
    };

    let (Some(&(stage_first, _)), Some(&(stage_last, _))) = (stage_rows.first(), stage_rows.last())
    else {
        println!("stage series: empty — no parseable rows in {sp}");
        return;
    };
    let (Some(&(precip_first, _)), Some(&(precip_last, _))) =
        (precip_rows.first(), precip_rows.last())
    else {
        println!("precip series: empty — no parseable rows in {pp}");
        return;
    };

    let (precip, stage) = align_hourly(&stage_rows, &precip_rows);
    let n = precip.len();
    println!(
        "stage  : {} .. {} | rows {} (skipped {})",
        civil_of_unix(stage_first),
        civil_of_unix(stage_last),
        stage_rows.len(),
        stage_skipped
    );
    println!(
        "precip : {} .. {} | rows {} (skipped {})",
        civil_of_unix(precip_first),
        civil_of_unix(precip_last),
        precip_rows.len(),
        precip_skipped
    );
    println!(
        "alignment: stage value = last 10-min reading at-or-before each precip hour (<= 60 min back) | aligned n = {n}"
    );
    println!();

    if n < 30 {
        println!("n = {n} < 30 -> no finding (underdetermination, no fabrication)");
        return;
    }

    println!(
        "pair: precipitation <-> stage | n = {n} | lags = {:?} | null: 10 shuffled surrogates, mean+2sigma",
        lags
    );
    println!("convention: transfer_entropy_lag(x, y) = TE(y -> x) — second argument = source;");
    println!(
        "columns carry the true direction (the registered §3.5 mirror error is not repeated)."
    );
    println!();
    println!(
        "{:>4} | {:>18} | {:>12} | {:>18} | {:>12} | {}",
        "lag", "TE(precip->stage)", "threshold", "TE(stage->precip)", "threshold", "verdict"
    );
    for &lag in &lags {
        let te_fwd = transfer_entropy_lag(&stage, &precip, lag);
        let thr_fwd = surrogate_threshold_lag(&stage, &precip, lag, SEED);
        let te_rev = transfer_entropy_lag(&precip, &stage, lag);
        let thr_rev = surrogate_threshold_lag(&precip, &stage, lag, SEED);
        let (Some(te_fwd), Some(thr_fwd), Some(te_rev), Some(thr_rev)) =
            (te_fwd, thr_fwd, te_rev, thr_rev)
        else {
            println!("{lag:>4} | too few data");
            continue;
        };
        let sig_fwd = te_fwd > thr_fwd;
        let sig_rev = te_rev > thr_rev;
        let verdict = match (sig_fwd, sig_rev) {
            (true, true) => "both".to_string(),
            (true, false) => "precip -> stage".to_string(),
            (false, true) => "stage -> precip".to_string(),
            _ => "no finding".to_string(),
        };
        println!(
            "{:>4} | {:>18.5e} | {:>12.5e} | {:>18.5e} | {:>12.5e} | {}",
            lag, te_fwd, thr_fwd, te_rev, thr_rev, verdict
        );
    }
    println!();
    println!(
        "TE > threshold (mean+2sigma shuffled surrogates) = significant arrow; else no finding."
    );
    println!(
        "window note: the flood-window stage series is carried only by a wayback page snapshot,"
    );
    println!("not by the live rolling buffer (which covers ~4.6 d before the fetch).");
}
