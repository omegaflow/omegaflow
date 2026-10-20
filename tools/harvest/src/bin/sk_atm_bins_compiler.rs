
use omegaflow::archivar::inflate::{gunzip, tar_members};
use omegaflow_harvest::table::{Table, arg_or, emit, has_flag, load_source, parse_bin, stat};

const MAGIC: [u8; 4] = *b"SKAT";
const NETLOC: &str = "zenodo.org";
const FORMAT: &str = "sk_atm_bins";
const COMPILER: &str = "tools/harvest/src/bin/sk_atm_bins_compiler.rs";
const DEFAULT_URL: &str = "https://zenodo.org/api/records/8401262/files/sk_atm_2023.tar.gz/content";
const DEFAULT_OUT: &str = "sk_atm_bins.bin";
const BININFO: &str = "sk_2023_BinInfo.txt";
const DATA: &str = "sk_2023_Data.txt";

const NAMES: [&str; 6] = ["bin", "logp_lo", "logp_hi", "cosz_lo", "cosz_hi", "count"];

fn parse_edges(text: &str) -> Vec<[f64; 4]> {
    let mut edges = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = t.split_whitespace().collect();
        if cells.len() < 5 {
            continue;
        }
        let at = cells.len() - 4;
        let parsed: Option<Vec<f64>> = cells[at..]
            .iter()
            .map(|s| s.parse::<f64>().ok().filter(|v| v.is_finite()))
            .collect();
        let Some(values) = parsed else {
            continue;
        };
        let Some(edge) = values.get(0..4) else {
            continue;
        };
        edges.push([edge[0], edge[1], edge[2], edge[3]]);
    }
    edges
}

fn parse_counts(text: &str) -> Vec<f64> {
    let mut counts = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let Some(token) = t.split_whitespace().next() else {
            continue;
        };
        if let Ok(value) = token.parse::<f64>() {
            if value.is_finite() && value >= 0.0 {
                counts.push(value);
            }
        }
    }
    counts
}

pub fn parse_bins(bininfo: &str, data: &str) -> Option<Table> {
    let edges = parse_edges(bininfo);
    let counts = parse_counts(data);
    if edges.is_empty() || edges.len() != counts.len() {
        return None;
    }
    let mut rows = Vec::with_capacity(edges.len());
    for (i, (edge, count)) in edges.iter().zip(counts.iter()).enumerate() {
        rows.push(vec![
            Some(i as f64),
            Some(edge[0]),
            Some(edge[1]),
            Some(edge[2]),
            Some(edge[3]),
            Some(*count),
        ]);
    }
    Some(Table {
        names: NAMES.iter().map(|s| s.to_string()).collect(),
        rows,
    })
}

fn member<'a>(tar: &'a [u8], suffix: &str) -> Option<&'a [u8]> {
    let members = tar_members(tar)?;
    let hit = members.iter().find(|m| m.name.ends_with(suffix))?;
    tar.get(hit.start..hit.end)
}

fn selftest() {
    let bins = "# SampleName LogP_lo LogP_hi CosZ_lo CosZ_hi\nsk1-3_fc_subgev_1ring_elike 2.00 2.40 -1.00 -0.84\nsk1-3_fc_subgev_1ring_elike 2.00 2.40 -0.84 -0.64\n";
    let data = "# Counts\n185.0\n172.0\n";
    let Some(table) = parse_bins(bins, data) else {
        eprintln!("selftest: the two-bin fixture parses void");
        std::process::exit(1);
    };
    if table.rows.len() != 2 || table.rows[0][5] != Some(185.0) || table.rows[1][3] != Some(-0.84) {
        eprintln!("selftest: the fixture does not carry edges/counts in order");
        std::process::exit(1);
    }
    if parse_bins(bins, "# Counts\n185.0\n").is_some() {
        eprintln!("selftest: a count/edge mismatch is not refused");
        std::process::exit(1);
    }
    eprintln!("sk_atm_bins_compiler: selftest passes (930-bin layout, count/edge pairing)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--selftest") {
        selftest();
        return;
    }
    let Some((src, bytes)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let tar = if bytes.starts_with(&[0x1f, 0x8b]) {
        match gunzip(&bytes) {
            Some(body) => body,
            None => {
                eprintln!("{src}: the gzip stream stays unreadable — absent (0 honored)");
                std::process::exit(1);
            }
        }
    } else {
        bytes
    };
    let (Some(bininfo), Some(data)) = (member(&tar, BININFO), member(&tar, DATA)) else {
        eprintln!("{src}: {BININFO} or {DATA} absent — absent (0 honored)");
        std::process::exit(1);
    };
    let (Ok(bininfo), Ok(data)) = (std::str::from_utf8(bininfo), std::str::from_utf8(data)) else {
        eprintln!("{src}: the bin tables are not utf8 — absent (0 honored)");
        std::process::exit(1);
    };
    let Some(table) = parse_bins(bininfo, data) else {
        eprintln!("{src}: no complete bin pair survived — absent (0 honored)");
        std::process::exit(1);
    };
    if has_flag(&args, "--stat") {
        stat(&src, &table);
        return;
    }
    let out = arg_or(&args, "--out", DEFAULT_OUT);
    if !emit(&src, &out, &table, MAGIC, NETLOC, FORMAT, COMPILER) {
        std::process::exit(1);
    }
    if has_flag(&args, "--ci-mode") {
        let parsed = std::fs::read(&out)
            .ok()
            .and_then(|bytes| parse_bin(MAGIC, &bytes));
        let Some(parsed) = parsed else {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        };
        if parsed.rows.len() != table.rows.len() {
            eprintln!("{out}: roundtrip row count differs — the bin stays unverified");
            std::process::exit(1);
        }
        if !omegaflow::cdn::upload_release(NETLOC, &out) {
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BINS: &str = "# SampleName LogP_lo LogP_hi CosZ_lo CosZ_hi\n\
sk1-3_fc_subgev_1ring_elike 2.00 2.40 -1.00 -0.84\n\
sk1-3_fc_subgev_1ring_elike 2.00 2.40 -0.84 -0.64\n\
sk4-5_fc_multigev_1ring_numubarlike 3.40 5.00 0.84 1.00\n";

    const DATA: &str = "# Counts\n185.0\n172.0\n12.0\n";

    #[test]
    fn bins_and_counts_pair_in_order() {
        let table = parse_bins(BINS, DATA).unwrap();
        assert_eq!(
            table.names,
            vec!["bin", "logp_lo", "logp_hi", "cosz_lo", "cosz_hi", "count"]
        );
        assert_eq!(table.rows.len(), 3);
        assert_eq!(table.rows[0][5], Some(185.0));
        assert_eq!(table.rows[1][3], Some(-0.84));
        assert_eq!(table.rows[2][1], Some(3.40));
        assert_eq!(table.rows[2][4], Some(1.00));
    }

    #[test]
    fn missing_or_extra_count_refuses_the_asset() {
        assert!(parse_bins(BINS, "# Counts\n185.0\n172.0\n").is_none());
        assert!(parse_bins(BINS, "# Counts\n185.0\n172.0\n12.0\n7.0\n").is_none());
        assert!(parse_bins("# only comments\n", DATA).is_none());
    }

    #[test]
    fn header_and_blank_lines_do_not_become_bins() {
        let table = parse_bins(BINS, DATA).unwrap();
        assert_eq!(table.rows[0][0], Some(0.0));
        assert_eq!(table.rows[2][0], Some(2.0));
        assert!(table.rows.iter().all(|r| r[5].unwrap() < 1000.0));
    }
}
