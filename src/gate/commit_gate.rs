use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::archivar::units::{allowed_units_for_force, normalize_unit};
use crate::force::force_id_of;
use crate::json::{jstr, parse_json, JsonVal};

pub const FORCE_NAMES: [&str; 9] = [
    "em",
    "gravity",
    "acoustic",
    "seismic-body",
    "seismic-surface",
    "thermal",
    "diffusion",
    "advective",
    "electric",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Home {
    English,
    German,
}

const DE_DETERMINERS: [&str; 8] = ["der", "die", "das", "ein", "eine", "dem", "den", "des"];

const DE_WORKS: [&str; 4] = [
    "the-counter-slope",
    "die-vier-schilde",
    "der-paradigmenwechsel",
    "kybernetische-astrophysik",
];

const DE_CONTENT_TITLES: [&str; 12] = [
    "kybernetische",
    "paradigmen",
    "astrophysik",
    "kausalpfeil",
    "instrument",
    "spektrale",
    "protokoll",
    "kuratierung",
    "korrelations",
    "benennung",
    "exzellenz",
    "doku",
];

const DE_FUNCTION_WORDS: [&str; 34] = [
    "der", "die", "das", "und", "nicht", "eine", "ist", "den", "dem", "des", "ein", "ich", "du",
    "sie", "es", "zu", "von", "mit", "fuer", "für", "auf", "wird", "sind", "war", "als", "auch",
    "im", "am", "sich", "nur", "noch", "ueber", "über", "bei",
];

const EN_FUNCTION_WORDS: [&str; 30] = [
    "the", "and", "of", "to", "in", "is", "was", "that", "for", "with", "on", "at", "by", "from",
    "it", "not", "are", "as", "this", "be", "been", "will", "would", "can", "there", "has", "have",
    "which", "but", "about",
];

fn classify_home(path: &str) -> Option<Home> {
    let lower = path.to_lowercase();
    if is_code_path(&lower) {
        return Some(Home::English);
    }
    if lower.contains("docs/paper/")
        || lower.contains("docs/reference/")
        || lower.ends_with("readme.md")
    {
        return Some(Home::English);
    }
    if lower.contains("docs/handover/")
        || lower.contains("docs/surveys/")
        || lower.contains("docs/auftrag/")
        || lower.contains("docs/blatt/")
        || lower.contains("status/")
        || lower.ends_with("todo.md")
    {
        return Some(Home::German);
    }
    if lower.contains("docs/concepts/") {
        return Some(if german_concept_title(&lower) {
            Home::German
        } else {
            Home::English
        });
    }
    None
}

fn german_concept_title(path: &str) -> bool {
    let slug = path
        .trim_end_matches(".md")
        .rsplit('/')
        .next()
        .unwrap_or(path);
    if DE_WORKS.iter().any(|w| slug == *w) {
        return true;
    }
    let token = slug.split('-').next().unwrap_or("");
    let first_is_de = DE_DETERMINERS.iter().any(|d| *d == token);
    if first_is_de {
        return true;
    }
    let mut parts = slug.split(['-', ' ']);
    if parts.any(|w| DE_CONTENT_TITLES.iter().any(|d| *d == w)) {
        return true;
    }
    slug.contains(['ä', 'ö', 'ü', 'ß'])
}

fn is_code_path(path: &str) -> bool {
    path.ends_with(".rs")
        || path.ends_with(".js")
        || path.ends_with(".mjs")
        || path.ends_with(".wgsl")
        || path.ends_with(".sh")
        || path.ends_with(".toml")
        || path.ends_with(".yml")
        || path.ends_with(".yaml")
        || path.ends_with(".html")
        || path.ends_with(".φ")
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Severity {
    Hard,
    Soft,
}

#[derive(Clone, Debug)]
pub struct Verdict {
    pub severity: Severity,
    pub rule: String,
    pub quote: String,
    pub feedback: String,
}

#[derive(Clone, Debug)]
pub struct RegisterValue {
    pub anchor: String,
    pub value: f64,
    pub unit: String,
}

pub struct Gate {
    pub force_unit_pairs: HashSet<(String, String)>,
    pub unit_tokens: Vec<&'static str>,
    pub forbidden: Vec<&'static str>,
    pub speculation: Vec<&'static str>,
    pub zero_decl: Vec<&'static str>,
    pub register: Vec<RegisterValue>,
    pub learned_rules: Vec<String>,
    pub ledger_path: String,
    pub session: String,
    pub violations: u64,
}

impl Gate {
    pub fn new(session: &str, ledger_path: &str) -> Gate {
        let mut g = Gate {
            force_unit_pairs: HashSet::new(),
            unit_tokens: vec![
                "km/s", "m/s", "m/s²", "m/s2", "km/h", "m/s²", "w/m²", "w/m2", "nt", "hz", "khz",
                "mhz", "ghz", "k", "°c", "hpa", "pa", "sfu", "au", "mag", "s", "m", "km", "cm",
                "mm", "j", "w", "mw", "v", "a", "t", "µt", "ut", "μt", "m²", "m2", "knot", "kt",
                "gal", "mgal", "ev", "jy", "mjy", "pc", "kwh", "mv/m", "v/m",
            ],
            forbidden: vec![
                "failed", "error", "cannot", "crash", "secret", "fallback", "expected", "must",
                "should", "default",
            ],
            speculation: vec![
                "wahrscheinlich",
                "vermutlich",
                "ich vermute",
                "könnte sein",
                "könnte es sein",
                "möglicherweise",
                "vielleicht",
                "ich schätze",
                "ich glaube",
                "ich denke",
                "probably",
                "perhaps",
                "maybe",
                "likely",
                "possibly",
                "i guess",
                "i think",
                "i suspect",
                "i assume",
                "it seems",
                "seems like",
            ],
            zero_decl: vec![
                "pending",
                "fehlt",
                "absent",
                "0 honored",
                "null-echt",
                "pad",
            ],
            register: Vec::new(),
            learned_rules: Vec::new(),
            ledger_path: ledger_path.to_string(),
            session: session.to_string(),
            violations: 0,
        };
        for (force, units) in canonical_pairs() {
            g.force_unit_pairs.insert((force, units));
        }
        g.load_ledger();
        g
    }

    pub fn learn_sources(&mut self, content: &str) {
        for line in content.lines() {
            let t = line.trim();
            if !t.starts_with("field ") {
                continue;
            }
            let tokens: Vec<&str> = t.split_whitespace().collect();
            if tokens.len() < 6 {
                continue;
            }
            let force = tokens[4];
            let unit = tokens[5];
            if force_id_of(force).is_none() {
                continue;
            }
            let nu = normalize_unit(unit);
            if nu.is_empty() || nu == "1" {
                continue;
            }
            self.force_unit_pairs.insert((force.to_string(), nu));
        }
    }

    pub fn learn_register(&mut self, root: &str) {
        for dir in ["docs/paper", "docs/concepts", "docs/surveys"] {
            let path = format!("{}/{}", root, dir);
            let entries = match fs::read_dir(&path) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) != Some("md") {
                    continue;
                }
                if let Ok(text) = fs::read_to_string(&p) {
                    scan_register_values(&text, &self.unit_tokens, &mut self.register);
                }
            }
        }
        if let Ok(text) = fs::read_to_string(format!("{}/docs/TODO.md", root)) {
            scan_register_values(&text, &self.unit_tokens, &mut self.register);
        }
    }

    pub fn load_ledger(&mut self) {
        let mut counts: Vec<(String, u64)> = Vec::new();
        if let Ok(text) = fs::read_to_string(&self.ledger_path) {
            for line in text.lines() {
                let rule = line.split('|').next().unwrap_or("").trim().to_lowercase();
                if rule.is_empty() {
                    continue;
                }
                match counts.iter_mut().find(|(r, _)| *r == rule) {
                    Some((_, c)) => *c += 1,
                    None => counts.push((rule, 1)),
                }
            }
        }
        for (rule, count) in counts {
            if count >= 3 {
                self.learned_rules.push(rule);
            }
        }
    }

    fn write_ledger(&self, rule: &str, quote: &str) {
        let clean = quote
            .chars()
            .map(|c| {
                if c == '|' || c == '\n' || c == '\r' {
                    ' '
                } else {
                    c
                }
            })
            .collect::<String>();
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if let Ok(mut f) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ledger_path)
        {
            let _ = writeln!(f, "{} | {} | {} | {}", rule, clean, epoch, self.session);
        }
    }

    pub fn record_violation(&mut self, rule: &str, quote: &str) {
        self.write_ledger(rule, quote);
        self.violations += 1;
    }

    pub fn record_note(&mut self, rule: &str, quote: &str) {
        self.write_ledger(rule, quote);
    }

    pub fn check_text(&mut self, text: &str) -> Option<Verdict> {
        self.find_speculation(text)
            .or_else(|| self.check_zero_fabrication(text))
            .or_else(|| self.check_force_unit(text))
            .or_else(|| self.find_learned(text))
            .or_else(|| self.check_register_numbers(text))
            .or_else(|| self.check_unbacked_claim(text))
    }

    pub fn check_input(&mut self, text: &str) -> Vec<Verdict> {
        let mut findings = Vec::new();
        for f in [
            self.find_speculation(text),
            self.check_zero_fabrication(text),
            self.check_force_unit(text),
            self.find_learned(text),
            self.check_register_numbers(text),
            self.check_unbacked_claim(text),
        ]
        .into_iter()
        .flatten()
        {
            findings.push(Verdict {
                severity: Severity::Soft,
                ..f
            });
            if findings.len() >= 8 {
                break;
            }
        }
        findings
    }

    fn find_speculation(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        for spec in &self.speculation {
            if !word_present(&lower, spec) {
                continue;
            }
            let in_backticks = lower.contains(&format!("`{}`", spec));
            let in_quotes = lower.contains(&format!("\"{}\"", spec))
                || lower.contains(&format!("\u{201C}{}\u{201D}", spec));
            if in_backticks || in_quotes {
                continue;
            }
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "speculation".to_string(),
                feedback: format!(
                    "A = A: the machine does not speculate. \"{}\" is a guess, not a measurement. Name what IS.",
                    spec
                ),
                quote: clip(text, 80),
            });
        }
        None
    }

    fn find_learned(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        for rule in &self.learned_rules {
            if lower.contains(rule.as_str()) {
                return Some(Verdict {
                    severity: Severity::Soft,
                    rule: "learned-rule".to_string(),
                    feedback: format!("the ledger has flagged \"{}\" three times before", rule),
                    quote: clip(text, 80),
                });
            }
        }
        None
    }

    fn check_unbacked_claim(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        let completion = [
            "fertig",
            "befund",
            "erledigt",
            "gelaufen",
            "abgeschlossen",
            "complete",
            "done",
            "verdict",
        ]
        .iter()
        .any(|w| word_present(&lower, w));
        if !completion {
            return None;
        }

        let anchored = ["src/", "tools/", "docs/", "archive/", "phi/"]
            .iter()
            .any(|p| lower.contains(p));
        if anchored {
            return None;
        }
        Some(Verdict {
            severity: Severity::Soft,
            rule: "unbacked-claim".to_string(),
            feedback: "a completion claim needs an anchor: name the path (src/…, docs/…) that backs it in the tree — a commit SHA is not a measurement".to_string(),
            quote: clip(text, 80),
        })
    }

    fn check_zero_fabrication(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if !(bytes[i] as char).is_ascii_digit() {
                i += 1;
                continue;
            }
            let start = i;
            while i < bytes.len() && ((bytes[i] as char).is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            let token = &text[start..i];
            let Ok(value) = token.parse::<f64>() else {
                continue;
            };
            if value != 0.0 {
                continue;
            }
            let tail = &lower[start..];
            let window: String = tail.chars().take(80).collect();
            if self
                .zero_decl
                .iter()
                .any(|d| window.contains(d) || lower.contains(d))
            {
                continue;
            }
            if token.contains('.') {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "zero-fabrication".to_string(),
                    feedback:
                        "0 honored: the value 0.0 was spoken without a declaration (pending/absent). Fabrication suspected."
                            .to_string(),
                    quote: clip(text, 80),
                });
            }
            let mut j = i;
            while j < bytes.len() && (bytes[j] as char) == ' ' {
                j += 1;
            }
            for unit in &self.unit_tokens {
                let raw = unit.to_lowercase();
                let nu = normalize_unit(unit);
                if unit_match_at(&lower, j, &raw).is_some()
                    || (!nu.is_empty() && nu != raw && unit_match_at(&lower, j, &nu).is_some())
                {
                    return Some(Verdict {
                        severity: Severity::Hard,
                        rule: "zero-fabrication".to_string(),
                        feedback: format!(
                            "0 honored: \"0 {}\" was spoken without a declaration (pending/absent). Fabrication suspected.",
                            unit
                        ),
                        quote: clip(text, 80),
                    });
                }
            }
        }
        None
    }

    fn check_force_unit(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        for force in FORCE_NAMES {
            if force == "em"
                && !lower.contains(" em ")
                && !lower.contains(" em,")
                && !lower.contains(" em:")
                && !lower.contains(" em.")
            {
                continue;
            }
            if !word_present(&lower, force) {
                continue;
            }
            for (pos, _) in lower.match_indices(force) {
                let window: String = lower[pos..].chars().take(70).collect();
                for unit in &self.unit_tokens {
                    let nu = normalize_unit(unit);
                    if nu.is_empty() {
                        continue;
                    }
                    if let Some(u_idx) = find_unit(&window, &nu) {
                        let span_start = pos;
                        let span_end = (pos + u_idx + nu.len()).min(lower.len());
                        let span: String = lower[span_start..span_end].to_string();
                        let pair = (force.to_string(), nu);
                        if self.force_unit_pairs.contains(&pair) {
                            continue;
                        }
                        return Some(Verdict {
                            severity: Severity::Hard,
                            rule: "force-unit-gate".to_string(),
                            feedback: format!(
                                "the force \"{}\" paired with the unit \"{}\" is not in the registry ({} carries its own units)",
                                force, unit, force
                            ),
                            quote: clip(&span, 80),
                        });
                    }
                }
            }
        }
        None
    }

    fn check_register_numbers(&self, text: &str) -> Option<Verdict> {
        for (num, unit, anchor) in scan_numbers_with_units(text, &self.unit_tokens) {
            let nu = normalize_unit(&unit);
            let same_unit: Vec<&RegisterValue> =
                self.register.iter().filter(|r| r.unit == nu).collect();
            if same_unit.is_empty() {
                continue;
            }
            let nearest = same_unit
                .iter()
                .min_by(|a, b| {
                    let da = (a.value - num).abs();
                    let db = (b.value - num).abs();
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                })
                .copied();
            let Some(nearest) = nearest else { continue };
            if nearest.value.abs() > 1e-30 {
                let rel = (nearest.value - num).abs() / nearest.value.abs();
                if rel < 0.005 {
                    continue;
                }
            } else if (nearest.value - num).abs() < 1e-9 {
                continue;
            }
            let anchor_words: HashSet<&str> = anchor.split_whitespace().collect();
            for r in &same_unit {
                let rw: HashSet<&str> = r.anchor.split_whitespace().collect();
                let shared = anchor_words.intersection(&rw).count();
                if shared >= 2 {
                    return Some(Verdict {
                        severity: Severity::Hard,
                        rule: "register-contradiction".to_string(),
                        feedback: format!(
                            "the number {} {} contradicts the registered measurement {} {} in the same context",
                            num, unit, r.value, r.unit
                        ),
                        quote: clip(text, 90),
                    });
                }
            }
            return Some(Verdict {
                severity: Severity::Soft,
                rule: "unverified-number".to_string(),
                feedback: format!(
                    "the number {} {} stands in no register entry — pending, not proven",
                    num, unit
                ),
                quote: clip(text, 90),
            });
        }
        None
    }

    pub fn check_tool_call(&mut self, tool: &str, args_json: &str) -> Option<Verdict> {
        if !matches!(tool, "edit" | "write" | "patch" | "multiedit") {
            return None;
        }
        let obj = parse_json(args_json)?;
        let path = jstr(&obj, "filePath")
            .or_else(|| jstr(&obj, "path"))
            .or_else(|| jstr(&obj, "file"))
            .unwrap_or_default();
        let content = jstr(&obj, "newString")
            .or_else(|| jstr(&obj, "content"))
            .or_else(|| jstr(&obj, "text"))
            .or_else(|| jstr(&obj, "file_text"));
        let Some(content) = content else {
            return None;
        };
        let is_code = path.ends_with(".rs")
            || path.ends_with(".js")
            || path.ends_with(".wgsl")
            || path.ends_with(".sh")
            || path.ends_with(".toml")
            || path.ends_with(".yml")
            || path.ends_with(".html")
            || path.ends_with(".φ");
        let root_basename = path.trim_start_matches("./");
        let canonical_root_doc = root_basename.ends_with(".md")
            && !root_basename.contains('/')
            && root_basename != "AGENTS.md"
            && root_basename != "README.md";
        if canonical_root_doc {
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "canonical-doc-home".to_string(),
                feedback: "a root-level markdown document is not a canonical home — the document lives under docs/; the root carries only AGENTS.md, README.md and code".to_string(),
                quote: clip(&path, 90),
            });
        }
        let lower_content = content.to_lowercase();
        for word in [
            "strand",
            "strang",
            "worktree",
            "gate-tragen",
            "leitstelle",
            "branch-marker",
            "omegaflow-strand",
            "zweig-modell",
        ] {
            if lower_content.contains(word) {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "single-path".to_string(),
                    feedback: "the text names a branch/strand/worktree model that does not exist — this system is one path, one line, one truth".to_string(),
                    quote: clip(&content, 90),
                });
            }
        }
        for line in content.lines() {
            let t = line.trim_start();
            if t.starts_with("///") || t.starts_with("//!") {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "docstring".to_string(),
                    feedback: "code is self-documenting — docstrings are dead. Remove the line."
                        .to_string(),
                    quote: clip(t, 80),
                });
            }
            if t.starts_with("field ") {
                let tokens: Vec<&str> = t.split_whitespace().collect();
                if tokens.len() >= 6 {
                    let force = tokens[4];
                    let unit = normalize_unit(tokens[5]);
                    if force_id_of(force).is_some()
                        && !unit.is_empty()
                        && unit != "1"
                        && !self.force_unit_pairs.contains(&(force.to_string(), unit))
                    {
                        return Some(Verdict {
                            severity: Severity::Hard,
                            rule: "force-unit-gate".to_string(),
                            feedback: format!(
                                "field line: the force \"{}\" with the unit \"{}\" is not in the registry",
                                force, tokens[5]
                            ),
                            quote: clip(t, 90),
                        });
                    }
                }
            }
        }
        if is_code {
            let lower = content.to_lowercase();
            if content.contains('ä')
                || content.contains('ö')
                || content.contains('ü')
                || content.contains('ß')
                || lower.contains(" der ")
                || lower.contains(" die ")
                || lower.contains(" das ")
                || lower.contains(" und ")
                || lower.contains(" nicht ")
                || lower.contains(" eine ")
            {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "german-in-code".to_string(),
                    feedback: "the code speaks English — German is the counter-slope of the register and the philosophy, not of code".to_string(),
                    quote: clip(&content, 90),
                });
            }
            if content.contains("unwrap_or(0.0)") || content.contains("unwrap_or(0)") {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "zero-fabrication".to_string(),
                    feedback: "unwrap_or(0.0) is a fabricated zero — the physical value is absent, not zero".to_string(),
                    quote: clip(&content, 90),
                });
            }
            for (marker, hint) in [
                ("unwrap_or_default(", "unwrap_or_default fabricates a default — the value is absent, not defaulted"),
                ("unwrap_or_else(", "unwrap_or_else fabricates a fallback — the value is absent, not derived"),
                ("derive(Default)", "#[derive(Default)] is a fabricated default state — the truth is the query"),
                ("_ => 0,", "\"_ => 0\" writes an unmeasured zero — the value is absent, not zero"),
                ("_ => 0.0", "\"_ => 0.0\" writes an unmeasured zero — the value is absent, not zero"),
                (".max(1)", ".max(1) clamps to a fabricated floor — the floor must come from the measurement"),
            ] {
                if content.contains(marker) {
                    return Some(Verdict {
                        severity: Severity::Hard,
                        rule: "fabrication".to_string(),
                        feedback: hint.to_string(),
                        quote: clip(&content, 90),
                    });
                }
            }
            for marker in [
                "eprintln!(\"",
                "println!(\"",
                "assert!(\"",
                "assert_eq!(\"",
                "panic!(\"",
            ] {
                if let Some(idx) = content.find(marker) {
                    let start = idx + marker.len();
                    let rest = &content[start..];
                    let msg: String = rest.chars().take(120).collect();
                    for bad in &self.forbidden {
                        if word_present(&msg.to_lowercase(), bad) {
                            return Some(Verdict {
                                severity: Severity::Hard,
                                rule: "forbidden-diagnostic".to_string(),
                                feedback: format!(
                                    "the diagnostic carries \"{}\" — diagnostics name what IS",
                                    bad
                                ),
                                quote: clip(&msg, 90),
                            });
                        }
                    }
                }
            }
        }
        if !is_code {
            if let Some(home) = classify_home(&path) {
                if let Some(verdict) = home_drift(&content, home) {
                    return Some(verdict);
                }
            }
        }
        None
    }
}

pub fn home_of(path: &str) -> Option<&'static str> {
    match classify_home(path) {
        Some(Home::German) => Some("deutsch"),
        Some(Home::English) => Some("englisch"),
        None => None,
    }
}

pub fn scan_home(path: &str, content: &str) -> Option<Verdict> {
    let home = classify_home(path)?;
    home_drift(content, home)
}

fn home_drift(content: &str, home: Home) -> Option<Verdict> {
    let body = strip_code_blocks(content);
    let de = count_words(&body, &DE_FUNCTION_WORDS);
    let en = count_words(&body, &EN_FUNCTION_WORDS);
    let floor = 4usize;
    match home {
        Home::German => {
            if en >= floor && en > de {
                Some(Verdict {
                    severity: Severity::Hard,
                    rule: "english-in-german".to_string(),
                    feedback: "this document lives in a German home (register, handover, philosophy) — English-dominant prose is drift, not the measurement".to_string(),
                    quote: clip(&body, 90),
                })
            } else {
                None
            }
        }
        Home::English => {
            if de >= floor && de > en {
                Some(Verdict {
                    severity: Severity::Hard,
                    rule: "german-in-english".to_string(),
                    feedback: "this document lives in an English home (code, paper, spec) — German-dominant prose is drift, not the measurement".to_string(),
                    quote: clip(&body, 90),
                })
            } else {
                None
            }
        }
    }
}

fn strip_code_blocks(content: &str) -> String {
    let mut out = String::new();
    let mut in_fence = false;
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn count_words(hay: &str, words: &[&str]) -> usize {
    let lower = hay.to_lowercase();
    let mut n = 0;
    for w in words {
        if word_present(&lower, w) {
            n += 1;
        }
    }
    n
}

fn canonical_pairs() -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    for name in FORCE_NAMES {
        let Some(id) = force_id_of(name) else {
            continue;
        };
        for unit in allowed_units_for_force(id) {
            let nu = normalize_unit(unit);
            if nu.is_empty() || nu == "1" {
                continue;
            }
            pairs.push((name.to_string(), nu));
        }
    }
    pairs
}

fn word_present(lower: &str, word: &str) -> bool {
    let w = word.to_lowercase();
    if let Some(pos) = lower.find(&w) {
        let before = pos == 0 || {
            let c = lower.as_bytes()[pos - 1] as char;
            !c.is_alphanumeric()
        };
        let after = pos + w.len() >= lower.len() || {
            let c = lower.as_bytes()[pos + w.len()] as char;
            !c.is_alphanumeric()
        };
        before && after
    } else {
        false
    }
}

fn find_unit(window: &str, unit: &str) -> Option<usize> {
    if let Some(pos) = window.find(unit) {
        let before_ok = pos == 0 || {
            let c = window.as_bytes()[pos - 1] as char;
            c == ' ' || c == '=' || c == '(' || c == '[' || c == '/'
        };
        let after_ok = pos + unit.len() >= window.len() || {
            let c = window.as_bytes()[pos + unit.len()] as char;
            !c.is_alphanumeric()
        };
        if before_ok && after_ok {
            return Some(pos);
        }
    }
    None
}

fn clip(text: &str, max: usize) -> String {
    text.chars()
        .take(max)
        .collect::<String>()
        .trim()
        .to_string()
}

fn char_back(text: &str, i: usize, n: usize) -> usize {
    let mut start = i.saturating_sub(n);
    while start > 0 && !text.is_char_boundary(start) {
        start -= 1;
    }
    start
}

fn unit_match_at(text: &str, j: usize, unit_norm: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    let ub = unit_norm.as_bytes();
    if j + ub.len() > bytes.len() {
        return None;
    }
    for (k, ubc) in ub.iter().enumerate() {
        if bytes[j + k].to_ascii_lowercase() != *ubc {
            return None;
        }
    }
    let after = j + ub.len();
    let boundary_ok = after >= bytes.len() || !(bytes[after] as char).is_alphanumeric();
    if boundary_ok {
        Some(after)
    } else {
        None
    }
}

fn scan_register_values(text: &str, units: &[&'static str], out: &mut Vec<RegisterValue>) {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if !(bytes[i] as char).is_ascii_digit() {
            i += 1;
            continue;
        }
        let Some((value, end)) = read_number(text, i) else {
            i += 1;
            continue;
        };
        let mut j = end;
        while j < bytes.len() && (bytes[j] as char) == ' ' {
            j += 1;
        }
        for unit in units {
            let nu = normalize_unit(unit);
            if nu.is_empty() {
                continue;
            }
            if let Some(after) = unit_match_at(text, j, &nu) {
                let anchor_start = char_back(text, i, 40);
                let anchor = text[anchor_start..i]
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                    .collect::<String>();
                out.push(RegisterValue {
                    anchor,
                    value,
                    unit: nu.clone(),
                });
                i = after;
                break;
            }
        }
        i += 1;
    }
}

fn read_number(text: &str, start: usize) -> Option<(f64, usize)> {
    let bytes = text.as_bytes();
    let mut end = start;
    let mut seen_dot = false;
    let mut seen_comma = false;
    while end < bytes.len() {
        let c = bytes[end] as char;
        if c.is_ascii_digit() {
            end += 1;
        } else if c == '.' && !seen_dot && !seen_comma {
            seen_dot = true;
            end += 1;
        } else if c == ',' && !seen_comma && !seen_dot {
            seen_comma = true;
            end += 1;
        } else if (c == 'e' || c == 'E') && end + 1 < bytes.len() {
            let next = bytes[end + 1] as char;
            if next.is_ascii_digit() || next == '+' || next == '-' {
                end += 2;
                while end < bytes.len() && (bytes[end] as char).is_ascii_digit() {
                    end += 1;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if end == start {
        return None;
    }
    let raw = &text[start..end];
    let parsed = if seen_comma {
        raw.replace(',', ".").parse::<f64>().ok()
    } else {
        raw.parse::<f64>().ok()
    };
    parsed.map(|v| (v, end))
}

fn scan_numbers_with_units(text: &str, units: &[&'static str]) -> Vec<(f64, String, String)> {
    let mut hits = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if !(bytes[i] as char).is_ascii_digit() {
            i += 1;
            continue;
        }
        let Some((value, end)) = read_number(text, i) else {
            i += 1;
            continue;
        };
        let mut j = end;
        while j < bytes.len() && (bytes[j] as char) == ' ' {
            j += 1;
        }
        for unit in units {
            let nu = normalize_unit(unit);
            if nu.is_empty() {
                continue;
            }
            if let Some(after) = unit_match_at(text, j, &nu) {
                let anchor_start = char_back(text, i, 40);
                let anchor = text[anchor_start..i]
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                    .collect::<String>();
                hits.push((value, unit.to_string(), anchor));
                i = after;
                break;
            }
        }
        i += 1;
    }
    hits
}

pub fn json_write(v: &JsonVal) -> String {
    match v {
        JsonVal::Null => "null".to_string(),
        JsonVal::Bool(b) => b.to_string(),
        JsonVal::Num(n) => {
            if n.is_finite() {
                format!("{}", n)
            } else {
                "0".to_string()
            }
        }
        JsonVal::Str(s) => {
            let mut out = String::from("\"");
            for c in s.chars() {
                match c {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                    c => out.push(c),
                }
            }
            out.push('"');
            out
        }
        JsonVal::Arr(items) => {
            let parts: Vec<String> = items.iter().map(json_write).collect();
            format!("[{}]", parts.join(","))
        }
        JsonVal::Obj(map) => {
            let mut parts: Vec<String> = Vec::new();
            for (k, val) in map {
                parts.push(format!(
                    "{}:{}",
                    json_write(&JsonVal::Str(k.clone())),
                    json_write(val)
                ));
            }
            format!("{{{}}}", parts.join(","))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_gate() -> Gate {
        let mut g = Gate::new("kalibrier", "/tmp/llm_gate_test_ledger.φ");
        g.learn_sources(
            "field flux density inverse-square em w/m2\nfield speed linear advective m/s\n",
        );
        g
    }

    #[test]
    fn fp_force_unit_mismatch() {
        let mut g = test_gate();
        let v = g.check_text("the signal carries em km/s").unwrap();
        assert_eq!(v.severity, Severity::Hard);
        assert_eq!(v.rule, "force-unit-gate");
    }

    #[test]
    fn fp_force_unit_mismatch_full_sentence() {
        let mut g = test_gate();
        let v = g
            .check_text("the signal carries em km/s and that is the whole story.")
            .unwrap();
        assert_eq!(v.severity, Severity::Hard);
        assert_eq!(v.rule, "force-unit-gate");
    }

    #[test]
    fn input_finds_are_soft_and_collected() {
        let mut g = test_gate();
        let findings = g.check_input("the old doc says 0.0 and em km/s wahrscheinlich");
        assert!(!findings.is_empty());
        assert!(findings.iter().all(|v| v.severity == Severity::Soft));
        assert!(findings.iter().any(|v| v.rule == "zero-fabrication"));
    }

    #[test]
    fn input_clean_text_yields_nothing() {
        let mut g = test_gate();
        assert!(g
            .check_input("the block carries the measured series, nothing else")
            .is_empty());
    }

    #[test]
    fn fp_speculation_actual_guess_blocked() {
        let mut g = test_gate();
        let v = g.check_text("the channel probably sits at 10 m").unwrap();
        assert_eq!(v.rule, "speculation");
    }

    #[test]
    fn fn_speculation_substring_not_blocked() {
        let mut g = test_gate();
        assert!(g
            .check_text("the data are unlikely to arrive, and the likelihood is low")
            .is_none());
    }

    #[test]
    fn fn_speculation_case_insensitive_quote_not_blocked() {
        let mut g = test_gate();
        assert!(g
            .check_text("the register warns that \u{201C}Probably\u{201D} is a guess")
            .is_none());
    }

    #[test]
    fn fp_unbacked_claim_blocked() {
        let mut g = test_gate();
        let v = g.check_text("Fertig. die Arbeit ist getan.").unwrap();
        assert_eq!(v.rule, "unbacked-claim");
        assert_eq!(v.severity, Severity::Soft);
    }

    #[test]
    fn fp_unbacked_claim_commit_sha_not_anchor() {
        let mut g = test_gate();

        let v = g.check_text("Fertig — committe als 59a7062").unwrap();
        assert_eq!(v.rule, "unbacked-claim");
    }

    #[test]
    fn fn_backed_claim_anchored_path_not_blocked() {
        let mut g = test_gate();
        assert!(g
            .check_text("Fertig. tools/src/bin/claim_verify.rs is on main")
            .is_none());
    }

    #[test]
    fn fp_speculation_capitalized_guess_blocked() {
        let mut g = test_gate();
        let v = g.check_text("the channel Probably sits at 10 m").unwrap();
        assert_eq!(v.rule, "speculation");
    }

    #[test]
    fn fn_speculation_rule_named_as_quote_not_blocked() {
        let mut g = test_gate();
        assert!(g
            .check_text(
                "das Wort \"vermutlich\" und `wahrscheinlich` sind Spekulation, keine Messung"
            )
            .is_none());
    }

    #[test]
    fn fp_zero_without_declaration() {
        let mut g = test_gate();
        let v = g
            .check_text("the residual is 0.0 and that is the answer")
            .unwrap();
        assert_eq!(v.rule, "zero-fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_zero_with_pending() {
        let mut g = test_gate();
        assert!(g
            .check_text("the residual is 0.0 — pending, the harvest is absent")
            .is_none());
    }

    #[test]
    fn fn_zero_in_violation_report_with_declaration_later_in_text() {
        let mut g = test_gate();
        assert!(g
            .check_text(
                "Z.245 e.coord.unwrap_or((0.0, 0.0)): bei nur --ort-lat wird die fehlende Koordinate als 0.0 gesetzt statt die Sache als pending zu benennen — die Meldung deklariert den Verstoß im Text"
            )
            .is_none());
    }

    #[test]
    fn fp_zero_without_any_declaration() {
        let mut g = test_gate();
        let v = g.check_text("the value is 0.0 in the record").unwrap();
        assert_eq!(v.rule, "zero-fabrication");
    }

    #[test]
    fn fn_version_number_is_not_zero() {
        let mut g = test_gate();
        assert!(g.check_text("omegaflow v1.0.0 compiles clean").is_none());
    }

    #[test]
    fn fn_count_without_unit_is_no_measurement() {
        let mut g = test_gate();
        assert!(g
            .check_text("cargo check gives 0 Fehler, 0 Warnungen")
            .is_none());
    }

    #[test]
    fn fn_nonzero_number_is_not_zero() {
        let mut g = test_gate();
        assert!(g
            .check_text("the channel sits at 10 m, fully archived")
            .is_none());
        assert!(g
            .check_text("the series reaches 100.0 s without drift")
            .is_none());
    }

    #[test]
    fn fp_zero_with_unit() {
        let mut g = test_gate();
        let v = g.check_text("the anchor rests at 0 m").unwrap();
        assert_eq!(v.rule, "zero-fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_comparison_zero_passes() {
        let mut g = test_gate();
        assert!(g
            .check_text("die Maschine filtert aktuell gm > 0")
            .is_none());
        assert!(g
            .check_text("die Maschine filtert aktuell gm > 0`")
            .is_none());
        assert!(g
            .check_text("der Messwert-fehlt ist pending, nie 0,0")
            .is_none());
        assert!(g.check_text("Keine 0").is_none());
    }

    #[test]
    fn fp_zero_decimal_with_unit() {
        let mut g = test_gate();
        let v = g.check_text("the flux is 0.0 w/m2 here").unwrap();
        assert_eq!(v.rule, "zero-fabrication");
    }

    #[test]
    fn fp_speculation() {
        let mut g = test_gate();
        let v = g
            .check_text("the peak is wahrscheinlich an artifact")
            .unwrap();
        assert_eq!(v.rule, "speculation");
    }

    #[test]
    fn fn_clean_text_passes() {
        let mut g = test_gate();
        assert!(g
            .check_text("the field carries the measured series; the gate holds")
            .is_none());
    }

    #[test]
    fn fn_valid_pair_passes() {
        let mut g = test_gate();
        assert!(g.check_text("the advective field carries m/s").is_none());
    }

    #[test]
    fn forbidden_words_not_flagged_in_text() {
        let mut g = test_gate();
        assert!(g.check_text("the channel default was hit").is_none());
    }

    #[test]
    fn fp_tool_forbidden_diagnostic() {
        let mut g = test_gate();
        let args = r#"{"filePath":"src/x.rs","newString":"eprintln!(\"the fetch failed\")"}"#;
        let v = g.check_tool_call("edit", args).unwrap();
        assert_eq!(v.rule, "forbidden-diagnostic");
    }

    #[test]
    fn fp_tool_docstring() {
        let mut g = test_gate();
        let args = r#"{"filePath":"src/x.rs","newString":"/// a docstring\npub fn f() {}"}"#;
        let v = g.check_tool_call("edit", args).unwrap();
        assert_eq!(v.rule, "docstring");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_german_in_code() {
        let mut g = test_gate();
        let args =
            r#"{"filePath":"src/x.rs","newString":"// die datei trägt den wert\npub fn f() {}"}"#;
        let v = g.check_tool_call("edit", args).unwrap();
        assert_eq!(v.rule, "german-in-code");
    }

    #[test]
    fn fn_tool_german_in_markdown_passes() {
        let mut g = test_gate();
        let args = r#"{"filePath":"docs/handover/handover-2026-09-02-a.md","newString":"die prosa ist der gegenhang"}"#;
        assert!(g.check_tool_call("write", args).is_none());
    }

    #[test]
    fn fp_tool_english_in_german_home() {
        let mut g = test_gate();
        let args = r#"{"filePath":"docs/handover/handover-2026-09-02-b.md","newString":"the prose is the counter slope and it will not be in the register"}"#;
        let v = g.check_tool_call("write", args).unwrap();
        assert_eq!(v.rule, "english-in-german");
    }

    #[test]
    fn fp_tool_german_in_english_home() {
        let mut g = test_gate();
        let args = r#"{"filePath":"docs/concepts/binary-protocol.md","newString":"der datenstrom ist das protokoll und es muss die form tragen"}"#;
        let v = g.check_tool_call("write", args).unwrap();
        assert_eq!(v.rule, "german-in-english");
    }

    #[test]
    fn fn_tool_german_in_german_title_concept_passes() {
        let mut g = test_gate();
        let args = r#"{"filePath":"docs/concepts/die-vier-schilde.md","newString":"die vier schilde halten den gegenhang und die prosa bleibt"}"#;
        assert!(g.check_tool_call("write", args).is_none());
    }

    #[test]
    fn fp_tool_english_in_counter_slope_work() {
        let mut g = test_gate();
        let args = r#"{"filePath":"docs/concepts/the-counter-slope.md","newString":"the counter slope is the instrument of this duty and it will hold"}"#;
        let v = g.check_tool_call("write", args).unwrap();
        assert_eq!(v.rule, "english-in-german");
    }

    #[test]
    fn fn_tool_german_in_counter_slope_work_passes() {
        let mut g = test_gate();
        let args = r#"{"filePath":"docs/concepts/the-counter-slope.md","newString":"die vier schilde halten den gegenhang und die prosa bleibt"}"#;
        assert!(g.check_tool_call("write", args).is_none());
    }

    #[test]
    fn fp_tool_unwrap_zero() {
        let mut g = test_gate();
        let args = r#"{"filePath":"src/x.rs","newString":"let v = val.unwrap_or(0.0);"}"#;
        let v = g.check_tool_call("edit", args).unwrap();
        assert_eq!(v.rule, "zero-fabrication");
    }

    #[test]
    fn fp_tool_fabrication_default() {
        let mut g = test_gate();
        let args = r#"{"filePath":"src/x.rs","newString":"let v = val.unwrap_or_default();"}"#;
        let v = g.check_tool_call("edit", args).unwrap();
        assert_eq!(v.rule, "fabrication");
    }

    #[test]
    fn fp_tool_fabrication_derive_default() {
        let mut g = test_gate();
        let args = r##"{"filePath":"src/x.rs","newString":"#[derive(Default)]\npub struct S;"}"##;
        let v = g.check_tool_call("edit", args).unwrap();
        assert_eq!(v.rule, "fabrication");
    }

    #[test]
    fn fp_tool_fabrication_underscore_zero() {
        let mut g = test_gate();
        let args =
            r#"{"filePath":"src/x.rs","newString":"let x = match o { Some(v) => v, _ => 0, };"}"#;
        let v = g.check_tool_call("edit", args).unwrap();
        assert_eq!(v.rule, "fabrication");
    }

    #[test]
    fn fp_tool_fabrication_max_floor() {
        let mut g = test_gate();
        let args = r#"{"filePath":"src/x.rs","newString":"let n = c.max(1);"}"#;
        let v = g.check_tool_call("edit", args).unwrap();
        assert_eq!(v.rule, "fabrication");
    }

    #[test]
    fn fp_tool_bad_field_line() {
        let mut g = test_gate();
        let args =
            r#"{"filePath":"phi/x.φ","newString":"field wind speed inverse-square em km/s\n"}"#;
        let v = g.check_tool_call("write", args).unwrap();
        assert_eq!(v.rule, "force-unit-gate");
    }

    #[test]
    fn fn_clean_tool_passes() {
        let mut g = test_gate();
        let args = r#"{"filePath":"src/x.rs","newString":"pub fn f() -> f64 { 1.0 }"}"#;
        assert!(g.check_tool_call("edit", args).is_none());
    }

    #[test]
    fn ledger_grows_on_violation() {
        let mut g = test_gate();
        let _ = fs::remove_file("/tmp/llm_gate_test_ledger.φ");
        let before = g.violations;
        g.record_violation("zero-fabrication", "0.0 without declaration");
        assert_eq!(g.violations, before + 1);
        let text = fs::read_to_string("/tmp/llm_gate_test_ledger.φ").unwrap();
        assert!(text.contains("zero-fabrication"));
        let _ = fs::remove_file("/tmp/llm_gate_test_ledger.φ");
    }

    #[test]
    fn register_contradiction_is_hard() {
        let mut g = test_gate();
        g.register.push(RegisterValue {
            anchor: "the measured peak of the band sits at".to_string(),
            value: 19.7,
            unit: "s".to_string(),
        });
        let v = g
            .check_text("the measured peak of the band sits at 21.3 s")
            .unwrap();
        assert_eq!(v.rule, "register-contradiction");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn register_match_passes() {
        let mut g = test_gate();
        g.register.push(RegisterValue {
            anchor: "the measured peak of the band sits at".to_string(),
            value: 19.7,
            unit: "s".to_string(),
        });
        assert!(g
            .check_text("the measured peak of the band sits at 19.7 s")
            .is_none());
    }

    #[test]
    fn fp_tool_root_markdown_is_not_a_home() {
        let mut g = test_gate();
        for root_doc in [
            r###"{"filePath":"TODO.md","newString":"# offen"}"###,
            r###"{"filePath":"granit.md","newString":"## A = A"}"###,
            r###"{"filePath":"LIESMICH.md","newString":"# lies"}"###,
        ] {
            let v = g.check_tool_call("edit", root_doc).unwrap();
            assert_eq!(v.rule, "canonical-doc-home");
            assert_eq!(v.severity, Severity::Hard);
        }
    }

    #[test]
    fn fn_tool_canonical_homes_pass() {
        let mut g = test_gate();
        for ok in [
            r###"{"filePath":"docs/TODO.md","newString":"# offen"}"###,
            r###"{"filePath":"docs/granit.md","newString":"## A = A"}"###,
            r###"{"filePath":"docs/status/lose-enden.md","newString":"# lose"}"###,
            r##"{"filePath":"AGENTS.md","newString":"# omegaflow"}"##,
            r##"{"filePath":"README.md","newString":"# omegaflow"}"##,
            r###"{"filePath":"src/handover_template.md","newString":"## title"}"###,
        ] {
            assert!(g.check_tool_call("edit", ok).is_none(), "should pass: {ok}");
        }
    }

    #[test]
    fn fp_tool_branch_model_named_is_blocked() {
        let mut g = test_gate();
        for bad in [
            r###"{"filePath":"docs/concepts/x.md","newString":"die Arbeit laeuft auf einem Strang"}"###,
            r###"{"filePath":"src/x.rs","newString":"let wt = git_worktree(&name);"}"###,
            r###"{"filePath":"docs/handover/x.md","newString":"die Leitstelle uebergibt"}"###,
        ] {
            let v = g.check_tool_call("edit", bad).unwrap();
            assert_eq!(v.rule, "single-path");
            assert_eq!(v.severity, Severity::Hard);
        }
    }

    #[test]
    fn fn_tool_plain_code_passes_single_path() {
        let mut g = test_gate();
        for ok in [
            r###"{"filePath":"src/x.rs","newString":"if cond { a } else { b }"}"###,
            r###"{"filePath":"docs/concepts/x.md","newString":"ein Pfad, eine Wahrheit"}"###,
        ] {
            assert!(g.check_tool_call("edit", ok).is_none(), "should pass: {ok}");
        }
    }
}
