use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::LeapSeconds;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsTable};
use omegaflow::inflate::gunzip;
use omegaflow::lsk::days_from_civil;

const CDN_TAG: &str = "lasp.colorado.edu";

const BASE: &str = "https://lasp.colorado.edu/eve/data_access/evewebdata/products/level2";
const MAGIC: [u8; 4] = *b"EVL1";
const LINES: [(u32, &str); 11] = [
    (0, "Fe XVIII 94A"),
    (1, "Fe VIII 131A"),
    (3, "Fe IX 171A"),
    (6, "Fe XII 195A"),
    (8, "Fe XIV 211A"),
    (10, "Fe XV 284A"),
    (11, "He II 304A"),
    (12, "Fe XVI 335A"),
    (23, "He I 584A"),
    (36, "C III 977A"),
    (38, "O VI 1032A"),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn cell_array_f32(
    table: &FitsTable,
    buf: &[u8],
    row: usize,
    col: &omegaflow::fits::FitsColumn,
    k: usize,
) -> Option<f64> {
    if col.code != 'E' || k >= col.repeat {
        return None;
    }
    let base = table.data_start + row * table.row_bytes + col.tbcol - 1;
    let off = base + k * 4;
    let raw = buf.get(off..off + 4)?;
    let v = f32::from_be_bytes(raw.try_into().ok()?);
    if !v.is_finite() || v <= 0.0 {
        return None;
    }
    Some(v as f64)
}

fn extract_hour(
    path: &str,
    lsk: &LeapSeconds,
    records: &mut Vec<(f64, f64, u32)>,
) -> Option<usize> {
    let gz = std::fs::read(path).ok()?;
    let bytes = match gunzip(&gz) {
        Some(b) => b,
        None => {
            eprintln!("{}: gunzip void", path);
            return None;
        }
    };
    let (_, mut off) = match FitsHeader::parse(&bytes, 0) {
        Some(v) => v,
        None => {
            eprintln!("{}: primary header void", path);
            return None;
        }
    };
    let mut ext_count = 0usize;
    while off + 80 <= bytes.len() {
        let Some((table, next)) = FitsTable::parse(&bytes, off) else {
            eprintln!("{}: table parse void at ext {}", path, ext_count);
            break;
        };
        ext_count += 1;
        let Some(irr) = table.column("LINE_IRRADIANCE") else {
            off = next;
            continue;
        };
        if irr.repeat < 71 {
            off = next;
            continue;
        }
        let tai = table.column("TAI");
        let doy = table.column("YYYYDOY");
        let sod = table.column("SOD");
        let flags = table.column("FLAGS");
        let mut kept = 0usize;
        for row in 0..table.n_rows {
            if let Some(f) = flags {
                if table
                    .cell_f64(&bytes, row, f)
                    .map(|v| v != 0.0)
                    .unwrap_or(true)
                {
                    continue;
                }
            }
            let Some(yy_doy) = doy.and_then(|c| table.cell_f64(&bytes, row, c)) else {
                continue;
            };
            let Some(sod_v) = sod.and_then(|c| table.cell_f64(&bytes, row, c)) else {
                continue;
            };
            let _tai_v = tai.and_then(|c| table.cell_f64(&bytes, row, c));
            let yyyydoy = yy_doy as i64;
            let year = yyyydoy / 1000;
            let doy_of_year = yyyydoy % 1000;
            let days = days_from_civil(year, 1, 1)?;
            let unix = (days + doy_of_year - 1) as f64 * 86400.0 + sod_v;
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            for (idx, _name) in LINES {
                if let Some(v) = cell_array_f32(&table, &bytes, row, irr, idx as usize) {
                    records.push((tdb, v, idx));
                    kept += 1;
                }
            }
            if let Some(dio) = table.column("DIODE_IRRADIANCE") {
                if let Some(v) = cell_array_f32(&table, &bytes, row, dio, 0) {
                    records.push((tdb, v, 100));
                    kept += 1;
                }
            }
        }
        return Some(kept);
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "eve_lines.bin".to_string());
    let cache_dir = arg_value(&args, "--cache-dir").unwrap_or_else(|| {
        omegaflow::archivar::cache_root()
            .join("omegaflow_eve_cache")
            .to_string_lossy()
            .into_owned()
    });
    let start_doy: i64 = arg_value(&args, "--start-doy")
        .and_then(|v| v.parse().ok())
        .unwrap_or(60);
    let end_doy: i64 = arg_value(&args, "--end-doy")
        .and_then(|v| v.parse().ok())
        .unwrap_or(150);
    let year: i64 = arg_value(&args, "--year")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2014);
    let lsk = match omegaflow::archivar::embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("the time base is absent");
            std::process::exit(1);
        }
    };
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{}: cache dir stays uncreatable", cache_dir);
        std::process::exit(1);
    }
    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    if let Some(path) = arg_value(&args, "--file") {
        match extract_hour(&path, &lsk, &mut records) {
            Some(k) => eprintln!("{}: {} line records kept", path, k),
            None => eprintln!("{}: parses void", path),
        }
    } else {
        let mut voids = 0usize;
        for doy in start_doy..=end_doy {
            for hour in 0..24 {
                let name = format!("EVL_L2_{}{:03}_{:02}_008_01.fit.gz", year, doy, hour);
                let url = format!("{}/{}/{}/{}", BASE, year, format!("{:03}", doy), name);
                let cache_path = format!("{}/{}", cache_dir, name);
                if std::fs::metadata(&cache_path).is_err() {
                    match fetch_raw_bytes(&url, 86400) {
                        Some(bytes) => {
                            if std::fs::write(&cache_path, &bytes).is_err() {
                                voids += 1;
                                continue;
                            }
                        }
                        None => {
                            voids += 1;
                            continue;
                        }
                    }
                }
                match extract_hour(&cache_path, &lsk, &mut records) {
                    Some(k) => {
                        if k == 0 {
                            eprintln!("{}: 0 records", name);
                        }
                    }
                    None => {
                        eprintln!("{}: parses void", name);
                        voids += 1;
                    }
                }
            }
        }
        eprintln!("harvest: {} void files, {} records", voids, records.len());
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    records.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
    if records.is_empty() {
        eprintln!("{}: no records — the bin stays unwritten (0 honored)", out);
        std::process::exit(1);
    }
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, idx) in &records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&idx.to_le_bytes());
    }
    if std::fs::write(&out, &buf).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    let (t0, _, _) = records[0];
    let (t1, _, _) = records[records.len() - 1];
    eprintln!(
        "{}: {} records ({} B), epoch_tdb {:.0}..{:.0}",
        out,
        records.len(),
        buf.len(),
        t0,
        t1
    );
    if ci_mode && !upload_release(CDN_TAG, &out) {
        std::process::exit(1);
    }
}
