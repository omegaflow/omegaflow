use omegaflow::archivar::lsk::days_from_civil;

pub struct NodalPlane {
    pub strike: Option<f64>,
    pub dip: Option<f64>,
    pub rake: Option<f64>,
}

pub struct MomentTensor {
    pub mrr: Option<f64>,
    pub mtt: Option<f64>,
    pub mpp: Option<f64>,
    pub mrt: Option<f64>,
    pub mrp: Option<f64>,
    pub mtp: Option<f64>,
    pub scalar_moment_nm: Option<f64>,
    pub double_couple: Option<f64>,
}

pub struct Centroid {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub depth_m: Option<f64>,
    pub time_unix: Option<f64>,
    pub time_iso: Option<String>,
}

pub struct MwwRecord {
    pub centroid: Centroid,
    pub magnitude: Option<f64>,
    pub magnitude_type: Option<String>,
    pub moment_tensor: MomentTensor,
    pub np1: NodalPlane,
    pub np2: NodalPlane,
}

fn boundary(c: u8) -> bool {
    c == b'>' || c == b'/' || c.is_ascii_whitespace()
}

pub fn element_slices<'a>(xml: &'a str, name: &str) -> Vec<&'a str> {
    let needle = format!("<{name}");
    let close = format!("</{name}>");
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = xml[from..].find(&needle) {
        let start = from + rel;
        let at_boundary = xml
            .as_bytes()
            .get(start + needle.len())
            .is_some_and(|c| boundary(*c));
        if at_boundary {
            if let Some(gt_rel) = xml[start..].find('>') {
                let body_start = start + gt_rel + 1;
                if let Some(crel) = xml[body_start..].find(&close) {
                    let end = body_start + crel;
                    out.push(&xml[start..end + close.len()]);
                    from = end + close.len();
                    continue;
                }
            }
        }
        from = start + needle.len();
    }
    out
}

pub fn element_inner<'a>(xml: &'a str, name: &str) -> Option<&'a str> {
    let full = element_slices(xml, name).into_iter().next()?;
    let open_end = full.find('>')? + 1;
    let close = format!("</{name}>");
    let close_start = full.len() - close.len();
    Some(&full[open_end..close_start])
}

pub fn text_of<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
    let inner = element_inner(xml, tag)?;
    let t = inner.trim();
    if t.is_empty() { None } else { Some(t) }
}

pub fn child_value<'a>(parent: &'a str, tag: &str) -> Option<&'a str> {
    let inner = element_inner(parent, tag)?;
    text_of(inner, "value")
}

pub fn f64_of(s: &str) -> Option<f64> {
    let v = s.trim().parse::<f64>().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

pub fn child_f64(parent: &str, tag: &str) -> Option<f64> {
    child_value(parent, tag).and_then(f64_of)
}

pub fn attr_value<'a>(element: &'a str, attr: &str) -> Option<&'a str> {
    let needle = format!("{attr}=\"");
    let i = element.find(&needle)? + needle.len();
    let rest = &element[i..];
    let j = rest.find('"')?;
    Some(&rest[..j])
}

pub fn select_by_public_id<'a>(items: &[&'a str], preferred: Option<&str>) -> Option<&'a str> {
    if let Some(id) = preferred {
        if let Some(item) = items
            .iter()
            .copied()
            .find(|it| attr_value(it, "publicID") == Some(id))
        {
            return Some(item);
        }
    }
    items.first().copied()
}

pub fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = s.split_once('T').or_else(|| s.split_once(' '))?;
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let days = days_from_civil(y, m, d)? as f64;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: f64 = tp.next()?.parse().ok()?;
    let mm: f64 = tp.next().unwrap_or("0").parse().ok()?;
    let ss: f64 = tp.next().unwrap_or("0").parse().ok()?;
    Some(days * 86400.0 + hh * 3600.0 + mm * 60.0 + ss)
}

pub fn iso_ymd(s: &str) -> Option<(i32, u32, u32)> {
    let s = s.trim();
    let date = s.split_once('T').or_else(|| s.split_once(' '))?.0;
    let mut p = date.split('-');
    Some((
        p.next()?.parse().ok()?,
        p.next()?.parse().ok()?,
        p.next()?.parse().ok()?,
    ))
}

pub fn parse_quakeml(xml: &str) -> Option<MwwRecord> {
    let event = element_inner(xml, "event")?;

    let preferred_origin = text_of(event, "preferredOriginID");
    let origin_slices = element_slices(event, "origin");
    let origin = select_by_public_id(&origin_slices, preferred_origin)?;
    let time_iso = child_value(origin, "time").map(|s| s.to_string());
    let time_unix = time_iso.as_deref().and_then(iso_to_unix);
    let centroid = Centroid {
        lat: child_f64(origin, "latitude"),
        lon: child_f64(origin, "longitude"),
        depth_m: child_f64(origin, "depth"),
        time_unix,
        time_iso,
    };

    let preferred_mag = text_of(event, "preferredMagnitudeID");
    let mag_slices = element_slices(event, "magnitude");
    let mww = mag_slices
        .iter()
        .copied()
        .find(|m| text_of(m, "type") == Some("mww"));
    let magnitude = mww
        .or_else(|| select_by_public_id(&mag_slices, preferred_mag))
        .or_else(|| mag_slices.first().copied());
    let magnitude_type = magnitude
        .and_then(|m| text_of(m, "type"))
        .map(|s| s.to_string());
    let magnitude_value = magnitude.and_then(|m| child_f64(m, "mag"));

    let preferred_fm = text_of(event, "preferredFocalMechanismID");
    let fm_slices: Vec<&str> = element_slices(event, "focalMechanism")
        .into_iter()
        .filter(|f| element_inner(f, "momentTensor").is_some())
        .collect();
    let fm = select_by_public_id(&fm_slices, preferred_fm)
        .or_else(|| {
            fm_slices
                .iter()
                .copied()
                .find(|f| attr_value(f, "catalog:dataid").is_some_and(|d| d.contains("mww")))
        })
        .or_else(|| fm_slices.first().copied())?;

    let mt = element_inner(fm, "momentTensor")?;
    let tensor = element_inner(mt, "tensor");
    let moment_tensor = MomentTensor {
        mrr: tensor.and_then(|t| child_f64(t, "Mrr")),
        mtt: tensor.and_then(|t| child_f64(t, "Mtt")),
        mpp: tensor.and_then(|t| child_f64(t, "Mpp")),
        mrt: tensor.and_then(|t| child_f64(t, "Mrt")),
        mrp: tensor.and_then(|t| child_f64(t, "Mrp")),
        mtp: tensor.and_then(|t| child_f64(t, "Mtp")),
        scalar_moment_nm: child_f64(mt, "scalarMoment"),
        double_couple: text_of(mt, "doubleCouple").and_then(f64_of),
    };

    let nodal = element_inner(fm, "nodalPlanes");
    let np1 = nodal.and_then(|n| element_inner(n, "nodalPlane1"));
    let np2 = nodal.and_then(|n| element_inner(n, "nodalPlane2"));
    let plane = |p: Option<&str>| NodalPlane {
        strike: p.and_then(|x| child_f64(x, "strike")),
        dip: p.and_then(|x| child_f64(x, "dip")),
        rake: p.and_then(|x| child_f64(x, "rake")),
    };

    Some(MwwRecord {
        centroid,
        magnitude: magnitude_value,
        magnitude_type,
        moment_tensor,
        np1: plane(np1),
        np2: plane(np2),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EVENT_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<q:quakeml xmlns="http://quakeml.org/xmlns/bed/1.2" xmlns:catalog="http://anss.org/xmlns/catalog/0.1" xmlns:q="http://quakeml.org/xmlns/quakeml/1.2">
 <eventParameters publicID="quakeml:test">
  <event catalog:datasource="us" catalog:eventid="test" publicID="quakeml:test/event">
   <origin catalog:dataid="us_test" publicID="quakeml:test/origin">
    <time><value>2023-02-06T01:17:34.342Z</value></time>
    <longitude><value>37.0143</value></longitude>
    <latitude><value>37.2256</value></latitude>
    <depth><value>10000</value><uncertainty>1784</uncertainty></depth>
   </origin>
   <magnitude catalog:dataid="us_test" publicID="quakeml:test/origin#magnitude">
    <mag><value>7.8</value></mag>
    <type>mww</type>
   </magnitude>
   <focalMechanism catalog:dataid="us_test_mww" publicID="quakeml:test/mww">
    <nodalPlanes>
     <nodalPlane1><strike><value>317.63</value></strike><dip><value>88.71</value></dip><rake><value>-179.18</value></rake></nodalPlane1>
     <nodalPlane2><strike><value>227.61</value></strike><dip><value>89.18</value></dip><rake><value>-1.29</value></rake></nodalPlane2>
    </nodalPlanes>
    <momentTensor publicID="quakeml:test/mww#mt">
     <scalarMoment><value>5.39E+20</value></scalarMoment>
     <tensor>
      <Mrr><value>-5.375E+19</value></Mrr>
      <Mtt><value>-5.076E+20</value></Mtt>
      <Mpp><value>5.6135E+20</value></Mpp>
      <Mrt><value>1.203E+19</value></Mrt>
      <Mrp><value>2.97E+18</value></Mrp>
      <Mtp><value>4.901E+19</value></Mtp>
     </tensor>
     <doubleCouple>0.8103</doubleCouple>
    </momentTensor>
   </focalMechanism>
   <preferredOriginID>quakeml:test/origin</preferredOriginID>
   <preferredMagnitudeID>quakeml:test/origin#magnitude</preferredMagnitudeID>
   <preferredFocalMechanismID>quakeml:test/mww</preferredFocalMechanismID>
  </event>
 </eventParameters>
</q:quakeml>"#;

    const NO_TENSOR_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<q:quakeml xmlns="http://quakeml.org/xmlns/bed/1.2" xmlns:q="http://quakeml.org/xmlns/quakeml/1.2">
 <eventParameters>
  <event publicID="quakeml:test/event">
   <origin publicID="quakeml:test/origin">
    <time><value>2023-02-06T01:17:34.342Z</value></time>
    <longitude><value>37.0143</value></longitude>
    <latitude><value>37.2256</value></latitude>
    <depth><value>10000</value></depth>
   </origin>
   <magnitude publicID="quakeml:test/mag"><mag><value>7.8</value></mag><type>mww</type></magnitude>
  </event>
 </eventParameters>
</q:quakeml>"#;

    #[test]
    fn parses_the_mww_moment_tensor_centroid() {
        let r = parse_quakeml(EVENT_XML).expect("moment tensor present");
        assert!((r.centroid.lat.unwrap() - 37.2256).abs() < 1e-9);
        assert!((r.centroid.lon.unwrap() - 37.0143).abs() < 1e-9);
        assert!((r.centroid.depth_m.unwrap() - 10000.0).abs() < 1e-9);
        assert!((r.centroid.time_unix.unwrap() - 1675646254.0).abs() < 1.0);
        assert_eq!(r.magnitude_type.as_deref(), Some("mww"));
        assert!((r.magnitude.unwrap() - 7.8).abs() < 1e-9);
        let mt = &r.moment_tensor;
        assert!((mt.mrr.unwrap() - -5.375e19).abs() < 1e12);
        assert!((mt.mtt.unwrap() - -5.076e20).abs() < 1e12);
        assert!((mt.mpp.unwrap() - 5.6135e20).abs() < 1e12);
        assert!((mt.mrt.unwrap() - 1.203e19).abs() < 1e12);
        assert!((mt.mrp.unwrap() - 2.97e18).abs() < 1e12);
        assert!((mt.mtp.unwrap() - 4.901e19).abs() < 1e12);
        assert!((mt.scalar_moment_nm.unwrap() - 5.39e20).abs() < 1e12);
        assert!((mt.double_couple.unwrap() - 0.8103).abs() < 1e-9);
        assert!((r.np1.strike.unwrap() - 317.63).abs() < 1e-9);
        assert!((r.np1.dip.unwrap() - 88.71).abs() < 1e-9);
        assert!((r.np1.rake.unwrap() - -179.18).abs() < 1e-9);
        assert!((r.np2.strike.unwrap() - 227.61).abs() < 1e-9);
        assert!((r.np2.dip.unwrap() - 89.18).abs() < 1e-9);
        assert!((r.np2.rake.unwrap() - -1.29).abs() < 1e-9);
    }

    #[test]
    fn an_absent_moment_tensor_stays_absent() {
        assert!(parse_quakeml(NO_TENSOR_XML).is_none());
        assert!(parse_quakeml("").is_none());
        assert!(parse_quakeml("<q:quakeml></q:quakeml>").is_none());
    }
}
