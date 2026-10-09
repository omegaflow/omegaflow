#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Conserved {
    Mass = 0,
    Momentum = 1,
    Energy = 2,
    Charge = 3,
}

pub fn conserved_name(c: Conserved) -> &'static str {
    match c {
        Conserved::Mass => "mass",
        Conserved::Momentum => "momentum",
        Conserved::Energy => "energy",
        Conserved::Charge => "charge",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FluxKind {
    Fick = 0,
    Fourier = 1,
    Ohm = 2,
    NewtonViscous = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TransportOp {
    Flux(FluxKind) = 0,
    Advective = 1,
    Wave = 2,
    Poisson = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Medium {
    Vacuum = 0,
    Fluid = 1,
    ElasticSolid = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Boundary {
    None = 0,
    FreeSurface = 1,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChannelDescriptor {
    pub conserved: Conserved,
    pub op: TransportOp,
    pub medium: Medium,
    pub boundary: Boundary,
    pub unit: &'static str,
}

impl ChannelDescriptor {
    pub fn new(
        conserved: Conserved,
        op: TransportOp,
        medium: Medium,
        boundary: Boundary,
        unit: &'static str,
    ) -> Self {
        Self {
            conserved,
            op,
            medium,
            boundary,
            unit,
        }
    }

    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        h = fnv1a(&[self.conserved as u8], h);
        match self.op {
            TransportOp::Flux(kind) => h = fnv1a(&[0u8, kind as u8], h),
            TransportOp::Advective => h = fnv1a(&[1u8], h),
            TransportOp::Wave => h = fnv1a(&[2u8], h),
            TransportOp::Poisson => h = fnv1a(&[3u8], h),
        }
        h = fnv1a(&[self.medium as u8], h);
        h = fnv1a(&[self.boundary as u8], h);
        fnv1a(self.unit.as_bytes(), h)
    }
}

const FNV_PRIME: u64 = 0x100000001b3;

fn fnv1a(bytes: &[u8], mut h: u64) -> u64 {
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

pub fn is_admissible(conserved: Conserved, op: TransportOp, medium: Medium) -> bool {
    match op {
        TransportOp::Flux(FluxKind::Fick) => conserved == Conserved::Mass,
        TransportOp::Flux(FluxKind::Fourier) => conserved == Conserved::Energy,
        TransportOp::Flux(FluxKind::Ohm) => conserved == Conserved::Charge,
        TransportOp::Flux(FluxKind::NewtonViscous) => conserved == Conserved::Momentum,
        TransportOp::Advective => medium == Medium::Fluid,
        TransportOp::Wave => {
            (conserved == Conserved::Momentum || conserved == Conserved::Energy)
                && (medium == Medium::Fluid || medium == Medium::ElasticSolid)
        }
        TransportOp::Poisson => {
            (conserved == Conserved::Mass || conserved == Conserved::Charge)
                && (medium == Medium::Vacuum || medium == Medium::Fluid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn differing_fields_differ_in_hash() {
        let a = ChannelDescriptor::new(
            Conserved::Mass,
            TransportOp::Flux(FluxKind::Fick),
            Medium::Fluid,
            Boundary::None,
            "kg",
        );
        let b = ChannelDescriptor::new(
            Conserved::Mass,
            TransportOp::Flux(FluxKind::Fick),
            Medium::Vacuum,
            Boundary::None,
            "kg",
        );
        assert_ne!(a.hash(), b.hash());
    }

    #[test]
    fn hash_is_stable() {
        let d = ChannelDescriptor::new(
            Conserved::Momentum,
            TransportOp::Wave,
            Medium::ElasticSolid,
            Boundary::FreeSurface,
            "kg m / s",
        );
        assert_eq!(d.hash(), d.hash());
    }

    #[test]
    fn mass_fourier_is_forbidden() {
        assert!(!is_admissible(
            Conserved::Mass,
            TransportOp::Flux(FluxKind::Fourier),
            Medium::Fluid
        ));
    }

    #[test]
    fn mass_fick_is_admissible() {
        assert!(is_admissible(
            Conserved::Mass,
            TransportOp::Flux(FluxKind::Fick),
            Medium::Fluid
        ));
    }

    #[test]
    fn energy_fourier_is_admissible() {
        assert!(is_admissible(
            Conserved::Energy,
            TransportOp::Flux(FluxKind::Fourier),
            Medium::Fluid
        ));
    }

    #[test]
    fn charge_ohm_is_admissible() {
        assert!(is_admissible(
            Conserved::Charge,
            TransportOp::Flux(FluxKind::Ohm),
            Medium::Fluid
        ));
    }

    #[test]
    fn momentum_wave_in_solid_is_admissible() {
        assert!(is_admissible(
            Conserved::Momentum,
            TransportOp::Wave,
            Medium::ElasticSolid
        ));
    }

    #[test]
    fn energy_wave_in_vacuum_is_forbidden() {
        assert!(!is_admissible(
            Conserved::Energy,
            TransportOp::Wave,
            Medium::Vacuum
        ));
    }
}
