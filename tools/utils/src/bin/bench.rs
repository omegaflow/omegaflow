use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::Instant;

use omegaflow::json::{JsonVal, parse_json};
use omegaflow::te::{permutation_entropy, transfer_entropy};

const GATE: &str = "http://127.0.0.1:4100/v1/chat/completions";

const TASKS: [(&str, &str, &str); 6] = [
    (
        "force-m-s",
        "Which omegaflow force maps to the unit m/s? Answer with exactly one word.",
        "advective",
    ),
    (
        "em-hpa",
        "Is `em hPa` a legal force-unit pair in the register? Answer yes or no.",
        "no",
    ),
    (
        "zero-kanon",
        "Name one 0-Kanon term for an absent value.",
        "pending",
    ),
    (
        "a-equals-a",
        "State the A = A axiom of omegaflow in one short sentence.",
        "oscillator",
    ),
    (
        "presence",
        "In the block universe, does the presence rest or travel? One word.",
        "rest",
    ),
    (
        "translate",
        "Translate to English: die Prosa ist der Gegenhang.",
        "counter-slope",
    ),
];

const MATRIX: [(&str, &str); 4] = [
    ("deepseek-v4-pro", "high"),
    ("deepseek-v4-pro", "default"),
    ("deepseek-v4-flash", "high"),
    ("deepseek-v4-flash", "low"),
];

struct Run {
    latency_ms: u128,
    response: String,
    reasoning_tokens: u64,
}

fn main() {
    let mut rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = Vec::new();
    for (model, variant) in MATRIX {
        eprintln!("bench: {} {}", model, variant);
        let mut lat_sum = 0u128;
        let mut tok_sum = 0u64;
        let mut reas_sum = 0u64;
        let mut correct = 0u64;
        let mut te_sum = 0f64;
        let mut pe_sum = 0f64;
        let mut n = 0u64;
        for (name, prompt, answer) in TASKS {
            let Some(run) = fetch(model, variant, prompt) else {
                eprintln!("  {} void", name);
                continue;
            };
            eprintln!("  {} {}ms", name, run.latency_ms);
            n += 1;
            lat_sum += run.latency_ms;
            tok_sum += word_count(&run.response) as u64;
            reas_sum += run.reasoning_tokens;
            if run.response.to_lowercase().contains(answer) {
                correct += 1;
            }
            let (te, pe) = metrics(prompt, &run.response);
            if let Some(t) = te {
                te_sum += t;
            }
            if let Some(p) = pe {
                pe_sum += p;
            }
        }
        if n == 0 {
            rows.push((
                model.to_string(),
                variant.to_string(),
                "void".to_string(),
                "void".to_string(),
                "void".to_string(),
                "void".to_string(),
                "void".to_string(),
                "void".to_string(),
            ));
            continue;
        }
        rows.push((
            model.to_string(),
            variant.to_string(),
            format!("{:.0}", lat_sum as f64 / n as f64),
            format!("{:.0}", tok_sum as f64 / n as f64),
            format!("{:.0}", reas_sum as f64 / n as f64),
            format!("{}/{}", correct, n),
            format!("{:.4}", te_sum / n as f64),
            format!("{:.4}", pe_sum / n as f64),
        ));
    }
    println!("model variant lat_ms tok reasoning correct TE PE");
    for r in &rows {
        println!(
            "{} {} {} {} {} {} {} {}",
            r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7
        );
    }
}

fn fetch(model: &str, variant: &str, prompt: &str) -> Option<Run> {
    let mut body = format!(
        "{{\"model\":\"{}\",\"stream\":true,\"messages\":[{{\"role\":\"user\",\"content\":\"{}\"}}]",
        model,
        json_escape(prompt)
    );
    if variant != "default" {
        body.push_str(&format!(",\"reasoning_effort\":\"{}\"", variant));
    }
    body.push_str("}");
    let start = Instant::now();
    let mut child = Command::new("curl")
        .arg("-sS")
        .arg("-N")
        .arg("--max-time")
        .arg("120")
        .arg(GATE)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg("Authorization: Bearer gate")
        .arg("--data-binary")
        .arg(&body)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let mut response = String::new();
    let mut reasoning_tokens = 0u64;
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            let line = line.trim_end().to_string();
            if !line.starts_with("data:") {
                continue;
            }
            let payload = line[5..].trim();
            if payload == "[DONE]" {
                break;
            }
            let Some(data) = parse_json(payload) else {
                continue;
            };
            if let Some(c) = delta_content(&data) {
                response.push_str(&c);
            }
            if let Some(u) = usage_reasoning(&data) {
                reasoning_tokens = u;
            }
        }
    }
    let _ = child.wait();
    let latency_ms = start.elapsed().as_millis();
    if response.trim().is_empty() {
        return None;
    }
    Some(Run {
        latency_ms,
        response,
        reasoning_tokens,
    })
}

fn delta_content(data: &JsonVal) -> Option<String> {
    let JsonVal::Obj(map) = data else { return None };
    let JsonVal::Arr(choices) = map.get("choices")? else {
        return None;
    };
    let JsonVal::Obj(first) = choices.first()? else {
        return None;
    };
    let JsonVal::Obj(delta) = first.get("delta")? else {
        return None;
    };
    match delta.get("content") {
        Some(JsonVal::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

fn usage_reasoning(data: &JsonVal) -> Option<u64> {
    let JsonVal::Obj(map) = data else { return None };
    let JsonVal::Obj(usage) = map.get("usage")? else {
        return None;
    };
    let JsonVal::Obj(details) = usage.get("completion_tokens_details")? else {
        return None;
    };
    match details.get("reasoning_tokens") {
        Some(JsonVal::Num(n)) => Some(*n as u64),
        _ => None,
    }
}

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn word_series(text: &str) -> Vec<f32> {
    text.split_whitespace()
        .map(|w| {
            let mut h: u32 = 2166136261;
            for b in w.bytes() {
                h ^= b as u32;
                h = h.wrapping_mul(16777619);
            }
            (h % 100000) as f32 / 100000.0
        })
        .collect()
}

fn metrics(prompt: &str, response: &str) -> (Option<f64>, Option<f64>) {
    let px = word_series(prompt);
    let rx = word_series(response);
    let n = px.len().min(rx.len());
    let te = if n >= 12 {
        transfer_entropy(&px[..n], &rx[..n])
    } else {
        None
    };
    let pe = if rx.len() >= 6 {
        let r64: Vec<f64> = rx.iter().map(|v| *v as f64).collect();
        permutation_entropy(&r64, 4, 1)
    } else {
        None
    };
    (te, pe)
}

fn json_escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_series_hash() {
        let s = word_series("a b c");
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn escape_quotes() {
        assert_eq!(json_escape("a\"b"), "a\\\"b");
    }

    #[test]
    fn metrics_run() {
        let (te, pe) = metrics(
            "Which omegaflow force maps to the unit m/s",
            "the advective force carries m/s in the register",
        );
        let _ = (te, pe);
    }
}
