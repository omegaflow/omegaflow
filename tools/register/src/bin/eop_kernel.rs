use std::fs;

const COL_MJD: usize = 4;
const COL_XP: usize = 5;
const COL_YP: usize = 6;
const COL_UT1_UTC: usize = 7;
const COL_DX: usize = 8;
const COL_DY: usize = 9;
const COL_LOD: usize = 12;

struct Row {
    mjd: f64,
    ut1: f64,
    xp: f64,
    yp: f64,
    lod: f64,
    dx: f64,
    dy: f64,
}

fn header(source: &str, source_sha256: &str, rows: &[Row]) -> Option<String> {
    let first = rows.first()?.mjd;
    let last = rows.last()?.mjd;
    Some(format!(
        "# EOP (IERS C04, ITRF2020) — mjd, UT1-UTC s, x_p arcsec, y_p arcsec, LOD s, dX arcsec, dY arcsec\n\
         # source: {source}\n\
         # source-sha256: {source_sha256}\n\
         # layout: tokens 0-based YR MM DD HH MJD x y UT1-UTC dX dY xrt yrt LOD — dX=8, dY=9, LOD=12\n\
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
    let (Some(input), Some(out), Some(source), Some(source_sha256)) = (
        arg_value(&args, "--input"),
        arg_value(&args, "--emit-tsv"),
        arg_value(&args, "--source"),
        arg_value(&args, "--source-sha256"),
    ) else {
        eprintln!(
            "usage: eop_kernel --input <eopc04.txt> --source <product/url> --source-sha256 <hex> --emit-tsv <eop.tsv>"
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
    let mut rows: Vec<Row> = Vec::new();
    let mut skipped = 0usize;
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = t.split_whitespace().collect();
        if c.len() <= COL_LOD {
            skipped += 1;
            continue;
        }
        let (Ok(mjd), Ok(xp), Ok(yp), Ok(ut1), Ok(dx), Ok(dy), Ok(lod)) = (
            c[COL_MJD].parse::<f64>(),
            c[COL_XP].parse::<f64>(),
            c[COL_YP].parse::<f64>(),
            c[COL_UT1_UTC].parse::<f64>(),
            c[COL_DX].parse::<f64>(),
            c[COL_DY].parse::<f64>(),
            c[COL_LOD].parse::<f64>(),
        ) else {
            skipped += 1;
            continue;
        };
        if ![mjd, xp, yp, ut1, dx, dy, lod]
            .iter()
            .all(|v| v.is_finite())
        {
            skipped += 1;
            continue;
        }
        rows.push(Row {
            mjd,
            ut1,
            xp,
            yp,
            lod,
            dx,
            dy,
        });
    }
    if rows.is_empty() {
        eprintln!("{input} carries no complete EOP row — nothing written");
        std::process::exit(1);
    }
    rows.sort_by(|a, b| a.mjd.total_cmp(&b.mjd));
    let Some(mut out_text) = header(&source, &source_sha256, &rows) else {
        eprintln!("{input} carries no complete EOP row — nothing written");
        std::process::exit(1);
    };
    for r in &rows {
        out_text.push_str(&format!(
            "{:.0}\t{:.7}\t{:.4}\t{:.4}\t{:.7}\t{:.6}\t{:.6}\n",
            r.mjd, r.ut1, r.xp, r.yp, r.lod, r.dx, r.dy
        ));
    }
    if let Err(e) = fs::write(&out, out_text) {
        eprintln!("{out} not written ({e})");
        std::process::exit(1);
    }
    println!(
        "eop: {} rows, {} skipped, MJD {:.0}..{:.0} -> {out}",
        rows.len(),
        skipped,
        rows[0].mjd,
        rows[rows.len() - 1].mjd
    );
}

#[cfg(test)]
mod tests {
    use super::{Row, header};

    #[test]
    fn the_header_names_source_layout_and_range() {
        let rows = [
            Row {
                mjd: 37665.0,
                ut1: 0.0,
                xp: 0.0,
                yp: 0.0,
                lod: 0.0,
                dx: 0.0,
                dy: 0.0,
            },
            Row {
                mjd: 61256.0,
                ut1: 0.0,
                xp: 0.0,
                yp: 0.0,
                lod: 0.0,
                dx: 0.0,
                dy: 0.0,
            },
        ];
        let h = header("eopc04", "abc", &rows).expect("two rows");
        assert!(h.contains("# source: eopc04"), "{h}");
        assert!(h.contains("# source-sha256: abc"), "{h}");
        assert!(h.contains("dX=8, dY=9, LOD=12"), "{h}");
        assert!(h.contains("# rows: 2 — MJD 37665..61256"), "{h}");
        assert!(header("x", "y", &[]).is_none());
    }
}
