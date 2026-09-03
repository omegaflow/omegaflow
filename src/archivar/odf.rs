pub const ORBIT_KEY: u32 = 1031;
pub const PK_FILE_LABEL: u32 = 101;
pub const PK_IDENTIFIER: u32 = 107;
pub const PK_ORBIT_HEADER: u32 = 109;
pub const PK_RAMP: u32 = 2030;
pub const PK_CLOCK: u32 = 2040;
pub const PK_SUMMARY: u32 = 105;

pub const ODF_TDB_OFFSET: f64 = -1577664000.0;

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
    let t_int = words[0] as i64;
    let t_frac = bits(words, 33, 42);
    let observable = twos(words[2] as i64, 32) as f64 + twos(words[3] as i64, 32) as f64 / 1.0e9;
    let dss_rx = bits(words, 132, 138);
    let dss_tx = bits(words, 139, 145);
    let data_type = bits(words, 148, 153);
    let downlink_band = bits(words, 154, 155);
    let uplink_band = bits(words, 156, 157);
    let valid = bits(words, 160, 160) == 0;
    let scid = bits(words, 168, 177);
    let ref_mhz = bits(words, 179, 224);
    let compression = bits(words, 245, 266);
    Some(OdOrbit {
        t_since_1950: t_int as f64 + t_frac as f64 / 1.0e10,
        observable_hz: observable,
        dss_rx,
        dss_tx,
        data_type,
        downlink_band,
        uplink_band,
        valid,
        scid,
        ref_hz: ref_mhz as f64 / 1000.0,
        compression_s: compression as f64 / 100.0,
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
    if bytes.len() % 36 != 0 {
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
                if in_orbit {
                    if let Some(r) = orbit_record(&words) {
                        out.push(r);
                    }
                }
            }
        }
    }
    Some(out)
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
    fn p11r_roundtrip() {
        let recs = vec![[-8.0e8, 12.5, 2.3e9, 2.3e9, -1.7e4, 43.0, 43.0, 12.0, 60.0]];
        let bytes = write_p11r_bin(&recs);
        let parsed = parse_p11r_bin(&bytes).unwrap();
        assert_eq!(parsed, recs);
        assert!(parse_p11r_bin(b"X").is_none());
    }
}
