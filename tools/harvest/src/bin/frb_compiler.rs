use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::cdn::upload_asset;
use std::io::Write;
use std::process::Command;

const TABLE_URL: &str = "https://cdsarc.cds.unistra.fr/ftp/J/ApJS/257/59/table2.dat";
const MJD_UNIX_OFFSET: f64 = 40587.0;
const SECONDS_PER_DAY: f64 = 86400.0;
const MHZ_TO_HZ: f64 = 1_000_000.0;
const RPNAME_VOID: &str = "-9999";

const FIELDS: [(&str, usize, usize); 13] = [
    ("name", 0, 12),
    ("rpname", 29, 41),
    ("ra", 42, 53),
    ("dec", 63, 73),
    ("dm", 136, 142),
    ("dm_fitb", 171, 181),
    ("e_dm_fitb", 182, 189),
    ("nsb", 269, 270),
    ("mjd400", 271, 288),
    ("mjdinf", 303, 320),
    ("b_freq", 389, 394),
    ("b_freq_lo", 395, 400),
    ("flag", 433, 434),
];

fn fetch_table() -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("300")
        .arg("--compressed")
        .arg(TABLE_URL)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn mjd_to_tdb(mjd: f64, lsk: &omegaflow::lsk::LeapSeconds) -> Option<f64> {
    lsk.unix_to_tdb((mjd - MJD_UNIX_OFFSET) * SECONDS_PER_DAY)
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn field_str(line: &str, range: (usize, usize)) -> Option<String> {
    line.get(range.0..range.1)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn field_num(line: &str, range: (usize, usize)) -> Option<f64> {
    field_str(line, range).and_then(|s| s.parse().ok())
}

struct RowStats {
    rows_in: usize,
    rows_out: usize,
    with_epoch: usize,
    with_band: usize,
    with_dm: usize,
    repeater: usize,
    flagged: usize,
}

fn build_row(
    line: &str,
    lsk: &omegaflow::lsk::LeapSeconds,
    stats: &mut RowStats,
) -> Option<String> {
    stats.rows_in += 1;
    let name = match field_str(line, (FIELDS[0].1, FIELDS[0].2)) {
        Some(n) if !n.is_empty() => n,
        _ => return None,
    };
    let ra = match field_num(line, (FIELDS[2].1, FIELDS[2].2)) {
        Some(v) if v.is_finite() => v,
        _ => return None,
    };
    let dec = match field_num(line, (FIELDS[3].1, FIELDS[3].2)) {
        Some(v) if v.is_finite() => v,
        _ => return None,
    };
    let mut parts: Vec<(String, String)> = Vec::new();
    parts.push(("name".into(), json_str(&name)));
    parts.push(("ra".into(), format!("{}", ra)));
    parts.push(("dec".into(), format!("{}", dec)));
    if let Some(v) = field_num(line, (FIELDS[4].1, FIELDS[4].2)) {
        if v.is_finite() && v > 0.0 {
            parts.push(("dm".into(), format!("{}", v)));
            stats.with_dm += 1;
        }
    }
    if let Some(v) = field_num(line, (FIELDS[5].1, FIELDS[5].2)) {
        if v.is_finite() && v > 0.0 {
            parts.push(("dm_fitb".into(), format!("{}", v)));
        }
    }
    if let Some(v) = field_num(line, (FIELDS[6].1, FIELDS[6].2)) {
        if v.is_finite() && v > 0.0 {
            parts.push(("e_dm_fitb".into(), format!("{}", v)));
        }
    }
    if let Some(mjd) = field_num(line, (FIELDS[8].1, FIELDS[8].2)) {
        if mjd.is_finite() {
            if let Some(tdb) = mjd_to_tdb(mjd, lsk) {
                parts.push(("epoch_tdb".into(), format!("{}", tdb)));
                stats.with_epoch += 1;
            }
        }
    }
    if let Some(mjd) = field_num(line, (FIELDS[9].1, FIELDS[9].2)) {
        if mjd.is_finite() {
            if let Some(tdb) = mjd_to_tdb(mjd, lsk) {
                parts.push(("epoch_inf_tdb".into(), format!("{}", tdb)));
            }
        }
    }
    let hi = field_num(line, (FIELDS[10].1, FIELDS[10].2));
    let lo = field_num(line, (FIELDS[11].1, FIELDS[11].2));
    if let (Some(b), Some(b_lo)) = (hi, lo) {
        if b.is_finite() && b_lo.is_finite() && b > b_lo && b_lo > 0.0 {
            let freq_hz = (b + b_lo) * 0.5 * MHZ_TO_HZ;
            let bin_width_hz = (b - b_lo) * MHZ_TO_HZ;
            parts.push(("freq_hz".into(), format!("{}", freq_hz)));
            parts.push(("bin_width_hz".into(), format!("{}", bin_width_hz)));
            stats.with_band += 1;
        }
    }
    if let Some(rp) = field_str(line, (FIELDS[1].1, FIELDS[1].2)) {
        if !rp.is_empty() && rp != RPNAME_VOID {
            parts.push(("rpname".into(), json_str(&rp)));
            stats.repeater += 1;
        }
    }
    if let Some(v) = field_num(line, (FIELDS[12].1, FIELDS[12].2)) {
        if v.is_finite() {
            parts.push(("flag".into(), format!("{}", v)));
            if v == 1.0 {
                stats.flagged += 1;
            }
        }
    }
    if let Some(v) = field_num(line, (FIELDS[7].1, FIELDS[7].2)) {
        if v.is_finite() {
            parts.push(("nsb".into(), format!("{}", v)));
        }
    }
    stats.rows_out += 1;
    emit(parts)
}

fn emit(parts: Vec<(String, String)>) -> Option<String> {
    let mut out = String::from("{");
    for (k, (key, val)) in parts.iter().enumerate() {
        if k > 0 {
            out.push(',');
        }
        out.push_str(&json_str(key));
        out.push(':');
        out.push_str(val);
    }
    out.push('}');
    Some(out)
}

fn split_csv(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for c in line.chars() {
        match c {
            '"' if in_quotes => in_quotes = false,
            '"' => in_quotes = true,
            ',' if !in_quotes => {
                fields.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    fields.push(cur.trim().to_string());
    fields
}

fn build_cat2_row(
    fields: &[String],
    idx: &[(String, usize)],
    lsk: &omegaflow::lsk::LeapSeconds,
    stats: &mut RowStats,
) -> Option<String> {
    stats.rows_in += 1;
    let col = |key: &str| -> Option<usize> {
        idx.iter()
            .find(|(n, _)| n == key)
            .map(|(_, i)| *i)
            .filter(|&i| i < fields.len())
    };
    let get = |key: &str| -> Option<f64> {
        col(key)
            .and_then(|i| fields[i].parse::<f64>().ok())
            .filter(|v| v.is_finite())
    };
    let name = match col("tns_name").and_then(|i| {
        let n = fields[i].trim();
        if n.is_empty() {
            None
        } else {
            Some(n.to_string())
        }
    }) {
        Some(n) => n,
        None => return None,
    };
    let ra = match get("ra") {
        Some(v) => v,
        None => return None,
    };
    let dec = match get("dec") {
        Some(v) => v,
        None => return None,
    };
    let mut parts: Vec<(String, String)> = Vec::new();
    parts.push(("name".into(), json_str(&name)));
    parts.push(("ra".into(), format!("{}", ra)));
    parts.push(("dec".into(), format!("{}", dec)));
    if let Some(v) = get("bonsai_dm") {
        if v > 0.0 {
            parts.push(("dm".into(), format!("{}", v)));
            stats.with_dm += 1;
        }
    }
    if let Some(v) = get("dm_fitb") {
        if v > 0.0 {
            parts.push(("dm_fitb".into(), format!("{}", v)));
        }
    }
    if let Some(v) = get("dm_fitb_err") {
        if v > 0.0 {
            parts.push(("e_dm_fitb".into(), format!("{}", v)));
        }
    }
    if let Some(mjd) = get("mjd_400") {
        if let Some(tdb) = mjd_to_tdb(mjd, lsk) {
            parts.push(("epoch_tdb".into(), format!("{}", tdb)));
            stats.with_epoch += 1;
        }
    }
    if let Some(mjd) = get("mjd_inf") {
        if let Some(tdb) = mjd_to_tdb(mjd, lsk) {
            parts.push(("epoch_inf_tdb".into(), format!("{}", tdb)));
        }
    }
    let hi = get("high_freq");
    let lo = get("low_freq");
    if let (Some(b), Some(b_lo)) = (hi, lo) {
        if b > b_lo && b_lo > 0.0 {
            let freq_hz = (b + b_lo) * 0.5 * MHZ_TO_HZ;
            let bin_width_hz = (b - b_lo) * MHZ_TO_HZ;
            parts.push(("freq_hz".into(), format!("{}", freq_hz)));
            parts.push(("bin_width_hz".into(), format!("{}", bin_width_hz)));
            stats.with_band += 1;
        }
    }
    if let Some(i) = col("repeater_name") {
        let rp = fields[i].trim();
        if !rp.is_empty() {
            parts.push(("rpname".into(), json_str(rp)));
            stats.repeater += 1;
        }
    }
    if let Some(v) = get("excluded_flag") {
        parts.push(("flag".into(), format!("{}", v)));
        if v == 1.0 {
            stats.flagged += 1;
        }
    }
    if let Some(v) = get("sub_num") {
        parts.push(("nsb".into(), format!("{}", v)));
    }
    stats.rows_out += 1;
    emit(parts)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out = String::from("phi/frb_harvest/frb_chime_cat1.json");
    let mut ci_mode = false;
    let mut cat2_csv: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out = args.get(i + 1).cloned().unwrap_or_default();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--cat2" => {
                cat2_csv = args.get(i + 1).cloned();
                out = String::from("phi/frb_harvest/frb_chime_cat2.json");
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("naif0012 table void — the harvest stays unwritten (0 honored)");
            std::process::exit(1);
        }
    };
    let mut stats = RowStats {
        rows_in: 0,
        rows_out: 0,
        with_epoch: 0,
        with_band: 0,
        with_dm: 0,
        repeater: 0,
        flagged: 0,
    };
    let mut rows_out: Vec<String> = Vec::new();
    if let Some(csv_path) = cat2_csv {
        let body = match std::fs::read_to_string(&csv_path) {
            Ok(b) => b,
            Err(err) => {
                eprintln!(
                    "cat2-csv {} unreadable — the channel stays silent (0 honored): {}",
                    csv_path, err
                );
                std::process::exit(1);
            }
        };
        let mut lines = body.lines();
        let header = match lines.next() {
            Some(h) => h,
            None => {
                eprintln!(
                    "cat2-csv {} carries no header line — the channel stays silent",
                    csv_path
                );
                std::process::exit(1);
            }
        };
        let header_fields = split_csv(header);
        let idx: Vec<(String, usize)> = header_fields
            .into_iter()
            .enumerate()
            .map(|(i, n)| (n.trim().to_string(), i))
            .collect();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let fields = split_csv(line);
            if let Some(r) = build_cat2_row(&fields, &idx, &lsk, &mut stats) {
                rows_out.push(r);
            }
        }
        if rows_out.is_empty() {
            eprintln!(
                "cat2-csv carried {} rows, none carried away — the harvest stays unwritten",
                stats.rows_in
            );
            std::process::exit(1);
        }
    } else {
        let body = match fetch_table() {
            Some(b) => b,
            None => std::process::exit(1),
        };
        for line in body.lines() {
            if line.len() < 434 {
                continue;
            }
            if let Some(r) = build_row(line, &lsk, &mut stats) {
                rows_out.push(r);
            }
        }
        if rows_out.is_empty() {
            eprintln!(
                "table2.dat carried {} rows, none carried away — the harvest stays unwritten",
                stats.rows_in
            );
            std::process::exit(1);
        }
    }
    let mut buf = String::from("[");
    for (k, r) in rows_out.iter().enumerate() {
        if k > 0 {
            buf.push_str(",\n");
        }
        buf.push_str(r);
    }
    buf.push_str("]\n");
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                eprintln!("mkdir {} returned void: {}", parent.display(), err);
                std::process::exit(1);
            }
        }
    }
    match std::fs::File::create(&out) {
        Ok(mut f) => {
            if let Err(err) = f.write_all(buf.as_bytes()) {
                eprintln!("write {}: {}", out, err);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("write {} returned void: {}", out, err);
            std::process::exit(1);
        }
    }
    eprintln!(
        "frb_compiler: {}/{} Reihen → {} ({} B) — epoch {}, band {}, dm {}, repeater {}, flag {}",
        stats.rows_out,
        stats.rows_in,
        out,
        buf.len(),
        stats.with_epoch,
        stats.with_band,
        stats.with_dm,
        stats.repeater,
        stats.flagged,
    );
    if ci_mode && !upload_asset(&out) {
        eprintln!("upload: {} did not reach the CDN", out);
        std::process::exit(1);
    }
}
