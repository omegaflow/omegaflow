use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const COMP_AMP: u32 = 0;

const COLUMNS: &[(u32, &str, &str)] = &[(COMP_AMP, "vlf_awesome_narrowband_amp", "db")];

fn parse_num(cell: &str) -> Option<f64> {
    let t = cell.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    COLUMNS
        .iter()
        .map(|(_, name, unit)| FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            kernel,
            force,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: (*unit).to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: Aperture::None,
        })
        .collect()
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut cells = line.split(',');
        let Some(unix) = cells.next().and_then(parse_num) else {
            continue;
        };
        let Some(amplitude_db) = cells.next().and_then(parse_num) else {
            continue;
        };
        let Some(t) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        out.push((t, amplitude_db, COMP_AMP));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSV: &str = "unix,amplitude_db,station,transmitter,channel,latitude,longitude,altitude\n\
1420070700,32.0,EACF,NAA,N/S,50.0,10.0,100.0\n\
1420070701,,EACF,NAA,N/S,50.0,10.0,100.0\n\
1420070702,0.0,EACF,NAA,N/S,50.0,10.0,100.0\n";

    #[test]
    fn rows_carry_the_amplitude_at_the_unix_stamp() {
        let series = parse_series(CSV.as_bytes()).expect("csv parses");
        assert_eq!(series.len(), 2);
        assert_eq!(series[0].2, COMP_AMP);
        assert_eq!(series[0].1, 32.0);
        assert_eq!(series[1].1, 0.0);
    }

    #[test]
    fn an_absent_amplitude_is_never_a_fabricated_zero() {
        let series =
            parse_series(b"unix,amplitude_db\n1420070700,\n1420070701,7.5\n").expect("csv parses");
        assert_eq!(series.len(), 1);
        assert_eq!(series[0].1, 7.5);
    }

    #[test]
    fn the_stamp_converts_through_the_leap_second_table() {
        let series = parse_series(b"unix,amplitude_db\n1420070700,1.5\n").expect("csv parses");
        assert_eq!(series.len(), 1);
        assert!(series[0].0 > 1_420_070_700.0);
    }

    #[test]
    fn component_and_field_carry_the_read_site() {
        assert_eq!(component_name(COMP_AMP), Some("vlf_awesome_narrowband_amp"));
        assert_eq!(component_name(1), None);
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "vlf_awesome_narrowband_amp");
        assert_eq!(fields[0].unit, "db");
        assert_eq!(fields[0].force, force_id_of("em").unwrap());
    }
}
