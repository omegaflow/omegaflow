use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::motion::body_fixed_to_icrs;
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::las::las_series::{HEADER_BYTES, LasSample, REC_BYTES, parse_series, write_bin};
use omegaflow::las::{LasCrs, LasHeader, LazDecoder, has_laszip_vlr, projection_crs};
use std::collections::HashMap;
use std::io::{BufWriter, Write};

const NETLOC: &str = "usgs-lidar-public.s3.amazonaws.com";
const GPS_UNIX_OFFSET: f64 = 315_964_800.0;
const WEB_MERCATOR_R: f64 = 6_378_137.0;

enum CrsAxis {
    Geographic,
    WebMercator,
}

impl CrsAxis {
    fn name(&self) -> &'static str {
        match self {
            CrsAxis::Geographic => "geographic (EPSG:4326, lon=x lat=y)",
            CrsAxis::WebMercator => "WGS 84 / Pseudo-Mercator (EPSG:3857)",
        }
    }
}

fn wkt_epsg(wkt: &str) -> Option<u16> {
    let pos = wkt.find("ID[\"EPSG\",")?;
    let rest = &wkt[pos + 10..];
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn resolve_crs(crs: &LasCrs) -> Option<CrsAxis> {
    match crs {
        LasCrs::Epsg(4326) => Some(CrsAxis::Geographic),
        LasCrs::Epsg(3857) => Some(CrsAxis::WebMercator),
        LasCrs::Wkt(wkt) => wkt_epsg(wkt).and_then(|code| resolve_crs(&LasCrs::Epsg(code))),
        _ => None,
    }
}

fn crs_to_geodetic(axis: &CrsAxis, x: f64, y: f64, z: f64) -> Option<(f64, f64, f64)> {
    match axis {
        CrsAxis::Geographic => Some((y, x, z)),
        CrsAxis::WebMercator => {
            let lon = x / WEB_MERCATOR_R;
            let lat = 2.0 * (y / WEB_MERCATOR_R).exp().atan() - std::f64::consts::FRAC_PI_2;
            Some((lat.to_degrees(), lon.to_degrees(), z))
        }
    }
}

fn gps_to_unix(lsk: &omegaflow::archivar::LeapSeconds, gps: f64) -> Option<f64> {
    for &(offset, _) in lsk.deltas.iter().rev() {
        let unix = gps + GPS_UNIX_OFFSET - offset;
        if lsk.leap_at(unix) == Some(offset) {
            return Some(unix);
        }
    }
    None
}

struct Skips {
    clock: usize,
    frame: usize,
    decode: usize,
}

fn collect(
    bytes: &[u8],
    header: &LasHeader,
    compressed: bool,
    axis: &CrsAxis,
    lsk: &omegaflow::archivar::LeapSeconds,
    eph: &HashMap<String, omegaflow::archivar::motion::BodyEphemeris>,
) -> Result<(Vec<LasSample>, Skips), String> {
    let mut dec = if compressed {
        Some(
            LazDecoder::new(header, bytes)
                .map_err(|note| format!("laszip layout unread — {note:?}"))?,
        )
    } else {
        None
    };
    let mut samples = Vec::new();
    let mut skips = Skips {
        clock: 0,
        frame: 0,
        decode: 0,
    };
    for i in 0..header.point_count {
        let point = match &mut dec {
            Some(d) => d.point_at(i),
            None => header.point_at(bytes, i),
        };
        let Ok(point) = point else {
            skips.decode += 1;
            continue;
        };
        if !point.x.is_finite() || !point.y.is_finite() || !point.z.is_finite() {
            skips.frame += 1;
            continue;
        }
        let Some(gps) = point.gps_time else {
            skips.clock += 1;
            continue;
        };
        let Some(tdb) = gps_to_unix(lsk, gps).and_then(|unix| lsk.unix_to_tdb(unix)) else {
            skips.clock += 1;
            continue;
        };
        let Some((lat, lon, alt)) = crs_to_geodetic(axis, point.x, point.y, point.z) else {
            skips.frame += 1;
            continue;
        };
        let Some(icrs) = body_fixed_to_icrs("earth", lat, lon, alt, tdb, eph) else {
            skips.frame += 1;
            continue;
        };
        samples.push(LasSample {
            t_tdb: tdb,
            x: icrs[0],
            y: icrs[1],
            z: icrs[2],
            intensity: point.intensity,
            classification: point.classification,
        });
    }
    Ok((samples, skips))
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn crs_text(crs: &LasCrs) -> String {
    match crs {
        LasCrs::Epsg(code) => format!("EPSG:{code}"),
        LasCrs::Wkt(wkt) => {
            let cut = wkt.chars().take(120).collect::<String>();
            format!("WKT {cut}…")
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: las_compiler --input <las|laz|copc.laz> | --url <https-url> --out <path> [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };

    let (bytes, name) = match arg_value(&args, "--input") {
        Some(path) => {
            let leaf = path.rsplit('/').next().unwrap_or(&path).to_string();
            match std::fs::read(&path) {
                Ok(b) => (b, leaf),
                Err(e) => {
                    eprintln!("{path}: read returned void ({e})");
                    std::process::exit(1);
                }
            }
        }
        None => match arg_value(&args, "--url") {
            Some(url) => {
                let leaf = url.rsplit('/').next().unwrap_or(&url).to_string();
                match fetch_raw_bytes(&url, 600) {
                    Some(b) => (b, leaf),
                    None => {
                        eprintln!("{url}: fetch returned void");
                        std::process::exit(1);
                    }
                }
            }
            None => {
                eprintln!("{usage}");
                std::process::exit(1);
            }
        },
    };

    let header = match LasHeader::parse(&bytes) {
        Ok(h) => h,
        Err(note) => {
            eprintln!("{name}: header parse returned void — {note:?}");
            std::process::exit(1);
        }
    };
    let vlrs = match header.vlrs(&bytes) {
        Ok(v) => v,
        Err(note) => {
            eprintln!("{name}: VLR block unread — {note:?}");
            std::process::exit(1);
        }
    };
    let crs = match projection_crs(&vlrs) {
        Some(c) => c,
        None => {
            eprintln!(
                "{name}: no projection VLR carries a resolvable CRS (no GeoKey EPSG, no WKT ID) — the points stay unframed (0 honored)"
            );
            std::process::exit(1);
        }
    };
    let axis = match resolve_crs(&crs) {
        Some(a) => a,
        None => {
            eprintln!(
                "{name}: crs {} unresolved — no inverse projection is built for it (pending)",
                crs_text(&crs)
            );
            std::process::exit(1);
        }
    };
    eprintln!("crs {} — {}", crs_text(&crs), axis.name());
    eprintln!(
        "z carried as delivered (height m); the file's VLRs carry no vertical datum — the geoid undulation to the WGS84 ellipsoid stays unmeasured (pending)"
    );

    let eph_bytes = match fetch_raw_bytes(&body_url("earth"), 600) {
        Some(b) => b,
        None => {
            eprintln!("earth ephemeris: fetch returned void — the ICRS frame stays unreached");
            std::process::exit(1);
        }
    };
    let earth = match parse_ephemeris_binary(&eph_bytes) {
        Some(e) => e,
        None => {
            eprintln!("earth ephemeris: binary unread — the ICRS frame stays unreached");
            std::process::exit(1);
        }
    };
    let eph = HashMap::from([("earth".to_string(), earth)]);
    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("leap table: the embedded naif0012 table does not parse");
            std::process::exit(1);
        }
    };

    let compressed = has_laszip_vlr(&vlrs);
    eprintln!(
        "{}: {} points, {} {}",
        name,
        header.point_count,
        if compressed { "laszip" } else { "uncompressed" },
        if compressed { "chunks" } else { "records" }
    );

    let (samples, skips) = match collect(&bytes, &header, compressed, &axis, &lsk, &eph) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("las_compiler: {e}");
            std::process::exit(1);
        }
    };
    eprintln!(
        "{} samples framed, {} without clock (absent gps_time), {} without frame, {} decode void",
        samples.len(),
        skips.clock,
        skips.frame,
        skips.decode
    );
    if samples.is_empty() {
        eprintln!("las_compiler: no framed sample — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }

    let bin = write_bin(&samples);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let file = match std::fs::File::create(&out_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create {out_path}: {e}");
            std::process::exit(1);
        }
    };
    let mut out = BufWriter::new(file);
    if let Err(e) = out.write_all(&bin) {
        eprintln!("write {out_path}: {e}");
        std::process::exit(1);
    }
    if let Err(e) = out.flush() {
        eprintln!("flush {out_path}: {e}");
        std::process::exit(1);
    }
    let expect = HEADER_BYTES + samples.len() * REC_BYTES;
    let actual = match std::fs::metadata(&out_path) {
        Ok(m) => m.len() as usize,
        Err(e) => {
            eprintln!("stat {out_path}: {e}");
            std::process::exit(1);
        }
    };
    if actual != expect {
        eprintln!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        );
        std::process::exit(1);
    }

    match parse_series(&bin) {
        Some(parsed) if parsed.len() == samples.len() * 5 => {
            let t_min = samples
                .iter()
                .map(|s| s.t_tdb)
                .fold(f64::INFINITY, f64::min);
            let t_max = samples
                .iter()
                .map(|s| s.t_tdb)
                .fold(f64::NEG_INFINITY, f64::max);
            let first = &samples[0];
            eprintln!(
                "las: {} samples, {} B -> {out_path}, roundtrip parses ({} series points), tdb span {t_min:.3}..{t_max:.3} s, first icrs ({:.3}, {:.3}, {:.3}) m intensity {} class {}",
                samples.len(),
                expect,
                parsed.len(),
                first.x,
                first.y,
                first.z,
                first.intensity,
                first.classification
            );
        }
        _ => {
            eprintln!("{out_path}: roundtrip parse returned void — the bin stays unverified");
            std::process::exit(1);
        }
    }

    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("upload: {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::LeapSeconds;

    fn test_lsk() -> LeapSeconds {
        LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(16.0, 1.1e9), (17.0, 1.3e9)],
        }
    }

    #[test]
    fn gps_time_maps_to_unix_through_the_leap_table() {
        let lsk = test_lsk();
        let unix = 1.2e9;
        let gps = unix - GPS_UNIX_OFFSET + 16.0;
        assert_eq!(gps_to_unix(&lsk, gps), Some(unix));
        assert_eq!(
            lsk.unix_to_tdb(unix),
            Some(unix + 32.184 + 16.0 - 946_728_000.0)
        );
        let unix = 1.35e9;
        let gps = unix - GPS_UNIX_OFFSET + 17.0;
        assert_eq!(gps_to_unix(&lsk, gps), Some(unix));
    }

    #[test]
    fn gps_time_before_the_first_leap_entry_stays_unmapped() {
        let lsk = test_lsk();
        let gps = 0.0;
        assert_eq!(gps_to_unix(&lsk, gps), None);
    }

    #[test]
    fn wkt_epsg_reads_the_first_epsg_id() {
        assert_eq!(
            wkt_epsg(
                "PROJCRS[\"WGS 84 / UTM zone 11N\",BASEGEOGCRS[\"WGS 84\"],ID[\"EPSG\",32611]]"
            ),
            Some(32611)
        );
        assert_eq!(
            wkt_epsg("PROJCRS[\"WGS 84 / Pseudo-Mercator\",ID[\"EPSG\",3857]]"),
            Some(3857)
        );
        assert_eq!(wkt_epsg("PROJCRS[\"no id\"]"), None);
    }

    #[test]
    fn resolves_the_two_built_axes_only() {
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(4326)),
            Some(CrsAxis::Geographic)
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(3857)),
            Some(CrsAxis::WebMercator)
        ));
        assert!(resolve_crs(&LasCrs::Epsg(32611)).is_none());
        assert!(resolve_crs(&LasCrs::Epsg(26911)).is_none());
        assert!(matches!(
            resolve_crs(&LasCrs::Wkt(
                "PROJCRS[\"WGS 84 / Pseudo-Mercator\",ID[\"EPSG\",3857]]".to_string()
            )),
            Some(CrsAxis::WebMercator)
        ));
        assert!(
            resolve_crs(&LasCrs::Wkt(
                "PROJCRS[\"raw\",METHOD[\"Transverse Mercator\"]]".to_string()
            ))
            .is_none()
        );
    }

    #[test]
    fn web_mercator_inverse_roundtrips_the_reference_point() {
        let (lat, lon, alt) =
            crs_to_geodetic(&CrsAxis::WebMercator, -11_688_571.76, 4_865_942.28, 1600.0).unwrap();
        assert!((lat - 40.0).abs() < 1e-4);
        assert!((lon - -105.0).abs() < 1e-4);
        assert_eq!(alt, 1600.0);
    }

    #[test]
    fn geographic_axis_passes_lon_lat_through() {
        let (lat, lon, alt) = crs_to_geodetic(&CrsAxis::Geographic, -105.0, 40.0, 1600.0).unwrap();
        assert_eq!(lat, 40.0);
        assert_eq!(lon, -105.0);
        assert_eq!(alt, 1600.0);
    }
}
