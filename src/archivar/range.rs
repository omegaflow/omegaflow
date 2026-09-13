use super::*;

fn append_ca(cmd: &mut Command) {
    let path = match std::env::var("OMEGAFLOW_CA_BUNDLE") {
        Ok(p) if !p.is_empty() && std::path::Path::new(&p).is_file() => p,
        _ => return,
    };
    cmd.arg("--cacert").arg(path);
}

const RANGE_MAX_TIME_S: u64 = 1 << 7;
const RANGE_RETRY: u64 = 3;

fn offset_end(offset: u64, len: u64) -> Option<(u64, u64)> {
    if len == 0 {
        return None;
    }
    let end = offset.checked_add(len)?;
    Some((offset, end - 1))
}

pub fn fetch_range(
    url: &str,
    offset: u64,
    len: u64,
    headers: &[(String, String)],
) -> Option<Vec<u8>> {
    let (start, end) = offset_end(offset, len)?;
    let range = format!("{}-{}", start, end);
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg(RANGE_RETRY.to_string())
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(RANGE_MAX_TIME_S.to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string())
        .arg("-r")
        .arg(range);
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    append_ca(&mut cmd);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "\r\x1b[Krange returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_arithmetic_is_inclusive_end() {
        assert_eq!(offset_end(512, 16), Some((512, 527)));
        assert_eq!(offset_end(0, 1), Some((0, 0)));
        assert_eq!(offset_end(u64::MAX, 2), None);
        assert_eq!(offset_end(10, 0), None);
    }

    #[test]
    fn empty_range_is_absent() {
        assert!(fetch_range("https://example.com/absent", 0, 0, &[]).is_none());
    }
}
