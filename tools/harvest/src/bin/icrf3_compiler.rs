use omegaflow::cdn::upload_release;
use std::io::Write;
use std::process::Command;

const SX_URL: &str =
    "https://datacenter.iers.org/products/reference-systems/celestial/icrf3/icrf3sx.txt";
const K_URL: &str =
    "https://datacenter.iers.org/products/reference-systems/celestial/icrf3/icrf3k.txt";

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("300")
        .arg("--compressed")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit(parts: &[(String, String)]) -> String {
    let mut out = String::from("{");
    for (k, (key, val)) in parts.iter().enumerate() {
        if k > 0 {
            out.push(',');
        }
        out.push_str(&json_str(key));
        out.push(':');
        out.push_str(val);
    }
    out.push('}');
    out
}

fn take_f64(toks: &[&str], pos: &mut usize) -> Option<f64> {
    let v = toks.get(*pos)?.parse::<f64>().ok()?;
    *pos += 1;
    Some(v)
}

fn alpha_token(t: &str) -> bool {
    !t.is_empty() && t.bytes().all(|b| b.is_ascii_alphabetic())
}

struct RowStats {
    rows_in: usize,
    rows_out: usize,
    defining: usize,
    skipped: usize,
}

fn build_row(line: &str, band: &str, stats: &mut RowStats) -> Option<String> {
    if !line.starts_with("ICRF J") {
        return None;
    }
    stats.rows_in += 1;
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks.len() < 17 {
        stats.skipped += 1;
        return None;
    }
    let name = format!("{} {}", toks[0], toks[1]);
    let ivs = toks[2].to_string();
    let mut pos = 3usize;
    let defining = if alpha_token(toks[pos]) {
        pos += 1;
        true
    } else {
        false
    };
    let ra_h = take_f64(&toks, &mut pos)?;
    let ra_m = take_f64(&toks, &mut pos)?;
    let ra_s = take_f64(&toks, &mut pos)?;
    let dec_neg = toks.get(pos)?.starts_with('-');
    let dec_d = take_f64(&toks, &mut pos)?.abs();
    let dec_m = take_f64(&toks, &mut pos)?;
    let dec_s = take_f64(&toks, &mut pos)?;
    let e_ra_s = take_f64(&toks, &mut pos)?;
    let e_dec_arcsec = take_f64(&toks, &mut pos)?;
    let corr = take_f64(&toks, &mut pos)?;
    let mean_mjd = take_f64(&toks, &mut pos)?;
    let first_mjd = take_f64(&toks, &mut pos)?;
    let last_mjd = take_f64(&toks, &mut pos)?;
    let nb_sess = take_f64(&toks, &mut pos)?;
    let nb_del = take_f64(&toks, &mut pos)?;
    let nb_rat = take_f64(&toks, &mut pos)?;

    if !(ra_h.is_finite()
        && ra_m.is_finite()
        && ra_s.is_finite()
        && dec_d.is_finite()
        && dec_m.is_finite()
        && dec_s.is_finite()
        && e_ra_s.is_finite()
        && e_dec_arcsec.is_finite()
        && corr.is_finite()
        && mean_mjd.is_finite()
        && first_mjd.is_finite()
        && last_mjd.is_finite()
        && nb_sess.is_finite()
        && nb_del.is_finite()
        && nb_rat.is_finite())
    {
        stats.skipped += 1;
        return None;
    }
    if !(0.0..24.0).contains(&ra_h) || !(0.0..60.0).contains(&ra_m) || !(0.0..60.0).contains(&ra_s)
    {
        stats.skipped += 1;
        return None;
    }
    if !(0.0..90.0).contains(&dec_d)
        || !(0.0..60.0).contains(&dec_m)
        || !(0.0..60.0).contains(&dec_s)
    {
        stats.skipped += 1;
        return None;
    }
    let ra_deg = 15.0 * (ra_h + ra_m / 60.0 + ra_s / 3600.0);
    let dec_mag = dec_d + dec_m / 60.0 + dec_s / 3600.0;
    let dec_deg = if dec_neg { -dec_mag } else { dec_mag };

    if defining {
        stats.defining += 1;
    }
    stats.rows_out += 1;
    let parts: Vec<(String, String)> = vec![
        ("name".into(), json_str(&name)),
        ("ivs".into(), json_str(&ivs)),
        ("band".into(), json_str(band)),
        (
            "defining".into(),
            if defining { "1".into() } else { "0".into() },
        ),
        ("ra_deg".into(), format!("{ra_deg}")),
        ("dec_deg".into(), format!("{dec_deg}")),
        ("e_ra_s".into(), format!("{e_ra_s}")),
        ("e_dec_arcsec".into(), format!("{e_dec_arcsec}")),
        ("corr".into(), format!("{corr}")),
        ("mean_mjd".into(), format!("{mean_mjd}")),
        ("first_mjd".into(), format!("{first_mjd}")),
        ("last_mjd".into(), format!("{last_mjd}")),
        ("nb_sess".into(), format!("{nb_sess}")),
        ("nb_del".into(), format!("{nb_del}")),
        ("nb_rat".into(), format!("{nb_rat}")),
    ];
    Some(emit(&parts))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out = String::from("phi/icrf3_harvest/icrf3.json");
    let mut ci_mode = false;
    let mut band = String::from("both");
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                if let Some(v) = args.get(i + 1) {
                    out = v.clone();
                }
                i += 1;
            }
            "--band" => {
                if let Some(v) = args.get(i + 1) {
                    band = v.clone();
                }
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }

    let wanted: Vec<(&str, &str)> = match band.as_str() {
        "sx" => vec![("sx", SX_URL)],
        "k" => vec![("k", K_URL)],
        _ => vec![("sx", SX_URL), ("k", K_URL)],
    };

    let mut stats = RowStats {
        rows_in: 0,
        rows_out: 0,
        defining: 0,
        skipped: 0,
    };
    let mut rows_out: Vec<String> = Vec::new();
    for (tag, url) in &wanted {
        let body = match curl(url) {
            Some(b) => b,
            None => {
                eprintln!("{tag}: fetch void — band stays absent (0 honored)");
                continue;
            }
        };
        for line in body.lines() {
            if let Some(r) = build_row(line, tag, &mut stats) {
                rows_out.push(r);
            }
        }
    }
    if rows_out.is_empty() {
        eprintln!(
            "icrf3: {} header/rows read, none carried away — the harvest stays unwritten (0 honored)",
            stats.rows_in
        );
        std::process::exit(1);
    }
    let mut buf = String::from("[");
    for (k, r) in rows_out.iter().enumerate() {
        if k > 0 {
            buf.push_str(",\n");
        }
        buf.push_str(r);
    }
    buf.push_str("]\n");
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                eprintln!("mkdir {} returned void: {}", parent.display(), err);
                std::process::exit(1);
            }
        }
    }
    match std::fs::File::create(&out) {
        Ok(mut f) => {
            if let Err(err) = f.write_all(buf.as_bytes()) {
                eprintln!("write {}: {}", out, err);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("write {} returned void: {}", out, err);
            std::process::exit(1);
        }
    }
    eprintln!(
        "icrf3_compiler: {}/{} rows → {} ({} B) — defining {}, skipped {}",
        stats.rows_out,
        stats.rows_in,
        out,
        buf.len(),
        stats.defining,
        stats.skipped,
    );
    if ci_mode && !upload_release("datacenter.iers.org", &out) {
        eprintln!("upload: {} did not reach the CDN", out);
        std::process::exit(1);
    }
}
