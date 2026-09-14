use super::*;
use crate::lsk::days_from_civil;

pub fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> Option<(f64, f64, f64)> {
    const A: f64 = 6378137.0;
    const E2: f64 = 6.69437999014e-3;
    let b = A * (1.0 - E2).sqrt();
    let ep2 = (A * A - b * b) / (b * b);
    let lon = y.atan2(x);
    let p = (x * x + y * y).sqrt();
    if p < 1e-6 {
        return None;
    }
    let theta = (z * A).atan2(p * b);
    let lat = (z + ep2 * b * theta.sin().powi(3)).atan2(p - E2 * A * theta.cos().powi(3));
    let n = A / (1.0 - E2 * lat.sin().powi(2)).sqrt();
    let h = p / lat.cos() - n;
    Some((lat.to_degrees(), lon.to_degrees(), h))
}

pub fn build_rinex_channels(
    src: &SourceConfig,
    text: &str,
    now: f64,
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let mut channels = Vec::new();
    let decoded;
    let text = if is_hatanaka(text) {
        match crx2rnx(text) {
            Some(d) => {
                decoded = d;
                decoded.as_str()
            }
            None => return channels,
        }
    } else {
        text
    };
    let Some(header) = parse_rinex_header(text) else {
        return channels;
    };
    let mut fields: Vec<&FieldConfig> = Vec::new();
    for ext in &src.extracts {
        if let Extract::Field(fc) = ext {
            fields.push(fc);
        }
    }
    if fields.is_empty() {
        return channels;
    }
    match header.file_type {
        RinexFileType::Observation => {
            let position = header
                .approx_pos_xyz
                .and_then(|(x, y, z)| ecef_to_geodetic(x, y, z))
                .map(|(lat, lon, alt)| Position::Surface {
                    body_name: "earth".to_string(),
                    lat,
                    lon,
                    alt,
                })
                .unwrap_or(Position::Source);
            let n_obs = header.obs_types.len();
            let mut emitted = 0usize;
            for e in parse_rinex_obs(text, n_obs) {
                if e.epoch_unix > now {
                    continue;
                }
                let Some(epoch) = lsk.unix_to_tdb(e.epoch_unix) else {
                    continue;
                };
                for sat in &e.sats {
                    for (i, v) in sat.values.iter().enumerate() {
                        let Some(v) = v else { continue };
                        let Some(obs_type) = header.obs_types.get(i) else {
                            continue;
                        };
                        let obs_type = obs_type.as_str();
                        for fc in &fields {
                            if !fc.key.eq_ignore_ascii_case(obs_type) {
                                continue;
                            }
                            channels.push((
                                Channel {
                                    z: 0.0,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch,
                                    position: position.clone(),
                                    name: fc.name.clone(),
                                    value: *v,
                                },
                                (*fc).clone(),
                            ));
                            emitted += 1;
                            if emitted >= 4096 {
                                return channels;
                            }
                        }
                    }
                }
            }
        }
        RinexFileType::Navigation => {
            let mut emitted = 0usize;
            let navs = if header.version >= 3.0 {
                parse_rinex_nav_gps3(text)
            } else {
                parse_rinex_nav_gps(text)
            };
            for n in navs {
                if n.epoch_unix > now {
                    continue;
                }
                let Some(epoch) = lsk.unix_to_tdb(n.epoch_unix) else {
                    continue;
                };
                for fc in &fields {
                    let value = match fc.key.as_str() {
                        "a0" => n.a0,
                        "a1" => n.a1,
                        "a2" => n.a2,
                        "iode" => n.iode,
                        "crs" => n.crs,
                        "dn" => n.dn,
                        "m0" => n.m0,
                        "cuc" => n.cuc,
                        "ecc" => n.ecc,
                        "cus" => n.cus,
                        "sqrt_a" => n.sqrt_a,
                        "toe" => n.toe,
                        "cic" => n.cic,
                        "om0" => n.om0,
                        "cis" => n.cis,
                        "i0" => n.i0,
                        "crc" => n.crc,
                        "om" => n.om,
                        "om_dot" => n.om_dot,
                        "idot" => n.idot,
                        "sv_acc" => n.sv_acc,
                        "sv_health" => n.sv_health,
                        "tgd" => n.tgd,
                        "iodc" => n.iodc,
                        "toa" => n.toa,
                        "fit" => n.fit,
                        "week" => n.week,
                        "l2_codes" => n.l2_codes,
                        "l2_p" => n.l2_p,
                        _ => continue,
                    };
                    channels.push((
                        Channel {
                            z: 0.0,
                            freq: 0.0,
                            bin_width: 0.0,
                            epoch,
                            position: Position::Source,
                            name: fc.name.clone(),
                            value,
                        },
                        (*fc).clone(),
                    ));
                    emitted += 1;
                    if emitted >= 4096 {
                        return channels;
                    }
                }
            }
        }
        RinexFileType::Unknown => {}
    }
    channels
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RinexFileType {
    Observation,
    Navigation,
    Unknown,
}

pub struct RinexHeader {
    pub version: f64,
    pub file_type: RinexFileType,
    pub marker_name: String,
    pub approx_pos_xyz: Option<(f64, f64, f64)>,
    pub obs_types: Vec<String>,
    pub interval_s: Option<f64>,
    pub antenna_delta: Option<(f64, f64, f64)>,
}

fn rinex_num(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let e = t.replace('D', "E").replace('d', "E");
    let v = e.parse::<f64>().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn slice(s: &str, a: usize, b: usize) -> Option<&str> {
    s.get(a..b)
}

fn header_label(s: &str) -> &str {
    let b = s.as_bytes();
    let end = b.len().min(80);
    let start = end.saturating_sub(20);
    s.get(start..end).unwrap_or("").trim()
}

fn civil_unix(y: i64, mo: u32, d: u32, h: u32, mi: u32, se: f64) -> Option<f64> {
    let days = days_from_civil(y, mo as i64, d as i64)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + se)
}

pub fn parse_rinex_header(body: &str) -> Option<RinexHeader> {
    let mut version = 0.0f64;
    let mut file_type = RinexFileType::Unknown;
    let mut marker_name = String::new();
    let mut approx_pos_xyz = None;
    let mut obs_types = Vec::new();
    let mut interval_s = None;
    let mut antenna_delta = None;
    let mut have = false;
    for line in body.lines() {
        let label = header_label(line);
        if label == "RINEX VERSION / TYPE" {
            have = true;
            version = rinex_num(slice(line, 0, 9).unwrap_or(""))?;
            let kind = line.get(20..21).unwrap_or("").trim();
            file_type = match kind {
                "O" | "o" => RinexFileType::Observation,
                "N" | "n" => RinexFileType::Navigation,
                _ => RinexFileType::Unknown,
            };
        } else if label == "MARKER NAME" {
            marker_name = line.get(0..60).unwrap_or("").trim().to_string();
        } else if label == "APPROX POSITION XYZ" {
            let x = rinex_num(slice(line, 0, 14).unwrap_or(""));
            let y = rinex_num(slice(line, 14, 28).unwrap_or(""));
            let z = rinex_num(slice(line, 28, 42).unwrap_or(""));
            if let (Some(x), Some(y), Some(z)) = (x, y, z) {
                approx_pos_xyz = Some((x, y, z));
            }
        } else if label.ends_with("# / TYPES OF OBSERV") {
            let mut at = 6usize;
            while at + 6 <= 60 {
                if let Some(w) = slice(line, at, at + 6) {
                    let w = w.trim();
                    if !w.is_empty() {
                        obs_types.push(w.to_string());
                    }
                }
                at += 6;
            }
        } else if label == "INTERVAL" {
            interval_s = rinex_num(slice(line, 0, 10).unwrap_or(""));
        } else if label == "ANTENNA: DELTA H/E/N" {
            let h = rinex_num(slice(line, 0, 14).unwrap_or(""));
            let e = rinex_num(slice(line, 14, 28).unwrap_or(""));
            let n = rinex_num(slice(line, 28, 42).unwrap_or(""));
            if let (Some(h), Some(e), Some(n)) = (h, e, n) {
                antenna_delta = Some((h, e, n));
            }
        } else if label == "END OF HEADER" {
            break;
        }
    }
    if !have {
        return None;
    }
    Some(RinexHeader {
        version,
        file_type,
        marker_name,
        approx_pos_xyz,
        obs_types,
        interval_s,
        antenna_delta,
    })
}

pub struct RinexNavGps {
    pub prn: u32,
    pub epoch_unix: f64,
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub iode: f64,
    pub crs: f64,
    pub dn: f64,
    pub m0: f64,
    pub cuc: f64,
    pub ecc: f64,
    pub cus: f64,
    pub sqrt_a: f64,
    pub toe: f64,
    pub cic: f64,
    pub om0: f64,
    pub cis: f64,
    pub i0: f64,
    pub crc: f64,
    pub om: f64,
    pub om_dot: f64,
    pub idot: f64,
    pub l2_codes: f64,
    pub week: f64,
    pub l2_p: f64,
    pub sv_acc: f64,
    pub sv_health: f64,
    pub tgd: f64,
    pub iodc: f64,
    pub toa: f64,
    pub fit: f64,
}

fn two_digit_year(y: i64) -> i64 {
    if y < 80 { y + 2000 } else { y + 1900 }
}

fn nav_epoch_unix(l0: &str) -> Option<f64> {
    let y = slice(l0, 3, 5)?.trim().parse::<i64>().ok()?;
    let mo = slice(l0, 6, 8)?.trim().parse::<u32>().ok()?;
    let d = slice(l0, 9, 11)?.trim().parse::<u32>().ok()?;
    let h = slice(l0, 12, 14)?.trim().parse::<u32>().ok()?;
    let mi = slice(l0, 15, 17)?.trim().parse::<u32>().ok()?;
    let se = rinex_num(slice(l0, 18, 22).unwrap_or(""))?;
    civil_unix(two_digit_year(y), mo, d, h, mi, se)
}

fn nav3_epoch_unix(l0: &str) -> Option<f64> {
    let y = slice(l0, 4, 8)?.trim().parse::<i64>().ok()?;
    let mo = slice(l0, 9, 11)?.trim().parse::<u32>().ok()?;
    let d = slice(l0, 12, 14)?.trim().parse::<u32>().ok()?;
    let h = slice(l0, 15, 17)?.trim().parse::<u32>().ok()?;
    let mi = slice(l0, 18, 20)?.trim().parse::<u32>().ok()?;
    let se = rinex_num(slice(l0, 21, 23).unwrap_or(""))?;
    civil_unix(y, mo, d, h, mi, se)
}

fn nav_block(
    lines: &[&str],
    i: usize,
    prn: u32,
    epoch_unix: f64,
    l1_off: usize,
    l_off: usize,
) -> Option<RinexNavGps> {
    let f = |line_idx: usize, slot: usize| -> Option<f64> {
        let l = lines.get(i + line_idx)?;
        let a = if line_idx == 0 {
            l1_off + 19 * slot
        } else {
            l_off + 19 * slot
        };
        let end = (a + 19).min(l.len());
        if end <= a {
            return None;
        }
        rinex_num(&l[a..end])
    };
    let (Some(a0), Some(a1), Some(a2)) = (f(0, 0), f(0, 1), f(0, 2)) else {
        return None;
    };
    let (Some(iode), Some(crs), Some(dn), Some(m0)) = (f(1, 0), f(1, 1), f(1, 2), f(1, 3)) else {
        return None;
    };
    let (Some(cuc), Some(ecc), Some(cus), Some(sqrt_a)) = (f(2, 0), f(2, 1), f(2, 2), f(2, 3))
    else {
        return None;
    };
    let (Some(toe), Some(cic), Some(om0), Some(cis)) = (f(3, 0), f(3, 1), f(3, 2), f(3, 3)) else {
        return None;
    };
    let (Some(i0), Some(crc), Some(om), Some(om_dot)) = (f(4, 0), f(4, 1), f(4, 2), f(4, 3)) else {
        return None;
    };
    let (Some(idot), Some(l2_codes), Some(week), Some(l2_p)) = (f(5, 0), f(5, 1), f(5, 2), f(5, 3))
    else {
        return None;
    };
    let (Some(sv_acc), Some(sv_health), Some(tgd), Some(iodc)) =
        (f(6, 0), f(6, 1), f(6, 2), f(6, 3))
    else {
        return None;
    };
    let (Some(toa), Some(fit)) = (f(7, 0), f(7, 1)) else {
        return None;
    };
    Some(RinexNavGps {
        prn,
        epoch_unix,
        a0,
        a1,
        a2,
        iode,
        crs,
        dn,
        m0,
        cuc,
        ecc,
        cus,
        sqrt_a,
        toe,
        cic,
        om0,
        cis,
        i0,
        crc,
        om,
        om_dot,
        idot,
        l2_codes,
        week,
        l2_p,
        sv_acc,
        sv_health,
        tgd,
        iodc,
        toa,
        fit,
    })
}

pub fn parse_rinex_nav_gps(body: &str) -> Vec<RinexNavGps> {
    let lines: Vec<&str> = body.lines().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 8 <= lines.len() {
        let l0 = lines[i];
        let Some(prn) = l0
            .get(0..2)
            .and_then(|s| s.trim().parse::<u32>().ok())
            .filter(|p| *p > 0)
        else {
            i += 1;
            continue;
        };
        let Some(epoch_unix) = nav_epoch_unix(l0) else {
            i += 1;
            continue;
        };
        if let Some(n) = nav_block(&lines, i, prn, epoch_unix, 22, 3) {
            out.push(n);
        }
        i += 8;
    }
    out
}

pub fn parse_rinex_nav_gps3(body: &str) -> Vec<RinexNavGps> {
    let lines: Vec<&str> = body.lines().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 8 <= lines.len() {
        let l0 = lines[i];
        let first = l0.chars().next().unwrap_or(' ');
        if first != 'G' && first != 'g' {
            i += 1;
            continue;
        }
        let Some(prn) = l0
            .get(1..3)
            .and_then(|s| s.trim().parse::<u32>().ok())
            .filter(|p| *p > 0)
        else {
            i += 1;
            continue;
        };
        let Some(epoch_unix) = nav3_epoch_unix(l0) else {
            i += 1;
            continue;
        };
        if let Some(n) = nav_block(&lines, i, prn, epoch_unix, 23, 4) {
            out.push(n);
        }
        i += 8;
    }
    out
}

pub struct RinexObsSat {
    pub sat: String,
    pub values: Vec<Option<f64>>,
}

pub struct RinexObsEpoch {
    pub epoch_unix: f64,
    pub epoch_flag: u32,
    pub sats: Vec<RinexObsSat>,
}

fn obs_epoch_unix(line: &str) -> Option<f64> {
    let y = slice(line, 0, 3)?.trim().parse::<i64>().ok()?;
    let mo = slice(line, 3, 6)?.trim().parse::<u32>().ok()?;
    let d = slice(line, 6, 9)?.trim().parse::<u32>().ok()?;
    let h = slice(line, 9, 12)?.trim().parse::<u32>().ok()?;
    let mi = slice(line, 12, 15)?.trim().parse::<u32>().ok()?;
    let se = rinex_num(slice(line, 15, 26).unwrap_or(""))?;
    civil_unix(two_digit_year(y), mo, d, h, mi, se)
}

pub fn parse_rinex_obs(body: &str, n_obs: usize) -> Vec<RinexObsEpoch> {
    let lines: Vec<&str> = body.lines().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() && header_label(lines[i]) != "END OF HEADER" {
        i += 1;
    }
    i += 1;
    while i < lines.len() {
        let line = lines[i];
        if line.len() < 32 {
            i += 1;
            continue;
        }
        let Some(epoch_unix) = obs_epoch_unix(line) else {
            i += 1;
            continue;
        };
        let flag = match slice(line, 26, 29).and_then(|s| s.trim().parse::<u32>().ok()) {
            Some(v) => v,
            None => {
                i += 1;
                continue;
            }
        };
        let n_sat = match slice(line, 29, 32).and_then(|s| s.trim().parse::<usize>().ok()) {
            Some(v) => v,
            None => {
                i += 1;
                continue;
            }
        };
        if n_sat == 0 {
            i += 1;
            continue;
        }
        let mut sats: Vec<String> = Vec::with_capacity(n_sat);
        let mut cur = line;
        loop {
            let mut at = 32usize;
            while at + 3 <= cur.len() && sats.len() < n_sat {
                let sat = cur.get(at..at + 3).unwrap_or("").trim().to_string();
                if !sat.is_empty() {
                    sats.push(sat);
                }
                at += 3;
            }
            if sats.len() >= n_sat || i + 1 >= lines.len() {
                break;
            }
            i += 1;
            cur = lines[i];
        }
        i += 1;
        let lines_per_sat = (n_obs + 4) / 5;
        let mut epoch = RinexObsEpoch {
            epoch_unix,
            epoch_flag: flag,
            sats: Vec::new(),
        };
        for sat in sats {
            if i >= lines.len() {
                break;
            }
            let mut values: Vec<Option<f64>> = vec![None; n_obs];
            for k in 0..lines_per_sat {
                if i >= lines.len() {
                    break;
                }
                let rec_line = lines[i];
                for j in 0..5 {
                    let idx = 5 * k + j;
                    if idx >= n_obs {
                        break;
                    }
                    let at = 16 * j;
                    if rec_line.len() >= at + 14 {
                        values[idx] = rinex_num(&rec_line[at..at + 14]);
                    }
                }
                i += 1;
            }
            if n_obs > 0 {
                epoch.sats.push(RinexObsSat { sat, values });
            }
        }
        out.push(epoch);
    }
    out
}

const CRX_V1_NUMSAT_OFFSET: usize = 28;
const CRX_V1_SAT_OFFSET: usize = 31;

pub fn is_hatanaka(body: &str) -> bool {
    body.lines()
        .take(5)
        .any(|l| l.contains("CRINEX VERS") || l.contains("COMPACT RINEX FORMAT"))
}

struct CrxTextDiff {
    buffer: Vec<u8>,
}

impl CrxTextDiff {
    fn new(data: &str) -> Self {
        Self {
            buffer: data.as_bytes().to_vec(),
        }
    }

    fn force_init(&mut self, data: &str) {
        self.buffer = data.as_bytes().to_vec();
    }

    fn decompress(&mut self, data: &str) -> String {
        let bytes = data.as_bytes();
        if bytes.len() > self.buffer.len() {
            let from = self.buffer.len();
            self.buffer.extend_from_slice(&bytes[from..]);
        }
        for (i, &byte) in bytes.iter().enumerate() {
            if let Some(b) = self.buffer.get_mut(i) {
                if byte != b' ' {
                    *b = if byte == b'&' { b' ' } else { byte };
                }
            }
        }
        String::from_utf8_lossy(&self.buffer).into_owned()
    }
}

struct CrxNumDiff {
    level: usize,
    m: usize,
    buf: [i64; 6],
}

impl CrxNumDiff {
    fn new(data: i64, level: usize) -> Self {
        let mut buf = [0i64; 6];
        buf[0] = data;
        Self { level, m: 0, buf }
    }

    fn force_init(&mut self, data: i64, level: usize) {
        self.level = level;
        self.m = 0;
        self.rotate(data);
    }

    fn rotate(&mut self, data: i64) {
        self.buf.copy_within(0..5, 1);
        self.buf[0] = data;
    }

    fn decompress(&mut self, data: i64) -> i64 {
        if self.m < self.level {
            self.m += 1;
        }
        let new = match self.m {
            1 => data + self.buf[0],
            2 => data + 2 * self.buf[0] - self.buf[1],
            3 => data + 3 * self.buf[0] - 3 * self.buf[1] + self.buf[2],
            4 => data + 4 * self.buf[0] - 6 * self.buf[1] + 4 * self.buf[2] - self.buf[3],
            5 => {
                data + 5 * self.buf[0] - 10 * self.buf[1] + 10 * self.buf[2] - 5 * self.buf[3]
                    + self.buf[4]
            }
            6 => {
                data + 6 * self.buf[0] - 15 * self.buf[1] + 20 * self.buf[2] - 15 * self.buf[3]
                    + 6 * self.buf[4]
                    - self.buf[5]
            }
            _ => data,
        };
        self.rotate(new);
        new
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CrxState {
    Epoch,
    Clock,
    Reading,
}

struct CrxDecoder {
    state: CrxState,
    numobs: usize,
    numsat: usize,
    flag: u32,
    sat_index: usize,
    epoch_diff: CrxTextDiff,
    epoch_descriptor: String,
    clock_diff: CrxNumDiff,
    obs_diff: HashMap<([u8; 3], usize), CrxNumDiff>,
    clock: Option<i64>,
}

impl CrxDecoder {
    fn new(numobs: usize) -> Self {
        Self {
            state: CrxState::Epoch,
            numobs,
            numsat: 0,
            flag: 0,
            sat_index: 0,
            epoch_diff: CrxTextDiff::new(""),
            epoch_descriptor: String::new(),
            clock_diff: CrxNumDiff::new(0, 0),
            obs_diff: HashMap::new(),
            clock: None,
        }
    }

    fn read_epoch(&mut self, line: &str) -> bool {
        if line.len() < 17 {
            return false;
        }
        let trimmed = line.get(1..).unwrap_or("").trim_end();
        if line.starts_with('&') {
            self.epoch_diff.force_init(trimmed);
            self.epoch_descriptor = trimmed.to_string();
        } else {
            self.epoch_descriptor = self.epoch_diff.decompress(trimmed);
        }
        let Some(numsat) = self
            .epoch_descriptor
            .get(CRX_V1_NUMSAT_OFFSET..CRX_V1_NUMSAT_OFFSET + 3)
            .and_then(|s| s.trim().parse::<usize>().ok())
        else {
            return false;
        };
        let Some(flag) = self
            .epoch_descriptor
            .get(25..28)
            .and_then(|s| s.trim().parse::<u32>().ok())
        else {
            return false;
        };
        self.numsat = numsat;
        self.flag = flag;
        true
    }

    fn read_clock(&mut self, line: &str) {
        self.clock = None;
        let len = line.len();
        if len > 2 {
            if line.get(1..).is_some_and(|s| s.starts_with('&')) {
                let order = line.get(..1).and_then(|s| s.parse::<usize>().ok());
                let val = line.get(2..).and_then(|s| s.parse::<i64>().ok());
                if let (Some(order), Some(val)) = (order, val) {
                    if order <= 6 {
                        self.clock_diff.force_init(val, order);
                        self.clock = Some(val);
                    }
                }
            } else if let Ok(val) = line.trim().parse::<i64>() {
                self.clock = Some(self.clock_diff.decompress(val));
            }
        } else if len == 1 {
            if let Ok(val) = line.trim().parse::<i64>() {
                self.clock = Some(self.clock_diff.decompress(val));
            }
        }
    }

    fn write_epoch(&self, out: &mut String) {
        let desc = self.epoch_descriptor.as_bytes();
        let first_len = desc.len().min(67);
        out.push(' ');
        if let Ok(s) = std::str::from_utf8(&desc[..first_len]) {
            out.push_str(s);
        }
        if let Some(clock) = self.clock {
            out.push_str(&format!(" {:15.12}", clock as f64));
        }
        out.push('\n');
        let extra = if self.numsat > 12 {
            (self.numsat - 1) / 12
        } else {
            0
        };
        let mut offset = 67usize;
        for _ in 0..extra {
            out.push_str("                                ");
            let end = (offset + 36).min(desc.len());
            if let Some(chunk) = desc
                .get(offset..end)
                .and_then(|c| std::str::from_utf8(c).ok())
            {
                out.push_str(chunk);
            }
            out.push('\n');
            offset += 36;
        }
    }

    fn sat_id(&self) -> Option<[u8; 3]> {
        let start = CRX_V1_SAT_OFFSET + self.sat_index * 3;
        let b = self.epoch_descriptor.as_bytes().get(start..start + 3)?;
        Some([b[0], b[1], b[2]])
    }

    fn read_obs(&mut self, line: &str) -> Vec<Option<f64>> {
        let Some(sat) = self.sat_id() else {
            return Vec::new();
        };
        let len = line.len();
        let mut consumed = 0usize;
        let mut values = Vec::with_capacity(self.numobs);
        let mut ptr = 0usize;
        while ptr < self.numobs {
            if consumed >= len {
                values.push(None);
                ptr += 1;
                continue;
            }
            let rest = &line[consumed..];
            let Some(offset) = rest.find(' ') else {
                let slice = rest.trim();
                values.push(self.decode_token(sat, ptr, slice));
                break;
            };
            if offset > 1 {
                let slice = rest[..offset].trim();
                values.push(self.decode_token(sat, ptr, slice));
                consumed += offset + 1;
            } else {
                let slice = rest[..offset].trim();
                if slice.is_empty() {
                    values.push(None);
                    consumed += 1;
                } else {
                    values.push(self.decode_token(sat, ptr, slice));
                    consumed += 2;
                }
            }
            ptr += 1;
        }
        while values.len() < self.numobs {
            values.push(None);
        }
        values
    }

    fn decode_token(&mut self, sat: [u8; 3], ptr: usize, slice: &str) -> Option<f64> {
        if slice.is_empty() {
            return None;
        }
        if let Some(amp) = slice.find('&') {
            if amp == 1 {
                let level = slice.get(..amp).and_then(|s| s.parse::<usize>().ok());
                let value = slice.get(amp + 1..).and_then(|s| s.parse::<i64>().ok());
                if let (Some(level), Some(value)) = (level, value) {
                    if level <= 6 {
                        self.obs_diff
                            .insert((sat, ptr), CrxNumDiff::new(value, level));
                        return Some(value as f64 / 1000.0);
                    }
                }
            }
            return None;
        }
        if let Ok(value) = slice.parse::<i64>() {
            if let Some(kernel) = self.obs_diff.get_mut(&(sat, ptr)) {
                return Some(kernel.decompress(value) as f64 / 1000.0);
            }
        }
        None
    }

    fn write_obs(&self, out: &mut String, values: &[Option<f64>]) {
        for (i, v) in values.iter().enumerate() {
            match v {
                Some(x) => out.push_str(&format!("{x:14.3}  ")),
                None => out.push_str("                "),
            }
            if i % 5 == 4 {
                out.push('\n');
            }
        }
        if values.len() % 5 != 0 {
            out.push('\n');
        }
    }
}

pub fn crx2rnx(body: &str) -> Option<String> {
    let header = parse_rinex_header(body)?;
    let numobs = header.obs_types.len();
    if numobs == 0 {
        return None;
    }
    let lines: Vec<&str> = body.lines().collect();
    let end_header = lines
        .iter()
        .position(|l| header_label(l) == "END OF HEADER")?;
    let mut out = String::with_capacity(body.len() * 2);
    for l in &lines[..=end_header] {
        out.push_str(l);
        out.push('\n');
    }
    let mut dec = CrxDecoder::new(numobs);
    let mut i = end_header + 1;
    while i < lines.len() {
        match dec.state {
            CrxState::Epoch => {
                if !dec.read_epoch(lines[i]) {
                    i += 1;
                    continue;
                }
                i += 1;
                if dec.flag >= 2 {
                    dec.write_epoch(&mut out);
                    for _ in 0..dec.numsat {
                        if i >= lines.len() {
                            break;
                        }
                        out.push_str(lines[i]);
                        out.push('\n');
                        i += 1;
                    }
                    dec.state = CrxState::Epoch;
                } else {
                    dec.state = CrxState::Clock;
                }
            }
            CrxState::Clock => {
                dec.read_clock(lines[i]);
                dec.write_epoch(&mut out);
                dec.sat_index = 0;
                dec.state = if dec.numsat == 0 {
                    CrxState::Epoch
                } else {
                    CrxState::Reading
                };
                i += 1;
            }
            CrxState::Reading => {
                let values = dec.read_obs(lines[i]);
                dec.write_obs(&mut out, &values);
                i += 1;
                dec.sat_index += 1;
                if dec.sat_index >= dec.numsat {
                    dec.state = CrxState::Epoch;
                }
            }
        }
    }
    Some(out)
}

pub const SBF_SYNC: [u8; 2] = [0x24, 0x40];

pub struct SbfBlock {
    pub id: u16,
    pub payload: Vec<u8>,
}

pub fn sbf_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0x0000;
    for &b in data {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x1021
            } else {
                crc << 1
            };
        }
    }
    crc
}

pub fn sbf_blocks(bytes: &[u8]) -> Vec<SbfBlock> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 8 <= bytes.len() {
        if bytes[i] != SBF_SYNC[0] || bytes[i + 1] != SBF_SYNC[1] {
            i += 1;
            continue;
        }
        let stored_crc = u16::from_le_bytes([bytes[i + 2], bytes[i + 3]]);
        let id = u16::from_le_bytes([bytes[i + 4], bytes[i + 5]]);
        let length = u16::from_le_bytes([bytes[i + 6], bytes[i + 7]]) as usize;
        if length < 8 || i + length > bytes.len() {
            i += 1;
            continue;
        }
        if sbf_crc16(&bytes[i + 4..i + length]) != stored_crc {
            i += 1;
            continue;
        }
        out.push(SbfBlock {
            id,
            payload: bytes[i + 8..i + length].to_vec(),
        });
        i += length;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d19(v: &str) -> String {
        format!("{v:>19}")
    }

    fn nav_file() -> String {
        let mut s = String::new();
        s.push_str(
            "     2.11           N: GPS NAV DATA                         RINEX VERSION / TYPE\n",
        );
        s.push_str(
            "XXRINEXN V1.0       AIUB                04-JAN-24 00:00     PGM / RUN BY / DATE \n",
        );
        s.push_str(
            "                                                            END OF HEADER       \n",
        );
        let l1 = format!(
            "{:>2} {:02} {:>2} {:>2} {:>2} {:>2} {:>4}{}{}{}",
            1u32,
            24u32,
            1u32,
            4u32,
            0u32,
            0u32,
            "0.0",
            d19(".465376257896D-04"),
            d19(".227373675443D-11"),
            d19(".000000000000D+00")
        );
        let l2 = format!(
            "   {}{}{}{}",
            d19(".100000000000D+01"),
            d19(".812500000000D+02"),
            d19(".463081082618D-08"),
            d19(".182379839194D+01")
        );
        let l3 = format!(
            "   {}{}{}{}",
            d19(".454302370548D-05"),
            d19(".920886592493D-02"),
            d19(".142462804914D-04"),
            d19(".515365489006D+04")
        );
        let l4 = format!(
            "   {}{}{}{}",
            d19(".720000000000D+05"),
            d19("-.158324837685D-06"),
            d19(".306375833900D+01"),
            d19(".819563865662D-06")
        );
        let l5 = format!(
            "   {}{}{}{}",
            d19(".958295770100D+00"),
            d19(".159218750000D+02"),
            d19("-.271653748420D+01"),
            d19("-.801984540900D-08")
        );
        let l6 = format!(
            "   {}{}{}{}",
            d19(".102672990769D-09"),
            d19(".000000000000D+00"),
            d19(".234000000000D+04"),
            d19(".000000000000D+00")
        );
        let l7 = format!(
            "   {}{}{}{}",
            d19(".200000000000D+01"),
            d19(".000000000000D+00"),
            d19("-.139698386192D-08"),
            d19(".234000000000D+04")
        );
        let l8 = format!(
            "   {}{}{}{}",
            d19(".720000000000D+05"),
            d19(".400000000000D+01"),
            d19(".000000000000D+00"),
            d19(".000000000000D+00")
        );
        for l in [l1, l2, l3, l4, l5, l6, l7, l8] {
            s.push_str(&l);
            s.push('\n');
        }
        s
    }

    fn obs_file() -> String {
        let mut s = String::new();
        s.push_str(
            "     2.11           OBSERVATION DATA    G (GPS)             RINEX VERSION / TYPE\n",
        );
        s.push_str(&format!("{:60}{:>20}\n", "TEST", "MARKER NAME"));
        s.push_str(
            "  4100516.2851  -455185.4244   4404346.7061                  APPROX POSITION XYZ\n",
        );
        s.push_str(
            "        0.0000        0.0000        0.0000                  ANTENNA: DELTA H/E/N\n",
        );
        s.push_str(
            "     4    C1    L1    S1    D1                              # / TYPES OF OBSERV\n",
        );
        s.push_str("    30.0000                                                  INTERVAL\n");
        s.push_str(
            "                                                            END OF HEADER       \n",
        );
        let epoch = format!(
            "{:>3}{:>3}{:>3}{:>3}{:>3}{:>11.7}{:>3}{:>3}G01G02",
            24i64, 1i64, 4i64, 0i64, 0i64, 0.0, 0i64, 2i64
        );
        s.push_str(&epoch);
        s.push('\n');
        let cell = |v: Option<f64>| -> String {
            match v {
                Some(x) => format!("{x:>14.3}  "),
                None => "                ".to_string(),
            }
        };
        let rec = |c1: Option<f64>, l1: Option<f64>, s1: Option<f64>, d1: Option<f64>| {
            format!("{}{}{}{}", cell(c1), cell(l1), cell(s1), cell(d1))
        };
        s.push_str(&rec(
            Some(21345678.123),
            Some(-12345678.123),
            None,
            Some(1234.567),
        ));
        s.push('\n');
        s.push_str(&rec(
            Some(21345679.123),
            Some(-12345679.123),
            None,
            Some(1234.567),
        ));
        s.push('\n');
        s
    }

    #[test]
    fn header_parses_version_type_position_obs_types() {
        let h = parse_rinex_header(&obs_file()).unwrap();
        assert_eq!(h.version, 2.11);
        assert_eq!(h.file_type, RinexFileType::Observation);
        assert_eq!(h.marker_name, "TEST");
        let (x, y, z) = h.approx_pos_xyz.unwrap();
        assert!((x - 4100516.2851).abs() < 1e-3);
        assert!((y - -455185.4244).abs() < 1e-3);
        assert!((z - 4404346.7061).abs() < 1e-3);
        assert_eq!(h.obs_types, vec!["C1", "L1", "S1", "D1"]);
        assert_eq!(h.interval_s, Some(30.0));
    }

    #[test]
    fn nav_parses_one_gps_ephemeris_block() {
        let h = parse_rinex_header(&nav_file()).unwrap();
        assert_eq!(h.file_type, RinexFileType::Navigation);
        let navs = parse_rinex_nav_gps(&nav_file());
        assert_eq!(navs.len(), 1);
        let n = &navs[0];
        assert_eq!(n.prn, 1);
        assert!((n.a0 - 0.465376257896e-4).abs() < 1e-16);
        assert!((n.sqrt_a - 5153.65489006).abs() < 1e-6);
        assert!((n.ecc - 0.920886592493e-2).abs() < 1e-12);
        assert!((n.om0 - 3.063758339).abs() < 1e-9);
    }

    #[test]
    fn obs_parses_epoch_and_satellite_values() {
        let epochs = parse_rinex_obs(&obs_file(), 4);
        assert_eq!(epochs.len(), 1);
        let e = &epochs[0];
        assert_eq!(e.epoch_flag, 0);
        let expected = civil_unix(2024, 1, 4, 0, 0, 0.0).unwrap();
        assert!((e.epoch_unix - expected).abs() < 1e-9);
        assert_eq!(e.sats.len(), 2);
        assert_eq!(e.sats[0].sat, "G01");
        assert_eq!(e.sats[1].sat, "G02");
        assert_eq!(e.sats[0].values.len(), 4);
        assert!((e.sats[0].values[0].unwrap() - 21345678.123).abs() < 1e-3);
        assert!(e.sats[0].values[2].is_none(), "blank S1 stays absent");
    }

    #[test]
    fn obs_epoch_line_continuation_reads_over_twelve_satellites() {
        let mut s = String::new();
        s.push_str(
            "     2.11           OBSERVATION DATA    G (GPS)             RINEX VERSION / TYPE\n",
        );
        s.push_str(
            "                                                            END OF HEADER       \n",
        );
        let epoch_head = format!(
            "{:>3}{:>3}{:>3}{:>3}{:>3}{:>11.7}{:>3}{:>3}",
            24i64, 1i64, 4i64, 0i64, 0i64, 0.0, 0i64, 13i64
        );
        s.push_str(&format!(
            "{epoch_head}G01G02G03G04G05G06G07G08G09G10G11G12\n"
        ));
        s.push_str(&format!("{:<32}G13\n", ""));
        for i in 0..13 {
            let c1 = 21345678.0 + i as f64;
            let l1 = -12345678.0 + i as f64;
            s.push_str(&format!("{c1:>14.3}  {l1:>14.3}  \n"));
        }
        let epochs = parse_rinex_obs(&s, 2);
        assert_eq!(epochs.len(), 1);
        let e = &epochs[0];
        assert_eq!(e.sats.len(), 13);
        assert_eq!(e.sats[0].sat, "G01");
        assert_eq!(e.sats[11].sat, "G12");
        assert_eq!(e.sats[12].sat, "G13");
        assert!((e.sats[12].values[0].unwrap() - 21345690.0).abs() < 1e-3);
        assert!((e.sats[12].values[1].unwrap() - -12345666.0).abs() < 1e-3);
    }

    fn rinex3_nav_file() -> String {
        let mut s = String::new();
        s.push_str(
            "     3.04           N: GNSS NAV DATA    M: MIXED            RINEX VERSION / TYPE\n",
        );
        s.push_str(
            "                                                            END OF HEADER       \n",
        );
        for l in [
            "G01 2026 09 06 00 00 00 1.792814582590E-04-9.094947017730E-12 0.000000000000E+00",
            "     1.080000000000E+02 8.712500000000E+01 4.353752779850E-09-1.504221466100E+00",
            "     4.678964614870E-06 1.924931886610E-03 5.586072802540E-06 5.153594476700E+03",
            "     0.000000000000E+00-5.587935447690E-08-2.464042733270E-01-3.352761268620E-08",
            "     9.568097449780E-01 2.715937500000E+02 1.482051131320E-01-8.083908155790E-09",
            "    -1.750072897560E-11 1.000000000000E+00 2.435000000000E+03 0.000000000000E+00",
            "     2.000000000000E+00 0.000000000000E+00-8.847564458850E-09 3.640000000000E+02",
            "    -6.120000000000E+02 4.000000000000E+00",
        ] {
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    fn sbf_frame(id: u16, payload: &[u8]) -> Vec<u8> {
        let length = 8 + payload.len();
        let mut frame = Vec::with_capacity(length);
        frame.extend_from_slice(&SBF_SYNC);
        frame.extend_from_slice(&[0u8; 2]);
        frame.extend_from_slice(&id.to_le_bytes());
        frame.extend_from_slice(&(length as u16).to_le_bytes());
        frame.extend_from_slice(payload);
        let crc = sbf_crc16(&frame[4..]);
        frame[2..4].copy_from_slice(&crc.to_le_bytes());
        frame
    }

    #[test]
    fn sbf_crc16_matches_published_check_value() {
        assert_eq!(sbf_crc16(b"123456789"), 0x31C3);
    }

    #[test]
    fn sbf_blocks_frames_payloads() {
        let mut stream = Vec::new();
        stream.extend_from_slice(b"noise");
        stream.extend_from_slice(&sbf_frame(4027, &[1, 2, 3, 4, 5, 6, 7, 8]));
        stream.extend_from_slice(&sbf_frame(4006, &[9, 10, 11, 12]));
        let blocks = sbf_blocks(&stream);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].id, 4027);
        assert_eq!(blocks[0].payload, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(blocks[1].id, 4006);
        assert_eq!(blocks[1].payload, vec![9, 10, 11, 12]);
    }

    #[test]
    fn sbf_blocks_drop_bad_crc() {
        let mut frame = sbf_frame(4027, &[1, 2, 3, 4]);
        frame[8] ^= 0xFF;
        assert!(sbf_blocks(&frame).is_empty());
    }

    #[test]
    fn sbf_blocks_drop_truncated_frame() {
        let frame = sbf_frame(4027, &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(sbf_blocks(&frame[..6]).is_empty());
    }

    #[test]
    fn rinex3_nav_parses_gps_ephemeris() {
        let h = parse_rinex_header(&rinex3_nav_file()).unwrap();
        assert_eq!(h.version, 3.04);
        assert_eq!(h.file_type, RinexFileType::Navigation);
        let navs = parse_rinex_nav_gps3(&rinex3_nav_file());
        assert_eq!(navs.len(), 1);
        let n = &navs[0];
        assert_eq!(n.prn, 1);
        assert!((n.a0 - 1.792814582590e-4).abs() < 1e-16);
        assert!((n.a1 - -9.094947017730e-12).abs() < 1e-20);
        assert!((n.sqrt_a - 5153.594476700).abs() < 1e-6);
        assert!((n.ecc - 1.924931886610e-3).abs() < 1e-14);
        assert!((n.om0 - -2.464042733270e-1).abs() < 1e-12);
        assert!((n.i0 - 9.568097449780e-1).abs() < 1e-12);
    }

    fn crx_epoch(
        y: i64,
        mo: i64,
        d: i64,
        h: i64,
        mi: i64,
        se: f64,
        flag: i64,
        n: i64,
        sats: &str,
    ) -> String {
        format!("{y:>2}{mo:>3}{d:>3}{h:>3}{mi:>3}{se:>11.7}{flag:>3}{n:>3}{sats}")
    }

    fn crx_header() -> String {
        let mut s = String::new();
        s.push_str(
            "1.0                 COMPACT RINEX FORMAT                    CRINEX VERS   / TYPE\n",
        );
        s.push_str(
            "     2.11           OBSERVATION DATA    M (MIXED)           RINEX VERSION / TYPE\n",
        );
        s.push_str(
            "     2    C1    L1                                          # / TYPES OF OBSERV\n",
        );
        s.push_str(
            "                                                            END OF HEADER       \n",
        );
        s
    }

    #[test]
    fn crx_textdiff_recovers_masked_text() {
        let mut diff = CrxTextDiff::new("ABCDEFG 12 000 33 XXACQmpLf");
        let compressed = [
            "         3   1 44 xxACq   F",
            "        4 ",
            " 11 22   x   0 4  y     p  ",
            "              1     ",
            "                   z",
            " ",
            "                           &",
            "&                           ",
            " ",
        ];
        let expected = [
            "ABCDEFG 13 001 44 xxACqmpLF",
            "ABCDEFG 43 001 44 xxACqmpLF",
            "A11D22G 4x 000 44 yxACqmpLF",
            "A11D22G 4x 000144 yxACqmpLF",
            "A11D22G 4x 000144 yzACqmpLF",
            "A11D22G 4x 000144 yzACqmpLF",
            "A11D22G 4x 000144 yzACqmpLF ",
            " 11D22G 4x 000144 yzACqmpLF ",
            " 11D22G 4x 000144 yzACqmpLF ",
        ];
        for i in 0..compressed.len() {
            assert_eq!(diff.decompress(compressed[i]), expected[i], "at {i}");
        }
    }

    #[test]
    fn crx_numdiff_recovers_differences() {
        let mut diff = CrxNumDiff::new(126298057858, 3);
        assert_eq!(diff.decompress(-15603288), 126282454570);
        assert_eq!(diff.decompress(521089), 126267372371);
        assert_eq!(diff.decompress(-752), 126252810509);
        assert_eq!(diff.decompress(1575419284), 127814188268);
        assert_eq!(diff.decompress(-3150848707), 127800656941);
        assert_eq!(diff.decompress(1575424909), 127787641437);
        assert_eq!(diff.decompress(-135), 127775141621);
        diff.force_init(111982965979, 3);
        assert_eq!(diff.decompress(-16266911), 111966699068);
        assert_eq!(diff.decompress(609858), 111951042015);
        assert_eq!(diff.decompress(-213), 111935994607);
    }

    #[test]
    fn hatanaka_crx_v1_roundtrips_epochs_and_satellites() {
        let mut s = crx_header();
        s.push('&');
        s.push_str(&crx_epoch(24, 1, 1, 0, 0, 0.0, 0, 1, "G01"));
        s.push('\n');
        s.push('\n');
        s.push_str("1&1000000 1&2000000\n");
        s.push('&');
        s.push_str(&crx_epoch(24, 1, 1, 0, 0, 30.0, 0, 1, "G01"));
        s.push('\n');
        s.push('\n');
        s.push_str("1000 2000\n");

        assert!(is_hatanaka(&s));
        let rnx = crx2rnx(&s).unwrap();
        let epochs = parse_rinex_obs(&rnx, 2);
        assert_eq!(epochs.len(), 2);
        assert_eq!(epochs[0].sats.len(), 1);
        assert_eq!(epochs[0].sats[0].sat, "G01");
        assert!((epochs[0].sats[0].values[0].unwrap() - 1000.0).abs() < 1e-6);
        assert!((epochs[0].sats[0].values[1].unwrap() - 2000.0).abs() < 1e-6);
        assert!((epochs[1].sats[0].values[0].unwrap() - 1001.0).abs() < 1e-6);
        assert!((epochs[1].sats[0].values[1].unwrap() - 2002.0).abs() < 1e-6);
        let expected = civil_unix(2024, 1, 1, 0, 0, 30.0).unwrap();
        assert!((epochs[1].epoch_unix - expected).abs() < 1e-9);
    }

    #[test]
    fn hatanaka_event_epoch_passes_records_through() {
        let mut s = crx_header();
        s.push('&');
        s.push_str(&crx_epoch(24, 1, 1, 1, 0, 0.0, 4, 1, ""));
        s.push('\n');
        s.push_str("101 (COGO code)                                             COMMENT\n");
        s.push('&');
        s.push_str(&crx_epoch(24, 1, 1, 1, 0, 0.0, 0, 1, "G01"));
        s.push('\n');
        s.push('\n');
        s.push_str("1&1000000 1&2000000\n");

        let rnx = crx2rnx(&s).unwrap();
        assert!(rnx.contains("101 (COGO code)"));
        assert!(rnx.contains("G01"));
    }

    #[test]
    fn hatanaka_plain_rinex_is_left_alone() {
        assert!(!is_hatanaka(&obs_file()));
    }
}
