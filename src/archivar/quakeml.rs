#[derive(Clone, Debug)]
pub struct QuakeMlEvent {
    pub id: String,
    pub time: f64,
    pub lat: f64,
    pub lon: f64,
    pub depth_km: f64,
    pub magnitude: Option<f64>,
    pub mag_type: Option<String>,
}

fn finite_num(s: &str) -> Option<f64> {
    let v: f64 = s.trim().parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn leaf_text<'a>(s: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = s.find(&open)? + open.len();
    let end = s[start..].find(&close)? + start;
    Some(s[start..end].trim())
}

fn is_open(rest: &str) -> bool {
    matches!(
        rest.as_bytes().first(),
        Some(b'>') | Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')
    )
}

fn attr_value(s: &str, name: &str) -> Option<String> {
    let gt = s.find('>')?;
    let open = &s[..gt];
    let key = format!("{}=\"", name);
    let start = open.find(&key)? + key.len();
    let end = open[start..].find('"')? + start;
    Some(open[start..end].to_string())
}

fn iso8601_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else if let Some((d, t)) = s.split_once(' ') {
        (d, t)
    } else {
        return None;
    };
    let mut dp = date.split('-');
    let year: i64 = dp.next()?.parse().ok()?;
    let month: u32 = dp.next()?.parse().ok()?;
    let day: u32 = dp.next()?.parse().ok()?;
    let time = time.trim_end_matches(|c| c == 'Z' || c == 'z');
    let mut tp = time.split(':');
    let hour: f64 = tp.next()?.parse().ok()?;
    let minute: f64 = tp.next().unwrap_or("0").parse().ok()?;
    let second: f64 = tp.next().unwrap_or("0").parse().ok()?;
    let days = super::ymd_to_days(year, month, day)? as f64;
    let unix = days * 86400.0 + hour * 3600.0 + minute * 60.0 + second;
    if unix.is_finite() { Some(unix) } else { None }
}

fn parse_origin(o: &str) -> Option<(f64, f64, f64, f64)> {
    let time = leaf_text(o, "time")
        .and_then(|b| leaf_text(b, "value"))
        .and_then(iso8601_unix)?;
    let lat = leaf_text(o, "latitude")
        .and_then(|b| leaf_text(b, "value"))
        .and_then(finite_num)?;
    let lon = leaf_text(o, "longitude")
        .and_then(|b| leaf_text(b, "value"))
        .and_then(finite_num)?;
    let depth_m = leaf_text(o, "depth")
        .and_then(|b| leaf_text(b, "value"))
        .and_then(finite_num)?;
    Some((time, lat, lon, depth_m / 1000.0))
}

fn parse_magnitude(m: &str) -> Option<(Option<f64>, Option<String>)> {
    let value = leaf_text(m, "mag")
        .and_then(|b| leaf_text(b, "value"))
        .and_then(finite_num);
    let ty = leaf_text(m, "type")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty());
    if value.is_some() || ty.is_some() {
        Some((value, ty))
    } else {
        None
    }
}

pub fn parse_quakeml(body: &str) -> Vec<QuakeMlEvent> {
    let mut events = Vec::new();
    for ev in body.split("<event").skip(1) {
        if !is_open(ev) {
            continue;
        }
        let id = match attr_value(ev, "publicID") {
            Some(v) => v,
            None => continue,
        };
        let preferred_origin = leaf_text(ev, "preferredOriginID")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty());
        let preferred_magnitude = leaf_text(ev, "preferredMagnitudeID")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty());

        let mut origins: Vec<(Option<String>, &str)> = Vec::new();
        for o in ev.split("<origin").skip(1) {
            if !is_open(o) {
                continue;
            }
            origins.push((attr_value(o, "publicID"), o));
        }
        let mut ordered: Vec<&(Option<String>, &str)> = origins.iter().collect();
        if let Some(pref) = &preferred_origin {
            if let Some(pos) = ordered
                .iter()
                .position(|(oid, _)| oid.as_deref() == Some(pref.as_str()))
            {
                let item = ordered.remove(pos);
                ordered.insert(0, item);
            }
        }
        let mut origin_data = None;
        for (_, o) in ordered {
            if let Some(d) = parse_origin(o) {
                origin_data = Some(d);
                break;
            }
        }
        let (time, lat, lon, depth_km) = match origin_data {
            Some(d) => d,
            None => continue,
        };

        let mut magnitudes: Vec<(Option<String>, &str)> = Vec::new();
        for m in ev.split("<magnitude").skip(1) {
            if !is_open(m) {
                continue;
            }
            magnitudes.push((attr_value(m, "publicID"), m));
        }
        let mut mordered: Vec<&(Option<String>, &str)> = magnitudes.iter().collect();
        if let Some(pref) = &preferred_magnitude {
            if let Some(pos) = mordered
                .iter()
                .position(|(mid, _)| mid.as_deref() == Some(pref.as_str()))
            {
                let item = mordered.remove(pos);
                mordered.insert(0, item);
            }
        }
        let mut magnitude_data: Option<(Option<f64>, Option<String>)> = None;
        for (_, m) in mordered {
            if let Some(d) = parse_magnitude(m) {
                magnitude_data = Some(d);
                break;
            }
        }
        let (magnitude, mag_type) = magnitude_data.unwrap_or((None, None));

        events.push(QuakeMlEvent {
            id,
            time,
            lat,
            lon,
            depth_km,
            magnitude,
            mag_type,
        });
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    const BULLETIN: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<q:quakeml xmlns:q="http://quakeml.org/xmlns/quakeml/1.2" xmlns="http://quakeml.org/xmlns/bed/1.2">
  <eventParameters publicID="smi:ISC/bulletin">
    <event publicID="smi:ISC/evid=636373818">
      <preferredOriginID>smi:ISC/origid=638932982</preferredOriginID>
      <preferredMagnitudeID>smi:ISC/magid=1</preferredMagnitudeID>
      <type>earthquake</type>
      <origin publicID="smi:ISC/origid=638932982">
        <time><value>2024-01-01T07:06:05.77Z</value></time>
        <latitude><value>37.4929</value></latitude>
        <longitude><value>137.2624</value></longitude>
        <depth><value>10742.6</value></depth>
      </origin>
      <origin publicID="smi:ISC/origid=637650172">
        <time><value>2024-01-01T07:05:57.20Z</value></time>
        <latitude><value>37.3900</value></latitude>
        <longitude><value>138.0300</value></longitude>
        <depth><value>6000.0</value></depth>
      </origin>
      <magnitude publicID="smi:ISC/magid=1">
        <mag><value>7.5</value></mag>
        <type>MW</type>
      </magnitude>
    </event>
    <event publicID="smi:ISC/evid=636373819">
      <origin publicID="smi:ISC/origid=638932984">
        <time><value>2024-01-01T09:10:11.00Z</value></time>
        <latitude><value>-20.0</value></latitude>
        <longitude><value>120.5</value></longitude>
        <depth><value>5000.0</value></depth>
      </origin>
    </event>
  </eventParameters>
</q:quakeml>"#;

    #[test]
    fn extracts_preferred_origin_and_magnitude() {
        let events = parse_quakeml(BULLETIN);
        assert_eq!(events.len(), 2);
        let e = &events[0];
        assert_eq!(e.id, "smi:ISC/evid=636373818");
        assert!((e.time - 1704092765.77).abs() < 1e-6);
        assert!((e.lat - 37.4929).abs() < 1e-9);
        assert!((e.lon - 137.2624).abs() < 1e-9);
        assert!((e.depth_km - 10.7426).abs() < 1e-9);
        assert_eq!(e.magnitude, Some(7.5));
        assert_eq!(e.mag_type.as_deref(), Some("MW"));
    }

    #[test]
    fn falls_back_to_first_origin_and_absent_magnitude() {
        let events = parse_quakeml(BULLETIN);
        let e = &events[1];
        assert_eq!(e.id, "smi:ISC/evid=636373819");
        assert!((e.time - 1704100211.0).abs() < 1e-6);
        assert!((e.lat - -20.0).abs() < 1e-9);
        assert!((e.lon - 120.5).abs() < 1e-9);
        assert!((e.depth_km - 5.0).abs() < 1e-9);
        assert_eq!(e.magnitude, None);
        assert_eq!(e.mag_type, None);
    }

    #[test]
    fn skips_event_parameters_container() {
        assert!(!BULLETIN.contains("<eventNoSuch"));
        let events = parse_quakeml(BULLETIN);
        assert!(events.iter().all(|e| e.id.starts_with("smi:ISC/evid=")));
    }
}
