use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::archivar::units::{allowed_units_for_force, normalize_unit};
use crate::force::force_id_of;
use crate::json::{JsonVal, jstr, parse_json};

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

struct Vocab {
    single_path: Vec<String>,
    fabrication: Vec<(String, String)>,
    zero_fabrication: Vec<String>,
    state_claim: Vec<String>,
    serial_priority: Vec<String>,
    unstable_pointer: Vec<String>,
    line_routing: Vec<String>,
    routing_act_home: Vec<String>,
    german_chars: Vec<char>,
    german_function_words: Vec<String>,
    speculation: Vec<String>,
    forbidden: Vec<String>,
    pii: Vec<String>,
    template_slang: Vec<String>,
    zero_decl: Vec<String>,
    measure_step_markers: Vec<String>,
    measure_step_due: Vec<String>,
    consent_acts: Vec<(String, String)>,
    human_threshold: Vec<String>,
    registered_word: String,
    unit_tokens: Vec<String>,
    diagnostic_markers: Vec<String>,
    de_determiners: Vec<String>,
    de_works: Vec<String>,
    de_content_titles: Vec<String>,
    de_function_words: Vec<String>,
    en_function_words: Vec<String>,
    feedback: std::collections::HashMap<String, String>,
    #[cfg(test)]
    fixtures: std::collections::HashMap<String, String>,
}

fn vocab() -> &'static Vocab {
    static V: OnceLock<Vocab> = OnceLock::new();
    V.get_or_init(load_vocab)
}

fn str_list(json: &JsonVal, key: &str) -> Vec<String> {
    match json {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Arr(items)) => items
                .iter()
                .filter_map(|v| match v {
                    JsonVal::Str(s) => Some(s.clone()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

fn str_value(json: &JsonVal, key: &str) -> String {
    match json {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => String::new(),
        },
        _ => String::new(),
    }
}

fn pair_list(json: &JsonVal, key: &str) -> Vec<(String, String)> {
    match json {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Arr(items)) => items
                .iter()
                .filter_map(|v| match v {
                    JsonVal::Arr(pair) => {
                        let marker = pair.get(0).and_then(|x| match x {
                            JsonVal::Str(s) => Some(s.clone()),
                            _ => None,
                        });
                        let hint = pair.get(1).and_then(|x| match x {
                            JsonVal::Str(s) => Some(s.clone()),
                            _ => None,
                        });
                        match (marker, hint) {
                            (Some(m), Some(h)) => Some((m, h)),
                            _ => None,
                        }
                    }
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

fn load_vocab() -> Vocab {
    let raw = include_str!("commit_gate_vocab.json");
    let json = match parse_json(raw) {
        Some(j) => j,
        None => JsonVal::Null,
    };
    let german_chars: Vec<char> = str_list(&json, "german_chars")
        .iter()
        .flat_map(|s| s.chars())
        .collect();
    Vocab {
        single_path: str_list(&json, "single_path"),
        fabrication: pair_list(&json, "fabrication"),
        zero_fabrication: str_list(&json, "zero_fabrication"),
        state_claim: str_list(&json, "state_claim"),
        serial_priority: str_list(&json, "serial_priority"),
        unstable_pointer: str_list(&json, "unstable_pointer"),
        line_routing: str_list(&json, "line_routing"),
        routing_act_home: str_list(&json, "routing_act_home"),
        german_chars,
        german_function_words: str_list(&json, "german_function_words"),
        speculation: str_list(&json, "speculation"),
        forbidden: str_list(&json, "forbidden"),
        pii: str_list(&json, "pii"),
        template_slang: str_list(&json, "template_slang"),
        zero_decl: str_list(&json, "zero_decl"),
        measure_step_markers: str_list(&json, "measure_step_markers"),
        measure_step_due: str_list(&json, "measure_step_due"),
        consent_acts: pair_list(&json, "consent_acts"),
        human_threshold: str_list(&json, "human_threshold"),
        registered_word: str_value(&json, "registered_word"),
        unit_tokens: str_list(&json, "unit_tokens"),
        diagnostic_markers: str_list(&json, "diagnostic_markers"),
        de_determiners: str_list(&json, "de_determiners"),
        de_works: str_list(&json, "de_works"),
        de_content_titles: str_list(&json, "de_content_titles"),
        de_function_words: str_list(&json, "de_function_words"),
        en_function_words: str_list(&json, "en_function_words"),
        feedback: map_list(&json, "feedback"),
        #[cfg(test)]
        fixtures: map_list(&json, "fixtures"),
    }
}

fn map_list(json: &JsonVal, key: &str) -> std::collections::HashMap<String, String> {
    match json {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Obj(inner)) => inner
                .iter()
                .filter_map(|(k, v)| match v {
                    JsonVal::Str(s) => Some((k.clone(), s.clone())),
                    _ => None,
                })
                .collect(),
            _ => std::collections::HashMap::new(),
        },
        _ => std::collections::HashMap::new(),
    }
}

fn feedback(key: &str) -> &'static str {
    match vocab().feedback.get(key) {
        Some(s) => s.as_str(),
        None => "",
    }
}

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
    let v = vocab();
    let slug = match path.trim_end_matches(".md").rsplit('/').next() {
        Some(s) => s,
        None => path,
    };
    if v.de_works.iter().any(|w| w.as_str() == slug) {
        return true;
    }
    let token = match slug.split('-').next() {
        Some(t) => t,
        None => "",
    };
    if v.de_determiners.iter().any(|d| d.as_str() == token) {
        return true;
    }
    let mut parts = slug.split(['-', ' ']);
    if parts.any(|w| v.de_content_titles.iter().any(|d| d.as_str() == w)) {
        return true;
    }
    slug.chars().any(|c| v.german_chars.contains(&c))
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
    pub line: usize,
    pub quote: String,
    pub feedback: String,
}

fn line_of(content: &str, byte_idx: usize) -> usize {
    let end = byte_idx.min(content.len());
    content.as_bytes()[..end]
        .iter()
        .filter(|b| **b == b'\n')
        .count()
        + 1
}

#[derive(Clone, Debug)]
pub struct RegisterValue {
    pub anchor: String,
    pub value: f64,
    pub unit: String,
}

pub struct Gate {
    pub force_unit_pairs: HashSet<(String, String)>,
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
        for dir in [
            "docs/paper",
            "docs/concepts",
            "docs/surveys",
            "docs/handover",
            "docs/blatt",
        ] {
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
                    scan_register_values(&text, &vocab().unit_tokens, &mut self.register);
                }
            }
        }
    }

    pub fn load_ledger(&mut self) {
        let mut counts: Vec<(String, u64)> = Vec::new();
        if let Ok(text) = fs::read_to_string(&self.ledger_path) {
            for line in text.lines() {
                let rule = match line.split_once('|') {
                    Some((r, _)) => r.trim().to_lowercase(),
                    None => continue,
                };
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
        let epoch = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(_) => return,
        };
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
        self.check_pii(text)
            .or_else(|| self.find_speculation(text))
            .or_else(|| self.check_zero_fabrication(text))
            .or_else(|| self.check_force_unit(text))
            .or_else(|| self.find_learned(text))
            .or_else(|| self.check_register_numbers(text))
            .or_else(|| self.check_unbacked_claim(text))
            .or_else(|| self.check_state_claim(text))
            .or_else(|| self.check_serial_priority(text))
            .or_else(|| self.check_measure_step(text))
    }

    pub fn check_input(&mut self, text: &str) -> Vec<Verdict> {
        let mut findings = Vec::new();
        for f in [
            self.check_pii(text),
            self.find_speculation(text),
            self.check_zero_fabrication(text),
            self.check_force_unit(text),
            self.find_learned(text),
            self.check_register_numbers(text),
            self.check_unbacked_claim(text),
            self.check_state_claim(text),
            self.check_serial_priority(text),
            self.check_measure_step(text),
            self.check_consent_act(text),
            self.check_human_threshold(text),
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
        for spec in &vocab().speculation {
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
                line: 0,
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
                    line: 0,
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
            line: 0,
            feedback: "a completion claim needs an anchor: name the path (src/…, docs/…) that backs it in the tree — a commit SHA is not a measurement".to_string(),
            quote: clip(text, 80),
        })
    }

    fn check_state_claim(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        for claim in &vocab().state_claim {
            let cl = claim.to_lowercase();
            if !lower.contains(&cl) {
                continue;
            }
            let in_backticks = lower.contains(&format!("`{}`", cl));
            let in_quotes = lower.contains(&format!("\"{}\"", cl))
                || lower.contains(&format!("\u{201C}{}\u{201D}", cl));
            if in_backticks || in_quotes {
                continue;
            }
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "state-claim".to_string(),
                line: 0,
                feedback: feedback("state_claim").to_string(),
                quote: clip(text, 90),
            });
        }
        None
    }

    fn check_serial_priority(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        for phrase in &vocab().serial_priority {
            let pl = phrase.to_lowercase();
            if !lower.contains(&pl) {
                continue;
            }
            let in_backticks = lower.contains(&format!("`{}`", pl));
            let in_quotes = lower.contains(&format!("\"{}\"", pl))
                || lower.contains(&format!("\u{201C}{}\u{201D}", pl));
            if in_backticks || in_quotes {
                continue;
            }
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "serial-priority".to_string(),
                line: 0,
                feedback: feedback("serial_priority").to_string(),
                quote: clip(text, 90),
            });
        }
        None
    }

    fn check_measure_step(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        let v = vocab();
        for marker in &v.measure_step_markers {
            let ml = marker.to_lowercase();
            let mut search_from = 0;
            while search_from < lower.len() {
                let pos = match lower[search_from..].find(&ml) {
                    Some(p) => p,
                    None => break,
                };
                let start = search_from + pos;
                let win_start = char_back(&lower, start, 60);
                let mut win_end = (start + ml.len() + 120).min(lower.len());
                while win_end < lower.len() && !lower.is_char_boundary(win_end) {
                    win_end += 1;
                }
                let window = &lower[win_start..win_end];
                let declared = v.measure_step_due.iter().any(|d| word_present(window, d));
                if !declared {
                    return Some(Verdict {
                        severity: Severity::Hard,
                        rule: "measure-step".to_string(),
                        line: 0,
                        feedback: "A = A: a step that names \"measure again\" carries no due date and no trigger — re-measuring the same state is not a measurement; name the due or the trigger".to_string(),
                        quote: clip(text, 80),
                    });
                }
                search_from = start + ml.len();
            }
        }
        None
    }

    fn check_consent_act(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        for (marker, act) in &vocab().consent_acts {
            if lower.contains(marker.as_str()) {
                return Some(Verdict {
                    severity: Severity::Soft,
                    rule: "consent-act".to_string(),
                    line: 0,
                    feedback: format!(
                        "the write path \"{}\" is a {} — writing at a third party needs the operator's per-act consent (mail send, account/API-key, application or data-rights request, submission, contract, payment, foreign-account deletion); {}",
                        marker,
                        act,
                        feedback("consent_act")
                    ),
                    quote: clip(text, 80),
                });
            }
        }
        None
    }

    fn check_human_threshold(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        let word = vocab().registered_word.to_lowercase();
        for marker in &vocab().human_threshold {
            let ml = marker.to_lowercase();
            if !lower.contains(&ml) {
                continue;
            }
            if lower.contains(&word) {
                continue;
            }
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "human-threshold".to_string(),
                line: 0,
                feedback: feedback("human_threshold").to_string(),
                quote: clip(text, 80),
            });
        }
        None
    }

    fn check_pii(&self, text: &str) -> Option<Verdict> {
        let lower = text.to_lowercase();
        for marker in &vocab().pii {
            if lower.contains(marker.as_str()) {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "pii".to_string(),
                    line: 0,
                    feedback: feedback("pii").to_string(),
                    quote: clip(text, 90),
                });
            }
        }
        if let Some((start, end)) = home_path_hit(text) {
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "pii".to_string(),
                line: line_of(text, start),
                feedback: feedback("pii").to_string(),
                quote: clip(&text[start..end], 90),
            });
        }
        None
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
            if vocab()
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
                    line: 0,
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
            for unit in &vocab().unit_tokens {
                let raw = unit.to_lowercase();
                let nu = normalize_unit(unit);
                if unit_match_at(&lower, j, &raw).is_some()
                    || (!nu.is_empty() && nu != raw && unit_match_at(&lower, j, &nu).is_some())
                {
                    return Some(Verdict {
                        severity: Severity::Hard,
                        rule: "zero-fabrication".to_string(),
                        line: 0,
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
                for unit in &vocab().unit_tokens {
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
                            line: 0,
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
        for (num, unit, anchor) in scan_numbers_with_units(text, &vocab().unit_tokens) {
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
                    da.total_cmp(&db)
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
                        line: 0,
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
                line: 0,
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
        if tool == "bash" {
            let obj = parse_json(args_json)?;
            let command = jstr(&obj, "command")?;
            if let Some(v) = self.check_human_threshold(&command) {
                return Some(v);
            }
            return self.check_consent_act(&command);
        }
        if !matches!(tool, "edit" | "write" | "patch" | "multiedit") {
            return None;
        }
        let obj = parse_json(args_json)?;
        let path = jstr(&obj, "filePath")
            .or_else(|| jstr(&obj, "path"))
            .or_else(|| jstr(&obj, "file"));
        let Some(path) = path else {
            return None;
        };
        let content = jstr(&obj, "newString")
            .or_else(|| jstr(&obj, "content"))
            .or_else(|| jstr(&obj, "text"))
            .or_else(|| jstr(&obj, "file_text"));
        let Some(content) = content else {
            return None;
        };
        if let Some(v) = self.check_human_threshold(&content) {
            return Some(v);
        }
        if let Some(v) = self.check_pii(&content) {
            return Some(v);
        }
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
            && root_basename != "README.md"
            && root_basename != "SECURITY.md";
        if canonical_root_doc {
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "canonical-doc-home".to_string(),
                line: 0,
                feedback: "a root-level markdown document is not a canonical home — the document lives under docs/; the root carries only AGENTS.md, README.md and code".to_string(),
                quote: clip(&path, 90),
            });
        }
        if let Some(v) = check_line_routing(&path, &content) {
            return Some(v);
        }
        if let Some(v) = check_routing_act_home(&path, &content) {
            return Some(v);
        }
        let lower_content = content.to_lowercase();
        for word in &vocab().single_path {
            if let Some(idx) = lower_content.find(word.as_str()) {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "single-path".to_string(),
                    line: line_of(&content, idx),
                    feedback: feedback("single_path").to_string(),
                    quote: clip(&content, 90),
                });
            }
        }
        for (line_idx, line) in content.lines().enumerate() {
            let t = line.trim_start();
            if t.starts_with("//") {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "comment".to_string(),
                    line: line_idx + 1,
                    feedback: "code is self-documenting — comments are dead. Remove the line."
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
                            line: line_idx + 1,
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
            let v = vocab();
            let german_char_idx = v.german_chars.iter().find_map(|c| content.find(*c));
            let german_word_idx = v
                .german_function_words
                .iter()
                .find_map(|w| lower.find(w.as_str()));
            if let Some(idx) = german_char_idx.or(german_word_idx) {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "german-in-code".to_string(),
                    line: line_of(&content, idx),
                    feedback: "the code speaks English — German is the counter-slope of the register and the philosophy, not of code".to_string(),
                    quote: clip(&content, 90),
                });
            }
            let zf_idx = vocab()
                .zero_fabrication
                .iter()
                .filter_map(|m| content.find(m.as_str()))
                .min();
            if let Some(idx) = zf_idx {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "zero-fabrication".to_string(),
                    line: line_of(&content, idx),
                    feedback: feedback("zero_fabrication").to_string(),
                    quote: clip(&content, 90),
                });
            }
            for (marker, hint) in &vocab().fabrication {
                if let Some(idx) = content.find(marker.as_str()) {
                    return Some(Verdict {
                        severity: Severity::Hard,
                        rule: "fabrication".to_string(),
                        line: line_of(&content, idx),
                        feedback: hint.clone(),
                        quote: clip(&content, 90),
                    });
                }
            }
            for word in &vocab().template_slang {
                if let Some(idx) = content.to_lowercase().find(word.as_str()) {
                    return Some(Verdict {
                        severity: Severity::Hard,
                        rule: "template-slang".to_string(),
                        line: line_of(&content, idx),
                        feedback: feedback("template_slang").to_string(),
                        quote: clip(&content, 90),
                    });
                }
            }
            for marker in &vocab().diagnostic_markers {
                if let Some(idx) = content.find(marker.as_str()) {
                    let start = idx + marker.len();
                    let rest = &content[start..];
                    let msg: String = rest.chars().take(120).collect();
                    for bad in &vocab().forbidden {
                        if word_present(&msg.to_lowercase(), bad) {
                            return Some(Verdict {
                                severity: Severity::Hard,
                                rule: "forbidden-diagnostic".to_string(),
                                line: line_of(&content, idx),
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
            if let Some(v) = self.check_state_claim(&content) {
                return Some(v);
            }
            if let Some(v) = self.check_serial_priority(&content) {
                return Some(v);
            }
            if path.starts_with("docs/") && path.ends_with(".md") {
                for marker in &vocab().unstable_pointer {
                    if let Some(idx) = content.find(marker.as_str()) {
                        return Some(Verdict {
                            severity: Severity::Hard,
                            rule: "unstable-pointer".to_string(),
                            line: line_of(&content, idx),
                            feedback: feedback("unstable_pointer").to_string(),
                            quote: clip(&content, 90),
                        });
                    }
                }
            }
            if let Some(home) = classify_home(&path) {
                if let Some(verdict) = home_drift(&content, home) {
                    return Some(verdict);
                }
            }
        }
        None
    }
}

fn check_line_routing(path: &str, content: &str) -> Option<Verdict> {
    if path.contains("docs/") && path.contains("/archiv/") {
        return None;
    }
    if path.contains("src/gate/") {
        return None;
    }
    let path_lower = path.to_lowercase();
    for marker in &vocab().line_routing {
        if path_lower.contains(marker.as_str()) {
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "line-routing".to_string(),
                line: 0,
                feedback: feedback("line_routing").to_string(),
                quote: clip(path, 90),
            });
        }
    }
    let lower = content.to_lowercase();
    for marker in &vocab().line_routing {
        if let Some(idx) = lower.find(marker.as_str()) {
            return Some(Verdict {
                severity: Severity::Hard,
                rule: "line-routing".to_string(),
                line: line_of(content, idx),
                feedback: feedback("line_routing").to_string(),
                quote: clip(content, 90),
            });
        }
    }
    None
}

fn check_routing_act_home(path: &str, content: &str) -> Option<Verdict> {
    if !path.starts_with("docs/handover/") {
        return None;
    }
    if path.starts_with("docs/handover/archiv/") || path == "docs/handover/post.md" {
        return None;
    }
    for (idx, line) in content.lines().enumerate() {
        let t = line.trim_start().to_lowercase();
        for marker in &vocab().routing_act_home {
            if t.starts_with(marker.as_str()) {
                return Some(Verdict {
                    severity: Severity::Hard,
                    rule: "routing-act-home".to_string(),
                    line: idx + 1,
                    feedback: feedback("routing_act_home").to_string(),
                    quote: clip(line, 90),
                });
            }
        }
    }
    None
}

pub fn canon_diff(tracked: &[String], declared: &[String]) -> (Vec<String>, Vec<String>) {
    let tracked_set: HashSet<&str> = tracked.iter().map(String::as_str).collect();
    let declared_set: HashSet<&str> = declared.iter().map(String::as_str).collect();
    let mut tracked_not_declared: Vec<String> = tracked
        .iter()
        .filter(|p| !declared_set.contains(p.as_str()))
        .cloned()
        .collect();
    tracked_not_declared.sort();
    tracked_not_declared.dedup();
    let mut declared_not_tracked: Vec<String> = declared
        .iter()
        .filter(|p| !tracked_set.contains(p.as_str()))
        .cloned()
        .collect();
    declared_not_tracked.sort();
    declared_not_tracked.dedup();
    (tracked_not_declared, declared_not_tracked)
}

pub fn declared_canon() -> Vec<String> {
    let text = match fs::read_to_string("phi/canon.φ") {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter(|l| l.starts_with("phi/") && l.ends_with(".φ"))
        .map(str::to_string)
        .collect()
}

pub const PHI_NOTE_MAX: usize = 256;

pub fn prose_violation(line: &str) -> Option<&'static str> {
    if line.starts_with("note ") && line.chars().count() > PHI_NOTE_MAX {
        return Some("phi-note-length");
    }
    if line.starts_with('#') {
        return Some("phi-register-comment");
    }
    None
}

const REGISTER_SECTIONS: [&str; 6] = [
    "Maschinen-Register",
    "Dispositions-Register",
    "Harvest-Master",
    "Bindings",
    "Stationstabellen",
    "Reports",
];

pub fn register_classes() -> Vec<String> {
    let text = match fs::read_to_string("phi/canon.φ") {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    let mut out: Vec<String> = Vec::new();
    let mut active = false;
    for line in text.lines() {
        let t = line.trim();
        if let Some(name) = t
            .strip_prefix("# --- ")
            .and_then(|rest| rest.strip_suffix(" ---"))
        {
            active = REGISTER_SECTIONS.contains(&name);
            continue;
        }
        if active && t.starts_with("phi/") && t.ends_with(".φ") {
            out.push(t.to_string());
        }
    }
    out
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
    let de = count_words(&body, &vocab().de_function_words);
    let en = count_words(&body, &vocab().en_function_words);
    let floor = 4usize;
    match home {
        Home::German => {
            if en >= floor && en > de {
                Some(Verdict {
                    severity: Severity::Hard,
                    rule: "english-in-german".to_string(),
                    line: 0,
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
                    line: 0,
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

fn count_words(hay: &str, words: &[String]) -> usize {
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

const SYSTEM_HOMES: [&str; 13] = [
    "runner",
    "ubuntu",
    "root",
    "admin",
    "ec2-user",
    "codespace",
    "git",
    "www-data",
    "vagrant",
    "operator",
    "probe",
    "omegaflow",
    "o",
];

fn is_user_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.'
}

fn home_path_hit(text: &str) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if !bytes[i..].starts_with(b"/home/") {
            i += 1;
            continue;
        }
        let mut j = i + 6;
        while j < bytes.len() && is_user_byte(bytes[j]) {
            j += 1;
        }
        if j == i + 6 {
            i += 6;
            continue;
        }
        let has_slash = j < bytes.len() && bytes[j] == b'/';
        let at_end = j >= bytes.len();
        if !has_slash && !at_end {
            i += 6;
            continue;
        }
        let name = &text[i + 6..j];
        if SYSTEM_HOMES.iter().any(|h| name.eq_ignore_ascii_case(*h)) {
            i = j;
            continue;
        }
        return Some((i, j));
    }
    None
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
    if boundary_ok { Some(after) } else { None }
}

fn scan_register_values(text: &str, units: &[String], out: &mut Vec<RegisterValue>) {
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

fn scan_numbers_with_units(text: &str, units: &[String]) -> Vec<(f64, String, String)> {
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
        let mut g = Gate::new("kalibrier", "/tmp/commit_gate_test_ledger.φ");
        g.learn_sources(
            "field flux density inverse-square em w/m2\nfield speed linear advective m/s\n",
        );
        g
    }

    fn fx(key: &str) -> String {
        match vocab().fixtures.get(key) {
            Some(s) => s.clone(),
            None => String::new(),
        }
    }

    fn tool_args(path: &str, content: &str) -> String {
        format!(r#"{{"filePath":"{}","newString":"{}"}}"#, path, content)
    }

    #[test]
    fn canon_diff_both_directions() {
        let tracked = vec![
            "phi/a.φ".to_string(),
            "phi/b.φ".to_string(),
            "phi/c.φ".to_string(),
        ];
        let declared = vec![
            "phi/b.φ".to_string(),
            "phi/c.φ".to_string(),
            "phi/d.φ".to_string(),
        ];
        let (tracked_not_declared, declared_not_tracked) = canon_diff(&tracked, &declared);
        assert_eq!(tracked_not_declared, vec!["phi/a.φ".to_string()]);
        assert_eq!(declared_not_tracked, vec!["phi/d.φ".to_string()]);
    }

    #[test]
    fn canon_diff_equal_yields_empty() {
        let tracked = vec!["phi/a.φ".to_string(), "phi/b.φ".to_string()];
        let declared = vec!["phi/b.φ".to_string(), "phi/a.φ".to_string()];
        let (tracked_not_declared, declared_not_tracked) = canon_diff(&tracked, &declared);
        assert!(tracked_not_declared.is_empty());
        assert!(declared_not_tracked.is_empty());
    }

    #[test]
    fn canon_diff_sorted_without_duplicates() {
        let tracked = vec![
            "phi/z.φ".to_string(),
            "phi/a.φ".to_string(),
            "phi/a.φ".to_string(),
        ];
        let declared: Vec<String> = Vec::new();
        let (tracked_not_declared, declared_not_tracked) = canon_diff(&tracked, &declared);
        assert_eq!(
            tracked_not_declared,
            vec!["phi/a.φ".to_string(), "phi/z.φ".to_string()]
        );
        assert!(declared_not_tracked.is_empty());
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
        assert!(
            g.check_input("the block carries the measured series, nothing else")
                .is_empty()
        );
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
        assert!(
            g.check_text("the data are unlikely to arrive, and the likelihood is low")
                .is_none()
        );
    }

    #[test]
    fn fn_speculation_case_insensitive_quote_not_blocked() {
        let mut g = test_gate();
        assert!(
            g.check_text("the register warns that \u{201C}Probably\u{201D} is a guess")
                .is_none()
        );
    }

    #[test]
    fn fp_unbacked_claim_blocked() {
        let mut g = test_gate();
        let v = g.check_text(&fx("claim_fertig")).unwrap();
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
        assert!(
            g.check_text("Fertig. tools/register/src/bin/register_verify.rs is on main")
                .is_none()
        );
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
        assert!(g.check_text(&fx("speculation_quote")).is_none());
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
        assert!(
            g.check_text("the residual is 0.0 — pending, the harvest is absent")
                .is_none()
        );
    }

    #[test]
    fn fn_zero_in_violation_report_with_declaration_later_in_text() {
        let mut g = test_gate();
        assert!(g.check_text(&fx("zero_declaration")).is_none());
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
        assert!(
            g.check_text("cargo check gives 0 Fehler, 0 Warnungen")
                .is_none()
        );
    }

    #[test]
    fn fn_nonzero_number_is_not_zero() {
        let mut g = test_gate();
        assert!(
            g.check_text("the channel sits at 10 m, fully archived")
                .is_none()
        );
        assert!(
            g.check_text("the series reaches 100.0 s without drift")
                .is_none()
        );
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
        assert!(g.check_text(&fx("comparison_zero_a")).is_none());
        assert!(g.check_text(&fx("comparison_zero_b")).is_none());
        assert!(g.check_text(&fx("comparison_zero_c")).is_none());
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
        assert!(
            g.check_text("the field carries the measured series; the gate holds")
                .is_none()
        );
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
        assert_eq!(v.rule, "comment");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_german_in_code() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("german_code"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "german-in-code");
    }

    #[test]
    fn fn_tool_german_in_markdown_passes() {
        let mut g = test_gate();
        let args = tool_args(
            "docs/handover/handover-2026-09-02-a.md",
            &fx("german_prose_short"),
        );
        assert!(g.check_tool_call("write", &args).is_none());
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
        let args = tool_args(
            "docs/concepts/binary-protocol.md",
            &fx("german_prose_english_home"),
        );
        let v = g.check_tool_call("write", &args).unwrap();
        assert_eq!(v.rule, "german-in-english");
    }

    #[test]
    fn fn_tool_german_in_german_title_concept_passes() {
        let mut g = test_gate();
        let args = tool_args(
            "docs/concepts/die-vier-schilde.md",
            &fx("german_prose_title"),
        );
        assert!(g.check_tool_call("write", &args).is_none());
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
        let args = tool_args(
            "docs/concepts/the-counter-slope.md",
            &fx("german_prose_title"),
        );
        assert!(g.check_tool_call("write", &args).is_none());
    }

    #[test]
    fn fp_tool_zero_fabrication_markers_blocked() {
        let mut g = test_gate();
        for marker in &vocab().zero_fabrication {
            let args = tool_args("src/x.rs", marker);
            let v = g.check_tool_call("edit", &args).unwrap();
            assert_eq!(v.rule, "zero-fabrication");
        }
    }

    #[test]
    fn fp_tool_fabrication_markers_blocked() {
        let mut g = test_gate();
        for (marker, _) in &vocab().fabrication {
            let args = tool_args("src/x.rs", marker);
            let v = g.check_tool_call("edit", &args).unwrap();
            assert_eq!(v.rule, "fabrication");
        }
    }

    #[test]
    fn fp_tool_line_routing_old_names_blocked() {
        let mut g = test_gate();
        for marker in &vocab().line_routing {
            let args = tool_args("docs/handover/handover-2026-09-22-x.md", marker);
            let v = g.check_tool_call("write", &args).unwrap();
            assert_eq!(v.rule, "line-routing", "marker {} not blocked", marker);
            assert_eq!(v.severity, Severity::Hard);
        }
    }

    #[test]
    fn fn_tool_line_routing_archived_docs_pass() {
        let mut g = test_gate();
        for marker in &vocab().line_routing {
            let args = tool_args(
                "docs/handover/archiv/handover-2026-09-18-x-folge74.md",
                marker,
            );
            assert!(
                g.check_tool_call("write", &args).is_none(),
                "marker {} blocked in handover archive",
                marker
            );
        }
        let slug_marker = vocab()
            .line_routing
            .iter()
            .find(|m| m.starts_with('-'))
            .unwrap()
            .clone();
        let slug_path = format!(
            "docs/handover/archiv/handover-2026-09-18{}74.md",
            slug_marker
        );
        let args = tool_args(&slug_path, "the point rests in the archive");
        assert!(g.check_tool_call("write", &args).is_none());
        let post_marker = vocab()
            .line_routing
            .iter()
            .find(|m| m.starts_with("an "))
            .unwrap()
            .clone();
        let post_text = format!("{} stays", post_marker);
        let args = tool_args("docs/auftrag/archiv/auftrag-alt.md", &post_text);
        assert!(g.check_tool_call("write", &args).is_none());
    }

    #[test]
    fn fn_tool_line_routing_voice_names_pass() {
        let mut g = test_gate();
        let clean = [
            (
                "docs/handover/handover-2026-09-22-future-folge84.md",
                "the future line carries the point",
            ),
            ("docs/handover/post.md", "An mountain: tree red (step: fix)"),
            (
                "docs/handover/handover-2026-09-22-river-folge1.md",
                "/river_go switches the agent",
            ),
            (
                "docs/handover/handover-2026-09-22-mycelium-folge2.md",
                "linie:sensory takes over",
            ),
        ];
        for (path, text) in clean {
            let args = tool_args(path, text);
            assert!(g.check_tool_call("write", &args).is_none(), "{path}");
        }
    }

    #[test]
    fn fn_tool_line_routing_general_nouns_pass() {
        let mut g = test_gate();
        let args = tool_args(
            "docs/handover/handover-2026-09-22-x.md",
            "ernte forschung entscheidet gebaut",
        );
        assert!(g.check_tool_call("write", &args).is_none());
    }

    #[test]
    fn fp_tool_line_routing_old_slug_in_live_path_blocked() {
        let mut g = test_gate();
        let marker = vocab()
            .line_routing
            .iter()
            .find(|m| m.starts_with('-'))
            .unwrap()
            .clone();
        let path = format!("docs/handover/handover-2026-09-22{}137.md", marker);
        let args = tool_args(&path, "the live routing carries the point");
        let v = g.check_tool_call("write", &args).unwrap();
        assert_eq!(v.rule, "line-routing");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_tool_line_routing_old_slug_in_archived_path_pass() {
        let mut g = test_gate();
        let marker = vocab()
            .line_routing
            .iter()
            .find(|m| m.starts_with('-'))
            .unwrap()
            .clone();
        let path = format!("docs/handover/archiv/handover-2026-09-18{}74.md", marker);
        let args = tool_args(&path, "the point rests in the archive");
        assert!(g.check_tool_call("write", &args).is_none());
    }

    #[test]
    fn fn_tool_routing_act_home() {
        let mut g = test_gate();
        let live = "docs/handover/handover-2026-09-22-river-folge5.md";
        for marker in &vocab().routing_act_home {
            let args = tool_args(live, marker);
            let v = g.check_tool_call("write", &args).unwrap();
            assert_eq!(v.rule, "routing-act-home");
            assert_eq!(v.severity, Severity::Hard);
        }
        for marker in &vocab().routing_act_home {
            let args = tool_args("docs/handover/post.md", marker);
            assert!(g.check_tool_call("write", &args).is_none());
        }
        for marker in &vocab().routing_act_home {
            let args = tool_args("docs/handover/archiv/handover-2026-09-18-x.md", marker);
            assert!(g.check_tool_call("write", &args).is_none());
        }
        let args = tool_args(live, "Post An future steht");
        assert!(g.check_tool_call("write", &args).is_none());
        let args = tool_args("docs/auftrag/auftrag-x.md", "An future:");
        assert!(g.check_tool_call("write", &args).is_none());
        let args = tool_args(
            "docs/handover/_template.md",
            "the line carries its open points",
        );
        assert!(g.check_tool_call("write", &args).is_none());
    }

    #[test]
    fn fp_state_claim_prose_blocked() {
        let mut g = test_gate();
        let v = g
            .check_text("the node is currently built on an ESP32-S3")
            .unwrap();
        assert_eq!(v.rule, "state-claim");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_state_claim_prose_blocked() {
        let mut g = test_gate();
        let args = tool_args(
            "docs/handover/handover-2026-09-21-example.md",
            "the node is currently built on an ESP32-S3",
        );
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "state-claim");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_state_claim_backticked_passes() {
        let mut g = test_gate();
        assert!(
            g.check_text("the phrase `currently built on` is a fabrication marker")
                .is_none()
        );
    }

    #[test]
    fn fp_input_state_claim_collected() {
        let mut g = test_gate();
        let findings = g.check_input("the device is currently built on the Ox64");
        assert!(findings.iter().any(|v| v.rule == "state-claim"));
    }

    #[test]
    fn fp_serial_priority_blocked() {
        let mut g = test_gate();
        for phrase in &vocab().serial_priority {
            let v = g.check_text(phrase).unwrap();
            assert_eq!(v.rule, "serial-priority");
            assert_eq!(v.severity, Severity::Hard);
        }
    }

    #[test]
    fn fp_tool_serial_priority_prose_blocked() {
        let mut g = test_gate();
        let args = tool_args(
            "docs/handover/handover-2026-09-21-example.md",
            "the first open section names the hardest point",
        );
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "serial-priority");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_serial_priority_quoted_passes() {
        let mut g = test_gate();
        assert!(
            g.check_text("there is no \"hardest point\" and no priority ladder")
                .is_none()
        );
    }

    #[test]
    fn fp_riss_as_absent_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("riss_as_absent"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_riss_as_zero_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("riss_as_zero"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_pcmci_cond_endpoint_series_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("pcmci_cond_endpoint_series"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_pcmci_cond_lag_dedup_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("pcmci_cond_lag_dedup"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_arx_null_single_lag_col_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/mathematikerin/te.rs", &fx("arx_null_single_lag_col"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_kernel_id_second_table_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("kernel_id_second_table"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_cdn_capped_release_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/archivar/cdn.rs", &fx("cdn_capped_release_blocked"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_field_unit_literal_1_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/archivar/port.rs", &fx("field_unit_literal_1"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_field_tau_ttl_tenth_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/archivar/port.rs", &fx("field_tau_ttl_tenth"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_riss_keeps_its_word_passes() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("riss_kept"));
        assert!(g.check_tool_call("edit", &args).is_none());
    }

    #[test]
    fn fp_tool_default_tuple_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("fabrication_default_tuple"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_else_zero_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("fabrication_else_zero"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_absent_anchor_tuple_blocked() {
        let mut g = test_gate();
        let args = tool_args(
            "tools/harvest/src/bin/nexrad_level2_compiler.rs",
            &fx("fabrication_absent_anchor"),
        );
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_arx_null_raw_series_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/mathematikerin/te.rs", &fx("arx_null_raw_series"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_wkt_first_id_find_blocked() {
        let mut g = test_gate();
        let args = tool_args(
            "tools/harvest/src/bin/las_compiler.rs",
            &fx("wkt_first_id_find"),
        );
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_geokey_unconditional_geodetic_fallback_blocked() {
        let mut g = test_gate();
        let args = tool_args(
            "src/archivar/las/mod.rs",
            &fx("geokey_unconditional_geodetic_fallback"),
        );
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_tnf_format_gate_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/archivar/odf.rs", &fx("tnf_format_gate"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_bare_or_swallow_gh_issue_blocked() {
        let mut g = test_gate();
        let args = tool_args(".github/workflows/x.yml", &fx("or_swallow_gh_issue"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_coord_default_in_markers_blocked() {
        let mut g = test_gate();
        let args = tool_args(
            "src/archivar/fetch.rs",
            &fx("fabrication_marker_coord_default"),
        );
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "fabrication");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_live_markers_carry_no_coordinate_keys() {
        let keys: Vec<String> = crate::archivar::fetch::live_markers()
            .into_iter()
            .map(|(k, _)| k)
            .collect();
        for key in [
            "{lat}",
            "{lon}",
            "{lat_min}",
            "{lat_max}",
            "{lon_min}",
            "{lon_max}",
            "{grid}",
            "{nearest_station}",
        ] {
            assert!(
                !keys.contains(&key.to_string()),
                "live_markers carries the coordinate key {key}"
            );
        }
    }

    #[test]
    fn fn_tool_wrapped_and_standalone_gh_issue_pass() {
        let mut g = test_gate();
        for ok in [fx("or_wrapped_gh_issue"), fx("standalone_gh_issue")] {
            let args = tool_args(".github/workflows/x.yml", &ok);
            assert!(
                g.check_tool_call("edit", &args).is_none(),
                "clean fixture: {ok}"
            );
        }
    }

    #[test]
    fn fp_tool_unstable_pointer_blocked() {
        let mut g = test_gate();
        let args = tool_args(
            "docs/zustand/external-state.md",
            &fx("unstable_pointer_drift"),
        );
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "unstable-pointer");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_tool_unstable_pointer_only_in_docs_markdown() {
        let mut g = test_gate();
        let args = tool_args("phi/sources.φ", &fx("unstable_pointer_offsite"));
        assert!(g.check_tool_call("edit", &args).is_none());
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
        let _ = fs::remove_file("/tmp/commit_gate_test_ledger.φ");
        let before = g.violations;
        g.record_violation("zero-fabrication", "0.0 without declaration");
        assert_eq!(g.violations, before + 1);
        let text = fs::read_to_string("/tmp/commit_gate_test_ledger.φ").unwrap();
        assert!(text.contains("zero-fabrication"));
        let _ = fs::remove_file("/tmp/commit_gate_test_ledger.φ");
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
        assert!(
            g.check_text("the measured peak of the band sits at 19.7 s")
                .is_none()
        );
    }

    #[test]
    fn fp_tool_root_markdown_is_not_a_home() {
        let mut g = test_gate();
        for root_doc in [
            r###"{"filePath":"handover.md","newString":"# offen"}"###,
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
            r###"{"filePath":"docs/handover/handover-2026-09-09-te-atom-4.md","newString":"# offen"}"###,
            r###"{"filePath":"docs/granit.md","newString":"## A = A"}"###,
            r###"{"filePath":"docs/auftrag/auftrag-beispiel.md","newString":"# lose"}"###,
            r##"{"filePath":"AGENTS.md","newString":"# omegaflow"}"##,
            r##"{"filePath":"README.md","newString":"# omegaflow"}"##,
            r##"{"filePath":"SECURITY.md","newString":"# responsible use"}"##,
            r###"{"filePath":"src/handover_template.md","newString":"## title"}"###,
        ] {
            assert!(
                g.check_tool_call("edit", ok).is_none(),
                "clean fixture: {ok}"
            );
        }
    }

    #[test]
    fn fp_tool_branch_model_named_is_blocked() {
        let mut g = test_gate();
        for word in &vocab().single_path {
            let args = tool_args("src/x.rs", word);
            let v = g.check_tool_call("edit", &args).unwrap();
            assert_eq!(v.rule, "single-path");
            assert_eq!(v.severity, Severity::Hard);
        }
    }

    #[test]
    fn fn_tool_plain_code_passes_single_path() {
        let mut g = test_gate();
        for ok in [
            tool_args("src/x.rs", "if cond { a } else { b }"),
            tool_args("docs/concepts/x.md", &fx("single_path_clean")),
        ] {
            assert!(
                g.check_tool_call("edit", &ok).is_none(),
                "clean fixture: {ok}"
            );
        }
    }

    #[test]
    fn fp_tool_template_slang_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("template_slang_code"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "template-slang");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_measure_step_without_due_blocked() {
        let mut g = test_gate();
        let v = g.check_text(&fx("measure_step_fail")).unwrap();
        assert_eq!(v.rule, "measure-step");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_measure_step_with_due_passes() {
        let mut g = test_gate();
        assert!(g.check_text(&fx("measure_step_clean")).is_none());
    }

    #[test]
    fn fp_consent_send_flagged() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("consent_send"));
        assert!(findings.iter().any(|v| v.rule == "consent-act"));
        assert!(findings.iter().all(|v| v.severity == Severity::Soft));
    }

    #[test]
    fn fp_consent_http_write_flagged() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("consent_http_write"));
        assert!(findings.iter().any(|v| v.rule == "consent-act"));
    }

    #[test]
    fn fp_consent_submission_flagged() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("consent_submission"));
        assert!(findings.iter().any(|v| v.rule == "consent-act"));
    }

    #[test]
    fn fn_consent_get_passes() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("consent_get"));
        assert!(findings.iter().all(|v| v.rule != "consent-act"));
    }

    #[test]
    fn fn_consent_dry_run_passes() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("consent_dry_run"));
        assert!(findings.iter().all(|v| v.rule != "consent-act"));
    }

    #[test]
    fn fp_human_threshold_send_without_word() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("human_threshold_send"));
        assert!(findings.iter().any(|v| v.rule == "human-threshold"));
        let args = format!(r#"{{"command":"{}"}}"#, fx("human_threshold_send"));
        let v = g.check_tool_call("bash", &args).unwrap();
        assert_eq!(v.rule, "human-threshold");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_human_threshold_dry_run_passes() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("human_threshold_dry_run"));
        assert!(findings.iter().all(|v| v.rule != "human-threshold"));
    }

    #[test]
    fn fn_human_threshold_with_word_passes() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("human_threshold_with_word"));
        assert!(findings.iter().all(|v| v.rule != "human-threshold"));
    }

    #[test]
    fn fp_pii_private_mail_domain_blocked() {
        let mut g = test_gate();
        let v = g.check_text(&fx("pii_mail_domain")).unwrap();
        assert_eq!(v.rule, "pii");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fp_pii_home_path_blocked() {
        let mut g = test_gate();
        let v = g.check_text(&fx("pii_home_path")).unwrap();
        assert_eq!(v.rule, "pii");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_pii_role_address_passes() {
        let mut g = test_gate();
        assert!(g.check_text(&fx("pii_role_address")).is_none());
    }

    #[test]
    fn fn_pii_ci_home_passes() {
        let mut g = test_gate();
        assert!(g.check_text(&fx("pii_ci_home")).is_none());
    }

    #[test]
    fn fn_pii_clean_text_passes() {
        let mut g = test_gate();
        assert!(
            g.check_text("the field carries the measured series; the gate holds")
                .is_none()
        );
    }

    #[test]
    fn fp_tool_pii_markers_blocked() {
        let mut g = test_gate();
        for marker in &vocab().pii {
            let args = tool_args("src/x.rs", marker);
            let v = g.check_tool_call("edit", &args).unwrap();
            assert_eq!(v.rule, "pii", "marker {} not blocked", marker);
            assert_eq!(v.severity, Severity::Hard);
        }
    }

    #[test]
    fn fp_tool_pii_home_path_blocked() {
        let mut g = test_gate();
        let args = tool_args("src/x.rs", &fx("pii_home_path"));
        let v = g.check_tool_call("edit", &args).unwrap();
        assert_eq!(v.rule, "pii");
        assert_eq!(v.severity, Severity::Hard);
    }

    #[test]
    fn fn_tool_pii_ci_home_passes() {
        let mut g = test_gate();
        let args = tool_args(".github/workflows/x.yml", &fx("pii_ci_home"));
        assert!(g.check_tool_call("edit", &args).is_none());
    }

    #[test]
    fn fp_input_pii_collected() {
        let mut g = test_gate();
        let findings = g.check_input(&fx("pii_mail_domain"));
        assert!(findings.iter().any(|v| v.rule == "pii"));
        assert!(findings.iter().all(|v| v.severity == Severity::Soft));
    }

    #[test]
    fn fp_tool_bash_consent_write_flagged() {
        let mut g = test_gate();
        let args = format!(r#"{{"command":"{}"}}"#, fx("consent_http_write"));
        let v = g.check_tool_call("bash", &args).unwrap();
        assert_eq!(v.rule, "consent-act");
        assert_eq!(v.severity, Severity::Soft);
    }

    #[test]
    fn fn_tool_bash_get_passes() {
        let mut g = test_gate();
        let args = format!(r#"{{"command":"{}"}}"#, fx("consent_get"));
        assert!(g.check_tool_call("bash", &args).is_none());
    }

    #[test]
    fn fn_prose_note_256_chars_passes() {
        let line = format!("note {}", "x".repeat(PHI_NOTE_MAX - "note ".len()));
        assert_eq!(line.chars().count(), PHI_NOTE_MAX);
        assert!(prose_violation(&line).is_none());
    }

    #[test]
    fn fp_prose_note_257_chars_blocked() {
        let line = format!("note {}", "x".repeat(PHI_NOTE_MAX + 1 - "note ".len()));
        assert_eq!(line.chars().count(), PHI_NOTE_MAX + 1);
        assert_eq!(prose_violation(&line), Some("phi-note-length"));
    }

    #[test]
    fn fp_prose_comment_line_blocked() {
        assert_eq!(prose_violation("# comment"), Some("phi-register-comment"));
        assert_eq!(
            prose_violation("# --- section ---"),
            Some("phi-register-comment")
        );
    }

    #[test]
    fn fn_prose_clean_line_passes() {
        assert!(prose_violation("note prosa bleibt kurz").is_none());
        assert!(prose_violation("url https://example.org").is_none());
        assert!(prose_violation("").is_none());
    }

    #[test]
    fn fp_prose_fixtures_carry_the_two_kinds() {
        assert_eq!(
            prose_violation(&fx("phi_register_note_over_256")),
            Some("phi-note-length")
        );
        assert_eq!(
            prose_violation(&fx("phi_register_comment")),
            Some("phi-register-comment")
        );
    }

    #[test]
    fn fn_register_classes_hold_registers_not_canon() {
        let classes = register_classes();
        assert!(classes.iter().any(|p| p == "phi/witnesses.φ"));
        assert!(!classes.iter().any(|p| p == "phi/canon.φ"));
        assert!(!classes.iter().any(|p| p.starts_with("phi/pipeline/")));
        assert!(classes.iter().any(|p| p == "phi/bindings/dust-maske.φ"));
        assert!(classes.iter().any(|p| p == "phi/reports/scan_coverage.φ"));
    }
}
