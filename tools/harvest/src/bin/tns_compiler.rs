use omegaflow::archivar::tns::{write_bin, TnsObject, NAME_LEN};
use omegaflow::cdn::upload_asset;
use std::process::Command;

const TNS_BASE: &str = "https://www.wis-tns.org";
const OUT_DEFAULT: &str = "/tmp/opencode/tns.bin";
const CONE_RADIUS_DEG: f64 = 0.05;

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    for path in [".secrets.local"] {
        if let Ok(body) = std::fs::read_to_string(path) {
            for line in body.lines() {
                if let Some((k, v)) = line.split_once('=') {
                    if k.trim() == name && !v.trim().is_empty() {
                        return Some(v.trim().to_string());
                    }
                }
            }
        }
    }
    None
}

fn tns_post(endpoint: &str, data: &str, ua: &str, key: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg(format!("user-agent: {}", ua))
        .arg("--data-urlencode")
        .arg(format!("api_key={}", key))
        .arg("--data-urlencode")
        .arg(format!("data={}", data))
        .arg(format!("{}{}", TNS_BASE, endpoint))
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!("tns {}: http {}", endpoint, out.status,);
        None
    }
}

fn parse_field(body: &str, key: &str) -> Option<f64> {
    let needle = format!("\"{}\":", key);
    let idx = body.find(&needle)?;
    let rest = &body[idx + needle.len()..];
    let num: String = rest
        .trim_start()
        .chars()
        .take_while(|c| {
            c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+' || *c == 'e' || *c == 'E'
        })
        .collect();
    num.parse::<f64>().ok()
}

fn cone_names(ra: f64, dec: f64, radius: f64, ua: &str, key: &str) -> Vec<String> {
    let data = format!(
        "{{\"ra\":{},\"dec\":{},\"radius\":{},\"units\":\"deg\"}}",
        ra, dec, radius
    );
    let Some(body) = tns_post("/api/get/search", &data, ua, key) else {
        eprintln!("  tns_post search: leer (http-fehl)");
        return Vec::new();
    };
    let mut names = Vec::new();
    let mut rest = body.as_str();
    let mut guard = 0usize;
    while let Some(idx) = rest.find("\"objname\":\"") {
        guard += 1;
        if guard > 50 {
            break;
        }
        rest = &rest[idx + 11..];
        let end = rest.find('"').unwrap_or(0);
        if end > 0 {
            names.push(rest[..end].to_string());
        }
        rest = &rest[end..];
    }
    names
}

fn object_z(name: &str, ua: &str, key: &str) -> Option<(f64, f64, f64)> {
    let data = format!("{{\"objname\":\"{}\"}}", name);
    let body = tns_post("/api/get/object", &data, ua, key)?;
    let ra = parse_field(&body, "radeg")?;
    let dec = parse_field(&body, "decdeg")?;
    let z = parse_field(&body, "redshift")?;
    Some((ra, dec, z))
}

fn run(out_path: &str, candidates: &[(f64, f64)], radius: f64, ci: bool) -> Result<(), String> {
    let ua = secret("TNS_UA").ok_or("TNS_UA absent (.secrets.local) — refused")?;
    let key = secret("TNS_API_KEY").ok_or("TNS_API_KEY absent (.secrets.local) — refused")?;
    let mut objs: Vec<TnsObject> = Vec::new();
    for &(ra, dec) in candidates {
        let names = cone_names(ra, dec, radius, &ua, &key);
        eprintln!("cone ({ra},{dec}) r={radius}: {} objekte", names.len());
        for name in names {
            if let Some((or_, od, oz)) = object_z(&name, &ua, &key) {
                let mut nb = [0u8; NAME_LEN];
                let bytes = name.as_bytes();
                let n = bytes.len().min(NAME_LEN);
                nb[..n].copy_from_slice(&bytes[..n]);
                objs.push(TnsObject {
                    name: nb,
                    ra_deg: or_,
                    dec_deg: od,
                    z: oz,
                });
            } else {
                eprintln!("  {name}: no redshift (skipped)");
            }
        }
    }
    if objs.is_empty() {
        return Err("no TNS objects harvested — the output would be an empty bin (refused)".into());
    }
    let bytes = write_bin(&objs).ok_or("write_bin: non-finite value refused")?;
    std::fs::write(out_path, &bytes).map_err(|e| format!("{out_path}: {e}"))?;
    println!("tns.bin: {} objects, {} B", objs.len(), bytes.len());
    if ci {
        if !upload_asset(out_path) {
            return Err(format!("{out_path}: CDN upload returned void"));
        }
        println!("tns.bin: uploaded to the CDN");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out = OUT_DEFAULT.to_string();
    let mut ci = false;
    let mut radius = CONE_RADIUS_DEG;
    let mut i = 0usize;
    let mut candidates: Vec<(f64, f64)> = Vec::new();
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(OUT_DEFAULT.to_string());
            }
            "--ci-mode" => ci = true,
            "--radius" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse::<f64>().ok()) {
                    radius = v;
                }
            }
            "--cone" => {
                i += 1;
                let ra = args.get(i).and_then(|s| s.parse().ok());
                i += 1;
                let dec = args.get(i).and_then(|s| s.parse().ok());
                if let (Some(r), Some(d)) = (ra, dec) {
                    candidates.push((r, d));
                }
            }
            other => {
                eprintln!("tns_compiler: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if candidates.is_empty() {
        eprintln!("tns_compiler: no --cone ra dec given — refused");
        std::process::exit(1);
    }
    if let Err(msg) = run(&out, &candidates, radius, ci) {
        eprintln!("tns_compiler: {msg}");
        std::process::exit(1);
    }
}
