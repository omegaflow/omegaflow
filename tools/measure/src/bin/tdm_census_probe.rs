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
    let total = tdm::observations(&tdm).count();
    println!(
        "tdm-census {path}: version {} · originator {} · {} block(s) · {total} observation(s)",
        tdm.version,
        tdm.originator,
        tdm.blocks.len()
    );
    for (i, block) in tdm.blocks.iter().enumerate() {
        println!(
            "tdm-census: block {} {} .. {} · {} observations · freq_offset {:?} · correction_range {:?} · range_modulus {:?}",
            i + 1,
            block.meta.start_time,
            block.meta.stop_time,
            block.observations.len(),
            block.meta.freq_offset,
            block.meta.correction_range,
            block.meta.range_modulus,
        );
    }
    for (name, count) in tdm::counts(&tdm) {
        println!("tdm-census: {name} {count}");
    }
    let bad = tdm::observations(&tdm)
        .filter(|o| !o.value.is_finite())
        .count();
    if bad > 0 {
        println!("tdm-census: {bad} observation(s) not finite — named, not repaired");
    }
}
