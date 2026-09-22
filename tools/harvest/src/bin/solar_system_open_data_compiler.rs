use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use std::process::Command;

const URL: &str = "https://api.le-systeme-solaire.net/rest/bodies/";
const NETLOC: &str = "api.le-systeme-solaire.net";
const G_M3_KG_S2: f64 = 6.67430e-11;
const NAME_BYTES: usize = 64;

const S_MEAN_RADIUS: usize = 0;
const S_EQUA_RADIUS: usize = 1;
const S_POLAR_RADIUS: usize = 2;
const S_GM: usize = 3;
const S_MASS: usize = 4;
const S_GRAVITY: usize = 5;
const S_DENSITY: usize = 6;
const S_OMEGA: usize = 7;
const S_AXIAL_TILT: usize = 8;
const S_SEMIMAJOR: usize = 9;
const S_FLATTENING: usize = 10;
const N_SLOTS: usize = 11;

const RECORD_STRIDE: usize = NAME_BYTES + NAME_BYTES + 8 + 8 + N_SLOTS * 8;
const MAGIC: [u8; 4] = [0xCF, 0x86, 0x0A, 0x00];

#[derive(Clone)]
struct Body {
    name: String,
    parent: String,
    is_planet: bool,
    slots: [Option<f64>; N_SLOTS],
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_secret(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();
        let line = line.strip_prefix("export ").unwrap_or(line);
        if let Some((k, v)) = line.split_once('=') {
            let v = v.trim().trim_matches('"').trim_matches('\'');
            if k.trim() == key && !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    parse_secret(&body, name)
}

fn obj_get<'a>(v: &'a JsonVal, key: &str) -> Option<&'a JsonVal> {
    match v {
        JsonVal::Obj(map) => map.get(key),
        _ => None,
    }
}

fn num(v: &JsonVal) -> Option<f64> {
    match v {
        JsonVal::Num(n) => Some(*n),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}

fn str_of(v: &JsonVal) -> Option<String> {
    match v {
        JsonVal::Str(s) => Some(s.clone()),
        _ => None,
    }
}

fn bool_of(v: &JsonVal) -> Option<bool> {
    match v {
        JsonVal::Bool(b) => Some(*b),
        _ => None,
    }
}

fn positive(v: f64) -> Option<f64> {
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

fn fetch_bodies(token: &str) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSLf")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("120")
        .arg("-H")
        .arg(format!("Authorization: Bearer {token}"))
        .arg(URL);
    let out = cmd.output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch {} returned ({}): {}",
            URL,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn body_of(entry: &JsonVal) -> Option<Body> {
    let name = str_of(obj_get(entry, "englishName")?)?;
    let parent = match obj_get(entry, "aroundPlanet") {
        Some(JsonVal::Obj(map)) => match map.get("planet") {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => String::new(),
        },
        _ => String::new(),
    };
    let is_planet = bool_of(obj_get(entry, "isPlanet")?).unwrap_or(false);

    let mass_kg = obj_get(entry, "mass").and_then(|m| {
        let value = num(obj_get(m, "massValue")?)?;
        let exponent = num(obj_get(m, "massExponent")?)?;
        positive(value * 10f64.powf(exponent))
    });
    let gm_m3_s2 = mass_kg.map(|m| m * G_M3_KG_S2);

    let mean_radius_m = obj_get(entry, "meanRadius")
        .and_then(num)
        .and_then(|v| positive(v * 1000.0));
    let equa_radius_m = obj_get(entry, "equaRadius")
        .and_then(num)
        .and_then(|v| positive(v * 1000.0));
    let polar_radius_m = obj_get(entry, "polarRadius")
        .and_then(num)
        .and_then(|v| positive(v * 1000.0));

    let gravity_m_s2 = obj_get(entry, "gravity").and_then(num).and_then(positive);
    let density_kg_m3 = obj_get(entry, "density")
        .and_then(num)
        .and_then(|v| positive(v * 1000.0));

    let omega_rad_s = obj_get(entry, "sideralRotation")
        .and_then(num)
        .and_then(|h| {
            if h.is_finite() && h > 0.0 {
                Some(2.0 * std::f64::consts::PI / (h * 3600.0))
            } else {
                None
            }
        });

    let axial_tilt_deg = obj_get(entry, "axialTilt").and_then(num).and_then(|v| {
        if v.is_finite() && v != 0.0 {
            Some(v)
        } else {
            None
        }
    });

    let semimajor_m = obj_get(entry, "semimajorAxis")
        .and_then(num)
        .and_then(|v| positive(v * 1000.0));

    let flattening = match (equa_radius_m, polar_radius_m) {
        (Some(a), Some(c)) if a > 0.0 => {
            let f = (a - c) / a;
            if f.is_finite() { Some(f) } else { None }
        }
        _ => None,
    };

    let mut slots = [None; N_SLOTS];
    slots[S_MEAN_RADIUS] = mean_radius_m;
    slots[S_EQUA_RADIUS] = equa_radius_m;
    slots[S_POLAR_RADIUS] = polar_radius_m;
    slots[S_GM] = gm_m3_s2;
    slots[S_MASS] = mass_kg;
    slots[S_GRAVITY] = gravity_m_s2;
    slots[S_DENSITY] = density_kg_m3;
    slots[S_OMEGA] = omega_rad_s;
    slots[S_AXIAL_TILT] = axial_tilt_deg;
    slots[S_SEMIMAJOR] = semimajor_m;
    slots[S_FLATTENING] = flattening;

    Some(Body {
        name,
        parent,
        is_planet,
        slots,
    })
}

fn write_catalog(bodies: &[Body]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(12 + bodies.len() * RECORD_STRIDE);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(bodies.len() as u32).to_le_bytes());
    for b in bodies {
        let mut name = [0u8; NAME_BYTES];
        let nb = b.name.as_bytes();
        let n = nb.len().min(NAME_BYTES);
        name[..n].copy_from_slice(&nb[..n]);
        buf.extend_from_slice(&name);

        let mut parent = [0u8; NAME_BYTES];
        let pb = b.parent.as_bytes();
        let p = pb.len().min(NAME_BYTES);
        parent[..p].copy_from_slice(&pb[..p]);
        buf.extend_from_slice(&parent);

        buf.push(b.is_planet as u8);
        buf.extend_from_slice(&[0u8; 7]);

        let mut mask: u64 = 0;
        for (i, s) in b.slots.iter().enumerate() {
            if s.is_some() {
                mask |= 1u64 << i;
            }
        }
        buf.extend_from_slice(&mask.to_le_bytes());
        for s in &b.slots {
            match s {
                Some(x) => buf.extend_from_slice(&x.to_le_bytes()),
                None => buf.extend_from_slice(&0.0_f64.to_le_bytes()),
            }
        }
    }
    buf
}

fn cstr(b: &[u8]) -> String {
    let end = b.iter().position(|&x| x == 0).unwrap_or(b.len());
    String::from_utf8_lossy(&b[..end]).into_owned()
}

fn parse_catalog(bytes: &[u8]) -> Option<Vec<Body>> {
    if bytes.len() < 12 || bytes[..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let expected = 12usize.checked_add(count.checked_mul(RECORD_STRIDE)?)?;
    if bytes.len() != expected {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    let mut pos = 12usize;
    for _ in 0..count {
        let name = cstr(&bytes[pos..pos + NAME_BYTES]);
        pos += NAME_BYTES;
        let parent = cstr(&bytes[pos..pos + NAME_BYTES]);
        pos += NAME_BYTES;
        let is_planet = bytes[pos] != 0;
        pos += 8;
        let mask = u64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
        pos += 8;
        let mut slots = [None; N_SLOTS];
        for i in 0..N_SLOTS {
            let v = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
            pos += 8;
            if mask & (1u64 << i) != 0 {
                slots[i] = Some(v);
            }
        }
        out.push(Body {
            name,
            parent,
            is_planet,
            slots,
        });
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "solar_system_bodies.bin".to_string(),
    };

    let token = match secret("SOLAR_SYSTEM_OPEN_DATA_KEY") {
        Some(t) => t,
        None => {
            eprintln!(
                "SOLAR_SYSTEM_OPEN_DATA_KEY absent — the environment and .secrets.local carry no token (0 honored)"
            );
            std::process::exit(1);
        }
    };

    let Some(raw) = fetch_bodies(&token) else {
        eprintln!("no catalog returned — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };

    let Some(root) = parse_json(&String::from_utf8_lossy(&raw)) else {
        eprintln!("the catalog body is not JSON — the bin stays unwritten");
        std::process::exit(1);
    };
    let Some(JsonVal::Arr(entries)) = obj_get(&root, "bodies") else {
        eprintln!("the catalog carries no bodies array — the bin stays unwritten");
        std::process::exit(1);
    };

    let mut bodies: Vec<Body> = entries.iter().filter_map(body_of).collect();
    bodies.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    if bodies.is_empty() {
        eprintln!("no bodies parsed — the catalog stays unwritten (0 honored)");
        std::process::exit(1);
    }

    let planets = bodies.iter().filter(|b| b.is_planet).count();
    let with_gm = bodies.iter().filter(|b| b.slots[S_GM].is_some()).count();
    let with_omega = bodies.iter().filter(|b| b.slots[S_OMEGA].is_some()).count();

    let bytes = write_catalog(&bodies);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }

    match parse_catalog(&bytes) {
        Some(parsed) if parsed.len() == bodies.len() => {
            eprintln!(
                "{}: {} bodies ({} planets, gm on {}, omega on {}), {} B — roundtrip parses",
                out,
                parsed.len(),
                planets,
                with_gm,
                with_omega,
                bytes.len()
            );
        }
        _ => {
            eprintln!(
                "{}: roundtrip parse void — the catalog stays unverified",
                out
            );
            std::process::exit(1);
        }
    }

    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
