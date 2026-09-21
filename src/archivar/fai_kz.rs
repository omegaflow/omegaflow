pub const MAGIC: [u8; 4] = *b"FAI1";
pub const RECORD_STRIDE_BYTES: usize = 57;

pub const PRESENCE_EM: u8 = 0x01;
pub const PRESENCE_EXPTIME: u8 = 0x02;

pub const TAP_SYNC: &str = "https://dachs.fai.kz/tap/sync";
pub const OBSCORE_TABLE: &str = "ivoa.obscore";
pub const SELECT_COLUMNS: &str = "s_ra, s_dec, t_min, t_max, t_exptime, em_min, em_max";

pub const MJD_UNIX_EPOCH: f64 = 40587.0;
pub const SECONDS_PER_DAY: f64 = 86400.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObsRecord {
    pub s_ra_deg: f64,
    pub s_dec_deg: f64,
    pub t_min_mjd: f64,
    pub t_max_mjd: f64,
    pub em_band_m: Option<(f64, f64)>,
    pub t_exptime_s: Option<f64>,
}

pub fn mjd_to_unix(mjd: f64) -> f64 {
    (mjd - MJD_UNIX_EPOCH) * SECONDS_PER_DAY
}

fn mandatory_valid(r: &ObsRecord) -> bool {
    r.s_ra_deg.is_finite()
        && (0.0..360.0).contains(&r.s_ra_deg)
        && r.s_dec_deg.is_finite()
        && (-90.0..=90.0).contains(&r.s_dec_deg)
        && r.t_min_mjd.is_finite()
        && r.t_max_mjd.is_finite()
        && r.t_max_mjd >= r.t_min_mjd
}

fn em_band_valid(band: &(f64, f64)) -> bool {
    band.0.is_finite() && band.1.is_finite() && band.0 > 0.0 && band.1 >= band.0
}

fn exptime_valid(s: f64) -> bool {
    s.is_finite() && s >= 0.0
}

fn valid(r: &ObsRecord) -> bool {
    if !mandatory_valid(r) {
        return false;
    }
    if r.em_band_m.as_ref().is_some_and(|band| !em_band_valid(band)) {
        return false;
    }
    if r.t_exptime_s.is_some_and(|s| !exptime_valid(s)) {
        return false;
    }
    true
}

pub fn write_bin(records: &[ObsRecord]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(8 + records.len() * RECORD_STRIDE_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        if !valid(r) {
            return None;
        }
        let mut presence: u8 = 0;
        let mut em_min = 0.0f64;
        let mut em_max = 0.0f64;
        let mut exptime = 0.0f64;
        if let Some((lo, hi)) = r.em_band_m {
            presence |= PRESENCE_EM;
            em_min = lo;
            em_max = hi;
        }
        if let Some(s) = r.t_exptime_s {
            presence |= PRESENCE_EXPTIME;
            exptime = s;
        }
        out.push(presence);
        out.extend_from_slice(&r.s_ra_deg.to_le_bytes());
        out.extend_from_slice(&r.s_dec_deg.to_le_bytes());
        out.extend_from_slice(&r.t_min_mjd.to_le_bytes());
        out.extend_from_slice(&r.t_max_mjd.to_le_bytes());
        out.extend_from_slice(&em_min.to_le_bytes());
        out.extend_from_slice(&em_max.to_le_bytes());
        out.extend_from_slice(&exptime.to_le_bytes());
    }
    Some(out)
}

fn f64_le(bytes: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(
        bytes.get(off..off + 8)?.try_into().ok()?,
    ))
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<ObsRecord>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = 8usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        let presence = *bytes.get(off)?;
        let r = ObsRecord {
            s_ra_deg: f64_le(bytes, off + 1)?,
            s_dec_deg: f64_le(bytes, off + 9)?,
            t_min_mjd: f64_le(bytes, off + 17)?,
            t_max_mjd: f64_le(bytes, off + 25)?,
            em_band_m: if presence & PRESENCE_EM != 0 {
                Some((f64_le(bytes, off + 33)?, f64_le(bytes, off + 41)?))
            } else {
                None
            },
            t_exptime_s: if presence & PRESENCE_EXPTIME != 0 {
                Some(f64_le(bytes, off + 49)?)
            } else {
                None
            },
        };
        if !valid(&r) {
            return None;
        }
        off += RECORD_STRIDE_BYTES;
        records.push(r);
    }
    if off != bytes.len() {
        return None;
    }
    Some(records)
}

fn cell_f64(cells: &[&str], k: usize) -> Option<f64> {
    let cell = cells.get(k)?.trim();
    if cell.is_empty() {
        return None;
    }
    let v: f64 = cell.parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

pub struct ObscoreCsvCounts {
    pub rows: usize,
    pub emitted: usize,
    pub position_void: usize,
    pub time_void: usize,
    pub spectral_void: usize,
    pub exposure_void: usize,
}

pub fn parse_obscore_csv(body: &str) -> Option<(Vec<ObsRecord>, ObscoreCsvCounts)> {
    let mut lines = body.lines();
    let header = lines.next()?;
    let cols: Vec<&str> = header.split(',').map(|c| c.trim()).collect();
    let index_of = |name: &str| cols.iter().position(|c| *c == name);
    let (
        Some(i_ra),
        Some(i_dec),
        Some(i_tmin),
        Some(i_tmax),
        Some(i_texp),
        Some(i_emin),
        Some(i_emax),
    ) = (
        index_of("s_ra"),
        index_of("s_dec"),
        index_of("t_min"),
        index_of("t_max"),
        index_of("t_exptime"),
        index_of("em_min"),
        index_of("em_max"),
    )
    else {
        return None;
    };
    let mut counts = ObscoreCsvCounts {
        rows: 0,
        emitted: 0,
        position_void: 0,
        time_void: 0,
        spectral_void: 0,
        exposure_void: 0,
    };
    let mut records = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split(',').collect();
        counts.rows += 1;
        let (Some(ra), Some(dec)) = (cell_f64(&cells, i_ra), cell_f64(&cells, i_dec)) else {
            counts.position_void += 1;
            continue;
        };
        if !(0.0..360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
            counts.position_void += 1;
            continue;
        }
        let (Some(tmin), Some(tmax)) = (cell_f64(&cells, i_tmin), cell_f64(&cells, i_tmax)) else {
            counts.time_void += 1;
            continue;
        };
        if tmax < tmin {
            counts.time_void += 1;
            continue;
        }
        let em_band = match (cell_f64(&cells, i_emin), cell_f64(&cells, i_emax)) {
            (Some(lo), Some(hi)) if lo > 0.0 && hi >= lo => Some((lo, hi)),
            (Some(_), Some(_)) => {
                counts.spectral_void += 1;
                None
            }
            _ => None,
        };
        let exptime = match cell_f64(&cells, i_texp) {
            Some(s) if s >= 0.0 => Some(s),
            Some(_) => {
                counts.exposure_void += 1;
                None
            }
            None => None,
        };
        records.push(ObsRecord {
            s_ra_deg: ra,
            s_dec_deg: dec,
            t_min_mjd: tmin,
            t_max_mjd: tmax,
            em_band_m: em_band,
            t_exptime_s: exptime,
        });
        counts.emitted += 1;
    }
    if counts.rows == 0 {
        return None;
    }
    Some((records, counts))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ObsRecord {
        ObsRecord {
            s_ra_deg: 251.25752526992915,
            s_dec_deg: 72.3420533060491,
            t_min_mjd: 59107.72385694459,
            t_max_mjd: 59107.72385694459,
            em_band_m: Some((5.42e-7, 6.99e-7)),
            t_exptime_s: Some(240.0),
        }
    }

    #[test]
    fn bin_roundtrip_preserves_the_measured_observation() {
        let rec = sample();
        let bytes = write_bin(&[rec]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], rec);
    }

    #[test]
    fn bin_roundtrip_carries_absent_optional_fields() {
        let rec = ObsRecord {
            em_band_m: None,
            t_exptime_s: None,
            ..sample()
        };
        let bytes = write_bin(&[rec]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].em_band_m, None);
        assert_eq!(parsed[0].t_exptime_s, None);
        assert_eq!(parsed[0].s_ra_deg, rec.s_ra_deg);
    }

    #[test]
    fn bin_refuses_non_finite_and_out_of_range_values() {
        let bad = ObsRecord {
            s_ra_deg: 400.0,
            ..sample()
        };
        assert!(write_bin(&[bad]).is_none());
        let bad = ObsRecord {
            em_band_m: Some((0.0, 6.99e-7)),
            ..sample()
        };
        assert!(write_bin(&[bad]).is_none());
        let bad = ObsRecord {
            t_exptime_s: Some(-1.0),
            ..sample()
        };
        assert!(write_bin(&[bad]).is_none());
    }

    #[test]
    fn parse_bin_rejects_a_truncated_stream() {
        let bytes = write_bin(&[sample()]).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        assert!(parse_bin(&bytes[..7]).is_none());
    }

    #[test]
    fn mjd_to_unix_carries_the_epoch_offset() {
        assert_eq!(mjd_to_unix(MJD_UNIX_EPOCH), 0.0);
        assert!((mjd_to_unix(59107.72385694459) - 1600194560.0).abs() < 1e4);
    }

    #[test]
    fn parse_obscore_csv_reads_a_measured_row() {
        let body = "s_ra,s_dec,t_min,t_max,t_exptime,em_min,em_max\n251.25752526992915,72.3420533060491,59107.72385694459,59107.72385694459,240.0,5.42e-07,6.99e-07\n";
        let (records, counts) = parse_obscore_csv(body).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0], sample());
        assert_eq!(counts.emitted, 1);
        assert_eq!(counts.position_void, 0);
    }

    #[test]
    fn parse_obscore_csv_skips_a_row_whose_position_is_void() {
        let body = "s_ra,s_dec,t_min,t_max,t_exptime,em_min,em_max\n,72.3420533060491,59107.7,59107.7,240.0,5.42e-07,6.99e-07\n";
        let (records, counts) = parse_obscore_csv(body).unwrap();
        assert!(records.is_empty());
        assert_eq!(counts.position_void, 1);
    }

    #[test]
    fn parse_obscore_csv_carries_absent_spectral_band() {
        let body = "s_ra,s_dec,t_min,t_max,t_exptime,em_min,em_max\n251.2575,72.3420,59107.7,59107.7,240.0,,\n";
        let (records, counts) = parse_obscore_csv(body).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].em_band_m, None);
        assert_eq!(records[0].t_exptime_s, Some(240.0));
        assert_eq!(counts.emitted, 1);
    }
}
