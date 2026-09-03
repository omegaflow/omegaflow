use std::collections::HashMap;

use crate::json::{jstr, parse_json};

#[derive(Clone, Copy, Debug)]
pub struct ModelProps {
    pub context: f64,
    pub output: f64,
}

pub fn parse_model_register(content: &str) -> HashMap<String, ModelProps> {
    let mut models = HashMap::new();
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let tokens: Vec<&str> = t.split_whitespace().collect();
        if tokens.len() < 4 || tokens[0] != "model" {
            continue;
        }
        let id = tokens[1].to_string();
        let context = match tokens[2].parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => continue,
        };
        let output = match tokens[3].parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => continue,
        };
        models.insert(id, ModelProps { context, output });
    }
    models
}

pub struct Friction {
    pub window: Vec<String>,
    pub posture: Vec<&'static str>,
    pub limit_phrases: Vec<&'static str>,
    pub model: Option<ModelProps>,
    pub prompt_tokens: Option<u64>,
    pub fp_run: u32,
    pub last_fp: String,
    pub turn_files: Vec<String>,
    pub handover_marked: bool,
    pub tool_sigs: Vec<String>,
    pub tool_name_counts: Vec<(String, u32)>,
    pub tool_calls: u32,
}

pub enum Advice {
    Loop,
    Repetition,
    Posture,
    Handover,
}

impl Friction {
    pub fn new() -> Friction {
        Friction {
            window: Vec::new(),
            posture: vec![
                "the human",
                "for humans",
                "human perception",
                "the user is",
                "as an ai",
                "i am an ai",
                "i'm an ai",
                "the observer sees",
                "the camera sees",
                "my system",
                "our machine",
                "the model should",
            ],
            limit_phrases: vec![
                "context is full",
                "my context",
                "context window",
                "i'm losing",
                "i am losing",
                "losing track",
                "repeating myself",
                "i am repeating",
                "out of context",
                "context budget",
            ],
            model: None,
            prompt_tokens: None,
            fp_run: 0,
            last_fp: String::new(),
            turn_files: Vec::new(),
            handover_marked: false,
            tool_sigs: Vec::new(),
            tool_name_counts: Vec::new(),
            tool_calls: 0,
        }
    }

    pub fn check(&mut self, text: &str) -> Vec<Advice> {
        let mut advice = Vec::new();
        let lower = text.to_lowercase();
        for p in &self.posture {
            if lower.contains(p) {
                advice.push(Advice::Posture);
                break;
            }
        }
        if !self.handover_marked {
            for p in &self.limit_phrases {
                if lower.contains(p) {
                    self.handover_marked = true;
                    advice.push(Advice::Handover);
                    break;
                }
            }
        }
        if let Some(fp) = fingerprint(text) {
            if fp == self.last_fp {
                self.fp_run += 1;
            } else {
                self.fp_run = 0;
                self.last_fp = fp.clone();
            }
            if self.fp_run >= 2 {
                advice.push(Advice::Loop);
            } else {
                self.window.push(fp);
                if self.window.len() > 40 {
                    self.window.remove(0);
                }
                let count = self.window.iter().filter(|w| **w == self.last_fp).count();
                if count >= 2 && !self.handover_marked {
                    advice.push(Advice::Repetition);
                }
            }
        }
        advice
    }

    pub fn set_model(&mut self, id: &str, register: &HashMap<String, ModelProps>) -> bool {
        self.prompt_tokens = None;
        match register.get(id) {
            Some(props) => {
                self.model = Some(*props);
                true
            }
            None => {
                self.model = None;
                false
            }
        }
    }

    pub fn note_usage(&mut self, prompt_tokens: u64) {
        self.prompt_tokens = Some(prompt_tokens);
    }

    pub fn context_fill(&self) -> Option<f64> {
        let model = self.model?;
        let tokens = self.prompt_tokens? as f64;
        let fill = tokens / model.context;
        if fill.is_finite() {
            Some(fill)
        } else {
            None
        }
    }

    pub fn threshold(&self) -> Option<f64> {
        let model = self.model?;
        let t = 1.0 - model.output / model.context;
        if t.is_finite() {
            Some(t)
        } else {
            None
        }
    }

    pub fn check_fill(&mut self) -> Vec<Advice> {
        if self.handover_marked {
            return Vec::new();
        }
        let Some(fill) = self.context_fill() else {
            return Vec::new();
        };
        let Some(threshold) = self.threshold() else {
            return Vec::new();
        };
        if fill >= threshold {
            self.handover_marked = true;
            vec![Advice::Handover]
        } else {
            Vec::new()
        }
    }

    pub fn note_tool(&mut self, args_json: &str) {
        let Some(obj) = parse_json(args_json) else {
            return;
        };
        let path = jstr(&obj, "filePath")
            .or_else(|| jstr(&obj, "path"))
            .or_else(|| jstr(&obj, "file"))
            .unwrap_or_default();
        if path.is_empty() {
            return;
        }
        if !self.turn_files.contains(&path) {
            self.turn_files.push(path);
        }
    }

    pub fn tool_friction(&mut self, name: &str, args: &str) -> Vec<Advice> {
        let mut advice = Vec::new();
        self.tool_calls += 1;
        let sig = format!(
            "{}|{}",
            name,
            args.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .chars()
                .take(120)
                .collect::<String>()
        );
        let same_run = self
            .tool_sigs
            .iter()
            .rev()
            .take_while(|s| **s == sig)
            .count();
        if same_run >= 2 {
            advice.push(Advice::Loop);
        }
        match self.tool_name_counts.iter_mut().find(|(n, _)| *n == name) {
            Some((_, c)) => {
                *c += 1;
                if *c >= 8 {
                    advice.push(Advice::Repetition);
                }
            }
            None => self.tool_name_counts.push((name.to_string(), 1)),
        }
        self.tool_sigs.push(sig);
        if self.tool_sigs.len() > 60 {
            self.tool_sigs.remove(0);
        }
        if self.tool_calls >= 120 && !self.handover_marked {
            self.handover_marked = true;
            advice.push(Advice::Handover);
        }
        advice
    }

    pub fn end_turn(&mut self) {
        self.turn_files.clear();
        self.last_fp.clear();
        self.fp_run = 0;
        self.window.clear();
        self.tool_sigs.clear();
        self.tool_name_counts.clear();
        self.tool_calls = 0;
        self.prompt_tokens = None;
    }

    pub fn commit_advice(&self, changes: &[String]) -> Option<String> {
        if changes.is_empty() {
            return None;
        }
        let register_touch = changes.iter().any(|f| {
            f.starts_with("TODO.md") || f.starts_with("status/") || f.starts_with("docs/")
        });
        let summary: String = changes
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        let more = changes.len().saturating_sub(5);
        let tail = if more > 0 {
            format!(", +{} more", more)
        } else {
            String::new()
        };
        let reason = if register_touch {
            "the register was touched — the commit is the checkmark"
        } else {
            "uncommitted work sits in the tree"
        };
        Some(format!(
            "commit recommended: {} ({}{})",
            reason, summary, tail
        ))
    }
}

fn fingerprint(text: &str) -> Option<String> {
    let fp: String = text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    if fp.len() < 30 {
        return None;
    }
    Some(fp.chars().take(80).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_after_repetition() {
        let mut f = Friction::new();
        let text = "the transfer entropy of the series stays above its null across the window";
        let mut advice = Vec::new();
        for _ in 0..3 {
            advice = f.check(text);
        }
        assert!(advice.iter().any(|a| matches!(a, Advice::Loop)));
    }

    #[test]
    fn posture_flagged() {
        let mut f = Friction::new();
        let advice = f.check("this view is designed for humans to understand the field");
        assert!(advice.iter().any(|a| matches!(a, Advice::Posture)));
    }

    #[test]
    fn limit_phrase_triggers_handover_once() {
        let mut f = Friction::new();
        let advice = f.check("my context is full and the session drifts");
        assert!(advice.iter().any(|a| matches!(a, Advice::Handover)));
        let second = f.check("my context is full and the session drifts");
        assert!(!second.iter().any(|a| matches!(a, Advice::Handover)));
    }

    #[test]
    fn fill_threshold_triggers_handover() {
        let mut f = Friction::new();
        let reg = parse_model_register("model deepseek-v4-pro 1000 100\n");
        assert!(f.set_model("deepseek-v4-pro", &reg));
        f.note_usage(950);
        let advice = f.check_fill();
        assert!(advice.iter().any(|a| matches!(a, Advice::Handover)));
    }

    #[test]
    fn fill_below_threshold_stays_quiet() {
        let mut f = Friction::new();
        let reg = parse_model_register("model deepseek-v4-pro 1000 100\n");
        assert!(f.set_model("deepseek-v4-pro", &reg));
        f.note_usage(500);
        assert!(f.check_fill().is_empty());
    }

    #[test]
    fn unknown_model_is_pending() {
        let mut f = Friction::new();
        let reg = parse_model_register("model deepseek-v4-pro 1000 100\n");
        assert!(!f.set_model("deepseek-v4-flash", &reg));
        f.note_usage(950);
        assert!(f.check_fill().is_empty());
    }

    #[test]
    fn threshold_derives_from_model_props() {
        let mut f = Friction::new();
        let reg = parse_model_register("model deepseek-v4-pro 1000 100\n");
        assert!(f.set_model("deepseek-v4-pro", &reg));
        assert_eq!(f.threshold(), Some(0.9));
        f.note_usage(900);
        assert_eq!(f.context_fill(), Some(0.9));
    }

    #[test]
    fn register_skips_pending_and_comments() {
        let reg = parse_model_register(
            "# harvested from the model card\nmodel a pending pending\nmodel b 1000 100\n",
        );
        assert!(!reg.contains_key("a"));
        assert!(reg.contains_key("b"));
    }

    #[test]
    fn register_skips_nonpositive_context() {
        let reg = parse_model_register("model a 0 100\nmodel b -5 100\nmodel c 1000 100\n");
        assert!(!reg.contains_key("a"));
        assert!(!reg.contains_key("b"));
        assert!(reg.contains_key("c"));
    }

    #[test]
    fn commit_advice_on_register_touch() {
        let f = Friction::new();
        let advice = f.commit_advice(&["TODO.md".to_string(), "src/x.rs".to_string()]);
        assert!(advice.unwrap().contains("register"));
    }

    #[test]
    fn no_commit_advice_when_clean() {
        let f = Friction::new();
        assert!(f.commit_advice(&[]).is_none());
    }

    #[test]
    fn tool_loop_after_repeated_identical_calls() {
        let mut f = Friction::new();
        let mut advice = Vec::new();
        for _ in 0..3 {
            advice = f.tool_friction(
                "edit",
                "{\"filePath\":\"src/x.rs\",\"newString\":\"same content\"}",
            );
        }
        assert!(advice.iter().any(|a| matches!(a, Advice::Loop)));
    }

    #[test]
    fn tool_churn_flagged() {
        let mut f = Friction::new();
        let mut advice = Vec::new();
        for i in 0..8 {
            advice = f.tool_friction("edit", &format!("content {}", i));
        }
        assert!(advice.iter().any(|a| matches!(a, Advice::Repetition)));
    }

    #[test]
    fn distinct_tools_stay_quiet() {
        let mut f = Friction::new();
        let advice = f.tool_friction("read", "one file");
        assert!(advice.is_empty());
    }
}
