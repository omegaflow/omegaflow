use super::*;
use crate::lsk::days_from_civil;

fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> Option<(f64, f64, f64)> {
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
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
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
            if let Some(n) = rinex_num(slice(line, 0, 6).unwrap_or("")).map(|v| v as usize) {
                for k in 0..n {
                    let at = 6 + 6 * k;
                    if let Some(w) = slice(line, at, at + 6) {
                        let w = w.trim();
                        if !w.is_empty() {
                            obs_types.push(w.to_string());
                        }
                    }
                }
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
    if y < 80 {
        y + 2000
    } else {
        y + 1900
    }
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
    let y = slice(line, 3, 5)?.trim().parse::<i64>().ok()?;
    let mo = slice(line, 6, 8)?.trim().parse::<u32>().ok()?;
    let d = slice(line, 9, 11)?.trim().parse::<u32>().ok()?;
    let h = slice(line, 12, 14)?.trim().parse::<u32>().ok()?;
    let mi = slice(line, 15, 17)?.trim().parse::<u32>().ok()?;
    let se = rinex_num(slice(line, 18, 29).unwrap_or(""))?;
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
        if line.len() < 29 {
            i += 1;
            continue;
        }
        let Some(epoch_unix) = obs_epoch_unix(line) else {
            i += 1;
            continue;
        };
        let flag = match slice(line, 30, 32).and_then(|s| s.trim().parse::<u32>().ok()) {
            Some(v) => v,
            None => {
                i += 1;
                continue;
            }
        };
        let n_sat = match slice(line, 32, 35).and_then(|s| s.trim().parse::<usize>().ok()) {
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
        i += 1;
        let mut epoch = RinexObsEpoch {
            epoch_unix,
            epoch_flag: flag,
            sats: Vec::new(),
        };
        for _ in 0..n_sat {
            if i >= lines.len() {
                break;
            }
            let sat_line = lines[i];
            let sat = sat_line.get(0..3).unwrap_or("").trim().to_string();
            if sat.is_empty() {
                break;
            }
            i += 1;
            let mut values = Vec::new();
            let mut at = 3usize;
            for _ in 0..n_obs {
                if sat_line.len() < at + 14 {
                    break;
                }
                let v = rinex_num(&sat_line[at..at + 14]);
                values.push(v);
                at += 16;
            }
            if !values.is_empty() {
                epoch.sats.push(RinexObsSat { sat, values });
            }
        }
        out.push(epoch);
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
            "{:>2} {:02} {:>2} {:>2} {:>2} {:>2} {:>11.7} {:>2}{:>3}",
            0u32, 24u32, 1u32, 4u32, 0u32, 0u32, 0.0, 0u32, 2u32
        );
        s.push_str(&epoch);
        s.push('\n');
        let sat1 = format!(
            "G01{:>14.3}  {:>14.3}  {:>14}  {:>14.3}",
            21345678.123f64, -12345678.123f64, "", 1234.567f64
        );
        let sat2 = format!(
            "G02{:>14.3}  {:>14.3}  {:>14}  {:>14.3}",
            21345679.123f64, -12345679.123f64, "", 1234.567f64
        );
        s.push_str(&sat1);
        s.push('\n');
        s.push_str(&sat2);
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
        assert_eq!(e.sats.len(), 2);
        assert_eq!(e.sats[0].sat, "G01");
        assert_eq!(e.sats[0].values.len(), 4);
        assert!((e.sats[0].values[0].unwrap() - 21345678.123).abs() < 1e-3);
        assert!(e.sats[0].values[2].is_none(), "blank S1 stays absent");
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
}
