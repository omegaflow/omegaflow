use std::collections::HashMap;
use std::sync::Arc;

use crate::archivar::{
    body_barycenter_position, state_at, AsteroidRec, BodyEphemeris, J2000_EPOCH,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Placed,
    Absent,
    DirectionOnly,
}

impl Verdict {
    pub fn word(&self) -> &'static str {
        match self {
            Verdict::Placed => "placed",
            Verdict::Absent => "absent",
            Verdict::DirectionOnly => "direction-only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyLine {
    Spk,
    Dastcom,
}

impl BodyLine {
    pub fn word(&self) -> &'static str {
        match self {
            BodyLine::Spk => "spk-ephemeris",
            BodyLine::Dastcom => "dastcom-keplerian",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Agreement {
    Placed { sep_m: f64 },
    Riss { sep_m: f64 },
}

pub fn classify(sep_m: f64, tol_m: f64) -> Agreement {
    if sep_m <= tol_m {
        Agreement::Placed { sep_m }
    } else {
        Agreement::Riss { sep_m }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyOutcome {
    Placed { sep_m: f64 },
    Absent { line: BodyLine },
    Riss { sep_m: f64, knot: [BodyLine; 2] },
}

impl BodyOutcome {
    pub fn fadenpruefung(&self) -> Option<Verdict> {
        match self {
            BodyOutcome::Placed { .. } => Some(Verdict::Placed),
            BodyOutcome::Absent { .. } => Some(Verdict::Absent),
            BodyOutcome::Riss { .. } => None,
        }
    }

    pub fn word(&self) -> &'static str {
        match self {
            BodyOutcome::Placed { .. } => "placed",
            BodyOutcome::Absent { .. } => "absent",
            BodyOutcome::Riss { .. } => "riss",
        }
    }
}

pub struct BodyVerdict {
    pub name: String,
    pub outcome: BodyOutcome,
}

pub struct BodyThread {
    pub name: String,
    pub rec: Option<AsteroidRec>,
}

pub struct WeberinFeed {
    pub eph: Arc<HashMap<String, BodyEphemeris>>,
    pub sun: Arc<HashMap<String, BodyEphemeris>>,
    pub recs: Vec<AsteroidRec>,
}

pub struct Weberin {
    pub threads: Vec<BodyThread>,
    pub verdicts: Vec<BodyVerdict>,
    pub eph: Option<Arc<HashMap<String, BodyEphemeris>>>,
    pub sun: Option<Arc<HashMap<String, BodyEphemeris>>>,
    pub woven: bool,
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
    let mut by_num: HashMap<u32, &AsteroidRec> = HashMap::new();
    for r in recs {
        by_num.entry(r.number).or_insert(r);
    }
    let mut threads = Vec::new();
    for name in body_names {
        let rec = body_number(name)
            .and_then(|num| by_num.get(&num).copied())
            .cloned();
        threads.push(BodyThread {
            name: name.clone(),
            rec,
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

impl Weberin {
    pub fn new() -> Self {
        Weberin {
            threads: Vec::new(),
            verdicts: Vec::new(),
            eph: None,
            sun: None,
            woven: false,
        }
    }

    pub fn feed(&mut self, feed: WeberinFeed) {
        let eph = feed.eph;
        let mut names: Vec<String> = eph.keys().cloned().collect();
        for (n, _) in BODY_NUMBER {
            if !names.iter().any(|k| k == n) {
                names.push((*n).to_string());
            }
        }
        self.threads = build_threads(&names, &feed.recs);
        self.eph = Some(eph);
        self.sun = Some(feed.sun);
    }

    pub fn weave(&mut self, tdb: f64, tol_m: f64) {
        let (Some(eph), Some(sun_map)) = (self.eph.as_ref(), self.sun.as_ref()) else {
            return;
        };
        let Some(sun) = body_barycenter_position("sun", tdb, sun_map) else {
            return;
        };
        self.verdicts.clear();
        let jd = tdb / 86400.0 + J2000_EPOCH;
        for t in &self.threads {
            let spk = body_barycenter_position(&t.name, tdb, eph);
            let kepler = t.rec.as_ref().and_then(|r| state_at(r, jd));
            let outcome = match (spk, kepler) {
                (Some(spk_p), Some((helio, _))) => {
                    let sep_m = separation_m(spk_p, add_sun(helio, sun));
                    match classify(sep_m, tol_m) {
                        Agreement::Placed { sep_m } => BodyOutcome::Placed { sep_m },
                        Agreement::Riss { sep_m } => BodyOutcome::Riss {
                            sep_m,
                            knot: [BodyLine::Spk, BodyLine::Dastcom],
                        },
                    }
                }
                (Some(_), None) => BodyOutcome::Absent {
                    line: BodyLine::Dastcom,
                },
                (None, _) => BodyOutcome::Absent {
                    line: BodyLine::Spk,
                },
            };
            self.verdicts.push(BodyVerdict {
                name: t.name.clone(),
                outcome,
            });
        }
        self.woven = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::motion::{ChebyshevGranule, CHEBYSHEV_N};
    use std::sync::atomic::AtomicUsize;

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

    fn constant_eph(p: [f64; 3], jd: f64) -> BodyEphemeris {
        let mut cx = [0.0; CHEBYSHEV_N];
        let mut cy = [0.0; CHEBYSHEV_N];
        let mut cz = [0.0; CHEBYSHEV_N];
        cx[0] = p[0];
        cy[0] = p[1];
        cz[0] = p[2];
        BodyEphemeris {
            granules: vec![ChebyshevGranule {
                t0_jd: jd,
                dt_jd: 365.25,
                cx,
                cy,
                cz,
            }],
            rotation_matrices: Vec::new(),
            props: None,
            orbit: None,
            granule_hint: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn map_of(pairs: &[(&str, [f64; 3])]) -> Arc<HashMap<String, BodyEphemeris>> {
        Arc::new(
            pairs
                .iter()
                .map(|(n, p)| ((*n).to_string(), constant_eph(*p, J2000_EPOCH)))
                .collect(),
        )
    }

    fn woven(
        eph_pairs: &[(&str, [f64; 3])],
        sun: [f64; 3],
        recs: Vec<AsteroidRec>,
        tol: f64,
    ) -> Weberin {
        let eph = map_of(eph_pairs);
        let sun_map = map_of(&[("sun", sun)]);
        let mut w = Weberin::new();
        w.feed(WeberinFeed {
            eph,
            sun: sun_map,
            recs,
        });
        w.weave(0.0, tol);
        w
    }

    fn outcome<'a>(w: &'a Weberin, name: &str) -> Option<&'a BodyOutcome> {
        w.verdicts
            .iter()
            .find(|v| v.name == name)
            .map(|v| &v.outcome)
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
    fn build_threads_keeps_every_union_body_with_its_second_line_optional() {
        let names: Vec<String> = ["ceres", "vesta", "charon"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let recs = vec![rec(1), rec(4)];
        let threads = build_threads(&names, &recs);
        assert_eq!(threads.len(), 3);
        let ceres = threads.iter().find(|t| t.name == "ceres").unwrap();
        assert!(ceres.rec.is_some());
        let vesta = threads.iter().find(|t| t.name == "vesta").unwrap();
        assert!(vesta.rec.is_some());
        let charon = threads.iter().find(|t| t.name == "charon").unwrap();
        assert!(charon.rec.is_none());
    }

    #[test]
    fn classify_places_within_tolerance_and_risses_beyond() {
        assert!(matches!(classify(1.0, 10.0), Agreement::Placed { .. }));
        assert!(matches!(classify(10.0, 10.0), Agreement::Placed { .. }));
        assert!(matches!(classify(10.1, 10.0), Agreement::Riss { .. }));
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

    #[test]
    fn weave_places_two_converging_lines() {
        let ceres = rec(1);
        let jd = J2000_EPOCH;
        let (helio, _) = state_at(&ceres, jd).unwrap();
        let sun = [-helio[0], -helio[1], -helio[2]];
        let w = woven(&[("ceres", [0.0; 3])], sun, vec![ceres], WEBERIN_TOL_M);
        match outcome(&w, "ceres") {
            Some(BodyOutcome::Placed { sep_m }) => {
                assert!(sep_m.is_finite(), "a measured separation stays finite");
                assert!(*sep_m <= WEBERIN_TOL_M);
                assert!(matches!(
                    outcome(&w, "ceres").and_then(|o| o.fadenpruefung()),
                    Some(Verdict::Placed)
                ));
            }
            other => panic!("the folded lines read {other:?}"),
        }
    }

    #[test]
    fn weave_risses_when_two_present_lines_refuse_to_converge() {
        let ceres = rec(1);
        let w = woven(&[("ceres", [0.0; 3])], [0.0; 3], vec![ceres], 1.0e6);
        match outcome(&w, "ceres") {
            Some(BodyOutcome::Riss { sep_m, knot }) => {
                assert!(sep_m.is_finite());
                assert!(*sep_m > 1.0e6, "the refusing lines sit far apart: {sep_m}");
                match knot {
                    [BodyLine::Spk, BodyLine::Dastcom] => {}
                    other => panic!("the riss knot reads {other:?}"),
                }
                assert_eq!(
                    outcome(&w, "ceres").and_then(|o| o.fadenpruefung()),
                    None,
                    "a riss is not a fadenpruefung verdict"
                );
            }
            other => panic!("the refusing lines read {other:?}"),
        }
    }

    #[test]
    fn weave_absent_names_the_missing_dastcom_line() {
        let w = woven(&[("ceres", [0.0; 3])], [0.0; 3], Vec::new(), WEBERIN_TOL_M);
        match outcome(&w, "ceres") {
            Some(BodyOutcome::Absent { line }) => {
                assert!(matches!(line, BodyLine::Dastcom));
                assert!(matches!(
                    outcome(&w, "ceres").and_then(|o| o.fadenpruefung()),
                    Some(Verdict::Absent)
                ));
            }
            other => panic!("the record-less line reads {other:?}"),
        }
    }

    #[test]
    fn weave_absent_names_the_missing_spk_line() {
        let w = woven(&[], [0.0; 3], vec![rec(1)], WEBERIN_TOL_M);
        match outcome(&w, "ceres") {
            Some(BodyOutcome::Absent { line }) => {
                assert!(matches!(line, BodyLine::Spk));
            }
            other => panic!("the ephemeris-less line reads {other:?}"),
        }
    }

    #[test]
    fn weave_covers_the_full_body_union_even_when_only_the_second_line_lies_in() {
        let w = woven(&[], [0.0; 3], vec![rec(134340)], WEBERIN_TOL_M);
        assert!(
            outcome(&w, "pluto").is_some(),
            "pluto is woven from its record alone"
        );
        assert!(
            outcome(&w, "ceres").is_some(),
            "ceres is woven from the body number alone"
        );
        assert!(w.woven);
    }
}
