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
        | "rad" | "m/s" | "m/s2" | "m/s²" | "j" | "n m" | "v/m" | "s/m" | "ntu" | "1" | "count"
        | "cycle" | "dbm" | "cd/m2" | "cd/m²" => Some(value),
        "wm2_1au" => Some(value * 1.495978707e11 * 1.495978707e11),
        "1e-4w/m2" => Some(value * 1e-4),
        "pfu" => Some(value * 1e4),
        "pfu/mev" => Some(value * 6.241509074e16),
        "w/m^2/nm" => Some(value * 1.0e9),
        "micromolequanta/m^2/sec" => Some(value * 1.0e-6),
        "km" | "km/s" => Some(value * 1e3),
        "cm" => Some(value * 1e-2),
        "mm" | "ms" => Some(value * 1e-3),
        "d" => Some(value * 86400.0),
        "hpa" | "mb" => Some(value * 100.0),
        "decibar" | "dbar" => Some(value * 1e4),
        "npa" => Some(value * 1e-9),
        "nt" => Some(value * 1e-9),
        "gal" => Some(value * 1e-2),
        "mgal" => Some(value * 1e-5),
        "g" => Some(value * 9.80665),
        "km/h" | "kmh" => Some(value / 3.6),
        "knot" | "kt" => Some(value * 0.514444),
        "c" | "°c" | "degc" | "degree_c" => Some(value + 273.15),
        "f" | "°f" => Some((value - 32.0) * 5.0 / 9.0 + 273.15),
        "ppm" => Some(value * 1e-6),
        "ppb" => Some(value * 1e-9),
        "ppbc" => Some(value * 1e-9),
        "pct" | "%" => Some(value * 1e-2),
        "psu" => Some(value * 1e-3),
        "jy" => Some(value * 1e-26),
        "mjy" => Some(value * 1e-29),
        "ujy" => Some(value * 1e-32),
        "sfu" => Some(value * 1e-22),
        "millionths" => Some(value * 2.0 * std::f64::consts::PI * 6.957e8 * 6.957e8 * 1e-6),
        "au" => Some(value * 1.495978707e11),
        "pc" => Some(value * 3.085677581e16),
        "kpc" => Some(value * 3.085677581e19),
        "mpc" => Some(value * 3.085677581e22),
        "pc/cm3" => Some(value * 3.085677581e22),
        "ev" => Some(value * 1.602176634e-19),
        "gev" => Some(value * 1.602176634e-10),
        "gv" => Some(value * 1e9),
        "ft" => Some(value * 0.3048),
        "inch" => Some(value * 0.0254),
        "mile" | "miles" => Some(value * 1609.344),
        "nmi" => Some(value * 1852.0),
        "deg" => Some(value * std::f64::consts::PI / 180.0),
        "arcsec" => Some(value * 4.84813681109536e-6),
        "arcmin" => Some(value * 2.9088820866572e-4),
        "m_sun" => Some(value * 1.98847e30),
        "m_earth" => Some(value * 5.9722e24),
        "m_jup" => Some(value * 1.89813e27),
        "r_earth" => Some(value * 6.371e6),
        "r_jup" => Some(value * 7.1492e7),
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
        "dbhz" => Some(10.0f64.powf(value / 10.0)),
        "cpm" => Some(value * 1.0e-6 / (334.0 * 3600.0)),
        "usv/h" => Some(value * 1.0e-6 / 3600.0),
        "h" => Some(value * 3600.0),
        "nm" => Some(value * 1e-9),
        "mas" => Some(value * 4.84813681109536e-9),
        "au/d" => Some(value * 1.731456e6),
        "e22j" => Some(value * 1e22),
        "kwh/m2" => Some(value * 3.6e6),
        "ur/h" => Some(value * 2.4344e-12),
        "mol/cm2" => Some(value * 1e4),
        "mm/yr" => Some(value * 3.16881e-11),
        "cm/yr" => Some(value * 3.16881e-10),
        "mm/h" => Some(value * 2.7777778e-7),
        "yr" => Some(value * 3.15576e7),
        "mw/m2" => Some(value * 1e-3),
        "mw/m2/sr" => Some(value * 1e-3),
        "e10j" => Some(value * 1.0e10),
        "kt_tnt" => Some(value * 4.184e12),
        "kt_mass" => Some(value * 1e6),
        "gt" => Some(value * 1e12),
        "kg/m3" | "kg/m³" => Some(value),
        "1/m3" => Some(value),
        "bq/l" => Some(value * 1e3),
        "bq/m3" | "bq/m³" => Some(value),
        "tecu" => Some(value * 1e16),
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
    let a = a?;
    Some(match op {
        1 => (a + b?) * 0.5,
        2 => a - b?,
        3 => a + b?,
        4 => a.to_radians().sin(),
        5 => a.to_radians().cos(),
        _ => a + b?,
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
        .replace(['\u{b5}', '\u{3bc}'], "u")
}

pub fn unit_from_name_suffix(name: &str) -> Option<&'static str> {
    let kl = name.to_lowercase();
    if kl.contains("magnetosphere") {
        Some("nT")
    } else if kl.ends_with("_mass_kt") {
        Some("kt_mass")
    } else if kl.ends_with("_km_s") {
        Some("km/s")
    } else if kl.ends_with("_kt") {
        Some("kt")
    } else if kl.ends_with("_knots") {
        Some("knot")
    } else if kl.ends_with("_kmh") {
        Some("km/h")
    } else if kl.ends_with("_inches") || kl.ends_with("_inch") {
        Some("inch")
    } else if kl.ends_with("_miles") {
        Some("mile")
    } else if kl.ends_with("_degrees") {
        Some("deg")
    } else if kl.ends_with("_celsius") {
        Some("C")
    } else if kl.ends_with("_f") {
        Some("f")
    } else if kl.ends_with("_count") {
        Some("count")
    } else if kl.ends_with("_umol_kg") || kl.ends_with("_umolkg") {
        Some("micromole/kg")
    } else if kl.ends_with("_mg_m3") || kl.ends_with("_mgm3") {
        Some("mg/m3")
    } else if kl.ends_with("_ug_m3") || kl.ends_with("_ugm3") {
        Some("ug/m3")
    } else if kl.ends_with("_jm2") || kl.ends_with("_j_m2") {
        Some("J/m2")
    } else if kl.ends_with("_percent") || kl.ends_with("_pct") {
        Some("%")
    } else if kl.ends_with("_ppmv") {
        Some("ppmv")
    } else if kl.ends_with("_ppb") {
        Some("ppb")
    } else if kl.ends_with("_ppm") {
        Some("ppm")
    } else if kl.ends_with("_ppt") {
        Some("ppt")
    } else if kl.ends_with("_mhz") {
        Some("MHz")
    } else if kl.ends_with("_kelvin") {
        Some("K")
    } else if kl.ends_with("_m_s2") {
        Some("m/s2")
    } else if kl.ends_with("_m_s") {
        Some("m/s")
    } else if kl.ends_with("_percc") {
        Some("cm-3")
    } else if kl.ends_with("_kms") {
        Some("km/s")
    } else if kl.ends_with("_cm3") {
        Some("cm-3")
    } else if kl.ends_with("_m3") {
        Some("1/m3")
    } else if kl.ends_with("_npa") {
        Some("nPa")
    } else if kl.ends_with("_ncc") {
        Some("cm-3")
    } else if kl.ends_with("_nt") {
        Some("nT")
    } else if kl.ends_with("_wm2nm") {
        Some("w/m^2/nm")
    } else if kl.ends_with("_wm2") || kl.ends_with("_w_m2") {
        Some("W/m2")
    } else if kl.ends_with("_arcsec") {
        Some("arcsec")
    } else if kl.ends_with("_arcmin") {
        Some("arcmin")
    } else if kl.ends_with("_mjy") {
        Some("mJy")
    } else if kl.ends_with("_jy") {
        Some("Jy")
    } else if kl.ends_with("_msun") || kl.ends_with("_solar") {
        Some("m_sun")
    } else if kl.ends_with("_hpa") {
        Some("hPa")
    } else if kl.ends_with("_dbar") {
        Some("dbar")
    } else if kl.ends_with("_mb") {
        Some("mb")
    } else if kl.ends_with("_au") {
        Some("au")
    } else if kl.ends_with("_days") {
        Some("d")
    } else if kl.ends_with("_sec") {
        Some("s")
    } else if kl.ends_with("_deg") {
        Some("deg")
    } else if kl.ends_with("_psu") {
        Some("psu")
    } else if kl.ends_with("_magnitude") {
        Some("mag")
    } else if kl.contains("tecu") {
        Some("tecu")
    } else if kl.ends_with("_du") || kl.contains("dobson") {
        Some("du")
    } else if kl.ends_with("_vm") {
        Some("V/m")
    } else if kl.ends_with("_v") {
        Some("V")
    } else if kl.ends_with("_mag") {
        Some("mag")
    } else if kl.ends_with("_mm") {
        Some("mm")
    } else if kl.ends_with("_cm") {
        Some("cm")
    } else if kl.ends_with("_km") {
        Some("km")
    } else if kl.ends_with("_c") {
        Some("C")
    } else if kl.ends_with("_k") {
        Some("K")
    } else if kl.ends_with("_au_day") {
        Some("au/d")
    } else if kl.ends_with("_hours") {
        Some("h")
    } else if kl.ends_with("_angstrom") {
        Some("1")
    } else if kl.ends_with("earth_radius") {
        Some("r_earth")
    } else if kl.ends_with("signal_to_noise") {
        Some("1")
    } else if kl.ends_with("semi_amplitude") {
        Some("m/s")
    } else if kl.ends_with("_10e22j") {
        Some("e22j")
    } else if kl.ends_with("_kwh_m2") {
        Some("kwh/m2")
    } else if kl.ends_with("_ur_h") {
        Some("ur/h")
    } else if kl.ends_with("_mol_cm2") {
        Some("mol/cm2")
    } else if kl.ends_with("_mm_yr") {
        Some("mm/yr")
    } else if kl.ends_with("_cm_yr") {
        Some("cm/yr")
    } else if kl.ends_with("_yr") {
        Some("yr")
    } else if kl.ends_with("_nm") {
        Some("nm")
    } else if kl.ends_with("_mas") {
        Some("mas")
    } else if kl.ends_with("_m") {
        Some("m")
    } else {
        None
    }
}

pub fn allowed_units_for_force(force: u8) -> &'static [&'static str] {
    match force {
        0 => &[
            "w",
            "w/m2",
            "mw/m2/sr",
            "t",
            "nt",
            "nt/hz**1/2",
            "ev",
            "gev",
            "gv",
            "jy",
            "mjy",
            "ujy",
            "jy_km/s",
            "hz",
            "dbhz",
            "dbm",
            "m",
            "km",
            "mag",
            "pc/cm3",
            "ph/s/cm2",
            "erg/cm2",
            "k.m/s",
            "crab",
            "cpm",
            "count",
            "usv/h",
            "e10j",
            "kt_tnt",
            "sfu",
            "1/cm3",
            "1/m3",
            "bq/l",
            "bq/m3",
            "tecu",
            "wm2_1au",
            "1e-4w/m2",
            "1",
            "%",
            "pfu",
            "pfu/mev",
            "w/m^2/nm",
            "micromolequanta/m^2/sec",
            "rad",
            "deg",
            "cycle",
            "m-2.s-1.tev-1",
            "tev",
            "s",
            "d",
            "ms",
            "j",
            "arcsec",
            "arcmin",
            "mile",
            "nmi",
            "mhz",
            "ppt",
            "nm",
            "h",
            "yr",
            "ur/h",
            "kwh/m2",
            "mw/m2",
        ],
        1 => &[
            "m/s2", "m/s", "gal", "mgal", "kg", "m_sun", "m_earth", "m_jup", "au", "pc", "kpc",
            "mpc", "t", "nt", "m", "ft", "r_earth", "r_jup", "gt", "logg", "deg", "arcsec", "mas",
            "ms", "s", "au/d", "cm/yr", "1",
        ],
        2 => &[
            "pa", "hpa", "npa", "m", "mm", "hz", "m/s", "s", "deg", "rad", "db", "count", "dbar",
            "inch", "mm/yr", "mm/h",
        ],
        3 => &[
            "m", "mm", "km", "m/s2", "n·m", "gal", "pa", "hz", "mw", "mm/yr", "count", "g",
        ],
        4 => &["m", "mm", "cm", "km", "pa", "m/s", "mw"],
        5 => &[
            "k", "c", "f", "degc", "degree_c", "w/m2", "w", "j", "j/m2", "mw", "%", "km/s", "e22j",
        ],
        6 => &[
            "ppm",
            "ppb",
            "ppbc",
            "ppmv",
            "mg/m3",
            "ug/m3",
            "mg/kg",
            "psu",
            "ntu",
            "%",
            "pct",
            "hpa",
            "uatm",
            "du",
            "cm-3",
            "1/cm3",
            "kg/m3",
            "kg",
            "micromole/kg",
            "m-1",
            "cm",
            "mm",
            "kt_mass",
            "mol/cm2",
            "1",
        ],
        7 => &[
            "m/s", "km/h", "km/s", "cm/s", "knot", "kt", "m3/s", "cfs", "pa", "hpa", "mb", "m",
            "decibar", "npa", "deg",
        ],
        8 => &[
            "v/m", "v", "a", "s/m", "ua/m2", "mv/m", "us/cm", "m/s", "1", "%",
        ],
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
    if days < 0 { None } else { Some(days as u64) }
}

pub fn is_unit_name(name: &str) -> bool {
    let kl = name.to_lowercase();
    kl == "degt"
        || kl == "m/s"
        || kl == "sec"
        || kl == "hpa"
        || kl == "degc"
        || kl == "degree_c"
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

pub fn cf_time_unix_seconds(units: &str, value: f64) -> Option<f64> {
    let (unit, epoch) = units.split_once(" since ")?;
    let factor = match unit.trim().to_ascii_lowercase().as_str() {
        "day" | "days" => 86400.0,
        "hour" | "hours" => 3600.0,
        "minute" | "minutes" => 60.0,
        "second" | "seconds" => 1.0,
        _ => return None,
    };
    let mut fields = epoch.split_whitespace();
    let date = fields.next()?;
    let mut date_parts = date.split('-');
    let year = date_parts.next()?.parse::<i64>().ok()?;
    let month = date_parts.next()?.parse::<i64>().ok()?;
    let day = date_parts.next()?.parse::<i64>().ok()?;
    if date_parts.next().is_some() {
        return None;
    }
    let day_seconds = crate::lsk::days_from_civil(year, month, day)? as f64 * 86400.0;
    let clock = match fields.next() {
        Some(token) => clock_seconds(token)?,
        None => 0.0,
    };
    Some(day_seconds + clock + value * factor)
}

fn clock_seconds(token: &str) -> Option<f64> {
    let parts: Vec<&str> = token.trim_end_matches('Z').split(':').collect();
    if parts.is_empty() || parts.len() > 3 {
        return None;
    }
    let mut seconds = 0.0;
    for (i, part) in parts.iter().enumerate() {
        let unit = match i {
            0 => 3600.0,
            1 => 60.0,
            _ => 1.0,
        };
        seconds += part.parse::<f64>().ok()? * unit;
    }
    Some(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cf_time_seconds_resolves_days_and_clock() {
        let base = crate::lsk::days_from_civil(1800, 1, 1).unwrap() as f64 * 86400.0;
        let godas = cf_time_unix_seconds("days since 1800-01-01 00:00:0.0", 81814.0).unwrap();
        assert_eq!(godas, base + 81814.0 * 86400.0);

        let gbase = crate::lsk::days_from_civil(1850, 1, 1).unwrap() as f64 * 86400.0;
        let gistemp = cf_time_unix_seconds("days since 1850-01-01 00:00:00", 14.0).unwrap();
        assert_eq!(gistemp, gbase + 14.0 * 86400.0);

        let hbase =
            crate::lsk::days_from_civil(2000, 1, 1).unwrap() as f64 * 86400.0 + 6.0 * 3600.0;
        let hours = cf_time_unix_seconds("hours since 2000-01-01 06:00:00", 2.0).unwrap();
        assert_eq!(hours, hbase + 2.0 * 3600.0);
    }

    #[test]
    fn cf_time_seconds_absent_for_foreign_units_and_dates() {
        assert_eq!(cf_time_unix_seconds("furlongs since 1800-01-01", 1.0), None);
        assert_eq!(cf_time_unix_seconds("days", 1.0), None);
        assert_eq!(cf_time_unix_seconds("days since 1800-13-01", 1.0), None);
    }

    #[test]
    fn kt_is_knots_and_kt_mass_is_the_mass_unit() {
        assert_eq!(convert_to_si(1.0, "kt"), Some(0.514444));
        assert_eq!(convert_to_si(1.0, "kt_mass"), Some(1e6));
        assert_eq!(
            unit_from_name_suffix("so2_emission_mass_kt"),
            Some("kt_mass")
        );
        assert_eq!(unit_from_name_suffix("so2_emission_kt"), Some("kt"));
        assert!(!allowed_units_for_force(6).contains(&normalize_unit("kt").as_str()));
        assert!(allowed_units_for_force(6).contains(&normalize_unit("kt_mass").as_str()));
    }
}
