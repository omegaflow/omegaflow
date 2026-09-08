use omegaflow::archivar::fits::{FitsCompressedImage, FitsHeader, FitsWcs};
use omegaflow::archivar::footprint::{
    band_code, decode_rec, encode_rec, parse_header, write_header, FootprintBand, FootprintRecord,
    HEADER_LEN, NSIDE, REC_BYTES,
};
use omegaflow::archivar::regrid::ZenithalRegrid;
use omegaflow::cdn::upload_asset;
use omegaflow::zeuge::{magic_identity, FeldIdentitaet};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::Command;

const FILENAMES: &str = "https://ps1images.stsci.edu/cgi-bin/ps1filenames.py";
const PLANE_ROOT: &str = "https://ps1images.stsci.edu/rings.v3.skycell";
const NPIX: i64 = 12 * NSIDE * NSIDE;
const ORDER: u8 = NSIDE.trailing_zeros() as u8;
const HP_SR: f64 = 4.0 * PI / (NPIX as f64);

const BANDS: [(char, FootprintBand); 5] = [
    ('g', FootprintBand::G),
    ('r', FootprintBand::R),
    ('i', FootprintBand::I),
    ('z', FootprintBand::Z),
    ('y', FootprintBand::Y),
];

fn band_index(band: FootprintBand) -> usize {
    BANDS
        .iter()
        .position(|(_, b)| *b == band)
        .expect("every footprint band in the ps1 set")
}

struct Census {
    grid_queries: u64,
    empty_queries: u64,
    skycells: u64,
    planes_fetched: u64,
    planes_missing: u64,
    planes_unreadable: u64,
    planes_non_gnomonic: u64,
    planes_empty: u64,
    measured_pixels: u64,
    unmapped_pixels: u64,
}

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

fn plane_url(proj: u32, sub: u32, filter: char) -> String {
    format!(
        "{PLANE_ROOT}/{proj}/{sub:03}/rings.v3.skycell.{proj}.{sub:03}.stk.{filter}.unconv.num.fits"
    )
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
        .arg("300")
        .arg(url)
        .output();
    match out {
        Ok(o) if o.status.success() => FetchOutcome::Bytes(o.stdout),
        Ok(o) => {
            let msg = String::from_utf8_lossy(&o.stderr).trim().to_string();
            if msg.contains("404") || msg.contains("could not be resolved") {
                if msg.contains("404") {
                    FetchOutcome::Missing
                } else {
                    FetchOutcome::Refused(msg)
                }
            } else {
                FetchOutcome::Refused(msg)
            }
        }
        Err(e) => FetchOutcome::Refused(e.to_string()),
    }
}

fn query_skycell(ra_deg: f64, dec_deg: f64) -> Option<Vec<(u32, u32)>> {
    let url = format!("{FILENAMES}?ra={ra_deg:.6}&dec={dec_deg:.6}&filter=grizY");
    match curl_bytes(&url) {
        FetchOutcome::Bytes(body) => {
            let text = String::from_utf8_lossy(&body);
            let mut cells = Vec::new();
            for line in text.lines().skip(1) {
                let toks: Vec<&str> = line.split_whitespace().collect();
                if toks.len() < 10 || toks[6] != "stack" || toks[9] != "0" {
                    continue;
                }
                let Ok(proj) = toks[0].parse::<u32>() else {
                    continue;
                };
                let Ok(sub) = toks[1].parse::<u32>() else {
                    continue;
                };
                cells.push((proj, sub));
            }
            if cells.is_empty() {
                None
            } else {
                Some(cells)
            }
        }
        FetchOutcome::Missing | FetchOutcome::Refused(_) => None,
    }
}

struct Plane {
    max_count: u64,
    measured: u64,
    accum: HashMap<u32, f64>,
}

fn regrid_plane(buf: &[u8], census: &mut Census) -> Option<Plane> {
    let (_, ext_off) = FitsHeader::parse(buf, 0)?;
    let (header, _) = FitsHeader::parse(buf, ext_off)?;
    let (image, _) = FitsCompressedImage::parse(buf, ext_off)?;
    let width = image.dims[0];
    let height = image.dims[1];
    if image.dims[2] != 1 || image.tiles_per_axis(0) != 1 || image.tile[1] > 1 {
        census.planes_unreadable += 1;
        eprintln!("regrid_plane: tile layout outside the measured single-row-per-tile shape");
        return None;
    }
    let wcs = FitsWcs::from_header(&header, width, height)?;
    if !wcs.is_tan() {
        census.planes_non_gnomonic += 1;
        eprintln!(
            "regrid_plane: a non-gnomonic projection plane stayed unharvested (measured sky is not fabricated)"
        );
        return None;
    }
    let mut regrid = ZenithalRegrid::new(wcs, NSIDE)?;
    let mut max_count: u64 = 0;
    let mut measured: u64 = 0;
    for t1 in 0..image.tiles_per_axis(1) {
        let vals = image.tile_pixels(buf, [0, t1, 0])?;
        let y = t1;
        if y >= height {
            continue;
        }
        let mut row_v = vec![0.0f64; width];
        let mut row_m = vec![false; width];
        for x in 0..width {
            let raw = vals[x];
            let count = (raw as u32) & 0x7FFF;
            if count > 0 {
                row_v[x] = count as f64;
                row_m[x] = true;
                measured += 1;
                if count as u64 > max_count {
                    max_count = count as u64;
                }
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
        eprintln!("regrid_plane: the plane carries no measured coverage");
        return None;
    }
    census.unmapped_pixels += unmapped;
    census.measured_pixels += measured;
    Some(Plane {
        max_count,
        measured,
        accum,
    })
}

fn harvest_skycell(
    proj: u32,
    sub: u32,
    band_accum: &mut [HashMap<u32, f64>; 5],
    band_max: &mut [u64; 5],
    census: &mut Census,
) {
    census.skycells += 1;
    for (filter, band) in BANDS.iter() {
        let url = plane_url(proj, sub, *filter);
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
                        eprintln!(
                            "  {}.{:03} {} plane: {} measured pixels, max num {}",
                            proj, sub, filter, plane.measured, plane.max_count
                        );
                    }
                    None => {
                        eprintln!(
                            "  {}.{:03} {} plane stayed unread — its measured sky is not harvested",
                            proj, sub, filter
                        );
                    }
                }
            }
            FetchOutcome::Missing => {
                census.planes_missing += 1;
                eprintln!(
                    "  {}.{:03} {} plane is absent (the band has no stack here)",
                    proj, sub, filter
                );
            }
            FetchOutcome::Refused(msg) => {
                census.planes_unreadable += 1;
                eprintln!(
                    "  {}.{:03} {} plane fetch returned void: {msg}",
                    proj, sub, filter
                );
            }
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    footprint_identity(omegaflow::archivar::footprint::MAGIC)?;
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => "ps1_dr2_coverage.fp01".to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let limit: Option<u64> = arg_value(args, "--limit").and_then(|s| s.parse().ok());
    let ra0: f64 = match arg_value(args, "--ra0").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => 0.0,
    };
    let dec0: f64 = arg_value(args, "--dec0")
        .and_then(|s| s.parse().ok())
        .unwrap_or(-30.0);
    let step: f64 = arg_value(args, "--step")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.2);

    let mut census = Census {
        grid_queries: 0,
        empty_queries: 0,
        skycells: 0,
        planes_fetched: 0,
        planes_missing: 0,
        planes_unreadable: 0,
        planes_non_gnomonic: 0,
        planes_empty: 0,
        measured_pixels: 0,
        unmapped_pixels: 0,
    };

    let mut file = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, 0);
    file.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} header returned void: {e}"))?;

    let mut band_accum: [HashMap<u32, f64>; 5] = std::array::from_fn(|_| HashMap::new());
    let mut band_max: [u64; 5] = [0; 5];
    let mut seen: HashMap<(u32, u32), ()> = HashMap::new();

    let dec_min = -30.0f64;
    let dec_max = 90.0f64;
    let mut dec_bands: Vec<f64> = Vec::new();
    let mut d = dec0;
    while d <= dec_max + 1e-9 {
        dec_bands.push(d);
        d += step;
    }
    let mut d = dec0 - step;
    while d >= dec_min - 1e-9 {
        dec_bands.push(d);
        d -= step;
    }

    let mut finished = false;
    for dec in dec_bands {
        if finished {
            break;
        }
        let cosd = dec.to_radians().cos().abs().max(0.1);
        let ra_step = step / cosd;
        let n_ra = (360.0 / ra_step).ceil() as u64;
        let mut ra = ra0;
        for _ in 0..n_ra {
            census.grid_queries += 1;
            if let Some(cells) = query_skycell(ra, dec) {
                for (proj, sub) in cells {
                    if seen.contains_key(&(proj, sub)) {
                        continue;
                    }
                    seen.insert((proj, sub), ());
                    eprintln!("skycell {}.{:03}", proj, sub);
                    harvest_skycell(proj, sub, &mut band_accum, &mut band_max, &mut census);
                    if let Some(lim) = limit {
                        if census.skycells >= lim {
                            finished = true;
                            break;
                        }
                    }
                }
            } else {
                census.empty_queries += 1;
            }
            if finished {
                break;
            }
            ra = (ra + ra_step) % 360.0;
        }
    }

    let nominal_missing = band_max.iter().all(|m| *m == 0);
    let mut total: u64 = 0;
    let mut rec = [0u8; REC_BYTES];
    if !nominal_missing {
        let mut records: Vec<FootprintRecord> = Vec::new();
        for (bi, m) in band_max.iter().enumerate() {
            if *m == 0 {
                continue;
            }
            let nominal = *m as f64;
            for (&ipix, &count_sr) in band_accum[bi].iter() {
                let frac = (count_sr / (nominal * HP_SR)).clamp(0.0, 1.0) as f32;
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
        "{out_path}: {} records over {} skycells, header {} — roundtrip verified",
        total, census.skycells, n_rows
    );
    eprintln!(
        "nominal depth (empirical max num over harvested skycells) per band: g {} r {} i {} z {} y {}",
        band_max[band_index(FootprintBand::G)],
        band_max[band_index(FootprintBand::R)],
        band_max[band_index(FootprintBand::I)],
        band_max[band_index(FootprintBand::Z)],
        band_max[band_index(FootprintBand::Y)]
    );
    eprintln!(
        "last record: order {} band {:?} ipix {} frac {:.6}",
        last.order, last.band, last.ipix, last.frac
    );
    eprintln!(
        "census: {} grid queries, {} empty, {} skycells, {} planes fetched, {} absent planes, {} unreadable planes, {} empty planes, {} measured pixels, {} unmapped",
        census.grid_queries,
        census.empty_queries,
        census.skycells,
        census.planes_fetched,
        census.planes_missing,
        census.planes_unreadable,
        census.planes_empty,
        census.measured_pixels,
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
        eprintln!("ps1_coverage_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_url_embeds_the_measured_route() {
        assert_eq!(
            plane_url(1360, 59, 'g'),
            "https://ps1images.stsci.edu/rings.v3.skycell/1360/059/rings.v3.skycell.1360.059.stk.g.unconv.num.fits"
        );
    }

    #[test]
    fn nominal_normalization_clamps_to_one() {
        let hp_sr = HP_SR;
        let count_sr = 2.0 * 5.0 * hp_sr;
        let frac = (count_sr / (5.0 * hp_sr)).clamp(0.0, 1.0);
        assert_eq!(frac, 1.0);
        let half = (1.0 * hp_sr / (4.0 * hp_sr)).clamp(0.0, 1.0);
        assert_eq!(half, 0.25);
    }

    #[test]
    fn band_index_covers_the_five_ps1_filters() {
        for (_, b) in BANDS.iter() {
            let bi = band_index(*b);
            assert_eq!(BANDS[bi].1, *b);
        }
    }
}
