use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BabamulAlert {
    pub ra: f64,
    pub dec: f64,
    pub magpsf: Option<f64>,
    pub magap: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BabamulParse {
    Alerts(Vec<BabamulAlert>),
    Empty,
    NotJson,
    NoData,
    Unplaced,
}

pub fn parse_alerts(body: &str) -> BabamulParse {
    let Some(json) = parse_json(body) else {
        return BabamulParse::NotJson;
    };
    let JsonVal::Obj(root) = &json else {
        return BabamulParse::NoData;
    };
    let Some(JsonVal::Arr(data)) = root.get("data") else {
        return BabamulParse::NoData;
    };
    if data.is_empty() {
        return BabamulParse::Empty;
    }
    let mut out = Vec::new();
    for item in data {
        let JsonVal::Obj(o) = item else { continue };
        let Some(cand) = o.get("candidate") else {
            continue;
        };
        let (Some(ra), Some(dec)) = (jnum(cand, "ra"), jnum(cand, "dec")) else {
            continue;
        };
        if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
            continue;
        }
        let magpsf = jnum(cand, "magpsf").filter(|v| v.is_finite());
        let magap = jnum(cand, "magap").filter(|v| v.is_finite());
        if magpsf.is_none() && magap.is_none() {
            continue;
        }
        out.push(BabamulAlert {
            ra,
            dec,
            magpsf,
            magap,
        });
    }
    if out.is_empty() {
        BabamulParse::Unplaced
    } else {
        BabamulParse::Alerts(out)
    }
}

pub fn to_skymap(alerts: &[BabamulAlert]) -> Vec<crate::skymap::SkymapRecord> {
    let mut out = Vec::new();
    for a in alerts {
        let Some(val) = a.magpsf.or(a.magap) else {
            continue;
        };
        let Some((order, ipix)) = crate::skymap::SkymapRecord::pixel_of(a.ra, a.dec) else {
            continue;
        };
        out.push(crate::skymap::SkymapRecord {
            order,
            kind: crate::skymap::KIND_GENERIC,
            ipix,
            ra_deg: a.ra as f32,
            dec_deg: a.dec as f32,
            value: val as f32,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_ALERTS: &str = r#"{"message":"found 2 alerts matching query","data":[
        {"objectId":"ZTF19aailsan","candidate":{"jd":2459000.7302083,"ra":219.9366568,"dec":16.5589908,"magpsf":15.894465446472168,"magap":16.23200035095215}},
        {"objectId":"ZTF19aailsao","candidate":{"jd":2459000.8,"ra":10.0,"dec":-5.0,"magpsf":null,"magap":18.5}}
    ]}"#;

    #[test]
    fn parse_alerts_carries_measured_candidates_and_drops_absent_magnitudes() {
        let BabamulParse::Alerts(rows) = parse_alerts(MEASURED_ALERTS) else {
            panic!("the measured alert JSON carries alerts");
        };
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].ra, 219.9366568);
        assert_eq!(rows[0].dec, 16.5589908);
        assert_eq!(rows[0].magpsf, Some(15.894465446472168));
        assert_eq!(rows[1].magpsf, None);
        assert_eq!(rows[1].magap, Some(18.5));
    }

    #[test]
    fn parse_alerts_names_the_truthful_empty_apart_from_the_parse_breaks() {
        assert_eq!(
            parse_alerts(r#"{"message":"found 0 alerts","data":[]}"#),
            BabamulParse::Empty
        );
        assert_eq!(parse_alerts(""), BabamulParse::NotJson);
        assert_eq!(
            parse_alerts(r#"{"message":"unauthorized"}"#),
            BabamulParse::NoData
        );
        assert_eq!(
            parse_alerts(r#"{"data":[{"candidate":{"ra":10.0,"dec":5.0}}]}"#),
            BabamulParse::Unplaced
        );
        assert_eq!(
            parse_alerts(r#"{"data":[{"candidate":{"ra":400.0,"dec":5.0,"magpsf":15.0}}]}"#),
            BabamulParse::Unplaced
        );
    }

    #[test]
    fn to_skymap_places_one_record_per_candidate() {
        let BabamulParse::Alerts(rows) = parse_alerts(MEASURED_ALERTS) else {
            panic!("the measured alert JSON carries alerts");
        };
        let skymap = to_skymap(&rows);
        assert_eq!(skymap.len(), 2);
        assert_eq!(skymap[0].ra_deg, 219.936_66);
        assert_eq!(skymap[0].value, 15.894_465);
        assert_eq!(skymap[0].kind, crate::skymap::KIND_GENERIC);
        assert_eq!(skymap[1].value, 18.5);
    }
}
