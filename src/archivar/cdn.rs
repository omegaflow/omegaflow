use std::collections::HashSet;
use std::process::Command;
use std::sync::{Mutex, OnceLock};

pub const CDN_RELEASE: &str = "ssd.jpl.nasa.gov";
pub const CDN_REPO: &str = "omegaflow/sources";
pub const CDN_BASE: &str = "https://github.com/omegaflow/sources/releases/download";

static VERIFIED_RELEASES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn verified_releases() -> &'static Mutex<HashSet<String>> {
    VERIFIED_RELEASES.get_or_init(|| Mutex::new(HashSet::new()))
}

fn release_verified(tag: &str) -> bool {
    verified_releases()
        .lock()
        .map(|set| set.contains(tag))
        .unwrap_or(false)
}

fn mark_release_verified(tag: &str) {
    if let Ok(mut set) = verified_releases().lock() {
        set.insert(tag.to_string());
    }
}

pub fn upload_asset(path: &str) -> bool {
    upload_release(CDN_RELEASE, path)
}

pub fn upload_release(tag: &str, path: &str) -> bool {
    if std::env::var("GH_TOKEN").is_err() {
        eprintln!("upload {}: GH_TOKEN absent", path);
        return false;
    }
    if !ensure_release(tag) {
        eprintln!("upload {}: release {} not created", path, tag);
        return false;
    }
    let out = Command::new("gh")
        .arg("release")
        .arg("upload")
        .arg(tag)
        .arg(path)
        .arg("--clobber")
        .arg("--repo")
        .arg(CDN_REPO)
        .output();
    match out {
        Ok(o) if o.status.success() => true,
        Ok(o) => {
            eprintln!(
                "upload {}: gh returned void: {}",
                path,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("upload {}: gh absent: {}", path, e);
            false
        }
    }
}

pub fn body_url(name: &str) -> String {
    format!("{}/{}/ephemeris_{}.bin", CDN_BASE, CDN_RELEASE, name)
}

pub fn ensure_release(tag: &str) -> bool {
    if std::env::var("GH_TOKEN").is_err() {
        return false;
    }
    if release_verified(tag) {
        return true;
    }
    let view = Command::new("gh")
        .arg("release")
        .arg("view")
        .arg(tag)
        .arg("--repo")
        .arg(CDN_REPO)
        .output();
    if view.map(|o| o.status.success()).unwrap_or(false) {
        mark_release_verified(tag);
        return true;
    }
    let out = Command::new("gh")
        .arg("release")
        .arg("create")
        .arg(tag)
        .arg("--repo")
        .arg(CDN_REPO)
        .arg("--title")
        .arg(tag)
        .arg("--notes")
        .arg("reference dataset mirror")
        .output();
    match out {
        Ok(o) if o.status.success() => {
            mark_release_verified(tag);
            true
        }
        Ok(o) => {
            eprintln!(
                "ensure release {}: gh returned void: {}",
                tag,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("ensure release {}: gh absent: {}", tag, e);
            false
        }
    }
}
