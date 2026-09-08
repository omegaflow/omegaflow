pub const DES_Y6_DESIG_BYTES: usize = 12;
pub const DES_Y6_RECORD_STRIDE: usize = 76;
pub const DES_Y6_EPOCH_JD: f64 = 2457389.0;
pub const AU_M: f64 = 1.495978707e11;
pub const AU_YR_TO_M_S: f64 = 1.495978707e11 / 31557600.0;

#[derive(Clone, Copy, Debug)]
pub struct DesY6Rec {
    pub desig: [u8; DES_Y6_DESIG_BYTES],
    pub epoch_jd: f64,
    pub x_m: f64,
    pub y_m: f64,
    pub z_m: f64,
    pub vx_ms: f64,
    pub vy_ms: f64,
    pub vz_ms: f64,
    pub sigma_m: f64,
}

pub fn encode_record(rec: &DesY6Rec, out: &mut Vec<u8>) {
    out.extend_from_slice(&rec.desig);
    out.extend_from_slice(&rec.epoch_jd.to_le_bytes());
    out.extend_from_slice(&rec.x_m.to_le_bytes());
    out.extend_from_slice(&rec.y_m.to_le_bytes());
    out.extend_from_slice(&rec.z_m.to_le_bytes());
    out.extend_from_slice(&rec.vx_ms.to_le_bytes());
    out.extend_from_slice(&rec.vy_ms.to_le_bytes());
    out.extend_from_slice(&rec.vz_ms.to_le_bytes());
    out.extend_from_slice(&rec.sigma_m.to_le_bytes());
}

fn f64_at(buf: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(buf.get(off..off + 8)?.try_into().ok()?))
}

pub fn parse_record(buf: &[u8]) -> Option<DesY6Rec> {
    if buf.len() < DES_Y6_RECORD_STRIDE {
        return None;
    }
    let mut desig = [0u8; DES_Y6_DESIG_BYTES];
    desig.copy_from_slice(&buf[0..DES_Y6_DESIG_BYTES]);
    Some(DesY6Rec {
        desig,
        epoch_jd: f64_at(buf, 12)?,
        x_m: f64_at(buf, 20)?,
        y_m: f64_at(buf, 28)?,
        z_m: f64_at(buf, 36)?,
        vx_ms: f64_at(buf, 44)?,
        vy_ms: f64_at(buf, 52)?,
        vz_ms: f64_at(buf, 60)?,
        sigma_m: f64_at(buf, 68)?,
    })
}

pub fn desig_of(rec: &DesY6Rec) -> &str {
    std::str::from_utf8(&rec.desig).unwrap_or("").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stride_roundtrip() {
        let mut desig = [b' '; DES_Y6_DESIG_BYTES];
        desig[..9].copy_from_slice(b"2013 RQ98");
        let rec = DesY6Rec {
            desig,
            epoch_jd: DES_Y6_EPOCH_JD,
            x_m: 1.0e12,
            y_m: -2.0e12,
            z_m: 3.0e12,
            vx_ms: 1.0,
            vy_ms: -2.0,
            vz_ms: 3.0,
            sigma_m: 4.0e8,
        };
        let mut buf = Vec::new();
        encode_record(&rec, &mut buf);
        assert_eq!(buf.len(), DES_Y6_RECORD_STRIDE);
        let back = parse_record(&buf).unwrap();
        assert_eq!(desig_of(&back), "2013 RQ98");
        assert_eq!(back.epoch_jd, DES_Y6_EPOCH_JD);
        assert_eq!(back.x_m, 1.0e12);
        assert_eq!(back.sigma_m, 4.0e8);
    }
}
