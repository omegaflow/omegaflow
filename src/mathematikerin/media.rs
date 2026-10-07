pub struct MediumParams {
    pub sound_speed_m_s: f64,
    pub p_wave_m_s: f64,
    pub s_wave_m_s: f64,
    pub thermal_diffusivity_m2_s: f64,
    pub molecular_diffusivity_m2_s: f64,
}

impl MediumParams {
    pub fn wire(self) -> [f64; 5] {
        [
            self.sound_speed_m_s,
            self.p_wave_m_s,
            self.s_wave_m_s,
            self.thermal_diffusivity_m2_s,
            self.molecular_diffusivity_m2_s,
        ]
    }
}

static MEDIA_PARAMS: &str = include_str!("media_params.tsv");

pub fn medium_params_of(body_name: &str) -> Option<MediumParams> {
    for line in MEDIA_PARAMS.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        let Some(name) = it.next() else {
            continue;
        };
        if name != body_name {
            continue;
        }
        let vals: Vec<f64> = it.filter_map(|s| s.parse::<f64>().ok()).collect();
        if vals.len() != 5 {
            return None;
        }
        return Some(MediumParams {
            sound_speed_m_s: vals[0],
            p_wave_m_s: vals[1],
            s_wave_m_s: vals[2],
            thermal_diffusivity_m2_s: vals[3],
            molecular_diffusivity_m2_s: vals[4],
        });
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_every_data_row_parses() {
        for line in super::MEDIA_PARAMS.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let name = line.split_whitespace().next().expect("a body name");
            assert!(super::medium_params_of(name).is_some());
        }
    }

    #[test]
    fn test_probe_absent() {
        assert!(super::medium_params_of("iss").is_none());
        assert!(super::medium_params_of("voyager1").is_none());
    }

    #[test]
    fn test_first_row_wire_order() {
        let line = super::MEDIA_PARAMS
            .lines()
            .find(|l| !l.starts_with('#') && !l.trim().is_empty())
            .expect("a first row");
        let name = line.split_whitespace().next().expect("a body name");
        let m = super::medium_params_of(name).expect("row present");
        assert_eq!(
            m.wire(),
            [
                m.sound_speed_m_s,
                m.p_wave_m_s,
                m.s_wave_m_s,
                m.thermal_diffusivity_m2_s,
                m.molecular_diffusivity_m2_s,
            ]
        );
    }
}
