use std::collections::HashMap;
use std::sync::Arc;

use omegaflow::archivar::{parse_ephemeris_binary, BodyEphemeris};
use omegaflow::dastcom::{parse_record, AsteroidRec, RECORD_STRIDE};
use omegaflow::gaia_sso::{parse_bin, GaiaBody};
use omegaflow::weberin::{GaiaFold, Weberin};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() {
    println!(
        "usage: gaia_sso_weave_probe [--eph-dir <data-root>] [--dastcom <dastcom_asteroids.bin>] [--gaia <gaia_sso_tno.bin>]"
    );
}

fn read_recs(path: &str) -> Option<Vec<AsteroidRec>> {
    let bytes = std::fs::read(path).ok()?;
    Some(
        bytes
            .chunks_exact(RECORD_STRIDE)
            .filter_map(parse_record)
            .collect(),
    )
}

fn read_eph(dir: &str, name: &str) -> Option<BodyEphemeris> {
    let path = format!("{dir}/ssd.jpl.nasa.gov/ephemeris_{name}.bin");
    let bytes = std::fs::read(&path).ok()?;
    parse_ephemeris_binary(&bytes)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let eph_dir = match arg_value(&args, "--eph-dir") {
        Some(d) => d,
        None => "data".to_string(),
    };
    let dastcom_path = match arg_value(&args, "--dastcom") {
        Some(d) => d,
        None => "data/ssd.jpl.nasa.gov/dastcom_asteroids.bin".to_string(),
    };
    let gaia_path = match arg_value(&args, "--gaia") {
        Some(d) => d,
        None => "data/gea.esac.esa.int/gaia_sso_tno.bin".to_string(),
    };

    let recs = match read_recs(&dastcom_path) {
        Some(r) if !r.is_empty() => r,
        _ => {
            println!("gaia_sso weave: {dastcom_path} bin void — the MPC-Kepler line stays unread");
            return;
        }
    };
    let gaia_bytes = match std::fs::read(&gaia_path) {
        Ok(b) => b,
        Err(_) => {
            println!(
                "gaia_sso weave: {gaia_path} bin void — run gaia_sso_compiler first; the Gaia line stays unread"
            );
            return;
        }
    };
    let gaia_bodies: Vec<GaiaBody> = match parse_bin(&gaia_bytes) {
        Some(b) => b,
        None => {
            println!("gaia_sso weave: {gaia_path} reads but does not parse to transit bodies");
            return;
        }
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut sun_map: HashMap<String, BodyEphemeris> = HashMap::new();
    let Some(earth) = read_eph(&eph_dir, "earth") else {
        println!(
            "gaia_sso weave: the earth ephemeris bin is void — the observer frame stays unread"
        );
        return;
    };
    eph.insert("earth".to_string(), earth);
    let Some(sun) = read_eph(&eph_dir, "sun") else {
        println!(
            "gaia_sso weave: the sun ephemeris bin is void — the heliocentric fold stays unread"
        );
        return;
    };
    sun_map.insert("sun".to_string(), sun);

    let mut w = Weberin::new();
    w.eph = Some(Arc::new(eph));
    w.sun = Some(Arc::new(sun_map));
    let verdicts = w.weave_gaia_line(&recs, &[], &gaia_bodies);
    if verdicts.is_empty() {
        println!("gaia_sso weave: no body carries both lines — the fold stays closed");
        return;
    }
    let transits_all: usize = gaia_bodies.iter().map(|b| b.transits.len()).sum();
    println!("=== the second body line — MPC Kepler (dastcom elements) against the Gaia DR3 SSO measured astrometry (angular, no distance fabricated) ===");
    let mut placed = 0usize;
    let mut riss = 0usize;
    let mut absent = 0usize;
    let mut unjudgeable = 0usize;
    for v in &verdicts {
        match &v.fold {
            GaiaFold::Placed {
                total,
                within_sigma,
                median_sep_arcsec,
                median_sigma_arcsec,
                max_sep_arcsec,
            } => {
                placed += 1;
                println!(
                    "weberin-gaia {} placed angular transits {total} within-sigma {within_sigma} median-residual {median_sep_arcsec:.3} arcsec median-gaia-sigma {median_sigma_arcsec:.3} arcsec max-residual {max_sep_arcsec:.3} arcsec",
                    v.name
                );
            }
            GaiaFold::Riss {
                total,
                within_sigma,
                median_sep_arcsec,
                median_sigma_arcsec,
                max_sep_arcsec,
            } => {
                riss += 1;
                println!(
                    "weberin-gaia {} riss angular transits {total} within-sigma {within_sigma} median-residual {median_sep_arcsec:.3} arcsec median-gaia-sigma {median_sigma_arcsec:.3} arcsec max-residual {max_sep_arcsec:.3} arcsec knot mpc-keplerian+gaia-astrometry",
                    v.name
                );
            }
            GaiaFold::Absent { line } => {
                absent += 1;
                println!("weberin-gaia {} absent sep absent missing {line}", v.name);
            }
            GaiaFold::Unjudgeable { transits } => {
                unjudgeable += 1;
                println!(
                    "weberin-gaia {} unjudgeable transits {transits} — the observer reference is void at the transit epochs",
                    v.name
                );
            }
        }
    }
    println!(
        "weberin-gaia tally: {placed} placed | {riss} riss | {absent} absent | {unjudgeable} unjudgeable | {transits_all} Gaia transits held"
    );
}
