pub const LINE_COUNT: usize = 12_000;
pub const SAMPLE_COUNT: usize = 20_000;
pub const ELEMENT_BYTES: usize = 4;
pub const ELEMENT_TYPE: &str = "IEEE754LSBSingle";

#[derive(Clone, Debug, PartialEq)]
pub struct Raster {
    pub lines: usize,
    pub samples: usize,
    pub values: Vec<Option<f64>>,
}

pub fn component_name(comp: u32) -> String {
    format!("venus_radar_line_{comp:05}")
}

fn sample_at(bytes: &[u8], index: usize) -> Option<f64> {
    let at = index.checked_mul(ELEMENT_BYTES)?;
    let cell = bytes.get(at..at + ELEMENT_BYTES)?;
    let v = f32::from_le_bytes([cell[0], cell[1], cell[2], cell[3]]);
    if v.is_finite() { Some(v as f64) } else { None }
}

pub fn decode_map(bytes: &[u8], lines: usize, samples: usize) -> Option<Raster> {
    let count = lines.checked_mul(samples)?;
    let expected = count.checked_mul(ELEMENT_BYTES)?;
    if lines == 0 || samples == 0 || bytes.len() != expected {
        return None;
    }
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        values.push(sample_at(bytes, index));
    }
    Some(Raster {
        lines,
        samples,
        values,
    })
}

pub fn parse_raster(bytes: &[u8]) -> Option<Raster> {
    decode_map(bytes, LINE_COUNT, SAMPLE_COUNT)
}

pub fn line_means(bytes: &[u8], lines: usize, samples: usize) -> Option<Vec<(f64, f64, u32)>> {
    let count = lines.checked_mul(samples)?;
    let expected = count.checked_mul(ELEMENT_BYTES)?;
    if lines == 0 || samples == 0 || bytes.len() != expected {
        return None;
    }
    let mut out = Vec::with_capacity(lines);
    let mut index = 0usize;
    for line in 0..lines {
        let mut sum = 0.0f64;
        let mut present = 0usize;
        for _ in 0..samples {
            if let Some(v) = sample_at(bytes, index) {
                sum += v;
                present += 1;
            }
            index += 1;
        }
        if present > 0 {
            out.push((line as f64, sum / present as f64, line as u32));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    line_means(bytes, LINE_COUNT, SAMPLE_COUNT)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn img(values: &[f32]) -> Vec<u8> {
        let mut out = Vec::with_capacity(values.len() * ELEMENT_BYTES);
        for v in values {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    #[test]
    fn the_constants_are_the_measured_label_axes() {
        assert_eq!(LINE_COUNT, 12_000);
        assert_eq!(SAMPLE_COUNT, 20_000);
        assert_eq!(ELEMENT_BYTES, 4);
        assert_eq!(ELEMENT_TYPE, "IEEE754LSBSingle");
        assert_eq!(LINE_COUNT * SAMPLE_COUNT * ELEMENT_BYTES, 960_000_000);
    }

    #[test]
    fn decode_map_reads_row_major_with_samples_fastest() {
        let bytes = img(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let raster = decode_map(&bytes, 2, 3).expect("map decodes");
        assert_eq!(raster.lines, 2);
        assert_eq!(raster.samples, 3);
        assert_eq!(
            raster.values,
            vec![
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
            ]
        );
    }

    #[test]
    fn line_means_group_the_samples_fastest_rows() {
        let bytes = img(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let rows = line_means(&bytes, 2, 3).expect("map decodes");
        assert_eq!(rows, vec![(0.0, 2.0, 0), (1.0, 5.0, 1)]);
    }

    #[test]
    fn non_finite_samples_stay_absent_in_the_decode_and_the_mean() {
        let bytes = img(&[1.0, f32::NAN, 3.0, 4.0, f32::INFINITY, 6.0]);
        let raster = decode_map(&bytes, 2, 3).expect("map decodes");
        assert_eq!(
            raster.values,
            vec![Some(1.0), None, Some(3.0), Some(4.0), None, Some(6.0)]
        );
        let rows = line_means(&bytes, 2, 3).expect("map decodes");
        assert_eq!(rows, vec![(0.0, 2.0, 0), (1.0, 5.0, 1)]);
    }

    #[test]
    fn a_fully_absent_line_carries_no_channel() {
        let bytes = img(&[f32::NAN, f32::NAN, 7.0, 9.0]);
        let rows = line_means(&bytes, 2, 2).expect("the present line carries a channel");
        assert_eq!(rows, vec![(1.0, 8.0, 1)]);
    }

    #[test]
    fn foreign_misaligned_and_empty_bytes_refuse() {
        assert!(decode_map(b"XXXX", 2, 3).is_none());
        assert!(decode_map(&[0u8; 20], 2, 3).is_none());
        assert!(line_means(&[0u8; 20], 2, 3).is_none());
        assert!(parse_series(b"").is_none());
    }

    #[test]
    fn component_names_index_the_declared_line_axis() {
        assert_eq!(component_name(0), "venus_radar_line_00000");
        assert_eq!(component_name(11_999), "venus_radar_line_11999");
    }
}
