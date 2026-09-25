use std::collections::HashMap;

use omegaflow::archivar::{
    BodyEphemeris, ExtractResult, Frame, J2000_EPOCH, SourceConfig, body_barycenter_position,
    embedded_lsk, extract, fetch_raw_bytes, load_sources,
};
use omegaflow::mathematikerin::{S2_LMAX, S2_OSC_CAP, S2Osc};

const DAY_S: f64 = 86400.0;
const AU_M: f64 = 1.495978707e11;
const PI_OVER_64: f64 = std::f64::consts::PI / 64.0;

fn date_of(tdb: f64) -> String {
    let jd = tdb / DAY_S + J2000_EPOCH;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn local_path(url: &str) -> Option<String> {
    let rest = url.split("/releases/download/").nth(1)?;
    let (netloc, asset) = rest.split_once('/')?;
    Some(format!("data/{netloc}/{asset}"))
}

fn ensure_bin(url: &str) -> Option<Vec<u8>> {
    let path = local_path(url)?;
    if let Ok(bytes) = std::fs::read(&path) {
        return Some(bytes);
    }
    let bytes = fetch_raw_bytes(url)?;
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&path, &bytes).is_err() {
        return None;
    }
    Some(bytes)
}

fn span_tdb(eph: &BodyEphemeris) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in &eph.granules {
        lo = lo.min((g.t0_jd - g.dt_jd - J2000_EPOCH) * DAY_S);
        hi = hi.max((g.t0_jd + g.dt_jd - J2000_EPOCH) * DAY_S);
    }
    if lo.is_finite() && hi.is_finite() && hi > lo {
        Some((lo, hi))
    } else {
        None
    }
}

fn norm3(p: [f64; 3]) -> f64 {
    (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()
}

fn body_format(s: &SourceConfig) -> bool {
    s.format == "ephemeris_binary" || s.format == "orbit_bin"
}

fn main() {
    let sources = load_sources();
    let mut body_sources: Vec<(String, SourceConfig)> = sources
        .iter()
        .filter(|s| body_format(s))
        .filter_map(|s| s.body.clone().map(|b| (b, s.clone())))
        .collect();
    body_sources.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.url.cmp(&b.1.url)));
    body_sources.dedup_by(|a, b| a.0 == b.0);

    let mut stations: Vec<(String, f64, f64, f64)> = sources
        .iter()
        .filter_map(|s| match &s.frame {
            Frame::Surface {
                body_name,
                lat,
                lon,
                alt,
            } => Some((body_name.clone(), *lat, *lon, *alt)),
            Frame::Barycenter { .. } | Frame::Manifest => None,
        })
        .collect();
    stations.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.1.total_cmp(&b.1))
            .then(a.2.total_cmp(&b.2))
            .then(a.3.total_cmp(&b.3))
    });

    let Some(lsk) = embedded_lsk() else {
        println!(
            "s2-weberin-probe: the embedded naif0012.tls reads empty - the epoch stays absent (0 honored)"
        );
        std::process::exit(2);
    };
    let Some(tdb) = lsk.system_now_tdb() else {
        println!(
            "s2-weberin-probe: system_now_tdb carries no TDB - the epoch stays absent (0 honored)"
        );
        std::process::exit(2);
    };
    let date = date_of(tdb);

    println!("=== s2-weberin-probe - the council measurements for step 3 (body fold, sorted) ===");
    println!("epoch   : now, TDB {tdb:.3} s since J2000 = {date} (naif0012.tls, system_now_tdb)");
    println!(
        "source  : phi/sources.φ via load_sources - {} registered body source(s) (ephemeris_binary/orbit_bin), {} registered station(s) (Frame::Surface)",
        body_sources.len(),
        stations.len()
    );
    println!(
        "kernel  : S2_LMAX = {S2_LMAX}, pi/64 = {PI_OVER_64:.6} rad = {:.3} deg, cap = {S2_OSC_CAP}",
        PI_OVER_64.to_degrees()
    );

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut absent_bins: Vec<String> = Vec::new();
    for (name, src) in &body_sources {
        let Some(path) = local_path(&src.url) else {
            absent_bins.push(name.clone());
            continue;
        };
        if ensure_bin(&src.url).is_none() {
            absent_bins.push(name.clone());
            continue;
        }
        match extract(src, &path, tdb, &lsk) {
            ExtractResult::WithEphemeris(_, body_eph) => {
                eph.insert(name.clone(), *body_eph);
            }
            _ => absent_bins.push(name.clone()),
        }
    }
    println!(
        "bodies  : {} of {} loaded, {} bin(s) empty/absent{}",
        eph.len(),
        body_sources.len(),
        absent_bins.len(),
        if absent_bins.is_empty() {
            String::new()
        } else {
            format!(": {}", absent_bins.join(", "))
        }
    );
    println!();

    let mut body_names: Vec<String> = eph.keys().cloned().collect();
    body_names.sort();

    let mut max_alpha: Option<(String, f64, f64)> = None;
    let mut sun_d: Option<f64> = None;
    let mut sun_r: Option<f64> = None;
    let mut sun_alpha: Option<f64> = None;
    for name in &body_names {
        let Some(eph_body) = eph.get(name) else {
            continue;
        };
        let Some(props) = &eph_body.props else {
            continue;
        };
        let r = props.radius_m;
        let Some(p) = body_barycenter_position(name, tdb, &eph) else {
            continue;
        };
        let d = norm3(p);
        if name == "sun" {
            sun_d = Some(d);
            sun_r = Some(r);
            if r.is_finite() && r > 0.0 && d > r {
                sun_alpha = Some(2.0 * (r / d).asin());
            }
            continue;
        }
        if r.is_finite() && r > 0.0 && d > r {
            let alpha = 2.0 * (r / d).asin();
            if max_alpha.as_ref().is_none_or(|(_, ma, _)| alpha > *ma) {
                max_alpha = Some((name.clone(), alpha, d));
            }
        }
    }
    println!(
        "1. sub-resolution (alpha = 2*asin(radius_m/d), d = SSB distance, radius_m from BodyProperties)"
    );
    match &max_alpha {
        Some((name, alpha, _d)) => {
            let sub = *alpha < PI_OVER_64;
            let sun_sub = sun_alpha.is_some_and(|a| a < PI_OVER_64);
            let sun_txt = match (sun_d, sun_r, sun_alpha) {
                (Some(ds), Some(rs), Some(a)) => format!(
                    "sun outside: SSB distance {ds:.6e} m ({:.4} AU) against R_sun {rs:.6e} m, alpha_sun = {a:.6} rad",
                    ds / AU_M
                ),
                (Some(ds), Some(rs), None) => format!(
                    "sun inside: SSB distance {ds:.6e} m against R_sun {rs:.6e} m (d <= radius_m, alpha absent)"
                ),
                _ => "sun absent (no radius_m or no position)".to_string(),
            };
            let verdict = if !sub {
                format!("{name} lies above pi/64 - not sub-resolution")
            } else if sun_sub {
                "sub-resolution for all bodies, the sun included".to_string()
            } else {
                "sub-resolution for all bodies except the sun".to_string()
            };
            println!(
                "   largest body angular extent ({date}): {name} alpha = {alpha:.6} rad against pi/64 - {verdict} ({sun_txt})"
            );
        }
        None => println!(
            "   largest body angular extent ({date}): absent - no body carries radius_m with d > radius_m"
        ),
    }

    let mut body_live = 0usize;
    for name in &body_names {
        if S2Osc::from_body(name, tdb, &eph).is_some() {
            body_live += 1;
        }
    }
    let mut n_e = 0usize;
    let mut silent_stations = 0usize;
    for (body, lat, lon, alt) in &stations {
        match S2Osc::from_station(body, *lat, *lon, *alt, tdb, &eph) {
            Some(_) => {
                if body == "earth" {
                    n_e += 1;
                }
            }
            None => silent_stations += 1,
        }
    }
    let station_live = stations.len() - silent_stations;
    let n_live = body_live + station_live;
    let earth_d = eph
        .get("earth")
        .and_then(|_| body_barycenter_position("earth", tdb, &eph))
        .map(norm3);
    let earth_r = eph
        .get("earth")
        .and_then(|e| e.props.as_ref())
        .map(|p| p.radius_m);
    println!();
    println!(
        "2. earth stacking (theta_max = asin(R_E/d_E), R_E from BodyProperties, stations from phi/sources.φ)"
    );
    match (earth_d, earth_r) {
        (Some(d), Some(r)) if r.is_finite() && r > 0.0 && d > r => {
            let theta = (r / d).asin();
            let f_e = if n_live > 0 {
                format!("{:.6}", (n_e as f64 + 1.0) / n_live as f64)
            } else {
                "absent".to_string()
            };
            println!(
                "   {n_e} station(s) + 1 earth body fold within theta_max = {theta:.6e} rad = {:.4} deg of the earth direction (kernel pi/64 ~ {:.1} deg) - the earth stack carries f_E = {f_e} of the shell sum-omega.",
                theta.to_degrees(),
                PI_OVER_64.to_degrees()
            );
        }
        (Some(d), Some(r)) => println!(
            "   earth stacking: absent - d_E = {d:.6e} m against R_E = {r:.6e} m (d <= radius_m, theta_max not measurable)"
        ),
        _ => println!(
            "   earth stacking: absent - earth carries no position or no radius_m at {date}"
        ),
    }
    if silent_stations > 0 {
        println!(
            "   {silent_stations} station(s) without coverage rest - 0 honored (their anchor body carries no point at {date})"
        );
    }

    let mut silent_worldlines: Vec<String> = Vec::new();
    let mut next_boundary: Option<(String, f64)> = None;
    let mut spans: Vec<(String, Option<(f64, f64)>)> = Vec::new();
    for name in &body_names {
        let Some(eph_body) = eph.get(name) else {
            continue;
        };
        match span_tdb(eph_body) {
            Some((lo, hi)) => {
                spans.push((name.clone(), Some((lo, hi))));
                if tdb < lo || tdb > hi {
                    silent_worldlines.push(name.clone());
                }
                if hi > tdb {
                    let remaining = hi - tdb;
                    if next_boundary.as_ref().is_none_or(|(_, r)| remaining < *r) {
                        next_boundary = Some((name.clone(), remaining));
                    }
                }
            }
            None => {
                spans.push((name.clone(), None));
                silent_worldlines.push(name.clone());
            }
        }
    }
    let n_bodies = body_names.len();
    println!();
    println!("3. coverage at t_presence (granule span in TDB, J2000-relative)");
    for (name, span) in &spans {
        match span {
            Some((lo, hi)) => println!(
                "   {name}: [{lo:.0}, {hi:.0}] s = [{} .. {}]",
                date_of(*lo),
                date_of(*hi)
            ),
            None => println!("   {name}: span absent (no granules)"),
        }
    }
    let boundary_txt = match &next_boundary {
        Some((name, remaining)) => format!(
            "{name} ends {} (in {:.1} d)",
            date_of(tdb + remaining),
            remaining / DAY_S
        ),
        None => "no open boundary after the epoch".to_string(),
    };
    let silent_txt = if silent_worldlines.is_empty() {
        "none".to_string()
    } else {
        silent_worldlines.join(", ")
    };
    println!(
        "   at t_presence = {date}: {} of {n_bodies} worldlines without coverage: {silent_txt}; next boundary: {boundary_txt}",
        silent_worldlines.len()
    );

    println!();
    println!("4. shell baseline (weight each 1.0, directions/events absent)");
    println!(
        "   shell baseline ({date}, without directions/events): sum-omega = {n_live} (bodies {body_live} + stations {station_live})."
    );

    let cap = S2_OSC_CAP as usize;
    let margin = cap as i64 - n_live as i64;
    println!();
    println!("5. cap: 2^13 = {cap}, live folded N = {n_live}, margin = {margin}");
}
