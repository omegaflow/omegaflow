use std::env;
use std::fs;

fn ymd_to_days(y: i64, m: i64, d: i64) -> Option<f64> {
    let (m, y) = if m <= 2 { (m + 12, y - 1) } else { (m, y) };
    let a = y / 100;
    let b = 2 - a + a / 4;
    Some(
        (365.25 * (y + 4716) as f64).floor()
            + (30.6001 * (m + 1) as f64).floor()
            + d as f64
            + b as f64
            - 1524.5,
    )
}

fn mseed_time(h: &[u8]) -> Option<f64> {
    let year = ((h[20] as u32) << 8 | h[21] as u32) as i64;
    let doy = ((h[22] as u32) << 8 | h[23] as u32) as u32;

    let hour = h[25] as f64;
    let min = h[26] as f64;
    let sec = h[27] as f64;
    let frac = h[29] as f64 / 100.0;
    let days = ymd_to_days(year, 1, 1)?;
    Some((days + doy as f64 - 1.0) * 86400.0 + hour * 3600.0 + min * 60.0 + sec + frac)
}

fn sign_extend(v: u32, bits: u32) -> i64 {
    let shift = 32 - bits;
    ((v << shift) as i32 as i64) >> shift
}

fn steim_decode(words: &[u32], encoding: u8, nsamp: usize) -> Vec<i32> {
    let nframes = words.len() / 16;
    let mut out: Vec<i32> = Vec::with_capacity(nsamp.min(words.len() * 8));
    let mut xn: i64 = 0;
    for fi in 0..nframes {
        if out.len() >= nsamp {
            break;
        }
        let base = fi * 16;
        let ctrl = words[base];
        let start: usize = if fi == 0 {
            out.push(words[base + 1] as i32);
            xn = words[base + 1] as i64;
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
                            _ => {
                                diffs.push(sign_extend((w >> 20) & 0x3FF, 10));
                                diffs.push(sign_extend((w >> 10) & 0x3FF, 10));
                                diffs.push(sign_extend(w & 0x3FF, 10));
                            }
                        }
                    }
                }
                3 => {
                    if encoding == 10 {
                        diffs.push(sign_extend(w, 32));
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
        let dstart = if fi == 0 { 1 } else { 0 };
        for idx in dstart..diffs.len() {
            if out.len() >= nsamp {
                break;
            }
            xn += diffs[idx];
            out.push(xn as i32);
        }
    }
    out.truncate(nsamp);
    out
}

fn mseed_samples(
    h: &[u8],
    data: &[u8],
    encoding: u8,
    byte_order: u8,
    nsamp: usize,
    rate: f64,
) -> Vec<(f64, f64)> {
    let big = byte_order == 1;
    let rd16 = |i: usize| -> u16 {
        let b = [data[i], data[i + 1]];
        if big {
            u16::from_be_bytes(b)
        } else {
            u16::from_le_bytes(b)
        }
    };
    let rd32 = |i: usize| -> u32 {
        let b = [data[i], data[i + 1], data[i + 2], data[i + 3]];
        if big {
            u32::from_be_bytes(b)
        } else {
            u32::from_le_bytes(b)
        }
    };
    let mut raw: Vec<i32> = Vec::with_capacity(nsamp);

    let avail = data.len() / 2;
    let nsamp = nsamp.min(avail);
    match encoding {
        1 | 100 => {
            for i in 0..nsamp {
                raw.push(rd16(i * 2) as i16 as i32);
            }
        }
        2 => {
            for i in 0..nsamp {
                let b = [data[i * 3], data[i * 3 + 1], data[i * 3 + 2]];
                let v = if big {
                    (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32
                } else {
                    (b[2] as u32) << 16 | (b[1] as u32) << 8 | b[0] as u32
                };
                raw.push(sign_extend(v, 24) as i32);
            }
        }
        3 => {
            for i in 0..nsamp {
                raw.push(rd32(i * 4) as i32);
            }
        }
        4 => {
            for i in 0..nsamp {
                let b = [
                    data[i * 4],
                    data[i * 4 + 1],
                    data[i * 4 + 2],
                    data[i * 4 + 3],
                ];
                let f = if big {
                    f32::from_be_bytes(b)
                } else {
                    f32::from_le_bytes(b)
                };
                raw.push(f as i32);
            }
        }
        5 => {
            for i in 0..nsamp {
                let b = [
                    data[i * 8],
                    data[i * 8 + 1],
                    data[i * 8 + 2],
                    data[i * 8 + 3],
                    data[i * 8 + 4],
                    data[i * 8 + 5],
                    data[i * 8 + 6],
                    data[i * 8 + 7],
                ];
                let f = if big {
                    f64::from_be_bytes(b)
                } else {
                    f64::from_le_bytes(b)
                };
                raw.push(f as i32);
            }
        }
        10 | 11 => {
            let mut words = Vec::with_capacity(data.len() / 4);
            let mut i = 0usize;
            while i + 4 <= data.len() {
                words.push(rd32(i));
                i += 4;
            }
            raw = steim_decode(&words, encoding, nsamp);
        }
        _ => return Vec::new(),
    }
    raw.truncate(nsamp);
    let t = mseed_time(h).unwrap_or(0.0);
    let mut out = Vec::with_capacity(raw.len());
    for (i, v) in raw.iter().enumerate() {
        out.push((t + i as f64 / rate, *v as f64));
    }
    out
}

fn parse_mseed(bytes: &[u8]) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos + 64 <= bytes.len() {
        let h = &bytes[pos..pos + 64];
        let data_start = ((h[44] as usize) << 8 | h[45] as usize).max(64);
        let nsamp = (h[30] as usize) << 8 | h[31] as usize;
        let factor = i16::from_be_bytes([h[32], h[33]]);
        let mult = i16::from_be_bytes([h[34], h[35]]);
        let rate = if factor > 0 {
            factor as f64 * mult as f64
        } else if factor < 0 {
            -(mult as f64) / (factor as f64)
        } else {
            0.0
        };
        let encoding = h[52];
        let byte_order = h[53];

        let rec_len = if h[54] == 0 {
            512usize
        } else {
            1usize << h[54]
        };
        if data_start >= rec_len || rate <= 0.0 {
            break;
        }
        let end = (pos + rec_len).min(bytes.len());
        let data = &bytes[pos + data_start..end];
        out.extend(mseed_samples(h, data, encoding, byte_order, nsamp, rate));
        pos += rec_len;
        if pos >= bytes.len() {
            break;
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: mseed_messen <file.mseed> [start_epoch] [end_epoch]");
        return;
    }
    let bytes = match fs::read(&args[1]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("lesen {}: {e}", args[1]);
            return;
        }
    };
    let samples = parse_mseed(&bytes);
    if samples.is_empty() {
        println!("no samples");
        return;
    }
    let start = args.get(2).and_then(|s| s.parse::<f64>().ok());
    let end = args.get(3).and_then(|s| s.parse::<f64>().ok());
    println!("samples: {}", samples.len());
    let (t0, t1) = (
        samples.first().map(|x| x.0).unwrap(),
        samples.last().map(|x| x.0).unwrap(),
    );
    println!("zeitbereich: {} .. {}  (UTC)", t0, t1);

    let mut mn = f64::INFINITY;
    let mut mx = f64::NEG_INFINITY;
    for (_, v) in &samples {
        if *v < mn {
            mn = *v;
        }
        if *v > mx {
            mx = *v;
        }
    }
    println!("global: min {mn:.1} max {mx:.1}");

    let lo = start.unwrap_or(t0);
    let hi = end.unwrap_or(t1);
    let mut cell = (lo / 60.0).floor() * 60.0;
    let mut acc: Vec<(f64, f64, f64, f64, usize)> = Vec::new();
    let mut cur: Option<(f64, f64, f64, f64, usize)> = None;
    for (t, v) in &samples {
        if *t < lo || *t > hi {
            continue;
        }
        let c = (t / 60.0).floor() * 60.0;
        if cur.is_none() {
            cur = Some((c, 0.0, *v, *v, 1));
            cell = c;
        }
        if c != cell {
            if let Some((cb, s, cmx, cmn, n)) = cur.take() {
                let rms = (s / n as f64).sqrt();
                acc.push((cb, rms, cmx, cmn, n));
            }
            cell = c;
            cur = Some((c, 0.0, *v, *v, 1));
        }
        let (_, s, cmx, cmn, n) = cur.as_mut().unwrap();
        *s += v * v;
        *n += 1;
        if *v > *cmx {
            *cmx = *v;
        }
        if *v < *cmn {
            *cmn = *v;
        }
    }
    if let Some((cb, s, cmx, cmn, n)) = cur {
        let rms = (s / n as f64).sqrt();
        acc.push((cb, rms, cmx, cmn, n));
    }
    println!("== 1-min cells (start_utc | n | rms | max | min) ==");
    for (cb, rms, cmx, cmn, n) in acc {
        println!("{:.0} | {n} | {rms:.1} | {cmx:.1} | {cmn:.1}", cb);
    }

    let ymd = ymd_to_days(2026, 8, 26).unwrap();
    let kollab = (ymd + 0.0) * 86400.0;
    println!("kollab_epoch_referenz: {kollab:.0}");
}
