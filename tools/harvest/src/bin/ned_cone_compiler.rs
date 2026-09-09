use std::collections::HashMap;
use std::process::Command;

const CONE_ENDPOINT: &str = "https://ned.ipac.caltech.edu/NED::API/ConeSearchByPosition";

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

fn hash64(s: &str) -> u64 {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    for b in s.as_bytes() {
        h = (h ^ u64::from(*b)).wrapping_mul(0x1000_0000_01B3);
    }
    splitmix64(h)
}

fn identity_rank(identity: &str) -> f64 {
    let h = hash64(identity);
    (h >> 11) as f64 / (1u64 << 53) as f64
}

fn latlon_cell(ra: f64, dec: f64, n: i64) -> Option<i64> {
    if !(ra.is_finite() && dec.is_finite() && (-90.0..=90.0).contains(&dec)) {
        return None;
    }
    let dec_idx = (((dec + 90.0) / 180.0 * n as f64).floor() as i64).clamp(0, n - 1);
    let ra_idx = ((ra.rem_euclid(360.0) / 360.0 * n as f64).floor() as i64).clamp(0, n - 1);
    Some(dec_idx * n + ra_idx)
}

struct Ring {
    dec: f64,
    n_ra: usize,
}

fn grid_rings(r_deg: f64, sigma_deg: f64) -> Vec<Ring> {
    let mut rings = Vec::new();
    let d_ring = 0.5 * 3.0f64.sqrt() * sigma_deg;
    let mut dec = -90.0 + r_deg;
    while dec <= 90.0 - r_deg {
        let circ = 360.0 * dec.to_radians().cos();
        let n = (circ / sigma_deg).round().max(1.0) as usize;
        rings.push(Ring { dec, n_ra: n });
        dec += d_ring;
    }
    rings
}

fn grid_total(r_deg: f64, sigma_deg: f64) -> usize {
    2 + grid_rings(r_deg, sigma_deg)
        .iter()
        .map(|r| r.n_ra)
        .sum::<usize>()
}

fn grid_center(index: usize, r_deg: f64, sigma_deg: f64) -> Option<(f64, f64)> {
    if index == 0 {
        return Some((0.0, 90.0));
    }
    if index == 1 {
        return Some((0.0, -90.0));
    }
    let rings = grid_rings(r_deg, sigma_deg);
    let mut rest = index - 2;
    for (k, ring) in rings.iter().enumerate() {
        if rest < ring.n_ra {
            let step = 360.0 / ring.n_ra as f64;
            let offset = if k % 2 == 1 { 0.5 * step } else { 0.0 };
            let ra = (rest as f64 * step + offset).rem_euclid(360.0);
            return Some((ra, ring.dec));
        }
        rest -= ring.n_ra;
    }
    None
}

fn fmt_num(v: f64) -> String {
    let s = format!("{}", v);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

fn cone_url(root: &str, ra: f64, dec: f64, sr_arcmin: f64, maxrec: usize) -> String {
    format!(
        "{}?RA={}d&DEC={}d&CSYS=Equatorial&EQUINOX=J2000.0&SR={}&Z_CONSTRAINT=Available&MAXREC={}",
        root,
        fmt_num(ra),
        fmt_num(dec),
        fmt_num(sr_arcmin),
        maxrec
    )
}

fn fetch_body(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("170")
        .arg("--retry")
        .arg("1")
        .arg("--retry-connrefused")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "ned_cone: curl http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn attr_map(tag: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let chars: Vec<char> = tag.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '=' {
            i += 1;
            continue;
        }
        let mut j = i;
        while j > 0 && (chars[j - 1].is_ascii_alphanumeric() || chars[j - 1] == '_') {
            j -= 1;
        }
        let key: String = chars[j..i].iter().collect();
        if key.is_empty() {
            i += 1;
            continue;
        }
        let mut k = i + 1;
        while k < chars.len() && chars[k].is_whitespace() {
            k += 1;
        }
        if k >= chars.len() || (chars[k] != '"' && chars[k] != '\'') {
            i += 1;
            continue;
        }
        let quote = chars[k];
        let mut v = String::new();
        k += 1;
        while k < chars.len() && chars[k] != quote {
            v.push(chars[k]);
            k += 1;
        }
        out.insert(key, v);
        i = k + 1;
    }
    out
}

fn votable_fields(body: &str) -> Vec<HashMap<String, String>> {
    let mut out = Vec::new();
    for f in body.split("<FIELD").skip(1) {
        let seg = match f.split_once('>') {
            Some((s, _)) => s,
            None => continue,
        };
        if seg.contains('=') {
            out.push(attr_map(seg));
        }
    }
    out
}

fn column_index(fields: &[HashMap<String, String>], keys: &[&str]) -> Option<usize> {
    fields.iter().position(|m| {
        keys.iter().any(|k| {
            m.get("ID").map(|v| v == k).unwrap_or(false)
                || m.get("name")
                    .map(|v| v.eq_ignore_ascii_case(k))
                    .unwrap_or(false)
                || m.get("ucd").map(|v| v == k).unwrap_or(false)
        })
    })
}

fn cone_rows(data: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    for tr in data.split("<TR>").skip(1) {
        let end = match tr.split_once("</TR>") {
            Some((e, _)) => e,
            None => continue,
        };
        let mut cells: Vec<String> = Vec::new();
        let mut pos = 0;
        loop {
            let rel = match end[pos..].find("<TD") {
                Some(i) => i,
                None => break,
            };
            let start = pos + rel;
            let tag_end = match end[start..].find('>') {
                Some(i) => start + i,
                None => break,
            };
            let tag = &end[start..tag_end];
            let self_closed = tag.trim_end().ends_with('/');
            let content_start = tag_end + 1;
            if self_closed {
                cells.push(String::new());
                pos = content_start;
                continue;
            }
            let close = match end[content_start..].find("</TD>") {
                Some(i) => content_start + i,
                None => break,
            };
            let raw = &end[content_start..close];
            let v = if let Some(c) = raw.strip_prefix("<![CDATA[") {
                c.strip_suffix("]]>").unwrap_or(c).trim().to_string()
            } else if let Some(st) = raw.find('<') {
                raw[..st].trim().to_string()
            } else {
                raw.trim().to_string()
            };
            cells.push(v);
            pos = close + "</TD>".len();
        }
        rows.push(cells);
    }
    rows
}

fn query_status(body: &str) -> Option<String> {
    let mut best: Option<String> = None;
    let mut cursor = 0;
    while let Some(p) = body[cursor..].find("PARAM") {
        let start = cursor + p;
        let end = match body[start..].find('>') {
            Some(i) => start + i,
            None => break,
        };
        let tag = &body[start..end];
        let m = attr_map(tag);
        if m.get("name").map(|v| v == "QUERY_STATUS").unwrap_or(false) {
            best = m.get("value").cloned();
        }
        cursor = end + 1;
    }
    best
}

struct Object {
    identity: String,
    name: Option<String>,
    ra: f64,
    dec: f64,
    z: f64,
    cz: Option<f64>,
    ptype: Option<String>,
    n_spectra: Option<i64>,
    cell: i64,
    rank: f64,
}

fn parse_cone(body: &str, lattice: i64) -> Option<(Vec<Object>, usize)> {
    let status = query_status(body)?;
    if !status.eq_ignore_ascii_case("OK") {
        let reason = body
            .split("DESCRIPTION")
            .nth(1)
            .and_then(|d| d.split('<').next())
            .map(|s| s.trim().to_string());
        match reason {
            Some(r) => eprintln!("ned_cone: query status {} — {}", status, r),
            None => eprintln!("ned_cone: query status {}", status),
        }
        return None;
    }
    let data = match body.split("<DATA>").nth(1) {
        Some(d) => match d.split("</DATA>").next() {
            Some(inner) => inner,
            None => d,
        },
        None => {
            eprintln!("ned_cone: <DATA> absent");
            return None;
        }
    };
    let fields = votable_fields(body);
    let i_name = column_index(&fields, &["prefname", "Object Name"]);
    let i_ra = column_index(&fields, &["ra", "RA"]);
    let i_dec = column_index(&fields, &["dec", "Dec"]);
    let i_z = column_index(&fields, &["z", "Redshift", "Redshift (z)"]);
    let i_cz = column_index(&fields, &["velocity", "cz"]);
    let i_ptype = column_index(&fields, &["ptype", "Phys Type"]);
    let i_n_spectra = column_index(&fields, &["n_spectra", "Spectra"]);
    let (Some(i_ra), Some(i_dec), Some(i_z)) = (i_ra, i_dec, i_z) else {
        eprintln!(
            "ned_cone: ra/dec/z column absent (ra {:?} dec {:?} z {:?})",
            i_ra, i_dec, i_z
        );
        return None;
    };
    let mut seen: HashMap<String, ()> = HashMap::new();
    let mut out = Vec::new();
    let mut skipped = 0usize;
    for row in cone_rows(data) {
        let c = |i: usize| row.get(i).map(|s| s.as_str()).unwrap_or("");
        let fcell = |i: usize| c(i).parse::<f64>().ok();
        let (Some(ra), Some(dec), Some(z)) = (fcell(i_ra), fcell(i_dec), fcell(i_z)) else {
            skipped += 1;
            continue;
        };
        if !(ra.is_finite() && dec.is_finite() && z.is_finite()) {
            skipped += 1;
            continue;
        }
        if !(-90.0..=90.0).contains(&dec) {
            skipped += 1;
            continue;
        }
        let name_raw = i_name.map(c).unwrap_or("").trim();
        let name = if name_raw.is_empty() {
            None
        } else {
            Some(name_raw.to_string())
        };
        let identity = match &name {
            Some(n) => n.clone(),
            None => format!("{:x}.{:x}", ra.to_bits(), dec.to_bits()),
        };
        if seen.contains_key(&identity) {
            skipped += 1;
            continue;
        }
        seen.insert(identity.clone(), ());
        let cz = i_cz.and_then(fcell).filter(|v| v.is_finite());
        let ptype = i_ptype
            .map(c)
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string());
        let n_spectra = i_n_spectra
            .and_then(fcell)
            .filter(|v| v.is_finite())
            .map(|v| v as i64);
        let cell = latlon_cell(ra, dec, lattice)?;
        let rank = identity_rank(&identity);
        out.push(Object {
            identity,
            name,
            ra,
            dec,
            z,
            cz,
            ptype,
            n_spectra,
            cell,
            rank,
        });
    }
    let key = |o: &Object| (o.cell, (o.rank * 1e12) as i64, o.identity.clone());
    out.sort_by_key(key);
    Some((out, skipped))
}

fn json_string(s: &str) -> String {
    let mut o = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn write_array(path: &str, objs: &[Object]) {
    let mut buf = String::from("[\n");
    for (i, o) in objs.iter().enumerate() {
        if i > 0 {
            buf.push_str(",\n");
        }
        buf.push_str("  {");
        let mut first = true;
        let mut field = |k: &str, v: String| {
            if !first {
                buf.push(',');
            }
            first = false;
            buf.push_str(&format!("\"{}\":{}", k, v));
        };
        if let Some(n) = &o.name {
            field("name", json_string(n));
        }
        field("ra", format!("{}", o.ra));
        field("dec", format!("{}", o.dec));
        field("z", format!("{}", o.z));
        if let Some(cz) = o.cz {
            field("cz", format!("{}", cz));
        }
        if let Some(p) = &o.ptype {
            field("ptype", json_string(p));
        }
        if let Some(n) = o.n_spectra {
            field("n_spectra", format!("{}", n));
        }
        field("cell", format!("{}", o.cell));
        field("rank", format!("{}", o.rank));
        buf.push('}');
    }
    buf.push_str("\n]\n");
    let tmp = format!("{}.tmp", path);
    match std::fs::write(&tmp, buf.as_bytes()) {
        Ok(_) => match std::fs::rename(&tmp, path) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("write {}: {}", path, err);
                std::process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("write {}: {}", tmp, err);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut index: Option<usize> = None;
    let mut ra_dir: Option<f64> = None;
    let mut dec_dir: Option<f64> = None;
    let mut sr_arcmin: f64 = 30.0;
    let mut spacing_deg: Option<f64> = None;
    let mut maxrec: usize = 2000;
    let mut lattice: i64 = 1024;
    let mut out: Option<String> = None;
    let mut root = CONE_ENDPOINT.to_string();
    let mut count_only = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--index" => {
                index = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--ra" => {
                ra_dir = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--dec" => {
                dec_dir = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--sr-arcmin" => {
                sr_arcmin = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(30.0);
                i += 1;
            }
            "--spacing-deg" => {
                spacing_deg = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--maxrec" => {
                maxrec = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(2000);
                i += 1;
            }
            "--lattice" => {
                lattice = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1024);
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--root" => {
                if let Some(v) = args.get(i + 1) {
                    root = v.clone();
                }
                i += 1;
            }
            "--count" => count_only = true,
            _ => {}
        }
        i += 1;
    }
    let r_deg = sr_arcmin / 60.0;
    let sigma = spacing_deg.unwrap_or(r_deg * 3.0f64.sqrt());
    if count_only {
        println!("{}", grid_total(r_deg, sigma));
        return;
    }
    let (ra, dec) = match (ra_dir, dec_dir) {
        (Some(a), Some(d)) => (a, d),
        _ => {
            let idx = match index {
                Some(v) => v,
                None => {
                    eprintln!("ned_cone: --index absent (or give --ra and --dec)");
                    std::process::exit(1);
                }
            };
            match grid_center(idx, r_deg, sigma) {
                Some(c) => c,
                None => {
                    eprintln!(
                        "ned_cone: index {} outside the grid (total {})",
                        idx,
                        grid_total(r_deg, sigma)
                    );
                    std::process::exit(1);
                }
            }
        }
    };
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("ned_cone: --out absent");
            std::process::exit(1);
        }
    };
    let url = cone_url(&root, ra, dec, sr_arcmin, maxrec);
    let body = match fetch_body(&url) {
        Some(b) => b,
        None => {
            eprintln!(
                "ned_cone: fetch void at index {:?} ({}, {})",
                index, ra, dec
            );
            std::process::exit(1);
        }
    };
    match parse_cone(&body, lattice) {
        Some((objs, skipped)) => {
            write_array(&out_path, &objs);
            eprintln!(
                "ned_cone: ({:.4}, {:.4}) sr {}' : {} objects written ({} skipped) → {}",
                ra,
                dec,
                fmt_num(sr_arcmin),
                objs.len(),
                skipped,
                out_path
            );
        }
        None => {
            eprintln!(
                "ned_cone: parse void at ({:.4}, {:.4}) — the cone stays unharvested",
                ra, dec
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"<?xml version="1.0"?>
<VOTABLE version="1.3">
<RESOURCE type="results">
<PARAM name="QUERY_STATUS" value="OK"/>
<TABLE>
<FIELD ID="prefname" name="Object Name"/>
<FIELD ID="ra" name="RA"/>
<FIELD ID="dec" name="Dec"/>
<FIELD ID="z" name="Redshift (z)"/>
<FIELD ID="velocity" name="cz"/>
<FIELD ID="ptype" name="Phys Type"/>
<FIELD ID="n_spectra" name="Spectra"/>
<DATA><TABLEDATA>
<TR><TD>A</TD><TD>10.5</TD><TD>41.0</TD><TD>0.1</TD><TD>29979.2</TD><TD>G</TD><TD>3</TD></TR>
<TR><TD>A</TD><TD>10.5</TD><TD>41.0</TD><TD>0.1</TD><TD>29979.2</TD><TD>G</TD><TD>3</TD></TR>
<TR><TD>B</TD><TD>10.6</TD><TD>40.9</TD><TD>0.2</TD><TD>59958.4</TD><TD/><TD>0</TD></TR>
<TR><TD>C</TD><TD>10.7</TD><TD>41.2</TD><TD></TD><TD></TD><TD>G</TD><TD>1</TD></TR>
</TABLEDATA></DATA>
</TABLE>
</RESOURCE>
</VOTABLE>"#;

    #[test]
    fn parse_dedups_and_skips_absent_z() {
        let (objs, skipped) = parse_cone(FIXTURE, 1024).unwrap();
        assert_eq!(objs.len(), 2, "A deduplicated, C has no z");
        assert_eq!(skipped, 2, "the duplicate A and the z-less C");
        let a = objs
            .iter()
            .find(|o| o.name.as_deref() == Some("A"))
            .unwrap();
        let b = objs
            .iter()
            .find(|o| o.name.as_deref() == Some("B"))
            .unwrap();
        assert!((a.z - 0.1).abs() < 1e-12);
        assert!((a.cz.unwrap() - 29979.2).abs() < 1e-9);
        assert_eq!(a.n_spectra, Some(3));
        assert_eq!(a.ptype.as_deref(), Some("G"));
        assert_eq!(
            b.ptype, None,
            "the <TD/> cell is an absent ptype, not a value"
        );
        assert_eq!(
            b.n_spectra,
            Some(0),
            "a zero spectrum count is a real value"
        );
    }

    #[test]
    fn parse_is_deterministic_and_sorts_by_cell_rank() {
        let (objs, _) = parse_cone(FIXTURE, 1024).unwrap();
        let (objs2, _) = parse_cone(FIXTURE, 1024).unwrap();
        assert_eq!(objs.len(), objs2.len());
        for (o, p) in objs.iter().zip(objs2.iter()) {
            assert_eq!(o.identity, p.identity);
            assert_eq!(o.cell, p.cell);
            assert_eq!(o.rank.to_bits(), p.rank.to_bits());
            assert_eq!(o.ra.to_bits(), p.ra.to_bits());
        }
        let windows = objs.windows(2);
        for w in windows {
            assert!(w[0].cell <= w[1].cell);
        }
    }

    #[test]
    fn rank_lies_in_unit_interval_and_is_stable() {
        for name in ["Messier 031", "NGC 224", "2CXO J004243.8+411603", ""] {
            let r = identity_rank(name);
            assert!((0.0..1.0).contains(&r), "rank {} for {:?}", r, name);
            assert_eq!(r.to_bits(), identity_rank(name).to_bits());
        }
    }

    #[test]
    fn grid_index_map_is_total_and_unique() {
        let r = 0.5;
        let sigma = r * 3.0f64.sqrt();
        let total = grid_total(r, sigma);
        assert!(total > 1000);
        let mut seen = std::collections::HashSet::new();
        for i in 0..total {
            let (ra, dec) = grid_center(i, r, sigma).unwrap();
            assert!((-90.0..=90.0).contains(&dec), "dec {} out of range", dec);
            assert!((0.0..360.0).contains(&ra), "ra {} out of range", ra);
            assert!(seen.insert((ra.to_bits(), dec.to_bits())));
        }
        assert_eq!(seen.len(), total);
        assert!(grid_center(total, r, sigma).is_none());
        assert_eq!(grid_center(0, r, sigma), Some((0.0, 90.0)));
        assert_eq!(grid_center(1, r, sigma), Some((0.0, -90.0)));
    }

    #[test]
    fn cells_are_lattice_unique() {
        assert_eq!(latlon_cell(0.0, 0.0, 1024), Some(512 * 1024 + 0));
        assert_eq!(latlon_cell(359.9, -90.0, 1024), Some(1023));
        assert_eq!(
            latlon_cell(10.0, 41.0, 1024),
            Some(((41.0 + 90.0) / 180.0 * 1024.0) as i64 * 1024 + (10.0 / 360.0 * 1024.0) as i64)
        );
        assert!(latlon_cell(f64::NAN, 0.0, 1024).is_none());
    }
}
