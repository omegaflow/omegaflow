#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub area_m2: f64,
    pub normal: [f64; 3],
    pub absorption: f64,
    pub diffuse_ratio: f64,
}

pub const SPEED_OF_LIGHT_M_S: f64 = 299_792_458.0;

pub fn force_n(faces: &[Face], sun_direction: [f64; 3], flux_w_m2: f64) -> Option<[f64; 3]> {
    let norm = (sun_direction[0] * sun_direction[0]
        + sun_direction[1] * sun_direction[1]
        + sun_direction[2] * sun_direction[2])
        .sqrt();
    if !norm.is_finite() || norm <= 0.0 || !flux_w_m2.is_finite() || flux_w_m2 <= 0.0 {
        return None;
    }
    let s = [
        sun_direction[0] / norm,
        sun_direction[1] / norm,
        sun_direction[2] / norm,
    ];
    let pressure = flux_w_m2 / SPEED_OF_LIGHT_M_S;
    let mut total = [0.0; 3];
    for face in faces {
        let n_norm = (face.normal[0] * face.normal[0]
            + face.normal[1] * face.normal[1]
            + face.normal[2] * face.normal[2])
            .sqrt();
        if !n_norm.is_finite() || n_norm <= 0.0 {
            return None;
        }
        let n = [
            face.normal[0] / n_norm,
            face.normal[1] / n_norm,
            face.normal[2] / n_norm,
        ];
        let cosine = s[0] * n[0] + s[1] * n[1] + s[2] * n[2];
        if cosine <= 0.0 {
            continue;
        }
        let reflectivity = 1.0 - face.absorption;
        let diffuse = face.diffuse_ratio * reflectivity;
        let specular = reflectivity - diffuse;
        let weight = pressure * face.area_m2 * cosine;
        for axis in 0..3 {
            total[axis] += weight
                * ((1.0 - specular) * s[axis]
                    + (2.0 * specular * cosine + 2.0 / 3.0 * diffuse) * n[axis]);
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn face(area_m2: f64, normal: [f64; 3], absorption: f64, diffuse_ratio: f64) -> Face {
        Face {
            area_m2,
            normal,
            absorption,
            diffuse_ratio,
        }
    }

    #[test]
    fn a_black_face_pushes_along_the_sun_direction() {
        let faces = [face(1.0, [0.0, 0.0, 1.0], 1.0, 0.0)];
        let force = force_n(&faces, [0.0, 0.0, 1.0], 1361.0).expect("a force");
        let expected = 1361.0 / SPEED_OF_LIGHT_M_S;
        assert!((force[0]).abs() < 1e-18);
        assert!((force[1]).abs() < 1e-18);
        assert!((force[2] - expected).abs() < 1e-18);
    }

    #[test]
    fn a_mirror_doubles_the_normal_push() {
        let faces = [face(1.0, [0.0, 0.0, 1.0], 0.0, 0.0)];
        let force = force_n(&faces, [0.0, 0.0, 1.0], 1361.0).expect("a force");
        let expected = 2.0 * 1361.0 / SPEED_OF_LIGHT_M_S;
        assert!((force[2] - expected).abs() < 1e-18);
    }

    #[test]
    fn a_face_turned_away_carries_no_force() {
        let faces = [face(1.0, [0.0, 0.0, -1.0], 0.0, 0.0)];
        let force = force_n(&faces, [0.0, 0.0, 1.0], 1361.0).expect("a force");
        assert_eq!(force, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn an_oblique_face_takes_the_cosine() {
        let faces = [face(1.0, [1.0, 0.0, 0.0], 1.0, 0.0)];
        let force = force_n(&faces, [1.0, 0.0, 0.0], 1361.0).expect("a force");
        let expected = 1361.0 / SPEED_OF_LIGHT_M_S;
        assert!((force[0] - expected).abs() < 1e-18);
        let tilted = force_n(
            &faces,
            [
                std::f64::consts::FRAC_1_SQRT_2,
                0.0,
                std::f64::consts::FRAC_1_SQRT_2,
            ],
            1361.0,
        )
        .expect("a force");
        assert!((tilted[0] - expected * std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-18);
    }

    #[test]
    fn a_diffuse_face_takes_two_thirds_of_its_reflection_on_the_normal() {
        let faces = [face(1.0, [0.0, 0.0, 1.0], 0.0, 1.0)];
        let force = force_n(&faces, [0.0, 0.0, 1.0], 1361.0).expect("a force");
        let expected = 1361.0 / SPEED_OF_LIGHT_M_S * (1.0 + 2.0 / 3.0);
        assert!((force[2] - expected).abs() < 1e-18);
    }

    #[test]
    fn a_flux_of_zero_is_absent_not_zero_force() {
        let faces = [face(1.0, [0.0, 0.0, 1.0], 1.0, 0.0)];
        assert!(force_n(&faces, [0.0, 0.0, 1.0], 0.0).is_none());
        assert!(force_n(&faces, [0.0, 0.0, 0.0], 1361.0).is_none());
    }
}
