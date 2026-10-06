use omegaflow::cdn::upload_release;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const NETLOC: &str = "onetcenter.org";
const URL: &str = "https://www.onetcenter.org/dl_files/database/db_31_0_csv/occupation_data.csv";
const ASSET: &str = "onet_occupation_data.csv";

const MISSING_AXIS: &str = "no time axis: occupation_data.csv is a static dimension table \
(O*NET-SOC Code, Title, Description) with neither a temporal column nor a numeric element \
column. The value-bearing O*NET elements (work context, work styles, abilities, skills) live \
in separate files keyed by (O*NET-SOC Code, Element ID, Scale ID); the axis that would carry \
an exposure value is the occupation-element-scale tuple, never epoch. Epoch-value emission \
stays absent (0 honored) until the element file and its axis contract are designed; the raw \
table is manifested as-is.";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn temp_path() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .ok();
    let name = match stamp {
        Some(nanos) => format!("onet_{}_{}.csv", std::process::id(), nanos),
        None => format!("onet_{}.csv", std::process::id()),
    };
    std::env::temp_dir().join(name)
}

fn download(path: &Path) -> Option<()> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("-m")
        .arg("300")
        .arg("-o")
        .arg(path)
        .arg(URL)
        .output()
        .ok()?;
    if out.status.success() {
        Some(())
    } else {
        eprintln!(
            "onet_compiler: {URL} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn split_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    fields.push(cur);
    fields
}

fn field<'a>(row: &'a [String], i: usize) -> &'a str {
    match row.get(i) {
        Some(v) => v.as_str(),
        None => "",
    }
}

fn is_numeric(s: &str) -> bool {
    match s.trim().parse::<f64>() {
        Ok(v) => v.is_finite(),
        Err(_) => false,
    }
}

fn numeric_columns(header: &[String], sample: &[String]) -> Vec<usize> {
    header
        .iter()
        .enumerate()
        .filter(|(i, _)| match sample.get(*i) {
            Some(v) => is_numeric(v),
            None => false,
        })
        .map(|(i, _)| i)
        .collect()
}

fn time_column(header: &[String]) -> Option<usize> {
    header.iter().position(|c| {
        let lower = c.to_ascii_lowercase();
        lower.contains("date")
            || lower.contains("time")
            || lower.contains("year")
            || lower.contains("epoch")
    })
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .join(ASSET)
            .to_string_lossy()
            .into_owned(),
    };

    let tmp = temp_path();
    if download(&tmp).is_none() {
        return Err(format!("{URL}: fetch void — no asset (0 honored)"));
    }
    let bytes = std::fs::read(&tmp).map_err(|e| format!("read {} returned {e}", tmp.display()))?;
    let _ = std::fs::remove_file(&tmp);

    let text = String::from_utf8_lossy(&bytes);
    let data_rows = text.lines().count().saturating_sub(1);
    let mut lines = text.lines();
    let header_line = lines
        .next()
        .ok_or("occupation_data.csv carries no header row")?;
    let header = split_fields(header_line);
    let sample_line = lines
        .next()
        .ok_or("occupation_data.csv carries no data row")?;
    let sample = split_fields(sample_line);

    eprintln!("onet_compiler: {URL}");
    eprintln!(
        "onet_compiler: schema {} column(s), {data_rows} data row(s), {} B",
        header.len(),
        bytes.len()
    );
    for (i, name) in header.iter().enumerate() {
        eprintln!("onet_compiler:   column[{i}] = {name}");
    }
    eprintln!(
        "onet_compiler: sample column[0] = {} | column[1] = {} | column[2] = {} char(s)",
        field(&sample, 0),
        field(&sample, 1),
        field(&sample, 2).len()
    );

    let numeric = numeric_columns(&header, &sample);
    eprintln!("onet_compiler: numeric element columns = {numeric:?}");
    match time_column(&header) {
        Some(i) => eprintln!(
            "onet_compiler: time axis column[{i}] = {}",
            field(&header, i)
        ),
        None => {
            eprintln!("onet_compiler: no time axis in the header — epoch-value emission undefined");
            eprintln!("onet_compiler: {MISSING_AXIS}");
        }
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned {e}"))?;
    eprintln!(
        "onet_compiler: raw table written to {out}; 0 epoch-value rows (no time axis, 0 honored)"
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("onet_compiler: {msg}");
        std::process::exit(2);
    }
}
