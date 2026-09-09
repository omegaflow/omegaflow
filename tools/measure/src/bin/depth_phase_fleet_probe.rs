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

    println!("=== depth-phase fleet — many events x stations, sigma and sqrt(N) ===");
    println!("selection rule (registered before the first fetch):");
    println!(
        "  depth >= {MIN_DEPTH_KM} km, magnitude >= {MIN_MAG}, land epicenter (Hindu Kush box lat {}..{} lon {}..{}), GBCO witness per event",
        REGION[0], REGION[1], REGION[2], REGION[3]
    );
    println!(
        "  station band {MIN_DIST_DEG}..{MAX_DIST_DEG} deg, SNR gate >= {SNR_GATE}, up to {max_events} events (orderby magnitude)"
    );
    println!("error budget (before the run): 1 s pick scatter -> ~3.2 km per station; the per-event median");
    println!("  narrows with sqrt(n) stations, the fleet mean narrows with sqrt(N) events");
    println!("polarity witness: free-surface R_pp is negative across the steep band (befund");
    println!("  tiefenphasen-polaritaet) — a sign flip carries the source term, not the angle");
    println!("match gate: catalog depth uncertainty ~ +/- {DEPTH_MATCH_GATE_KM} km");
    println!();

    let cat_url = format!(
        "{CATALOG_URL}?format=geojson&starttime={SEARCH_START}&endtime={search_end}&minmagnitude={MIN_MAG}&mindepth={MIN_DEPTH_KM}&minlatitude={}&maxlatitude={}&minlongitude={}&maxlongitude={}&orderby=magnitude&limit=100",
        REGION[0], REGION[1], REGION[2], REGION[3]
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

    let mut offsets: Vec<f64> = Vec::new();
    let mut event_scatters: Vec<f64> = Vec::new();
    let mut pending_events: Vec<String> = Vec::new();

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
        let mut skips: Vec<String> = Vec::new();
        let mut first = true;
        for st in &stations {
            if !first {
                thread::sleep(Duration::from_millis(1000));
            }
            first = false;
            let m = dp::measure_station(event, st, &start, &end);
            match m.skip {
                Some(reason) => skips.push(format!("{} ({reason})", m.key)),
                None => match m.depth_km {
                    Some(h) => depths.push(h),
                    None => skips.push(format!("{} (pP below the correlation gate)", m.key)),
                },
            }
        }

        let picked = stations.len() - skips.len();
        if depths.is_empty() {
            pending_events.push(format!(
                "{} (0 stations inverted a depth; {} skipped)",
                event.id,
                skips.len()
            ));
            continue;
        }
        let median_depth = dp::median(&mut depths.clone());
        let offset = median_depth - event.depth_km;
        let sd = sample_sd(&depths);
        let verdict = if offset.abs() <= DEPTH_MATCH_GATE_KM {
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
            "  {picked} stations passed the gate; depths [{depth_list}] km -> median {median_depth:.0} km, sd {} km, offset {:+.1} km ({verdict})",
            sd.map(|v| format!("{v:.1}")).unwrap_or("pending".into()),
            offset
        );
        if !skips.is_empty() {
            println!("  skipped {}: {}", skips.len(), skips.join("; "));
        }
        offsets.push(offset);
        if let Some(s) = sd {
            event_scatters.push(s);
        }
    }

    println!();
    println!("=== fleet summary ===");
    let n = offsets.len();
    if n == 0 {
        println!("no event inverted a depth — the fleet carries no statistic (0 honored)");
    } else {
        let mean_offset = mean(&offsets).unwrap();
        let sd_offset = sample_sd(&offsets);
        let se = sd_offset.map(|s| s / (n as f64).sqrt());
        let offsets_txt = offsets
            .iter()
            .map(|o| format!("{:+.0}", o))
            .collect::<Vec<_>>()
            .join(", ");
        println!("N = {n} events carried an inverted depth");
        println!("per-event offsets (median - catalog): [{offsets_txt}] km");
        println!(
            "mean offset {:+.1} km, sd across events {} km, se = sd/sqrt(N) = {} km",
            mean_offset,
            sd_offset
                .map(|v| format!("{v:.1}"))
                .unwrap_or("pending".into()),
            se.map(|v| format!("{v:.1}")).unwrap_or("pending".into())
        );
        let median_station_scatter = if event_scatters.is_empty() {
            None
        } else {
            Some(dp::median(&mut event_scatters.clone()))
        };
        match median_station_scatter {
            Some(s) => {
                let comparison = match sd_offset {
                    Some(v) if v > s => "above the station scatter".to_string(),
                    Some(_) => "within the station scatter".to_string(),
                    None => {
                        "scatter relation pending (the event-to-event sd stays unread)".to_string()
                    }
                };
                println!(
                    "typical within-event station scatter (median per-event sd): {s:.1} km — the event-to-event sd {} km sits {comparison}",
                    sd_offset.map(|v| format!("{v:.1}")).unwrap_or("pending".into()),
                );
            }
            None => {
                println!("within-event station scatter stays pending (no event carried 2+ depths)")
            }
        }
    }
    if !pending_events.is_empty() {
        println!(
            "pending (not smoothed, not counted): {}",
            pending_events.join("; ")
        );
    }
}
