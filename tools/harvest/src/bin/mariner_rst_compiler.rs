use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mariner_rst;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::{gunzip, tar_members};
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "spdf.gsfc.nasa.gov";
const FORMAT: &str = "mariner_rst";
const URL: &str = "https://spdf.gsfc.nasa.gov/pub/data/mariner/mariner6/celestial_mechanics/2_way_doppler_radio_tracking/PSCM-00017_DD005408_05-MAR-69.tar";

const HEADER_RECORDS: usize = 3;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn words_of(payload: &[u8]) -> Vec<u64> {
    payload
        .chunks_exact(6)
        .map(|c| {
            let mut w = 0u64;
            for &b in c {
                w = (w << 6) | (b & 0x3F) as u64;
            }
            w
        })
        .collect()
}

fn unix_from_words(w2: u64, w3: u64) -> Option<f64> {
    let v: u128 = ((w2 as u128) << 36) | (w3 as u128);
    let exp = ((v >> 63) & 0xFF) as i32;
    let frac = (v & ((1u128 << 63) - 1)) as f64;
    let sign = if (v >> 71) & 1 == 1 { -1.0 } else { 1.0 };
    let seconds = sign * (frac / (2f64.powi(63))) * 2f64.powi(exp - 128);
    if !seconds.is_finite() {
        return None;
    }
    let epoch1950 = (days_from_civil(1950, 1, 1)? as f64) * 86400.0;
    Some(epoch1950 + seconds)
}

fn hex_single(w: u64) -> f64 {
    let sign = if (w >> 35) & 1 == 1 { -1.0 } else { 1.0 };
    let exp = ((w >> 28) & 0x7F) as i32;
    let frac = (w & 0x0FFF_FFFF) as f64 / (2f64.powi(28));
    sign * frac * 16f64.powi(exp - 64)
}

fn code_bits(code: u64) -> Vec<u32> {
    (1..=12u32)
        .filter(|&k| (code >> (36 - k)) & 1 == 1)
        .collect()
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out-bin") {
        Some(v) => v,
        None => "mariner_rst.bin".to_string(),
    };

    let raw = match arg_value(args, "--in") {
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?
        }
        None => fetch_raw_bytes(URL).ok_or_else(|| format!("{URL}: fetch void"))?,
    };
    let mut rows: Vec<(f64, f64, u32)> = Vec::new();

    let gunzipped;
    let dats: Vec<&[u8]> = if let Some(members) = tar_members(&raw) {
        members
            .iter()
            .filter(|m| m.name.ends_with("_F1.DAT"))
            .filter_map(|m| raw.get(m.start..m.end))
            .collect()
    } else if let Some(t) = gunzip(&raw) {
        gunzipped = t;
        let members = tar_members(&gunzipped)
            .ok_or_else(|| format!("{URL}: tar stream does not enumerate"))?;
        members
            .iter()
            .filter(|m| m.name.ends_with("_F1.DAT"))
            .filter_map(|m| gunzipped.get(m.start..m.end))
            .collect()
    } else {
        vec![&raw[..]]
    };

    for data in dats {
        let mut off = 0usize;
        let mut index = 0usize;
        while off + 2 <= data.len() {
            let len = u16::from_be_bytes([data[off], data[off + 1]]) as usize;
            let payload = off + 2;
            let end = payload + len;
            if len == 0 || end > data.len() {
                break;
            }
            let words = words_of(&data[payload..end]);
            if index >= HEADER_RECORDS && words.len() >= 8 {
                if let Some(t) = unix_from_words(words[2], words[3]) {
                    if !(-252_460_800.0..=189_345_600.0).contains(&t) {
                        off = end;
                        index += 1;
                        continue;
                    }
                    let bits = code_bits(words[7]);
                    for (j, &dtype) in bits.iter().enumerate() {
                        if let Some(&w) = words.get(8 + j) {
                            rows.push((t, hex_single(w), dtype));
                        }
                    }
                }
            }
            off = end;
            index += 1;
        }
    }
    if rows.is_empty() {
        return Err("no observables parsed — the bin stays unwritten (0 honored)".to_string());
    }
    let bin = mariner_rst::write_bin(&rows);
    if mariner_rst::parse_bin(&bin).is_none() {
        return Err(format!("{out}: roundtrip parse void"));
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    let t_min = rows.iter().map(|r| r.0).fold(f64::INFINITY, f64::min);
    let t_max = rows.iter().map(|r| r.0).fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} observables, t [{t_min:.0}..{t_max:.0}] unix, format {FORMAT}, sha256 {}",
        rows.len(),
        sha256_hex(&bin)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: mariner_rst_compiler [--in <F1.DAT|tar>] [--out-bin <name>] [--ci-mode]");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("mariner_rst_compiler: {msg}");
        std::process::exit(1);
    }
}
