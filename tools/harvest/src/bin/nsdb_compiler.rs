use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::collections::BTreeSet;

const NETLOC: &str = "nsdb.imcce.fr";
const COMPILER: &str = "tools/harvest/src/bin/nsdb_compiler.rs";
const FORMAT: &str = "nsdb";

const MAGIC: [u8; 4] = *b"NSDB";
const HEADER_BYTES: usize = 8;
const RECORD_BYTES: usize = 40;

const FLAG_MAG: u32 = 1 << 8;

const HOUR_DEG: f64 = 15.0;
const DAY_S: f64 = 86400.0;

const SAMPLE: &str = "200205 2002 08 14.01345  20 45 49.07  -18 10 00.2    -    -  807\n\
200205 2002 08 14.17147  20 45 48.04  -18 10 04.4   25.3  R  807\n\
202101 2021 09 07.45001  23 33 34.149 -04 19 20.31  26.5  r  568\n";

#[derive(Clone, Copy, Debug, PartialEq)]
struct Obs {
    epoch_unix: f64,
    ra_deg: f64,
    dec_deg: f64,
    mag: Option<f64>,
    band: u8,
    obscode: u32,
}

fn finite(tok: &str) -> Option<f64> {
    let v: f64 = tok.parse().ok()?;
    v.is_finite().then_some(v)
}

fn sexagesimal(sum: f64) -> bool {
    sum.is_finite()
}

fn parse_line(line: &str) -> Option<Obs> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 13 {
        return None;
    }
    let year: i64 = t[1].parse().ok()?;
    let month: i64 = t[2].parse().ok()?;
    let day_frac: f64 = t[3].parse().ok()?;
    if !day_frac.is_finite() {
        return None;
    }
    let day = day_frac.trunc() as i64;
    let frac = day_frac.fract();
    let days = days_from_civil(year, month, day)?;
    let epoch_unix = days as f64 * DAY_S + frac * DAY_S;

    let ra_h = finite(t[4])?;
    let ra_m = finite(t[5])?;
    let ra_s = finite(t[6])?;
    let ra_deg = HOUR_DEG * (ra_h + ra_m / 60.0 + ra_s / 3600.0);

    let dec_d = finite(t[7])?;
    let dec_m = finite(t[8])?;
    let dec_s = finite(t[9])?;
    let sign = if dec_d < 0.0 { -1.0 } else { 1.0 };
    let dec_deg = sign * (dec_d.abs() + dec_m / 60.0 + dec_s / 3600.0);
    if !(-90.0..=90.0).contains(&dec_deg) || !sexagesimal(ra_deg) {
        return None;
    }

    let mag = if t[10] == "-" { None } else { finite(t[10]) };
    let band = if t[11] == "-" { 0 } else { t[11].as_bytes()[0] };
    let obscode: u32 = t[12].parse().ok()?;

    Some(Obs {
        epoch_unix,
        ra_deg,
        dec_deg,
        mag,
        band,
        obscode,
    })
}

fn parse_nsdb(bytes: &[u8]) -> Option<Vec<Obs>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out = Vec::new();
    for line in text.lines() {
        if let Some(obs) = parse_line(line) {
            out.push(obs);
        }
    }
    (!out.is_empty()).then_some(out)
}

fn put_opt_f64(out: &mut Vec<u8>, v: Option<f64>, flags: &mut u32, bit: u32) -> bool {
    match v {
        Some(x) => {
            if !x.is_finite() {
                return false;
            }
            *flags |= bit;
            out.extend_from_slice(&x.to_le_bytes());
        }
        None => out.extend_from_slice(&0.0f64.to_le_bytes()),
    }
    true
}

fn write_bin(obs: &[Obs]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(HEADER_BYTES + obs.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(obs.len() as u32).to_le_bytes());
    for o in obs {
        if !o.epoch_unix.is_finite() || !o.ra_deg.is_finite() || !o.dec_deg.is_finite() {
            return None;
        }
        let mut flags = u32::from(o.band);
        out.extend_from_slice(&o.epoch_unix.to_le_bytes());
        out.extend_from_slice(&o.ra_deg.to_le_bytes());
        out.extend_from_slice(&o.dec_deg.to_le_bytes());
        if !put_opt_f64(&mut out, o.mag, &mut flags, FLAG_MAG) {
            return None;
        }
        out.extend_from_slice(&o.obscode.to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
    }
    Some(out)
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<Obs>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() < HEADER_BYTES + count * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * RECORD_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            Some(f64::from_le_bytes(
                bytes.get(base + o..base + o + 8)?.try_into().ok()?,
            ))
        };
        let u32_at = |o: usize| -> Option<u32> {
            Some(u32::from_le_bytes(
                bytes.get(base + o..base + o + 4)?.try_into().ok()?,
            ))
        };
        let epoch_unix = f64_at(0)?;
        let ra_deg = f64_at(8)?;
        let dec_deg = f64_at(16)?;
        let mag_raw = f64_at(24)?;
        let obscode = u32_at(32)?;
        let flags = u32_at(36)?;
        let mag = if flags & FLAG_MAG != 0 {
            Some(mag_raw)
        } else {
            None
        };
        out.push(Obs {
            epoch_unix,
            ra_deg,
            dec_deg,
            mag,
            band: (flags & 0xFF) as u8,
            obscode,
        });
    }
    Some(out)
}

fn measured(obs: Vec<Obs>) -> (Vec<Obs>, usize) {
    let total = obs.len();
    let held: Vec<Obs> = obs.into_iter().filter(|o| o.ra_deg.is_finite()).collect();
    let dropped = total - held.len();
    (held, dropped)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let input = arg_value(args, "--input")
        .ok_or_else(|| "--input <nsdb_table.txt> absent — the file stays unread".to_string())?;
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{FORMAT}.bin"),
    };

    let bytes = std::fs::read(&input).map_err(|e| format!("{input}: read void ({e})"))?;
    let obs = parse_nsdb(&bytes)
        .ok_or_else(|| format!("{input}: carries no NSDB observation row — refused"))?;
    let (mut named, dropped) = measured(obs);
    if dropped > 0 {
        eprintln!("{input}: {dropped} rows carry no finite right ascension — held out, named");
    }
    if named.is_empty() {
        return Err(format!(
            "{input}: no row carries a finite position — the bin stays unwritten (0 honored)"
        ));
    }
    named.sort_by(|a, b| a.epoch_unix.total_cmp(&b.epoch_unix));

    let bin = write_bin(&named)
        .ok_or_else(|| "a held position is not finite — the bin stays unwritten".to_string())?;
    match parse_bin(&bin) {
        Some(parsed) if parsed == named => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;

    let codes: BTreeSet<u32> = named.iter().map(|o| o.obscode).collect();
    let banded = named.iter().filter(|o| o.band != 0).count();
    let magged = named.iter().filter(|o| o.mag.is_some()).count();

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {input}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} observations, {} observatory codes {codes:?}, {banded} banded, {magged} magnitude, {} B, roundtrip parses",
        named.len(),
        codes.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let Some(obs) = parse_nsdb(SAMPLE.as_bytes()) else {
        eprintln!("selftest: the sample NSDB table does not parse");
        std::process::exit(1);
    };
    if obs.len() != 3 {
        eprintln!("selftest: the sample does not carry exactly three observations");
        std::process::exit(1);
    }
    let first = obs[0];
    if (first.ra_deg - 311.4545).abs() > 1e-3 {
        eprintln!(
            "selftest: the right ascension does not match ({} deg)",
            first.ra_deg
        );
        std::process::exit(1);
    }
    if (first.dec_deg + 18.166_722).abs() > 1e-3 {
        eprintln!(
            "selftest: the declination does not match ({} deg)",
            first.dec_deg
        );
        std::process::exit(1);
    }
    if first.mag.is_some() || first.obscode != 807 {
        eprintln!("selftest: the absent magnitude / observatory code do not match");
        std::process::exit(1);
    }
    if obs[1].mag != Some(25.3) || obs[1].band != b'R' {
        eprintln!("selftest: the present magnitude / band do not match");
        std::process::exit(1);
    }
    let Some(bin) = write_bin(&obs) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    if bin.len() != HEADER_BYTES + 3 * RECORD_BYTES {
        eprintln!("selftest: the bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if parse_bin(&bin) != Some(obs.clone()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    eprintln!("nsdb_compiler: selftest passes (NSDB observations -> epoch/position series)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: nsdb_compiler --input <nsdb_table.txt> [--out <file.bin>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("nsdb_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_roundtrips_the_measured_stride() {
        let obs = parse_nsdb(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = write_bin(&obs).expect("finite observations encode");
        assert_eq!(bin.len(), HEADER_BYTES + 3 * RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(obs));
    }

    #[test]
    fn parse_nsdb_refuses_foreign_bytes() {
        assert_eq!(parse_nsdb(b"not an nsdb table"), None);
        assert_eq!(parse_nsdb(b""), None);
    }

    #[test]
    fn absent_magnitude_stays_absent() {
        let obs = parse_nsdb(SAMPLE.as_bytes()).expect("the sample parses");
        assert_eq!(obs[0].mag, None);
        assert_eq!(obs[0].band, 0);
        assert_eq!(obs[1].mag, Some(25.3));
        assert_eq!(obs[2].band, b'r');
    }
}
