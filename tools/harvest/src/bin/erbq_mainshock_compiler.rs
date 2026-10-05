use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::json::{JsonVal, jnum, jpath_val, jstr, parse_json};
use omegaflow::archivar::quake_event::{
    HEADER_LEN, PRES_DEPTH, PRES_LAT, PRES_LON, PRES_MAG, PRES_TIME, QuakeEvent, REC_BYTES,
    decode_rec, encode_rec, parse_header, write_header,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::lsk::days_from_civil;

const CDN_TAG: &str = "quake-ptevent";
const BASE_URL: &str = "https://services7.arcgis.com/XnmMAZERCcHaTPxC/arcgis/rest/services/tohoku_seismicity/FeatureServer/5/query";
const FULL_URL: &str = "https://services7.arcgis.com/XnmMAZERCcHaTPxC/arcgis/rest/services/tohoku_seismicity/FeatureServer/5/query?where=1%3D1&outFields=*&outSR=4326&f=geojson&resultRecordCount=2000";

struct Item {
    objectid: Option<f64>,
    lat: Option<f64>,
    lon: Option<f64>,
    mag: Option<f64>,
    depth_km: Option<f64>,
    unix: Option<f64>,
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_iso8601(s: &str) -> Option<f64> {
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let hour: i64 = s.get(11..13)?.parse().ok()?;
    let minute: i64 = s.get(14..16)?.parse().ok()?;
    let second: f64 = s.get(17..19)?.parse().ok()?;
    let mut rest = s.get(19..)?;
    if let Some(fraction) = rest.strip_prefix('.') {
        let end = fraction
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(fraction.len());
        rest = &fraction[end..];
    }
    let offset_secs = if rest.starts_with('Z') {
        0i64
    } else {
        let sign = match rest.as_bytes().first()? {
            b'+' => 1i64,
            b'-' => -1i64,
            _ => return None,
        };
        let hh: i64 = rest.get(1..3)?.parse().ok()?;
        let mm: i64 = rest.get(4..6)?.parse().ok()?;
        sign * (hh * 3600 + mm * 60)
    };
    if !(second.is_finite() && second >= 0.0) {
        return None;
    }
    let days = days_from_civil(year, month, day)?;
    let unix =
        days as f64 * 86400.0 + (hour * 3600 + minute * 60) as f64 + second - offset_secs as f64;
    if !unix.is_finite() {
        return None;
    }
    Some(unix)
}

fn parse_geojson(text: &str) -> Vec<Item> {
    let Some(root) = parse_json(text) else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(features)) = jpath_val(&root, "features") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for feature in features {
        let Some(props) = jpath_val(feature, "properties") else {
            continue;
        };
        out.push(Item {
            objectid: jnum(props, "OBJECTID"),
            lat: jnum(props, "latitude"),
            lon: jnum(props, "longitude"),
            mag: jnum(props, "mag"),
            depth_km: jnum(props, "depth"),
            unix: jstr(props, "time").and_then(|s| parse_iso8601(&s)),
        });
    }
    out
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0088_f64;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().asin()
}

fn decluster(items: &[Item]) -> Vec<usize> {
    let n = items.len();
    let complete: Vec<usize> = (0..n)
        .filter(|&i| {
            items[i].lat.is_some()
                && items[i].lon.is_some()
                && items[i].mag.is_some()
                && items[i].unix.is_some()
        })
        .collect();
    let mut removed = vec![false; n];
    for &i in &complete {
        let (lat_i, lon_i) = (items[i].lat.unwrap(), items[i].lon.unwrap());
        let t_i = items[i].unix.unwrap();
        let m_i = items[i].mag.unwrap();
        let l_km = 10f64.powf(0.1238 * m_i + 0.983);
        let t_days = 10f64.powf(0.032 * m_i + 2.738);
        for &j in &complete {
            if j == i || removed[j] || items[j].mag.unwrap() >= m_i {
                continue;
            }
            let dt_days = (items[j].unix.unwrap() - t_i).abs() / 86400.0;
            if dt_days <= t_days
                && haversine_km(lat_i, lon_i, items[j].lat.unwrap(), items[j].lon.unwrap()) <= l_km
            {
                removed[j] = true;
            }
        }
    }
    (0..n).filter(|&i| !removed[i]).collect()
}

fn event_of(item: &Item) -> Option<QuakeEvent> {
    let mut present = 0u8;
    let mut order = 0u8;
    let mut ipix = 0u32;
    let mut lat_deg = 0.0f32;
    let mut lon_deg = 0.0f32;
    let mut mag = 0.0f32;
    let mut depth_km = 0.0f32;
    let mut jd_utc = 0.0f64;
    if let (Some(lat), Some(lon)) = (item.lat, item.lon) {
        if let Some((pixel_order, pixel)) = QuakeEvent::pixel_of(lat, lon) {
            order = pixel_order;
            ipix = pixel;
            lat_deg = lat as f32;
            lon_deg = lon as f32;
            present |= PRES_LAT | PRES_LON;
        }
    }
    if let Some(m) = item.mag {
        if m.is_finite() {
            mag = m as f32;
            present |= PRES_MAG;
        }
    }
    if let Some(d) = item.depth_km {
        if d.is_finite() && d >= 0.0 {
            depth_km = d as f32;
            present |= PRES_DEPTH;
        }
    }
    if let Some(u) = item.unix {
        if let Some(jd) = QuakeEvent::jd_from_unix(u) {
            jd_utc = jd;
            present |= PRES_TIME;
        }
    }
    if present == 0 {
        return None;
    }
    Some(QuakeEvent {
        order,
        present,
        ipix,
        lat_deg,
        lon_deg,
        mag,
        depth_km,
        jd_utc,
    })
}

fn pack(events: &[QuakeEvent]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + events.len() * REC_BYTES);
    write_header(&mut out, events.len() as u64);
    for event in events {
        let mut rec = [0u8; REC_BYTES];
        encode_rec(&mut rec, event);
        out.extend_from_slice(&rec);
    }
    out
}

fn roundtrip_holds(bytes: &[u8], n: usize) -> bool {
    if parse_header(bytes) != Some(n as u64) {
        return false;
    }
    let Some(body) = bytes.get(HEADER_LEN..) else {
        return false;
    };
    if body.len() != n * REC_BYTES {
        return false;
    }
    body.chunks(REC_BYTES).all(|chunk| {
        let Ok(fixed) = <[u8; REC_BYTES]>::try_from(chunk) else {
            return false;
        };
        decode_rec(&fixed).is_some()
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_dir = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .to_string_lossy()
            .into_owned(),
    };

    let Some(bytes) = fetch_raw_bytes(FULL_URL) else {
        eprintln!("fetch void — the mainshock set stays absent (0 honored)");
        std::process::exit(1);
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("response not utf8 — the mainshock set stays absent (0 honored)");
        std::process::exit(1);
    };
    let items = parse_geojson(text);
    if items.is_empty() {
        eprintln!("no point event survived — the mainshock set stays absent (0 honored)");
        std::process::exit(1);
    }
    let kept = decluster(&items);
    let ids: Vec<String> = kept
        .iter()
        .filter_map(|&i| items[i].objectid.map(|v| format!("{}", v as i64)))
        .collect();
    let events: Vec<QuakeEvent> = kept.iter().filter_map(|&i| event_of(&items[i])).collect();
    if events.len() != kept.len() {
        eprintln!(
            "{} of {} kept events carried no wire record — the set stays unverified (0 honored)",
            kept.len() - events.len(),
            kept.len()
        );
        std::process::exit(1);
    }
    let bin = pack(&events);
    if !roundtrip_holds(&bin, events.len()) {
        eprintln!("roundtrip void — the set stays unverified (0 honored)");
        std::process::exit(1);
    }
    let asset = "quake_ptevent_mainshock.bin";
    let out_path = format!("{}/{}", out_dir.trim_end_matches('/'), asset);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    let where_url = format!(
        "{}?where=OBJECTID%20IN%20({})&outFields=*&outSR=4326&f=geojson&resultRecordCount=2000",
        BASE_URL,
        ids.join(",")
    );
    eprintln!(
        "{}: {} of {} event(s), {} byte(s), sha256 {}",
        out_path,
        events.len(),
        items.len(),
        bin.len(),
        sha256_hex(&bin)
    );
    println!("mainshock_set {} of {}", events.len(), items.len());
    println!("url https://github.com/omegaflow/sources/releases/download/{CDN_TAG}/{asset}");
    println!("format quake_ptevent");
    println!("ttl 604800");
    println!();
    println!("filter {where_url}");
    if ci_mode && !omegaflow::cdn::upload_release(CDN_TAG, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
    }
}
