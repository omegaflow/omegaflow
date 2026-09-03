use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const WEBHOOK_HEALTH: &str = "https://mail.omegaflow.space/health";

fn main() {
    let tunnel = curl_code(WEBHOOK_HEALTH);
    let recv_active = service_active("smail-recv.service");
    let tunnel_active = service_active("smail-tunnel.service");
    let ledger_age = ledger_age_s();
    let line = report_line(epoch(), &tunnel, recv_active, tunnel_active, ledger_age);
    append_report(&state_dir().join("reports/mail_watchdog.φ"), &line);
    println!("{}", line.trim_end());
}

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from(".")
}

fn curl_code(url: &str) -> String {
    match Command::new("curl")
        .arg("-s")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}")
        .arg("--max-time")
        .arg("15")
        .arg(url)
        .output()
    {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => "void".to_string(),
    }
}

fn service_active(name: &str) -> bool {
    match Command::new("systemctl")
        .arg("--user")
        .arg("is-active")
        .arg(name)
        .output()
    {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim() == "active",
        Err(_) => false,
    }
}

fn ledger_age_s() -> Option<u64> {
    let text = std::fs::read_to_string(state_dir().join("mail/mail_ledger.φ")).ok()?;
    let last = text.lines().last()?;
    let ts = last.split('\t').nth(1)?.parse::<u64>().ok()?;
    let now = epoch();
    Some(now.saturating_sub(ts))
}

fn report_line(
    epoch: u64,
    tunnel: &str,
    recv_active: bool,
    tunnel_active: bool,
    ledger_age: Option<u64>,
) -> String {
    let age = match ledger_age {
        Some(a) => a.to_string(),
        None => "absent".to_string(),
    };
    format!(
        "mail_watchdog | {} | health={} recv={} tunnel={} ledger_age_s={}\n",
        epoch, tunnel, recv_active, tunnel_active, age
    )
}

fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn append_report<P: AsRef<std::path::Path>>(path: P, line: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        use std::io::Write;
        let _ = f.write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_line_has_five_tokens() {
        let l = report_line(100, "200", true, true, Some(42));
        let parts: Vec<&str> = l.trim().split(" | ").collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "mail_watchdog");
        assert!(parts[2].contains("health=200"));
        assert!(parts[2].contains("ledger_age_s=42"));
    }

    #[test]
    fn report_line_missing_ledger() {
        let l = report_line(100, "void", false, false, None);
        assert!(l.contains("ledger_age_s=absent"));
    }
}
