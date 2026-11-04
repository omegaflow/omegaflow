#[derive(Clone, Debug, PartialEq)]
pub struct CspAdjust {
    pub part: String,
    pub complex: String,
    pub from: String,
    pub to: String,
    pub coefficients: Vec<f64>,
    pub fitsig: Option<f64>,
}

fn field_after<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let start = text.find(key)? + key.len();
    let rest = &text[start..];
    let end = rest.find(')')?;
    Some(rest[..end].trim())
}

fn list_after<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let start = text.find(key)? + key.len();
    let rest = &text[start..];
    let end = rest.find(')')?;
    Some(&rest[..end])
}

fn window_seconds(stamp: &str) -> Option<f64> {
    let (date, clock) = stamp.split_once(',')?;
    let mut parts = date.split('/');
    let yy: i64 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    let year = if yy < 70 { 2000 + yy } else { 1900 + yy };
    let mut clock_parts = clock.split(':');
    let hour: u32 = clock_parts.next()?.parse().ok()?;
    let minute: u32 = clock_parts.next()?.parse().ok()?;
    if clock_parts.next().is_some() {
        return None;
    }
    let iso = format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:00");
    crate::archivar::parse_iso_unix(&iso)
}

impl CspAdjust {
    pub fn span(&self) -> Option<(f64, f64)> {
        let start = window_seconds(&self.from)?;
        let end = window_seconds(&self.to)?;
        if !(end > start) {
            return None;
        }
        Some((start, end))
    }

    pub fn at_extrapolated(&self, unix: f64) -> Option<f64> {
        let (start, end) = self.span()?;
        if !unix.is_finite() {
            return None;
        }
        let x = 2.0 * ((unix - start) / (end - start)) - 1.0;
        let mut value = 0.0;
        for coefficient in self.coefficients.iter().rev() {
            value = value * x + coefficient;
        }
        value.is_finite().then_some(value)
    }

    pub fn at(&self, unix: f64) -> Option<f64> {
        let (start, end) = self.span()?;
        if !unix.is_finite() || unix < start || unix > end {
            return None;
        }
        let x = 2.0 * ((unix - start) / (end - start)) - 1.0;
        let mut value = 0.0;
        for coefficient in self.coefficients.iter().rev() {
            value = value * x + coefficient;
        }
        value.is_finite().then_some(value)
    }
}

#[cfg(test)]
mod window_tests {
    use super::*;

    const SAMPLE: &str = "# FITSIG= .0154976
ADJUST(DOPRNG)BY NRMPOW(   0.6721,  -0.1052,   1.6716)                 MODEL(CHPART)
FROM(05/03/01,02:16)TO(05/03/01,14:24)DSN( 74)SCID(226).   #S01
";

    #[test]
    fn a_window_parses_to_unix_seconds() {
        let adjustments = parse_csp(SAMPLE).expect("adjustments");
        let (start, end) = adjustments[0].span().expect("a span");
        assert!(end > start);
        assert!((end - start - 12.0 * 3600.0 - 8.0 * 60.0).abs() < 1.0);
    }

    #[test]
    fn the_series_hits_its_endpoints() {
        let adjustments = parse_csp(SAMPLE).expect("adjustments");
        let (start, end) = adjustments[0].span().expect("a span");
        let at_start = adjustments[0].at(start).expect("a value");
        let at_end = adjustments[0].at(end).expect("a value");
        let expected_start = 0.6721 - 0.1052 + 1.6716;
        let expected_end = 0.6721 + 0.1052 + 1.6716;
        assert!((at_start - expected_start).abs() < 1e-12);
        assert!((at_end - expected_end).abs() < 1e-12);
    }

    #[test]
    fn the_midpoint_is_the_constant_term() {
        let adjustments = parse_csp(SAMPLE).expect("adjustments");
        let (start, end) = adjustments[0].span().expect("a span");
        let middle = adjustments[0].at(0.5 * (start + end)).expect("a value");
        assert!((middle - 0.6721).abs() < 1e-12);
    }

    #[test]
    fn outside_the_window_is_absent() {
        let adjustments = parse_csp(SAMPLE).expect("adjustments");
        let (start, _) = adjustments[0].span().expect("a span");
        assert!(adjustments[0].at(start - 1.0).is_none());
    }
}

pub fn parse_csp(text: &str) -> Option<Vec<CspAdjust>> {
    let mut out = Vec::new();
    let mut statement = String::new();
    let mut fitsig: Option<f64> = None;
    for raw in text.lines() {
        let line = raw.trim();
        let line = match line.find(" #") {
            Some(i) => line[..i].trim_end(),
            None => line,
        };
        if let Some(rest) = line.strip_prefix("# FITSIG=") {
            fitsig = rest.trim().parse().ok();
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if !statement.is_empty() {
            statement.push(' ');
        }
        statement.push_str(line);
        if !line.ends_with('.') {
            continue;
        }
        let done = std::mem::take(&mut statement);
        if !done.starts_with("ADJUST") {
            fitsig = None;
            continue;
        }
        let Some(coeffs) = list_after(&done, "NRMPOW(") else {
            fitsig = None;
            continue;
        };
        let mut coefficients = Vec::new();
        let mut bad = false;
        for token in coeffs.split(|c: char| c == ',' || c.is_whitespace()) {
            if token.is_empty() {
                continue;
            }
            match token.parse::<f64>() {
                Ok(v) => coefficients.push(v),
                Err(_) => bad = true,
            }
        }
        if bad || coefficients.is_empty() {
            fitsig = None;
            continue;
        }
        let part = if let Some(i) = done.find("(WET ") {
            let rest = &done[i + 1..];
            rest.split_whitespace().next().unwrap_or("").to_string()
        } else if let Some(i) = done.find("(DRY ") {
            let rest = &done[i + 1..];
            rest.split_whitespace().next().unwrap_or("").to_string()
        } else if let Some(model) = field_after(&done, "MODEL(") {
            model.to_string()
        } else {
            continue;
        };
        let Some(from) = field_after(&done, "FROM(") else {
            fitsig = None;
            continue;
        };
        let Some(to) = field_after(&done, "TO(") else {
            fitsig = None;
            continue;
        };
        let Some(complex) = field_after(&done, "DSN(") else {
            fitsig = None;
            continue;
        };
        out.push(CspAdjust {
            part,
            complex: complex.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            coefficients,
            fitsig,
        });
        fitsig = None;
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# FITSIG= .0011118
ADJUST(ALL)BY NRMPOW(   0.0061,   0.0226,  -0.0378)                          MODEL
(WET NUPART)FROM(05/02/01,06:00:00.001)TO(05/02/01,18:00:00)DSN(C10). #    050208 17:43
# FITSIG= .0003008
ADJUST(ALL)BY NRMPOW(   0.0020,  -0.0004)                                     MODEL
(DRY NUPART)FROM(05/02/01,06:00:00.001)TO(05/02/01,18:00:00)DSN(C40). #    050208 17:43
";

    #[test]
    fn a_nrmpow_statement_carries_part_complex_and_coefficients() {
        let rows = parse_csp(SAMPLE).expect("statements");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].part, "WET");
        assert_eq!(rows[0].complex, "C10");
        assert_eq!(rows[0].coefficients.len(), 3);
        assert_eq!(rows[0].coefficients[0], 0.0061);
        assert_eq!(rows[0].fitsig, Some(0.0011118));
        assert_eq!(rows[1].part, "DRY");
        assert_eq!(rows[1].complex, "C40");
    }

    #[test]
    fn a_chpart_statement_reads_without_wet_or_dry() {
        let text = "# FITSIG= .0366583
ADJUST(DOPRNG)BY NRMPOW(   0.9246,  -0.1937)                 MODEL(CHPART)
FROM(05/02/01,03:57)TO(05/02/01,16:13)DSN(C10)SCID(226).   #S01 ADJ 050204 16:31
";
        let rows = parse_csp(text).expect("a statement");
        assert_eq!(rows[0].part, "CHPART");
        assert_eq!(rows[0].complex, "C10");
        assert_eq!(rows[0].coefficients.len(), 2);
    }

    #[test]
    fn text_without_an_adjust_statement_is_absent() {
        assert!(parse_csp("# only a comment\n").is_none());
    }

    #[test]
    fn a_statement_without_coefficients_is_skipped() {
        let text = SAMPLE.replace("NRMPOW(   0.0061,   0.0226,  -0.0378)", "NRMPOW()");
        let rows = parse_csp(&text).expect("one statement");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].part, "DRY");
    }
}
