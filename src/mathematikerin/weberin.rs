use std::collections::HashMap;
use std::sync::Arc;

use crate::archivar::{
    body_barycenter_position, state_at, AsteroidRec, BodyEphemeris, J2000_EPOCH,
};

pub enum Verdict {
    Placed,
    Riss,
}

pub struct BodyVerdict {
    pub name: String,
    pub sep_m: f64,
    pub state: Verdict,
}

pub struct BodyThread {
    pub name: String,
    pub rec: AsteroidRec,
}

pub struct WeberinFeed {
    pub eph: Arc<HashMap<String, BodyEphemeris>>,
    pub recs: Vec<AsteroidRec>,
}

pub const WEBERIN_TOL_M: f64 = 1.0e6;

pub const BODY_NUMBER: &[(&str, u32)] = &[
    ("ceres", 1),
    ("pallas", 2),
    ("juno", 3),
    ("vesta", 4),
    ("pluto", 134340),
    ("bennu", 101955),
    ("apophis", 99942),
    ("eris", 136199),
    ("makemake", 136472),
    ("haumea", 136108),
];

pub fn body_number(name: &str) -> Option<u32> {
    BODY_NUMBER
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, num)| *num)
}

pub fn build_threads(body_names: &[String], recs: &[AsteroidRec]) -> Vec<BodyThread> {
    let mut by_num: HashMap<u32, AsteroidRec> = HashMap::new();
    for r in recs {
        by_num.entry(r.number).or_insert_with(|| r.clone());
    }
    let mut threads = Vec::new();
    for name in body_names {
        let Some(num) = body_number(name) else {
            continue;
        };
        let Some(rec) = by_num.get(&num) else {
            continue;
        };
        threads.push(BodyThread {
            name: name.clone(),
            rec: rec.clone(),
        });
    }
    threads.sort_by(|a, b| a.name.cmp(&b.name));
    threads
}

pub fn separation_m(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn add_sun(helio: [f64; 3], sun: [f64; 3]) -> [f64; 3] {
    [helio[0] + sun[0], helio[1] + sun[1], helio[2] + sun[2]]
}

pub fn classify(sep_m: f64, tol_m: f64) -> Verdict {
    if sep_m <= tol_m {
        Verdict::Placed
    } else {
        Verdict::Riss
    }
}

pub struct Weberin {
    pub threads: Vec<BodyThread>,
    pub verdicts: Vec<BodyVerdict>,
    pub eph: Option<Arc<HashMap<String, BodyEphemeris>>>,
    pub woven: bool,
}

impl Weberin {
    pub fn new() -> Self {
        Weberin {
            threads: Vec::new(),
            verdicts: Vec::new(),
            eph: None,
            woven: false,
        }
    }

    pub fn feed(&mut self, feed: WeberinFeed) {
        let names: Vec<String> = feed.eph.keys().cloned().collect();
        self.threads = build_threads(&names, &feed.recs);
        self.eph = Some(feed.eph);
    }

    pub fn weave(&mut self, tdb: f64, tol_m: f64) {
        let Some(eph) = self.eph.as_ref() else {
            return;
        };
        let Some(sun) = body_barycenter_position("sun", tdb, eph) else {
            return;
        };
        self.verdicts.clear();
        let jd = tdb / 86400.0 + J2000_EPOCH;
        for t in &self.threads {
            let Some(eph_p) = body_barycenter_position(&t.name, tdb, eph) else {
                continue;
            };
            let Some((helio, _)) = state_at(&t.rec, jd) else {
                continue;
            };
            let kep_bary = add_sun(helio, sun);
            let sep_m = separation_m(eph_p, kep_bary);
            let state = classify(sep_m, tol_m);
            self.verdicts.push(BodyVerdict {
                name: t.name.clone(),
                sep_m,
                state,
            });
        }
        self.woven = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(number: u32) -> AsteroidRec {
        AsteroidRec {
            number,
            epoch_jd: J2000_EPOCH,
            a_au: 2.7,
            e: 0.07,
            incl_deg: 10.0,
            node_deg: 80.0,
            peri_deg: 73.0,
            ma_deg: 10.0,
            h: 0.0,
            g: 0.0,
            albedo: 0.0,
            rot_period_h: 0.0,
            radius_km: 0.0,
            gm_km3_s2: 0.0,
            sptype: [0; 5],
        }
    }

    #[test]
    fn body_number_maps_known_bodies() {
        assert_eq!(body_number("ceres"), Some(1));
        assert_eq!(body_number("vesta"), Some(4));
        assert_eq!(body_number("bennu"), Some(101955));
        assert_eq!(body_number("apophis"), Some(99942));
        assert_eq!(body_number("pluto"), Some(134340));
        assert_eq!(body_number("moon"), None);
        assert_eq!(body_number("charon"), None);
    }

    #[test]
    fn build_threads_matches_bodies_with_a_second_line() {
        let names = vec![
            "ceres".to_string(),
            "vesta".to_string(),
            "charon".to_string(),
        ];
        let recs = vec![rec(1), rec(4)];
        let threads = build_threads(&names, &recs);
        let got: Vec<&str> = threads.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(got, vec!["ceres", "vesta"]);
    }

    #[test]
    fn classify_places_within_tolerance_and_risses_beyond() {
        assert!(matches!(classify(1.0, 10.0), Verdict::Placed));
        assert!(matches!(classify(10.0, 10.0), Verdict::Placed));
        assert!(matches!(classify(10.1, 10.0), Verdict::Riss));
    }

    #[test]
    fn separation_is_euclidean() {
        assert_eq!(separation_m([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]), 5.0);
    }

    #[test]
    fn add_sun_shifts_helio_to_barycenter() {
        let bary = add_sun([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
        assert_eq!(bary, [5.0, 7.0, 9.0]);
    }
}
