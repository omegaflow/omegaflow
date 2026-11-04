use crate::filters;
use crate::groups;
use crate::json::{self, Json};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Profile {
    pub name: Option<String>,
    pub group: Option<String>,
    pub arms: Option<Vec<String>>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub kind: Option<String>,
    pub language: Option<String>,
    pub max: Option<usize>,
    pub stop: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Defaults {
    pub group: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub kind: Option<String>,
    pub language: Option<String>,
    pub max: Option<usize>,
    pub stop: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileError {
    pub subject: String,
    pub message: String,
}

impl ProfileError {
    fn new(subject: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.subject, self.message)
    }
}

pub fn merge(cli: Defaults, profile: &Profile) -> Defaults {
    Defaults {
        group: cli.group.or_else(|| profile.group.clone()),
        since: cli.since.or_else(|| profile.since.clone()),
        until: cli.until.or_else(|| profile.until.clone()),
        kind: cli.kind.or_else(|| profile.kind.clone()),
        language: cli.language.or_else(|| profile.language.clone()),
        max: cli.max.or(profile.max),
        stop: cli.stop.or(profile.stop),
    }
}

pub fn parse(text: &str) -> Result<Profile, String> {
    let value = json::parse(text).ok_or_else(|| "invalid JSON".to_string())?;
    let map = match value {
        Json::Obj(map) => map,
        _ => return Err("top level must be a JSON object".to_string()),
    };
    let name = text_field(&map, "name")?;
    let group = text_field(&map, "group")?;
    if let Some(group) = &group {
        if !groups::valid(group) {
            return Err(format!(
                "group {group} is not astro|literature|web|data|all"
            ));
        }
    }
    let arms = arms_field(&map)?;
    let since = match text_field(&map, "since")? {
        Some(raw) => Some(
            filters::parse_since(&raw)
                .ok_or_else(|| format!("since {raw} is not YYYY or YYYY-MM-DD"))?,
        ),
        None => None,
    };
    let until = match text_field(&map, "until")? {
        Some(raw) => Some(
            filters::parse_until(&raw)
                .ok_or_else(|| format!("until {raw} is not YYYY or YYYY-MM-DD"))?,
        ),
        None => None,
    };
    if let (Some(since), Some(until)) = (&since, &until) {
        if since > until {
            return Err(format!("since {since} lies after until {until}"));
        }
    }
    let kind = match text_field(&map, "type")? {
        Some(kind) => Some(kind.to_ascii_lowercase()),
        None => None,
    };
    let language = match text_field(&map, "language")? {
        Some(code) => {
            let code = code.to_ascii_lowercase();
            if !filters::valid_language(&code) {
                return Err(format!("language {code} is not an ISO 639-1 code"));
            }
            Some(code)
        }
        None => None,
    };
    let max = count_field(&map, "max")?;
    let stop = count_field(&map, "stop")?;
    Ok(Profile {
        name,
        group,
        arms,
        since,
        until,
        kind,
        language,
        max,
        stop,
    })
}

fn text_field(map: &HashMap<String, Json>, key: &str) -> Result<Option<String>, String> {
    match map.get(key) {
        None => Ok(None),
        Some(Json::Str(value)) if !value.trim().is_empty() => Ok(Some(value.trim().to_string())),
        Some(_) => Err(format!("{key} must be a non-empty string")),
    }
}

fn arms_field(map: &HashMap<String, Json>) -> Result<Option<Vec<String>>, String> {
    match map.get("arms") {
        None => Ok(None),
        Some(Json::Arr(items)) => {
            let mut arms = Vec::new();
            for item in items {
                match item {
                    Json::Str(arm) if !arm.trim().is_empty() => arms.push(arm.trim().to_string()),
                    _ => return Err("arms must be a list of non-empty strings".to_string()),
                }
            }
            if arms.is_empty() {
                return Err("arms must not be empty".to_string());
            }
            Ok(Some(arms))
        }
        Some(_) => Err("arms must be a list of non-empty strings".to_string()),
    }
}

fn count_field(map: &HashMap<String, Json>, key: &str) -> Result<Option<usize>, String> {
    match map.get(key) {
        None => Ok(None),
        Some(Json::Num(value)) if value.is_finite() && value.fract() == 0.0 && *value >= 1.0 => {
            Ok(Some(*value as usize))
        }
        Some(_) => Err(format!("{key} must be a positive integer")),
    }
}

pub fn load(arg: &str) -> Result<Profile, ProfileError> {
    let path = resolve(arg)?;
    load_path(&path)
}

pub fn load_path(path: &Path) -> Result<Profile, ProfileError> {
    let text = fs::read_to_string(path).map_err(|err| {
        ProfileError::new(
            path.display().to_string(),
            format!("cannot read profile: {err}"),
        )
    })?;
    parse(&text).map_err(|message| ProfileError::new(path.display().to_string(), message))
}

pub fn resolve(arg: &str) -> Result<PathBuf, ProfileError> {
    let direct = PathBuf::from(arg);
    if direct.is_file() {
        return Ok(direct);
    }
    if !arg.contains('/') {
        if let Some(dir) = profiles_dir() {
            let file = if arg.ends_with(".json") {
                arg.to_string()
            } else {
                format!("{arg}.json")
            };
            let candidate = dir.join(file);
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
    }
    if direct.is_absolute() || arg.contains('/') || arg.ends_with(".json") {
        Err(ProfileError::new(arg, "profile not found"))
    } else {
        Err(ProfileError::new(
            arg,
            "unknown profile (no such file in docs/specs/profiles)",
        ))
    }
}

pub fn list() -> Result<Vec<(String, Profile)>, ProfileError> {
    let dir = profiles_dir().ok_or_else(|| {
        ProfileError::new(
            "profiles",
            "cannot locate docs/specs/profiles (set OMEGAFLOW_REPO)",
        )
    })?;
    list_in(&dir)
}

pub fn list_in(dir: &Path) -> Result<Vec<(String, Profile)>, ProfileError> {
    let entries = fs::read_dir(dir).map_err(|err| {
        ProfileError::new(
            dir.display().to_string(),
            format!("cannot list profiles: {err}"),
        )
    })?;
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect();
    paths.sort();
    let mut profiles = Vec::new();
    for path in paths {
        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("profile")
            .to_string();
        profiles.push((name, load_path(&path)?));
    }
    Ok(profiles)
}

pub fn profiles_dir() -> Option<PathBuf> {
    if let Ok(root) = env::var("OMEGAFLOW_REPO") {
        let dir = PathBuf::from(root)
            .join("docs")
            .join("specs")
            .join("profiles");
        if dir.is_dir() {
            return Some(dir);
        }
    }
    if let Ok(cwd) = env::current_dir() {
        if let Some(dir) = find_profiles(&cwd) {
            return Some(dir);
        }
    }
    env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().and_then(find_profiles))
}

fn find_profiles(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        let candidate = dir.join("docs").join("specs").join("profiles");
        if candidate.is_dir() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut dir = env::temp_dir();
        dir.push(format!(
            "omegaflow-profile-{label}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn parses_every_profile_field() {
        let profile = parse(
            r#"{
                "name": "astro-solar",
                "group": "astro",
                "arms": ["ads", "simbad", "vizier"],
                "since": "2015",
                "until": "2024-12-31",
                "type": "Article",
                "language": "EN",
                "max": 30,
                "stop": 10
            }"#,
        )
        .unwrap();
        assert_eq!(profile.name.as_deref(), Some("astro-solar"));
        assert_eq!(profile.group.as_deref(), Some("astro"));
        assert_eq!(
            profile.arms,
            Some(vec![
                "ads".to_string(),
                "simbad".to_string(),
                "vizier".to_string()
            ])
        );
        assert_eq!(profile.since.as_deref(), Some("2015-01-01"));
        assert_eq!(profile.until.as_deref(), Some("2024-12-31"));
        assert_eq!(profile.kind.as_deref(), Some("article"));
        assert_eq!(profile.language.as_deref(), Some("en"));
        assert_eq!(profile.max, Some(30));
        assert_eq!(profile.stop, Some(10));
    }

    #[test]
    fn rejects_broken_json_and_bad_fields() {
        assert_eq!(parse("{").unwrap_err(), "invalid JSON");
        assert!(
            parse(r#"{"group": "space"}"#)
                .unwrap_err()
                .contains("group")
        );
        assert!(parse(r#"{"since": "soon"}"#).unwrap_err().contains("since"));
        assert!(parse(r#"{"arms": []}"#).unwrap_err().contains("arms"));
        assert!(parse(r#"{"max": 0}"#).unwrap_err().contains("max"));
        assert!(parse(r#"{"type": 7}"#).unwrap_err().contains("type"));
    }

    #[test]
    fn cli_values_override_profile_defaults() {
        let profile = parse(
            r#"{"group": "astro", "since": "2015", "type": "article", "max": 30, "stop": 10}"#,
        )
        .unwrap();
        let cli = Defaults {
            group: Some("web".to_string()),
            since: Some("2020-01-01".to_string()),
            kind: Some("dataset".to_string()),
            max: Some(7),
            stop: None,
            ..Defaults::default()
        };
        let merged = merge(cli, &profile);
        assert_eq!(merged.group.as_deref(), Some("web"));
        assert_eq!(merged.since.as_deref(), Some("2020-01-01"));
        assert_eq!(merged.kind.as_deref(), Some("dataset"));
        assert_eq!(merged.max, Some(7));
        assert_eq!(merged.stop, Some(10));
        assert_eq!(merged.until, None);
        assert_eq!(merged.language, None);
    }

    #[test]
    fn unknown_profile_names_the_argument() {
        let err = resolve("/nicht/da.json").unwrap_err();
        assert_eq!(err.subject, "/nicht/da.json");
        assert!(err.message.contains("not found"));
        let err = resolve("gibt-es-nicht").unwrap_err();
        assert_eq!(err.subject, "gibt-es-nicht");
        assert!(err.message.contains("unknown profile"));
    }

    #[test]
    fn list_reads_the_json_profiles_in_name_order() {
        let dir = temp_dir("list");
        fs::write(
            dir.join("b.json"),
            r#"{"name": "b", "group": "web", "max": 5}"#,
        )
        .unwrap();
        fs::write(dir.join("a.json"), r#"{"name": "a", "arms": ["ads"]}"#).unwrap();
        fs::write(dir.join("note.txt"), "not a profile").unwrap();
        let items = list_in(&dir).unwrap();
        let names: Vec<&str> = items.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, vec!["a", "b"]);
        assert_eq!(items[0].1.arms.as_ref().unwrap(), &vec!["ads".to_string()]);
        assert_eq!(items[1].1.max, Some(5));
        fs::remove_dir_all(&dir).unwrap();
    }
}
