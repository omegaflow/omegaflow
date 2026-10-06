use omegaflow::archivar::fink_cutout::{
    FORCE_EM, KERNEL_INVERSE_SQUARE, MAGIC, REC_BYTES, SLOT_EPOCH, SLOT_PRESENCE, SLOT_VAL, TAU_S,
    TTL_S, write_bin,
};
use omegaflow::archivar::fits::{FitsHeader, FitsImage};
use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::process::Command;

const CUTOUT_ENDPOINT: &str = "https://api.lsst.fink-portal.org/api/v1/cutouts";
const CDN_TAG: &str = "api.lsst.fink-portal.org";
const ASSET: &str = "fink_cutout.bin";
const KINDS: [&str; 3] = ["Science", "Template", "Difference"];
const MJD_UNIX_EPOCH: f64 = 40587.0;
const SECS_PER_DAY: f64 = 86400.0;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn mjd_tai_to_tdb(lsk: &LeapSeconds, mjd_tai: f64) -> Option<f64> {
    let unix_tai = (mjd_tai - MJD_UNIX_EPOCH) * SECS_PER_DAY;
    let leap = lsk.leap_at(unix_tai)?;
    lsk.unix_to_tdb(unix_tai - leap)
}

fn fetch_cutout(id: &str, kind: &str) -> Option<Vec<u8>> {
    let body = format!(
        "{{\"diaSourceId\":\"{}\",\"kind\":\"{}\",\"output-format\":\"FITS\"}}",
        id, kind
    );
    let out = Command::new("curl")
        .args([
            "-sS",
            "-m",
            "60",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "--data-binary",
            &body,
            "-w",
            "\n%{http_code}",
            CUTOUT_ENDPOINT,
        ])
        .output()
        .ok()?;
    let stdout = out.stdout;
    let nl = stdout.iter().rposition(|&b| b == b'\n')?;
    let code: u16 = std::str::from_utf8(&stdout[nl + 1..])
        .ok()?
        .trim()
        .parse()
        .ok()?;
    if code != 200 {
        eprintln!("{CUTOUT_ENDPOINT}: http {code} carries no FITS image");
        return None;
    }
    Some(stdout[..nl].to_vec())
}

fn unit_vector(ra_deg: f64, dec_deg: f64) -> [f64; 3] {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let (sr, cr) = ra.sin_cos();
    let (sd, cd) = dec.sin_cos();
    [cd * cr, cd * sr, sd]
}

fn angular_step(image: &FitsImage, x: usize, y: usize) -> Option<f64> {
    let (ra0, dec0) = image.world(x as f64 + 1.0, y as f64 + 1.0)?;
    let (ra1, dec1) = image.world(x as f64 + 2.0, y as f64 + 1.0)?;
    if !(ra0.is_finite()
        && dec0.is_finite()
        && ra1.is_finite()
        && dec1.is_finite())
    {
        return None;
    }
    let d0 = dec0.to_radians();
    let d1 = dec1.to_radians();
    let dra = (ra1 - ra0).to_radians();
    let cos_sep = d0.sin() * d1.sin() + d0.cos() * d1.cos() * dra.cos();
    let sep = cos_sep.clamp(-1.0, 1.0).acos();
    if sep.is_finite() && sep > 0.0 {
        Some(sep)
    } else {
        None
    }
}

fn pixel_record(pos: [f64; 3], val: Option<f64>, epoch: f64, extent: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[SLOT_EPOCH] = epoch;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = extent;
    r[8] = KERNEL_INVERSE_SQUARE;
    r[9] = FORCE_EM;
    match val {
        Some(v) => {
            r[SLOT_VAL] = v;
            r[SLOT_PRESENCE] = 1.0;
        }
        None => r[SLOT_PRESENCE] = 0.0,
    }
    r
}

fn compile(buf: &[u8], epoch_mjd: Option<f64>, lsk: &LeapSeconds) -> Result<Vec<[f64; 26]>, String> {
    let (header, _) = FitsHeader::parse(buf, 0)
        .ok_or_else(|| "the primary header stays unread".to_string())?;
    let (image, _) = FitsImage::parse(buf, 0)
        .ok_or_else(|| "the primary image stays unread (BITPIX/NAXIS outside the gate)".to_string())?;
    let dims = image.dims;
    if dims[0] == 0 || dims[1] == 0 {
        return Err("the cutout carries an empty spatial axis".into());
    }
    if dims[2] != 1 {
        return Err(format!(
            "the cutout carries {} planes — the cutout contract is a 2D plane",
            dims[2]
        ));
    }
    let mjd = match epoch_mjd.or_else(|| header.f64("MJD-OBS")) {
        Some(v) if v.is_finite() => v,
        _ => return Err("MJD-OBS absent and no --mjd-tai given — no fabricated epoch".into()),
    };
    let epoch = mjd_tai_to_tdb(lsk, mjd)
        .ok_or_else(|| format!("MJD {mjd} lies outside the embedded leap table"))?;
    let extent = angular_step(&image, dims[0] / 2, dims[1] / 2)
        .or_else(|| angular_step(&image, 0, 0))
        .ok_or_else(|| "the WCS carries no measurable pixel step — the extent stays absent".to_string())?;
    let mut records: Vec<[f64; 26]> = Vec::with_capacity(dims[0] * dims[1]);
    let mut absent = 0usize;
    let mut unplaced = 0usize;
    for j in 0..dims[1] {
        for i in 0..dims[0] {
            let Some((ra, dec)) = image.world(i as f64 + 1.0, j as f64 + 1.0) else {
                unplaced += 1;
                continue;
            };
            if !(ra.is_finite() && dec.is_finite() && (-90.0..=90.0).contains(&dec)) {
                unplaced += 1;
                continue;
            }
            let val = image
                .value_f64(buf, [i, j, 0])
                .filter(|v| v.is_finite());
            if val.is_none() {
                absent += 1;
            }
            records.push(pixel_record(unit_vector(ra, dec), val, epoch, extent));
        }
    }
    if records.is_empty() {
        return Err(format!(
            "no pixel landed on the sphere — {unplaced} unplaced, {absent} absent"
        ));
    }
    eprintln!(
        "fink_cutout {}x{}: {} pixels, {} present, {} absent, {} unplaced, extent {:.3e} rad, epoch {mjd} MJD-TAI",
        dims[0],
        dims[1],
        records.len(),
        records.len() - absent,
        absent,
        unplaced,
        extent,
    );
    Ok(records)
}

fn read_back(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * REC_BYTES {
        return None;
    }
    let mut present = 0usize;
    for i in 0..n {
        let base = 8 + i * REC_BYTES + SLOT_PRESENCE * 8;
        if f64::from_le_bytes(bytes[base..base + 8].try_into().ok()?) == 1.0 {
            present += 1;
        }
    }
    Some(present)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let kind = match arg_value(args, "--kind") {
        Some(v) => v,
        None => "Science".to_string(),
    };
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("--kind {kind} is no Fink cutout kind"));
    }
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{CDN_TAG}/{ASSET}"),
    };
    let epoch_mjd = match arg_value(args, "--mjd-tai") {
        Some(v) => Some(
            v.parse::<f64>()
                .map_err(|_| format!("--mjd-tai {v} is not a number"))?,
        ),
        None => None,
    };
    let buf = match (
        arg_value(args, "--input"),
        arg_value(args, "--dia-source-id"),
    ) {
        (Some(path), _) => std::fs::read(&path).map_err(|e| format!("read {path}: {e}"))?,
        (None, Some(id)) => fetch_cutout(&id, &kind)
            .ok_or_else(|| format!("cutout fetch void for diaSourceId {id}"))?,
        (None, None) => {
            return Err(
                "--dia-source-id <id> or --input <fits> absent — refused".into(),
            );
        }
    };
    let lsk = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — the TAI→TDB step is absent".to_string())?;
    let records = compile(&buf, epoch_mjd, &lsk)?;
    let bin = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out}: {e}"))?;
    let read = std::fs::read(&out).map_err(|e| format!("read {out}: {e}"))?;
    if read != bin {
        return Err(format!("{out}: read-back differs — the asset stays unverified"));
    }
    let present = read_back(&read)
        .ok_or_else(|| format!("{out}: roundtrip parse void — the asset stays unverified"))?;
    eprintln!(
        "{out}: {} records, {} present, {} B, sha256 {}",
        records.len(),
        present,
        bin.len(),
        sha256_hex(&bin),
    );
    println!(
        "url https://github.com/omegaflow/sources/releases/download/{CDN_TAG}/{ASSET}"
    );
    println!("format fink_cutout");
    println!("origin {CUTOUT_ENDPOINT}");
    println!("ttl {}", TTL_S as u64);
    println!(
        "field fink_cutout_flux_njy fink_cutout_flux_njy inverse-square em nJy {} 0.0 0.0",
        TTL_S as u64
    );
    if ci_mode && !upload_release(CDN_TAG, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("fink_cutout_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(key: &str, value: &str) -> [u8; 80] {
        let mut c = [b' '; 80];
        let k = key.as_bytes();
        let kl = k.len().min(8);
        c[..kl].copy_from_slice(&k[..kl]);
        c[8] = b'=';
        let v = value.as_bytes();
        let vl = v.len().min(20);
        c[10..10 + vl].copy_from_slice(&v[..vl]);
        c
    }

    fn synthetic_cutout() -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&card("SIMPLE", "T"));
        header.extend_from_slice(&card("BITPIX", "-32"));
        header.extend_from_slice(&card("NAXIS", "2"));
        header.extend_from_slice(&card("NAXIS1", "30"));
        header.extend_from_slice(&card("NAXIS2", "30"));
        header.extend_from_slice(&card("CTYPE1", "'RA---TAN'"));
        header.extend_from_slice(&card("CTYPE2", "'DEC--TAN'"));
        header.extend_from_slice(&card("CRVAL1", "54.4145363293"));
        header.extend_from_slice(&card("CRVAL2", "29.5128706656"));
        header.extend_from_slice(&card("CRPIX1", "15.5"));
        header.extend_from_slice(&card("CRPIX2", "15.5"));
        header.extend_from_slice(&card("CDELT1", "0.0002777"));
        header.extend_from_slice(&card("CDELT2", "0.0002777"));
        header.extend_from_slice(&card("MJD-OBS", "61058.0826902019"));
        header.extend_from_slice(&card("END", ""));
        while !header.len().is_multiple_of(2880) {
            header.extend_from_slice(&[b' '; 80]);
        }
        let mut buf = header;
        for i in 0..900 {
            buf.extend_from_slice(&(i as f32).to_be_bytes());
        }
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn synthetic_cutout_folds_to_one_field_of_pixels() {
        let buf = synthetic_cutout();
        let lsk = embedded_lsk().expect("lsk");
        let records = compile(&buf, None, &lsk).expect("the cutout compiles");
        assert_eq!(records.len(), 900);
        assert!(records.iter().all(|r| r[SLOT_PRESENCE] == 1.0));
        let bytes = write_bin(&records);
        assert_eq!(read_back(&bytes), Some(900));
    }

    #[test]
    fn unit_vector_lands_on_the_sphere() {
        let v = unit_vector(54.4145363293, 29.5128706656);
        let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        assert!((n - 1.0).abs() < 1e-12);
    }

    #[test]
    fn a_cutout_without_an_epoch_is_refused() {
        let mut buf = synthetic_cutout();
        let lsk = embedded_lsk().expect("lsk");
        let (img, _) = FitsImage::parse(&buf, 0).expect("image parses");
        assert!(img.dims[0] == 30);
        if let Some(pos) = buf.windows(7).position(|w| w == b"MJD-OBS".as_slice()) {
            for b in &mut buf[pos + 10..pos + 30] {
                *b = b' ';
            }
        }
        assert!(compile(&buf, None, &lsk).is_err());
    }
}
