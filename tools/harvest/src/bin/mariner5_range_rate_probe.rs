use std::collections::HashMap;
use std::env;
use std::fs;

use omegaflow::archivar::BodyEphemeris;
use omegaflow::archivar::BodyProperties;
use omegaflow::archivar::cdn::{body_ephemeris, body_props};
use omegaflow::archivar::motion::{
    body_barycenter_position, body_barycenter_velocity, orientation_angles_at,
};

const AU_KM: f64 = 1.495_978_707e8;
const MU_VENUS_KM3_S2: f64 = 324_858.6;
const C_KM_S: f64 = 299_792.458;
const DOPPLER_HZ: f64 = 2_295e6;
const J2000_JD: f64 = 2_451_545.0;
const UNIX_JD: f64 = 2_440_587.5;
const DEG: f64 = std::f64::consts::PI / 180.0;
const TAU: f64 = std::f64::consts::TAU;
const LIGHT_RANGE_AU: f64 = 0.5332;
const METRES_PER_KM: f64 = 1_000.0;

#[derive(Clone, Copy)]
struct Elements {
    a: f64,
    e: f64,
    incl: f64,
    mean_long: f64,
    peri_long: f64,
    node: f64,
}

const SOLAR_THIRD_ELEMENTS: Elements = Elements {
    a: 1.00000261,
    e: 0.01671123,
    incl: -0.00001531,
    mean_long: 100.46457166,
    peri_long: 102.93768193,
    node: 0.0,
};

const SOLAR_THIRD_RATES: Elements = Elements {
    a: 0.00000562,
    e: -0.00004392,
    incl: -0.01294668,
    mean_long: 35999.37244981,
    peri_long: 0.32327364,
    node: 0.0,
};

const INNER_ELEMENTS: Elements = Elements {
    a: 0.72333566,
    e: 0.00677672,
    incl: 3.39467605,
    mean_long: 181.97909950,
    peri_long: 131.60246718,
    node: 76.67984255,
};

const INNER_RATES: Elements = Elements {
    a: 0.00000390,
    e: -0.00004107,
    incl: -0.00078890,
    mean_long: 58517.81538729,
    peri_long: 0.00268329,
    node: -0.27769418,
};

#[derive(Clone, Copy)]
struct Hyperbola {
    a: f64,
    e: f64,
    incl_deg: f64,
    node_deg: f64,
    peri_deg: f64,
    t_peri_unix: f64,
}

#[derive(Clone, Copy)]
struct Station {
    lat_deg: f64,
    lon_deg: f64,
}

#[derive(Clone, Copy)]
struct BodySpin {
    radius_m: f64,
    flattening: f64,
    rotation_rad_s: f64,
    pole: [f64; 3],
    node: [f64; 3],
    cross: [f64; 3],
    w0_deg: f64,
    dw_dt_deg_per_day: f64,
}

#[derive(Clone, Copy)]
struct Stats {
    n: usize,
    mean: f64,
    scatter: f64,
}

fn unix_from_civil(year: i64, month: i64, day: i64, hour: i64, minute: i64, second: f64) -> f64 {
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;
    let jdn = day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    (jdn as f64 - 2_440_588.0) * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second
}

fn mariner5_encounter() -> Hyperbola {
    Hyperbola {
        a: -34_824.285,
        e: 1.2914715,
        incl_deg: 32.001963,
        node_deg: 341.30976,
        peri_deg: 152.82175,
        t_peri_unix: unix_from_civil(1967, 10, 19, 17, 34, 55.841),
    }
}

fn rot_z(angle: f64, v: [f64; 3]) -> [f64; 3] {
    let (s, c) = angle.sin_cos();
    [c * v[0] - s * v[1], s * v[0] + c * v[1], v[2]]
}

fn rot_x(angle: f64, v: [f64; 3]) -> [f64; 3] {
    let (s, c) = angle.sin_cos();
    [v[0], c * v[1] - s * v[2], s * v[1] + c * v[2]]
}

fn rotate(v: [f64; 3], incl_deg: f64, node_deg: f64, peri_deg: f64) -> [f64; 3] {
    rot_z(node_deg * DEG, rot_x(incl_deg * DEG, rot_z(peri_deg * DEG, v)))
}

fn planet_position(el: Elements, rate: Elements, jd: f64) -> [f64; 3] {
    let t = (jd - J2000_JD) / 36525.0;
    let a = el.a + rate.a * t;
    let e = el.e + rate.e * t;
    let incl = (el.incl + rate.incl * t) * DEG;
    let mean_long = (el.mean_long + rate.mean_long * t) * DEG;
    let peri_long = (el.peri_long + rate.peri_long * t) * DEG;
    let node = (el.node + rate.node * t) * DEG;
    let m = (mean_long - peri_long + std::f64::consts::PI).rem_euclid(TAU) - std::f64::consts::PI;
    let mut ecc = m + e * m.sin();
    for _ in 0..12 {
        ecc -= (ecc - e * ecc.sin() - m) / (1.0 - e * ecc.cos());
    }
    let xp = a * (ecc.cos() - e);
    let yp = a * (1.0 - e * e).sqrt() * ecc.sin();
    let arg = peri_long - node;
    let (so, co) = node.sin_cos();
    let (si, ci) = incl.sin_cos();
    let (sa, ca) = arg.sin_cos();
    let x = (ca * co - sa * so * ci) * xp + (-sa * co - ca * so * ci) * yp;
    let y = (ca * so + sa * co * ci) * xp + (-sa * so + ca * co * ci) * yp;
    let z = (sa * si) * xp + (ca * si) * yp;
    [x * AU_KM, y * AU_KM, z * AU_KM]
}

fn planet_velocity(el: Elements, rate: Elements, jd: f64) -> [f64; 3] {
    let step = 0.5;
    let before = planet_position(el, rate, jd - step);
    let after = planet_position(el, rate, jd + step);
    let span = 2.0 * step * 86400.0;
    [
        (after[0] - before[0]) / span,
        (after[1] - before[1]) / span,
        (after[2] - before[2]) / span,
    ]
}

fn hyperbolic_state(orbit: Hyperbola, t_unix: f64) -> ([f64; 3], [f64; 3]) {
    let p = orbit.a * (1.0 - orbit.e * orbit.e);
    let n = (MU_VENUS_KM3_S2 / orbit.a.abs().powi(3)).sqrt();
    let mean_anomaly = n * (t_unix - orbit.t_peri_unix);
    let mut h = (mean_anomaly / orbit.e).asinh();
    for _ in 0..120 {
        h -= (orbit.e * h.sinh() - h - mean_anomaly) / (orbit.e * h.cosh() - 1.0);
    }
    let nu = 2.0 * (((orbit.e + 1.0) / (orbit.e - 1.0)).sqrt() * (h / 2.0).tanh()).atan();
    let r = p / (1.0 + orbit.e * nu.cos());
    let k = (MU_VENUS_KM3_S2 / p).sqrt();
    let pos = rotate(
        [r * nu.cos(), r * nu.sin(), 0.0],
        orbit.incl_deg,
        orbit.node_deg,
        orbit.peri_deg,
    );
    let vel = rotate(
        [-k * nu.sin(), k * (orbit.e + nu.cos()), 0.0],
        orbit.incl_deg,
        orbit.node_deg,
        orbit.peri_deg,
    );
    (pos, vel)
}

fn body_spin(props: &BodyProperties) -> Result<BodySpin, String> {
    let flattening = props
        .flattening
        .ok_or_else(|| "the body record carries no flattening".to_string())?;
    let (alpha0, delta0, w0) = orientation_angles_at(props, J2000_JD)
        .ok_or_else(|| "the body record carries no orientation at J2000".to_string())?;
    let (_, _, w1) = orientation_angles_at(props, J2000_JD + 1.0)
        .ok_or_else(|| "the body record carries no orientation one day on".to_string())?;
    let dw_dt_deg_per_day = w1 - w0;
    let (sa, ca) = (alpha0 * DEG).sin_cos();
    let (sd, cd) = (delta0 * DEG).sin_cos();
    Ok(BodySpin {
        radius_m: props.radius_m,
        flattening,
        rotation_rad_s: dw_dt_deg_per_day * DEG / 86400.0,
        pole: [cd * ca, cd * sa, sd],
        node: [-sa, ca, 0.0],
        cross: [-sd * ca, -sd * sa, cd],
        w0_deg: w0,
        dw_dt_deg_per_day,
    })
}

fn station_state(jd: f64, station: Station, spin: BodySpin) -> ([f64; 3], [f64; 3]) {
    let w = (spin.w0_deg + spin.dw_dt_deg_per_day * (jd - J2000_JD)) * DEG;
    let (sw, cw) = w.sin_cos();
    let mut prime = [0.0; 3];
    let mut side = [0.0; 3];
    for i in 0..3 {
        prime[i] = cw * spin.node[i] + sw * spin.cross[i];
        side[i] = -sw * spin.node[i] + cw * spin.cross[i];
    }
    let e2 = 2.0 * spin.flattening - spin.flattening * spin.flattening;
    let (lat_s, lat_c) = (station.lat_deg * DEG).sin_cos();
    let (lon_s, lon_c) = (station.lon_deg * DEG).sin_cos();
    let vertical = spin.radius_m / 1_000.0 / (1.0 - e2 * lat_s * lat_s).sqrt();
    let mut r = [0.0; 3];
    for i in 0..3 {
        r[i] = vertical * lat_c * (lon_c * prime[i] + lon_s * side[i])
            + vertical * (1.0 - e2) * lat_s * spin.pole[i];
    }
    let omega = spin.rotation_rad_s;
    let v = [
        omega * (spin.pole[1] * r[2] - spin.pole[2] * r[1]),
        omega * (spin.pole[2] * r[0] - spin.pole[0] * r[2]),
        omega * (spin.pole[0] * r[1] - spin.pole[1] * r[0]),
    ];
    (r, v)
}

fn doppler_scale_cps_per_km_s() -> f64 {
    2.0 * DOPPLER_HZ / C_KM_S
}

struct Paths {
    chebyshev: Option<(String, String, HashMap<String, BodyEphemeris>)>,
}

impl Paths {
    fn analytic() -> Paths {
        Paths { chebyshev: None }
    }

    fn carrier_position(&self, jd: f64) -> Result<[f64; 3], String> {
        match &self.chebyshev {
            None => Ok(planet_position(SOLAR_THIRD_ELEMENTS, SOLAR_THIRD_RATES, jd)),
            Some((carrier, _, eph)) => {
                body_barycenter_position(carrier, (jd - J2000_JD) * 86400.0, eph)
                    .map(|v| [v[0] / METRES_PER_KM, v[1] / METRES_PER_KM, v[2] / METRES_PER_KM])
                    .ok_or_else(|| format!("{carrier} carries no position at jd {jd:.6}"))
            }
        }
    }

    fn carrier_velocity(&self, jd: f64) -> Result<[f64; 3], String> {
        match &self.chebyshev {
            None => Ok(planet_velocity(SOLAR_THIRD_ELEMENTS, SOLAR_THIRD_RATES, jd)),
            Some((carrier, _, eph)) => {
                body_barycenter_velocity(carrier, (jd - J2000_JD) * 86400.0, eph)
                    .map(|v| [v[0] / METRES_PER_KM, v[1] / METRES_PER_KM, v[2] / METRES_PER_KM])
                    .ok_or_else(|| format!("{carrier} carries no velocity at jd {jd:.6}"))
            }
        }
    }

    fn target_position(&self, jd: f64) -> Result<[f64; 3], String> {
        match &self.chebyshev {
            None => Ok(planet_position(INNER_ELEMENTS, INNER_RATES, jd)),
            Some((_, target, eph)) => {
                body_barycenter_position(target, (jd - J2000_JD) * 86400.0, eph)
                    .map(|v| [v[0] / METRES_PER_KM, v[1] / METRES_PER_KM, v[2] / METRES_PER_KM])
                    .ok_or_else(|| format!("{target} carries no position at jd {jd:.6}"))
            }
        }
    }

    fn target_velocity(&self, jd: f64) -> Result<[f64; 3], String> {
        match &self.chebyshev {
            None => Ok(planet_velocity(INNER_ELEMENTS, INNER_RATES, jd)),
            Some((_, target, eph)) => {
                body_barycenter_velocity(target, (jd - J2000_JD) * 86400.0, eph)
                    .map(|v| [v[0] / METRES_PER_KM, v[1] / METRES_PER_KM, v[2] / METRES_PER_KM])
                    .ok_or_else(|| format!("{target} carries no velocity at jd {jd:.6}"))
            }
        }
    }
}

fn range_rate_cps(
    t_unix: f64,
    orbit: Hyperbola,
    station: Station,
    spin: BodySpin,
    paths: &Paths,
) -> Result<f64, String> {
    let jd = t_unix / 86400.0 + UNIX_JD;
    let carrier_pos = paths.carrier_position(jd)?;
    let carrier_vel = paths.carrier_velocity(jd)?;
    let (st_pos, st_vel) = station_state(jd, station, spin);
    let mut light_time = LIGHT_RANGE_AU * AU_KM / C_KM_S;
    let mut line = [0.0; 3];
    let mut rel = [0.0; 3];
    let mut range = 0.0;
    for _ in 0..3 {
        let jd_craft = jd - light_time / 86400.0;
        let target_pos = paths.target_position(jd_craft)?;
        let target_vel = paths.target_velocity(jd_craft)?;
        let (craft_pos, craft_vel) = hyperbolic_state(orbit, t_unix - light_time);
        for i in 0..3 {
            line[i] = target_pos[i] + craft_pos[i] - carrier_pos[i] - st_pos[i];
            rel[i] = target_vel[i] + craft_vel[i] - carrier_vel[i] - st_vel[i];
        }
        range = (line[0] * line[0] + line[1] * line[1] + line[2] * line[2]).sqrt();
        light_time = range / C_KM_S;
    }
    let unit = [line[0] / range, line[1] / range, line[2] / range];
    let rate = unit[0] * rel[0] + unit[1] * rel[1] + unit[2] * rel[2];
    Ok(doppler_scale_cps_per_km_s() * rate)
}

struct Row {
    iso: String,
    t_unix: f64,
    segment: String,
    observed: f64,
}

fn parse_rows(text: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    for line in text.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 6 {
            continue;
        }
        let t_unix = cols[1].parse::<f64>();
        let observed = cols[5].parse::<f64>();
        if let (Ok(t_unix), Ok(observed)) = (t_unix, observed) {
            rows.push(Row {
                iso: cols[2].to_string(),
                t_unix,
                segment: cols[3].to_string(),
                observed,
            });
        }
    }
    rows
}

fn stats(values: &[f64]) -> Stats {
    if values.is_empty() {
        return Stats { n: 0, mean: f64::NAN, scatter: f64::NAN };
    }
    let n = values.len();
    let mean = values.iter().sum::<f64>() / n as f64;
    let scatter = (values.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n as f64).sqrt();
    Stats { n, mean, scatter }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let usage = "usage: mariner5_range_rate_probe <wings.csv> <body> <lat> <lon> [<lat> <lon> ...] [--rows] [--ephemeris <carrier> <target>]";
    let mut positional: Vec<String> = Vec::new();
    let mut ephemeris: Option<(String, String)> = None;
    let mut every_row = false;
    let mut ephemeris_probe = false;
    let mut scan = 0;
    while scan < args.len() {
        match args[scan].as_str() {
            "--rows" => {
                every_row = true;
                scan += 1;
            }
            "--ephemeris-probe" => {
                ephemeris_probe = true;
                scan += 1;
            }
            "--ephemeris" => {
                let carrier = args.get(scan + 1).ok_or_else(|| usage.to_string())?;
                let target = args.get(scan + 2).ok_or_else(|| usage.to_string())?;
                ephemeris = Some((carrier.clone(), target.clone()));
                scan += 3;
            }
            other => {
                positional.push(other.to_string());
                scan += 1;
            }
        }
    }
    let path = positional.first().ok_or_else(|| usage.to_string())?;
    let body = positional.get(1).ok_or_else(|| usage.to_string())?;
    let rest = positional.get(2..).ok_or_else(|| usage.to_string())?;
    let mut stations = Vec::new();
    let mut index = 0;
    while index + 1 < rest.len() {
        let lat_deg = rest[index]
            .parse::<f64>()
            .map_err(|_| format!("latitude {} carries no number", rest[index]))?;
        let lon_deg = rest[index + 1]
            .parse::<f64>()
            .map_err(|_| format!("longitude {} carries no number", rest[index + 1]))?;
        stations.push(Station { lat_deg, lon_deg });
        index += 2;
    }
    if stations.is_empty() {
        return Err(usage.to_string());
    }
    let text = fs::read_to_string(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let rows = parse_rows(&text);
    if rows.is_empty() {
        return Err(format!("{path} carries no wing rows"));
    }
    let props = body_props(body)?;
    let spin = body_spin(&props)?;
    let paths = match ephemeris {
        None => Paths::analytic(),
        Some((carrier, target)) => {
            let mut eph = HashMap::new();
            eph.insert(carrier.clone(), body_ephemeris(&carrier)?);
            eph.insert(target.clone(), body_ephemeris(&target)?);
            println!(
                "paths {carrier} and {target} from the ephemeris binaries, {} and {} granules",
                eph[&carrier].granules.len(),
                eph[&target].granules.len()
            );
            Paths { chebyshev: Some((carrier, target, eph)) }
        }
    };
    let orbit = mariner5_encounter();
    if ephemeris_probe {
        let jd = rows[0].t_unix / 86400.0 + UNIX_JD;
        let p = paths.carrier_position(jd)?;
        let v = paths.carrier_velocity(jd)?;
        let t = paths.target_position(jd)?;
        let tv = paths.target_velocity(jd)?;
        println!(
            "carrier |r| {:.3e} |v| {:.3e}   target |r| {:.3e} |v| {:.3e}",
            (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt(),
            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(),
            (t[0] * t[0] + t[1] * t[1] + t[2] * t[2]).sqrt(),
            (tv[0] * tv[0] + tv[1] * tv[1] + tv[2] * tv[2]).sqrt()
        );
    }
    println!(
        "rows {}  scale {:.1} cps per km/s  body {body}  radius {:.0} m  flattening {:.8}  rotation {:.6} deg/day",
        rows.len(),
        doppler_scale_cps_per_km_s(),
        spin.radius_m,
        spin.flattening,
        spin.rotation_rad_s / DEG * 86400.0
    );
    println!(
        "orbit a {:.3} km  e {:.7}  t_peri {:.3}",
        orbit.a, orbit.e, orbit.t_peri_unix
    );
    for station in &stations {
        let mut pre = Vec::new();
        let mut post = Vec::new();
        for row in &rows {
            let residual = row.observed - range_rate_cps(row.t_unix, orbit, *station, spin, &paths)?;
            if row.segment == "pre" {
                pre.push(residual);
            } else {
                post.push(residual);
            }
        }
        let a = stats(&pre);
        let b = stats(&post);
        println!(
            "station {:.2} {:.2}  pre n={} mean {:.0} scatter {:.0}  post n={} mean {:.0} scatter {:.0}  step {:.0} cps",
            station.lat_deg,
            station.lon_deg,
            a.n,
            a.mean,
            a.scatter,
            b.n,
            b.mean,
            b.scatter,
            b.mean - a.mean
        );
    }
    let station = stations[0];
        if every_row {
        println!("idx,iso,seg,observed,model,residual");
        for (index, row) in rows.iter().enumerate() {
            let model = range_rate_cps(row.t_unix, orbit, station, spin, &paths)?;
            println!(
                "{},{},{},{:.3},{:.3},{:.3}",
                index,
                row.iso,
                row.segment,
                row.observed,
                model,
                row.observed - model
            );
        }
        return Ok(());
    }
    println!();
    println!("samples for station {:.2} {:.2}:", station.lat_deg, station.lon_deg);
    for (index, row) in rows.iter().enumerate() {
        if index % 30 != 0 && index + 1 != rows.len() {
            continue;
        }
        let model = range_rate_cps(row.t_unix, orbit, station, spin, &paths)?;
        println!(
            "  {} {} observed {:.0} model {:.0} residual {:.0}",
            row.iso,
            row.segment,
            row.observed,
            model,
            row.observed - model
        );
    }
    Ok(())
}

#[cfg(test)]
#[path = "mariner5_range_rate_probe/tests.rs"]
mod tests;
