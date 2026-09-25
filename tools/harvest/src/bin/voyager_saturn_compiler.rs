use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::voyager_saturn;
use omegaflow::cdn::upload_release;

const ROUTES: [&str; 2] = [
    "https://spdf.gsfc.nasa.gov/pub/data/voyager/voyager1/radio_science_rss/saturn_encounter_data/",
    "https://spdf.gsfc.nasa.gov/pub/data/voyager/voyager2/radio_science_rss/saturn_encounter_data/",
];

fn tars_of(base: &str) -> Vec<String> {
    let Some(bytes) = fetch_raw_bytes(base) else {
        eprintln!("voyager_saturn: dir listing void ({base})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("voyager_saturn: dir listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for token in text.split("href=\"") {
        let Some(end) = token.find('"') else {
            continue;
        };
        let name = &token[..end];
        if name.ends_with(".tar") {
            out.push(name.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

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
    let mut rows: Vec<[f64; voyager_saturn::VSAT_STRIDE]> = Vec::new();
    let mut stations = std::collections::BTreeSet::new();
    let mut first_time: Option<voyager_saturn::VoyagerSaturnTime> = None;
    let mut last_time: Option<voyager_saturn::VoyagerSaturnTime> = None;
    for base in ROUTES {
        let tars = tars_of(base);
        eprintln!("{base}: {} tar files", tars.len());
        for tar in tars {
            let url = format!("{base}{tar}");
            let Some(bytes) = fetch_raw_bytes(&url) else {
                eprintln!("{tar}: fetch void ({url})");
                continue;
            };
            let Some(dat) = tar_dat(&bytes) else {
                eprintln!("{tar}: no .DAT member ({} B)", bytes.len());
                continue;
            };
            let Some(recs) = voyager_saturn::parse(&dat) else {
                eprintln!("{tar}: parse void — {} B", dat.len());
                continue;
            };
            let mut by_kind = [0usize; 3];
            for r in &recs {
                let code = voyager_saturn::kind_code(r.kind) as usize;
                by_kind[code] += 1;
                stations.insert(r.station);
                if first_time.is_none() {
                    first_time = Some(r.time);
                }
                last_time = Some(r.time);
                rows.push(voyager_saturn::to_bin_row(r));
            }
            eprintln!(
                "{tar}: {} tracking records (doppler {}, range {}, angle {})",
                recs.len(),
                by_kind[0],
                by_kind[1],
                by_kind[2]
            );
        }
    }
    if let (Some(a), Some(b)) = (first_time, last_time) {
        let st: Vec<String> = stations.iter().map(u64::to_string).collect();
        eprintln!(
            "span {}-{:03} {:02}:{:02}:{:02} .. {}-{:03} {:02}:{:02}:{:02}; stations {}",
            a.year,
            a.day_of_year,
            a.hour,
            a.minute,
            a.second,
            b.year,
            b.day_of_year,
            b.hour,
            b.minute,
            b.second,
            st.join(",")
        );
    }
    if rows.is_empty() {
        eprintln!("voyager_saturn: no tracking records — the series stays unwritten (0 honored)");
        return;
    }
    let out = "data/spdf.gsfc.nasa.gov/voyager_saturn.bin";
    std::fs::create_dir_all("data/spdf.gsfc.nasa.gov").ok();
    let bin = voyager_saturn::write_vsat_bin(&rows);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match voyager_saturn::parse_vsat_bin(&bin) {
        Some(parsed) => {
            let mut by_kind = [0usize; 3];
            for r in &parsed {
                by_kind[r[0] as usize] += 1;
            }
            eprintln!(
                "{out}: {} tracking records (doppler {}, range {}, angle {}), {} B — roundtrip parses",
                parsed.len(),
                by_kind[0],
                by_kind[1],
                by_kind[2],
                bin.len()
            );
        }
        None => eprintln!("{out}: roundtrip parse void — the series stays unverified"),
    }
    if ci_mode && !upload_release("spdf.gsfc.nasa.gov", out) {
        std::process::exit(1);
    }
}
