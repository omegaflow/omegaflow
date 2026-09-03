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

pub fn medium_params_of(body_name: &str) -> Option<MediumParams> {
    let m = |sound: f64, p: f64, s: f64, ath: f64, dd: f64| MediumParams {
        sound_speed_m_s: sound,
        p_wave_m_s: p,
        s_wave_m_s: s,
        thermal_diffusivity_m2_s: ath,
        molecular_diffusivity_m2_s: dd,
    };
    match body_name {
        "sun" => Some(m(11557.5, 0.0, 0.0, 0.0, 0.0)),
        "mercury" => Some(m(515.4, 5200.0, 3000.0, 0.0, 0.0)),
        "venus" => Some(m(433.1, 6200.0, 3600.0, 7.92e-7, 7.27e-7)),
        "earth" => Some(m(340.2, 5950.0, 3630.0, 2.18e-5, 2.00e-5)),
        "moon" => Some(m(355.3, 5000.0, 2900.0, 0.0, 0.0)),
        "mars" => Some(m(231.5, 5400.0, 3200.0, 1.74e-3, 1.60e-3)),
        "jupiter" => Some(m(933.5, 0.0, 0.0, 3.42e-5, 3.13e-5)),
        "saturn" => Some(m(871.2, 0.0, 0.0, 2.59e-5, 2.37e-5)),
        "uranus" => Some(m(580.9, 0.0, 0.0, 9.79e-6, 8.98e-6)),
        "neptune" => Some(m(568.7, 0.0, 0.0, 9.08e-6, 8.33e-6)),
        "pluto" => Some(m(125.7, 0.0, 0.0, 8.18e-2, 7.50e-2)),
        "io" => Some(m(137.8, 5800.0, 3400.0, 0.0, 0.0)),
        "europa" => Some(m(208.3, 3800.0, 1900.0, 0.0, 0.0)),
        "ganymede" => Some(m(218.5, 3800.0, 1900.0, 0.0, 0.0)),
        "callisto" => Some(m(228.2, 3800.0, 1900.0, 0.0, 0.0)),
        "titan" => Some(m(197.7, 3800.0, 1900.0, 2.85e-6, 2.62e-6)),
        "triton" => Some(m(125.7, 2800.0, 1400.0, 7.59e-2, 6.96e-2)),
        "enceladus" => Some(m(0.0, 3800.0, 1900.0, 0.0, 0.0)),
        "rhea" => Some(m(0.0, 3800.0, 1900.0, 0.0, 0.0)),
        "dione" => Some(m(0.0, 3800.0, 1900.0, 0.0, 0.0)),
        "tethys" => Some(m(0.0, 3800.0, 1900.0, 0.0, 0.0)),
        "phobos" => Some(m(0.0, 3200.0, 1800.0, 0.0, 0.0)),
        "deimos" => Some(m(0.0, 3200.0, 1800.0, 0.0, 0.0)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_earth_row() {
        let m = super::medium_params_of("earth").expect("earth row present");
        assert_eq!(m.sound_speed_m_s, 340.2);
        assert_eq!(m.p_wave_m_s, 5950.0);
        assert_eq!(m.s_wave_m_s, 3630.0);
        assert_eq!(m.thermal_diffusivity_m2_s, 2.18e-5);
        assert_eq!(m.molecular_diffusivity_m2_s, 2.00e-5);
    }

    #[test]
    fn test_sun_row_gas_only() {
        let m = super::medium_params_of("sun").expect("sun row present");
        assert_eq!(m.sound_speed_m_s, 11557.5);
        assert_eq!(m.p_wave_m_s, 0.0);
        assert_eq!(m.s_wave_m_s, 0.0);
    }

    #[test]
    fn test_enceladus_sound_absent_null_echt() {
        let m = super::medium_params_of("enceladus").expect("enceladus row present");
        assert_eq!(m.sound_speed_m_s, 0.0);
        assert_eq!(m.p_wave_m_s, 3800.0);
    }

    #[test]
    fn test_probe_absent() {
        assert!(super::medium_params_of("iss").is_none());
        assert!(super::medium_params_of("voyager1").is_none());
    }

    #[test]
    fn test_wire_order_contract() {
        let m = super::medium_params_of("earth").expect("earth row present");
        let w = m.wire();
        assert_eq!(w, [340.2, 5950.0, 3630.0, 2.18e-5, 2.00e-5]);
    }
}
