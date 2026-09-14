use omegaflow::archivar::fetch::fetch_raw;
use omegaflow::archivar::quakeml::{QuakeMlEvent, parse_quakeml};
use omegaflow::cdn::upload_release;

const ROUTE: &str = "https://www.isc.ac.uk/fdsnws/event/1/query";
const NETLOC: &str = "www.isc.ac.uk";

const MAGIC: [u8; 4] = *b"ISCB";
const VERSION: u8 = 1;
const HEADER_LEN: usize = 13;
const REC_BYTES: usize = 56;
const MAG_TYPE_LEN: usize = 8;
const PRES_MAG: u8 = 0x01;
const PRES_MAGTYPE: u8 = 0x02;
const FETCH_TTL: u64 = 3600;

#[derive(Clone, Debug, PartialEq)]
struct IscEvent {
    time: f64,
    lat: f64,
    lon: f64,
    depth_km: f64,
    magnitude: Option<f64>,
    mag_type: Option<String>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn build_url(start: &str, end: &str, minmag: Option<&str>) -> String {
    let mut url = format!("{ROUTE}?format=xml&starttime={start}&endtime={end}");
    if let Some(m) = minmag {
        url.push_str("&minmagnitude=");
        url.push_str(m);
    }
    url
}

fn encode_mag_type(out: &mut [u8; REC_BYTES], ty: &str) {
    let bytes = ty.as_bytes();
    let n = bytes.len().min(MAG_TYPE_LEN);
    out[48..48 + n].copy_from_slice(&bytes[..n]);
}

fn decode_mag_type(b: &[u8; REC_BYTES]) -> Option<String> {
    let field = &b[48..48 + MAG_TYPE_LEN];
    let end = field.iter().position(|&c| c == 0).unwrap_or(MAG_TYPE_LEN);
    let raw = &field[..end];
    if raw.is_empty() || !raw.iter().all(|c| c.is_ascii_graphic() || *c == b' ') {
        return None;
    }
    Some(String::from_utf8_lossy(raw).into_owned())
}

fn write_header(out: &mut Vec<u8>, n_events: u64) {
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&n_events.to_le_bytes());
}

fn parse_header(b: &[u8]) -> Option<u64> {
    if b.len() < HEADER_LEN || b[0..4] != MAGIC || b[4] != VERSION {
        return None;
    }
    Some(u64::from_le_bytes(b[5..13].try_into().ok()?))
}

fn encode_rec(out: &mut [u8; REC_BYTES], e: &IscEvent) {
    out.fill(0);
    if e.magnitude.is_some() {
        out[0] |= PRES_MAG;
    }
    if e.mag_type.is_some() {
        out[0] |= PRES_MAGTYPE;
    }
    out[8..16].copy_from_slice(&e.time.to_le_bytes());
    out[16..24].copy_from_slice(&e.lat.to_le_bytes());
    out[24..32].copy_from_slice(&e.lon.to_le_bytes());
    out[32..40].copy_from_slice(&e.depth_km.to_le_bytes());
    out[40..48].copy_from_slice(
        &match e.magnitude {
            Some(v) => v,
            None => 0.0,
        }
        .to_le_bytes(),
    );
    if let Some(ty) = &e.mag_type {
        encode_mag_type(out, ty);
    }
}

fn decode_rec(b: &[u8]) -> Option<IscEvent> {
    if b.len() != REC_BYTES {
        return None;
    }
    let mut buf = [0u8; REC_BYTES];
    buf.copy_from_slice(b);
    let time = f64::from_le_bytes(buf[8..16].try_into().ok()?);
    let lat = f64::from_le_bytes(buf[16..24].try_into().ok()?);
    let lon = f64::from_le_bytes(buf[24..32].try_into().ok()?);
    let depth_km = f64::from_le_bytes(buf[32..40].try_into().ok()?);
    if !(time.is_finite() && lat.is_finite() && lon.is_finite() && depth_km.is_finite()) {
        return None;
    }
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    let present = buf[0];
    let magnitude = if present & PRES_MAG != 0 {
        let v = f64::from_le_bytes(buf[40..48].try_into().ok()?);
        if v.is_finite() {
            Some(v)
        } else {
            return None;
        }
    } else {
        None
    };
    let mag_type = if present & PRES_MAGTYPE != 0 {
        match decode_mag_type(&buf) {
            Some(t) => Some(t),
            None => return None,
        }
    } else {
        None
    };
    Some(IscEvent {
        time,
        lat,
        lon,
        depth_km,
        magnitude,
        mag_type,
    })
}

fn write_bin(events: &[IscEvent]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + events.len() * REC_BYTES);
    write_header(&mut out, events.len() as u64);
    let mut rec = [0u8; REC_BYTES];
    for e in events {
        encode_rec(&mut rec, e);
        out.extend_from_slice(&rec);
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<IscEvent>> {
    let n = parse_header(bytes)?;
    let n = n as usize;
    if bytes.len() != HEADER_LEN + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let off = HEADER_LEN + i * REC_BYTES;
        out.push(decode_rec(&bytes[off..off + REC_BYTES])?);
    }
    Some(out)
}

fn to_event(q: &QuakeMlEvent) -> Option<IscEvent> {
    if !(-90.0..=90.0).contains(&q.lat) || !(-180.0..=180.0).contains(&q.lon) {
        return None;
    }
    let magnitude = match q.magnitude {
        Some(v) if v.is_finite() => Some(v),
        _ => None,
    };
    let mag_type = match &q.mag_type {
        Some(t) if !t.is_empty() && t.len() <= MAG_TYPE_LEN => Some(t.clone()),
        _ => None,
    };
    Some(IscEvent {
        time: q.time,
        lat: q.lat,
        lon: q.lon,
        depth_km: q.depth_km,
        magnitude,
        mag_type,
    })
}

fn run(args: &[String]) -> Result<(), String> {
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("usage: isc_bulletin_compiler --start <iso> --end <iso> [--minmag <m>] [--url <route>] [--input <quakeml.xml>] --out <isc_bulletin.iscb> [--ci-mode] — refused".into());
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let body = if let Some(input) = arg_value(args, "--input") {
        std::fs::read_to_string(&input).map_err(|e| format!("read {input} returned void: {e}"))?
    } else {
        let url = match arg_value(args, "--url") {
            Some(u) => u,
            None => {
                let start = arg_value(args, "--start")
                    .ok_or_else(|| "--start names the bulletin window — refused".to_string())?;
                let end = arg_value(args, "--end")
                    .ok_or_else(|| "--end names the bulletin window — refused".to_string())?;
                build_url(&start, &end, arg_value(args, "--minmag").as_deref())
            }
        };
        eprintln!("isc: fetch {url}");
        match fetch_raw(&url, None, &[], FETCH_TTL) {
            Some(b) if !b.trim().is_empty() => b,
            Some(_) => {
                return Err(format!(
                    "{url}: HTTP 200 with an empty body — no events present"
                ));
            }
            None => {
                return Err(format!(
                    "{url}: the route carried no body — fetch returned void"
                ));
            }
        }
    };

    let parsed = parse_quakeml(&body);
    if parsed.is_empty() {
        return Err(
            "no event parsed from the bulletin — the asset stays unwritten (0 honored)".into(),
        );
    }

    let mut events = Vec::with_capacity(parsed.len());
    let mut position_rejected = 0u64;
    let mut magnitude_absent = 0u64;
    let mut mag_type_absent = 0u64;
    for q in &parsed {
        match to_event(q) {
            Some(e) => {
                if e.magnitude.is_none() {
                    magnitude_absent += 1;
                }
                if e.mag_type.is_none() {
                    mag_type_absent += 1;
                }
                events.push(e);
            }
            None => position_rejected += 1,
        }
    }
    if events.is_empty() {
        return Err(
            "no plausible event in the bulletin — the asset stays unwritten (0 honored)".into(),
        );
    }

    let bytes = write_bin(&events);
    std::fs::write(&out_path, &bytes)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let actual = std::fs::metadata(&out_path)
        .map_err(|e| format!("stat {out_path} returned void: {e}"))?
        .len() as usize;
    if actual != bytes.len() {
        return Err(format!(
            "{out_path}: {actual} bytes written, {} expected — the asset stays unwritten",
            bytes.len()
        ));
    }

    let verified = parse_bin(&bytes).ok_or_else(|| format!("{out_path}: roundtrip parse void"))?;
    if verified.len() != events.len() {
        return Err(format!(
            "{out_path}: {} records roundtripped, {} expected",
            verified.len(),
            events.len()
        ));
    }

    let last = verified
        .last()
        .ok_or_else(|| "no last record — the asset stays unwritten".to_string())?;
    eprintln!(
        "isc: {} events from {} parsed, {} B -> {out_path}, roundtrip verified",
        events.len(),
        parsed.len(),
        bytes.len()
    );
    eprintln!(
        "last event: time {:.3} lat {:.4} lon {:.4} depth {:.3} km magnitude {} type {}",
        last.time,
        last.lat,
        last.lon,
        last.depth_km,
        match last.magnitude {
            Some(v) => format!("{v}"),
            None => "absent".to_string(),
        },
        match &last.mag_type {
            Some(t) => t.clone(),
            None => "absent".to_string(),
        }
    );
    eprintln!(
        "census: {position_rejected} position-rejected, {magnitude_absent} magnitude-absent, {mag_type_absent} mag-type-absent"
    );
    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("isc_bulletin_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BULLETIN: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<q:quakeml xmlns:q="http://quakeml.org/xmlns/quakeml/1.2" xmlns="http://quakeml.org/xmlns/bed/1.2">
  <eventParameters publicID="smi:ISC/bulletin">
    <event publicID="smi:ISC/evid=636373819">
      <preferredOriginID>smi:ISC/origid=638932984</preferredOriginID>
      <type>earthquake</type>
      <origin publicID="smi:ISC/origid=638932984">
        <time><value>2024-01-01T07:10:10.41Z</value></time>
        <latitude><value>37.4747</value></latitude>
        <longitude><value>137.3070</value></longitude>
        <depth><value>9704.0</value></depth>
      </origin>
      <magnitude publicID="smi:ISC/magid=648135071">
        <mag><value>6.0</value></mag>
        <type>mb</type>
      </magnitude>
    </event>
    <event publicID="smi:ISC/evid=636373818">
      <origin publicID="smi:ISC/origid=638932982">
        <time><value>2024-01-01T07:06:05.77Z</value></time>
        <latitude><value>37.4929</value></latitude>
        <longitude><value>137.2624</value></longitude>
        <depth><value>10742.6</value></depth>
      </origin>
    </event>
  </eventParameters>
</q:quakeml>"#;

    #[test]
    fn parses_real_bulletin_and_roundtrips() {
        let parsed = parse_quakeml(BULLETIN);
        assert_eq!(parsed.len(), 2);
        let events: Vec<IscEvent> = parsed.iter().filter_map(to_event).collect();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].magnitude, Some(6.0));
        assert_eq!(events[0].mag_type.as_deref(), Some("mb"));
        assert_eq!(events[1].magnitude, None);
        assert_eq!(events[1].mag_type, None);
        let bytes = write_bin(&events);
        assert_eq!(bytes.len(), HEADER_LEN + 2 * REC_BYTES);
        let back = parse_bin(&bytes).unwrap();
        assert_eq!(back, events);
    }

    #[test]
    fn header_roundtrip() {
        let mut b = Vec::new();
        write_header(&mut b, 7);
        assert_eq!(b.len(), HEADER_LEN);
        assert_eq!(parse_header(&b), Some(7));
        assert_eq!(parse_header(&b[..b.len() - 1]), None);
        let mut bad = b.clone();
        bad[0] = b'X';
        assert_eq!(parse_header(&bad), None);
    }

    #[test]
    fn decode_refuses_corruption() {
        let events = parse_quakeml(BULLETIN)
            .iter()
            .filter_map(to_event)
            .collect::<Vec<_>>();
        let bytes = write_bin(&events);
        assert!(parse_bin(&bytes).is_some());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut bad_version = bytes.clone();
        bad_version[4] = 9;
        assert!(parse_bin(&bad_version).is_none());
        let mut bad_lat = bytes.clone();
        let lat_off = HEADER_LEN + 16;
        bad_lat[lat_off..lat_off + 8].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_bin(&bad_lat).is_none());
    }

    #[test]
    fn mag_type_encodes_and_decodes_ascii() {
        let mut b = [0u8; REC_BYTES];
        encode_mag_type(&mut b, "MW");
        assert_eq!(&b[48..50], b"MW");
        assert_eq!(decode_mag_type(&b).as_deref(), Some("MW"));
    }

    #[test]
    fn build_url_carries_window_and_minmag() {
        let u = build_url("2024-01-01", "2024-01-03", Some("6"));
        assert_eq!(
            u,
            "https://www.isc.ac.uk/fdsnws/event/1/query?format=xml&starttime=2024-01-01&endtime=2024-01-03&minmagnitude=6"
        );
        let u2 = build_url("2024-01-01", "2024-01-03", None);
        assert!(!u2.contains("minmagnitude"));
    }
}
