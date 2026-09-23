use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GbcoRec, MAGIC_GMR, parse_gmr, write_gmr};
use omegaflow::archivar::tiff::parse_tiff;
use omegaflow::cdn::upload_release;
use omegaflow::zeuge::{FeldIdentitaet, ZeugeArt, magic_identity};

const NETLOC: &str = "www.gmrt.org";
const GRIDSERVER: &str = "https://www.gmrt.org/services/GridServer";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn witness_gestalt_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt)) => {
            eprintln!(
                "{} reads as a gestalt witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not gestalt — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn grid_url(minlon: f64, maxlon: f64, minlat: f64, maxlat: f64) -> String {
    format!(
        "{GRIDSERVER}?minlongitude={minlon}&maxlongitude={maxlon}&minlatitude={minlat}&maxlatitude={maxlat}&format=geotiff"
    )
}

fn collect_elevation(bytes: &[u8]) -> Result<Vec<GbcoRec>, String> {
    let Some(img) = parse_tiff(bytes) else {
        return Err(
            "the GridServer body does not read as a GeoTIFF the reader parses — the asset stays unwritten"
                .to_string(),
        );
    };
    eprintln!(
        "image {}×{}, samples/pixel {}, bits {:?}, compression {}, photometric {:?}",
        img.width,
        img.height,
        img.samples_per_pixel,
        img.bits_per_sample,
        img.compression,
        img.photometric
    );
    if img.pixels.is_empty() {
        return Err(
            "no decoded pixels — the strip decode returned void, the asset stays unwritten (0 honored)"
                .to_string(),
        );
    }
    let Some(&first_bits) = img.bits_per_sample.first() else {
        return Err("the raster declares no bits per sample".to_string());
    };
    if first_bits != 32 {
        return Err(format!(
            "the first band is {first_bits} bits per sample — the GMRT elevation path reads float32 bands only"
        ));
    }
    if img.samples_per_pixel == 0 {
        return Err("samples per pixel is zero".to_string());
    }
    let geo = match &img.geo {
        Some(g) => g,
        None => {
            return Err(
                "the GeoTIFF carries no georeferencing — lat/lon absent, the asset stays unwritten"
                    .to_string(),
            );
        }
    };
    let mut bytes_per_pixel = 0usize;
    for &b in &img.bits_per_sample {
        if b % 8 != 0 {
            return Err(format!("band width {b} bits is not byte-aligned"));
        }
        bytes_per_pixel += (b / 8) as usize;
    }
    if bytes_per_pixel == 0 {
        return Err("the pixel carries no bytes".to_string());
    }
    let width = img.width as usize;
    let height = img.height as usize;
    let Some(expected) = width
        .checked_mul(bytes_per_pixel)
        .and_then(|row| row.checked_mul(height))
    else {
        return Err("width × bytes-per-pixel × height overflowed".to_string());
    };
    if img.pixels.len() < expected {
        return Err(
            "the pixel buffer is shorter than width × bytes-per-pixel × height".to_string(),
        );
    }
    let mut recs = Vec::new();
    for row in 0..height {
        for col in 0..width {
            let lon = geo.x0 + (col as f64 + 0.5) * geo.dx;
            let lat = geo.y0 + (row as f64 + 0.5) * geo.dy;
            if !lat.is_finite()
                || !lon.is_finite()
                || lat < -90.0
                || lat > 90.0
                || lon < -180.0
                || lon > 180.0
            {
                continue;
            }
            let off = (row * width + col) * bytes_per_pixel;
            let Some(v) = img.pixels.get(off..off + 4) else {
                continue;
            };
            let Ok(b) = <[u8; 4]>::try_from(v) else {
                continue;
            };
            let elev = f32::from_le_bytes(b);
            if !elev.is_finite() {
                continue;
            }
            recs.push(GbcoRec {
                lat,
                lon,
                elev: elev as f64,
            });
        }
    }
    if recs.is_empty() {
        return Err(
            "no pixel yielded a finite lat/lon/elevation — the asset stays unwritten (0 honored)"
                .to_string(),
        );
    }
    Ok(recs)
}

fn report_value_ranges(recs: &[GbcoRec]) {
    let min = recs.iter().map(|r| r.elev).fold(f64::INFINITY, f64::min);
    let max = recs
        .iter()
        .map(|r| r.elev)
        .fold(f64::NEG_INFINITY, f64::max);
    let mean = recs.iter().map(|r| r.elev).sum::<f64>() / recs.len() as f64;
    eprintln!("elevation: min {min:.3} max {max:.3} mean {mean:.3} m");
}

fn write_asset(recs: &[GbcoRec], out_path: &str) -> Result<usize, String> {
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    let bytes = write_gmr(recs);
    std::fs::write(out_path, &bytes).map_err(|e| format!("write {out_path} returned void: {e}"))?;
    match parse_gmr(&bytes) {
        Some(parsed) if parsed.len() == recs.len() => Ok(parsed.len()),
        Some(parsed) => Err(format!(
            "{out_path}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            recs.len()
        )),
        None => Err(format!(
            "{out_path}: roundtrip parse void — the asset stays unverified"
        )),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    witness_gestalt_identity(MAGIC_GMR)?;
    let parse_deg = |name: &str| -> Result<f64, String> {
        let Some(v) = arg_value(args, name) else {
            return Err(format!(
                "{name} <deg>: the bbox corner is never silent — refused"
            ));
        };
        v.parse::<f64>()
            .map_err(|_| format!("{name} {v} does not parse as degrees"))
    };
    let minlon = parse_deg("--minlongitude")?;
    let maxlon = parse_deg("--maxlongitude")?;
    let minlat = parse_deg("--minlatitude")?;
    let maxlat = parse_deg("--maxlatitude")?;
    if !minlon.is_finite() || !maxlon.is_finite() || !minlat.is_finite() || !maxlat.is_finite() {
        return Err("a bbox corner is not finite — refused".to_string());
    }
    if minlon >= maxlon || minlat >= maxlat {
        return Err(format!(
            "bbox {minlon},{maxlon} × {minlat},{maxlat} carries no positive extent — refused"
        ));
    }
    if minlon < -180.0 || maxlon > 180.0 || minlat < -90.0 || maxlat > 90.0 {
        return Err(format!(
            "bbox {minlon},{maxlon} × {minlat},{maxlat} lies outside the geographic grid — refused"
        ));
    }
    let out_path = match arg_value(args, "--out") {
        Some(p) => p,
        None => format!("data/{NETLOC}/gmrt_bathymetry.bin"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let url = grid_url(minlon, maxlon, minlat, maxlat);
    eprintln!("gmrt: {url}");
    let Some(bytes) = fetch_raw_bytes(&url, 600) else {
        return Err(format!("{url}: fetch returned void"));
    };
    let recs = collect_elevation(&bytes)?;
    report_value_ranges(&recs);
    let written = write_asset(&recs, &out_path)?;
    eprintln!("{out_path}: {written} elevation records, roundtrip parses");

    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("gmrt_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_url_carries_the_bbox() {
        assert_eq!(
            grid_url(-10.0, -9.0, 40.0, 41.0),
            "https://www.gmrt.org/services/GridServer?minlongitude=-10&maxlongitude=-9&minlatitude=40&maxlatitude=41&format=geotiff"
        );
        assert_eq!(
            grid_url(-9.5, -9.0, 40.25, 40.75),
            "https://www.gmrt.org/services/GridServer?minlongitude=-9.5&maxlongitude=-9&minlatitude=40.25&maxlatitude=40.75&format=geotiff"
        );
    }

    #[test]
    fn gmr_magic_roundtrips_as_gestalt() {
        let recs = vec![GbcoRec {
            lat: 40.5,
            lon: -9.5,
            elev: -294.0,
        }];
        let bytes = write_gmr(&recs);
        assert_eq!(parse_gmr(&bytes).unwrap().len(), 1);
        assert_eq!(
            magic_identity(MAGIC_GMR),
            Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt))
        );
    }
}
