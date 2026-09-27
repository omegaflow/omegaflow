use std::collections::HashSet;
use std::process::Command;
use std::sync::{Mutex, OnceLock};

pub const EPHEMERIS_TAG: &str = "ssd.jpl.nasa.gov-ephemeris";
pub const CAPPED_RELEASE: &str = "ssd.jpl.nasa.gov";
pub const CDN_REPO: &str = "omegaflow/sources";
pub const CDN_BASE: &str = "https://github.com/omegaflow/sources/releases/download";
pub const PS1_SLAB_BANDS: u32 = 80;
pub const MODIS_LST_CMG_FAMILY: &str = "data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg";

pub fn ps1_slab_tag(proj: u32) -> String {
    format!("ps1-dr2-{}", (proj / PS1_SLAB_BANDS) * PS1_SLAB_BANDS)
}

pub fn modis_lst_cmg_tag_of(name: &str) -> Option<String> {
    let rest = name.strip_prefix("modis_lst_cmg_")?;
    let (product, tail) = rest.split_once('_')?;
    if product.is_empty() {
        return None;
    }
    let year = modis_lst_cmg_year(tail)?;
    Some(format!("{MODIS_LST_CMG_FAMILY}-{product}-{year}"))
}

fn modis_lst_cmg_year(tail: &str) -> Option<&str> {
    if let Some(y) = tail.strip_suffix(".manifest") {
        return ascii_year(y);
    }
    let granule = tail.strip_suffix(".bin")?;
    let a = granule.find(".A")?;
    ascii_year(granule.get(a + 2..)?)
}

fn ascii_year(s: &str) -> Option<&str> {
    let y = s.get(0..4)?;
    if y.bytes().all(|b| b.is_ascii_digit()) {
        Some(y)
    } else {
        None
    }
}

pub fn cdn_base() -> String {
    if let Ok(base) = std::env::var("OMEGAFLOW_CDN_BASE") {
        return base;
    }
    CDN_BASE.to_string()
}

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

pub fn upload_release(tag: &str, path: &str) -> bool {
    if tag == CAPPED_RELEASE {
        eprintln!(
            "upload {}: release {} is capped (1000 assets) — upload to the family tag \"<host>-<family>\" instead",
            path, tag
        );
        return false;
    }
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
    format!("{}/{}/ephemeris_{}.bin", CDN_BASE, EPHEMERIS_TAG, name)
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
            let re = Command::new("gh")
                .arg("release")
                .arg("view")
                .arg(tag)
                .arg("--repo")
                .arg(CDN_REPO)
                .output();
            if re.map(|r| r.status.success()).unwrap_or(false) {
                mark_release_verified(tag);
                return true;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modis_lst_cmg_tag_parses_the_granule_shard() {
        assert_eq!(
            modis_lst_cmg_tag_of("modis_lst_cmg_8day_MOD11C2.A2000049.061.2020330085614.bin")
                .as_deref(),
            Some("data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-8day-2000")
        );
        assert_eq!(
            modis_lst_cmg_tag_of("modis_lst_cmg_monthly_MOD11C3.A2026348.061.2026182060603.bin")
                .as_deref(),
            Some("data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-monthly-2026")
        );
    }

    #[test]
    fn modis_lst_cmg_tag_parses_the_year_manifest() {
        assert_eq!(
            modis_lst_cmg_tag_of("modis_lst_cmg_8day_2000.manifest").as_deref(),
            Some("data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-8day-2000")
        );
        assert_eq!(
            modis_lst_cmg_tag_of("modis_lst_cmg_monthly_2026.manifest").as_deref(),
            Some("data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-monthly-2026")
        );
    }

    #[test]
    fn modis_lst_cmg_tag_returns_void_for_series_manifests_and_foreign_names() {
        assert_eq!(modis_lst_cmg_tag_of("modis_lst_cmg_8day.manifest"), None);
        assert_eq!(modis_lst_cmg_tag_of("modis_lst_cmg_monthly.manifest"), None);
        assert_eq!(modis_lst_cmg_tag_of("modis_lst_cmg_8day_garbage.bin"), None);
        assert_eq!(modis_lst_cmg_tag_of("modis_lst_cmg__2000.manifest"), None);
        assert_eq!(modis_lst_cmg_tag_of("ps1-dr2-0"), None);
        assert_eq!(modis_lst_cmg_tag_of(""), None);
    }
}
