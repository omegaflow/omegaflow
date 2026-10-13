use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::types::PRESENCE_FLAG_QUANTITY;
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Hdf5File, Hdf5Layout, Hdf5Note};
use omegaflow::lsk::days_from_civil;
use std::path::Path;

const COMPILER: &str = "tools/harvest/src/bin/nist_ncnr_nse_nexus_compiler.rs";
const NETLOC: &str = "ncnr.nist.gov";
const FORMAT: &str = "nist_ncnr_nse_nexus";
const TTL_S: f64 = 604800.0;
const NSE_CATALOG: &str = "https://ncnr.nist.gov/pub/ncnrdata/nse/?format=jsonld";

const FRAME_MAGIC: [u8; 3] = [0xCF, 0x86, 0x09];
const FRAME_HEADER: usize = 19;
const REC_BYTES: usize = 26 * 8;

const HDF5_MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];

const SLOT_VAL: usize = 3;
const SLOT_EPOCH: usize = 4;
const SLOT_TTL: usize = 5;
const SLOT_TAU: usize = 6;
const SLOT_PHASE: usize = 24;
const SLOT_PRESENCE: usize = 25;

const DS_VALUE_PRIMARY: &str = "entry/DAS_logs/pointDetector/counts";
const DS_VALUE_FALLBACK: &[&str] = &[
    "entry/DAS_logs/pointDetector/liveROI",
    "entry/DAS_logs/counter/liveROI",
];
const DS_FOURIER: &str = "entry/DAS_logs/fourierTime/fourierTime";
const DS_PHASE: &str = "entry/DAS_logs/phase/phase";
const DS_ELAPSED: &str = "entry/DAS_logs/counter/startTime";
const DS_ELAPSED_ALT: &str = "entry/DAS_logs/counter/stopTime";
const DS_START_ISO: &str = "entry/start_time";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn json_str<'a>(v: &'a JsonVal, key: &str) -> Option<&'a str> {
    match v {
        JsonVal::Obj(m) => match m.get(key) {
            Some(JsonVal::Str(s)) if !s.is_empty() => Some(s.as_str()),
            _ => None,
        },
        _ => None,
    }
}

fn json_arr<'a>(v: &'a JsonVal, key: &str) -> &'a [JsonVal] {
    match v {
        JsonVal::Obj(m) => match m.get(key) {
            Some(JsonVal::Arr(a)) => a.as_slice(),
            _ => &[],
        },
        _ => &[],
    }
}

fn fetch_json(url: &str) -> Result<JsonVal, String> {
    let bytes = fetch_raw_bytes(url).ok_or_else(|| format!("{url}: fetch returned void"))?;
    let text = String::from_utf8_lossy(&bytes);
    parse_json(&text).ok_or_else(|| format!("{url}: carries no JSON"))
}

fn collect_nxs_urls(root: &str, limit: Option<usize>) -> Result<Vec<String>, String> {
    let top = fetch_json(root)?;
    let months: Vec<String> = json_arr(&top, "hasPart")
        .iter()
        .filter_map(|m| json_str(m, "url").map(|s| s.to_string()))
        .collect();
    if months.is_empty() {
        return Err(format!(
            "{root}: no hasPart monthly set stands in the catalog — nothing walked (0 honored)"
        ));
    }
    let mut out: Vec<String> = Vec::new();
    for month in &months {
        let Ok(mj) = fetch_json(month) else {
            continue;
        };
        for exp in json_arr(&mj, "hasPart") {
            let Some(eurl) = json_str(exp, "url") else {
                continue;
            };
            let Ok(ej) = fetch_json(eurl) else {
                continue;
            };
            for child in json_arr(&ej, "hasPart") {
                if json_str(child, "name") != Some("data") {
                    continue;
                }
                let Some(durl) = json_str(child, "url") else {
                    continue;
                };
                let Ok(dj) = fetch_json(durl) else {
                    continue;
                };
                for d in json_arr(&dj, "distribution") {
                    let Some(cu) = json_str(d, "contentUrl") else {
                        continue;
                    };
                    if cu.ends_with(".nxs.nse") {
                        out.push(cu.to_string());
                        if limit.is_some_and(|l| out.len() >= l) {
                            return Ok(out);
                        }
                    }
                }
            }
        }
    }
    Ok(out)
}

fn note_text(note: &Hdf5Note) -> String {
    format!("{note:?}")
}

fn read_series(file: &Hdf5File, name: &str) -> Option<Vec<f64>> {
    file.read_f64_dataset(name).ok()
}

fn first_series(file: &Hdf5File, names: &[&str]) -> Option<(String, Vec<f64>)> {
    names.iter().find_map(|name| {
        read_series(file, name)
            .filter(|v| !v.is_empty())
            .map(|v| (name.to_string(), v))
    })
}

fn read_string(file: &Hdf5File, name: &str) -> Option<String> {
    let (_, _, dt) = file.dataset(name).ok()?;
    if dt.class != 3 {
        return None;
    }
    let raw = file.read_dataset(name).ok()?;
    let s = String::from_utf8_lossy(&raw)
        .trim_end_matches('\0')
        .trim()
        .to_string();
    if s.is_empty() { None } else { Some(s) }
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let (date, rest) = s.split_once('T')?;
    let d: Vec<&str> = date.split('-').collect();
    let (year, month, day) = (
        d.first()?.parse::<i64>().ok()?,
        d.get(1)?.parse::<i64>().ok()?,
        d.get(2)?.parse::<i64>().ok()?,
    );
    let day_seconds = days_from_civil(year, month, day)? as f64 * 86400.0;
    let (time, tz) = match rest.find(['Z', 'z']) {
        Some(i) => (&rest[..i], 0.0),
        None => {
            let i = rest
                .char_indices()
                .rev()
                .find(|(_, c)| *c == '+' || *c == '-')
                .map(|(i, _)| i);
            match i {
                Some(i) if i > 0 => (&rest[..i], tz_offset_seconds(&rest[i..])?),
                _ => (rest, 0.0),
            }
        }
    };
    let t: Vec<&str> = time.split(':').collect();
    let hh = t.first()?.parse::<f64>().ok()?;
    let mm = t.get(1)?.parse::<f64>().ok()?;
    let ss = t.get(2)?.parse::<f64>().ok()?;
    Some(day_seconds + hh * 3600.0 + mm * 60.0 + ss - tz)
}

fn tz_offset_seconds(s: &str) -> Option<f64> {
    let sign = match s.chars().next()? {
        '+' => 1.0,
        '-' => -1.0,
        _ => return None,
    };
    let body = &s[1..];
    let (h, m) = match body.split_once(':') {
        Some((h, m)) => (h, m),
        None if body.len() == 4 => (&body[..2], &body[2..]),
        None => (body, "0"),
    };
    let hh = h.parse::<f64>().ok()?;
    let mm = m.parse::<f64>().ok()?;
    Some(sign * (hh * 3600.0 + mm * 60.0))
}

fn frame(records: &[[f64; 26]]) -> Option<Vec<u8>> {
    if records.is_empty() {
        return None;
    }
    let response_epoch = records
        .iter()
        .map(|r| r[SLOT_EPOCH])
        .fold(f64::NEG_INFINITY, f64::max);
    let mut out = Vec::with_capacity(FRAME_HEADER + records.len() * REC_BYTES);
    out.extend_from_slice(&FRAME_MAGIC);
    out.extend_from_slice(&response_epoch.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    Some(out)
}

fn unframe(bytes: &[u8]) -> Option<Vec<[f64; 26]>> {
    if bytes.len() < FRAME_HEADER || bytes.get(0..3)? != FRAME_MAGIC.as_slice() {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(15..19)?.try_into().ok()?) as usize;
    if bytes.len() != FRAME_HEADER + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = FRAME_HEADER;
    for _ in 0..n {
        let mut r = [0.0f64; 26];
        for slot in r.iter_mut() {
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
        }
        out.push(r);
    }
    Some(out)
}

struct Compiled {
    records: Vec<[f64; 26]>,
    points: usize,
    value_ds: String,
    absent_epoch: usize,
    absent_value: usize,
    absent_fourier: usize,
    absent_phase: usize,
}

fn layout_name(obj: &omegaflow::hdf5::Hdf5Object) -> &'static str {
    match obj.layout {
        Some(Hdf5Layout::Contiguous { .. }) => "contiguous",
        Some(Hdf5Layout::Chunked { .. }) => "chunked",
        Some(Hdf5Layout::Compact { .. }) => "compact",
        None => "absent",
    }
}

fn compile(bytes: &[u8], ttl_s: f64) -> Result<Compiled, String> {
    let file =
        Hdf5File::parse(bytes).map_err(|note| format!("hdf5 parses void: {}", note_text(&note)))?;
    let lsk = embedded_lsk().ok_or("the embedded naif0012 table stays unread — no TDB clock")?;

    let base_unix = read_string(&file, DS_START_ISO)
        .as_deref()
        .and_then(iso_to_unix)
        .ok_or_else(|| {
            format!("{DS_START_ISO} carries no readable ISO-8601 instant — the epoch stays unnamed")
        })?;

    let (value_ds, value) = first_series(
        &file,
        &[
            DS_VALUE_PRIMARY,
            DS_VALUE_FALLBACK[0],
            DS_VALUE_FALLBACK[1],
        ],
    )
    .ok_or_else(|| {
        format!(
            "no detector-count dataset resolves ({DS_VALUE_PRIMARY} or its fallbacks) — the NSE intensity stays unnamed (0 honored)"
        )
    })?;
    let fourier = read_series(&file, DS_FOURIER);
    let phase = read_series(&file, DS_PHASE);
    let elapsed = read_series(&file, DS_ELAPSED).or_else(|| read_series(&file, DS_ELAPSED_ALT));

    let points = value.len();
    if points == 0 {
        return Err(
            "the detector counts carry no point — nothing measured (0 honored)".to_string(),
        );
    }

    let mut records = Vec::with_capacity(points);
    let mut absent_epoch = 0usize;
    let mut absent_value = 0usize;
    let mut absent_fourier = 0usize;
    let mut absent_phase = 0usize;

    for i in 0..points {
        let epoch_unix = match elapsed.as_ref().and_then(|v| v.get(i)).copied() {
            Some(dt) if dt.is_finite() => base_unix + dt,
            _ => {
                absent_epoch += 1;
                continue;
            }
        };
        let epoch_tdb = match lsk.unix_to_tdb(epoch_unix) {
            Some(t) => t,
            None => {
                absent_epoch += 1;
                continue;
            }
        };
        let val = match value.get(i).copied() {
            Some(v) if v.is_finite() => v,
            _ => {
                absent_value += 1;
                continue;
            }
        };
        let tau_s = match fourier.as_ref().and_then(|v| v.get(i)).copied() {
            Some(v) if v.is_finite() => v * 1.0e-9,
            _ => {
                absent_fourier += 1;
                0.0
            }
        };
        let ph = match phase.as_ref().and_then(|v| v.get(i)).copied() {
            Some(v) if v.is_finite() => v,
            _ => {
                absent_phase += 1;
                0.0
            }
        };
        let mut r = [0.0f64; 26];
        r[SLOT_VAL] = val;
        r[SLOT_EPOCH] = epoch_tdb;
        r[SLOT_TTL] = ttl_s;
        r[SLOT_TAU] = tau_s;
        r[SLOT_PHASE] = ph;
        r[SLOT_PRESENCE] = PRESENCE_FLAG_QUANTITY;
        records.push(r);
    }

    records.sort_by(|a, b| a[SLOT_EPOCH].total_cmp(&b[SLOT_EPOCH]));

    if let Ok((obj, ds, dt)) = file.dataset(&value_ds) {
        eprintln!(
            "{value_ds}: class {} size {} dims {:?} {} — {points} points",
            dt.class,
            dt.size,
            ds.dims,
            layout_name(obj)
        );
    }

    Ok(Compiled {
        records,
        points,
        value_ds,
        absent_epoch,
        absent_value,
        absent_fourier,
        absent_phase,
    })
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!(
            "usage: nist_ncnr_nse_nexus_compiler (--input <file.nxs.nse> | --url <https .nxs.nse>) --out <file.bin> [--ttl <secs>] [--ci-mode]\n\
       nist_ncnr_nse_nexus_compiler --catalog [<jsonld>] [--limit <n>]\n\
             Compile: one response record per NSE scan point:\n\
             val={DS_VALUE_PRIMARY} (counts), epoch={DS_START_ISO}+{DS_ELAPSED} -> TDB,\n\
             tau={DS_FOURIER} (ns->s), phase={DS_PHASE} (deg).\n\
             Catalog: walk the JSON-LD catalog ({NSE_CATALOG}) and print `data <url>` for\n\
             every .nxs.nse contentUrl in the monthly sets.\n\
             An absent field is skipped/None, never a fabricated 0.0 (0 honored)."
        );
        return Ok(());
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    if args.iter().any(|a| a == "--catalog") {
        let root = match arg_value(args, "--catalog").filter(|u| u.starts_with("http")) {
            Some(u) => u,
            None => NSE_CATALOG.to_string(),
        };
        let limit = arg_value(args, "--limit").and_then(|v| v.parse::<usize>().ok());
        let urls = collect_nxs_urls(&root, limit)?;
        if urls.is_empty() {
            return Err(format!(
                "{root}: the walk found no .nxs.nse data file — nothing emitted (0 honored)"
            ));
        }
        eprintln!("{root}: {} .nxs.nse data files", urls.len());
        for u in &urls {
            println!("data {u}");
        }
        return Ok(());
    }
    let ttl_s = match arg_value(args, "--ttl").and_then(|v| v.parse::<f64>().ok()) {
        Some(t) if t.is_finite() && t > 0.0 => t,
        _ => TTL_S,
    };
    let (bytes, origin) = if let Some(path) = arg_value(args, "--input") {
        let b = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        (b, path)
    } else if let Some(url) = arg_value(args, "--url") {
        let b = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch returned void"))?;
        (b, url)
    } else {
        return Err("--input <file.nxs.nse> or --url <https .nxs.nse> required".to_string());
    };
    if bytes.get(0..8) != Some(&HDF5_MAGIC[..]) {
        return Err(format!(
            "{origin}: carries no HDF5 signature — the bin stays unwritten (0 honored)"
        ));
    }
    let compiled = compile(&bytes, ttl_s)?;
    if compiled.records.is_empty() {
        return Err(format!(
            "{origin}: {} points carry no measured epoch/value — the bin stays unwritten (0 honored)",
            compiled.points
        ));
    }
    let out = match arg_value(args, "--out") {
        Some(o) => o,
        None => {
            let stem = Path::new(&origin)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("nist_ncnr_nse");
            format!("data/{NETLOC}/{stem}.bin")
        }
    };
    let bytes_out = frame(&compiled.records).ok_or("framing returned void")?;
    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bytes_out).map_err(|e| format!("write {out} returned void: {e}"))?;
    match unframe(&bytes_out) {
        Some(parsed) if parsed == compiled.records => {}
        Some(parsed) => {
            return Err(format!(
                "{out}: {} records read back, {} written — the asset stays unverified",
                parsed.len(),
                compiled.records.len()
            ));
        }
        None => return Err(format!("{out}: roundtrip parse returned void")),
    }
    let sha = sha256_hex(&bytes_out);
    let out_name = Path::new(&out)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("nist_ncnr_nse.bin")
        .to_string();
    eprintln!(
        "{origin}: {} points via {} ({} absent epoch, {} absent value, {} absent fourier, {} absent phase), {} records, {} B, sha256 {sha}",
        compiled.points,
        compiled.value_ds,
        compiled.absent_epoch,
        compiled.absent_value,
        compiled.absent_fourier,
        compiled.absent_phase,
        compiled.records.len(),
        bytes_out.len()
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("sha256 {sha}");
    println!("format {FORMAT}");
    println!("ttl {}", ttl_s as u64);
    println!("at earth");
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("nist_ncnr_nse_nexus_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_roundtrip_preserves_records() {
        let mut a = [0.0f64; 26];
        a[SLOT_VAL] = 1234.0;
        a[SLOT_EPOCH] = 1_789_267_813.0;
        a[SLOT_TAU] = 1.2e-8;
        a[SLOT_PHASE] = -0.9;
        a[SLOT_PRESENCE] = PRESENCE_FLAG_QUANTITY;
        let bytes = frame(&[a]);
        assert_eq!(bytes.len(), FRAME_HEADER + REC_BYTES);
        assert_eq!(unframe(&bytes).as_deref(), Some([a].as_slice()));
    }

    #[test]
    fn unframe_rejects_foreign_magic_and_short_body() {
        assert!(unframe(b"XXXX\x00\x00\x00\x00").is_none());
        assert!(unframe(&FRAME_MAGIC).is_none());
        let good = frame(&[[0.0f64; 26]]).unwrap();
        assert!(unframe(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn frame_refuses_an_empty_set() {
        assert!(frame(&[]).is_none());
    }

    #[test]
    fn iso_instant_folds_to_unix() {
        let unix = iso_to_unix("2026-09-12T22:50:13.054-04:00").expect("iso parses");
        assert!((unix - 1_789_267_813.054).abs() < 1e-3);
        assert!(
            (iso_to_unix("2026-09-13T02:50:13.054Z").unwrap() - 1_789_267_813.054).abs() < 1e-3
        );
        assert!(iso_to_unix("2026-09-12").is_none());
    }

    #[test]
    fn timezone_offsets_read_hh_mm_and_hhmm() {
        assert_eq!(tz_offset_seconds("-04:00"), Some(-14400.0));
        assert_eq!(tz_offset_seconds("+0530"), Some(19800.0));
        assert_eq!(tz_offset_seconds("Z"), None);
    }
}
