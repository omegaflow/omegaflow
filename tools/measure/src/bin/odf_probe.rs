fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            println!("usage: odf_probe <file.SC226>");
            return;
        }
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            println!("odf_probe: {path}: {e}");
            return;
        }
    };
    let orbits = omegaflow::odf::parse_odf(&bytes).map(|v| v.len());
    let label = omegaflow::odf::PK_FILE_LABEL.to_be_bytes();
    let mut origin = None;
    for i in 0..8usize {
        if bytes.get(i..i + 4).is_some_and(|s| s == label) {
            origin = Some(i);
            break;
        }
    }
    let mut ramps = 0usize;
    let mut in_ramp = false;
    let mut blocks = 0usize;
    if let Some(o) = origin {
        let n = (bytes.len() - o) / omegaflow::odf::RECORD_BYTES;
        blocks = n;
        for i in 0..n {
            let at = o + i * omegaflow::odf::RECORD_BYTES;
            let w = u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap_or([0; 4]));
            match w {
                omegaflow::odf::PK_FILE_LABEL
                | omegaflow::odf::PK_IDENTIFIER
                | omegaflow::odf::PK_ORBIT_HEADER
                | omegaflow::odf::PK_RAMP
                | omegaflow::odf::PK_CLOCK
                | omegaflow::odf::PK_SUMMARY => {
                    in_ramp = w == omegaflow::odf::PK_RAMP;
                }
                _ => {
                    if in_ramp {
                        ramps += 1;
                    }
                }
            }
        }
    }
    println!(
        "{} bytes, {} records, orbit rows {:?}, ramp rows {}",
        bytes.len(),
        blocks,
        orbits,
        ramps
    );
}
