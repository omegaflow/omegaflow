use omegaflow::archivar::ifms;

fn main() {
    let arg = std::env::args().skip(1).find(|a| !a.starts_with('-'));
    let Some(path) = arg else {
        eprintln!("ifms-census: --file absent — no IFMS file to count");
        return;
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("ifms-census: read void ({path})");
        return;
    };
    let Some(file) = ifms::parse_ifms(&text) else {
        eprintln!("ifms-census: parse void — {path} carries no header or no body");
        return;
    };
    println!(
        "ifms-census {path}: station {} · spacecraft {} · kind {} · type {} · period {} · {} row(s)",
        file.header
            .get("station_id")
            .map(String::as_str)
            .unwrap_or("-"),
        file.header
            .get("spacecraft_id")
            .map(String::as_str)
            .unwrap_or("-"),
        file.header
            .get("dset_kind")
            .map(String::as_str)
            .unwrap_or("-"),
        file.header
            .get("dap_type")
            .map(String::as_str)
            .unwrap_or("-"),
        file.header
            .get("sample_period")
            .map(String::as_str)
            .unwrap_or("-"),
        ifms::rows(&file)
    );
    for body in &file.bodies {
        let first = match body.rows.first().and_then(|r| r.get(1)) {
            Some(v) => v.clone(),
            None => String::new(),
        };
        let last = match body.rows.last().and_then(|r| r.get(1)) {
            Some(v) => v.clone(),
            None => String::new(),
        };
        println!(
            "ifms-census: body_{} {} column(s) {:?} · first {} · last {}",
            body.name,
            body.columns.len(),
            body.columns,
            first,
            last,
        );
    }
}
