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
const GRS80_A: f64 = 6_378_137.0;
const GRS80_INV_F: f64 = 298.257_222_101;
const UTM_K0: f64 = 0.9996;
const UTM_FALSE_EASTING: f64 = 500_000.0;
const UTM_FALSE_NORTHING_SOUTH: f64 = 10_000_000.0;

enum CrsAxis {
    Geographic,
    WebMercator,
    Utm { zone: u16, southern: bool },
    Nad83Utm { zone: u16 },
}

impl CrsAxis {
    fn name(&self) -> String {
        match self {
            CrsAxis::Geographic => "geographic (EPSG:4326, lon=x lat=y)".to_string(),
            CrsAxis::WebMercator => "WGS 84 / Pseudo-Mercator (EPSG:3857)".to_string(),
            CrsAxis::Utm { zone, southern } => format!(
                "WGS 84 / UTM zone {zone}{} (GRS80 series inverse)",
                if *southern { "S" } else { "N" }
            ),
            CrsAxis::Nad83Utm { zone } => {
                format!("NAD83 / UTM zone {zone}N (GRS80 series inverse)")
            }
        }
    }
}

fn wkt_epsg(wkt: &str) -> Option<u16> {
    let bytes = wkt.as_bytes();
    let mut depth = 0u32;
    let mut in_string = false;
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        match b {
            b'"' => {
                in_string = true;
                i += 1;
            }
            b'[' => {
                depth += 1;
                i += 1;
            }
            b']' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            _ => {
                if depth == 1 && i + 10 <= bytes.len() && &bytes[i..i + 3] == b"ID[" {
                    let rest = &bytes[i + 3..];
                    let tag = &rest[..rest.len().min(7)];
                    if tag.starts_with(b"\"EPSG\",") {
                        let digits = &rest[7..];
                        let count = digits
                            .iter()
                            .take_while(|c| c.is_ascii_digit())
                            .count();
                        if count > 0 {
                            return std::str::from_utf8(&digits[..count])
                                .ok()
                                .and_then(|s| s.parse().ok());
                        }
                    }
                }
                i += 1;
            }
        }
    }
    None
}

fn resolve_crs(crs: &LasCrs) -> Option<CrsAxis> {
    match crs {
        LasCrs::Epsg(4326) => Some(CrsAxis::Geographic),
        LasCrs::Epsg(3857) => Some(CrsAxis::WebMercator),
        LasCrs::Epsg(code) if (32601..=32660).contains(code) => Some(CrsAxis::Utm {
            zone: code - 32600,
            southern: false,
        }),
        LasCrs::Epsg(code) if (32701..=32760).contains(code) => Some(CrsAxis::Utm {
            zone: code - 32700,
            southern: true,
        }),
        LasCrs::Epsg(code) if (26901..=26923).contains(code) => Some(CrsAxis::Nad83Utm {
            zone: code - 26900,
        }),
        LasCrs::Wkt(wkt) => wkt_epsg(wkt).and_then(|code| resolve_crs(&LasCrs::Epsg(code))),
        _ => None,
    }
}

fn utm_inverse(zone: u16, southern: bool, easting: f64, northing: f64) -> Option<(f64, f64)> {
    let f = 1.0 / GRS80_INV_F;
    if !f.is_finite() {
        return None;
    }
    let e2 = f * (2.0 - f);
    if !e2.is_finite() {
        return None;
    }
    let ep2 = e2 / (1.0 - e2);
    if !ep2.is_finite() {
        return None;
    }
    let x = easting - UTM_FALSE_EASTING;
    let y = northing - if southern {
        UTM_FALSE_NORTHING_SOUTH
    } else {
        0.0
    };
    let m = y / UTM_K0;
    if !m.is_finite() {
        return None;
    }
    let mu = m
        / (GRS80_A * (1.0 - e2 / 4.0 - 3.0 * e2 * e2 / 64.0 - 5.0 * e2 * e2 * e2 / 256.0));
    if !mu.is_finite() {
        return None;
    }
    let e1 = (1.0 - (1.0 - e2).sqrt()) / (1.0 + (1.0 - e2).sqrt());
    if !e1.is_finite() {
        return None;
    }
    let phi1 = mu
        + (3.0 * e1 / 2.0 - 27.0 * e1.powi(3) / 32.0) * (2.0 * mu).sin()
        + (21.0 * e1.powi(2) / 16.0 - 55.0 * e1.powi(4) / 32.0) * (4.0 * mu).sin()
        + (151.0 * e1.powi(3) / 96.0) * (6.0 * mu).sin()
        + (1097.0 * e1.powi(4) / 512.0) * (8.0 * mu).sin();
    if !phi1.is_finite() {
        return None;
    }
    let sin_phi1 = phi1.sin();
    let cos_phi1 = phi1.cos();
    if !sin_phi1.is_finite() || !cos_phi1.is_finite() {
        return None;
    }
    let tan_phi1 = phi1.tan();
    if !tan_phi1.is_finite() {
        return None;
    }
    let denom = 1.0 - e2 * sin_phi1 * sin_phi1;
    let n1 = GRS80_A / denom.sqrt();
    if !n1.is_finite() {
        return None;
    }
    let r1 = GRS80_A * (1.0 - e2) / denom.powf(1.5);
    if !r1.is_finite() {
        return None;
    }
    let t1 = tan_phi1 * tan_phi1;
    let c1 = ep2 * cos_phi1 * cos_phi1;
    let d = x / (n1 * UTM_K0);
    if !d.is_finite() {
        return None;
    }
    let phi = phi1
        - (n1 * tan_phi1 / r1)
            * (d * d / 2.0
                - (5.0 + 3.0 * t1 + 10.0 * c1 - 4.0 * c1 * c1 - 9.0 * ep2) * d.powi(4) / 24.0
                + (61.0 + 90.0 * t1 + 298.0 * c1 + 45.0 * t1 * t1 - 252.0 * ep2
                    - 3.0 * c1 * c1)
                    * d.powi(6)
                    / 720.0);
    if !phi.is_finite() {
        return None;
    }
    let lam = (zone as f64 * 6.0 - 183.0).to_radians()
        + (d - (1.0 + 2.0 * t1 + c1) * d.powi(3) / 6.0
            + (5.0 - 2.0 * c1 + 28.0 * t1 - 3.0 * c1 * c1 + 8.0 * ep2 + 24.0 * t1 * t1)
                * d.powi(5)
                / 120.0)
            / cos_phi1;
    if !lam.is_finite() {
        return None;
    }
    let lat = phi.to_degrees();
    let lon = lam.to_degrees();
    if lat.is_finite() && lon.is_finite() {
        Some((lat, lon))
    } else {
        None
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
        CrsAxis::Utm { zone, southern } => {
            utm_inverse(*zone, *southern, x, y).map(|(lat, lon)| (lat, lon, z))
        }
        CrsAxis::Nad83Utm { zone } => {
            utm_inverse(*zone, false, x, y).map(|(lat, lon)| (lat, lon, z))
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
        "z carried as delivered (height m); no vertical datum chain is built — the geoid undulation to the WGS84 ellipsoid and tide datums (MLLW) stay unmeasured (pending)"
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
    fn wkt_epsg_reads_the_root_level_epsg_id() {
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
    fn wkt_epsg_ignores_ids_nested_in_the_base_geogcrs() {
        assert_eq!(
            wkt_epsg(
                "PROJCRS[\"WGS 84 / UTM zone 23S\",BASEGEOGCRS[\"WGS 84\",DATUM[\"World Geodetic System 1984\"],ID[\"EPSG\",4326]],CONVERSION[\"UTM zone 23S\",METHOD[\"Transverse Mercator\",ID[\"EPSG\",9807]],PARAMETER[\"Latitude of natural origin\",0,ID[\"EPSG\",8801]]]]"
            ),
            None
        );
    }

    #[test]
    fn resolves_built_axes_and_utm_zones_only() {
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(4326)),
            Some(CrsAxis::Geographic)
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(3857)),
            Some(CrsAxis::WebMercator)
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(32611)),
            Some(CrsAxis::Utm {
                zone: 11,
                southern: false
            })
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(32711)),
            Some(CrsAxis::Utm {
                zone: 11,
                southern: true
            })
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(32601)),
            Some(CrsAxis::Utm {
                zone: 1,
                southern: false
            })
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(32660)),
            Some(CrsAxis::Utm {
                zone: 60,
                southern: false
            })
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(32760)),
            Some(CrsAxis::Utm {
                zone: 60,
                southern: true
            })
        ));
        assert!(resolve_crs(&LasCrs::Epsg(32600)).is_none());
        assert!(resolve_crs(&LasCrs::Epsg(32661)).is_none());
        assert!(resolve_crs(&LasCrs::Epsg(32700)).is_none());
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(26904)),
            Some(CrsAxis::Nad83Utm { zone: 4 })
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(26901)),
            Some(CrsAxis::Nad83Utm { zone: 1 })
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Epsg(26923)),
            Some(CrsAxis::Nad83Utm { zone: 23 })
        ));
        assert!(resolve_crs(&LasCrs::Epsg(26900)).is_none());
        assert!(resolve_crs(&LasCrs::Epsg(26924)).is_none());
        assert!(matches!(
            resolve_crs(&LasCrs::Wkt(
                "PROJCRS[\"WGS 84 / Pseudo-Mercator\",ID[\"EPSG\",3857]]".to_string()
            )),
            Some(CrsAxis::WebMercator)
        ));
        assert!(matches!(
            resolve_crs(&LasCrs::Wkt(
                "PROJCRS[\"WGS 84 / UTM zone 11N\",BASEGEOGCRS[\"WGS 84\"],ID[\"EPSG\",32611]]"
                    .to_string()
            )),
            Some(CrsAxis::Utm {
                zone: 11,
                southern: false
            })
        ));
        assert!(
            resolve_crs(&LasCrs::Wkt(
                "PROJCRS[\"raw\",METHOD[\"Transverse Mercator\"]]".to_string()
            ))
            .is_none()
        );
        assert!(
            resolve_crs(&LasCrs::Wkt(
                "PROJCRS[\"WGS 84 / UTM zone 23S\",BASEGEOGCRS[\"WGS 84\",ID[\"EPSG\",4326]]]"
                    .to_string()
            ))
            .is_none()
        );
    }

    #[test]
    fn utm_inverse_zone18n_central_meridian_reference() {
        let (lat, lon) = utm_inverse(18, false, 500000.0, 4649776.224884).unwrap();
        assert!((lat - 42.0).abs() < 1e-6);
        assert!((lon - (-75.0)).abs() < 1e-6);
    }

    #[test]
    fn utm_inverse_zone18n_dd10045_corner_reference() {
        let (lat, lon) = utm_inverse(18, false, 368952.482, 4340727.52).unwrap();
        assert!((lat - 39.205974).abs() < 1e-5);
        assert!((lon - (-76.517841)).abs() < 1e-5);
    }

    #[test]
    fn utm_inverse_zone32n_proj_reference() {
        let (lat, lon) = utm_inverse(32, false, 691875.63214, 6098907.82501).unwrap();
        assert!((lat - 55.0).abs() < 1e-5);
        assert!((lon - 12.0).abs() < 1e-5);
    }

    #[test]
    fn utm_inverse_southern_equator_lands_on_the_false_northing() {
        let (lat, lon) = utm_inverse(18, true, 500000.0, 10000000.0).unwrap();
        assert!((lat - 0.0).abs() < 1e-6);
        assert!((lon - (-75.0)).abs() < 1e-6);
    }

    #[test]
    fn utm_inverse_southern_arc_mirrors_the_northern_reference() {
        let (lat, lon) = utm_inverse(18, true, 500000.0, 10000000.0 - 4649776.224884).unwrap();
        assert!((lat - (-42.0)).abs() < 1e-6);
        assert!((lon - (-75.0)).abs() < 1e-6);
    }

    #[test]
    fn utm_inverse_rejects_non_finite_input() {
        assert!(utm_inverse(18, false, f64::NAN, 4649776.224884).is_none());
        assert!(utm_inverse(18, false, 500000.0, f64::INFINITY).is_none());
        assert!(utm_inverse(18, false, f64::INFINITY, 4649776.224884).is_none());
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
