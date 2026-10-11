use omegaflow::cdn::upload_release;
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const TAG: &str = "data.fingrid.fi";
const DATASETS_URL: &str = "https://data.fingrid.fi/api/datasets";
const KEY_NAME: &str = "FINGRID_API_KEY";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn find_repo_root() -> Option<PathBuf> {
    if let Ok(env_root) = env::var("OMEGAFLOW_REPO") {
        if !env_root.is_empty() {
            let root = PathBuf::from(env_root);
            if root.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(root);
            }
            return None;
        }
    }
    let mut starts: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = env::current_dir() {
        starts.push(cwd);
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            starts.push(dir.to_path_buf());
        }
    }
    for start in starts {
        let mut dir = start;
        loop {
            if dir.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(dir);
            }
            match dir.parent() {
                Some(parent) => dir = parent.to_path_buf(),
                None => break,
            }
        }
    }
    None
}

fn read_secret(name: &str) -> Option<String> {
    let root = find_repo_root()?;
    let text = std::fs::read_to_string(root.join(".secrets.local")).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq) = line.find('=') {
            if line[..eq].trim() == name {
                let value = line[eq + 1..].trim().trim_matches(['"', '\'']);
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

fn fetch(url: &str, key: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .args([
            "-sS",
            "-L",
            "-m",
            "60",
            "--connect-timeout",
            "15",
            "-H",
            &format!("x-api-key: {key}"),
            url,
        ])
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!("curl {url} status {:?}", out.status.code()));
    }
    if out.stdout.is_empty() {
        return Err(format!("{url} carries no bytes"));
    }
    Ok(out.stdout)
}

fn dataset_count(bytes: &[u8]) -> usize {
    String::from_utf8_lossy(bytes).matches("\"id\":").count()
}

fn run(args: &[String]) -> Result<(), String> {
    let out_path =
        arg_value(args, "--out").ok_or("fingrid_compiler needs --out <path> (the CDN asset)")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let Some(key) = read_secret(KEY_NAME) else {
        return Err(format!("{KEY_NAME} absent from .secrets.local/.env"));
    };

    let bytes = fetch(DATASETS_URL, &key)?;
    let count = dataset_count(&bytes);

    if let Some(parent) = Path::new(&out_path).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|e| format!("create parent void: {e}"))?;
    }

    let mut file = std::fs::File::create(&out_path)
        .map_err(|e| format!("{out_path} create returned void: {e}"))?;
    file.write_all(&bytes)
        .map_err(|e| format!("{out_path} write returned void: {e}"))?;

    println!(
        "{out_path}: {} bytes, {count} dataset id(s) in the Fingrid ableList",
        bytes.len()
    );

    if ci_mode && !upload_release(TAG, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("fingrid_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dataset_count_reads_the_id_marker() {
        let body = br#"[{"id":244,"name":"a"},{"id":245,"name":"b"},{"id":246,"name":"c"}]"#;
        assert_eq!(dataset_count(body), 3);
    }

    #[test]
    fn an_empty_body_carries_no_dataset() {
        assert_eq!(dataset_count(b""), 0);
    }
}
