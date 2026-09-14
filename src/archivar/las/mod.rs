const MAGIC: [u8; 4] = *b"LASF";

const HEADER_MIN: usize = 227;
const HEADER_14: usize = 375;
const VLR_HEADER: usize = 54;

fn le_u16(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}
fn le_u32(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}
fn le_u64(b: &[u8]) -> u64 {
    u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}
fn le_i32(b: &[u8]) -> i32 {
    i32::from_le_bytes([b[0], b[1], b[2], b[3]])
}
fn le_f64(b: &[u8]) -> f64 {
    f64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}
fn le_f32(b: &[u8]) -> f32 {
    f32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

#[derive(Clone, Copy, Debug)]
pub enum LasNote {
    Magic { bytes: [u8; 4] },
    EndAtByte { off: usize },
    HeaderSize { size: u16 },
    PointFormat { format: u8 },
    PointLength { format: u8, length: u16 },
    PointDataAt { off: usize },
    LazAbsent,
    LazItem { item: u16 },
    LazChunkTable { off: usize },
    LazChunkOverrun { off: usize },
    LazCoderStall { off: usize },
}

pub fn point_format_len(format: u8) -> Option<u16> {
    match format {
        0 => Some(20),
        1 => Some(28),
        2 => Some(26),
        3 => Some(34),
        4 => Some(57),
        5 => Some(63),
        6 => Some(30),
        7 => Some(36),
        8 => Some(38),
        9 => Some(59),
        10 => Some(67),
        _ => None,
    }
}

pub fn point_format_name(format: u8) -> String {
    match format {
        0 => "core".to_string(),
        1 => "core+gpstime".to_string(),
        2 => "core+rgb".to_string(),
        3 => "core+gpstime+rgb".to_string(),
        4 => "core+gpstime+waveform".to_string(),
        5 => "core+gpstime+rgb+waveform".to_string(),
        6 => "core+gpstime (1.4)".to_string(),
        7 => "core+gpstime+rgb (1.4)".to_string(),
        8 => "core+gpstime+rgb+nir (1.4)".to_string(),
        9 => "core+gpstime+waveform (1.4)".to_string(),
        10 => "core+gpstime+rgb+nir+waveform (1.4)".to_string(),
        n => format!("format {n}"),
    }
}

#[derive(Clone, Debug)]
pub struct LasHeader {
    pub version_major: u8,
    pub version_minor: u8,
    pub header_size: u16,
    pub offset_to_points: u32,
    pub num_vlrs: u32,
    pub point_format: u8,
    pub point_length: u16,
    pub point_count: u64,
    pub global_encoding: u16,
    pub scale: [f64; 3],
    pub offset: [f64; 3],
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub system_identifier: String,
    pub generating_software: String,
}

impl LasHeader {
    pub fn parse(bytes: &[u8]) -> Result<LasHeader, LasNote> {
        if bytes.len() < HEADER_MIN {
            return Err(LasNote::EndAtByte { off: bytes.len() });
        }
        let magic = [bytes[0], bytes[1], bytes[2], bytes[3]];
        if magic != MAGIC {
            return Err(LasNote::Magic { bytes: magic });
        }
        let header_size = le_u16(&bytes[94..96]);
        if (header_size as usize) < HEADER_MIN || header_size as usize > bytes.len() {
            return Err(LasNote::HeaderSize { size: header_size });
        }
        let version_major = bytes[24];
        let version_minor = bytes[25];
        let point_format = bytes[104] & 0x3f;
        let point_length = le_u16(&bytes[105..107]);
        match point_format_len(point_format) {
            Some(expected) if point_length >= expected => {}
            Some(_) => {
                return Err(LasNote::PointLength {
                    format: point_format,
                    length: point_length,
                });
            }
            None => {
                return Err(LasNote::PointFormat {
                    format: point_format,
                });
            }
        }
        let legacy_count = le_u32(&bytes[107..111]);
        let count64 = if version_minor >= 4 && bytes.len() >= HEADER_14 {
            le_u64(&bytes[247..255])
        } else {
            0
        };
        let point_count = if count64 != 0 {
            count64
        } else {
            legacy_count as u64
        };
        let system_identifier = trim_field(&bytes[26..58]);
        let generating_software = trim_field(&bytes[58..90]);
        Ok(LasHeader {
            version_major,
            version_minor,
            header_size,
            offset_to_points: le_u32(&bytes[96..100]),
            num_vlrs: le_u32(&bytes[100..104]),
            point_format,
            point_length,
            point_count,
            global_encoding: le_u16(&bytes[6..8]),
            scale: [
                le_f64(&bytes[131..139]),
                le_f64(&bytes[139..147]),
                le_f64(&bytes[147..155]),
            ],
            offset: [
                le_f64(&bytes[155..163]),
                le_f64(&bytes[163..171]),
                le_f64(&bytes[171..179]),
            ],
            min: [
                le_f64(&bytes[187..195]),
                le_f64(&bytes[203..211]),
                le_f64(&bytes[219..227]),
            ],
            max: [
                le_f64(&bytes[179..187]),
                le_f64(&bytes[195..203]),
                le_f64(&bytes[211..219]),
            ],
            system_identifier,
            generating_software,
        })
    }

    pub fn vlrs(&self, bytes: &[u8]) -> Result<Vec<LasVlr>, LasNote> {
        let mut out = Vec::new();
        let mut cursor = self.header_size as usize;
        for _ in 0..self.num_vlrs {
            if cursor + VLR_HEADER > bytes.len() {
                return Err(LasNote::EndAtByte { off: cursor });
            }
            let hdr = &bytes[cursor..cursor + VLR_HEADER];
            let user_id = trim_field(&hdr[2..18]);
            let record_id = le_u16(&hdr[18..20]);
            let record_len = le_u16(&hdr[20..22]) as usize;
            let description = trim_field(&hdr[22..54]);
            let payload_start = cursor + VLR_HEADER;
            if payload_start + record_len > bytes.len() {
                return Err(LasNote::EndAtByte { off: payload_start });
            }
            let payload = bytes[payload_start..payload_start + record_len].to_vec();
            out.push(LasVlr {
                user_id,
                record_id,
                description,
                payload,
            });
            cursor = payload_start + record_len;
        }
        Ok(out)
    }

    pub fn point_at(&self, bytes: &[u8], index: u64) -> Result<LasPoint, LasNote> {
        let start = self.offset_to_points as usize;
        let off = start + index as usize * self.point_length as usize;
        decode_point(self, bytes, off)
    }
}

fn trim_field(field: &[u8]) -> String {
    let end = field.iter().position(|&b| b == 0).unwrap_or(field.len());
    String::from_utf8_lossy(&field[..end]).trim().to_string()
}

#[derive(Clone, Debug)]
pub struct LasVlr {
    pub user_id: String,
    pub record_id: u16,
    pub description: String,
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
pub struct WavePacket {
    pub descriptor_index: u8,
    pub offset: u64,
    pub packet_size: u32,
    pub return_point: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct LasPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub intensity: u16,
    pub return_number: u8,
    pub number_of_returns: u8,
    pub classification: u8,
    pub gps_time: Option<f64>,
    pub red: Option<u16>,
    pub green: Option<u16>,
    pub blue: Option<u16>,
    pub nir: Option<u16>,
    pub waveform: Option<WavePacket>,
}

fn scaled(header: &LasHeader, raw: i32, axis: usize) -> f64 {
    raw as f64 * header.scale[axis] + header.offset[axis]
}

fn decode_point(header: &LasHeader, bytes: &[u8], off: usize) -> Result<LasPoint, LasNote> {
    let len = header.point_length as usize;
    if off + len > bytes.len() {
        return Err(LasNote::PointDataAt { off });
    }
    let rec = &bytes[off..off + len];
    let x = scaled(header, le_i32(&rec[0..4]), 0);
    let y = scaled(header, le_i32(&rec[4..8]), 1);
    let z = scaled(header, le_i32(&rec[8..12]), 2);
    let intensity = le_u16(&rec[12..14]);
    let format = header.point_format;
    let (return_number, number_of_returns, classification, gps_time, rgb, nir) = if format <= 5 {
        let rn = rec[14] & 0x07;
        let nr = (rec[14] >> 3) & 0x07;
        let cls = rec[15];
        let gps = if format == 1 || format == 3 || format == 4 || format == 5 {
            Some(le_f64(&rec[20..28]))
        } else {
            None
        };
        let (rgb_off, has_rgb) = if format == 2 || format == 3 {
            (20usize, true)
        } else if format == 5 {
            (28usize, true)
        } else {
            (0usize, false)
        };
        let rgb = if has_rgb {
            Some([
                le_u16(&rec[rgb_off..rgb_off + 2]),
                le_u16(&rec[rgb_off + 2..rgb_off + 4]),
                le_u16(&rec[rgb_off + 4..rgb_off + 6]),
            ])
        } else {
            None
        };
        (rn, nr, cls, gps, rgb, None)
    } else {
        let rn = rec[14] & 0x0f;
        let nr = (rec[14] >> 4) & 0x0f;
        let cls = rec[16];
        let gps = Some(le_f64(&rec[22..30]));
        let (rgb_off, has_rgb) = if format == 7 || format == 8 || format == 10 {
            (30usize, true)
        } else {
            (0usize, false)
        };
        let rgb = if has_rgb {
            Some([
                le_u16(&rec[rgb_off..rgb_off + 2]),
                le_u16(&rec[rgb_off + 2..rgb_off + 4]),
                le_u16(&rec[rgb_off + 4..rgb_off + 6]),
            ])
        } else {
            None
        };
        let nir = if format == 8 || format == 10 {
            Some(le_u16(&rec[36..38]))
        } else {
            None
        };
        (rn, nr, cls, gps, rgb, nir)
    };
    let wp_off = match format {
        4 => Some(28usize),
        5 => Some(34usize),
        9 => Some(30usize),
        10 => Some(38usize),
        _ => None,
    };
    let waveform = wp_off.map(|o| WavePacket {
        descriptor_index: rec[o],
        offset: le_u64(&rec[o + 1..o + 9]),
        packet_size: le_u32(&rec[o + 9..o + 13]),
        return_point: le_f32(&rec[o + 13..o + 17]),
        x: le_f32(&rec[o + 17..o + 21]),
        y: le_f32(&rec[o + 21..o + 25]),
        z: le_f32(&rec[o + 25..o + 29]),
    });
    Ok(LasPoint {
        x,
        y,
        z,
        intensity,
        return_number,
        number_of_returns,
        classification,
        gps_time,
        red: rgb.map(|r| r[0]),
        green: rgb.map(|r| r[1]),
        blue: rgb.map(|r| r[2]),
        nir,
        waveform,
    })
}

const COPC_INFO_ID: u16 = 1;

#[derive(Clone, Copy, Debug)]
pub struct CopcInfo {
    pub center: [f64; 3],
    pub halfsize: f64,
    pub spacing: f64,
    pub root_hier_offset: u64,
    pub root_hier_size: u64,
    pub gpstime_min: f64,
    pub gpstime_max: f64,
}

pub fn copc_info(vlrs: &[LasVlr]) -> Option<CopcInfo> {
    let vlr = vlrs
        .iter()
        .find(|v| v.user_id == "copc" && v.record_id == COPC_INFO_ID)?;
    let p = &vlr.payload;
    if p.len() < 160 {
        return None;
    }
    Some(CopcInfo {
        center: [le_f64(&p[0..8]), le_f64(&p[8..16]), le_f64(&p[16..24])],
        halfsize: le_f64(&p[24..32]),
        spacing: le_f64(&p[32..40]),
        root_hier_offset: le_u64(&p[40..48]),
        root_hier_size: le_u64(&p[48..56]),
        gpstime_min: le_f64(&p[56..64]),
        gpstime_max: le_f64(&p[64..72]),
    })
}

#[derive(Clone, Copy, Debug)]
pub struct CopcEntry {
    pub level: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub offset: u64,
    pub byte_size: i32,
    pub point_count: i32,
}

pub const COPC_ENTRY_SIZE: usize = 32;

fn parse_entries(bytes: &[u8], page_off: u64, page_size: u64) -> Option<Vec<CopcEntry>> {
    if page_size % COPC_ENTRY_SIZE as u64 != 0 {
        return None;
    }
    let start = page_off as usize;
    let count = (page_size as usize) / COPC_ENTRY_SIZE;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let off = start + i * COPC_ENTRY_SIZE;
        if off + COPC_ENTRY_SIZE > bytes.len() {
            return None;
        }
        let e = &bytes[off..off + COPC_ENTRY_SIZE];
        out.push(CopcEntry {
            level: le_i32(&e[0..4]),
            x: le_i32(&e[4..8]),
            y: le_i32(&e[8..12]),
            z: le_i32(&e[12..16]),
            offset: le_u64(&e[16..24]),
            byte_size: le_i32(&e[24..28]),
            point_count: le_i32(&e[28..32]),
        });
    }
    Some(out)
}

pub fn copc_hierarchy(bytes: &[u8], info: &CopcInfo) -> Option<Vec<CopcEntry>> {
    let mut out = Vec::new();
    let mut queue = vec![(info.root_hier_offset, info.root_hier_size)];
    let mut visited = 0usize;
    while let Some((off, size)) = queue.pop() {
        if visited > 1_000_000 {
            return None;
        }
        visited += 1;
        let entries = parse_entries(bytes, off, size)?;
        for e in entries {
            if e.point_count == -1 {
                if e.byte_size > 0 {
                    queue.push((e.offset, e.byte_size as u64));
                }
            } else {
                out.push(e);
            }
        }
    }
    Some(out)
}

#[derive(Clone, Debug)]
pub struct EptSchemaField {
    pub name: String,
    pub field_type: String,
    pub size: u64,
    pub scale: f64,
    pub offset: f64,
}

#[derive(Clone, Debug)]
pub struct EptLayout {
    pub span: u64,
    pub points: u64,
    pub data_type: String,
    pub hierarchy_type: String,
    pub bounds: [f64; 6],
    pub schema: Vec<EptSchemaField>,
}

pub fn ept_json(text: &str) -> Option<EptLayout> {
    let json = crate::json::parse_json(text)?;
    let span = crate::json::jnum(&json, "span")? as u64;
    let points = crate::json::jnum(&json, "points")? as u64;
    let data_type = crate::json::jstr(&json, "dataType")?;
    let hierarchy_type = crate::json::jstr(&json, "hierarchyType")?;
    let bounds = match crate::json::jpath_val(&json, "bounds") {
        Some(crate::json::JsonVal::Arr(a)) => {
            let mut b = [0.0f64; 6];
            for (i, v) in a.iter().take(6).enumerate() {
                b[i] = crate::json::scalar_of(v)?;
            }
            b
        }
        _ => return None,
    };
    let schema = match crate::json::jpath_val(&json, "schema") {
        Some(crate::json::JsonVal::Arr(a)) => {
            let mut s = Vec::new();
            for v in a {
                let name = crate::json::jstr(v, "name")?;
                let field_type = crate::json::jstr(v, "type")?;
                let size = crate::json::jnum(v, "size")? as u64;
                let scale = crate::json::jnum(v, "scale").unwrap_or(1.0);
                let offset = match crate::json::jnum(v, "offset") {
                    Some(o) => o,
                    None => 0.0,
                };
                s.push(EptSchemaField {
                    name,
                    field_type,
                    size,
                    scale,
                    offset,
                });
            }
            s
        }
        _ => return None,
    };
    Some(EptLayout {
        span,
        points,
        data_type,
        hierarchy_type,
        bounds,
        schema,
    })
}

pub mod laszip;
pub use laszip::{LazDecoder, has_laszip_vlr};

pub fn ept_key_decode(key: &str) -> Option<(i32, i32, i32, i32)> {
    let mut parts = key.split('-');
    let level = parts.next()?.parse().ok()?;
    let x = parts.next()?.parse().ok()?;
    let y = parts.next()?.parse().ok()?;
    let z = parts.next()?.parse().ok()?;
    Some((level, x, y, z))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put_u16(b: &mut [u8], off: usize, v: u16) {
        b[off..off + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn put_u32(b: &mut [u8], off: usize, v: u32) {
        b[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn put_u64(b: &mut [u8], off: usize, v: u64) {
        b[off..off + 8].copy_from_slice(&v.to_le_bytes());
    }
    fn put_f64(b: &mut [u8], off: usize, v: f64) {
        b[off..off + 8].copy_from_slice(&v.to_le_bytes());
    }

    fn synthetic_header(format: u8, point_count: u64) -> Vec<u8> {
        let mut b = vec![0u8; 375];
        b[0..4].copy_from_slice(b"LASF");
        b[24] = 1;
        b[25] = 4;
        put_u16(&mut b, 94, 375);
        put_u32(&mut b, 96, 375);
        put_u32(&mut b, 100, 0);
        b[104] = format;
        put_u16(&mut b, 105, point_format_len(format).unwrap());
        put_u32(&mut b, 107, point_count.min(u32::MAX as u64) as u32);
        put_u64(&mut b, 247, point_count);
        put_f64(&mut b, 131, 0.01);
        put_f64(&mut b, 139, 0.01);
        put_f64(&mut b, 147, 0.01);
        put_f64(&mut b, 155, 100.0);
        put_f64(&mut b, 163, 200.0);
        put_f64(&mut b, 171, 300.0);
        put_f64(&mut b, 179, 110.0);
        put_f64(&mut b, 187, 100.0);
        put_f64(&mut b, 195, 210.0);
        put_f64(&mut b, 203, 200.0);
        put_f64(&mut b, 211, 320.0);
        put_f64(&mut b, 219, 300.0);
        b
    }

    #[test]
    fn parses_synthetic_header() {
        let bytes = synthetic_header(3, 1000);
        let h = LasHeader::parse(&bytes).unwrap();
        assert_eq!(h.version_major, 1);
        assert_eq!(h.version_minor, 4);
        assert_eq!(h.point_format, 3);
        assert_eq!(h.point_count, 1000);
        assert_eq!(h.point_length, 34);
        assert_eq!(h.scale, [0.01, 0.01, 0.01]);
        assert_eq!(h.offset, [100.0, 200.0, 300.0]);
        assert_eq!(h.min, [100.0, 200.0, 300.0]);
        assert_eq!(h.max, [110.0, 210.0, 320.0]);
    }

    #[test]
    fn rejects_bad_magic() {
        let mut bytes = synthetic_header(3, 1);
        bytes[0] = b'X';
        assert!(matches!(
            LasHeader::parse(&bytes),
            Err(LasNote::Magic { .. })
        ));
    }

    #[test]
    fn rejects_short_header() {
        let bytes = vec![0u8; 100];
        assert!(matches!(
            LasHeader::parse(&bytes),
            Err(LasNote::EndAtByte { .. })
        ));
    }

    #[test]
    fn record_lengths_are_standard() {
        let expected = [20u16, 28, 26, 34, 57, 63, 30, 36, 38, 59, 67];
        for f in 0..=10u8 {
            assert_eq!(point_format_len(f), Some(expected[f as usize]));
        }
        assert_eq!(point_format_len(11), None);
    }

    #[test]
    fn decodes_format_zero_points() {
        let header = LasHeader::parse(&synthetic_header(0, 2)).unwrap();
        let mut rec = vec![0u8; 20];
        rec[0..4].copy_from_slice(&1000i32.to_le_bytes());
        rec[4..8].copy_from_slice(&2000i32.to_le_bytes());
        rec[8..12].copy_from_slice(&3000i32.to_le_bytes());
        rec[12..14].copy_from_slice(&42u16.to_le_bytes());
        rec[14] = 0x1b; // return 3 of 3 (0b011 << 3 | 0b011)
        rec[15] = 2;
        let mut bytes = synthetic_header(0, 2);
        bytes.extend_from_slice(&rec);
        bytes.extend_from_slice(&rec);
        let p = header.point_at(&bytes, 0).unwrap();
        assert_eq!(p.x, 110.0);
        assert_eq!(p.y, 220.0);
        assert_eq!(p.z, 330.0);
        assert_eq!(p.intensity, 42);
        assert_eq!(p.return_number, 3);
        assert_eq!(p.number_of_returns, 3);
        assert_eq!(p.classification, 2);
        assert!(p.gps_time.is_none());
    }

    #[test]
    fn decodes_format_one_gps_time() {
        let header = LasHeader::parse(&synthetic_header(1, 1)).unwrap();
        let mut rec = vec![0u8; 28];
        rec[20..28].copy_from_slice(&1234.5f64.to_le_bytes());
        let mut bytes = synthetic_header(1, 1);
        bytes.extend_from_slice(&rec);
        let p = header.point_at(&bytes, 0).unwrap();
        assert_eq!(p.gps_time, Some(1234.5));
    }

    #[test]
    fn decodes_format_six_classification_flags_layout() {
        let header = LasHeader::parse(&synthetic_header(6, 1)).unwrap();
        let mut rec = vec![0u8; 30];
        rec[0..4].copy_from_slice(&0i32.to_le_bytes());
        rec[4..8].copy_from_slice(&0i32.to_le_bytes());
        rec[8..12].copy_from_slice(&0i32.to_le_bytes());
        rec[14] = 0x31; // return 1 of 3 (0b0011 << 4 | 0b0001)
        rec[16] = 5;
        rec[22..30].copy_from_slice(&99.0f64.to_le_bytes());
        let mut bytes = synthetic_header(6, 1);
        bytes.extend_from_slice(&rec);
        let p = header.point_at(&bytes, 0).unwrap();
        assert_eq!(p.return_number, 1);
        assert_eq!(p.number_of_returns, 3);
        assert_eq!(p.classification, 5);
        assert_eq!(p.gps_time, Some(99.0));
    }

    #[test]
    fn parses_synthetic_copc_hierarchy_vlr() {
        let mut info = vec![0u8; 160];
        put_f64(&mut info, 0, 100.0);
        put_f64(&mut info, 8, 200.0);
        put_f64(&mut info, 16, 300.0);
        put_f64(&mut info, 24, 50.0);
        put_f64(&mut info, 32, 1.0);
        put_u64(&mut info, 40, 1000);
        put_u64(&mut info, 48, 64);
        let vlr = LasVlr {
            user_id: "copc".to_string(),
            record_id: COPC_INFO_ID,
            description: String::new(),
            payload: info,
        };
        let parsed = copc_info(&[vlr]).unwrap();
        assert_eq!(parsed.center, [100.0, 200.0, 300.0]);
        assert_eq!(parsed.halfsize, 50.0);
        assert_eq!(parsed.root_hier_size, 64);
    }

    #[test]
    fn parses_copc_hierarchy_entries() {
        let mut file = vec![0u8; 4096];
        let entry_off = 1000usize;
        let mut put_entry =
            |i: usize, level: i32, x: i32, y: i32, z: i32, off: u64, size: i32, count: i32| {
                let e = entry_off + i * COPC_ENTRY_SIZE;
                put_u32(&mut file, e, level as u32);
                put_u32(&mut file, e + 4, x as u32);
                put_u32(&mut file, e + 8, y as u32);
                put_u32(&mut file, e + 12, z as u32);
                put_u64(&mut file, e + 16, off);
                put_u32(&mut file, e + 24, size as u32);
                put_u32(&mut file, e + 28, count as u32);
            };
        put_entry(0, 0, 0, 0, 0, 1200, 100, 50);
        put_entry(1, 1, 1, 0, 0, 1300, 80, 30);
        let info = CopcInfo {
            center: [0.0, 0.0, 0.0],
            halfsize: 1.0,
            spacing: 1.0,
            root_hier_offset: 1000,
            root_hier_size: 64,
            gpstime_min: 0.0,
            gpstime_max: 0.0,
        };
        let entries = copc_hierarchy(&file, &info).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].point_count, 50);
        assert_eq!(entries[1].x, 1);
        assert_eq!(entries[1].offset, 1300);
    }

    #[test]
    fn parses_ept_json_layout() {
        let text = r#"{
            "bounds": [-105.0, 40.0, 1000.0, -104.0, 41.0, 2000.0],
            "dataType": "laszip",
            "hierarchyType": "json",
            "points": 123456,
            "span": 256,
            "schema": [
                {"name": "X", "type": "signed", "size": 4, "scale": 0.01, "offset": 0.0},
                {"name": "Intensity", "type": "unsigned", "size": 2, "scale": 1.0, "offset": 0.0}
            ]
        }"#;
        let layout = ept_json(text).unwrap();
        assert_eq!(layout.span, 256);
        assert_eq!(layout.points, 123456);
        assert_eq!(layout.data_type, "laszip");
        assert_eq!(layout.hierarchy_type, "json");
        assert_eq!(layout.bounds, [-105.0, 40.0, 1000.0, -104.0, 41.0, 2000.0]);
        assert_eq!(layout.schema.len(), 2);
        assert_eq!(layout.schema[0].name, "X");
        assert_eq!(layout.schema[0].scale, 0.01);
    }

    #[test]
    fn decodes_ept_key() {
        assert_eq!(ept_key_decode("3-4-5-6"), Some((3, 4, 5, 6)));
        assert_eq!(ept_key_decode("3-4-5"), None);
    }

    fn push_opt_f64(out: &mut Vec<u8>, v: Option<f64>) {
        match v {
            Some(x) => {
                out.push(1);
                out.extend_from_slice(&x.to_bits().to_le_bytes());
            }
            None => out.push(0),
        }
    }

    fn push_opt_u16(out: &mut Vec<u8>, v: Option<u16>) {
        match v {
            Some(x) => {
                out.push(1);
                out.extend_from_slice(&x.to_le_bytes());
            }
            None => out.push(0),
        }
    }

    fn point_digest_bytes(p: &LasPoint, out: &mut Vec<u8>) {
        out.extend_from_slice(&p.x.to_bits().to_le_bytes());
        out.extend_from_slice(&p.y.to_bits().to_le_bytes());
        out.extend_from_slice(&p.z.to_bits().to_le_bytes());
        out.extend_from_slice(&p.intensity.to_le_bytes());
        out.push(p.return_number);
        out.push(p.number_of_returns);
        out.push(p.classification);
        push_opt_f64(out, p.gps_time);
        push_opt_u16(out, p.red);
        push_opt_u16(out, p.green);
        push_opt_u16(out, p.blue);
        push_opt_u16(out, p.nir);
        match &p.waveform {
            Some(w) => {
                out.push(1);
                out.push(w.descriptor_index);
                out.extend_from_slice(&w.offset.to_le_bytes());
                out.extend_from_slice(&w.packet_size.to_le_bytes());
                out.extend_from_slice(&w.return_point.to_bits().to_le_bytes());
                out.extend_from_slice(&w.x.to_bits().to_le_bytes());
                out.extend_from_slice(&w.y.to_bits().to_le_bytes());
                out.extend_from_slice(&w.z.to_bits().to_le_bytes());
            }
            None => out.push(0),
        }
    }

    fn decoded_digest(bytes: &[u8]) -> String {
        let h = LasHeader::parse(bytes).unwrap();
        let mut dec = LazDecoder::new(&h, bytes).unwrap();
        let mut buf = Vec::with_capacity(h.point_count as usize * 64);
        for i in 0..h.point_count {
            point_digest_bytes(&dec.point_at(i).unwrap(), &mut buf);
        }
        crate::archivar::sha256::sha256_hex(&buf)
    }

    fn uncompressed_digest(bytes: &[u8]) -> String {
        let h = LasHeader::parse(bytes).unwrap();
        let mut buf = Vec::with_capacity(h.point_count as usize * 64);
        for i in 0..h.point_count {
            point_digest_bytes(&h.point_at(bytes, i).unwrap(), &mut buf);
        }
        crate::archivar::sha256::sha256_hex(&buf)
    }

    #[test]
    fn simple_format3_rgb_fixture_digest() {
        let bytes = include_bytes!("fixtures/simple.laz");
        let h = LasHeader::parse(bytes).unwrap();
        assert_eq!(h.point_format, 3);
        assert_eq!(h.point_count, 1065);
        let mut dec = LazDecoder::new(&h, bytes).unwrap();
        let first = dec.point_at(0).unwrap();
        assert!(first.x >= h.min[0] && first.x <= h.max[0]);
        assert!(first.y >= h.min[1] && first.y <= h.max[1]);
        assert!(first.z >= h.min[2] && first.z <= h.max[2]);
        assert!(first.red.unwrap() > 0);
        assert!(first.green.unwrap() > 0);
        assert!(first.blue.unwrap() > 0);
        assert!(first.nir.is_none());
        assert!(first.waveform.is_none());
        assert_eq!(
            decoded_digest(bytes),
            "1d2a6da4511abcb1e122a18b062346dfb8f1fa85d56118703a49afe34560e1c7"
        );
    }

    #[test]
    fn fullwave_format10_rgb_nir_waveform_fixture_digest() {
        let laz = include_bytes!("fixtures/fullwave.laz");
        let las = include_bytes!("fixtures/fullwave.las");
        let h = LasHeader::parse(laz).unwrap();
        assert_eq!(h.point_format, 10);
        assert_eq!(h.point_count, 10750);
        let mut dec = LazDecoder::new(&h, laz).unwrap();
        let first = dec.point_at(0).unwrap();
        assert!(first.red.is_some());
        assert!(first.green.is_some());
        assert!(first.blue.is_some());
        assert!(first.nir.is_some());
        assert!(first.waveform.is_some());
        assert_eq!(decoded_digest(laz), uncompressed_digest(las));
    }

    #[test]
    fn autzen_format3_gpstime_unchanged_selector_digest() {
        let bytes = include_bytes!("fixtures/autzen_trim.laz");
        let h = LasHeader::parse(bytes).unwrap();
        assert_eq!(h.point_format, 3);
        assert_eq!(h.point_count, 110000);
        let mut dec = LazDecoder::new(&h, bytes).unwrap();
        let first = dec.point_at(0).unwrap();
        assert!(first.x >= h.min[0] && first.x <= h.max[0]);
        assert!(first.red.unwrap() > 0);
        assert!(first.green.unwrap() > 0);
        assert!(first.blue.unwrap() > 0);
        assert!(first.gps_time.is_some());
        assert_eq!(
            decoded_digest(bytes),
            "ecad0830b38100e0627865ab01b837e90019a2b52131b7c059011dfce46853c9"
        );
    }

    #[test]
    fn simple_laz_matches_uncompressed_las_reference() {
        let laz = include_bytes!("fixtures/simple.laz");
        let las = include_bytes!("fixtures/simple.las");
        assert_eq!(decoded_digest(laz), uncompressed_digest(las));
    }

    #[test]
    fn autzen_trim_laz_matches_uncompressed_las_reference() {
        let laz = include_bytes!("fixtures/autzen_trim.laz");
        let las = include_bytes!("fixtures/autzen_trim.las");
        assert_eq!(decoded_digest(laz), uncompressed_digest(las));
    }

    #[test]
    fn pdrf4_pointwise_byte_wavepacket13_fixture_digest() {
        let laz = include_bytes!("fixtures/pdrf4-1.3.laz");
        let las = include_bytes!("fixtures/pdrf4-1.3.las");
        let h = LasHeader::parse(laz).unwrap();
        assert_eq!(h.point_format, 4);
        assert_eq!(h.point_count, 1024);
        let mut dec = LazDecoder::new(&h, laz).unwrap();
        let first = dec.point_at(0).unwrap();
        assert!(first.waveform.is_some());
        assert!(first.gps_time.is_some());
        assert_eq!(decoded_digest(laz), uncompressed_digest(las));
    }

    #[test]
    fn pdrf4_uncompressed_format4_waveform_fixture_digest() {
        let bytes = include_bytes!("fixtures/pdrf4-1.3.las");
        let h = LasHeader::parse(bytes).unwrap();
        assert_eq!(h.point_format, 4);
        assert_eq!(h.point_count, 1024);
        let first = h.point_at(bytes, 0).unwrap();
        assert!(first.waveform.is_some());
        assert!(first.gps_time.is_some());
        assert_eq!(
            uncompressed_digest(bytes),
            "f0fa10aa6b31ec91babdb1c615cd0b371d7b9183c19e4fed46c4ed274bf5281b"
        );
    }

    #[test]
    fn rgb14_format7_standalone_fixture_digest() {
        let bytes = include_bytes!("fixtures/1.2-with-color.copc.laz");
        let h = LasHeader::parse(bytes).unwrap();
        assert_eq!(h.point_format, 7);
        assert_eq!(h.point_count, 1065);
        assert_eq!(
            decoded_digest(bytes),
            "ecde10ad6cac9fddb9fd306ba9bc50e9cbc4e7afae867bfed3b34389ae1a8778"
        );
    }

    #[test]
    fn byte14_extra_bytes_fixture_digest() {
        let bytes = include_bytes!("fixtures/2019_saipan_waveform.laz");
        let h = LasHeader::parse(bytes).unwrap();
        assert_eq!(h.point_format, 6);
        assert_eq!(h.point_count, 5198);
        assert_eq!(
            decoded_digest(bytes),
            "85f7001a81cef22bd43ba4ca63d76243554d02cb6d2780100c06861f43523f23"
        );
    }

    #[test]
    fn point10_format0_pointwise_fixture_digest() {
        let bytes = include_bytes!("fixtures/point10.las.laz");
        let h = LasHeader::parse(bytes).unwrap();
        assert_eq!(h.point_format, 0);
        assert_eq!(h.point_count, 1065);
        assert_eq!(
            decoded_digest(bytes),
            "a236aeedce414ef4bf70d12ffc448ea8fb1cc559affd0ca5f794bb015c4e26dc"
        );
    }

    #[test]
    fn point10_rgb12_format2_pointwise_fixture_digest() {
        let bytes = include_bytes!("fixtures/point-color.las.laz");
        let h = LasHeader::parse(bytes).unwrap();
        assert_eq!(h.point_format, 2);
        assert_eq!(h.point_count, 1065);
        assert_eq!(
            decoded_digest(bytes),
            "c4265a4210696d66a65d97b05d237e80bf7bb06788dbd8677010699be5385ab1"
        );
    }

    #[test]
    fn format6_layered_matches_uncompressed_las_reference() {
        let laz = include_bytes!("fixtures/1_4_w_evlr.laz");
        let las = include_bytes!("fixtures/1_4_w_evlr.las");
        let h = LasHeader::parse(laz).unwrap();
        assert_eq!(h.point_format, 6);
        let mut dec = LazDecoder::new(&h, laz).unwrap();
        let first = dec.point_at(0).unwrap();
        assert!(first.gps_time.is_some());
        assert_eq!(decoded_digest(laz), uncompressed_digest(las));
    }

    #[test]
    fn format7_layered_rgb_matches_uncompressed_las_reference() {
        let laz = include_bytes!("fixtures/autzen_trim_7.laz");
        let las = include_bytes!("fixtures/autzen_trim_7.las");
        let h = LasHeader::parse(laz).unwrap();
        assert_eq!(h.point_format, 7);
        let mut dec = LazDecoder::new(&h, laz).unwrap();
        let first = dec.point_at(0).unwrap();
        assert!(first.red.is_some());
        assert!(first.green.is_some());
        assert!(first.blue.is_some());
        assert_eq!(decoded_digest(laz), uncompressed_digest(las));
    }
}
