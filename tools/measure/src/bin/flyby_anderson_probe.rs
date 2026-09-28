use omegaflow::doppler::{ANDERSON_TOWARD_EARTH, FlybyResidual, flyby_anomaly_mm_s};

const DEFAULT_RESIDUALS: &str = "data/flyby2/anderson_residuals.tsv";
const DEFAULT_REGISTER: &str = "data/flyby2/anderson-probe-2026-09-28.json";

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

fn register_json(lines: &[FlybyResidual]) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"probe\": \"flyby_anderson_probe\",\n");
    s.push_str(&format!("  \"convention\": \"{ANDERSON_TOWARD_EARTH}\",\n"));
    s.push_str("  \"residuals\": [\n");
    for (i, l) in lines.iter().enumerate() {
        let anomaly = flyby_anomaly_mm_s(l);
        s.push_str(&format!(
            "    {{\"t_utc\": {}, \"dz_meas_mm_s\": {}, \"dz_pred_mm_s\": {}, \"anomaly_mm_s\": {}}}",
            l.t_utc,
            num_json(l.dz_meas_mm_s),
            num_json(l.dz_pred_mm_s),
            num_json(anomaly),
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

    for l in &lines {
        println!(
            "t_utc {} — meas {} mm/s, pred {} mm/s, anomaly {} mm/s",
            l.t_utc,
            num_label(l.dz_meas_mm_s),
            num_label(l.dz_pred_mm_s),
            num_label(flyby_anomaly_mm_s(l)),
        );
    }
    let n_meas = lines.iter().filter(|l| l.dz_meas_mm_s.is_some()).count();
    let n_pred = lines.iter().filter(|l| l.dz_pred_mm_s.is_some()).count();
    let n_anom = lines
        .iter()
        .filter(|l| flyby_anomaly_mm_s(l).is_some())
        .count();
    println!("count: {n_meas} measured, {n_pred} predicted, {n_anom} residual lines");

    let json = register_json(&lines);
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
