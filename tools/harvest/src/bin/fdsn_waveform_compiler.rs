use omegaflow::archivar::fetch::civil_date;
use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec, COMP_FDSN_BHZ, MAGIC_FDSN};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const ROUTE: &str = "https://service.earthscope.org/fdsnws/dataselect/1/query";
const STATION_ROUTE: &str = "https://service.earthscope.org/fdsnws/station/1/query";
const NETLOC: &str = "service.earthscope.org";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn be16(b: &[u8], i: usize) -> u16 {
    ((b[i] as u16) << 8) | b[i + 1] as u16
}

fn be_f32(b: &[u8], i: usize) -> f32 {
    f32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

fn field_str(h: &[u8], start: usize, len: usize) -> String {
    let mut s = String::new();
    for c in &h[start..start + len] {
        let ch = *c as char;
        if ch.is_ascii() && !ch.is_control() {
            s.push(ch);
        }
    }
    s.trim().to_string()
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = ((m + 9) % 12) as i64;
    let doy = (153 * mp + 2) / 5 + (d as i64) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn seed_unix(h: &[u8]) -> Option<f64> {
    let year = be16(h, 20) as i64;
    let doy = be16(h, 22) as u32;
    if !(1..=366).contains(&doy) {
        return None;
    }
    let hour = h[24] as f64;
    let minute = h[25] as f64;
    let second = h[26] as f64;
    let frac = be16(h, 28) as f64 / 10000.0;
    let day0 = days_from_civil(year, 1, 1) as f64;
    Some((day0 + doy as f64 - 1.0) * 86400.0 + hour * 3600.0 + minute * 60.0 + second + frac)
}

fn nominal_rate(fact: i16, mult: i16) -> Option<f64> {
    let r = if fact > 0 {
        fact as f64 * mult as f64
    } else if fact < 0 {
        -(mult as f64) / (fact as f64)
    } else {
        return None;
    };
    if r.is_finite() && r > 0.0 {
        Some(r)
    } else {
        None
    }
}

fn curl_bytes(url: &str) -> (Option<u16>, Vec<u8>) {
    let tmp = env::temp_dir().join(format!("fdsn_waveform_{}.mseed", std::process::id()));
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-g")
        .arg("-m")
        .arg("120")
        .arg("--connect-timeout")
        .arg("20")
        .arg("-o")
        .arg(&tmp)
        .arg("-w")
        .arg("%{http_code}")
        .arg(url)
        .output();
    let code = match out {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            s.trim().parse::<u16>().ok().filter(|c| *c > 0)
        }
        Err(_) => None,
    };
    let body = match fs::read(&tmp) {
        Ok(b) => b,
        Err(_) => Vec::new(),
    };
    let _ = fs::remove_file(&tmp);
    (code, body)
}

struct StationAnchor {
    lat: Option<f64>,
    lon: Option<f64>,
    elev: Option<f64>,
}

fn load_stations(path: &str) -> (HashMap<(String, String), StationAnchor>, usize, usize) {
    let mut map = HashMap::new();
    let mut rows = 0usize;
    let mut rejected = 0usize;
    let body = match fs::read_to_string(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("fdsnwf: station table {} stayed unread", path);
            return (map, 0, 0);
        }
    };
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = t.split('|').collect();
        if cols.len() < 4 {
            rejected += 1;
            continue;
        }
        rows += 1;
        let net = cols[0].trim().to_string();
        let sta = cols[1].trim().to_string();
        let lat = parse_finite(cols[2]);
        let lon = parse_finite(cols[3]);
        let elev = cols.get(4).and_then(|e| parse_finite(e));
        map.insert((net, sta), StationAnchor { lat, lon, elev });
    }
    (map, rows, rejected)
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() || t == "--" {
        return None;
    }
    let b = t.as_bytes();
    if b.len() < 10 {
        return None;
    }
    let n = |i: usize, j: usize| t.get(i..j)?.parse::<i64>().ok();
    let (y, mo, d) = (n(0, 4)?, n(5, 7)?, n(8, 10)?);
    let (h, mi, se) = if b.len() >= 19 {
        (n(11, 13)?, n(14, 16)?, n(17, 19)?)
    } else {
        (0, 0, 0)
    };
    let day = days_from_civil(y, mo as u32, d as u32) as f64;
    Some(day * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + se as f64)
}

#[derive(Clone, Debug, PartialEq)]
struct ResponseRow {
    start: f64,
    end: f64,
    scale: f64,
    units: String,
    elev: Option<f64>,
}

#[derive(Clone)]
struct Response {
    rows: Vec<ResponseRow>,
}

impl Response {
    fn row_at(&self, t: f64) -> Option<&ResponseRow> {
        self.rows.iter().find(|r| t >= r.start && t < r.end)
    }
}

fn parse_response_text(body: &str) -> Response {
    let mut header: Vec<String> = Vec::new();
    let mut data = String::new();
    for line in body.lines() {
        if header.is_empty() && line.trim_start().starts_with('#') {
            header = line[1..].split('|').map(|c| c.trim().to_string()).collect();
        } else if !header.is_empty() {
            data.push_str(line);
            data.push('\n');
        }
    }
    let col = |name: &str| header.iter().position(|c| c == name);
    let c_scale = col("Scale")
        .or_else(|| col("Sensitivity"))
        .or_else(|| col("InstrumentSensitivity"));
    let c_units = col("ScaleUnits");
    let c_elev = col("Elevation");
    let c_start = col("StartTime");
    let c_end = col("EndTime");
    let mut rows = Vec::new();
    for line in data.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let fields: Vec<&str> = t.split('|').map(|c| c.trim()).collect();
        let Some(si) = c_scale.and_then(|i| fields.get(i).and_then(|v| parse_finite(v))) else {
            continue;
        };
        if !(si.is_finite() && si > 0.0) {
            continue;
        }
        let start = c_start
            .and_then(|i| fields.get(i))
            .and_then(|v| iso_to_unix(v))
            .unwrap_or(f64::NEG_INFINITY);
        let end = c_end
            .and_then(|i| fields.get(i))
            .and_then(|v| iso_to_unix(v))
            .unwrap_or(f64::INFINITY);
        let units = match c_units.and_then(|i| fields.get(i)) {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        let elev = c_elev
            .and_then(|i| fields.get(i))
            .and_then(|v| parse_finite(v));
        rows.push(ResponseRow {
            start,
            end,
            scale: si,
            units,
            elev,
        });
    }
    rows.sort_by(|a, b| a.start.total_cmp(&b.start));
    Response { rows }
}

fn station_response_url(net: &str, sta: &str, cha: &str, loc: &str) -> String {
    let mut url = format!(
        "{}?network={}&station={}&channel={}&level=channel&format=text",
        STATION_ROUTE, net, sta, cha
    );
    if !loc.is_empty() {
        url.push_str("&location=");
        url.push_str(loc);
    }
    url
}

fn load_response(
    cache: &mut HashMap<(String, String, String), Response>,
    net: &str,
    sta: &str,
    cha: &str,
    loc: &str,
) -> Response {
    let key = (net.to_string(), sta.to_string(), cha.to_string());
    if !cache.contains_key(&key) {
        let url = station_response_url(net, sta, cha, loc);
        let (code, body) = curl_bytes(&url);
        eprintln!(
            "fdsnwf: station channel {}·{}·{} HTTP {}",
            net,
            sta,
            cha,
            match code {
                Some(c) => c.to_string(),
                None => "no-verdict".to_string(),
            }
        );
        let resp = if code == Some(200) {
            let text = String::from_utf8_lossy(&body);
            let r = parse_response_text(&text);
            if r.rows.is_empty() {
                eprintln!(
                    "fdsnwf: station channel {}·{}·{} carries no sensitivity rows — samples stay counts",
                    net, sta, cha
                );
            }
            r
        } else {
            eprintln!(
                "fdsnwf: station channel {}·{}·{} response fetch void — samples stay counts",
                net, sta, cha
            );
            Response { rows: Vec::new() }
        };
        cache.insert(key.clone(), resp);
    }
    match cache.get(&key) {
        Some(r) => r.clone(),
        None => Response { rows: Vec::new() },
    }
}

fn parse_finite(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn sign_extend(v: u32, bits: u32) -> i64 {
    let shift = 32 - bits;
    ((v << shift) as i32 as i64) >> shift
}

fn steim_decode(words: &[u32], encoding: u8, nsamp: usize) -> Vec<i64> {
    let frames = words.len() / 16;
    let mut out: Vec<i64> = Vec::new();
    let mut acc: i64 = 0;
    let mut produced = 0usize;
    for fi in 0..frames {
        if produced >= nsamp {
            break;
        }
        let base = fi * 16;
        let ctrl = words[base];
        let start: usize = if fi == 0 {
            let x0 = words[base + 1] as i32 as i64;
            out.push(x0);
            acc = x0;
            produced = 1;
            3
        } else {
            1
        };
        let mut diffs: Vec<i64> = Vec::new();
        for widx in start..16 {
            let w = words[base + widx];
            let nib = (ctrl >> (30 - 2 * widx)) & 3;
            match nib {
                0 => {}
                1 => {
                    for j in 0..4 {
                        diffs.push(((w >> (24 - 8 * j)) & 0xFF) as i8 as i64);
                    }
                }
                2 => {
                    if encoding == 10 {
                        diffs.push(sign_extend((w >> 16) & 0xFFFF, 16));
                        diffs.push(sign_extend(w & 0xFFFF, 16));
                    } else {
                        match (w >> 30) & 3 {
                            0 => return out,
                            1 => diffs.push(sign_extend(w & 0x3FFF_FFFF, 30)),
                            2 => {
                                diffs.push(sign_extend((w >> 15) & 0x7FFF, 15));
                                diffs.push(sign_extend(w & 0x7FFF, 15));
                            }
                            3 => {
                                diffs.push(sign_extend((w >> 20) & 0x3FF, 10));
                                diffs.push(sign_extend((w >> 10) & 0x3FF, 10));
                                diffs.push(sign_extend(w & 0x3FF, 10));
                            }
                            _ => return out,
                        }
                    }
                }
                3 => {
                    if encoding == 10 {
                        diffs.push(w as i32 as i64);
                    } else {
                        match (w >> 30) & 3 {
                            0 => {
                                for j in 0..5 {
                                    diffs.push(sign_extend((w >> (24 - 6 * j)) & 0x3F, 6));
                                }
                            }
                            1 => {
                                for j in 0..6 {
                                    diffs.push(sign_extend((w >> (25 - 5 * j)) & 0x1F, 5));
                                }
                            }
                            2 => {
                                for j in 0..7 {
                                    diffs.push(sign_extend((w >> (24 - 4 * j)) & 0xF, 4));
                                }
                            }
                            _ => return out,
                        }
                    }
                }
                _ => {}
            }
        }
        let skip = if fi == 0 { 1 } else { 0 };
        for d in diffs.iter().skip(skip) {
            if produced >= nsamp {
                break;
            }
            acc += d;
            out.push(acc);
            produced += 1;
        }
    }
    out.truncate(nsamp);
    out
}

fn encoding_name(e: u8) -> &'static str {
    match e {
        1 => "1/int16",
        2 => "2/int24",
        3 => "3/int32",
        4 => "4/ieee-f32",
        5 => "5/ieee-f64",
        10 => "10/steim1",
        11 => "11/steim2",
        _ => "unhandled",
    }
}

fn encoding_supported(e: u8) -> bool {
    matches!(e, 1 | 2 | 3 | 4 | 5 | 10 | 11)
}

struct RecordMeta {
    encoding: u8,
    big: bool,
    reclen: usize,
    data_offset: usize,
    b100_rate: Option<f64>,
}

fn record_meta(rec: &[u8]) -> Option<RecordMeta> {
    if rec.len() < 48 {
        return None;
    }
    let data_offset = be16(rec, 44) as usize;
    let mut encoding: Option<u8> = None;
    let mut big = true;
    let mut exp: Option<usize> = None;
    let mut b100_rate: Option<f64> = None;
    let mut off = be16(rec, 46) as usize;
    let mut guard = 0usize;
    while off >= 48 && off + 4 <= data_offset && guard < 16 {
        let typ = be16(rec, off) as usize;
        let next = be16(rec, off + 2) as usize;
        if typ == 1000 {
            encoding = rec.get(off + 4).copied();
            big = rec.get(off + 5).copied() == Some(1);
            exp = rec.get(off + 6).map(|b| *b as usize);
        } else if typ == 100 && off + 8 <= data_offset {
            let r = be_f32(rec, off + 4) as f64;
            if r.is_finite() && r > 0.0 {
                b100_rate = Some(r);
            }
        }
        if next <= off {
            break;
        }
        off = next;
        guard += 1;
    }
    let (Some(encoding), Some(e)) = (encoding, exp) else {
        return None;
    };
    if e == 0 || e > 20 {
        return None;
    }
    Some(RecordMeta {
        encoding,
        big,
        reclen: 1 << e,
        data_offset,
        b100_rate,
    })
}

fn decode_record(meta: &RecordMeta, rec: &[u8], rate: f64) -> Vec<(f64, f64)> {
    if meta.data_offset >= meta.reclen {
        return Vec::new();
    }
    let data = &rec[meta.data_offset..meta.reclen.min(rec.len())];
    let nsamp = be16(rec, 30) as usize;
    let t0 = match seed_unix(rec) {
        Some(t) => t,
        None => return Vec::new(),
    };
    let samples = decode_encoding(meta.encoding, meta.big, data, nsamp);
    let mut out = Vec::with_capacity(samples.len());
    for (i, v) in samples.iter().enumerate() {
        out.push((t0 + i as f64 / rate, *v));
    }
    out
}

fn decode_encoding(enc: u8, big: bool, data: &[u8], nsamp: usize) -> Vec<f64> {
    let mut out: Vec<f64> = Vec::new();
    match enc {
        1 => {
            for i in 0..nsamp {
                let at = i * 2;
                if at + 2 > data.len() {
                    break;
                }
                let raw = if big {
                    i16::from_be_bytes([data[at], data[at + 1]])
                } else {
                    i16::from_le_bytes([data[at], data[at + 1]])
                };
                out.push(raw as f64);
            }
        }
        2 => {
            for i in 0..nsamp {
                let at = i * 3;
                if at + 3 > data.len() {
                    break;
                }
                let raw = if big {
                    (data[at] as u32) << 16 | (data[at + 1] as u32) << 8 | data[at + 2] as u32
                } else {
                    (data[at + 2] as u32) << 16 | (data[at + 1] as u32) << 8 | data[at] as u32
                };
                out.push(sign_extend(raw, 24) as f64);
            }
        }
        3 => {
            for i in 0..nsamp {
                let at = i * 4;
                if at + 4 > data.len() {
                    break;
                }
                let raw = if big {
                    i32::from_be_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
                } else {
                    i32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
                };
                out.push(raw as f64);
            }
        }
        4 => {
            for i in 0..nsamp {
                let at = i * 4;
                if at + 4 > data.len() {
                    break;
                }
                let f = if big {
                    f32::from_be_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
                } else {
                    f32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
                };
                let v = f as f64;
                if v.is_finite() {
                    out.push(v);
                }
            }
        }
        5 => {
            for i in 0..nsamp {
                let at = i * 8;
                if at + 8 > data.len() {
                    break;
                }
                let mut b = [0u8; 8];
                b.copy_from_slice(&data[at..at + 8]);
                let f = if big {
                    f64::from_be_bytes(b)
                } else {
                    f64::from_le_bytes(b)
                };
                if f.is_finite() {
                    out.push(f);
                }
            }
        }
        10 | 11 => {
            let mut words: Vec<u32> = Vec::new();
            let mut i = 0usize;
            while i + 4 <= data.len() {
                words.push(u32::from_be_bytes([
                    data[i],
                    data[i + 1],
                    data[i + 2],
                    data[i + 3],
                ]));
                i += 4;
            }
            for v in steim_decode(&words, enc, nsamp) {
                out.push(v as f64);
            }
        }
        _ => {}
    }
    out
}

fn curl_verdict(code: u16) -> &'static str {
    match code {
        200 => "open",
        204 => "no-records-in-window",
        401 | 403 => "auth-required",
        404 => "not-found",
        429 => "rate-limited",
        _ => "refused",
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let out_path = arg_value(&args, "--out");
    let stations_path = arg_value(&args, "--stations");
    let network = match arg_value(&args, "--network") {
        Some(v) => v,
        None => String::new(),
    };
    let station = match arg_value(&args, "--station") {
        Some(v) => v,
        None => String::new(),
    };
    let channel = match arg_value(&args, "--channel") {
        Some(v) => v,
        None => String::new(),
    };
    let location = match arg_value(&args, "--location") {
        Some(v) => v,
        None => String::new(),
    };
    let start = arg_value(&args, "--start");
    let end = arg_value(&args, "--end");
    let (Some(start), Some(end)) = (start, end) else {
        eprintln!("fdsnwf: --start and --end are the required window");
        std::process::exit(1);
    };
    if stations_path.is_none() {
        eprintln!("fdsnwf: --stations names the fdsn_station_compiler table");
        std::process::exit(1);
    }

    let mut url = format!(
        "{}?network={}&station={}&starttime={}&endtime={}&format=miniseed",
        ROUTE, network, station, start, end
    );
    if !channel.is_empty() {
        url.push_str("&channel=");
        url.push_str(&channel);
    }
    if !location.is_empty() {
        url.push_str("&location=");
        url.push_str(&location);
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs());
    match now {
        Some(s) => {
            let (y, m, d) = civil_date(s);
            eprintln!("fdsnwf: dataselect {} · {}-{:02}-{:02} UTC", url, y, m, d);
        }
        None => eprintln!(
            "fdsnwf: dataselect {} · clock unread — the date stays absent",
            url
        ),
    }

    let (code, body) = curl_bytes(&url);
    let code = match code {
        Some(c) => c,
        None => {
            eprintln!("fdsnwf: the route carried no HTTP verdict — transport unread");
            std::process::exit(1);
        }
    };
    eprintln!("fdsnwf: HTTP {} · {}", code, curl_verdict(code));
    if code != 200 {
        let note = String::from_utf8_lossy(&body);
        let note = note.trim();
        if note.is_empty() {
            eprintln!("fdsnwf: {} — no body in the response", curl_verdict(code));
        } else {
            eprintln!("fdsnwf: {} — {}", curl_verdict(code), note);
        }
        std::process::exit(1);
    }
    if body.is_empty() {
        eprintln!("fdsnwf: HTTP 200 with an empty body — no miniSEED records present");
        std::process::exit(1);
    }

    let st_path = match stations_path {
        Some(p) => p,
        None => String::new(),
    };
    let (stations, station_rows, rejected_rows) = load_stations(&st_path);
    eprintln!(
        "fdsnwf: station table {} rows, {} rejected lines",
        station_rows, rejected_rows
    );
    if stations.is_empty() {
        eprintln!("fdsnwf: no station anchors loaded — the join stays unread");
        std::process::exit(1);
    }

    let fallback_net = network.clone();
    let fallback_sta = station.clone();
    let emit_bin_path = arg_value(&args, "--emit-bin");
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let lsk = match &emit_bin_path {
        Some(_) => {
            let text = match arg_value(&args, "--lsk").and_then(|p| fs::read_to_string(p).ok()) {
                Some(t) => t,
                None => {
                    eprintln!("fdsnwf: --emit-bin needs --lsk <naif0012.tls> — the TDB clock stays unread");
                    std::process::exit(1);
                }
            };
            match parse_lsk(&text) {
                Some(l) => Some(l),
                None => {
                    eprintln!("fdsnwf: --lsk parses void");
                    std::process::exit(1);
                }
            }
        }
        None => None,
    };
    let mut fh = match out_path {
        Some(ref p) => match fs::File::create(p) {
            Ok(f) => Some(f),
            Err(_) => {
                eprintln!("fdsnwf: {} stayed unwritable", p);
                std::process::exit(1);
            }
        },
        None => None,
    };
    if let Some(ref mut f) = fh {
        let _ = f.write_all(b"#unit=m/s for velocity channels - m/s = counts * (1/InstrumentSensitivity) at ScaleFreq; the scalar seed sensitivity, not a full deconvolution\n");
        let _ =
            f.write_all(b"#unit=counts where the channel response is absent - never fabricated\n");
        let _ = f.write_all(b"#station|lat|lon|channel|unix_t|sample|unit\n");
    }

    let mut handled: HashMap<u8, usize> = HashMap::new();
    let mut unhandled_enc: HashMap<u8, usize> = HashMap::new();
    let mut samples_total = 0usize;
    let mut rows_written = 0usize;
    let mut station_unmatched = 0usize;
    let mut station_position_absent = 0usize;
    let mut rate_absent_records = 0usize;
    let mut meta_absent_records = 0usize;
    let mut time_absent_records = 0usize;
    let mut counts_samples = 0usize;
    let mut response_absent_records = 0usize;
    let mut channel_unmapped_records = 0usize;
    let mut elevation_absent_records = 0usize;
    let mut bin_recs: Vec<GeoRec> = Vec::new();
    let mut responses: HashMap<(String, String, String), Response> = HashMap::new();
    let byte_len = body.len();
    let mut pos = 0usize;

    while pos + 48 <= byte_len {
        let meta = match record_meta(&body[pos..]) {
            Some(m) => m,
            None => {
                meta_absent_records += 1;
                break;
            }
        };
        if meta.reclen < 64 || pos + meta.reclen > byte_len {
            eprintln!(
                "fdsnwf: record at {} carries reclen {} beyond the body — {} bytes unread",
                pos,
                meta.reclen,
                byte_len - pos
            );
            break;
        }
        let rec = &body[pos..pos + meta.reclen];
        let net_hdr = field_str(rec, 18, 2);
        let sta_hdr = field_str(rec, 8, 5);
        let loc_hdr = field_str(rec, 13, 2);
        let cha_hdr = field_str(rec, 15, 3);
        let net = if net_hdr.is_empty() {
            &fallback_net
        } else {
            &net_hdr
        };
        let sta = if sta_hdr.is_empty() {
            &fallback_sta
        } else {
            &sta_hdr
        };
        let key = (net.clone(), sta.clone());

        let fact = i16::from_be_bytes([rec[32], rec[33]]);
        let mult = i16::from_be_bytes([rec[34], rec[35]]);
        let mut rate = nominal_rate(fact, mult);
        if rate.is_none() {
            rate = meta.b100_rate;
        }
        let rate = match rate {
            Some(r) => r,
            None => {
                rate_absent_records += 1;
                pos += meta.reclen;
                continue;
            }
        };

        let enc = meta.encoding;
        if !encoding_supported(enc) {
            let n = unhandled_enc.entry(enc).or_insert(0);
            *n += 1;
            pos += meta.reclen;
            continue;
        }
        let n = handled.entry(enc).or_insert(0);
        *n += 1;
        let decoded = decode_record(&meta, rec, rate);
        samples_total += decoded.len();
        if decoded.is_empty() {
            time_absent_records += 1;
            pos += meta.reclen;
            continue;
        }

        let anchor = match stations.get(&key) {
            Some(a) => a,
            None => {
                station_unmatched += 1;
                pos += meta.reclen;
                continue;
            }
        };
        let (Some(lat), Some(lon)) = (anchor.lat, anchor.lon) else {
            station_position_absent += 1;
            pos += meta.reclen;
            continue;
        };
        let station_key = format!("{}.{}", net, sta);
        let chan = if loc_hdr.is_empty() {
            cha_hdr.clone()
        } else {
            format!("{}.{}", loc_hdr, cha_hdr)
        };

        let t0 = decoded[0].0;
        let resp = load_response(&mut responses, net, sta, &cha_hdr, &loc_hdr);
        let row = resp.row_at(t0);
        let gain = match row {
            Some(rw) if rw.units == "m/s" && rw.scale.is_finite() && rw.scale > 0.0 => {
                Some(1.0 / rw.scale)
            }
            _ => None,
        };
        if gain.is_none() {
            response_absent_records += 1;
        }
        let elev = match row.and_then(|rw| rw.elev).or(anchor.elev) {
            Some(e) if e.is_finite() => Some(e),
            _ => None,
        };
        let comp = if cha_hdr == "BHZ" {
            Some(COMP_FDSN_BHZ)
        } else {
            None
        };
        if emit_bin_path.is_some() {
            if gain.is_some() && comp.is_none() {
                channel_unmapped_records += 1;
            }
            if gain.is_some() && comp.is_some() && elev.is_none() {
                elevation_absent_records += 1;
            }
        }

        for (t, v) in &decoded {
            let si = gain.map(|g| *v * g);
            let val = si.unwrap_or(*v);
            let unit = if si.is_some() { "m/s" } else { "counts" };
            if si.is_none() {
                counts_samples += 1;
            }
            let mut line = String::new();
            line.push_str(&station_key);
            line.push('|');
            line.push_str(&lat.to_string());
            line.push('|');
            line.push_str(&lon.to_string());
            line.push('|');
            line.push_str(&chan);
            line.push('|');
            line.push_str(&format!("{:.4}", t));
            line.push('|');
            line.push_str(&val.to_string());
            line.push('|');
            line.push_str(unit);
            line.push('\n');
            match fh {
                Some(ref mut f) => {
                    if f.write_all(line.as_bytes()).is_err() {
                        eprintln!("fdsnwf: write stalled at sample {}", rows_written);
                        std::process::exit(1);
                    }
                }
                None => {
                    print!("{}", line);
                }
            }
            rows_written += 1;
            if emit_bin_path.is_some() {
                if let (Some(g), Some(c), Some(e), Some(lk)) = (si, comp, elev, lsk.as_ref()) {
                    if let Some(tdb) = lk.unix_to_tdb(*t) {
                        bin_recs.push(GeoRec {
                            t: tdb,
                            lat,
                            lon,
                            alt: e,
                            freq: 0.0,
                            bin_width: 0.0,
                            val: g,
                            comp: c,
                            station: 0,
                        });
                    }
                }
            }
        }
        pos += meta.reclen;
    }

    if let Some(path) = &emit_bin_path {
        if bin_recs.is_empty() {
            eprintln!(
                "fdsnwf: no SI m/s samples with a registered channel and a measured elevation — the bin stays unwritten (0 honored)"
            );
            std::process::exit(1);
        }
        bin_recs.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
        let bytes = write_bin(MAGIC_FDSN, &bin_recs);
        if fs::write(path, &bytes).is_err() {
            eprintln!("write {} returned void", path);
            std::process::exit(1);
        }
        match parse_bin(MAGIC_FDSN, &bytes) {
            Some(parsed) => eprintln!(
                "fdsnwf: {}: {} geo records, {} B, roundtrip parses",
                path,
                parsed.len(),
                bytes.len()
            ),
            None => {
                eprintln!("fdsnwf: {} roundtrip parse void", path);
                std::process::exit(1);
            }
        }
        if ci_mode && !upload_release(NETLOC, path) {
            std::process::exit(1);
        }
    }

    let mut enc_names: Vec<(&str, usize)> = Vec::new();
    let mut enc_keys: Vec<u8> = handled.keys().copied().collect();
    enc_keys.sort_unstable();
    for k in enc_keys {
        enc_names.push((encoding_name(k), handled[&k]));
    }
    for (name, n) in &enc_names {
        eprintln!("fdsnwf: encoding {} → {} records decoded", name, n);
    }
    let mut unhandled_keys: Vec<u8> = unhandled_enc.keys().copied().collect();
    unhandled_keys.sort_unstable();
    for k in unhandled_keys {
        eprintln!(
            "fdsnwf: encoding {} → {} records skipped, decode not built",
            k, unhandled_enc[&k]
        );
    }
    eprintln!(
        "fdsnwf: {} samples decoded, {} rows joined & written ({} in m/s, {} in counts)",
        samples_total,
        rows_written,
        rows_written - counts_samples,
        counts_samples
    );
    eprintln!(
        "fdsnwf: stations unmatched {} · position-absent {} · records: rate-absent {}, meta-absent {}, time-absent {}, response-absent {}, channel-unmapped {}, elevation-absent {}",
        station_unmatched,
        station_position_absent,
        rate_absent_records,
        meta_absent_records,
        time_absent_records,
        response_absent_records,
        channel_unmapped_records,
        elevation_absent_records
    );
    let sample_name = match out_path {
        Some(p) => p,
        None => "stdout".to_string(),
    };
    eprintln!("fdsnwf: {} → {}", sample_name, url);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_from_civil_is_epoch_correct() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2000, 1, 1), 10957);
        assert_eq!(days_from_civil(2020, 1, 1), 18262);
        assert_eq!(days_from_civil(2026, 9, 1), 20697);
    }

    #[test]
    fn seed_time_reads_header_clock() {
        let mut h = [0u8; 64];
        h[20] = 0x07;
        h[21] = 0xE4;
        h[22] = 0x00;
        h[23] = 0x01;
        h[24] = 0;
        h[25] = 0;
        h[26] = 0;
        h[28] = 0;
        h[29] = 0;
        assert_eq!(seed_unix(&h), Some(1577836800.0));
    }

    #[test]
    fn steim1_decodes_sample_series() {
        let mut words = [0u32; 16];
        words[0] = (2 << 24) | (1 << 22);
        words[1] = 1000;
        words[2] = 1010;
        words[3] = 0x0000_0001;
        words[4] = 0xFE05_0600;
        let got = steim_decode(&words, 10, 5);
        assert_eq!(got, vec![1000, 1001, 999, 1004, 1010]);
    }

    #[test]
    fn steim2_subcode_words_decode_sample_series() {
        let mut words = [0u32; 16];
        words[0] = (2 << 24) | (3 << 22);
        words[1] = 1000;
        words[2] = 1010;
        words[3] = (2 << 30) | 1;
        words[4] = (1 << 30) | (30 << 25) | (5 << 20) | (6 << 15);
        let got = steim_decode(&words, 11, 8);
        assert_eq!(got, vec![1000, 1001, 999, 1004, 1010, 1010, 1010, 1010]);
    }

    #[test]
    fn nominal_rate_from_factor_and_multiplier() {
        assert_eq!(nominal_rate(40, 1), Some(40.0));
        assert_eq!(nominal_rate(-10, 1), Some(0.1));
        assert_eq!(nominal_rate(0, 1), None);
        assert_eq!(nominal_rate(0, 0), None);
    }

    #[test]
    fn encoding_support_marks_built_decoders() {
        for e in [1u8, 2, 3, 4, 5, 10, 11] {
            assert!(encoding_supported(e));
            assert_ne!(encoding_name(e), "unhandled");
        }
        assert!(!encoding_supported(12));
        assert!(!encoding_supported(13));
        assert!(!encoding_supported(16));
    }

    #[test]
    fn iso_to_unix_parses_fdsn_channel_epochs() {
        assert_eq!(iso_to_unix("2020-01-01T00:00:00.0000"), Some(1577836800.0));
        assert_eq!(iso_to_unix("1989-08-29T00:00:00.0000"), Some(620352000.0));
        assert_eq!(iso_to_unix(""), None);
        assert_eq!(iso_to_unix("--"), None);
    }

    #[test]
    fn response_text_extracts_scale_gain_and_units() {
        let body = "#Network | Station | Location | Channel | Latitude | Longitude | Elevation | Depth | Azimuth | Dip | SensorDescription | Scale | ScaleFreq | ScaleUnits | SampleRate | StartTime | EndTime\nIU|ANMO|00|BHZ|34.945981|-106.457133|1671.0|145.0|0.0|-90.0|Geotech KS-54000 Borehole Seismometer|8.11548E8|0.02|m/s|20.0|2002-11-19T21:07:00.0000|2008-06-30T00:00:00.0000\nIU|ANMO|00|BHZ|34.945981|-106.457133|1671.0|145.0|0.0|-90.0|Geotech KS-54000 Borehole Seismometer|3.27511E9|0.02|m/s|20.0|2008-06-30T00:00:00.0000|\n";
        let r = parse_response_text(body);
        assert_eq!(r.rows.len(), 2, "rows: {:?}", r.rows);
        assert_eq!(r.rows[0].units, "m/s");
        assert!((r.rows[0].scale - 8.11548e8).abs() < 1.0);
        assert_eq!(r.rows[0].elev, Some(1671.0));
        assert_eq!(r.rows[0].start, 1037740020.0);
        assert_eq!(r.rows[1].start, 1214784000.0);
        let row1 = r.row_at(1041379200.0).unwrap();
        assert_eq!(row1.scale, 8.11548e8);
        assert_eq!(
            r.row_at(1250553600.0).map(|x| x.scale),
            Some(3.27511e9),
            "rows {:?}",
            r.rows
        );
        assert_eq!(r.row_at(1.0e9), None);
    }
}
