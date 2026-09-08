use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

const ORIGIN_UNIX: f64 = 1786744701.505;
const EVENT_LAT: f64 = -8.3514;
const EVENT_LON: f64 = 121.3478;
const EVENT_ID: &str = "us6000tkt2";
const START: &str = "2026-08-14T21:58:21";
const END: &str = "2026-08-14T22:58:21";
const ROUTE: &str = "https://service.earthscope.org/fdsnws/dataselect/1/query";
const R_EARTH_KM: f64 = 6371.0;
const V_SURFACE_LO: f64 = 1.5;
const V_SURFACE_HI: f64 = 5.5;
const SHELF_FORCE: u8 = 4;
const PI: f64 = std::f64::consts::PI;

const BANDS: [f64; 11] = [
    20.0, 25.0, 30.0, 40.0, 50.0, 60.0, 80.0, 100.0, 125.0, 150.0, 200.0,
];

const STATIONS: [(&str, &str, f64, f64); 19] = [
    ("II", "KAPI", -5.0142, 119.7517),
    ("II", "COCO", -12.1901, 96.8349),
    ("IU", "PMG", -9.4047, 147.1597),
    ("IU", "GUMO", 13.5893, 144.8684),
    ("IU", "TATO", 24.9735, 121.4971),
    ("IU", "CHTO", 18.8140, 98.9445),
    ("II", "TAU", -42.9082, 147.3210),
    ("II", "PALK", 7.2728, 80.7022),
    ("IU", "MAJO", 36.5457, 138.2041),
    ("II", "DGAR", -7.4121, 72.4525),
    ("IU", "ULN", 47.8651, 107.0532),
    ("II", "TLY", 51.6807, 103.6438),
    ("II", "NIL", 33.6506, 73.2686),
    ("IU", "MAKZ", 46.8080, 81.9770),
    ("II", "UOSS", 24.9453, 56.2042),
    ("II", "ABPO", -19.0180, 47.2290),
    ("G", "ATD", 11.5307, 42.8466),
    ("IU", "GNI", 40.1480, 44.7410),
    ("II", "KIV", 43.9553, 42.6863),
];

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
    let tmp = env::temp_dir().join(format!("rayleigh_{}.mseed", std::process::id()));
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

fn decode_body(body: &[u8]) -> Option<(Vec<(f64, f64)>, f64)> {
    let meta = record_meta(body)?;
    let reclen = meta.reclen;
    if reclen == 0 {
        return None;
    }
    let fact = be16(body, 32) as i16;
    let mult = be16(body, 34) as i16;
    let rate = nominal_rate(fact, mult).or(meta.b100_rate)?;
    let mut samples: Vec<(f64, f64)> = Vec::new();
    let mut off = 0usize;
    while off + reclen <= body.len() {
        let rec = &body[off..off + reclen];
        if let Some(m) = record_meta(rec) {
            samples.extend(decode_record(&m, rec, rate));
        }
        off += reclen;
    }
    if samples.is_empty() {
        return None;
    }
    Some((samples, rate))
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let (a1, b1) = (lat1.to_radians(), lon1.to_radians());
    let (a2, b2) = (lat2.to_radians(), lon2.to_radians());
    let dh = (a2 - a1) / 2.0;
    let dl = (b2 - b1) / 2.0;
    let h = dh.sin() * dh.sin() + a1.cos() * a2.cos() * dl.sin() * dl.sin();
    2.0 * R_EARTH_KM * h.sqrt().atan2((1.0 - h).sqrt())
}

fn goertzel(x: &[f64], freq: f64, rate: f64) -> f64 {
    let w = 2.0 * PI * freq / rate;
    let coeff = 2.0 * w.cos();
    let mut s0 = 0.0;
    let mut s1 = 0.0;
    for &v in x {
        let s = v + coeff * s0 - s1;
        s1 = s0;
        s0 = s;
    }
    (s0 * s0 + s1 * s1 - coeff * s0 * s1).sqrt()
}

fn band_group_velocities(samples: &[(f64, f64)], rate: f64, dist_km: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for &period in BANDS.iter() {
        let freq = 1.0 / period;
        let w = (3.0 * period * rate) as usize;
        if w < 3 || w > samples.len() {
            continue;
        }
        let step = w / 3;
        let t_start = ORIGIN_UNIX + dist_km / V_SURFACE_HI;
        let mut best = f64::NEG_INFINITY;
        let mut best_t = t_start;
        let mut i = 0usize;
        while i + w <= samples.len() {
            let tc = samples[i + w / 2].0;
            if tc >= t_start {
                let mut window = Vec::with_capacity(w);
                for k in i..i + w {
                    window.push(samples[k].1);
                }
                let mag = goertzel(&window, freq, rate);
                if mag > best {
                    best = mag;
                    best_t = tc;
                }
            }
            i += step;
        }
        if best > 0.0 && best_t > t_start {
            let vg = dist_km / (best_t - ORIGIN_UNIX);
            if vg.is_finite() && vg >= V_SURFACE_LO && vg <= V_SURFACE_HI {
                out.push((freq, vg));
            }
        }
    }
    out
}

fn station_velocities(net: &str, sta: &str, dist_km: f64) -> Vec<(f64, f64)> {
    let url = format!(
        "{}?network={}&station={}&channel=BHZ&starttime={}&endtime={}&format=miniseed",
        ROUTE, net, sta, START, END
    );
    let (code, body) = curl_bytes(&url);
    if code != Some(200) || body.is_empty() {
        eprintln!("{}·{} dataselect HTTP {:?} — skipped", net, sta, code);
        return Vec::new();
    }
    let Some((samples, rate)) = decode_body(&body) else {
        eprintln!("{}·{} carries no decodable record — skipped", net, sta);
        return Vec::new();
    };
    band_group_velocities(&samples, rate, dist_km)
}

fn sha256_hex(data: &[u8]) -> String {
    let mut digest = [0u32; 8];
    digest[0] = 0x6a09e667;
    digest[1] = 0xbb67ae85;
    digest[2] = 0x3c6ef372;
    digest[3] = 0xa54ff53a;
    digest[4] = 0x510e527f;
    digest[5] = 0x9b05688c;
    digest[6] = 0x1f83d9ab;
    digest[7] = 0x5be0cd19;
    let mut msg: Vec<u8> = Vec::with_capacity(data.len() + 64);
    msg.extend_from_slice(data);
    let bitlen = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    for i in 0..8 {
        let shift = (7 - i) * 8;
        msg.push(((bitlen >> shift) & 0xff) as u8);
    }
    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = ((chunk[i * 4] as u32) << 24)
                | ((chunk[i * 4 + 1] as u32) << 16)
                | ((chunk[i * 4 + 2] as u32) << 8)
                | (chunk[i * 4 + 3] as u32);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut a = digest[0];
        let mut b = digest[1];
        let mut c = digest[2];
        let mut d = digest[3];
        let mut e = digest[4];
        let mut f = digest[5];
        let mut g = digest[6];
        let mut h = digest[7];
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        digest[0] = digest[0].wrapping_add(a);
        digest[1] = digest[1].wrapping_add(b);
        digest[2] = digest[2].wrapping_add(c);
        digest[3] = digest[3].wrapping_add(d);
        digest[4] = digest[4].wrapping_add(e);
        digest[5] = digest[5].wrapping_add(f);
        digest[6] = digest[6].wrapping_add(g);
        digest[7] = digest[7].wrapping_add(h);
    }
    let mut out = String::with_capacity(64);
    for v in digest {
        out.push_str(&format!("{:08x}", v));
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let shelf_path = arg_value(&args, "--write-shelf");
    let results_path = arg_value(&args, "--results");

    let mut done: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    if let Some(rp) = &results_path {
        if let Ok(body) = fs::read_to_string(rp) {
            for line in body.lines() {
                let Some((key, vals)) = line.split_once('|') else {
                    continue;
                };
                let mut v: Vec<(f64, f64)> = Vec::new();
                for pair in vals.split(',') {
                    if let Some((f, vg)) = pair.split_once(':') {
                        if let (Ok(f), Ok(vg)) = (f.parse::<f64>(), vg.parse::<f64>()) {
                            v.push((f, vg));
                        }
                    }
                }
                done.insert(key.to_string(), v);
            }
        }
    }

    println!(
        "=== rayleigh dispersion — {} M7.8, {} BHZ stations, {} bands ===",
        EVENT_ID,
        STATIONS.len(),
        BANDS.len()
    );
    let mut acc: HashMap<String, Vec<(f64, f64)>> = done;
    for (net, sta, lat, lon) in STATIONS.iter() {
        let key = format!("{}.{}", net, sta);
        if acc.contains_key(&key) {
            eprintln!("{} already measured — skipped", key);
            continue;
        }
        let dist = haversine_km(EVENT_LAT, EVENT_LON, *lat, *lon);
        let vs = station_velocities(net, sta, dist);
        if vs.is_empty() {
            eprintln!("{} dist {:.0} km — no band velocity", key, dist);
            continue;
        }
        let line = format!(
            "{}|{}",
            key,
            vs.iter()
                .map(|(f, v)| format!("{}:{:.4}", f, v))
                .collect::<Vec<_>>()
                .join(",")
        );
        if let Some(rp) = &results_path {
            if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(rp) {
                let _ = writeln!(file, "{}", line);
            }
        }
        eprintln!(
            "{} dist {:.0} km — {} bands: {}",
            key,
            dist,
            vs.len(),
            vs.iter()
                .map(|(f, v)| format!("{:.3}Hz {:.3}km/s", f, v))
                .collect::<Vec<_>>()
                .join(", ")
        );
        acc.insert(key, vs);
    }

    println!();
    println!("band | period_s | freq_hz | vg_median_km_s | vg_unc_km_s | stations");
    let mut shelf_rows: Vec<String> = Vec::new();
    let mut by_freq: HashMap<u64, Vec<f64>> = HashMap::new();
    for vs in acc.values() {
        for (f, v) in vs {
            by_freq.entry(f.to_bits()).or_default().push(*v);
        }
    }
    let mut freqs: Vec<(f64, Vec<f64>)> = by_freq
        .into_iter()
        .map(|(b, v)| (f64::from_bits(b), v))
        .collect();
    freqs.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (freq, mut vs) in freqs {
        vs.sort_by(|a, b| a.total_cmp(b));
        let n = vs.len();
        if n < 3 {
            println!(
                "{:>4.0}s | {:>7.1} | {:>9.5} | skipped (<3 stations)",
                1.0 / freq,
                1.0 / freq,
                freq
            );
            continue;
        }
        let median = vs[n / 2];
        let q1 = vs[n / 4];
        let q3 = vs[3 * n / 4];
        let unc = 0.5 * (q3 - q1);
        println!(
            "{:>4.0}s | {:>7.1} | {:>9.5} | {:>13.3} | {:>11.3} | {:>5}",
            1.0 / freq,
            1.0 / freq,
            freq,
            median,
            unc,
            n
        );
        let v_m_s = median * 1000.0;
        let unc_m_s = unc * 1000.0;
        let bin_width = freq / 3.0;
        shelf_rows.push(format!(
            "{} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e}",
            SHELF_FORCE, freq, bin_width, v_m_s, unc_m_s, ORIGIN_UNIX
        ));
    }

    if let Some(path) = shelf_path {
        let mut existing: Vec<String> = Vec::new();
        if let Ok(raw) = fs::read_to_string(&path) {
            for line in raw.lines() {
                if line.starts_with('#') {
                    continue;
                }
                if let Some(f) = line.split_whitespace().next() {
                    if f.parse::<u8>().ok() != Some(SHELF_FORCE) {
                        existing.push(line.to_string());
                    }
                }
            }
        }
        let mut body = String::new();
        for line in existing.iter().chain(shelf_rows.iter()) {
            body.push_str(line);
            body.push('\n');
        }
        let hex = sha256_hex(body.as_bytes());
        let mut out = String::from("# v_freq_shelf v1 sha256:");
        out.push_str(&hex);
        out.push_str("\n# columns: force_type freq_hz bin_width_hz v_m_s v_unc_m_s epoch_tdb\n");
        out.push_str(&body);
        if fs::write(&path, out).is_ok() {
            println!(
                "shelf written: force-4 rows {}, sha256 {}",
                shelf_rows.len(),
                hex
            );
        } else {
            eprintln!("{} unwritable", path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_synthetic_dispersive_arrival_is_recovered() {
        let rate = 20.0;
        let dist_km = 3000.0;
        let freq = 0.025;
        let vg = 3.5;
        let n = (3600.0 * rate) as usize;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = ORIGIN_UNIX + i as f64 / rate;
            let dt = t - (ORIGIN_UNIX + dist_km / vg);
            let env = (-(dt * dt) / (2.0 * 120.0 * 120.0)).exp();
            let v = 5.0 * env * (2.0 * PI * freq * dt).sin();
            samples.push((t, v));
        }
        let vs = band_group_velocities(&samples, rate, dist_km);
        let hit = vs.iter().find(|(f, _)| (*f - 0.025).abs() < 1e-4);
        assert!(hit.is_some(), "the injected 40-s band must be found");
        let (_, v) = hit.unwrap();
        assert!(
            (v - vg).abs() < 0.2,
            "recovered v_g {} must track the injected {}",
            v,
            vg
        );
    }

    #[test]
    fn a_flat_quiet_signal_carries_no_velocity() {
        let rate = 20.0;
        let dist_km = 3000.0;
        let n = (3600.0 * rate) as usize;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = ORIGIN_UNIX + i as f64 / rate;
            samples.push((t, 0.0));
        }
        let vs = band_group_velocities(&samples, rate, dist_km);
        assert!(vs.is_empty(), "silence must yield no band velocity");
    }
}
