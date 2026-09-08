use std::collections::BTreeMap;

const N_PAIRS: usize = 72;
const MIN_EVENTS_FLOOR: usize = 30;
const LAG_MAX: usize = 12;
const N_SURR: usize = 10;

const NAMES: [&str; 9] = [
    "304A", "131A", "171A", "193A", "211A", "335A", "94A", "XRSA", "XRSB",
];
const KINDS: [&str; 9] = [
    "aia", "aia", "aia", "aia", "aia", "aia", "aia", "xrs", "xrs",
];

struct Row {
    from: usize,
    to: usize,
    n_ev: usize,
    cells: f64,
    lag: Option<usize>,
    d: f64,
    thr: f64,
    pos: usize,
}

struct Report {
    sha: Option<String>,
    corpus: Option<String>,
    rows: Vec<Row>,
    surr_max: Option<f64>,
}

fn parse_f64(token: &str) -> f64 {
    token.parse().unwrap_or(f64::NAN)
}

fn parse_report(path: &std::path::Path) -> Report {
    let mut sha = None;
    let mut corpus = None;
    let mut rows = Vec::new();
    let mut surr_max = None;
    let Ok(text) = std::fs::read_to_string(path) else {
        eprintln!("{} reads void", path.display());
        return Report {
            sha,
            corpus,
            rows,
            surr_max,
        };
    };
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("SHA") => {
                sha = parts.next().map(str::to_string);
            }
            Some("CORPUS") => {
                corpus = parts.next().map(str::to_string);
            }
            Some("ROW") => {
                let Some(from) = parts.next().and_then(|t| t.parse::<usize>().ok()) else {
                    continue;
                };
                let Some(to) = parts.next().and_then(|t| t.parse::<usize>().ok()) else {
                    continue;
                };
                let Some(n_ev) = parts.next().and_then(|t| t.parse::<usize>().ok()) else {
                    continue;
                };
                let cells_s = parts.next().unwrap_or("-");
                let lag_s = parts.next().unwrap_or("-");
                let d_s = parts.next().unwrap_or("-");
                let thr_s = parts.next().unwrap_or("-");
                let Some(pos) = parts.next().and_then(|t| t.parse::<usize>().ok()) else {
                    continue;
                };
                rows.push(Row {
                    from,
                    to,
                    n_ev,
                    cells: if cells_s == "-" {
                        f64::NAN
                    } else {
                        parse_f64(cells_s)
                    },
                    lag: lag_s.parse().ok(),
                    d: if d_s == "-" { f64::NAN } else { parse_f64(d_s) },
                    thr: if thr_s == "-" {
                        f64::NAN
                    } else {
                        parse_f64(thr_s)
                    },
                    pos,
                });
            }
            Some("SURRM_MAX") => {
                let t = parts.next().unwrap_or("-");
                if t != "-" {
                    surr_max = Some(parse_f64(t)).filter(|v| v.is_finite());
                }
            }
            _ => {}
        }
    }
    Report {
        sha,
        corpus,
        rows,
        surr_max,
    }
}

fn sig_s(v: f64) -> String {
    if v.is_finite() {
        format!("{:+.4e}", v)
    } else {
        "-".to_string()
    }
}

fn block_rank(from_kind: &str, to_kind: &str) -> usize {
    match (from_kind, to_kind) {
        ("aia", "aia") => 0,
        ("aia", "xrs") => 1,
        ("xrs", "aia") => 2,
        _ => 3,
    }
}

fn block_label(from_kind: &str, to_kind: &str) -> &'static str {
    match (from_kind, to_kind) {
        ("aia", "aia") => "intra-AIA",
        ("aia", "xrs") => "AIA -> XRS",
        ("xrs", "aia") => "XRS -> AIA",
        _ => "XRS-internal",
    }
}

fn verdict_of(r: &Row, fam: f64) -> &'static str {
    if r.n_ev < MIN_EVENTS_FLOOR || !r.d.is_finite() {
        "no-statement"
    } else if fam.is_finite() && r.d > fam {
        "ARROW"
    } else if r.thr.is_finite() && r.d > r.thr {
        "family bound"
    } else {
        "still"
    }
}

fn pair_index(from: usize, to: usize) -> usize {
    from * 8 + to - if to > from { 1 } else { 0 }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dir = match args
        .iter()
        .position(|a| a == "--dir")
        .and_then(|i| args.get(i + 1))
        .cloned()
    {
        Some(v) => v,
        None => ".".to_string(),
    };
    let out_path = match args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned()
    {
        Some(v) => v,
        None => "solar_matrix_sheet.txt".to_string(),
    };
    let mut reports = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".txt") && name != out_path {
                let r = parse_report(&entry.path());
                eprintln!(
                    "sonde report {}: {} rows, surr_max {}",
                    name,
                    r.rows.len(),
                    r.surr_max.map_or("-".to_string(), sig_s)
                );
                reports.push(r);
            }
        }
    }
    let mut sha_set = std::collections::BTreeSet::new();
    let mut corpus_set = std::collections::BTreeSet::new();
    for r in &reports {
        if let Some(s) = &r.sha {
            sha_set.insert(s.clone());
        }
        if let Some(c) = &r.corpus {
            corpus_set.insert(c.clone());
        }
    }
    let sha_drift = sha_set.len() > 1;
    let corpus_drift = corpus_set.len() > 1;
    let mut rows_by_index: BTreeMap<usize, Row> = BTreeMap::new();
    let mut duplicate = Vec::new();
    for r in &reports {
        for row in &r.rows {
            let idx = pair_index(row.from, row.to);
            if rows_by_index
                .insert(
                    idx,
                    Row {
                        from: row.from,
                        to: row.to,
                        n_ev: row.n_ev,
                        cells: row.cells,
                        lag: row.lag,
                        d: row.d,
                        thr: row.thr,
                        pos: row.pos,
                    },
                )
                .is_some()
            {
                duplicate.push(idx);
            }
        }
    }
    let missing: Vec<usize> = (0..N_PAIRS)
        .filter(|i| !rows_by_index.contains_key(i))
        .collect();
    let mut fam = f64::NEG_INFINITY;
    for r in &reports {
        if let Some(v) = r.surr_max {
            if v.is_finite() && v > fam {
                fam = v;
            }
        }
    }
    let mut sheet = String::new();
    sheet.push_str("=== Solar 24-s flare-stacked TE matrix - the reduced sheet ===\n");
    if sha_set.len() == 1 {
        sheet.push_str(&format!(
            "sha {}\n",
            sha_set.iter().next().map(String::as_str).unwrap_or("-")
        ));
    } else {
        sheet.push_str(&format!(
            "sha drift: {} distinct SHAs over {} sondes\n",
            sha_set.len(),
            reports.len()
        ));
    }
    if corpus_set.len() == 1 {
        sheet.push_str(&format!(
            "corpus {}\n",
            corpus_set.iter().next().map(String::as_str).unwrap_or("-")
        ));
    } else if corpus_drift {
        sheet.push_str(&format!(
            "corpus drift: {} distinct corpus hashes over {} sondes\n",
            corpus_set.len(),
            reports.len()
        ));
    } else {
        sheet.push_str("corpus -\n");
    }
    sheet.push_str(&format!(
        "sondes {} | rows {} | missing {}",
        reports.len(),
        rows_by_index.len(),
        missing.len()
    ));
    if !missing.is_empty() {
        let list: Vec<String> = missing.iter().map(|i| i.to_string()).collect();
        sheet.push_str(&format!(" ({})", list.join(",")));
    }
    sheet.push('\n');
    if !duplicate.is_empty() {
        let list: Vec<String> = duplicate.iter().map(|i| i.to_string()).collect();
        sheet.push_str(&format!("duplicate pair rows: {}\n", list.join(",")));
    }
    if sha_drift || corpus_drift {
        sheet.push_str(
            "the sheet refuses the family bound - the anchor drifts across the sondes; \
             the rows below carry no verdict (0 honored).\n",
        );
        std::fs::write(&out_path, &sheet).ok();
        print!("{}", sheet);
        std::process::exit(1);
    }
    if fam.is_finite() {
        sheet.push_str(&format!(
            "fam = {:.4e} over {} directed pairs x {} lags x {} surrogates.\n",
            fam,
            N_PAIRS,
            LAG_MAX + 1,
            N_SURR
        ));
    } else {
        sheet.push_str("fam stays undefined - no surrogate draw over the round (0 honored).\n");
    }
    sheet.push_str(
        "\n=== The 9 x 9 event-wise matrix (stacked per-event D, best lag in 24-s cells) ===\n",
    );
    let mut sorted: Vec<&Row> = rows_by_index.values().collect();
    sorted.sort_by_key(|r| (block_rank(KINDS[r.from], KINDS[r.to]), r.from, r.to));
    let mut last_block: Option<usize> = None;
    for r in &sorted {
        let block = block_rank(KINDS[r.from], KINDS[r.to]);
        if last_block != Some(block) {
            sheet.push('\n');
            sheet.push_str(&format!(
                "--- {} ---\n",
                block_label(KINDS[r.from], KINDS[r.to])
            ));
            last_block = Some(block);
        }
        let cell_s = if r.n_ev > 0 {
            format!("{:.1}", r.cells)
        } else {
            "-".to_string()
        };
        let lag_s = match r.lag {
            Some(l) => format!("{}", l),
            None => "-".to_string(),
        };
        sheet.push_str(&format!(
            "{:>8} -> {:<8} | n_ev {:>4} | {:>6} cells/ev | lag {:>2} | D {:>11} | thr {:>11} | pos {:>4}/{:>4} | {}\n",
            NAMES[r.from],
            NAMES[r.to],
            r.n_ev,
            cell_s,
            lag_s,
            sig_s(r.d),
            sig_s(r.thr),
            r.pos,
            r.n_ev,
            verdict_of(r, fam)
        ));
    }
    sheet.push('\n');
    let mut fam_counts: [usize; 4] = [0, 0, 0, 0];
    let mut intra_counts: [usize; 4] = [0, 0, 0, 0];
    let mut cross_counts: [usize; 4] = [0, 0, 0, 0];
    let mut xrs_counts: [usize; 4] = [0, 0, 0, 0];
    for r in &sorted {
        let v = match verdict_of(r, fam) {
            "ARROW" => 0,
            "family bound" => 1,
            "still" => 2,
            _ => 3,
        };
        fam_counts[v] += 1;
        let block = block_rank(KINDS[r.from], KINDS[r.to]);
        match block {
            0 => intra_counts[v] += 1,
            3 => xrs_counts[v] += 1,
            _ => cross_counts[v] += 1,
        }
    }
    let label = |c: [usize; 4]| {
        format!(
            "ARROW {} | family bound {} | still {} | no-statement {}",
            c[0], c[1], c[2], c[3]
        )
    };
    sheet.push_str("=== Relationship blocks (fam is the full-matrix surrogate bound) ===\n");
    sheet.push_str(&format!(
        "intra-AIA (42 directed pairs): {}\n",
        label(intra_counts)
    ));
    sheet.push_str(&format!(
        "AIA-XRS (28 directed pairs): {}\n",
        label(cross_counts)
    ));
    sheet.push_str(&format!(
        "XRS-internal (2 directed pairs): {}\n",
        label(xrs_counts)
    ));
    sheet.push_str(&format!(
        "whole matrix (72 directed pairs): {}\n",
        label(fam_counts)
    ));
    sheet.push('\n');
    let arrows: Vec<&&Row> = sorted
        .iter()
        .filter(|r| r.d > fam && r.n_ev >= MIN_EVENTS_FLOOR)
        .collect();
    if arrows.is_empty() {
        sheet.push_str(
            "No stacked D clears the full-round family bound fam - silence is a finding (0 honored).\n",
        );
    } else {
        for r in arrows {
            let Some(lag) = r.lag else {
                continue;
            };
            sheet.push_str(&format!(
                "{} -> {} (lag {} cells = {} s, D {:.4e} > fam {:.4e}, {} events, pos {}/{})\n",
                NAMES[r.from],
                NAMES[r.to],
                lag,
                lag * 24,
                r.d,
                fam,
                r.n_ev,
                r.pos,
                r.n_ev
            ));
        }
    }
    sheet.push('\n');
    sheet.push_str(&format!(
        "Lag in 24-s cells (0..{} s); D > 0 means the first actor drives the second within the flare window; the cascade windows are GOES b_flux flare windows.\n",
        LAG_MAX * 24
    ));
    if std::fs::write(&out_path, &sheet).is_err() {
        eprintln!("{} write void", out_path);
        std::process::exit(1);
    }
    print!("{}", sheet);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report_text(sha: &str, corpus: &str, rows: &str, surr: &str) -> String {
        format!(
            "SHA {}\nCORPUS {}\n{}\nSURRM_MAX {}\n",
            sha, corpus, rows, surr
        )
    }

    fn write_report(dir: &std::path::Path, name: &str, text: &str) {
        std::fs::write(dir.join(name), text).unwrap();
    }

    fn report_dir(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!(
            "solar_matrix_reduce_test_{}_{}",
            std::process::id(),
            tag
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn parses_a_report() {
        let d = report_dir("parses");
        write_report(
            &d,
            "report-0.txt",
            &report_text(
                "abc123",
                "def456",
                "ROW 0 1 150 100.0 3 1.0e-3 9.0e-4 90\nROW 0 2 40 98.0 5 -5.0e-4 8.0e-4 12",
                "1.2e-3",
            ),
        );
        let r = parse_report(&d.join("report-0.txt"));
        assert_eq!(r.sha.as_deref(), Some("abc123"));
        assert_eq!(r.corpus.as_deref(), Some("def456"));
        assert_eq!(r.rows.len(), 2);
        assert_eq!(r.rows[0].from, 0);
        assert_eq!(r.rows[0].to, 1);
        assert_eq!(r.rows[0].n_ev, 150);
        assert_eq!(r.rows[0].d, 1.0e-3);
        assert_eq!(r.surr_max, Some(1.2e-3));
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn pair_index_maps_the_probe_order() {
        assert_eq!(pair_index(0, 1), 0);
        assert_eq!(pair_index(0, 8), 7);
        assert_eq!(pair_index(1, 0), 8);
        assert_eq!(pair_index(8, 7), 71);
    }

    #[test]
    fn surr_max_absent_parses_as_none() {
        let d = report_dir("absent");
        write_report(
            &d,
            "report-1.txt",
            &report_text(
                "abc123",
                "def456",
                "ROW 1 0 20 50.0 1 3.0e-4 7.0e-4 11",
                "-",
            ),
        );
        let r = parse_report(&d.join("report-1.txt"));
        assert_eq!(r.surr_max, None);
        std::fs::remove_dir_all(&d).ok();
    }
}
