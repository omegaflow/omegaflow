pub struct MadrigalIsprint {
    pub title: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<f64>>>,
}

impl MadrigalIsprint {
    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|column| column == name)
    }

    pub fn series(&self, epoch: &str, value: &str) -> Vec<(f64, f64)> {
        let (Some(epoch_i), Some(value_i)) = (self.column_index(epoch), self.column_index(value))
        else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for row in &self.rows {
            let (Some(epoch), Some(value)) = (
                row.get(epoch_i).copied().flatten(),
                row.get(value_i).copied().flatten(),
            ) else {
                continue;
            };
            out.push((epoch, value));
        }
        out
    }
}

pub fn parse_isprint(bytes: &[u8]) -> Option<MadrigalIsprint> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut lines = text.lines().map(str::trim).filter(|line| !line.is_empty());
    let title = lines.next()?.to_string();
    let columns: Vec<String> = lines
        .next()?
        .split_whitespace()
        .map(str::to_string)
        .collect();
    if columns.is_empty() || columns.iter().any(|column| column.parse::<f64>().is_ok()) {
        return None;
    }
    let mut rows = Vec::new();
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != columns.len() {
            continue;
        }
        rows.push(
            tokens
                .iter()
                .map(|token| token.parse::<f64>().ok().filter(|value| value.is_finite()))
                .collect(),
        );
    }
    if rows.is_empty() {
        return None;
    }
    Some(MadrigalIsprint {
        title,
        columns,
        rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Millstone Hill UHF Zenith Antenna: 2024-01-04 2304:15-2304:25\n    UT1_UNIX          NE      \n1704409455.000            nan  \n1704409455.000    1.86253e+10  \n";

    #[test]
    fn parses_measured_isprint_header_and_rows() {
        let print = parse_isprint(SAMPLE.as_bytes()).expect("the isprint body parses");
        assert_eq!(
            print.title,
            "Millstone Hill UHF Zenith Antenna: 2024-01-04 2304:15-2304:25"
        );
        assert_eq!(print.columns, vec!["UT1_UNIX", "NE"]);
        assert_eq!(print.rows.len(), 2);
        assert_eq!(print.rows[0][0], Some(1704409455.0));
        assert_eq!(print.rows[0][1], None);
        assert_eq!(print.rows[1][1], Some(1.86253e10));
    }

    #[test]
    fn series_drops_absent_cells() {
        let print = parse_isprint(SAMPLE.as_bytes()).unwrap();
        assert_eq!(
            print.series("UT1_UNIX", "NE"),
            vec![(1704409455.0, 1.86253e10)]
        );
        assert!(print.series("UT1_UNIX", "TI").is_empty());
    }

    #[test]
    fn rejects_a_body_without_a_mnemonic_header() {
        assert!(parse_isprint(b"").is_none());
        assert!(parse_isprint(b"title only\n").is_none());
        assert!(parse_isprint(b"title\n1.0 2.0\n3.0 4.0\n").is_none());
    }

    #[test]
    fn rejects_non_utf8() {
        assert!(parse_isprint(&[0xff, 0xfe]).is_none());
    }
}
