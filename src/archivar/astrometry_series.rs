use crate::archivar::lsk::LeapSeconds;
use crate::archivar::weberin_verdicts::{VerdictLine, VerdictWord};

pub const MAGIC: [u8; 4] = *b"AST1";
pub const HEADER_BYTES: usize = 8;

pub const UNIX_JD_OFFSET: f64 = 2440587.5;
const SECONDS_PER_DAY: f64 = 86400.0;
pub const SAMPLE_BYTES: usize = 5 * 8;

#[derive(Clone)]
pub struct AstroSample {
    pub tdb: f64,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub e_ra_mas: f64,
    pub e_dec_mas: f64,
}

#[derive(Clone)]
pub struct AstroSeries {
    pub name: String,
    pub samples: Vec<AstroSample>,
}

pub fn jd_utc_to_tdb(lsk: &LeapSeconds, jd_utc: f64) -> Option<f64> {
    if !jd_utc.is_finite() {
        return None;
    }
    let unix = (jd_utc - UNIX_JD_OFFSET) * SECONDS_PER_DAY;
    lsk.unix_to_tdb(unix)
}

fn sample_is_a_measurement(s: &AstroSample) -> bool {
    s.tdb.is_finite()
        && s.ra_deg.is_finite()
        && s.dec_deg.is_finite()
        && s.e_ra_mas.is_finite()
        && s.e_ra_mas > 0.0
        && s.e_dec_mas.is_finite()
        && s.e_dec_mas > 0.0
}

pub fn write_bin(series: &[AstroSeries]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(HEADER_BYTES + series.len() * 64);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(series.len() as u32).to_le_bytes());
    for s in series {
        let name_len = u32::try_from(s.name.len()).ok()?;
        out.extend_from_slice(&name_len.to_le_bytes());
        out.extend_from_slice(s.name.as_bytes());
        let samples_len = u32::try_from(s.samples.len()).ok()?;
        out.extend_from_slice(&samples_len.to_le_bytes());
        for sample in &s.samples {
            if !sample_is_a_measurement(sample) {
                return None;
            }
            out.extend_from_slice(&sample.tdb.to_le_bytes());
            out.extend_from_slice(&sample.ra_deg.to_le_bytes());
            out.extend_from_slice(&sample.dec_deg.to_le_bytes());
            out.extend_from_slice(&sample.e_ra_mas.to_le_bytes());
            out.extend_from_slice(&sample.e_dec_mas.to_le_bytes());
        }
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<AstroSeries>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = HEADER_BYTES;
    let mut series = Vec::with_capacity(count);
    for _ in 0..count {
        let name_len = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
        off += 4;
        let name_bytes = bytes.get(off..off + name_len)?;
        off += name_len;
        let name = std::str::from_utf8(name_bytes).ok()?.to_string();
        let samples_len = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
        off += 4;
        let mut samples = Vec::with_capacity(samples_len);
        for _ in 0..samples_len {
            let f64_at = |o: &mut usize| -> Option<f64> {
                let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
                *o += 8;
                Some(v)
            };
            let sample = AstroSample {
                tdb: f64_at(&mut off)?,
                ra_deg: f64_at(&mut off)?,
                dec_deg: f64_at(&mut off)?,
                e_ra_mas: f64_at(&mut off)?,
                e_dec_mas: f64_at(&mut off)?,
            };
            if !sample_is_a_measurement(&sample) {
                return None;
            }
            samples.push(sample);
        }
        series.push(AstroSeries { name, samples });
    }
    if off != bytes.len() {
        return None;
    }
    Some(series)
}

pub fn direction_only_lines(series: &[AstroSeries], weave_epoch: f64) -> Vec<VerdictLine> {
    series
        .iter()
        .map(|s| VerdictLine {
            name: s.name.clone(),
            word: VerdictWord::DirectionOnly,
            knot: [None, None],
            sep: None,
            weave_epoch,
        })
        .collect()
}

pub fn merge_direction_only(
    verdicts: &mut Vec<VerdictLine>,
    series: &[AstroSeries],
    weave_epoch: f64,
) -> usize {
    let mut held = 0usize;
    for line in direction_only_lines(series, weave_epoch) {
        if line.name.is_empty() {
            continue;
        }
        if verdicts.iter().any(|existing| existing.name == line.name) {
            continue;
        }
        verdicts.push(line);
        held += 1;
    }
    held
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::embedded_lsk;

    fn sample_series() -> AstroSeries {
        AstroSeries {
            name: "Camargo+2015 Uranu".to_string(),
            samples: vec![
                AstroSample {
                    tdb: 3.1e8,
                    ra_deg: 12.3456,
                    dec_deg: -4.321,
                    e_ra_mas: 0.512,
                    e_dec_mas: 0.734,
                },
                AstroSample {
                    tdb: 3.2e8,
                    ra_deg: 359.998,
                    dec_deg: 61.2,
                    e_ra_mas: 12.0,
                    e_dec_mas: 13.5,
                },
            ],
        }
    }

    #[test]
    fn bin_roundtrip_keeps_both_errors_separate() {
        let bytes = write_bin(&[sample_series()]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "Camargo+2015 Uranu");
        assert_eq!(parsed[0].samples.len(), 2);
        let s = &parsed[0].samples[0];
        assert_eq!(s.tdb, 3.1e8);
        assert_eq!(s.ra_deg, 12.3456);
        assert_eq!(s.dec_deg, -4.321);
        assert_eq!(s.e_ra_mas, 0.512);
        assert_eq!(s.e_dec_mas, 0.734);
        assert_ne!(s.e_ra_mas, s.e_dec_mas);
        assert_eq!(parsed[0].samples[1].ra_deg, 359.998);
        assert_eq!(parsed[0].samples[1].e_dec_mas, 13.5);
    }

    #[test]
    fn an_empty_asset_is_a_valid_held_state() {
        let bytes = write_bin(&[]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert!(parsed.is_empty());
        assert!(parse_bin(b"AST1").is_none());
        assert!(parse_bin(b"AST2\x00\x00\x00\x00").is_none());
    }

    #[test]
    fn bin_refuses_non_finite_or_absent_measurements() {
        let mut bad = sample_series();
        bad.samples[0].tdb = f64::NAN;
        assert!(write_bin(&[bad]).is_none());

        let mut bad = sample_series();
        bad.samples[0].ra_deg = f64::INFINITY;
        assert!(write_bin(&[bad]).is_none());

        let mut bad = sample_series();
        bad.samples[0].e_ra_mas = 0.0;
        assert!(write_bin(&[bad]).is_none());

        let mut bad = sample_series();
        bad.samples[0].e_dec_mas = -1.0;
        assert!(write_bin(&[bad]).is_none());

        let bytes = write_bin(&[sample_series()]).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        assert!(parse_bin(b"").is_none());
    }

    #[test]
    fn a_direction_series_reads_as_a_direction_only_witness_without_a_distance() {
        let lines = direction_only_lines(&[sample_series()], 3.2e8);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].name, "Camargo+2015 Uranu");
        assert_eq!(lines[0].word, VerdictWord::DirectionOnly);
        assert_eq!(lines[0].knot, [None, None]);
        assert_eq!(lines[0].sep, None);
        assert_eq!(lines[0].weave_epoch, 3.2e8);
    }

    #[test]
    fn a_direction_only_line_never_displaces_a_held_word() {
        let weave = 3.2e8;
        let mut held = vec![VerdictLine {
            name: "Camargo+2015 Uranu".to_string(),
            word: VerdictWord::Riss,
            knot: [None, None],
            sep: Some(1.0),
            weave_epoch: weave,
        }];
        let added = merge_direction_only(&mut held, &[sample_series()], weave);
        assert_eq!(added, 0);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].word, VerdictWord::Riss);

        let mut empty: Vec<VerdictLine> = Vec::new();
        let added = merge_direction_only(&mut empty, &[sample_series()], weave);
        assert_eq!(added, 1);
        assert_eq!(empty.len(), 1);
        assert_eq!(empty[0].name, "Camargo+2015 Uranu");
        assert_eq!(empty[0].word, VerdictWord::DirectionOnly);
        assert_eq!(empty[0].knot, [None, None]);
        assert_eq!(empty[0].sep, None);
        assert_eq!(empty[0].weave_epoch, weave);
    }

    #[test]
    fn jd_utc_folds_onto_the_tdb_axis_through_the_embedded_lsk() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let jd = 2451545.0;
        let tdb = jd_utc_to_tdb(&lsk, jd).expect("J2000 folds onto the TDB axis");
        let expect = lsk
            .unix_to_tdb((jd - UNIX_JD_OFFSET) * SECONDS_PER_DAY)
            .expect("the same unix epoch folds");
        assert_eq!(tdb, expect);
        assert!(tdb.is_finite());
        assert!(jd_utc_to_tdb(&lsk, f64::NAN).is_none());
    }

    #[test]
    fn sample_stride_is_five_f64() {
        assert_eq!(SAMPLE_BYTES, 40);
    }
}
