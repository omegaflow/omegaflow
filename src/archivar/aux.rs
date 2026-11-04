#[derive(Clone, Debug, PartialEq)]
pub struct AuxBlock {
    pub header: Vec<i64>,
    pub records: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AuxFile {
    pub file_type: i64,
    pub columns: usize,
    pub derivatives: usize,
    pub block_header_size: usize,
    pub blocks: Vec<AuxBlock>,
}

fn after_colon<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(key)?;
    Some(rest.trim())
}

pub fn parse_aux(text: &str) -> Option<AuxFile> {
    let mut file = AuxFile::default();
    let mut saw_header = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(value) = after_colon(line, "File type:") {
            file.file_type = value.parse().ok()?;
            saw_header = true;
            continue;
        }
        if let Some(value) = after_colon(line, "Number of columns:") {
            file.columns = value.parse().ok()?;
            continue;
        }
        if let Some(value) = after_colon(line, "Number of derivatives:") {
            file.derivatives = value.parse().ok()?;
            continue;
        }
        if let Some(value) = after_colon(line, "Block header size:") {
            file.block_header_size = value.parse().ok()?;
            continue;
        }
        if line.starts_with("Key type:") || line.starts_with("Value type:") {
            continue;
        }
        if let Some(value) = after_colon(line, "User file header:") {
            let _ = value;
            continue;
        }
        if let Some(value) = after_colon(line, "Block header:") {
            let header: Vec<i64> = value
                .split_whitespace()
                .filter_map(|t| t.parse().ok())
                .collect();
            if header.is_empty() {
                return None;
            }
            file.blocks.push(AuxBlock {
                header,
                records: Vec::new(),
            });
            continue;
        }
        if line.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '+') {
            let Some(block) = file.blocks.last_mut() else {
                return None;
            };
            let mut record = Vec::new();
            for token in line.split_whitespace() {
                record.push(token.parse::<f64>().ok()?);
            }
            if record.len() != file.columns + 1 {
                return None;
            }
            block.records.push(record);
        }
    }
    if !saw_header || file.blocks.is_empty() || file.columns == 0 {
        return None;
    }
    if file.blocks.iter().any(|b| b.records.is_empty()) {
        return None;
    }
    Some(file)
}

pub fn records(file: &AuxFile) -> usize {
    file.blocks.iter().map(|b| b.records.len()).sum()
}

pub fn span(file: &AuxFile) -> Option<(f64, f64)> {
    let mut first: Option<f64> = None;
    let mut last: Option<f64> = None;
    for block in &file.blocks {
        for record in &block.records {
            let t = record[0];
            if first.is_none_or(|f| t < f) {
                first = Some(t);
            }
            if last.is_none_or(|l| t > l) {
                last = Some(t);
            }
        }
    }
    Some((first?, last?))
}

pub fn column_range(file: &AuxFile, index: usize) -> Option<(f64, f64)> {
    let mut low: Option<f64> = None;
    let mut high: Option<f64> = None;
    for block in &file.blocks {
        for record in &block.records {
            let v = *record.get(index + 1)?;
            if low.is_none_or(|l| v < l) {
                low = Some(v);
            }
            if high.is_none_or(|h| v > h) {
                high = Some(v);
            }
        }
    }
    Some((low?, high?))
}

pub const MJD2000_ORIGIN_UNIX: f64 = 946_684_800.0;

pub fn unix_from_mjd2000(mjd2000: f64) -> f64 {
    MJD2000_ORIGIN_UNIX + mjd2000 * 86_400.0
}

pub fn record_pairs<'a>(file: &'a AuxFile) -> impl Iterator<Item = (&'a [f64], &'a [f64])> {
    file.blocks.iter().flat_map(move |block| {
        block
            .records
            .windows(2)
            .map(move |pair| (pair[0].as_slice(), pair[1].as_slice()))
    })
}

pub fn interpolate_at(file: &AuxFile, mjd2000: f64) -> Option<(Vec<f64>, f64, f64)> {
    let (left, right) = record_pairs(file).find(|(a, b)| a[0] <= mjd2000 && mjd2000 <= b[0])?;
    let span = right[0] - left[0];
    if span <= 0.0 {
        return None;
    }
    let w = (mjd2000 - left[0]) / span;
    let mut values = Vec::with_capacity(left.len().saturating_sub(1));
    for index in 1..left.len() {
        let a = *left.get(index)?;
        let b = *right.get(index)?;
        values.push(a + (b - a) * w);
    }
    Some((values, left[0], right[0]))
}

pub fn peak(file: &AuxFile, columns: usize) -> Option<(f64, f64, Vec<f64>)> {
    let mut best: Option<(f64, f64, Vec<f64>)> = None;
    for block in &file.blocks {
        for record in &block.records {
            let mut norm = 0.0;
            let mut values = Vec::with_capacity(columns);
            for index in 0..columns {
                let v = *record.get(index + 1)?;
                values.push(v);
                norm += v * v;
            }
            let norm = norm.sqrt();
            if best.as_ref().is_none_or(|(b, _, _)| norm > *b) {
                best = Some((norm, record[0], values));
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# a comment
File type: 6
Key type: double
Value type: double
Number of columns: 2
Number of derivatives: 0
Block header size: 32
User file header: 0 11 0 0

# Block 7, 2 records,      1874.5840423192931 --      1874.5919644884530
Block header:  0 3 0 0
     1874.5840423192931    -0.80120404773575216     0.17561048387722181
     1874.5919644884530    -0.80120404773575216     0.17561048387722181
";

    #[test]
    fn an_aux_file_carries_header_columns_and_blocks() {
        let file = parse_aux(SAMPLE).expect("an aux file");
        assert_eq!(file.file_type, 6);
        assert_eq!(file.columns, 2);
        assert_eq!(file.block_header_size, 32);
        assert_eq!(file.blocks.len(), 1);
        assert_eq!(file.blocks[0].header, vec![0, 3, 0, 0]);
        assert_eq!(records(&file), 2);
        assert_eq!(file.blocks[0].records[0][0], 1874.5840423192931);
    }

    #[test]
    fn a_record_with_the_wrong_arity_is_refused() {
        let text = SAMPLE.replace(
            "     1874.5919644884530    -0.80120404773575216     0.17561048387722181",
            "     1874.5919644884530    -0.80120404773575216",
        );
        assert!(parse_aux(&text).is_none());
    }

    #[test]
    fn a_file_without_block_is_absent() {
        let text = SAMPLE.replace("Block header:  0 3 0 0", "");
        assert!(parse_aux(&text).is_none());
    }

    #[test]
    fn span_and_column_range_read_the_records() {
        let file = parse_aux(SAMPLE).expect("an aux file");
        let (first, last) = span(&file).expect("span");
        assert!((last - first - 0.00792216916).abs() < 1e-9);
        let (low, high) = column_range(&file, 0).expect("range");
        assert!((low + 0.80120404773575216).abs() < 1e-12);
        assert!((high + 0.80120404773575216).abs() < 1e-12);
        let (low2, high2) = column_range(&file, 1).expect("range");
        assert!(low2 <= high2);
    }

    #[test]
    fn the_epoch_origin_is_the_first_of_january_twenty_hundred() {
        assert_eq!(unix_from_mjd2000(0.0), 946_684_800.0);
        assert_eq!(unix_from_mjd2000(1.0), 946_771_200.0);
    }

    #[test]
    fn an_interpolated_point_sits_between_its_two_records() {
        let text = SAMPLE
            .replace(
                "     1874.5919644884530    -0.80120404773575216     0.17561048387722181",
                "     1874.5950000000000    -0.60120404773575216     0.27561048387722181",
            )
            .replace("Number of columns: 2", "Number of columns: 2");
        let file = parse_aux(&text).expect("an aux file");
        let middle = 1874.5920000000000;
        let (values, left, right) = interpolate_at(&file, middle).expect("a bracket");
        assert_eq!(left, 1874.5840423192931);
        assert_eq!(right, 1874.5950000000000);
        let w = (middle - left) / (right - left);
        let expected = -0.80120404773575216 + (-0.60120404773575216 + 0.80120404773575216) * w;
        assert!((values[0] - expected).abs() < 1e-15);
    }

    #[test]
    fn an_interpolation_outside_the_records_is_absent() {
        let file = parse_aux(SAMPLE).expect("an aux file");
        assert!(interpolate_at(&file, 1875.0).is_none());
        assert!(interpolate_at(&file, 1800.0).is_none());
    }

    #[test]
    fn the_peak_carries_its_epoch_and_components() {
        let file = parse_aux(SAMPLE).expect("an aux file");
        let (norm, at, values) = peak(&file, 2).expect("a peak");
        assert!(
            (norm - (0.80120404773575216f64.powi(2) + 0.17561048387722181f64.powi(2)).sqrt()).abs()
                < 1e-15
        );
        assert_eq!(at, 1874.5840423192931);
        assert_eq!(values.len(), 2);
    }
}
