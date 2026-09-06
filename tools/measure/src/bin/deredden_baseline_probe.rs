use omegaflow::archivar::spatial::{star_stride, STAR_RECORD_BYTES};
use omegaflow_measure::weberin::deredden::{
    build_star_index, dwarf_color_type, type_label, DustMap, StarIndex, WANG_GBP_FACTOR,
};

const DEFAULT_RADIUS_DEG: f64 = 0.25;
const N_SIG_DEFAULT: f64 = 3.0;
const MIN_NEIGHBORS_DEFAULT: usize = 8;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_num(args: &[String], name: &str) -> Option<f64> {
    arg_value(args, name)
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite())
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    arg_value(args, name)
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|v| *v > 0)
}

fn type_of(bp_rp: f64) -> String {
    let t = match dwarf_color_type(bp_rp) {
        Some((t, _)) => t,
        None => f64::NAN,
    };
    type_label(t)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum VerdictWord {
    Outlier,
    Typical,
    Pending,
}

impl VerdictWord {
    fn word(self) -> &'static str {
        match self {
            VerdictWord::Outlier => "outlier",
            VerdictWord::Typical => "typical",
            VerdictWord::Pending => "pending",
        }
    }
}

fn median_of(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    Some(if n % 2 == 1 {
        sorted[n / 2]
    } else {
        0.5 * (sorted[n / 2 - 1] + sorted[n / 2])
    })
}

fn mad_sigma_about(values: &[f64], center: f64) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let deviations: Vec<f64> = values.iter().map(|v| (v - center).abs()).collect();
    Some(1.4826 * median_of(&deviations)?)
}

fn robust_window(values: &[f64], n_sigma: f64) -> Option<(f64, f64, f64, f64)> {
    let center = median_of(values)?;
    let scale = mad_sigma_about(values, center)?;
    Some((
        center,
        scale,
        center - n_sigma * scale,
        center + n_sigma * scale,
    ))
}

fn outside_threshold(value: f64, center: f64, scale: f64, n_sigma: f64) -> bool {
    let dev = (value - center).abs();
    if scale > 0.0 {
        dev > n_sigma * scale
    } else {
        dev > 0.0
    }
}

struct RawMember {
    star_idx: usize,
    sep_center_arcsec: f64,
    observed_bp_rp: f64,
    av: f64,
    e_bprp: f64,
    bp_rp0: f64,
    d_pc: f64,
}

struct ScoredMember {
    star_idx: usize,
    sep_center_arcsec: f64,
    observed_bp_rp: f64,
    av: f64,
    e_bprp: f64,
    bp_rp0: f64,
    d_pc: f64,
    neighbors: usize,
    baseline_median: Option<f64>,
    baseline_sigma: Option<f64>,
    verdict: VerdictWord,
}

struct AbsentMember {
    star_idx: usize,
    reason: &'static str,
}

struct FieldReport {
    members: Vec<ScoredMember>,
    absent: Vec<AbsentMember>,
}

fn scan_field(
    map_path: &str,
    idx: &StarIndex,
    center_ra: f64,
    center_dec: f64,
    radius_deg: f64,
    n_sigma: f64,
    min_neighbors: usize,
) -> Result<FieldReport, String> {
    let mut map = DustMap::open(map_path)?;
    let found = idx.within(center_ra, center_dec, radius_deg);
    let mut absent: Vec<AbsentMember> = Vec::new();
    let mut raw: Vec<RawMember> = Vec::new();
    for (k, sep) in &found {
        let s = &idx.stars[*k];
        if !s.color_index.is_finite() {
            absent.push(AbsentMember {
                star_idx: *k,
                reason: "no finite color",
            });
            continue;
        }
        let d_pc = 1000.0 / s.plx_mas;
        if !(d_pc.is_finite() && d_pc > 0.0) {
            absent.push(AbsentMember {
                star_idx: *k,
                reason: "no positive parallax distance",
            });
            continue;
        }
        let Some(dust) = map.at(s.ra_deg, s.dec_deg, d_pc) else {
            absent.push(AbsentMember {
                star_idx: *k,
                reason: "no dust leaf or the distance stays outside the model grid",
            });
            continue;
        };
        let av = dust.av;
        if !(av.is_finite() && av >= 0.0) {
            absent.push(AbsentMember {
                star_idx: *k,
                reason: "A_V refused (non-finite or negative)",
            });
            continue;
        }
        let e_bprp = av / WANG_GBP_FACTOR;
        let bp_rp0 = s.color_index - e_bprp;
        if !bp_rp0.is_finite() {
            absent.push(AbsentMember {
                star_idx: *k,
                reason: "the intrinsic color stays non-finite",
            });
            continue;
        }
        raw.push(RawMember {
            star_idx: *k,
            sep_center_arcsec: *sep,
            observed_bp_rp: s.color_index,
            av,
            e_bprp,
            bp_rp0,
            d_pc,
        });
    }
    let measured = raw.len();
    let mut members: Vec<ScoredMember> = Vec::with_capacity(measured);
    for i in 0..measured {
        let m = &raw[i];
        let neighbors = measured - 1;
        if measured <= min_neighbors {
            members.push(ScoredMember {
                star_idx: m.star_idx,
                sep_center_arcsec: m.sep_center_arcsec,
                observed_bp_rp: m.observed_bp_rp,
                av: m.av,
                e_bprp: m.e_bprp,
                bp_rp0: m.bp_rp0,
                d_pc: m.d_pc,
                neighbors,
                baseline_median: None,
                baseline_sigma: None,
                verdict: VerdictWord::Pending,
            });
            continue;
        }
        let others: Vec<f64> = (0..measured)
            .filter(|j| *j != i)
            .map(|j| raw[j].bp_rp0)
            .collect();
        let center = median_of(&others);
        let scale = center.and_then(|c| mad_sigma_about(&others, c));
        let verdict = match (center, scale) {
            (Some(c), Some(s)) => {
                if outside_threshold(m.bp_rp0, c, s, n_sigma) {
                    VerdictWord::Outlier
                } else {
                    VerdictWord::Typical
                }
            }
            _ => VerdictWord::Pending,
        };
        members.push(ScoredMember {
            star_idx: m.star_idx,
            sep_center_arcsec: m.sep_center_arcsec,
            observed_bp_rp: m.observed_bp_rp,
            av: m.av,
            e_bprp: m.e_bprp,
            bp_rp0: m.bp_rp0,
            d_pc: m.d_pc,
            neighbors,
            baseline_median: center,
            baseline_sigma: scale,
            verdict,
        });
    }
    Ok(FieldReport { members, absent })
}

fn usage() {
    println!(
        "usage: deredden_baseline_probe --map <bayestar.be19> --stars <dr3_stars.bin> --ra <deg> --dec <deg> [--radius <deg=0.25>] [--n-sigma <3.0>] [--min-neighbors <8>]"
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
    let Some(ra) = arg_num(&args, "--ra") else {
        usage();
        return;
    };
    let Some(dec) = arg_num(&args, "--dec") else {
        usage();
        return;
    };
    if !(ra >= 0.0 && ra <= 360.0 && dec >= -90.0 && dec <= 90.0) {
        usage();
        return;
    }
    let radius_deg = arg_num(&args, "--radius")
        .filter(|v| *v > 0.0)
        .unwrap_or(DEFAULT_RADIUS_DEG);
    let n_sigma = arg_num(&args, "--n-sigma")
        .filter(|v| *v > 0.0)
        .unwrap_or(N_SIG_DEFAULT);
    let min_neighbors = arg_usize(&args, "--min-neighbors").unwrap_or(MIN_NEIGHBORS_DEFAULT);

    let star_bytes = match std::fs::read(&stars_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {stars_path} returned void: {e}");
            return;
        }
    };
    if star_stride(&star_bytes).is_none() {
        eprintln!(
            "star bin {} bytes: no {}-byte records — the catalog stays unread",
            star_bytes.len(),
            STAR_RECORD_BYTES
        );
        return;
    }
    let idx = build_star_index(&star_bytes);
    eprintln!("catalog: {} stars indexed", idx.stars.len());

    let report = match scan_field(&map_path, &idx, ra, dec, radius_deg, n_sigma, min_neighbors) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let measured = report.members.len();

    println!("=== deredden_baseline_probe — the local intrinsic color field, dust subtracted ===");
    println!(
        "extinction law: A_V = 3.1 E(B-V); E(BP-RP) = A_V/{WANG_GBP_FACTOR} — Wang & Chen 2019 (ApJ 877, 116), the identical coefficients deredden_crossmatch_probe applies"
    );
    println!(
        "field {radius_deg:.2} deg around ra {ra:.5} dec {dec:.5}; the field median is the local baseline, the scale is the MAD sigma, the threshold median +/- {n_sigma}·sigma; a member's own color never enters its own window"
    );

    if report.members.is_empty() {
        println!(
            "the region carries no measured intrinsic color — {} source(s) in the cone stay absent (0 honored); the local field baseline stays unmeasured",
            report.absent.len()
        );
        for a in &report.absent {
            let s = &idx.stars[a.star_idx];
            println!(
                "---- dr3[{}] ra {:.6} dec {:.6}: absent ({})",
                a.star_idx, s.ra_deg, s.dec_deg, a.reason
            );
        }
        return;
    }

    let observed: Vec<f64> = report.members.iter().map(|m| m.observed_bp_rp).collect();
    let intrinsic: Vec<f64> = report.members.iter().map(|m| m.bp_rp0).collect();
    if let Some((oc, os, olo, ohi)) = robust_window(&observed, n_sigma) {
        println!(
            "observed BP-RP field over {measured} member(s): median {oc:.3}, MAD-sigma {os:.3}, {n_sigma}·sigma window [{olo:.3}, {ohi:.3}]"
        );
    }
    if let Some((ic, isg, ilo, ihi)) = robust_window(&intrinsic, n_sigma) {
        println!(
            "intrinsic BP-RP0 field after the dust subtraction: median {ic:.3}, MAD-sigma {isg:.3}, {n_sigma}·sigma window [{ilo:.3}, {ihi:.3}]"
        );
    }
    if let (Some((_, os, _, _)), Some((_, isg, _, _))) = (
        robust_window(&observed, n_sigma),
        robust_window(&intrinsic, n_sigma),
    ) {
        println!(
            "the dust subtraction narrows the field spread from {os:.3} to {isg:.3} MAD-sigma"
        );
    }

    for m in &report.members {
        let s = &idx.stars[m.star_idx];
        println!(
            "---- dr3[{}] ra {:.6} dec {:.6} | center sep {:.1} arcsec | plx {:.3} mas -> d {:.0} pc",
            m.star_idx, s.ra_deg, s.dec_deg, m.sep_center_arcsec, s.plx_mas, m.d_pc
        );
        println!(
            "    dust column: A_V {:.3} mag, E(BP-RP) {:.3} mag (Bayestar19, truncated at the parallax distance)",
            m.av, m.e_bprp
        );
        println!(
            "    color: observed BP-RP {:.3} ({}) -> intrinsic BP-RP0 {:.3} ({})",
            m.observed_bp_rp,
            type_of(m.observed_bp_rp),
            m.bp_rp0,
            type_of(m.bp_rp0)
        );
        match (m.baseline_median, m.baseline_sigma) {
            (Some(c), Some(sg)) => {
                let dev = m.bp_rp0 - c;
                let sig = if sg > 0.0 { dev / sg } else { 0.0 };
                let side = if dev > 0.0 { "above" } else { "below" };
                println!(
                    "    local baseline over the other {} measured member(s): median {c:.3}, MAD-sigma {sg:.3} | BP-RP0 sits {:.2} sigma {side} the median -> {}",
                    m.neighbors, sig, m.verdict.word()
                );
            }
            _ => {
                println!(
                    "    local baseline stays pending: the other measured members ({}) lie below the {min_neighbors}-member neighborhood floor -> {}",
                    m.neighbors,
                    m.verdict.word()
                );
            }
        }
    }
    for a in &report.absent {
        let s = &idx.stars[a.star_idx];
        println!(
            "---- dr3[{}] ra {:.6} dec {:.6}: absent ({})",
            a.star_idx, s.ra_deg, s.dec_deg, a.reason
        );
    }

    let outliers = report
        .members
        .iter()
        .filter(|m| m.verdict == VerdictWord::Outlier)
        .count();
    let typical = report
        .members
        .iter()
        .filter(|m| m.verdict == VerdictWord::Typical)
        .count();
    let pending = report
        .members
        .iter()
        .filter(|m| m.verdict == VerdictWord::Pending)
        .count();
    println!(
        "field verdict: {outliers} color outlier candidate(s) survive the dust subtraction; {typical} typical; {pending} pending; {} absent — over {} cone source(s) at ra {ra:.5} dec {dec:.5}",
        report.absent.len(),
        report.members.len() + report.absent.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::bayestar::{
        encode_rec, write_header, Be19Row, MapHeader, BE19_BINS, BE19_DMU, BE19_MU0, REC_BYTES,
    };
    use omegaflow::healpix::{ang2pix_nest, galactic_to_icrs, icrs_to_galactic, pix2ang_nest};

    const TEST_NSIDE: i64 = 512;
    const TEST_IPIX: i64 = 3_000_000;

    fn region_center() -> (f64, f64) {
        let (theta, phi) = pix2ang_nest(TEST_NSIDE, TEST_IPIX).unwrap();
        galactic_to_icrs(theta, phi)
    }

    fn star_record(ra: f64, dec: f64, plx_mas: f64, mag: f64, bp_rp: f64) -> Vec<u8> {
        let mut b = Vec::with_capacity(STAR_RECORD_BYTES);
        b.extend_from_slice(&ra.to_le_bytes());
        b.extend_from_slice(&dec.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&(plx_mas as f32).to_le_bytes());
        b.extend_from_slice(&(mag as f32).to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&(bp_rp as f32).to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b
    }

    fn write_be19(path: &str, center_ra: f64, center_dec: f64, ebv_per_dm: f64) {
        let (theta, phi) = icrs_to_galactic(center_ra, center_dec);
        let ipix = ang2pix_nest(TEST_NSIDE, theta, phi).unwrap();
        assert_eq!(ipix, TEST_IPIX);
        let mut best_fit = [0.0f32; BE19_BINS];
        for (j, v) in best_fit.iter_mut().enumerate() {
            *v = (ebv_per_dm * BE19_DMU * j as f64) as f32;
        }
        let mut bytes = Vec::new();
        write_header(
            &mut bytes,
            &MapHeader {
                n_rows: 1,
                mu0: BE19_MU0,
                dmu: BE19_DMU,
                bins: BE19_BINS as u16,
            },
        );
        let mut row = [0u8; REC_BYTES];
        encode_rec(
            &mut row,
            &Be19Row {
                nside: TEST_NSIDE as u32,
                ipix: TEST_IPIX as u32,
                converged: true,
                dm_min: 4.0,
                dm_max: 18.0,
                n_good: 8,
                best_fit,
            },
        );
        bytes.extend_from_slice(&row);
        std::fs::write(path, bytes).unwrap();
    }

    fn av_at(map: &mut DustMap, ra: f64, dec: f64, plx_mas: f64) -> f64 {
        let d_pc = 1000.0 / plx_mas;
        map.at(ra, dec, d_pc).unwrap().av
    }

    fn offset_ra(center_ra: f64, k: usize) -> f64 {
        center_ra + ((k % 6) as f64) * 2.2 / 3600.0
    }

    fn offset_dec(center_dec: f64, k: usize) -> f64 {
        center_dec + ((k / 6) as f64) * 2.2 / 3600.0
    }

    fn build_field_bin(
        map_path: &str,
        center_ra: f64,
        center_dec: f64,
        plx_list: &[f64],
        intrinsic_jitter: f64,
        special: Option<(f64, f64)>,
    ) -> Vec<u8> {
        let mut map = DustMap::open(map_path).unwrap();
        let mut bytes = Vec::new();
        for (k, plx) in plx_list.iter().enumerate() {
            let intrinsic = 0.30 + ((k % 7) as f64 - 3.0) * intrinsic_jitter;
            let (ra, dec) = (offset_ra(center_ra, k), offset_dec(center_dec, k));
            let av = av_at(&mut map, ra, dec, *plx);
            let observed = intrinsic + av / WANG_GBP_FACTOR;
            bytes.extend_from_slice(&star_record(ra, dec, *plx, 15.0, observed));
        }
        if let Some((plx, intrinsic)) = special {
            let k = plx_list.len();
            let (ra, dec) = (offset_ra(center_ra, k), offset_dec(center_dec, k));
            let av = av_at(&mut map, ra, dec, plx);
            let observed = intrinsic + av / WANG_GBP_FACTOR;
            bytes.extend_from_slice(&star_record(ra, dec, plx, 15.0, observed));
        }
        bytes
    }

    fn star_bytes(path: &str) -> Vec<u8> {
        std::fs::read(path).unwrap()
    }

    fn remove_file(path: &str) {
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn red_star_behind_a_dust_column_is_typical_after_the_subtraction() {
        let (center_ra, center_dec) = region_center();
        let map_path = "/tmp/opencode/db_probe_behind_dust.be19";
        let bin_path = "/tmp/opencode/db_probe_behind_dust.bin";
        write_be19(map_path, center_ra, center_dec, 0.2);
        let plx: Vec<f64> = vec![8.0, 6.5, 5.0, 4.0, 3.2, 2.6, 2.1, 1.7, 1.4, 1.1, 0.9, 0.8]
            .iter()
            .cycle()
            .take(24)
            .cloned()
            .collect();
        let candidate = Some((0.7, 0.30));
        let bin = build_field_bin(map_path, center_ra, center_dec, &plx, 0.004, candidate);
        std::fs::write(bin_path, bin).unwrap();
        let idx = build_star_index(&star_bytes(bin_path));
        assert_eq!(idx.stars.len(), 25);
        let report = scan_field(
            map_path,
            &idx,
            center_ra,
            center_dec,
            0.05,
            N_SIG_DEFAULT,
            MIN_NEIGHBORS_DEFAULT,
        )
        .unwrap();
        assert_eq!(report.members.len(), 25);
        let candidate_star = report
            .members
            .iter()
            .find(|m| m.star_idx == 24)
            .expect("the reddened candidate sits at the last record");
        assert_eq!(candidate_star.verdict, VerdictWord::Typical);
        assert!(
            (candidate_star.bp_rp0 - 0.30).abs() < 0.02,
            "the dust subtraction restores the injected intrinsic BP-RP0 {:.4}",
            candidate_star.bp_rp0
        );
        let reddest_observed = report
            .members
            .iter()
            .map(|m| m.observed_bp_rp)
            .fold(f64::NEG_INFINITY, f64::max);
        assert!(
            (candidate_star.observed_bp_rp - reddest_observed).abs() < 1e-9,
            "the candidate is the reddest star in observed color {:.3}",
            reddest_observed
        );
        remove_file(map_path);
        remove_file(bin_path);
    }

    #[test]
    fn intrinsic_red_survives_the_dust_subtraction_as_outlier() {
        let (center_ra, center_dec) = region_center();
        let map_path = "/tmp/opencode/db_probe_intrinsic_red.be19";
        let bin_path = "/tmp/opencode/db_probe_intrinsic_red.bin";
        write_be19(map_path, center_ra, center_dec, 0.05);
        let plx: Vec<f64> = vec![8.0, 6.5, 5.0, 4.0, 3.2, 2.6, 2.1, 1.7, 1.4, 1.1, 0.9, 0.8]
            .iter()
            .cycle()
            .take(24)
            .cloned()
            .collect();
        let candidate = Some((2.0, 1.6));
        let bin = build_field_bin(map_path, center_ra, center_dec, &plx, 0.004, candidate);
        std::fs::write(bin_path, bin).unwrap();
        let idx = build_star_index(&star_bytes(bin_path));
        assert_eq!(idx.stars.len(), 25);
        let report = scan_field(
            map_path,
            &idx,
            center_ra,
            center_dec,
            0.05,
            N_SIG_DEFAULT,
            MIN_NEIGHBORS_DEFAULT,
        )
        .unwrap();
        assert_eq!(report.members.len(), 25);
        let candidate_star = report
            .members
            .iter()
            .find(|m| m.star_idx == 24)
            .expect("the intrinsically red star sits at the last record");
        assert_eq!(candidate_star.verdict, VerdictWord::Outlier);
        let typical = report
            .members
            .iter()
            .filter(|m| m.verdict == VerdictWord::Typical)
            .count();
        assert_eq!(typical, 24);
        remove_file(map_path);
        remove_file(bin_path);
    }
}
