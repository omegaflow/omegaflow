use omegaflow::archivar::ccor::{parse_bin, write_bin, COMP_INTENSITY};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{rice_decompress, FitsHeader, FitsTable};
use omegaflow::lsk::{days_from_civil, parse as parse_lsk, LeapSeconds};

const NETLOC: &str = "noaa-nesdis-swfo-ccor-1-pds.s3.amazonaws.com";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_unix(s: &str) -> Option<f64> {
    let s = s.trim().trim_matches('\'');
    if s.len() < 19 {
        return None;
    }
    let y: i64 = s.get(0..4)?.parse().ok()?;
    let m: i64 = s.get(5..7)?.parse().ok()?;
    let d: i64 = s.get(8..10)?.parse().ok()?;
    let h: i64 = s.get(11..13)?.parse().ok()?;
    let mi: i64 = s.get(14..16)?.parse().ok()?;
    let sec: f64 = s.get(17..19)?.parse().ok()?;
    Some(days_from_civil(y, m, d)? as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + sec)
}

fn zval(header: &FitsHeader, name: &str) -> Option<f64> {
    for i in 1..=16 {
        let n = header.str_unescaped(&format!("ZNAME{}", i))?;
        if n.trim() == name {
            return header.f64(&format!("ZVAL{}", i));
        }
    }
    None
}

fn decode_tile(raw: &[u8], n_pixels: usize, bytepix: usize, block: usize) -> Option<Vec<i64>> {
    let out = rice_decompress(raw, n_pixels, bytepix, block)?;
    if out.len() != n_pixels * bytepix {
        return None;
    }
    let mut vals = Vec::with_capacity(n_pixels);
    for i in 0..n_pixels {
        let v = match bytepix {
            1 => out[i] as i8 as i64,
            2 => i16::from_be_bytes(out[i * 2..i * 2 + 2].try_into().ok()?) as i64,
            4 => i32::from_be_bytes(out[i * 4..i * 4 + 4].try_into().ok()?) as i64,
            8 => i64::from_be_bytes(out[i * 8..i * 8 + 8].try_into().ok()?),
            _ => return None,
        };
        vals.push(v);
    }
    Some(vals)
}

fn frame_mean(bytes: &[u8], lsk: &LeapSeconds) -> Option<(f64, f64)> {
    let (_, next0) = FitsHeader::parse(bytes, 0)?;
    let (h1, _) = FitsHeader::parse(bytes, next0)?;
    if h1.value("ZIMAGE") != Some("T") {
        return None;
    }
    if h1.str_unescaped("ZCMPTYPE").as_deref().map(str::trim) != Some("RICE_1") {
        return None;
    }
    let znaxis = h1.int("ZNAXIS")? as usize;
    if znaxis == 0 || znaxis > 3 {
        return None;
    }
    let mut dims = [1usize; 3];
    let mut tile = [1usize; 3];
    for i in 1..=znaxis {
        let d = h1.int(&format!("ZNAXIS{}", i))? as usize;
        let t = h1.int(&format!("ZTILE{}", i)).unwrap_or(d as i64) as usize;
        if t == 0 {
            return None;
        }
        tile[i - 1] = t;
        dims[i - 1] = d;
    }
    let block = zval(&h1, "BLOCKSIZE").unwrap_or(32.0) as usize;
    let bytepix = zval(&h1, "BYTEPIX").unwrap_or(4.0) as usize;
    let (table, _) = FitsTable::parse(bytes, next0)?;
    let compressed = table.column("COMPRESSED_DATA")?;
    let zscale_col = table.column("ZSCALE");
    let zzero_col = table.column("ZZERO");
    let tiles = dims[0].div_ceil(tile[0]) * dims[1].div_ceil(tile[1]) * dims[2].div_ceil(tile[2]);
    let n_pixels = tile[0] * tile[1] * tile[2];
    let null_sentinel = i32::MIN as i64;
    let mut sum = 0.0f64;
    let mut n = 0usize;
    for row in 0..tiles {
        let Some(raw) = table.cell_varlen(bytes, row, compressed) else {
            continue;
        };
        let zscale = zscale_col
            .and_then(|c| table.cell_f64(bytes, row, c))
            .unwrap_or(1.0);
        let zzero = match zzero_col.and_then(|c| table.cell_f64(bytes, row, c)) {
            Some(v) => v,
            None => 0.0,
        };
        if let Some(vals) = decode_tile(raw, n_pixels, bytepix, block) {
            for v in vals {
                if v == null_sentinel {
                    continue;
                }
                let v = v as f64 * zscale + zzero;
                if v.is_finite() {
                    sum += v;
                    n += 1;
                }
            }
        }
    }
    if n == 0 {
        return None;
    }
    let date = h1
        .str_unescaped("DATE-AVG")
        .or_else(|| h1.str_unescaped("DATE-BEG"))
        .or_else(|| h1.str_unescaped("DATE-OBS"))?;
    let unix = iso_unix(&date)?;
    let tdb = lsk.unix_to_tdb(unix)?;
    Some((tdb, sum / n as f64))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => {
            eprintln!("--url (CCOR-1 .fits granule) required");
            std::process::exit(1);
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let lsk = match arg_value(&args, "--lsk")
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| parse_lsk(&t))
    {
        Some(l) => l,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void");
            std::process::exit(1);
        }
    };
    let bytes = match fetch_raw_bytes(&url, 3600) {
        Some(b) => b,
        None => {
            eprintln!("{url}: fetch void");
            std::process::exit(1);
        }
    };
    let (tdb, mean) = match frame_mean(&bytes, &lsk) {
        Some(v) => v,
        None => {
            eprintln!("{url}: compressed intensity frame reads void — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let records = vec![(tdb, mean, COMP_INTENSITY)];
    let bin = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bin) {
        Some(parsed) => eprintln!(
            "{url}: {} frame record(s), mean intensity {mean:.6e} DN, roundtrip parses",
            parsed.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
