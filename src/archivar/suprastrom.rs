pub const SRD62_MAGIC: [u8; 2] = [0xCF, 0x86];
pub const SRD62_VERSION: u8 = 0x06;

pub const MU0: f64 = 1.25663706212e-6;

#[derive(Clone, Debug)]
pub struct SuprastromPoint {
    pub t_k: f64,
    pub lambda_m: f64,
}

#[derive(Clone, Debug)]
pub struct SuprastromCitation {
    pub id: String,
    pub points: Vec<SuprastromPoint>,
}

#[derive(Clone, Debug)]
pub struct SuprastromBin {
    pub citations: Vec<SuprastromCitation>,
}

pub fn parse_suprastrom_bin(bytes: &[u8]) -> Option<SuprastromBin> {
    if bytes.len() < 8
        || bytes[0] != SRD62_MAGIC[0]
        || bytes[1] != SRD62_MAGIC[1]
        || bytes[2] != SRD62_VERSION
    {
        return None;
    }
    let n_cit = u32::from_le_bytes(bytes[3..7].try_into().ok()?) as usize;
    let mut pos = 7usize;
    let mut citations: Vec<SuprastromCitation> = Vec::with_capacity(n_cit);
    for _ in 0..n_cit {
        if pos >= bytes.len() {
            return None;
        }
        let id_len = bytes[pos] as usize;
        pos += 1;
        if pos + id_len > bytes.len() {
            return None;
        }
        let id = std::str::from_utf8(&bytes[pos..pos + id_len])
            .ok()?
            .to_string();
        pos += id_len;
        citations.push(SuprastromCitation {
            id,
            points: Vec::new(),
        });
    }
    if pos + 4 > bytes.len() {
        return None;
    }
    let n_points = u32::from_le_bytes(bytes[pos..pos + 4].try_into().ok()?) as usize;
    pos += 4;
    let mut points: Vec<(u32, SuprastromPoint)> = Vec::with_capacity(n_points);
    for _ in 0..n_points {
        if pos + 4 + 16 > bytes.len() {
            return None;
        }
        let cit_index = u32::from_le_bytes(bytes[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let t_k = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
        pos += 8;
        let lambda_m = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
        pos += 8;
        points.push((cit_index, SuprastromPoint { t_k, lambda_m }));
    }
    for (cit_index, p) in points {
        if (cit_index as usize) < citations.len() {
            citations[cit_index as usize].points.push(p);
        }
    }
    Some(SuprastromBin { citations })
}

pub fn encode_suprastrom_bin(bin: &SuprastromBin) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&SRD62_MAGIC);
    out.push(SRD62_VERSION);
    out.extend_from_slice(&(bin.citations.len() as u32).to_le_bytes());
    for c in &bin.citations {
        let id = c.id.as_bytes();
        out.push(id.len() as u8);
        out.extend_from_slice(id);
    }
    let n_points: usize = bin.citations.iter().map(|c| c.points.len()).sum();
    out.extend_from_slice(&(n_points as u32).to_le_bytes());
    for (i, c) in bin.citations.iter().enumerate() {
        for p in &c.points {
            out.extend_from_slice(&(i as u32).to_le_bytes());
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
    fn suprastrom_bin_round_trip_preserves_provenance() {
        let bin = SuprastromBin {
            citations: vec![
                SuprastromCitation {
                    id: "A00039".to_string(),
                    points: vec![
                        SuprastromPoint {
                            t_k: 40.0,
                            lambda_m: 1.1e-6,
                        },
                        SuprastromPoint {
                            t_k: 77.0,
                            lambda_m: 7.2e-6,
                        },
                    ],
                },
                SuprastromCitation {
                    id: "U00037".to_string(),
                    points: vec![SuprastromPoint {
                        t_k: 20.0,
                        lambda_m: 5.0e-7,
                    }],
                },
            ],
        };
        let bytes = encode_suprastrom_bin(&bin);
        assert_eq!(bytes[0], SRD62_MAGIC[0]);
        assert_eq!(bytes[1], SRD62_MAGIC[1]);
        assert_eq!(bytes[2], SRD62_VERSION);
        let back = parse_suprastrom_bin(&bytes).unwrap();
        assert_eq!(back.citations.len(), 2);
        assert_eq!(back.citations[0].id, "A00039");
        assert_eq!(back.citations[0].points.len(), 2);
        assert_eq!(back.citations[1].id, "U00037");
        assert_eq!(back.citations[1].points.len(), 1);
        assert!((back.citations[0].points[1].lambda_m - 7.2e-6).abs() < 1e-15);
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
