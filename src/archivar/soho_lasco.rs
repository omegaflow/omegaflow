use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};
use crate::lsk::days_from_civil;

pub const MAGIC: [u8; 4] = *b"SLCM";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 1 + 6 * 8;

pub const COMP_SPEED: u32 = 0;
pub const COMP_WIDTH: u32 = 1;
pub const COMP_PA: u32 = 2;
pub const COMP_ACCEL: u32 = 3;
pub const COMP_MASS: u32 = 4;
pub const COMP_KE: u32 = 5;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_SPEED, "soho_lasco_cme_speed_kms", "km/s"),
    (COMP_WIDTH, "soho_lasco_cme_width_deg", "deg"),
    (COMP_PA, "soho_lasco_cme_pa_deg", "deg"),
    (COMP_ACCEL, "soho_lasco_cme_accel_m_s2", "m/s2"),
    (COMP_MASS, "soho_lasco_cme_mass_g", "g"),
    (COMP_KE, "soho_lasco_cme_ke_erg", "erg"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct CmeRecord {
    pub t_unix: f64,
    pub speed_kms: Option<f64>,
    pub width_deg: Option<f64>,
    pub pa_deg: Option<f64>,
    pub accel_m_s2: Option<f64>,
    pub mass_g: Option<f64>,
    pub ke_erg: Option<f64>,
}

impl CmeRecord {
    fn values(&self) -> [Option<f64>; 6] {
        [
            self.speed_kms,
            self.width_deg,
            self.pa_deg,
            self.accel_m_s2,
            self.mass_g,
            self.ke_erg,
        ]
    }
}

fn parse_num(tok: &str) -> Option<f64> {
    let t = tok.trim_end_matches('*');
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_date_time(date: &str, time: &str) -> Option<f64> {
    let mut dp = date.split('/');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let days = days_from_civil(y, m, d)?;
    let mut tp = time.split(':');
    let h: i64 = tp.next()?.parse().ok()?;
    let mi: i64 = tp.next()?.parse().ok()?;
    let s: i64 = tp.next()?.parse().ok()?;
    Some((days * 86400 + h * 3600 + mi * 60 + s) as f64)
}

pub fn parse_text(text: &str) -> Vec<CmeRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with(|c: char| c.is_ascii_digit()) {
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 11 {
            continue;
        }
        let Some(t_unix) = parse_date_time(f[0], f[1]) else {
            continue;
        };
        out.push(CmeRecord {
            t_unix,
            speed_kms: parse_num(f[4]),
            width_deg: parse_num(f[3]),
            pa_deg: parse_num(f[2]),
            accel_m_s2: parse_num(f[8]),
            mass_g: parse_num(f[9]),
            ke_erg: parse_num(f[10]),
        });
    }
    out
}

pub fn write_bin(records: &[CmeRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        let mut present = 0u8;
        let mut vals = [0.0f64; 6];
        for (i, v) in r.values().iter().enumerate() {
            if let Some(x) = v {
                present |= 1 << i;
                vals[i] = *x;
            }
        }
        buf.push(present);
        for x in vals {
            buf.extend_from_slice(&x.to_le_bytes());
        }
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<CmeRecord>> {
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
        let present = *bytes.get(off)?;
        off += 1;
        if !t_unix.is_finite() {
            return None;
        }
        let mut vals = [None; 6];
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
        out.push(CmeRecord {
            t_unix,
            speed_kms: vals[0],
            width_deg: vals[1],
            pa_deg: vals[2],
            accel_m_s2: vals[3],
            mass_g: vals[4],
            ke_erg: vals[5],
        });
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("advective") else {
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
        for (comp, v) in [
            (COMP_SPEED, r.speed_kms),
            (COMP_WIDTH, r.width_deg),
            (COMP_PA, r.pa_deg),
            (COMP_ACCEL, r.accel_m_s2),
            (COMP_MASS, r.mass_g),
            (COMP_KE, r.ke_erg),
        ] {
            if let Some(v) = v {
                out.push((t, v, comp));
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2024/05/01  00:24:05    261     26    229     228    230    234      0.1*   1.5e+14    3.9e+28    257   Very Poor Event\n\
2024/05/03  02:48:05   Halo    360    808     851    763    791     -4.2    9.8e+15*   3.2e+31*    17\n";

    #[test]
    fn table_reads_present_columns_and_flags_absent_as_none() {
        let records = parse_text(SAMPLE);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].pa_deg, Some(261.0));
        assert_eq!(records[0].width_deg, Some(26.0));
        assert_eq!(records[0].speed_kms, Some(229.0));
        assert_eq!(records[0].mass_g, Some(1.5e14));
        assert_eq!(records[1].pa_deg, None);
        assert_eq!(records[1].mass_g, Some(9.8e15));
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_columns() {
        let records = parse_text(SAMPLE);
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn a_missing_mass_is_never_a_fabricated_zero() {
        let text = "2024/05/01  02:12:05    167      8    475     692    282      0   -120.1*   -------    -------    169   Poor Event\n";
        let records = parse_text(text);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].mass_g, None);
        assert_eq!(records[0].ke_erg, None);
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(2592000.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "soho_lasco_cme_speed_kms");
        assert_eq!(fields[0].unit, "km/s");
        assert_eq!(fields[4].name, "soho_lasco_cme_mass_g");
        assert_eq!(fields[4].unit, "g");
        assert_eq!(component_name(COMP_KE), Some("soho_lasco_cme_ke_erg"));
        assert_eq!(component_name(9), None);
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&parse_text(SAMPLE));
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
