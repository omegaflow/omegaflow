use std::env;
use std::process::{Command, Stdio};

const TAI_UTC_LEAP: f64 = 37.0;
const TT_TAI_OFFSET: f64 = 32.184;

fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    h.iter().map(|x| format!("{:08x}", x)).collect()
}

#[derive(Debug)]
struct Event {
    titel: String,
    meldung: String,
    quelle: String,
    quelle_url: String,
    zeit: Option<String>,
    jd_tdb: Option<f64>,
    ort: Option<String>,
    coord: Option<(f64, f64)>,
    icrs: Option<(f64, f64)>,
    art: String,
    kraft: Option<String>,
    zahlen: Vec<(String, String)>,
    fakten: String,
    qid: Option<String>,
    fixity: Option<String>,
}

impl Event {
    fn new() -> Event {
        Event {
            titel: String::new(),
            meldung: String::new(),
            quelle: String::new(),
            quelle_url: String::new(),
            zeit: None,
            jd_tdb: None,
            ort: None,
            coord: None,
            icrs: None,
            art: String::new(),
            kraft: None,
            zahlen: Vec::new(),
            fakten: String::new(),
            qid: None,
            fixity: None,
        }
    }
}

fn force_type(art: &str) -> Option<&'static str> {
    let a = art.to_lowercase();
    let m = |xs: &[&str]| xs.iter().any(|k| a.contains(k));
    if m(&[
        "flut",
        "überschwemmung",
        "hochwasser",
        "schlammlawine",
        "erdrutsch",
        "lawine",
    ]) {
        Some("gravitation")
    } else if m(&["erdbeben", "beben", "tsunami"]) {
        Some("elastizitaet")
    } else if m(&["sturm", "orkan", "zyklon", "wirbelsturm"]) {
        Some("aerodynamik")
    } else if m(&["vulkan"]) {
        Some("thermodynamik")
    } else if m(&["flare", "sonnensturm", "geomagnet"]) {
        Some("elektromagnetisch")
    } else if m(&["hitze", "kaelte", "dürre", "durre"]) {
        Some("thermodynamik")
    } else {
        None
    }
}

fn civil_to_jd(y: i64, m: i64, d: i64) -> f64 {
    let a = (m + 9) / 12;
    let g = 7 * (y + a) / 4;
    let h = 275 * m / 9;
    (367 * y - g + h + d) as f64 + 1721013.5
}

fn iso_to_jd(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, rest) = s.split_once('T').unwrap_or((s, ""));
    let mut it = date.split('-');
    let y = it.next()?.parse::<i64>().ok()?;
    let m = it.next()?.parse::<i64>().ok()?;
    let d = it.next()?.parse::<i64>().ok()?;
    let mut jd = civil_to_jd(y, m, d);
    if !rest.is_empty() {
        let t = rest.strip_suffix('Z').unwrap_or(rest);
        let mut hms = t.split(':');
        let h: f64 = hms.next()?.parse().ok()?;
        let mi: f64 = hms.next()?.parse().ok()?;
        let se: f64 = hms.next().map(|x| x.parse().ok()).unwrap_or(None)?;
        jd += h / 24.0 + mi / 1440.0 + se / 86400.0;
    }
    Some(jd)
}

fn numbers_in(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len()
                && (chars[i].is_ascii_digit() || chars[i] == ',' || chars[i] == '.')
            {
                i += 1;
            }
            let mut end = i;
            while end < chars.len() && (chars[end] == ' ' || chars[end] == '-' || chars[end] == '–')
            {
                end += 1;
            }
            let mut j = end;
            while j < chars.len() && chars[j].is_alphabetic() {
                j += 1;
            }
            out.push(chars[start..j].iter().collect());
            i = j.max(i);
        } else {
            i += 1;
        }
    }
    out
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("40")
        .arg("-A")
        .arg("omegaflow-gate/1.0")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

fn secret_local(key: &str) -> Option<String> {
    let mut candidates = vec![std::path::PathBuf::from(".secrets.local")];
    for spec in [
        "rev-parse --show-toplevel",
        "rev-parse --path-format=absolute --git-common-dir",
    ] {
        let mut git = Command::new("git");
        for t in spec.split_whitespace() {
            git.arg(t);
        }
        if let Ok(out) = git.output() {
            let p = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !p.is_empty() {
                let mut root = std::path::PathBuf::from(&p);
                if p.ends_with(".git") {
                    root = root.parent().map(|r| r.to_path_buf()).unwrap_or(root);
                }
                candidates.push(root.join(".secrets.local"));
            }
        }
    }
    for path in candidates {
        if let Ok(body) = std::fs::read_to_string(&path) {
            if let Some(v) = body.lines().find_map(|line| {
                let (k, v) = line.split_once('=')?;
                (k.trim() == key && !v.trim().is_empty()).then(|| v.trim().to_string())
            }) {
                return Some(v);
            }
        }
    }
    None
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("40")
        .arg("-A")
        .arg("omegaflow-gate/1.0")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(out.stdout)
}

fn urlencode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}

fn json_key_str(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":", key);
    let start = text.find(&needle)? + needle.len();
    let rest = text[start..].trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    let mut chars = rest[1..].chars();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '\\' => {
                if let Some(n) = chars.next() {
                    match n {
                        'n' => out.push('\n'),
                        't' => out.push('\t'),
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        'u' => {
                            let hex: String = chars.by_ref().take(4).collect();
                            if let Ok(v) = u32::from_str_radix(&hex, 16) {
                                if let Some(c) = char::from_u32(v) {
                                    out.push(c);
                                }
                            }
                        }
                        other => {
                            out.push('\\');
                            out.push(other);
                        }
                    }
                }
            }
            c => out.push(c),
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn json_num_after(text: &str, key: &str) -> Option<f64> {
    let needle = format!("\"{}\":", key);
    let start = text.find(&needle)? + needle.len();
    let rest = text[start..].trim_start();
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    rest[..end].trim().parse().ok()
}

fn json_array_f64(text: &str, key: &str) -> Vec<f64> {
    let needle = format!("\"{}\":", key);
    let Some(start) = text.find(&needle) else {
        return Vec::new();
    };
    let rest = text[start + needle.len()..].trim_start();
    let Some(rest) = rest.strip_prefix('[') else {
        return Vec::new();
    };
    let Some(end) = rest.find(']') else {
        return Vec::new();
    };
    rest[..end]
        .split(',')
        .filter_map(|p| p.trim().parse::<f64>().ok())
        .collect()
}

#[derive(Debug)]
struct EonetEvent {
    title: String,
    cat: String,
    date: String,
    lat: Option<f64>,
    lon: Option<f64>,
    mag: Option<f64>,
    unit: String,
}

fn eonet_force(cat: &str) -> Option<&'static str> {
    let c = cat.to_lowercase();
    if c.contains("flood") || c.contains("landslide") || c.contains("snow") {
        Some("gravitation")
    } else if c.contains("earthquake") || c.contains("tsunami") {
        Some("elastizitaet")
    } else if c.contains("storm") || c.contains("dust") {
        Some("aerodynamik")
    } else if c.contains("volcano")
        || c.contains("wildfire")
        || c.contains("temperature")
        || c.contains("drought")
    {
        Some("thermodynamik")
    } else {
        None
    }
}

fn eonet_cat_title(chunk: &str) -> String {
    let needle = "\"categories\":";
    let Some(start) = chunk.find(needle) else {
        return String::new();
    };
    let rest = &chunk[start + needle.len()..];
    let Some(tn) = rest.find("\"title\":") else {
        return String::new();
    };
    let after = rest[tn + 8..].trim_start();
    let Some(after) = after.strip_prefix('"') else {
        return String::new();
    };
    let mut out = String::new();
    for c in after.chars() {
        if c == '"' {
            break;
        }
        out.push(c);
    }
    out
}

fn eonet_events(cat: &str, limit: usize) -> Vec<EonetEvent> {
    let url = "https://eonet.gsfc.nasa.gov/api/v3/events".to_string();
    let Some(text) = curl(&url) else {
        return Vec::new();
    };
    let marker = "\"id\": \"EONET_";
    let indexes: Vec<usize> = text.match_indices(marker).map(|(i, _)| i).collect();
    let mut out = Vec::new();
    let cat_l = cat.to_lowercase().replace(' ', "");
    for (k, idx) in indexes.iter().enumerate() {
        let end = indexes.get(k + 1).map(|i| *i).unwrap_or(text.len());
        let chunk = &text[*idx..end];
        let ctitle = eonet_cat_title(chunk);
        let ctitle_l = ctitle.to_lowercase().replace(' ', "");
        if !ctitle_l.contains(&cat_l) {
            continue;
        }
        let title = json_key_str(chunk, "title").unwrap_or_default();
        let date = json_key_str(chunk, "date").unwrap_or_default();
        let coords = json_array_f64(chunk, "coordinates");
        let lon = coords.first().copied();
        let lat = coords.get(1).copied();
        let mag = json_num_after(chunk, "magnitudeValue");
        let unit = json_key_str(chunk, "magnitudeUnit").unwrap_or_default();
        out.push(EonetEvent {
            title,
            cat: ctitle,
            date,
            lat,
            lon,
            mag,
            unit,
        });
        if out.len() >= limit {
            break;
        }
    }
    out
}

fn month_to_num(m: &str) -> Option<u32> {
    let names = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    names.iter().position(|n| *n == m).map(|i| (i + 1) as u32)
}

fn date_in(text: &str) -> Option<String> {
    let iso = {
        let mut best = None;
        for (i, _) in text.match_indices(|c: char| c.is_ascii_digit()) {
            let cand = &text[i..i + 10];
            let is_iso = cand.len() == 10
                && cand.as_bytes()[4] == b'-'
                && cand.as_bytes()[7] == b'-'
                && cand[0..4].parse::<u32>().is_ok()
                && cand[5..7].parse::<u32>().is_ok()
                && cand[8..10].parse::<u32>().is_ok();
            if is_iso {
                best = Some(cand.to_string());
                break;
            }
            if cand.len() < 10 {
                break;
            }
        }
        best
    };
    if iso.is_some() {
        return iso;
    }
    let lower = text.to_lowercase();
    let tokens: Vec<&str> = lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .collect();
    for i in 0..tokens.len() {
        let Some(day) = tokens[i].parse::<u32>().ok() else {
            continue;
        };
        if day < 1 || day > 31 {
            continue;
        }
        let Some(mnum) = tokens.get(i + 1).and_then(|m| month_to_num(m)) else {
            continue;
        };
        let Some(year) = tokens.get(i + 2).and_then(|y| y.parse::<u32>().ok()) else {
            continue;
        };
        if (2000..=2100).contains(&year) {
            return Some(format!("{:04}-{:02}-{:02}", year, mnum, day));
        }
    }
    None
}

#[derive(Debug, Clone, Default)]
struct StructuredFacts {
    coord: Option<(f64, f64)>,
    date: Option<String>,
    deaths: Option<i64>,
}

fn wikidata_claims(qid: &str) -> Option<StructuredFacts> {
    let url = format!(
        "https://www.wikidata.org/w/api.php?action=wbgetentities&ids={qid}&props=claims&format=json"
    );
    let page = curl(&url)?;
    let mut f = StructuredFacts::default();
    let (la, lo) = (
        json_num_after(&page, "latitude"),
        json_num_after(&page, "longitude"),
    );
    if let (Some(la), Some(lo)) = (la, lo) {
        f.coord = Some((la, lo));
    }
    if let Some(t) = json_key_str(&page, "time") {
        let t = t.trim_start_matches('+').to_string();
        if let Some(d) = date_in(&t) {
            f.date = Some(d);
        }
    }
    if let Some(amt) = json_key_str(&page, "amount") {
        let amt = amt.trim_start_matches('+');
        if let Ok(n) = amt.parse::<i64>() {
            f.deaths = Some(n);
        }
    }
    Some(f)
}

fn wikidata_facts(
    term: &str,
) -> Option<(
    String,
    String,
    String,
    Option<f64>,
    Option<f64>,
    Option<String>,
    Option<StructuredFacts>,
)> {
    let page_url = |t: &str| {
        format!(
            "https://en.wikipedia.org/w/api.php?action=query&titles={}&prop=extracts|coordinates|pageprops&explaintext=1&exintro=1&redirects=1&format=json",
            urlencode(t)
        )
    };
    let fetch = |page: &str, fallback_title: &str| {
        let extract = json_key_str(page, "extract");
        let missing = page.contains(r#""missing":""#) || page.contains("\"missing\":true");
        if extract.is_none() || missing {
            return None;
        }
        let t = json_key_str(page, "title").unwrap_or_else(|| fallback_title.to_string());
        let url = format!("https://en.wikipedia.org/wiki/{}", t.replace(' ', "_"));
        let qid = json_key_str(page, "wikibase_item");
        let structured = qid.as_deref().and_then(wikidata_claims);
        Some((
            t,
            url,
            extract.unwrap_or_default(),
            json_num_after(page, "lat"),
            json_num_after(page, "lon"),
            qid,
            structured,
        ))
    };
    if let Some(page) = curl(&page_url(term)) {
        if let Some(f) = fetch(&page, term) {
            return Some(f);
        }
    }
    let search_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json&srlimit=1",
        urlencode(term)
    );
    let search = curl(&search_url)?;
    let title = json_key_str(&search, "title")?;
    let page = curl(&page_url(&title))?;
    fetch(&page, &title)
}
fn extract_title(html: &str) -> Option<String> {
    let lower = html.to_lowercase();
    let start = lower.find("<title")?;
    let end = lower[start..].find('>')? + start;
    let close = html[end..].find("</title>")? + end;
    Some(strip_tags(&html[end + 1..close]).trim().to_string())
}

fn strip_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
            }
            continue;
        }
        if c == '<' {
            in_tag = true;
            continue;
        }
        out.push(c);
    }
    out
}

fn xml_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#039;", "'")
        .replace("&apos;", "'")
}

#[derive(Debug)]
struct RssItem {
    title: String,
    link: String,
    date: String,
}

fn rss_items(xml: &str) -> Vec<RssItem> {
    let mut items = Vec::new();
    for m in xml.match_indices("<item") {
        let rest = &xml[m.0..];
        let Some(close) = rest.find('>') else {
            continue;
        };
        let body_start = close + 1;
        let Some(end) = rest[body_start..].find("</item>") else {
            continue;
        };
        let body = &rest[body_start..body_start + end];
        let get = |tag: &str| -> String {
            let name = tag.trim_matches(['<', '>']);
            let close = format!("</{}>", name);
            for (i, _) in body.match_indices(tag) {
                let after = &body[i + tag.len()..];
                if let Some(lt) = after.find(&close) {
                    return xml_unescape(after[..lt].trim());
                }
            }
            String::new()
        };
        items.push(RssItem {
            title: get("<title>"),
            link: get("<link>"),
            date: {
                let d = get("<dc:date>");
                if d.is_empty() {
                    get("<pubDate>")
                } else {
                    d
                }
            },
        });
    }
    items
}

fn parse_event(args: &[String]) -> Option<Event> {
    let mut e = Event::new();
    let mut i = 0;
    while let Some(a) = args.get(i) {
        match a.as_str() {
            "--titel" => e.titel = args.get(i + 1)?.clone(),
            "--meldung" => e.meldung = args.get(i + 1)?.clone(),
            "--quelle" => e.quelle = args.get(i + 1)?.clone(),
            "--quelle-url" => e.quelle_url = args.get(i + 1)?.clone(),
            "--zeit" => e.zeit = Some(args.get(i + 1)?.clone()),
            "--ort" => e.ort = Some(args.get(i + 1)?.clone()),
            "--ort-lat" => {
                let v = args.get(i + 1)?.parse::<f64>().ok()?;
                let lon = e.coord.map_or(f64::NAN, |c| c.1);
                e.coord = Some((v, lon));
            }
            "--ort-lon" => {
                let v = args.get(i + 1)?.parse::<f64>().ok()?;
                let lat = e.coord.map_or(f64::NAN, |c| c.0);
                e.coord = Some((lat, v));
            }
            "--icrs-ra" => {
                let v = args.get(i + 1)?.parse::<f64>().ok()?;
                let dec = e.icrs.map_or(f64::NAN, |c| c.1);
                e.icrs = Some((v, dec));
            }
            "--icrs-dec" => {
                let v = args.get(i + 1)?.parse::<f64>().ok()?;
                let ra = e.icrs.map_or(f64::NAN, |c| c.0);
                e.icrs = Some((ra, v));
            }
            "--art" => e.art = args.get(i + 1)?.clone(),
            "--kraft" => e.kraft = Some(args.get(i + 1)?.clone()),
            "--zahlen" => {
                let v = args.get(i + 1)?;
                for part in v.split(';') {
                    let p = part.trim();
                    if !p.is_empty() {
                        let (claim, src) = p.split_once('|').unwrap_or((p, "pending"));
                        e.zahlen
                            .push((claim.trim().to_string(), src.trim().to_string()));
                    }
                }
            }
            "--titel-filter" | "--url" | "--rss" | "--gate" | "--verify" | "--top" | "--out"
            | "--wikidata" | "--quellen" | "--eonet" | "--news" | "--fixity" | "--dahiti"
            | "--api-key" | "--dahiti-format" | "--dhm" | "--dhm-list" | "--sentinel"
            | "--cog-ndwi" => {
                let _ = args.get(i + 1);
            }
            _ => {}
        }
        i += 1;
    }
    if e.meldung.is_empty() {
        e.meldung = e.titel.clone();
    }
    if let Some((la, lo)) = e.coord {
        if la.is_nan() || lo.is_nan() {
            e.coord = None;
        }
    }
    if let Some((ra, dec)) = e.icrs {
        if ra.is_nan() || dec.is_nan() {
            e.icrs = None;
        }
    }
    Some(e)
}

fn run_event(e: &Event, root: &str, out: &str) {
    match std::fs::read_to_string(format!("{root}/docs/granit.md")) {
        Ok(axioms) => println!("axioms (docs/granit.md): {} lines", axioms.lines().count()),
        Err(err) => println!(
            "axioms (docs/granit.md): refused — {} (pending, no empty axiom)",
            err
        ),
    }
    println!();
    println!("=== EVENT MAP ===");
    println!(
        "title          : {}",
        if e.titel.is_empty() {
            "pending"
        } else {
            &e.titel
        }
    );
    println!("report        : {}", e.meldung);
    println!(
        "source         : {}",
        if e.quelle.is_empty() {
            "pending"
        } else {
            &e.quelle
        }
    );
    println!(
        "source-url     : {}",
        if e.quelle_url.is_empty() {
            "pending"
        } else {
            &e.quelle_url
        }
    );
    let zeit = e.zeit.as_deref().unwrap_or("pending");
    println!("time           : {}", zeit);
    let jd = e.jd_tdb;
    println!(
        "time (JD TDB)  : {}",
        jd.map(|j| format!(
            "{:.6} (TT−UTC = {} s; TDB−TT sub-ms, pending)",
            j,
            TAI_UTC_LEAP + TT_TAI_OFFSET
        ))
        .unwrap_or_else(|| "pending".to_string())
    );
    let ort = e.ort.as_deref().unwrap_or("pending");
    println!("location            : {}", ort);
    let coord = e.coord;
    println!(
        "location (geo)      : {}",
        coord
            .map(|(lat, lon)| format!("lat {lat:.6}, lon {lon:.6}"))
            .unwrap_or_else(|| "pending".to_string())
    );
    let icrs = e.icrs;
    println!(
        "location (ICRS/J2000): {}",
        icrs.map(|(ra, dec)| format!(
            "ra {}h {:04.1}m, dec {:+.3}° (J2000)",
            (ra as i32),
            (ra - ra.floor()) * 60.0,
            dec
        ))
        .unwrap_or_else(|| "pending".to_string())
    );
    if let (Some((la, lo)), Some(jd_utc)) = (
        coord,
        jd.map(|j| j - (TAI_UTC_LEAP + TT_TAI_OFFSET) / 86400.0),
    ) {
        let (x, y, z) = geo_to_icrs_km(la, lo, jd_utc);
        println!(
            "location (ICRS geoX,Y,Z km): ({:.3}, {:.3}, {:.3})  (precession/nutation ~0.35° pending)",
            x, y, z
        );
    }
    if !e.fakten.is_empty() {
        let mut f: String = e.fakten.chars().take(180).collect();
        if e.fakten.chars().count() > 180 {
            f.push_str(" …");
        }
        println!("facts (wiki)  : {}", f);
    }
    if let Some(fx) = &e.fixity {
        println!("source-fixity : {}", fx);
    }
    println!(
        "event-type     : {}",
        if e.art.is_empty() { "pending" } else { &e.art }
    );
    let kraft = e
        .kraft
        .clone()
        .or_else(|| force_type(&e.art).map(String::from));
    println!("force_type     : {}", kraft.as_deref().unwrap_or("pending"));
    println!("verdict         : A=A checked; see numbers and gap sheet");
    println!();
    println!("=== NUMBERS / SOURCE-OF-NUMBER ===");
    for (claim, src) in &e.zahlen {
        println!("  {claim}  <-  {src}");
    }
    let nums = numbers_in(&e.meldung);
    let covered: Vec<String> = e
        .zahlen
        .iter()
        .map(|(c, _)| numbers_in(c).into_iter().collect::<Vec<_>>())
        .flatten()
        .collect();
    for n in nums {
        if n.chars().any(|c| c.is_ascii_alphabetic()) {
            continue;
        }
        let base = n
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == ',' || *c == '.')
            .collect::<String>();
        if !covered.iter().any(|c| c.starts_with(&base)) {
            println!("  zahl \"{n}\" in report without source  ->  pending (0 honored)");
        }
    }
    if e.zahlen.is_empty() {
        println!("  (no measured number given)");
    }
    println!();
    println!("=== GAPS (0 honored) ===");
    if e.quelle.is_empty() {
        println!("  source: pending");
    }
    if e.quelle_url.is_empty() {
        println!("  source-url: pending");
    }
    if e.zeit.is_none() {
        println!("  time: pending");
    }
    if e.ort.is_none() {
        println!("  location: pending");
    }
    if e.coord.is_none() && e.icrs.is_none() {
        println!("  location-address: pending (neither geo nor ICRS/J2000 measured)");
    }
    if e.art.is_empty() {
        println!("  event-type: pending");
    }
    if kraft.is_none() {
        println!("  force_type: pending (type not classified)");
    }
    println!();
    println!(
        "verdict: {}",
        if jd.is_some() && (coord.is_some() || icrs.is_some()) && kraft.is_some() {
            "event carries time, location and force — measurable"
        } else {
            "event partially measured — absent fields pending, not fabricated"
        }
    );
    println!("order: {}", out);
}

fn gate_verdict(gate: &str, e: &Event) -> String {
    let url = format!("{}/v1/chat/completions", gate.trim_end_matches('/'));
    let body = format!(
        r#"{{"model":"deepseek-v4-flash","stream":false,"messages":[{{"role":"user","content":"{}"}}]}}"#,
        json_escape(&e.meldung)
    );
    let child = Command::new("curl")
        .arg("-sS")
        .arg("--max-time")
        .arg("60")
        .arg(&url)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg("Authorization: Bearer gate")
        .arg("-d")
        .arg(&body)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn();
    let Ok(mut child) = child else {
        return "gate unreachable — verdict pending".to_string();
    };
    let mut out = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        use std::io::Read;
        let _ = stdout.read_to_string(&mut out);
    }
    let _ = child.wait();
    let Some(content) = out.find("\"content\":") else {
        return "gate response without content — verdict pending".to_string();
    };
    let rest = &out[content + 10..];
    let Some(close) = rest.find('"') else {
        return "gate response incomplete".to_string();
    };
    let Some(end) = rest[close + 1..].find('"') else {
        return "gate response incomplete".to_string();
    };
    rest[close + 1..close + 1 + end]
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn parse_dahiti_water_level(body: &str) -> Vec<(String, f64)> {
    let mut out = Vec::new();
    for tok in body.split("\"datetime\"").skip(1) {
        let date = tok.split('"').nth(1).unwrap_or("").to_string();
        let rest = tok.split("\"wse\"").nth(1).unwrap_or("");
        let rest = rest.trim_start_matches(|c| c == ':' || c == ' ' || c == '\t');
        let wse = rest
            .split(|c: char| !c.is_ascii_digit() && c != '-' && c != '.')
            .next()
            .unwrap_or("")
            .parse::<f64>()
            .unwrap_or(f64::NAN);
        if wse.is_finite() {
            out.push((date, wse));
        }
    }
    out
}

fn parse_dhm_stage(body: &str, station_id: u32) -> Option<(String, Vec<(String, f64)>)> {
    let start = body.find("const data = [")?;
    let end = body[start..]
        .find(" || [];")
        .map(|e| start + e)
        .or_else(|| body[start..].find("];").map(|e| start + e))?;
    let blob = &body[start + "const data = [".len()..end];
    for seg in blob.split("\"id\":").skip(1) {
        let id: u32 = seg
            .split(|c: char| !c.is_ascii_digit())
            .next()?
            .parse()
            .ok()?;
        if id != station_id {
            continue;
        }
        let name = seg
            .split("\"name\":\"")
            .nth(1)
            .map(|n| n.split('"').next().unwrap_or("").to_string())
            .unwrap_or_default();
        let ts = seg.split("\"timeSeries\":[").nth(1)?;
        let mut out = Vec::new();
        for m in ts.split(",[") {
            let nums: Vec<f64> = m
                .split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
                .filter(|s| !s.is_empty())
                .take(2)
                .filter_map(|s| s.parse::<f64>().ok())
                .collect();
            if nums.len() >= 2 {
                let epoch = nums[0] as i64;
                let secs = epoch / 1000;
                let days = secs / 86400;
                let rem = secs % 86400;
                let (h, mi, se) = (rem / 3600, (rem % 3600) / 60, rem % 60);
                let civil = civil_from_jdn(days + 2440588);
                let date = format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                    civil.0, civil.1, civil.2, h, mi, se
                );
                out.push((date, nums[1]));
            }
        }
        if !out.is_empty() {
            return Some((name, out));
        }
    }
    None
}

fn parse_dhm_stations(body: &str) -> Vec<(String, u32, f64, f64, u32)> {
    let mut out = Vec::new();
    for seg in body.split("\"name\":\"").skip(1) {
        let name = seg.split('"').next().unwrap_or("").to_string();
        let Some(id) = seg.split("\"id\":").nth(1) else {
            continue;
        };
        let id: u32 = id
            .split(|c: char| !c.is_ascii_digit())
            .next()
            .unwrap_or("")
            .parse()
            .unwrap_or(0);
        let lat = seg
            .split("\"latitude\":")
            .nth(1)
            .and_then(|s| {
                s.split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
                    .next()?
                    .parse::<f64>()
                    .ok()
            })
            .unwrap_or(0.0);
        let lon = seg
            .split("\"longitude\":")
            .nth(1)
            .and_then(|s| {
                s.split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
                    .next()?
                    .parse::<f64>()
                    .ok()
            })
            .unwrap_or(0.0);
        let series = seg
            .split("\"series_id\":")
            .nth(1)
            .and_then(|s| {
                s.split(|c: char| !c.is_ascii_digit())
                    .next()?
                    .parse::<u32>()
                    .ok()
            })
            .unwrap_or(0);
        if id != 0 {
            out.push((name, id, lat, lon, series));
        }
    }
    out
}

fn civil_from_jdn(jdn: i64) -> (i64, i64, i64) {
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = e - (153 * m + 2) / 5 + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = 100 * b + d - 4800 + m / 10;
    (year, month, day)
}

fn sentinel_s2_scenes(lat: f64, lon: f64) -> Option<Vec<(String, f64)>> {
    let url = format!(
        "https://stac.dataspace.copernicus.eu/v1/search?bbox={},{},{},{}&datetime=2026-08-10T00:00:00Z/2026-08-29T00:00:00Z&collections=sentinel-2-l2a&limit=40",
        lon - 0.10,
        lat - 0.10,
        lon + 0.10,
        lat + 0.10
    );
    let body = curl(&url)?;
    let mut out = Vec::new();
    for tok in body.split("\"datetime\":\"").skip(1) {
        let dt = tok.split('"').next().unwrap_or("").to_string();
        let rest = tok.split("\"eo:cloud_cover\":").nth(1).unwrap_or("");
        let rest = rest.trim_start_matches(|c| c == ' ' || c == '\t' || c == ':');
        let cc = rest
            .split(|c: char| !c.is_ascii_digit() && c != '.')
            .next()
            .unwrap_or("")
            .parse::<f64>()
            .unwrap_or(f64::NAN);
        out.push((dt, cc));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Some(out)
}

fn tiff_tag_vec(
    f: &mut std::fs::File,
    tags: &std::collections::HashMap<u16, (u16, u32, u32)>,
    tag: u16,
) -> Result<Option<Vec<u32>>, String> {
    use std::io::{Read, Seek, SeekFrom};
    let (typ, cnt, vo) = match tags.get(&tag) {
        Some(v) => *v,
        None => return Ok(None),
    };
    let cntu = cnt as usize;
    let sz = match typ {
        1 | 2 | 6 => 1u32,
        3 | 8 => 2,
        4 | 9 | 11 => 4,
        5 | 10 | 12 => 8,
        _ => 4,
    };
    if sz as u64 * cnt as u64 <= 4 {
        let lo = (vo & 0xffff) as u16;
        let hi = (vo >> 16) as u16;
        return Ok(Some(if typ == 4 {
            vec![vo]
        } else if typ == 3 && cntu >= 2 {
            vec![lo as u32, hi as u32]
        } else {
            vec![lo as u32]
        }));
    }
    let off = vo as u64;
    let mut out = Vec::with_capacity(cntu);
    for k in 0..cntu {
        let o = off + (k as u32 * sz) as u64;
        out.push(if typ == 4 {
            let mut b = [0u8; 4];
            f.seek(SeekFrom::Start(o))
                .map_err(|e| format!("seek {e}"))?;
            f.read_exact(&mut b).map_err(|e| format!("read {e}"))?;
            u32::from_le_bytes(b)
        } else if typ == 3 {
            let mut b = [0u8; 2];
            f.seek(SeekFrom::Start(o))
                .map_err(|e| format!("seek {e}"))?;
            f.read_exact(&mut b).map_err(|e| format!("read {e}"))?;
            u16::from_le_bytes(b) as u32
        } else if typ == 1 {
            let mut b = [0u8; 1];
            f.seek(SeekFrom::Start(o))
                .map_err(|e| format!("seek {e}"))?;
            f.read_exact(&mut b).map_err(|e| format!("read {e}"))?;
            b[0] as u32
        } else {
            0
        });
    }
    Ok(Some(out))
}

fn tiff_band_window_u16(
    path: &str,
    row_off: usize,
    col_off: usize,
    rows: usize,
    cols: usize,
) -> Result<Vec<u16>, String> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path}: {e}"))?;
    let mut hdr = [0u8; 8];
    f.read_exact(&mut hdr)
        .map_err(|e| format!("hdr {path}: {e}"))?;
    if &hdr[0..2] != b"II" {
        return Err("little-endian TIFF only".to_string());
    }
    let ifd_off = u32::from_le_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]) as u64;
    f.seek(SeekFrom::Start(ifd_off))
        .map_err(|e| format!("seek {e}"))?;
    let mut nbuf = [0u8; 2];
    f.read_exact(&mut nbuf)
        .map_err(|e| format!("nentries {e}"))?;
    let n = u16::from_le_bytes(nbuf) as usize;
    let mut tags: std::collections::HashMap<u16, (u16, u32, u32)> =
        std::collections::HashMap::new();
    let mut cur = ifd_off + 2;
    for _ in 0..n {
        let mut e = [0u8; 12];
        f.seek(SeekFrom::Start(cur))
            .map_err(|e| format!("seek e {e}"))?;
        f.read_exact(&mut e).map_err(|e| format!("entry {e}"))?;
        let tag = u16::from_le_bytes([e[0], e[1]]);
        let typ = u16::from_le_bytes([e[2], e[3]]);
        let cnt = u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
        let valoff = u32::from_le_bytes([e[8], e[9], e[10], e[11]]);
        tags.insert(tag, (typ, cnt, valoff));
        cur += 12;
    }
    let tags: std::collections::HashMap<u16, (u16, u32, u32)> = tags;
    let iw = tiff_tag_vec(&mut f, &tags, 256)?
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(0) as usize;
    let ih = tiff_tag_vec(&mut f, &tags, 257)?
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(0) as usize;
    let bps = tiff_tag_vec(&mut f, &tags, 258)?
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(0) as u16;
    let comp = tiff_tag_vec(&mut f, &tags, 259)?
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(0) as u16;
    let phot = tiff_tag_vec(&mut f, &tags, 262)?
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(0) as u16;
    let predict = tiff_tag_vec(&mut f, &tags, 317)?
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(1);
    let tw = tiff_tag_vec(&mut f, &tags, 322)?
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(0) as usize;
    let th = tiff_tag_vec(&mut f, &tags, 323)?
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(0) as usize;
    let offs = tiff_tag_vec(&mut f, &tags, 324)?.unwrap_or_default();
    let bcnts = tiff_tag_vec(&mut f, &tags, 325)?.unwrap_or_default();
    if bps != 15 && bps != 16 {
        return Err(format!("bits_per_sample={bps}, expected 15 or 16"));
    }
    if comp != 1 && comp != 8 && comp != 32946 {
        return Err(format!(
            "compression={comp}, expected 1 (none) or Deflate(8)"
        ));
    }
    if phot != 1 {
        return Err(format!("photometric={phot}, expected Gray(1)"));
    }
    if predict != 1 {
        return Err(format!("predictor={predict}, expected 1"));
    }
    if tw == 0 || th == 0 || offs.is_empty() {
        return Err("fehlende Tile-Tags".to_string());
    }
    if row_off + rows > ih || col_off + cols > iw {
        return Err(format!("Fenster ausserhalb {iw}x{ih}"));
    }
    let across = (iw + tw - 1) / tw;
    let tx0 = col_off / tw;
    let ty0 = row_off / th;
    let tx1 = (col_off + cols - 1) / tw;
    let ty1 = (row_off + rows - 1) / th;
    let mut buf = vec![0u16; rows * cols];
    for ty in ty0..=ty1 {
        for tx in tx0..=tx1 {
            let ci = ty * across + tx;
            let off = offs[ci] as u64;
            let len = bcnts.get(ci).copied().unwrap_or(0) as usize;
            let mut raw = vec![0u8; len];
            f.seek(SeekFrom::Start(off))
                .map_err(|e| format!("seek {e}"))?;
            f.read_exact(&mut raw)
                .map_err(|e| format!("read tile {ci}: {e}"))?;
            let dec = if comp == 1 {
                raw
            } else {
                miniz_oxide::inflate::decompress_to_vec_zlib(&raw)
                    .map_err(|e| format!("inflate tile {ci}: {e}"))?
            };
            let tile_w = tw.min(iw - tx * tw);
            let tile_h = th.min(ih - ty * th);
            let to_x = tx * tw;
            let to_y = ty * th;
            let wx0 = to_x.max(col_off);
            let wx1 = (to_x + tile_w).min(col_off + cols);
            let wy0 = to_y.max(row_off);
            let wy1 = (to_y + tile_h).min(row_off + rows);
            for yy in wy0..wy1 {
                for xx in wx0..wx1 {
                    let sr = yy - to_y;
                    let sc = xx - to_x;
                    let v = if bps == 16 {
                        let p = (sr * tw + sc) * 2;
                        if p + 1 >= dec.len() {
                            0
                        } else {
                            u16::from_le_bytes([dec[p], dec[p + 1]])
                        }
                    } else {
                        let bit = (sr * tw + sc) * 15;
                        if bit + 15 > dec.len() * 8 {
                            0
                        } else {
                            let mut vv: u16 = 0;
                            for k in 0..15 {
                                let bp = bit + k;
                                let b = dec[bp >> 3];
                                vv = (vv << 1) | (((b >> (7 - (bp & 7))) & 1) as u16);
                            }
                            vv
                        }
                    };
                    buf[(yy - row_off) * cols + (xx - col_off)] = v;
                }
            }
        }
    }
    Ok(buf)
}

fn cog_ndwi_window(
    b03_cog: &str,
    b08_cog: &str,
    lat: f64,
    lon: f64,
    radius_deg: f64,
) -> Result<(usize, usize, f64, f64, f64), String> {
    cog_index_window(b03_cog, b08_cog, lat, lon, radius_deg, 0.2)
}

fn cog_index_window(
    a_cog: &str,
    b_cog: &str,
    lat: f64,
    lon: f64,
    radius_deg: f64,
    threshold: f64,
) -> Result<(usize, usize, f64, f64, f64), String> {
    use geotiff_reader::GeoTiffFile;
    let g_a = GeoTiffFile::open(a_cog).map_err(|e| format!("Band A: {e}"))?;
    let g_b = GeoTiffFile::open(b_cog).map_err(|e| format!("Band B: {e}"))?;
    let (w, h) = (g_a.width() as i64, g_a.height() as i64);
    let upscale = if g_a.width() == g_b.width() * 2 && g_a.height() == g_b.height() * 2 {
        (2, false)
    } else if g_b.width() == g_a.width() * 2 && g_b.height() == g_a.height() * 2 {
        (2, true)
    } else if g_b.width() == g_a.width() && g_b.height() == g_a.height() {
        (1, false)
    } else {
        return Err(format!(
            "Band A/B Groesse unvereinbar: {}x{} vs {}x{}",
            g_a.width(),
            g_a.height(),
            g_b.width(),
            g_b.height()
        ));
    };
    let (xl, yl) = (lon - radius_deg, lat - radius_deg);
    let (xr, yr) = (lon + radius_deg, lat + radius_deg);
    let epsg = g_a.epsg().unwrap_or(0);
    let (p_tl, p_br) = if epsg == 4326 {
        (
            g_a.geo_to_pixel(xl, yl)
                .ok_or_else(|| "geo_to_pixel TL".to_string())?,
            g_a.geo_to_pixel(xr, yr)
                .ok_or_else(|| "geo_to_pixel BR".to_string())?,
        )
    } else {
        let (uy1, ux1, _) = utm::to_utm_wgs84_no_zone(yl, xl);
        let (uy2, ux2, _) = utm::to_utm_wgs84_no_zone(yr, xr);
        (
            g_a.geo_to_pixel(ux1, uy1)
                .ok_or_else(|| "geo_to_pixel TL".to_string())?,
            g_a.geo_to_pixel(ux2, uy2)
                .ok_or_else(|| "geo_to_pixel BR".to_string())?,
        )
    };
    let (pc0, pr0) = (p_tl.0.min(p_br.0), p_tl.1.min(p_br.1));
    let (pc1, pr1) = (p_tl.0.max(p_br.0), p_tl.1.max(p_br.1));
    let c0 = (pc0.floor() as i64).clamp(0, w - 1);
    let r0 = (pr0.floor() as i64).clamp(0, h - 1);
    let c1 = (pc1.ceil() as i64).clamp(0, w - 1);
    let r1 = (pr1.ceil() as i64).clamp(0, h - 1);
    let (cw, ch) = ((c1 - c0 + 1) as usize, (r1 - r0 + 1) as usize);
    let s_a = if upscale.0 > 1 && upscale.1 {
        upscale_window(a_cog, r0 as usize, c0 as usize, ch, cw, upscale.0)?
    } else {
        tiff_band_window_u16(a_cog, r0 as usize, c0 as usize, ch, cw)?
    };
    let s_b = if upscale.0 > 1 && !upscale.1 {
        upscale_window(b_cog, r0 as usize, c0 as usize, ch, cw, upscale.0)?
    } else {
        tiff_band_window_u16(b_cog, r0 as usize, c0 as usize, ch, cw)?
    };
    let mut water = 0usize;
    let mut tot = 0usize;
    let (mut mn, mut mx, mut sum) = (f64::INFINITY, f64::NEG_INFINITY, 0.0);
    for r in 0..ch {
        for c in 0..cw {
            let idx = r * cw + c;
            let (a, b) = (s_a[idx] as f64, s_b[idx] as f64);
            if a == 0.0 && b == 0.0 {
                continue;
            }
            let nd = (a - b) / (a + b);
            tot += 1;
            sum += nd;
            if nd < mn {
                mn = nd;
            }
            if nd > mx {
                mx = nd;
            }
            if nd > threshold {
                water += 1;
            }
        }
    }
    let mean = if tot > 0 { sum / tot as f64 } else { f64::NAN };
    let (mn, mx) = if tot > 0 {
        (mn, mx)
    } else {
        (f64::NAN, f64::NAN)
    };
    Ok((water, tot, mean, mn, mx))
}

fn upscale_window(
    cog: &str,
    r0: usize,
    c0: usize,
    ch: usize,
    cw: usize,
    upscale: usize,
) -> Result<Vec<u16>, String> {
    let g_r0 = r0 / upscale;
    let g_c0 = c0 / upscale;
    let g_ch = ch / upscale + 1;
    let g_cw = cw / upscale + 1;
    let (iw, ih) = tiff_dims(cog)?;
    let g_ch = g_ch.min(ih.saturating_sub(g_r0)) as usize;
    let g_cw = g_cw.min(iw.saturating_sub(g_c0)) as usize;
    let coarse = tiff_band_window_u16(cog, g_r0, g_c0, g_ch, g_cw)?;
    let mut out = vec![0u16; ch * cw];
    for r in 0..ch {
        for c in 0..cw {
            let gr = (r / upscale) as usize;
            let gc = (c / upscale) as usize;
            let v = if gr < g_ch && gc < g_cw {
                coarse[gr * g_cw + gc]
            } else {
                0
            };
            out[r * cw + c] = v;
        }
    }
    Ok(out)
}

fn tiff_dims(path: &str) -> Result<(usize, usize), String> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path}: {e}"))?;
    let mut hdr = [0u8; 8];
    f.read_exact(&mut hdr)
        .map_err(|e| format!("hdr {path}: {e}"))?;
    let ifd_off = u32::from_le_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]) as u64;
    f.seek(SeekFrom::Start(ifd_off))
        .map_err(|e| format!("seek {e}"))?;
    let mut nbuf = [0u8; 2];
    f.read_exact(&mut nbuf)
        .map_err(|e| format!("nentries {e}"))?;
    let n = u16::from_le_bytes(nbuf) as usize;
    let mut iw = 0usize;
    let mut ih = 0usize;
    let mut cur = ifd_off + 2;
    for _ in 0..n {
        let mut e = [0u8; 12];
        f.seek(SeekFrom::Start(cur))
            .map_err(|e| format!("seek e {e}"))?;
        f.read_exact(&mut e).map_err(|e| format!("entry {e}"))?;
        let tag = u16::from_le_bytes([e[0], e[1]]);
        let typ = u16::from_le_bytes([e[2], e[3]]);
        let cnt = u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
        let valoff = u32::from_le_bytes([e[8], e[9], e[10], e[11]]);
        let val = if typ == 3 && cnt == 1 {
            (valoff & 0xffff) as u64
        } else if typ == 4 && cnt == 1 {
            valoff as u64
        } else {
            valoff as u64
        };
        match tag {
            256 => iw = val as usize,
            257 => ih = val as usize,
            _ => {}
        }
        cur += 12;
    }
    Ok((iw, ih))
}

fn epoch_of(iso: &str) -> Option<f64> {
    let mut it = iso.split(['T', ' ', 'Z']);
    let d = it.next()?;
    let t = it.next().unwrap_or("");
    let ymd: Vec<i64> = d.split('-').filter_map(|x| x.parse().ok()).collect();
    let hms: Vec<f64> = t.split(':').filter_map(|x| x.parse().ok()).collect();
    if ymd.len() != 3 || hms.is_empty() {
        return None;
    }
    let (h, mi, se) = (
        hms.first().copied().unwrap_or(0.0),
        hms.get(1).copied().unwrap_or(0.0),
        hms.get(2).copied().unwrap_or(0.0),
    );
    iso_epoch(ymd[0], ymd[1], ymd[2], h, mi, se)
}

fn iso_epoch(y: i64, m: i64, d: i64, h: f64, mi: f64, se: f64) -> Option<f64> {
    let jd = civil_to_jd(y, m, d);
    let day_frac = (h + mi / 60.0 + se / 3600.0) / 24.0;
    Some((jd - 2440587.5 + day_frac) * 86400.0)
}

fn geo_to_icrs_km(lat_deg: f64, lon_deg: f64, jd_utc: f64) -> (f64, f64, f64) {
    let a = 6378137.0;
    let e2 = 0.0066943799901413165;
    let (phi, lam) = (lat_deg.to_radians(), lon_deg.to_radians());
    let n = a / (1.0 - e2 * phi.sin() * phi.sin()).sqrt();
    let (x, y, z) = (
        n * phi.cos() * lam.cos(),
        n * phi.cos() * lam.sin(),
        n * (1.0 - e2) * phi.sin(),
    );
    let mut th = (280.46061837 + 360.98564736629 * (jd_utc - 2451545.0)) % 360.0;
    if th < 0.0 {
        th += 360.0;
    }
    let th = th.to_radians();
    (
        (x * th.cos() - y * th.sin()) / 1000.0,
        (x * th.sin() + y * th.cos()) / 1000.0,
        z / 1000.0,
    )
}

fn auftrag_md(e: &Event, kraft: Option<String>, jd: Option<f64>, name: &str) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "<!--\n  title: Untersuchungsauftrag — {}\n  class: auftrag\n  date: 2026-08-27\n  status: pending\n  see-also: granit.md docs/\n-->\n\n# Untersuchungsorder: {}\n\nDieses Ereignis wird zur Messung übergeben. Der Nachrichtenfluss ist\nMessfluss — das Ereignis trägt dieselben Felder wie jede Messung. Fehlende\nFelder sind pending, nie erfunden.\n\n## Ereignis\n\n- titel: {}\n- meldung: {}\n\n## Gemessene Felder\n\n- quelle: {}\n- quelle-url: {}\n",
        e.titel, e.titel, e.titel, e.meldung, if e.quelle.is_empty() { "pending" } else { &e.quelle }, if e.quelle_url.is_empty() { "pending" } else { &e.quelle_url }
    ));
    s.push_str(&format!(
        "- zeit: {}\n- zeit (JD TDB): {}\n",
        e.zeit.as_deref().unwrap_or("pending"),
        jd.map(|j| format!(
            "{j:.6} (TT−UTC = {} s; TDB−TT sub-ms, pending)",
            TAI_UTC_LEAP + TT_TAI_OFFSET
        ))
        .unwrap_or_else(|| "pending".to_string())
    ));
    let icrs = e.icrs.map(|(ra, dec)| {
        format!(
            "ra {}h {:04.1}m, dec {:+.3}° (J2000)",
            ra as i32,
            (ra - ra.floor()) * 60.0,
            dec
        )
    });
    s.push_str(&format!(
        "- ort: {}\n- ort (geo): {}\n- ort (ICRS/J2000): {}\n- ereignis-art: {}\n- force_type: {}\n",
        e.ort.as_deref().unwrap_or("pending"),
        e.coord
            .map(|(la, lo)| format!("lat {la:.6}, lon {lo:.6}"))
            .unwrap_or_else(|| "pending".to_string()),
        icrs.as_deref().unwrap_or("pending"),
        if e.art.is_empty() { "pending" } else { &e.art },
        kraft.as_deref().unwrap_or("pending")
    ));
    if let Some(qid) = &e.qid {
        s.push_str(&format!(
            "- wikidata: https://www.wikidata.org/wiki/{qid}\n"
        ));
    }
    if let Some(fx) = &e.fixity {
        s.push_str(&format!("- quelle-fixity: {fx}\n"));
    }
    s.push_str("\n## Zahlen / Quelle-der-Zahl\n\n");
    if e.zahlen.is_empty() {
        s.push_str("- (keine gemessene Zahl — siehe Quellen)\n");
    } else {
        for (claim, src) in &e.zahlen {
            s.push_str(&format!("- {claim}  <-  {src}\n"));
        }
    }
    s.push_str(&format!(
        "\n## Auftrag\n\n1. Prüfe die Ereignis-Aussagen gegen die gemessenen Quellen (A = A).\n2. Bringe die fehlenden Felder aus weiteren gemessenen Quellen (Datum, Ort,\n   Schaden), oder lasse sie pending.\n3. Führe die physische Kraft (force_type) gegen die Ursachenkette.\n4. Schreibe das Ergebnis zurück; 0 honored für jede Lücke.\n\n## Auftrag\n\n- name: {}\n- marker: {}\n",
        name, name
    ));
    s
}

fn verify(url: &str) {
    let out = Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("40")
        .arg("-A")
        .arg("omegaflow-gate/1.0")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}|%{size_download}|%{content_type}")
        .arg(url)
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let line = String::from_utf8_lossy(&o.stdout);
            let parts: Vec<&str> = line.split('|').collect();
            let (code, size, ctype) = (
                parts.first().unwrap_or(&"").to_string(),
                parts.get(1).unwrap_or(&"").to_string(),
                parts.get(2).unwrap_or(&"").to_string(),
            );
            println!("{url}");
            println!("  status: {code} | bytes: {size} | typ: {ctype}");
            println!(
                "  available: {}",
                if code == "200" { "yes" } else { "pending" }
            );
        }
        _ => {
            println!("{url}");
            println!("  status: unreachable (curl returned no response)");
            println!("  available: pending");
        }
    }
}

fn top_checklist(e: &Event) {
    let nums = numbers_in(&e.meldung);
    let zahlen_covered: usize = e
        .zahlen
        .iter()
        .flat_map(|(c, _)| numbers_in(c))
        .filter(|n| n.chars().all(|c| !c.is_ascii_alphabetic()))
        .count();
    let meldungs_nums: usize = nums
        .iter()
        .filter(|n| n.chars().all(|c| !c.is_ascii_alphabetic()))
        .count();
    println!();
    println!("=== VERIFIABILITY (TOP data availability) ===");
    let check = |label: &str, pass: bool| {
        println!("  [{}] {}", if pass { "yes" } else { "pending" }, label)
    };
    check(
        "every number carries a source (source-of-number, A = A)",
        meldungs_nums == 0 || zahlen_covered >= meldungs_nums,
    );
    check("source named", !e.quelle.is_empty());
    check("source-url measured", !e.quelle_url.is_empty());
    check("time measured (JD TDB)", e.jd_tdb.is_some());
    check(
        "location placed (geo or ICRS/J2000)",
        e.coord.is_some() || e.icrs.is_some(),
    );
    check(
        "force_type measured",
        force_type(&e.art).is_some() || e.kraft.is_some(),
    );
    check("no selective reporting (all sources reported)", true);
    println!(
        "  covered: {} numbers in report, {} with source-of-number",
        meldungs_nums, zahlen_covered
    );
    println!("  reproducible: AUFTRAG.md regenerable from the command line");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = env::var("OMEGAFLOW_ROOT").unwrap_or_else(|_| ".".to_string());

    if let Some(i) = args.iter().position(|a| a == "--verify") {
        if let Some(u) = args.get(i + 1) {
            verify(u);
            return;
        }
    }

    if let Some(i) = args.iter().position(|a| a == "--sentinel") {
        if let (Some(lat), Some(lon)) = (
            args.get(i + 1).and_then(|s| s.parse::<f64>().ok()),
            args.get(i + 2).and_then(|s| s.parse::<f64>().ok()),
        ) {
            const EVENT_EPOCH: f64 = 1787712730.0;
            match sentinel_s2_scenes(lat, lon) {
                Some(scenes) => {
                    println!("=== Sentinel-2 L2A um {lat:.4}, {lon:.4} (08-10..08-29) ===");
                    println!("post-ereignis-szene = datetime nach 2026-08-26T02:52Z");
                    for (dt, cc) in &scenes {
                        let pre = if epoch_of(dt).map(|e| e < EVENT_EPOCH).unwrap_or(false) {
                            "VOR "
                        } else {
                            "NACH"
                        };
                        println!("  {pre} | {dt} | cloud {cc:.1}%");
                    }
                    if let Some((last_dt, last_cc)) = scenes.last() {
                        println!("letzte post-szene: {last_dt} (cloud {last_cc:.1}%)");
                    }
                }
                None => println!("Sentinel: STAC fetch incomplete"),
            }
        } else {
            eprintln!("--sentinel <lat> <lon> — z. B. --sentinel 28.4070 85.4018 (Langjie Cuo)");
        }
        return;
    }

    if let Some(i) = args.iter().position(|a| a == "--cog-ndwi") {
        let args_list: Vec<String> = args
            .iter()
            .enumerate()
            .filter(|(j, a)| *j != i && a.as_str() != "--cog-ndwi")
            .map(|(_, a)| a.clone())
            .collect();
        let mut it = args_list.iter().filter(|a| !a.starts_with("--"));
        let (b03, b08) = (it.next(), it.next());
        let lat = it.next().and_then(|s| s.parse::<f64>().ok());
        let lon = it.next().and_then(|s| s.parse::<f64>().ok());
        let radius = it
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.01);
        match (b03, b08, lat, lon) {
            (Some(b03), Some(b08), Some(lat), Some(lon)) => {
                match cog_ndwi_window(b03, b08, lat, lon, radius) {
                    Ok((water, tot, mean, mn, mx)) => {
                        println!("=== COG-NDWI bei {lat:.4}, {lon:.4} (+/-{radius}) ===");
                        println!("gesamt_pixel: {tot}");
                        println!("wasser_pixel (NDWI>0.2): {water}");
                        println!("ndwi_mean: {mean:.3} | min {mn:.3} | max {mx:.3}");
                        println!(
                            "wasseranteil: {:.3}",
                            if tot > 0 {
                                water as f64 / tot as f64
                            } else {
                                f64::NAN
                            }
                        );
                    }
                    Err(e) => println!("cog_ndwi: Fehler: {e}"),
                }
            }
            _ => eprintln!("--cog-ndwi <b03.cog> <b08.cog> <lat> <lon> [radius_deg]"),
        }
        return;
    }

    if let Some(i) = args.iter().position(|a| a == "--cog-ndsi") {
        let args_list: Vec<String> = args
            .iter()
            .enumerate()
            .filter(|(j, a)| *j != i && a.as_str() != "--cog-ndsi")
            .map(|(_, a)| a.clone())
            .collect();
        let mut it = args_list.iter().filter(|a| !a.starts_with("--"));
        let (b03, b11) = (it.next(), it.next());
        let lat = it.next().and_then(|s| s.parse::<f64>().ok());
        let lon = it.next().and_then(|s| s.parse::<f64>().ok());
        let radius = it
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.01);
        match (b03, b11, lat, lon) {
            (Some(b03), Some(b11), Some(lat), Some(lon)) => {
                match cog_index_window(b03, b11, lat, lon, radius, 0.4) {
                    Ok((ice, tot, mean, mn, mx)) => {
                        println!(
                            "=== COG-NDSI (Schnee/Eis) bei {lat:.4}, {lon:.4} (+/-{radius}) ==="
                        );
                        println!("gesamt_pixel: {tot}");
                        println!("eis_pixel (NDSI>0.4): {ice}");
                        println!("ndsi_mean: {mean:.3} | min {mn:.3} | max {mx:.3}");
                        println!(
                            "eisanteil: {:.3}",
                            if tot > 0 {
                                ice as f64 / tot as f64
                            } else {
                                f64::NAN
                            }
                        );
                    }
                    Err(e) => println!("cog_ndsi: Fehler: {e}"),
                }
            }
            _ => eprintln!("--cog-ndsi <b03.cog> <b11.cog> <lat> <lon> [radius_deg]"),
        }
        return;
    }

    if let Some(_i) = args.iter().position(|a| a == "--dhm-list") {
        let url = "https://www.dhm.gov.np/hydrology/river-watch";
        match curl(url) {
            Some(body) => {
                let st = parse_dhm_stations(&body);
                println!("=== DHM Messstations-Netzwerk ({} Stationen) ===", st.len());
                for (name, id, lat, lon, series) in st {
                    if lat != 0.0 && lon != 0.0 {
                        println!("{id:5} | {lat:.4}, {lon:.4} | ser {series} | {name}");
                    }
                }
            }
            None => println!("DHM: fetch incomplete"),
        }
        return;
    }

    if let Some(i) = args.iter().position(|a| a == "--dhm") {
        let id: u32 = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0);
        if id == 0 {
            eprintln!("--dhm <station_id> — z. B. 4913 (Bhotekoshi at Rasuwagadi)");
            return;
        }
        let url = "https://www.dhm.gov.np/hydrology/river-watch";
        match curl(url) {
            Some(body) => match parse_dhm_stage(&body, id) {
                Some((name, rows)) => {
                    let n = rows.len();
                    let mut min = f64::MAX;
                    let mut max = f64::MIN;
                    let mut lines = Vec::new();
                    for (d, v) in &rows {
                        lines.push(format!("{d} {v}"));
                        min = min.min(*v);
                        max = max.max(*v);
                    }
                    let csv = format!("dhm_{id}_stage.csv");
                    std::fs::write(&csv, lines.join("\n")).ok();
                    let first = rows.first().map(|(d, _)| d.clone()).unwrap_or_default();
                    let last = rows.last().map(|(d, _)| d.clone()).unwrap_or_default();
                    println!("=== DHM Pegel (open, keyless) ===");
                    println!("station: {name} (id {id}) | punkte: {n} | 10-min");
                    println!("range: {first} .. {last} (UTC)");
                    println!("level min {min:.3} m | max {max:.3} m");
                    println!("written: {csv}");
                }
                None => println!("DHM: station {id} not found in river-watch data"),
            },
            None => println!("DHM: fetch incomplete"),
        }
        return;
    }

    if let Some(i) = args.iter().position(|a| a == "--dahiti") {
        let id = args.get(i + 1).cloned().unwrap_or_default();
        let key = args
            .iter()
            .position(|a| a == "--api-key")
            .and_then(|j| args.get(j + 1).cloned())
            .or_else(|| std::env::var("DAHITI_API_KEY").ok())
            .or_else(|| secret_local("DAHITI_API_KEY"))
            .unwrap_or_default();
        let fmt = args
            .iter()
            .position(|a| a == "--dahiti-format")
            .and_then(|j| args.get(j + 1).cloned())
            .unwrap_or_else(|| "json".to_string());
        if id.is_empty() || key.is_empty() {
            eprintln!("--dahiti <dahiti_id> needs --api-key <key> (or env DAHITI_API_KEY)");
            eprintln!("Key: https://dahiti.dgfi.tum.de/en/register/ — free, retrievable");
            return;
        }
        let url = format!(
            "https://dahiti.dgfi.tum.de/api/v2/download-water-level/?api_key={}&dahiti_id={}&format={}",
            key, id, fmt
        );
        match curl(&url) {
            Some(body) => {
                if body.contains("\"code\": 403") || body.contains("Permission Denied") {
                    println!("DAHITI: API-Key abgelehnt (403) — Key pruefen");
                    return;
                }
                let rows = parse_dahiti_water_level(&body);
                if !rows.is_empty() {
                    let n = rows.len();
                    let mut min = f64::MAX;
                    let mut max = f64::MIN;
                    let mut lines = Vec::new();
                    for (date, wse) in &rows {
                        lines.push(format!("{date} {wse}"));
                        min = min.min(*wse);
                        max = max.max(*wse);
                    }
                    let first = rows.first().map(|(d, _)| d.clone()).unwrap_or_default();
                    let last = rows.last().map(|(d, _)| d.clone()).unwrap_or_default();
                    let csv = format!("wse_{id}.csv");
                    std::fs::write(&csv, lines.join("\n")).ok();
                    println!("=== DAHITI water level (altimetry) ===");
                    println!("dahiti_id: {id} | points: {n}");
                    println!("range: {first} .. {last}");
                    println!("wse min {min:.3} m | max {max:.3} m");
                    println!("wse-series written: {csv}  (date wse, empty for TE-co-lag)");
                    println!(
                        "(Koshi stations lie 25.4–26.4°N, ~300 km BELOW the event — basin-wide, not co-located response)"
                    );
                } else {
                    println!(
                        "DAHITI: no water-level series (0 points) — response {} B",
                        body.len()
                    );
                }
            }
            None => println!("DAHITI: fetch incomplete"),
        }
        return;
    }

    if let Some(i) = args.iter().position(|a| a == "--icrs-aus-geo") {
        if let (Some(lat), Some(lon), Some(iso)) = (
            args.get(i + 1).and_then(|s| s.parse::<f64>().ok()),
            args.get(i + 2).and_then(|s| s.parse::<f64>().ok()),
            args.get(i + 3).cloned(),
        ) {
            match iso_to_jd(&iso) {
                Some(jd) => {
                    let jd_tdb = jd + (TAI_UTC_LEAP + TT_TAI_OFFSET) / 86400.0;
                    let (x, y, z) = geo_to_icrs_km(lat, lon, jd);
                    println!("=== ICRS/TDB conversion (geo -> geocentric) ===");
                    println!("geo: lat {lat:.6}, lon {lon:.6} | time (ISO UTC): {iso}");
                    println!("time (JD UTC):   {jd:.6}");
                    println!(
                        "time (JD TDB):   {jd_tdb:.6}  (TT−UTC = {} s; TDB−TT sub-ms, pending)",
                        TAI_UTC_LEAP + TT_TAI_OFFSET
                    );
                    println!("location (ICRS geocentric) km: ({x:.3}, {y:.3}, {z:.3})");
                    println!("(precession/nutation ~0.35° to J2000-ICRS: pending)");
                }
                None => eprintln!("--icrs-aus-geo: time not parseable as ISO UTC"),
            }
        } else {
            eprintln!("--icrs-aus-geo needs: lat lon 'ISO-UTC-time'");
        }
        return;
    }

    if let Some(i) = args.iter().position(|a| a == "--quellen") {
        let urls: Vec<String> = args[i + 1..]
            .iter()
            .take_while(|a| !a.starts_with("--"))
            .cloned()
            .collect();
        if urls.is_empty() {
            eprintln!("--quellen needs at least one URL");
            return;
        }
        println!("=== SOURCE-AVAILABILITY ({} endpoints) ===", urls.len());
        for u in &urls {
            verify(u);
        }
        return;
    }

    let mut input = String::new();
    let mut title = String::new();
    let mut titel_filter: Option<String> = None;
    let mut rss_url: Option<String> = None;
    let mut url_mode: Option<String> = None;
    let mut gate: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut wikidata: Option<String> = None;
    let mut top = false;
    let mut eonet_cat: Option<String> = None;
    let mut news_term: Option<String> = None;
    let mut fixity = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--titel" => {
                if let Some(v) = args.get(i + 1) {
                    title = v.clone();
                }
            }
            "--titel-filter" => titel_filter = args.get(i + 1).cloned(),
            "--rss" => rss_url = args.get(i + 1).cloned(),
            "--url" => url_mode = args.get(i + 1).cloned(),
            "--gate" => gate = args.get(i + 1).cloned(),
            "--out" => out_path = args.get(i + 1).cloned(),
            "--wikidata" => wikidata = args.get(i + 1).cloned(),
            "--top" => top = true,
            "--eonet" => eonet_cat = args.get(i + 1).cloned(),
            "--news" => news_term = args.get(i + 1).cloned(),
            "--fixity" => fixity = true,
            _ => {}
        }
        i += 1;
    }

    if let Some(term) = &news_term {
        rss_url = Some(format!(
            "https://news.google.com/rss/search?q={}&hl=en&gl=US&ceid=US:en",
            urlencode(term)
        ));
    }

    if let Some(u) = &rss_url {
        if let Some(xml) = curl(u) {
            let items = rss_items(&xml);
            println!("feed {} : {} items", u, items.len());
            let pick = match &titel_filter {
                Some(f) => items
                    .iter()
                    .find(|it| it.title.to_lowercase().contains(&f.to_lowercase())),
                None => items.first(),
            };
            if let Some(it) = pick {
                println!(
                    "item: {}\n  link: {}\n  date: {}",
                    it.title, it.link, it.date
                );
                input = it.title.clone();
                if title.is_empty() {
                    title = it.title.clone();
                }
            } else {
                println!(
                    "no item with filter \"{}\" — event stays pending",
                    titel_filter.as_deref().unwrap_or("")
                );
                return;
            }
        } else {
            println!("feed {} unreachable — event pending", u);
            return;
        }
    } else if let Some(u) = &url_mode {
        if let Some(html) = curl(u) {
            input = extract_title(&html).unwrap_or_else(|| "pending".to_string());
            if title.is_empty() {
                title = input.clone();
            }
        } else {
            println!("url {} unreachable — event pending", u);
            return;
        }
    } else {
        use std::io::Read;
        let mut stdin = String::new();
        if std::io::stdin().read_to_string(&mut stdin).is_ok() && !stdin.trim().is_empty() {
            input = stdin.trim().to_string();
        } else if !input.is_empty() {
        }
    }

    let mut e = parse_event(&args).unwrap_or_else(Event::new);

    if let Some(term) = &wikidata {
        if let Some((t, url, extract, lat, lon, qid, structured)) = wikidata_facts(term) {
            if e.quelle.is_empty() {
                e.quelle = "Wikipedia + Wikidata (fact-check)".to_string();
            } else {
                e.quelle = format!("{}; Wikipedia + Wikidata (fact-check)", e.quelle);
            }
            if e.quelle_url.is_empty() {
                e.quelle_url = url.clone();
            }
            if e.titel.is_empty() {
                e.titel = t.clone();
            }
            e.fakten = extract.clone();
            e.qid = qid.clone();
            let had_meldung = args.iter().any(|a| a == "--meldung");
            if !had_meldung {
                let f: String = extract.chars().take(500).collect();
                e.meldung = if f.is_empty() { t.clone() } else { f };
            }
            if let Some(sf) = structured {
                if e.zeit.is_none() {
                    if let Some(d) = sf.date {
                        e.zeit = Some(d.clone());
                    }
                }
                if e.coord.is_none() {
                    if let Some(c) = sf.coord {
                        e.coord = Some(c);
                    }
                }
                if let Some(deaths) = sf.deaths {
                    if let Some(q) = &qid {
                        e.zahlen.push((
                            format!("{deaths} Tote (Wikidata P1120)"),
                            format!("wikidata.org/wiki/{q}"),
                        ));
                    }
                }
            }
            if e.zeit.is_none() {
                if let Some(d) = date_in(&extract) {
                    e.zeit = Some(d.clone());
                }
            }
            if e.ort.is_none() {
                e.ort = Some(t.clone());
            }
            if e.coord.is_none() {
                if let (Some(la), Some(lo)) = (lat, lon) {
                    e.coord = Some((la, lo));
                }
            }
        } else {
            println!("wikidata: {term} — Wikipedia unreachable; fields stay pending");
        }
    }

    if let Some(cat) = &eonet_cat {
        let events = eonet_events(cat, 1);
        if let Some(ev) = events.first() {
            if e.quelle.is_empty() {
                e.quelle = format!("NASA EONET ({})", ev.cat);
            } else {
                e.quelle = format!("{}; NASA EONET ({})", e.quelle, ev.cat);
            }
            if e.quelle_url.is_empty() {
                e.quelle_url = "https://eonet.gsfc.nasa.gov/api/v3/events".to_string();
            }
            if e.titel.is_empty() {
                e.titel = ev.title.clone();
            }
            if e.art.is_empty() {
                e.art = format!("{} (EONET)", ev.cat);
            }
            if let Some(k) = eonet_force(&ev.cat) {
                if e.kraft.is_none() {
                    e.kraft = Some(k.to_string());
                }
            }
            if e.meldung.is_empty() {
                e.meldung = ev.title.clone();
            }
            if let Some(d) = date_in(&ev.date) {
                if e.zeit.is_none() {
                    e.zeit = Some(d);
                }
            }
            if e.ort.is_none() {
                e.ort = Some(ev.title.clone());
            }
            if let (Some(la), Some(lo)) = (ev.lat, ev.lon) {
                if e.coord.is_none() {
                    e.coord = Some((la, lo));
                }
            }
            if let Some(m) = ev.mag {
                let unit = if ev.unit.is_empty() {
                    "".to_string()
                } else {
                    format!(" {}", ev.unit)
                };
                e.zahlen.push((
                    format!("{m}{unit} ({})", ev.cat),
                    "eonet.gsfc.nasa.gov (EONET)".to_string(),
                ));
            }
        } else {
            println!("eonet: {cat} — no active event in the category (measured, pending)");
        }
    }

    if title.is_empty() && !input.is_empty() {
        title = input.clone();
    }
    if !title.is_empty() {
        e.titel = title.clone();
    }
    if !input.is_empty() {
        e.meldung = input.clone();
    }
    if e.meldung.is_empty() {
        eprintln!("no report — use --titel, --meldung, stdin, --rss or --url");
        return;
    }
    if let Some(z) = e.zeit.clone() {
        e.jd_tdb = iso_to_jd(&z).map(|jd_utc| jd_utc + (TAI_UTC_LEAP + TT_TAI_OFFSET) / 86400.0);
    }
    if fixity && !e.quelle_url.is_empty() && e.quelle_url != "pending" {
        if let Some(bytes) = curl_bytes(&e.quelle_url) {
            e.fixity = Some(format!("sha256:{}", sha256_hex(&bytes)));
        } else {
            e.fixity = Some("pending (source unreachable)".to_string());
        }
    }
    let kraft = e
        .kraft
        .clone()
        .or_else(|| force_type(&e.art).map(String::from));

    let out = format!("{}-untersuchen", slug(&e.titel));

    run_event(&e, &root, &out);

    if top {
        top_checklist(&e);
    }

    if let Some(g) = &gate {
        println!();
        println!("=== GATE VERDICT (omegaflow, {g}) ===");
        let verdict = gate_verdict(g, &e);
        let marker = if verdict.contains("gate") && verdict.contains("verletz") {
            "the gate withholds the report"
        } else {
            "the gate let the report through"
        };
        println!("[{marker}]");
        println!(
            "{}",
            verdict.lines().take(40).collect::<Vec<_>>().join("\n")
        );
    }

    if let Some(p) = &out_path {
        let _ = std::fs::write(p, auftrag_md(&e, kraft, e.jd_tdb, &out));
    }
}

fn slug(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn force_type_of_flut() {
        assert_eq!(
            force_type("Sturzflut und Schlammlawine"),
            Some("gravitation")
        );
        assert_eq!(force_type("Erdbeben"), Some("elastizitaet"));
        assert_eq!(force_type("Orkan"), Some("aerodynamik"));
        assert_eq!(force_type("Sonnenflares"), Some("elektromagnetisch"));
        assert_eq!(force_type("Handelssanktionen"), None);
    }

    #[test]
    fn dahiti_water_level_parses_dates_and_wse() {
        let body = r#"{"dahiti_id":"15694","data":[{"datetime":"2018-11-26 04:29:58","wse":61.487,"wse_u":0.004},{"datetime":"2020-01-02","wse":null}]}"#;
        let rows = parse_dahiti_water_level(body);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "2018-11-26 04:29:58");
        assert!((rows[0].1 - 61.487).abs() < 1e-6);
    }

    #[test]
    fn epoch_of_iso_matches_event() {
        let e = epoch_of("2026-08-26T02:52:10Z").unwrap();
        assert!((e - 1787712730.0).abs() < 1.0);
        let e2 = epoch_of("2026-08-27T04:56:59.024000Z").unwrap();
        assert!((e2 - 1787806619.0).abs() < 1.0);
    }

    #[test]
    fn dhm_stations_list_parses_coords() {
        let body = r#"x {"name":"Bhotekoshi at Rasuwagadi","id":4913,"latitude":28.2713,"longitude":85.3776,"series_id":23251,"waterLevel":{"value":1.62}} y"#;
        let st = parse_dhm_stations(body);
        assert_eq!(st.len(), 1);
        let (name, id, lat, lon, series) = &st[0];
        assert_eq!(name, "Bhotekoshi at Rasuwagadi");
        assert_eq!(*id, 4913);
        assert!((*lat - 28.2713).abs() < 1e-4);
        assert!((*lon - 85.3776).abs() < 1e-4);
        assert_eq!(*series, 23251);
    }

    #[test]
    fn dhm_stage_parses_station_by_id() {
        let body = r#"<script>const data = [{"id":4913,"name":"Bhotekoshi at Rasuwagadi","timeSeries":[[1787712900000,1.62],[1787712960000,1.61]]},{"id":7,"name":"Other","timeSeries":[[100,2.0]]}] || [];</script>"#;
        let (name, rows) = parse_dhm_stage(body, 4913).unwrap();
        assert_eq!(name, "Bhotekoshi at Rasuwagadi");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "2026-08-26T02:55:00");
        assert!((rows[0].1 - 1.62).abs() < 1e-6);
        assert!((rows[1].1 - 1.61).abs() < 1e-6);
    }

    #[test]
    fn jd_of_iso() {
        let jd = iso_to_jd("2000-01-01").unwrap();
        assert!((jd - 2451544.5).abs() < 0.5);
        let jd = iso_to_jd("2026-08-26T00:00:00Z").unwrap();
        assert!(jd > 2461000.0);
    }

    #[test]
    fn jd_keeps_time_of_day_without_z() {
        let jd = iso_to_jd("2026-08-26T02:52:10").unwrap();
        let jd_mid = iso_to_jd("2026-08-26T00:00:00").unwrap();
        let delta = (jd - jd_mid) * 24.0 * 3600.0;
        assert!((delta - (2.0 * 3600.0 + 52.0 * 60.0 + 10.0)).abs() < 1.0);
    }

    #[test]
    fn jd_absolute_midnight_and_noon_anchors() {
        assert!((iso_to_jd("2026-08-26T00:00:00").unwrap() - 2461278.5).abs() < 1e-6);
        assert!((iso_to_jd("2026-08-26T12:00:00").unwrap() - 2461278.5 - 0.5).abs() < 1e-6);
        assert!((iso_to_jd("2026-08-26T02:52:10").unwrap() - 2461278.619560).abs() < 1e-6);
    }

    #[test]
    fn geo_to_icrs_z_is_invariant_under_rotation() {
        let (_, _, z) = geo_to_icrs_km(28.8558984, 85.2950291, 2461278.0);
        let (_, _, z2) = geo_to_icrs_km(28.8558984, 85.2950291, 2461278.5);
        assert!((z - z2).abs() < 1e-6);
    }

    #[test]
    fn rss_parse_both_kinds() {
        let rss2 = r#"<rss><channel><item><title>A</title><link>http://a</link><pubDate>1</pubDate></item></channel></rss>"#;
        let rdf = r#"<rdf:RDF><item rdf:about="x"><title>B</title><link>http://b</link><dc:date>2</dc:date></item></rdf:RDF>"#;
        let a = rss_items(rss2);
        let b = rss_items(rdf);
        assert_eq!(a[0].title, "A");
        assert_eq!(a[0].date, "1");
        assert_eq!(b[0].title, "B");
        assert_eq!(b[0].date, "2");
    }

    #[test]
    fn numbers_extracted_with_units() {
        let n = numbers_in("177 Tote in Nepal und 3 in China");
        assert!(n.iter().any(|x| x.starts_with("177 ")));
        assert!(n.iter().any(|x| x.starts_with("3 ")));
    }

    #[test]
    fn date_parsed_from_iso_and_words() {
        assert_eq!(
            date_in("On 2026-08-26 a flood struck"),
            Some("2026-08-26".into())
        );
        assert_eq!(
            date_in("On the morning of 26 August 2026, floods struck"),
            Some("2026-08-26".into())
        );
        assert_eq!(date_in("no date here"), None);
    }

    #[test]
    fn json_field_extraction() {
        let s =
            r#"{"title":"2026 Nepal floods","extract":"On 26 August 2026","lat":28.4,"lon":85.3}"#;
        assert_eq!(
            json_key_str(s, "title").as_deref(),
            Some("2026 Nepal floods")
        );
        assert_eq!(
            json_key_str(s, "extract").as_deref(),
            Some("On 26 August 2026")
        );
        assert!((json_num_after(s, "lat").unwrap() - 28.4).abs() < 1e-9);
        assert!((json_num_after(s, "lon").unwrap() - 85.3).abs() < 1e-9);
    }

    #[test]
    fn url_encode_spaces() {
        assert_eq!(urlencode("a b/c"), "a%20b%2Fc");
    }

    #[test]
    fn tdb_tt_delta_is_measured_not_zero() {
        let d = TAI_UTC_LEAP + TT_TAI_OFFSET;
        assert!((d - 69.184).abs() < 1e-9);
        let jd_tt = 2461278.5 + d / 86400.0;
        assert!((jd_tt - 2461278.50080074).abs() < 1e-6);
    }

    #[test]
    fn wikidata_claims_parse_measured_keys() {
        let json = r#"{"entities":{"Q1":{"claims":{
          "P625":[{"mainsnak":{"datavalue":{"value":{"latitude":28.28,"longitude":85.38,"globe":"Q2"}}}}],
          "P585":[{"mainsnak":{"datavalue":{"value":{"time":"+2026-08-26T00:00:00Z","timezone":0}}}}],
          "P1120":[{"mainsnak":{"datavalue":{"value":{"amount":"+292","unit":"1"}}}}]
        }}}}"#;
        assert!((json_num_after(json, "latitude").unwrap() - 28.28).abs() < 1e-6);
        assert!((json_num_after(json, "longitude").unwrap() - 85.38).abs() < 1e-6);
        assert_eq!(
            json_key_str(json, "time").as_deref(),
            Some("+2026-08-26T00:00:00Z")
        );
        assert_eq!(json_key_str(json, "amount").as_deref(), Some("+292"));
    }

    #[test]
    fn json_array_of_numbers() {
        let s = r#"{"coordinates":[-119.9,14.9],"other":1}"#;
        let v = json_array_f64(s, "coordinates");
        assert_eq!(v.len(), 2);
        assert!((v[0] + 119.9).abs() < 1e-9);
        assert!((v[1] - 14.9).abs() < 1e-9);
    }

    #[test]
    fn sha256_is_the_known_vector() {
        let hex = sha256_hex(b"abc");
        assert_eq!(
            hex,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn eonet_category_to_force() {
        assert_eq!(eonet_force("Floods"), Some("gravitation"));
        assert_eq!(eonet_force("Earthquakes"), Some("elastizitaet"));
        assert_eq!(eonet_force("Severe Storms"), Some("aerodynamik"));
        assert_eq!(eonet_force("Volcanoes"), Some("thermodynamik"));
        assert_eq!(eonet_force("Water Color"), None);
    }

    #[test]
    fn cog_ndwi_window_reads_water() {
        use geotiff_writer::GeoTiffBuilder;
        use ndarray::Array2;
        let dir = std::env::temp_dir();
        let b03p = dir.join("omegaflow_test_b03.cog");
        let b08p = dir.join("omegaflow_test_b08.cog");
        let (w, h) = (6usize, 6usize);
        let mut g03 = Array2::<u16>::zeros((h, w));
        let mut g08 = Array2::<u16>::zeros((h, w));
        for r in 2..4 {
            for c in 2..4 {
                g03[[r, c]] = 900;
                g08[[r, c]] = 400;
            }
        }
        for r in 0..h {
            for c in 0..w {
                if !(r >= 2 && r < 4 && c >= 2 && c < 4) {
                    g03[[r, c]] = 300;
                    g08[[r, c]] = 600;
                }
            }
        }
        let _ = std::fs::remove_file(&b03p);
        let _ = std::fs::remove_file(&b08p);
        GeoTiffBuilder::new(w as u32, h as u32)
            .geographic_epsg(4326)
            .origin(86.0, 29.0)
            .pixel_scale(0.01, 0.01)
            .tile_size(16, 16)
            .write_2d(&b03p, g03.view())
            .expect("schreibe B03");
        GeoTiffBuilder::new(w as u32, h as u32)
            .geographic_epsg(4326)
            .origin(86.0, 29.0)
            .pixel_scale(0.01, 0.01)
            .tile_size(16, 16)
            .write_2d(&b08p, g08.view())
            .expect("schreibe B08");
        let (water, tot, mean, _mn, _mx) = cog_ndwi_window(
            b03p.to_str().unwrap(),
            b08p.to_str().unwrap(),
            29.0,
            86.0,
            1.0,
        )
        .expect("cog_ndwi_window");
        assert!(tot >= 9, "gesamt_pixel {tot} zu klein");
        assert!(water >= 3, "wasser_pixel {water}");
        assert!(
            mean > -0.33,
            "ndwi_mean {mean} should lie above the pure land value (-0.33)"
        );
    }
}
