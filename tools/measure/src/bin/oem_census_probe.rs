use omegaflow::archivar::oem;

fn main() {
    let arg = std::env::args().skip(1).find(|a| !a.starts_with('-'));
    let Some(path) = arg else {
        eprintln!("oem-census: --file absent — no OEM to count");
        return;
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("oem-census: read void ({path})");
        return;
    };
    let Some(segments) = oem::parse_oem(&text) else {
        eprintln!("oem-census: parse void — {path} is not a CCSDS OEM in key-value form");
        return;
    };
    let total = oem::records(&segments);
    match oem::span(&segments) {
        Some((first, last)) => println!(
            "oem-census {path}: {} segment(s), {total} record(s), {first} .. {last}",
            segments.len()
        ),
        None => println!(
            "oem-census {path}: {} segment(s), no record",
            segments.len()
        ),
    }
    for (i, seg) in segments.iter().enumerate() {
        println!(
            "oem-census: segment {} {} / {} / {} / {} · {} records · {}",
            i + 1,
            seg.object_name,
            seg.center_name,
            seg.ref_frame,
            seg.time_system,
            seg.records.len(),
            seg.interpolation,
        );
    }
}
