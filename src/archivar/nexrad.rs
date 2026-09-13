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
            if data_start + data_len <= buf.len() {
                if let Some(radial) = parse_msg31(&buf[data_start..data_start + data_len]) {
                    radials.push(radial);
                }
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
}
