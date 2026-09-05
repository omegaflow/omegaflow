use omegaflow::archivar::spatial::{parse_star_record, star_stride, StarRec, STAR_RECORD_BYTES};
use omegaflow::bayestar::{
    build_index, decode_rec, ebv_at, index_add, index_sort, leaf_record, mu_of_r_pc, parse_header,
    Be19Row, ASSET_HEADER_LEN, BE19_BINS, REC_BYTES,
};
use omegaflow::healpix::icrs_to_galactic;
use omegaflow::json::{parse_json, JsonVal};
use std::io::{Read, Seek, SeekFrom};

const RV: f64 = 3.1;
const WANG_GBP_FACTOR: f64 = 2.429;
const WANG_G_FACTOR: f64 = 1.890;
const VALID_B_MIN_DEG: f64 = 20.0;
const BACKGROUND_PC_MIN: f64 = 200.0;
const DEC_BIN_DEG: f64 = 0.25;
const CROSSMATCH_RADIUS_AS_DEFAULT: f64 = 90.0;
const MIN_AV_DEFAULT: f64 = 0.2;

const DWARF_ANCHORS: &[(f64, &str, f64, f64)] = &[
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

fn type_label(num: f64) -> String {
    let letters = ["A", "F", "G", "K", "M"];
    if !(num.is_finite()) {
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

fn dwarf_color_type(bp_rp: f64) -> Option<(f64, f64)> {
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

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_arg(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn object_arg(args: &[String]) -> Option<(f64, f64)> {
    let i = args.iter().position(|a| a == "--object")?;
    let mut tokens: Vec<&str> = Vec::new();
    for a in &args[i + 1..] {
        if a.starts_with("--") {
            break;
        }
        tokens.extend(a.split_whitespace());
    }
    if tokens.len() < 2 {
        return None;
    }
    let ra = tokens[0].parse::<f64>().ok()?;
    let dec = tokens[1].parse::<f64>().ok()?;
    if ra.is_finite() && dec.is_finite() {
        Some((ra, dec))
    } else {
        None
    }
}

fn ang_sep_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let (s1, c1) = dec1.to_radians().sin_cos();
    let (s2, c2) = dec2.to_radians().sin_cos();
    let d = (ra1 - ra2).to_radians();
    let cos = (s1 * s2 + c1 * c2 * d.cos()).clamp(-1.0, 1.0);
    cos.acos().to_degrees() * 3600.0
}

struct StarBand {
    ra_deg: Vec<f64>,
    idx: Vec<u32>,
    dec_abs_max: f64,
}

struct StarIndex {
    dec_lo: f64,
    n_band: usize,
    bands: Vec<StarBand>,
    stars: Vec<StarRec>,
}

impl StarIndex {
    fn band_of(&self, dec: f64) -> usize {
        let b = ((dec - self.dec_lo) / DEC_BIN_DEG).floor() as i64;
        b.clamp(0, self.n_band as i64 - 1) as usize
    }
    fn within(&self, ra: f64, dec: f64, r_deg: f64) -> Vec<(usize, f64)> {
        let mut out: Vec<(usize, f64)> = Vec::new();
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
            let mut ranges: Vec<(f64, f64)> = Vec::new();
            let lo = ra - w;
            let hi = ra + w;
            if lo < 0.0 {
                ranges.push((0.0, hi));
                ranges.push((lo + 360.0, 360.0));
            } else if hi > 360.0 {
                ranges.push((lo, 360.0));
                ranges.push((0.0, hi - 360.0));
            } else {
                ranges.push((lo, hi));
            }
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

fn build_star_index(bytes: &[u8], stride: usize) -> StarIndex {
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

struct DustMap {
    idx: omegaflow::bayestar::MapQuery,
    file: MapFile,
}

struct DustHit {
    av: f64,
    av_full: f64,
    converged: bool,
    dm_min: f64,
    dm_max: f64,
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

fn load_map(path: &str, idx: &mut omegaflow::bayestar::MapQuery) -> Result<MapFile, String> {
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let mut head = [0u8; ASSET_HEADER_LEN];
    f.read_exact(&mut head)
        .map_err(|e| format!("read {path} header returned void: {e}"))?;
    let h = parse_header(&head).ok_or_else(|| format!("{path}: the header stays unread"))?;
    if h.mu0 != omegaflow::bayestar::BE19_MU0
        || h.dmu != omegaflow::bayestar::BE19_DMU
        || h.bins as usize != BE19_BINS
    {
        return Err(format!(
            "{path}: grid mu0 {} dmu {} bins {} disagrees with the reader grid — refused",
            h.mu0, h.dmu, h.bins
        ));
    }
    let mut rec = vec![0u8; REC_BYTES];
    let mut row_no = 0u64;
    while row_no < h.n_rows {
        f.read_exact(&mut rec)
            .map_err(|e| format!("read {path} record returned void: {e}"))?;
        let r = decode_rec(&rec).ok_or_else(|| format!("{path}: record {row_no} stays unread"))?;
        index_add(idx, row_no, r.nside.trailing_zeros() as u8, r.ipix);
        row_no += 1;
    }
    index_sort(idx);
    Ok(MapFile {
        file: f,
        last_idx: u64::MAX,
        last: None,
    })
}

impl DustMap {
    fn at(&mut self, theta: f64, phi: f64, d_pc: f64) -> Option<DustHit> {
        if !(d_pc.is_finite() && d_pc > 0.0) {
            return None;
        }
        let leaf = leaf_record(&self.idx, theta, phi)?;
        let row = self.file.read(leaf)?;
        let av_full = RV * row.best_fit[BE19_BINS - 1] as f64;
        let dm_min = row.dm_min as f64;
        let dm_max = row.dm_max as f64;
        let ebv = ebv_at(&row.best_fit, mu_of_r_pc(d_pc)?)?;
        Some(DustHit {
            av: RV * ebv,
            av_full,
            converged: row.converged,
            dm_min,
            dm_max,
        })
    }
}

struct Transient {
    id: String,
    alt: Option<String>,
    ra_deg: f64,
    dec_deg: f64,
    mag: Option<f64>,
}

fn parse_transient_file(path: &str) -> Option<Vec<Transient>> {
    let text = std::fs::read_to_string(path).ok()?;
    let root = parse_json(&text)?;
    let JsonVal::Arr(list) = root else {
        return None;
    };
    let mut out = Vec::new();
    for r in list {
        let JsonVal::Obj(m) = r else {
            continue;
        };
        let id = match m.get("id") {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => match m.get("obj") {
                Some(JsonVal::Str(s)) => s.clone(),
                _ => continue,
            },
        };
        let alt = match m.get("obj") {
            Some(JsonVal::Str(s)) => Some(s.clone()),
            _ => None,
        };
        let (Some(JsonVal::Num(ra)), Some(JsonVal::Num(dec))) = (m.get("ra"), m.get("dec")) else {
            continue;
        };
        if !ra.is_finite() || !dec.is_finite() {
            continue;
        }
        let mag = match m.get("mag") {
            Some(JsonVal::Num(n)) if n.is_finite() && *n > 0.0 => Some(*n),
            _ => None,
        };
        out.push(Transient {
            id,
            alt,
            ra_deg: *ra,
            dec_deg: *dec,
            mag,
        });
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

struct MatchStar {
    star_idx: usize,
    sep_arcsec: f64,
    b_deg: f64,
    d_pc: f64,
    dust: Option<DustHit>,
    dust_refused: &'static str,
}

fn abs_mag(g: f64, plx_mas: f64) -> f64 {
    g + 5.0 * plx_mas.log10() - 10.0
}

fn report_object(
    idx: &StarIndex,
    map: &mut DustMap,
    t: &Transient,
    radius_as: f64,
    min_av: f64,
    print_all: bool,
) {
    let r_deg = radius_as / 3600.0;
    let found = idx.within(t.ra_deg, t.dec_deg, r_deg);
    let (theta, _) = icrs_to_galactic(t.ra_deg, t.dec_deg);
    let b_t = 90.0 - theta.to_degrees();
    let mut matched: Vec<MatchStar> = Vec::new();
    for (k, sep) in &found {
        let s = &idx.stars[*k];
        let d_pc = 1000.0 / s.plx_mas;
        let (th, ph) = icrs_to_galactic(s.ra_deg, s.dec_deg);
        let b = 90.0 - th.to_degrees();
        let dust_refused;
        let dust;
        match map.at(th, ph, d_pc) {
            Some(h) => {
                dust = Some(h);
                dust_refused = "";
            }
            None => {
                dust = None;
                dust_refused = "no map leaf or distance outside the model grid";
            }
        }
        matched.push(MatchStar {
            star_idx: *k,
            sep_arcsec: *sep,
            b_deg: b,
            d_pc,
            dust,
            dust_refused,
        });
    }
    println!(
        "\n=== transient {} at ra {:.6} dec {:.6} (galactic |b| {:.2} deg) — {} catalog star(s) within {radius_as:.0} arcsec ===",
        t.id,
        t.ra_deg,
        t.dec_deg,
        b_t.abs(),
        found.len()
    );
    if let Some(mag) = t.mag {
        println!("broker alert magnitude: {mag:.2} (the transient detection, uncorrected)");
    }
    if found.is_empty() {
        println!(
            "no Gaia DR3 star within the crossmatch radius — the counterpart stays absent (0 honored)"
        );
        return;
    }
    let mut shown = 0usize;
    for m in &matched {
        let s = &idx.stars[m.star_idx];
        let bg = m.b_deg.abs() >= VALID_B_MIN_DEG && m.d_pc > BACKGROUND_PC_MIN;
        let dusty = m
            .dust
            .as_ref()
            .map(|d| d.av.is_finite() && d.av >= min_av)
            .unwrap_or(false);
        let pass = bg && dusty;
        if !pass && !print_all {
            continue;
        }
        shown += 1;
        let (col_type, col_mg) = match dwarf_color_type(s.color_index) {
            Some(x) => x,
            None => (f64::NAN, f64::NAN),
        };
        match &m.dust {
            Some(d) if d.av.is_finite() && d.av > 0.0 => {
                let e_bprp = d.av / WANG_GBP_FACTOR;
                let a_g = WANG_G_FACTOR * e_bprp;
                let g0 = s.mag - a_g;
                let bp_rp0 = s.color_index - e_bprp;
                let (t0, mg0) = match dwarf_color_type(bp_rp0) {
                    Some(x) => x,
                    None => (f64::NAN, f64::NAN),
                };
                let m_g0 = abs_mag(g0, s.plx_mas);
                let lum_delta = m_g0 - mg0;
                println!("---- counterpart candidate (dr3 record {}) ra {:.6} dec {:.6} | separation {:.2} arcsec | |b| {:.2} deg | plx {:.3} mas -> d {:.0} pc",
                    m.star_idx, s.ra_deg, s.dec_deg, m.sep_arcsec, m.b_deg.abs(), s.plx_mas, m.d_pc);
                println!(
                    "   dust column (Bayestar19, 3D, truncated at the parallax distance): A_V {:.3} mag (E(BP-RP) {e_bprp:.3}), full line-of-sight A_V {:.3} | converged {} | map DM window [{:.2}, {:.2}]",
                    d.av, d.av_full, d.converged, d.dm_min, d.dm_max
                );
                println!(
                    "   OBSERVED  (dust in):  G {:.3}  BP-RP {:.3}  -> dwarf-seq type {} (M_G dwarf expectation {:.2})",
                    s.mag, s.color_index, type_label(col_type), col_mg
                );
                println!(
                    "   DEREDDENED (G0, BP-RP0): G0 {:.3}  BP-RP0 {:.3}  -> dwarf-seq type {} (M_G dwarf expectation {:.2}) | M_G0 measured {:.2} (delta to dwarf {:.2})",
                    g0, bp_rp0, type_label(t0), mg0, m_g0, lum_delta
                );
                let m_g_obs = abs_mag(s.mag, s.plx_mas);
                println!(
                    "   reading: without the dust column the star is typed {}; with the {:.3}-mag column removed it is typed {} (dwarf-sequence color) — the identification of the counterpart changes. M_G (uncorrected) {:.2} vs M_G0 {:.2}: the star is {}.",
                    type_label(col_type),
                    d.av,
                    type_label(t0),
                    m_g_obs,
                    m_g0,
                    if lum_delta < -3.0 {
                        "evolved (a giant/subgiant: far brighter than a dwarf of its intrinsic color)"
                    } else if lum_delta > 3.0 {
                        "fainter than a dwarf of its intrinsic color (an unreliable parallax or a subdwarf/white-dwarf blend)"
                    } else {
                        "consistent with a dwarf of its intrinsic color (the dwarf-sequence type applies)"
                    }
                );
            }
            Some(d) if !(d.av.is_finite() && d.av > 0.0) => {
                println!(
                    "---- counterpart candidate (dr3 record {}) ra {:.6} dec {:.6} | separation {:.2} arcsec | |b| {:.2} deg | plx {:.3} mas -> d {:.0} pc | A_V at that distance: {:.3} mag (measured zero or non-positive — the star sits in front of the dust column)",
                    m.star_idx, s.ra_deg, s.dec_deg, m.sep_arcsec, m.b_deg.abs(), s.plx_mas, m.d_pc, d.av
                );
            }
            _ => {
                println!(
                    "---- counterpart candidate (dr3 record {}) ra {:.6} dec {:.6} | separation {:.2} arcsec | |b| {:.2} deg | plx {:.3} mas -> d {:.0} pc | A_V at that distance: unmeasured ({})",
                    m.star_idx, s.ra_deg, s.dec_deg, m.sep_arcsec, m.b_deg.abs(), s.plx_mas, m.d_pc, m.dust_refused
                );
            }
        }
    }
    if shown == 0 {
        println!(
            "no star within {radius_as:.0} arcsec passes the background+dust gate (|b| >= {VALID_B_MIN_DEG} deg, d > {BACKGROUND_PC_MIN:.0} pc, A_V >= {min_av:.2} mag) — the demonstration stays void here (0 honored)"
        );
    }
}

fn usage() {
    println!(
        "usage: deredden_crossmatch_probe --map <bayestar.be19> --stars <dr3_stars.bin> [--radius <arcsec=90>] [--min-av <mag=0.2>] (--object <ra> <dec> | --transients <loci.json>) [--all] [--name <id>]"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(map_path) = arg_value(&args, "--map") else {
        usage();
        return;
    };
    let Some(stars_path) = arg_value(&args, "--stars") else {
        usage();
        return;
    };
    let radius_as = arg_value(&args, "--radius")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|r| *r > 0.0 && r.is_finite())
        .unwrap_or(CROSSMATCH_RADIUS_AS_DEFAULT);
    let min_av = arg_value(&args, "--min-av")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite() && *v >= 0.0)
        .unwrap_or(MIN_AV_DEFAULT);
    let print_all = has_arg(&args, "--all");

    let star_bytes = match std::fs::read(&stars_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {stars_path} returned void: {e}");
            return;
        }
    };
    let stride = match star_stride(&star_bytes) {
        Some(s) => s,
        None => {
            eprintln!(
                "star bin {} bytes: no {}-byte records — the catalog stays unread",
                star_bytes.len(),
                STAR_RECORD_BYTES
            );
            return;
        }
    };
    let idx = build_star_index(&star_bytes, stride);
    eprintln!(
        "catalog: {} stars indexed in {} dec bands",
        idx.stars.len(),
        idx.n_band
    );

    let mut mq = build_index();
    let mf = match load_map(&map_path, &mut mq) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let mut map = DustMap { idx: mq, file: mf };

    println!(
        "=== deredden_crossmatch_probe — the dust column changes the identification of a transient's static-catalog counterpart ==="
    );
    println!(
        "extinction law: A_V = {RV} E(B-V); E(BP-RP) = A_V/{WANG_GBP_FACTOR}; A_G = {WANG_G_FACTOR} E(BP-RP) — Wang & Chen 2019 (ApJ 877, 116), the identical coefficients the dust_cleaning_3d_probe applies"
    );
    println!(
        "spectral type: the mean dwarf color sequence of Pecaut & Mamajek (2013, ApJS 208, 9; EEM dwarf table v2022.04.16), Gaia BP-RP — a color-only type; the M_G0 delta names the evolved/dwarf call"
    );
    println!(
        "crossmatch radius {radius_as:.0} arcsec; background+dust gate |b| >= {VALID_B_MIN_DEG} deg, parallax distance > {BACKGROUND_PC_MIN:.0} pc, distance-truncated A_V >= {min_av:.2} mag"
    );

    match (object_arg(&args), arg_value(&args, "--transients")) {
        (Some((ra, dec)), _) => {
            let name = match arg_value(&args, "--name") {
                Some(n) => n,
                None => format!("obj_{ra:.5}_{dec:.5}"),
            };
            let t = Transient {
                id: name,
                alt: None,
                ra_deg: ra,
                dec_deg: dec,
                mag: None,
            };
            report_object(&idx, &mut map, &t, radius_as, min_av, print_all);
        }
        (None, Some(path)) => {
            let Some(transients) = parse_transient_file(&path) else {
                eprintln!(
                    "{path}: the file carries no transient row array of {{id,ra,dec}} — the scan stays void (0 honored)"
                );
                return;
            };
            println!(
                "\nscanning {} real alert/transient positions from {path}",
                transients.len()
            );
            let mut passes = 0usize;
            for t in &transients {
                let found = idx.within(t.ra_deg, t.dec_deg, radius_as / 3600.0);
                for (k, sep) in &found {
                    let s = &idx.stars[*k];
                    let d_pc = 1000.0 / s.plx_mas;
                    if !(d_pc > BACKGROUND_PC_MIN) {
                        continue;
                    }
                    let (th, ph) = icrs_to_galactic(s.ra_deg, s.dec_deg);
                    let b = (90.0 - th.to_degrees()).abs();
                    if b < VALID_B_MIN_DEG {
                        continue;
                    }
                    let Some(d) = map.at(th, ph, d_pc) else {
                        continue;
                    };
                    if !(d.av.is_finite() && d.av >= min_av) {
                        continue;
                    }
                    passes += 1;
                    let e_bprp = d.av / WANG_GBP_FACTOR;
                    let bp_rp0 = s.color_index - e_bprp;
                    let g0 = s.mag - WANG_G_FACTOR * e_bprp;
                    let col = dwarf_color_type(s.color_index);
                    let col0 = dwarf_color_type(bp_rp0);
                    let m_g0 = abs_mag(g0, s.plx_mas);
                    println!(
                        "PASS {passes:>4} | {:<12} {:<10} ra {:.5} dec {:.5} | star dr3[{}] ra {:.5} dec {:.5} sep {:.1} as | b {:.1} d {:.0} pc plx {:.3} | A_V(trunc) {:.3} A_V(full) {:.3} | G {:.2} BP-RP {:.2} | BP-RP0 {:.2} G0 {:.2} | type {} -> {} | M_G0 {:.2}",
                        t.id,
                        t.alt.as_deref().unwrap_or(""),
                        t.ra_deg,
                        t.dec_deg,
                        k,
                        s.ra_deg,
                        s.dec_deg,
                        *sep,
                        b,
                        d_pc,
                        s.plx_mas,
                        d.av,
                        d.av_full,
                        s.mag,
                        s.color_index,
                        bp_rp0,
                        g0,
                        match col {
                            Some((n, _)) => type_label(n),
                            None => "?".to_string(),
                        },
                        match col0 {
                            Some((n, _)) => type_label(n),
                            None => "?".to_string(),
                        },
                        m_g0
                    );
                }
            }
            println!("\nverdict: {passes} transient-to-star crossmatch case(s) carry a background star behind A_V >= {min_av:.2} mag within {radius_as:.0} arcsec");
            if passes == 0 {
                println!(
                    "no case meets the gate at this radius/A_V — raise --radius, lower --min-av, or feed more transients (0 honored; the field A_V and parallax distances are measured, not fabricated)"
                );
            }
        }
        _ => usage(),
    }
}
