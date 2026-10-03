use omegaflow::archivar::{
    BodyEphemeris, NAIF_LSK_EMBEDDED, body_barycenter_position, parse_ephemeris_binary,
};
use omegaflow::doppler::{ANDERSON_TOWARD_EARTH, FlybyResidual, flyby_anomaly_mm_s};
use omegaflow::lsk::parse as parse_lsk;
use std::collections::HashMap;

const DEFAULT_RESIDUALS: &str = "data/flyby2/anderson_residuals.tsv";
const DEFAULT_REGISTER: &str = "data/flyby2/anderson-probe-2026-09-28.json";
const DEFAULT_DE: &str = "data/ssd.jpl.nasa.gov-de/ephemeris_de441_earth.bin";
const DEFAULT_INPOP: &str = "data/ftp.imcce.fr/ephemeris_inpop_earth.bin";
const DEFAULT_EPM: &str = "data/ftp.iaaras.ru/ephemeris_epm_earth.bin";
const EARTH: &str = "earth";
const FIRST_ROW_REF_S: f64 = 86_400.0;

fn arg_str(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_or(args: &[String], flag: &str, default: &str) -> String {
    match arg_str(args, flag) {
        Some(v) => v,
        None => default.to_string(),
    }
}

fn num_label(v: Option<f64>) -> String {
    match v {
        None => "pending".to_string(),
        Some(x) => format!("{x}"),
    }
}

fn num_json(v: Option<f64>) -> String {
    match v {
        None => "\"pending\"".to_string(),
        Some(x) => format!("{x}"),
    }
}

fn residual_word(s: &str) -> Result<Option<f64>, String> {
    let t = s.trim();
    if t.is_empty() || t == "pending" || t == "absent" {
        return Ok(None);
    }
    match t.parse::<f64>() {
        Ok(v) if v.is_finite() => Ok(Some(v)),
        Ok(_) => Err(format!("{t} carries no finite f64")),
        Err(_) => Err(format!("{t} carries no f64")),
    }
}

fn parse_residuals(text: &str) -> Result<Vec<FlybyResidual>, String> {
    let mut out = Vec::new();
    for (li, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() != 3 {
            return Err(format!(
                "line {} carries {} columns, not 3",
                li + 1,
                cols.len()
            ));
        }
        let t_utc = cols[0]
            .parse::<f64>()
            .map_err(|_| format!("line {} t_utc {} carries no f64", li + 1, cols[0]))?;
        if !t_utc.is_finite() {
            return Err(format!(
                "line {} t_utc {} carries no finite f64",
                li + 1,
                cols[0]
            ));
        }
        let dz_meas_mm_s = residual_word(cols[1])
            .map_err(|e| format!("line {} dz_meas {}: {e}", li + 1, cols[1]))?;
        let dz_pred_mm_s = residual_word(cols[2])
            .map_err(|e| format!("line {} dz_pred {}: {e}", li + 1, cols[2]))?;
        out.push(FlybyResidual {
            t_utc,
            dz_meas_mm_s,
            dz_pred_mm_s,
        });
    }
    Ok(out)
}

fn load_house(path: &str) -> Option<HashMap<String, BodyEphemeris>> {
    let bytes = std::fs::read(path).ok()?;
    let e = parse_ephemeris_binary(&bytes)?;
    let mut map = HashMap::new();
    map.insert(EARTH.to_string(), e);
    Some(map)
}

fn pair_delta_km(
    a: &HashMap<String, BodyEphemeris>,
    b: &HashMap<String, BodyEphemeris>,
    t_tdb: f64,
) -> Option<f64> {
    let pa = body_barycenter_position(EARTH, t_tdb, a)?;
    let pb = body_barycenter_position(EARTH, t_tdb, b)?;
    let dx = pa[0] - pb[0];
    let dy = pa[1] - pb[1];
    let dz = pa[2] - pb[2];
    let d = (dx * dx + dy * dy + dz * dz).sqrt() / 1000.0;
    if d.is_finite() { Some(d) } else { None }
}

fn pair_slope_mm_s(offset_prev_km: f64, offset_curr_km: f64, dt_s: f64) -> Option<f64> {
    if !(dt_s > 0.0) {
        return None;
    }
    let slope = (offset_curr_km - offset_prev_km).abs() * 1.0e6 / dt_s;
    if slope.is_finite() { Some(slope) } else { None }
}

struct HouseLine {
    de_inpop_km: Option<f64>,
    de_epm_km: Option<f64>,
    inpop_epm_km: Option<f64>,
    tdot_max_mm_s: Option<f64>,
    verdict: &'static str,
}

fn house_line(
    de: &Option<HashMap<String, BodyEphemeris>>,
    inpop: &Option<HashMap<String, BodyEphemeris>>,
    epm: &Option<HashMap<String, BodyEphemeris>>,
    t_tdb: Option<f64>,
    anomaly_mm_s: Option<f64>,
    prev: Option<&HouseLine>,
    dt_prev_s: Option<f64>,
) -> HouseLine {
    let (de_inpop, de_epm, inpop_epm) = match (de, inpop, epm, t_tdb) {
        (Some(d), Some(i), Some(e), Some(t)) => (
            pair_delta_km(d, i, t),
            pair_delta_km(d, e, t),
            pair_delta_km(i, e, t),
        ),
        _ => (None, None, None),
    };
    let mut tdot_max: Option<f64> = None;
    if let (Some(p), Some(dt)) = (prev, dt_prev_s) {
        let mut m = 0.0f64;
        let mut seen = false;
        for (cur, old) in [
            (de_inpop, p.de_inpop_km),
            (de_epm, p.de_epm_km),
            (inpop_epm, p.inpop_epm_km),
        ] {
            if let (Some(c), Some(o)) = (cur, old) {
                if let Some(slope) = pair_slope_mm_s(o, c, dt) {
                    if slope > m {
                        m = slope;
                    }
                    seen = true;
                }
            }
        }
        if seen {
            tdot_max = Some(m);
        }
    }
    let verdict = match (tdot_max, anomaly_mm_s) {
        (Some(bound), Some(a)) if bound < a.abs() => "rift-excluded",
        (Some(_), Some(_)) => "rift-generator",
        _ => "pending",
    };
    HouseLine {
        de_inpop_km: de_inpop,
        de_epm_km: de_epm,
        inpop_epm_km: inpop_epm,
        tdot_max_mm_s: tdot_max,
        verdict,
    }
}

fn register_json(lines: &[FlybyResidual], house: &[HouseLine]) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"probe\": \"flyby_anderson_probe\",\n");
    s.push_str(&format!("  \"convention\": \"{ANDERSON_TOWARD_EARTH}\",\n"));
    s.push_str(&format!("  \"first_row_ref_s\": {FIRST_ROW_REF_S},\n"));
    s.push_str(
        "  \"epoch_basis\": \"row t_utc -> TDB via NAIF LSK; pair deltas at that single instant\",\n",
    );
    s.push_str(
        "  \"tdot_max_scope\": \"max over de_inpop/de_epm/inpop_epm pair slopes over the inter-row TDB delta; row 0 over first_row_ref_s\",\n",
    );
    s.push_str("  \"residuals\": [\n");
    for (i, l) in lines.iter().enumerate() {
        let anomaly = flyby_anomaly_mm_s(l);
        let h = &house[i];
        s.push_str(&format!(
            "    {{\"t_utc\": {}, \"dz_meas_mm_s\": {}, \"dz_pred_mm_s\": {}, \"anomaly_mm_s\": {}, \
             \"de_inpop_km\": {}, \"de_epm_km\": {}, \"inpop_epm_km\": {}, \
             \"tdot_max_mm_s\": {}, \"verdict\": \"{}\"}}",
            l.t_utc,
            num_json(l.dz_meas_mm_s),
            num_json(l.dz_pred_mm_s),
            num_json(anomaly),
            num_json(h.de_inpop_km),
            num_json(h.de_epm_km),
            num_json(h.inpop_epm_km),
            num_json(h.tdot_max_mm_s),
            h.verdict,
        ));
        if i + 1 < lines.len() {
            s.push(',');
        }
        s.push('\n');
    }
    s.push_str("  ]\n");
    s.push_str("}\n");
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let residuals_path = arg_or(&args, "--residuals", DEFAULT_RESIDUALS);
    let register_path = arg_or(&args, "--register", DEFAULT_REGISTER);
    let de_path = arg_or(&args, "--de", DEFAULT_DE);
    let inpop_path = arg_or(&args, "--inpop", DEFAULT_INPOP);
    let epm_path = arg_or(&args, "--epm", DEFAULT_EPM);

    println!("flyby_anderson_probe — Anderson flyby residual, convention {ANDERSON_TOWARD_EARTH}");

    let lines: Vec<FlybyResidual> = match std::fs::read_to_string(&residuals_path) {
        Ok(text) => match parse_residuals(&text) {
            Ok(v) => {
                println!("residuals: {residuals_path} — {} lines", v.len());
                v
            }
            Err(e) => {
                eprintln!("flyby_anderson_probe: {e}");
                std::process::exit(2);
            }
        },
        Err(_) => {
            println!("residuals: absent ({residuals_path}) — the table stays pending");
            Vec::new()
        }
    };

    let de = load_house(&de_path);
    let inpop = load_house(&inpop_path);
    let epm = load_house(&epm_path);
    let lsk = parse_lsk(NAIF_LSK_EMBEDDED);

    let mut house: Vec<HouseLine> = Vec::with_capacity(lines.len());
    let mut t_prev: Option<f64> = None;
    for (i, l) in lines.iter().enumerate() {
        let t_tdb = lsk.as_ref().and_then(|lsk| lsk.unix_to_tdb(l.t_utc));
        let anomaly = flyby_anomaly_mm_s(l);
        let h = if i == 0 {
            match t_tdb {
                Some(t) => {
                    let reference = house_line(
                        &de,
                        &inpop,
                        &epm,
                        Some(t - FIRST_ROW_REF_S),
                        None,
                        None,
                        None,
                    );
                    house_line(
                        &de,
                        &inpop,
                        &epm,
                        t_tdb,
                        anomaly,
                        Some(&reference),
                        Some(FIRST_ROW_REF_S),
                    )
                }
                None => house_line(&de, &inpop, &epm, t_tdb, anomaly, None, None),
            }
        } else {
            let prev = house.last();
            let dt_prev = match (t_tdb, t_prev) {
                (Some(t), Some(tp)) if t > tp => Some(t - tp),
                _ => None,
            };
            house_line(&de, &inpop, &epm, t_tdb, anomaly, prev, dt_prev)
        };
        println!(
            "t_utc {} — meas {} mm/s, pred {} mm/s, anomaly {} mm/s | tdot_max {} mm/s | {}",
            l.t_utc,
            num_label(l.dz_meas_mm_s),
            num_label(l.dz_pred_mm_s),
            num_label(anomaly),
            num_label(h.tdot_max_mm_s),
            h.verdict,
        );
        if t_tdb.is_some() {
            t_prev = t_tdb;
        }
        house.push(h);
    }

    let n_meas = lines.iter().filter(|l| l.dz_meas_mm_s.is_some()).count();
    let n_pred = lines.iter().filter(|l| l.dz_pred_mm_s.is_some()).count();
    let n_anom = lines
        .iter()
        .filter(|l| flyby_anomaly_mm_s(l).is_some())
        .count();
    println!("count: {n_meas} measured, {n_pred} predicted, {n_anom} residual lines");

    let json = register_json(&lines, &house);
    if let Some(parent) = std::path::Path::new(&register_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    match std::fs::write(&register_path, &json) {
        Ok(_) => println!("register: {register_path}"),
        Err(e) => {
            eprintln!("register write void ({register_path}): {e}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_slope_reads_the_change_not_the_offset() {
        assert_eq!(pair_slope_mm_s(16.0, 16.0, 3600.0), Some(0.0));
        assert_eq!(pair_slope_mm_s(0.0, 0.0, 3600.0), Some(0.0));
        let s = pair_slope_mm_s(1.0, 1.001, 3600.0).unwrap();
        assert!((s - 1.0e6 * 0.001 / 3600.0).abs() < 1e-9);
        assert_eq!(pair_slope_mm_s(1.0, 2.0, 0.0), None);
    }
}
