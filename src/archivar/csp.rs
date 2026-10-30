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
