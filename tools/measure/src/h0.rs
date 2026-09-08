#[derive(Clone)]
pub struct Cepheid {
    pub name: String,
    pub logp: f64,
    pub mw: f64,
    pub sig_mw: f64,
    pub feh: Option<f64>,
    pub pi_edr3: Option<f64>,
    pub sig_edr3: Option<f64>,
    pub marked: bool,
}

pub fn parse_cepheids(tex: &str) -> (Vec<Cepheid>, usize) {
    let mut stars = Vec::new();
    let mut absent = 0usize;
    let mut in_data = false;
    for line in tex.lines() {
        let t = line.trim();
        if t.starts_with("\\startdata") {
            in_data = true;
            continue;
        }
        if t.starts_with("\\enddata") {
            break;
        }
        if !in_data {
            continue;
        }
        if t.is_empty() || t.starts_with("\\multicolumn") || t.starts_with("\\table") {
            continue;
        }
        let body = t.trim_end_matches('\\').trim();
        let cells: Vec<&str> = body.split('&').map(|c| c.trim()).collect();
        if cells.len() != 15 {
            continue;
        }
        let marked = cells[0].contains('$');
        let name = cells[0]
            .chars()
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        let logp = match cells[1].parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let mw = match cells[8].parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let sig_mw = match cells[9].parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let feh = cells[10].parse::<f64>().ok();
        let pi = cells[13].parse::<f64>().ok();
        let sig_pi = cells[14].parse::<f64>().ok();
        if pi.is_none() || sig_pi.is_none() {
            absent += 1;
            stars.push(Cepheid {
                name,
                logp,
                mw,
                sig_mw,
                feh,
                pi_edr3: None,
                sig_edr3: None,
                marked,
            });
            continue;
        }
        stars.push(Cepheid {
            name,
            logp,
            mw,
            sig_mw,
            feh,
            pi_edr3: pi,
            sig_edr3: sig_pi,
            marked,
        });
    }
    (stars, absent)
}

pub fn normalize_name(raw: &str) -> String {
    let stem = raw.split('$').next().unwrap_or(raw).trim();
    let parts: Vec<&str> = stem.split('-').collect();
    if parts.len() < 2 {
        return stem.to_string();
    }
    let con = con_expand(parts[parts.len() - 1]);
    let des = parts[..parts.len() - 1].join("");
    format!("{} {}", strip_v_zeros(&des), con)
}

fn strip_v_zeros(des: &str) -> String {
    if let Some(rest) = des.strip_prefix('V') {
        if !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(num) = rest.parse::<u64>() {
                return format!("V{num}");
            }
        }
    }
    des.to_string()
}

fn con_expand(code: &str) -> String {
    match code {
        "CMA" => "CMa".to_string(),
        "TRA" => "TrA".to_string(),
        _ => {
            let mut c = code.chars();
            match c.next() {
                Some(f) => {
                    let up: String = f.to_uppercase().collect();
                    format!("{}{}", up, c.as_str().to_lowercase())
                }
                None => code.to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cepheid_rows_and_counts_absent() {
        let tex = "\\startdata\nAA-GEM & 1.053 & 9.9130 & 0.029 & 8.542 & 0.025 & 7.348 & 0.017 & 6.860 & 0.023 & -0.080 & 0.259 & 0.008 & 0.311 & 0.019 \\\\\nCY-AUR & 0.9 & 8.0 & 0.02 & 7.0 & 0.02 & 6.0 & 0.02 & 5.0 & 0.02 & -0.1 & 0.2 & 0.01 & \\nd & \\nd \\\\\n\\enddata\n";
        let (stars, absent) = parse_cepheids(tex);
        assert_eq!(stars.len(), 2);
        assert_eq!(absent, 1);
        assert_eq!(stars[0].name, "AA-GEM");
        assert!(stars[0].pi_edr3.is_some());
        assert!(stars[1].pi_edr3.is_none());
    }

    #[test]
    fn normalizes_table_names_to_simbad_ids() {
        assert_eq!(normalize_name("AA-GEM"), "AA Gem");
        assert_eq!(normalize_name("V-339-CEN"), "V339 Cen");
        assert_eq!(normalize_name("V0386-CYG"), "V386 Cyg");
        assert_eq!(normalize_name("V0482-SCO"), "V482 Sco");
        assert_eq!(normalize_name("S-CRU$^e$"), "S Cru");
        assert_eq!(normalize_name("S-VUL$^*$$^*$"), "S Vul");
        assert_eq!(normalize_name("S-TRA"), "S TrA");
        assert_eq!(normalize_name("RY-CMA"), "RY CMa");
        assert_eq!(normalize_name("T-MON"), "T Mon");
        assert_eq!(normalize_name("U-AQL"), "U Aql");
        assert_eq!(normalize_name("VY-CAR"), "VY Car");
        assert_eq!(normalize_name("W-GEM"), "W Gem");
    }
}
