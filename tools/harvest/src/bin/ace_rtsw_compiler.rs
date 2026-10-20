use omegaflow::cdn::upload_release;
use std::collections::BTreeMap;
use std::process::Command;

const BASE: &str = "https://cdaweb.gsfc.nasa.gov/hapi/data";
const TAG: &str = "cdaweb.gsfc.nasa.gov";
const FILL_ABS: f64 = 1e30;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_csv(id: &str, params: &str, tmin: &str, tmax: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("--fail-with-body")
        .arg("-m")
        .arg("600")
        .arg("-G")
        .arg("--data-urlencode")
        .arg(format!("id={id}"))
        .arg("--data-urlencode")
        .arg(format!("time.min={tmin}"))
        .arg("--data-urlencode")
        .arg(format!("time.max={tmax}"))
        .arg("--data-urlencode")
        .arg(format!("parameters={params}"))
        .arg("--data-urlencode")
        .arg("format=csv")
        .arg(BASE)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        None
    }
}

fn next_month(y: i64, m: i64) -> (i64, i64) {
    if m < 12 { (y, m + 1) } else { (y + 1, 1) }
}

fn parse_month(s: &str) -> Option<(i64, i64)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() < 2 {
        return None;
    }
    Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
}

fn bucket_csv(text: &str, nvals: usize, buckets: &mut BTreeMap<String, (Vec<f64>, usize)>) {
    for line in text.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < nvals + 1 || parts[0].len() < 13 || !parts[0].contains('T') {
            continue;
        }
        let Some(values) = (0..nvals)
            .map(|i| parts.get(i + 1).and_then(|v| v.trim().parse::<f64>().ok()))
            .collect::<Option<Vec<f64>>>()
        else {
            continue;
        };
        if values.iter().any(|v| !v.is_finite() || v.abs() >= FILL_ABS) {
            continue;
        }
        let key = parts[0][..13].to_string();
        let entry = buckets.entry(key).or_insert_with(|| (vec![0.0; nvals], 0));
        for (sum, v) in entry.0.iter_mut().zip(values.iter()) {
            *sum += v;
        }
        entry.1 += 1;
    }
}

fn emit_mfi(buckets: &BTreeMap<String, (Vec<f64>, usize)>) -> String {
    let mut out = String::from(
        "{\n\"HAPI\": \"2.0\",\n\"status\": {\"code\": 1200, \"message\": \"OK\"},\n\"format\": \"json\",\n\"parameters\": [\n\
         {\"name\": \"Time\", \"type\": \"isotime\", \"units\": \"UTC\", \"length\": 24, \"fill\": null},\n\
         {\"name\": \"Magnitude\", \"type\": \"double\", \"units\": \"nT\", \"fill\": \"-1.0E31\"},\n\
         {\"name\": \"BGSM\", \"type\": \"double\", \"units\": \"nT\", \"fill\": \"-1.0E31\", \"size\": [3]}\n\
         ],\n\"data\":[\n",
    );
    let mut first = true;
    for (key, (sums, n)) in buckets {
        if *n == 0 {
            continue;
        }
        let f = *n as f64;
        if !first {
            out.push_str(",\n");
        }
        first = false;
        out.push_str(&format!(
            "[\"{key}:00:00.000Z\",{:.6},[{:.6},{:.6},{:.6}]]",
            sums[0] / f,
            sums[1] / f,
            sums[2] / f,
            sums[3] / f
        ));
    }
    out.push_str("\n]}\n");
    out
}

fn emit_swe(buckets: &BTreeMap<String, (Vec<f64>, usize)>) -> String {
    let mut out = String::from(
        "{\n\"HAPI\": \"2.0\",\n\"status\": {\"code\": 1200, \"message\": \"OK\"},\n\"format\": \"json\",\n\"parameters\": [\n\
         {\"name\": \"Time\", \"type\": \"isotime\", \"units\": \"UTC\", \"length\": 24, \"fill\": null},\n\
         {\"name\": \"Np\", \"type\": \"double\", \"units\": \"1/cm3\", \"fill\": \"-1.0E31\"},\n\
         {\"name\": \"Vp\", \"type\": \"double\", \"units\": \"km/s\", \"fill\": \"-1.0E31\"}\n\
         ],\n\"data\":[\n",
    );
    let mut first = true;
    for (key, (sums, n)) in buckets {
        if *n == 0 {
            continue;
        }
        let f = *n as f64;
        if !first {
            out.push_str(",\n");
        }
        first = false;
        out.push_str(&format!(
            "[\"{key}:00:00.000Z\",{:.6},{:.6}]",
            sums[0] / f,
            sums[1] / f
        ));
    }
    out.push_str("\n]}\n");
    out
}

fn run(args: &[String]) -> Result<(), String> {
    let dataset = arg_value(args, "--dataset").ok_or("--dataset mfi|swe — refused")?;
    let (id, params, nvals) = match dataset.as_str() {
        "mfi" => ("AC_H0_MFI", "Magnitude,BGSM", 4usize),
        "swe" => ("AC_K0_SWE", "Np,Vp", 2usize),
        _ => return Err("--dataset mfi|swe — refused".into()),
    };
    let from = arg_value(args, "--from").ok_or("--from YYYY-MM-DD absent — refused")?;
    let to = arg_value(args, "--to").ok_or("--to YYYY-MM-DD absent — refused")?;
    let out_path = arg_value(args, "--out")
        .ok_or("usage: ace_rtsw_compiler --dataset mfi|swe --out <asset.json> [--from YYYY-MM-DD] [--to YYYY-MM-DD] [--ci-mode] — refused")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let (mut y, mut m) = parse_month(&from).ok_or("--from is not YYYY-MM-DD — refused")?;
    let (ty, tm) = parse_month(&to).ok_or("--to is not YYYY-MM-DD — refused")?;
    let to_day: u32 = to
        .get(8..10)
        .and_then(|s| s.parse().ok())
        .ok_or("--to is not YYYY-MM-DD — refused")?;

    let mut buckets: BTreeMap<String, (Vec<f64>, usize)> = BTreeMap::new();
    let mut months = 0usize;
    loop {
        let next = next_month(y, m);
        let tmin = format!("{y:04}-{m:02}-01T00:00:00Z");
        let tmax = if (y, m) == (ty, tm) {
            format!("{ty:04}-{tm:02}-{to_day:02}T00:00:00Z")
        } else {
            format!("{:04}-{:02}-01T00:00:00Z", next.0, next.1)
        };
        if (y, m) > (ty, tm) {
            break;
        }
        let Some(text) = fetch_csv(id, params, &tmin, &tmax) else {
            return Err(format!("{id} {tmin}..{tmax} carries void"));
        };
        bucket_csv(&text, nvals, &mut buckets);
        months += 1;
        eprintln!("{id}: {tmin}..{tmax} ok ({months})");
        if (y, m) == (ty, tm) {
            break;
        }
        (y, m) = next;
    }
    if buckets.is_empty() {
        return Err(format!(
            "{id}: no finite hourly bucket — the asset stays unwritten"
        ));
    }
    let body = if dataset == "mfi" {
        emit_mfi(&buckets)
    } else {
        emit_swe(&buckets)
    };
    std::fs::write(&out_path, body.as_bytes())
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    eprintln!(
        "ace_rtsw_compiler: {dataset} -> {out_path} ({} hourly rows, {} B)",
        buckets.len(),
        body.len()
    );
    if ci_mode && !upload_release(TAG, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ace_rtsw_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hourly_mean_and_fill_filter() {
        let csv = "2020-01-01T00:00:12.000Z,2.0,1.0,2.0,3.0\n\
                   2020-01-01T00:00:28.000Z,4.0,3.0,4.0,5.0\n\
                   2020-01-01T00:10:00.000Z,-1.0E31,1.0,1.0,1.0\n\
                   2020-01-01T01:00:00.000Z,9.0,9.0,9.0,9.0\n";
        let mut b = BTreeMap::new();
        bucket_csv(csv, 4, &mut b);
        assert_eq!(b.len(), 2);
        let (sums, n) = &b["2020-01-01T00"];
        assert_eq!(*n, 2);
        assert!((sums[0] - 6.0).abs() < 1e-12);
        assert!((sums[3] - 8.0).abs() < 1e-12);
    }

    #[test]
    fn emit_keeps_the_hapi_parameter_names() {
        let mut b = BTreeMap::new();
        b.insert("2020-01-01T00".to_string(), (vec![2.0, 1.0, 2.0, 3.0], 1));
        let s = emit_mfi(&b);
        assert!(s.contains("\"Magnitude\""));
        assert!(s.contains("\"BGSM\""));
        assert!(s.contains("2020-01-01T00:00:00.000Z"));
    }
}
