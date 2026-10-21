use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use omegaflow::sha256::sha256_hex;

const ICRF3_SX_URL: &str = "https://hpiers.obspm.fr/icrs-pc/icrf/icrf3sx.txt";
const QCAL1_PAGE_URL: &str = "http://astrogeo.org/qcal1";
const QCAL1_T1_URL: &str = "http://astrogeo.org/qcal1/datafile1.txt";
const QCAL1_T2_URL: &str = "http://astrogeo.org/qcal1/datafile2.txt";
const TAP_SYNC: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const QCAL1_T3: &str = "J/AJ/144/150/table3";
const QCAL1_T4: &str = "J/AJ/144/150/table4";
const LANYI_T3: &str = "J/AJ/139/1695/table3";
const QCAL1_README: &str = "https://cdsarc.cds.unistra.fr/ftp/cats/J/AJ/144/150/ReadMe";
const LANYI_README: &str = "https://cdsarc.cds.unistra.fr/ftp/cats/J/AJ/139/1695/ReadMe";
const QCRF_RECORD: &str = "https://zenodo.org/api/records/10834955";
const KCRF_RECORD: &str = "https://zenodo.org/api/records/10893171";
const MATCH_ARCSEC: f64 = 1.5;
const MAS_PER_ARCSEC: f64 = 1000.0;

struct Fetch {
    url: String,
    status: Option<u16>,
    bytes: usize,
    sha256: String,
    content_type: Option<String>,
    last_modified: Option<String>,
    origin: &'static str,
    body: Vec<u8>,
}

fn header_value(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name.trim().eq_ignore_ascii_case(key) {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

fn curl(url: &str, body_path: &Path, hdr_path: &Path) -> Option<u16> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-m")
        .arg("180")
        .arg("-A")
        .arg("omegaflow-qband-catalog-probe/1.0")
        .arg("-D")
        .arg(hdr_path)
        .arg("-o")
        .arg(body_path)
        .arg("-w")
        .arg("%{http_code}")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!("curl: exit {:?} for {}", out.status.code(), url);
        return None;
    }
    String::from_utf8_lossy(&out.stdout).trim().parse().ok()
}

fn fetch(url: &str, cache: &Path, offline: bool) -> Option<Fetch> {
    if let Some(parent) = cache.parent() {
        if fs::create_dir_all(parent).is_err() {
            eprintln!("cache: cannot create {}", parent.display());
        }
    }
    let hdr_path = cache.with_extension("hdr");
    let mut status = None;
    let mut origin = "cache";
    if !offline {
        origin = "http";
        match curl(url, cache, &hdr_path) {
            Some(code) => status = Some(code),
            None => {
                eprintln!("fetch: curl failed for {url}");
                origin = "cache";
            }
        }
    }
    let body = match fs::read(cache) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("fetch: unreadable {} ({e})", cache.display());
            return None;
        }
    };
    let hdr = match fs::read_to_string(&hdr_path) {
        Ok(text) => text,
        Err(_) => String::new(),
    };
    Some(Fetch {
        url: url.to_string(),
        status,
        bytes: body.len(),
        sha256: sha256_hex(&body),
        content_type: header_value(&hdr, "content-type"),
        last_modified: header_value(&hdr, "last-modified"),
        origin,
        body,
    })
}

fn tap_csv_url(table: &str) -> String {
    let query = format!("SELECT * FROM \"{table}\"");
    let mut enc = String::new();
    for b in query.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                enc.push(b as char)
            }
            _ => enc.push_str(&format!("%{b:02X}")),
        }
    }
    format!("{TAP_SYNC}?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY={enc}")
}

#[derive(Clone)]
struct Icrf {
    name: String,
    iers: String,
    defining: bool,
    ra_deg: f64,
    dec_deg: f64,
}

fn parse_icrf(text: &str) -> Vec<Icrf> {
    let mut out = Vec::new();
    for line in text.lines() {
        if !line.starts_with("ICRF J") {
            continue;
        }
        let t: Vec<&str> = line.split_whitespace().collect();
        let name = match t.get(1) {
            Some(v) => v.to_string(),
            None => continue,
        };
        let iers = match t.get(2) {
            Some(v) => v.to_string(),
            None => continue,
        };
        let mut i = 3;
        let defining = t.get(i) == Some(&"D");
        if defining {
            i += 1;
        }
        let (Some(h), Some(m), Some(s)) = (
            t.get(i).and_then(|v| v.parse::<f64>().ok()),
            t.get(i + 1).and_then(|v| v.parse::<f64>().ok()),
            t.get(i + 2).and_then(|v| v.parse::<f64>().ok()),
        ) else {
            continue;
        };
        i += 3;
        let dec_token = match t.get(i) {
            Some(v) => *v,
            None => continue,
        };
        let dec_sign = if dec_token.starts_with('-') { -1.0 } else { 1.0 };
        let (Some(dd), Some(dm), Some(ds)) = (
            dec_token.parse::<f64>().ok(),
            t.get(i + 1).and_then(|v| v.parse::<f64>().ok()),
            t.get(i + 2).and_then(|v| v.parse::<f64>().ok()),
        ) else {
            continue;
        };
        let ra_deg = (h + m / 60.0 + s / 3600.0) * 15.0;
        let dec_deg = (dd.abs() + dm / 60.0 + ds / 3600.0) * dec_sign;
        if !(ra_deg.is_finite() && dec_deg.is_finite()) {
            continue;
        }
        out.push(Icrf {
            name,
            iers,
            defining,
            ra_deg,
            dec_deg,
        });
    }
    out
}

fn csv_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    for c in line.chars() {
        match c {
            '"' => quoted = !quoted,
            ',' if !quoted => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

struct Table {
    header: Vec<String>,
    rows: Vec<BTreeMap<String, String>>,
}

impl Table {
    fn column(&self, name: &str, row: &BTreeMap<String, String>) -> Option<f64> {
        if !self.header.iter().any(|h| h == name) {
            return None;
        }
        row.get(name)?.trim().parse().ok()
    }
}

fn parse_csv(text: &str) -> Table {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = match lines.next() {
        Some(h) => csv_fields(h),
        None => Vec::new(),
    };
    let mut rows = Vec::new();
    for line in lines {
        let cells = csv_fields(line);
        let mut row = BTreeMap::new();
        for (i, key) in header.iter().enumerate() {
            if let Some(v) = cells.get(i) {
                row.insert(key.clone(), v.trim().to_string());
            }
        }
        rows.push(row);
    }
    Table { header, rows }
}

fn sep_arcsec(ra1: f64, de1: f64, ra2: f64, de2: f64) -> f64 {
    let dec_mid = ((de1 + de2) * 0.5).to_radians();
    let dra = (ra1 - ra2).to_radians() * dec_mid.cos();
    let dde = (de1 - de2).to_radians();
    dra.hypot(dde) * 180.0 / std::f64::consts::PI * 3600.0
}

struct Nearest {
    sep_arcsec: f64,
    index: usize,
}

fn nearest(cat_ra: f64, cat_dec: f64, icrf: &[Icrf]) -> Option<Nearest> {
    let mut best: Option<Nearest> = None;
    for (index, s) in icrf.iter().enumerate() {
        let sep_arcsec = sep_arcsec(cat_ra, cat_dec, s.ra_deg, s.dec_deg);
        let better = match &best {
            Some(b) => sep_arcsec < b.sep_arcsec,
            None => true,
        };
        if better {
            best = Some(Nearest { sep_arcsec, index });
        }
    }
    best
}

fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    Some(if n % 2 == 1 {
        values[n / 2]
    } else {
        0.5 * (values[n / 2 - 1] + values[n / 2])
    })
}

fn pct(part: usize, n: usize) -> String {
    if n == 0 {
        "n/a".to_string()
    } else {
        format!("{:.1}%", 100.0 * part as f64 / n as f64)
    }
}

fn source_rows(t: &Table) -> Vec<(String, f64, f64)> {
    let mut out = Vec::new();
    for row in &t.rows {
        let name = match row.get("JName").cloned() {
            Some(name) => name,
            None => String::new(),
        };
        let (Some(ra), Some(dec)) = (t.column("RAJ2000", row), t.column("DEJ2000", row)) else {
            continue;
        };
        if ra.is_finite() && dec.is_finite() {
            out.push((name, ra, dec));
        }
    }
    out
}

struct Coverage {
    label: String,
    n: usize,
    matched: usize,
    matched_d: usize,
    median_sep_mas: Option<f64>,
    max_sep_mas: Option<f64>,
    above5_mas: usize,
}

fn coverage(label: &str, srcs: &[(String, f64, f64)], icrf: &[Icrf]) -> Coverage {
    let mut matched = 0usize;
    let mut matched_d = 0usize;
    let mut above5 = 0usize;
    let mut seps = Vec::new();
    for (_, ra, dec) in srcs {
        let Some(hit) = nearest(*ra, *dec, icrf) else {
            continue;
        };
        if hit.sep_arcsec > MATCH_ARCSEC {
            continue;
        }
        matched += 1;
        if icrf[hit.index].defining {
            matched_d += 1;
        }
        if hit.sep_arcsec * MAS_PER_ARCSEC > 5.0 {
            above5 += 1;
        }
        seps.push(hit.sep_arcsec * MAS_PER_ARCSEC);
    }
    let max = seps.iter().copied().fold(0.0f64, f64::max);
    Coverage {
        label: label.to_string(),
        n: srcs.len(),
        matched,
        matched_d,
        median_sep_mas: median(&mut seps),
        max_sep_mas: if seps.is_empty() { None } else { Some(max) },
        above5_mas: above5,
    }
}

fn print_channel(label: &str, f: &Fetch) {
    println!(
        "{label}: {}\n  origin {} | HTTP {} | bytes {} | sha256 {}\n  content-type {} | last-modified {}",
        f.url,
        f.origin,
        match f.status {
            Some(c) => c.to_string(),
            None => "?".to_string(),
        },
        f.bytes,
        f.sha256,
        f.content_type.as_deref().unwrap_or("?"),
        f.last_modified.as_deref().unwrap_or("?")
    );
}

fn print_table(label: &str, t: &Table, declared: Option<usize>) {
    let check = match declared {
        Some(d) if d == t.rows.len() => "ok",
        Some(_) => "MISMATCH",
        None => "no declared count",
    };
    println!(
        "{label}: rows {} | declared {} | match {check}",
        t.rows.len(),
        match declared {
            Some(d) => d.to_string(),
            None => "?".to_string(),
        }
    );
    println!("  columns: {}", t.header.join(", "));
}

fn print_range(label: &str, t: &Table, column: &str) {
    let mut values = Vec::new();
    for row in &t.rows {
        if let Some(v) = t.column(column, row) {
            if v.is_finite() {
                values.push(v);
            }
        }
    }
    if values.is_empty() {
        println!("{label} {column}: absent");
        return;
    }
    println!(
        "{label} {column}: min {:.3} | max {:.3} | n {}",
        values.iter().copied().fold(f64::INFINITY, f64::min),
        values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        values.len()
    );
}

fn declared_records(text: &str, file: &str) -> Option<usize> {
    for line in text.lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.first() == Some(&file) {
            if let Some(count) = t.get(2) {
                if let Ok(v) = count.parse::<usize>() {
                    return Some(v);
                }
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = match args.iter().position(|a| a == "--sources") {
        Some(i) => match args.get(i + 1) {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("sources"),
        },
        None => PathBuf::from("sources"),
    };
    let offline = args.iter().any(|a| a == "--offline");
    let out_path = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from);

    println!("== Q-band quasar catalog probe (43 GHz)");
    println!("cache root: {} | offline {}", root.display(), offline);

    println!("\n== Channels (measured)");
    let icrf_fetch = fetch(&ICRF3_SX_URL, &root.join("hpiers.obspm.fr/icrf3sx.txt"), offline);
    let page_fetch = fetch(&QCAL1_PAGE_URL, &root.join("astrogeo.org/qcal1.html"), offline);
    let t1_fetch = fetch(&QCAL1_T1_URL, &root.join("astrogeo.org/qcal1_datafile1.txt"), offline);
    let t2_fetch = fetch(&QCAL1_T2_URL, &root.join("astrogeo.org/qcal1_datafile2.txt"), offline);
    let q3_fetch = fetch(
        &tap_csv_url(QCAL1_T3),
        &root.join("tapvizier.cds.unistra.fr/qcal1_table3.csv"),
        offline,
    );
    let q4_fetch = fetch(
        &tap_csv_url(QCAL1_T4),
        &root.join("tapvizier.cds.unistra.fr/qcal1_table4.csv"),
        offline,
    );
    let ly_fetch = fetch(
        &tap_csv_url(LANYI_T3),
        &root.join("tapvizier.cds.unistra.fr/lanyi_table3.csv"),
        offline,
    );
    let qcal_readme_fetch = fetch(
        QCAL1_README,
        &root.join("cdsarc.cds.unistra.fr/qcal1_ReadMe.txt"),
        offline,
    );
    let lanyi_readme_fetch = fetch(
        LANYI_README,
        &root.join("cdsarc.cds.unistra.fr/lanyi_ReadMe.txt"),
        offline,
    );
    let qcrf_fetch = fetch(
        QCRF_RECORD,
        &root.join("zenodo.org/qcrf_2021_record.json"),
        offline,
    );
    let kcrf_fetch = fetch(
        KCRF_RECORD,
        &root.join("zenodo.org/kcrf_2024_record.json"),
        offline,
    );

    for (label, f) in [
        ("ICRF3 S/X", &icrf_fetch),
        ("QCAL-1 page", &page_fetch),
        ("QCAL-1 table3 raw", &t1_fetch),
        ("QCAL-1 table4 raw", &t2_fetch),
        ("VizieR QCAL-1 table3", &q3_fetch),
        ("VizieR QCAL-1 table4", &q4_fetch),
        ("VizieR Lanyi table3", &ly_fetch),
        ("QCAL-1 ReadMe", &qcal_readme_fetch),
        ("Lanyi ReadMe", &lanyi_readme_fetch),
        ("QCRF-2021 record", &qcrf_fetch),
        ("KCRF-2024 record", &kcrf_fetch),
    ] {
        match f {
            Some(f) => print_channel(label, f),
            None => println!("{label}: absent/unreadable (no body, no cache)"),
        }
    }

    let icrf_body = match &icrf_fetch {
        Some(f) => String::from_utf8_lossy(&f.body).to_string(),
        None => {
            println!("\nverdict: absent — ICRF3 S/X unreadable, coverage not measurable");
            return;
        }
    };
    let icrf = parse_icrf(&icrf_body);
    let declaring = icrf.iter().filter(|s| s.defining).count();
    println!(
        "\n== ICRF3 S/X reference: parsed {} | D {} | declared header {}",
        icrf.len(),
        declaring,
        match icrf_body.lines().next() {
            Some(l) => l.to_string(),
            None => "?".to_string(),
        }
    );

    let mut coverage_rows: Vec<Coverage> = Vec::new();
    let mut catalog_tables: Vec<(String, Table, usize)> = Vec::new();

    let qcal_readme = qcal_readme_fetch
        .as_ref()
        .map(|f| String::from_utf8_lossy(&f.body).to_string());
    let lanyi_readme = lanyi_readme_fetch
        .as_ref()
        .map(|f| String::from_utf8_lossy(&f.body).to_string());

    let declared_q3 = qcal_readme
        .as_deref()
        .and_then(|t| declared_records(t, "table3.dat"));
    let declared_q4 = qcal_readme
        .as_deref()
        .and_then(|t| declared_records(t, "table4.dat"));
    let declared_ly = lanyi_readme
        .as_deref()
        .and_then(|t| declared_records(t, "table3.dat"));

    if let Some(f) = &q3_fetch {
        let t = parse_csv(&String::from_utf8_lossy(&f.body));
        print_table("QCAL-1 table3 (VizieR)", &t, declared_q3);
        let srcs = source_rows(&t);
        coverage_rows.push(coverage("QCAL-1 table3 (637 flux catalog)", &srcs, &icrf));
        catalog_tables.push(("QCAL-1 table3".to_string(), t, 637));
    } else {
        println!("QCAL-1 table3 (VizieR): absent/unreadable — columns and rows not measurable");
    }

    if let Some(f) = &q4_fetch {
        let t = parse_csv(&String::from_utf8_lossy(&f.body));
        print_table("QCAL-1 table4 (VizieR)", &t, declared_q4);
        let srcs = source_rows(&t);
        coverage_rows.push(coverage("QCAL-1 table4 (33 first-VLBI positions)", &srcs, &icrf));
        catalog_tables.push(("QCAL-1 table4".to_string(), t, 33));
    } else {
        println!("QCAL-1 table4 (VizieR): absent/unreadable — columns and rows not measurable");
    }

    if let Some(f) = &ly_fetch {
        let t = parse_csv(&String::from_utf8_lossy(&f.body));
        print_table("Lanyi+2010 table3 (VizieR)", &t, declared_ly);
        let srcs = source_rows(&t);
        coverage_rows.push(coverage("Lanyi+2010 Q-band positions", &srcs, &icrf));
        catalog_tables.push(("Lanyi Q-band".to_string(), t, 131));
    } else {
        println!("Lanyi+2010 table3 (VizieR): absent/unreadable — columns and rows not measurable");
    }

    if let Some(f) = &t1_fetch {
        let text = String::from_utf8_lossy(&f.body);
        let rows = text
            .lines()
            .filter(|l| l.starts_with('J') && l.len() > 20)
            .count();
        println!(
            "QCAL-1 table3 (astrogeo raw): data rows {} | declared {} | match {}",
            rows,
            match declared_q3 {
                Some(d) => d.to_string(),
                None => "?".to_string(),
            },
            match declared_q3 {
                Some(d) if d == rows => "ok",
                Some(_) => "MISMATCH",
                None => "no declared count",
            }
        );
    } else {
        println!("QCAL-1 table3 (astrogeo raw): absent/unreadable");
    }

    if let Some(f) = &t2_fetch {
        let text = String::from_utf8_lossy(&f.body);
        let rows = text
            .lines()
            .filter(|l| l.starts_with('J') && l.len() > 20)
            .count();
        println!(
            "QCAL-1 table4 (astrogeo raw): data rows {} | declared {} | match {}",
            rows,
            match declared_q4 {
                Some(d) => d.to_string(),
                None => "?".to_string(),
            },
            match declared_q4 {
                Some(d) if d == rows => "ok",
                Some(_) => "MISMATCH",
                None => "no declared count",
            }
        );
    } else {
        println!("QCAL-1 table4 (astrogeo raw): absent/unreadable");
    }

    if let Some(f) = &page_fetch {
        let html = String::from_utf8_lossy(&f.body);
        println!(
            "\nQCAL-1 page: format {} | catalog links {} | raw FITS links {}",
            if html.contains("<HTML") { "HTML" } else { "unknown" },
            html.matches("datafile1.txt").count(),
            html.matches(".fits").count() + html.matches(".FITS").count()
        );
        println!(
            "  license/terms line present: {}",
            if html.contains("cannot be displayed on-line") {
                "yes (content may not be mirrored without written permission)"
            } else {
                "not found"
            }
        );
    }

    println!("\n== Coverage against ICRF3 S/X (nearest neighbour, tolerance 1.5 arcsec)");
    println!("catalog | rows | matched | matched D | fraction | median sep [mas] | >5 mas | max sep [mas]");
    for c in &coverage_rows {
        println!(
            "{} | {} | {} | {} | {} | {} | {} | {}",
            c.label,
            c.n,
            c.matched,
            c.matched_d,
            pct(c.matched, c.n),
            match c.median_sep_mas {
                Some(v) => format!("{v:.2}"),
                None => "n/a".to_string(),
            },
            c.above5_mas,
            match c.max_sep_mas {
                Some(v) => format!("{v:.2}"),
                None => "n/a".to_string(),
            }
        );
    }

    println!("\n== Position precision of the Q-band catalogs (formal errors)");
    for (label, t, _) in &catalog_tables {
        let erakey = if t.header.iter().any(|h| h == "e_RAJ2000") {
            "e_RAJ2000"
        } else {
            ""
        };
        if erakey.is_empty() {
            println!("{label}: no per-source position error column");
            continue;
        }
        let mut era: Vec<f64> = Vec::new();
        let mut ede: Vec<f64> = Vec::new();
        for row in &t.rows {
            if let (Some(a), Some(b)) = (t.column("e_RAJ2000", row), t.column("e_DEJ2000", row)) {
                if a.is_finite() && b.is_finite() && a > 0.0 && b > 0.0 {
                    era.push(a);
                    ede.push(b);
                }
            }
        }
        let is_sec = label.contains("Lanyi");
        let scale_ra = if is_sec { 15.0 * MAS_PER_ARCSEC } else { 1.0 };
        let scale_de = if is_sec { MAS_PER_ARCSEC } else { 1.0 };
        let mut ra_mas: Vec<f64> = era.iter().map(|v| v * scale_ra).collect();
        let mut de_mas: Vec<f64> = ede.iter().map(|v| v * scale_de).collect();
        println!(
            "{label}: n {} | median e_RA {} mas | median e_DE {} mas",
            ra_mas.len(),
            match median(&mut ra_mas) {
                Some(v) => format!("{v:.2}"),
                None => "n/a".to_string(),
            },
            match median(&mut de_mas) {
                Some(v) => format!("{v:.2}"),
                None => "n/a".to_string(),
            }
        );
    }

    println!("\n== Sky coverage and epoch");
    for (label, t, _) in &catalog_tables {
        let mut decs = Vec::new();
        for row in &t.rows {
            if let Some(d) = t.column("DEJ2000", row) {
                if d.is_finite() {
                    decs.push(d);
                }
            }
        }
        if decs.is_empty() {
            continue;
        }
        let south = decs.iter().filter(|d| **d < -40.0).count();
        println!(
            "{label}: dec min {:.2} deg | max {:.2} deg | south of -40 deg {}",
            decs.iter().copied().fold(f64::INFINITY, f64::min),
            decs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            south
        );
    }
    for (label, t, _) in &catalog_tables {
        for column in ["N", "Nobs", "Nep", "S43"] {
            if t.header.iter().any(|h| h == column) {
                print_range(label, t, column);
            }
        }
    }

    if let Some(path) = out_path {
        let mut tsv = String::from("catalog\tsource\tra_deg\tdec_deg\tnearest_icrf\tiers\tdefining\tsep_mas\n");
        for (label, t, _) in &catalog_tables {
            for (name, ra, dec) in source_rows(t) {
                let hit = nearest(ra, dec, &icrf);
                let (icrf_name, iers, defining, sep) = match hit {
                    Some(h) if h.sep_arcsec <= MATCH_ARCSEC => (
                        icrf[h.index].name.clone(),
                        icrf[h.index].iers.clone(),
                        icrf[h.index].defining,
                        format!("{:.2}", h.sep_arcsec * MAS_PER_ARCSEC),
                    ),
                    Some(h) => (
                        "-".to_string(),
                        "-".to_string(),
                        false,
                        format!("{:.2}", h.sep_arcsec * MAS_PER_ARCSEC),
                    ),
                    None => ("-".to_string(), "-".to_string(), false, "n/a".to_string()),
                };
                tsv.push_str(&format!(
                    "{label}\t{name}\t{ra:.8}\t{dec:.8}\t{icrf_name}\t{iers}\t{}\t{sep}\n",
                    if defining { "D" } else { "" }
                ));
            }
        }
        match fs::write(&path, tsv) {
            Ok(()) => println!("\ncrossmatch written: {}", path.display()),
            Err(e) => eprintln!("crossmatch not written: {} ({e})", path.display()),
        }
    }

    println!("\n== Verdict");
    println!(
        "Lanyi+2010 (VizieR {LANYI_T3}): machine-readable Q-band positions, 131 sources, 2010 epoch, VLBA."
    );
    println!(
        "QCAL-1 (VizieR {QCAL1_T3}): machine-readable 43 GHz flux densities for 637 sources, 2011 December epoch, KVN."
    );
    println!(
        "Raw astrogeo text is a mirror of the CDS byte-by-byte tables, not an independent measurement."
    );
    println!(
        "Independence: Q-band is a different frequency and resolves more compact cores, but Lanyi+2010 shares sources and telescopes with the ICRF; it is a structure test, not a frame origin."
    );
}
