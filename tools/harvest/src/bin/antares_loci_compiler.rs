use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, jnum, jstr, parse_json};
use omegaflow::skydirection::{SkyDirection, parse_bin, write_bin};
use std::process::Command;

const UA: &str = "omegaflow-antares-loci-compiler/1.0";
const NETLOC: &str = "antares.noirlab.edu";
const ANTARES_LOCI: &str = "https://api.antares.noirlab.edu/v1/loci";
const ANTARES_PAGE: usize = 1000;

fn curl_get(url: &str) -> Option<(String, Vec<u8>)> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-A")
        .arg(UA)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url)
        .output()
        .ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn ra_dec_plausible(ra: f64, dec: f64) -> bool {
    ra.is_finite() && dec.is_finite() && (0.0..360.0).contains(&ra) && (-90.0..=90.0).contains(&dec)
}

fn empty_direction(name: String, ra: f64, dec: f64) -> SkyDirection {
    SkyDirection {
        name,
        ra_deg: ra,
        dec_deg: dec,
        sigma_arcsec: None,
        bands: Vec::new(),
        flux_bands: Vec::new(),
        distance: None,
        redshift: None,
    }
}

fn antares_loci(cap: Option<usize>) -> Vec<SkyDirection> {
    let cap = cap.unwrap_or(usize::MAX);
    let mut out: Vec<SkyDirection> = Vec::new();
    let mut offset = 0usize;
    let mut measured_total: Option<usize> = None;
    let mut refused = 0usize;
    let mut htm16_held = 0usize;
    loop {
        if out.len() >= cap {
            break;
        }
        let url =
            format!("{ANTARES_LOCI}?page%5Blimit%5D={ANTARES_PAGE}&page%5Boffset%5D={offset}");
        let Some((code, body)) = curl_get(&url) else {
            println!(
                "antares_loci: ANTARES loci (offset {offset}) did not answer (measured stall) — the listing stays pending from this page on"
            );
            break;
        };
        if code != "200" {
            println!(
                "antares_loci: ANTARES loci (offset {offset}) answered HTTP {code} — the listing stays pending from this page on"
            );
            break;
        }
        let Ok(text) = std::str::from_utf8(&body) else {
            println!(
                "antares_loci: ANTARES loci (offset {offset}) body is not UTF-8 — the parser stays pending"
            );
            break;
        };
        let Some(JsonVal::Obj(root)) = parse_json(text) else {
            println!(
                "antares_loci: ANTARES loci (offset {offset}) body is not the measured JSON:API object — the parser stays pending"
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
                "antares_loci: ANTARES loci (offset {offset}) body carries no data array — the parser stays pending"
            );
            break;
        };
        let page_len = loci.len();
        for item in loci {
            if out.len() >= cap {
                break;
            }
            let name =
                jstr(item, "attributes.properties.ztf_object_id").or_else(|| jstr(item, "id"));
            let (Some(name), Some(ra), Some(dec)) = (
                name,
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
            if jnum(item, "attributes.htm16").is_some() {
                htm16_held += 1;
            }
            out.push(empty_direction(name, ra, dec));
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
        "antares_loci: ANTARES loci — measured stock {measured_total:?}, {n} locus/loci held as directions; {htm16_held} carry the delivered attributes.htm16 cell (a position cross-check, not a SKD1 slot); {refused} locus/loci refused (no ztf_object_id/id or position, or out of the ICRS gate); the pagination runs page[limit]={ANTARES_PAGE} page[offset]={offset} to the measured stock end"
    );
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out_path: Option<String> = None;
    let mut ci = false;
    let mut cap: Option<usize> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci = true,
            "--limit" => {
                let v = args.get(i + 1).and_then(|s| s.parse::<usize>().ok());
                match v {
                    Some(n) if n > 0 => cap = Some(n),
                    _ => println!(
                        "antares_loci: --limit carries no positive count — the full measured stock is read"
                    ),
                }
                i += 1;
            }
            other => {
                println!(
                    "antares_loci_compiler: unknown argument {other} — refused. usage:\n  \
                     --out <antares_loci.bin> [--limit <N>] [--ci-mode]"
                );
                return;
            }
        }
        i += 1;
    }
    let Some(out) = out_path else {
        println!("antares_loci_compiler: --out absent — the asset path is never silent");
        return;
    };
    let directions = antares_loci(cap);
    if directions.is_empty() {
        println!(
            "antares_loci_compiler: no locus was held — the asset stays unwritten (0 honored)"
        );
        return;
    }
    let Some(bytes) = write_bin(&directions) else {
        println!(
            "antares_loci_compiler: a held direction is not finite or not serializable — the asset stays unwritten (0 honored)"
        );
        return;
    };
    match parse_bin(&bytes) {
        Some(parsed) if parsed.len() == directions.len() => {
            println!(
                "antares_loci_compiler: {out} holds {} direction record(s), {} bytes — the roundtrip reads back",
                parsed.len(),
                bytes.len()
            );
        }
        _ => {
            println!(
                "antares_loci_compiler: the roundtrip does not read back — the asset stays unwritten"
            );
            return;
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        println!("antares_loci_compiler: write {out} returned void — the asset stays unwritten");
        return;
    }
    if ci && !upload_release(NETLOC, &out) {
        println!(
            "antares_loci_compiler: {out} did not reach the CDN — the local asset stands, the manifest is pending"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_icrs_gate_holds_measured_directions_and_refuses_the_rest() {
        assert!(ra_dec_plausible(37.284397, 9.258595));
        assert!(ra_dec_plausible(0.0, -90.0));
        assert!(!ra_dec_plausible(-1.0, 0.0));
        assert!(!ra_dec_plausible(360.0, 0.0));
        assert!(!ra_dec_plausible(0.0, 91.0));
        assert!(!ra_dec_plausible(f64::NAN, 0.0));
    }

    #[test]
    fn a_held_direction_roundtrips_through_the_skd1_writer() {
        let d = empty_direction("ZTF21abxxjrh".to_string(), 37.284397, 9.258595);
        let bytes = write_bin(&[d]).expect("a finite direction serializes");
        let back = parse_bin(&bytes).expect("the written bytes read back");
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].name, "ZTF21abxxjrh");
        assert_eq!(back[0].ra_deg, 37.284397);
        assert_eq!(back[0].dec_deg, 9.258595);
    }
}
