use std::env;
use std::fs;
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use omegaflow::archivar::{
    date_str, extract_netloc, live_markers, load_env, load_sources, render_headers, resolve_secret,
    unresolved_key,
};

const HOST_PAUSE_S: u64 = 2;
const REQUEST_TIMEOUT_S: u64 = 20;
const MAX_FILESIZE_BYTES: u64 = 1 << 20;
const DEFAULT_TIME_CAP_S: u64 = 7200;
const DEFAULT_OUT: &str = "phi/reports/source_latency_census.φ";
const ABSENT: &str = "absent";
const CURL_FORMAT: &str = "%{http_code}\t%{time_namelookup}\t%{time_connect}\t%{time_appconnect}\t%{time_starttransfer}\t%{size_download}";

#[derive(Clone)]
struct Target {
    url: String,
    headers: Vec<(String, String)>,
}

#[derive(Clone)]
struct Timings {
    dns_ms: Option<f64>,
    connect_ms: Option<f64>,
    tls_ms: Option<f64>,
    ttfb_ms: Option<f64>,
    http_code: Option<u16>,
}

struct Block {
    url: String,
    timings: Timings,
    probe: &'static str,
    date: String,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_blocked_urls(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(content) => content
            .lines()
            .map(str::trim)
            .filter_map(|l| l.strip_prefix("url "))
            .map(str::trim)
            .filter(|u| u.starts_with("http"))
            .map(str::to_string)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn host_key(url: &str) -> String {
    match extract_netloc(url) {
        Some(n) => n.to_string(),
        None => String::new(),
    }
}

fn substitute_markers(url: &str, markers: &[(String, String)]) -> String {
    let mut out = url.to_string();
    for (k, v) in markers {
        out = out.replace(k, v);
    }
    out
}

fn parse_timing(token: &str) -> Option<f64> {
    let seconds: f64 = token.trim().parse().ok()?;
    if seconds.is_finite() && seconds > 0.0 {
        Some(seconds * 1000.0)
    } else {
        None
    }
}

fn parse_curl_line(line: &str) -> Timings {
    let fields: Vec<&str> = line.trim().split('\t').collect();
    let http_code = fields
        .first()
        .and_then(|t| t.trim().parse::<u16>().ok())
        .filter(|c| *c > 0);
    let at = |i: usize| fields.get(i).copied().and_then(parse_timing);
    Timings {
        dns_ms: at(1),
        connect_ms: at(2),
        tls_ms: at(3),
        ttfb_ms: at(4),
        http_code,
    }
}

fn format_ms(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{:.2}", x),
        None => ABSENT.to_string(),
    }
}

fn format_block(b: &Block) -> String {
    let code = match b.timings.http_code {
        Some(c) => c.to_string(),
        None => ABSENT.to_string(),
    };
    format!(
        "url {}\ndns_ms {}\nconnect_ms {}\ntls_ms {}\nttfb_ms {}\nhttp_code {}\nprobe {}\ndate {}\n",
        b.url,
        format_ms(b.timings.dns_ms),
        format_ms(b.timings.connect_ms),
        format_ms(b.timings.tls_ms),
        format_ms(b.timings.ttfb_ms),
        code,
        b.probe,
        b.date,
    )
}

fn curl_census(url: &str, headers: &[(String, String)]) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-L")
        .arg("-g")
        .arg("-o")
        .arg("/dev/null")
        .arg("-r")
        .arg("0-65535")
        .arg("--max-filesize")
        .arg(MAX_FILESIZE_BYTES.to_string())
        .arg("-m")
        .arg(REQUEST_TIMEOUT_S.to_string())
        .arg("-w")
        .arg(CURL_FORMAT);
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn today() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => date_str(d.as_secs()),
        Err(_) => ABSENT.to_string(),
    }
}

fn nearest_rank_p90(sorted: &[f64]) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let rank = ((sorted.len() as f64 * 0.9).ceil() as usize).clamp(1, sorted.len());
    Some(sorted[rank - 1])
}

fn p90_per_host(blocks: &[Block]) -> Vec<(String, f64, usize)> {
    let mut by_host: Vec<(String, Vec<f64>)> = Vec::new();
    for b in blocks {
        if b.probe != "ok" {
            continue;
        }
        let Some(ttfb) = b.timings.ttfb_ms else {
            continue;
        };
        let host = host_key(&b.url);
        match by_host.iter_mut().find(|(h, _)| *h == host) {
            Some((_, vals)) => vals.push(ttfb),
            None => by_host.push((host, vec![ttfb])),
        }
    }
    by_host.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = Vec::new();
    for (host, mut vals) in by_host {
        vals.sort_by(|a, b| a.total_cmp(b));
        let n = vals.len();
        if let Some(p90) = nearest_rank_p90(&vals) {
            out.push((host, p90, n));
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let limit: Option<usize> = arg_value(&args, "--limit").and_then(|s| s.parse().ok());
    let include_blocked = args.iter().any(|a| a == "--blocked");
    let out = arg_value(&args, "--out").unwrap_or(DEFAULT_OUT.to_string());
    let time_cap_s: u64 = arg_value(&args, "--time-cap")
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_TIME_CAP_S);

    let env_map = load_env();
    let markers = live_markers();

    let mut targets: Vec<Target> = load_sources()
        .into_iter()
        .map(|s| Target {
            url: s.url,
            headers: s.headers,
        })
        .collect();
    if include_blocked {
        for u in read_blocked_urls("phi/blocked_sources.φ") {
            targets.push(Target {
                url: u,
                headers: Vec::new(),
            });
        }
    }
    targets.sort_by(|a, b| host_key(&a.url).cmp(&host_key(&b.url)));

    let started = Instant::now();
    let date = today();
    let mut blocks: Vec<Block> = Vec::new();
    let mut prev_host: Option<String> = None;
    let mut counted = 0usize;

    for t in &targets {
        if let Some(l) = limit {
            if counted >= l {
                break;
            }
        }
        if started.elapsed().as_secs() >= time_cap_s {
            eprintln!(
                "time cap {} s reached at {} of {}",
                time_cap_s,
                counted,
                targets.len()
            );
            break;
        }
        counted += 1;

        let host = host_key(&t.url);
        if let Some(prev) = &prev_host {
            if *prev != host {
                std::thread::sleep(std::time::Duration::from_secs(HOST_PAUSE_S));
            }
        }
        prev_host = Some(host);

        let substituted = substitute_markers(&t.url, &markers);
        let mut key_absent = unresolved_key(&substituted, &env_map);
        if key_absent.is_none() {
            for (_, v) in &t.headers {
                if let Some(k) = unresolved_key(v, &env_map) {
                    key_absent = Some(k);
                    break;
                }
            }
        }

        let block = match key_absent {
            Some(_) => Block {
                url: substituted,
                timings: Timings {
                    dns_ms: None,
                    connect_ms: None,
                    tls_ms: None,
                    ttfb_ms: None,
                    http_code: None,
                },
                probe: "key-absent",
                date: date.clone(),
            },
            None => {
                let resolved = resolve_secret(&substituted, &env_map);
                let headers = render_headers(&t.headers, &env_map);
                let line = curl_census(&resolved, &headers);
                let timings = match &line {
                    Some(l) => parse_curl_line(l),
                    None => Timings {
                        dns_ms: None,
                        connect_ms: None,
                        tls_ms: None,
                        ttfb_ms: None,
                        http_code: None,
                    },
                };
                let probe: &'static str = match timings.http_code {
                    None => "broken",
                    Some(c) if (400..=599).contains(&c) => "refused",
                    Some(_) => "ok",
                };
                Block {
                    url: substituted,
                    timings,
                    probe,
                    date: date.clone(),
                }
            }
        };
        eprintln!("{} -> {}", block.url, block.probe);
        blocks.push(block);
    }

    let families = p90_per_host(&blocks);
    let mut report = String::new();
    for b in &blocks {
        report.push_str(&format_block(b));
        report.push('\n');
    }
    report.push_str("# p90 ttfb_ms per host family (probe ok only; own distribution)\n");
    for (host, p90, n) in &families {
        report.push_str(&format!("host {} p90_ttfb_ms {:.2} n {}\n", host, p90, n));
    }

    if let Some(dir) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(dir);
    }
    match fs::write(&out, &report) {
        Ok(()) => {
            eprintln!("wrote {} blocks to {}", blocks.len(), out);
            println!(
                "blocks {} (ok {} / refused {} / broken {} / key-absent {})",
                blocks.len(),
                blocks.iter().filter(|b| b.probe == "ok").count(),
                blocks.iter().filter(|b| b.probe == "refused").count(),
                blocks.iter().filter(|b| b.probe == "broken").count(),
                blocks.iter().filter(|b| b.probe == "key-absent").count(),
            );
            for (host, p90, n) in &families {
                println!("host {} p90_ttfb_ms {:.2} n {}", host, p90, n);
            }
        }
        Err(e) => {
            eprintln!("write {} returned void: {}", out, e);
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitute_markers_replaces_every_known_marker() {
        let markers = vec![
            ("{lat}".to_string(), "29.5".to_string()),
            ("{lon}".to_string(), "-95.0".to_string()),
            ("{yesterday}".to_string(), "2026-09-13".to_string()),
        ];
        let out = substitute_markers(
            "https://example.test/api?lat={lat}&lon={lon}&d={yesterday}",
            &markers,
        );
        assert_eq!(
            out,
            "https://example.test/api?lat=29.5&lon=-95.0&d=2026-09-13"
        );
    }

    #[test]
    fn parse_curl_line_maps_timing_fields_and_absent_appconnect() {
        let t = parse_curl_line("206\t0.016435\t0.029843\t0.000000\t0.056402\t559");
        assert_eq!(t.http_code, Some(206));
        assert!(matches!(t.dns_ms, Some(v) if (v - 16.435).abs() < 0.01));
        assert!(matches!(t.connect_ms, Some(v) if (v - 29.843).abs() < 0.01));
        assert_eq!(t.tls_ms, None);
        assert!(matches!(t.ttfb_ms, Some(v) if (v - 56.402).abs() < 0.01));
    }

    #[test]
    fn parse_curl_line_https_carries_tls() {
        let t = parse_curl_line("200\t0.000976\t0.014197\t0.056043\t0.080131\t65536");
        assert_eq!(t.http_code, Some(200));
        assert!(matches!(t.tls_ms, Some(v) if (v - 56.043).abs() < 0.01));
    }

    #[test]
    fn parse_curl_line_broken_has_no_code() {
        let t = parse_curl_line("000\t0.000\t0.000\t0.000\t0.000\t0");
        assert_eq!(t.http_code, None);
        assert_eq!(t.dns_ms, None);
        assert_eq!(t.ttfb_ms, None);
    }

    #[test]
    fn format_block_emits_absent_tokens_not_zero() {
        let b = Block {
            url: "https://example.test/x".to_string(),
            timings: Timings {
                dns_ms: Some(12.5),
                connect_ms: None,
                tls_ms: None,
                ttfb_ms: Some(300.0),
                http_code: Some(200),
            },
            probe: "ok",
            date: "2026-09-14".to_string(),
        };
        let s = format_block(&b);
        assert!(s.contains("url https://example.test/x\n"));
        assert!(s.contains("dns_ms 12.50\n"));
        assert!(s.contains("connect_ms absent\n"));
        assert!(s.contains("tls_ms absent\n"));
        assert!(s.contains("ttfb_ms 300.00\n"));
        assert!(s.contains("http_code 200\n"));
        assert!(s.contains("probe ok\n"));
        assert!(s.contains("date 2026-09-14\n"));
    }

    #[test]
    fn format_block_key_absent_has_absent_http_code() {
        let b = Block {
            url: "https://example.test/{TNS_UA}".to_string(),
            timings: Timings {
                dns_ms: None,
                connect_ms: None,
                tls_ms: None,
                ttfb_ms: None,
                http_code: None,
            },
            probe: "key-absent",
            date: "2026-09-14".to_string(),
        };
        let s = format_block(&b);
        assert!(s.contains("http_code absent\n"));
        assert!(s.contains("probe key-absent\n"));
        assert!(s.contains("ttfb_ms absent\n"));
    }

    #[test]
    fn nearest_rank_p90_picks_the_ninetieth_percentile() {
        let vals: Vec<f64> = (1..=10).map(|x| x as f64).collect();
        assert_eq!(nearest_rank_p90(&vals), Some(9.0));
        assert_eq!(nearest_rank_p90(&[]), None);
    }
}
