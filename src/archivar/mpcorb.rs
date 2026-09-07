pub const DESIG_BYTES: usize = 16;
pub const RECORD_STRIDE: usize = 85;

pub const FLAG_H_PRESENT: u8 = 1;
pub const FLAG_G_PRESENT: u8 = 2;

#[derive(Clone, Copy, Debug)]
pub struct MpcorbRec {
    pub number: u32,
    pub desig: [u8; DESIG_BYTES],
    pub epoch_jd: f64,
    pub a_au: f64,
    pub e: f64,
    pub incl_deg: f64,
    pub node_deg: f64,
    pub peri_deg: f64,
    pub ma_deg: f64,
    pub h_mag: f32,
    pub g_mag: f32,
    pub flags: u8,
}

pub fn desig_of(rec: &MpcorbRec) -> &str {
    let end = rec
        .desig
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(DESIG_BYTES);
    std::str::from_utf8(&rec.desig[..end]).unwrap_or("")
}

pub fn number_text(number: u32) -> String {
    if number == 0 {
        String::new()
    } else {
        format!("{number}")
    }
}

pub fn encode_record(rec: &MpcorbRec, out: &mut Vec<u8>) {
    out.extend_from_slice(&rec.number.to_le_bytes());
    out.extend_from_slice(&rec.desig);
    out.extend_from_slice(&rec.epoch_jd.to_le_bytes());
    out.extend_from_slice(&rec.a_au.to_le_bytes());
    out.extend_from_slice(&rec.e.to_le_bytes());
    out.extend_from_slice(&rec.incl_deg.to_le_bytes());
    out.extend_from_slice(&rec.node_deg.to_le_bytes());
    out.extend_from_slice(&rec.peri_deg.to_le_bytes());
    out.extend_from_slice(&rec.ma_deg.to_le_bytes());
    out.extend_from_slice(&rec.h_mag.to_le_bytes());
    out.extend_from_slice(&rec.g_mag.to_le_bytes());
    out.push(rec.flags);
}

fn f32_at(buf: &[u8], off: usize) -> Option<f32> {
    Some(f32::from_le_bytes(buf.get(off..off + 4)?.try_into().ok()?))
}

fn f64_at(buf: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(buf.get(off..off + 8)?.try_into().ok()?))
}

pub fn parse_record(buf: &[u8]) -> Option<MpcorbRec> {
    if buf.len() < RECORD_STRIDE {
        return None;
    }
    let mut desig = [0u8; DESIG_BYTES];
    desig.copy_from_slice(&buf[4..4 + DESIG_BYTES]);
    Some(MpcorbRec {
        number: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        desig,
        epoch_jd: f64_at(buf, 20)?,
        a_au: f64_at(buf, 28)?,
        e: f64_at(buf, 36)?,
        incl_deg: f64_at(buf, 44)?,
        node_deg: f64_at(buf, 52)?,
        peri_deg: f64_at(buf, 60)?,
        ma_deg: f64_at(buf, 68)?,
        h_mag: f32_at(buf, 76)?,
        g_mag: f32_at(buf, 80)?,
        flags: *buf.get(84)?,
    })
}

pub fn state_at(rec: &MpcorbRec, t_jd: f64) -> Option<([f64; 3], [f64; 3])> {
    crate::kepler::elements_to_icrs_state(
        rec.a_au,
        rec.e,
        rec.incl_deg,
        rec.node_deg,
        rec.peri_deg,
        rec.ma_deg,
        rec.epoch_jd,
        t_jd,
    )
}

fn number_of(text: &str) -> Option<u32> {
    let t = text.trim();
    let b = t.as_bytes();
    if b.len() >= 3 && b[0] == b'(' && b[b.len() - 1] == b')' {
        let inner = &t[1..b.len() - 1];
        if !inner.is_empty() && inner.bytes().all(|c| c.is_ascii_digit()) {
            return inner.parse::<u32>().ok();
        }
    }
    None
}

pub fn rec_from_object(obj: &crate::json::JsonVal) -> Option<MpcorbRec> {
    let desig = crate::json::jstr(obj, "Principal_desig")?;
    let desig = desig.trim();
    if desig.is_empty() {
        return None;
    }
    let epoch_jd = crate::json::jnum(obj, "Epoch")?;
    let a_au = crate::json::jnum(obj, "a")?;
    let e = crate::json::jnum(obj, "e")?;
    let incl_deg = crate::json::jnum(obj, "i")?;
    let node_deg = crate::json::jnum(obj, "Node")?;
    let peri_deg = crate::json::jnum(obj, "Peri")?;
    let ma_deg = crate::json::jnum(obj, "M")?;
    if !epoch_jd.is_finite() || !a_au.is_finite() || a_au <= 0.0 {
        return None;
    }
    if !e.is_finite() || e < 0.0 || e >= 1.0 {
        return None;
    }
    if !incl_deg.is_finite()
        || !node_deg.is_finite()
        || !peri_deg.is_finite()
        || !ma_deg.is_finite()
    {
        return None;
    }
    let mut desig_bytes = [0u8; DESIG_BYTES];
    let bytes = desig.as_bytes();
    if bytes.len() > DESIG_BYTES {
        return None;
    }
    desig_bytes[..bytes.len()].copy_from_slice(bytes);
    let number = match crate::json::jstr(obj, "Number")
        .as_deref()
        .and_then(number_of)
    {
        Some(n) => n,
        None => 0,
    };
    let mut flags = 0u8;
    let mut h_mag = 0.0f32;
    let mut g_mag = 0.0f32;
    if let Some(h) = crate::json::jnum(obj, "H") {
        if h.is_finite() {
            h_mag = h as f32;
            flags |= FLAG_H_PRESENT;
        }
    }
    if let Some(g) = crate::json::jnum(obj, "G") {
        if g.is_finite() {
            g_mag = g as f32;
            flags |= FLAG_G_PRESENT;
        }
    }
    Some(MpcorbRec {
        number,
        desig: desig_bytes,
        epoch_jd,
        a_au,
        e,
        incl_deg,
        node_deg,
        peri_deg,
        ma_deg,
        h_mag,
        g_mag,
        flags,
    })
}

pub fn is_distant_object(obj: &crate::json::JsonVal) -> bool {
    crate::json::jstr(obj, "Orbit_type").as_deref() == Some("Distant Object")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::parse_json;

    fn ceres() -> MpcorbRec {
        let mut desig = [0u8; DESIG_BYTES];
        desig[..7].copy_from_slice(b"A801 AA");
        MpcorbRec {
            number: 1,
            desig,
            epoch_jd: 2461200.5,
            a_au: 2.7655526,
            e: 0.0796923,
            incl_deg: 10.58803,
            node_deg: 80.24863,
            peri_deg: 73.2942,
            ma_deg: 274.41935,
            h_mag: 3.34,
            g_mag: 0.15,
            flags: FLAG_H_PRESENT | FLAG_G_PRESENT,
        }
    }

    #[test]
    fn stride_roundtrip() {
        let mut buf = Vec::new();
        encode_record(&ceres(), &mut buf);
        assert_eq!(buf.len(), RECORD_STRIDE);
        let rec = parse_record(&buf).unwrap();
        assert_eq!(rec.number, 1);
        assert_eq!(desig_of(&rec), "A801 AA");
        assert!((rec.a_au - 2.7655526).abs() < 1e-15);
        assert!((rec.ma_deg - 274.41935).abs() < 1e-15);
        assert!((rec.h_mag - 3.34).abs() < 1e-6);
        assert_eq!(rec.flags, FLAG_H_PRESENT | FLAG_G_PRESENT);
    }

    #[test]
    fn rec_from_object_reads_measured_ceres() {
        let obj = parse_json(
            r#"{
"Number": "(1)",
"Name": "Ceres",
"Principal_desig": "A801 AA",
"Epoch": 2461200.5,
"M": 274.41935,
"Peri": 73.2942,
"Node": 80.24863,
"i": 10.58803,
"e": 0.0796923,
"n": 0.21430445,
"a": 2.7655526,
"Tp": 2461599.84154,
"H": 3.34,
"G": 0.15,
"Orbit_type": "MBA"
}"#,
        )
        .unwrap();
        let rec = rec_from_object(&obj).unwrap();
        assert_eq!(rec.number, 1);
        assert_eq!(desig_of(&rec), "A801 AA");
        assert_eq!(rec.flags, FLAG_H_PRESENT | FLAG_G_PRESENT);
        assert!(!is_distant_object(&obj));
    }

    #[test]
    fn rec_from_object_reads_measured_distant_tno() {
        let obj = parse_json(
            r#"{
"Principal_desig": "2004 PC112",
"Epoch": 2453240.5,
"M": 0.03358,
"Peri": 277.30161,
"Node": 69.53412,
"i": 2.35778,
"e": 0.0417836,
"n": 0.00333727,
"a": 44.348104,
"Tp": 2453230.43767,
"Orbit_type": "Distant Object"
}"#,
        )
        .unwrap();
        assert!(is_distant_object(&obj));
        let rec = rec_from_object(&obj).unwrap();
        assert_eq!(rec.number, 0);
        assert_eq!(desig_of(&rec), "2004 PC112");
        assert_eq!(rec.flags, 0);
        assert!((rec.a_au - 44.348104).abs() < 1e-12);
    }

    #[test]
    fn rec_from_object_reads_numbered_distant() {
        let obj = parse_json(
            r#"{
"Number": "(90377)",
"Name": "Sedna",
"Principal_desig": "2003 VB12",
"Epoch": 2461200.5,
"M": 358.596,
"Peri": 311.099,
"Node": 144.506,
"i": 11.925,
"e": 0.85988,
"n": 0.0000158,
"a": 543.7195,
"Tp": 2472322.4,
"H": 1.5,
"G": 0.15,
"Orbit_type": "Distant Object"
}"#,
        )
        .unwrap();
        assert!(is_distant_object(&obj));
        let rec = rec_from_object(&obj).unwrap();
        assert_eq!(rec.number, 90377);
        assert_eq!(desig_of(&rec), "2003 VB12");
        assert_eq!(rec.flags, FLAG_H_PRESENT | FLAG_G_PRESENT);
    }

    #[test]
    fn hyperbolic_and_nonpositive_a_rejected() {
        let obj = parse_json(
            r#"{"Principal_desig":"X","Epoch":2451545.0,"M":1.0,"Peri":1.0,"Node":1.0,"i":1.0,"e":1.2,"a":2.0,"Orbit_type":"Distant Object"}"#,
        )
        .unwrap();
        assert!(rec_from_object(&obj).is_none());
    }

    #[test]
    fn state_at_epoch_reaches_measured_sedna_scale() {
        let obj = parse_json(
            r#"{
"Number": "(90377)",
"Principal_desig": "2003 VB12",
"Epoch": 2461200.5,
"M": 358.596,
"Peri": 311.099,
"Node": 144.506,
"i": 11.925,
"e": 0.85988,
"a": 543.7195,
"H": 1.5,
"G": 0.15,
"Orbit_type": "Distant Object"
}"#,
        )
        .unwrap();
        let rec = rec_from_object(&obj).unwrap();
        let (p, _) = state_at(&rec, rec.epoch_jd).unwrap();
        let r_au = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt() / crate::kepler::AU_M;
        let ecc = crate::kepler::solve_kepler_ecc(rec.ma_deg.to_radians(), rec.e);
        let expect = rec.a_au * (1.0 - rec.e * ecc.cos());
        assert!(
            (r_au - expect).abs() < 1e-9,
            "r {r_au} au, expect {expect} au"
        );
    }
}
