use omegaflow::archivar::fetch::{RetryPolicy, fetch_raw_to_file_with};
use omegaflow::archivar::md5::Md5;
use omegaflow::archivar::sha256::{Sha256, sha256_hex};
use omegaflow::cdn::upload_release;
use std::io::{BufReader, Read, Write};

const CDN_DOWNLOAD_MARKER: &str = "/releases/download/";

const ORIGIN_TRANSFER_BOUND_S: u64 = 3 * 3600;

const STREAM_CHUNK_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone)]
enum Gate {
    Sha256(String),
    Md5(String),
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

fn is_md5_hex(value: &str) -> bool {
    value.len() == 32 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

fn parse_gate(args: &[String]) -> Result<Gate, String> {
    let sha = arg_value(args, "--sha256");
    let md5 = arg_value(args, "--md5");
    match (sha, md5) {
        (Some(_), Some(_)) => {
            Err("--sha256 and --md5 given together — one fingerprint only".to_string())
        }
        (None, None) => {
            Err("--sha256 or --md5 absent — an unverified body is never mirrored".to_string())
        }
        (Some(s), None) => {
            let s = s.trim().to_ascii_lowercase();
            if !is_sha256_hex(&s) {
                return Err(format!(
                    "--sha256 {s}: not a 64-hex sha256 — the raw asset stays unmirrored"
                ));
            }
            Ok(Gate::Sha256(s))
        }
        (None, Some(m)) => {
            let m = m.trim().to_ascii_lowercase();
            if !is_md5_hex(&m) {
                return Err(format!(
                    "--md5 {m}: not a 32-hex md5 — the raw asset stays unmirrored"
                ));
            }
            Ok(Gate::Md5(m))
        }
    }
}

fn cdn_release_path(url: &str) -> Option<(String, String)> {
    let rest = url.split_once(CDN_DOWNLOAD_MARKER)?.1;
    let (tag, name) = rest.split_once('/')?;
    if tag.is_empty() || name.is_empty() || name.contains('/') {
        return None;
    }
    Some((tag.to_string(), name.to_string()))
}

fn verify(gate: &Gate, sha256_got: &str, md5_got: &str, origin: &str) -> Result<(), String> {
    match gate {
        Gate::Sha256(registered) => {
            if sha256_got != registered {
                return Err(format!(
                    "{origin}: delivered sha256 {sha256_got} instead of {registered} — no upload (the hash is the gate)"
                ));
            }
        }
        Gate::Md5(registered) => {
            if md5_got != registered {
                return Err(format!(
                    "{origin}: delivered md5 {md5_got} instead of {registered} — no upload (the hash is the gate)"
                ));
            }
        }
    }
    Ok(())
}

fn split_streaming(
    temp: &std::path::Path,
    tag: &str,
    name: &str,
    origin: &str,
    gate: &Gate,
    ci_mode: bool,
) -> Result<(), String> {
    let dir = std::path::Path::new("data").join(tag);
    std::fs::create_dir_all(&dir).map_err(|e| format!("create {} void: {e}", dir.display()))?;

    let file = std::fs::File::open(temp).map_err(|e| format!("open {}: {e}", temp.display()))?;
    let mut reader = BufReader::with_capacity(STREAM_CHUNK_BYTES, file);
    let mut whole_sha = Sha256::new();
    let mut whole_md5 = Md5::new();
    let mut parts: Vec<(String, String, u64)> = Vec::new();
    let mut part_idx = 0usize;
    let mut part: Option<(String, std::fs::File, Sha256, u64)> = None;
    let mut buf = vec![0u8; STREAM_CHUNK_BYTES];
    let mut total: u64 = 0;
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| format!("read {}: {e}", temp.display()))?;
        if n == 0 {
            break;
        }
        total += n as u64;
        let mut slice = &buf[..n];
        whole_sha.update(slice);
        whole_md5.update(slice);
        while !slice.is_empty() {
            if part.is_none() {
                let pname = omegaflow::cdn::part_name(name, part_idx);
                let ppath = dir.join(&pname);
                let f = std::fs::File::create(&ppath)
                    .map_err(|e| format!("create {} void: {e}", ppath.display()))?;
                part = Some((pname, f, Sha256::new(), 0));
            }
            let (_, f, hasher, len) = part.as_mut().expect("a part is open");
            let room = omegaflow::cdn::CDN_PART_MAX_BYTES - *len;
            let take = room.min(slice.len() as u64) as usize;
            f.write_all(&slice[..take])
                .map_err(|e| format!("write part void: {e}"))?;
            hasher.update(&slice[..take]);
            *len += take as u64;
            slice = &slice[take..];
            if *len == omegaflow::cdn::CDN_PART_MAX_BYTES {
                let (pname, f, hasher, len) = part.take().expect("a part is open");
                f.sync_all().map_err(|e| format!("sync part void: {e}"))?;
                drop(f);
                parts.push((
                    pname,
                    omegaflow::archivar::sha256::hex(&hasher.finalize()),
                    len,
                ));
                part_idx += 1;
            }
        }
    }
    if let Some((pname, f, hasher, len)) = part.take() {
        f.sync_all().map_err(|e| format!("sync part void: {e}"))?;
        drop(f);
        if len > 0 {
            parts.push((
                pname,
                omegaflow::archivar::sha256::hex(&hasher.finalize()),
                len,
            ));
        }
    }

    let whole_sha_hex = omegaflow::archivar::sha256::hex(&whole_sha.finalize());
    let whole_md5_hex = omegaflow::archivar::md5::hex(&whole_md5.finalize());
    verify(gate, &whole_sha_hex, &whole_md5_hex, origin)?;

    let manifest = omegaflow::cdn::format_parts_manifest(&whole_sha_hex, &parts);
    let mpath = dir.join(omegaflow::cdn::parts_manifest_name(name));
    std::fs::write(&mpath, manifest)
        .map_err(|e| format!("{}: write void ({e})", mpath.display()))?;

    if ci_mode {
        for (pname, _, _) in &parts {
            let ppath = dir.join(pname);
            let pstr = ppath.to_str().ok_or_else(|| {
                format!(
                    "{}: path is not utf-8 — the part upload stays unraised",
                    ppath.display()
                )
            })?;
            if !upload_release(tag, pstr) {
                return Err(format!("{tag}/{pname}: the CDN part upload returned void"));
            }
        }
        let mstr = mpath.to_str().ok_or_else(|| {
            format!(
                "{}: path is not utf-8 — the manifest upload stays unraised",
                mpath.display()
            )
        })?;
        if !upload_release(tag, mstr) {
            return Err(format!(
                "{tag}/{name}.parts: the CDN manifest upload returned void"
            ));
        }
    }

    eprintln!(
        "{tag}/{name}: {total} B over the GitHub asset limit — split into {} part(s) + manifest (whole sha256 {whole_sha_hex})",
        parts.len()
    );
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = arg_value(args, "--url").ok_or_else(|| {
        "--url <cdn release url> absent — the raw asset stays unmirrored".to_string()
    })?;
    let origin = arg_value(args, "--origin").ok_or_else(|| {
        "--origin <source url> absent — the raw asset stays unmirrored".to_string()
    })?;
    let gate = parse_gate(args)?;

    let (tag, name) = cdn_release_path(&url).ok_or_else(|| {
        format!(
            "{url}: carries no /releases/download/<tag>/<asset> — the release path stays unread"
        )
    })?;

    let temp = std::env::temp_dir().join(format!("raw_mirror-body-{}", std::process::id()));
    let fetched = fetch_raw_to_file_with(&origin, RetryPolicy::All, ORIGIN_TRANSFER_BOUND_S, &temp);
    let size = match fetched {
        Some(size) if size > 0 => size,
        _ => {
            let _ = std::fs::remove_file(&temp);
            return Err(format!(
                "{origin}: live body void — {tag}/{name} stays untouched"
            ));
        }
    };

    if size > omegaflow::cdn::GITHUB_RELEASE_ASSET_MAX_BYTES {
        let outcome = split_streaming(&temp, &tag, &name, &origin, &gate, ci_mode);
        let _ = std::fs::remove_file(&temp);
        return outcome;
    }

    let body = std::fs::read(&temp).ok();
    let _ = std::fs::remove_file(&temp);
    let body = body
        .filter(|b| !b.is_empty())
        .ok_or_else(|| format!("{origin}: live body void — {tag}/{name} stays untouched"))?;

    let got = sha256_hex(&body);
    match &gate {
        Gate::Sha256(_) => verify(&gate, &got, "", &origin)?,
        Gate::Md5(_) => {
            let got_md5 = omegaflow::archivar::md5::md5_hex(&body);
            verify(&gate, "", &got_md5, &origin)?;
        }
    }

    let out = std::path::Path::new("data").join(&tag).join(&name);
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create {} void: {e}", parent.display()))?;
    }
    std::fs::write(&out, &body).map_err(|e| format!("{}: write void ({e})", out.display()))?;

    if ci_mode {
        let out_str = out.to_str().ok_or_else(|| {
            format!(
                "{}: path is not utf-8 — the upload stays unraised",
                out.display()
            )
        })?;
        if !upload_release(&tag, out_str) {
            return Err(format!("{tag}/{name}: the CDN upload returned void"));
        }
    }

    let gate_note = match &gate {
        Gate::Sha256(_) => format!("sha256 {got}"),
        Gate::Md5(md5) => format!("md5 {md5}"),
    };
    eprintln!("{tag}/{name}: verified {gate_note}, {} B", body.len());
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: raw_mirror --url <cdn release url> --origin <source url> (--sha256 <hex> | --md5 <hex>) [--ci-mode]"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("raw_mirror: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_path_splits_tag_and_asset() {
        assert_eq!(
            cdn_release_path(
                "https://github.com/omegaflow/sources/releases/download/ser.im-ldi.com/mariner_m10_camera_angles.txt"
            ),
            Some((
                "ser.im-ldi.com".to_string(),
                "mariner_m10_camera_angles.txt".to_string()
            ))
        );
        assert_eq!(
            cdn_release_path(
                "https://github.com/omegaflow/sources/releases/download/zenodo.org/jades_early_galaxies.tar"
            ),
            Some((
                "zenodo.org".to_string(),
                "jades_early_galaxies.tar".to_string()
            ))
        );
    }

    #[test]
    fn a_non_release_url_carries_no_path() {
        assert_eq!(
            cdn_release_path("https://ser.im-ldi.com/M10/camera_angles.txt"),
            None
        );
        assert_eq!(
            cdn_release_path("https://github.com/omegaflow/sources/releases/download/tag"),
            None
        );
        assert_eq!(
            cdn_release_path("https://github.com/omegaflow/sources/releases/download/tag/a/b"),
            None
        );
    }

    #[test]
    fn sha256_hex_measures_the_stride() {
        assert!(is_sha256_hex(
            "66892fad7075c91ac2ea8bc4e129119b0cd55f796835c990fec88e840f554ae5"
        ));
        assert!(!is_sha256_hex("66892fad"));
        assert!(!is_sha256_hex(
            "66892fad7075c91ac2ea8bc4e129119b0cd55f796835c990fec88e840f554aez"
        ));
    }

    #[test]
    fn the_gate_takes_exactly_one_fingerprint() {
        assert!(matches!(
            parse_gate(&["--sha256".to_string(), "a".repeat(64)]),
            Ok(Gate::Sha256(_))
        ));
        assert!(matches!(
            parse_gate(&[
                "--md5".to_string(),
                "9d7ccd0d12e402b82a8390ea3837ab1f".to_string()
            ]),
            Ok(Gate::Md5(_))
        ));
        assert!(parse_gate(&[]).is_err());
        assert!(
            parse_gate(&[
                "--sha256".to_string(),
                "a".repeat(64),
                "--md5".to_string(),
                "b".repeat(32)
            ])
            .is_err()
        );
        assert!(parse_gate(&["--md5".to_string(), "not-hex".to_string()]).is_err());
    }
}
