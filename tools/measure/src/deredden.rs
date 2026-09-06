use omegaflow::archivar::spatial::{parse_star_record, StarRec};
use omegaflow::bayestar::{
    build_index, decode_rec, ebv_at, index_add, index_sort, leaf_record, mu_of_r_pc, parse_header,
    Be19Row, ASSET_HEADER_LEN, BE19_BINS, BE19_DMU, BE19_MU0, REC_BYTES,
};
use omegaflow::healpix::icrs_to_galactic;
use std::io::{Read, Seek, SeekFrom};

pub const RV: f64 = 3.1;
pub const WANG_GBP_FACTOR: f64 = 2.429;
pub const WANG_G_FACTOR: f64 = 1.890;
pub const BACKGROUND_PC_MIN: f64 = 200.0;
const DEC_BIN_DEG: f64 = 0.25;

pub const DWARF_ANCHORS: &[(f64, &str, f64, f64)] = &[
    (-0.037, "A0", 20.000, 1.00),
    (0.005, "A1", 21.000, 1.16),
    (0.068, "A2", 22.000, 1.34),
    (0.110, "A3", 23.000, 1.69),
    (0.166, "A4", 24.000, 1.92),
    (0.194, "A5", 25.000, 1.98),
    (0.222, "A6", 26.000, 2.09),
    (0.263, "A7", 27.000, 2.19),
    (0.320, "A8", 28.000, 2.27),
    (0.327, "A9", 29.000, 2.37),
    (0.377, "F0", 30.000, 2.51),
    (0.434, "F1", 31.000, 2.69),
    (0.490, "F2", 32.000, 2.89),
    (0.518, "F3", 33.000, 2.99),
    (0.546, "F4", 34.000, 3.10),
    (0.587, "F5", 35.000, 3.26),
    (0.640, "F6", 36.000, 3.56),
    (0.670, "F7", 37.000, 3.66),
    (0.694, "F8", 38.000, 3.90),
    (0.719, "F9", 39.000, 4.11),
    (0.767, "F9.5", 39.500, 4.20),
    (0.784, "G0", 40.000, 4.33),
    (0.803, "G1", 41.000, 4.46),
    (0.823, "G2", 42.000, 4.63),
    (0.832, "G3", 43.000, 4.70),
    (0.841, "G4", 44.000, 4.76),
    (0.850, "G5", 45.000, 4.80),
    (0.869, "G6", 46.000, 4.91),
    (0.880, "G7", 47.000, 5.01),
    (0.900, "G8", 48.000, 5.10),
    (0.950, "G9", 49.000, 5.34),
    (0.983, "K0", 50.000, 5.55),
    (1.010, "K1", 51.000, 5.65),
    (1.100, "K2", 52.000, 5.83),
    (1.210, "K3", 53.000, 6.20),
    (1.340, "K4", 54.000, 6.53),
    (1.430, "K5", 55.000, 6.83),
    (1.530, "K6", 56.000, 7.02),
    (1.700, "K7", 57.000, 7.57),
    (1.730, "K8", 58.000, 7.74),
    (1.790, "K9", 59.000, 8.03),
    (1.840, "M0", 60.000, 8.16),
    (1.970, "M0.5", 60.500, 8.44),
    (2.090, "M1", 61.000, 8.82),
    (2.130, "M1.5", 61.500, 8.98),
    (2.230, "M2", 62.000, 9.29),
    (2.390, "M2.5", 62.500, 9.67),
    (2.500, "M3", 63.000, 10.05),
    (2.780, "M3.5", 63.500, 10.87),
    (2.940, "M4", 64.000, 11.21),
    (3.160, "M4.5", 64.500, 12.04),
    (3.350, "M5", 65.000, 12.45),
    (3.710, "M5.5", 65.500, 13.35),
    (4.160, "M6", 66.000, 14.26),
    (4.500, "M6.5", 66.500, 14.40),
    (4.650, "M7", 67.000, 14.72),
    (4.720, "M7.5", 67.500, 15.20),
    (4.860, "M8", 68.000, 15.20),
    (5.100, "M8.5", 68.500, 15.90),
];

pub fn type_label(num: f64) -> String {
    let letters = ["A", "F", "G", "K", "M"];
    if !num.is_finite() {
        return "out-of-sequence".to_string();
    }
    let base = ((num / 10.0).floor() * 10.0).clamp(20.0, 60.0) as i32;
    if base < 20 || base > 60 {
        return "out-of-sequence".to_string();
    }
    let li = ((base - 20) / 10) as usize;
    let li = li.min(letters.len() - 1);
    let sub = num - base as f64;
    if sub >= 9.5 && li + 1 < letters.len() {
        return format!("{}{:.1}", letters[li + 1], sub - 9.5);
    }
    format!("{}{}", letters[li], (sub * 10.0).round() / 10.0)
}

pub fn dwarf_color_type(bp_rp: f64) -> Option<(f64, f64)> {
    if !bp_rp.is_finite() {
        return None;
    }
    let n = DWARF_ANCHORS.len();
    if bp_rp <= DWARF_ANCHORS[0].0 {
        let a = &DWARF_ANCHORS[0];
        return Some((a.2, a.3));
    }
    if bp_rp >= DWARF_ANCHORS[n - 1].0 {
        let a = &DWARF_ANCHORS[n - 1];
        return Some((a.2, a.3));
    }
    let pos = DWARF_ANCHORS.partition_point(|a| a.0 < bp_rp);
    let hi = pos.min(n - 1);
    let lo = hi - 1;
    let (bp0, _, num0, mg0) = DWARF_ANCHORS[lo];
    let (bp1, _, num1, mg1) = DWARF_ANCHORS[hi];
    let span = bp1 - bp0;
    if span <= 0.0 {
        return None;
    }
    let t = ((bp_rp - bp0) / span).clamp(0.0, 1.0);
    Some((num0 + (num1 - num0) * t, mg0 + (mg1 - mg0) * t))
}

pub fn abs_mag(g: f64, plx_mas: f64) -> f64 {
    g + 5.0 * plx_mas.log10() - 10.0
}

pub fn ang_sep_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let (s1, c1) = dec1.to_radians().sin_cos();
    let (s2, c2) = dec2.to_radians().sin_cos();
    let d = (ra1 - ra2).to_radians();
    let cos = (s1 * s2 + c1 * c2 * d.cos()).clamp(-1.0, 1.0);
    cos.acos().to_degrees() * 3600.0
}

struct MapFile {
    file: std::fs::File,
    last_idx: u64,
    last: Option<Be19Row>,
}

impl MapFile {
    fn read(&mut self, idx: u64) -> Option<&Be19Row> {
        if self.last_idx != idx {
            let off = ASSET_HEADER_LEN as u64 + idx * REC_BYTES as u64;
            self.file.seek(SeekFrom::Start(off)).ok()?;
            let mut buf = vec![0u8; REC_BYTES];
            self.file.read_exact(&mut buf).ok()?;
            self.last = decode_rec(&buf);
            self.last_idx = idx;
        }
        self.last.as_ref()
    }
}

pub struct DustHit {
    pub av: f64,
    pub av_full: f64,
    pub converged: bool,
    pub dm_min: f64,
    pub dm_max: f64,
}

pub struct DustMap {
    index: omegaflow::bayestar::MapQuery,
    file: MapFile,
}

impl DustMap {
    pub fn open(path: &str) -> Result<DustMap, String> {
        let mut f =
            std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
        let mut head = [0u8; ASSET_HEADER_LEN];
        f.read_exact(&mut head)
            .map_err(|e| format!("read {path} header returned void: {e}"))?;
        let h = parse_header(&head).ok_or_else(|| format!("{path}: the header stays unread"))?;
        if h.mu0 != BE19_MU0 || h.dmu != BE19_DMU || h.bins as usize != BE19_BINS {
            return Err(format!(
                "{path}: grid mu0 {} dmu {} bins {} disagrees with the reader grid — refused",
                h.mu0, h.dmu, h.bins
            ));
        }
        let mut index = build_index();
        let mut rec = vec![0u8; REC_BYTES];
        let mut row_no = 0u64;
        while row_no < h.n_rows {
            f.read_exact(&mut rec)
                .map_err(|e| format!("read {path} record returned void: {e}"))?;
            let r =
                decode_rec(&rec).ok_or_else(|| format!("{path}: record {row_no} stays unread"))?;
            index_add(&mut index, row_no, r.nside.trailing_zeros() as u8, r.ipix);
            row_no += 1;
        }
        index_sort(&mut index);
        Ok(DustMap {
            index,
            file: MapFile {
                file: f,
                last_idx: u64::MAX,
                last: None,
            },
        })
    }

    pub fn at(&mut self, ra_deg: f64, dec_deg: f64, d_pc: f64) -> Option<DustHit> {
        if !(d_pc.is_finite() && d_pc > 0.0) {
            return None;
        }
        let (theta, phi) = icrs_to_galactic(ra_deg, dec_deg);
        let leaf = leaf_record(&self.index, theta, phi)?;
        let row = self.file.read(leaf)?;
        let av_full = RV * row.best_fit[BE19_BINS - 1] as f64;
        let ebv = ebv_at(&row.best_fit, mu_of_r_pc(d_pc)?)?;
        Some(DustHit {
            av: RV * ebv,
            av_full,
            converged: row.converged,
            dm_min: row.dm_min as f64,
            dm_max: row.dm_max as f64,
        })
    }
}

struct StarBand {
    ra_deg: Vec<f64>,
    idx: Vec<u32>,
    dec_abs_max: f64,
}

pub struct StarIndex {
    dec_lo: f64,
    n_band: usize,
    bands: Vec<StarBand>,
    pub stars: Vec<StarRec>,
}

impl StarIndex {
    fn band_of(&self, dec: f64) -> usize {
        let b = ((dec - self.dec_lo) / DEC_BIN_DEG).floor() as i64;
        b.clamp(0, self.n_band as i64 - 1) as usize
    }

    pub fn within(&self, ra: f64, dec: f64, r_deg: f64) -> Vec<(usize, f64)> {
        let mut out: Vec<(usize, f64)> = Vec::new();
        if self.stars.is_empty() || !(r_deg.is_finite() && r_deg >= 0.0) {
            return out;
        }
        let b0 = self.band_of(dec - r_deg);
        let b1 = self.band_of(dec + r_deg);
        for b in b0..=b1 {
            let band = &self.bands[b];
            if band.idx.is_empty() {
                continue;
            }
            let cosmin = band.dec_abs_max.to_radians().cos();
            let w = if cosmin > 1e-4 {
                (r_deg / cosmin).min(360.0)
            } else {
                360.0
            };
            let lo = ra - w;
            let hi = ra + w;
            let ranges: Vec<(f64, f64)> = if lo < 0.0 {
                vec![(0.0, hi), (lo + 360.0, 360.0)]
            } else if hi > 360.0 {
                vec![(lo, 360.0), (0.0, hi - 360.0)]
            } else {
                vec![(lo, hi)]
            };
            for (r0, r1) in ranges {
                if r1 <= r0 {
                    continue;
                }
                let lo_p = band.ra_deg.partition_point(|&x| x < r0);
                let hi_p = band.ra_deg.partition_point(|&x| x < r1);
                for j in lo_p..hi_p {
                    let k = band.idx[j] as usize;
                    let s = &self.stars[k];
                    let sep = ang_sep_arcsec(ra, dec, s.ra_deg, s.dec_deg);
                    if sep <= r_deg * 3600.0 {
                        out.push((k, sep));
                    }
                }
            }
        }
        out.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        out
    }
}

pub fn build_star_index(bytes: &[u8]) -> StarIndex {
    let stride = omegaflow::archivar::spatial::star_stride(bytes);
    let Some(stride) = stride else {
        return StarIndex {
            dec_lo: 0.0,
            n_band: 0,
            bands: Vec::new(),
            stars: Vec::new(),
        };
    };
    let mut stars: Vec<StarRec> = Vec::new();
    let mut dec_min = f64::INFINITY;
    let mut dec_max = f64::NEG_INFINITY;
    for chunk in bytes.chunks_exact(stride) {
        if let Some(rec) = parse_star_record(chunk) {
            if rec.ra_deg.is_finite() && rec.dec_deg.is_finite() {
                dec_min = dec_min.min(rec.dec_deg);
                dec_max = dec_max.max(rec.dec_deg);
                stars.push(rec);
            }
        }
    }
    if stars.is_empty() {
        return StarIndex {
            dec_lo: 0.0,
            n_band: 0,
            bands: Vec::new(),
            stars,
        };
    }
    let dec_lo = (dec_min / DEC_BIN_DEG).floor() * DEC_BIN_DEG;
    let dec_hi = (dec_max / DEC_BIN_DEG).floor() * DEC_BIN_DEG;
    let n_band = ((dec_hi - dec_lo) / DEC_BIN_DEG).floor() as usize + 1;
    let mut bands: Vec<StarBand> = (0..n_band)
        .map(|_| StarBand {
            ra_deg: Vec::new(),
            idx: Vec::new(),
            dec_abs_max: 0.0,
        })
        .collect();
    for (i, s) in stars.iter().enumerate() {
        let b = ((s.dec_deg - dec_lo) / DEC_BIN_DEG).floor() as i64;
        let b = b.clamp(0, n_band as i64 - 1) as usize;
        let blo = dec_lo + b as f64 * DEC_BIN_DEG;
        let bhi = blo + DEC_BIN_DEG;
        let a = blo.abs().max(bhi.abs());
        let band = &mut bands[b];
        if a > band.dec_abs_max {
            band.dec_abs_max = a;
        }
        band.ra_deg.push(s.ra_deg);
        band.idx.push(i as u32);
    }
    for band in bands.iter_mut() {
        let mut pairs: Vec<(f64, u32)> = band
            .ra_deg
            .iter()
            .copied()
            .zip(band.idx.iter().copied())
            .collect();
        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        band.ra_deg = pairs.iter().map(|p| p.0).collect();
        band.idx = pairs.iter().map(|p| p.1).collect();
    }
    StarIndex {
        dec_lo,
        n_band,
        bands,
        stars,
    }
}

pub struct Intrinsic {
    pub e_bprp: f64,
    pub a_g: f64,
    pub bp_rp0: f64,
    pub g0: f64,
    pub m_g0: f64,
}

pub fn intrinsic_of(star: &StarRec, av: f64) -> Option<Intrinsic> {
    if !(av.is_finite() && av > 0.0) {
        return None;
    }
    if !(star.color_index.is_finite() && star.mag.is_finite()) {
        return None;
    }
    let e_bprp = av / WANG_GBP_FACTOR;
    let a_g = WANG_G_FACTOR * e_bprp;
    let bp_rp0 = star.color_index - e_bprp;
    let g0 = star.mag - a_g;
    let m_g0 = abs_mag(g0, star.plx_mas);
    if !(bp_rp0.is_finite() && g0.is_finite() && m_g0.is_finite()) {
        return None;
    }
    Some(Intrinsic {
        e_bprp,
        a_g,
        bp_rp0,
        g0,
        m_g0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::spatial::STAR_RECORD_BYTES;

    fn star(color: f64, mag: f64, plx_mas: f64) -> Vec<u8> {
        let mut b = Vec::with_capacity(STAR_RECORD_BYTES);
        b.extend_from_slice(&246.5f64.to_le_bytes());
        b.extend_from_slice(&(-16.8f64).to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&(plx_mas as f32).to_le_bytes());
        b.extend_from_slice(&(mag as f32).to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&(color as f32).to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b
    }

    fn rec(bytes: &[u8]) -> StarRec {
        parse_star_record(bytes).unwrap()
    }

    #[test]
    fn dwarf_sequence_labels_a_reddened_m05_as_k27_intrinsic() {
        let s = rec(&star(1.958, 11.041, 1.512));
        let (t_num, _) = dwarf_color_type(s.color_index).unwrap();
        assert_eq!(type_label(t_num), "M0.5");
        let itr = intrinsic_of(&s, 1.891).unwrap();
        let (t0, _) = dwarf_color_type(itr.bp_rp0).unwrap();
        assert_eq!(type_label(t0), "K2.7");
        assert!((itr.e_bprp - 1.891 / WANG_GBP_FACTOR).abs() < 1e-12);
        assert!(
            (itr.g0 - (s.mag - WANG_G_FACTOR * itr.e_bprp)).abs() < 1e-9,
            "the intrinsic magnitude follows from the catalog magnitude and the same column"
        );
    }

    #[test]
    fn intrinsic_stays_absent_without_a_positive_column() {
        let s = rec(&star(1.958, 11.041, 1.512));
        assert!(intrinsic_of(&s, 0.0).is_none());
        assert!(intrinsic_of(&s, f64::NAN).is_none());
        assert!(intrinsic_of(&s, -0.5).is_none());
    }

    #[test]
    fn type_label_handles_the_sequence_edges() {
        assert_eq!(type_label(20.0), "A0");
        assert_eq!(type_label(60.5), "M0.5");
        assert_eq!(type_label(52.7), "K2.7");
        assert_eq!(type_label(39.4), "F9.4");
        assert_eq!(type_label(39.5), "G0.0");
        assert_eq!(type_label(40.0), "G0");
        assert_eq!(type_label(f64::NAN), "out-of-sequence");
    }

    #[test]
    fn star_index_records_beyond_a_crossmatch_radius_stay_out() {
        let bytes = star(246.5, -16.8, 1.512);
        let index = build_star_index(&bytes);
        assert_eq!(index.stars.len(), 1);
        let hit = index.within(246.5, -16.8, 1.0 / 3600.0);
        assert_eq!(hit.len(), 1);
        assert!(hit[0].1 < 0.1);
        let far = index.within(246.5, -16.0, 1.0 / 3600.0);
        assert!(far.is_empty());
    }

    #[test]
    fn galactic_abs_mag_of_the_demo_case_matches() {
        let s = rec(&star(1.958, 11.041, 1.512));
        let expect = 11.041 + 5.0 * 1.512f64.log10() - 10.0;
        assert!(
            (abs_mag(s.mag, s.plx_mas) - expect).abs() < 1e-6,
            "the f32 catalog round trip keeps the abs-G within 1e-6"
        );
    }
}
