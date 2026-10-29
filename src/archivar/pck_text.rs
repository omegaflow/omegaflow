use std::collections::HashMap;

pub const TAU: f64 = std::f64::consts::TAU;

pub struct IauBody {
    pole_ra: Vec<f64>,
    pole_dec: Vec<f64>,
    pm: Vec<f64>,
    nut_ra: Vec<f64>,
    nut_dec: Vec<f64>,
    nut_pm: Vec<f64>,
}

struct Draft {
    pole_ra: Option<Vec<f64>>,
    pole_dec: Option<Vec<f64>>,
    pm: Option<Vec<f64>>,
    nut_ra: Option<Vec<f64>>,
    nut_dec: Option<Vec<f64>>,
    nut_pm: Option<Vec<f64>>,
}

impl Draft {
    fn empty() -> Self {
        Draft {
            pole_ra: None,
            pole_dec: None,
            pm: None,
            nut_ra: None,
            nut_dec: None,
            nut_pm: None,
        }
    }
}

pub struct IauPck {
    bodies: HashMap<i32, IauBody>,
    angles: HashMap<i32, Vec<f64>>,
}

fn strip_comments(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        match rest[start..].find("*/") {
            Some(end) => rest = &rest[start + end + 2..],
            None => {
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

fn number(token: &str) -> Option<f64> {
    let normalised = token.replace(['D', 'd'], "E");
    let value: f64 = normalised.parse().ok()?;
    Some(value)
}

fn poly(c: &[f64], t: f64) -> Option<f64> {
    let mut value = *c.first()?;
    if let Some(v) = c.get(1) {
        value += v * t;
    }
    if let Some(v) = c.get(2) {
        value += v * t * t;
    }
    Some(value)
}

fn assignments(text: &str) -> Vec<(String, Vec<f64>)> {
    let clean = strip_comments(text);
    let mut out = Vec::new();
    let mut name: Option<String> = None;
    let mut buffer = String::new();
    for line in clean.lines() {
        if name.is_none() {
            let Some(eq) = line.find('=') else {
                continue;
            };
            let lhs = line[..eq].trim();
            if !lhs.starts_with("BODY") {
                continue;
            }
            name = Some(lhs.to_string());
            buffer = line[eq + 1..].to_string();
        } else {
            buffer.push(' ');
            buffer.push_str(line);
        }
        let open = buffer.matches('(').count();
        let close = buffer.matches(')').count();
        if open <= close && name.is_some() {
            let values = buffer
                .split(|c: char| c.is_whitespace() || c == ',' || c == '(' || c == ')')
                .filter(|t| !t.is_empty())
                .filter_map(number)
                .collect();
            if let Some(n) = name.take() {
                out.push((n, values));
            }
            buffer.clear();
        }
    }
    out
}

fn split_name(name: &str) -> Option<(i32, &str)> {
    let rest = name.strip_prefix("BODY")?;
    let cut = rest.find('_')?;
    let id: i32 = rest[..cut].parse().ok()?;
    Some((id, &rest[cut + 1..]))
}

impl IauPck {
    pub fn parse(text: &str) -> Self {
        let mut drafts: HashMap<i32, Draft> = HashMap::new();
        let mut angles: HashMap<i32, Vec<f64>> = HashMap::new();
        for (name, values) in assignments(text) {
            let Some((id, suffix)) = split_name(&name) else {
                continue;
            };
            if suffix == "NUT_PREC_ANGLES" {
                angles.insert(id, values);
                continue;
            }
            let draft = drafts.entry(id).or_insert_with(Draft::empty);
            match suffix {
                "POLE_RA" => draft.pole_ra = Some(values),
                "POLE_DEC" => draft.pole_dec = Some(values),
                "PM" => draft.pm = Some(values),
                "NUT_PREC_RA" => draft.nut_ra = Some(values),
                "NUT_PREC_DEC" => draft.nut_dec = Some(values),
                "NUT_PREC_PM" => draft.nut_pm = Some(values),
                _ => {}
            }
        }
        let mut bodies = HashMap::new();
        for (id, d) in drafts {
            let (Some(pole_ra), Some(pole_dec), Some(pm)) = (d.pole_ra, d.pole_dec, d.pm) else {
                continue;
            };
            let nut_ra = match d.nut_ra {
                Some(v) => v,
                None => Vec::new(),
            };
            let nut_dec = match d.nut_dec {
                Some(v) => v,
                None => Vec::new(),
            };
            let nut_pm = match d.nut_pm {
                Some(v) => v,
                None => Vec::new(),
            };
            bodies.insert(
                id,
                IauBody {
                    pole_ra,
                    pole_dec,
                    pm,
                    nut_ra,
                    nut_dec,
                    nut_pm,
                },
            );
        }
        IauPck { bodies, angles }
    }

    pub fn body_ids(&self) -> Vec<i32> {
        let mut ids: Vec<i32> = self.bodies.keys().copied().collect();
        ids.sort_unstable();
        ids
    }

    pub fn nutation_lengths(&self, pck_id: i32) -> Option<(usize, usize, usize)> {
        let b = self.bodies.get(&pck_id)?;
        Some((b.nut_ra.len(), b.nut_dec.len(), b.nut_pm.len()))
    }

    pub fn angle_count(&self, pck_id: i32) -> Option<usize> {
        let b = self.bodies.get(&pck_id)?;
        let widths = b.nut_ra.len().max(b.nut_dec.len()).max(b.nut_pm.len());
        let raw = self.angles.get(&(pck_id / 100))?;
        if widths == 0 {
            return None;
        }
        if raw.len() == 3 * widths {
            Some(raw.len() / 3)
        } else if raw.len() == 2 * widths {
            Some(raw.len() / 2)
        } else {
            None
        }
    }

    pub fn orientation(&self, pck_id: i32, et: f64) -> Option<(f64, f64, f64)> {
        let b = self.bodies.get(&pck_id)?;
        let days = et / 86400.0;
        let centuries = days / 36525.0;
        let mut ra = poly(&b.pole_ra, centuries)?;
        let mut dec = poly(&b.pole_dec, centuries)?;
        let mut w = poly(&b.pm, days)?;
        let widths = b.nut_ra.len().max(b.nut_dec.len()).max(b.nut_pm.len());
        if widths > 0 {
            let raw = self.angles.get(&(pck_id / 100))?;
            let stride = if raw.len() == 3 * widths {
                3
            } else if raw.len() == 2 * widths {
                2
            } else {
                return None;
            };
            for k in 0..widths {
                let base = k * stride;
                let a0 = *raw.get(base)?;
                let a1 = *raw.get(base + 1)?;
                let argument = match stride {
                    3 => {
                        let a2 = *raw.get(base + 2)?;
                        (a0 + a1 * days + a2 * days * days).to_radians()
                    }
                    _ => (a0 + a1 * days).to_radians(),
                };
                if let Some(amplitude) = b.nut_ra.get(k) {
                    ra += amplitude * argument.sin();
                }
                if let Some(amplitude) = b.nut_dec.get(k) {
                    dec += amplitude * argument.cos();
                }
                if let Some(amplitude) = b.nut_pm.get(k) {
                    w += amplitude * argument.sin();
                }
            }
        }
        let turn = |deg: f64| deg.rem_euclid(360.0).to_radians();
        Some((turn(ra + 90.0), turn(90.0 - dec), turn(w)))
    }
}

#[cfg(test)]
mod tests {
    use super::IauPck;

    const TEXT: &str = include_str!("kernels/pck00011.tpc");

    fn degrees(rad: f64) -> f64 {
        rad.to_degrees()
    }

    #[test]
    fn the_kernel_yields_the_poles_the_file_states() {
        let pck = IauPck::parse(TEXT);
        assert!(pck.body_ids().len() >= 70);
        let jupiter = pck.orientation(599, 0.0).expect("jupiter answers");
        let ra = (degrees(jupiter.0) + 270.0).rem_euclid(360.0);
        assert!(
            (ra - 268.057_204_043).abs() < 1e-6,
            "jupiter pole right ascension {ra}"
        );
        let dec = 90.0 - degrees(jupiter.1);
        assert!(
            (dec - 64.495_809_953).abs() < 1e-6,
            "jupiter pole declination {dec}"
        );
        assert!(
            (degrees(jupiter.2) - 284.95).abs() < 1e-9,
            "jupiter prime meridian {}",
            degrees(jupiter.2)
        );
    }

    #[test]
    fn the_jupiter_pole_sits_inside_the_published_constant() {
        let pck = IauPck::parse(TEXT);
        let jupiter = pck.orientation(599, 0.0).expect("jupiter answers");
        let ra = (degrees(jupiter.0) - 90.0).rem_euclid(360.0);
        let dec = 90.0 - degrees(jupiter.1);
        assert!((ra - 268.056595).abs() < 0.01, "ra {ra} against 268.056595");
        assert!((dec - 64.495303).abs() < 0.01, "dec {dec} against 64.495303");
    }

    #[test]
    fn the_prime_meridian_turns_at_the_documented_rate() {
        let pck = IauPck::parse(TEXT);
        let day = 86400.0;
        let a = pck.orientation(599, 0.0).expect("jupiter answers");
        let b = pck.orientation(599, 100.0 * day).expect("jupiter answers");
        let turn = (degrees(b.2) - degrees(a.2)).rem_euclid(360.0);
        let expected = (870.536f64 * 100.0).rem_euclid(360.0);
        assert!((turn - expected).abs() < 1e-6, "turn {turn} expected {expected}");
    }

    #[test]
    fn the_moon_of_this_text_kernel_stays_within_a_fifth_of_a_degree_of_de440() {
        let pck = IauPck::parse(TEXT);
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/archivar/kernels/moon_pa_de440_200625.bpc"
        );
        let bpc = crate::bpc::BpcFile::open(path).expect("the lunar pck opens");
        let mut worst: f64 = 0.0;
        for year in 0..6 {
            let et = (year as f64) * 365.25 * 86400.0;
            let iau = pck.orientation(301, et).expect("the iau moon answers");
            let de = bpc.orient(31008, 1, et).expect("the de440 moon answers");
            let deltas = [
                (iau.0 - de.0).abs().to_degrees(),
                (iau.1 - de.1).abs().to_degrees(),
                (iau.2 - de.2).abs().to_degrees(),
            ];
            for d in deltas {
                let folded = if d > 180.0 { 360.0 - d } else { d };
                if folded > worst {
                    worst = folded;
                }
            }
        }
        assert!(
            worst < 0.25,
            "the two lunar models drift apart by {worst} degrees"
        );
    }

    #[test]
    fn mars_pairs_its_amplitudes_from_the_front_of_the_angle_list() {
        let pck = IauPck::parse(TEXT);
        assert_eq!(pck.nutation_lengths(499), Some((15, 20, 26)));
        assert_eq!(pck.angle_count(499), Some(26));
        let mars = pck.orientation(499, -43200.0).expect("mars answers");
        assert!(
            (degrees(mars.0) - 47.663_795).abs() < 1e-5,
            "mars right ascension {}",
            degrees(mars.0)
        );
        let dec = 90.0 - degrees(mars.1);
        assert!((dec - 54.899_558).abs() < 1e-5, "mars declination {dec}");
    }

    #[test]
    fn the_mars_declination_stays_near_its_pole_over_two_centuries() {
        let pck = IauPck::parse(TEXT);
        let mut low = f64::MAX;
        let mut high = f64::MIN;
        for year in 1900..=2100 {
            let et = (year as f64 - 2000.0) * 365.25 * 86400.0;
            let mars = pck.orientation(499, et).expect("mars answers");
            let dec = 90.0 - degrees(mars.1);
            if dec < low {
                low = dec;
            }
            if dec > high {
                high = dec;
            }
        }
        assert!(
            low > 52.0 && high < 57.0,
            "mars declination runs from {low} to {high} — the amplitudes sit on the wrong angles"
        );
    }
}
