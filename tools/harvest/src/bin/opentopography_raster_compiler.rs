use omegaflow::archivar::geo::GbcoRec;
use omegaflow::archivar::range::fetch_range;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Hdf5Layout, LazyHdf5};
use omegaflow::witness::{FieldIdentity, WitnessKind, magic_identity};
use std::env;
use std::fs;
use std::path::Path;

const NETLOC: &str = "topex.ucsd.edu";
const URL: &str = "https://topex.ucsd.edu/pub/srtm15_plus/SRTM15_V2.5.5.nc";
const COMPILER: &str = "tools/harvest/src/bin/opentopography_raster_compiler.rs";
const FORMAT: &str = "srtm";
const MAGIC: [u8; 4] = *b"SRTM";
const VAR: &str = "z";
const REC_BYTES: usize = 24;
const PREFIX_WINDOW: u64 = 1 << 16;
const LAT_MIN: f64 = -90.0;
const LON_MIN: f64 = -180.0;
const ELEV_MIN_M: f64 = -12000.0;
const ELEV_MAX_M: f64 = 9000.0;

fn usage() -> &'static str {
    "usage: opentopography_raster_compiler --bbox <south,north,west,east> --out <path> \
     [--url <srtm15.nc>] [--stride <n>] [--limit <n>] [--ci-mode]  — the OpenTopography \
     SRTM15+ V2.5.5 netCDF-4 grid, read over HTTP range; the elevation grid within the \
     bbox is written as a gestalt witness (magic SRTM: lat, lon, elev)"
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_usize(args: &[String], name: &str, default: usize) -> Result<usize, String> {
    match arg_value(args, name) {
        Some(v) => match v.parse::<usize>() {
            Ok(n) if n > 0 => Ok(n),
            _ => Err(format!("{name} {v} carries no positive step")),
        },
        None => Ok(default),
    }
}

fn parse_bbox(spec: &str) -> Result<(f64, f64, f64, f64), String> {
    let mut it = spec.split(',');
    let south = it.next().and_then(|t| t.trim().parse::<f64>().ok());
    let north = it.next().and_then(|t| t.trim().parse::<f64>().ok());
    let west = it.next().and_then(|t| t.trim().parse::<f64>().ok());
    let east = it.next().and_then(|t| t.trim().parse::<f64>().ok());
    let (south, north, west, east) = match (south, north, west, east) {
        (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
        _ => {
            return Err(format!(
                "--bbox {spec}: four numbers south,north,west,east are expected"
            ));
        }
    };
    if !(south.is_finite() && north.is_finite() && west.is_finite() && east.is_finite()) {
        return Err(format!("--bbox {spec}: a corner carries no finite number"));
    }
    if !(north > south) {
        return Err(format!(
            "--bbox {spec}: north {north} does not stand over south {south}"
        ));
    }
    if !(east > west) {
        return Err(format!(
            "--bbox {spec}: east {east} does not stand over west {west}"
        ));
    }
    if south < LAT_MIN || north > LAT_MIN + 180.0 || west < LON_MIN || east > LON_MIN + 360.0 {
        return Err(format!(
            "--bbox {spec}: a corner lies outside the geographic grid"
        ));
    }
    Ok((south, north, west, east))
}

fn witness_gestalt_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FieldIdentity::Witness(WitnessKind::Gestalt)) => {
            eprintln!(
                "{} reads as a gestalt witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not gestalt — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn lat_of(row: u64, rows: u64) -> f64 {
    LAT_MIN + (row as f64 + 0.5) * (180.0 / rows as f64)
}

fn lon_of(col: u64, cols: u64) -> f64 {
    LON_MIN + (col as f64 + 0.5) * (360.0 / cols as f64)
}

fn axis_lo(lo: f64, min: f64, span: f64, count: u64) -> u64 {
    let v = (lo - min) / span - 0.5;
    let hi = (count - 1) as f64;
    v.floor().clamp(0.0, hi) as u64
}

fn axis_hi(hi_value: f64, min: f64, span: f64, count: u64) -> u64 {
    let v = (hi_value - min) / span - 0.5;
    let hi = (count - 1) as f64;
    v.ceil().clamp(0.0, hi) as u64
}

fn write_bin(records: &[GbcoRec]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * REC_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.lat.to_le_bytes());
        buf.extend_from_slice(&r.lon.to_le_bytes());
        buf.extend_from_slice(&r.elev.to_le_bytes());
    }
    buf
}

fn read_bin_count(data: &[u8]) -> Option<usize> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + n * REC_BYTES {
        return None;
    }
    Some(n)
}

fn grid_shape(prefix: &[u8], url: &str) -> Result<(u64, u64, u64, u64), String> {
    let mut file = LazyHdf5::open(prefix, |off, len| fetch_range(url, off, len, &[]))
        .map_err(|note| format!("{url}: the HDF5 superblock stays unread ({note:?})"))?;
    let (obj, ds, dt) = file
        .dataset(VAR)
        .map_err(|note| format!("{url}: dataset {VAR} stays absent ({note:?})"))?;
    if dt.class != 1 {
        return Err(format!(
            "{url}: {VAR} carries datatype class {} — a floating grid is expected",
            dt.class
        ));
    }
    if dt.size != 4 && dt.size != 8 {
        return Err(format!(
            "{url}: {VAR} carries {}-byte elements — a 4- or 8-byte grid is expected",
            dt.size
        ));
    }
    let dims = ds.dims.clone();
    if dims.len() != 2 {
        return Err(format!(
            "{url}: {VAR} carries rank {} — a 2-D lat/lon grid is expected",
            dims.len()
        ));
    }
    let (rows, cols) = (dims[0], dims[1]);
    if rows == 0 || cols == 0 {
        return Err(format!("{url}: {VAR} carries an empty {rows}x{cols} grid"));
    }
    let (cr, cc) = match obj.layout.as_ref() {
        Some(Hdf5Layout::Chunked { chunk_dims, .. }) if chunk_dims.len() == 2 => {
            (u64::from(chunk_dims[0]), u64::from(chunk_dims[1]))
        }
        _ => {
            return Err(format!(
                "{url}: {VAR} is not a 2-D chunked dataset — the bbox read stays unwritten"
            ));
        }
    };
    if cr == 0 || cc == 0 {
        return Err(format!("{url}: {VAR} carries an empty chunk {cr}x{cc}"));
    }
    Ok((rows, cols, cr, cc))
}

fn run(args: &[String]) -> Result<(), String> {
    witness_gestalt_identity(MAGIC)?;
    let url = match arg_value(args, "--url").filter(|u| !u.is_empty()) {
        Some(u) => u,
        None => URL.to_string(),
    };
    let Some(out_path) = arg_value(args, "--out").filter(|o| !o.is_empty()) else {
        return Err(usage().to_string());
    };
    let Some(bbox_spec) = arg_value(args, "--bbox") else {
        return Err(format!(
            "{} — the bbox is mandatory (a full grid is never written)",
            usage()
        ));
    };
    let (south, north, west, east) = parse_bbox(&bbox_spec)?;
    let stride = parse_usize(args, "--stride", 1)? as u64;
    let limit = parse_usize(args, "--limit", usize::MAX)?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let prefix = fetch_range(&url, 0, PREFIX_WINDOW, &[])
        .ok_or_else(|| format!("{url}: the prefix range read returned void"))?;
    let (rows, cols, cr, cc) = grid_shape(&prefix, &url)?;
    eprintln!("srtm: {rows}x{cols} grid, chunk {cr}x{cc}");

    let lat_span = 180.0 / rows as f64;
    let lon_span = 360.0 / cols as f64;
    let row_lo = axis_lo(south, LAT_MIN, lat_span, rows);
    let row_hi = axis_hi(north, LAT_MIN, lat_span, rows);
    let col_lo = axis_lo(west, LON_MIN, lon_span, cols);
    let col_hi = axis_hi(east, LON_MIN, lon_span, cols);

    let mut file = LazyHdf5::open(&prefix, |off, len| fetch_range(&url, off, len, &[]))
        .map_err(|note| format!("{url}: the HDF5 superblock stays unread ({note:?})"))?;

    let mut records: Vec<GbcoRec> = Vec::new();
    let mut chunks = 0u64;
    let chi0 = row_lo / cr;
    let chi1 = row_hi / cr;
    let cj0 = col_lo / cc;
    let cj1 = col_hi / cc;
    'outer: for chi in chi0..=chi1 {
        for chj in cj0..=cj1 {
            let coords = [chi, chj];
            let vals = match file
                .read_chunk_diag(VAR, &coords, |off, len| fetch_range(&url, off, len, &[]))
            {
                Ok(v) => v,
                Err(diag) => return Err(format!("chunk {chi},{chj}: {diag:?}")),
            };
            chunks += 1;
            let mut i = 0u64;
            while i < cr {
                let row = chi * cr + i;
                if row >= rows {
                    break;
                }
                if row >= row_lo && row <= row_hi && row % stride == 0 {
                    let lat = lat_of(row, rows);
                    if lat.is_finite() && lat >= south && lat <= north {
                        let mut j = 0u64;
                        while j < cc {
                            let col = chj * cc + j;
                            if col >= cols {
                                break;
                            }
                            if col >= col_lo && col <= col_hi && col % stride == 0 {
                                let lon = lon_of(col, cols);
                                if lon.is_finite() && lon >= west && lon <= east {
                                    let idx = (i * cc + j) as usize;
                                    if let Some(elev) = vals.get(idx).copied() {
                                        if elev.is_finite()
                                            && (ELEV_MIN_M..=ELEV_MAX_M).contains(&elev)
                                        {
                                            records.push(GbcoRec { lat, lon, elev });
                                            if records.len() >= limit {
                                                break 'outer;
                                            }
                                        }
                                    }
                                }
                            }
                            j += 1;
                        }
                    }
                }
                i += 1;
            }
        }
    }
    if records.is_empty() {
        return Err(format!(
            "the bbox {bbox_spec} leaves no measured cell over {chunks} chunks — the asset stays unwritten (0 honored)"
        ));
    }

    let bin = write_bin(&records);
    fs::write(&out_path, &bin).map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let back = fs::read(&out_path).map_err(|e| format!("read {out_path} returned void: {e}"))?;
    let count = read_bin_count(&back)
        .ok_or_else(|| format!("{out_path}: roundtrip parse void — the asset stays unverified"))?;
    if count != records.len() {
        return Err(format!(
            "{out_path}: {count} records read, {} written — the asset stays unwritten",
            records.len()
        ));
    }
    let name = match Path::new(&out_path).file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => return Err(format!("{out_path}: carries no file name")),
    };
    eprintln!(
        "srtm: {} records over {chunks} chunks, {} B -> {out_path} (roundtrip {count})",
        records.len(),
        bin.len()
    );

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
    println!("format {FORMAT}");
    println!("origin {url}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bin));

    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("{}", usage());
        eprintln!("source {URL}");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("opentopography_raster_compiler: {msg}");
        std::process::exit(2);
    }
}
