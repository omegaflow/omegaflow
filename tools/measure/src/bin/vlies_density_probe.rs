use omegaflow::mathematikerin::healpix::ang2pix_nest;
use std::f64::consts::PI;

const MAGIC: [u8; 4] = *b"VLDE";
const VERSION: u8 = 1;
const HEADER_BYTES: usize = 9;

struct DensityField {
    nside: i64,
    counts: Vec<u64>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn npix_of(nside: i64) -> Option<usize> {
    if nside <= 0 || nside & (nside - 1) != 0 {
        return None;
    }
    let n = nside as u64;
    let npix = 12u64.checked_mul(n)?.checked_mul(n)?;
    usize::try_from(npix).ok()
}

fn parse_asset(b: &[u8]) -> Option<DensityField> {
    if b.len() < HEADER_BYTES || &b[0..4] != MAGIC || b[4] != VERSION {
        return None;
    }
    let nside = u32::from_le_bytes(b[5..9].try_into().ok()?) as i64;
    let npix = npix_of(nside)?;
    if b.len() != HEADER_BYTES + npix * 8 {
        return None;
    }
    let mut counts = Vec::with_capacity(npix);
    for off in (HEADER_BYTES..b.len()).step_by(8) {
        let raw: [u8; 8] = b[off..off + 8].try_into().ok()?;
        counts.push(u64::from_le_bytes(raw));
    }
    Some(DensityField { nside, counts })
}

fn query(field: &DensityField, ra_deg: f64, dec_deg: f64) -> Option<(i64, u64)> {
    let theta = (90.0 - dec_deg).to_radians();
    let phi = ra_deg.to_radians();
    let pix = ang2pix_nest(field.nside, theta, phi)?;
    let count = *field.counts.get(pix as usize)?;
    Some((pix, count))
}

fn direction_of(args: &[String], name: &str) -> Option<f64> {
    let v = arg_value(args, name)?;
    let n: f64 = v.parse().ok()?;
    if n.is_finite() {
        Some(n)
    } else {
        None
    }
}

fn usage() {
    eprintln!("usage: vlies_density_probe --map <asset.vlde> --ra <deg> --dec <deg>");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(map_path) = arg_value(&args, "--map") else {
        usage();
        std::process::exit(1);
    };
    let Some(ra) = direction_of(&args, "--ra") else {
        usage();
        std::process::exit(1);
    };
    let Some(dec) = direction_of(&args, "--dec") else {
        usage();
        std::process::exit(1);
    };
    if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
        eprintln!("--ra {ra} --dec {dec}: the direction leaves the sphere — refused");
        std::process::exit(1);
    }
    let bytes = match std::fs::read(&map_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("bin void {map_path}: {e}");
            std::process::exit(1);
        }
    };
    let Some(field) = parse_asset(&bytes) else {
        eprintln!("bin void {map_path}: the VLDE asset stays unread");
        std::process::exit(1);
    };
    let Some((pix, count)) = query(&field, ra, dec) else {
        eprintln!("bin void {map_path}: the pixel stays unread");
        std::process::exit(1);
    };
    let npix = field.counts.len() as f64;
    let pixel_sr = 4.0 * PI / npix;
    let per_sr = count as f64 / pixel_sr;
    println!("vlies density ra {ra} dec {dec} pix {pix} count {count} per-sr {per_sr:.6}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::spatial::{parse_star_record, STAR_RECORD_BYTES};

    fn star_rec(ra: f64, dec: f64, plx: f32) -> Vec<u8> {
        let mut b = Vec::with_capacity(STAR_RECORD_BYTES);
        b.extend_from_slice(&ra.to_le_bytes());
        b.extend_from_slice(&dec.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&plx.to_le_bytes());
        b.extend_from_slice(&11.0f32.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&0.8f32.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b
    }

    fn compile_counts(bytes: &[u8], nside: i64) -> Vec<u64> {
        let npix = (12 * nside * nside) as usize;
        let mut counts = vec![0u64; npix];
        for chunk in bytes.chunks_exact(STAR_RECORD_BYTES) {
            let Some(rec) = parse_star_record(chunk) else {
                continue;
            };
            let theta = (90.0 - rec.dec_deg).to_radians();
            let phi = rec.ra_deg.to_radians();
            if let Some(pix) = ang2pix_nest(nside, theta, phi) {
                counts[pix as usize] += 1;
            }
        }
        counts
    }

    fn asset_bytes(nside: i64, counts: &[u64]) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_BYTES + counts.len() * 8);
        out.extend_from_slice(&MAGIC);
        out.push(VERSION);
        out.extend_from_slice(&(nside as u32).to_le_bytes());
        for c in counts {
            out.extend_from_slice(&c.to_le_bytes());
        }
        out
    }

    #[test]
    fn compiled_density_reads_back_at_probed_directions() {
        let nside = 4i64;
        let dirs = [
            (0.0, 0.0),
            (120.0, 45.0),
            (200.0, -60.0),
            (266.405, -28.936),
            (0.0, 0.0),
        ];
        let mut bin = Vec::new();
        for (ra, dec) in dirs {
            bin.extend(star_rec(ra, dec, 1.0));
        }
        bin.extend(star_rec(100.0, 20.0, -0.5));
        let counts = compile_counts(&bin, nside);
        let field = parse_asset(&asset_bytes(nside, &counts)).unwrap();
        assert_eq!(field.nside, nside);
        assert_eq!(field.counts, counts);
        let mut seen: Vec<(f64, f64)> = Vec::new();
        for (ra, dec) in dirs {
            if seen.contains(&(ra, dec)) {
                continue;
            }
            seen.push((ra, dec));
            let (pix, count) = query(&field, ra, dec).unwrap();
            let expect_pix =
                ang2pix_nest(nside, (90.0 - dec).to_radians(), ra.to_radians()).unwrap();
            assert_eq!(pix, expect_pix, "pixel for ra {ra} dec {dec}");
            let n = dirs.iter().filter(|&&(r, d)| r == ra && d == dec).count() as u64;
            assert_eq!(count, n, "count for ra {ra} dec {dec}");
        }
        let (_, void) = query(&field, 90.0, 89.0).unwrap();
        assert_eq!(void, 0);
    }

    #[test]
    fn void_and_truncated_assets_stay_void() {
        assert!(parse_asset(&[]).is_none());
        assert!(parse_asset(b"VLDE\x01").is_none());
        let mut wrong = asset_bytes(2, &vec![0u64; 48]);
        wrong[0] = b'?';
        assert!(parse_asset(&wrong).is_none());
        let asset = asset_bytes(2, &vec![0u64; 48]);
        assert!(parse_asset(&asset[..asset.len() - 8]).is_none());
    }

    #[test]
    fn query_pixel_matches_ang2pix() {
        let nside = 8i64;
        let counts = vec![0u64; (12 * nside * nside) as usize];
        let field = parse_asset(&asset_bytes(nside, &counts)).unwrap();
        for (ra, dec) in [(0.0, 90.0), (90.0, 0.0), (200.0, -45.0), (359.9, -89.9)] {
            let (pix, count) = query(&field, ra, dec).unwrap();
            assert_eq!(
                pix,
                ang2pix_nest(nside, (90.0 - dec).to_radians(), ra.to_radians()).unwrap()
            );
            assert_eq!(count, 0);
        }
    }
}
