use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MARKER_STATEMENTS: &str = "% === MANDATORY STATEMENTS (generated) ===";
const MAX_TITLE: usize = 75;
const MAX_ABSTRACT_WORDS: usize = 200;

#[derive(Debug, Default)]
struct Header {
    fields: BTreeMap<String, String>,
    title: Option<String>,
    class: Option<String>,
    status: Option<String>,
    sha256: Option<String>,
    date: Option<String>,
    see_also: Option<String>,
}

fn strip_and_parse_header(src: &str) -> (String, Header) {
    let mut hdr = Header::default();
    if let Some(start) = src.find("<!--") {
        if let Some(end) = src[start + 4..].find("-->") {
            let block = &src[start + 4..start + 4 + end];

            let mut rest = start + 4 + end + 3;
            if src[rest..].starts_with("\r\n") {
                rest += 2;
            } else if src[rest..].starts_with('\n') {
                rest += 1;
            }
            let body = src[rest..].to_string();
            for line in block.lines() {
                let line = line.trim();
                if let Some((k, v)) = line.split_once(':') {
                    let k = k.trim().to_string();
                    let v = v.trim().to_string();
                    match k.as_str() {
                        "title" => hdr.title = Some(v.clone()),
                        "class" => hdr.class = Some(v.clone()),
                        "status" => hdr.status = Some(v.clone()),
                        "sha256" => hdr.sha256 = Some(v.clone()),
                        "date" => hdr.date = Some(v.clone()),
                        "see-also" => hdr.see_also = Some(v.clone()),
                        _ => {}
                    }
                    hdr.fields.insert(k, v);
                }
            }
            return (body, hdr);
        }
    }
    (src.to_string(), hdr)
}

fn sha256_hex(data: &[u8]) -> String {
    let mut digest = [0u32; 8];
    digest[0] = 0x6a09e667;
    digest[1] = 0xbb67ae85;
    digest[2] = 0x3c6ef372;
    digest[3] = 0xa54ff53a;
    digest[4] = 0x510e527f;
    digest[5] = 0x9b05688c;
    digest[6] = 0x1f83d9ab;
    digest[7] = 0x5be0cd19;
    let mut msg: Vec<u8> = Vec::with_capacity(data.len() + 64);
    msg.extend_from_slice(data);
    let bitlen = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    for i in 0..8 {
        let shift = (7 - i) * 8;
        msg.push(((bitlen >> shift) & 0xff) as u8);
    }
    let k: [u32; 64] = [
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
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = ((chunk[i * 4] as u32) << 24)
                | ((chunk[i * 4 + 1] as u32) << 16)
                | ((chunk[i * 4 + 2] as u32) << 8)
                | (chunk[i * 4 + 3] as u32);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut a = digest[0];
        let mut b = digest[1];
        let mut c = digest[2];
        let mut d = digest[3];
        let mut e = digest[4];
        let mut f = digest[5];
        let mut g = digest[6];
        let mut h = digest[7];
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        digest[0] = digest[0].wrapping_add(a);
        digest[1] = digest[1].wrapping_add(b);
        digest[2] = digest[2].wrapping_add(c);
        digest[3] = digest[3].wrapping_add(d);
        digest[4] = digest[4].wrapping_add(e);
        digest[5] = digest[5].wrapping_add(f);
        digest[6] = digest[6].wrapping_add(g);
        digest[7] = digest[7].wrapping_add(h);
    }
    let mut out = String::with_capacity(64);
    for v in digest {
        out.push_str(&format!("{:08x}", v));
    }
    out
}

fn roman_ascii(c: char) -> Option<&'static str> {
    match c {
        'Ⅰ' | 'ⅰ' => Some("I"),
        'Ⅱ' | 'ⅱ' => Some("II"),
        'Ⅲ' | 'ⅲ' => Some("III"),
        'Ⅳ' | 'ⅳ' => Some("IV"),
        'Ⅴ' | 'ⅴ' => Some("V"),
        'Ⅵ' | 'ⅵ' => Some("VI"),
        'Ⅶ' | 'ⅶ' => Some("VII"),
        'Ⅷ' | 'ⅷ' => Some("VIII"),
        'Ⅸ' | 'ⅸ' => Some("IX"),
        'Ⅹ' | 'ⅹ' => Some("X"),
        'Ⅺ' | 'ⅺ' => Some("XI"),
        'Ⅻ' | 'ⅻ' => Some("XII"),
        _ => None,
    }
}

fn superscript_value(c: char) -> Option<char> {
    match c {
        '⁰' => Some('0'),
        '¹' => Some('1'),
        '²' => Some('2'),
        '³' => Some('3'),
        '⁴' => Some('4'),
        '⁵' => Some('5'),
        '⁶' => Some('6'),
        '⁷' => Some('7'),
        '⁸' => Some('8'),
        '⁹' => Some('9'),
        '⁺' => Some('+'),
        '⁻' => Some('-'),
        _ => None,
    }
}

fn subscript_value(c: char) -> Option<char> {
    match c {
        '₀' => Some('0'),
        '₁' => Some('1'),
        '₂' => Some('2'),
        '₃' => Some('3'),
        '₄' => Some('4'),
        '₅' => Some('5'),
        '₆' => Some('6'),
        '₇' => Some('7'),
        '₈' => Some('8'),
        '₉' => Some('9'),
        _ => None,
    }
}

fn math_glyph(c: char) -> Option<&'static str> {
    match c {
        '×' => Some("\\times"),
        '·' => Some("\\cdot"),
        'ϖ' => Some("\\varpi"),
        'Ω' => Some("\\Omega"),
        'ω' => Some("\\omega"),
        'σ' => Some("\\sigma"),
        'Σ' => Some("\\Sigma"),
        'λ' => Some("\\lambda"),
        'τ' => Some("\\tau"),
        '→' => Some("\\to"),
        '←' => Some("\\gets"),
        '≥' => Some("\\geq"),
        '≤' => Some("\\leq"),
        '⊕' => Some("\\oplus"),
        '≡' => Some("\\equiv"),
        '≈' => Some("\\approx"),
        '±' => Some("\\pm"),
        '∈' => Some("\\in"),
        '−' => Some("-"),
        _ => None,
    }
}

fn degree_glyph(c: char) -> bool {
    c == '°'
}

fn normalize_arxiv(s: &str) -> (String, bool) {
    let mut out = String::new();
    let mut ascii = true;
    for c in s.chars() {
        if let Some(r) = roman_ascii(c) {
            out.push_str(r);
            ascii = false;
        } else if c.is_ascii() {
            out.push(c);
        } else {
            out.push(c);
            ascii = false;
        }
    }
    (out, ascii)
}

fn escape_text(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '&' => out.push_str("\\&"),
            '%' => out.push_str("\\%"),
            '#' => out.push_str("\\#"),
            '_' => out.push_str("\\_"),
            '$' => out.push_str("\\$"),
            '~' => out.push_str("\\textasciitilde{}"),
            '^' => out.push_str("\\^{}"),
            '\\' => out.push_str("\\textbackslash{}"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            _ => out.push(c),
        }
    }
    out
}

fn escape_code(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '&' => out.push_str("\\&"),
            '%' => out.push_str("\\%"),
            '#' => out.push_str("\\#"),
            '_' => out.push_str("\\_"),
            '$' => out.push_str("\\$"),
            '~' => out.push_str("\\textasciitilde{}"),
            '^' => out.push_str("\\^{}"),
            '\\' => out.push_str("\\textbackslash{}"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            _ => out.push(c),
        }
    }
    out
}

fn render_inline(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut out = String::new();
    let mut i = 0;

    while i < n {
        if chars[i] == '*' && i + 1 < n && chars[i + 1] == '*' {
            if let Some(j) = find_after(&chars, i + 2, "**") {
                let inner: String = chars[i + 2..j].iter().collect();
                out.push_str("\\textbf{");
                out.push_str(&render_inline(&inner));
                out.push('}');
                i = j + 2;
                continue;
            }
        }

        if chars[i] == '`' {
            let mut j = i + 1;
            while j < n && chars[j] != '`' {
                j += 1;
            }
            let inner: String = chars[i + 1..j].iter().collect();
            out.push_str("\\texttt{");
            out.push_str(&escape_code(&inner));
            out.push('}');
            i = j + 1;
            continue;
        }

        if chars[i] == '*' {
            if let Some(j) = find_after(&chars, i + 1, "*") {
                if j + 1 < n && chars[j + 1] != '*' {
                    let inner: String = chars[i + 1..j].iter().collect();
                    out.push_str("\\emph{");
                    out.push_str(&render_inline(&inner));
                    out.push('}');
                    i = j + 1;
                    continue;
                }
            }
        }

        if chars[i] == '[' {
            if let Some(close) = find_after(&chars, i + 1, "](") {
                if let Some(end) = chars[close + 2..].iter().position(|&c| c == ')') {
                    let text: String = chars[i + 1..close].iter().collect();
                    let url: String = chars[close + 2..close + 2 + end].iter().collect();
                    out.push_str(&render_inline(&text));
                    out.push_str(" (");
                    out.push_str(&escape_text(&url));
                    out.push(')');
                    i = close + 2 + end + 1;
                    continue;
                }
            }
        }

        if superscript_value(chars[i]).is_some() {
            let mut run = String::new();
            while i < n {
                if let Some(v) = superscript_value(chars[i]) {
                    run.push(v);
                    i += 1;
                } else {
                    break;
                }
            }
            out.push_str("$^{");
            out.push_str(&run);
            out.push_str("}$");
            continue;
        }

        if subscript_value(chars[i]).is_some() {
            let mut run = String::new();
            while i < n {
                if let Some(v) = subscript_value(chars[i]) {
                    run.push(v);
                    i += 1;
                } else {
                    break;
                }
            }
            out.push_str("$_{");
            out.push_str(&run);
            out.push_str("}$");
            continue;
        }

        if degree_glyph(chars[i]) {
            out.push_str("$^{\\circ}$");
            i += 1;
            continue;
        }

        if let Some(cmd) = math_glyph(chars[i]) {
            out.push_str("$");
            out.push_str(cmd);
            out.push_str("$");
            i += 1;
            continue;
        }

        let c = chars[i];
        match c {
            '&' => out.push_str("\\&"),
            '%' => out.push_str("\\%"),
            '#' => out.push_str("\\#"),
            '_' => out.push_str("\\_"),
            '$' => out.push_str("\\$"),
            '~' => out.push_str("\\textasciitilde{}"),
            '^' => out.push_str("\\^{}"),
            '\\' => out.push_str("\\textbackslash{}"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            _ => out.push(c),
        }
        i += 1;
    }
    out
}

fn find_after(chars: &[char], from: usize, pat: &str) -> Option<usize> {
    let pc: Vec<char> = pat.chars().collect();
    let mut i = from;
    while i + pc.len() <= chars.len() {
        if chars[i..i + pc.len()] == pc[..] {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn is_table_sep(line: &str) -> bool {
    let t = line.trim();
    t.starts_with('|') && t.contains("---")
}

fn render_table(rows: &[String]) -> String {
    let mut out = String::new();
    out.push_str("\\begin{table}[htbp]\n");
    out.push_str("\\centering\n");
    let header_row = &rows[0];
    let header_cells = split_cells(header_row);
    out.push_str(&format!(
        "\\begin{{tabular}}{{{}}}\n\\hline\n",
        header_cells
            .iter()
            .map(|_| "l")
            .collect::<Vec<_>>()
            .join("")
    ));
    for (idx, row) in rows.iter().enumerate() {
        if is_table_sep(row) {
            continue;
        }
        let cells = split_cells(row);
        let rendered: Vec<String> = cells.iter().map(|c| render_inline(c)).collect();
        out.push_str(&format!("{}\\\\\n", rendered.join(" & ")));
        if idx == 0 {
            out.push_str("\\hline\n");
        }
    }
    out.push_str("\\hline\n\\end{tabular}\n\\end{table}\n");
    out
}

fn split_cells(line: &str) -> Vec<String> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').unwrap_or(t);
    t.split('|').map(|s| s.trim().to_string()).collect()
}

fn is_bullet(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("- ") || t.starts_with("* ")
}

fn is_ordered(line: &str) -> bool {
    let mut it = line.trim_start().chars().peekable();
    let mut digits = 0;
    while let Some(&c) = it.peek() {
        if c.is_ascii_digit() {
            digits += 1;
            it.next();
        } else {
            break;
        }
    }
    if digits == 0 {
        return false;
    }
    if it.peek() != Some(&'.') {
        return false;
    }
    it.next();
    match it.peek() {
        None => true,
        Some(&c) => c.is_whitespace(),
    }
}

fn collect_assets(body: &str) -> Vec<String> {
    let mut set = BTreeSet::new();
    for token in
        body.split(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-' && c != '.')
    {
        let t = token.trim();
        let lower = t.to_ascii_lowercase();
        if lower.ends_with(".bin") || lower.ends_with(".json") {
            let stem = &t[..t.len() - 4];

            let starts_ok = t
                .chars()
                .next()
                .map(|c| c.is_ascii_alphanumeric())
                .unwrap_or(false);
            if starts_ok && stem.chars().any(|c| c.is_ascii_alphanumeric()) {
                set.insert(t.to_string());
            }
        }
    }
    set.into_iter().collect()
}

fn git_head() -> (Option<String>, Option<String>) {
    let sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    let date = Command::new("git")
        .args(["log", "-1", "--format=%ci"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    (sha, date)
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

struct PaperReport {
    slug: String,
    title_raw_chars: usize,
    title_arxiv_chars: usize,
    title_ok: bool,
    abstract_words: usize,
    abstract_ok: bool,
    body_sha: String,
    header_sha: String,
    sha_match: bool,
    numbers_match: bool,
    number_count: usize,
}

fn build_document(md_path: &Path, out_dir: &Path, check_only: bool) -> PaperReport {
    let src = fs::read_to_string(md_path).expect("read paper");
    let slug = md_path.file_stem().unwrap().to_string_lossy().to_string();

    let (body, hdr) = strip_and_parse_header(&src);

    let title = hdr
        .title
        .clone()
        .unwrap_or_else(|| "Pending title — the header carries no title".to_string());
    let (title_arxiv, _) = normalize_arxiv(&title);
    let title_raw_chars = title.chars().count();
    let title_arxiv_chars = title_arxiv.chars().count();
    let title_ok = title_arxiv_chars <= MAX_TITLE;

    let body_sha = sha256_hex(body.as_bytes());
    let header_sha = hdr.sha256.clone().unwrap_or_default();
    let sha_match = !header_sha.is_empty() && header_sha == body_sha;

    let (commit_sha, commit_date) = git_head();

    let mut abstract_text = String::new();

    let mut latex_body = String::new();
    let mut i = 0usize;
    let lines: Vec<&str> = body.lines().collect();
    let nlines = lines.len();

    while i < nlines {
        let raw = lines[i];
        let line = raw.trim_end();

        if line.starts_with("# ") {
            i += 1;
            continue;
        }

        if line.trim() == "## Abstract" || line.trim() == "# Abstract" {
            i += 1;
            let mut para: Vec<String> = Vec::new();
            while i < nlines {
                let l = lines[i].trim_end();
                if l.starts_with('#') {
                    break;
                }
                if l.trim().is_empty() {
                    i += 1;
                    continue;
                }
                para.push(l.to_string());
                i += 1;
            }
            abstract_text = para.join(" ");
            continue;
        }

        if line.trim() == "---" || line.trim() == "***" || line.trim() == "___" {
            latex_body.push_str("\\medskip\n\\hrule\n\\medskip\n\n");
            i += 1;
            continue;
        }

        if line.starts_with("### ") {
            let text = render_inline(line[4..].trim());
            latex_body.push_str(&format!("\\subsection{{{}}}\n\n", text));
            i += 1;
            continue;
        }
        if line.starts_with("#### ") {
            let text = render_inline(line[5..].trim());
            latex_body.push_str(&format!("\\subsubsection{{{}}}\n\n", text));
            i += 1;
            continue;
        }
        if line.starts_with("## ") {
            let text = render_inline(line[3..].trim());
            if text == "References" {
                latex_body.push_str("\\section{References}\n\n");
            } else {
                latex_body.push_str(&format!("\\section{{{}}}\n\n", text));
            }
            i += 1;
            continue;
        }

        if line.trim().is_empty() {
            latex_body.push_str("\n");
            i += 1;
            continue;
        }

        if line.trim().starts_with('|') {
            let mut block: Vec<String> = Vec::new();
            while i < nlines && lines[i].trim().starts_with('|') {
                block.push(lines[i].to_string());
                i += 1;
            }

            latex_body.push_str(&render_table(&block));
            latex_body.push_str("\n");
            continue;
        }

        let block_start = i;
        while i < nlines {
            let l = lines[i].trim_end();
            if l.trim().is_empty()
                || l.starts_with('#')
                || l.trim().starts_with('|')
                || l.trim() == "---"
                || l.trim() == "***"
                || l.trim() == "___"
            {
                break;
            }
            i += 1;
        }
        let block: Vec<String> = lines[block_start..i]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let first = block[0].trim_start();

        if is_ordered(&first) {
            let mut items: Vec<String> = Vec::new();
            let mut cur = String::new();
            for l in &block {
                let t = l.trim_start();
                if is_ordered(t) {
                    if !cur.is_empty() {
                        items.push(std::mem::take(&mut cur));
                    }
                    let pos = t.find('.').unwrap();
                    let num = t[..pos].trim();
                    cur = format!("\\item[{}] {}", num, render_inline(t[pos + 1..].trim()));
                } else {
                    cur.push(' ');
                    cur.push_str(&render_inline(t));
                }
            }
            if !cur.is_empty() {
                items.push(cur);
            }
            latex_body.push_str("\\begin{enumerate}\n");
            for it in items {
                latex_body.push_str(&it);
                latex_body.push('\n');
            }
            latex_body.push_str("\\end{enumerate}\n\n");
            continue;
        }
        if is_bullet(&first) {
            let mut items: Vec<String> = Vec::new();
            let mut cur = String::new();
            for l in &block {
                let t = l.trim_start();
                if is_bullet(t) {
                    if !cur.is_empty() {
                        items.push(std::mem::take(&mut cur));
                    }
                    cur = render_inline(t[2..].trim());
                } else {
                    cur.push(' ');
                    cur.push_str(&render_inline(t));
                }
            }
            if !cur.is_empty() {
                items.push(cur);
            }
            latex_body.push_str("\\begin{itemize}\n");
            for it in items {
                latex_body.push_str(&format!("\\item {}\n", it));
            }
            latex_body.push_str("\\end{itemize}\n\n");
            continue;
        }
        if first.trim_start().starts_with('>') {
            let mut quote = String::new();
            for l in &block {
                let t = l.trim_start();
                if let Some(rest) = t.strip_prefix('>') {
                    let rest = rest.strip_prefix(' ').unwrap_or(rest);
                    quote.push(' ');
                    quote.push_str(rest.trim());
                } else {
                    quote.push(' ');
                    quote.push_str(t.trim());
                }
            }
            latex_body.push_str(&format!(
                "\\begin{{quote}}{}\n\\end{{quote}}\n\n",
                render_inline(quote.trim())
            ));
            continue;
        }

        let joined = block.join(" ");
        latex_body.push_str(&format!("{}\n\n", render_inline(&joined)));
    }

    let assets = collect_assets(&body);
    let asset_list = if assets.is_empty() {
        "pending — the paper body names no flat CDN asset".to_string()
    } else {
        assets.join(", ")
    };
    let code_sha = commit_sha
        .clone()
        .unwrap_or_else(|| "pending — no git commit read".to_string());
    let code_date = commit_date.unwrap_or_else(|| "pending".to_string());

    let data_avail = format!(
        "Data availability. The raw artefacts are flat CDN assets ({}), \
         byte-stable; the exported body carries sha256 {}. All time series and \
         ephemeris bins follow the field contract (ICRS/TDB addresses, force_type, f64).",
        asset_list, body_sha
    );
    let code_avail = format!(
        "Code availability. The full chain (compilers, probes, transfer-entropy \
         estimators, the register) lives in the omegaflow repository under `src/` \
         and `tools/` (Rust std-only). Version: commit {} ({}). License: PolyForm \
         Noncommercial 1.0.0 (see LICENSE in the repository root).",
        code_sha, code_date
    );
    let competing = "Competing interests. The authors declare no competing interests.";
    let authors = "Author contributions. pending — the operator names authorship \
                   and contribution at submission; the header of this paper carries \
                   no author field.";

    let mut statements = String::new();
    statements.push_str("\\section*{Data Availability}\n");
    statements.push_str(&data_avail);
    statements.push_str("\n\n");
    statements.push_str("\\section*{Code Availability}\n");
    statements.push_str(&code_avail);
    statements.push_str("\n\n");
    statements.push_str("\\section*{Competing Interests}\n");
    statements.push_str(competing);
    statements.push_str("\n\n");
    statements.push_str("\\section*{Author Contributions}\n");
    statements.push_str(authors);
    statements.push_str("\n\n");

    let mut doc = String::new();
    doc.push_str("% generated by tools/src/bin/export_latex.rs\n");
    doc.push_str(&format!("% source: docs/paper/{}.md\n", slug));
    doc.push_str(&format!("% body sha256: {}\n", body_sha));
    doc.push_str("\\documentclass[11pt]{article}\n");
    doc.push_str("\\usepackage[margin=1in]{geometry}\n");
    doc.push_str("\\usepackage{amsmath,amssymb}\n");
    doc.push_str("\\usepackage[T1]{fontenc}\n");
    doc.push_str("\\usepackage{microtype}\n");
    doc.push_str("\\usepackage{booktabs}\n");
    doc.push_str("\\usepackage{array}\n");
    doc.push_str("\\usepackage{xcolor}\n");
    doc.push_str("\\usepackage{url}\n");
    doc.push_str("\\setlength{\\parindent}{0pt}\n");
    doc.push_str("\\setlength{\\parskip}{0.5em}\n");
    doc.push_str("\\begin{document}\n\n");
    doc.push_str("\\title{");
    doc.push_str(&escape_text(&title_arxiv));
    doc.push_str("}\n");
    doc.push_str("\\author{omegaflow field}\n");
    doc.push_str(&format!(
        "\\date{{{}}}\n",
        escape_text(&hdr.date.clone().unwrap_or_default())
    ));
    doc.push_str("\\maketitle\n\n");
    if !abstract_text.is_empty() {
        doc.push_str("\\begin{abstract}\n");
        doc.push_str(&render_inline(&abstract_text));
        doc.push_str("\n\\end{abstract}\n\n");
    }
    doc.push_str(&latex_body);
    doc.push_str("\n");
    doc.push_str(MARKER_STATEMENTS);
    doc.push_str("\n\n");
    doc.push_str(&statements);
    doc.push_str("\\end{document}\n");

    let mut exported_body = String::new();
    if !abstract_text.is_empty() {
        exported_body.push_str(&abstract_text);
        exported_body.push(' ');
    }
    exported_body.push_str(&latex_body);

    let verif_body: Vec<&str> = body
        .lines()
        .filter(|l| !l.trim_start().starts_with("# "))
        .collect();
    let verif_body = verif_body.join("\n");
    let body_nums = numeric_tokens(&verif_body);
    let tex_nums = numeric_tokens(&exported_body);

    let mut body_sorted = body_nums.clone();
    let mut tex_sorted = tex_nums.clone();
    body_sorted.sort();
    tex_sorted.sort();
    let numbers_match = body_sorted == tex_sorted;
    let number_count = body_nums.len();
    if !numbers_match && std::env::var("EXPORT_DEBUG").is_ok() {
        let mut bi = 0usize;
        let mut ti = 0usize;
        while bi < body_sorted.len() && ti < tex_sorted.len() {
            if body_sorted[bi] == tex_sorted[ti] {
                bi += 1;
                ti += 1;
            } else if body_sorted[bi] < tex_sorted[ti] {
                eprintln!("DBG[{}] only-in-body: {}", slug, body_sorted[bi]);
                bi += 1;
            } else {
                eprintln!("DBG[{}] only-in-tex: {}", slug, tex_sorted[ti]);
                ti += 1;
            }
        }
        while bi < body_sorted.len() {
            eprintln!("DBG[{}] only-in-body: {}", slug, body_sorted[bi]);
            bi += 1;
        }
        while ti < tex_sorted.len() {
            eprintln!("DBG[{}] only-in-tex: {}", slug, tex_sorted[ti]);
            ti += 1;
        }
    }

    let abstract_words = count_words(&abstract_text);
    let abstract_ok = abstract_words <= MAX_ABSTRACT_WORDS;

    if !check_only {
        fs::create_dir_all(out_dir).expect("create export dir");
        let out_file = out_dir.join(format!("{}.tex", slug));
        fs::write(&out_file, doc).expect("write tex");
    }

    PaperReport {
        slug,
        title_raw_chars,
        title_arxiv_chars,
        title_ok,
        abstract_words,
        abstract_ok,
        body_sha,
        header_sha,
        sha_match,
        numbers_match,
        number_count,
    }
}

fn numeric_tokens(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut norm = String::new();
    let mut i = 0;
    while i < chars.len() {
        if superscript_value(chars[i]).is_some() || subscript_value(chars[i]).is_some() {
            let mut run = String::new();
            while i < chars.len()
                && (superscript_value(chars[i]).is_some() || subscript_value(chars[i]).is_some())
            {
                if let Some(v) = superscript_value(chars[i]) {
                    run.push(v);
                } else if let Some(v) = subscript_value(chars[i]) {
                    run.push(v);
                }
                i += 1;
            }
            norm.push(' ');
            norm.push_str(&run);
            norm.push(' ');
        } else {
            norm.push(chars[i]);
            i += 1;
        }
    }
    let chars: Vec<char> = norm.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            let start = i;
            let mut j = i;
            while j < chars.len() && chars[j].is_ascii_digit() {
                j += 1;
            }
            if j + 1 < chars.len() && chars[j] == '.' && chars[j + 1].is_ascii_digit() {
                j += 1;
                while j < chars.len() && chars[j].is_ascii_digit() {
                    j += 1;
                }
            }
            out.push(chars[start..j].iter().collect());
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let out_dir = PathBuf::from("docs/paper/export");

    let check_only = args.iter().any(|s| s == "--check");
    args.retain(|s| s != "--check");

    if args.first().map(|s| s.as_str()) == Some("--selftest") {
        println!("sha empty = {}", sha256_hex(b""));
        println!("sha abc   = {}", sha256_hex(b"abc"));
        println!("expect empty = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        println!("expect abc   = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        return;
    }

    if args.is_empty() {
        let mut list: Vec<String> = Vec::new();
        if let Ok(rd) = fs::read_dir("docs/paper") {
            for e in rd.flatten() {
                let p = e.path();
                if p.extension().map(|x| x == "md").unwrap_or(false) {
                    list.push(p.to_string_lossy().to_string());
                }
            }
        }
        list.sort();
        args = list;
    }

    println!("EXPORT  title<=75 abstract<=200 nums-match assets  sha-match");
    let mut any_diff = false;
    for arg in &args {
        let p = Path::new(arg);
        if !p.exists() {
            eprintln!("no such paper: {}", arg);
            continue;
        }
        let r = build_document(p, &out_dir, check_only);
        println!(
            "{:42} {:>5}/{:>5}/{:>4} {:>4}/{:>4} {:>7} {:>5} {:>8}",
            r.slug,
            r.title_raw_chars,
            r.title_arxiv_chars,
            if r.title_ok { "ok" } else { "long" },
            r.abstract_words,
            if r.abstract_ok { "ok" } else { "long" },
            r.number_count,
            if r.numbers_match { "ok" } else { "DIFF" },
            r.header_sha,
        );
        if !r.title_ok || !r.abstract_ok || !r.numbers_match || !r.sha_match {
            any_diff = true;
            println!(
                "    -> {}: title={} (arxiv={}), abstract={}w, nums={}, sha={}/{}",
                r.slug,
                r.title_raw_chars,
                r.title_arxiv_chars,
                r.abstract_words,
                if r.numbers_match { "ok" } else { "DIFF" },
                r.body_sha,
                if r.header_sha.is_empty() {
                    "absent"
                } else {
                    r.header_sha.as_str()
                },
            );
        }
    }
    if !check_only {
        println!("\noutput dir: {}", out_dir.display());
    }
    if any_diff {
        eprintln!("one or more papers carry a named difference (not silently fixed)");
        std::process::exit(1);
    }
}
