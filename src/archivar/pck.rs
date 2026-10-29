use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PckBody {
    pub naif_id: i32,
    pub gm_m3_s2: Option<f64>,
    pub pole_ra_deg: Option<f64>,
    pub pole_ra_rate_deg_per_century: Option<f64>,
    pub pole_dec_deg: Option<f64>,
    pub pole_dec_rate_deg_per_century: Option<f64>,
    pub pm_deg: Option<f64>,
    pub pm_rate_deg_per_day: Option<f64>,
    pub radii_m: Option<[f64; 3]>,
    pub j2: Option<f64>,
    pub j4: Option<f64>,
    pub nut_ra: Option<Vec<[f64; 3]>>,
    pub nut_dec: Option<Vec<[f64; 3]>>,
    pub nut_pm: Option<Vec<[f64; 3]>>,
}

impl PckBody {
    pub fn minimal(naif_id: i32) -> Self {
        PckBody {
            naif_id,
            gm_m3_s2: None,
            pole_ra_deg: None,
            pole_ra_rate_deg_per_century: None,
            pole_dec_deg: None,
            pole_dec_rate_deg_per_century: None,
            pm_deg: None,
            pm_rate_deg_per_day: None,
            radii_m: None,
            j2: None,
            j4: None,
            nut_ra: None,
            nut_dec: None,
            nut_pm: None,
        }
    }

    pub fn absent() -> Self {
        PckBody {
            naif_id: 0,
            gm_m3_s2: None,
            pole_ra_deg: None,
            pole_ra_rate_deg_per_century: None,
            pole_dec_deg: None,
            pole_dec_rate_deg_per_century: None,
            pm_deg: None,
            pm_rate_deg_per_day: None,
            radii_m: None,
            j2: None,
            j4: None,
            nut_ra: None,
            nut_dec: None,
            nut_pm: None,
        }
    }

    pub fn pole_ra_at(&self, centuries: f64) -> Option<f64> {
        let base = self.pole_ra_deg? + self.pole_ra_rate_deg_per_century? * centuries;
        match &self.nut_ra {
            Some(terms) => Some(base + nut_sum(terms, centuries)),
            None => Some(base),
        }
    }

    pub fn pole_dec_at(&self, centuries: f64) -> Option<f64> {
        let base = self.pole_dec_deg? + self.pole_dec_rate_deg_per_century? * centuries;
        match &self.nut_dec {
            Some(terms) => Some(base + nut_sum(terms, centuries)),
            None => Some(base),
        }
    }

    pub fn pm_at(&self, days: f64) -> Option<f64> {
        let base = self.pm_deg? + self.pm_rate_deg_per_day? * days;
        match &self.nut_pm {
            Some(terms) => Some(base + nut_sum(terms, days / 36525.0)),
            None => Some(base),
        }
    }

    pub fn flattening(&self) -> Option<f64> {
        let radii = self.radii_m?;
        if radii[0] > 0.0 {
            Some((radii[0] - radii[2]) / radii[0])
        } else {
            None
        }
    }
}

const LUNAR_PCK_ID: i32 = 301;

const LUNAR_E_ARGUMENTS: [(f64, f64); 13] = [
    (125.045, -0.0529921),
    (250.089, -0.1059842),
    (260.008, 13.0120009),
    (176.625, 13.3407154),
    (357.529, 0.9856003),
    (311.589, 26.4057084),
    (134.963, 13.0649930),
    (276.617, 0.3287146),
    (34.226, 1.7484877),
    (15.134, -0.1589763),
    (119.743, 0.0036096),
    (239.961, 0.1643573),
    (25.053, 12.9590088),
];

fn lunar_terms(id: i32, amplitudes: &[f64], cosine: bool) -> Option<Vec<[f64; 3]>> {
    if id != LUNAR_PCK_ID {
        return None;
    }
    let deg = std::f64::consts::PI / 180.0;
    let terms: Vec<[f64; 3]> = amplitudes
        .iter()
        .zip(LUNAR_E_ARGUMENTS.iter())
        .map(|(amp, &(constant_deg, rate_deg_per_day))| {
            let phase_deg = if cosine {
                constant_deg + 90.0
            } else {
                constant_deg
            };
            [*amp, rate_deg_per_day * 36525.0 * deg, phase_deg * deg]
        })
        .collect();
    if terms.is_empty() { None } else { Some(terms) }
}

fn nut_terms(
    id: i32,
    amplitudes: &[f64],
    cosine: bool,
    angles: &HashMap<i32, Vec<f64>>,
) -> Option<Vec<[f64; 3]>> {
    let planet = id / 100;
    let Some(raw) = angles.get(&planet) else {
        return lunar_terms(id, amplitudes, cosine);
    };
    let deg = std::f64::consts::PI / 180.0;
    let stride = if raw.len() == 3 * amplitudes.len() {
        3
    } else if raw.len() == 2 * amplitudes.len() {
        2
    } else {
        return None;
    };
    let mut terms: Vec<[f64; 3]> = Vec::with_capacity(amplitudes.len());
    for (k, amplitude) in amplitudes.iter().enumerate() {
        let base = k * stride;
        let a0 = *raw.get(base)?;
        let a1 = *raw.get(base + 1)?;
        if stride == 3 {
            let a2 = *raw.get(base + 2)?;
            if a2 != 0.0 && *amplitude != 0.0 {
                return None;
            }
        }
        let phase_deg = if cosine { a0 + 90.0 } else { a0 };
        terms.push([*amplitude, a1 * 36525.0 * deg, phase_deg * deg]);
    }
    if terms.is_empty() { None } else { Some(terms) }
}

fn nut_sum(terms: &[[f64; 3]], t: f64) -> f64 {
    terms
        .iter()
        .map(|&[amplitude, frequency, phase]| amplitude * (frequency * t + phase).sin())
        .sum()
}

pub fn parse(gm_text: Option<&str>, body_text: Option<&str>) -> HashMap<i32, PckBody> {
    let mut bodies: HashMap<i32, PckBody> = HashMap::new();
    if let Some(text) = body_text {
        let entries = scan_entries(text);
        let mut angles: HashMap<i32, Vec<f64>> = HashMap::new();
        for ((id, key), values) in &entries {
            if key == "NUT_PREC_ANGLES" {
                angles.insert(*id, values.clone());
            }
        }
        for ((id, key), values) in entries {
            let entry = bodies.entry(id).or_insert_with(|| PckBody::minimal(id));
            match key.as_str() {
                "POLE_RA" if values.len() >= 2 => {
                    entry.pole_ra_deg = Some(values[0]);
                    entry.pole_ra_rate_deg_per_century = Some(values[1]);
                }
                "POLE_DEC" if values.len() >= 2 => {
                    entry.pole_dec_deg = Some(values[0]);
                    entry.pole_dec_rate_deg_per_century = Some(values[1]);
                }
                "PM" if values.len() >= 2 => {
                    entry.pm_deg = Some(values[0]);
                    entry.pm_rate_deg_per_day = Some(values[1]);
                }
                "RADII" if values.len() >= 3 => {
                    entry.radii_m =
                        Some([values[0] * 1000.0, values[1] * 1000.0, values[2] * 1000.0]);
                }
                "CONSTANT_J2" if !values.is_empty() => entry.j2 = Some(values[0]),
                "CONSTANT_J4" if !values.is_empty() => entry.j4 = Some(values[0]),
                "J2" if !values.is_empty() => entry.j2 = Some(values[0]),
                "J4" if !values.is_empty() => entry.j4 = Some(values[0]),
                "NUT_PREC_RA" if !values.is_empty() => {
                    entry.nut_ra = nut_terms(id, &values, false, &angles)
                }
                "NUT_PREC_DEC" if !values.is_empty() => {
                    entry.nut_dec = nut_terms(id, &values, true, &angles)
                }
                "NUT_PREC_PM" if !values.is_empty() => {
                    entry.nut_pm = nut_terms(id, &values, false, &angles)
                }
                _ => {}
            }
        }
    }
    if let Some(text) = gm_text {
        for ((id, key), values) in scan_entries(text) {
            if key == "GM" && !values.is_empty() {
                let entry = bodies.entry(id).or_insert_with(|| PckBody::minimal(id));
                entry.gm_m3_s2 = Some(values[0] * 1.0e9);
            }
        }
    }
    bodies
}

fn scan_entries(text: &str) -> HashMap<(i32, String), Vec<f64>> {
    let mut out = HashMap::new();
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i + 4 < bytes.len() {
        if &bytes[i..i + 4] != b"BODY" || !bytes[i + 4].is_ascii_digit() {
            i += 1;
            continue;
        }
        let mut j = i + 4;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        let id: i32 = match text[i + 4..j].parse() {
            Ok(id) => id,
            Err(_) => {
                i += 1;
                continue;
            }
        };
        let mut k = j;
        if k >= bytes.len() || bytes[k] != b'_' {
            i += 1;
            continue;
        }
        k += 1;
        let key_start = k;
        while k < bytes.len() && bytes[k] != b'=' && !bytes[k].is_ascii_whitespace() {
            k += 1;
        }
        let key = text[key_start..k].to_string();
        let key_line_end = match text[k..].find('\n') {
            Some(p) => k + p,
            None => bytes.len(),
        };
        let eq_rel = match text[k..key_line_end].find('=') {
            Some(p) => p,
            None => {
                i += 1;
                continue;
            }
        };
        k += eq_rel + 1;
        while k < bytes.len() && bytes[k].is_ascii_whitespace() {
            k += 1;
        }
        let mut values = Vec::new();
        let mut complete = true;
        if k < bytes.len() && bytes[k] == b'(' {
            k += 1;
            let close = match text[k..].find(')') {
                Some(p) => k + p,
                None => {
                    i += 1;
                    continue;
                }
            };
            for tok in text[k..close].split_whitespace() {
                let norm: String = tok
                    .chars()
                    .map(|c| if c == 'D' || c == 'd' { 'E' } else { c })
                    .collect();
                match norm.parse::<f64>() {
                    Ok(v) => values.push(v),
                    Err(_) => {
                        complete = false;
                        break;
                    }
                }
            }
            if complete && !values.is_empty() {
                out.insert((id, key), values);
            }
            i = close + 1;
        } else {
            let line_end = match text[k..].find('\n') {
                Some(p) => k + p,
                None => bytes.len(),
            };
            for tok in text[k..line_end].split_whitespace() {
                let norm: String = tok
                    .chars()
                    .map(|c| if c == 'D' || c == 'd' { 'E' } else { c })
                    .collect();
                match norm.parse::<f64>() {
                    Ok(v) => values.push(v),
                    Err(_) => {
                        complete = false;
                        break;
                    }
                }
            }
            if complete && !values.is_empty() {
                out.insert((id, key), values);
            }
            i = line_end;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_lunar_e_series_reproduces_the_iau_orientation_at_j2000() {
        let text = "BODY301_POLE_RA = ( 269.9949 0.0031 0. )
BODY301_POLE_DEC = ( 66.5392 0.0130 0. )
BODY301_PM = ( 38.3213 13.17635815 0. )
BODY301_NUT_PREC_RA = ( -3.8787 -0.1204 0.0700 -0.0172 0.0 0.0072 0.0 0.0 0.0 -0.0052 0.0 0.0 0.0043 )
BODY301_NUT_PREC_DEC = ( 1.5419 0.0239 -0.0278 0.0068 0.0 -0.0029 0.0009 0.0 0.0 0.0008 0.0 0.0 -0.0009 )
BODY301_NUT_PREC_PM = ( 3.5610 0.1208 -0.0642 0.0158 0.0252 -0.0066 -0.0047 -0.0046 0.0028 0.0052 0.0040 0.0019 -0.0044 )";
        let bodies = parse(None, Some(text));
        let moon = &bodies[&301];
        let ra = moon.pole_ra_at(0.0).expect("moon pole ra");
        let dec = moon.pole_dec_at(0.0).expect("moon pole dec");
        let w = moon.pm_at(0.0).expect("moon prime meridian");
        assert!((ra - 266.857733).abs() < 1.0e-3, "ra {ra}");
        assert!((dec - 65.641103).abs() < 1.0e-3, "dec {dec}");
        assert!((w - 41.195264).abs() < 1.0e-3, "w {w}");
        let compass = (w - 38.3213) + (ra - 269.9949) * 0.9165;
        assert!(
            compass.abs() < 0.05,
            "the E terms must compensate: {compass}"
        );
    }

    #[test]
    fn the_moon_from_the_file_matches_the_moon_from_the_hardcoded_series() {
        let bodies = parse(None, Some(include_str!("kernels/pck00011.tpc")));
        let moon = &bodies[&301];
        let ra = moon.pole_ra_at(0.0).expect("moon pole ra");
        let dec = moon.pole_dec_at(0.0).expect("moon pole dec");
        let w = moon.pm_at(0.0).expect("moon prime meridian");
        assert!((ra - 266.857_733).abs() < 1e-5, "ra {ra}");
        assert!((dec - 65.641_103).abs() < 1e-5, "dec {dec}");
        assert!((w - 41.195_264).abs() < 1e-5, "w {w}");
    }

    #[test]
    fn mars_and_jupiter_take_their_nutation_from_the_angle_list_in_the_file() {
        let bodies = parse(None, Some(include_str!("kernels/pck00011.tpc")));
        let mars = &bodies[&499];
        let dec = mars.pole_dec_at(0.0).expect("mars dec");
        assert!((dec - 52.886_439).abs() < 1e-5, "mars dec {dec}");
        let w = mars.pm_at(0.0).expect("mars pm");
        assert!((w - 176.632_060).abs() < 1e-5, "mars meridian {w}");
        let base = mars.pole_dec_deg.expect("mars base pole");
        assert!(
            (base - dec).abs() > 1.5,
            "the nutation must move the pole by more than a degree, it moved {}",
            base - dec
        );
        let jupiter = &bodies[&599];
        let dec_j = jupiter.pole_dec_at(0.0).expect("jupiter dec");
        assert!((dec_j - 64.495_810).abs() < 1e-5, "jupiter dec {dec_j}");
    }

    #[test]
    fn mars_declination_stays_near_its_pole_over_two_centuries() {
        let bodies = parse(None, Some(include_str!("kernels/pck00011.tpc")));
        let mars = &bodies[&499];
        let mut low = f64::MAX;
        let mut high = f64::MIN;
        for year in 1900..=2100 {
            let centuries = (year as f64 - 2000.0) / 100.0;
            let dec = mars.pole_dec_at(centuries).expect("mars dec");
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

    #[test]
    fn scan_parses_gm_and_body_entries() {
        let text = "KPL/PCK\n\nBODY10_GM = ( 1.3271244004193938E+11 )\n\
BODY399_POLE_RA = ( 270.000033 0.0030 0. 0. )\n\
BODY399_POLE_DEC = ( +66.5416227 0.0130 0. 0. )\n\
BODY399_PM = ( 38.3172982 14460.0000 0. 0. )\n\
BODY399_RADII = ( 6378.1366 6378.1366 6356.7519 )\n\
BODY399_CONSTANT_J2 = ( 1.08262668E-3 )\n\
BODY399_CONSTANT_J4 = ( -1.619621591367E-6 )\n\
BODY399_NUT_PREC_RA = ( -0.177596746667 -0.0178254 0.\n 0. 0. 0. )\n";
        let entries = scan_entries(text);
        assert!(entries.contains_key(&(10, "GM".to_string())));
        assert!(entries.contains_key(&(399, "POLE_RA".to_string())));
        assert_eq!(entries[&(399, "RADII".to_string())].len(), 3);
    }

    #[test]
    fn absent_body_stays_out_of_the_map() {
        let text = "BODY399_POLE_RA = ( 270.0 0.003 )\n";
        let bodies = parse(Some("BODY10_GM = ( 1.3271244004193938E+11 )"), Some(text));
        assert!(bodies.contains_key(&399));
        assert_eq!(bodies[&399].gm_m3_s2, None);
        assert!(!bodies.contains_key(&1));
    }

    #[test]
    fn nut_sum_applies_sine_terms() {
        let terms = [[1.0, 2.0, 0.0], [0.5, 1.0, std::f64::consts::FRAC_PI_2]];
        assert!((nut_sum(&terms, 0.0) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn every_model_claiming_function_names_its_reference() {
        let table = std::fs::read_to_string("src/archivar/kernels/reference_coverage.tsv")
            .expect("the reference coverage table");
        let listed: std::collections::HashSet<&str> = table
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| l.split('\t').next())
            .collect();
        for line in table.lines().filter(|l| !l.starts_with('#')) {
            let mut cols = line.split('\t');
            let (Some(_), Some(state), Some(reference)) = (cols.next(), cols.next(), cols.next())
            else {
                continue;
            };
            if state == "reference" {
                assert!(
                    reference.len() > 4 && reference != "no reference test",
                    "a reference row must name its source: {line}"
                );
            }
        }
        let claim_tokens = [
            "iau", "sofa", "erfa", "iers", "tdb", "icrs", "exact", "precise",
        ];
        let mut missing: Vec<String> = Vec::new();
        for dir in ["src/archivar", "src/mathematikerin"] {
            let Ok(entries) = std::fs::read_dir(dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                    continue;
                }
                let Ok(text) = std::fs::read_to_string(&path) else {
                    continue;
                };
                for line in text.lines() {
                    let Some(rest) = line.trim_start().strip_prefix("pub fn ") else {
                        continue;
                    };
                    let name: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    let claims = name
                        .split('_')
                        .any(|token| claim_tokens.iter().any(|prefix| token.starts_with(prefix)));
                    if claims && !listed.contains(name.as_str()) {
                        missing.push(name);
                    }
                }
            }
        }
        assert!(
            missing.is_empty(),
            "functions claiming a model without a line in reference_coverage.tsv: {missing:?}"
        );
    }
}
