use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const HLSP_PAGE: &str = "https://archive.stsci.edu/hlsp/ps1-strm";
const README_URL: &str =
    "https://archive.stsci.edu/hlsps/ps1-strm/hlsp_ps1-strm_ps1_gpc1_all_multi_v1_readme.txt";
const STRIP_BASE: &str = "https://archive.stsci.edu/hlsps/ps1-strm";
const DOI: &str = "10.17909/t9-rnk7-gr88";
const PS1_DEC_MIN_DEG: f64 = -30.0;
const DEFAULT_ANCHORS: &str = "/tmp/opencode/gaia/missing88.csv";
const DEFAULT_OUT: &str = "/tmp/opencode/panstarrs-quasar-88.tsv";
const DEFAULT_RADIUS_ARCSEC: f64 = 2.0;
const CONE_URL: &str = "https://gsss.stsci.edu/webservices/vo/ConeSearch.aspx?CAT=PS1V3OBJECTS";

struct Http {
    status: u32,
    body: String,
}

struct Head {
    status: u32,
    length: Option<u64>,
}

struct Anchor {
    iers: String,
    ra_deg: f64,
    dec_deg: f64,
}

struct Ps1Object {
    obj_name: String,
    obj_id: String,
    n_detections: Option<u32>,
    ra_deg: Option<f64>,
    dec_deg: Option<f64>,
    ra_err_arcsec: Option<f64>,
    dec_err_arcsec: Option<f64>,
    g_psf: Option<f64>,
    r_psf: Option<f64>,
    i_psf: Option<f64>,
    z_psf: Option<f64>,
    y_psf: Option<f64>,
}

struct Match {
    anchor: Anchor,
    in_footprint: bool,
    object: Option<Ps1Object>,
    sep_mas: Option<f64>,
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn curl_head(url: &str) -> Option<Head> {
    let out = Command::new("curl")
        .arg("-sSI")
        .arg("-m")
        .arg("45")
        .arg("-A")
        .arg("omegaflow-panstarrs-probe/1.0")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut status = None;
    let mut length = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("HTTP/") {
            let code = rest.split_whitespace().nth(1);
            if let Some(c) = code {
                status = c.parse::<u32>().ok();
            }
        }
        let lower = line.to_ascii_lowercase();
        if let Some(rest) = lower.strip_prefix("content-length:") {
            length = rest.trim().parse::<u64>().ok();
        }
    }
    Some(Head {
        status: status?,
        length,
    })
}

fn curl_get(url: &str) -> Option<Http> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-L")
        .arg("-A")
        .arg("omegaflow-panstarrs-probe/1.0")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let raw = out.stdout;
    let idx = raw.iter().rposition(|&b| b == b'\n')?;
    let status = String::from_utf8_lossy(&raw[idx + 1..])
        .trim()
        .parse::<u32>()
        .ok()?;
    let body = String::from_utf8_lossy(&raw[..idx]).to_string();
    Some(Http { status, body })
}

fn read_anchors(path: &str) -> Vec<Anchor> {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{path}: {e}");
            return Vec::new();
        }
    };
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 3 {
            continue;
        }
        let ra = f[1].trim().parse::<f64>();
        let dec = f[2].trim().parse::<f64>();
        if let (Ok(ra), Ok(dec)) = (ra, dec) {
            if ra.is_finite() && dec.is_finite() {
                out.push(Anchor {
                    iers: f[0].trim().to_string(),
                    ra_deg: ra,
                    dec_deg: dec,
                });
            }
        }
    }
    out
}

fn parse_readme_columns(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_table = false;
    for line in text.lines() {
        if line.starts_with("Column Name") {
            in_table = true;
            continue;
        }
        if !in_table {
            continue;
        }
        if line.starts_with("-----") || line.trim().is_empty() {
            if !out.is_empty() {
                break;
            }
            continue;
        }
        let f: Vec<&str> = line.split('\t').filter(|x| !x.trim().is_empty()).collect();
        if f.len() >= 3 {
            let name = f[0].trim();
            if name
                .chars()
                .next()
                .map(|c| c.is_ascii_alphabetic() || c == '_')
                .unwrap_or(false)
            {
                out.push(name.to_string());
            }
        }
    }
    out
}

fn field_indexes(fields: &[String]) -> std::collections::BTreeMap<String, usize> {
    fields
        .iter()
        .enumerate()
        .map(|(i, f)| (f.clone(), i))
        .collect()
}

fn xml_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn parse_cone(body: &str) -> Vec<Ps1Object> {
    let mut fields = Vec::new();
    for chunk in body.split("<FIELD ").skip(1) {
        let head = chunk.split('>').next().unwrap_or("");
        if let Some(rest) = head.split("name=\"").nth(1) {
            if let Some(name) = rest.split('"').next() {
                fields.push(name.to_string());
            }
        }
    }
    let idx = field_indexes(&fields);
    let get = |name: &str| idx.get(name).copied();
    let mut out = Vec::new();
    for row in body.split("<TR>").skip(1) {
        let row = row.split("</TR>").next().unwrap_or("");
        let cells: Vec<String> = row
            .split("<TD>")
            .skip(1)
            .map(|c| xml_unescape(c.split("</TD>").next().unwrap_or("").trim()))
            .collect();
        if cells.is_empty() {
            continue;
        }
        let text_at = |name: &str| -> Option<String> {
            get(name).and_then(|i| cells.get(i)).cloned()
        };
        let num_at = |name: &str| -> Option<f64> {
            text_at(name).and_then(|v| v.parse::<f64>().ok())
        };
        let mut o = Ps1Object {
            obj_name: match text_at("objName") {
                Some(value) => value,
                None => String::new(),
            },
            obj_id: match text_at("objID") {
                Some(value) => value,
                None => String::new(),
            },
            n_detections: text_at("nDetections").and_then(|v| v.parse::<u32>().ok()),
            ra_deg: num_at("RAmean"),
            dec_deg: num_at("DECmean"),
            ra_err_arcsec: num_at("RAmeanErr"),
            dec_err_arcsec: num_at("DECmeanErr"),
            g_psf: num_at("gMeanPSFMag"),
            r_psf: num_at("rMeanPSFMag"),
            i_psf: num_at("iMeanPSFMag"),
            z_psf: num_at("zMeanPSFMag"),
            y_psf: num_at("yMeanPSFMag"),
        };
        if o.ra_deg.map(|v| v.is_finite()).unwrap_or(false)
            && o.dec_deg.map(|v| v.is_finite()).unwrap_or(false)
        {
            if let Some(id) = text_at("objID") {
                o.obj_id = id;
            }
            out.push(o);
        }
    }
    out
}

fn separation_mas(a_ra: f64, a_dec: f64, b_ra: f64, b_dec: f64) -> f64 {
    let d_dec = (b_dec - a_dec).to_radians();
    let d_ra = (b_ra - a_ra).to_radians() * ((a_dec + b_dec) / 2.0).to_radians().cos();
    let sep_rad = (d_ra * d_ra + d_dec * d_dec).sqrt();
    sep_rad.to_degrees() * 3_600_000.0
}

fn cone_search(anchor: &Anchor, radius_arcsec: f64) -> Vec<Ps1Object> {
    let sr_deg = radius_arcsec / 3600.0;
    let url = format!(
        "{CONE_URL}&RA={:.9}&DEC={:.9}&SR={:.9}",
        anchor.ra_deg, anchor.dec_deg, sr_deg
    );
    for attempt in 0..3 {
        if let Some(http) = curl_get(&url) {
            if http.status == 200 {
                return parse_cone(&http.body);
            }
            eprintln!("cone {}: HTTP {}", anchor.iers, http.status);
        }
        sleep(Duration::from_millis(500 * (attempt + 1)));
    }
    Vec::new()
}

fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = v.len();
    if n % 2 == 1 {
        Some(v[n / 2])
    } else {
        Some((v[n / 2 - 1] + v[n / 2]) / 2.0)
    }
}

fn rms(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let sum: f64 = values.iter().map(|v| v * v).sum();
    Some((sum / values.len() as f64).sqrt())
}

fn fmt_opt(v: Option<f64>, digits: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.digits$}"),
        _ => "absent".to_string(),
    }
}

fn fmt_mag(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() && x > 0.0 => format!("{x:.3}"),
        _ => "absent".to_string(),
    }
}

struct StripSample {
    rows: usize,
    fields: usize,
    classes: BTreeMap<String, usize>,
}

fn strip_sample(url: &str) -> Option<StripSample> {
    let gz_path = "/tmp/opencode/.panstarrs_strm_sample.gz";
    let ok = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-r")
        .arg("0-999999")
        .arg("--max-filesize")
        .arg("2000000")
        .arg("-o")
        .arg(gz_path)
        .arg(url)
        .status()
        .ok()?
        .success();
    if !ok {
        return None;
    }
    let out = Command::new("gzip").arg("-dc").arg(gz_path).output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let mut rows = 0usize;
    let mut fields = 0usize;
    let mut classes = BTreeMap::new();
    for line in text.lines().take(2000) {
        if line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        if fields == 0 {
            fields = f.len();
        }
        if f.len() >= 7 {
            rows += 1;
            *classes.entry(f[6].trim().to_string()).or_insert(0) += 1;
        }
    }
    let _ = fs::remove_file(gz_path);
    Some(StripSample {
        rows,
        fields,
        classes,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let anchors_path = arg_after(&args, "--anchors").unwrap_or(DEFAULT_ANCHORS);
    let out_path = arg_after(&args, "--out").unwrap_or(DEFAULT_OUT);
    let radius_arcsec = arg_after(&args, "--radius-arcsec")
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| v.is_finite() && *v > 0.0)
        .unwrap_or(DEFAULT_RADIUS_ARCSEC);
    let metadata_only = has_flag(&args, "--metadata-only");
    let limit = arg_after(&args, "--limit")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0);

    println!("# Pan-STARRS1 Quasar-Katalog: PS1-STRM (Beck et al. 2020)");
    println!("doi\t{DOI}");
    println!("hlsp_page\t{HLSP_PAGE}");
    println!("readme\t{README_URL}");

    if let Some(h) = curl_head(HLSP_PAGE) {
        println!("hlsp_http\t{}", h.status);
    } else {
        println!("hlsp_http\tabsent");
    }

    let (readme_status, readme_body) = match curl_get(README_URL) {
        Some(h) => (h.status, h.body),
        None => (0, String::new()),
    };
    println!("readme_http\t{readme_status}");
    println!("readme_bytes\t{}", readme_body.len());
    let columns = parse_readme_columns(&readme_body);
    println!("columns\t{}", columns.len());
    println!("column_names\t{}", columns.join(","));

    let source_count = readme_body
        .lines()
        .find(|l| l.contains("2,902,054,648"))
        .map(|_| 2_902_054_648u64);
    println!(
        "sources\t{}",
        match source_count {
            Some(n) => n.to_string(),
            None => "absent".to_string(),
        }
    );
    println!("footprint_dec_min_deg\t{PS1_DEC_MIN_DEG}");

    for strip in [
        "hlsp_ps1-strm_ps1_gpc1_m15-m12_multi_v1_cat.csv.gz",
        "hlsp_ps1-strm_ps1_gpc1_p01-p05_multi_v1_cat.csv.gz",
        "hlsp_ps1-strm_ps1_gpc1_p25-p28_multi_v1_cat.csv.gz",
        "hlsp_ps1-strm_ps1_gpc1_p38-p41_multi_v1_cat.csv.gz",
    ] {
        let url = format!("{STRIP_BASE}/{strip}");
        match curl_head(&url) {
            Some(h) => println!(
                "strip\t{strip}\thttp={}\tbytes={}",
                h.status,
                match h.length {
                    Some(n) => n.to_string(),
                    None => "absent".to_string(),
                }
            ),
            None => println!("strip\t{strip}\thttp=absent\tbytes=absent"),
        }
        sleep(Duration::from_millis(120));
    }

    if !has_flag(&args, "--no-strip-sample") {
        let sample_url = format!(
            "{STRIP_BASE}/hlsp_ps1-strm_ps1_gpc1_p01-p05_multi_v1_cat.csv.gz"
        );
        match strip_sample(&sample_url) {
            Some(s) => {
                println!("strip_sample_rows\t{}", s.rows);
                println!("strip_sample_fields\t{}", s.fields);
                let classes: Vec<String> = s
                    .classes
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect();
                println!("strip_sample_classes\t{}", classes.join(","));
            }
            None => {
                println!("strip_sample_rows\tabsent");
                println!("strip_sample_fields\tabsent");
                println!("strip_sample_classes\tabsent");
            }
        }
    }

    let anchors = read_anchors(anchors_path);
    let in_footprint = anchors
        .iter()
        .filter(|a| a.dec_deg >= PS1_DEC_MIN_DEG)
        .count();
    println!("anchors\t{}", anchors.len());
    println!("in_footprint\t{in_footprint}");
    println!("outside_footprint\t{}", anchors.len() - in_footprint);

    if metadata_only {
        println!("measurement\tmetadata-only");
        return;
    }

    let selected: Vec<&Anchor> = match limit {
        Some(n) => anchors.iter().take(n).collect(),
        None => anchors.iter().collect(),
    };

    let mut matches = Vec::with_capacity(selected.len());
    for (i, a) in selected.iter().enumerate() {
        let objects = cone_search(a, radius_arcsec);
        let mut best: Option<(Ps1Object, f64)> = None;
        for o in objects {
            let (ora, odec) = match (o.ra_deg, o.dec_deg) {
                (Some(ra), Some(dec)) => (ra, dec),
                _ => continue,
            };
            let sep = separation_mas(a.ra_deg, a.dec_deg, ora, odec);
            if !sep.is_finite() {
                continue;
            }
            if sep <= radius_arcsec * 1000.0 {
                match &best {
                    Some((_, s)) if *s <= sep => {}
                    _ => best = Some((o, sep)),
                }
            }
        }
        let (object, sep_mas) = match best {
            Some((o, s)) => (Some(o), Some(s)),
            None => (None, None),
        };
        matches.push(Match {
            anchor: Anchor {
                iers: a.iers.clone(),
                ra_deg: a.ra_deg,
                dec_deg: a.dec_deg,
            },
            in_footprint: a.dec_deg >= PS1_DEC_MIN_DEG,
            object,
            sep_mas,
        });
        if i + 1 < selected.len() {
            sleep(Duration::from_millis(120));
        }
    }

    let matched: Vec<&Match> = matches.iter().filter(|m| m.object.is_some()).collect();
    let in_fp: Vec<&Match> = matches.iter().filter(|m| m.in_footprint).collect();
    let matched_in_fp = in_fp.iter().filter(|m| m.object.is_some()).count();
    let seps: Vec<f64> = matched
        .iter()
        .filter_map(|m| m.sep_mas)
        .collect();
    let ra_errs: Vec<f64> = matched
        .iter()
        .filter_map(|m| m.object.as_ref())
        .filter_map(|o| o.ra_err_arcsec)
        .map(|v| v * 1000.0)
        .filter(|v| v.is_finite() && *v >= 0.0)
        .collect();
    let dec_errs: Vec<f64> = matched
        .iter()
        .filter_map(|m| m.object.as_ref())
        .filter_map(|o| o.dec_err_arcsec)
        .map(|v| v * 1000.0)
        .filter(|v| v.is_finite() && *v >= 0.0)
        .collect();

    println!("radius_arcsec\t{radius_arcsec:.3}");
    println!("matched\t{}", matched.len());
    println!("unmatched\t{}", matches.len() - matched.len());
    println!("matched_in_footprint\t{matched_in_fp}");
    println!("unmatched_in_footprint\t{}", in_fp.len() - matched_in_fp);
    println!("offset_median_mas\t{}", fmt_opt(median(&seps), 3));
    println!("offset_rms_mas\t{}", fmt_opt(rms(&seps), 3));
    println!("offset_p95_mas\t{}", fmt_opt(percentile(&seps, 0.95), 3));
    println!("offset_max_mas\t{}", fmt_opt(seps.iter().copied().reduce(f64::max), 3));
    let core_cut_mas = 500.0;
    let core: Vec<f64> = seps.iter().copied().filter(|s| *s <= core_cut_mas).collect();
    let suspect = seps.len() - core.len();
    println!("core_cut_mas\t{core_cut_mas:.1}");
    println!("core_matched\t{}", core.len());
    println!("tail_matched\t{suspect}");
    println!("core_median_mas\t{}", fmt_opt(median(&core), 3));
    println!("core_rms_mas\t{}", fmt_opt(rms(&core), 3));
    println!("core_p95_mas\t{}", fmt_opt(percentile(&core, 0.95), 3));
    for cut in [100.0, 200.0, 500.0, 1000.0] {
        let n = seps.iter().filter(|s| **s <= cut).count();
        println!("matched_le_{cut:.0}mas\t{n}");
    }
    println!(
        "match_rate_2arcsec\t{}",
        if in_fp.is_empty() {
            "absent".to_string()
        } else {
            format!("{:.4}", matched_in_fp as f64 / in_fp.len() as f64)
        }
    );
    println!(
        "core_rate_2arcsec\t{}",
        if in_fp.is_empty() {
            "absent".to_string()
        } else {
            format!("{:.4}", core.len() as f64 / in_fp.len() as f64)
        }
    );
    println!("ra_err_median_mas\t{}", fmt_opt(median(&ra_errs), 3));
    println!("dec_err_median_mas\t{}", fmt_opt(median(&dec_errs), 3));
    println!("ra_err_rms_mas\t{}", fmt_opt(rms(&ra_errs), 3));
    println!("dec_err_rms_mas\t{}", fmt_opt(rms(&dec_errs), 3));
    println!("qso_class\tabsent-ps1-strm-casjobs-required");

    let mut tsv = String::new();
    tsv.push_str("iers\tra_deg\tdec_deg\tin_footprint\tmatched\tassociation\tsep_mas\tps1_objname\tps1_objid\tps1_ra_deg\tps1_dec_deg\tps1_ra_err_mas\tps1_dec_err_mas\tps1_ndet\tg_psf\tr_psf\ti_psf\tz_psf\ty_psf\n");
    for m in &matches {
        let (obj_name, obj_id, ora, odec, rae, dece, ndet, g, r, i, z, y) = match &m.object {
            Some(o) => (
                o.obj_name.clone(),
                o.obj_id.clone(),
                fmt_opt(o.ra_deg, 9),
                fmt_opt(o.dec_deg, 9),
                fmt_opt(o.ra_err_arcsec.map(|v| v * 1000.0), 3),
                fmt_opt(o.dec_err_arcsec.map(|v| v * 1000.0), 3),
                match o.n_detections {
                    Some(value) => value.to_string(),
                    None => "absent".to_string(),
                },
                fmt_mag(o.g_psf),
                fmt_mag(o.r_psf),
                fmt_mag(o.i_psf),
                fmt_mag(o.z_psf),
                fmt_mag(o.y_psf),
            ),
            None => (
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
                "absent".to_string(),
            ),
        };
        tsv.push_str(&format!(
            "{}\t{:.9}\t{:.9}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            m.anchor.iers,
            m.anchor.ra_deg,
            m.anchor.dec_deg,
            if m.in_footprint { "yes" } else { "no" },
            if m.object.is_some() { "yes" } else { "no" },
            match m.sep_mas {
                Some(s) if s <= 500.0 => "core",
                Some(_) => "tail",
                None => "absent",
            },
            fmt_opt(m.sep_mas, 3),
            obj_name,
            obj_id,
            ora,
            odec,
            rae,
            dece,
            ndet,
            g,
            r,
            i,
            z,
            y
        ));
    }
    match fs::write(out_path, &tsv) {
        Ok(()) => println!("out\t{out_path}"),
        Err(e) => println!("out\tabsent:{e}"),
    }

    println!();
    println!("## Naechste fuenf Anker (groesster Versatz)");
    let mut worst: Vec<&Match> = matched.clone();
    worst.sort_by(|a, b| {
        let sa = a.sep_mas.unwrap_or(-1.0);
        let sb = b.sep_mas.unwrap_or(-1.0);
        sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
    });
    println!("| iers | sep mas | ps1 objid | ra err mas | dec err mas |");
    println!("|---|---:|---|---:|---:|");
    for m in worst.iter().take(5) {
        let o = match &m.object {
            Some(o) => o,
            None => continue,
        };
        println!(
            "| {} | {} | {} | {} | {} |",
            m.anchor.iers,
            fmt_opt(m.sep_mas, 3),
            o.obj_id,
            fmt_opt(o.ra_err_arcsec.map(|v| v * 1000.0), 3),
            fmt_opt(o.dec_err_arcsec.map(|v| v * 1000.0), 3)
        );
    }

    println!();
    println!("## Anker ohne PS1-Gegenstueck innerhalb {radius_arcsec:.1}\"");
    let mut missing: Vec<&Match> = matches.iter().filter(|m| m.object.is_none()).collect();
    missing.sort_by(|a, b| a.anchor.iers.cmp(&b.anchor.iers));
    for m in missing {
        println!(
            "{} {} {} footprint={}",
            m.anchor.iers,
            m.anchor.ra_deg,
            m.anchor.dec_deg,
            if m.in_footprint { "yes" } else { "no" }
        );
    }
}

fn percentile(values: &[f64], q: f64) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let pos = q.clamp(0.0, 1.0) * (v.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    if lo == hi {
        Some(v[lo])
    } else {
        let t = pos - lo as f64;
        Some(v[lo] * (1.0 - t) + v[hi] * t)
    }
}
