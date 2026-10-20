use std::io::Write;
use std::process::Command;

use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::fits::FitsHeader;
use omegaflow::archivar::hips::parse_properties;
use omegaflow::archivar::hips3d::{
    Hips3DManifest, axis2_npix_bound, parse_bin, parse_sfmoc_tiles, spatial_npix_bound, tile_url,
    write_bin,
};
use omegaflow::archivar::sha256::sha256_hex;

const DEFAULT_BASE: &str =
    "https://alasky.cds.unistra.fr/HIPS3D/ASKAP/CDS_C_ASKAP_WALLABY-Pilot-DR2";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_text(url: &str) -> Result<String, String> {
    let bytes = fetch_raw_bytes(url).ok_or_else(|| format!("{url}: fetch returned void"))?;
    String::from_utf8(bytes).map_err(|_| format!("{url}: body carries no utf8 text"))
}

fn fetch_range(url: &str, first: u64, last: u64) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg("2")
        .arg("--connect-timeout")
        .arg("32")
        .arg("-m")
        .arg("120")
        .arg("-r")
        .arg(format!("{first}-{last}"))
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() || out.stdout.is_empty() {
        eprintln!(
            "range {url}: status {} stderr {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    Some(out.stdout)
}

fn probe_tile(url: &str) -> Option<String> {
    let bytes = fetch_range(url, 0, 11_519)?;
    let Some((header, _)) = FitsHeader::parse(&bytes, 0) else {
        eprintln!(
            "range {url}: {} B do not parse as a FITS header: {:?}",
            bytes.len(),
            String::from_utf8_lossy(&bytes[..bytes.len().min(80)])
        );
        return None;
    };
    let naxis1 = header.int("NAXIS1");
    let naxis2 = header.int("NAXIS2");
    let naxis3 = header.int("NAXIS3");
    let crval3 = header.f64("CRVAL3");
    let cdelt3 = header.f64("CDELT3");
    let bunit = header.str_unescaped("BUNIT");
    Some(format!(
        "{}x{}x{} CRVAL3={} CDELT3={} BUNIT={}",
        naxis1.map_or("-".to_string(), |v| v.to_string()),
        naxis2.map_or("-".to_string(), |v| v.to_string()),
        naxis3.map_or("-".to_string(), |v| v.to_string()),
        crval3.map_or("-".to_string(), |v| format!("{v:.6e}")),
        cdelt3.map_or("-".to_string(), |v| format!("{v:.6e}")),
        match bunit.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => "-",
        }
    ))
}

fn parse_limit(args: &[String], name: &str) -> Result<Option<usize>, String> {
    match arg_value(args, name) {
        Some(v) => v
            .parse::<usize>()
            .map(Some)
            .map_err(|_| format!("{name} carries no usize")),
        None => Ok(None),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let properties_url = arg_value(args, "--properties")
        .ok_or("--properties <url>: the HiPS3D manifest is never silent — refused")?;
    let out_path = arg_value(args, "--out")
        .ok_or("--out <manifest path>: the manifest path is never silent — refused")?;
    let ext = match arg_value(args, "--ext") {
        Some(v) => v,
        None => "fits".to_string(),
    };
    if ext != "fits" && ext != "png" {
        return Err(format!("--ext {ext} is neither fits nor png — refused"));
    }
    let text = fetch_text(&properties_url)?;
    let props = parse_properties(&text)
        .ok_or_else(|| format!("{properties_url}: properties carry no HiPS manifest"))?;
    if props.dataproduct_type.as_deref() != Some("spectral-cube") {
        return Err(format!(
            "dataproduct_type {:?} is not spectral-cube — the HiPS3D arm refuses",
            props.dataproduct_type
        ));
    }
    let order = props
        .hips_order
        .ok_or("properties carry no hips_order — the spatial depth is never guessed")?;
    let order_axis2 = props
        .hips_order_axis2
        .or(props.hips_order_freq)
        .ok_or("properties carry neither hips_order_axis2 nor hips_order_freq")?;
    if spatial_npix_bound(order).is_none() || axis2_npix_bound(order_axis2).is_none() {
        return Err(format!(
            "Norder{order}_{order_axis2} leaves the addressable tree — refused"
        ));
    }
    let tile_width = props
        .hips_tile_width
        .ok_or("properties carry no hips_tile_width")?;
    let tile_depth = props
        .hips_tile_depth
        .ok_or("properties carry no hips_tile_depth")?;
    let (em_min, em_max) = match (props.em_min, props.em_max) {
        (Some(a), Some(b)) if a.is_finite() && b.is_finite() && a <= b => (a, b),
        _ => return Err("properties carry no ordered em_min/em_max pair".into()),
    };
    let base = match arg_value(args, "--base") {
        Some(b) => b,
        None => match properties_url.strip_suffix("/properties") {
            Some(b) => b.to_string(),
            None => DEFAULT_BASE.to_string(),
        },
    };
    let moc_url = format!("{}/Moc.fits", base.trim_end_matches('/'));
    let moc = fetch_raw_bytes(&moc_url).ok_or_else(|| format!("{moc_url}: fetch returned void"))?;
    let mut tiles = parse_sfmoc_tiles(&moc, order_axis2, order).ok_or_else(|| {
        format!("{moc_url}: SFMOC does not degrade to Norder{order}_{order_axis2}")
    })?;
    if let Some(limit) = parse_limit(args, "--limit")? {
        tiles.truncate(limit);
    }
    let suffix = if args.iter().any(|a| a == "--limit") {
        " (limited)"
    } else {
        ""
    };
    eprintln!(
        "{base}: Norder{order}_{order_axis2}, {tile_width}x{tile_width}x{tile_depth}, {} covered tiles{suffix}",
        tiles.len()
    );
    if let Some(probe) = parse_limit(args, "--probe")? {
        for tile in tiles.iter().take(probe) {
            let url = tile_url(
                &base,
                tile.order,
                tile.npix,
                tile.order_axis2,
                tile.npix_axis2,
                &ext,
            )
            .ok_or("a manifest address renders no URL — refused")?;
            match probe_tile(&url) {
                Some(summary) => println!("probe {url}: {summary}"),
                None => eprintln!("probe {url}: header void"),
            }
        }
    }
    let manifest = Hips3DManifest {
        order,
        order_axis2,
        tile_width,
        tile_depth,
        em_min,
        em_max,
        base: base.clone(),
        tiles,
    };
    let bytes = write_bin(&manifest)
        .ok_or("the manifest refuses its own values — the asset stays unwritten")?;
    if let Some(parent) = std::path::Path::new(&out_path).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
    }
    let mut file = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    file.write_all(&bytes)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let parsed = parse_bin(&bytes).ok_or("manifest roundtrip parse void — asset unverified")?;
    if parsed != manifest {
        return Err("manifest roundtrip differs from the written tree — asset unverified".into());
    }
    println!(
        "{out_path}: {} B, {} tiles, sha256 {}",
        bytes.len(),
        parsed.tiles.len(),
        sha256_hex(&bytes)
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("hips3d_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::hips3d::{Hips3DTile, axis2_dir_of, dir_of};

    #[test]
    fn addresses_use_the_hips3d_tree() {
        assert_eq!(dir_of(99_701), 90_000);
        assert_eq!(axis2_dir_of(253_834), 253_830);
        let url = tile_url("https://example.org/tree", 7, 99_701, 18, 253_834, "fits").unwrap();
        assert_eq!(
            url,
            "https://example.org/tree/Norder7_18/Dir90000_253830/Npix99701_253834.fits"
        );
        assert!(tile_url("https://example.org/tree", 7, 196_608, 18, 0, "fits").is_none());
    }

    #[test]
    fn manifest_header_and_records_stay_consistent() {
        let manifest = Hips3DManifest {
            order: 7,
            order_axis2: 18,
            tile_width: 256,
            tile_depth: 16,
            em_min: 0.208,
            em_max: 0.231,
            base: "https://example.org/tree".into(),
            tiles: vec![Hips3DTile {
                order: 7,
                npix: 99_701,
                dir: 90_000,
                order_axis2: 18,
                npix_axis2: 253_834,
            }],
        };
        let bytes = write_bin(&manifest).unwrap();
        assert_eq!(parse_bin(&bytes).unwrap(), manifest);
    }
}
