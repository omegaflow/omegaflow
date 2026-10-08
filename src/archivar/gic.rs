pub const GIC_AURORAL_MIN: f64 = 60.0;
pub const GIC_SUBAURORAL_MIN: f64 = 50.0;
pub const GIC_FAMILY_NAMES: [&str; 3] = ["auroral", "sub-auroral", "mid-latitude"];

pub fn family_of(abs_cgm: f64) -> &'static str {
    if abs_cgm >= GIC_AURORAL_MIN {
        GIC_FAMILY_NAMES[0]
    } else if abs_cgm >= GIC_SUBAURORAL_MIN {
        GIC_FAMILY_NAMES[1]
    } else {
        GIC_FAMILY_NAMES[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_of_holds_the_declared_boundaries() {
        assert_eq!(family_of(60.0), "auroral");
        assert_eq!(family_of(59.99), "sub-auroral");
        assert_eq!(family_of(50.0), "sub-auroral");
        assert_eq!(family_of(49.99), "mid-latitude");
        assert_eq!(family_of(0.0), "mid-latitude");
    }
}
