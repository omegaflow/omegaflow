use omegaflow::archivar::fetch_raw;
use omegaflow_measure::depthphase as dp;
use omegaflow_measure::depthphase::{
    CATALOG_URL, DEPTH_MATCH_GATE_KM, MAX_DIST_DEG, MAX_STATIONS, MIN_DEPTH_KM, MIN_DIST_DEG,
    MIN_MAG, P_WINDOW_AFTER_ORIGIN_S, REGION, SEARCH_START, SNR_GATE, STATION_URL,
};
use omegaflow_measure::stats::{mean, sample_sd};
use std::env;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let search_end = match dp::arg_value(&args, "--end") {
        Some(v) => v,
        None => match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => dp::unix_to_iso(d.as_secs_f64()),
            Err(_) => {
                eprintln!("depth-phase fleet: the system clock precedes the epoch — no end time, no fabricated zero");
                return;
            }
        },
    };
    let max_events = dp::arg_value(&args, "--max-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(8);
    let region = match dp::arg_value(&args, "--region") {
        Some(v) => {
            let parts: Vec<f64> = v.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            if parts.len() == 4 && parts.iter().all(|p| p.is_finite()) {
                [parts[0], parts[1], parts[2], parts[3]]
            } else {
                eprintln!(
                    "depth-phase fleet: --region needs four comma-separated finite numbers (lat0,lat1,lon0,lon1) — the default box stands"
                );
                REGION
            }
        }
        None => REGION,
    };
    let min_depth_km = match dp::arg_value(&args, "--mindepth") {
        Some(v) => match v.parse::<f64>() {
            Ok(d) if d.is_finite() && d >= 0.0 => d,
            _ => {
                eprintln!(
                    "depth-phase fleet: --mindepth is not a finite non-negative number — the default {MIN_DEPTH_KM} km stands"
                );
                MIN_DEPTH_KM
            }
        },
        None => MIN_DEPTH_KM,
    };
    let sp_gate = dp::arg_value(&args, "--sp-gate")
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|g| g.is_finite() && *g > 0.0);

    println!("=== depth-phase fleet — many events x stations, sigma and sqrt(N) ===");
    println!("selection rule (registered before the first fetch):");
    println!(
        "  depth >= {min_depth_km} km, magnitude >= {MIN_MAG}, box lat {}..{} lon {}..{}, GBCO witness per event",
        region[0], region[1], region[2], region[3]
    );
    println!(
        "  station band {MIN_DIST_DEG}..{MAX_DIST_DEG} deg, SNR gate >= {SNR_GATE}, up to {max_events} events (orderby magnitude)"
    );
    println!("error budget (before the run): 1 s pick scatter -> ~3.2 km per station; the per-event median");
    println!("  narrows with sqrt(n) stations, the fleet mean narrows with sqrt(N) events");
    println!("polarity witness: free-surface R_pp is negative across the steep band (befund");
    println!("  tiefenphasen-polaritaet) — a sign flip carries the source term, not the angle");
    println!(
        "Δ-restriction gate (named instrument, registered before the first fetch): a station whose"
    );
    println!(
        "  code-read pP-lag changes across 2δ above the measured smooth teleseismic gradient × a"
    );
    println!(
        "  factor is branch-unstable — the pP family folds there; the station is skipped by name,"
    );
    println!("  never fed into the inversion as an ambiguous pick (660-edge gate untouched)");
    println!("match gate: catalog depth uncertainty ~ +/- {DEPTH_MATCH_GATE_KM} km");
    println!();

    let cat_url = format!(
        "{CATALOG_URL}?format=geojson&starttime={SEARCH_START}&endtime={search_end}&minmagnitude={MIN_MAG}&mindepth={min_depth_km}&minlatitude={}&maxlatitude={}&minlongitude={}&maxlongitude={}&orderby=magnitude&limit=100",
        region[0], region[1], region[2], region[3]
    );
    let Some(body) = fetch_raw(&cat_url, None, &[], 86400) else {
        eprintln!("catalog carries no body — the channel stays unmeasured (0 honored)");
        return;
    };
    let events = dp::catalog_events(&body);
    if events.is_empty() {
        eprintln!("no registered deep event in the region box — nothing measured (0 honored)");
        return;
    }
    let measured = max_events.min(events.len());
    println!(
        "{} registered deep events in the box; measuring the top {measured}",
        events.len()
    );

    let mut offsets_with_clamp: Vec<f64> = Vec::new();
    let mut offsets_after_exclusion: Vec<f64> = Vec::new();
    let mut joint_offsets: Vec<f64> = Vec::new();
    let mut joint_pending: Vec<String> = Vec::new();
    let mut dual_offsets: Vec<f64> = Vec::new();
    let mut dual_pending: Vec<String> = Vec::new();
    let mut fleet_sp_corrs: Vec<f64> = Vec::new();
    let mut event_scatters: Vec<f64> = Vec::new();
    let mut pending_events: Vec<String> = Vec::new();
    let mut branch_skips_total = 0usize;

    for (idx, event) in events.iter().take(max_events).enumerate() {
        println!();
        println!(
            "event {}/{}: {} M{:.1} catalog {:.1} km, origin {}",
            idx + 1,
            measured,
            event.id,
            event.mag,
            event.depth_km,
            dp::unix_to_iso(event.t0)
        );
        let land = dp::gebco_elevation(event.lat, event.lon);
        match land {
            Some(e) => println!("  land witness: GEBCO surface elevation {e:.1} m"),
            None => {
                println!("  land witness: GEBCO returned no elevation (the box is the land rule)")
            }
        }

        let start = dp::unix_to_iso(event.t0);
        let end = dp::unix_to_iso(event.t0 + P_WINDOW_AFTER_ORIGIN_S);
        let st_url = format!(
            "{STATION_URL}?format=text&level=channel&latitude={:.4}&longitude={:.4}&minradius={MIN_DIST_DEG}&maxradius={MAX_DIST_DEG}&channel=BHZ&starttime={start}&endtime={end}&includerestricted=false",
            event.lat, event.lon
        );
        let Some(st_body) = fetch_raw(&st_url, None, &[], 86400) else {
            pending_events.push(format!("{} (station query void)", event.id));
            continue;
        };
        let mut stations = dp::parse_stations_text(&st_body);
        let mut seen = std::collections::HashSet::new();
        stations.retain(|s| seen.insert(format!("{}.{}", s.net, s.sta)));
        stations.sort_by(|a, b| {
            let da = dp::arc_deg(event.lat, event.lon, a.lat, a.lon);
            let db = dp::arc_deg(event.lat, event.lon, b.lat, b.lon);
            da.total_cmp(&db)
        });
        stations.truncate(MAX_STATIONS);

        let mut depths: Vec<f64> = Vec::new();
        let mut edge_clamped = 0usize;
        let mut saturated = 0usize;
        let mut skips: Vec<String> = Vec::new();
        let mut branch_unstable = 0usize;
        let mut first = true;
        let mut measures: Vec<dp::StationMeasure> = Vec::new();
        for st in &stations {
            if !first {
                thread::sleep(Duration::from_millis(1000));
            }
            first = false;
            let m = dp::measure_station(event, st, &start, &end);
            if m.branch_unstable {
                branch_unstable += 1;
            }
            match &m.skip {
                Some(reason) => skips.push(format!("{} ({reason})", m.key)),
                None => match m.inversion {
                    Some(dp::DepthInversion::Depth(h)) => depths.push(h),
                    Some(dp::DepthInversion::EdgeDiscontinuity) => edge_clamped += 1,
                    Some(dp::DepthInversion::SaturatedBound) => saturated += 1,
                    Some(dp::DepthInversion::Absent) => {
                        skips.push(format!("{} (inversion: no finite residual)", m.key))
                    }
                    None => skips.push(format!("{} (pP below the correlation gate)", m.key)),
                },
            }
            measures.push(m);
        }
        branch_skips_total += branch_unstable;

        let carried = depths.len() + edge_clamped + saturated;
        if carried == 0 {
            pending_events.push(format!(
                "{} (0 stations inverted a depth; {} skipped)",
                event.id,
                skips.len()
            ));
            continue;
        }
        println!(
            "  {edge_clamped} of {carried} stations edge-clamped at 660; {saturated} of {carried} saturated at 700"
        );
        if branch_unstable > 0 {
            println!(
                "  the Δ-gate skipped {branch_unstable} of {} branch-unstable stations (pP fold band)",
                stations.len()
            );
        }
        if let Some(smooth_grad) = dp::smooth_p_p_lag_gradient_s_per_deg(event.depth_km) {
            println!(
                "  Δ-gate reference at {:.0} km: smooth pP-lag gradient {smooth_grad:.2} s/deg, fold gate {:.2} s/deg (×{})",
                event.depth_km,
                smooth_grad * dp::FOLD_GATE_OVER_SMOOTH_FACTOR,
                dp::FOLD_GATE_OVER_SMOOTH_FACTOR
            );
        }
        let mut combined = depths.clone();
        combined.extend(std::iter::repeat(660.0).take(edge_clamped));
        combined.extend(std::iter::repeat(700.0).take(saturated));
        let with_clamp_median = dp::median(&mut combined);
        let offset_wc = with_clamp_median - event.depth_km;
        if depths.is_empty() {
            println!("  after-exclusion depth: absent (no station measured a depth — all {carried} carried a clamp at the model bounds, never 0)");
        } else {
            let median_depth = dp::median(&mut depths.clone());
            let offset_ae = median_depth - event.depth_km;
            let sd = sample_sd(&depths);
            let verdict = if offset_ae.abs() <= DEPTH_MATCH_GATE_KM {
                "inside the match gate"
            } else {
                "outside the match gate"
            };
            let depth_list = depths
                .iter()
                .map(|d| format!("{d:.0}"))
                .collect::<Vec<_>>()
                .join(", ");
            println!(
                "  {} of {carried} stations measured a depth [{depth_list}] km -> after-exclusion median {median_depth:.0} km, sd {} km, offset {:+.1} km ({verdict})",
                depths.len(),
                sd.map(|v| format!("{v:.1}")).unwrap_or("pending".into()),
                offset_ae
            );
            offsets_after_exclusion.push(offset_ae);
            if let Some(s) = sd {
                event_scatters.push(s);
            }
        }
        println!(
            "  with-clamp median {with_clamp_median:.0} km (edge-clamped counted as 660, saturated as 700), offset {:+.1} km",
            offset_wc
        );
        offsets_with_clamp.push(offset_wc);
        if !skips.is_empty() {
            println!("  skipped {}: {}", skips.len(), skips.join("; "));
        }
        let mut fit_deltas: Vec<f64> = Vec::new();
        let mut fit_lags: Vec<f64> = Vec::new();
        let mut fit_weights: Vec<f64> = Vec::new();
        let mut sigma_absent = 0usize;
        let mut ratios: Vec<String> = Vec::new();
        for m in &measures {
            if let Some(p) = &m.p_p {
                ratios.push(match p.runner_up_ratio {
                    Some(r) => format!("{r:.2}"),
                    None => "-".to_string(),
                });
            }
            if m.skip.is_some() {
                continue;
            }
            let Some(p) = &m.p_p else {
                continue;
            };
            if p.corr.abs() < dp::SECONDARY_CORR_MIN {
                continue;
            }
            match m.p_p_sigma_s {
                Some(sigma) if sigma.is_finite() && sigma > 0.0 => {
                    fit_deltas.push(m.delta_deg);
                    fit_lags.push(p.lag_s);
                    fit_weights.push(1.0 / (sigma * sigma));
                }
                _ => sigma_absent += 1,
            }
        }
        let weighted_n = fit_deltas.len();
        if weighted_n == 0 {
            println!(
                "  weighted joint fit: pending (no station carried a lag weight — sigma absent or below the correlation gate)"
            );
        } else {
            let joint = dp::invert_depth_weighted(&fit_deltas, &fit_lags, &fit_weights);
            let pick_stations = weighted_n + sigma_absent;
            let depth_txt = match joint {
                dp::DepthInversion::Depth(h) => format!("{h:.0} km"),
                dp::DepthInversion::EdgeDiscontinuity => {
                    "edge-clamped at the 660 km wall".to_string()
                }
                dp::DepthInversion::SaturatedBound => "saturated at the 700 km ceiling".to_string(),
                dp::DepthInversion::Absent => "absent".to_string(),
            };
            println!(
                "  weighted joint fit: {weighted_n} of {pick_stations} pick stations carry a weight ({sigma_absent} sigma-absent excluded: edge/truncated peak), depth {depth_txt}"
            );
            match joint {
                dp::DepthInversion::Depth(h) => joint_offsets.push(h - event.depth_km),
                dp::DepthInversion::EdgeDiscontinuity => {
                    joint_pending.push(format!("{} (joint clamped at the 660 wall)", event.id));
                }
                dp::DepthInversion::SaturatedBound => {
                    joint_pending
                        .push(format!("{} (joint saturated at the 700 ceiling)", event.id));
                }
                dp::DepthInversion::Absent => {
                    joint_pending.push(format!("{} (joint residual never finite)", event.id));
                }
            }
        }
        if !ratios.is_empty() {
            println!(
                "  runner-up ratio per station (measurement, no gate): {}",
                ratios.join(", ")
            );
        }
        let mut dual_deltas: Vec<f64> = Vec::new();
        let mut dual_lags: Vec<f64> = Vec::new();
        let mut dual_weights: Vec<f64> = Vec::new();
        let mut dual_phases: Vec<dp::DepthPhase> = Vec::new();
        let mut sp_legs = 0usize;
        let mut sp_below_gate = 0usize;
        let mut sp_sigma_absent = 0usize;
        let mut sp_corrs: Vec<f64> = Vec::new();
        for m in &measures {
            if let Some(sp) = &m.s_p {
                sp_corrs.push(sp.corr.abs());
            }
            if m.skip.is_some() {
                continue;
            }
            if let Some(p) = &m.p_p {
                if p.corr.abs() >= dp::SECONDARY_CORR_MIN {
                    match m.p_p_sigma_s {
                        Some(sigma) if sigma.is_finite() && sigma > 0.0 => {
                            dual_deltas.push(m.delta_deg);
                            dual_lags.push(p.lag_s);
                            dual_weights.push(1.0 / (sigma * sigma));
                            dual_phases.push(dp::DepthPhase::PP);
                        }
                        _ => {}
                    }
                }
            }
            if let Some(sp) = &m.s_p {
                match sp_gate {
                    Some(g) if sp.corr.abs() >= g => match m.s_p_sigma_s {
                        Some(sigma) if sigma.is_finite() && sigma > 0.0 => {
                            dual_deltas.push(m.delta_deg);
                            dual_lags.push(sp.lag_s);
                            dual_weights.push(1.0 / (sigma * sigma));
                            dual_phases.push(dp::DepthPhase::SP);
                            sp_legs += 1;
                        }
                        _ => sp_sigma_absent += 1,
                    },
                    Some(_) => sp_below_gate += 1,
                    None => {}
                }
            }
        }
        fleet_sp_corrs.extend(sp_corrs.iter().copied());
        if !sp_corrs.is_empty() {
            let mut sorted = sp_corrs.clone();
            sorted.sort_by(|a, b| a.total_cmp(b));
            let s_txt = sorted
                .iter()
                .map(|c| format!("{c:.2}"))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  sP |corr| per station (measurement, no gate): {s_txt}");
        }
        match sp_gate {
            None => println!(
                "  dual-phase fit: pending (the sP corr gate is unset — measure the sP |corr| distribution first, then set --sp-gate)"
            ),
            Some(_) if dual_deltas.is_empty() => println!(
                "  dual-phase fit: pending (no leg carried a weight — sigma absent or below the correlation gates)"
            ),
            Some(g) => {
                let dual =
                    dp::invert_depth_dual(&dual_deltas, &dual_lags, &dual_weights, &dual_phases);
                let depth_txt = match dual {
                    dp::DepthInversion::Depth(h) => format!("{h:.0} km"),
                    dp::DepthInversion::EdgeDiscontinuity => {
                        "edge-clamped at the 660 km wall".to_string()
                    }
                    dp::DepthInversion::SaturatedBound => {
                        "saturated at the 700 km ceiling".to_string()
                    }
                    dp::DepthInversion::Absent => "absent".to_string(),
                };
                println!(
                    "  dual-phase fit: {sp_legs} sP legs (gate {g:.2}, {sp_below_gate} below, {sp_sigma_absent} sigma-absent), depth {depth_txt}"
                );
                match dual {
                    dp::DepthInversion::Depth(h) => dual_offsets.push(h - event.depth_km),
                    dp::DepthInversion::EdgeDiscontinuity => {
                        dual_pending.push(format!("{} (dual clamped at the 660 wall)", event.id));
                    }
                    dp::DepthInversion::SaturatedBound => {
                        dual_pending
                            .push(format!("{} (dual saturated at the 700 ceiling)", event.id));
                    }
                    dp::DepthInversion::Absent => {
                        dual_pending.push(format!("{} (dual residual never finite)", event.id));
                    }
                }
            }
        }
    }

    println!();
    println!("=== fleet summary ===");
    if branch_skips_total > 0 {
        println!(
            "the Δ-gate skipped {branch_skips_total} stations fleet-wide as branch-unstable (pP fold band)"
        );
    } else {
        println!("the Δ-gate skipped no station (no measured station sat in a pP fold band)");
    }
    let n_wc = offsets_with_clamp.len();
    let n_ae = offsets_after_exclusion.len();
    if n_wc == 0 && n_ae == 0 {
        println!(
            "no event carried an inversion state — the fleet carries no statistic (0 honored)"
        );
    } else {
        if n_wc == 0 {
            println!("with-clamp mean offset: pending (no event carried a depth or clamp)");
        } else {
            let mean_wc = mean(&offsets_with_clamp).unwrap();
            let sd_wc = sample_sd(&offsets_with_clamp);
            let se_wc = sd_wc.map(|s| s / (n_wc as f64).sqrt());
            println!(
                "with-clamp mean offset {:+.1} km over {n_wc} events (edge-clamped counted as 660, saturated as 700); sd across events {} km, se = sd/sqrt(N) = {} km",
                mean_wc,
                sd_wc
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or("pending".into()),
                se_wc
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or("pending".into())
            );
        }
        if n_ae == 0 {
            println!("after-exclusion mean offset: pending (no event measured an unclamped depth)");
        } else {
            let mean_ae = mean(&offsets_after_exclusion).unwrap();
            let sd_ae = sample_sd(&offsets_after_exclusion);
            let se_ae = sd_ae.map(|s| s / (n_ae as f64).sqrt());
            let offsets_txt = offsets_after_exclusion
                .iter()
                .map(|o| format!("{:+.0}", o))
                .collect::<Vec<_>>()
                .join(", ");
            println!("after-exclusion mean offset {:+.1} km over {n_ae} events; per-event offsets [{offsets_txt}] km", mean_ae);
            println!(
                "after-exclusion sd across events {} km, se = sd/sqrt(N) = {} km",
                sd_ae.map(|v| format!("{v:.1}")).unwrap_or("pending".into()),
                se_ae.map(|v| format!("{v:.1}")).unwrap_or("pending".into())
            );
            let median_station_scatter = if event_scatters.is_empty() {
                None
            } else {
                Some(dp::median(&mut event_scatters.clone()))
            };
            match median_station_scatter {
                Some(s) => {
                    let comparison = match sd_ae {
                        Some(v) if v > s => "above the station scatter".to_string(),
                        Some(_) => "within the station scatter".to_string(),
                        None => "scatter relation pending (the event-to-event sd stays unread)"
                            .to_string(),
                    };
                    println!(
                        "typical within-event station scatter (median per-event sd): {s:.1} km — the after-exclusion event-to-event sd {} km sits {comparison}",
                        sd_ae.map(|v| format!("{v:.1}")).unwrap_or("pending".into()),
                    );
                }
                None => {
                    println!(
                        "within-event station scatter stays pending (no event carried 2+ depths)"
                    )
                }
            }
        }
    }
    let n_joint = joint_offsets.len();
    if n_joint == 0 {
        println!("weighted joint mean offset: pending (no event carried a joint depth)");
    } else {
        let mean_joint = mean(&joint_offsets).unwrap();
        let sd_joint = sample_sd(&joint_offsets);
        let se_joint = sd_joint.map(|s| s / (n_joint as f64).sqrt());
        println!(
            "weighted joint mean offset {:+.1} km over {n_joint} events; sd across events {} km, se = sd/sqrt(N) = {} km",
            mean_joint,
            sd_joint
                .map(|v| format!("{v:.1}"))
                .unwrap_or("pending".into()),
            se_joint
                .map(|v| format!("{v:.1}"))
                .unwrap_or("pending".into())
        );
    }
    if !joint_pending.is_empty() {
        println!(
            "weighted joint fit pending (named, not counted): {} — {}",
            joint_pending.len(),
            joint_pending.join("; ")
        );
    }
    let n_dual = dual_offsets.len();
    if n_dual == 0 {
        println!(
            "dual-phase mean offset: pending (no event carried a dual depth — the sP gate may be unset)"
        );
    } else {
        let mean_dual = mean(&dual_offsets).unwrap();
        let sd_dual = sample_sd(&dual_offsets);
        let se_dual = sd_dual.map(|s| s / (n_dual as f64).sqrt());
        println!(
            "dual-phase mean offset {:+.1} km over {n_dual} events; sd across events {} km, se = sd/sqrt(N) = {} km",
            mean_dual,
            sd_dual.map(|v| format!("{v:.1}")).unwrap_or("pending".into()),
            se_dual.map(|v| format!("{v:.1}")).unwrap_or("pending".into())
        );
    }
    if !dual_pending.is_empty() {
        println!(
            "dual-phase fit pending (named, not counted): {} — {}",
            dual_pending.len(),
            dual_pending.join("; ")
        );
    }
    if !fleet_sp_corrs.is_empty() {
        let mut sorted = fleet_sp_corrs.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let n = sorted.len();
        let median = sorted[n / 2];
        let p25 = sorted[n / 4];
        let p75 = sorted[(3 * n) / 4];
        println!(
            "sP |corr| distribution over the fleet (measurement): n={n}, min {:.2}, p25 {:.2}, median {:.2}, p75 {:.2}, max {:.2}",
            sorted[0],
            p25,
            median,
            p75,
            sorted[n - 1]
        );
    }
    if !pending_events.is_empty() {
        println!(
            "pending (not smoothed, not counted): {}",
            pending_events.join("; ")
        );
    }
}
