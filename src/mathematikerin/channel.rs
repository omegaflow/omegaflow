use super::actuators::CHANNEL_CAP;
use super::force::force_name_of;
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
#[repr(u8)]
pub enum QuantityRole {
    Primary = 0,
    Derived = 1,
    Geometry = 2,
    SourceParameter = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PdeType {
    Elliptic = 0,
    Parabolic = 1,
    Hyperbolic = 2,
    Advective = 3,
    Mixed = 4,
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
    Dirichlet = 2,
    Neumann = 3,
    Robin = 4,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Domain {
    Unspecified = 0,
    Line = 1,
    Rectangle = 2,
    Circle = 3,
    Sphere = 4,
}

impl Conserved {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "mass" => Some(Conserved::Mass),
            "momentum" => Some(Conserved::Momentum),
            "energy" => Some(Conserved::Energy),
            "charge" => Some(Conserved::Charge),
            _ => None,
        }
    }
}

impl QuantityRole {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "primary" => Some(QuantityRole::Primary),
            "derived" => Some(QuantityRole::Derived),
            "geometry" => Some(QuantityRole::Geometry),
            "source-parameter" => Some(QuantityRole::SourceParameter),
            _ => None,
        }
    }
}

impl PdeType {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "elliptic" => Some(PdeType::Elliptic),
            "parabolic" => Some(PdeType::Parabolic),
            "hyperbolic" => Some(PdeType::Hyperbolic),
            "advective" => Some(PdeType::Advective),
            "mixed" => Some(PdeType::Mixed),
            _ => None,
        }
    }
}

impl TransportOp {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "flux-fick" => Some(TransportOp::Flux(FluxKind::Fick)),
            "flux-fourier" => Some(TransportOp::Flux(FluxKind::Fourier)),
            "flux-ohm" => Some(TransportOp::Flux(FluxKind::Ohm)),
            "flux-newton-viscous" => Some(TransportOp::Flux(FluxKind::NewtonViscous)),
            "advective" => Some(TransportOp::Advective),
            "wave" => Some(TransportOp::Wave),
            "poisson" => Some(TransportOp::Poisson),
            "maxwell" => Some(TransportOp::Maxwell),
            _ => None,
        }
    }
}

impl Medium {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "vacuum" => Some(Medium::Vacuum),
            "fluid" => Some(Medium::Fluid),
            "elastic-solid" => Some(Medium::ElasticSolid),
            _ => None,
        }
    }
}

impl Domain {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "unspecified" => Some(Domain::Unspecified),
            "line" => Some(Domain::Line),
            "rectangle" => Some(Domain::Rectangle),
            "circle" => Some(Domain::Circle),
            "sphere" => Some(Domain::Sphere),
            _ => None,
        }
    }
}

impl Boundary {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "none" => Some(Boundary::None),
            "free-surface" => Some(Boundary::FreeSurface),
            "dirichlet" => Some(Boundary::Dirichlet),
            "neumann" => Some(Boundary::Neumann),
            "robin" => Some(Boundary::Robin),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChannelDescriptor {
    pub conserved: Conserved,
    pub role: QuantityRole,
    pub op: TransportOp,
    pub pde_type: PdeType,
    pub medium: Medium,
    pub domain: Domain,
    pub boundary: Boundary,
    pub unit: &'static str,
}

impl ChannelDescriptor {
    pub fn new(
        conserved: Conserved,
        role: QuantityRole,
        op: TransportOp,
        pde_type: PdeType,
        medium: Medium,
        domain: Domain,
        boundary: Boundary,
        unit: &'static str,
    ) -> Self {
        Self {
            conserved,
            role,
            op,
            pde_type,
            medium,
            domain,
            boundary,
            unit,
        }
    }

    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        h = fnv1a(&[self.conserved as u8], h);
        h = fnv1a(&[self.role as u8], h);
        match self.op {
            TransportOp::Flux(kind) => h = fnv1a(&[0u8, kind as u8], h),
            TransportOp::Advective => h = fnv1a(&[1u8], h),
            TransportOp::Wave => h = fnv1a(&[2u8], h),
            TransportOp::Poisson => h = fnv1a(&[3u8], h),
            TransportOp::Maxwell => h = fnv1a(&[4u8], h),
        }
        h = fnv1a(&[self.pde_type as u8], h);
        h = fnv1a(&[self.medium as u8], h);
        h = fnv1a(&[self.domain as u8], h);
        fnv1a(&[self.boundary as u8], h)
    }

    pub fn parse_spec(spec: &str, unit: &'static str) -> Result<Self, String> {
        let t: Vec<&str> = spec.split(':').collect();
        if t.len() != 7 {
            return Err(format!(
                "channel spec needs 7 axes conserved:role:op:pde_type:medium:domain:boundary, got {}",
                t.len()
            ));
        }
        let conserved =
            Conserved::parse(t[0]).ok_or_else(|| format!("unknown conserved \"{}\"", t[0]))?;
        let role = QuantityRole::parse(t[1]).ok_or_else(|| format!("unknown role \"{}\"", t[1]))?;
        let op = TransportOp::parse(t[2]).ok_or_else(|| format!("unknown op \"{}\"", t[2]))?;
        let pde_type =
            PdeType::parse(t[3]).ok_or_else(|| format!("unknown pde_type \"{}\"", t[3]))?;
        let medium = Medium::parse(t[4]).ok_or_else(|| format!("unknown medium \"{}\"", t[4]))?;
        let domain = Domain::parse(t[5]).ok_or_else(|| format!("unknown domain \"{}\"", t[5]))?;
        let boundary =
            Boundary::parse(t[6]).ok_or_else(|| format!("unknown boundary \"{}\"", t[6]))?;
        if !is_admissible(conserved, op, medium) {
            return Err(format!(
                "inadmissible channel {}/{}/{} is not in the admissibility relation",
                t[0], t[2], t[4]
            ));
        }
        Ok(ChannelDescriptor::new(
            conserved, role, op, pde_type, medium, domain, boundary, unit,
        ))
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

pub fn descriptor_for_force(name: &str, medium: Medium) -> Option<ChannelDescriptor> {
    let (conserved, op, pde_type, boundary, unit) = match name {
        "em" | "electric" => (
            Conserved::Energy,
            TransportOp::Maxwell,
            PdeType::Mixed,
            Boundary::None,
            "V/m",
        ),
        "gravity" => (
            Conserved::Mass,
            TransportOp::Poisson,
            PdeType::Elliptic,
            Boundary::None,
            "m/s^2",
        ),
        "acoustic" => (
            Conserved::Energy,
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Boundary::None,
            "Pa",
        ),
        "seismic-body" => (
            Conserved::Energy,
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Boundary::None,
            "Pa",
        ),
        "seismic-surface" => (
            Conserved::Energy,
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Boundary::FreeSurface,
            "Pa",
        ),
        "thermal" => (
            Conserved::Energy,
            TransportOp::Flux(FluxKind::Fourier),
            PdeType::Parabolic,
            Boundary::None,
            "K",
        ),
        "diffusion" => (
            Conserved::Mass,
            TransportOp::Flux(FluxKind::Fick),
            PdeType::Parabolic,
            Boundary::None,
            "kg/m^3",
        ),
        "advective" => (
            Conserved::Mass,
            TransportOp::Advective,
            PdeType::Advective,
            Boundary::None,
            "kg/(m^2 s)",
        ),
        _ => return None,
    };
    if !is_admissible(conserved, op, medium) {
        return None;
    }
    Some(ChannelDescriptor::new(
        conserved,
        QuantityRole::Primary,
        op,
        pde_type,
        medium,
        Domain::Unspecified,
        boundary,
        unit,
    ))
}

const LIVE_FORCE_MEDIA: [Medium; 9] = [
    Medium::Vacuum,
    Medium::Vacuum,
    Medium::Fluid,
    Medium::ElasticSolid,
    Medium::ElasticSolid,
    Medium::Fluid,
    Medium::Fluid,
    Medium::Fluid,
    Medium::Vacuum,
];

pub fn descriptor_for_force_type(ft: u8) -> Option<ChannelDescriptor> {
    let name = force_name_of(ft)?;
    let medium = *LIVE_FORCE_MEDIA.get(ft as usize)?;
    descriptor_for_force(name, medium)
}

pub fn live_channel_registry() -> ChannelRegistry {
    let mut reg = ChannelRegistry::with_capacity(CHANNEL_CAP);
    for ft in 0..9u8 {
        if let Some(d) = descriptor_for_force_type(ft) {
            reg.register(d);
        }
    }
    reg
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModeRegime {
    Propagating = 0,
    Advective = 1,
    Diffusive = 2,
    Constraint = 3,
}

pub fn mode_regime(op: TransportOp) -> ModeRegime {
    match op {
        TransportOp::Wave | TransportOp::Maxwell => ModeRegime::Propagating,
        TransportOp::Advective => ModeRegime::Advective,
        TransportOp::Flux(_) => ModeRegime::Diffusive,
        TransportOp::Poisson => ModeRegime::Constraint,
    }
}

#[cfg(test)]
const CIRCLE_DRUM_MODE_RATIOS: [f64; 9] = [
    1.0, 1.593_34, 2.135_36, 2.295_49, 2.653_07, 2.917_28, 3.155_46, 3.500_11, 3.598_23,
];

const FIRST_BESSEL_J0_ZERO: f64 = 2.404_825_557_7;

const CIRCLE_ZERO_SEARCH_MAX: f64 = 20.0;
const CIRCLE_ZERO_STEP: f64 = 0.05;

fn bessel_j(m: u32, x: f64) -> f64 {
    if x == 0.0 {
        return if m == 0 { 1.0 } else { 0.0 };
    }
    let half = 0.5 * x;
    let mut term = 1.0f64;
    for k in 1..=m {
        term *= half / k as f64;
    }
    let mut sum = term;
    let mut max_abs = term.abs();
    let mut k: u32 = 0;
    loop {
        k += 1;
        term *= -(half * half) / (k as f64 * (k as f64 + m as f64));
        sum += term;
        let a = term.abs();
        if a > max_abs {
            max_abs = a;
        }
        if a <= max_abs * 1e-17 || k >= 4096 {
            break;
        }
    }
    sum
}

fn bessel_zero(m: u32, lo: f64, hi: f64) -> f64 {
    let mut a = lo;
    let mut b = hi;
    let mut fa = bessel_j(m, a);
    for _ in 0..200 {
        if b - a <= 1e-13 {
            break;
        }
        let mid = 0.5 * (a + b);
        let fm = bessel_j(m, mid);
        if (fm < 0.0) != (fa < 0.0) {
            b = mid;
        } else {
            a = mid;
            fa = fm;
        }
    }
    0.5 * (a + b)
}

fn circular_membrane_zeros(count: usize) -> Vec<f64> {
    let mut pool: Vec<f64> = Vec::new();
    for m in 0..=(count as u32) {
        let mut prev_x = CIRCLE_ZERO_STEP;
        let mut prev = bessel_j(m, prev_x);
        let mut x = prev_x + CIRCLE_ZERO_STEP;
        while x <= CIRCLE_ZERO_SEARCH_MAX {
            let cur = bessel_j(m, x);
            if (cur < 0.0) != (prev < 0.0) {
                pool.push(bessel_zero(m, prev_x, x));
            }
            prev_x = x;
            prev = cur;
            x += CIRCLE_ZERO_STEP;
        }
    }
    pool.sort_by(|a, b| a.total_cmp(b));
    pool
}

pub fn eigen_wavenumbers(
    domain: Domain,
    boundary: Boundary,
    extent: f64,
    count: usize,
) -> Option<Vec<f64>> {
    if !extent.is_finite() || extent <= 0.0 || count == 0 {
        return None;
    }
    match (domain, boundary) {
        (Domain::Line, Boundary::Dirichlet) | (Domain::Line, Boundary::FreeSurface) => Some(
            (1..=count)
                .map(|j| j as f64 * std::f64::consts::PI / extent)
                .collect(),
        ),
        (Domain::Line, Boundary::Neumann) => Some(
            (0..count)
                .map(|j| j as f64 * std::f64::consts::PI / extent)
                .collect(),
        ),
        (Domain::Sphere, Boundary::Dirichlet) => Some(
            (1..=count)
                .map(|j| j as f64 * std::f64::consts::PI / extent)
                .collect(),
        ),
        (Domain::Circle, Boundary::Dirichlet) if count <= CHANNEL_CAP => {
            let zeros = circular_membrane_zeros(count);
            if zeros.len() < count || (zeros[0] - FIRST_BESSEL_J0_ZERO).abs() > 1e-9 {
                return None;
            }
            Some(zeros[..count].iter().map(|z| z / extent).collect())
        }
        _ => None,
    }
}

pub fn mode_frequency_hz(op: TransportOp, speed: f64, k: f64) -> Option<f64> {
    if !k.is_finite() {
        return None;
    }
    match mode_regime(op) {
        ModeRegime::Propagating => {
            if !speed.is_finite() || speed <= 0.0 {
                return None;
            }
            Some(speed * k / std::f64::consts::TAU)
        }
        ModeRegime::Advective | ModeRegime::Diffusive | ModeRegime::Constraint => None,
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

    pub fn schema_hash(&self) -> u32 {
        let mut h: u64 = 0xcbf29ce484222325;
        for d in &self.descs {
            for b in d.hash().to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(FNV_PRIME);
            }
        }
        h as u32
    }
}

pub fn live_schema_hash() -> u32 {
    live_channel_registry().schema_hash()
}

pub fn channel_registry_from_sources(sources: &[crate::archivar::SourceConfig]) -> ChannelRegistry {
    let mut reg = ChannelRegistry::with_capacity(CHANNEL_CAP);
    for s in sources {
        for d in &s.channels {
            reg.register(*d);
        }
    }
    if reg.is_empty() {
        return live_channel_registry();
    }
    reg
}

pub fn unit_token(token: &str) -> Option<&'static str> {
    match token {
        "V/m" => Some("V/m"),
        "m/s^2" => Some("m/s^2"),
        "Pa" => Some("Pa"),
        "K" => Some("K"),
        "kg/m^3" => Some("kg/m^3"),
        "kg/(m^2 s)" => Some("kg/(m^2 s)"),
        _ => None,
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
            QuantityRole::Primary,
            TransportOp::Flux(FluxKind::Fick),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
            Boundary::None,
            "kg",
        );
        let b = ChannelDescriptor::new(
            Conserved::Mass,
            QuantityRole::Primary,
            TransportOp::Flux(FluxKind::Fick),
            PdeType::Parabolic,
            Medium::Vacuum,
            Domain::Unspecified,
            Boundary::None,
            "kg",
        );
        assert_ne!(a.hash(), b.hash());
    }

    #[test]
    fn hash_is_stable() {
        let d = ChannelDescriptor::new(
            Conserved::Momentum,
            QuantityRole::Primary,
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Line,
            Boundary::FreeSurface,
            "kg m / s",
        );
        assert_eq!(d.hash(), d.hash());
    }

    #[test]
    fn domain_enters_the_hash() {
        let unspecified = ChannelDescriptor::new(
            Conserved::Momentum,
            QuantityRole::Primary,
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Unspecified,
            Boundary::None,
            "kg m / s",
        );
        let line = ChannelDescriptor::new(
            Conserved::Momentum,
            QuantityRole::Primary,
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Line,
            Boundary::None,
            "kg m / s",
        );
        assert_ne!(
            unspecified.hash(),
            line.hash(),
            "the domain axis must enter the hash"
        );

        let dirichlet_line = ChannelDescriptor::new(
            Conserved::Momentum,
            QuantityRole::Primary,
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Line,
            Boundary::Dirichlet,
            "kg m / s",
        );
        assert_ne!(dirichlet_line.hash(), unspecified.hash());
    }

    #[test]
    fn role_enters_the_hash() {
        let primary = ChannelDescriptor::new(
            Conserved::Mass,
            QuantityRole::Primary,
            TransportOp::Flux(FluxKind::Fick),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
            Boundary::None,
            "kg",
        );
        let geometry = ChannelDescriptor::new(
            Conserved::Mass,
            QuantityRole::Geometry,
            TransportOp::Flux(FluxKind::Fick),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
            Boundary::None,
            "kg",
        );
        assert_ne!(
            primary.hash(),
            geometry.hash(),
            "the role axis must enter the hash"
        );
    }

    #[test]
    fn pde_type_enters_the_hash() {
        let elliptic = ChannelDescriptor::new(
            Conserved::Energy,
            QuantityRole::Primary,
            TransportOp::Maxwell,
            PdeType::Elliptic,
            Medium::Vacuum,
            Domain::Unspecified,
            Boundary::None,
            "V/m",
        );
        let hyperbolic = ChannelDescriptor::new(
            Conserved::Energy,
            QuantityRole::Primary,
            TransportOp::Maxwell,
            PdeType::Hyperbolic,
            Medium::Vacuum,
            Domain::Unspecified,
            Boundary::None,
            "V/m",
        );
        assert_ne!(
            elliptic.hash(),
            hyperbolic.hash(),
            "Maxwell carries two regimes; the pde_type is identity"
        );
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
            QuantityRole::Primary,
            TransportOp::Flux(FluxKind::Fourier),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
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
            QuantityRole::Primary,
            TransportOp::Flux(FluxKind::Fick),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
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
            QuantityRole::Primary,
            TransportOp::Flux(FluxKind::Ohm),
            PdeType::Elliptic,
            Medium::Fluid,
            Domain::Unspecified,
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
            QuantityRole::Primary,
            TransportOp::Flux(FluxKind::Fourier),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
            Boundary::None,
            "J / (m2 s)",
        );
        let b = ChannelDescriptor::new(
            Conserved::Energy,
            QuantityRole::Primary,
            TransportOp::Flux(FluxKind::Fourier),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
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

    #[test]
    fn every_legacy_force_label_maps_onto_its_admissible_descriptor() {
        let declared = [
            ("em", Medium::Vacuum),
            ("gravity", Medium::Vacuum),
            ("acoustic", Medium::Fluid),
            ("seismic-body", Medium::ElasticSolid),
            ("seismic-surface", Medium::ElasticSolid),
            ("thermal", Medium::Fluid),
            ("diffusion", Medium::Fluid),
            ("advective", Medium::Fluid),
            ("electric", Medium::Vacuum),
        ];
        for (name, medium) in declared {
            let d = descriptor_for_force(name, medium).unwrap_or_else(|| panic!("{name} must map"));
            assert!(
                is_admissible(d.conserved, d.op, medium),
                "{name} must be admissible in {medium:?}"
            );
        }
    }

    #[test]
    fn the_bridge_refuses_a_medium_the_relation_forbids() {
        assert!(descriptor_for_force("acoustic", Medium::Vacuum).is_none());
        assert!(descriptor_for_force("thermal", Medium::Vacuum).is_none());
        assert!(descriptor_for_force("diffusion", Medium::Vacuum).is_none());
    }

    #[test]
    fn electric_is_the_em_alias() {
        let em = descriptor_for_force("em", Medium::Vacuum).expect("em");
        let electric = descriptor_for_force("electric", Medium::Vacuum).expect("electric");
        assert_eq!(em.hash(), electric.hash());
    }

    #[test]
    fn an_unknown_label_is_refused() {
        assert!(descriptor_for_force("phlogiston", Medium::Fluid).is_none());
    }

    #[test]
    fn a_line_is_harmonic() {
        let k = eigen_wavenumbers(Domain::Line, Boundary::Dirichlet, 1.0, 3).expect("line modes");
        let pi = std::f64::consts::PI;
        assert_eq!(k, vec![pi, 2.0 * pi, 3.0 * pi]);
    }

    #[test]
    fn a_free_surface_line_is_harmonic() {
        let dirichlet = eigen_wavenumbers(Domain::Line, Boundary::Dirichlet, 2.0, 2).expect("dir");
        let free = eigen_wavenumbers(Domain::Line, Boundary::FreeSurface, 2.0, 2).expect("free");
        assert_eq!(dirichlet, free);
    }

    #[test]
    fn a_neumann_line_keeps_the_constant_mode() {
        let k = eigen_wavenumbers(Domain::Line, Boundary::Neumann, 1.0, 3).expect("neumann");
        assert_eq!(k[0], 0.0);
        assert!(k[1] > 0.0);
    }

    #[test]
    fn a_circle_is_the_drum_spectrum() {
        let k = eigen_wavenumbers(Domain::Circle, Boundary::Dirichlet, 1.0, 3).expect("circle");
        assert!((k[0] - FIRST_BESSEL_J0_ZERO).abs() < 1e-9);
        assert!((k[1] - 3.831_706).abs() < 1e-4);
        assert!((k[2] - 5.135_622).abs() < 1e-4);
    }

    #[test]
    fn a_circle_keeps_the_documented_mode_ratios_through_nine() {
        let extent = FIRST_BESSEL_J0_ZERO;
        let k = eigen_wavenumbers(Domain::Circle, Boundary::Dirichlet, extent, 9)
            .expect("circle modes");
        assert_eq!(k.len(), 9);
        assert!((k[0] - 1.0).abs() < 1e-9);
        for (j, &ratio) in CIRCLE_DRUM_MODE_RATIOS.iter().enumerate() {
            let measured = k[j] / k[0];
            assert!(
                (measured - ratio).abs() < 1e-3,
                "mode {j}: measured ratio {measured} vs documented {ratio}"
            );
        }
    }

    #[test]
    fn a_circle_carries_the_full_channel_capacity() {
        let k = eigen_wavenumbers(Domain::Circle, Boundary::Dirichlet, 1.0, CHANNEL_CAP)
            .expect("16 circle modes");
        assert_eq!(k.len(), CHANNEL_CAP);
        for j in 1..k.len() {
            assert!(k[j] > k[j - 1], "mode {j} must exceed mode {}", j - 1);
        }
    }

    #[test]
    fn a_circle_with_zero_count_has_no_mode() {
        assert!(eigen_wavenumbers(Domain::Circle, Boundary::Dirichlet, 1.0, 0).is_none());
    }

    #[test]
    fn a_domain_without_a_closed_mode_is_absent() {
        assert!(eigen_wavenumbers(Domain::Unspecified, Boundary::None, 1.0, 3).is_none());
        assert!(eigen_wavenumbers(Domain::Rectangle, Boundary::Dirichlet, 1.0, 3).is_none());
        assert!(eigen_wavenumbers(Domain::Line, Boundary::Robin, 1.0, 3).is_none());
        assert!(
            eigen_wavenumbers(Domain::Circle, Boundary::Dirichlet, 1.0, CHANNEL_CAP + 1).is_none()
        );
    }

    #[test]
    fn a_non_positive_extent_has_no_mode() {
        assert!(eigen_wavenumbers(Domain::Line, Boundary::Dirichlet, 0.0, 3).is_none());
        assert!(eigen_wavenumbers(Domain::Line, Boundary::Dirichlet, -1.0, 3).is_none());
        assert!(eigen_wavenumbers(Domain::Line, Boundary::Dirichlet, f64::NAN, 3).is_none());
    }

    #[test]
    fn a_wave_carries_a_pitch() {
        let pi = std::f64::consts::PI;
        let f = mode_frequency_hz(TransportOp::Wave, 343.0, 2.0 * pi).expect("wave tone");
        assert!((f - 343.0).abs() < 1e-9);
    }

    #[test]
    fn an_elliptic_or_diffusive_mode_carries_no_pitch() {
        assert!(mode_frequency_hz(TransportOp::Poisson, 1.0, 1.0).is_none());
        assert!(mode_frequency_hz(TransportOp::Flux(FluxKind::Fourier), 1.0, 1.0).is_none());
        assert!(mode_frequency_hz(TransportOp::Advective, 1.0, 1.0).is_none());
    }

    #[test]
    fn a_wave_without_a_speed_carries_no_pitch() {
        assert!(mode_frequency_hz(TransportOp::Wave, 0.0, 1.0).is_none());
        assert!(mode_frequency_hz(TransportOp::Wave, f64::NAN, 1.0).is_none());
    }

    #[test]
    fn a_parsed_channel_spec_meets_the_forced_descriptor() {
        let gravity = ChannelDescriptor::parse_spec(
            "mass:primary:poisson:elliptic:vacuum:unspecified:none",
            "m/s^2",
        )
        .expect("gravity spec parses");
        assert_eq!(
            gravity.hash(),
            descriptor_for_force("gravity", Medium::Vacuum)
                .expect("gravity descriptor")
                .hash()
        );

        let em = ChannelDescriptor::parse_spec(
            "energy:primary:maxwell:mixed:vacuum:unspecified:none",
            "V/m",
        )
        .expect("em spec parses");
        assert_eq!(
            em.hash(),
            descriptor_for_force("em", Medium::Vacuum)
                .expect("em descriptor")
                .hash()
        );

        let seismic = ChannelDescriptor::parse_spec(
            "energy:primary:wave:hyperbolic:elastic-solid:unspecified:none",
            "Pa",
        )
        .expect("seismic spec parses");
        assert_eq!(
            seismic.hash(),
            descriptor_for_force("seismic-body", Medium::ElasticSolid)
                .expect("seismic descriptor")
                .hash()
        );
    }

    #[test]
    fn a_channel_spec_with_an_unknown_token_is_refused() {
        let err = ChannelDescriptor::parse_spec(
            "mass:primary:poisson:elliptic:aether:unspecified:none",
            "m/s^2",
        )
        .expect_err("aether is not a medium");
        assert!(err.contains("aether"), "the refused token is named: {err}");

        let arity = ChannelDescriptor::parse_spec("mass:poisson:vacuum", "m/s^2")
            .expect_err("a partial spec is refused");
        assert!(arity.contains("7 axes"), "the arity is named: {arity}");
    }

    #[test]
    fn an_inadmissible_channel_spec_is_refused() {
        let err = ChannelDescriptor::parse_spec(
            "mass:primary:flux-fourier:parabolic:fluid:unspecified:none",
            "K",
        )
        .expect_err("Fourier carries energy, not mass");
        assert!(
            err.contains("inadmissible"),
            "the refusal names the relation: {err}"
        );
    }

    #[test]
    fn every_live_force_type_carries_a_descriptor() {
        for ft in 0..9u8 {
            assert!(descriptor_for_force_type(ft).is_some(), "force type {ft}");
        }
        assert!(descriptor_for_force_type(9).is_none());
    }

    #[test]
    fn a_live_registry_carries_nine_channels_and_a_stable_fingerprint() {
        let reg = live_channel_registry();
        assert_eq!(reg.len(), 9);
        assert_eq!(reg.schema_hash(), live_schema_hash());
        assert_eq!(reg.schema_hash(), live_channel_registry().schema_hash());
    }

    #[test]
    fn the_fingerprint_changes_when_a_channel_differs() {
        let full = live_channel_registry();
        let mut one = ChannelRegistry::with_capacity(CHANNEL_CAP);
        one.register(descriptor_for_force("em", Medium::Vacuum).expect("em"));
        assert_ne!(full.schema_hash(), one.schema_hash());
    }
}
