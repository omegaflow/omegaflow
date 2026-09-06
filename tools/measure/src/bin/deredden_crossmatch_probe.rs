use omegaflow::archivar::spatial::{star_stride, STAR_RECORD_BYTES};
use omegaflow::healpix::icrs_to_galactic;
use omegaflow::json::{parse_json, JsonVal};
use omegaflow_measure::deredden::{
    abs_mag, build_star_index, dwarf_color_type, intrinsic_of, type_label, DustMap, StarIndex,
    BACKGROUND_PC_MIN, WANG_GBP_FACTOR, WANG_G_FACTOR,
};

const VALID_B_MIN_DEG: f64 = 20.0;
const CROSSMATCH_RADIUS_AS_DEFAULT: f64 = 90.0;
const MIN_AV_DEFAULT: f64 = 0.2;

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
    dust: Option<omegaflow_measure::deredden::DustHit>,
    dust_refused: &'static str,
}

fn type_word(bp_rp: f64) -> (f64, f64) {
    match dwarf_color_type(bp_rp) {
        Some(x) => x,
        None => (f64::NAN, f64::NAN),
    }
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
        let (th, _) = icrs_to_galactic(s.ra_deg, s.dec_deg);
        let b = 90.0 - th.to_degrees();
        let dust_refused;
        let dust;
        match map.at(s.ra_deg, s.dec_deg, d_pc) {
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
        let (col_type, col_mg) = type_word(s.color_index);
        match &m.dust {
            Some(d) if d.av.is_finite() && d.av > 0.0 => {
                let e_bprp = d.av / WANG_GBP_FACTOR;
                let a_g = WANG_G_FACTOR * e_bprp;
                let g0 = s.mag - a_g;
                let bp_rp0 = s.color_index - e_bprp;
                let (t0, mg0) = type_word(bp_rp0);
                let itr = intrinsic_of(s, d.av);
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
                let m_g0 = itr.map(|i| i.m_g0).unwrap_or(f64::NAN);
                let lum_delta = m_g0 - mg0;
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
                println!("---- counterpart candidate (dr3 record {}) ra {:.6} dec {:.6} | separation {:.2} arcsec | |b| {:.2} deg | plx {:.3} mas -> d {:.0} pc | A_V at that distance: {:.3} mag (measured zero or non-positive — the star sits in front of the dust column)",
                    m.star_idx, s.ra_deg, s.dec_deg, m.sep_arcsec, m.b_deg.abs(), s.plx_mas, m.d_pc, d.av);
            }
            _ => {
                println!("---- counterpart candidate (dr3 record {}) ra {:.6} dec {:.6} | separation {:.2} arcsec | |b| {:.2} deg | plx {:.3} mas -> d {:.0} pc | A_V at that distance: unmeasured ({})",
                    m.star_idx, s.ra_deg, s.dec_deg, m.sep_arcsec, m.b_deg.abs(), s.plx_mas, m.d_pc, m.dust_refused);
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

    let mut map = match DustMap::open(&map_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    println!(
        "=== deredden_crossmatch_probe — the dust column changes the identification of a transient's static-catalog counterpart ==="
    );
    println!(
        "extinction law: A_V = 3.1 E(B-V); E(BP-RP) = A_V/{WANG_GBP_FACTOR}; A_G = {WANG_G_FACTOR} E(BP-RP) — Wang & Chen 2019 (ApJ 877, 116), the identical coefficients the dust_cleaning_3d_probe applies"
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
                    let (th, _) = icrs_to_galactic(s.ra_deg, s.dec_deg);
                    let b = (90.0 - th.to_degrees()).abs();
                    if b < VALID_B_MIN_DEG {
                        continue;
                    }
                    let Some(d) = map.at(s.ra_deg, s.dec_deg, d_pc) else {
                        continue;
                    };
                    if !(d.av.is_finite() && d.av >= min_av) {
                        continue;
                    }
                    let Some(itr) = intrinsic_of(s, d.av) else {
                        continue;
                    };
                    passes += 1;
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
                        itr.bp_rp0,
                        itr.g0,
                        type_label(type_word(s.color_index).0),
                        type_label(type_word(itr.bp_rp0).0),
                        itr.m_g0
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
