use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct IagaSample {
    pub t: f64,
    pub values: Vec<Option<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IagaFile {
    pub station: u32,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub comps: Vec<u32>,
    pub samples: Vec<IagaSample>,
}

fn header_kv(line: &str) -> Option<(&str, &str)> {
    let s = line.trim_end_matches(['|', ' ']);
    if s.len() < 24 {
        return None;
    }
    Some((s.get(0..24)?.trim(), s.get(24..)?.trim()))
}

fn comp_of_letter(letter: u8) -> Option<u32> {
    match letter {
        b'X' => Some(crate::geo::COMP_IAGA_X),
        b'Y' => Some(crate::geo::COMP_IAGA_Y),
        b'Z' => Some(crate::geo::COMP_IAGA_Z),
        b'F' => Some(crate::geo::COMP_IAGA_F),
        b'H' => Some(crate::geo::COMP_IAGA_H),
        b'D' => Some(crate::geo::COMP_IAGA_D),
        _ => None,
    }
}

fn iaga_value(token: &str) -> Option<f64> {
    let whole = token.split('.').next().unwrap_or("");
    if whole == "88888" || whole == "99999" {
        return None;
    }
    token.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn header_f64(value: &str) -> Option<f64> {
    value.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn wrap_lon(lon: f64) -> f64 {
    if lon > 180.0 { lon - 360.0 } else { lon }
}

pub fn parse_text(text: &str, lsk: &LeapSeconds) -> Option<IagaFile> {
    let lines: Vec<&str> = text.lines().collect();
    let header_end = lines
        .iter()
        .position(|l| l.trim_start().starts_with("DATE"))?;
    let first = lines.iter().find(|l| !l.trim().is_empty())?;
    let (format_key, format_value) = header_kv(first)?;
    if format_key != "Format" || format_value != "IAGA-2002" {
        return None;
    }
    let mut code: Option<&str> = None;
    let mut station: Option<u32> = None;
    let mut lat: Option<f64> = None;
    let mut lon: Option<f64> = None;
    let mut alt: Option<f64> = None;
    for line in &lines[..header_end] {
        let Some((key, value)) = header_kv(line) else {
            continue;
        };
        match key {
            "IAGA CODE" => {
                code = Some(value);
                station = crate::geo::pack_iaga(value);
            }
            "Geodetic Latitude" => lat = header_f64(value),
            "Geodetic Longitude" => lon = header_f64(value),
            "Elevation" => alt = header_f64(value),
            _ => {}
        }
    }
    let (Some(code), Some(station)) = (code, station) else {
        return None;
    };
    let (Some(lat), Some(lon), Some(alt)) = (lat, lon, alt) else {
        return None;
    };
    if !(-90.0..=90.0).contains(&lat) {
        return None;
    }
    let lon = wrap_lon(lon);
    if !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    let column_line = lines[header_end].trim().trim_end_matches('|').trim_end();
    let tokens: Vec<&str> = column_line.split_whitespace().collect();
    if tokens.len() < 4 || tokens[0] != "DATE" || tokens[1] != "TIME" || tokens[2] != "DOY" {
        return None;
    }
    let mut comps = Vec::new();
    for token in &tokens[3..] {
        let b = token.as_bytes();
        if b.len() != 4 || &b[..3] != code.as_bytes() {
            return None;
        }
        comps.push(comp_of_letter(b[3])?);
    }
    let mut samples: Vec<IagaSample> = Vec::new();
    for line in &lines[header_end + 1..] {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split_whitespace().collect();
        if cells.len() != 3 + comps.len() || cells[2].parse::<u32>().is_err() {
            continue;
        }
        let iso = format!("{} {}", cells[0], cells[1]);
        let Some(t) = parse_iso_tdb(&iso, lsk) else {
            continue;
        };
        let values: Vec<Option<f64>> = cells[3..].iter().map(|c| iaga_value(c)).collect();
        samples.push(IagaSample { t, values });
    }
    if samples.is_empty() {
        return None;
    }
    Some(IagaFile {
        station,
        lat,
        lon,
        alt,
        comps,
        samples,
    })
}

pub fn to_geo_rows(file: &IagaFile) -> Vec<crate::geo::GeoRec> {
    let mut out = Vec::with_capacity(file.samples.len() * file.comps.len());
    for sample in &file.samples {
        for (comp, value) in file.comps.iter().zip(sample.values.iter()) {
            let Some(value) = value else { continue };
            out.push(crate::geo::GeoRec {
                t: sample.t,
                lat: file.lat,
                lon: file.lon,
                alt: file.alt,
                freq: crate::spectral::SPECTRAL_NO_BAND,
                bin_width: crate::spectral::SPECTRAL_NO_BAND,
                val: *value,
                comp: *comp,
                station: file.station,
            });
        }
    }
    out
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        crate::geo::COMP_IAGA_X => Some("iaga_x_nt"),
        crate::geo::COMP_IAGA_Y => Some("iaga_y_nt"),
        crate::geo::COMP_IAGA_Z => Some("iaga_z_nt"),
        crate::geo::COMP_IAGA_F => Some("iaga_f_nt"),
        crate::geo::COMP_IAGA_H => Some("iaga_h_nt"),
        crate::geo::COMP_IAGA_D => Some("iaga_d_arcmin_east"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_FCC: &str = r#"
 Format                 IAGA-2002                                    |
 Source of Data         Geological Survey of Canada (GSC)            |
 Station Name           Fort Churchill                               |
 IAGA CODE              FCC                                          |
 Geodetic Latitude      58.759                                       |
 Geodetic Longitude     265.912                                      |
 Elevation              15.000                                       |
 Reported               XYZF                                         |
 Sensor Orientation     XYZF                                         |
 Digital Sampling       8 Hz                                         |
 Data Interval Type     1-second                                     |
 Data Type              variation                                    |
 # DECBAS               000000 (Baseline declination value in        |
 #                      tenths of minutes East (0-216,000)).         |
 # This data file was created by the Ottawa GIN from Reported data.  |
 # Final data will be available on the INTERMAGNET DVD.              |
 # Go to www.intermagnet.org for details on obtaining this product.  |
 # CONDITIONS OF USE: The Conditions of Use for data provided        |
 # through INTERMAGNET and acknowledgement templates can be found    |
 # at www.intermagnet.org                                            |
DATE       TIME         DOY     FCCX      FCCY      FCCZ      FCCF   |
2023-04-23 00:00:00.000 113     10053.54   -180.70  57176.96  58030.31
2023-04-23 00:00:01.000 113     10053.74   -180.72  57177.12  88888.00
2023-04-23 00:00:02.000 113     10053.85   -180.68  57177.39  88888.00
2023-04-23 00:01:00.000 113     99999.00  99999.00  99999.00  99999.00
"#;

    #[test]
    fn parse_text_reads_the_measured_fcc_second_family() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        let file = parse_text(MEASURED_FCC, &lsk).expect("the measured IAGA-2002 text parses");
        assert_eq!(file.station, crate::geo::pack_iaga("FCC").unwrap());
        assert_eq!(file.lat, 58.759);
        assert!((file.lon + 94.088).abs() < 1e-9, "lon {}", file.lon);
        assert_eq!(file.alt, 15.0);
        assert_eq!(
            file.comps,
            vec![
                crate::geo::COMP_IAGA_X,
                crate::geo::COMP_IAGA_Y,
                crate::geo::COMP_IAGA_Z,
                crate::geo::COMP_IAGA_F,
            ]
        );
        assert_eq!(file.samples.len(), 4);
        assert_eq!(
            file.samples[0].values,
            vec![
                Some(10053.54),
                Some(-180.70),
                Some(57176.96),
                Some(58030.31)
            ]
        );
        assert_eq!(
            file.samples[1].values[..3],
            [Some(10053.74), Some(-180.72), Some(57177.12)]
        );
        assert_eq!(file.samples[1].values[3], None);
        assert_eq!(file.samples[2].values[3], None);
        assert_eq!(file.samples[3].values, vec![None, None, None, None]);
        let unix0 = crate::lsk::days_from_civil(2023, 4, 23).unwrap() as f64 * 86400.0;
        let t0 = lsk.unix_to_tdb(unix0).unwrap();
        assert!((file.samples[0].t - t0).abs() < 1e-6);
        assert_eq!(file.samples[1].t - file.samples[0].t, 1.0);
        assert_eq!(file.samples[3].t - file.samples[0].t, 60.0);
    }

    #[test]
    fn to_geo_rows_emits_one_record_per_present_component() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        let file = parse_text(MEASURED_FCC, &lsk).unwrap();
        let rows = to_geo_rows(&file);
        assert_eq!(rows.len(), 10);
        assert_eq!(rows[0].comp, crate::geo::COMP_IAGA_X);
        assert_eq!(rows[0].val, 10053.54);
        assert_eq!(rows[3].comp, crate::geo::COMP_IAGA_F);
        assert_eq!(rows[3].val, 58030.31);
        assert_eq!(rows[4].comp, crate::geo::COMP_IAGA_X);
        assert_eq!(rows[4].val, 10053.74);
        assert_eq!(rows[6].val, 57177.12);
        assert_eq!(rows[9].val, 57177.39);
        assert_eq!(rows[0].station, file.station);
        assert_eq!(rows[0].lat, 58.759);
        assert!((rows[0].lon + 94.088).abs() < 1e-9, "lon {}", rows[0].lon);
        assert_eq!(rows[0].alt, 15.0);
        assert_eq!(rows[0].freq, crate::spectral::SPECTRAL_NO_BAND);
        assert_eq!(rows[0].bin_width, crate::spectral::SPECTRAL_NO_BAND);
    }

    #[test]
    fn parse_text_refuses_foreign_and_unreadable_text() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        assert!(parse_text("", &lsk).is_none());
        assert!(
            parse_text(
                "MSTK  53.351 247.026 20230423 GEODETIC nT  1Hz\n20230423000000 13698.197  3501.444 54350.669 .\n",
                &lsk
            )
            .is_none()
        );
        assert!(
            parse_text(
                " Format                 IAGA-2002                                    |\n",
                &lsk
            )
            .is_none()
        );
        let unknown_component = MEASURED_FCC.replace("FCCF", "FCCQ");
        assert!(parse_text(&unknown_component, &lsk).is_none());
        let foreign_station = MEASURED_FCC.replace("FCCX", "ABCX");
        assert!(parse_text(&foreign_station, &lsk).is_none());
        let not_iaga = MEASURED_FCC.replace("IAGA-2002", "IAGA-2003");
        assert!(parse_text(&not_iaga, &lsk).is_none());
        let missing_lat = MEASURED_FCC.replace("58.759", "      ");
        assert!(parse_text(&missing_lat, &lsk).is_none());
    }

    #[test]
    fn component_names_name_the_measured_components() {
        assert_eq!(component_name(crate::geo::COMP_IAGA_X), Some("iaga_x_nt"));
        assert_eq!(component_name(crate::geo::COMP_IAGA_F), Some("iaga_f_nt"));
        assert_eq!(
            component_name(crate::geo::COMP_IAGA_D),
            Some("iaga_d_arcmin_east")
        );
        assert_eq!(component_name(99), None);
    }
}
