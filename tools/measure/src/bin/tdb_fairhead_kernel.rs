use std::fs;

const TAG: &str = "erfa-dtdb-series-1";

const TEST_DATE1: f64 = 2_448_939.5;
const TEST_DATE2: f64 = 0.123;
const TEST_UT: f64 = 0.76543;
const TEST_ELONG: f64 = 5.0123;
const TEST_U: f64 = 5525.242;
const TEST_V: f64 = 3190.0;
const TEST_WANT: f64 = -0.1280368005936998991e-2;
const TEST_TOL: f64 = 1e-15;

const DJ00: f64 = 2_451_545.0;
const DJM: f64 = 365_250.0;
const D2PI: f64 = std::f64::consts::TAU;
const DD2R: f64 = std::f64::consts::PI / 180.0;

fn parse_series(text: &str) -> Option<Vec<(f64, f64, f64)>> {
    let start = text.find("fairhd[787][3]")?;
    let open = text[start..].find('{')? + start;
    let close = text[open..].find("};")? + open;
    let body = &text[open + 1..close];
    let mut out = Vec::with_capacity(787);
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("/*") {
            continue;
        }
        let clean = t.trim_end_matches(',').trim();
        let clean = clean.trim_start_matches('{').trim_end_matches('}').trim();
        if clean.is_empty() {
            continue;
        }
        let parts: Vec<&str> = clean
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty())
            .collect();
        if parts.len() != 3 {
            return None;
        }
        let amp = parts[0].parse::<f64>().ok()?;
        let freq = parts[1].parse::<f64>().ok()?;
        let phase = parts[2].parse::<f64>().ok()?;
        out.push((amp, freq, phase));
    }
    if out.len() == 787 { Some(out) } else { None }
}

fn dtdb(s: &[(f64, f64, f64)], date1: f64, date2: f64, ut: f64, elong: f64, u: f64, v: f64) -> f64 {
    let t = ((date1 - DJ00) + date2) / DJM;
    let tsol = (ut % 1.0) * D2PI + elong;
    let w = t / 3600.0;
    let elsun = (280.46645683_f64 + 1_296_027_711.03429 * w).rem_euclid(360.0) * DD2R;
    let emsun = (357.52910918_f64 + 1_295_965_810.481 * w).rem_euclid(360.0) * DD2R;
    let d = (297.85019547_f64 + 16_029_616_012.090 * w).rem_euclid(360.0) * DD2R;
    let elj = (34.35151874_f64 + 109_306_899.89453 * w).rem_euclid(360.0) * DD2R;
    let els = (50.07744430_f64 + 44_046_398.47038 * w).rem_euclid(360.0) * DD2R;
    let wt = 0.00029e-10 * u * (tsol + elsun - els).sin()
        + 0.00100e-10 * u * (tsol - 2.0 * emsun).sin()
        + 0.00133e-10 * u * (tsol - d).sin()
        + 0.00133e-10 * u * (tsol + elsun - elj).sin()
        - 0.00229e-10 * u * (tsol + 2.0 * elsun + emsun).sin()
        - 0.02200e-10 * v * (elsun + emsun).cos()
        + 0.05312e-10 * u * (tsol - emsun).sin()
        - 0.13677e-10 * u * (tsol + 2.0 * elsun).sin()
        - 1.31840e-10 * v * elsun.cos()
        + 3.17679e-10 * u * tsol.sin();
    let mut w0 = 0.0;
    for &(a, f, p) in &s[0..474] {
        w0 += a * (f * t + p).sin();
    }
    let mut w1 = 0.0;
    for &(a, f, p) in &s[474..679] {
        w1 += a * (f * t + p).sin();
    }
    let mut w2 = 0.0;
    for &(a, f, p) in &s[679..764] {
        w2 += a * (f * t + p).sin();
    }
    let mut w3 = 0.0;
    for &(a, f, p) in &s[764..784] {
        w3 += a * (f * t + p).sin();
    }
    let mut w4 = 0.0;
    for &(a, f, p) in &s[784..787] {
        w4 += a * (f * t + p).sin();
    }
    let wf = t * (t * (t * (t * w4 + w3) + w2) + w1) + w0;
    let wj = 0.00065e-6 * (6069.776754 * t + 4.021194).sin()
        + 0.00033e-6 * (213.299095 * t + 5.543132).sin()
        + (-0.00196e-6 * (6208.294251 * t + 5.696701).sin())
        + (-0.00173e-6 * (74.781599 * t + 2.435900).sin())
        + 0.03638e-6 * t * t;
    wt + wf + wj
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
        arg_value(&args, "--dtdb-c"),
        arg_value(&args, "--emit-tsv"),
        arg_value(&args, "--source"),
        arg_value(&args, "--source-sha256"),
    ) else {
        eprintln!("usage: tdb_fairhead_kernel --dtdb-c <dtdb.c> --source <label> --source-sha256 <hex> --emit-tsv <tdb_fairhead.tsv>");
        std::process::exit(2);
    };
    let text = match fs::read_to_string(&input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{input} unreadable — the series stays unread ({e})");
            std::process::exit(1);
        }
    };
    let Some(series) = parse_series(&text) else {
        eprintln!("{input} does not carry 787 fairhd triples — nothing written");
        std::process::exit(1);
    };
    let got = dtdb(&series, TEST_DATE1, TEST_DATE2, TEST_UT, TEST_ELONG, TEST_U, TEST_V);
    if (got - TEST_WANT).abs() > TEST_TOL {
        eprintln!(
            "the published eraDtdb vector is {TEST_WANT}, this series gives {got} — the parse or the arithmetic is wrong, nothing written"
        );
        std::process::exit(1);
    }
    let mut text_out = format!(
        "# TDB-TT, Fairhead & Bretagnon 1990 — amp s, freq rad/millennium, phase rad\n\
         # source: {source}\n\
         # source-sha256: {source_sha256}\n\
         # tool: {TAG}\n\
         # published check: eraDtdb(2448939.5, 0.123, 0.76543, 5.0123, 5525.242, 3190.0) = {TEST_WANT} (|d| <= {TEST_TOL})\n\
         # rows: {} — groups T^0 0..473, T^1 474..678, T^2 679..763, T^3 764..783, T^4 784..786\n",
        series.len()
    );
    for (a, f, p) in &series {
        text_out.push_str(&format!("{a}\t{f}\t{p}\n"));
    }
    if let Err(e) = fs::write(&out, text_out) {
        eprintln!("{out} not written ({e})");
        std::process::exit(1);
    }
    println!(
        "tdb: {} terms, published vector {got:.18e} (want {TEST_WANT:.18e}) -> {out}",
        series.len()
    );

    let mut worst = 0.0f64;
    let mut worst_jd = 0.0f64;
    let start = 2_436_934.5;
    let end = 2_469_809.5;
    let mut jd = start;
    while jd <= end {
        let d = jd - DJ00;
        let g = (357.53 + 0.9856003 * d) * DD2R;
        let lm = (246.11 + 0.9025179 * d) * DD2R;
        let two_term = 0.001657 * g.sin() + 0.000022 * lm.sin();
        let full = dtdb(&series, jd, 0.0, 0.0, 0.0, 0.0, 0.0);
        let diff = (full - two_term).abs();
        if diff > worst {
            worst = diff;
            worst_jd = jd;
        }
        jd += 1.0;
    }
    println!(
        "tdb: two-term approximation off by up to {worst:.3e} s (JD {worst_jd:.1}) over 1960-2050"
    );
}

#[cfg(test)]
mod tests {
    use super::{dtdb, parse_series};

    const C: &str = "static const double fairhd[787][3] = {\n";

    #[test]
    fn a_short_array_is_refused() {
        assert!(parse_series(C).is_none());
        assert!(parse_series("no array here").is_none());
    }

    #[test]
    fn a_triple_line_parses() {
        let mut text = String::from(C);
        for _ in 0..787 {
            text.push_str("      {    0.001e-6,     6283.075849991,  5.705257275 },\n");
        }
        text.push_str("   };\n");
        let s = parse_series(&text).expect("787 triples");
        assert_eq!(s.len(), 787);
        assert!((s[0].0 - 0.001e-6).abs() < 1e-20);
        let v = dtdb(&s, 2_448_939.5, 0.123, 0.0, 0.0, 0.0, 0.0);
        assert!(v.is_finite());
    }
}
