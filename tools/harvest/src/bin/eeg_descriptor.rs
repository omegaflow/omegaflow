use std::env;
use std::fs;
use std::process::Command;

const HEADER_LEN: usize = 100;
const MAGIC: &[u8; 4] = b"EEGB";
const FLAG_DOUBLE: u32 = 1;
const HEAD_BYTES: usize = 1 << 17;

fn gh_token() -> Result<String, String> {
    let out = Command::new("gh")
        .args(["auth", "token"])
        .output()
        .map_err(|e| format!("gh auth token returned void: {e}"))?;
    if !out.status.success() {
        return Err("gh auth token reported a failure".to_string());
    }
    let token = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if token.is_empty() {
        return Err("gh auth token carries nothing".to_string());
    }
    Ok(token)
}

pub fn asset_ref(url: &str) -> Result<(String, usize), String> {
    let rest = url
        .strip_prefix("https://github.com/")
        .ok_or_else(|| format!("{url} is not a github release address"))?;
    let parts: Vec<&str> = rest.split('/').collect();
    if parts.len() != 6 || parts[2] != "releases" || parts[3] != "download" {
        return Err(format!(
            "{url} is not of the form owner/repo/releases/download/tag/name"
        ));
    }
    let (owner, repo, tag, name) = (parts[0], parts[1], parts[4], parts[5]);
    let jq = format!(
        r#".assets[] | select(.name=="{}") | "\(.id) \(.size)""#,
        name
    );
    let out = Command::new("gh")
        .args([
            "api",
            &format!("repos/{owner}/{repo}/releases/tags/{tag}"),
            "--jq",
            &jq,
        ])
        .output()
        .map_err(|e| format!("gh api for {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!("{url}: the release answers {}", out.status));
    }
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let mut fields = text.split_whitespace();
    let (Some(id), Some(size)) = (fields.next(), fields.next()) else {
        return Err(format!("{url}: the release carries no such asset"));
    };
    let size = size
        .parse::<usize>()
        .map_err(|_| format!("{url}: the asset size {size} carries no number"))?;
    Ok((
        format!("https://api.github.com/repos/{owner}/{repo}/releases/assets/{id}"),
        size,
    ))
}

fn head_of(api: &str, token: &str, len: usize) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .args([
            "-sSfL",
            "--max-time",
            "120",
            "-H",
            &format!("Authorization: Bearer {token}"),
            "-H",
            "Accept: application/octet-stream",
            "-r",
            &format!("0-{}", len - 1),
            api,
        ])
        .output()
        .map_err(|e| format!("curl range for {api} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{api}: range 0-{} returned {}",
            len - 1,
            out.status
        ));
    }
    Ok(out.stdout)
}

fn u32_at(bytes: &[u8], at: usize) -> Result<u32, String> {
    bytes
        .get(at..at + 4)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or_else(|| format!("the head ends before byte {}", at + 4))
}

fn u64_at(bytes: &[u8], at: usize) -> Result<u64, String> {
    bytes
        .get(at..at + 8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or_else(|| format!("the head ends before byte {}", at + 8))
}

#[derive(Clone, Debug)]
pub struct Head {
    pub nbchan: u32,
    pub pnts: u64,
    pub trials: u32,
    pub srate: Option<f64>,
    pub labels: usize,
    pub samples_len: u64,
    pub samples_offset: usize,
    pub double: bool,
}

pub fn parse_head(bytes: &[u8]) -> Result<Head, String> {
    if bytes.len() < HEADER_LEN {
        return Err(format!(
            "the head carries {} bytes, the header needs {HEADER_LEN}",
            bytes.len()
        ));
    }
    if &bytes[0..4] != MAGIC {
        return Err("the file does not carry the EEGB signature".to_string());
    }
    let version = u32_at(bytes, 4)?;
    if version != 1 {
        return Err(format!("the header carries version {version}, not 1"));
    }
    let flags = u32_at(bytes, 8)?;
    let nbchan = u32_at(bytes, 12)?;
    let pnts = u64_at(bytes, 16)?;
    let trials = u32_at(bytes, 24)?;
    let srate = if bytes[28] != 0 {
        Some(f64::from_le_bytes(
            bytes[32..40]
                .try_into()
                .map_err(|_| "the head ends in the rate".to_string())?,
        ))
    } else {
        None
    };
    let tag_len = u32_at(bytes, 72)? as usize;
    let hexsha_len = u32_at(bytes, 76)? as usize;
    let url_len = u32_at(bytes, 80)? as usize;
    let labels_n = u32_at(bytes, 84)? as usize;
    let events_len = u32_at(bytes, 88)? as usize;
    let samples_len = u64_at(bytes, 92)?;

    if labels_n != nbchan as usize {
        return Err(format!(
            "the header names {labels_n} labels for {nbchan} channels"
        ));
    }
    let mut at = HEADER_LEN + tag_len + hexsha_len + url_len;
    for index in 0..labels_n {
        let len = bytes
            .get(at..at + 2)
            .and_then(|slice| slice.try_into().ok())
            .map(u16::from_le_bytes)
            .ok_or_else(|| format!("the head ends inside label {index} at byte {at}"))?
            as usize;
        at += 2 + len;
    }
    at += events_len;
    if at > bytes.len() {
        return Err(format!(
            "the head carries {} bytes, the samples start at {at}",
            bytes.len()
        ));
    }
    Ok(Head {
        nbchan,
        pnts,
        trials,
        srate,
        labels: labels_n,
        samples_len,
        samples_offset: at,
        double: flags & FLAG_DOUBLE != 0,
    })
}

#[derive(Clone, Debug)]
pub struct Report {
    pub head: Head,
    pub expected_total: usize,
    pub duration_s: Option<f64>,
}

pub fn describe(head_bytes: &[u8], total: usize) -> Result<Report, String> {
    let head = parse_head(head_bytes)?;
    let width: u64 = if head.double { 8 } else { 4 };
    let expected = head.nbchan as u64 * head.pnts * head.trials as u64 * width;
    if expected != head.samples_len {
        return Err(format!(
            "{} channels of {} points in {} trials at {width} bytes are {expected}, the header declares {} sample bytes",
            head.nbchan, head.pnts, head.trials, head.samples_len
        ));
    }
    let expected_total = head.samples_offset + head.samples_len as usize;
    if expected_total != total {
        let delta = total as i64 - expected_total as i64;
        return Err(format!(
            "{expected_total} bytes are declared, the file holds {total} ({delta:+})"
        ));
    }
    if head.pnts == 0 || head.trials == 0 {
        return Err("the header carries no samples".to_string());
    }
    let duration_s = match head.srate {
        Some(rate) if rate > 0.0 => Some(head.pnts as f64 / rate),
        Some(rate) => return Err(format!("the header carries a rate of {rate}")),
        None => None,
    };
    if let Some(duration) = duration_s
        && !(1e-3..=86400.0).contains(&duration)
    {
        let rate = match head.srate {
            Some(rate) => format!("{rate:.3} Hz"),
            None => "no rate".to_string(),
        };
        return Err(format!(
            "{} points at {rate} is a trial of {duration:.6} s",
            head.pnts
        ));
    }
    Ok(Report {
        head,
        expected_total,
        duration_s,
    })
}

fn entries(register: &str, format: &str) -> Result<Vec<(String, String)>, String> {
    let text =
        fs::read_to_string(register).map_err(|e| format!("read {register} returned void: {e}"))?;
    let mut blocks: Vec<std::collections::BTreeMap<String, String>> = Vec::new();
    let mut current: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let flush = |current: &mut std::collections::BTreeMap<String, String>,
                 blocks: &mut Vec<std::collections::BTreeMap<String, String>>| {
        if !current.is_empty() {
            blocks.push(std::mem::take(current));
        }
    };
    for line in text.lines() {
        if line.trim().is_empty() {
            flush(&mut current, &mut blocks);
            continue;
        }
        let mut parts = line.splitn(2, char::is_whitespace);
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").trim();
        if !key.is_empty() {
            current.insert(key.to_string(), value.to_string());
        }
    }
    flush(&mut current, &mut blocks);
    let mut found = Vec::new();
    for block in blocks {
        if block.get("format").map(|f| f == format).unwrap_or(false)
            && let (Some(asset), Some(origin)) = (block.get("url"), block.get("origin"))
        {
            found.push((asset.clone(), origin.clone()));
        }
    }
    if found.is_empty() {
        return Err(format!("{register} carries no {format} entry"));
    }
    Ok(found)
}

fn run_register(args: &[String]) -> Result<(), String> {
    let usage = "usage: eeg_descriptor --register <phi/sources.φ> --format <name> [--limit <n>] [--out <tsv>]";
    let mut register = None;
    let mut format = None;
    let mut limit = usize::MAX;
    let mut out: Option<String> = None;
    let mut index = 0;
    while index < args.len() {
        let value = || {
            args.get(index + 1)
                .cloned()
                .ok_or_else(|| usage.to_string())
        };
        match args[index].as_str() {
            "--register" => register = Some(value()?),
            "--format" => format = Some(value()?),
            "--limit" => {
                limit = value()?
                    .parse::<usize>()
                    .map_err(|_| "limit carries no number".to_string())?
            }
            "--out" => out = Some(value()?),
            other => return Err(format!("{other}: not a flag of this mode")),
        }
        index += 2;
    }
    let register = register.ok_or_else(|| usage.to_string())?;
    let format = format.ok_or_else(|| usage.to_string())?;
    let list = entries(&register, &format)?;
    let mut sink: Option<std::io::BufWriter<fs::File>> = match &out {
        None => None,
        Some(path) => {
            let file =
                fs::File::create(path).map_err(|e| format!("create {path} returned void: {e}"))?;
            let mut writer = std::io::BufWriter::new(file);
            use std::io::Write;
            writeln!(
                writer,
                "asset\tchannels\tpoints\ttrials\trate_hz\ttrial_s\tbytes\tverdict"
            )
            .map_err(|e| format!("write {path} returned void: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("flush {path} returned void: {e}"))?;
            Some(writer)
        }
    };
    let mut gated = 0usize;
    let mut absent = 0usize;
    for (asset, _) in list.iter().take(limit) {
        let name = asset.rsplit('/').next().unwrap_or("unnamed").to_string();
        let line = match (asset_ref(asset), gh_token()) {
            (Err(reason), _) | (_, Err(reason)) => {
                absent += 1;
                format!("{name}\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}")
            }
            (Ok((api, size)), Ok(token)) => match head_of(&api, &token, HEAD_BYTES) {
                Err(reason) => {
                    absent += 1;
                    format!("{name}\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}")
                }
                Ok(head_bytes) => match describe(&head_bytes, size) {
                    Ok(report) => {
                        gated += 1;
                        format!(
                            "{name}\t{}\t{}\t{}\t{}\t{}\t{}\tgated",
                            report.head.nbchan,
                            report.head.pnts,
                            report.head.trials,
                            match report.head.srate {
                                Some(rate) => format!("{rate:.3}"),
                                None => "absent".to_string(),
                            },
                            match report.duration_s {
                                Some(d) => format!("{d:.3}"),
                                None => "absent".to_string(),
                            },
                            report.expected_total
                        )
                    }
                    Err(reason) => {
                        absent += 1;
                        format!("{name}\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}")
                    }
                },
            },
        };
        eprintln!("{line}");
        if let Some(writer) = sink.as_mut() {
            use std::io::Write;
            writeln!(writer, "{line}").map_err(|e| format!("write returned void: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("flush returned void: {e}"))?;
        }
    }
    println!(
        "gated {gated}  absent {absent}  of {} entries",
        list.len().min(limit)
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--register") {
        return run_register(&args);
    }
    let usage = "usage: eeg_descriptor <asset.bin> [<asset.bin> ...]";
    if args.is_empty() {
        return Err(usage.to_string());
    }
    for path in &args {
        let whole = fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
        match describe(&whole, whole.len()) {
            Ok(report) => println!(
                "{path}\t{}\t{}\t{}\t{}\t{}",
                report.head.nbchan,
                report.head.pnts,
                report.head.trials,
                match report.duration_s {
                    Some(d) => format!("{d:.3}"),
                    None => "absent".to_string(),
                },
                "gated"
            ),
            Err(reason) => println!("{path}\tabsent\t{reason}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(nbchan: u32, pnts: u64, trials: u32, srate: Option<f64>, samples_len: u64) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(MAGIC);
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&nbchan.to_le_bytes());
        buf.extend_from_slice(&pnts.to_le_bytes());
        buf.extend_from_slice(&trials.to_le_bytes());
        buf.push(srate.is_some() as u8);
        buf.extend_from_slice(&[0u8; 3]);
        buf.extend_from_slice(&srate.unwrap_or(0.0).to_le_bytes());
        buf.extend_from_slice(&[0u8; 32]);
        for len in [0u32, 0, 0, nbchan, 0] {
            buf.extend_from_slice(&len.to_le_bytes());
        }
        buf.extend_from_slice(&samples_len.to_le_bytes());
        for index in 0..nbchan {
            let label = format!("C{index}");
            buf.extend_from_slice(&(label.len() as u16).to_le_bytes());
            buf.extend_from_slice(label.as_bytes());
        }
        buf
    }

    #[test]
    fn a_consistent_head_passes_with_its_trial_length() {
        let head = build(2, 100, 3, Some(50.0), 2 * 100 * 3 * 4);
        let total = head.len() + 2 * 100 * 3 * 4;
        let report = describe(&head, total).expect("gated");
        assert_eq!(report.head.nbchan, 2);
        assert_eq!(report.head.labels, 2);
        assert!((report.duration_s.expect("rate") - 2.0).abs() < 1e-9);
    }

    #[test]
    fn a_sample_count_that_contradicts_the_header_is_absent() {
        let head = build(2, 100, 3, Some(50.0), 2 * 100 * 3 * 4 + 4);
        let total = head.len() + 2 * 100 * 3 * 4 + 4;
        let reason = describe(&head, total).expect_err("must not pass");
        assert!(
            reason.contains("2404") && reason.contains("2400"),
            "{reason}"
        );
    }

    #[test]
    fn a_trial_length_outside_the_day_is_absent() {
        let head = build(1, 100, 1, Some(0.001), 400);
        let total = head.len() + 400;
        let reason = describe(&head, total).expect_err("must not pass");
        assert!(reason.contains("s"), "{reason}");
    }
}
