use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mariner_occlt;
use omegaflow::cdn::upload_release;

const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/mariner/mariner10/celestial_mechanics_and_radio_science/red_tele_signal_data_venus_occlt/";

const TARS: [&str; 9] = [
    "PSPA-00316_DD029604_05-FEB-74.tar",
    "PSPA-00316_DD029605_05-FEB-74.tar",
    "PSPA-00316_DD029606_05-FEB-74.tar",
    "PSPA-00316_DD029607_05-FEB-74.tar",
    "PSPA-00316_DD029626_05-FEB-74.tar",
    "PSPA-00316_DD029627_05-FEB-74.tar",
    "PSPA-00316_DD029655_05-FEB-74.tar",
    "PSPA-00316_DD029656_05-FEB-74.tar",
    "PSPA-00316_DD029657_05-FEB-74.tar",
];

fn tar_octal(field: &[u8]) -> Option<usize> {
    let end = field
        .iter()
        .position(|&b| b == 0 || b == b' ')
        .unwrap_or(field.len());
    usize::from_str_radix(std::str::from_utf8(&field[..end]).ok()?, 8).ok()
}

fn tar_dat(bytes: &[u8]) -> Option<Vec<u8>> {
    let mut off = 0usize;
    while off + 512 <= bytes.len() {
        let header = &bytes[off..off + 512];
        if header.iter().all(|&b| b == 0) {
            return None;
        }
        let name_end = header[0..100].iter().position(|&b| b == 0).unwrap_or(100);
        let name = std::str::from_utf8(&header[0..name_end]).ok()?.to_string();
        let size = tar_octal(&header[124..136])?;
        let data_start = off + 512;
        let data_end = data_start + size;
        if data_end > bytes.len() {
            return None;
        }
        if name.ends_with(".DAT") {
            return Some(bytes[data_start..data_end].to_vec());
        }
        off = data_start + ((size + 511) / 512) * 512;
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let mut rows: Vec<[f64; mariner_occlt::MOCC_STRIDE]> = Vec::new();
    let mut first_clock: Option<(u64, u64, u64)> = None;
    let mut last_clock: Option<(u64, u64, u64)> = None;
    for (file_index, tar) in TARS.iter().enumerate() {
        let url = format!("{BASE}{tar}");
        let Some(bytes) = fetch_raw_bytes(&url) else {
            eprintln!("{tar}: fetch void ({url})");
            continue;
        };
        let Some(dat) = tar_dat(&bytes) else {
            eprintln!("{tar}: no .DAT member ({} B)", bytes.len());
            continue;
        };
        let Some(recs) = mariner_occlt::parse(&dat) else {
            eprintln!("{tar}: parse void — {} B", dat.len());
            continue;
        };
        for (record_index, r) in recs.iter().enumerate() {
            let clock = (r.clock_hh, r.clock_mm, r.second);
            if first_clock.is_none() {
                first_clock = Some(clock);
            }
            last_clock = Some(clock);
            rows.push(mariner_occlt::to_bin_row(
                r,
                file_index as u64,
                record_index as u64,
            ));
        }
        eprintln!(
            "{tar}: {} records of {} samples",
            recs.len(),
            mariner_occlt::SAMPLES_PER_RECORD
        );
    }
    if let (Some(a), Some(b)) = (first_clock, last_clock) {
        eprintln!(
            "clock span {:02}:{:02}:{:02} .. {:02}:{:02}:{:02}",
            a.0, a.1, a.2, b.0, b.1, b.2
        );
    }
    if rows.is_empty() {
        eprintln!("mariner_occlt: no records — the series stays unwritten (0 honored)");
        return;
    }
    let out = "data/spdf.gsfc.nasa.gov/mariner_occlt.bin";
    std::fs::create_dir_all("data/spdf.gsfc.nasa.gov").ok();
    let bin = mariner_occlt::write_mocc_bin(&rows);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match mariner_occlt::parse_mocc_bin(&bin) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, {} B — roundtrip parses",
                parsed.len(),
                bin.len()
            );
        }
        None => eprintln!("{out}: roundtrip parse void — the series stays unverified"),
    }
    if ci_mode && !upload_release("spdf.gsfc.nasa.gov", out) {
        std::process::exit(1);
    }
}
