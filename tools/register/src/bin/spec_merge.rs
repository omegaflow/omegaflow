use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

fn or_empty(v: Option<String>) -> String {
    match v {
        Some(s) => s,
        None => String::new(),
    }
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
    Some(applies.clone())
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
    write_out(REG, out, "register");
    if !states.is_empty() {
        let mut old = match fs::read_to_string(state_path) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        if !old.is_empty() && !old.ends_with("\n\n") {
            old.push_str("\n\n");
        }
        old.push_str(&states);
        write_out(state_path, old, "states");
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
    write_out(PHI, lines.join("\n") + "\n", "phi");
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
    write_out(REG, out, "register");
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
    write_out(REG, out, "register");
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
    write_out(REG, out, "register");
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
    write_out(REG, out, "register");
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
    write_out(PHI, lines.join("\n") + "\n", "phi");
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
    write_out(REG, out, "register");
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
    write_out(PHI, lines.join("\n") + "\n", "phi");
    println!("repair: {} falsche Instrumentzeilen entfernt", drop.len());
}

fn backfill_fields() {
    let reg_text = fs::read_to_string(REG).expect("register");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let phi_blocks = blocks(&phi_text);
    let mut rows = blocks(&reg_text);
    let mut filled = 0usize;
    for (_, row) in rows.iter_mut() {
        if row.iter().any(|l| l.starts_with("  fields "))
            && !field(row, "applies").is_some_and(|a| a.contains("id="))
            && !field(row, "probe").is_some()
        {
            continue;
        }
        let Some(applies) = field(row, "applies") else {
            continue;
        };
        let Some(host) = host_of(row) else {
            continue;
        };
        let mut keys: Vec<String> = Vec::new();
        for line in row.iter() {
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
        if keys.is_empty() {
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
        }
        if applies.contains("id=")
            && let Some(idpos) = applies.find("id=")
            && let Ok(line_no) = applies[idpos + 3..]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .parse::<usize>()
            && let Some((_, block)) = phi_blocks.iter().find(|(s, _)| *s == line_no)
        {
            keys.clear();
            for line in block {
                let p: Vec<&str> = line.split_whitespace().collect();
                if matches!(p.first(), Some(&"field") | Some(&"quantity") | Some(&"last"))
                    && let Some(k) = p.get(1)
                {
                    let k = k.trim_matches('"').to_string();
                    if !keys.iter().any(|x| x.eq_ignore_ascii_case(&k)) {
                        keys.push(k);
                    }
                }
            }
        }
        if keys.is_empty()
            && let Some(probe) = field(row, "probe")
                .and_then(|p| p.split_whitespace().last().map(str::to_string))
        {
            let pstem = probe.split(['?', '{']).next().unwrap_or(&probe).to_string();
            for (_, block) in &phi_blocks {
                let Some(origin) = field(block, "origin") else {
                    continue;
                };
                let Some(url) = origin.split_whitespace().next() else {
                    continue;
                };
                let ostem = url.split(['?', '{']).next().unwrap_or(url);
                if !(pstem.starts_with(ostem) || ostem.starts_with(&pstem)) {
                    continue;
                }
                for line in block {
                    let p: Vec<&str> = line.split_whitespace().collect();
                    if matches!(p.first(), Some(&"field") | Some(&"quantity") | Some(&"last"))
                        && let Some(k) = p.get(1)
                    {
                        let k = k.trim_matches('"').to_string();
                        if !keys.iter().any(|x| x.eq_ignore_ascii_case(&k)) {
                            keys.push(k);
                        }
                    }
                }
            }
        }
        if keys.is_empty() {
            continue;
        }
        keys.sort();
        row.retain(|l| !l.starts_with("  fields "));
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
    write_out(REG, out, "register");
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
    write_out(PHI, lines.join("\n") + "\n", "phi");
    println!("align: {fixed} Blockinstrumente auf die Spec gezogen");
}

fn apply_fields(path: &str) {
    let tsv = fs::read_to_string(path).expect("tsv");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let mut lines: Vec<String> = phi_text.lines().map(str::to_string).collect();
    let phi_blocks = blocks(&phi_text);
    let mut ins: Vec<(usize, String)> = Vec::new();
    let mut skipped = 0usize;
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(url), Some(fline)) = (it.next(), it.next()) else {
            continue;
        };
        let fline = fline.trim();
        if !(fline.starts_with("field ") || fline.starts_with("quantity ") || fline.starts_with("last ")) {
            continue;
        }
        let Some((start, block)) = phi_blocks
            .iter()
            .find(|(_, b)| field(b, "url").as_deref() == Some(url.trim()))
            .or_else(|| {
                phi_blocks.iter().find(|(_, b)| {
                    let Some(u) = field(b, "url") else {
                        return false;
                    };
                    u.ends_with(&format!("/{}", url.trim().rsplit('/').next().unwrap_or(url.trim())))
                })
            })
        else {
            skipped += 1;
            continue;
        };
        let has = block.iter().any(|l| {
            l.starts_with("field ") || l.starts_with("quantity ") || l.starts_with("last ")
        });
        if has {
            continue;
        }
        if let Some(pos) = block.iter().position(|l| l.starts_with("at ")) {
            ins.push((start + pos, fline.to_string()));
        } else {
            ins.push((start + block.len(), fline.to_string()));
        }
    }
    for (at, line) in ins.iter().rev() {
        lines.insert(*at, line.clone());
    }
    write_out(PHI, lines.join("\n") + "\n", "phi");
    println!("fields: {} Zeilen in Bloecke eingesetzt, {skipped} ohne Block", ins.len());
}

fn instrumentize(path: &str) {
    let src = fs::read_to_string(path).expect("rows");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let mut lines: Vec<String> = phi_text.lines().map(str::to_string).collect();
    let phi_blocks = blocks(&phi_text);
    let mut ins: Vec<(usize, String)> = Vec::new();
    for (_, row) in blocks(&src) {
        let Some(inst) = field(&row, "instrument") else {
            continue;
        };
        let Some(applies) = field(&row, "applies") else {
            continue;
        };
        let url = applies.split_whitespace().next().unwrap_or("");
        let Some((start, block)) = phi_blocks
            .iter()
            .find(|(_, b)| field(b, "url").as_deref() == Some(url))
            .or_else(|| {
                phi_blocks.iter().find(|(_, b)| {
                    let Some(origin) = field(b, "origin") else {
                        return false;
                    };
                    let Some(ourl) = origin.split_whitespace().next() else {
                        return false;
                    };
                    let stem = ourl.split(['?', '{']).next().unwrap_or(ourl);
                    url.starts_with(stem) || stem.starts_with(url)
                })
            })
        else {
            continue;
        };
        if block.iter().any(|l| l.starts_with("instrument ")) {
            continue;
        }
        if let Some(pos) = block.iter().position(|l| l.starts_with("origin ")) {
            ins.push((start + pos, format!("instrument {inst}")));
        }
    }
    for (at, line) in ins.iter().rev() {
        lines.insert(*at, line.clone());
    }
    write_out(PHI, lines.join("\n") + "\n", "phi");
    println!("instrument: {} in Bloecke nach url eingesetzt", ins.len());
}

fn set_instruments(path: &str) {
    use std::collections::BTreeMap;
    let tsv = fs::read_to_string(path).expect("tsv");
    let mut map: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(n), Some(inst)) = (it.next(), it.next()) else {
            continue;
        };
        let Ok(start) = n.trim().parse::<usize>() else {
            continue;
        };
        let names: Vec<String> = if inst.trim() == "-" {
            Vec::new()
        } else {
            inst.split(',').map(|s| s.trim().to_string()).collect()
        };
        map.insert(start, names);
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let mut out: Vec<String> = Vec::new();
    let mut done = 0usize;
    for (start, block) in blocks(&text) {
        let Some(names) = map.get(&start) else {
            out.extend(block);
            out.push(String::new());
            continue;
        };
        done += 1;
        let mut inserted = false;
        for l in block {
            if l.starts_with("instrument ") {
                continue;
            }
            out.push(l.clone());
            if !inserted && l.starts_with("origin ") {
                for n in names {
                    out.push(format!("instrument {n}"));
                }
                inserted = true;
            }
        }
        if !inserted {
            for n in names {
                out.push(format!("instrument {n}"));
            }
        }
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    println!("set-instruments: {done} Bloecke gesetzt");
}

fn apply_instruments_url(path: &str) {
    let tsv = fs::read_to_string(path).expect("tsv");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let mut lines: Vec<String> = phi_text.lines().map(str::to_string).collect();
    let phi_blocks = blocks(&phi_text);
    let mut ins: Vec<(usize, String)> = Vec::new();
    let mut open = 0usize;
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(url), Some(inst)) = (it.next(), it.next()) else {
            continue;
        };
        let inst = inst.trim();
        if inst == "OFFEN" || inst == "-" || inst.is_empty() {
            open += 1;
            continue;
        }
        let Some((start, block)) = phi_blocks
            .iter()
            .find(|(_, b)| field(b, "url").as_deref() == Some(url.trim()))
            .or_else(|| {
                phi_blocks.iter().find(|(_, b)| {
                    let Some(u) = field(b, "url") else {
                        return false;
                    };
                    u.ends_with(&format!("/{}", url.trim().rsplit('/').next().unwrap_or(url.trim())))
                })
            })
            .or_else(|| {
                phi_blocks.iter().find(|(_, b)| {
                    let Some(origin) = field(b, "origin") else {
                        return false;
                    };
                    let Some(ourl) = origin.split_whitespace().next() else {
                        return false;
                    };
                    ourl == url.trim()
                        || ourl.starts_with(url.trim())
                        || url.trim().starts_with(ourl)
                })
            })
        else {
            continue;
        };
        let existing = block.iter().any(|l| l.starts_with("instrument "));
        let mut extra = 0usize;
        for (i, name) in inst.split(',').map(str::trim).filter(|s| !s.is_empty()).enumerate() {
            if existing {
                continue;
            }
            if let Some(pos) = block.iter().position(|l| l.starts_with("origin ")) {
                ins.push((start + pos + i, format!("instrument {name}")));
                extra += 1;
            }
        }
        let _ = extra;
    }
    for (at, line) in ins.iter().rev() {
        lines.insert(*at, line.clone());
    }
    write_out(PHI, lines.join("\n") + "\n", "phi");
    println!("instrument-url: {} gesetzt, {open} OFFEN uebersprungen", ins.len());
}

fn aussondern(class: &str, out_path: &str, derived_list: Option<&str>) {
    use std::collections::BTreeSet;
    let text = fs::read_to_string(PHI).expect("phi");
    let mut keep: Vec<String> = Vec::new();
    let mut moved: Vec<String> = Vec::new();
    let mut count = 0usize;
    let mut wanted: BTreeSet<usize> = BTreeSet::new();
    if class == "derived"
        && let Some(list) = derived_list
    {
        let list_text = match fs::read_to_string(list) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        for line in list_text.lines() {
            if line.contains("DERIVED")
                && let Some(n) = line.split('\t').next()
                && let Ok(n) = n.trim().parse::<usize>()
            {
                wanted.insert(n);
            }
        }
    }
    for (start, block) in blocks(&text) {
        let fmt = or_empty(field(&block, "format"));
        let is = match class {
            "reference" => fmt == "reference",
            "ephemeris" => {
                fmt.starts_with("ephemeris_")
                    || fmt == "spk"
                    || fmt == "orbit_bin"
                    || fmt == "kernel_text"
            }
            "derived" => wanted.contains(&start),
            _ => false,
        };
        if !is {
            keep.extend(block);
            keep.push(String::new());
            continue;
        }
        let mut stripped: Vec<String> = Vec::new();
        for l in block {
            let head = l.split_whitespace().next().unwrap_or("");
            if matches!(head, "field" | "quantity" | "last" | "instrument") {
                continue;
            }
            if head == "at" || head == "on" || head == "query" {
                continue;
            }
            stripped.push(l);
        }
        moved.extend(stripped);
        moved.push(String::new());
        count += 1;
    }
    write_out(PHI, keep.join("\n") + "\n", "phi");
    let mut out = match fs::read_to_string(out_path) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    if !out.is_empty() && !out.ends_with("\n\n") {
        out.push_str("\n\n");
    }
    out.push_str(&moved.join("\n"));
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    write_out(out_path, out, "out");
    println!("aussondern {class}: {count} Bloecke verschoben");
}

fn restore_missing(gitref: &str) {
    use std::collections::BTreeSet;
    let cur = fs::read_to_string(PHI).expect("phi");
    let mut have: BTreeSet<String> = BTreeSet::new();
    for (_, b) in blocks(&cur) {
        if let Some(u) = field(&b, "url") {
            have.insert(u);
        }
    }
    let out = Command::new("git")
        .args(["show", &format!("{gitref}:phi/sources.\u{3c6}")])
        .output()
        .expect("git show");
    let old = String::from_utf8_lossy(&out.stdout).to_string();
    let mut add: Vec<String> = Vec::new();
    let mut n = 0usize;
    for (_, b) in blocks(&old) {
        let Some(u) = field(&b, "url") else {
            continue;
        };
        if have.contains(&u) {
            continue;
        }
        add.extend(b);
        add.push(String::new());
        n += 1;
    }
    let mut all = cur;
    if !all.ends_with("\n\n") {
        all.push_str("\n\n");
    }
    all.push_str(&add.join("\n"));
    write_out(PHI, all, "phi");
    println!("restore: {n} fehlende Bloecke aus {gitref} zurueck");
}

fn verify_specs() {
    let text = fs::read_to_string(REG).expect("register");
    let mut rows = 0usize;
    let mut quote_missing = 0usize;
    let mut inst_not_in_quote = 0usize;
    let mut api_quote_missing = 0usize;
    let mut cite_missing = 0usize;
    let mut examples: Vec<String> = Vec::new();
    for (_, r) in blocks(&text) {
        let Some(inst) = field(&r, "instrument") else {
            continue;
        };
        rows += 1;
        let quote = or_empty(field(&r, "instrument_quote"));
        let api_quote = or_empty(field(&r, "api_quote"));
        let api_cite = or_empty(field(&r, "api_cite"));
        let inst_cite = or_empty(field(&r, "instrument_cite"));
        if quote.trim().is_empty() {
            quote_missing += 1;
        } else if !quote.to_lowercase().contains(&inst.to_lowercase()) {
            inst_not_in_quote += 1;
            if examples.len() < 20 {
                examples.push(format!("{inst}\t{}\t{quote}", or_empty(field(&r, "applies"))));
            }
        }
        if api_quote.trim().is_empty() {
            api_quote_missing += 1;
        }
        if api_cite.trim().is_empty() || inst_cite.trim().is_empty() {
            cite_missing += 1;
        }
    }
    println!("Zeilen: {rows}");
    println!("instrument_quote leer: {quote_missing}");
    println!("instrument nicht im instrument_quote: {inst_not_in_quote}");
    println!("api_quote leer: {api_quote_missing}");
    println!("api_cite oder instrument_cite leer: {cite_missing}");
    println!("\nBeispiele (instrument \\t applies \\t quote):");
    for e in examples {
        println!("{e}");
    }
}

fn sync_fields() {
    let reg_text = fs::read_to_string(REG).expect("register");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let phi_blocks = blocks(&phi_text);
    let mut rows = blocks(&reg_text);
    let mut synced = 0usize;
    for (_, row) in rows.iter_mut() {
        let Some(applies) = field(row, "applies") else {
            continue;
        };
        let origin = applies.split_whitespace().next().unwrap_or("");
        if origin.is_empty() {
            continue;
        }
        let Some((_, block)) = phi_blocks.iter().find(|(_, b)| {
            let Some(o) = field(b, "origin") else {
                return false;
            };
            let Some(u) = o.split_whitespace().next() else {
                return false;
            };
            u == origin || u.starts_with(origin) || origin.starts_with(u)
        }) else {
            continue;
        };
        let mut keys: Vec<String> = Vec::new();
        for line in block {
            let p: Vec<&str> = line.split_whitespace().collect();
            if matches!(p.first(), Some(&"field") | Some(&"quantity") | Some(&"last"))
                && let Some(k) = p.get(1)
            {
                let k = k.trim_matches('"').to_string();
                if !keys.iter().any(|x| x.eq_ignore_ascii_case(&k)) {
                    keys.push(k);
                }
            }
        }
        if keys.is_empty() {
            continue;
        }
        row.retain(|l| !l.starts_with("  fields "));
        let at = row
            .iter()
            .position(|l| l.starts_with("  applies "))
            .unwrap_or(row.len());
        row.insert(at, format!("  fields {}", keys.join(" ")));
        synced += 1;
    }
    let mut out = String::new();
    for (_, b) in &rows {
        out.push_str(&b.join("\n"));
        out.push_str("\n\n");
    }
    write_out(REG, out, "register");
    println!("sync-fields: {synced} Zeilen an Block-Keys gebunden");
}

fn set_instruments_by_asset(path: &str) {
    use std::collections::BTreeMap;
    let tsv = fs::read_to_string(path).expect("tsv");
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(asset), Some(inst)) = (it.next(), it.next()) else {
            continue;
        };
        let asset = asset.trim().to_string();
        let names: Vec<String> = if inst.trim() == "-" {
            Vec::new()
        } else {
            inst.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
        };
        if !asset.is_empty() {
            map.insert(asset, names);
        }
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let mut out: Vec<String> = Vec::new();
    let mut done = 0usize;
    for (_, block) in blocks(&text) {
        let url = match field(&block, "url") {
            Some(u) => u,
            None => String::new(),
        };
        let names = map.iter().find(|(a, _)| url.ends_with(&format!("/{a}"))).map(|(_, v)| v.clone());
        let Some(names) = names else {
            out.extend(block);
            out.push(String::new());
            continue;
        };
        done += 1;
        let mut inserted = false;
        for l in block {
            if l.starts_with("instrument ") {
                continue;
            }
            out.push(l.clone());
            if !inserted && l.starts_with("origin ") {
                for n in &names {
                    out.push(format!("instrument {n}"));
                }
                inserted = true;
            }
        }
        if !inserted {
            for n in &names {
                out.push(format!("instrument {n}"));
            }
        }
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    println!("set-instruments-by-asset: {done} Bloecke gesetzt");
}

fn strip_block_for_derived(block: &[String]) -> Vec<String> {
    let mut stripped: Vec<String> = Vec::new();
    for l in block {
        let head = l.split_whitespace().next().unwrap_or("");
        if matches!(head, "field" | "quantity" | "last" | "instrument") {
            continue;
        }
        if head == "at" || head == "on" || head == "query" {
            continue;
        }
        stripped.push(l.clone());
    }
    stripped
}

fn append_derived(out_path: &str, moved: &[String]) {
    let mut out = match fs::read_to_string(out_path) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    if !out.is_empty() && !out.ends_with("\n\n") {
        out.push_str("\n\n");
    }
    out.push_str(&moved.join("\n"));
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    write_out(out_path, out, "out");
}

fn aussondern_format(format_value: &str, out_path: &str) {
    let text = fs::read_to_string(PHI).expect("phi");
    let mut keep: Vec<String> = Vec::new();
    let mut moved: Vec<String> = Vec::new();
    let mut count = 0usize;
    for (_, block) in blocks(&text) {
        if field(&block, "format").as_deref() == Some(format_value) {
            moved.extend(strip_block_for_derived(&block));
            moved.push(String::new());
            count += 1;
            continue;
        }
        keep.extend(block);
        keep.push(String::new());
    }
    write_out(PHI, keep.join("\n") + "\n", "phi");
    append_derived(out_path, &moved);
    println!("aussondern-format {format_value}: {count} Bloecke verschoben");
}

fn url_matches(template: &str, url: &str) -> bool {
    if url == template || url.ends_with(&format!("/{template}")) {
        return true;
    }
    if let (Some(open), Some(close)) = (template.find('<'), template.find('>'))
        && open < close
    {
        let pre = &template[..open];
        let suf = &template[close + 1..];
        return url.starts_with(pre) && url.ends_with(suf);
    }
    false
}

fn aussondern_assets(list_path: &str, out_path: &str) {
    let mut wanted: BTreeMap<String, String> = BTreeMap::new();
    let list = match fs::read_to_string(list_path) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    for line in list.lines() {
        if line.contains("REFERENZ") {
            let cols: Vec<&str> = line.split('\t').collect();
            let asset = match cols.first() {
                Some(s) => s.trim().to_string(),
                None => String::new(),
            };
            if asset.is_empty() {
                continue;
            }
            let reason = if cols.len() > 2 {
                cols[2..].join(" ").trim().to_string()
            } else {
                String::new()
            };
            wanted.insert(asset, reason);
        }
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let all_blocks = blocks(&text);
    let url_owner: BTreeSet<String> = wanted
        .keys()
        .filter(|a| {
            all_blocks.iter().any(|(_, b)| {
                let u = or_empty(field(b, "url"));
                !u.is_empty() && url_matches(a, &u)
            })
        })
        .cloned()
        .collect();
    let mut keep: Vec<String> = Vec::new();
    let mut moved: Vec<String> = Vec::new();
    let mut count = 0usize;
    for (_, block) in blocks(&text) {
        let url = match field(&block, "url") {
            Some(u) => u,
            None => String::new(),
        };
        let origin = or_empty(field(&block, "origin"));
        let url_hit = wanted
            .iter()
            .find(|(a, _)| url_matches(a, &url))
            .map(|(a, _)| a.clone());
        let hit_key = match url_hit {
            Some(k) => Some(k),
            None => wanted
                .iter()
                .find(|(a, _)| {
                    !url_owner.contains(*a)
                        && !origin.is_empty()
                        && (url_matches(a, &origin)
                            || origin.split(';').any(|o| url_matches(a, o.trim())))
                })
                .map(|(a, _)| a.clone()),
        };
        let Some(key) = hit_key else {
            keep.extend(block);
            keep.push(String::new());
            continue;
        };
        let reason = or_empty(wanted.remove(&key));
        let mut stripped = strip_block_for_derived(&block);
        if !reason.is_empty() && !stripped.iter().any(|l| l.starts_with("keine_messung ")) {
            stripped.push(format!("keine_messung \"{reason}\""));
        }
        println!("  verschoben: {url}");
        moved.extend(stripped);
        moved.push(String::new());
        count += 1;
    }
    write_out(PHI, keep.join("\n") + "\n", "phi");
    append_derived(out_path, &moved);
    println!("aussondern-assets: {count} Bloecke verschoben");
    for key in wanted.keys() {
        println!("  nicht getroffen: {key}");
    }
}

fn replace_blocks(path: &str) {
    use std::collections::BTreeMap;
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let text = fs::read_to_string(PHI).expect("phi");
    let mut new_blocks: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (_, b) in blocks(&src) {
        if let Some(u) = field(&b, "url") {
            new_blocks.insert(u, b);
        }
    }
    let mut out: Vec<String> = Vec::new();
    let mut replaced = 0usize;
    for (_, b) in blocks(&text) {
        let url = field(&b, "url");
        if let Some(u) = &url
            && let Some(nb) = new_blocks.remove(u)
        {
            out.extend(nb);
            out.push(String::new());
            replaced += 1;
            continue;
        }
        out.extend(b);
        out.push(String::new());
    }
    let added = new_blocks.len();
    for (_, b) in new_blocks {
        out.extend(b);
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    println!("replace-blocks: {replaced} ersetzt, {added} neu angehaengt");
}

fn github_token() -> String {
    let candidates = [
        or_empty(std::env::var("OMEGAFLOW_SECRETS_FILE").ok()),
        format!(
            "{}/.secrets.local",
            or_empty(std::env::var("HOME").ok())
        ),
        ".secrets.local".to_string(),
    ];
    for path in candidates {
        if path.is_empty() {
            continue;
        }
        let text = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        for line in text.lines() {
            if let Some(v) = line.strip_prefix("GH_TOKEN=") {
                let t = v.trim().trim_matches('"').to_string();
                if !t.is_empty() {
                    return t;
                }
            }
        }
    }
    String::new()
}

fn derived_dedup(path: &str) {
    let text = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut by_url: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    let mut total = 0usize;
    for (_, b) in blocks(&text) {
        let url = or_empty(field(&b, "url"));
        total += 1;
        match by_url.get_mut(&url) {
            Some(prev) if prev.len() >= b.len() => {}
            Some(prev) => {
                *prev = b;
            }
            None => {
                order.push(url.clone());
                by_url.insert(url, b);
            }
        }
    }
    let mut out: Vec<String> = Vec::new();
    for url in &order {
        if let Some(b) = by_url.remove(url) {
            out.extend(b);
            out.push(String::new());
        }
    }
    write_out(path, out.join("\n") + "\n", "derived");
    println!(
        "derived-dedup: {total} gelesen, {} eindeutig, {} Dubletten entfernt",
        order.len(),
        total - order.len()
    );
}

fn fill_sha(report_path: Option<&str>, max_bytes: u64) {
    let token = github_token();
    if token.is_empty() {
        println!("fill-sha: GH_TOKEN fehlt");
        return;
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let mut out: Vec<String> = Vec::new();
    let mut report: Vec<String> = Vec::new();
    let mut cache: BTreeMap<String, String> = BTreeMap::new();
    let mut done = 0usize;
    let mut corrected = 0usize;
    let mut skipped = 0usize;
    let mut downloaded = 0usize;
    let mut api_calls = 0usize;
    for (_, block) in blocks(&text) {
        let existing = or_empty(field(&block, "sha256"));
        let url = match field(&block, "url") {
            Some(u) => u,
            None => String::new(),
        };
        let marker = "/releases/download/";
        if !url.contains(marker) {
            out.extend(block);
            out.push(String::new());
            continue;
        }
        let rest = url.split(marker).nth(1).unwrap_or("");
        let mut parts = rest.splitn(2, '/');
        let tag = parts.next().unwrap_or("");
        let asset = rest.rsplit('/').next().unwrap_or("");
        if tag.is_empty() || asset.is_empty() || asset == tag {
            skipped += 1;
            report.push(format!("{url}\t{tag}\t{asset}\tpfad-ohne-asset\t0"));
            out.extend(block);
            out.push(String::new());
            continue;
        }
        let body = match cache.get(tag) {
            Some(b) => b.clone(),
            None => {
                let api =
                    format!("https://api.github.com/repos/omegaflow/sources/releases/tags/{tag}");
                let rel = Command::new("curl")
                    .args(["-sS", "-H", &format!("Authorization: Bearer {token}"), &api])
                    .output();
                api_calls += 1;
                let b = match rel {
                    Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                    Err(_) => String::new(),
                };
                cache.insert(tag.to_string(), b.clone());
                b
            }
        };
        let needle = format!("\"name\": \"{asset}\"");
        let mut hash = String::new();
        let mut asset_id = String::new();
        let mut asset_size = 0u64;
        let mut found = false;
        if let Some(pos) = body.find(&needle) {
            found = true;
            let before_lo = pos.saturating_sub(400);
            let before = &body[before_lo..pos];
            if let Some(ip) = before.rfind("\"id\":") {
                asset_id = before[ip + 5..]
                    .trim_start()
                    .split(|c: char| !c.is_ascii_digit())
                    .next()
                    .unwrap_or("")
                    .to_string();
            }
            let after_start = pos + needle.len();
            let after_end = match body[after_start..].find("\"name\":") {
                Some(i) => after_start + i,
                None => (after_start + 4000).min(body.len()),
            };
            let after = &body[after_start..after_end];
            if let Some(dp) = after.find("\"digest\": \"sha256:") {
                hash = after[dp + 18..]
                    .split('"')
                    .next()
                    .unwrap_or("")
                    .to_string();
            }
            if let Some(sp) = after.find("\"size\":") {
                let digits = after[sp + 7..]
                    .trim_start()
                    .split(|c: char| !c.is_ascii_digit())
                    .next()
                    .unwrap_or("");
                if let Ok(sz) = digits.parse::<u64>() {
                    asset_size = sz;
                }
            }
        }
        if hash.len() != 64 && found && !asset_id.is_empty() && asset_size > 0 && asset_size <= max_bytes
        {
            let dl = format!("https://api.github.com/repos/omegaflow/sources/releases/assets/{asset_id}");
            let tmp = std::env::temp_dir().join(format!("omegaflow-sha-{asset_id}"));
            let got = Command::new("curl")
                .args([
                    "-sSL",
                    "-H",
                    &format!("Authorization: Bearer {token}"),
                    "-H",
                    "Accept: application/octet-stream",
                    "-o",
                    &tmp.to_string_lossy(),
                    &dl,
                ])
                .status();
            if got.map(|s| s.success()).unwrap_or(false) {
                if let Ok(o) = Command::new("sha256sum").arg(&tmp).output() {
                    let h = String::from_utf8_lossy(&o.stdout)
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string();
                    if h.len() == 64 {
                        hash = h;
                        downloaded += 1;
                    }
                }
            }
            let _ = fs::remove_file(&tmp);
        }
        if hash.len() != 64 {
            skipped += 1;
            let reason = if !found {
                if body.contains("\"message\": \"Not Found\"") {
                    "tag-404"
                } else {
                    "asset-nicht-im-release"
                }
            } else if asset_size > max_bytes {
                "kein-digest-zu-gross"
            } else {
                "kein-digest"
            };
            report.push(format!("{url}\t{tag}\t{asset}\t{reason}\t{asset_size}"));
            out.extend(block);
            out.push(String::new());
            continue;
        }
        let mut inserted = false;
        for l in block {
            if l.starts_with("sha256 ") {
                out.push(format!("sha256 {hash}"));
                inserted = true;
                continue;
            }
            out.push(l.clone());
            if !inserted && l.starts_with("compiler ") {
                out.push(format!("sha256 {hash}"));
                inserted = true;
            }
        }
        if !inserted {
            out.push(format!("sha256 {hash}"));
        }
        out.push(String::new());
        done += 1;
        if !existing.is_empty() && existing != hash {
            corrected += 1;
            report.push(format!("{url}\t{tag}\t{asset}\tkORRIGIERT\t{existing}->{hash}"));
        }
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    if let Some(p) = report_path {
        if let Some(parent) = std::path::Path::new(p).parent() {
            let _ = fs::create_dir_all(parent);
        }
        write_out(p, report.join("\n") + "\n", "report");
    }
    println!(
        "fill-sha: {done} Hashes geprueft/gesetzt ({downloaded} per Download, {corrected} korrigiert), {skipped} nicht aufloesbar, {api_calls} API-Aufrufe"
    );
}

fn has_measurement_line(block: &[String]) -> bool {
    block.iter().any(|l| {
        l.starts_with("field ") || l.starts_with("quantity ") || l.starts_with("last ")
    })
}

fn split_field(path: &str, with_path: &str, without_path: &str) {
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut with: Vec<String> = Vec::new();
    let mut without: Vec<String> = Vec::new();
    let (mut nw, mut no) = (0usize, 0usize);
    for (_, b) in blocks(&src) {
        if has_measurement_line(&b) {
            with.extend(b);
            with.push(String::new());
            nw += 1;
        } else {
            without.extend(b);
            without.push(String::new());
            no += 1;
        }
    }
    if let Some(p) = std::path::Path::new(with_path).parent() {
        let _ = fs::create_dir_all(p);
    }
    write_out(with_path, with.join("\n") + "\n", "with");
    write_out(without_path, without.join("\n") + "\n", "without");
    println!("split-field: {nw} mit Messzeile, {no} ohne Messzeile");
}

fn derived_clean(path: &str) {
    let text = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut keep: Vec<String> = Vec::new();
    let mut removed: Vec<String> = Vec::new();
    for (_, b) in blocks(&text) {
        if has_measurement_line(&b) {
            removed.push(or_empty(field(&b, "url")));
            continue;
        }
        keep.extend(b);
        keep.push(String::new());
    }
    write_out(path, keep.join("\n") + "\n", "derived");
    println!("derived-clean: {} Messbloecke entfernt", removed.len());
    for u in removed {
        println!("  raus: {u}");
    }
}

fn merge_rebuild(blocks_path: &str, derived_path: &str) {
    let src = match fs::read_to_string(blocks_path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut measurements: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut refs: Vec<(String, Vec<String>)> = Vec::new();
    for (_, b) in blocks(&src) {
        let url = match field(&b, "url") {
            Some(u) => u,
            None => continue,
        };
        if has_measurement_line(&b) {
            measurements.insert(url, b);
        } else {
            refs.push((url, b));
        }
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let mut out: Vec<String> = Vec::new();
    let (mut replaced, mut appended) = (0usize, 0usize);
    for (_, b) in blocks(&text) {
        let url = or_empty(field(&b, "url"));
        if let Some(nb) = measurements.remove(&url) {
            out.extend(nb);
            out.push(String::new());
            replaced += 1;
            continue;
        }
        if refs.iter().any(|(u, _)| *u == url) {
            continue;
        }
        out.extend(b);
        out.push(String::new());
    }
    let extra = measurements.len();
    for (_, b) in measurements {
        out.extend(b);
        out.push(String::new());
        appended += 1;
    }
    write_out(PHI, out.join("\n") + "\n", "phi");

    let mut derived: Vec<String> = Vec::new();
    let mut ref_count = 0usize;
    for (_, b) in refs {
        derived.extend(strip_block_for_derived(&b));
        derived.push(String::new());
        ref_count += 1;
    }
    if !derived.is_empty() {
        append_derived(derived_path, &derived);
    }
    println!(
        "merge-rebuild: {replaced} ersetzt, {extra} neu angehaengt, {ref_count} Referenzen ausgesondert"
    );
    let _ = appended;
}

fn filter_rows(rows_path: &str, blocks_path: &str, out_path: &str) {
    use std::collections::BTreeSet;
    let rows = match fs::read_to_string(rows_path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let block_src = match fs::read_to_string(blocks_path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut urls: BTreeSet<String> = BTreeSet::new();
    for (_, b) in blocks(&block_src) {
        if let Some(u) = field(&b, "url") {
            urls.insert(u);
        }
    }
    let mut out: Vec<String> = Vec::new();
    let (mut kept, mut dropped) = (0usize, 0usize);
    for (_, b) in blocks(&rows) {
        let applies = or_empty(field(&b, "applies"));
        let target = applies.split_whitespace().next().unwrap_or("");
        if urls.contains(target) {
            out.extend(b);
            out.push(String::new());
            kept += 1;
        } else {
            dropped += 1;
        }
    }
    if let Some(p) = std::path::Path::new(out_path).parent() {
        let _ = fs::create_dir_all(p);
    }
    write_out(out_path, out.join("\n") + "\n", "out");
    println!("filter-rows: {kept} behalten, {dropped} verworfen");
}

fn dups(mode: &str) {
    let text = fs::read_to_string(PHI).expect("phi");
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (_, b) in blocks(&text) {
        let url = or_empty(field(&b, "url"));
        let key = match mode {
            "sha256" => or_empty(field(&b, "sha256")),
            "origin" => or_empty(field(&b, "origin")),
            _ => {
                let origin = or_empty(field(&b, "origin"));
                let fk = match b.iter().find(|l| {
                    l.starts_with("field ") || l.starts_with("last ") || l.starts_with("quantity ")
                }) {
                    Some(l) => l.split_whitespace().nth(1).unwrap_or("").to_string(),
                    None => String::new(),
                };
                format!("{origin}\t{fk}")
            }
        };
        if key.is_empty() {
            continue;
        }
        groups.entry(key).or_default().push(url);
    }
    let mut n = 0usize;
    for (key, urls) in &groups {
        if urls.len() > 1 {
            n += 1;
            println!("{key}");
            for u in urls {
                println!("  {u}");
            }
        }
    }
    println!("dups {mode}: {n} Gruppen mit mehr als einem Block");
}

fn suggest_assets(path: &str, out_path: &str) {
    use std::collections::BTreeSet;
    let token = github_token();
    if token.is_empty() {
        println!("suggest-assets: GH_TOKEN fehlt");
        return;
    }
    let list = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut cache: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut out: Vec<String> = Vec::new();
    for line in list.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 3 {
            continue;
        }
        let tag = cols[1].trim();
        let asset = cols[2].trim();
        if tag.is_empty() || asset.is_empty() {
            continue;
        }
        let names = match cache.get(tag) {
            Some(v) => v.clone(),
            None => {
                let api =
                    format!("https://api.github.com/repos/omegaflow/sources/releases/tags/{tag}");
                let body = match Command::new("curl")
                    .args(["-sS", "-H", &format!("Authorization: Bearer {token}"), &api])
                    .output()
                {
                    Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                    Err(_) => String::new(),
                };
                let mut names: Vec<String> = Vec::new();
                if let Some(apos) = body.find("\"assets\":") {
                    let mut rest = &body[apos..];
                    while let Some(p) = rest.find("\"name\": \"") {
                        let tail = &rest[p + 9..];
                        let name = tail.split('"').next().unwrap_or("").to_string();
                        let nlen = name.len();
                        if !name.is_empty() {
                            names.push(name);
                        }
                        rest = &tail[nlen.min(tail.len())..];
                    }
                }
                cache.insert(tag.to_string(), names.clone());
                names
            }
        };
        let mut cands: BTreeSet<String> = BTreeSet::new();
        let stem = asset.split('.').next().unwrap_or(asset);
        for n in &names {
            let nstem = n.split('.').next().unwrap_or(n);
            if n == asset || nstem == stem || (stem.len() >= 6 && nstem.starts_with(stem)) {
                cands.insert(n.clone());
            }
        }
        let pick: Vec<String> = cands.into_iter().take(3).collect();
        out.push(format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            cols[0].trim(),
            tag,
            asset,
            cols.get(3).map(|s| s.trim()).unwrap_or(""),
            pick.len(),
            pick.join(",")
        ));
    }
    if let Some(p) = std::path::Path::new(out_path).parent() {
        let _ = fs::create_dir_all(p);
    }
    write_out(out_path, out.join("\n") + "\n", "suggest");
    let with = out.iter().filter(|l| !l.ends_with("\t0\t")).count();
    println!(
        "suggest-assets: {} Zeilen, {with} mit Vorschlag, {} Tags abgefragt",
        out.len(),
        cache.len()
    );
}

fn block_indent(block: &[String]) -> &'static str {
    match block.first() {
        Some(l) if l.starts_with("host ") => "  ",
        Some(l) if l.starts_with(' ') => "  ",
        _ => "  ",
    }
}

fn set_row_key(block: &mut Vec<String>, key: &str, value: &str) {
    let prefix = format!("{key} ");
    let indent = block_indent(block);
    if let Some(pos) = block
        .iter()
        .position(|l| l.trim_start().starts_with(&prefix))
    {
        block[pos] = format!("{indent}{key} {value}");
        return;
    }
    let pos = block
        .iter()
        .position(|l| l.trim_start().starts_with("applies "))
        .unwrap_or(block.len());
    block.insert(pos, format!("{indent}{key} {value}"));
}

fn set_instrument_lines(block: &mut Vec<String>, instruments: &[String]) {
    let indent = block_indent(block);
    block.retain(|l| !l.trim_start().starts_with("instrument "));
    let pos = block
        .iter()
        .position(|l| l.trim_start().starts_with("applies "))
        .unwrap_or(block.len());
    for (i, name) in instruments.iter().enumerate() {
        block.insert(pos + i, format!("{indent}instrument {name}"));
    }
}

fn spec_matches_block(spec_url: &str, url: &str, origin: &str, fkey: &str) -> bool {
    if (!url.is_empty() && url == spec_url)
        || (!origin.is_empty()
            && origin.split(';').any(|o| o.trim() == spec_url))
    {
        return true;
    }
    if origin.contains("<name>") {
        let base = spec_url.rsplit('/').next().unwrap_or("").to_lowercase();
        let stem: String = base
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect();
        if stem.len() >= 4 && !fkey.is_empty() && fkey.starts_with(&stem) {
            return true;
        }
    }
    false
}

fn apply_specs(list_path: &str, derived_path: &str) {
    let list = match fs::read_to_string(list_path) {
        Ok(s) => s,
        Err(_) => return,
    };
    struct Spec {
        url: String,
        instruments: Vec<String>,
        inst_cite: String,
        inst_quote: String,
        data_cite: String,
        data_quote: String,
        status: String,
    }
    let mut specs: Vec<Spec> = Vec::new();
    for line in list.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 7 {
            continue;
        }
        specs.push(Spec {
            url: cols[0].trim().to_string(),
            instruments: cols[1]
                .split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty() && s != "-")
                .collect(),
            inst_cite: cols[2].trim().to_string(),
            inst_quote: cols[3].trim().to_string(),
            data_cite: cols[4].trim().to_string(),
            data_quote: cols[5].trim().to_string(),
            status: cols[6].trim().to_string(),
        });
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let mut block_url_by_spec: BTreeMap<String, String> = BTreeMap::new();
    for (_, b) in blocks(&text) {
        let url = or_empty(field(&b, "url"));
        let origin = or_empty(field(&b, "origin"));
        let fkey = b
            .iter()
            .find(|l| l.starts_with("field ") || l.starts_with("last ") || l.starts_with("quantity "))
            .and_then(|l| l.split_whitespace().nth(2).or_else(|| l.split_whitespace().nth(1)))
            .unwrap_or("")
            .to_lowercase();
        if let Some(spec) = specs
            .iter()
            .find(|s| spec_matches_block(&s.url, &url, &origin, &fkey))
        {
            block_url_by_spec.insert(spec.url.clone(), url.clone());
        }
    }

    let reg = fs::read_to_string(REG).expect("register");
    let mut reg_out: Vec<String> = Vec::new();
    let mut rows_touched = 0usize;
    for (_, mut b) in blocks(&reg) {
        let applies = or_empty(field(&b, "applies"));
        let target = applies.split_whitespace().next().unwrap_or("").to_string();
        let hit = specs.iter().find(|s| {
            !target.is_empty()
                && (target == s.url
                    || block_url_by_spec
                        .get(&s.url)
                        .map(|u| *u == target)
                        .unwrap_or(false))
        });
        if let Some(spec) = hit {
            if !spec.instruments.is_empty() {
                set_instrument_lines(&mut b, &spec.instruments);
            }
            if !spec.data_cite.is_empty() {
                set_row_key(&mut b, "api_cite", &spec.data_cite);
            }
            if !spec.data_quote.is_empty() {
                set_row_key(&mut b, "api_quote", &spec.data_quote);
            }
            if !spec.inst_cite.is_empty() {
                set_row_key(&mut b, "instrument_cite", &spec.inst_cite);
            }
            if !spec.inst_quote.is_empty() {
                set_row_key(&mut b, "instrument_quote", &spec.inst_quote);
            }
            rows_touched += 1;
        }
        reg_out.extend(b);
        reg_out.push(String::new());
    }
    write_out(REG, reg_out.join("\n") + "\n", "register");

    let mut keep: Vec<String> = Vec::new();
    let mut moved: Vec<String> = Vec::new();
    let mut blocks_touched = 0usize;
    let mut refs = 0usize;
    for (_, mut b) in blocks(&text) {
        let url = or_empty(field(&b, "url"));
        let origin = or_empty(field(&b, "origin"));
        let fkey = b
            .iter()
            .find(|l| l.starts_with("field ") || l.starts_with("last ") || l.starts_with("quantity "))
            .and_then(|l| l.split_whitespace().nth(2).or_else(|| l.split_whitespace().nth(1)))
            .unwrap_or("")
            .to_lowercase();
        let hit = specs
            .iter()
            .find(|s| spec_matches_block(&s.url, &url, &origin, &fkey));
        let Some(spec) = hit else {
            keep.extend(b);
            keep.push(String::new());
            continue;
        };
        if spec.status == "keine_messung" {
            let mut stripped = strip_block_for_derived(&b);
            if !spec.data_quote.is_empty() {
                stripped.push(format!("keine_messung \"{}\"", spec.data_quote));
            }
            moved.extend(stripped);
            moved.push(String::new());
            refs += 1;
            continue;
        }
        if !spec.instruments.is_empty() {
            set_instrument_lines(&mut b, &spec.instruments);
        }
        keep.extend(b);
        keep.push(String::new());
        blocks_touched += 1;
    }
    write_out(PHI, keep.join("\n") + "\n", "phi");
    if !moved.is_empty() {
        append_derived(derived_path, &moved);
    }
    println!(
        "apply-specs: {rows_touched} Registerzeilen, {blocks_touched} Bloecke, {refs} Referenzen ausgesondert, {} Spec-Zeilen gelesen",
        specs.len()
    );
}

fn url_cut(path: &str, needle: &str) {
    let text = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut keep: Vec<String> = Vec::new();
    let mut n = 0usize;
    for (_, b) in blocks(&text) {
        let u = or_empty(field(&b, "url"));
        if !u.is_empty() && (u == needle || u.ends_with(&format!("/{needle}"))) {
            n += 1;
            continue;
        }
        keep.extend(b);
        keep.push(String::new());
    }
    write_out(path, keep.join("\n") + "\n", "url-cut");
    println!("url-cut: {n} Block/Bloecke entfernt aus {path}");
}

fn align_many(path: &str) {
    let list = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut wanted: Vec<(String, Vec<String>, Option<String>)> = Vec::new();
    for line in list.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 2 {
            continue;
        }
        let url = cols[0].trim().to_string();
        let instruments: Vec<String> = cols[1]
            .split('|')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let host = cols.get(2).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        if !url.is_empty() && !instruments.is_empty() {
            wanted.push((url, instruments, host));
        }
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let mut out: Vec<String> = Vec::new();
    let mut targets: Vec<(String, String, Vec<String>, Option<String>)> = Vec::new();
    let mut hit = 0usize;
    for (_, mut b) in blocks(&text) {
        let url = or_empty(field(&b, "url"));
        let origin = or_empty(field(&b, "origin"));
        let found = wanted.iter().find(|(spec, _, _)| {
            (!url.is_empty() && url == *spec)
                || (!origin.is_empty() && origin.split(';').any(|o| o.trim() == spec))
        });
        if let Some((_, instruments, row_host)) = found {
            set_instrument_lines(&mut b, instruments);
            targets.push((url, origin, instruments.clone(), row_host.clone()));
            hit += 1;
        }
        out.extend(b);
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");

    let reg = fs::read_to_string(REG).expect("register");
    let mut reg_out: Vec<String> = Vec::new();
    let mut rows = 0usize;
    for (_, mut b) in blocks(&reg) {
        let applies = or_empty(field(&b, "applies"));
        let target = applies.split_whitespace().next().unwrap_or("").to_string();
        let found = targets.iter().find(|(url, origin, _, _)| {
            (!url.is_empty() && *url == target)
                || (!origin.is_empty()
                    && origin.split(';').any(|o| o.trim() == target))
        });
        if let Some((_, _, instruments, row_host)) = found {
            set_instrument_lines(&mut b, instruments);
            if let Some(h) = row_host {
                set_row_key(&mut b, "host", h);
            }
            rows += 1;
        }
        reg_out.extend(b);
        reg_out.push(String::new());
    }
    write_out(REG, reg_out.join("\n") + "\n", "register");
    println!("align-many: {hit} Bloecke, {rows} Registerzeilen gesetzt");
}

fn stitch_orphans() {
    let text = fs::read_to_string(PHI).expect("phi");
    let mut groups: Vec<Vec<String>> = Vec::new();
    for (_, b) in blocks(&text) {
        groups.push(b);
    }
    let mut stitched = 0usize;
    for i in 1..groups.len() {
        let orphan = groups[i].clone();
        let has_url = orphan.iter().any(|l| l.starts_with("url "));
        let has_measure = orphan.iter().any(|l| {
            l.starts_with("field ") || l.starts_with("quantity ") || l.starts_with("last ")
        });
        if !has_url && has_measure && groups[i - 1].iter().any(|l| l.starts_with("url ")) {
            groups[i - 1].extend(orphan);
            groups[i].clear();
            stitched += 1;
        }
    }
    let mut out: Vec<String> = Vec::new();
    for b in groups {
        if b.is_empty() {
            continue;
        }
        out.extend(b);
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    println!("stitch-orphans: {stitched} Waisenzeilen an den vorigen Block genaeht");
}

fn set_fields(path: &str) {
    let tsv = fs::read_to_string(path).expect("tsv");
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let mut groups: Vec<Vec<String>> = Vec::new();
    for (_, b) in blocks(&phi_text) {
        groups.push(b);
    }
    let mut changed = 0usize;
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(url), Some(fline)) = (it.next(), it.next()) else {
            continue;
        };
        let fline = fline.trim();
        if !(fline.starts_with("field ") || fline.starts_with("quantity ") || fline.starts_with("last ")) {
            continue;
        }
        let head: Vec<&str> = fline.split_whitespace().take(5).collect();
        if head.len() < 5 {
            continue;
        }
        let fline = head.join(" ");
        let mut idx: Option<usize> = None;
        for (i, b) in groups.iter().enumerate() {
            if field(b, "url").as_deref() == Some(url.trim()) {
                idx = Some(i);
                break;
            }
        }
        if idx.is_none() {
            for (i, b) in groups.iter().enumerate() {
                if field(b, "url")
                    .map(|u| u.ends_with(&format!("/{}", url.trim().rsplit('/').next().unwrap_or(url.trim()))))
                    .unwrap_or(false)
                {
                    idx = Some(i);
                    break;
                }
            }
        }
        let Some(i) = idx else {
            continue;
        };
        let b = &mut groups[i];
        b.retain(|l| {
            !(l.starts_with("field ") || l.starts_with("quantity ") || l.starts_with("last "))
        });
        let pos = b
            .iter()
            .position(|l| l.starts_with("at "))
            .unwrap_or(b.len());
        b.insert(pos, fline.to_string());
        changed += 1;
    }
    let mut out: Vec<String> = Vec::new();
    for b in groups {
        out.extend(b);
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    println!("set-fields: {changed} Messzeilen ersetzt");
}

fn audit_blocks() {
    let text = fs::read_to_string(PHI).expect("phi");
    let mut foreign_fields = 0usize;
    let mut dup_sha = 0usize;
    let mut mixed = 0usize;
    let mut bad_format = 0usize;
    let mut host_mismatch = 0usize;
    let mut samples: Vec<String> = Vec::new();
    for (start, b) in blocks(&text) {
        let url = or_empty(field(&b, "url"));
        let compiler = or_empty(field(&b, "compiler"));
        let src = if compiler.is_empty() || compiler == "none" {
            String::new()
        } else {
            match fs::read_to_string(&compiler) {
                Ok(s) => s,
                Err(_) => String::new(),
            }
        };
        let sha_n = b.iter().filter(|l| l.starts_with("sha256 ")).count();
        if sha_n > 1 {
            dup_sha += 1;
            if samples.len() < 20 {
                samples.push(format!("dup_sha\t{start}\t{url}"));
            }
        }
        let has_field = b.iter().any(|l| l.starts_with("field "));
        let has_quantity = b.iter().any(|l| l.starts_with("quantity "));
        let has_last = b.iter().any(|l| l.starts_with("last "));
        if [has_field, has_quantity, has_last].iter().filter(|x| **x).count() > 1 {
            mixed += 1;
            if samples.len() < 20 {
                samples.push(format!("mixed\t{start}\t{url}"));
            }
        }
        if !src.is_empty() {
            for l in &b {
                let p: Vec<&str> = l.split_whitespace().collect();
                if !matches!(p.first(), Some(&"field") | Some(&"quantity") | Some(&"last")) {
                    continue;
                }
                let name = p.get(2).copied().unwrap_or("");
                if name.len() < 4 {
                    continue;
                }
                if !src.contains(name) {
                    foreign_fields += 1;
                    if samples.len() < 40 {
                        samples.push(format!("foreign_field\t{start}\t{url}\t{name}\t{compiler}"));
                    }
                }
            }
        }
        let fmt = or_empty(field(&b, "format"));
        if !fmt.is_empty() && !compiler.is_empty() && compiler != "none" {
            let stem = compiler
                .rsplit('/')
                .next()
                .unwrap_or("")
                .trim_end_matches("_compiler.rs");
            let ok = fmt.contains(stem) || stem.contains(&fmt) || fmt == "ephemeris_binary";
            if !ok {
                bad_format += 1;
                if samples.len() < 60 {
                    samples.push(format!("format_compiler\t{start}\t{url}\t{fmt}\t{stem}"));
                }
            }
        }
        let origin = or_empty(field(&b, "origin"));
        if let Some(tag) = url
            .split("/releases/download/")
            .nth(1)
            .and_then(|r| r.split('/').next())
            && let Some(oh) = origin.split("://").nth(1).and_then(|r| r.split(['/', ' ', '?']).next())
            && !tag.is_empty()
            && !oh.is_empty()
            && !tag.starts_with(oh)
            && !oh.starts_with(&tag.replace('-', "."))
        {
            host_mismatch += 1;
            if samples.len() < 80 {
                samples.push(format!("host\t{start}\t{url}\t{tag}\t{oh}"));
            }
        }
    }
    println!("audit: foreign_field={foreign_fields} dup_sha={dup_sha} mixed_lines={mixed} format_compiler={bad_format} host_mismatch={host_mismatch}");
    for s in samples {
        println!("{s}");
    }
}

fn secrets() -> BTreeMap<String, String> {
    omegaflow::archivar::fetch::load_env().into_iter().collect()
}

#[allow(dead_code)]
fn auth_candidates(host: &str, env: &BTreeMap<String, String>) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = vec![Vec::new()];
    if host.contains("frost") {
        if let (Some(id), Some(secret)) = (env.get("FROST_CLIENT_ID"), env.get("FROST_CLIENT_SECRET")) {
            if !id.is_empty() && !secret.is_empty() {
                out.push(vec!["-u".to_string(), format!("{id}:{secret}")]);
                return out;
            }
        }
    }
    let key = if host.contains("lasair") {
        "LASAIR_TOKEN"
    } else if host.contains("openaq") {
        "OPENAQ_API_KEY"
    } else if host.contains("entsoe") {
        "ENTSOE_API_TOKEN"
    } else if host.contains("earthdata") || host.contains("gesdisc") || host.contains("podaac") {
        "EARTHDATA_EDL_TOKEN"
    } else if host.contains("frost") {
        "FROST_BASIC_AUTH"
    } else if host.contains("tns") {
        "TNS_API_KEY"
    } else if host.contains("ncdc") || host.contains("noaa") {
        "NOAA_CDO_TOKEN"
    } else if host.contains("oceannetworks") {
        "OCEANNETWORKS_TOKEN"
    } else if host.contains("zenodo") {
        "ZENODO_TOKEN"
    } else if host.contains("copernicus") || host.contains("cmems") {
        "CMEMS_PASS"
    } else {
        ""
    };
    if key.is_empty() {
        return out;
    }
    if let Some(v) = env.get(key) {
        if v.is_empty() {
            return out;
        }
        let header = if host.contains("openaq") {
            format!("X-API-Key: {v}")
        } else if host.contains("ncdc") || host.contains("noaa") {
            format!("token: {v}")
        } else if host.contains("tns") {
            format!("api_key: {v}")
        } else if host.contains("earthdata") || host.contains("gesdisc") || host.contains("podaac") {
            format!("Authorization: Bearer {v}")
        } else {
            String::new()
        };
        if !header.is_empty() {
            out.push(vec!["-H".to_string(), header]);
        }
    }
    out
}

fn render_url(url: &str, anchor: &str, env: &BTreeMap<String, String>, today: &str) -> String {
    let mut s = url.to_string();
    for (k, v) in env {
        s = s.replace(&format!("{{{k}}}"), v);
    }
    s = s.replace("{today}", today);
    if let Some(y) = Command::new("date")
        .args(["-u", "-d", "yesterday", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
    {
        s = s.replace("{yesterday}", y.trim());
    }
    if let Some(t) = Command::new("date")
        .args(["-u", "-d", "tomorrow", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
    {
        s = s.replace("{tomorrow}", t.trim());
    }
    if let Some(w) = Command::new("date")
        .args(["-u", "-d", "7 days ago", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
    {
        s = s.replace("{week_ago}", w.trim());
    }
    if let Some(d) = Command::new("date")
        .args(["-u", "-d", "3 days ago", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
    {
        s = s.replace("{jd_start}", d.trim());
    }
    s = s.replace("{jd_end}", today);
    if let Ok(st) = std::env::var("OMEGAFLOW_STATION") {
        if !st.is_empty() {
            s = s.replace("{station}", &st);
        }
    }
    for part in anchor.split_whitespace() {
        if let Some((k, v)) = part.split_once('=')
            && !k.is_empty()
            && !v.is_empty()
        {
            s = s.replace(&format!("{{{k}}}"), v);
        }
    }
    s
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn probe_age_days(date: &str, today: &str) -> Option<i64> {
    let p = |s: &str| -> Option<(i64, i64, i64)> {
        let y = s.get(0..4)?.parse::<i64>().ok()?;
        let m = s.get(5..7)?.parse::<i64>().ok()?;
        let d = s.get(8..10)?.parse::<i64>().ok()?;
        Some((y, m, d))
    };
    let (ay, am, ad) = p(date)?;
    let (by, bm, bd) = p(today)?;
    Some(days_from_civil(ay, am, ad) - days_from_civil(by, bm, bd))
}

fn reprobe(report: &str) {
    let env = secrets();
    let today = match Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
    {
        Some(v) => v,
        None => String::new(),
    };
    let text = fs::read_to_string(REG).expect("register");
    let lines: Vec<String> = text.lines().map(str::to_string).collect();
    let mut repl: BTreeMap<usize, String> = BTreeMap::new();
    let mut report_lines: Vec<String> = Vec::new();
    let (mut fixed, mut failed, mut skipped) = (0usize, 0usize, 0usize);
    let phi_text = fs::read_to_string(PHI).expect("phi");
    let mut headers_by_block: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (_, b) in blocks(&phi_text) {
        let u = or_empty(field(&b, "url"));
        if u.is_empty() { continue; }
        let hs: Vec<String> = b.iter().filter(|l| l.trim_start().starts_with("header ")).map(|l| l.trim_start().to_string()).collect();
        if !hs.is_empty() { headers_by_block.insert(u, hs); }
    }
    for (start, block) in blocks(&text) {
        let Some(pos) = block.iter().position(|l| l.trim_start().starts_with("probe ")) else {
            continue;
        };
        let p: Vec<&str> = block[pos].split_whitespace().collect();
        if p.len() < 4 {
            continue;
        }
        let status = p[2];
        if matches!(status, "200" | "206" | "204" | "202" | "201") {
            continue;
        }
        let force = std::env::var("OMEGAFLOW_PROBE_FORCE").ok().as_deref() == Some("1");
        let ttl = match or_empty(field(&block, "ttl")).parse::<u64>() {
            Ok(v) => v,
            Err(_) => 0,
        };
        if ttl > 0 && !force {
            if let Some(age) = probe_age_days(p[1], &today) {
                if age >= 0 && (age as u64) * 86400 < ttl {
                    skipped += 1;
                    continue;
                }
            }
        }
        let raw_url = p[3..].join(" ");
        let anchor = or_empty(field(&block, "at"));
        let url = render_url(&raw_url, &anchor, &env, &today);
        if url.contains('{') {
            report_lines.push(format!("{status}\t{raw_url}\tUNGERENDERT"));
            failed += 1;
            continue;
        }
        let host = url
            .split_once("://")
            .map(|(_, r)| r)
            .unwrap_or("")
            .split(['/', '?'])
            .next()
            .unwrap_or("")
            .to_string();
        let skip = match std::env::var("OMEGAFLOW_PROBE_SKIP") {
            Ok(v) => v,
            Err(_) => String::new(),
        };
        if !skip.is_empty() && skip.split(',').any(|h| !h.trim().is_empty() && host.contains(h.trim())) {
            skipped += 1;
            continue;
        }
        let mut final_status = String::new();
        if host.contains("tns") {
            if let (Some(ua), Some(k)) = (env.get("TNS_UA"), env.get("TNS_API_KEY")) {
                if !ua.is_empty() && !k.is_empty() {
                    let out = Command::new("curl")
                        .args([
                            "-sSL",
                            "-o",
                            "/dev/null",
                            "-w",
                            "%{http_code}",
                            "--max-time",
                            "30",
                            "-A",
                            ua,
                            "-X",
                            "POST",
                            "--data-urlencode",
                            &format!("api_key={k}"),
                            "--data-urlencode",
                            "data={\"name\":\"SN 2024a\"}",
                            &url,
                        ])
                        .output();
                    final_status = match out
                        .ok()
                        .and_then(|o| String::from_utf8(o.stdout).ok())
                        .map(|s| s.trim().to_string())
                    {
                        Some(v) => v,
                        None => String::new(),
                    };
                }
            }
        }
        for attempt in 0..1 {
            if !final_status.is_empty() {
                break;
            }
            let mut args: Vec<String> = vec![
                "-sSL".to_string(),
                "-o".to_string(),
                "/dev/null".to_string(),
                "-w".to_string(),
                "%{http_code}".to_string(),
                "--max-time".to_string(),
                "15".to_string(),
            ];
            let applies = or_empty(field(&block, "applies"));
            let first = applies.split_whitespace().next().unwrap_or("");
            if let Some(headers) = headers_by_block.get(first) {
                for line in headers {
                    let rest = line.trim_start_matches("header ");
                    if let Some((name, val)) = rest.split_once(' ') {
                        let mut value = val.trim().to_string();
                        for (k, v) in &env {
                            value = value.replace(&format!("{{{k}}}"), v);
                        }
                        args.push("-H".to_string());
                        args.push(format!("{name}: {value}"));
                    }
                }
            }
            if let Ok(ca) = std::env::var("OMEGAFLOW_CA_FILE") {
                if std::path::Path::new(&ca).exists() {
                    args.push("--cacert".to_string());
                    args.push(ca);
                }
            }
            args.push(url.clone());
            let out = Command::new("curl").args(&args).output();
            final_status = match out
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
            {
                Some(v) => v,
                None => String::new(),
            };
            if matches!(final_status.as_str(), "200" | "206" | "204" | "202" | "201") {
                break;
            }
            if matches!(final_status.as_str(), "429" | "500" | "502" | "503" | "504" | "000") {
                std::thread::sleep(std::time::Duration::from_secs(1u64 << attempt));
                continue;
            }
            break;
        }
        let abs = start - 1 + pos;
        if matches!(final_status.as_str(), "200" | "206" | "204" | "202" | "201") {
            repl.insert(abs, format!("  probe {today}T00:00:00Z {final_status} {raw_url}"));
            fixed += 1;
        } else {
            report_lines.push(format!("{status}->{final_status}\t{url}\tOFFEN"));
            failed += 1;
        }
    }
    let mut out: Vec<String> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        out.push(match repl.get(&i) {
            Some(n) => n.clone(),
            None => l.clone(),
        });
    }
    write_out(REG, out.join("\n") + "\n", "register");
    write_out(report, report_lines.join("\n") + "\n", "report");
    println!("reprobe: {fixed} auf 2xx gebracht, {failed} offen, {skipped} nach ttl oder Sperrliste ausgelassen (siehe {report})");
}

fn normalize_blocks() {
    let text = fs::read_to_string(PHI).expect("phi");
    let mut out: Vec<String> = Vec::new();
    let mut removed = 0usize;
    let mut fixed = 0usize;
    for (_, b) in blocks(&text) {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut nb: Vec<String> = Vec::new();
        let mut changed = false;
        for l in b {
            let key = l.split_whitespace().next().unwrap_or("").to_string();
            let singleton = matches!(
                key.as_str(),
                "url" | "terms" | "format" | "origin" | "compiler" | "sha256" | "ttl" | "witness"
                    | "record" | "zustand"
            );
            if singleton && !seen.insert(key) {
                removed += 1;
                changed = true;
                continue;
            }
            nb.push(l);
        }
        if changed {
            fixed += 1;
        }
        out.extend(nb);
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    println!("normalize: {fixed} Bloecke, {removed} doppelte Zeilen entfernt");
}

fn write_out(path: &str, content: String, what: &str) {
    if std::env::var("OMEGAFLOW_DRY").ok().as_deref() == Some("1") {
        println!(
            "dry-run: {what} -> {path}: {} Zeilen, nicht geschrieben",
            content.lines().count()
        );
        return;
    }
    fs::write(path, content).expect("write");
}

fn add_line(path: &str) {
    let tsv = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let text = fs::read_to_string(PHI).expect("phi");
    let mut groups: Vec<Vec<String>> = Vec::new();
    for (_, b) in blocks(&text) {
        groups.push(b);
    }
    let mut added = 0usize;
    let mut skipped = 0usize;
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(url), Some(newline)) = (it.next(), it.next()) else {
            continue;
        };
        let newline = newline.trim();
        if newline.is_empty() {
            continue;
        }
        let Some(b) = groups.iter_mut().find(|b| {
            field(b, "url")
                .map(|u| u == url.trim() || u.ends_with(&format!("/{}", url.trim().rsplit('/').next().unwrap_or(url.trim()))))
                .unwrap_or(false)
        }) else {
            skipped += 1;
            continue;
        };
        if b.iter().any(|l| l.trim() == newline) {
            skipped += 1;
            continue;
        }
        let pos = b
            .iter()
            .position(|l| l.starts_with("format "))
            .map(|i| i + 1)
            .unwrap_or(b.len());
        b.insert(pos, newline.to_string());
        added += 1;
    }
    let mut out: Vec<String> = Vec::new();
    for b in groups {
        out.extend(b);
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    println!("add-line: {added} Zeilen eingefuegt, {skipped} uebersprungen");
}

fn refs_audit(path: &str) {
    let list = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let text = fs::read_to_string(PHI).expect("phi");
    let mut checked = 0usize;
    for line in list.lines() {
        if !line.contains("REFERENZ") {
            continue;
        }
        let url = line.split('\t').next().unwrap_or("").trim();
        if url.is_empty() {
            continue;
        }
        let Some(b) = blocks(&text).into_iter().map(|(_, b)| b).find(|b| {
            let u = or_empty(field(b, "url"));
            let o = or_empty(field(b, "origin"));
            u == url
                || u.ends_with(&format!("/{url}"))
                || o.split(';').any(|s| s.trim() == url)
        }) else {
            println!("FEHLT\t{url}");
            continue;
        };
        checked += 1;
        let measure = b.iter().any(|l| {
            l.starts_with("field ") || l.starts_with("quantity ") || l.starts_with("last ")
        });
        let instr = or_empty(field(&b, "instrument"));
        println!(
            "{}\t{}\t{}\t{url}",
            if measure { "MESSZEILE" } else { "ohne" },
            if instr.is_empty() { "-" } else { instr.as_str() },
            or_empty(field(&b, "format"))
        );
    }
    println!("refs-audit: {checked} Bloecke gefunden");
}


fn probe_terms(host_filter: &str) {
    let env = secrets();
    let today = match Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
    {
        Some(v) => v,
        None => String::new(),
    };
    let phi = fs::read_to_string(PHI).expect("phi");
    let mut terms_by_block: BTreeMap<String, String> = BTreeMap::new();
    for (_, b) in blocks(&phi) {
        let origin = or_empty(field(&b, "origin"));
        if !origin.contains(host_filter) {
            continue;
        }
        let url = or_empty(field(&b, "url"));
        let terms = or_empty(field(&b, "terms"));
        if let Some(t) = terms.split_whitespace().find(|t| t.starts_with("http")) {
            if !url.is_empty() {
                terms_by_block.insert(url, t.to_string());
            }
        }
    }
    let reg = fs::read_to_string(REG).expect("register");
    let lines: Vec<String> = reg.lines().map(str::to_string).collect();
    let mut repl: BTreeMap<usize, String> = BTreeMap::new();
    let (mut fixed, mut failed, mut skipped) = (0usize, 0usize, 0usize);
    for (start, block) in blocks(&reg) {
        let applies = or_empty(field(&block, "applies"));
        let first = applies.split_whitespace().next().unwrap_or("");
        let Some(term_url) = terms_by_block.get(first) else {
            skipped += 1;
            continue;
        };
        let Some(pos) = block
            .iter()
            .position(|l| l.trim_start().starts_with("probe "))
        else {
            skipped += 1;
            continue;
        };
        let url = render_url(term_url, "", &env, &today);
        if url.contains('{') {
            skipped += 1;
            continue;
        }
        let mut status = String::new();
        for attempt in 0..3 {
            let out = Command::new("curl")
                .args(["-sSL", "-o", "/dev/null", "-w", "%{http_code}", "--max-time", "30", &url])
                .output();
            status = match out
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
            {
                Some(v) => v,
                None => String::new(),
            };
            if matches!(status.as_str(), "200" | "206" | "204" | "202") {
                break;
            }
            if matches!(status.as_str(), "429" | "500" | "502" | "503" | "504" | "000") {
                std::thread::sleep(std::time::Duration::from_secs(1u64 << attempt));
                continue;
            }
            break;
        }
        let abs = start - 1 + pos;
        if matches!(status.as_str(), "200" | "206" | "204" | "202") {
            repl.insert(abs, format!("  probe {today}T00:00:00Z {status} {term_url}"));
            fixed += 1;
        } else {
            failed += 1;
        }
    }
    let mut out: Vec<String> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        out.push(match repl.get(&i) {
            Some(n) => n.clone(),
            None => l.clone(),
        });
    }
    write_out(REG, out.join("\n") + "\n", "register");
    println!("probe-terms: {fixed} gesetzt, {failed} ohne 2xx, {skipped} ohne Zuordnung");
}

fn probe_map(path: &str) {
    let tsv = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let env = secrets();
    let today = match Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
    {
        Some(v) => v,
        None => String::new(),
    };
    let mut pairs: Vec<(String, String)> = Vec::new();
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(key), Some(target)) = (it.next(), it.next()) else {
            continue;
        };
        if key.trim().is_empty() || target.trim().is_empty() {
            continue;
        }
        pairs.push((key.trim().to_string(), target.trim().to_string()));
    }
    let reg = fs::read_to_string(REG).expect("register");
    let lines: Vec<String> = reg.lines().map(str::to_string).collect();
    let mut repl: BTreeMap<usize, String> = BTreeMap::new();
    let (mut fixed, mut failed) = (0usize, 0usize);
    for (start, block) in blocks(&reg) {
        let Some(pos) = block
            .iter()
            .position(|l| l.trim_start().starts_with("probe "))
        else {
            continue;
        };
        let p: Vec<&str> = block[pos].split_whitespace().collect();
        if p.len() < 4 {
            continue;
        }
        let cur = p[3..].join(" ");
        let Some((_, target)) = pairs
            .iter()
            .find(|(key, _)| !key.is_empty() && cur.contains(key.as_str()))
        else {
            continue;
        };
        let mut url = render_url(target, "", &env, &today);
        if url.contains('{') {
            failed += 1;
            continue;
        }
        let mut status = String::new();
        for attempt in 0..3 {
            let out = Command::new("curl")
                .args(["-sSL", "-o", "/dev/null", "-w", "%{http_code}", "--max-time", "30", &url])
                .output();
            status = match out
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
            {
                Some(v) => v,
                None => String::new(),
            };
            if matches!(status.as_str(), "200" | "206" | "204" | "202") {
                break;
            }
            if matches!(status.as_str(), "429" | "500" | "502" | "503" | "504" | "000") {
                std::thread::sleep(std::time::Duration::from_secs(1u64 << attempt));
                continue;
            }
            break;
        }
        let abs = start - 1 + pos;
        if matches!(status.as_str(), "200" | "206" | "204" | "202") {
            repl.insert(abs, format!("  probe {today}T00:00:00Z {status} {target}"));
            fixed += 1;
        } else {
            failed += 1;
        }
        let _ = &mut url;
    }
    let mut out: Vec<String> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        out.push(match repl.get(&i) {
            Some(n) => n.clone(),
            None => l.clone(),
        });
    }
    write_out(REG, out.join("\n") + "\n", "register");
    println!("probe-map: {fixed} gesetzt, {failed} ohne 2xx");
}

fn dataset_id(block: &[String]) -> Option<String> {
    for line in block {
        let bytes = line.as_bytes();
        let mut i = 0;
        while i + 2 < bytes.len() {
            if bytes[i] == b'd' && bytes[i + 1] == b's' && bytes[i + 2].is_ascii_digit() {
                let mut j = i + 2;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                if j - (i + 2) >= 5 {
                    return Some(line[i..j].to_string());
                }
                i = j;
            } else {
                i += 1;
            }
        }
    }
    None
}

fn probe_openneuro() {
    let today = match Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
    {
        Some(v) => v,
        None => String::new(),
    };
    let reg = fs::read_to_string(REG).expect("register");
    let lines: Vec<String> = reg.lines().map(str::to_string).collect();
    let mut repl: BTreeMap<usize, String> = BTreeMap::new();
    let (mut fixed, mut without_id, mut failed) = (0usize, 0usize, 0usize);
    for (start, block) in blocks(&reg) {
        let Some(pos) = block
            .iter()
            .position(|l| l.trim_start().starts_with("probe "))
        else {
            continue;
        };
        let p: Vec<&str> = block[pos].split_whitespace().collect();
        if p.len() < 4 || !p[3..].join(" ").contains("openneuro.org/crn/graphql") {
            continue;
        }
        let Some(id) = dataset_id(&block) else {
            without_id += 1;
            continue;
        };
        let target = format!("https://openneuro.org/datasets/{id}");
        let out = Command::new("curl")
            .args(["-sSL", "-o", "/dev/null", "-w", "%{http_code}", "--max-time", "30", &target])
            .output();
        let status = match out
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
        {
            Some(v) => v,
            None => String::new(),
        };
        let abs = start - 1 + pos;
        if matches!(status.as_str(), "200" | "206" | "204" | "202") {
            repl.insert(abs, format!("  probe {today}T00:00:00Z {status} {target}"));
            fixed += 1;
        } else {
            failed += 1;
        }
    }
    let mut out: Vec<String> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        out.push(match repl.get(&i) {
            Some(n) => n.clone(),
            None => l.clone(),
        });
    }
    write_out(REG, out.join("\n") + "\n", "register");
    println!("probe-openneuro: {fixed} gesetzt, {failed} ohne 2xx, {without_id} ohne Datensatz-ID");
}

fn untangle_probes() {
    let reg = fs::read_to_string(REG).expect("register");
    let lines: Vec<String> = reg.lines().map(str::to_string).collect();
    let mut repl: BTreeMap<usize, String> = BTreeMap::new();
    let mut fixed = 0usize;
    let today = match Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
    {
        Some(v) => v,
        None => String::new(),
    };
    for (start, block) in blocks(&reg) {
        let Some(pos) = block
            .iter()
            .position(|l| l.trim_start().starts_with("probe "))
        else {
            continue;
        };
        let p: Vec<&str> = block[pos].split_whitespace().collect();
        if p.len() < 5 {
            continue;
        }
        let mut idx = 3usize;
        while idx + 1 < p.len() && p[idx].chars().all(|c| c.is_ascii_digit()) {
            idx += 1;
        }
        if idx == 3 {
            continue;
        }
        let url = p[idx..].join(" ");
        if !url.starts_with("http") {
            continue;
        }
        let out = Command::new("curl")
            .args(["-sSL", "-o", "/dev/null", "-w", "%{http_code}", "--max-time", "10", &url])
            .output();
        let status = match out
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
        {
            Some(v) => v,
            None => String::new(),
        };
        let abs = start - 1 + pos;
        if matches!(status.as_str(), "200" | "206" | "204" | "202") {
            repl.insert(abs, format!("  probe {today}T00:00:00Z {status} {url}"));
            fixed += 1;
        } else {
            repl.insert(abs, format!("  probe {today}T00:00:00Z {status} {url}"));
        }
    }
    let mut out: Vec<String> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        out.push(match repl.get(&i) {
            Some(n) => n.clone(),
            None => l.clone(),
        });
    }
    write_out(REG, out.join("\n") + "\n", "register");
    println!("untangle-probes: {fixed} auf 2xx, {} Zeilen entwirrt", repl.len());
}

fn set_header(path: &str) {
    let tsv = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let text = fs::read_to_string(PHI).expect("phi");
    let mut groups: Vec<Vec<String>> = Vec::new();
    for (_, b) in blocks(&text) {
        groups.push(b);
    }
    let mut changed = 0usize;
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(url), Some(hdr)) = (it.next(), it.next()) else {
            continue;
        };
        let hdr = hdr.trim();
        if !hdr.starts_with("header ") {
            continue;
        }
        let base = url.trim().rsplit('/').next().unwrap_or(url.trim());
        let Some(b) = groups.iter_mut().find(|b| {
            field(b, "url")
                .map(|u| u == url.trim() || u.ends_with(&format!("/{base}")))
                .unwrap_or(false)
        }) else {
            continue;
        };
        b.retain(|l| !l.trim_start().starts_with("header "));
        let pos = b
            .iter()
            .position(|l| l.trim_start().starts_with("format "))
            .map(|i| i + 1)
            .unwrap_or(b.len());
        b.insert(pos, hdr.to_string());
        changed += 1;
    }
    let mut out: Vec<String> = Vec::new();
    for b in groups {
        out.extend(b);
        out.push(String::new());
    }
    write_out(PHI, out.join("\n") + "\n", "phi");
    println!("set-header: {changed} Bloecke gesetzt");
}

fn list_missing(key: &str) {
    let reg = fs::read_to_string(REG).expect("register");
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for (_, b) in blocks(&reg) {
        let has = b.iter().any(|l| l.trim_start().starts_with(&format!("{key} ")));
        if has {
            continue;
        }
        let host = or_empty(field(&b, "host"));
        if host.is_empty() {
            continue;
        }
        *counts.entry(host).or_insert(0) += 1;
    }
    let mut rows: Vec<(String, usize)> = counts.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));
    for (h, n) in &rows {
        println!("{n}\t{h}");
    }
    println!("list-missing {key}: {} Hosts, {} Zeilen", rows.len(), rows.iter().map(|r| r.1).sum::<usize>());
}

fn set_host_doc(path: &str) {
    let tsv = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut docs: BTreeMap<String, (String, String)> = BTreeMap::new();
    for line in tsv.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 3 {
            continue;
        }
        let host = cols[0].trim();
        let url = cols[1].trim();
        let quote = cols[2].trim();
        if host.is_empty() || url.is_empty() || url == "-" {
            continue;
        }
        docs.insert(host.to_string(), (url.to_string(), quote.to_string()));
    }
    let reg = fs::read_to_string(REG).expect("register");
    let mut out: Vec<String> = Vec::new();
    let mut rows = 0usize;
    for (_, mut b) in blocks(&reg) {
        let host = or_empty(field(&b, "host"));
        if let Some((url, quote)) = docs.get(&host) {
            set_row_key(&mut b, "host_cite", url);
            if !quote.is_empty() {
                set_row_key(&mut b, "host_quote", &format!("\"{quote}\""));
            }
            rows += 1;
        }
        out.extend(b);
        out.push(String::new());
    }
    write_out(REG, out.join("\n") + "\n", "register");
    println!("set-host-doc: {rows} Zeilen mit {} Hostdokumenten", docs.len());
}

fn list_missing_rows(key: &str) {
    let reg = fs::read_to_string(REG).expect("register");
    for (_, b) in blocks(&reg) {
        let has = b
            .iter()
            .any(|l| l.trim_start().starts_with(&format!("{key} ")));
        if has {
            continue;
        }
        let host = or_empty(field(&b, "host"));
        let applies = or_empty(field(&b, "applies"));
        let status = or_empty(field(&b, "status"));
        println!("{host}\t{status}\t{applies}");
    }
}

fn looks_html(s: &str) -> bool {
    s.contains("</") || s.contains("<!") || s.contains("<html") || s.contains("<a href") || s.contains("<BR") || s.contains("<br")
}

fn strip_markup(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&amp;", "&")
        .replace("&#177;", "±")
        .replace("&plusmn;", "±")
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}

fn verify_cites(path: &str) {
    let tsv = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let (mut ok, mut bad_http, mut bad_quote, mut empty) = (0usize, 0usize, 0usize, 0usize);
    for line in tsv.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 7 {
            continue;
        }
        let pairs = [
            (cols[2].trim(), cols[3].trim(), "instrument"),
            (cols[4].trim(), cols[5].trim(), "data"),
        ];
        for (url, quote, kind) in pairs {
            if url.is_empty() || url == "-" || !url.starts_with("http") {
                empty += 1;
                continue;
            }
            let out = Command::new("curl")
                .args(["-sSL", "--max-time", "30", url])
                .output();
            let raw = match out {
                Ok(o) => o.stdout,
                Err(_) => Vec::new(),
            };
            if raw.is_empty() {
                bad_http += 1;
                println!("HTTP\t{kind}\t{url}");
                continue;
            }
            let needle: String = quote
                .trim_matches('"')
                .split_whitespace()
                .take(8)
                .collect::<Vec<_>>()
                .join(" ");
            let mut text = String::from_utf8_lossy(&raw).to_string();
            if raw.starts_with(b"PK\x03\x04") || url.to_lowercase().ends_with(".zip") {
                let tmp = std::env::temp_dir().join(format!("zip-{}", std::process::id()));
                let _ = fs::write(&tmp, &raw);
                if let Ok(o) = Command::new("unzip").args(["-p", &tmp.to_string_lossy()]).output() {
                    text = String::from_utf8_lossy(&o.stdout).to_string();
                }
                let _ = fs::remove_file(&tmp);
            }
            if raw.starts_with(b"%PDF") || url.to_lowercase().ends_with(".pdf") {
                let tmp = std::env::temp_dir().join(format!("cite-{}", std::process::id()));
                let _ = fs::write(&tmp, &raw);
                if let Ok(o) = Command::new("pdftotext")
                    .args(["-layout", &tmp.to_string_lossy(), "-"])
                    .output()
                {
                    text = String::from_utf8_lossy(&o.stdout).to_string();
                }
                let _ = fs::remove_file(&tmp);
            }
            if looks_html(&text) {
                text = strip_markup(&text);
            }
            if text.contains('\u{FFFD}') {
                let tmp = std::env::temp_dir().join(format!("enc-{}", std::process::id()));
                let _ = fs::write(&tmp, &raw);
                if let Ok(o) = Command::new("iconv")
                    .args(["-f", "cp1251", "-t", "utf-8", &tmp.to_string_lossy()])
                    .output()
                {
                    text = String::from_utf8_lossy(&o.stdout).to_string();
                }
                let _ = fs::remove_file(&tmp);
            }
            text = text.replace("-\n", "").replace("-\r\n", "");
            let hay: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
            if needle.is_empty() || quote_holds(&hay, quote) {
                ok += 1;
            } else {
                let reader = format!("https://r.jina.ai/{url}");
                let rbody = match Command::new("curl")
                    .args(["-sSL", "--max-time", "40", &reader])
                    .output()
                {
                    Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                    Err(_) => String::new(),
                };
                let rhay: String = rbody.split_whitespace().collect::<Vec<_>>().join(" ");
                if !needle.is_empty() && quote_holds(&rhay, quote) {
                    ok += 1;
                } else {
                    bad_quote += 1;
                    println!("ZITAT\t{kind}\t{url}\t{needle}");
                }
            }
        }
    }
    println!("verify-cites {path}: ok={ok} http={bad_http} zitat={bad_quote} leer={empty}");
}

fn set_doc_quote(path: &str) {
    let tsv = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let reg = fs::read_to_string(REG).expect("register");
    let mut out: Vec<String> = Vec::new();
    let mut rows = 0usize;
    for (_, mut b) in blocks(&reg) {
        let api_cite = or_empty(field(&b, "api_cite"));
        let inst_cite = or_empty(field(&b, "instrument_cite"));
        let mut hit: Option<(String, String, String)> = None;
        for line in tsv.lines() {
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 3 {
                continue;
            }
            let doc = cols[0].trim();
            let norm = |s: &str| {
                s.trim()
                    .trim_end_matches('/')
                    .replace("http://", "https://")
                    .to_lowercase()
            };
            let matches_api = norm(&api_cite) == norm(doc);
            let matches_inst = norm(&inst_cite) == norm(doc);
            if matches_api || matches_inst {
                let beleg = match cols.get(3) {
                    Some(s) => s.trim().to_string(),
                    None => String::new(),
                };
                hit = Some((doc.to_string(), cols[2].trim().to_string(), beleg));
                break;
            }
        }
        if let Some((doc, quote, beleg)) = hit {
            let q = quote.trim_matches('"');
            let norm = |s: &str| {
                s.trim()
                    .trim_end_matches('/')
                    .replace("http://", "https://")
                    .to_lowercase()
            };
            if norm(&api_cite) == norm(&doc) {
                let beleg = beleg.clone();
                if !beleg.is_empty() && beleg.starts_with("http") {
                    set_row_key(&mut b, "api_cite", &beleg);
                }
                set_row_key(&mut b, "api_quote", &format!("\"{q}\""));
            }
            if norm(&inst_cite) == norm(&doc) {
                let beleg = beleg.clone();
                if !beleg.is_empty() && beleg.starts_with("http") {
                    set_row_key(&mut b, "instrument_cite", &beleg);
                }
                set_row_key(&mut b, "instrument_quote", &format!("\"{q}\""));
            }
            rows += 1;
        }
        out.extend(b);
        out.push(String::new());
    }
    write_out(REG, out.join("\n") + "\n", "register");
    println!("set-doc-quote: {rows} Zeilen gesetzt");
}

fn quote_holds(hay: &str, quote: &str) -> bool {
    let q = quote.trim().trim_matches('"');
    let mut pos = 0usize;
    for seg in q.split(" ... ") {
        let needle: String = seg.split_whitespace().take(6).collect::<Vec<_>>().join(" ");
        if needle.is_empty() {
            continue;
        }
        match hay.get(pos..).and_then(|rest| rest.find(&needle)) {
            Some(p) => pos += p + needle.len(),
            None => return false,
        }
    }
    true
}

fn verify_register(filter_path: &str) {
    use std::collections::BTreeSet;
    let filter = match fs::read_to_string(filter_path) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    let mut docs: BTreeSet<String> = BTreeSet::new();
    for line in filter.lines() {
        if let Some(u) = line.split('\t').next() {
            let u = u.trim();
            if u.starts_with("http") {
                docs.insert(u.to_string());
            }
        }
    }
    let reg = fs::read_to_string(REG).expect("register");
    let (mut ok, mut bad, mut http) = (0usize, 0usize, 0usize);
    for (_, b) in blocks(&reg) {
        let pairs = [
            (or_empty(field(&b, "instrument_cite")), or_empty(field(&b, "instrument_quote")), "instrument"),
            (or_empty(field(&b, "api_cite")), or_empty(field(&b, "api_quote")), "data"),
        ];
        for (url, quote, kind) in pairs {
            if url.is_empty() || quote.is_empty() || !docs.contains(&url) {
                continue;
            }
            let out = Command::new("curl")
                .args(["-sSL", "--max-time", "30", &url])
                .output();
            let raw = match out {
                Ok(o) => o.stdout,
                Err(_) => Vec::new(),
            };
            let mut text = String::from_utf8_lossy(&raw).to_string();
            if raw.starts_with(b"PK\x03\x04") || url.to_lowercase().ends_with(".zip") {
                let tmp = std::env::temp_dir().join(format!("zip-{}", std::process::id()));
                let _ = fs::write(&tmp, &raw);
                if let Ok(o) = Command::new("unzip").args(["-p", &tmp.to_string_lossy()]).output() {
                    text = String::from_utf8_lossy(&o.stdout).to_string();
                }
                let _ = fs::remove_file(&tmp);
            }
            if text.is_empty() {
                http += 1;
                println!("HTTP\t{kind}\t{url}");
                continue;
            }
            if raw.starts_with(b"%PDF") || url.to_lowercase().ends_with(".pdf") {
                let tmp = std::env::temp_dir().join(format!("reg-{}", std::process::id()));
                let _ = fs::write(&tmp, &raw);
                if let Ok(o) = Command::new("pdftotext")
                    .args(["-layout", &tmp.to_string_lossy(), "-"])
                    .output()
                {
                    text = String::from_utf8_lossy(&o.stdout).to_string();
                }
                let _ = fs::remove_file(&tmp);
            }
            if looks_html(&text) {
                text = strip_markup(&text);
            }
            if text.contains('\u{FFFD}') {
                let tmp = std::env::temp_dir().join(format!("enc-{}", std::process::id()));
                let _ = fs::write(&tmp, text.as_bytes());
                if let Ok(o) = Command::new("iconv")
                    .args(["-f", "cp1251", "-t", "utf-8", &tmp.to_string_lossy()])
                    .output()
                {
                    text = String::from_utf8_lossy(&o.stdout).to_string();
                }
                let _ = fs::remove_file(&tmp);
            }
            text = text.replace("-\n", "").replace("-\r\n", "");
            let needle: String = quote
                .trim_matches('"')
                .split_whitespace()
                .take(8)
                .collect::<Vec<_>>()
                .join(" ");
            let hay: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
            if !needle.is_empty() && quote_holds(&hay, &quote) {
                ok += 1;
            } else {
                let reader = format!("https://r.jina.ai/{url}");
                let rbody = match Command::new("curl")
                    .args(["-sSL", "--max-time", "45", &reader])
                    .output()
                {
                    Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                    Err(_) => String::new(),
                };
                let rhay: String = rbody.split_whitespace().collect::<Vec<_>>().join(" ");
                if !needle.is_empty() && quote_holds(&rhay, &quote) {
                    ok += 1;
                } else {
                    bad += 1;
                    println!("ZITAT\t{kind}\t{url}\t{needle}");
                }
            }
        }
    }
    println!("verify-register: ok={ok} zitat={bad} http={http}");
}

fn block_audit() {
    let reg = fs::read_to_string(REG).expect("register");
    let (mut nohost, mut dup) = (0usize, 0usize);
    for (start, b) in blocks(&reg) {
        let first = match b.first() {
            Some(l) => l.clone(),
            None => String::new(),
        };
        if !first.trim_start().starts_with("host ") {
            nohost += 1;
            println!("OHNE-HOST\t{}\t{}", start + 1, first.chars().take(80).collect::<String>());
        }
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for line in &b {
            let key = match line.split_whitespace().next() {
                Some(k) => k.to_string(),
                None => continue,
            };
            if key == "host" || key.starts_with('#') {
                continue;
            }
            if !seen.insert(key.clone()) {
                dup += 1;
                println!("DOPPELT\t{}\t{}\t{}", start + 1, key, first.chars().take(60).collect::<String>());
            }
        }
    }
    println!("block-audit: {nohost} Bloecke ohne host, {dup} doppelte Schluessel");
}

fn stray_report(limit: usize) {
    let reg = fs::read_to_string(REG).expect("register");
    let mut shown = 0usize;
    let mut blocks_with_tail = 0usize;
    let mut sigs: BTreeMap<String, usize> = BTreeMap::new();
    for (start, b) in blocks(&reg) {
        let tail_at = b.iter().enumerate().skip(1).find_map(|(i, l)| {
            if l.starts_with(' ') || l.starts_with('#') || l.trim().is_empty() {
                None
            } else {
                Some(i)
            }
        });
        let Some(i) = tail_at else {
            continue;
        };
        blocks_with_tail += 1;
        let tail = &b[i..];
        let mut keys: Vec<String> = Vec::new();
        for l in tail {
            let k = match l.split_whitespace().next() {
                Some(k) => k.to_string(),
                None => continue,
            };
            if !keys.contains(&k) {
                keys.push(k);
            }
        }
        *sigs.entry(keys.join("+")).or_insert(0) += 1;
        if shown >= limit {
            continue;
        }
        shown += 1;
        let val = |lines: &[String], k: &str| -> String {
            match lines
                .iter()
                .find(|l| l.trim_start().starts_with(&format!("{k} ")))
            {
                Some(l) => l.trim_start()[k.len() + 1..].chars().take(70).collect(),
                None => String::new(),
            }
        };
        let head = &b[..i];
        let tail = &b[i..];
        println!("== Block ab Zeile {} · host {}", start + 1, val(head, "host"));
        println!("   applies       {}", val(head, "applies"));
        println!("   Kopf api      {}", val(head, "api_cite"));
        println!("   Rest api      {}", val(tail, "api_cite"));
        println!("   Kopf instr    {}", val(head, "instrument_cite"));
        println!("   Rest instr    {}", val(tail, "instrument_cite"));
        println!("   Kopf Geraet   {}", val(head, "instrument"));
        println!("   Rest Geraet   {}", val(tail, "instrument"));
    }
    for (sig, n) in &sigs {
        println!("SIGNATUR\t{n}\t{sig}");
    }
    println!("stray-report: {blocks_with_tail} Bloecke mit Restzeilen");
}

fn tail_value(lines: &[String], key: &str) -> String {
    match lines
        .iter()
        .find(|l| l.trim_start().starts_with(&format!("{key} ")))
    {
        Some(l) => l.trim_start()[key.len() + 1..].trim().to_string(),
        None => String::new(),
    }
}

fn fix_stray(mode: &str) {
    let reg = fs::read_to_string(REG).expect("register");
    let (mut fixed, open, mut skipped, mut flipped) = (0usize, 0usize, 0usize, 0usize);
    let mut report: Vec<String> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    for (start, b) in blocks(&reg) {
        let unindented = b.iter().enumerate().skip(1).find_map(|(i, l)| {
            if l.starts_with(' ') || l.starts_with('#') || l.trim().is_empty() {
                None
            } else {
                Some(i)
            }
        });
        let (mut seen_api, mut seen_inst, mut dup_at) = (0usize, 0usize, None);
        if unindented.is_none() {
            for (i, l) in b.iter().enumerate().skip(1) {
                let t = l.trim_start();
                if t.starts_with("api_cite ") {
                    seen_api += 1;
                    if seen_api == 2 {
                        dup_at = Some(i);
                        break;
                    }
                }
                if t.starts_with("instrument_cite ") {
                    seen_inst += 1;
                    if seen_inst == 2 {
                        dup_at = Some(i);
                        break;
                    }
                }
            }
        }
        let from_dup = dup_at.is_some();
        let tail_at = match unindented {
            Some(v) => Some(v),
            None => dup_at,
        };
        let Some(i) = tail_at else {
            out.extend(b);
            out.push(String::new());
            continue;
        };
        let mut head: Vec<String> = b[..i].to_vec();
        let tail: Vec<String> = b[i..].to_vec();
        let tail_keys: BTreeSet<String> = tail
            .iter()
            .filter_map(|l| l.split_whitespace().next().map(|k| k.to_string()))
            .collect();
        let only_instrument = tail_keys.iter().all(|k| k == "instrument");
        let has_docs = tail_keys.contains("api_cite")
            && tail_keys.contains("instrument_cite")
            && !tail_keys.contains("applies")
            && !tail_keys.contains("host_cite");
        let has_own_id = tail_keys.contains("applies") || tail_keys.contains("host_cite");
        let api_only = tail_keys.contains("api_cite")
            && tail_keys.contains("api_quote")
            && !tail_keys.contains("instrument_cite")
            && !has_own_id;

        let take = match mode {
            "instrument-only" => only_instrument,
            "doc-triple" => has_docs,
            "split-entry" => has_own_id,
            "api-only" => api_only,
            "dup-docs" => from_dup,
            _ => false,
        };
        if !take {
            skipped += 1;
            out.extend(b);
            out.push(String::new());
            continue;
        }
        if mode == "doc-triple" {
            let head_api = or_empty(field(&head, "api_cite"));
            let tail_api = tail_value(&tail, "api_cite");
            if !head_api.is_empty() && head_api != tail_api {
                flipped += 1;
                let tail_instr = tail_value(&tail, "instrument_cite");
                report.push(format!("{}\t{}\t{}\t{}", start + 1, head_api, tail_api, tail_instr));
            }
        }
        if mode == "split-entry" {
            let head_applies = or_empty(field(&head, "applies"));
            let tail_applies = tail_value(&tail, "applies");
            let distinct = !tail_applies.is_empty() && tail_applies != head_applies;
            if distinct {
                let host = or_empty(field(&head, "host"));
                out.extend(head);
                out.push(String::new());
                out.push(format!("host {host}"));
                for line in &tail {
                    out.push(format!("  {}", line.trim()));
                }
                out.push(String::new());
                fixed += 1;
                continue;
            }
        }
        for line in &tail {
            let t = line.trim_start();
            let mut it = t.splitn(2, ' ');
            let key = match it.next() {
                Some(k) => k,
                None => continue,
            };
            if key.is_empty() {
                continue;
            }
            let value = it.next().unwrap_or("").trim();
            if key == "instrument" {
                head.push(format!("  instrument {value}"));
                continue;
            }
            set_row_key(&mut head, key, value);
        }
        fixed += 1;
        if mode == "dup-docs" {
            println!("DUP-MERGE\t{}\ttail_bei\t{}", start + 1, i + 1);
        }
        out.extend(head);
        out.push(String::new());
    }
    write_out(REG, out.join("\n") + "\n", "register");
    if let Ok(p) = std::env::var("OMEGAFLOW_STRAY_REPORT") {
        let _ = fs::write(&p, report.join("\n") + "\n");
    }
    println!("fix-stray {mode}: {fixed} Bloecke zusammengefuehrt, {flipped} mit anderem Datendokument, {open} unentschieden, {skipped} andere Muster");
}

fn entry_audit() {
    let reg = fs::read_to_string(REG).expect("register");
    let (mut host_in_block, mut multi_applies, mut multi_block) = (0usize, 0usize, 0usize);
    for (start, b) in blocks(&reg) {
        let hosts = b.iter().skip(1).filter(|l| l.starts_with("host ")).count();
        if hosts > 0 {
            host_in_block += hosts;
            println!("HOST-IM-BLOCK\t{}\t{hosts}", start + 1);
        }
        let applies = b
            .iter()
            .filter(|l| l.trim_start().starts_with("applies "))
            .count();
        if applies > 1 {
            multi_applies += 1;
            println!("MULTI-APPLIES\t{}\t{applies}", start + 1);
        }
        let blocks_n = b
            .iter()
            .filter(|l| l.trim_start().starts_with("block "))
            .count();
        if blocks_n > 1 {
            multi_block += 1;
        }
    }
    println!("entry-audit: {host_in_block} host-Zeilen in Bloecken, {multi_applies} Bloecke mit mehreren applies, {multi_block} mit mehreren block");
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
        Some("apply-fields") if args.len() >= 3 => apply_fields(&args[2]),
        Some("instrumentize") if args.len() >= 3 => instrumentize(&args[2]),
        Some("set-instruments") if args.len() >= 3 => set_instruments(&args[2]),
        Some("apply-instruments-url") if args.len() >= 3 => apply_instruments_url(&args[2]),
        Some("set-instruments-by-asset") if args.len() >= 3 => set_instruments_by_asset(&args[2]),
        Some("aussondern-assets") if args.len() >= 4 => aussondern_assets(&args[2], &args[3]),
        Some("aussondern-format") if args.len() >= 4 => aussondern_format(&args[2], &args[3]),
        Some("replace-blocks") if args.len() >= 3 => replace_blocks(&args[2]),
        Some("split-field") if args.len() >= 5 => split_field(&args[2], &args[3], &args[4]),
        Some("merge-rebuild") if args.len() >= 4 => merge_rebuild(&args[2], &args[3]),
        Some("derived-clean") if args.len() >= 3 => derived_clean(&args[2]),
        Some("derived-dedup") if args.len() >= 3 => derived_dedup(&args[2]),
        Some("dups") if args.len() >= 3 => dups(&args[2]),
        Some("suggest-assets") if args.len() >= 4 => suggest_assets(&args[2], &args[3]),
        Some("apply-specs") if args.len() >= 4 => apply_specs(&args[2], &args[3]),
        Some("url-cut") if args.len() >= 4 => url_cut(&args[2], &args[3]),
        Some("align-many") if args.len() >= 3 => align_many(&args[2]),
        Some("stitch-orphans") => stitch_orphans(),
        Some("set-fields") if args.len() >= 3 => set_fields(&args[2]),
        Some("audit-blocks") => audit_blocks(),
        Some("reprobe") if args.len() >= 3 => reprobe(&args[2]),
        Some("normalize") => normalize_blocks(),
        Some("add-line") if args.len() >= 3 => add_line(&args[2]),
        Some("refs-audit") if args.len() >= 3 => refs_audit(&args[2]),
        Some("probe-terms") if args.len() >= 3 => probe_terms(&args[2]),
        Some("probe-map") if args.len() >= 3 => probe_map(&args[2]),
        Some("probe-openneuro") => probe_openneuro(),
        Some("untangle-probes") => untangle_probes(),
        Some("set-header") if args.len() >= 3 => set_header(&args[2]),
        Some("list-missing") if args.len() >= 3 => list_missing(&args[2]),
        Some("list-missing-rows") if args.len() >= 3 => list_missing_rows(&args[2]),
        Some("set-host-doc") if args.len() >= 3 => set_host_doc(&args[2]),
        Some("set-doc-quote") if args.len() >= 3 => set_doc_quote(&args[2]),
        Some("verify-register") if args.len() >= 3 => verify_register(&args[2]),
        Some("block-audit") => block_audit(),
        Some("entry-audit") => entry_audit(),
        Some("stray-report") => stray_report(12),
        Some("fix-stray") if args.len() >= 3 => fix_stray(&args[2]),
        Some("verify-cites") if args.len() >= 3 => verify_cites(&args[2]),
        Some("filter-rows") if args.len() >= 5 => filter_rows(&args[2], &args[3], &args[4]),
        Some("fill-sha") => fill_sha(
            args.get(2).map(String::as_str),
            args.get(3)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(200_000_000),
        ),
        Some("aussondern") if args.len() >= 4 => aussondern(&args[2], &args[3], args.get(4).map(String::as_str)),
        Some("restore-missing") if args.len() >= 3 => restore_missing(&args[2]),
        Some("verify-specs") => verify_specs(),
        Some("sync-fields") => sync_fields(),
        Some("repair-instruments") => repair_instruments(),
        _ => {
            eprintln!("spec_merge rows <out.φ> <states.φ> | instruments <tsv> | hostdocs");
            std::process::exit(2);
        }
    }
}
