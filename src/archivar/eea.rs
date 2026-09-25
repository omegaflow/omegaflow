use super::parquet::{ParquetColumn, ParquetValue};
use super::*;

pub const EEA_AQ_CONTENT_TYPE: &str = "application/json";

pub fn fetch_eea_aq(
    post_url: &str,
    post_body: &str,
    headers: &[(String, String)],
) -> Option<Vec<(String, Vec<u8>)>> {
    let csv = fetch_raw(post_url, Some(post_body), headers)?;
    let urls = split_parquet_urls(&csv)?;
    let mut files = Vec::new();
    for u in urls {
        match fetch_raw_bytes(&u) {
            Some(b) => files.push((u, b)),
            None => eprintln!("eea_aq {}: parquet fetch void — retry in ttl/Φ", u),
        }
    }
    if files.is_empty() { None } else { Some(files) }
}

pub fn split_parquet_urls(csv: &str) -> Option<Vec<String>> {
    let urls: Vec<String> = csv
        .trim_start_matches('\u{feff}')
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("http://") || l.starts_with("https://"))
        .map(str::to_string)
        .collect();
    if urls.is_empty() { None } else { Some(urls) }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EeaMeasurement {
    pub value: f64,
    pub epoch_unix: f64,
    pub unit: Option<String>,
    pub validity: Option<f64>,
    pub verification: Option<f64>,
}

fn num_at(col: Option<&ParquetColumn>, i: usize) -> Option<f64> {
    let c = col?;
    match c.values.get(i)? {
        ParquetValue::Double(v) if v.is_finite() => Some(*v),
        ParquetValue::Float(v) if v.is_finite() => Some(*v as f64),
        ParquetValue::I32(v) => Some(*v as f64),
        ParquetValue::I64(v) => Some(*v as f64),
        ParquetValue::Bytes(b) => {
            let scale = c.scale?;
            if b.len() != 16 || !(0..=38).contains(&scale) {
                return None;
            }
            let unscaled = i128::from_be_bytes(b.as_slice().try_into().ok()?);
            Some(unscaled as f64 * 10f64.powi(-scale))
        }
        _ => None,
    }
}

fn text_at(col: Option<&ParquetColumn>, i: usize) -> Option<String> {
    match col?.values.get(i)? {
        ParquetValue::Bytes(b) => Some(String::from_utf8_lossy(b).into_owned()),
        _ => None,
    }
}

fn unix_of_epoch_col(col: Option<&ParquetColumn>, i: usize) -> Option<f64> {
    match col?.values.get(i)? {
        ParquetValue::Int96(b) => {
            let nanos = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as f64;
            let julian = u32::from_le_bytes([b[8], b[9], b[10], b[11]]) as f64;
            if !(2_400_000.0..2_600_000.0).contains(&julian) || nanos >= 8.64e13 {
                return None;
            }
            Some((julian - 2_440_588.0) * 86_400.0 + nanos / 1.0e9)
        }
        other => {
            let v = match other {
                ParquetValue::I64(v) => *v as f64,
                ParquetValue::I32(v) => *v as f64,
                ParquetValue::Double(v) if v.is_finite() => *v,
                _ => return None,
            };
            if !v.is_finite() {
                return None;
            }
            if (1.0e11..1.0e13).contains(&v) {
                Some(v / 1.0e3)
            } else if (1.0e14..1.0e16).contains(&v) {
                Some(v / 1.0e6)
            } else if (1.0e8..1.0e10).contains(&v) {
                Some(v)
            } else {
                None
            }
        }
    }
}

pub fn latest_measurement(cols: &[ParquetColumn]) -> Option<EeaMeasurement> {
    let col = |name: &str| cols.iter().find(|c| c.name.eq_ignore_ascii_case(name));
    let value_col = col("Value")?;
    let start_col = col("Start");
    let result_col = col("ResultTime");
    let unit_col = col("Unit");
    let validity_col = col("Validity");
    let verification_col = col("Verification");
    for i in (0..value_col.values.len()).rev() {
        let Some(value) = num_at(Some(value_col), i) else {
            continue;
        };
        let Some(epoch_unix) =
            unix_of_epoch_col(start_col, i).or_else(|| unix_of_epoch_col(result_col, i))
        else {
            continue;
        };
        return Some(EeaMeasurement {
            value,
            epoch_unix,
            unit: text_at(unit_col, i),
            validity: num_at(validity_col, i),
            verification: num_at(verification_col, i),
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::parquet::testkit::{Enc, uvarint, zigzag};
    use crate::archivar::parquet::{ParquetColumn, ParquetValue};

    const CT_I32: u8 = 5;
    const CT_BINARY: u8 = 8;
    const CT_LIST: u8 = 9;
    const CT_STRUCT: u8 = 12;

    fn plain_bytes(cols: &[&str]) -> Vec<u8> {
        let mut b = Vec::new();
        for c in cols {
            b.extend((c.len() as u32).to_le_bytes());
            b.extend(c.as_bytes());
        }
        b
    }

    fn data_page_header(body_len: usize, num_values: i32) -> Vec<u8> {
        let mut ph = Enc::new();
        ph.i32(1, 0);
        ph.i32(2, body_len as i32);
        ph.i32(3, body_len as i32);
        ph.field(5, CT_STRUCT);
        {
            let mut d = Enc::new();
            d.i32(1, num_values);
            d.i32(2, 0);
            d.i32(3, body_len as i32);
            d.i32(4, body_len as i32);
            d.stop();
            ph.buf.extend(d.buf);
        }
        ph.stop();
        ph.buf
    }

    struct PlainCol {
        name: &'static str,
        type_tag: i32,
        scale: Option<i32>,
        body: Vec<u8>,
        num_values: i32,
    }

    fn plain_columns_file(cols: &[PlainCol]) -> Vec<u8> {
        let mut file = Vec::new();
        file.extend(b"PAR1");
        let mut leaf_bufs: Vec<Vec<u8>> = Vec::new();
        let mut chunk_bufs: Vec<Vec<u8>> = Vec::new();
        let mut off = 4usize;
        let mut total = 0i64;
        for c in cols {
            let ph = data_page_header(c.body.len(), c.num_values);
            let page_len = (ph.len() + c.body.len()) as i64;
            total += page_len;
            let mut cm = Enc::new();
            cm.i32(1, c.type_tag);
            cm.field(2, CT_LIST);
            cm.buf.push((1 << 4) | CT_I32);
            cm.buf.extend(zigzag(0));
            cm.field(3, CT_LIST);
            cm.buf.push((1 << 4) | CT_BINARY);
            cm.buf.extend(uvarint(c.name.len() as u64));
            cm.buf.extend(c.name.as_bytes());
            cm.i32(4, 0);
            cm.i64(5, c.num_values as i64);
            cm.i64(6, page_len);
            cm.i64(7, page_len);
            cm.i64(9, off as i64);
            cm.stop();
            let mut cc = Enc::new();
            cc.i64(2, off as i64);
            cc.field(3, CT_STRUCT);
            cc.buf.extend(cm.buf);
            cc.stop();
            chunk_bufs.push(cc.buf);
            let mut leaf = Enc::new();
            leaf.i32(1, c.type_tag);
            if c.type_tag == 7 {
                leaf.i32(2, 16);
            }
            leaf.i32(3, 0);
            leaf.binary(4, c.name.as_bytes());
            if let Some(s) = c.scale {
                leaf.i32(7, s);
            }
            leaf.stop();
            leaf_bufs.push(leaf.buf);
            off += page_len as usize;
            file.extend(&ph);
            file.extend(&c.body);
        }
        let mut rg = Enc::new();
        rg.list_struct(1, &chunk_bufs);
        rg.i64(2, total);
        rg.i64(3, 3);
        rg.stop();
        let mut schema: Vec<Vec<u8>> = vec![crate::archivar::parquet::testkit::schema_root_bytes()];
        schema.extend(leaf_bufs);
        let footer = crate::archivar::parquet::testkit::file_metadata_footer(&schema, &[rg.buf]);
        file.extend(&footer);
        file.extend((footer.len() as u32).to_le_bytes());
        file.extend(b"PAR1");
        file
    }

    fn f64_body(vals: &[f64]) -> Vec<u8> {
        vals.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn i32_body(vals: &[i32]) -> Vec<u8> {
        vals.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn int96_body(unix_seconds: &[f64]) -> Vec<u8> {
        let mut b = Vec::new();
        for u in unix_seconds {
            let day = (u / 86_400.0).floor() + 2_440_588.0;
            let nanos = ((u - (day - 2_440_588.0) * 86_400.0) * 1.0e9).round();
            b.extend((nanos as u64).to_le_bytes());
            b.extend((day as u32).to_le_bytes());
        }
        b
    }

    fn decimal_body(vals: &[f64]) -> Vec<u8> {
        let mut b = Vec::new();
        for v in vals {
            let unscaled = (v * 1.0e18).round() as i128;
            b.extend(unscaled.to_be_bytes());
        }
        b
    }

    #[test]
    fn split_parquet_urls_reads_the_csv_body() {
        assert_eq!(
            split_parquet_urls(
                "\u{feff}ParquetFileUrl\nhttps://e.de/SPO.A.parquet\nhttps://e.de/SPO.B.parquet\n"
            ),
            Some(vec![
                "https://e.de/SPO.A.parquet".to_string(),
                "https://e.de/SPO.B.parquet".to_string(),
            ])
        );
        assert!(split_parquet_urls("\u{feff}ParquetFileUrl\n").is_none());
        assert!(split_parquet_urls("").is_none());
        assert!(split_parquet_urls("no urls here\n").is_none());
        assert_eq!(
            split_parquet_urls("\u{feff}ParquetFileUrl\n\nhttps://e.de/SPO.A.parquet\nnot-a-url\n"),
            Some(vec!["https://e.de/SPO.A.parquet".to_string()])
        );
    }

    #[test]
    fn latest_measurement_reads_the_latest_row_from_a_synthetic_parquet_file() {
        let ms = 1_767_225_600_000.0f64;
        let cols = vec![
            PlainCol {
                name: "Start",
                type_tag: 3,
                scale: None,
                body: int96_body(&[ms / 1.0e3, ms / 1.0e3 + 3_600.0, ms / 1.0e3 + 7_200.0]),
                num_values: 3,
            },
            PlainCol {
                name: "Value",
                type_tag: 7,
                scale: Some(18),
                body: decimal_body(&[10.5, 12.0, 12.5]),
                num_values: 3,
            },
            PlainCol {
                name: "Unit",
                type_tag: 6,
                scale: None,
                body: plain_bytes(&["ug.m-3", "ug.m-3", "ug.m-3"]),
                num_values: 3,
            },
            PlainCol {
                name: "Validity",
                type_tag: 1,
                scale: None,
                body: i32_body(&[1, 1, 1]),
                num_values: 3,
            },
            PlainCol {
                name: "Verification",
                type_tag: 1,
                scale: None,
                body: i32_body(&[1, 2, 1]),
                num_values: 3,
            },
        ];
        let bytes = plain_columns_file(&cols);
        let parsed =
            crate::archivar::parquet::parse_parquet(&bytes).expect("synthetic parquet parses");
        let m = latest_measurement(&parsed).expect("the latest row carries the measurement");
        assert!((m.value - 12.5).abs() < 1.0e-9);
        assert_eq!(m.epoch_unix, ms / 1.0e3 + 7200.0);
        assert_eq!(m.unit.as_deref(), Some("ug.m-3"));
        assert_eq!(m.validity, Some(1.0));
        assert_eq!(m.verification, Some(1.0));
    }

    #[test]
    fn latest_measurement_skips_a_null_row_and_reads_the_previous_measurement() {
        let ms = 1_767_225_600_000.0f64;
        let cols = vec![
            PlainCol {
                name: "Start",
                type_tag: 3,
                scale: None,
                body: int96_body(&[ms / 1.0e3, ms / 1.0e3 + 3_600.0]),
                num_values: 2,
            },
            PlainCol {
                name: "Value",
                type_tag: 5,
                scale: None,
                body: f64_body(&[10.5, f64::NAN]),
                num_values: 2,
            },
        ];
        let bytes = plain_columns_file(&cols);
        let parsed =
            crate::archivar::parquet::parse_parquet(&bytes).expect("synthetic parquet parses");
        let m = latest_measurement(&parsed).expect("the previous row carries the measurement");
        assert_eq!(m.value, 10.5);
        assert_eq!(m.epoch_unix, ms / 1.0e3);
    }

    #[test]
    fn latest_measurement_void_when_the_value_column_is_absent() {
        let cols = vec![ParquetColumn {
            name: "Unit".to_string(),
            scale: None,
            values: vec![ParquetValue::Bytes(b"ug.m-3".to_vec())],
        }];
        assert_eq!(latest_measurement(&cols), None);
    }

    #[test]
    fn latest_measurement_void_when_no_row_carries_an_epoch() {
        let cols = vec![
            ParquetColumn {
                name: "Value".to_string(),
                scale: None,
                values: vec![ParquetValue::Double(12.5)],
            },
            ParquetColumn {
                name: "Unit".to_string(),
                scale: None,
                values: vec![ParquetValue::Bytes(b"ug.m-3".to_vec())],
            },
        ];
        assert_eq!(latest_measurement(&cols), None);
    }
}
