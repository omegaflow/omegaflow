pub const MAGIC: [u8; 4] = *b"DSN1";
pub const VERSION: u8 = 1;

pub const BANDS: [&str; 8] = ["S", "X", "K", "Ka", "Ku", "L", "N", "?"];

pub const DIR_DOWN: u8 = 0;
pub const DIR_UP: u8 = 1;

pub const KIND_POWER: u32 = 0;
pub const KIND_ACTIVE: u32 = 1;

pub fn band_index(band: &str) -> u32 {
    match BANDS.iter().position(|b| *b == band) {
        Some(i) => i as u32,
        None => 7,
    }
}

pub fn component_of(band: &str, dir: u8, kind: u32) -> u32 {
    kind * 16 + band_index(band) * 2 + (dir & 1) as u32
}

pub fn component_name(comp: u32) -> Option<String> {
    let kind = comp / 16;
    let rest = comp % 16;
    let band = BANDS.get((rest / 2) as usize)?;
    let dir = if rest % 2 == 0 { "down" } else { "up" };
    let suffix = match kind {
        KIND_POWER => "power",
        KIND_ACTIVE => "active",
        _ => return None,
    };
    Some(format!(
        "dsn_{}_{}_{}",
        band.to_ascii_lowercase(),
        dir,
        suffix
    ))
}

#[derive(Clone, Debug, PartialEq)]
pub struct Signal {
    pub dish: String,
    pub spacecraft: String,
    pub band: String,
    pub uplink: bool,
    pub active: bool,
    pub power: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub t: f64,
    pub dish: String,
    pub spacecraft: String,
    pub band: String,
    pub uplink: bool,
    pub active: bool,
    pub power: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub epoch_unix: f64,
    pub records: Vec<Record>,
}

fn attr(tag: &str, name: &str) -> Option<String> {
    let b = tag.as_bytes();
    let mut i = 0usize;
    while let Some(p) = tag[i..].find(name) {
        let start = i + p;
        let after = start + name.len();
        let boundary = start == 0 || b[start - 1].is_ascii_whitespace();
        if boundary && b.get(after) == Some(&b'=') {
            let mut j = after + 1;
            while b.get(j).is_some_and(|c| c.is_ascii_whitespace()) {
                j += 1;
            }
            if b.get(j) == Some(&b'"') {
                let vs = j + 1;
                let ve = vs + tag[vs..].find('"')?;
                return Some(tag[vs..ve].to_string());
            }
        }
        i = after;
    }
    None
}

fn tag_end(s: &str, from: usize) -> Option<usize> {
    let b = s.as_bytes();
    let mut i = from;
    let mut in_quotes = false;
    while i < b.len() {
        match b[i] {
            b'"' => in_quotes = !in_quotes,
            b'>' if !in_quotes => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}

fn tags<'a>(body: &'a str, name: &str) -> Vec<&'a str> {
    let open = format!("<{name}");
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(p) = body[i..].find(&open) {
        let s = i + p;
        let after = s + open.len();
        let boundary = matches!(
            body.as_bytes().get(after),
            Some(b' ') | Some(b'>') | Some(b'/') | Some(b'\t') | Some(b'\r') | Some(b'\n')
        );
        if !boundary {
            i = after;
            continue;
        }
        let Some(gt) = tag_end(body, s) else {
            break;
        };
        out.push(&body[s..=gt]);
        i = gt + 1;
    }
    out
}

fn elements<'a>(body: &'a str, name: &str) -> Vec<(&'a str, &'a str)> {
    let open = format!("<{name}");
    let close = format!("</{name}>");
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(p) = body[i..].find(&open) {
        let s = i + p;
        let after = s + open.len();
        let boundary = matches!(
            body.as_bytes().get(after),
            Some(b' ') | Some(b'>') | Some(b'\t') | Some(b'\r') | Some(b'\n')
        );
        if !boundary {
            i = after;
            continue;
        }
        let Some(gt) = tag_end(body, s) else {
            break;
        };
        if body.as_bytes().get(gt.wrapping_sub(1)) == Some(&b'/') {
            i = gt + 1;
            continue;
        }
        let Some(cp) = body[gt + 1..].find(&close) else {
            break;
        };
        out.push((&body[s..gt], &body[gt + 1..gt + 1 + cp]));
        i = gt + 1 + cp + close.len();
    }
    out
}

fn element_text<'a>(body: &'a str, name: &str) -> Option<&'a str> {
    let open = format!("<{name}>");
    let close = format!("</{name}>");
    let start = body.find(&open)? + open.len();
    let end = start + body[start..].find(&close)?;
    Some(body[start..end].trim())
}

pub fn parse_xml(body: &str) -> Option<(f64, Vec<Signal>)> {
    let epoch_ms: i64 = element_text(body, "timestamp")?.parse().ok()?;
    if epoch_ms <= 0 {
        return None;
    }
    let epoch_unix = epoch_ms as f64 / 1000.0;
    let mut signals = Vec::new();
    for (open, inner) in elements(body, "dish") {
        let Some(dish) = attr(open, "name").filter(|s| !s.is_empty()) else {
            continue;
        };
        for (tag_name, uplink) in [("downSignal", false), ("upSignal", true)] {
            for tag in tags(inner, tag_name) {
                let (Some(spacecraft), Some(band)) = (
                    attr(tag, "spacecraft").filter(|s| !s.is_empty()),
                    attr(tag, "band").filter(|s| !s.is_empty()),
                ) else {
                    continue;
                };
                let active = attr(tag, "active").is_some_and(|v| v == "true");
                let power = attr(tag, "power")
                    .and_then(|v| v.parse::<f64>().ok())
                    .filter(|v| v.is_finite());
                signals.push(Signal {
                    dish: dish.clone(),
                    spacecraft,
                    band,
                    uplink,
                    active,
                    power: if active { power } else { None },
                });
            }
        }
    }
    if signals.is_empty() {
        return None;
    }
    Some((epoch_unix, signals))
}

pub fn records_at(epoch_tdb: f64, signals: Vec<Signal>) -> Vec<Record> {
    signals
        .into_iter()
        .map(|s| Record {
            t: epoch_tdb,
            dish: s.dish,
            spacecraft: s.spacecraft,
            band: s.band,
            uplink: s.uplink,
            active: s.active,
            power: s.power,
        })
        .collect()
}

pub fn write_bin(epoch_unix: f64, records: &[Record]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(16 + records.len() * 40);
    buf.extend_from_slice(&MAGIC);
    buf.push(VERSION);
    buf.extend_from_slice(&epoch_unix.to_le_bytes());
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t.to_le_bytes());
        let (pad, present) = match r.power {
            Some(power) => (power, 1u8),
            None => (0.0, 0u8),
        };
        buf.extend_from_slice(&pad.to_le_bytes());
        buf.push(present);
        buf.push(if r.active { 1 } else { 0 });
        buf.push(if r.uplink { 1 } else { 0 });
        buf.push(band_index(&r.band) as u8);
        let dish = r.dish.as_bytes();
        buf.push(dish.len().min(255) as u8);
        buf.extend_from_slice(&dish[..dish.len().min(255)]);
        let sc = r.spacecraft.as_bytes();
        buf.push(sc.len().min(255) as u8);
        buf.extend_from_slice(&sc[..sc.len().min(255)]);
    }
    buf
}

fn take_u8(bytes: &[u8], off: &mut usize) -> Option<u8> {
    let v = *bytes.get(*off)?;
    *off += 1;
    Some(v)
}

fn take_str(bytes: &[u8], off: &mut usize) -> Option<String> {
    let len = take_u8(bytes, off)? as usize;
    let s = bytes.get(*off..*off + len)?;
    *off += len;
    std::str::from_utf8(s).ok().map(str::to_string)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Snapshot> {
    if bytes.len() < 17 || bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    let mut off = 5usize;
    let epoch_unix = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
    off += 8;
    let count = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
    off += 4;
    if !epoch_unix.is_finite() || count > bytes.len() {
        return None;
    }
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let raw_power = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let flags = take_u8(bytes, &mut off)?;
        let active = take_u8(bytes, &mut off)? != 0;
        let uplink = take_u8(bytes, &mut off)? != 0;
        let band_code = take_u8(bytes, &mut off)? as usize;
        if band_code >= BANDS.len() {
            return None;
        }
        let dish = take_str(bytes, &mut off)?;
        let spacecraft = take_str(bytes, &mut off)?;
        if !t.is_finite() {
            return None;
        }
        let power = if flags & 1 != 0 {
            if !raw_power.is_finite() {
                return None;
            }
            Some(raw_power)
        } else {
            None
        };
        records.push(Record {
            t,
            dish,
            spacecraft,
            band: BANDS[band_code].to_string(),
            uplink,
            active,
            power,
        });
    }
    Some(Snapshot {
        epoch_unix,
        records,
    })
}

pub fn series_parse(bytes: &[u8]) -> Option<(Vec<(f64, f64, u32)>, Vec<String>)> {
    let snapshot = parse_bin(bytes)?;
    let mut rows = Vec::new();
    for r in &snapshot.records {
        let dir = if r.uplink { DIR_UP } else { DIR_DOWN };
        rows.push((
            r.t,
            if r.active { 1.0 } else { 0.0 },
            component_of(&r.band, dir, KIND_ACTIVE),
        ));
        if let Some(power) = r.power {
            rows.push((r.t, power, component_of(&r.band, dir, KIND_POWER)));
        }
    }
    if rows.is_empty() {
        return None;
    }
    let names: Vec<String> = (0..32u32).filter_map(component_name).collect();
    Some((rows, names))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<dsn>
	<station name="gdscc" friendlyName="Goldstone" timeUTC="1790904871000" timeZoneOffset="-25200000.0"/>
	<dish name="DSS23" azimuthAngle="0" elevationAngle="90" windSpeed="" isMSPA="false" activity="Spacecraft Telemetry, Tracking, and Command">
		<downSignal active="true" signalType="data" dataRate="0" frequency="0" band="X" power="-170" spacecraft="VGR1" spacecraftID="-31"/>
		<downSignal active="true" signalType="data" dataRate="160" frequency="0" band="X" power="-160" spacecraft="VGR1" spacecraftID="-31"/>
		<target name="VGR1" id="31" uplegRange="25800000000" downlegRange="25800000000" rtlt="-1"/>
	</dish>
	<dish name="DSS56" azimuthAngle="193" elevationAngle="69" windSpeed="4" isMSPA="false" activity="Spacecraft Telemetry, Tracking, and Command">
		<upSignal active="true" signalType="data" dataRate="0" frequency="0" band="X" power="0.1" spacecraft="JWST" spacecraftID="-170"/>
		<downSignal active="true" signalType="data" dataRate="40000" frequency="0" band="S" power="-120" spacecraft="JWST" spacecraftID="-170"/>
		<downSignal active="false" signalType="none" dataRate="0" frequency="0" band="Ka" power="-480" spacecraft="JWST" spacecraftID="-170"/>
	</dish>
	<timestamp>1790904871000</timestamp>
</dsn>"#;

    #[test]
    fn parses_dish_signals_and_epoch() {
        let (epoch, signals) = parse_xml(SAMPLE).unwrap();
        assert_eq!(epoch, 1790904871.0);
        assert_eq!(signals.len(), 5);
        assert_eq!(signals[0].dish, "DSS23");
        assert_eq!(signals[0].spacecraft, "VGR1");
        assert_eq!(signals[0].band, "X");
        assert!(!signals[0].uplink);
        assert!(signals[0].active);
        assert_eq!(signals[0].power, Some(-170.0));
        assert_eq!(signals[2].band, "S");
        assert!(!signals[3].active);
        assert_eq!(signals[3].power, None);
        assert!(signals[4].uplink);
        assert_eq!(signals[4].power, Some(0.1));
    }

    #[test]
    fn bin_roundtrip_keeps_the_snapshot() {
        let (epoch, signals) = parse_xml(SAMPLE).unwrap();
        let records = records_at(epoch, signals);
        let bytes = write_bin(epoch, &records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.epoch_unix, epoch);
        assert_eq!(parsed.records, records);
        assert!(parse_bin(b"XX").is_none());
        assert!(parse_bin(b"DSN1\x02").is_none());
    }

    #[test]
    fn series_projects_activity_and_power() {
        let (epoch, signals) = parse_xml(SAMPLE).unwrap();
        let records = records_at(epoch, signals);
        let bytes = write_bin(epoch, &records);
        let (rows, names) = series_parse(&bytes).unwrap();
        assert_eq!(names.len(), 32);
        assert_eq!(
            names[component_of("X", DIR_DOWN, KIND_POWER) as usize],
            "dsn_x_down_power"
        );
        assert_eq!(
            names[component_of("Ka", DIR_DOWN, KIND_ACTIVE) as usize],
            "dsn_ka_down_active"
        );
        assert_eq!(rows.len(), 9);
        let ka_active = rows
            .iter()
            .find(|(_, _, c)| *c == component_of("Ka", DIR_DOWN, KIND_ACTIVE))
            .unwrap();
        assert_eq!(ka_active.1, 0.0);
    }

    #[test]
    fn component_names_cover_the_band_axis() {
        assert_eq!(
            component_name(component_of("S", DIR_UP, KIND_POWER)),
            Some("dsn_s_up_power".to_string())
        );
        assert_eq!(
            component_name(component_of("?", DIR_DOWN, KIND_ACTIVE)),
            Some("dsn_?_down_active".to_string())
        );
        assert_eq!(component_name(40), None);
    }
}
