use omegaflow::archivar::redr;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("usage: redr_probe <file.odr>");
            std::process::exit(2);
        }
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{path}: {e}");
            std::process::exit(2);
        }
    };
    println!("{path}: {} bytes", bytes.len());
    println!(
        "record arithmetic: {} x {} = {}, rest {}",
        bytes.len() / redr::RECORD_BYTES,
        redr::RECORD_BYTES,
        (bytes.len() / redr::RECORD_BYTES) * redr::RECORD_BYTES,
        bytes.len() % redr::RECORD_BYTES
    );
    let records = match redr::parse(&bytes) {
        Some(r) => r,
        None => {
            eprintln!("the file is not whole REDR records");
            std::process::exit(1);
        }
    };
    let mut valid = 0usize;
    let mut invalid = 0usize;
    let mut recreated = 0usize;
    let mut s_low = 127i16;
    let mut s_high = -128i16;
    let mut x_low = 127i16;
    let mut x_high = -128i16;
    for record in &records {
        match record.header.validity {
            0 => valid += 1,
            2 => recreated += 1,
            _ => invalid += 1,
        }
        for &s in &record.s_band {
            s_low = s_low.min(s as i16);
            s_high = s_high.max(s as i16);
        }
        for &x in &record.x_band {
            x_low = x_low.min(x as i16);
            x_high = x_high.max(x as i16);
        }
    }
    let first = &records[0].header;
    let last = &records[records.len() - 1].header;
    println!("records: {}", records.len());
    println!("validity: {valid} valid, {invalid} invalid, {recreated} recreated");
    println!(
        "first: year {} day {} {:02}:{:02}:{:05.2}  rate {}",
        first.year, first.day, first.hour, first.minute, first.second, first.sample_rate
    );
    println!(
        "last:  year {} day {} {:02}:{:02}:{:05.2}  rate {}",
        last.year, last.day, last.hour, last.minute, last.second, last.sample_rate
    );
    let spacecraft = match redr::spacecraft_id(&bytes) {
        Some(v) => v.to_string(),
        None => "absent".to_string(),
    };
    let station = match redr::station_id(&bytes) {
        Some(v) => v.to_string(),
        None => "absent".to_string(),
    };
    println!("spacecraft {spacecraft} station {station} at the first record");
    println!("S-band samples: {s_low} .. {s_high}");
    println!("X-band samples: {x_low} .. {x_high}");
}
