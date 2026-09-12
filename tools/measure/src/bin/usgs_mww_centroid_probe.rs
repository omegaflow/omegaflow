use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::ndk;
use omegaflow::archivar::fetch_raw;
use omegaflow_measure::depthphase::{arc_deg, arg_value, CATALOG_URL};

const GCMT_NDK_URL: &str =
    "https://www.ldeo.columbia.edu/~gcmt/projects/CMT/catalog/jan76_dec25.ndk";
const QUERY_TTL_S: u64 = 3600;
const GCMT_TTL_S: u64 = 86400;
const KM_PER_DEG: f64 = 111.195;

struct NodalPlane {
    strike: Option<f64>,
    dip: Option<f64>,
    rake: Option<f64>,
}

struct MomentTensor {
    mrr: Option<f64>,
    mtt: Option<f64>,
    mpp: Option<f64>,
    mrt: Option<f64>,
    mrp: Option<f64>,
    mtp: Option<f64>,
    scalar_moment_nm: Option<f64>,
    double_couple: Option<f64>,
}

struct Centroid {
    lat: Option<f64>,
    lon: Option<f64>,
    depth_m: Option<f64>,
    time_unix: Option<f64>,
    time_iso: Option<String>,
}

struct MwwRecord {
    centroid: Centroid,
    magnitude: Option<f64>,
    magnitude_type: Option<String>,
    moment_tensor: MomentTensor,
    np1: NodalPlane,
    np2: NodalPlane,
}

fn boundary(c: u8) -> bool {
    c == b'>' || c == b'/' || c.is_ascii_whitespace()
}

fn element_slices<'a>(xml: &'a str, name: &str) -> Vec<&'a str> {
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

fn element_inner<'a>(xml: &'a str, name: &str) -> Option<&'a str> {
    let full = element_slices(xml, name).into_iter().next()?;
    let open_end = full.find('>')? + 1;
    let close = format!("</{name}>");
    let close_start = full.len() - close.len();
    Some(&full[open_end..close_start])
}

fn text_of<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
    let inner = element_inner(xml, tag)?;
    let t = inner.trim();
    if t.is_empty() {
        None
    } else {
        Some(t)
    }
}

fn child_value<'a>(parent: &'a str, tag: &str) -> Option<&'a str> {
    let inner = element_inner(parent, tag)?;
    text_of(inner, "value")
}

fn f64_of(s: &str) -> Option<f64> {
    let v = s.trim().parse::<f64>().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn child_f64(parent: &str, tag: &str) -> Option<f64> {
    child_value(parent, tag).and_then(f64_of)
}

fn attr_value<'a>(element: &'a str, attr: &str) -> Option<&'a str> {
    let needle = format!("{attr}=\"");
    let i = element.find(&needle)? + needle.len();
    let rest = &element[i..];
    let j = rest.find('"')?;
    Some(&rest[..j])
}

fn select_by_public_id<'a>(items: &[&'a str], preferred: Option<&str>) -> Option<&'a str> {
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

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = s.split_once('T').or_else(|| s.split_once(' '))?;
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let days = days_from_civil(y, m, d)? as f64;
    let t = time.split(|c: char| c == '.' || c == 'Z' || c == 'z').next()?;
    let mut tp = t.split(':');
    let hh: f64 = tp.next()?.parse().ok()?;
    let mm: f64 = tp.next().unwrap_or("0").parse().ok()?;
    let ss: f64 = tp.next().unwrap_or("0").parse().ok()?;
    Some(days * 86400.0 + hh * 3600.0 + mm * 60.0 + ss)
}

fn iso_ymd(s: &str) -> Option<(i32, u32, u32)> {
    let s = s.trim();
    let date = s.split_once('T').or_else(|| s.split_once(' '))?.0;
    let mut p = date.split('-');
    Some((p.next()?.parse().ok()?, p.next()?.parse().ok()?, p.next()?.parse().ok()?))
}

fn parse_quakeml(xml: &str) -> Option<MwwRecord> {
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
            fm_slices.iter().copied().find(|f| {
                attr_value(f, "catalog:dataid").is_some_and(|d| d.contains("mww"))
            })
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

fn human(v: Option<f64>, prec: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{:.*}", prec, x),
        _ => "absent".to_string(),
    }
}

fn json_num(v: Option<f64>, prec: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{:.*}", prec, x),
        _ => "null".to_string(),
    }
}

fn json_sci(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.6e}"),
        _ => "null".to_string(),
    }
}

fn json_str(v: Option<&str>) -> String {
    match v {
        Some(s) => format!("\"{s}\""),
        None => "null".to_string(),
    }
}

fn plane_json(p: &NodalPlane) -> String {
    format!(
        "{{\"strike\": {}, \"dip\": {}, \"rake\": {}}}",
        json_num(p.strike, 4),
        json_num(p.dip, 4),
        json_num(p.rake, 4)
    )
}

fn gcmt_crosscheck(record: &MwwRecord) -> Option<String> {
    let lat = record.centroid.lat?;
    let lon = record.centroid.lon?;
    let (y, m, d) = iso_ymd(record.centroid.time_iso.as_deref()?)?;
    let events = ndk::fetch_events(GCMT_NDK_URL, GCMT_TTL_S)?;
    let ev = events
        .iter()
        .filter(|e| e.year == y && e.month == m && (e.day as i64 - d as i64).abs() <= 1)
        .filter(|e| (e.hyp_lat - lat).abs() < 2.0 && (e.hyp_lon - lon).abs() < 2.0)
        .min_by(|a, b| {
            let da = (a.hyp_lat - lat).powi(2) + (a.hyp_lon - lon).powi(2);
            let db = (b.hyp_lat - lat).powi(2) + (b.hyp_lon - lon).powi(2);
            da.total_cmp(&db)
        })?;

    let scale = 10f64.powi(ev.exponent) * 1e-7;
    let gcmt = [ev.m_rr, ev.m_tt, ev.m_pp, ev.m_rt, ev.m_rp, ev.m_tp];
    let usgs = [
        record.moment_tensor.mrr,
        record.moment_tensor.mtt,
        record.moment_tensor.mpp,
        record.moment_tensor.mrt,
        record.moment_tensor.mrp,
        record.moment_tensor.mtp,
    ];
    let mut num = 0.0;
    let mut den = 0.0;
    let mut n = 0usize;
    let mut g_nm = [0.0f64; 6];
    for i in 0..6 {
        let gi = gcmt[i] * scale;
        g_nm[i] = gi;
        if let Some(ui) = usgs[i] {
            num += (ui - gi) * (ui - gi);
            den += ui * ui;
            n += 1;
        }
    }
    let rel = if n > 0 && den > 0.0 {
        Some((num / den).sqrt())
    } else {
        None
    };
    let g_m0_nm = ev.scalar_moment_dyne_cm() * 1e-7;
    let ratio = record
        .moment_tensor
        .scalar_moment_nm
        .filter(|m| *m > 0.0)
        .zip(if g_m0_nm > 0.0 { Some(g_m0_nm) } else { None })
        .map(|(a, b)| a / b);
    let comps = g_nm
        .iter()
        .map(|v| format!("{v:.6e}"))
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "{{\"name\": \"{}\", \"gcmt_components_nm\": [{}], \"gcmt_scalar_moment_nm\": {:.6e}, \"tensor_rms_relative\": {}, \"scalar_moment_ratio\": {}}}",
        ev.name,
        comps,
        g_m0_nm,
        rel.map(|v| format!("{v:.6}")).unwrap_or("null".into()),
        ratio.map(|v| format!("{v:.6}")).unwrap_or("null".into()),
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(eventid) = arg_value(&args, "--eventid") else {
        println!("usgs_mww_centroid_probe: --eventid <id> absent — the centroid record stays absent");
        return;
    };
    let url = format!("{CATALOG_URL}?eventid={eventid}&format=quakeml&magnitudetype=mww");
    let Some(xml) = fetch_raw(&url, None, &[], QUERY_TTL_S) else {
        println!("usgs_mww_centroid_probe: {url} carries no body — the centroid record stays absent");
        return;
    };
    let Some(record) = parse_quakeml(&xml) else {
        println!("usgs_mww_centroid_probe: {eventid} carries no moment tensor — the centroid record stays absent");
        return;
    };

    println!("=== USGS FDSN mww moment-tensor centroid (M9 source location) ===");
    println!("endpoint: {url}");
    println!("eventid: {eventid}");
    println!(
        "M9 source location (USGS mww centroid): lat {}, lon {}, depth_m {}, time {}",
        human(record.centroid.lat, 6),
        human(record.centroid.lon, 6),
        human(record.centroid.depth_m, 1),
        record.centroid.time_iso.as_deref().unwrap_or("absent"),
    );
    println!();

    println!("{{");
    println!("  \"probe\": \"usgs_mww_centroid\",");
    println!("  \"endpoint\": \"{url}\",");
    println!("  \"eventid\": \"{eventid}\",");
    println!("  \"m9_source_location\": \"USGS mww centroid\",");
    println!(
        "  \"centroid\": {{\"lat\": {}, \"lon\": {}, \"depth_m\": {}, \"time_unix\": {}, \"time_iso\": {}}},",
        json_num(record.centroid.lat, 6),
        json_num(record.centroid.lon, 6),
        json_num(record.centroid.depth_m, 1),
        json_num(record.centroid.time_unix, 3),
        json_str(record.centroid.time_iso.as_deref()),
    );
    println!(
        "  \"magnitude\": {{\"type\": {}, \"value\": {}}},",
        json_str(record.magnitude_type.as_deref()),
        json_num(record.magnitude, 3)
    );
    println!(
        "  \"moment_tensor\": {{\"mrr\": {}, \"mtt\": {}, \"mpp\": {}, \"mrt\": {}, \"mrp\": {}, \"mtp\": {}, \"scalar_moment_nm\": {}, \"double_couple\": {}}},",
        json_sci(record.moment_tensor.mrr),
        json_sci(record.moment_tensor.mtt),
        json_sci(record.moment_tensor.mpp),
        json_sci(record.moment_tensor.mrt),
        json_sci(record.moment_tensor.mrp),
        json_sci(record.moment_tensor.mtp),
        json_sci(record.moment_tensor.scalar_moment_nm),
        json_num(record.moment_tensor.double_couple, 4),
    );
    println!(
        "  \"nodal_planes\": {{\"np1\": {}, \"np2\": {}}},",
        plane_json(&record.np1),
        plane_json(&record.np2)
    );

    let picker_lat = arg_value(&args, "--picker-lat")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite());
    let picker_lon = arg_value(&args, "--picker-lon")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite());
    match (picker_lat, picker_lon, record.centroid.lat, record.centroid.lon) {
        (Some(plat), Some(plon), Some(clat), Some(clon)) => {
            let deg = arc_deg(clat, clon, plat, plon);
            println!(
                "  \"picker_offset\": {{\"picker_lat\": {plat}, \"picker_lon\": {plon}, \"arc_deg\": {deg:.6}, \"km\": {:.3}}},",
                deg * KM_PER_DEG
            );
        }
        (Some(_), Some(_), _, _) => {
            println!("  \"picker_offset\": \"pending (centroid lat/lon absent)\",");
        }
        _ => println!("  \"picker_offset\": null,"),
    }

    if args.iter().any(|a| a == "--gcmt") {
        match gcmt_crosscheck(&record) {
            Some(txt) => println!("  \"gcmt_crosscheck\": {txt}"),
            None => println!("  \"gcmt_crosscheck\": \"pending (no matching GCMT centroid)\""),
        }
    } else {
        println!("  \"gcmt_crosscheck\": \"pending (--gcmt)\"");
    }
    println!("}}");
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
