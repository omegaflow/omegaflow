#[derive(Clone, Debug, PartialEq)]
pub struct AemSegment {
    pub object_name: String,
    pub object_id: String,
    pub ref_frame_a: String,
    pub ref_frame_b: String,
    pub time_system: String,
    pub start_time: String,
    pub stop_time: String,
    pub attitude_type: String,
    pub interpolation_method: String,
    pub interpolation_degree: Option<u32>,
    pub quaternion_type: Option<String>,
    pub euler_rot_seq: Option<String>,
    pub records: Vec<AemRecord>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AemRecord {
    pub time: String,
    pub values: Vec<f64>,
}

impl AemRecord {
    pub fn quaternion_qc_last(&self) -> Option<[f64; 4]> {
        if self.values.len() != 4 {
            return None;
        }
        Some([
            self.values[0],
            self.values[1],
            self.values[2],
            self.values[3],
        ])
    }
}

fn key_value(line: &str) -> Option<(&str, &str)> {
    let (k, v) = line.split_once('=')?;
    Some((k.trim(), v.trim()))
}

fn parse_record(line: &str) -> Option<AemRecord> {
    let mut it = line.split_whitespace();
    let time = it.next()?.to_string();
    if !time.contains('T') {
        return None;
    }
    let mut values = Vec::new();
    for token in it {
        values.push(token.parse().ok()?);
    }
    if values.len() != 3 && values.len() != 4 {
        return None;
    }
    Some(AemRecord { time, values })
}

pub fn parse_aem(text: &str) -> Option<Vec<AemSegment>> {
    let mut segments: Vec<AemSegment> = Vec::new();
    let mut version = String::new();
    let mut current: Option<AemSegment> = None;
    let mut in_meta = false;
    let mut in_data = false;
    let mut saw_data = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("COMMENT") {
            continue;
        }
        if line == "META_START" {
            in_meta = true;
            in_data = false;
            if let Some(done) = current.take() {
                if done.records.is_empty() {
                    return None;
                }
                segments.push(done);
            }
            current = Some(AemSegment {
                object_name: String::new(),
                object_id: String::new(),
                ref_frame_a: String::new(),
                ref_frame_b: String::new(),
                time_system: String::new(),
                start_time: String::new(),
                stop_time: String::new(),
                attitude_type: String::new(),
                interpolation_method: String::new(),
                interpolation_degree: None,
                quaternion_type: None,
                euler_rot_seq: None,
                records: Vec::new(),
            });
            continue;
        }
        if line == "META_STOP" {
            in_meta = false;
            continue;
        }
        if line == "DATA_START" {
            in_data = true;
            continue;
        }
        if line == "DATA_STOP" {
            in_data = false;
            continue;
        }
        if in_data {
            if let Some(record) = parse_record(line)
                && let Some(seg) = current.as_mut()
            {
                saw_data = true;
                seg.records.push(record);
            }
            continue;
        }
        let Some((key, value)) = key_value(line) else {
            continue;
        };
        if version.is_empty() && key == "CCSDS_AEM_VERS" {
            version = value.to_string();
            continue;
        }
        let Some(seg) = current.as_mut() else {
            continue;
        };
        if !in_meta {
            continue;
        }
        match key {
            "OBJECT_NAME" => seg.object_name = value.to_string(),
            "OBJECT_ID" => seg.object_id = value.to_string(),
            "REF_FRAME_A" => seg.ref_frame_a = value.to_string(),
            "REF_FRAME_B" => seg.ref_frame_b = value.to_string(),
            "TIME_SYSTEM" => seg.time_system = value.to_string(),
            "START_TIME" => seg.start_time = value.to_string(),
            "STOP_TIME" => seg.stop_time = value.to_string(),
            "ATTITUDE_TYPE" => seg.attitude_type = value.to_string(),
            "INTERPOLATION_METHOD" => seg.interpolation_method = value.to_string(),
            "INTERPOLATION_DEGREE" => seg.interpolation_degree = value.parse().ok(),
            "QUATERNION_TYPE" => seg.quaternion_type = Some(value.to_string()),
            "EULER_ROT_SEQ" => seg.euler_rot_seq = Some(value.to_string()),
            _ => {}
        }
    }
    if let Some(done) = current.take() {
        if done.records.is_empty() {
            return None;
        }
        segments.push(done);
    }
    if version.is_empty() || !saw_data {
        return None;
    }
    Some(segments)
}

pub fn records(segments: &[AemSegment]) -> usize {
    segments.iter().map(|s| s.records.len()).sum()
}

pub fn worst_norm_deviation(segments: &[AemSegment]) -> Option<(f64, String)> {
    let mut worst: Option<(f64, String)> = None;
    for segment in segments {
        for record in &segment.records {
            let Some(q) = record.quaternion_qc_last() else {
                continue;
            };
            let norm = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
            let dev = (norm - 1.0).abs();
            if worst.as_ref().is_none_or(|(d, _)| dev > *d) {
                worst = Some((dev, record.time.clone()));
            }
        }
    }
    worst
}

pub fn euler_angle_deg(a: &[f64; 4], b: &[f64; 4]) -> f64 {
    let dot = (a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]).abs();
    let clamped = dot.min(1.0);
    2.0 * clamped.acos().to_degrees()
}

pub fn body_to_reference(q: [f64; 4]) -> [[f64; 3]; 3] {
    let [x, y, z, w] = q;
    [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - z * w),
            2.0 * (x * z + y * w),
        ],
        [
            2.0 * (x * y + z * w),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - x * w),
        ],
        [
            2.0 * (x * z - y * w),
            2.0 * (y * z + x * w),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ]
}

pub fn to_body_frame(q: [f64; 4], reference_vector: [f64; 3]) -> [f64; 3] {
    let r = body_to_reference(q);
    [
        r[0][0] * reference_vector[0]
            + r[1][0] * reference_vector[1]
            + r[2][0] * reference_vector[2],
        r[0][1] * reference_vector[0]
            + r[1][1] * reference_vector[1]
            + r[2][1] * reference_vector[2],
        r[0][2] * reference_vector[0]
            + r[1][2] * reference_vector[1]
            + r[2][2] * reference_vector[2],
    ]
}

fn lagrange_at(nodes: &[(f64, [f64; 4])], seconds: f64) -> Option<[f64; 4]> {
    if nodes.is_empty() {
        return None;
    }
    let mut out = [0.0; 4];
    for (index, (t_i, q_i)) in nodes.iter().enumerate() {
        let mut weight = 1.0;
        for (other, (t_j, _)) in nodes.iter().enumerate() {
            if other == index {
                continue;
            }
            let denominator = t_i - t_j;
            if denominator == 0.0 {
                return None;
            }
            weight *= (seconds - t_j) / denominator;
        }
        for component in 0..4 {
            out[component] += weight * q_i[component];
        }
    }
    let norm = (out[0] * out[0] + out[1] * out[1] + out[2] * out[2] + out[3] * out[3]).sqrt();
    if norm <= 0.0 {
        return None;
    }
    Some([out[0] / norm, out[1] / norm, out[2] / norm, out[3] / norm])
}

pub fn quaternion_at(
    segments: &[AemSegment],
    seconds: f64,
    lsk: &crate::archivar::LeapSeconds,
    degree: usize,
) -> Option<[f64; 4]> {
    let mut nodes: Vec<(f64, [f64; 4])> = Vec::new();
    for segment in segments {
        for record in &segment.records {
            let Some(t) = crate::archivar::parse_iso_tdb(&record.time, lsk) else {
                continue;
            };
            let Some(q) = record.quaternion_qc_last() else {
                continue;
            };
            nodes.push((t, q));
        }
    }
    nodes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    nodes.dedup_by(|a, b| a.0 == b.0);
    for index in 1..nodes.len() {
        let previous = nodes[index - 1].1;
        let current = nodes[index].1;
        let dot = previous[0] * current[0]
            + previous[1] * current[1]
            + previous[2] * current[2]
            + previous[3] * current[3];
        if dot < 0.0 {
            nodes[index].1 = [-current[0], -current[1], -current[2], -current[3]];
        }
    }
    if nodes.len() < 2 {
        return None;
    }
    let mut pivot = 0;
    let mut best = f64::INFINITY;
    for (index, (t, _)) in nodes.iter().enumerate() {
        let distance = (t - seconds).abs();
        if distance < best {
            best = distance;
            pivot = index;
        }
    }
    let half = degree / 2;
    let start = pivot
        .saturating_sub(half)
        .min(nodes.len().saturating_sub(degree + 1));
    let end = (start + degree + 1).min(nodes.len());
    lagrange_at(&nodes[start..end], seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "CCSDS_AEM_VERS = 2.0
COMMENT a note
ORIGINATOR = ESOC

META_START
OBJECT_NAME = ROSETTA
OBJECT_ID = 226
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_0
TIME_SYSTEM = TDB
START_TIME = 2005-02-17T14:01:01.25638692
STOP_TIME = 2005-02-17T14:12:25.73180234
ATTITUDE_TYPE = QUATERNION
INTERPOLATION_METHOD = LAGRANGE
INTERPOLATION_DEGREE = 9
META_STOP

DATA_START
2005-02-17T14:01:01.25638692 1.1415531389562372e-01 -3.9764970487758200e-01 2.5822009217893738e-01 -8.7302099660618959e-01
2005-02-17T14:12:25.73180234 1.1415531389562372e-01 -3.9764970487758200e-01 2.5822009217893738e-01 -8.7302099660618959e-01
DATA_STOP
";

    #[test]
    fn an_aem_segment_carries_frames_and_quaternions() {
        let segments = parse_aem(SAMPLE).expect("segments");
        assert_eq!(segments.len(), 1);
        let seg = &segments[0];
        assert_eq!(seg.ref_frame_a, "EME2000");
        assert_eq!(seg.ref_frame_b, "SC_BODY_0");
        assert_eq!(seg.attitude_type, "QUATERNION");
        assert_eq!(seg.interpolation_degree, Some(9));
        assert_eq!(seg.records.len(), 2);
        let q = seg.records[0].quaternion_qc_last().expect("quaternion");
        assert_eq!(q[3], -8.7302099660618959e-1);
    }

    #[test]
    fn a_unit_quaternion_has_no_deviation() {
        let segments = parse_aem(SAMPLE).expect("segments");
        let (dev, _) = worst_norm_deviation(&segments).expect("a deviation");
        assert!(dev < 1e-12, "norm deviation {dev}");
    }

    #[test]
    fn an_aem_without_data_is_absent() {
        let text = SAMPLE.replace("DATA_START", "").replace("DATA_STOP", "");
        assert!(parse_aem(&text).is_none());
    }

    #[test]
    fn a_record_with_the_wrong_arity_is_skipped() {
        let text = SAMPLE.replace(
            "2005-02-17T14:12:25.73180234 1.1415531389562372e-01 -3.9764970487758200e-01 2.5822009217893738e-01 -8.7302099660618959e-01",
            "2005-02-17T14:12:25.73180234 1.0 0.0",
        );
        let segments = parse_aem(&text).expect("segments");
        assert_eq!(segments[0].records.len(), 1);
    }

    #[test]
    fn the_euler_angle_between_identical_attitudes_is_zero() {
        let a = [0.1, -0.4, 0.25, -0.87];
        assert!(euler_angle_deg(&a, &a) < 1e-9);
    }
}
