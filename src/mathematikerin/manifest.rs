use crate::archivar::kernel_id_of;
use std::collections::BTreeSet;

pub const MANIFEST_PATH: &str = "src/mathematikerin/manifest.phi";

pub fn load_manifest(path: &str) -> Result<Vec<ManifestConfig>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|error| format!("{path}: manifest unreadable: {error}"))?;
    parse_manifest(&content)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManifestTauSource {
    Measured,
    Derived,
    Chosen,
    Default,
    Pending,
}

impl ManifestTauSource {
    fn parse(token: &str) -> Option<Self> {
        match token {
            "measured" => Some(Self::Measured),
            "derived" => Some(Self::Derived),
            "chosen" => Some(Self::Chosen),
            "default" => Some(Self::Default),
            "pending" => Some(Self::Pending),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManifestIntent {
    Science,
    Art,
    Test,
}

impl ManifestIntent {
    fn parse(token: &str) -> Option<Self> {
        match token {
            "science" => Some(Self::Science),
            "art" => Some(Self::Art),
            "test" => Some(Self::Test),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ManifestConfig {
    pub source_id: String,
    pub field_key: String,
    pub anchor_ref: String,
    pub kernel: Option<u8>,
    pub tau: Option<f64>,
    pub tau_src: Option<ManifestTauSource>,
    pub absorption: Option<f64>,
    pub advection: Option<f64>,
    pub intent: Option<ManifestIntent>,
}

fn parse_slot(token: &str) -> Option<Option<f64>> {
    match token {
        "absent" | "pending" => Some(None),
        _ => token
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite() && *value >= 0.0)
            .map(Some),
    }
}

pub fn parse_manifest(content: &str) -> Result<Vec<ManifestConfig>, String> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    let mut current: Option<ManifestConfig> = None;
    let mut last_rank = 0u8;

    for (line_no, raw) in content.lines().enumerate() {
        let line_no = line_no + 1;
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let head = parts.first().copied().unwrap_or("");
        if head == "manifest" {
            if let Some(block) = current.take() {
                out.push(block);
            }
            if parts.len() != 3 {
                return Err(format!("line {line_no}: manifest needs <source_id> <field_key>"));
            }
            if parts[1].is_empty() || parts[2].is_empty() {
                return Err(format!("line {line_no}: source_id and field_key must not be empty"));
            }
            if !seen.insert((parts[1].to_string(), parts[2].to_string())) {
                return Err(format!(
                    "line {line_no}: duplicate manifest {} {}",
                    parts[1], parts[2]
                ));
            }
            current = Some(ManifestConfig {
                source_id: parts[1].to_string(),
                field_key: parts[2].to_string(),
                anchor_ref: String::new(),
                kernel: None,
                tau: None,
                tau_src: None,
                absorption: None,
                advection: None,
                intent: None,
            });
            last_rank = 0;
            continue;
        }

        let Some(block) = current.as_mut() else {
            return Err(format!("line {line_no}: {head} outside a manifest block"));
        };

        let rank = match head {
            "anchor_ref" => 1,
            "kernel" => 2,
            "tau" => 3,
            "absorption" => 4,
            "advection" => 5,
            "intent" => 6,
            _ => return Err(format!("line {line_no}: unread manifest directive {head}")),
        };
        if rank < last_rank {
            return Err(format!(
                "line {line_no}: manifest directives out of order at {head}"
            ));
        }
        last_rank = rank;

        match head {
            "anchor_ref" if parts.len() == 2 => block.anchor_ref = parts[1].to_string(),
            "kernel" if parts.len() == 2 => {
                block.kernel = Some(
                    kernel_id_of(parts[1])
                        .ok_or_else(|| format!("line {line_no}: unknown kernel {}", parts[1]))?,
                );
            }
            "tau" if parts.len() == 3 => {
                block.tau_src = Some(
                    ManifestTauSource::parse(parts[2])
                        .ok_or_else(|| format!("line {line_no}: unknown tau source {}", parts[2]))?,
                );
                block.tau = if parts[1] == "pending" {
                    None
                } else {
                    Some(
                        parts[1]
                            .parse::<f64>()
                            .ok()
                            .filter(|value| value.is_finite() && *value > 0.0)
                            .ok_or_else(|| format!("line {line_no}: tau must be positive or pending"))?,
                    )
                };
            }
            "absorption" if parts.len() == 2 => {
                block.absorption = parse_slot(parts[1])
                    .ok_or_else(|| format!("line {line_no}: absorption value unreadable"))?;
            }
            "advection" if parts.len() == 2 => {
                block.advection = parse_slot(parts[1])
                    .ok_or_else(|| format!("line {line_no}: advection value unreadable"))?;
            }
            "intent" if parts.len() == 2 => {
                block.intent = Some(
                    ManifestIntent::parse(parts[1])
                        .ok_or_else(|| format!("line {line_no}: unknown intent {}", parts[1]))?,
                );
            }
            _ => return Err(format!("line {line_no}: unread manifest directive {head}")),
        }
    }

    if let Some(block) = current {
        out.push(block);
    }

    for block in &out {
        if block.anchor_ref.is_empty() {
            return Err(format!(
                "manifest {} {}: anchor_ref absent",
                block.source_id, block.field_key
            ));
        }
        if block.kernel.is_none() {
            return Err(format!(
                "manifest {} {}: kernel absent",
                block.source_id, block.field_key
            ));
        }
        if block.tau_src.is_none() {
            return Err(format!(
                "manifest {} {}: tau absent",
                block.source_id, block.field_key
            ));
        }
        if block.intent.is_none() {
            return Err(format!(
                "manifest {} {}: intent absent",
                block.source_id, block.field_key
            ));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_complete_manifest() {
        let text = "manifest abc field_1\n\
anchor_ref station_42\n\
kernel inverse-square\n\
tau 60 chosen\n\
absorption absent\n\
advection 400.0\n\
intent science\n";
        let out = parse_manifest(text).expect("manifest parses");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].source_id, "abc");
        assert_eq!(out[0].field_key, "field_1");
        assert_eq!(out[0].anchor_ref, "station_42");
        assert_eq!(out[0].kernel, Some(0));
        assert_eq!(out[0].tau, Some(60.0));
        assert_eq!(out[0].tau_src, Some(ManifestTauSource::Chosen));
        assert_eq!(out[0].absorption, None);
        assert_eq!(out[0].advection, Some(400.0));
        assert_eq!(out[0].intent, Some(ManifestIntent::Science));
    }

    #[test]
    fn duplicate_source_field_is_refused() {
        let text = "manifest a b\n\
anchor_ref x\n\
kernel inverse-square\n\
tau 1 chosen\n\
intent test\n\
manifest a b\n\
anchor_ref y\n\
kernel inverse-square\n\
tau 1 chosen\n\
intent test\n";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn missing_anchor_is_refused() {
        let text = "manifest a b\nkernel inverse-square\ntau 1 chosen\nintent test\n";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn unknown_kernel_and_intent_are_refused() {
        let bad_kernel = "manifest a b\nanchor_ref x\nkernel nope\ntau 1 chosen\nintent test\n";
        assert!(parse_manifest(bad_kernel).is_err());
        let bad_intent = "manifest a b\nanchor_ref x\nkernel inverse-square\ntau 1 chosen\nintent nope\n";
        assert!(parse_manifest(bad_intent).is_err());
    }

    #[test]
    fn missing_kernel_or_intent_is_refused() {
        let no_kernel = "manifest a b\nanchor_ref x\ntau 1 chosen\nintent test\n";
        assert!(parse_manifest(no_kernel).is_err());
        let no_intent = "manifest a b\nanchor_ref x\nkernel inverse-square\ntau 1 chosen\n";
        assert!(parse_manifest(no_intent).is_err());
    }

    #[test]
    fn out_of_order_is_refused() {
        let text = "manifest a b\ntau 1 chosen\nanchor_ref x\nkernel inverse-square\nintent test\n";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn pending_tau_is_none_not_zero() {
        let text = "manifest a b\nanchor_ref x\nkernel inverse-square\ntau pending pending\nintent art\n";
        let out = parse_manifest(text).expect("pending parses");
        assert_eq!(out[0].tau, None);
        assert_eq!(out[0].tau_src, Some(ManifestTauSource::Pending));
    }
}
