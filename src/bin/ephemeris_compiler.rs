use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::{body_url, upload_asset};
use omegaflow::pck::{neutral, PckBody};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::io::Write;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const CHEBYSHEV_DEGREE: usize = 17;
const GRANULE_DAYS: f64 = 32.0;
const N_SAMPLES: usize = 25;
const J2000_EPOCH: f64 = 2451545.0;
const MAGIC_HEADER: [u8; 4] = [0xCF, 0x86, 0x01, 0x00];
const NAIF_ID_TABLE: &str = include_str!("../../docs/reference/naif_body_ids.tsv");

struct BodyId {
    name: String,
    parent: Option<i32>,
}

fn body_table() -> HashMap<i32, BodyId> {
    let mut table = HashMap::new();
    for line in NAIF_ID_TABLE.lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let id: i32 = match parts.next().and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => continue,
        };
        let name = match parts.next() {
            Some(n) => n.to_string(),
            None => continue,
        };
        let parent = parts.next().and_then(|p| p.parse().ok());
        table.insert(id, BodyId { name, parent });
    }
    table
}

fn parent_of(body: i32) -> Option<i32> {
    body_table().get(&body).and_then(|b| b.parent)
}

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

fn pck_id_of(target: i32) -> i32 {
    match target {
        1 => 199,
        2 => 299,
        4 => 499,
        5 => 599,
        6 => 699,
        7 => 799,
        8 => 899,
        9 => 999,
        other => other,
    }
}

fn chebyshev_nodes(n: usize) -> Vec<f64> {
    let mut nodes = Vec::with_capacity(n);
    for k in 0..n {
        nodes.push(((std::f64::consts::PI * (n as f64 - k as f64 - 0.5)) / n as f64).cos());
    }
    nodes
}

fn chebyshev_polys(n: usize, x: f64) -> Vec<f64> {
    let mut t = vec![1.0, x];
    for k in 2..n {
        let next = 2.0 * x * t[k - 1] - t[k - 2];
        t.push(next);
    }
    t.truncate(n);
    t
}

fn chebyshev_fit(
    samples: &[(f64, f64, f64)],
    degree: usize,
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let m = samples.len();
    if m < degree + 1 {
        return None;
    }
    let nodes = chebyshev_nodes(m);
    let mut a = vec![vec![0.0; degree + 1]; m];
    for i in 0..m {
        let polys = chebyshev_polys(degree + 1, nodes[i]);
        for j in 0..=degree {
            a[i][j] = polys[j];
        }
    }
    let mut ata = vec![vec![0.0; degree + 1]; degree + 1];
    for i in 0..m {
        for j in 0..=degree {
            for k in 0..=degree {
                ata[j][k] += a[i][j] * a[i][k];
            }
        }
    }
    let mut atx = vec![0.0; degree + 1];
    let mut aty = vec![0.0; degree + 1];
    let mut atz = vec![0.0; degree + 1];
    for i in 0..m {
        for j in 0..=degree {
            atx[j] += a[i][j] * samples[i].0;
            aty[j] += a[i][j] * samples[i].1;
            atz[j] += a[i][j] * samples[i].2;
        }
    }
    let (cx, cy, cz) = match solve_normal_equations(&ata, &atx, &aty, &atz) {
        Some(c) => c,
        None => return None,
    };
    Some((cx, cy, cz))
}

fn solve_normal_equations(
    ata: &[Vec<f64>],
    atx: &[f64],
    aty: &[f64],
    atz: &[f64],
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let n = ata.len();
    let mut a = ata.to_vec();
    for i in 0..n {
        let mut pivot = i;
        for j in i + 1..n {
            if a[j][i].abs() > a[pivot][i].abs() {
                pivot = j;
            }
        }
        if a[pivot][i].abs() < 1e-15 {
            return None;
        }
        a.swap(i, pivot);
        for j in i + 1..n {
            let factor = a[j][i] / a[i][i];
            for k in i..n {
                a[j][k] -= factor * a[i][k];
            }
        }
    }
    let x = back_substitute(&a, atx);
    let y = back_substitute(&a, aty);
    let z = back_substitute(&a, atz);
    Some((x, y, z))
}

fn back_substitute(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = a.len();
    let mut x = b.to_vec();
    for i in (0..n).rev() {
        for j in i + 1..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }
    x
}

fn state_ssb(spk: &SpkFile, target: i32, et: f64) -> Option<[f64; 6]> {
    if let Ok(s) = spk.state(target, 0, et) {
        return Some(s);
    }
    let parent = parent_of(target)?;
    let moon_state = spk.state(target, parent, et).ok()?;
    let planet_state = state_ssb(spk, parent, et)?;
    Some([
        moon_state[0] + planet_state[0],
        moon_state[1] + planet_state[1],
        moon_state[2] + planet_state[2],
        moon_state[3] + planet_state[3],
        moon_state[4] + planet_state[4],
        moon_state[5] + planet_state[5],
    ])
}

fn compute_rotation_matrix(wgccre: &PckBody, jd: f64) -> Option<[f64; 9]> {
    let tc = (jd - J2000_EPOCH) / 36525.0;
    let a = wgccre.pole_ra_at(tc)?.to_radians();
    let d = wgccre.pole_dec_at(tc)?.to_radians();
    let w = (wgccre.pm_at(jd - J2000_EPOCH)? - wgccre.pole_ra_at(tc)?).to_radians();
    let (sa, ca) = a.sin_cos();
    let (sd, cd) = d.sin_cos();
    let (sw, cw) = w.sin_cos();
    let xt_target = cw * ca - sw * sa * cd;
    let yt_target = cw * sa + sw * ca * cd;
    let zt_target = sw * sd;
    let xt_up = -sw * ca - cw * sa * cd;
    let yt_up = -sw * sa + cw * ca * cd;
    let zt_up = cw * sd;
    let xt_east = -sa * sd;
    let yt_east = ca * sd;
    let zt_east = -cd;
    Some([
        xt_target, yt_target, zt_target, xt_east, yt_east, zt_east, xt_up, yt_up, zt_up,
    ])
}

fn extract_granules(
    spk: &SpkFile,
    target: i32,
    wgccre: &PckBody,
) -> (
    Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)>,
    Vec<(f64, [f64; 9])>,
) {
    let mut granules = Vec::new();
    let mut rotations = Vec::new();
    let segments = spk.segments();
    let relevant: Vec<_> = segments
        .iter()
        .filter(|s| s.target == target && s.data_type == 2)
        .collect();
    if relevant.is_empty() {
        return (granules, rotations);
    }
    let mut min_et = f64::MAX;
    let mut max_et = f64::MIN;
    for seg in &relevant {
        if seg.start_et < min_et {
            min_et = seg.start_et;
        }
        if seg.end_et > max_et {
            max_et = seg.end_et;
        }
    }
    let granule_half_sec = GRANULE_DAYS * 86400.0 / 2.0;
    let n_granules = ((max_et - min_et) / (GRANULE_DAYS * 86400.0)).ceil() as usize;
    for i in 0..n_granules {
        let mid_et = min_et + (i as f64 + 0.5) * GRANULE_DAYS * 86400.0;
        let mid_jd = mid_et / 86400.0 + J2000_EPOCH;
        let half_jd = GRANULE_DAYS / 2.0;
        let cheb_nodes = chebyshev_nodes(N_SAMPLES);
        let mut samples_x = Vec::with_capacity(N_SAMPLES);
        let mut samples_y = Vec::with_capacity(N_SAMPLES);
        let mut samples_z = Vec::with_capacity(N_SAMPLES);
        let mut valid = true;
        for tau in &cheb_nodes {
            let et = mid_et + tau * granule_half_sec;
            match state_ssb(spk, target, et) {
                Some([x, y, z, _, _, _]) => {
                    samples_x.push(x * 1000.0);
                    samples_y.push(y * 1000.0);
                    samples_z.push(z * 1000.0);
                }
                None => {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            continue;
        }
        let combined: Vec<(f64, f64, f64)> = (0..N_SAMPLES)
            .map(|k| (samples_x[k], samples_y[k], samples_z[k]))
            .collect();
        if let Some((cx, cy, cz)) = chebyshev_fit(&combined, CHEBYSHEV_DEGREE) {
            granules.push((mid_jd, half_jd, cx, cy, cz));
        }
        if let Some(rot_m) = compute_rotation_matrix(wgccre, mid_jd) {
            rotations.push((mid_jd, rot_m));
        }
    }
    (granules, rotations)
}

fn write_binary(
    path: &str,
    body_name: &str,
    granules: &[(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)],
    rotations: &[(f64, [f64; 9])],
    wgccre: &PckBody,
) -> bool {
    let mut n_sections: u32 = 2;
    if !rotations.is_empty() {
        n_sections += 1;
    }
    let mut buf = Vec::new();
    buf.extend_from_slice(&MAGIC_HEADER);
    buf.extend_from_slice(&n_sections.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&(granules.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(CHEBYSHEV_DEGREE as u32).to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for (t0, dt, cx, cy, cz) in granules {
        buf.extend_from_slice(&t0.to_le_bytes());
        buf.extend_from_slice(&dt.to_le_bytes());
        for &c in cx {
            buf.extend_from_slice(&c.to_le_bytes());
        }
        for &c in cy {
            buf.extend_from_slice(&c.to_le_bytes());
        }
        for &c in cz {
            buf.extend_from_slice(&c.to_le_bytes());
        }
    }
    {
        let section_stype: u32 = 1;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&12u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let params: [f64; 12] = [
            neutral(wgccre.pole_ra_deg),
            neutral(wgccre.pole_ra_rate_deg_per_century),
            neutral(wgccre.pole_dec_deg),
            neutral(wgccre.pole_dec_rate_deg_per_century),
            neutral(wgccre.pm_deg),
            neutral(wgccre.pm_rate_deg_per_day),
            neutral(wgccre.radii_m.map(|r| r[0])),
            neutral(wgccre.radii_m.map(|r| r[1])),
            neutral(wgccre.radii_m.map(|r| r[2])),
            neutral(wgccre.j2),
            neutral(wgccre.j4),
            neutral(wgccre.gm_m3_s2),
        ];
        for &p in &params {
            buf.extend_from_slice(&p.to_le_bytes());
        }
    }
    {
        let section_stype: u32 = 2;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let kernel_params: [f64; 5] = [0.0; 5];
        for &p in &kernel_params {
            buf.extend_from_slice(&p.to_le_bytes());
        }
    }
    if !rotations.is_empty() {
        let section_stype: u32 = 3;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&(rotations.len() as u32).to_le_bytes());
        buf.extend_from_slice(&8u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for (t0, mat) in rotations {
            buf.extend_from_slice(&t0.to_le_bytes());
            for &v in mat {
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
    match std::fs::write(path, &buf) {
        Ok(()) => {
            eprintln!(
                "  {}: {} granules, {} rotations, {} B",
                body_name,
                granules.len(),
                rotations.len(),
                buf.len()
            );
            true
        }
        Err(e) => {
            eprintln!("write {}: {}", path, e);
            false
        }
    }
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
    if name.ends_with(".bsp") || name.ends_with(".bpc") || name.ends_with(".bc") {
        if url.contains("/satellites/") {
            return "spk-satellites";
        }
        if url.contains("/planets/") {
            return "spk-planets";
        }
        return "spk";
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

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
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
    let end = (0..=cap)
        .rev()
        .find(|&i| after.is_char_boundary(i))
        .unwrap_or(0);
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
                mtime = days_from_civil(y, mo, d) as u64 * 86400;
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
    if let Some(best) = pick_spk(&spk) {
        let base = base_of(&best.name);
        for e in &spk {
            if base_of(&e.name) == base {
                out.push(e.clone());
            }
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
        let fresh = std::fs::metadata(&path)
            .map(|m| m.len() == e.size)
            .unwrap_or(false);
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
        }
        paths.push(path);
    }
    paths
}

fn classify(paths: &[String]) -> (Vec<String>, Option<String>, Vec<String>) {
    let mut kernels = Vec::new();
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
    (kernels, gm, vec![pck_text])
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
    gm_text: Option<&str>,
    pck_text: Option<&str>,
    ci_mode: bool,
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
    let pck_bodies: HashMap<i32, PckBody> = omegaflow::pck::parse(gm_text, pck_text);
    let targets = flatten_targets(&spk_files);
    let mut written = Vec::new();
    for (target_id, body_name, _) in &targets {
        let wgccre = match pck_bodies.get(&pck_id_of(*target_id)) {
            Some(w) => w,
            None => {
                eprintln!("  SKIP {}: no PCK params", body_name);
                continue;
            }
        };
        let mut granules: Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)> = Vec::new();
        let mut rotations: Vec<(f64, [f64; 9])> = Vec::new();
        for spk in &spk_files {
            let has_coverage = spk.segments().iter().any(|s| s.target == *target_id);
            if !has_coverage {
                continue;
            }
            let (g, r) = extract_granules(spk, *target_id, &wgccre);
            granules.extend(g);
            rotations.extend(r);
        }
        if granules.is_empty() && *target_id != 10 {
            eprintln!("  SKIP {}: no granules in any kernel", body_name);
            continue;
        }
        granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        rotations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let path = format!("ephemeris_{}.bin", body_name);
        if write_binary(&path, body_name, &granules, &rotations, &wgccre) {
            written.push(body_name.clone());
            if ci_mode {
                let _ = upload_asset(&path);
            }
        }
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
        "planets", "jupiter", "saturn", "mars", "uranus", "neptune", "pluto",
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
        "# KERNEL_INDEX — ω-Flattener-Quelleninventar\n\n\
         Generiert von `ephemeris_compiler --summarize` aus `phi/sources_index.φ`.\n\
         Kanonisch ist `sources_index.φ` (maschinenlesbar); dieses Dokument ist die Lesefassung.\n\n\
         ## Wurzeln\n\
         - https://ssd.jpl.nasa.gov/ftp/ (JPL SSD — Planeten-/Satelliten-/Kleinkörper-SPKs, PCKs, DASTCOM)\n\
         - https://naif.jpl.nasa.gov/pub/ (NAIF SPICE — generic_kernels + Missions-Trees)\n\n\
         Nur HTTPS. Voll rekursiv. CK/IK/SCLK/EK/DBK werden indexiert, aber vom Flattener\n\
         nicht geladen (keine Kameras, keine Bordzeit — NAIF-PDF-Bewertung).\n\n\
         ## Familienbestand ({} Dateien, {} B)\n\
         | Familie | Dateien | Bytes | Neueste mtime (unix) |\n|---|---|---|---|\n{}\n\
         ## System-Auflösung (Flattener-Auswahl)\n\
         | System | SPK | PCK |\n|---|---|---|\n{}\n\
         Auswahlregel: erste Ziffernfolge im Namen = Version, höchste gewinnt; bei\n\
         Gleichstand bevorzugt die ssd.jpl.nasa.gov-Wurzel, dann die Light-Variante\n\
         (`l`/`s`-Suffix), dann der kürzeste Name. Planeten: höchste DE-Basis, bei\n\
         Gleichstand volle Präzision (nicht die `s`-Kurzvariante); `_part-N`-Dateien\n\
         einer Basis werden vollständig geladen. pck0001*-Dateien werden aufsteigend\n\
         geladen (00011 überschreibt 00010, NAIF-Precedence).\n\n\
         ## Mond-PCK-Befund (Session 2026-08-14)\n\
         Die Mond-Text-PCKs (ssd.jpl.nasa.gov/ftp/misc/pck/\n\
         pck.sat441/pck.jup365/pck.mar099/pck.ura182/pck.plu060, je .tpc) tragen\n\
         RADII + POLE für die Monde, aber KEINE J2/J4-Werte — die einzigen Treffer sind\n\
         Kommentarzeilen (BODYnnn_JCOEF-Dokumentation). Mond-Harmonische aus Text-PCKs\n\
         manifestieren daher 0 (0 honored, keine Fabrikation). Echte Zonal-Terme liegen in\n\
         den Binary-PCKs (moon_pa_de440, Satelliten-.bpc) — das ist K02. pck00010.tpc\n\
         deckt Phobos/Triton/Charon mit POLE+RADII ab; GM aller Körper kommt aus\n\
         gm_Horizons.pck (km³/s², Parser skaliert ×1e9).\n\n\
         ## Kraft-Abdeckung (9-Kanal-Matrix)\n\
         | Kanal | Was der Index trägt | Rest |\n|---|---|---|\n\
         | gravity | SPK (Bahnen), PCK (GM/Radii/J2/J4/Pole), DSK (Form, später), DASTCOM (Kleinkörper) | CDDIS (Auth, I03) |\n\
         | em | DASTCOM-Albedo; Tycho-2 (K04) liegt nicht auf diesen Wurzeln | HEASARC/IRSA/MAST (TAP, sources-Repo) |\n\
         | acoustic | — | GONG/SOHO, NOAA (Kuration) |\n\
         | seismic-body | — | USGS/IRIS (live), PDS InSight/Apollo (Auth/API) |\n\
         | seismic-surface | — | USGS (live) |\n\
         | thermal | DASTCOM H/Albedo | GOES-Thermal (Kuration) |\n\
         | diffusion | — | SWPC/OMNI (live) |\n\
         | advective | — | DSCOVR/SWPC (live) |\n\
         | electric | — | SWPC/OmniWeb/Swarm (Kuration) |\n\n\
         ## Flatten-Policy\n\
         K01: Planeten + Monde (SPK/PCK) + Sonden (Horizons-Compiler). Kleinkörper sind im\n\
         Index registriert (Familie `spk`), der Flatten-Pass liegt am K03-Zweig (DASTCOM+Kepler).\n",
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
            "usage: ephemeris_compiler <kernel_path>... [--gm <gm>] [--pck <pck>] [--ci-mode]"
        );
        eprintln!(
            "       ephemeris_compiler --index --root <url> [--depth N] [--out <file>] [--delay-ms N] [--jobs N]"
        );
        eprintln!("       ephemeris_compiler --summarize <index.φ> <KERNEL_INDEX.md>");
        eprintln!(
            "       ephemeris_compiler --fetch-from <index.φ> --systems a,b,... [--extras name,...] --dest <dir> [--ci-mode] [--index <index.φ>]"
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
    let mut ci_mode = false;
    let mut index_path: Option<String> = None;
    let mut fetch_from: Option<String> = None;
    let mut systems: Vec<String> = Vec::new();
    let mut extras: Vec<String> =
        vec!["gm_Horizons.pck".to_string(), "geophysical.ker".to_string()];
    let mut dest = "kernels".to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ci-mode" => ci_mode = true,
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
            other => kernel_paths.push(other.to_string()),
        }
        i += 1;
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
        let (kernels, gm_text, pck_texts) = classify(&paths);
        let pck_merged: String = pck_texts.concat();
        let pck = if pck_merged.is_empty() {
            None
        } else {
            Some(pck_merged)
        };
        let written = flatten(&kernels, gm_text.as_deref(), pck.as_deref(), ci_mode);
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
        gm_text.as_deref(),
        pck_text.as_deref(),
        ci_mode,
    );
}
