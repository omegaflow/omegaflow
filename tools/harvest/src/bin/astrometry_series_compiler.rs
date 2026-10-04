use omegaflow::archivar::astrometry_series::{
    AstroSample, AstroSeries, jd_utc_to_tdb, parse_bin, write_bin,
};
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::LeapSeconds;

const DEFAULT_NETLOC: &str = "vizier.cfa.harvard.edu";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Role {
    Time,
    Ra,
    Dec,
    ERa,
    EDec,
}

fn ucd_of_column(line: &str) -> Option<&str> {
    let start = line.find("ucd=")? + 4;
    let rest = &line[start..];
    let end = rest.find(']').unwrap_or(rest.len());
    let ucd = rest[..end].trim();
    if ucd.is_empty() { None } else { Some(ucd) }
}

fn role_of(name: &str, ucd: Option<&str>) -> Option<Role> {
    if let Some(u) = ucd {
        let l = u.to_ascii_lowercase();
        if l.contains("stat.error") {
            if l.contains("ra") {
                return Some(Role::ERa);
            }
            if l.contains("dec") {
                return Some(Role::EDec);
            }
        }
        if l.contains("pos.eq.ra") {
            return Some(Role::Ra);
        }
        if l.contains("pos.eq.dec") {
            return Some(Role::Dec);
        }
        if l.contains("instr.obsty") || l.contains("time") {
            return Some(Role::Time);
        }
    }
    let n: String = name
        .trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| *c != '_' && *c != ' ')
        .collect();
    if n.is_empty() {
        return None;
    }
    if n == "jd" || n == "hjd" || n == "mjd" || n == "epoch" || n == "date" || n == "jdutc" {
        return Some(Role::Time);
    }
    if n.starts_with('e') && n.contains("ra") {
        return Some(Role::ERa);
    }
    if n.starts_with('e') && n.contains("dec") {
        return Some(Role::EDec);
    }
    if n.contains("raj") || n.contains("raicrs") || n == "ra" || n.starts_with("ra") {
        return Some(Role::Ra);
    }
    if n.contains("dej") || n.contains("deicrs") || n == "dec" || n == "de" || n.starts_with("de") {
        return Some(Role::Dec);
    }
    None
}

fn parse_angle(raw: &str, hours: bool) -> Option<f64> {
    let s = raw.trim().replace('\u{2212}', "-");
    if s.is_empty() {
        return None;
    }
    let (value, sexagesimal) = if s.contains(':') {
        let parts: Vec<&str> = s.split(':').collect();
        if !(2..=3).contains(&parts.len()) {
            return None;
        }
        let d: f64 = parts[0].parse().ok()?;
        let m: f64 = parts[1].parse().ok()?;
        let sec: f64 = match parts.get(2) {
            Some(t) => t.parse().ok()?,
            None => 0.0,
        };
        (d.abs() + m / 60.0 + sec / 3600.0, true)
    } else {
        let toks: Vec<&str> = s.split_whitespace().collect();
        if toks.len() >= 2 {
            let d: f64 = toks[0].parse().ok()?;
            let m: f64 = toks[1].parse().ok()?;
            let sec: f64 = match toks.get(2) {
                Some(t) => t.parse().ok()?,
                None => 0.0,
            };
            (d.abs() + m / 60.0 + sec / 3600.0, true)
        } else {
            (s.parse().ok()?, false)
        }
    };
    if !value.is_finite() {
        return None;
    }
    let neg = sexagesimal && s.trim_start().starts_with('-');
    let signed = if neg { -value } else { value };
    Some(if hours { signed * 15.0 } else { signed })
}

fn positive_mas(raw: &str) -> Option<f64> {
    let v: f64 = raw.trim().parse().ok()?;
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

struct Parsed {
    name: String,
    samples: Vec<AstroSample>,
    rows_in: usize,
    skipped_void: usize,
    skipped_error: usize,
    skipped_riss: usize,
    ra_corrected: usize,
}

fn parse_asu_tsv(
    text: &str,
    lsk: &LeapSeconds,
    name_override: Option<&str>,
) -> Result<Parsed, String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut series_name: Option<String> = None;
    let mut col_ucds: Vec<String> = Vec::new();
    let mut header: Option<Vec<String>> = None;
    let mut units: Vec<String> = Vec::new();
    let mut data_start = 0usize;

    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i].trim_end_matches('\r');
        if line.starts_with('#') {
            let body = line.trim_start_matches('#');
            if let Some(rest) = body.strip_prefix("Name:") {
                let n = rest.trim();
                if !n.is_empty() {
                    series_name = Some(n.to_string());
                }
            }
            if body.starts_with("Column") {
                col_ucds.push(ucd_of_column(line).unwrap_or("").to_string());
            }
            i += 1;
            continue;
        }
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        header = Some(
            line.split('\t')
                .map(|c| c.trim().to_string())
                .collect::<Vec<String>>(),
        );
        let mut d = i + 1;
        if d < lines.len() {
            let l = lines[d].trim_end_matches('\r');
            if !l.trim().is_empty() && !l.trim_start().starts_with('-') {
                units = l.split('\t').map(|c| c.trim().to_string()).collect();
                d += 1;
            }
        }
        while d < lines.len() && lines[d].trim_end_matches('\r').trim().is_empty() {
            d += 1;
        }
        if d < lines.len()
            && lines[d]
                .trim_end_matches('\r')
                .trim_start()
                .starts_with('-')
        {
            d += 1;
        }
        data_start = d;
        break;
    }

    let header = header.ok_or_else(|| "the input carries no ASU-TSV header row".to_string())?;
    let ucd_ok = col_ucds.len() == header.len();

    let mut time_idx = None;
    let mut ra_idx = None;
    let mut dec_idx = None;
    let mut era_idx = None;
    let mut edec_idx = None;
    for (idx, h) in header.iter().enumerate() {
        let ucd = if ucd_ok {
            let u = col_ucds[idx].as_str();
            if u.is_empty() { None } else { Some(u) }
        } else {
            None
        };
        match role_of(h, ucd) {
            Some(Role::Time) if time_idx.is_none() => time_idx = Some(idx),
            Some(Role::Ra) if ra_idx.is_none() => ra_idx = Some(idx),
            Some(Role::Dec) if dec_idx.is_none() => dec_idx = Some(idx),
            Some(Role::ERa) if era_idx.is_none() => era_idx = Some(idx),
            Some(Role::EDec) if edec_idx.is_none() => edec_idx = Some(idx),
            _ => {}
        }
    }

    let time_idx = time_idx.ok_or_else(|| {
        format!(
            "the ASU-TSV header {:?} carries no time column (JD/pos;instr.obsty)",
            header
        )
    })?;
    let ra_idx = ra_idx.ok_or_else(|| {
        format!(
            "the ASU-TSV header {:?} carries no right-ascension column",
            header
        )
    })?;
    let dec_idx = dec_idx.ok_or_else(|| {
        format!(
            "the ASU-TSV header {:?} carries no declination column",
            header
        )
    })?;
    let era_idx = era_idx.ok_or_else(|| {
        format!(
            "the ASU-TSV header {:?} carries no e_RA error column (mas mandatory for AST1)",
            header
        )
    })?;
    let edec_idx = edec_idx.ok_or_else(|| {
        format!(
            "the ASU-TSV header {:?} carries no e_DE error column (mas mandatory for AST1)",
            header
        )
    })?;

    let ra_hours = units
        .get(ra_idx)
        .map(|u| !u.contains('d') || u.contains('h'))
        .unwrap_or(true);

    let Some(name) = name_override.map(|s| s.to_string()).or(series_name) else {
        return Err(
            "the ASU-TSV carries no series name — the table stays unwritten (0 honored)"
                .to_string(),
        );
    };

    let mut parsed = Parsed {
        name,
        samples: Vec::new(),
        rows_in: 0,
        skipped_void: 0,
        skipped_error: 0,
        skipped_riss: 0,
        ra_corrected: 0,
    };

    for line in &lines[data_start.min(lines.len())..] {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        parsed.rows_in += 1;
        let field = |idx: usize| f.get(idx).copied().unwrap_or("");
        if f.len() < header.len() {
            parsed.skipped_void += 1;
            continue;
        }
        let Some(jd) = field(time_idx)
            .trim()
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite())
        else {
            parsed.skipped_void += 1;
            continue;
        };
        let Some(ra) = parse_angle(field(ra_idx), ra_hours)
            .filter(|v| v.is_finite() && *v >= 0.0 && *v < 360.0)
        else {
            parsed.skipped_void += 1;
            continue;
        };
        let Some(dec) = parse_angle(field(dec_idx), false)
            .filter(|v| v.is_finite() && *v >= -90.0 && *v <= 90.0)
        else {
            parsed.skipped_void += 1;
            continue;
        };
        let (Some(e_ra_mas), Some(e_dec_mas)) =
            (positive_mas(field(era_idx)), positive_mas(field(edec_idx)))
        else {
            parsed.skipped_error += 1;
            continue;
        };
        let Some(tdb) = jd_utc_to_tdb(lsk, jd) else {
            parsed.skipped_void += 1;
            continue;
        };
        parsed.samples.push(AstroSample {
            tdb,
            ra_deg: ra,
            dec_deg: dec,
            e_ra_mas,
            e_dec_mas,
        });
    }

    Ok(parsed)
}

#[derive(Clone, Debug)]
enum Json {
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
    Other,
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(m) => m.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    fn as_array(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }
}

fn json_starts(b: &[char], i: usize, lit: &str) -> bool {
    lit.chars()
        .enumerate()
        .all(|(k, c)| b.get(i + k) == Some(&c))
}

fn json_ws(b: &[char], i: &mut usize) {
    while *i < b.len() && matches!(b[*i], ' ' | '\t' | '\n' | '\r') {
        *i += 1;
    }
}

fn json_hex4(b: &[char], i: &mut usize) -> Option<char> {
    let mut v = 0u32;
    for _ in 0..4 {
        let c = *b.get(*i)?;
        let d = c.to_digit(16)?;
        v = v * 16 + d;
        *i += 1;
    }
    char::from_u32(v)
}

fn json_string(b: &[char], i: &mut usize) -> Option<String> {
    if *b.get(*i)? != '"' {
        return None;
    }
    *i += 1;
    let mut s = String::new();
    loop {
        let c = *b.get(*i)?;
        *i += 1;
        match c {
            '"' => return Some(s),
            '\\' => {
                let e = *b.get(*i)?;
                *i += 1;
                match e {
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    '/' => s.push('/'),
                    'b' => s.push('\u{8}'),
                    'f' => s.push('\u{c}'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    'u' => s.push(json_hex4(b, i)?),
                    _ => return None,
                }
            }
            _ => s.push(c),
        }
    }
}

fn json_number(b: &[char], i: &mut usize) -> Option<f64> {
    let start = *i;
    while *i < b.len() && matches!(b[*i], '0'..='9' | '-' | '+' | '.' | 'e' | 'E') {
        *i += 1;
    }
    if *i == start {
        return None;
    }
    b[start..*i].iter().collect::<String>().parse().ok()
}

fn json_value(b: &[char], i: &mut usize) -> Option<Json> {
    json_ws(b, i);
    match *b.get(*i)? {
        '{' => {
            *i += 1;
            let mut m = Vec::new();
            json_ws(b, i);
            if *b.get(*i)? == '}' {
                *i += 1;
                return Some(Json::Obj(m));
            }
            loop {
                json_ws(b, i);
                let k = json_string(b, i)?;
                json_ws(b, i);
                if *b.get(*i)? != ':' {
                    return None;
                }
                *i += 1;
                let v = json_value(b, i)?;
                m.push((k, v));
                json_ws(b, i);
                match *b.get(*i)? {
                    ',' => *i += 1,
                    '}' => {
                        *i += 1;
                        return Some(Json::Obj(m));
                    }
                    _ => return None,
                }
            }
        }
        '[' => {
            *i += 1;
            let mut a = Vec::new();
            json_ws(b, i);
            if *b.get(*i)? == ']' {
                *i += 1;
                return Some(Json::Arr(a));
            }
            loop {
                a.push(json_value(b, i)?);
                json_ws(b, i);
                match *b.get(*i)? {
                    ',' => *i += 1,
                    ']' => {
                        *i += 1;
                        return Some(Json::Arr(a));
                    }
                    _ => return None,
                }
            }
        }
        '"' => Some(Json::Str(json_string(b, i)?)),
        't' => {
            if json_starts(b, *i, "true") {
                *i += 4;
                Some(Json::Other)
            } else {
                None
            }
        }
        'f' => {
            if json_starts(b, *i, "false") {
                *i += 5;
                Some(Json::Other)
            } else {
                None
            }
        }
        'n' => {
            if json_starts(b, *i, "null") {
                *i += 4;
                Some(Json::Other)
            } else {
                None
            }
        }
        _ => json_number(b, i).map(Json::Num),
    }
}

fn json_parse(text: &str) -> Option<Json> {
    let b: Vec<char> = text.chars().collect();
    let mut i = 0usize;
    let v = json_value(&b, &mut i)?;
    json_ws(&b, &mut i);
    if i != b.len() {
        return None;
    }
    Some(v)
}

fn collect_event_rows<'a>(node: &'a Json, out: &mut Vec<&'a Json>) {
    if let Some(rows) = node.get("rows").and_then(Json::as_array) {
        out.extend(rows.iter());
    } else if node.get("ra_position").is_some() || node.get("position_date").is_some() {
        out.push(node);
    }
}

fn ra_hour_token_is_single_digit(raw: &str) -> bool {
    raw.trim()
        .split([' ', ':'])
        .find(|t| !t.is_empty())
        .map(|t| t.chars().count() == 1)
        .unwrap_or(false)
}

fn rewrite_page(url: &str, page: usize) -> String {
    if let Some(i) = url.find("page=") {
        let after = i + 5;
        let end = url[after..]
            .find('&')
            .map(|j| after + j)
            .unwrap_or(url.len());
        format!("{}{}{}", &url[..after], page, &url[end..])
    } else if url.contains('?') {
        format!("{url}&page={page}")
    } else {
        format!("{url}?page={page}")
    }
}

fn source_host(url: &str) -> Option<String> {
    let rest = url.split("://").nth(1).unwrap_or(url);
    let host = rest.split(['/', '?']).next().unwrap_or("");
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

fn lesia_object_ra(html: &str) -> Option<f64> {
    let i = html.find("Object astrometric position")?;
    let rest = &html[i..];
    let a = rest.find("<td>")? + 4;
    let b = rest[a..].find("</td>")? + a;
    let cleaned: String = rest[a..b]
        .chars()
        .map(|c| match c {
            '\n' | '\t' | '\r' => ' ',
            _ => c,
        })
        .collect();
    let toks: Vec<&str> = cleaned.split_whitespace().collect();
    if toks.len() < 3 {
        return None;
    }
    let ra_raw = format!("{} {} {}", toks[0], toks[1], toks[2]);
    parse_angle(&ra_raw, true).filter(|v| v.is_finite())
}

fn lesia_reference_ra(link: &str) -> Option<f64> {
    if !link.contains("lesia.obspm.fr") {
        return None;
    }
    let bytes = fetch_raw_bytes(link)?;
    let html = String::from_utf8_lossy(&bytes);
    lesia_object_ra(&html)
}

fn parse_events_json(
    bodies: &[String],
    lsk: &LeapSeconds,
    name: &str,
    lesia_ra_of: &dyn Fn(&str) -> Option<f64>,
) -> Result<Parsed, String> {
    let mut parsed = Parsed {
        name: name.to_string(),
        samples: Vec::new(),
        rows_in: 0,
        skipped_void: 0,
        skipped_error: 0,
        skipped_riss: 0,
        ra_corrected: 0,
    };

    for body in bodies {
        let root = json_parse(body).ok_or_else(|| {
            "the events body does not parse as JSON — the series stays unwritten".to_string()
        })?;
        let mut rows: Vec<&Json> = Vec::new();
        collect_event_rows(&root, &mut rows);
        for row in rows {
            parsed.rows_in += 1;
            let field = |k: &str| {
                row.get(k)
                    .and_then(Json::as_str)
                    .map(|s| s.trim().to_string())
            };
            let Some(jd) = field("position_date")
                .and_then(|t| t.parse::<f64>().ok())
                .filter(|v| v.is_finite())
            else {
                parsed.skipped_void += 1;
                continue;
            };
            let (Some(ra_raw), Some(dec_raw)) = (field("ra_position"), field("dec_position"))
            else {
                parsed.skipped_void += 1;
                continue;
            };
            let Some(mut ra) =
                parse_angle(&ra_raw, true).filter(|v| v.is_finite() && *v >= 0.0 && *v < 360.0)
            else {
                parsed.skipped_void += 1;
                continue;
            };
            let Some(dec) =
                parse_angle(&dec_raw, false).filter(|v| v.is_finite() && *v >= -90.0 && *v <= 90.0)
            else {
                parsed.skipped_void += 1;
                continue;
            };

            if ra_hour_token_is_single_digit(&ra_raw) {
                let link = match field("link") {
                    Some(v) => v,
                    None => String::new(),
                };
                match lesia_ra_of(&link) {
                    Some(ref_ra)
                        if (ref_ra - ra).abs() >= 100.0 && (ref_ra - ra).abs() <= 200.0 =>
                    {
                        ra = ref_ra;
                        parsed.ra_corrected += 1;
                    }
                    Some(ref_ra) if (ref_ra - ra).abs() < 0.01 => {}
                    _ => {
                        parsed.skipped_riss += 1;
                        continue;
                    }
                }
            }

            let (Some(e_ra_mas), Some(e_dec_mas)) = (
                field("ra_position_error").and_then(|t| positive_mas(&t)),
                field("dec_position_error").and_then(|t| positive_mas(&t)),
            ) else {
                parsed.skipped_error += 1;
                continue;
            };

            if e_dec_mas / e_ra_mas > 100.0 {
                parsed.skipped_riss += 1;
                continue;
            }

            let Some(tdb) = jd_utc_to_tdb(lsk, jd) else {
                parsed.skipped_void += 1;
                continue;
            };
            parsed.samples.push(AstroSample {
                tdb,
                ra_deg: ra,
                dec_deg: dec,
                e_ra_mas,
                e_dec_mas,
            });
        }
    }

    Ok(parsed)
}

const FIXTURE: &str = "\
#   VizieR Astronomical Server vizier.cfa.harvard.edu\n\
#Name: J/A+A/582/A8/ariel_j\n\
#Column\tJD\t(F16.8)\tUTC instant of the satellite position (JD)\t[ucd=pos;instr.obsty]\n\
#Column\tRAJ2000\t(A12)\tRight ascension (J2000) of satellite at date\t[ucd=pos.eq.ra;meta.main]\n\
#Column\te_RAJ2000\t(I3)\tError in RA (J2000) at date (1)\t[ucd=stat.error;pos.eq.ra]\n\
#Column\tDEJ2000\t(A12)\tDeclination (J2000) of satellite at date\t[ucd=pos.eq.dec;meta.main]\n\
#Column\te_DEJ2000\t(I3)\tError in DE (J2000) at date\t[ucd=stat.error;pos.eq.dec]\n\
#Column\tSat\t(A4)\tSatellites that contributed\t[ucd=meta.code]\n\
JD\tRAJ2000\te_RAJ2000\tDEJ2000\te_DEJ2000\tSat\n\
d\t\"h:m:s\"\tmas\t\"d:m:s\"\tmas\t\n\
----------------\t------------\t---\t------------\t---\t----\n\
2448782.70138981\t19 14 55.313\t55\t-22 45 10.84\t48\t\n\
2448782.75270463\t19 14 54.841\t0\t-22 45 13.48\t55\t\n\
2448783.00000000\t\t52\t-22 45 13.60\t53\t\n";

fn selftest() {
    let Some(lsk) = embedded_lsk() else {
        eprintln!("selftest: embedded naif0012 parses void — the TDB axis stays unread");
        std::process::exit(1);
    };
    let parsed = match parse_asu_tsv(FIXTURE, &lsk, None) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("selftest parse void: {e}");
            std::process::exit(1);
        }
    };
    if parsed.name != "J/A+A/582/A8/ariel_j" {
        eprintln!(
            "selftest: series name {:?} is not the #Name table",
            parsed.name
        );
        std::process::exit(1);
    }
    if parsed.samples.len() != 1 || parsed.rows_in != 3 {
        eprintln!(
            "selftest: {} of {} rows measured, 1 expected (zero error and void RA stay absent)",
            parsed.samples.len(),
            parsed.rows_in
        );
        std::process::exit(1);
    }
    if parsed.skipped_error != 1 || parsed.skipped_void != 1 {
        eprintln!(
            "selftest: skipped_error {} / skipped_void {} (1/1 expected)",
            parsed.skipped_error, parsed.skipped_void
        );
        std::process::exit(1);
    }
    let s = &parsed.samples[0];
    let ra_expect = (19.0 + 14.0 / 60.0 + 55.313 / 3600.0) * 15.0;
    let dec_expect = -(22.0 + 45.0 / 60.0 + 10.84 / 3600.0);
    if (s.ra_deg - ra_expect).abs() > 1e-9 || (s.dec_deg - dec_expect).abs() > 1e-9 {
        eprintln!(
            "selftest: RA/Dec {} / {} is not the sexagesimal fold {} / {}",
            s.ra_deg, s.dec_deg, ra_expect, dec_expect
        );
        std::process::exit(1);
    }
    let Some(tdb_expect) = jd_utc_to_tdb(&lsk, 2448782.70138981) else {
        eprintln!("selftest: the JD does not fold onto the TDB axis");
        std::process::exit(1);
    };
    if s.tdb != tdb_expect || s.e_ra_mas != 55.0 || s.e_dec_mas != 48.0 {
        eprintln!(
            "selftest: tdb/e_ra/e_dec {} / {} / {} is not the measured sample",
            s.tdb, s.e_ra_mas, s.e_dec_mas
        );
        std::process::exit(1);
    }
    let series = AstroSeries {
        name: parsed.name,
        samples: parsed.samples,
    };
    let Some(bytes) = write_bin(std::slice::from_ref(&series)) else {
        eprintln!("selftest: write_bin refused the measured series");
        std::process::exit(1);
    };
    match parse_bin(&bytes) {
        Some(back) if back.len() == 1 && back[0].samples.len() == 1 => {
            let b = &back[0].samples[0];
            let a = &series.samples[0];
            if b.tdb != a.tdb || b.ra_deg != a.ra_deg || b.dec_deg != a.dec_deg {
                eprintln!("selftest: AST1 roundtrip changed a sample");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("selftest: AST1 roundtrip reads void");
            std::process::exit(1);
        }
    }
    let js = match parse_events_json(
        &[JSON_FIXTURE.to_string()],
        &lsk,
        "sosb_lucky_star",
        &|link| {
            if link.contains("173384") {
                Some(169.0234575817)
            } else {
                None
            }
        },
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("selftest JSON parse void: {e}");
            std::process::exit(1);
        }
    };
    if js.samples.len() != 2 || js.rows_in != 4 || js.skipped_riss != 2 || js.ra_corrected != 1 {
        eprintln!(
            "selftest: JSON {} samples of {} rows, {} riss, {} corrected (2/4/2/1 expected)",
            js.samples.len(),
            js.rows_in,
            js.skipped_riss,
            js.ra_corrected
        );
        std::process::exit(1);
    }
    let j0 = &js.samples[0];
    let ra0 = (18.0 + 21.0 / 60.0 + 42.8677703 / 3600.0) * 15.0;
    let dec0 = -(15.0 + 12.0 / 60.0 + 45.829691 / 3600.0);
    if (j0.ra_deg - ra0).abs() > 1e-9 || (j0.dec_deg - dec0).abs() > 1e-9 {
        eprintln!(
            "selftest JSON: RA/Dec {} / {} is not the sexagesimal fold {} / {}",
            j0.ra_deg, j0.dec_deg, ra0, dec0
        );
        std::process::exit(1);
    }
    if j0.e_ra_mas != 0.249 || j0.e_dec_mas != 0.230 {
        eprintln!(
            "selftest JSON: e_ra/e_dec {} / {} is not the measured mas 0.249 / 0.230 (unit is mas)",
            j0.e_ra_mas, j0.e_dec_mas
        );
        std::process::exit(1);
    }
    let j1 = &js.samples[1];
    if (j1.ra_deg - 169.0234575817).abs() > 1e-9 || j1.e_dec_mas != 0.18 {
        eprintln!(
            "selftest JSON: corrected RA {} / e_dec {} is not the lesia-repaired 169.0234575817 / 0.18",
            j1.ra_deg, j1.e_dec_mas
        );
        std::process::exit(1);
    }
    let single = r#"{"id":5,"position_date":"2459800.7736523","ra_position":"18 21 42.8677703","ra_position_error":"0.249","dec_position":"-15 12 45.829691","dec_position_error":"0.230","link":""}"#;
    match parse_events_json(&[single.to_string()], &lsk, "one", &|_| None) {
        Ok(p) if p.rows_in == 1 && p.samples.len() == 1 => {}
        _ => {
            eprintln!("selftest JSON: a single event object does not fold to one sample");
            std::process::exit(1);
        }
    }
    eprintln!(
        "astrometry_series_compiler: selftest passes (ASU-TSV fold + JSON events fold + AST1 roundtrip)"
    );
}

const JSON_FIXTURE: &str = r#"{"total":4,"rows":[
 {"id":1,"position_date":"2459800.7736523","ra_position":"18 21 42.8677703","ra_position_error":"0.249","dec_position":"-15 12 45.829691","dec_position_error":"0.230","link":""},
 {"id":2,"position_date":"2461174.648913426","ra_position":"1 16 05.6286153","ra_position_error":"0.14","dec_position":"-6 23 13.031160","dec_position_error":"0.18","link":"https://lesia.obspm.fr/lucky-star/occ.php?p=173384"},
 {"id":3,"position_date":"2459800.7736523","ra_position":"18 21 42.8677703","ra_position_error":"0.249","dec_position":"-15 12 45.829691","dec_position_error":"230","link":""},
 {"id":4,"position_date":"2461174.648913426","ra_position":"1 16 05.6286153","ra_position_error":"0.14","dec_position":"-6 23 13.031160","dec_position_error":"0.18","link":"https://example.org/not-lesia"}
]}"#;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let Some(lsk) = embedded_lsk() else {
        eprintln!("embedded naif0012 parses void — the TDB axis stays unread");
        std::process::exit(1);
    };

    let source_url = arg_value(&args, "--source");

    let text: String = if let Some(path) = arg_value(&args, "--in") {
        match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        }
    } else if let Some(url) = source_url.as_deref() {
        let Some(bytes) = fetch_raw_bytes(url) else {
            eprintln!("{url}: fetch void — the series stays unwritten");
            std::process::exit(1);
        };
        String::from_utf8_lossy(&bytes).into_owned()
    } else {
        eprintln!(
            "usage: astrometry_series_compiler (--in <path> | --source <url>) [--out <path>] [--name <name>] [--pages <n>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    };

    let name_override = arg_value(&args, "--name");
    let trimmed = text.trim_start();
    let is_json = trimmed.starts_with('{') || trimmed.starts_with('[');
    let parsed = if is_json {
        let mut bodies = vec![text.clone()];
        if let Some(url) = source_url.as_deref() {
            let pages = if let Some(v) = arg_value(&args, "--pages") {
                match v.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => 1,
                }
            } else {
                match json_parse(&text) {
                    Some(root) => {
                        let total = match root.get("total") {
                            Some(Json::Num(n)) if *n >= 0.0 => Some(*n as usize),
                            Some(_) => None,
                            None => None,
                        };
                        let first = match root.get("rows").and_then(Json::as_array) {
                            Some(a) => Some(a.len()),
                            None => None,
                        };
                        match (total, first) {
                            (Some(t), Some(f)) if t > 0 && f > 0 => ((t + f - 1) / f).min(64),
                            _ => 1,
                        }
                    }
                    None => 1,
                }
            };
            for page in 2..=pages {
                let page_url = rewrite_page(url, page);
                match fetch_raw_bytes(&page_url) {
                    Some(bytes) => bodies.push(String::from_utf8_lossy(&bytes).into_owned()),
                    None => {
                        eprintln!("{page_url}: page fetch void — the whole series stays unwritten");
                        std::process::exit(1);
                    }
                }
            }
        }
        let default_name = match source_url.as_deref().and_then(source_host) {
            Some(h) => h,
            None => "events_json".to_string(),
        };
        let series_name = name_override.clone().unwrap_or(default_name);
        parse_events_json(&bodies, &lsk, &series_name, &lesia_reference_ra)
    } else {
        parse_asu_tsv(&text, &lsk, name_override.as_deref())
    };
    let parsed = match parsed {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    if parsed.samples.is_empty() {
        eprintln!(
            "{}: {} rows carry no measured JD/RA/Dec/error — the series stays unwritten (0 honored)",
            parsed.name, parsed.rows_in
        );
        std::process::exit(1);
    }

    let series = AstroSeries {
        name: parsed.name.clone(),
        samples: parsed.samples,
    };
    let Some(bytes) = write_bin(std::slice::from_ref(&series)) else {
        eprintln!("{}: AST1 encode void", series.name);
        std::process::exit(1);
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("data/{DEFAULT_NETLOC}/{}.ast1", sanitize(&series.name)),
    };
    if let Some(parent) = std::path::Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if let Err(e) = std::fs::write(&out, &bytes) {
        eprintln!("write {out} returned void: {e}");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(back) if back.len() == 1 && back[0].samples.len() == series.samples.len() => {
            eprintln!(
                "astrometry_series_compiler: {} — {} samples of {} rows → {} ({} B), {} void, {} absent error, {} riss ({} RA corrected), roundtrip parses",
                series.name,
                series.samples.len(),
                parsed.rows_in,
                out,
                bytes.len(),
                parsed.skipped_void,
                parsed.skipped_error,
                parsed.skipped_riss,
                parsed.ra_corrected,
            );
        }
        _ => {
            eprintln!("{out}: roundtrip parse void — the AST1 stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(DEFAULT_NETLOC, &out) {
        std::process::exit(1);
    }
}
