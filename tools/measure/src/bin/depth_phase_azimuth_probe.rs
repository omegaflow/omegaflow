use omegaflow::archivar::fetch_raw;
use omegaflow_measure::depthphase as dp;
use omegaflow_measure::depthphase::{
    MAX_DIST_DEG, MAX_STATIONS, MIN_DIST_DEG, SNR_GATE, STATION_URL,
};

const PILOT_LAT: f64 = 36.5244;
const PILOT_LON: f64 = 70.3676;
const PILOT_DEPTH_KM: f64 = 231.0;
const PILOT_START: &str = "2015-10-26T09:09:42";
const PILOT_END: &str = "2015-10-26T09:34:42";

fn main() {
    println!(
        "=== depth-phase calibration azimuths — the six BHZ stations of pilot us10003re5 (M7.5 Hindu Kush) ==="
    );
    println!(
        "pilot: {PILOT_LAT:.4} N {PILOT_LON:.4} E, catalog depth {PILOT_DEPTH_KM:.1} km, origin {PILOT_START}"
    );
    println!(
        "selection rule: BHZ in the {MIN_DIST_DEG}..{MAX_DIST_DEG} deg band, sorted by distance, first {MAX_STATIONS} (the field-probe route)"
    );
    println!();

    let st_url = format!(
        "{STATION_URL}?format=text&level=channel&latitude={PILOT_LAT:.4}&longitude={PILOT_LON:.4}&minradius={MIN_DIST_DEG}&maxradius={MAX_DIST_DEG}&channel=BHZ&starttime={PILOT_START}&endtime={PILOT_END}&includerestricted=false"
    );
    let Some(st_body) = fetch_raw(&st_url, None, &[], 86400) else {
        eprintln!("station query carries no body — the azimuths stay unmeasured (0 honored)");
        return;
    };
    let mut stations = dp::parse_stations_text(&st_body);
    let mut seen = std::collections::HashSet::new();
    stations.retain(|s| seen.insert(format!("{}.{}", s.net, s.sta)));
    stations.sort_by(|a, b| {
        let da = dp::arc_deg(PILOT_LAT, PILOT_LON, a.lat, a.lon);
        let db = dp::arc_deg(PILOT_LAT, PILOT_LON, b.lat, b.lon);
        da.total_cmp(&db)
    });
    stations.truncate(MAX_STATIONS);
    println!(
        "{} BHZ stations in the {MIN_DIST_DEG}..{MAX_DIST_DEG} deg band",
        stations.len()
    );
    println!();

    println!(
        "{:>12}  {:>9}  {:>10}  {:>8}  {:>10}  {:>7}  {:>6}",
        "net.sta", "lat", "lon", "dist_deg", "azimut_deg", "snr", "gate"
    );
    let mut gate_pass = 0usize;
    for st in &stations {
        let key = format!("{}.{}", st.net, st.sta);
        let delta = dp::arc_deg(PILOT_LAT, PILOT_LON, st.lat, st.lon);
        let azimuth = dp::azimuth_deg(PILOT_LAT, PILOT_LON, st.lat, st.lon);
        let (snr_txt, gate_txt) = match dp::fetch_station_body(st, PILOT_START, PILOT_END) {
            Some((samples, rate)) => match dp::p_onset(&samples, rate) {
                Some(t_p) => {
                    let bp = dp::bandpass(&samples, rate);
                    let i_p = dp::onset_index(&samples, rate, t_p);
                    match dp::onset_snr(&bp, rate, i_p) {
                        Some(snr) if snr >= SNR_GATE => {
                            gate_pass += 1;
                            (format!("{snr:.1}"), "pass".to_string())
                        }
                        Some(snr) => (format!("{snr:.1}"), "below".to_string()),
                        None => ("-".to_string(), "no floor".to_string()),
                    }
                }
                None => ("-".to_string(), "no pick".to_string()),
            },
            None => ("-".to_string(), "no record".to_string()),
        };
        println!(
            "{key:>12}  {:>9.4}  {:>10.4}  {delta:>8.2}  {azimuth:>10.2}  {snr_txt:>7}  {gate_txt:>6}",
            st.lat, st.lon
        );
    }
    println!();
    println!(
        "{gate_pass} of {} stations passed the SNR gate >= {SNR_GATE}",
        stations.len()
    );
}
