use omegaflow::archivar::aia::MAGIC;
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::{JsonVal, parse_json};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{LeapSeconds, days_from_civil};
use std::collections::VecDeque;
use std::process::Command;
use std::sync::{Arc, Mutex, mpsc};

const TAG: &str = "jsoc.stanford.edu";
const BASE: &str = "http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_info";

const CHANNELS: [(i64, u32); 7] = [
    (94, 0),
    (131, 1),
    (171, 2),
    (193, 3),
    (211, 4),
    (304, 5),
    (335, 6),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_day(s: &str) -> Option<(i64, i64, i64)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    Some((
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
    ))
}

fn unix_of(y: i64, m: i64, d: i64, hh: f64, mm: f64, ss: f64) -> Option<f64> {
    Some(days_from_civil(y, m, d)? as f64 * 86400.0 + hh * 3600.0 + mm * 60.0 + ss)
}

fn parse_iso_t_rec(s: &str, lsk: &LeapSeconds) -> Option<f64> {
    let s = s.trim_end_matches('Z');
    let (d, t) = s.split_once('T')?;
    let dp: Vec<&str> = d.split('-').collect();
    let tp: Vec<&str> = t.split(':').collect();
    if dp.len() != 3 || tp.len() != 3 {
        return None;
    }
    let y: i64 = dp[0].parse().ok()?;
    let mo: i64 = dp[1].parse().ok()?;
    let da: i64 = dp[2].parse().ok()?;
    let hh: f64 = tp[0].parse().ok()?;
    let mm: f64 = tp[1].parse().ok()?;
    let ss: f64 = tp[2].parse().ok()?;
    lsk.unix_to_tdb(unix_of(y, mo, da, hh, mm, ss)?)
}

fn fetch_channel(start: &str, wave: i64) -> Option<JsonVal> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("300")
        .arg("-G")
        .arg("--data-urlencode")
        .arg(format!(
            "ds=aia.lev1_euv_12s[{start}/31d@1h][? WAVELNTH={wave} ?]"
        ))
        .arg("--data-urlencode")
        .arg("op=rs_list")
        .arg("--data-urlencode")
        .arg("key=DATAMEAN,T_REC")
        .arg(BASE)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    parse_json(&String::from_utf8_lossy(&out.stdout))
}

fn keyword_values(root: &JsonVal, name: &str) -> Option<Vec<String>> {
    let JsonVal::Obj(obj) = root else {
        return None;
    };
    let JsonVal::Arr(keywords) = obj.get("keywords")? else {
        return None;
    };
    for kw in keywords {
        let JsonVal::Obj(entry) = kw else {
            continue;
        };
        let Some(JsonVal::Str(found)) = entry.get("name") else {
            continue;
        };
        if found != name {
            continue;
        }
        let JsonVal::Arr(values) = entry.get("values")? else {
            return None;
        };
        return Some(
            values
                .iter()
                .filter_map(|v| match v {
                    JsonVal::Str(s) => Some(s.clone()),
                    JsonVal::Num(n) => Some(n.to_string()),
                    _ => None,
                })
                .collect(),
        );
    }
    None
}

fn write_aia(
    records: &mut Vec<(f64, f64, u32)>,
    out_path: &str,
    ci_mode: bool,
) -> Result<(), String> {
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    records.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
    if records.is_empty() {
        return Err("no DATAMEAN record — the bin stays unwritten (0 honored)".into());
    }
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, c) in records.iter() {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
    }
    std::fs::write(out_path, &buf).map_err(|e| format!("write {out_path} returned void: {e}"))?;
    eprintln!(
        "aia_rs_list_compiler: {} records ({} B) -> {out_path}",
        records.len(),
        buf.len()
    );
    if ci_mode && !upload_release(TAG, out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let from = arg_value(args, "--from").ok_or("--from YYYY-MM-DD absent — refused")?;
    let to = arg_value(args, "--to").ok_or("--to YYYY-MM-DD absent — refused")?;
    let out_path = arg_value(args, "--out").ok_or("--out <aia.bin> absent — refused")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let lsk = embedded_lsk()
        .ok_or("the embedded leap-second table stays unread — T_REC stays unassigned")?;
    let (fy, fm, fd) = parse_day(&from).ok_or(format!("{from}: not a day — refused"))?;
    let (ty, tm, td) = parse_day(&to).ok_or(format!("{to}: not a day — refused"))?;
    let t_lo = lsk
        .unix_to_tdb(unix_of(fy, fm, fd, 0.0, 0.0, 0.0).ok_or("--from stays unassigned")?)
        .ok_or("--from stays unassigned")?;
    let t_hi = lsk
        .unix_to_tdb(unix_of(ty, tm, td, 23.0, 59.0, 59.0).ok_or("--to stays unassigned")?)
        .ok_or("--to stays unassigned")?;

    let jobs: usize = arg_value(args, "--jobs")
        .ok_or("--jobs <n> absent — refused")?
        .parse()
        .map_err(|_| "--jobs carries no number — refused")?;
    if jobs == 0 {
        return Err("--jobs 0 carries no worker — refused".into());
    }

    let mut tasks: Vec<(i64, u32, String)> = Vec::new();
    let (mut y, mut m) = (fy, fm);
    while (y, m) <= (ty, tm) {
        let start = format!("{y:04}.{m:02}.01");
        for (wave, comp) in CHANNELS {
            tasks.push((wave, comp, start.clone()));
        }
        if m == 12 {
            y += 1;
            m = 1;
        } else {
            m += 1;
        }
    }
    eprintln!("aia: {} monthly tasks, {jobs} workers", tasks.len());

    let queue = Arc::new(Mutex::new(VecDeque::from(tasks)));
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..jobs {
        let queue = Arc::clone(&queue);
        let tx = tx.clone();
        let lsk = lsk.clone();
        handles.push(std::thread::spawn(move || {
            loop {
                let task = queue.lock().ok().and_then(|mut q| q.pop_front());
                let Some((wave, comp, start)) = task else {
                    break;
                };
                let Some(root) = fetch_channel(&start, wave) else {
                    eprintln!("{wave} {start}: rs_list carries void");
                    continue;
                };
                let (Some(values), Some(t_recs)) = (
                    keyword_values(&root, "DATAMEAN"),
                    keyword_values(&root, "T_REC"),
                ) else {
                    eprintln!("{wave} {start}: DATAMEAN/T_REC absent");
                    continue;
                };
                let rows = values.len().min(t_recs.len());
                let mut batch = Vec::new();
                for i in 0..rows {
                    let Ok(value) = values[i].parse::<f64>() else {
                        continue;
                    };
                    if !value.is_finite() {
                        continue;
                    }
                    let Some(t) = parse_iso_t_rec(&t_recs[i], &lsk) else {
                        continue;
                    };
                    if t < t_lo || t > t_hi {
                        continue;
                    }
                    batch.push((t, value, comp));
                }
                if !batch.is_empty() {
                    tx.send(batch).ok();
                }
            }
        }));
    }
    drop(tx);
    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    for batch in rx {
        records.extend(batch);
    }
    for handle in handles {
        handle.join().ok();
    }
    write_aia(&mut records, &out_path, ci_mode)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("aia_rs_list_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso_t_rec_lands_in_tdb() {
        let lsk = embedded_lsk().unwrap();
        let tdb = parse_iso_t_rec("2013-01-01T00:00:01Z", &lsk).unwrap();
        assert!((tdb - 410270468.184).abs() < 1.0);
    }

    #[test]
    fn keywords_are_read_from_the_rs_list_shape() {
        let s = r#"{"keywords":[{"name":"DATAMEAN","values":["35.5","2.0"]},{"name":"T_REC","values":["2013-01-01T00:00:01Z","2013-01-01T01:00:01Z"]}]}"#;
        let root = parse_json(s).unwrap();
        assert_eq!(keyword_values(&root, "DATAMEAN").unwrap().len(), 2);
        assert_eq!(
            keyword_values(&root, "T_REC").unwrap()[0],
            "2013-01-01T00:00:01Z"
        );
    }
}
