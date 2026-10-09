use std::collections::HashMap;

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
    Maxwell = 4,
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
            TransportOp::Maxwell => h = fnv1a(&[4u8], h),
        }
        h = fnv1a(&[self.medium as u8], h);
        fnv1a(&[self.boundary as u8], h)
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
        TransportOp::Flux(FluxKind::Fick) => {
            conserved == Conserved::Mass && medium != Medium::Vacuum
        }
        TransportOp::Flux(FluxKind::Fourier) => {
            conserved == Conserved::Energy && medium != Medium::Vacuum
        }
        TransportOp::Flux(FluxKind::Ohm) => {
            conserved == Conserved::Charge && medium == Medium::Fluid
        }
        TransportOp::Flux(FluxKind::NewtonViscous) => {
            conserved == Conserved::Momentum && medium == Medium::Fluid
        }
        TransportOp::Advective => medium == Medium::Fluid,
        TransportOp::Wave => matches!(
            (conserved, medium),
            (Conserved::Momentum | Conserved::Energy, Medium::Fluid)
                | (
                    Conserved::Momentum | Conserved::Energy,
                    Medium::ElasticSolid
                )
        ),
        TransportOp::Maxwell => conserved == Conserved::Energy || conserved == Conserved::Momentum,
        TransportOp::Poisson => matches!(conserved, Conserved::Mass | Conserved::Charge),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TriState {
    Absent = 0,
    Pending = 1,
    Present = 2,
}

pub struct ChannelLatch {
    pub state: TriState,
    pub misses: u32,
    pub hysteresis: u32,
}

impl ChannelLatch {
    pub fn new(hysteresis: u32) -> Self {
        Self {
            state: TriState::Absent,
            misses: 0,
            hysteresis,
        }
    }

    pub fn observe(&mut self, value_present: bool) -> TriState {
        if value_present {
            self.state = TriState::Present;
            self.misses = 0;
        } else if self.state == TriState::Present {
            self.misses += 1;
            if self.misses > self.hysteresis {
                self.state = TriState::Absent;
            }
        }
        self.state
    }

    pub fn mark_pending(&mut self) {
        self.state = TriState::Pending;
        self.misses = 0;
    }
}

pub struct ChannelRegistry {
    cap: usize,
    descs: Vec<ChannelDescriptor>,
    id: HashMap<u64, usize>,
    aliases: HashMap<&'static str, u64>,
}

impl ChannelRegistry {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            cap,
            descs: Vec::new(),
            id: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.descs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.descs.is_empty()
    }

    pub fn register(&mut self, d: ChannelDescriptor) -> usize {
        let h = d.hash();
        if let Some(&i) = self.id.get(&h) {
            return i;
        }
        let i = self.descs.len();
        self.descs.push(d);
        self.id.insert(h, i);
        i
    }

    pub fn alias(&mut self, name: &'static str, target: &ChannelDescriptor) {
        self.aliases.insert(name, target.hash());
    }

    pub fn resolve(&self, name: &str) -> Option<&ChannelDescriptor> {
        let h = self.aliases.get(name)?;
        let &i = self.id.get(h)?;
        Some(&self.descs[i])
    }

    pub fn descriptor(&self, hash: u64) -> Option<&ChannelDescriptor> {
        self.id.get(&hash).map(|&i| &self.descs[i])
    }
}

#[derive(Clone, Copy)]
pub struct ChannelSum {
    pub weighted: f64,
    pub weight: f64,
    pub active: u32,
}

impl ChannelSum {
    pub const EMPTY: Self = Self {
        weighted: 0.0,
        weight: 0.0,
        active: 0,
    };

    pub fn absorb(&mut self, value: f32, weight: f32, state: TriState) {
        if state != TriState::Present {
            return;
        }
        self.weighted += value as f64 * weight as f64;
        self.weight += weight as f64;
        self.active += 1;
    }

    pub fn combine(self, other: Self) -> Self {
        Self {
            weighted: self.weighted + other.weighted,
            weight: self.weight + other.weight,
            active: self.active + other.active,
        }
    }

    pub fn mean(self) -> Option<f32> {
        if self.active == 0 || self.weight == 0.0 {
            return None;
        }
        Some((self.weighted / self.weight) as f32)
    }
}

pub fn channel_reduce(phi: &[f32], weight: &[f32], state: &[TriState]) -> Option<f32> {
    let n = phi.len().min(weight.len()).min(state.len());
    let mut acc = ChannelSum::EMPTY;
    for i in 0..n {
        acc.absorb(phi[i], weight[i], state[i]);
    }
    acc.mean()
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

    fn heat() -> ChannelDescriptor {
        ChannelDescriptor::new(
            Conserved::Energy,
            TransportOp::Flux(FluxKind::Fourier),
            Medium::Fluid,
            Boundary::None,
            "J / (m2 s)",
        )
    }

    #[test]
    fn identical_descriptors_dedupe_to_one_instance() {
        let mut reg = ChannelRegistry::with_capacity(16);
        let d = heat();
        let a = reg.register(d);
        let b = reg.register(d);
        assert_eq!(a, b);
        assert_eq!(reg.len(), 1);
        assert_eq!(reg.descriptor(d.hash()).map(|x| x.hash()), Some(d.hash()));
    }

    #[test]
    fn distinct_conserved_quantities_stay_distinct() {
        let mut reg = ChannelRegistry::with_capacity(16);
        reg.register(heat());
        reg.register(ChannelDescriptor::new(
            Conserved::Mass,
            TransportOp::Flux(FluxKind::Fick),
            Medium::Fluid,
            Boundary::None,
            "kg / (m2 s)",
        ));
        assert_eq!(reg.len(), 2);
    }

    #[test]
    fn an_alias_resolves_to_the_registered_descriptor() {
        let mut reg = ChannelRegistry::with_capacity(16);
        let em = ChannelDescriptor::new(
            Conserved::Charge,
            TransportOp::Flux(FluxKind::Ohm),
            Medium::Fluid,
            Boundary::None,
            "A / m2",
        );
        let idx = reg.register(em);
        reg.alias("electric", &em);
        assert_eq!(reg.resolve("electric").map(|d| d.hash()), Some(em.hash()));
        assert!(reg.id.get(&em.hash()) == Some(&idx));
        assert!(reg.resolve("unknown").is_none());
    }

    #[test]
    fn the_unit_follows_from_quantity_and_operator_not_identity() {
        let a = ChannelDescriptor::new(
            Conserved::Energy,
            TransportOp::Flux(FluxKind::Fourier),
            Medium::Fluid,
            Boundary::None,
            "J / (m2 s)",
        );
        let b = ChannelDescriptor::new(
            Conserved::Energy,
            TransportOp::Flux(FluxKind::Fourier),
            Medium::Fluid,
            Boundary::None,
            "W / m2",
        );
        assert_eq!(a.hash(), b.hash(), "the unit is not an identity axis");
    }

    #[test]
    fn a_flux_operator_needs_a_constitutive_medium() {
        assert!(!is_admissible(
            Conserved::Energy,
            TransportOp::Flux(FluxKind::Fourier),
            Medium::Vacuum
        ));
        assert!(!is_admissible(
            Conserved::Charge,
            TransportOp::Flux(FluxKind::Ohm),
            Medium::Vacuum
        ));
        assert!(!is_admissible(
            Conserved::Momentum,
            TransportOp::Flux(FluxKind::NewtonViscous),
            Medium::Vacuum
        ));
    }

    #[test]
    fn mass_wave_is_a_carried_riss() {
        assert!(!is_admissible(
            Conserved::Mass,
            TransportOp::Wave,
            Medium::Fluid
        ));
    }

    #[test]
    fn em_is_its_own_operator_in_vacuum_and_matter() {
        assert!(is_admissible(
            Conserved::Energy,
            TransportOp::Maxwell,
            Medium::Vacuum
        ));
        assert!(is_admissible(
            Conserved::Momentum,
            TransportOp::Maxwell,
            Medium::ElasticSolid
        ));
        assert!(!is_admissible(
            Conserved::Charge,
            TransportOp::Maxwell,
            Medium::Vacuum
        ));
    }

    #[test]
    fn constraint_holds_in_every_medium() {
        assert!(is_admissible(
            Conserved::Mass,
            TransportOp::Poisson,
            Medium::ElasticSolid
        ));
        assert!(is_admissible(
            Conserved::Charge,
            TransportOp::Poisson,
            Medium::ElasticSolid
        ));
    }

    #[test]
    fn the_reduction_divides_by_the_active_weight() {
        let phi = [2.0, 4.0, 6.0];
        let w = [1.0, 1.0, 1.0];
        let all = [TriState::Present; 3];
        assert_eq!(channel_reduce(&phi, &w, &all), Some(4.0));
        let w = [1.0, 9.0, 3.0];
        let active = [TriState::Present, TriState::Absent, TriState::Present];
        assert_eq!(channel_reduce(&phi, &w, &active), Some(5.0));
    }

    #[test]
    fn pending_and_absent_stand_in_neither_sum() {
        let phi = [10.0, 20.0, 30.0];
        let w = [1.0, 1.0, 1.0];
        let state = [TriState::Present, TriState::Pending, TriState::Absent];
        assert_eq!(channel_reduce(&phi, &w, &state), Some(10.0));
    }

    #[test]
    fn no_active_channel_has_no_mean() {
        let phi = [1.0, 2.0];
        let w = [1.0, 1.0];
        let none = [TriState::Absent, TriState::Pending];
        assert_eq!(channel_reduce(&phi, &w, &none), None);
    }

    #[test]
    fn the_monoid_combines_associatively() {
        let mut a = ChannelSum::EMPTY;
        a.absorb(2.0, 1.0, TriState::Present);
        let mut b = ChannelSum::EMPTY;
        b.absorb(4.0, 3.0, TriState::Present);
        let mut c = ChannelSum::EMPTY;
        c.absorb(100.0, 1.0, TriState::Absent);
        assert_eq!(a.combine(b).mean(), Some(3.5));
        assert_eq!(a.combine(b).combine(c).mean(), Some(3.5));
    }

    #[test]
    fn the_latch_holds_present_through_hysteresis() {
        let mut latch = ChannelLatch::new(2);
        assert_eq!(latch.observe(true), TriState::Present);
        assert_eq!(latch.observe(false), TriState::Present);
        assert_eq!(latch.observe(false), TriState::Present);
        assert_eq!(latch.observe(false), TriState::Absent);
    }

    #[test]
    fn pending_is_a_register_duty_never_a_miss() {
        let mut latch = ChannelLatch::new(0);
        latch.observe(true);
        latch.mark_pending();
        assert_eq!(latch.state, TriState::Pending);
        assert_eq!(latch.observe(false), TriState::Pending);
    }
}
