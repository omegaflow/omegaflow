use std::fs;

fn days_in_month(y: i64, m: i64) -> i64 {
    if m == 2 {
        if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
            29
        } else {
            28
        }
    } else if m == 4 || m == 6 || m == 9 || m == 11 {
        30
    } else {
        31
    }
}

fn civil_to_doy(y: i64, m: i64, d: i64) -> i64 {
    let mut days = 0i64;
    let mut mm = 1i64;
    while mm < m {
        days += days_in_month(y, mm);
        mm += 1;
    }
    days + d
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("galileo GO-SUN SWSE4 coverage probe: <index path> <report path>");
        return;
    }
    let Ok(text) = fs::read_to_string(&args[0]) else {
        eprintln!("index read void");
        return;
    };
    let mut out: Vec<String> = Vec::new();
    let mut rows: Vec<[String; 5]> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 6 {
            continue;
        }
        let vol = f[0].trim_matches('"').to_string();
        let prod = f[2].trim_matches('"').to_string();
        let start = f[3].trim_matches('"').to_string();
        let stop = f[4].trim_matches('"').to_string();
        if vol.is_empty() || start.len() < 10 {
            continue;
        }
        rows.push([vol, prod, start, stop, line.to_string()]);
    }
    out.push(format!(
        "galileo GO-SUN-RSS-1-ODR-V1.0 SWS4-era anchor-day coverage: manifest census of {} data rows",
        rows.len()
    ));

    let mut anchor: Vec<(i64, String)> = Vec::new();
    let anchors: [(i64, i64, i64); 4] = [
        (1995, 11, 24),
        (1995, 12, 4),
        (1995, 12, 5),
        (1995, 12, 6),
    ];
    for (y, m, d) in anchors {
        anchor.push((civil_to_doy(y, m, d), format!("{y}-{m:02}-{d:02}")));
    }

    let mut per_day: Vec<(i64, Vec<usize>)> = Vec::new();
    for (idx, r) in rows.iter().enumerate() {
        let ymd: Vec<i64> = r[2]
            .split('T')
            .next()
            .unwrap_or("")
            .split('-')
            .filter_map(|x| x.parse().ok())
            .collect();
        if ymd.len() != 3 {
            continue;
        }
        let (y, m, d) = (ymd[0], ymd[1], ymd[2]);
        if y < 1995 || (y == 1995 && m < 11) || (y == 1995 && m == 11 && d < 20) || (y == 1996 && m == 1 && d > 20) || y > 1996 {
            continue;
        }
        let doy = civil_to_doy(y, m, d);
        match per_day.iter_mut().find(|e| e.0 == doy) {
            Some(e) => e.1.push(idx),
            None => per_day.push((doy, vec![idx])),
        }
    }
    per_day.sort_by_key(|e| e.0);
    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;
    for (_, idxs) in &per_day {
        for idx in idxs {
            if first.is_none() {
                first = Some(*idx);
            }
            last = Some(*idx);
        }
    }
    if let Some(idx) = first {
        out.push(format!(
            "earliest ODR file in manifest: {} start {}",
            rows[idx][1], rows[idx][2]
        ));
    }
    if let Some(idx) = last {
        out.push(format!(
            "latest ODR file in manifest: {} start {}",
            rows[idx][1], rows[idx][2]
        ));
    }
    out.push(format!("SWS4-era daily file counts (1995-11-20 .. 1996-01-20):"));
    for (doy, idxs) in &per_day {
        let mut names: Vec<String> = Vec::new();
        for idx in idxs {
            names.push(rows[*idx][1].clone());
        }
        out.push(format!("  DOY {doy}: n {} {names:?}", idxs.len()));
    }
    for (doy, label) in &anchor {
        match per_day.iter().find(|e| e.0 == *doy) {
            Some(e) => out.push(format!("anchor {label} (DOY {doy}): n {}", e.1.len())),
            None => out.push(format!(
                "anchor {label} (DOY {doy}): n 0 — no GO-SUN ODR file on this day (0 honored)"
            )),
        }
    }
    let s = out.join("\n");
    println!("{s}");
    if let Err(e) = fs::write(&args[1], s + "\n") {
        eprintln!("report write void: {e}");
    }
}
