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
    ((nmax as usize + 1) * (nmax as usize + 2) / 2) as usize
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

    pub fn synthesize(&self, colat_deg: f64, lon_deg: f64, radius_m: f64) -> Option<GfcValue> {
        let sums = self.harmonic_sums(colat_deg, lon_deg, radius_m)?;
        let gm = self.head.gm_m3_s2;
        Some(GfcValue {
            potential: gm / radius_m * sums.sigma,
            anomaly: gm / (radius_m * radius_m) * sums.weighted,
        })
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GfcValue {
    pub potential: f64,
    pub anomaly: f64,
}

pub fn fully_normalized_legendre(nmax: usize, theta: f64) -> Vec<Vec<f64>> {
    let costh = theta.cos();
    let sinth = theta.sin();
    let mut p = vec![vec![0.0f64; nmax + 1]; nmax + 1];
    p[0][0] = 1.0;
    if nmax >= 1 {
        p[1][0] = 3.0f64.sqrt() * costh;
        p[1][1] = 3.0f64.sqrt() * sinth;
    }
    for n in 2..=nmax {
        let nf = n as f64;
        p[n][n] = ((2.0 * nf + 1.0) / (2.0 * nf)).sqrt() * sinth * p[n - 1][n - 1];
        p[n][n - 1] = (2.0 * nf + 1.0).sqrt() * costh * p[n - 1][n - 1];
        for m in 0..=(n - 2) {
            let mf = m as f64;
            let a = (((2.0 * nf - 1.0) * (2.0 * nf + 1.0)) / ((nf - mf) * (nf + mf))).sqrt();
            let b = (((2.0 * nf + 1.0) * (nf + mf - 1.0) * (nf - mf - 1.0))
                / ((nf - mf) * (nf + mf) * (2.0 * nf - 3.0)))
                .sqrt();
            p[n][m] = a * costh * p[n - 1][m] - b * p[n - 2][m];
        }
    }
    p
}

pub fn unnormalized_legendre(nmax: usize, theta: f64) -> Vec<Vec<f64>> {
    let costh = theta.cos();
    let sinth = theta.sin();
    let mut p = vec![vec![0.0f64; nmax + 1]; nmax + 1];
    p[0][0] = 1.0;
    for m in 1..=nmax {
        p[m][m] = (2.0 * m as f64 - 1.0) * sinth * p[m - 1][m - 1];
    }
    for m in 0..nmax {
        p[m + 1][m] = (2.0 * m as f64 + 1.0) * costh * p[m][m];
    }
    for m in 0..nmax {
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
            .synthesize(colat, lon, r)
            .expect("the potential stands");
        let expected = -4.843731614932750E+03;
        assert!(
            (value.potential - expected).abs() <= 1e-9 * expected.abs(),
            "JEOD case 13: computed {} vs reference {}",
            value.potential,
            expected
        );
    }
}
