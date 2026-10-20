use omegaflow::cdn::upload_release;
use omegaflow::spectral::{parse_spectral_bin, write_spectral_bin};

const DEFAULT_OUT: &str = "edges.bin";
const EDGES_EPOCH_UNIX: f64 = 1_473_292_800.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Case {
    E,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P8,
}

impl Case {
    fn token(self) -> &'static str {
        match self {
            Case::E => "e",
            Case::H1 => "h1",
            Case::H2 => "h2",
            Case::H3 => "h3",
            Case::H4 => "h4",
            Case::H5 => "h5",
            Case::H6 => "h6",
            Case::P8 => "p8",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "e" | "t21" => Some(Case::E),
            "h1" => Some(Case::H1),
            "h2" => Some(Case::H2),
            "h3" => Some(Case::H3),
            "h4" => Some(Case::H4),
            "h5" => Some(Case::H5),
            "h6" => Some(Case::H6),
            "p8" => Some(Case::P8),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Header {
    freq_col: usize,
    weight_col: usize,
    t21_col: usize,
}

fn split_header(line: &str) -> Vec<String> {
    line.split(',')
        .map(|s| s.trim().trim_start_matches('\u{feff}').to_string())
        .collect()
}

fn column_of(cols: &[String], want: &str) -> Option<usize> {
    cols.iter().position(|c| c.eq_ignore_ascii_case(want))
}

fn case_column(cols: &[String], case: Case, suffix: &str) -> Option<usize> {
    let want = format!("{}: {}", case.token(), suffix);
    cols.iter().position(|c| c.eq_ignore_ascii_case(&want))
}

fn resolve_header(line: &str, requested: Option<Case>) -> Result<Header, String> {
    let cols = split_header(line);
    if cols.is_empty() {
        return Err("header is empty".into());
    }
    let freq_col = cols
        .iter()
        .position(|c| c.to_ascii_lowercase().starts_with("frequency"))
        .ok_or_else(|| format!("Frequency column absent in header: {line}"))?;

    if let (Some(weight_col), Some(t21_col)) = (
        column_of(&cols, "Weight"),
        cols.iter()
            .position(|c| c.to_ascii_lowercase().contains("t21")),
    ) {
        if requested.is_none() || requested == Some(Case::E) {
            return Ok(Header {
                freq_col,
                weight_col,
                t21_col,
            });
        }
    }

    let wanted = match requested {
        Some(c) if c != Case::E => vec![c],
        _ => vec![
            Case::H2,
            Case::H1,
            Case::H3,
            Case::H4,
            Case::H5,
            Case::H6,
            Case::P8,
        ],
    };
    for case in wanted {
        if let (Some(weight_col), Some(t21_col)) = (
            case_column(&cols, case, "Weight"),
            case_column(&cols, case, "T21 [K]"),
        ) {
            return Ok(Header {
                freq_col,
                weight_col,
                t21_col,
            });
        }
    }
    Err(format!(
        "no Weight/T21 column pair for case {:?} in header: {line}",
        requested.map(|c| c.token()).unwrap_or("auto")
    ))
}

fn parse_edges_csv(text: &str, requested: Option<Case>) -> Result<Vec<(f64, f64, f64)>, String> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header_line = lines.next().ok_or("CSV is empty")?;
    let header = resolve_header(header_line, requested)?;

    let mut rows: Vec<(f64, f64)> = Vec::new();
    let mut malformed = 0usize;
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() <= header.t21_col || parts.len() <= header.weight_col {
            malformed += 1;
            continue;
        }
        let weight: f64 = match parts[header.weight_col].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                malformed += 1;
                continue;
            }
        };
        if weight != 1.0 {
            continue;
        }
        let freq_mhz: f64 = match parts[header.freq_col].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                malformed += 1;
                continue;
            }
        };
        let t21: f64 = match parts[header.t21_col].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                malformed += 1;
                continue;
            }
        };
        if !freq_mhz.is_finite() || !t21.is_finite() {
            malformed += 1;
            continue;
        }
        rows.push((freq_mhz, t21));
    }
    if malformed > 0 {
        eprintln!("{} malformed rows skipped", malformed);
    }
    if rows.is_empty() {
        return Err("no Weight=1 rows — the bin stays unwritten".into());
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut bins = Vec::with_capacity(rows.len());
    for (i, &(freq_mhz, t21)) in rows.iter().enumerate() {
        let freq = freq_mhz * 1e6;
        let prev = i.checked_sub(1).map(|j| rows[j].0 * 1e6);
        let next = rows.get(i + 1).map(|r| r.0 * 1e6);
        let bin_width = match (prev, next) {
            (Some(p), Some(n)) => (n - p).abs() * 0.5,
            (Some(p), None) => (freq - p).abs(),
            (None, Some(n)) => (n - freq).abs(),
            (None, None) => 0.0,
        };
        bins.push((freq, bin_width, t21));
    }
    Ok(bins)
}

fn run(
    input: &str,
    requested: Option<Case>,
    out: &str,
    epoch: f64,
    ci: bool,
) -> Result<usize, String> {
    let text = std::fs::read_to_string(input).map_err(|e| format!("read {input}: {e}"))?;
    let bins = parse_edges_csv(&text, requested)?;
    let bytes = write_spectral_bin(epoch, &bins);
    std::fs::write(out, &bytes).map_err(|e| format!("write {out}: {e}"))?;
    match parse_spectral_bin(&bytes) {
        Some((parsed_epoch, parsed)) => {
            eprintln!(
                "{out}: {} bins, epoch {parsed_epoch} — roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => return Err(format!("{out}: roundtrip parse void")),
    }
    if ci && !upload_release("loco.lab.asu.edu", out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(bins.len())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci = args.iter().any(|a| a == "--ci-mode");
    let input = match arg_value(&args, "--input") {
        Some(v) => v,
        None => {
            eprintln!("--input absent — the EDGES CSV is undeclared");
            std::process::exit(1);
        }
    };
    let requested = match arg_value(&args, "--case") {
        Some(v) => match Case::parse(&v) {
            Some(c) => Some(c),
            None => {
                eprintln!("--case {v} is not one of e,h1..h6,p8");
                std::process::exit(1);
            }
        },
        None => None,
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let epoch = match arg_value(&args, "--epoch") {
        Some(v) => match v.parse::<f64>() {
            Ok(e) if e.is_finite() => e,
            _ => {
                eprintln!("--epoch {v} is not a finite unix second");
                std::process::exit(1);
            }
        },
        None => EDGES_EPOCH_UNIX,
    };
    if let Err(msg) = run(&input, requested, &out, epoch, ci) {
        eprintln!("{msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const H790: &str = "\
Frequency [MHz], Weight, a: Tsky [K], b: Tres1 [K], c: Tres2 [K], d: Tmodel [K], e: T21 [K]
50.012207,0,0,0,0,0,0
50.402832,1,4600.0,-0.10,-0.02,-0.50,-0.52
50.793457,1,4500.0,-0.20,-0.03,-0.50,-0.53
51.184082,0,4400.0,-0.30,-0.04,-0.50,-0.54
51.574707,1,4350.0,-0.40,-0.05,-0.50,-0.55
";

    const H792: &str = "\
Frequency [MHz], Redshift, Age [Myr], H1: Weight, H1: T21 [K], H2: Weight, H2: T21 [K], H3: Weight, H3: T21 [K], H4: Weight, H4: T21 [K], H5: Weight, H5: T21 [K], H6: Weight, H6: T21 [K], P8: Weight, P8: T21 [K]
50.012207,27.393068,114.764730,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
50.402832,27.100000,113.000000,1,-0.11,0,0,0,0,0,0,0,0,0,0,0,0
50.793457,26.800000,112.000000,1,-0.12,1,-0.60,1,-0.13,0,0,0,0,0,0,0,0
51.184082,26.500000,111.000000,0,0,1,-0.61,0,0,1,-0.14,0,0,0,0,0,0
51.574707,26.200000,110.000000,1,-0.15,1,-0.62,1,-0.16,1,-0.17,1,-0.18,1,-0.19,1,-0.20
";

    #[test]
    fn edges_790_header_weight_filter_and_neighbour_widths() {
        let bins = parse_edges_csv(H790, None).expect("790 parses");
        assert_eq!(bins.len(), 3, "only Weight=1 rows reach the bin");
        assert!((bins[0].0 - 50_402_832.0).abs() < 1e-6);
        assert!((bins[1].0 - 50_793_457.0).abs() < 1e-6);
        assert!((bins[2].0 - 51_574_707.0).abs() < 1e-6);
        assert!((bins[0].2 - (-0.52)).abs() < 1e-9);
        assert!((bins[1].2 - (-0.53)).abs() < 1e-9);
        assert!((bins[2].2 - (-0.55)).abs() < 1e-9);
        assert!((bins[1].1 - (51_574_707.0 - 50_402_832.0) * 0.5).abs() < 1e-6);
    }

    #[test]
    fn edges_790_weight_zero_row_is_excluded() {
        let bins = parse_edges_csv(H790, Some(Case::E)).expect("790 parses");
        assert!(bins.iter().all(|b| (b.0 - 50_012_207.0).abs() > 1.0));
        assert!(bins.iter().all(|b| (b.0 - 51_184_082.0).abs() > 1.0));
    }

    #[test]
    fn edges_792_selects_h2_and_filters_its_weight() {
        let bins = parse_edges_csv(H792, Some(Case::H2)).expect("792 parses");
        assert_eq!(bins.len(), 3, "H2 Weight=1 rows only");
        assert!((bins[0].0 - 50_793_457.0).abs() < 1e-6);
        assert!((bins[0].2 - (-0.60)).abs() < 1e-9);
        assert!((bins[1].0 - 51_184_082.0).abs() < 1e-6);
        assert!((bins[1].2 - (-0.61)).abs() < 1e-9);
        assert!((bins[2].0 - 51_574_707.0).abs() < 1e-6);
        assert!((bins[2].2 - (-0.62)).abs() < 1e-9);
        assert!(!bins.iter().any(|b| (b.0 - 50_402_832.0).abs() < 1.0));
    }

    #[test]
    fn edges_792_auto_prefers_the_canonical_h2_case() {
        let bins = parse_edges_csv(H792, None).expect("792 auto parses");
        assert_eq!(bins.len(), 3);
        assert!((bins[0].2 - (-0.60)).abs() < 1e-9, "auto picks H2");
    }

    #[test]
    fn edges_unknown_case_is_refused() {
        let err = parse_edges_csv(H790, Some(Case::H2)).expect_err("790 has no H2");
        assert!(err.contains("no Weight/T21 column pair"));
    }

    #[test]
    fn edges_empty_weight_is_refused() {
        let text = "Frequency [MHz], Weight, e: T21 [K]\n50.0,0,0\n";
        let err = parse_edges_csv(text, None).expect_err("all weights zero");
        assert!(err.contains("no Weight=1 rows"));
    }
}
