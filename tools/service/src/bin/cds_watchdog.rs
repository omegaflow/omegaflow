use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const CDN_BASE: &str = "https://github.com/omegaflow/sources/releases/download";

const ASSETS: [(&str, &str); 10] = [
    ("jwst_spectra.bin", "ssd.jpl.nasa.gov"),
    ("spectra.bin", "ssd.jpl.nasa.gov"),
    ("nvss.json", "ssd.jpl.nasa.gov"),
    ("first14.json", "ssd.jpl.nasa.gov"),
    ("chandra_csc.json", "ssd.jpl.nasa.gov"),
    ("ztf_lightcurves.bin", "irsa.ipac.caltech.edu"),
    ("ztf_lightcurves_fresh.bin", "irsa.ipac.caltech.edu"),
    ("dr3_stars.bin", "ssd.jpl.nasa.gov"),
    ("twomass_psc.bin", "irsa.ipac.caltech.edu"),
    ("ephemeris_sun.bin", "ssd.jpl.nasa.gov"),
];

fn main() {
    let mut statuses = Vec::new();
    for (asset, release) in ASSETS {
        let url = format!("{}/{}/{}", CDN_BASE, release, asset);
        let code = match Command::new("curl")
            .arg("-sIL")
            .arg("-o")
            .arg("/dev/null")
            .arg("-w")
            .arg("%{http_code}")
            .arg("--max-time")
            .arg("20")
            .arg(&url)
            .output()
        {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            _ => "void".to_string(),
        };
        statuses.push(format!("{}={}", asset, code));
    }
    let Some(now) = epoch() else {
        return;
    };
    let line = report_line(now, &statuses.join(" "));
    append_report(&reports_dir().join("cds_watchdog.φ"), &line);
    println!("{}", line.trim_end());
}

fn reports_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir).join("reports");
    }
    std::path::PathBuf::from("state").join("reports")
}

fn report_line(epoch: u64, statuses: &str) -> String {
    format!("cds_watchdog | {} | {}\n", epoch, statuses)
}

fn epoch() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .ok()
}

fn append_report<P: AsRef<std::path::Path>>(path: P, line: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        use std::io::Write;
        let _ = f.write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_format() {
        let line = report_line(1787600000, "spectra.bin=200 nvss.json=404");
        assert!(line.starts_with("cds_watchdog | 1787600000 | "));
        assert!(line.ends_with("nvss.json=404\n"));
    }

    #[test]
    fn append_creates_file() {
        let path = "/tmp/cds_test_report.φ";
        let _ = std::fs::remove_file(path);
        append_report(path, "cds_watchdog | 1 | a=200\n");
        append_report(path, "cds_watchdog | 2 | a=404\n");
        let text = std::fs::read_to_string(path).unwrap();
        assert_eq!(text.lines().count(), 2);
        let _ = std::fs::remove_file(path);
    }
}
