pub const OSSOS_DESIG_BYTES: usize = 7;
pub const OSSOS_RECORD_STRIDE: usize = 79;
pub const MJD_TO_JD: f64 = 2400000.5;

#[derive(Clone, Copy, Debug)]
pub struct OssosRec {
    pub desig: [u8; OSSOS_DESIG_BYTES],
    pub a_au: f64,
    pub e: f64,
    pub i_deg: f64,
    pub node_deg: f64,
    pub peri_deg: f64,
    pub tperi_jd: f64,
    pub sigma_a_au: f64,
    pub sigma_e: f64,
    pub sigma_i_deg: f64,
}

pub fn encode_record(rec: &OssosRec, out: &mut Vec<u8>) {
    out.extend_from_slice(&rec.desig);
    out.extend_from_slice(&rec.a_au.to_le_bytes());
    out.extend_from_slice(&rec.e.to_le_bytes());
    out.extend_from_slice(&rec.i_deg.to_le_bytes());
    out.extend_from_slice(&rec.node_deg.to_le_bytes());
    out.extend_from_slice(&rec.peri_deg.to_le_bytes());
    out.extend_from_slice(&rec.tperi_jd.to_le_bytes());
    out.extend_from_slice(&rec.sigma_a_au.to_le_bytes());
    out.extend_from_slice(&rec.sigma_e.to_le_bytes());
    out.extend_from_slice(&rec.sigma_i_deg.to_le_bytes());
}

fn f64_at(buf: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(buf.get(off..off + 8)?.try_into().ok()?))
}

pub fn parse_record(buf: &[u8]) -> Option<OssosRec> {
    if buf.len() < OSSOS_RECORD_STRIDE {
        return None;
    }
    let mut desig = [0u8; OSSOS_DESIG_BYTES];
    desig.copy_from_slice(&buf[0..OSSOS_DESIG_BYTES]);
    Some(OssosRec {
        desig,
        a_au: f64_at(buf, 7)?,
        e: f64_at(buf, 15)?,
        i_deg: f64_at(buf, 23)?,
        node_deg: f64_at(buf, 31)?,
        peri_deg: f64_at(buf, 39)?,
        tperi_jd: f64_at(buf, 47)?,
        sigma_a_au: f64_at(buf, 55)?,
        sigma_e: f64_at(buf, 63)?,
        sigma_i_deg: f64_at(buf, 71)?,
    })
}

pub fn desig_of(rec: &OssosRec) -> &str {
    std::str::from_utf8(&rec.desig).unwrap_or("").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stride_roundtrip() {
        let mut desig = [b' '; OSSOS_DESIG_BYTES];
        desig[..7].copy_from_slice(b"K02GG6G");
        let rec = OssosRec {
            desig,
            a_au: 44.0,
            e: 0.1,
            i_deg: 2.0,
            node_deg: 80.0,
            peri_deg: 70.0,
            tperi_jd: 2457389.0,
            sigma_a_au: 0.01,
            sigma_e: 0.001,
            sigma_i_deg: 0.01,
        };
        let mut buf = Vec::new();
        encode_record(&rec, &mut buf);
        assert_eq!(buf.len(), OSSOS_RECORD_STRIDE);
        let back = parse_record(&buf).unwrap();
        assert_eq!(desig_of(&back), "K02GG6G");
        assert_eq!(back.a_au, 44.0);
        assert_eq!(back.tperi_jd, 2457389.0);
    }
}
