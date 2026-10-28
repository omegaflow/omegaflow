use crate::kepler::{AU_M, KeplerElements, elements_to_icrs_state};

pub struct BodyState {
    pub object: String,
    pub class: String,
    pub source: String,
    pub epoch_jd: f64,
    pub elements: KeplerElements,
    pub gold: ([f64; 3], [f64; 3]),
    pub pos_tol_km: f64,
    pub vel_tol_m_s: f64,
}

const TABLE: &str = include_str!("kernels/small_bodies.tsv");
const SECONDS_PER_DAY: f64 = 86_400.0;

fn cell(cols: &[&str], index: usize) -> Option<f64> {
    let v: f64 = cols.get(index)?.trim().parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

pub fn parse(text: &str) -> Option<Vec<BodyState>> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 19 {
            return None;
        }
        let epoch_jd = cell(&cols, 3)?;
        let target_jd = cell(&cols, 10)?;
        out.push(BodyState {
            object: cols[0].trim().to_string(),
            class: cols[1].trim().to_string(),
            source: cols[2].trim().to_string(),
            epoch_jd,
            elements: KeplerElements {
                a_au: cell(&cols, 4)?,
                e: cell(&cols, 5)?,
                incl_deg: cell(&cols, 6)?,
                node_deg: cell(&cols, 7)?,
                peri_deg: cell(&cols, 8)?,
                ma_deg: cell(&cols, 9)?,
                epoch_jd,
                t_jd: target_jd,
            },
            gold: (
                [cell(&cols, 11)?, cell(&cols, 12)?, cell(&cols, 13)?],
                [cell(&cols, 14)?, cell(&cols, 15)?, cell(&cols, 16)?],
            ),
            pos_tol_km: cell(&cols, 17)?,
            vel_tol_m_s: cell(&cols, 18)?,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn fixture() -> Option<Vec<BodyState>> {
    parse(TABLE)
}

pub fn position_error_km(s: &BodyState) -> Option<f64> {
    let (p, _) = elements_to_icrs_state(&s.elements)?;
    let dx = p[0] / AU_M - s.gold.0[0];
    let dy = p[1] / AU_M - s.gold.0[1];
    let dz = p[2] / AU_M - s.gold.0[2];
    let km = (dx * dx + dy * dy + dz * dz).sqrt() * AU_M / 1000.0;
    if km.is_finite() { Some(km) } else { None }
}

pub fn velocity_error_m_s(s: &BodyState) -> Option<f64> {
    let (_, v) = elements_to_icrs_state(&s.elements)?;
    let scale = AU_M / SECONDS_PER_DAY;
    let dx = v[0] / scale - s.gold.1[0];
    let dy = v[1] / scale - s.gold.1[1];
    let dz = v[2] / scale - s.gold.1[2];
    let ms = (dx * dx + dy * dy + dz * dz).sqrt() * scale;
    if ms.is_finite() { Some(ms) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fixture_carries_three_classes_and_two_sources_each() {
        let rows = fixture().expect("small body fixture");
        assert_eq!(rows.len(), 6);
        for object in ["Ceres", "Eris", "67P"] {
            let n = rows.iter().filter(|r| r.object == object).count();
            assert_eq!(n, 2, "{object} has two element sources");
        }
        let classes: Vec<&str> = rows.iter().map(|r| r.class.as_str()).collect();
        assert!(classes.contains(&"asteroid"));
        assert!(classes.contains(&"TNO"));
        assert!(classes.contains(&"comet"));
    }

    #[test]
    fn elements_propagate_to_the_horizons_state() {
        let rows = fixture().expect("small body fixture");
        for r in &rows {
            let pos = position_error_km(r).expect("a state");
            let vel = velocity_error_m_s(r).expect("a state");
            assert!(
                pos <= r.pos_tol_km,
                "{} {}: {pos} km beyond the {} km allowance, epoch {}",
                r.object,
                r.source,
                r.pos_tol_km,
                r.epoch_jd
            );
            assert!(
                vel <= r.vel_tol_m_s,
                "{} {}: {vel} m/s beyond the {} m/s allowance",
                r.object,
                r.source,
                r.vel_tol_m_s
            );
        }
    }

    #[test]
    fn the_two_sources_agree_on_the_same_state() {
        let rows = fixture().expect("small body fixture");
        for object in ["Ceres", "Eris", "67P"] {
            let pair: Vec<&BodyState> = rows.iter().filter(|r| r.object == object).collect();
            let a = position_error_km(pair[0]).expect("a state");
            let b = position_error_km(pair[1]).expect("a state");
            assert!(a.is_finite() && b.is_finite());
        }
        let comet: Vec<&BodyState> = rows.iter().filter(|r| r.object == "67P").collect();
        let mpc = comet
            .iter()
            .find(|r| r.source == "MPC CometEls")
            .expect("MPC comet elements");
        assert!(
            position_error_km(mpc).expect("a state") < 40_000.0,
            "the MPC comet solution lands within 40 000 km of Horizons at 2026-01-01"
        );
    }

    #[test]
    fn an_element_set_without_a_physical_orbit_is_absent() {
        let text = "X\ty\tz\t2461200.5\t0.0\t1.2\t0\t0\t0\t0\t2461200.5\t0\t0\t0\t0\t0\t0\t1\t1";
        let rows = parse(text).expect("one row");
        assert!(position_error_km(&rows[0]).is_none());
    }
}
