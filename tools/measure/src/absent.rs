pub fn number(v: Option<f64>, precision: usize) -> String {
    match v {
        Some(x) => format!("{x:.precision$}"),
        None => "absent".to_string(),
    }
}

pub fn scientific(v: Option<f64>, precision: usize) -> String {
    match v {
        Some(x) => format!("{x:.precision$e}"),
        None => "absent".to_string(),
    }
}
