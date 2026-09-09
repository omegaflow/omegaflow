use omegaflow::archivar::fits::{FitsHeader, FitsImage, FitsWcs, WcsProjection};
use omegaflow::archivar::footprint::{
    band_code, decode_rec, encode_rec, parse_header, write_header, FootprintBand, FootprintRecord,
    HEADER_LEN, REC_BYTES,
};
use omegaflow::archivar::regrid::ZenithalRegrid;
use omegaflow::cdn::upload_asset;
use omegaflow::inflate::gunzip;
use omegaflow::zeuge::{magic_identity, FeldIdentitaet};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const TAP_ROOT: &str = "https://irsa.ipac.caltech.edu/TAP/sync";
const COV_ROOT: &str = "https://irsa.ipac.caltech.edu/ibe/data/wise/allwise/p3am_cdd";
const TAP_TABLE: &str = "wise.wise_allwise_p3am_cdd";
const NSIDE: i64 = 1024;
const NPIX: i64 = 12 * NSIDE * NSIDE;
const ORDER: u8 = NSIDE.trailing_zeros() as u8;
const HP_SR: f64 = 4.0 * PI / (NPIX as f64);
const BAND_COUNT: usize = 4;

const BANDS: [(u8, FootprintBand); BAND_COUNT] = [
    (1, FootprintBand::W1),
    (2, FootprintBand::W2),
    (3, FootprintBand::W3),
    (4, FootprintBand::W4),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn footprint_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Footprint) => {
            eprintln!(
                "{} reads as a survey-footprint asset (sibling of the witnesses, not a witness)",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not a footprint — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn cov_url(coadd_id: &str, band: u8) -> String {
    format!(
        "{COV_ROOT}/{}/{}/{}/{}-w{}-cov-3.fits.gz",
        &coadd_id[..2],
        &coadd_id[..4],
        coadd_id,
        coadd_id,
        band
    )
}

fn band_index(band: FootprintBand) -> usize {
    BANDS
        .iter()
        .position(|(_, b)| *b == band)
        .expect("every footprint band in the allwise set")
}

fn tap_csv(adql: &str, maxrec: u32) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("600")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("MAXREC={maxrec}"))
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg(TAP_ROOT)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap_csv http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn column_index(header: &str, name: &str) -> Option<usize> {
    header.split(',').map(|c| c.trim()).position(|c| c == name)
}

fn parse_coadd_csv(body: &str) -> Vec<String> {
    let mut lines = body.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let Some(idx) = column_index(header, "coadd_id") else {
        return Vec::new();
    };
    let mut ids = Vec::new();
    for line in lines {
        let Some(cell) = line.split(',').map(|c| c.trim()).nth(idx) else {
            continue;
        };
        if !cell.is_empty() {
            ids.push(cell.to_string());
        }
    }
    ids
}

fn enumerate_coadds() -> Option<Vec<String>> {
    let adql = format!("SELECT coadd_id FROM {TAP_TABLE} GROUP BY coadd_id");
    let body = tap_csv(&adql, 20000)?;
    let ids = parse_coadd_csv(&body);
    if ids.is_empty() {
        None
    } else {
        Some(ids)
    }
}

fn nominal_depth_per_band() -> Option<[f64; BAND_COUNT]> {
    let adql = format!("SELECT band, MAX(maxcov) FROM {TAP_TABLE} GROUP BY band");
    let body = tap_csv(&adql, 100)?;
    let mut lines = body.lines();
    lines.next()?;
    let mut nominal = [0.0f64; BAND_COUNT];
    for line in lines {
        let cells: Vec<&str> = line.split(',').collect();
        let band: u8 = cells.first()?.trim().parse().ok()?;
        let v: f64 = cells.get(1)?.trim().parse().ok()?;
        if v.is_finite() && v > 0.0 && (1..=4).contains(&band) {
            nominal[band as usize - 1] = v;
        }
    }
    if nominal.iter().all(|v| *v > 0.0) {
        Some(nominal)
    } else {
        None
    }
}

fn validate_coadd(coadd_id: &str) -> Result<(), String> {
    if coadd_id.len() < 4 {
        return Err(format!(
            "{coadd_id}: a coadd id needs at least four characters (the path holds its first two and four) — refused"
        ));
    }
    if coadd_id
        .bytes()
        .any(|b| !b.is_ascii_alphanumeric() && b != b'_')
    {
        return Err(format!(
            "{coadd_id}: the path would escape the measured route — refused"
        ));
    }
    Ok(())
}

enum FetchOutcome {
    Bytes(Vec<u8>),
    Missing,
    Refused(String),
}

fn curl_bytes(url: &str) -> FetchOutcome {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("600")
        .arg(url)
        .output();
    match out {
        Ok(o) if o.status.success() => FetchOutcome::Bytes(o.stdout),
        Ok(o) => {
            let msg = String::from_utf8_lossy(&o.stderr).trim().to_string();
            if msg.contains("404") {
                FetchOutcome::Missing
            } else {
                FetchOutcome::Refused(msg)
            }
        }
        Err(e) => FetchOutcome::Refused(e.to_string()),
    }
}

struct Plane {
    max_count: f64,
    measured: u64,
    zero_pixels: u64,
    absent_pixels: u64,
    accum: HashMap<u32, f64>,
}

struct Census {
    tap_queries: u64,
    tiles_harvested: u64,
    planes_fetched: u64,
    planes_missing: u64,
    planes_unreadable: u64,
    planes_non_zenithal: u64,
    planes_empty: u64,
    measured_pixels: u64,
    zero_pixels: u64,
    absent_pixels: u64,
    unmapped_pixels: u64,
    nominal_from_tap: bool,
}

impl Census {
    fn empty() -> Self {
        Census {
            tap_queries: 0,
            tiles_harvested: 0,
            planes_fetched: 0,
            planes_missing: 0,
            planes_unreadable: 0,
            planes_non_zenithal: 0,
            planes_empty: 0,
            measured_pixels: 0,
            zero_pixels: 0,
            absent_pixels: 0,
            unmapped_pixels: 0,
            nominal_from_tap: false,
        }
    }

    fn merge(&mut self, other: Census) {
        self.tap_queries += other.tap_queries;
        self.tiles_harvested += other.tiles_harvested;
        self.planes_fetched += other.planes_fetched;
        self.planes_missing += other.planes_missing;
        self.planes_unreadable += other.planes_unreadable;
        self.planes_non_zenithal += other.planes_non_zenithal;
        self.planes_empty += other.planes_empty;
        self.measured_pixels += other.measured_pixels;
        self.zero_pixels += other.zero_pixels;
        self.absent_pixels += other.absent_pixels;
        self.unmapped_pixels += other.unmapped_pixels;
        self.nominal_from_tap |= other.nominal_from_tap;
    }
}

fn regrid_plane(buf: &[u8], census: &mut Census) -> Option<Plane> {
    let raw = gunzip(buf)?;
    let (header, _) = FitsHeader::parse(&raw, 0)?;
    let (image, _) = FitsImage::parse(&raw, 0)?;
    let width = image.dims[0];
    let height = image.dims[1];
    if image.dims[2] != 1 {
        census.planes_unreadable += 1;
        eprintln!(
            "regrid_plane: an image shape outside the measured 4095x4095 plane stayed unharvested"
        );
        return None;
    }
    let wcs = FitsWcs::from_header(&header, width, height)?;
    let projection = wcs.projection();
    if projection != WcsProjection::Tan && projection != WcsProjection::Sin {
        census.planes_non_zenithal += 1;
        eprintln!(
            "regrid_plane: a {projection:?}-projection cov plane stayed unharvested (measured sky is not fabricated)"
        );
        return None;
    }
    let mut regrid = ZenithalRegrid::new(wcs, NSIDE)?;
    let mut max_count: f64 = 0.0;
    let mut measured: u64 = 0;
    let mut zero_pixels: u64 = 0;
    let mut absent_pixels: u64 = 0;
    for y in 0..height {
        let mut row_v = vec![0.0f64; width];
        let mut row_m = vec![false; width];
        for x in 0..width {
            let v = image.value_f64(&raw, [x, y, 0]).unwrap_or(f64::NAN);
            if v.is_finite() && v >= 0.0 {
                row_m[x] = true;
                row_v[x] = v;
                measured += 1;
                if v == 0.0 {
                    zero_pixels += 1;
                } else if v > max_count {
                    max_count = v;
                }
            } else {
                absent_pixels += 1;
            }
        }
        regrid.push_row(y, &row_v, &row_m);
    }
    let unmapped = regrid.unmapped_pixels();
    let accum: HashMap<u32, f64> = regrid
        .into_accum()
        .into_iter()
        .map(|(k, a)| (k, a.count_sr))
        .collect();
    if accum.is_empty() {
        census.planes_empty += 1;
        eprintln!("regrid_plane: the cov plane carries no measured coverage");
        return None;
    }
    census.unmapped_pixels += unmapped;
    Some(Plane {
        max_count,
        measured,
        zero_pixels,
        absent_pixels,
        accum,
    })
}

fn harvest_coadd(
    coadd_id: &str,
    band_accum: &mut [HashMap<u32, f64>; BAND_COUNT],
    band_max: &mut [f64; BAND_COUNT],
    census: &mut Census,
) {
    census.tiles_harvested += 1;
    for (number, band) in BANDS.iter() {
        let url = cov_url(coadd_id, *number);
        match curl_bytes(&url) {
            FetchOutcome::Bytes(buf) => {
                census.planes_fetched += 1;
                match regrid_plane(&buf, census) {
                    Some(plane) => {
                        let bi = band_index(*band);
                        let dest = &mut band_accum[bi];
                        for (ipix, count_sr) in plane.accum {
                            *dest.entry(ipix).or_insert(0.0) += count_sr;
                        }
                        if plane.max_count > band_max[bi] {
                            band_max[bi] = plane.max_count;
                        }
                        census.measured_pixels += plane.measured;
                        census.zero_pixels += plane.zero_pixels;
                        census.absent_pixels += plane.absent_pixels;
                        eprintln!(
                            "  {coadd_id} w{number} cov plane: {} measured pixels ({} zero, {} absent), max effective pixels {}",
                            plane.measured, plane.zero_pixels, plane.absent_pixels, plane.max_count
                        );
                    }
                    None => {
                        eprintln!(
                            "  {coadd_id} w{number} cov plane stayed unread — its measured sky is not harvested"
                        );
                    }
                }
            }
            FetchOutcome::Missing => {
                census.planes_missing += 1;
                eprintln!("  {coadd_id} w{number} cov plane is absent (the band has no cov here)");
            }
            FetchOutcome::Refused(msg) => {
                census.planes_unreadable += 1;
                eprintln!("  {coadd_id} w{number} cov plane fetch returned void: {msg}");
            }
        }
    }
}

fn harvest_parallel(
    ids: &[String],
) -> Result<([HashMap<u32, f64>; BAND_COUNT], [f64; BAND_COUNT], Census), String> {
    let workers = 4usize;
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(workers);
        for _ in 0..workers {
            let ids = ids;
            let next = &next;
            handles.push(s.spawn(move || {
                let mut accum: [HashMap<u32, f64>; BAND_COUNT] =
                    std::array::from_fn(|_| HashMap::new());
                let mut band_max = [0.0f64; BAND_COUNT];
                let mut census = Census::empty();
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    if i >= ids.len() {
                        break;
                    }
                    eprintln!("coadd {}", ids[i]);
                    harvest_coadd(&ids[i], &mut accum, &mut band_max, &mut census);
                }
                (accum, band_max, census)
            }));
        }
        let mut total_accum: [HashMap<u32, f64>; BAND_COUNT] =
            std::array::from_fn(|_| HashMap::new());
        let mut total_max = [0.0f64; BAND_COUNT];
        let mut total_census = Census::empty();
        for h in handles {
            let (a, m, c) = h
                .join()
                .map_err(|_| "a harvest worker stayed unjoined".to_string())?;
            for (bi, map) in a.into_iter().enumerate() {
                for (ipix, v) in map {
                    *total_accum[bi].entry(ipix).or_insert(0.0) += v;
                }
                if m[bi] > total_max[bi] {
                    total_max[bi] = m[bi];
                }
            }
            total_census.merge(c);
        }
        Ok((total_accum, total_max, total_census))
    })
}

fn run(args: &[String]) -> Result<(), String> {
    footprint_identity(omegaflow::archivar::footprint::MAGIC)?;
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => "allwise_coverage.fp01".to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let limit: Option<u64> = arg_value(args, "--limit").and_then(|s| s.parse().ok());
    let coadd_override: Option<String> = arg_value(args, "--coadd");
    let coadd_lo: Option<usize> = match arg_value(args, "--coadd-lo") {
        Some(s) => Some(
            s.parse::<usize>()
                .map_err(|_| format!("--coadd-lo {s} reads no index"))?,
        ),
        None => None,
    };
    let coadd_hi: Option<usize> = match arg_value(args, "--coadd-hi") {
        Some(s) => Some(
            s.parse::<usize>()
                .map_err(|_| format!("--coadd-hi {s} reads no index"))?,
        ),
        None => None,
    };
    let coadd_range: Option<(usize, usize)> = match (coadd_lo, coadd_hi) {
        (Some(lo), Some(hi)) => Some((lo, hi)),
        (None, None) => None,
        _ => {
            return Err("--coadd-lo and --coadd-hi name the inclusive tile span together — one without the other is refused".into())
        }
    };

    if let Some(id) = &coadd_override {
        validate_coadd(id)?;
    }

    let mut census = Census {
        tap_queries: 0,
        tiles_harvested: 0,
        planes_fetched: 0,
        planes_missing: 0,
        planes_unreadable: 0,
        planes_non_zenithal: 0,
        planes_empty: 0,
        measured_pixels: 0,
        zero_pixels: 0,
        absent_pixels: 0,
        unmapped_pixels: 0,
        nominal_from_tap: false,
    };

    let mut file = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, 0);
    file.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} header returned void: {e}"))?;

    let nominal = {
        census.tap_queries += 1;
        match nominal_depth_per_band() {
            Some(n) => {
                census.nominal_from_tap = true;
                eprintln!(
                    "survey nominal effective-pixel depth per band (TAP maxcov maximum, {TAP_TABLE}): w1 {} w2 {} w3 {} w4 {}",
                    n[0], n[1], n[2], n[3]
                );
                Some(n)
            }
            None => {
                eprintln!(
                    "the TAP nominal-depth query stayed void — the nominal will be the harvested-plane maximum (named choice, biased under --limit)"
                );
                None
            }
        }
    };

    let mut ids: Vec<String> = match coadd_override {
        Some(id) => vec![id],
        None => {
            census.tap_queries += 1;
            let mut ids = enumerate_coadds()
                .ok_or("the coadd enumeration stayed void — nothing harvested")?;
            ids.sort();
            eprintln!("coadd enumeration: {} tiles (sorted)", ids.len());
            ids
        }
    };
    if let Some((lo, hi)) = coadd_range {
        if lo > hi || hi >= ids.len() {
            return Err(format!(
                "--coadd-lo {lo} --coadd-hi {hi} lies outside the {}-tile enumeration — refused",
                ids.len()
            ));
        }
        ids = ids[lo..=hi].to_vec();
    } else if let Some(n) = limit {
        ids.truncate(n as usize);
    }

    let (band_accum, band_max, harvest_census) = harvest_parallel(&ids)?;
    census.merge(harvest_census);

    let nominal: [f64; BAND_COUNT] = match nominal {
        Some(n) => n,
        None => {
            census.nominal_from_tap = false;
            eprintln!(
                "nominal depth per band (empirical max over the harvested cov planes): w1 {} w2 {} w3 {} w4 {}",
                band_max[0], band_max[1], band_max[2], band_max[3]
            );
            band_max
        }
    };

    let nominal_missing = nominal.iter().all(|n| *n <= 0.0);
    let mut total: u64 = 0;
    let mut rec = [0u8; REC_BYTES];
    if !nominal_missing {
        let mut records: Vec<FootprintRecord> = Vec::new();
        for (bi, n) in nominal.iter().enumerate() {
            if *n <= 0.0 {
                continue;
            }
            for (&ipix, &count_sr) in band_accum[bi].iter() {
                let frac = (count_sr / (*n * HP_SR)).clamp(0.0, 1.0) as f32;
                records.push(FootprintRecord {
                    order: ORDER,
                    band: BANDS[bi].1,
                    ipix,
                    frac,
                });
            }
        }
        records.sort_by(|a, b| {
            a.ipix
                .cmp(&b.ipix)
                .then(band_code(a.band).cmp(&band_code(b.band)))
                .then(a.frac.to_bits().cmp(&b.frac.to_bits()))
        });
        let mut buf = Vec::with_capacity(records.len() * REC_BYTES);
        for r in &records {
            encode_rec(&mut rec, r);
            buf.extend_from_slice(&rec);
        }
        file.write_all(&buf)
            .map_err(|e| format!("write {out_path} records returned void: {e}"))?;
        total = records.len() as u64;
    }

    if total == 0 {
        return Err("no coverage record harvested — the asset stays unwritten (0 honored)".into());
    }

    file.seek(SeekFrom::Start(5))
        .map_err(|e| format!("seek {out_path} header returned void: {e}"))?;
    file.write_all(&total.to_le_bytes())
        .map_err(|e| format!("patch {out_path} header returned void: {e}"))?;
    file.flush()
        .map_err(|e| format!("flush {out_path} returned void: {e}"))?;

    let expect = HEADER_LEN as u64 + total * REC_BYTES as u64;
    let actual = std::fs::metadata(&out_path)
        .map_err(|e| format!("stat {out_path} returned void: {e}"))?
        .len();
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }

    let mut vf = std::fs::File::open(&out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut head = [0u8; HEADER_LEN];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {out_path} header returned void: {e}"))?;
    let n_rows =
        parse_header(&head).ok_or_else(|| format!("{out_path}: the header stays unread"))?;
    if n_rows != total {
        return Err(format!(
            "{out_path}: header {n_rows} rows, {total} written — the asset stays unwritten"
        ));
    }
    let last_off = HEADER_LEN as u64 + (total - 1) * REC_BYTES as u64;
    vf.seek(SeekFrom::Start(last_off))
        .map_err(|e| format!("seek {out_path} tail returned void: {e}"))?;
    let mut tail = vec![0u8; REC_BYTES];
    vf.read_exact(&mut tail)
        .map_err(|e| format!("read {out_path} tail returned void: {e}"))?;
    let last =
        decode_rec(&tail).ok_or_else(|| format!("{out_path}: the last record stays unread"))?;

    eprintln!(
        "{out_path}: {} records over {} coadd tiles, header {} — roundtrip verified",
        total, census.tiles_harvested, n_rows
    );
    eprintln!(
        "nominal depth per band ({}): w1 {} w2 {} w3 {} w4 {}",
        if census.nominal_from_tap {
            "survey max from the TAP maxcov column"
        } else {
            "empirical max over the harvested cov planes"
        },
        nominal[0],
        nominal[1],
        nominal[2],
        nominal[3]
    );
    eprintln!(
        "last record: order {} band {:?} ipix {} frac {:.6}",
        last.order, last.band, last.ipix, last.frac
    );
    eprintln!(
        "census: {} tap queries, {} coadd tiles, {} cov planes fetched, {} absent planes, {} unreadable planes, {} non-zenithal planes, {} empty planes, {} measured pixels, {} real-zero pixels, {} absent pixels, {} unmapped",
        census.tap_queries,
        census.tiles_harvested,
        census.planes_fetched,
        census.planes_missing,
        census.planes_unreadable,
        census.planes_non_zenithal,
        census.planes_empty,
        census.measured_pixels,
        census.zero_pixels,
        census.absent_pixels,
        census.unmapped_pixels
    );
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("wise_coverage_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cov_url_embeds_the_measured_route() {
        assert_eq!(
            cov_url("2417p302_ac51", 1),
            "https://irsa.ipac.caltech.edu/ibe/data/wise/allwise/p3am_cdd/24/2417/2417p302_ac51/2417p302_ac51-w1-cov-3.fits.gz"
        );
    }

    #[test]
    fn band_index_covers_one_to_four() {
        for (number, band) in BANDS.iter() {
            assert_eq!(BANDS[band_index(*band)], (*number, *band));
        }
        assert_eq!(BANDS[band_index(FootprintBand::W1)], (1, FootprintBand::W1));
        assert_eq!(BANDS[band_index(FootprintBand::W2)], (2, FootprintBand::W2));
        assert_eq!(BANDS[band_index(FootprintBand::W3)], (3, FootprintBand::W3));
        assert_eq!(BANDS[band_index(FootprintBand::W4)], (4, FootprintBand::W4));
    }

    #[test]
    fn nominal_normalization_clamps_to_one() {
        let frac = (2.0 * 5.0 * HP_SR / (5.0 * HP_SR)).clamp(0.0, 1.0);
        assert_eq!(frac, 1.0);
        let half = (1.0 * HP_SR / (4.0 * HP_SR)).clamp(0.0, 1.0);
        assert_eq!(half, 0.25);
    }

    #[test]
    fn parse_coadd_csv_reads_the_measured_header() {
        let body = "coadd_id\n2417p302_ac51\n2434p302_ac51\n0000p000\n";
        assert_eq!(
            parse_coadd_csv(body),
            vec!["2417p302_ac51", "2434p302_ac51", "0000p000"]
        );
        assert!(parse_coadd_csv("other_col\nx\n").is_empty());
    }

    #[test]
    fn validate_coadd_refuses_an_escaping_id() {
        assert!(validate_coadd("2417p302_ac51").is_ok());
        assert!(validate_coadd("ab").is_err());
        assert!(validate_coadd("../etc").is_err());
        assert!(validate_coadd("a/bc").is_err());
    }
}
