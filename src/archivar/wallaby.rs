use super::json::JsonVal;

pub const MAGIC: [u8; 4] = *b"WAB1";
pub const VERSION: u8 = 1;
pub const HEADER_BYTES: usize = 12;
pub const REC_BYTES: usize = 7 * 8;

pub const KEYS: [&str; 7] = ["ra", "dec", "freq", "w20", "w50", "dist_h", "log_m_hi"];
pub const COMP_NAMES: [&str; 7] = [
    "wallaby_ra_deg",
    "wallaby_dec_deg",
    "wallaby_hi_freq_hz",
    "wallaby_w20_hz",
    "wallaby_w50_hz",
    "wallaby_dist_mpc",
    "wallaby_log_m_hi_dex",
];

pub const COMP_RA: u32 = 0;
pub const COMP_DEC: u32 = 1;
pub const COMP_FREQ: u32 = 2;
pub const COMP_W20: u32 = 3;
pub const COMP_W50: u32 = 4;
pub const COMP_DIST_H: u32 = 5;
pub const COMP_LOG_M_HI: u32 = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallabySource {
    pub ra: f64,
    pub dec: f64,
    pub freq: f64,
    pub w20: Option<f64>,
    pub w50: Option<f64>,
    pub dist_h: Option<f64>,
    pub log_m_hi: Option<f64>,
}

impl WallabySource {
    pub fn value(&self, comp: u32) -> Option<f64> {
        match comp {
            COMP_RA => Some(self.ra),
            COMP_DEC => Some(self.dec),
            COMP_FREQ => Some(self.freq),
            COMP_W20 => self.w20,
            COMP_W50 => self.w50,
            COMP_DIST_H => self.dist_h,
            COMP_LOG_M_HI => self.log_m_hi,
            _ => None,
        }
    }

    pub fn value_by_key(&self, key: &str) -> Option<f64> {
        let comp = KEYS.iter().position(|k| *k == key)? as u32;
        self.value(comp)
    }
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COMP_NAMES.get(comp as usize).copied()
}

pub fn component_index(key: &str) -> Option<u32> {
    KEYS.iter().position(|k| *k == key).map(|i| i as u32)
}

fn finite(v: Option<&JsonVal>) -> Option<f64> {
    match v? {
        JsonVal::Num(x) if x.is_finite() => Some(*x),
        _ => None,
    }
}

pub fn parse_json_catalogue(text: &str) -> Option<Vec<WallabySource>> {
    let JsonVal::Arr(rows) = super::json::parse_json(text)? else {
        return None;
    };
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let JsonVal::Obj(map) = row else {
            return None;
        };
        let ra = finite(map.get("ra"))?;
        let dec = finite(map.get("dec"))?;
        let freq = finite(map.get("freq"))?;
        if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) || freq <= 0.0 {
            return None;
        }
        out.push(WallabySource {
            ra,
            dec,
            freq,
            w20: finite(map.get("w20")),
            w50: finite(map.get("w50")),
            dist_h: finite(map.get("dist_h")),
            log_m_hi: finite(map.get("log_m_hi")),
        });
    }
    Some(out)
}

fn plausible(source: &WallabySource) -> bool {
    source.ra.is_finite()
        && (0.0..=360.0).contains(&source.ra)
        && source.dec.is_finite()
        && (-90.0..=90.0).contains(&source.dec)
        && source.freq.is_finite()
        && source.freq > 0.0
        && source.w20.is_none_or(|v| v.is_finite())
        && source.w50.is_none_or(|v| v.is_finite())
        && source.dist_h.is_none_or(|v| v.is_finite())
        && source.log_m_hi.is_none_or(|v| v.is_finite())
}

pub fn write_bin(sources: &[WallabySource]) -> Option<Vec<u8>> {
    if sources.len() > u32::MAX as usize || sources.iter().any(|s| !plausible(s)) {
        return None;
    }
    let mut out = Vec::with_capacity(HEADER_BYTES + sources.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.push(0);
    out.extend_from_slice(&[0u8; 2]);
    out.extend_from_slice(&(sources.len() as u32).to_le_bytes());
    for s in sources {
        out.extend_from_slice(&s.ra.to_le_bytes());
        out.extend_from_slice(&s.dec.to_le_bytes());
        out.extend_from_slice(&s.freq.to_le_bytes());
        out.extend_from_slice(&s.w20?.to_le_bytes());
        out.extend_from_slice(&s.w50?.to_le_bytes());
        out.extend_from_slice(&s.dist_h?.to_le_bytes());
        out.extend_from_slice(&s.log_m_hi?.to_le_bytes());
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<WallabySource>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC || bytes[4] != VERSION || bytes[5] != 0 {
        return None;
    }
    let count = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES.checked_add(count.checked_mul(REC_BYTES)?)? {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    let mut off = HEADER_BYTES;
    for _ in 0..count {
        let get = |i: usize| -> Option<f64> {
            let start = off + i * 8;
            Some(f64::from_le_bytes(
                bytes.get(start..start + 8)?.try_into().ok()?,
            ))
        };
        let opt = |v: f64| if v.is_nan() { None } else { Some(v) };
        let source = WallabySource {
            ra: get(0)?,
            dec: get(1)?,
            freq: get(2)?,
            w20: opt(get(3)?),
            w50: opt(get(4)?),
            dist_h: opt(get(5)?),
            log_m_hi: opt(get(6)?),
        };
        off += REC_BYTES;
        if !plausible(&source) {
            return None;
        }
        out.push(source);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &str = r#"[
        {"ra":154.2297749022268,"dec":-48.8773454083931,"name":"WALLABY J101655-485238","freq":1.407512971336861E9,"w20":1809946.485445322,"w50":1703609.096101342,"dist_h":39.22986672308076,"log_m_hi":10.239972898827773},
        {"ra":153.7,"dec":-43.6,"freq":1.4061E9,"w20":null,"w50":1.69E6,"dist_h":43.35,"log_m_hi":10.06}
    ]"#;

    #[test]
    fn json_rows_become_the_seven_columns() {
        let sources = parse_json_catalogue(JSON).unwrap();
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].ra, 154.2297749022268);
        assert_eq!(sources[0].freq, 1.407512971336861E9);
        assert_eq!(sources[1].w20, None);
        assert_eq!(sources[1].w50, Some(1.69E6));
        assert_eq!(component_name(COMP_RA), Some("wallaby_ra_deg"));
        assert_eq!(component_name(COMP_LOG_M_HI), Some("wallaby_log_m_hi_dex"));
        assert_eq!(component_name(7), None);
        assert_eq!(sources[0].value_by_key("dist_h"), sources[0].dist_h);
    }

    #[test]
    fn table_roundtrips() {
        let sources = parse_json_catalogue(JSON).unwrap();
        let bytes = write_bin(&sources).unwrap();
        assert_eq!(parse_bin(&bytes).unwrap(), sources);
        assert_eq!(parse_bin(&bytes[..bytes.len() - 1]), None);
    }

    #[test]
    fn malformed_rows_are_refused() {
        assert!(parse_json_catalogue("[]").unwrap().is_empty());
        assert!(parse_json_catalogue(r#"[{"ra":400,"dec":0,"freq":1}]"#).is_none());
        assert!(parse_json_catalogue(r#"[{"ra":1,"dec":0,"freq":0}]"#).is_none());
        assert!(parse_json_catalogue(r#"[{"ra":1,"dec":0}]"#).is_none());
        assert!(
            write_bin(&[WallabySource {
                ra: -1.0,
                dec: 0.0,
                freq: 1.0,
                w20: None,
                w50: None,
                dist_h: None,
                log_m_hi: None,
            }])
            .is_none()
        );
        assert!(parse_bin(b"WAB1").is_none());
    }
}
