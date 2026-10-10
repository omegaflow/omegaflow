use std::collections::HashMap;

pub const MAGIC: [u8; 4] = *b"IRF ";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 124;

const P_X: u32 = 1 << 0;
const P_Y: u32 = 1 << 1;
const P_Z: u32 = 1 << 2;
const P_VX: u32 = 1 << 3;
const P_VY: u32 = 1 << 4;
const P_VZ: u32 = 1 << 5;
const P_LON: u32 = 1 << 6;
const P_LAT: u32 = 1 << 7;
const P_H: u32 = 1 << 8;
const P_EPOCH: u32 = 1 << 9;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RefEpoch {
    pub year: u16,
    pub day_of_year: u16,
    pub second_of_day: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Station {
    pub code: String,
    pub point: String,
    pub domes: Option<String>,
    pub description: Option<String>,
    pub approx_lon_rad: Option<f64>,
    pub approx_lat_rad: Option<f64>,
    pub approx_h_m: Option<f64>,
    pub x_m: Option<f64>,
    pub y_m: Option<f64>,
    pub z_m: Option<f64>,
    pub vx_m_y: Option<f64>,
    pub vy_m_y: Option<f64>,
    pub vz_m_y: Option<f64>,
    pub ref_epoch: Option<RefEpoch>,
}

fn meas_f64(tok: &str) -> Option<f64> {
    let v: f64 = tok.parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn nonempty(field: &str) -> Option<String> {
    let t = field.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn dms_rad(deg: &str, min: &str, sec: &str) -> Option<f64> {
    let d: f64 = deg.parse().ok()?;
    let m: f64 = min.parse().ok()?;
    let s: f64 = sec.parse().ok()?;
    if !d.is_finite() || !m.is_finite() || !s.is_finite() {
        return None;
    }
    if !(0.0..60.0).contains(&m) || !(0.0..60.0).contains(&s) {
        return None;
    }
    let sign = if deg.trim_start().starts_with('-') {
        -1.0
    } else {
        1.0
    };
    Some((sign * (d.abs() + m / 60.0 + s / 3600.0)).to_radians())
}

fn parse_ref_epoch(tok: &str) -> Option<RefEpoch> {
    let mut fields = tok.split(':');
    let year: u16 = fields.next()?.parse().ok()?;
    let day_of_year: u16 = fields.next()?.parse().ok()?;
    let second_of_day: f64 = fields.next()?.parse().ok()?;
    if fields.next().is_some() || !second_of_day.is_finite() {
        return None;
    }
    Some(RefEpoch {
        year,
        day_of_year,
        second_of_day,
    })
}

pub fn parse_sinex(bytes: &[u8]) -> Option<Vec<Station>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut lines = text.lines();
    let first = lines.next()?;
    if !first.starts_with("%=SNX") {
        return None;
    }

    let mut stations: Vec<Station> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();
    let mut in_site_id = false;
    let mut in_estimate = false;

    for raw in lines {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('+') {
            in_site_id = rest.starts_with("SITE/ID");
            in_estimate = rest.starts_with("SOLUTION/ESTIMATE");
            continue;
        }
        if line.starts_with('-') {
            in_site_id = false;
            in_estimate = false;
            continue;
        }
        if line.starts_with('*') || line.starts_with('%') {
            continue;
        }
        if in_site_id {
            let b = line.as_bytes();
            if b.len() < 8 {
                continue;
            }
            let code = std::str::from_utf8(&b[1..5]).ok()?;
            let point = std::str::from_utf8(&b[7..8]).ok()?;
            let domes = if b.len() >= 18 {
                nonempty(std::str::from_utf8(&b[9..18]).ok()?)
            } else {
                None
            };
            let description = if b.len() >= 43 {
                nonempty(std::str::from_utf8(&b[21..43]).ok()?)
            } else {
                None
            };
            let (approx_lon_rad, approx_lat_rad, approx_h_m) = if b.len() > 43 {
                let tail: Vec<&str> = std::str::from_utf8(&b[43..])
                    .ok()?
                    .split_whitespace()
                    .collect();
                if tail.len() >= 7 {
                    (
                        dms_rad(tail[0], tail[1], tail[2]),
                        dms_rad(tail[3], tail[4], tail[5]),
                        meas_f64(tail[6]),
                    )
                } else {
                    (None, None, None)
                }
            } else {
                (None, None, None)
            };
            let key = format!("{} {}", code.trim_end(), point.trim());
            if index.contains_key(&key) {
                continue;
            }
            index.insert(key, stations.len());
            stations.push(Station {
                code: code.trim_end().to_string(),
                point: point.trim().to_string(),
                domes,
                description,
                approx_lon_rad,
                approx_lat_rad,
                approx_h_m,
                x_m: None,
                y_m: None,
                z_m: None,
                vx_m_y: None,
                vy_m_y: None,
                vz_m_y: None,
                ref_epoch: None,
            });
        } else if in_estimate {
            let toks: Vec<&str> = line.split_whitespace().collect();
            if toks.len() < 10 {
                continue;
            }
            let key = format!("{} {}", toks[2], toks[3]);
            let Some(&idx) = index.get(&key) else {
                continue;
            };
            let station = &mut stations[idx];
            match toks[1] {
                "STAX" => station.x_m = meas_f64(toks[8]),
                "STAY" => station.y_m = meas_f64(toks[8]),
                "STAZ" => station.z_m = meas_f64(toks[8]),
                "VELX" => station.vx_m_y = meas_f64(toks[8]),
                "VELY" => station.vy_m_y = meas_f64(toks[8]),
                "VELZ" => station.vz_m_y = meas_f64(toks[8]),
                _ => {}
            }
            if station.ref_epoch.is_none() {
                station.ref_epoch = parse_ref_epoch(toks[5]);
            }
        }
    }
    Some(stations)
}

fn put_f64(out: &mut Vec<u8>, v: Option<f64>, bit: u32, present: &mut u32) -> bool {
    match v {
        Some(x) => {
            if !x.is_finite() {
                return false;
            }
            *present |= bit;
            out.extend_from_slice(&x.to_le_bytes());
        }
        None => out.extend_from_slice(&0.0f64.to_le_bytes()),
    }
    true
}

fn put_epoch(out: &mut Vec<u8>, epoch: Option<RefEpoch>, present: &mut u32) -> bool {
    match epoch {
        Some(e) => {
            if !e.second_of_day.is_finite() {
                return false;
            }
            *present |= P_EPOCH;
            out.extend_from_slice(&e.year.to_le_bytes());
            out.extend_from_slice(&e.day_of_year.to_le_bytes());
            out.extend_from_slice(&e.second_of_day.to_le_bytes());
        }
        None => {
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&0.0f64.to_le_bytes());
        }
    }
    true
}

fn put_ascii(out: &mut Vec<u8>, s: &str, width: usize) -> Option<()> {
    if !s.is_ascii() || s.len() > width {
        return None;
    }
    out.extend_from_slice(s.as_bytes());
    for _ in s.len()..width {
        out.push(b' ');
    }
    Some(())
}

fn put_opt_ascii(out: &mut Vec<u8>, v: Option<&str>, width: usize) -> Option<()> {
    match v {
        Some(s) => put_ascii(out, s, width),
        None => {
            for _ in 0..width {
                out.push(b' ');
            }
            Some(())
        }
    }
}

pub fn write_bin(records: &[Station]) -> Option<Vec<u8>> {
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        let mut present = 0u32;
        if !put_f64(&mut out, r.x_m, P_X, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.y_m, P_Y, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.z_m, P_Z, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.vx_m_y, P_VX, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.vy_m_y, P_VY, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.vz_m_y, P_VZ, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.approx_lon_rad, P_LON, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.approx_lat_rad, P_LAT, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.approx_h_m, P_H, &mut present) {
            return None;
        }
        if !put_epoch(&mut out, r.ref_epoch, &mut present) {
            return None;
        }
        out.extend_from_slice(&present.to_le_bytes());
        put_ascii(&mut out, &r.code, 4)?;
        put_ascii(&mut out, &r.point, 1)?;
        put_opt_ascii(&mut out, r.domes.as_deref(), 9)?;
        put_opt_ascii(&mut out, r.description.as_deref(), 22)?;
    }
    Some(out)
}

struct Reader<'a> {
    bytes: &'a [u8],
    off: usize,
}

impl<'a> Reader<'a> {
    fn f64(&mut self) -> Option<f64> {
        let v = f64::from_le_bytes(self.bytes.get(self.off..self.off + 8)?.try_into().ok()?);
        self.off += 8;
        Some(v)
    }

    fn u16(&mut self) -> Option<u16> {
        let v = u16::from_le_bytes(self.bytes.get(self.off..self.off + 2)?.try_into().ok()?);
        self.off += 2;
        Some(v)
    }

    fn u32(&mut self) -> Option<u32> {
        let v = u32::from_le_bytes(self.bytes.get(self.off..self.off + 4)?.try_into().ok()?);
        self.off += 4;
        Some(v)
    }

    fn ascii(&mut self, width: usize) -> Option<&'a str> {
        let bytes: &'a [u8] = self.bytes;
        let s = std::str::from_utf8(bytes.get(self.off..self.off + width)?).ok()?;
        self.off += width;
        Some(s)
    }
}

fn present_f64(raw: f64, present: u32, bit: u32) -> Option<Option<f64>> {
    if present & bit != 0 {
        if raw.is_finite() {
            Some(Some(raw))
        } else {
            None
        }
    } else if raw == 0.0 {
        Some(None)
    } else {
        None
    }
}

fn present_ascii(field: &str) -> Option<String> {
    let t = field.trim_end();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<Station>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(RECORD_BYTES)? {
        return None;
    }
    let mut rd = Reader {
        bytes,
        off: HEADER_BYTES,
    };
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let x_raw = rd.f64()?;
        let y_raw = rd.f64()?;
        let z_raw = rd.f64()?;
        let vx_raw = rd.f64()?;
        let vy_raw = rd.f64()?;
        let vz_raw = rd.f64()?;
        let lon_raw = rd.f64()?;
        let lat_raw = rd.f64()?;
        let h_raw = rd.f64()?;
        let epoch_year = rd.u16()?;
        let epoch_doy = rd.u16()?;
        let epoch_sec = rd.f64()?;
        let present = rd.u32()?;
        let code = rd.ascii(4)?;
        let point = rd.ascii(1)?;
        let domes = rd.ascii(9)?;
        let description = rd.ascii(22)?;

        let ref_epoch = if present & P_EPOCH != 0 {
            if epoch_sec.is_finite() {
                Some(RefEpoch {
                    year: epoch_year,
                    day_of_year: epoch_doy,
                    second_of_day: epoch_sec,
                })
            } else {
                return None;
            }
        } else if epoch_year == 0 && epoch_doy == 0 && epoch_sec == 0.0 {
            None
        } else {
            return None;
        };

        out.push(Station {
            code: code.trim_end().to_string(),
            point: point.trim_end().to_string(),
            domes: present_ascii(domes),
            description: present_ascii(description),
            approx_lon_rad: present_f64(lon_raw, present, P_LON)?,
            approx_lat_rad: present_f64(lat_raw, present, P_LAT)?,
            approx_h_m: present_f64(h_raw, present, P_H)?,
            x_m: present_f64(x_raw, present, P_X)?,
            y_m: present_f64(y_raw, present, P_Y)?,
            z_m: present_f64(z_raw, present, P_Z)?,
            vx_m_y: present_f64(vx_raw, present, P_VX)?,
            vy_m_y: present_f64(vy_raw, present, P_VY)?,
            vz_m_y: present_f64(vz_raw, present, P_VZ)?,
            ref_epoch,
        });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"%=SNX 2.02 TEST 22:001:00000 TEST 79:329:00000 21:003:00000 C 01224 2 X V  
*-------------------------------------------------------------------------------
+SITE/ID
*CODE PT __DOMES__ T _STATION DESCRIPTION__ APPROX_LON_ APPROX_LAT_ _APP_H_
 7203  A 14209S001   EFLSBERG Effelsberg, G   6 53 01.0  50 31 29.4   416.8
-SITE/ID
*-------------------------------------------------------------------------------
+SOLUTION/ESTIMATE
*INDEX TYPE__ CODE PT SOLN _REF_EPOCH__ UNIT S __ESTIMATED VALUE____ _STD_DEV___
     1 STAX   7203  A    1 15:001:00000 m    2 0.403394728676361E+07 0.92450E-02
     2 STAY   7203  A    1 15:001:00000 m    2 0.486990823348219E+06 0.38376E-02
     3 STAZ   7203  A    1 15:001:00000 m    2 0.490043108394323E+07 0.10455E-01
     4 VELX   7203  A    1 15:001:00000 m/y  2 -.139698991002166E-01 0.33955E-03
     5 VELY   7203  A    1 15:001:00000 m/y  2 0.169886132211163E-01 0.15577E-03
     6 VELZ   7203  A    1 15:001:00000 m/y  2 0.107017114287353E-01 0.39626E-03
-SOLUTION/ESTIMATE
%ENDSNX
"#;

    #[test]
    fn parses_the_measured_station() {
        let stations = parse_sinex(SAMPLE.as_bytes()).expect("the sample parses");
        assert_eq!(stations.len(), 1);
        let s = &stations[0];
        assert_eq!(s.code, "7203");
        assert_eq!(s.point, "A");
        assert_eq!(s.domes.as_deref(), Some("14209S001"));
        assert_eq!(s.description.as_deref(), Some("EFLSBERG Effelsberg, G"));
        assert_eq!(s.approx_h_m, Some(416.8));
        assert!(s.approx_lon_rad.is_some_and(f64::is_finite));
        assert!(s.approx_lat_rad.is_some_and(f64::is_finite));
        assert_eq!(s.x_m, Some(0.403394728676361E+07));
        assert_eq!(s.y_m, Some(0.486990823348219E+06));
        assert_eq!(s.z_m, Some(0.490043108394323E+07));
        assert_eq!(s.vx_m_y, Some(-.139698991002166E-01));
        assert_eq!(s.vy_m_y, Some(0.169886132211163E-01));
        assert_eq!(s.vz_m_y, Some(0.107017114287353E-01));
        assert_eq!(
            s.ref_epoch,
            Some(RefEpoch {
                year: 15,
                day_of_year: 1,
                second_of_day: 0.0,
            })
        );
    }

    #[test]
    fn parse_sinex_refuses_foreign_bytes() {
        assert_eq!(parse_sinex(b"not a sinex file at all"), None);
        assert_eq!(parse_sinex(b""), None);
        assert_eq!(parse_sinex(b"%%END OF SINEX FILE\n"), None);
    }

    #[test]
    fn roundtrip_preserves_present_and_absent_fields() {
        let stations = parse_sinex(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = write_bin(&stations).expect("the stations encode");
        assert_eq!(bin.len(), HEADER_BYTES + RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(stations));
    }

    #[test]
    fn roundtrip_preserves_absent_fields() {
        let stations = vec![Station {
            code: "7203".to_string(),
            point: "A".to_string(),
            domes: None,
            description: None,
            approx_lon_rad: None,
            approx_lat_rad: None,
            approx_h_m: None,
            x_m: Some(1.0),
            y_m: None,
            z_m: Some(3.0),
            vx_m_y: None,
            vy_m_y: None,
            vz_m_y: None,
            ref_epoch: None,
        }];
        let bin = write_bin(&stations).expect("the station encodes");
        assert_eq!(parse_bin(&bin), Some(stations));
    }

    #[test]
    fn parse_bin_refuses_foreign_and_truncated_bytes() {
        assert_eq!(parse_bin(b"IRF "), None);
        assert_eq!(parse_bin(b"XXXX\x01\x00\x00\x00"), None);
        let stations = parse_sinex(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = write_bin(&stations).expect("the stations encode");
        assert_eq!(parse_bin(&bin[..bin.len() - 1]), None);
    }

    #[test]
    #[ignore = "reads the ITRF2020 SINEX named by OMEGAFLOW_ITRF_SSC"]
    fn real_sinex_yields_the_measured_station_count() {
        let path = std::env::var("OMEGAFLOW_ITRF_SSC").expect("OMEGAFLOW_ITRF_SSC names a SINEX");
        let bytes = std::fs::read(&path).expect("read the SINEX");
        let stations = parse_sinex(&bytes).expect("the SINEX parses");
        assert_eq!(stations.len(), 154);
    }
}
