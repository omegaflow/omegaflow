use super::*;

pub fn convert_to_si(value: f64, unit: &str) -> Option<f64> {
    match unit.trim() {
        "MW" => return Some(value * 1.0e6),
        "Mw" => return Some(10.0f64.powf(1.5 * value + 9.1)),
        "M" => return None,
        _ => {}
    }
    match unit.trim().to_lowercase().as_str() {
        "" | "m" | "s" | "k" | "kg" | "pa" | "w" | "w/m2" | "w/m²" | "t" | "hz" | "v" | "a"
        | "rad" | "m/s" | "m/s2" | "m/s²" | "j" | "v/m" | "s/m" | "ntu" | "1" => Some(value),
        "wm2_1au" => Some(value * 1.495978707e11 * 1.495978707e11),
        "1e-4w/m2" => Some(value * 1e-4),
        "pfu" => Some(value * 1e4),
        "pfu/mev" => Some(value * 6.241509074e16),
        "km" | "km/s" => Some(value * 1e3),
        "cm" => Some(value * 1e-2),
        "mm" | "ms" => Some(value * 1e-3),
        "d" => Some(value * 86400.0),
        "hpa" | "mb" => Some(value * 100.0),
        "decibar" => Some(value * 1e4),
        "npa" => Some(value * 1e-9),
        "nt" => Some(value * 1e-9),
        "gal" => Some(value * 1e-2),
        "mgal" => Some(value * 1e-5),
        "km/h" | "kmh" => Some(value / 3.6),
        "knot" | "kt" => Some(value * 0.514444),
        "c" | "°c" => Some(value + 273.15),
        "ppm" => Some(value * 1e-6),
        "ppb" => Some(value * 1e-9),
        "pct" | "%" => Some(value * 1e-2),
        "psu" => Some(value * 1e-3),
        "jy" => Some(value * 1e-26),
        "mjy" => Some(value * 1e-29),
        "sfu" => Some(value * 1e-22),
        "au" => Some(value * 1.495978707e11),
        "pc" => Some(value * 3.085677581e16),
        "pc/cm3" => Some(value * 3.085677581e22),
        "ev" => Some(value * 1.602176634e-19),
        "ft" => Some(value * 0.3048),
        "deg" => Some(value * std::f64::consts::PI / 180.0),
        "arcsec" => Some(value * 4.84813681109536e-6),
        "m_sun" => Some(value * 1.98847e30),
        "m_earth" => Some(value * 5.9722e24),
        "r_earth" => Some(value * 6.371e6),
        "mg/m3" | "mg/m³" | "mg/kg" => Some(value * 1e-6),
        "ug/m3" | "ug/m³" | "µg/m3" | "µg/m³" => Some(value * 1e-9),
        "ua/m2" | "ua/m²" | "µa/m2" | "µa/m²" => Some(value * 1e-6),
        "mv/m" => Some(value * 1e-3),
        "us/cm" => Some(value * 1e-4),
        "uatm" => Some(value * 0.101325),
        "erg/cm2" => Some(value * 1e-3),
        "m3/s" | "m³/s" => Some(value),
        "cfs" => Some(value * 0.028316846592),
        "n/cc" | "cm-3" | "1/cm3" => Some(value * 1e6),
        "du" => Some(value * 2.6867e20),
        "jy_km/s" => Some(value * 1e-23),
        "crab" => Some(value * 2.4e-14),
        "logg" => Some(10.0f64.powf(value) * 0.01),
        "cpm" => Some(value * 1.0e-6 / (334.0 * 3600.0)),
        "e10j" => Some(value * 1.0e10),
        "kt_tnt" => Some(value * 4.184e12),
        _ => None,
    }
}

pub fn register_unconverted_unit(unit: &str, name: &str) {
    static REPORTED: std::sync::Mutex<Option<std::collections::HashSet<String>>> =
        std::sync::Mutex::new(None);
    let mut guard = match REPORTED.lock() {
        Ok(g) => g,
        Err(_) => return,
    };
    let set = guard.get_or_insert_with(std::collections::HashSet::new);
    if set.insert(unit.to_string()) {
        eprintln!(
            "unit \"{}\" unconverted — SI absent; samples like \"{}\" stay unmanifested (pending curation)",
            unit, name
        );
    }
}

pub fn fold_value(a: Option<f64>, b: Option<f64>, op: u8) -> Option<f64> {
    let (a, b) = (a?, b?);
    Some(match op {
        1 => (a + b) * 0.5,
        2 => a - b,
        _ => a + b,
    })
}

pub fn is_moment_magnitude(t: &str) -> bool {
    matches!(
        t.trim().to_ascii_lowercase().as_str(),
        "mw" | "mww" | "mwc" | "mwb" | "mwr" | "mwp" | "mwpd" | "mi"
    )
}

#[derive(Clone)]
pub struct Anomaly {
    pub category: &'static str,
    pub url: String,
    pub details: String,
}

pub static ANOMALIES: std::sync::Mutex<Vec<Anomaly>> = std::sync::Mutex::new(Vec::new());

thread_local! {
    pub static ANOMALY_COLLECT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn report_anomaly(category: &'static str, url: &str, details: &str) {
    if !ANOMALY_COLLECT.with(|c| c.get()) {
        return;
    }
    if let Ok(mut v) = ANOMALIES.lock() {
        v.push(Anomaly {
            category,
            url: url.to_string(),
            details: details.to_string(),
        });
    }
}

pub fn take_anomalies() -> Vec<Anomaly> {
    match ANOMALIES.lock() {
        Ok(mut v) => std::mem::take(&mut *v),
        Err(_) => Vec::new(),
    }
}

pub fn anomaly_issue_body(anomalies: &[Anomaly]) -> String {
    let mut body = String::from("| Category | URL | Details |\n|---|---|---|\n");
    for a in anomalies {
        body.push_str(&format!("| {} | {} | {} |\n", a.category, a.url, a.details));
    }
    body
}

pub fn normalize_unit(unit: &str) -> String {
    unit.trim()
        .to_lowercase()
        .replace('\u{b2}', "2")
        .replace('\u{b3}', "3")
        .replace('\u{b5}', "u")
        .replace('\u{3bc}', "u")
}

pub fn allowed_units_for_force(force: u8) -> &'static [&'static str] {
    match force {
        0 => &[
            "w", "w/m2", "t", "nt", "ev", "jy", "mjy", "jy_km/s", "hz", "m", "km", "mag", "pc/cm3",
            "erg/cm2", "crab", "cpm", "e10j", "kt_tnt", "sfu", "1/cm3", "wm2_1au", "1e-4w/m2", "1",
            "pfu", "pfu/mev",
        ],
        1 => &[
            "m/s2", "gal", "mgal", "kg", "m_sun", "m_earth", "au", "pc", "t", "nt", "m", "r_earth",
            "logg", "1",
        ],
        2 => &["pa", "hpa", "m", "mm", "hz"],
        3 => &["m", "mm", "km", "m/s2", "gal", "pa", "hz", "mw"],
        4 => &["m", "mm", "cm", "km", "pa", "m/s", "mw"],
        5 => &["k", "c", "w/m2", "w", "j", "mw"],
        6 => &[
            "ppm", "ppb", "mg/m3", "ug/m3", "mg/kg", "psu", "ntu", "%", "pct", "hpa", "uatm", "du",
            "cm-3", "1/cm3",
        ],
        7 => &[
            "m/s", "km/h", "km/s", "knot", "kt", "m3/s", "cfs", "pa", "hpa", "mb", "m", "decibar",
            "npa",
        ],
        8 => &["v/m", "v", "a", "s/m", "ua/m2", "mv/m", "us/cm"],
        _ => &[],
    }
}

pub fn report_physics_mismatch(force: u8, unit: &str, key: &str, url: &str) {
    if !allowed_units_for_force(force).contains(&normalize_unit(unit).as_str()) {
        report_anomaly(
            "Physics Mismatch",
            url,
            &format!("field {}: unit \"{}\" not in force registry", key, unit),
        );
    }
}

pub fn ymd_to_days(year: i64, month: u32, day: u32) -> Option<u64> {
    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };
    let a = y / 100;
    let b = 2 - a + a / 4;
    let jdn =
        (365.25 * (y + 4716) as f64) as i64 + (30.6001 * (m + 1) as f64) as i64 + day as i64 + b
            - 1524;
    let days = jdn - 2440588;
    if days < 0 {
        None
    } else {
        Some(days as u64)
    }
}

pub fn is_unit_name(name: &str) -> bool {
    let kl = name.to_lowercase();
    kl == "degt"
        || kl == "m/s"
        || kl == "sec"
        || kl == "hpa"
        || kl == "degc"
        || kl == "nmi"
        || kl == "ft"
        || kl == "m"
        || kl == "s"
        || kl == "cm"
        || kl == "mm"
        || kl == "km"
        || kl == "in"
        || kl == "inhg"
        || kl == "mb"
        || kl == "mbar"
        || kl == "kt"
        || kl == "mph"
        || kl == "knots"
        || kl == "m/sec"
        || kl == "deg"
}

pub fn days_to_ymd(total_days: u64) -> (u32, u32, u32) {
    let mut d = total_days as u32;
    let mut y = 1970u32;
    loop {
        let yd = if is_leap(y) { 366 } else { 365 };
        if d < yd {
            break;
        }
        d -= yd;
        y += 1;
    }
    let months: [u32; 12] = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0u32;
    while d >= months[m as usize] {
        d -= months[m as usize];
        m += 1;
    }
    (y, m + 1, d + 1)
}
