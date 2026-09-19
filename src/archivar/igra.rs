#[derive(Clone, Debug)]
pub struct IgraSounding {
    pub id: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub reltime: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub levels: Vec<IgraLevel>,
}

#[derive(Clone, Debug)]
pub struct IgraLevel {
    pub lvltyp1: Option<u32>,
    pub lvltyp2: Option<u32>,
    pub etime_min: Option<f64>,
    pub press_pa: Option<f64>,
    pub gph_m: Option<f64>,
    pub temp_c: Option<f64>,
    pub rh_pct: Option<f64>,
    pub dpdp_c: Option<f64>,
    pub wdir_deg: Option<f64>,
    pub wspd_ms: Option<f64>,
}

fn col(line: &str, start: usize, end: usize) -> Option<&str> {
    if start == 0 || end < start {
        return None;
    }
    line.get(start - 1..end)
}

fn int_col(line: &str, start: usize, end: usize) -> Option<i64> {
    let t = col(line, start, end)?.trim();
    if t.is_empty() {
        return None;
    }
    let v: i64 = t.parse().ok()?;
    if v == -9999 { None } else { Some(v) }
}

fn num_col(line: &str, start: usize, end: usize) -> Option<f64> {
    int_col(line, start, end).map(|v| v as f64)
}

fn scaled_col(line: &str, start: usize, end: usize, scale: f64) -> Option<f64> {
    int_col(line, start, end).map(|v| v as f64 / scale)
}

fn digit_col(line: &str, pos: usize) -> Option<u32> {
    col(line, pos, pos).and_then(|c| c.trim().parse::<u32>().ok())
}

fn parse_level(line: &str) -> IgraLevel {
    IgraLevel {
        lvltyp1: digit_col(line, 1),
        lvltyp2: digit_col(line, 2),
        etime_min: num_col(line, 4, 8),
        press_pa: num_col(line, 10, 15),
        gph_m: num_col(line, 17, 21),
        temp_c: scaled_col(line, 23, 27, 10.0),
        rh_pct: num_col(line, 29, 33),
        dpdp_c: scaled_col(line, 35, 39, 10.0),
        wdir_deg: num_col(line, 41, 45),
        wspd_ms: scaled_col(line, 47, 51, 10.0),
    }
}

pub fn parse_igra(body: &str) -> Vec<IgraSounding> {
    let mut soundings = Vec::new();
    let mut lines = body.lines();
    while let Some(line) = lines.next() {
        if !line.starts_with('#') {
            continue;
        }
        let id = match col(line, 2, 12) {
            Some(s) => s.trim().to_string(),
            None => continue,
        };
        let Some(year) = col(line, 14, 17).and_then(|c| c.trim().parse::<i32>().ok()) else {
            continue;
        };
        let Some(month) = col(line, 19, 20).and_then(|c| c.trim().parse::<u32>().ok()) else {
            continue;
        };
        let Some(day) = col(line, 22, 23).and_then(|c| c.trim().parse::<u32>().ok()) else {
            continue;
        };
        let Some(hour) = col(line, 25, 26).and_then(|c| c.trim().parse::<u32>().ok()) else {
            continue;
        };
        let reltime = col(line, 28, 31)
            .map(|c| c.trim().to_string())
            .filter(|s| !s.is_empty());
        let Some(numlev) = col(line, 33, 36).and_then(|c| c.trim().parse::<usize>().ok()) else {
            continue;
        };
        let Some(lat) = scaled_col(line, 56, 62, 10000.0) else {
            continue;
        };
        let Some(lon) = scaled_col(line, 64, 71, 10000.0) else {
            continue;
        };
        let mut levels = Vec::with_capacity(numlev);
        for _ in 0..numlev {
            let Some(dl) = lines.next() else {
                break;
            };
            if dl.starts_with('#') {
                break;
            }
            levels.push(parse_level(dl));
        }
        soundings.push(IgraSounding {
            id,
            year,
            month,
            day,
            hour,
            reltime,
            lat,
            lon,
            levels,
        });
    }
    soundings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_header(numlev: usize) -> String {
        format!(
            "#{:<11} {:<4} {:<2} {:<2} {:<2} {:<4} {:>4} {:<8} {:<8} {:>7} {:>8}",
            "USM00070026",
            "2026",
            "01",
            "01",
            "00",
            "2330",
            numlev,
            "ncdc-nws",
            "ncdc-gts",
            "712889",
            "-1567833"
        )
    }

    struct DataLineFields {
        lvltyp: &'static str,
        etime: &'static str,
        press: &'static str,
        pflag: &'static str,
        gph: &'static str,
        zflag: &'static str,
        temp: &'static str,
        tflag: &'static str,
        rh: &'static str,
        dpdp: &'static str,
        wdir: &'static str,
        wspd: &'static str,
    }

    fn data_line(d: &DataLineFields) -> String {
        format!(
            "{:<2} {:>5} {:>6}{}{:>5}{}{:>5}{}{:>5} {:>5} {:>5} {:>5}",
            d.lvltyp,
            d.etime,
            d.press,
            d.pflag,
            d.gph,
            d.zflag,
            d.temp,
            d.tflag,
            d.rh,
            d.dpdp,
            d.wdir,
            d.wspd
        )
    }

    fn measured_body() -> String {
        let mut body = String::new();
        body.push_str("# leading comment line\n");
        body.push_str(&sample_header(3));
        body.push('\n');
        body.push_str(&data_line(&DataLineFields {
            lvltyp: "21",
            etime: "0",
            press: "103574",
            pflag: "B",
            gph: "14",
            zflag: " ",
            temp: "-254",
            tflag: "B",
            rh: "820",
            dpdp: "22",
            wdir: "329",
            wspd: "62",
        }));
        body.push('\n');
        body.push_str(&data_line(&DataLineFields {
            lvltyp: "11",
            etime: "10",
            press: "-9999",
            pflag: " ",
            gph: "100",
            zflag: " ",
            temp: "-9999",
            tflag: " ",
            rh: "55",
            dpdp: "-9999",
            wdir: "180",
            wspd: "20",
        }));
        body.push('\n');
        body.push_str(&data_line(&DataLineFields {
            lvltyp: "31",
            etime: "20",
            press: "50000",
            pflag: " ",
            gph: "5600",
            zflag: " ",
            temp: "5",
            tflag: " ",
            rh: "10",
            dpdp: "-100",
            wdir: "270",
            wspd: "100",
        }));
        body.push('\n');
        body
    }

    #[test]
    fn header_fields_parse() {
        let s = parse_igra(&measured_body());
        assert_eq!(s.len(), 1);
        let h = &s[0];
        assert_eq!(h.id, "USM00070026");
        assert_eq!(h.year, 2026);
        assert_eq!(h.month, 1);
        assert_eq!(h.day, 1);
        assert_eq!(h.hour, 0);
        assert_eq!(h.reltime.as_deref(), Some("2330"));
        assert!((h.lat - 71.2889).abs() < 1e-9);
        assert!((h.lon - -156.7833).abs() < 1e-9);
    }

    #[test]
    fn first_level_parses_measured_values() {
        let s = parse_igra(&measured_body());
        let l = &s[0].levels[0];
        assert_eq!(l.lvltyp1, Some(2));
        assert_eq!(l.lvltyp2, Some(1));
        assert_eq!(l.etime_min, Some(0.0));
        assert_eq!(l.press_pa, Some(103574.0));
        assert_eq!(l.gph_m, Some(14.0));
        assert_eq!(l.temp_c, Some(-25.4));
        assert_eq!(l.wspd_ms, Some(6.2));
        assert_eq!(l.wdir_deg, Some(329.0));
    }

    #[test]
    fn missing_9999_becomes_none() {
        let s = parse_igra(&measured_body());
        let l = &s[0].levels[1];
        assert_eq!(l.press_pa, None);
        assert_eq!(l.temp_c, None);
        assert_eq!(l.dpdp_c, None);
        assert_eq!(l.gph_m, Some(100.0));
        assert_eq!(l.wspd_ms, Some(2.0));
    }

    #[test]
    fn second_sounding_parses_after_first() {
        let mut body = measured_body();
        body.push_str(&sample_header(2));
        body.push('\n');
        body.push_str(&data_line(&DataLineFields {
            lvltyp: "21",
            etime: "0",
            press: "101325",
            pflag: " ",
            gph: "5",
            zflag: " ",
            temp: "150",
            tflag: " ",
            rh: "70",
            dpdp: "50",
            wdir: "0",
            wspd: "30",
        }));
        body.push('\n');
        body.push_str(&data_line(&DataLineFields {
            lvltyp: "21",
            etime: "5",
            press: "90000",
            pflag: " ",
            gph: "900",
            zflag: " ",
            temp: "20",
            tflag: " ",
            rh: "60",
            dpdp: "-100",
            wdir: "90",
            wspd: "50",
        }));
        body.push('\n');
        let s = parse_igra(&body);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].levels.len(), 3);
        assert_eq!(s[1].levels.len(), 2);
        assert_eq!(s[1].id, "USM00070026");
        assert_eq!(s[1].levels[0].press_pa, Some(101325.0));
    }
}
