use omegaflow::json::{jstr, parse_json, JsonVal};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

const DEFAULT_TOP: usize = 8;
const MAX_LINE: usize = 400;
const MAX_SOURCE: usize = 180;

struct Kws {
    kind: &'static str,
    terms: &'static [&'static str],
}

const MATRIX: &[Kws] = &[
    Kws {
        kind: "measure",
        terms: &[
            "detect",
            "detection",
            "evidence",
            "signature",
            "significance",
            "retrieval",
            "abundance",
        ],
    },
    Kws {
        kind: "geometry",
        terms: &["transmission", "transit", "eclipse", "emission", "dayside"],
    },
    Kws {
        kind: "shape",
        terms: &["featureless", "flat", "haze", "cloud"],
    },
    Kws {
        kind: "water",
        terms: &["water"],
    },
    Kws {
        kind: "methane",
        terms: &["methane"],
    },
    Kws {
        kind: "species",
        terms: &[
            "sodium",
            "potassium",
            "titanium",
            "vanadium",
            "sulfur",
            "sulphur",
            "hydrogen sulfide",
            "hydrogen cyanide",
            "ammonia",
            "carbon dioxide",
            "carbon monoxide",
            "carbonyl sulfide",
            "helium",
            "rayleigh",
            "alkali",
        ],
    },
    Kws {
        kind: "instrument",
        terms: &[
            "wfc3", "stis", "g280", "g141", "spitzer", "jwst", "nirspec", "hubble", "hst", "grism",
            "uv",
        ],
    },
    Kws {
        kind: "upper-limit",
        terms: &["upper limit", "upper limits"],
    },
    Kws {
        kind: "consistency",
        terms: &["consistent with", "inconsistent with"],
    },
];

const TOKEN_PHRASES: &[&str] = &[
    "h2o", "ch4", "co2", "so2", "nh3", "h2s", "hcn", "ocs", "sio", "tio", "vo", "na i", "k i",
];

fn lower_tokens(s: &str) -> Vec<String> {
    s.to_ascii_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

fn phrase_hit(tokens: &[String], phrase: &str) -> bool {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    if words.len() > tokens.len() {
        return false;
    }
    tokens
        .windows(words.len())
        .any(|w| w.iter().zip(words.iter()).all(|(a, b)| a.as_str() == *b))
}

fn substring_hit(sentence: &str) -> bool {
    MATRIX
        .iter()
        .any(|g| g.terms.iter().any(|t| sentence.contains(t)))
}

fn has_sigma_claim(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let mut j = i + 1;
            let mut gap = 0;
            while j < chars.len() && gap <= 4 {
                let c = chars[j];
                if c == 'σ' {
                    return true;
                }
                if c == 's'
                    && j + 4 < chars.len()
                    && chars[j + 1] == 'i'
                    && chars[j + 2] == 'g'
                    && chars[j + 3] == 'm'
                    && chars[j + 4] == 'a'
                {
                    return true;
                }
                if c.is_ascii_whitespace() || matches!(c, '-' | '–' | '.' | ',' | '×' | '·') {
                    gap += 1;
                    j += 1;
                } else {
                    break;
                }
            }
        }
        i += 1;
    }
    false
}

fn evidence_hit(sentence: &str, extra: &[String]) -> bool {
    let lower = sentence.to_ascii_lowercase();
    let tokens = lower_tokens(&lower);
    let token = TOKEN_PHRASES.iter().any(|p| phrase_hit(&tokens, p));
    let base = substring_hit(&lower);
    let xtra = extra
        .iter()
        .any(|t| !t.is_empty() && lower.contains(&t.to_ascii_lowercase()));
    base || token || xtra || has_sigma_claim(&lower)
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
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

fn collapse_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn split_sentences(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        cur.push(c);
        if matches!(c, '.' | '!' | '?') {
            if chars.get(i + 1).map(|n| n.is_whitespace()).unwrap_or(false) {
                let s = cur.trim().to_string();
                if !s.is_empty() {
                    out.push(s);
                }
                cur.clear();
                i += 1;
                while i < chars.len() && chars[i].is_whitespace() {
                    i += 1;
                }
                continue;
            }
        }
        i += 1;
    }
    let tail = cur.trim().to_string();
    if !tail.is_empty() {
        out.push(tail);
    }
    out
}

fn evidence_lines(text: &str, extra: &[String]) -> Vec<String> {
    let flat = collapse_ws(&strip_tags(text));
    let mut keep: Vec<String> = Vec::new();
    for sentence in split_sentences(&flat) {
        if evidence_hit(&sentence, extra) && !keep.contains(&sentence) {
            keep.push(sentence);
        }
    }
    keep
}

fn clip(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        return s.to_string();
    }
    let head: String = chars[..max].iter().collect();
    format!("{}...", head)
}

struct Target {
    host: String,
    facility: Option<String>,
    class: Option<String>,
    detection: Option<String>,
    source: Option<String>,
    species: Vec<String>,
    bibcodes: Vec<String>,
}

impl Target {
    fn new(host: String) -> Target {
        Target {
            host,
            facility: None,
            class: None,
            detection: None,
            source: None,
            species: Vec::new(),
            bibcodes: Vec::new(),
        }
    }
}

fn push_str_unique(v: &mut Vec<String>, x: &str) {
    if !v.iter().any(|s| s == x) {
        v.push(x.to_string());
    }
}

fn set_field(field: &mut Option<String>, value: Option<String>) {
    if field.is_none() {
        if let Some(v) = value {
            if !v.is_empty() {
                *field = Some(v);
            }
        }
    }
}

fn arr_strs(row: &JsonVal, key: &str) -> Vec<String> {
    let JsonVal::Obj(map) = row else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(items)) = map.get(key) else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|v| match v {
            JsonVal::Str(s) => Some(s.clone()),
            _ => None,
        })
        .collect()
}

fn bibcode_tokens(s: &str, out: &mut Vec<String>) {
    for tok in s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '.')) {
        let tok = tok.trim();
        if tok.len() < 15 {
            continue;
        }
        let b = tok.as_bytes();
        if b.len() < 5 {
            continue;
        }
        if !b[..4].iter().all(|c| c.is_ascii_digit()) {
            continue;
        }
        if !b[b.len() - 1].is_ascii_alphabetic() {
            continue;
        }
        if tok.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        push_str_unique(out, tok);
    }
}

fn host_of(row: &JsonVal) -> Option<String> {
    jstr(row, "hostname").or_else(|| jstr(row, "host"))
}

fn read_census_file(path: &str, targets: &mut HashMap<String, Target>) -> Result<usize, String> {
    let body = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let root = parse_json(&body).ok_or_else(|| format!("{path}: json absent"))?;
    let JsonVal::Arr(rows) = &root else {
        return Err(format!("{path}: root is not an array"));
    };
    let mut named = 0usize;
    for row in rows {
        let Some(host) = host_of(row) else {
            continue;
        };
        named += 1;
        let t = targets
            .entry(host.clone())
            .or_insert_with(|| Target::new(host));
        if jstr(row, "class").is_some() {
            set_field(&mut t.class, jstr(row, "class"));
            set_field(&mut t.source, jstr(row, "note"));
            continue;
        }
        if jstr(row, "hostname").is_some() {
            set_field(&mut t.facility, jstr(row, "facility"));
            set_field(&mut t.detection, jstr(row, "detection"));
            set_field(&mut t.source, jstr(row, "detection_source"));
            if t.source.is_none() {
                set_field(&mut t.source, jstr(row, "note"));
            }
            if let Some(b) = jstr(row, "primary_bibcode") {
                push_str_unique(&mut t.bibcodes, &b);
            }
            let mut joined = String::new();
            for f in ["detection_source", "note"] {
                if let Some(v) = jstr(row, f) {
                    if !joined.is_empty() {
                        joined.push(' ');
                    }
                    joined.push_str(&v);
                }
            }
            if !joined.is_empty() {
                bibcode_tokens(&joined, &mut t.bibcodes);
            }
            for s in arr_strs(row, "species") {
                push_str_unique(&mut t.species, &s);
            }
            continue;
        }
        if let Some(b) = jstr(row, "bibcode") {
            push_str_unique(&mut t.bibcodes, &b);
        }
        if let Some(s) = jstr(row, "species") {
            push_str_unique(&mut t.species, &s);
        }
    }
    Ok(named)
}

fn emit_lines(prefix: &str, lines: &[String], top: usize) {
    if lines.is_empty() {
        println!("{prefix}(keine Treffer)");
        return;
    }
    for s in lines.iter().take(top) {
        println!("{prefix}- {}", clip(s, MAX_LINE));
    }
}

fn file_evidence(dir: &str, bibcode: &str, extra: &[String], top: usize) {
    let p = format!("{dir}/{bibcode}.txt");
    match fs::read_to_string(&p) {
        Err(_) => println!("    absent — {dir} carries no {bibcode}.txt"),
        Ok(txt) => emit_lines("    ", &evidence_lines(&txt, extra), top),
    }
}

fn print_target(t: &Target, dir: &str, extra: &[String], top: usize) {
    println!("== {} ==", t.host);
    if let Some(f) = &t.facility {
        println!("  facility: {}", f);
    }
    if let Some(c) = &t.class {
        println!("  class: {}", c);
    }
    if let Some(d) = &t.detection {
        println!("  detection: {}", d);
    }
    if let Some(s) = &t.source {
        println!("  source: {}", clip(s, MAX_SOURCE));
    }
    if !t.species.is_empty() {
        println!("  species: {}", t.species.join(", "));
    }
    if t.bibcodes.is_empty() {
        println!("  no bibcode registered — the census names no abstract for this host");
        return;
    }
    for b in &t.bibcodes {
        println!("  bibcode {}:", b);
        file_evidence(dir, b, extra, top);
    }
}

fn list_matrix() {
    for g in MATRIX {
        println!("{:14} {}", g.kind, g.terms.join(" | "));
    }
    println!("{:14} {}", "token", TOKEN_PHRASES.join(" | "));
    println!(
        "{:14} digit<gap<=4>sigma|σ — a significance claim (fixed rule)",
        "sigma"
    );
}

fn run_one(bibcode: &str, extra: &[String], top: usize) -> i32 {
    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_err() {
        eprintln!("absent — stdin carried no text");
        return 1;
    }
    println!("== {} ==", bibcode);
    emit_lines("  ", &evidence_lines(&buf, extra), top);
    0
}

fn run_scan(dir: &str, bib: Option<&str>, extra: &[String], top: usize) -> i32 {
    if !Path::new(dir).is_dir() {
        eprintln!("absent — the abstract directory is no directory: {dir}");
        return 1;
    }
    let mut stems: Vec<String> = Vec::new();
    if let Some(b) = bib {
        stems.push(b.to_string());
    } else {
        let rd = fs::read_dir(dir);
        match rd {
            Err(_) => {
                eprintln!("absent — the abstract directory is not readable: {dir}");
                return 1;
            }
            Ok(entries) => {
                for e in entries.flatten() {
                    let p = e.path();
                    if p.extension().map(|x| x == "txt").unwrap_or(false) {
                        if let Some(stem) = p.file_stem() {
                            stems.push(stem.to_string_lossy().to_string());
                        }
                    }
                }
                stems.sort();
            }
        }
    }
    if stems.is_empty() {
        eprintln!("absent — the abstract directory carries no *.txt: {dir}");
        return 1;
    }
    for stem in stems {
        println!("== {} ==", stem);
        file_evidence(dir, &stem, extra, top);
    }
    0
}

fn run_census(dir: &str, files: &[String], extra: &[String], top: usize) -> i32 {
    if !Path::new(dir).is_dir() {
        eprintln!("absent — the abstract directory is no directory: {dir}");
        return 1;
    }
    let mut targets: HashMap<String, Target> = HashMap::new();
    for f in files {
        match read_census_file(f, &mut targets) {
            Err(msg) => {
                eprintln!("absent — census not read: {msg}");
                return 1;
            }
            Ok(named) => {
                if named == 0 {
                    eprintln!("absent — the census row set is empty: {f}");
                    return 1;
                }
            }
        }
    }
    let mut hosts: Vec<String> = targets.keys().cloned().collect();
    hosts.sort();
    for h in hosts {
        let t = targets.get(&h).expect("the host row is present");
        let mut bibcodes = t.bibcodes.clone();
        bibcodes.sort();
        let mut tt = Target::new(h.clone());
        tt.facility = t.facility.clone();
        tt.class = t.class.clone();
        tt.detection = t.detection.clone();
        tt.source = t.source.clone();
        tt.species = t.species.clone();
        tt.bibcodes = bibcodes;
        print_target(&tt, dir, extra, top);
    }
    0
}

fn usage() {
    eprintln!(
        "usage: evidence_sieve --list\n\
         \x20      evidence_sieve one <bibcode> [--term WORD] [--top N]   — abstract on stdin\n\
         \x20      evidence_sieve scan <abs-dir> [--bib BIBCODE] [--term WORD] [--top N]\n\
         \x20      evidence_sieve census --abs-dir <abs-dir> --census <census.json>... [--term WORD] [--top N]"
    );
}

struct Opts {
    abs_dir: Option<String>,
    bib: Option<String>,
    census: Vec<String>,
    terms: Vec<String>,
    top: usize,
}

fn parse_opts(args: &[String]) -> Result<(Opts, Vec<String>), ()> {
    let mut o = Opts {
        abs_dir: None,
        bib: None,
        census: Vec::new(),
        terms: Vec::new(),
        top: DEFAULT_TOP,
    };
    let mut pos: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--abs-dir" => {
                let v = args.get(i + 1).ok_or(())?;
                o.abs_dir = Some(v.clone());
                i += 2;
            }
            "--bib" => {
                let v = args.get(i + 1).ok_or(())?;
                o.bib = Some(v.clone());
                i += 2;
            }
            "--census" => {
                let v = args.get(i + 1).ok_or(())?;
                o.census.push(v.clone());
                i += 2;
            }
            "--term" => {
                let v = args.get(i + 1).ok_or(())?;
                o.terms.push(v.clone());
                i += 2;
            }
            "--top" => {
                let v = args.get(i + 1).ok_or(())?;
                o.top = v.parse().ok().filter(|n| *n > 0).ok_or(())?;
                i += 2;
            }
            s if s.starts_with("--") => {
                return Err(());
            }
            s => {
                pos.push(s.to_string());
                i += 1;
            }
        }
    }
    Ok((o, pos))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--list") {
        list_matrix();
        return;
    }
    let (o, pos) = match parse_opts(&args) {
        Ok(p) => p,
        Err(_) => {
            usage();
            std::process::exit(2);
        }
    };
    if pos.is_empty() {
        usage();
        std::process::exit(2);
    }
    let code = match pos[0].as_str() {
        "one" => {
            let bib = pos.get(1).or_else(|| o.bib.as_ref()).map(|s| s.as_str());
            match bib {
                Some(b) => run_one(b, &o.terms, o.top),
                None => {
                    usage();
                    2
                }
            }
        }
        "scan" => {
            let dir = pos.get(1).or_else(|| o.abs_dir.as_ref());
            match dir {
                Some(d) => run_scan(d, o.bib.as_deref(), &o.terms, o.top),
                None => {
                    usage();
                    2
                }
            }
        }
        "census" => {
            if o.census.is_empty() || o.abs_dir.is_none() {
                usage();
                2
            } else {
                run_census(
                    o.abs_dir.as_deref().expect("abs_dir present"),
                    &o.census,
                    &o.terms,
                    o.top,
                )
            }
        }
        _ => {
            usage();
            2
        }
    };
    if code != 0 {
        std::process::exit(if code == 1 { 1 } else { 2 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_hits_positive_sentences() {
        assert!(evidence_hit("We detect water in the atmosphere.", &[]));
        assert!(evidence_hit("The spectrum is flat and featureless.", &[]));
        assert!(evidence_hit("A methane abundance is retrieved.", &[]));
        assert!(evidence_hit("Na I absorption at 0.589 um is present.", &[]));
    }

    #[test]
    fn matrix_hits_absence_sentences() {
        assert!(evidence_hit("We find no evidence of clouds.", &[]));
        assert!(evidence_hit("The upper limit on H2O is 3 ppm.", &[]));
        assert!(evidence_hit("No signature of TiO is seen.", &[]));
    }

    #[test]
    fn sigma_claims_are_recognized() {
        assert!(evidence_hit("H2O is detected at 4.3 sigma.", &[]));
        assert!(evidence_hit("The detection reaches 5σ significance.", &[]));
        assert!(!evidence_hit("The weather improved.", &[]));
        assert!(!evidence_hit("We saw sigma waves twice.", &[]));
    }

    #[test]
    fn extra_terms_join_the_matrix() {
        let extra = vec!["disequilibrium".to_string()];
        assert!(evidence_hit("Disequilibrium chemistry is favored.", &extra));
        assert!(!evidence_hit("Disequilibrium chemistry is favored.", &[]));
    }

    #[test]
    fn token_terms_do_not_fire_inside_words() {
        assert!(!evidence_hit(
            "The planet-to-star flux ratio is measured.",
            &[]
        ));
        assert!(!evidence_hit("We favor scenario A.", &[]));
        assert!(!evidence_hit("We work in the field.", &[]));
        assert!(evidence_hit("TiO and VO are present.", &[]));
        assert!(evidence_hit("Na I absorption at 0.589 um.", &[]));
    }

    #[test]
    fn case_does_not_matter() {
        assert!(evidence_hit("WATER WAS DETECTED", &[]));
        assert!(evidence_hit("water was detected", &[]));
    }

    #[test]
    fn sentence_split_matches_the_naive_python_split() {
        let text = "First sentence. Second et al. 2020 report. Third?";
        let s = split_sentences(&collapse_ws(text));
        assert_eq!(s[0], "First sentence.");
        assert_eq!(s[1], "Second et al.");
        assert_eq!(s[2], "2020 report.");
        assert_eq!(s[3], "Third?");
    }

    #[test]
    fn tags_and_wrapping_are_stripped() {
        let text = "<p>The H2O detection stands.\n</p><p>No CH4.</p>";
        let lines = evidence_lines(text, &[]);
        assert!(lines
            .iter()
            .any(|l| l.starts_with("The H2O detection stands.")));
        assert!(lines.iter().any(|l| l.starts_with("No CH4.")));
    }

    #[test]
    fn sentences_are_deduplicated() {
        let text = "Water is seen. Water is seen. No clouds.";
        let lines = evidence_lines(text, &[]);
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn bibcode_tokens_find_ads_codes() {
        let mut out = Vec::new();
        bibcode_tokens(
            "ADS 2016ApJ...820...99T; HST eclipse only (2018AJ....156...17K)",
            &mut out,
        );
        assert!(out.contains(&"2016ApJ...820...99T".to_string()));
        assert!(out.contains(&"2018AJ....156...17K".to_string()));
        assert!(!out.contains(&"ADS".to_string()));
        assert!(!out.contains(&"HST".to_string()));
    }

    #[test]
    fn census_host_census_and_seed_merge() {
        let dir = std::env::temp_dir();
        let host_census = dir.join("evsieve_host_test.json");
        let seed = dir.join("evsieve_seed_test.json");
        fs::write(
            &host_census,
            r#"[{"hostname":"WASP-39","class":"detection"},
                {"hostname":"GJ 1132","class":"non_detection"}]"#,
        )
        .expect("write host census fixture");
        fs::write(
            &seed,
            r#"[{"host":"WASP-39","species":"CO2","bibcode":"2023Natur.614..664A"},
                {"host":"WASP-39","species":"H2O","bibcode":"2023Natur.614..664A"}]"#,
        )
        .expect("write seed fixture");
        let mut targets: HashMap<String, Target> = HashMap::new();
        read_census_file(&host_census.to_string_lossy(), &mut targets)
            .expect("read host census fixture");
        read_census_file(&seed.to_string_lossy(), &mut targets).expect("read seed fixture");
        let t = targets.get("WASP-39").expect("the merged host row");
        assert_eq!(t.class.as_deref(), Some("detection"));
        assert_eq!(t.bibcodes, vec!["2023Natur.614..664A".to_string()]);
        assert_eq!(t.species, vec!["CO2".to_string(), "H2O".to_string()]);
        let g = targets.get("GJ 1132").expect("the non-detection row");
        assert_eq!(g.class.as_deref(), Some("non_detection"));
        fs::remove_file(&host_census).expect("remove fixture");
        fs::remove_file(&seed).expect("remove fixture");
    }
}
