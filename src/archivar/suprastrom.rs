pub const SRD62_MAGIC: [u8; 2] = [0xCF, 0x86];
pub const SRD62_VERSION: u8 = 0x07;

pub const MU0: f64 = 1.25663706212e-6;

#[derive(Clone, Debug)]
pub struct SuprastromPoint {
    pub t_k: f64,
    pub lambda_m: f64,
}

#[derive(Clone, Debug)]
pub struct SuprastromSeries {
    pub id: String,
    pub label: String,
    pub points: Vec<SuprastromPoint>,
}

#[derive(Clone, Debug)]
pub struct SuprastromBin {
    pub series: Vec<SuprastromSeries>,
}

fn read_u8(bytes: &[u8], pos: &mut usize) -> Option<u8> {
    let v = *bytes.get(*pos)?;
    *pos += 1;
    Some(v)
}

fn read_str(bytes: &[u8], pos: &mut usize) -> Option<String> {
    let len = read_u8(bytes, pos)? as usize;
    let s = std::str::from_utf8(bytes.get(*pos..*pos + len)?).ok()?;
    *pos += len;
    Some(s.to_string())
}

fn read_u32(bytes: &[u8], pos: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(bytes.get(*pos..*pos + 4)?.try_into().ok()?);
    *pos += 4;
    Some(v)
}

fn read_f64(bytes: &[u8], pos: &mut usize) -> Option<f64> {
    let v = f64::from_le_bytes(bytes.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    Some(v)
}

pub fn parse_suprastrom_bin(bytes: &[u8]) -> Option<SuprastromBin> {
    if bytes.len() < 8
        || bytes[0] != SRD62_MAGIC[0]
        || bytes[1] != SRD62_MAGIC[1]
        || bytes[2] != SRD62_VERSION
    {
        return None;
    }
    let mut pos = 3usize;
    let n_series = read_u32(bytes, &mut pos)? as usize;
    let mut series: Vec<SuprastromSeries> = Vec::with_capacity(n_series);
    for _ in 0..n_series {
        let id = read_str(bytes, &mut pos)?;
        let label = read_str(bytes, &mut pos)?;
        let n_points = read_u32(bytes, &mut pos)? as usize;
        let mut points = Vec::with_capacity(n_points);
        for _ in 0..n_points {
            let t_k = read_f64(bytes, &mut pos)?;
            let lambda_m = read_f64(bytes, &mut pos)?;
            points.push(SuprastromPoint { t_k, lambda_m });
        }
        series.push(SuprastromSeries { id, label, points });
    }
    Some(SuprastromBin { series })
}

fn push_str(out: &mut Vec<u8>, s: &str) {
    let b = s.as_bytes();
    out.push(b.len() as u8);
    out.extend_from_slice(b);
}

pub fn encode_suprastrom_bin(bin: &SuprastromBin) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&SRD62_MAGIC);
    out.push(SRD62_VERSION);
    out.extend_from_slice(&(bin.series.len() as u32).to_le_bytes());
    for s in &bin.series {
        push_str(&mut out, &s.id);
        push_str(&mut out, &s.label);
        out.extend_from_slice(&(s.points.len() as u32).to_le_bytes());
        for p in &s.points {
            out.extend_from_slice(&p.t_k.to_le_bytes());
            out.extend_from_slice(&p.lambda_m.to_le_bytes());
        }
    }
    out
}

pub fn lambda_inv2_m2(lambda_m: f64) -> Option<f64> {
    let v = lambda_m;
    if v.is_finite() && v > 0.0 {
        let inv2 = 1.0 / (v * v);
        if inv2.is_finite() {
            Some(inv2)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suprastrom_bin_round_trip_preserves_series() {
        let bin = SuprastromBin {
            series: vec![
                SuprastromSeries {
                    id: "A00316".to_string(),
                    label: "//ab-plane".to_string(),
                    points: vec![
                        SuprastromPoint {
                            t_k: 7.0,
                            lambda_m: 1.4e-7,
                        },
                        SuprastromPoint {
                            t_k: 40.0,
                            lambda_m: 1.6e-7,
                        },
                    ],
                },
                SuprastromSeries {
                    id: "A00316".to_string(),
                    label: "//c-axis".to_string(),
                    points: vec![SuprastromPoint {
                        t_k: 8.0,
                        lambda_m: 1.04e-6,
                    }],
                },
            ],
        };
        let bytes = encode_suprastrom_bin(&bin);
        assert_eq!(bytes[0], SRD62_MAGIC[0]);
        assert_eq!(bytes[1], SRD62_MAGIC[1]);
        assert_eq!(bytes[2], SRD62_VERSION);
        let back = parse_suprastrom_bin(&bytes).unwrap();
        assert_eq!(back.series.len(), 2);
        assert_eq!(back.series[0].id, "A00316");
        assert_eq!(back.series[0].label, "//ab-plane");
        assert_eq!(back.series[0].points.len(), 2);
        assert_eq!(back.series[1].label, "//c-axis");
        assert_eq!(back.series[1].points.len(), 1);
        assert!((back.series[0].points[1].lambda_m - 1.6e-7).abs() < 1e-18);
    }

    #[test]
    fn suprastrom_parse_rejects_bad_magic() {
        assert!(parse_suprastrom_bin(&[0, 0, SRD62_VERSION, 0, 0, 0, 0]).is_none());
    }

    #[test]
    fn suprastrom_lambda_inv2_rejects_non_physical() {
        assert!(lambda_inv2_m2(0.0).is_none());
        assert!(lambda_inv2_m2(-1.0).is_none());
        assert!(lambda_inv2_m2(f64::NAN).is_none());
        let inv2 = lambda_inv2_m2(1.0e-6).unwrap();
        assert!((inv2 - 1.0e12).abs() < 1.0);
    }
}
