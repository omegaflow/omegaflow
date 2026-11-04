use omegaflow_measure::weberin::fink_alerce::{
    ALERCE_OBJECTS, AlerceObject, FINK_ZTF_CONESEARCH, FinkAlert, alerce_cone_body, fink_cone_body,
    parse_alerce_objects, parse_fink_cone,
};
use omegaflow_measure::weberin::nadel_gate::sep_arcsec;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_f64(args: &[String], name: &str) -> Option<f64> {
    let s = arg_value(args, name)?;
    let v: f64 = s.trim().parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn usage() {
    eprintln!(
        "usage: weberin_fink_alerce_probe --ra <deg> --dec <deg> --radius <arcsec> [--columns <csv>]"
    );
}

fn fmt_num(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.6}"),
        None => "absent".to_string(),
    }
}

fn fmt_word(v: &Option<String>) -> String {
    match v {
        Some(s) => s.clone(),
        None => "absent".to_string(),
    }
}

fn row_id(object_id: &Option<String>, ra: Option<f64>, dec: Option<f64>) -> String {
    match object_id {
        Some(id) => id.clone(),
        None => format!("ra {} dec {}", fmt_num(ra), fmt_num(dec)),
    }
}

fn fink_row_line(r: &FinkAlert) {
    println!(
        "fink {} ra {} dec {} jd {} magpsf {} sigmapsf {} fid {} class {}",
        row_id(&r.object_id, r.ra_deg, r.dec_deg),
        fmt_num(r.ra_deg),
        fmt_num(r.dec_deg),
        fmt_num(r.jd),
        fmt_num(r.magpsf),
        fmt_num(r.sigmapsf),
        fmt_num(r.fid.map(|f| f as f64)),
        fmt_word(&r.classification)
    );
}

fn alerce_row_line(r: &AlerceObject) {
    println!(
        "alerce {} ra {} dec {} firstmjd {} lastmjd {} ndethist {} class {}",
        row_id(&r.oid, r.meanra, r.meandec),
        fmt_num(r.meanra),
        fmt_num(r.meandec),
        fmt_num(r.firstmjd),
        fmt_num(r.lastmjd),
        fmt_num(r.ndethist.map(|n| n as f64)),
        fmt_word(&r.class)
    );
}

fn fink_rows(ra: f64, dec: f64, radius: f64, columns: Option<&str>) -> Option<Vec<FinkAlert>> {
    let (code, body) = match fink_cone_body(ra, dec, radius, columns) {
        Some(v) => v,
        None => {
            println!(
                "fink cone ra {ra:.4} dec {dec:.4}: the query did not answer (measured stall) — pending"
            );
            return None;
        }
    };
    if code != "200" {
        println!(
            "fink cone ra {ra:.4} dec {dec:.4}: {FINK_ZTF_CONESEARCH} answered HTTP {code} — pending"
        );
        return None;
    }
    match parse_fink_cone(&body) {
        Some(rows) => {
            for r in &rows {
                fink_row_line(r);
            }
            println!("fink rows {}", rows.len());
            Some(rows)
        }
        None => {
            println!(
                "fink cone ra {ra:.4} dec {dec:.4}: the HTTP {code} body is not a row array — parser pending on the real schema"
            );
            None
        }
    }
}

fn alerce_rows(ra: f64, dec: f64, radius: f64) -> Option<Vec<AlerceObject>> {
    let (code, body) = match alerce_cone_body(ra, dec, radius) {
        Some(v) => v,
        None => {
            println!(
                "alerce cone ra {ra:.4} dec {dec:.4}: the query did not answer (measured stall) — pending"
            );
            return None;
        }
    };
    if code != "200" {
        println!(
            "alerce cone ra {ra:.4} dec {dec:.4}: {ALERCE_OBJECTS} answered HTTP {code} — pending"
        );
        return None;
    }
    match parse_alerce_objects(&body) {
        Some(rows) => {
            for r in &rows {
                alerce_row_line(r);
            }
            println!("alerce rows {}", rows.len());
            Some(rows)
        }
        None => {
            println!(
                "alerce cone ra {ra:.4} dec {dec:.4}: the HTTP {code} body is not an items object — parser pending on the real schema"
            );
            None
        }
    }
}

fn fold(finks: &[FinkAlert], alerces: &[AlerceObject], radius: f64) {
    let mut placed = 0usize;
    for f in finks {
        let id = row_id(&f.object_id, f.ra_deg, f.dec_deg);
        let best = alerces
            .iter()
            .filter_map(|a| match (&f.object_id, &a.oid) {
                (Some(fi), Some(ai)) if fi == ai => Some((0.0, a)),
                _ => match (f.ra_deg, f.dec_deg, a.meanra, a.meandec) {
                    (Some(fr), Some(fd), Some(ar), Some(ad)) => {
                        let s = sep_arcsec(fr, fd, ar, ad);
                        if s <= radius { Some((s, a)) } else { None }
                    }
                    _ => None,
                },
            })
            .min_by(|x, y| x.0.total_cmp(&y.0));
        match best {
            Some((sep, _)) => {
                placed += 1;
                println!("weberin {id} state placed sep {sep:.3} arcsec");
            }
            None => println!("weberin {id} state absent sep absent missing alerce"),
        }
    }
    for a in alerces {
        let in_fink = finks.iter().any(|f| match (&f.object_id, &a.oid) {
            (Some(fi), Some(ai)) => fi == ai,
            _ => false,
        });
        if !in_fink {
            let id = row_id(&a.oid, a.meanra, a.meandec);
            println!("weberin {id} state absent sep absent missing fink");
        }
    }
    println!(
        "weberin fold fink {} alerce {} placed {placed}",
        finks.len(),
        alerces.len()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(ra), Some(dec), Some(radius)) = (
        parse_f64(&args, "--ra"),
        parse_f64(&args, "--dec"),
        parse_f64(&args, "--radius"),
    ) else {
        usage();
        std::process::exit(2);
    };
    if radius <= 0.0 {
        usage();
        std::process::exit(2);
    }
    let columns = arg_value(&args, "--columns");
    match (
        fink_rows(ra, dec, radius, columns.as_deref()),
        alerce_rows(ra, dec, radius),
    ) {
        (Some(finks), Some(alerces)) => fold(&finks, &alerces, radius),
        _ => println!(
            "the weberin fold stays pending — both broker lines need a 200 with a parsed row array"
        ),
    }
}
