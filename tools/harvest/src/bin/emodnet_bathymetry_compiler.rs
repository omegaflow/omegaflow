use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GbcoRec, MAGIC_EMOD, parse_emod, write_emod};
use omegaflow::cdn::upload_release;
use omegaflow::json::{jpath, parse_json};
use omegaflow::witness::{FieldIdentity, WitnessKind, magic_identity};

const NETLOC: &str = "emodnet-bathymetry.eu";
const ENDPOINT: &str = "https://rest.emodnet-bathymetry.eu/depth_sample";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn witness_gestalt_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FieldIdentity::Witness(WitnessKind::Gestalt)) => {
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

fn sample_url(lon: f64, lat: f64) -> String {
    format!("{ENDPOINT}?geom=POINT({lon}%20{lat})")
}

fn grid_points(minlon: f64, maxlon: f64, minlat: f64, maxlat: f64, step: f64) -> Vec<(f64, f64)> {
    let nlon = ((maxlon - minlon) / step).floor() as i64;
    let nlat = ((maxlat - minlat) / step).floor() as i64;
    let mut pts = Vec::new();
    for iy in 0..=nlat {
        for ix in 0..=nlon {
            let lon = minlon + ix as f64 * step;
            let lat = minlat + iy as f64 * step;
            if lon.is_finite() && lat.is_finite() {
                pts.push((lon, lat));
            }
        }
    }
    pts
}

fn fetch_depth(lon: f64, lat: f64) -> Option<f64> {
    let url = sample_url(lon, lat);
    let bytes = fetch_raw_bytes(&url)?;
    let body = String::from_utf8_lossy(&bytes);
    let json = parse_json(&body)?;
    let v = jpath(&json, "avg")?;
    if v.is_finite() { Some(v) } else { None }
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
    let bytes = write_emod(recs);
    std::fs::write(out_path, &bytes).map_err(|e| format!("write {out_path} returned void: {e}"))?;
    match parse_emod(&bytes) {
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
    witness_gestalt_identity(MAGIC_EMOD)?;
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
    let Some(step_raw) = arg_value(args, "--step") else {
        return Err("--step <deg>: the sample spacing is never silent — refused".to_string());
    };
    let step = step_raw
        .parse::<f64>()
        .map_err(|_| format!("--step {step_raw} does not parse as degrees"))?;
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
    if !step.is_finite() || step <= 0.0 {
        return Err(format!(
            "--step {step} carries no positive sampling interval — refused"
        ));
    }
    let out_path = match arg_value(args, "--out") {
        Some(p) => p,
        None => format!("data/{NETLOC}/emodnet_bathymetry.bin"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let pts = grid_points(minlon, maxlon, minlat, maxlat, step);
    eprintln!(
        "emodnet: {minlon},{minlat} .. {maxlon},{maxlat} at step {step}° -> {} points",
        pts.len()
    );
    let mut recs = Vec::new();
    for (lon, lat) in &pts {
        match fetch_depth(*lon, *lat) {
            Some(d) => recs.push(GbcoRec {
                lat: *lat,
                lon: *lon,
                elev: d,
            }),
            None => eprintln!("emodnet: {lon},{lat} returned no depth — skipped (0 honored)"),
        }
    }
    if recs.is_empty() {
        return Err("no depth measured — the asset stays unwritten (0 honored)".to_string());
    }
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
        eprintln!("emodnet_bathymetry_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_url_carries_the_point() {
        assert_eq!(
            sample_url(10.0, 54.0),
            "https://rest.emodnet-bathymetry.eu/depth_sample?geom=POINT(10%2054)"
        );
        assert_eq!(
            sample_url(-30.0, 40.5),
            "https://rest.emodnet-bathymetry.eu/depth_sample?geom=POINT(-30%2040.5)"
        );
    }

    #[test]
    fn grid_points_cover_the_bbox_corners() {
        let pts = grid_points(0.0, 1.0, 0.0, 1.0, 0.5);
        assert_eq!(pts.len(), 9);
        assert_eq!(pts[0], (0.0, 0.0));
        assert_eq!(*pts.last().unwrap(), (1.0, 1.0));
    }

    #[test]
    fn emod_magic_roundtrips_as_gestalt() {
        let recs = vec![GbcoRec {
            lat: 54.0,
            lon: 10.0,
            elev: 21.7,
        }];
        let bytes = write_emod(&recs);
        assert_eq!(parse_emod(&bytes).unwrap().len(), 1);
        assert_eq!(
            magic_identity(MAGIC_EMOD),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
    }
}
