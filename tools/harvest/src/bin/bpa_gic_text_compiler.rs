use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "transmission.bpa.gov";
const URL: &str = "https://transmission.bpa.gov/business/operations/gic/gic.txt";
const MIN_DATA_ROWS: usize = 100;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("-m")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "bpa_gic_text_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn is_data_row(line: &str) -> bool {
    let mut fields = line.split_whitespace();
    let Some(stamp) = fields.next() else {
        return false;
    };
    let Some(clock) = fields.next() else {
        return false;
    };
    let mut parts = stamp.split('/');
    let (Some(month), Some(day), Some(year), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    let numeric = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if !numeric(month) || !numeric(day) || year.len() != 4 || !numeric(year) {
        return false;
    }
    let mut clock_parts = clock.split(':');
    let (Some(hour), Some(minute), None) =
        (clock_parts.next(), clock_parts.next(), clock_parts.next())
    else {
        return false;
    };
    if !numeric(hour) || !numeric(minute) {
        return false;
    }
    let tail: Vec<&str> = fields.collect();
    !tail.is_empty()
        && tail
            .iter()
            .all(|v| v.parse::<f64>().map(|x| x.is_finite()).unwrap_or(false))
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/bpa_gic.txt"),
    };

    let bytes = curl(URL).ok_or_else(|| format!("{URL}: fetch void"))?;
    let text = String::from_utf8(bytes.clone()).map_err(|_| format!("{URL}: body is not UTF-8"))?;
    let data_rows = text.lines().filter(|line| is_data_row(line)).count();
    if data_rows < MIN_DATA_ROWS {
        return Err(format!(
            "{URL} carries {data_rows} data rows, fewer than {MIN_DATA_ROWS} — the asset stays unwritten"
        ));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "bpa_gic_text_compiler: {data_rows} data rows copied verbatim ({} B, sha256 {}) to {out}",
        bytes.len(),
        sha256_hex(&bytes)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("bpa_gic_text_compiler: {msg}");
        std::process::exit(2);
    }
}
