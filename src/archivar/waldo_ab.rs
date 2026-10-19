use crate::archivar::mat4::{Mat4Data, Mat4Var, parse_mat4};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderOrder {
    Old,
    Awesome,
}

impl HeaderOrder {
    pub fn label(self) -> &'static str {
        match self {
            HeaderOrder::Old => "A",
            HeaderOrder::Awesome => "B",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaldoMeta {
    pub order: HeaderOrder,
    pub station: String,
    pub call_sign: String,
    pub frequency_hz: Option<f64>,
    pub sample_rate_hz: Option<f64>,
    pub start_unix: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaldoSeries {
    pub meta: WaldoMeta,
    pub samples: Vec<f32>,
}

pub const MIN_VARIABLES: usize = 9;
pub const MIN_SAMPLES: usize = 2;

fn var<'a>(vars: &'a [Mat4Var], name: &str) -> Option<&'a Mat4Var> {
    vars.iter().find(|v| v.name == name)
}

fn text_of(vars: &[Mat4Var], name: &str) -> Option<String> {
    let var = var(vars, name)?;
    match &var.data {
        Mat4Data::UInt8(bytes) => {
            let raw = bytes.split(|&b| b == 0).next()?;
            Some(String::from_utf8_lossy(raw).trim().to_string())
        }
        _ => None,
    }
}

fn number_of(vars: &[Mat4Var], name: &str) -> Option<f64> {
    let var = var(vars, name)?;
    let value = match &var.data {
        Mat4Data::Double(v) => v.first().copied()?,
        Mat4Data::Single(v) => f64::from(*v.first()?),
        Mat4Data::Int32(v) => f64::from(*v.first()?),
        Mat4Data::Int16(v) => f64::from(*v.first()?),
        Mat4Data::UInt16(v) => f64::from(*v.first()?),
        Mat4Data::UInt8(v) => f64::from(*v.first()?),
    };
    value.is_finite().then_some(value)
}

pub fn header_order(vars: &[Mat4Var]) -> Option<HeaderOrder> {
    match vars.first()?.name.as_str() {
        "start_year" => Some(HeaderOrder::Old),
        "VERSION" => Some(HeaderOrder::Awesome),
        _ => None,
    }
}

fn series_of(vars: &[Mat4Var]) -> Option<Vec<f32>> {
    let var = var(vars, "data")?;
    let samples: Vec<f32> = match &var.data {
        Mat4Data::Single(v) => v.clone(),
        Mat4Data::Double(v) => v.iter().map(|x| *x as f32).collect(),
        _ => return None,
    };
    if samples.len() < MIN_SAMPLES || samples.iter().any(|x| !x.is_finite()) {
        return None;
    }
    Some(samples)
}

pub fn decode(bytes: &[u8]) -> Result<WaldoSeries, String> {
    let Some(vars) = parse_mat4(bytes) else {
        return Err("carries no complete MATLAB Level-4 variable table".to_string());
    };
    if vars.len() < MIN_VARIABLES {
        return Err(format!(
            "carries {} variable(s), below the {MIN_VARIABLES} the WALDO header needs",
            vars.len()
        ));
    }
    let Some(order) = header_order(&vars) else {
        return Err(
            "neither header order A (start_year first) nor B (VERSION first) is present"
                .to_string(),
        );
    };
    let Some(samples) = series_of(&vars) else {
        return Err(format!(
            "carries no finite float series `data` with at least {MIN_SAMPLES} samples"
        ));
    };
    let Some(call_sign) = text_of(&vars, "call_sign").filter(|s| !s.is_empty()) else {
        return Err("carries no call_sign".to_string());
    };
    let station = match text_of(&vars, "station_name") {
        Some(s) if !s.is_empty() => s,
        _ => "absent".to_string(),
    };
    let mut parts = [0i64; 6];
    for (slot, name) in parts.iter_mut().zip([
        "start_year",
        "start_month",
        "start_day",
        "start_hour",
        "start_minute",
        "start_second",
    ]) {
        let Some(value) = number_of(&vars, name) else {
            return Err(format!("carries no numeric {name}"));
        };
        *slot = value as i64;
    }
    let [year, month, day, hour, minute, second] = parts;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(format!("start date {year}-{month}-{day} is out of range"));
    }
    let Some(days) = crate::archivar::lsk::days_from_civil(year, month, day) else {
        return Err(format!("start date {year}-{month}-{day} does not resolve"));
    };
    let start_unix = days * 86400 + hour * 3600 + minute * 60 + second;
    Ok(WaldoSeries {
        meta: WaldoMeta {
            order,
            station,
            call_sign,
            frequency_hz: number_of(&vars, "Fc"),
            sample_rate_hz: number_of(&vars, "Fs").filter(|f| *f > 0.0),
            start_unix,
        },
        samples,
    })
}

fn csv_cell(value: &str) -> String {
    if value.contains(',') || value.contains('"') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub fn to_csv(series: &WaldoSeries) -> String {
    let mut out = String::from(
        "unix,offset_s,amplitude,sample_rate_hz,frequency_hz,call_sign,station,header_order\n",
    );
    let fs_cell = match series.meta.sample_rate_hz {
        Some(v) => v.to_string(),
        None => String::from("absent"),
    };
    let fc = match series.meta.frequency_hz {
        Some(v) => v.to_string(),
        None => String::from("absent"),
    };
    let call = csv_cell(&series.meta.call_sign);
    let station = csv_cell(&series.meta.station);
    for (i, amplitude) in series.samples.iter().enumerate() {
        let (unix, offset) = match series.meta.sample_rate_hz {
            Some(fs) => (
                (series.meta.start_unix + (i as f64 / fs).floor() as i64).to_string(),
                (i as f64 / fs).to_string(),
            ),
            None => (String::from("absent"), String::from("absent")),
        };
        out.push_str(&format!(
            "{unix},{offset},{amplitude},{fs_cell},{fc},{call},{station},{}\n",
            series.meta.order.label()
        ));
    }
    out
}
