use omegaflow::ak135::{p_p_travel, p_travel_depth, s_p_travel};

const SOURCE_LAT: f64 = -8.3514;
const SOURCE_LON: f64 = 121.3478;
const SOURCE_DEPTH_KM: f64 = 10.0;
const DEPTHS_FINE: [f64; 21] = [
    0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0,
    150.0, 200.0, 250.0, 300.0, 400.0,
];

fn p_p_lag(delta_deg: f64, depth_km: f64) -> Option<f64> {
    Some(p_p_travel(delta_deg, depth_km)? - p_travel_depth(delta_deg, depth_km)?)
}

fn s_p_lag(delta_deg: f64, depth_km: f64) -> Option<f64> {
    Some(s_p_travel(delta_deg, depth_km)? - p_travel_depth(delta_deg, depth_km)?)
}

fn invert_depth_from_p_p_lag(deltas: &[f64], lags: &[f64]) -> Option<f64> {
    let mut best = (f64::INFINITY, 0.0f64);
    for &h in DEPTHS_FINE.iter() {
        let mut s = 0.0;
        let mut ok = true;
        for (&d, &lag) in deltas.iter().zip(lags.iter()) {
            match p_p_lag(d, h) {
                Some(pred) => {
                    let r = lag - pred;
                    s += r * r;
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok && s < best.0 {
            best = (s, h);
        }
    }
    Some(best.1)
}

fn main() {
    println!("=== depth-phase diagonal — pP/sP lag vs source depth (ak135) ===");
    println!(
        "source {} N {} E (us6000tkt2), catalog depth {:.0} km",
        SOURCE_LAT, SOURCE_LON, SOURCE_DEPTH_KM
    );
    println!();
    for &d in [30.0, 60.0, 90.0].iter() {
        println!("delta = {d:.0} deg:");
        for &h in [0.0, 10.0, 20.0, 35.0, 50.0, 100.0, 200.0].iter() {
            let pp = p_p_lag(d, h)
                .map(|v| format!("{v:.1}"))
                .unwrap_or("-".into());
            let sp = s_p_lag(d, h)
                .map(|v| format!("{v:.1}"))
                .unwrap_or("-".into());
            println!("  h = {h:>3.0} km   pP-P = {pp:>6} s   sP-P = {sp:>6} s");
        }
    }
    println!();
    let deltas = vec![30.0, 45.0, 60.0, 75.0, 90.0];
    let lags: Vec<f64> = deltas
        .iter()
        .filter_map(|&d| p_p_lag(d, SOURCE_DEPTH_KM))
        .collect();
    let deltas_valid: Vec<f64> = deltas
        .iter()
        .copied()
        .filter(|&d| p_p_lag(d, SOURCE_DEPTH_KM).is_some())
        .collect();
    let recovered = invert_depth_from_p_p_lag(&deltas_valid, &lags);
    println!(
        "synthetic pP-P lags at the catalog depth ({:.0} km) invert to h = {:?} km",
        SOURCE_DEPTH_KM, recovered
    );
    let lag_per_5km = p_p_lag(30.0, 15.0).unwrap() - p_p_lag(30.0, 10.0).unwrap();
    println!(
        "the pP-P lag reads the source depth directly (~2 h/vp): {:.1} s of lag per 5 km, so a {:.1} s pick scatter maps to {:.1} km",
        lag_per_5km,
        1.0,
        5.0 / lag_per_5km
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_p_lag_vanishes_at_zero_depth() {
        for d in [20.0, 40.0, 60.0, 80.0] {
            let lag = p_p_lag(d, 0.0).unwrap();
            assert!(lag.abs() < 1e-6, "pP-P at h=0 should vanish, got {lag}");
        }
    }

    #[test]
    fn lag_grows_across_the_shallow_band() {
        for d in [20.0, 30.0, 60.0] {
            let l0 = p_p_lag(d, 0.0).unwrap();
            let l10 = p_p_lag(d, 10.0).unwrap();
            let l20 = p_p_lag(d, 20.0).unwrap();
            let l100 = p_p_lag(d, 100.0).unwrap();
            assert!(
                l10 > l0 && l20 > l10,
                "shallow lag must grow: {l0} {l10} {l20}"
            );
            assert!(l100 > l20, "deep lag above shallow: {l20} vs {l100}");
        }
    }

    #[test]
    fn s_p_lag_exceeds_p_p_lag() {
        for d in [20.0, 40.0, 60.0] {
            for h in [10.0, 20.0, 35.0, 50.0] {
                assert!(s_p_lag(d, h).unwrap() > p_p_lag(d, h).unwrap());
            }
        }
    }

    #[test]
    fn invert_recovers_the_source_depth() {
        for true_h in [10.0, 20.0, 35.0] {
            let deltas = [20.0, 35.0, 50.0, 65.0, 80.0];
            let lags: Vec<f64> = deltas
                .iter()
                .map(|&d| p_p_lag(d, true_h).unwrap())
                .collect();
            let h = invert_depth_from_p_p_lag(&deltas, &lags).unwrap();
            assert!(
                (h - true_h).abs() <= 5.0,
                "inverted {h} km vs true {true_h} km"
            );
        }
    }

    #[test]
    fn inversion_survives_a_second_of_pick_scatter() {
        let true_h = 20.0;
        let deltas = [20.0, 35.0, 50.0, 65.0, 80.0];
        let lags: Vec<f64> = deltas
            .iter()
            .map(|&d| p_p_lag(d, true_h).unwrap() + 0.5)
            .collect();
        let h = invert_depth_from_p_p_lag(&deltas, &lags).unwrap();
        assert!(
            (h - true_h).abs() <= 10.0,
            "with +0.5 s bias, inverted {h} km"
        );
    }
}
