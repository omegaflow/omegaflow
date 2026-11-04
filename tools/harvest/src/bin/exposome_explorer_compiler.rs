use omegaflow::inflate::inflate;
use std::process::Command;

const NETLOC: &str = "exposome-explorer.iarc.fr";
const URL: &str =
    "https://exposome-explorer.iarc.fr/system/downloads/current/environmental_pollutants.csv.zip";
const MEMBER_SUFFIX: &str = ".csv";
const ZIP64_SENTINEL: usize = 0xFFFF_FFFF;
const TIME_TOKENS: [&str; 8] = [
    "date",
    "time",
    "year",
    "epoch",
    "period",
    "month",
    "day",
    "timestamp",
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn download(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSLf")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "exposome_explorer_compiler: fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn le16(d: &[u8], off: usize) -> usize {
    d[off] as usize | (d[off + 1] as usize) << 8
}

fn le32(d: &[u8], off: usize) -> u32 {
    (d[off] as u32)
        | (d[off + 1] as u32) << 8
        | (d[off + 2] as u32) << 16
        | (d[off + 3] as u32) << 24
}

fn le64(d: &[u8], off: usize) -> u64 {
    let mut v = 0u64;
    for k in 0..8 {
        v |= (d[off + k] as u64) << (8 * k);
    }
    v
}

struct LocalEntry {
    name: String,
    method: usize,
    data_start: usize,
    comp_size: usize,
}

fn local_entries(data: &[u8]) -> Vec<LocalEntry> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 30 <= data.len() {
        if data.get(i..i + 4) != Some(b"PK\x03\x04") {
            i += 1;
            continue;
        }
        let method = le16(data, i + 8);
        let mut comp_size = le32(data, i + 18) as usize;
        let uncomp_size = le32(data, i + 22) as usize;
        let name_len = le16(data, i + 26);
        let extra_len = le16(data, i + 28);
        if i + 30 + name_len > data.len() {
            break;
        }
        let name = String::from_utf8_lossy(&data[i + 30..i + 30 + name_len]).into_owned();
        let extra_start = i + 30 + name_len;
        let extra_end = (extra_start + extra_len).min(data.len());
        let extra = &data[extra_start..extra_end];
        let mut e = 0usize;
        while e + 4 <= extra.len() {
            let id = le16(extra, e);
            let sz = le16(extra, e + 2);
            let field_end = (e + 4 + sz).min(extra.len());
            if id == 0x0001 {
                let base = e + 4;
                if comp_size == ZIP64_SENTINEL {
                    let at = if uncomp_size == ZIP64_SENTINEL {
                        base + 8
                    } else {
                        base
                    };
                    if at + 8 <= field_end {
                        comp_size = le64(extra, at) as usize;
                    }
                }
            }
            e += 4 + sz;
        }
        let data_start = i + 30 + name_len + extra_len;
        if data_start + comp_size <= data.len() {
            out.push(LocalEntry {
                name,
                method,
                data_start,
                comp_size,
            });
        }
        i = data_start + comp_size;
        if comp_size == 0 {
            i += 1;
        }
    }
    out
}

fn member_bytes(data: &[u8], entry: &LocalEntry) -> Option<Vec<u8>> {
    let body = data.get(entry.data_start..entry.data_start + entry.comp_size)?;
    match entry.method {
        0 => Some(body.to_vec()),
        8 => inflate(body),
        _ => None,
    }
}

fn split_csv_line(line: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if quoted && chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    quoted = !quoted;
                }
            }
            ',' if !quoted => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn truncate(s: &str, max: usize) -> String {
    let mut out: String = s.chars().take(max).collect();
    if s.chars().count() > max {
        out.push('…');
    }
    out
}

enum ColKind {
    Numeric,
    Text,
    Empty,
}

fn kind_name(kind: &ColKind) -> &'static str {
    match kind {
        ColKind::Numeric => "numeric",
        ColKind::Text => "text",
        ColKind::Empty => "empty",
    }
}

fn is_identifier(name: &str) -> bool {
    let l = name.to_ascii_lowercase();
    l == "id" || l == "name" || l.contains("cas")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => URL.to_string(),
    };

    let bytes = match download(&url) {
        Some(b) => b,
        None => {
            eprintln!("exposome_explorer_compiler: {url}: fetch void — no schema read (0 honored)");
            std::process::exit(1);
        }
    };

    let entries = local_entries(&bytes);
    let Some(entry) = entries.iter().find(|e| e.name.ends_with(MEMBER_SUFFIX)) else {
        eprintln!(
            "exposome_explorer_compiler: {url}: no {MEMBER_SUFFIX} member among {} zip member(s) — 0 honored",
            entries.len()
        );
        std::process::exit(1);
    };
    let Some(csv) = member_bytes(&bytes, entry) else {
        eprintln!(
            "exposome_explorer_compiler: {url}: member {} stays unreadable ({}) — 0 honored",
            entry.name, entry.method
        );
        std::process::exit(1);
    };
    let text = match String::from_utf8(csv) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("exposome_explorer_compiler: {url}: the csv member is not utf-8");
            std::process::exit(1);
        }
    };

    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let Some(header_line) = lines.next() else {
        eprintln!("exposome_explorer_compiler: {url}: the csv carries no header line — 0 honored");
        std::process::exit(1);
    };
    let header = split_csv_line(header_line);
    let rows: Vec<Vec<String>> = lines.map(split_csv_line).collect();

    let mut kinds: Vec<ColKind> = Vec::with_capacity(header.len());
    for j in 0..header.len() {
        let mut non_empty = 0usize;
        let mut numeric = 0usize;
        for row in &rows {
            if let Some(cell) = row.get(j) {
                let t = cell.trim();
                if !t.is_empty() {
                    non_empty += 1;
                    if is_number(t) {
                        numeric += 1;
                    }
                }
            }
        }
        kinds.push(if non_empty == 0 {
            ColKind::Empty
        } else if numeric == non_empty {
            ColKind::Numeric
        } else {
            ColKind::Text
        });
    }

    let time_cols: Vec<&String> = header
        .iter()
        .filter(|h| {
            let l = h.to_ascii_lowercase();
            TIME_TOKENS.iter().any(|t| l.contains(t))
        })
        .collect();

    println!(
        "exposome_explorer_compiler: {url} -> {} B, {} zip member(s), {} data row(s)",
        bytes.len(),
        entries.len(),
        rows.len()
    );
    println!("origin {NETLOC}");
    println!("schema: {} columns", header.len());
    for (j, name) in header.iter().enumerate() {
        let tag = if is_identifier(name) {
            "identifier"
        } else {
            "-"
        };
        println!("  [{j}] {name} | {} | {tag}", kind_name(&kinds[j]));
    }
    for (i, row) in rows.iter().take(3).enumerate() {
        let cells: Vec<String> = row.iter().map(|c| truncate(c, 40)).collect();
        println!("sample[{i}]: {}", cells.join(" | "));
    }

    if !time_cols.is_empty() {
        eprintln!(
            "exposome_explorer_compiler: time axis columns {:?} — the axis-value emission is the next bound step, not built here (0 honored)",
            time_cols
        );
        std::process::exit(1);
    }

    eprintln!(
        "exposome_explorer_compiler: STATIC reference table — no column carries a time axis ({TIME_TOKENS:?}); no epoch value series can be derived (0 honored)"
    );
    eprintln!(
        "missing design: the manifestation form of a static chemical reference catalog in omegaflow (identity/mass/publication-count rows, no epoch, no measured concentration) — catalog/register entry vs. per-row oscillator is an operator/council decision; not invented here"
    );
    std::process::exit(1);
}
