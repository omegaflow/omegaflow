use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::json::{JsonVal, jnum, jpath_val, jstr, parse_json};
use omegaflow::archivar::quake_event::{
    HEADER_LEN, PRES_DEPTH, PRES_LAT, PRES_LON, PRES_MAG, PRES_TIME, QuakeEvent, REC_BYTES,
    decode_rec, encode_rec, parse_header, write_header,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;

const CDN_TAG: &str = "quake-ptevent";

struct Feed {
    name: &'static str,
    url: &'static str,
}

const FEEDS: [Feed; 3] = [
    Feed {
        name: "chile",
        url: "https://services1.arcgis.com/ZGrptGlLV2IILABw/arcgis/rest/services/Semptember_Chile_Earthquake/FeatureServer/0/query?where=1%3D1&outFields=*&outSR=4326&f=geojson&resultRecordCount=2000",
    },
    Feed {
        name: "tohoku",
        url: "https://services7.arcgis.com/XnmMAZERCcHaTPxC/arcgis/rest/services/tohoku_seismicity/FeatureServer/0/query?where=1%3D1&outFields=*&outSR=4326&f=geojson&resultRecordCount=2000",
    },
    Feed {
        name: "jma",
        url: "https://www.jma.go.jp/bosai/quake/data/list.json",
    },
];

struct Item {
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

fn parse_cod(cod: &str) -> (Option<f64>, Option<f64>, Option<f64>) {
    let body = cod.strip_suffix('/').unwrap_or(cod);
    let bytes = body.as_bytes();
    let mut values: Vec<f64> = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'+' || bytes[i] == b'-' {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            let Ok(value) = body[start..i].parse::<f64>() else {
                return (None, None, None);
            };
            if !value.is_finite() {
                return (None, None, None);
            }
            values.push(value);
        } else {
            i += 1;
        }
    }
    match values.len() {
        2 => (Some(values[0]), Some(values[1]), None),
        n if n >= 3 => (
            Some(values[0]),
            Some(values[1]),
            Some(values[2].abs() / 1000.0),
        ),
        _ => (None, None, None),
    }
}

fn parse_geojson(text: &str, time_ms: bool) -> Vec<Item> {
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
        let unix = if time_ms {
            jnum(props, "time_").map(|ms| ms / 1000.0)
        } else {
            jstr(props, "time").and_then(|s| parse_iso8601(&s))
        };
        out.push(Item {
            lat: jnum(props, "latitude"),
            lon: jnum(props, "longitude"),
            mag: jnum(props, "mag"),
            depth_km: jnum(props, "depth"),
            unix,
        });
    }
    out
}

fn parse_jma(text: &str) -> Vec<Item> {
    let Some(JsonVal::Arr(entries)) = parse_json(text) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in &entries {
        let (lat, lon, depth_km) = match jstr(entry, "cod") {
            Some(cod) => parse_cod(&cod),
            None => (None, None, None),
        };
        out.push(Item {
            lat,
            lon,
            mag: jstr(entry, "mag").and_then(|s| s.trim().parse::<f64>().ok()),
            depth_km,
            unix: jstr(entry, "at").and_then(|s| parse_iso8601(&s)),
        });
    }
    out
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
        Some(dir) => dir,
        None => omegaflow::archivar::cache_root()
            .to_string_lossy()
            .into_owned(),
    };
    let mut written = 0usize;
    for feed in &FEEDS {
        let asset = format!("quake_ptevent_{}.bin", feed.name);
        let Some(bytes) = fetch_raw_bytes(feed.url) else {
            eprintln!(
                "{}: fetch void — the feed stays absent (0 honored)",
                feed.name
            );
            continue;
        };
        let Ok(text) = std::str::from_utf8(&bytes) else {
            eprintln!(
                "{}: response not utf8 — the feed stays absent (0 honored)",
                feed.name
            );
            continue;
        };
        let items = match feed.name {
            "jma" => parse_jma(text),
            "chile" => parse_geojson(text, true),
            _ => parse_geojson(text, false),
        };
        let events: Vec<QuakeEvent> = items.iter().filter_map(event_of).collect();
        if events.is_empty() {
            eprintln!(
                "{}: no point event survived — the feed stays absent (0 honored)",
                feed.name
            );
            continue;
        }
        let bin = pack(&events);
        if !roundtrip_holds(&bin, events.len()) {
            eprintln!(
                "{}: roundtrip void — the feed stays unverified (0 honored)",
                feed.name
            );
            continue;
        }
        let out_path = format!("{}/{}", out_dir.trim_end_matches('/'), asset);
        if let Some(parent) = std::path::Path::new(&out_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if std::fs::write(&out_path, &bin).is_err() {
            eprintln!("{}: write {} returned void", feed.name, out_path);
            continue;
        }
        eprintln!(
            "{}: {} event(s), {} byte(s), sha256 {}",
            out_path,
            events.len(),
            bin.len(),
            sha256_hex(&bin)
        );
        println!("url https://github.com/omegaflow/sources/releases/download/{CDN_TAG}/{asset}");
        println!("format quake_ptevent");
        println!("ttl 604800");
        println!();
        if ci_mode && !upload_release(CDN_TAG, &out_path) {
            eprintln!("{asset}: CDN upload returned void");
        }
        written += 1;
    }
    if written == 0 {
        eprintln!("no quake point event written (0 honored)");
        std::process::exit(1);
    }
}
