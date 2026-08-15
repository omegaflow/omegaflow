pub fn force_id_of(name: &str) -> Option<u8> {
    match name {
        "em" => Some(0),
        "gravity" => Some(1),
        "acoustic" => Some(2),
        "seismic-body" => Some(3),
        "seismic-surface" => Some(4),
        "thermal" => Some(5),
        "diffusion" => Some(6),
        "advective" => Some(7),
        "electric" => Some(8),
        _ => None,
    }
}

pub fn force_name_of(id: u8) -> Option<&'static str> {
    match id {
        0 => Some("em"),
        1 => Some("gravity"),
        2 => Some("acoustic"),
        3 => Some("seismic-body"),
        4 => Some("seismic-surface"),
        5 => Some("thermal"),
        6 => Some("diffusion"),
        7 => Some("advective"),
        8 => Some("electric"),
        _ => None,
    }
}

pub fn kernel_id_for_force(force: u8) -> Option<u8> {
    match force {
        0 | 1 => Some(0),
        2 | 3 | 4 | 7 | 8 => Some(1),
        5 | 6 => Some(3),
        _ => None,
    }
}

pub fn default_kernel_for(force: &str) -> Option<(&'static str, &'static str)> {
    match force {
        "em" => Some(("inverse-square", "em")),
        "gravity" => Some(("inverse-square", "gravity")),
        "acoustic" => Some(("gaussian-inverse-square", "acoustic")),
        "seismic-body" => Some(("gaussian-inverse-square", "seismic-body")),
        "seismic-surface" => Some(("erfc", "seismic-surface")),
        "thermal" => Some(("exponential-decay", "thermal")),
        "diffusion" => Some(("gaussian-inverse-square", "diffusion")),
        "advective" => Some(("patch-levy", "advective")),
        _ => None,
    }
}

pub struct TagWeight {
    pub weight: i32,
    pub force: Option<String>,
    pub tag: String,
}

pub struct GateWeight {
    pub weight: i32,
    pub force: Option<u8>,
    pub matched: Vec<String>,
}

pub fn parse_library(content: &str) -> Vec<TagWeight> {
    let mut lib = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let mut it = t.split_whitespace();
        let weight = match it.next().and_then(|w| w.parse::<i32>().ok()) {
            Some(w) => w,
            None => continue,
        };
        let force = match it.next() {
            Some("-") | None => None,
            Some(f) => Some(f.to_string()),
        };
        let tag = it.collect::<Vec<_>>().join(" ");
        if tag.is_empty() {
            continue;
        }
        lib.push(TagWeight { weight, force, tag });
    }
    lib
}

fn tag_matches(hay: &str, tag: &str) -> bool {
    if tag.contains(' ') || tag.contains('_') || tag.contains('-') {
        hay.contains(tag)
    } else {
        hay.split(|c: char| !c.is_ascii_alphanumeric())
            .any(|w| w.eq_ignore_ascii_case(tag))
    }
}

pub fn gate_weigh(text: &str, library: &[TagWeight]) -> GateWeight {
    let t = text.to_lowercase();
    let mut weight: i32 = 0;
    let mut matched: Vec<String> = Vec::new();
    let mut force_hits: Vec<(u8, i32)> = Vec::new();
    let mut has_position = false;
    for tw in library {
        let tag = tw.tag.to_lowercase();
        if tag.is_empty() || !tag_matches(&t, &tag) {
            continue;
        }
        weight += tw.weight;
        matched.push(tw.tag.clone());
        if tw.weight > 0 && tw.force.is_none() {
            has_position = true;
        }
        if let Some(fname) = &tw.force {
            if let Some(fid) = force_id_of(fname) {
                match force_hits.iter_mut().find(|(f, _)| *f == fid) {
                    Some(hit) => hit.1 += tw.weight,
                    None => force_hits.push((fid, tw.weight)),
                }
            }
        }
    }
    if !has_position && weight > 0 {
        weight -= 16;
        matched.push("absent:position".to_string());
    }
    let force = force_hits
        .into_iter()
        .max_by_key(|(_, w)| *w)
        .map(|(f, _)| f);
    GateWeight {
        weight,
        force,
        matched,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_lib() -> Vec<TagWeight> {
        parse_library(
            "8 em Magnetic susceptibility\n-8 - Forecast\n4 - Station\n8 thermal sea_water_temperature\n",
        )
    }

    #[test]
    fn test_force_roundtrip() {
        for id in 0..9 {
            let name = force_name_of(id).unwrap();
            assert_eq!(force_id_of(name), Some(id));
        }
        assert_eq!(force_id_of("biotic"), None);
        assert_eq!(force_name_of(9), None);
    }

    #[test]
    fn test_kernel_ids() {
        assert_eq!(kernel_id_for_force(8), Some(1));
        assert_eq!(kernel_id_for_force(0), Some(0));
        assert_eq!(kernel_id_for_force(5), Some(3));
        assert_eq!(kernel_id_for_force(9), None);
    }

    #[test]
    fn test_library_parse() {
        let lib = sample_lib();
        assert_eq!(lib.len(), 4);
        assert_eq!(lib[0].tag, "Magnetic susceptibility");
        assert_eq!(lib[0].weight, 8);
        assert_eq!(lib[0].force.as_deref(), Some("em"));
        assert_eq!(lib[2].tag, "Station");
        assert_eq!(lib[2].force, None);
    }

    #[test]
    fn test_gate_force_and_absence() {
        let lib = sample_lib();
        let g = gate_weigh("Magnetic susceptibility record", &lib);
        assert_eq!(g.weight, -8);
        assert_eq!(g.force, Some(0));
        assert!(g.matched.iter().any(|m| m == "absent:position"));
    }

    #[test]
    fn test_gate_position_tag() {
        let lib = sample_lib();
        let g = gate_weigh("Station data sea_water_temperature", &lib);
        assert_eq!(g.weight, 12);
        assert_eq!(g.force, Some(5));
        assert!(!g.matched.iter().any(|m| m == "absent:position"));
    }

    #[test]
    fn test_gate_mixed_decline() {
        let lib = sample_lib();
        let g = gate_weigh("Magnetic susceptibility Forecast", &lib);
        assert_eq!(g.weight, 0);
        assert_eq!(g.force, Some(0));
    }
}
