use omegaflow::archivar::aem;

fn main() {
    let arg = std::env::args().skip(1).find(|a| !a.starts_with('-'));
    let Some(path) = arg else {
        eprintln!("aem-census: --file absent — no AEM to count");
        return;
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("aem-census: read void ({path})");
        return;
    };
    let Some(segments) = aem::parse_aem(&text) else {
        eprintln!("aem-census: parse void — {path} is not a CCSDS AEM in key-value form");
        return;
    };
    let total = aem::records(&segments);
    let first = match segments.first() {
        Some(s) => s.start_time.clone(),
        None => String::new(),
    };
    let last = match segments.last() {
        Some(s) => s.stop_time.clone(),
        None => String::new(),
    };
    println!(
        "aem-census {path}: {} segment(s), {total} record(s), {first} .. {last}",
        segments.len()
    );
    let types: std::collections::BTreeSet<&str> =
        segments.iter().map(|s| s.attitude_type.as_str()).collect();
    let frames: std::collections::BTreeSet<String> = segments
        .iter()
        .map(|s| format!("{} -> {}", s.ref_frame_a, s.ref_frame_b))
        .collect();
    println!(
        "aem-census: attitude types {:?}, frames {:?}",
        types, frames
    );
    let declared: std::collections::BTreeSet<&str> = segments
        .iter()
        .filter_map(|s| s.quaternion_type.as_deref())
        .collect();
    if declared.is_empty() {
        println!(
            "aem-census: quaternion order Q1 Q2 Q3 QC by CCSDS 504.0-B-2 table 4-4; QUATERNION_TYPE was removed in that revision, its absence is correct"
        );
    }
    match aem::worst_norm_deviation(&segments) {
        Some((dev, time)) => println!("aem-census: worst |q|-1 = {dev:.3e} at {time}"),
        None => println!("aem-census: no quaternion record"),
    }
}
