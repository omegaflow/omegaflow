use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GbcoRec, parse_slab2};
use omegaflow::te::{conditional_te_stats_lagged, transfer_entropy_conditional};
use omegaflow::volume::Volume;
use omegaflow_measure::depthphase as dp;
use omegaflow_measure::depthphase::{
    CATALOG_URL, MAX_DIST_DEG, MAX_STATIONS, MIN_DEPTH_KM, MIN_DIST_DEG, MIN_MAG,
    P_WINDOW_AFTER_ORIGIN_S, REGION, SEARCH_START, STATION_URL,
};
use omegaflow_measure::driver_scatter::{ols_residual_sd, sigma0};
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
const TE_FLOOR_N: usize = 32;
const N_SURR_DEFAULT: usize = 20;

struct ConditionalMask {
    n: usize,
    sigma0: f64,
    sigma1: f64,
    rho: f64,
    te: f64,
    threshold: f64,
    significant: bool,
}

fn conditional_mask(
    echo: &[f64],
    driver: &[f64],
    confound: &[f64],
    lag: usize,
    max_lag: usize,
    seed: u64,
    n_surr: usize,
) -> Option<ConditionalMask> {
    let n = echo.len();
    if n < TE_FLOOR_N || driver.len() < TE_FLOOR_N || confound.len() < TE_FLOOR_N {
        return None;
    }
    let pairs: Vec<(f64, f64)> = driver
        .iter()
        .zip(echo.iter())
        .map(|(d, e)| (*d, *e))
        .collect();
    let s0 = sigma0(&pairs)?;
    let s1 = ols_residual_sd(&pairs)?;
    if !(s0.is_finite() && s0 > 0.0) {
        return None;
    }
    let rho = s1 / s0;
    let x: Vec<f32> = echo.iter().map(|&v| v as f32).collect();
    let y: Vec<f32> = driver.iter().map(|&v| v as f32).collect();
    let c: Vec<f32> = confound.iter().map(|&v| v as f32).collect();
    let te = transfer_entropy_conditional(&x, &y, &c, lag)?;
    let (_, _, threshold) = conditional_te_stats_lagged(&x, &y, &c, lag, max_lag, seed, n_surr)?;
    Some(ConditionalMask {
        n,
        sigma0: s0,
        sigma1: s1,
        rho,
        te,
        threshold,
        significant: te > threshold,
    })
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let search_end = match dp::arg_value(&args, "--end") {
        Some(v) => v,
        None => match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => dp::unix_to_iso(d.as_secs_f64()),
            Err(_) => {
                eprintln!(
                    "depth-phase positive mask: the system clock precedes the epoch — no end time, no fabricated zero"
                );
                return;
            }
        },
    };
    let max_events = dp::arg_value(&args, "--max-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(40);
    let region = match dp::arg_value(&args, "--region") {
        Some(v) => {
            let parts: Vec<f64> = v.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            if parts.len() == 4 && parts.iter().all(|p| p.is_finite()) {
                [parts[0], parts[1], parts[2], parts[3]]
            } else {
                eprintln!(
                    "depth-phase positive mask: --region needs four comma-separated finite numbers (lat0,lat1,lon0,lon1) — the default box stands"
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
                    "depth-phase positive mask: --mindepth is not a finite non-negative number — the default {MIN_DEPTH_KM} km stands"
                );
                MIN_DEPTH_KM
            }
        },
        None => MIN_DEPTH_KM,
    };
    let lag = match dp::arg_value(&args, "--lag") {
        Some(v) => v.parse::<usize>().ok(),
        None => Some(1),
    };
    let max_lag = match dp::arg_value(&args, "--max-lag") {
        Some(v) => v.parse::<usize>().ok(),
        None => Some(4),
    };
    let n_surr = match dp::arg_value(&args, "--surrogates") {
        Some(v) => v.parse::<usize>().ok(),
        None => Some(N_SURR_DEFAULT),
    };
    let slab2_file = match dp::arg_value(&args, "--slab2-file") {
        Some(v) => v,
        None => SLAB2_LOCAL_PATH.to_string(),
    };
    let volume_file = match dp::arg_value(&args, "--volume-file") {
        Some(v) => v,
        None => VOLUME_LOCAL_PATH.to_string(),
    };

    println!("=== depth-phase positive mask — per-station path condition + conditional TE ===");
    println!("instrument (registered before the first fetch):");
    println!(
        "  part A: the ray tracer classifies each station by its pP path at (delta, catalog depth)"
    );
    println!(
        "    into fold / branch-unstable / clear; fold and branch-unstable stations fall out of the inversion"
    );
    println!(
        "  part B: the depth-phase echo (per-event offset) is conditioned on a driver (slab2 depth or LLNL dlnVp)"
    );
    println!(
        "    with the second driver as the confounder; the rift is the echo sample sd before and after"
    );
    println!(
        "    removing the driver (OLS residual); the conditional TE floor is {TE_FLOOR_N} events (a named power of two, 2^5)"
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

    let slab_records = load_slab2(&slab2_file);
    let volume = load_volume(&volume_file);

    let mut fold_total = 0usize;
    let mut unstable_total = 0usize;
    let mut clear_total = 0usize;
    let mut station_depths: Vec<f64> = Vec::new();
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
        let stations = select_spread_stations(
            &stations,
            event.lat,
            event.lon,
            MIN_DIST_DEG,
            MAX_DIST_DEG,
            MAX_STATIONS,
        );

        let mut fold = 0usize;
        let mut unstable = 0usize;
        let mut clear = 0usize;
        let mut depths: Vec<f64> = Vec::new();
        let mut first = true;
        for st in &stations {
            let delta = dp::arc_deg(event.lat, event.lon, st.lat, st.lon);
            let branch = dp::p_p_branch(delta, event.depth_km);
            match branch {
                dp::PPBranch::Fold => fold += 1,
                dp::PPBranch::BranchUnstable => unstable += 1,
                dp::PPBranch::Clear => clear += 1,
            }
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
        fold_total += fold;
        unstable_total += unstable;
        clear_total += clear;
        station_depths.extend(depths.iter().copied());
        println!(
            "  path condition over {n} stations: {fold} fold, {unstable} branch-unstable, {clear} clear",
            n = stations.len()
        );
        if depths.is_empty() {
            pending_events.push(format!(
                "{} (no station measured an unclamped depth)",
                event.id
            ));
            continue;
        }
        let median_depth = dp::median(&mut depths);
        let offset = median_depth - event.depth_km;
        println!(
            "  {} clear stations inverted a depth -> median {median_depth:.0} km, offset {:+.1} km",
            depths.len(),
            offset
        );
        fleet.push((event.clone(), offset));
    }

    println!();
    println!("=== part A — the per-station path condition (ray tracer) ===");
    println!(
        "fleet-wide classification: {fold_total} fold, {unstable_total} branch-unstable, {clear_total} clear stations"
    );
    let station_scatter = sample_sd(&station_depths);
    println!(
        "station depth scatter over {} clear-station depths: {} km",
        station_depths.len(),
        station_scatter
            .map(|v| format!("{v:.1}"))
            .unwrap_or("pending".into())
    );

    println!();
    println!("=== part B — the conditional TE (n >= {TE_FLOOR_N} floor) ===");
    println!(
        "lag {}, max_lag {}, surrogates {}",
        lag.map_or("pending".to_string(), |v| v.to_string()),
        max_lag.map_or("pending".to_string(), |v| v.to_string()),
        n_surr.map_or("pending".to_string(), |v| v.to_string())
    );
    if fleet.len() < TE_FLOOR_N {
        println!(
            "the fleet carries {n} events — below the {TE_FLOOR_N}-event floor (2^5); the conditional TE path stays unmeasured (0 honored, never a fabricated rho)",
            n = fleet.len()
        );
    } else {
        let mut ordered: Vec<(f64, f64, f64, f64)> = fleet
            .iter()
            .filter_map(|(ev, off)| {
                let slab = slab2_depth_at(slab_records.as_deref()?, ev.lat, ev.lon)?;
                let col = column_mean_dlnvp(volume.as_ref()?, ev.lat, ev.lon, ev.depth_km)?;
                Some((ev.t0, *off, slab, col))
            })
            .collect();
        ordered.sort_by(|a, b| a.0.total_cmp(&b.0));
        let echo: Vec<f64> = ordered.iter().map(|r| r.1).collect();
        let slab: Vec<f64> = ordered.iter().map(|r| r.2).collect();
        let col: Vec<f64> = ordered.iter().map(|r| r.3).collect();
        let n = echo.len();
        if n < TE_FLOOR_N {
            println!(
                "{n} events carry both drivers (floor {TE_FLOOR_N}) — the conditional TE path stays unmeasured (0 honored)"
            );
        } else {
            emit_mask(
                "slab2 depth | LLNL dlnVp",
                &echo,
                &slab,
                &col,
                lag,
                max_lag,
                n_surr,
            );
            emit_mask(
                "LLNL dlnVp | slab2 depth",
                &echo,
                &col,
                &slab,
                lag,
                max_lag,
                n_surr,
            );
        }
    }

    if !pending_events.is_empty() {
        println!();
        println!(
            "pending (not smoothed, not counted): {}",
            pending_events.join("; ")
        );
    }
}

fn emit_mask(
    label: &str,
    echo: &[f64],
    driver: &[f64],
    confound: &[f64],
    lag: Option<usize>,
    max_lag: Option<usize>,
    n_surr: Option<usize>,
) {
    let lag = lag.unwrap_or(1);
    let max_lag = max_lag.unwrap_or(4);
    let n_surr = n_surr.unwrap_or(N_SURR_DEFAULT);
    match conditional_mask(
        echo,
        driver,
        confound,
        lag,
        max_lag,
        0x9E37_79B9_7F4A_7C15,
        n_surr,
    ) {
        Some(m) => println!(
            "  {label}: n={} sigma0={:.1} km sigma1={:.1} km rho={:.3} | cTE {:.3e} threshold {:.3e} -> {}",
            m.n,
            m.sigma0,
            m.sigma1,
            m.rho,
            m.te,
            m.threshold,
            if m.significant {
                "driver significant beyond the confound"
            } else {
                "driver within the confound null"
            }
        ),
        None => println!(
            "  {label}: the mask stays unmeasured (below the floor, a constant driver, or a degenerate null)"
        ),
    }
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
        _ => match fetch_raw_bytes(cdn, 3600) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn ar1(n: usize, phi: f64, seed: u64) -> Vec<f64> {
        let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut out = Vec::with_capacity(n);
        let mut x = 0.0f64;
        for _ in 0..n {
            rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let noise = ((rng >> 33) as f64) / ((u32::MAX >> 1) as f64) * 2.0 - 1.0;
            x = phi * x + noise;
            out.push(x);
        }
        out
    }

    #[test]
    fn the_floor_is_the_named_power_of_two() {
        assert_eq!(TE_FLOOR_N, 32);
        assert_eq!(TE_FLOOR_N, 1 << 5, "the floor is a named power of two, 2^5");
    }

    #[test]
    fn below_the_floor_stays_absent() {
        let echo = ar1(31, 0.7, 1);
        let driver = ar1(31, 0.5, 2);
        let confound = ar1(31, 0.3, 3);
        assert!(
            conditional_mask(&echo, &driver, &confound, 1, 4, 1, 10).is_none(),
            "31 events stay below the floor — absent, never a fabricated rho"
        );
    }

    #[test]
    fn at_the_floor_the_mask_runs() {
        let echo = ar1(32, 0.7, 1);
        let driver = ar1(32, 0.5, 2);
        let confound = ar1(32, 0.3, 3);
        let m = conditional_mask(&echo, &driver, &confound, 1, 4, 1, 10)
            .expect("32 events meet the floor and carry a finite null");
        assert_eq!(m.n, 32);
        assert!(m.sigma0.is_finite() && m.sigma0 > 0.0);
        assert!(m.rho.is_finite());
        assert!(m.te.is_finite());
        assert!(m.threshold.is_finite());
    }

    #[test]
    fn a_constant_driver_carries_no_mask() {
        let echo = ar1(40, 0.7, 1);
        let driver = vec![3.0f64; 40];
        let confound = ar1(40, 0.3, 3);
        assert!(
            conditional_mask(&echo, &driver, &confound, 1, 4, 1, 10).is_none(),
            "a constant driver carries no OLS regression — absent, never a division by zero"
        );
    }

    #[test]
    fn a_strongly_coupled_driver_shrinks_the_rift() {
        let driver = ar1(64, 0.5, 2);
        let noise = ar1(64, 0.4, 4);
        let echo: Vec<f64> = driver
            .iter()
            .zip(noise.iter())
            .map(|(d, n)| 2.0 * d + 0.2 * n)
            .collect();
        let confound = ar1(64, 0.3, 3);
        let m = conditional_mask(&echo, &driver, &confound, 1, 4, 1, 10)
            .expect("64 events carry a finite null");
        assert!(
            m.rho < 0.5,
            "a strongly coupled driver shrinks the rift (rho {:.3})",
            m.rho
        );
    }
}
