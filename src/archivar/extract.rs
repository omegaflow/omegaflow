use super::*;

pub fn series_parse_bin(format: &str, bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    match format {
        "rpw_efield" => crate::rpw::parse_bin(bytes),
        "goes_xrs" => goes::parse_bin(bytes),
        "omni2_serie" => omni2::parse_bin(bytes),
        _ => None,
    }
}



pub fn series_component_name(format: &str, comp: u32) -> Option<&'static str> {
    match format {
        "rpw_efield" => match comp {
            crate::rpw::COMP_EY => Some("rpw_e_y"),
            crate::rpw::COMP_EZ => Some("rpw_e_z"),
            _ => None,
        },
        "goes_xrs" => match comp {
            goes::COMP_XRSA => Some("goes_xrs_xrsa"),
            goes::COMP_XRSB => Some("goes_xrs_xrsb"),
            _ => None,
        },
        "omni2_serie" => match comp {
            omni2::COMP_V1800 => Some("omni_solarwind_flow_speed_kms"),
            omni2::COMP_N1800 => Some("omni_solarwind_density_percc"),
            omni2::COMP_T1800 => Some("omni_solarwind_temp_k"),
            omni2::COMP_BX => Some("omni_imf_bx_gse_nt"),
            omni2::COMP_BY => Some("omni_imf_by_gsm_nt"),
            omni2::COMP_BZ => Some("omni_imf_bz_gsm_nt"),
            omni2::COMP_PRESSURE => Some("omni_solarwind_pressure_npa"),
            _ => None,
        },
        _ => None,
    }
}



pub fn jlast(json: &JsonVal, key: &str) -> Option<f64> {
    if let Some((target_path, final_key)) = key.rsplit_once('.') {
        let parent = if target_path.is_empty() {
            json
        } else {
            jpath_val(json, target_path)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.last().and_then(|v| {
                if let JsonVal::Obj(o) = v {
                    o.get(final_key).and_then(scalar_of)
                } else {
                    scalar_of(v)
                }
            });
        }
        return None;
    }
    match json {
        JsonVal::Arr(arr) => arr.last().and_then(|v| match v {
            JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
            other => scalar_of(other),
        }),
        JsonVal::Obj(map) => map.get(key).and_then(|v| {
            if let JsonVal::Arr(a) = v {
                a.last().and_then(scalar_of)
            } else {
                None
            }
        }),
        _ => None,
    }
}



pub fn jfirst(json: &JsonVal, key: &str) -> Option<f64> {
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.first().and_then(|v| match v {
                JsonVal::Obj(o) => o.get(final_key).and_then(scalar_of),
                other => scalar_of(other),
            });
        }
        return None;
    }
    match json {
        JsonVal::Arr(arr) => arr.first().and_then(|v| match v {
            JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
            other => scalar_of(other),
        }),
        JsonVal::Obj(map) => map.get(key).and_then(|v| {
            if let JsonVal::Arr(a) = v {
                a.first().and_then(scalar_of)
            } else {
                None
            }
        }),
        _ => None,
    }
}



pub fn row_matches(el: &JsonVal, fk: &str, fv: &str) -> bool {
    let JsonVal::Obj(map) = el else {
        return false;
    };
    match map.get(fk) {
        Some(JsonVal::Str(s)) => s == fv,
        Some(JsonVal::Num(n)) => fv.parse::<f64>().map_or(false, |f| f == *n),
        _ => false,
    }
}



pub fn row_value(el: &JsonVal, key: &str) -> Option<f64> {
    match el {
        JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
        other => scalar_of(other),
    }
}



pub fn jfirst_where(json: &JsonVal, key: &str, filter: Option<&(String, String)>) -> Option<f64> {
    let Some((fk, fv)) = filter else {
        return jfirst(json, key);
    };
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        let JsonVal::Arr(arr) = parent else {
            return None;
        };
        return arr
            .iter()
            .find(|v| row_matches(v, fk, fv))
            .and_then(|v| row_value(v, final_key));
    }
    let JsonVal::Arr(arr) = json else {
        return None;
    };
    arr.iter()
        .find(|v| row_matches(v, fk, fv))
        .and_then(|v| row_value(v, key))
}



pub fn jlast_where(json: &JsonVal, key: &str, filter: Option<&(String, String)>) -> Option<f64> {
    let Some((fk, fv)) = filter else {
        return jlast(json, key);
    };
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        let JsonVal::Arr(arr) = parent else {
            return None;
        };
        return arr
            .iter()
            .rev()
            .find(|v| row_matches(v, fk, fv))
            .and_then(|v| row_value(v, final_key));
    }
    let JsonVal::Arr(arr) = json else {
        return None;
    };
    arr.iter()
        .rev()
        .find(|v| row_matches(v, fk, fv))
        .and_then(|v| row_value(v, key))
}



pub fn kernel_id_of(name: &str) -> Option<u8> {
    match name {
        "inverse-square" => Some(0),
        "gaussian-inverse-square" => Some(1),
        "gaussian-inverse" => Some(2),
        "erfc" => Some(3),
        "exponential-decay" => Some(4),
        "patch-levy" => Some(5),
        "inverse-linear" => Some(6),
        _ => None,
    }
}



pub fn extract_fields(ext: &Extract) -> Vec<FieldConfig> {
    match ext {
        Extract::Map { fields, .. }
        | Extract::CelestialMap { fields, .. }
        | Extract::Rows { fields, .. }
        | Extract::Flatten { fields, .. }
        | Extract::CmrPolygon { fields, .. }
        | Extract::CelestialPolygon { fields, .. }
        | Extract::KeplerMap { fields, .. }
        | Extract::ProfileMap { fields, .. } => fields.clone(),
        Extract::Field(fc)
        | Extract::First(fc, _)
        | Extract::Last(fc, _)
        | Extract::Count(fc)
        | Extract::LastRow(fc)
        | Extract::ObjLast(fc)
        | Extract::Path(fc)
        | Extract::Deep(fc)
        | Extract::Regex(fc) => vec![fc.clone()],
        Extract::GeojsonEvents {
            outputs,
            tau,
            absorption,
            advection,
            ..
        } => {
            if outputs.len() < 2 {
                return Vec::new();
            }
            vec![
                FieldConfig {
                    key: outputs[0].clone(),
                    name: outputs[0].clone(),
                    kernel: 0,
                    force: 3,
                    tau: *tau,
                    absorption: *absorption,
                    advection: *advection,
                    unit: "Mw".to_string(),
                    fold: None,
                },
                FieldConfig {
                    key: outputs[1].clone(),
                    name: outputs[1].clone(),
                    kernel: 0,
                    force: 3,
                    tau: *tau,
                    absorption: *absorption,
                    advection: *advection,
                    unit: String::new(),
                    fold: None,
                },
            ]
        }
        _ => Vec::new(),
    }
}



pub fn extract_header(s: &str, n: &str) -> Option<String> {
    for l in s.lines() {
        if let Some(c) = l.find(':') {
            if l[..c].trim().eq_ignore_ascii_case(n) {
                return Some(l[c + 1..].trim().to_string());
            }
        }
    }
    None
}



pub fn split_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    fields.push(cur);
    fields
}



pub fn csv_to_json(text: &str) -> Option<JsonVal> {
    let mut lines = text
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'));
    let header_line = lines.find(|l| l.contains(','))?;
    let headers = split_csv_line(header_line);
    if headers.len() < 2 {
        return None;
    }
    let mut rows = Vec::new();
    for line in lines {
        if !line.contains(',') {
            continue;
        }
        let fields = split_csv_line(line);
        if fields.len() != headers.len() {
            continue;
        }
        let mut obj = HashMap::new();
        for (h, f) in headers.iter().zip(fields.iter()) {
            obj.insert(h.clone(), JsonVal::Str(f.clone()));
        }
        rows.push(JsonVal::Obj(obj));
    }
    Some(JsonVal::Arr(rows))
}



pub fn universal_auto_detect(j: &JsonVal) -> Vec<Extract> {
    let arr = match jpath_val(j, "data").and_then(|v| {
        if let JsonVal::Arr(a) = v {
            Some(a)
        } else {
            None
        }
    }) {
        Some(a) => a,
        None => return vec![],
    };
    let first = match arr.first() {
        Some(JsonVal::Obj(m)) => m,
        _ => return vec![],
    };
    let has_ra = first.contains_key("ra");
    let has_dec = first.contains_key("dec");
    let has_lat = first.contains_key("lat");
    let has_lon = first.contains_key("lon");
    if has_ra && has_dec {
        let plx_key = if first.contains_key("plx") { "plx" } else { "" };
        let pmra_key = if first.contains_key("pmra") {
            "pmra"
        } else {
            ""
        };
        let pmdec_key = if first.contains_key("pmdec") {
            "pmdec"
        } else {
            ""
        };
        let rv_key = if first.contains_key("radvel") {
            "radvel"
        } else {
            ""
        };
        let dist_key = if first.contains_key("dist") {
            "dist"
        } else {
            ""
        };
        let z_key = if first.contains_key("z") { "z" } else { "" };
        let epoch_key = if first.contains_key("t") { "t" } else { "" };
        let mut fields = vec![];
        if first.contains_key("val") {
            fields.push(FieldConfig {
                key: "val".into(),
                name: "val".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        if first.contains_key("extent") {
            fields.push(FieldConfig {
                key: "extent".into(),
                name: "extent".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        if first.contains_key("tau") {
            fields.push(FieldConfig {
                key: "tau".into(),
                name: "tau".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        vec![Extract::CelestialMap {
            arr_path: "data".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: dist_key.into(),
            dist_scale: 1.0,
            plx_key: plx_key.into(),
            z_key: z_key.into(),
            pmra_key: pmra_key.into(),
            pmdec_key: pmdec_key.into(),
            rv_key: rv_key.into(),
            rv_scale: 1.0,
            epoch_key: epoch_key.into(),
            fields,
            tau_key: String::new(),
        }]
    } else if has_lat && has_lon {
        let alt_key = if first.contains_key("alt") { "alt" } else { "" };
        let epoch_key = if first.contains_key("t") { "t" } else { "" };
        let vel_key = if first.contains_key("vel") { "vel" } else { "" };
        let trk_key = if first.contains_key("trk") { "trk" } else { "" };
        let vr_key = if first.contains_key("vr") { "vr" } else { "" };
        let mut fields = vec![];
        if first.contains_key("val") {
            fields.push(FieldConfig {
                key: "val".into(),
                name: "val".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        if first.contains_key("extent") {
            fields.push(FieldConfig {
                key: "extent".into(),
                name: "extent".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        vec![Extract::Map {
            arr_path: "data".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            alt_key: alt_key.into(),
            epoch_key: epoch_key.into(),
            val_key: String::new(),
            alt_scale: -1.0,
            vel_key: vel_key.into(),
            vel_scale: 1.0,
            trk_key: trk_key.into(),
            vr_key: vr_key.into(),
            fields,
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }]
    } else {
        vec![]
    }
}



pub fn jcount(json: &JsonVal, path: &str) -> Option<f64> {
    if path == "." || path.is_empty() {
        if let JsonVal::Arr(arr) = json {
            return Some(arr.len() as f64);
        }
        return None;
    }
    if path.contains('.') {
        let target = jpath_val(json, path)?;
        if let JsonVal::Arr(arr) = target {
            return Some(arr.len() as f64);
        }
        return None;
    }
    match json {
        JsonVal::Obj(map) => {
            if let Some(JsonVal::Arr(arr)) = map.get(path) {
                Some(arr.len() as f64)
            } else {
                None
            }
        }
        _ => None,
    }
}



pub fn jdeep_find_num(json: &JsonVal, key: &str) -> Option<f64> {
    match json {
        JsonVal::Obj(map) => {
            if let Some(v) = map.get(key) {
                if let Some(n) = scalar_of(v) {
                    return Some(n);
                }
            }
            for v in map.values() {
                if let Some(n) = jdeep_find_num(v, key) {
                    return Some(n);
                }
            }
            None
        }
        JsonVal::Arr(arr) => {
            for v in arr {
                if let Some(n) = jdeep_find_num(v, key) {
                    return Some(n);
                }
            }
            None
        }
        _ => None,
    }
}



pub fn j2d_last_row(json: &JsonVal, col: &str) -> Option<f64> {
    if let JsonVal::Arr(arr) = json {
        if arr.len() < 2 {
            return None;
        }
        if let JsonVal::Arr(headers) = &arr[0] {
            let col_idx = headers.iter().position(|h| {
                if let JsonVal::Str(s) = h {
                    s.eq_ignore_ascii_case(col) || s.starts_with(col)
                } else {
                    false
                }
            })?;
            if let Some(JsonVal::Arr(last_row)) = arr.last() {
                return last_row.get(col_idx).and_then(scalar_of);
            }
        }
    }
    None
}



pub fn text_last_col(data: &str, col: &str) -> Option<f64> {
    let mut header_idx: Option<usize> = None;
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let stripped = (if let Some(s) = trimmed.strip_prefix('#') {
            s
        } else {
            trimmed
        })
        .trim();
        let cols = split_data_line(stripped);
        if header_idx.is_none() {
            if let Some(idx) = cols
                .iter()
                .position(|c| c.eq_ignore_ascii_case(col) || c.starts_with(col))
            {
                header_idx = Some(idx);
                break;
            }
            continue;
        }
    }
    let idx = header_idx?;
    for line in data.lines().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.chars().next().is_some_and(|c| c.is_alphabetic())
        {
            continue;
        }
        let cols = split_data_line(trimmed);
        if let Some(v) = cols.get(idx) {
            if let Ok(f) = v.trim_matches('"').parse::<f64>() {
                return Some(f);
            }
        }
    }
    None
}



pub fn is_drop_key(key: &str) -> bool {
    let kl = key.to_lowercase();
    kl == "id"
        || kl == "hex"
        || kl == "flight"
        || kl == "callsign"
        || kl == "icao24"
        || kl == "origin_country"
        || kl == "evid"
        || kl == "publicid"
        || kl == "locality"
        || kl == "place"
        || kl == "region"
        || kl == "flynn_region"
        || kl == "satellite"
        || kl == "net"
        || kl == "source"
        || kl == "station"
        || kl == "name"
        || kl == "stid"
        || kl == "icao"
        || kl == "station_name"
        || kl == "country"
        || kl == "sitename"
        || kl == "variablename"
        || kl == "hypocenter"
        || kl == "code"
        || kl == "wmo"
        || kl == "wban"
        || kl == "usaf"
        || kl == "buoy_id"
        || kl == "platform"
        || kl == "sensor"
        || kl == "catalog"
        || is_time_key(key)
        || kl == "timestamp_utc"
        || kl == "observed_date"
        || kl == "generated"
        || kl == "local_date_time"
        || kl == "datetime"
        || kl == "timezone"
        || kl == "origintime"
        || kl == "obstime"
        || kl == "lastupdated"
        || kl == "begintime"
        || kl == "peaktime"
        || kl == "endtime"
        || kl == "announcedtime"
        || kl == "daynum"
        || kl == "type"
        || kl == "status"
        || kl == "alert"
        || kl == "magtype"
        || kl == "evtype"
        || kl == "auth"
        || kl == "iscancel"
        || kl == "isfinal"
        || kl == "domestictsunami"
        || kl == "issea"
        || kl == "istraining"
        || kl == "active"
        || kl == "count"
        || kl == "total"
        || kl == "number_spots"
        || kl == "station_count"
        || kl == "event_count"
        || kl == "multiplicity"
        || kl.starts_with("count_")
        || kl.starts_with("number_")
        || (kl.starts_with("n_") && kl.len() <= 5)
        || kl.ends_with("_index")
        || kl.ends_with("_scale")
        || kl.ends_with("_code")
        || kl.ends_with("_pct")
        || kl == "ssn"
        || kl == "kp_index"
        || kl == "estimated_kp"
        || kl == "kp"
        || kl == "a_running"
        || kl == "uv_index"
        || kl == "weather_code"
        || kl == "cdi"
        || kl == "mmi"
        || kl == "sig"
        || kl == "felt"
        || kl == "tsunami"
        || kl == "confidence"
        || kl == "dmin"
        || kl == "nst"
        || kl == "rms"
        || kl == "gap"
        || kl == "flare_index"
        || kl == "storm_level"
        || kl == "noaa_scale"
        || kl == "class"
        || kl == "classtype"
        || kl == "sample_size"
        || kl.ends_with("_size")
}



pub fn text_to_json(text: &str) -> Option<JsonVal> {
    let header = text.lines().find_map(|line| {
        let t = line.trim();
        if !t.starts_with('#') {
            return None;
        }
        let stripped = t.trim_start_matches('#').trim();
        if stripped.is_empty() {
            return None;
        }
        let cols: Vec<String> = stripped.split_whitespace().map(|s| s.to_string()).collect();
        if cols.len() > 5 {
            Some(cols)
        } else {
            None
        }
    })?;
    let data = text.lines().find_map(|line| {
        let t = line.trim();
        if t.starts_with('#') {
            return None;
        }
        let cols: Vec<String> = t.split_whitespace().map(|s| s.to_string()).collect();
        if cols.len() >= header.len() {
            Some(cols)
        } else {
            None
        }
    })?;
    let mut obj = HashMap::new();
    for (name, value) in header[5..].iter().zip(data[5..].iter()) {
        let lower = name.to_lowercase();
        if lower == "yy" || lower == "mm" || lower == "dd" || lower == "hh" || lower == "min" {
            continue;
        }
        if is_unit_name(&lower) || is_drop_key(&lower) {
            continue;
        }
        if let Ok(n) = value.parse::<f64>() {
            obj.insert(name.clone(), JsonVal::Num(n));
        }
    }
    if obj.is_empty() {
        None
    } else {
        Some(JsonVal::Obj(obj))
    }
}



pub fn tap_to_json(val: &JsonVal) -> Option<JsonVal> {
    let obj = match val {
        JsonVal::Obj(m) => m,
        _ => return None,
    };
    let metadata = match obj.get("metadata") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let data = match obj.get("data") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let mut names: Vec<String> = Vec::new();
    for m in metadata {
        if let JsonVal::Obj(mo) = m {
            if let Some(JsonVal::Str(name)) = mo.get("name") {
                names.push(name.clone());
            }
        }
    }
    if names.is_empty() {
        return None;
    }
    let mut rows: Vec<JsonVal> = Vec::new();
    for d in data {
        if let JsonVal::Arr(row) = d {
            let mut row_map = HashMap::new();
            for (name, cell) in names.iter().zip(row.iter()) {
                row_map.insert(name.clone(), cell.clone());
            }
            rows.push(JsonVal::Obj(row_map));
        }
    }
    if rows.is_empty() {
        None
    } else {
        Some(JsonVal::Arr(rows))
    }
}



pub fn tdb_to_jd(tdb_secs: f64) -> f64 {
    tdb_secs / 86400.0 + J2000_EPOCH
}



pub fn flatten_geojson_coords(val: &[JsonVal]) -> Vec<(f64, f64, Option<f64>)> {
    if let Some(JsonVal::Num(_)) = val.first() {
        if val.len() >= 2 {
            if let (Some(lon), Some(lat)) = (scalar_of(&val[0]), scalar_of(&val[1])) {
                let z = if val.len() >= 3 {
                    scalar_of(&val[2])
                } else {
                    None
                };
                return vec![(lon, lat, z)];
            }
        }
        return Vec::new();
    }
    let mut result = Vec::new();
    for v in val {
        if let JsonVal::Arr(inner) = v {
            result.extend(flatten_geojson_coords(inner));
        }
    }
    result
}



pub fn split_data_line(line: &str) -> Vec<&str> {
    if line.contains('|') && line.split('|').count() > 2 {
        line.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains('\t') && line.split('\t').count() > 2 {
        line.split('\t')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains(';') {
        line.split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains(',') && line.split(',').count() > 2 {
        line.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        line.split_whitespace().collect()
    }
}



pub enum ExtractResult {
    Measurements(Vec<(Channel, FieldConfig)>),
    WithEphemeris(Vec<(Channel, FieldConfig)>, BodyEphemeris),
}



pub fn extract(src: &SourceConfig, body: &str, now: f64, lsk: &LeapSeconds) -> ExtractResult {
    if src.format == "ephemeris_binary" {
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        if let Some(eph) = parse_ephemeris_binary(&buf) {
            return ExtractResult::WithEphemeris(vec![], eph);
        }
        return ExtractResult::Measurements(vec![]);
    }
    if src.format == "orbit_bin" {
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        if let Some(records) = crate::wind_orbit::parse_bin(&buf) {
            let rec = std::sync::Arc::new(crate::wind_orbit::orbit_rec(&records));
            return ExtractResult::WithEphemeris(
                vec![],
                BodyEphemeris {
                    granules: Vec::new(),
                    rotation_matrices: Vec::new(),
                    props: None,
                    orbit: Some(rec),
                },
            );
        }
        return ExtractResult::Measurements(vec![]);
    }
    let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
    let mut extracted: HashMap<String, f64> = HashMap::new();
    let parsed_json = if src.format == "csv_zip" {
        std::fs::read(body)
            .ok()
            .and_then(|b| unzip(&b))
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .as_deref()
            .and_then(csv_to_json)
    } else if src.format == "csv" {
        csv_to_json(body)
    } else if src.format == "free text" {
        text_to_json(body)
    } else if src.format == "tap" {
        parse_json(body).and_then(|j| tap_to_json(&j))
    } else if src.format == "json" || src.format.is_empty() || src.format == "universal" {
        let body = body
            .strip_prefix("OK")
            .and_then(|r| r.strip_prefix('\n').or_else(|| r.strip_prefix("\r\n")))
            .unwrap_or(body);
        parse_json(body)
    } else {
        None
    };
    let auto_extracts: Option<Vec<Extract>>;
    let effective_extracts: &[Extract] = if src.format == "universal" && src.extracts.is_empty() {
        if let Some(ref j) = parsed_json {
            auto_extracts = Some(universal_auto_detect(j));
            if let Some(ref auto) = auto_extracts {
                auto.as_slice()
            } else {
                return ExtractResult::Measurements(vec![]);
            }
        } else {
            return ExtractResult::Measurements(vec![]);
        }
    } else {
        &src.extracts
    };
    for ext in effective_extracts {
        match ext {
            Extract::Field(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jnum(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::First(fc, filter) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jfirst_where(j, &fc.key, filter.as_ref()) {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::Last(fc, filter) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jlast_where(j, &fc.key, filter.as_ref()) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if fc.key == "line" {
                    if let Some(v) = body
                        .lines()
                        .rev()
                        .filter(|l| {
                            let t = l.trim();
                            !t.is_empty() && !t.starts_with('#')
                        })
                        .find_map(|l| {
                            split_data_line(l)
                                .last()
                                .and_then(|c| c.trim_matches('"').parse::<f64>().ok())
                        })
                    {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::Count(fc) => {
                let v = if src.format == "csv" || fc.key == "lines" {
                    Some(
                        body.lines()
                            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                            .count() as f64,
                    )
                } else {
                    parsed_json.as_ref().and_then(|j| jcount(j, &fc.key))
                };
                if let Some(v) = v {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::LastRow(fc) => {
                if src.format == "csv" {
                    if let Some(v) = text_last_col(body, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if let Some(ref j) = parsed_json {
                    if let Some(v) = j2d_last_row(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if let Some(v) = text_last_col(body, &fc.key) {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::Path(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jpath(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::Deep(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jdeep_find_num(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::LastLine(n) => {
                if let Some(v) = body
                    .lines()
                    .filter(|l| {
                        let t = l.trim();
                        !t.is_empty() && !t.starts_with('#')
                    })
                    .last()
                    .and_then(|line| {
                        split_data_line(line)
                            .into_iter()
                            .filter_map(|t| t.parse::<f64>().ok())
                            .last()
                    })
                {
                    extracted.insert(n.clone(), v);
                }
            }
            Extract::ObjLast(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(obj) = jpath_val(j, &fc.key) {
                        if let JsonVal::Obj(m) = obj {
                            if let Some(last_key) = m.keys().max_by(|a, b| {
                                if let (Ok(ka), Ok(kb)) = (a.parse::<i64>(), b.parse::<i64>()) {
                                    ka.cmp(&kb)
                                } else {
                                    a.cmp(b)
                                }
                            }) {
                                if let Some(val) = m.get(last_key).and_then(scalar_of) {
                                    extracted.insert(fc.name.clone(), val);
                                }
                            }
                        }
                    }
                }
            }
            Extract::Regex(fc) => {
                if let Some(v) = extract_regex_val(body, &fc.key) {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::XmlCount(tag, n) => {
                let count = body.matches(&format!("<{}>", tag)).count() as f64;
                extracted.insert(n.clone(), count);
            }
            Extract::LastObj(fk, fv, ek, n) => {
                if let Some(ref j) = parsed_json {
                    if let JsonVal::Arr(arr) = j {
                        for v in arr.iter().rev() {
                            if let JsonVal::Obj(o) = v {
                                if let Some(JsonVal::Str(s)) = o.get(fk) {
                                    if s == fv {
                                        if let Some(val) = jnum(v, ek) {
                                            extracted.insert(n.clone(), val);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Extract::Map {
                arr_path,
                lat_key,
                lon_key,
                alt_key,
                epoch_key,
                val_key,
                alt_scale,
                vel_key,
                vel_scale,
                trk_key,
                vr_key,
                fields,
                lat_sign,
                lon_sign,
                epoch_scale,
                tau_key,
                mag_type_key,
            } => {
                let eff_lat_key = lat_key.clone();
                let eff_lon_key = lon_key.clone();
                let eff_epoch_key = epoch_key.clone();
                if let Some(ref j) = parsed_json {
                    let rows: Vec<&JsonVal> = match jpath_val(j, arr_path) {
                        Some(JsonVal::Arr(arr)) => arr.iter().collect(),
                        Some(obj @ JsonVal::Obj(_)) => vec![obj],
                        _ => Vec::new(),
                    };
                    {
                        for v in rows {
                            let lat = jpath(v, &eff_lat_key);
                            let lon = jpath(v, &eff_lon_key);
                            let alt = if alt_key.is_empty() {
                                Some(0.0)
                            } else {
                                jpath(v, alt_key).map(|a| a * alt_scale)
                            };
                            if let (Some(la), Some(lo), Some(al)) = (lat, lon, alt) {
                                let mut lat_val = la;
                                if let Some(sign_key) = lat_sign {
                                    if let Some(vv) = jpath_val(v, sign_key) {
                                        if let JsonVal::Str(s) = vv {
                                            if s.contains('S') || s.contains('s') {
                                                lat_val = -la;
                                            }
                                        }
                                    }
                                }
                                let mut lon_val = lo;
                                if let Some(sign_key) = lon_sign {
                                    if let Some(vv) = jpath_val(v, sign_key) {
                                        if let JsonVal::Str(s) = vv {
                                            if s.contains('W') || s.contains('w') {
                                                lon_val = -lo;
                                            }
                                        }
                                    }
                                }
                                let speed = if vel_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, vel_key).map(|s| s * vel_scale)
                                };
                                let track = if trk_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, trk_key)
                                };
                                let vrate = if vr_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, vr_key).map(|s| s * vel_scale)
                                };
                                let position = if let (Some(sp), Some(tr)) = (speed, track) {
                                    Position::SurfaceFlow {
                                        body_name: frame_body_name(&src.frame),
                                        lat: lat_val,
                                        lon: lon_val,
                                        alt: al,
                                        speed: sp,
                                        track: tr,
                                        vrate,
                                    }
                                } else {
                                    Position::Surface {
                                        body_name: frame_body_name(&src.frame),
                                        lat: lat_val,
                                        lon: lon_val,
                                        alt: al,
                                    }
                                };
                                let epoch = if eff_epoch_key.is_empty() {
                                    now
                                } else if let Some(ev) = jpath_val(v, &eff_epoch_key) {
                                    match ev {
                                        JsonVal::Str(s) => {
                                            if let Some(t) = parse_iso_tdb(s, lsk) {
                                                t
                                            } else {
                                                continue;
                                            }
                                        }
                                        JsonVal::Num(n) => {
                                            match lsk.unix_to_tdb(*n * epoch_scale) {
                                                Some(t) => t,
                                                None => continue,
                                            }
                                        }
                                        _ => continue,
                                    }
                                } else {
                                    continue;
                                };
                                let row_tau: Option<f64> = if tau_key.is_empty() {
                                    None
                                } else {
                                    match jpath(v, tau_key) {
                                        Some(t) if t > 0.0 => Some(t),
                                        Some(_) => continue,
                                        None => None,
                                    }
                                };
                                for fc in fields {
                                    if !val_key.is_empty() && fc.name != *val_key {
                                        continue;
                                    }
                                    let mut raw = jpath(v, &fc.key);
                                    if !mag_type_key.is_empty()
                                        && fc.unit.eq_ignore_ascii_case("mw")
                                    {
                                        if let Some(t) = jstr(v, &mag_type_key) {
                                            if !is_moment_magnitude(&t) {
                                                continue;
                                            }
                                        }
                                    }
                                    let mut transformed = false;
                                    if let Some((op, key_b)) = &fc.fold {
                                        raw = fold_value(raw, jpath(v, key_b), *op);
                                    } else if let Some(ref mag_key) = src.flux_from_mag {
                                        if fc.key == *mag_key {
                                            raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                            transformed = true;
                                        }
                                    }
                                    let val = match raw {
                                        Some(vv) => vv,
                                        None => continue,
                                    };
                                    if !val.is_finite() {
                                        continue;
                                    }
                                    let mut eff_fc = (*fc).clone();
                                    if transformed {
                                        eff_fc.unit.clear();
                                    }
                                    if let Some(t) = row_tau {
                                        eff_fc.tau = t;
                                    }
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch,
                                            position: position.clone(),
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        eff_fc,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::ProfileMap {
                arr_path,
                lat_key,
                lon_key,
                epoch_key,
                pressure_var,
                pressure_scale,
                fields,
            } => {
                if let Some(ref j) = parsed_json {
                    let rows: Vec<&JsonVal> = match jpath_val(j, arr_path) {
                        Some(JsonVal::Arr(arr)) => arr.iter().collect(),
                        Some(obj @ JsonVal::Obj(_)) => vec![obj],
                        _ => Vec::new(),
                    };
                    for v in rows {
                        let lat = jpath(v, &lat_key);
                        let lon = jpath(v, &lon_key);
                        if let (Some(la), Some(lo)) = (lat, lon) {
                            let epoch = if epoch_key.is_empty() {
                                now
                            } else if let Some(ev) = jpath_val(v, &epoch_key) {
                                match ev {
                                    JsonVal::Str(s) => {
                                        if let Some(t) = parse_iso_tdb(s, lsk) {
                                            t
                                        } else {
                                            continue;
                                        }
                                    }
                                    JsonVal::Num(n) => match lsk.unix_to_tdb(*n) {
                                        Some(t) => t,
                                        None => continue,
                                    },
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let data = match jpath_val(v, "data") {
                                Some(JsonVal::Arr(d)) => d,
                                _ => continue,
                            };
                            let (var_names, pressure_idx) = match jpath_val(v, "data_info") {
                                Some(JsonVal::Arr(info)) => {
                                    let names: Vec<String> = match info.first() {
                                        Some(JsonVal::Arr(n)) => n
                                            .iter()
                                            .filter_map(|e| match e {
                                                JsonVal::Str(s) => Some(s.clone()),
                                                _ => None,
                                            })
                                            .collect(),
                                        _ => Vec::new(),
                                    };
                                    let pidx = names.iter().position(|n| n == pressure_var);
                                    (names, pidx)
                                }
                                _ => (Vec::new(), None),
                            };
                            let pidx = match pressure_idx {
                                Some(i) => i,
                                None => continue,
                            };
                            let pressure = match data.get(pidx) {
                                Some(JsonVal::Arr(p)) => p,
                                _ => continue,
                            };
                            let n_levels = pressure.len();
                            for fc in fields {
                                let vidx = match var_names.iter().position(|n| *n == fc.key) {
                                    Some(i) => i,
                                    None => continue,
                                };
                                let values = match data.get(vidx) {
                                    Some(JsonVal::Arr(a)) => a,
                                    _ => continue,
                                };
                                for k in 0..n_levels {
                                    let p = match pressure.get(k) {
                                        Some(JsonVal::Num(x)) => *x,
                                        _ => continue,
                                    };
                                    let val = match values.get(k) {
                                        Some(JsonVal::Num(x)) => *x,
                                        _ => continue,
                                    };
                                    if !val.is_finite() {
                                        continue;
                                    }
                                    let position = Position::Surface {
                                        body_name: frame_body_name(&src.frame),
                                        lat: la,
                                        lon: lo,
                                        alt: -p * pressure_scale,
                                    };
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch,
                                            position,
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        fc.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::Flatten {
                arr_path,
                geom_path,
                epoch_key,
                fields,
            } => {
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for v in arr.iter() {
                            let coords = if geom_path.is_empty() {
                                match jpath_val(v, "coordinates") {
                                    Some(JsonVal::Arr(c)) => c,
                                    _ => match v {
                                        JsonVal::Arr(c) => c,
                                        _ => continue,
                                    },
                                }
                            } else {
                                let geom = match jpath_val(v, geom_path) {
                                    Some(g) => g,
                                    None => continue,
                                };
                                match jpath_val(geom, "coordinates") {
                                    Some(JsonVal::Arr(c)) => c,
                                    _ => match geom {
                                        JsonVal::Arr(c) => c,
                                        _ => continue,
                                    },
                                }
                            };
                            let vertices = flatten_geojson_coords(coords);
                            if vertices.is_empty() {
                                continue;
                            }
                            let row_epoch = if !epoch_key.is_empty() {
                                match jpath(v, epoch_key) {
                                    Some(ev) => ev,
                                    None => continue,
                                }
                            } else {
                                continue;
                            };
                            for (lon, lat, z) in vertices {
                                let position = Position::Surface {
                                    body_name: frame_body_name(&src.frame),
                                    lat,
                                    lon,
                                    alt: match z {
                                        Some(a) => a,
                                        None => continue,
                                    },
                                };
                                for fc in fields {
                                    let mut raw = jpath(v, &fc.key);
                                    let mut transformed = false;
                                    if let Some((op, key_b)) = &fc.fold {
                                        raw = fold_value(raw, jpath(v, key_b), *op);
                                    } else if let Some(ref mag_key) = src.flux_from_mag {
                                        if fc.key == *mag_key {
                                            raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                            transformed = true;
                                        }
                                    }
                                    let val = match raw {
                                        Some(vv) => vv,
                                        None => continue,
                                    };
                                    if !val.is_finite() {
                                        continue;
                                    }
                                    let mut eff_fc = (*fc).clone();
                                    if transformed {
                                        eff_fc.unit.clear();
                                    }
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch: row_epoch,
                                            position: position.clone(),
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        eff_fc,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::CmrPolygon {
                arr_path,
                fields,
                epoch_key,
                alt_key,
                val_key,
            } => {
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for v in arr.iter() {
                            let polys = match jpath_val(v, "polygons") {
                                Some(JsonVal::Arr(p)) => p,
                                _ => continue,
                            };
                            let mut vertices: Vec<(f64, f64)> = Vec::new();
                            for ring_list in polys {
                                if let JsonVal::Arr(rings) = ring_list {
                                    for ring_str_val in rings {
                                        if let JsonVal::Str(s) = ring_str_val {
                                            let nums: Vec<f64> = s
                                                .split_whitespace()
                                                .filter_map(|n| n.parse().ok())
                                                .collect();
                                            for pair in nums.chunks(2) {
                                                if pair.len() == 2 {
                                                    vertices.push((pair[1], pair[0]));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if vertices.is_empty() {
                                continue;
                            }
                            let epoch = if epoch_key.is_empty() {
                                continue;
                            } else if let Some(ev) = jpath_val(v, epoch_key) {
                                match ev {
                                    JsonVal::Str(s) => {
                                        if let Some(t) = parse_iso_tdb(s, lsk) {
                                            t
                                        } else {
                                            continue;
                                        }
                                    }
                                    JsonVal::Num(n) => match lsk.unix_to_tdb(*n) {
                                        Some(t) => t,
                                        None => continue,
                                    },
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let alt = match alt_key {
                                k if k.is_empty() => continue,
                                _ => match jpath(v, alt_key) {
                                    Some(a) => a,
                                    None => continue,
                                },
                            };
                            for fc in fields {
                                if !val_key.is_empty() && fc.name != *val_key {
                                    continue;
                                }
                                let mut raw = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        transformed = true;
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                for (lon, lat) in vertices.iter() {
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch,
                                            position: Position::Surface {
                                                body_name: frame_body_name(&src.frame),
                                                lat: *lat,
                                                lon: *lon,
                                                alt,
                                            },
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        eff_fc.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::CelestialPolygon {
                arr_path,
                radius,
                fields,
                epoch_key,
                val_key,
            } => {
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for v in arr.iter() {
                            let geom = match jpath_val(v, "geometry") {
                                Some(g) => g,
                                None => continue,
                            };
                            let coords = match jpath_val(geom, "coordinates") {
                                Some(JsonVal::Arr(c)) => c,
                                _ => continue,
                            };
                            let vertices = flatten_geojson_coords(coords);
                            if vertices.is_empty() || *radius <= 0.0 {
                                continue;
                            }
                            let row_epoch = if !epoch_key.is_empty() {
                                match jpath(v, epoch_key) {
                                    Some(ev) => ev,
                                    None => continue,
                                }
                            } else {
                                continue;
                            };
                            for fc in fields {
                                if !val_key.is_empty() && fc.name != *val_key {
                                    continue;
                                }
                                let mut raw = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        transformed = true;
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                for (ra_deg, dec_deg, _z) in &vertices {
                                    let ra = ra_deg.to_radians();
                                    let dec = dec_deg.to_radians();
                                    let (sa, ca) = ra.sin_cos();
                                    let (sd, cd) = dec.sin_cos();
                                    let p = [cd * ca * radius, cd * sa * radius, sd * radius];
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch: row_epoch,
                                            position: Position::StateVector {
                                                p,
                                                v: [0.0, 0.0, 0.0],
                                                track: false,
                                            },
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        eff_fc.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::Rows {
                last_line,
                fields,
                tau_key,
            } => {
                if let Frame::Surface { lat, lon, alt, .. } = src.frame {
                    let position = Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat,
                        lon,
                        alt,
                    };
                    let resolve_col = |key: &str| -> Option<usize> {
                        if let Ok(idx) = key.parse::<usize>() {
                            return Some(idx);
                        }
                        body.lines().find_map(|line| {
                            let t = line.trim();
                            if t.is_empty() {
                                return None;
                            }
                            let s = t.strip_prefix('#').unwrap_or(t).trim();
                            split_data_line(s)
                                .iter()
                                .position(|c| c.eq_ignore_ascii_case(key) || c.starts_with(key))
                        })
                    };
                    let col_fcs: Vec<(usize, Option<usize>, &FieldConfig)> = fields
                        .iter()
                        .filter_map(|fc| {
                            let idx = resolve_col(&fc.key)?;
                            let idx_b = match &fc.fold {
                                Some((_, kb)) => Some(resolve_col(kb)?),
                                None => None,
                            };
                            Some((idx, idx_b, fc))
                        })
                        .collect();
                    let tau_col = if tau_key.is_empty() {
                        None
                    } else {
                        resolve_col(&tau_key)
                    };
                    let lines: Vec<&str> = if *last_line {
                        body.lines()
                            .rev()
                            .find(|l| {
                                let t = l.trim();
                                !t.is_empty() && !t.starts_with('#')
                            })
                            .into_iter()
                            .collect()
                    } else {
                        body.lines()
                            .filter(|l| {
                                let t = l.trim();
                                !t.is_empty() && !t.starts_with('#')
                            })
                            .collect()
                    };
                    for line in lines {
                        let cols = split_data_line(line.trim());
                        let row_tau: Option<f64> = match tau_col {
                            None => None,
                            Some(idx) => match cols
                                .get(idx)
                                .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok())
                            {
                                Some(t) if t > 0.0 => Some(t),
                                Some(_) => continue,
                                None => None,
                            },
                        };
                        for (idx, idx_b, fc) in &col_fcs {
                            let raw = cols
                                .get(*idx)
                                .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok());
                            let val = match (&fc.fold, idx_b) {
                                (Some((op, _)), Some(bi)) => fold_value(
                                    raw,
                                    cols.get(*bi).and_then(|s| {
                                        s.trim().trim_matches('"').parse::<f64>().ok()
                                    }),
                                    *op,
                                ),
                                _ => raw,
                            };
                            let val = match val {
                                Some(v) => v,
                                None => continue,
                            };
                            if !val.is_finite() {
                                continue;
                            }
                            let mut eff_fc = (*fc).clone();
                            if let Some(t) = row_tau {
                                eff_fc.tau = t;
                            }
                            channels.push((
                                Channel {
                                    z: 0.0,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch: now,
                                    position: position.clone(),
                                    name: fc.name.clone(),
                                    value: val,
                                },
                                eff_fc,
                            ));
                        }
                    }
                }
            }
            Extract::KeplerMap {
                arr_path,
                a_key,
                e_key,
                i_key,
                om_key,
                w_key,
                ma_key,
                epoch_key,
                q_key,
                tp_key,
                fields,
            } => {
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        let jd_now = tdb_to_jd(now);
                        for v in arr.iter() {
                            let (Some(e_val), Some(i_val), Some(om_val), Some(w_val)) = (
                                jpath(v, e_key),
                                jpath(v, i_key),
                                jpath(v, om_key),
                                jpath(v, w_key),
                            ) else {
                                continue;
                            };
                            if !(0.0..1.0).contains(&e_val) {
                                continue;
                            }
                            let (Some(epoch_val),) = (jpath(v, epoch_key),) else {
                                continue;
                            };
                            let a_au = if !a_key.is_empty() {
                                match jpath(v, a_key) {
                                    Some(a) if a > 0.0 => a,
                                    _ => continue,
                                }
                            } else if !q_key.is_empty() {
                                match jpath(v, q_key) {
                                    Some(q) if q > 0.0 => q / (1.0 - e_val),
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let ma_deg = if !ma_key.is_empty() {
                                match jpath(v, ma_key) {
                                    Some(m) => m,
                                    None => continue,
                                }
                            } else if !tp_key.is_empty() {
                                let Some(tp) = jpath(v, tp_key) else {
                                    continue;
                                };
                                let n_deg_day = GAUSS_K / (a_au * a_au * a_au).sqrt()
                                    * (180.0 / std::f64::consts::PI);
                                n_deg_day * (epoch_val - tp)
                            } else {
                                continue;
                            };
                            let (p, vel) = match crate::kepler::elements_to_icrs_state(
                                a_au, e_val, i_val, om_val, w_val, ma_deg, epoch_val, jd_now,
                            ) {
                                Some(st) => st,
                                None => continue,
                            };
                            for fc in fields {
                                let mut raw = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        transformed = true;
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: now,
                                        position: Position::StateVector {
                                            p,
                                            v: vel,
                                            track: false,
                                        },
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    eff_fc,
                                ));
                            }
                        }
                    }
                }
            }
            Extract::CelestialMap {
                arr_path,
                ra_key,
                dec_key,
                dist_key,
                dist_scale,
                plx_key,
                z_key,
                pmra_key,
                pmdec_key,
                rv_key,
                rv_scale,
                epoch_key,
                fields,
                tau_key,
            } => {
                let default_epoch = if let Some(e) = src.catalog_epoch {
                    e
                } else {
                    now
                };
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for v in arr.iter() {
                            let (Some(ra_deg), Some(dec_deg)) =
                                (jpath(v, ra_key), jpath(v, dec_key))
                            else {
                                continue;
                            };
                            let d = if !plx_key.is_empty() {
                                match jpath(v, plx_key) {
                                    Some(plx) if plx.is_finite() && plx > 0.0 => {
                                        PARSEC_M * 1000.0 / plx
                                    }
                                    _ => {
                                        if !z_key.is_empty() {
                                            match jpath(v, z_key) {
                                                Some(z) if z.is_finite() && z > 0.0 => {
                                                    z * C_LIGHT / HUBBLE_H0
                                                }
                                                _ => {
                                                    if !dist_key.is_empty() {
                                                        match jpath(v, dist_key) {
                                                            Some(dd)
                                                                if dd.is_finite() && dd > 0.0 =>
                                                            {
                                                                dd * dist_scale
                                                            }
                                                            _ => continue,
                                                        }
                                                    } else {
                                                        continue;
                                                    }
                                                }
                                            }
                                        } else if !dist_key.is_empty() {
                                            match jpath(v, dist_key) {
                                                Some(dd) if dd.is_finite() && dd > 0.0 => {
                                                    dd * dist_scale
                                                }
                                                _ => continue,
                                            }
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            } else if !dist_key.is_empty() {
                                match jpath(v, dist_key) {
                                    Some(dd) if dd.is_finite() && dd > 0.0 => dd * dist_scale,
                                    _ => {
                                        if !z_key.is_empty() {
                                            match jpath(v, z_key) {
                                                Some(z) if z.is_finite() && z > 0.0 => {
                                                    z * C_LIGHT / HUBBLE_H0
                                                }
                                                _ => continue,
                                            }
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            } else if !z_key.is_empty() {
                                match jpath(v, z_key) {
                                    Some(z) if z.is_finite() && z > 0.0 => z * C_LIGHT / HUBBLE_H0,
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let zval = if !z_key.is_empty() {
                                jpath(v, z_key).filter(|z| z.is_finite() && *z > 0.0)
                            } else {
                                None
                            };
                            let ra = ra_deg.to_radians();
                            let dec = dec_deg.to_radians();
                            let (sa, ca) = ra.sin_cos();
                            let (sd, cd) = dec.sin_cos();
                            let p_hat = [cd * ca, cd * sa, sd];
                            let p = [p_hat[0] * d, p_hat[1] * d, p_hat[2] * d];
                            let mu_a = if pmra_key.is_empty() {
                                None
                            } else {
                                jpath(v, pmra_key)
                                    .filter(|x| x.is_finite())
                                    .map(|v| v * MAS_YR_TO_RAD_S)
                            };
                            let mu_d = if pmdec_key.is_empty() {
                                None
                            } else {
                                jpath(v, pmdec_key)
                                    .filter(|x| x.is_finite())
                                    .map(|v| v * MAS_YR_TO_RAD_S)
                            };
                            let vr = if rv_key.is_empty() {
                                None
                            } else {
                                jpath(v, rv_key)
                                    .filter(|x| x.is_finite())
                                    .map(|v| v * rv_scale)
                            };
                            let a_hat = [-sa, ca, 0.0];
                            let d_hat = [-sd * ca, -sd * sa, cd];
                            let vel = [
                                d * (mu_a.map_or(0.0, |m| m * a_hat[0])
                                    + mu_d.map_or(0.0, |m| m * d_hat[0]))
                                    + vr.map_or(0.0, |v| v * p_hat[0]),
                                d * (mu_a.map_or(0.0, |m| m * a_hat[1])
                                    + mu_d.map_or(0.0, |m| m * d_hat[1]))
                                    + vr.map_or(0.0, |v| v * p_hat[1]),
                                d * (mu_a.map_or(0.0, |m| m * a_hat[2])
                                    + mu_d.map_or(0.0, |m| m * d_hat[2]))
                                    + vr.map_or(0.0, |v| v * p_hat[2]),
                            ];
                            let sample_epoch = if !epoch_key.is_empty() {
                                if let Some(v) = jpath(v, epoch_key) {
                                    v
                                } else {
                                    continue;
                                }
                            } else {
                                default_epoch
                            };
                            let row_tau: Option<f64> = if tau_key.is_empty() {
                                None
                            } else {
                                match jpath(v, tau_key) {
                                    Some(t) if t > 0.0 => Some(t),
                                    Some(_) => continue,
                                    None => None,
                                }
                            };
                            for fc in fields {
                                let mut raw: Option<f64> = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some((op, key_b)) = &fc.fold {
                                    raw = fold_value(raw, jpath(v, key_b), *op);
                                } else if let Some(ref mag_field) = src.abs_mag_from {
                                    if fc.name == *mag_field {
                                        raw = raw.map(|v| {
                                            let dist_pc = d / PARSEC_M;
                                            let abs_m = v - 5.0 * (dist_pc / 10.0).log10();
                                            10.0f64.powf(-0.4 * abs_m)
                                        });
                                        transformed = true;
                                    }
                                } else if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        transformed = true;
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                if let Some(t) = row_tau {
                                    eff_fc.tau = t;
                                }
                                channels.push((
                                    Channel {
                                        z: zval.unwrap_or(0.0),
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: sample_epoch,
                                        position: Position::StateVector {
                                            p,
                                            v: vel,
                                            track: false,
                                        },
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    eff_fc,
                                ));
                            }
                        }
                    }
                }
            }
            Extract::GeojsonEvents {
                mag_key,
                min_mag,
                outputs,
                tau,
                absorption,
                advection,
                mag_type_key,
            } => {
                if outputs.len() >= 2 {
                    if let Some(ref j) = parsed_json {
                        if let JsonVal::Obj(root) = j {
                            if let Some(JsonVal::Arr(features)) = root.get("features") {
                                for feat in features {
                                    if let JsonVal::Obj(f) = feat {
                                        let mut elo = 0.0;
                                        let mut ela = 0.0;
                                        let mut ed = 0.0;
                                        let mut mag: Option<f64> = None;
                                        let mut valid = false;
                                        if let Some(JsonVal::Obj(geom)) = f.get("geometry") {
                                            if let Some(JsonVal::Arr(c)) = geom.get("coordinates") {
                                                if c.len() >= 3 {
                                                    if let JsonVal::Num(n) = c[0] {
                                                        elo = n;
                                                    }
                                                    if let JsonVal::Num(n) = c[1] {
                                                        ela = n;
                                                    }
                                                    if let JsonVal::Num(n) = c[2] {
                                                        ed = n;
                                                    }
                                                    valid = true;
                                                }
                                            }
                                        }
                                        if valid {
                                            if let Some(props) = f.get("properties") {
                                                if let Some(m) = jnum(props, mag_key) {
                                                    if m.is_finite() {
                                                        mag = Some(m);
                                                    }
                                                }
                                                if !mag_type_key.is_empty() {
                                                    if let Some(t) = jstr(props, &mag_type_key) {
                                                        if !is_moment_magnitude(&t) {
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        if let Some(mag) = mag {
                                            if mag >= *min_mag {
                                                channels.push((
                                                    Channel {
                                                        z: 0.0,
                                                        freq: 0.0,
                                                        bin_width: 0.0,
                                                        epoch: now,
                                                        position: Position::Surface {
                                                            body_name: frame_body_name(&src.frame),
                                                            lat: ela,
                                                            lon: elo,
                                                            alt: -ed * 1000.0,
                                                        },
                                                        name: outputs[0].clone(),
                                                        value: mag,
                                                    },
                                                    FieldConfig {
                                                        key: outputs[0].clone(),
                                                        name: outputs[0].clone(),
                                                        kernel: 0,
                                                        force: 3,
                                                        tau: *tau,
                                                        absorption: *absorption,
                                                        advection: *advection,
                                                        unit: "Mw".to_string(),
                                                        fold: None,
                                                    },
                                                ));
                                                channels.push((
                                                    Channel {
                                                        z: 0.0,
                                                        freq: 0.0,
                                                        bin_width: 0.0,
                                                        epoch: now,
                                                        position: Position::Surface {
                                                            body_name: frame_body_name(&src.frame),
                                                            lat: ela,
                                                            lon: elo,
                                                            alt: -ed * 1000.0,
                                                        },
                                                        name: outputs[1].clone(),
                                                        value: ed * 1000.0,
                                                    },
                                                    FieldConfig {
                                                        key: outputs[1].clone(),
                                                        name: outputs[1].clone(),
                                                        kernel: 0,
                                                        force: 3,
                                                        tau: *tau,
                                                        absorption: *absorption,
                                                        advection: *advection,
                                                        unit: String::new(),
                                                        fold: None,
                                                    },
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Extract::Hapi(pairs) => {
                if let Some(ref j) = parsed_json {
                    if let JsonVal::Obj(root) = j {
                        if let Some(JsonVal::Arr(data)) = root.get("data") {
                            let mut col: HashMap<String, usize> = HashMap::new();
                            let mut fill_of: HashMap<String, f64> = HashMap::new();
                            let mut has_params = false;
                            if let Some(JsonVal::Arr(params)) = root.get("parameters") {
                                for (i, p) in params.iter().enumerate() {
                                    if let JsonVal::Obj(po) = p {
                                        if let Some(JsonVal::Str(nn)) = po.get("name") {
                                            col.insert(nn.clone(), i);
                                            has_params = true;
                                            if let Some(fv) = po.get("fill").and_then(scalar_of) {
                                                fill_of.insert(nn.clone(), fv);
                                            }
                                        }
                                    }
                                }
                            }
                            for (k, v) in &src.hapi_fill {
                                fill_of.entry(k.clone()).or_insert(*v);
                            }
                            if !has_params {
                                if pairs.len() == 1 && !pairs[0].0.contains('.') {
                                    if let Some(JsonVal::Arr(row)) = data.last() {
                                        if let Some(val) = row.last().and_then(scalar_of) {
                                            if fill_of
                                                .get(pairs[0].0.as_str())
                                                .map_or(true, |&f| val != f)
                                            {
                                                extracted.insert(pairs[0].1.clone(), val);
                                            }
                                        }
                                    }
                                    continue;
                                }
                                let mut next_col = 0usize;
                                for (param, _) in pairs.iter() {
                                    let base = param.split('.').next().unwrap_or(param);
                                    if !col.contains_key(base) {
                                        next_col += 1;
                                        col.insert(base.to_string(), next_col);
                                    }
                                }
                            }
                            if let Some(last_row) = data.last() {
                                if let JsonVal::Arr(row) = last_row {
                                    for (param, name) in pairs {
                                        let (base, comp) = match param.rfind('.') {
                                            Some(dot)
                                                if param[dot + 1..]
                                                    .chars()
                                                    .all(|c| c.is_ascii_digit()) =>
                                            {
                                                (
                                                    &param[..dot],
                                                    param[dot + 1..].parse::<usize>().ok(),
                                                )
                                            }
                                            _ => (param.as_str(), None),
                                        };
                                        if let Some(&idx) = col.get(base) {
                                            let v = match comp {
                                                Some(i) => row.get(idx).and_then(|cell| {
                                                    if let JsonVal::Arr(a) = cell {
                                                        a.get(i).and_then(scalar_of)
                                                    } else {
                                                        None
                                                    }
                                                }),
                                                None => row.get(idx).and_then(scalar_of),
                                            };
                                            if let Some(val) = v {
                                                if fill_of.get(base).map_or(true, |&f| val != f) {
                                                    extracted.insert(name.clone(), val);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Extract::Alerce(_) => {}
        }
    }
    if !extracted.is_empty() {
        for (name, val) in &extracted {
            let fc = effective_extracts.iter().find_map(|ext| match ext {
                Extract::Field(fc)
                | Extract::First(fc, _)
                | Extract::Last(fc, _)
                | Extract::Count(fc)
                | Extract::LastRow(fc)
                | Extract::ObjLast(fc)
                | Extract::Path(fc)
                | Extract::Deep(fc)
                | Extract::Regex(fc) => {
                    if fc.name == *name && fc.tau > 0.0 {
                        Some(fc)
                    } else {
                        None
                    }
                }
                _ => None,
            });
            if let Some(fc) = fc {
                let mut raw = Some(*val);
                let mut transformed = false;
                if let Some(ref mag_key) = src.flux_from_mag {
                    if fc.key == *mag_key {
                        raw = raw.map(|v| 10.0f64.powf(-0.4 * v));
                        transformed = true;
                    }
                }
                let val = match raw {
                    Some(v) => v,
                    None => continue,
                };
                if !val.is_finite() {
                    continue;
                }
                let mut eff_fc = fc.clone();
                if transformed {
                    eff_fc.unit.clear();
                }
                channels.push((
                    Channel {
                        z: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        epoch: now,
                        position: Position::Source,
                        name: fc.name.clone(),
                        value: val,
                    },
                    eff_fc,
                ));
            }
        }
    }
    channels.retain(|(c, _)| c.value.is_finite());
    ExtractResult::Measurements(channels)
}



pub fn series_epoch_of(el: &JsonVal, lsk: &LeapSeconds) -> Option<f64> {
    match el {
        JsonVal::Obj(map) => {
            for (k, v) in map {
                if !is_time_key(k) {
                    continue;
                }
                match v {
                    JsonVal::Str(s) => {
                        if let Some(t) = parse_iso_tdb(s, lsk) {
                            return Some(t);
                        }
                    }
                    JsonVal::Num(n) => {
                        if let Some(t) = lsk.unix_to_tdb(*n) {
                            return Some(t);
                        }
                    }
                    _ => {}
                }
            }
            None
        }
        JsonVal::Arr(row) => {
            let first = row.first()?;
            match first {
                JsonVal::Str(s) => parse_iso_tdb(s, lsk),
                JsonVal::Num(n) => lsk.unix_to_tdb(*n),
                _ => None,
            }
        }
        _ => None,
    }
}



pub fn is_time_key(k: &str) -> bool {
    let kl = k.to_lowercase();
    kl == "time"
        || kl == "time_tag"
        || kl == "timestamp"
        || kl == "epoch"
        || kl == "t"
        || kl == "date"
        || kl.contains("time")
        || kl.contains("date")
}



pub fn extract_series(src: &SourceConfig, body: &str, lsk: &LeapSeconds) -> Vec<(f64, f64)> {
    let parsed = parse_json(body);
    let Some(ref j) = parsed else {
        return Vec::new();
    };
    let mut out: Vec<(f64, f64)> = Vec::new();
    for ext in &src.extracts {
        match ext {
            Extract::First(fc, filter) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    if let Some((fk, fv)) = filter {
                        if !row_matches(el, fk, fv) {
                            continue;
                        }
                    }
                    let raw = match el {
                        JsonVal::Obj(map) => map.get(&fc.key).and_then(scalar_of),
                        JsonVal::Arr(row) => {
                            if let Ok(idx) = fc.key.parse::<usize>() {
                                row.get(idx).and_then(scalar_of)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let (Some(raw), Some(epoch)) = (raw, series_epoch_of(el, lsk)) else {
                        continue;
                    };
                    let Some(val) = convert_to_si(raw, &fc.unit) else {
                        register_unconverted_unit(&fc.unit, &fc.name);
                        continue;
                    };
                    if val.is_finite() {
                        out.push((epoch, val));
                    }
                }
            }
            Extract::Last(fc, filter) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    if let Some((fk, fv)) = filter {
                        if !row_matches(el, fk, fv) {
                            continue;
                        }
                    }
                    let raw = match el {
                        JsonVal::Obj(map) => map.get(&fc.key).and_then(scalar_of),
                        JsonVal::Arr(row) => {
                            if let Ok(idx) = fc.key.parse::<usize>() {
                                row.get(idx).and_then(scalar_of)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let (Some(raw), Some(epoch)) = (raw, series_epoch_of(el, lsk)) else {
                        continue;
                    };
                    let Some(val) = convert_to_si(raw, &fc.unit) else {
                        register_unconverted_unit(&fc.unit, &fc.name);
                        continue;
                    };
                    if val.is_finite() {
                        out.push((epoch, val));
                    }
                }
            }
            Extract::Path(fc) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    let raw = jpath(el, &fc.key);
                    let (Some(raw), Some(epoch)) = (raw, series_epoch_of(el, lsk)) else {
                        continue;
                    };
                    let Some(val) = convert_to_si(raw, &fc.unit) else {
                        register_unconverted_unit(&fc.unit, &fc.name);
                        continue;
                    };
                    if val.is_finite() {
                        out.push((epoch, val));
                    }
                }
            }
            Extract::Hapi(pairs) => {
                let JsonVal::Obj(root) = j else {
                    continue;
                };
                let Some(JsonVal::Arr(data)) = root.get("data") else {
                    continue;
                };
                let mut col: HashMap<String, usize> = HashMap::new();
                let mut fill_of: HashMap<String, f64> = HashMap::new();
                if let Some(JsonVal::Arr(params)) = root.get("parameters") {
                    for (i, p) in params.iter().enumerate() {
                        if let JsonVal::Obj(po) = p {
                            if let Some(JsonVal::Str(nn)) = po.get("name") {
                                col.insert(nn.clone(), i);
                                if let Some(fv) = po.get("fill").and_then(scalar_of) {
                                    fill_of.insert(nn.clone(), fv);
                                }
                            }
                        }
                    }
                }
                for (k, v) in &src.hapi_fill {
                    fill_of.entry(k.clone()).or_insert(*v);
                }
                if col.is_empty() {
                    let mut next_col = 0usize;
                    for (param, _) in pairs.iter() {
                        let base = param.split('.').next().unwrap_or(param);
                        if !col.contains_key(base) {
                            next_col += 1;
                            col.insert(base.to_string(), next_col);
                        }
                    }
                }
                for row in data {
                    let JsonVal::Arr(cells) = row else {
                        continue;
                    };
                    let Some(epoch) = series_epoch_of(row, lsk) else {
                        continue;
                    };
                    for (param, name) in pairs {
                        let (base, comp) = match param.rfind('.') {
                            Some(dot) if param[dot + 1..].chars().all(|c| c.is_ascii_digit()) => {
                                (&param[..dot], param[dot + 1..].parse::<usize>().ok())
                            }
                            _ => (param.as_str(), None),
                        };
                        let Some(&idx) = col.get(base) else {
                            continue;
                        };
                        let v = match comp {
                            Some(i) => cells.get(idx).and_then(|cell| {
                                if let JsonVal::Arr(a) = cell {
                                    a.get(i).and_then(scalar_of)
                                } else {
                                    None
                                }
                            }),
                            None => cells.get(idx).and_then(scalar_of),
                        };
                        let Some(raw) = v else {
                            continue;
                        };
                        if fill_of.get(base).map_or(false, |&f| raw == f) {
                            continue;
                        }
                        let Some(fc) = src.extracts.iter().find_map(|e| match e {
                            Extract::Field(fc) if fc.name == *name => Some(fc),
                            _ => None,
                        }) else {
                            continue;
                        };
                        let Some(val) = convert_to_si(raw, &fc.unit) else {
                            register_unconverted_unit(&fc.unit, &fc.name);
                            continue;
                        };
                        if val.is_finite() {
                            out.push((epoch, val));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}
