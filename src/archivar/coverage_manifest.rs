use super::fetch::PresenceSample;
use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ShardPlacement {
    Placed([f64; 6]),
    Pending,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ShardDescriptor {
    pub url: String,
    pub placement: ShardPlacement,
    pub crs: Option<i32>,
    pub epoch: Option<f64>,
}

fn parse_crs(token: Option<&&str>) -> Option<i32> {
    let text = token?.trim();
    let digits = text
        .strip_prefix("EPSG:")
        .or_else(|| text.strip_prefix("epsg:"))
        .unwrap_or(text);
    if digits.eq_ignore_ascii_case("pending") || digits == "-" {
        return None;
    }
    digits.parse::<i32>().ok().filter(|c| *c > 0)
}

fn parse_bbox(tokens: &[&str]) -> ShardPlacement {
    let mut bbox = [0.0_f64; 6];
    for (i, slot) in bbox.iter_mut().enumerate() {
        let Some(token) = tokens.get(i) else {
            return ShardPlacement::Pending;
        };
        match token.parse::<f64>() {
            Ok(v) if v.is_finite() => *slot = v,
            _ => return ShardPlacement::Pending,
        }
    }
    ShardPlacement::Placed(bbox)
}

pub fn parse_coverage_manifest(text: &str) -> Vec<ShardDescriptor> {
    let mut shards = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens.first().copied() {
            Some("shard") => {
                let Some(url) = tokens.get(1) else { continue };
                let placement = parse_bbox(&tokens[2..]);
                let (crs, epoch) = match placement {
                    ShardPlacement::Placed(_) => (
                        parse_crs(tokens.get(8)),
                        tokens
                            .get(9)
                            .and_then(|t| t.parse::<f64>().ok())
                            .filter(|v| v.is_finite()),
                    ),
                    ShardPlacement::Pending => (None, None),
                };
                shards.push(ShardDescriptor {
                    url: (*url).to_string(),
                    placement,
                    crs,
                    epoch,
                });
            }
            Some("node") => {
                let Some(prefix) = tokens.get(1) else {
                    continue;
                };
                let placement = parse_bbox(&tokens[2..]);
                let (crs, epoch) = match placement {
                    ShardPlacement::Placed(_) => (
                        parse_crs(tokens.get(8)),
                        tokens
                            .get(9)
                            .and_then(|t| t.parse::<f64>().ok())
                            .filter(|v| v.is_finite()),
                    ),
                    ShardPlacement::Pending => (None, None),
                };
                shards.push(ShardDescriptor {
                    url: (*prefix).to_string(),
                    placement,
                    crs,
                    epoch,
                });
            }
            _ => {}
        }
    }
    shards
}

fn nearest_point_on_bbox(bbox: &[f64; 6], p: (f64, f64, f64)) -> [f64; 3] {
    let axis = |c: f64, a: f64, b: f64| c.max(a.min(b)).min(a.max(b));
    [
        axis(p.0, bbox[0], bbox[3]),
        axis(p.1, bbox[1], bbox[4]),
        axis(p.2, bbox[2], bbox[5]),
    ]
}

pub fn shard_intersects_hull(
    placement: &ShardPlacement,
    presences: &[PresenceSample],
    field: &EnclosureField<'_>,
    env: &AnchorEnvelope,
) -> bool {
    let bbox = match placement {
        ShardPlacement::Placed(bbox) => bbox,
        ShardPlacement::Pending => return true,
    };
    presences.iter().any(|sample| {
        let (_, px, py, pz, ..) = *sample;
        let point = nearest_point_on_bbox(bbox, (px, py, pz));
        record_in_enclosure(
            std::slice::from_ref(sample),
            Some(point),
            0.0,
            0.0,
            EnclosureField {
                config: field.config,
                medium: field.medium,
                body_radius: field.body_radius,
                receiver_aperture: field.receiver_aperture,
            },
            AnchorEnvelope {
                vmax: env.vmax,
                amax: env.amax,
                pad: env.pad,
                ttl: env.ttl,
            },
        )
    })
}

pub fn resolve_placement(
    placement: &ShardPlacement,
    crs: Option<i32>,
    epoch: Option<f64>,
    body_name: &str,
    tdb: f64,
    eph: &std::collections::HashMap<String, BodyEphemeris>,
) -> ShardPlacement {
    const EARTH_A: f64 = 6_378_137.0;
    const EARTH_F_INV: f64 = 298.257_223_563;
    let bbox = match placement {
        ShardPlacement::Placed(b) => b,
        ShardPlacement::Pending => return ShardPlacement::Pending,
    };
    let Some(code) = crs else {
        return ShardPlacement::Pending;
    };
    let (lon, lat, alt, d_east, d_north, dz) = if code == 4326 {
        let lon = 0.5 * (bbox[0] + bbox[3]);
        let lat = 0.5 * (bbox[1] + bbox[4]);
        let m_per_deg_lat = 111_320.0;
        let m_per_deg_lon = (m_per_deg_lat * lat.to_radians().cos()).abs().max(1.0);
        (
            lon,
            lat,
            0.5 * (bbox[2] + bbox[5]),
            0.5 * (bbox[3] - bbox[0]).abs() * m_per_deg_lon,
            0.5 * (bbox[4] - bbox[1]).abs() * m_per_deg_lat,
            0.5 * (bbox[5] - bbox[2]).abs(),
        )
    } else {
        let e_center = 0.5 * (bbox[0] + bbox[3]);
        let n_center = 0.5 * (bbox[1] + bbox[4]);
        let alt = 0.5 * (bbox[2] + bbox[5]);
        let Some((lon, lat, alt)) = super::projection::to_geodetic(
            &super::las::LasCrs::Epsg(code as u16),
            e_center,
            n_center,
            alt,
            EARTH_A,
            EARTH_F_INV,
        ) else {
            return ShardPlacement::Pending;
        };
        (
            lon,
            lat,
            alt,
            0.5 * (bbox[3] - bbox[0]).abs(),
            0.5 * (bbox[4] - bbox[1]).abs(),
            0.5 * (bbox[5] - bbox[2]).abs(),
        )
    };
    let Some(center) =
        super::motion::body_fixed_to_icrs(body_name, lat, lon, alt, epoch.unwrap_or(tdb), eph)
    else {
        return ShardPlacement::Pending;
    };
    let pad = (d_east * d_east + d_north * d_north + dz * dz).sqrt() + 1.0;
    ShardPlacement::Placed([
        center[0] - pad,
        center[1] - pad,
        center[2] - pad,
        center[0] + pad,
        center[1] + pad,
        center[2] + pad,
    ])
}

#[test]
fn coverage_manifest_parse_and_hull() {
    let text = "\
# coverage manifest fixture
shard https://cdn.example/shard-a.las 1 1 1 2 2 2 EPSG:4326
shard https://cdn.example/shard-b.las 10 10 10 20 20
node https://cdn.example/tile- 0 0 0 5 5 5
";
    let shards = parse_coverage_manifest(text);
    assert_eq!(shards.len(), 3);
    assert_eq!(shards[0].url, "https://cdn.example/shard-a.las");
    assert_eq!(
        shards[0].placement,
        ShardPlacement::Placed([1.0, 1.0, 1.0, 2.0, 2.0, 2.0])
    );
    assert_eq!(shards[0].crs, Some(4326));
    assert_eq!(shards[1].url, "https://cdn.example/shard-b.las");
    assert_eq!(shards[1].placement, ShardPlacement::Pending);
    assert_eq!(shards[2].url, "https://cdn.example/tile-");
    assert_eq!(
        shards[2].placement,
        ShardPlacement::Placed([0.0, 0.0, 0.0, 5.0, 5.0, 5.0])
    );

    let no_eph = std::collections::HashMap::new();
    assert_eq!(
        resolve_placement(
            &ShardPlacement::Placed([1.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            Some(4326),
            None,
            "Earth",
            0.0,
            &no_eph
        ),
        ShardPlacement::Pending
    );
    assert_eq!(
        resolve_placement(
            &ShardPlacement::Placed([1.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            None,
            None,
            "Earth",
            0.0,
            &no_eph
        ),
        ShardPlacement::Pending
    );

    let presences: [PresenceSample; 1] = [(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0)];
    let fc = FieldConfig {
        key: String::new(),
        name: String::new(),
        band_id: None,
        kernel: 0,
        channel_ref: 0,
        tau: 0.0,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
        aperture: Aperture::None,
    };
    let field = EnclosureField {
        config: &fc,
        medium: None,
        body_radius: None,
        receiver_aperture: None,
    };
    let env = AnchorEnvelope {
        vmax: 0.0,
        amax: 0.0,
        pad: 2.0,
        ttl: 60.0,
    };

    assert!(shard_intersects_hull(
        &ShardPlacement::Placed([1.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
        &presences,
        &field,
        &env
    ));
    assert!(!shard_intersects_hull(
        &ShardPlacement::Placed([100.0, 100.0, 100.0, 101.0, 101.0, 101.0]),
        &presences,
        &field,
        &env
    ));
    assert!(shard_intersects_hull(
        &ShardPlacement::Pending,
        &presences,
        &field,
        &env
    ));
}
