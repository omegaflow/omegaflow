use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

const REG: &str = "tools/register/data/instruments.φ";
const PHI: &str = "phi/sources.φ";

fn blocks(text: &str) -> Vec<(usize, Vec<String>)> {
    let mut out = Vec::new();
    let mut cur: Vec<String> = Vec::new();
    let mut start = 0usize;
    for (idx, line) in text.lines().enumerate() {
        let begins = line.starts_with("host ") || line.starts_with("zustand ");
        if begins && !cur.is_empty() {
            out.push((start, std::mem::take(&mut cur)));
        }
        if line.trim().is_empty() {
            if !cur.is_empty() {
                out.push((start, std::mem::take(&mut cur)));
            }
            continue;
        }
        if cur.is_empty() {
            start = idx + 1;
        }
        cur.push(line.to_string());
    }
    if !cur.is_empty() {
        out.push((start, cur));
    }
    out
}

fn field(block: &[String], name: &str) -> Option<String> {
    block.iter().find_map(|l| {
        let p: Vec<&str> = l.split_whitespace().collect();
        (p.first() == Some(&name)).then(|| p[1..].join(" "))
    })
}

fn host_of(block: &[String]) -> Option<String> {
    let url = field(block, "applies").or_else(|| field(block, "probe"))?;
    let rest = url.split_once("://")?.1;
    let host = rest.split(['/', '?', ':']).next()?;
    Some(host.trim_start_matches("www.").to_string())
}

fn product_of(block: &[String]) -> Option<String> {
    let applies = field(block, "applies")?;
    if let Some(i) = applies.find("table=") {
        return Some(applies[i + 6..].split_whitespace().next()?.to_string());
    }
    if let Some(i) = applies.find("source=") {
        return Some(applies[i + 7..].split('&').next()?.to_string());
    }
    applies.split_whitespace().next().map(str::to_string)
}

fn cites(block: &[String]) -> usize {
    block
        .iter()
        .filter(|l| {
            let p: Vec<&str> = l.split_whitespace().collect();
            matches!(
                p.first(),
                Some(&"api_cite") | Some(&"instrument_cite") | Some(&"host_cite")
            )
        })
        .count()
}

fn normalize(block: &mut [String]) {
    for line in block.iter_mut() {
        if let Some(rest) = line.strip_prefix("  cadence ") {
            *line = format!("  interval {rest}");
        }
        if line.starts_with("  probe probe ") {
            *line = line.replacen("  probe probe ", "  probe ", 1);
        }
    }
}

fn merge_rows(out_path: &str, state_path: &str) {
    let text = fs::read_to_string(out_path).expect("out file");
    let mut reg_text = fs::read_to_string(REG).expect("register");
    if !reg_text.ends_with('\n') {
        reg_text.push('\n');
    }
    let mut rows = blocks(&reg_text);
    rows.retain(|(_, b)| b.first().is_some_and(|l| l.starts_with("host ")));
    let mut seen: BTreeMap<(String, String), usize> = BTreeMap::new();
    for (i, (_, b)) in rows.iter().enumerate() {
        if let (Some(h), Some(p)) = (host_of(b), product_of(b)) {
            seen.entry((h, p)).or_insert(i);
        }
    }
    let mut states = String::new();
    let mut added = 0usize;
    let mut replaced = 0usize;
    for (_, mut b) in blocks(&text) {
        if b.first().is_some_and(|l| l.starts_with("zustand ")) {
            states.push_str(&b.join("\n"));
            states.push_str("\n\n");
            continue;
        }
        normalize(&mut b);
        if let (Some(h), Some(p)) = (host_of(&b), product_of(&b)) {
            let inst = match field(&b, "instrument") {
                Some(v) => v,
                None => String::new(),
            };
            let key = (h.clone(), format!("{p}\t{inst}"));
            if let Some(&i) = seen.get(&key) {
                if cites(&b) > cites(&rows[i].1) {
                    rows[i].1 = b;
                    replaced += 1;
                }
                continue;
            }
            seen.insert(key, rows.len());
            rows.push((0, b));
            added += 1;
        }
    }
    let mut out = String::new();
    for (_, b) in &rows {
        out.push_str(&b.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    if !states.is_empty() {
        let mut old = match fs::read_to_string(state_path) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        if !old.is_empty() && !old.ends_with("\n\n") {
            old.push_str("\n\n");
        }
        old.push_str(&states);
        fs::write(state_path, old).expect("states write");
    }
    println!("rows: +{added} neu, {replaced} ersetzt, {} gesamt", rows.len());
}

fn apply_instruments(tsv_path: &str) {
    let tsv = fs::read_to_string(tsv_path).expect("tsv");
    let mut map: BTreeMap<usize, String> = BTreeMap::new();
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(n), Some(inst)) = (it.next(), it.next()) else {
            continue;
        };
        let Ok(n) = n.trim().parse::<usize>() else {
            continue;
        };
        if inst.trim().eq_ignore_ascii_case("pending") || inst.trim() == "-" {
            continue;
        }
        map.insert(n, inst.trim().to_string());
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let starts: Vec<usize> = blocks(&text).into_iter().map(|(s, _)| s).collect();
    let mut added = 0usize;
    let mut ins: Vec<(usize, String)> = Vec::new();
    for (idx, start) in starts.iter().enumerate() {
        let Some(inst) = map.get(start) else {
            continue;
        };
        let end = starts.get(idx + 1).map(|s| s - 1).unwrap_or(lines.len());
        let has = lines[start - 1..end.min(lines.len())]
            .iter()
            .any(|l| l.starts_with("instrument "));
        if has {
            continue;
        }
        if let Some(pos) = lines[start - 1..end.min(lines.len())]
            .iter()
            .position(|l| l.starts_with("origin "))
        {
            ins.push((start + pos, format!("instrument {inst}")));
        }
    }
    for (at, line) in ins.into_iter().rev() {
        lines.insert(at, line);
        added += 1;
    }
    fs::write(PHI, lines.join("\n") + "\n").expect("phi write");
    println!("instrument: {added} eingesetzt");
}

fn host_docs() {
    let text = fs::read_to_string(REG).expect("register");
    let mut out = String::new();
    let mut docs = 0usize;
    let mut failed = 0usize;
    let mut rows = 0usize;
    let mut with_probe = 0usize;
    for (_, mut b) in blocks(&text) {
        rows += 1;
        if field(&b, "probe").is_some() {
            with_probe += 1;
        }
        if b.iter().any(|l| l.starts_with("  host_cite ")) {
            out.push_str(&b.join("\n"));
            out.push_str("\n\n");
            continue;
        }
        let Some(probe) = field(&b, "probe").and_then(|p| p.split_whitespace().last().map(str::to_string)) else {
            out.push_str(&b.join("\n"));
            out.push_str("\n\n");
            continue;
        };
        let mut headers = String::new();
        for args in [vec!["-sSI"], vec!["-sS", "-r", "0-0"]] {
            let mut cmd = Command::new("curl");
            cmd.args(["--max-time", "15"]);
            cmd.args(&args);
            cmd.arg(&probe);
            let Ok(res) = cmd.output() else { continue };
            headers = String::from_utf8_lossy(&res.stdout).to_string();
            if headers.starts_with("HTTP/") {
                break;
            }
            headers.clear();
        }
        let mut quote = String::new();
        for line in headers.lines() {
            let low = line.to_ascii_lowercase();
            if low.starts_with("cache-control:") || low.starts_with("last-modified:") || low.starts_with("etag:") {
                quote = line.trim().to_string();
                if low.starts_with("cache-control:") {
                    break;
                }
            }
        }
        if quote.is_empty() {
            failed += 1;
        } else {
            let at = b
                .iter()
                .position(|l| {
                    l.starts_with("  probe ")
                        || l.starts_with("  applies ")
                        || l.starts_with("  note ")
                })
                .unwrap_or(b.len());
            b.insert(at, format!("  host_cite {probe}"));
            b.insert(at + 1, format!("  host_quote \"{quote}\""));
            docs += 1;
        }
        out.push_str(&b.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    println!("rows {rows}, mit probe {with_probe}, host_cite: {docs} ergaenzt, {failed} ohne Header");
}

fn url_id(url: &str) -> Option<String> {
    let rest = url.split_once("id=")?.1;
    let end = rest.find('&').unwrap_or(rest.len());
    let id = &rest[..end];
    if id.is_empty() {
        return None;
    }
    Some(id.replace("%40", "@").replace("%2F", "/").replace("%2f", "/"))
}

fn probe_fill() {
    let reg_text = fs::read_to_string(REG).expect("register");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let mut rows = blocks(&reg_text);
    let mut filled = 0usize;
    let mut failed = 0usize;
    let mut considered = 0usize;
    let mut no_row = 0usize;
    let mut seen = 0usize;
    for (_, block) in blocks(&phi_text) {
        let has_fields = block.iter().any(|l| {
            let p: Vec<&str> = l.split_whitespace().collect();
            matches!(p.first(), Some(&"field") | Some(&"quantity"))
        });
        if !has_fields {
            continue;
        }
        let Some(origin) = field(&block, "origin") else {
            continue;
        };
        let Some(url) = origin.split_whitespace().next() else {
            continue;
        };
        let Some(host) = url
            .split_once("://")
            .and_then(|(_, r)| r.split(['/', '?', ':']).next())
            .map(|h| h.trim_start_matches("www.").to_string())
        else {
            continue;
        };
        let stem = url.split(['?', '{']).next().unwrap_or(url);
        let mut best: Option<(usize, usize)> = None;
        for (i, (_, r)) in rows.iter().enumerate() {
            if host_of(r).as_deref() != Some(host.as_str()) {
                continue;
            }
            let Some(applies) = field(r, "applies") else {
                continue;
            };
            if !(url.starts_with(&applies) || applies.starts_with(stem)) {
                continue;
            }
            if best.is_none_or(|(_, len)| applies.len() > len) {
                best = Some((i, applies.len()));
            }
        }
        let Some((i, _)) = best else {
            no_row += 1;
            continue;
        };
        considered += 1;
        let covered = rows[i]
            .1
            .iter()
            .filter(|l| l.trim_start().starts_with("probe "))
            .filter_map(|l| l.split_whitespace().nth(3))
            .any(|p| {
                let same_arm = match (url_id(p), url_id(url)) {
                    (Some(a), Some(b)) => a == b,
                    _ => true,
                };
                same_arm && (p.starts_with(stem) || stem.starts_with(p))
            });
        if covered {
            seen += 1;
            continue;
        }
        let out = Command::new("bin/archive_search")
            .arg("--probe")
            .arg(url)
            .output();
        let Ok(out) = out else {
            failed += 1;
            continue;
        };
        let text = String::from_utf8_lossy(&out.stdout);
        let Some(line) = text.lines().find(|l| l.trim_start().starts_with("probe ")) else {
            failed += 1;
            continue;
        };
        let at = rows[i]
            .1
            .iter()
            .position(|l| l.starts_with("  applies "))
            .unwrap_or(rows[i].1.len());
        rows[i].1.insert(at, format!("  {}", line.trim()));
        filled += 1;
    }
    let mut out = String::new();
    for (_, b) in &rows {
        out.push_str(&b.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    println!("probe: {filled} ergaenzt, {failed} ohne Antwort, {seen} schon belegt, {no_row} ohne Zeile, {considered} mit Zeile");
}

fn probe_drop_failed() {
    let text = fs::read_to_string(REG).expect("register");
    let mut out = String::new();
    let mut dropped = 0usize;
    for (_, b) in blocks(&text) {
        let mut keep: Vec<String> = Vec::new();
        for line in b {
            let p: Vec<&str> = line.split_whitespace().collect();
            let failed = p.first() == Some(&"probe")
                && p.get(2)
                    .and_then(|s| s.parse::<u16>().ok())
                    .is_some_and(|s| s >= 400);
            if failed {
                dropped += 1;
                continue;
            }
            keep.push(line);
        }
        out.push_str(&keep.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    println!("probe: {dropped} Fehlschlaege entfernt");
}

fn probes_from_tsv(path: &str) {
    let tsv = fs::read_to_string(path).expect("tsv");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let mut rows = blocks(&fs::read_to_string(REG).expect("register"));
    let phi_blocks = blocks(&phi_text);
    let mut added = 0usize;
    let mut missed = 0usize;
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(n), Some(probe)) = (it.next(), it.next()) else {
            continue;
        };
        let Ok(line_no) = n.trim().parse::<usize>() else {
            continue;
        };
        let probe = probe.trim();
        if !probe.starts_with("probe ") {
            continue;
        }
        let Some((_, block)) = phi_blocks.iter().find(|(s, _)| *s == line_no) else {
            missed += 1;
            continue;
        };
        let Some(origin) = field(block, "origin") else {
            missed += 1;
            continue;
        };
        let Some(url) = origin.split_whitespace().next() else {
            missed += 1;
            continue;
        };
        let Some(host) = url
            .split_once("://")
            .and_then(|(_, r)| r.split(['/', '?', ':']).next())
            .map(|h| h.trim_start_matches("www.").to_string())
        else {
            missed += 1;
            continue;
        };
        let stem = url.split(['?', '{']).next().unwrap_or(url);
        let mut best: Option<(usize, usize)> = None;
        for (i, (_, r)) in rows.iter().enumerate() {
            if host_of(r).as_deref() != Some(host.as_str()) {
                continue;
            }
            let Some(applies) = field(r, "applies") else {
                continue;
            };
            if !(url.starts_with(&applies) || applies.starts_with(stem)) {
                continue;
            }
            if best.is_none_or(|(_, len)| applies.len() > len) {
                best = Some((i, applies.len()));
            }
        }
        let Some((i, _)) = best else {
            missed += 1;
            continue;
        };
        let probe_url = probe.split_whitespace().nth(3).unwrap_or("");
        let exists = rows[i]
            .1
            .iter()
            .filter(|l| l.trim_start().starts_with("probe "))
            .any(|l| l.split_whitespace().nth(3) == Some(probe_url));
        if exists {
            continue;
        }
        let at = rows[i]
            .1
            .iter()
            .position(|l| l.starts_with("  applies "))
            .unwrap_or(rows[i].1.len());
        rows[i].1.insert(at, format!("  {probe}"));
        added += 1;
    }
    let mut out = String::new();
    for (_, b) in &rows {
        out.push_str(&b.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    println!("probes: {added} uebernommen, {missed} ohne Zeile");
}

fn fill_instruments() {
    let reg_text = fs::read_to_string(REG).expect("register");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let rows = blocks(&reg_text);
    let phi_blocks = blocks(&phi_text);
    let mut lines: Vec<String> = phi_text.lines().map(str::to_string).collect();
    let mut done = 0usize;
    let mut ambiguous = 0usize;
    let mut insertions: Vec<(usize, String)> = Vec::new();
    for (idx, (start, block)) in phi_blocks.iter().enumerate() {
        let has_fields = block.iter().any(|l| {
            let p: Vec<&str> = l.split_whitespace().collect();
            matches!(p.first(), Some(&"field") | Some(&"quantity"))
        });
        if !has_fields || block.iter().any(|l| l.starts_with("instrument ")) {
            continue;
        }
        let Some(origin) = field(block, "origin") else {
            continue;
        };
        let Some(url) = origin.split_whitespace().next() else {
            continue;
        };
        let Some(host) = url
            .split_once("://")
            .and_then(|(_, r)| r.split(['/', '?', ':']).next())
            .map(|h| h.trim_start_matches("www.").to_string())
        else {
            continue;
        };
        let stem = url.split(['?', '{']).next().unwrap_or(url);
        let mut names: Vec<String> = Vec::new();
        for (_, r) in &rows {
            if host_of(r).as_deref() != Some(host.as_str()) {
                continue;
            }
            let Some(applies) = field(r, "applies") else {
                continue;
            };
            if !url.starts_with(&applies) {
                continue;
            }
            if let Some(inst) = field(r, "instrument")
                && inst != "-"
                && !names.iter().any(|n| n.eq_ignore_ascii_case(&inst))
            {
                names.push(inst);
            }
        }
        if names.is_empty() {
            continue;
        }
        if names.len() > 1 {
            ambiguous += 1;
        }
        let end = phi_blocks
            .get(idx + 1)
            .map(|(s, _)| s - 1)
            .unwrap_or(lines.len());
        if let Some(pos) = lines[start - 1..end.min(lines.len())]
            .iter()
            .position(|l| l.starts_with("origin "))
        {
            for (i, name) in names.iter().enumerate() {
                insertions.push((start + pos + i, format!("instrument {name}")));
                done += 1;
            }
        }
    }
    for (at, line) in insertions.into_iter().rev() {
        lines.insert(at, line);
    }
    fs::write(PHI, lines.join("\n") + "\n").expect("phi write");
    println!("instrument: {done} eindeutig eingesetzt, {ambiguous} mehrdeutig");
}

fn clean() {
    let text = fs::read_to_string(REG).expect("register");
    let mut out = String::new();
    let mut removed = 0usize;
    for (_, b) in blocks(&text) {
        let mut keep: Vec<String> = Vec::new();
        for mut line in b {
            let p: Vec<&str> = line.split_whitespace().collect();
            if matches!(p.first(), Some(&"sampling") | Some(&"interval"))
                && p.get(1) == Some(&"none")
            {
                removed += 1;
                continue;
            }
            if p.first() == Some(&"instrument") {
                let rest = p[1..].join(" ");
                if rest.to_ascii_lowercase().starts_with("6df") {
                    line = "  instrument 6dF".to_string();
                }
            }
            if let Some(rest) = line.strip_prefix("  applies ") {
                let mut parts: Vec<&str> = rest.split_whitespace().collect();
                while let Some(last) = parts.last() {
                    if last.starts_with("id=") && last[3..].chars().all(|c| c.is_ascii_digit()) {
                        parts.pop();
                    } else {
                        break;
                    }
                }
                line = format!("  applies {}", parts.join(" "));
            }
            keep.push(line);
        }
        out.push_str(&keep.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    println!("clean: {removed} none-Zeilen entfernt");
}

fn repair_instruments() {
    let reg_text = fs::read_to_string(REG).expect("register");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let rows = blocks(&reg_text);
    let mut lines: Vec<String> = phi_text.lines().map(str::to_string).collect();
    let mut drop: Vec<usize> = Vec::new();
    for (start, block) in blocks(&phi_text) {
        let insts: Vec<(usize, String)> = block
            .iter()
            .enumerate()
            .filter_map(|(i, l)| l.strip_prefix("instrument ").map(|v| (start + i, v.to_string())))
            .collect();
        if insts.len() < 2 {
            continue;
        }
        let Some(origin) = field(&block, "origin") else {
            continue;
        };
        let Some(url) = origin.split_whitespace().next() else {
            continue;
        };
        let Some(host) = url
            .split_once("://")
            .and_then(|(_, r)| r.split(['/', '?', ':']).next())
            .map(|h| h.trim_start_matches("www.").to_string())
        else {
            continue;
        };
        let mut row_insts: Vec<String> = Vec::new();
        for (_, r) in &rows {
            if host_of(r).as_deref() != Some(host.as_str()) {
                continue;
            }
            let Some(applies) = field(r, "applies") else {
                continue;
            };
            if !url.starts_with(&applies) {
                continue;
            }
            if let Some(inst) = field(r, "instrument")
                && !row_insts.iter().any(|n| n.eq_ignore_ascii_case(&inst))
            {
                row_insts.push(inst);
            }
        }
        let keep: Vec<&(usize, String)> = insts
            .iter()
            .filter(|(_, i)| row_insts.iter().any(|r| r.eq_ignore_ascii_case(i)))
            .collect();
        if keep.len() == 1 {
            for (ln, _) in &insts {
                if *ln != keep[0].0 {
                    drop.push(*ln);
                }
            }
        }
    }
    drop.sort_unstable();
    drop.dedup();
    for ln in drop.iter().rev() {
        lines.remove(ln - 1);
    }
    fs::write(PHI, lines.join("\n") + "\n").expect("phi write");
    println!("repair: {} falsche Instrumentzeilen entfernt", drop.len());
}

fn backfill_fields() {
    let reg_text = fs::read_to_string(REG).expect("register");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let phi_blocks = blocks(&phi_text);
    let mut rows = blocks(&reg_text);
    let mut filled = 0usize;
    for (_, row) in rows.iter_mut() {
        if row.iter().any(|l| l.starts_with("  fields ")) {
            continue;
        }
        let Some(applies) = field(row, "applies") else {
            continue;
        };
        let Some(host) = host_of(row) else {
            continue;
        };
        let mut keys: Vec<String> = Vec::new();
        for (_, block) in &phi_blocks {
            let Some(origin) = field(block, "origin") else {
                continue;
            };
            let Some(url) = origin.split_whitespace().next() else {
                continue;
            };
            let Some(bh) = url
                .split_once("://")
                .and_then(|(_, r)| r.split(['/', '?', ':']).next())
                .map(|h| h.trim_start_matches("www.").to_string())
            else {
                continue;
            };
            if bh != host {
                continue;
            }
            let stem = url.split(['?', '{']).next().unwrap_or(url);
            if !(url.starts_with(&applies) || applies.starts_with(stem)) {
                continue;
            }
            for line in block {
                let p: Vec<&str> = line.split_whitespace().collect();
                if matches!(p.first(), Some(&"field") | Some(&"quantity"))
                    && let Some(k) = p.get(1)
                {
                    let k = k.trim_matches('"').to_string();
                    if !keys.iter().any(|x| x.eq_ignore_ascii_case(&k)) {
                        keys.push(k);
                    }
                }
            }
        }
        if keys.is_empty() {
            continue;
        }
        keys.sort();
        let at = row
            .iter()
            .position(|l| l.starts_with("  applies "))
            .unwrap_or(row.len());
        row.insert(at, format!("  fields {}", keys.join(" ")));
        filled += 1;
    }
    let mut out = String::new();
    for (_, b) in &rows {
        out.push_str(&b.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    println!("fields: {filled} Zeilen zugeordnet");
}

fn align_instruments() {
    let reg_text = fs::read_to_string(REG).expect("register");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let rows = blocks(&reg_text);
    let mut lines: Vec<String> = phi_text.lines().map(str::to_string).collect();
    let mut fixed = 0usize;
    for (start, block) in blocks(&phi_text) {
        let insts: Vec<(usize, String)> = block
            .iter()
            .enumerate()
            .filter_map(|(i, l)| l.strip_prefix("instrument ").map(|v| (start + i, v.to_string())))
            .collect();
        if insts.len() != 1 {
            continue;
        }
        let keys: Vec<String> = block
            .iter()
            .filter_map(|l| {
                let p: Vec<&str> = l.split_whitespace().collect();
                matches!(p.first(), Some(&"field") | Some(&"quantity"))
                    .then(|| p.get(1).map(|k| k.trim_matches('"').to_string()))
                    .flatten()
            })
            .collect();
        let mut names: Vec<String> = Vec::new();
        for (_, r) in &rows {
            let rkeys: Vec<&str> = r
                .iter()
                .filter_map(|l| l.strip_prefix("  fields ").map(str::trim))
                .flat_map(|s| s.split_whitespace())
                .collect();
            if !keys.iter().any(|k| rkeys.iter().any(|x| x.eq_ignore_ascii_case(k))) {
                continue;
            }
            if let Some(inst) = field(r, "instrument")
                && !names.iter().any(|n| n.eq_ignore_ascii_case(&inst))
            {
                names.push(inst);
            }
        }
        if names.len() == 1 && !names[0].eq_ignore_ascii_case(&insts[0].1) {
            lines[insts[0].0 - 1] = format!("instrument {}", names[0]);
            fixed += 1;
        }
    }
    fs::write(PHI, lines.join("\n") + "\n").expect("phi write");
    println!("align: {fixed} Blockinstrumente auf die Spec gezogen");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("rows") if args.len() >= 4 => merge_rows(&args[2], &args[3]),
        Some("instruments") if args.len() >= 3 => apply_instruments(&args[2]),
        Some("hostdocs") => host_docs(),
        Some("probe-fill") => probe_fill(),
        Some("probe-drop-failed") => probe_drop_failed(),
        Some("probes") if args.len() >= 3 => probes_from_tsv(&args[2]),
        Some("fill-instruments") => fill_instruments(),
        Some("clean") => clean(),
        Some("backfill-fields") => backfill_fields(),
        Some("align-instruments") => align_instruments(),
        Some("repair-instruments") => repair_instruments(),
        _ => {
            eprintln!("spec_merge rows <out.φ> <states.φ> | instruments <tsv> | hostdocs");
            std::process::exit(2);
        }
    }
}
