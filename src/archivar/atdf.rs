pub const PHYSICAL_RECORD: usize = 8064;
pub const PHYSICAL_RECORD_MARKER: usize = 8065;
pub const LOGICAL_RECORD: usize = 288;

pub const S_BAND_REF_LO: f64 = 2197e4;
pub const S_BAND_REF_HI: f64 = 2200e4;
pub const FSKY_BASE_HZ: f64 = 2292e6;

pub struct Field {
    pub item: u32,
    pub start: usize,
    pub stop: usize,
    pub signlength: i32,
    pub outlength: usize,
    pub name: &'static str,
}

const fn f(
    item: u32,
    start1: usize,
    stop1: usize,
    signlength: i32,
    outlength: usize,
    name: &'static str,
) -> Field {
    Field {
        item,
        start: start1 - 1,
        stop: stop1 - 1,
        signlength,
        outlength,
        name,
    }
}

pub const IDFORM: &[Field] = &[
    f(1, 1, 36, 29, 32, "DATA_LENGTH"),
    f(2, 37, 72, 29, 32, "RECORD_TYPE"),
    f(3, 73, 84, 0, 8, "YEAR_NUM"),
    f(4, 85, 100, 0, 16, "DAY_NUM"),
    f(5, 101, 108, 0, 8, "HOUR"),
    f(6, 109, 120, 0, 8, "MINUTE"),
    f(7, 121, 128, 0, 8, "SECOND"),
    f(9, 149, 156, 0, 8, "SPACECRAFT"),
    f(10, 157, 164, 0, 8, "ID1"),
    f(11, 165, 172, 0, 8, "ID2"),
    f(12, 173, 180, 0, 8, "ID3"),
    f(13, 181, 192, 0, 8, "ID4"),
    f(14, 193, 208, 0, 8, "ID5"),
    f(15, 209, 216, 0, 8, "ID6"),
    f(16, 217, 228, 0, 8, "ID7"),
    f(17, 229, 236, 0, 8, "ID8"),
];

pub const XPFORM: &[Field] = &[
    f(1, 1, 36, 29, 32, "DATA_LENGTH"),
    f(2, 37, 72, 29, 32, "RECORD_TYPE"),
    f(3, 73, 84, 0, 8, "YEAR_ON"),
    f(4, 85, 100, 0, 16, "DAY_ON"),
    f(5, 101, 108, 0, 8, "HOUR_ON"),
    f(6, 109, 120, 0, 8, "MINUTE_ON"),
    f(7, 121, 128, 0, 8, "SECOND_ON"),
    f(9, 149, 156, 0, 8, "SPACECRAFT"),
    f(11, 181, 192, 0, 8, "YEAR_OFF"),
    f(12, 193, 208, 0, 16, "DAY_OFF"),
    f(13, 209, 216, 0, 8, "HOUR_OFF"),
    f(14, 217, 228, 0, 8, "MINUTE_OFF"),
    f(15, 229, 236, 0, 8, "SECOND_OFF"),
    f(17, 253, 288, 0, 32, "SC_XPON_HP"),
    f(18, 289, 324, 0, 32, "SC_XPON_LP"),
];

pub const TKFORM: &[Field] = &[
    f(1, 1, 36, 29, 32, "DATA_LENGTH"),
    f(2, 37, 72, 29, 32, "RECORD_TYPE"),
    f(3, 73, 84, 0, 8, "YEAR_TAG"),
    f(4, 85, 100, 0, 16, "DAY_TAG"),
    f(5, 101, 108, 0, 8, "HOUR_TAG"),
    f(6, 109, 120, 0, 8, "MINUTE_TAG"),
    f(7, 121, 128, 0, 8, "SECOND_TAG"),
    f(8, 129, 156, 0, 8, "SPACECRAFT"),
    f(9, 157, 164, 0, 8, "NET_ID"),
    f(10, 165, 172, 0, 8, "STATION"),
    f(11, 173, 180, 0, 8, "DOWNLINK_BAND"),
    f(12, 181, 184, 0, 8, "DATA_TYPE"),
    f(13, 185, 192, 0, 8, "GROUND_MODE"),
    f(14, 193, 200, 0, 8, "RANGE_TYPE"),
    f(15, 201, 208, 0, 8, "ANGLE_TYPE"),
    f(16, 209, 216, 0, 8, "DRVID_TYPE"),
    f(17, 217, 221, 0, 8, "DOPPLER_GOOD0"),
    f(18, 222, 222, 0, 8, "DOPPLER_TOL0"),
    f(19, 223, 223, 0, 8, "ZERO_1"),
    f(20, 224, 227, -1, 16, "DOPPLER_BIAS"),
    f(21, 228, 228, 0, 8, "RESERVED_1"),
    f(22, 229, 229, 0, 8, "ANGLE_GOOD0"),
    f(23, 230, 232, 0, 8, "RESERVED_2"),
    f(24, 233, 235, 0, 8, "RESERVED_3"),
    f(25, 236, 236, 0, 8, "RCVR_LOCK0"),
    f(26, 237, 237, 0, 8, "XMTR_ON0"),
    f(27, 238, 239, 0, 8, "RESERVED_4"),
    f(28, 240, 242, 0, 8, "SOURCE_DESIG"),
    f(29, 241, 252, 0, 16, "RESERVED_5"),
    f(30, 253, 288, 0, 32, "SAMPLER_TIME"),
    f(31, 289, 324, 0, 32, "DOPPLER_CNT_HP"),
    f(32, 325, 360, 0, 32, "DOPPLER_CNT_LP"),
    f(33, 361, 396, 0, 32, "RANGE_DATA1"),
    f(34, 397, 432, 0, 32, "RANGE_DATA2"),
    f(35, 433, 452, 0, 32, "LOW_RANGE"),
    f(36, 453, 484, 0, 32, "RESERVED_6"),
    f(37, 525, 540, -1, 16, "DRVID_PNR"),
    f(38, 541, 576, 0, 32, "ANGLE1"),
    f(39, 577, 612, 0, 32, "ANGLE2"),
    f(40, 613, 648, 0, 32, "DOPPLER_REF"),
    f(41, 649, 684, -1, 32, "DRVID"),
    f(42, 685, 720, 0, 32, "DOPPLER_CNT2_HP"),
    f(43, 721, 756, 0, 32, "DOPPLER_CNT2_LP"),
    f(44, 757, 792, 0, 32, "DOPPLER_CNT3_HP"),
    f(45, 793, 828, 0, 32, "DOPPLER_CNT3_LP"),
    f(46, 829, 864, 0, 32, "DOPPLER_CNT4_HP"),
    f(47, 865, 900, 0, 32, "DOPPLER_CNT4_LP"),
    f(48, 901, 936, 0, 32, "DOPPLER_CNT5_HP"),
    f(49, 937, 972, 0, 32, "DOPPLER_CNT5_LP"),
    f(50, 973, 1008, 0, 32, "DOPPLER_CNT6_HP"),
    f(51, 1009, 1044, 0, 32, "DOPPLER_CNT6_LP"),
    f(52, 1045, 1080, 0, 32, "DOPPLER_CNT7_HP"),
    f(53, 1081, 1116, 0, 32, "DOPPLER_CNT7_LP"),
    f(54, 1117, 1152, 0, 32, "DOPPLER_CNT8_HP"),
    f(55, 1153, 1188, 0, 32, "DOPPLER_CNT8_LP"),
    f(56, 1189, 1224, 0, 32, "DOPPLER_CNT9_HP"),
    f(57, 1225, 1260, 0, 32, "DOPPLER_CNT9_LP"),
    f(58, 1261, 1296, 0, 32, "DOPPLER_CNTA_HP"),
    f(59, 1297, 1332, 0, 32, "DOPPLER_CNTA_LP"),
    f(60, 1333, 1368, -1, 32, "DOPPLER_RESID"),
    f(61, 1369, 1404, -1, 32, "RANGE_RESID"),
    f(62, 1405, 1422, -1, 32, "ANGLE1_RESID"),
    f(63, 1423, 1440, -1, 32, "ANGLE2_RESID"),
    f(64, 1441, 1443, 0, 8, "UPLINK_BAND"),
    f(65, 1444, 1446, 0, 8, "ANGLE_MODE"),
    f(66, 1447, 1448, 0, 8, "CONSCAN_MODE"),
    f(67, 1449, 1449, 0, 8, "ANGLE1_RESID_TOL0"),
    f(68, 1450, 1450, 0, 8, "ANGLE2_RESID_TOL0"),
    f(69, 1451, 1453, 0, 8, "DOPPLER_CHANNEL"),
    f(70, 1454, 1454, 0, 8, "FREQ_STD"),
    f(71, 1455, 1456, 0, 8, "DOPPLER_RCVR_REF"),
    f(72, 1457, 1462, 0, 8, "RESERVED_7"),
    f(73, 1463, 1463, 0, 8, "DOPPLER_RESID_TOL0"),
    f(74, 1464, 1464, 0, 8, "DOPPLER_NOISE_TOL0"),
    f(75, 1465, 1494, 0, 32, "RESERVED_8"),
    f(76, 1495, 1512, 0, 32, "SLIPPED_CYCLE"),
    f(77, 1513, 1530, 0, 32, "DOPPLER_NOISE"),
    f(78, 1531, 1548, -1, 32, "SIGNAL_STRENGTH"),
    f(79, 1549, 1584, -1, 32, "DIFF_DOPPLER_PHASE"),
    f(80, 1585, 1585, 0, 8, "RANGE_MOD_ON0"),
    f(81, 1586, 1586, 0, 8, "PRIME_RANGE_CHAN"),
    f(82, 1587, 1587, 0, 8, "PIPELINING_ON0"),
    f(83, 1588, 1588, 0, 8, "CHOPPER_FREQ_ON0"),
    f(84, 1589, 1589, 0, 8, "RESERVED_9"),
    f(85, 1590, 1590, 0, 8, "RANGE_VALID0"),
    f(86, 1591, 1591, 0, 8, "RANGE_CALIB_TOL0"),
    f(87, 1592, 1592, 0, 8, "RANGE_CONFIG_SAME0"),
    f(88, 1593, 1593, 0, 8, "RANGE_PNR_TOL0"),
    f(89, 1594, 1594, 0, 8, "RANGE_RESID_TOL0"),
    f(90, 1595, 1595, 0, 8, "PSEUDO_DRVID_TOL0"),
    f(91, 1596, 1596, 0, 8, "DIFF_RANGE_TOL0"),
    f(92, 1597, 1600, 0, 8, "RCVR_NUMBER"),
    f(93, 1601, 1601, 0, 8, "RESERVED_10"),
    f(94, 1602, 1603, 0, 8, "AMP_NUMBER"),
    f(95, 1604, 1605, 0, 8, "AMP_TYPE"),
    f(96, 1606, 1606, 0, 8, "XMTR_POWER_IND"),
    f(97, 1607, 1607, 0, 8, "RESERVED_11"),
    f(98, 1608, 1620, 0, 16, "XMTR_POWER"),
    f(99, 1621, 1644, 0, 32, "RANGE_CALIB"),
    f(100, 1645, 1656, -1, 16, "RANGE_PNR"),
    f(101, 1657, 1692, -1, 32, "AVG_DOPPLER_RESID"),
    f(102, 1693, 1728, -1, 32, "PSEUDO_DRVID"),
    f(103, 1729, 1764, -1, 32, "DIFF_S_X_RANGE"),
    f(104, 1765, 1786, -1, 32, "Z_CORRECTION"),
    f(105, 1787, 1786, 0, 8, "SPACECRAFT_DELAY"),
    f(106, 1801, 1833, 0, 8, "DRVID_NOISE"),
    f(107, 1834, 1834, 0, 8, "DRVID_VALID0"),
    f(108, 1835, 1835, 0, 8, "DRVID_NOISE_TOL0"),
    f(109, 1836, 1836, 0, 8, "DRIVD_PNR_TOL0"),
    f(110, 1837, 1872, -1, 32, "DIFF_S_X_DRVID"),
    f(111, 1873, 1877, 0, 8, "RAMP_CTRL"),
    f(112, 1878, 1908, -1, 32, "RAMP_RATE"),
    f(113, 1909, 1944, 0, 32, "RAMP_START1"),
    f(114, 1945, 1980, 0, 32, "RAMP_START2"),
    f(115, 1981, 2012, 0, 32, "RESERVED_12"),
    f(116, 2125, 2160, 0, 32, "XMTR_FREQ"),
];

pub fn field_of(table: &[Field], item: u32) -> Option<&Field> {
    table.iter().find(|x| x.item == item)
}

pub fn strip_markers(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() % PHYSICAL_RECORD_MARKER == 0 {
        let nrec = bytes.len() / PHYSICAL_RECORD_MARKER;
        let mut out = Vec::with_capacity(nrec * PHYSICAL_RECORD);
        for r in 0..nrec {
            let base = r * PHYSICAL_RECORD_MARKER;
            out.extend_from_slice(&bytes[base..base + PHYSICAL_RECORD]);
        }
        Some(out)
    } else if bytes.len() % PHYSICAL_RECORD == 0 {
        Some(bytes.to_vec())
    } else {
        None
    }
}

pub fn extract(record: &[u8], fld: &Field) -> i64 {
    if fld.start > fld.stop || fld.stop / 8 >= record.len() {
        return 0;
    }
    let bsize = fld.stop - fld.start + 1;
    let mmin = fld.start / 8;
    let mmax = fld.stop / 8;
    let omin = fld.start - mmin * 8;
    let omax = fld.stop - mmax * 8;
    let msize = mmax - mmin + 1;
    let mut val: u64 = 0;
    for i in 0..msize {
        val = (val << 8) | record[mmin + i] as u64;
    }
    if omin > 0 {
        let first_byte_shift = (msize - 1) * 8;
        val &= !(((1u64 << omin) - 1) << (first_byte_shift + 8 - omin));
    }
    val >>= 8 - omax - 1;
    val &= if fld.outlength >= 64 {
        u64::MAX
    } else {
        (1u64 << fld.outlength) - 1
    };
    let mut v = val as i64;

    if fld.outlength == 32 || fld.outlength == 16 {
        if v >= (1i64 << (fld.outlength - 1)) {
            v -= 1i64 << fld.outlength;
        }
    }
    if fld.signlength == -1 && fld.outlength > bsize && v >= (1i64 << (bsize - 1)) {
        v -= 1i64 << bsize;
    }
    v
}

pub struct Tracking {
    pub year: i64,
    pub day: i64,
    pub hour: i64,
    pub minute: i64,
    pub second: i64,
    pub spacecraft: i64,
    pub data_type: i64,
    pub ground_mode: i64,
    pub station: i64,
    pub doppler_bias: i64,
    pub sampler_time: i64,
    pub doppler_cnt_hp: i64,
    pub doppler_cnt_lp: i64,
    pub doppler_ref: i64,
    pub doppler_resid: i64,
    pub ramp_rate: i64,
    pub slipped_cycle: i64,
    pub signal_strength: i64,
}

pub fn tracking_record(rec: &[u8]) -> Tracking {
    Tracking {
        year: extract(rec, &TKFORM[2]),
        day: extract(rec, &TKFORM[3]),
        hour: extract(rec, &TKFORM[4]),
        minute: extract(rec, &TKFORM[5]),
        second: extract(rec, &TKFORM[6]),
        spacecraft: extract(rec, &TKFORM[7]),
        data_type: extract(rec, &TKFORM[11]),
        ground_mode: extract(rec, &TKFORM[12]),
        station: extract(rec, &TKFORM[9]),
        doppler_bias: extract(rec, &TKFORM[19]),
        sampler_time: extract(rec, &TKFORM[29]),
        doppler_cnt_hp: extract(rec, &TKFORM[30]),
        doppler_cnt_lp: extract(rec, &TKFORM[31]),
        doppler_ref: extract(rec, &TKFORM[39]),
        doppler_resid: extract(rec, &TKFORM[59]),
        ramp_rate: extract(rec, &TKFORM[111]),
        slipped_cycle: extract(rec, &TKFORM[75]),
        signal_strength: extract(rec, &TKFORM[77]),
    }
}

pub fn full_year(two: i64) -> i64 {
    if two < 70 {
        2000 + two
    } else {
        1900 + two
    }
}

pub struct SkySample {
    pub tdb_s: f64,
    pub fsky_hz: f64,
    pub doppler_ref_hz: f64,
    pub sampler_time_s: f64,
    pub doppler_bias: f64,
    pub data_type: f64,
    pub station: f64,
    pub doppler_cnt: f64,
    pub doppler_resid: f64,
    pub slipped_cycle: f64,
    pub signal_strength: f64,
    pub ramp_rate: f64,
    pub file_id: f64,
}

pub fn write_bin(records: &[[f64; 14]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 112);
    out.extend_from_slice(b"PASF");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<[f64; 14]>> {
    if data.len() < 8 || &data[0..4] != b"PASF" {
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

#[cfg(test)]
mod tests {
    use super::*;

    const FILE_RECORD_HEAD: [u8; 64] = [
        0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x0a, 0x05, 0xa0, 0x0e, 0xa0, 0xb0, 0x03,
        0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn extracts_file_record_date_from_known_bytes() {
        let mut rec = [0u8; LOGICAL_RECORD];
        rec[..64].copy_from_slice(&FILE_RECORD_HEAD);
        assert_eq!(extract(&rec, field_of(IDFORM, 3).unwrap()), 90);
        assert_eq!(extract(&rec, field_of(IDFORM, 4).unwrap()), 234);
        assert_eq!(extract(&rec, field_of(IDFORM, 5).unwrap()), 11);
        assert_eq!(extract(&rec, field_of(IDFORM, 6).unwrap()), 3);
        assert_eq!(extract(&rec, field_of(IDFORM, 7).unwrap()), 30);
    }

    #[test]
    fn sign_extends_two_complement_ramp_rate() {
        let field = field_of(TKFORM, 112).unwrap();
        let mut rec = [0u8; LOGICAL_RECORD];
        let bmin = field.start / 8;
        let bmax = field.stop / 8;
        let msize = bmax - bmin + 1;
        for i in 0..msize {
            rec[bmin + i] = 0xff;
        }
        let v = extract(&rec, field);
        assert!(v < 0, "31-bit two's complement of all ones is negative");
    }

    #[test]
    fn strip_markers_rejects_unaligned_sizes() {
        let ok = vec![0u8; PHYSICAL_RECORD * 2];
        assert_eq!(
            strip_markers(&ok).map(|v| v.len()),
            Some(PHYSICAL_RECORD * 2)
        );
        let marked = vec![0u8; PHYSICAL_RECORD_MARKER * 2];
        assert_eq!(
            strip_markers(&marked).map(|v| v.len()),
            Some(PHYSICAL_RECORD * 2)
        );
        assert!(strip_markers(&[0u8; 1000]).is_none());
    }

    #[test]
    fn signed_36bit_field_reads_two_complement() {
        let field = field_of(TKFORM, 60).unwrap();
        let mut rec = [0u8; LOGICAL_RECORD];
        let mmin = field.start / 8;
        let mmax = field.stop / 8;
        for i in mmin..=mmax {
            rec[i] = 0xff;
        }
        assert_eq!(
            extract(&rec, field),
            -1,
            "36-bit field of all ones is −1, not 2³²−1"
        );
    }

    #[test]
    fn pasf_roundtrip_carries_fourteen_slots() {
        let records = vec![
            [
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, -2.0, 91.0, 12.0, 3.0, 2.0,
            ],
            [
                10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 0.0, 0.0, 0.0, 0.0, 3.0,
            ],
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
        assert!(
            parse_bin(&bytes[..bytes.len() - 1]).is_none(),
            "truncated data stays uncarried"
        );
        assert!(parse_bin(b"PASF").is_none());
    }
}
