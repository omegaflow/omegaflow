use omegaflow::archivar::trisp_nse::{self, TrispNseTable};

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_text(path: &std::path::Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    String::from_utf8(bytes).ok()
}

fn asset_name(dat_path: &str) -> String {
    let base = dat_path.rsplit('/').next().unwrap_or(dat_path);
    let stem = base.strip_suffix(".dat").unwrap_or(base);
    format!("{stem}.bin")
}

fn print_inventory(table: &TrispNseTable) {
    eprintln!(
        "  {} row(s), {} column(s), {} log key(s); tau unit {}, TTA unit {}",
        table.rows.len(),
        table.columns.len(),
        table.log.len(),
        trisp_nse::TAU_UNIT,
        trisp_nse::TTA_UNIT
    );
}

fn compile_pair(dat_path: &std::path::Path, out_dir: &str) -> Option<String> {
    let dat_text = read_text(dat_path)?;
    let Some(mut table) = trisp_nse::parse_dat(&dat_text) else {
        eprintln!("parse void — the bin stays unwritten (0 honored)");
        return None;
    };
    let log_path = dat_path.with_extension("log");
    if let Some(log_text) = read_text(&log_path) {
        table.log = trisp_nse::parse_log(&log_text);
    }
    let bin = trisp_nse::pack(&table);
    let Some(parsed) = trisp_nse::parse_bin(&bin) else {
        eprintln!("roundtrip void — unverified");
        return None;
    };
    if parsed != table {
        eprintln!("roundtrip mismatch — unverified");
        return None;
    }
    let asset = asset_name(&dat_path.to_string_lossy());
    let out_path = std::path::Path::new(out_dir).join(&asset);
    if let Some(parent) = out_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("write returned void");
        return None;
    }
    eprintln!("packed {} byte(s)", bin.len());
    print_inventory(&table);
    Some(asset)
}

fn collect_pairs(dir: &str) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().is_some_and(|x| x == "dat") {
            out.push(p);
        }
    }
    out.sort();
    out
}

fn main() {
    omegaflow::archivar::cdn::require_ci();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dir = match arg_value(&args, "--dir") {
        Some(d) => d,
        None => {
            eprintln!("usage: trisp_nse_compiler --dir <path> [--out <path>]");
            std::process::exit(2);
        }
    };
    let out_dir = match arg_value(&args, "--out") {
        Some(d) => d,
        None => dir.clone(),
    };
    let pairs = collect_pairs(&dir);
    if pairs.is_empty() {
        eprintln!("no .dat file found — nothing written (0 honored)");
        std::process::exit(1);
    }
    let mut written = 0usize;
    for p in &pairs {
        if compile_pair(p, &out_dir).is_some() {
            written += 1;
        }
    }
    if written == 0 {
        eprintln!("no table packed — nothing written (0 honored)");
        std::process::exit(1);
    }
    eprintln!("{written} table(s) packed — output stays local");
}
