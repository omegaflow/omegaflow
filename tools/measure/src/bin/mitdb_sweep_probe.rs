use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mitdb::{decimate, decode_212, envelope, parse_hea};
use omegaflow::te::topological_te_phase;

const BASE: &str = "https://physionet.org/files/mitdb/1.0.0/";
const BUCKET_S: f64 = 1.0;
const TARGET: usize = 300;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

struct Row {
    record: String,
    a: String,
    b: String,
    te_ab: f64,
    thr_ab: f64,
    te_ba: f64,
    thr_ba: f64,
}

fn verdict(te: f64, thr: f64) -> &'static str {
    if te > thr { "ARROW" } else { "still" }
}

fn binom_tail_onesided(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let mut comb = 1.0f64;
    let mut sum = 0.0f64;
    for i in (k..=n).rev() {
        sum += comb;
        if i > 0 {
            comb *= i as f64 / (n - i + 1) as f64;
        }
    }
    sum / 2f64.powi(n as i32)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    let limit: Option<usize> = arg_value(&args, "--limit").and_then(|v| v.parse().ok());
    let Some(records_text) = fetch_raw_bytes(&format!("{}RECORDS", BASE), 3600) else {
        eprintln!("RECORDS absent — the sweep stays still (0 honored)");
        return;
    };
    let mut records: Vec<String> = String::from_utf8_lossy(&records_text)
        .lines()
        .map(|l| l.trim_end_matches('\r').trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    records.sort();
    records.dedup();
    if let Some(l) = limit {
        records.truncate(l);
    }
    if records.is_empty() {
        eprintln!("RECORDS carries no entries (0 honored)");
        return;
    }

    let records = std::sync::Arc::new(records);
    let rows = std::sync::Mutex::new(Vec::<Row>::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    let skipped = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..jobs {
            let records = std::sync::Arc::clone(&records);
            let rows = &rows;
            let next = &next;
            let skipped = &skipped;
            s.spawn(move || {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    if i >= records.len() {
                        break;
                    }
                    let record = &records[i];
                    match measure(record) {
                        Some(row) => rows.lock().unwrap_or_else(|e| e.into_inner()).push(row),
                        None => {
                            eprintln!("{}: skipped (0 honored)", record);
                            skipped.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        }
                    }
                }
            });
        }
    });

    let mut rows = rows.into_inner().unwrap_or_else(|e| e.into_inner());
    rows.sort_by(|x, y| x.record.cmp(&y.record));

    println!(
        "{} recordings paired (two leads of the same heart each), {} skipped",
        rows.len(),
        skipped.load(std::sync::atomic::Ordering::SeqCst)
    );
    println!();
    println!(
        "{:>5} | {:>4} {:>4} | {:>9} {:>9} | {:>9} {:>9} | {:>5} {:>5}",
        "rec", "A", "B", "te(A→B)", "thr", "te(B→A)", "thr", "A→B", "B→A"
    );
    let mut arrow_ab = 0usize;
    let mut arrow_ba = 0usize;
    for r in &rows {
        let vab = r.te_ab > r.thr_ab;
        let vba = r.te_ba > r.thr_ba;
        if vab {
            arrow_ab += 1;
        }
        if vba {
            arrow_ba += 1;
        }
        println!(
            "{:>5} | {:>4} {:>4} | {:>9.4e} {:>9.4e} | {:>9.4e} {:>9.4e} | {:>5} {:>5}",
            r.record,
            r.a,
            r.b,
            r.te_ab,
            r.thr_ab,
            r.te_ba,
            r.thr_ba,
            verdict(r.te_ab, r.thr_ab),
            verdict(r.te_ba, r.thr_ba)
        );
    }
    println!();
    let n_dir = rows.len() * 2;
    let expected = n_dir as f64 * 0.023;
    println!(
        "Arrows: A→B {}, B→A {}, total {} of {} direction tests; chance expectation (threshold = mean+2σ) ≈ {:.1}",
        arrow_ab,
        arrow_ba,
        arrow_ab + arrow_ba,
        n_dir,
        expected
    );
    if arrow_ab + arrow_ba > expected as usize {
        println!(
            "Finding: {} arrows over the chance expectation — the lead geometry carries direction (a real finding would be checked).",
            arrow_ab + arrow_ba
        );
    } else {
        println!(
            "Silence: {} arrows lie at the chance expectation ({:.1}) — two leads of the same heart are symmetric (A = A, 0 honored).",
            arrow_ab + arrow_ba,
            expected
        );
    }
    println!();
    println!("=== breakdown by lead type ===");
    let limb = |s: &str| s.eq_ignore_ascii_case("MLII");
    let chest = |s: &str| {
        s.eq_ignore_ascii_case("V1")
            || s.eq_ignore_ascii_case("V2")
            || s.eq_ignore_ascii_case("V4")
            || s.eq_ignore_ascii_case("V5")
    };
    let mut chest_to_limb = 0usize;
    let mut limb_to_chest = 0usize;
    let mut chest_to_chest = 0usize;
    let mut limb_to_limb = 0usize;
    let mut n_chest_limb = 0usize;
    let mut n_chest_chest = 0usize;
    let mut n_limb_limb = 0usize;
    for r in &rows {
        if chest(&r.a) && limb(&r.b) {
            n_chest_limb += 1;
            if r.te_ab > r.thr_ab {
                chest_to_limb += 1;
            }
            if r.te_ba > r.thr_ba {
                limb_to_chest += 1;
            }
        } else if limb(&r.a) && chest(&r.b) {
            n_chest_limb += 1;
            if r.te_ab > r.thr_ab {
                limb_to_chest += 1;
            }
            if r.te_ba > r.thr_ba {
                chest_to_limb += 1;
            }
        } else if chest(&r.a) && chest(&r.b) {
            n_chest_chest += 1;
            if r.te_ab > r.thr_ab || r.te_ba > r.thr_ba {
                chest_to_chest += 1;
            }
        } else if limb(&r.a) && limb(&r.b) {
            n_limb_limb += 1;
            if r.te_ab > r.thr_ab || r.te_ba > r.thr_ba {
                limb_to_limb += 1;
            }
        }
    }
    println!(
        "Chest → limb: {}/{} recordings carry the arrow in the limb direction",
        chest_to_limb, n_chest_limb
    );
    println!(
        "Limb → chest: {}/{} recordings carry the arrow in the chest direction",
        limb_to_chest, n_chest_limb
    );
    println!(
        "Chest ↔ chest: {}/{} recordings carry any arrow",
        chest_to_chest, n_chest_chest
    );
    println!(
        "Limb ↔ limb: {}/{} recordings carry any arrow",
        limb_to_limb, n_limb_limb
    );

    println!();
    println!("=== breakdown by lead pair (exact lead names) ===");
    #[derive(Default)]
    struct PairAgg {
        n: usize,
        chest_to_limb: usize,
        limb_to_chest: usize,
        chest_chest: usize,
        limb_limb: usize,
    }
    let mut pairs: std::collections::BTreeMap<String, PairAgg> = std::collections::BTreeMap::new();
    for r in &rows {
        let mut names = [r.a.as_str(), r.b.as_str()];
        names.sort_unstable();
        let key = format!("{}↔{}", names[0], names[1]);
        let agg = pairs.entry(key).or_insert_with(PairAgg::default);
        agg.n += 1;
        let dirs = [
            (r.a.as_str(), r.b.as_str(), r.te_ab > r.thr_ab),
            (r.b.as_str(), r.a.as_str(), r.te_ba > r.thr_ba),
        ];
        for (src, dst, arrow) in dirs {
            if !arrow {
                continue;
            }
            let s_chest = chest(src);
            let d_chest = chest(dst);
            if s_chest && !d_chest {
                agg.chest_to_limb += 1;
            } else if !s_chest && d_chest {
                agg.limb_to_chest += 1;
            } else if s_chest && d_chest {
                agg.chest_chest += 1;
            } else {
                agg.limb_limb += 1;
            }
        }
    }
    println!(
        "{:>9} | {:>4} | {:>11} {:>11} | {:>11} {:>11} | {:>7}",
        "pair", "n", "chest→limb", "limb→chest", "chest↔chest", "limb↔limb", "P(one)"
    );
    for (key, agg) in &pairs {
        let one_sided = agg.chest_to_limb + agg.limb_to_chest;
        let pstr = if one_sided == 0 {
            "-".to_string()
        } else {
            format!("{:.4}", binom_tail_onesided(one_sided, agg.chest_to_limb))
        };
        println!(
            "{:>9} | {:>4} | {:>11} {:>11} | {:>11} {:>11} | {:>7}",
            key, agg.n, agg.chest_to_limb, agg.limb_to_chest, agg.chest_chest, agg.limb_limb, pstr
        );
    }
}

fn measure(record: &str) -> Option<Row> {
    let hea_url = format!("{}{}.hea", BASE, record);
    let dat_url = format!("{}{}.dat", BASE, record);
    let hea_bytes = fetch_raw_bytes(&hea_url, 3600)?;
    let hea = parse_hea(&String::from_utf8_lossy(&hea_bytes))?;
    if hea.nchan != 2 {
        return None;
    }
    if hea.leads.iter().any(|l| l.format != 212) {
        return None;
    }
    let dat_bytes = fetch_raw_bytes(&dat_url, 3600)?;
    let expected = hea.nsamp.checked_mul(3)?;
    if dat_bytes.len() != expected {
        return None;
    }
    let (ch0, ch1) = decode_212(&dat_bytes, hea.nsamp)?;
    let a = envelope(
        &ch0,
        hea.leads[0].gain,
        hea.leads[0].adc_zero,
        hea.sample_rate,
        BUCKET_S,
    );
    let b = envelope(
        &ch1,
        hea.leads[1].gain,
        hea.leads[1].adc_zero,
        hea.sample_rate,
        BUCKET_S,
    );
    let n = a.len().min(b.len());
    let a = decimate(&a[..n], TARGET);
    let b = decimate(&b[..n], TARGET);
    let n = a.len().min(b.len());
    let a: Vec<f32> = a[..n].to_vec();
    let b: Vec<f32> = b[..n].to_vec();
    if n < 32 {
        return None;
    }
    let topo_ab = topological_te_phase(&b, &a, 3, 3, SEED);
    let topo_ba = topological_te_phase(&a, &b, 3, 3, SEED);
    let (te_ab, thr_ab) = topo_ab.map_or((f64::NAN, f64::NAN), |t| (t.te, t.threshold));
    let (te_ba, thr_ba) = topo_ba.map_or((f64::NAN, f64::NAN), |t| (t.te, t.threshold));
    Some(Row {
        record: record.to_string(),
        a: hea.leads[0].name.clone(),
        b: hea.leads[1].name.clone(),
        te_ab,
        thr_ab,
        te_ba,
        thr_ba,
    })
}
