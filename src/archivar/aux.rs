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
}
