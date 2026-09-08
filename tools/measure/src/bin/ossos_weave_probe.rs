use omegaflow::archivar::mpcorb::{self, MpcorbRec};
use omegaflow::archivar::ossos::{self, OssosRec};

const REL_A_TOL: f64 = 0.03;
const E_TOL: f64 = 0.05;
const I_TOL: f64 = 2.0;

fn load_mpcorb(path: &str) -> Vec<MpcorbRec> {
    match std::fs::read(path) {
        Ok(b) => b
            .chunks_exact(mpcorb::RECORD_STRIDE)
            .filter_map(mpcorb::parse_record)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn load_ossos(path: &str) -> Vec<OssosRec> {
    match std::fs::read(path) {
        Ok(b) => b
            .chunks_exact(ossos::OSSOS_RECORD_STRIDE)
            .filter_map(ossos::parse_record)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn main() {
    let mpcorb_path = "data/ssd.jpl.nasa.gov/mpcorb_distant.bin";
    let ossos_path = "data/cdsarc.cds.unistra.fr/ossos_tno.bin";

    let mpcorb_recs = load_mpcorb(mpcorb_path);
    let ossos_recs = load_ossos(ossos_path);
    if mpcorb_recs.is_empty() || ossos_recs.is_empty() {
        println!(
            "ossos weave: a line is void — mpcorb {} records, ossos {} records",
            mpcorb_recs.len(),
            ossos_recs.len()
        );
        return;
    }

    let mut matched = 0usize;
    let mut unmatched = 0usize;
    let mut sum_da = 0.0f64;
    let mut sum_de = 0.0f64;
    let mut sum_di = 0.0f64;
    let mut n_da = 0usize;
    for o in &ossos_recs {
        let mut best: Option<(&MpcorbRec, f64)> = None;
        for m in &mpcorb_recs {
            let da = (o.a_au - m.a_au).abs() / o.a_au;
            let de = (o.e - m.e).abs();
            let di = (o.i_deg - m.incl_deg).abs();
            if da < REL_A_TOL && de < E_TOL && di < I_TOL {
                let score = da + de;
                if best.map(|(_, s)| score < s).unwrap_or(true) {
                    best = Some((m, score));
                }
            }
        }
        let Some((m, _)) = best else {
            unmatched += 1;
            continue;
        };
        matched += 1;
        let da_rel = (o.a_au - m.a_au).abs() / o.a_au;
        let de = (o.e - m.e).abs();
        let di = (o.i_deg - m.incl_deg).abs();
        sum_da += da_rel;
        sum_de += de;
        sum_di += di;
        n_da += 1;
        println!(
            "ossos {} matched mpcorb | da/a {:.4e} | de {:.4e} | di {:.4e} deg (ossos a {:.6} e {:.6} i {:.3} vs mpcorb a {:.6} e {:.6} i {:.3})",
            ossos::desig_of(o),
            da_rel,
            de,
            di,
            o.a_au,
            o.e,
            o.i_deg,
            m.a_au,
            m.e,
            m.incl_deg
        );
    }
    let (ma, me, mi) = if n_da > 0 {
        (
            sum_da / n_da as f64,
            sum_de / n_da as f64,
            sum_di / n_da as f64,
        )
    } else {
        (0.0, 0.0, 0.0)
    };
    println!(
        "ossos tally: {matched}/{} matched mpcorb | unmatched {unmatched} | mean |da|/a {ma:.4e} | mean |de| {me:.4e} | mean |di| {mi:.4e} deg",
        ossos_recs.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tolerances_are_positive() {
        assert!(REL_A_TOL > 0.0);
        assert!(E_TOL > 0.0);
        assert!(I_TOL > 0.0);
    }
}
