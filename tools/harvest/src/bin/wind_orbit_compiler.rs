use omegaflow::cdf::value_present;
use omegaflow::cdf25::Cdf25File;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use omegaflow::wind_orbit::{parse_bin, write_bin};
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const PRE_BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/wind/orbit/pre_or/";
const DEF_BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/wind/orbit/def_or/";
const CDN_RELEASE: &str = "spdf.gsfc.nasa.gov";
const RECORDS_PER_DAY_CAP: usize = 288;

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn parse_days(s: &str) -> Option<i64> {
    let (y, rest) = s.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn listing_versions(base: &str, year: i64) -> HashMap<String, u32> {
    let url = format!("{base}{year}/");
    let Some(bytes) = fetch(&url) else {
        return HashMap::new();
    };
    let text = String::from_utf8_lossy(&bytes);
    let mut map: HashMap<String, u32> = HashMap::new();
    for token in text.split("\"") {
        if token.len() < 21 || !token.ends_with(".cdf") {
            continue;
        }
        let name = token.rsplit('/').next().unwrap_or(token);
        let stem = name.strip_suffix(".cdf").unwrap_or(name);
        let Some((_, v)) = stem.rsplit_once("_v") else {
            continue;
        };
        let Ok(version) = v.parse::<u32>() else {
            continue;
        };
        let date = match base {
            b if b.ends_with("pre_or/") => stem
                .strip_prefix("wi_or_pre_")
                .map(|s| &s[..8])
                .unwrap_or(stem),
            b if b.ends_with("def_or/") => stem
                .strip_prefix("wi_or_def_")
                .map(|s| &s[..8])
                .unwrap_or(stem),
            _ => stem,
        };
        let entry = map.entry(date.to_string()).or_insert(version);
        if version > *entry {
            *entry = version;
        }
    }
    map
}

struct DayPlan {
    date: String,
    url: Option<String>,
}

fn build_plans(start_day: i64, end_day: i64) -> Vec<DayPlan> {
    let mut plans = Vec::new();
    let mut years: Vec<i64> = (start_day..=end_day)
        .map(|d| civil_from_days(d).0)
        .collect();
    years.sort();
    years.dedup();
    let mut pre_maps: HashMap<i64, HashMap<String, u32>> = HashMap::new();
    let mut def_maps: HashMap<i64, HashMap<String, u32>> = HashMap::new();
    for year in &years {
        pre_maps.insert(*year, listing_versions(PRE_BASE, *year));
        if (1994..=1997).contains(year) {
            def_maps.insert(*year, listing_versions(DEF_BASE, *year));
        }
    }
    for day in start_day..=end_day {
        let (year, month, mday) = civil_from_days(day);
        let date = format!("{year:04}{month:02}{mday:02}");
        let def_url = def_maps
            .get(&year)
            .and_then(|m| m.get(&date))
            .map(|v| format!("{DEF_BASE}{year}/wi_or_def_{date}_v{v:02}.cdf"));
        let pre_url = pre_maps
            .get(&year)
            .and_then(|m| m.get(&date))
            .map(|v| format!("{PRE_BASE}{year}/wi_or_pre_{date}_v{v:02}.cdf"));
        plans.push(DayPlan {
            date,
            url: def_url.or(pre_url),
        });
    }
    plans
}

fn probe(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let file = match Cdf25File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: {:?}", note);
            std::process::exit(1);
        }
    };
    eprintln!(
        "{}: CDF {}.{}.{} encoding {} {} rVariables rdim {:?}",
        path,
        file.version.0,
        file.version.1,
        file.version.2,
        file.encoding,
        file.vars.len(),
        file.rdim_sizes,
    );
    for var in &file.vars {
        eprintln!(
            "  var {} {} type {} x{} max_rec {}",
            var.var_num, var.name, var.data_type, var.num_elements, var.max_rec
        );
    }
    let Some(epoch) = file.var("Epoch") else {
        eprintln!("  Epoch absent");
        return;
    };
    match file.var_records(&bytes, epoch) {
        Ok(records) => eprintln!(
            "  Epoch {} records first {:?} last {:?} (unix seconds)",
            records.len(),
            records.first().map(|v| v.first().copied()),
            records.last().map(|v| v.first().copied()),
        ),
        Err(note) => eprintln!("  Epoch: {:?}", note),
    }
    for name in ["GCI_POS", "GCI_VEL"] {
        let Some(var) = file.var(name) else {
            continue;
        };
        match file.var_records(&bytes, var) {
            Ok(records) => eprintln!(
                "  {name} {} records first {:?} last {:?}",
                records.len(),
                records.first(),
                records.last(),
            ),
            Err(note) => eprintln!("  {name}: {:?}", note),
        }
    }
}

struct DayOutcome {
    date: String,
    emitted: usize,
    note: Option<String>,
}

fn harvest_day(
    plan: &DayPlan,
    lsk: &omegaflow::lsk::LeapSeconds,
    records: &Mutex<Vec<(f64, [f64; 3], [f64; 3])>>,
    outcomes: &Mutex<Vec<DayOutcome>>,
) {
    let url = match &plan.url {
        Some(u) => u.clone(),
        None => {
            outcomes
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(DayOutcome {
                    date: plan.date.clone(),
                    emitted: 0,
                    note: Some("listing carries no file for this day".to_string()),
                });
            return;
        }
    };
    let Some(bytes) = fetch(&url) else {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                date: plan.date.clone(),
                emitted: 0,
                note: Some("fetch void".to_string()),
            });
        return;
    };
    let file = match Cdf25File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            outcomes
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(DayOutcome {
                    date: plan.date.clone(),
                    emitted: 0,
                    note: Some(format!("{note:?}")),
                });
            return;
        }
    };
    let read_var = |name: &str| -> Option<Vec<Vec<f64>>> {
        let var = file.var(name)?;
        file.var_records(&bytes, var).ok()
    };
    let Some(epoch_records) = read_var("Epoch") else {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                date: plan.date.clone(),
                emitted: 0,
                note: Some("Epoch absent or void".to_string()),
            });
        return;
    };
    let Some(pos_records) = read_var("GCI_POS") else {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                date: plan.date.clone(),
                emitted: 0,
                note: Some("GCI_POS absent or void".to_string()),
            });
        return;
    };
    let Some(vel_records) = read_var("GCI_VEL") else {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                date: plan.date.clone(),
                emitted: 0,
                note: Some("GCI_VEL absent or void".to_string()),
            });
        return;
    };
    let n = epoch_records.len();
    if n == 0 || pos_records.len() != n || vel_records.len() != n {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                date: plan.date.clone(),
                emitted: 0,
                note: Some(format!(
                    "record counts drift: Epoch {n}, GCI_POS {}, GCI_VEL {}",
                    pos_records.len(),
                    vel_records.len()
                )),
            });
        return;
    }
    let step = if n > RECORDS_PER_DAY_CAP {
        (n + RECORDS_PER_DAY_CAP - 1) / RECORDS_PER_DAY_CAP
    } else {
        1
    };
    let mut day_records: Vec<(f64, [f64; 3], [f64; 3])> = Vec::with_capacity(n / step + 1);
    for i in (0..n).step_by(step) {
        let Some(t_unix) = epoch_records[i].first().copied() else {
            continue;
        };
        if !t_unix.is_finite() {
            continue;
        }
        if pos_records[i].len() < 3 || vel_records[i].len() < 3 {
            continue;
        }
        let pos = [pos_records[i][0], pos_records[i][1], pos_records[i][2]];
        let vel = [vel_records[i][0], vel_records[i][1], vel_records[i][2]];
        if pos.iter().chain(vel.iter()).any(|v| !value_present(*v)) {
            continue;
        }
        let Some(t_tdb) = lsk.unix_to_tdb(t_unix) else {
            continue;
        };
        day_records.push((
            t_tdb,
            [pos[0] * 1000.0, pos[1] * 1000.0, pos[2] * 1000.0],
            [vel[0] * 1000.0, vel[1] * 1000.0, vel[2] * 1000.0],
        ));
    }
    let emitted = day_records.len();
    records
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .append(&mut day_records);
    outcomes
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(DayOutcome {
            date: plan.date.clone(),
            emitted,
            note: None,
        });
}

const J2000_JD: f64 = 2451545.0;

fn dash(date: &str) -> String {
    format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
}

fn parse_horizons(text: &str) -> Vec<(f64, [f64; 3], [f64; 3])> {
    let mut out = Vec::new();
    let mut in_data = false;
    for line in text.lines() {
        if line.starts_with("$$SOE") {
            in_data = true;
            continue;
        }
        if line.starts_with("$$EOE") {
            break;
        }
        if !in_data {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 8 {
            continue;
        }
        let jdt: f64 = cols[0].trim().parse().ok().unwrap_or(f64::NAN);
        let x: f64 = cols[2].trim().parse().ok().unwrap_or(f64::NAN);
        let y: f64 = cols[3].trim().parse().ok().unwrap_or(f64::NAN);
        let z: f64 = cols[4].trim().parse().ok().unwrap_or(f64::NAN);
        let vx: f64 = cols[5].trim().parse().ok().unwrap_or(f64::NAN);
        let vy: f64 = cols[6].trim().parse().ok().unwrap_or(f64::NAN);
        let vz: f64 = cols[7].trim().parse().ok().unwrap_or(f64::NAN);
        let t_tdb = (jdt - J2000_JD) * 86400.0;
        if !t_tdb.is_finite()
            || [x, y, z, vx, vy, vz]
                .iter()
                .any(|v| !v.is_finite() || v.abs() >= 1.0e8)
        {
            continue;
        }
        out.push((
            t_tdb,
            [x * 1000.0, y * 1000.0, z * 1000.0],
            [vx * 1000.0, vy * 1000.0, vz * 1000.0],
        ));
    }
    out
}

fn fetch_horizons(date: &str) -> Option<Vec<(f64, [f64; 3], [f64; 3])>> {
    let url = format!(
        "https://ssd.jpl.nasa.gov/api/horizons.api?format=text&COMMAND=-8&OBJ_DATA=NO&MAKE_EPHEM=YES&EPHEM_TYPE=VECTORS&CENTER=500@399&REF_PLANE=FRAME&START_TIME={}T00:00&STOP_TIME={}T23:50&STEP_SIZE=10m&VEC_TABLE=2&OUT_UNITS=KM-S&CSV_FORMAT=YES",
        dash(date),
        dash(date),
    );
    let bytes = fetch(&url)?;
    let text = String::from_utf8_lossy(&bytes);
    let recs = parse_horizons(&text);
    if recs.is_empty() { None } else { Some(recs) }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = arg_value(&args, "--probe") {
        probe(&path);
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let fill_horizons = args.iter().any(|a| a == "--fill-horizons");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "wind_orbit.bin".to_string());
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);
    let start_day = match arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => {
            eprintln!("--window-start absent — the harvest window stays undeclared");
            std::process::exit(1);
        }
    };
    let end_day = match arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => {
            eprintln!("--window-end absent — the harvest window stays undeclared");
            std::process::exit(1);
        }
    };
    if start_day > end_day {
        eprintln!("--window-start lies after --window-end — the window stays unharvested");
        std::process::exit(1);
    }
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let plans = build_plans(start_day, end_day);
    let total = plans.len();
    let declared = plans.iter().filter(|p| p.url.is_some()).count();
    eprintln!("window: {} days, {} carry a listing file", total, declared);
    let records: Arc<Mutex<Vec<(f64, [f64; 3], [f64; 3])>>> = Arc::new(Mutex::new(Vec::new()));
    let outcomes: Arc<Mutex<Vec<DayOutcome>>> = Arc::new(Mutex::new(Vec::new()));
    let next = Arc::new(AtomicI64::new(0));
    let done = Arc::new(AtomicUsize::new(0));
    let lsk_ref = &lsk;
    std::thread::scope(|s| {
        for _ in 0..jobs {
            let records = Arc::clone(&records);
            let outcomes = Arc::clone(&outcomes);
            let next = Arc::clone(&next);
            let done = Arc::clone(&done);
            let plans = &plans;
            s.spawn(move || {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst) as usize;
                    if i >= plans.len() {
                        break;
                    }
                    harvest_day(&plans[i], lsk_ref, &records, &outcomes);
                    let n = done.fetch_add(1, Ordering::SeqCst) + 1;
                    if n % 400 == 0 || n == total {
                        eprintln!("{n}/{total} days harvested");
                    }
                }
            });
        }
    });
    let outcomes = Arc::try_unwrap(outcomes)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut void = 0usize;
    let mut emitted_total = 0usize;
    let mut year_void: HashMap<i64, usize> = HashMap::new();
    for o in &outcomes {
        emitted_total += o.emitted;
        if let Some(note) = &o.note {
            void += 1;
            let year = o.date[..4].parse::<i64>().unwrap_or(0);
            *year_void.entry(year).or_default() += 1;
            eprintln!("{} void: {}", o.date, note);
        }
    }
    let mut void_years: Vec<i64> = year_void.keys().copied().collect();
    void_years.sort();
    eprintln!(
        "{} days, {} void, {} records emitted, void years: {:?}",
        total, void, emitted_total, void_years
    );
    let records = Arc::try_unwrap(records)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut records = records;
    if fill_horizons {
        let mut filled = 0usize;
        let mut emitted_fill = 0usize;
        for o in &outcomes {
            if o.note.as_deref() != Some("listing carries no file for this day") {
                continue;
            }
            match fetch_horizons(&o.date) {
                Some(recs) => {
                    emitted_fill += recs.len();
                    filled += 1;
                    eprintln!("{}: {} Horizons records filled", o.date, recs.len());
                    records.extend(recs);
                }
                None => eprintln!("{}: Horizons fill void — the day stays a gap", o.date),
            }
        }
        eprintln!(
            "{} days filled from Horizons (Wind -8, {} records)",
            filled, emitted_fill
        );
    }
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0));
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{out}: {} records, roundtrip parses", parsed.len());
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_RELEASE, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_horizons_csv() {
        let text = "API VERSION: 1.2\n$$SOE\n\
2460142.500000000, A.D. 2023-Jul-17 00:00:00.0000, -6.298255498538257E+05,  1.412329293954745E+06,  7.064144245636598E+05, -7.470608417721109E-02, -2.760906889224624E-02, -1.419973463542988E-02,\n\
$$EOE\n";
        let recs = parse_horizons(text);
        assert_eq!(recs.len(), 1);
        let (t, p, v) = recs[0];
        assert!((t - 742824000.0).abs() < 1.0e-6);
        assert!((p[0] - (-629825549.8538)).abs() < 1.0);
        assert!((p[1] - 1412329293.9547).abs() < 1.0);
        assert!((v[0] - (-74.70608417721109)).abs() < 1.0e-9);
    }

    #[test]
    fn dash_formats_dates() {
        assert_eq!(dash("20230717"), "2023-07-17");
        assert_eq!(dash("20250204"), "2025-02-04");
    }

    #[test]
    fn rejects_garbage_before_soe() {
        assert!(parse_horizons("no data here").is_empty());
    }
}
