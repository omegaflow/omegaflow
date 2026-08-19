// MPC cometels.json.gz → flat cmap catalog (ra/dec/dist_au at catalog epoch).
// Runtime channel: cmap + dist_scale (AU → m). e >= 1 skipped (Kepler domain, 0 honored).
// upload via --ci-mode (tag ssd.jpl.nasa.gov, asset cometels_flat.json).

use omegaflow::cdn::upload_asset;
use omegaflow::inflate::gunzip;
use omegaflow::kepler::{elements_to_icrs_state, AU_M, GM_SUN_M3_S2};
use std::collections::HashMap;
use std::io::Write;

const TAU: f64 = 2.0 * std::f64::consts::PI;

#[derive(Debug)]
enum Json {
    Null,
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(HashMap<String, Json>),
}

struct Jp<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Jp<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && (self.b[self.i] as char).is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn val(&mut self) -> Option<Json> {
        self.ws();
        match self.b.get(self.i).copied() {
            Some(b'n') => {
                if self.b[self.i..].starts_with(b"null") {
                    self.i += 4;
                    Some(Json::Null)
                } else {
                    None
                }
            }
            Some(b't') => {
                if self.b[self.i..].starts_with(b"true") {
                    self.i += 4;
                    Some(Json::Num(1.0))
                } else {
                    None
                }
            }
            Some(b'f') => {
                if self.b[self.i..].starts_with(b"false") {
                    self.i += 5;
                    Some(Json::Num(0.0))
                } else {
                    None
                }
            }
            Some(b'"') => self.string().map(Json::Str),
            Some(b'[') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Some(Json::Arr(v));
                }
                loop {
                    v.push(self.val()?);
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => {
                            self.i += 1;
                        }
                        Some(b']') => {
                            self.i += 1;
                            return Some(Json::Arr(v));
                        }
                        _ => return None,
                    }
                }
            }
            Some(b'{') => {
                self.i += 1;
                let mut m = HashMap::new();
                self.ws();
                if self.b.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Some(Json::Obj(m));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.ws();
                    if self.b.get(self.i) != Some(&b':') {
                        return None;
                    }
                    self.i += 1;
                    let v = self.val()?;
                    m.insert(k, v);
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => {
                            self.i += 1;
                        }
                        Some(b'}') => {
                            self.i += 1;
                            return Some(Json::Obj(m));
                        }
                        _ => return None,
                    }
                }
            }
            Some(_) => {
                let rest = &self.b[self.i..];
                let end = rest
                    .iter()
                    .position(|&c| c == b',' || c == b']' || c == b'}' || c.is_ascii_whitespace())
                    .unwrap_or(rest.len());
                let s = std::str::from_utf8(&rest[..end]).ok()?;
                if end == 0 {
                    return None;
                }
                self.i += end;
                s.trim().parse::<f64>().ok().map(Json::Num)
            }
            None => None,
        }
    }
    fn string(&mut self) -> Option<String> {
        if self.b.get(self.i) != Some(&b'"') {
            return None;
        }
        self.i += 1;
        let mut s = String::new();
        loop {
            let c = self.b.get(self.i).copied()?;
            self.i += 1;
            match c {
                b'"' => return Some(s),
                b'\\' => {
                    let e = self.b.get(self.i).copied()?;
                    self.i += 1;
                    match e {
                        b'n' => s.push('\n'),
                        b't' => s.push('\t'),
                        b'r' => s.push('\r'),
                        b'"' => s.push('"'),
                        b'\\' => s.push('\\'),
                        b'u' => {
                            let h = std::str::from_utf8(&self.b[self.i..self.i + 4]).ok()?;
                            self.i += 4;
                            s.push(char::from_u32(u32::from_str_radix(h, 16).ok()?)?);
                        }
                        _ => return None,
                    }
                }
                _ => s.push(c as char),
            }
        }
    }
}

fn parse_json(s: &str) -> Option<Json> {
    let mut p = Jp {
        b: s.as_bytes(),
        i: 0,
    };
    p.val()
}

fn get_num(o: &HashMap<String, Json>, key: &str) -> Option<f64> {
    match o.get(key) {
        Some(Json::Num(v)) => Some(*v),
        Some(Json::Str(t)) => t.trim().parse().ok(),
        _ => None,
    }
}

fn get_str(o: &HashMap<String, Json>, key: &str) -> Option<String> {
    match o.get(key) {
        Some(Json::Str(t)) => Some(t.clone()),
        Some(Json::Num(v)) => Some(format!("{}", v)),
        _ => None,
    }
}

struct CometRow {
    name: String,
    ra_deg: f64,
    dec_deg: f64,
    dist_au: f64,
    h: Option<f64>,
}

fn days_from_civil(year: i64, month: i64, day: i64) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146097 + doe - 719468)
}

fn jd_from(y: f64, m: f64, d_frac: f64) -> Option<f64> {
    let (ym, mm) = (y as i64, m as i64);
    if mm < 1 || mm > 12 || d_frac < 1.0 {
        return None;
    }
    let days = days_from_civil(ym, mm, 1)? as f64;
    Some(days + (d_frac - 1.0) + 2440587.5)
}

fn evaluate(obj: &HashMap<String, Json>) -> Option<CometRow> {
    let e = get_num(obj, "e")?;
    let q = get_num(obj, "Perihelion_dist")?;
    let incl = get_num(obj, "i")?;
    let node = get_num(obj, "Node")?;
    let peri = get_num(obj, "Peri")?;
    let tp_jd = jd_from(
        get_num(obj, "Year_of_perihelion")?,
        get_num(obj, "Month_of_perihelion")?,
        get_num(obj, "Day_of_perihelion")?,
    )?;
    let epoch_jd = jd_from(
        get_num(obj, "Epoch_year")?,
        get_num(obj, "Epoch_month")?,
        get_num(obj, "Epoch_day")?,
    )?;
    if e >= 1.0 || q <= 0.0 {
        return None;
    }
    let a = q / (1.0 - e);
    let a_m = a * AU_M;
    let n = (GM_SUN_M3_S2 / a_m.powi(3)).sqrt();
    let ma_deg = ((n * (epoch_jd - tp_jd) * 86400.0).rem_euclid(TAU)).to_degrees();
    let (p, _) = elements_to_icrs_state(a, e, incl, node, peri, ma_deg, epoch_jd, epoch_jd)?;
    let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    if !r.is_finite() || r <= 0.0 {
        return None;
    }
    let ra_deg = p[1].atan2(p[0]).to_degrees().rem_euclid(360.0);
    let dec_deg = (p[2] / r).asin().to_degrees();
    Some(CometRow {
        name: get_str(obj, "Designation_and_name")
            .or_else(|| get_str(obj, "Provisional_packed_desig"))
            .unwrap_or_else(|| "unnamed".to_string()),
        ra_deg,
        dec_deg,
        dist_au: r / AU_M,
        h: get_num(obj, "H"),
    })
}

fn json_string(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            _ => o.push(c),
        }
    }
    o.push('"');
    o
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut probe: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let input = match input {
        Some(p) => p,
        None => {
            eprintln!("--input absent");
            std::process::exit(1);
        }
    };
    let gz = match std::fs::read(&input) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("read {} returned void", input);
            std::process::exit(1);
        }
    };
    let text = match gunzip(&gz) {
        Some(t) => t,
        None => {
            eprintln!("gunzip {} returned void", input);
            std::process::exit(1);
        }
    };
    let text = match std::str::from_utf8(&text) {
        Ok(s) => s.to_string(),
        Err(_) => {
            eprintln!("utf8 returned void");
            std::process::exit(1);
        }
    };
    let json = match parse_json(&text) {
        Some(j) => j,
        None => {
            eprintln!("json returned void");
            std::process::exit(1);
        }
    };
    let arr = match json {
        Json::Arr(a) => a,
        _ => {
            eprintln!("root is not an array");
            std::process::exit(1);
        }
    };
    if let Some(Json::Obj(first)) = arr.first() {
        let keys: Vec<&str> = first.keys().map(|k| k.as_str()).collect();
        eprintln!("cometels: {} records, first keys: {:?}", arr.len(), keys);
    }
    if let Some(name) = &probe {
        for v in &arr {
            if let Json::Obj(obj) = v {
                let n = get_str(obj, "Designation_and_name")
                    .or_else(|| get_str(obj, "Provisional_packed_desig"))
                    .unwrap_or_default();
                if n.contains(name) {
                    let e = get_num(obj, "e");
                    let q = get_num(obj, "Perihelion_dist");
                    match evaluate(obj) {
                        Some(r) => eprintln!(
                            "probe {}: e={:?} q={:?} au → ra={:.6} dec={:.6} dist={:.6} au H={:?}",
                            n, e, q, r.ra_deg, r.dec_deg, r.dist_au, r.h
                        ),
                        None => {
                            eprintln!("probe {}: e={:?} outside Kepler domain (0 honored)", n, e)
                        }
                    }
                    return;
                }
            }
        }
        eprintln!("probe: {} not present", name);
        return;
    }
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    let mut rows: Vec<CometRow> = Vec::new();
    let mut skipped = 0usize;
    for v in &arr {
        if let Json::Obj(obj) = v {
            match evaluate(obj) {
                Some(r) => rows.push(r),
                None => skipped += 1,
            }
        } else {
            skipped += 1;
        }
    }
    let mut buf = String::from("[");
    for (k, r) in rows.iter().enumerate() {
        if k > 0 {
            buf.push(',');
        }
        buf.push_str(&format!(
            "{{\"name\":{},\"ra\":{},\"dec\":{},\"dist_au\":{},\"H\":{}}}",
            json_string(&r.name),
            r.ra_deg,
            r.dec_deg,
            r.dist_au,
            match r.h {
                Some(h) => h.to_string(),
                None => "null".to_string(),
            }
        ));
    }
    buf.push_str("]\n");
    if let Ok(mut f) = std::fs::File::create(&out_path) {
        let _ = f.write_all(buf.as_bytes());
    } else {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    eprintln!(
        "cometels: {} records written, {} skipped (e>=1 or void fields), {} B → {}",
        rows.len(),
        skipped,
        buf.len(),
        out_path
    );
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
