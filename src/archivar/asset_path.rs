
use std::path::PathBuf;

pub fn work_root() -> PathBuf {
    if let Some(dir) = std::env::var_os("OMEGAFLOW_WORK").filter(|v| !v.is_empty()) {
        return PathBuf::from(dir);
    }
    if let Some(home) = std::env::var_os("HOME").filter(|v| !v.is_empty()) {
        return PathBuf::from(home).join("projects/omegaflow-work");
    }
    PathBuf::from("omegaflow-work")
}

pub fn raw(netloc: &str, name: &str) -> PathBuf {
    work_root().join("sources").join(netloc).join(name)
}

pub fn compiled(netloc: &str, name: &str) -> PathBuf {
    work_root()
        .join("sources")
        .join(netloc)
        .join("compiled")
        .join(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        work: Option<std::ffi::OsString>,
        home: Option<std::ffi::OsString>,
    }

    impl EnvGuard {
        fn take() -> Self {
            Self {
                work: std::env::var_os("OMEGAFLOW_WORK"),
                home: std::env::var_os("HOME"),
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.work {
                    Some(v) => std::env::set_var("OMEGAFLOW_WORK", v),
                    None => std::env::remove_var("OMEGAFLOW_WORK"),
                }
                match &self.home {
                    Some(v) => std::env::set_var("HOME", v),
                    None => std::env::remove_var("HOME"),
                }
            }
        }
    }

    fn probe_root(tag: &str) -> PathBuf {
        PathBuf::from(format!(
            "/tmp/opencode/asset_path_{tag}_{}",
            std::process::id()
        ))
    }

    #[test]
    fn raw_joins_the_source_tree() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::take();
        let root = probe_root("raw");
        unsafe {
            std::env::set_var("OMEGAFLOW_WORK", &root);
        }
        assert_eq!(
            raw("example.com", "sample.bin"),
            root.join("sources/example.com/sample.bin")
        );
    }

    #[test]
    fn compiled_joins_the_compiled_folder_without_creating_it() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::take();
        let root = probe_root("compiled");
        unsafe {
            std::env::set_var("OMEGAFLOW_WORK", &root);
        }
        let path = compiled("example.com", "sample.bin");
        assert_eq!(
            path,
            root.join("sources/example.com/compiled/sample.bin")
        );
        assert!(
            !path.parent().unwrap().exists(),
            "compiled() only names the path"
        );
    }

    #[test]
    fn work_root_prefers_the_env_and_falls_back_to_home() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::take();
        unsafe {
            std::env::set_var("OMEGAFLOW_WORK", "/tmp/opencode/asset_path_env");
            std::env::set_var("HOME", "/home/probe");
        }
        assert_eq!(
            work_root(),
            PathBuf::from("/tmp/opencode/asset_path_env"),
            "OMEGAFLOW_WORK wins"
        );

        unsafe {
            std::env::remove_var("OMEGAFLOW_WORK");
        }
        assert_eq!(
            work_root(),
            PathBuf::from("/home/probe/projects/omegaflow-work"),
            "without the env the home root is used"
        );
    }
}
