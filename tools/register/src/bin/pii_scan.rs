use std::fs;
use std::path::Path;

const PII_TOKENS: [&str; 28] = [
    "email",
    "e_mail",
    "phone",
    "mobile",
    "address",
    "street",
    "postal",
    "zip",
    "iban",
    "card",
    "account",
    "passport",
    "ssn",
    "birth",
    "dob",
    "biometric",
    "fingerprint",
    "face",
    "voice",
    "medical",
    "patient",
    "diagnosis",
    "salary",
    "insurance",
    "tax",
    "user",
    "device",
    "session",
];

fn tokens(field_name: &str) -> Vec<String> {
    field_name
        .to_lowercase()
        .split(['_', '-', '.'])
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

fn is_personal(field_name: &str) -> Option<String> {
    tokens(field_name)
        .into_iter()
        .find(|t| PII_TOKENS.contains(&t.as_str()))
}

fn register_fields(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("field ") else {
            continue;
        };
        let mut parts = rest.split_whitespace();
        let _key = parts.next();
        if let Some(name) = parts.next() {
            out.push((i + 1, name.to_string()));
        }
    }
    out
}

fn scan(text: &str) -> Vec<String> {
    let mut findings = Vec::new();
    for (line, name) in register_fields(text) {
        if let Some(token) = is_personal(&name) {
            findings.push(format!("line {line}: field {name} carries `{token}`"));
        }
    }
    findings
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let register = match args.iter().position(|a| a == "--register") {
        Some(i) => match args.get(i + 1) {
            Some(path) => path.clone(),
            None => {
                eprintln!("pii_scan: --register needs a path");
                std::process::exit(2);
            }
        },
        None => "phi/sources.φ".to_string(),
    };
    let Ok(text) = fs::read_to_string(Path::new(&register)) else {
        eprintln!("pii_scan: {register} is unreadable");
        std::process::exit(2);
    };
    let findings = scan(&text);
    for f in &findings {
        println!("{f}");
    }
    println!(
        "pii_scan: {} field(s), {} with a personal-data token (NIST SP 800-122)",
        register_fields(&text).len(),
        findings.len()
    );
    if !findings.is_empty() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_personal_token_in_a_field_name_is_named() {
        let text = "field x station_email m/s 60\n";
        let findings = scan(text);
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(findings[0].contains("email"), "{findings:?}");
    }

    #[test]
    fn a_physical_field_passes() {
        let text = "field crosswind champ_thermosphere_crosswind_ms m/s 10\n";
        assert!(scan(text).is_empty());
    }

    #[test]
    fn a_token_inside_a_longer_word_does_not_count() {
        let text = "field x equipment_temperature K 60\n";
        assert!(scan(text).is_empty());
    }
}
