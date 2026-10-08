use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};
use crate::lsk::days_from_civil;

pub const MAGIC: [u8; 4] = *b"BPAG";
pub const HEADER_BYTES: usize = 8;
pub const CHANNELS: usize = 11;
pub const RECORD_BYTES: usize = 8 + 2 + CHANNELS * 8;

pub const COMP_EUGENE_OR: u32 = 0;
pub const COMP_SPOKANE_WA: u32 = 1;
pub const COMP_THE_DALLES_OR: u32 = 2;
pub const COMP_LEWISTON_ID: u32 = 3;
pub const COMP_KALISPELL_MT: u32 = 4;
pub const COMP_MONROE_WA: u32 = 5;
pub const COMP_WILSONVILLE_OR: u32 = 6;
pub const COMP_VANCOUVER_WA: u32 = 7;
pub const COMP_ELLENSBURG_WA: u32 = 8;
pub const COMP_BOTHELL_WA: u32 = 9;
pub const COMP_MOSES_LAKE_WA: u32 = 10;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_EUGENE_OR, "bpa_gic_eugene_or_a", "A"),
    (COMP_SPOKANE_WA, "bpa_gic_spokane_wa_a", "A"),
    (COMP_THE_DALLES_OR, "bpa_gic_the_dalles_or_a", "A"),
    (COMP_LEWISTON_ID, "bpa_gic_lewiston_id_a", "A"),
    (COMP_KALISPELL_MT, "bpa_gic_kalispell_mt_a", "A"),
    (COMP_MONROE_WA, "bpa_gic_monroe_wa_a", "A"),
    (COMP_WILSONVILLE_OR, "bpa_gic_wilsonville_or_a", "A"),
    (COMP_VANCOUVER_WA, "bpa_gic_vancouver_wa_a", "A"),
    (COMP_ELLENSBURG_WA, "bpa_gic_ellensburg_wa_a", "A"),
    (COMP_BOTHELL_WA, "bpa_gic_bothell_wa_a", "A"),
    (COMP_MOSES_LAKE_WA, "bpa_gic_moses_lake_wa_a", "A"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct BpaGicRecord {
    pub t_unix: f64,
    pub eugene_or: Option<f64>,
    pub spokane_wa: Option<f64>,
    pub the_dalles_or: Option<f64>,
    pub lewiston_id: Option<f64>,
    pub kalispell_mt: Option<f64>,
    pub monroe_wa: Option<f64>,
    pub wilsonville_or: Option<f64>,
    pub vancouver_wa: Option<f64>,
    pub ellensburg_wa: Option<f64>,
    pub bothell_wa: Option<f64>,
    pub moses_lake_wa: Option<f64>,
}

impl BpaGicRecord {
    pub fn values(&self) -> [Option<f64>; CHANNELS] {
        [
            self.eugene_or,
            self.spokane_wa,
            self.the_dalles_or,
            self.lewiston_id,
            self.kalispell_mt,
            self.monroe_wa,
            self.wilsonville_or,
            self.vancouver_wa,
            self.ellensburg_wa,
            self.bothell_wa,
            self.moses_lake_wa,
        ]
    }
}

fn parse_num(cell: Option<&str>) -> Option<f64> {
    let t = cell?.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn nth_sunday_of_month(year: i64, month: i64, n: i64) -> Option<i64> {
    let first = days_from_civil(year, month, 1)?;
    let weekday = (first + 4).rem_euclid(7);
    Some(first + (7 - weekday) % 7 + 7 * (n - 1))
}

fn pacific_utc_offset_seconds(year: i64, local_secs: i64) -> Option<i64> {
    let dst_start = nth_sunday_of_month(year, 3, 2)? * 86400 + 2 * 3600;
    let dst_end = nth_sunday_of_month(year, 11, 1)? * 86400 + 2 * 3600;
    Some(if local_secs >= dst_start && local_secs < dst_end {
        7 * 3600
    } else {
        8 * 3600
    })
}

fn parse_stamp(cell: &str) -> Option<f64> {
    let mut parts = cell.split_whitespace();
    let date = parts.next()?;
    let time = parts.next()?;
    let mut d = date.split('/');
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let year: i64 = d.next()?.parse().ok()?;
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let ss: i64 = match t.next() {
        Some(s) => s.parse().ok()?,
        None => 0,
    };
    if !(0..24).contains(&hh) || !(0..60).contains(&mm) || !(0..60).contains(&ss) {
        return None;
    }
    let days = days_from_civil(year, month, day)?;
    let local_secs = days * 86400 + hh * 3600 + mm * 60 + ss;
    let offset = pacific_utc_offset_seconds(year, local_secs)?;
    Some((local_secs + offset) as f64)
}

pub fn parse_text(text: &str) -> Vec<BpaGicRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split('\t').collect();
        let Some(t_unix) = parse_stamp(cells[0].trim()) else {
            continue;
        };
        let v = |i: usize| parse_num(cells.get(1 + i).copied());
        let rec = BpaGicRecord {
            t_unix,
            eugene_or: v(0),
            spokane_wa: v(1),
            the_dalles_or: v(2),
            lewiston_id: v(3),
            kalispell_mt: v(4),
            monroe_wa: v(5),
            wilsonville_or: v(6),
            vancouver_wa: v(7),
            ellensburg_wa: v(8),
            bothell_wa: v(9),
            moses_lake_wa: v(10),
        };
        if rec.values().iter().any(|v| v.is_some()) {
            out.push(rec);
        }
    }
    out
}

pub fn write_bin(records: &[BpaGicRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        let mut present = 0u16;
        let mut vals = [0.0f64; CHANNELS];
        for (i, v) in r.values().iter().enumerate() {
            if let Some(x) = v {
                present |= 1 << i;
                vals[i] = *x;
            }
        }
        buf.extend_from_slice(&present.to_le_bytes());
        for x in vals {
            buf.extend_from_slice(&x.to_le_bytes());
        }
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<BpaGicRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let t_unix = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let present = u16::from_le_bytes(bytes.get(off..off + 2)?.try_into().ok()?);
        off += 2;
        if !t_unix.is_finite() {
            return None;
        }
        let mut vals = [None; CHANNELS];
        for (i, slot) in vals.iter_mut().enumerate() {
            let raw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            if present & (1 << i) != 0 {
                if !raw.is_finite() {
                    return None;
                }
                *slot = Some(raw);
            }
        }
        out.push(BpaGicRecord {
            t_unix,
            eugene_or: vals[0],
            spokane_wa: vals[1],
            the_dalles_or: vals[2],
            lewiston_id: vals[3],
            kalispell_mt: vals[4],
            monroe_wa: vals[5],
            wilsonville_or: vals[6],
            vancouver_wa: vals[7],
            ellensburg_wa: vals[8],
            bothell_wa: vals[9],
            moses_lake_wa: vals[10],
        });
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("electric") else {
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
            aperture: Aperture::None,
        })
        .collect()
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_bin(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::new();
    for r in records {
        let Some(t) = lsk.unix_to_tdb(r.t_unix) else {
            continue;
        };
        for (comp, v) in r.values().iter().enumerate() {
            if let Some(v) = v {
                out.push((t, *v, comp as u32));
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "Transformer Geomagnetically Induced Current\n\
at 5-minute intervals, last 4 days\n\
Date/Time       	Eugene, OR	Spokane, WA\n\
10/03/2026 00:00	3.38175	2\n\
10/03/2026 00:05	2.231375	\n";

    #[test]
    fn header_lines_are_skipped_and_rows_parse() {
        let records = parse_text(HEADER);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].eugene_or, Some(3.38175));
        assert_eq!(records[0].spokane_wa, Some(2.0));
        assert_eq!(records[1].spokane_wa, None);
    }

    #[test]
    fn a_blank_cell_is_never_a_fabricated_zero() {
        let records = parse_text("10/03/2026 00:05	2.231375	\n");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].spokane_wa, None);
        assert_eq!(records[0].eugene_or, Some(2.231375));
    }

    #[test]
    fn a_row_with_no_measured_value_is_dropped() {
        assert!(parse_text("10/06/2026 23:55				\n").is_empty());
    }

    #[test]
    fn a_zero_is_a_real_measurement() {
        let records = parse_text("10/03/2026 00:15	0	0\n");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].eugene_or, Some(0.0));
        assert_eq!(records[0].spokane_wa, Some(0.0));
    }

    #[test]
    fn october_is_pacific_daylight_time() {
        let records = parse_text("10/03/2026 00:00	1\n");
        let days = days_from_civil(2026, 10, 3).unwrap();
        assert_eq!(records[0].t_unix, (days * 86400 + 7 * 3600) as f64);
    }

    #[test]
    fn january_is_pacific_standard_time() {
        let records = parse_text("01/15/2026 12:30	1\n");
        let days = days_from_civil(2026, 1, 15).unwrap();
        assert_eq!(
            records[0].t_unix,
            (days * 86400 + 12 * 3600 + 30 * 60 + 8 * 3600) as f64
        );
    }

    #[test]
    fn dst_boundaries_follow_the_us_rule() {
        let labels = parse_text(
            "03/08/2026 00:00	1\n03/08/2026 12:00	1\n11/01/2026 00:00	1\n11/01/2026 12:00	1\n",
        );
        let day_mar8 = days_from_civil(2026, 3, 8).unwrap();
        let day_nov1 = days_from_civil(2026, 11, 1).unwrap();
        assert_eq!(labels[0].t_unix, (day_mar8 * 86400 + 8 * 3600) as f64);
        assert_eq!(
            labels[1].t_unix,
            (day_mar8 * 86400 + 12 * 3600 + 7 * 3600) as f64
        );
        assert_eq!(labels[2].t_unix, (day_nov1 * 86400 + 7 * 3600) as f64);
        assert_eq!(
            labels[3].t_unix,
            (day_nov1 * 86400 + 12 * 3600 + 8 * 3600) as f64
        );
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_channels() {
        let records = parse_text(HEADER);
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn an_absent_channel_stays_absent_through_the_roundtrip() {
        let records = parse_text(HEADER);
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[1].spokane_wa, None);
        assert_eq!(parsed[1].eugene_or, Some(2.231375));
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(3600.0);
        assert_eq!(fields.len(), CHANNELS);
        assert_eq!(fields[0].name, "bpa_gic_eugene_or_a");
        assert_eq!(fields[0].unit, "A");
        assert_eq!(fields[0].force, force_id_of("electric").unwrap());
        assert_eq!(fields[10].name, "bpa_gic_moses_lake_wa_a");
        assert_eq!(
            component_name(COMP_MOSES_LAKE_WA),
            Some("bpa_gic_moses_lake_wa_a")
        );
        assert_eq!(component_name(11), None);
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&parse_text(HEADER));
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
