use crate::archivar::bzip2;

pub struct NexradMoment {
    pub name: [u8; 3],
    pub num_gates: u16,
    pub first_gate_km: f32,
    pub gate_width_km: f32,
    pub scale: f32,
    pub offset: f32,
    pub values: Vec<u16>,
}

pub struct NexradRadial {
    pub az_angle_deg: f32,
    pub el_angle_deg: f32,
    pub el_num: u8,
    pub moments: Vec<NexradMoment>,
}

pub struct NexradVolume {
    pub stid: Option<[u8; 4]>,
    pub version: Option<[u8; 9]>,
    pub vol_num: Option<[u8; 3]>,
    pub date: Option<u32>,
    pub time_ms: Option<u32>,
    pub radials: Vec<NexradRadial>,
}

fn be_u16(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_be_bytes(b.get(off..off + 2)?.try_into().ok()?))
}

fn be_u32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn be_i32(b: &[u8], off: usize) -> Option<i32> {
    Some(i32::from_be_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn be_f32(b: &[u8], off: usize) -> Option<f32> {
    Some(f32::from_bits(be_u32(b, off)?))
}

fn parse_msg31(data: &[u8]) -> Option<NexradRadial> {
    if data.len() < 32 {
        return None;
    }
    let az_angle = be_f32(data, 12)?;
    let el_num = data[22];
    let el_angle = be_f32(data, 24)?;
    let num_data_blks = be_u16(data, 30)? as usize;

    let mut moments = Vec::new();
    for i in 0..num_data_blks {
        let ptr = be_u32(data, 32 + i * 4)? as usize;
        if ptr == 0 || ptr + 4 > data.len() {
            continue;
        }
        if data[ptr] != b'D' {
            continue;
        }
        let name = [data[ptr + 1], data[ptr + 2], data[ptr + 3]];
        if ptr + 28 > data.len() {
            continue;
        }
        let num_gates = be_u16(data, ptr + 8)?;
        let first_gate = be_u16(data, ptr + 10)?;
        let gate_width = be_u16(data, ptr + 12)?;
        let data_size = data[ptr + 19];
        let scale = be_f32(data, ptr + 20)?;
        let offset = be_f32(data, ptr + 24)?;
        let elem = (data_size / 8) as usize;
        if elem != 1 && elem != 2 {
            continue;
        }
        if ptr + 28 + num_gates as usize * elem > data.len() {
            continue;
        }
        let mut values = Vec::with_capacity(num_gates as usize);
        for g in 0..num_gates as usize {
            let vo = ptr + 28 + g * elem;
            values.push(if elem == 1 {
                data[vo] as u16
            } else {
                be_u16(data, vo)?
            });
        }
        moments.push(NexradMoment {
            name,
            num_gates,
            first_gate_km: first_gate as f32 * 0.001,
            gate_width_km: gate_width as f32 * 0.001,
            scale,
            offset,
            values,
        });
    }

    Some(NexradRadial {
        az_angle_deg: az_angle,
        el_angle_deg: el_angle,
        el_num,
        moments,
    })
}

pub fn parse_nexrad(data: &[u8]) -> Option<NexradVolume> {
    let archive2 = data.starts_with(b"AR2V");
    let (stid, version, vol_num, date, time_ms) = if archive2 {
        if data.len() < 24 {
            return None;
        }
        (
            Some([data[20], data[21], data[22], data[23]]),
            Some([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
            ]),
            Some([data[9], data[10], data[11]]),
            Some(be_u32(data, 12)?),
            Some(be_u32(data, 16)?),
        )
    } else if data.starts_with(b"BZh") {
        (None, None, None, None, None)
    } else {
        return None;
    };

    let mut buf = Vec::new();
    if archive2 {
        let mut off = 24usize;
        while off + 4 <= data.len() {
            let size = be_i32(data, off)?.unsigned_abs() as usize;
            off += 4;
            if size == 0 || off + size > data.len() {
                break;
            }
            let decompressed = bzip2::decompress(&data[off..off + size])?;
            buf.extend_from_slice(&decompressed);
            off += size;
        }
    } else {
        buf.extend_from_slice(&bzip2::decompress(data)?);
    }

    let mut radials = Vec::new();
    let mut offset = 0usize;
    while offset + 28 <= buf.len() {
        let hdr = &buf[offset + 12..offset + 28];
        let size_hw = be_u16(hdr, 0)?;
        let msg_type = hdr[3];
        let num_segments = be_u16(hdr, 12)?;
        let segment_num = be_u16(hdr, 14)?;

        let msg_bytes = if size_hw == 0 {
            2432usize
        } else if size_hw == 65535 {
            (((num_segments as usize) << 16) | segment_num as usize) + 12
        } else if msg_type == 29 || msg_type == 31 {
            12 + 2 * size_hw as usize
        } else {
            2432usize
        };

        if msg_type == 31 {
            let data_start = offset + 28;
            let data_len = 2 * size_hw as usize - 16;
            if data_start + data_len <= buf.len()
                && let Some(radial) = parse_msg31(&buf[data_start..data_start + data_len])
            {
                radials.push(radial);
            }
        }

        offset += msg_bytes;
    }

    Some(NexradVolume {
        stid,
        version,
        vol_num,
        date,
        time_ms,
        radials,
    })
}

const EARTH_RADIUS_KM: f64 = 6371.0;
const REFRACTION_KE: f64 = 4.0 / 3.0;
const FEET_PER_M: f64 = 0.3048;

pub struct NexradSite {
    pub stid: [u8; 4],
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_m: f64,
}

const SITES: &[(&[u8; 4], f64, f64, f64)] = &[
    (b"KABR", 45.45583, -98.41306, 1302.0),
    (b"KABX", 35.14972, -106.82333, 5870.0),
    (b"KAKQ", 36.98389, -77.00750, 112.0),
    (b"KAMA", 35.23333, -101.70889, 3587.0),
    (b"KAMX", 25.61056, -80.41306, 14.0),
    (b"KAPX", 44.90722, -84.71972, 1464.0),
    (b"KARX", 43.82278, -91.19111, 1276.0),
    (b"KATX", 48.19472, -122.49444, 494.0),
    (b"KBBX", 39.49611, -121.63167, 173.0),
    (b"KBGM", 42.19972, -75.98500, 1606.0),
    (b"KBHX", 40.49833, -124.29194, 2402.0),
    (b"KBIS", 46.77083, -100.76028, 1658.0),
    (b"KBIX", 30.52389, -88.98472, 136.0),
    (b"KBLX", 45.85389, -108.60611, 3598.0),
    (b"KBMX", 33.17194, -86.76972, 645.0),
    (b"KBOX", 41.95583, -71.13750, 118.0),
    (b"KBRO", 25.91556, -97.41861, 23.0),
    (b"KBUF", 42.94861, -78.73694, 693.0),
    (b"KBYX", 24.59694, -81.70333, 8.0),
    (b"KCAE", 33.94861, -81.11861, 231.0),
    (b"KCBW", 46.03917, -67.80694, 746.0),
    (b"KCBX", 43.49083, -116.23444, 3061.0),
    (b"KCCX", 40.92306, -78.00389, 2405.0),
    (b"KCLE", 41.41306, -81.86000, 763.0),
    (b"KCLX", 32.65556, -81.04222, 97.0),
    (b"KCRP", 27.78389, -97.51083, 45.0),
    (b"KCXX", 44.51111, -73.16639, 317.0),
    (b"KCYS", 41.15194, -104.80611, 6128.0),
    (b"KDAX", 38.50111, -121.67667, 30.0),
    (b"KDDC", 37.76083, -99.96833, 2590.0),
    (b"KDFX", 29.27250, -100.28028, 1131.0),
    (b"KDIX", 39.94694, -74.41111, 149.0),
    (b"KDLH", 46.83694, -92.20972, 1428.0),
    (b"KDMX", 41.73111, -93.72278, 981.0),
    (b"KDOX", 38.82556, -75.44000, 50.0),
    (b"KDTX", 42.69972, -83.47167, 1072.0),
    (b"KDVN", 41.61167, -90.58083, 754.0),
    (b"KDYX", 32.53833, -99.25417, 1517.0),
    (b"KEAX", 38.81028, -94.26417, 995.0),
    (b"KEMX", 31.89361, -110.63028, 5202.0),
    (b"KENX", 42.58639, -74.06444, 1826.0),
    (b"KEOX", 31.46028, -85.45944, 434.0),
    (b"KEPZ", 31.87306, -106.69750, 4104.0),
    (b"KESX", 35.70111, -114.89139, 4867.0),
    (b"KEVX", 30.56417, -85.92139, 140.0),
    (b"KEWX", 29.70361, -98.02806, 140.0),
    (b"KEYX", 35.09778, -117.56000, 2757.0),
    (b"KFCX", 37.02417, -80.27417, 2868.0),
    (b"KFDR", 34.36222, -98.97611, 1267.0),
    (b"KFDX", 34.63528, -103.62944, 4650.0),
    (b"KFFC", 33.36333, -84.56583, 858.0),
    (b"KFSD", 43.58778, -96.72889, 1430.0),
    (b"KFSX", 34.57444, -111.19694, 1430.0),
    (b"KFTG", 39.78667, -104.54528, 5497.0),
    (b"KFWS", 32.57278, -97.30278, 683.0),
    (b"KGGW", 48.20639, -106.62417, 2276.0),
    (b"KGJX", 39.06222, -108.21306, 9992.0),
    (b"KGLD", 39.36694, -101.70000, 3651.0),
    (b"KGRB", 44.49833, -88.11111, 682.0),
    (b"KGRK", 30.72167, -97.38278, 538.0),
    (b"KGRR", 42.89389, -85.54472, 778.0),
    (b"KGSP", 34.88306, -82.22028, 940.0),
    (b"KGWX", 33.89667, -88.32889, 476.0),
    (b"KGYX", 43.89139, -70.25694, 409.0),
    (b"KHDX", 33.07639, -106.12222, 4222.0),
    (b"KHGX", 29.47194, -95.07889, 18.0),
    (b"KHNX", 36.31417, -119.63111, 243.0),
    (b"KHPX", 36.73667, -87.28500, 576.0),
    (b"KHTX", 34.93056, -86.08361, 1760.0),
    (b"KICT", 37.65444, -97.44250, 1335.0),
    (b"KICX", 37.59083, -112.86222, 10600.0),
    (b"KILN", 39.42028, -83.82167, 1056.0),
    (b"KILX", 40.15056, -89.33667, 582.0),
    (b"KIND", 39.70750, -86.28028, 790.0),
    (b"KINX", 36.17500, -95.56444, 668.0),
    (b"KIWA", 33.28917, -111.66917, 1353.0),
    (b"KIWX", 41.40861, -85.70000, 960.0),
    (b"KJAX", 30.48444, -81.70194, 33.0),
    (b"KJGX", 32.67500, -83.35111, 521.0),
    (b"KJKL", 37.59083, -83.31306, 1364.0),
    (b"KLBB", 33.65417, -101.81361, 3259.0),
    (b"KLCH", 30.12500, -93.21583, 13.0),
    (b"KLIX", 30.33667, -89.82528, 24.0),
    (b"KLNX", 41.95778, -100.57583, 2970.0),
    (b"KLOT", 41.60444, -88.08472, 663.0),
    (b"KLRX", 40.73972, -116.80278, 6744.0),
    (b"KLSX", 38.69889, -90.68278, 608.0),
    (b"KLTX", 33.98917, -78.42917, 64.0),
    (b"KLVX", 37.97528, -85.94389, 719.0),
    (b"KLWX", 38.97528, -77.47806, 272.0),
    (b"KLZK", 34.83639, -92.26194, 20.0),
    (b"KMAF", 31.94333, -102.18889, 2868.0),
    (b"KMAX", 42.08111, -122.71611, 7513.0),
    (b"KMBX", 48.39250, -100.86444, 1493.0),
    (b"KMHX", 34.77583, -76.87639, 31.0),
    (b"KMKX", 42.96778, -88.55056, 958.0),
    (b"KMLB", 28.11306, -80.65444, 35.0),
    (b"KMOB", 30.67944, -88.23972, 208.0),
    (b"KMPX", 44.84889, -93.56528, 946.0),
    (b"KMQT", 46.53111, -87.54833, 1411.0),
    (b"KMRX", 36.16833, -83.40194, 1337.0),
    (b"KMSX", 47.04111, -113.98611, 7855.0),
    (b"KMTX", 41.26278, -112.44694, 6460.0),
    (b"KMUX", 37.15528, -121.89750, 3469.0),
    (b"KMVX", 47.52806, -97.32500, 986.0),
    (b"KMXX", 32.53667, -85.78972, 400.0),
    (b"KNKX", 32.91889, -117.04194, 955.0),
    (b"KNQA", 35.34472, -89.87333, 282.0),
    (b"KOAX", 41.32028, -96.36639, 1148.0),
    (b"KOHX", 36.24722, -86.56250, 579.0),
    (b"KOKX", 40.86556, -72.86444, 85.0),
    (b"KOTX", 47.68056, -117.62583, 2384.0),
    (b"KPAH", 37.06833, -88.77194, 392.0),
    (b"KPBZ", 40.53167, -80.21833, 1185.0),
    (b"KPDT", 45.69056, -118.85278, 1515.0),
    (b"KPOE", 31.15528, -92.97583, 408.0),
    (b"KPUX", 38.45944, -104.18139, 5249.0),
    (b"KRAX", 35.66528, -78.49000, 348.0),
    (b"KRGX", 39.75417, -119.46111, 8299.0),
    (b"KRIW", 43.06611, -108.47667, 5568.0),
    (b"KRLX", 38.31194, -81.72389, 1080.0),
    (b"KRMX", 45.71500, -122.96417, 1572.0),
    (b"KSFX", 43.10583, -112.68528, 4474.0),
    (b"KSGF", 37.23528, -93.40028, 1278.0),
    (b"KSHV", 32.45056, -93.84111, 273.0),
    (b"KSJT", 31.37111, -100.49222, 1890.0),
    (b"KSOX", 33.81778, -117.63500, 3027.0),
    (b"KSRX", 35.29056, -94.36167, 1516.0),
    (b"KTBW", 27.70528, -82.40194, 41.0),
    (b"KTFX", 47.45972, -111.38444, 3714.0),
    (b"KTLH", 30.39750, -84.32889, 63.0),
    (b"KTLX", 35.33306, -97.27750, 1213.0),
    (b"KTWX", 38.99694, -96.23250, 1367.0),
    (b"KTYX", 43.75583, -75.68000, 1846.0),
    (b"KUDX", 44.12500, -102.82944, 3016.0),
    (b"KUEX", 40.32083, -98.44167, 1976.0),
    (b"KVAX", 30.89000, -83.00194, 178.0),
    (b"KVBX", 34.83806, -120.39583, 1233.0),
    (b"KVNX", 36.74083, -98.12750, 1210.0),
    (b"KVTX", 34.41167, -119.17861, 2726.0),
    (b"KVWX", 32.49528, -114.65583, 174.0),
    (b"PABC", 60.79278, -161.87417, 162.0),
    (b"PACG", 56.85278, -135.52917, 270.0),
    (b"PAEC", 64.51139, -165.29500, 54.0),
    (b"PAHG", 60.72639, -151.34917, 242.0),
    (b"PAIH", 59.46194, -146.30111, 67.0),
    (b"PAKC", 58.67944, -156.62944, 63.0),
    (b"PAPD", 65.03556, -147.49917, 2593.0),
    (b"PHKI", 21.89417, -159.55222, 179.0),
    (b"PHKM", 20.12556, -155.77778, 3812.0),
    (b"PHMO", 21.13278, -157.18000, 1363.0),
    (b"PHWA", 19.09500, -155.56889, 1370.0),
    (b"TJUA", 18.11750, -66.07861, 2794.0),
];

pub fn nexrad_site(stid: &[u8; 4]) -> Option<NexradSite> {
    let (code, lat, lon, elev_ft) = SITES.iter().find(|(code, ..)| *code == stid)?;
    Some(NexradSite {
        stid: **code,
        lat_deg: *lat,
        lon_deg: *lon,
        alt_m: *elev_ft * FEET_PER_M,
    })
}

fn great_circle_destination(
    lat0_deg: f64,
    lon0_deg: f64,
    bearing_deg: f64,
    dist_km: f64,
) -> Option<(f64, f64)> {
    if !lat0_deg.is_finite()
        || !lon0_deg.is_finite()
        || !bearing_deg.is_finite()
        || !dist_km.is_finite()
        || dist_km < 0.0
    {
        return None;
    }
    let lat1 = lat0_deg.to_radians();
    let lon1 = lon0_deg.to_radians();
    let bearing = bearing_deg.to_radians();
    let delta = dist_km / EARTH_RADIUS_KM;
    let lat2 = (lat1.sin() * delta.cos() + lat1.cos() * delta.sin() * bearing.cos()).asin();
    let lon2 = lon1
        + (bearing.sin() * delta.sin() * lat1.cos()).atan2(delta.cos() - lat1.sin() * lat2.sin());
    Some((lat2.to_degrees(), lon2.to_degrees()))
}

pub fn nexrad_gate_position(
    site: &NexradSite,
    az_deg: f64,
    el_deg: f64,
    slant_km: f64,
) -> Option<(f64, f64, f64)> {
    if !az_deg.is_finite() || !el_deg.is_finite() || !slant_km.is_finite() || slant_km <= 0.0 {
        return None;
    }
    let re = EARTH_RADIUS_KM * REFRACTION_KE;
    let el = el_deg.to_radians();
    let r = slant_km;
    let height_km = (r * r + re * re + 2.0 * r * re * el.sin()).sqrt() - re;
    if !height_km.is_finite() || height_km < 0.0 {
        return None;
    }
    let ground_km = re * (r * el.cos() / (re + height_km)).asin();
    if !ground_km.is_finite() || ground_km < 0.0 {
        return None;
    }
    let (lat, lon) = great_circle_destination(site.lat_deg, site.lon_deg, az_deg, ground_km)?;
    Some((lat, lon, site.alt_m + height_km * 1000.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_msg31(num_gates: u16, vals: &[u8]) -> Vec<u8> {
        let mut d = vec![0u8; 32];
        d[0..4].copy_from_slice(b"TEST");
        d[12..16].copy_from_slice(&90.0f32.to_bits().to_be_bytes());
        d[22] = 1;
        d[24..28].copy_from_slice(&0.5f32.to_bits().to_be_bytes());
        d[30..32].copy_from_slice(&1u16.to_be_bytes());

        d.extend_from_slice(&36u32.to_be_bytes());

        let mut blk = vec![0u8; 28];
        blk[0] = b'D';
        blk[1..4].copy_from_slice(b"REF");
        blk[8..10].copy_from_slice(&num_gates.to_be_bytes());
        blk[10..12].copy_from_slice(&2125u16.to_be_bytes());
        blk[12..14].copy_from_slice(&250u16.to_be_bytes());
        blk[19] = 8;
        blk[20..24].copy_from_slice(&2.0f32.to_bits().to_be_bytes());
        blk[24..28].copy_from_slice(&66.0f32.to_bits().to_be_bytes());
        d.extend_from_slice(&blk);
        d.extend_from_slice(vals);
        d
    }

    #[test]
    fn parses_msg31_ref() {
        let d = build_msg31(4, &[10, 20, 30, 0]);
        let radial = parse_msg31(&d).unwrap();
        assert_eq!(radial.az_angle_deg, 90.0);
        assert_eq!(radial.el_angle_deg, 0.5);
        assert_eq!(radial.el_num, 1);
        assert_eq!(radial.moments.len(), 1);
        let m = &radial.moments[0];
        assert_eq!(&m.name, b"REF");
        assert_eq!(m.num_gates, 4);
        assert_eq!(m.first_gate_km, 2.125);
        assert_eq!(m.gate_width_km, 0.25);
        assert_eq!(m.scale, 2.0);
        assert_eq!(m.offset, 66.0);
        assert_eq!(m.values, vec![10, 20, 30, 0]);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_nexrad(b"").is_none());
        assert!(parse_nexrad(b"PK\x03\x04").is_none());
        assert!(parse_nexrad(b"AR2V").is_none());
        assert!(parse_nexrad(b"BZh9").is_none());
    }

    fn from_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn parses_archive2_pipeline() {
        let data = from_hex(
            "41523256303030362e35343900004d0c000013444b544c58\
             00000067\
             425a683931415926535982130f640000037e48fc74040184101000d7021c0004000400\
             0010200054443400000018943d4006d4d064658ceb2a3830b0e589aa36210cc44d7ca9\
             bfaf69526219420fd762192a726306bb6ea5101f0f819f17724538509082130f64",
        );
        let vol = parse_nexrad(&data).unwrap();
        assert_eq!(vol.stid, Some(*b"KTLX"));
        assert_eq!(vol.date, Some(19724));
        assert_eq!(vol.time_ms, Some(4932));
        assert_eq!(vol.radials.len(), 1);
        let m = &vol.radials[0].moments[0];
        assert_eq!(&m.name, b"REF");
        assert_eq!(m.num_gates, 4);
        assert_eq!(m.first_gate_km, 2.125);
        assert_eq!(m.gate_width_km, 0.25);
        assert_eq!(m.values, vec![10, 20, 30, 0]);
    }

    #[test]
    fn site_anchor_resolves_ktlx_and_refuses_unknown() {
        let ktlx = nexrad_site(b"KTLX").expect("KTLX resolves");
        assert_eq!(ktlx.stid, *b"KTLX");
        assert!((ktlx.lat_deg - 35.33306).abs() < 1e-5);
        assert!((ktlx.lon_deg - -97.2775).abs() < 1e-5);
        assert!((ktlx.alt_m - 1213.0 * 0.3048).abs() < 1e-6);
        assert!(nexrad_site(b"XXXX").is_none());
        assert!(nexrad_site(b"LTXK").is_none());
    }

    #[test]
    fn gate_position_walks_north_at_zero_elevation() {
        let site = nexrad_site(b"KTLX").unwrap();
        let (lat, lon, alt) = nexrad_gate_position(&site, 0.0, 0.0, 111.195).unwrap();
        assert!((lat - site.lat_deg - 1.0).abs() < 0.05, "north lat {lat}");
        assert!((lon - site.lon_deg).abs() < 0.05);
        assert!(alt > site.alt_m, "beam rises above the site, alt {alt}");
        assert!(nexrad_gate_position(&site, 0.0, 0.0, -1.0).is_none());
        assert!(nexrad_gate_position(&site, f64::NAN, 0.0, 10.0).is_none());
    }
}
