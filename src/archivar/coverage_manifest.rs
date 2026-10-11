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
                shards.push(ShardDescriptor {
                    url: (*url).to_string(),
                    placement: parse_bbox(&tokens[2..]),
                });
            }
            Some("node") => {
                let Some(prefix) = tokens.get(1) else {
                    continue;
                };
                shards.push(ShardDescriptor {
                    url: (*prefix).to_string(),
                    placement: parse_bbox(&tokens[2..]),
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

#[test]
fn coverage_manifest_parse_and_hull() {
    let text = "\
# coverage manifest fixture
shard https://cdn.example/shard-a.las 1 1 1 2 2 2
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
    assert_eq!(shards[1].url, "https://cdn.example/shard-b.las");
    assert_eq!(shards[1].placement, ShardPlacement::Pending);
    assert_eq!(shards[2].url, "https://cdn.example/tile-");
    assert_eq!(
        shards[2].placement,
        ShardPlacement::Placed([0.0, 0.0, 0.0, 5.0, 5.0, 5.0])
    );

    let presences: [PresenceSample; 1] = [(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0)];
    let fc = FieldConfig {
        key: String::new(),
        name: String::new(),
        band_id: None,
        kernel: 0,
        force: 0,
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
