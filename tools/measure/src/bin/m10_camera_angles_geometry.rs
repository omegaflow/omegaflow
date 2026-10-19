use std::env;
use std::fmt::Write as _;
use std::fs;
use std::process::exit;

use omegaflow::archivar::camera_angles::{CameraAngleFrame, parse_camera_angles};

const DEFAULT_IN: &str = "zustand/research/m10-camera-angles/camera_angles.txt";
const UNVERIFIED_RATIO: f64 = 2.0;

struct Span {
    min: f64,
    max: f64,
    sum: f64,
    n: usize,
}

impl Span {
    fn new() -> Self {
        Span {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: 0.0,
            n: 0,
        }
    }

    fn push(&mut self, v: f64) {
        if v < self.min {
            self.min = v;
        }
        if v > self.max {
            self.max = v;
        }
        self.sum += v;
        self.n += 1;
    }

    fn mean(&self) -> f64 {
        self.sum / self.n as f64
    }

    fn ratio(&self) -> f64 {
        self.max / self.min
    }
}

fn magnitude(f: &CameraAngleFrame) -> f64 {
    (f.sx * f.sx + f.sy * f.sy + f.sz * f.sz).sqrt()
}

fn main() {
    let mut in_path = DEFAULT_IN.to_string();
    let mut out_path: Option<String> = None;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--in" => {
                i += 1;
                match args.get(i) {
                    Some(v) => in_path = v.clone(),
                    None => {
                        eprintln!("--in requires a path");
                        exit(2);
                    }
                }
            }
            "--out" => {
                i += 1;
                match args.get(i) {
                    Some(v) => out_path = Some(v.clone()),
                    None => {
                        eprintln!("--out requires a name");
                        exit(2);
                    }
                }
            }
            other => {
                eprintln!("unknown argument: {other}");
                exit(2);
            }
        }
        i += 1;
    }

    let text = match fs::read_to_string(&in_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("input not readable: {in_path}: {e}");
            exit(2);
        }
    };

    let frames = parse_camera_angles(&text);
    if frames.is_empty() {
        eprintln!("no camera-angle records parsed from {in_path}");
        exit(2);
    }

    let mut ordered = frames;
    ordered.sort_by(|a, b| a.jd.total_cmp(&b.jd));

    let jd_min = ordered[0].jd;
    let jd_max = ordered[ordered.len() - 1].jd;

    let mut overall = Span::new();
    for f in &ordered {
        overall.push(magnitude(f));
    }

    let gaps: Vec<f64> = ordered
        .windows(2)
        .map(|w| w[1].jd - w[0].jd)
        .filter(|g| *g > 0.0)
        .collect();

    let max_gap = gaps.iter().copied().fold(0.0f64, f64::max);
    let mut sorted_gaps = gaps.clone();
    sorted_gaps.sort_by(|a, b| a.total_cmp(b));
    let median_gap = if sorted_gaps.is_empty() {
        0.0
    } else {
        sorted_gaps[sorted_gaps.len() / 2]
    };

    const SEP_FACTOR: f64 = 1000.0;
    let has_separator = !gaps.is_empty() && max_gap >= SEP_FACTOR * median_gap;

    let (threshold, max_intra, min_inter) = if has_separator {
        let t = max_gap / 2.0;
        let intra = gaps
            .iter()
            .copied()
            .filter(|g| *g < t)
            .fold(0.0f64, f64::max);
        let inter = gaps
            .iter()
            .copied()
            .filter(|g| *g >= t)
            .fold(f64::INFINITY, f64::min);
        (t, intra, inter)
    } else {
        (f64::INFINITY, max_gap, 0.0)
    };

    let mut clusters: Vec<Vec<CameraAngleFrame>> = Vec::new();
    let mut current: Vec<CameraAngleFrame> = Vec::new();
    for f in ordered {
        if let Some(prev) = current.last() {
            if f.jd - prev.jd > threshold {
                clusters.push(std::mem::take(&mut current));
            }
        }
        current.push(f);
    }
    if !current.is_empty() {
        clusters.push(current);
    }

    let mut out = String::new();
    let _ = writeln!(out, "m10 camera angles — geometry report");
    let _ = writeln!(out, "source: {in_path}");
    let _ = writeln!(out, "records parsed: {}", overall.n);
    let _ = writeln!(out, "jd span: {jd_min:.6} .. {jd_max:.6}");
    let _ = writeln!(
        out,
        "gap measurement: largest gap {max_gap:.6} d, median positive gap {median_gap:.6} d, ratio {:.1}x (separator declared at >= {SEP_FACTOR:.0}x)",
        if median_gap > 0.0 {
            max_gap / median_gap
        } else {
            0.0
        }
    );
    if has_separator {
        let _ = writeln!(
            out,
            "cluster gap threshold: {threshold:.6} d (max intra-cluster gap {max_intra:.6} d, min inter-cluster gap {min_inter:.6} d)"
        );
    } else {
        let _ = writeln!(
            out,
            "cluster gap threshold: none — no encounter separator measured; the file is one cluster"
        );
    }
    let _ = writeln!(
        out,
        "|S| overall km: min {:.3}  max {:.3}  ratio {:.2}x over {} frames",
        overall.min,
        overall.max,
        overall.ratio(),
        overall.n
    );
    if overall.ratio() > UNVERIFIED_RATIO {
        let _ = writeln!(
            out,
            "S-FRAME VERDICT: UNVERIFIED — |S| spans {:.3}..{:.3} km (ratio {:.1}x), not a single range; a target-centred frame holds |S| near-constant. The SXSYSZ origin is not confirmed target-centred; the per-encounter |S| values are measured but the frame is unverified.",
            overall.min,
            overall.max,
            overall.ratio()
        );
    } else {
        let _ = writeln!(
            out,
            "S-FRAME VERDICT: range — |S| spans {:.3}..{:.3} km (ratio {:.2}x), consistent with a target-centred frame within the {UNVERIFIED_RATIO:.1}x tolerance.",
            overall.min,
            overall.max,
            overall.ratio()
        );
    }

    for (idx, cluster) in clusters.iter().enumerate() {
        let mut s_span = Span::new();
        let mut c1 = Span::new();
        let mut c2 = Span::new();
        let mut c3 = Span::new();
        let mut s_min_frame: Option<(f64, String, f64)> = None;
        let mut fds_min: Option<(u64, String)> = None;
        let mut fds_max: Option<(u64, String)> = None;
        let mut jd_lo = cluster[0].jd;
        let mut jd_hi = cluster[0].jd;

        for f in cluster {
            let m = magnitude(f);
            s_span.push(m);
            c1.push(f.c1);
            c2.push(f.c2);
            c3.push(f.c3);
            if f.jd < jd_lo {
                jd_lo = f.jd;
            }
            if f.jd > jd_hi {
                jd_hi = f.jd;
            }
            if s_min_frame.as_ref().map_or(true, |(_, _, mv)| m < *mv) {
                s_min_frame = Some((f.jd, f.fds.clone(), m));
            }
            if let Ok(v) = f.fds.parse::<u64>() {
                if fds_min.as_ref().map_or(true, |(mv, _)| v < *mv) {
                    fds_min = Some((v, f.fds.clone()));
                }
                if fds_max.as_ref().map_or(true, |(mv, _)| v > *mv) {
                    fds_max = Some((v, f.fds.clone()));
                }
            }
        }

        let fds_min_s = match &fds_min {
            Some((_, s)) => s.clone(),
            None => "n/a".to_string(),
        };
        let fds_max_s = match &fds_max {
            Some((_, s)) => s.clone(),
            None => "n/a".to_string(),
        };

        let _ = writeln!(out, "");
        let _ = writeln!(out, "encounter {}", idx + 1);
        let _ = writeln!(out, "  frames: {}", cluster.len());
        let _ = writeln!(out, "  jd: {jd_lo:.6} .. {jd_hi:.6}");
        let _ = writeln!(out, "  fds: {fds_min_s} .. {fds_max_s}");
        let flag = if s_span.ratio() > UNVERIFIED_RATIO {
            " [unverified frame]"
        } else {
            " [range]"
        };
        let _ = writeln!(
            out,
            "  |S| km: min {:.3}  mean {:.3}  max {:.3}  (spread {:.2}x){flag}",
            s_span.min,
            s_span.mean(),
            s_span.max,
            s_span.ratio()
        );
        if let Some((jd, fds, m)) = &s_min_frame {
            let _ = writeln!(out, "  |S| min frame: JD {jd:.6}  FDS {fds}  |S| {m:.3} km");
        }
        let _ = writeln!(out, "  c1: {:.6} .. {:.6}", c1.min, c1.max);
        let _ = writeln!(out, "  c2: {:.6} .. {:.6}", c2.min, c2.max);
        let _ = writeln!(out, "  c3: {:.6} .. {:.6}", c3.min, c3.max);
    }

    print!("{out}");

    if let Some(path) = out_path {
        if let Err(e) = fs::write(&path, &out) {
            eprintln!("output not writable: {path}: {e}");
            exit(2);
        }
    }
}
