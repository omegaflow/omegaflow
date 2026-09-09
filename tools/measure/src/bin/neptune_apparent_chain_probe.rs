use std::collections::HashMap;

use omegaflow::archivar::astrometry::{
    aberration_apply, delta_t_espenak_meeus, diurnal_velocity_icrs, fk4_b1950_to_fk5_j2000,
    frame_bias_fk5_to_icrs, mat3_transpose, mat3_vec, nutation_matrix, parallax_geo_to_topo,
    parallax_topo_to_geo, precession_newcomb_to_b1950, vec3_norm_u, JD_J2000,
};
use omegaflow::archivar::{
    body_barycenter_position, body_barycenter_velocity, body_fixed_to_icrs, light_time_worldline,
    parse_ephemeris_binary, BodyEphemeris,
};

const MAS_PER_RAD: f64 = 206_264_806.247_096_36;
const BIN_TTL_S: u64 = 604800;
const J2000_UNIX_OFFSET: f64 = 946728000.0;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Frame {
    AppTopo,
    AppGeo,
    B1950Geo,
}

#[derive(Clone)]
struct Obs {
    unix: f64,
    delta_t_s: f64,
    ra_deg: f64,
    dec_deg: Option<f64>,
    frame: Frame,
    obs_num: Option<i64>,
}

fn month_num(m: &str) -> Option<i64> {
    match m {
        "Jan" => Some(1),
        "Feb" => Some(2),
        "Mar" => Some(3),
        "Apr" => Some(4),
        "May" => Some(5),
        "Jun" => Some(6),
        "Jul" => Some(7),
        "Aug" => Some(8),
        "Sep" => Some(9),
        "Oct" => Some(10),
        "Nov" => Some(11),
        "Dec" => Some(12),
        _ => None,
    }
}

fn days_from_civil(year: i64, month: i64, day: i64) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146097 + doe - 719468)
}

fn col(line: &str, a: usize, b: usize) -> Option<f64> {
    let bytes = line.as_bytes();
    if b > bytes.len() {
        return None;
    }
    let s = std::str::from_utf8(&bytes[a..b]).ok()?.trim();
    let v: f64 = s.parse().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn col_str(line: &str, a: usize, b: usize) -> Option<&str> {
    let bytes = line.as_bytes();
    if b > bytes.len() {
        return None;
    }
    std::str::from_utf8(&bytes[a..b]).ok().map(|s| s.trim())
}

fn year_decimal(year: i64, month: i64, day: i64) -> Option<f64> {
    let d = days_from_civil(year, month, day)?;
    let jan1 = days_from_civil(year, 1, 1)?;
    Some(year as f64 + ((d - jan1) as f64 + 0.5) / 365.25)
}

fn leading(line: &str) -> Option<(f64, i64, i64, i64, f64, Option<f64>, String)> {
    let year: i64 = col(line, 10, 14)? as i64;
    let day: i64 = col(line, 15, 17)? as i64;
    let month = month_num(col_str(line, 18, 21)?)?;
    let hh = col(line, 22, 24)?;
    let mm = col(line, 25, 27)?;
    let ss = col(line, 28, 35)?;
    let ts = col_str(line, 36, 39)?.to_string();
    let ra_h = col(line, 41, 43)?;
    let ra_m = col(line, 44, 46)?;
    let ra_s = col(line, 47, 55)?;
    let dec_sd = col_str(line, 66, 69)?;
    let (sign, had_sign, deg) = if let Some(rest) = dec_sd.strip_prefix('-') {
        (-1.0, true, rest.trim().parse::<f64>().ok()?)
    } else if let Some(rest) = dec_sd.strip_prefix('+') {
        (1.0, true, rest.trim().parse::<f64>().ok()?)
    } else {
        (1.0, false, dec_sd.trim().parse::<f64>().ok()?)
    };
    let dec_m = col(line, 70, 72)?;
    let dec_s = col(line, 73, 80)?;
    let ra_deg = (ra_h + ra_m / 60.0 + ra_s / 3600.0) * 15.0;
    let dec_absent = !had_sign && deg == 0.0 && dec_m == 0.0 && dec_s == 0.0;
    let dec_deg = if dec_absent {
        None
    } else {
        Some(sign * (deg + dec_m / 60.0 + dec_s / 3600.0))
    };
    let unix = days_from_civil(year, month, day)? as f64 * 86400.0 + hh * 3600.0 + mm * 60.0 + ss;
    Some((unix, year, month, day, ra_deg, dec_deg, ts))
}

fn hilton_code_to_obsnum(code: &str) -> Option<i64> {
    match code {
        "MUSN" | "8USN" | "9USN" | "6USN" => Some(1),
        "CAMB" => Some(6),
        "CAPE" => Some(3),
        "RADC" => Some(11),
        "GREN" => Some(2),
        "GTOK" => Some(4),
        "NICE" => Some(34),
        "PARI" => Some(5),
        "TOUL" => Some(43),
        "UCCL" => Some(7),
        _ => None,
    }
}

fn parse_hilton(line: &str) -> Option<Obs> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 17 || !t.iter().any(|x| *x == "App") {
        return None;
    }
    let year: i64 = t[1].parse().ok()?;
    let day: i64 = t[2].parse().ok()?;
    let month = month_num(t[3])?;
    let hh: f64 = t[4].parse().ok()?;
    let mm: f64 = t[5].parse().ok()?;
    let ss: f64 = t[6].parse().ok()?;
    let ts = t[7];
    let ra_h: f64 = t[8].parse().ok()?;
    let ra_m: f64 = t[9].parse().ok()?;
    let ra_s: f64 = t[10].parse().ok()?;
    let (sign, had_sign, deg) = if let Some(r) = t[12].strip_prefix('-') {
        (-1.0, true, r.parse::<f64>().ok()?)
    } else if let Some(r) = t[12].strip_prefix('+') {
        (1.0, true, r.parse::<f64>().ok()?)
    } else {
        (1.0, false, t[12].parse::<f64>().ok()?)
    };
    let dec_m: f64 = t[13].parse().ok()?;
    let dec_s: f64 = t[14].parse().ok()?;
    let ra_deg = (ra_h + ra_m / 60.0 + ra_s / 3600.0) * 15.0;
    let dec_absent = !had_sign && deg == 0.0 && dec_m == 0.0 && dec_s == 0.0;
    let dec_deg = if dec_absent {
        None
    } else {
        Some(sign * (deg + dec_m / 60.0 + dec_s / 3600.0))
    };
    let unix = days_from_civil(year, month, day)? as f64 * 86400.0 + hh * 3600.0 + mm * 60.0 + ss;
    let delta_t = if ts == "UT1" {
        delta_t_espenak_meeus(year_decimal(year, month, day)?)
    } else {
        0.0
    };
    Some(Obs {
        unix,
        delta_t_s: delta_t,
        ra_deg,
        dec_deg,
        frame: Frame::AppTopo,
        obs_num: hilton_code_to_obsnum(t[16]),
    })
}

fn parse_urss(line: &str, photo: bool) -> Option<Obs> {
    if line.len() < 96 {
        return None;
    }
    let (unix, year, month, day, ra_deg, dec_deg, ts) = leading(line)?;
    let trailing: Vec<&str> = line[92..].split_whitespace().collect();
    if trailing.is_empty() {
        return None;
    }
    let frame = if photo {
        if !trailing.iter().any(|t| *t == "Ast") {
            return None;
        }
        Frame::B1950Geo
    } else {
        if !trailing.iter().any(|t| *t == "App") {
            return None;
        }
        Frame::AppGeo
    };
    let delta_t = if ts == "UT1" {
        delta_t_espenak_meeus(year_decimal(year, month, day)?)
    } else {
        0.0
    };
    Some(Obs {
        unix,
        delta_t_s: delta_t,
        ra_deg,
        dec_deg,
        frame,
        obs_num: None,
    })
}

fn parse_obslist(text: &str) -> HashMap<i64, (f64, f64, f64)> {
    let mut out = HashMap::new();
    for line in text.lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 4 {
            continue;
        }
        let code: i64 = match t[0].parse() {
            Ok(c) => c,
            Err(_) => continue,
        };
        if out.contains_key(&code) {
            continue;
        }
        let geo = t[1];
        if geo.len() < 21 {
            continue;
        }
        let lon_sign = if &geo[2..3] == "-" { -1.0 } else { 1.0 };
        let lon = &geo[3..12];
        let lat_sign = if &geo[12..13] == "-" { -1.0 } else { 1.0 };
        let lat = &geo[13..21];
        let (lon_h, lon_m, lon_s, lat_d, lat_m, lat_s, alt) = {
            let p = |s: &str| s.parse::<f64>().ok();
            let a = p(&lon[0..2]);
            let b = p(&lon[2..4]);
            let c = p(&lon[4..9]).map(|v| v / 1000.0);
            let d = p(&lat[0..2]);
            let e = p(&lat[2..4]);
            let f = p(&lat[4..8]).map(|v| v / 100.0);
            let g = t[2].parse::<f64>().ok();
            match (a, b, c, d, e, f, g) {
                (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g)) => {
                    (a, b, c, d, e, f, g)
                }
                _ => continue,
            }
        };
        let lon_deg = lon_sign * (lon_h + lon_m / 60.0 + lon_s / 3600.0) * 15.0;
        let lat_deg = lat_sign * (lat_d + lat_m / 60.0 + lat_s / 3600.0);
        out.insert(code, (lat_deg, lon_deg, alt));
    }
    out
}

fn icrs_unit(ra_deg: f64, dec_deg: f64) -> [f64; 3] {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let cd = dec.cos();
    [cd * ra.cos(), cd * ra.sin(), dec.sin()]
}

fn unit_to_deg(u: [f64; 3]) -> Option<(f64, f64)> {
    let p = (u[0] * u[0] + u[1] * u[1]).sqrt();
    if !p.is_finite() || p <= 0.0 {
        return None;
    }
    let ra = u[1].atan2(u[0]).to_degrees().rem_euclid(360.0);
    let dec = u[2].atan2(p).to_degrees();
    Some((ra, dec))
}

fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec_len(v: [f64; 3]) -> Option<f64> {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if n.is_finite() && n > 0.0 {
        Some(n)
    } else {
        None
    }
}

fn toward_unit(from: [f64; 3], to: [f64; 3]) -> Option<[f64; 3]> {
    let d = vec_sub(to, from);
    let n = vec_len(d)?;
    Some([d[0] / n, d[1] / n, d[2] / n])
}

fn tangent_basis(ra_deg: f64, dec_deg: f64) -> ([f64; 3], [f64; 3]) {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    (
        [-ra.sin(), ra.cos(), 0.0],
        [-ra.cos() * dec.sin(), -ra.sin() * dec.sin(), dec.cos()],
    )
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn residual_mas(u_obs: [f64; 3], u_geo: [f64; 3]) -> Option<(f64, f64)> {
    let (ra, dec) = unit_to_deg(u_geo)?;
    let (e_ra, e_dec) = tangent_basis(ra, dec);
    let r = vec_sub(u_obs, u_geo);
    Some((dot(r, e_ra) * MAS_PER_RAD, dot(r, e_dec) * MAS_PER_RAD))
}

fn inject_offset(u_geo: [f64; 3], dra_mas: f64, ddec_mas: f64) -> Option<[f64; 3]> {
    let (ra, dec) = unit_to_deg(u_geo)?;
    let (e_ra, e_dec) = tangent_basis(ra, dec);
    let off_ra = dra_mas / MAS_PER_RAD;
    let off_dec = ddec_mas / MAS_PER_RAD;
    let v = [
        u_geo[0] + off_ra * e_ra[0] + off_dec * e_dec[0],
        u_geo[1] + off_ra * e_ra[1] + off_dec * e_dec[1],
        u_geo[2] + off_ra * e_ra[2] + off_dec * e_dec[2],
    ];
    vec3_norm_u(v)
}

fn inverse_chain(
    frame: Frame,
    tdb: f64,
    u_obs: [f64; 3],
    geocenter: [f64; 3],
    obs_minus_geo: [f64; 3],
    v_annual: [f64; 3],
    d: f64,
) -> Option<[f64; 3]> {
    let aoki = fk4_b1950_to_fk5_j2000();
    let bias = frame_bias_fk5_to_icrs();
    let jd = tdb / 86400.0 + JD_J2000;
    let mut u = u_obs;
    match frame {
        Frame::AppTopo => {
            let vd = diurnal_velocity_icrs(obs_minus_geo);
            let vt = [
                v_annual[0] + vd[0],
                v_annual[1] + vd[1],
                v_annual[2] + vd[2],
            ];
            u = aberration_apply(u, vt, false)?;
            u = parallax_topo_to_geo(u, obs_minus_geo, d)?;
        }
        Frame::AppGeo => {
            u = aberration_apply(u, v_annual, false)?;
        }
        Frame::B1950Geo => {}
    }
    let u_mean = if frame == Frame::B1950Geo {
        u
    } else {
        mat3_vec(&nutation_matrix(jd, false), u)
    };
    let u_b1950 = if frame == Frame::B1950Geo {
        u_mean
    } else {
        mat3_vec(&precession_newcomb_to_b1950(jd), u_mean)
    };
    let _ = geocenter;
    Some(mat3_vec(&bias, mat3_vec(&aoki, u_b1950)))
}

fn forward_chain(
    frame: Frame,
    tdb: f64,
    u_icrs: [f64; 3],
    obs_minus_geo: [f64; 3],
    v_annual: [f64; 3],
    d: f64,
) -> Option<[f64; 3]> {
    let aoki = fk4_b1950_to_fk5_j2000();
    let bias = frame_bias_fk5_to_icrs();
    let jd = tdb / 86400.0 + JD_J2000;
    let u_j2000 = mat3_vec(&mat3_transpose(&bias), u_icrs);
    let u_b1950 = mat3_vec(&mat3_transpose(&aoki), u_j2000);
    let u_mean = if frame == Frame::B1950Geo {
        u_b1950
    } else {
        mat3_vec(&mat3_transpose(&precession_newcomb_to_b1950(jd)), u_b1950)
    };
    let u_true = if frame == Frame::B1950Geo {
        u_mean
    } else {
        mat3_vec(&nutation_matrix(jd, true), u_mean)
    };
    let mut u = u_true;
    match frame {
        Frame::AppTopo => {
            let vd = diurnal_velocity_icrs(obs_minus_geo);
            let vt = [
                v_annual[0] + vd[0],
                v_annual[1] + vd[1],
                v_annual[2] + vd[2],
            ];
            u = parallax_geo_to_topo(u, obs_minus_geo, d)?;
            u = aberration_apply(u, vt, true)?;
        }
        Frame::AppGeo => {
            u = aberration_apply(u, v_annual, true)?;
        }
        Frame::B1950Geo => {}
    }
    Some(u)
}

fn ensure_bin(path: &str, netloc: &str, asset: &str, ttl: u64) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("https://github.com/omegaflow/sources/releases/download/{netloc}/{asset}");
    let bytes = omegaflow::archivar::fetch_raw_bytes(&url, ttl)?;
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(path, &bytes).ok()?;
    Some(bytes)
}

fn load(name: &str) -> Option<BodyEphemeris> {
    let path = format!("data/ssd.jpl.nasa.gov/{name}");
    ensure_bin(&path, "ssd.jpl.nasa.gov", name, BIN_TTL_S)
        .and_then(|bytes| parse_ephemeris_binary(&bytes))
}

struct Series {
    path: &'static str,
    frame: Frame,
    name: &'static str,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let calibrate = args.iter().any(|a| a == "--calibrate");
    println!("Neptune apparent-place chain — the 7289 App rows and the Nikolaiev B1950 rows reduced against the DE441 planet center.");

    let Some(center) = load("ephemeris_neptune_c.bin") else {
        eprintln!(
            "neptune-apparent-chain: ephemeris_neptune_c.bin void — the center line is absent"
        );
        return;
    };
    let Some(earth) = load("ephemeris_earth.bin") else {
        eprintln!("neptune-apparent-chain: ephemeris_earth.bin void — the geocenter is absent");
        return;
    };
    let mut center_map = HashMap::new();
    center_map.insert("neptune_c".to_string(), center);
    let mut earth_map = HashMap::new();
    earth_map.insert("earth".to_string(), earth);

    let obslist_text = match std::fs::read("data/www.geoazur.fr/apdb_obslist.opt") {
        Ok(b) => String::from_utf8_lossy(&b).to_string(),
        Err(e) => {
            eprintln!("neptune-apparent-chain: obslist reads void — {e}");
            return;
        }
    };
    let obslist = parse_obslist(&obslist_text);

    let series: Vec<Series> = vec![
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_usno",
            frame: Frame::AppTopo,
            name: "HILTON USNO",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_besa",
            frame: Frame::AppTopo,
            name: "HILTON BESA",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_camb",
            frame: Frame::AppTopo,
            name: "HILTON CAMB",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_cape",
            frame: Frame::AppTopo,
            name: "HILTON CAPE",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_radc",
            frame: Frame::AppTopo,
            name: "HILTON RADC",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_gren",
            frame: Frame::AppTopo,
            name: "HILTON GREN",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_gtok",
            frame: Frame::AppTopo,
            name: "HILTON GTOK",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_nice",
            frame: Frame::AppTopo,
            name: "HILTON NICE",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_pari",
            frame: Frame::AppTopo,
            name: "HILTON PARI",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_stra",
            frame: Frame::AppTopo,
            name: "HILTON STRA",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_toul",
            frame: Frame::AppTopo,
            name: "HILTON TOUL",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_uccl",
            frame: Frame::AppTopo,
            name: "HILTON UCCL",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_usno_urss",
            frame: Frame::AppGeo,
            name: "URSS USNO",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_nik",
            frame: Frame::AppGeo,
            name: "URSS NIK",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_golo",
            frame: Frame::AppGeo,
            name: "URSS GOLO",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_transit_tky",
            frame: Frame::AppGeo,
            name: "URSS TKY",
        },
        Series {
            path: "data/www.geoazur.fr/apdb_neptune_photo_nik",
            frame: Frame::B1950Geo,
            name: "NIK photo B1950",
        },
    ];

    let mut total_dra: Vec<f64> = Vec::new();
    let mut total_ddec: Vec<f64> = Vec::new();
    let mut total_parsed = 0usize;
    let mut total_dec_absent = 0usize;
    let mut total_skipped = 0usize;

    for s in &series {
        let text = match std::fs::read_to_string(s.path) {
            Ok(t) => t,
            Err(_) => {
                println!("{:<18} absent", s.name);
                continue;
            }
        };
        let parse = |line: &str| match s.frame {
            Frame::AppTopo => parse_hilton(line),
            Frame::AppGeo => parse_urss(line, false),
            Frame::B1950Geo => parse_urss(line, true),
        };
        let obs: Vec<Obs> = text.lines().filter_map(parse).collect();
        total_parsed += obs.len();
        let dec_absent = obs.iter().filter(|o| o.dec_deg.is_none()).count();
        total_dec_absent += dec_absent;

        let mut dra = Vec::new();
        let mut ddec = Vec::new();
        let mut skip = 0usize;
        for o in &obs {
            let Some(dec_deg) = o.dec_deg else {
                continue;
            };
            let tdb = o.unix - J2000_UNIX_OFFSET + o.delta_t_s;
            let Some(geocenter) = body_barycenter_position("earth", tdb, &earth_map) else {
                skip += 1;
                continue;
            };
            let worldline = |t: f64| body_barycenter_position("neptune_c", t, &center_map);
            let Some((s_em, _)) = light_time_worldline(geocenter, tdb, &worldline) else {
                skip += 1;
                continue;
            };
            let Some(u_geo) = toward_unit(geocenter, s_em) else {
                skip += 1;
                continue;
            };
            let Some(d) = vec_len(vec_sub(s_em, geocenter)) else {
                skip += 1;
                continue;
            };
            let Some(v_annual) = body_barycenter_velocity("earth", tdb, &earth_map) else {
                skip += 1;
                continue;
            };
            let obs_minus_geo = if o.frame == Frame::AppTopo {
                let Some(n) = o.obs_num else {
                    skip += 1;
                    continue;
                };
                let Some((lat, lon, alt)) = obslist.get(&n) else {
                    skip += 1;
                    continue;
                };
                let Some(obs_pos) = body_fixed_to_icrs("earth", *lat, *lon, *alt, tdb, &earth_map)
                else {
                    skip += 1;
                    continue;
                };
                vec_sub(obs_pos, geocenter)
            } else {
                [0.0, 0.0, 0.0]
            };
            let u_obs = if calibrate {
                let Some(u_app) = forward_chain(o.frame, tdb, u_geo, obs_minus_geo, v_annual, d)
                else {
                    skip += 1;
                    continue;
                };
                match inject_offset(u_app, 40.0, -25.0) {
                    Some(v) => v,
                    None => {
                        skip += 1;
                        continue;
                    }
                }
            } else {
                icrs_unit(o.ra_deg, dec_deg)
            };
            let Some(u_icrs) =
                inverse_chain(o.frame, tdb, u_obs, geocenter, obs_minus_geo, v_annual, d)
            else {
                skip += 1;
                continue;
            };
            let Some((r_ra, r_dec)) = residual_mas(u_icrs, u_geo) else {
                skip += 1;
                continue;
            };
            dra.push(r_ra);
            ddec.push(r_dec);
        }
        total_skipped += skip;
        let n_ra = dra.len();
        let n_dec = ddec.len();
        if n_ra == 0 {
            println!("{:<18} {} rows, 0 reduced (0 honored)", s.name, obs.len());
            continue;
        }
        let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
        let rms = |v: &[f64]| (v.iter().map(|x| x * x).sum::<f64>() / v.len() as f64).sqrt();
        total_dra.extend_from_slice(&dra);
        total_ddec.extend_from_slice(&ddec);
        let m_ra = mean(&dra);
        let m_dec = if n_dec > 0 { mean(&ddec) } else { f64::NAN };
        let r_dec = if n_dec > 0 { rms(&ddec) } else { f64::NAN };
        println!(
            "{:<18} {} rows, dec-absent {}, reduced {} RA / {} Dec, mean ΔRA·cosδ {:+.1}, mean ΔDec {:+.1}, RMS {:+.1} / {:+.1}",
            s.name, obs.len(), dec_absent, n_ra, n_dec, m_ra, m_dec, rms(&dra), r_dec
        );
    }

    if total_dra.is_empty() {
        eprintln!("neptune-apparent-chain: no reduced rows (0 honored)");
        return;
    }
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let rms = |v: &[f64]| (v.iter().map(|x| x * x).sum::<f64>() / v.len() as f64).sqrt();
    println!(
        "TOTAL: {} parsed, {} dec-absent, {} skipped | mean ΔRA·cosδ {:+.1} mas, mean ΔDec {:+.1} mas, RMS {:+.1} / {:+.1} mas",
        total_parsed, total_dec_absent, total_skipped, mean(&total_dra), mean(&total_ddec), rms(&total_dra), rms(&total_ddec)
    );
    if calibrate {
        println!(
            "calibration gate: injected +40 mas RA / -25 mas Dec; recovered mean {:+.3} / {:+.3} mas",
            mean(&total_dra), mean(&total_ddec)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hilton_reads_one_usno_row() {
        let line = "Neptune   1969 26 Jun   2 30  6.3360  ET   15 38  1.74800  0.03330  -17 38 59.5000  0.5000   6USN          6\" Transit 198.0  Trans    App                57,58";
        let o = parse_hilton(line).expect("row parses");
        assert!((o.ra_deg - (15.0 + 38.0 / 60.0 + 1.74800 / 3600.0) * 15.0).abs() < 1e-6);
        let d = o.dec_deg.unwrap();
        assert!((d - (-17.0 - 38.0 / 60.0 - 59.5000 / 3600.0)).abs() < 1e-6);
        assert!(o.frame == Frame::AppTopo);
        assert_eq!(o.obs_num, Some(1));
        assert_eq!(o.delta_t_s, 0.0);
    }

    #[test]
    fn parse_hilton_marks_the_pari_dec_placeholder_absent() {
        let line = "Neptune   1938 10 May   7 52 49.0900  ET   11 19 24.18000  0.06670    0  0  0.0000  1.0000   PARI    various transits   0.0  Trans    App                   35";
        let o = parse_hilton(line).expect("row parses");
        assert!(o.dec_deg.is_none());
        assert_eq!(o.obs_num, Some(5));
    }

    #[test]
    fn parse_urss_reads_one_app_geo_row() {
        let line = " Neptune  1866-27-Jul  9:38:27.9565 UT1   0 49 33.47100  0.90000  + 3 36 33.6900 -0.6500  786 Trans App True  Geo FK4 GC GC XX  14- 14";
        let o = parse_urss(line, false).expect("row parses");
        assert!(o.frame == Frame::AppGeo);
        assert!((o.ra_deg - (0.0 + 49.0 / 60.0 + 33.47100 / 3600.0) * 15.0).abs() < 1e-6);
        let d = o.dec_deg.unwrap();
        assert!((d - (3.0 + 36.0 / 60.0 + 33.6900 / 3600.0)).abs() < 1e-6);
        assert!(o.delta_t_s > 0.0);
    }

    #[test]
    fn parse_urss_reads_one_b1950_photo_row() {
        let line = " Neptune  1961- 6-Apr 23:18:23.4824 ET   14 34 19.78300 -0.33000  -13 12 35.3200  0.0500   89 Photo Ast B1950 Geo FK4 GC GC XX  68- 68";
        let o = parse_urss(line, true).expect("row parses");
        assert!(o.frame == Frame::B1950Geo);
        let d = o.dec_deg.unwrap();
        assert!((d - (-13.0 - 12.0 / 60.0 - 35.3200 / 3600.0)).abs() < 1e-6);
    }

    #[test]
    fn obslist_parses_the_usno_position() {
        let line = "001 01-050814860+38533925   82 Washington U.S.Naval obs Tr+MuC 845-865   0   0";
        let map = parse_obslist(line);
        let (lat, lon, alt) = map[&1];
        assert!((lon - (-77.062)).abs() < 0.001);
        assert!((lat - 38.894).abs() < 0.001);
        assert!((alt - 82.0).abs() < 0.1);
    }
}
