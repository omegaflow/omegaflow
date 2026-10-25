use std::fs;

const COL_MJD: usize = 4;
const COL_XP: usize = 5;
const COL_YP: usize = 6;
const COL_UT1_UTC: usize = 7;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(input), Some(out)) = (arg_value(&args, "--input"), arg_value(&args, "--emit-tsv"))
    else {
        eprintln!("usage: eop_kernel_compiler --input <eopc04.txt> --emit-tsv <eop.tsv>");
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
    let mut out_text =
        String::from("# EOP (IERS C04, ITRF2020) — mjd, UT1-UTC s, x_p arcsec, y_p arcsec\n");
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
