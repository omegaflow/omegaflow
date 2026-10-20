use omegaflow::archivar::inflate::zip_members;
use omegaflow::archivar::skymap::{
    KIND_GENERIC, REC_BYTES, SkymapRecord, encode_rec, write_header,
};
use omegaflow::cdn::upload_release;

const TAG: &str = "zenodo.org";
const DEFAULT_URL: &str =
    "https://github.com/omegaflow/sources/releases/download/zenodo.org/epta_dr2.zip";
const OBLIQUITY_J2000_DEG: f64 = 23.4392911;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn hms_deg(s: &str) -> Option<f64> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() < 2 {
        return s.parse::<f64>().ok();
    }
    let h: f64 = parts[0].parse().ok()?;
    let m: f64 = parts[1].parse().ok()?;
    let sec: f64 = parts.get(2)?.parse().ok()?;
    Some((h + m / 60.0 + sec / 3600.0) * 15.0)
}

fn dms_deg(s: &str) -> Option<f64> {
    let neg = s.starts_with('-');
    let t = s.trim_start_matches(['+', '-']);
    let parts: Vec<&str> = t.split(':').collect();
    if parts.len() < 2 {
        return s.parse::<f64>().ok();
    }
    let d: f64 = parts[0].parse().ok()?;
    let m: f64 = parts[1].parse().ok()?;
    let sec: f64 = parts.get(2)?.parse().ok()?;
    let v = d + m / 60.0 + sec / 3600.0;
    Some(if neg { -v } else { v })
}

fn ecliptic_to_equatorial(lon_deg: f64, lat_deg: f64) -> Option<(f64, f64)> {
    if !(lon_deg.is_finite() && lat_deg.is_finite()) {
        return None;
    }
    let l = lon_deg.to_radians();
    let b = lat_deg.to_radians();
    let eps = OBLIQUITY_J2000_DEG.to_radians();
    let (sl, cl) = l.sin_cos();
    let (sb, cb) = b.sin_cos();
    let x = cb * cl;
    let y = cb * sl;
    let z = sb;
    let y_eq = y * eps.cos() - z * eps.sin();
    let z_eq = y * eps.sin() + z * eps.cos();
    let ra = y_eq.atan2(x).to_degrees().rem_euclid(360.0);
    let dec = z_eq.clamp(-1.0, 1.0).asin().to_degrees();
    Some((ra, dec))
}

fn parse_par(text: &str) -> Option<(String, f64, f64, f64)> {
    let mut name = String::new();
    let mut raj = None;
    let mut decj = None;
    let mut elong = None;
    let mut elat = None;
    let mut tres = None;
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let mut it = t.split_whitespace();
        let (Some(key), Some(val)) = (it.next(), it.next()) else {
            continue;
        };
        match key {
            "PSRJ" => name = val.to_string(),
            "RAJ" => raj = hms_deg(val),
            "DECJ" => decj = dms_deg(val),
            "ELONG" => elong = val.parse::<f64>().ok(),
            "ELAT" => elat = val.parse::<f64>().ok(),
            "TRES" => tres = val.parse::<f64>().ok(),
            _ => {}
        }
    }
    let (ra, dec) = match (raj, decj) {
        (Some(r), Some(d)) => (r, d),
        _ => ecliptic_to_equatorial(elong?, elat?)?,
    };
    let tres_s = tres? * 1e-6;
    if !(tres_s.is_finite() && tres_s > 0.0) {
        return None;
    }
    if !(0.0..360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
        return None;
    }
    Some((name, ra, dec, tres_s))
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = std::process::Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("600")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let out_path = arg_value(args, "--out").ok_or(
        "usage: epta_dr2_compiler --out <epta_dr2.sky1> [--zip <path>] [--ci-mode] — refused",
    )?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let bytes = match arg_value(args, "--zip") {
        Some(p) => std::fs::read(&p).map_err(|e| format!("read {p} returned void: {e}"))?,
        None => fetch(DEFAULT_URL).ok_or(format!("{DEFAULT_URL} carries void"))?,
    };

    let mut records: Vec<SkymapRecord> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let mut pars = 0usize;
    zipped_dr2full(&bytes, |name, text| {
        pars += 1;
        let Some((psr, ra, dec, tres_s)) = parse_par(text) else {
            return;
        };
        let key = if psr.is_empty() {
            name.to_string()
        } else {
            psr
        };
        if seen.iter().any(|s| s == &key) {
            return;
        }
        let Some((order, ipix)) = SkymapRecord::pixel_of(ra, dec) else {
            return;
        };
        seen.push(key);
        records.push(SkymapRecord {
            order,
            kind: KIND_GENERIC,
            ipix,
            ra_deg: ra as f32,
            dec_deg: dec as f32,
            value: tres_s as f32,
        });
    })
    .ok_or("the zip directory stays unread — the asset stays unwritten")?;

    if records.is_empty() {
        return Err(format!(
            "{pars} .par files read, no TRES/position pair — the asset stays unwritten (0 honored)"
        ));
    }

    let mut buf = Vec::with_capacity(13 + records.len() * REC_BYTES);
    write_header(&mut buf, records.len() as u64);
    let mut rec = [0u8; REC_BYTES];
    for r in &records {
        encode_rec(&mut rec, r);
        buf.extend_from_slice(&rec);
    }
    let expect = 13 + records.len() * REC_BYTES;
    if buf.len() != expect {
        return Err(format!(
            "{} bytes laid out, {expect} expected — the asset stays unwritten",
            buf.len()
        ));
    }
    std::fs::write(&out_path, &buf).map_err(|e| format!("write {out_path} returned void: {e}"))?;
    eprintln!(
        "epta_dr2_compiler: {} pulsars with TRES from {pars} .par files -> {out_path} ({} bytes, SKY1)",
        records.len(),
        buf.len()
    );
    if ci_mode && !upload_release(TAG, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn zipped_dr2full<F: FnMut(&str, &str)>(bytes: &[u8], mut sink: F) -> Option<usize> {
    let mut hit = 0usize;
    zip_members(bytes, |name, data| {
        if name.contains("/DR2full/") && name.ends_with(".par") {
            if let Ok(text) = std::str::from_utf8(data) {
                hit += 1;
                sink(name, text);
            }
        }
    })?;
    Some(hit)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("epta_dr2_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAR: &str = "PSRJ           J1022+1001\n\
ELONG          153.8658669115706568025769840 1  0.00000000737350537945\n\
ELAT           -0.0639297390877120912458520 1  0.00000730676153732687\n\
TRES           1.022\n";

    #[test]
    fn sexagesimal_carries_the_expected_degrees() {
        assert!((hms_deg("17:46:52.123").unwrap() - 266.717179).abs() < 1e-5);
        assert!((dms_deg("-34:12:34.5").unwrap() + 34.209583).abs() < 1e-5);
        assert!((dms_deg("+12:30:00").unwrap() - 12.5).abs() < 1e-12);
    }

    #[test]
    fn ecliptic_zero_maps_to_equatorial_zero_and_pole_to_obliquity() {
        let (ra, dec) = ecliptic_to_equatorial(0.0, 0.0).unwrap();
        assert!(ra.abs() < 1e-9 && dec.abs() < 1e-9);
        let (ra, dec) = ecliptic_to_equatorial(90.0, 0.0).unwrap();
        assert!((ra - 90.0).abs() < 1e-9);
        assert!((dec - OBLIQUITY_J2000_DEG).abs() < 1e-9);
    }

    #[test]
    fn par_reads_tres_in_seconds_with_the_ecliptic_fallback() {
        let (name, ra, dec, tres) = parse_par(PAR).unwrap();
        assert_eq!(name, "J1022+1001");
        assert!((tres - 1.022e-6).abs() < 1e-15);
        assert!((ra - 155.7417).abs() < 1e-3);
        assert!((dec - 10.0313).abs() < 1e-3);
    }

    #[test]
    fn missing_tres_is_dropped_not_zeroed() {
        let text = "PSRJ J0000+0000\nRAJ 00:00:00\nDECJ +00:00:00\n";
        assert!(parse_par(text).is_none());
    }
}
