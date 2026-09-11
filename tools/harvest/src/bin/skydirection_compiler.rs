use omegaflow::archivar::{embedded_lsk, LeapSeconds};
use omegaflow::cdn::upload_asset;
use omegaflow::json::{jnum, jstr, parse_json, JsonVal};
use omegaflow::skydirection::{write_bin, SkyBandSeries, SkyDirection, SkySample};
use std::process::Command;

const UA: &str = "omegaflow-skydirection-compiler/1.0";
const LAS_QUERY: &str = "https://lasair-ztf.lsst.ac.uk/api/query/";
const ANTARES_LOCI: &str = "https://api.antares.noirlab.edu/v1/loci";
const ALERCE_OBJECTS: &str = "https://api.alerce.online/objects";
const FINK_CONE: &str = "https://api.lsst.fink-portal.org/api/v1/conesearch";
const HTTP_RETRY: usize = 3;
const RATE_LIMIT_BACKOFF_MS: u64 = 3000;
const LAS_LIMIT: usize = 1000;
const ANTARES_PAGE: usize = 1000;

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from("state")
}

fn token_from(key: &str) -> Option<String> {
    if let Ok(t) = std::env::var(key) {
        if !t.is_empty() {
            return Some(t);
        }
    }
    let body = std::fs::read_to_string(state_dir().join(".secrets.local")).ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn curl_get(url: &str, token: Option<&str>) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-A")
        .arg(UA)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    if let Some(t) = token {
        cmd.arg("-H").arg(format!("Authorization: Token {t}"));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn curl_post_json(url: &str, body: &str) -> Option<(String, Vec<u8>)> {
    for attempt in 0..HTTP_RETRY {
        let mut cmd = Command::new("curl");
        cmd.arg("-sS")
            .arg("-m")
            .arg("90")
            .arg("-A")
            .arg(UA)
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("-X")
            .arg("POST")
            .arg("-d")
            .arg(body)
            .arg("-o")
            .arg("-")
            .arg("-w")
            .arg("\n%{http_code}");
        cmd.arg(url);
        let out = cmd.output().ok()?;
        let stdout = out.stdout;
        let idx = stdout.iter().rposition(|&b| b == b'\n')?;
        let code = String::from_utf8_lossy(&stdout[idx + 1..])
            .trim()
            .to_string();
        if code != "429" {
            return Some((code, stdout[..idx].to_vec()));
        }
        let backoff = RATE_LIMIT_BACKOFF_MS * (attempt as u64 + 1);
        println!(
            "skydirection: Fink/LSST cone answered HTTP 429 — the endpoint asks for a slower pace; {backoff} ms before the next try (try {})",
            attempt + 1
        );
        std::thread::sleep(std::time::Duration::from_millis(backoff));
    }
    println!(
        "skydirection: Fink/LSST cone held HTTP 429 across {HTTP_RETRY} backed-off tries — the cone stays pending"
    );
    None
}

fn ra_dec_plausible(ra: f64, dec: f64) -> bool {
    ra.is_finite() && dec.is_finite() && (0.0..360.0).contains(&ra) && (-90.0..=90.0).contains(&dec)
}

fn jd_utc_to_tdb(lsk: &LeapSeconds, jd: f64) -> Option<f64> {
    lsk.unix_to_tdb((jd - 2440587.5) * 86400.0)
}

fn mjd_to_tdb(lsk: &LeapSeconds, mjd: f64) -> Option<f64> {
    lsk.unix_to_tdb((mjd - 40587.0) * 86400.0)
}

fn empty_direction(name: String, ra: f64, dec: f64) -> SkyDirection {
    SkyDirection {
        name,
        ra_deg: ra,
        dec_deg: dec,
        sigma_arcsec: None,
        bands: Vec::new(),
        distance: None,
        redshift: None,
    }
}

fn g_series(tdb: f64, mag: f64) -> SkyBandSeries {
    SkyBandSeries {
        band: Some("g".to_string()),
        samples: vec![SkySample { tdb, mag }],
    }
}

fn unbanded_series(samples: Vec<SkySample>) -> SkyBandSeries {
    SkyBandSeries {
        band: None,
        samples,
    }
}

fn lasair_window(lsk: &LeapSeconds, jd_start: f64) -> Vec<SkyDirection> {
    let Some(token) = token_from("LASAIR_TOKEN") else {
        println!(
            "skydirection: Lasair-ZTF — LASAIR_TOKEN absent (env or the .secrets.local key in the omegaflow state dir) — the window stays unqueried"
        );
        return Vec::new();
    };
    let selected = "objectId,ramean,decmean,gmag,jdmin,jdmax";
    let conditions = format!("jdmax%3E{jd_start}");
    let url = format!(
        "{LAS_QUERY}?selected={selected}&tables=objects&conditions={conditions}&limit={LAS_LIMIT}"
    );
    let Some((code, body)) = curl_get(&url, Some(&token)) else {
        println!(
            "skydirection: Lasair-ZTF window jdmax>{jd_start} did not answer (measured stall) — the window stays pending"
        );
        return Vec::new();
    };
    if code != "200" {
        println!(
            "skydirection: Lasair-ZTF window jdmax>{jd_start} answered HTTP {code} — the window stays pending"
        );
        return Vec::new();
    }
    let Ok(text) = std::str::from_utf8(&body) else {
        println!("skydirection: Lasair-ZTF window body is not UTF-8 — the parser stays pending");
        return Vec::new();
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        println!(
            "skydirection: Lasair-ZTF window body is not the measured row array — the parser stays pending"
        );
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut refused = 0usize;
    let mut anchored_g = 0usize;
    let mut multi_detection_g = 0usize;
    let mut epoch_void_g = 0usize;
    for r in &rows {
        let (Some(name), Some(ra), Some(dec)) =
            (jstr(r, "objectId"), jnum(r, "ramean"), jnum(r, "decmean"))
        else {
            refused += 1;
            continue;
        };
        if !ra_dec_plausible(ra, dec) {
            refused += 1;
            continue;
        }
        let mut d = empty_direction(name, ra, dec);
        match (jnum(r, "gmag"), jnum(r, "jdmin"), jnum(r, "jdmax")) {
            (Some(mag), Some(jd_min), Some(jd_max))
                if mag.is_finite() && jd_min.is_finite() && jd_max.is_finite() =>
            {
                if jd_min == jd_max {
                    match jd_utc_to_tdb(lsk, jd_max) {
                        Some(tdb) => {
                            d.bands.push(g_series(tdb, mag));
                            anchored_g += 1;
                        }
                        None => epoch_void_g += 1,
                    }
                } else {
                    multi_detection_g += 1;
                }
            }
            _ => {}
        }
        out.push(d);
    }
    let n = out.len();
    println!(
        "skydirection: Lasair-ZTF window jdmax>{jd_start} — HTTP {code}, {n} object row(s) held as directions; {anchored_g} carry the delivered gmag anchored to a single-detection epoch (jdmin==jdmax); {multi_detection_g} carry a g magnitude without an anchorable epoch (multi-detection object, the g-band epoch is not delivered); {epoch_void_g} carry a g magnitude whose JD does not fold onto the TDB clock (outside the leap table) — those magnitudes stay unheld (0 honored); {refused} row(s) refused (no name/position or out of the ICRS gate)"
    );
    out
}

fn antares_loci(lsk: &LeapSeconds, cap: Option<usize>) -> Vec<SkyDirection> {
    let cap = cap.unwrap_or(usize::MAX);
    let mut out: Vec<SkyDirection> = Vec::new();
    let mut offset = 0usize;
    let mut measured_total: Option<usize> = None;
    let mut refused = 0usize;
    let mut magnitude_samples = 0usize;
    loop {
        if out.len() >= cap {
            break;
        }
        let url =
            format!("{ANTARES_LOCI}?page%5Blimit%5D={ANTARES_PAGE}&page%5Boffset%5D={offset}");
        let Some((code, body)) = curl_get(&url, None) else {
            println!(
                "skydirection: ANTARES loci (offset {offset}) did not answer (measured stall) — the listing stays pending from this page on"
            );
            break;
        };
        if code != "200" {
            println!(
                "skydirection: ANTARES loci (offset {offset}) answered HTTP {code} — the listing stays pending from this page on"
            );
            break;
        }
        let Ok(text) = std::str::from_utf8(&body) else {
            println!(
                "skydirection: ANTARES loci (offset {offset}) body is not UTF-8 — the parser stays pending"
            );
            break;
        };
        let Some(JsonVal::Obj(root)) = parse_json(text) else {
            println!(
                "skydirection: ANTARES loci (offset {offset}) body is not the measured JSON:API object — the parser stays pending"
            );
            break;
        };
        if let Some(JsonVal::Obj(meta)) = root.get("meta") {
            if let Some(JsonVal::Num(count)) = meta.get("count") {
                if count.is_finite() && *count >= 0.0 {
                    measured_total = Some(*count as usize);
                }
            }
        }
        let Some(JsonVal::Arr(loci)) = root.get("data") else {
            println!(
                "skydirection: ANTARES loci (offset {offset}) body carries no data array — the parser stays pending"
            );
            break;
        };
        let page_len = loci.len();
        for item in loci {
            if out.len() >= cap {
                break;
            }
            let (Some(name), Some(ra), Some(dec)) = (
                jstr(item, "id"),
                jnum(item, "attributes.ra"),
                jnum(item, "attributes.dec"),
            ) else {
                refused += 1;
                continue;
            };
            if !ra_dec_plausible(ra, dec) {
                refused += 1;
                continue;
            }
            let mag_epochs = [
                ("newest_alert_magnitude", "newest_alert_observation_time"),
                ("oldest_alert_magnitude", "oldest_alert_observation_time"),
                (
                    "brightest_alert_magnitude",
                    "brightest_alert_observation_time",
                ),
            ];
            let mut samples: Vec<SkySample> = Vec::new();
            for (mag_key, time_key) in mag_epochs {
                let mag_path = format!("attributes.properties.{mag_key}");
                let time_path = format!("attributes.properties.{time_key}");
                let (Some(mag), Some(mjd)) = (jnum(item, &mag_path), jnum(item, &time_path)) else {
                    continue;
                };
                if !mag.is_finite() || !mjd.is_finite() {
                    continue;
                }
                let Some(tdb) = mjd_to_tdb(lsk, mjd) else {
                    continue;
                };
                let dup = samples.iter().any(|s| s.tdb == tdb && s.mag == mag);
                if !dup {
                    samples.push(SkySample { tdb, mag });
                }
            }
            let mut d = empty_direction(name, ra, dec);
            if !samples.is_empty() {
                magnitude_samples += samples.len();
                d.bands.push(unbanded_series(samples));
            }
            out.push(d);
        }
        if page_len < ANTARES_PAGE {
            break;
        }
        offset += page_len;
        if let Some(total) = measured_total {
            if offset >= total {
                break;
            }
        }
    }
    let n = out.len();
    println!(
        "skydirection: ANTARES loci — measured stock {measured_total:?}, {n} locus/loci held as directions; {magnitude_samples} delivered alert-magnitude sample(s) held unbanded (the loci listing delivers no passband), epoch-anchored by the delivered observation times; {refused} locus/loci refused (no id/position or out of the ICRS gate); the pagination runs page[limit]={ANTARES_PAGE} page[offset]={offset} to the measured stock end"
    );
    out
}

fn extract_id_tokens(body: &[u8]) -> Vec<String> {
    let Ok(text) = std::str::from_utf8(body) else {
        return Vec::new();
    };
    let needle = "\"r:diaObjectId\":";
    let mut out: Vec<String> = Vec::new();
    let mut pos = 0;
    while let Some(rel) = text[pos..].find(needle) {
        let s = pos + rel + needle.len();
        let mut digits = String::new();
        for c in text[s..].chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else {
                break;
            }
        }
        if !digits.is_empty() {
            let consumed = digits.len();
            out.push(digits);
            pos = s + consumed;
        } else {
            pos = s + 1;
        }
    }
    out
}

fn fink_cone(ra: f64, dec: f64, radius_as: f64) -> Vec<SkyDirection> {
    if !ra_dec_plausible(ra, dec) || !radius_as.is_finite() || radius_as <= 0.0 {
        println!(
            "skydirection: Fink/LSST cone ({ra}, {dec}, {radius_as} arcsec) refused — the position or radius is not plausible; no cone runs"
        );
        return Vec::new();
    }
    let payload = format!(
        "{{\"ra\": {ra}, \"dec\": {dec}, \"radius\": {radius_as}, \"columns\": \"r:diaObjectId,r:ra,r:dec\"}}"
    );
    let Some((code, body)) = curl_post_json(FINK_CONE, &payload) else {
        println!(
            "skydirection: Fink/LSST cone ({ra}, {dec}, {radius_as} arcsec) did not answer — the cone stays pending"
        );
        return Vec::new();
    };
    if code != "200" {
        println!(
            "skydirection: Fink/LSST cone ({ra}, {dec}, {radius_as} arcsec) answered HTTP {code} — the cone stays pending"
        );
        return Vec::new();
    }
    let Ok(text) = std::str::from_utf8(&body) else {
        println!("skydirection: Fink/LSST cone body is not UTF-8 — the parser stays pending");
        return Vec::new();
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        println!(
            "skydirection: Fink/LSST cone body is not the measured row array — the parser stays pending"
        );
        return Vec::new();
    };
    let ids = extract_id_tokens(&body);
    if ids.len() != rows.len() {
        println!(
            "skydirection: Fink/LSST cone ({ra}, {dec}, {radius_as} arcsec) — {} row(s) but {} r:diaObjectId token(s) — the id alignment stays pending",
            rows.len(),
            ids.len()
        );
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut refused = 0usize;
    for (idx, r) in rows.iter().enumerate() {
        let (Some(ra_v), Some(dec_v)) = (jnum(r, "r:ra"), jnum(r, "r:dec")) else {
            refused += 1;
            continue;
        };
        if !ra_dec_plausible(ra_v, dec_v) {
            refused += 1;
            continue;
        }
        out.push(empty_direction(ids[idx].clone(), ra_v, dec_v));
    }
    let n = out.len();
    println!(
        "skydirection: Fink/LSST cone ({ra}, {dec}, {radius_as} arcsec) — HTTP {code}, {n} diaObject row(s) held as directions; the cone rows deliver no em photometry (r:ra/r:dec/r:diaObjectId only) — the magnitudes stay absent, the per-object light curves stay a named pending (0 honored); {refused} row(s) refused (no position or out of the ICRS gate)"
    );
    out
}

fn alerce_probe() {
    let Some((code, body)) = curl_get(ALERCE_OBJECTS, None) else {
        println!(
            "skydirection: ALeRCE api.alerce.online/objects did not answer (measured stall) — unreachable, excluded, never a negative"
        );
        return;
    };
    println!(
        "skydirection: ALeRCE api.alerce.online/objects answered HTTP {code} with {} byte(s) — the direct-database stub is the retired surface (dead_sources.φ: Direct database access is being retired); unreachable, excluded, never a negative",
        body.len()
    );
}

fn push_unique(out: &mut Vec<SkyDirection>, incoming: Vec<SkyDirection>) -> usize {
    let mut added = 0usize;
    for d in incoming {
        if out.iter().any(|x| x.name == d.name) {
            continue;
        }
        out.push(d);
        added += 1;
    }
    added
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_path: Option<String> = None;
    let mut ci = false;
    let mut lasair_jd: Option<f64> = None;
    let mut antares = false;
    let mut antares_cap: Option<usize> = None;
    let mut cones: Vec<(f64, f64, f64)> = Vec::new();
    let mut alerce = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci = true,
            "--lasair-window" => {
                let v = args
                    .get(i + 1)
                    .and_then(|s| s.parse::<f64>().ok())
                    .filter(|v| v.is_finite());
                if let Some(jd) = v {
                    lasair_jd = Some(jd);
                } else {
                    println!("skydirection: --lasair-window carries no finite JD — the window stays closed");
                }
                i += 1;
            }
            "--antares" => antares = true,
            "--antares-limit" => {
                let v = args.get(i + 1).and_then(|s| s.parse::<usize>().ok());
                match v {
                    Some(n) if n > 0 => {
                        antares = true;
                        antares_cap = Some(n);
                    }
                    _ => println!("skydirection: --antares-limit carries no positive count — the cap stays closed"),
                }
                i += 1;
            }
            "--fink-cone" => {
                let mut ra_dec_r: Option<(f64, f64, f64)> = None;
                let mut j = i + 1;
                let mut tokens: Vec<f64> = Vec::new();
                while j < args.len() && tokens.len() < 3 {
                    if args[j].starts_with("--") {
                        break;
                    }
                    if let Ok(v) = args[j].parse::<f64>() {
                        if v.is_finite() {
                            tokens.push(v);
                        }
                    }
                    j += 1;
                }
                if tokens.len() == 3 {
                    ra_dec_r = Some((tokens[0], tokens[1], tokens[2]));
                }
                if let Some(c) = ra_dec_r {
                    cones.push(c);
                } else {
                    println!("skydirection: --fink-cone needs ra dec radius-arcsec — the cone stays closed");
                }
                i = j - 1;
            }
            "--alerce" => alerce = true,
            other => {
                println!(
                    "skydirection_compiler: unknown argument {other} — refused. usage:\n  \
                     --out <sky_directions.bin> [--ci-mode]\n  \
                     --lasair-window <jd_start>   Lasair-ZTF objects window (LASAIR_TOKEN)\n  \
                     --antares                    ANTARES ZTF loci listing, full measured stock (anonymous)\n  \
                     --antares-limit <N>          cap the ANTARES harvest at N held loci\n  \
                     --fink-cone <ra> <dec> <radius-arcsec>  Fink-LSST diaObject cone (anonymous, repeatable)\n  \
                     --alerce                     read api.alerce.online/objects and name the measured code"
                );
                return;
            }
        }
        i += 1;
    }
    let Some(out) = out_path else {
        println!("skydirection_compiler: --out absent — the asset path is never silent");
        return;
    };
    if alerce {
        alerce_probe();
    }
    let harvests_epochs = lasair_jd.is_some() || antares;
    let harvests_any = harvests_epochs || !cones.is_empty();
    if !harvests_any {
        println!("skydirection_compiler: no harvest source selected (--lasair-window | --antares | --fink-cone) — the probe runs, the asset stays unwritten");
        return;
    }
    let mut directions: Vec<SkyDirection> = Vec::new();
    if harvests_epochs {
        if let Some(lsk) = embedded_lsk() {
            if let Some(jd_start) = lasair_jd {
                let added = push_unique(&mut directions, lasair_window(&lsk, jd_start));
                println!("skydirection: Lasair-ZTF window added {added} new direction(s)");
            }
            if antares {
                let added = push_unique(&mut directions, antares_loci(&lsk, antares_cap));
                println!("skydirection: ANTARES added {added} new direction(s)");
            }
        } else {
            println!("skydirection: the embedded naif0012.tls leap table is absent — no sample epoch folds to the TDB clock; the epoch-bearing harvests stay unrun (0 honored, pending)");
        }
    }
    for (ra, dec, radius_as) in cones {
        let added = push_unique(&mut directions, fink_cone(ra, dec, radius_as));
        println!("skydirection: Fink/LSST cone added {added} new direction(s)");
    }
    if directions.is_empty() {
        println!(
            "skydirection_compiler: no direction was held — the asset stays unwritten (0 honored)"
        );
        return;
    }
    let Some(bytes) = write_bin(&directions) else {
        println!("skydirection_compiler: a held direction is not finite or not serializable — the asset stays unwritten (0 honored)");
        return;
    };
    match omegaflow::skydirection::parse_bin(&bytes) {
        Some(parsed) if parsed.len() == directions.len() => {
            let magnitude_rows: usize = parsed.iter().map(|d| d.bands.len()).sum();
            println!(
                "skydirection_compiler: {out} holds {} direction record(s), {} band series across them, {} bytes — the roundtrip reads back",
                parsed.len(),
                magnitude_rows,
                bytes.len()
            );
        }
        _ => {
            println!("skydirection_compiler: the roundtrip does not read back — the asset stays unwritten");
            return;
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        println!("skydirection_compiler: write {out} returned void — the asset stays unwritten");
        return;
    }
    if ci && !upload_asset(&out) {
        println!("skydirection_compiler: {out} did not reach the CDN — the local asset stands, the manifest is pending");
    }
}
