#[derive(Clone, Debug, PartialEq)]
pub struct OemSegment {
    pub object_name: String,
    pub object_id: String,
    pub center_name: String,
    pub ref_frame: String,
    pub time_system: String,
    pub start_time: String,
    pub stop_time: String,
    pub interpolation: String,
    pub interpolation_degree: Option<u32>,
    pub records: Vec<OemRecord>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OemRecord {
    pub time: String,
    pub position_km: [f64; 3],
    pub velocity_km_s: [f64; 3],
    pub acceleration_km_s2: Option<[f64; 3]>,
}

fn key_value(line: &str) -> Option<(&str, &str)> {
    let (k, v) = line.split_once('=')?;
    Some((k.trim(), v.trim()))
}

fn parse_record(line: &str) -> Option<OemRecord> {
    let mut it = line.split_whitespace();
    let time = it.next()?.to_string();
    if !time.contains('T') {
        return None;
    }
    let mut numbers: Vec<f64> = Vec::new();
    for token in it {
        numbers.push(token.parse().ok()?);
    }
    if numbers.len() < 6 {
        return None;
    }
    let mut acceleration = None;
    if numbers.len() >= 9 {
        acceleration = Some([numbers[6], numbers[7], numbers[8]]);
    }
    Some(OemRecord {
        time,
        position_km: [numbers[0], numbers[1], numbers[2]],
        velocity_km_s: [numbers[3], numbers[4], numbers[5]],
        acceleration_km_s2: acceleration,
    })
}

pub fn parse_oem(text: &str) -> Option<Vec<OemSegment>> {
    let mut segments: Vec<OemSegment> = Vec::new();
    let mut version = String::new();
    let mut current: Option<OemSegment> = None;
    let mut in_meta = false;
    let mut saw_data = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("COMMENT") {
            continue;
        }
        if line == "META_START" {
            in_meta = true;
            if let Some(done) = current.take() {
                if done.records.is_empty() {
                    return None;
                }
                segments.push(done);
            }
            current = Some(OemSegment {
                object_name: String::new(),
                object_id: String::new(),
                center_name: String::new(),
                ref_frame: String::new(),
                time_system: String::new(),
                start_time: String::new(),
                stop_time: String::new(),
                interpolation: String::new(),
                interpolation_degree: None,
                records: Vec::new(),
            });
            continue;
        }
        if line == "META_STOP" {
            in_meta = false;
            continue;
        }
        let Some((key, value)) = key_value(line) else {
            if current.is_some() && !in_meta {
                if let Some(record) = parse_record(line)
                    && let Some(seg) = current.as_mut()
                {
                    saw_data = true;
                    seg.records.push(record);
                }
            }
            continue;
        };
        if !in_meta && version.is_empty() && key == "CCSDS_OEM_VERS" {
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
            "CENTER_NAME" => seg.center_name = value.to_string(),
            "REF_FRAME" => seg.ref_frame = value.to_string(),
            "TIME_SYSTEM" => seg.time_system = value.to_string(),
            "START_TIME" => seg.start_time = value.to_string(),
            "STOP_TIME" => seg.stop_time = value.to_string(),
            "INTERPOLATION" => seg.interpolation = value.to_string(),
            "INTERPOLATION_DEGREE" => {
                seg.interpolation_degree = value.parse().ok();
            }
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

pub fn records(segments: &[OemSegment]) -> usize {
    segments.iter().map(|s| s.records.len()).sum()
}

pub fn span(segments: &[OemSegment]) -> Option<(String, String)> {
    let mut first: Option<&str> = None;
    let mut last: Option<&str> = None;
    for seg in segments {
        for record in &seg.records {
            if first.is_none_or(|f| record.time.as_str() < f) {
                first = Some(record.time.as_str());
            }
            if last.is_none_or(|l| record.time.as_str() > l) {
                last = Some(record.time.as_str());
            }
        }
    }
    Some((first?.to_string(), last?.to_string()))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OemPoint {
    pub seconds: f64,
    pub position_km: [f64; 3],
    pub velocity_km_s: [f64; 3],
    pub acceleration_km_s2: Option<[f64; 3]>,
}

pub fn positions_in_center(segments: &[OemSegment], center: &str) -> Vec<OemPoint> {
    let mut out = Vec::new();
    for segment in segments {
        if segment.center_name != center {
            continue;
        }
        for record in &segment.records {
            let Some(seconds) = crate::archivar::parse_iso_tdb_label(&record.time) else {
                continue;
            };
            out.push(OemPoint {
                seconds,
                position_km: record.position_km,
                velocity_km_s: record.velocity_km_s,
                acceleration_km_s2: record.acceleration_km_s2,
            });
        }
    }
    out.sort_by(|a, b| {
        a.seconds
            .partial_cmp(&b.seconds)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    out.dedup_by(|a, b| a.seconds == b.seconds);
    out
}

pub fn position_at(points: &[OemPoint], seconds: f64) -> Option<[f64; 3]> {
    let first = points.first()?;
    let last = points.last()?;
    if seconds < first.seconds || seconds > last.seconds {
        return None;
    }
    let mut low = 0usize;
    let mut high = points.len() - 1;
    while high - low > 1 {
        let middle = (low + high) / 2;
        if points[middle].seconds <= seconds {
            low = middle;
        } else {
            high = middle;
        }
    }
    let a = points[low];
    let b = points[high];
    let span = b.seconds - a.seconds;
    if span <= 0.0 {
        return Some(a.position_km);
    }
    let s = (seconds - a.seconds) / span;
    let s2 = s * s;
    let s3 = s2 * s;
    let s4 = s3 * s;
    let s5 = s4 * s;
    let mut out = [0.0; 3];
    match (a.acceleration_km_s2, b.acceleration_km_s2) {
        (Some(a_acc), Some(b_acc)) => {
            let h00 = 1.0 - 10.0 * s3 + 15.0 * s4 - 6.0 * s5;
            let h10 = s - 6.0 * s3 + 8.0 * s4 - 3.0 * s5;
            let h20 = 0.5 * s2 - 1.5 * s3 + 1.5 * s4 - 0.5 * s5;
            let h01 = 10.0 * s3 - 15.0 * s4 + 6.0 * s5;
            let h11 = -4.0 * s3 + 7.0 * s4 - 3.0 * s5;
            let h21 = 0.5 * s3 - s4 + 0.5 * s5;
            for axis in 0..3 {
                out[axis] = a.position_km[axis] * h00
                    + a.velocity_km_s[axis] * span * h10
                    + a_acc[axis] * span * span * h20
                    + b.position_km[axis] * h01
                    + b.velocity_km_s[axis] * span * h11
                    + b_acc[axis] * span * span * h21;
            }
        }
        _ => {
            let h00 = 2.0 * s3 - 3.0 * s2 + 1.0;
            let h10 = s3 - 2.0 * s2 + s;
            let h01 = -2.0 * s3 + 3.0 * s2;
            let h11 = s3 - s2;
            for axis in 0..3 {
                out[axis] = a.position_km[axis] * h00
                    + a.velocity_km_s[axis] * span * h10
                    + b.position_km[axis] * h01
                    + b.velocity_km_s[axis] * span * h11;
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "CCSDS_OEM_VERS = 2.0
COMMENT a note
ORIGINATOR = ESOC

META_START
OBJECT_NAME = ROSETTA
OBJECT_ID = 226
CENTER_NAME = SOLAR SYSTEM BARYCENTER
REF_FRAME = ICRF
TIME_SYSTEM = TDB
START_TIME = 2005-02-17T14:02:25.73180234
STOP_TIME = 2005-02-17T14:12:25.73180234
INTERPOLATION = HERMITE
INTERPOLATION_DEGREE = 11
META_STOP

2005-02-17T14:02:25.73180234  -1.312072714458328e+08  7.120124345736103e+07  3.054026550260468e+07 -1.184908073808264e+01 -2.448021630411433e+01 -1.038422565374082e+01  4.906157272981018e-06 -2.644884334201375e-06 -1.134295439082151e-06
2005-02-17T14:03:25.73180234  -1.312079823818460e+08  7.119977463962205e+07  3.053964244702375e+07 -1.184878636725333e+01 -2.448037499585140e+01 -1.038429371090523e+01
";

    #[test]
    fn a_constant_acceleration_is_reproduced_by_the_quintic() {
        let a = [1.0e-3, -2.0e-3, 3.0e-3];
        let p0 = [8331.0, -120.0, 44.0];
        let v0 = [0.3, 10.5, -0.2];
        let make = |t: f64| {
            let mut p = [0.0; 3];
            let mut v = [0.0; 3];
            for k in 0..3 {
                p[k] = p0[k] + v0[k] * t + 0.5 * a[k] * t * t;
                v[k] = v0[k] + a[k] * t;
            }
            (p, v)
        };
        let (p1, v1) = make(60.0);
        let points = [
            OemPoint {
                seconds: 0.0,
                position_km: p0,
                velocity_km_s: v0,
                acceleration_km_s2: Some(a),
            },
            OemPoint {
                seconds: 60.0,
                position_km: p1,
                velocity_km_s: v1,
                acceleration_km_s2: Some(a),
            },
        ];
        let (expect, _) = make(23.0);
        let got = position_at(&points, 23.0).expect("a position");
        for k in 0..3 {
            assert!(
                (got[k] - expect[k]).abs() < 1.0e-9,
                "axis {k}: {got:?} against {expect:?}"
            );
        }
    }

    #[test]
    fn the_quintic_stays_below_a_millimetre_where_a_straight_line_loses_metres() {
        let a = [4.9e-3, -4.9e-3, 0.0];
        let p0 = [0.0, 0.0, 0.0];
        let v0 = [10.5, 0.0, 0.0];
        let make = |t: f64| {
            let mut p = [0.0; 3];
            let mut v = [0.0; 3];
            for k in 0..3 {
                p[k] = p0[k] + v0[k] * t + 0.5 * a[k] * t * t;
                v[k] = v0[k] + a[k] * t;
            }
            (p, v)
        };
        let (p1, v1) = make(8.6);
        let points = [
            OemPoint {
                seconds: 0.0,
                position_km: p0,
                velocity_km_s: v0,
                acceleration_km_s2: Some(a),
            },
            OemPoint {
                seconds: 8.6,
                position_km: p1,
                velocity_km_s: v1,
                acceleration_km_s2: Some(a),
            },
        ];
        let (expect, _) = make(4.3);
        let got = position_at(&points, 4.3).expect("a position");
        let straight = [
            0.5 * (p0[0] + p1[0]),
            0.5 * (p0[1] + p1[1]),
            0.5 * (p0[2] + p1[2]),
        ];
        let straight_error = (straight[0] - expect[0]).abs() + (straight[1] - expect[1]).abs();
        let hermit_error = (got[0] - expect[0]).abs() + (got[1] - expect[1]).abs();
        assert!(straight_error > 1.0e-3);
        assert!(hermit_error < 1.0e-9);
    }

    #[test]
    fn without_acceleration_the_cubic_still_beats_the_straight_line() {
        let p0 = [0.0, 0.0, 0.0];
        let p1 = [100.0, 40.0, -20.0];
        let v0 = [10.0, 1.0, 0.0];
        let v1 = [11.0, 3.0, -1.0];
        let points = [
            OemPoint {
                seconds: 0.0,
                position_km: p0,
                velocity_km_s: v0,
                acceleration_km_s2: None,
            },
            OemPoint {
                seconds: 10.0,
                position_km: p1,
                velocity_km_s: v1,
                acceleration_km_s2: None,
            },
        ];
        let got = position_at(&points, 5.0).expect("a position");
        assert!((got[0] - 52.5).abs() < 1.0e-9);
        assert!((got[1] - 20.0).abs() < 1.0e-9);
        assert!(position_at(&points, 11.0).is_none());
    }

    #[test]
    fn an_oem_segment_carries_its_meta_and_records() {
        let segments = parse_oem(SAMPLE).expect("segments");
        assert_eq!(segments.len(), 1);
        let seg = &segments[0];
        assert_eq!(seg.object_name, "ROSETTA");
        assert_eq!(seg.center_name, "SOLAR SYSTEM BARYCENTER");
        assert_eq!(seg.ref_frame, "ICRF");
        assert_eq!(seg.time_system, "TDB");
        assert_eq!(seg.interpolation_degree, Some(11));
        assert_eq!(seg.records.len(), 2);
        assert_eq!(seg.records[0].position_km[0], -1.312072714458328e8);
        assert!(seg.records[0].acceleration_km_s2.is_some());
        assert!(seg.records[1].acceleration_km_s2.is_none());
    }

    #[test]
    fn an_oem_without_meta_is_absent() {
        let text = SAMPLE.replace("META_START", "").replace("META_STOP", "");
        assert!(parse_oem(&text).is_none());
    }

    #[test]
    fn a_broken_record_is_skipped_not_zeroed() {
        let text = SAMPLE.replace(
            "2005-02-17T14:03:25.73180234",
            "2005-02-17T14:03:25.73180234",
        ) + "2005-02-17T14:04:25.73180234  1.0 2.0\n";
        let segments = parse_oem(&text).expect("segments");
        assert_eq!(segments[0].records.len(), 2);
    }

    #[test]
    fn span_reads_first_and_last_record() {
        let segments = parse_oem(SAMPLE).expect("segments");
        let (first, last) = span(&segments).expect("span");
        assert_eq!(first, "2005-02-17T14:02:25.73180234");
        assert_eq!(last, "2005-02-17T14:03:25.73180234");
    }
}
