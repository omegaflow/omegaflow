use std::collections::BTreeMap;

pub struct Constant {
    pub key: String,
    pub value: f64,
    pub uncertainty: Option<f64>,
    pub unit: String,
    pub source: String,
}

const TABLE: &str = include_str!("kernels/constants.tsv");

pub fn kernel() -> Option<&'static BTreeMap<String, Constant>> {
    static KERNEL: std::sync::OnceLock<Option<BTreeMap<String, Constant>>> =
        std::sync::OnceLock::new();
    KERNEL
        .get_or_init(|| {
            let mut out = BTreeMap::new();
            for line in TABLE.lines() {
                if line.starts_with('#') || line.trim().is_empty() {
                    continue;
                }
                let c: Vec<&str> = line.split('\t').collect();
                if c.len() < 5 {
                    return None;
                }
                let key = c[0].trim().to_string();
                if key == "key" {
                    continue;
                }
                let value = c[1].trim().parse::<f64>().ok()?;
                let uncertainty = match c[2].trim() {
                    "exact" => None,
                    other => Some(other.parse::<f64>().ok()?),
                };
                out.insert(
                    key,
                    Constant {
                        key: c[0].trim().to_string(),
                        value,
                        uncertainty,
                        unit: c[3].trim().to_string(),
                        source: c[4].trim().to_string(),
                    },
                );
            }
            if out.is_empty() { None } else { Some(out) }
        })
        .as_ref()
}

pub fn value(key: &str) -> Option<f64> {
    kernel()?.get(key).map(|c| c.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_constant_carries_a_unit_and_a_source() {
        let k = kernel().expect("constants kernel");
        assert!(k.len() >= 10, "{} entries", k.len());
        for (key, c) in k {
            assert_eq!(key, &c.key);
            assert!(!c.unit.is_empty(), "{key} has no unit");
            assert!(!c.source.is_empty(), "{key} has no source");
            assert!(c.value.is_finite(), "{key} is not finite");
            if let Some(u) = c.uncertainty {
                assert!(u > 0.0 && u.is_finite(), "{key} uncertainty {u}");
            }
        }
    }

    #[test]
    fn the_exact_relations_hold() {
        let k = kernel().expect("constants kernel");
        let c = k.get("c").expect("speed of light").value;
        let h = k.get("h").expect("planck").value;
        let kb = k.get("k_B").expect("boltzmann").value;
        let na = k.get("N_A").expect("avogadro").value;
        let au = k.get("AU").expect("astronomical unit").value;
        let r = kb * na;
        assert!(
            (r - 8.31446261815324).abs() < 1e-12,
            "R = k_B * N_A is {r}"
        );
        let light_time = au / c;
        assert!(
            (light_time - 499.00478383615643).abs() < 1e-9,
            "AU/c = {light_time} s"
        );
        assert!((h / c).is_finite());
    }

    #[test]
    fn the_kernel_agrees_with_the_projection_constant() {
        let a = value("a_WGS84").expect("WGS84 semi-major axis");
        assert!((a - 6_378_137.0).abs() < 1e-9, "{a}");
        let f = value("f_WGS84").expect("WGS84 flattening");
        assert!((1.0 / f - 298.257223563).abs() < 1e-9, "{}", 1.0 / f);
    }
}
