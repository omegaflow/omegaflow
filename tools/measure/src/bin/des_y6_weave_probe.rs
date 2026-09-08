use std::collections::HashMap;

use omegaflow::archivar::des_y6::{self, DesY6Rec, DES_Y6_EPOCH_JD};
use omegaflow::archivar::mpcorb::{self, MpcorbRec};
use omegaflow::archivar::{
    body_barycenter_position, parse_ephemeris_binary, BodyEphemeris, J2000_EPOCH,
};

fn load_mpcorb(path: &str) -> Vec<MpcorbRec> {
    match std::fs::read(path) {
        Ok(b) => b
            .chunks_exact(mpcorb::RECORD_STRIDE)
            .filter_map(mpcorb::parse_record)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn load_des(path: &str) -> Vec<DesY6Rec> {
    match std::fs::read(path) {
        Ok(b) => b
            .chunks_exact(des_y6::DES_Y6_RECORD_STRIDE)
            .filter_map(des_y6::parse_record)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn separation_m(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn main() {
    let mpcorb_path = "data/ssd.jpl.nasa.gov/mpcorb_distant.bin";
    let des_path = "data/cdsarc.cds.unistra.fr/des_y6_tno.bin";
    let sun_path = "data/ssd.jpl.nasa.gov/ephemeris_sun.bin";
    let tol_m = 1.0e6;

    let mpcorb_recs = load_mpcorb(mpcorb_path);
    let des_recs = load_des(des_path);
    let sun_eph: Option<BodyEphemeris> = std::fs::read(sun_path)
        .ok()
        .and_then(|b| parse_ephemeris_binary(&b));

    if mpcorb_recs.is_empty() || des_recs.is_empty() {
        println!(
            "des_y6 weave: a line is void — mpcorb {} records, des {} records",
            mpcorb_recs.len(),
            des_recs.len()
        );
        return;
    }
    let Some(sun) = &sun_eph else {
        println!("des_y6 weave: ephemeris_sun.bin void — the fold to barycentric is unread");
        return;
    };
    let mut sun_map: HashMap<String, BodyEphemeris> = HashMap::new();
    sun_map.insert("sun".to_string(), sun.clone());
    let tdb = (DES_Y6_EPOCH_JD - J2000_EPOCH) * 86400.0;
    let Some(sun_p) = body_barycenter_position("sun", tdb, &sun_map) else {
        println!("des_y6 weave: the sun barycentric position at epoch is void");
        return;
    };

    let mut by_desig: HashMap<String, &MpcorbRec> = HashMap::new();
    for r in &mpcorb_recs {
        by_desig.entry(mpcorb::desig_of(r).to_string()).or_insert(r);
    }

    let mut placed = 0usize;
    let mut riss = 0usize;
    let mut unmatched = 0usize;
    let mut sigma_void = 0usize;
    for d in &des_recs {
        let desig = des_y6::desig_of(d).to_string();
        let Some(m) = by_desig.get(&desig) else {
            unmatched += 1;
            continue;
        };
        let Some((helio, _)) = mpcorb::state_at(m, DES_Y6_EPOCH_JD) else {
            sigma_void += 1;
            continue;
        };
        let mpc_bary = [
            helio[0] + sun_p[0],
            helio[1] + sun_p[1],
            helio[2] + sun_p[2],
        ];
        let sep = separation_m([d.x_m, d.y_m, d.z_m], mpc_bary);
        let verdict = if sep <= tol_m {
            placed += 1;
            "placed"
        } else {
            riss += 1;
            "riss"
        };
        println!(
            "des_y6 {desig} state {verdict} sep {sep:.3e} m des-sigma {:.3e} m",
            d.sigma_m
        );
    }
    println!(
        "des_y6 tally: {}/{} des record(s) matched mpcorb | placed {placed} | riss {riss} | unmatched {unmatched} | state_void {sigma_void} | tolerance {tol_m:.3e} m",
        des_recs.len() - unmatched - sigma_void,
        des_recs.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separation_is_euclidean() {
        assert_eq!(separation_m([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]), 5.0);
    }
}
