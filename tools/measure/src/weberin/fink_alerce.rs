use std::collections::HashMap;
use std::process::Command;

pub const UA: &str = "omegaflow-weberin-fink-alerce/1.0";

pub const FINK_ZTF_CONESEARCH: &str = "https://api.ztf.fink-portal.org/api/v1/conesearch";
pub const FINK_ZTF_OBJECTS: &str = "https://api.ztf.fink-portal.org/api/v1/objects";
pub const FINK_ZTF_LATESTS: &str = "https://api.ztf.fink-portal.org/api/v1/latests";
pub const FINK_ZTF_SCHEMA: &str = "https://api.ztf.fink-portal.org/api/v1/schema";

pub const ALERCE_OBJECTS: &str = "https://api.alerce.online/alerts/v1/objects";

pub struct FinkAlert {
    pub object_id: Option<String>,
    pub ra_deg: Option<f64>,
    pub dec_deg: Option<f64>,
    pub jd: Option<f64>,
    pub magpsf: Option<f64>,
    pub sigmapsf: Option<f64>,
    pub fid: Option<i64>,
    pub candid: Option<i64>,
    pub classification: Option<String>,
    pub separation_arcsec: Option<f64>,
    pub rf_snia_vs_nonia: Option<f64>,
    pub rf_kn_vs_nonkn: Option<f64>,
    pub roid: Option<i64>,
    pub snn_sn_vs_all: Option<f64>,
    pub snn_snia_vs_nonia: Option<f64>,
}

pub struct AlerceObject {
    pub oid: Option<String>,
    pub meanra: Option<f64>,
    pub meandec: Option<f64>,
    pub sigmara: Option<f64>,
    pub sigmadec: Option<f64>,
    pub firstmjd: Option<f64>,
    pub lastmjd: Option<f64>,
    pub ndethist: Option<i64>,
    pub stellar: Option<bool>,
    pub class: Option<String>,
    pub probability: Option<f64>,
}

fn obj_str(m: &HashMap<String, omegaflow::json::JsonVal>, key: &str) -> Option<String> {
    match m.get(key) {
        Some(omegaflow::json::JsonVal::Str(s)) if !s.is_empty() => Some(s.clone()),
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

fn num_any(m: &HashMap<String, omegaflow::json::JsonVal>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(omegaflow::json::JsonVal::Num(n)) if n.is_finite() => Some(*n),
        Some(omegaflow::json::JsonVal::Str(s)) => {
            let v: f64 = s.trim().parse().ok()?;
            if v.is_finite() { Some(v) } else { None }
        }
        _ => None,
    }
}

fn obj_bool(m: &HashMap<String, omegaflow::json::JsonVal>, key: &str) -> Option<bool> {
    match m.get(key) {
        Some(omegaflow::json::JsonVal::Bool(b)) => Some(*b),
        _ => None,
    }
}

pub fn curl_get(url: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-A")
        .arg(UA)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

pub fn fink_cone_body(
    ra: f64,
    dec: f64,
    radius_arcsec: f64,
    columns: Option<&str>,
) -> Option<(String, Vec<u8>)> {
    let mut url =
        format!("{FINK_ZTF_CONESEARCH}?ra={ra:.6}&dec={dec:.6}&radius={radius_arcsec:.3}");
    if let Some(c) = columns {
        url.push_str("&columns=");
        url.push_str(c);
    }
    curl_get(&url)
}

pub fn alerce_cone_body(ra: f64, dec: f64, radius_arcsec: f64) -> Option<(String, Vec<u8>)> {
    let url = format!("{ALERCE_OBJECTS}?ra={ra:.6}&dec={dec:.6}&radius={radius_arcsec:.3}");
    curl_get(&url)
}

fn row_maps(body: &[u8]) -> Option<Vec<HashMap<String, omegaflow::json::JsonVal>>> {
    let text = std::str::from_utf8(body).ok()?;
    let root = omegaflow::json::parse_json(text)?;
    let omegaflow::json::JsonVal::Arr(rows) = root else {
        return None;
    };
    let mut out = Vec::new();
    for r in rows {
        match r {
            omegaflow::json::JsonVal::Obj(m) => out.push(m),
            _ => return None,
        }
    }
    Some(out)
}

pub fn parse_fink_cone(body: &[u8]) -> Option<Vec<FinkAlert>> {
    row_maps(body).map(|rows| {
        rows.iter()
            .map(|m| FinkAlert {
                object_id: obj_str(m, "i:objectId"),
                ra_deg: obj_f64(m, "i:ra"),
                dec_deg: obj_f64(m, "i:dec"),
                jd: obj_f64(m, "i:jd"),
                magpsf: obj_f64(m, "i:magpsf"),
                sigmapsf: obj_f64(m, "i:sigmapsf"),
                fid: obj_i64(m, "i:fid"),
                candid: obj_i64(m, "i:candid"),
                classification: obj_str(m, "v:classification")
                    .or_else(|| obj_str(m, "d:classification")),
                separation_arcsec: obj_f64(m, "v:separation_degree").map(|d| d * 3600.0),
                rf_snia_vs_nonia: obj_f64(m, "d:rf_snia_vs_nonia"),
                rf_kn_vs_nonkn: obj_f64(m, "d:rf_kn_vs_nonkn"),
                roid: obj_i64(m, "d:roid"),
                snn_sn_vs_all: obj_f64(m, "d:snn_sn_vs_all"),
                snn_snia_vs_nonia: obj_f64(m, "d:snn_snia_vs_nonia"),
            })
            .collect()
    })
}

pub fn parse_alerce_objects(body: &[u8]) -> Option<Vec<AlerceObject>> {
    let text = std::str::from_utf8(body).ok()?;
    let root = omegaflow::json::parse_json(text)?;
    let omegaflow::json::JsonVal::Obj(map) = root else {
        return None;
    };
    let items = match map.get("items") {
        Some(omegaflow::json::JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let mut out = Vec::new();
    for r in items {
        match r {
            omegaflow::json::JsonVal::Obj(m) => out.push(AlerceObject {
                oid: obj_str(m, "oid"),
                meanra: obj_f64(m, "meanra"),
                meandec: obj_f64(m, "meandec"),
                sigmara: obj_f64(m, "sigmara"),
                sigmadec: obj_f64(m, "sigmadec"),
                firstmjd: obj_f64(m, "firstmjd"),
                lastmjd: obj_f64(m, "lastmjd"),
                ndethist: num_any(m, "ndethist").and_then(|v| {
                    if v.fract() == 0.0 {
                        Some(v as i64)
                    } else {
                        None
                    }
                }),
                stellar: obj_bool(m, "stellar"),
                class: obj_str(m, "class"),
                probability: obj_f64(m, "probability"),
            }),
            _ => return None,
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FINK_CONE_BODY: &[u8] = br#"[{"d:cdsxmatch":"Fail 503","d:classification":"Unknown","d:nalerthist":35,"i:candid":3432436160215015018,"i:dec":-13.3842553,"i:distnr":8.828539,"i:drb":0.99997747,"i:fid":1,"i:jd":2461186.9361689999,"i:jdstarthist":2461171.9541667001,"i:magpsf":18.429005,"i:objectId":"ZTF26aawdxet","i:ra":304.3485634,"i:sigmapsf":0.073919326,"v:separation_degree":0.0000280975}]"#;

    #[test]
    fn fink_cone_parses_the_measured_default_columns() {
        let rows = parse_fink_cone(FINK_CONE_BODY).unwrap();
        assert_eq!(rows.len(), 1);
        let r = &rows[0];
        assert_eq!(r.object_id.as_deref(), Some("ZTF26aawdxet"));
        assert!((r.ra_deg.unwrap() - 304.3485634).abs() < 1e-9);
        assert!((r.dec_deg.unwrap() + 13.3842553).abs() < 1e-9);
        assert_eq!(r.fid, Some(1));
        assert!((r.magpsf.unwrap() - 18.429005).abs() < 1e-6);
        assert_eq!(r.classification.as_deref(), Some("Unknown"));
        let sep = r.separation_arcsec.unwrap();
        assert!((sep - 0.0000280975 * 3600.0).abs() < 1e-6);
        assert!(r.rf_kn_vs_nonkn.is_none());
    }

    #[test]
    fn fink_empty_cone_is_a_realized_empty_not_an_absence() {
        let rows = parse_fink_cone(b"[]").unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn fink_non_row_bodies_stay_unparsed() {
        assert!(parse_fink_cone(b"{}").is_none());
        assert!(parse_fink_cone(b"not json").is_none());
    }

    const ALERCE_OBJECT_BODY: &[u8] = br#"{"total": null, "page": null, "items": [{"oid": "ZTF26aawdxet", "ndethist": "35", "ncovhist": 2786, "corrected": false, "stellar": false, "ndet": 35, "firstmjd": 61171.45416670013, "lastmjd": 61186.43616899988, "meanra": 304.34856182411835, "meandec": -13.384288319775203, "sigmara": 0.005439814769073407, "sigmadec": 0.005292065975420122, "class": null, "probability": null}]}"#;

    #[test]
    fn alerce_objects_parses_the_measured_items_schema() {
        let rows = parse_alerce_objects(ALERCE_OBJECT_BODY).unwrap();
        assert_eq!(rows.len(), 1);
        let r = &rows[0];
        assert_eq!(r.oid.as_deref(), Some("ZTF26aawdxet"));
        assert!((r.meanra.unwrap() - 304.34856182411835).abs() < 1e-9);
        assert!((r.meandec.unwrap() + 13.384288319775203).abs() < 1e-9);
        assert_eq!(r.ndethist, Some(35));
        assert_eq!(r.stellar, Some(false));
        assert_eq!(r.class, None);
        assert_eq!(r.probability, None);
    }

    #[test]
    fn alerce_empty_items_is_a_realized_empty() {
        let rows = parse_alerce_objects(br#"{"items": []}"#).unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn alerce_non_object_bodies_stay_unparsed() {
        assert!(parse_alerce_objects(b"[]").is_none());
        assert!(parse_alerce_objects(b"{}").is_none());
    }
}
