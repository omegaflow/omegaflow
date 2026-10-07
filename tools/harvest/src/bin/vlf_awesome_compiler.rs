use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mat4::{Mat4Data, Mat4Var, parse_mat4};
use omegaflow::cdn::upload_release;
use std::io::Write;

const NETLOC: &str = "craam-files-bucket.s3.amazonaws.com";
const SAMPLE_URL: &str = "https://craam-files-bucket.s3.amazonaws.com/2015/01/01/narrowband/FE/FE150101000500NAA_008A.mat";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn ascii(vars: &[Mat4Var], name: &str) -> Option<String> {
    let var = vars.iter().find(|v| v.name == name)?;
    match &var.data {
        Mat4Data::UInt8(b) => {
            let raw = b.split(|&c| c == 0).next()?;
            Some(String::from_utf8_lossy(raw).trim().to_string())
        }
        _ => None,
    }
}

fn i16_scalar(vars: &[Mat4Var], name: &str) -> Option<i16> {
    let var = vars.iter().find(|v| v.name == name)?;
    match &var.data {
        Mat4Data::Int16(v) => v.first().copied(),
        _ => None,
    }
}

fn f32_series<'a>(vars: &'a [Mat4Var], name: &str) -> Option<&'a [f32]> {
    let var = vars.iter().find(|v| v.name == name)?;
    match &var.data {
        Mat4Data::Single(v) => Some(v.as_slice()),
        _ => None,
    }
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn unix_time(vars: &[Mat4Var]) -> Option<i64> {
    let year = i16_scalar(vars, "start_year")? as i64;
    let month = i16_scalar(vars, "start_month")? as i64;
    let day = i16_scalar(vars, "start_day")? as i64;
    let hour = i16_scalar(vars, "start_hour")? as i64;
    let minute = i16_scalar(vars, "start_minute")? as i64;
    let second = i16_scalar(vars, "start_second")? as i64;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some(days_from_civil(year, month, day) * 86400 + hour * 3600 + minute * 60 + second)
}

fn channel_label(vars: &[Mat4Var]) -> Option<String> {
    match i16_scalar(vars, "adc_channel_number") {
        Some(0) => Some("N/S".to_string()),
        Some(1) => Some("E/W".to_string()),
        _ => None,
    }
}

fn text_cell(v: Option<String>) -> String {
    match v {
        Some(s) if !s.is_empty() => s,
        _ => String::new(),
    }
}

fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn num_cell(v: Option<i64>) -> String {
    match v {
        Some(x) => format!("{x}"),
        None => String::new(),
    }
}

fn asset_name(vars: &[Mat4Var]) -> Option<String> {
    let station = ascii(vars, "station_name")?;
    let call = ascii(vars, "call_sign")?;
    let year = i16_scalar(vars, "start_year")?;
    let month = i16_scalar(vars, "start_month")?;
    let day = i16_scalar(vars, "start_day")?;
    let hour = i16_scalar(vars, "start_hour")?;
    let minute = i16_scalar(vars, "start_minute")?;
    let safe = |s: &str| -> String {
        s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
    };
    let station = safe(&station);
    let call = safe(&call);
    if station.is_empty() || call.is_empty() {
        return None;
    }
    Some(format!(
        "vlf_awesome_{station}_{year:04}{month:02}{day:02}{hour:02}{minute:02}_{call}.csv"
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => SAMPLE_URL.to_string(),
    };
    let out_arg = arg_value(&args, "--out");

    let bytes = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {} returned void: {}", path, e);
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("fetch {} returned void", url);
                std::process::exit(1);
            }
        },
    };

    let Some(vars) = parse_mat4(&bytes) else {
        eprintln!(
            "{} ({} byte(s)) carries no complete MATLAB Level-4 variable table — the asset stays unwritten (0 honored)",
            url,
            bytes.len()
        );
        std::process::exit(1);
    };
    let Some(series) = f32_series(&vars, "data") else {
        eprintln!("{url} carries no float32 data variable — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(asset) = asset_name(&vars) else {
        eprintln!(
            "{url} carries no station/transmitter/time header — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    };

    let out_path = match out_arg {
        Some(p) => p,
        None => format!("data/{NETLOC}/vlf_awesome/{asset}"),
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut file = match std::fs::File::create(&out_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create {} returned void: {}", out_path, e);
            std::process::exit(1);
        }
    };

    let station = text_cell(ascii(&vars, "station_name"));
    let call = text_cell(ascii(&vars, "call_sign"));
    let channel = text_cell(channel_label(&vars));
    let latitude = text_cell(ascii(&vars, "latitude"));
    let longitude = text_cell(ascii(&vars, "longitude"));
    let altitude = text_cell(ascii(&vars, "altitude"));
    let unix = num_cell(unix_time(&vars));

    let header = "unix,amplitude_db,station,transmitter,channel,latitude,longitude,altitude\n";
    if file.write_all(header.as_bytes()).is_err() {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    let meta = format!(
        "{},{},{},{},{},{}",
        csv_field(&station),
        csv_field(&call),
        csv_field(&channel),
        csv_field(&latitude),
        csv_field(&longitude),
        csv_field(&altitude)
    );
    let mut records: u64 = 0;
    for amplitude in series {
        let row = format!("{unix},{amplitude},{meta}\n");
        if file.write_all(row.as_bytes()).is_err() {
            eprintln!("write {} returned void", out_path);
            std::process::exit(1);
        }
        records += 1;
    }
    if file.flush().is_err() {
        eprintln!("flush {} returned void", out_path);
        std::process::exit(1);
    }
    eprintln!(
        "vlf_awesome compiled {} sample(s) ({station}{call}) -> {out_path}",
        records
    );
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("upload_release for {} returned void", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VLF: &[u8] =
        include_bytes!("../../../../src/archivar/mat4_fixtures/vlf_awesome_narrowband_A.mat");

    #[test]
    fn reads_station_transmitter_channel_and_time() {
        let vars = parse_mat4(VLF).expect("fixture parses");
        assert_eq!(ascii(&vars, "station_name").as_deref(), Some("EACF"));
        assert_eq!(ascii(&vars, "call_sign").as_deref(), Some("NAA"));
        assert_eq!(channel_label(&vars).as_deref(), Some("N/S"));
        assert_eq!(unix_time(&vars), Some(1420070700));
        assert_eq!(
            asset_name(&vars).as_deref(),
            Some("vlf_awesome_EACF_201501010005_NAA.csv")
        );
        let series = f32_series(&vars, "data").expect("data series");
        assert_eq!(series.len(), 85800);
        assert_eq!(series[0], f32::from_bits(0x4204_6B42));
    }
}
