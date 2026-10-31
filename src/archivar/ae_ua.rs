#[derive(Debug, Clone)]
pub struct AeBlock {
    pub ivar: i32,
    pub ib: usize,
    pub ie: usize,
    pub orbit: i64,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct AeDay {
    pub year_day: i64,
    pub blocks: Vec<AeBlock>,
}

pub const FILL: f64 = 1.0e35;

fn parse_int(token: &str) -> Option<i64> {
    token.trim().parse::<i64>().ok()
}

fn values_of_line(line: &str) -> Option<Vec<f64>> {
    let chars: Vec<char> = line
        .chars()
        .filter(|c| c != &char::from(10) && c != &char::from(13))
        .collect();
    if chars.len() >= 10 {
        let mut values = Vec::new();
        let mut ok = true;
        let mut i = 0;
        while i < chars.len() {
            let end = (i + 10).min(chars.len());
            let chunk: String = chars[i..end].iter().collect();
            let token = chunk.trim();
            if !token.is_empty() {
                match token.parse::<f64>() {
                    Ok(v) if v.is_finite() => values.push(v),
                    _ => {
                        ok = false;
                        break;
                    }
                }
            }
            i = end;
        }
        if ok && !values.is_empty() {
            return Some(values);
        }
    }
    let mut values = Vec::new();
    for token in line.split_whitespace() {
        match token.parse::<f64>() {
            Ok(v) if v.is_finite() => values.push(v),
            _ => return None,
        }
    }
    Some(values)
}

pub fn parse(text: &str) -> Option<AeDay> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = lines.next()?;
    let year_day = parse_int(header.get(0..6)?)?;

    let mut blocks: Vec<AeBlock> = Vec::new();
    let mut pending: Option<(i32, usize, usize, i64)> = None;
    let mut values: Vec<f64> = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        let looks_int =
            tokens.len() >= 4 && tokens.iter().take(4).all(|t| t.parse::<i64>().is_ok());
        if looks_int && pending.is_some() {
            let (ivar, ib, ie, orbit) = pending.take()?;
            let expected = ie.saturating_sub(ib) + 1;
            values.truncate(expected);
            blocks.push(AeBlock {
                ivar,
                ib,
                ie,
                orbit,
                values: std::mem::take(&mut values),
            });
        }
        if looks_int {
            let ivar = tokens[0].parse::<i32>().ok()?;
            let ib = tokens[1].parse::<usize>().ok()?;
            let ie = tokens[2].parse::<usize>().ok()?;
            let orbit = tokens[3].parse::<i64>().ok()?;
            pending = Some((ivar, ib, ie, orbit));
            values.clear();
            continue;
        }
        match values_of_line(line) {
            Some(v) => values.extend(v),
            None => return None,
        }
    }
    if let Some((ivar, ib, ie, orbit)) = pending.take() {
        let expected = ie.saturating_sub(ib) + 1;
        values.truncate(expected);
        blocks.push(AeBlock {
            ivar,
            ib,
            ie,
            orbit,
            values,
        });
    }
    Some(AeDay { year_day, blocks })
}

fn civil_to_days(year: i64, month: i64, day: i64) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146097 + doe - 719468)
}

pub fn day_start_unix(year_day: i64) -> Option<f64> {
    let year = 1900 + year_day / 1000;
    let day = year_day % 1000;
    if !(1..=366).contains(&day) {
        return None;
    }
    let days = civil_to_days(year, 1, 1)? + (day - 1);
    Some(days as f64 * 86400.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "76002   19   12    9    5    4    8   21   10    9   20 18 12 16 20 12 18   8 82.00   79.70         \n  -1   2545   2576     2\n 1.119E+02 1.128E+02 1.136E+02 1.144E+02 1.152E+02 1.161E+02 1.169E+02 1.178E+02 1.186E+02 1.195E+02 1.203E+02 1.212E+02 1.220E+02 1.229E+02 1.238E+02\n 1.247E+02 1.256E+02 1.264E+02 1.273E+02 1.282E+02 1.291E+02 1.300E+02 1.309E+02 1.318E+02 1.328E+02 1.337E+02 1.346E+02 1.355E+02 1.364E+02 1.373E+02\n 1.383E+02 1.392E+02\n   46   3000   3014     7\n 1.0E+35 2.0E+01 -3.0E+01 4.0E+01 -5.0E+01 6.0E+01 -7.0E+01 8.0E+01 -9.0E+01 1.0E+02 -1.1E+02 1.2E+02 -1.3E+02 1.4E+02 -1.5E+02\n";

    #[test]
    fn a_day_parses_into_blocks() {
        let day = parse(SAMPLE).expect("a day");
        assert_eq!(day.year_day, 76002);
        assert_eq!(day.blocks.len(), 2);
        assert_eq!(day.blocks[0].ivar, -1);
        assert_eq!(day.blocks[0].ib, 2545);
        assert_eq!(day.blocks[0].ie, 2576);
        assert_eq!(day.blocks[0].values.len(), 32);
        assert!((day.blocks[0].values[0] - 111.9).abs() < 1e-9);
        assert_eq!(day.blocks[1].ivar, 46);
        assert_eq!(day.blocks[1].values.len(), 15);
        assert!((day.blocks[1].values[0] - FILL).abs() < 1.0);
    }

    #[test]
    fn the_day_start_is_new_years_day_1976() {
        let unix = day_start_unix(76002).expect("a day start");
        assert_eq!(unix, 189388800.0);
        let first = day_start_unix(76001).expect("a day start");
        assert_eq!(unix - first, 86400.0);
    }
}
