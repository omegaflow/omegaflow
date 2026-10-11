use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GbcoRec, MAGIC_GMR, parse_gmr, write_gmr};
use omegaflow::cdn::upload_release;
use omegaflow::witness::{FieldIdentity, WitnessKind, magic_identity};

const NETLOC: &str = "erddap.riddc.brown.edu";
const GRIDDAP: &str = "https://erddap.riddc.brown.edu/erddap/griddap/bluetopo_5899_7471_de41.csv";

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

fn grid_url(minlon: f64, maxlon: f64, minlat: f64, maxlat: f64) -> String {
    format!("{GRIDDAP}?bathymetry%5B({minlat}):1:({maxlat})%5D%5B({minlon}):1:({maxlon})%5D")
}

fn collect_points(body: &str) -> Vec<GbcoRec> {
    let mut recs = Vec::new();
    for line in body.lines().skip(2) {
        let mut it = line.split(',');
        let (Some(ls), Some(ln), Some(le)) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        let (Ok(lat), Ok(lon), Ok(elev)) = (
            ls.trim().parse::<f64>(),
            ln.trim().parse::<f64>(),
            le.trim().parse::<f64>(),
        ) else {
            continue;
        };
        if !lat.is_finite() || !lon.is_finite() || !elev.is_finite() {
            continue;
        }
        if lat < -90.0 || lat > 90.0 || lon < -180.0 || lon > 180.0 {
            continue;
        }
        recs.push(GbcoRec { lat, lon, elev });
    }
    recs
}

fn report_value_ranges(recs: &[GbcoRec]) {
    let min = recs.iter().map(|r| r.elev).fold(f64::INFINITY, f64::min);
    let max = recs
        .iter()
        .map(|r| r.elev)
        .fold(f64::NEG_INFINITY, f64::max);
    let mean = recs.iter().map(|r| r.elev).sum::<f64>() / recs.len() as f64;
    eprintln!("bathymetry: min {min:.3} max {max:.3} mean {mean:.3} m");
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
        None => format!("data/{NETLOC}/bluetopo_bathymetry.bin"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let url = grid_url(minlon, maxlon, minlat, maxlat);
    eprintln!("bluetopo: {url}");
    let Some(bytes) = fetch_raw_bytes(&url) else {
        return Err(format!("{url}: fetch returned void"));
    };
    let body = String::from_utf8_lossy(&bytes);
    let recs = collect_points(&body);
    if recs.is_empty() {
        return Err(
            "no finite bathymetry point in the griddap response — the asset stays unwritten (0 honored)"
                .to_string(),
        );
    }
    report_value_ranges(&recs);
    let written = write_asset(&recs, &out_path)?;
    eprintln!("{out_path}: {written} bathymetry records, roundtrip parses");

    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("bluetopo_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn griddap_csv_yields_gestalt_points() {
        let body = "latitude,longitude,bathymetry\n\
                    degrees_north,degrees_east,\n\
                    42.00000731796417,-70.50000732413663,NaN\n\
                    42.00000731796417,-70.49997732413664,-37.126385\n\
                    42.00999731796417,-70.48998732413664,-39.033833\n\
                    999.0,-70.4,-12.0\n";
        let recs = collect_points(body);
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].lat, 42.00000731796417);
        assert_eq!(recs[0].lon, -70.49997732413664);
        assert_eq!(recs[0].elev, -37.126385);
        assert_eq!(recs[1].elev, -39.033833);
        let bytes = write_gmr(&recs);
        assert_eq!(parse_gmr(&bytes).unwrap().len(), 2);
        assert_eq!(
            magic_identity(MAGIC_GMR),
            Some(FieldIdentity::Witness(WitnessKind::Gestalt))
        );
    }
}
