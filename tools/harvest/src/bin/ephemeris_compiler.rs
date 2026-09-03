use omegaflow::bpc::BpcFile;
use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::{body_url, upload_asset};
use omegaflow::fk::FkFile;
use omegaflow::lsk::days_from_civil;
use omegaflow::mat::matmul;
use omegaflow::pck::PckBody;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::io::Write;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use omegaflow::ephemeris::{
    body_table, extract_granules, iau_angles_from_matrix, libration_matrix, pck_id_of,
    write_binary, ASTEROID_GRANULE_DAYS, GRANULE_DAYS, J2000_EPOCH,
};

fn flatten_targets(kernels: &[SpkFile]) -> Vec<(i32, String, Option<i32>)> {
    let table = body_table();
    let mut by_id: BTreeMap<i32, (String, Option<i32>)> = BTreeMap::new();
    for spk in kernels {
        for seg in spk.segments() {
            if let Some(b) = table.get(&seg.target) {
                by_id
                    .entry(seg.target)
                    .or_insert_with(|| (b.name.clone(), b.parent));
            }
        }
    }
    let mut seen = HashSet::new();
    by_id
        .into_iter()
        .filter(|(_, (name, _))| seen.insert(name.clone()))
        .map(|(id, (name, parent))| (id, name, parent))
        .collect()
}

#[derive(Clone)]
struct IndexEntry {
    url: String,
    name: String,
    family: String,
    size: u64,
    mtime: u64,
}

fn family_of(url: &str) -> &'static str {
    let name = url.rsplit('/').next().unwrap_or(url);
    if name == "gm_Horizons.pck" || name.starts_with("gm_") {
        return "gm";
    }
    if name == "naif0012.tls" || name.ends_with(".tls") {
        return "lsk";
    }
    if name.contains("dcom") || name.contains("dastcom") {
        return "dastcom";
    }
    if name.ends_with(".bsp") {
        if url.contains("/satellites/") {
            return "spk-satellites";
        }
        if url.contains("/planets/") {
            return "spk-planets";
        }
        return "spk";
    }
    if name.ends_with(".bpc") {
        return "bpc";
    }
    if name.ends_with(".bc") {
        return "ck";
    }
    if name.ends_with(".tpc") || name.ends_with(".ker") {
        return "pck-text";
    }
    if name.ends_with(".tf") {
        return "fk";
    }
    if name.ends_with(".tsc") {
        return "sclk";
    }
    if name.ends_with(".ck") {
        return "ck";
    }
    if name.ends_with(".ti") {
        return "ik";
    }
    if name.ends_with(".db") {
        return "dbk";
    }
    if name.ends_with(".ek") {
        return "ek";
    }
    if name.ends_with(".bds") {
        return "dsk";
    }
    if name.ends_with(".tm") {
        return "mk";
    }
    "misc"
}

fn fetch_text(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--max-time")
        .arg("90")
        .arg("--retry")
        .arg("2")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        None
    }
}

fn text_of(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
            continue;
        }
        if c == '>' {
            in_tag = false;
            continue;
        }
        if !in_tag {
            out.push(c);
        }
    }
    out
}

fn extract_meta(after: &str) -> (u64, u64) {
    let cap = 400usize.min(after.len());
    let mut end = cap;
    while !after.is_char_boundary(end) {
        end -= 1;
    }
    let text = text_of(&after[..end]);
    let b = text.as_bytes();
    let mut mtime = 0u64;
    let mut size = 0u64;
    let mut date_end = 0usize;
    let mut i = 0;
    while i + 10 < b.len() {
        if b[i].is_ascii_digit() && b[i + 4] == b'-' && b[i + 7] == b'-' {
            let ok = text[i..i + 4].parse::<i64>().ok().is_some()
                && text[i + 5..i + 7].parse::<i64>().ok().is_some()
                && text[i + 8..i + 10].parse::<i64>().ok().is_some();
            if ok {
                let y: i64 = text[i..i + 4].parse().unwrap();
                let mo: i64 = text[i + 5..i + 7].parse().unwrap();
                let d: i64 = text[i + 8..i + 10].parse().unwrap();
                if let Some(days) = days_from_civil(y, mo, d) {
                    mtime = days as u64 * 86400;
                }
                if i + 16 <= b.len() {
                    if let (Ok(h), Ok(mn)) = (
                        text[i + 11..i + 13].parse::<u64>(),
                        text[i + 14..i + 16].parse::<u64>(),
                    ) {
                        mtime += h * 3600 + mn * 60;
                    }
                }
                date_end = i + 16;
                break;
            }
        }
        i += 1;
    }
    let tail = if date_end > 0 {
        &text[date_end..]
    } else {
        &text[..]
    };
    for tok in tail.split_whitespace() {
        let upper = tok.to_uppercase();
        let (num, mult) = if let Some(n) = upper.strip_suffix('T') {
            (n, 1u64 << 40)
        } else if let Some(n) = upper.strip_suffix('G') {
            (n, 1u64 << 30)
        } else if let Some(n) = upper.strip_suffix('M') {
            (n, 1u64 << 20)
        } else if let Some(n) = upper.strip_suffix('K') {
            (n, 1u64 << 10)
        } else {
            (upper.as_str(), 1u64)
        };
        if num.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(v) = num.parse::<u64>() {
                size = v * mult;
                break;
            }
        }
    }
    (size, mtime)
}

fn parse_listing(base: &str, html: &str, files: &mut Vec<IndexEntry>, dirs: &mut Vec<String>) {
    let mut rest = html;
    while let Some(pos) = rest.find("href=\"") {
        rest = &rest[pos + 6..];
        let end = match rest.find('"') {
            Some(e) => e,
            None => break,
        };
        let href = &rest[..end];
        if href.starts_with('?') || href.starts_with('#') || href.starts_with('/') {
            continue;
        }
        if href == "../" || href == "./" || href.contains("Parent Directory") {
            continue;
        }
        let after = &rest[end..];
        let (size, mtime) = extract_meta(after);
        let name = href
            .split('?')
            .next()
            .unwrap_or(href)
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or("")
            .replace("%20", " ");
        let full = if href.starts_with("http") {
            href.to_string()
        } else {
            format!("{}{}", base, href.replace(' ', "%20"))
        };
        if href.ends_with('/') {
            dirs.push(full);
        } else {
            let family = family_of(&full).to_string();
            files.push(IndexEntry {
                url: full,
                family,
                name,
                size,
                mtime,
            });
        }
    }
}

fn load_known_lines(out_path: &str) -> (HashSet<String>, HashSet<String>) {
    let mut dirs = HashSet::new();
    let mut files = HashSet::new();
    if let Ok(content) = std::fs::read_to_string(out_path) {
        for line in content.lines() {
            if let Some(url) = line.strip_prefix("dir ") {
                dirs.insert(url.trim().to_string());
            } else if let Some(rest) = line.strip_prefix("kernel ") {
                if let Some(url) = rest.split_whitespace().next() {
                    files.insert(url.to_string());
                }
            }
        }
    }
    (dirs, files)
}

fn crawl(roots: &[String], depth: usize, out_path: &str, delay_ms: u64, jobs: usize) {
    let (done_dirs, done_files) = load_known_lines(out_path);
    let done_dirs = Arc::new(done_dirs);
    let done_files = Arc::new(done_files);
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    for r in roots {
        queue.lock().unwrap().push_back((r.clone(), 0usize));
    }
    let lines = Arc::new(Mutex::new(Vec::new()));
    let written = Arc::new(Mutex::new(HashSet::new()));
    let dead = Arc::new(Mutex::new(HashSet::new()));
    let roots_shared = Arc::new(roots.to_vec());
    let file_count = Arc::new(AtomicUsize::new(0));
    let dir_count = Arc::new(AtomicUsize::new(0));
    let mut workers = Vec::new();
    for _ in 0..jobs.max(1) {
        let queue = Arc::clone(&queue);
        let lines = Arc::clone(&lines);
        let written = Arc::clone(&written);
        let dead = Arc::clone(&dead);
        let done_dirs = Arc::clone(&done_dirs);
        let done_files = Arc::clone(&done_files);
        let roots_shared = Arc::clone(&roots_shared);
        let file_count = Arc::clone(&file_count);
        let dir_count = Arc::clone(&dir_count);
        workers.push(std::thread::spawn(move || loop {
            let next = {
                let mut q = queue.lock().unwrap();
                q.pop_front()
            };
            let (url, d) = match next {
                Some(v) => v,
                None => break,
            };
            if done_dirs.contains(&url) {
                continue;
            }
            let html = match fetch_text(&url) {
                Some(h) => h,
                None => {
                    if dead.lock().unwrap().insert(url.clone()) {
                        eprintln!("index: listing returned void: {}", url);
                    }
                    continue;
                }
            };
            let mut files = Vec::new();
            let mut sub = Vec::new();
            parse_listing(&url, &html, &mut files, &mut sub);
            sub.retain(|s| roots_shared.iter().any(|r| s.starts_with(r.as_str())));
            {
                let mut w = written.lock().unwrap();
                let mut l = lines.lock().unwrap();
                for f in &files {
                    if w.insert(f.url.clone()) && !done_files.contains(&f.url) {
                        l.push(format!(
                            "kernel {} {} {} {}\n",
                            f.url, f.family, f.size, f.mtime
                        ));
                        file_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
                if w.insert(url.clone()) {
                    l.push(format!("dir {}\n", url));
                    dir_count.fetch_add(1, Ordering::Relaxed);
                }
            }
            if depth == 0 || d + 1 < depth {
                let mut q = queue.lock().unwrap();
                for s in sub {
                    q.push_back((s, d + 1));
                }
            }
            if delay_ms > 0 {
                std::thread::sleep(Duration::from_millis(delay_ms));
            }
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let mut out = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_path);
    if let Ok(f) = out.as_mut() {
        let l = lines.lock().unwrap();
        for line in l.iter() {
            let _ = std::io::Write::write_all(f, line.as_bytes());
        }
    }
    let _ = out.as_mut().map(|f| f.flush());
    eprintln!(
        "index: {} files, {} dirs → {}",
        file_count.load(Ordering::Relaxed),
        dir_count.load(Ordering::Relaxed),
        out_path
    );
}

fn load_index(path: &str) -> Vec<IndexEntry> {
    let mut entries = Vec::new();
    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("kernel ") {
                let mut parts = rest.split_whitespace();
                let url = match parts.next() {
                    Some(u) => u.to_string(),
                    None => continue,
                };
                let family = match parts.next() {
                    Some(f) => f.to_string(),
                    None => continue,
                };
                let size: u64 = match parts.next().and_then(|p| p.parse().ok()) {
                    Some(s) => s,
                    None => continue,
                };
                let mtime: u64 = match parts.next().and_then(|p| p.parse().ok()) {
                    Some(m) => m,
                    None => 0,
                };
                let name = url.rsplit('/').next().unwrap_or(&url).to_string();
                entries.push(IndexEntry {
                    url,
                    name,
                    family,
                    size,
                    mtime,
                });
            }
        }
    }
    entries
}

fn numeric_of(name: &str) -> u64 {
    let digits: String = name
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().unwrap_or(0)
}

fn base_of(name: &str) -> String {
    let no_part = match name.find("_part-") {
        Some(i) => &name[..i],
        None => name,
    };
    match no_part.rsplit_once('.') {
        Some((stem, _)) => stem.to_string(),
        None => no_part.to_string(),
    }
}

fn is_ssd(url: &str) -> bool {
    url.contains("ssd.jpl.nasa.gov")
}

fn is_light(name: &str) -> bool {
    name.contains('l') || name.ends_with('s')
}

fn pick_spk(candidates: &[IndexEntry]) -> Option<IndexEntry> {
    candidates
        .iter()
        .max_by(|a, b| {
            numeric_of(&a.name)
                .cmp(&numeric_of(&b.name))
                .then(is_ssd(&a.url).cmp(&is_ssd(&b.url)))
                .then(is_light(&a.name).cmp(&is_light(&b.name)))
                .then(b.name.len().cmp(&a.name.len()))
        })
        .cloned()
}

fn select_system(entries: &[IndexEntry], system: &str) -> Vec<IndexEntry> {
    let mut out = Vec::new();
    if system == "asteroids" {
        let mut asteroid_carrier = Vec::new();
        for e in entries
            .iter()
            .filter(|e| e.family == "spk" && e.name == "sb441-n16.bsp")
        {
            asteroid_carrier.push(e.clone());
        }
        if asteroid_carrier.is_empty() {
            eprintln!(
                "asteroids: sb441-n16.bsp absent from the index — the longbow has no carrier"
            );
            std::process::exit(1);
        }
        out.extend(asteroid_carrier);
        let mut sun_carrier = Vec::new();
        for e in entries
            .iter()
            .filter(|e| e.family == "spk-planets" && e.name == "de441.bsp")
        {
            sun_carrier.push(e.clone());
        }
        if sun_carrier.is_empty() {
            eprintln!(
                "asteroids: de441.bsp absent from the index — the sun's SSB state has no carrier"
            );
            std::process::exit(1);
        }
        out.extend(sun_carrier);
        out.sort_by(|a, b| numeric_of(&a.name).cmp(&numeric_of(&b.name)));
        return out;
    }
    if system == "planets" {
        let mut bases: BTreeMap<String, Vec<IndexEntry>> = BTreeMap::new();
        for e in entries.iter().filter(|e| e.family == "spk-planets") {
            bases
                .entry(base_of(&e.name))
                .or_insert_with(Vec::new)
                .push(e.clone());
        }
        if let Some((_, best)) = bases.iter().max_by(|(a, _), (b, _)| {
            numeric_of(a)
                .cmp(&numeric_of(b))
                .then((!is_light(a)).cmp(&(!is_light(b))))
                .then(b.len().cmp(&a.len()))
        }) {
            out.extend(best.clone());
        }
        for e in entries
            .iter()
            .filter(|e| e.family == "pck-text" && e.name.starts_with("pck0001"))
        {
            out.push(e.clone());
        }
        let bpc: Vec<IndexEntry> = entries
            .iter()
            .filter(|e| e.family == "bpc" && e.name.starts_with("moon_pa"))
            .cloned()
            .collect();
        if let Some(best) = pick_spk(&bpc) {
            out.push(best);
        }
        let fk_sel: Vec<IndexEntry> = entries
            .iter()
            .filter(|e| e.family == "fk" && e.name.starts_with("moon_de440"))
            .cloned()
            .collect();
        if let Some(best) = pick_spk(&fk_sel) {
            out.push(best);
        }
        out.sort_by(|a, b| numeric_of(&a.name).cmp(&numeric_of(&b.name)));
        return out;
    }
    let prefix = match system {
        "jupiter" => "jup",
        "saturn" => "sat",
        "mars" => "mar",
        "uranus" => "ura",
        "neptune" => "nep",
        "pluto" => "plu",
        _ => return out,
    };
    let spk: Vec<IndexEntry> = entries
        .iter()
        .filter(|e| e.family == "spk-satellites" && e.name.starts_with(prefix))
        .cloned()
        .collect();
    let moon_carriers: &[&str] = match system {
        "jupiter" => &["jup365"],
        "saturn" => &["sat441"],
        "neptune" => &["nep097"],
        _ => &[],
    };
    let mut ranked = spk.clone();
    ranked.sort_by(|a, b| {
        numeric_of(&b.name)
            .cmp(&numeric_of(&a.name))
            .then(is_ssd(&b.url).cmp(&is_ssd(&a.url)))
            .then(is_light(&b.name).cmp(&is_light(&a.name)))
            .then(a.name.len().cmp(&b.name.len()))
    });
    let mut bases: Vec<String> = Vec::new();
    for wanted in moon_carriers {
        if let Some(e) = spk.iter().find(|e| base_of(&e.name) == *wanted) {
            let base = base_of(&e.name);
            if !bases.contains(&base) {
                bases.push(base);
            }
        }
    }
    if bases.is_empty() {
        for e in &ranked {
            let base = base_of(&e.name);
            if !bases.contains(&base) {
                bases.push(base);
            }
            if bases.len() >= 2 {
                break;
            }
        }
    }
    let mut pushed: HashSet<String> = HashSet::new();
    for e in &spk {
        let base = base_of(&e.name);
        if bases.contains(&base) && pushed.insert(e.name.clone()) {
            out.push(e.clone());
        }
    }
    let pck: Vec<IndexEntry> = entries
        .iter()
        .filter(|e| e.family == "pck-text" && e.name.starts_with(&format!("pck.{}", prefix)))
        .cloned()
        .collect();
    if let Some(best) = pick_spk(&pck) {
        out.push(best);
    }
    out
}

fn download_missing(entries: &[IndexEntry], dest: &str) -> Vec<String> {
    let _ = std::fs::create_dir_all(dest);
    let mut paths = Vec::new();
    for e in entries {
        let path = format!("{}/{}", dest, e.name);
        let meta = std::fs::metadata(&path).ok();
        let fresh = match meta {
            Some(m) if e.size > 0 => m.len() == e.size,
            Some(_) => true,
            None => false,
        };
        if fresh {
            eprintln!("fetch: {} fresh ({} B)", e.name, e.size);
        } else {
            eprintln!("fetch: {} ({} B)", e.url, e.size);
            let status = Command::new("curl")
                .arg("-sSfL")
                .arg("--retry")
                .arg("3")
                .arg("--max-time")
                .arg("5400")
                .arg("-o")
                .arg(&path)
                .arg(&e.url)
                .status();
            match status {
                Ok(s) if s.success() => {}
                _ => {
                    eprintln!("fetch: {} returned void", e.url);
                    continue;
                }
            }
            let landed = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            if landed != e.size {
                eprintln!(
                    "fetch: {} size mismatch — landed {} B, index {} B; the download stays rejected",
                    e.name, landed, e.size
                );
                let _ = std::fs::remove_file(&path);
                continue;
            }
        }
        paths.push(path);
    }
    paths
}

fn classify(
    paths: &[String],
) -> (
    Vec<String>,
    Vec<String>,
    Option<String>,
    Vec<String>,
    Vec<String>,
) {
    let mut kernels = Vec::new();
    let mut bpcs = Vec::new();
    let mut fks = Vec::new();
    let mut gm_text = String::new();
    let mut pck_text = String::new();
    for p in paths {
        let name = p.rsplit('/').next().unwrap_or(p);
        let text = match family_of(p) {
            "gm" | "pck-text" => match std::fs::read_to_string(p) {
                Ok(t) => Some(t),
                Err(e) => {
                    eprintln!("read {}: {}", p, e);
                    continue;
                }
            },
            "spk-planets" | "spk-satellites" | "spk" => {
                kernels.push(p.clone());
                None
            }
            "bpc" => {
                bpcs.push(p.clone());
                None
            }
            "fk" => {
                fks.push(p.clone());
                None
            }
            family => {
                eprintln!(
                    "classify: {} family {} not consumed by flattener",
                    name, family
                );
                continue;
            }
        };
        if let Some(t) = text {
            match family_of(p) {
                "gm" => {
                    gm_text.push_str(&t);
                    gm_text.push('\n');
                }
                "pck-text" => {
                    pck_text.push_str(&t);
                    pck_text.push('\n');
                }
                _ => {}
            }
        }
    }
    let gm = if gm_text.is_empty() {
        None
    } else {
        Some(gm_text)
    };
    (kernels, bpcs, gm, vec![pck_text], fks)
}

fn update_index_bodies(index_path: &str, bodies: &[String]) {
    let content = match std::fs::read_to_string(index_path) {
        Ok(c) => c,
        Err(_) => return,
    };
    let mut out = String::new();
    for line in content.lines() {
        if !line.starts_with("body ") {
            out.push_str(line);
            out.push('\n');
        }
    }
    for name in bodies {
        out.push_str(&format!("body {} {}\n", name, body_url(name)));
    }
    if let Err(e) = std::fs::write(index_path, out) {
        eprintln!("update index {}: {}", index_path, e);
    }
}

fn flatten(
    kernels: &[String],
    bpcs: &[String],
    fks: &[String],
    gm_text: Option<&str>,
    pck_text: Option<&str>,
    ci_mode: bool,
    omega_g: Option<(String, f64, f64)>,
    small_bodies_only: bool,
) -> Vec<String> {
    let mut spk_files = Vec::new();
    for kernel_path in kernels {
        match SpkFile::open(kernel_path) {
            Ok(s) => spk_files.push(s),
            Err(e) => {
                eprintln!("open {}: {}", kernel_path, e);
                std::process::exit(1);
            }
        }
    }
    let mut bpc_files = Vec::new();
    for bpc_path in bpcs {
        match BpcFile::open(bpc_path) {
            Ok(b) => bpc_files.push(b),
            Err(e) => eprintln!("open {}: {}", bpc_path, e),
        }
    }
    let mut fk = FkFile::parse("");
    for fk_path in fks {
        match FkFile::open(fk_path) {
            Ok(f) => fk.insert_file(f),
            Err(e) => eprintln!("open {}: {}", fk_path, e),
        }
    }
    let pck_bodies: HashMap<i32, PckBody> = omegaflow::pck::parse(gm_text, pck_text);
    let targets = flatten_targets(&spk_files);
    let mut written = Vec::new();
    let mut upload_failed = 0usize;
    for (target_id, body_name, _) in &targets {
        if small_bodies_only && *target_id < 2000000 {
            continue;
        }
        let wgccre = match pck_bodies.get(&pck_id_of(*target_id)) {
            Some(w) => w,
            None => {
                eprintln!("  SKIP {}: no PCK params", body_name);
                continue;
            }
        };
        let mut granules: Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)> = Vec::new();
        let mut rotations: Vec<(f64, [f64; 9])> = Vec::new();
        let mut nutation: Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)> = Vec::new();
        let granule_days = if *target_id >= 2000000 {
            ASTEROID_GRANULE_DAYS
        } else {
            GRANULE_DAYS
        };
        for spk in &spk_files {
            let has_coverage = spk.segments().iter().any(|s| s.target == *target_id);
            if !has_coverage {
                continue;
            }
            let (g, r, n) = extract_granules(
                spk,
                &spk_files,
                *target_id,
                &wgccre,
                &bpc_files,
                &fk,
                granule_days,
            );
            granules.extend(g);
            rotations.extend(r);
            nutation.extend(n);
        }
        if granules.is_empty() && *target_id != 10 {
            eprintln!("  SKIP {}: no granules in any kernel", body_name);
            continue;
        }
        granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        rotations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        nutation.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let path = format!("ephemeris_{}.bin", body_name);
        let og = match &omega_g {
            Some((n, v, s)) if n == body_name => Some((*v, *s)),
            _ => None,
        };
        if write_binary(
            &path, body_name, &granules, &rotations, &nutation, &wgccre, og,
        ) {
            written.push(body_name.clone());
            if ci_mode && !upload_asset(&path) {
                upload_failed += 1;
            }
        }
    }
    if ci_mode && upload_failed > 0 {
        eprintln!(
            "upload: {} of {} assets did not reach the CDN",
            upload_failed,
            written.len()
        );
        std::process::exit(1);
    }
    written
}

fn summarize(index_path: &str, out_path: &str) {
    let entries = load_index(index_path);
    let mut family_files: BTreeMap<String, (usize, u64, u64)> = BTreeMap::new();
    for e in &entries {
        let slot = family_files.entry(e.family.clone()).or_insert((0, 0, 0));
        slot.0 += 1;
        slot.1 += e.size;
        if e.mtime > slot.2 {
            slot.2 = e.mtime;
        }
    }
    let mut rows = String::new();
    let mut total_bytes = 0u64;
    for (family, (count, bytes, newest)) in &family_files {
        total_bytes += bytes;
        rows.push_str(&format!(
            "| {} | {} | {} | {}\n",
            family, count, bytes, newest
        ));
    }
    let mut systems = String::new();
    for sys in [
        "planets",
        "asteroids",
        "jupiter",
        "saturn",
        "mars",
        "uranus",
        "neptune",
        "pluto",
    ] {
        let sel = select_system(&entries, sys);
        let mut spk_name = "—";
        let mut pck_name = "—";
        for e in &sel {
            match e.family.as_str() {
                "spk-planets" | "spk-satellites" | "spk" => spk_name = &e.name,
                "pck-text" => pck_name = &e.name,
                _ => {}
            }
        }
        systems.push_str(&format!("| {} | {} | {} |\n", sys, spk_name, pck_name));
    }
    let doc = format!(
        "# KERNEL_INDEX — ω-flattener source inventory\n\n\
         Generated by `ephemeris_compiler --summarize` from `phi/sources_index.φ`.\n\
         Canonical is `sources_index.φ` (machine-readable); this document is the reading copy.\n\n\
         ## Roots\n\
         - https://ssd.jpl.nasa.gov/ftp/ (JPL SSD — planet/satellite/small-body SPKs, PCKs, DASTCOM)\n\
         - https://naif.jpl.nasa.gov/pub/ (NAIF SPICE — generic_kernels + mission trees)\n\n\
         HTTPS only. Fully recursive. CK/IK/SCLK/EK/DBK are indexed but not loaded\n\
         by the flattener (no cameras, no onboard time — NAIF PDF assessment).\n\n\
         ## Family inventory ({} files, {} B)\n\
         | Family | Files | Bytes | Newest mtime (unix) |\n|---|---|---|---|\n{}\n\
         ## System resolution (flattener selection)\n\
         | System | SPK | PCK |\n|---|---|---|\n{}\n\
         Selection rule: first digit sequence in the name = version, highest wins; on\n\
         ties the ssd.jpl.nasa.gov root wins, then the light variant\n\
         (`l`/`s` suffix), then the shortest name. Planets: highest DE base, on\n\
         ties full precision (not the `s` short variant); `_part-N` files of one\n\
         base are loaded completely. pck0001* files are loaded ascending\n\
         (00011 overrides 00010, NAIF precedence).\n\n\
         ## Moon PCK finding (session 2026-08-14)\n\
         The moon text PCKs (ssd.jpl.nasa.gov/ftp/misc/pck/\n\
         pck.sat441/pck.jup365/pck.mar099/pck.ura182/pck.plu060, each .tpc) carry\n\
         RADII + POLE for the moons but NO J2/J4 values — the only hits are\n\
         comment lines (BODYnnn_JCOEF documentation). Moon harmonics from text PCKs\n\
         therefore manifest 0 (no fabrication). Real zonal terms live in\n\
         the binary PCKs (moon_pa_de440, satellite .bpc) — that is K02. pck00010.tpc\n\
         covers Phobos/Triton/Charon with POLE+RADII; the GM of all bodies comes from\n\
         gm_Horizons.pck (km³/s², parser scales ×1e9).\n\n\
         ## Force coverage (9-channel matrix)\n\
         | Channel | What the index carries | Rest |\n|---|---|---|\n\
         | gravity | SPK (orbits), PCK (GM/radii/J2/J4/poles), DSK (shape, later), DASTCOM (small bodies) | CDDIS (Auth, I03) |\n\
         | em | DASTCOM albedo; Tycho-2 (K04) does not live on these roots | HEASARC/IRSA/MAST (TAP, sources repo) |\n\
         | acoustic | — | GONG/SOHO, NOAA (curation) |\n\
         | seismic-body | — | USGS/IRIS (live), PDS InSight/Apollo (Auth/API) |\n\
         | seismic-surface | — | USGS (live) |\n\
         | thermal | DASTCOM H/albedo | GOES thermal (curation) |\n\
         | diffusion | — | SWPC/OMNI (live) |\n\
         | advective | — | DSCOVR/SWPC (live) |\n\
         | electric | — | SWPC/OmniWeb/Swarm (curation) |\n\n\
          ## Flatten policy\n\
          K01: planets + moons (SPK/PCK) + probes (Horizons compiler) + the asteroid\n\
          longbow (`asteroids` = sb441-n16.bsp + de441.bsp for the sun's SSB state,\n\
          256-day raster, heliocentric segment + SSB chain) + the sb441-n373 split\n\
          (spk_split streams the 15.2-GB file in one sequential pass — summary chain\n\
          walk, pointer monotonicity gate, per-body DAF-in-RAM with a uniform address\n\
          shift — and compiles the TNOs with sources.φ blocks: eris 2136199,\n\
          haumea 2136108, makemake 2136472; GM from IOM Table 1 via\n\
          phi/pipeline/catalog/asteroid_gm_sb441.φ; roundtrip gate against the\n\
          stream, 400 epochs, ≤ 100 m). The remaining 370 n373 bodies stay named\n\
          pending (sources.φ blocks = SOURCE_PORT curation); the 12 bodies without\n\
          an SPK segment carry the Horizons 12-month windows; DASTCOM (K03) carries\n\
          their mass/radius.\n",
        entries.len(),
        total_bytes,
        rows,
        systems
    );
    match std::fs::write(out_path, doc) {
        Ok(()) => eprintln!(
            "summarize: {} files, {} B → {}",
            entries.len(),
            total_bytes,
            out_path
        ),
        Err(e) => eprintln!("write {}: {}", out_path, e),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: ephemeris_compiler <kernel_path>... [--gm <gm>] [--pck <pck>] [--bpc <bpc>] [--ci-mode]"
        );
        eprintln!(
            "       ephemeris_compiler --index --root <url> [--depth N] [--out <file>] [--delay-ms N] [--jobs N]"
        );
        eprintln!(
            "       ephemeris_compiler --summarize <index.φ> <docs/reference/KERNEL_INDEX.md>"
        );
        eprintln!(
            "       ephemeris_compiler --fetch-from <index.φ> --systems a,b,... [--extras name,...] [--small-bodies] --dest <dir> [--ci-mode] [--index <index.φ>]"
        );
        eprintln!("output: ephemeris_<body>.bin in current directory");
        std::process::exit(1);
    }
    if args[1] == "--index" {
        let mut roots = Vec::new();
        let mut depth = 0usize;
        let mut out = "phi/sources_index.φ".to_string();
        let mut delay_ms = 300u64;
        let mut jobs = 1usize;
        let mut i = 2;
        while i < args.len() {
            match args[i].as_str() {
                "--root" => {
                    if let Some(r) = args.get(i + 1) {
                        roots.push(r.clone());
                    }
                    i += 1;
                }
                "--depth" => {
                    depth = args.get(i + 1).and_then(|d| d.parse().ok()).unwrap_or(0);
                    i += 1;
                }
                "--out" => {
                    if let Some(o) = args.get(i + 1) {
                        out = o.clone();
                    }
                    i += 1;
                }
                "--delay-ms" => {
                    delay_ms = args.get(i + 1).and_then(|d| d.parse().ok()).unwrap_or(300);
                    i += 1;
                }
                "--jobs" => {
                    jobs = args.get(i + 1).and_then(|d| d.parse().ok()).unwrap_or(1);
                    i += 1;
                }
                _ => {}
            }
            i += 1;
        }
        if roots.is_empty() {
            eprintln!("--index: no roots given");
            std::process::exit(1);
        }
        crawl(&roots, depth, &out, delay_ms, jobs);
        return;
    }
    if args[1] == "--summarize" {
        let index_path = match args.get(2) {
            Some(p) => p,
            None => {
                eprintln!("--summarize: index path absent");
                std::process::exit(1);
            }
        };
        let out_path = match args.get(3) {
            Some(p) => p,
            None => {
                eprintln!("--summarize: output path absent");
                std::process::exit(1);
            }
        };
        summarize(index_path, out_path);
        return;
    }
    let mut kernel_paths: Vec<String> = Vec::new();
    let mut gm_path: Option<String> = None;
    let mut pck_paths: Vec<String> = Vec::new();
    let mut bpc_paths: Vec<String> = Vec::new();
    let mut fk_paths: Vec<String> = Vec::new();
    let mut probe_jd: Option<f64> = None;
    let mut ci_mode = false;
    let mut index_path: Option<String> = None;
    let mut fetch_from: Option<String> = None;
    let mut systems: Vec<String> = Vec::new();
    let mut extras: Vec<String> =
        vec!["gm_Horizons.pck".to_string(), "geophysical.ker".to_string()];
    let mut dest = "kernels".to_string();
    let mut omega_g_path: Option<String> = None;
    let mut small_bodies_only = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ci-mode" => ci_mode = true,
            "--small-bodies" => small_bodies_only = true,
            "--gm" => {
                gm_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--pck" => {
                if let Some(p) = args.get(i + 1) {
                    pck_paths.push(p.clone());
                }
                i += 1;
            }
            "--bpc" => {
                if let Some(p) = args.get(i + 1) {
                    bpc_paths.push(p.clone());
                }
                i += 1;
            }
            "--fk" => {
                if let Some(p) = args.get(i + 1) {
                    fk_paths.push(p.clone());
                }
                i += 1;
            }
            "--probe" => {
                probe_jd = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--index" => {
                index_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--fetch-from" => {
                fetch_from = args.get(i + 1).cloned();
                i += 1;
            }
            "--systems" => {
                if let Some(s) = args.get(i + 1) {
                    systems = s.split(',').map(|x| x.trim().to_string()).collect();
                }
                i += 1;
            }
            "--extras" => {
                if let Some(s) = args.get(i + 1) {
                    extras = s.split(',').map(|x| x.trim().to_string()).collect();
                }
                i += 1;
            }
            "--dest" => {
                if let Some(d) = args.get(i + 1) {
                    dest = d.clone();
                }
                i += 1;
            }
            "--omega-g" => {
                omega_g_path = args.get(i + 1).cloned();
                i += 1;
            }
            other => kernel_paths.push(other.to_string()),
        }
        i += 1;
    }
    let omega_g = match &omega_g_path {
        Some(p) => match std::fs::read_to_string(p) {
            Ok(text) => {
                let mut parts = text.lines().next().unwrap_or("").split_whitespace();
                let name = parts.next().unwrap_or("").to_string();
                let nhz = parts.next().and_then(|v| v.parse::<f64>().ok());
                let sigma_nhz = parts.next().and_then(|v| v.parse::<f64>().ok());
                match (name.is_empty(), nhz, sigma_nhz) {
                    (false, Some(v), Some(s)) if v > 0.0 && s > 0.0 => {
                        Some((name, v * 1e-9, s * 1e-9))
                    }
                    _ => {
                        eprintln!(
                            "--omega-g {}: the line carries no body/value/sigma triple (nHz)",
                            p
                        );
                        None
                    }
                }
            }
            Err(_) => {
                eprintln!("--omega-g {}: read returned void", p);
                None
            }
        },
        None => None,
    };
    if let Some(jd) = probe_jd {
        let mut fk = FkFile::parse("");
        for fk_path in &fk_paths {
            match FkFile::open(fk_path) {
                Ok(f) => fk.insert_file(f),
                Err(e) => eprintln!("open {}: {}", fk_path, e),
            }
        }
        for f in &fk.frames {
            eprintln!(
                "fk frame {} {} class {:?} center {:?} tk={}",
                f.id,
                f.name,
                f.class,
                f.center,
                f.tk.as_ref()
                    .and_then(|t| t.relative.clone())
                    .unwrap_or_default()
            );
        }
        for bpc_path in &bpc_paths {
            let b = match BpcFile::open(bpc_path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("open {}: {}", bpc_path, e);
                    continue;
                }
            };
            for frame in fk.frames.iter().filter(|f| f.class == Some(2)) {
                let et = (jd - J2000_EPOCH) * 86400.0;
                let Some((phi, theta, psi)) = b.orient(frame.id, 1, et) else {
                    continue;
                };
                eprintln!(
                    "probe {} ({}) jd {}: phi={:.6} deg theta={:.9} rad psi={:.9} rad",
                    frame.name, frame.id, jd, phi, theta, psi
                );
                let Some(child) = fk.tkframe_child_of(&frame.name) else {
                    continue;
                };
                let Some((rot, via)) = fk.tkframe_rotation(child.id) else {
                    continue;
                };
                let m_me = matmul(&rot, &libration_matrix(phi, theta, psi));
                let (ra2, dec2, w2) = iau_angles_from_matrix(m_me);
                eprintln!(
                    "probe {} ({}) via {}: me=({:.6}, {:.6}, {:.6})",
                    child.name, child.id, via, ra2, dec2, w2
                );
                let et2 = (jd + 10.0 - J2000_EPOCH) * 86400.0;
                if let Some((phi2, theta2, psi2)) = b.orient(frame.id, 1, et2) {
                    let m_me2 = matmul(&rot, &libration_matrix(phi2, theta2, psi2));
                    let (_, _, w3) = iau_angles_from_matrix(m_me2);
                    let mut drift = w3 - w2;
                    while drift > 180.0 {
                        drift -= 360.0;
                    }
                    while drift < -180.0 {
                        drift += 360.0;
                    }
                    eprintln!("probe: me-W drift over 10 d = {:.6} deg/d", drift / 10.0);
                }
            }
        }
        return;
    }
    if let Some(index_file) = &fetch_from {
        if index_path.is_none() {
            index_path = Some(index_file.clone());
        }
        let entries = load_index(index_file);
        let mut selected = Vec::new();
        for sys in &systems {
            selected.extend(select_system(&entries, sys));
        }
        for name in &extras {
            if let Some(e) = entries.iter().find(|e| &e.name == name) {
                selected.push(e.clone());
            }
        }
        if selected.is_empty() {
            eprintln!("--fetch-from: selection empty for systems {:?}", systems);
            std::process::exit(1);
        }
        for e in &selected {
            eprintln!("selected: {} ({}, {} B)", e.name, e.family, e.size);
        }
        let paths = download_missing(&selected, &dest);
        let (kernels, bpcs, gm_text, pck_texts, fks) = classify(&paths);
        let pck_merged: String = pck_texts.concat();
        let pck = if pck_merged.is_empty() {
            None
        } else {
            Some(pck_merged)
        };
        let written = flatten(
            &kernels,
            &bpcs,
            &fks,
            gm_text.as_deref(),
            pck.as_deref(),
            ci_mode,
            omega_g.clone(),
            small_bodies_only,
        );
        if ci_mode {
            if let Some(idx) = &index_path {
                update_index_bodies(idx, &written);
            }
        }
        return;
    }
    if kernel_paths.is_empty() {
        eprintln!("no kernel paths given");
        std::process::exit(1);
    }
    let gm_text = gm_path.and_then(|p| std::fs::read_to_string(p).ok());
    let mut pck_merged = String::new();
    for p in &pck_paths {
        if let Ok(t) = std::fs::read_to_string(p) {
            pck_merged.push_str(&t);
            pck_merged.push('\n');
        }
    }
    let pck_text = if pck_merged.is_empty() {
        None
    } else {
        Some(pck_merged)
    };
    flatten(
        &kernel_paths,
        &bpc_paths,
        &fk_paths,
        gm_text.as_deref(),
        pck_text.as_deref(),
        ci_mode,
        omega_g,
        small_bodies_only,
    );
}
