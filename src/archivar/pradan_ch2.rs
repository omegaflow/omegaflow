use crate::archivar::extract::SeriesRow;
use crate::archivar::fits::{FitsHeader, FitsTable};

pub const COMP_COUNTS: u32 = 0;
pub const FIELD_NAME: &str = "pradan_ch2_cla_l1_counts";

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_COUNTS => Some(FIELD_NAME),
        _ => None,
    }
}

fn start_unix_of(name: &str) -> Option<f64> {
    let stem = name.rsplit('/').next()?.strip_suffix(".fits")?;
    let stamp = stem.strip_prefix("ch2_cla_l1_")?.split('_').next()?;
    if stamp.len() < 17 || stamp.as_bytes().get(8) != Some(&b'T') {
        return None;
    }
    let year: i64 = stamp.get(0..4)?.parse().ok()?;
    let month: i64 = stamp.get(4..6)?.parse().ok()?;
    let day: i64 = stamp.get(6..8)?.parse().ok()?;
    let hour: i64 = stamp.get(9..11)?.parse().ok()?;
    let minute: i64 = stamp.get(11..13)?.parse().ok()?;
    let second: i64 = stamp.get(13..15)?.parse().ok()?;
    let millis: i64 = stamp.get(15..18)?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let days = crate::lsk::days_from_civil(year, month, day)?;
    Some(
        days as f64 * 86400.0
            + hour as f64 * 3600.0
            + minute as f64 * 60.0
            + second as f64
            + millis as f64 / 1000.0,
    )
}

fn spectrum_counts(fits: &[u8]) -> Option<f64> {
    let (_, mut off) = FitsHeader::parse(fits, 0)?;
    loop {
        if off + 80 > fits.len() {
            return None;
        }
        let (header, next) = FitsHeader::parse(fits, off)?;
        if header.value("XTENSION") == Some("'BINTABLE'") {
            let (table, _) = FitsTable::parse(fits, off)?;
            let counts = table.column("COUNTS")?;
            let mut sum = 0.0f64;
            for row in 0..table.n_rows {
                if let Some(v) = table.cell_f64(fits, row, counts)
                    && v.is_finite()
                {
                    sum += v;
                }
            }
            return sum.is_finite().then_some(sum);
        }
        if next <= off {
            return None;
        }
        off = next;
    }
}

pub fn parse_named_series(zip: &[u8]) -> Option<(Vec<String>, Vec<SeriesRow>)> {
    let lsk = crate::archivar::embedded_lsk()?;
    let mut rows: Vec<SeriesRow> = Vec::new();
    crate::archivar::inflate::zip_members(zip, |name, data| {
        if !name.ends_with(".fits") {
            return;
        }
        let Some(unix) = start_unix_of(name) else {
            return;
        };
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            return;
        };
        let Some(value) = spectrum_counts(data) else {
            return;
        };
        rows.push(SeriesRow {
            t: tdb,
            value,
            comp: COMP_COUNTS,
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
        });
    })?;
    if rows.is_empty() {
        None
    } else {
        Some((vec![FIELD_NAME.to_string()], rows))
    }
}

pub fn parse_series(zip: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    parse_named_series(zip)
        .map(|(_, rows)| rows.into_iter().map(|r| (r.t, r.value, r.comp)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(keyword: &str, value: &str) -> [u8; 80] {
        let mut c = [b' '; 80];
        let k = keyword.as_bytes();
        c[..k.len()].copy_from_slice(k);
        c[8] = b'=';
        c[9] = b' ';
        let v = value.as_bytes();
        let n = v.len().min(60);
        c[10..10 + n].copy_from_slice(&v[..n]);
        c
    }

    fn block(cards: &[(&str, &str)]) -> Vec<u8> {
        let mut out = Vec::new();
        for (k, v) in cards {
            out.extend_from_slice(&card(k, v));
        }
        out.extend_from_slice(&card("END", ""));
        while out.len() % 2880 != 0 {
            out.push(b' ');
        }
        out
    }

    fn spectrum_fits() -> Vec<u8> {
        let mut out = block(&[
            ("SIMPLE", "T"),
            ("BITPIX", "8"),
            ("NAXIS", "0"),
            ("EXTEND", "T"),
        ]);
        out.extend_from_slice(&block(&[
            ("XTENSION", "'BINTABLE'"),
            ("BITPIX", "8"),
            ("NAXIS", "2"),
            ("NAXIS1", "6"),
            ("NAXIS2", "2"),
            ("PCOUNT", "0"),
            ("GCOUNT", "1"),
            ("TFIELDS", "2"),
            ("TFORM1", "'1I'"),
            ("TFORM2", "'1E'"),
            ("TTYPE1", "'CHANNEL'"),
            ("TTYPE2", "'COUNTS'"),
            ("TUNIT2", "'count'"),
            ("EXTNAME", "'SPECTRUM'"),
        ]));
        out.extend_from_slice(&1i16.to_be_bytes());
        out.extend_from_slice(&2.0f32.to_be_bytes());
        out.extend_from_slice(&2i16.to_be_bytes());
        out.extend_from_slice(&3.0f32.to_be_bytes());
        while !out.len().is_multiple_of(2880) {
            out.push(0);
        }
        out
    }

    fn zip_one(name: &str, data: &[u8]) -> Vec<u8> {
        let n = name.as_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(n.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(n);
        out.extend_from_slice(data);
        let cd_off = out.len();
        out.extend_from_slice(b"PK\x01\x02");
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(n.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(n);
        let cd_size = out.len() - cd_off;
        out.extend_from_slice(b"PK\x05\x06");
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&(cd_size as u32).to_le_bytes());
        out.extend_from_slice(&(cd_off as u32).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out
    }

    #[test]
    fn reads_a_synthetic_zip_spectrum() {
        let name =
            "cla/data/calibrated/2025/10/07/ch2_cla_l1_20251007T050633821_20251007T050641821.fits";
        let zip = zip_one(name, &spectrum_fits());
        let (names, rows) = parse_named_series(&zip).expect("the zip spectrum parses");
        assert_eq!(names, vec![FIELD_NAME.to_string()]);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].comp, COMP_COUNTS);
        assert!((rows[0].value - 5.0).abs() < 1e-9);
        assert!(rows[0].t.is_finite() && rows[0].t > 0.0);
        assert_eq!(rows[0].freq, 0.0);
        assert_eq!(component_name(COMP_COUNTS), Some(FIELD_NAME));
        assert_eq!(component_name(1), None);
    }

    #[test]
    fn rejects_a_zip_without_a_fits_member() {
        let zip = zip_one("cla/data/calibrated/2025/10/07/sidecar.xml", b"<x/>");
        assert!(parse_series(&zip).is_none());
    }
}
