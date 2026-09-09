use omegaflow_measure::depthphase as dp;

const SOURCE_LAT: f64 = -8.3514;
const SOURCE_LON: f64 = 121.3478;
const SOURCE_DEPTH_KM: f64 = 10.0;

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
            let pp = dp::p_p_lag(d, h)
                .map(|v| format!("{v:.1}"))
                .unwrap_or("-".into());
            let sp = dp::s_p_lag(d, h)
                .map(|v| format!("{v:.1}"))
                .unwrap_or("-".into());
            println!("  h = {h:>3.0} km   pP-P = {pp:>6} s   sP-P = {sp:>6} s");
        }
    }
    println!();
    let deltas = vec![30.0, 45.0, 60.0, 75.0, 90.0];
    let lags: Vec<f64> = deltas
        .iter()
        .filter_map(|&d| dp::p_p_lag(d, SOURCE_DEPTH_KM))
        .collect();
    let deltas_valid: Vec<f64> = deltas
        .iter()
        .copied()
        .filter(|&d| dp::p_p_lag(d, SOURCE_DEPTH_KM).is_some())
        .collect();
    let recovered = dp::invert_depth_multi(&deltas_valid, &lags);
    println!(
        "synthetic pP-P lags at the catalog depth ({:.0} km) invert to h = {:?} km",
        SOURCE_DEPTH_KM, recovered
    );
    let lag_per_5km = dp::p_p_lag(30.0, 15.0).unwrap() - dp::p_p_lag(30.0, 10.0).unwrap();
    println!(
        "the pP-P lag reads the source depth directly (~2 h/vp): {:.1} s of lag per 5 km, so a {:.1} s pick scatter maps to {:.1} km",
        lag_per_5km,
        1.0,
        5.0 / lag_per_5km
    );
}
