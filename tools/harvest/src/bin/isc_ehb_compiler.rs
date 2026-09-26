use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;

const ROUTE: &str = "http://download.isc.ac.uk/isc-ehb";
const NETLOC: &str = "download.isc.ac.uk";
const DEFAULT_YEAR: &str = "1964";

const MAGIC: [u8; 4] = *b"EHB1";
const VERSION: u8 = 1;
const HEADER_LEN: usize = 13;
const REC_BYTES: usize = 80;
const STATION_LEN: usize = 8;
const PHASE_LEN: usize = 8;
const PRES_MAG: u8 = 0x01;
const PRES_PHASE: u8 = 0x02;

#[derive(Clone, Debug, PartialEq)]
struct EhbArrival {
    time: f64,
    lat: f64,
    lon: f64,
    depth_km: f64,
    magnitude: Option<f64>,
    station: Option<String>,
    phase: Option<String>,
    residual_s: Option<f64>,
    distance_deg: Option<f64>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn col_str(line: &[u8], start: usize, end: usize) -> Option<&str> {
    if line.len() < end {
        return None;
    }
    std::str::from_utf8(&line[start - 1..end]).ok()
}

fn col_f64(line: &[u8], start: usize, end: usize) -> Option<f64> {
    col_str(line, start, end)?
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
}

fn col_i64(line: &[u8], start: usize, end: usize) -> Option<i64> {
    col_str(line, start, end)?.trim().parse::<i64>().ok()
}

fn fixed_ascii(s: &str, out: &mut [u8], cap: usize) -> bool {
    let bytes = s.trim().as_bytes();
    if bytes.is_empty() || bytes.len() > cap {
        return false;
    }
    if !bytes.iter().all(|c| c.is_ascii_graphic() || *c == b' ') {
        return false;
    }
    out.fill(0);
    out[..bytes.len()].copy_from_slice(bytes);
    true
}

fn decode_ascii(b: &[u8]) -> Option<String> {
    let end = b.iter().position(|&c| c == 0).unwrap_or(b.len());
    let raw = &b[..end];
    if raw.is_empty() || !raw.iter().all(|c| c.is_ascii_graphic() || *c == b' ') {
        return None;
    }
    Some(String::from_utf8_lossy(raw).into_owned())
}

fn write_header(out: &mut Vec<u8>, n: u64) {
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&n.to_le_bytes());
}

fn parse_header(b: &[u8]) -> Option<u64> {
    if b.len() < HEADER_LEN || b[0..4] != MAGIC || b[4] != VERSION {
        return None;
    }
    Some(u64::from_le_bytes(b[5..13].try_into().ok()?))
}

fn encode_rec(out: &mut [u8; REC_BYTES], a: &EhbArrival) {
    out.fill(0);
    if a.magnitude.is_some() {
        out[0] |= PRES_MAG;
    }
    if a.phase.is_some() {
        out[0] |= PRES_PHASE;
    }
    out[8..16].copy_from_slice(&a.time.to_le_bytes());
    out[16..24].copy_from_slice(&a.lat.to_le_bytes());
    out[24..32].copy_from_slice(&a.lon.to_le_bytes());
    out[32..40].copy_from_slice(&a.depth_km.to_le_bytes());
    out[40..48].copy_from_slice(
        &match a.magnitude {
            Some(v) => v,
            None => 0.0,
        }
        .to_le_bytes(),
    );
    out[48..56].copy_from_slice(
        &match a.residual_s {
            Some(v) => v,
            None => 0.0,
        }
        .to_le_bytes(),
    );
    out[56..64].copy_from_slice(
        &match a.distance_deg {
            Some(v) => v,
            None => 0.0,
        }
        .to_le_bytes(),
    );
    if let Some(s) = &a.station {
        let mut buf = [0u8; STATION_LEN];
        if fixed_ascii(s, &mut buf, STATION_LEN) {
            out[64..64 + STATION_LEN].copy_from_slice(&buf);
        }
    }
    if let Some(p) = &a.phase {
        let mut buf = [0u8; PHASE_LEN];
        if fixed_ascii(p, &mut buf, PHASE_LEN) {
            out[72..72 + PHASE_LEN].copy_from_slice(&buf);
        }
    }
}

fn decode_rec(b: &[u8]) -> Option<EhbArrival> {
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
    let residual_s = if present & PRES_PHASE != 0 {
        let v = f64::from_le_bytes(buf[48..56].try_into().ok()?);
        if v.is_finite() {
            Some(v)
        } else {
            return None;
        }
    } else {
        None
    };
    let distance_deg = if present & PRES_PHASE != 0 {
        let v = f64::from_le_bytes(buf[56..64].try_into().ok()?);
        if v.is_finite() {
            Some(v)
        } else {
            return None;
        }
    } else {
        None
    };
    let station = if present & PRES_PHASE != 0 {
        match decode_ascii(&buf[64..64 + STATION_LEN]) {
            Some(s) => Some(s),
            None => return None,
        }
    } else {
        None
    };
    let phase = if present & PRES_PHASE != 0 {
        match decode_ascii(&buf[72..72 + PHASE_LEN]) {
            Some(p) => Some(p),
            None => return None,
        }
    } else {
        None
    };
    Some(EhbArrival {
        time,
        lat,
        lon,
        depth_km,
        magnitude,
        station,
        phase,
        residual_s,
        distance_deg,
    })
}

fn write_bin(events: &[EhbArrival]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + events.len() * REC_BYTES);
    write_header(&mut out, events.len() as u64);
    let mut rec = [0u8; REC_BYTES];
    for e in events {
        encode_rec(&mut rec, e);
        out.extend_from_slice(&rec);
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<EhbArrival>> {
    let n = parse_header(bytes)? as usize;
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

fn origin_unix(line: &[u8]) -> Option<f64> {
    let year = col_i64(line, 32, 36)?;
    let month = col_i64(line, 37, 39)?;
    let day = col_i64(line, 40, 42)?;
    let hour = col_i64(line, 45, 47)?;
    let minute = col_i64(line, 48, 50)?;
    let sec = col_f64(line, 51, 56)?;
    let days = omegaflow::lsk::days_from_civil(year, month, day)? as f64;
    Some(days * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + sec)
}

fn parse_res_line(line: &[u8]) -> Option<EhbArrival> {
    let time = origin_unix(line)?;
    let elat = col_f64(line, 57, 64)?;
    let elon = col_f64(line, 65, 72)?;
    let depth_km = col_f64(line, 73, 78)?;
    let lat = 90.0 - elat;
    let mut lon = elon;
    if lon > 180.0 {
        lon -= 360.0;
    }
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    let fmb = col_f64(line, 79, 82).filter(|v| *v > 0.0);
    let fms = col_f64(line, 83, 86).filter(|v| *v > 0.0);
    let magnitude = fmb.or(fms);
    let station = col_str(line, 103, 108)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned);
    let phase = col_str(line, 159, 166)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned);
    let residual_s = col_f64(line, 326, 332);
    let distance_deg = col_f64(line, 132, 139).filter(|v| *v > 0.0);
    Some(EhbArrival {
        time,
        lat,
        lon,
        depth_km,
        magnitude,
        station,
        phase,
        residual_s,
        distance_deg,
    })
}

fn parse_res(raw: &[u8]) -> Vec<EhbArrival> {
    let mut out = Vec::new();
    for line in raw.split(|&b| b == b'\n') {
        if line.len() < 332 {
            continue;
        }
        if let Some(a) = parse_res_line(line) {
            out.push(a);
        }
    }
    out
}

fn fetch_body(args: &[String]) -> Result<Vec<u8>, String> {
    if let Some(input) = arg_value(args, "--input") {
        return std::fs::read(&input).map_err(|e| format!("read {input} returned void: {e}"));
    }
    let url = match arg_value(args, "--url") {
        Some(u) => u,
        None => {
            let year = arg_value(args, "--year").unwrap_or(DEFAULT_YEAR.to_string());
            format!("{ROUTE}/{year}.res.gz")
        }
    };
    eprintln!("isc_ehb: fetch {url}");
    match fetch_raw_bytes(&url) {
        Some(b) if !b.is_empty() => Ok(b),
        Some(_) => Err(format!(
            "{url}: HTTP 200 with an empty body — no bulletin present"
        )),
        None => Err(format!(
            "{url}: the route carried no body — fetch returned void"
        )),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let out_path = arg_value(args, "--out");
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let dump = arg_value(args, "--dump").and_then(|v| v.parse::<usize>().ok());

    let gz = fetch_body(args)?;
    let raw = gunzip(&gz).ok_or_else(|| "gzip stream stays unreadable".to_string())?;

    if let Some(n) = dump {
        for (i, line) in raw.split(|&b| b == b'\n').take(n).enumerate() {
            eprintln!("{:05} |{}|", i + 1, String::from_utf8_lossy(line));
        }
        return Ok(());
    }

    let arrivals = parse_res(&raw);
    if arrivals.is_empty() {
        return Err(
            "no event parsed from the bulletin — the asset stays unwritten (0 honored)".into(),
        );
    }

    let census = census(&arrivals);
    eprintln!("isc_ehb: {} arrivals parsed", arrivals.len());
    eprintln!("census: {census}");

    let Some(out_path) = out_path else {
        eprintln!("isc_ehb: --out not named — no asset written");
        return Ok(());
    };

    let bytes = write_bin(&arrivals);
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
    if verified.len() != arrivals.len() {
        return Err(format!(
            "{out_path}: {} records roundtripped, {} expected",
            verified.len(),
            arrivals.len()
        ));
    }

    let last = verified
        .last()
        .ok_or_else(|| "no last record — the asset stays unwritten".to_string())?;
    eprintln!(
        "isc_ehb: {} arrivals, {} B -> {out_path}, roundtrip verified",
        arrivals.len(),
        bytes.len()
    );
    eprintln!(
        "last arrival: time {:.3} lat {:.4} lon {:.4} depth {:.3} km mag {} station {} phase {} residual {} distance {}",
        last.time,
        last.lat,
        last.lon,
        last.depth_km,
        match last.magnitude {
            Some(v) => format!("{v}"),
            None => "absent".to_string(),
        },
        match &last.station {
            Some(s) => s.clone(),
            None => "absent".to_string(),
        },
        match &last.phase {
            Some(p) => p.clone(),
            None => "absent".to_string(),
        },
        match last.residual_s {
            Some(v) => format!("{v}"),
            None => "absent".to_string(),
        },
        match last.distance_deg {
            Some(v) => format!("{v}"),
            None => "absent".to_string(),
        }
    );
    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn census(arrivals: &[EhbArrival]) -> String {
    let n = arrivals.len();
    let mut mag = 0usize;
    let mut phase = 0usize;
    let mut station = 0usize;
    let mut residual = 0usize;
    let mut distance = 0usize;
    let mut min_lat = f64::INFINITY;
    let mut max_lat = f64::NEG_INFINITY;
    let mut min_lon = f64::INFINITY;
    let mut max_lon = f64::NEG_INFINITY;
    for a in arrivals {
        if a.magnitude.is_some() {
            mag += 1;
        }
        if a.phase.is_some() {
            phase += 1;
        }
        if a.station.is_some() {
            station += 1;
        }
        if a.residual_s.is_some() {
            residual += 1;
        }
        if a.distance_deg.is_some() {
            distance += 1;
        }
        min_lat = min_lat.min(a.lat);
        max_lat = max_lat.max(a.lat);
        min_lon = min_lon.min(a.lon);
        max_lon = max_lon.max(a.lon);
    }
    format!(
        "{n} arrivals, {mag} magnitude, {phase} phase, {station} station, {residual} residual, {distance} distance; lat [{min_lat:.2}, {max_lat:.2}], lon [{min_lon:.2}, {max_lon:.2}]"
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: isc_ehb_compiler [--year YYYY] [--url <route>] [--input <file.gz>] [--dump <n>] [--out <path>] [--ci-mode]"
        );
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("isc_ehb_compiler: {msg}");
        std::process::exit(1);
    }
}
