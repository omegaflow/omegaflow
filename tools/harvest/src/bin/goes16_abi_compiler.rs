use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::goes_abi::s3::S3Key;
use omegaflow::goes_abi::{self, CALIB_GSICS_PENDING, calib_name, parse_granule, parse_gsics_txt};

const CDN_TAG: &str = "noaa-goes16.s3.amazonaws.com";
const BUCKET: &str = "noaa-goes16.s3.amazonaws.com";
const RADC_PREFIX: &str = "ABI-L1b-RadC/";
const GSICS_DEFAULT_URL: &str = "https://www.star.nesdis.noaa.gov/GOESCal/images/GSICS/GSICS_Harmonization_release_May2025_current.txt";
const CHANNELS: usize = 16;

fn newest_subdir(prefix: &str) -> Option<String> {
    let list = goes_abi::s3::list(BUCKET, prefix)?;
    list.prefixes.into_iter().max()
}

fn latest_hour_prefix() -> Option<String> {
    let year = newest_subdir(RADC_PREFIX)?;
    let day = newest_subdir(&year)?;
    newest_subdir(&day)
}

fn channel_index(name: &str) -> Option<usize> {
    let p = name.find("-M6C")? + 4;
    let digits = name.get(p..p + 2)?;
    let n = digits.parse::<usize>().ok()?;
    if (1..=CHANNELS).contains(&n) {
        Some(n - 1)
    } else {
        None
    }
}

fn newest_per_channel(keys: &[S3Key]) -> Vec<&S3Key> {
    let mut chosen: [Option<&S3Key>; CHANNELS] = [None; CHANNELS];
    for k in keys {
        let Some(name) = k.key.rsplit('/').next() else {
            continue;
        };
        if !(name.starts_with("OR_ABI-L1b-RadC-M6C")
            && name.contains("_G16_")
            && name.ends_with(".nc"))
        {
            continue;
        }
        let Some(ci) = channel_index(name) else {
            continue;
        };
        match chosen[ci] {
            Some(cur) if cur.key <= k.key => chosen[ci] = Some(k),
            None => chosen[ci] = Some(k),
            _ => {}
        }
    }
    chosen.into_iter().flatten().collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let gsics_text = if ci_mode {
        fetch_raw_bytes(GSICS_DEFAULT_URL).and_then(|b| String::from_utf8(b).ok())
    } else {
        None
    };
    let gsics_table = gsics_text.as_deref().map(parse_gsics_txt);
    let Some(hour) = latest_hour_prefix() else {
        eprintln!(
            "noaa-goes16: bucket index fetch void — the manifest stays unwritten (0 honored)"
        );
        std::process::exit(1);
    };
    let Some(list) = goes_abi::s3::list(BUCKET, &hour) else {
        eprintln!("{hour}: bucket index fetch void — the manifest stays unwritten (0 honored)");
        std::process::exit(1);
    };
    eprintln!("{hour}: {} keys in the newest hour", list.keys.len());
    let chosen = newest_per_channel(&list.keys);
    eprintln!("noaa-goes16: {} M6 RadC granules selected", chosen.len());
    if chosen.is_empty() {
        eprintln!(
            "noaa-goes16: no G16 M6 RadC granule in the newest hour — the manifest stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let mut records = Vec::with_capacity(chosen.len());
    for key in &chosen {
        let url = format!("https://{BUCKET}/{}", key.key);
        let Some(bytes) = fetch_raw_bytes(&url) else {
            eprintln!("{url}: fetch void — granule skipped (0 honored)");
            continue;
        };
        let granule = match parse_granule(&bytes, gsics_table.as_ref()) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("{url}: {e} — granule skipped");
                continue;
            }
        };
        if gsics_text.is_some() && granule.calib == CALIB_GSICS_PENDING {
            eprintln!(
                "gsics: band {} has no GOES-16 coefficient in the txt — calib stays gsics-pending",
                granule.band_id
            );
        }
        eprintln!(
            "granule: band {} wavelength {:.4} um t {:.1} (J2000 s) calib {} radiance mean {:.4} W m-2 sr-1 um-1",
            granule.band_id,
            granule.band_wavelength,
            granule.t,
            calib_name(granule.calib),
            granule.rad_mean
        );
        records.push(granule);
    }
    if records.is_empty() {
        eprintln!("noaa-goes16: no granule parsed — the manifest stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t));
    let Some(bin) = goes_abi::write_bin(&records) else {
        eprintln!(
            "noaa-goes16: a record refuses the GAB1 gate — the manifest stays unwritten (0 honored)"
        );
        std::process::exit(1);
    };
    let out = "data/noaa-goes16.s3.amazonaws.com/goes16_abi.bin";
    std::fs::create_dir_all("data/noaa-goes16.s3.amazonaws.com").ok();
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    let Some(parsed) = goes_abi::parse_bin(&bin) else {
        eprintln!("{out}: roundtrip parse void — the manifest stays unverified");
        std::process::exit(1);
    };
    let (Some(first), Some(last)) = (parsed.first(), parsed.last()) else {
        eprintln!("{out}: roundtrip parse empty — the manifest stays unverified");
        std::process::exit(1);
    };
    eprintln!(
        "{out}: {} granules (t {:.1}..{:.1}), {} B — roundtrip parses",
        parsed.len(),
        first.t,
        last.t,
        bin.len()
    );
    if ci_mode && !upload_release(CDN_TAG, out) {
        eprintln!("upload {out}: did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_index_reads_band_from_m6_name() {
        let name = "OR_ABI-L1b-RadC-M6C01_G16_s20250971801174_e20250971803547_c20250971803585.nc";
        assert_eq!(channel_index(name), Some(0));
        let name = "OR_ABI-L1b-RadC-M6C16_G16_s20250971801174_e20250971803547_c20250971803585.nc";
        assert_eq!(channel_index(name), Some(15));
        let name = "OR_ABI-L1b-RadC-M3C01_G16_s20250971801174_e20250971803547_c20250971803585.nc";
        assert_eq!(channel_index(name), None);
        let name = "OR_ABI-L1b-RadC-M6C17_G16_s20250971801174_e20250971803547_c20250971803585.nc";
        assert_eq!(channel_index(name), None);
        let name = "not_abi.nc";
        assert_eq!(channel_index(name), None);
    }

    #[test]
    fn newest_per_channel_keeps_only_latest_g16_m6() {
        let mk = |key: &str| S3Key {
            key: key.to_string(),
            size: 1,
        };
        let keys = vec![
            mk(
                "ABI-L1b-RadC/2025/097/18/OR_ABI-L1b-RadC-M6C01_G16_s20250971801174_e20250971803547_c20250971803585.nc",
            ),
            mk(
                "ABI-L1b-RadC/2025/097/18/OR_ABI-L1b-RadC-M6C01_G16_s20250971811174_e20250971813547_c20250971813584.nc",
            ),
            mk(
                "ABI-L1b-RadC/2025/097/18/OR_ABI-L1b-RadC-M6C02_G16_s20250971801174_e20250971803546_c20250971803586.nc",
            ),
            mk(
                "ABI-L1b-RadC/2025/097/18/OR_ABI-L1b-RadC-M3C01_G16_s20250971811174_e20250971813547_c20250971813584.nc",
            ),
            mk(
                "ABI-L1b-RadC/2025/097/18/OR_ABI-L1b-RadC-M6C01_G17_s20250971811174_e20250971813547_c20250971813584.nc",
            ),
        ];
        let chosen = newest_per_channel(&keys);
        assert_eq!(chosen.len(), 2);
        let c01 = chosen
            .iter()
            .find(|k| k.key.contains("M6C01"))
            .expect("C01 chosen");
        assert!(c01.key.contains("s20250971811174"));
        assert!(chosen.iter().any(|k| k.key.contains("M6C02")));
        assert!(!chosen.iter().any(|k| k.key.contains("M3C")));
        assert!(!chosen.iter().any(|k| k.key.contains("_G17_")));
    }
}
