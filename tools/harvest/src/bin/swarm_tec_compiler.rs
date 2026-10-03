use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdf::{CdfFile, value_present};
use std::collections::HashMap;

const MAGIC: &[u8; 4] = b"SCTE";
const FIELDS: usize = 5;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn type_name(t: u32) -> &'static str {
    match t {
        1 => "CDF_INT1",
        2 => "CDF_INT2",
        4 => "CDF_INT4",
        8 => "CDF_INT8",
        11 => "CDF_UINT1",
        12 => "CDF_UINT2",
        14 => "CDF_UINT4",
        21 => "CDF_REAL4",
        22 => "CDF_REAL8",
        31 => "CDF_EPOCH",
        32 => "CDF_EPOCH16",
        33 => "CDF_TIME_TT2000",
        41 => "CDF_BYTE",
        44 => "CDF_FLOAT",
        45 => "CDF_DOUBLE",
        51 => "CDF_CHAR",
        52 => "CDF_UCHAR",
        _ => "unknown",
    }
}

fn probe(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: {:?}", note);
            std::process::exit(1);
        }
    };
    eprintln!(
        "{}: CDF {}.{}.{} encoding {} majority {} {} zVariables {} attributes eof {}",
        path,
        file.version.0,
        file.version.1,
        file.version.2,
        file.encoding,
        file.majority,
        file.vars.len(),
        file.num_att,
        file.eof,
    );
    for var in &file.vars {
        let n_rec = file.var_records(&bytes, var).map(|r| r.len()).ok();
        eprintln!(
            "  var {} {} {} x{} dims {:?} vary {:?} record_vary {} max_rec {} records {:?}",
            var.var_num,
            var.name,
            type_name(var.data_type),
            var.num_elements,
            var.dim_sizes,
            var.dim_vary,
            var.record_vary,
            var.max_rec,
            n_rec,
        );
        match file.var_records(&bytes, var) {
            Ok(records) => {
                if let Some((rec, vals)) = records.first() {
                    let real: Vec<f64> = vals
                        .iter()
                        .copied()
                        .filter(|v| value_present(*v))
                        .take(4)
                        .collect();
                    eprintln!(
                        "    rec {rec} n {} real-first {:?} fill {:?}",
                        vals.len(),
                        real,
                        vals.first().copied()
                    );
                }
            }
            Err(note) => eprintln!("    records: {:?}", note),
        }
    }
}

fn scalar_map(file: &CdfFile, bytes: &[u8], name: &str) -> Option<HashMap<u32, f64>> {
    let var = file.var(name)?;
    let records = file.var_records(bytes, var).ok()?;
    let mut map = HashMap::with_capacity(records.len());
    for (rec, vals) in records {
        if let Some(v) = vals.first().copied() {
            map.insert(rec, v);
        }
    }
    Some(map)
}

fn write_bin(records: &[[f64; FIELDS]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * FIELDS * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; FIELDS]>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
    if bytes.len() != 8 + n * FIELDS * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut r = [0.0f64; FIELDS];
        for (j, slot) in r.iter_mut().enumerate() {
            let off = 8 + i * FIELDS * 8 + j * 8;
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

fn compile(path: &str, out: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: {:?}", note);
            std::process::exit(1);
        }
    };
    let Some(epoch_var) = file.var("Timestamp") else {
        eprintln!("Timestamp absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let epoch = match file.epoch_map(&bytes, epoch_var) {
        Ok(m) => m,
        Err(note) => {
            eprintln!("Timestamp: {:?}", note);
            std::process::exit(1);
        }
    };
    let Some(lat) = scalar_map(&file, &bytes, "Latitude") else {
        eprintln!("Latitude absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(lon) = scalar_map(&file, &bytes, "Longitude") else {
        eprintln!("Longitude absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(radius) = scalar_map(&file, &bytes, "Radius") else {
        eprintln!("Radius absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(vtec) = scalar_map(&file, &bytes, "Absolute_VTEC") else {
        eprintln!("Absolute_VTEC absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };

    let mut records: Vec<[f64; FIELDS]> = Vec::with_capacity(epoch.len());
    let mut skipped = 0usize;
    for (rec, t) in &epoch {
        let (Some(&la), Some(&lo), Some(&ra), Some(&vt)) =
            (lat.get(rec), lon.get(rec), radius.get(rec), vtec.get(rec))
        else {
            skipped += 1;
            continue;
        };
        let radius_km = ra / 1000.0;
        let all_present = value_present(*t)
            && value_present(la)
            && value_present(lo)
            && value_present(radius_km)
            && value_present(vt);
        if !all_present || radius_km <= 0.0 {
            skipped += 1;
            continue;
        }
        records.push([la, lo, radius_km, *t, vt]);
    }
    records.sort_by(|a, b| {
        a[3].total_cmp(&b[3])
            .then(a[0].total_cmp(&b[0]))
            .then(a[1].total_cmp(&b[1]))
            .then(a[2].total_cmp(&b[2]))
            .then(a[4].total_cmp(&b[4]))
    });

    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes_out = write_bin(&records);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let vt_min = records.iter().map(|r| r[4]).fold(f64::INFINITY, f64::min);
    let vt_max = records
        .iter()
        .map(|r| r[4])
        .fold(f64::NEG_INFINITY, f64::max);
    let t_min = records.first().map(|r| r[3]).unwrap_or(f64::NAN);
    let t_max = records.last().map(|r| r[3]).unwrap_or(f64::NAN);
    eprintln!(
        "{out}: {} records ({} skipped), VTEC [{}, {}] TECU, t [{}, {}] unix",
        records.len(),
        skipped,
        vt_min,
        vt_max,
        t_min,
        t_max
    );
    eprintln!(
        "  records {} bytes, sha256 {}",
        bytes_out.len(),
        sha256_hex(&bytes_out)
    );
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == records => {
            eprintln!("  roundtrip: {} records parse, identical", parsed.len());
        }
        Some(parsed) => {
            eprintln!(
                "  roundtrip: {} records parse but differ from the emitted set",
                parsed.len()
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("  roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }

    println!("format swarm_tec");
    println!("cmap .");
    println!("lat Latitude");
    println!("lon Longitude");
    println!("field absolute_vtec_tecu absolute_vtec_tecu inverse-square em TECU 86400 0.0 0.0");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = arg_value(&args, "--probe") {
        probe(&path);
        return;
    }
    if let Some(path) = arg_value(&args, "--file") {
        let Some(out) = arg_value(&args, "--out") else {
            eprintln!("--out absent — the bin stays unnamed");
            std::process::exit(2);
        };
        compile(&path, &out);
        return;
    }
    eprintln!("usage: swarm_tec_compiler --probe <cdf> | --file <cdf> --out <bin>");
    std::process::exit(2);
}
