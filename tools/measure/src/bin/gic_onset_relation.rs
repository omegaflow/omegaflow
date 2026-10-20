use omegaflow::archivar::bpa_gic;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GeoRec, MAGIC_GIC, parse_bin as parse_geo};
use omegaflow::archivar::mcon_event::{parse_bin as parse_mcon, unix_from_jd};
use std::collections::BTreeMap;

const J2000_UNIX_OFFSET: f64 = 946728000.0;
const DAY_S: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const PRE_S: f64 = 3600.0;
fn default_mcon() -> &'static str {
    omegaflow::temp_path("mcpherron_onsets.bin")
}
const DEFAULT_GIC: &str =
    "https://github.com/omegaflow/sources/releases/download/space.fmi.fi/fmi_gic_1min.bin";
const DEFAULT_GIC2: &str =
    "https://github.com/omegaflow/sources/releases/download/transmission.bpa.gov/bpa_gic.bin";
const SURROGATE_OFFSETS_H: [f64; 10] = [-24.0, -12.0, -6.0, -3.0, -1.0, 1.0, 3.0, 6.0, 12.0, 24.0];

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

fn iso(tdb: f64) -> String {
    let t = tdb + J2000_UNIX_OFFSET;
    let days = (t / DAY_S).floor() as i64;
    let secs = t - days as f64 * DAY_S;
    let (y, m, d) = civil_from_days(days);
    let hh = (secs / HOUR) as i64;
    let mm = ((secs - hh as f64 * HOUR) / 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}Z")
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn read_bytes(path_or_url: &str) -> Option<Vec<u8>> {
    if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
        fetch_raw_bytes(path_or_url)
    } else {
        std::fs::read(path_or_url).ok()
    }
}

fn median(v: &mut Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = v.len();
    if n % 2 == 1 {
        Some(v[n / 2])
    } else {
        Some(0.5 * (v[n / 2 - 1] + v[n / 2]))
    }
}

fn in_range(unix: f64, lo: f64, hi: f64) -> bool {
    unix >= lo && unix <= hi
}

fn response_at(pts: &[(f64, f64)], t: f64, window_s: f64) -> Option<f64> {
    let lo = pts.partition_point(|p| p.0 < t - PRE_S);
    let hi = pts.partition_point(|p| p.0 < t);
    if hi <= lo {
        return None;
    }
    let mut pre: Vec<f64> = pts[lo..hi].iter().map(|p| p.1).collect();
    let pre_med = median(&mut pre)?;
    let p_lo = pts.partition_point(|p| p.0 < t);
    let p_hi = pts.partition_point(|p| p.0 < t + window_s);
    if p_hi <= p_lo {
        return None;
    }
    let post = pts[p_lo..p_hi]
        .iter()
        .map(|p| p.1)
        .fold(f64::NEG_INFINITY, f64::max);
    if !post.is_finite() {
        return None;
    }
    Some(post - pre_med)
}

fn station_series(recs: &[GeoRec]) -> BTreeMap<u32, Vec<(f64, f64)>> {
    let mut series: BTreeMap<u32, Vec<(f64, f64)>> = BTreeMap::new();
    for r in recs {
        if !r.val.is_finite() {
            continue;
        }
        series
            .entry(r.station)
            .or_default()
            .push((r.t + J2000_UNIX_OFFSET, r.val.abs()));
    }
    series
}

fn bpa_channel_series(recs: &[bpa_gic::BpaGicRecord]) -> BTreeMap<u32, Vec<(f64, f64)>> {
    let mut series: BTreeMap<u32, Vec<(f64, f64)>> = BTreeMap::new();
    for r in recs {
        if !r.t_unix.is_finite() {
            continue;
        }
        for (comp, v) in r.values().iter().enumerate() {
            if let Some(x) = v {
                if x.is_finite() {
                    series
                        .entry(comp as u32)
                        .or_default()
                        .push((r.t_unix, x.abs()));
                }
            }
        }
    }
    series
}

fn finite_mean(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        None
    } else {
        Some(v.iter().sum::<f64>() / v.len() as f64)
    }
}

fn stddev(v: &[f64], mean: f64) -> Option<f64> {
    if v.len() < 2 {
        return None;
    }
    let var = v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (v.len() - 1) as f64;
    if var.is_finite() && var >= 0.0 {
        Some(var.sqrt())
    } else {
        None
    }
}

fn wrap_into(t: f64, offset: f64, t_min: f64, t_max: f64) -> f64 {
    let span = t_max - t_min;
    if span <= 0.0 {
        t + offset
    } else {
        t_min + (t + offset - t_min).rem_euclid(span)
    }
}

fn series_span(series: &BTreeMap<u32, Vec<(f64, f64)>>) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for v in series.values() {
        for &(t, _) in v {
            if t < lo {
                lo = t;
            }
            if t > hi {
                hi = t;
            }
        }
    }
    if lo.is_finite() && hi.is_finite() && hi >= lo {
        Some((lo, hi))
    } else {
        None
    }
}

fn report_series(
    label: &str,
    stations: Vec<(String, Vec<(f64, f64)>)>,
    epochs: &[f64],
    t_min: f64,
    t_max: f64,
    window_s: f64,
    control_s: f64,
) {
    let span = t_max - t_min;
    for (name, mut pts) in stations {
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut responses: Vec<f64> = Vec::new();
        let mut controls: Vec<f64> = Vec::new();
        for &t in epochs {
            if let Some(r) = response_at(&pts, t, window_s) {
                responses.push(r);
            }
            if let Some(c) = response_at(&pts, t + control_s, window_s) {
                controls.push(c);
            }
        }
        let mut surrogate: Vec<f64> = Vec::new();
        if span > 0.0 {
            for &off_h in &SURROGATE_OFFSETS_H {
                let off = off_h * HOUR;
                for &t in epochs {
                    let shifted = wrap_into(t, off, t_min, t_max);
                    if let Some(r) = response_at(&pts, shifted, window_s) {
                        surrogate.push(r);
                    }
                }
            }
        }
        let n = responses.len();
        let mean_res = finite_mean(&responses);
        let med_res = median(&mut responses);
        let med_ctl = median(&mut controls);
        let ratio = match (med_res, med_ctl) {
            (Some(r), Some(c)) if c > 0.0 => Some(r / c),
            _ => None,
        };
        let med_sur = median(&mut surrogate);
        let mean_sur = finite_mean(&surrogate);
        let sigma_sur = match mean_sur {
            Some(m) => stddev(&surrogate, m),
            None => None,
        };
        print!("{label} {name}: n={n}");
        match med_res {
            Some(v) => print!(", median_response={v:.6}"),
            None => print!(", median_response void"),
        }
        match mean_res {
            Some(v) => print!(", mean_response={v:.6}"),
            None => print!(", mean_response void"),
        }
        match med_ctl {
            Some(v) => print!(", median_control={v:.6}"),
            None => print!(", median_control void"),
        }
        match ratio {
            Some(v) => print!(", ratio={v:.6}"),
            None => print!(", ratio void"),
        }
        print!("; surrogate null (provisional, measurement not claim):");
        match med_sur {
            Some(v) => print!(" median={v:.6}"),
            None => print!(" median void"),
        }
        match (mean_sur, sigma_sur) {
            (Some(m), Some(s)) => print!(", mean={m:.6}, sigma={s:.6}"),
            (Some(m), None) => print!(", mean={m:.6}, sigma void"),
            _ => print!(", mean void"),
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mcon_path = match arg_after(&args, "--mcon") {
        Some(v) => v.to_string(),
        None => default_mcon().to_string(),
    };
    let gic_arg = arg_after(&args, "--gic").unwrap_or(DEFAULT_GIC);
    let gic2_arg = arg_after(&args, "--gic2").unwrap_or(DEFAULT_GIC2);
    let window_min: f64 = arg_after(&args, "--window")
        .and_then(|v| v.parse().ok())
        .unwrap_or(30.0);
    let control_shift_h: f64 = arg_after(&args, "--control-shift")
        .and_then(|v| v.parse().ok())
        .unwrap_or(6.0);
    let window_s = window_min * 60.0;
    let control_s = control_shift_h * HOUR;

    let mcon_bytes = match read_bytes(&mcon_path) {
        Some(b) => b,
        None => {
            println!("MCON: no bytes at {mcon_path} — the relation stays pending");
            std::process::exit(0);
        }
    };
    let onsets = match parse_mcon(&mcon_bytes) {
        Some(o) => o,
        None => {
            println!("MCON: bytes at {mcon_path} do not parse — the relation stays pending");
            std::process::exit(0);
        }
    };
    let total = onsets.len();
    let mut epochs: Vec<f64> = Vec::new();
    for e in &onsets {
        if let Some(u) = unix_from_jd(e.jd_utc) {
            epochs.push(u);
        }
    }
    epochs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let with_epoch = epochs.len();
    println!("MCON onsets: {total} total, {with_epoch} with finite epoch");
    match (epochs.first(), epochs.last()) {
        (Some(lo), Some(hi)) => println!(
            "onset range: {} .. {}",
            iso(lo - J2000_UNIX_OFFSET),
            iso(hi - J2000_UNIX_OFFSET)
        ),
        _ => println!("onset range: absent"),
    }

    let gic_bytes = match read_bytes(gic_arg) {
        Some(b) => b,
        None => {
            println!("GIC: no bytes at {gic_arg} — the relation stays pending");
            std::process::exit(0);
        }
    };
    let recs = match parse_geo(MAGIC_GIC, &gic_bytes) {
        Some(r) => r,
        None => {
            println!("GIC: bytes carry no GIC1 records — the relation stays pending");
            std::process::exit(0);
        }
    };
    let series = station_series(&recs);
    let station_count = series.len();
    let gic_count: usize = series.values().map(|v| v.len()).sum();
    if gic_count == 0 {
        println!("GIC: 0 finite records — the relation stays pending");
        std::process::exit(0);
    }
    let mut t_min = f64::INFINITY;
    let mut t_max = f64::NEG_INFINITY;
    for v in series.values() {
        for &(t, _) in v {
            if t < t_min {
                t_min = t;
            }
            if t > t_max {
                t_max = t;
            }
        }
    }
    println!("GIC records: {gic_count} across {station_count} station(s)");
    println!(
        "GIC range: {} .. {}",
        iso(t_min - J2000_UNIX_OFFSET),
        iso(t_max - J2000_UNIX_OFFSET)
    );

    let in_range_epochs: Vec<f64> = epochs
        .iter()
        .copied()
        .filter(|&u| in_range(u, t_min, t_max))
        .collect();
    println!(
        "overlap: {} onsets inside the GIC span",
        in_range_epochs.len()
    );
    if in_range_epochs.len() < 10 {
        println!("overlap < 10 onsets — the relation stays pending (no fabricated value)");
        std::process::exit(0);
    }

    let mut stations: Vec<(u32, Vec<(f64, f64)>)> = series.into_iter().collect();
    if stations.len() != 1 {
        stations.retain(|(_, v)| v.len() > 100);
    }
    let named: Vec<(String, Vec<(f64, f64)>)> = stations
        .into_iter()
        .map(|(station, v)| (station.to_string(), v))
        .collect();
    report_series(
        "fmi_gic_1min",
        named,
        &in_range_epochs,
        t_min,
        t_max,
        window_s,
        control_s,
    );

    report_bpa_gic(gic2_arg, &epochs, window_s, control_s);
}

fn report_bpa_gic(gic2_arg: &str, epochs: &[f64], window_s: f64, control_s: f64) {
    let bytes = match read_bytes(gic2_arg) {
        Some(b) => b,
        None => {
            println!("bpa_gic: no bytes at {gic2_arg} — the series stays pending");
            return;
        }
    };
    let recs = match bpa_gic::parse_bin(&bytes) {
        Some(r) => r,
        None => {
            println!("bpa_gic: bytes carry no BPAG records — the series stays pending");
            return;
        }
    };
    let series = bpa_channel_series(&recs);
    for comp in 0..bpa_gic::CHANNELS as u32 {
        if !series.contains_key(&comp) {
            let name = match bpa_gic::component_name(comp) {
                Some(n) => n,
                None => "unnamed",
            };
            println!("bpa_gic channel {comp} ({name}): absent — skipped");
        }
    }
    let gic_count: usize = series.values().map(|v| v.len()).sum();
    if gic_count == 0 {
        println!("bpa_gic: 0 finite records — the series stays pending");
        return;
    }
    let (t_min, t_max) = match series_span(&series) {
        Some(s) => s,
        None => {
            println!("bpa_gic: no finite span — the series stays pending");
            return;
        }
    };
    println!(
        "bpa_gic records: {gic_count} across {} channel(s)",
        series.len()
    );
    println!(
        "bpa_gic range: {} .. {}",
        iso(t_min - J2000_UNIX_OFFSET),
        iso(t_max - J2000_UNIX_OFFSET)
    );
    let in_range_epochs: Vec<f64> = epochs
        .iter()
        .copied()
        .filter(|&u| in_range(u, t_min, t_max))
        .collect();
    println!(
        "bpa_gic overlap: {} onsets inside the span",
        in_range_epochs.len()
    );
    if in_range_epochs.len() < 10 {
        println!("bpa_gic overlap < 10 onsets — the series stays pending (no fabricated value)");
        return;
    }
    let named: Vec<(String, Vec<(f64, f64)>)> = series
        .into_iter()
        .map(|(comp, v)| {
            let name = match bpa_gic::component_name(comp) {
                Some(n) => n.to_string(),
                None => format!("channel_{comp}"),
            };
            (name, v)
        })
        .collect();
    report_series(
        "bpa_gic",
        named,
        &in_range_epochs,
        t_min,
        t_max,
        window_s,
        control_s,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::mcon_event::{
        MconEvent, PRES_ALL, REC_BYTES, encode_rec, write_header,
    };

    fn synthetic_mcon() -> Vec<u8> {
        let mut bin = Vec::new();
        write_header(&mut bin, 2);
        for jd in [2439314.5f64, 2439315.5f64] {
            let r = MconEvent {
                present: PRES_ALL,
                jd_utc: jd,
                sml: -178.0,
                amplitude: 163.0,
                bay_decrease: 134.0,
                bay_duration: 12.0,
                bay_width: 36.0,
            };
            let mut buf = [0u8; REC_BYTES];
            encode_rec(&mut buf, &r);
            bin.extend_from_slice(&buf);
        }
        bin
    }

    #[test]
    fn mcon_roundtrip_reads_two_events() {
        let bytes = synthetic_mcon();
        let events = parse_mcon(&bytes).unwrap();
        assert_eq!(events.len(), 2);
        assert!((events[0].jd_utc - 2439314.5).abs() < 1e-9);
        assert!((events[1].jd_utc - 2439315.5).abs() < 1e-9);
    }

    #[test]
    fn median_of_known_slice() {
        let mut odd = vec![3.0, 1.0, 2.0];
        assert_eq!(median(&mut odd), Some(2.0));
        let mut even = vec![4.0, 1.0, 3.0, 2.0];
        assert_eq!(median(&mut even), Some(2.5));
        let mut empty: Vec<f64> = Vec::new();
        assert_eq!(median(&mut empty), None);
    }

    #[test]
    fn in_range_filter_excludes_outside_epoch() {
        assert!(in_range(25.0, 0.0, 50.0));
        assert!(!in_range(100.0, 0.0, 50.0));
        assert!(!in_range(-1.0, 0.0, 50.0));
    }
}
