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

pub fn decode_body(body: &[u8]) -> Option<(Vec<(f64, f64)>, f64)> {
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
