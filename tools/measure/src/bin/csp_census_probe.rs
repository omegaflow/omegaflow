use omegaflow::archivar::csp;

fn main() {
    let arg = std::env::args().skip(1).find(|a| !a.starts_with('-'));
    let Some(path) = arg else {
        eprintln!("csp-census: --file absent — no CSP file to count");
        return;
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("csp-census: read void ({path})");
        return;
    };
    let Some(rows) = csp::parse_csp(&text) else {
        eprintln!("csp-census: parse void — {path} carries no NRMPOW statement");
        return;
    };
    let wet = rows.iter().filter(|r| r.part == "WET").count();
    let dry = rows.iter().filter(|r| r.part == "DRY").count();
    let complexes: std::collections::BTreeSet<&str> =
        rows.iter().map(|r| r.complex.as_str()).collect();
    let orders: Vec<usize> = rows.iter().map(|r| r.coefficients.len()).collect();
    let degree = match (orders.iter().min(), orders.iter().max()) {
        (Some(a), Some(b)) => format!("{a}..{b}"),
        _ => "absent".to_string(),
    };
    println!(
        "csp-census {path}: {} statement(s), {wet} wet, {dry} dry, complexes {:?}, degree {degree}",
        rows.len(),
        complexes
    );
    if let (Some(first), Some(last)) = (rows.first(), rows.last()) {
        println!(
            "csp-census: first {} {} .. {} · last {} {} .. {}",
            first.part, first.from, first.to, last.part, last.from, last.to
        );
    }
}
