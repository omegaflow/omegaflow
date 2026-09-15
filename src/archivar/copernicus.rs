use crate::geo::GeoRec;
use crate::lsk::{days_from_civil, LeapSeconds};

const DAY_S: f64 = 86400.0;

macro_rules! cdm_vars {
    ($($var:ident = $id:expr;)*) => {
        pub const VARIABLES: &[(&str, u32)] = &[ $( (stringify!($var), $id), )* ];

        pub fn component_name(comp: u32) -> Option<&'static str> {
            match comp {
                $( $id => Some(concat!("copernicus_", stringify!($var))), )*
                _ => None,
            }
        }
    };
}

cdm_vars! {
    air_pressure_at_sea_level = 1;
    air_temperature = 2;
    dew_point_temperature = 3;
    water_temperature = 4;
    wet_bulb_temperature = 5;
    wind_from_direction = 6;
    wind_speed = 7;
    accumulated_precipitation = 8;
    air_pressure = 9;
    fresh_snow = 10;
    snow_depth = 11;
    snow_water_equivalent = 12;
    daily_global_solar_radiation = 13;
    daily_maximum_air_temperature = 14;
    daily_maximum_relative_humidity = 15;
    daily_mean_air_temperature = 16;
    daily_minimum_air_temperature = 17;
    daily_minimum_relative_humidity = 18;
    maximum_soil_temperature = 19;
    maximum_solar_irradiance = 20;
    minimum_soil_temperature = 21;
    minimum_solar_irradiance = 22;
    monthly_global_solar_radiation = 23;
    relative_humidity = 24;
    soil_moisture_100cm_from_earth_surface = 25;
    soil_moisture_10cm_from_earth_surface = 26;
    soil_moisture_20cm_from_earth_surface = 27;
    soil_moisture_50cm_from_earth_surface = 28;
    soil_moisture_5cm_from_earth_surface = 29;
    soil_temperature = 30;
    soil_temperature_100cm_from_earth_surface = 31;
    soil_temperature_10cm_from_earth_surface = 32;
    soil_temperature_20cm_from_earth_surface = 33;
    soil_temperature_50cm_from_earth_surface = 34;
    soil_temperature_5cm_from_earth_surface = 35;
    solar_irradiance = 36;
    wetness = 37;
    wind_speed_2_meters_from_earth_surface = 38;
    air_dewpoint_depression = 39;
    ascent_speed = 40;
    eastward_wind_speed = 41;
    frost_point_temperature = 42;
    geopotential_height = 43;
    northward_wind_speed = 44;
    solar_zenith_angle = 45;
    water_vapour_volume_mixing_ratio = 46;
    ozone_partial_pressure = 47;
    total_column_ozone = 48;
    total_column_ozone_standard_deviation = 49;
    total_column_sulphur_dioxide = 50;
    altitude = 51;
    relative_humidity_effective_vertical_resolution = 52;
    shortwave_radiation = 53;
    time_since_launch = 54;
    vertical_speed_of_radiosonde = 55;
    air_dewpoint = 56;
    dew_point_depression = 57;
    specific_humidity = 58;
    total_column_water_vapour = 59;
    total_column_water_vapour_era5 = 60;
    zenith_total_delay = 61;
}

pub const VARIABLE_COUNT: u32 = VARIABLES.len() as u32;

pub const API: &str = "https://cds.climate.copernicus.eu/api";

pub fn comp_of(variable: &str) -> Option<u32> {
    VARIABLES
        .iter()
        .find(|(name, _)| *name == variable)
        .map(|(_, comp)| *comp)
}

fn num(field: &str) -> Option<f64> {
    let t = field.trim().trim_matches('"');
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn parse_timestamp(stamp: &str) -> Option<f64> {
    let s = stamp.trim();
    if s.len() < 19 {
        return None;
    }
    let year = s.get(0..4)?.parse::<i64>().ok()?;
    let month = s.get(5..7)?.parse::<i64>().ok()?;
    let day = s.get(8..10)?.parse::<i64>().ok()?;
    let hour = s.get(11..13)?.parse::<i64>().ok()?;
    let minute = s.get(14..16)?.parse::<i64>().ok()?;
    let second = s.get(17..19)?.parse::<i64>().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * DAY_S + (hour * 3600 + minute * 60 + second) as f64)
}

fn column(header: &[String], name: &str) -> Option<usize> {
    header.iter().position(|h| h == name)
}

fn alt_of(
    fields: &[String],
    z: Option<usize>,
    surface: Option<usize>,
    sea: Option<usize>,
) -> Option<f64> {
    let at = |idx: Option<usize>| idx.and_then(|i| fields.get(i)).and_then(|s| num(s));
    at(z).or_else(|| at(surface)).or_else(|| at(sea))
}

fn rec(t: f64, lat: f64, lon: f64, alt: f64, val: f64, comp: u32) -> Option<GeoRec> {
    if !t.is_finite()
        || !lat.is_finite()
        || !lon.is_finite()
        || !alt.is_finite()
        || !val.is_finite()
    {
        return None;
    }
    Some(GeoRec {
        t,
        lat,
        lon,
        alt,
        freq: 0.0,
        bin_width: 0.0,
        val,
        comp,
        station: 0,
    })
}

pub fn parse_cdm_obs(text: &str, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let mut out = Vec::new();
    let mut lines = text.lines();
    let mut header: Option<Vec<String>> = None;
    for line in lines.by_ref() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        header = Some(super::noaa_nodd::csv_fields(line));
        break;
    }
    let Some(header) = header else {
        return out;
    };
    let Some(i_var) = column(&header, "observed_variable") else {
        return out;
    };
    let Some(i_val) = column(&header, "observation_value") else {
        return out;
    };
    let Some(i_ts) = column(&header, "report_timestamp") else {
        return out;
    };
    let Some(i_lat) = column(&header, "latitude") else {
        return out;
    };
    let Some(i_lon) = column(&header, "longitude") else {
        return out;
    };
    let i_z = column(&header, "z_coordinate");
    let i_surface = column(&header, "observation_height_above_station_surface");
    let i_sea = column(&header, "height_of_station_above_sea_level");

    for line in lines {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let fields = super::noaa_nodd::csv_fields(line);
        let Some(comp) = fields.get(i_var).and_then(|s| comp_of(s.trim())) else {
            continue;
        };
        let Some(val) = fields.get(i_val).and_then(|s| num(s)) else {
            continue;
        };
        let Some(t) = fields
            .get(i_ts)
            .and_then(|s| parse_timestamp(s))
            .and_then(|unix| lsk.unix_to_tdb(unix))
        else {
            continue;
        };
        let Some(lat) = fields.get(i_lat).and_then(|s| num(s)) else {
            continue;
        };
        let Some(lon) = fields.get(i_lon).and_then(|s| num(s)) else {
            continue;
        };
        let Some(alt) = alt_of(&fields, i_z, i_surface, i_sea) else {
            continue;
        };
        if let Some(r) = rec(t, lat, lon, alt, val, comp) {
            out.push(r);
        }
    }
    out
}

fn json_string(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":\"", key);
    let start = body.find(&needle)? + needle.len();
    let rest = &body[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

pub fn retrieve(dataset: &str, inputs_json: &str, token: &str, ttl: u64) -> Option<Vec<u8>> {
    let headers = [("PRIVATE-TOKEN".to_string(), token.to_string())];
    let submit_headers = [
        ("PRIVATE-TOKEN".to_string(), token.to_string()),
        ("Content-Type".to_string(), "application/json".to_string()),
    ];
    let submit = super::fetch_raw(
        &format!("{API}/retrieve/v1/processes/{dataset}/execution"),
        Some(inputs_json),
        &submit_headers,
        ttl,
    )?;
    let job = json_string(&submit, "jobID")?;
    let monitor = format!("{API}/retrieve/v1/jobs/{job}");
    for _ in 0..90 {
        let status = super::fetch_raw(&monitor, None, &headers, ttl)?;
        match json_string(&status, "status").as_deref() {
            Some("successful") => {
                let results = super::fetch_raw(&format!("{monitor}/results"), None, &headers, ttl)?;
                let href = json_string(&results, "href")?;
                let archive = super::fetch_raw_bytes_headers(&href, &headers, ttl)?;
                return crate::inflate::unzip(&archive);
            }
            Some("failed") | Some("rejected") => {
                eprintln!("{dataset}: job {job} did not complete");
                return None;
            }
            _ => std::thread::sleep(std::time::Duration::from_secs(6)),
        }
    }
    eprintln!("{dataset}: job {job} did not complete within the window");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "########################################\n\
# This is a CSV file following the CDS convention cdm-obs\n\
# Variables selected and units:\n\
# air_temperature [K]\n\
########################################\n\
index,observation_id,longitude,latitude,observation_height_above_station_surface,observed_variable,observation_value,units,height_of_station_above_sea_level,report_timestamp\n\
0,ICOADS-1,-9.8,32.5,,air_temperature,291.05,K,0.0,2020-01-01 00:00:00\n\
1,ICOADS-2,-9.1,33.3,42.0,air_temperature,291.65,K,12.0,2020-01-01 01:00:00\n\
2,ICOADS-3,-9.1,33.3,,not_a_variable,1.0,K,0.0,2020-01-01 01:00:00\n\
3,ICOADS-4,-9.1,33.3,,wind_speed,7.5,m/s,0.0,2020-01-01 02:00:00\n\
4,ICOADS-5,,, ,air_temperature,1.0,K,0.0,2020-01-01 02:00:00\n";

    fn lsk() -> LeapSeconds {
        LeapSeconds {
            delta_t_a: 0.0,
            deltas: vec![(0.0, 0.0)],
        }
    }

    #[test]
    fn registry_is_bijective() {
        for (i, (name, comp)) in VARIABLES.iter().enumerate() {
            assert_eq!(comp_of(name), Some(*comp), "variable {name}");
            assert!(component_name(*comp).is_some(), "comp {comp}");
            for (other, other_comp) in &VARIABLES[i + 1..] {
                assert_ne!(name, other);
                assert_ne!(comp, other_comp);
            }
        }
        assert_eq!(comp_of("not_a_variable"), None);
        assert_eq!(component_name(0), None);
        assert_eq!(component_name(VARIABLE_COUNT + 1), None);
    }

    #[test]
    fn timestamp_reads_utc() {
        let t = parse_timestamp("2020-01-01 00:00:00").unwrap();
        assert_eq!(t, 1577836800.0);
        assert_eq!(parse_timestamp("2020-01-01 01:30:15").unwrap(), t + 5415.0);
        assert_eq!(parse_timestamp(""), None);
        assert_eq!(parse_timestamp("2020-13-01 00:00:00"), None);
    }

    #[test]
    fn parses_measured_rows_and_skips_absent() {
        let records = parse_cdm_obs(SAMPLE, &lsk());
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].comp, comp_of("air_temperature").unwrap());
        assert_eq!(records[0].val, 291.05);
        assert_eq!(records[0].alt, 0.0);
        assert_eq!(records[1].alt, 42.0);
        assert_eq!(records[2].comp, comp_of("wind_speed").unwrap());
    }

    #[test]
    fn json_string_reads_quoted_values() {
        assert_eq!(
            json_string(r#"{"jobID":"abc","status":"accepted"}"#, "jobID").as_deref(),
            Some("abc")
        );
        assert_eq!(json_string("{}", "jobID"), None);
    }
}
