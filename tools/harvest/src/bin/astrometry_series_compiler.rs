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
    eprintln!("astrometry_series_compiler: selftest passes (ASU-TSV fold + AST1 roundtrip)");
}

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

    let text: String = if let Some(path) = arg_value(&args, "--in") {
        match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        }
    } else if let Some(url) = arg_value(&args, "--source") {
        let Some(bytes) = fetch_raw_bytes(&url) else {
            eprintln!("{url}: fetch void — the series stays unwritten");
            std::process::exit(1);
        };
        String::from_utf8_lossy(&bytes).into_owned()
    } else {
        eprintln!(
            "usage: astrometry_series_compiler (--in <path> | --source <url>) [--out <path>] [--name <name>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    };

    let name_override = arg_value(&args, "--name");
    let parsed = match parse_asu_tsv(&text, &lsk, name_override.as_deref()) {
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
                "astrometry_series_compiler: {} — {} samples of {} rows → {} ({} B), {} void, {} absent error, roundtrip parses",
                series.name,
                series.samples.len(),
                parsed.rows_in,
                out,
                bytes.len(),
                parsed.skipped_void,
                parsed.skipped_error,
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
