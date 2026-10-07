use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;

const MAGIC: &[u8; 4] = b"CRSM";
const FIELDS: usize = 4;
const NETLOC: &str = "donnees-data.asc-csa.gc.ca";
const BASE: &str = "https://donnees-data.asc-csa.gc.ca/users/OpenData_DonneesOuvertes/pub/CARISMA/carisma_csv/mag/daily";
const COMPILER: &str = "tools/harvest/src/bin/carisma_mag_compiler.rs";
const TTL_S: f64 = 86400.0;
const SAMPLE_TAU_S: f64 = 1.0;
const FILL_NT: f64 = 99999.999;
const VALID_FLAG: &str = ".";

#[derive(Clone, Debug, PartialEq)]
struct MagStation {
    code: String,
    lat: f64,
    lon: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct MagSample {
    t_unix: f64,
    b: Option<[f64; 3]>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_station(line: &str) -> Option<MagStation> {
    let mut it = line.split_whitespace();
    let code = it.next()?.to_string();
    let lat: f64 = it.next()?.parse().ok()?;
    let lon: f64 = it.next()?.parse().ok()?;
    let day = it.next()?;
    let coord = it.next()?;
    let unit = it.next()?;
    if day.len() != 8 || !day.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if coord != "GEODETIC" || unit != "nT" {
        return None;
    }
    if !lat.is_finite() || !lon.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return None;
    }
    Some(MagStation { code, lat, lon })
}

fn parse_datetime(date: &str, time: &str) -> Option<f64> {
    let mut d = date.split('/');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    if d.next().is_some() {
        return None;
    }
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let ss: f64 = t.next()?.parse().ok()?;
    if t.next().is_some() || !(0.0..=60.0).contains(&ss) {
        return None;
    }
    let days = days_from_civil(y, mo, da)?;
    Some(days as f64 * 86400.0 + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss)
}

fn component(token: &str) -> Option<f64> {
    let v: f64 = token.parse().ok()?;
    if v.is_finite() && v != FILL_NT {
        Some(v)
    } else {
        None
    }
}

fn parse_row(line: &str) -> Option<MagSample> {
    let cells: Vec<&str> = line.split(',').collect();
    if cells.len() != 6 {
        return None;
    }
    let t_unix = parse_datetime(cells[0].trim(), cells[1].trim())?;
    let present = cells[5].trim() == VALID_FLAG;
    let b = match (
        component(cells[2].trim()),
        component(cells[3].trim()),
        component(cells[4].trim()),
    ) {
        (Some(x), Some(y), Some(z)) if present => Some([x, y, z]),
        _ => None,
    };
    Some(MagSample { t_unix, b })
}

fn parse_carisma(text: &str) -> Option<(MagStation, Vec<MagSample>)> {
    let lines: Vec<&str> = text.lines().collect();
    let site_idx = lines.iter().position(|l| l.starts_with("# Site"))?;
    let station_line = lines[site_idx + 1..]
        .iter()
        .find(|l| !l.trim().is_empty())?;
    let station = parse_station(station_line)?;
    let date_idx = lines.iter().position(|l| l.starts_with("# Date"))?;
    let mut samples: Vec<MagSample> = Vec::new();
    for line in &lines[date_idx + 1..] {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(s) = parse_row(line) {
            samples.push(s);
        }
    }
    if samples.is_empty() {
        return None;
    }
    Some((station, samples))
}

fn to_records(samples: &[MagSample]) -> Vec<[f64; FIELDS]> {
    let mut out = Vec::new();
    for s in samples {
        let Some([x, y, z]) = s.b else { continue };
        out.push([s.t_unix, x, y, z]);
    }
    out
}

fn write_bin(records: &[[f64; FIELDS]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * FIELDS * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; FIELDS]>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
    if bytes.len() != 8 + n * FIELDS * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut r = [0.0f64; FIELDS];
        for (j, slot) in r.iter_mut().enumerate() {
            let off = 8 + i * FIELDS * 8 + j * 8;
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

fn emit(text: &str, out: &str, source: &str) {
    let Some((station, samples)) = parse_carisma(text) else {
        eprintln!("{source}: CARISMA CSV parses void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let mut records = to_records(&samples);
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));
    if records.is_empty() {
        eprintln!(
            "{source}: {} samples carry no present vector — the bin stays unwritten (0 honored)",
            samples.len()
        );
        std::process::exit(1);
    }
    let bytes_out = write_bin(&records);
    if let Some(parent) = std::path::Path::new(out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let span = |i: usize| {
        (
            records.iter().map(|r| r[i]).fold(f64::INFINITY, f64::min),
            records
                .iter()
                .map(|r| r[i])
                .fold(f64::NEG_INFINITY, f64::max),
        )
    };
    let (x0, x1) = span(1);
    let (y0, y1) = span(2);
    let (z0, z1) = span(3);
    eprintln!(
        "{out}: {} records ({} samples, {} absent skipped) — X [{x0}, {x1}] Y [{y0}, {y1}] Z [{z0}, {z1}] nT",
        records.len(),
        samples.len(),
        samples.len() - records.len(),
    );
    eprintln!(
        "  station {} lat {} lon {} — {} bytes, sha256 {}",
        station.code,
        station.lat,
        station.lon,
        bytes_out.len(),
        sha256_hex(&bytes_out)
    );
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == records => {
            eprintln!("  roundtrip: {} records parse, identical", parsed.len());
        }
        Some(parsed) => {
            eprintln!(
                "  roundtrip: {} records parse but differ from the emitted set",
                parsed.len()
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("  roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    let out_name = match std::path::Path::new(out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.to_string(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {source}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format carisma_mag");
    println!("ttl {}", TTL_S as u64);
    println!("at earth");
    println!(
        "field carisma_mag_x_nt carisma_mag_x_nt inverse-square em nT {} 0.0 0.0",
        SAMPLE_TAU_S as u64
    );
    println!(
        "field carisma_mag_y_nt carisma_mag_y_nt inverse-square em nT {} 0.0 0.0",
        SAMPLE_TAU_S as u64
    );
    println!(
        "field carisma_mag_z_nt carisma_mag_z_nt inverse-square em nT {} 0.0 0.0",
        SAMPLE_TAU_S as u64
    );
}

fn default_url(station: &str, date: &str) -> Option<String> {
    let year = date.get(0..4)?;
    if date.len() != 8 {
        return None;
    }
    Some(format!("{BASE}/{year}/{station}/{date}{station}.MAG.csv"))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "carisma_mag.bin".to_string(),
    };
    let (text, source) = match arg_value(&args, "--file") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => (t, path),
            Err(_) => {
                eprintln!("{path} unreadable — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--url") {
                Some(u) => u,
                None => {
                    let station = match arg_value(&args, "--station") {
                        Some(v) => v.to_uppercase(),
                        None => {
                            eprintln!(
                                "carisma_mag_compiler: --station <CODE> or --url <url> is required"
                            );
                            std::process::exit(2);
                        }
                    };
                    let date = match arg_value(&args, "--date") {
                        Some(v) => v,
                        None => {
                            eprintln!(
                                "carisma_mag_compiler: --date <YYYYMMDD> is required with --station"
                            );
                            std::process::exit(2);
                        }
                    };
                    match default_url(&station, &date) {
                        Some(u) => u,
                        None => {
                            eprintln!("carisma_mag_compiler: --date must be YYYYMMDD");
                            std::process::exit(2);
                        }
                    }
                }
            };
            match fetch_raw(&url, None, &[]) {
                Some(t) => (t, url),
                None => {
                    eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
                    std::process::exit(1);
                }
            }
        }
    };
    emit(&text, &out, &source);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured() -> String {
        let mut s = String::new();
        s.push_str(
            "#All materials produced using CARISMA data are subject to the GO Canada rules.\n",
        );
        s.push_str("# Site Lat Long yyyymmdd CoordSys Units no of records\n");
        s.push_str("DAWS  64.048 220.890 20090105 GEODETIC nT  1Hz\n");
        s.push_str(" \n");
        s.push_str("# Date(dd/mm/yyyy),time(hh:mi:ss),X,Y,Z,F=. if the data is valid anything else the data is suspect.\n");
        s.push_str("2009/01/05,00:00:00,11191.027,4689.269,56870.568,.\n");
        s.push_str("2009/01/05,00:00:01,11190.973,4689.264,56870.615,.\n");
        s.push_str("2009/01/05,00:00:02,99999.999,99999.999,99999.999,x\n");
        s.push_str("2009/01/05,00:00:03,11190.000,4689.000,56870.000,x\n");
        s
    }

    #[test]
    fn parse_carisma_reads_the_measured_file() {
        let (station, samples) =
            parse_carisma(&measured()).expect("the measured CARISMA CSV parses");
        assert_eq!(station.code, "DAWS");
        assert_eq!(station.lat, 64.048);
        assert_eq!(station.lon, 220.890);
        assert_eq!(samples.len(), 4);
        assert_eq!(samples[0].b, Some([11191.027, 4689.269, 56870.568]));
        assert_eq!(samples[1].b, Some([11190.973, 4689.264, 56870.615]));
        assert_eq!(samples[2].b, None);
        assert_eq!(samples[3].b, None);
        let days = days_from_civil(2009, 1, 5).unwrap();
        assert_eq!(samples[0].t_unix, (days * 86400) as f64);
        assert_eq!(samples[1].t_unix - samples[0].t_unix, 1.0);
    }

    #[test]
    fn to_records_skips_absent_vectors() {
        let (_, samples) = parse_carisma(&measured()).unwrap();
        let records = to_records(&samples);
        assert_eq!(records.len(), 2);
        let days = days_from_civil(2009, 1, 5).unwrap();
        assert_eq!(
            records[0],
            [(days * 86400) as f64, 11191.027, 4689.269, 56870.568]
        );
        assert_eq!(
            records[1],
            [(days * 86400 + 1) as f64, 11190.973, 4689.264, 56870.615]
        );
    }

    #[test]
    fn parse_carisma_refuses_foreign_text() {
        assert!(parse_carisma("").is_none());
        assert!(parse_carisma("# Site Lat Long yyyymmdd CoordSys Units no of records\nPCN  77.467 290.767 20150101 GEODETIC nT  1Hz\n").is_none());
        let wrong_unit = measured().replace("GEODETIC nT", "GEODETIC mV/m");
        assert!(parse_carisma(&wrong_unit).is_none());
        let no_site = measured().replace(
            "# Site Lat Long yyyymmdd CoordSys Units no of records",
            "# nothing here",
        );
        assert!(parse_carisma(&no_site).is_none());
        let empty_body = "# Site Lat Long yyyymmdd CoordSys Units no of records\nDAWS  64.048 220.890 20090105 GEODETIC nT  1Hz\n# Date(dd/mm/yyyy),time(hh:mi:ss),X,Y,Z,F\n";
        assert!(parse_carisma(empty_body).is_none());
    }

    #[test]
    fn component_honors_the_sentinel() {
        assert_eq!(component("0.0"), Some(0.0));
        assert_eq!(component("-115.212"), Some(-115.212));
        assert_eq!(component("99999.999"), None);
        assert_eq!(component("x"), None);
        assert_eq!(component("NaN"), None);
    }

    #[test]
    fn default_url_builds_the_measured_product_path() {
        let url = default_url("DAWS", "20090105").unwrap();
        assert_eq!(
            url,
            "https://donnees-data.asc-csa.gc.ca/users/OpenData_DonneesOuvertes/pub/CARISMA/carisma_csv/mag/daily/2009/DAWS/20090105DAWS.MAG.csv"
        );
        assert!(default_url("DAWS", "200901").is_none());
    }

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![
            [1_231_200_000.0, 11191.027, 4689.269, 56870.568],
            [1_231_200_001.0, -115.212, 0.0, 59011.588],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"CRSM").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }
}
