use omegaflow::archivar::fits::{FitsHeader, FitsImage};

fn arg_values(args: &[String], key: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == key {
            out.push(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

fn sum32(bytes: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 4 <= bytes.len() {
        let w = u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
        let (s, c) = sum.overflowing_add(w);
        sum = s;
        if c {
            sum = sum.wrapping_add(1);
        }
        i += 4;
    }
    let mut tail = 0u32;
    let rem = bytes.len() - i;
    for (k, b) in bytes[i..].iter().enumerate() {
        tail |= u32::from(*b) << (24 - 8 * k);
    }
    if rem > 0 {
        let (s, c) = sum.overflowing_add(tail);
        sum = s;
        if c {
            sum = sum.wrapping_add(1);
        }
    }
    sum
}

fn keyword(h: &FitsHeader, key: &str, absent: i64) -> i64 {
    match h.int(key) {
        Some(v) => v,
        None => absent,
    }
}

fn data_len(h: &FitsHeader) -> Option<usize> {
    let naxis = keyword(h, "NAXIS", 0).max(0) as usize;
    if naxis == 0 {
        return Some(0);
    }
    let mut n = 1usize;
    for k in 1..=naxis {
        let key = format!("NAXIS{k}");
        let v = keyword(h, &key, 0).max(0) as usize;
        n = n.saturating_mul(v);
    }
    let pcount = keyword(h, "PCOUNT", 0).max(0) as usize;
    let gcount = keyword(h, "GCOUNT", 1).max(0) as usize;
    let bitpix = keyword(h, "BITPIX", 8);
    let bits = if bitpix == 0 {
        8
    } else {
        bitpix.unsigned_abs() as usize
    };
    let bytes = gcount
        .saturating_mul(pcount.saturating_add(n))
        .saturating_mul(bits)
        / 8;
    Some(bytes.div_ceil(2880) * 2880)
}

fn unquote(v: &str) -> String {
    v.trim().trim_matches('\'').trim().to_string()
}

fn check_file(path: &str) -> bool {
    let Some(buf) = std::fs::read(path).ok() else {
        println!("{path}: unreadable");
        return false;
    };
    let mut start = 0usize;
    let mut hdu = 0usize;
    let mut ok = true;
    while start + 2880 <= buf.len() {
        let Some((h, next)) = FitsHeader::parse(&buf, start) else {
            println!("{path}: HDU {hdu}: header unreadable");
            return false;
        };
        let Some(dlen) = data_len(&h) else {
            println!("{path}: HDU {hdu}: data size undetermined");
            return false;
        };
        let end = next + dlen;
        if end > buf.len() {
            println!("{path}: HDU {hdu}: data truncated");
            return false;
        }
        let ext = match h.value("EXTNAME") {
            Some(v) => v,
            None => "-",
        };
        let bitpix = keyword(&h, "BITPIX", 8);
        let naxis = keyword(&h, "NAXIS", 0);
        let checksum = h.value("CHECKSUM");
        let datasum = h.value("DATASUM");
        let mut line = format!(
            "{path}: HDU {hdu} {ext} BITPIX {bitpix} NAXIS {naxis}"
        );
        if let Some(cs) = checksum {
            let total = sum32(&buf[start..end]);
            let pass = total == u32::MAX;
            line.push_str(&format!(
                " CHECKSUM {} -> sum {total:08X} {}",
                unquote(cs),
                if pass { "holds" } else { "breaks" }
            ));
            ok &= pass;
        } else {
            line.push_str(" CHECKSUM absent");
        }
        if let Some(ds) = datasum {
            let data = &buf[next..end];
            let got = sum32(data);
            let want: Option<u32> = unquote(ds).parse().ok();
            match want {
                Some(w) => {
                    let pass = got == w;
                    line.push_str(&format!(
                        " DATASUM {} computed {} {}",
                        w,
                        got,
                        if pass { "holds" } else { "breaks" }
                    ));
                    ok &= pass;
                }
                None => line.push_str(&format!(" DATASUM unparsed ({})", ds.trim())),
            }
        }
        if keyword(&h, "NAXIS", 0) >= 2
            && let Some(img) = FitsImage::parse(&buf, start).map(|(i, _)| i)
        {
            line.push_str(&format!(" image {}x{}", img.dims[0], img.dims[1]));
        }
        println!("{line}");
        hdu += 1;
        start = end;
        if start == buf.len() {
            break;
        }
    }
    ok
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let files = arg_values(&args, "--file");
    if files.is_empty() {
        eprintln!("usage: fits_sample_probe --file <sample.fits> [--file ...]");
        std::process::exit(2);
    }
    let mut ok = true;
    for f in &files {
        ok &= check_file(f);
    }
    std::process::exit(if ok { 0 } else { 1 });
}
