use omegaflow::archivar::geo::{
    parse_anr, write_anr, AnrRec, COMP_ANR_BRIGHTEST, COMP_ANR_NEWEST, COMP_ANR_OLDEST,
};
use omegaflow::archivar::{embedded_lsk, LeapSeconds};
use omegaflow::json::{jnum, parse_json, JsonVal};
use std::process::Command;

const UA: &str = "omegaflow-antares-loci-compiler/1.0";
const ANTARES_LOCI: &str = "https://api.antares.noirlab.edu/v1/loci";
const PAGE_SIZE: usize = 100;

fn curl_get(url: &str) -> Option<(String, Vec<u8>)> {
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
    cmd.arg(url);
    let out = cmd.output().ok()?;
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

fn mjd_to_tdb(lsk: &LeapSeconds, mjd: f64) -> Option<f64> {
    lsk.unix_to_tdb((mjd - 40587.0) * 86400.0)
}

fn locus_records(lsk: &LeapSeconds, item: &JsonVal, start: f64, end: f64) -> Option<Vec<AnrRec>> {
    let (Some(ra), Some(dec)) = (jnum(item, "attributes.ra"), jnum(item, "attributes.dec")) else {
        return None;
    };
    if !ra_dec_plausible(ra, dec) {
        return None;
    }
    let flavors = [
        (
            COMP_ANR_NEWEST,
            "newest_alert_magnitude",
            "newest_alert_observation_time",
        ),
        (
            COMP_ANR_OLDEST,
            "oldest_alert_magnitude",
            "oldest_alert_observation_time",
        ),
        (
            COMP_ANR_BRIGHTEST,
            "brightest_alert_magnitude",
            "brightest_alert_observation_time",
        ),
    ];
    let mut out = Vec::new();
    for (comp, mag_key, time_key) in flavors {
        let mag_path = format!("attributes.properties.{mag_key}");
        let time_path = format!("attributes.properties.{time_key}");
        let (Some(mag), Some(mjd)) = (jnum(item, &mag_path), jnum(item, &time_path)) else {
            continue;
        };
        if !mag.is_finite() || !mjd.is_finite() {
            continue;
        }
        if mjd < start || mjd > end {
            continue;
        }
        let Some(tdb) = mjd_to_tdb(lsk, mjd) else {
            continue;
        };
        out.push(AnrRec {
            t: tdb,
            ra,
            dec,
            mag,
            comp,
        });
    }
    Some(out)
}

fn locus_carries(item: &JsonVal) -> (bool, bool, bool) {
    let newest =
        jnum(item, "attributes.properties.newest_alert_magnitude").is_some_and(|v| v.is_finite());
    let oldest =
        jnum(item, "attributes.properties.oldest_alert_magnitude").is_some_and(|v| v.is_finite());
    let brightest = jnum(item, "attributes.properties.brightest_alert_magnitude")
        .is_some_and(|v| v.is_finite());
    (newest, oldest, brightest)
}

fn fetch_window(lsk: &LeapSeconds, start: f64, end: f64, limit: usize) -> Vec<AnrRec> {
    let mut records: Vec<AnrRec> = Vec::new();
    let mut scanned = 0usize;
    let mut refused = 0usize;
    let mut windowed = 0usize;
    let mut pages = 0usize;
    let (mut newest_carried, mut oldest_carried, mut brightest_carried) = (0usize, 0usize, 0usize);
    let mut offset = 0usize;
    while scanned < limit {
        let ps = (limit - scanned).min(PAGE_SIZE);
        let url = format!("{ANTARES_LOCI}?page%5Blimit%5D={ps}&page%5Boffset%5D={offset}");
        let Some((code, body)) = curl_get(&url) else {
            println!(
                "antares: loci page offset {offset} did not answer (measured stall) — the page stays unharvested, the scan stops"
            );
            break;
        };
        if code != "200" {
            println!(
                "antares: loci page offset {offset} answered HTTP {code} — the page stays unharvested, the scan stops"
            );
            break;
        }
        let Ok(text) = std::str::from_utf8(&body) else {
            println!("antares: loci page offset {offset} body is not UTF-8 — the parser stays pending, the scan stops");
            break;
        };
        let Some(JsonVal::Obj(root)) = parse_json(text) else {
            println!(
                "antares: loci page offset {offset} body is not the measured JSON:API object — the parser stays pending, the scan stops"
            );
            break;
        };
        let Some(JsonVal::Arr(loci)) = root.get("data") else {
            println!(
                "antares: loci page offset {offset} body carries no data array — the parser stays pending, the scan stops"
            );
            break;
        };
        pages += 1;
        let page_len = loci.len();
        for item in loci {
            let (n, o, b) = locus_carries(item);
            if n {
                newest_carried += 1;
            }
            if o {
                oldest_carried += 1;
            }
            if b {
                brightest_carried += 1;
            }
            match locus_records(lsk, item, start, end) {
                Some(samples) if !samples.is_empty() => {
                    windowed += 1;
                    records.extend(samples);
                }
                Some(_) => {}
                None => refused += 1,
            }
        }
        scanned += page_len;
        if page_len < ps {
            break;
        }
        offset += page_len;
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then_with(|| a.comp.cmp(&b.comp)));
    println!(
        "antares: loci window MJD [{start}, {end}] — scanned {scanned} locus/loci across {pages} page(s); {windowed} locus/loci landed sample(s) in the window; field coverage across the scan — newest magnitude on {newest_carried} locus/loci, oldest on {oldest_carried}, brightest on {brightest_carried}; {refused} locus/loci refused (no ra/dec or out of the ICRS gate)"
    );
    records
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_path: Option<String> = None;
    let mut start: Option<f64> = None;
    let mut end: Option<f64> = None;
    let mut limit: usize = 1000;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--start" => {
                start = args
                    .get(i + 1)
                    .and_then(|s| s.parse::<f64>().ok())
                    .filter(|v| v.is_finite());
                i += 1;
            }
            "--end" => {
                end = args
                    .get(i + 1)
                    .and_then(|s| s.parse::<f64>().ok())
                    .filter(|v| v.is_finite());
                i += 1;
            }
            "--limit" => {
                if let Some(n) = args.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                    limit = n;
                }
                i += 1;
            }
            other => {
                println!(
                    "antares_loci_compiler: unknown argument {other} — refused. usage:\n  \
                     --out <antares_loci.bin> --start <mjd> --end <mjd> [--limit N]\n  \
                     window: MJD inclusive bounds; --limit caps the loci scanned (no server-side time filter)"
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
    let (Some(start), Some(end)) = (start, end) else {
        println!("antares_loci_compiler: --start/--end (MJD) absent — the window is never silent");
        return;
    };
    if start > end {
        println!("antares_loci_compiler: --start {start} lies after --end {end} — the window stays closed");
        return;
    }
    if limit == 0 {
        println!(
            "antares_loci_compiler: --limit carries no positive count — the scan stays closed"
        );
        return;
    }
    let Some(lsk) = embedded_lsk() else {
        println!(
            "antares_loci_compiler: the embedded naif0012.tls leap table is absent — no sample epoch folds to the TDB clock; the harvest stays unrun (0 honored, pending)"
        );
        return;
    };
    let records = fetch_window(&lsk, start, end, limit);
    if records.is_empty() {
        println!(
            "antares_loci_compiler: no locus sample fell in the window — the asset stays unwritten (0 honored)"
        );
        return;
    }
    let bytes = write_anr(&records);
    match parse_anr(&bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            let newest = records.iter().filter(|r| r.comp == COMP_ANR_NEWEST).count();
            let oldest = records.iter().filter(|r| r.comp == COMP_ANR_OLDEST).count();
            let brightest = records
                .iter()
                .filter(|r| r.comp == COMP_ANR_BRIGHTEST)
                .count();
            println!(
                "antares_loci_compiler: {out} holds {} locus sample record(s) (newest {newest}, oldest {oldest}, brightest {brightest}), {} bytes — the roundtrip reads back",
                parsed.len(),
                bytes.len()
            );
        }
        _ => {
            println!("antares_loci_compiler: the roundtrip does not read back — the asset stays unwritten");
            return;
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        println!("antares_loci_compiler: write {out} returned void — the asset stays unwritten");
    }
}
