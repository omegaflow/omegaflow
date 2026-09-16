use std::fs;

pub const BLOCK_BYTES: usize = 289;
pub const RECORD_BYTES: usize = 64;
pub const MAGIC: &[u8; 4] = b"DMTR";

pub const COMP_ORBIT: u32 = 1;
pub const COMP_NE: u32 = 2;
pub const COMP_NI: u32 = 3;
pub const COMP_TE: u32 = 4;
pub const COMP_VF: u32 = 5;
pub const COMP_VI0: u32 = 6;

#[derive(Debug, Clone, Copy)]
pub struct DemeterBlock {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub orbit: f64,
    pub ne: f32,
    pub ni: f32,
    pub te: f32,
    pub vf: f32,
    pub vi0: f32,
    pub vs: f32,
}

impl DemeterBlock {
    pub fn unix_seconds(&self) -> i64 {
        days_from_civil(self.year as i64, self.month as i64, self.day as i64) * 86400
            + self.hour as i64 * 3600
            + self.minute as i64 * 60
            + self.second as i64
    }
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn read_u16_be(bytes: &[u8], off: usize) -> u16 {
    u16::from_be_bytes([bytes[off], bytes[off + 1]])
}

pub fn parse_block(bytes: &[u8]) -> Option<DemeterBlock> {
    if bytes.len() < BLOCK_BYTES {
        return None;
    }
    if &bytes[26..34] != b"TOULOUSE" {
        return None;
    }
    if &bytes[204..214] != b"ISL SURVEY" {
        return None;
    }
    let f32s = [
        f32::from_be_bytes([bytes[265], bytes[266], bytes[267], bytes[268]]),
        f32::from_be_bytes([bytes[269], bytes[270], bytes[271], bytes[272]]),
        f32::from_be_bytes([bytes[273], bytes[274], bytes[275], bytes[276]]),
        f32::from_be_bytes([bytes[277], bytes[278], bytes[279], bytes[280]]),
        f32::from_be_bytes([bytes[281], bytes[282], bytes[283], bytes[284]]),
        f32::from_be_bytes([bytes[285], bytes[286], bytes[287], bytes[288]]),
    ];
    Some(DemeterBlock {
        year: read_u16_be(bytes, 8),
        month: read_u16_be(bytes, 10),
        day: read_u16_be(bytes, 12),
        hour: read_u16_be(bytes, 14),
        minute: read_u16_be(bytes, 16),
        second: read_u16_be(bytes, 18),
        orbit: read_u16_be(bytes, 22) as f64,
        ne: f32s[0],
        ni: f32s[1],
        te: f32s[2],
        vf: f32s[3],
        vi0: f32s[4],
        vs: f32s[5],
    })
}

pub fn parse_blocks(data: &[u8]) -> Vec<DemeterBlock> {
    let mut out = Vec::new();
    let mut off = 0;
    while off + BLOCK_BYTES <= data.len() {
        if let Some(b) = parse_block(&data[off..off + BLOCK_BYTES]) {
            out.push(b);
        } else {
            break;
        }
        off += BLOCK_BYTES;
    }
    out
}

pub fn write_bin(blocks: &[DemeterBlock], out: &mut Vec<u8>) {
    let mut prev_vs: f32 = 0.0;
    for (i, b) in blocks.iter().enumerate() {
        let vs = if i == 0 { 0.0 } else { prev_vs };
        let rec = [
            vs as f64,
            b.unix_seconds() as f64,
            b.orbit,
            b.ne as f64,
            b.ni as f64,
            b.te as f64,
            b.vf as f64,
            b.vi0 as f64,
        ];
        for v in rec {
            out.extend_from_slice(&v.to_le_bytes());
        }
        prev_vs = b.vs;
    }
}

pub fn parse_bin(data: &[u8]) -> Vec<[f64; 8]> {
    let mut out = Vec::new();
    let mut off = 0;
    while off + RECORD_BYTES <= data.len() {
        let mut rec = [0.0; 8];
        for k in 0..8 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&data[off + k * 8..off + (k + 1) * 8]);
            rec[k] = f64::from_le_bytes(bytes);
        }
        out.push(rec);
        off += RECORD_BYTES;
    }
    out
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let recs = parse_bin(data);
    if recs.is_empty() {
        return None;
    }
    let mut out = Vec::with_capacity(recs.len() * 6);
    for rec in recs {
        let t = rec[1];
        if !t.is_finite() || t <= 0.0 {
            continue;
        }
        for (k, comp) in [
            (2usize, COMP_ORBIT),
            (3usize, COMP_NE),
            (4usize, COMP_NI),
            (5usize, COMP_TE),
            (6usize, COMP_VF),
            (7usize, COMP_VI0),
        ] {
            let v = rec[k];
            if v.is_finite() {
                out.push((t, v, comp));
            }
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

pub fn compile_file(path: &str) -> std::io::Result<(Vec<DemeterBlock>, Vec<u8>)> {
    let data = fs::read(path)?;
    let blocks = parse_blocks(&data);
    if blocks.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "no ISL blocks",
        ));
    }
    let mut bin = Vec::new();
    write_bin(&blocks, &mut bin);
    Ok((blocks, bin))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_isl_block() {
        let mut blk = vec![0u8; BLOCK_BYTES];
        blk[26..34].copy_from_slice(b"TOULOUSE");
        blk[204..214].copy_from_slice(b"ISL SURVEY");
        blk[8] = 0x07;
        blk[9] = 0xd4;
        blk[10] = 0x00;
        blk[11] = 0x08;
        blk[12] = 0x00;
        blk[13] = 0x0b;
        blk[14] = 0x00;
        blk[15] = 0x0f;
        blk[16] = 0x00;
        blk[17] = 0x39;
        blk[18] = 0x00;
        blk[19] = 0x24;
        blk[22] = 0x02;
        blk[23] = 0x49;
        blk[265..289].copy_from_slice(&[
            0x47, 0x2a, 0xbc, 0xb1, 0x47, 0x0c, 0xee, 0x33, 0x45, 0x43, 0x5a, 0xe9, 0x3f, 0x82,
            0x5a, 0x97, 0xbd, 0xf5, 0xc2, 0x8e, 0xbd, 0xd9, 0x10, 0xc5,
        ]);
        let b = parse_block(&blk).expect("block parses");
        assert_eq!(b.year, 2004);
        assert_eq!(b.month, 8);
        assert_eq!(b.day, 11);
        assert_eq!(b.hour, 15);
        assert_eq!(b.minute, 57);
        assert_eq!(b.second, 36);
        assert_eq!(b.orbit, 585.0);
        assert!((b.ne - 43_708.69).abs() < 0.01);
        assert!((b.ni - 36_078.2).abs() < 0.01);
        assert!((b.te - 3125.682).abs() < 0.01);
        assert!((b.vf - 1.0184).abs() < 0.001);
        assert!((b.vi0 + 0.12).abs() < 0.001);
        assert!((b.vs + 0.106).abs() < 0.001);
    }

    #[test]
    fn roundtrip_bin() {
        let mut blk = vec![0u8; BLOCK_BYTES];
        blk[26..34].copy_from_slice(b"TOULOUSE");
        blk[204..214].copy_from_slice(b"ISL SURVEY");
        blk[8] = 0x07;
        blk[9] = 0xd4;
        blk[10] = 0x00;
        blk[11] = 0x08;
        blk[12] = 0x00;
        blk[13] = 0x0b;
        blk[14] = 0x00;
        blk[15] = 0x0f;
        blk[16] = 0x00;
        blk[17] = 0x39;
        blk[18] = 0x00;
        blk[19] = 0x24;
        blk[22] = 0x02;
        blk[23] = 0x49;
        blk[265..289].copy_from_slice(&[
            0x47, 0x2a, 0xbc, 0xb1, 0x47, 0x0c, 0xee, 0x33, 0x45, 0x43, 0x5a, 0xe9, 0x3f, 0x82,
            0x5a, 0x97, 0xbd, 0xf5, 0xc2, 0x8e, 0xbd, 0xd9, 0x10, 0xc5,
        ]);
        let b = parse_block(&blk).unwrap();
        let mut bin = Vec::new();
        write_bin(&[b], &mut bin);
        let recs = parse_bin(&bin);
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0][1] as i64, 1092239856);
        assert_eq!(recs[0][2], 585.0);
    }

    #[test]
    fn parse_series_exposes_real_isl_record_components() {
        let mut blk = vec![0u8; BLOCK_BYTES];
        blk[26..34].copy_from_slice(b"TOULOUSE");
        blk[204..214].copy_from_slice(b"ISL SURVEY");
        blk[8] = 0x07;
        blk[9] = 0xd4;
        blk[10] = 0x00;
        blk[11] = 0x08;
        blk[12] = 0x00;
        blk[13] = 0x0b;
        blk[14] = 0x00;
        blk[15] = 0x0f;
        blk[16] = 0x00;
        blk[17] = 0x39;
        blk[18] = 0x00;
        blk[19] = 0x24;
        blk[22] = 0x02;
        blk[23] = 0x49;
        blk[265..289].copy_from_slice(&[
            0x47, 0x2a, 0xbc, 0xb1, 0x47, 0x0c, 0xee, 0x33, 0x45, 0x43, 0x5a, 0xe9, 0x3f, 0x82,
            0x5a, 0x97, 0xbd, 0xf5, 0xc2, 0x8e, 0xbd, 0xd9, 0x10, 0xc5,
        ]);
        let b = parse_block(&blk).unwrap();
        let mut bin = Vec::new();
        write_bin(&[b], &mut bin);
        let series = parse_series(&bin).expect("series parses");
        let t = 1092239856.0;
        assert_eq!(series.len(), 6);
        assert_eq!(series[0], (t, 585.0, COMP_ORBIT));
        assert_eq!(series[1], (t, b.ne as f64, COMP_NE));
        assert_eq!(series[2], (t, b.ni as f64, COMP_NI));
        assert_eq!(series[3], (t, b.te as f64, COMP_TE));
        assert_eq!(series[4], (t, b.vf as f64, COMP_VF));
        assert_eq!(series[5], (t, b.vi0 as f64, COMP_VI0));
    }

    #[test]
    fn parse_series_skips_absent_time_records() {
        let mut blk = vec![0u8; BLOCK_BYTES];
        blk[26..34].copy_from_slice(b"TOULOUSE");
        blk[204..214].copy_from_slice(b"ISL SURVEY");
        blk[8] = 0x07;
        blk[9] = 0xd4;
        blk[10] = 0x00;
        blk[11] = 0x08;
        blk[12] = 0x00;
        blk[13] = 0x0b;
        blk[14] = 0x00;
        blk[15] = 0x0f;
        blk[16] = 0x00;
        blk[17] = 0x39;
        blk[18] = 0x00;
        blk[19] = 0x24;
        blk[22] = 0x02;
        blk[23] = 0x49;
        blk[265..289].copy_from_slice(&[
            0x47, 0x2a, 0xbc, 0xb1, 0x47, 0x0c, 0xee, 0x33, 0x45, 0x43, 0x5a, 0xe9, 0x3f, 0x82,
            0x5a, 0x97, 0xbd, 0xf5, 0xc2, 0x8e, 0xbd, 0xd9, 0x10, 0xc5,
        ]);
        let b = parse_block(&blk).unwrap();
        let mut bin = Vec::new();
        write_bin(&[b], &mut bin);
        let absent = [0u8; RECORD_BYTES];
        bin.extend_from_slice(&absent);
        let series = parse_series(&bin).expect("series parses");
        assert_eq!(series.len(), 6);
    }

    #[test]
    fn rejects_non_survey_block() {
        let blk = vec![0u8; BLOCK_BYTES];
        assert!(parse_block(&blk).is_none());
    }
}
