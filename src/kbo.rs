pub const NAME_BYTES: usize = 48;

pub const FAM_REST: u8 = 0;
pub const FAM_CLASSICAL: u8 = 1;
pub const FAM_SCATTERED: u8 = 2;
pub const FAM_3_2: u8 = 3;
pub const FAM_2_1: u8 = 4;
pub const FAM_5_2: u8 = 5;
pub const FAM_7_4: u8 = 6;
pub const FAM_4_3: u8 = 7;
pub const FAM_5_3: u8 = 8;
pub const FAM_ETNO: u8 = 9;

pub const MPC_ABSENT: u8 = 0;
pub const MPC_AGREE: u8 = 1;
pub const MPC_DISAGREE: u8 = 2;

const W_3_2: (f64, f64) = (39.2, 39.9);
const W_2_1: (f64, f64) = (47.6, 48.3);
const W_5_2: (f64, f64) = (54.9, 55.9);
const W_7_4: (f64, f64) = (43.4, 44.0);
const W_4_3: (f64, f64) = (36.2, 36.7);
const W_5_3: (f64, f64) = (42.0, 42.5);

#[derive(Clone, Copy)]
pub struct KboRec {
    pub name: [u8; NAME_BYTES],
    pub a_au: f64,
    pub e: f64,
    pub incl_deg: f64,
    pub node_deg: f64,
    pub peri_deg: f64,
    pub ma_deg: f64,
    pub epoch_jd: f64,
    pub h_mag: f64,
    pub family: u8,
    pub mpc_flag: u8,
}

pub fn family_of(a_au: f64, e: f64) -> u8 {
    if a_au >= 250.0 {
        return FAM_ETNO;
    }
    if within(a_au, W_3_2) {
        return FAM_3_2;
    }
    if within(a_au, W_2_1) {
        return FAM_2_1;
    }
    if within(a_au, W_5_2) {
        return FAM_5_2;
    }
    if within(a_au, W_7_4) {
        return FAM_7_4;
    }
    if within(a_au, W_4_3) {
        return FAM_4_3;
    }
    if within(a_au, W_5_3) {
        return FAM_5_3;
    }
    if a_au >= 38.0 && a_au <= 50.0 && e < 0.24 {
        return FAM_CLASSICAL;
    }
    if e >= 0.3 {
        return FAM_SCATTERED;
    }
    FAM_REST
}

fn within(v: f64, w: (f64, f64)) -> bool {
    v >= w.0 && v <= w.1
}

pub fn family_name(f: u8) -> &'static str {
    match f {
        FAM_CLASSICAL => "klassisch",
        FAM_SCATTERED => "gestreut",
        FAM_3_2 => "3:2",
        FAM_2_1 => "2:1",
        FAM_5_2 => "5:2",
        FAM_7_4 => "7:4",
        FAM_4_3 => "4:3",
        FAM_5_3 => "5:3",
        FAM_ETNO => "etno",
        _ => "uebrig",
    }
}

pub fn name_of(rec: &KboRec) -> &str {
    let end = rec.name.iter().position(|&b| b == 0).unwrap_or(NAME_BYTES);
    std::str::from_utf8(&rec.name[..end]).unwrap_or("")
}

pub fn write_json(recs: &[KboRec]) -> Vec<u8> {
    let mut out = String::with_capacity(recs.len() * 200);
    out.push_str("{\"data\":[");
    for (i, r) in recs.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"full_name\":{},\"a\":{},\"e\":{},\"i\":{},\"om\":{},\"w\":{},\"ma\":{},\"epoch\":{},\"H\":{},\"class\":\"TNO\",\"family\":{},\"mpc_flag\":{}}}",
            json_string(name_of(r)),
            r.a_au,
            r.e,
            r.incl_deg,
            r.node_deg,
            r.peri_deg,
            r.ma_deg,
            r.epoch_jd,
            r.h_mag,
            r.family,
            r.mpc_flag
        ));
    }
    out.push_str("]}");
    out.into_bytes()
}

fn json_string(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => o.push(' '),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

pub fn state_at(rec: &KboRec, t_jd: f64) -> Option<([f64; 3], [f64; 3])> {
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

pub fn packed_epoch_to_jd(text: &str) -> Option<f64> {
    let t = text.trim();
    let b = t.as_bytes();
    if b.len() < 5 {
        return None;
    }
    let century = match b[0] as char {
        'I' => 1800i64,
        'J' => 1900,
        'K' => 2000,
        'L' => 2100,
        _ => return None,
    };
    let year = century + std::str::from_utf8(&b[1..3]).ok()?.parse::<i64>().ok()?;
    let month = pack_char(b[3])?;
    let day = pack_char(b[4])?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let frac = if let Some(dot) = t.find('.') {
        format!("0{}", &t[dot..]).parse::<f64>().ok()?
    } else {
        0.0
    };
    let days = crate::lsk::days_from_civil(year, month, day)?;
    Some(days as f64 + 2440587.5 + frac)
}

fn pack_char(c: u8) -> Option<i64> {
    match c {
        b'0'..=b'9' => Some((c - b'0') as i64),
        b'A'..=b'V' => Some((c - b'A' + 10) as i64),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::JsonVal;

    fn rec(name: &str, a: f64, e: f64) -> KboRec {
        let mut nm = [0u8; NAME_BYTES];
        nm[..name.len()].copy_from_slice(name.as_bytes());
        KboRec {
            name: nm,
            a_au: a,
            e,
            incl_deg: 0.0,
            node_deg: 0.0,
            peri_deg: 0.0,
            ma_deg: 0.0,
            epoch_jd: 2461200.5,
            h_mag: 7.0,
            family: family_of(a, e),
            mpc_flag: MPC_ABSENT,
        }
    }

    #[test]
    fn family_windows() {
        assert_eq!(family_of(39.5, 0.2), FAM_3_2);
        assert_eq!(family_of(48.0, 0.1), FAM_2_1);
        assert_eq!(family_of(55.2, 0.4), FAM_5_2);
        assert_eq!(family_of(43.7, 0.2), FAM_7_4);
        assert_eq!(family_of(36.4, 0.1), FAM_4_3);
        assert_eq!(family_of(42.2, 0.2), FAM_5_3);
        assert_eq!(family_of(42.7, 0.1), FAM_CLASSICAL);
        assert_eq!(family_of(80.0, 0.5), FAM_SCATTERED);
        assert_eq!(family_of(506.0, 0.86), FAM_ETNO);
        assert_eq!(family_of(52.0, 0.1), FAM_REST);
    }

    #[test]
    fn json_catalog_shape() {
        let recs = vec![
            rec(
                "15760 Albion (1992 QB1)",
                44.13128015101105,
                0.07115576064918994,
            ),
            rec("90377 Sedna", 543.7195, 0.85988),
        ];
        let bytes = write_json(&recs);
        let root = crate::json::parse_json(std::str::from_utf8(&bytes).unwrap()).unwrap();
        let JsonVal::Obj(m) = &root else {
            panic!("expected object");
        };
        let JsonVal::Arr(rows) = m.get("data").unwrap() else {
            panic!("expected data array");
        };
        assert_eq!(rows.len(), 2);
        assert_eq!(
            crate::json::jstr(&rows[0], "full_name").unwrap(),
            "15760 Albion (1992 QB1)"
        );
        assert!((crate::json::jnum(&rows[0], "a").unwrap() - 44.13128015101105).abs() < 1e-12);
        assert_eq!(
            crate::json::jnum(&rows[1], "family").unwrap() as u8,
            FAM_ETNO
        );
    }

    #[test]
    fn packed_epoch_decodes() {
        let jd = packed_epoch_to_jd("K2669").unwrap();
        assert!((jd - 2461200.5).abs() < 1e-9);
        assert!((packed_epoch_to_jd("K2669.5").unwrap() - 2461201.0).abs() < 1e-9);
        assert!((packed_epoch_to_jd("K023R").unwrap() - 2452360.5).abs() < 1e-9);
    }
}
