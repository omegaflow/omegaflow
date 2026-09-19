use std::collections::{BTreeSet, HashMap};

use omegaflow::archivar::cdn::{CDN_BASE, CDN_RELEASE};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::spatial::{
    parse_star_record, star_position_at, star_stride, STAR_RECORD_BYTES,
};
use omegaflow::te::{benjamini_hochberg, gaussian, silverman};

const STARS_CDN_FILE: &str = "dr3_stars.bin";
const MIN_STARS: usize = 32;
const MIN_CELLS: usize = 8;
const MIN_EXPECT_COUNT: f64 = 0.5;
const FDR_LEVEL: f64 = 0.05;

type CellKey = (i64, i64, i64);

#[derive(Debug, PartialEq)]
struct SilenceMap {
    total_cells: usize,
    still_cells: usize,
    absent_cells: usize,
    blind_cells: usize,
    consistent_cells: usize,
    fdr_cutoff: f64,
    fdr_level: f64,
    deficits: Vec<f64>,
    counts: Vec<usize>,
}

struct Catalog {
    points: Vec<[f64; 3]>,
    records: usize,
    refused: usize,
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

fn center_of(k: CellKey, cell: f64) -> [f64; 3] {
    [
        (k.0 as f64 + 0.5) * cell,
        (k.1 as f64 + 0.5) * cell,
        (k.2 as f64 + 0.5) * cell,
    ]
}

fn dist2(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

fn is_power_of_two(v: f64) -> bool {
    if !(v.is_finite() && v > 0.0) {
        return false;
    }
    let bits = v.to_bits();
    let mantissa = bits & ((1u64 << 52) - 1);
    mantissa == 0
}

fn ceil_power_of_two(v: f64) -> Option<f64> {
    if !(v.is_finite() && v > 0.0) {
        return None;
    }
    Some(2f64.powi(v.log2().ceil() as i32))
}

fn ln_factorial(k: u64) -> f64 {
    let mut s = 0.0;
    for i in 2..=k {
        s += (i as f64).ln();
    }
    s
}

fn poisson_lower_tail(count: u64, lam: f64) -> f64 {
    if !(lam > 0.0) {
        return 1.0;
    }
    let l_lam = lam.ln();
    let mut max = f64::NEG_INFINITY;
    let mut terms: Vec<f64> = Vec::with_capacity(count as usize + 1);
    for k in 0..=count {
        let log_term = -lam + (k as f64) * l_lam - ln_factorial(k);
        if log_term > max {
            max = log_term;
        }
        terms.push(log_term);
    }
    let sum: f64 = terms.iter().map(|t| (t - max).exp()).sum();
    (max + sum.ln()).exp().min(1.0)
}

fn median_nearest_neighbor(points: &[[f64; 3]]) -> Option<f64> {
    if points.len() < 2 {
        return None;
    }
    let mut order: Vec<usize> = (0..points.len()).collect();
    order.sort_by(|&a, &b| points[a][0].total_cmp(&points[b][0]));
    let mut nn: Vec<f64> = Vec::with_capacity(points.len());
    for pos in 0..order.len() {
        let i = order[pos];
        let pi = points[i];
        let mut best = f64::INFINITY;
        let mut left = pos;
        while left > 0 {
            left -= 1;
            let j = order[left];
            let dx = pi[0] - points[j][0];
            if dx * dx >= best {
                break;
            }
            let d2 = dist2(pi, points[j]);
            if d2 < best {
                best = d2;
            }
        }
        for right in (pos + 1)..order.len() {
            let j = order[right];
            let dx = points[j][0] - pi[0];
            if dx * dx >= best {
                break;
            }
            let d2 = dist2(pi, points[j]);
            if d2 < best {
                best = d2;
            }
        }
        if best.is_finite() {
            nn.push(best.sqrt());
        }
    }
    if nn.is_empty() {
        return None;
    }
    nn.sort_by(f64::total_cmp);
    Some(nn[nn.len() / 2])
}

fn bandwidths(points: &[[f64; 3]], cell: f64) -> Option<[f64; 3]> {
    let xs: Vec<f32> = points.iter().map(|p| p[0] as f32).collect();
    let ys: Vec<f32> = points.iter().map(|p| p[1] as f32).collect();
    let zs: Vec<f32> = points.iter().map(|p| p[2] as f32).collect();
    Some([
        silverman(&xs)?.max(cell),
        silverman(&ys)?.max(cell),
        silverman(&zs)?.max(cell),
    ])
}

fn load_catalog(bytes: &[u8]) -> Option<Catalog> {
    let stride = star_stride(bytes)?;
    let records = bytes.len() / stride;
    let mut points: Vec<[f64; 3]> = Vec::new();
    let mut refused = 0usize;
    for chunk in bytes.chunks_exact(stride) {
        match parse_star_record(chunk) {
            Some(rec) => points.push(star_position_at(&rec, 0.0).0),
            None => refused += 1,
        }
    }
    Some(Catalog {
        points,
        records,
        refused,
    })
}

fn silence_map(points: &[[f64; 3]], cell: f64) -> Option<SilenceMap> {
    if points.len() < MIN_STARS {
        return None;
    }
    let band = bandwidths(points, cell)?;
    let mut counts: HashMap<CellKey, usize> = HashMap::new();
    for p in points {
        let entry = counts.entry(cell_key(*p, cell)).or_insert(0);
        *entry += 1;
    }
    let mut evaluated: BTreeSet<CellKey> = BTreeSet::new();
    for k in counts.keys() {
        for di in -1i64..=1 {
            for dj in -1i64..=1 {
                for dk in -1i64..=1 {
                    evaluated.insert((k.0 + di, k.1 + dj, k.2 + dk));
                }
            }
        }
    }
    if evaluated.len() < MIN_CELLS {
        return None;
    }
    let keys: Vec<CellKey> = evaluated.into_iter().collect();
    let volume = cell * cell * cell;
    let mut expectation: Vec<f64> = Vec::with_capacity(keys.len());
    for k in &keys {
        let c = center_of(*k, cell);
        let mut sum = 0.0;
        for p in points {
            sum += gaussian(c[0] - p[0], band[0])
                * gaussian(c[1] - p[1], band[1])
                * gaussian(c[2] - p[2], band[2]);
        }
        expectation.push(sum * volume);
    }
    let mut absent = 0usize;
    let mut blind = 0usize;
    let mut deficits: Vec<f64> = Vec::new();
    let mut obs_counts: Vec<usize> = Vec::new();
    let mut p_values: Vec<f64> = Vec::new();
    for (i, k) in keys.iter().enumerate() {
        let count = match counts.get(k) {
            Some(c) => *c,
            None => 0,
        };
        let lam = expectation[i];
        if count == 0 && lam < MIN_EXPECT_COUNT {
            absent += 1;
            continue;
        }
        let threshold = lam - 2.0 * lam.sqrt();
        if threshold <= 0.0 {
            blind += 1;
            continue;
        }
        deficits.push(count as f64 - threshold);
        obs_counts.push(count);
        p_values.push(poisson_lower_tail(count as u64, lam));
    }
    let cutoff = match benjamini_hochberg(&p_values, FDR_LEVEL) {
        Some(c) => c,
        None => 0.0,
    };
    let still = p_values
        .iter()
        .filter(|&&p| cutoff > 0.0 && p <= cutoff)
        .count();
    let consistent = p_values.len() - still;
    Some(SilenceMap {
        total_cells: keys.len(),
        still_cells: still,
        absent_cells: absent,
        blind_cells: blind,
        consistent_cells: consistent,
        fdr_cutoff: cutoff,
        fdr_level: FDR_LEVEL,
        deficits,
        counts: obs_counts,
    })
}

fn summarize(v: &[f64]) -> (f64, f64, f64) {
    let n = v.len() as f64;
    let mut sum = 0.0;
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for x in v {
        sum += x;
        if *x < lo {
            lo = *x;
        }
        if *x > hi {
            hi = *x;
        }
    }
    (lo, sum / n, hi)
}

fn histogram(counts: &[usize]) -> String {
    let mut bins = [0usize; 7];
    for c in counts {
        bins[(*c).min(6)] += 1;
    }
    let mut parts = Vec::with_capacity(7);
    for (i, b) in bins.iter().enumerate() {
        if i == 6 {
            parts.push(format!("6+:{b}"));
        } else {
            parts.push(format!("{i}:{b}"));
        }
    }
    parts.join(" ")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let cell: Option<f64> = match arg_value(&args, "--cell") {
        Some(v) => match v.parse::<f64>() {
            Ok(c) if is_power_of_two(c) => Some(c),
            Ok(c) => {
                eprintln!(
                    "--cell {c}: not a power-of-two meter edge (2^n m) — the grid stays unbuilt"
                );
                std::process::exit(2);
            }
            Err(..) => {
                eprintln!("--cell {v}: not a finite meter edge — the grid stays unbuilt");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let stars_arg = arg_value(&args, "--stars");
    let cdn_url = format!("{CDN_BASE}/{CDN_RELEASE}/{STARS_CDN_FILE}");
    let (label, bytes): (String, Vec<u8>) = match &stars_arg {
        Some(path) => match std::fs::read(path) {
            Ok(b) => (path.clone(), b),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(2);
            }
        },
        None => match fetch_raw_bytes(&cdn_url, 3600) {
            Some(b) => (cdn_url.clone(), b),
            None => {
                eprintln!("dr3_stars.bin: the CDN release {cdn_url} carried no bytes");
                std::process::exit(2);
            }
        },
    };

    let Some(catalog) = load_catalog(&bytes) else {
        eprintln!(
            "star bin {} bytes: no {STAR_RECORD_BYTES}-byte records — the catalog stays unread",
            bytes.len()
        );
        std::process::exit(2);
    };

    let cell_m: f64 = match cell {
        Some(c) => c,
        None => match median_nearest_neighbor(&catalog.points) {
            Some(d) => match ceil_power_of_two(d) {
                Some(p) => p,
                None => {
                    eprintln!("median neighbor distance {d}: no power-of-two edge — the grid stays unbuilt");
                    std::process::exit(2);
                }
            },
            None => {
                eprintln!(
                    "fewer than two positioned stars — the median neighbor distance is absent (0 honored)"
                );
                std::process::exit(2);
            }
        },
    };

    println!(
        "=== silence-map-probe — catalog deficit against the BH-corrected null ==="
    );
    println!(
        "catalog {label}: {} records | {} positioned stars | {} refused",
        catalog.records,
        catalog.points.len(),
        catalog.refused
    );
    println!(
        "cell {cell_m} m (2^{}) | FDR level {FDR_LEVEL}",
        cell_m.log2() as i64
    );

    let band = match bandwidths(&catalog.points, cell_m) {
        Some(b) => b,
        None => {
            eprintln!("the position field carries no variance — no null bandwidth (0 honored)");
            std::process::exit(2);
        }
    };
    println!(
        "null bandwidth h (Silverman floored at the cell edge — the deficit resolution, per axis): hx {:.6e} m | hy {:.6e} m | hz {:.6e} m — a hole smaller than h is constructively invisible",
        band[0], band[1], band[2]
    );

    match silence_map(&catalog.points, cell_m) {
        Some(m) => {
            println!("evaluated cells (occupied + 1-cell dilation): {}", m.total_cells);
            println!(
                "blind cells (lambda_hat <= 4, a deficit below the null floor is untestable): {} / {}",
                m.blind_cells, m.total_cells
            );
            println!(
                "still cells (BH-significant deficit, lower-tail Poisson p): {} / {}",
                m.still_cells, m.total_cells
            );
            println!(
                "absent cells (no observed stars and no expectation): {} / {}",
                m.absent_cells, m.total_cells
            );
            println!(
                "consistent cells: {} / {}",
                m.consistent_cells, m.total_cells
            );
            let testable = m.still_cells + m.consistent_cells;
            println!(
                "BH FDR level {} over {testable} testable cells | cutoff {:.6} | significant still cells {} / {testable}",
                m.fdr_level, m.fdr_cutoff, m.still_cells
            );
            if m.still_cells > 0 {
                println!(
                    "map verdict: catalog deficit holds — at least one cell carries a BH-significant deficit"
                );
            } else {
                println!(
                    "map verdict: no catalog deficit over the BH-corrected null — no cell carries a significant deficit"
                );
            }
            println!(
                "confound: the catalog deficit is a property of the catalog selection function (magnitude limit, survey coverage), not of the field of bodies"
            );
            println!(
                "0-Kanon: the registers phi/sources.φ and phi/pipeline/ledger.φ carry no sky footprint (82 ra/dec point columns, 0 coverage/healpix/polygon fields, measured 2026-09-18) — a still cell cannot be bound to a registered source, so the cell's zero stays a property of the catalog selection function, not a pending harvest"
            );
            if !m.deficits.is_empty() {
                let (dmin, dmean, dmax) = summarize(&m.deficits);
                println!(
                    "deficit distribution (observed count - threshold, per testable cell): min {dmin:.6e} | mean {dmean:.6e} | max {dmax:.6e}"
                );
            }
            println!(
                "observed star-count distribution per testable cell: {}",
                histogram(&m.counts)
            );
            println!(
                "verdict tally: {} still | {} blind | {} absent | {} consistent | {} total cells",
                m.still_cells, m.blind_cells, m.absent_cells, m.consistent_cells, m.total_cells
            );
        }
        None => {
            eprintln!(
                "below the measurement floor ({} stars / {} cells minimum) — no verdict (0 honored)",
                MIN_STARS, MIN_CELLS
            );
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POISSON_NORMAL_LAMBDA: f64 = 700.0;

    fn next_rng(rng: &mut u64) -> f64 {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
    }

    fn poisson(lambda: f64, rng: &mut u64) -> u64 {
        if lambda <= 0.0 {
            return 0;
        }
        if lambda > POISSON_NORMAL_LAMBDA {
            let u1 = next_rng(rng).max(1.0e-12);
            let u2 = next_rng(rng);
            let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            return (lambda + z * lambda.sqrt()).max(0.0).round() as u64;
        }
        let l = (-lambda).exp();
        let mut k: u64 = 0;
        let mut p = 1.0;
        loop {
            k += 1;
            p *= next_rng(rng);
            if p <= l {
                break;
            }
        }
        k - 1
    }

    fn poisson_field(ncells: usize, cell: f64, density: f64, rng: &mut u64) -> Vec<[f64; 3]> {
        let side = ncells as f64 * cell;
        let total = poisson(density * (ncells as f64).powi(3), rng) as usize;
        let mut points = Vec::with_capacity(total);
        for _ in 0..total {
            points.push([
                next_rng(rng) * side,
                next_rng(rng) * side,
                next_rng(rng) * side,
            ]);
        }
        points
    }

    #[test]
    fn homogeneous_poisson_field_stays_near_chance() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let field = poisson_field(8, 1.0, 10.0, &mut rng);
        let map = silence_map(&field, 1.0)
            .expect("a homogeneous field must be measurable");
        let testable = map.still_cells + map.consistent_cells;
        assert!(testable > 0, "FP gate: the homogeneous field carries no testable cells");
        let fraction = map.still_cells as f64 / testable as f64;
        assert!(
            fraction < 0.1,
            "FP gate: a homogeneous Poisson field reports {:.4} still fraction — the null does not hold",
            fraction
        );
    }

    #[test]
    fn an_inserted_hole_is_detected() {
        let mut rng = 0x517C_C1B7_2722_0A95u64;
        let field = poisson_field(8, 1.0, 25.0, &mut rng);
        let baseline = silence_map(&field, 1.0)
            .expect("the baseline field must be measurable");
        let hole_lo = 3.0;
        let hole_hi = 5.0;
        let holed: Vec<[f64; 3]> = field
            .iter()
            .filter(|p| {
                !(p[0] >= hole_lo
                    && p[0] < hole_hi
                    && p[1] >= hole_lo
                    && p[1] < hole_hi
                    && p[2] >= hole_lo
                    && p[2] < hole_hi)
            })
            .copied()
            .collect();
        let holed_map = silence_map(&holed, 1.0)
            .expect("the holed field must be measurable");
        assert!(
            holed_map.still_cells > baseline.still_cells,
            "FN gate: the hole is not detected ({} still vs {} baseline)",
            holed_map.still_cells,
            baseline.still_cells
        );
        assert!(
            holed_map.fdr_cutoff > 0.0,
            "BH gate: the detected hole carries no positive FDR cutoff"
        );
    }

    #[test]
    fn a_low_density_field_names_its_cells_blind() {
        let mut rng = 0x6A09_E667_F3BC_C909u64;
        let field = poisson_field(8, 1.0, 1.0, &mut rng);
        let map = silence_map(&field, 1.0)
            .expect("a lambda=1 field must be measurable");
        assert_eq!(
            map.still_cells, 0,
            "blind gate: a lambda=1 field must carry no still cells"
        );
        assert_eq!(
            map.consistent_cells, 0,
            "blind gate: a lambda=1 field must carry no consistent cells"
        );
        assert!(
            map.blind_cells > 0,
            "blind gate: a lambda=1 field must name its untestable cells as blind"
        );
    }

    #[test]
    fn a_deficit_free_field_does_not_hold_the_map_verdict() {
        let mut rng = 0xBB67_AE85_84CA_A73Bu64;
        let field = poisson_field(8, 1.0, 10.0, &mut rng);
        let map = silence_map(&field, 1.0)
            .expect("a homogeneous field must be measurable");
        let testable = map.still_cells + map.consistent_cells;
        assert!(testable > 0, "BH gate: the homogeneous field carries no testable cells");
        assert!(
            map.still_cells as f64 <= FDR_LEVEL * testable as f64,
            "BH gate: a deficit-free field reports {} still of {} testable — above the FDR budget",
            map.still_cells,
            testable
        );
    }

    #[test]
    fn identical_inputs_measure_equal() {
        let mut rng = 0x2722_0A95_517C_C1B7u64;
        let field = poisson_field(6, 1.0, 8.0, &mut rng);
        let a = silence_map(&field, 1.0);
        let b = silence_map(&field, 1.0);
        match (a, b) {
            (Some(x), Some(y)) => assert_eq!(
                x, y,
                "symmetry gate: identical input and seed measure unequal"
            ),
            (None, None) => {}
            _ => panic!("symmetry gate: one run measurable, the other not"),
        }
    }

    #[test]
    fn n_floor_no_statement_below_minimum() {
        let points = vec![
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0],
            [3.0, 3.0, 3.0],
        ];
        let map = silence_map(&points, 1.0);
        assert!(
            map.is_none(),
            "n-floor gate: a 4-star field must carry no verdict"
        );
    }
}
