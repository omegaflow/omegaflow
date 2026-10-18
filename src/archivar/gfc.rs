use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 3] = [0xCF, 0x86, 0x09];
pub const FRAME_HEADER: usize = 19;
pub const SLOTS: usize = 26;
pub const RECORD_BYTES: usize = SLOTS * 8;

pub const SLOT_VAL: usize = 3;
pub const SLOT_EPOCH: usize = 4;
pub const SLOT_TTL: usize = 5;
pub const SLOT_KERNEL: usize = 8;
pub const SLOT_CHANNEL: usize = 9;
pub const SLOT_FREQ: usize = 22;
pub const SLOT_BIN_WIDTH: usize = 23;
pub const SLOT_PRESENCE: usize = 25;

pub const FUNC_GFC: u32 = 0;
pub const FUNC_GFCT: u32 = 1;

pub struct Channel {
    pub comp: u32,
    pub name: &'static str,
    pub unit: &'static str,
}

const CHANNELS: &[Channel] = &[
    Channel {
        comp: 1,
        name: "icgem_gfc_c",
        unit: "1",
    },
    Channel {
        comp: 2,
        name: "icgem_gfc_s",
        unit: "1",
    },
    Channel {
        comp: 3,
        name: "icgem_gfct_c",
        unit: "1",
    },
    Channel {
        comp: 4,
        name: "icgem_gfct_s",
        unit: "1",
    },
];

pub fn channels() -> &'static [Channel] {
    CHANNELS
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    CHANNELS.iter().find(|c| c.comp == comp).map(|c| c.name)
}

pub fn channel_of(functional: u32, sine: bool) -> Option<u32> {
    let base = match functional {
        FUNC_GFC => 1,
        FUNC_GFCT => 3,
        _ => return None,
    };
    Some(base + u32::from(sine))
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(channel_ref) = force_id_of("gravity") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(channel_ref) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(CHANNELS.len());
    for c in CHANNELS {
        if component_name(c.comp).is_none() {
            continue;
        }
        out.push(FieldConfig {
            key: c.name.to_string(),
            name: c.name.to_string(),
            band_id: None,
            kernel,
            channel_ref: Some(channel_ref),
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: c.unit.to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: Aperture::None,
        });
    }
    out
}

pub fn write_bin(records: &[[f64; 26]]) -> Option<Vec<u8>> {
    if records.is_empty() {
        return None;
    }
    let response_epoch = records
        .iter()
        .map(|r| r[SLOT_EPOCH])
        .fold(f64::NEG_INFINITY, f64::max);
    let mut out = Vec::with_capacity(FRAME_HEADER + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&response_epoch.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    Some(out)
}

pub fn parse_records(bytes: &[u8]) -> Option<Vec<[f64; 26]>> {
    if bytes.len() < FRAME_HEADER || bytes.get(0..3)? != MAGIC.as_slice() {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(15..19)?.try_into().ok()?) as usize;
    if bytes.len() != FRAME_HEADER + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = FRAME_HEADER;
    for _ in 0..n {
        let mut r = [0.0f64; 26];
        for slot in r.iter_mut() {
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
        }
        out.push(r);
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_records(bytes)?;
    let mut out = Vec::with_capacity(records.len());
    for r in records {
        let val = r[SLOT_VAL];
        let epoch = r[SLOT_EPOCH];
        let channel = r[SLOT_CHANNEL];
        if !channel.is_finite() || channel < 1.0 || channel > CHANNELS.len() as f64 {
            continue;
        }
        let comp = channel as u32;
        if comp as f64 != channel {
            continue;
        }
        if !epoch.is_finite() || !val.is_finite() {
            continue;
        }
        out.push((epoch, val, comp));
    }
    if out.is_empty() { None } else { Some(out) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Norm {
    FullyNormalized,
    Unnormalized,
}

pub const PRODUCT_TYPE_GRAVITY_FIELD: &str = "gravity_field";

pub const GFC_SYNTHESIS_CONVENTION: &str = "ICGEM gravity-field spherical-harmonic synthesis: V(r,theta,lambda) = (GM/r) * sum_n (a/r)^n * sum_m [C_nm cos(m*lambda) + S_nm sin(m*lambda)] * P_nm(cos theta), with GM = earth_gravity_constant [m^3/s^2] and a = radius [m]; `fully_normalized` is the geodesy 4*pi normalisation P_nm = sqrt((2-delta_m0)(2n+1)(n-m)!/(n+m)!) * P_nm_unnormalized (no Condon-Shortley phase) — verified at the tree by J2 = -sqrt(5) * C20 for the measured XGM2019e C20; the gravity anomaly uses the (n-1) radial weight, dg = (GM/r^2) * sum_{n>=2} (n-1) (a/r)^n * sum_m [...] * P_nm; the normal field's own low-degree coefficients are not in the ICGEM head, so the caller supplies the disturbing reduction";

pub const GFC_HEAD_MEASURED: &str = "XGM2019e (measured 2026-10-15, https://icgem.gfz.de/getmodel/gfc/8e01a2aba4f291ac0638b79a390155713b70fdc9b266981023c756839c11c9b8/XGM2019e.gfc): product_type gravity_field, earth_gravity_constant 3.986004415E+14 m^3/s^2, radius 6378136.3 m, max_degree 5540, norm fully_normalized, tide_system zero_tide";

pub const GFC_DEGREE_SEAM_MEASURED: &str = "XGM2019e degree provenance (measured 2026-10-18): the head carries max_degree 5540 and no 2190 boundary or provenance note; the DOI 10.5880/ICGEM.2019.007 abstract and Zingerle et al. 2020 (DOI 10.1007/s00190-020-01398-0) both say 'up to d/o 5399' and place topographic forward modelling 'beyond d/o 719'. A 2190 processing seam is not supported by anything measured. The publication does name 2190, but as a delivery truncation: 'provide the model pre-calculated at three different spectral resolutions: d/o 5540, 2190 and 760' (Sect. 4.3), and 'only been used up to a spectral resolution of d/o 2190 ... to be comparable with the resolution of EGM2008 and EIGEN6-C4' (Sect. 4.2). The same paper cuts the spheroidal model 'at d/o 5480' (Sect. 3.3), while the delivered spherical file carries nonzero coefficients at degree 5540; 5399 (abstract) is repeated nowhere in the body and remains an unresolved wording difference";

#[derive(Clone, Debug, PartialEq)]
pub struct GfcHead {
    pub product_type: Option<String>,
    pub modelname: String,
    pub gm_m3_s2: f64,
    pub radius_m: f64,
    pub max_degree: u32,
    pub norm: Norm,
    pub tide_system: Option<String>,
    pub epoch: Option<String>,
}

impl Norm {
    pub fn parse(tok: &str) -> Option<Norm> {
        match tok {
            "fully_normalized" => Some(Norm::FullyNormalized),
            "unnormalized" => Some(Norm::Unnormalized),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Norm::FullyNormalized => "fully_normalized",
            Norm::Unnormalized => "unnormalized",
        }
    }
}

pub fn parse_head(bytes: &[u8]) -> Option<(GfcHead, usize)> {
    let mut product_type: Option<String> = None;
    let mut modelname: Option<String> = None;
    let mut gm: Option<f64> = None;
    let mut radius: Option<f64> = None;
    let mut max_degree: Option<u32> = None;
    let mut norm: Option<Norm> = None;
    let mut norm_seen = false;
    let mut tide_system: Option<String> = None;
    let mut epoch: Option<String> = None;
    let mut in_head = false;
    let mut end_line = None;
    for (i, raw) in bytes.split(|&b| b == b'\n').enumerate() {
        let Some(line) = std::str::from_utf8(raw).ok().map(str::trim) else {
            continue;
        };
        if line.starts_with("begin_of_head") {
            in_head = true;
            continue;
        }
        if line.starts_with("end_of_head") {
            end_line = Some(i);
            break;
        }
        if !in_head {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(key), Some(value)) = (it.next(), it.next()) else {
            continue;
        };
        match key {
            "product_type" => product_type = Some(value.to_string()),
            "modelname" => modelname = Some(value.to_string()),
            "earth_gravity_constant" => gm = value.parse::<f64>().ok(),
            "radius" => radius = value.parse::<f64>().ok(),
            "max_degree" => max_degree = value.parse::<u32>().ok(),
            "norm" => {
                norm = Norm::parse(value);
                norm_seen = true;
            }
            "tide_system" => tide_system = Some(value.to_string()),
            "epoch" | "ref_epoch" => epoch = Some(value.to_string()),
            _ => {}
        }
    }
    let gm = gm?;
    let radius = radius?;
    if !gm.is_finite() || gm <= 0.0 || !radius.is_finite() || radius <= 0.0 {
        return None;
    }
    if norm_seen && norm.is_none() {
        return None;
    }
    let head = GfcHead {
        product_type,
        modelname: modelname?,
        gm_m3_s2: gm,
        radius_m: radius,
        max_degree: max_degree?,
        norm: norm.unwrap_or(Norm::FullyNormalized),
        tide_system,
        epoch,
    };
    Some((head, end_line?))
}

fn tri_index(n: u32, m: u32) -> usize {
    (n * (n + 1) / 2 + m) as usize
}

fn tri_len(nmax: u32) -> usize {
    (nmax as usize + 1) * (nmax as usize + 2) / 2
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GfcCoeff {
    pub n: u32,
    pub m: u32,
    pub c: f64,
    pub s: f64,
}

pub struct GfcModel {
    pub head: GfcHead,
    pub nmax: u32,
    c: Vec<f64>,
    s: Vec<f64>,
}

impl GfcModel {
    pub fn from_coeffs(head: GfcHead, coeffs: &[GfcCoeff]) -> Option<GfcModel> {
        let max_n = coeffs.iter().map(|k| k.n).max()?;
        let nmax = max_n.min(head.max_degree);
        let mut c = vec![0.0f64; tri_len(nmax)];
        let mut s = vec![0.0f64; tri_len(nmax)];
        let mut seen = false;
        for k in coeffs {
            if k.n > nmax || k.m > k.n {
                continue;
            }
            if !k.c.is_finite() || !k.s.is_finite() {
                continue;
            }
            let i = tri_index(k.n, k.m);
            c[i] = k.c;
            s[i] = k.s;
            seen = true;
        }
        if !seen {
            return None;
        }
        Some(GfcModel { head, nmax, c, s })
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<GfcModel> {
        let (head, end_line) = parse_head(bytes)?;
        let mut coeffs = Vec::new();
        for (i, raw) in bytes.split(|&b| b == b'\n').enumerate() {
            if i <= end_line {
                continue;
            }
            let Ok(line) = std::str::from_utf8(raw) else {
                continue;
            };
            let t: Vec<&str> = line.split_whitespace().collect();
            if t.len() < 5 || (t[0] != "gfc" && t[0] != "gfct") {
                continue;
            }
            let (Ok(n), Ok(m), Ok(c), Ok(s)) = (
                t[1].parse::<u32>(),
                t[2].parse::<u32>(),
                t[3].parse::<f64>(),
                t[4].parse::<f64>(),
            ) else {
                continue;
            };
            coeffs.push(GfcCoeff { n, m, c, s });
        }
        GfcModel::from_coeffs(head, &coeffs)
    }

    pub fn from_wire(head: GfcHead, bytes: &[u8]) -> Option<GfcModel> {
        let records = parse_records(bytes)?;
        let coeffs = coeffs_from_records(&records);
        GfcModel::from_coeffs(head, &coeffs)
    }

    pub fn harmonic_sums(&self, colat_deg: f64, lon_deg: f64, radius_m: f64) -> Option<GfcSums> {
        if !colat_deg.is_finite() || !(0.0..=180.0).contains(&colat_deg) {
            return None;
        }
        if !lon_deg.is_finite() || !radius_m.is_finite() || radius_m <= 0.0 {
            return None;
        }
        let theta = colat_deg.to_radians();
        let p = legendre(self.nmax as usize, theta, self.head.norm);
        let phi = lon_deg.to_radians();
        let ar = self.head.radius_m / radius_m;
        let mut sigma = 0.0;
        let mut weighted = 0.0;
        let mut arn = 1.0;
        for n in 0..=self.nmax {
            let mut ring = 0.0;
            for m in 0..=n {
                let i = tri_index(n, m);
                let (c, s) = (self.c[i], self.s[i]);
                if c == 0.0 && s == 0.0 {
                    continue;
                }
                let mp = m as f64 * phi;
                ring += (c * mp.cos() + s * mp.sin()) * p[n as usize][m as usize];
            }
            sigma += arn * ring;
            if n >= 2 {
                weighted += (n as f64 - 1.0) * arn * ring;
            }
            arn *= ar;
        }
        if !sigma.is_finite() || !weighted.is_finite() {
            return None;
        }
        Some(GfcSums { sigma, weighted })
    }

    pub fn evaluate(
        &self,
        radius_m: f64,
        colat_deg: f64,
        lon_deg: f64,
        functional: Functional,
    ) -> Option<f64> {
        let sums = self.harmonic_sums(colat_deg, lon_deg, radius_m)?;
        let gm = self.head.gm_m3_s2;
        match functional {
            Functional::Potential => Some(gm / radius_m * sums.sigma),
            Functional::GravityAnomaly => Some(gm / (radius_m * radius_m) * sums.weighted),
            Functional::GeoidHeight => None,
        }
    }
}

pub const GFC_DERIVED_CLASS: &str = "the spherical-harmonic derived class evaluates one named functional at a point: evaluate(r, colat, lon, funktional), the measured Rat signature (M4.1, 2026-10-15). Potential V = GM/r * sigma [m^2/s^2] and GravityAnomaly dg = GM/r^2 * weighted [m/s^2] (the (n-1)-weighted radial form; the normal-field reduction is the caller's, per GFC_SYNTHESIS_CONVENTION) stand on the coefficients; GeoidHeight N = T/gamma0 [m] is gated on the GRS80 normal field (a, f, GM, omega) and gamma0 and returns None (pending) until they are registered, never a fabricated value. The named functionals are never merged into one value (M4.4), and the class stays separate from the seven point kernels: the synthesis is a sum over global basis functions and has no source point (M4.1).";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Functional {
    Potential,
    GravityAnomaly,
    GeoidHeight,
}

impl Functional {
    pub fn name(self) -> &'static str {
        match self {
            Functional::Potential => "potential",
            Functional::GravityAnomaly => "gravity_anomaly",
            Functional::GeoidHeight => "geoid_height",
        }
    }

    pub fn unit(self) -> &'static str {
        match self {
            Functional::Potential => "m^2/s^2",
            Functional::GravityAnomaly => "m/s^2",
            Functional::GeoidHeight => "m",
        }
    }

    pub fn is_measurable(self) -> bool {
        !matches!(self, Functional::GeoidHeight)
    }
}

pub const FUNCTIONALS: &[Functional] = &[
    Functional::Potential,
    Functional::GravityAnomaly,
    Functional::GeoidHeight,
];

pub fn coeffs_from_records(records: &[[f64; 26]]) -> Vec<GfcCoeff> {
    let mut pending: std::collections::HashMap<(u32, u32), GfcCoeff> =
        std::collections::HashMap::new();
    for r in records {
        let comp = r[SLOT_CHANNEL];
        let n = r[SLOT_FREQ];
        let m = r[SLOT_BIN_WIDTH];
        if !comp.is_finite() || !n.is_finite() || !m.is_finite() {
            continue;
        }
        if comp < 1.0 || comp > 4.0 || n < 0.0 || m < 0.0 || m > n {
            continue;
        }
        let (n, m) = (n as u32, m as u32);
        if (comp as u32) <= 2 {
            let e = pending.entry((n, m)).or_insert(GfcCoeff {
                n,
                m,
                c: 0.0,
                s: 0.0,
            });
            if !r[SLOT_VAL].is_finite() {
                continue;
            }
            if comp < 1.5 {
                e.c = r[SLOT_VAL];
            } else {
                e.s = r[SLOT_VAL];
            }
        }
    }
    pending.into_values().collect()
}

pub const GFC_DELIVERY_GATE: &str = "a gfc frame is admitted only when the source line carries the measured model head (gm/radius/norm/max_degree) and every coefficient rides inside max_degree; a frame without the head, a coefficient beyond the degree, an unread frame or a coefficientless frame is refused; the norm is constrained to fully_normalized/unnormalized at parse time (an unknown token leaves the head absent, so it too is refused)";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryRefusal {
    FrameUnread,
    NoCoefficients,
    HeadAbsent,
    DegreeBeyondHead { n: u32, max_degree: u32 },
}

pub fn gate_delivery(
    head: Option<&crate::archivar::ModelHead>,
    bytes: &[u8],
) -> Result<Vec<GfcCoeff>, DeliveryRefusal> {
    let head = head.ok_or(DeliveryRefusal::HeadAbsent)?;
    let records = parse_records(bytes).ok_or(DeliveryRefusal::FrameUnread)?;
    let coeffs = coeffs_from_records(&records);
    if coeffs.is_empty() {
        return Err(DeliveryRefusal::NoCoefficients);
    }
    for k in &coeffs {
        if k.n > head.max_degree {
            return Err(DeliveryRefusal::DegreeBeyondHead {
                n: k.n,
                max_degree: head.max_degree,
            });
        }
    }
    Ok(coeffs)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GfcSums {
    pub sigma: f64,
    pub weighted: f64,
}

#[derive(Clone, Copy, Debug)]
struct DoubleDouble {
    hi: f64,
    lo: f64,
}

impl DoubleDouble {
    fn from_f64(x: f64) -> Self {
        Self { hi: x, lo: 0.0 }
    }

    fn to_f64(self) -> f64 {
        self.hi + self.lo
    }
}

fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    (s, (a - (s - bb)) + (b - bb))
}

fn dd_add(a: DoubleDouble, b: DoubleDouble) -> DoubleDouble {
    let (s, e) = two_sum(a.hi, b.hi);
    let (hi, lo) = two_sum(s, a.lo + b.lo + e);
    DoubleDouble { hi, lo }
}

fn dd_sub(a: DoubleDouble, b: DoubleDouble) -> DoubleDouble {
    dd_add(
        a,
        DoubleDouble {
            hi: -b.hi,
            lo: -b.lo,
        },
    )
}

fn dd_mul(a: DoubleDouble, b: DoubleDouble) -> DoubleDouble {
    let p = a.hi * b.hi;
    let e = a.hi.mul_add(b.hi, -p) + (a.hi * b.lo + a.lo * b.hi);
    let (hi, lo) = two_sum(p, e);
    DoubleDouble { hi, lo }
}

fn dd_div_f64(a: DoubleDouble, b: f64) -> DoubleDouble {
    let q1 = a.hi / b;
    let r = dd_sub(
        a,
        dd_mul(DoubleDouble::from_f64(q1), DoubleDouble::from_f64(b)),
    );
    DoubleDouble {
        hi: q1,
        lo: r.hi / b,
    }
}

fn dd_cos_theta(theta: f64) -> DoubleDouble {
    let half = 0.5 * theta;
    let two = DoubleDouble::from_f64(2.0);
    if half.abs() <= std::f64::consts::FRAC_PI_4 {
        let s = DoubleDouble::from_f64(half.sin());
        dd_sub(DoubleDouble::from_f64(1.0), dd_mul(two, dd_mul(s, s)))
    } else {
        let c = DoubleDouble::from_f64(half.cos());
        dd_add(DoubleDouble::from_f64(-1.0), dd_mul(two, dd_mul(c, c)))
    }
}

fn zonal_column(nmax: usize, theta: f64) -> Vec<DoubleDouble> {
    let x = dd_cos_theta(theta);
    let mut p = Vec::with_capacity(nmax + 1);
    p.push(DoubleDouble::from_f64(1.0));
    if nmax == 0 {
        return p;
    }
    p.push(x);
    for n in 2..=nmax {
        let nf = n as f64;
        let a = dd_mul(DoubleDouble::from_f64(2.0 * nf - 1.0), dd_mul(x, p[n - 1]));
        let b = dd_mul(DoubleDouble::from_f64(nf - 1.0), p[n - 2]);
        p.push(dd_div_f64(dd_sub(a, b), nf));
    }
    p
}

fn scaled_by_pow2(mut x: f64, mut e: i32) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return x;
    }
    let two_pow_1023 = f64::from_bits(0x7fe0_0000_0000_0000);
    let two_pow_m1022 = f64::from_bits(0x0010_0000_0000_0000);
    while e > 1023 {
        x *= two_pow_1023;
        e -= 1023;
        if !x.is_finite() {
            return x;
        }
    }
    while e < -1022 {
        x *= two_pow_m1022;
        e += 1022;
        if x == 0.0 {
            return x;
        }
    }
    x * f64::from_bits(((e + 1023) as u64) << 52)
}

pub const GFC_LEGENDRE_RECURRENCE: &str = "fully_normalized_legendre runs column by column over m with a mantissa/exponent pair (scaled_by_pow2); the sectorial seed Pbar_mm ~ m^(1/4) * sin^m(theta) falls below f64 around m ~ 2155 at 45 deg, and a quantised subnormal seed would be amplified by the forward recurrence in n (measured 1.3e21 at degree 3211, 2.3e271 at 5540); the column is rescaled so the returned values underflow only where the true function itself is below f64. The m = 0 column is the exception: it uses the unnormalised recurrence with integer coefficients, P_n = ((2n-1) x P_{n-1} - (n-1) P_{n-2})/n, in double-double arithmetic with x = cos(theta) carried as hi+lo via the half-angle identity, then multiplies by sqrt(2n+1) (measured 2026-10-18: at 0 deg/5540 the normalised-coefficient recurrence drifted to 8.167e-10 in the addition theorem, the integer f64 form to 2.2e-16. At 0.001 deg/5540 the integer f64 form still drifted to 1.7e-10 in Pbar_n0 and 3.4e-10 in the addition theorem because rounding cos(theta) to f64 loses 1e-17 of the 1.5e-10 distance from 1, amplified by dP/dx ~ n/sin(theta); the double-double form brings Pbar_n0 to 2.8e-17 and the zonal addition to 2.2e-16 at 0 deg; at 0.001 deg the full addition theorem is 6.7e-12 because the m >= 1 columns still use the rounded cosine, with unnormalised_legendre sharing the same zonal column)";

pub fn fully_normalized_legendre(nmax: usize, theta: f64) -> Vec<Vec<f64>> {
    let costh = theta.cos();
    let sinth = theta.sin();
    let mut p = vec![vec![0.0f64; nmax + 1]; nmax + 1];
    p[0][0] = 1.0;
    if nmax == 0 {
        return p;
    }
    let zonal = zonal_column(nmax, theta);
    for n in 1..=nmax {
        p[n][0] = (2.0 * n as f64 + 1.0).sqrt() * zonal[n].to_f64();
    }
    const LN_2: f64 = std::f64::consts::LN_2;
    let two_pow_500 = f64::from_bits(((500 + 1023) as u64) << 52);
    let two_pow_m500 = f64::from_bits(((1023 - 500) as u64) << 52);
    let mut log_seed = 0.0f64;
    for m in 1..=nmax {
        if m == 1 {
            log_seed = 0.5 * 3.0f64.ln() + sinth.ln();
        } else {
            let mf = m as f64;
            log_seed += 0.5 * ((2.0 * mf + 1.0) / (2.0 * mf)).ln() + sinth.ln();
        }
        if !log_seed.is_finite() {
            continue;
        }
        let mut exp = (log_seed / LN_2).floor() as i32;
        let mut q = (log_seed - exp as f64 * LN_2).exp();
        let mf = m as f64;
        p[m][m] = scaled_by_pow2(q, exp);
        if m == nmax {
            continue;
        }
        let mut q1 = (2.0 * mf + 3.0).sqrt() * costh * q;
        p[m + 1][m] = scaled_by_pow2(q1, exp);
        for n in (m + 2)..=nmax {
            let nf = n as f64;
            let a = (((2.0 * nf - 1.0) * (2.0 * nf + 1.0)) / ((nf - mf) * (nf + mf))).sqrt();
            let b = (((2.0 * nf + 1.0) * (nf + mf - 1.0) * (nf - mf - 1.0))
                / ((nf - mf) * (nf + mf) * (2.0 * nf - 3.0)))
                .sqrt();
            let mut q2 = a * costh * q1 - b * q;
            let mx = q1.abs().max(q2.abs());
            if mx > two_pow_500 {
                q1 *= two_pow_m500;
                q2 *= two_pow_m500;
                exp += 500;
            } else if mx < two_pow_m500 && mx > 0.0 {
                q1 *= two_pow_500;
                q2 *= two_pow_500;
                exp -= 500;
            }
            p[n][m] = scaled_by_pow2(q2, exp);
            q = q1;
            q1 = q2;
        }
    }
    p
}

#[allow(clippy::needless_range_loop)] // p[n][m] index-arithmetic recurrences; m/n are grid indices
pub fn unnormalized_legendre(nmax: usize, theta: f64) -> Vec<Vec<f64>> {
    let costh = theta.cos();
    let sinth = theta.sin();
    let mut p = vec![vec![0.0f64; nmax + 1]; nmax + 1];
    p[0][0] = 1.0;
    let zonal = zonal_column(nmax, theta);
    for n in 1..=nmax {
        p[n][0] = zonal[n].to_f64();
    }
    for m in 1..=nmax {
        p[m][m] = (2.0 * m as f64 - 1.0) * sinth * p[m - 1][m - 1];
    }
    for m in 1..nmax {
        p[m + 1][m] = (2.0 * m as f64 + 1.0) * costh * p[m][m];
    }
    for m in 1..nmax {
        for n in (m + 2)..=nmax {
            let nf = n as f64;
            let mf = m as f64;
            p[n][m] = ((2.0 * nf - 1.0) * costh * p[n - 1][m] - (nf + mf - 1.0) * p[n - 2][m])
                / (nf - mf);
        }
    }
    p
}

pub fn legendre(nmax: usize, theta: f64, norm: Norm) -> Vec<Vec<f64>> {
    match norm {
        Norm::FullyNormalized => fully_normalized_legendre(nmax, theta),
        Norm::Unnormalized => unnormalized_legendre(nmax, theta),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(epoch: f64, val: f64, channel: f64, n: f64, m: f64) -> [f64; 26] {
        let mut r = [0.0f64; 26];
        r[SLOT_VAL] = val;
        r[SLOT_EPOCH] = epoch;
        r[SLOT_CHANNEL] = channel;
        r[SLOT_FREQ] = n;
        r[SLOT_BIN_WIDTH] = m;
        r[SLOT_PRESENCE] = 1.0;
        r
    }

    #[test]
    fn frame_roundtrips_and_carries_the_wire_magic() {
        let records = vec![
            record(0.0, 1.0, 1.0, 0.0, 0.0),
            record(0.0, -4.84e-4, 2.0, 2.0, 0.0),
        ];
        let bytes = write_bin(&records).expect("non-empty frames");
        assert_eq!(&bytes[0..3], MAGIC.as_slice());
        assert_eq!(bytes.len(), FRAME_HEADER + 2 * RECORD_BYTES);
        assert_eq!(parse_records(&bytes), Some(records));
        assert!(write_bin(&[]).is_none());
    }

    #[test]
    fn series_folds_each_coefficient_onto_its_channel() {
        let bytes = write_bin(&[record(0.0, 2.44e-6, 1.0, 2.0, 2.0)]).expect("frames");
        assert_eq!(parse_series(&bytes), Some(vec![(0.0, 2.44e-6, 1)]));
    }

    #[test]
    fn a_foreign_or_truncated_frame_is_void() {
        assert!(parse_records(b"XXXX").is_none());
        let good = write_bin(&[record(1.0, 1.0, 1.0, 0.0, 0.0)]).expect("frames");
        assert!(parse_records(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn channels_map_to_their_functional_and_letter() {
        assert_eq!(channel_of(FUNC_GFC, false), Some(1));
        assert_eq!(channel_of(FUNC_GFC, true), Some(2));
        assert_eq!(channel_of(FUNC_GFCT, false), Some(3));
        assert_eq!(channel_of(FUNC_GFCT, true), Some(4));
        assert_eq!(channel_of(9, false), None);
    }

    #[test]
    fn declared_fields_carry_gravity_and_the_read_site_names() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), CHANNELS.len());
        assert_eq!(fields[0].name, "icgem_gfc_c");
        assert_eq!(fields[0].unit, "1");
        assert_eq!(fields[0].channel_ref, Some(force_id_of("gravity").unwrap()));
        assert_eq!(component_name(4), Some("icgem_gfct_s"));
        assert_eq!(component_name(9), None);
    }

    const MEASURED_HEAD: &str = "begin_of_head =====\n\
product_type                gravity_field\n\
modelname                   XGM2019e\n\
earth_gravity_constant      3.986004415E+14\n\
radius                      6378136.3\n\
max_degree                  5540\n\
errors                      formal\n\
norm                        fully_normalized\n\
tide_system                 zero_tide\n\
key      L     M                        C                        S                  sigma C                  sigma S\n\
end_of_head =====\n\
gfc      0     0    1.000000000000000E+00    0.000000000000000E+00    0.000000000000000E+00    0.000000000000000E+00\n\
gfc      2     0   -4.841694947475625E-04    0.000000000000000E+00    6.678382494792630E-13    0.000000000000000E+00\n\
gfc      2     2    2.439370331756250E-06   -1.400307649958125E-06    8.117529944828666E-13    9.254489422410342E-13\n";

    #[test]
    fn parse_head_reads_the_measured_xgm2019e_head() {
        let (head, end_line) = parse_head(MEASURED_HEAD.as_bytes()).expect("the head parses");
        assert_eq!(
            head.product_type.as_deref(),
            Some(PRODUCT_TYPE_GRAVITY_FIELD)
        );
        assert_eq!(head.modelname, "XGM2019e");
        assert_eq!(head.gm_m3_s2, 3.986004415E14);
        assert!((head.radius_m - 6378136.3).abs() < 1e-6);
        assert_eq!(head.max_degree, 5540);
        assert_eq!(head.norm, Norm::FullyNormalized);
        assert_eq!(head.tide_system.as_deref(), Some("zero_tide"));
        assert_eq!(end_line, 10);
    }

    #[test]
    fn an_unnamed_or_unmeasured_head_stays_absent() {
        let text = "begin_of_head\nmodelname M\nradius 6371000\nmax_degree 2\nend_of_head\n";
        assert!(parse_head(text.as_bytes()).is_none());
        let text = "begin_of_head\nmodelname M\nearth_gravity_constant 3.9E14\nradius 6371000\nmax_degree 2\nnorm schmidt\nend_of_head\n";
        assert!(parse_head(text.as_bytes()).is_none());
    }

    #[test]
    fn fully_normalized_legendre_matches_the_closed_forms() {
        let theta = 1.1f64;
        let (c, s) = (theta.cos(), theta.sin());
        let p = fully_normalized_legendre(2, theta);
        assert!((p[0][0] - 1.0).abs() < 1e-15);
        assert!((p[1][0] - 3f64.sqrt() * c).abs() < 1e-15);
        assert!((p[1][1] - 3f64.sqrt() * s).abs() < 1e-15);
        assert!((p[2][0] - 5f64.sqrt() * (3.0 * c * c - 1.0) / 2.0).abs() < 1e-14);
        assert!((p[2][1] - 15f64.sqrt() * s * c).abs() < 1e-14);
        assert!((p[2][2] - 15f64.sqrt() / 2.0 * s * s).abs() < 1e-14);
    }

    const MP_REF: &[(usize, usize, f64, f64)] = &[
        (85, 0, 45.0, -1.240_460_375_601_274_2),
        (85, 1, 45.0, -0.733_937_868_587_970_2),
        (85, 42, 45.0, 1.475_070_386_880_264_2),
        (85, 85, 45.0, 7.349_820_037_226_633e-13),
        (85, 0, 89.9, 0.167_760_417_724_461_74),
        (85, 1, 89.9, 1.578_078_032_111_302_6),
        (85, 42, 89.9, -0.221_623_109_596_276_43),
        (85, 85, 89.9, 4.570_832_390_217_962),
        (100, 0, 45.0, -1.239_077_344_447_987_2),
        (100, 1, 45.0, 0.719_698_546_670_530_3),
        (100, 50, 45.0, 1.671_356_558_707_947_3),
        (100, 100, 45.0, 4.227_215_051_064_45e-15),
        (100, 0, 89.9, 1.111_058_682_100_732_7),
        (100, 1, 89.9, -0.278_468_858_106_332_15),
        (100, 50, 89.9, -1.693_532_440_208_351_1),
        (100, 100, 89.9, 4.758_696_185_753_562),
        (2190, 0, 45.0, -0.513_584_648_405_589_2),
        (2190, 1, 45.0, -1.753_370_618_062_240_4),
        (2190, 1095, 45.0, 2.190_809_170_169_514_7),
        (2190, 2190, 45.0, 0.0),
        (2190, 0, 89.9, 0.876_295_874_357_907_6),
        (2190, 1, 89.9, -1.005_332_330_848_525_8),
        (2190, 1095, 89.9, 0.289_418_666_210_955_85),
        (2190, 2190, 89.9, 10.243_352_470_919_65),
        (3211, 0, 45.0, -0.513_465_657_577_726_6),
        (3211, 1, 45.0, 1.753_161_435_501_976_2),
        (3211, 1605, 45.0, -2.060_105_296_791_817),
        (3211, 3211, 45.0, 0.0),
        (3211, 0, 89.9, 0.707_814_303_922_252_6),
        (3211, 1, 89.9, -1.242_771_160_852_704_1),
        (3211, 1605, 89.9, -0.243_587_973_843_969_74),
        (3211, 3211, 89.9, 11.253_924_186_718_486),
        (5540, 0, 45.0, -1.239_720_674_032_834_6),
        (5540, 1, 45.0, 0.726_099_688_549_371),
        (5540, 2770, 45.0, -2.116_574_908_348_008),
        (5540, 5540, 45.0, 0.0),
        (5540, 0, 89.9, -1.094_623_667_513_928_3),
        (5540, 1, 89.9, 0.387_402_664_135_626_46),
        (5540, 2770, 89.9, 0.853_109_637_978_650_5),
        (5540, 5540, 89.9, 12.851_982_633_058_219),
    ];

    #[test]
    fn fully_normalized_legendre_holds_from_degree_85_to_5540() {
        for &nmax in &[85usize, 100, 2190, 3211, 5540] {
            for &colat in &[0.0f64, 45.0, 89.9] {
                let p = fully_normalized_legendre(nmax, colat.to_radians());
                let mut max_abs = 0.0f64;
                let mut zeros = 0usize;
                for (n, row) in p.iter().enumerate() {
                    for (m, &v) in row.iter().enumerate().take(n + 1) {
                        assert!(
                            v.is_finite(),
                            "P[{n}][{m}] not finite at nmax {nmax}, colat {colat}: {v}"
                        );
                        if v == 0.0 {
                            zeros += 1;
                        }
                        max_abs = max_abs.max(v.abs());
                    }
                }
                let mut worst_rel = 0.0f64;
                let mut worst_n = 0usize;
                for n in 0..=nmax {
                    let sum: f64 = p[n][..=n].iter().map(|v| v * v).sum();
                    let expected = (2 * n + 1) as f64;
                    let rel = (sum - expected).abs() / expected;
                    if rel > worst_rel {
                        worst_rel = rel;
                        worst_n = n;
                    }
                }
                let mut worst_ref = 0.0f64;
                for &(rn, rm, rc, expected) in MP_REF {
                    if rn != nmax || rc != colat {
                        continue;
                    }
                    let got = p[rn][rm];
                    if expected.abs() < 1e-300 {
                        assert_eq!(
                            got, 0.0,
                            "P[{rn}][{rm}] at colat {colat} must underflow to zero, got {got}"
                        );
                    } else {
                        let rel = (got - expected).abs() / expected.abs();
                        worst_ref = worst_ref.max(rel);
                        assert!(
                            rel <= 1e-10,
                            "P[{rn}][{rm}] at colat {colat}: got {got}, mpmath {expected}, \
                             rel {rel:.3e}"
                        );
                    }
                }
                eprintln!(
                    "nmax={nmax:5} colat={colat:4.1} max|P|={max_abs:.6e} zeros={zeros:8} \
                     addition_rel={worst_rel:.3e} at n={worst_n} ref_rel={worst_ref:.3e}"
                );
                assert!(
                    worst_rel <= 1e-8,
                    "addition theorem fails at nmax {nmax}, colat {colat}: \
                     rel {worst_rel:.3e} at n {worst_n}"
                );
            }
        }
    }

    #[test]
    fn the_factorial_boundary_is_measured_below_degree_86() {
        let f170: f64 = (1..=170).map(|k| k as f64).product();
        assert!(f170.is_finite() && f170 <= f64::MAX);
        assert!((f170 * 171.0).is_infinite());
        let theta = std::f64::consts::FRAC_PI_4;
        for m in [85usize, 86, 100] {
            let p = fully_normalized_legendre(m, theta);
            assert!(p[m][m].is_finite(), "P[{m}][{m}] must stay finite");
        }
    }

    #[test]
    fn the_zonal_column_holds_the_addition_theorem_at_degree_5540() {
        let nmax = 5540usize;
        let p = fully_normalized_legendre(nmax, 0.0);
        let mut worst_rel = 0.0f64;
        let mut worst_n = 0usize;
        for n in 0..=nmax {
            let sum = p[n][0] * p[n][0];
            let expected = (2 * n + 1) as f64;
            let rel = (sum - expected).abs() / expected;
            if rel > worst_rel {
                worst_rel = rel;
                worst_n = n;
            }
            for m in 1..=n {
                assert_eq!(
                    p[n][m], 0.0,
                    "P[{n}][{m}] at the pole must be exactly zero, got {}",
                    p[n][m]
                );
            }
        }
        eprintln!("zonal 0 deg: addition_rel={worst_rel:.3e} at n={worst_n}");
        assert!(
            worst_rel <= 1e-14,
            "zonal addition theorem drifted: rel {worst_rel:.3e} at n {worst_n}"
        );
    }

    #[test]
    fn the_unnormalized_recurrence_overflows_before_degree_162_at_45_deg() {
        let theta = std::f64::consts::FRAC_PI_4;
        let p = unnormalized_legendre(200, theta);
        let first = (1..=200)
            .find(|&m| !p[m][m].is_finite())
            .expect("the unnormalized diagonal overflows below degree 201");
        assert!(
            (155..=165).contains(&first),
            "first unnormalized overflow at m = {first}, expected near 161"
        );
        assert!(p[first - 1][first - 1].is_finite());

        let head = GfcHead {
            product_type: Some(PRODUCT_TYPE_GRAVITY_FIELD.to_string()),
            modelname: "unnormalized-overflow".to_string(),
            gm_m3_s2: 3.986004415E14,
            radius_m: 6378136.3,
            max_degree: 200,
            norm: Norm::Unnormalized,
            tide_system: None,
            epoch: None,
        };
        let model = GfcModel::from_coeffs(
            head,
            &[GfcCoeff {
                n: 200,
                m: 200,
                c: 1.0,
                s: 0.0,
            }],
        )
        .expect("the unnormalized probe model builds");
        assert!(
            model.harmonic_sums(45.0, 0.0, 6378136.3).is_none(),
            "an overflowed unnormalized synthesis must decline, not fabricate"
        );
    }

    #[test]
    fn fully_normalized_legendre_equals_the_definition_times_unnormalized() {
        let theta = 0.7f64;
        let nmax = 14usize;
        let pn = fully_normalized_legendre(nmax, theta);
        let pu = unnormalized_legendre(nmax, theta);
        for n in 0..=nmax {
            for m in 0..=n {
                let delta: f64 = if m == 0 { 1.0 } else { 2.0 };
                let ln_n = 0.5
                    * ((2.0 * n as f64 + 1.0).ln() + delta.ln() + ln_factorial(n - m)
                        - ln_factorial(n + m));
                let expected = ln_n.exp() * pu[n][m];
                assert!(
                    (pn[n][m] - expected).abs() <= 1e-12 * (1.0 + expected.abs()),
                    "P[{n}][{m}] {} vs definition {expected}",
                    pn[n][m]
                );
            }
        }
    }

    fn ln_factorial(k: usize) -> f64 {
        (1..=k).map(|i| (i as f64).ln()).sum()
    }

    #[test]
    fn a_degree_two_zonal_model_reproduces_the_j2_relation() {
        let head = GfcHead {
            product_type: Some(PRODUCT_TYPE_GRAVITY_FIELD.to_string()),
            modelname: "zonal".to_string(),
            gm_m3_s2: 3.986004415E14,
            radius_m: 6378136.3,
            max_degree: 2,
            norm: Norm::FullyNormalized,
            tide_system: Some("zero_tide".to_string()),
            epoch: None,
        };
        let c20 = -4.841694947475625E-04;
        let model = GfcModel::from_coeffs(
            head,
            &[GfcCoeff {
                n: 2,
                m: 0,
                c: c20,
                s: 0.0,
            }],
        )
        .expect("the zonal model builds");
        let r = model.head.radius_m;
        let colat = 90.0;
        let sums = model.harmonic_sums(colat, 0.0, r).expect("the sums stand");
        let expected = (model.head.radius_m / r).powi(2) * c20 * (-5f64.sqrt() / 2.0);
        assert!((sums.sigma - expected).abs() < 1e-18);
        assert!((sums.weighted - expected).abs() < 1e-18);
        let j2 = -5f64.sqrt() * c20;
        assert!((1.08260e-3..=1.08266e-3).contains(&j2), "j2 {j2}");
        assert!((j2 - 1.08262668e-3).abs() > 5e-9, "j2 {j2}");
    }

    #[test]
    fn synthesis_declines_nonfinite_and_out_of_range_input() {
        let head = GfcHead {
            product_type: Some(PRODUCT_TYPE_GRAVITY_FIELD.to_string()),
            modelname: "probe".to_string(),
            gm_m3_s2: 3.986004415E14,
            radius_m: 6378136.3,
            max_degree: 2,
            norm: Norm::FullyNormalized,
            tide_system: None,
            epoch: None,
        };
        let model = GfcModel::from_coeffs(
            head,
            &[GfcCoeff {
                n: 0,
                m: 0,
                c: 1.0,
                s: 0.0,
            }],
        )
        .expect("builds");
        assert!(model.harmonic_sums(f64::NAN, 0.0, 6378136.3).is_none());
        assert!(model.harmonic_sums(181.0, 0.0, 6378136.3).is_none());
        assert!(model.harmonic_sums(0.0, 0.0, 0.0).is_none());
    }

    #[test]
    fn a_wire_model_roundtrips_from_the_bin() {
        let head = GfcHead {
            product_type: Some(PRODUCT_TYPE_GRAVITY_FIELD.to_string()),
            modelname: "wire".to_string(),
            gm_m3_s2: 3.986004415E14,
            radius_m: 6378136.3,
            max_degree: 2,
            norm: Norm::FullyNormalized,
            tide_system: None,
            epoch: None,
        };
        let records = vec![
            record(0.0, 1.0, 1.0, 0.0, 0.0),
            record(0.0, -4.841694947475625E-04, 1.0, 2.0, 0.0),
            record(0.0, -1.400307649958125E-06, 2.0, 2.0, 2.0),
        ];
        let bytes = write_bin(&records).expect("frames");
        let model = GfcModel::from_wire(head, &bytes).expect("the wire model parses");
        let sums = model.harmonic_sums(90.0, 0.0, model.head.radius_m).unwrap();
        assert!(sums.sigma.is_finite());
    }

    fn measured_model_head(max_degree: u32) -> crate::archivar::ModelHead {
        crate::archivar::ModelHead {
            gm_m3_s2: 3.986004415E14,
            radius_m: 6378136.3,
            norm: Norm::FullyNormalized,
            tide_system: Some("zero_tide".to_string()),
            max_degree,
        }
    }

    #[test]
    fn the_delivery_gate_refuses_a_frame_without_a_head() {
        let bytes = write_bin(&[record(0.0, 1.0, 1.0, 0.0, 0.0)]).expect("frames");
        assert_eq!(
            gate_delivery(None, &bytes),
            Err(DeliveryRefusal::HeadAbsent)
        );
    }

    #[test]
    fn the_delivery_gate_admits_a_frame_within_the_head() {
        let bytes = write_bin(&[
            record(0.0, 1.0, 1.0, 0.0, 0.0),
            record(0.0, -4.841694947475625E-04, 1.0, 2.0, 0.0),
        ])
        .expect("frames");
        let head = measured_model_head(2);
        let coeffs = gate_delivery(Some(&head), &bytes).expect("the frame is within the head");
        assert!(coeffs.iter().any(|k| k.n == 2 && k.m == 0));
    }

    #[test]
    fn the_delivery_gate_refuses_a_coefficient_beyond_the_head() {
        let bytes =
            write_bin(&[record(0.0, -4.841694947475625E-04, 1.0, 3.0, 0.0)]).expect("frames");
        let head = measured_model_head(2);
        assert_eq!(
            gate_delivery(Some(&head), &bytes),
            Err(DeliveryRefusal::DegreeBeyondHead {
                n: 3,
                max_degree: 2
            })
        );
    }

    #[test]
    fn the_delivery_gate_refuses_an_unread_frame_and_an_empty_one() {
        let head = measured_model_head(2);
        assert_eq!(
            gate_delivery(Some(&head), b"XXXX"),
            Err(DeliveryRefusal::FrameUnread)
        );
        let mut empty = Vec::new();
        empty.extend_from_slice(&MAGIC);
        empty.extend_from_slice(&0.0f64.to_le_bytes());
        empty.extend_from_slice(&0u32.to_le_bytes());
        empty.extend_from_slice(&0u32.to_le_bytes());
        assert_eq!(empty.len(), FRAME_HEADER);
        assert_eq!(
            gate_delivery(Some(&head), &empty),
            Err(DeliveryRefusal::NoCoefficients)
        );
    }

    #[test]
    fn only_the_two_documented_norms_are_known() {
        assert_eq!(Norm::parse("fully_normalized"), Some(Norm::FullyNormalized));
        assert_eq!(Norm::parse("unnormalized"), Some(Norm::Unnormalized));
        assert_eq!(Norm::parse("schmidt_semi_normalized"), None);
    }

    const JEOD_GGM02C_DEGREE10: &str = "begin_of_head\n\
        product_type gravity_field\n\
        modelname GGM02C_deg10\n\
        earth_gravity_constant 3.986004415E+14\n\
        radius 6378136.3\n\
        max_degree 10\n\
        norm fully_normalized\n\
        end_of_head\n\
        gfc 2 0 -4.8416938905481E-04 +0.0000000000000E+00\n\
        gfc 2 1 -2.0458338184745E-10 +1.3968195379551E-09\n\
        gfc 2 2 +2.4393233001191E-06 -1.4002662003867E-06\n\
        gfc 3 0 +9.5718508415439E-07 +0.0000000000000E+00\n\
        gfc 3 1 +2.0304752656064E-06 +2.4817416903031E-07\n\
        gfc 3 2 +9.0480066975068E-07 -6.1900441427103E-07\n\
        gfc 3 3 +7.2128924247650E-07 +1.4143556434052E-06\n\
        gfc 4 0 +5.3999143526074E-07 +0.0000000000000E+00\n\
        gfc 4 1 -5.3617583789434E-07 -4.7356802287476E-07\n\
        gfc 4 2 +3.5051159931087E-07 +6.6243944849186E-07\n\
        gfc 4 3 +9.9085503541734E-07 -2.0097529442342E-07\n\
        gfc 4 4 -1.8846750474516E-07 +3.0882228278756E-07\n\
        gfc 5 0 +6.8715981001179E-08 +0.0000000000000E+00\n\
        gfc 5 1 -6.2904051380481E-08 -9.4373263356928E-08\n\
        gfc 5 2 +6.5210393080303E-07 -3.2334838450788E-07\n\
        gfc 5 3 -4.5187965449909E-07 -2.1500140801223E-07\n\
        gfc 5 4 -2.9533633996919E-07 +4.9817834613976E-08\n\
        gfc 5 5 +1.7479367861529E-07 -6.6937444851133E-07\n\
        gfc 6 0 -1.4994011973125E-07 +0.0000000000000E+00\n\
        gfc 6 1 -7.5903534713091E-08 +2.6516969820404E-08\n\
        gfc 6 2 +4.8671477410889E-08 -3.7379063632786E-07\n\
        gfc 6 3 +5.7235251140830E-08 +8.9355908317172E-09\n\
        gfc 6 4 -8.6024306020762E-08 -4.7142492778928E-07\n\
        gfc 6 5 -2.6717044535375E-07 -5.3648870593762E-07\n\
        gfc 6 6 +9.4667858117668E-09 -2.3740563878695E-07\n\
        gfc 7 0 +9.0504630229132E-08 +0.0000000000000E+00\n\
        gfc 7 1 +2.8088898145081E-07 +9.5119644062781E-08\n\
        gfc 7 2 +3.3041560746535E-07 +9.2985239787248E-08\n\
        gfc 7 3 +2.5045166263142E-07 -2.1714748211003E-07\n\
        gfc 7 4 -2.7498938059553E-07 -1.2406704411494E-07\n\
        gfc 7 5 +1.6601317328580E-09 +1.7933888039325E-08\n\
        gfc 7 6 -3.5880775879961E-07 +1.5179454735699E-07\n\
        gfc 7 7 +1.5062884219099E-09 +2.4116259277940E-08\n\
        gfc 8 0 +4.9481334104780E-08 +0.0000000000000E+00\n\
        gfc 8 1 +2.3159979727734E-08 +5.8896665124862E-08\n\
        gfc 8 2 +8.0015152191272E-08 +6.5278835020898E-08\n\
        gfc 8 3 -1.9378833774969E-08 -8.5977473175173E-08\n\
        gfc 8 4 -2.4436744168468E-07 +6.9808169573661E-08\n\
        gfc 8 5 -2.5695381020603E-08 +8.9195503483206E-08\n\
        gfc 8 6 -6.5962471839471E-08 +3.0894479673499E-07\n\
        gfc 8 7 +6.7261439849585E-08 +7.4875670873080E-08\n\
        gfc 8 8 -1.2403873503204E-07 +1.2055330686769E-07\n\
        gfc 9 0 +2.8023188203329E-08 +0.0000000000000E+00\n\
        gfc 9 1 +1.4215021812885E-07 +2.1399410624745E-08\n\
        gfc 9 2 +2.1413308310842E-08 -3.1693757099588E-08\n\
        gfc 9 3 -1.6061777555625E-07 -7.4288065727748E-08\n\
        gfc 9 4 -9.3744494781812E-09 +1.9903426483849E-08\n\
        gfc 9 5 -1.6310476153586E-08 -5.4041923229450E-08\n\
        gfc 9 6 +6.2782214351230E-08 +2.2295178138467E-07\n\
        gfc 9 7 -1.1797705231858E-07 -9.6928434857123E-08\n\
        gfc 9 8 +1.8814003080096E-07 -3.0006845470535E-09\n\
        gfc 9 9 -4.7556714055660E-08 +9.6880058387687E-08\n\
        gfc 10 0 +5.3320010526550E-08 +0.0000000000000E+00\n\
        gfc 10 1 +8.3764843251410E-08 -1.3109151284691E-07\n\
        gfc 10 2 -9.3986202660537E-08 -5.1279822748794E-08\n\
        gfc 10 3 -7.0166481788361E-09 -1.5414522056325E-07\n\
        gfc 10 4 -8.4472635586329E-08 -7.9031973795022E-08\n\
        gfc 10 5 -4.9292034854895E-08 -5.0611717909905E-08\n\
        gfc 10 6 -3.7585195034396E-08 -7.9769574266997E-08\n\
        gfc 10 7 +8.2601322177926E-09 -3.0472813339748E-09\n\
        gfc 10 8 +4.0596116481898E-08 -9.1718696376024E-08\n\
        gfc 10 9 +1.2538272379586E-07 -3.7954367789185E-08\n\
        gfc 10 10 +1.0042456986404E-07 -2.3859595666514E-08\n";

    #[test]
    fn the_synthesis_matches_the_nasa_jeod_ggm02c_degree10_potential() {
        let model =
            GfcModel::from_bytes(JEOD_GGM02C_DEGREE10.as_bytes()).expect("the GGM02C head parses");
        let (x, y, z) = (
            -6404550.640081473_f64,
            -1493096.048926443_f64,
            6158937.856787768_f64,
        );
        let r = (x * x + y * y + z * z).sqrt();
        let colat = (z / r).clamp(-1.0, 1.0).acos().to_degrees();
        let lon = y.atan2(x).to_degrees();
        let value = model
            .evaluate(r, colat, lon, Functional::Potential)
            .expect("the potential stands");
        let expected = -4.843_731_614_932_75E3;
        assert!(
            (value - expected).abs() <= 1e-9 * expected.abs(),
            "JEOD case 13: computed {value} vs reference {expected}"
        );
    }

    #[test]
    fn the_functionals_carry_their_own_units_and_the_geoid_stays_pending() {
        assert_eq!(Functional::Potential.unit(), "m^2/s^2");
        assert_eq!(Functional::GravityAnomaly.unit(), "m/s^2");
        assert_eq!(Functional::GeoidHeight.unit(), "m");
        assert!(Functional::Potential.is_measurable());
        assert!(Functional::GravityAnomaly.is_measurable());
        assert!(!Functional::GeoidHeight.is_measurable());
        assert_eq!(FUNCTIONALS.len(), 3);

        let model =
            GfcModel::from_bytes(JEOD_GGM02C_DEGREE10.as_bytes()).expect("the GGM02C head parses");
        let (x, y, z) = (
            -6404550.640081473_f64,
            -1493096.048926443_f64,
            6158937.856787768_f64,
        );
        let r = (x * x + y * y + z * z).sqrt();
        let colat = (z / r).clamp(-1.0, 1.0).acos().to_degrees();
        let lon = y.atan2(x).to_degrees();
        let potential = model
            .evaluate(r, colat, lon, Functional::Potential)
            .expect("the potential stands");
        let anomaly = model
            .evaluate(r, colat, lon, Functional::GravityAnomaly)
            .expect("the anomaly stands");
        assert!(potential.is_finite() && potential < 0.0);
        assert!(anomaly.is_finite() && anomaly.abs() > 0.0);
        assert!(
            model
                .evaluate(r, colat, lon, Functional::GeoidHeight)
                .is_none(),
            "N without the GRS80 normal field is pending, never a value"
        );
    }

    #[test]
    fn the_zonal_column_holds_near_the_pole() {
        let nmax = 5540usize;
        let refs = [
            (0.0f64, 105.266_328_899_605_88_f64),
            (0.001, 105.020_389_384_116_48),
            (0.01, 82.059_368_706_961_11),
            (0.1, -22.968_183_890_069_593),
            (1.0, -0.807_659_192_519_504_9),
        ];
        for (colat, expected) in refs {
            let theta = colat.to_radians();
            let p = fully_normalized_legendre(nmax, theta);
            let got = p[nmax][0];
            let rel = (got - expected).abs() / expected.abs();
            let u = unnormalized_legendre(nmax, theta);
            let expected_u = expected / (2.0 * nmax as f64 + 1.0).sqrt();
            let rel_u = (u[nmax][0] - expected_u).abs() / expected_u.abs();
            let mut worst_sum = 0.0f64;
            let mut worst_n = 0usize;
            for n in 0..=nmax {
                let sum: f64 = p[n][..=n].iter().map(|v| v * v).sum();
                let expected_sum = (2 * n + 1) as f64;
                let rel_sum = (sum - expected_sum).abs() / expected_sum;
                if rel_sum > worst_sum {
                    worst_sum = rel_sum;
                    worst_n = n;
                }
            }
            eprintln!(
                "colat={colat:6} zonal_rel={rel:.3e} unnormalised_rel={rel_u:.3e} \
                 addition_rel={worst_sum:.3e} at n={worst_n}"
            );
            assert!(
                rel <= 1e-12,
                "Pbar[{nmax}][0] at colat {colat}: got {got}, mpmath {expected}, rel {rel:.3e}"
            );
            assert!(
                rel_u <= 1e-12,
                "unnormalised P[{nmax}][0] at colat {colat}: got {}, mpmath {expected_u}, \
                 rel {rel_u:.3e}",
                u[nmax][0]
            );
            assert!(
                worst_sum <= 1e-9,
                "addition theorem fails at colat {colat}: rel {worst_sum:.3e} at n {worst_n}"
            );
        }
    }
}
