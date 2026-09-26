use crate::archivar::kepler::{AU_M, GM_SUN_M3_S2, KeplerElements, elements_to_icrs_state};

pub const MAGIC: [u8; 4] = *b"CTL1";

pub const DESIG_BYTES: usize = 40;

pub const RECORD_STRIDE: usize = DESIG_BYTES + 7 * 8;

#[derive(Clone, Copy, Debug)]
pub struct CometelsRec {
    pub desig: [u8; DESIG_BYTES],
    pub epoch_jd: f64,
    pub e: f64,
    pub q_au: f64,
    pub incl_deg: f64,
    pub node_deg: f64,
    pub peri_deg: f64,
    pub tp_jd: f64,
}

impl CometelsRec {
    pub fn state_at(&self, t_jd: f64) -> Option<([f64; 3], [f64; 3])> {
        if !self.e.is_finite() || !(0.0..1.0).contains(&self.e) {
            return None;
        }
        if !self.q_au.is_finite() || self.q_au <= 0.0 {
            return None;
        }
        let a_au = self.q_au / (1.0 - self.e);
        let a_m = a_au * AU_M;
        let n = (GM_SUN_M3_S2 / a_m.powi(3)).sqrt();
        let ma_deg = ((n * (self.epoch_jd - self.tp_jd) * 86400.0)
            .rem_euclid(std::f64::consts::TAU))
        .to_degrees();
        elements_to_icrs_state(&KeplerElements {
            a_au,
            e: self.e,
            incl_deg: self.incl_deg,
            node_deg: self.node_deg,
            peri_deg: self.peri_deg,
            ma_deg,
            epoch_jd: self.epoch_jd,
            t_jd,
        })
    }
}

pub fn desig_of(rec: &CometelsRec) -> &str {
    let end = rec
        .desig
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(DESIG_BYTES);
    std::str::from_utf8(&rec.desig[..end]).unwrap_or("")
}

fn f64_at(buf: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(buf.get(off..off + 8)?.try_into().ok()?))
}

pub fn encode_record(rec: &CometelsRec, out: &mut Vec<u8>) {
    out.extend_from_slice(&rec.desig);
    out.extend_from_slice(&rec.epoch_jd.to_le_bytes());
    out.extend_from_slice(&rec.e.to_le_bytes());
    out.extend_from_slice(&rec.q_au.to_le_bytes());
    out.extend_from_slice(&rec.incl_deg.to_le_bytes());
    out.extend_from_slice(&rec.node_deg.to_le_bytes());
    out.extend_from_slice(&rec.peri_deg.to_le_bytes());
    out.extend_from_slice(&rec.tp_jd.to_le_bytes());
}

pub fn parse_record(buf: &[u8]) -> Option<CometelsRec> {
    if buf.len() < RECORD_STRIDE {
        return None;
    }
    let mut desig = [0u8; DESIG_BYTES];
    desig.copy_from_slice(&buf[0..DESIG_BYTES]);
    let rec = CometelsRec {
        desig,
        epoch_jd: f64_at(buf, DESIG_BYTES)?,
        e: f64_at(buf, DESIG_BYTES + 8)?,
        q_au: f64_at(buf, DESIG_BYTES + 16)?,
        incl_deg: f64_at(buf, DESIG_BYTES + 24)?,
        node_deg: f64_at(buf, DESIG_BYTES + 32)?,
        peri_deg: f64_at(buf, DESIG_BYTES + 40)?,
        tp_jd: f64_at(buf, DESIG_BYTES + 48)?,
    };
    if !rec.epoch_jd.is_finite() || !rec.e.is_finite() || !(0.0..1.0).contains(&rec.e) {
        return None;
    }
    if !rec.q_au.is_finite() || rec.q_au <= 0.0 {
        return None;
    }
    if !rec.incl_deg.is_finite()
        || !rec.node_deg.is_finite()
        || !rec.peri_deg.is_finite()
        || !rec.tp_jd.is_finite()
    {
        return None;
    }
    Some(rec)
}

pub fn write_catalog(records: &[CometelsRec]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * RECORD_STRIDE);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        encode_record(r, &mut buf);
    }
    buf
}

pub fn parse_catalog(bytes: &[u8]) -> Vec<CometelsRec> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return Vec::new();
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap_or([0; 4])) as usize;
    if n > (bytes.len() - 8) / RECORD_STRIDE {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        if let Some(r) = bytes.get(off..off + RECORD_STRIDE).and_then(parse_record) {
            out.push(r);
        }
        off += RECORD_STRIDE;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encke() -> CometelsRec {
        let mut desig = [0u8; DESIG_BYTES];
        desig[..8].copy_from_slice(b"2P/Encke");
        CometelsRec {
            desig,
            epoch_jd: 2451545.0,
            e: 0.848,
            q_au: 0.336,
            incl_deg: 11.8,
            node_deg: 334.0,
            peri_deg: 186.0,
            tp_jd: 2451505.0,
        }
    }

    #[test]
    fn stride_roundtrip_keeps_every_element() {
        let mut buf = Vec::new();
        encode_record(&encke(), &mut buf);
        assert_eq!(buf.len(), RECORD_STRIDE);
        let back = parse_record(&buf).unwrap();
        assert_eq!(desig_of(&back), "2P/Encke");
        assert_eq!(back.epoch_jd, 2451545.0);
        assert_eq!(back.e, 0.848);
        assert_eq!(back.q_au, 0.336);
        assert_eq!(back.incl_deg, 11.8);
        assert_eq!(back.node_deg, 334.0);
        assert_eq!(back.peri_deg, 186.0);
        assert_eq!(back.tp_jd, 2451505.0);
    }

    #[test]
    fn catalog_roundtrip_counts_the_records() {
        let bytes = write_catalog(&[encke(), encke()]);
        let back = parse_catalog(&bytes);
        assert_eq!(back.len(), 2);
        assert_eq!(desig_of(&back[0]), "2P/Encke");
    }

    #[test]
    fn foreign_magic_reads_void() {
        assert!(parse_catalog(b"not a cometels catalog").is_empty());
    }

    #[test]
    fn implausible_elements_read_void() {
        let mut bad = encke();
        bad.e = 1.2;
        let mut buf = Vec::new();
        encode_record(&bad, &mut buf);
        assert!(parse_record(&buf).is_none());
        let mut void = encke();
        void.q_au = -1.0;
        let mut buf = Vec::new();
        encode_record(&void, &mut buf);
        assert!(parse_record(&buf).is_none());
    }

    #[test]
    fn state_at_epoch_matches_two_body_radius() {
        let rec = encke();
        let (p, _) = rec.state_at(rec.epoch_jd).unwrap();
        let r_au = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt() / AU_M;
        let a_au = rec.q_au / (1.0 - rec.e);
        let a_m = a_au * AU_M;
        let n = (GM_SUN_M3_S2 / a_m.powi(3)).sqrt();
        let ma_rad = (n * (rec.epoch_jd - rec.tp_jd) * 86400.0).rem_euclid(std::f64::consts::TAU);
        let ecc = crate::archivar::kepler::solve_kepler_ecc(ma_rad, rec.e);
        let expect = a_au * (1.0 - rec.e * ecc.cos());
        assert!(
            (r_au - expect).abs() < 1e-9,
            "r {r_au} au, expect {expect} au"
        );
    }
}
