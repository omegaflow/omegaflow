use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

const USAGE: &str = "usage: coverage_id_diff --origin <path> --mirror <path> [--out <path>] [--normalize basename] [--strip-prefix <s>]  (compares two --emit-ids sidecars, each line <id>\\t<epoch>\\t<href> with an absent field rendered as '-'; reports the id set relationship; --normalize basename compares the last path segment, suffix-stripped (.copc.laz/.laz/.las/.tif) and lowercased, alongside the raw counts; --strip-prefix <s> strips the given prefix from the normalized key when it starts with it (e.g. c_); --out writes the Markdown report there, otherwise stdout)";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_ids(text: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for line in text.lines() {
        let id = line.split('\t').next().unwrap_or("-");
        if id.is_empty() || id == "-" {
            continue;
        }
        ids.insert(id.to_string());
    }
    ids
}

fn missing_ids(origin: &BTreeSet<String>, mirror: &BTreeSet<String>) -> Vec<String> {
    origin.difference(mirror).cloned().collect()
}

const NORMALIZE_SUFFIXES: [&str; 4] = [".copc.laz", ".laz", ".las", ".tif"];

fn normalize_id(id: &str, strip_prefix: Option<&str>) -> String {
    let basename = id.rsplit('/').next().unwrap_or(id);
    let lower = basename.to_ascii_lowercase();
    let suffix_stripped = match NORMALIZE_SUFFIXES
        .iter()
        .find_map(|s| lower.strip_suffix(s))
    {
        Some(stripped) => stripped.to_string(),
        None => lower,
    };
    if let Some(prefix) = strip_prefix {
        let prefix_lower = prefix.to_ascii_lowercase();
        if let Some(stripped) = suffix_stripped.strip_prefix(prefix_lower.as_str()) {
            return stripped.to_string();
        }
    }
    suffix_stripped
}

fn normalize_set(ids: &BTreeSet<String>, strip_prefix: Option<&str>) -> BTreeSet<String> {
    ids.iter()
        .map(|id| normalize_id(id, strip_prefix))
        .collect()
}

fn report(
    origin: &BTreeSet<String>,
    mirror: &BTreeSet<String>,
    normalize: bool,
    strip_prefix: Option<&str>,
) -> String {
    let missing = missing_ids(origin, mirror);
    let mut out = String::new();
    out.push_str("# coverage id diff\n\n");
    out.push_str(&format!("- origin ids: {}\n", origin.len()));
    out.push_str(&format!("- mirror ids: {}\n", mirror.len()));
    out.push_str(&format!("- missing: {}\n\n", missing.len()));
    if missing.is_empty() {
        out.push_str(&format!("mirror ⊇ origin by id ({} ids)\n", origin.len()));
    } else {
        out.push_str(&format!(
            "mirror ⊉ origin: {} of {} origin ids missing\n",
            missing.len(),
            origin.len()
        ));
        out.push_str("\n## missing ids\n\n");
        for id in &missing {
            out.push_str(&format!("- {id}\n"));
        }
    }
    if normalize {
        let origin_norm = normalize_set(origin, strip_prefix);
        let mirror_norm = normalize_set(mirror, strip_prefix);
        let missing_norm = missing_ids(&origin_norm, &mirror_norm);
        out.push_str("\n## normalized (basename)\n\n");
        if let Some(prefix) = strip_prefix {
            out.push_str(&format!("- strip-prefix: {prefix}\n"));
        }
        out.push_str(&format!("- origin ids: {}\n", origin_norm.len()));
        out.push_str(&format!("- mirror ids: {}\n", mirror_norm.len()));
        out.push_str(&format!("- missing: {}\n\n", missing_norm.len()));
        if missing_norm.is_empty() {
            out.push_str(&format!(
                "mirror ⊇ origin by normalized id ({} ids)\n",
                origin_norm.len()
            ));
        } else {
            out.push_str(&format!(
                "mirror ⊉ origin: {} of {} normalized origin ids missing\n",
                missing_norm.len(),
                origin_norm.len()
            ));
            out.push_str("\n### missing normalized ids\n\n");
            for id in &missing_norm {
                out.push_str(&format!("- {id}\n"));
            }
        }
    }
    out
}

fn read_sidecar(path: &str) -> Result<BTreeSet<String>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    Ok(parse_ids(&text))
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!("{USAGE}");
        return Ok(());
    }
    let origin_path = match arg_value(args, "--origin").filter(|p| !p.is_empty()) {
        Some(p) => p,
        None => return Err("--origin absent — no origin sidecar named".to_string()),
    };
    let mirror_path = match arg_value(args, "--mirror").filter(|p| !p.is_empty()) {
        Some(p) => p,
        None => return Err("--mirror absent — no mirror sidecar named".to_string()),
    };
    let normalize = match args.iter().position(|a| a == "--normalize") {
        Some(i) => match args.get(i + 1).map(String::as_str) {
            Some("basename") => true,
            Some(other) => {
                return Err(format!(
                    "--normalize {other} unknown — only basename exists"
                ));
            }
            None => return Err("--normalize absent value — basename expected".to_string()),
        },
        None => false,
    };
    let strip_prefix = arg_value(args, "--strip-prefix").filter(|p| !p.is_empty());
    let origin = read_sidecar(&origin_path)?;
    let mirror = read_sidecar(&mirror_path)?;
    let text = report(&origin, &mirror, normalize, strip_prefix.as_deref());
    match arg_value(args, "--out").filter(|p| !p.is_empty()) {
        Some(path) => {
            if let Some(parent) = Path::new(&path).parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
                }
            }
            fs::write(&path, &text).map_err(|e| format!("write {path} returned void: {e}"))?;
            eprintln!("coverage_id_diff: report -> {path}");
        }
        None => print!("{text}"),
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("coverage_id_diff: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_ids_names_covered_absent_and_ignores_a_dash() {
        let origin = parse_ids("a\t-\thttps://example/a\nb\t2020\tX\nc\t-\tY\n");
        let mirror = parse_ids("a\t-\thttps://example/a\nb\t2020\tX\nd\t-\tZ\n-\t-\tW\n");
        assert_eq!(origin.len(), 3);
        assert_eq!(mirror.len(), 3);
        assert!(!mirror.contains("-"));
        assert_eq!(missing_ids(&origin, &mirror), vec!["c".to_string()]);
    }

    #[test]
    fn basename_normalizes_full_object_key_to_bare_tile() {
        assert_eq!(normalize_id("data/NL/x/01CZ1.copc.laz", None), "01cz1");
        assert_eq!(normalize_id("01CZ1", None), "01cz1");
        assert_eq!(normalize_id("tiles/A/02D2.LAZ", None), "02d2");
        assert_eq!(normalize_id("tiles/A/03E3.las", None), "03e3");
        assert_eq!(normalize_id("tiles/A/04F4.tif", None), "04f4");
        let origin = parse_ids("01CZ1\t-\thttps://example/a\n");
        let mirror = parse_ids("data/NL/x/01CZ1.copc.laz\t-\thttps://example/m\n");
        assert_eq!(missing_ids(&origin, &mirror), vec!["01CZ1".to_string()]);
        assert!(
            missing_ids(&normalize_set(&origin, None), &normalize_set(&mirror, None)).is_empty()
        );
    }

    #[test]
    fn strip_prefix_matches_c_prefixed_basename_to_bare_tile() {
        assert_eq!(
            normalize_id("data/NL/x/C_01CZ1.copc.laz", Some("c_")),
            "01cz1"
        );
        assert_eq!(
            normalize_id("data/NL/x/C_01CZ1.copc.laz", Some("C_")),
            "01cz1"
        );
        assert_eq!(normalize_id("tiles/A/02D2.LAZ", Some("c_")), "02d2");
        let origin = parse_ids("01CZ1\t-\thttps://example/a\n");
        let mirror = parse_ids("data/NL/x/C_01CZ1.copc.laz\t-\thttps://example/m\n");
        assert_eq!(
            missing_ids(
                &normalize_set(&origin, Some("c_")),
                &normalize_set(&mirror, Some("c_"))
            ),
            Vec::<String>::new()
        );
    }
}
