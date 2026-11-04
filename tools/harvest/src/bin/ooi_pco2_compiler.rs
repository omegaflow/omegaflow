use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GeoRec, MAGIC_OOI, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::spectral::SPECTRAL_NO_BAND;

const NETLOC: &str = "erddap.dataexplorer.oceanobservatories.org";
const DEFAULT_URL: &str = "https://erddap.dataexplorer.oceanobservatories.org/erddap/tabledap/ooi-ga01sumo-sbd12-04-pco2aa000.csv?time,sea_surface_temperature,surface_partial_pressure_of_carbon_dioxide_in_sea_water";
const STATION_LAT: f64 = 46.05;
const STATION_LON: f64 = -125.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

/// ERDDAP CSV: a name row, a unit row, then data. The time column is ISO 8601.
fn parse_csv(text: &str) -> Vec<GeoRec> {
    let mut header: Option<Vec<String>> = None;
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let cols: Vec<&str> = t.split(',').collect();
        if header.is_none() {
            header = Some(cols.iter().map(|c| c.trim().to_string()).collect());
            continue;
        }
        let Some(names) = header.as_ref() else {
            continue;
        };
        // the unit row carries the same width and no parseable timestamp
        let Some(ti) = names.iter().position(|n| n == "time") else {
            continue;
        };
        let Some(time_s) = cols.get(ti).and_then(|v| parse_time(v.trim())) else {
            continue;
        };
        let Some(pi) = names
            .iter()
            .position(|n| n == "surface_partial_pressure_of_carbon_dioxide_in_sea_water")
        else {
            continue;
        };
        let Some(pco2) = cols.get(pi).and_then(|v| v.trim().parse::<f64>().ok()) else {
            continue;
        };
        if !pco2.is_finite() || pco2 <= 0.0 {
            continue;
        }
        out.push(GeoRec {
            t: time_s,
            lat: STATION_LAT,
            lon: STATION_LON,
            alt: 0.0,
            freq: SPECTRAL_NO_BAND,
            bin_width: 3600.0,
            val: pco2,
            comp: 0,
            station: 0,
        });
        if let Some(si) = names.iter().position(|n| n == "sea_surface_temperature") {
            if let Some(sst) = cols.get(si).and_then(|v| v.trim().parse::<f64>().ok()) {
                if sst.is_finite() {
                    out.push(GeoRec {
                        t: time_s,
                        lat: STATION_LAT,
                        lon: STATION_LON,
                        alt: 0.0,
                        freq: SPECTRAL_NO_BAND,
                        bin_width: 3600.0,
                        val: sst,
                        comp: 1,
                        station: 0,
                    });
                }
            }
        }
    }
    out
}

fn parse_time(s: &str) -> Option<f64> {
    // 2024-01-01T00:00:00Z
    if s.len() < 19 {
        return None;
    }
    let y: i64 = s.get(0..4)?.parse().ok()?;
    let mo: i64 = s.get(5..7)?.parse().ok()?;
    let d: i64 = s.get(8..10)?.parse().ok()?;
    let h: i64 = s.get(11..13)?.parse().ok()?;
    let mi: i64 = s.get(14..16)?.parse().ok()?;
    let se: i64 = s.get(17..19)?.parse().ok()?;
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) || h > 23 || mi > 59 || se > 60 {
        return None;
    }
    let days = omegaflow::lsk::days_from_civil(y, mo, d)?;
    Some(days as f64 * 86400.0 + (h * 3600 + mi * 60 + se) as f64)
}

fn selftest() {
    let csv = "time,sea_surface_temperature,surface_partial_pressure_of_carbon_dioxide_in_sea_water\nUTC,degree_Celsius,uatm\n2024-01-01T00:00:00Z,10.5,410.2\n2024-01-01T01:00:00Z,,411.0\n";
    let recs = parse_csv(csv);
    assert_eq!(recs.len(), 3, "two pCO2 samples and one SST sample");
    assert_eq!(recs[0].comp, 0);
    assert_eq!(recs[0].val, 410.2);
    assert_eq!(recs[1].comp, 1);
    assert_eq!(recs[1].val, 10.5);
    assert_eq!(recs[2].comp, 0);
    assert_eq!(recs[2].val, 411.0);
    println!("ooi_pco2_compiler selftest: 3 samples, missing temperature skipped");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    omegaflow::archivar::cdn::require_ci();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = arg_value(&args, "--url").unwrap_or_else(|| DEFAULT_URL.to_string());
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "ooi_pco2.bin".to_string(),
    };
    let Some(body) = fetch_raw_bytes(&url) else {
        eprintln!("ooi_pco2_compiler: fetch void ({url})");
        std::process::exit(1);
    };
    let text = String::from_utf8_lossy(&body);
    let records = parse_csv(&text);
    if records.is_empty() {
        eprintln!("ooi_pco2_compiler: the table carries no pCO2 sample — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(MAGIC_OOI, &records);
    std::fs::write(&out, &bytes).unwrap_or_else(|e| {
        eprintln!("ooi_pco2_compiler: write {out}: {e}");
        std::process::exit(1);
    });
    eprintln!(
        "ooi_pco2_compiler: {} samples, {} bytes",
        records.len(),
        bytes.len()
    );
    if ci_mode {
        upload_release(NETLOC, &out);
    }
}
