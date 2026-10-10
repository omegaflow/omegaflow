use super::actuators::CHANNEL_CAP;
use super::force::force_name_of;
use super::media::MediumParams;
use std::collections::HashMap;

pub const VACUUM_SPEED_M_S: f64 = 299_792_458.0;

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
pub struct Quantity {
    pub conserved: Conserved,
    pub role: QuantityRole,
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
pub enum ModeFamily {
    Scalar = 0,
    Spheroidal = 1,
    Toroidal = 2,
}

impl ModeFamily {
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "scalar" => Some(ModeFamily::Scalar),
            "spheroidal" => Some(ModeFamily::Spheroidal),
            "toroidal" => Some(ModeFamily::Toroidal),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Domain {
    Unspecified,
    Line,
    Rectangle { lx: f64, ly: f64 },
    Circle,
    Sphere { l: u32 },
    Shell { r_in: f64, r_out: f64 },
    Ellipsoid { a: f64, b: f64, c: f64 },
}

impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Domain::Unspecified, Domain::Unspecified)
            | (Domain::Line, Domain::Line)
            | (Domain::Circle, Domain::Circle) => true,
            (Domain::Sphere { l: a }, Domain::Sphere { l: b }) => a == b,
            (Domain::Rectangle { lx: a, ly: b }, Domain::Rectangle { lx: c, ly: d }) => {
                a.to_bits() == c.to_bits() && b.to_bits() == d.to_bits()
            }
            (Domain::Shell { r_in: a, r_out: b }, Domain::Shell { r_in: c, r_out: d }) => {
                a.to_bits() == c.to_bits() && b.to_bits() == d.to_bits()
            }
            (
                Domain::Ellipsoid {
                    a: a1,
                    b: b1,
                    c: c1,
                },
                Domain::Ellipsoid {
                    a: a2,
                    b: b2,
                    c: c2,
                },
            ) => {
                a1.to_bits() == a2.to_bits()
                    && b1.to_bits() == b2.to_bits()
                    && c1.to_bits() == c2.to_bits()
            }
            _ => false,
        }
    }
}

impl Eq for Domain {}

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
            "circle" => Some(Domain::Circle),
            "sphere" => Some(Domain::Sphere { l: 0 }),
            _ => parse_compound_domain(token),
        }
    }

    fn tag(&self) -> u8 {
        match self {
            Domain::Unspecified => 0,
            Domain::Line => 1,
            Domain::Rectangle { .. } => 2,
            Domain::Circle => 3,
            Domain::Sphere { .. } => 4,
            Domain::Shell { .. } => 5,
            Domain::Ellipsoid { .. } => 6,
        }
    }

    fn hash_into(&self, h: u64) -> u64 {
        let h = fnv1a(&[self.tag()], h);
        match self {
            Domain::Rectangle { lx, ly } => fnv1a(
                &ly.to_bits().to_le_bytes(),
                fnv1a(&lx.to_bits().to_le_bytes(), h),
            ),
            Domain::Shell { r_in, r_out } => fnv1a(
                &r_out.to_bits().to_le_bytes(),
                fnv1a(&r_in.to_bits().to_le_bytes(), h),
            ),
            Domain::Ellipsoid { a, b, c } => {
                let h = fnv1a(&a.to_bits().to_le_bytes(), h);
                let h = fnv1a(&b.to_bits().to_le_bytes(), h);
                fnv1a(&c.to_bits().to_le_bytes(), h)
            }
            Domain::Sphere { l } => fnv1a(&l.to_le_bytes(), h),
            _ => h,
        }
    }
}

fn parse_compound_domain(token: &str) -> Option<Domain> {
    let open = token.find('(')?;
    if !token.ends_with(')') {
        return None;
    }
    let close = token.len() - 1;
    if close <= open {
        return None;
    }
    let kind = &token[..open];
    if kind == "sphere" {
        let l: u32 = token[open + 1..close].trim().parse().ok()?;
        return Some(Domain::Sphere { l });
    }
    let mut args: Vec<f64> = Vec::new();
    for raw in token[open + 1..close].split(',') {
        let v: f64 = raw.trim().parse().ok()?;
        if !v.is_finite() || v <= 0.0 {
            return None;
        }
        args.push(v);
    }
    match (kind, args.len()) {
        ("rectangle", 2) => Some(Domain::Rectangle {
            lx: args[0],
            ly: args[1],
        }),
        ("shell", 2) => {
            if args[1] <= args[0] {
                return None;
            }
            Some(Domain::Shell {
                r_in: args[0],
                r_out: args[1],
            })
        }
        ("ellipsoid", 3) => Some(Domain::Ellipsoid {
            a: args[0],
            b: args[1],
            c: args[2],
        }),
        _ => None,
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

#[derive(Clone, Debug)]
pub struct ChannelDescriptor {
    pub conserved: Conserved,
    pub role: QuantityRole,
    pub op: TransportOp,
    pub pde_type: PdeType,
    pub medium: Medium,
    pub domain: Domain,
    pub boundary: Boundary,
    pub extent: Option<f64>,
    pub family: ModeFamily,
    pub body: Option<String>,
    pub unit: &'static str,
}

impl PartialEq for ChannelDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.conserved == other.conserved
            && self.role == other.role
            && self.op == other.op
            && self.pde_type == other.pde_type
            && self.medium == other.medium
            && self.domain == other.domain
            && self.boundary == other.boundary
            && self.extent.map(f64::to_bits) == other.extent.map(f64::to_bits)
            && self.family == other.family
            && self.body == other.body
            && self.unit == other.unit
    }
}

impl Eq for ChannelDescriptor {}

impl ChannelDescriptor {
    pub fn new(
        quantity: Quantity,
        op: TransportOp,
        pde_type: PdeType,
        medium: Medium,
        domain: Domain,
        boundary: Boundary,
        unit: &'static str,
    ) -> Self {
        Self {
            conserved: quantity.conserved,
            role: quantity.role,
            op,
            pde_type,
            medium,
            domain,
            boundary,
            extent: None,
            family: match medium {
                Medium::ElasticSolid => ModeFamily::Spheroidal,
                Medium::Vacuum | Medium::Fluid => ModeFamily::Scalar,
            },
            body: None,
            unit,
        }
    }

    pub fn with_extent(mut self, extent: Option<f64>) -> Self {
        self.extent = extent;
        self
    }

    pub fn with_body(mut self, body: Option<String>) -> Self {
        self.body = body;
        self
    }

    pub fn with_family(mut self, family: ModeFamily) -> Self {
        self.family = family;
        self
    }

    pub fn mode_wavenumbers(&self, count: usize) -> Option<Vec<f64>> {
        eigen_wavenumbers(&self.domain, self.effective_boundary(), self.extent, count)
    }

    fn effective_boundary(&self) -> Boundary {
        match (self.boundary, self.medium) {
            (Boundary::FreeSurface, Medium::Fluid) => Boundary::Dirichlet,
            (Boundary::FreeSurface, Medium::ElasticSolid) => Boundary::Neumann,
            (boundary, _) => boundary,
        }
    }

    pub fn mode_frequencies_hz(&self, speed: f64, count: usize) -> Option<Vec<f64>> {
        self.mode_wavenumbers(count)?
            .into_iter()
            .map(|k| mode_frequency_hz(self.op, speed, k))
            .collect()
    }

    pub fn carrier_wavenumber(&self) -> Option<f64> {
        self.mode_wavenumbers(1)?.into_iter().next()
    }

    pub fn phase_velocity_m_s(&self) -> Option<f64> {
        if !matches!(self.family, ModeFamily::Scalar) {
            return None;
        }
        let k = self.carrier_wavenumber()?;
        if !(k.is_finite() && k > 0.0) {
            return None;
        }
        Some(std::f64::consts::TAU * self.fundamental_hz()? / k)
    }

    pub fn fundamental_hz(&self) -> Option<f64> {
        match self.body.as_deref() {
            Some(body) => self
                .mode_frequencies_hz_for_body(body, 1)?
                .into_iter()
                .next(),
            None => {
                let speed = characteristic_speed(self.medium, None)?;
                self.mode_frequencies_hz(speed, 1)?.into_iter().next()
            }
        }
    }

    pub fn mode_frequencies_hz_for_body(&self, body_name: &str, count: usize) -> Option<Vec<f64>> {
        let params = super::media::medium_params_of(body_name);
        match self.family {
            ModeFamily::Scalar => {
                let speed = characteristic_speed(self.medium, params.as_ref())?;
                self.mode_frequencies_hz(speed, count)
            }
            ModeFamily::Toroidal => {
                let l = match &self.domain {
                    Domain::Sphere { l } => *l,
                    _ => return None,
                };
                let r = self.extent.filter(|v| v.is_finite() && *v > 0.0)?;
                let (_, cs) = elastic_speeds(params.as_ref()?)?;
                elastic_toroidal_frequencies(l, cs, r, count)
            }
            ModeFamily::Spheroidal => {
                let l = match &self.domain {
                    Domain::Sphere { l } => *l,
                    _ => return None,
                };
                let r = self.extent.filter(|v| v.is_finite() && *v > 0.0)?;
                let (cp, cs) = elastic_speeds(params.as_ref()?)?;
                elastic_spheroidal_frequencies(l, cp, cs, r, count)
            }
        }
    }

    pub fn mode_degeneracy(&self) -> Option<u32> {
        match (&self.domain, self.effective_boundary()) {
            (Domain::Sphere { l }, Boundary::Dirichlet) => Some(2 * *l + 1),
            _ => None,
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
        h = fnv1a(&[self.family as u8], h);
        h = self.domain.hash_into(h);
        h = fnv1a(&[self.boundary as u8], h);
        h = match self.extent {
            None => fnv1a(&[0u8], h),
            Some(e) => fnv1a(&e.to_bits().to_le_bytes(), fnv1a(&[1u8], h)),
        };
        match &self.body {
            None => fnv1a(&[0u8], h),
            Some(b) => fnv1a(b.as_bytes(), fnv1a(&[1u8], h)),
        }
    }

    pub fn spec_token(&self) -> String {
        let conserved = conserved_name(self.conserved);
        let role = match self.role {
            QuantityRole::Primary => "primary",
            QuantityRole::Derived => "derived",
            QuantityRole::Geometry => "geometry",
            QuantityRole::SourceParameter => "source-parameter",
        };
        let op = match self.op {
            TransportOp::Flux(FluxKind::Fick) => "flux-fick",
            TransportOp::Flux(FluxKind::Fourier) => "flux-fourier",
            TransportOp::Flux(FluxKind::Ohm) => "flux-ohm",
            TransportOp::Flux(FluxKind::NewtonViscous) => "flux-newton-viscous",
            TransportOp::Advective => "advective",
            TransportOp::Wave => "wave",
            TransportOp::Poisson => "poisson",
            TransportOp::Maxwell => "maxwell",
        };
        let pde = match self.pde_type {
            PdeType::Elliptic => "elliptic",
            PdeType::Parabolic => "parabolic",
            PdeType::Hyperbolic => "hyperbolic",
            PdeType::Advective => "advective",
            PdeType::Mixed => "mixed",
        };
        let medium = match self.medium {
            Medium::Vacuum => "vacuum",
            Medium::Fluid => "fluid",
            Medium::ElasticSolid => "elastic-solid",
        };
        let domain = match &self.domain {
            Domain::Unspecified => "unspecified".to_string(),
            Domain::Line => "line".to_string(),
            Domain::Circle => "circle".to_string(),
            Domain::Sphere { l: 0 } => "sphere".to_string(),
            Domain::Sphere { l } => format!("sphere({l})"),
            Domain::Rectangle { lx, ly } => format!("rectangle({lx},{ly})"),
            Domain::Shell { r_in, r_out } => format!("shell({r_in},{r_out})"),
            Domain::Ellipsoid { a, b, c } => format!("ellipsoid({a},{b},{c})"),
        };
        let boundary = match self.boundary {
            Boundary::None => "none",
            Boundary::FreeSurface => "free-surface",
            Boundary::Dirichlet => "dirichlet",
            Boundary::Neumann => "neumann",
            Boundary::Robin => "robin",
        };
        let mut out = format!("{conserved}:{role}:{op}:{pde}:{medium}:{domain}:{boundary}");
        if let Some(e) = self.extent {
            out.push_str(&format!(":{e}"));
        }
        out
    }

    pub fn parse_spec(spec: &str, unit: &'static str) -> Result<Self, String> {
        let t: Vec<&str> = spec.split(':').collect();
        if t.len() != 7 && t.len() != 8 {
            return Err(format!(
                "channel spec needs 7 axes conserved:role:op:pde_type:medium:domain:boundary (or 8 with :extent), got {}",
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
        let extent = if t.len() == 8 {
            parse_extent(t[7])?
        } else {
            None
        };
        if !is_admissible(conserved, op, medium) {
            return Err(format!(
                "inadmissible channel {}/{}/{} is not in the admissibility relation",
                t[0], t[2], t[4]
            ));
        }
        Ok(ChannelDescriptor::new(
            Quantity { conserved, role },
            op,
            pde_type,
            medium,
            domain,
            boundary,
            unit,
        )
        .with_extent(extent))
    }
}

fn parse_extent(token: &str) -> Result<Option<f64>, String> {
    match token {
        "unspecified" | "absent" | "-" => Ok(None),
        _ => {
            let v: f64 = token
                .parse()
                .map_err(|_| format!("unknown extent \"{}\"", token))?;
            if !v.is_finite() || v <= 0.0 {
                return Err(format!(
                    "extent must be a positive finite length, got \"{}\"",
                    token
                ));
            }
            Ok(Some(v))
        }
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

pub fn descriptor_from_axes(
    role: &str,
    conserved: &str,
    operator: &str,
    pde_type: &str,
    medium: &str,
    boundary: &str,
    unit: &'static str,
) -> Result<ChannelDescriptor, String> {
    let role = QuantityRole::parse(role).ok_or_else(|| format!("unknown role \"{role}\""))?;
    let conserved =
        Conserved::parse(conserved).ok_or_else(|| format!("unknown conserved \"{conserved}\""))?;
    let op =
        TransportOp::parse(operator).ok_or_else(|| format!("unknown operator \"{operator}\""))?;
    let pde_type =
        PdeType::parse(pde_type).ok_or_else(|| format!("unknown pde_type \"{pde_type}\""))?;
    let medium_token = medium;
    let medium =
        Medium::parse(medium_token).ok_or_else(|| format!("unknown medium \"{medium_token}\""))?;
    let boundary =
        Boundary::parse(boundary).ok_or_else(|| format!("unknown boundary \"{boundary}\""))?;
    if !is_admissible(conserved, op, medium) {
        return Err(format!(
            "inadmissible operator/medium pair \"{operator}/{medium_token}\" is not in the admissibility relation"
        ));
    }
    Ok(ChannelDescriptor::new(
        Quantity { conserved, role },
        op,
        pde_type,
        medium,
        Domain::Unspecified,
        boundary,
        unit,
    ))
}

pub fn live_channel_hash_of(d: &ChannelDescriptor) -> Option<u64> {
    let h = d.hash();
    live_channel_registry().descriptor(h).map(|_| h)
}

pub fn descriptor_for_force(name: &str, medium: Medium) -> Option<ChannelDescriptor> {
    let (conserved, op, pde_type, boundary, unit) = match name {
        "em" => (
            Conserved::Energy,
            TransportOp::Maxwell,
            PdeType::Mixed,
            Boundary::None,
            "V/m",
        ),
        "electric" => (
            Conserved::Energy,
            TransportOp::Maxwell,
            PdeType::Elliptic,
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
        Quantity {
            conserved,
            role: QuantityRole::Primary,
        },
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

pub fn force_type_of_descriptor(d: &ChannelDescriptor) -> Option<u8> {
    let h = d.hash();
    (0..9u8).find(|&ft| descriptor_for_force_type(ft).is_some_and(|fd| fd.hash() == h))
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

const SPHERE_ZERO_EPS: f64 = 1e-4;
const SPHERE_ZERO_STEP: f64 = 0.02;
const SPHERE_ZERO_SEARCH_MAX: f64 = 200.0;

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

fn spherical_bessel_j(l: u32, x: f64) -> f64 {
    if x == 0.0 {
        return if l == 0 { 1.0 } else { 0.0 };
    }
    let mut jm1 = x.sin() / x;
    if l == 0 {
        return jm1;
    }
    let mut jn = x.sin() / (x * x) - x.cos() / x;
    for n in 1..l {
        let jp1 = (2.0 * n as f64 + 1.0) / x * jn - jm1;
        jm1 = jn;
        jn = jp1;
    }
    jn
}

fn spherical_bessel_zero(l: u32, lo: f64, hi: f64) -> f64 {
    let mut a = lo;
    let mut b = hi;
    let mut fa = spherical_bessel_j(l, a);
    for _ in 0..200 {
        if b - a <= 1e-13 {
            break;
        }
        let mid = 0.5 * (a + b);
        let fm = spherical_bessel_j(l, mid);
        if (fm < 0.0) != (fa < 0.0) {
            b = mid;
        } else {
            a = mid;
            fa = fm;
        }
    }
    0.5 * (a + b)
}

fn spherical_bessel_zeros(l: u32, count: usize) -> Option<Vec<f64>> {
    let mut zeros: Vec<f64> = Vec::with_capacity(count);
    let mut prev_x = SPHERE_ZERO_EPS;
    let mut prev = spherical_bessel_j(l, prev_x);
    let mut x = prev_x + SPHERE_ZERO_STEP;
    while x <= SPHERE_ZERO_SEARCH_MAX && zeros.len() < count {
        let cur = spherical_bessel_j(l, x);
        if (cur < 0.0) != (prev < 0.0) {
            zeros.push(spherical_bessel_zero(l, prev_x, x));
        }
        prev_x = x;
        prev = cur;
        x += SPHERE_ZERO_STEP;
    }
    if zeros.len() == count {
        Some(zeros)
    } else {
        None
    }
}

const ELASTIC_SEARCH_MAX: f64 = 400.0;
const ELASTIC_ZERO_START: f64 = 0.5;

fn spherical_bessel_j_prime(l: u32, x: f64) -> f64 {
    if l == 0 {
        -spherical_bessel_j(1, x)
    } else {
        spherical_bessel_j(l - 1, x) - (l + 1) as f64 / x * spherical_bessel_j(l, x)
    }
}

fn toroidal_root_function(l: u32, x: f64) -> f64 {
    spherical_bessel_j(l, x) - x * spherical_bessel_j_prime(l, x)
}

fn spheroidal_det(l: u32, xi: f64, eta: f64) -> f64 {
    let lf = l as f64;
    let t11 = (lf * lf - lf - eta * eta / 2.0) * spherical_bessel_j(l, xi)
        + 2.0 * xi * spherical_bessel_j(l + 1, xi);
    let t13 = lf
        * (lf + 1.0)
        * ((lf - 1.0) * spherical_bessel_j(l, eta) - eta * spherical_bessel_j(l + 1, eta));
    let t41 = (lf - 1.0) * spherical_bessel_j(l, xi) - xi * spherical_bessel_j(l + 1, xi);
    let t43 = (lf * lf - 1.0 - eta * eta / 2.0) * spherical_bessel_j(l, eta)
        + eta * spherical_bessel_j(l + 1, eta);
    t11 * t43 - t13 * t41
}

fn spheroidal_root_function(l: u32, cs_over_cp: f64, eta: f64) -> f64 {
    let xi = cs_over_cp * eta;
    if l == 0 {
        -(eta * eta / 2.0) * spherical_bessel_j(0, xi) + 2.0 * xi * spherical_bessel_j(1, xi)
    } else {
        spheroidal_det(l, xi, eta)
    }
}

fn bisect(f: impl Fn(f64) -> f64, mut a: f64, mut b: f64) -> f64 {
    let mut fa = f(a);
    for _ in 0..200 {
        if b - a <= 1e-13 {
            break;
        }
        let mid = 0.5 * (a + b);
        let fm = f(mid);
        if (fm < 0.0) != (fa < 0.0) {
            b = mid;
        } else {
            a = mid;
            fa = fm;
        }
    }
    0.5 * (a + b)
}

fn scan_roots(f: impl Fn(f64) -> f64, count: usize) -> Option<Vec<f64>> {
    let mut roots: Vec<f64> = Vec::with_capacity(count);
    let mut a = ELASTIC_ZERO_START;
    let mut fa = f(a);
    let mut x = a + SPHERE_ZERO_STEP;
    while x <= ELASTIC_SEARCH_MAX && roots.len() < count {
        let fx = f(x);
        if fx.is_finite() && fa.is_finite() && (fx < 0.0) != (fa < 0.0) {
            roots.push(bisect(&f, a, x));
        }
        a = x;
        fa = fx;
        x += SPHERE_ZERO_STEP;
    }
    if roots.len() == count {
        Some(roots)
    } else {
        None
    }
}

fn elastic_toroidal_frequencies(l: u32, cs: f64, r: f64, count: usize) -> Option<Vec<f64>> {
    let roots = scan_roots(|x| toroidal_root_function(l, x), count)?;
    Some(
        roots
            .into_iter()
            .map(|x| cs * x / (std::f64::consts::TAU * r))
            .collect(),
    )
}

fn elastic_spheroidal_frequencies(
    l: u32,
    cp: f64,
    cs: f64,
    r: f64,
    count: usize,
) -> Option<Vec<f64>> {
    let ratio = cs / cp;
    let roots = scan_roots(|eta| spheroidal_root_function(l, ratio, eta), count)?;
    Some(
        roots
            .into_iter()
            .map(|eta| cs * eta / (std::f64::consts::TAU * r))
            .collect(),
    )
}

pub fn eigen_wavenumbers(
    domain: &Domain,
    boundary: Boundary,
    extent: Option<f64>,
    count: usize,
) -> Option<Vec<f64>> {
    if count == 0 {
        return None;
    }
    match domain {
        Domain::Rectangle { lx, ly } => rectangular_wavenumbers(*lx, *ly, boundary, count),
        Domain::Line | Domain::Circle | Domain::Sphere { .. } => {
            let extent = extent?;
            if !extent.is_finite() || extent <= 0.0 {
                return None;
            }
            match (domain, boundary) {
                (Domain::Line, Boundary::Dirichlet) => Some(
                    (1..=count)
                        .map(|j| j as f64 * std::f64::consts::PI / extent)
                        .collect(),
                ),
                (Domain::Line, Boundary::Neumann) => Some(
                    (0..count)
                        .map(|j| j as f64 * std::f64::consts::PI / extent)
                        .collect(),
                ),
                (Domain::Sphere { l: 0 }, Boundary::Dirichlet) => Some(
                    (1..=count)
                        .map(|j| j as f64 * std::f64::consts::PI / extent)
                        .collect(),
                ),
                (Domain::Sphere { l }, Boundary::Dirichlet) => {
                    let zeros = spherical_bessel_zeros(*l, count)?;
                    Some(zeros.into_iter().map(|z| z / extent).collect())
                }
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
        Domain::Unspecified | Domain::Shell { .. } | Domain::Ellipsoid { .. } => None,
    }
}

fn rectangular_wavenumbers(lx: f64, ly: f64, boundary: Boundary, count: usize) -> Option<Vec<f64>> {
    let start = match boundary {
        Boundary::Dirichlet => 1usize,
        Boundary::Neumann => 0usize,
        _ => return None,
    };
    let mut bound = count + 1;
    loop {
        let mut modes: Vec<f64> = Vec::new();
        for m in start..=bound {
            for n in start..=bound {
                let km = m as f64 * std::f64::consts::PI / lx;
                let kn = n as f64 * std::f64::consts::PI / ly;
                modes.push((km * km + kn * kn).sqrt());
            }
        }
        if modes.len() < count {
            return None;
        }
        modes.sort_by(|a, b| a.total_cmp(b));
        let k_max = modes[count - 1];
        let need_x = (k_max * lx / std::f64::consts::PI).ceil() as usize + 1;
        let need_y = (k_max * ly / std::f64::consts::PI).ceil() as usize + 1;
        if need_x <= bound && need_y <= bound {
            return Some(modes[..count].to_vec());
        }
        bound = need_x.max(need_y).max(bound + 1);
        if bound > 4096 {
            return None;
        }
    }
}

pub fn characteristic_speed(medium: Medium, params: Option<&MediumParams>) -> Option<f64> {
    match medium {
        Medium::Vacuum => Some(VACUUM_SPEED_M_S),
        Medium::Fluid => {
            let p = params?;
            if p.sound_speed_m_s.is_finite() && p.sound_speed_m_s > 0.0 {
                Some(p.sound_speed_m_s)
            } else {
                None
            }
        }
        Medium::ElasticSolid => None,
    }
}

pub fn elastic_speeds(params: &MediumParams) -> Option<(f64, f64)> {
    let cp = params.p_wave_m_s;
    let cs = params.s_wave_m_s;
    let finite_positive = |v: f64| v.is_finite() && v > 0.0;
    if finite_positive(cp) && finite_positive(cs) && cp >= cs {
        Some((cp, cs))
    } else {
        None
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

    pub fn descriptors(&self) -> &[ChannelDescriptor] {
        &self.descs
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

    pub fn mode_frequencies_hz_for_body(
        &self,
        body_name: &str,
        count: usize,
    ) -> Vec<Option<Vec<f64>>> {
        self.descs
            .iter()
            .map(|d| d.mode_frequencies_hz_for_body(body_name, count))
            .collect()
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
            let mut d = d.clone();
            if d.body.is_none() {
                d.body = s.body.clone();
            }
            reg.register(d);
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
            Quantity {
                conserved: Conserved::Mass,
                role: QuantityRole::Primary,
            },
            TransportOp::Flux(FluxKind::Fick),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
            Boundary::None,
            "kg",
        );
        let b = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Mass,
                role: QuantityRole::Primary,
            },
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
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
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
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Unspecified,
            Boundary::None,
            "kg m / s",
        );
        let line = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
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
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
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
            Quantity {
                conserved: Conserved::Mass,
                role: QuantityRole::Primary,
            },
            TransportOp::Flux(FluxKind::Fick),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
            Boundary::None,
            "kg",
        );
        let geometry = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Mass,
                role: QuantityRole::Geometry,
            },
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
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Maxwell,
            PdeType::Elliptic,
            Medium::Vacuum,
            Domain::Unspecified,
            Boundary::None,
            "V/m",
        );
        let hyperbolic = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
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
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
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
        let a = reg.register(d.clone());
        let b = reg.register(d.clone());
        assert_eq!(a, b);
        assert_eq!(reg.len(), 1);
        assert_eq!(reg.descriptor(d.hash()).map(|x| x.hash()), Some(d.hash()));
    }

    #[test]
    fn distinct_conserved_quantities_stay_distinct() {
        let mut reg = ChannelRegistry::with_capacity(16);
        reg.register(heat());
        reg.register(ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Mass,
                role: QuantityRole::Primary,
            },
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
            Quantity {
                conserved: Conserved::Charge,
                role: QuantityRole::Primary,
            },
            TransportOp::Flux(FluxKind::Ohm),
            PdeType::Elliptic,
            Medium::Fluid,
            Domain::Unspecified,
            Boundary::None,
            "A / m2",
        );
        let idx = reg.register(em.clone());
        reg.alias("electric", &em);
        assert_eq!(reg.resolve("electric").map(|d| d.hash()), Some(em.hash()));
        assert!(reg.id.get(&em.hash()) == Some(&idx));
        assert!(reg.resolve("unknown").is_none());
    }

    #[test]
    fn the_unit_follows_from_quantity_and_operator_not_identity() {
        let a = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Flux(FluxKind::Fourier),
            PdeType::Parabolic,
            Medium::Fluid,
            Domain::Unspecified,
            Boundary::None,
            "J / (m2 s)",
        );
        let b = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
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
    fn electric_is_the_quasi_static_subset_of_em_and_no_longer_collapses() {
        let em = descriptor_for_force("em", Medium::Vacuum).expect("em");
        let electric = descriptor_for_force("electric", Medium::Vacuum).expect("electric");
        assert_ne!(em.hash(), electric.hash());
        assert_eq!(em.pde_type, PdeType::Mixed);
        assert_eq!(electric.pde_type, PdeType::Elliptic);
    }

    #[test]
    fn an_unknown_label_is_refused() {
        assert!(descriptor_for_force("phlogiston", Medium::Fluid).is_none());
    }

    #[test]
    fn a_line_is_harmonic() {
        let k = eigen_wavenumbers(&Domain::Line, Boundary::Dirichlet, Some(1.0), 3)
            .expect("line modes");
        let pi = std::f64::consts::PI;
        assert_eq!(k, vec![pi, 2.0 * pi, 3.0 * pi]);
    }

    #[test]
    fn a_sphere_dirichlet_is_the_l0_radial_sector() {
        let pi = std::f64::consts::PI;
        let k = eigen_wavenumbers(&Domain::Sphere { l: 0 }, Boundary::Dirichlet, Some(1.0), 2)
            .expect("sphere l=0");
        assert_eq!(k, vec![pi, 2.0 * pi]);
    }

    #[test]
    fn a_sphere_l1_is_the_j1_zero_spectrum() {
        let k = eigen_wavenumbers(&Domain::Sphere { l: 1 }, Boundary::Dirichlet, Some(1.0), 3)
            .expect("sphere l=1");
        assert!((k[0] - 4.493_409).abs() < 1e-3, "first j1 zero {}", k[0]);
        assert!((k[1] - 7.725_252).abs() < 1e-3, "second j1 zero {}", k[1]);
        assert!(k[2] > k[1]);
    }

    #[test]
    fn a_sphere_carries_its_sector_in_the_hash() {
        let l0 = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Sphere { l: 0 },
            Boundary::Dirichlet,
            "Pa",
        );
        let l1 = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Sphere { l: 1 },
            Boundary::Dirichlet,
            "Pa",
        );
        assert_ne!(l0.hash(), l1.hash());
        assert_eq!(Domain::parse("sphere"), Some(Domain::Sphere { l: 0 }));
        assert_eq!(Domain::parse("sphere(2)"), Some(Domain::Sphere { l: 2 }));
    }

    #[test]
    fn a_bare_free_surface_has_no_closed_spectrum() {
        assert!(eigen_wavenumbers(&Domain::Line, Boundary::FreeSurface, Some(2.0), 2).is_none());
    }

    #[test]
    fn a_free_surface_binds_through_the_medium() {
        let fluid = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Line,
            Boundary::FreeSurface,
            "Pa",
        )
        .with_extent(Some(2.0));
        let dirichlet =
            eigen_wavenumbers(&Domain::Line, Boundary::Dirichlet, Some(2.0), 2).expect("dir");
        assert_eq!(fluid.mode_wavenumbers(2), Some(dirichlet));

        let solid = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Line,
            Boundary::FreeSurface,
            "m",
        )
        .with_extent(Some(2.0));
        let neumann =
            eigen_wavenumbers(&Domain::Line, Boundary::Neumann, Some(2.0), 2).expect("neumann");
        assert_eq!(solid.mode_wavenumbers(2), Some(neumann));
        assert_eq!(solid.mode_wavenumbers(2).expect("solid modes")[0], 0.0);
    }

    #[test]
    fn a_neumann_line_keeps_the_constant_mode() {
        let k = eigen_wavenumbers(&Domain::Line, Boundary::Neumann, Some(1.0), 3).expect("neumann");
        assert_eq!(k[0], 0.0);
        assert!(k[1] > 0.0);
    }

    #[test]
    fn a_circle_is_the_drum_spectrum() {
        let k =
            eigen_wavenumbers(&Domain::Circle, Boundary::Dirichlet, Some(1.0), 3).expect("circle");
        assert!((k[0] - FIRST_BESSEL_J0_ZERO).abs() < 1e-9);
        assert!((k[1] - 3.831_706).abs() < 1e-4);
        assert!((k[2] - 5.135_622).abs() < 1e-4);
    }

    #[test]
    fn a_circle_keeps_the_documented_mode_ratios_through_nine() {
        let extent = FIRST_BESSEL_J0_ZERO;
        let k = eigen_wavenumbers(&Domain::Circle, Boundary::Dirichlet, Some(extent), 9)
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
        let k = eigen_wavenumbers(&Domain::Circle, Boundary::Dirichlet, Some(1.0), CHANNEL_CAP)
            .expect("16 circle modes");
        assert_eq!(k.len(), CHANNEL_CAP);
        for j in 1..k.len() {
            assert!(k[j] > k[j - 1], "mode {j} must exceed mode {}", j - 1);
        }
    }

    #[test]
    fn a_circle_with_zero_count_has_no_mode() {
        assert!(eigen_wavenumbers(&Domain::Circle, Boundary::Dirichlet, Some(1.0), 0).is_none());
    }

    #[test]
    fn a_domain_without_a_closed_mode_is_absent() {
        assert!(eigen_wavenumbers(&Domain::Unspecified, Boundary::None, Some(1.0), 3).is_none());
        assert!(
            eigen_wavenumbers(
                &Domain::Shell {
                    r_in: 1.0,
                    r_out: 2.0
                },
                Boundary::Dirichlet,
                Some(1.0),
                3
            )
            .is_none()
        );
        assert!(eigen_wavenumbers(&Domain::Line, Boundary::Robin, Some(1.0), 3).is_none());
        assert!(
            eigen_wavenumbers(
                &Domain::Circle,
                Boundary::Dirichlet,
                Some(1.0),
                CHANNEL_CAP + 1
            )
            .is_none()
        );
    }

    #[test]
    fn a_rectangular_membrane_is_the_pythagorean_drum() {
        let pi = std::f64::consts::PI;
        let k = eigen_wavenumbers(
            &Domain::Rectangle { lx: 2.0, ly: 3.0 },
            Boundary::Dirichlet,
            Some(1.0),
            4,
        )
        .expect("rectangle modes");
        let expected = [
            pi * (0.25_f64 + 1.0 / 9.0).sqrt(),
            pi * (0.25_f64 + 4.0 / 9.0).sqrt(),
            pi * (1.0_f64 + 1.0 / 9.0).sqrt(),
            pi * (0.25_f64 + 1.0).sqrt(),
        ];
        for (j, e) in expected.iter().enumerate() {
            assert!((k[j] - e).abs() < 1e-9, "mode {j}: {} vs {}", k[j], e);
        }
    }

    #[test]
    fn a_rectangle_carries_its_arity_in_the_domain_not_in_extent() {
        let d = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Rectangle { lx: 1.0, ly: 1.0 },
            Boundary::Dirichlet,
            "Pa",
        );
        assert!(d.mode_wavenumbers(2).is_some());
    }

    #[test]
    fn the_compound_domain_token_parses_its_arity() {
        assert_eq!(
            Domain::parse("rectangle(2.0,3.0)"),
            Some(Domain::Rectangle { lx: 2.0, ly: 3.0 })
        );
        assert_eq!(
            Domain::parse("shell(1.0,2.0)"),
            Some(Domain::Shell {
                r_in: 1.0,
                r_out: 2.0
            })
        );
        assert_eq!(
            Domain::parse("ellipsoid(1.0,2.0,3.0)"),
            Some(Domain::Ellipsoid {
                a: 1.0,
                b: 2.0,
                c: 3.0
            })
        );
        assert_eq!(Domain::parse("shell(2.0,1.0)"), None);
        assert_eq!(Domain::parse("rectangle(2.0)"), None);
        assert_eq!(Domain::parse("rectangle(0.0,3.0)"), None);
    }

    #[test]
    fn a_non_positive_extent_has_no_mode() {
        assert!(eigen_wavenumbers(&Domain::Line, Boundary::Dirichlet, Some(0.0), 3).is_none());
        assert!(eigen_wavenumbers(&Domain::Line, Boundary::Dirichlet, Some(-1.0), 3).is_none());
        assert!(eigen_wavenumbers(&Domain::Line, Boundary::Dirichlet, Some(f64::NAN), 3).is_none());
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
    fn a_wave_line_carries_harmonic_mode_frequencies() {
        let d = ChannelDescriptor::parse_spec(
            "energy:primary:wave:hyperbolic:fluid:line:dirichlet:2.0",
            "Pa",
        )
        .expect("wave line spec parses");
        let f = d.mode_frequencies_hz(343.0, 3).expect("modes carry pitch");
        assert_eq!(f.len(), 3);
        for (i, fj) in f.iter().enumerate() {
            let j = (i + 1) as f64;
            assert!((fj - 343.0 * j / (2.0 * 2.0)).abs() < 1e-9);
        }
    }

    #[test]
    fn a_diffusive_mode_carries_no_mode_frequency() {
        let d = ChannelDescriptor::parse_spec(
            "energy:primary:flux-fourier:parabolic:fluid:line:dirichlet:2.0",
            "K",
        )
        .expect("fourier spec parses");
        assert!(d.mode_frequencies_hz(1.0, 3).is_none());
    }

    #[test]
    fn a_wave_without_extent_carries_no_mode_frequency() {
        let d = ChannelDescriptor::parse_spec(
            "energy:primary:wave:hyperbolic:fluid:line:dirichlet",
            "Pa",
        )
        .expect("wave line spec parses");
        assert!(d.mode_frequencies_hz(343.0, 3).is_none());
    }

    #[test]
    fn a_medium_constitutes_its_characteristic_speed() {
        let fluid = MediumParams {
            sound_speed_m_s: 343.0,
            p_wave_m_s: 0.0,
            s_wave_m_s: 0.0,
            thermal_diffusivity_m2_s: 0.0,
            molecular_diffusivity_m2_s: 0.0,
        };
        assert_eq!(
            characteristic_speed(Medium::Vacuum, None),
            Some(VACUUM_SPEED_M_S)
        );
        assert_eq!(
            characteristic_speed(Medium::Fluid, Some(&fluid)),
            Some(343.0)
        );
        assert_eq!(characteristic_speed(Medium::Fluid, None), None);
        assert_eq!(
            characteristic_speed(Medium::ElasticSolid, Some(&fluid)),
            None
        );
    }

    #[test]
    fn the_declared_body_constitutes_the_channel_frequencies() {
        let fluid = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Line,
            Boundary::Dirichlet,
            "Pa",
        )
        .with_extent(Some(2.0));
        let f = fluid
            .mode_frequencies_hz_for_body("earth", 2)
            .expect("earth carries a sound speed");
        assert_eq!(f.len(), 2);
        assert!(f[0] > 0.0);

        let solid = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Line,
            Boundary::Neumann,
            "m",
        )
        .with_extent(Some(2.0));
        assert!(solid.mode_frequencies_hz_for_body("earth", 2).is_none());

        let vacuum = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Maxwell,
            PdeType::Mixed,
            Medium::Vacuum,
            Domain::Line,
            Boundary::Dirichlet,
            "V/m",
        )
        .with_extent(Some(2.0));
        assert!(
            vacuum.mode_frequencies_hz_for_body("iss", 2).is_some(),
            "vacuum is the constant light speed, no body needed"
        );
    }

    #[test]
    fn an_elastic_solid_constitutes_two_speeds() {
        let solid = MediumParams {
            sound_speed_m_s: 0.0,
            p_wave_m_s: 5950.0,
            s_wave_m_s: 3630.0,
            thermal_diffusivity_m2_s: 0.0,
            molecular_diffusivity_m2_s: 0.0,
        };
        assert_eq!(elastic_speeds(&solid), Some((5950.0, 3630.0)));
        let inverted = MediumParams {
            p_wave_m_s: 3630.0,
            s_wave_m_s: 5950.0,
            ..solid.clone()
        };
        assert_eq!(elastic_speeds(&inverted), None);
    }

    #[test]
    fn the_elastic_sphere_solver_matches_its_closed_forms() {
        let radial =
            elastic_spheroidal_frequencies(0, 3f64.sqrt(), 1.0, 1.0, 1).expect("radial l=0");
        assert!(
            (radial[0] - 0.7067).abs() < 0.02,
            "radial l=0 first tone {}",
            radial[0]
        );
        let tor = elastic_toroidal_frequencies(1, 1.0, 1.0, 1).expect("toroidal l=1");
        assert!(
            (tor[0] - 5.7635 / std::f64::consts::TAU).abs() < 0.02,
            "toroidal l=1 first tone {}",
            tor[0]
        );
    }

    #[test]
    fn the_earth_seismic_channel_rings_through_its_body() {
        let d = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Sphere { l: 0 },
            Boundary::FreeSurface,
            "Pa",
        )
        .with_extent(Some(6_371_000.0))
        .with_body(Some("earth".to_string()));
        let f = d
            .fundamental_hz()
            .expect("the earth seismic channel carries a fundamental tone");
        assert!(f.is_finite() && f > 0.0, "fundamental {f}");
    }

    #[test]
    fn a_channel_without_geometry_stays_silent() {
        let d = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Maxwell,
            PdeType::Mixed,
            Medium::Vacuum,
            Domain::Unspecified,
            Boundary::None,
            "V/m",
        );
        assert_eq!(d.fundamental_hz(), None);
    }

    #[test]
    fn the_propagating_phase_velocity_equals_the_medium_speed() {
        let d = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Sphere { l: 0 },
            Boundary::Dirichlet,
            "Pa",
        )
        .with_extent(Some(2.0))
        .with_body(Some("earth".to_string()));
        let k = d
            .carrier_wavenumber()
            .expect("geometry carries a wavenumber");
        let v_p = d
            .phase_velocity_m_s()
            .expect("scalar branch carries a velocity");
        let params = crate::media::medium_params_of("earth");
        let c = characteristic_speed(Medium::Fluid, params.as_ref()).expect("earth sound speed");
        assert!(k > 0.0);
        assert!((v_p - c).abs() < 1e-9, "v_p {v_p} vs c {c}");
    }

    #[test]
    fn an_elastic_channel_carries_no_single_phase_velocity() {
        let d = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Sphere { l: 0 },
            Boundary::FreeSurface,
            "Pa",
        )
        .with_extent(Some(6_371_000.0))
        .with_body(Some("earth".to_string()));
        assert_eq!(d.phase_velocity_m_s(), None);
    }

    #[test]
    fn an_elastic_solid_defaults_to_the_spheroidal_family() {
        let base = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::ElasticSolid,
            Domain::Sphere { l: 1 },
            Boundary::FreeSurface,
            "m",
        )
        .with_extent(Some(1.0));
        assert_eq!(base.family, ModeFamily::Spheroidal);
        let toroidal = base.clone().with_family(ModeFamily::Toroidal);
        assert_ne!(base.hash(), toroidal.hash());
        let fluid = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Energy,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Sphere { l: 1 },
            Boundary::Dirichlet,
            "Pa",
        )
        .with_extent(Some(1.0));
        assert_eq!(fluid.family, ModeFamily::Scalar);
        assert_eq!(ModeFamily::parse("toroidal"), Some(ModeFamily::Toroidal));
    }

    #[test]
    fn a_sphere_sector_carries_its_degeneracy() {
        let d = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Sphere { l: 2 },
            Boundary::Dirichlet,
            "Pa",
        );
        assert_eq!(d.mode_degeneracy(), Some(5));
        let line = ChannelDescriptor::new(
            Quantity {
                conserved: Conserved::Momentum,
                role: QuantityRole::Primary,
            },
            TransportOp::Wave,
            PdeType::Hyperbolic,
            Medium::Fluid,
            Domain::Line,
            Boundary::Dirichlet,
            "Pa",
        );
        assert_eq!(line.mode_degeneracy(), None);
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
    fn an_eight_axis_spec_carries_the_geometry_extent() {
        let d = ChannelDescriptor::parse_spec(
            "energy:primary:wave:hyperbolic:fluid:circle:dirichlet:2.0",
            "Pa",
        )
        .expect("circle channel with extent parses");
        assert_eq!(d.extent, Some(2.0));
        let k = d.mode_wavenumbers(3).expect("circle modes");
        assert_eq!(k.len(), 3);
        assert!((k[0] - FIRST_BESSEL_J0_ZERO / 2.0).abs() < 1e-9);
    }

    #[test]
    fn a_seven_axis_spec_leaves_the_extent_absent() {
        let d = ChannelDescriptor::parse_spec(
            "mass:primary:poisson:elliptic:vacuum:line:dirichlet",
            "m/s^2",
        )
        .expect("seven axes parse");
        assert_eq!(d.extent, None);
        assert!(d.mode_wavenumbers(3).is_none(), "absent extent is no mode");
    }

    #[test]
    fn a_descriptor_round_trips_through_its_spec_token() {
        let d = ChannelDescriptor::parse_spec(
            "momentum:primary:wave:hyperbolic:fluid:rectangle(2,3):dirichlet",
            "Pa",
        )
        .expect("rectangle channel parses")
        .with_extent(Some(2.5));
        let token = d.spec_token();
        let back = ChannelDescriptor::parse_spec(&token, "Pa").expect("round-trips");
        assert_eq!(token, back.spec_token(), "the token is a fixed point");
        assert_eq!(d, back, "the descriptor survives its own token");
        assert_eq!(d.hash(), back.hash(), "identity is stable");
    }

    #[test]
    fn the_extent_enters_the_channel_identity() {
        let bare = ChannelDescriptor::parse_spec(
            "energy:primary:wave:hyperbolic:fluid:line:dirichlet",
            "Pa",
        )
        .expect("bare parses");
        let sized = bare.clone().with_extent(Some(1.0));
        assert_ne!(bare.hash(), sized.hash());
        assert_ne!(bare, sized);
    }

    #[test]
    fn a_nonpositive_or_nan_extent_is_refused() {
        for token in ["0", "-1", "NaN", "furlongs"] {
            let err = ChannelDescriptor::parse_spec(
                &format!("energy:primary:wave:hyperbolic:fluid:circle:dirichlet:{token}"),
                "Pa",
            )
            .expect_err("bad extent is refused");
            assert!(err.contains("extent"), "the refusal names the axis: {err}");
        }
    }

    #[test]
    fn every_live_force_type_carries_a_descriptor() {
        for ft in 0..9u8 {
            assert!(descriptor_for_force_type(ft).is_some(), "force type {ft}");
        }
        assert!(descriptor_for_force_type(9).is_none());
    }

    #[test]
    fn p10_descriptor_axes_resolve_in_the_live_registry_and_refuse_otherwise() {
        let em = descriptor_from_axes(
            "primary", "energy", "maxwell", "mixed", "vacuum", "none", "V/m",
        )
        .expect("the em axes build a descriptor");
        assert!(
            live_channel_hash_of(&em).is_some(),
            "the em descriptor is an exact registry entry"
        );

        assert!(
            descriptor_from_axes(
                "primary", "energy", "maxwell", "warp", "vacuum", "none", "V/m"
            )
            .is_err(),
            "an unknown pde_type is a named refusal, never a default"
        );
        assert!(
            descriptor_from_axes(
                "primary",
                "mass",
                "flux-fourier",
                "parabolic",
                "fluid",
                "none",
                "W/m^2"
            )
            .is_err(),
            "Fourier couples to energy, not mass — the admissibility relation refuses"
        );
    }

    #[test]
    fn every_force_type_round_trips_through_its_descriptor() {
        for ft in 0..9u8 {
            let d = descriptor_for_force_type(ft).expect("descriptor");
            assert_eq!(force_type_of_descriptor(&d), Some(ft), "force type {ft}");
        }
        let em = descriptor_for_force("em", Medium::Vacuum).expect("em");
        let electric = descriptor_for_force("electric", Medium::Vacuum).expect("electric");
        assert_eq!(force_type_of_descriptor(&em), Some(0));
        assert_eq!(force_type_of_descriptor(&electric), Some(8));
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
