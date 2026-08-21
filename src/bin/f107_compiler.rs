// F10.7-Penticton-Historie (Atom 5 der Sonnen-Abdeckung): die Jahresdateien
// `pent_noontime-flux_1947.txt .. _<Jahr>.txt` (NCEI noontime-flux, keyless)
// tragen je Zeile `YYMMDD PENT <flux>` — sfu (10⁻²² W m⁻² Hz⁻¹), die
// Mittags-Messung (20:00 UTC, Penticton local noon). Das Jahr trägt der
// Dateiname (YYMMDD ist zweistellig). Fehlende Messungen sind Abwesenheit
// (Zeile ohne Wert, z. B. `260521 PENT         `) — Skip, nie 0.0-Fabrikation;
// sfu <= 0 ist physikalisch unmöglich und wird übersprungen.
// Epoche: Tage seit 1970-01-01 (der Kalendertag selbst) — der LSK-Pfad lässt
// prä-1972-Epochen void (erste Schaltsekunde), die Datei trägt aber den
// Kalendertag für alle Jahre 1947–heute; Schaltsekunden liegen unterhalb der
// Tagespräzision der Mittags-Messung und werden nicht hineingerechnet.
// Binär: Magie "F107", u32 count, Records (days i64 LE, flux_w_m2_hz f64 LE).

use omegaflow::archivar::fetch_raw;
use omegaflow::cdn::upload_asset;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const BASE: &str = "https://www.ngdc.noaa.gov/stp/space-weather/solar-data/solar-features/solar-radio/noontime-flux/penticton";
const MAGIC: [u8; 4] = *b"F107";
const FIRST_YEAR: i64 = 1947;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn write_bin(records: &[(i64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 16);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (d, v) in records {
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
    }
    buf
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<(i64, f64)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    if bytes.len() != 8 + n * 16 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 16;
        let d = i64::from_le_bytes(bytes[o..o + 8].try_into().ok()?);
        let v = f64::from_le_bytes(bytes[o + 8..o + 16].try_into().ok()?);
        out.push((d, v));
    }
    Some(out)
}

fn parse_line(line: &str, year: i64) -> Option<(i64, f64)> {
    let mut it = line.split_whitespace();
    let date = it.next()?;
    let station = it.next()?;
    if station != "PENT" {
        return None;
    }
    let value: f64 = it.next()?.parse().ok()?;
    if !value.is_finite() || value <= 0.0 {
        return None;
    }
    if date.len() != 6 || !date.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let month: i64 = date[2..4].parse().ok()?;
    let day: i64 = date[4..6].parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some((days, value * 1e-22))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "f107_penticton.bin".to_string());
    let now_unix = match std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
    {
        Ok(u) => u,
        Err(_) => {
            eprintln!("the clock stays unread — the harvest window is undeclared");
            std::process::exit(1);
        }
    };
    let current_year = match civil_from_days(now_unix / 86400) {
        Some((y, _, _)) => y as i64,
        None => {
            eprintln!("civil_from_days returned void — the current year stays unread");
            std::process::exit(1);
        }
    };
    let mut records: Vec<(i64, f64)> = Vec::new();
    let mut skipped_lines = 0usize;
    let mut void_years = 0usize;
    for year in FIRST_YEAR..=current_year {
        let url = format!("{}/pent_noontime-flux_{}.txt", BASE, year);
        let Some(body) = fetch_raw(&url, None, &[], 86400) else {
            eprintln!("pent_noontime-flux_{}.txt returned void", year);
            void_years += 1;
            continue;
        };
        for line in body.lines() {
            let Some(record) = parse_line(line, year) else {
                skipped_lines += 1;
                continue;
            };
            records.push(record);
        }
    }
    records.sort_by(|a, b| a.0.cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0);
    if records.is_empty() {
        eprintln!("no valid records — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            let (d0, v0) = parsed[0];
            let (d1, v1) = parsed[parsed.len() - 1];
            let span = |d: i64| -> String {
                match civil_from_days(d) {
                    Some((y, m, dd)) => format!("{}-{:02}-{:02}", y, m, dd),
                    None => format!("day {}", d),
                }
            };
            eprintln!(
                "{}: {} records ({}..{}), flux {:.0}..{:.0} sfu, {} skipped lines, {} void years — roundtrip parses ({} B)",
                out,
                parsed.len(),
                span(d0),
                span(d1),
                v1 / 1e-22,
                v0 / 1e-22,
                skipped_lines,
                void_years,
                bytes.len()
            );
        }
        None => {
            eprintln!(
                "{}: roundtrip parse void — the series stays unverified",
                out
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
