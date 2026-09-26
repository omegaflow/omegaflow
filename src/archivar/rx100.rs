use super::{fetch_raw, jstr, parse_json};
use std::net::UdpSocket;
use std::time::Duration;

pub const MAGIC: [u8; 4] = *b"RX1V";

pub const COMP_LUMINANCE: u32 = 1;
pub const COMP_MAX: u32 = 1;

pub const K_REFLECTED_CDSM2: f64 = 12.5;
pub const TAU_S: f64 = 60.0;

pub const FREQ_PHOTOPIC_HZ: f64 = 5.45e14;
pub const BIN_WIDTH_PHOTOPIC_HZ: f64 = 3.2e14;

pub const F_NUMBER_MIN: f64 = 1.7;
pub const F_NUMBER_MAX: f64 = 12.0;
pub const EXPOSURE_MAX_S: f64 = 30.0;
pub const ISO_MIN: f64 = 64.0;
pub const ISO_MAX: f64 = 25600.0;
pub const LUMINANCE_MIN_CDM2: f64 = 1e-4;
pub const LUMINANCE_MAX_CDM2: f64 = 1e5;

pub fn luminance(n: f64, t: f64, s: f64) -> Option<f64> {
    if !(n.is_finite() && (F_NUMBER_MIN..=F_NUMBER_MAX).contains(&n)) {
        return None;
    }
    if !(t.is_finite() && t > 0.0 && t <= EXPOSURE_MAX_S) {
        return None;
    }
    if !(s.is_finite() && (ISO_MIN..=ISO_MAX).contains(&s)) {
        return None;
    }
    let lv = K_REFLECTED_CDSM2 * n * n / (t * s);
    if !(lv.is_finite() && (LUMINANCE_MIN_CDM2..=LUMINANCE_MAX_CDM2).contains(&lv)) {
        return None;
    }
    Some(lv)
}

pub fn parse_f_number(raw: &str) -> Option<f64> {
    let s = raw.trim();
    let s = s
        .strip_prefix('F')
        .or_else(|| s.strip_prefix('f'))
        .unwrap_or(s);
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0)
}

pub fn parse_shutter(raw: &str) -> Option<f64> {
    let s = raw.trim();
    if let Some((num, den)) = s.split_once('/') {
        let n: f64 = num.trim().parse().ok()?;
        let d: f64 = den.trim().parse().ok()?;
        if !d.is_finite() || d == 0.0 || !n.is_finite() || n <= 0.0 {
            return None;
        }
        return Some(n / d);
    }
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0)
}

pub fn parse_iso(raw: &str) -> Option<f64> {
    let s = raw.trim();
    let s = s
        .strip_prefix("ISO")
        .or_else(|| s.strip_prefix("iso"))
        .unwrap_or(s);
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0)
}

pub fn ssdp_location(body: &str) -> Option<String> {
    for line in body.lines() {
        if let Some(rest) = line
            .strip_prefix("LOCATION:")
            .or_else(|| line.strip_prefix("location:"))
        {
            return Some(rest.trim().to_string());
        }
    }
    None
}

pub fn ssdp_discover(timeout: Duration) -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.set_read_timeout(Some(timeout)).ok()?;
    let msg = "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: ssdp:all\r\n\r\n";
    socket
        .send_to(msg.as_bytes(), "239.255.255.250:1900")
        .ok()?;
    let mut buf = [0u8; 4096];
    loop {
        let (n, _src) = socket.recv_from(&mut buf).ok()?;
        let body = String::from_utf8_lossy(&buf[..n]);
        if let Some(loc) = ssdp_location(&body) {
            return Some(loc);
        }
    }
}

pub fn camera_service_url(xml_body: &str) -> Option<String> {
    let marker = "X_ScalarWebAPI_ActionList_URL>";
    let start = xml_body.find(marker)? + marker.len();
    let rest = &xml_body[start..];
    let end = rest.find('<')?;
    let url = rest[..end].trim();
    if url.is_empty() {
        return None;
    }
    Some(url.to_string())
}

pub fn sony_rpc(service_url: &str, method: &str, params: &str) -> Option<String> {
    let body =
        format!("{{\"method\":\"{method}\",\"params\":{params},\"id\":1,\"version\":\"1.0\"}}");
    let headers = [("Content-Type".to_string(), "application/json".to_string())];
    fetch_raw(service_url, Some(&body), &headers)
}

pub fn get_event_luminance(service_url: &str) -> Option<f64> {
    let body = sony_rpc(service_url, "getEvent", "[]")?;
    let j = parse_json(&body)?;
    let n = jstr(&j, "result.1.fNumber").and_then(|s| parse_f_number(&s))?;
    let t = jstr(&j, "result.1.shutterSpeed").and_then(|s| parse_shutter(&s))?;
    let s = jstr(&j, "result.1.isoSpeedRate").and_then(|s| parse_iso(&s))?;
    luminance(n, t, s)
}

pub fn scene_luminance(service_url: &str) -> Option<f64> {
    let _ = sony_rpc(service_url, "startRecMode", "[]");
    get_event_luminance(service_url)
}

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if !(COMP_LUMINANCE..=COMP_MAX).contains(&comp) {
            return None;
        }
        out.push((t, val, comp));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminance_holds_the_exposure_equation() {
        let lv = luminance(2.8, 1.0 / 60.0, 100.0).unwrap();
        let expected = 12.5 * 2.8 * 2.8 / ((1.0 / 60.0) * 100.0);
        assert!((lv - expected).abs() < 1e-9, "{lv} vs {expected}");
    }

    #[test]
    fn luminance_skips_absent_readbacks() {
        assert!(luminance(0.0, 1.0 / 60.0, 100.0).is_none());
        assert!(luminance(f64::NAN, 1.0 / 60.0, 100.0).is_none());
        assert!(luminance(2.8, 0.0, 100.0).is_none());
        assert!(luminance(2.8, 31.0, 100.0).is_none());
        assert!(luminance(2.8, 1.0 / 60.0, 32.0).is_none());
        assert!(luminance(1.0, 1.0 / 60.0, 100.0).is_none());
    }

    #[test]
    fn luminance_skips_saturated_scenes() {
        assert!(luminance(12.0, 1.0 / 32000.0, 64.0).is_none());
        assert!(luminance(1.7, 30.0, 25600.0).is_none());
    }

    #[test]
    fn f_number_strips_the_sony_prefix() {
        assert_eq!(parse_f_number("F2.8"), Some(2.8));
        assert_eq!(parse_f_number("f4.0"), Some(4.0));
        assert_eq!(parse_f_number("5.6"), Some(5.6));
        assert!(parse_f_number("").is_none());
        assert!(parse_f_number("F0").is_none());
    }

    #[test]
    fn shutter_parses_fractions_and_decimals() {
        let s = parse_shutter("1/60").unwrap();
        assert!((s - 1.0 / 60.0).abs() < 1e-12);
        assert_eq!(parse_shutter("0.5"), Some(0.5));
        assert_eq!(parse_shutter("30"), Some(30.0));
        assert!(parse_shutter("1/0").is_none());
        assert!(parse_shutter("").is_none());
    }

    #[test]
    fn iso_strips_the_sony_prefix() {
        assert_eq!(parse_iso("ISO100"), Some(100.0));
        assert_eq!(parse_iso("3200"), Some(3200.0));
        assert!(parse_iso("").is_none());
    }

    #[test]
    fn ssdp_location_reads_the_location_header() {
        let body =
            "HTTP/1.1 200 OK\r\nLOCATION: http://192.168.122.1:64321/DmsRmtDesc.xml\r\nUSN: x\r\n";
        assert_eq!(
            ssdp_location(body).as_deref(),
            Some("http://192.168.122.1:64321/DmsRmtDesc.xml")
        );
        assert_eq!(
            ssdp_location("location: http://10.0.0.1/dd.xml").as_deref(),
            Some("http://10.0.0.1/dd.xml")
        );
        assert!(ssdp_location("X-LOCATION: http://a/b").is_none());
    }

    #[test]
    fn camera_service_url_reads_the_action_list_url() {
        let xml = "<av:X_ScalarWebAPI_ServiceList><av:X_ScalarWebAPI_Service>\
<av:X_ScalarWebAPI_ServiceType>camera</av:X_ScalarWebAPI_ServiceType>\
<av:X_ScalarWebAPI_ActionList_URL>http://192.168.122.1:8080/sony/camera</av:X_ScalarWebAPI_ActionList_URL>\
</av:X_ScalarWebAPI_Service></av:X_ScalarWebAPI_ServiceList>";
        assert_eq!(
            camera_service_url(xml).as_deref(),
            Some("http://192.168.122.1:8080/sony/camera")
        );
        assert!(camera_service_url("<nothing/>").is_none());
    }

    #[test]
    fn bin_roundtrip() {
        let records = vec![(-220_000_000.0, 58.8, COMP_LUMINANCE)];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert!((parsed[0].1 - 58.8).abs() < 1e-9);
        assert_eq!(parsed[0].2, COMP_LUMINANCE);
    }

    #[test]
    fn bin_rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"RX1Vabc").is_none());
        let bad = write_bin(&[(1.0, 1.0, 4)]);
        assert!(parse_bin(&bad).is_none());
    }
}
