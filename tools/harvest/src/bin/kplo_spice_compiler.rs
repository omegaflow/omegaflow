use std::collections::BTreeMap;
use std::process::Command;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::ephemeris::{
    GRANULE_DAYS, J2000_EPOCH, extract_granules, state_ssb_multi, write_binary,
};
use omegaflow::fk::FkFile;
use omegaflow::pck::PckBody;

const KPLO_NAME: &str = "kplo";
const CDN_TAG: &str = "www.kari.re.kr";
const DEFAULT_LISTING: &str =
    "https://www.kari.re.kr/kpds/published/KPLO/KPLO/PublicRelease/kernels/spice_kernels/spk/";
const DEFAULT_DM_BASE: &str = "https://www.kari.re.kr/kpds/search/dirviewer/download/KPLO/KPLO/PublicRelease/kernels/spice_kernels/spk/";
const PLANET_KERNEL: &str = "de421.bsp";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == name {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
            i += 1;
        }
        i += 1;
    }
    out
}

fn fetch_text(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--max-time")
        .arg("90")
        .arg("--retry")
        .arg("3")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn bsp_names(html: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut rest = html;
    while let Some(pos) = rest.find("href=\"") {
        rest = &rest[pos + 6..];
        let end = match rest.find('"') {
            Some(e) => e,
            None => break,
        };
        let href = &rest[..end];
        if let Some(name) = href.rsplit('/').next()
            && name.ends_with(".bsp")
            && !names.iter().any(|n| n == name)
        {
            names.push(name.to_string());
        }
        rest = &rest[end..];
    }
    names
}

fn download(base: &str, name: &str, dest: &str) -> Option<String> {
    let path = format!("{dest}/{name}");
    if std::fs::metadata(&path).map_or(false, |m| m.len() > 0) {
        eprintln!("fetch: {name} fresh");
        return Some(path);
    }
    if std::fs::create_dir_all(dest).is_err() {
        return None;
    }
    let url = format!("{base}{name}");
    eprintln!("fetch: {url}");
    let status = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("5400")
        .arg("-o")
        .arg(&path)
        .arg(&url)
        .status();
    match status {
        Ok(s) if s.success() => {}
        _ => {
            let _ = std::fs::remove_file(&path);
            return None;
        }
    }
    if !std::fs::metadata(&path).map_or(false, |m| m.len() > 0) {
        let _ = std::fs::remove_file(&path);
        return None;
    }
    Some(path)
}

fn kplo_target(kernels: &[SpkFile]) -> Option<i32> {
    let mut counts: BTreeMap<i32, usize> = BTreeMap::new();
    for spk in kernels {
        for seg in spk.segments() {
            if seg.target < 0 {
                *counts.entry(seg.target).or_insert(0) += 1;
            }
        }
    }
    counts.into_iter().max_by_key(|(_, c)| *c).map(|(t, _)| t)
}

fn compile(kernel_paths: &[String], out: &str, ci_mode: bool) -> Result<(), String> {
    let mut kernels: Vec<SpkFile> = Vec::new();
    for p in kernel_paths {
        match SpkFile::open(p) {
            Ok(s) => kernels.push(s),
            Err(e) => return Err(format!("open {p}: {e}")),
        }
    }
    let target = kplo_target(&kernels)
        .ok_or("no negative NAIF target in the loaded kernels — the anchor stays unwritten")?;
    eprintln!("kplo target: {target}");
    let absent_pck = PckBody::absent();
    let mut granules = Vec::new();
    for spk in &kernels {
        if !spk.segments().iter().any(|s| s.target == target) {
            continue;
        }
        let (g, _, _) = extract_granules(
            spk,
            &kernels,
            target,
            &absent_pck,
            &[],
            &FkFile::parse(""),
            GRANULE_DAYS,
        );
        granules.extend(g);
    }
    if granules.is_empty() {
        return Err("no granules fit — the anchor stays unwritten".into());
    }
    granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    eprintln!("granules: {} ({GRANULE_DAYS} days wide)", granules.len());
    if !write_binary(out, KPLO_NAME, &granules, &[], &[], &absent_pck, None) {
        return Err(format!("{out}: write returned void"));
    }
    let bytes = std::fs::read(out).map_err(|e| format!("read {out}: {e}"))?;
    let eph = omegaflow::archivar::motion::parse_ephemeris_binary(&bytes)
        .ok_or_else(|| format!("{out}: parse_ephemeris_binary reads void"))?;
    let mut map = std::collections::HashMap::new();
    map.insert(KPLO_NAME.to_string(), eph);
    let mut max_delta_m = 0.0f64;
    let mut probes = 0usize;
    for (t0, half, _, _, _) in &granules {
        for tau in [-0.7f64, 0.0, 0.7] {
            let et = (t0 + tau * half - J2000_EPOCH) * 86400.0;
            let Some(src) = state_ssb_multi(&kernels, target, et) else {
                continue;
            };
            let Some(p) =
                omegaflow::archivar::motion::body_barycenter_position(KPLO_NAME, et, &map)
            else {
                continue;
            };
            let d = ((p[0] - src[0] * 1000.0).powi(2)
                + (p[1] - src[1] * 1000.0).powi(2)
                + (p[2] - src[2] * 1000.0).powi(2))
            .sqrt();
            if d > max_delta_m {
                max_delta_m = d;
            }
            probes += 1;
        }
    }
    eprintln!("{out}: {} B, sha256 {}", bytes.len(), sha256_hex(&bytes));
    eprintln!("roundtrip: {probes} probes, max granule-vs-source delta {max_delta_m:.1} m");
    eprintln!("CDN asset: {}", body_url(KPLO_NAME));
    if ci_mode && !upload_release(CDN_TAG, out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let dest = match arg_value(args, "--dest") {
        Some(v) => v,
        None => "kplo_kernels".to_string(),
    };
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("ephemeris_{KPLO_NAME}.bin"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let listing = match arg_value(args, "--listing") {
        Some(v) => v,
        None => DEFAULT_LISTING.to_string(),
    };
    let dm_base = match arg_value(args, "--dm-base") {
        Some(v) => v,
        None => DEFAULT_DM_BASE.to_string(),
    };

    let given = arg_values(args, "--kernel");
    let mut kernel_paths: Vec<String> = Vec::new();
    if !given.is_empty() {
        kernel_paths.extend(given);
    } else {
        let html =
            fetch_text(&listing).ok_or_else(|| format!("listing returned void: {listing}"))?;
        let names = bsp_names(&html);
        let wanted: Vec<String> = names
            .iter()
            .filter(|n| n.starts_with("kplo_dm_") || n.as_str() == PLANET_KERNEL)
            .cloned()
            .collect();
        if !wanted.iter().any(|n| n == PLANET_KERNEL) {
            return Err(format!(
                "{PLANET_KERNEL} absent from the listing — no SSB carrier"
            ));
        }
        if !wanted.iter().any(|n| n.starts_with("kplo_dm_")) {
            return Err("no kplo_dm_*.bsp in the listing — the trajectory has no carrier".into());
        }
        for name in &wanted {
            let path = download(&dm_base, name, &dest)
                .ok_or_else(|| format!("fetch: {name} returned void"))?;
            kernel_paths.push(path);
        }
    }
    compile(&kernel_paths, &out, ci_mode)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("kplo_spice_compiler: {msg}");
        std::process::exit(1);
    }
}
