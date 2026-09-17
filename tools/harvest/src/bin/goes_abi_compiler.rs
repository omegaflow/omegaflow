use omegaflow::cdn::upload_release;
use omegaflow::goes_abi::{
    self, CALIB_GSICS_PENDING, HEADER_BYTES, REC_BYTES, calib_name, parse_gsics_txt, parse_granule,
};
use std::process::Command;

const CDN_TAG: &str = "noaa-goes19.s3.amazonaws.com";
const GSICS_DEFAULT_URL: &str = "https://www.star.nesdis.noaa.gov/GOESCal/images/GSICS/GSICS_Harmonization_release_May2025_current.txt";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn read_gsics_source(src: &str) -> Option<String> {
    if src.starts_with("http://") || src.starts_with("https://") {
        curl_bytes(src).and_then(|b| String::from_utf8(b).ok())
    } else {
        match std::fs::read_to_string(src) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("{src}: read void: {e}");
                None
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "goes_abi_rad.bin".to_string(),
    };
    let input = arg_value(&args, "--input");
    let url = arg_value(&args, "--url");
    let gsics_src = arg_value(&args, "--gsics");
    let bytes = match (input, url) {
        (Some(path), _) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{path}: read void: {e}");
                std::process::exit(1);
            }
        },
        (None, Some(u)) => match curl_bytes(&u) {
            Some(b) => b,
            None => {
                eprintln!("{u}: fetch void");
                std::process::exit(1);
            }
        },
        (None, None) => {
            eprintln!(
                "usage: goes_abi_compiler (--input <granule.nc> | --url <https url>) --out <goes_abi_rad.bin> [--ci-mode] [--gsics <txt url-or-file>]"
            );
            std::process::exit(1);
        }
    };
    let gsics_text = match gsics_src {
        Some(src) => read_gsics_source(&src),
        None if ci_mode => curl_bytes(GSICS_DEFAULT_URL).and_then(|b| String::from_utf8(b).ok()),
        None => None,
    };
    let gsics_table = gsics_text.as_deref().map(parse_gsics_txt);
    let granule = match parse_granule(&bytes, gsics_table.as_ref()) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("goes_abi_compiler: {e}");
            std::process::exit(1);
        }
    };
    if gsics_text.is_some() && granule.calib == CALIB_GSICS_PENDING {
        eprintln!(
            "gsics: band {} has no GOES-16 coefficient in the txt — calib stays gsics-pending",
            granule.band_id
        );
    }
    eprintln!(
        "granule: band {} wavelength {:.4} um t {:.1} (J2000 s) sub_lon {:.2} deg persp_h {:.0} m calib {}",
        granule.band_id,
        granule.band_wavelength,
        granule.t,
        granule.sub_lon,
        granule.persp_h,
        calib_name(granule.calib)
    );
    eprintln!(
        "radiance: mean {:.4} std {:.4} min {:.4} max {:.4} W m-2 sr-1 um-1, {} valid / {} pixels, esun {:.4} W m-2 um-1, kappa0 {:.6} (W m-2 um-1)-1",
        granule.rad_mean,
        granule.rad_std,
        granule.rad_min,
        granule.rad_max,
        granule.valid,
        granule.total,
        granule.esun,
        granule.kappa0
    );
    let records = vec![granule];
    let Some(bin) = goes_abi::write_bin(&records) else {
        eprintln!("goes_abi_compiler: a record refuses the GAB1 gate — the asset stays unwritten");
        std::process::exit(1);
    };
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("write {out_path} void");
        std::process::exit(1);
    }
    let Some(parsed) = goes_abi::parse_bin(&bin) else {
        eprintln!("{out_path}: roundtrip parse void — the asset stays unverified");
        std::process::exit(1);
    };
    let Some(last) = parsed.last() else {
        eprintln!("{out_path}: roundtrip parse empty — the asset stays unverified");
        std::process::exit(1);
    };
    eprintln!(
        "last granule: t {:.1} band {} wavelength {:.4} um mean {:.4} W m-2 sr-1 um-1",
        last.t, last.band_id, last.band_wavelength, last.rad_mean
    );
    eprintln!(
        "{}: {} record(s), {} B -> {}",
        out_path,
        parsed.len(),
        HEADER_BYTES + parsed.len() * REC_BYTES,
        out_path
    );
    if ci_mode && !upload_release(CDN_TAG, &out_path) {
        eprintln!("upload {}: did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_goes16_abi_granule_compiles() {
        let path = "phi/pipeline/catalog/noaa_goes16/OR_ABI-L1b-RadC-M6C01_G16_s20240010001173_e20240010003546_c20240010004005.nc";
        if !std::path::Path::new(path).exists() {
            eprintln!(
                "skipped (fixture absent): goes16 abi — fetch from noaa-goes19.s3.amazonaws.com/ABI-L1b-RadC/2024/001/00/"
            );
            return;
        }
        let bytes = std::fs::read(path).expect("fixture read");
        let g = parse_granule(&bytes, None).expect("granule parses");
        assert_eq!(g.band_id, 1);
        assert!(g.total == 15_000_000);
        assert!(g.valid > 0 && g.valid <= g.total);
        assert!(g.rad_mean.is_finite());
        assert!(g.esun > 0.0);
        assert!(g.kappa0 > 0.0);
        assert!((g.sub_lon - -75.2).abs() < 1.0);
    }
}
