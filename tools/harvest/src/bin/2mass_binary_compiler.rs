use omegaflow::archivar::footprint::{
    decode_rec, encode_rec, parse_header, write_header, FootprintBand, FootprintRecord, HEADER_LEN,
    MAGIC, REC_BYTES,
};
use omegaflow::cdn::upload_asset;
use omegaflow::healpix::pix2ang_nest;
use omegaflow::zeuge::{magic_identity, FeldIdentitaet};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::process::Command;

const TAP_ROOT: &str = "https://irsa.ipac.caltech.edu/TAP/sync";
const TAP_TABLE: &str = "fp_scan_dat";
const NSIDE: i64 = 256;
const ORDER: u8 = NSIDE.trailing_zeros() as u8;
const NPIX: i64 = 12 * NSIDE * NSIDE;
const BANDS: [FootprintBand; 3] = [FootprintBand::J, FootprintBand::H, FootprintBand::Ks];
const EXPECTED_TILES: usize = 59_731;
const ZONE_LO: f64 = -90.0;
const ZONE_BINS: usize = 1800;
const ZONE_BW: f64 = 0.1;

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

#[derive(Debug, Clone, Copy)]
struct Tile {
    ra: f64,
    dec: f64,
    cos_dec: f64,
    hx_sky: f64,
    hy_sky: f64,
}

impl Tile {
    fn contains(&self, ra: f64, dec: f64) -> bool {
        if (dec - self.dec).abs() > self.hy_sky {
            return false;
        }
        let mut dra = ra - self.ra;
        dra = (dra + 540.0).rem_euclid(360.0) - 180.0;
        dra.abs() * self.cos_dec <= self.hx_sky
    }
}

fn tile_from(ra: f64, dec: f64, corners: [(f64, f64); 4]) -> Option<Tile> {
    if !(ra.is_finite() && dec.is_finite() && (-90.0..=90.0).contains(&dec)) {
        return None;
    }
    let cos_dec = dec.to_radians().cos();
    let mut hx = 0.0f64;
    let mut hy = 0.0f64;
    for (cra, cdec) in corners {
        if !(cra.is_finite() && cdec.is_finite() && (-90.0..=90.0).contains(&cdec)) {
            return None;
        }
        hy = hy.max((cdec - dec).abs());
        let mut dra = cra - ra;
        dra = (dra + 540.0).rem_euclid(360.0) - 180.0;
        hx = hx.max(dra.abs() * cos_dec);
    }
    if hy <= 0.0 {
        return None;
    }
    Some(Tile {
        ra,
        dec,
        cos_dec,
        hx_sky: hx,
        hy_sky: hy,
    })
}

fn enumerate_tiles() -> Result<Vec<Tile>, String> {
    let adql = format!(
        "SELECT ra, dec, ra_1, dec_1, ra_2, dec_2, ra_3, dec_3, ra_4, dec_4 FROM {TAP_TABLE}"
    );
    let body = tap_csv(&adql, 60000).ok_or("the fp_scan_dat enumeration stayed void")?;
    let mut lines = body.lines();
    let header = lines
        .next()
        .ok_or("the fp_scan_dat csv header stayed void")?;
    let i_ra = column_index(header, "ra").ok_or("the enumeration carries no ra column")?;
    let i_dec = column_index(header, "dec").ok_or("the enumeration carries no dec column")?;
    let i_ra1 = column_index(header, "ra_1").ok_or("the enumeration carries no ra_1 column")?;
    let i_dec1 = column_index(header, "dec_1").ok_or("the enumeration carries no dec_1 column")?;
    let i_ra2 = column_index(header, "ra_2").ok_or("the enumeration carries no ra_2 column")?;
    let i_dec2 = column_index(header, "dec_2").ok_or("the enumeration carries no dec_2 column")?;
    let i_ra3 = column_index(header, "ra_3").ok_or("the enumeration carries no ra_3 column")?;
    let i_dec3 = column_index(header, "dec_3").ok_or("the enumeration carries no dec_3 column")?;
    let i_ra4 = column_index(header, "ra_4").ok_or("the enumeration carries no ra_4 column")?;
    let i_dec4 = column_index(header, "dec_4").ok_or("the enumeration carries no dec_4 column")?;

    let mut tiles = Vec::new();
    for line in lines {
        let cells: Vec<&str> = line.split(',').map(|c| c.trim()).collect();
        let f = |i: usize| cells.get(i).and_then(|c| c.parse::<f64>().ok());
        let (Some(ra), Some(dec)) = (f(i_ra), f(i_dec)) else {
            continue;
        };
        let corners = [
            match (f(i_ra1), f(i_dec1)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            },
            match (f(i_ra2), f(i_dec2)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            },
            match (f(i_ra3), f(i_dec3)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            },
            match (f(i_ra4), f(i_dec4)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            },
        ];
        if let Some(tile) = tile_from(ra, dec, corners) {
            tiles.push(tile);
        }
    }
    if tiles.len() != EXPECTED_TILES {
        return Err(format!(
            "the fp_scan_dat enumeration carried {} tiles, the measured survey holds {EXPECTED_TILES} — the enumeration stays incomplete, the asset stays unwritten",
            tiles.len()
        ));
    }
    Ok(tiles)
}

struct ZoneIndex {
    bins: Vec<Vec<u32>>,
}

impl ZoneIndex {
    fn new() -> Self {
        ZoneIndex {
            bins: vec![Vec::new(); ZONE_BINS],
        }
    }

    fn add(&mut self, tile: &Tile, idx: u32) {
        let lo_f = ((tile.dec - tile.hy_sky - ZONE_LO) / ZONE_BW).floor();
        let hi_f = ((tile.dec + tile.hy_sky - ZONE_LO) / ZONE_BW).floor();
        let lo_i = lo_f as i64;
        let hi_i = hi_f as i64;
        if hi_i < 0 || lo_i >= self.bins.len() as i64 {
            return;
        }
        let lo = if lo_i < 0 { 0 } else { lo_i as usize };
        let hi = if hi_i >= self.bins.len() as i64 {
            self.bins.len() - 1
        } else {
            hi_i as usize
        };
        for b in lo..=hi {
            self.bins[b].push(idx);
        }
    }

    fn covered_at(&self, ra: f64, dec: f64, tiles: &[Tile]) -> bool {
        let k = ((dec - ZONE_LO) / ZONE_BW) as i64;
        if k < 0 || k >= self.bins.len() as i64 {
            return false;
        }
        for &ti in &self.bins[k as usize] {
            if tiles[ti as usize].contains(ra, dec) {
                return true;
            }
        }
        false
    }
}

fn build_index(tiles: &[Tile]) -> ZoneIndex {
    let mut idx = ZoneIndex::new();
    for (i, tile) in tiles.iter().enumerate() {
        idx.add(tile, i as u32);
    }
    idx
}

fn emit_pixel(ipix: u32, out: &mut [u8; 36], rec: &mut [u8; REC_BYTES]) -> usize {
    let mut len = 0;
    for band in BANDS {
        encode_rec(
            rec,
            &FootprintRecord {
                order: ORDER,
                band,
                ipix,
                frac: 1.0,
            },
        );
        out[len..len + REC_BYTES].copy_from_slice(rec);
        len += REC_BYTES;
    }
    len
}

fn raster_region(
    start: i64,
    end: i64,
    idx: &ZoneIndex,
    tiles: &[Tile],
    out: &mut Vec<u8>,
) -> Result<u64, String> {
    let mut total: u64 = 0;
    let mut ob = [0u8; 36];
    let mut rec = [0u8; REC_BYTES];
    for ipix in start..end {
        let (theta, phi) =
            pix2ang_nest(NSIDE, ipix).ok_or("a nested pixel index stayed unmapped")?;
        let dec = 90.0 - theta.to_degrees();
        let ra = phi.to_degrees();
        if !idx.covered_at(ra, dec, tiles) {
            continue;
        }
        let len = emit_pixel(ipix as u32, &mut ob, &mut rec);
        out.extend_from_slice(&ob[..len]);
        total += (len / REC_BYTES) as u64;
    }
    Ok(total)
}

fn raster_full(writer: &mut impl Write, idx: &ZoneIndex, tiles: &[Tile]) -> Result<u64, String> {
    let block = 1 << 18;
    let mut total: u64 = 0;
    let mut start = 0i64;
    while start < NPIX {
        let end = (start + block).min(NPIX);
        let mut buf = Vec::with_capacity((end - start) as usize);
        total += raster_region(start, end, idx, tiles, &mut buf)?;
        writer
            .write_all(&buf)
            .map_err(|e| format!("write a footprint block returned void: {e}"))?;
        start = end;
    }
    Ok(total)
}

fn run(args: &[String]) -> Result<(), String> {
    footprint_identity(MAGIC)?;
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => "2mass_binary.fp01".to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let limit: Option<usize> = arg_value(args, "--limit").and_then(|s| s.parse().ok());

    let mut tiles = enumerate_tiles()?;
    eprintln!("2MASS scan tiles enumerated: {} tiles", tiles.len());
    if let Some(n) = limit {
        if n == 0 || n >= tiles.len() {
            return Err(format!(
                "--limit {n} lies outside the {}-tile enumeration",
                tiles.len()
            ));
        }
        tiles.truncate(n);
        eprintln!(
            "--limit {n}: rastering the first {n} tiles (a partial footprint, never --ci-mode)"
        );
    }

    let idx = build_index(&tiles);

    let mut file = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, 0);
    file.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} header returned void: {e}"))?;
    let mut w = BufWriter::with_capacity(1 << 20, file);
    let total = raster_full(&mut w, &idx, &tiles)
        .map_err(|e| format!("raster {out_path} returned void: {e}"))?;
    if total == 0 {
        return Err(
            "no observed tile carried a footprint record — the asset stays unwritten (0 honored)"
                .into(),
        );
    }
    w.flush()
        .map_err(|e| format!("flush {out_path} returned void: {e}"))?;
    let mut file = w
        .into_inner()
        .map_err(|e| format!("reopen {out_path} returned void: {e}"))?;

    file.seek(SeekFrom::Start(5))
        .map_err(|e| format!("seek {out_path} header returned void: {e}"))?;
    file.write_all(&total.to_le_bytes())
        .map_err(|e| format!("patch {out_path} header returned void: {e}"))?;
    file.flush()
        .map_err(|e| format!("flush {out_path} header returned void: {e}"))?;

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
        parse_header(&head).ok_or_else(|| format!("{out_path}: the header stayed unread"))?;
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
        decode_rec(&tail).ok_or_else(|| format!("{out_path}: the last record stayed unread"))?;

    eprintln!(
        "{out_path}: {} records, header {} — roundtrip verified",
        total, n_rows
    );
    eprintln!(
        "record shape: order {} band {:?} ipix {} frac {:.3}",
        last.order, last.band, last.ipix, last.frac
    );
    eprintln!("bands: j h ks share the one 2MASS scan footprint (simultaneous observation), frac 1.0 for observed");
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("2mass_binary_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::footprint::band_code;

    #[test]
    fn tile_contains_uses_the_center_rule_and_wraps_ra() {
        let t = Tile {
            ra: 0.0,
            dec: -30.0,
            cos_dec: (-30.0f64).to_radians().cos(),
            hx_sky: 0.2,
            hy_sky: 0.2,
        };
        assert!(t.contains(0.1, -30.1));
        assert!(t.contains(359.9, -30.1));
        assert!(!t.contains(0.6, -30.0));
        assert!(!t.contains(0.0, -30.4));
    }

    #[test]
    fn tile_from_spans_the_four_corners() {
        let tile = tile_from(
            91.8221,
            32.9312,
            [
                (91.9079, 36.0078),
                (91.7337, 36.0071),
                (91.9045, 29.8552),
                (91.7422, 29.8545),
            ],
        )
        .unwrap();
        assert!((tile.hy_sky - 3.0766).abs() < 0.001);
        assert!(tile.hx_sky > 0.05 && tile.hx_sky < 0.1);
        assert!(tile.contains(91.82, 33.0));
        assert!(!tile.contains(91.82, 25.0));
    }

    #[test]
    fn tile_from_refuses_a_flat_tile() {
        assert!(tile_from(0.0, 0.0, [(0.0, 0.0); 4]).is_none());
    }

    #[test]
    fn emit_pixel_writes_three_bands_in_code_order() {
        let mut out = [0u8; 36];
        let mut rec = [0u8; REC_BYTES];
        let len = emit_pixel(77, &mut out, &mut rec);
        assert_eq!(len, 3 * REC_BYTES);
        let j = decode_rec(&out[0..REC_BYTES]).unwrap();
        let h = decode_rec(&out[REC_BYTES..2 * REC_BYTES]).unwrap();
        let ks = decode_rec(&out[2 * REC_BYTES..3 * REC_BYTES]).unwrap();
        assert_eq!(j.band, FootprintBand::J);
        assert_eq!(h.band, FootprintBand::H);
        assert_eq!(ks.band, FootprintBand::Ks);
        assert!(band_code(j.band) < band_code(h.band));
        assert!(band_code(h.band) < band_code(ks.band));
        for r in [j, h, ks] {
            assert_eq!(r.order, ORDER);
            assert_eq!(r.ipix, 77);
            assert_eq!(r.frac, 1.0);
        }
    }

    #[test]
    fn zone_index_covers_a_tile_across_its_declination_span() {
        let t = Tile {
            ra: 10.0,
            dec: 0.0,
            cos_dec: 1.0,
            hx_sky: 1.0,
            hy_sky: 3.0,
        };
        let idx = build_index(&[t]);
        assert!(idx.covered_at(10.5, 1.0, &[t]));
        assert!(idx.covered_at(10.5, -2.0, &[t]));
        assert!(!idx.covered_at(12.0, 0.0, &[t]));
        assert!(!idx.covered_at(10.0, 5.0, &[t]));
    }

    #[test]
    fn binary_raster_is_nside_256_order_8() {
        assert_eq!(NSIDE, 256);
        assert_eq!(ORDER, 8);
        assert_eq!(NPIX, 12 * 256 * 256);
    }
}
