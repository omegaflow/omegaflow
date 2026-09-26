use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::voyager_occlt::{
    pack_many, parse_mediumband, parse_narrowband, parse_packed, parse_series,
};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "spdf.gsfc.nasa.gov";

const NARROWBAND_BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/voyager/voyager1/radio_science_rss/saturn_occultation_narrow_band/";
const NARROWBAND_TARS: [&str; 4] = [
    "PSPA-00217_DD059825_13-NOV-80.tar",
    "PSPA-00217_DD059826_13-NOV-80.tar",
    "PSPA-00217_DD059827_13-NOV-80.tar",
    "PSPA-00217_DD059828_13-NOV-80.tar",
];

const MEDIUMBAND_BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/voyager/voyager1/radio_science_rss/titan_occultation_medium_band/";
const MEDIUMBAND_TARS: [&str; 8] = [
    "PSPA-00189_DD059817_12-NOV-80.tar",
    "PSPA-00189_DD059818_12-NOV-80.tar",
    "PSPA-00189_DD059819_12-NOV-80.tar",
    "PSPA-00189_DD059820_12-NOV-80.tar",
    "PSPA-00189_DD059821_12-NOV-80.tar",
    "PSPA-00189_DD059822_12-NOV-80.tar",
    "PSPA-00189_DD059823_12-NOV-80.tar",
    "PSPA-00189_DD059824_12-NOV-80.tar",
];

fn tar_octal(field: &[u8]) -> Option<usize> {
    let end = field
        .iter()
        .position(|&b| b == 0 || b == b' ')
        .unwrap_or(field.len());
    usize::from_str_radix(std::str::from_utf8(&field[..end]).ok()?, 8).ok()
}

fn tar_members(bytes: &[u8]) -> Option<Vec<(String, Vec<u8>)>> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + 512 <= bytes.len() {
        let header = &bytes[off..off + 512];
        if header.iter().all(|&b| b == 0) {
            return Some(out);
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
            let leaf = name.rsplit('/').next().unwrap_or(&name).to_string();
            out.push((leaf, bytes[data_start..data_end].to_vec()));
        }
        off = data_start + ((size + 511) / 512) * 512;
    }
    Some(out)
}

fn tar_year(tar: &str) -> Option<u32> {
    let stem = tar.strip_suffix(".tar")?;
    let date = stem.rsplit('_').next()?;
    let yy: u32 = date.rsplit('-').next()?.parse().ok()?;
    if (70..=99).contains(&yy) {
        Some(1900 + yy)
    } else if yy <= 69 {
        Some(2000 + yy)
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
    {
        Some(path) => path.clone(),
        None => "data/spdf.gsfc.nasa.gov/voyager_occlt.bin".to_string(),
    };
    let mut files: Vec<(Vec<u8>, String, u32)> = Vec::new();
    let mut medium_records = 0usize;
    let mut narrow_records = 0usize;
    for (base, tars) in [
        (NARROWBAND_BASE, &NARROWBAND_TARS[..]),
        (MEDIUMBAND_BASE, &MEDIUMBAND_TARS[..]),
    ] {
        for tar in tars {
            let year = match tar_year(tar) {
                Some(y) => y,
                None => {
                    eprintln!("{tar}: tar year void — rows stay pending (0 honored)");
                    0
                }
            };
            let url = format!("{base}{tar}");
            let Some(bytes) = fetch_raw_bytes(&url) else {
                eprintln!("{tar}: fetch void ({url})");
                continue;
            };
            let Some(members) = tar_members(&bytes) else {
                eprintln!("{tar}: tar read void");
                continue;
            };
            for (name, dat) in members {
                if let Some(recs) = parse_mediumband(&dat) {
                    medium_records += recs.len();
                    files.push((dat, name, year));
                } else if let Some(nb) = parse_narrowband(&dat) {
                    narrow_records += nb.records.len();
                    files.push((dat, name, year));
                } else {
                    eprintln!("{name} ({tar}): framing void — member skipped");
                }
            }
        }
    }
    if files.is_empty() {
        eprintln!("voyager_occlt: no framed .DAT members — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let refs: Vec<(&[u8], &str, u32)> = files
        .iter()
        .map(|(bytes, name, year)| (bytes.as_slice(), name.as_str(), *year))
        .collect();
    let bin = pack_many(&refs);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let Some(parsed) = parse_packed(&bin) else {
        eprintln!("{out}: packed read void — the series stays unverified (0 honored)");
        std::process::exit(1);
    };
    if parsed.files.len() != refs.len() {
        eprintln!("{out}: packed member count void — the series stays unverified (0 honored)");
        std::process::exit(1);
    }
    let Some(rows) = parse_series(&bin) else {
        eprintln!("{out}: series read void — the series stays unverified (0 honored)");
        std::process::exit(1);
    };
    eprintln!(
        "{out}: {} .DAT member(s) packed ({} mediumband + {} narrowband records), {} B, {} series rows",
        parsed.files.len(),
        medium_records,
        narrow_records,
        bin.len(),
        rows.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::voyager_occlt::{
        MED_HEADER_BYTES, MED_MAGIC, MED_RECORD_BYTES, NB_DATA_RECORD_BYTES, NB_HEADER_RECORD_BYTES,
    };

    fn tar_bytes(members: &[(&str, &[u8])]) -> Vec<u8> {
        let mut out = Vec::new();
        for (name, data) in members {
            let mut header = [0u8; 512];
            let nameb = name.as_bytes();
            header[..nameb.len()].copy_from_slice(nameb);
            let mode = b"0000644\0";
            header[100..108].copy_from_slice(mode);
            let uid = b"0006752\0";
            header[108..116].copy_from_slice(uid);
            let gid = b"0000716\0";
            header[116..124].copy_from_slice(gid);
            let size = format!("{:011o}\0", data.len());
            header[124..136].copy_from_slice(size.as_bytes());
            let mtime = b"11523022400\0";
            header[136..148].copy_from_slice(mtime);
            header[148..156].copy_from_slice(b"015667\0 ");
            out.extend_from_slice(&header);
            out.extend_from_slice(data);
            let pad = (512 - data.len() % 512) % 512;
            out.extend_from_slice(&vec![0u8; pad]);
        }
        out.extend_from_slice(&[0u8; 1024]);
        out
    }

    fn sample_mediumband_dat(records: usize) -> Vec<u8> {
        let mut dat = Vec::new();
        for _ in 0..records {
            let mut rec = vec![0u8; MED_RECORD_BYTES];
            rec[0..2].copy_from_slice(&4800u16.to_be_bytes());
            rec[6..10].copy_from_slice(&MED_MAGIC.to_be_bytes());
            rec[12..14].copy_from_slice(&512u16.to_be_bytes());
            rec[2..6].copy_from_slice(&1_231_585_997u32.to_be_bytes());
            let data_base = 2 + MED_HEADER_BYTES;
            for k in 0..512 {
                let base = data_base + k * 8;
                let (i, q) = if k == 0 {
                    (1.0f32, 0.0f32)
                } else {
                    (0.0f32, 1.0f32)
                };
                rec[base..base + 4].copy_from_slice(&i.to_be_bytes());
                rec[base + 4..base + 8].copy_from_slice(&q.to_be_bytes());
            }
            dat.extend_from_slice(&rec);
        }
        dat
    }

    #[test]
    fn tar_members_extracts_every_dat_member() {
        let f1 = sample_mediumband_dat(1);
        let f2 = sample_mediumband_dat(1);
        let tar = tar_bytes(&[
            ("PSPA-00189_DD059817_12-NOV-80.dir/DD059817_F1.DAT", &f1),
            ("PSPA-00189_DD059817_12-NOV-80.dir/DD059817_F2.DAT", &f2),
        ]);
        let members = tar_members(&tar).unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].0, "DD059817_F1.DAT");
        assert_eq!(members[0].1, f1);
        assert_eq!(members[1].0, "DD059817_F2.DAT");
        assert_eq!(members[1].1, f2);
    }

    #[test]
    fn pack_and_parse_series_hold_for_measured_members() {
        let med = sample_mediumband_dat(2);
        let refs: Vec<(&[u8], &str, u32)> = vec![(&med[..], "DD059817_F1.DAT", 1980)];
        let bin = pack_many(&refs);
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0].year, 1980);
        let rows = parse_series(&bin).unwrap();
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0].0, 315_532_800.0);
    }

    #[test]
    fn tar_year_reads_two_digit_year() {
        assert_eq!(tar_year("PSPA-00189_DD059817_12-NOV-80.tar"), Some(1980));
        assert_eq!(tar_year("PSPA-00217_DD059825_13-NOV-80.tar"), Some(1980));
        assert_eq!(tar_year("S0A.tar"), None);
        assert_eq!(tar_year("S0A"), None);
    }

    #[test]
    fn narrowband_geometry_totals_hold() {
        assert_eq!(NB_HEADER_RECORD_BYTES, 122);
        assert_eq!(NB_DATA_RECORD_BYTES, 4098);
    }
}
