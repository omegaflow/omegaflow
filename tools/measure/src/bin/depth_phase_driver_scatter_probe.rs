use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GbcoRec, parse_slab2};
use omegaflow::volume::Volume;
use omegaflow_measure::depthphase as dp;
use omegaflow_measure::depthphase::{
    CATALOG_URL, MAX_DIST_DEG, MAX_STATIONS, MIN_DEPTH_KM, MIN_DIST_DEG, MIN_MAG,
    P_WINDOW_AFTER_ORIGIN_S, REGION, SEARCH_START, STATION_URL,
};
use omegaflow_measure::driver_scatter::{MIN_N, ScatterResult, scatter};
use omegaflow_measure::stats::sample_sd;
use std::env;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SLAB2_CDN_URL: &str =
    "https://github.com/omegaflow/sources/releases/download/www.sciencebase.gov/slab2_depth.bin";
const SLAB2_LOCAL_PATH: &str = "data/www.sciencebase.gov/slab2_depth.bin";
const VOLUME_CDN_URL: &str = "https://github.com/omegaflow/sources/releases/download/media.githubusercontent.com/LLNL_G3D_JPS.volume.bin";
const VOLUME_LOCAL_PATH: &str = "data/LLNL_G3D_JPS.volume.bin";
const SLAB_NEAR_RADIUS_DEG: f64 = 0.5;
const N_PERMS_DEFAULT: usize = 1000;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let search_end = match dp::arg_value(&args, "--end") {
        Some(v) => v,
        None => match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => dp::unix_to_iso(d.as_secs_f64()),
            Err(_) => {
                eprintln!(
                    "depth-phase driver scatter: the system clock precedes the epoch — no end time, no fabricated zero"
                );
                return;
            }
        },
    };
    let max_events = dp::arg_value(&args, "--max-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(16);
    let region = match dp::arg_value(&args, "--region") {
        Some(v) => {
            let parts: Vec<f64> = v.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            if parts.len() == 4 && parts.iter().all(|p| p.is_finite()) {
                [parts[0], parts[1], parts[2], parts[3]]
            } else {
                eprintln!(
                    "depth-phase driver scatter: --region needs four comma-separated finite numbers (lat0,lat1,lon0,lon1) — the default box stands"
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
                    "depth-phase driver scatter: --mindepth is not a finite non-negative number — the default {MIN_DEPTH_KM} km stands"
                );
                MIN_DEPTH_KM
            }
        },
        None => MIN_DEPTH_KM,
    };
    let n_perms = match dp::arg_value(&args, "--permutations") {
        Some(v) => v.parse::<usize>().unwrap_or(N_PERMS_DEFAULT).max(2),
        None => N_PERMS_DEFAULT,
    };
    let slab2_file = match dp::arg_value(&args, "--slab2-file") {
        Some(v) => v,
        None => SLAB2_LOCAL_PATH.to_string(),
    };
    let volume_file = match dp::arg_value(&args, "--volume-file") {
        Some(v) => v,
        None => VOLUME_LOCAL_PATH.to_string(),
    };

    println!("=== depth-phase driver scatter — per-driver sigma reduction, permutation null ===");
    println!("instrument (registered before the first fetch):");
    println!(
        "  sigma0 = sample standard deviation of the fleet per-event depth offset (offset = measured depth - catalog depth),"
    );
    println!(
        "  re-measured fresh from depthphase.rs — the audit's 19 km is a citation, not an input"
    );
    println!(
        "  per driver, separately: OLS of offset on the driver -> sigma1 = residual sample sd; rho = sigma1/sigma0"
    );
    println!(
        "  null = {n_perms} permutations of the driver<->event pairing -> the rho_null distribution;"
    );
    println!(
        "  the test is two-sided (rho beyond mean +/- 2 sd of rho_null); the mask question (reduction) is answered by the lower tail alone"
    );
    println!(
        "selection rule: depth >= {min_depth_km} km, magnitude >= {MIN_MAG}, box lat {}..{} lon {}..{}, up to {max_events} events",
        region[0], region[1], region[2], region[3]
    );
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

    let slab_records = load_slab2(&slab2_file);
    let volume = load_volume(&volume_file);

    let mut fleet: Vec<(dp::Event, f64)> = Vec::new();
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
        match measure_event_offset(event) {
            Some(offset) => {
                println!("  measured offset {:+.1} km", offset);
                fleet.push((event.clone(), offset));
            }
            None => pending_events.push(format!(
                "{} (no station measured an unclamped depth)",
                event.id
            )),
        }
    }

    println!();
    println!("=== fleet sigma0 ===");
    if fleet.is_empty() {
        println!("no event measured an offset — the fleet carries no statistic (0 honored)");
        return;
    }
    let fleet_offsets: Vec<f64> = fleet.iter().map(|(_, o)| *o).collect();
    let fleet_sd = sample_sd(&fleet_offsets);
    let fleet_txt = fleet_offsets
        .iter()
        .map(|o| format!("{:+.0}", o))
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "sigma0 = {} km over {} events; per-event offsets [{fleet_txt}] km",
        fleet_sd
            .map(|v| format!("{v:.1}"))
            .unwrap_or("pending".into()),
        fleet.len()
    );

    println!();
    println!("=== driver 1: slab2 depth at the epicenter ===");
    println!(
        "AUDIT: slab2 depths and the catalog depth may share ancestry (slab-constrained catalogs) — the circularity stands, never smoothed"
    );
    println!(
        "slab2 record: elev carries the negative interface depth in meters (SLB2 grid); slab depth km = -elev/1000"
    );
    match &slab_records {
        Some(records) => {
            let pairs: Vec<(f64, f64)> = fleet
                .iter()
                .filter_map(|(ev, off)| {
                    let d = slab2_depth_at(records, ev.lat, ev.lon)?;
                    Some((d, *off))
                })
                .collect();
            emit_driver("slab2 depth at the epicenter (km)", &pairs, n_perms);
        }
        None => println!(
            "  slab2 driver: absent (no SLB2 record stands) — the scatter stays unmeasured (0 honored)"
        ),
    }

    println!();
    println!("=== driver 2: LLNL_G3D_JPS velocity column ===");
    println!(
        "velocity driver statistic: thickness-weighted column mean of dlnVp — the trapezoidal integral of dlnVp(z)"
    );
    println!(
        "  at the epicenter (lat, lon) over the model depth levels from the surface (shallowest model level) down to the catalog depth,"
    );
    println!(
        "  divided by the catalog depth; the single estimated-depth point is never sampled alone"
    );
    match &volume {
        Some(vol) => {
            let pairs: Vec<(f64, f64)> = fleet
                .iter()
                .filter_map(|(ev, off)| {
                    let d = column_mean_dlnvp(vol, ev.lat, ev.lon, ev.depth_km)?;
                    Some((d, *off))
                })
                .collect();
            emit_driver(
                "column mean dlnVp (surface -> catalog depth)",
                &pairs,
                n_perms,
            );
        }
        None => println!(
            "  velocity driver: absent (no volume contract stands) — the scatter stays unmeasured (0 honored)"
        ),
    }

    if !pending_events.is_empty() {
        println!();
        println!(
            "pending (not smoothed, not counted): {}",
            pending_events.join("; ")
        );
    }
}

fn emit_driver(label: &str, pairs: &[(f64, f64)], n_perms: usize) {
    if pairs.len() < MIN_N {
        println!(
            "  {label}: {} events carry both driver and offset (floor {MIN_N}) — the scatter stays unmeasured (0 honored)",
            pairs.len()
        );
        return;
    }
    match scatter(pairs, n_perms, 1) {
        Some(r) => emit_result(label, &r),
        None => println!(
            "  {label}: the scatter stays unmeasured (constant driver or degenerate residuals)"
        ),
    }
}

fn emit_result(label: &str, r: &ScatterResult) {
    println!(
        "  {label}: n={} sigma0={:.1} km sigma1={:.1} km rho={:.3} | null rho mean {:.3} sd {:.3} band [{:.3}, {:.3}] -> {}",
        r.n,
        r.sigma0,
        r.sigma1,
        r.rho,
        r.null_mean,
        r.null_sd,
        r.null_band_lo,
        r.null_band_hi,
        r.verdict_word()
    );
}

fn measure_event_offset(event: &dp::Event) -> Option<f64> {
    let start = dp::unix_to_iso(event.t0);
    let end = dp::unix_to_iso(event.t0 + P_WINDOW_AFTER_ORIGIN_S);
    let st_url = format!(
        "{STATION_URL}?format=text&level=channel&latitude={:.4}&longitude={:.4}&minradius={MIN_DIST_DEG}&maxradius={MAX_DIST_DEG}&channel=BHZ&starttime={start}&endtime={end}&includerestricted=false",
        event.lat, event.lon
    );
    let st_body = fetch_raw(&st_url, None, &[])?;
    let mut stations = dp::parse_stations_text(&st_body);
    let mut seen = std::collections::HashSet::new();
    stations.retain(|s| seen.insert(format!("{}.{}", s.net, s.sta)));
    let stations = select_spread_stations(
        &stations,
        event.lat,
        event.lon,
        MIN_DIST_DEG,
        MAX_DIST_DEG,
        MAX_STATIONS,
    );

    let mut depths: Vec<f64> = Vec::new();
    let mut first = true;
    for st in &stations {
        if !first {
            thread::sleep(Duration::from_millis(1000));
        }
        first = false;
        let m = dp::measure_station(event, st, &start, &end, None);
        if m.skip.is_some() {
            continue;
        }
        if let Some(dp::DepthInversion::Depth(h)) = m.inversion {
            depths.push(h);
        }
    }
    if depths.is_empty() {
        return None;
    }
    let median_depth = dp::median(&mut depths);
    Some(median_depth - event.depth_km)
}

fn slab2_depth_at(records: &[GbcoRec], lat: f64, lon: f64) -> Option<f64> {
    let mut best: Option<(f64, f64)> = None;
    for r in records {
        let d = dp::arc_deg(lat, lon, r.lat, r.lon);
        if !d.is_finite() || d > SLAB_NEAR_RADIUS_DEG {
            continue;
        }
        match best {
            Some((bd, _)) if d >= bd => {}
            _ => best = Some((d, -r.elev / 1000.0)),
        }
    }
    best.map(|(_, depth)| depth)
}

fn column_mean_dlnvp(vol: &Volume, lat: f64, lon: f64, depth_km: f64) -> Option<f64> {
    if !(depth_km.is_finite() && depth_km > 0.0) {
        return None;
    }
    let mut levels: Vec<f64> = vol.axes[0]
        .values
        .iter()
        .copied()
        .filter(|z| *z >= 0.0 && *z <= depth_km)
        .collect();
    if levels.is_empty() {
        return None;
    }
    levels.sort_by(|a, b| a.total_cmp(b));
    let mut zs: Vec<f64> = Vec::with_capacity(levels.len() + 1);
    let mut vs: Vec<f64> = Vec::with_capacity(levels.len() + 1);
    let mut prev: Option<f64> = None;
    for z in levels {
        if let Some(p) = prev {
            if (z - p).abs() < 1e-9 {
                continue;
            }
        }
        let v = vol.sample_at([z, lat, lon])?;
        zs.push(z);
        vs.push(v);
        prev = Some(z);
    }
    let deepest = *zs.last()?;
    if (depth_km - deepest).abs() > 1e-9 {
        let v = vol.sample_at([depth_km, lat, lon])?;
        zs.push(depth_km);
        vs.push(v);
    }
    if zs.len() < 2 {
        return None;
    }
    let shallow = zs[0];
    let span = depth_km - shallow;
    if span <= 0.0 {
        return None;
    }
    let mut integral = 0.0;
    for i in 1..zs.len() {
        integral += (vs[i - 1] + vs[i]) * (zs[i] - zs[i - 1]) / 2.0;
    }
    Some(integral / span)
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

fn load_slab2(local: &str) -> Option<Vec<GbcoRec>> {
    let bytes = load_bytes(local, SLAB2_CDN_URL)?;
    let records = parse_slab2(&bytes);
    match &records {
        Some(r) => println!("slab2: {} slab depth records", r.len()),
        None => {
            println!("slab2: the body reads no SLB2 record — the driver stays absent (0 honored)")
        }
    }
    records
}

fn load_volume(local: &str) -> Option<Volume> {
    let bytes = load_bytes(local, VOLUME_CDN_URL)?;
    let vol = Volume::read_bin(&bytes);
    match &vol {
        Some(v) => println!(
            "volume: {}x{}x{} (depth,lat,lon)",
            v.dims[0], v.dims[1], v.dims[2]
        ),
        None => println!(
            "volume: the body reads no volume contract — the driver stays absent (0 honored)"
        ),
    }
    vol
}

fn load_bytes(local: &str, cdn: &str) -> Option<Vec<u8>> {
    match std::fs::read(local) {
        Ok(bytes) if !bytes.is_empty() => {
            println!("{local}: {} bytes read from the local file", bytes.len());
            Some(bytes)
        }
        _ => match fetch_raw_bytes(cdn) {
            Some(bytes) => {
                println!(
                    "{local}: absent locally — {} bytes fetched from the CDN",
                    bytes.len()
                );
                Some(bytes)
            }
            None => {
                println!(
                    "{local}: absent locally and the CDN returned void — the driver stays absent (0 honored)"
                );
                None
            }
        },
    }
}
