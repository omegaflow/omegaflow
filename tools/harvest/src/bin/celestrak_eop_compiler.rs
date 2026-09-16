use omegaflow::archivar::celestrak_eop::{COMP_PMX, COMP_PMY, COMP_UT1_UTC, parse_bin, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;
use std::process::Command;

const NETLOC: &str = "celestrak.org";
const URL: &str = "https://celestrak.org/SpaceData/EOP-All.csv";
const MJD_UNIX_EPOCH: f64 = 40587.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn parse_row(columns: &[&str]) -> Option<(f64, Option<f64>, Option<f64>, Option<f64>, bool)> {
    if columns.len() < 12 {
        return None;
    }
    let mjd: f64 = columns[1].trim().parse().ok()?;
    if !mjd.is_finite() {
        return None;
    }
    let num = |i: usize| -> Option<f64> {
        let t = columns[i].trim();
        if t.is_empty() {
            return None;
        }
        t.parse::<f64>().ok().filter(|v| v.is_finite())
    };
    let observed = columns[11].trim() == "O";
    Some((mjd, num(2), num(3), num(4), observed))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_bin = match arg_value(&args, "--emit-bin") {
        Some(v) => v,
        None => {
            eprintln!("--emit-bin <path> required");
            std::process::exit(1);
        }
    };
    let text = match arg_value(&args, "--input") {
        Some(p) => match std::fs::read_to_string(&p) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("--input {p} unreadable — the CSV stays unread");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--url") {
                Some(u) => u,
                None => URL.to_string(),
            };
            match fetch(&url) {
                Some(t) => t,
                None => {
                    eprintln!(
                        "{url}: fetch void — the CSV stays unread (ip-blocked without a proxy)"
                    );
                    std::process::exit(1);
                }
            }
        }
    };
    let lsk = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => match parse_lsk(&t) {
            Some(l) => l,
            None => {
                eprintln!("--lsk parses void — the leap-second table stays unread");
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };

    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    let mut observed_rows = 0usize;
    let mut predicted_rows = 0usize;
    let mut pre_lsk_rows = 0usize;
    let mut absent_components = 0usize;
    for line in text.lines() {
        let columns: Vec<&str> = line.split(',').collect();
        let Some((mjd, x, y, ut1, observed)) = parse_row(&columns) else {
            continue;
        };
        if !observed {
            predicted_rows += 1;
            continue;
        }
        observed_rows += 1;
        let unix = (mjd - MJD_UNIX_EPOCH) * 86400.0;
        let Some(t) = lsk.unix_to_tdb(unix) else {
            pre_lsk_rows += 1;
            continue;
        };
        match ut1 {
            Some(v) => records.push((t, v, COMP_UT1_UTC)),
            None => absent_components += 1,
        }
        match x {
            Some(v) => records.push((t, v, COMP_PMX)),
            None => absent_components += 1,
        }
        match y {
            Some(v) => records.push((t, v, COMP_PMY)),
            None => absent_components += 1,
        }
    }
    if records.is_empty() {
        eprintln!("{out_bin}: 0 records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    let bytes = write_bin(&records);
    if std::fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {out_bin} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!(
            "{out_bin}: {} records ({} observed rows, {} predicted rows skipped as forecast, {} pre-1972 rows unharvested (leap table void), {} absent components skipped) — roundtrip parses ({} B)",
            parsed.len(),
            observed_rows,
            predicted_rows,
            pre_lsk_rows,
            absent_components,
            bytes.len()
        ),
        None => {
            eprintln!("{out_bin}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_measured_header_and_rows() {
        let csv = "DATE,MJD,X,Y,UT1-UTC,LOD,DPSI,DEPS,DX,DY,DAT,DATA_TYPE\n\
1962-01-01,37665,-0.012700,0.213000,0.0326338,0.0017230,0.064261,0.006067,0.000000,0.000000,2,O\n\
1962-01-02,37666,-0.015900,0.214100,0.0320547,0.0016690,0.063979,0.006290,0.000000,0.000000,2,O\n\
2027-03-14,61478,0.091090,0.431718,-0.1665246,0.0008081,-0.115606,-0.010778,0.000181,-0.000073,37,P\n";
        let mut rows = Vec::new();
        for line in csv.lines() {
            let columns: Vec<&str> = line.split(',').collect();
            if let Some(row) = parse_row(&columns) {
                rows.push(row);
            }
        }
        assert_eq!(rows.len(), 3);
        let (mjd, x, y, ut1, observed) = rows[0];
        assert_eq!(mjd, 37665.0);
        assert_eq!(x, Some(-0.012700));
        assert_eq!(y, Some(0.213000));
        assert_eq!(ut1, Some(0.0326338));
        assert!(observed);
        assert!(!rows[2].4);
        assert_eq!(rows[2].0, 61478.0);
        assert_eq!(rows[2].3, Some(-0.1665246));
    }

    #[test]
    fn rejects_the_header_and_absent_cells() {
        let csv = "DATE,MJD,X,Y,UT1-UTC,LOD,DPSI,DEPS,DX,DY,DAT,DATA_TYPE\n\
1962-01-01,37665,,0.213000,0.0326338,0.0017230,0.064261,0.006067,0.000000,0.000000,2,O\n";
        let mut rows = Vec::new();
        for line in csv.lines() {
            let columns: Vec<&str> = line.split(',').collect();
            if let Some(row) = parse_row(&columns) {
                rows.push(row);
            }
        }
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1, None);
        assert_eq!(rows[0].2, Some(0.213000));
        assert_eq!(rows[0].3, Some(0.0326338));
    }

    #[test]
    fn non_finite_cells_read_absent() {
        let csv = "DATE,MJD,X,Y,UT1-UTC,LOD,DPSI,DEPS,DX,DY,DAT,DATA_TYPE\n\
1962-01-01,37665,NaN,0.213000,0.0326338,0.0017230,0.064261,0.006067,0.000000,0.000000,2,O\n";
        let mut rows = Vec::new();
        for line in csv.lines() {
            let columns: Vec<&str> = line.split(',').collect();
            if let Some(row) = parse_row(&columns) {
                rows.push(row);
            }
        }
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1, None);
    }
}
