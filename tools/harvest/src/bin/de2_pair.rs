use omegaflow::de2_raw::{self, IdmAxis};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: de2_pair <wats.asc> <idm.asc> [max_dt_ms]");
        std::process::exit(2);
    }
    let max_dt: i64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1500);
    let wats_text = std::fs::read_to_string(&args[1]).expect("read wats");
    let idm_text = std::fs::read_to_string(&args[2]).expect("read idm");
    let wats = de2_raw::parse_wats_2s(&wats_text);
    let idm = de2_raw::parse_idm_250ms(&idm_text);

    let mut w: Vec<(i64, &de2_raw::WatsRecord)> = wats
        .iter()
        .filter_map(|r| r.unix_ms().map(|t| (t, r)))
        .collect();
    w.sort_by_key(|(t, _)| *t);

    let mut samples = 0usize;
    let mut pairs = 0usize;
    let mut good = 0usize;
    let mut sum_dt = 0i128;
    let mut shown = 0usize;

    for rec in &idm {
        for s in &rec.samples {
            let Some(su) = de2_raw::yyddd_unix_ms(rec.date_yyddd, s.ut_ms) else {
                continue;
            };
            samples += 1;
            let idx = match w.binary_search_by_key(&su, |(t, _)| *t) {
                Ok(i) => i,
                Err(i) => i,
            };
            let mut best: Option<(i64, &de2_raw::WatsRecord)> = None;
            for i in [idx.saturating_sub(1), idx.min(w.len().saturating_sub(1))] {
                if let Some((t, r)) = w.get(i) {
                    let dt = (t - su).abs();
                    if best.map_or(true, |(b, _)| dt < b) {
                        best = Some((dt, r));
                    }
                }
            }
            if let Some((dt, r)) = best {
                if dt <= max_dt {
                    pairs += 1;
                    sum_dt += dt as i128;
                    let horiz = s.axis == IdmAxis::HorizontalZ;
                    let v = r.corrected_wind_ms();
                    if s.quality_good() && v.is_some() {
                        good += 1;
                    }
                    if shown < 10 {
                        println!(
                            "idm_ut={} v_idm={:.2} axis={} q={} | wats_dt={}ms v_geo_cor={:?}",
                            s.ut_ms,
                            s.velocity_ms,
                            if horiz { "z/horiz" } else { "y/vert" },
                            s.quality,
                            dt,
                            v
                        );
                        shown += 1;
                    }
                }
            }
        }
    }

    println!(
        "wats_records={} idm_records={} idm_samples={} pairs(<= {} ms)={} good_pairs={} mean|dt|={:.1} ms",
        wats.len(),
        idm.len(),
        samples,
        max_dt,
        pairs,
        good,
        if pairs > 0 {
            sum_dt as f64 / pairs as f64
        } else {
            0.0
        }
    );
}
