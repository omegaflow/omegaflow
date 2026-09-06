use crate::weberin::nadel_gate::{sep_arcsec, UA};
use std::collections::HashMap;
use std::process::Command;

pub const FINK_LSST_OBJECTS: &str = "https://api.lsst.fink-portal.org/api/v1/objects";
pub const FINK_LSST_CONESEARCH: &str = "https://api.lsst.fink-portal.org/api/v1/conesearch";
pub const FINK_LSST_CLASS: &str = "f:main_label_classifier";
pub const FINK_LSST_SIMBAD: &str = "f:xm_simbad_otype";
pub const FINK_LSST_CLASS_ABSENT: i64 = -1;
pub const FINK_LSST_SIMBAD_ABSENT: [&str; 2] = ["Fail", "nan"];
pub const FINK_LSST_BROKER: &str = "Fink-LSST";

pub struct BrokerVerdict {
    pub broker: &'static str,
    pub dia_object_id: Option<String>,
    pub ra_deg: Option<f64>,
    pub dec_deg: Option<f64>,
    pub class: i64,
    pub simbad_otype: Option<String>,
    pub probability: Option<f64>,
    pub http_code: String,
}

impl BrokerVerdict {
    fn new(broker: &'static str, http_code: &str) -> BrokerVerdict {
        BrokerVerdict {
            broker,
            dia_object_id: None,
            ra_deg: None,
            dec_deg: None,
            class: FINK_LSST_CLASS_ABSENT,
            simbad_otype: None,
            probability: None,
            http_code: http_code.to_string(),
        }
    }
}

pub fn class_reads_natural(class: i64) -> bool {
    class != FINK_LSST_CLASS_ABSENT
}

pub fn simbad_otype_known(otype: Option<&str>) -> bool {
    match otype {
        Some(t) => !FINK_LSST_SIMBAD_ABSENT.contains(&t),
        None => false,
    }
}

fn obj_str<'a>(m: &'a HashMap<String, omegaflow::json::JsonVal>, key: &str) -> Option<&'a str> {
    match m.get(key) {
        Some(omegaflow::json::JsonVal::Str(s)) => Some(s),
        _ => None,
    }
}

fn obj_f64(m: &HashMap<String, omegaflow::json::JsonVal>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(omegaflow::json::JsonVal::Num(n)) if n.is_finite() => Some(*n),
        _ => None,
    }
}

fn obj_i64(m: &HashMap<String, omegaflow::json::JsonVal>, key: &str) -> Option<i64> {
    match m.get(key) {
        Some(omegaflow::json::JsonVal::Num(n)) if n.is_finite() && n.fract() == 0.0 => {
            Some(*n as i64)
        }
        _ => None,
    }
}

fn parse_classified_rows(body: &[u8]) -> Option<Vec<HashMap<String, omegaflow::json::JsonVal>>> {
    let text = std::str::from_utf8(body).ok()?;
    let root = omegaflow::json::parse_json(text)?;
    let omegaflow::json::JsonVal::Arr(rows) = root else {
        return None;
    };
    let mut out = Vec::new();
    for r in rows {
        if let omegaflow::json::JsonVal::Obj(m) = r {
            out.push(m);
        }
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

fn classify_of(m: &HashMap<String, omegaflow::json::JsonVal>) -> i64 {
    obj_i64(m, FINK_LSST_CLASS).unwrap_or(FINK_LSST_CLASS_ABSENT)
}

pub fn parse_objects_verdict(body: &[u8]) -> Option<BrokerVerdict> {
    let rows = parse_classified_rows(body)?;
    let m = rows.into_iter().next()?;
    let mut v = BrokerVerdict::new(FINK_LSST_BROKER, "200");
    v.class = classify_of(&m);
    v.ra_deg = obj_f64(&m, "r:ra");
    v.dec_deg = obj_f64(&m, "r:dec");
    Some(v)
}

pub fn parse_cone_verdict(body: &[u8], ra: f64, dec: f64) -> Option<BrokerVerdict> {
    let rows = parse_classified_rows(body)?;
    let mut best: Option<(f64, HashMap<String, omegaflow::json::JsonVal>)> = None;
    for m in rows {
        let s_deg = obj_f64(&m, "v:separation_degree");
        let sep = match s_deg {
            Some(s) => s * 3600.0,
            None => {
                let (Some(r), Some(d)) = (obj_f64(&m, "r:ra"), obj_f64(&m, "r:dec")) else {
                    continue;
                };
                sep_arcsec(ra, dec, r, d)
            }
        };
        match &best {
            Some((b, _)) if *b <= sep => {}
            _ => best = Some((sep, m)),
        }
    }
    let (_, m) = best?;
    let mut v = BrokerVerdict::new(FINK_LSST_BROKER, "200");
    v.class = classify_of(&m);
    v.ra_deg = obj_f64(&m, "r:ra");
    v.dec_deg = obj_f64(&m, "r:dec");
    v.simbad_otype = obj_str(&m, FINK_LSST_SIMBAD).map(str::to_string);
    Some(v)
}

fn curl_post(url: &str, json_body: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-A")
        .arg(UA)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(json_body)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn register_line(v: &BrokerVerdict) {
    let class_word = if v.class == FINK_LSST_CLASS_ABSENT {
        "no class (-1)".to_string()
    } else {
        format!("class {}", v.class)
    };
    let id = match v.dia_object_id.as_deref() {
        Some(i) => i.to_string(),
        None => match (v.ra_deg, v.dec_deg) {
            (Some(r), Some(d)) => format!("ra {r:.4} dec {d:.4}"),
            _ => "the queried position".to_string(),
        },
    };
    println!(
        "Borrowed sense ({}): {} — HTTP {}, registered verdict: {}; classifier probability absent (the anonymous Fink-LSST surface carries none — measured)",
        v.broker, id, v.http_code, class_word
    );
}

pub fn fink_object_witness(id: &str) -> Option<BrokerVerdict> {
    let payload = format!("{{\"diaObjectId\": \"{id}\"}}");
    let Some((code, body)) = curl_post(FINK_LSST_OBJECTS, &payload) else {
        println!(
            "Borrowed sense (Fink-LSST /objects {id}): the query did not answer (measured stall) — pending"
        );
        return None;
    };
    if code != "200" {
        println!(
            "Borrowed sense (Fink-LSST /objects {id}): the endpoint answered HTTP {code} — pending"
        );
        return None;
    }
    let mut v = match parse_objects_verdict(&body) {
        Some(v) => v,
        None => {
            println!(
                "Borrowed sense (Fink-LSST /objects {id}): the HTTP {code} body is not an object row array — parser pending on the real schema"
            );
            return None;
        }
    };
    v.dia_object_id = Some(id.to_string());
    v.http_code = code;
    register_line(&v);
    Some(v)
}

pub fn fink_cone_witness(ra: f64, dec: f64, radius_arcsec: f64) -> Option<BrokerVerdict> {
    let payload = format!(
        "{{\"ra\": {ra}, \"dec\": {dec}, \"radius\": {radius_arcsec}, \"columns\": \"r:ra,r:dec,{},{}\"}}",
        FINK_LSST_CLASS, FINK_LSST_SIMBAD
    );
    let Some((code, body)) = curl_post(FINK_LSST_CONESEARCH, &payload) else {
        println!(
            "Borrowed sense (Fink-LSST conesearch ra {ra:.4} dec {dec:.4}): the query did not answer (measured stall) — pending"
        );
        return None;
    };
    if code != "200" {
        println!(
            "Borrowed sense (Fink-LSST conesearch ra {ra:.4} dec {dec:.4}): the endpoint answered HTTP {code} — pending"
        );
        return None;
    }
    let mut v = match parse_cone_verdict(&body, ra, dec) {
        Some(v) => v,
        None => {
            println!(
                "Borrowed sense (Fink-LSST conesearch ra {ra:.4} dec {dec:.4}): the HTTP {code} body is not a row array — parser pending on the real schema"
            );
            return None;
        }
    };
    v.http_code = code;
    register_line(&v);
    Some(v)
}

pub fn row_verdict(id: &str, ra: f64, dec: f64, class: i64) -> BrokerVerdict {
    let mut v = BrokerVerdict::new(FINK_LSST_BROKER, "200");
    v.dia_object_id = Some(id.to_string());
    v.ra_deg = Some(ra);
    v.dec_deg = Some(dec);
    v.class = class;
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objects_verdict_reads_the_measured_classifier_schema() {
        let body = br#"[{"f:main_label_classifier":11,"r:ra":148.8341540938,"r:dec":2.5448708154,"r:diaObjectId":170059285657550883}]"#;
        let v = parse_objects_verdict(body).unwrap();
        assert_eq!(v.class, 11);
        assert!((v.ra_deg.unwrap() - 148.8341540938).abs() < 1e-9);
        assert!(class_reads_natural(v.class));
    }

    #[test]
    fn objects_verdict_keeps_the_absent_class_sentinel() {
        let body =
            br#"[{"f:main_label_classifier":-1,"r:ra":148.8371916933,"r:dec":2.5476495154}]"#;
        let v = parse_objects_verdict(body).unwrap();
        assert_eq!(v.class, FINK_LSST_CLASS_ABSENT);
        assert!(!class_reads_natural(v.class));
    }

    #[test]
    fn cone_verdict_picks_the_nearest_row() {
        let body = br#"[{"f:main_label_classifier":-1,"r:ra":148.8371916933,"r:dec":2.5476495154,"f:xm_simbad_otype":"nan"},{"f:main_label_classifier":21,"r:ra":148.8310542298,"r:dec":2.5593009344,"f:xm_simbad_otype":"Fail"}]"#;
        let v = parse_cone_verdict(body, 148.8372, 2.5476).unwrap();
        assert_eq!(v.class, FINK_LSST_CLASS_ABSENT);
        let v2 = parse_cone_verdict(body, 148.8311, 2.5593).unwrap();
        assert_eq!(v2.class, 21);
        assert_eq!(v2.simbad_otype.as_deref(), Some("Fail"));
        assert!(!simbad_otype_known(v2.simbad_otype.as_deref()));
    }

    #[test]
    fn absent_sentinels_do_not_read_as_simbad_otypes() {
        assert!(!simbad_otype_known(Some("Fail")));
        assert!(!simbad_otype_known(Some("nan")));
        assert!(!simbad_otype_known(None));
        assert!(simbad_otype_known(Some("SN")));
    }

    #[test]
    fn non_row_bodies_stay_unparsed() {
        assert!(parse_objects_verdict(b"{}").is_none());
        assert!(parse_objects_verdict(b"not json").is_none());
        assert!(parse_cone_verdict(b"{\"a\":1}", 0.0, 0.0).is_none());
    }
}
