pub const PHYSICAL_RECORD: usize = 8064;
pub const PHYSICAL_RECORD_MARKER: usize = 8065;
pub const LOGICAL_RECORD: usize = 288;

pub const S_BAND_REF_LO: f64 = 2197e4;
pub const S_BAND_REF_HI: f64 = 2200e4;
pub const FSKY_BASE_HZ: f64 = 2292e6;

pub const DTYPE_ONEWAY_DOPPLER: i64 = 1;
pub const DTYPE_TWOWAY_DOPPLER: i64 = 2;
pub const DTYPE_THREEWAY_DOPPLER: i64 = 3;
pub const DTYPE_RAMP: i64 = 6;

pub const ULY_BAND_S: i64 = 1;
pub const ULY_BAND_X: i64 = 2;

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

pub const XPFORM8: &[Field] = &[
    f(1, 1, 32, 0, 32, "RECORD_FORMAT"),
    f(4, 73, 84, 0, 8, "XPON_ON_YEAR"),
    f(5, 85, 100, 0, 16, "XPON_ON_DOY"),
    f(6, 101, 108, 0, 8, "XPON_ON_HOUR"),
    f(7, 109, 120, 0, 8, "XPON_ON_MINUTE"),
    f(8, 121, 128, 0, 8, "XPON_ON_SECOND"),
    f(10, 141, 156, 0, 16, "SPACECRAFT"),
    f(14, 181, 192, 0, 8, "XPON_OFF_YEAR"),
    f(15, 193, 208, 0, 16, "XPON_OFF_DOY"),
    f(16, 209, 216, 0, 8, "XPON_OFF_HOUR"),
    f(17, 217, 228, 0, 8, "XPON_OFF_MINUTE"),
    f(18, 229, 236, 0, 8, "XPON_OFF_SECOND"),
    f(21, 265, 288, 0, 24, "SC_XPON_HP"),
    f(23, 301, 324, 0, 24, "SC_XPON_LP"),
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

pub const FMT_TRK_225: u8 = 4;
pub const FMT_SFOC_NAV_225: u8 = 8;

const FMT_BITS: Field = f(1, 1, 32, 0, 32, "RECORD_FORMAT");

pub fn record_format(rec: &[u8]) -> Option<u8> {
    match extract(rec, &FMT_BITS) {
        v if v == FMT_TRK_225 as i64 => Some(FMT_TRK_225),
        v if v == FMT_SFOC_NAV_225 as i64 => Some(FMT_SFOC_NAV_225),
        _ => None,
    }
}

pub const TKFORM8: &[Field] = &[
    f(1, 1, 32, 0, 32, "RECORD_FORMAT"),
    f(3, 41, 72, 0, 32, "RECORD_TYPE"),
    f(4, 73, 84, 0, 8, "SAMPLE_YEAR"),
    f(5, 85, 100, 0, 16, "SAMPLE_DOY"),
    f(6, 101, 108, 0, 8, "SAMPLE_HOUR"),
    f(7, 109, 116, 0, 8, "SAMPLE_MINUTE"),
    f(8, 117, 124, 0, 8, "SAMPLE_SECOND"),
    f(10, 145, 154, 0, 32, "RECEIVING_STATION"),
    f(11, 155, 162, 0, 8, "DOWNLINK_BAND"),
    f(12, 163, 168, 0, 8, "DATA_TYPE"),
    f(14, 173, 176, 0, 8, "GROUND_MODE"),
    f(15, 177, 192, 0, 16, "SPACECRAFT"),
    f(20, 218, 235, -1, 32, "DOPPLER_BIAS"),
    f(29, 257, 288, 0, 32, "SAMPLE_INTERVAL"),
    f(30, 289, 312, 0, 24, "DOPPLER_CNT_HP"),
    f(31, 313, 336, 0, 24, "DOPPLER_CNT_IP"),
    f(32, 337, 360, 0, 24, "DOPPLER_CNT_LP"),
    f(43, 589, 620, 0, 32, "DOPPLER_REF_HP"),
    f(44, 621, 652, 0, 32, "DOPPLER_REF_LP"),
    f(74, 1337, 1368, 0, 32, "DOPPLER_RESID"),
    f(87, 1467, 1476, 0, 32, "SLIPPED_CYCLE"),
    f(89, 1495, 1512, -1, 32, "SIGNAL_STRENGTH"),
    f(120, 1809, 1840, 0, 32, "RAMP_RATE_HP"),
    f(121, 1841, 1872, 0, 32, "RAMP_RATE_LP"),
    f(22, 237, 237, 0, 8, "FREQ_LEVEL"),
    f(140, 1959, 1986, 0, 32, "XMTR_REF_HP"),
    f(141, 1987, 2016, 0, 32, "XMTR_REF_LP"),
];

pub fn field_of(table: &[Field], item: u32) -> Option<&Field> {
    table.iter().find(|x| x.item == item)
}

pub fn strip_markers(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len().is_multiple_of(PHYSICAL_RECORD_MARKER) {
        let nrec = bytes.len() / PHYSICAL_RECORD_MARKER;
        let mut out = Vec::with_capacity(nrec * PHYSICAL_RECORD);
        for r in 0..nrec {
            let base = r * PHYSICAL_RECORD_MARKER;
            out.extend_from_slice(&bytes[base..base + PHYSICAL_RECORD]);
        }
        Some(out)
    } else if bytes.len().is_multiple_of(PHYSICAL_RECORD) {
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

    if (fld.outlength == 32 || fld.outlength == 16) && v >= (1i64 << (fld.outlength - 1)) {
        v -= 1i64 << fld.outlength;
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
    pub ref_sky: Option<bool>,
    pub xmtr_ref: Option<i64>,
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
        ref_sky: None,
        xmtr_ref: None,
    }
}

pub fn tracking_record_f8(rec: &[u8]) -> Tracking {
    let cnt_hp = extract(rec, &TKFORM8[14]);
    let cnt_ip = extract(rec, &TKFORM8[15]);
    let cnt_lp = extract(rec, &TKFORM8[16]);
    let ref_hp = extract(rec, &TKFORM8[17]);
    let ref_lp = extract(rec, &TKFORM8[18]);
    let ramp_hp = extract(rec, &TKFORM8[22]);
    let ramp_lp = extract(rec, &TKFORM8[23]);
    let xmtr_hp = extract(rec, &TKFORM8[25]);
    let xmtr_lp = extract(rec, &TKFORM8[26]);
    Tracking {
        year: extract(rec, &TKFORM8[2]),
        day: extract(rec, &TKFORM8[3]),
        hour: extract(rec, &TKFORM8[4]),
        minute: extract(rec, &TKFORM8[5]),
        second: extract(rec, &TKFORM8[6]),
        spacecraft: extract(rec, &TKFORM8[11]),
        data_type: extract(rec, &TKFORM8[9]),
        ground_mode: extract(rec, &TKFORM8[10]),
        station: extract(rec, &TKFORM8[7]),
        doppler_bias: extract(rec, &TKFORM8[12]) / 1000,
        sampler_time: extract(rec, &TKFORM8[13]),
        doppler_cnt_hp: cnt_hp * 10_000 + cnt_ip / 1_000,
        doppler_cnt_lp: (cnt_ip % 1_000) * 10_000 + (cnt_lp + 500) / 1_000,
        doppler_ref: ref_hp * 10_000 + (ref_lp + 50_000) / 100_000,
        doppler_resid: extract(rec, &TKFORM8[19]),
        ramp_rate: ramp_hp * 1_000_000_000 + ramp_lp,
        slipped_cycle: extract(rec, &TKFORM8[20]),
        signal_strength: extract(rec, &TKFORM8[21]),
        ref_sky: Some(extract(rec, &TKFORM8[24]) == 1),
        xmtr_ref: Some(xmtr_hp * 10_000 + (xmtr_lp + 50_000) / 100_000),
    }
}

pub fn full_year(two: i64) -> i64 {
    if two < 70 { 2000 + two } else { 1900 + two }
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

pub fn write_skyfreq_bin(records: &[[f64; 14]], magic: &[u8; 4]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 112);
    out.extend_from_slice(magic);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_skyfreq_bin(data: &[u8], magic: &[u8; 4]) -> Option<Vec<[f64; 14]>> {
    if data.len() < 8 || &data[0..4] != magic {
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

pub fn write_bin(records: &[[f64; 14]]) -> Vec<u8> {
    write_skyfreq_bin(records, b"PASF")
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<[f64; 14]>> {
    parse_skyfreq_bin(data, b"PASF")
}

pub const COMP_SKYFREQ: u32 = 0;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_SKYFREQ => Some("pioneer_sky_frequency_hz"),
        _ => None,
    }
}

pub const COMP_ULY_SKYFREQ: u32 = 1;
pub const COMP_ULY_SKYFREQ_X: u32 = 2;

pub fn uly_component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_ULY_SKYFREQ => Some("ulysses_sky_frequency_hz"),
        COMP_ULY_SKYFREQ_X => Some("ulysses_sky_frequency_x_hz"),
        _ => None,
    }
}

pub const COMP_GLL_SKYFREQ: u32 = 3;
pub const COMP_GLL_SKYFREQ_X: u32 = 4;

pub fn gll_component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_GLL_SKYFREQ => Some("gll_rss_atdf_sky_frequency_hz"),
        COMP_GLL_SKYFREQ_X => Some("gll_rss_atdf_x_sky_frequency_hz"),
        _ => None,
    }
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_bin(bytes)?;
    let out: Vec<(f64, f64, u32)> = rows
        .into_iter()
        .filter(|r| r[0].is_finite() && r[1].is_finite() && r[1] > 0.0)
        .map(|r| (r[0], r[1], COMP_SKYFREQ))
        .collect();
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_uly_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_bin(bytes)?;
    let out: Vec<(f64, f64, u32)> = rows
        .into_iter()
        .filter(|r| r[0].is_finite() && r[1].is_finite() && r[1] > 0.0)
        .map(|r| (r[0], r[1], COMP_ULY_SKYFREQ))
        .collect();
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_uly_series_x(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_bin(bytes)?;
    let out: Vec<(f64, f64, u32)> = rows
        .into_iter()
        .filter(|r| r[0].is_finite() && r[1].is_finite() && r[1] > 0.0)
        .map(|r| (r[0], r[1], COMP_ULY_SKYFREQ_X))
        .collect();
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_gll_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_bin(bytes)?;
    let out: Vec<(f64, f64, u32)> = rows
        .into_iter()
        .filter(|r| r[0].is_finite() && r[1].is_finite() && r[1] > 0.0)
        .map(|r| (r[0], r[1], COMP_GLL_SKYFREQ))
        .collect();
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_gll_series_x(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_bin(bytes)?;
    let out: Vec<(f64, f64, u32)> = rows
        .into_iter()
        .filter(|r| r[0].is_finite() && r[1].is_finite() && r[1] > 0.0)
        .map(|r| (r[0], r[1], COMP_GLL_SKYFREQ_X))
        .collect();
    if out.is_empty() { None } else { Some(out) }
}

pub const S_BAND_RATIO: f64 = 96.0 * 240.0 / 221.0;
pub const RATE_OFFSET: f64 = 1e6;
pub const FSKY_MED_HALF_WIDTH: f64 = 0.6e6;
pub const GAP_DAY: f64 = 0.1;

pub const S_BAND_TURNAROUND: f64 = 240.0 / 221.0;
pub const S_UPLINK_REF_LO: f64 = 2110e6;
pub const S_UPLINK_REF_HI: f64 = 2120e6;
pub const F8_DOPPLER_RATE_HALF_WIDTH: f64 = 524_288.0;
pub const F8_FSKY_MED_HALF_WIDTH: f64 = 2_097_152.0;

pub const X_BAND_RATIO: f64 = 96.0 * 880.0 / 221.0;
pub const X_FSKY_BASE_HZ: f64 = 8408.209876e6;
pub const X_BAND_REF_LO: f64 = 2197e4;
pub const X_BAND_REF_HI: f64 = 2200e4;
pub const X_FSKY_MED_HALF_WIDTH: f64 = FSKY_MED_HALF_WIDTH * X_BAND_RATIO / S_BAND_RATIO;

fn tdb_of(tr: &Tracking, lsk: &crate::lsk::LeapSeconds) -> Option<f64> {
    if tr.day <= 0 || tr.day > 366 {
        return None;
    }
    let year = full_year(tr.year);
    let days = crate::lsk::days_from_civil(year, 1, 1)? + tr.day - 1;
    let unix = days as f64 * 86400.0
        + tr.hour as f64 * 3600.0
        + tr.minute as f64 * 60.0
        + tr.second as f64;
    lsk.unix_to_tdb(unix)
}

fn header_of(stripped: &[u8], fmt: u8) -> (i64, f64, f64) {
    let rec0 = &stripped[0..LOGICAL_RECORD];
    let rec1 = &stripped[LOGICAL_RECORD..2 * LOGICAL_RECORD];
    let year = extract(rec0, &IDFORM[2]);
    let day = extract(rec0, &IDFORM[3]);
    let (sc, xpon_hp, xpon_lp) = if fmt == FMT_SFOC_NAV_225 {
        (
            extract(rec1, &XPFORM8[6]),
            extract(rec1, &XPFORM8[12]),
            extract(rec1, &XPFORM8[13]),
        )
    } else {
        (
            extract(rec1, &XPFORM[7]),
            extract(rec1, &XPFORM[13]),
            extract(rec1, &XPFORM[14]),
        )
    };
    let xpon = xpon_hp as f64 * 1e4 + xpon_lp as f64 / 1e3;
    (sc, full_year(year) as f64 + (day - 1) as f64 / 366.0, xpon)
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.is_empty() {
        return f64::NAN;
    }
    v[v.len() / 2]
}

pub fn reduce_skyfreq(
    name: &str,
    file_id: f64,
    bytes: &[u8],
    lsk: &crate::lsk::LeapSeconds,
    ref_lo: f64,
    ref_hi: f64,
) -> Option<Vec<[f64; 14]>> {
    let stripped = strip_markers(bytes)?;
    let nlog = stripped.len() / LOGICAL_RECORD;
    if nlog < 3 {
        eprintln!("{name}: {nlog} logical records — too short");
        return None;
    }
    let (sc, file_year, xpon) = header_of(&stripped, FMT_TRK_225);
    let mut recs: Vec<Tracking> = Vec::with_capacity(nlog - 2);
    let mut skipped_zero = 0usize;
    for i in 2..nlog {
        let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
        let tr = tracking_record(rec);
        if tr.day == 0 {
            skipped_zero += 1;
            continue;
        }
        recs.push(tr);
    }
    if recs.len() < 2 {
        eprintln!("{name}: {} tracking records — too short", recs.len());
        return None;
    }
    let mut n = recs.len();
    let mut t = vec![0.0f64; n];
    let mut dcnt = vec![0.0f64; n];
    let mut ref_hz = vec![0.0f64; n];
    let mut bias = vec![0i64; n];
    let mut sampler = vec![0.0f64; n];
    let mut dtype = vec![0i64; n];
    let mut mode = vec![0i64; n];
    let mut station = vec![0i64; n];
    let mut resid = vec![0.0f64; n];
    let mut slipped = vec![0i64; n];
    let mut strength = vec![0i64; n];
    let mut ramp = vec![0i64; n];
    let mut kept = 0usize;
    for tr in recs.iter() {
        let Some(tdb) = tdb_of(tr, lsk) else {
            continue;
        };
        let r = tr.doppler_ref as f64 / 10.0;
        let s = tr.sampler_time as f64 / 100.0;
        t[kept] = tdb;
        dcnt[kept] = tr.doppler_cnt_hp as f64 * 1e4 + tr.doppler_cnt_lp as f64 / 1e3;
        ref_hz[kept] = r;
        bias[kept] = tr.doppler_bias;
        sampler[kept] = s;
        dtype[kept] = tr.data_type;
        mode[kept] = tr.ground_mode;
        station[kept] = tr.station;
        resid[kept] = tr.doppler_resid as f64 / 1000.0;
        slipped[kept] = tr.slipped_cycle;
        strength[kept] = tr.signal_strength;
        ramp[kept] = tr.ramp_rate;
        kept += 1;
    }
    n = kept;
    if n < 2 {
        eprintln!("{name}: {n} timestamped records — too short");
        return None;
    }
    t.truncate(n);
    dcnt.truncate(n);
    ref_hz.truncate(n);
    bias.truncate(n);
    sampler.truncate(n);
    dtype.truncate(n);
    mode.truncate(n);
    station.truncate(n);
    resid.truncate(n);
    slipped.truncate(n);
    strength.truncate(n);
    ramp.truncate(n);

    let ref_min = ref_hz.iter().copied().fold(f64::INFINITY, f64::min);
    let ref_max = ref_hz.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut fsky = vec![0.0f64; n.saturating_sub(1)];
    let mut good = vec![false; n.saturating_sub(1)];
    let mut dtype_hist: Vec<(i64, usize)> = Vec::new();
    for i in 0..n - 1 {
        let hist = dtype_hist.iter_mut().find(|(d, _)| *d == dtype[i]);
        match hist {
            Some((_, c)) => *c += 1,
            None => dtype_hist.push((dtype[i], 1)),
        }
        if !(dtype[i] == DTYPE_ONEWAY_DOPPLER || dtype[i] == DTYPE_TWOWAY_DOPPLER)
            || sampler[i] <= 0.0
        {
            continue;
        }
        let doff = if dcnt[i + 1] < dcnt[i] {
            2f64.powi(32)
        } else {
            0.0
        };
        let drate = (dcnt[i + 1] + doff - dcnt[i]) / sampler[i];
        let sdoppler = if bias[i] >= 0 { 1.0 } else { -1.0 };
        fsky[i] = S_BAND_RATIO * ref_hz[i] - sdoppler * (drate - RATE_OFFSET);
        good[i] = true;
    }
    let mut fsky_finite: Vec<f64> = fsky.iter().copied().filter(|x| x.is_finite()).collect();
    let fmed = median(&mut fsky_finite);
    let mut out: Vec<[f64; 14]> = Vec::new();
    let mut ramp_records = 0usize;
    let mut bias_rejected = 0usize;
    let mut ref_rejected = 0usize;
    let mut gap_rejected = 0usize;
    let mut wrap_rejected = 0usize;
    let mut med_rejected = 0usize;
    for i in 0..n - 1 {
        if !good[i] {
            if dtype[i] == DTYPE_RAMP {
                ramp_records += 1;
            }
            continue;
        }
        if bias[i].abs() > 1 {
            bias_rejected += 1;
            continue;
        }
        if !(ref_lo..=ref_hi).contains(&ref_hz[i]) {
            ref_rejected += 1;
            continue;
        }
        let gap_days = (t[i + 1] - t[i]) / 86400.0;
        if gap_days >= GAP_DAY {
            gap_rejected += 1;
            continue;
        }
        if dcnt[i + 1] <= dcnt[i] {
            wrap_rejected += 1;
            continue;
        }
        if (fsky[i] - fmed).abs() >= FSKY_MED_HALF_WIDTH {
            med_rejected += 1;
            continue;
        }
        out.push([
            t[i],
            fsky[i],
            ref_hz[i],
            sampler[i],
            bias[i] as f64,
            dtype[i] as f64,
            station[i] as f64,
            dcnt[i],
            resid[i],
            slipped[i] as f64,
            strength[i] as f64,
            ramp[i] as f64,
            file_id,
            mode[i] as f64,
        ]);
    }
    let n_out = out.len();
    let n_slipped = out.iter().filter(|r| r[9] != 0.0).count();
    let mut stations: Vec<i64> = out.iter().map(|r| r[6] as i64).collect();
    stations.sort_unstable();
    stations.dedup();
    dtype_hist.sort_by_key(|(d, _)| *d);
    eprintln!(
        "{name}: SC {sc}, file year {file_year:.1}, Xponder {xpon:.3e} Hz, {n} tracking records ({skipped_zero} null records), dtype {dtype_hist:?}, {n_out} fsky samples (median {fmed:.6e} Hz), ref {ref_min:.3e}..{ref_max:.3e} Hz, {n_slipped} with slipped cycle, stations {stations:?} — separated: {ramp_records} ramp, {bias_rejected} bias, {ref_rejected} ref, {gap_rejected} gap, {wrap_rejected} wrap, {med_rejected} median"
    );
    if out.is_empty() { None } else { Some(out) }
}

pub struct UlySkyFreq {
    pub sband: Vec<[f64; 14]>,
    pub xband: Vec<[f64; 14]>,
    pub n: usize,
    pub no_pair: usize,
    pub no_doppler: usize,
    pub bias_rejected: usize,
    pub ref_rejected: usize,
    pub gap_rejected: usize,
    pub wrap_rejected: usize,
    pub rate_rejected: usize,
    pub med_rejected: usize,
}

pub fn reduce_uly_skyfreq(
    name: &str,
    file_id: f64,
    bytes: &[u8],
    lsk: &crate::lsk::LeapSeconds,
) -> Option<UlySkyFreq> {
    let stripped = strip_markers(bytes)?;
    let nlog = stripped.len() / LOGICAL_RECORD;
    if nlog < 3 {
        eprintln!("{name}: {nlog} logical records — too short");
        return None;
    }
    let band_field4 = &TKFORM[10];
    let band_field8 = &TKFORM8[8];
    let mut file_fmt: Option<u8> = None;
    let mut recs: Vec<(Tracking, i64, u8)> = Vec::with_capacity(nlog - 2);
    let mut skipped_zero = 0usize;
    let mut skipped_unknown = 0usize;
    let mut n_fmt4 = 0usize;
    let mut n_fmt8 = 0usize;
    let mut n_sband = 0usize;
    let mut n_xband = 0usize;
    for i in 2..nlog {
        let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
        let Some(fmt) = record_format(rec) else {
            skipped_unknown += 1;
            continue;
        };
        file_fmt.get_or_insert(fmt);
        let (tr, band) = if fmt == FMT_SFOC_NAV_225 {
            n_fmt8 += 1;
            (tracking_record_f8(rec), extract(rec, band_field8))
        } else {
            n_fmt4 += 1;
            (tracking_record(rec), extract(rec, band_field4))
        };
        if tr.day == 0 {
            skipped_zero += 1;
            continue;
        }
        match band {
            ULY_BAND_S => n_sband += 1,
            ULY_BAND_X => n_xband += 1,
            _ => {}
        }
        recs.push((tr, band, fmt));
    }
    if recs.len() < 2 {
        eprintln!("{name}: {} tracking records — too short", recs.len());
        return None;
    }
    let fmt = file_fmt?;
    let (sc, file_year, xpon) = header_of(&stripped, fmt);
    let mut n = recs.len();
    let mut t = vec![0.0f64; n];
    let mut dcnt = vec![0.0f64; n];
    let mut ref_hz = vec![0.0f64; n];
    let mut bias = vec![0i64; n];
    let mut sampler = vec![0.0f64; n];
    let mut dtype = vec![0i64; n];
    let mut mode = vec![0i64; n];
    let mut station = vec![0i64; n];
    let mut resid = vec![0.0f64; n];
    let mut slipped = vec![0i64; n];
    let mut strength = vec![0i64; n];
    let mut ramp = vec![0i64; n];
    let mut bands = vec![0i64; n];
    let mut fmts = vec![0u8; n];
    let mut uplink_ref = vec![false; n];
    let mut kept = 0usize;
    let mut bias8_hist: Vec<(i64, usize)> = Vec::new();
    let mut ref8_khz_hist: Vec<(i64, usize)> = Vec::new();
    let mut mode8_hist: Vec<(i64, usize)> = Vec::new();
    let mut xmtr8_khz_hist: Vec<(i64, usize)> = Vec::new();
    for (tr, band, fmt) in recs.iter() {
        let Some(tdb) = tdb_of(tr, lsk) else {
            continue;
        };
        let threeway = *fmt == FMT_SFOC_NAV_225 && (tr.ground_mode == 3 || tr.ground_mode == 4);
        if *fmt == FMT_SFOC_NAV_225
            && (tr.data_type == DTYPE_ONEWAY_DOPPLER || tr.data_type == DTYPE_TWOWAY_DOPPLER)
        {
            match bias8_hist.iter_mut().find(|(b, _)| *b == tr.doppler_bias) {
                Some((_, c)) => *c += 1,
                None => bias8_hist.push((tr.doppler_bias, 1)),
            }
            match mode8_hist.iter_mut().find(|(m, _)| *m == tr.ground_mode) {
                Some((_, c)) => *c += 1,
                None => mode8_hist.push((tr.ground_mode, 1)),
            }
            if (tr.ground_mode == 3 || tr.ground_mode == 4)
                && let Some(xmtr_ref) = tr.xmtr_ref
            {
                let khz = (xmtr_ref as f64 / 10_000.0).round() as i64;
                match xmtr8_khz_hist.iter_mut().find(|(k, _)| *k == khz) {
                    Some((_, c)) => *c += 1,
                    None => xmtr8_khz_hist.push((khz, 1)),
                }
            }
        }
        if *fmt == FMT_SFOC_NAV_225
            && (tr.data_type == DTYPE_ONEWAY_DOPPLER || tr.data_type == DTYPE_TWOWAY_DOPPLER)
            && tr.ref_sky == Some(true)
        {
            let khz = (tr.doppler_ref as f64 / 10_000.0).round() as i64;
            match ref8_khz_hist.iter_mut().find(|(k, _)| *k == khz) {
                Some((_, c)) => *c += 1,
                None => ref8_khz_hist.push((khz, 1)),
            }
        }
        let r = match (threeway, tr.xmtr_ref) {
            (true, Some(x)) => x as f64 / 10.0,
            (true, None) => continue,
            (false, _) => tr.doppler_ref as f64 / 10.0,
        };
        let s = tr.sampler_time as f64 / 100.0;
        t[kept] = tdb;
        dcnt[kept] = tr.doppler_cnt_hp as f64 * 1e4 + tr.doppler_cnt_lp as f64 / 1e3;
        ref_hz[kept] = r;
        bias[kept] = tr.doppler_bias;
        sampler[kept] = s;
        dtype[kept] = tr.data_type;
        mode[kept] = tr.ground_mode;
        station[kept] = tr.station;
        resid[kept] = tr.doppler_resid as f64 / 1000.0;
        slipped[kept] = tr.slipped_cycle;
        strength[kept] = tr.signal_strength;
        ramp[kept] = tr.ramp_rate;
        bands[kept] = *band;
        fmts[kept] = *fmt;
        uplink_ref[kept] = *fmt == FMT_SFOC_NAV_225 && (tr.ref_sky == Some(true) || threeway);
        kept += 1;
    }
    n = kept;
    if n < 2 {
        eprintln!("{name}: {n} timestamped records — too short");
        return None;
    }
    t.truncate(n);
    dcnt.truncate(n);
    ref_hz.truncate(n);
    bias.truncate(n);
    sampler.truncate(n);
    dtype.truncate(n);
    mode.truncate(n);
    station.truncate(n);
    resid.truncate(n);
    slipped.truncate(n);
    strength.truncate(n);
    ramp.truncate(n);
    bands.truncate(n);
    fmts.truncate(n);
    uplink_ref.truncate(n);

    let (ref4_min, ref4_max, ref8_min, ref8_max) = ref_hz.iter().zip(fmts.iter()).fold(
        (
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ),
        |(r4lo, r4hi, r8lo, r8hi), (&r, &f)| {
            if f == FMT_SFOC_NAV_225 {
                (r4lo, r4hi, r8lo.min(r), r8hi.max(r))
            } else {
                (r4lo.min(r), r4hi.max(r), r8lo, r8hi)
            }
        },
    );
    bias8_hist.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
    bias8_hist.truncate(8);
    ref8_khz_hist.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
    ref8_khz_hist.truncate(8);
    mode8_hist.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
    mode8_hist.truncate(8);
    xmtr8_khz_hist.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
    xmtr8_khz_hist.truncate(8);
    let mut next_same = vec![n; n];
    let mut last_s = n;
    let mut last_x = n;
    for i in (0..n).rev() {
        next_same[i] = match bands[i] {
            ULY_BAND_S => last_s,
            ULY_BAND_X => last_x,
            _ => n,
        };
        match bands[i] {
            ULY_BAND_S => last_s = i,
            ULY_BAND_X => last_x = i,
            _ => {}
        }
    }
    let mut fsky = vec![0.0f64; n];
    let mut good = vec![false; n];
    let mut dtype_hist: Vec<(i64, usize)> = Vec::new();
    for i in 0..n {
        let hist = dtype_hist.iter_mut().find(|(d, _)| *d == dtype[i]);
        match hist {
            Some((_, c)) => *c += 1,
            None => dtype_hist.push((dtype[i], 1)),
        }
        if !(dtype[i] == DTYPE_ONEWAY_DOPPLER || dtype[i] == DTYPE_TWOWAY_DOPPLER) {
            continue;
        }
        let j = next_same[i];
        if j == n || t[j] <= t[i] {
            continue;
        }
        let doff = if dcnt[j] < dcnt[i] {
            2f64.powi(32)
        } else {
            0.0
        };
        let drate = (dcnt[j] + doff - dcnt[i]) / (t[j] - t[i]);
        let sdoppler = if bias[i] >= 0 { 1.0 } else { -1.0 };
        let ratio = if bands[i] == ULY_BAND_X {
            X_BAND_RATIO
        } else if uplink_ref[i] {
            S_BAND_TURNAROUND
        } else {
            S_BAND_RATIO
        };
        fsky[i] = ratio * ref_hz[i] - sdoppler * (drate - RATE_OFFSET);
        good[i] = true;
    }
    let mut fsky_s: Vec<f64> = Vec::new();
    let mut fsky_x: Vec<f64> = Vec::new();
    for i in 0..n {
        if !good[i] || !fsky[i].is_finite() {
            continue;
        }
        if bands[i] == ULY_BAND_X {
            fsky_x.push(fsky[i]);
        } else {
            fsky_s.push(fsky[i]);
        }
    }
    let fmed_s = median(&mut fsky_s);
    let fmed_x = median(&mut fsky_x);
    let mut out_s: Vec<[f64; 14]> = Vec::new();
    let mut out_x: Vec<[f64; 14]> = Vec::new();
    let mut ramp_records = 0usize;
    let mut no_pair = 0usize;
    let mut no_doppler = 0usize;
    let mut bias_rejected = 0usize;
    let mut ref_rejected = 0usize;
    let mut gap_rejected = 0usize;
    let mut wrap_rejected = 0usize;
    let mut rate_rejected = 0usize;
    let mut med_rejected = 0usize;
    let mut unpaired_mode8_hist: Vec<(i64, usize)> = Vec::new();
    for i in 0..n {
        if !good[i] {
            if dtype[i] == DTYPE_RAMP {
                ramp_records += 1;
            }
            if dtype[i] == DTYPE_ONEWAY_DOPPLER || dtype[i] == DTYPE_TWOWAY_DOPPLER {
                no_pair += 1;
                match unpaired_mode8_hist.iter_mut().find(|(m, _)| *m == mode[i]) {
                    Some((_, c)) => *c += 1,
                    None => unpaired_mode8_hist.push((mode[i], 1)),
                }
            } else {
                no_doppler += 1;
            }
            continue;
        }
        let j = next_same[i];
        if bias[i].abs() > 1 {
            bias_rejected += 1;
            continue;
        }
        let is_x = bands[i] == ULY_BAND_X;
        let (ref_lo, ref_hi) = if is_x {
            (X_BAND_REF_LO, X_BAND_REF_HI)
        } else if uplink_ref[i] {
            (S_UPLINK_REF_LO, S_UPLINK_REF_HI)
        } else {
            (S_BAND_REF_LO, S_BAND_REF_HI)
        };
        if !(ref_lo..=ref_hi).contains(&ref_hz[i]) {
            ref_rejected += 1;
            continue;
        }
        let gap_days = (t[j] - t[i]) / 86400.0;
        if gap_days >= GAP_DAY {
            gap_rejected += 1;
            continue;
        }
        if dcnt[j] <= dcnt[i] {
            wrap_rejected += 1;
            continue;
        }
        if fmts[i] == FMT_SFOC_NAV_225 {
            let drate = (dcnt[j] - dcnt[i]) / (t[j] - t[i]);
            if (drate - RATE_OFFSET).abs() > F8_DOPPLER_RATE_HALF_WIDTH {
                rate_rejected += 1;
                continue;
            }
        }
        let half_width = if is_x {
            X_FSKY_MED_HALF_WIDTH
        } else if fmts[i] == FMT_SFOC_NAV_225 {
            F8_FSKY_MED_HALF_WIDTH
        } else {
            FSKY_MED_HALF_WIDTH
        };
        let fmed = if is_x { fmed_x } else { fmed_s };
        if (fsky[i] - fmed).abs() >= half_width {
            med_rejected += 1;
            continue;
        }
        let row = [
            t[i],
            fsky[i],
            ref_hz[i],
            sampler[i],
            bias[i] as f64,
            dtype[i] as f64,
            station[i] as f64,
            dcnt[i],
            resid[i],
            slipped[i] as f64,
            strength[i] as f64,
            ramp[i] as f64,
            file_id,
            mode[i] as f64,
        ];
        if is_x {
            out_x.push(row);
        } else {
            out_s.push(row);
        }
    }
    let n_sband_out = out_s.len();
    let n_xband_out = out_x.len();
    let n_slipped = out_s
        .iter()
        .chain(out_x.iter())
        .filter(|r| r[9] != 0.0)
        .count();
    let mut stations: Vec<i64> = out_s
        .iter()
        .chain(out_x.iter())
        .map(|r| r[6] as i64)
        .collect();
    stations.sort_unstable();
    stations.dedup();
    dtype_hist.sort_by_key(|(d, _)| *d);
    unpaired_mode8_hist.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
    unpaired_mode8_hist.truncate(8);
    eprintln!(
        "{name}: SC {sc}, file year {file_year:.1}, Xponder {xpon:.3e} Hz, {n} tracking records ({skipped_zero} null, {skipped_unknown} unknown-format), fmt4 {n_fmt4} / fmt8 {n_fmt8}, bands S {n_sband} / X {n_xband}, dtype {dtype_hist:?}, {n_sband_out} S-band / {n_xband_out} X-band fsky samples (median S {fmed_s:.6e} / X {fmed_x:.6e} Hz), ref4 {ref4_min:.3e}..{ref4_max:.3e} / ref8 {ref8_min:.3e}..{ref8_max:.3e} Hz, ref8kHz {ref8_khz_hist:?}, mode8 {mode8_hist:?}, xmtr8kHz {xmtr8_khz_hist:?}, bias8 {bias8_hist:?}, {n_slipped} with slipped cycle, stations {stations:?} — separated: {ramp_records} ramp, {bias_rejected} bias, {ref_rejected} ref, {gap_rejected} gap, {wrap_rejected} wrap, {rate_rejected} rate, {med_rejected} median, {no_pair} unpaired {unpaired_mode8_hist:?}"
    );
    if out_s.is_empty() && out_x.is_empty() {
        None
    } else {
        Some(UlySkyFreq {
            sband: out_s,
            xband: out_x,
            n,
            no_pair,
            no_doppler,
            bias_rejected,
            ref_rejected,
            gap_rejected,
            wrap_rejected,
            rate_rejected,
            med_rejected,
        })
    }
}

pub fn write_resid_bin(records: &[[f64; 8]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 64);
    out.extend_from_slice(b"GASR");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_resid_bin(data: &[u8]) -> Option<Vec<[f64; 8]>> {
    if data.len() < 8 || &data[0..4] != b"GASR" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 64 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 64;
        let mut r = [0.0f64; 8];
        for k in 0..8 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub fn reduce_resid(
    name: &str,
    bytes: &[u8],
    lsk: &crate::lsk::LeapSeconds,
) -> Option<Vec<[f64; 8]>> {
    let stripped = strip_markers(bytes)?;
    let nlog = stripped.len() / LOGICAL_RECORD;
    if nlog < 3 {
        eprintln!("{name}: {nlog} logical records — too short");
        return None;
    }
    let mut out: Vec<[f64; 8]> = Vec::new();
    let mut n_dop = 0usize;
    let mut resid_min = f64::INFINITY;
    let mut resid_max = f64::NEG_INFINITY;
    let mut stations: Vec<i64> = Vec::new();
    let mut modes: Vec<i64> = Vec::new();
    for i in 2..nlog {
        let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
        let tr = tracking_record(rec);
        if tr.day == 0
            || !(tr.data_type == DTYPE_ONEWAY_DOPPLER || tr.data_type == DTYPE_TWOWAY_DOPPLER)
        {
            continue;
        }
        n_dop += 1;
        let sampler = tr.sampler_time as f64 / 100.0;
        if sampler <= 0.0 {
            continue;
        }
        let Some(tdb) = tdb_of(&tr, lsk) else {
            continue;
        };
        let resid = tr.doppler_resid as f64 / 1000.0;
        if !resid.is_finite() {
            continue;
        }
        if !stations.contains(&tr.station) {
            stations.push(tr.station);
        }
        if !modes.contains(&tr.ground_mode) {
            modes.push(tr.ground_mode);
        }
        resid_min = resid_min.min(resid);
        resid_max = resid_max.max(resid);
        out.push([
            tdb,
            resid,
            tr.station as f64,
            tr.ground_mode as f64,
            tr.data_type as f64,
            tr.doppler_ref as f64 / 10.0,
            sampler,
            tr.signal_strength as f64,
        ]);
    }
    if out.is_empty() {
        eprintln!("{name}: no doppler resid samples");
        return None;
    }
    stations.sort_unstable();
    modes.sort_unstable();
    eprintln!(
        "{name}: {n_dop} doppler records, {} resid samples, resid {resid_min:.3}..{resid_max:.3} Hz, stations {stations:?}, modes {modes:?}",
        out.len()
    );
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
        for b in &mut rec[mmin..=mmax] {
            *b = 0xff;
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

    #[test]
    fn gasr_roundtrip_carries_eight_slots() {
        let records = vec![
            [-8.0e8, -4.5, 43.0, 1.0, 1.0, 2.201e7, 1.0, 90.0],
            [-7.0e8, 12.5, 14.0, 3.0, 1.0, 2.201e7, 1.0, 80.0],
        ];
        let bytes = write_resid_bin(&records);
        let parsed = parse_resid_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
        assert!(parse_resid_bin(&bytes[..bytes.len() - 1]).is_none());
        assert!(parse_resid_bin(b"GASR").is_none());
        assert!(parse_resid_bin(b"PASF").is_none());
    }

    #[test]
    fn ulysses_series_roundtrip_and_component_name() {
        let row = [
            1.0, 2.293e9, 2.2e7, 1.0, 0.0, 1.0, 43.0, 1.0e6, 0.0, 0.0, 90.0, 0.0, 3.0, 1.0,
        ];
        let bytes = write_bin(&[row]);
        let parsed = parse_uly_series(&bytes).expect("ulysses series parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].2, crate::atdf::COMP_ULY_SKYFREQ);
        assert_eq!(
            uly_component_name(COMP_ULY_SKYFREQ),
            Some("ulysses_sky_frequency_hz")
        );
        assert_eq!(uly_component_name(99), None);
    }

    #[test]
    fn ulysses_series_skips_absent_frequency() {
        let row = [
            1.0, 0.0, 2.2e7, 1.0, 0.0, 1.0, 43.0, 1.0e6, 0.0, 0.0, 90.0, 0.0, 3.0, 1.0,
        ];
        let bytes = write_bin(&[row]);
        assert!(parse_uly_series(&bytes).is_none());
    }

    fn set_field(rec: &mut [u8], fld: &Field, value: i64) {
        for b in fld.start..=fld.stop {
            rec[b / 8] &= !(1u8 << (7 - (b % 8)));
        }
        let v = value as u64;
        let nbits = fld.stop - fld.start + 1;
        for k in 0..nbits {
            if (v >> k) & 1 == 1 {
                let bit = fld.stop - k;
                rec[bit / 8] |= 1u8 << (7 - (bit % 8));
            }
        }
    }

    fn set_tk(rec: &mut [u8], band: i64, second: i64, cnt_hp: i64) {
        set_field(rec, field_of(TKFORM, 1).unwrap(), 64);
        set_field(rec, field_of(TKFORM, 3).unwrap(), 90);
        set_field(rec, field_of(TKFORM, 4).unwrap(), 1);
        set_field(rec, field_of(TKFORM, 7).unwrap(), second);
        set_field(rec, field_of(TKFORM, 11).unwrap(), band);
        set_field(rec, field_of(TKFORM, 12).unwrap(), DTYPE_ONEWAY_DOPPLER);
        set_field(rec, field_of(TKFORM, 20).unwrap(), 0);
        set_field(rec, field_of(TKFORM, 30).unwrap(), 100);
        set_field(rec, field_of(TKFORM, 31).unwrap(), cnt_hp);
        set_field(rec, field_of(TKFORM, 40).unwrap(), 219_800_000);
    }

    fn ulysses_file(bands: &[i64]) -> Vec<u8> {
        let mut file = vec![0u8; PHYSICAL_RECORD];
        set_field(
            &mut file[0..LOGICAL_RECORD],
            field_of(IDFORM, 3).unwrap(),
            90,
        );
        set_field(
            &mut file[0..LOGICAL_RECORD],
            field_of(IDFORM, 4).unwrap(),
            1,
        );
        for (idx, &band) in bands.iter().enumerate() {
            let lo = (2 + idx) * LOGICAL_RECORD;
            let hi = (3 + idx) * LOGICAL_RECORD;
            set_tk(&mut file[lo..hi], band, idx as i64, idx as i64 * 100);
        }
        file
    }

    #[test]
    fn ulysses_x_series_roundtrip_and_component_name() {
        let row = [
            1.0, 8.4e9, 2.2e7, 1.0, 0.0, 1.0, 43.0, 1.0e6, 0.0, 0.0, 90.0, 0.0, 3.0, 1.0,
        ];
        let bytes = write_bin(&[row]);
        let parsed = parse_uly_series_x(&bytes).expect("ulysses X series parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].2, crate::atdf::COMP_ULY_SKYFREQ_X);
        assert_eq!(
            uly_component_name(COMP_ULY_SKYFREQ_X),
            Some("ulysses_sky_frequency_x_hz")
        );
    }

    #[test]
    fn reduce_uly_skyfreq_routes_x_band_to_x_output() {
        let lsk = crate::archivar::embedded_lsk().expect("embedded naif0012 parses");
        let file = ulysses_file(&[ULY_BAND_X, ULY_BAND_X]);
        let res = reduce_uly_skyfreq("x_band", 1.0, &file, &lsk).expect("reduce returns samples");
        assert!(
            res.sband.is_empty(),
            "X records must not land in the S vector"
        );
        assert_eq!(res.xband.len(), 1);
        let expected = X_BAND_RATIO * 21_980_000.0;
        assert!(
            (res.xband[0][1] - expected).abs() < 1e-6,
            "X fsky {} != {}",
            res.xband[0][1],
            expected
        );
    }

    #[test]
    fn reduce_uly_skyfreq_keeps_s_and_x_in_separate_vectors() {
        let lsk = crate::archivar::embedded_lsk().expect("embedded naif0012 parses");
        let file = ulysses_file(&[ULY_BAND_S, ULY_BAND_S, ULY_BAND_X, ULY_BAND_X]);
        let res = reduce_uly_skyfreq("mixed", 1.0, &file, &lsk).expect("reduce returns samples");
        let UlySkyFreq {
            sband: out_s,
            xband: out_x,
            n,
            no_pair,
            no_doppler,
            bias_rejected,
            ref_rejected,
            gap_rejected,
            wrap_rejected,
            rate_rejected,
            med_rejected,
        } = res;
        assert_eq!(out_s.len(), 1);
        assert_eq!(out_x.len(), 1);
        assert_eq!(no_pair, 2, "the tail record of each band stays unpaired");
        assert_eq!(
            out_s.len()
                + out_x.len()
                + no_pair
                + bias_rejected
                + ref_rejected
                + gap_rejected
                + wrap_rejected
                + rate_rejected
                + med_rejected
                + no_doppler,
            n
        );
        let s_expected = S_BAND_RATIO * 21_980_000.0;
        let x_expected = X_BAND_RATIO * 21_980_000.0;
        assert!((out_s[0][1] - s_expected).abs() < 1e-6);
        assert!((out_x[0][1] - x_expected).abs() < 1e-6);
    }

    #[test]
    fn reduce_uly_skyfreq_alternating_bands_pair_within_band() {
        let lsk = crate::archivar::embedded_lsk().expect("embedded naif0012 parses");
        let file = ulysses_file(&[
            ULY_BAND_S, ULY_BAND_X, ULY_BAND_S, ULY_BAND_X, ULY_BAND_S, ULY_BAND_X,
        ]);
        let res = reduce_uly_skyfreq("alternating", 1.0, &file, &lsk)
            .expect("reduce returns samples for both bands");
        let UlySkyFreq {
            sband: out_s,
            xband: out_x,
            n,
            no_pair,
            no_doppler,
            bias_rejected,
            ref_rejected,
            gap_rejected,
            wrap_rejected,
            rate_rejected,
            med_rejected,
        } = res;
        assert_eq!(out_s.len(), 2, "S records pair with the next S record");
        assert_eq!(out_x.len(), 2, "X records pair with the next X record");
        assert_eq!(no_pair, 2, "the tail record of each band stays unpaired");
        assert_eq!(
            out_s.len()
                + out_x.len()
                + no_pair
                + bias_rejected
                + ref_rejected
                + gap_rejected
                + wrap_rejected
                + rate_rejected
                + med_rejected
                + no_doppler,
            n
        );
        let s_expected = S_BAND_RATIO * 21_980_000.0;
        let x_expected = X_BAND_RATIO * 21_980_000.0;
        for r in &out_s {
            assert!(
                (r[1] - s_expected).abs() < 1e-6,
                "S fsky {} != {}",
                r[1],
                s_expected
            );
        }
        for r in &out_x {
            assert!(
                (r[1] - x_expected).abs() < 1e-6,
                "X fsky {} != {}",
                r[1],
                x_expected
            );
        }
    }

    #[test]
    fn reduce_uly_skyfreq_contiguous_band_anchors_sequential_pairing() {
        let lsk = crate::archivar::embedded_lsk().expect("embedded naif0012 parses");
        let file = ulysses_file(&[ULY_BAND_X, ULY_BAND_X, ULY_BAND_X]);
        let res =
            reduce_uly_skyfreq("contiguous_x", 1.0, &file, &lsk).expect("reduce returns samples");
        let UlySkyFreq {
            sband: out_s,
            xband: out_x,
            n,
            no_pair,
            no_doppler,
            bias_rejected,
            ref_rejected,
            gap_rejected,
            wrap_rejected,
            rate_rejected,
            med_rejected,
        } = res;
        assert!(out_s.is_empty(), "a contiguous X block carries no S record");
        assert_eq!(out_x.len(), 2, "each X record pairs with its successor");
        assert_eq!(no_pair, 1, "the tail record stays unpaired");
        assert_eq!(
            out_s.len()
                + out_x.len()
                + no_pair
                + bias_rejected
                + ref_rejected
                + gap_rejected
                + wrap_rejected
                + rate_rejected
                + med_rejected
                + no_doppler,
            n
        );
        let expected = X_BAND_RATIO * 21_980_000.0;
        for r in &out_x {
            assert!(
                (r[1] - expected).abs() < 1e-6,
                "X fsky {} != {}",
                r[1],
                expected
            );
        }
    }

    #[test]
    fn uly_band_field_is_downlink_band_not_station() {
        assert_eq!(field_of(TKFORM, 10).map(|f| f.name), Some("STATION"));
        assert_eq!(field_of(TKFORM, 11).map(|f| f.name), Some("DOWNLINK_BAND"));
        assert_eq!(ULY_BAND_S, 1);
        assert_eq!(ULY_BAND_X, 2);
    }

    #[test]
    fn record_format_discriminates_trk_and_sfoc() {
        let mut rec4 = [0u8; LOGICAL_RECORD];
        set_field(&mut rec4, field_of(TKFORM, 1).unwrap(), 64);
        assert_eq!(record_format(&rec4), Some(FMT_TRK_225));
        let mut rec8 = [0u8; LOGICAL_RECORD];
        set_field(&mut rec8, field_of(TKFORM8, 1).unwrap(), 8);
        assert_eq!(record_format(&rec8), Some(FMT_SFOC_NAV_225));
        let mut recx = [0u8; LOGICAL_RECORD];
        set_field(&mut recx, field_of(TKFORM8, 1).unwrap(), 3);
        assert_eq!(record_format(&recx), None);
        assert_eq!(record_format(&[0u8; LOGICAL_RECORD]), None);
    }

    #[test]
    fn tracking_record_leaves_f8_only_fields_absent() {
        let mut rec = [0u8; LOGICAL_RECORD];
        set_tk(&mut rec, ULY_BAND_S, 0, 100);
        let tr = tracking_record(&rec);
        assert_eq!(
            tr.ref_sky, None,
            "the format-4 record carries no frequency level"
        );
        assert_eq!(
            tr.xmtr_ref, None,
            "the format-4 record carries no transmitter reference"
        );
    }

    #[test]
    fn tracking_record_f8_maps_sfoc_fields() {
        let mut rec = [0u8; LOGICAL_RECORD];
        set_field(&mut rec, field_of(TKFORM8, 1).unwrap(), 8);
        set_field(&mut rec, field_of(TKFORM8, 4).unwrap(), 102);
        set_field(&mut rec, field_of(TKFORM8, 5).unwrap(), 100);
        set_field(&mut rec, field_of(TKFORM8, 6).unwrap(), 9);
        set_field(&mut rec, field_of(TKFORM8, 7).unwrap(), 30);
        set_field(&mut rec, field_of(TKFORM8, 8).unwrap(), 15);
        set_field(&mut rec, field_of(TKFORM8, 10).unwrap(), 43);
        set_field(
            &mut rec,
            field_of(TKFORM8, 12).unwrap(),
            DTYPE_TWOWAY_DOPPLER,
        );
        set_field(&mut rec, field_of(TKFORM8, 14).unwrap(), 2);
        set_field(&mut rec, field_of(TKFORM8, 15).unwrap(), 77);
        set_field(&mut rec, field_of(TKFORM8, 20).unwrap(), -1000);
        set_field(&mut rec, field_of(TKFORM8, 22).unwrap(), 0);
        set_field(&mut rec, field_of(TKFORM8, 29).unwrap(), 6000);
        set_field(&mut rec, field_of(TKFORM8, 30).unwrap(), 1000);
        set_field(&mut rec, field_of(TKFORM8, 31).unwrap(), 2000);
        set_field(&mut rec, field_of(TKFORM8, 32).unwrap(), 3000);
        set_field(&mut rec, field_of(TKFORM8, 43).unwrap(), 22000);
        set_field(&mut rec, field_of(TKFORM8, 44).unwrap(), 1234);
        set_field(&mut rec, field_of(TKFORM8, 74).unwrap(), -123);
        set_field(&mut rec, field_of(TKFORM8, 87).unwrap(), 5);
        set_field(&mut rec, field_of(TKFORM8, 89).unwrap(), -7);
        set_field(&mut rec, field_of(TKFORM8, 120).unwrap(), 7);
        set_field(&mut rec, field_of(TKFORM8, 121).unwrap(), 1234);
        let tr = tracking_record_f8(&rec);
        assert_eq!(tr.year, 102);
        assert_eq!(tr.day, 100);
        assert_eq!(tr.hour, 9);
        assert_eq!(tr.minute, 30);
        assert_eq!(tr.second, 15);
        assert_eq!(tr.station, 43);
        assert_eq!(tr.data_type, DTYPE_TWOWAY_DOPPLER);
        assert_eq!(tr.ground_mode, 2);
        assert_eq!(tr.spacecraft, 77);
        assert_eq!(tr.doppler_bias, -1);
        assert_eq!(tr.sampler_time, 6000);
        assert_eq!(tr.doppler_cnt_hp, 10_000_002);
        assert_eq!(tr.doppler_cnt_lp, 3);
        assert_eq!(tr.doppler_ref, 220_000_000);
        assert_eq!(tr.doppler_resid, -123);
        assert_eq!(tr.slipped_cycle, 5);
        assert_eq!(tr.signal_strength, -7);
        assert_eq!(tr.ramp_rate, 7_000_001_234);
        assert_eq!(
            tr.ref_sky,
            Some(false),
            "item 22 = 0 names the DCO reference level"
        );
        assert_eq!(
            tr.xmtr_ref,
            Some(0),
            "items 140/141 unset carry a zero xmtr reference"
        );
    }

    fn set_f8(rec: &mut [u8], day: i64, second: i64, cnt_ip: i64) {
        set_field(rec, field_of(TKFORM8, 1).unwrap(), 8);
        set_field(rec, field_of(TKFORM8, 4).unwrap(), 102);
        set_field(rec, field_of(TKFORM8, 5).unwrap(), day);
        set_field(rec, field_of(TKFORM8, 8).unwrap(), second);
        set_field(rec, field_of(TKFORM8, 10).unwrap(), 43);
        set_field(rec, field_of(TKFORM8, 11).unwrap(), ULY_BAND_S);
        set_field(rec, field_of(TKFORM8, 12).unwrap(), DTYPE_ONEWAY_DOPPLER);
        set_field(rec, field_of(TKFORM8, 14).unwrap(), 1);
        set_field(rec, field_of(TKFORM8, 22).unwrap(), 1);
        set_field(rec, field_of(TKFORM8, 29).unwrap(), 6000);
        set_field(rec, field_of(TKFORM8, 30).unwrap(), 1000);
        set_field(rec, field_of(TKFORM8, 31).unwrap(), cnt_ip * 100_000);
        set_field(rec, field_of(TKFORM8, 43).unwrap(), 2113312);
    }

    fn sfoc_file(days: &[i64]) -> Vec<u8> {
        let mut file = vec![0u8; PHYSICAL_RECORD];
        set_field(
            &mut file[0..LOGICAL_RECORD],
            field_of(IDFORM, 3).unwrap(),
            90,
        );
        set_field(
            &mut file[0..LOGICAL_RECORD],
            field_of(IDFORM, 4).unwrap(),
            1,
        );
        for (idx, &day) in days.iter().enumerate() {
            let lo = (2 + idx) * LOGICAL_RECORD;
            let hi = (3 + idx) * LOGICAL_RECORD;
            set_f8(&mut file[lo..hi], day, idx as i64 * 100, idx as i64 * 100);
        }
        file
    }

    #[test]
    fn reduce_uly_skyfreq_parses_sfoc_format_8_file() {
        let lsk = crate::archivar::embedded_lsk().expect("embedded naif0012 parses");
        let file = sfoc_file(&[1, 1]);
        let res = reduce_uly_skyfreq("sfoc_f8", 1.0, &file, &lsk).expect("reduce returns samples");
        assert_eq!(
            res.sband.len(),
            1,
            "the format-8 S-band pair yields one sample"
        );
        assert!(res.xband.is_empty(), "no X-band records in the fixture");
        let expected = S_BAND_TURNAROUND * 2_113_312_000.0 - (1_000_000.0 - RATE_OFFSET);
        assert!(
            (res.sband[0][1] - expected).abs() < 1e-3,
            "S fsky {} != {}",
            res.sband[0][1],
            expected
        );
        assert_eq!(res.sband[0][6] as i64, 43, "station carried from item 10");
    }
}
