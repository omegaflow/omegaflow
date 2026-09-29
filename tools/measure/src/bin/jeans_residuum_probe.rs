use std::collections::HashMap;

use omegaflow::archivar::PARSEC_M;
use omegaflow::archivar::spatial::{
    STAR_RECORD_BYTES, parse_star_record, star_position_at, star_stride,
};

const DEFAULT_STARS: &str = "data/ssd.jpl.nasa.gov/dr3_stars.bin";
const DEFAULT_CELL_PC: f64 = 50.0;
const MIN_STARS: usize = 32;
const PLX_STRONG_MAS: f64 = 5.0;
const PLX_WEAK_MAS: f64 = 2.0;

type CellKey = (i64, i64, i64);

struct CellBin {
    n_total: usize,
    n_plx5: usize,
    n_plx2: usize,
    plx_mas: Vec<f64>,
}

impl CellBin {
    fn new() -> Self {
        CellBin {
            n_total: 0,
            n_plx5: 0,
            n_plx2: 0,
            plx_mas: Vec::new(),
        }
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn cell_key(p: [f64; 3], cell: f64) -> CellKey {
    (
        (p[0] / cell).floor() as i64,
        (p[1] / cell).floor() as i64,
        (p[2] / cell).floor() as i64,
    )
}

fn median_f64(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    let n = v.len();
    if n % 2 == 1 {
        Some(v[n / 2])
    } else {
        Some(0.5 * (v[n / 2 - 1] + v[n / 2]))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let stars_path = match arg_value(&args, "--stars") {
        Some(v) => v,
        None => DEFAULT_STARS.to_string(),
    };
    let cell_pc = match arg_value(&args, "--cell-pc") {
        Some(v) => match v.parse::<f64>() {
            Ok(x) if x.is_finite() && x > 0.0 => x,
            _ => {
                eprintln!("--cell-pc wants a positive finite pc value, read: {v}");
                std::process::exit(2);
            }
        },
        None => DEFAULT_CELL_PC,
    };

    let bytes = match std::fs::read(&stars_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {stars_path} returned void: {e}");
            std::process::exit(2);
        }
    };
    let Some(stride) = star_stride(&bytes) else {
        eprintln!(
            "star bin {} bytes: no {}-byte records — pending recompilation, the census stays absent",
            bytes.len(),
            STAR_RECORD_BYTES
        );
        std::process::exit(2);
    };
    let record_total = bytes.len() / stride;
    let cell_m = cell_pc * PARSEC_M;

    let mut bins: HashMap<CellKey, CellBin> = HashMap::new();
    let mut records_parsed = 0usize;
    for chunk in bytes.chunks_exact(stride) {
        let Some(rec) = parse_star_record(chunk) else {
            continue;
        };
        records_parsed += 1;
        let (p, _) = star_position_at(&rec, 0.0);
        let bin = bins.entry(cell_key(p, cell_m)).or_insert_with(CellBin::new);
        bin.n_total += 1;
        if rec.plx_mas > PLX_STRONG_MAS {
            bin.n_plx5 += 1;
        }
        if rec.plx_mas > PLX_WEAK_MAS {
            bin.n_plx2 += 1;
        }
        bin.plx_mas.push(rec.plx_mas);
    }
    let refused = record_total - records_parsed;

    let occupied = bins.len();
    let cells_ge_min = bins.values().filter(|b| b.n_total >= MIN_STARS).count();

    let n5_global: usize = bins.values().map(|b| b.n_plx5).sum();
    let n2_global: usize = bins.values().map(|b| b.n_plx2).sum();
    let cells_with_n5 = bins.values().filter(|b| b.n_plx5 >= 1).count();
    let cells_with_n2 = bins.values().filter(|b| b.n_plx2 >= 1).count();

    let mut cell_counts: Vec<f64> = bins.values().map(|b| b.n_total as f64).collect();
    let mut cell_n5: Vec<f64> = bins.values().map(|b| b.n_plx5 as f64).collect();
    let mut cell_n2: Vec<f64> = bins.values().map(|b| b.n_plx2 as f64).collect();
    let median_n_total = median_f64(&mut cell_counts);
    let median_cell_n5 = median_f64(&mut cell_n5);
    let median_cell_n2 = median_f64(&mut cell_n2);

    let mut cell_medians: Vec<f64> = Vec::with_capacity(occupied);
    for bin in bins.values_mut() {
        if let Some(m) = median_f64(&mut bin.plx_mas) {
            cell_medians.push(m);
        }
    }
    let median_of_cell_medians = median_f64(&mut cell_medians);

    println!("jeans_residuum_probe — census mode: per-voxel stellar kinematics");
    println!(
        "stars: {stars_path} ({} B, {record_total} records of {stride} B)",
        bytes.len()
    );
    println!("cell: {cell_pc} pc = {cell_m} m");
    println!();
    println!(
        "read: {records_parsed} parsed (positive parallax) | {refused} refused (non-finite or parallax <= 0)"
    );
    println!("occupied cells (>= 1 star): {occupied}");
    println!("cells >= MIN_STARS ({MIN_STARS}): {cells_ge_min}");
    match median_n_total {
        Some(m) => println!("median N_total over occupied cells: {m}"),
        None => println!("median N_total over occupied cells: absent (no occupied cell)"),
    }
    match median_of_cell_medians {
        Some(m) => println!("median over occupied cells of per-cell median parallax: {m} mas"),
        None => println!("median over occupied cells of per-cell median parallax: absent"),
    }
    println!();
    match records_parsed {
        0 => println!(
            "global N(plx > {PLX_STRONG_MAS} mas): {n5_global} (fraction absent, no parsed star)"
        ),
        n => println!(
            "global N(plx > {PLX_STRONG_MAS} mas): {n5_global} ({:.4})",
            n5_global as f64 / n as f64
        ),
    }
    match records_parsed {
        0 => println!(
            "global N(plx > {PLX_WEAK_MAS} mas): {n2_global} (fraction absent, no parsed star)"
        ),
        n => println!(
            "global N(plx > {PLX_WEAK_MAS} mas): {n2_global} ({:.4})",
            n2_global as f64 / n as f64
        ),
    }
    println!("cells with >= 1 star plx > {PLX_STRONG_MAS} mas: {cells_with_n5}");
    println!("cells with >= 1 star plx > {PLX_WEAK_MAS} mas: {cells_with_n2}");
    match median_cell_n5 {
        Some(m) => println!("median per-cell N(plx > {PLX_STRONG_MAS} mas): {m}"),
        None => {
            println!("median per-cell N(plx > {PLX_STRONG_MAS} mas): absent (no occupied cell)")
        }
    }
    match median_cell_n2 {
        Some(m) => println!("median per-cell N(plx > {PLX_WEAK_MAS} mas): {m}"),
        None => println!("median per-cell N(plx > {PLX_WEAK_MAS} mas): absent (no occupied cell)"),
    }
}
