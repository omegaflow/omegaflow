pub struct PlanetState {
    pub planet: String,
    pub target: i32,
    pub source: String,
    pub epoch_jd: f64,
    pub position_km: [f64; 3],
    pub velocity_km_s: [f64; 3],
    pub pos_tol_km: f64,
    pub vel_tol_m_s: f64,
}

const TABLE: &str = include_str!("kernels/planet_positions.tsv");

fn cell(cols: &[&str], index: usize) -> Option<f64> {
    let v: f64 = cols.get(index)?.trim().parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

pub fn parse(text: &str) -> Option<Vec<PlanetState>> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 12 {
            return None;
        }
        out.push(PlanetState {
            planet: cols[0].trim().to_string(),
            target: cols[1].trim().parse().ok()?,
            source: cols[2].trim().to_string(),
            epoch_jd: cell(&cols, 3)?,
            position_km: [cell(&cols, 4)?, cell(&cols, 5)?, cell(&cols, 6)?],
            velocity_km_s: [cell(&cols, 7)?, cell(&cols, 8)?, cell(&cols, 9)?],
            pos_tol_km: cell(&cols, 10)?,
            vel_tol_m_s: cell(&cols, 11)?,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn fixture() -> Option<Vec<PlanetState>> {
    parse(TABLE)
}

pub fn position_distance_km(a: &PlanetState, b: &PlanetState) -> f64 {
    let d = [
        a.position_km[0] - b.position_km[0],
        a.position_km[1] - b.position_km[1],
        a.position_km[2] - b.position_km[2],
    ];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

pub fn velocity_distance_m_s(a: &PlanetState, b: &PlanetState) -> f64 {
    let d = [
        a.velocity_km_s[0] - b.velocity_km_s[0],
        a.velocity_km_s[1] - b.velocity_km_s[1],
        a.velocity_km_s[2] - b.velocity_km_s[2],
    ];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() * 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fixture_carries_nine_planets_from_three_ephemerides() {
        let rows = fixture().expect("planet fixture");
        assert_eq!(rows.len(), 27);
        for planet in [
            "Merkur", "Venus", "Erde", "Mars", "Jupiter", "Saturn", "Uranus", "Neptun", "Pluto",
        ] {
            let set: Vec<&PlanetState> = rows.iter().filter(|r| r.planet == planet).collect();
            assert_eq!(set.len(), 3, "{planet} has three sources");
            for expected in ["INPOP19a", "EPM2021", "Horizons DE441"] {
                assert!(
                    set.iter().any(|r| r.source == expected),
                    "{planet} is missing {expected}"
                );
            }
        }
    }

    #[test]
    fn the_three_ephemerides_agree_within_the_published_spread() {
        let rows = fixture().expect("planet fixture");
        for planet in [
            "Merkur", "Venus", "Erde", "Mars", "Jupiter", "Saturn", "Uranus", "Neptun", "Pluto",
        ] {
            let set: Vec<&PlanetState> = rows.iter().filter(|r| r.planet == planet).collect();
            for i in 0..set.len() {
                for j in i + 1..set.len() {
                    let pos = position_distance_km(set[i], set[j]);
                    let vel = velocity_distance_m_s(set[i], set[j]);
                    assert!(
                        pos <= set[i].pos_tol_km,
                        "{planet} {} vs {}: {pos} km beyond the {} km spread",
                        set[i].source,
                        set[j].source,
                        set[i].pos_tol_km
                    );
                    assert!(
                        vel <= set[i].vel_tol_m_s,
                        "{planet} {} vs {}: {vel} m/s beyond the {} m/s spread",
                        set[i].source,
                        set[j].source,
                        set[i].vel_tol_m_s
                    );
                }
            }
        }
    }

    #[test]
    fn the_three_sources_are_not_copies_of_one_another() {
        let rows = fixture().expect("planet fixture");
        for planet in ["Merkur", "Venus", "Erde", "Mars", "Jupiter", "Saturn"] {
            let set: Vec<&PlanetState> = rows.iter().filter(|r| r.planet == planet).collect();
            let spread = set
                .iter()
                .map(|a| set.iter().map(|b| position_distance_km(a, b)).fold(0.0, f64::max))
                .fold(0.0, f64::max);
            assert!(
                spread > 1.0,
                "{planet}: the three rows differ by {spread} km, which is a copy"
            );
        }
    }

    #[test]
    fn the_rows_share_one_epoch_and_one_target_id_per_planet() {
        let rows = fixture().expect("planet fixture");
        for planet in ["Merkur", "Erde", "Jupiter", "Pluto"] {
            let set: Vec<&PlanetState> = rows.iter().filter(|r| r.planet == planet).collect();
            let epoch = set[0].epoch_jd;
            let target = set[0].target;
            for r in &set {
                assert_eq!(r.epoch_jd, epoch, "{planet} epoch");
                assert_eq!(r.target, target, "{planet} target");
            }
        }
    }

    #[test]
    fn a_row_without_all_columns_is_absent() {
        assert!(parse("Merkur\t1\tINPOP19a\t2461041.5\t1\t2\t3").is_none());
    }
}
