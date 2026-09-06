use omegaflow::archivar::{
    angular_distance_deg, cache_root, fetch_raw, load_sources, parse_json, source_name_from_url,
    Extract, JsonVal, SourceConfig,
};
use omegaflow::intermagnet;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const GROUND_FIELDS: &str = "intermagnet_xyz_x_nt+intermagnet_xyz_y_nt+intermagnet_xyz_z_nt";
const SWARM_FIELD: &str = "swarm_magnetic_field_intensity_nt";
const GROUND_NETLOC: &str = "imag-data.bgs.ac.uk";
const SWARM_NETLOC: &str = "vires.services";
const GROUND_DATASET: &str = "ABK";
const SWARM_NEEDLE: &str = "SW_FAST_MAGA_LR_1B";
const ABK_HAPI: &str = "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id={station}/best-avail/PT1M/xyzf&start={start}&stop={stop}&format=json";
const SWARM_HAPI: &str = "https://vires.services/hapi/data?id=SW_FAST_MAGA_LR_1B&start={start}&stop={stop}&parameters=Latitude,Longitude,F&format=json";
const FILL_NT: f64 = 99999.0;
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const DEFAULT_RADIUS_DEG: f64 = 5.0;
const ENCOUNTER_MIN_LIMIT: f64 = 3.0 * MINUTE;
const GROUND_SIGMA_WINDOW_S: f64 = 30.0 * MINUTE;
const SWARM_SIGMA_WINDOW_S: f64 = 60.0;
const SIGMA_K: f64 = 2.0;

#[derive(Clone)]
struct Row {
    t: f64,
    f: f64,
    lat: Option<f64>,
    lon: Option<f64>,
}

struct Encounter {
    epoch: f64,
    distance_deg: f64,
    ground_f: f64,
    swarm_f: f64,
    ground_sigma: f64,
    swarm_sigma: f64,
}

enum Verdict {
    Placed,
    Absent,
    Riss,
}

fn now_unix() -> Option<f64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs_f64())
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else {
        s.split_once(' ')?
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let ss: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * HOUR + mm as f64 * MINUTE + ss as f64)
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / DAY).floor() as i64;
    let day_secs = unix.max(0.0) - total as f64 * DAY;
    let z = total + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let hh = (day_secs / HOUR) as i64;
    let mm = ((day_secs - hh as f64 * HOUR) / MINUTE) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * MINUTE) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn arg_f64(args: &[String], name: &str) -> Option<f64> {
    arg_after(args, name).and_then(|v| v.parse::<f64>().ok())
}

fn usage() {
    println!("usage: station_convergence_probe [--station ABK] [--lat f64 --lon f64] [--radius deg] [--tolerance-nT f64] [--live] [--start ISO] [--stop ISO] [--data-dir DIR] [--grammar]");
    println!("default: offline over the flowing fanout-ring caches under the archivar cache dir; --live fetches the two HAPI lines (INTERMAGNET ground xyzf, SWARM scalar F with Latitude/Longitude).");
}

fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().sum::<f64>() / xs.len() as f64)
    }
}

fn std(xs: &[f64]) -> Option<f64> {
    if xs.len() < 2 {
        return None;
    }
    let m = mean(xs)?;
    let v = xs.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / xs.len() as f64;
    Some(v.sqrt())
}

fn source_carries(src: &SourceConfig, field: &str) -> bool {
    src.extracts.iter().any(|e| match e {
        Extract::Field(fc)
        | Extract::First(fc, _)
        | Extract::Last(fc, _)
        | Extract::Count(fc)
        | Extract::LastRow(fc)
        | Extract::ObjLast(fc)
        | Extract::Path(fc)
        | Extract::Deep(fc)
        | Extract::Regex(fc) => fc.name == field,
        Extract::Hapi(pairs) => pairs.iter().any(|(_, n)| n == field),
        _ => false,
    })
}

fn register_station_point(sources: &[SourceConfig]) -> Option<(f64, f64)> {
    for src in sources {
        if source_carries(src, "intermagnet_xyz_x_nt") {
            if let omegaflow::archivar::Frame::Surface { lat, lon, .. } = src.frame {
                return Some((lat, lon));
            }
        }
    }
    None
}

fn parse_vector_rows(text: &str) -> Vec<Row> {
    let Some(json) = parse_json(text) else {
        return Vec::new();
    };
    let JsonVal::Obj(map) = json else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(data)) = map.get("data") else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        if cells.len() < 2 {
            continue;
        }
        let Some(t) = (match &cells[0] {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let JsonVal::Arr(comps) = &cells[1] else {
            continue;
        };
        if comps.len() < 3 {
            continue;
        }
        let mut f = 0.0;
        let mut ok = true;
        for c in comps.iter().take(3) {
            match c {
                JsonVal::Num(n) if n.is_finite() && *n != FILL_NT => {
                    f += n * n;
                }
                _ => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let mag = f.sqrt();
        if mag.is_finite() && mag > 0.0 {
            rows.push(Row {
                t,
                f: mag,
                lat: None,
                lon: None,
            });
        }
    }
    rows.sort_by(|a, b| a.t.total_cmp(&b.t));
    rows
}

fn parse_scalar_rows(text: &str) -> Vec<Row> {
    let Some(json) = parse_json(text) else {
        return Vec::new();
    };
    let JsonVal::Obj(map) = json else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(data)) = map.get("data") else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        if cells.len() < 2 {
            continue;
        }
        let Some(t) = (match &cells[0] {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let nums: Vec<f64> = cells[1..]
            .iter()
            .filter_map(|c| match c {
                JsonVal::Num(n) if n.is_finite() => Some(*n),
                _ => None,
            })
            .collect();
        let row = match nums.len() {
            1 => Row {
                t,
                f: nums[0],
                lat: None,
                lon: None,
            },
            3 => Row {
                t,
                f: nums[2],
                lat: Some(nums[0]),
                lon: Some(nums[1]),
            },
            _ => continue,
        };
        if row.f.is_finite() && row.f != FILL_NT {
            rows.push(row);
        }
    }
    rows.sort_by(|a, b| a.t.total_cmp(&b.t));
    rows
}

fn newest_cache_file(dir: &PathBuf, needle: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut found: Vec<(u128, PathBuf)> = Vec::new();
    for e in entries.flatten() {
        let p = e.path();
        let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !name.contains(needle) || !name.ends_with(".json") {
            continue;
        }
        let m = match e.metadata().and_then(|md| md.modified()) {
            Ok(m) => m,
            Err(_) => continue,
        };
        let key = match m.duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_millis(),
            Err(_) => continue,
        };
        found.push((key, p));
    }
    found.sort_by(|a, b| b.0.cmp(&a.0));
    found.first().map(|(_, p)| p.clone())
}

fn cache_dir_for(netloc: &str) -> PathBuf {
    cache_root().join(netloc)
}

fn ground_fetch_url(station: &str, start: &str, stop: &str) -> String {
    ABK_HAPI
        .replace("{station}", station)
        .replace("{start}", start)
        .replace("{stop}", stop)
}

fn swarm_fetch_url(start: &str, stop: &str) -> String {
    SWARM_HAPI.replace("{start}", start).replace("{stop}", stop)
}

fn read_json_file(path: &PathBuf) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn write_json_cache(netloc: &str, url: &str, text: &str) -> PathBuf {
    let dir = cache_dir_for(netloc);
    let _ = std::fs::create_dir_all(&dir);
    let name = source_name_from_url(url);
    let path = dir.join(format!("{name}.json"));
    let _ = std::fs::write(&path, text);
    path
}

fn load_ground_offline(station: &str) -> (Vec<Row>, String) {
    let dir = cache_dir_for(GROUND_NETLOC);
    let needle = format!("{}-best", station);
    match newest_cache_file(&dir, &needle) {
        Some(p) => match read_json_file(&p) {
            Some(text) => {
                let rows = parse_vector_rows(&text);
                if rows.is_empty() {
                    (
                        rows,
                        format!("ground bin {} reads no vector rows", p.display()),
                    )
                } else {
                    let note = format!(
                        "ground ring cache {} carries {} minute sample(s)",
                        p.display(),
                        rows.len()
                    );
                    (rows, note)
                }
            }
            None => (Vec::new(), format!("ground bin {} reads void", p.display())),
        },
        None => (
            Vec::new(),
            format!(
                "ground bin void — {} holds no {station} xyzf ring cache",
                dir.display()
            ),
        ),
    }
}

fn load_ground_derived_bin(station: &str, data_dir: &PathBuf) -> Option<String> {
    let code = station.to_lowercase();
    let bin = data_dir
        .join(GROUND_NETLOC)
        .join(format!("{code}_dbdt_1h.bin"));
    let bytes = std::fs::read(&bin).ok()?;
    let recs = intermagnet::parse_bin(&bytes)?;
    Some(format!(
        "local bin {} reads {} hour(s) of the derived |dB/dt| — not the intermagnet_xyz B magnitude, the convergence quantity stays absent",
        bin.display(),
        recs.len()
    ))
}

fn load_swarm_offline() -> (Vec<Row>, String) {
    let dir = cache_dir_for(SWARM_NETLOC);
    match newest_cache_file(&dir, SWARM_NEEDLE) {
        Some(p) => match read_json_file(&p) {
            Some(text) => {
                let rows = parse_scalar_rows(&text);
                if rows.is_empty() {
                    (
                        rows,
                        format!("swarm cache {} reads no scalar rows", p.display()),
                    )
                } else {
                    let geo = rows.iter().any(|r| r.lat.is_some() && r.lon.is_some());
                    let geom = if geo {
                        "carries Latitude/Longitude"
                    } else {
                        "carries time+F only — the station-point geometry is not in the cache"
                    };
                    let note = format!(
                        "swarm ring cache {} holds {} sample(s), {geom}",
                        p.display(),
                        rows.len()
                    );
                    (rows, note)
                }
            }
            None => (
                Vec::new(),
                format!("swarm cache {} reads void", p.display()),
            ),
        },
        None => (
            Vec::new(),
            format!(
                "swarm cache void — {} holds no {SWARM_NEEDLE} ring cache",
                dir.display()
            ),
        ),
    }
}

fn encounter(
    ground: &[Row],
    swarm: &[Row],
    st_lat: f64,
    st_lon: f64,
    radius_deg: f64,
) -> Option<Encounter> {
    let mut best: Option<(f64, usize)> = None;
    for (i, r) in swarm.iter().enumerate() {
        let (Some(la), Some(lo)) = (r.lat, r.lon) else {
            continue;
        };
        let d = angular_distance_deg(st_lat, st_lon, la, lo);
        if best.map_or(true, |(bd, _)| d < bd) {
            best = Some((d, i));
        }
    }
    let (dist, si) = best?;
    if !(dist <= radius_deg) {
        return None;
    }
    let enc_t = swarm[si].t;
    let swarm_f = swarm[si].f;
    let mut nearest: Option<(f64, usize)> = None;
    for (i, r) in ground.iter().enumerate() {
        let d = (r.t - enc_t).abs();
        if nearest.map_or(true, |(bd, _)| d < bd) {
            nearest = Some((d, i));
        }
    }
    let (gd, gi) = nearest?;
    if gd > ENCOUNTER_MIN_LIMIT {
        return None;
    }
    let ground_f = ground[gi].f;
    let g_win: Vec<f64> = ground
        .iter()
        .filter(|r| (r.t - enc_t).abs() <= GROUND_SIGMA_WINDOW_S)
        .map(|r| r.f)
        .collect();
    let s_win: Vec<f64> = swarm
        .iter()
        .filter(|r| (r.t - enc_t).abs() <= SWARM_SIGMA_WINDOW_S)
        .map(|r| r.f)
        .collect();
    let ground_sigma = std(&g_win).unwrap_or(f64::NAN);
    let swarm_sigma = std(&s_win).unwrap_or(f64::NAN);
    Some(Encounter {
        epoch: enc_t,
        distance_deg: dist,
        ground_f,
        swarm_f,
        ground_sigma,
        swarm_sigma,
    })
}

fn classify(ground_ok: bool, swarm_ok: bool, deviation: f64, tolerance: Option<f64>) -> Verdict {
    match (ground_ok, swarm_ok) {
        (true, true) => match tolerance {
            Some(t) if t.is_finite() && t >= 0.0 => {
                if deviation <= t {
                    Verdict::Placed
                } else {
                    Verdict::Riss
                }
            }
            _ => {
                if deviation == 0.0 {
                    Verdict::Placed
                } else {
                    Verdict::Riss
                }
            }
        },
        _ => Verdict::Absent,
    }
}

fn verdict_line(
    id: &str,
    v: &Verdict,
    ground_note: &str,
    swarm_note: &str,
    enc: Option<&Encounter>,
    tolerance: Option<f64>,
) -> String {
    let point = |e: &Encounter| {
        format!(
            "ground {:.1} nT ({GROUND_FIELDS}, vector magnitude) swarm {:.1} nT ({SWARM_FIELD}, overflight {:.2} deg off at {})",
            e.ground_f,
            e.swarm_f,
            e.distance_deg,
            iso_utc(e.epoch)
        )
    };
    match v {
        Verdict::Placed => {
            let e = enc.expect("a placed verdict carries an encounter");
            let tol = match tolerance {
                Some(t) => format!("{t:.1}"),
                None => "0.0".to_string(),
            };
            format!(
                "station {id} state placed {} deviation {:.1} nT within tolerance {tol} nT — two independent lines carry the same point field (zwirn)",
                point(e),
                (e.ground_f - e.swarm_f).abs()
            )
        }
        Verdict::Riss => {
            let e = enc.expect("a riss verdict carries an encounter");
            let tol = match tolerance {
                Some(t) => format!("{t:.1}"),
                None => "0.0".to_string(),
            };
            format!(
                "station {id} state riss {} deviation {:.1} nT beyond tolerance {tol} nT — two independent lines refuse to converge, the riss stays visible",
                point(e),
                (e.ground_f - e.swarm_f).abs()
            )
        }
        Verdict::Absent => {
            let g = if ground_note.contains("carries") || ground_note.contains("reads") {
                "present"
            } else {
                "absent"
            };
            let s = if swarm_note.contains("holds") || swarm_note.contains("reads") {
                "present"
            } else {
                "absent"
            };
            format!(
                "station {id} state absent ground_line ({GROUND_FIELDS}) {g}: {ground_note}; swarm_line ({SWARM_FIELD}) {s}: {swarm_note} — the station carries fewer than two independent matched lines at the point"
            )
        }
    }
}

fn grammar() {
    let tol = SIGMA_K * (0.4f64 * 0.4 + 0.6f64 * 0.6).sqrt();
    let placed = verdict_line(
        "ABK",
        &Verdict::Placed,
        "",
        "",
        Some(&Encounter {
            epoch: 1.7e9,
            distance_deg: 0.02,
            ground_f: 53699.2,
            swarm_f: 53700.6,
            ground_sigma: 0.4,
            swarm_sigma: 0.6,
        }),
        Some(tol),
    );
    let absent = verdict_line(
        "ABK",
        &Verdict::Absent,
        "ground bin void — the ring cache is not local",
        "swarm ring cache holds 3600 sample(s), carries time+F only — the station-point geometry is not in the cache",
        None,
        None,
    );
    let riss = verdict_line(
        "ABK",
        &Verdict::Riss,
        "",
        "",
        Some(&Encounter {
            epoch: 1.7e9,
            distance_deg: 0.02,
            ground_f: 53699.2,
            swarm_f: 44510.0,
            ground_sigma: 0.4,
            swarm_sigma: 0.6,
        }),
        Some(tol),
    );
    println!("{placed}");
    println!("{absent}");
    println!("{riss}");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if flag(&args, "--help") || flag(&args, "-h") {
        usage();
        return;
    }
    if flag(&args, "--grammar") {
        println!("=== the station verdict grammar (three canonical classifier states) ===");
        grammar();
        return;
    }
    let live = flag(&args, "--live");
    let station = arg_after(&args, "--station")
        .unwrap_or(GROUND_DATASET)
        .to_string();
    let sources = load_sources();
    let register = register_station_point(&sources);
    let (st_lat, st_lon) = match (arg_f64(&args, "--lat"), arg_f64(&args, "--lon")) {
        (Some(la), Some(lo)) if la.is_finite() && lo.is_finite() => (la, lo),
        _ => match register {
            Some((la, lo)) => (la, lo),
            None => {
                eprintln!("the station point is not in the register and no --lat/--lon given");
                return;
            }
        },
    };
    let radius = match arg_f64(&args, "--radius") {
        Some(r) if r.is_finite() && r > 0.0 => r,
        _ => DEFAULT_RADIUS_DEG,
    };
    let tol_arg = arg_f64(&args, "--tolerance-nT");
    let data_dir = match arg_after(&args, "--data-dir") {
        Some(d) => PathBuf::from(d),
        None => PathBuf::from("data"),
    };

    let (start, stop) = if live {
        let Some(now) = now_unix() else {
            return;
        };
        (
            match arg_after(&args, "--start") {
                Some(s) => s.to_string(),
                None => iso_utc(now - 38.0 * HOUR),
            },
            match arg_after(&args, "--stop") {
                Some(s) => s.to_string(),
                None => iso_utc(now - 26.0 * HOUR),
            },
        )
    } else {
        (String::new(), String::new())
    };

    let (ground_rows, ground_note) = if live {
        let url = ground_fetch_url(&station, &start, &stop);
        match fetch_raw(&url, None, &[], 600) {
            Some(text) => {
                let rows = parse_vector_rows(&text);
                if rows.is_empty() {
                    (rows, format!("live fetch {url} reads no vector rows"))
                } else {
                    let note = format!(
                        "live fetch {url} (cached {}) carries {} minute sample(s)",
                        write_json_cache(GROUND_NETLOC, &url, &text).display(),
                        rows.len()
                    );
                    (rows, note)
                }
            }
            None => (Vec::new(), format!("live fetch {url} returned void")),
        }
    } else {
        load_ground_offline(&station)
    };
    let ground_present = !ground_rows.is_empty();

    let (swarm_rows, swarm_note) = if live {
        let url = swarm_fetch_url(&start, &stop);
        match fetch_raw(&url, None, &[], 600) {
            Some(text) => {
                let rows = parse_scalar_rows(&text);
                if rows.is_empty() {
                    (rows, format!("live fetch {url} reads no scalar rows"))
                } else {
                    let note = format!(
                        "live fetch {url} (cached {}) carries {} sample(s)",
                        write_json_cache(SWARM_NETLOC, &url, &text).display(),
                        rows.len()
                    );
                    (rows, note)
                }
            }
            None => (Vec::new(), format!("live fetch {url} returned void")),
        }
    } else {
        load_swarm_offline()
    };
    let swarm_present = !swarm_rows.is_empty();
    let swarm_geometry = swarm_rows
        .iter()
        .any(|r| r.lat.is_some() && r.lon.is_some());

    println!(
        "=== station convergence probe — the station Verdict on two independent network lines ==="
    );
    println!("station {station} at {st_lat:.3}N {st_lon:.3}E (register point, body earth)");
    println!("line 1 {GROUND_FIELDS}: {ground_note}");
    println!("line 2 {SWARM_FIELD}: {swarm_note}");
    if let Some(note) = load_ground_derived_bin(&station, &data_dir) {
        println!("line 1 register note: {note}");
    }
    println!("encounter bound: overflight capture {radius} deg; tolerance from the two lines' measured variability (2σ) or an operator --tolerance-nT");

    let enc = if ground_present && swarm_present && swarm_geometry {
        encounter(&ground_rows, &swarm_rows, st_lat, st_lon, radius)
    } else {
        None
    };
    match &enc {
        Some(e) => {
            let tol = tol_arg.or_else(|| {
                if e.ground_sigma.is_finite() && e.swarm_sigma.is_finite() {
                    Some(
                        SIGMA_K
                            * (e.ground_sigma * e.ground_sigma + e.swarm_sigma * e.swarm_sigma)
                                .sqrt(),
                    )
                } else {
                    None
                }
            });
            let dev = (e.ground_f - e.swarm_f).abs();
            let v = classify(true, true, dev, tol);
            println!(
                "encounter: closest swarm sample {:.2} deg off the station at {}; ground {} nT at the same minute; swarm {} nT; ground σ {:.2} nT, swarm σ {:.2} nT over their windows",
                e.distance_deg,
                iso_utc(e.epoch),
                e.ground_f,
                e.swarm_f,
                e.ground_sigma,
                e.swarm_sigma
            );
            println!(
                "{}",
                verdict_line(&station, &v, &ground_note, &swarm_note, Some(e), tol)
            );
        }
        None => {
            let v = classify(ground_present, swarm_present && swarm_geometry, 0.0, None);
            println!(
                "{}",
                verdict_line(&station, &v, &ground_note, &swarm_note, None, None)
            );
        }
    }
    println!("Verdict vocabulary: Placed = two independent lines agree (zwirn); Absent = fewer than two carry a value at the point; Riss = two present but refuse to converge — never smoothed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso_round_trip() {
        let u = 1.766_16e9;
        let s = iso_utc(u);
        assert_eq!(iso_to_unix(&s), Some(u));
    }

    #[test]
    fn iso_parses_minute_precision_with_z() {
        let u = iso_to_unix("2026-09-05T12:00Z").unwrap();
        assert_eq!(iso_utc(u), "2026-09-05T12:00:00Z");
    }

    #[test]
    fn vector_rows_give_ground_magnitudes() {
        let text = r#"{"data":[["2026-09-05T12:00Z",[11127.7,2136.4,52490.6],53699.2],["2026-09-05T12:01Z",[11127.4,2136.9,52490.5],53699.1]]}"#;
        let rows = parse_vector_rows(text);
        assert_eq!(rows.len(), 2);
        let want = (11127.7f64.powi(2) + 2136.4f64.powi(2) + 52490.6f64.powi(2)).sqrt();
        assert!((rows[0].f - want).abs() < 1e-3);
        assert!(rows[0].lat.is_none());
    }

    #[test]
    fn scalar_rows_carry_geometry_when_present() {
        let text = r#"{"data":[["2026-09-05T12:00:00.000Z",9.344,-137.772,26639.44],["2026-09-05T12:00:01.000Z",9.409,-137.773,26653.7]]}"#;
        let rows = parse_scalar_rows(text);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].lat, Some(9.344));
        assert_eq!(rows[0].f, 26639.44);
    }

    #[test]
    fn scalar_rows_without_geometry_stay_geometry_less() {
        let text = r#"{"data":[["2026-09-05T12:00:00.000Z",26639.44]]}"#;
        let rows = parse_scalar_rows(text);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].lat.is_none());
    }

    #[test]
    fn fill_values_refuse_a_vector_row() {
        let text = r#"{"data":[["2026-09-05T12:00Z",[99999.0,2136.4,52490.6],53699.2]]}"#;
        assert!(parse_vector_rows(text).is_empty());
    }

    #[test]
    fn two_present_lines_that_agree_are_placed() {
        assert!(matches!(
            classify(true, true, 1.4, Some(4.8)),
            Verdict::Placed
        ));
    }

    #[test]
    fn two_present_lines_that_refuse_are_riss() {
        assert!(matches!(
            classify(true, true, 9189.2, Some(4.8)),
            Verdict::Riss
        ));
    }

    #[test]
    fn a_missing_line_is_absent() {
        assert!(matches!(classify(false, true, 0.0, None), Verdict::Absent));
        assert!(matches!(classify(true, false, 0.0, None), Verdict::Absent));
        assert!(matches!(classify(false, false, 0.0, None), Verdict::Absent));
    }

    #[test]
    fn stats_are_measured() {
        assert_eq!(std(&[2.0, 4.0]), Some(1.0));
        assert_eq!(std(&[2.0]), None);
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn verdict_lines_name_both_lines() {
        let placed = verdict_line(
            "ABK",
            &Verdict::Placed,
            "",
            "",
            Some(&Encounter {
                epoch: 1.7e9,
                distance_deg: 0.02,
                ground_f: 53699.2,
                swarm_f: 53700.6,
                ground_sigma: 0.4,
                swarm_sigma: 0.6,
            }),
            Some(1.44),
        );
        assert!(placed.contains("state placed"));
        assert!(placed.contains(GROUND_FIELDS));
        assert!(placed.contains(SWARM_FIELD));
        assert!(placed.contains("53699.2"));
        assert!(placed.contains("53700.6"));
        let absent = verdict_line(
            "ABK",
            &Verdict::Absent,
            "ground bin void",
            "swarm cache void",
            None,
            None,
        );
        assert!(absent.contains("state absent"));
        assert!(absent.contains("ground bin void"));
        let riss = verdict_line(
            "ABK",
            &Verdict::Riss,
            "",
            "",
            Some(&Encounter {
                epoch: 1.7e9,
                distance_deg: 0.02,
                ground_f: 53699.2,
                swarm_f: 44510.0,
                ground_sigma: 0.4,
                swarm_sigma: 0.6,
            }),
            Some(1.44),
        );
        assert!(riss.contains("state riss"));
        assert!(riss.contains("riss stays visible"));
    }
}
