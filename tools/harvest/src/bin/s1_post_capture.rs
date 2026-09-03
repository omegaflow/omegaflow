use std::process::Command;

const EVENT_EPOCH: &str = "2026-08-26T02:52:10Z";
const BBOX: (f64, f64, f64, f64) = (85.40, 28.15, 85.60, 28.40);

fn curl(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("60")
        .arg("-A")
        .arg("omegaflow-gate/1.0")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn sas_token(collection: &str) -> Option<String> {
    let url = format!("https://planetarycomputer.microsoft.com/api/sas/v1/token/{collection}");
    let body = String::from_utf8(curl(&url)?).ok()?;
    let key = "\"token\":\"";
    let i = body.find(key)? + key.len();
    let tok = body[i..].split('"').next()?.to_string();
    if tok.is_empty() { None } else { Some(tok) }
}

fn first_post_scene() -> Option<(String, String, String)> {
    let url = format!(
        "https://planetarycomputer.microsoft.com/api/stac/v1/search?collections=sentinel-1-grd&bbox={},{},{},{}&datetime={}/2030-01-01T00:00:00Z&limit=5&sortby=datetime",
        BBOX.0, BBOX.1, BBOX.2, BBOX.3, EVENT_EPOCH
    );
    let body = String::from_utf8(curl(&url)?).ok()?;
    let mut rest = body.as_str();
    while let Some(i) = rest.find("\"id\":\"S1") {
        rest = &rest[i..];
        let id = rest[6..].split('"').next()?.to_string();
        let dt = rest
            .split("\"datetime\":\"")
            .nth(1)?
            .split('"')
            .next()?
            .to_string();
        if dt.as_str() > EVENT_EPOCH {
            let href = rest
                .split("\"vv\":{\"href\":\"")
                .nth(1)?
                .split('"')
                .next()?
                .to_string();
            return Some((id, href, dt));
        }
        rest = &rest[6..];
    }
    None
}

fn main() {
    let tok = match sas_token("sentinel-1-grd") {
        Some(t) => t,
        None => {
            println!("SAS-Token: pending (fehlgeschlagen)");
            return;
        }
    };
    match first_post_scene() {
        Some((id, href, dt)) => {
            println!("=== S1-Post-Scene gefunden ===");
            println!("id: {id}");
            println!("datetime: {dt}  (Kollab {} )", EVENT_EPOCH);
            println!("vv-asset: {href}");
            let path = format!("/tmp/opencode/s1post/{id}_vv.tif");
            std::fs::create_dir_all("/tmp/opencode/s1post").ok();
            println!("lade VV-COG nach {path} …");
            let out = Command::new("curl")
                .arg("-sL")
                .arg("--max-time")
                .arg("1500")
                .arg("-C")
                .arg("-")
                .arg("-A")
                .arg("omegaflow-gate/1.0")
                .arg("-o")
                .arg(&path)
                .arg(format!("{href}?{tok}"))
                .output();
            match out {
                Ok(o) if o.status.success() => {
                    let sz = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    println!("fertig: {sz} bytes → {path}");
                    let vor = std::env::var("OMEGAFLOW_S1_VOR").unwrap_or_default();
                    if !vor.is_empty() {
                        println!("Vor-Baseline gesetzt: {vor} → Post-vs-Vor-Differenz bereit.");
                    } else {
                        println!("no pre-baseline set (OMEGAFLOW_S1_VOR) — difference pending.");
                    }
                }
                _ => println!("Download: pending (curl fehlgeschlagen)"),
            }
        }
        None => {
            println!(
                "S1-Post-Scene: PENDING — no pass since {} at (85.4–85.6/28.15–28.4).",
                EVENT_EPOCH
            );
            println!("next pass expected ~08-30 (6-day cycle: 08-24 -> 08-30).");
        }
    }
}
