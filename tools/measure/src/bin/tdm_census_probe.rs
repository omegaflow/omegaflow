use omegaflow::archivar::tdm;

fn main() {
    let arg = std::env::args().skip(1).find(|a| !a.starts_with('-'));
    let Some(path) = arg else {
        eprintln!("tdm-census: --file absent — no TDM to count");
        return;
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("tdm-census: read void ({path})");
        return;
    };
    let Some(tdm) = tdm::parse_tdm(&text) else {
        eprintln!("tdm-census: parse void — {path} is not a CCSDS TDM in key-value form");
        return;
    };
    println!(
        "tdm-census {path}: version {} · originator {} · {} .. {} · {} observations",
        tdm.version,
        tdm.originator,
        tdm.start_time,
        tdm.stop_time,
        tdm.observations.len()
    );
    if let (Some(a), Some(b)) = (&tdm.participant_1, &tdm.participant_2) {
        println!("tdm-census: participants {a} / {b}");
    }
    for (name, count) in tdm::counts(&tdm) {
        println!("tdm-census: {name} {count}");
    }
    let bad = tdm
        .observations
        .iter()
        .filter(|o| !o.value.is_finite())
        .count();
    if bad > 0 {
        println!("tdm-census: {bad} observation(s) not finite — named, not repaired");
    }
}
