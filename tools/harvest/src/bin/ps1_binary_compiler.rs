use omegaflow::archivar::footprint::{
    decode_rec, encode_rec, parse_header, write_header, FootprintBand, FootprintRecord, HEADER_LEN,
    MAGIC, NSIDE, REC_BYTES,
};
use omegaflow::cdn::upload_asset;
use omegaflow::fits::{FitsHeader, FitsTable};
use omegaflow::healpix::pix2ang_nest;
use omegaflow::zeuge::{magic_identity, FeldIdentitaet};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;

const GRID_URL: &str = "https://outerspace.stsci.edu/download/attachments/298812317/ps1grid.fits?version=1&modificationDate=1532367528459&api=v2";
const FILENAMES_URL: &str = "https://ps1images.stsci.edu/cgi-bin/ps1filenames.py";
const PIXEL_DEG: f64 = 0.25 / 3600.0;
const SUBCELLS: usize = 100;
const PROBE_WORKERS: u32 = 8;
const ZONE_LO: f64 = -33.5;
const ZONE_BINS: usize = 315;
const ZONE_BW: f64 = 0.02;
const ORDER: u8 = NSIDE.trailing_zeros() as u8;
const NPIX: i64 = 12 * NSIDE * NSIDE;
const ALL5: u8 = 0b11111;

const FILTER_BITS: [(char, usize); 5] = [('g', 0), ('r', 1), ('i', 2), ('z', 3), ('y', 4)];
const BIT_BANDS: [FootprintBand; 5] = [
    FootprintBand::G,
    FootprintBand::R,
    FootprintBand::I,
    FootprintBand::Z,
    FootprintBand::Y,
];

const COMPLETE_SAMPLES: [(u32, u32); 11] = [
    (714, 0),
    (714, 9),
    (714, 90),
    (714, 99),
    (1322, 50),
    (1590, 99),
    (2289, 0),
    (2634, 0),
    (2634, 99),
    (2643, 0),
    (2643, 99),
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

#[derive(Debug, Clone, Copy)]
struct ZoneRow {
    zone: i64,
    first_proj: i64,
    nband: i64,
    dec_min: f64,
    dec_max: f64,
    xcell_px: i64,
    ycell_px: i64,
}

struct Grid {
    rows: Vec<ZoneRow>,
}

impl Grid {
    fn row(&self, zone: i64) -> Option<&ZoneRow> {
        self.rows.iter().find(|r| r.zone == zone)
    }
}

fn parse_grid(buf: &[u8]) -> Result<Grid, String> {
    let (_, ext_off) =
        FitsHeader::parse(buf, 0).ok_or("the ps1grid primary header stayed unread")?;
    let (table, _) = FitsTable::parse(buf, ext_off).ok_or("the ps1grid BINTABLE stayed unread")?;
    let zone_c = table
        .column("ZONE")
        .ok_or("the ps1grid table carries no ZONE column")?;
    let proj_c = table
        .column("PROJCELL")
        .ok_or("the ps1grid table carries no PROJCELL column")?;
    let nband_c = table
        .column("NBAND")
        .ok_or("the ps1grid table carries no NBAND column")?;
    let dec_min_c = table
        .column("DEC_MIN")
        .ok_or("the ps1grid table carries no DEC_MIN column")?;
    let dec_max_c = table
        .column("DEC_MAX")
        .ok_or("the ps1grid table carries no DEC_MAX column")?;
    let xcell_c = table
        .column("XCELL")
        .ok_or("the ps1grid table carries no XCELL column")?;
    let ycell_c = table
        .column("YCELL")
        .ok_or("the ps1grid table carries no YCELL column")?;
    let mut rows = Vec::with_capacity(table.n_rows);
    for r in 0..table.n_rows {
        let zone = table
            .cell_i64(buf, r, zone_c)
            .ok_or("a ps1grid ZONE cell stayed unread")?;
        let first_proj = table
            .cell_i64(buf, r, proj_c)
            .ok_or("a ps1grid PROJCELL cell stayed unread")?;
        let nband = table
            .cell_i64(buf, r, nband_c)
            .ok_or("a ps1grid NBAND cell stayed unread")?;
        let dec_min = table
            .cell_f64(buf, r, dec_min_c)
            .ok_or("a ps1grid DEC_MIN cell stayed unread")?;
        let dec_max = table
            .cell_f64(buf, r, dec_max_c)
            .ok_or("a ps1grid DEC_MAX cell stayed unread")?;
        let xcell_px = table
            .cell_i64(buf, r, xcell_c)
            .ok_or("a ps1grid XCELL cell stayed unread")?;
        let ycell_px = table
            .cell_i64(buf, r, ycell_c)
            .ok_or("a ps1grid YCELL cell stayed unread")?;
        if nband <= 0 || xcell_px <= 0 || ycell_px <= 0 {
            return Err(format!(
                "zone {zone}: nband {nband} xcell {xcell_px} ycell {ycell_px} — a zone geometry that carries no area stayed unread"
            ));
        }
        if !(dec_min.is_finite() && dec_max.is_finite() && dec_min < dec_max) {
            return Err(format!(
                "zone {zone}: dec_min {dec_min} dec_max {dec_max} — the declination band stayed unread"
            ));
        }
        rows.push(ZoneRow {
            zone,
            first_proj,
            nband,
            dec_min,
            dec_max,
            xcell_px,
            ycell_px,
        });
    }
    if rows.len() < 30 {
        return Err(format!(
            "the ps1grid table carries {} zone rows, the measured RINGS.V3 grid carries 33 — the geometry stayed unread",
            rows.len()
        ));
    }
    Ok(Grid { rows })
}

fn skycell_url(proj: u32, sub: u32) -> String {
    format!("{FILENAMES_URL}?SKYCELL={proj:04}.{sub:03}&filters=g,r,i,z,y")
}

#[derive(Debug, Clone, Copy)]
struct Skycell {
    ra: f64,
    dec: f64,
    mask: u8,
}

fn parse_probe_body(text: &str) -> Option<Skycell> {
    let mut ra: Option<f64> = None;
    let mut dec: Option<f64> = None;
    let mut mask: u8 = 0;
    for line in text.lines() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 10 || toks[0] == "projcell" || toks[6] != "stack" {
            continue;
        }
        let f = toks[4].chars().next()?;
        let bit = FILTER_BITS.iter().find(|(c, _)| *c == f)?;
        mask |= 1u8 << bit.1;
        if ra.is_none() {
            ra = toks[2].parse::<f64>().ok();
            dec = toks[3].parse::<f64>().ok();
        }
    }
    let ra = ra?;
    let dec = dec?;
    if mask == 0 || !(ra.is_finite() && dec.is_finite()) {
        return None;
    }
    Some(Skycell { ra, dec, mask })
}

fn curl_bytes(url: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .args(["-sSf", "-m", "90"])
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if out.status.success() {
        Ok(out.stdout)
    } else {
        Err(format!(
            "curl {url} returned http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ))
    }
}

fn probe_skycell(proj: u32, sub: u32) -> Result<Option<Skycell>, String> {
    let url = skycell_url(proj, sub);
    let mut last = String::new();
    for _ in 0..3 {
        match curl_bytes(&url) {
            Ok(body) => return Ok(parse_probe_body(&String::from_utf8_lossy(&body))),
            Err(msg) => last = msg,
        }
    }
    Err(format!(
        "skycell {proj}.{sub:03} probe stayed refused after three attempts: {last}"
    ))
}

struct Census {
    requests: u64,
    zone15_probes: u64,
    present: u64,
    absent: u64,
    samples: u64,
    records: u64,
}

fn load_grid(grid_arg: &Option<String>) -> Result<Vec<u8>, String> {
    match grid_arg {
        Some(path) => {
            std::fs::read(path).map_err(|e| format!("read local grid {path} returned void: {e}"))
        }
        None => curl_bytes(GRID_URL),
    }
}

fn probe_zone15(
    first_proj: u32,
    n_probe: usize,
    census: &mut Census,
) -> Result<Vec<Skycell>, String> {
    let next = AtomicU64::new(0);
    let refused = AtomicU64::new(0);
    let (tx, rx) = mpsc::channel::<(usize, Option<Skycell>)>();
    let mut slots: Vec<Option<Skycell>> = Vec::with_capacity(n_probe);
    slots.resize_with(n_probe, || None);
    let _ = std::thread::scope(|s| {
        for _ in 0..PROBE_WORKERS {
            let tx = tx.clone();
            let next = &next;
            let refused = &refused;
            s.spawn(move || loop {
                if refused.load(Ordering::Relaxed) > 0 {
                    break;
                }
                let idx = next.fetch_add(1, Ordering::Relaxed) as usize;
                if idx >= n_probe {
                    break;
                }
                let seq = idx as u32;
                let proj = first_proj + seq / SUBCELLS as u32;
                let sub = seq % SUBCELLS as u32;
                match probe_skycell(proj, sub) {
                    Ok(cell) => {
                        let _ = tx.send((idx, cell));
                    }
                    Err(_) => {
                        refused.fetch_add(1, Ordering::Relaxed);
                        let _ = tx.send((idx, None));
                    }
                }
            });
        }
        drop(tx);
        for (idx, cell) in rx {
            if idx < n_probe {
                slots[idx] = cell;
            }
        }
    });
    let refused_n = refused.load(Ordering::Relaxed);
    if refused_n > 0 {
        return Err(format!(
            "{refused_n} skycell probes stayed refused after three attempts each — the zone 15 grid stayed incompletely enumerated, the asset stays unwritten"
        ));
    }
    census.zone15_probes += n_probe as u64;
    census.requests += n_probe as u64;
    let mut present = Vec::new();
    for cell in slots.into_iter().flatten() {
        census.present += 1;
        present.push(cell);
    }
    census.absent = census.zone15_probes - census.present;
    Ok(present)
}

fn sample_complete_zones(census: &mut Census) -> Result<(), String> {
    for (proj, sub) in COMPLETE_SAMPLES {
        census.samples += 1;
        census.requests += 1;
        match probe_skycell(proj, sub) {
            Ok(Some(cell)) if cell.mask == ALL5 => {}
            Ok(Some(cell)) => {
                return Err(format!(
                    "complete-zone skycell {proj}.{sub:03} carries band mask {:05b}, the measured complete 10x10 grid carries g r i z y in every skycell — the cap geometry is refuted, the asset stays unwritten",
                    cell.mask
                ));
            }
            Ok(None) => {
                return Err(format!(
                    "complete-zone skycell {proj}.{sub:03} is absent, the measured complete 10x10 grid holds every subcell — the cap geometry is refuted, the asset stays unwritten"
                ));
            }
            Err(msg) => return Err(msg),
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    ra: f64,
    dec: f64,
    cos_dec: f64,
    hx_sky: f64,
    hy_sky: f64,
    mask: u8,
}

impl Cell {
    fn contains(&self, ra: f64, dec: f64) -> bool {
        if (dec - self.dec).abs() > self.hy_sky {
            return false;
        }
        let mut dra = ra - self.ra;
        dra = (dra + 540.0).rem_euclid(360.0) - 180.0;
        dra.abs() * self.cos_dec <= self.hx_sky
    }
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

    fn add(&mut self, cell: &Cell, idx: u32) {
        let lo_f = ((cell.dec - cell.hy_sky - ZONE_LO) / ZONE_BW).floor();
        let hi_f = ((cell.dec + cell.hy_sky - ZONE_LO) / ZONE_BW).floor();
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

    fn mask_at(&self, ra: f64, dec: f64, cells: &[Cell]) -> u8 {
        if dec < ZONE_LO {
            return 0;
        }
        let k = ((dec - ZONE_LO) / ZONE_BW) as usize;
        if k >= self.bins.len() {
            return 0;
        }
        let mut mask: u8 = 0;
        for &ci in &self.bins[k] {
            if cells[ci as usize].contains(ra, dec) {
                mask |= cells[ci as usize].mask;
            }
        }
        mask
    }
}

fn build_index(cells: &[Cell]) -> ZoneIndex {
    let mut idx = ZoneIndex::new();
    for (i, cell) in cells.iter().enumerate() {
        idx.add(cell, i as u32);
    }
    idx
}

fn cell_top(cells: &[Cell]) -> f64 {
    cells
        .iter()
        .map(|c| c.dec + c.hy_sky)
        .fold(f64::NEG_INFINITY, f64::max)
}

fn emit_mask(mask: u8, ipix: u32, out: &mut [u8; 60], rec: &mut [u8; REC_BYTES]) -> usize {
    let mut len = 0;
    for bit in 0..5 {
        if mask & (1u8 << bit) != 0 {
            encode_rec(
                rec,
                &FootprintRecord {
                    order: ORDER,
                    band: BIT_BANDS[bit],
                    ipix,
                    frac: 1.0,
                },
            );
            out[len..len + REC_BYTES].copy_from_slice(rec);
            len += REC_BYTES;
        }
    }
    len
}

fn raster_region(
    start: i64,
    end: i64,
    idx: &ZoneIndex,
    cells: &[Cell],
    cap_min: Option<f64>,
    south_top: f64,
    out: &mut Vec<u8>,
) -> Result<u64, String> {
    let mut total: u64 = 0;
    let mut ob = [0u8; 60];
    let mut rec = [0u8; REC_BYTES];
    for ipix in start..end {
        let (theta, phi) =
            pix2ang_nest(NSIDE, ipix).ok_or("a nested pixel index stayed unmapped")?;
        let dec = 90.0 - theta.to_degrees();
        let mut mask: u8 = 0;
        if let Some(cap) = cap_min {
            if dec >= cap {
                mask = ALL5;
            }
        }
        if mask == 0 && dec > ZONE_LO && dec < south_top {
            let ra = phi.to_degrees();
            mask = idx.mask_at(ra, dec, cells);
        }
        if mask == 0 {
            continue;
        }
        let len = emit_mask(mask, ipix as u32, &mut ob, &mut rec);
        out.extend_from_slice(&ob[..len]);
        total += (len / REC_BYTES) as u64;
    }
    Ok(total)
}

fn raster_partial(
    idx: &ZoneIndex,
    cells: &[Cell],
    south_top: f64,
) -> Result<(Vec<u8>, u64), String> {
    let threads = 8usize;
    let chunk = (NPIX + threads as i64 - 1) / threads as i64;
    let results = std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(threads);
        for t in 0..threads {
            let start = t as i64 * chunk;
            let end = (start + chunk).min(NPIX);
            if start >= end {
                continue;
            }
            handles.push(s.spawn(move || {
                let mut buf = Vec::new();
                let n = raster_region(start, end, idx, cells, None, south_top, &mut buf)?;
                Ok::<(Vec<u8>, u64), String>((buf, n))
            }));
        }
        let mut collected = Vec::with_capacity(handles.len());
        for h in handles {
            match h.join() {
                Ok(Ok(pair)) => collected.push(pair),
                Ok(Err(msg)) => return Err(msg),
                Err(_) => return Err("a raster thread stayed unjoined".to_string()),
            }
        }
        Ok(collected)
    });
    let buffers = results?;
    let mut out = Vec::new();
    let mut total: u64 = 0;
    for (buf, n) in buffers {
        total += n;
        out.extend_from_slice(&buf);
    }
    Ok((out, total))
}

fn raster_full(
    writer: &mut impl Write,
    idx: &ZoneIndex,
    cells: &[Cell],
    cap_min: f64,
    south_top: f64,
) -> Result<u64, String> {
    let block = 1 << 20;
    let mut total: u64 = 0;
    let mut start = 0i64;
    while start < NPIX {
        let end = (start + block).min(NPIX);
        let mut buf = Vec::with_capacity((end - start) as usize);
        total += raster_region(start, end, idx, cells, Some(cap_min), south_top, &mut buf)?;
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
        None => "ps1_dr2_binary.fp01".to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let limit: Option<u64> = match arg_value(args, "--limit") {
        None => None,
        Some(s) => Some(
            s.parse::<u64>()
                .map_err(|_| format!("--limit {s} reads no count"))?,
        ),
    };
    let grid_arg: Option<String> = match arg_value(args, "--grid") {
        None => None,
        Some(s) => Some(s),
    };

    let grid_bytes = load_grid(&grid_arg)?;
    eprintln!("ps1grid geometry: {} bytes read", grid_bytes.len());
    let grid = parse_grid(&grid_bytes)?;
    let z15 = grid
        .row(15)
        .ok_or("the ps1grid carries no zone 15 — the ragged survey band stayed unread")?;
    let z16 = grid
        .row(16)
        .ok_or("the ps1grid carries no zone 16 — the complete-grid boundary stayed unread")?;
    eprintln!(
        "zones in the grid: {}, band boundary zone 15..16 measured from the geometry rows",
        grid.rows.len()
    );
    eprintln!(
        "zone 15: projcell {}..{}, {} projection cells, declination {:.6}..{:.6}, skycell image {}x{} px at {:.3} arcsec/px",
        z15.first_proj,
        z15.first_proj + z15.nband - 1,
        z15.nband,
        z15.dec_min,
        z15.dec_max,
        z15.xcell_px,
        z15.ycell_px,
        0.25
    );

    let n_zone15 = z15.nband as usize * SUBCELLS;
    let probe_n = match limit {
        Some(l) => (l as usize).min(n_zone15),
        None => n_zone15,
    };
    let full = match limit {
        Some(l) => l as usize >= n_zone15,
        None => true,
    };
    let mut census = Census {
        requests: 0,
        zone15_probes: 0,
        present: 0,
        absent: 0,
        samples: 0,
        records: 0,
    };

    if full {
        sample_complete_zones(&mut census)?;
        eprintln!(
            "complete-zone sampling: {} skycells across zones 16..45 each carry g r i z y — the complete grid geometry holds",
            census.samples
        );
    }

    let present = probe_zone15(z15.first_proj as u32, probe_n, &mut census)?;
    let hy_sky = z15.ycell_px as f64 * PIXEL_DEG * 0.5;
    let hx_sky = z15.xcell_px as f64 * PIXEL_DEG * 0.5;
    let cells: Vec<Cell> = present
        .iter()
        .map(|s| Cell {
            ra: s.ra,
            dec: s.dec,
            cos_dec: s.dec.to_radians().cos(),
            hx_sky,
            hy_sky,
            mask: s.mask,
        })
        .collect();
    let idx = build_index(&cells);
    let south_top = cell_top(&cells);
    let cap_min = match full {
        true => Some(z16.dec_min - 0.02),
        false => None,
    };
    match cap_min {
        Some(c) => eprintln!(
            "complete region: declination >= {c:.6} across the full right-ascension circle in g r i z y (zones 16..45), zone 15 carried per skycell"
        ),
        None => eprintln!(
            "partial region: only the {} existence-probed zone 15 skycells enter the asset",
            census.zone15_probes
        ),
    }

    let mut file = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, 0);
    file.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} header returned void: {e}"))?;
    let mut w = BufWriter::with_capacity(1 << 20, file);
    let total = match cap_min {
        Some(c) => raster_full(&mut w, &idx, &cells, c, south_top)
            .map_err(|e| format!("raster {out_path} returned void: {e}"))?,
        None => {
            let (buf, n) = raster_partial(&idx, &cells, south_top)
                .map_err(|e| format!("raster {out_path} returned void: {e}"))?;
            w.write_all(&buf)
                .map_err(|e| format!("write {out_path} records returned void: {e}"))?;
            n
        }
    };
    if total == 0 {
        return Err("no observed skycell carried a footprint record — the asset stays unwritten (0 honored)".into());
    }
    census.records = total;
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
        "record shape: order {} band code {:?} ipix {} frac {:.3}",
        last.order, last.band, last.ipix, last.frac
    );
    eprintln!(
        "sorted: records streamed by ascending nested ipix with ascending band code within the pixel"
    );
    eprintln!(
        "census: {} existence probes ({} zone 15 skycells, {} complete-zone samples), {} present, {} absent",
        census.requests,
        census.zone15_probes,
        census.samples,
        census.present,
        census.absent
    );
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ps1_binary_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::footprint::band_code;

    #[test]
    fn skycell_url_embeds_the_measured_route() {
        assert_eq!(
            skycell_url(635, 9),
            "https://ps1images.stsci.edu/cgi-bin/ps1filenames.py?SKYCELL=0635.009&filters=g,r,i,z,y"
        );
        assert_eq!(
            skycell_url(2643, 99),
            "https://ps1images.stsci.edu/cgi-bin/ps1filenames.py?SKYCELL=2643.099&filters=g,r,i,z,y"
        );
    }

    #[test]
    fn parse_probe_body_reads_the_measured_header() {
        let body = "\
projcell subcell ra dec filter mjd type filename shortname badflag
635 0 2.128573126861541 -31.790888721856746 i 0.0 stack /rings.v3.skycell/0635/000/rings.v3.skycell.0635.000.stk.i.unconv.fits rings.v3.skycell.0635.000.stk.i.unconv.fits 0
";
        let cell = parse_probe_body(body).unwrap();
        assert_eq!(cell.mask, 1u8 << 2);
        assert!((cell.ra - 2.128573126861541).abs() < 1e-9);
        assert!((cell.dec + 31.790888721856746).abs() < 1e-9);
        assert!(parse_probe_body(
            "projcell subcell ra dec filter mjd type filename shortname badflag\n"
        )
        .is_none());
    }

    #[test]
    fn parse_probe_body_unions_every_band_row() {
        let body = "\
projcell subcell ra dec filter mjd type filename shortname badflag
2643 0 44.9994473839998 87.45606403015682 g 0.0 stack /rings.v3.skycell/2643/000/rings.v3.skycell.2643.000.stk.g.unconv.fits rings.v3.skycell.2643.000.stk.g.unconv.fits 0
2643 0 44.9994473839998 87.45606403015682 y 0.0 stack /rings.v3.skycell/2643/000/rings.v3.skycell.2643.000.stk.y.unconv.fits rings.v3.skycell.2643.000.stk.y.unconv.fits 0
";
        let cell = parse_probe_body(body).unwrap();
        assert_eq!(cell.mask, (1 << 0) | (1 << 4));
        assert_eq!(cell.mask & ALL5, (1 << 0) | (1 << 4));
    }

    #[test]
    fn cell_contains_uses_the_center_rule_and_wraps_ra() {
        let c = Cell {
            ra: 0.0,
            dec: -30.0,
            cos_dec: (-30.0f64).to_radians().cos(),
            hx_sky: 0.2,
            hy_sky: 0.2,
            mask: ALL5,
        };
        assert!(c.contains(0.1, -30.1));
        assert!(c.contains(359.9, -30.1));
        assert!(c.contains(0.0, -29.8));
        assert!(!c.contains(0.6, -30.0));
        assert!(!c.contains(0.0, -30.4));
    }

    #[test]
    fn zone_index_unions_overlapping_cells() {
        let a = Cell {
            ra: 0.0,
            dec: -30.0,
            cos_dec: 1.0,
            hx_sky: 0.5,
            hy_sky: 0.5,
            mask: 1 << 0,
        };
        let b = Cell {
            ra: 0.3,
            dec: -30.0,
            cos_dec: 1.0,
            hx_sky: 0.5,
            hy_sky: 0.5,
            mask: 1 << 2,
        };
        let cells = [a, b];
        let idx = build_index(&cells);
        let mask = idx.mask_at(0.15, -30.0, &cells);
        assert_eq!(mask, (1 << 0) | (1 << 2));
        assert_eq!(idx.mask_at(30.0, -30.0, &cells), 0);
        assert_eq!(idx.mask_at(0.0, -40.0, &cells), 0);
    }

    #[test]
    fn emit_mask_writes_ascending_band_codes() {
        let mut out = [0u8; 60];
        let mut rec = [0u8; REC_BYTES];
        let len = emit_mask(0b01101, 77, &mut out, &mut rec);
        assert_eq!(len, 3 * REC_BYTES);
        let g = decode_rec(&out[0..REC_BYTES]).unwrap();
        let i = decode_rec(&out[REC_BYTES..2 * REC_BYTES]).unwrap();
        let z = decode_rec(&out[2 * REC_BYTES..3 * REC_BYTES]).unwrap();
        assert_eq!(g.band, FootprintBand::G);
        assert_eq!(i.band, FootprintBand::I);
        assert_eq!(z.band, FootprintBand::Z);
        for rec_band in [g, i, z] {
            assert_eq!(rec_band.order, ORDER);
            assert_eq!(rec_band.ipix, 77);
            assert_eq!(rec_band.frac, 1.0);
        }
        assert!(band_code(g.band) < band_code(i.band));
        assert!(band_code(i.band) < band_code(z.band));
    }

    #[test]
    fn filter_bits_cover_the_five_ps1_bands_in_code_order() {
        for (bit, (_c, b)) in FILTER_BITS.iter().enumerate() {
            assert_eq!(*b, bit);
        }
        let codes: Vec<u8> = BIT_BANDS.iter().map(|b| band_code(*b)).collect();
        assert_eq!(codes, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn cap_raster_emits_five_bands_for_every_complete_pixel() {
        let idx = ZoneIndex::new();
        let cells: Vec<Cell> = Vec::new();
        let mut buf = Vec::new();
        let n = raster_region(0, 64, &idx, &cells, Some(-90.0), -28.0, &mut buf).unwrap();
        assert_eq!(n, 320);
        assert_eq!(buf.len(), 320 * REC_BYTES);
        let first = decode_rec(&buf[0..REC_BYTES]).unwrap();
        assert_eq!(first.band, FootprintBand::G);
        assert_eq!(first.ipix, 0);
        let fifth = decode_rec(&buf[4 * REC_BYTES..5 * REC_BYTES]).unwrap();
        assert_eq!(fifth.band, FootprintBand::Y);
        assert_eq!(fifth.ipix, 0);
    }

    #[test]
    fn raster_without_cap_and_without_cells_stays_empty() {
        let idx = ZoneIndex::new();
        let cells: Vec<Cell> = Vec::new();
        let mut buf = Vec::new();
        let n = raster_region(0, 512, &idx, &cells, None, -27.9, &mut buf).unwrap();
        assert_eq!(n, 0);
        assert!(buf.is_empty());
    }
}
