use omegaflow::archivar::{jstr, parse_json, JsonVal};
use std::collections::HashMap;
use std::process::Command;

pub fn curl_text(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-g")
        .arg("-m")
        .arg("60")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

pub fn tstr_minutes(t: &str) -> Option<f64> {
    let hhmm = t.split(' ').nth(1)?;
    let mut it = hhmm.split(':');
    let h: f64 = it.next()?.parse().ok()?;
    let m: f64 = it.next()?.parse().ok()?;
    Some(h * 60.0 + m)
}

pub fn series(text: &str, key: &str) -> Option<Vec<(f64, f64)>> {
    let json = parse_json(text)?;
    let arr = match &json {
        JsonVal::Obj(map) => map.get(key),
        _ => None,
    }?;
    let JsonVal::Arr(items) = arr else {
        return None;
    };
    let mut out = Vec::new();
    for item in items {
        let t = jstr(item, "t")?;
        let v = jstr(item, "v")?;
        let (Some(min), Ok(val)) = (tstr_minutes(&t), v.parse::<f64>()) else {
            continue;
        };
        out.push((min, val));
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

pub fn anomaly(measured: &[(f64, f64)], predicted: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let pred: HashMap<i64, f64> = predicted
        .iter()
        .map(|&(m, v)| (m.round() as i64, v))
        .collect();
    measured
        .iter()
        .filter_map(|&(m, mv)| pred.get(&(m.round() as i64)).map(|&pv| (m, mv - pv)))
        .collect()
}

pub fn onset_and_peak(anom: &[(f64, f64)], quake_min: f64, threshold: f64) -> (f64, f64, f64) {
    let mut onset = f64::NAN;
    let mut peak = 0.0;
    let mut peak_t = f64::NAN;
    for &(m, a) in anom {
        if m < quake_min {
            continue;
        }
        if a.abs() > threshold && onset.is_nan() {
            onset = m;
        }
        if a.abs() > peak {
            peak = a.abs();
            peak_t = m;
        }
    }
    (onset, peak, peak_t)
}

pub fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let (a1, b1) = (lat1.to_radians(), lon1.to_radians());
    let (a2, b2) = (lat2.to_radians(), lon2.to_radians());
    let dh = (a2 - a1) / 2.0;
    let dl = (b2 - b1) / 2.0;
    let h = dh.sin() * dh.sin() + a1.cos() * a2.cos() * dl.sin() * dl.sin();
    2.0 * 6371.0 * h.sqrt().atan2((1.0 - h).sqrt())
}

pub fn fmt_hhmm(min: f64) -> String {
    if min.is_nan() {
        return "-".to_string();
    }
    let total = min.round() as i64;
    format!("{:02}:{:02}", total / 60, total % 60)
}

fn coops_url(station: &str, product: &str, begin: &str, end: &str) -> String {
    format!(
        "https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station={}&product={}&datum=MLLW&begin_date={}&end_date={}&units=metric&time_zone=gmt&format=json",
        station, product, begin, end
    )
}

pub fn water_level_url(station: &str, begin: &str, end: &str) -> String {
    coops_url(station, "water_level", begin, end)
}

pub fn predictions_url(station: &str, begin: &str, end: &str) -> String {
    coops_url(station, "predictions", begin, end)
}

pub struct StationMeasure {
    pub onset_min: f64,
    pub peak_m: f64,
    pub peak_min: f64,
    pub pre_floor: f64,
}

pub fn measure_station(
    station: &str,
    begin: &str,
    end: &str,
    quake_min: f64,
    threshold: f64,
) -> Option<StationMeasure> {
    let wl = curl_text(&water_level_url(station, begin, end))?;
    let pr = curl_text(&predictions_url(station, begin, end))?;
    let measured = series(&wl, "data")?;
    let predicted = series(&pr, "predictions")?;
    let anom = anomaly(&measured, &predicted);
    let pre_floor = anom
        .iter()
        .filter(|&&(m, _)| m < quake_min)
        .map(|&(_, a)| a.abs())
        .fold(0.0, f64::max);
    let (onset, peak, peak_t) = onset_and_peak(&anom, quake_min, threshold);
    Some(StationMeasure {
        onset_min: onset,
        peak_m: peak,
        peak_min: peak_t,
        pre_floor,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_parses_to_minutes() {
        assert_eq!(tstr_minutes("2011-03-11 05:46"), Some(346.0));
        assert_eq!(tstr_minutes("2011-03-11 00:00"), Some(0.0));
        assert_eq!(tstr_minutes("2011-03-11 23:54"), Some(1434.0));
        assert_eq!(tstr_minutes("no time"), None);
    }

    #[test]
    fn a_tide_only_trace_carries_no_onset() {
        let measured: Vec<(f64, f64)> = (0..288)
            .map(|i| {
                let m = i as f64 * 5.0;
                (m, 1.0 * (2.0 * std::f64::consts::PI * m / 720.0).sin())
            })
            .collect();
        let predicted = measured.clone();
        let flat = anomaly(&measured, &predicted);
        let (onset, peak, _) = onset_and_peak(&flat, 346.0, 0.3);
        assert!(onset.is_nan(), "a tide-only trace carries no tsunami onset");
        assert_eq!(peak, 0.0);
    }

    #[test]
    fn a_step_anomaly_is_picked_after_the_quake() {
        let mut measured = Vec::new();
        let mut predicted = Vec::new();
        for i in 0..288 {
            let m = i as f64 * 5.0;
            let tide = 0.5 * (2.0 * std::f64::consts::PI * m / 720.0).sin();
            predicted.push((m, tide));
            let wave = if m > 400.0 { 1.0 } else { 0.0 };
            measured.push((m, tide + wave));
        }
        let anom = anomaly(&measured, &predicted);
        let (onset, peak, _) = onset_and_peak(&anom, 346.0, 0.3);
        assert!(
            (onset - 400.0).abs() <= 5.0,
            "onset {onset} should sit at the injected 400-min step"
        );
        assert!((peak - 1.0).abs() < 0.01);
    }
}
