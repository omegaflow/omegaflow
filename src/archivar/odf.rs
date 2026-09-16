pub const ORBIT_KEY: u32 = 1031;
pub const PK_FILE_LABEL: u32 = 101;
pub const PK_IDENTIFIER: u32 = 107;
pub const PK_ORBIT_HEADER: u32 = 109;
pub const PK_RAMP: u32 = 2030;
pub const PK_CLOCK: u32 = 2040;
pub const PK_SUMMARY: u32 = 105;

pub const ODF_TDB_OFFSET: f64 = -1577664000.0;

pub const COMP_OBSERVABLE: u32 = 1;

pub const PODF_COL_TDB: usize = 0;
pub const PODF_COL_OBSERVABLE: usize = 1;

#[derive(Clone, Copy, Debug)]
pub struct OdOrbit {
    pub t_since_1950: f64,
    pub observable_hz: f64,
    pub dss_rx: i64,
    pub dss_tx: i64,
    pub data_type: i64,
    pub downlink_band: i64,
    pub uplink_band: i64,
    pub valid: bool,
    pub scid: i64,
    pub ref_hz: f64,
    pub compression_s: f64,
}

pub fn bits(words: &[u32; 9], first: usize, last: usize) -> i64 {
    let mut v: i64 = 0;
    for b in first..=last {
        let word = (b - 1) / 32;
        let wbit = (b - 1) % 32;
        let bit = (words[word] >> (31 - wbit)) & 1;
        v = (v << 1) | bit as i64;
    }
    v
}

pub fn twos(v: i64, width: usize) -> i64 {
    let sign = 1i64 << (width - 1);
    if v & sign != 0 {
        v - (1i64 << width)
    } else {
        v
    }
}

pub fn orbit_record(words: &[u32; 9]) -> Option<OdOrbit> {
    if words[0] < 0x400000 {
        return None;
    }
    let fmt = bits(words, 129, 131);
    let t_int = words[0] as i64;
    let observable = twos(words[2] as i64, 32) as f64 + twos(words[3] as i64, 32) as f64 / 1.0e9;
    let dss_rx = bits(words, 132, 138);
    let dss_tx = bits(words, 139, 145);
    let (t_frac, t_div, data_type, downlink_band, uplink_band, valid, scid, ref_hz, compression) =
        if fmt == 1 {
            (
                bits(words, 33, 64),
                1.0e9,
                bits(words, 150, 155),
                bits(words, 148, 149),
                bits(words, 187, 188),
                bits(words, 200, 200) == 0,
                bits(words, 161, 167),
                bits(words, 225, 256) as f64 * 10.0 + bits(words, 257, 264) as f64 * 0.1,
                bits(words, 201, 224) as f64 / 100.0,
            )
        } else {
            (
                bits(words, 33, 42),
                1.0e10,
                bits(words, 148, 153),
                bits(words, 154, 155),
                bits(words, 156, 157),
                bits(words, 160, 160) == 0,
                bits(words, 168, 177),
                bits(words, 179, 224) as f64 / 1000.0,
                bits(words, 245, 266) as f64 / 100.0,
            )
        };
    Some(OdOrbit {
        t_since_1950: t_int as f64 + t_frac as f64 / t_div,
        observable_hz: observable,
        dss_rx,
        dss_tx,
        data_type,
        downlink_band,
        uplink_band,
        valid,
        scid,
        ref_hz,
        compression_s: compression,
    })
}

pub fn write_podf_bin(records: &[[f64; 9]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 72);
    out.extend_from_slice(b"PODF");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub const PODF_SHARD_BUDGET: usize = 1 << 30;
pub const PODF_SHARD_LIMIT: usize = 1 << 31;

pub fn podf_shard_ranges(count: usize, budget: usize) -> Vec<(usize, usize)> {
    if count == 0 {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    let mut lo = 0usize;
    while lo < count {
        let mut hi = lo + 1;
        while hi < count && 8 + (hi + 1 - lo) * 72 <= budget {
            hi += 1;
        }
        ranges.push((lo, hi));
        lo = hi;
    }
    ranges
}

pub fn podf_shard_name(prefix: &str, t_lo: f64, t_hi: f64) -> String {
    format!("{prefix}_t{}_{}.bin", t_lo as i64, t_hi as i64)
}

pub fn podf_shard_name_ord(prefix: &str, t_lo: f64, t_hi: f64, ord: usize) -> String {
    format!("{prefix}_t{}_{}_{}.bin", t_lo as i64, t_hi as i64, ord)
}

pub fn parse_podf_shard_name(filename: &str) -> Option<(&str, f64, f64)> {
    let stem = filename.strip_suffix(".bin")?;
    let (name, range) = stem.rsplit_once("_t")?;
    let mut it = range.split('_');
    let lo: f64 = it.next()?.parse().ok()?;
    let hi: f64 = it.next()?.parse().ok()?;
    Some((name, lo, hi))
}

pub fn parse_podf_bin(data: &[u8]) -> Option<Vec<[f64; 9]>> {
    if data.len() < 8 || &data[0..4] != b"PODF" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 72 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 72;
        let mut r = [0.0f64; 9];
        for k in 0..9 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_podf_bin(bytes)?;
    let mut out = Vec::with_capacity(rows.len());
    for r in &rows {
        let t = r[PODF_COL_TDB];
        let v = r[PODF_COL_OBSERVABLE];
        if !t.is_finite() || !v.is_finite() {
            continue;
        }
        out.push((t, v, COMP_OBSERVABLE));
    }
    Some(out)
}

pub fn write_p11r_bin(records: &[[f64; 9]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 72);
    out.extend_from_slice(b"P11R");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_p11r_bin(data: &[u8]) -> Option<Vec<[f64; 9]>> {
    if data.len() < 8 || &data[0..4] != b"P11R" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 72 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 72;
        let mut r = [0.0f64; 9];
        for k in 0..9 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub fn parse_odf(bytes: &[u8]) -> Option<Vec<OdOrbit>> {
    if !bytes.len().is_multiple_of(36) {
        return None;
    }
    let mut out = Vec::new();
    let mut in_orbit = false;
    let n = bytes.len() / 36;
    for i in 0..n {
        let mut words = [0u32; 9];
        for k in 0..9 {
            words[k] =
                u32::from_be_bytes(bytes[i * 36 + k * 4..i * 36 + k * 4 + 4].try_into().ok()?);
        }
        match words[0] {
            PK_FILE_LABEL | PK_IDENTIFIER | PK_ORBIT_HEADER | PK_RAMP | PK_CLOCK | PK_SUMMARY => {
                in_orbit = words[0] == PK_ORBIT_HEADER;
            }
            _ => {
                if in_orbit && let Some(r) = orbit_record(&words) {
                    out.push(r);
                }
            }
        }
    }
    Some(out)
}

pub const TNF_LABEL_LEN: usize = 20;

#[derive(Clone, Copy, Debug)]
pub struct TnfSfdu {
    pub offset: usize,
    pub total_len: usize,
    pub data_description_id: [u8; 4],
    pub sfdu_length: u64,
    pub aggr_chdo_type: u16,
    pub aggr_chdo_length: u16,
    pub primary_chdo_type: u16,
    pub primary_chdo_length: u16,
    pub mjr_data_class: u8,
    pub mnr_data_class: u8,
    pub mission_id: u8,
    pub format_code: u8,
    pub secondary_chdo_type: u16,
    pub secondary_chdo_length: u16,
    pub scft_id: u8,
    pub rec_seq_num: u32,
    pub year: u16,
    pub doy: u16,
    pub sec: f64,
}

fn be16(b: &[u8]) -> u16 {
    u16::from_be_bytes([b[0], b[1]])
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

fn be64(b: &[u8]) -> u64 {
    u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

fn read_tnf_sfdu(bytes: &[u8], offset: usize) -> Option<TnfSfdu> {
    if bytes.len() < 60 || &bytes[0..4] != b"NJPL" {
        return None;
    }
    let sfdu_length = be64(&bytes[12..20]);
    let total_len = TNF_LABEL_LEN + sfdu_length as usize;
    if bytes.len() < total_len {
        return None;
    }
    let mut data_description_id = [0u8; 4];
    data_description_id.copy_from_slice(&bytes[8..12]);
    Some(TnfSfdu {
        offset,
        total_len,
        data_description_id,
        sfdu_length,
        aggr_chdo_type: be16(&bytes[20..22]),
        aggr_chdo_length: be16(&bytes[22..24]),
        primary_chdo_type: be16(&bytes[24..26]),
        primary_chdo_length: be16(&bytes[26..28]),
        mjr_data_class: bytes[28],
        mnr_data_class: bytes[29],
        mission_id: bytes[30],
        format_code: bytes[31],
        secondary_chdo_type: be16(&bytes[32..34]),
        secondary_chdo_length: be16(&bytes[34..36]),
        scft_id: bytes[39],
        rec_seq_num: be32(&bytes[44..48]),
        year: be16(&bytes[48..50]),
        doy: be16(&bytes[50..52]),
        sec: f64::from_be_bytes(bytes[52..60].try_into().ok()?),
    })
}

pub fn scan_tnf_sfdus(bytes: &[u8]) -> Option<Vec<TnfSfdu>> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off < bytes.len() {
        let frame = read_tnf_sfdu(&bytes[off..], off)?;
        off += frame.total_len;
        out.push(frame);
    }
    Some(out)
}

#[derive(Clone, Copy, Debug)]
pub struct TnfDt0 {
    pub orig_id: u8,
    pub last_modifier_id: u8,
    pub upl_rec_seq_num: u32,
    pub rct_day: u16,
    pub rct_msec: u32,
    pub ul_dss_id: u8,
    pub ul_band: u8,
    pub ul_assembly_num: u8,
    pub transmit_num: u8,
    pub transmit_stat: u8,
    pub transmit_mode: u8,
    pub cmd_modul_stat: u8,
    pub rng_modul_stat: u8,
    pub fts_vld_flag: u8,
    pub transmit_time_tag_delay: f64,
    pub ul_zheight_corr: f32,
    pub mod_day: u16,
    pub mod_msec: u32,
    pub version_num: u8,
    pub sub_version_num: u8,
    pub sub_sub_version_num: u8,
    pub data_chdo_type: u16,
    pub data_chdo_length: u16,
    pub ul_hi_phs_cycles: u32,
    pub ul_lo_phs_cycles: u32,
    pub ul_frac_phs_cycles: u32,
    pub ramp_freq: f64,
    pub ramp_rate: f64,
    pub transmit_switch_stat: u8,
    pub ramp_type: u8,
    pub transmit_op_pwr: f32,
    pub sup_data_id: [u8; 8],
    pub sup_data_rev: [u8; 8],
    pub prdx_time_offset: f64,
    pub prdx_freq_offset: f64,
    pub time_tag_corr_flag: u8,
    pub type_time_corr_flag: u8,
    pub fabricated_sfdu_flag: u8,
}

pub fn tnf_dt0(frame: &TnfSfdu, bytes: &[u8]) -> Option<TnfDt0> {
    if frame.format_code != 0 || bytes.len() < 182 {
        return None;
    }
    let mut sup_data_id = [0u8; 8];
    sup_data_id.copy_from_slice(&bytes[140..148]);
    let mut sup_data_rev = [0u8; 8];
    sup_data_rev.copy_from_slice(&bytes[148..156]);
    Some(TnfDt0 {
        orig_id: bytes[36],
        last_modifier_id: bytes[37],
        upl_rec_seq_num: be32(&bytes[40..44]),
        rct_day: be16(&bytes[60..62]),
        rct_msec: be32(&bytes[62..66]),
        ul_dss_id: bytes[66],
        ul_band: bytes[67],
        ul_assembly_num: bytes[68],
        transmit_num: bytes[69],
        transmit_stat: bytes[70],
        transmit_mode: bytes[71],
        cmd_modul_stat: bytes[72],
        rng_modul_stat: bytes[73],
        fts_vld_flag: bytes[74],
        transmit_time_tag_delay: f64::from_be_bytes(bytes[76..84].try_into().ok()?),
        ul_zheight_corr: f32::from_be_bytes(bytes[84..88].try_into().ok()?),
        mod_day: be16(&bytes[88..90]),
        mod_msec: be32(&bytes[90..94]),
        version_num: bytes[94],
        sub_version_num: bytes[95],
        sub_sub_version_num: bytes[96],
        data_chdo_type: be16(&bytes[102..104]),
        data_chdo_length: be16(&bytes[104..106]),
        ul_hi_phs_cycles: be32(&bytes[106..110]),
        ul_lo_phs_cycles: be32(&bytes[110..114]),
        ul_frac_phs_cycles: be32(&bytes[114..118]),
        ramp_freq: f64::from_be_bytes(bytes[118..126].try_into().ok()?),
        ramp_rate: f64::from_be_bytes(bytes[126..134].try_into().ok()?),
        transmit_switch_stat: bytes[134],
        ramp_type: bytes[135],
        transmit_op_pwr: f32::from_be_bytes(bytes[136..140].try_into().ok()?),
        sup_data_id,
        sup_data_rev,
        prdx_time_offset: f64::from_be_bytes(bytes[156..164].try_into().ok()?),
        prdx_freq_offset: f64::from_be_bytes(bytes[164..172].try_into().ok()?),
        time_tag_corr_flag: bytes[172],
        type_time_corr_flag: bytes[173],
        fabricated_sfdu_flag: bytes[174],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_words() -> [u32; 9] {
        [
            0x6C028048, 0x00000000, 0xFFFA28EE, 0xD86F2B24, 0x4FC005C4, 0x02764217, 0x77808DE8,
            0x00000005, 0xDC000000,
        ]
    }

    fn galileo_delta_dor_words() -> [u32; 9] {
        [
            0x578AF474, 0x00000000, 0x000010CD, 0x10877FE6, 0x23802830, 0x9A000FC0, 0x00015F90,
            0x0DADE373, 0x1300000B,
        ]
    }

    #[test]
    fn golden_record_unpacks() {
        let r = orbit_record(&example_words()).unwrap();
        assert!((r.t_since_1950 - 1812103240.0).abs() < 1.0e-9);
        assert!((r.observable_hz - (-382738.663803100)).abs() < 1.0e-6);
        assert_eq!(r.dss_rx, 63);
        assert_eq!(r.dss_tx, 0);
        assert_eq!(r.data_type, 11);
        assert_eq!(r.downlink_band, 2);
        assert!(r.valid);
        assert_eq!(r.scid, 236);
        assert!((r.ref_hz - 2299812417.0).abs() < 1.0e-6);
        assert!((r.compression_s - 60.0).abs() < 1.0e-9);
    }

    #[test]
    fn golden_file_parses() {
        let bytes = std::fs::read("src/archivar/kernels/odf07155.dat").unwrap();
        let recs = parse_odf(&bytes).unwrap();
        assert!(!recs.is_empty());
        let first = recs[0];
        assert!((first.t_since_1950 - 1812103240.0).abs() < 1.0e-9);
        assert_eq!(first.data_type, 11);
    }

    #[test]
    fn real_galileo_delta_dor_record_decodes() {
        let r = orbit_record(&galileo_delta_dor_words()).unwrap();
        assert!((r.t_since_1950 - 1468724340.0).abs() < 1.0e-9);
        assert!((r.observable_hz - 4301.277315558).abs() < 1.0e-6);
        assert_eq!(r.dss_rx, 14);
        assert_eq!(r.dss_tx, 0);
        assert_eq!(r.data_type, 1);
        assert_eq!(r.downlink_band, 1);
        assert_eq!(r.uplink_band, 0);
        assert!(r.valid);
        assert_eq!(r.scid, 77);
        assert!((r.ref_hz - 2294997631.9).abs() < 1.0e-6);
        assert!((r.compression_s - 900.0).abs() < 1.0e-9);
    }

    fn set_bits(words: &mut [u32; 9], first: usize, last: usize, value: i64) {
        let width = last - first + 1;
        for b in 0..width {
            let bitpos = first + b;
            let word = (bitpos - 1) / 32;
            let wbit = (bitpos - 1) % 32;
            let mask = 1u32 << (31 - wbit);
            if (value >> (width - 1 - b)) & 1 == 1 {
                words[word] |= mask;
            } else {
                words[word] &= !mask;
            }
        }
    }

    #[test]
    fn format_1_record_decodes_from_the_1988_sis_layout() {
        let mut w = [0u32; 9];
        w[0] = 0x6C028048;
        set_bits(&mut w, 129, 131, 1);
        set_bits(&mut w, 150, 155, 12);
        set_bits(&mut w, 148, 149, 1);
        set_bits(&mut w, 187, 188, 1);
        set_bits(&mut w, 200, 200, 0);
        set_bits(&mut w, 161, 167, 77);
        set_bits(&mut w, 225, 256, 229_981_241);
        set_bits(&mut w, 257, 264, 7);
        set_bits(&mut w, 201, 224, 6000);
        let r = orbit_record(&w).unwrap();
        assert_eq!(r.data_type, 12);
        assert_eq!(r.downlink_band, 1);
        assert_eq!(r.uplink_band, 1);
        assert!(r.valid);
        assert_eq!(r.scid, 77);
        assert!((r.ref_hz - 2299812410.7).abs() < 1.0e-6);
        assert!((r.compression_s - 60.0).abs() < 1.0e-9);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_odf(b"X").is_none());
    }

    #[test]
    fn podf_roundtrip() {
        let recs = vec![[1.0, 2.0, 3.0, 14.0, 43.0, 12.0, 1.0, 24.0, 60.0]];
        let bytes = write_podf_bin(&recs);
        let parsed = parse_podf_bin(&bytes).unwrap();
        assert_eq!(parsed, recs);
        assert!(parse_podf_bin(b"X").is_none());
    }

    #[test]
    fn shard_ranges_cover_and_respect_budget() {
        let ranges = podf_shard_ranges(5, 160);
        assert_eq!(ranges, vec![(0, 2), (2, 4), (4, 5)]);
        for &(lo, hi) in &ranges {
            assert!(lo < hi);
            assert!(8 + (hi - lo) * 72 <= 160);
        }
        let mut covered: Vec<usize> = Vec::new();
        for &(lo, hi) in &ranges {
            covered.extend(lo..hi);
        }
        assert_eq!(covered, (0..5).collect::<Vec<usize>>());
    }

    #[test]
    fn shard_ranges_empty_for_zero_records() {
        assert!(podf_shard_ranges(0, 1 << 30).is_empty());
    }

    #[test]
    fn shard_ranges_single_oversize_record_is_one_shard() {
        assert_eq!(podf_shard_ranges(1, 10), vec![(0, 1)]);
    }

    #[test]
    fn shard_names_unique_for_multi_shard_partition() {
        let names = [
            podf_shard_name("mro_odf", 700_000_000.0, 800_000_000.0),
            podf_shard_name("mro_odf", 800_000_000.0, 900_000_000.0),
            podf_shard_name("mro_odf", 900_000_000.0, 1_000_000_000.0),
        ];
        let mut set = std::collections::BTreeSet::new();
        for n in names {
            assert!(set.insert(n), "duplicate shard name");
        }
    }

    #[test]
    fn shard_name_ordinal_disambiguates_equal_tdb_seconds() {
        assert_eq!(
            podf_shard_name("mro_odf", 123_456_789.2, 123_456_789.2),
            podf_shard_name("mro_odf", 123_456_789.8, 123_456_789.8)
        );
        assert_ne!(
            podf_shard_name_ord("mro_odf", 123_456_789.2, 123_456_789.2, 0),
            podf_shard_name_ord("mro_odf", 123_456_789.8, 123_456_789.8, 1)
        );
    }

    #[test]
    fn shard_name_roundtrips_through_the_parser() {
        let name = podf_shard_name("mars_express_odf", 700_000_000.0, 800_000_000.0);
        assert_eq!(
            parse_podf_shard_name(&name),
            Some(("mars_express_odf", 700_000_000.0, 800_000_000.0))
        );
        let ord = podf_shard_name_ord("mro_odf", 123_456_789.2, 123_456_789.8, 1);
        assert_eq!(
            parse_podf_shard_name(&ord),
            Some(("mro_odf", 123_456_789.0, 123_456_789.0))
        );
        assert_eq!(parse_podf_shard_name("mro_odf.bin"), None);
        assert_eq!(parse_podf_shard_name("mro_odf_t700.bin"), None);
    }

    #[test]
    fn shard_roundtrip_reconstructs_record_order() {
        let mut merged: Vec<[f64; 9]> = Vec::new();
        for i in 0..10 {
            let t = i as f64 * 100.0;
            merged.push([t, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        }
        let ranges = podf_shard_ranges(merged.len(), 500);
        assert!(ranges.len() > 1);
        let mut reconstructed: Vec<[f64; 9]> = Vec::new();
        for &(lo, hi) in &ranges {
            let bin = write_podf_bin(&merged[lo..hi]);
            let parsed = parse_podf_bin(&bin).unwrap();
            reconstructed.extend_from_slice(&parsed);
        }
        assert_eq!(reconstructed, merged);
    }

    #[test]
    fn p11r_roundtrip() {
        let recs = vec![[-8.0e8, 12.5, 2.3e9, 2.3e9, -1.7e4, 43.0, 43.0, 12.0, 60.0]];
        let bytes = write_p11r_bin(&recs);
        let parsed = parse_p11r_bin(&bytes).unwrap();
        assert_eq!(parsed, recs);
        assert!(parse_p11r_bin(b"X").is_none());
    }

    const NHREX_TNF_HEAD_2: &str = "4e4a504c324930304331323300000000000000a20001004e00020004060e18000084004230310062000000000000000007dc001540e77da000000000506304b6727a1a0201010000000001003f142f61ed5ae1ce344b7abd0000000000002200020000000000000a004c0042cd063881085a59b3cfd941faa3a0f766b3b600000000000000000000388c38480000000000000000000000000000000000000000000000000000000000000000000000000000000000004e4a504c324930304331323300000000000000a20001004e00020004060e18000084004230310062000000010000000107dc001540e77dc000000000506304b6727a1a0201010000000001003f142f61ed5ae1ce344b7abd0000000000002200020000000000000a004c0042cd07e2bb17d0c4ef343341faa3a0f766b3b600000000000000000003390b52d6000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    fn unhex(s: &str) -> Vec<u8> {
        let b = s.as_bytes();
        let mut out = Vec::with_capacity(b.len() / 2);
        let mut i = 0;
        while i + 1 < b.len() {
            let hi = (b[i] as char).to_digit(16).unwrap() as u8;
            let lo = (b[i + 1] as char).to_digit(16).unwrap() as u8;
            out.push((hi << 4) | lo);
            i += 2;
        }
        out
    }

    #[test]
    fn nhrex_tnf_sfdu_framing_decodes() {
        let bytes = unhex(NHREX_TNF_HEAD_2);
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        assert_eq!(frames.len(), 2);

        let f = &frames[0];
        assert_eq!(f.offset, 0);
        assert_eq!(f.total_len, 182);
        assert_eq!(&f.data_description_id, b"C123");
        assert_eq!(f.sfdu_length, 162);
        assert_eq!(f.aggr_chdo_type, 1);
        assert_eq!(f.aggr_chdo_length, 78);
        assert_eq!(f.primary_chdo_type, 2);
        assert_eq!(f.primary_chdo_length, 4);
        assert_eq!(f.mjr_data_class, 6);
        assert_eq!(f.mnr_data_class, 14);
        assert_eq!(f.mission_id, 24);
        assert_eq!(f.format_code, 0);
        assert_eq!(f.secondary_chdo_type, 132);
        assert_eq!(f.secondary_chdo_length, 66);
        assert_eq!(f.scft_id, 98);
        assert_eq!(f.rec_seq_num, 0);
        assert_eq!(f.year, 2012);
        assert_eq!(f.doy, 21);
        assert_eq!(f.sec, 48109.0);

        let g = &frames[1];
        assert_eq!(g.offset, 182);
        assert_eq!(g.total_len, 182);
        assert_eq!(&g.data_description_id, b"C123");
        assert_eq!(g.format_code, 0);
        assert_eq!(g.scft_id, 98);
        assert_eq!(g.rec_seq_num, 1);
        assert_eq!(g.year, 2012);
        assert_eq!(g.doy, 21);
        assert_eq!(g.sec, 48110.0);
    }

    #[test]
    fn nhrex_tnf_dt0_uplink_carrier_phase_decodes() {
        let bytes = unhex(NHREX_TNF_HEAD_2);
        let frames = scan_tnf_sfdus(&bytes).unwrap();
        let f = &frames[0];
        let d = tnf_dt0(f, &bytes[f.offset..]).unwrap();

        assert_eq!(d.orig_id, 0x30);
        assert_eq!(d.last_modifier_id, 0x31);
        assert_eq!(d.upl_rec_seq_num, 0);
        assert_eq!(d.rct_day, 20579);
        assert_eq!(d.rct_msec, 79065722);
        assert_eq!(d.ul_dss_id, 26);
        assert_eq!(d.ul_band, 2);
        assert_eq!(d.ul_assembly_num, 1);
        assert_eq!(d.transmit_num, 1);
        assert_eq!(d.transmit_stat, 0);
        assert_eq!(d.transmit_mode, 0);
        assert_eq!(d.cmd_modul_stat, 0);
        assert_eq!(d.rng_modul_stat, 0);
        assert_eq!(d.fts_vld_flag, 1);
        assert_eq!(d.transmit_time_tag_delay.to_bits(), 0x3f142f61ed5ae1ce);
        assert_eq!(d.ul_zheight_corr.to_bits(), 0x344b7abd);
        assert_eq!(d.mod_day, 0);
        assert_eq!(d.mod_msec, 0);
        assert_eq!(d.version_num, 34);
        assert_eq!(d.sub_version_num, 0);
        assert_eq!(d.sub_sub_version_num, 2);
        assert_eq!(d.data_chdo_type, 10);
        assert_eq!(d.data_chdo_length, 76);
        assert_eq!(d.ul_hi_phs_cycles, 0x0042cd06);
        assert_eq!(d.ul_lo_phs_cycles, 0x3881085a);
        assert_eq!(d.ul_frac_phs_cycles, 0x59b3cfd9);
        assert_eq!(d.ramp_freq.to_bits(), 0x41faa3a0f766b3b6);
        assert_eq!(d.ramp_rate, 0.0);
        assert_eq!(d.transmit_switch_stat, 0);
        assert_eq!(d.ramp_type, 0);
        assert_eq!(d.transmit_op_pwr.to_bits(), 0x388c3848);
        assert_eq!(d.sup_data_id, [0u8; 8]);
        assert_eq!(d.sup_data_rev, [0u8; 8]);
        assert_eq!(d.prdx_time_offset, 0.0);
        assert_eq!(d.prdx_freq_offset, 0.0);
        assert_eq!(d.time_tag_corr_flag, 0);
        assert_eq!(d.type_time_corr_flag, 0);
        assert_eq!(d.fabricated_sfdu_flag, 0);
    }
}
