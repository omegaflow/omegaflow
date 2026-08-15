fn parse_three(s: &str) -> Option<(f64, f64, f64)> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let (sign, rest) = match t.as_bytes()[0] {
        b'+' => (1.0, &t[1..]),
        b'-' => (-1.0, &t[1..]),
        _ => (1.0, t),
    };
    let parts: Vec<&str> = rest
        .split(|c: char| c == ' ' || c == ':' || c == '\t')
        .filter(|p| !p.is_empty())
        .collect();
    if parts.len() != 3 {
        return None;
    }
    let a: f64 = parts[0].parse().ok()?;
    let b: f64 = parts[1].parse().ok()?;
    let c: f64 = parts[2].parse().ok()?;
    if !(a >= 0.0) || !(0.0..60.0).contains(&b) || !(0.0..60.0).contains(&c) {
        return None;
    }
    Some((sign * a, b, c))
}

pub fn sexagesimal_ra_to_deg(s: &str) -> Option<f64> {
    let (h, m, sec) = parse_three(s)?;
    if !(0.0..24.0).contains(&h) {
        return None;
    }
    Some(15.0 * (h + m / 60.0 + sec / 3600.0))
}

pub fn sexagesimal_dec_to_deg(s: &str) -> Option<f64> {
    let (d, m, sec) = parse_three(s)?;
    if d.abs() > 90.0 {
        return None;
    }
    Some(d + d.signum() * (m / 60.0 + sec / 3600.0))
}

#[cfg(test)]
mod tests {
    use super::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};

    #[test]
    fn magnetar_ra_hours() {
        let v = sexagesimal_ra_to_deg("01 00 43.14").unwrap();
        assert!((v - 15.17975).abs() < 1e-4);
    }

    #[test]
    fn magnetar_dec_signed() {
        let v = sexagesimal_dec_to_deg("-72 11 33.8").unwrap();
        assert!((v - (-72.1927222)).abs() < 1e-5);
    }

    #[test]
    fn tevcat_crab() {
        assert!((sexagesimal_ra_to_deg("05 34 31.9").unwrap() - 83.63291666).abs() < 1e-5);
        assert!((sexagesimal_dec_to_deg("+22 00 52.2").unwrap() - 22.014500).abs() < 1e-5);
    }

    #[test]
    fn colon_separators() {
        assert!((sexagesimal_ra_to_deg("05:34:31.9").unwrap() - 83.63291666).abs() < 1e-5);
    }

    #[test]
    fn out_of_range_is_void() {
        assert!(sexagesimal_ra_to_deg("25 00 00").is_none());
        assert!(sexagesimal_dec_to_deg("+91 00 00").is_none());
        assert!(sexagesimal_ra_to_deg("01 61 00").is_none());
    }
}
