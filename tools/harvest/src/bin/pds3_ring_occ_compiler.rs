use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds3_ring_occ::{
    PACK_ENTRY_BYTES, PACK_RECORD_BYTES, RingOccSample, pack_many, parse_packed, parse_series,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-rings.seti.org";
const SHARD_BUDGET: usize = 1 << 30;
const ROUTES: [&str; 2] = [
    "https://pds-rings.seti.org/holdings/volumes/VG_28xx/VG_2803/S_RINGS/EDITDATA/",
    "https://pds-rings.seti.org/holdings/volumes/VG_28xx/VG_2803/U_RINGS/EDITDATA/",
];

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn hrefs(text: &str) -> Vec<String> {
    let low = text.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(p) = low[from..].find("href=\"") {
        let start = from + p + 6;
        let Some(end) = low[start..].find('"') else {
            break;
        };
        out.push(text[start..start + end].to_string());
        from = start + end;
    }
    out
}

fn odl_kv(text: &str) -> Vec<(String, String)> {
    let mut cleaned = String::with_capacity(text.len());
    let mut rest = text;
    loop {
        let Some(open) = rest.find("/*") else {
            cleaned.push_str(rest);
            break;
        };
        cleaned.push_str(&rest[..open]);
        let Some(close) = rest[open + 2..].find("*/") else {
            break;
        };
        rest = &rest[open + 2 + close + 2..];
    }
    let mut out = Vec::new();
    for line in cleaned.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = value.trim().trim_matches('"').trim().to_string();
        if value.is_empty() {
            continue;
        }
        out.push((key.trim().to_string(), value));
    }
    out
}

#[derive(Clone, Debug)]
struct Column {
    name: String,
    data_type: String,
    start_byte: usize,
    bytes: usize,
}

#[derive(Clone, Debug, Default)]
struct SeriesMeta {
    record_bytes: Option<usize>,
    interchange: Option<String>,
    columns: Vec<Column>,
    param_name: Option<String>,
    param_min: Option<f64>,
    param_interval: Option<f64>,
    direction: Option<String>,
}

fn parse_label(text: &str) -> Option<SeriesMeta> {
    let mut meta = SeriesMeta::default();
    let mut column: Option<Column> = None;
    for (key, value) in odl_kv(text) {
        match key.as_str() {
            "OBJECT" => {
                if value == "COLUMN" {
                    column = Some(Column {
                        name: String::new(),
                        data_type: String::new(),
                        start_byte: 0,
                        bytes: 0,
                    });
                }
            }
            "NAME" => {
                if let Some(c) = column.as_mut() {
                    c.name = value;
                }
            }
            "DATA_TYPE" => {
                if let Some(c) = column.as_mut() {
                    c.data_type = value;
                }
            }
            "START_BYTE" => {
                if let Some(c) = column.as_mut() {
                    c.start_byte = value.parse().ok()?;
                }
            }
            "BYTES" => {
                if let Some(c) = column.as_mut() {
                    c.bytes = value.parse().ok()?;
                }
            }
            "END_OBJECT" => {
                if value == "COLUMN" {
                    if let Some(c) = column.take() {
                        meta.columns.push(c);
                    }
                }
            }
            "RECORD_BYTES" => meta.record_bytes = value.parse().ok(),
            "INTERCHANGE_FORMAT" => meta.interchange = Some(value),
            "SAMPLING_PARAMETER_NAME" => meta.param_name = Some(value),
            "MINIMUM_SAMPLING_PARAMETER" => meta.param_min = value.parse().ok(),
            "SAMPLING_PARAMETER_INTERVAL" => meta.param_interval = value.parse().ok(),
            "RING_OCCULTATION_DIRECTION" => meta.direction = Some(value),
            _ => {}
        }
    }
    Some(meta)
}

fn decode_dat(bytes: &[u8], meta: &SeriesMeta) -> Option<(Vec<RingOccSample>, usize)> {
    if meta.interchange.as_deref() != Some("BINARY") || meta.record_bytes != Some(8) {
        return None;
    }
    if meta.columns.len() != 2 {
        return None;
    }
    let mut cols = meta.columns.clone();
    cols.sort_by_key(|c| c.start_byte);
    if cols[0].name != "SIGNAL_RE"
        || cols[1].name != "SIGNAL_IM"
        || cols[0].data_type != "IEEE_REAL"
        || cols[1].data_type != "IEEE_REAL"
        || cols[0].start_byte != 1
        || cols[1].start_byte != 5
        || cols[0].bytes != 4
        || cols[1].bytes != 4
    {
        return None;
    }
    let Some(param_min) = meta.param_min else {
        return None;
    };
    let Some(param_interval) = meta.param_interval else {
        return None;
    };
    if meta.param_name.as_deref() != Some("NOMINAL_RING_RADIUS")
        || !param_min.is_finite()
        || !param_interval.is_finite()
        || !(param_min >= 0.0)
        || !(param_interval > 0.0)
    {
        return None;
    }
    let complete = bytes.len() / 8;
    let mut samples = Vec::with_capacity(complete);
    let mut skipped = 0usize;
    for i in 0..complete {
        let at = i * 8;
        let re = f32::from_be_bytes(bytes[at..at + 4].try_into().ok()?) as f64;
        let im = f32::from_be_bytes(bytes[at + 4..at + 8].try_into().ok()?) as f64;
        if !re.is_finite() || !im.is_finite() {
            skipped += 1;
            continue;
        }
        samples.push(RingOccSample {
            radius_km: param_min + i as f64 * param_interval,
            signal_re: re,
            signal_im: im,
        });
    }
    if samples.is_empty() {
        return None;
    }
    Some((samples, skipped))
}

fn fetch_or_read(spec: &str) -> Option<Vec<u8>> {
    if spec.starts_with("http://") || spec.starts_with("https://") {
        fetch_raw_bytes(spec)
    } else {
        std::fs::read(spec).ok()
    }
}

fn pack_and_verify(entries: &[(Vec<RingOccSample>, String, String)], out: &str) -> Option<Vec<u8>> {
    let refs: Vec<(&[RingOccSample], &str, &str)> = entries
        .iter()
        .map(|(s, n, d)| (s.as_slice(), n.as_str(), d.as_str()))
        .collect();
    let bin = pack_many(&refs);
    let Some(files) = parse_packed(&bin) else {
        eprintln!("{out}: packed read void — the series stays unverified (0 honored)");
        std::process::exit(1);
    };
    if files.len() != entries.len() {
        eprintln!("{out}: entry count void — the series stays unverified (0 honored)");
        std::process::exit(1);
    }
    for (f, (samples, name, direction)) in files.iter().zip(entries.iter()) {
        if f.name != *name || f.direction != *direction || f.samples != *samples {
            eprintln!("{out}: {name} roundtrip void — the series stays unverified (0 honored)");
            std::process::exit(1);
        }
    }
    if let Some(parent) = std::path::Path::new(out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    Some(bin)
}

fn shard_ranges(data_bytes: &[usize], budget: usize) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut lo = 0usize;
    while lo < data_bytes.len() {
        let mut hi = lo + 1;
        let mut data = data_bytes[lo];
        while hi < data_bytes.len()
            && 8 + (hi + 1 - lo) * PACK_ENTRY_BYTES + data + data_bytes[hi] <= budget
        {
            data += data_bytes[hi];
            hi += 1;
        }
        ranges.push((lo, hi));
        lo = hi;
    }
    ranges
}

fn print_register_line(ord: usize) {
    println!(
        "url https://github.com/omegaflow/sources/releases/download/{NETLOC}/pds3_ring_occ_s{ord}.bin"
    );
    println!("format pds3_ring_occ");
    println!("at earth");
    println!("ttl 604800");
    println!("field signal_re pds3_ring_occ_signal_re inverse-square em count 604800 0.0 0.0");
    println!("field signal_im pds3_ring_occ_signal_im inverse-square em count 604800 0.0 0.0");
    println!();
}

fn compile_entry(
    name: &str,
    label_url: &str,
    dat_url: &str,
) -> Option<(Vec<RingOccSample>, String, String)> {
    let Some(label_bytes) = fetch_or_read(label_url) else {
        eprintln!("{name}: label fetch void ({label_url})");
        return None;
    };
    let Ok(label_text) = std::str::from_utf8(&label_bytes) else {
        eprintln!("{name}: label not utf8");
        return None;
    };
    let Some(meta) = parse_label(label_text) else {
        eprintln!("{name}: label parse void");
        return None;
    };
    let Some(dat_bytes) = fetch_or_read(dat_url) else {
        eprintln!("{name}: data fetch void ({dat_url})");
        return None;
    };
    let Some((samples, skipped)) = decode_dat(&dat_bytes, &meta) else {
        eprintln!(
            "{name}: {} byte(s) carry no measured IEEE_REAL pair series — the file stays unwritten (0 honored)",
            dat_bytes.len()
        );
        return None;
    };
    let trailing = dat_bytes.len() % 8;
    if trailing > 0 {
        eprintln!(
            "{name}: {trailing} trailing byte(s) after {} sample(s)",
            samples.len()
        );
    }
    if skipped > 0 {
        eprintln!("{name}: {skipped} non-finite sample(s) skipped");
    }
    let Some(direction) = meta.direction else {
        eprintln!("{name}: direction absent in the label — the file stays unwritten");
        return None;
    };
    Some((samples, name.to_string(), direction))
}

fn collect_dir(dir_url: &str, entries: &mut Vec<(Vec<RingOccSample>, String, String)>) {
    let Some(bytes) = fetch_raw_bytes(dir_url) else {
        eprintln!("{dir_url}: listing fetch void");
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{dir_url}: listing not utf8");
        return;
    };
    let mut names: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.ends_with(".DAT"))
        .map(|h| {
            h.trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string()
        })
        .collect();
    names.sort();
    names.dedup();
    let base_trim = dir_url.trim_end_matches('/');
    for name in &names {
        let Some(stem) = name.strip_suffix(".DAT") else {
            continue;
        };
        let label_url = format!("{base_trim}/{stem}.LBL");
        let dat_url = format!("{base_trim}/{name}");
        match compile_entry(name, &label_url, &dat_url) {
            Some(entry) => {
                eprintln!("{name}: {} sample(s) read", entry.0.len());
                entries.push(entry);
            }
            None => eprintln!("{name}: entry skipped"),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/pds3_ring_occ.bin"),
    };
    let mut entries: Vec<(Vec<RingOccSample>, String, String)> = Vec::new();
    match (arg_value(&args, "--dat"), arg_value(&args, "--label")) {
        (Some(dat), Some(label)) => {
            let name = dat
                .rsplit('/')
                .next()
                .unwrap_or(&dat)
                .split_once('?')
                .map(|(h, _)| h)
                .unwrap_or(&dat)
                .to_string();
            match compile_entry(&name, &label, &dat) {
                Some(entry) => entries.push(entry),
                None => {
                    eprintln!("{name}: entry skipped — nothing written (0 honored)");
                    std::process::exit(1);
                }
            }
        }
        (Some(_), None) | (None, Some(_)) => {
            eprintln!("--dat <file|url> and --label <file|url> come as a pair");
            std::process::exit(1);
        }
        (None, None) => {
            for route in ROUTES {
                collect_dir(route, &mut entries);
            }
        }
    }
    if entries.is_empty() {
        eprintln!("no series packed — nothing written (0 honored)");
        std::process::exit(1);
    }
    let lengths: Vec<usize> = entries
        .iter()
        .map(|(s, _, _)| s.len() * PACK_RECORD_BYTES)
        .collect();
    let ranges = shard_ranges(&lengths, SHARD_BUDGET);
    let total: usize = entries.iter().map(|(s, _, _)| s.len()).sum();
    if ranges.len() <= 1 {
        let Some(bin) = pack_and_verify(&entries, &out) else {
            std::process::exit(1);
        };
        match parse_series(&bin) {
            Some(series) if series.len() == total * 2 => {
                eprintln!(
                    "{out}: {} file(s) packed ({} samples, {} bytes), sha256 {}, roundtrip holds",
                    entries.len(),
                    total,
                    bin.len(),
                    sha256_hex(&bin)
                );
            }
            _ => {
                eprintln!("{out}: series read void — the bin stays unverified (0 honored)");
                std::process::exit(1);
            }
        }
        if ci_mode && !upload_release(NETLOC, &out) {
            std::process::exit(1);
        }
        return;
    }
    for (ord, &(lo, hi)) in ranges.iter().enumerate() {
        let path = format!("data/{NETLOC}/pds3_ring_occ_s{ord}.bin");
        let Some(bin) = pack_and_verify(&entries[lo..hi], &path) else {
            std::process::exit(1);
        };
        eprintln!(
            "{path}: {} file(s) packed ({} bytes), roundtrip holds",
            hi - lo,
            bin.len()
        );
        if ci_mode && !upload_release(NETLOC, &path) {
            std::process::exit(1);
        }
    }
    for ord in 0..ranges.len() {
        print_register_line(ord);
    }
}
