use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use omegaflow::archivar::geo::{
    pack_iaga, parse_bin, smg_record_at, smg_record_bytes, write_bin, GeoRec, COMP_SMG_E_GEO,
    COMP_SMG_E_NEZ, COMP_SMG_N_GEO, COMP_SMG_N_NEZ, COMP_SMG_Z_GEO, COMP_SMG_Z_NEZ, MAGIC_SMG,
    SMG_REC_BYTES,
};
use omegaflow::archivar::{fetch_raw, jpath, parse_json, JsonVal};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{parse as parse_lsk, LeapSeconds};

const NETLOC: &str = "supermag.jhuapl.edu";
const DATA_API: &str = "https://supermag.jhuapl.edu/services/data-api.php";
const MAGSTID: &str = "https://supermag.jhuapl.edu/lib/php/magstid.php";
const LOGON: &str = "omegaflow";
const FILL_NT: f64 = 999999.0;
const CHUNK_S: f64 = 2419200.0;
const DAY: f64 = 86400.0;

struct StationPos {
    code: String,
    lat: f64,
    lon: f64,
}

#[derive(Clone, Copy, PartialEq)]
struct Key {
    t: f64,
    comp: u32,
    station: u32,
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t
            .total_cmp(&other.t)
            .then_with(|| self.comp.cmp(&other.comp))
            .then_with(|| self.station.cmp(&other.station))
    }
}

fn key_of(r: &GeoRec) -> Key {
    Key {
        t: r.t,
        comp: r.comp,
        station: r.station,
    }
}

struct PartReader {
    inner: std::io::BufReader<std::fs::File>,
    current: GeoRec,
}

impl PartReader {
    fn open(path: &Path) -> std::io::Result<Option<Self>> {
        let file = std::fs::File::open(path)?;
        let mut inner = std::io::BufReader::new(file);
        let mut buf = [0u8; SMG_REC_BYTES];
        match inner.read_exact(&mut buf) {
            Ok(_) => {
                let current = smg_record_at(&buf, 0).ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "short part record")
                })?;
                Ok(Some(Self { inner, current }))
            }
            Err(_) => Ok(None),
        }
    }

    fn advance(&mut self) -> bool {
        let mut buf = [0u8; SMG_REC_BYTES];
        match self.inner.read_exact(&mut buf) {
            Ok(_) => match smg_record_at(&buf, 0) {
                Some(r) => {
                    self.current = r;
                    true
                }
                None => false,
            },
            Err(_) => false,
        }
    }
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else {
        s.split_once(' ')?
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let ss: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64)
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / DAY).floor() as i64;
    let day_secs = unix.max(0.0) - total as f64 * DAY;
    let z = total + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let hh = (day_secs / 3600.0) as i64;
    let mm = ((day_secs - hh as f64 * 3600.0) / 60.0) as i64;
    let ss = (day_secs - hh as f64 * 3600.0 - mm as f64 * 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn num_after(s: &str, key: &str) -> Option<f64> {
    let i = s.find(key)? + key.len();
    let tail = &s[i..];
    let j = tail.find(|c: char| c == ',' || c == '}' || c == '\n')?;
    tail[..j].trim().parse().ok()
}

fn all_stations() -> Option<Vec<StationPos>> {
    let text = fetch_raw(MAGSTID, None, &[], 3600)?;
    let mut out = Vec::new();
    let mut rest: &str = &text;
    loop {
        let Some(start) = rest.find("id:\"") else {
            break;
        };
        let after = &rest[start + 4..];
        let Some(end) = after.find('"') else { break };
        let code = &after[..end];
        let region = &rest[start..];
        rest = &after[end..];
        if code.len() != 3
            || !code
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            continue;
        }
        let Some(lat) = num_after(region, "geolat:") else {
            continue;
        };
        let Some(lon_raw) = num_after(region, "geolon:") else {
            continue;
        };
        let lon = if lon_raw > 180.0 {
            lon_raw - 360.0
        } else {
            lon_raw
        };
        if !(lat.is_finite() && lon.is_finite()) {
            continue;
        }
        out.push(StationPos {
            code: code.to_string(),
            lat,
            lon,
        });
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn component_value(record: &JsonVal, path: &str) -> Option<f64> {
    match jpath(record, path) {
        Some(x) if x.is_finite() && (x - FILL_NT).abs() > 1.0 => Some(x),
        _ => None,
    }
}

fn fetch_chunk(start: &str, extent: f64, station: &str) -> Option<Vec<JsonVal>> {
    let url = format!(
        "{DATA_API}?fmt=json&logon={LOGON}&start={start}&extent={extent:.0}&all&station={station}"
    );
    for attempt in 0..3 {
        if attempt > 0 {
            std::thread::sleep(std::time::Duration::from_secs(15));
        }
        let Some(text) = fetch_raw(&url, None, &[], 600) else {
            continue;
        };
        let Some(JsonVal::Arr(records)) = parse_json(&text) else {
            return None;
        };
        if records.is_empty() {
            return None;
        }
        return Some(records);
    }
    None
}

fn harvest_station(pos: &StationPos, start_unix: f64, days: f64, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let Some(station) = pack_iaga(&pos.code) else {
        return Vec::new();
    };
    let mut out: Vec<GeoRec> = Vec::new();
    let mut cursor = start_unix;
    let end = start_unix + days * DAY;
    while cursor < end {
        let extent = (end - cursor).min(CHUNK_S);
        let chunk_start = iso_utc(cursor);
        let Some(records) = fetch_chunk(&chunk_start, extent, &pos.code) else {
            eprintln!(
                "{} {chunk_start}: chunk void — no records flow from the API",
                pos.code
            );
            cursor += extent;
            continue;
        };
        for rec in &records {
            let Some(tval) = jpath(rec, "tval") else {
                continue;
            };
            if !tval.is_finite() {
                continue;
            }
            let Some(bin_width) = jpath(rec, "ext") else {
                continue;
            };
            if !(bin_width > 0.0) || !bin_width.is_finite() {
                continue;
            }
            let Some(tdb) = lsk.unix_to_tdb(tval) else {
                continue;
            };
            let components = [
                (COMP_SMG_N_NEZ, "N.nez"),
                (COMP_SMG_E_NEZ, "E.nez"),
                (COMP_SMG_Z_NEZ, "Z.nez"),
                (COMP_SMG_N_GEO, "N.geo"),
                (COMP_SMG_E_GEO, "E.geo"),
                (COMP_SMG_Z_GEO, "Z.geo"),
            ];
            for (comp, path) in components {
                let Some(val) = component_value(rec, path) else {
                    continue;
                };
                out.push(GeoRec {
                    t: tdb,
                    lat: pos.lat,
                    lon: pos.lon,
                    alt: 0.0,
                    freq: 0.0,
                    bin_width,
                    val,
                    comp,
                    station,
                });
            }
        }
        cursor += extent;
    }
    out.sort_by(|a, b| a.t.total_cmp(&b.t).then_with(|| a.comp.cmp(&b.comp)));
    out
}

fn write_part(path: &Path, records: &[GeoRec]) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(records.len() * SMG_REC_BYTES);
    for r in records {
        buf.extend_from_slice(&smg_record_bytes(r));
    }
    std::fs::write(path, &buf)
}

fn merge_parts(parts: &[std::path::PathBuf], out: &str) -> std::io::Result<u64> {
    let mut readers: Vec<PartReader> = Vec::new();
    for p in parts {
        if let Some(r) = PartReader::open(p)? {
            readers.push(r);
        }
    }
    let mut heap = BinaryHeap::new();
    for (i, r) in readers.iter().enumerate() {
        heap.push(Reverse((key_of(&r.current), i)));
    }
    let mut f = std::fs::File::create(out)?;
    f.write_all(&MAGIC_SMG)?;
    f.write_all(&0u32.to_le_bytes())?;
    let mut n: u64 = 0;
    let mut buf = Vec::with_capacity(SMG_REC_BYTES * 4096);
    while let Some(Reverse((_, i))) = heap.pop() {
        buf.extend_from_slice(&smg_record_bytes(&readers[i].current));
        n += 1;
        if readers[i].advance() {
            heap.push(Reverse((key_of(&readers[i].current), i)));
        }
        if buf.len() >= SMG_REC_BYTES * 4096 {
            f.write_all(&buf)?;
            buf.clear();
        }
    }
    f.write_all(&buf)?;
    f.flush()?;
    f.seek(SeekFrom::Start(4))?;
    f.write_all(&(n as u32).to_le_bytes())?;
    f.flush()?;
    Ok(n)
}

fn verify_stream(out: &str, expected: u64) -> bool {
    let Ok(mut f) = std::fs::File::open(out) else {
        return false;
    };
    let mut header = [0u8; 8];
    if f.read_exact(&mut header).is_err() || header[0..4] != MAGIC_SMG {
        return false;
    }
    let n = u32::from_le_bytes(header[4..8].try_into().unwrap()) as u64;
    if n != expected {
        return false;
    }
    let mut prev: Option<Key> = None;
    let mut buf = [0u8; SMG_REC_BYTES];
    for _ in 0..n {
        if f.read_exact(&mut buf).is_err() {
            return false;
        }
        let Some(r) = smg_record_at(&buf, 0) else {
            return false;
        };
        let k = key_of(&r);
        if let Some(p) = prev {
            if p > k {
                return false;
            }
        }
        prev = Some(k);
    }
    let mut tail = [0u8; 1];
    f.read_exact(&mut tail).is_err()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let all = args.iter().any(|a| a == "--all");
    let stations_file = arg_value(&args, "--stations");
    let single = arg_value(&args, "--station");

    if args.iter().any(|a| a == "--list-stations") {
        let Some(mut positions) = all_stations() else {
            eprintln!("magstid.php carries no station list — nothing to list");
            std::process::exit(1);
        };
        positions.sort_by(|a, b| a.code.cmp(&b.code));
        for p in &positions {
            println!("{}", p.code);
        }
        return;
    }

    let start = match arg_value(&args, "--start") {
        Some(v) => v,
        None => {
            eprintln!("--start (ISO, e.g. 2025-03-01T00:00:00) required");
            std::process::exit(1);
        }
    };
    let start_unix = match iso_to_unix(&start) {
        Some(u) => u,
        None => {
            eprintln!("--start {start} parses void");
            std::process::exit(1);
        }
    };
    let days: f64 = arg_value(&args, "--days")
        .and_then(|v| v.parse().ok())
        .unwrap_or(28.0);
    if !(days > 0.0) || !days.is_finite() {
        eprintln!("--days {days} carries no positive window");
        std::process::exit(1);
    }
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
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };

    let positions = match all_stations() {
        Some(p) => p,
        None => {
            eprintln!("magstid.php carries no station list — nothing to harvest");
            std::process::exit(1);
        }
    };

    let mut chosen: Vec<&StationPos> = Vec::new();
    if all {
        chosen = positions.iter().collect();
    } else if let Some(path) = stations_file {
        let Ok(text) = std::fs::read_to_string(&path) else {
            eprintln!("--stations {path} unreadable");
            std::process::exit(1);
        };
        for line in text.lines() {
            let code = line.trim();
            if code.is_empty() || code.starts_with('#') {
                continue;
            }
            match positions.iter().find(|p| p.code == code) {
                Some(p) => chosen.push(p),
                None => eprintln!("{code}: magstid.php carries no position — station skipped"),
            }
        }
    } else if let Some(code) = single {
        match positions.iter().find(|p| p.code == code) {
            Some(p) => chosen.push(p),
            None => {
                eprintln!("{code}: magstid.php carries no position — nothing to harvest");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("--station <IAGA>, --stations <file>, or --all required");
        std::process::exit(1);
    }

    if chosen.is_empty() {
        eprintln!("no stations to harvest — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    if chosen.len() == 1 {
        let records = harvest_station(chosen[0], start_unix, days, &lsk);
        if records.is_empty() {
            eprintln!(
                "{}: no measured nT records — the bin stays unwritten (0 honored)",
                chosen[0].code
            );
            std::process::exit(1);
        }
        let bytes = write_bin(MAGIC_SMG, &records);
        if std::fs::write(&out, &bytes).is_err() {
            eprintln!("write {out} returned void");
            std::process::exit(1);
        }
        match parse_bin(MAGIC_SMG, &bytes) {
            Some(parsed) => eprintln!(
                "{}: {} geo records ({:.0} d window, {} B) written, roundtrip parses",
                chosen[0].code,
                parsed.len(),
                days,
                bytes.len()
            ),
            None => {
                eprintln!("{out}: roundtrip parse void — the bin stays unverified");
                std::process::exit(1);
            }
        }
    } else {
        let mut parts: Vec<std::path::PathBuf> = Vec::new();
        let mut station_count = 0usize;
        for (i, pos) in chosen.iter().enumerate() {
            let records = harvest_station(pos, start_unix, days, &lsk);
            if records.is_empty() {
                eprintln!("{}: no measured nT records — station skipped", pos.code);
                continue;
            }
            let part = format!("{out}.part.{i}");
            if let Err(e) = write_part(std::path::Path::new(&part), &records) {
                eprintln!("write {part} returned void: {e}");
                std::process::exit(1);
            }
            parts.push(std::path::PathBuf::from(part));
            station_count += 1;
        }
        if parts.is_empty() {
            eprintln!(
                "no station carried measured nT records — the bin stays unwritten (0 honored)"
            );
            std::process::exit(1);
        }
        let n = match merge_parts(&parts, &out) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("merge {out} returned void: {e}");
                std::process::exit(1);
            }
        };
        for p in &parts {
            let _ = std::fs::remove_file(p);
        }
        if !verify_stream(&out, n) {
            eprintln!("{out}: stream verify void — the bin stays unverified");
            std::process::exit(1);
        }
        let bytes = 8 + n * SMG_REC_BYTES as u64;
        eprintln!(
            "{station_count} stations: {n} geo records ({:.0} d window, {bytes} B) merged, stream verify holds",
            days
        );
    }

    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
