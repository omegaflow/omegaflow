use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::spatial::{parse_star_record, star_stride, STAR_RECORD_BYTES};
use omegaflow::cdn::{upload_asset, CDN_BASE};
use omegaflow::mathematikerin::healpix::ang2pix_nest;

const NSIDE: i64 = 128;
const MAGIC: [u8; 4] = *b"VLDE";
const VERSION: u8 = 1;
const HEADER_BYTES: usize = 9;

const TWOMASS_MAGIC: [u8; 4] = *b"2MPS";
const TWOMASS_HEADER_BYTES: usize = 8;
const TWOMASS_RECORD_BYTES: usize = 64;

struct DensityAsset {
    nside: i64,
    counts: Vec<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Catalog {
    Stars,
    Twomass,
}

impl Catalog {
    fn parse(s: &str) -> Option<Catalog> {
        match s {
            "stars" => Some(Catalog::Stars),
            "twomass" => Some(Catalog::Twomass),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Catalog::Stars => "stars",
            Catalog::Twomass => "twomass",
        }
    }
}

const BIN_TTL_S: u64 = 604800;

fn cdn_asset(kind: Catalog) -> (&'static str, &'static str) {
    match kind {
        Catalog::Stars => ("ssd.jpl.nasa.gov", "dr3_stars.bin"),
        Catalog::Twomass => ("irsa.ipac.caltech.edu", "twomass_psc.bin"),
    }
}

fn ensure_bin(path: &str, netloc: &str, asset: &str, ttl: u64) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("{}/{}/{}", CDN_BASE, netloc, asset);
    let bytes = fetch_raw_bytes(&url, ttl)?;
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(path, &bytes).is_err() {
        return None;
    }
    Some(bytes)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn npix_of(nside: i64) -> Option<usize> {
    if nside <= 0 || nside & (nside - 1) != 0 {
        return None;
    }
    let n = nside as u64;
    let npix = 12u64.checked_mul(n)?.checked_mul(n)?;
    usize::try_from(npix).ok()
}

fn collect_inputs(args: &[String]) -> Result<Vec<(Catalog, String)>, String> {
    let mut inputs = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--catalog" => {
                let Some(kind) = args.get(i + 1) else {
                    return Err("--catalog <kind> <bin>: the kind is never silent — refused".into());
                };
                let Some(path) = args.get(i + 2) else {
                    return Err(format!(
                        "--catalog {kind} <bin>: the bin path is never silent — refused"
                    ));
                };
                let Some(cat) = Catalog::parse(kind) else {
                    return Err(format!(
                        "--catalog {kind}: unknown kind — known kinds: stars, twomass — refused"
                    ));
                };
                inputs.push((cat, path.clone()));
                i += 3;
            }
            "--stars" => {
                let Some(path) = args.get(i + 1) else {
                    return Err(
                        "--stars <dr3_stars.bin>: the bin path is never silent — refused".into(),
                    );
                };
                inputs.push((Catalog::Stars, path.clone()));
                i += 2;
            }
            "--out" => {
                if args.get(i + 1).is_none() {
                    return Err(
                        "--out <asset.vlde>: the asset path is never silent — refused".into(),
                    );
                }
                i += 2;
            }
            "--ci-mode" => i += 1,
            other => {
                return Err(format!("unknown argument {other} — refused"));
            }
        }
    }
    if inputs.is_empty() {
        return Err(
            "usage: vlies_density_compiler (--stars <dr3_stars.bin> | --catalog <kind> <bin>)... --out <asset.vlde> [--ci-mode], kind in {stars, twomass} — refused"
                .into(),
        );
    }
    Ok(inputs)
}

fn twomass_body(bytes: &[u8]) -> Option<&[u8]> {
    if bytes.len() < TWOMASS_HEADER_BYTES || &bytes[0..4] != &TWOMASS_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let body = bytes.get(TWOMASS_HEADER_BYTES..)?;
    if body.len() != count * TWOMASS_RECORD_BYTES {
        return None;
    }
    Some(body)
}

fn twomass_radec(b: &[u8]) -> Option<(f64, f64)> {
    if b.len() != TWOMASS_RECORD_BYTES {
        return None;
    }
    let ra = f64::from_le_bytes(b[0..8].try_into().ok()?);
    let dec = f64::from_le_bytes(b[8..16].try_into().ok()?);
    if !ra.is_finite() || !dec.is_finite() {
        return None;
    }
    if !(0.0..360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
        return None;
    }
    Some((ra, dec))
}

fn place_pixel(counts: &mut [u64], ra_deg: f64, dec_deg: f64) -> bool {
    let theta = (90.0 - dec_deg).to_radians();
    let phi = ra_deg.to_radians();
    match ang2pix_nest(NSIDE, theta, phi) {
        Some(pix) => {
            counts[pix as usize] += 1;
            true
        }
        None => false,
    }
}

fn count_into(counts: &mut [u64], bytes: &[u8], kind: Catalog) -> Result<(u64, u64, u64), String> {
    let mut records = 0u64;
    let mut counted = 0u64;
    let mut skipped = 0u64;
    match kind {
        Catalog::Stars => {
            let Some(stride) = star_stride(bytes) else {
                return Err(format!(
                    "bin {} bytes carry no {}-byte star records — the density field stays unwritten",
                    bytes.len(),
                    STAR_RECORD_BYTES
                ));
            };
            for chunk in bytes.chunks_exact(stride) {
                records += 1;
                let Some(rec) = parse_star_record(chunk) else {
                    skipped += 1;
                    continue;
                };
                if place_pixel(counts, rec.ra_deg, rec.dec_deg) {
                    counted += 1;
                } else {
                    skipped += 1;
                }
            }
        }
        Catalog::Twomass => {
            let Some(body) = twomass_body(bytes) else {
                return Err(format!(
                    "bin {} bytes carry no 2MPS header ({}-byte records after {} bytes of header) — the density field stays unwritten",
                    bytes.len(),
                    TWOMASS_RECORD_BYTES,
                    TWOMASS_HEADER_BYTES
                ));
            };
            for chunk in body.chunks_exact(TWOMASS_RECORD_BYTES) {
                records += 1;
                let Some((ra_deg, dec_deg)) = twomass_radec(chunk) else {
                    skipped += 1;
                    continue;
                };
                if place_pixel(counts, ra_deg, dec_deg) {
                    counted += 1;
                } else {
                    skipped += 1;
                }
            }
        }
    }
    Ok((records, counted, skipped))
}

fn serialize_asset(nside: i64, counts: &[u64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + counts.len() * 8);
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&(nside as u32).to_le_bytes());
    for c in counts {
        out.extend_from_slice(&c.to_le_bytes());
    }
    out
}

fn parse_asset(b: &[u8]) -> Option<DensityAsset> {
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
    Some(DensityAsset { nside, counts })
}

fn run(args: &[String]) -> Result<(), String> {
    let inputs = collect_inputs(args)?;
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("--out <asset.vlde>: the asset path is never silent — refused".into());
    };
    let ci_mode = has_flag(args, "--ci-mode");

    let Some(npix) = npix_of(NSIDE) else {
        return Err(format!(
            "nside {NSIDE} carries no pixel count — the field stays unwritten"
        ));
    };
    let mut counts = vec![0u64; npix];
    let mut grand_records = 0u64;
    let mut grand_counted = 0u64;
    let mut grand_skipped = 0u64;

    for (kind, path) in &inputs {
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                if !path.starts_with("data/") {
                    return Err(format!("bin void {path}: {e}"));
                }
                let (netloc, asset) = cdn_asset(*kind);
                match ensure_bin(path, netloc, asset, BIN_TTL_S) {
                    Some(b) => b,
                    None => {
                        return Err(format!(
                            "bin void {path}: absent on disk and the CDN fetch of {asset} returned non-200"
                        ));
                    }
                }
            }
        };
        let (records, counted, skipped) = count_into(&mut counts, &bytes, *kind)?;
        grand_records += records;
        grand_counted += counted;
        grand_skipped += skipped;
        eprintln!(
            "vlies_density_compiler: {} {}: {} record(s), {} counted, {} skipped (refused by the record reader or unplaceable)",
            kind.name(),
            path,
            records,
            counted,
            skipped
        );
    }

    let total: u64 = counts.iter().sum();
    if total != grand_counted {
        return Err(format!(
            "{} record(s) counted across {} input(s) but the field sums to {total} — the asset stays unwritten",
            grand_counted,
            inputs.len()
        ));
    }
    let occupied = counts.iter().filter(|&&c| c > 0).count();

    let asset = serialize_asset(NSIDE, &counts);
    std::fs::write(&out_path, &asset)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;

    let back =
        std::fs::read(&out_path).map_err(|e| format!("read {out_path} returned void: {e}"))?;
    let field =
        parse_asset(&back).ok_or_else(|| format!("{out_path}: the asset does not read back"))?;
    if field.nside != NSIDE || field.counts != counts {
        return Err(format!(
            "{out_path}: roundtrip mismatch — the asset stays unverified"
        ));
    }

    eprintln!(
        "vlies_density_compiler: {} input(s), {} record(s) total, {} counted, {} skipped, {} occupied pixel(s) of {}",
        inputs.len(),
        grand_records,
        grand_counted,
        grand_skipped,
        occupied,
        field.counts.len()
    );
    eprintln!(
        "vlies_density_compiler: {out_path}: {} bytes written and read back — magic VLDE version {VERSION}, nside {NSIDE}, {} pixel count(s) of 8 bytes",
        asset.len(),
        field.counts.len()
    );
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("vlies_density_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::twomass::write_bin;

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

    fn pixel_of(nside: i64, ra: f64, dec: f64) -> i64 {
        ang2pix_nest(nside, (90.0 - dec).to_radians(), ra.to_radians()).unwrap()
    }

    fn twomass_bin(rows: &[[f64; 8]]) -> Vec<u8> {
        write_bin(rows)
    }

    #[test]
    fn counts_follow_the_directions() {
        let dirs = [
            (0.0, 0.0),
            (180.0, 0.0),
            (90.0, 60.0),
            (45.0, -30.0),
            (0.0, 90.0),
            (200.0, -80.0),
        ];
        let mut bin = Vec::new();
        for (ra, dec) in dirs {
            bin.extend(star_rec(ra, dec, 1.0));
            bin.extend(star_rec(ra, dec, 2.0));
        }
        let npix = (12 * NSIDE * NSIDE) as usize;
        let mut counts = vec![0u64; npix];
        let (records, counted, skipped) = count_into(&mut counts, &bin, Catalog::Stars).unwrap();
        assert_eq!(records, (dirs.len() * 2) as u64);
        assert_eq!(counted, (dirs.len() * 2) as u64);
        assert_eq!(skipped, 0);
        for (ra, dec) in dirs {
            let p = pixel_of(NSIDE, ra, dec) as usize;
            assert_eq!(counts[p], 2, "pixel {p} for ra {ra} dec {dec}");
        }
    }

    #[test]
    fn non_finite_and_refused_records_are_skipped() {
        let mut bin = star_rec(10.0, 10.0, 1.0);
        bin.extend(star_rec(f64::NAN, 10.0, 1.0));
        bin.extend(star_rec(10.0, f64::NEG_INFINITY, 1.0));
        bin.extend(star_rec(10.0, 10.0, -1.0));
        let npix = (12 * NSIDE * NSIDE) as usize;
        let mut counts = vec![0u64; npix];
        let (_, counted, skipped) = count_into(&mut counts, &bin, Catalog::Stars).unwrap();
        assert_eq!(counted, 1);
        assert_eq!(skipped, 3);
        let p = pixel_of(NSIDE, 10.0, 10.0) as usize;
        assert_eq!(counts[p], 1);
    }

    #[test]
    fn twomass_counts_follow_the_directions() {
        let rows = [
            [0.0, 0.0, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [180.0, 0.0, 12.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [90.0, 60.0, 13.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [45.0, -30.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 90.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];
        let bin = twomass_bin(&rows);
        assert_eq!(
            bin.len(),
            TWOMASS_HEADER_BYTES + rows.len() * TWOMASS_RECORD_BYTES
        );
        let npix = (12 * NSIDE * NSIDE) as usize;
        let mut counts = vec![0u64; npix];
        let (records, counted, skipped) = count_into(&mut counts, &bin, Catalog::Twomass).unwrap();
        assert_eq!(records, rows.len() as u64);
        assert_eq!(counted, rows.len() as u64);
        assert_eq!(skipped, 0);
        let p = pixel_of(NSIDE, 0.0, 0.0) as usize;
        assert_eq!(counts[p], 2);
        let p = pixel_of(NSIDE, 180.0, 0.0) as usize;
        assert_eq!(counts[p], 1);
        let p = pixel_of(NSIDE, 90.0, 60.0) as usize;
        assert_eq!(counts[p], 1);
    }

    #[test]
    fn twomass_non_finite_and_out_of_range_records_are_skipped() {
        let rows = [
            [10.0, 10.0, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [f64::NAN, 10.0, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [10.0, f64::INFINITY, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [400.0, 10.0, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [10.0, -95.0, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];
        let bin = twomass_bin(&rows);
        let npix = (12 * NSIDE * NSIDE) as usize;
        let mut counts = vec![0u64; npix];
        let (records, counted, skipped) = count_into(&mut counts, &bin, Catalog::Twomass).unwrap();
        assert_eq!(records, rows.len() as u64);
        assert_eq!(counted, 1);
        assert_eq!(skipped, 4);
        let p = pixel_of(NSIDE, 10.0, 10.0) as usize;
        assert_eq!(counts[p], 1);
    }

    #[test]
    fn twomass_structurally_void_bins_are_refused() {
        let npix = (12 * NSIDE * NSIDE) as usize;
        let mut counts = vec![0u64; npix];
        assert!(count_into(&mut counts, b"", Catalog::Twomass).is_err());
        assert!(count_into(&mut counts, b"XXXX", Catalog::Twomass).is_err());
        let mut wrong = twomass_bin(&[[10.0, 10.0, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0]]);
        wrong[0] = b'X';
        assert!(count_into(&mut counts, &wrong, Catalog::Twomass).is_err());
        let mut truncated = twomass_bin(&[
            [10.0, 10.0, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [20.0, 20.0, 12.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ]);
        truncated.truncate(truncated.len() - TWOMASS_RECORD_BYTES);
        assert!(count_into(&mut counts, &truncated, Catalog::Twomass).is_err());
    }

    #[test]
    fn catalog_kind_parse_known_and_unknown() {
        assert_eq!(Catalog::parse("stars"), Some(Catalog::Stars));
        assert_eq!(Catalog::parse("twomass"), Some(Catalog::Twomass));
        assert_eq!(Catalog::parse("tycho2"), None);
        assert_eq!(Catalog::parse("asteroids"), None);
        assert_eq!(Catalog::parse(""), None);
    }

    #[test]
    fn multiple_catalogs_share_one_field() {
        let dirs = [(0.0, 0.0), (180.0, 0.0), (90.0, 60.0)];
        let mut star_bin = Vec::new();
        for (ra, dec) in dirs {
            star_bin.extend(star_rec(ra, dec, 1.0));
        }
        let twomass_rows: Vec<[f64; 8]> = dirs
            .iter()
            .map(|&(ra, dec)| [ra, dec, 11.0, 0.0, 0.0, 0.0, 0.0, 0.0])
            .collect();
        let twomass_bin = twomass_bin(&twomass_rows);
        let npix = (12 * NSIDE * NSIDE) as usize;
        let mut counts = vec![0u64; npix];
        let (s_records, s_counted, s_skipped) =
            count_into(&mut counts, &star_bin, Catalog::Stars).unwrap();
        let (t_records, t_counted, t_skipped) =
            count_into(&mut counts, &twomass_bin, Catalog::Twomass).unwrap();
        assert_eq!(s_records, dirs.len() as u64);
        assert_eq!(s_counted, dirs.len() as u64);
        assert_eq!(s_skipped, 0);
        assert_eq!(t_records, dirs.len() as u64);
        assert_eq!(t_counted, dirs.len() as u64);
        assert_eq!(t_skipped, 0);
        for (ra, dec) in dirs {
            let p = pixel_of(NSIDE, ra, dec) as usize;
            assert_eq!(counts[p], 2, "pixel {p} for ra {ra} dec {dec}");
        }
        assert_eq!(counts.iter().sum::<u64>(), (dirs.len() * 2) as u64);
    }

    #[test]
    fn absent_catalog_is_bin_void_and_leaves_no_asset() {
        let missing = "/nonexistent/vlies/twomass_psc.bin";
        let out = "/nonexistent/vlies/never.vlde";
        let err = run(&[
            "--catalog".to_string(),
            "twomass".to_string(),
            missing.to_string(),
            "--out".to_string(),
            out.to_string(),
        ])
        .unwrap_err();
        assert!(err.starts_with("bin void "), "{err}");
        assert!(err.contains(missing), "{err}");
    }

    #[test]
    fn unknown_catalog_kind_is_refused() {
        let err = run(&[
            "--catalog".to_string(),
            "tycho2".to_string(),
            "x.bin".to_string(),
            "--out".to_string(),
            "y.vlde".to_string(),
        ])
        .unwrap_err();
        assert!(err.contains("unknown kind"), "{err}");
    }

    #[test]
    fn serialize_asset_roundtrips() {
        let mut counts = vec![0u64; (12 * NSIDE * NSIDE) as usize];
        let last = counts.len() - 1;
        counts[0] = 7;
        counts[last] = 3;
        let asset = serialize_asset(NSIDE, &counts);
        assert_eq!(asset.len(), HEADER_BYTES + counts.len() * 8);
        let field = parse_asset(&asset).unwrap();
        assert_eq!(field.nside, NSIDE);
        assert_eq!(field.counts, counts);
    }

    #[test]
    fn parse_refuses_void_and_foreign_assets() {
        assert!(parse_asset(&[]).is_none());
        assert!(parse_asset(b"VLDE\x01").is_none());
        let mut wrong = serialize_asset(NSIDE, &vec![0u64; (12 * NSIDE * NSIDE) as usize]);
        wrong[0] = b'X';
        assert!(parse_asset(&wrong).is_none());
        let mut foreign = serialize_asset(NSIDE, &vec![0u64; (12 * NSIDE * NSIDE) as usize]);
        foreign[4] = 9;
        assert!(parse_asset(&foreign).is_none());
        let truncated = serialize_asset(NSIDE, &vec![0u64; (12 * NSIDE * NSIDE) as usize]);
        assert!(parse_asset(&truncated[..truncated.len() - 5]).is_none());
    }

    #[test]
    fn npix_refuses_non_power_of_two() {
        assert!(npix_of(0).is_none());
        assert!(npix_of(3).is_none());
        assert!(npix_of(4294967296).is_none());
        assert_eq!(npix_of(128), Some((12 * 128 * 128) as usize));
    }
}
