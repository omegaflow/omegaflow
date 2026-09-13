pub const MAGIC: [u8; 4] = *b"CRX1";

pub const REC_BYTES: usize = 72;

pub const PRES_POSITION: u32 = 1 << 0;
pub const PRES_FREQ: u32 = 1 << 1;
pub const PRES_BIN_WIDTH: u32 = 1 << 2;

pub struct CorsRecord {
    pub epoch: f64,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub freq: f64,
    pub bin_width: f64,
    pub value: f64,
    pub sat: [u8; 3],
    pub obs: [u8; 2],
    pub station: u32,
    pub present: u32,
}

pub fn pack_station(name: &str) -> u32 {
    let mut b = [b' '; 4];
    for (i, c) in name.bytes().take(4).enumerate() {
        b[i] = c.to_ascii_uppercase();
    }
    (b[0] as u32) << 24 | (b[1] as u32) << 16 | (b[2] as u32) << 8 | b[3] as u32
}

pub fn station_of(packed: u32) -> String {
    let b = [
        ((packed >> 24) & 0xff) as u8,
        ((packed >> 16) & 0xff) as u8,
        ((packed >> 8) & 0xff) as u8,
        (packed & 0xff) as u8,
    ];
    String::from_utf8_lossy(&b).trim_end().to_string()
}

pub fn satellite_of(sat: [u8; 3]) -> String {
    String::from_utf8_lossy(&sat).trim_end().to_string()
}

pub fn encode_rec(buf: &mut [u8; REC_BYTES], r: &CorsRecord) {
    buf[0..8].copy_from_slice(&r.epoch.to_le_bytes());
    buf[8..16].copy_from_slice(&r.lat.to_le_bytes());
    buf[16..24].copy_from_slice(&r.lon.to_le_bytes());
    buf[24..32].copy_from_slice(&r.alt.to_le_bytes());
    buf[32..40].copy_from_slice(&r.freq.to_le_bytes());
    buf[40..48].copy_from_slice(&r.bin_width.to_le_bytes());
    buf[48..56].copy_from_slice(&r.value.to_le_bytes());
    buf[56..59].copy_from_slice(&r.sat);
    buf[59..61].copy_from_slice(&r.obs);
    buf[64..68].copy_from_slice(&r.station.to_le_bytes());
    buf[68..72].copy_from_slice(&r.present.to_le_bytes());
}

pub fn decode_rec(buf: &[u8; REC_BYTES]) -> Option<CorsRecord> {
    let f = |a: usize| -> Option<f64> {
        buf.get(a..a + 8)
            .and_then(|x| x.try_into().ok())
            .map(f64::from_le_bytes)
    };
    let epoch = f(0)?;
    let lat = f(8)?;
    let lon = f(16)?;
    let alt = f(24)?;
    let freq = f(32)?;
    let bin_width = f(40)?;
    let value = f(48)?;
    let sat = [*buf.get(56)?, *buf.get(57)?, *buf.get(58)?];
    let obs = [*buf.get(59)?, *buf.get(60)?];
    let station = u32::from_le_bytes(buf.get(64..68)?.try_into().ok()?);
    let present = u32::from_le_bytes(buf.get(68..72)?.try_into().ok()?);
    if !epoch.is_finite() || !value.is_finite() {
        return None;
    }
    if present & PRES_POSITION != 0 && !(lat.is_finite() && lon.is_finite() && alt.is_finite()) {
        return None;
    }
    if present & PRES_FREQ != 0 && !freq.is_finite() {
        return None;
    }
    if present & PRES_BIN_WIDTH != 0 && !bin_width.is_finite() {
        return None;
    }
    Some(CorsRecord {
        epoch,
        lat,
        lon,
        alt,
        freq,
        bin_width,
        value,
        sat,
        obs,
        station,
        present,
    })
}

pub fn write_bin(records: &[CorsRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * REC_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        let mut rec = [0u8; REC_BYTES];
        encode_rec(&mut rec, r);
        buf.extend_from_slice(&rec);
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<CorsRecord>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let rec: &[u8; REC_BYTES] = bytes.get(off..off + REC_BYTES)?.try_into().ok()?;
        out.push(decode_rec(rec)?);
        off += REC_BYTES;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> CorsRecord {
        CorsRecord {
            epoch: 758_937_600.0,
            lat: 30.40741930,
            lon: -91.18025411,
            alt: -5.208,
            freq: 0.0,
            bin_width: 0.0,
            value: 21_345_678.123,
            sat: *b"G02",
            obs: *b"C1",
            station: pack_station("1lsu"),
            present: PRES_POSITION,
        }
    }

    #[test]
    fn roundtrip() {
        let records = vec![sample()];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        let r = &parsed[0];
        assert_eq!(r.epoch, 758_937_600.0);
        assert_eq!(r.sat, *b"G02");
        assert_eq!(r.obs, *b"C1");
        assert_eq!(station_of(r.station), "1LSU");
        assert_eq!(satellite_of(r.sat), "G02");
        assert!((r.value - 21_345_678.123).abs() < 1e-3);
        assert_eq!(r.present & PRES_POSITION, PRES_POSITION);
    }

    #[test]
    fn absent_position_carries_no_flag() {
        let mut r = sample();
        r.present = 0;
        r.lat = 0.0;
        r.lon = 0.0;
        r.alt = 0.0;
        let bytes = write_bin(&[r]);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].present & PRES_POSITION, 0);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"CRX1abc").is_none());
    }
}
