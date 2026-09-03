use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::doppler::{parse_bin, write_bin};
use omegaflow::inflate::gunzip;
use omegaflow::spectral::civil_from_days;

const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/pioneer";
const SOURCES: &[(&str, &str)] = &[("pioneer10", "SC_23"), ("pioneer11", "SC_24")];
const KU_HZ: f64 = 1.0e10;
const J2000_EPOCH: f64 = 2451545.0;
const JD_UNIX_EPOCH: f64 = 2440587.5;

fn fields(line: &str) -> Vec<(String, String)> {
    let toks: Vec<&str> = line.split_whitespace().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        let t = toks[i];
        if let Some(eq) = t.find('=') {
            let key = &t[..eq];
            let val_after = &t[eq + 1..];
            if !val_after.is_empty() {
                out.push((key.to_string(), val_after.to_string()));
                i += 1;
            } else if i + 1 < toks.len() {
                out.push((key.to_string(), toks[i + 1].to_string()));
                i += 2;
            } else {
                i += 1;
            }
        } else if i + 2 < toks.len() && toks[i + 1] == "=" {
            out.push((t.to_string(), toks[i + 2].to_string()));
            i += 3;
        } else {
            i += 1;
        }
    }
    out
}

fn num(f: &[(String, String)], key: &str) -> Option<f64> {
    f.iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
}

fn jd_date(tdb_s: f64) -> String {
    let jd = J2000_EPOCH + tdb_s / 86400.0;
    let unix_day = (jd - JD_UNIX_EPOCH).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0} s"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    for (name, sc) in SOURCES {
        let url = format!("{BASE}/{name}/radio/{name}_doppler_tracking_{sc}.asc.gz");
        let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
            eprintln!("{name}: fetch void ({url})");
            continue;
        };
        let Some(text_bytes) = gunzip(&bytes) else {
            eprintln!("{name}: gunzip void");
            continue;
        };
        let text = String::from_utf8_lossy(&text_bytes);
        let mut records: Vec<[f64; 6]> = Vec::new();
        let mut dtype = f64::NAN;
        let mut sc_num = f64::NAN;
        let mut ku_corrected = 0usize;
        for line in text.lines() {
            let f = fields(line);
            if let Some(d) = num(&f, "DTYPE") {
                dtype = d;
            }
            if let Some(s) = num(&f, "SC") {
                sc_num = s;
            }
            let (Some(timtag), Some(obs), Some(freq), Some(cmptime)) = (
                num(&f, "TIMTAG"),
                num(&f, "OBSVBL"),
                num(&f, "FREQCY"),
                num(&f, "CMPTIM"),
            ) else {
                continue;
            };
            let freq = if freq > KU_HZ {
                ku_corrected += 1;
                (freq + 7.0e9) * 96.0 / 1000.0
            } else {
                freq
            };
            records.push([timtag, obs, freq, cmptime, dtype, sc_num]);
        }
        if records.is_empty() {
            eprintln!("{name}: no records — the series stays unwritten (0 honored)");
            continue;
        }
        let out = format!("data/{name}_doppler.bin");
        std::fs::create_dir_all("data").ok();
        let bin = write_bin(&records);
        if std::fs::write(&out, &bin).is_err() {
            eprintln!("{name}: write {out} void");
            continue;
        }
        match parse_bin(&bin) {
            Some(parsed) => {
                let d0 = parsed[0];
                let d1 = parsed[parsed.len() - 1];
                let dtype_set: Vec<i64> = {
                    let mut v: Vec<i64> = parsed.iter().map(|r| r[4] as i64).collect();
                    v.sort_unstable();
                    v.dedup();
                    v
                };
                eprintln!(
                    "{out}: {} records ({}..{}), doppler {:.0}..{:.0} Hz, freq {:.3e}..{:.3e} Hz, {} ku-korrigiert, DTYPE {:?}, {} B — roundtrip parses",
                    parsed.len(),
                    jd_date(d0[0]),
                    jd_date(d1[0]),
                    d1[1],
                    d0[1],
                    d1[2],
                    d0[2],
                    ku_corrected,
                    dtype_set,
                    bin.len()
                );
            }
            None => {
                eprintln!("{out}: roundtrip parse void — the series stays unverified");
                continue;
            }
        }
        if ci_mode && !upload_release("spdf.gsfc.nasa.gov", &out) {
            std::process::exit(1);
        }
    }
}
