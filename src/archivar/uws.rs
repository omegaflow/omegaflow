use super::*;

pub const UWS_CONNECT_BOUND_S: u64 = 1 << 5;
pub const UWS_SUBMIT_BOUND_S: u64 = 1 << 6;
pub const UWS_PHASE_BOUND_S: u64 = 1 << 5;
pub const UWS_RESULT_BOUND_S: u64 = 3600;
pub const UWS_POLL_STEP_S: u64 = 10;
pub const UWS_POLL_BUDGET_S: u64 = 1800;

fn uws_curl() -> Command {
    let mut cmd = Command::new("curl");
    cmd.arg("-s").arg("-S").arg("-g");
    super::fetch::append_ca(&mut cmd);
    cmd
}

pub fn uws_location(raw_headers: &str) -> Option<String> {
    raw_headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("location:"))
        .map(|l| l["location:".len()..].trim().to_string())
        .filter(|l| !l.is_empty())
}

pub fn uws_resolve(async_base: &str, location: &str) -> Option<String> {
    if location.starts_with("http://") || location.starts_with("https://") {
        return Some(location.to_string());
    }
    let (scheme, after) = async_base.split_once("://")?;
    let host = after.split('/').next()?;
    if location.starts_with('/') {
        return Some(format!("{scheme}://{host}{location}"));
    }
    let dir = after.rsplit_once('/').map(|(d, _)| d).unwrap_or(after);
    Some(format!("{scheme}://{host}{dir}/{location}"))
}

pub fn uws_submit(
    async_base: &str,
    params: &[(&str, &str)],
    headers: &[(String, String)],
) -> Option<String> {
    let mut cmd = uws_curl();
    cmd.arg("-f")
        .arg("-D")
        .arg("-")
        .arg("-o")
        .arg("/dev/null")
        .arg("-m")
        .arg(UWS_SUBMIT_BOUND_S.to_string())
        .arg("--connect-timeout")
        .arg(UWS_CONNECT_BOUND_S.to_string())
        .arg("-X")
        .arg("POST");
    for (k, v) in params {
        cmd.arg("--data-urlencode").arg(format!("{k}={v}"));
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    cmd.arg(async_base);
    let out = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("uws submit: curl spawn void at {async_base}: {e}");
            return None;
        }
    };
    if !out.status.success() {
        eprintln!(
            "uws submit http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    let raw = String::from_utf8_lossy(&out.stdout);
    let location = match uws_location(&raw) {
        Some(l) => l,
        None => {
            eprintln!(
                "uws submit: location header absent at {async_base} — the job stays uncreated"
            );
            return None;
        }
    };
    match uws_resolve(async_base, &location) {
        Some(job) => Some(job),
        None => {
            eprintln!("uws submit: location {location} unresolvable — the job stays unharvested");
            None
        }
    }
}

pub fn uws_phase(job: &str, headers: &[(String, String)]) -> Option<String> {
    let mut cmd = uws_curl();
    cmd.arg("-f")
        .arg("-m")
        .arg(UWS_PHASE_BOUND_S.to_string())
        .arg("--connect-timeout")
        .arg(UWS_CONNECT_BOUND_S.to_string());
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    cmd.arg(format!("{job}/phase"));
    let out = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("uws phase: curl spawn void at {job}: {e}");
            return None;
        }
    };
    if !out.status.success() {
        eprintln!(
            "uws phase http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    String::from_utf8(out.stdout)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn uws_post_phase(job: &str, new_phase: &str, headers: &[(String, String)]) -> bool {
    let mut cmd = uws_curl();
    cmd.arg("-f")
        .arg("-o")
        .arg("/dev/null")
        .arg("-m")
        .arg(UWS_SUBMIT_BOUND_S.to_string())
        .arg("--connect-timeout")
        .arg(UWS_CONNECT_BOUND_S.to_string())
        .arg("-X")
        .arg("POST")
        .arg("--data-urlencode")
        .arg(format!("PHASE={new_phase}"));
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    cmd.arg(format!("{job}/phase"));
    match cmd.output() {
        Ok(out) if out.status.success() => true,
        Ok(out) => {
            eprintln!(
                "uws phase post http {}: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("uws phase post: curl spawn void at {job}: {e}");
            false
        }
    }
}

pub fn uws_result_bytes(job: &str, headers: &[(String, String)]) -> Option<Vec<u8>> {
    let mut cmd = uws_curl();
    cmd.arg("-f")
        .arg("-m")
        .arg(UWS_RESULT_BOUND_S.to_string())
        .arg("--connect-timeout")
        .arg(UWS_CONNECT_BOUND_S.to_string());
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    cmd.arg(format!("{job}/results/result"));
    let out = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("uws result: curl spawn void at {job}: {e}");
            return None;
        }
    };
    if !out.status.success() {
        eprintln!(
            "uws result http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    Some(out.stdout)
}

pub fn uws_delete(job: &str, headers: &[(String, String)]) -> bool {
    let mut cmd = uws_curl();
    cmd.arg("-f")
        .arg("-o")
        .arg("/dev/null")
        .arg("-m")
        .arg(UWS_SUBMIT_BOUND_S.to_string())
        .arg("--connect-timeout")
        .arg(UWS_CONNECT_BOUND_S.to_string())
        .arg("-X")
        .arg("DELETE");
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    cmd.arg(job);
    match cmd.output() {
        Ok(out) if out.status.success() => true,
        Ok(out) => {
            eprintln!(
                "uws delete http {}: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("uws delete: curl spawn void at {job}: {e}");
            false
        }
    }
}

pub fn uws_poll(
    job: &str,
    step_s: u64,
    budget_s: u64,
    headers: &[(String, String)],
) -> Option<String> {
    let mut waited = 0u64;
    loop {
        if let Some(phase) = uws_phase(job, headers)
            && matches!(
                phase.as_str(),
                "COMPLETED" | "ERROR" | "ABORTED" | "ARCHIVED"
            )
        {
            return Some(phase);
        }
        let spent = waited + step_s;
        if spent > budget_s {
            let last = uws_phase(job, headers);
            let last_name = match last.as_deref() {
                Some(p) => p,
                None => "void",
            };
            eprintln!("uws poll: phase {last_name} after {spent} s — the job stays unharvested");
            return None;
        }
        std::thread::sleep(std::time::Duration::from_secs(step_s));
        waited = spent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uws_location_reads_the_location_header() {
        let headers = "HTTP/2 303\r\nserver: nginx\r\nlocation: https://irsa.ipac.caltech.edu/TAP/async/23572236\r\n";
        assert_eq!(
            uws_location(headers).as_deref(),
            Some("https://irsa.ipac.caltech.edu/TAP/async/23572236")
        );
        assert_eq!(
            uws_location("HTTP/2 200\r\ncontent-type: text/plain\r\n"),
            None
        );
    }

    #[test]
    fn uws_resolve_handles_absolute_root_relative_and_sibling_locations() {
        let base = "https://irsa.ipac.caltech.edu/TAP/async";
        assert_eq!(
            uws_resolve(base, "https://irsa.ipac.caltech.edu/TAP/async/42").as_deref(),
            Some("https://irsa.ipac.caltech.edu/TAP/async/42")
        );
        assert_eq!(
            uws_resolve(base, "/TAP/async/42").as_deref(),
            Some("https://irsa.ipac.caltech.edu/TAP/async/42")
        );
        assert_eq!(
            uws_resolve(base, "42").as_deref(),
            Some("https://irsa.ipac.caltech.edu/TAP/async/42")
        );
        assert_eq!(uws_resolve("no-scheme", "/x"), None);
    }
}
