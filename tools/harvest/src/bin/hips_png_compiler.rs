use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::hips::{
    HipsBand, HipsTile, band_sums, parse_asset, parse_bin, parse_png, parse_properties,
    tile_from_url, write_bin,
};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "alasky.cds.unistra.fr";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn band_stats(values: &[Option<f64>], total: u32) -> Option<HipsBand> {
    let mut sum = 0.0f64;
    let mut count = 0usize;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for v in values {
        let Some(&v) = v.as_ref() else { continue };
        sum += v;
        count += 1;
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    }
    if count == 0 {
        return None;
    }
    let mean = sum / count as f64;
    let mut sq = 0.0f64;
    for v in values.iter().flatten() {
        let d = v - mean;
        sq += d * d;
    }
    Some(HipsBand {
        mean: mean as f32,
        std: (sq / count as f64).sqrt() as f32,
        min: min as f32,
        max: max as f32,
        valid: count as u32,
        total,
    })
}

fn run(args: &[String]) -> Result<(), String> {
    let properties_url = match arg_value(args, "--properties") {
        Some(p) => p,
        None => {
            return Err("--properties <url>: the HiPS manifest is never silent — refused".into());
        }
    };
    let tile_url = match arg_value(args, "--tile") {
        Some(p) => p,
        None => return Err("--tile <url>: the tile is never silent — refused".into()),
    };
    let out_path = match arg_value(args, "--out") {
        Some(p) => p,
        None => match tile_from_url(&tile_url) {
            Some((order, npix)) => {
                format!("data/{NETLOC}/hips_png_{order}_{npix}.bin")
            }
            None => "data/alasky.cds.unistra.fr/hips_png.bin".to_string(),
        },
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let props_bytes = fetch_raw_bytes(&properties_url)
        .ok_or_else(|| format!("{properties_url}: fetch returned void"))?;
    let props_text =
        String::from_utf8(props_bytes).map_err(|_| "properties carry no utf8 text".to_string())?;
    let props = parse_properties(&props_text).ok_or("properties carry no HiPS manifest")?;
    let tile_format = match props.hips_tile_format.as_deref() {
        Some(f) => f,
        None => return Err("properties carry no hips_tile_format".into()),
    };
    if tile_format != "png" {
        return Err(format!(
            "hips_tile_format {tile_format} is not png — the png arm refuses"
        ));
    }
    let tile_width = match props.hips_tile_width {
        Some(w) => w,
        None => return Err("properties carry no hips_tile_width".into()),
    };
    let pixel_scale = match props.hips_pixel_scale {
        Some(s) if s.is_finite() && s > 0.0 => s,
        _ => return Err("properties carry no positive hips_pixel_scale".into()),
    };
    let (order, npix) = match tile_from_url(&tile_url) {
        Some(t) => t,
        None => return Err("the tile URL carries no Norder/Npix path — refused".into()),
    };

    let bytes =
        fetch_raw_bytes(&tile_url).ok_or_else(|| format!("{tile_url}: fetch returned void"))?;
    let raster = match parse_png(&bytes) {
        Some(r) => r,
        None => {
            return Err(
                "the tile body does not read as an 8-bit non-interlaced PNG the reader parses"
                    .into(),
            );
        }
    };
    eprintln!(
        "tile Norder{order}/Npix{npix}: {}×{}, color type {}, {} bands",
        raster.samples, raster.lines, raster.color_type, raster.bands
    );
    if raster.samples != tile_width as usize || raster.lines != tile_width as usize {
        return Err(format!(
            "tile dims {}×{} differ from hips_tile_width {tile_width} — the asset stays unwritten",
            raster.samples, raster.lines
        ));
    }
    let total = (tile_width as u64) * (tile_width as u64);
    let mut bands = Vec::with_capacity(raster.bands);
    for b in 0..raster.bands {
        let start = b * raster.lines * raster.samples;
        let band = band_stats(
            &raster.values[start..start + raster.lines * raster.samples],
            total as u32,
        )
        .ok_or("no pixel yielded a channel value — the asset stays unwritten (0 honored)")?;
        eprintln!(
            "{}: mean {:.3} std {:.3} min {:.1} max {:.1} valid {} total {}",
            raster.band_names[b], band.mean, band.std, band.min, band.max, band.valid, band.total
        );
        bands.push(band);
    }
    let tile = HipsTile {
        order,
        npix,
        color_type: raster.color_type,
        lines: raster.lines as u32,
        samples: raster.samples as u32,
        pixel_scale,
        bands,
    };
    let asset =
        write_bin(&tile).ok_or("the record refuses its own values — the asset stays unwritten")?;
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out_path, &asset)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let parsed = parse_bin(&asset).ok_or(format!(
        "{out_path}: roundtrip parse void — the asset stays unverified"
    ))?;
    if parsed != tile {
        return Err(format!(
            "{out_path}: roundtrip differs from the written tile — the asset stays unverified"
        ));
    }
    eprintln!(
        "{out_path}: {} B, roundtrip parses; per-band sums {:?}",
        asset.len(),
        band_sums(&raster)
    );
    if let Some(series) = parse_asset(&asset) {
        eprintln!("series: {series:?}");
    }

    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("hips_png_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_stats_reads_mean_std_min_max() {
        let values = vec![Some(10.0), Some(20.0), None, Some(30.0)];
        let band = band_stats(&values, 4).expect("stats");
        assert_eq!(band.valid, 3);
        assert_eq!(band.total, 4);
        assert_eq!(band.mean, 20.0);
        assert_eq!(band.min, 10.0);
        assert_eq!(band.max, 30.0);
        assert!((band.std - ((200.0f64 / 3.0).sqrt() as f32)).abs() < 1e-6);
        assert!(band_stats(&vec![None; 4], 4).is_none());
    }
}
