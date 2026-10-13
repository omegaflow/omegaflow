use crate::lsk::{LeapSeconds, days_from_civil};

pub const COMP_MERID: u32 = 1;
pub const COMP_ZONAL: u32 = 2;

const COLUMNS: usize = 10;
const SECONDS_PER_DAY: f64 = 86400.0;
const SECONDS_PER_HOUR: f64 = 3600.0;
const MERID_UT: usize = 1;
const MERID_WIND: usize = 2;
const ZONAL_UT: usize = 5;
const ZONAL_WIND: usize = 6;

fn epoch_tdb(yyday: i64, ut_hours: f64, lsk: &LeapSeconds) -> Option<f64> {
    let year = 2000 + yyday / 1000;
    let day_of_year = yyday % 1000;
    if !(1..=366).contains(&day_of_year)
        || !ut_hours.is_finite()
        || !(0.0..24.0).contains(&ut_hours)
    {
        return None;
    }
    let day = days_from_civil(year, 1, 1)? + (day_of_year - 1);
    lsk.unix_to_tdb(day as f64 * SECONDS_PER_DAY + ut_hours * SECONDS_PER_HOUR)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let lsk = crate::archivar::embedded_lsk()?;
    let text = String::from_utf8_lossy(bytes);
    let mut out = Vec::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < COLUMNS {
            continue;
        }
        let Ok(yyday) = fields[0].parse::<i64>() else {
            continue;
        };
        let Ok(ut_merid) = fields[MERID_UT].parse::<f64>() else {
            continue;
        };
        let Ok(merid) = fields[MERID_WIND].parse::<f64>() else {
            continue;
        };
        let Ok(ut_zonal) = fields[ZONAL_UT].parse::<f64>() else {
            continue;
        };
        let Ok(zonal) = fields[ZONAL_WIND].parse::<f64>() else {
            continue;
        };
        if merid.is_finite()
            && let Some(t) = epoch_tdb(yyday, ut_merid, &lsk)
        {
            out.push((t, merid, COMP_MERID));
        }
        if zonal.is_finite()
            && let Some(t) = epoch_tdb(yyday, ut_zonal, &lsk)
        {
            out.push((t, zonal, COMP_ZONAL));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
yyday  ut(hr)  merid (m/s) merid err  dir ut(hr)  zonal (m/s)  zonal err dir  skytemp
24001  00.36     84.89     2.82 78  00.38     68.96     2.69 69   -24.4
24001  00.40     12.39     2.00 83  00.43    141.45     2.38 87   -24.7
24001  00.78     93.67     3.00 78  00.81     66.62     3.43 69   -25.0
";

    #[test]
    fn reads_the_measured_athabasca_rows() {
        let rows = parse_series(SAMPLE.as_bytes()).expect("the measured sample parses");
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0].2, COMP_MERID);
        assert_eq!(rows[1].2, COMP_ZONAL);
        assert!((rows[0].1 - 84.89).abs() < 1e-9);
        assert!((rows[1].1 - 68.96).abs() < 1e-9);
        assert!((rows[4].1 - 93.67).abs() < 1e-9);
        assert!((rows[5].1 - 66.62).abs() < 1e-9);

        let lsk = crate::archivar::embedded_lsk()
            .expect("the embedded naif0012 table is program identity");
        let day_one = days_from_civil(2024, 1, 1).expect("2024-01-01 resolves");
        let merid_unix = day_one as f64 * SECONDS_PER_DAY + 0.36 * SECONDS_PER_HOUR;
        let zonal_unix = day_one as f64 * SECONDS_PER_DAY + 0.38 * SECONDS_PER_HOUR;
        let merid_back = lsk.tdb_to_unix(rows[0].0).expect("the merid epoch inverts");
        let zonal_back = lsk.tdb_to_unix(rows[1].0).expect("the zonal epoch inverts");
        assert!((merid_back - merid_unix).abs() < 1e-6);
        assert!((zonal_back - zonal_unix).abs() < 1e-6);
        assert!(((rows[1].0 - rows[0].0) - 72.0).abs() < 1e-9);
    }
}
