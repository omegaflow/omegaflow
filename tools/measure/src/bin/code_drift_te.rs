use omegaflow::te::{surrogate_stats, transfer_entropy_lag};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const GERMAN_CHARS: &[char] = &['ä', 'ö', 'ü', 'ß', 'Ä', 'Ö', 'Ü'];

const DICT_GERMAN: &str = "/usr/share/dict/ngerman";
const DICT_ENGLISH: &str = "/usr/share/dict/american-english";

const CODE_EXTS: &[&str] = &["rs", "wgsl", "js", "mjs"];

const CODE_NOISE: &[&str] = &[
    "mut", "abs", "dir", "dom", "phi", "reg", "eid", "kde", "incl", "div", "cds", "esc", "bgs",
    "sas", "spd", "hpa", "irr", "splitter", "hostname", "typ", "kur", "vor", "ruck", "serie",
    "bsp", "abk", "uni", "prosa", "vage", "sehr", "wart", "hin", "weg", "hinweis", "art", "beide",
    "selbst", "soll", "zwei", "drei", "vier", "fuenf", "sechs", "sieben", "acht", "neun", "zehn",
    "elf", "zwoelf", "reih", "tief", "hoch", "breit", "lang", "kurz",
];

const GERMAN_WORDS: &[&str] = &[
    "aber",
    "alle",
    "alles",
    "als",
    "also",
    "an",
    "andere",
    "anderen",
    "auch",
    "auf",
    "aus",
    "außer",
    "bei",
    "beim",
    "beispiel",
    "bereits",
    "besonders",
    "bzw",
    "da",
    "dabei",
    "dafür",
    "danach",
    "dann",
    "dass",
    "dazu",
    "de",
    "deine",
    "dem",
    "den",
    "denen",
    "der",
    "deren",
    "des",
    "dessen",
    "deutsch",
    "deutsche",
    "die",
    "dies",
    "diese",
    "diesem",
    "diesen",
    "dieser",
    "dieses",
    "doch",
    "dort",
    "durch",
    "ein",
    "eine",
    "einem",
    "einen",
    "einer",
    "eines",
    "einige",
    "einzelnen",
    "es",
    "etwas",
    "falls",
    "führen",
    "für",
    "gegen",
    "gibt",
    "gleich",
    "hier",
    "hin",
    "hinter",
    "ich",
    "ihm",
    "ihn",
    "ihnen",
    "ihr",
    "im",
    "immer",
    "in",
    "indem",
    "ins",
    "ist",
    "jede",
    "jedem",
    "jeden",
    "jeder",
    "jedes",
    "jemals",
    "jetzt",
    "kann",
    "können",
    "lässt",
    "macht",
    "man",
    "manche",
    "mehr",
    "meiner",
    "mit",
    "muss",
    "müssen",
    "nach",
    "nicht",
    "noch",
    "nur",
    "ob",
    "oder",
    "ohne",
    "sehr",
    "sein",
    "seine",
    "seinem",
    "seinen",
    "seiner",
    "seit",
    "sich",
    "sie",
    "sind",
    "soll",
    "sollen",
    "sondern",
    "sonst",
    "sowie",
    "später",
    "statt",
    "trotz",
    "über",
    "um",
    "und",
    "uns",
    "unter",
    "viel",
    "viele",
    "vom",
    "von",
    "vor",
    "war",
    "waren",
    "wäre",
    "während",
    "weil",
    "weiter",
    "weiterhin",
    "welche",
    "welchem",
    "welchen",
    "welcher",
    "welches",
    "wenig",
    "wenn",
    "wer",
    "werden",
    "wird",
    "wir",
    "wo",
    "wurde",
    "würde",
    "zu",
    "zum",
    "zur",
    "zwei",
    "zwischen",
    "vermutlich",
    "vermutliche",
    "vermutlichen",
    "vermutlicher",
    "anscheinend",
    "scheinbar",
    "wahrscheinlich",
];

const GERMAN_NAME_WORDS: &[&str] = &[
    "erfassen",
    "messen",
    "negativ",
    "doppel",
    "anomalie",
    "korrelation",
    "auswertung",
    "bewegung",
    "feld",
    "gefäß",
    "gefaess",
    "gravitation",
    "kraft",
    "masse",
    "zeit",
];

fn german_chars(s: &str) -> u32 {
    s.chars().filter(|c| GERMAN_CHARS.contains(c)).count() as u32
}

fn german_name_score(s: &str) -> u32 {
    if german_chars(s) > 0 {
        return 1;
    }
    for tok in s.split(|c: char| !c.is_alphanumeric()) {
        if GERMAN_NAME_WORDS.contains(&tok.to_lowercase().as_str()) {
            return 1;
        }
    }
    0
}

fn german_words(s: &str) -> u32 {
    let mut n = 0u32;
    for tok in s.split(|c: char| !c.is_alphanumeric()) {
        let lower = tok.to_lowercase();
        if GERMAN_WORDS.contains(&lower.as_str()) {
            n += 1;
        }
    }
    n
}

fn walk_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk_rs(&p, out);
            } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                out.push(p);
            }
        }
    }
}

fn is_docstring(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("///") || t.starts_with("//!")
}

fn collect_channels(text: &str) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut chars = Vec::new();
    let mut words = Vec::new();
    let mut docs = Vec::new();
    for line in text.lines() {
        chars.push(german_chars(line) as f32);
        words.push(german_words(line) as f32);
        docs.push(if is_docstring(line) { 1.0 } else { 0.0 });
    }
    (chars, words, docs)
}

fn line_level(dir: &Path, seed: u64) {
    let mut files = Vec::new();
    walk_rs(dir, &mut files);
    files.sort();
    let mut chars = Vec::new();
    let mut words = Vec::new();
    let mut docs = Vec::new();
    for f in &files {
        if let Ok(text) = fs::read_to_string(f) {
            let (c, w, d) = collect_channels(&text);
            chars.extend(c);
            words.extend(w);
            docs.extend(d);
        }
    }
    let n = chars.len();
    if n < 16 {
        println!("scope={} level=line n={} units-below-min", dir.display(), n);
        return;
    }
    const MAX: usize = 3000;
    if n > MAX {
        let step = n.div_ceil(MAX);
        let mut c2 = Vec::with_capacity(n / step + 1);
        let mut w2 = Vec::with_capacity(n / step + 1);
        let mut d2 = Vec::with_capacity(n / step + 1);
        for ((c, w), d) in chars
            .into_iter()
            .zip(words.into_iter())
            .zip(docs.into_iter())
            .step_by(step)
        {
            c2.push(c);
            w2.push(w);
            d2.push(d);
        }
        chars = c2;
        words = w2;
        docs = d2;
    }
    let te_cw = transfer_entropy_lag(&chars, &words, 1).unwrap_or(0.0);
    let te_wc = transfer_entropy_lag(&words, &chars, 1).unwrap_or(0.0);
    let (_, _, thr_cw) = surrogate_stats(&chars, &words, 1, seed).unwrap_or((0.0, 0.0, 0.0));
    let (_, _, thr_wc) = surrogate_stats(&words, &chars, 1, seed).unwrap_or((0.0, 0.0, 0.0));
    let sig_cw = te_cw > thr_cw;
    let sig_wc = te_wc > thr_wc;
    println!(
        "scope={} level=line n={} german_char_lines={} german_word_lines={} docstring_lines={} te_char->word={:.6} (thr={:.6} {}) te_word->char={:.6} (thr={:.6} {})",
        dir.display(),
        n,
        chars.iter().filter(|&&v| v > 0.0).count(),
        words.iter().filter(|&&v| v > 0.0).count(),
        docs.iter().filter(|&&v| v > 0.0).count(),
        te_cw,
        thr_cw,
        if sig_cw { "SIGNIFICANT" } else { "null" },
        te_wc,
        thr_wc,
        if sig_wc { "SIGNIFICANT" } else { "null" }
    );
}

fn litmus_file(path: &Path, seed: u64, expect_violation: bool) {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => {
            println!("litmus={} UNREADABLE", path.display());
            return;
        }
    };
    let (chars, words, docs) = collect_channels(&text);
    let n = chars.len();
    if n < 16 {
        println!("litmus={} n={} units-below-min", path.display(), n);
        return;
    }
    let te_cw = transfer_entropy_lag(&chars, &words, 1).unwrap_or(0.0);
    let (_, _, thr_cw) = surrogate_stats(&chars, &words, 1, seed).unwrap_or((0.0, 0.0, 0.0));
    let sig_cw = te_cw > thr_cw;
    let doc_lines = docs.iter().filter(|&&v| v > 0.0).count();
    let doc_burst = doc_lines > 0;
    let clean = !sig_cw && !doc_burst;
    let pass = clean == !expect_violation;
    println!(
        "litmus={} n={} docstring_lines={} german_char_lines={} te_char->word={:.6} (thr={:.6} {}) verdict={} expect_violation={} gate={}",
        path.display(),
        n,
        doc_lines,
        chars.iter().filter(|&&v| v > 0.0).count(),
        te_cw,
        thr_cw,
        if sig_cw { "SIGNIFICANT" } else { "null" },
        if clean { "CLEAN" } else { "VIOLATION" },
        if expect_violation { "dirty" } else { "clean" },
        if pass { "PASS" } else { "FAIL" }
    );
}

fn file_level(dir: &Path, seed: u64) {
    let mut files = Vec::new();
    walk_rs(dir, &mut files);
    files.sort();
    let n = files.len();
    if n < 8 {
        println!("scope={} level=file n={} units-below-min", dir.display(), n);
        return;
    }
    let mut name_score = Vec::new();
    let mut content_chars = Vec::new();
    let mut german_named = Vec::new();
    for f in &files {
        let nm = f.file_name().and_then(|x| x.to_str()).unwrap_or("");
        name_score.push(german_name_score(nm) as f32);
        if german_name_score(nm) > 0 {
            german_named.push(nm.to_string());
        }
        let text = fs::read_to_string(f).unwrap_or_default();
        content_chars.push(german_chars(&text) as f32);
    }
    let te_nc = transfer_entropy_lag(&name_score, &content_chars, 1).unwrap_or(0.0);
    let (_, _, thr_nc) =
        surrogate_stats(&name_score, &content_chars, 1, seed).unwrap_or((0.0, 0.0, 0.0));
    let sig_nc = te_nc > thr_nc;
    println!(
        "scope={} level=file n={} german_named={} te_name->content={:.6} (thr={:.6} {})",
        dir.display(),
        n,
        german_named.len(),
        te_nc,
        thr_nc,
        if sig_nc { "SIGNIFICANT" } else { "null" }
    );
    for nm in &german_named {
        println!("  german-named file: {}", nm);
    }
}

fn walk_code(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk_code(&p, out);
            } else if p
                .extension()
                .and_then(|x| x.to_str())
                .map(|x| CODE_EXTS.contains(&x))
                .unwrap_or(false)
            {
                out.push(p);
            }
        }
    }
}

fn load_wordlist_lower(path: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    if let Ok(text) = fs::read_to_string(path) {
        for line in text.lines() {
            let w = line.trim();
            if w.is_empty() {
                continue;
            }
            set.insert(w.to_lowercase());
        }
    }
    set
}

fn lexicon_scan(dirs: &[PathBuf]) {
    let german = load_wordlist_lower(DICT_GERMAN);
    let english = load_wordlist_lower(DICT_ENGLISH);
    if german.is_empty() {
        println!("lexicon={} EMPTY", DICT_GERMAN);
        return;
    }

    let mut files = Vec::new();
    for d in dirs {
        walk_code(d, &mut files);
    }
    files.sort();

    let mut hits: Vec<(String, usize, String)> = Vec::new();
    for f in &files {
        if let Ok(text) = fs::read_to_string(f) {
            for (i, line) in text.lines().enumerate() {
                for tok in line.split(|c: char| !c.is_alphanumeric()) {
                    if tok.len() < 3 {
                        continue;
                    }
                    if tok.chars().all(|c| c.is_uppercase()) {
                        continue;
                    }
                    let lower = tok.to_lowercase();
                    if !german.contains(&lower) {
                        continue;
                    }
                    if english.contains(&lower) {
                        continue;
                    }
                    if CODE_NOISE.contains(&lower.as_str()) {
                        continue;
                    }
                    hits.push((f.display().to_string(), i + 1, tok.to_string()));
                }
            }
        }
    }

    hits.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    let mut by_word: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    let mut cur_file = String::new();
    for (file, line, word) in &hits {
        if *file != cur_file {
            cur_file = file.clone();
            println!("=== {} ===", file);
        }
        println!("  {:>5}: {}", line, word);
        *by_word.entry(word.clone()).or_insert(0) += 1;
    }
    println!();
    println!("=== german lexicon words ({}) by count ===", by_word.len());
    let mut sorted: Vec<(usize, String)> = by_word.into_iter().map(|(w, c)| (c, w)).collect();
    sorted.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    for (count, word) in sorted {
        println!("  {:>4}  {}", count, word);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let lexicon = args.iter().any(|a| a == "--lexicon");
    let dirs: Vec<PathBuf> = args
        .iter()
        .filter(|a| **a != "--lexicon")
        .map(PathBuf::from)
        .collect();
    let dirs = if dirs.is_empty() {
        vec![
            PathBuf::from("src"),
            PathBuf::from("tools/gate/src/bin"),
            PathBuf::from("tools/harvest/src/bin"),
            PathBuf::from("tools/measure/src/bin"),
            PathBuf::from("tools/register/src/bin"),
            PathBuf::from("tools/science/src/bin"),
            PathBuf::from("tools/service/src/bin"),
            PathBuf::from("tools/utils/src/bin"),
        ]
    } else {
        dirs
    };
    if lexicon {
        lexicon_scan(&dirs);
        return;
    }

    let seed = 0x5f3759df_u64;
    let litmus_clean: Option<PathBuf> = std::env::var("LITMUS_CLEAN").ok().map(PathBuf::from);
    let litmus_dirty: Option<PathBuf> = std::env::var("LITMUS_DIRTY").ok().map(PathBuf::from);
    if let Some(lc) = &litmus_clean {
        litmus_file(lc, seed, false);
    }
    if let Some(ld) = &litmus_dirty {
        litmus_file(ld, seed, true);
    }
    for d in &dirs {
        if !d.is_dir() {
            println!("scope={} NOT-A-DIR", d.display());
            continue;
        }
        line_level(d, seed);
        file_level(d, seed);
    }
}
