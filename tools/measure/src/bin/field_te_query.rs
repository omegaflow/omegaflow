use std::env;
use std::process::exit;

use omegaflow::archivar::witness::WitnessKind;
use omegaflow::archivar::{
    Extract, FieldConfig, SourceConfig, embedded_lsk, extract_series, fetch_raw_bytes_headers,
    geo_series_component_name, geo_series_parse_bin, load_sources, series_component_name,
    series_rows,
};
use omegaflow::lsk::days_from_civil;
use omegaflow::mathematikerin::wy_max_t::{Member, observed_family};
use omegaflow::te::{
    conditional_embedded_te_phase, surrogate_max_phase_n, surrogate_stats_phase_n,
};

const MONTH_S: f64 = 2_592_000.0;
const CAL_MONTHS: usize = 12;
const CLIMATOLOGY_FLOOR: usize = 10;
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const TE_FLOOR: usize = 8;

const REC_FAM: f64 = 2.9610e-1;
const REC_D2T_WORD: &str = "silent";
const REC_D2T_LAG: usize = 8;
const REC_D2T_TE: f64 = 2.0700e-1;
const REC_T2D_WORD: &str = "silent";
const REC_T2D_LAG: usize = 9;
const REC_T2D_TE: f64 = 2.2693e-1;
const REC_CTE: f64 = 8.3587e-3;
const REC_CTE_THR: f64 = 2.6118e-2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Built,
    Pending,
    Probe,
}

impl State {
    fn parse(token: &str) -> Option<State> {
        match token {
            "built" => Some(State::Built),
            "pending" => Some(State::Pending),
            "probe" => Some(State::Probe),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            State::Built => "built",
            State::Pending => "pending",
            State::Probe => "probe",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Seasonal {
    None,
    Climatology,
}

impl Seasonal {
    fn name(self) -> &'static str {
        match self {
            Seasonal::None => "none",
            Seasonal::Climatology => "climatology+standardize",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Register {
    Sources,
    Witnesses,
}

#[derive(Clone)]
struct WitnessRecord {
    kind: Option<WitnessKind>,
    kind_token: String,
    key: String,
    url: String,
    records: Vec<String>,
    force: Option<String>,
}

#[derive(Clone)]
struct Arm {
    name: String,
    state: State,
}

struct Descriptor {
    pair: Option<String>,
    driver: Arm,
    target: Arm,
    cond: Option<Arm>,
    events: Vec<(String, String)>,
    gates: Vec<(String, String)>,
    seasonal: Seasonal,
    lags: Vec<usize>,
    surrogate: usize,
    register: Register,
    bin: Option<f64>,
}

fn default_lags() -> Vec<usize> {
    vec![0, 1, 3, 6, 12]
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn parse_lags(token: &str) -> Result<Vec<usize>, String> {
    let mut out = Vec::new();
    for part in token.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let lag: usize = part
            .parse()
            .map_err(|_| format!("lags token '{part}' carries no index"))?;
        out.push(lag);
    }
    if out.is_empty() {
        return Err("lags carries no index".into());
    }
    Ok(out)
}

fn parse_descriptor(text: &str) -> Result<Descriptor, String> {
    let mut pair = None;
    let mut driver: Option<Arm> = None;
    let mut target: Option<Arm> = None;
    let mut cond: Option<Arm> = None;
    let mut events = Vec::new();
    let mut gates = Vec::new();
    let mut cadence = false;
    let mut seasonal = Seasonal::None;
    let mut lags: Option<Vec<usize>> = None;
    let mut surrogate: Option<usize> = None;
    let mut register = Register::Sources;
    let mut witness_primary = false;
    let mut bin: Option<f64> = None;

    for (lineno, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let Some(head) = parts.first().copied() else {
            continue;
        };
        let at = lineno + 1;
        match head {
            "pair" => {
                let name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: pair carries no label"))?;
                pair = Some(name.to_string());
            }
            "driver" | "target" | "cond" => {
                let name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: {head} carries no field name"))?;
                let state = match parts.get(2).copied() {
                    Some(t) => State::parse(t).ok_or_else(|| {
                        format!("descriptor:{at}: {head} state '{t}' names no built|pending|probe")
                    })?,
                    None => State::Built,
                };
                if head == "driver" && witness_primary {
                    return Err(format!(
                        "descriptor:{at}: driver and witness name one primary arm — one per round"
                    ));
                }
                let arm = Arm {
                    name: name.to_string(),
                    state,
                };
                match head {
                    "driver" => driver = Some(arm),
                    "target" => target = Some(arm),
                    _ => cond = Some(arm),
                }
            }
            "witness" => {
                let name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: witness carries no name"))?;
                let state = match parts.get(2).copied() {
                    Some(t) => State::parse(t).ok_or_else(|| {
                        format!("descriptor:{at}: witness state '{t}' names no built|pending|probe")
                    })?,
                    None => State::Built,
                };
                if driver.is_some() || witness_primary {
                    return Err(format!(
                        "descriptor:{at}: witness and driver name one primary arm — one per round"
                    ));
                }
                witness_primary = true;
                register = Register::Witnesses;
                driver = Some(Arm {
                    name: name.to_string(),
                    state,
                });
            }
            "register" => {
                let token = parts
                    .get(1)
                    .copied()
                    .ok_or_else(|| format!("descriptor:{at}: register carries no token"))?;
                register = match token {
                    "sources" => Register::Sources,
                    "witnesses" => Register::Witnesses,
                    other => {
                        return Err(format!(
                            "descriptor:{at}: register '{other}' names no sources|witnesses"
                        ));
                    }
                };
            }
            "event" | "gate" => {
                let name = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: {head} carries no ref"))?;
                let state = parts.get(2).copied().ok_or_else(|| {
                    format!("descriptor:{at}: {head} needs a pending|probe state")
                })?;
                if !matches!(state, "pending" | "probe") {
                    return Err(format!(
                        "descriptor:{at}: {head} state '{state}' is no pending|probe ref (never a silent number)"
                    ));
                }
                let entry = (name.to_string(), state.to_string());
                if head == "event" {
                    events.push(entry);
                } else {
                    gates.push(entry);
                }
            }
            "cadence" => {
                let token = parts
                    .get(1)
                    .copied()
                    .ok_or_else(|| format!("descriptor:{at}: cadence carries no value"))?;
                if token != "live" {
                    return Err(format!(
                        "descriptor:{at}: cadence '{token}' is no live measurement — the cadence is measured, never defaulted"
                    ));
                }
                cadence = true;
            }
            "seasonal" => {
                let token = parts
                    .get(1)
                    .copied()
                    .ok_or_else(|| format!("descriptor:{at}: seasonal carries no mode"))?;
                seasonal = match token {
                    "none" => Seasonal::None,
                    "climatology+standardize" => Seasonal::Climatology,
                    _ => {
                        return Err(format!(
                            "descriptor:{at}: seasonal '{token}' names no none|climatology+standardize"
                        ));
                    }
                };
            }
            "lags" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: lags carries no list"))?;
                lags = Some(parse_lags(token)?);
            }
            "surrogate" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: surrogate carries no count"))?;
                let n: usize = token.parse().map_err(|_| {
                    format!("descriptor:{at}: surrogate '{token}' carries no count")
                })?;
                surrogate = Some(n);
            }
            "bin" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: bin carries no width"))?;
                let seconds: f64 = token.parse().map_err(|_| {
                    format!("descriptor:{at}: bin '{token}' carries no second count")
                })?;
                if !(seconds.is_finite() && seconds > 0.0) {
                    return Err(format!(
                        "descriptor:{at}: bin '{token}' is no positive finite width"
                    ));
                }
                bin = Some(seconds);
            }
            other => {
                return Err(format!("descriptor:{at}: unknown directive '{other}'"));
            }
        }
    }

    if !cadence {
        return Err(
            "cadence absent — the cadence is measured from the aligned grid, never defaulted"
                .into(),
        );
    }
    let driver = driver.ok_or_else(|| "descriptor carries no driver arm".to_string())?;
    let target = target.ok_or_else(|| "descriptor carries no target arm".to_string())?;
    Ok(Descriptor {
        pair,
        driver,
        target,
        cond,
        events,
        gates,
        seasonal,
        lags: match lags {
            Some(l) => l,
            None => default_lags(),
        },
        surrogate: match surrogate {
            Some(s) => s,
            None => 100,
        },
        register,
        bin,
    })
}

fn descriptor_from_args(args: &[String]) -> Result<Descriptor, String> {
    let driver = arg_after(args, "--driver").ok_or("--driver absent")?;
    let target = arg_after(args, "--target")
        .or(arg_after(args, "--field"))
        .ok_or("--target/--field absent")?;
    let cond = arg_after(args, "--cond").map(|n| Arm {
        name: n.to_string(),
        state: State::Built,
    });
    let seasonal = match arg_after(args, "--seasonal") {
        Some(s) => s,
        None => "none",
    };
    let seasonal = match seasonal {
        "none" => Seasonal::None,
        "climatology+standardize" => Seasonal::Climatology,
        other => {
            return Err(format!(
                "--seasonal '{other}' names no none|climatology+standardize"
            ));
        }
    };
    let lags = match arg_after(args, "--lags") {
        Some(t) => parse_lags(t)?,
        None => default_lags(),
    };
    let surrogate = match arg_after(args, "--surrogate") {
        Some(t) => t
            .parse()
            .map_err(|_| format!("--surrogate '{t}' carries no count"))?,
        None => 100,
    };
    let register = match arg_after(args, "--register") {
        Some("sources") => Register::Sources,
        Some("witnesses") => Register::Witnesses,
        Some(other) => {
            return Err(format!("--register '{other}' names no sources|witnesses"));
        }
        None => Register::Sources,
    };
    let bin = match arg_after(args, "--bin") {
        Some(t) => {
            let seconds: f64 = t
                .parse()
                .map_err(|_| format!("--bin '{t}' carries no second count"))?;
            if !(seconds.is_finite() && seconds > 0.0) {
                return Err(format!("--bin '{t}' is no positive finite width"));
            }
            Some(seconds)
        }
        None => None,
    };
    Ok(Descriptor {
        pair: None,
        driver: Arm {
            name: driver.to_string(),
            state: State::Built,
        },
        target: Arm {
            name: target.to_string(),
            state: State::Built,
        },
        cond,
        events: Vec::new(),
        gates: Vec::new(),
        seasonal,
        lags,
        surrogate,
        register,
        bin,
    })
}

fn field_matches(fc: &FieldConfig, name: &str) -> bool {
    fc.name == name || fc.key == name
}

fn source_fields(s: &SourceConfig) -> Vec<FieldConfig> {
    let mut out = Vec::new();
    for e in &s.extracts {
        match e {
            Extract::Field(fc)
            | Extract::First(fc, _)
            | Extract::Last(fc, _)
            | Extract::Count(fc)
            | Extract::LastRow(fc)
            | Extract::ObjLast(fc)
            | Extract::Path(fc)
            | Extract::Deep(fc)
            | Extract::Regex(fc) => out.push(fc.clone()),
            Extract::Map { fields, .. }
            | Extract::CelestialMap { fields, .. }
            | Extract::ProfileMap { fields, .. }
            | Extract::EpnCore { fields, .. }
            | Extract::Rows { fields, .. }
            | Extract::Flatten { fields, .. }
            | Extract::CmrPolygon { fields, .. }
            | Extract::CelestialPolygon { fields, .. }
            | Extract::KeplerMap { fields, .. } => out.extend(fields.iter().cloned()),
            _ => {}
        }
    }
    out
}

fn find_field_source(sources: &[SourceConfig], name: &str) -> Option<(SourceConfig, FieldConfig)> {
    for s in sources {
        for fc in source_fields(s) {
            if field_matches(&fc, name) {
                return Some((s.clone(), fc));
            }
        }
    }
    None
}

fn witness_kind_token(token: &str) -> Option<WitnessKind> {
    match token {
        "s2-direction" => Some(WitnessKind::S2Direction),
        "point-event" => Some(WitnessKind::PointEvent),
        "gestalt" => Some(WitnessKind::Gestalt),
        "presence" => Some(WitnessKind::Presence),
        "substance" => Some(WitnessKind::Substance),
        _ => None,
    }
}

fn load_witnesses() -> Vec<WitnessRecord> {
    match std::fs::read_to_string("phi/witnesses.φ") {
        Ok(content) => parse_witnesses(&content),
        Err(_) => Vec::new(),
    }
}

fn parse_witnesses(content: &str) -> Vec<WitnessRecord> {
    let mut out: Vec<WitnessRecord> = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    for raw in content.lines().chain(std::iter::once("")) {
        let line = raw.trim();
        if line.is_empty() {
            if !block.is_empty() {
                if let Some(w) = witness_from_block(&block, &out) {
                    out.push(w);
                }
                block.clear();
            }
            continue;
        }
        block.push(line);
    }
    out
}

fn witness_from_block(lines: &[&str], out: &[WitnessRecord]) -> Option<WitnessRecord> {
    let mut kind: Option<WitnessKind> = None;
    let mut kind_token = String::new();
    let mut url: Option<String> = None;
    let mut records: Vec<String> = Vec::new();
    let mut force: Option<String> = None;
    for line in lines {
        if line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(head) = parts.next() else {
            continue;
        };
        match head {
            "witness" => {
                let Some(tok) = parts.next() else {
                    continue;
                };
                kind = witness_kind_token(tok);
                kind_token = tok.to_string();
            }
            "url" => {
                if let Some(v) = parts.next() {
                    url = Some(v.to_string());
                }
            }
            "record" => {
                for t in parts {
                    records.push(t.to_string());
                }
            }
            "force" => {
                force = parts.next().map(|s| s.to_string());
            }
            _ => {}
        }
    }
    let url = url?;
    if url.is_empty() || kind_token.is_empty() {
        return None;
    }
    let idx = out.iter().filter(|w| w.kind_token == kind_token).count();
    let key = format!("{kind_token}#{idx}");
    Some(WitnessRecord {
        kind,
        kind_token,
        key,
        url,
        records,
        force,
    })
}

fn find_witness<'a>(witnesses: &'a [WitnessRecord], name: &str) -> Option<&'a WitnessRecord> {
    if let Some(w) = witnesses.iter().find(|w| w.key == name) {
        return Some(w);
    }
    let by_record: Vec<&WitnessRecord> = witnesses
        .iter()
        .filter(|w| w.records.iter().any(|r| r == name))
        .collect();
    if by_record.len() == 1 {
        return Some(by_record[0]);
    }
    let by_url: Vec<&WitnessRecord> = witnesses.iter().filter(|w| w.url.contains(name)).collect();
    if by_url.len() == 1 {
        return Some(by_url[0]);
    }
    None
}

fn witness_series(w: &WitnessRecord) -> Result<Vec<(f64, f64)>, String> {
    let Some(bytes) = fetch_raw_bytes_headers(&w.url, &[]) else {
        return Err(format!("witness '{}' fetch void", w.key));
    };
    let text = String::from_utf8_lossy(&bytes);
    let stamps = event_unix_from_text(&text);
    if stamps.is_empty() {
        return Err(format!(
            "witness '{}' record carries no parsed event timestamp",
            w.key
        ));
    }
    let Some(lsk) = embedded_lsk() else {
        return Err("leap-second table absent — the witness event train stays unmeasured".into());
    };
    let mut out = Vec::new();
    for unix in stamps {
        if let Some(t) = lsk.unix_to_tdb(unix) {
            if t.is_finite() {
                out.push((t, 1.0));
            }
        }
    }
    if out.is_empty() {
        return Err(format!(
            "witness '{}' carries no timestamp inside the leap-second window",
            w.key
        ));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    Ok(out)
}

fn event_unix_from_text(text: &str) -> Vec<f64> {
    let mut out = Vec::new();
    for key in ["\"time_\"", "\"time\"", "\"at\""] {
        let mut from = 0usize;
        while let Some(rel) = text[from..].find(key) {
            let at = from + rel + key.len();
            from = at;
            let rest = text[at..].trim_start();
            let Some(body) = rest.strip_prefix(':') else {
                continue;
            };
            if let Some(v) = parse_json_time_scalar(body.trim_start()) {
                out.push(v);
            }
        }
    }
    out
}

fn parse_json_time_scalar(body: &str) -> Option<f64> {
    if let Some(inner) = body.strip_prefix('"') {
        let end = inner.find('"')?;
        return parse_epoch(&inner[..end]);
    }
    let end = body
        .find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace())
        .unwrap_or(body.len());
    let v: f64 = body[..end].parse().ok()?;
    if !v.is_finite() {
        return None;
    }
    Some(if v.abs() >= 1.0e11 { v / 1_000.0 } else { v })
}

fn offset_sign(time: &str) -> Option<usize> {
    if let Some(i) = time.rfind('+') {
        return Some(i);
    }
    match time.rfind('-') {
        Some(i) if i > 0 => Some(i),
        _ => None,
    }
}

fn parse_epoch(s: &str) -> Option<f64> {
    let s = s.trim();
    if let Ok(v) = s.parse::<f64>() {
        return if v.is_finite() { Some(v) } else { None };
    }
    let (date, time) = match s.split_once('T') {
        Some((d, t)) => (d, t),
        None => match s.split_once(' ') {
            Some((d, t)) => (d, t),
            None => (s, "0"),
        },
    };
    let (time, shift_s) = match offset_sign(time) {
        Some(i) => {
            let sign = if time.as_bytes()[i] == b'+' {
                1i64
            } else {
                -1i64
            };
            let off = &time[i + 1..];
            let mut op = off.split(':');
            let oh: i64 = op.next()?.parse().ok()?;
            let om: i64 = match op.next() {
                Some(v) => v.parse().ok()?,
                None => 0,
            };
            (&time[..i], sign * (oh * 3_600 + om * 60))
        }
        None => (time, 0),
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let clock = match time.split(|c| c == '.' || c == 'Z' || c == 'z').next() {
        Some(c) => c,
        None => "0",
    };
    let mut tp = clock.split(':');
    let hh: i64 = match tp.next() {
        Some(v) => v.parse().ok()?,
        None => 0,
    };
    let mm: i64 = match tp.next() {
        Some(v) => v.parse().ok()?,
        None => 0,
    };
    let ss: i64 = match tp.next() {
        Some(v) => v.parse().ok()?,
        None => 0,
    };
    let days = days_from_civil(y, m, d)?;
    Some(
        days as f64 * 86_400.0 + hh as f64 * 3_600.0 + mm as f64 * 60.0 + ss as f64
            - shift_s as f64,
    )
}

fn load_text_rows(src: &SourceConfig, fc: &FieldConfig, bytes: &[u8]) -> Option<Vec<(f64, f64)>> {
    let lsk = embedded_lsk()?;
    let (epoch_col, field_key) = src.extracts.iter().find_map(|e| match e {
        Extract::Rows {
            fields, epoch_cols, ..
        } if fields.iter().any(|f| field_matches(f, &fc.name)) => {
            let epoch = match epoch_cols
                .first()
                .and_then(|c| c.trim().parse::<usize>().ok())
            {
                Some(v) => v,
                None => 0,
            };
            let key = fc.key.trim().parse::<usize>().ok()?;
            Some((epoch, key))
        }
        _ => None,
    })?;
    let text = String::from_utf8_lossy(bytes);
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(|c| c.trim()).collect();
        let Some(epoch_s) = cols.get(epoch_col) else {
            continue;
        };
        let Some(unix) = parse_epoch(epoch_s) else {
            continue;
        };
        let Some(t) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let Some(raw) = cols.get(field_key).and_then(|c| c.parse::<f64>().ok()) else {
            continue;
        };
        if raw.is_finite() {
            out.push((t, raw));
        }
    }
    if out.is_empty() {
        None
    } else {
        out.sort_by(|a, b| a.0.total_cmp(&b.0));
        Some(out)
    }
}

fn text_source_for_field(src: &SourceConfig, name: &str) -> Option<SourceConfig> {
    let mut kept = Vec::new();
    for e in &src.extracts {
        match e {
            Extract::Field(fc) if field_matches(fc, name) => kept.push(e.clone()),
            Extract::First(fc, _)
            | Extract::Last(fc, _)
            | Extract::Path(fc)
            | Extract::Deep(fc)
            | Extract::Regex(fc)
                if field_matches(fc, name) =>
            {
                kept.push(e.clone());
            }
            Extract::Hapi(pairs) if pairs.iter().any(|(_, n)| n == name) => kept.push(e.clone()),
            _ => {}
        }
    }
    if kept.is_empty() {
        None
    } else {
        let mut filtered = src.clone();
        filtered.extracts = kept;
        Some(filtered)
    }
}

fn load_field(src: &SourceConfig, fc: &FieldConfig) -> Result<Vec<(f64, f64)>, String> {
    let Some(bytes) = fetch_raw_bytes_headers(&src.url, &src.headers) else {
        return Err(format!("{} fetch void", src.url));
    };
    if let Some(rows) = series_rows(&src.format, &bytes) {
        let mut out = Vec::new();
        for r in &rows {
            if series_component_name(&src.format, r.comp) == Some(fc.name.as_str())
                && r.value.is_finite()
            {
                out.push((r.t, r.value));
            }
        }
        if out.is_empty() {
            return Err(format!(
                "format {} carries no component '{}'",
                src.format, fc.name
            ));
        }
        out.sort_by(|a, b| a.0.total_cmp(&b.0));
        return Ok(out);
    }
    if let Some(recs) = geo_series_parse_bin(&src.format, &bytes) {
        let mut out = Vec::new();
        for r in &recs {
            if geo_series_component_name(&src.format, r.comp) == Some(fc.name.as_str())
                && r.val.is_finite()
            {
                out.push((r.t, r.val));
            }
        }
        if out.is_empty() {
            return Err(format!(
                "format {} carries no component '{}'",
                src.format, fc.name
            ));
        }
        out.sort_by(|a, b| a.0.total_cmp(&b.0));
        return Ok(out);
    }
    if src.format == "text" {
        if let Some(series) = load_text_rows(src, fc, &bytes) {
            return Ok(series);
        }
    }
    if let Some(filtered) = text_source_for_field(src, &fc.name) {
        let Some(lsk) = embedded_lsk() else {
            return Err(format!(
                "leap-second table absent — format {} text arm stays unmeasured",
                src.format
            ));
        };
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let mut series = extract_series(&filtered, &text, &lsk);
        if !series.is_empty() {
            series.sort_by(|a, b| a.0.total_cmp(&b.0));
            return Ok(series);
        }
        return Err(format!(
            "format {} text arm carries no series for '{}'",
            src.format, fc.name
        ));
    }
    Err(format!(
        "format {} carries no field selector arm for '{}'",
        src.format, fc.name
    ))
}

enum ArmLoad {
    Ready {
        source: SourceConfig,
        field: FieldConfig,
        series: Vec<(f64, f64)>,
    },
    WitnessReady {
        detail: String,
        series: Vec<(f64, f64)>,
    },
    Pending(String),
}

fn try_source(sources: &[SourceConfig], arm: &Arm) -> Option<ArmLoad> {
    find_field_source(sources, &arm.name).map(|(source, field)| match load_field(&source, &field) {
        Ok(series) => ArmLoad::Ready {
            source,
            field,
            series,
        },
        Err(reason) => ArmLoad::Pending(reason),
    })
}

fn try_witness(witnesses: &[WitnessRecord], arm: &Arm) -> Option<ArmLoad> {
    find_witness(witnesses, &arm.name).map(|w| load_witness_arm(w, arm.state))
}

fn load_witness_arm(w: &WitnessRecord, state: State) -> ArmLoad {
    if state == State::Pending {
        return ArmLoad::Pending("descriptor state pending (register duty)".into());
    }
    if w.kind != Some(WitnessKind::PointEvent) {
        return ArmLoad::Pending(format!(
            "witness '{}' kind {} is no event channel (τ=0 catalogue/spectrum/gestalt) — never poured into a series, never 0.0",
            w.key, w.kind_token
        ));
    }
    match witness_series(w) {
        Ok(series) => ArmLoad::WitnessReady {
            detail: format!(
                "record {} force {} | {}",
                w.records.join("+"),
                w.force.as_deref().unwrap_or("absent"),
                w.url
            ),
            series,
        },
        Err(reason) => ArmLoad::Pending(reason),
    }
}

fn load_arm(
    sources: &[SourceConfig],
    witnesses: &[WitnessRecord],
    arm: &Arm,
    register: Register,
) -> ArmLoad {
    if arm.state == State::Pending {
        return ArmLoad::Pending("descriptor state pending (register duty)".into());
    }
    let order: [u8; 2] = if register == Register::Witnesses {
        [1, 0]
    } else {
        [0, 1]
    };
    for which in order {
        let attempt = if which == 0 {
            try_source(sources, arm)
        } else {
            try_witness(witnesses, arm)
        };
        if let Some(load) = attempt {
            return load;
        }
    }
    ArmLoad::Pending(format!("'{}' stands in no register block", arm.name))
}

fn print_arm(role: &str, arm: &Arm, load: &ArmLoad) {
    match load {
        ArmLoad::Ready {
            source,
            field,
            series,
        } => println!(
            "ARM {role:<6} {} {} | format {} | force {} unit {} | n = {} | {}",
            arm.name,
            arm.state.name(),
            source.format,
            field.force,
            field.unit,
            series.len(),
            source.url
        ),
        ArmLoad::WitnessReady { detail, series } => {
            println!(
                "ARM {role:<6} {} {} | witness | n = {} | {detail}",
                arm.name,
                arm.state.name(),
                series.len()
            );
        }
        ArmLoad::Pending(reason) => {
            println!(
                "PENDING {role:<6} {} {} — {reason}",
                arm.name,
                arm.state.name()
            );
        }
    }
}

fn print_count_arm(role: &str, cells: &[Option<f64>]) {
    let carried = cells.iter().filter(|c| c.is_some()).count();
    let pending = cells.len() - carried;
    println!(
        "count arm {role}: {carried} bins carry events | {pending} bins pending (no event inside the fetched train — never 0.0 as a physical value)"
    );
}

fn median_dt(series: &[(f64, f64)]) -> Option<f64> {
    if series.len() < 2 {
        return None;
    }
    let mut d: Vec<f64> = series
        .windows(2)
        .map(|w| w[1].0 - w[0].0)
        .filter(|x| x.is_finite() && *x > 0.0)
        .collect();
    if d.is_empty() {
        return None;
    }
    d.sort_by(|a, b| a.total_cmp(b));
    Some(d[d.len() / 2])
}

fn bin_to_grid(series: &[(f64, f64)], grid: &[f64], last_step: f64) -> Vec<Option<f64>> {
    let mut sums = vec![0.0f64; grid.len()];
    let mut counts = vec![0u32; grid.len()];
    let mut mi = 0usize;
    for &(t, v) in series {
        if !v.is_finite() {
            continue;
        }
        while mi + 1 < grid.len() && t >= grid[mi + 1] {
            mi += 1;
        }
        if t < grid[mi] {
            continue;
        }
        let hi = match grid.get(mi + 1) {
            Some(&h) => h,
            None => grid[mi] + last_step,
        };
        if t >= hi {
            continue;
        }
        sums[mi] += v;
        counts[mi] += 1;
    }
    (0..grid.len())
        .map(|i| {
            if counts[i] > 0 {
                Some(sums[i] / counts[i] as f64)
            } else {
                None
            }
        })
        .collect()
}

fn bin_count_to_grid(series: &[(f64, f64)], grid: &[f64], last_step: f64) -> Vec<Option<f64>> {
    let mut counts = vec![0u32; grid.len()];
    let mut mi = 0usize;
    for &(t, _) in series {
        while mi + 1 < grid.len() && t >= grid[mi + 1] {
            mi += 1;
        }
        if t < grid[mi] {
            continue;
        }
        let hi = match grid.get(mi + 1) {
            Some(&h) => h,
            None => grid[mi] + last_step,
        };
        if t >= hi {
            continue;
        }
        counts[mi] += 1;
    }
    counts
        .iter()
        .map(|&c| if c > 0 { Some(c as f64) } else { None })
        .collect()
}

fn deseasonalize(series: &[Option<f64>]) -> Vec<Option<f64>> {
    let mut sums = [0.0f64; CAL_MONTHS];
    let mut sumsq = [0.0f64; CAL_MONTHS];
    let mut counts = [0u32; CAL_MONTHS];
    for (i, v) in series.iter().enumerate() {
        if let Some(x) = v {
            if x.is_finite() {
                sums[i % CAL_MONTHS] += x;
                sumsq[i % CAL_MONTHS] += x * x;
                counts[i % CAL_MONTHS] += 1;
            }
        }
    }
    let mut means = [0.0f64; CAL_MONTHS];
    let mut sds = [0.0f64; CAL_MONTHS];
    for (m, mean) in means.iter_mut().enumerate() {
        if counts[m] >= CLIMATOLOGY_FLOOR as u32 {
            let n = counts[m] as f64;
            *mean = sums[m] / n;
            let var = (sumsq[m] / n - *mean * *mean).max(0.0);
            sds[m] = var.sqrt();
        } else if counts[m] > 0 {
            println!(
                "deseasonalize: calendar month {:02} carries n = {} < floor {CLIMATOLOGY_FLOOR} — its values stay unchanged (climatology not removed)",
                m + 1,
                counts[m]
            );
        }
    }
    series
        .iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(x) if counts[i % CAL_MONTHS] >= CLIMATOLOGY_FLOOR as u32 => {
                let m = i % CAL_MONTHS;
                let centered = *x - means[m];
                if sds[m].is_finite() && sds[m] > 0.0 {
                    Some(centered / sds[m])
                } else {
                    Some(centered)
                }
            }
            other => *other,
        })
        .collect()
}

struct Aligned {
    driver: Vec<Option<f64>>,
    target: Vec<Option<f64>>,
    cond: Option<Vec<Option<f64>>>,
    grid: Vec<f64>,
    cadence_s: f64,
}

fn align(
    driver: &[(f64, f64)],
    target: &[(f64, f64)],
    cond: Option<&[(f64, f64)]>,
    seasonal: Seasonal,
    driver_count: bool,
    target_count: bool,
    cond_count: bool,
    bin_seconds: Option<f64>,
) -> Result<Aligned, String> {
    let monthly = matches!(seasonal, Seasonal::Climatology);
    let (grid, grid_dt) = match bin_seconds {
        Some(step) => {
            if !(step.is_finite() && step > 0.0) {
                return Err(format!("bin {step} is no positive finite width"));
            }
            let d0 = driver
                .first()
                .map(|p| p.0)
                .ok_or("driver carries no stamped sample")?;
            let d1 = driver
                .last()
                .map(|p| p.0)
                .ok_or("driver carries no stamped sample")?;
            let t0 = target
                .first()
                .map(|p| p.0)
                .ok_or("target carries no stamped sample")?;
            let t1 = target
                .last()
                .map(|p| p.0)
                .ok_or("target carries no stamped sample")?;
            let mut start = d0.max(t0);
            let mut end = d1.min(t1);
            if let Some(c) = cond {
                if let (Some(c0), Some(c1)) = (c.first().map(|p| p.0), c.last().map(|p| p.0)) {
                    start = start.max(c0);
                    end = end.min(c1);
                }
            }
            if !(end > start) {
                return Err("bin window carries no overlap between the arms".into());
            }
            let n = ((end - start) / step).ceil() as usize;
            if n == 0 {
                return Err("bin window carries no cell".into());
            }
            let grid: Vec<f64> = (0..n).map(|i| start + i as f64 * step).collect();
            (grid, step)
        }
        None => {
            let d_dt =
                median_dt(driver).ok_or("driver cadence underdetermined (< 2 stamped samples)")?;
            let t_dt =
                median_dt(target).ok_or("target cadence underdetermined (< 2 stamped samples)")?;
            let grid_from_target = if target_count && !driver_count {
                false
            } else if driver_count && !target_count {
                true
            } else {
                t_dt >= d_dt
            };
            let grid_dt = if grid_from_target { t_dt } else { d_dt };
            let grid: Vec<f64> = if grid_from_target {
                target.iter().map(|p| p.0).collect()
            } else {
                driver.iter().map(|p| p.0).collect()
            };
            (grid, grid_dt)
        }
    };
    if monthly && !(25.0 * 86_400.0..=32.0 * 86_400.0).contains(&grid_dt) {
        return Err(format!(
            "climatology+standardize needs a monthly grid arm (slowest cadence {grid_dt:.0} s); the deseasonalization lives on grid index mod {CAL_MONTHS}"
        ));
    }
    let last_step = if monthly { MONTH_S } else { grid_dt };
    let mut d_cells = if driver_count {
        bin_count_to_grid(driver, &grid, last_step)
    } else {
        bin_to_grid(driver, &grid, last_step)
    };
    let mut t_cells = if target_count {
        bin_count_to_grid(target, &grid, last_step)
    } else {
        bin_to_grid(target, &grid, last_step)
    };
    let mut c_cells = cond.map(|c| {
        if cond_count {
            bin_count_to_grid(c, &grid, last_step)
        } else {
            bin_to_grid(c, &grid, last_step)
        }
    });
    if monthly {
        d_cells = deseasonalize(&d_cells);
        t_cells = deseasonalize(&t_cells);
        c_cells = c_cells.map(|c| deseasonalize(&c));
    }
    Ok(Aligned {
        driver: d_cells,
        target: t_cells,
        cond: c_cells,
        grid,
        cadence_s: grid_dt,
    })
}

fn pair2(a: &[Option<f64>], b: &[Option<f64>]) -> (Vec<f32>, Vec<f32>) {
    let mut av = Vec::new();
    let mut bv = Vec::new();
    for (x, y) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (x, y) {
            if x.is_finite() && y.is_finite() {
                av.push(*x as f32);
                bv.push(*y as f32);
            }
        }
    }
    (av, bv)
}

fn pair3(
    a: &[Option<f64>],
    b: &[Option<f64>],
    c: &[Option<f64>],
) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut av = Vec::new();
    let mut bv = Vec::new();
    let mut cv = Vec::new();
    for ((x, y), z) in a.iter().zip(b.iter()).zip(c.iter()) {
        if let (Some(x), Some(y), Some(z)) = (x, y, z) {
            if x.is_finite() && y.is_finite() && z.is_finite() {
                av.push(*x as f32);
                bv.push(*y as f32);
                cv.push(*z as f32);
            }
        }
    }
    (av, bv, cv)
}

struct LagRow {
    lag: usize,
    te_d2t: Option<f64>,
    thr_d2t: Option<f64>,
    te_t2d: Option<f64>,
    thr_t2d: Option<f64>,
}

struct Direction {
    word: String,
    best_lag: Option<usize>,
    best_te: Option<f64>,
}

struct QueryResult {
    n_paired: usize,
    fam: Option<f64>,
    fam_core: Option<f64>,
    rows: Vec<LagRow>,
    d2t: Direction,
    t2d: Direction,
    cte: Option<(f64, f64)>,
}

fn direction_of(rows: &[LagRow], d2t: bool, fam: Option<f64>) -> Direction {
    let mut best: Option<(usize, f64, Option<f64>)> = None;
    for r in rows {
        let (te, thr) = if d2t {
            (r.te_d2t, r.thr_d2t)
        } else {
            (r.te_t2d, r.thr_t2d)
        };
        let Some(te) = te else {
            continue;
        };
        if best.map_or(true, |(_, b, _)| te > b) {
            best = Some((r.lag, te, thr));
        }
    }
    match best {
        None => Direction {
            word: "absent".into(),
            best_lag: None,
            best_te: None,
        },
        Some((lag, te, thr)) => {
            let word = match fam {
                Some(f) if te > f => "arrow",
                _ => match thr {
                    Some(t) if te > t => "family bound",
                    _ => "silent",
                },
            };
            Direction {
                word: word.into(),
                best_lag: Some(lag),
                best_te: Some(te),
            }
        }
    }
}

fn analyze(
    desc: &Descriptor,
    target_s: &[f32],
    driver_s: &[f32],
    cond_triple: Option<(&[f32], &[f32], &[f32])>,
    fam_override: Option<f64>,
) -> QueryResult {
    let mut members: Vec<Member> = Vec::new();
    let mut meta: Vec<(usize, bool)> = Vec::new();
    for &lag in &desc.lags {
        members.push(Member::new(
            "driver->target",
            0,
            lag,
            target_s.to_vec(),
            driver_s.to_vec(),
        ));
        meta.push((lag, true));
        members.push(Member::new(
            "target->driver",
            1,
            lag,
            driver_s.to_vec(),
            target_s.to_vec(),
        ));
        meta.push((lag, false));
    }
    let observed = observed_family(&members);

    let mut rows: Vec<LagRow> = Vec::with_capacity(desc.lags.len());
    for &lag in &desc.lags {
        let thr_d2t =
            surrogate_stats_phase_n(target_s, driver_s, lag, SURROGATE_SEED, desc.surrogate)
                .map(|(_, _, t)| t);
        let thr_t2d =
            surrogate_stats_phase_n(driver_s, target_s, lag, SURROGATE_SEED, desc.surrogate)
                .map(|(_, _, t)| t);
        rows.push(LagRow {
            lag,
            te_d2t: None,
            thr_d2t,
            te_t2d: None,
            thr_t2d,
        });
    }
    for (mi, (lag, d2t)) in meta.iter().enumerate() {
        let Some(te) = observed.get(mi).copied().flatten() else {
            continue;
        };
        if let Some(row) = rows.iter_mut().find(|r| r.lag == *lag) {
            if *d2t {
                row.te_d2t = Some(te);
            } else {
                row.te_t2d = Some(te);
            }
        }
    }

    let mut fam_core: Option<f64> = None;
    for &lag in &desc.lags {
        for (x, y) in [(target_s, driver_s), (driver_s, target_s)] {
            if let Some(m) = surrogate_max_phase_n(x, y, lag, SURROGATE_SEED, desc.surrogate) {
                fam_core = Some(match fam_core {
                    Some(f) => f.max(m),
                    None => m,
                });
            }
        }
    }
    let fam = fam_override.or(fam_core);

    let d2t = direction_of(&rows, true, fam);
    let t2d = direction_of(&rows, false, fam);
    let cte = cond_triple.and_then(|(t, d, c)| {
        conditional_embedded_te_phase(t, d, c, 3, SURROGATE_SEED).map(|v| (v.te, v.threshold))
    });

    QueryResult {
        n_paired: target_s.len(),
        fam,
        fam_core,
        rows,
        d2t,
        t2d,
        cte,
    }
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "absent".to_string(),
    }
}

fn print_result(result: &QueryResult, desc: &Descriptor) {
    println!();
    println!(
        "=== TE machine (omegaflow::te, phase-randomized null, {} surrogates) | paired n = {} ===",
        desc.surrogate, result.n_paired
    );
    println!(
        "{:>4} | {:>14} | {:>14} | {:>14} | {:>14}",
        "lag", "TE(d->t)", "thr(d->t)", "TE(t->d)", "thr(t->d)"
    );
    for r in &result.rows {
        println!(
            "{:>4} | {:>14} | {:>14} | {:>14} | {:>14}",
            r.lag,
            fmt_opt(r.te_d2t),
            fmt_opt(r.thr_d2t),
            fmt_opt(r.te_t2d),
            fmt_opt(r.thr_t2d)
        );
    }
    println!();
    println!(
        "family bound fam = {} (round max over the measured directed pairs x lags, phase surrogates)",
        fmt_opt(result.fam)
    );
    if result.fam != result.fam_core {
        println!(
            "core family bound over the declared arms = {}",
            fmt_opt(result.fam_core)
        );
    }
    println!(
        "driver -> target: {} | best lag {} | TE {}",
        result.d2t.word,
        match result.d2t.best_lag {
            Some(l) => l.to_string(),
            None => "absent".to_string(),
        },
        fmt_opt(result.d2t.best_te)
    );
    println!(
        "target -> driver: {} | best lag {} | TE {}",
        result.t2d.word,
        match result.t2d.best_lag {
            Some(l) => l.to_string(),
            None => "absent".to_string(),
        },
        fmt_opt(result.t2d.best_te)
    );
    match result.cte {
        Some((te, thr)) => {
            let word = if te > thr {
                "conditioned arrow"
            } else {
                "conditioned silent"
            };
            println!("cTE(target -> driver | cond) = {te:.4e} | threshold {thr:.4e} | {word}");
        }
        None => {
            if desc.cond.is_some() {
                println!("cTE pending — the conditioning arm carries no aligned series");
            }
        }
    }
}

fn execute(
    desc: &Descriptor,
    sources: &[SourceConfig],
    witnesses: &[WitnessRecord],
    fam_override: Option<f64>,
) -> Option<QueryResult> {
    println!("=== field_te_query — fields by name over the Archivar path, the one TE machine ===");
    if let Some(p) = &desc.pair {
        println!("round: {p}");
    }
    println!(
        "alignment: seasonal {} | lags {:?} | surrogate {} | cadence measured live",
        desc.seasonal.name(),
        desc.lags,
        desc.surrogate
    );

    let driver_load = load_arm(sources, witnesses, &desc.driver, desc.register);
    let target_load = load_arm(sources, witnesses, &desc.target, desc.register);
    let cond_load = desc
        .cond
        .as_ref()
        .map(|a| load_arm(sources, witnesses, a, desc.register));
    print_arm("driver", &desc.driver, &driver_load);
    print_arm("target", &desc.target, &target_load);
    if let (Some(arm), Some(load)) = (&desc.cond, &cond_load) {
        print_arm("cond", arm, load);
    }
    if !desc.events.is_empty() || !desc.gates.is_empty() {
        println!();
        println!("pending register (refs, never a silent number):");
        for (r, s) in &desc.events {
            println!("  event {r} {s}");
        }
        for (r, s) in &desc.gates {
            println!("  gate {r} {s}");
        }
    }

    let driver_series = match &driver_load {
        ArmLoad::Ready { series, .. } | ArmLoad::WitnessReady { series, .. } => series,
        ArmLoad::Pending(reason) => {
            println!();
            println!("driver arm pending ({reason}) — the pair stays unmeasured");
            return None;
        }
    };
    let target_series = match &target_load {
        ArmLoad::Ready { series, .. } | ArmLoad::WitnessReady { series, .. } => series,
        ArmLoad::Pending(reason) => {
            println!();
            println!("target arm pending ({reason}) — the pair stays unmeasured");
            return None;
        }
    };
    let cond_series = match &cond_load {
        Some(ArmLoad::Ready { series, .. }) | Some(ArmLoad::WitnessReady { series, .. }) => {
            Some(series.as_slice())
        }
        _ => None,
    };

    let driver_count = match &driver_load {
        ArmLoad::WitnessReady { .. } => true,
        _ => false,
    };
    let target_count = match &target_load {
        ArmLoad::WitnessReady { .. } => true,
        _ => false,
    };
    let cond_count = match &cond_load {
        Some(ArmLoad::WitnessReady { .. }) => true,
        _ => false,
    };
    let aligned = match align(
        driver_series,
        target_series,
        cond_series,
        desc.seasonal,
        driver_count,
        target_count,
        cond_count,
        desc.bin,
    ) {
        Ok(a) => a,
        Err(reason) => {
            println!();
            println!("alignment absent: {reason}");
            return None;
        }
    };
    println!();
    if desc.bin.is_some() {
        println!(
            "aligned grid: cells {} | cadence {} s (declared bin)",
            aligned.grid.len(),
            fmt_opt(Some(aligned.cadence_s))
        );
    } else {
        println!(
            "aligned grid: cells {} | cadence {} s (measured from the slowest arm)",
            aligned.grid.len(),
            fmt_opt(Some(aligned.cadence_s))
        );
    }
    if driver_count {
        print_count_arm("driver", &aligned.driver);
    }
    if target_count {
        print_count_arm("target", &aligned.target);
    }
    if cond_count {
        if let Some(c) = &aligned.cond {
            print_count_arm("cond", c);
        }
    }
    let (target_s, driver_s) = pair2(&aligned.target, &aligned.driver);
    let cond_triple = aligned
        .cond
        .as_ref()
        .map(|c| pair3(&aligned.target, &aligned.driver, c));
    if target_s.len() < TE_FLOOR {
        println!(
            "paired samples n = {} < {TE_FLOOR} — the TE stays unmeasured",
            target_s.len()
        );
        return None;
    }
    let cond_in = cond_triple
        .as_ref()
        .map(|(t, d, c)| (t.as_slice(), d.as_slice(), c.as_slice()));
    let result = analyze(desc, &target_s, &driver_s, cond_in, fam_override);
    print_result(&result, desc);
    Some(result)
}

fn is_arrow(word: &str) -> bool {
    word == "arrow"
}

fn run_parity(sources: &[SourceConfig], witnesses: &[WitnessRecord]) -> i32 {
    let desc = Descriptor {
        pair: Some("enso-bz-sst (Blatt I)".into()),
        driver: Arm {
            name: "omni_hro_imf_bz_gsm_nt".into(),
            state: State::Built,
        },
        target: Arm {
            name: "ersstv5_nino34_ssta".into(),
            state: State::Built,
        },
        cond: Some(Arm {
            name: "tao_wnd_zonal_m_s".into(),
            state: State::Built,
        }),
        events: Vec::new(),
        gates: Vec::new(),
        seasonal: Seasonal::Climatology,
        lags: (0..=12).collect(),
        surrogate: 100,
        register: Register::Sources,
        bin: None,
    };
    let Some(result) = execute(&desc, sources, witnesses, Some(REC_FAM)) else {
        println!();
        println!("PARITY: UNMEASURED (an arm or the alignment stays absent)");
        return 0;
    };

    println!();
    println!("=== parity bridge — core vs the recorded ENSO Blatt I verdict ===");
    println!(
        "recorded: fam {REC_FAM:.4e} | Bz->SST {REC_D2T_WORD} best lag {REC_D2T_LAG} TE {REC_D2T_TE:.4e} | SST->Bz {REC_T2D_WORD} best lag {REC_T2D_LAG} TE {REC_T2D_TE:.4e} | cTE {REC_CTE:.4e} < {REC_CTE_THR:.4e}"
    );
    println!(
        "  (docs/blatt/sonne-erde-blatt.md:24; docs/handover/archiv/handover-2026-09-30-river-folge74.md:91-92)"
    );
    let cte_word = match result.cte {
        Some((te, thr)) => {
            if te > thr {
                "conditioned arrow"
            } else {
                "conditioned silent"
            }
        }
        None => "absent",
    };
    println!(
        "core:     fam {} | Bz->SST {} best lag {} TE {} | SST->Bz {} best lag {} TE {} | cTE {}",
        fmt_opt(result.fam),
        result.d2t.word,
        match result.d2t.best_lag {
            Some(l) => l.to_string(),
            None => "absent".to_string(),
        },
        fmt_opt(result.d2t.best_te),
        result.t2d.word,
        match result.t2d.best_lag {
            Some(l) => l.to_string(),
            None => "absent".to_string(),
        },
        fmt_opt(result.t2d.best_te),
        cte_word
    );
    let d2t_ok = is_arrow(&result.d2t.word) == is_arrow(REC_D2T_WORD);
    let t2d_ok = is_arrow(&result.t2d.word) == is_arrow(REC_T2D_WORD);
    let cte_ok = match cte_word {
        "conditioned silent" => true,
        "conditioned arrow" => false,
        _ => false,
    };
    println!(
        "Bz->SST: {} (core {} vs recorded {})",
        if d2t_ok { "GLEICH" } else { "ABWEICHEND" },
        result.d2t.word,
        REC_D2T_WORD
    );
    println!(
        "SST->Bz: {} (core {} vs recorded {})",
        if t2d_ok { "GLEICH" } else { "ABWEICHEND" },
        result.t2d.word,
        REC_T2D_WORD
    );
    println!(
        "cTE:     {} (core {} vs recorded conditioned silent)",
        if cte_ok { "GLEICH" } else { "ABWEICHEND" },
        cte_word
    );
    if result.d2t.best_lag == Some(REC_D2T_LAG) {
        if let Some(te) = result.d2t.best_te {
            println!(
                "Bz->SST lag/TE drift: recorded lag {REC_D2T_LAG} TE {REC_D2T_TE:.4e} vs core lag {REC_D2T_LAG} TE {te:.4e}"
            );
        }
    }
    if result.t2d.best_lag == Some(REC_T2D_LAG) {
        if let Some(te) = result.t2d.best_te {
            println!(
                "SST->Bz lag/TE drift: recorded lag {REC_T2D_LAG} TE {REC_T2D_TE:.4e} vs core lag {REC_T2D_LAG} TE {te:.4e}"
            );
        }
    }
    if d2t_ok && t2d_ok && cte_ok {
        println!("PARITY: GLEICH — the core reproduces the recorded ENSO Blatt I verdict");
        0
    } else {
        println!("PARITY: ABWEICHEND — a probe stays held until the divergence is named");
        1
    }
}

fn run_parity_witness(name: &str, witnesses: &[WitnessRecord]) -> i32 {
    println!("=== parity bridge — witness register over phi/witnesses.φ ===");
    let Some(w) = find_witness(witnesses, name) else {
        println!("PARITY-WITNESS: pending — '{name}' stands in no witness block");
        return 0;
    };
    println!(
        "witness {} | kind {} | record {} | force {} | {}",
        w.key,
        w.kind_token,
        w.records.join("+"),
        w.force.as_deref().unwrap_or("absent"),
        w.url
    );
    let arm = Arm {
        name: w.key.clone(),
        state: State::Built,
    };
    let load = load_witness_arm(w, State::Built);
    print_arm("witness", &arm, &load);
    match &load {
        ArmLoad::WitnessReady { series, .. } => println!(
            "event train: {} stamped events, t in TDB from the loaded record",
            series.len()
        ),
        ArmLoad::Pending(reason) => println!("witness arm stays pending — {reason}"),
        ArmLoad::Ready { .. } => {}
    }
    println!(
        "PARITY-WITNESS: pending — no recorded witness TE verdict in docs/ to compare against (measured 2026-10-03 via sgrep over docs/; the point-event register verdict docs/handover/archiv/handover-2026-10-01-mountain-folge218.md:69 names the record build, no TE parity). The arm is measured, the parity stays named pending, never invented."
    );
    0
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    println!(
        "grammar: pair <label> | driver|target|cond <field> [built|pending|probe] | witness <name> [built|pending|probe] | register sources|witnesses | event|gate <ref> pending|probe | cadence live | seasonal none|climatology+standardize | lags <list> | surrogate <n> | bin <seconds>"
    );
    let sources = load_sources();
    let witnesses = load_witnesses();
    if let Some(name) = arg_after(&args, "--parity-witness") {
        exit(run_parity_witness(name, &witnesses));
    }
    if sources.is_empty() {
        eprintln!("phi/sources.φ carries no block — the register stays unread");
        exit(2);
    }

    if args.iter().any(|a| a == "--parity") {
        exit(run_parity(&sources, &witnesses));
    }

    let desc = if let Some(path) = arg_after(&args, "--descriptor") {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("--descriptor {path}: {e}");
                exit(2);
            }
        };
        match parse_descriptor(&text) {
            Ok(d) => d,
            Err(reason) => {
                eprintln!("{reason}");
                exit(2);
            }
        }
    } else {
        match descriptor_from_args(&args) {
            Ok(d) => d,
            Err(reason) => {
                eprintln!("{reason}");
                exit(2);
            }
        }
    };

    match execute(&desc, &sources, &witnesses, None) {
        Some(_) => {
            println!("Silent lines are findings. Exit 0.");
        }
        None => {
            println!("The query stays unmeasured; pending arms are named above. Exit 0.");
        }
    }
}
