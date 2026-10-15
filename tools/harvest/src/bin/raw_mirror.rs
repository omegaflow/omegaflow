use omegaflow::archivar::fetch::{RetryPolicy, fetch_raw_to_file_with};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const CDN_DOWNLOAD_MARKER: &str = "/releases/download/";

const ORIGIN_TRANSFER_BOUND_S: u64 = 3 * 3600;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

fn cdn_release_path(url: &str) -> Option<(String, String)> {
    let rest = url.split_once(CDN_DOWNLOAD_MARKER)?.1;
    let (tag, name) = rest.split_once('/')?;
    if tag.is_empty() || name.is_empty() || name.contains('/') {
        return None;
    }
    Some((tag.to_string(), name.to_string()))
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = arg_value(args, "--url").ok_or_else(|| {
        "--url <cdn release url> absent — the raw asset stays unmirrored".to_string()
    })?;
    let origin = arg_value(args, "--origin").ok_or_else(|| {
        "--origin <source url> absent — the raw asset stays unmirrored".to_string()
    })?;
    let registered = arg_value(args, "--sha256").ok_or_else(|| {
        "--sha256 <hex> absent — an unverified body is never mirrored".to_string()
    })?;
    let registered = registered.trim().to_ascii_lowercase();
    if !is_sha256_hex(&registered) {
        return Err(format!(
            "--sha256 {registered}: not a 64-hex sha256 — the raw asset stays unmirrored"
        ));
    }

    let (tag, name) = cdn_release_path(&url).ok_or_else(|| {
        format!(
            "{url}: carries no /releases/download/<tag>/<asset> — the release path stays unread"
        )
    })?;

    let temp = std::env::temp_dir().join(format!("raw_mirror-body-{}", std::process::id()));
    let fetched = fetch_raw_to_file_with(&origin, RetryPolicy::All, ORIGIN_TRANSFER_BOUND_S, &temp);
    let body = match fetched {
        Some(_) => std::fs::read(&temp).ok(),
        None => None,
    };
    let _ = std::fs::remove_file(&temp);
    let body = body
        .filter(|b| !b.is_empty())
        .ok_or_else(|| format!("{origin}: live body void — {tag}/{name} stays untouched"))?;

    let got = sha256_hex(&body);
    if got != registered {
        return Err(format!(
            "{origin}: delivered sha256 {got} instead of {registered} — no upload (the hash is the gate)"
        ));
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
        if body.len() as u64 > omegaflow::cdn::GITHUB_RELEASE_ASSET_MAX_BYTES {
            let part_max = omegaflow::cdn::CDN_PART_MAX_BYTES as usize;
            let mut parts: Vec<(String, String, u64)> = Vec::new();
            for (idx, chunk) in body.chunks(part_max).enumerate() {
                let pname = omegaflow::cdn::part_name(&name, idx);
                let ppath = std::path::Path::new("data").join(&tag).join(&pname);
                std::fs::write(&ppath, chunk)
                    .map_err(|e| format!("{}: write void ({e})", ppath.display()))?;
                let pstr = ppath.to_str().ok_or_else(|| {
                    format!(
                        "{}: path is not utf-8 — the part upload stays unraised",
                        ppath.display()
                    )
                })?;
                if !upload_release(&tag, pstr) {
                    return Err(format!("{tag}/{pname}: the CDN part upload returned void"));
                }
                parts.push((pname, sha256_hex(chunk), chunk.len() as u64));
            }
            let manifest = omegaflow::cdn::format_parts_manifest(&got, &parts);
            let mpath = std::path::Path::new("data")
                .join(&tag)
                .join(omegaflow::cdn::parts_manifest_name(&name));
            std::fs::write(&mpath, manifest)
                .map_err(|e| format!("{}: write void ({e})", mpath.display()))?;
            let mstr = mpath.to_str().ok_or_else(|| {
                format!(
                    "{}: path is not utf-8 — the manifest upload stays unraised",
                    mpath.display()
                )
            })?;
            if !upload_release(&tag, mstr) {
                return Err(format!(
                    "{tag}/{name}.parts: the CDN manifest upload returned void"
                ));
            }
            eprintln!(
                "{tag}/{name}: {} B over the GitHub asset limit — split into {} part(s) + manifest",
                body.len(),
                parts.len()
            );
        } else if !upload_release(&tag, out_str) {
            return Err(format!("{tag}/{name}: the CDN upload returned void"));
        }
    }

    eprintln!("{tag}/{name}: verified sha256 {got}, {} B", body.len());
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: raw_mirror --url <cdn release url> --origin <source url> --sha256 <hex> [--ci-mode]"
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
}
