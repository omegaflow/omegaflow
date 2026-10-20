
use omegaflow::archivar::inflate::tar_entries;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::tar_zst::{Member, encode};
use std::io::{BufReader, Read};
use std::process::{Child, ChildStdout, Command, Stdio};

const COMPILER: &str = "tools/harvest/src/bin/tar_zst_compiler.rs";
const DEFAULT_URL: &str =
    "https://zenodo.org/api/records/18776222/files/lp_dataset_patched_c000_phase_metadata.tar.zst/content";
const DEFAULT_OUT: &str = "tar_zst_members.bin";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

struct CurlStream {
    child: Child,
    out: ChildStdout,
}

impl Read for CurlStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.out.read(buf)
    }
}

impl Drop for CurlStream {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn curl_source(url: &str, range_bytes: Option<u64>) -> Option<Box<dyn Read>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSfL")
        .arg("--retry")
        .arg("3")
        .arg("--retry-delay")
        .arg("2")
        .arg("--connect-timeout")
        .arg("32")
        .arg("--speed-limit")
        .arg("1")
        .arg("--speed-time")
        .arg("128");
    if let Some(n) = range_bytes {
        if n == 0 {
            eprintln!("--range-bytes needs at least one byte");
            return None;
        }
        cmd.arg("-r").arg(format!("0-{}", n - 1));
    }
    let mut child = match cmd
        .arg(url)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("curl for {url} returned void: {e}");
            return None;
        }
    };
    let out = child.stdout.take()?;
    Some(Box::new(CurlStream { child, out }))
}

fn file_source(path: &str) -> Option<Box<dyn Read>> {
    match std::fs::File::open(path) {
        Ok(f) => Some(Box::new(BufReader::with_capacity(1 << 20, f))),
        Err(e) => {
            eprintln!("open {path} returned void: {e}");
            None
        }
    }
}

fn member_filter(args: &[String]) -> Option<Vec<String>> {
    arg_value(args, "--members").map(|v| {
        v.split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect()
    })
}

fn selected(name: &str, filter: Option<&[String]>) -> bool {
    match filter {
        Some(patterns) if !patterns.is_empty() => {
            patterns.iter().any(|p| name == p || name.contains(p))
        }
        _ => true,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--help") || has_flag(&args, "-h") {
        eprintln!(
            "usage: tar_zst_compiler (--url <url> | --file <path.tar.zst>) [--limit N] \
             [--members pat,pat] [--range-bytes N] [--out <manifest.bin>] [--stat]\n\
             compiler {COMPILER}"
        );
        return;
    }

    let (src, range) = match (
        arg_value(&args, "--file"),
        arg_value(&args, "--url"),
    ) {
        (Some(path), _) => (path, None),
        (None, url) => {
            let url = match url {
                Some(v) => v,
                None => DEFAULT_URL.to_string(),
            };
            let range = arg_value(&args, "--range-bytes").and_then(|v| v.parse::<u64>().ok());
            (url, range)
        }
    };
    let reader = match (arg_value(&args, "--file"), &range) {
        (Some(path), _) => file_source(&path),
        _ => curl_source(&src, range),
    };
    let Some(reader) = reader else {
        eprintln!("{src}: source stays unread — absent (0 honored)");
        std::process::exit(1);
    };
    let decoder = match zstd::stream::read::Decoder::new(reader) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{src}: zstd decoder returned void: {e}");
            std::process::exit(1);
        }
    };

    let limit: Option<usize> = arg_value(&args, "--limit").and_then(|v| v.parse().ok());
    let filter = member_filter(&args);
    if limit == Some(0) {
        eprintln!("{src}: --limit 0 leaves the manifest empty — absent (0 honored)");
        std::process::exit(1);
    }

    let mut members: Vec<Member> = Vec::new();
    let mut seen = 0u64;
    let mut stopped = false;
    let scan = tar_entries(decoder, |entry| {
        seen += 1;
        if selected(&entry.name, filter.as_deref()) {
            let member = Member {
                name: entry.name.clone(),
                size: entry.size,
                offset: entry.data_offset,
                mtime: entry.mtime,
                order: members.len() as u32,
                kind: entry.typeflag,
            };
            eprintln!(
                "{}\t{}\t{}\t{}\t{}",
                member.order,
                member.kind_name(),
                member.size,
                member.offset,
                member.name
            );
            members.push(member);
            if limit.is_some_and(|n| members.len() >= n) {
                stopped = true;
                return false;
            }
        }
        true
    });
    if let Err(e) = scan {
        if range.is_some() && !members.is_empty() {
            eprintln!("{src}: range ended after {} member(s): {e}", members.len());
        } else {
            eprintln!("{src}: tar stream returned void: {e}");
            std::process::exit(1);
        }
    }

    eprintln!(
        "{src}: {} member(s) selected of {} header(s) scanned{}",
        members.len(),
        seen,
        if stopped { " (limit reached, stream closed)" } else { "" }
    );
    if members.is_empty() {
        eprintln!("{src}: no member collected — absent (0 honored)");
        std::process::exit(1);
    }
    if has_flag(&args, "--stat") {
        let files = members.iter().filter(|m| m.is_file()).count();
        let dirs = members.iter().filter(|m| m.is_dir()).count();
        let bytes: u64 = members.iter().map(|m| m.size).sum();
        eprintln!(
            "{src}: {files} file(s), {dirs} dir(s), {} other, {bytes} member byte(s)",
            members.len() - files - dirs
        );
        return;
    }

    let Some(bin) = encode(&members) else {
        eprintln!("{src}: manifest encoding returned void — absent (0 honored)");
        std::process::exit(1);
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    if let Err(e) = std::fs::write(&out, &bin) {
        eprintln!("write {out} returned void: {e}");
        std::process::exit(1);
    }
    eprintln!(
        "{out}: {} B, sha256 {} (compiler {COMPILER})",
        bin.len(),
        sha256_hex(&bin)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::inflate::tar_entries;
    use omegaflow::archivar::tar_zst::parse;

    fn tar_header(name: &str, size: usize, kind: u8) -> [u8; 512] {
        let mut h = [0u8; 512];
        h[..name.len()].copy_from_slice(name.as_bytes());
        let octal = format!("{size:011o}");
        h[124..135].copy_from_slice(octal.as_bytes());
        h[156] = kind;
        h[257..262].copy_from_slice(b"ustar");
        h[148..156].fill(b' ');
        let sum: u32 = h.iter().map(|&b| b as u32).sum();
        let checksum = format!("{sum:06o}");
        h[148..154].copy_from_slice(checksum.as_bytes());
        h[154] = 0;
        h[155] = b' ';
        h
    }

    fn sample_tar() -> Vec<u8> {
        let mut tar = Vec::new();
        tar.extend_from_slice(&tar_header("lp_dataset_patched/", 0, b'5'));
        tar.extend_from_slice(&tar_header("lp_dataset_patched/06/ph012_lc2_172.npy", 5, b'0'));
        tar.extend_from_slice(b"abcde");
        tar.resize(tar.len().div_ceil(512) * 512, 0);
        tar.extend_from_slice(&[0u8; 1024]);
        tar
    }

    fn collect(tar: &[u8], limit: Option<usize>) -> (Vec<Member>, u64) {
        let mut out = Vec::new();
        let mut seen = 0u64;
        let offset = tar_entries(std::io::Cursor::new(tar), |e| {
            seen += 1;
            out.push(Member {
                name: e.name.clone(),
                size: e.size,
                offset: e.data_offset,
                mtime: e.mtime,
                order: out.len() as u32,
                kind: e.typeflag,
            });
            limit.is_none_or(|n| out.len() < n)
        })
        .expect("the tar scans");
        assert_eq!(seen as usize, out.len());
        (out, offset)
    }

    #[test]
    fn manifest_roundtrips_through_the_reader() {
        let tar = sample_tar();
        let (members, offset) = collect(&tar, None);
        assert_eq!(offset, 2048, "the scan stops at the first zero block");
        assert_eq!(members.len(), 2);
        assert!(members[0].is_dir());
        assert!(members[1].is_file());
        assert_eq!(members[1].size, 5);
        assert_eq!(members[1].offset, 1024);
        assert_eq!(members[1].name, "lp_dataset_patched/06/ph012_lc2_172.npy");
        let bin = encode(&members).expect("the manifest encodes");
        assert_eq!(parse(&bin).as_deref(), Some(members.as_slice()));
    }

    #[test]
    fn limit_stops_before_the_member_body() {
        let tar = sample_tar();
        let (members, offset) = collect(&tar, Some(1));
        assert_eq!(members.len(), 1);
        assert_eq!(offset, 512, "the scan stops after the first header");
    }

    #[test]
    fn member_selection_matches_exact_and_substring() {
        let filter = vec!["ph012".to_string()];
        assert!(selected("lp_dataset_patched/06/ph012_lc2_172.npy", Some(&filter)));
        assert!(!selected("lp_dataset_patched/06/ph020_lc1_199.npy", Some(&filter)));
        let dirs = vec!["lp_dataset_patched/06/".to_string()];
        assert!(selected("lp_dataset_patched/06/", Some(&dirs)));
        assert!(selected("anything", None));
    }
}
