use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use omegaflow::lsk::days_from_civil;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "services.swpc.noaa.gov";
const WIND: &str = "https://services.swpc.noaa.gov/json/rtsw/rtsw_wind_1m.json";
const MAG: &str = "https://services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "swpc_rtsw_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn parse_iso_unix(s: &str) -> Option<i64> {
    let text = s.trim();
    let text = match text.strip_suffix('Z') {
        Some(t) => t,
        None => text,
    };
    let (date, clock) = text.split_once('T')?;
    let mut d = date.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let mut t = clock.split(':');
    let hour: i64 = t.next()?.parse().ok()?;
    let minute: i64 = t.next()?.parse().ok()?;
    let second: i64 = if let Some(sec) = t.next() {
        sec.split('.').next()?.parse().ok()?
    } else {
        0
    };
    Some(days * 86400 + hour * 3600 + minute * 60 + second)
}

fn member<'a>(root: &'a JsonVal, key: &str) -> Option<&'a JsonVal> {
    match root {
        JsonVal::Obj(map) => map.get(key),
        _ => None,
    }
}

fn num(root: &JsonVal, key: &str) -> Option<f64> {
    match member(root, key) {
        Some(JsonVal::Num(v)) if v.is_finite() => Some(*v),
        _ => None,
    }
}

fn text<'a>(root: &'a JsonVal, key: &str) -> Option<&'a str> {
    match member(root, key) {
        Some(JsonVal::Str(s)) => Some(s.as_str()),
        _ => None,
    }
}

struct Wind {
    unix: i64,
    source: String,
    speed: f64,
    density: f64,
}

struct Row {
    unix: i64,
    source: String,
    speed: f64,
    density: f64,
    bz: f64,
    bt: f64,
}

fn wind_rows(root: &JsonVal) -> Option<Vec<Wind>> {
    let JsonVal::Arr(rows) = root else {
        return None;
    };
    let mut out = Vec::new();
    for row in rows {
        let Some(unix) = text(row, "time_tag").and_then(parse_iso_unix) else {
            continue;
        };
        let (Some(speed), Some(density)) = (num(row, "proton_speed"), num(row, "proton_density"))
        else {
            continue;
        };
        let Some(source) = text(row, "source") else {
            continue;
        };
        out.push(Wind {
            unix,
            source: source.to_string(),
            speed,
            density,
        });
    }
    Some(out)
}

fn mag_rows(root: &JsonVal) -> Option<HashMap<(i64, String), (f64, f64)>> {
    let JsonVal::Arr(rows) = root else {
        return None;
    };
    let mut out: HashMap<(i64, String), (f64, f64)> = HashMap::new();
    for row in rows {
        let Some(unix) = text(row, "time_tag").and_then(parse_iso_unix) else {
            continue;
        };
        let (Some(bz), Some(bt)) = (num(row, "bz_gsm"), num(row, "bt")) else {
            continue;
        };
        let Some(source) = text(row, "source") else {
            continue;
        };
        out.entry((unix.div_euclid(60), source.to_string()))
            .or_insert((bz, bt));
    }
    Some(out)
}

fn join(wind: Vec<Wind>, mag: &HashMap<(i64, String), (f64, f64)>) -> Vec<Row> {
    let mut rows = Vec::new();
    for w in wind {
        let Some((bz, bt)) = mag.get(&(w.unix.div_euclid(60), w.source.clone())) else {
            continue;
        };
        rows.push(Row {
            unix: w.unix,
            source: w.source,
            speed: w.speed,
            density: w.density,
            bz: *bz,
            bt: *bt,
        });
    }
    rows.sort_by(|a, b| (a.unix, &a.source).cmp(&(b.unix, &b.source)));
    rows
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/rtsw.txt"),
    };

    let wind_body = curl(WIND).ok_or_else(|| format!("{WIND}: fetch void"))?;
    let mag_body = curl(MAG).ok_or_else(|| format!("{MAG}: fetch void"))?;
    let wind_root = parse_json(&wind_body).ok_or_else(|| format!("{WIND}: response parse void"))?;
    let mag_root = parse_json(&mag_body).ok_or_else(|| format!("{MAG}: response parse void"))?;
    let Some(wind) = wind_rows(&wind_root) else {
        return Err(format!("{WIND}: response carries no array"));
    };
    let Some(mag) = mag_rows(&mag_root) else {
        return Err(format!("{MAG}: response carries no array"));
    };
    if wind.is_empty() {
        println!(
            "swpc_rtsw_compiler: {WIND} carries no measured proton row — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    }
    if mag.is_empty() {
        println!(
            "swpc_rtsw_compiler: {MAG} carries no measured bz/bt row — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    }

    let rows = join(wind, &mag);
    if rows.is_empty() {
        println!(
            "swpc_rtsw_compiler: no timestamp carries a finite speed, density, bz and bt — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    }

    let mut body = format!(
        "# swpc rtsw | unix speed_km_s density_p_cm3 bz_gsm_nt bt_nt source | wind/mag paired on the same source and UTC minute | speed/density {WIND} | bz/bt {MAG}\n"
    );
    for r in &rows {
        body.push_str(&format!(
            "{} {} {} {} {} {}\n",
            r.unix, r.speed, r.density, r.bz, r.bt, r.source
        ));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, body.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "swpc_rtsw_compiler: {} rows written to {out} ({} B)",
        rows.len(),
        body.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("swpc_rtsw_compiler: {msg}");
        std::process::exit(2);
    }
}
