use omegaflow::archivar::fetch::civil_date;
use omegaflow::archivar::geo::{COMP_CEIN_BDF, GeoRec, MAGIC_CEIN, parse_bin, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const ROUTE: &str = "http://ceein.infp.ro/fdsnws/dataselect/1/query";
const STATION_ROUTE: &str = "http://ceein.infp.ro/fdsnws/station/1/query";
const NETLOC: &str = "ceein.infp.ro";
const NETWORK: &str = "C9";
const CHANNEL: &str = "BDF";
const DEFAULT_START: &str = "2023-10-01T00:00:00";
const DEFAULT_END: &str = "2023-10-01T00:10:00";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_finite(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn be16(b: &[u8], i: usize) -> u16 {
    ((b[i] as u16) << 8) | b[i + 1] as u16
}

fn be_f32(b: &[u8], i: usize) -> f32 {
    f32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
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
    let tmp = env::temp_dir().join(format!("ceein_infrasound_{}.bin", std::process::id()));
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-g")
        .arg("-m")
        .arg("240")
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

struct Anchor {
    lat: f64,
    lon: f64,
    elev: f64,
}

struct RespRow {
    start: f64,
    end: f64,
    scale: f64,
    units: String,
}

impl RespRow {
    fn sensor_gain(&self) -> Option<f64> {
        if self.units.eq_ignore_ascii_case("Pa") && self.scale.is_finite() && self.scale > 0.0 {
            Some(1.0 / self.scale)
        } else {
            None
        }
    }
}

fn parse_channel_table(
    body: &str,
) -> (
    HashMap<(String, String), Anchor>,
    HashMap<(String, String, String), Vec<RespRow>>,
) {
    let mut anchors = HashMap::new();
    let mut resp: HashMap<(String, String, String), Vec<RespRow>> = HashMap::new();
    for line in body.lines() {
        let t = line.trim_end_matches('\r').trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = t.split('|').map(|x| x.trim()).collect();
        if c.len() < 17 {
            continue;
        }
        let net = c[0].to_string();
        let sta = c[1].to_string();
        let cha = c[3].to_string();
        if let (Some(lat), Some(lon), Some(elev)) =
            (parse_finite(c[4]), parse_finite(c[5]), parse_finite(c[6]))
        {
            anchors
                .entry((net.clone(), sta.clone()))
                .or_insert(Anchor { lat, lon, elev });
        }
        let scale = match parse_finite(c[11]) {
            Some(s) if s > 0.0 => s,
            _ => continue,
        };
        let start = iso_to_unix(c[15]).unwrap_or(f64::NEG_INFINITY);
        let end = iso_to_unix(c[16]).unwrap_or(f64::INFINITY);
        resp.entry((net, sta, cha)).or_default().push(RespRow {
            start,
            end,
            scale,
            units: c[13].to_string(),
        });
    }
    (anchors, resp)
}

fn response_at<'a>(rows: &'a [RespRow], t: f64) -> Option<&'a RespRow> {
    rows.iter().find(|r| t >= r.start && t < r.end)
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
    let out_bin = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => {
            eprintln!("ceein: --out-bin names the geo bin path");
            std::process::exit(1);
        }
    };
    let lsk = match arg_value(&args, "--lsk").and_then(|p| fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("ceein: --lsk <naif0012.tls> — the TDB clock stays unread");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk) {
        Some(l) => l,
        None => {
            eprintln!("ceein: --lsk parses void");
            std::process::exit(1);
        }
    };
    let start = match arg_value(&args, "--start") {
        Some(v) => v,
        None => DEFAULT_START.to_string(),
    };
    let end = match arg_value(&args, "--end") {
        Some(v) => v,
        None => DEFAULT_END.to_string(),
    };
    let channel = match arg_value(&args, "--channel") {
        Some(v) => v,
        None => CHANNEL.to_string(),
    };
    let network = match arg_value(&args, "--network") {
        Some(v) => v,
        None => NETWORK.to_string(),
    };
    let station_filter = arg_value(&args, "--station").filter(|s| !s.is_empty());
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs());
    match now {
        Some(s) => {
            let (y, m, d) = civil_date(s);
            eprintln!("ceein: dataselect window {start} → {end} · {y}-{m:02}-{d:02} UTC");
        }
        None => eprintln!("ceein: clock unread — the reading date stays absent"),
    }

    let station_url = format!(
        "{}?network={}&level=channel&format=text",
        STATION_ROUTE, network
    );
    let (code, body) = curl_bytes(&station_url);
    let code = match code {
        Some(c) => c,
        None => {
            eprintln!("ceein: the station route carried no HTTP verdict — transport unread");
            std::process::exit(1);
        }
    };
    if code != 200 {
        eprintln!("ceein: station HTTP {} · {}", code, curl_verdict(code));
        std::process::exit(1);
    }
    let table = String::from_utf8_lossy(&body);
    let (anchors, responses) = parse_channel_table(&table);
    eprintln!(
        "ceein: channel table {} anchors, {} channel keys · {}",
        anchors.len(),
        responses.len(),
        station_url
    );
    if anchors.is_empty() {
        eprintln!("ceein: no station anchors in the channel table — the bin stays unwritten");
        std::process::exit(1);
    }

    let mut targets: Vec<(String, String)> = responses
        .iter()
        .filter(|((net, _, cha), rows)| {
            net == &network && cha == &channel && rows.iter().any(|r| r.sensor_gain().is_some())
        })
        .map(|((net, sta, _), _)| (net.clone(), sta.clone()))
        .collect();
    targets.sort();
    targets.dedup();
    if let Some(f) = &station_filter {
        targets.retain(|(_, sta)| sta == f);
    }
    if targets.is_empty() {
        eprintln!(
            "ceein: no station carries {channel} in Pa within the channel table — the bin stays unwritten"
        );
        std::process::exit(1);
    }
    eprintln!("ceein: {} station(s) target {channel}", targets.len());

    let mut records: Vec<GeoRec> = Vec::new();
    let mut handled: HashMap<u8, usize> = HashMap::new();
    let mut unhandled_enc: HashMap<u8, usize> = HashMap::new();
    let mut samples_total = 0usize;
    let mut records_void = 0usize;
    let mut anchor_absent = 0usize;
    let mut response_absent = 0usize;
    let mut rate_absent = 0usize;
    let mut time_absent = 0usize;

    for (net, sta) in &targets {
        let url = format!(
            "{}?network={}&station={}&channel={}&starttime={}&endtime={}&format=miniseed",
            ROUTE, net, sta, channel, start, end
        );
        let (code, body) = curl_bytes(&url);
        let code = match code {
            Some(c) => c,
            None => {
                eprintln!("ceein: {}·{} HTTP unread — skipped", net, sta);
                continue;
            }
        };
        eprintln!("ceein: {}·{} HTTP {} · {} B", net, sta, code, body.len());
        if code != 200 || body.is_empty() {
            continue;
        }
        let Some(anchor) = anchors.get(&(net.clone(), sta.clone())) else {
            anchor_absent += 1;
            continue;
        };
        let resp_key = (net.clone(), sta.clone(), channel.clone());
        let resp_rows = match responses.get(&resp_key) {
            Some(r) => r,
            None => {
                response_absent += 1;
                continue;
            }
        };

        let byte_len = body.len();
        let mut pos = 0usize;
        while pos + 48 <= byte_len {
            let meta = match record_meta(&body[pos..]) {
                Some(m) => m,
                None => break,
            };
            if meta.reclen < 64 || pos + meta.reclen > byte_len {
                eprintln!(
                    "ceein: {}·{} record at {} carries reclen {} beyond the body — {} bytes unread",
                    net,
                    sta,
                    pos,
                    meta.reclen,
                    byte_len - pos
                );
                break;
            }
            let rec = &body[pos..pos + meta.reclen];
            if !encoding_supported(meta.encoding) {
                *unhandled_enc.entry(meta.encoding).or_insert(0) += 1;
                pos += meta.reclen;
                continue;
            }
            *handled.entry(meta.encoding).or_insert(0) += 1;

            let fact = i16::from_be_bytes([rec[32], rec[33]]);
            let mult = i16::from_be_bytes([rec[34], rec[35]]);
            let rate = nominal_rate(fact, mult).or(meta.b100_rate);
            let rate = match rate {
                Some(r) => r,
                None => {
                    rate_absent += 1;
                    pos += meta.reclen;
                    continue;
                }
            };
            let decoded = decode_record(&meta, rec, rate);
            samples_total += decoded.len();
            if decoded.is_empty() {
                time_absent += 1;
                pos += meta.reclen;
                continue;
            }
            let t0 = decoded[0].0;
            let gain = match response_at(resp_rows, t0).and_then(|r| r.sensor_gain()) {
                Some(g) => g,
                None => {
                    response_absent += 1;
                    pos += meta.reclen;
                    continue;
                }
            };
            let mut pushed = 0usize;
            for (t, v) in &decoded {
                let val = *v * gain;
                if !val.is_finite() {
                    continue;
                }
                let Some(tdb) = lsk.unix_to_tdb(*t) else {
                    continue;
                };
                records.push(GeoRec {
                    t: tdb,
                    lat: anchor.lat,
                    lon: anchor.lon,
                    alt: anchor.elev,
                    freq: 0.0,
                    bin_width: 0.0,
                    val,
                    comp: COMP_CEIN_BDF,
                    station: 0,
                });
                pushed += 1;
            }
            if pushed == 0 {
                records_void += 1;
            }
            pos += meta.reclen;
        }
    }

    if records.is_empty() {
        eprintln!(
            "ceein: no Pa samples with a measured anchor in {start} → {end} — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bytes = write_bin(MAGIC_CEIN, &records);
    if fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {} returned void", out_bin);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_CEIN, &bytes) {
        Some(parsed) => eprintln!(
            "ceein: {}: {} geo records, {} B, roundtrip parses",
            out_bin,
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("ceein: {} roundtrip parse void", out_bin);
            std::process::exit(1);
        }
    }

    let mut enc_keys: Vec<u8> = handled.keys().copied().collect();
    enc_keys.sort_unstable();
    for k in enc_keys {
        eprintln!(
            "ceein: encoding {} → {} records decoded",
            encoding_name(k),
            handled[&k]
        );
    }
    let mut unhandled_keys: Vec<u8> = unhandled_enc.keys().copied().collect();
    unhandled_keys.sort_unstable();
    for k in unhandled_keys {
        eprintln!(
            "ceein: encoding {} → {} records skipped, decode not built",
            k, unhandled_enc[&k]
        );
    }
    eprintln!(
        "ceein: {} samples decoded, {} Pa records written ({} void, anchor-absent {}, response-absent {}, rate-absent {}, time-absent {})",
        samples_total,
        records.len(),
        records_void,
        anchor_absent,
        response_absent,
        rate_absent,
        time_absent
    );

    if ci_mode && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_from_civil_is_epoch_correct() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2000, 1, 1), 10957);
        assert_eq!(days_from_civil(2023, 10, 1), 19631);
    }

    #[test]
    fn iso_to_unix_parses_channel_epochs() {
        assert_eq!(iso_to_unix("2023-10-01T00:00:00"), Some(1696118400.0));
        assert_eq!(iso_to_unix("2023-10-31T00:00:00"), Some(1698710400.0));
        assert_eq!(iso_to_unix(""), None);
        assert_eq!(iso_to_unix("--"), None);
    }

    #[test]
    fn channel_table_reads_anchor_and_pa_sensitivity() {
        let body = "#Network|Station|Location|Channel|Latitude|Longitude|Elevation|Depth|Azimuth|Dip|SensorDescription|Scale|ScaleFreq|ScaleUnits|SampleRate|StartTime|EndTime\nC9|PVCI2|00|BDF|50.52864|14.566283|315.0||||differential microbarometer|10000.0|1.0|Pa|25.0|2023-01-01T00:00:00|2023-10-30T23:59:59.999\n";
        let (anchors, resp) = parse_channel_table(body);
        let a = anchors
            .get(&("C9".to_string(), "PVCI2".to_string()))
            .unwrap();
        assert_eq!(a.elev, 315.0);
        let rows = resp
            .get(&("C9".to_string(), "PVCI2".to_string(), "BDF".to_string()))
            .unwrap();
        let r = response_at(rows, 1696118400.0).unwrap();
        assert_eq!(r.sensor_gain(), Some(0.0001));
    }

    #[test]
    fn non_pa_units_carry_no_gain() {
        let rows = vec![RespRow {
            start: f64::NEG_INFINITY,
            end: f64::INFINITY,
            scale: 10000.0,
            units: "counts".to_string(),
        }];
        assert_eq!(rows[0].sensor_gain(), None);
    }
}
