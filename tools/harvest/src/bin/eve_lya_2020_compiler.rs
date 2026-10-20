use omegaflow::archivar::embedded_lsk;
use omegaflow::fits::{FitsHeader, FitsTable};
use omegaflow::inflate::gunzip;
use omegaflow::lsk::days_from_civil;
use std::env;
use std::fs;
use std::io::Write;

const MAGIC: [u8; 4] = *b"EV20";
const MEGSP_INDEX: usize = 5;
const FLAG_MEGSP_MISSING: u8 = 8;
const FLAG_MEGSP_TOO_MANY: u8 = 128;
const TAI_1958_UNIX: f64 = 378_691_200.0;
const INSTRUMENT_EVE: u32 = 2;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn cell_array_f64(
    table: &FitsTable,
    buf: &[u8],
    row: usize,
    col: &omegaflow::fits::FitsColumn,
    k: usize,
) -> Option<f64> {
    if k >= col.repeat {
        return None;
    }
    let base = table.data_start + row * table.row_bytes + col.tbcol - 1;
    let elem = match col.code {
        'E' => 4,
        'D' => 8,
        _ => return None,
    };
    let off = base + k * elem;
    let raw = buf.get(off..off + elem)?;
    let v = match col.code {
        'E' => f32::from_be_bytes(raw[..4].try_into().ok()?) as f64,
        'D' => f64::from_be_bytes(raw.try_into().ok()?),
        _ => return None,
    };
    Some(v * col.tscal + col.tzero)
}

fn parse_file(
    path: &str,
    lsk: &omegaflow::archivar::LeapSeconds,
    records: &mut Vec<(f64, f64, u32, f32, f32)>,
) -> Option<(usize, usize, usize, f64)> {
    let gz = fs::read(path).ok()?;
    let bytes = gunzip(&gz)?;
    let (_, mut off) = FitsHeader::parse(&bytes, 0)?;
    while off + 80 <= bytes.len() {
        let Some((table, next)) = FitsTable::parse(&bytes, off) else {
            break;
        };
        let Some(diode) = table.column("DIODE_IRRADIANCE") else {
            off = next;
            continue;
        };
        if table.column("TAI").is_none() || table.column("SOD").is_none() {
            off = next;
            continue;
        }
        let tai = table.column("TAI").unwrap();
        let doy = table.column("YYYYDOY").unwrap();
        let sod = table.column("SOD").unwrap();
        let flags_col = table.column("FLAGS");
        let sc_col = table.column("SC_FLAGS");
        let precision = table.column("DIODE_PRECISION");
        let accuracy = table.column("DIODE_ACCURACY");
        let mut good = 0usize;
        let mut absent = 0usize;
        let mut pending = 0usize;
        let mut max_tai_delta = 0.0f64;
        for row in 0..table.n_rows {
            let tai_v = table.cell_f64(&bytes, row, tai);
            let unix = if let Some(t) = tai_v {
                let approx = t - TAI_1958_UNIX - 37.0;
                let delta = lsk.leap_at(approx).unwrap_or(37.0);
                t - TAI_1958_UNIX - delta
            } else {
                let Some(yyyydoy) = table.cell_i64(&bytes, row, doy) else {
                    absent += 1;
                    continue;
                };
                let Some(sod_v) = table.cell_f64(&bytes, row, sod) else {
                    absent += 1;
                    continue;
                };
                let year = yyyydoy / 1000;
                let doy_of_year = yyyydoy % 1000;
                let Some(days) = days_from_civil(year, 1, 1) else {
                    absent += 1;
                    continue;
                };
                (days + doy_of_year - 1) as f64 * 86_400.0 + sod_v
            };
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                absent += 1;
                continue;
            };
            if let (Some(t), Some(delta)) = (tai_v, lsk.leap_at(unix)) {
                let expected = unix + TAI_1958_UNIX + delta;
                let d = (t - expected).abs();
                if d > max_tai_delta {
                    max_tai_delta = d;
                }
            }
            let Some(flags) = flags_col.and_then(|c| table.cell_i64(&bytes, row, c)) else {
                absent += 1;
                continue;
            };
            let Some(sc) = sc_col.and_then(|c| table.cell_i64(&bytes, row, c)) else {
                absent += 1;
                continue;
            };
            let flags = flags as u8;
            let sc = sc as u8;
            let value = cell_array_f64(&table, &bytes, row, diode, MEGSP_INDEX);
            let missing = flags & (FLAG_MEGSP_MISSING | FLAG_MEGSP_TOO_MANY) != 0;
            let Some(v) = value else {
                absent += 1;
                continue;
            };
            if !v.is_finite() || v <= 0.0 || missing {
                absent += 1;
                continue;
            }
            if sc != 0 {
                pending += 1;
            }
            let Some(p) =
                precision.and_then(|c| cell_array_f64(&table, &bytes, row, c, MEGSP_INDEX))
            else {
                absent += 1;
                continue;
            };
            let Some(a) =
                accuracy.and_then(|c| cell_array_f64(&table, &bytes, row, c, MEGSP_INDEX))
            else {
                absent += 1;
                continue;
            };
            if !p.is_finite() || !a.is_finite() {
                absent += 1;
                continue;
            }
            let flag_word = (flags as u32) | ((sc as u32) << 8);
            records.push((tdb, v, flag_word, p as f32, a as f32));
            if sc == 0 {
                good += 1;
            }
        }
        return Some((good, absent, pending, max_tai_delta));
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let dir = match arg_value(&args, "--dir") {
        Some(v) => v,
        None => {
            eprintln!("--dir absent");
            std::process::exit(1);
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    let stats_path = arg_value(&args, "--stats");
    let lsk = match embedded_lsk() {
        Some(v) => v,
        None => {
            eprintln!("the time base is absent");
            std::process::exit(1);
        }
    };
    let mut files: Vec<String> = match fs::read_dir(&dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().into_owned())
            .filter(|p| p.ends_with(".fit.gz"))
            .collect(),
        Err(e) => {
            eprintln!("{}: read void ({})", dir, e);
            std::process::exit(1);
        }
    };
    files.sort();
    let mut records: Vec<(f64, f64, u32, f32, f32)> = Vec::new();
    let mut stats =
        String::from("file\ttotal\tgood\tabsent\tpending\tmax_tai_delta\ttdb_first\ttdb_last\n");
    let mut voids = 0usize;
    for path in &files {
        let before = records.len();
        match parse_file(path, &lsk, &mut records) {
            Some((good, absent, pending, max_delta)) => {
                let added = records.len() - before;
                let name = path.rsplit('/').next().unwrap_or(path);
                let first = match records.get(before) {
                    Some(r) => format!("{:.3}", r.0),
                    None => String::new(),
                };
                let last = match records.last() {
                    Some(r) => format!("{:.3}", r.0),
                    None => String::new(),
                };
                stats.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{:.6}\t{}\t{}\n",
                    name,
                    good + absent + pending,
                    good,
                    absent,
                    pending,
                    max_delta,
                    first,
                    last
                ));
                if added != good + pending {
                    eprintln!(
                        "{}: {} kept vs {} good + {} pending",
                        name, added, good, pending
                    );
                }
            }
            None => {
                voids += 1;
                eprintln!("{}: parses void", path);
            }
        }
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0);
    if records.is_empty() {
        eprintln!("{}: no records — the bin stays unwritten", out);
        std::process::exit(1);
    }
    let mut buf = Vec::with_capacity(8 + records.len() * 32);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, flags, p, a) in &records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&flags.to_le_bytes());
        buf.extend_from_slice(&INSTRUMENT_EVE.to_le_bytes());
        buf.extend_from_slice(&p.to_le_bytes());
        buf.extend_from_slice(&a.to_le_bytes());
    }
    if fs::write(&out, &buf).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    if let Some(p) = stats_path {
        if let Ok(mut f) = fs::File::create(&p) {
            let _ = f.write_all(stats.as_bytes());
        }
    }
    eprintln!(
        "{}: {} records ({} B), {} files, {} voids",
        out,
        records.len(),
        buf.len(),
        files.len(),
        voids
    );
}
