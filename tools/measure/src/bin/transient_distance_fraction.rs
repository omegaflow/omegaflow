use omegaflow::archivar::json::{jnum, jstr, parse_json, JsonVal};
use omegaflow::archivar::spatial::{parse_star_record, star_stride, STAR_RECORD_BYTES};
use omegaflow_measure::weberin::deredden::build_star_index;
use std::collections::HashSet;

const BACKGROUND_PC_MIN: f64 = 200.0;
const DEG2_PER_SR: f64 = 129600.0 / std::f64::consts::PI;

#[derive(Clone)]
struct Alert {
    oid: Option<String>,
    ra_deg: f64,
    dec_deg: f64,
    ndet: Option<u64>,
    firstmjd: Option<f64>,
}

struct Hit {
    sep_arcsec: f64,
    d_pc: f64,
}

struct ClassTally {
    background_nearest: u64,
    foreground_nearest: u64,
    no_match: u64,
    background_any: u64,
}

struct DistTally {
    background_pc: Vec<f64>,
    background_sep_le1: u64,
}

fn read_alert(m: &JsonVal) -> Option<Alert> {
    let ra = jnum(m, "meanra").or_else(|| jnum(m, "ra"))?;
    let dec = jnum(m, "meandec").or_else(|| jnum(m, "dec"))?;
    if !(ra.is_finite() && dec.is_finite()) {
        return None;
    }
    if !((0.0..=360.0).contains(&ra) && (-90.0..=90.0).contains(&dec)) {
        return None;
    }
    let ndet = jnum(m, "ndet")
        .filter(|v| v.is_finite() && *v > 0.0)
        .map(|v| v as u64);
    let firstmjd = jnum(m, "firstmjd").filter(|v| v.is_finite());
    Some(Alert {
        oid: jstr(m, "oid"),
        ra_deg: ra,
        dec_deg: dec,
        ndet,
        firstmjd,
    })
}

fn parse_objects(text: &str) -> Vec<Alert> {
    let Some(root) = parse_json(text) else {
        return Vec::new();
    };
    let list: &Vec<JsonVal> = match &root {
        JsonVal::Arr(a) => a,
        JsonVal::Obj(map) => match map.get("items") {
            Some(JsonVal::Arr(a)) => a,
            _ => return Vec::new(),
        },
        _ => return Vec::new(),
    };
    let mut out = Vec::new();
    for m in list.iter() {
        if let Some(a) = read_alert(m) {
            out.push(a);
        }
    }
    out
}

fn classify(
    idx: &omegaflow_measure::weberin::deredden::StarIndex,
    alert: &Alert,
    radius_as: f64,
) -> (ClassTally, Option<Hit>) {
    let mut t = ClassTally { background_nearest: 0, foreground_nearest: 0, no_match: 0, background_any: 0 };
    let r_deg = radius_as / 3600.0;
    let found = idx.within(alert.ra_deg, alert.dec_deg, r_deg);
    if found.is_empty() {
        t.no_match += 1;
        return (t, None);
    }
    for pair in &found {
        let k = pair.0;
        let s = &idx.stars[k];
        let d_pc = 1000.0 / s.plx_mas;
        if d_pc > BACKGROUND_PC_MIN {
            t.background_any += 1;
        }
    }
    let (k, sep) = found[0];
    let s = &idx.stars[k];
    let d_pc = 1000.0 / s.plx_mas;
    if d_pc > BACKGROUND_PC_MIN {
        t.background_nearest += 1;
    } else {
        t.foreground_nearest += 1;
    }
    (
        t,
        Some(Hit {
            sep_arcsec: sep,
            d_pc,
        }),
    )
}

fn median_sorted(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len();
    if n % 2 == 1 {
        Some(v[n / 2])
    } else {
        Some(0.5 * (v[n / 2 - 1] + v[n / 2]))
    }
}

fn record_plx(chunk: &[u8]) -> Option<f64> {
    let bytes: [u8; 4] = chunk.get(24..28)?.try_into().ok()?;
    Some(f32::from_le_bytes(bytes) as f64)
}

fn field_list(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut taking = false;
    for a in args {
        if a == flag {
            taking = true;
        } else if taking {
            if a.starts_with("--") {
                break;
            }
            out.push(a.clone());
        }
    }
    out
}

fn arg_after(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn each_after(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (i, a) in args.iter().enumerate() {
        if a == flag {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
        }
    }
    out
}

fn report_table(
    idx: &omegaflow_measure::weberin::deredden::StarIndex,
    alerts: &[Alert],
    radius_as: f64,
    window_label: &str,
) {
    let n = alerts.len() as u64;
    if n == 0 {
        println!("{window_label} radius {radius_as:.0} arcsec: no alert measured (0 honored)");
        return;
    }
    let mut tally = ClassTally { background_nearest: 0, foreground_nearest: 0, no_match: 0, background_any: 0 };
    let mut dt = DistTally { background_pc: Vec::new(), background_sep_le1: 0 };
    for a in alerts {
        let (t, hit) = classify(idx, a, radius_as);
        tally.background_nearest += t.background_nearest;
        tally.foreground_nearest += t.foreground_nearest;
        tally.no_match += t.no_match;
        tally.background_any += t.background_any;
        if let Some(h) = hit {
            if h.d_pc > BACKGROUND_PC_MIN {
                dt.background_pc.push(h.d_pc);
                if h.sep_arcsec <= 1.0 {
                    dt.background_sep_le1 += 1;
                }
            }
        }
    }
    dt.background_pc.sort_by(|a, b| a.total_cmp(b));
    let med = median_sorted(&dt.background_pc);
    let placeable = tally.background_nearest;
    let fg = tally.foreground_nearest;
    let nomatch = tally.no_match;
    let frac = placeable as f64 / n as f64;
    let fg_frac = fg as f64 / n as f64;
    let nm_frac = nomatch as f64 / n as f64;
    let any_frac = tally.background_any as f64 / n as f64;
    println!(
        "{window_label} radius {radius_as:.0} arcsec | alerts {n} | placeable background (> {BACKGROUND_PC_MIN:.0} pc) {placeable} ({frac_pct:.2} %) | foreground (<= {BACKGROUND_PC_MIN:.0} pc) {fg} ({fg_pct:.2} %) | no catalog match {nomatch} ({nm_pct:.2} %)",
        frac_pct = frac * 100.0,
        fg_pct = fg_frac * 100.0,
        nm_pct = nm_frac * 100.0
    );
    let n_bg = dt.background_pc.len();
    let pct_le1 = if n_bg > 0 {
        dt.background_sep_le1 as f64 / n_bg as f64 * 100.0
    } else {
        0.0
    };
    let med_word = match med {
        Some(m) => format!("{m:.0} pc"),
        None => "no placeable match — the median stays absent".to_string(),
    };
    println!(
        "{window_label} radius {radius_as:.0} arcsec | any background star inside the radius (nearest star not required) {any} ({any_pct:.2} %) | of the {n_bg} placeable: separation <= 1 arcsec {le1} ({pct_le1:.2} %), median parallax distance {med_word}",
        any = tally.background_any,
        any_pct = any_frac * 100.0,
        le1 = dt.background_sep_le1
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(stars_path) = arg_after(&args, "--stars") else {
        eprintln!("usage: transient_distance_fraction --stars <dr3_stars.bin> --objects <alerts.json> [--objects <more.json> ...] --radius <arcsec> [--radius <arcsec> ...]");
        return;
    };
    let object_files = field_list(&args, "--objects");
    if object_files.is_empty() {
        eprintln!("--objects <alerts.json>: the alert sample is never silent");
        return;
    }
    let radius_words = each_after(&args, "--radius");
    if radius_words.is_empty() {
        eprintln!("--radius <arcsec>: the search radius is the operator's decision — name it before the measurement");
        return;
    }
    let mut radii: Vec<f64> = Vec::with_capacity(radius_words.len());
    for w in &radius_words {
        match w.parse::<f64>() {
            Ok(r) if r.is_finite() && r > 0.0 => radii.push(r),
            _ => {
                eprintln!("--radius {w}: not a finite positive arcsec — the gate stays closed");
                return;
            }
        }
    }

    let star_bytes = match std::fs::read(&stars_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {stars_path} returned void: {e}");
            return;
        }
    };
    let Some(stride) = star_stride(&star_bytes) else {
        eprintln!(
            "star bin {} bytes: no {}-byte records — the catalog stays unread",
            star_bytes.len(),
            STAR_RECORD_BYTES
        );
        return;
    };
    let n_records = star_bytes.len() / stride;
    let mut parsed = 0usize;
    let mut plx_nonpositive = 0usize;
    let mut plx_nonfinite = 0usize;
    let mut other_skip = 0usize;
    for chunk in star_bytes.chunks_exact(stride) {
        let Some(plx) = record_plx(chunk) else {
            other_skip += 1;
            continue;
        };
        if !plx.is_finite() {
            plx_nonfinite += 1;
        } else if plx <= 0.0 {
            plx_nonpositive += 1;
        } else if parse_star_record(chunk).is_none() {
            other_skip += 1;
        } else {
            parsed += 1;
        }
    }
    let idx = build_star_index(&star_bytes);
    let density = idx.stars.len() as f64 / DEG2_PER_SR;
    println!(
        "catalog {stars_path}: {n_records} records | parsed + indexed {} (positive-parallax, finite) | unparsed {} ({} plx <= 0, {} plx non-finite, {} other non-finite field)",
        parsed, n_records - parsed, plx_nonpositive, plx_nonfinite, other_skip
    );

    let mut alerts: Vec<Alert> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for path in &object_files {
        let Ok(text) = std::fs::read_to_string(path) else {
            eprintln!("read {path} returned void — the window stays unmeasured");
            continue;
        };
        let rows = parse_objects(&text);
        let rows_read = rows.len();
        let mut kept = 0usize;
        for a in rows {
            let duplicate = match &a.oid {
                Some(o) => !seen.insert(o.clone()),
                None => false,
            };
            if !duplicate {
                alerts.push(a);
                kept += 1;
            }
        }
        println!(
            "window {path}: {} alert rows read, {} kept after oid dedupe",
            rows_read, kept
        );
    }

    if alerts.is_empty() {
        println!("no alert measured — the aggregate stays unmeasured (0 honored)");
        return;
    }
    let mjd_lo = alerts
        .iter()
        .filter_map(|a| a.firstmjd)
        .fold(f64::INFINITY, f64::min);
    let mjd_hi = alerts
        .iter()
        .filter_map(|a| a.firstmjd)
        .fold(f64::NEG_INFINITY, f64::max);
    let multi: Vec<Alert> = alerts
        .iter()
        .filter(|a| a.ndet.is_some_and(|n| n >= 2))
        .cloned()
        .collect();
    println!(
        "sample: {} direction-only alert objects from {} windows, firstmjd [{:.3}, {:.3}], {} with >= 2 detections",
        alerts.len(),
        object_files.len(),
        mjd_lo,
        mjd_hi,
        multi.len()
    );
    for r in radii {
        report_table(&idx, &alerts, r, "all alerts");
        if !multi.is_empty() {
            report_table(&idx, &multi, r, "ndet>=2 alerts");
        }
        let expected = alerts.len() as f64 * density * std::f64::consts::PI * (r / 3600.0).powi(2);
        println!(
            "radius {r:.0} arcsec | random-coincidence expectation over the sample (catalog all-sky mean density {density:.1} stars/deg2): {expected:.2} background star positions inside the radius by chance"
        );
    }
}
