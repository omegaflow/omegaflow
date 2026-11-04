use omegaflow::archivar::ifms;

fn column(columns: &[String], name: &str) -> Option<usize> {
    columns.iter().position(|column| column == name)
}

fn main() {
    let path = std::env::args().skip(1).find(|arg| !arg.starts_with('-'));
    let Some(path) = path else {
        eprintln!("ifms-delta: --file absent — no IFMS file to read");
        return;
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("ifms-delta: read void ({path})");
        return;
    };
    let Some(file) = ifms::parse_ifms(&text) else {
        eprintln!("ifms-delta: parse void — {path} is not an IFMS text file");
        return;
    };
    println!(
        "ifms-delta {path}: {} config entries, {} body(ies)",
        file.config.len(),
        file.bodies.len()
    );
    for (key, value) in &file.config {
        if ["Freq", "Rgd", "Rcd", "Scd", "UlmCar"]
            .iter()
            .any(|prefix| key.contains(prefix))
        {
            println!("ifms-delta: {key} = {value}");
        }
    }
    for body in &file.bodies {
        let Some(delta) = column(&body.columns, "DeltaDelay") else {
            continue;
        };
        let lock = column(&body.columns, "CarrLoopStatus");
        let first = body.rows.first().and_then(|row| row.get(delta));
        let last = body.rows.last().and_then(|row| row.get(delta));
        let locked = lock.map_or(0, |index| {
            body.rows
                .iter()
                .filter(|row| {
                    row.get(index)
                        .is_some_and(|value| value.as_str() == "Locked")
                })
                .count()
        });
        println!(
            "ifms-delta: body {} — {} rows; DeltaDelay first {:?}, last {:?}; CarrLoopStatus {locked} Locked of {}",
            body.name,
            body.rows.len(),
            first,
            last,
            body.rows.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_column_index_is_the_named_index() {
        let columns = vec!["Number".to_string(), "DeltaDelay".to_string()];
        assert_eq!(column(&columns, "DeltaDelay"), Some(1));
        assert_eq!(column(&columns, "CarrLoopStatus"), None);
    }
}
