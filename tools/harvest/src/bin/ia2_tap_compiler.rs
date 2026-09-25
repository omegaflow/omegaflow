use omegaflow::archivar::{fetch_raw_bytes, ia2_tap};
use omegaflow::cdn::upload_release;
use omegaflow::skymap::{
    HEADER_LEN, REC_BYTES, SkymapRecord, decode_rec, encode_rec, parse_header, write_header,
};

const NETLOC: &str = "ia2-tap.oats.inaf.it";
const BASE: &str =
    "http://ia2-tap.oats.inaf.it:8080/wgetap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=";
const SRC_QUERY: &str =
    "SELECT+TOP+5000+ra,dec_,psfMag_r,psfMag_g+FROM+wgesdss.laurino2011+WHERE+ra+IS+NOT+NULL";
const DEFAULT_OUT: &str = "data/ia2-tap.oats.inaf.it/ia2_wgesdss.bin";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn write_sky1(records: &[SkymapRecord]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + records.len() * REC_BYTES);
    write_header(&mut out, records.len() as u64);
    let mut rec = [0u8; REC_BYTES];
    for r in records {
        encode_rec(&mut rec, r);
        out.extend_from_slice(&rec);
    }
    out
}

fn read_sky1(data: &[u8]) -> Option<Vec<SkymapRecord>> {
    let n = parse_header(data)? as usize;
    if data.len() != HEADER_LEN + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_LEN;
    for _ in 0..n {
        let rec = decode_rec(data.get(off..off + REC_BYTES)?)?;
        off += REC_BYTES;
        out.push(rec);
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => format!("{BASE}{SRC_QUERY}"),
    };
    let bytes = match fetch_raw_bytes(&url) {
        Some(b) => b,
        None => {
            eprintln!("{url}: fetch void");
            std::process::exit(1);
        }
    };
    let body = String::from_utf8_lossy(&bytes);
    let sources = match ia2_tap::parse_sources(&body) {
        Some(s) => s,
        None => {
            eprintln!(
                "wgesdss.laurino2011: {} B carry no measured psfMag row — the bin stays unwritten (0 honored)",
                bytes.len()
            );
            std::process::exit(1);
        }
    };
    let records = ia2_tap::to_skymap(&sources);
    if records.is_empty() {
        eprintln!("wgesdss.laurino2011: no placeable source — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bin = write_sky1(&records);
    if let Some(parent) = std::path::Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match read_sky1(&bin) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} sources from wgesdss.laurino2011, {} B — roundtrip parses",
                records.len(),
                bin.len()
            );
        }
        _ => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
