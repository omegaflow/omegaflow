use omegaflow::ak135::{free_surface_pp, p_p_rayparam};
use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::ndk;
use omegaflow_measure::depthphase as dp;
use omegaflow_measure::depthphase::{
    CATALOG_URL, DEPTH_MATCH_GATE_KM, MAX_DIST_DEG, MAX_STATIONS, MIN_DEPTH_KM, MIN_DIST_DEG,
    MIN_MAG, P_WINDOW_AFTER_ORIGIN_S, REGION, SEARCH_START, SNR_GATE, STATION_URL,
};
use omegaflow_measure::iasp91;
use omegaflow_measure::mww;
use omegaflow_measure::stats::{mean, sample_sd};
use std::env;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const GCMT_NDK_URL: &str =
    "https://www.ldeo.columbia.edu/~gcmt/projects/CMT/catalog/jan76_dec25.ndk";
const KM_PER_DEG: f64 = 111.195;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let search_end = match dp::arg_value(&args, "--end") {
        Some(v) => v,
        None => match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => dp::unix_to_iso(d.as_secs_f64()),
            Err(_) => {
                eprintln!(
                    "depth-phase fleet: the system clock precedes the epoch — no end time, no fabricated zero"
                );
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
    let kalibrier = args.iter().any(|a| a == "--kalibrier");
    let mww = args.iter().any(|a| a == "--mww");

    println!("=== depth-phase fleet — many events x stations, sigma and sqrt(N) ===");
    println!("selection rule (registered before the first fetch):");
    println!(
        "  depth >= {min_depth_km} km, magnitude >= {MIN_MAG}, box lat {}..{} lon {}..{}, GBCO witness per event",
        region[0], region[1], region[2], region[3]
    );
    println!(
        "  station band {MIN_DIST_DEG}..{MAX_DIST_DEG} deg, SNR gate >= {SNR_GATE}, up to {max_events} events (orderby magnitude)"
    );
    println!(
        "uncertainty budget (before the run): 1 s pick scatter -> ~3.2 km per station; the per-event median"
    );
    println!("  narrows with sqrt(n) stations, the fleet mean narrows with sqrt(N) events");
    println!(
        "polarity witness: free-surface R_pp is negative across the steep band (src/archivar/ak135.rs free_surface_pp) — a sign flip carries the source term, not the angle"
    );
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
    let Some(body) = fetch_raw(&cat_url, None, &[]) else {
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
    let ndk_events = if kalibrier {
        match ndk::fetch_events(GCMT_NDK_URL, 3600) {
            Some(cents) => {
                println!(
                    "kalibrier-gate: {} GCMT centroids loaded from the NDK body — the source term is wired",
                    cents.len()
                );
                Some(cents)
            }
            None => {
                println!(
                    "kalibrier-gate: no GCMT NDK body — the per-station source prediction stays absent (never fabricated)"
                );
                None
            }
        }
    } else {
        None
    };

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

        let mww_record = if mww {
            let url = format!(
                "{CATALOG_URL}?eventid={}&format=quakeml&magnitudetype=mww",
                event.id
            );
            fetch_raw(&url, None, &[]).and_then(|xml| mww::parse_quakeml(&xml))
        } else {
            None
        };
        let anchor = if mww {
            anchor_register(event, mww_record.as_ref())
        } else {
            event.clone()
        };

        let start = dp::unix_to_iso(anchor.t0);
        let end = dp::unix_to_iso(anchor.t0 + P_WINDOW_AFTER_ORIGIN_S);
        let st_url = format!(
            "{STATION_URL}?format=text&level=channel&latitude={:.4}&longitude={:.4}&minradius={MIN_DIST_DEG}&maxradius={MAX_DIST_DEG}&channel=BHZ&starttime={start}&endtime={end}&includerestricted=false",
            anchor.lat, anchor.lon
        );
        let Some(st_body) = fetch_raw(&st_url, None, &[]) else {
            pending_events.push(format!("{} (station query void)", event.id));
            continue;
        };
        let mut stations = dp::parse_stations_text(&st_body);
        let mut seen = std::collections::HashSet::new();
        stations.retain(|s| seen.insert(format!("{}.{}", s.net, s.sta)));
        stations = select_spread_stations(
            &stations,
            anchor.lat,
            anchor.lon,
            MIN_DIST_DEG,
            MAX_DIST_DEG,
            MAX_STATIONS,
        );

        let mut depths: Vec<f64> = Vec::new();
        let mut edge_clamped = 0usize;
        let mut saturated = 0usize;
        let mut skips: Vec<String> = Vec::new();
        let mut branch_unstable = 0usize;
        let mut fold = 0usize;
        let mut first = true;
        let mut measures: Vec<dp::StationMeasure> = Vec::new();
        for st in &stations {
            if !first {
                thread::sleep(Duration::from_millis(1000));
            }
            first = false;
            let m = dp::measure_station(&anchor, st, &start, &end, None);
            match m.branch {
                dp::PPBranch::BranchUnstable => branch_unstable += 1,
                dp::PPBranch::Fold => fold += 1,
                dp::PPBranch::Clear => {}
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
        branch_skips_total += branch_unstable + fold;
        if kalibrier {
            emit_kalibrier(event, &stations, &measures, ndk_events.as_deref());
        }
        if mww {
            emit_mww(&anchor, &stations, &measures, mww_record.as_ref());
        }

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
        if branch_unstable > 0 || fold > 0 {
            println!(
                "  the Δ-gate skipped {branch_unstable} branch-unstable and {fold} fold stations of {} (pP fold band)",
                stations.len()
            );
        }
        if let Some(smooth_grad) = dp::smooth_p_p_lag_gradient_s_per_deg(anchor.depth_km) {
            println!(
                "  Δ-gate reference at {:.0} km: smooth pP-lag gradient {smooth_grad:.2} s/deg, fold gate {:.2} s/deg (×{})",
                anchor.depth_km,
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
            println!(
                "  after-exclusion depth: absent (no station measured a depth — all {carried} carried a clamp at the model bounds, never 0)"
            );
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
            "the Δ-gate skipped {branch_skips_total} stations fleet-wide as branch-unstable or fold (pP fold band)"
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
                sd_wc.map(|v| format!("{v:.1}")).unwrap_or("pending".into()),
                se_wc.map(|v| format!("{v:.1}")).unwrap_or("pending".into())
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
            println!(
                "after-exclusion mean offset {:+.1} km over {n_ae} events; per-event offsets [{offsets_txt}] km",
                mean_ae
            );
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
            sd_dual
                .map(|v| format!("{v:.1}"))
                .unwrap_or("pending".into()),
            se_dual
                .map(|v| format!("{v:.1}"))
                .unwrap_or("pending".into())
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

fn anchor_register(catalog: &dp::Event, record: Option<&mww::MwwRecord>) -> dp::Event {
    println!("  anchor register (which place and clock center the pP windows):");
    match record {
        None => {
            println!(
                "    anchor: catalog hypocentral lat {:.4}, lon {:.4}, depth {:.1} km, origin {} — no USGS mww record stands; the source term stays absent, never an NDK substitution",
                catalog.lat,
                catalog.lon,
                catalog.depth_km,
                dp::unix_to_iso(catalog.t0)
            );
            catalog.clone()
        }
        Some(r) => {
            let c = &r.centroid;
            match (c.lat, c.lon, c.depth_m, c.time_unix) {
                (Some(lat), Some(lon), Some(depth_m), Some(time_unix)) => {
                    let depth_km = depth_m / 1000.0;
                    let offset_deg = dp::arc_deg(catalog.lat, catalog.lon, lat, lon);
                    println!(
                        "    anchor: USGS mww centroid lat {lat:.4}, lon {lon:.4}, depth {depth_km:.1} km, time {} — all four fields present; catalog offset {offset_deg:.3} deg ({:.1} km), depth difference {:+.1} km; the pP windows center on the centroid clock",
                        dp::unix_to_iso(time_unix),
                        offset_deg * KM_PER_DEG,
                        depth_km - catalog.depth_km
                    );
                    dp::Event {
                        id: catalog.id.clone(),
                        t0: time_unix,
                        mag: catalog.mag,
                        lat,
                        lon,
                        depth_km,
                    }
                }
                _ => {
                    let mut missing: Vec<&str> = Vec::new();
                    if c.lat.is_none() {
                        missing.push("lat");
                    }
                    if c.lon.is_none() {
                        missing.push("lon");
                    }
                    if c.depth_m.is_none() {
                        missing.push("depth");
                    }
                    if c.time_unix.is_none() {
                        missing.push("time");
                    }
                    println!(
                        "    anchor: catalog hypocentral lat {:.4}, lon {:.4}, depth {:.1} km, origin {} — the mww record stands but {} absent; a hybrid anchor names no measured location, so the hypocentral anchor stands",
                        catalog.lat,
                        catalog.lon,
                        catalog.depth_km,
                        dp::unix_to_iso(catalog.t0),
                        missing.join(" and ")
                    );
                    catalog.clone()
                }
            }
        }
    }
}

fn emit_mww(
    anchor: &dp::Event,
    stations: &[dp::Station],
    measures: &[dp::StationMeasure],
    record: Option<&mww::MwwRecord>,
) {
    println!("  mww-gate — the USGS moment-tensor source term at the measured station azimuths:");
    let Some(rec) = record else {
        println!(
            "    no USGS mww record stands — the source term stays absent, never an NDK substitution"
        );
        return;
    };
    let Some(tensor) = moment_tensor_six(&rec.moment_tensor) else {
        println!(
            "    the mww moment tensor carries an absent component — the source term stays absent (0 honored)"
        );
        return;
    };
    println!(
        "    USGS mww anchor depth {:.1} km, magnitude {} {}, doubleCouple {}",
        anchor.depth_km,
        rec.magnitude
            .map(|m| format!("{m:.3}"))
            .unwrap_or("absent".into()),
        rec.magnitude_type.as_deref().unwrap_or("absent"),
        rec.moment_tensor
            .double_couple
            .map(|d| format!("{d:.4}"))
            .unwrap_or("absent".into())
    );
    println!(
        "    nodal planes (witness, not the source term): np1 strike/dip/rake {}/{}/{}, np2 {}/{}/{}",
        human_opt(rec.np1.strike),
        human_opt(rec.np1.dip),
        human_opt(rec.np1.rake),
        human_opt(rec.np2.strike),
        human_opt(rec.np2.dip),
        human_opt(rec.np2.rake)
    );
    let Some(reference) = measures.iter().find(|ms| ms.skip.is_none()) else {
        println!("    no station cleared the gates — the azimuth register stays empty (0 honored)");
        return;
    };
    let Some(i_ref) = iasp91::takeoff_angle_deg(reference.delta_deg, anchor.depth_km, true) else {
        println!(
            "    the upgoing take-off at the reference station stays unread — the radiation pattern stays absent"
        );
        return;
    };
    let (rp_min, rp_max, nodal) = upgoing_radiation(&tensor, i_ref);
    let nodal_txt = nodal
        .iter()
        .map(|n| format!("{n:.1}"))
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "    upgoing P radiation R_P spans [{rp_min:+.3e}, {rp_max:+.3e}] N m at the reference delta {:.1} deg; nodal azimuths {}",
        reference.delta_deg,
        if nodal.is_empty() {
            "none in the 1-deg sweep".to_string()
        } else {
            nodal_txt
        }
    );
    let mut agree = 0usize;
    let mut oppose = 0usize;
    let mut pending = 0usize;
    for (st, meas) in stations.iter().zip(measures.iter()) {
        let Some(az) = initial_azimuth_deg(anchor.lat, anchor.lon, st.lat, st.lon) else {
            println!(
                "    {}.{} azimuth unread — absent, never a fabricated bearing",
                st.net, st.sta
            );
            continue;
        };
        let source = iasp91::takeoff_angle_deg(meas.delta_deg, anchor.depth_km, true)
            .map(|i_up| ndk::rp(&tensor, &ndk::ray_direction(i_up, az, true)));
        let r_pp = p_p_rayparam(meas.delta_deg, anchor.depth_km).and_then(free_surface_pp);
        let predicted = match (source, r_pp) {
            (Some(s), Some(r)) => Some(recorded_sign(s, r)),
            _ => None,
        };
        let measured = meas
            .p_p
            .as_ref()
            .map(|p| if p.inverted { -1.0 } else { 1.0 });
        let verdict = match (predicted, measured) {
            (Some(a), Some(b)) => {
                if a * b > 0.0 {
                    agree += 1;
                    "agrees"
                } else {
                    oppose += 1;
                    "opposes"
                }
            }
            _ => {
                pending += 1;
                "pending"
            }
        };
        let source_txt = source
            .map(|v| format!("{v:+.3e}"))
            .unwrap_or("absent".into());
        let rpp_txt = r_pp.map(|v| format!("{v:+.3}")).unwrap_or("absent".into());
        let pred_txt = predicted.map(sign_mark).unwrap_or("absent".into());
        let meas_txt = measured.map(sign_mark).unwrap_or("absent".into());
        println!(
            "    {}.{} delta {:.1} deg az {az:.1} deg R_P_up={source_txt} N m R_pp={rpp_txt} predicted pP {pred_txt} measured pP {meas_txt} — {verdict}",
            st.net, st.sta, meas.delta_deg
        );
    }
    println!(
        "    mww-gate: {agree} stations agree, {oppose} oppose, {pending} pending (measured pP sign vs the raw USGS moment-tensor source term times the free-surface sign)"
    );
}

fn moment_tensor_six(mt: &mww::MomentTensor) -> Option<[f64; 6]> {
    Some([mt.mrr?, mt.mtt?, mt.mpp?, mt.mrt?, mt.mrp?, mt.mtp?])
}

fn human_opt(v: Option<f64>) -> String {
    v.map(|x| format!("{x:.2}")).unwrap_or("absent".into())
}

fn initial_azimuth_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> Option<f64> {
    if !(lat1.is_finite() && lon1.is_finite() && lat2.is_finite() && lon2.is_finite()) {
        return None;
    }
    let p1 = lat1.to_radians();
    let p2 = lat2.to_radians();
    let dl = (lon2 - lon1).to_radians();
    let y = dl.sin() * p2.cos();
    let x = p1.cos() * p2.sin() - p1.sin() * p2.cos() * dl.cos();
    if !(y.is_finite() && x.is_finite()) {
        return None;
    }
    Some(y.atan2(x).to_degrees().rem_euclid(360.0))
}

fn upgoing_radiation(m: &[f64; 6], takeoff_up_deg: f64) -> (f64, f64, Vec<f64>) {
    let mut nodal = Vec::new();
    let mut prev: Option<f64> = None;
    let mut prev_az = 0.0f64;
    let mut rp_min = f64::INFINITY;
    let mut rp_max = f64::NEG_INFINITY;
    for az in 0..360 {
        let a = az as f64;
        let r = ndk::ray_direction(takeoff_up_deg, a, true);
        let v = ndk::rp(m, &r);
        rp_min = rp_min.min(v);
        rp_max = rp_max.max(v);
        if let Some(p) = prev {
            if (p < 0.0 && v > 0.0) || (p > 0.0 && v < 0.0) {
                let frac = p / (p - v);
                nodal.push(prev_az + frac);
            }
        }
        prev = Some(v);
        prev_az = a;
    }
    (rp_min, rp_max, nodal)
}

fn recorded_sign(source_rp: f64, free_surface_rp: f64) -> f64 {
    source_rp.signum() * free_surface_rp.signum()
}

fn sign_mark(v: f64) -> String {
    if v < 0.0 {
        "-".to_string()
    } else {
        "+".to_string()
    }
}

fn emit_kalibrier(
    event: &dp::Event,
    stations: &[dp::Station],
    measures: &[dp::StationMeasure],
    ndk_events: Option<&[ndk::NdkEvent]>,
) {
    println!("  kalibrier-gate — the NDK source term at the measured station azimuths:");
    let Some(cents) = ndk_events else {
        println!(
            "    the GCMT NDK body stays absent — the per-station source polarity is pending, never fabricated"
        );
        return;
    };
    let date = dp::unix_to_iso(event.t0);
    let matched = cents
        .iter()
        .filter(|e| date.starts_with(&format!("{:04}-{:02}-{:02}", e.year, e.month, e.day)))
        .filter(|e| (e.hyp_lat - event.lat).abs() < 2.0 && (e.hyp_lon - event.lon).abs() < 2.0)
        .min_by(|a, b| {
            let da = (a.hyp_lat - event.lat).powi(2) + (a.hyp_lon - event.lon).powi(2);
            let db = (b.hyp_lat - event.lat).powi(2) + (b.hyp_lon - event.lon).powi(2);
            da.total_cmp(&db)
        });
    let Some(ev) = matched else {
        println!(
            "    no GCMT centroid in the event window — the source term stays absent (0 honored)"
        );
        return;
    };
    let m = ndk::dc_moment_tensor(ev.strike, ev.dip, ev.rake);
    println!(
        "    GCMT centroid {} ({:04}/{:02}/{:02}) strike/dip/rake {:.0}/{:.0}/{:.0}, centroid depth {:.1} km",
        ev.name, ev.year, ev.month, ev.day, ev.strike, ev.dip, ev.rake, ev.centroid_depth_km
    );
    let Some(reference) = measures.iter().find(|ms| ms.skip.is_none()) else {
        println!("    no station cleared the gates — the azimuth register stays empty (0 honored)");
        return;
    };
    let Some(i_ref) = iasp91::takeoff_angle_deg(reference.delta_deg, ev.centroid_depth_km, true)
    else {
        println!(
            "    the upgoing take-off at the reference station stays unread — the radiation pattern stays absent"
        );
        return;
    };
    let (rp_min, rp_max, nodal) = upgoing_radiation(&m, i_ref);
    let nodal_txt = nodal
        .iter()
        .map(|n| format!("{n:.1}"))
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "    upgoing P radiation R_P spans [{rp_min:+.2}, {rp_max:+.2}] (unit M) at the reference delta {:.1} deg; nodal azimuths {}",
        reference.delta_deg,
        if nodal.is_empty() {
            "none in the 1-deg sweep".to_string()
        } else {
            nodal_txt
        }
    );
    let mut agree = 0usize;
    let mut oppose = 0usize;
    let mut pending = 0usize;
    for (st, meas) in stations.iter().zip(measures.iter()) {
        let Some(az) = initial_azimuth_deg(event.lat, event.lon, st.lat, st.lon) else {
            println!(
                "    {}.{} azimuth unread — absent, never a fabricated bearing",
                st.net, st.sta
            );
            continue;
        };
        let source = iasp91::takeoff_angle_deg(meas.delta_deg, ev.centroid_depth_km, true)
            .map(|i_up| ndk::rp(&m, &ndk::ray_direction(i_up, az, true)));
        let r_pp = p_p_rayparam(meas.delta_deg, ev.centroid_depth_km).and_then(free_surface_pp);
        let predicted = match (source, r_pp) {
            (Some(s), Some(r)) => Some(recorded_sign(s, r)),
            _ => None,
        };
        let measured = meas
            .p_p
            .as_ref()
            .map(|p| if p.inverted { -1.0 } else { 1.0 });
        let verdict = match (predicted, measured) {
            (Some(a), Some(b)) => {
                if a * b > 0.0 {
                    agree += 1;
                    "agrees"
                } else {
                    oppose += 1;
                    "opposes"
                }
            }
            _ => {
                pending += 1;
                "pending"
            }
        };
        let source_txt = source
            .map(|v| format!("{v:+.3}"))
            .unwrap_or("absent".into());
        let rpp_txt = r_pp.map(|v| format!("{v:+.3}")).unwrap_or("absent".into());
        let pred_txt = predicted.map(sign_mark).unwrap_or("absent".into());
        let meas_txt = measured.map(sign_mark).unwrap_or("absent".into());
        println!(
            "    {}.{} delta {:.1} deg az {az:.1} deg R_P_up={source_txt} R_pp={rpp_txt} predicted pP {pred_txt} measured pP {meas_txt} — {verdict}",
            st.net, st.sta, meas.delta_deg
        );
    }
    println!(
        "    kalibrier-gate: {agree} stations agree, {oppose} oppose, {pending} pending (measured pP sign vs the NDK source term times the free-surface sign)"
    );
}

fn select_spread_stations(
    stations: &[dp::Station],
    anchor_lat: f64,
    anchor_lon: f64,
    min_dist: f64,
    max_dist: f64,
    cap: usize,
) -> Vec<dp::Station> {
    let band = max_dist - min_dist;
    let mut farthest: Vec<Option<dp::Station>> = vec![None; cap];
    for s in stations {
        let d = dp::arc_deg(anchor_lat, anchor_lon, s.lat, s.lon);
        if !d.is_finite() || d < min_dist || d > max_dist {
            continue;
        }
        let bin = (((d - min_dist) / band) * (cap as f64)).floor() as usize;
        let bin = bin.min(cap - 1);
        match &farthest[bin] {
            None => farthest[bin] = Some(s.clone()),
            Some(cur) => {
                let cur_d = dp::arc_deg(anchor_lat, anchor_lon, cur.lat, cur.lon);
                if d > cur_d {
                    farthest[bin] = Some(s.clone());
                }
            }
        }
    }
    farthest.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_azimuth_reads_the_cardinal_bearings() {
        assert!((initial_azimuth_deg(0.0, 0.0, 1.0, 0.0).unwrap() - 0.0).abs() < 1e-9);
        assert!((initial_azimuth_deg(0.0, 0.0, 0.0, 1.0).unwrap() - 90.0).abs() < 1e-9);
        assert!((initial_azimuth_deg(0.0, 0.0, -1.0, 0.0).unwrap() - 180.0).abs() < 1e-9);
        assert!((initial_azimuth_deg(0.0, 0.0, 0.0, -1.0).unwrap() - 270.0).abs() < 1e-9);
    }

    #[test]
    fn initial_azimuth_is_absent_for_a_non_finite_coordinate() {
        assert!(initial_azimuth_deg(f64::NAN, 0.0, 1.0, 0.0).is_none());
        assert!(initial_azimuth_deg(0.0, 0.0, f64::INFINITY, 0.0).is_none());
    }

    #[test]
    fn a_vertical_upgoing_ray_reads_the_thrust_compressional() {
        let m = ndk::dc_moment_tensor(0.0, 45.0, 90.0);
        let (rp_min, rp_max, nodal) = upgoing_radiation(&m, 0.0);
        assert!((rp_min - 1.0).abs() < 1e-12);
        assert!((rp_max - 1.0).abs() < 1e-12);
        assert!(
            nodal.is_empty(),
            "a constant pattern carries no nodal azimuth"
        );
    }

    #[test]
    fn the_recorded_sign_is_the_source_sign_times_the_free_surface_sign() {
        assert_eq!(recorded_sign(1.0, -1.0), -1.0);
        assert_eq!(recorded_sign(-1.0, -1.0), 1.0);
        assert_eq!(recorded_sign(1.0, 1.0), 1.0);
        assert_eq!(recorded_sign(-1.0, 1.0), -1.0);
    }

    #[test]
    fn the_six_tensor_components_flow_in_the_rp_order() {
        let mt = mww::MomentTensor {
            mrr: Some(1.0),
            mtt: Some(2.0),
            mpp: Some(3.0),
            mrt: Some(4.0),
            mrp: Some(5.0),
            mtp: Some(6.0),
            scalar_moment_nm: None,
            double_couple: None,
        };
        assert_eq!(moment_tensor_six(&mt), Some([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let absent = mww::MomentTensor {
            mrr: Some(1.0),
            mtt: None,
            mpp: Some(3.0),
            mrt: Some(4.0),
            mrp: Some(5.0),
            mtp: Some(6.0),
            scalar_moment_nm: None,
            double_couple: None,
        };
        assert!(moment_tensor_six(&absent).is_none());
    }

    #[test]
    fn spread_selection_keeps_the_outermost_station_of_each_bin() {
        let mk = |lat: f64| dp::Station {
            net: "N".to_string(),
            sta: format!("S{lat}"),
            lat,
            lon: 0.0,
        };
        let stations = vec![mk(40.0), mk(45.0), mk(60.0), mk(80.0), mk(85.0)];
        let picked = select_spread_stations(&stations, 0.0, 0.0, 30.0, 90.0, 3);
        let lats: Vec<f64> = picked.iter().map(|s| s.lat).collect();
        assert!(
            lats.contains(&85.0),
            "the outer bin keeps its outermost station"
        );
        assert!(
            lats.contains(&45.0),
            "the inner bin keeps its outermost station"
        );
        assert!(
            !lats.contains(&40.0),
            "the inner bin drops its inner station"
        );
        assert!(picked.len() <= 3);
    }

    #[test]
    fn spread_selection_never_exceeds_the_cap() {
        let mut stations = Vec::new();
        for lat in (30..90).step_by(2) {
            stations.push(dp::Station {
                net: "N".to_string(),
                sta: format!("S{lat}"),
                lat: lat as f64,
                lon: 0.0,
            });
        }
        let picked = select_spread_stations(&stations, 0.0, 0.0, 30.0, 90.0, 12);
        assert_eq!(picked.len(), 12);
    }
}
