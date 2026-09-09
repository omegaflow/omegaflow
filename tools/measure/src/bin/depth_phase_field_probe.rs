use omegaflow::ak135::{p_p_travel, p_travel_depth};
use omegaflow::archivar::fetch_raw;
use omegaflow_measure::depthphase as dp;
use omegaflow_measure::depthphase::{
    CATALOG_URL, DEPTH_MATCH_GATE_KM, MAX_DIST_DEG, MAX_STATIONS, MIN_DEPTH_KM, MIN_DIST_DEG,
    MIN_MAG, P_WAVELET_S, P_WINDOW_AFTER_ORIGIN_S, REGION, SEARCH_START, SECONDARY_CORR_MIN,
    SNR_GATE, STATION_URL,
};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let search_end = match dp::arg_value(&args, "--end") {
        Some(v) => v,
        None => match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => dp::unix_to_iso(d.as_secs_f64()),
            Err(_) => {
                eprintln!("depth-phase field probe: the system clock precedes the epoch — no end time, no fabricated zero");
                return;
            }
        },
    };

    let lag_5km = p_p_travel(45.0, 15.0).and_then(|pp| p_travel_depth(45.0, 10.0).map(|p| pp - p));
    let depth_precision_km = lag_5km.map(|l| 5.0 / l * 2.0);

    println!(
        "=== depth-phase field pilot — the pP/sP lag reads the depth from a real deep event ==="
    );
    println!("selection rule (registered before the first fetch):");
    println!(
        "  depth >= {MIN_DEPTH_KM} km, magnitude >= {MIN_MAG}, land epicenter (Hindu Kush box lat {}..{} lon {}..{}),",
        REGION[0], REGION[1], REGION[2], REGION[3]
    );
    println!(
        "  station distance band {MIN_DIST_DEG}..{MAX_DIST_DEG} deg, SNR gate >= {SNR_GATE} at the P onset BEFORE the pP window"
    );
    println!();
    match (lag_5km, depth_precision_km) {
        (Some(l), Some(d)) => println!(
            "error budget (before the run): the lag carries the onset pick (~2 s scatter); {l:.2} s of lag per 5 km -> expected depth precision ~ +/- {d:.1} km at a single event"
        ),
        _ => println!(
            "error budget: the ak135 45-deg depth leg stays uncomputed — no fabricated precision"
        ),
    }
    println!(
        "match gate: catalog depth uncertainty ~ +/- {DEPTH_MATCH_GATE_KM} km; outside the gate = two readings (our scatter or the catalog)"
    );
    println!(
        "polarity witness: at the free surface pP arrives inverted; a negative correlation peak is the pP signature, a positive-only peak is suspicious (coda)"
    );
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
    let event = &events[0];
    println!(
        "event {}: M{:.1} at {:.4} N {:.4} E, catalog depth {:.1} km, origin {}",
        event.id,
        event.mag,
        event.lat,
        event.lon,
        event.depth_km,
        dp::unix_to_iso(event.t0)
    );
    match dp::gebco_elevation(event.lat, event.lon) {
        Some(e) => println!("  land witness: GEBCO surface elevation {e:.1} m"),
        None => println!("  land witness: GEBCO returned no elevation (the box is the land rule)"),
    }
    println!();

    let start = dp::unix_to_iso(event.t0);
    let end = dp::unix_to_iso(event.t0 + P_WINDOW_AFTER_ORIGIN_S);
    let st_url = format!(
        "{STATION_URL}?format=text&level=channel&latitude={:.4}&longitude={:.4}&minradius={MIN_DIST_DEG}&maxradius={MAX_DIST_DEG}&channel=BHZ&starttime={start}&endtime={end}&includerestricted=false",
        event.lat, event.lon
    );
    let Some(st_body) = fetch_raw(&st_url, None, &[], 86400) else {
        eprintln!("station query carries no body — the channel stays unmeasured (0 honored)");
        return;
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
    println!(
        "{} BHZ stations in the {MIN_DIST_DEG}..{MAX_DIST_DEG} deg band",
        stations.len()
    );

    let mut picked = 0usize;
    let mut depths: Vec<f64> = Vec::new();
    let mut rows: Vec<String> = Vec::new();
    for st in &stations {
        let key = format!("{}.{}", st.net, st.sta);
        let delta = dp::arc_deg(event.lat, event.lon, st.lat, st.lon);
        let Some((samples, rate)) = dp::fetch_station_body(st, &start, &end) else {
            eprintln!("{key} carries no decodable record — skipped");
            continue;
        };
        let Some(t_p) = dp::p_onset(&samples, rate) else {
            eprintln!("{key} carries no P pick — skipped");
            continue;
        };
        let bp = dp::bandpass(&samples, rate);
        let i_p = dp::onset_index(&samples, rate, t_p);
        let Some(snr) = dp::onset_snr(&bp, rate, i_p) else {
            eprintln!("{key} carries no noise floor — skipped");
            continue;
        };
        if snr < SNR_GATE {
            eprintln!(
                "{key} P-onset SNR {snr:.1} below the {SNR_GATE} gate — skipped before the pP window"
            );
            continue;
        }
        let nw = (P_WAVELET_S * rate).round() as usize;
        if nw == 0 {
            eprintln!("{key} carries a degenerate rate — skipped");
            continue;
        }
        let p_p_lag_pred = match dp::p_p_lag(delta, event.depth_km) {
            Some(l) => l,
            None => {
                eprintln!("{key} delta {delta:.1} carries no ak135 pP prediction — skipped");
                continue;
            }
        };
        let pp = dp::correlate_window(&bp, i_p, nw, rate, p_p_lag_pred);
        let pp_txt = match &pp {
            Some(p) => {
                let word = if p.corr.abs() >= SECONDARY_CORR_MIN {
                    "found"
                } else {
                    "weak"
                };
                format!(
                    "lag {:.1} s (pred {:.1} s) corr {:+.2} {} edge={}",
                    p.lag_s, p_p_lag_pred, p.corr, word, p.at_edge
                )
            }
            None => "window out of trace".to_string(),
        };
        let sp_txt = match dp::s_p_lag(delta, event.depth_km)
            .and_then(|sp_lag| dp::correlate_window(&bp, i_p, nw, rate, sp_lag))
        {
            Some(p) if p.corr.abs() >= SECONDARY_CORR_MIN => {
                format!(
                    "lag {:.1} s corr {:+.2} inverted={}",
                    p.lag_s, p.corr, p.inverted
                )
            }
            Some(p) => format!("weak (corr {:+.2})", p.corr),
            None => "absent".to_string(),
        };
        let depth_txt = match &pp {
            Some(p) if p.corr.abs() >= SECONDARY_CORR_MIN => {
                match dp::invert_depth_single(delta, p.lag_s) {
                    dp::DepthInversion::Depth(h) => {
                        depths.push(h);
                        format!("{h:.0} km")
                    }
                    dp::DepthInversion::EdgeDiscontinuity => {
                        "edge-clamped at the 660 wall".to_string()
                    }
                    dp::DepthInversion::SaturatedBound => {
                        "saturated at the 700 ceiling".to_string()
                    }
                    dp::DepthInversion::Absent => "inversion void".to_string(),
                }
            }
            _ => "-".to_string(),
        };
        picked += 1;
        rows.push(format!(
            "{key:>12}  delta {delta:>5.1}  snr {snr:>5.1}  pP {pp_txt}  sP {sp_txt}  depth {depth_txt}"
        ));
    }

    println!();
    println!("{picked} stations passed the SNR gate");
    for r in &rows {
        println!("{r}");
    }
    println!();
    if depths.is_empty() {
        println!("no station carried a pP above the correlation gate — no depth inverted (0 honored, two readings: phase too weak or the catalog depth is off by > 30%)");
        return;
    }
    let median_depth = dp::median(&mut depths);
    let offset = median_depth - event.depth_km;
    let verdict = if offset.abs() <= DEPTH_MATCH_GATE_KM {
        "inside the match gate"
    } else {
        "outside the match gate (two readings: our scatter or the catalog)"
    };
    println!(
        "inverted depth: median {median_depth:.0} km ({} stations), catalog {:.1} km, offset {:+.1} km -> {verdict}",
        depths.len(),
        event.depth_km,
        offset
    );
    let edge_flags = rows.iter().filter(|r| r.contains("edge=true")).count();
    if edge_flags > 0 {
        println!("edge flag (pP pick at the window edge — the catalog depth may itself be off): {edge_flags} stations");
    }
}
