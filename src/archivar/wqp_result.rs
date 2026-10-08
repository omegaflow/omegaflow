use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const WQP_WATER_TEMPERATURE: u32 = 0;
pub const WQP_PH: u32 = 1;
pub const WQP_DISSOLVED_OXYGEN: u32 = 2;
pub const WQP_CONDUCTIVITY: u32 = 3;
pub const WQP_NITRATE_N: u32 = 4;
pub const WQP_AMMONIA: u32 = 5;
pub const WQP_ORTHOPHOSPHATE: u32 = 6;
pub const WQP_CHLORIDE: u32 = 7;
pub const WQP_ATRAZINE: u32 = 8;
pub const WQP_METOLACHLOR: u32 = 9;

const COLUMNS: &[(u32, &str, &str, &str)] = &[
    (
        WQP_WATER_TEMPERATURE,
        "wqp_water_temperature_degc",
        "Temperature, water",
        "deg C",
    ),
    (WQP_PH, "wqp_ph", "pH", "None"),
    (
        WQP_DISSOLVED_OXYGEN,
        "wqp_dissolved_oxygen_mgl",
        "Dissolved oxygen (DO)",
        "mg/L",
    ),
    (
        WQP_CONDUCTIVITY,
        "wqp_conductivity_uscm",
        "Specific conductance",
        "uS/cm",
    ),
    (WQP_NITRATE_N, "wqp_nitrate_n_mgl", "Nitrate", "mg/L"),
    (WQP_AMMONIA, "wqp_ammonia_mgl", "Ammonia", "mg/L"),
    (
        WQP_ORTHOPHOSPHATE,
        "wqp_orthophosphate_mgl",
        "Orthophosphate",
        "mg/L",
    ),
    (WQP_CHLORIDE, "wqp_chloride_mgl", "Chloride", "mg/L"),
    (WQP_ATRAZINE, "wqp_atrazine_ugl", "Atrazine", "ug/L"),
    (
        WQP_METOLACHLOR,
        "wqp_metolachlor_ugl",
        "Metolachlor",
        "ug/L",
    ),
];

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("diffusion") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(COLUMNS.len());
    for (comp, name, _, unit) in COLUMNS {
        if component_name(*comp).is_none() {
            continue;
        }
        out.push(FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            band_id: None,
            kernel,
            force,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: (*unit).to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: crate::archivar::Aperture::None,
        });
    }
    out
}

fn plausible(comp: u32, v: f64) -> bool {
    if !v.is_finite() {
        return false;
    }
    match comp {
        WQP_PH => (0.0..=14.0).contains(&v),
        WQP_WATER_TEMPERATURE => (-100.0..=100.0).contains(&v),
        _ => v >= 0.0,
    }
}

fn utc_offset(code: &str) -> Option<f64> {
    match code {
        "" => Some(0.0),
        "EST" => Some(5.0 * 3600.0),
        "EDT" => Some(4.0 * 3600.0),
        "CST" => Some(6.0 * 3600.0),
        "CDT" => Some(5.0 * 3600.0),
        "MST" => Some(7.0 * 3600.0),
        "MDT" => Some(6.0 * 3600.0),
        "PST" => Some(8.0 * 3600.0),
        "PDT" => Some(7.0 * 3600.0),
        "AKST" => Some(9.0 * 3600.0),
        "AKDT" => Some(8.0 * 3600.0),
        "HST" => Some(10.0 * 3600.0),
        _ => None,
    }
}

fn two(s: &str) -> Option<u32> {
    s.trim().parse().ok()
}

fn epoch_tdb(date: &str, time: &str, tz: &str, lsk: &crate::archivar::LeapSeconds) -> Option<f64> {
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m = two(dp.next()?)?;
    let d = two(dp.next()?)?;
    let (hh, mm, ss) = if time.trim().is_empty() {
        (0, 0, 0)
    } else {
        let mut tp = time.split(':');
        let hh = two(tp.next()?)?;
        let mm = tp.next().map(two).unwrap_or(Some(0))?;
        let ss = tp.next().map(two).unwrap_or(Some(0))?;
        (hh, mm, ss)
    };
    let offset = utc_offset(tz)?;
    let days = crate::archivar::ymd_to_days(y, m, d)?;
    let unix = days as f64 * 86400.0 + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64 + offset;
    lsk.unix_to_tdb(unix)
}

type Conversion = fn(f64) -> f64;

fn normalize(recorded: &str, declared: &str) -> Option<Conversion> {
    if recorded == declared {
        return Some(|v| v);
    }
    match (recorded, declared) {
        ("deg F", "deg C") => Some(|f| (f - 32.0) * 5.0 / 9.0),
        ("uS/cm @25C" | "umho/cm", "uS/cm") => Some(|v| v),
        _ if declared == "None"
            && (recorded.is_empty() || recorded.eq_ignore_ascii_case("none")) =>
        {
            Some(|v| v)
        }
        _ => None,
    }
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let unzipped;
    let text_bytes: &[u8] = if bytes.starts_with(b"PK\x03\x04") {
        unzipped = crate::archivar::unzip(bytes)?;
        unzipped.as_slice()
    } else {
        bytes
    };
    let text = std::str::from_utf8(text_bytes).ok()?;
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header_line = lines.next()?;
    let headers = crate::archivar::split_csv_line(header_line);
    let idx = |name: &str| headers.iter().position(|h| h == name);
    let date_i = idx("ActivityStartDate")?;
    let time_i = idx("ActivityStartTime/Time");
    let tz_i = idx("ActivityStartTime/TimeZoneCode");
    let char_i = idx("CharacteristicName")?;
    let val_i = idx("ResultMeasureValue")?;
    let unit_i = idx("ResultMeasure/MeasureUnitCode");
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::new();
    for line in lines {
        let f = crate::archivar::split_csv_line(line);
        let at = |i: usize| f.get(i).map(String::as_str);
        let Some(characteristic) = at(char_i) else {
            continue;
        };
        let Some((comp, _, declared_unit)) = COLUMNS
            .iter()
            .find(|c| c.2 == characteristic)
            .map(|c| (c.0, c.2, c.3))
        else {
            continue;
        };
        let Some(raw) = at(val_i).and_then(|s| s.trim().parse::<f64>().ok()) else {
            continue;
        };
        let value = match unit_i.and_then(at) {
            Some(recorded_unit) => match normalize(recorded_unit, declared_unit) {
                Some(convert) => convert(raw),
                None => continue,
            },
            None => raw,
        };
        if !plausible(comp, value) {
            continue;
        }
        let Some(date) = at(date_i) else {
            continue;
        };
        let time = time_i.and_then(at).unwrap_or("");
        let tz = tz_i.and_then(at).unwrap_or("");
        if let Some(tdb) = epoch_tdb(date, time, tz, &lsk) {
            out.push((tdb, value, comp));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "OrganizationIdentifier,ActivityStartDate,ActivityStartTime/Time,ActivityStartTime/TimeZoneCode,CharacteristicName,ResultMeasureValue,ResultMeasure/MeasureUnitCode";

    #[test]
    fn a_matching_row_folds_onto_its_epoch_and_component() {
        let body = format!(
            "{HEADER}\nARS,1983-01-03,00:00:00,EST,\"Temperature, water\",5.5,deg C\nARS,1983-01-03,00:00:00,EST,Nitrate,1.2,mg/L\n"
        );
        let rows = parse_series(body.as_bytes()).expect("the csv parses");
        assert_eq!(rows.len(), 2);
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let expected = lsk
            .unix_to_tdb(
                crate::archivar::ymd_to_days(1983, 1, 3).unwrap() as f64 * 86400.0 + 5.0 * 3600.0,
            )
            .unwrap();
        assert_eq!(rows[0], (expected, 5.5, WQP_WATER_TEMPERATURE));
        assert_eq!(rows[1].2, WQP_NITRATE_N);
    }

    #[test]
    fn a_foreign_or_implausible_row_stays_absent() {
        let body = format!(
            "{HEADER}\nARS,1983-01-03,00:00:00,EST,Alachlor,2.0,ug/L\nARS,1983-01-03,00:00:00,EST,pH,21.0,None\nARS,1983-01-03,00:00:00,EST,Nitrate,1.0,ug/L\n"
        );
        assert_eq!(parse_series(body.as_bytes()), Some(Vec::new()));
    }

    #[test]
    fn an_unrecorded_zone_leaves_the_epoch_absent() {
        let body =
            format!("{HEADER}\nARS,1983-01-03,00:00:00,XYZ,\"Temperature, water\",5.5,deg C\n");
        assert_eq!(parse_series(body.as_bytes()), Some(Vec::new()));
    }

    #[test]
    fn declared_columns_carry_the_read_site_names_and_units() {
        let fields = declared_fields(604800.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "wqp_water_temperature_degc");
        assert_eq!(fields[0].unit, "deg C");
        assert_eq!(fields[8].unit, "ug/L");
    }

    #[test]
    fn a_fahrenheit_row_folds_onto_its_celsius_value() {
        let body =
            format!("{HEADER}\nARS,1983-01-03,00:00:00,EST,\"Temperature, water\",50.0,deg F\n");
        let rows = parse_series(body.as_bytes()).expect("the csv parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1, 10.0);
        assert_eq!(rows[0].2, WQP_WATER_TEMPERATURE);
    }

    #[test]
    fn a_specific_conductance_at_25c_row_survives() {
        let body = format!(
            "{HEADER}\nARS,1983-01-03,00:00:00,EST,\"Specific conductance\",412.0,uS/cm @25C\n"
        );
        let rows = parse_series(body.as_bytes()).expect("the csv parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1, 412.0);
        assert_eq!(rows[0].2, WQP_CONDUCTIVITY);
    }

    #[test]
    fn a_umho_row_survives() {
        let body = format!(
            "{HEADER}\nARS,1983-01-03,00:00:00,EST,\"Specific conductance\",318.0,umho/cm\n"
        );
        let rows = parse_series(body.as_bytes()).expect("the csv parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1, 318.0);
        assert_eq!(rows[0].2, WQP_CONDUCTIVITY);
    }

    #[test]
    fn a_foreign_temperature_unit_drops() {
        let body =
            format!("{HEADER}\nARS,1983-01-03,00:00:00,EST,\"Temperature, water\",50.0,deg K\n");
        assert_eq!(parse_series(body.as_bytes()), Some(Vec::new()));
    }
}
