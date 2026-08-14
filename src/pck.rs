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
}

impl PckBody {
    fn entry(naif_id: i32) -> Self {
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
        Some(self.pm_deg? + self.pm_rate_deg_per_day? * days)
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

fn nut_sum(terms: &[[f64; 3]], t: f64) -> f64 {
    terms
        .iter()
        .map(|&[amplitude, frequency, phase]| amplitude * (frequency * t + phase).sin())
        .sum()
}

pub fn neutral(v: Option<f64>) -> f64 {
    match v {
        Some(x) => x,
        None => 0.0,
    }
}

pub fn parse(gm_text: Option<&str>, body_text: Option<&str>) -> HashMap<i32, PckBody> {
    let mut bodies: HashMap<i32, PckBody> = HashMap::new();
    if let Some(text) = body_text {
        for ((id, key), values) in scan_entries(text) {
            let entry = bodies.entry(id).or_insert_with(|| PckBody::entry(id));
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
                "NUT_PREC_RA" if values.len() % 3 == 0 => {
                    entry.nut_ra = Some(chunk_triples(&values))
                }
                "NUT_PREC_DEC" if values.len() % 3 == 0 => {
                    entry.nut_dec = Some(chunk_triples(&values))
                }
                _ => {}
            }
        }
    }
    if let Some(text) = gm_text {
        for ((id, key), values) in scan_entries(text) {
            if key == "GM" && !values.is_empty() {
                let entry = bodies.entry(id).or_insert_with(|| PckBody::entry(id));
                entry.gm_m3_s2 = Some(values[0] * 1.0e9);
            }
        }
    }
    bodies
}

fn chunk_triples(values: &[f64]) -> Vec<[f64; 3]> {
    values.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect()
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
}

