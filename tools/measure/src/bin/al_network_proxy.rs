use omegaflow::intermagnet::parse_xyz_bin;
use std::collections::BTreeMap;

const MAGIC: &[u8; 4] = b"IMXZ";

type Station = (String, Option<f64>, String, Option<(f64, f64)>);

fn in_mlt_sector(mlt: f64, min: f64, max: f64) -> bool {
    if min <= max {
        mlt >= min && mlt < max
    } else {
        mlt >= min || mlt < max
    }
}

fn write_imdt(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 16);
    out.extend_from_slice(b"IMDT");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, _) in records {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    Some(if n % 2 == 1 {
        values[n / 2]
    } else {
        0.5 * (values[n / 2 - 1] + values[n / 2])
    })
}

fn rotate_h(x: f64, y: f64, decl_deg: f64) -> f64 {
    let r = decl_deg.to_radians();
    x * r.cos() + y * r.sin()
}

fn derive_declination(samples: &[(f64, f64, f64, f64)]) -> Option<f64> {
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut n = 0usize;
    for (_, x, y, _) in samples {
        if x.is_finite() && y.is_finite() {
            sx += x;
            sy += y;
            n += 1;
        }
    }
    if n == 0 {
        return None;
    }
    Some((sy / n as f64).atan2(sx / n as f64).to_degrees())
}

fn block_median_baseline(values: &[f64], block: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity(values.len());
    let mut i = 0;
    while i < values.len() {
        let end = (i + block).min(values.len());
        let mut vals: Vec<f64> = values[i..end].to_vec();
        let Some(med) = median(&mut vals) else {
            i = end;
            continue;
        };
        for _ in i..end {
            out.push(med);
        }
        i = end;
    }
    out
}

fn station_dh(
    samples: &[(f64, f64, f64, f64)],
    decl: Option<f64>,
    block: usize,
) -> Option<Vec<(f64, f64)>> {
    let d = match decl {
        Some(v) => v,
        None => derive_declination(samples)?,
    };
    let h: Vec<f64> = samples
        .iter()
        .map(|(_, x, y, _)| rotate_h(*x, *y, d))
        .collect();
    let baseline = block_median_baseline(&h, block);
    Some(
        samples
            .iter()
            .zip(h.iter())
            .zip(baseline.iter())
            .map(|((s, hv), b)| (s.0, hv - b))
            .collect(),
    )
}

fn station_dbdt(samples: &[(f64, f64, f64, f64)]) -> Vec<(f64, f64)> {
    let mut out = Vec::with_capacity(samples.len());
    for w in samples.windows(2) {
        let (pt, px, py, pz) = w[0];
        let (t, x, y, z) = w[1];
        let dt = t - pt;
        if !(dt.is_finite() && dt > 0.0) {
            continue;
        }
        let dx = x - px;
        let dy = y - py;
        let dz = z - pz;
        if !(dx.is_finite() && dy.is_finite() && dz.is_finite()) {
            continue;
        }
        out.push((t, (dx * dx + dy * dy + dz * dz).sqrt()));
    }
    out
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Quantity {
    Dh,
    DbDt,
}

impl Quantity {
    fn word(self) -> &'static str {
        match self {
            Quantity::Dh => "dh",
            Quantity::DbDt => "dbdt",
        }
    }
}

fn quantity_of(args: &[String]) -> Result<Quantity, String> {
    match arg_value(args, "--quantity").as_deref() {
        None | Some("dh") => Ok(Quantity::Dh),
        Some("dbdt") => Ok(Quantity::DbDt),
        Some(other) => Err(format!("--quantity is neither dh nor dbdt: {other}")),
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Fusion {
    Min,
    Max,
}

impl Fusion {
    fn word(self) -> &'static str {
        match self {
            Fusion::Min => "min",
            Fusion::Max => "max",
        }
    }
}

fn fusion_of(args: &[String], quantity: Quantity) -> Result<Fusion, String> {
    match arg_value(args, "--fusion").as_deref() {
        Some("min") => Ok(Fusion::Min),
        Some("max") | Some("union") => Ok(Fusion::Max),
        None => Ok(match quantity {
            Quantity::Dh => Fusion::Min,
            Quantity::DbDt => Fusion::Max,
        }),
        Some(other) => Err(format!("--fusion is neither min, max nor union: {other}")),
    }
}

fn allow_global_min(args: &[String]) -> bool {
    args.iter().any(|a| a == "--allow-global-min")
}

fn check_global_min(quantity: Quantity, fusion: Fusion, allow: bool) -> Result<(), String> {
    if quantity == Quantity::DbDt && fusion == Fusion::Min && !allow {
        return Err(
            "the global min over an unsigned |dB/dt| magnitude selects the quietest station — a \
             veto, not the AL analogue (Rat 2026-10-15, deny global_min); the AL analogue of a \
             magnitude is max/union. Pass --allow-global-min only to reproduce the veto-net control."
                .to_string(),
        );
    }
    Ok(())
}

fn fold_envelope(acc: &mut BTreeMap<i64, (f64, u32)>, series: &[(f64, f64)], fusion: Fusion) {
    for (t, v) in series {
        if v.is_finite() {
            let e = acc.entry(*t as i64).or_insert((*v, 0));
            match fusion {
                Fusion::Min => {
                    if *v < e.0 {
                        e.0 = *v;
                    }
                }
                Fusion::Max => {
                    if *v > e.0 {
                        e.0 = *v;
                    }
                }
            }
            e.1 += 1;
        }
    }
}

fn envelope_series(acc: BTreeMap<i64, (f64, u32)>, min_stations: usize) -> Vec<(f64, f64)> {
    acc.into_iter()
        .filter(|(_, (_, c))| *c as usize >= min_stations)
        .map(|(t, (m, _))| (t as f64, m))
        .collect()
}

fn family_code(line: &str) -> Option<String> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let code = match line.strip_prefix("intermagnet_xyz_x_nt_") {
        Some(c) => c,
        None => line,
    };
    let code = code.trim().to_lowercase();
    if code.is_empty() { None } else { Some(code) }
}

fn load_family(path: &str, dir: &str) -> Vec<Station> {
    let Ok(text) = std::fs::read_to_string(path) else {
        eprintln!("al_network_proxy: {path} read void");
        std::process::exit(1);
    };
    let mut out = Vec::new();
    for line in text.lines() {
        if let Some(code) = family_code(line) {
            let bin = format!("{dir}/{code}_xyz_1m.bin");
            out.push((code, None, bin, None));
        }
    }
    out
}

fn load_manifest(path: &str) -> Vec<Station> {
    let Ok(text) = std::fs::read_to_string(path) else {
        eprintln!("al_network_proxy: {path} read void");
        std::process::exit(1);
    };
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            eprintln!(
                "al_network_proxy: manifest line needs `CODE decl|auto bin-path [geodetic_lat geodetic_lon]`: {line}"
            );
            std::process::exit(1);
        }
        let decl = if parts[1] == "auto" {
            None
        } else {
            match parts[1].parse::<f64>() {
                Ok(v) => Some(v),
                Err(_) => {
                    eprintln!(
                        "al_network_proxy: declination is neither `auto` nor a number: {}",
                        parts[1]
                    );
                    std::process::exit(1);
                }
            }
        };
        let geo = if parts.len() >= 5 {
            match (parts[3].parse::<f64>(), parts[4].parse::<f64>()) {
                (Ok(la), Ok(lo)) => Some((la, lo)),
                _ => {
                    eprintln!(
                        "al_network_proxy: manifest geodetic coordinate is not two numbers: {line}"
                    );
                    std::process::exit(1);
                }
            }
        } else {
            None
        };
        out.push((parts[0].to_string(), decl, parts[2].to_string(), geo));
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let stations = if let Some(manifest_path) = arg_value(&args, "--manifest") {
        load_manifest(&manifest_path)
    } else if let Some(family_path) = arg_value(&args, "--family") {
        let Some(dir) = arg_value(&args, "--dir") else {
            eprintln!("al_network_proxy: --family needs --dir <xyz-bin directory>");
            std::process::exit(2);
        };
        load_family(&family_path, &dir)
    } else {
        eprintln!(
            "al_network_proxy needs --manifest <file> (CODE decl|auto bin-path) or --family <file> --dir <path>"
        );
        std::process::exit(2);
    };
    let Some(block) = arg_value(&args, "--baseline-block").and_then(|v| v.parse::<usize>().ok())
    else {
        eprintln!("al_network_proxy needs --baseline-block <samples> (block-median baseline)");
        std::process::exit(2);
    };
    if block == 0 {
        eprintln!("--baseline-block must be positive");
        std::process::exit(2);
    }
    let Some(min_stations) =
        arg_value(&args, "--min-stations").and_then(|v| v.parse::<usize>().ok())
    else {
        eprintln!("al_network_proxy needs --min-stations <n> (1 = a single station's own series)");
        std::process::exit(2);
    };
    if min_stations < 1 {
        eprintln!(
            "--min-stations must be at least 1 (a 1-station envelope is that station's own series)"
        );
        std::process::exit(2);
    }
    if stations.is_empty() {
        eprintln!("al_network_proxy: manifest carries no station");
        std::process::exit(1);
    }
    let quantity = match quantity_of(&args) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("al_network_proxy: {e}");
            std::process::exit(2);
        }
    };
    let fusion = match fusion_of(&args, quantity) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("al_network_proxy: {e}");
            std::process::exit(2);
        }
    };
    if let Err(e) = check_global_min(quantity, fusion, allow_global_min(&args)) {
        eprintln!("al_network_proxy: {e}");
        std::process::exit(2);
    }
    let night_sector = args.iter().any(|a| a == "--night-sector");
    let mlt_min = arg_value(&args, "--mlt-min")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(16.0);
    let mlt_max = arg_value(&args, "--mlt-max")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(3.0);
    if night_sector {
        for (code, _, _, geo) in &stations {
            if geo.is_none() {
                eprintln!(
                    "al_network_proxy: {code} carries no geodetic coordinate — --night-sector needs the manifest's `lat lon` fields"
                );
                std::process::exit(2);
            }
        }
    }
    let mut acc: BTreeMap<i64, (f64, u32)> = BTreeMap::new();
    let mut station_count = 0usize;
    for (code, decl, path, geo) in &stations {
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("al_network_proxy: {code} {path} read void");
            std::process::exit(1);
        };
        let Some(samples) = parse_xyz_bin(&bytes) else {
            eprintln!(
                "al_network_proxy: {code} {path} is not an IMXZ bin (magic {})",
                String::from_utf8_lossy(MAGIC)
            );
            std::process::exit(1);
        };
        if samples.is_empty() {
            eprintln!("al_network_proxy: {code} {path} carries no sample");
            std::process::exit(1);
        }
        let series = match quantity {
            Quantity::Dh => {
                let Some(dh) = station_dh(&samples, *decl, block) else {
                    eprintln!(
                        "al_network_proxy: {code} cannot derive a declination (no finite sample)"
                    );
                    std::process::exit(1);
                };
                let dword = match decl {
                    Some(v) => format!("{v}"),
                    None => "auto".to_string(),
                };
                println!(
                    "station {code} decl {dword} samples {} baseline_block {block}",
                    samples.len()
                );
                dh
            }
            Quantity::DbDt => {
                println!("station {code} dbdt samples {}", samples.len());
                station_dbdt(&samples)
            }
        };
        let series: Vec<(f64, f64)> = if night_sector {
            let Some((glat, glon)) = geo else {
                eprintln!(
                    "al_network_proxy: {code} carries no geodetic coordinate — --night-sector needs the manifest's `lat lon` fields"
                );
                std::process::exit(2);
            };
            let total = series.len();
            let mut outside = 0usize;
            let gated: Vec<(f64, f64)> = series
                .into_iter()
                .filter(|(t, _)| {
                    match omegaflow::archivar::aacgm::aacgm_v2_mlt_unix(*t, *glat, *glon, 0.0) {
                        Some(mlt) => in_mlt_sector(mlt, mlt_min, mlt_max),
                        None => {
                            outside += 1;
                            false
                        }
                    }
                })
                .collect();
            if outside > 0 {
                println!(
                    "station {code} sector-gate: {outside} sample(s) outside the AACGM epochs (2010–2030) — pending, not zero"
                );
            }
            println!(
                "station {code} sector-gate mlt [{mlt_min},{mlt_max}) kept {} of {total} sample(s)",
                gated.len()
            );
            gated
        } else {
            series
        };
        fold_envelope(&mut acc, &series, fusion);
        station_count += 1;
    }
    let env = envelope_series(acc, min_stations);
    if env.is_empty() {
        eprintln!(
            "al_network_proxy: envelope empty — no timestamp carries {min_stations} station(s)"
        );
        std::process::exit(1);
    }
    println!(
        "envelope samples {} min_stations {min_stations} stations {} fusion {} quantity {}{}",
        env.len(),
        station_count,
        fusion.word(),
        quantity.word(),
        if night_sector {
            format!(" sector mlt [{mlt_min},{mlt_max})")
        } else {
            String::new()
        }
    );
    if let Some(out) = arg_value(&args, "--out") {
        let records: Vec<(f64, f64, u32)> = env.iter().map(|(t, v)| (*t, *v, 0)).collect();
        match std::fs::write(&out, write_imdt(&records)) {
            Ok(()) => println!("wrote {} envelope sample(s) → {out}", env.len()),
            Err(e) => {
                eprintln!("al_network_proxy: {out} write void: {e}");
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_at_zero_declination_is_x() {
        assert_eq!(rotate_h(3.0, 4.0, 0.0), 3.0);
    }

    #[test]
    fn rotation_at_ninety_degrees_is_y() {
        assert!((rotate_h(3.0, 4.0, 90.0) - 4.0).abs() < 1e-12);
    }

    #[test]
    fn the_block_baseline_is_the_block_median() {
        let b = block_median_baseline(&[10.0, 12.0, 14.0, 2.0], 3);
        assert_eq!(b, vec![12.0, 12.0, 12.0, 2.0]);
    }

    #[test]
    fn the_dh_series_removes_the_block_baseline() {
        let samples = vec![
            (0.0, 10.0, 0.0, 0.0),
            (60.0, 12.0, 0.0, 0.0),
            (120.0, 14.0, 0.0, 0.0),
        ];
        let dh = station_dh(&samples, Some(0.0), 3).expect("declination given");
        assert_eq!(dh, vec![(0.0, -2.0), (60.0, 0.0), (120.0, 2.0)]);
    }

    #[test]
    fn the_declination_derives_from_the_own_mean_field() {
        let samples = vec![(0.0, 1.0, 1.0, 0.0), (60.0, 1.0, 1.0, 0.0)];
        let d = derive_declination(&samples).expect("a mean field exists");
        assert!((d - 45.0).abs() < 1e-9);
    }

    #[test]
    fn auto_declination_rotates_the_mean_field_onto_north() {
        let samples = vec![(0.0, 1.0, 1.0, 0.0), (60.0, 3.0, 3.0, 0.0)];
        let dh = station_dh(&samples, None, 2).expect("declination derives");
        assert!(dh[0].1.abs() < 1e-9);
        assert!(dh[1].1.abs() < 1e-9);
    }

    #[test]
    fn the_family_line_yields_the_station_code() {
        assert_eq!(
            family_code("intermagnet_xyz_x_nt_abk"),
            Some("abk".to_string())
        );
        assert_eq!(family_code("SOD"), Some("sod".to_string()));
        assert_eq!(family_code("# comment"), None);
        assert_eq!(family_code("   "), None);
    }

    fn network_envelope(
        stations: &[Vec<(f64, f64)>],
        min_stations: usize,
        fusion: Fusion,
    ) -> Vec<(f64, f64)> {
        let mut acc: BTreeMap<i64, (f64, u32)> = BTreeMap::new();
        for s in stations {
            fold_envelope(&mut acc, s, fusion);
        }
        envelope_series(acc, min_stations)
    }

    #[test]
    fn the_lower_envelope_takes_the_minimum_and_gates_on_coverage() {
        let a = vec![(0.0, 5.0), (60.0, -3.0), (120.0, 7.0)];
        let b = vec![(0.0, 1.0), (60.0, -8.0)];
        assert_eq!(
            network_envelope(&[a, b], 2, Fusion::Min),
            vec![(0.0, 1.0), (60.0, -8.0)]
        );
    }

    #[test]
    fn the_upper_envelope_takes_the_maximum_the_non_conjunctive_union() {
        let a = vec![(0.0, 5.0), (60.0, -3.0), (120.0, 7.0)];
        let b = vec![(0.0, 1.0), (60.0, -8.0)];
        assert_eq!(
            network_envelope(&[a, b], 2, Fusion::Max),
            vec![(0.0, 5.0), (60.0, -3.0)]
        );
    }

    #[test]
    fn the_fusion_default_follows_the_quantity_and_an_unknown_one_is_refused() {
        assert_eq!(fusion_of(&[], Quantity::Dh).map(Fusion::word), Ok("min"));
        assert_eq!(fusion_of(&[], Quantity::DbDt).map(Fusion::word), Ok("max"));
        assert_eq!(
            fusion_of(&["--fusion".to_string(), "max".to_string()], Quantity::Dh).map(Fusion::word),
            Ok("max")
        );
        assert_eq!(
            fusion_of(
                &["--fusion".to_string(), "union".to_string()],
                Quantity::DbDt
            )
            .map(Fusion::word),
            Ok("max")
        );
        assert!(fusion_of(&["--fusion".to_string(), "mean".to_string()], Quantity::Dh).is_err());
    }

    #[test]
    fn the_global_min_over_a_magnitude_is_denied_until_explicitly_allowed() {
        assert!(check_global_min(Quantity::DbDt, Fusion::Min, false).is_err());
        assert!(check_global_min(Quantity::DbDt, Fusion::Min, true).is_ok());
        assert!(check_global_min(Quantity::Dh, Fusion::Min, false).is_ok());
        assert!(check_global_min(Quantity::DbDt, Fusion::Max, false).is_ok());
    }

    #[test]
    fn a_timestamp_with_one_station_is_not_an_envelope() {
        let a = vec![(0.0, 5.0), (60.0, -3.0)];
        let b = vec![(0.0, 1.0)];
        assert_eq!(network_envelope(&[a, b], 2, Fusion::Min), vec![(0.0, 1.0)]);
    }

    #[test]
    fn the_envelope_imdt_series_round_trips() {
        let env = vec![(0.0, -8.0), (60.0, -3.0)];
        let records: Vec<(f64, f64, u32)> = env.iter().map(|(t, v)| (*t, *v, 0)).collect();
        let bytes = write_imdt(&records);
        let parsed = omegaflow::intermagnet::parse_bin(&bytes).expect("parse void");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[1].1, -3.0);
    }

    #[test]
    fn the_quantity_reads_dh_and_an_unknown_one_is_refused() {
        assert_eq!(quantity_of(&[]).map(Quantity::word), Ok("dh"));
        assert_eq!(
            quantity_of(&["--quantity".to_string(), "dbdt".to_string()]).map(Quantity::word),
            Ok("dbdt")
        );
        assert!(quantity_of(&["--quantity".to_string(), "rate".to_string()]).is_err());
    }

    #[test]
    fn dbdt_is_the_per_minute_difference_magnitude() {
        let samples = vec![
            (0.0, 0.0, 0.0, 0.0),
            (60.0, 3.0, 4.0, 0.0),
            (120.0, 3.0, 4.0, 12.0),
        ];
        let d = station_dbdt(&samples);
        assert_eq!(d.len(), 2);
        assert!((d[0].1 - 5.0).abs() < 1e-12);
        assert!((d[1].1 - 12.0).abs() < 1e-12);
    }

    #[test]
    fn the_night_sector_wraps_midnight() {
        assert!(in_mlt_sector(16.0, 16.0, 3.0));
        assert!(in_mlt_sector(23.9, 16.0, 3.0));
        assert!(in_mlt_sector(2.9, 16.0, 3.0));
        assert!(!in_mlt_sector(3.0, 16.0, 3.0));
        assert!(!in_mlt_sector(12.0, 16.0, 3.0));
        assert!(in_mlt_sector(1.5, 0.0, 6.0));
        assert!(!in_mlt_sector(6.0, 0.0, 6.0));
    }
}
