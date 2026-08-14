use std::time::{SystemTime, UNIX_EPOCH};

const J2000_UNIX_OFFSET: f64 = 946728000.0;

#[derive(Clone, Debug)]
pub struct LeapSeconds {
    pub delta_t_a: f64,
    pub deltas: Vec<(f64, f64)>,
}

impl LeapSeconds {
    pub fn leap_at(&self, unix: f64) -> Option<f64> {
        self.deltas
            .iter()
            .rev()
            .find(|&&(_, effective)| effective <= unix)
            .map(|&(offset, _)| offset)
    }

    pub fn unix_to_tdb(&self, unix: f64) -> Option<f64> {
        Some(unix + self.delta_t_a + self.leap_at(unix)? - J2000_UNIX_OFFSET)
    }

    pub fn tdb_to_unix(&self, tdb: f64) -> Option<f64> {
        for &(offset, _) in self.deltas.iter().rev() {
            let unix = tdb - self.delta_t_a - offset + J2000_UNIX_OFFSET;
            if self.leap_at(unix) == Some(offset) {
                return Some(unix);
            }
        }
        None
    }

    pub fn system_now_tdb(&self) -> Option<f64> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => self.unix_to_tdb(d.as_secs_f64()),
            Err(_) => None,
        }
    }
}

pub fn parse(text: &str) -> Option<LeapSeconds> {
    let delta_t_a = scan_plain_value(text, "DELTET/DELTA_T_A")?;
    let (values, complete) = scan_paren_mixed(text, "DELTET/DELTA_AT")?;
    if !complete {
        return None;
    }
    let mut deltas = Vec::new();
    let mut it = values.iter();
    while let Some(v) = it.next() {
        let date = it.next()?;
        deltas.push((*v, *date));
    }
    if deltas.is_empty() {
        return None;
    }
    Some(LeapSeconds { delta_t_a, deltas })
}

fn scan_plain_value(text: &str, key: &str) -> Option<f64> {
    let pos = text.find(key)?;
    let rest = &text[pos + key.len()..];
    let eq = rest.find('=')?;
    let after_eq = &rest[eq + 1..];
    let line = after_eq.lines().next()?;
    line.trim().split_whitespace().next()?.parse().ok()
}

fn scan_paren_mixed(text: &str, key: &str) -> Option<(Vec<f64>, bool)> {
    let pos = text.find(key)?;
    let rest = &text[pos + key.len()..];
    let eq = rest.find('=')?;
    let after_eq = &rest[eq + 1..];
    let open = after_eq.find('(')?;
    let body = &after_eq[open + 1..];
    let close = body.find(')')?;
    let mut values = Vec::new();
    let mut complete = true;
    for tok in body[..close].split_whitespace() {
        let clean = tok.trim_end_matches(',');
        if let Some(date) = clean.strip_prefix('@') {
            values.push(date_unix(date)? as f64);
            continue;
        }
        match clean.parse::<f64>() {
            Ok(v) => values.push(v),
            Err(_) => {
                complete = false;
                break;
            }
        }
    }
    Some((values, complete))
}

fn date_unix(date: &str) -> Option<f64> {
    let (y, rest) = date.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    let year: i64 = y.parse().ok()?;
    let month = month_of(m)?;
    let day: i64 = d.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0)
}

fn month_of(name: &str) -> Option<i64> {
    match name {
        "JAN" => Some(1),
        "FEB" => Some(2),
        "MAR" => Some(3),
        "APR" => Some(4),
        "MAY" => Some(5),
        "JUN" => Some(6),
        "JUL" => Some(7),
        "AUG" => Some(8),
        "SEP" => Some(9),
        "OCT" => Some(10),
        "NOV" => Some(11),
        "DEC" => Some(12),
        _ => None,
    }
}

fn days_from_civil(year: i64, month: i64, day: i64) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146097 + doe - 719468)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_naif_style_entries() {
        let text = "KPL/LSK\n\
[2]       DELTA_AT  =  TAI - UTC\n\
[3]       DELTA_ET  =  ET - (TAI - DELTA_AT)\n\
\\begindata\n\n\
DELTET/DELTA_T_A       =   32.184\n\
DELTET/K               =    1.657D-3\n\
DELTET/M               = (  6.239996D0   1.99096871D-7 )\n\n\
DELTET/DELTA_AT        = ( 10,   @1972-JAN-1,\n 37,   @2017-JAN-1 )\n";
        let lsk = parse(text).unwrap();
        assert!((lsk.delta_t_a - 32.184).abs() < 1e-12);
        let now = 1700000000.0;
        assert_eq!(lsk.leap_at(now), Some(37.0));
        assert_eq!(lsk.unix_to_tdb(now), Some(now + 69.184 - J2000_UNIX_OFFSET));
        assert_eq!(lsk.tdb_to_unix(lsk.unix_to_tdb(now).unwrap()), Some(now));
    }

    #[test]
    fn absent_entries_leave_none() {
        assert!(parse("DELTET/DELTA_T_A = ( 32.184 )\n").is_none());
        assert!(parse("DELTA_AT = ( 37, @2017-JAN-1 )\n").is_none());
    }
}

