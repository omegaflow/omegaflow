use omegaflow::archivar::{
    body_barycenter_position, body_fixed_to_icrs_smooth, embedded_lsk, fetch_raw_bytes,
    light_time_worldline, parse_ephemeris_binary, BodyEphemeris, LeapSeconds,
};
use omegaflow::cdn::CDN_BASE;
use std::collections::HashMap;
use std::path::Path;

const DAY_S: f64 = 86400.0;
const RAD_DEG: f64 = 180.0 / std::f64::consts::PI;
const EARTH_MEAN_RADIUS_M: f64 = 6371.0e3;
const EVENT_UNIX: f64 = 1503273600.0;
const SEARCH_COARSE_S: f64 = 60.0;
const SEARCH_FINE_S: f64 = 0.01;
const BIN_TTL_S: u64 = 604800;

struct LineSpec {
    word: &'static str,
    netloc: &'static str,
    sun_asset: &'static str,
    moon_asset: &'static str,
    earth_asset: &'static str,
}

const LINES: [LineSpec; 3] = [
    LineSpec {
        word: "de441",
        netloc: "ssd.jpl.nasa.gov",
        sun_asset: "ephemeris_sun.bin",
        moon_asset: "ephemeris_moon.bin",
        earth_asset: "ephemeris_earth.bin",
    },
    LineSpec {
        word: "inpop19a",
        netloc: "ftp.imcce.fr",
        sun_asset: "ephemeris_inpop_sun.bin",
        moon_asset: "ephemeris_inpop_moon.bin",
        earth_asset: "ephemeris_inpop_earth.bin",
    },
    LineSpec {
        word: "epm2021",
        netloc: "ftp.iaaras.ru",
        sun_asset: "ephemeris_epm_sun.bin",
        moon_asset: "ephemeris_epm_moon.bin",
        earth_asset: "ephemeris_epm_earth.bin",
    },
];

struct Line {
    word: &'static str,
    map: HashMap<String, BodyEphemeris>,
    sun_radius_m: Option<f64>,
    moon_radius_m: Option<f64>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() {
    println!("usage: eclipse_shadow_probe [--eph-dir <data-root>]");
}

fn vlen(v: [f64; 3]) -> Option<f64> {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if n.is_finite() && n > 0.0 {
        Some(n)
    } else {
        None
    }
}

fn vsub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vdot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn vcross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn vunit(v: [f64; 3]) -> Option<[f64; 3]> {
    let n = vlen(v)?;
    Some([v[0] / n, v[1] / n, v[2] / n])
}

fn ensure_bin(path: &str, netloc: &str, asset: &str, ttl: u64) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("{CDN_BASE}/{netloc}/{asset}");
    let bytes = fetch_raw_bytes(&url, ttl)?;
    if let Some(parent) = Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, &bytes);
    Some(bytes)
}

fn load_line(spec: &LineSpec, eph_dir: &str) -> Option<Line> {
    let mut map: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut sun_radius_m = None;
    let mut moon_radius_m = None;
    for (name, asset) in [
        ("sun", spec.sun_asset),
        ("moon", spec.moon_asset),
        ("earth", spec.earth_asset),
    ] {
        let path = format!("{eph_dir}/{}/{asset}", spec.netloc);
        let Some(bytes) = ensure_bin(&path, spec.netloc, asset, BIN_TTL_S) else {
            println!(
                "eclipse {}: {path} bin void — absent on disk and the CDN fetch returned non-200 — the line stays unread",
                spec.word
            );
            return None;
        };
        let Some(eph) = parse_ephemeris_binary(&bytes) else {
            println!(
                "eclipse {}: {path} reads but does not parse to a BodyEphemeris",
                spec.word
            );
            return None;
        };
        let radius = eph.props.as_ref().and_then(|p| {
            let r = p.radius_m;
            if r.is_finite() && r > 0.0 {
                Some(r)
            } else {
                None
            }
        });
        if name == "sun" {
            sun_radius_m = radius;
        } else if name == "moon" {
            moon_radius_m = radius;
        }
        map.insert(name.to_string(), eph);
    }
    Some(Line {
        word: spec.word,
        map,
        sun_radius_m,
        moon_radius_m,
    })
}

fn covers_event(line: &Line, day_tdb: f64) -> bool {
    ["sun", "moon", "earth"]
        .iter()
        .all(|name| body_barycenter_position(name, day_tdb, &line.map).is_some())
}

fn apparent_bodies(line: &Line, station: [f64; 3], t: f64) -> Option<([f64; 3], [f64; 3])> {
    let sun = light_time_worldline(station, t, &|s| {
        body_barycenter_position("sun", s, &line.map)
    })?
    .0;
    let moon = light_time_worldline(station, t, &|s| {
        body_barycenter_position("moon", s, &line.map)
    })?
    .0;
    Some((sun, moon))
}

fn separation_rad(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let ua = vunit(a)?;
    let ub = vunit(b)?;
    let c = vdot(ua, ub).clamp(-1.0, 1.0);
    Some(c.acos())
}

fn elongation_rad(line: &Line, t: f64) -> Option<f64> {
    let geo = body_barycenter_position("earth", t, &line.map)?;
    let (sun, moon) = apparent_bodies(line, geo, t)?;
    separation_rad(vsub(sun, geo), vsub(moon, geo))
}

fn axis_state(line: &Line, t: f64) -> Option<([f64; 3], [f64; 3], [f64; 3])> {
    let geo = body_barycenter_position("earth", t, &line.map)?;
    let (sun, moon) = apparent_bodies(line, geo, t)?;
    Some((geo, sun, moon))
}

fn axis_unit(sun: [f64; 3], moon: [f64; 3]) -> Option<[f64; 3]> {
    vunit(vsub(moon, sun))
}

fn axis_geocenter_miss(geo: [f64; 3], sun: [f64; 3], moon: [f64; 3]) -> Option<f64> {
    let u = axis_unit(sun, moon)?;
    vlen(vcross(vsub(geo, moon), u))
}

fn angular_radius(body_radius_m: f64, dist_m: f64) -> Option<f64> {
    if !dist_m.is_finite() || dist_m <= body_radius_m {
        return None;
    }
    Some((body_radius_m / dist_m).asin())
}

fn magnitude_at(line: &Line, t: f64, obs: [f64; 3]) -> Option<f64> {
    let sun_r = line.sun_radius_m?;
    let moon_r = line.moon_radius_m?;
    let (sun, moon) = apparent_bodies(line, obs, t)?;
    let theta = separation_rad(vsub(sun, obs), vsub(moon, obs))?;
    let rs = angular_radius(sun_r, vlen(vsub(sun, obs))?)?;
    let rm = angular_radius(moon_r, vlen(vsub(moon, obs))?)?;
    let out = (rs + rm - theta) / (2.0 * rs);
    if out.is_finite() {
        Some(out)
    } else {
        None
    }
}

fn sunlit(line: &Line, t: f64, obs: [f64; 3], geo: [f64; 3]) -> bool {
    let Some(sun) =
        light_time_worldline(obs, t, &|s| body_barycenter_position("sun", s, &line.map))
            .map(|(p, _)| p)
    else {
        return false;
    };
    let (Some(outward), Some(sun_dir)) = (vunit(vsub(obs, geo)), vunit(vsub(sun, obs))) else {
        return false;
    };
    vdot(sun_dir, outward) > 0.0
}

fn best_time(lo: f64, hi: f64, cost: &dyn Fn(f64) -> Option<f64>) -> Option<(f64, f64)> {
    let mut best: Option<(f64, f64)> = None;
    let mut t = lo;
    while t <= hi {
        if let Some(c) = cost(t) {
            if best.map_or(true, |(_, bc)| c < bc) {
                best = Some((t, c));
            }
        }
        t += SEARCH_COARSE_S;
    }
    let (tb, cb) = best?;
    let fine_lo = (tb - SEARCH_COARSE_S).max(lo);
    let fine_hi = (tb + SEARCH_COARSE_S).min(hi);
    let mut best2: Option<(f64, f64)> = None;
    let mut tf = fine_lo;
    while tf <= fine_hi {
        if let Some(c) = cost(tf) {
            if best2.map_or(true, |(_, bc)| c < bc) {
                best2 = Some((tf, c));
            }
        }
        tf += SEARCH_FINE_S;
    }
    let (t2, c2) = best2?;
    if c2 < cb {
        Some((t2, c2))
    } else {
        Some((tb, cb))
    }
}

fn syzygy(line: &Line, day_tdb: f64) -> Option<(f64, f64)> {
    best_time(day_tdb, day_tdb + DAY_S, &|t| elongation_rad(line, t))
}

struct Surface {
    lat: f64,
    lon: f64,
    score: f64,
}

fn best_in_box(
    line: &Line,
    t: f64,
    geo: [f64; 3],
    lat_c: f64,
    lon_c: f64,
    half_lat: f64,
    half_lon: f64,
    step: f64,
) -> Option<Surface> {
    let mut best: Option<Surface> = None;
    let mut lat = lat_c - half_lat;
    while lat <= lat_c + half_lat {
        let mut lon = lon_c - half_lon;
        while lon <= lon_c + half_lon {
            let lon_w = wrap_lon(lon);
            let Some(obs) = body_fixed_to_icrs_smooth("earth", lat, lon_w, 0.0, t, &line.map)
            else {
                lon += step;
                continue;
            };
            if !sunlit(line, t, obs, geo) {
                lon += step;
                continue;
            }
            let Some(score) = magnitude_at(line, t, obs).filter(|m| m.is_finite()) else {
                lon += step;
                continue;
            };
            if best.as_ref().map_or(true, |b| score > b.score) {
                best = Some(Surface {
                    lat,
                    lon: lon_w,
                    score,
                });
            }
            lon += step;
        }
        lat += step;
    }
    best
}

fn wrap_lon(lon: f64) -> f64 {
    let mut l = lon.rem_euclid(360.0);
    if l > 180.0 {
        l -= 360.0;
    }
    l
}

fn deepest_pierce(line: &Line, t: f64, seed: Option<(f64, f64)>) -> Option<Surface> {
    let geo = body_barycenter_position("earth", t, &line.map)?;
    if line.sun_radius_m.is_none() || line.moon_radius_m.is_none() {
        return None;
    }
    let mut b = match seed {
        Some((lat, lon)) => Surface {
            lat,
            lon,
            score: 0.0,
        },
        None => Surface {
            lat: 0.0,
            lon: 0.0,
            score: 0.0,
        },
    };
    let (mut half, mut step) = match seed {
        Some(_) => (0.25, 0.05),
        None => (90.0, 5.0),
    };
    loop {
        if let Some(nb) = best_in_box(line, t, geo, b.lat, b.lon, half, half, step) {
            b = nb;
        }
        if step <= 0.002 {
            break;
        }
        half = step;
        step = step / 5.0;
    }
    Some(b)
}

fn greatest_time(line: &Line, day_tdb: f64) -> Option<(f64, f64)> {
    best_time(day_tdb, day_tdb + DAY_S, &|t| {
        axis_state(line, t).and_then(|(geo, sun, moon)| axis_geocenter_miss(geo, sun, moon))
    })
}

fn eclipse_window(line: &Line, day_tdb: f64) -> Option<(f64, f64)> {
    let step = 60.0;
    let mut lo: Option<f64> = None;
    let mut hi: Option<f64> = None;
    let mut t = day_tdb;
    while t < day_tdb + DAY_S {
        let inside = match axis_state(line, t) {
            Some((g, s, m)) => match axis_geocenter_miss(g, s, m) {
                Some(h) => h < 6.6e6,
                None => false,
            },
            None => false,
        };
        if inside {
            if lo.is_none() {
                lo = Some(t);
            }
            hi = Some(t);
        }
        t += step;
    }
    let lo = (lo? - 120.0).max(day_tdb);
    let hi = (hi? + 120.0).min(day_tdb + DAY_S);
    Some((lo, hi))
}

struct Coverage {
    t: f64,
    lat: f64,
    lon: f64,
    fraction: f64,
}

fn greatest_coverage(line: &Line, day_tdb: f64) -> Option<Coverage> {
    let (lo, hi) = eclipse_window(line, day_tdb)?;
    let mid = (lo + hi) * 0.5;
    let center = deepest_pierce(line, mid, None)?;
    let mut best = Coverage {
        t: mid,
        lat: center.lat,
        lon: center.lon,
        fraction: center.score,
    };
    let consider = |t: f64, b: &Surface, best: &mut Coverage| {
        if b.score > best.fraction {
            best.fraction = b.score;
            best.t = t;
            best.lat = b.lat;
            best.lon = b.lon;
        }
    };
    let mut walk = |t0: f64, toward: f64, lo: f64, hi: f64, mut prev: (f64, f64)| {
        let mut t = t0;
        while (toward > 0.0 && t <= hi) || (toward < 0.0 && t >= lo) {
            if let Some(b) = deepest_pierce(line, t, Some(prev)) {
                prev = (b.lat, b.lon);
                consider(t, &b, &mut best);
            }
            t += toward;
        }
    };
    walk(mid + 4.0, 4.0, lo, hi, (center.lat, center.lon));
    walk(mid - 4.0, -4.0, lo, hi, (center.lat, center.lon));
    let mut t = best.t - 4.0;
    while t <= best.t + 4.0 {
        if let Some(b) = deepest_pierce(line, t, Some((best.lat, best.lon))) {
            consider(t, &b, &mut best);
        }
        t += 0.2;
    }
    Some(best)
}

fn diameter_ratio_at(line: &Line, t: f64, obs: [f64; 3]) -> Option<f64> {
    let sun_r = line.sun_radius_m?;
    let moon_r = line.moon_radius_m?;
    let (sun, moon) = apparent_bodies(line, obs, t)?;
    let rs = angular_radius(sun_r, vlen(vsub(sun, obs))?)?;
    let rm = angular_radius(moon_r, vlen(vsub(moon, obs))?)?;
    Some(rm / rs)
}

fn arc_deg(a_lat: f64, a_lon: f64, b_lat: f64, b_lon: f64) -> f64 {
    let dlat = (a_lat - b_lat).to_radians();
    let dlon = (a_lon - b_lon).to_radians();
    let la = a_lat.to_radians();
    let lb = b_lat.to_radians();
    let h = (dlat / 2.0).sin() * (dlat / 2.0).sin()
        + la.cos() * lb.cos() * (dlon / 2.0).sin() * (dlon / 2.0).sin();
    2.0 * h.sqrt().min(1.0).asin() * RAD_DEG
}

fn arc_km(a_lat: f64, a_lon: f64, b_lat: f64, b_lon: f64) -> f64 {
    arc_deg(a_lat, a_lon, b_lat, b_lon) * std::f64::consts::PI * EARTH_MEAN_RADIUS_M
        / 180.0
        / 1000.0
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / DAY_S).floor() as i64;
    let day_secs = unix.max(0.0) - total as f64 * DAY_S;
    let (y, m, d) = civil_from_days(total);
    let hh = (day_secs / 3600.0) as i64;
    let mm = ((day_secs - hh as f64 * 3600.0) / 60.0) as i64;
    let ss = (day_secs - hh as f64 * 3600.0 - mm as f64 * 60.0) as i64;
    let frac = day_secs - hh as f64 * 3600.0 - mm as f64 * 60.0 - ss as f64;
    format!(
        "{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}.{:03}Z",
        (frac * 1000.0).round() as i64
    )
}

fn run_line(line: &Line, day_tdb: f64, lsk: &LeapSeconds) -> Option<(f64, f64, f64, f64)> {
    let (t_syz, elong) = syzygy(line, day_tdb)?;
    let cov = greatest_coverage(line, day_tdb)?;
    let (t_great, miss) = greatest_time(line, day_tdb)?;
    let pierce = deepest_pierce(line, t_great, None)?;
    let obs = body_fixed_to_icrs_smooth("earth", cov.lat, cov.lon, 0.0, cov.t, &line.map)?;
    let ratio_cov = diameter_ratio_at(line, cov.t, obs)?;
    let obs_p =
        body_fixed_to_icrs_smooth("earth", pierce.lat, pierce.lon, 0.0, t_great, &line.map)?;
    let ratio_p = diameter_ratio_at(line, t_great, obs_p)?;
    let utc_syz = lsk.tdb_to_unix(t_syz)?;
    let utc_great = lsk.tdb_to_unix(t_great)?;
    let utc_cov = lsk.tdb_to_unix(cov.t)?;
    println!(
        "eclipse {} stage1 syzygy: min geocentric elongation {:.3} arcsec at tdb {:.3} = {}",
        line.word,
        elong * 3600.0 * RAD_DEG,
        t_syz,
        iso_utc(utc_syz)
    );
    println!(
        "eclipse {} stage2a greatest eclipse (axis nearest the geocenter): axis miss {:.1} m | lat {:.4} lon {:.4} at tdb {:.3} = {} | magnitude {:.5}",
        line.word,
        miss,
        pierce.lat,
        pierce.lon,
        t_great,
        iso_utc(utc_great),
        ratio_p
    );
    println!(
        "eclipse {} stage2b deepest ground coverage (greatest duration locus): lat {:.4} lon {:.4} at tdb {:.3} = {} | fraction {:.5} magnitude {:.5}",
        line.word,
        cov.lat,
        cov.lon,
        cov.t,
        iso_utc(utc_cov),
        cov.fraction,
        ratio_cov
    );
    Some((t_great, pierce.lat, pierce.lon, ratio_p))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let eph_dir = match arg_value(&args, "--eph-dir") {
        Some(d) => d,
        None => "data".to_string(),
    };
    let day_unix = match arg_value(&args, "--day-unix").and_then(|w| w.parse::<f64>().ok()) {
        Some(u) if u.is_finite() && u > 0.0 => u,
        _ => EVENT_UNIX,
    };
    let is_2017 = (day_unix - EVENT_UNIX).abs() < 1.0;

    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "eclipse: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted"
        );
        return;
    };
    let Some(day_tdb) = lsk.unix_to_tdb(day_unix) else {
        eprintln!("eclipse: the event date reads void on the leap table — the search stays closed");
        return;
    };

    println!("=== eclipse shadow — {} event from the raw sun/moon/earth worldlines (no eclipse catalog enters the search) ===", iso_utc(day_unix));
    println!(
        "event day: tdb {day_tdb:.3} s past J2000 ({}) | lines: de441 (ssd.jpl.nasa.gov), inpop19a (ftp.imcce.fr), epm2021 (ftp.iaaras.ru)",
        iso_utc(day_unix)
    );

    let mut loaded: Vec<Line> = Vec::new();
    for spec in &LINES {
        match load_line(spec, &eph_dir) {
            Some(l) => {
                if covers_event(&l, day_tdb) {
                    loaded.push(l);
                } else {
                    println!(
                        "eclipse {} absent — no granule of the sun/moon/earth line covers the event date",
                        spec.word
                    );
                }
            }
            None => println!("eclipse {} absent — the line is not read", spec.word),
        }
    }

    let mut results: Vec<(String, f64, f64, f64, f64)> = Vec::new();
    for line in &loaded {
        if let Some((t, lat, lon, mag)) = run_line(line, day_tdb, &lsk) {
            results.push((line.word.to_string(), t, lat, lon, mag));
        }
    }

    println!();
    println!("eclipse stage3 — the three worldlines (each line carries its own sun/moon/earth):");
    for (word, t, lat, lon, mag) in &results {
        let utc = match lsk.tdb_to_unix(*t) {
            Some(u) => iso_utc(u),
            None => String::from("absent"),
        };
        println!(
            "eclipse {word}: greatest {} lat {lat:.4} lon {lon:.4} magnitude {mag:.4}",
            utc
        );
    }
    for a in 0..results.len() {
        for b in (a + 1)..results.len() {
            let da = results[a].1 - results[b].1;
            let dkm = arc_km(results[a].2, results[a].3, results[b].2, results[b].3);
            println!(
                "eclipse riss {} vs {}: {:.1} ms time, {:.1} km point",
                results[a].0,
                results[b].0,
                da * 1000.0,
                dkm
            );
        }
    }
    if results.len() >= 2 {
        let t0 = results[0].1;
        let spread = results.iter().map(|r| (r.1 - t0).abs()).fold(0.0, f64::max);
        println!(
            "eclipse rift across {} line(s): {:.1} ms peak time spread — the three worldlines, one shadow, their answers",
            results.len(),
            spread * 1000.0
        );
    }

    println!();
    if !is_2017 {
        println!("=== verdict — the canon constants below describe the 2017 event only; this run is another date, the canon is not laid beside ===");
    } else {
        println!("=== verdict — the catalog laid beside the computation (never inside it) ===");
    }
    let canon_unix = EVENT_UNIX + 18.0 * 3600.0 + 26.0 * 60.0 + 40.0;
    let canon_lat = 36.9667;
    let canon_lon = -87.6717;
    let canon_mag = 1.0306;
    if is_2017 {
        println!(
            "canon (Espenak greatest eclipse 2017): {} lat {canon_lat} lon {canon_lon} magnitude {canon_mag}",
            iso_utc(canon_unix)
        );
    }
    for (word, t, lat, lon, mag) in &results {
        if !is_2017 {
            break;
        }
        match lsk.tdb_to_unix(*t) {
            Some(u) => println!(
                "canon delta {word}: time {} ({:+.1} s) point {:.1} km magnitude {:+.4}",
                iso_utc(u),
                u - canon_unix,
                arc_km(*lat, *lon, canon_lat, canon_lon),
                mag - canon_mag
            ),
            None => println!(
                "canon delta {word}: the tdb-to-unix conversion reads absent — the delta is not computed"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_line() -> Line {
        Line {
            word: "synthetic",
            map: HashMap::new(),
            sun_radius_m: Some(6.957e8),
            moon_radius_m: Some(1.7374e6),
        }
    }

    #[test]
    fn wrap_lon_keeps_the_180_domain() {
        assert_eq!(wrap_lon(190.0), -170.0);
        assert_eq!(wrap_lon(-190.0), 170.0);
        assert_eq!(wrap_lon(87.0), 87.0);
    }

    #[test]
    fn iso_utc_renders_the_event_epoch() {
        assert_eq!(iso_utc(1503273600.0), "2017-08-21T00:00:00.000Z");
    }

    #[test]
    fn great_circle_arc_of_ten_degrees_is_about_1110_km() {
        let km = arc_km(0.0, 0.0, 10.0, 0.0);
        assert!((km - 1111.95).abs() < 1.0, "{km}");
    }

    #[test]
    fn separation_rad_is_zero_for_parallel_vectors() {
        let s = separation_rad([1.0, 0.0, 0.0], [2.0, 0.0, 0.0]).unwrap();
        assert!(s.abs() < 1e-12, "{s}");
    }

    #[test]
    fn axis_miss_equals_the_perpendicular_offset() {
        let sun = [0.0, 0.0, 0.0];
        let moon = [1.5e11, 0.0, 0.0];
        let geo = [1.5e11 + 3.84e8, 100.0, 0.0];
        let miss = axis_geocenter_miss(geo, sun, moon).unwrap();
        assert!((miss - 100.0).abs() < 1e-6, "{miss}");
    }

    #[test]
    fn central_alignment_gives_a_magnitude_above_one() {
        let line = synthetic_line();
        let sun = [1.5e11, 0.0, 0.0];
        let moon = [3.6e8, 0.0, 0.0];
        let obs = [0.0, 0.0, 0.0];
        let theta = separation_rad(vsub(sun, obs), vsub(moon, obs)).unwrap();
        let rs = angular_radius(line.sun_radius_m.unwrap(), vlen(vsub(sun, obs)).unwrap()).unwrap();
        let rm =
            angular_radius(line.moon_radius_m.unwrap(), vlen(vsub(moon, obs)).unwrap()).unwrap();
        let mag = (rs + rm - theta) / (2.0 * rs);
        assert!(mag > 1.0, "central total eclipse magnitude {mag}");
    }

    #[test]
    fn a_grazing_configuration_stays_below_one() {
        let line = synthetic_line();
        let sun = [1.5e11, 0.0, 0.0];
        let rs = angular_radius(line.sun_radius_m.unwrap(), vlen(sun).unwrap()).unwrap();
        let moon = [3.6e8, 0.0, 0.0];
        let rm = angular_radius(line.moon_radius_m.unwrap(), vlen(moon).unwrap()).unwrap();
        let off = rs + rm - 1e-6;
        let moon_off = vunit(moon).unwrap();
        let perp = [0.0, 0.0, 1.0];
        let shifted = [
            moon_off[0] * (3.6e8 * off.cos()) + perp[0] * (3.6e8 * off.sin()),
            moon_off[1] * (3.6e8 * off.cos()) + perp[1] * (3.6e8 * off.sin()),
            moon_off[2] * (3.6e8 * off.cos()) + perp[2] * (3.6e8 * off.sin()),
        ];
        let theta = separation_rad(sun, shifted).unwrap();
        let mag = (rs + rm - theta) / (2.0 * rs);
        assert!(
            mag.is_finite() && (0.0..1.0).contains(&mag),
            "grazing magnitude {mag}"
        );
    }

    fn local_bins_present(eph_dir: &str, netloc: &str, assets: &[&str]) -> bool {
        assets
            .iter()
            .all(|a| std::path::Path::new(&format!("{eph_dir}/{netloc}/{a}")).exists())
    }

    #[test]
    fn kalibrier_gate_de441_places_the_2017_greatest_eclipse_at_the_canon_point() {
        let spec = &LINES[0];
        if !local_bins_present(
            "data",
            spec.netloc,
            &[spec.sun_asset, spec.moon_asset, spec.earth_asset],
        ) {
            return;
        }
        let Some(line) = load_line(spec, "data") else {
            return;
        };
        let Some(lsk) = embedded_lsk() else {
            return;
        };
        let Some(day_tdb) = lsk.unix_to_tdb(EVENT_UNIX) else {
            return;
        };
        let Some((t_great, lat, lon, mag)) = run_line(&line, day_tdb, &lsk) else {
            panic!("kalibrier gate: the de441 line produced no greatest eclipse");
        };
        let _ = t_great;
        let canon_lat = 36.9667;
        let canon_lon = -87.6717;
        let km = arc_km(lat, lon, canon_lat, canon_lon);
        assert!(
            km < 15.0,
            "kalibrier gate: the de441 greatest-eclipse point lies {km:.1} km from the canon point"
        );
        assert!(
            (mag - 1.0306).abs() < 0.002,
            "kalibrier gate: the de441 magnitude {mag:.5} drifts from the canon 1.0306"
        );
    }
}
