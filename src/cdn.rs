use std::process::Command;

pub const CDN_RELEASE: &str = "ssd.jpl.nasa.gov";
pub const CDN_REPO: &str = "omegaflow/sources";
pub const CDN_BASE: &str = "https://github.com/omegaflow/sources/releases/download";

pub fn upload_asset(path: &str) -> bool {
    if std::env::var("GH_TOKEN").is_err() {
        eprintln!("upload {}: GH_TOKEN absent", path);
        return false;
    }
    let out = Command::new("gh")
        .arg("release")
        .arg("upload")
        .arg(CDN_RELEASE)
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
