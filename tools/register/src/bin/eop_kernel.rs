use std::fs;

const COL_MJD: usize = 4;
const COL_XP: usize = 5;
const COL_YP: usize = 6;
const COL_UT1_UTC: usize = 7;

fn header(source: &str, rows: &[(f64, f64, f64, f64)]) -> Option<String> {
    let first = rows.first()?.0;
    let last = rows.last()?.0;
    Some(format!(
        "# EOP (IERS C04, ITRF2020) — mjd, UT1-UTC s, x_p arcsec, y_p arcsec\n\
         # source: {source}\n\
         # rows: {} — MJD {first:.0}..{last:.0}\n",
        rows.len()
    ))
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(input), Some(out), Some(source)) = (
        arg_value(&args, "--input"),
        arg_value(&args, "--emit-tsv"),
        arg_value(&args, "--source"),
    ) else {
        eprintln!(
            "usage: eop_kernel_compiler --input <eopc04.txt> --source <product/url> --emit-tsv <eop.tsv>"
        );
        std::process::exit(2);
    };
    let text = match fs::read_to_string(&input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{input} unreadable — the EOP series stays unread ({e})");
            std::process::exit(1);
        }
    };
    let mut rows: Vec<(f64, f64, f64, f64)> = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = t.split_whitespace().collect();
        if c.len() <= COL_UT1_UTC {
            continue;
        }
        let (Ok(mjd), Ok(xp), Ok(yp), Ok(ut1)) = (
            c[COL_MJD].parse::<f64>(),
            c[COL_XP].parse::<f64>(),
            c[COL_YP].parse::<f64>(),
            c[COL_UT1_UTC].parse::<f64>(),
        ) else {
            continue;
        };
        if ![mjd, xp, yp, ut1].iter().all(|v| v.is_finite()) {
            continue;
        }
        rows.push((mjd, ut1, xp, yp));
    }
    if rows.is_empty() {
        eprintln!("{input} carries no complete EOP row — nothing written");
        std::process::exit(1);
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    let Some(mut out_text) = header(&source, &rows) else {
        eprintln!("{input} carries no complete EOP row — nothing written");
        std::process::exit(1);
    };
    for (mjd, ut1, xp, yp) in &rows {
        out_text.push_str(&format!("{mjd:.0}\t{ut1:.7}\t{xp:.4}\t{yp:.4}\n"));
    }
    if let Err(e) = fs::write(&out, out_text) {
        eprintln!("{out} not written ({e})");
        std::process::exit(1);
    }
    println!(
        "eop: {} rows, MJD {:.0}..{:.0} -> {out}",
        rows.len(),
        rows[0].0,
        rows[rows.len() - 1].0
    );
}

#[cfg(test)]
mod tests {
    use super::header;

    #[test]
    fn the_header_names_the_source_and_the_range() {
        let rows = [
            (37665.0, 0.0, 0.0, 0.0),
            (61256.0, 0.0, 0.0, 0.0),
        ];
        let h = header("EOP_20_C04_one_file_1962-now.txt", &rows).expect("two rows");
        assert!(h.contains("# source: EOP_20_C04_one_file_1962-now.txt"), "{h}");
        assert!(h.contains("# rows: 2 — MJD 37665..61256"), "{h}");
        assert!(header("x", &[]).is_none());
    }
}
