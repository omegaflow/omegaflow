use super::*;
use crate::weberin::BodyLine;

pub const VERDICT_TAG: u8 = 11;
pub const VERDICT_STALE_S: u64 = 604800;
const MAGIC: [u8; 2] = [0xCF, 0x86];
const NO_LINE: u8 = 0xFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerdictWord {
    Placed,
    Absent,
    DirectionOnly,
    Riss,
}

impl VerdictWord {
    pub fn word(self) -> &'static str {
        match self {
            VerdictWord::Placed => "placed",
            VerdictWord::Absent => "absent",
            VerdictWord::DirectionOnly => "direction-only",
            VerdictWord::Riss => "riss",
        }
    }

    pub fn code(self) -> u8 {
        match self {
            VerdictWord::Placed => 0,
            VerdictWord::Absent => 1,
            VerdictWord::DirectionOnly => 2,
            VerdictWord::Riss => 3,
        }
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(VerdictWord::Placed),
            1 => Some(VerdictWord::Absent),
            2 => Some(VerdictWord::DirectionOnly),
            3 => Some(VerdictWord::Riss),
            _ => None,
        }
    }
}

pub fn line_code(line: BodyLine) -> u8 {
    match line {
        BodyLine::Spk => 0,
        BodyLine::Dastcom => 1,
        BodyLine::Mpc => 2,
        BodyLine::Inpop => 3,
        BodyLine::Epm => 4,
    }
}

pub fn line_from_code(code: u8) -> Option<BodyLine> {
    match code {
        0 => Some(BodyLine::Spk),
        1 => Some(BodyLine::Dastcom),
        2 => Some(BodyLine::Mpc),
        3 => Some(BodyLine::Inpop),
        4 => Some(BodyLine::Epm),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VerdictLine {
    pub name: String,
    pub word: VerdictWord,
    pub knot: [Option<BodyLine>; 2],
    pub sep_m: Option<f64>,
    pub weave_epoch: f64,
}

pub fn parse_weberin_verdicts(bytes: &[u8]) -> Vec<VerdictLine> {
    if bytes.len() < 7 || bytes[0] != MAGIC[0] || bytes[1] != MAGIC[1] || bytes[2] != VERDICT_TAG {
        return Vec::new();
    }
    let mut off = 3;
    let Some(count) = take_u32(bytes, &mut off) else {
        return Vec::new();
    };
    let mut lines = Vec::new();
    for _ in 0..count {
        let Some(name_len) = bytes.get(off).copied() else {
            break;
        };
        off += 1;
        let end = off + name_len as usize;
        let Some(name_bytes) = bytes.get(off..end) else {
            break;
        };
        let name = String::from_utf8_lossy(name_bytes).to_string();
        off = end;
        let Some(&word_code) = bytes.get(off) else {
            break;
        };
        off += 1;
        let Some(word) = VerdictWord::from_code(word_code) else {
            break;
        };
        let (Some(&ka), Some(&kb)) = (bytes.get(off), bytes.get(off + 1)) else {
            break;
        };
        off += 2;
        let knot = [line_from_code(ka), line_from_code(kb)];
        let Some(&sep_flag) = bytes.get(off) else {
            break;
        };
        off += 1;
        let sep_m = if sep_flag == 1 {
            match take_f64(bytes, &mut off) {
                Some(v) => Some(v),
                None => break,
            }
        } else {
            None
        };
        let Some(weave_epoch) = take_f64(bytes, &mut off) else {
            break;
        };
        lines.push(VerdictLine {
            name,
            word,
            knot,
            sep_m,
            weave_epoch,
        });
    }
    lines
}

pub fn encode_weberin_verdicts(lines: &[VerdictLine]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.push(VERDICT_TAG);
    out.extend_from_slice(&(lines.len() as u32).to_le_bytes());
    for line in lines {
        let name = line.name.as_bytes();
        out.push(name.len().min(255) as u8);
        out.extend_from_slice(&name[..name.len().min(255)]);
        out.push(line.word.code());
        out.push(line.knot[0].map(line_code).unwrap_or(NO_LINE));
        out.push(line.knot[1].map(line_code).unwrap_or(NO_LINE));
        match line.sep_m {
            Some(sep) if sep.is_finite() => {
                out.push(1);
                out.extend_from_slice(&sep.to_le_bytes());
            }
            _ => out.push(0),
        }
        out.extend_from_slice(&line.weave_epoch.to_le_bytes());
    }
    out
}

pub fn load_weberin_verdicts(path: &str) -> Vec<VerdictLine> {
    match std::fs::read(path) {
        Ok(bytes) => parse_weberin_verdicts(&bytes),
        Err(_) => Vec::new(),
    }
}

pub fn riss_bodies(lines: &[VerdictLine]) -> Vec<&VerdictLine> {
    lines.iter().filter(|l| l.word == VerdictWord::Riss).collect()
}

pub fn is_stale(line: &VerdictLine, now_tdb: Option<f64>) -> bool {
    match now_tdb {
        Some(now) => now - line.weave_epoch >= VERDICT_STALE_S as f64,
        None => false,
    }
}

pub fn live_verdicts(lines: &[VerdictLine], now_tdb: Option<f64>) -> Vec<VerdictLine> {
    lines
        .iter()
        .filter(|line| !is_stale(line, now_tdb))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<VerdictLine> {
        vec![
            VerdictLine {
                name: "ceres".into(),
                word: VerdictWord::Placed,
                knot: [None, None],
                sep_m: Some(2.5e4),
                weave_epoch: 8.0e8,
            },
            VerdictLine {
                name: "apophis".into(),
                word: VerdictWord::Riss,
                knot: [Some(BodyLine::Spk), Some(BodyLine::Inpop)],
                sep_m: Some(2.3e9),
                weave_epoch: 8.0e8,
            },
            VerdictLine {
                name: "vesta".into(),
                word: VerdictWord::Absent,
                knot: [Some(BodyLine::Mpc), None],
                sep_m: None,
                weave_epoch: 8.0e8,
            },
        ]
    }

    #[test]
    fn the_wire_round_trips_every_word() {
        let lines = sample();
        let bytes = encode_weberin_verdicts(&lines);
        assert_eq!(bytes[0], 0xCF);
        assert_eq!(bytes[1], 0x86);
        assert_eq!(bytes[2], VERDICT_TAG);
        assert_eq!(parse_weberin_verdicts(&bytes), lines);
    }

    #[test]
    fn a_riss_keeps_its_name_and_both_witness_lines() {
        let bytes = encode_weberin_verdicts(&sample());
        let back = parse_weberin_verdicts(&bytes);
        let apophis = back.iter().find(|l| l.name == "apophis").expect("apophis");
        assert_eq!(apophis.word, VerdictWord::Riss);
        assert_eq!(apophis.knot, [Some(BodyLine::Spk), Some(BodyLine::Inpop)]);
        assert_eq!(apophis.sep_m, Some(2.3e9));
    }

    #[test]
    fn an_absent_sep_is_not_a_fabricated_zero() {
        let lines = sample();
        let back = parse_weberin_verdicts(&encode_weberin_verdicts(&lines));
        let vesta = back.iter().find(|l| l.name == "vesta").expect("vesta");
        assert_eq!(vesta.sep_m, None);
        assert_eq!(vesta.word, VerdictWord::Absent);
    }

    #[test]
    fn a_void_or_foreign_blob_reads_no_verdict() {
        assert!(parse_weberin_verdicts(&[]).is_empty());
        assert!(parse_weberin_verdicts(&[0xCF, 0x86, 9, 0, 0, 0, 0]).is_empty());
        assert!(parse_weberin_verdicts(b"not a verdict bin").is_empty());
    }

    #[test]
    fn the_riss_bodies_are_the_named_contradictions() {
        let lines = sample();
        let riss = riss_bodies(&lines);
        assert_eq!(riss.len(), 1);
        assert_eq!(riss[0].name, "apophis");
    }

    #[test]
    fn a_verdict_ages_into_stale_at_the_window() {
        let line = &sample()[0];
        let weave = line.weave_epoch;
        assert!(!is_stale(line, Some(weave + 100.0)));
        assert!(!is_stale(line, Some(weave + VERDICT_STALE_S as f64 - 1.0)));
        assert!(is_stale(line, Some(weave + VERDICT_STALE_S as f64)));
        assert!(is_stale(line, Some(weave + VERDICT_STALE_S as f64 + 1.0)));
    }

    #[test]
    fn an_absent_now_is_not_a_fabricated_stale() {
        assert!(!is_stale(&sample()[0], None));
    }

    #[test]
    fn a_future_weave_is_not_stale() {
        let line = &sample()[0];
        assert!(!is_stale(line, Some(line.weave_epoch - 100.0)));
    }

    #[test]
    fn live_verdicts_drops_only_the_expired_lines() {
        let mut lines = sample();
        let now = 8.0e8 + VERDICT_STALE_S as f64;
        lines[1].weave_epoch = now;
        let live = live_verdicts(&lines, Some(now));
        assert_eq!(live.len(), 2);
        assert!(live.iter().all(|l| l.name != "apophis"));
        assert_eq!(live_verdicts(&lines, None).len(), 3);
    }
}
