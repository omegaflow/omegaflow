use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::hips::{
    DirWalk, HipsBand, HipsProperties, HipsTile, HipsTileEntry, TileWalk, band_sums, dir_of,
    format_manifest_entry, manifest_base, parse_asset, parse_bin, parse_manifest_entry, parse_png,
    parse_properties, tile_from_url, tile_url, write_bin,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "alasky.cds.unistra.fr";
const FLUSH_EVERY: usize = 256;

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

fn fetch_properties(url: &str) -> Result<HipsProperties, String> {
    let props_bytes = fetch_raw_bytes(url).ok_or_else(|| format!("{url}: fetch returned void"))?;
    let props_text =
        String::from_utf8(props_bytes).map_err(|_| "properties carry no utf8 text".to_string())?;
    parse_properties(&props_text).ok_or_else(|| "properties carry no HiPS manifest".to_string())
}

fn fetch_tile(url: &str, tmp_path: &str) -> Option<(u32, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg("2")
        .arg("--retry-connrefused")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg("300")
        .arg("--connect-timeout")
        .arg("32")
        .arg("-o")
        .arg(tmp_path)
        .arg("-w")
        .arg("%{http_code}")
        .arg(url);
    let out = cmd.output().ok()?;
    if !out.status.success() {
        return None;
    }
    let code: u32 = String::from_utf8_lossy(&out.stdout).trim().parse().ok()?;
    let bytes = std::fs::read(tmp_path).ok()?;
    Some((code, bytes))
}

fn optional_u64(args: &[String], name: &str) -> Result<Option<u64>, String> {
    match arg_value(args, name) {
        Some(v) => v
            .parse::<u64>()
            .map(Some)
            .map_err(|_| format!("{name} carries no u64")),
        None => Ok(None),
    }
}

fn parse_orders(range: &str) -> Result<(u32, u32), String> {
    let (lo, hi) = range
        .split_once(':')
        .ok_or_else(|| format!("--orders {range} carries no lo:hi pair"))?;
    let lo: u32 = lo
        .parse()
        .map_err(|_| format!("--orders {range}: lo carries no u32"))?;
    let hi: u32 = hi
        .parse()
        .map_err(|_| format!("--orders {range}: hi carries no u32"))?;
    if lo > hi {
        return Err(format!("--orders {range}: lo above hi — refused"));
    }
    Ok((lo, hi))
}

fn tree_props(args: &[String]) -> Result<(HipsProperties, String, String), String> {
    let properties_url = match arg_value(args, "--properties") {
        Some(p) => p,
        None => {
            return Err("--properties <url>: the HiPS manifest is never silent — refused".into());
        }
    };
    let props = fetch_properties(&properties_url)?;
    match props.hips_tile_format.as_deref() {
        Some("png") => {}
        Some(f) => {
            return Err(format!(
                "hips_tile_format {f} is not png — the png arm refuses"
            ));
        }
        None => return Err("properties carry no hips_tile_format".into()),
    }
    let base = properties_url
        .strip_suffix("/properties")
        .ok_or("the properties URL carries no /properties suffix — the tree base stays unnamed")?
        .to_string();
    let tree = base
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .ok_or("the properties URL names no tree segment")?
        .to_string();
    Ok((props, base, tree))
}

fn tree_count(base: &str, lo: u32, hi: u32) -> Result<(), String> {
    let mut total: u64 = 0;
    for order in lo..=hi {
        let Some(tiles) = TileWalk::full(order) else {
            return Err(format!("Norder {order}: tile bound leaves u32 — refused"));
        };
        let Some(dirs) = DirWalk::new(order) else {
            return Err(format!("Norder {order}: dir walk void — refused"));
        };
        let tiles = tiles.len() as u64;
        println!(
            "{base}: Norder{order}: {tiles} tiles in {} Dir shards",
            dirs.len()
        );
        total += tiles;
    }
    println!("{base}: {total} tiles over Norder {lo}..{hi}");
    Ok(())
}

fn tree_list(
    base: &str,
    lo: u32,
    hi: u32,
    dir_filter: Option<u64>,
    limit: u64,
) -> Result<(), String> {
    let mut printed: u64 = 0;
    for order in lo..=hi {
        let Some(walk) = TileWalk::full(order) else {
            return Err(format!("Norder {order}: tile bound leaves u32 — refused"));
        };
        for npix in walk {
            if let Some(d) = dir_filter {
                if dir_of(npix as u64) != d {
                    continue;
                }
            }
            let Some(url) = tile_url(base, order, npix) else {
                return Err(format!("Norder{order}/Npix{npix}: url void — refused"));
            };
            println!("{url}");
            printed += 1;
            if printed >= limit {
                return Ok(());
            }
        }
    }
    Ok(())
}

fn read_manifest(out_path: &str, base: &str) -> Result<BTreeSet<u32>, String> {
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut content =
        std::fs::read(out_path).map_err(|e| format!("read {out_path} returned void: {e}"))?;
    if content.is_empty() {
        writeln!(file, "base {base}")
            .map_err(|e| format!("write {out_path} returned void: {e}"))?;
        return Ok(BTreeSet::new());
    }
    if let Some(&last) = content.last() {
        if last != b'\n' {
            let cut = content
                .iter()
                .rposition(|&b| b == b'\n')
                .map_or(0, |p| p + 1);
            content.truncate(cut);
            std::fs::write(out_path, &content)
                .map_err(|e| format!("write {out_path} returned void: {e}"))?;
            if content.is_empty() {
                writeln!(file, "base {base}")
                    .map_err(|e| format!("write {out_path} returned void: {e}"))?;
                return Ok(BTreeSet::new());
            }
        }
    }
    let text = String::from_utf8(content)
        .map_err(|_| format!("{out_path} carries no utf8 text — refuse to resume"))?;
    let mut done = BTreeSet::new();
    let mut header_present = false;
    for (idx, line) in text.lines().enumerate() {
        if let Some(b) = manifest_base(line) {
            if idx != 0 {
                return Err(format!(
                    "{out_path}:{idx}: base header off the first line — refuse to resume"
                ));
            }
            if b != base {
                return Err(format!(
                    "{out_path}: manifest base {b} differs from {base} — refuse to resume (riss)"
                ));
            }
            header_present = true;
            continue;
        }
        if let Some((npix, _)) = parse_manifest_entry(line) {
            done.insert(npix);
        } else if !line.is_empty() {
            return Err(format!(
                "{out_path}:{idx}: line does not read as a manifest entry — refuse to resume"
            ));
        }
    }
    if !header_present {
        return Err(format!(
            "{out_path}: manifest carries no base header — refuse to resume"
        ));
    }
    Ok(done)
}

fn flush_buffer(file: &mut std::fs::File, buffer: &mut Vec<String>) -> Result<(), String> {
    for line in buffer.drain(..) {
        writeln!(file, "{line}").map_err(|e| format!("append returned void: {e}"))?;
    }
    Ok(())
}

fn harvest_tiles(
    base: &str,
    order: u32,
    tile_width: u32,
    walk: TileWalk,
    limit: u64,
    out_path: &str,
    ci_mode: bool,
    tree: &str,
) -> Result<(), String> {
    let shard_tiles = walk.len();
    let done = read_manifest(out_path, base)?;
    let tmp_path = format!("{out_path}.tmp");
    let mut recorded: u64 = 0;
    let mut ok_count: u64 = 0;
    let mut absent_count: u64 = 0;
    let mut pending_count: u64 = 0;
    let mut buffer: Vec<String> = Vec::new();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;

    for npix in walk {
        if done.contains(&npix) {
            continue;
        }
        if recorded >= limit {
            eprintln!("{out_path}: limit {limit} reached — the next run resumes");
            break;
        }
        let Some(url) = tile_url(base, order, npix) else {
            return Err(format!("Norder{order}/Npix{npix}: url void — refused"));
        };
        match fetch_tile(&url, &tmp_path) {
            Some((200, bytes)) => match parse_png(&bytes) {
                Some(raster) => {
                    if raster.samples != tile_width as usize || raster.lines != tile_width as usize
                    {
                        eprintln!("Npix{npix}: dims differ from hips_tile_width — pending");
                        pending_count += 1;
                        continue;
                    }
                    buffer.push(format_manifest_entry(
                        npix,
                        &HipsTileEntry::Present {
                            sha256: sha256_hex(&bytes),
                            bytes: bytes.len() as u64,
                            color_type: raster.color_type,
                            band_sums: band_sums(&raster),
                        },
                    ));
                    ok_count += 1;
                }
                None => {
                    eprintln!("Npix{npix}: parse void — pending");
                    pending_count += 1;
                }
            },
            Some((404, _)) => {
                buffer.push(format_manifest_entry(npix, &HipsTileEntry::Absent));
                absent_count += 1;
            }
            Some((code, _)) => {
                eprintln!("Npix{npix}: HTTP {code} — pending");
                pending_count += 1;
            }
            None => {
                eprintln!("Npix{npix}: fetch void — pending");
                pending_count += 1;
            }
        }
        recorded = ok_count + absent_count;
        if buffer.len() >= FLUSH_EVERY {
            flush_buffer(&mut file, &mut buffer)?;
        }
    }
    flush_buffer(&mut file, &mut buffer)?;
    let _ = std::fs::remove_file(&tmp_path);

    if recorded == 0 {
        eprintln!(
            "{out_path}: {}/{shard_tiles} tiles already recorded — nothing new",
            done.len()
        );
        return Ok(());
    }
    eprintln!(
        "{out_path}: recorded {ok_count} present, {absent_count} absent, {pending_count} pending; shard {}/{shard_tiles} decided",
        done.len() + recorded as usize
    );
    if ci_mode {
        let tag = format!("{NETLOC}-{tree}");
        if !upload_release(&tag, out_path) {
            return Err(format!("{out_path}: CDN upload returned void"));
        }
    }
    Ok(())
}

fn tree_run(args: &[String]) -> Result<(), String> {
    let (props, base, tree) = tree_props(args)?;
    let max_order = props
        .hips_order
        .ok_or("properties carry no hips_order — the tree refuses to guess its depth")?;
    let min_order = props.hips_order_min;

    if let Some(range) = arg_value(args, "--orders") {
        let (lo, hi) = parse_orders(&range)?;
        if hi > max_order || min_order.is_some_and(|m| lo < m) {
            return Err(format!(
                "--orders {range} leaves the published range ..{max_order} — refused"
            ));
        }
        if args.iter().any(|a| a == "--count") {
            return tree_count(&base, lo, hi);
        }
        if args.iter().any(|a| a == "--list") {
            let limit = optional_u64(args, "--limit")?.unwrap_or(u64::MAX);
            let dir_filter = optional_u64(args, "--dir")?;
            return tree_list(&base, lo, hi, dir_filter, limit);
        }
        return Err("tree index mode needs --count or --list".into());
    }

    let order: u32 = arg_value(args, "--order")
        .ok_or("--order <n>: the Norder is never silent — refused")?
        .parse()
        .map_err(|_| "--order carries no u32".to_string())?;
    if order > max_order || min_order.is_some_and(|m| order < m) {
        return Err(format!(
            "--order {order} leaves the published range ..{max_order} — refused"
        ));
    }
    let tile_width = match props.hips_tile_width {
        Some(w) => w,
        None => return Err("properties carry no hips_tile_width".into()),
    };
    let dir_filter = optional_u64(args, "--dir")?;
    let limit = optional_u64(args, "--limit")?.unwrap_or(u64::MAX);
    let out_path = arg_value(args, "--out")
        .ok_or("--out <manifest path>: the manifest path is never silent — refused")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let walk = match dir_filter {
        Some(d) => TileWalk::dir(order, d)
            .ok_or_else(|| format!("--dir {d} is no Dir shard of Norder {order} — refused"))?,
        None => TileWalk::full(order).ok_or("the Norder walks no u32 tiling — refused")?,
    };

    harvest_tiles(
        &base, order, tile_width, walk, limit, &out_path, ci_mode, &tree,
    )
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
    let result = if args.iter().any(|a| a == "--tree") {
        tree_run(&args)
    } else {
        run(&args)
    };
    if let Err(msg) = result {
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

    #[test]
    fn orders_parse_lo_hi() {
        assert_eq!(parse_orders("0:7").unwrap(), (0, 7));
        assert_eq!(parse_orders("7:7").unwrap(), (7, 7));
        assert!(parse_orders("7:0").is_err());
        assert!(parse_orders("7").is_err());
        assert!(parse_orders("a:7").is_err());
        assert!(parse_orders("0:b").is_err());
    }

    #[test]
    fn tree_walk_enumerates_every_tile_url() {
        let base = "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC";
        let walk = TileWalk::full(7).expect("the walk stands");
        assert_eq!(walk.len(), 196_608);
        let first = tile_url(base, 7, 0).expect("first url");
        assert_eq!(
            first,
            "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/Norder7/Dir0/Npix0.png"
        );
        let last = tile_url(base, 7, 196_607).expect("last url");
        assert_eq!(
            last,
            "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/Norder7/Dir190000/Npix196607.png"
        );
    }
}
