use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::tdat::{
    self, AMS02_KIND_ENERGY, AMS02_KIND_RIGIDITY, AMS02_ROW_HIGH, AMS02_ROW_KIND, AMS02_ROW_LOW,
    AMS02_ROW_SPECIES, AMS02_ROW_TDB, AMS02_ROW_TDB_END,
};
use omegaflow::{maxi, odf};

const URL: &str =
    "https://heasarc.gsfc.nasa.gov/FTP/heasarc/dbase/tdat_files/heasarc_ams02spec.tdat.gz";
const NETLOC: &str = "heasarc.gsfc.nasa.gov";
const PREFIX: &str = "ams02_spec";
const DIR: &str = "data/heasarc.gsfc.nasa.gov";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let bytes: Vec<u8> = match args
        .iter()
        .position(|a| a == "--input")
        .and_then(|i| args.get(i + 1))
    {
        Some(path) => match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(URL) {
            Some(b) => b,
            None => {
                eprintln!("{URL}: fetch void");
                std::process::exit(1);
            }
        },
    };
    let plain = match gunzip(&bytes) {
        Some(p) => p,
        None => {
            eprintln!(
                "{URL}: gzip stream stays unreadable — the series stays unwritten (0 honored)"
            );
            std::process::exit(1);
        }
    };
    let Some(table) = tdat::parse_tdat(&plain) else {
        eprintln!(
            "ams02spec: {} B carry no <HEADER>/<DATA> table (0 honored)",
            plain.len()
        );
        std::process::exit(1);
    };
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let mut merged: Vec<[f64; 9]> = Vec::new();
    let mut skipped = 0usize;
    let mut species_seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for i in 0..table.rows.len() {
        let time = table.f64_cell(i, "time");
        let end_time = table.f64_cell(i, "end_time");
        let species = table.cell(i, "species");
        let (Some(time), Some(species)) = (time, species) else {
            skipped += 1;
            continue;
        };
        let Some(species) = species else {
            skipped += 1;
            continue;
        };
        if species == "<END>" {
            continue;
        }
        let energy_min = table.f64_cell(i, "energy_min");
        let energy_max = table.f64_cell(i, "energy_max");
        let rigidity_min = table.f64_cell(i, "rigidity_min");
        let rigidity_max = table.f64_cell(i, "rigidity_max");
        let (kind, low, high) = match (energy_min, energy_max, rigidity_min, rigidity_max) {
            (Some(lo), Some(hi), _, _) => (AMS02_KIND_ENERGY, lo, hi),
            (_, _, Some(lo), Some(hi)) => (AMS02_KIND_RIGIDITY, lo, hi),
            _ => {
                skipped += 1;
                continue;
            }
        };
        let Some(species_id) = tdat::ams02_species_id(species) else {
            skipped += 1;
            continue;
        };
        let Some(tdb) = maxi::mjd_to_tdb(time, &lsk) else {
            skipped += 1;
            continue;
        };
        let tdb_end = match end_time {
            Some(e) => maxi::mjd_to_tdb(e, &lsk),
            None => None,
        };
        let mut row = [0.0f64; 9];
        row[AMS02_ROW_TDB] = tdb;
        row[AMS02_ROW_LOW] = low;
        row[AMS02_ROW_HIGH] = high;
        row[AMS02_ROW_SPECIES] = species_id as f64;
        row[AMS02_ROW_KIND] = kind as f64;
        row[AMS02_ROW_TDB_END] = match tdb_end {
            Some(v) => v,
            None => continue,
        };
        species_seen.insert(species.to_string());
        merged.push(row);
    }
    if merged.is_empty() {
        eprintln!(
            "no AMS-02 rows compiled ({skipped} skipped) — the series stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    eprintln!(
        "ams02spec: {} rows kept ({skipped} skipped), species {:?}",
        merged.len(),
        species_seen
    );
    std::fs::create_dir_all(DIR).ok();
    let out = format!("{DIR}/{PREFIX}.bin");
    let bin = odf::write_podf_bin(&merged);
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            eprintln!(
                "{out}: {} AMS-02 spectral rows (tdb {}..{}), {} B — roundtrip parses",
                parsed.len(),
                d0[0],
                d1[0],
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
