pub const RECORD_BYTES: usize = 75;
pub const FIXED_LEAD_1: u8 = 0x0D;
pub const FIXED_LEAD_2: u8 = 0x0A;
pub const FIXED_LEAD_3: u8 = 0x01;
pub const FIXED_TAIL: u8 = 0x04;

pub const ANGLE_SCALE_DEG: f64 = 8.381903173e-8;
pub const RTLT_UNIT_S: f64 = 1.0 / 256e9;
pub const TX_FREQ_SCALE_HZ: f64 = 10.0;

pub const DOPPLER_BIAS_HZ: f64 = 2.4e8;
pub const DOPPLER_RATE_SCALE: f64 = 1000.0;
pub const S_BAND_RATIO: f64 = 240.0 / 221.0;

pub const R_VALID_BIT: u8 = 0x01;

fn u8_at(rec: &[u8], i: usize) -> u8 {
    rec[i]
}

fn u16_msb(rec: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([rec[i], rec[i + 1]])
}

fn u32_msb(rec: &[u8], i: usize) -> u32 {
    u32::from_be_bytes([rec[i], rec[i + 1], rec[i + 2], rec[i + 3]])
}

fn f32_be(rec: &[u8], i: usize) -> f32 {
    f32::from_be_bytes([rec[i], rec[i + 1], rec[i + 2], rec[i + 3]])
}

pub struct Tracking {
    pub year: i64,
    pub seconds: i64,
    pub micros: i64,
    pub sic: i64,
    pub vid: i64,
    pub tracker_type: i64,
    pub sample_rate: i64,
    pub agc: i64,
    pub mode_system: i64,
    pub validity: u8,
    pub r_valid: bool,
    pub doppler_cnt: i64,
    pub rtlt_ticks: i64,
    pub angle1: f64,
    pub angle2: f64,
    pub tx_freq_hz: f64,
}

pub fn tracking_record(rec: &[u8]) -> Option<Tracking> {
    if rec.len() != RECORD_BYTES {
        return None;
    }
    let sample_bits = ((u8_at(rec, 52) & 0x07) as i64) << 8 | u8_at(rec, 53) as i64;
    let sample_rate = if sample_bits >= 1024 {
        sample_bits - 2048
    } else {
        sample_bits
    };
    let doppler_cnt = ((u32_msb(rec, 32) as i64) << 16) | u16_msb(rec, 36) as i64;
    let rtlt_ticks = ((u32_msb(rec, 26) as i64) << 16) | u16_msb(rec, 30) as i64;
    let validity = u8_at(rec, 50);
    let geometry = u8_at(rec, 46) & 0x0F;
    let xy_antenna = geometry == 1 || geometry == 2;
    let mut angle1 = f32_be(rec, 18) as f64 * ANGLE_SCALE_DEG;
    let mut angle2 = f32_be(rec, 22) as f64 * ANGLE_SCALE_DEG;
    if xy_antenna {
        if angle1 > 180.0 {
            angle1 -= 360.0;
        }
        if angle2 > 180.0 {
            angle2 -= 360.0;
        }
    }
    Some(Tracking {
        year: u8_at(rec, 5) as i64,
        seconds: u32_msb(rec, 10) as i64,
        micros: u32_msb(rec, 14) as i64,
        sic: u16_msb(rec, 6) as i64,
        vid: u16_msb(rec, 8) as i64,
        tracker_type: (u8_at(rec, 52) >> 4) as i64,
        sample_rate,
        agc: u16_msb(rec, 38) as i64,
        mode_system: u16_msb(rec, 48) as i64,
        validity,
        r_valid: validity & R_VALID_BIT == R_VALID_BIT,
        doppler_cnt,
        rtlt_ticks,
        angle1,
        angle2,
        tx_freq_hz: f32_be(rec, 40) as f64 * TX_FREQ_SCALE_HZ,
    })
}

pub fn full_year(two: i64) -> i64 {
    if two < 70 { 2000 + two } else { 1900 + two }
}

fn sampler_of(rate: i64) -> Option<f64> {
    match rate {
        1..=1023 => Some(rate as f64),
        -1023..=-1 => Some(1.0 / (-rate) as f64),
        _ => None,
    }
}

fn tdb_of(tr: &Tracking, lsk: &crate::lsk::LeapSeconds) -> Option<f64> {
    if tr.seconds < 0 || tr.seconds > 366 * 86400 {
        return None;
    }
    let year = full_year(tr.year);
    let days = crate::lsk::days_from_civil(year, 1, 1)?;
    let unix = days as f64 * 86400.0 + tr.seconds as f64 + tr.micros as f64 / 1e6;
    lsk.unix_to_tdb(unix)
}

pub fn write_bin(records: &[[f64; 14]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 112);
    out.extend_from_slice(b"LUTD");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<[f64; 14]>> {
    if data.len() < 8 || &data[0..4] != b"LUTD" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 112 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 112;
        let mut r = [0.0f64; 14];
        for k in 0..14 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub const COMP_LRO_SKYFREQ: u32 = 0;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_LRO_SKYFREQ => Some("lro_sky_frequency_hz"),
        _ => None,
    }
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_bin(bytes)?;
    let out: Vec<(f64, f64, u32)> = rows
        .into_iter()
        .filter(|r| r[0].is_finite() && r[1].is_finite() && r[1] > 0.0)
        .map(|r| (r[0], r[1], COMP_LRO_SKYFREQ))
        .collect();
    if out.is_empty() { None } else { Some(out) }
}

pub fn reduce_lro_trk(
    name: &str,
    file_id: f64,
    bytes: &[u8],
    lsk: &crate::lsk::LeapSeconds,
) -> Option<Vec<[f64; 14]>> {
    if !bytes.len().is_multiple_of(RECORD_BYTES) {
        eprintln!(
            "{name}: {} bytes — not a multiple of {RECORD_BYTES} (0 honored)",
            bytes.len()
        );
        return None;
    }
    let nrec = bytes.len() / RECORD_BYTES;
    if nrec < 2 {
        eprintln!("{name}: {nrec} records — too short");
        return None;
    }
    let mut recs: Vec<Tracking> = Vec::with_capacity(nrec);
    let mut skipped_fixed = 0usize;
    let mut type_hist: Vec<(i64, usize)> = Vec::new();
    for i in 0..nrec {
        let rec = &bytes[i * RECORD_BYTES..(i + 1) * RECORD_BYTES];
        let Some(tr) = tracking_record(rec) else {
            continue;
        };
        let hist = type_hist.iter_mut().find(|(d, _)| *d == tr.tracker_type);
        match hist {
            Some((_, c)) => *c += 1,
            None => type_hist.push((tr.tracker_type, 1)),
        }
        if u8_at(rec, 0) != FIXED_LEAD_1
            || u8_at(rec, 1) != FIXED_LEAD_2
            || u8_at(rec, 2) != FIXED_LEAD_3
        {
            skipped_fixed += 1;
            continue;
        }
        recs.push(tr);
    }
    if recs.len() < 2 {
        eprintln!("{name}: {} framed records — too short", recs.len());
        return None;
    }
    let mut n = recs.len();
    let mut t = vec![0.0f64; n];
    let mut dcnt = vec![0.0f64; n];
    let mut tx = vec![0.0f64; n];
    let mut sampler = vec![0.0f64; n];
    let mut agc = vec![0i64; n];
    let mut sic = vec![0i64; n];
    let mut mode = vec![0i64; n];
    let mut validity = vec![0u8; n];
    let mut rtlt = vec![0.0f64; n];
    let mut angle1 = vec![0.0f64; n];
    let mut angle2 = vec![0.0f64; n];
    let mut rate = vec![0i64; n];
    let mut kept = 0usize;
    let mut invalid_r = 0usize;
    let mut no_sampler = 0usize;
    let mut no_tx = 0usize;
    for tr in recs.iter() {
        let Some(tdb) = tdb_of(tr, lsk) else {
            continue;
        };
        if !tr.r_valid {
            invalid_r += 1;
            continue;
        }
        let Some(s) = sampler_of(tr.sample_rate) else {
            no_sampler += 1;
            continue;
        };
        if !(tr.tx_freq_hz.is_finite() && tr.tx_freq_hz > 0.0) {
            no_tx += 1;
            continue;
        }
        t[kept] = tdb;
        dcnt[kept] = tr.doppler_cnt as f64;
        tx[kept] = tr.tx_freq_hz;
        sampler[kept] = s;
        agc[kept] = tr.agc;
        sic[kept] = tr.sic;
        mode[kept] = tr.mode_system;
        validity[kept] = tr.validity;
        rtlt[kept] = tr.rtlt_ticks as f64 * RTLT_UNIT_S;
        angle1[kept] = tr.angle1;
        angle2[kept] = tr.angle2;
        rate[kept] = tr.sample_rate;
        kept += 1;
    }
    n = kept;
    if n < 2 {
        eprintln!(
            "{name}: {} valid records ({invalid_r} without r-validity, {no_sampler} without sample rate, {no_tx} without transmit frequency) — too short",
            n
        );
        return None;
    }
    t.truncate(n);
    dcnt.truncate(n);
    tx.truncate(n);
    sampler.truncate(n);
    agc.truncate(n);
    sic.truncate(n);
    mode.truncate(n);
    validity.truncate(n);
    rtlt.truncate(n);
    angle1.truncate(n);
    angle2.truncate(n);
    rate.truncate(n);

    let tx_min = tx.iter().copied().fold(f64::INFINITY, f64::min);
    let tx_max = tx.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut fsky = vec![0.0f64; n - 1];
    let mut pair = vec![false; n - 1];
    let mut wrap_rejected = 0usize;
    for i in 0..n - 1 {
        if dcnt[i + 1] < dcnt[i] {
            wrap_rejected += 1;
            continue;
        }
        let drate = (dcnt[i + 1] - dcnt[i]) / sampler[i];
        let sdoppler = (drate - DOPPLER_BIAS_HZ) / DOPPLER_RATE_SCALE;
        fsky[i] = S_BAND_RATIO * tx[i] + sdoppler;
        pair[i] = fsky[i].is_finite();
    }
    let mut out: Vec<[f64; 14]> = Vec::new();
    for i in 0..n - 1 {
        if !pair[i] {
            continue;
        }
        out.push([
            t[i],
            fsky[i],
            tx[i],
            sampler[i],
            agc[i] as f64,
            validity[i] as f64,
            sic[i] as f64,
            dcnt[i],
            rtlt[i],
            angle1[i],
            angle2[i],
            rate[i] as f64,
            file_id,
            mode[i] as f64,
        ]);
    }
    let n_out = out.len();
    let mut sics: Vec<i64> = out.iter().map(|r| r[6] as i64).collect();
    sics.sort_unstable();
    sics.dedup();
    type_hist.sort_by_key(|(d, _)| *d);
    eprintln!(
        "{name}: {nrec} records ({skipped_fixed} without fixed lead), tracker types {type_hist:?}, {n_out} sky-frequency samples, tx {tx_min:.3e}..{tx_max:.3e} Hz, SICs {sics:?} — separated: {wrap_rejected} wrap"
    );
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec_with(bytes: &[(usize, u8)]) -> Vec<u8> {
        let mut rec = vec![0u8; RECORD_BYTES];
        for &(i, b) in bytes {
            rec[i] = b;
        }
        rec
    }

    #[test]
    fn fixed_lead_and_tail_positions() {
        let rec = rec_with(&[
            (0, FIXED_LEAD_1),
            (1, FIXED_LEAD_2),
            (2, FIXED_LEAD_3),
            (72, FIXED_TAIL),
            (73, FIXED_TAIL),
            (74, FIXED_TAIL),
        ]);
        let tr = tracking_record(&rec).unwrap();
        assert_eq!(tr.year, 0);
        assert_eq!(tr.sic, 0);
        assert_eq!(u8_at(&rec, 72), FIXED_TAIL);
    }

    #[test]
    fn header_fields_read_msb_first() {
        let mut rec = rec_with(&[]);
        rec[5] = 9;
        rec[6] = 0x00;
        rec[7] = 0x3B;
        rec[8] = 0x00;
        rec[9] = 0x01;
        rec[10..14].copy_from_slice(&0x002D_C6C7u32.to_be_bytes());
        rec[14..18].copy_from_slice(&5_000_000u32.to_be_bytes());
        rec[38..40].copy_from_slice(&0x0102u16.to_be_bytes());
        rec[48..50].copy_from_slice(&0x0A0Bu16.to_be_bytes());
        let tr = tracking_record(&rec).unwrap();
        assert_eq!(full_year(tr.year), 2009);
        assert_eq!(tr.sic, 59);
        assert_eq!(tr.vid, 1);
        assert_eq!(tr.seconds, 0x002D_C6C7);
        assert_eq!(tr.micros, 5_000_000);
        assert_eq!(tr.agc, 0x0102);
        assert_eq!(tr.mode_system, 0x0A0B);
    }

    #[test]
    fn sample_rate_signed_eleven_bits() {
        let mut rec = rec_with(&[(53, 0x0A)]);
        rec[52] = 0x00;
        assert_eq!(tracking_record(&rec).unwrap().sample_rate, 10);
        rec[52] = 0x07;
        rec[53] = 0xFF;
        assert_eq!(tracking_record(&rec).unwrap().sample_rate, -1);
    }

    #[test]
    fn doppler_and_rtlt_forty_eight_bit() {
        let mut rec = rec_with(&[]);
        rec[26..30].copy_from_slice(&0x0001_0000u32.to_be_bytes());
        rec[30..32].copy_from_slice(&0x0002u16.to_be_bytes());
        rec[32..36].copy_from_slice(&0x8000_0000u32.to_be_bytes());
        rec[36..38].copy_from_slice(&0x0001u16.to_be_bytes());
        let tr = tracking_record(&rec).unwrap();
        assert_eq!(tr.rtlt_ticks, (1i64 << 32) + 2);
        assert_eq!(tr.doppler_cnt, (0x8000_0000i64 << 16) + 1);
    }

    #[test]
    fn transmit_frequency_ieee_real_big_endian() {
        let mut rec = rec_with(&[]);
        rec[40..44].copy_from_slice(&1.0f32.to_be_bytes());
        let tr = tracking_record(&rec).unwrap();
        assert_eq!(tr.tx_freq_hz, 10.0);
    }

    #[test]
    fn lutd_roundtrip_carries_fourteen_slots() {
        let records = vec![
            [
                1.0, 2.2e9, 2.1e9, 1.0, 50.0, 1.0, 59.0, 1.0e11, 0.4, 45.0, -45.0, 1.0, 0.0, 1.0,
            ],
            [
                2.0, 2.2e9, 2.1e9, 1.0, 60.0, 1.0, 59.0, 1.0e11, 0.4, 45.0, -45.0, 1.0, 1.0, 1.0,
            ],
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        assert!(parse_bin(b"LUTD").is_none());
        assert!(parse_bin(b"PASF").is_none());
    }

    #[test]
    fn series_skips_absent_frequency() {
        let row = [
            1.0, 0.0, 2.1e9, 1.0, 50.0, 1.0, 59.0, 1.0e11, 0.4, 0.0, 0.0, 1.0, 0.0, 1.0,
        ];
        let bytes = write_bin(&[row]);
        assert!(parse_series(&bytes).is_none());
    }

    #[test]
    fn series_roundtrip_and_component_name() {
        let row = [
            1.0, 2.2e9, 2.1e9, 1.0, 50.0, 1.0, 59.0, 1.0e11, 0.4, 45.0, -45.0, 1.0, 0.0, 1.0,
        ];
        let bytes = write_bin(&[row]);
        let parsed = parse_series(&bytes).expect("lro series parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].2, COMP_LRO_SKYFREQ);
        assert_eq!(
            component_name(COMP_LRO_SKYFREQ),
            Some("lro_sky_frequency_hz")
        );
        assert_eq!(component_name(99), None);
    }

    #[test]
    fn sampler_intervals() {
        assert_eq!(sampler_of(60), Some(60.0));
        assert_eq!(sampler_of(-10), Some(0.1));
        assert_eq!(sampler_of(0), None);
        assert_eq!(sampler_of(1024), None);
        assert_eq!(sampler_of(-2048), None);
    }

    #[test]
    fn xy_antenna_angle_wraps_past_half_turn() {
        let mut rec = rec_with(&[]);
        rec[46] = 0x01;
        let angle_bits = 190.0 / ANGLE_SCALE_DEG;
        rec[18..22].copy_from_slice(&(angle_bits as f32).to_be_bytes());
        let tr = tracking_record(&rec).unwrap();
        assert!(
            (tr.angle1 - (-170.0)).abs() < 1e-3,
            "X-Y angle folds to {}",
            tr.angle1
        );
    }
}
