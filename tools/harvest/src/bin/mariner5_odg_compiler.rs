use std::env;
use std::fs;
use std::process;

const MAGIC: [u8; 4] = *b"ODG5";
const VERSION: u8 = 1;
const HEADER_LEN: usize = 13;
const REC_BYTES: usize = 8 + 8 + 4 + 4 + 4;

const EPOCH_1950: f64 = -631_152_000.0;

const LOGICAL_PER_RECORD: usize = 25;
const WORDS_PER_LOGICAL: usize = 8;
const RECORD_BYTES: usize = 1206;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Row {
    pub t: f64,
    pub obs: f64,
    pub ts: u32,
    pub code: u32,
    pub pass: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Packing {
    Unpacked,
    Packed,
}

fn chars_unpacked(payload: &[u8]) -> Vec<u64> {
    payload.iter().map(|b| u64::from(b & 0x3f)).collect()
}

fn chars_packed(payload: &[u8]) -> Vec<u64> {
    let mut out = Vec::with_capacity(payload.len() / 3 * 4);
    for group in payload.chunks_exact(3) {
        let v = (u64::from(group[0]) << 16) | (u64::from(group[1]) << 8) | u64::from(group[2]);
        out.push((v >> 18) & 0x3f);
        out.push((v >> 12) & 0x3f);
        out.push((v >> 6) & 0x3f);
        out.push(v & 0x3f);
    }
    out
}

fn words(chars: &[u64]) -> Vec<u64> {
    chars
        .chunks_exact(6)
        .map(|c| c.iter().fold(0u64, |w, &ch| (w << 6) | ch))
        .collect()
}

fn f36(w: u64) -> f64 {
    let sign = if (w >> 35) & 1 == 1 { -1.0 } else { 1.0 };
    let exp = ((w >> 27) & 0xff) as i32;
    let frac = (w & 0x07ff_ffff) as f64 / (1u64 << 27) as f64;
    sign * frac * 2f64.powi(exp - 128)
}

pub fn unix_from_words(w1: u64, w2: u64) -> Option<f64> {
    let v: u128 = (u128::from(w1) << 36) | u128::from(w2);
    let exp = ((v >> 63) & 0xff) as i32;
    let frac = (v & ((1u128 << 63) - 1)) as f64 / (1u128 << 63) as f64;
    let sign = if (v >> 71) & 1 == 1 { -1.0 } else { 1.0 };
    let seconds = sign * frac * 2f64.powi(exp - 128);
    let t = EPOCH_1950 + seconds;
    t.is_finite().then_some(t)
}

pub fn records(bytes: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while at + 2 <= bytes.len() {
        let n = usize::from(u16::from_be_bytes([bytes[at], bytes[at + 1]]));
        if n == 0 || at + 2 + n > bytes.len() {
            break;
        }
        out.push(&bytes[at + 2..at + 2 + n]);
        at += 2 + n;
    }
    out
}

pub fn decode(payload: &[u8], packing: Packing) -> (Vec<Row>, usize) {
    let mut rows = Vec::new();
    let mut skipped = 0usize;
    for rec in records(payload) {
        if rec.len() != RECORD_BYTES {
            skipped += 1;
            continue;
        }
        let chars = match packing {
            Packing::Unpacked => chars_unpacked(rec),
            Packing::Packed => chars_packed(rec),
        };
        let w = words(&chars);
        if w.len() < LOGICAL_PER_RECORD * WORDS_PER_LOGICAL + 1 {
            skipped += 1;
            continue;
        }
        for k in 0..LOGICAL_PER_RECORD {
            let b = k * WORDS_PER_LOGICAL;
            let Some(t) = unix_from_words(w[b + 1], w[b + 2]) else {
                skipped += 1;
                continue;
            };
            rows.push(Row {
                t,
                obs: f36(w[b + 6]),
                ts: (w[b + 7] & 0x7fff) as u32,
                code: (w[b + 5] & 0xffff_ffff) as u32,
                pass: (w[b + 4] & 0xffff_ffff) as u32,
            });
        }
    }
    (rows, skipped)
}

pub fn write_bin(rows: &[Row]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + rows.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&(rows.len() as u64).to_le_bytes());
    for r in rows {
        out.extend_from_slice(&r.t.to_le_bytes());
        out.extend_from_slice(&r.obs.to_le_bytes());
        out.extend_from_slice(&r.ts.to_le_bytes());
        out.extend_from_slice(&r.code.to_le_bytes());
        out.extend_from_slice(&r.pass.to_le_bytes());
    }
    out
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("mariner5_odg_compiler --in <datei.phys> --out-bin <ziel> [--packing unpacked|packed]");
        return;
    }
    let Some(path) = arg(&args, "--in") else {
        eprintln!("mariner5_odg_compiler: --in is missing");
        process::exit(2);
    };
    let out_path = match arg(&args, "--out-bin") {
        Some(v) => v,
        None => {
            eprintln!("mariner5_odg_compiler: --out-bin is missing — mariner5_odg.bin is written, named here instead of silent");
            "mariner5_odg.bin".to_string()
        }
    };
    let packing_name = match arg(&args, "--packing") {
        Some(v) => v,
        None => {
            eprintln!("mariner5_odg_compiler: --packing is missing — unpacked stands (Mariner 5), named here instead of silent");
            "unpacked".to_string()
        }
    };
    let packing = match packing_name.as_str() {
        "unpacked" => Packing::Unpacked,
        "packed" => Packing::Packed,
        other => {
            eprintln!("mariner5_odg_compiler: packing {other} is unknown");
            process::exit(2);
        }
    };
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("mariner5_odg_compiler: {path} is unreadable: {e}");
            process::exit(2);
        }
    };
    let (rows, skipped) = decode(&bytes, packing);
    if rows.is_empty() {
        eprintln!("mariner5_odg_compiler: no point in {path} — the bin stays unwritten");
        process::exit(1);
    }
    let bin = write_bin(&rows);
    if let Err(e) = fs::write(&out_path, &bin) {
        eprintln!("mariner5_odg_compiler: {out_path} is not writable: {e}");
        process::exit(2);
    }
    let t_min = rows.iter().map(|r| r.t).fold(f64::INFINITY, f64::min);
    let t_max = rows.iter().map(|r| r.t).fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out_path}: {} points, {skipped} records without a point, t [{t_min:.0}..{t_max:.0}] unix",
        rows.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const MARINER5_HEAD: &str = "00000700000013340624251e101e00000000122f31333b0001003100000100080215103c120e05033e0919231b00003c";

    const MARINER4_HEAD: &str =
        "0001c00004ef0526b44100000004afc70400511e03100008088a5848681cf3a6636c0258";

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
            .collect()
    }

    #[test]
    fn the_first_mariner5_point_lands_on_the_tape_label() {
        let chars = chars_unpacked(&hex(MARINER5_HEAD));
        let w = words(&chars);
        assert_eq!(w.len(), 8);
        let t = unix_from_words(w[1], w[2]).expect("time");
        assert!((t - (-80_499_342.0)).abs() < 1.0, "t = {t}");
        let obs = f36(w[6]);
        assert!((obs - 99_599.883_789).abs() < 1e-3, "obs = {obs}");
        assert_eq!((w[7] & 0x7fff) as u32, 60, "sample interval");
    }

    #[test]
    fn the_first_mariner4_point_lands_on_its_label() {
        let chars = chars_packed(&hex(MARINER4_HEAD));
        let w = words(&chars);
        assert_eq!(w.len(), 8);
        let t = unix_from_words(w[1], w[2]).expect("time");
        assert!((t - (-160_039_599.0)).abs() < 1.0, "t = {t}");
        let obs = f36(w[6]);
        assert!((obs - 53_305.903_320).abs() < 1e-3, "obs = {obs}");
        assert_eq!((w[7] & 0x7fff) as u32, 600, "sample interval");
    }

    #[test]
    fn one_unit_of_word_one_is_eight_seconds() {
        let chars = chars_unpacked(&hex(MARINER5_HEAD));
        let w = words(&chars);
        let v0: u128 = (u128::from(w[1]) << 36) | u128::from(w[2]);
        let exp = ((v0 >> 63) & 0xff) as i32;
        let one_unit = 2f64.powi(exp - 128) / (1u128 << 63) as f64 * (1u128 << 36) as f64;
        assert!((one_unit - 8.0).abs() < 1e-9, "one unit = {one_unit} s");
        assert!((7.5 * one_unit - 60.0).abs() < 1e-9);
    }

    #[test]
    fn a_truncated_record_yields_no_point() {
        let mut bytes = vec![0u8, 4, 0xb6];
        bytes.extend_from_slice(&[0u8; 100]);
        let (rows, _) = decode(&bytes, Packing::Unpacked);
        assert!(rows.is_empty(), "{rows:?}");
    }
}
