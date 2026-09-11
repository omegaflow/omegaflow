use crate::lsk::days_from_civil;

pub const WIN_NS_MAX: usize = 10000;
pub const BLOCK_HEADER: usize = 16;
pub const MARKER_LEN: usize = 4;

pub struct WinSample {
    pub t: f64,
    pub chan: u16,
    pub val: i32,
}

pub struct WinChannel {
    pub chan: u16,
    pub org1: u8,
    pub org2: u8,
    pub nsamp: usize,
    pub values: Vec<i32>,
}

fn bcd(v: u8) -> Option<u32> {
    let hi = v >> 4;
    let lo = v & 0x0F;
    if hi > 9 || lo > 9 {
        None
    } else {
        Some(hi as u32 * 10 + lo as u32)
    }
}

fn bcd_datetime(b: &[u8]) -> Option<(i64, u32, u32, u32, u32, u32)> {
    if b.len() < 7 {
        return None;
    }
    let y1 = bcd(b[0])?;
    let y2 = bcd(b[1])?;
    let year = y1 as i64 * 100 + y2 as i64;
    let month = bcd(b[2])?;
    let day = bcd(b[3])?;
    let hour = bcd(b[4])?;
    let minute = bcd(b[5])?;
    let second = bcd(b[6])?;
    if !(1913..=2100).contains(&year) {
        return None;
    }
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    Some((year, month, day, hour, minute, second))
}

fn unix_of(dt: &(i64, u32, u32, u32, u32, u32)) -> Option<f64> {
    let days = days_from_civil(dt.0, dt.1 as i64, dt.2 as i64)?;
    Some(days as f64 * 86400.0 + dt.3 as f64 * 3600.0 + dt.4 as f64 * 60.0 + dt.5 as f64)
}

fn nibble_signed(v: u8) -> i32 {
    let n = (v & 0x0F) as i32;
    if n & 0x08 != 0 {
        n - 16
    } else {
        n
    }
}

fn int24(b0: u8, b1: u8, b2: u8) -> i32 {
    let v = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
    if v & 0x800000 != 0 {
        v as i32 - 0x1000000
    } else {
        v as i32
    }
}

fn decode_channel(
    chan: u16,
    org1: u8,
    org2: u8,
    ss: u8,
    ns0: usize,
    data: &[u8],
) -> Option<WinChannel> {
    if ns0 < 1 || ns0 > WIN_NS_MAX || ss > 5 || data.len() < 4 {
        return None;
    }
    let first = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let mut values = Vec::with_capacity(ns0);
    values.push(first);
    let mut prev = first;
    let rest = &data[4..];
    match ss {
        0 => {
            let need = ns0 / 2;
            if rest.len() < need {
                return None;
            }
            let mut i = 1usize;
            while i < ns0 {
                let b = rest[(i - 1) / 2];
                prev += nibble_signed(b >> 4);
                values.push(prev);
                i += 1;
                if i < ns0 {
                    prev += nibble_signed(b & 0x0F);
                    values.push(prev);
                    i += 1;
                }
            }
        }
        1 => {
            let need = ns0 - 1;
            if rest.len() < need {
                return None;
            }
            for k in 0..need {
                prev += rest[k] as i8 as i32;
                values.push(prev);
            }
        }
        2 => {
            let need = (ns0 - 1) * 2;
            if rest.len() < need {
                return None;
            }
            for k in 0..ns0 - 1 {
                prev += i16::from_be_bytes([rest[k * 2], rest[k * 2 + 1]]) as i32;
                values.push(prev);
            }
        }
        3 => {
            let need = (ns0 - 1) * 3;
            if rest.len() < need {
                return None;
            }
            for k in 0..ns0 - 1 {
                prev += int24(rest[k * 3], rest[k * 3 + 1], rest[k * 3 + 2]);
                values.push(prev);
            }
        }
        4 => {
            let need = (ns0 - 1) * 4;
            if rest.len() < need {
                return None;
            }
            for k in 0..ns0 - 1 {
                prev += i32::from_be_bytes([
                    rest[k * 4],
                    rest[k * 4 + 1],
                    rest[k * 4 + 2],
                    rest[k * 4 + 3],
                ]);
                values.push(prev);
            }
        }
        5 => {
            let need = (ns0 - 1) * 4;
            if rest.len() < need {
                return None;
            }
            for k in 0..ns0 - 1 {
                values.push(i32::from_be_bytes([
                    rest[k * 4],
                    rest[k * 4 + 1],
                    rest[k * 4 + 2],
                    rest[k * 4 + 3],
                ]));
            }
        }
        _ => return None,
    }
    Some(WinChannel {
        chan,
        org1,
        org2,
        nsamp: ns0,
        values,
    })
}

pub fn parse_win32(bytes: &[u8]) -> Option<Vec<WinSample>> {
    if bytes.len() < MARKER_LEN || bytes[0..MARKER_LEN] != [0, 0, 0, 0] {
        return None;
    }
    let mut off = MARKER_LEN;
    let mut out = Vec::new();
    while off + BLOCK_HEADER <= bytes.len() {
        let Some(dt) = bcd_datetime(&bytes[off..off + 7]) else {
            break;
        };
        let sz = u32::from_be_bytes([
            bytes[off + 12],
            bytes[off + 13],
            bytes[off + 14],
            bytes[off + 15],
        ]) as usize;
        let data_start = off + BLOCK_HEADER;
        let data_end = data_start.saturating_add(sz);
        if data_end > bytes.len() || data_end <= data_start {
            break;
        }
        let Some(t0) = unix_of(&dt) else {
            break;
        };
        let mut p = data_start;
        while p + 10 <= data_end {
            let org1 = bytes[p];
            let org2 = bytes[p + 1];
            let chan = u16::from_be_bytes([bytes[p + 2], bytes[p + 3]]);
            let ss = (bytes[p + 4] >> 4) & 0x0F;
            let ns0 = (((bytes[p + 4] & 0x0F) as usize) << 8) | bytes[p + 5] as usize;
            let data_size = if ns0 == 1 {
                0
            } else if ss == 0 {
                ns0 / 2
            } else {
                (ns0 - 1) * if ss == 5 { 4 } else { ss as usize }
            };
            if p + 10 + data_size > data_end {
                break;
            }
            let data = &bytes[p + 6..p + 10 + data_size];
            if let Some(ch) = decode_channel(chan, org1, org2, ss, ns0, data) {
                let rate = ns0 as f64;
                let mut samples = Vec::with_capacity(ch.values.len());
                for (i, v) in ch.values.iter().enumerate() {
                    samples.push(WinSample {
                        t: t0 + i as f64 / rate,
                        chan,
                        val: *v,
                    });
                }
                for s in samples {
                    out.push(s);
                }
            }
            p += 10 + data_size;
        }
        off = data_end;
    }
    Some(out)
}

pub fn chan_of(station: u32) -> u16 {
    (station & 0xFFFF) as u16
}

pub fn station_of(chan: u16) -> u32 {
    chan as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn win32_fixture() -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0, 0, 0, 0]);
        let mut header = [0u8; BLOCK_HEADER];
        header[0] = 0x20;
        header[1] = 0x20;
        header[2] = 0x01;
        header[3] = 0x01;
        header[4] = 0x00;
        header[5] = 0x00;
        header[6] = 0x00;
        let packet = [
            0x00u8, 0x01, 0x01, 0x01, 0x10, 0x05, 0x00, 0x00, 0x03, 0xE8, 0x01, 0xFE, 0x03, 0xFC,
        ];
        let sz = packet.len() as u32;
        header[12] = (sz >> 24) as u8;
        header[13] = (sz >> 16) as u8;
        header[14] = (sz >> 8) as u8;
        header[15] = sz as u8;
        buf.extend_from_slice(&header);
        buf.extend_from_slice(&packet);
        buf
    }

    #[test]
    fn decodes_8bit_difference_packet() {
        let bytes = win32_fixture();
        let samples = parse_win32(&bytes).expect("fixture parses");
        assert_eq!(samples.len(), 5);
        assert_eq!(samples[0].chan, 0x0101);
        assert_eq!(samples[0].val, 1000);
        assert_eq!(samples[1].val, 1001);
        assert_eq!(samples[2].val, 999);
        assert_eq!(samples[3].val, 1002);
        assert_eq!(samples[4].val, 998);
        assert_eq!(samples[0].t, 1577836800.0);
        assert_eq!(samples[1].t, 1577836800.2);
    }

    fn packet_header(chan: u16, ss: u8, ns0: u16) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&[0x00, 0x00]);
        p.extend_from_slice(&chan.to_be_bytes());
        p.push((ss & 0x0F) << 4 | (((ns0 >> 8) & 0x0F) as u8));
        p.push((ns0 & 0xFF) as u8);
        p
    }

    fn wrap_packet(packet: &[u8]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0, 0, 0, 0]);
        let mut header = [0u8; BLOCK_HEADER];
        header[0] = 0x20;
        header[1] = 0x24;
        header[2] = 0x05;
        header[3] = 0x04;
        header[4] = 0x00;
        header[5] = 0x00;
        header[6] = 0x01;
        let sz = packet.len() as u32;
        header[12] = (sz >> 24) as u8;
        header[13] = (sz >> 16) as u8;
        header[14] = (sz >> 8) as u8;
        header[15] = sz as u8;
        buf.extend_from_slice(&header);
        buf.extend_from_slice(packet);
        buf
    }

    #[test]
    fn decodes_absolute_int32_packet() {
        let mut packet = packet_header(0x0202, 5, 6);
        packet.extend_from_slice(&0x11i32.to_be_bytes());
        for v in [0x22i32, 0x33, 0x44, 0x55, 0x66] {
            packet.extend_from_slice(&v.to_be_bytes());
        }
        let samples = parse_win32(&wrap_packet(&packet)).expect("absolute packet parses");
        assert_eq!(samples.len(), 6);
        let want = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
        for (i, s) in samples.iter().enumerate() {
            assert_eq!(s.val, want[i]);
        }
    }

    #[test]
    fn decodes_nibble_difference_packet() {
        let mut packet = packet_header(0x0303, 0, 6);
        packet.extend_from_slice(&1000i32.to_be_bytes());
        packet.extend_from_slice(&[0x12, 0x34, 0x56]);
        let samples = parse_win32(&wrap_packet(&packet)).expect("nibble packet parses");
        assert_eq!(samples.len(), 6);
        let mut expect = vec![1000i32];
        let mut prev = 1000i32;
        for pair in [[1i32, 2], [3, 4], [5, 6]] {
            prev += pair[0];
            expect.push(prev);
            prev += pair[1];
            expect.push(prev);
        }
        for (i, s) in samples.iter().enumerate() {
            assert_eq!(s.val, expect[i], "sample {i}");
        }
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_win32(b"XXXX").is_none());
        assert!(parse_win32(&[1, 2, 3, 4, 0, 0, 0, 0]).is_none());
    }

    #[test]
    fn channel_station_roundtrip() {
        for c in [0u16, 1, 0x0101, 0xFFFF] {
            assert_eq!(chan_of(station_of(c)), c);
        }
    }
}
