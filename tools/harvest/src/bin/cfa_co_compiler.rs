use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage};
use omegaflow::volume::{Axis, AxisKind, Volume};

const NETLOC: &str = "zenodo.org";
const COMPILER: &str = "tools/harvest/src/bin/cfa_co_compiler.rs";
const TTL: u64 = 31536000;
const DEFAULT_URL: &str =
    "https://zenodo.org/api/records/5501196/files/H2_dens_mean_BEG03.fits/content";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn uniform_axis(header: &FitsHeader, n: usize) -> Option<Axis> {
    let origin = header.f64(&format!("CRVAL{n}"))?;
    let step = header.f64(&format!("CDELT{n}"))?;
    if !origin.is_finite() || !step.is_finite() || step == 0.0 {
        return None;
    }
    Some(Axis {
        kind: AxisKind::Uniform,
        values: vec![origin, step],
    })
}

fn parse_grid(bytes: &[u8]) -> Option<Volume> {
    let (image, _) = FitsImage::parse(bytes, 0)?;
    let (header, _) = FitsHeader::parse(bytes, 0)?;
    if image.dims.iter().any(|d| *d == 0) {
        return None;
    }
    let axes = [
        uniform_axis(&header, 1)?,
        uniform_axis(&header, 2)?,
        uniform_axis(&header, 3)?,
    ];
    let dims = [
        image.dims[0] as u32,
        image.dims[1] as u32,
        image.dims[2] as u32,
    ];
    let cells = dims[0] as usize * dims[1] as usize * dims[2] as usize;
    let mut data = vec![0.0f32; cells];
    let mut mask = vec![0u8; cells.div_ceil(8)];
    for k in 0..dims[2] as usize {
        for j in 0..dims[1] as usize {
            for i in 0..dims[0] as usize {
                let linear = (k * dims[1] as usize + j) * dims[0] as usize + i;
                let f = match image.value_f64(bytes, [i, j, k]) {
                    Some(v) if v.is_finite() => v as f32,
                    _ => f32::NAN,
                };
                if f.is_finite() {
                    data[linear] = f;
                } else {
                    data[linear] = f32::NAN;
                    mask[linear / 8] |= 1 << (linear % 8);
                }
            }
        }
    }
    Some(Volume {
        dims,
        axes,
        data,
        mask,
        frame_body: None,
    })
}

fn asset_name(spec: &str) -> String {
    let base = match spec.split('?').next() {
        Some(b) => b,
        None => spec,
    };
    let segs: Vec<&str> = base.rsplit('/').filter(|s| !s.is_empty()).collect();
    let file = match segs.first() {
        Some(&"content") => segs.get(1).copied(),
        other => other.copied(),
    };
    let name = match file {
        Some(n) => n,
        None => "cfa_co",
    };
    let stem = match name.split('.').next() {
        Some(s) => s,
        None => name,
    };
    format!("{}.volume.bin", stem.to_ascii_lowercase())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let source = match arg_value(&args, "--url")
        .or_else(|| arg_value(&args, "--input"))
        .or_else(|| arg_value(&args, "--file"))
    {
        Some(s) => s,
        None => DEFAULT_URL.to_string(),
    };
    let bytes = if source.starts_with("http://") || source.starts_with("https://") {
        match fetch_raw_bytes(&source) {
            Some(b) => b,
            None => {
                eprintln!("cfa_co_compiler: fetch void ({source})");
                std::process::exit(1);
            }
        }
    } else {
        match std::fs::read(&source) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("cfa_co_compiler: read {source}: {e} — the grid stays unread");
                std::process::exit(1);
            }
        }
    };
    let volume = match parse_grid(&bytes) {
        Some(v) => v,
        None => {
            eprintln!(
                "cfa_co_compiler: {source}: no 3D f64/f32 IMAGE HDU with CRVAL/CDELT on axes 1..3 — the bin stays unwritten (0 honored)"
            );
            std::process::exit(1);
        }
    };
    let bin = volume.write_bin();
    let Some(back) = Volume::read_bin(&bin) else {
        eprintln!("cfa_co_compiler: roundtrip read void — the bin stays unverified");
        std::process::exit(1);
    };
    if back.dims != volume.dims || back.data.len() != volume.data.len() {
        eprintln!("cfa_co_compiler: roundtrip dims differ — the bin stays unverified");
        std::process::exit(1);
    }
    let asset = asset_name(&source);
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => asset.clone(),
    };
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("cfa_co_compiler: write {out} void");
        std::process::exit(1);
    }
    let digest = sha256_hex(&bin);
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    let mut absent = 0usize;
    for (i, v) in volume.data.iter().enumerate() {
        if volume.mask[i / 8] & (1 << (i % 8)) != 0 {
            absent += 1;
        } else {
            lo = lo.min(*v as f64);
            hi = hi.max(*v as f64);
        }
    }
    eprintln!(
        "cfa_co: {}x{}x{} grid, {} cell(s), {} absent, value [{lo}, {hi}], {} B -> {out}",
        volume.dims[0],
        volume.dims[1],
        volume.dims[2],
        volume.data.len(),
        absent,
        bin.len()
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format volume");
    println!("origin {source}");
    println!("compiler {COMPILER}");
    println!("sha256 {digest}");
    println!("ttl {TTL}");
    match arg_value(&args, "--at") {
        Some(body) => println!("at {body}"),
        None => eprintln!(
            "cfa_co: `at <body>` absent — the anchor and the force/field mapping are the line owner's decision; no at/field line printed (0 honored)"
        ),
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("cfa_co_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(kw: &str, value: &str) -> [u8; 80] {
        let mut c = [b' '; 80];
        let k = kw.as_bytes();
        c[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        c[8] = b'=';
        let v = value.as_bytes();
        c[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        c
    }

    fn cube_fixture(naxis3: bool) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        let mut cards: Vec<(&str, &str)> = vec![
            ("SIMPLE", "T"),
            ("BITPIX", "-64"),
            ("NAXIS", if naxis3 { "3" } else { "2" }),
            ("NAXIS1", "2"),
            ("NAXIS2", "2"),
        ];
        if naxis3 {
            cards.push(("NAXIS3", "2"));
        }
        cards.extend_from_slice(&[
            ("CRVAL1", "0.0"),
            ("CDELT1", "1.0"),
            ("CRVAL2", "10.0"),
            ("CDELT2", "0.5"),
        ]);
        if naxis3 {
            cards.extend_from_slice(&[("CRVAL3", "-1.0"), ("CDELT3", "0.25")]);
        }
        cards.push(("BUNIT", "'molecules cm^-3'"));
        cards.push(("END", ""));
        for (k, v) in cards {
            header.extend_from_slice(&card(k, v));
        }
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        let mut buf = header;
        let data: [f64; 8] = [1.0, 2.0, 3.0, f64::NAN, 5.0, 6.0, 7.0, 8.0];
        for v in data {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn grid_axes_and_values_survive() {
        let volume = parse_grid(&cube_fixture(true)).expect("2x2x2 f64 cube parses");
        assert_eq!(volume.dims, [2, 2, 2]);
        assert_eq!(volume.axes[0].values, vec![0.0, 1.0]);
        assert_eq!(volume.axes[1].values, vec![10.0, 0.5]);
        assert_eq!(volume.axes[2].values, vec![-1.0, 0.25]);
        assert_eq!(volume.data[0], 1.0);
        assert!(volume.data[3].is_nan());
        assert_ne!(
            volume.mask[0] & (1 << 3),
            0,
            "the NaN cell carries its mask bit"
        );
        assert_eq!(
            volume.mask[0] & (1 << 0),
            0,
            "a finite cell carries no mask bit"
        );
    }

    #[test]
    fn a_2d_map_has_no_third_axis() {
        assert!(parse_grid(&cube_fixture(false)).is_none());
    }

    #[test]
    fn bin_roundtrips_through_the_volume_reader() {
        let volume = parse_grid(&cube_fixture(true)).expect("cube parses");
        let bin = volume.write_bin();
        let back = Volume::read_bin(&bin).expect("the emitted bin reads back");
        assert_eq!(back.dims, volume.dims);
        assert_eq!(back.data.len(), volume.data.len());
    }

    #[test]
    fn asset_name_reads_the_bound_name() {
        assert_eq!(
            asset_name(
                "https://zenodo.org/api/records/5501196/files/H2_dens_mean_BEG03.fits/content"
            ),
            "h2_dens_mean_beg03.volume.bin"
        );
        assert_eq!(
            asset_name("local/H2_dens_stddev_SBM15.fits"),
            "h2_dens_stddev_sbm15.volume.bin"
        );
    }
}
