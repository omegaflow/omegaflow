use omegaflow::json::{parse_json, JsonVal};
use std::collections::{BTreeSet, HashMap};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const UA: &str = "omegaflow-pair-te-fetch/1.0";
const FINK_FP: &str = "https://api.lsst.fink-portal.org/api/v1/fp";
const OBJECT_KEY: &str = "r:diaObjectId";
const RA_KEY: &str = "r:ra";
const DEC_KEY: &str = "r:dec";
const NDIA_KEY: &str = "r:nDiaSources";
const SEP_KEY: &str = "v:separation_degree";
const BAND_KEY: &str = "r:band";
const VISIT_KEY: &str = "r:visit";
const MJD_KEY: &str = "r:midpointMjdTai";
const FLUX_KEY: &str = "r:scienceFlux";
const HTTP_RETRY: usize = 3;
const BACKOFF_MS: u64 = 3000;
const DEFAULT_OUT: &str = "tmp";
const DEFAULT_MIN_SHARED: usize = 30;
const DEFAULT_MAX_OBJECTS: usize = 24;
const DEFAULT_PAUSE_MS: u64 = 500;
const DEFAULT_BANDS: &str = "g,r,i,u,z,y";

type VisitMap = HashMap<i64, (f64, f64)>;
type Coverage = HashMap<String, VisitMap>;

fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}

fn obj_str<'a>(m: &'a HashMap<String, JsonVal>, key: &str) -> Option<&'a str> {
    match m.get(key) {
        Some(JsonVal::Str(s)) => Some(s),
        _ => None,
    }
}

fn obj_f64(m: &HashMap<String, JsonVal>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(JsonVal::Num(n)) if n.is_finite() => Some(*n),
        _ => None,
    }
}

struct ConeObject {
    id: String,
    ra: f64,
    dec: f64,
    n_dia: Option<usize>,
    sep_deg: Option<f64>,
}

fn scan_object_ids(text: &str) -> Vec<String> {
    let needle = "\"r:diaObjectId\":";
    let mut out = Vec::new();
    let mut rest: &str = text;
    while let Some(rel) = rest.find(needle) {
        let tail = &rest[rel + needle.len()..];
        let trimmed = tail.trim_start();
        if let Some(quoted) = trimmed.strip_prefix('"') {
            let Some(end) = quoted.find('"') else {
                break;
            };
            out.push(quoted[..end].to_string());
            let consumed = needle.len() + (tail.len() - trimmed.len()) + 1 + end;
            rest = &rest[rel + consumed..];
        } else {
            let digits = trimmed.bytes().take_while(|b| b.is_ascii_digit()).count();
            if digits == 0 {
                break;
            }
            out.push(trimmed[..digits].to_string());
            let consumed = needle.len() + (tail.len() - trimmed.len()) + digits;
            rest = &rest[rel + consumed..];
        }
    }
    out
}

fn parse_cone_objects(body: &[u8]) -> Option<Vec<ConeObject>> {
    let text = std::str::from_utf8(body).ok()?;
    let rows = match parse_json(text) {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let with_key = rows
        .iter()
        .filter(|r| matches!(r, JsonVal::Obj(m) if m.contains_key(OBJECT_KEY)))
        .count();
    let ids = scan_object_ids(text);
    if ids.len() != with_key {
        return None;
    }
    let mut it = ids.into_iter();
    let mut out = Vec::new();
    for r in &rows {
        let JsonVal::Obj(m) = r else {
            continue;
        };
        if !m.contains_key(OBJECT_KEY) {
            continue;
        }
        let Some(id) = it.next() else {
            return None;
        };
        let (Some(ra), Some(dec)) = (obj_f64(m, RA_KEY), obj_f64(m, DEC_KEY)) else {
            return None;
        };
        let n_dia = obj_f64(m, NDIA_KEY).map(|n| n as usize);
        let sep_deg = obj_f64(m, SEP_KEY);
        out.push(ConeObject {
            id,
            ra,
            dec,
            n_dia,
            sep_deg,
        });
    }
    Some(out)
}

struct FpRows {
    total: usize,
    cov: Coverage,
    neg: HashMap<String, usize>,
}

fn parse_fp_body(body: &[u8]) -> Option<FpRows> {
    let text = std::str::from_utf8(body).ok()?;
    let parsed = parse_json(text)?;
    let arr = match parsed {
        JsonVal::Arr(a) => a,
        JsonVal::Obj(m) => {
            let mut found = None;
            for k in ["rows", "data", "array"] {
                if let Some(JsonVal::Arr(a)) = m.get(k) {
                    found = Some(a.to_vec());
                    break;
                }
            }
            found?
        }
        _ => return None,
    };
    let mut cov = Coverage::new();
    let mut neg = HashMap::new();
    for r in &arr {
        let JsonVal::Obj(m) = r else {
            continue;
        };
        let (Some(band), Some(visit), Some(mjd), Some(flux)) = (
            obj_str(m, BAND_KEY),
            obj_f64(m, VISIT_KEY),
            obj_f64(m, MJD_KEY),
            obj_f64(m, FLUX_KEY),
        ) else {
            continue;
        };
        cov.entry(band.to_string())
            .or_default()
            .insert(visit as i64, (mjd, flux));
        if flux <= 0.0 {
            *neg.entry(band.to_string()).or_insert(0) += 1;
        }
    }
    Some(FpRows {
        total: arr.len(),
        cov,
        neg,
    })
}

fn curl_post_json(url: &str, body: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("120")
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
    Some((code, stdout[..idx].to_vec()))
}

fn curl_post_rate_aware(url: &str, body: &str, who: &str) -> Option<(String, Vec<u8>)> {
    for attempt in 0..HTTP_RETRY {
        let Some(resp) = curl_post_json(url, body) else {
            return None;
        };
        if resp.0 != "429" {
            return Some(resp);
        }
        let backoff = BACKOFF_MS * (attempt as u64 + 1);
        println!(
            "{who}: HTTP 429 — the endpoint asks for a slower pace; {backoff} ms before the next try (try {})",
            attempt + 1
        );
        sleep_ms(backoff);
    }
    println!(
        "{who}: HTTP 429 held across {HTTP_RETRY} backed-off tries — the rate limit stands, the fetch stays pending"
    );
    None
}

fn fetch_coverage(id: &str) -> Option<FpRows> {
    let payload = format!("{{\"diaObjectId\": \"{id}\"}}");
    match curl_post_rate_aware(FINK_FP, &payload, &format!("fp {id}")) {
        Some((code, body)) if code == "200" => match parse_fp_body(&body) {
            Some(rows) => Some(rows),
            None => {
                println!("fp {id}: the 200 body is not a measured fp row list — pending");
                None
            }
        },
        Some((code, _)) => {
            println!("fp {id}: http {code}");
            None
        }
        None => None,
    }
}

fn choose_band(
    cov_all: &HashMap<String, Coverage>,
    order: &[String],
    bands: &[String],
    min_shared: usize,
) -> Option<(String, Vec<String>, BTreeSet<i64>)> {
    for b in bands {
        let cov_len = |oid: &String| cov_all[oid][b].len();
        let mut present: Vec<String> = order
            .iter()
            .filter(|oid| {
                cov_all[*oid]
                    .get(b)
                    .map_or(false, |vm| vm.len() >= min_shared)
            })
            .cloned()
            .collect();
        if present.len() < 2 {
            continue;
        }
        present.sort_by(|x, y| cov_len(y).cmp(&cov_len(x)));
        let mut chosen = vec![present[0].clone()];
        let mut shared: BTreeSet<i64> = cov_all[&present[0]][b].keys().copied().collect();
        for o in &present[1..] {
            let theirs: BTreeSet<i64> = cov_all[o][b].keys().copied().collect();
            let cand: BTreeSet<i64> = shared.intersection(&theirs).copied().collect();
            if cand.len() >= min_shared {
                chosen.push(o.clone());
                shared = cand;
            }
        }
        if chosen.len() >= 2 && shared.len() >= min_shared {
            chosen.sort_by(|x, y| cov_len(y).cmp(&cov_len(x)));
            return Some((b.clone(), chosen, shared));
        }
    }
    None
}

fn median_of_sorted(v: &[f64]) -> Option<f64> {
    let n = v.len();
    if n == 0 {
        return None;
    }
    if n % 2 == 1 {
        Some(v[n / 2])
    } else {
        Some(0.5 * (v[n / 2 - 1] + v[n / 2]))
    }
}

fn usage() {
    eprintln!(
        "pair_te_fetch — the forced-photometry data pull for the pair transfer-entropy screen:\n\
         reads a Fink cone object list, fetches Fink/LSST /api/v1/fp for the nDiaSources-ranked objects,\n\
         selects the band and the object subset that share >= min-shared visits, writes one flux column\n\
         per object (shared visits only) and the manifest that pair_te_screen --set consumes:\n\
         \x20 pair_te_fetch --cone <cone_raw.json> [--out <dir>] [--min-shared <n>]\n\
         \x20              [--max-objects <n>] [--pause-ms <n>] [--bands g,r,i,u,z,y]\n\
         live fetch is anonymous (api.lsst.fink-portal.org), no token"
    );
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == name {
            return Some(args[i + 1].clone());
        }
        i += 1;
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let Some(cone_path) = arg_value(&args, "--cone") else {
        eprintln!("--cone <cone_raw.json> absent");
        std::process::exit(2);
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let min_shared = arg_value(&args, "--min-shared")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_MIN_SHARED);
    let max_objects = arg_value(&args, "--max-objects")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_MAX_OBJECTS);
    let pause_ms = arg_value(&args, "--pause-ms")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_PAUSE_MS);
    let bands: Vec<String> = match arg_value(&args, "--bands") {
        Some(v) => v,
        None => DEFAULT_BANDS.to_string(),
    }
    .split(',')
    .map(str::to_string)
    .collect();

    let body = match std::fs::read(&cone_path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("the cone file at {cone_path} is not readable — the pull stays pending");
            std::process::exit(2);
        }
    };
    let Some(mut objects) = parse_cone_objects(&body) else {
        eprintln!("the cone body at {cone_path} is not a measured object row list — pending");
        std::process::exit(2);
    };
    objects.sort_by(|a, b| match (a.n_dia, b.n_dia) {
        (Some(x), Some(y)) => y.cmp(&x),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });
    println!("candidates_by_ndia {}", objects.len());
    let mut seps: Vec<f64> = objects.iter().filter_map(|o| o.sep_deg).collect();
    if seps.is_empty() {
        println!("cone_separation_deg: no row carries v:separation_degree (absent)");
    } else {
        seps.sort_by(f64::total_cmp);
        println!(
            "cone_separation_deg_max {:.4} min {:.4}",
            seps[seps.len() - 1],
            seps[0]
        );
    }
    let fetch_up_to = max_objects.min(objects.len());
    println!("will_fetch_up_to {fetch_up_to}");
    objects.truncate(fetch_up_to);

    let mut cov_all: HashMap<String, Coverage> = HashMap::new();
    let mut fetched_order: Vec<String> = Vec::new();
    for o in &objects {
        match fetch_coverage(&o.id) {
            Some(rows) => {
                let mut counts: Vec<(String, usize)> = rows
                    .cov
                    .iter()
                    .map(|(b, vm)| (b.clone(), vm.len()))
                    .collect();
                counts.sort();
                let count_words: Vec<String> =
                    counts.iter().map(|(b, n)| format!("{b} {n}")).collect();
                println!(
                    "fp {} rows {} band_visits {} neg_flux_rows {}",
                    o.id,
                    rows.total,
                    count_words.join(", "),
                    rows.neg.len()
                );
                cov_all.insert(o.id.clone(), rows.cov);
                fetched_order.push(o.id.clone());
            }
            None => {
                println!("fp {} left without coverage — skipped", o.id);
            }
        }
        sleep_ms(pause_ms);
    }

    let Some((band, subset, shared)) = choose_band(&cov_all, &fetched_order, &bands, min_shared)
    else {
        println!(
            "NO_VALID_SUBSET shared_visits>={min_shared} across {} fetched object(s)",
            fetched_order.len()
        );
        std::process::exit(3);
    };
    let id_meta: HashMap<&str, &ConeObject> = objects.iter().map(|o| (o.id.as_str(), o)).collect();
    let mut midmap: HashMap<i64, f64> = HashMap::new();
    let mut manifest_lines: Vec<String> = Vec::new();
    for oid in &subset {
        for v in &shared {
            let (mjd, _flux) = cov_all[oid][&band][v];
            midmap.entry(*v).or_insert(mjd);
        }
        let csv_path = format!("{out}/fp_{band}_{oid}.csv");
        let mut csv = String::new();
        for v in &shared {
            let (_mjd, flux) = cov_all[oid][&band][v];
            csv.push_str(&format!("{flux}\n"));
        }
        if std::fs::write(&csv_path, &csv).is_err() {
            println!("the flux column was not written ({csv_path})");
        }
        let obj = id_meta[oid.as_str()];
        manifest_lines.push(format!("{csv_path} {oid} {:.8} {:.8} ?", obj.ra, obj.dec));
    }

    let mut mids: Vec<f64> = shared.iter().map(|v| midmap[v]).collect();
    mids.sort_by(f64::total_cmp);
    let mut dts: Vec<f64> = mids.windows(2).map(|w| (w[1] - w[0]) * 86400.0).collect();
    let cadence_s = if dts.is_empty() {
        None
    } else {
        dts.sort_by(f64::total_cmp);
        median_of_sorted(&dts)
    };
    let timespan_s = if mids.len() >= 2 {
        Some((mids[mids.len() - 1] - mids[0]) * 86400.0)
    } else {
        None
    };

    let subset_words: Vec<String> = subset
        .iter()
        .map(|oid| format!("({oid}, {})", cov_all[oid][&band].len()))
        .collect();
    let cad_word = match cadence_s {
        Some(c) => format!("{c:.1}"),
        None => "absent".to_string(),
    };
    let span_word = match timespan_s {
        Some(s) => format!("{s:.0}"),
        None => "absent".to_string(),
    };
    println!(
        "band {band} subset {} shared {} cadence_median_s {cad_word} timespan_s {span_word}",
        subset_words.join(" "),
        shared.len()
    );

    let manifest_path = format!("{out}/pair_te_manifest_{band}.manifest");
    let manifest_body = format!("{}\n", manifest_lines.join("\n"));
    if std::fs::write(&manifest_path, &manifest_body).is_err() {
        println!("the manifest was not written ({manifest_path})");
    }
    println!("manifest {manifest_path}");
    print!("{manifest_body}");
    match cadence_s {
        Some(c) => println!("cadence_s={c:.3}"),
        None => println!("cadence_s=absent"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_id_scan_reads_numeric_and_quoted_ids() {
        let numeric = br#"[{"r:diaObjectId":313998569858662581,"r:nDiaSources":1206}]"#;
        let text = std::str::from_utf8(numeric).unwrap();
        assert_eq!(
            scan_object_ids(text),
            vec!["313998569858662581".to_string()]
        );
        let quoted = br#"[{"r:diaObjectId":"313998569858662581"}]"#;
        let text = std::str::from_utf8(quoted).unwrap();
        assert_eq!(
            scan_object_ids(text),
            vec!["313998569858662581".to_string()]
        );
    }

    #[test]
    fn cone_parser_reads_the_measured_rows_without_id_precision_loss() {
        let body = br#"[{"r:dec":2.5208155633,"r:diaObjectId":313998569858662581,"r:midpointMjdTai":61205.9837394019,"r:nDiaSources":1206,"r:ra":148.8746049917,"v:separation_degree":0.0006046066},{"r:dec":2.5238002919,"r:diaObjectId":313972184676565044,"r:midpointMjdTai":61051.1974338807,"r:nDiaSources":1,"r:ra":148.8724228661,"v:separation_degree":0.0033888472}]"#;
        let objs = parse_cone_objects(body).expect("the measured cone parses");
        assert_eq!(objs.len(), 2);
        assert_eq!(objs[0].id, "313998569858662581");
        assert_eq!(objs[0].n_dia, Some(1206));
        assert_eq!(objs[0].sep_deg, Some(0.0006046066));
        assert!((objs[0].ra - 148.8746049917).abs() < 1e-9);
        assert_eq!(objs[1].id, "313972184676565044");
    }

    #[test]
    fn fp_parser_keeps_negative_flux_and_counts_it() {
        let body = br#"[{"r:band":"g","r:visit":1,"r:scienceFlux":-320.1,"r:midpointMjdTai":61204.9817515752},{"r:band":"g","r:visit":2,"r:scienceFlux":0.0,"r:midpointMjdTai":61205.9837394019},{"r:band":"r","r:visit":3,"r:scienceFlux":5100.0,"r:midpointMjdTai":61206.0},{"r:detector":7,"r:visit":4}]"#;
        let rows = parse_fp_body(body).expect("the measured fp rows parse");
        assert_eq!(rows.total, 4);
        assert_eq!(rows.cov["g"].len(), 2);
        assert_eq!(rows.cov["g"][&1].0, 61204.9817515752);
        assert_eq!(rows.cov["g"][&1].1, -320.1);
        assert_eq!(rows.cov["r"].len(), 1);
        assert_eq!(rows.neg["g"], 2);
    }

    #[test]
    fn fp_parser_reads_the_wrapped_row_schema() {
        let body = br#"{"rows":[{"r:band":"g","r:visit":1,"r:scienceFlux":100.0,"r:midpointMjdTai":61204.0}]}"#;
        let rows = parse_fp_body(body).expect("the wrapped rows parse");
        assert_eq!(rows.total, 1);
        assert_eq!(rows.cov["g"][&1].1, 100.0);
    }

    #[test]
    fn choose_band_returns_the_shared_visit_subset() {
        let make_cov = |visits: &[i64]| -> Coverage {
            let mut c = Coverage::new();
            let vm: VisitMap = visits.iter().map(|&v| (v, (v as f64, 100.0))).collect();
            c.insert("g".to_string(), vm);
            c
        };
        let mut cov_all = HashMap::new();
        let a = make_cov(&(0..40).collect::<Vec<i64>>());
        let b = make_cov(&(5..45).collect::<Vec<i64>>());
        let c = make_cov(&(10..50).collect::<Vec<i64>>());
        cov_all.insert("A".to_string(), a);
        cov_all.insert("B".to_string(), b);
        cov_all.insert("C".to_string(), c);
        let order = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let bands = vec!["g".to_string()];
        let (band, subset, shared) =
            choose_band(&cov_all, &order, &bands, 30).expect("the shared-visit subset stands");
        assert_eq!(band, "g");
        assert_eq!(subset.len(), 3);
        assert_eq!(shared.len(), 30);
        assert!(shared.iter().next().is_some_and(|v| *v == 10));
    }

    #[test]
    fn choose_band_stays_absent_when_no_band_meets_the_shared_floor() {
        let make_cov = |visits: &[i64]| -> Coverage {
            let mut c = Coverage::new();
            let vm: VisitMap = visits.iter().map(|&v| (v, (v as f64, 100.0))).collect();
            c.insert("g".to_string(), vm);
            c
        };
        let mut cov_all = HashMap::new();
        cov_all.insert("A".to_string(), make_cov(&(0..40).collect::<Vec<i64>>()));
        cov_all.insert("B".to_string(), make_cov(&(0..10).collect::<Vec<i64>>()));
        let order = vec!["A".to_string(), "B".to_string()];
        let bands = vec!["g".to_string()];
        assert!(choose_band(&cov_all, &order, &bands, 30).is_none());
    }
}
