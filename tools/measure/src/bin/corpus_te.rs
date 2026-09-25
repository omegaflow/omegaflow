use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

use omegaflow::te::{surrogate_stats_phase_n, transfer_entropy_lag};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAG: usize = 1;
const MAX_WINDOWS: usize = 256;
const MIN_WINDOWS: usize = 1;
const TE_FLOOR: usize = 9;
const DEFAULT_SURROGATES: usize = 100;
const FIXTURE_N: usize = 64;
const FIXTURE_SURROGATES: usize = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verdict {
    AB,
    BA,
    Both,
    NoFinding,
    Pending,
}

impl Verdict {
    fn as_str(self) -> &'static str {
        match self {
            Verdict::AB => "a -> b",
            Verdict::BA => "b -> a",
            Verdict::Both => "both",
            Verdict::NoFinding => "no finding",
            Verdict::Pending => "pending",
        }
    }
}

struct Direction {
    te: Option<f64>,
    threshold: Option<f64>,
    surr_mean: Option<f64>,
    surr_sd: Option<f64>,
}

struct FileSet {
    dir: String,
    files: Vec<PathBuf>,
    skipped: usize,
}

struct Corpus {
    dir: String,
    files: usize,
    skipped: usize,
    tokens: usize,
    series: Vec<f32>,
}

fn collect_files(dir: &Path, filter: Option<&str>, out: &mut Vec<PathBuf>) -> usize {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 1,
    };
    let mut skipped = 0usize;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            skipped += collect_files(&path, filter, out);
            continue;
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some("md") | Some("txt") => {}
            _ => continue,
        }
        if let Some(f) = filter {
            if !path.to_string_lossy().contains(f) {
                continue;
            }
        }
        out.push(path);
    }
    skipped
}

fn file_set(dir: &str, filter: Option<&str>) -> FileSet {
    let mut files = Vec::new();
    let skipped = collect_files(Path::new(dir), filter, &mut files);
    files.sort();
    FileSet {
        dir: dir.to_string(),
        files,
        skipped,
    }
}

fn corpus_from(set: &FileSet, windows: usize) -> Corpus {
    let mut counts: Vec<usize> = Vec::with_capacity(set.files.len());
    let mut skipped = set.skipped;
    for path in &set.files {
        match std::fs::read_to_string(path) {
            Ok(text) => counts.push(text.split_whitespace().count()),
            Err(_) => skipped += 1,
        }
    }
    let tokens: usize = counts.iter().sum();
    let series = window_token_counts(&counts, windows);
    Corpus {
        dir: set.dir.clone(),
        files: set.files.len(),
        skipped,
        tokens,
        series,
    }
}

fn window_token_counts(counts: &[usize], windows: usize) -> Vec<f32> {
    if counts.is_empty() {
        return Vec::new();
    }
    let per = counts.len() / windows;
    let extra = counts.len() % windows;
    let mut series = Vec::with_capacity(windows);
    let mut idx = 0usize;
    for w in 0..windows {
        let size = per + (w < extra) as usize;
        series.push(counts[idx..idx + size].iter().sum::<usize>() as f32);
        idx += size;
    }
    series
}

fn default_windows(file_count: usize) -> usize {
    let mut w = 1usize;
    while w * 2 <= file_count {
        w *= 2;
    }
    w.clamp(MIN_WINDOWS, MAX_WINDOWS)
}

fn pair_windows(requested: usize, files_a: usize, files_b: usize) -> usize {
    requested
        .min(files_a)
        .min(files_b)
        .clamp(MIN_WINDOWS, MAX_WINDOWS)
}

fn direction(target: &[f32], source: &[f32], surrogates: usize) -> Direction {
    let te = transfer_entropy_lag(target, source, LAG);
    match surrogate_stats_phase_n(target, source, LAG, SEED, surrogates) {
        Some((mean, sd, threshold)) => Direction {
            te,
            threshold: Some(threshold),
            surr_mean: Some(mean),
            surr_sd: Some(sd),
        },
        None => Direction {
            te,
            threshold: None,
            surr_mean: None,
            surr_sd: None,
        },
    }
}

fn verdict_of(dir_ab: &Direction, dir_ba: &Direction) -> Verdict {
    let (Some(te_ab), Some(thr_ab), Some(te_ba), Some(thr_ba)) =
        (dir_ab.te, dir_ab.threshold, dir_ba.te, dir_ba.threshold)
    else {
        return Verdict::Pending;
    };
    if te_ab.is_nan() || thr_ab.is_nan() || te_ba.is_nan() || thr_ba.is_nan() {
        return Verdict::Pending;
    }
    match (te_ab > thr_ab, te_ba > thr_ba) {
        (true, true) => Verdict::Both,
        (true, false) => Verdict::AB,
        (false, true) => Verdict::BA,
        (false, false) => Verdict::NoFinding,
    }
}

fn fmt_v(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.6}"),
        None => "pending".to_string(),
    }
}

fn print_direction(label: &str, d: &Direction) {
    let state = match (d.te, d.threshold) {
        (Some(te), Some(thr)) => {
            if te.is_nan() || thr.is_nan() {
                "pending"
            } else if te > thr {
                "significant"
            } else {
                "below threshold"
            }
        }
        _ => "pending",
    };
    println!(
        "{:<6} | {:>10} | {:>10} | {:>10} | {:>10} | {}",
        label,
        fmt_v(d.te),
        fmt_v(d.threshold),
        fmt_v(d.surr_mean),
        fmt_v(d.surr_sd),
        state
    );
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn json_number(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x}"),
        _ => "null".to_string(),
    }
}

fn json_direction(d: &Direction) -> String {
    format!(
        "{{\"te\":{},\"threshold\":{},\"surr_mean\":{},\"surr_sd\":{}}}",
        json_number(d.te),
        json_number(d.threshold),
        json_number(d.surr_mean),
        json_number(d.surr_sd)
    )
}

fn print_json(
    a: &Corpus,
    b: &Corpus,
    windows: usize,
    surrogates: usize,
    dir_ab: &Direction,
    dir_ba: &Direction,
    verdict: Verdict,
) {
    let series = |s: &[f32]| {
        s.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    };
    let seed_hex = format!("0x{SEED:X}");
    println!(
        "{{\"a\":\"{}\",\"b\":\"{}\",\"files_a\":{},\"files_b\":{},\"skipped_a\":{},\"skipped_b\":{},\"tokens_a\":{},\"tokens_b\":{},\"windows\":{},\"lag\":{},\"surrogates\":{},\"seed\":\"{}\",\"series_a\":[{}],\"series_b\":[{}],\"a_to_b\":{},\"b_to_a\":{},\"verdict\":\"{}\"}}",
        json_escape(&a.dir),
        json_escape(&b.dir),
        a.files,
        b.files,
        a.skipped,
        b.skipped,
        a.tokens,
        b.tokens,
        windows,
        LAG,
        surrogates,
        seed_hex,
        series(&a.series),
        series(&b.series),
        json_direction(dir_ab),
        json_direction(dir_ba),
        verdict.as_str()
    );
}

struct Fixture {
    name: &'static str,
    a: Vec<f32>,
    b: Vec<f32>,
    gate: Gate,
}

#[derive(Clone, Copy)]
enum Gate {
    DirectedAB,
    NoFinding,
    Symmetric,
}

struct FixtureReport {
    dir_ab: Direction,
    dir_ba: Direction,
    verdict: Verdict,
    pass: bool,
}

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn white(n: usize, rng: &mut u64) -> Vec<f32> {
    (0..n).map(|_| (next_rng(rng) * 2.0 - 1.0) as f32).collect()
}

fn fixtures() -> Vec<Fixture> {
    let mut rng = SEED;
    let a = white(FIXTURE_N, &mut rng);
    let mut coupled = vec![0.0f32; FIXTURE_N];
    for t in 0..FIXTURE_N {
        coupled[t] = if t == 0 {
            (next_rng(&mut rng) * 2.0 - 1.0) as f32
        } else {
            (0.9 * a[t - 1] as f64 + 0.1 * (next_rng(&mut rng) * 2.0 - 1.0)) as f32
        };
    }
    let independent = white(FIXTURE_N, &mut rng);
    let symmetric = a.clone();
    vec![
        Fixture {
            name: "coupled",
            a: a.clone(),
            b: coupled,
            gate: Gate::DirectedAB,
        },
        Fixture {
            name: "independent",
            a,
            b: independent,
            gate: Gate::NoFinding,
        },
        Fixture {
            name: "symmetric",
            a: symmetric.clone(),
            b: symmetric,
            gate: Gate::Symmetric,
        },
    ]
}

fn measure_fixture(f: &Fixture) -> FixtureReport {
    let dir_ab = direction(&f.b, &f.a, FIXTURE_SURROGATES);
    let dir_ba = direction(&f.a, &f.b, FIXTURE_SURROGATES);
    let verdict = verdict_of(&dir_ab, &dir_ba);
    let pass = match f.gate {
        Gate::DirectedAB => verdict == Verdict::AB,
        Gate::NoFinding => verdict == Verdict::NoFinding,
        Gate::Symmetric => {
            dir_ab.te == dir_ba.te && verdict != Verdict::AB && verdict != Verdict::BA
        }
    };
    FixtureReport {
        dir_ab,
        dir_ba,
        verdict,
        pass,
    }
}

fn run_fixtures() {
    let mut all_pass = true;
    for f in fixtures() {
        let r = measure_fixture(&f);
        println!(
            "fixture {:<11} te(a->b) {} thr {} | te(b->a) {} thr {} | verdict {:<10} {}",
            f.name,
            fmt_v(r.dir_ab.te),
            fmt_v(r.dir_ab.threshold),
            fmt_v(r.dir_ba.te),
            fmt_v(r.dir_ba.threshold),
            r.verdict.as_str(),
            if r.pass { "PASS" } else { "FAIL" }
        );
        all_pass &= r.pass;
    }
    if !all_pass {
        exit(2);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--fixture") {
        run_fixtures();
        return;
    }
    let opt = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .map(|s| s.as_str())
    };
    let (Some(a_dir), Some(b_dir)) = (opt("--a"), opt("--b")) else {
        eprintln!(
            "--a <dir> --b <dir> required; --windows <n> --surrogates <n> --filter <substr> --fixture"
        );
        exit(2);
    };
    let filter = opt("--filter");
    let requested: Option<usize> = opt("--windows").and_then(|v| v.parse::<usize>().ok());
    let surrogates: usize = match opt("--surrogates").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) => n,
        None => DEFAULT_SURROGATES,
    };

    let set_a = file_set(a_dir, filter);
    let set_b = file_set(b_dir, filter);
    if set_a.files.is_empty() {
        eprintln!("{a_dir}: no .md/.txt files found");
        exit(2);
    }
    if set_b.files.is_empty() {
        eprintln!("{b_dir}: no .md/.txt files found");
        exit(2);
    }

    let default = default_windows(set_a.files.len()).min(default_windows(set_b.files.len()));
    let wanted = match requested {
        Some(n) => n,
        None => default,
    };
    let windows = pair_windows(wanted, set_a.files.len(), set_b.files.len());
    if windows != wanted {
        println!(
            "windows clamped to {windows}: requested {wanted} > files (a={}, b={})",
            set_a.files.len(),
            set_b.files.len()
        );
    }

    let corpus_a = corpus_from(&set_a, windows);
    let corpus_b = corpus_from(&set_b, windows);

    println!(
        "corpus-te: TE over window token counts in sorted document order (structural volume coupling, lag {LAG})"
    );
    println!(
        "null: phase-randomized source, threshold = mean + 2 sd, {surrogates} surrogates, seed 0x{SEED:X}"
    );
    println!(
        "a: {}  files={} skipped={} tokens={}",
        corpus_a.dir, corpus_a.files, corpus_a.skipped, corpus_a.tokens
    );
    println!(
        "b: {}  files={} skipped={} tokens={}",
        corpus_b.dir, corpus_b.files, corpus_b.skipped, corpus_b.tokens
    );
    println!("windows={windows} surrogates={surrogates}");
    println!();
    println!(
        "{:<6} | {:>10} | {:>10} | {:>10} | {:>10} | {}",
        "arrow", "te", "threshold", "surr mean", "surr sd", "state"
    );

    let dir_ab;
    let dir_ba;
    let verdict;
    if windows < TE_FLOOR {
        dir_ab = Direction {
            te: None,
            threshold: None,
            surr_mean: None,
            surr_sd: None,
        };
        dir_ba = Direction {
            te: None,
            threshold: None,
            surr_mean: None,
            surr_sd: None,
        };
        verdict = Verdict::Pending;
        println!(
            "windows = {windows} < {TE_FLOOR}: the estimator floor — no TE (underdetermination, no fabrication)"
        );
    } else {
        dir_ab = direction(&corpus_b.series, &corpus_a.series, surrogates);
        dir_ba = direction(&corpus_a.series, &corpus_b.series, surrogates);
        verdict = verdict_of(&dir_ab, &dir_ba);
        print_direction("a -> b", &dir_ab);
        print_direction("b -> a", &dir_ba);
    }
    println!();
    println!("verdict: {}", verdict.as_str());
    println!();
    print_json(
        &corpus_a, &corpus_b, windows, surrogates, &dir_ab, &dir_ba, verdict,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coupled_series_yield_directed_finding() {
        let all = fixtures();
        let r = measure_fixture(&all[0]);
        assert!(r.pass, "coupled fixture: verdict {}", r.verdict.as_str());
        assert_eq!(r.verdict, Verdict::AB);
    }

    #[test]
    fn independent_series_yield_no_finding() {
        let all = fixtures();
        let r = measure_fixture(&all[1]);
        assert!(
            r.pass,
            "independent fixture: verdict {}",
            r.verdict.as_str()
        );
        assert_eq!(r.verdict, Verdict::NoFinding);
    }

    #[test]
    fn identical_series_are_symmetric() {
        let all = fixtures();
        let r = measure_fixture(&all[2]);
        assert_eq!(r.dir_ab.te, r.dir_ba.te);
        assert!(r.pass, "symmetric fixture: verdict {}", r.verdict.as_str());
    }

    #[test]
    fn window_partition_preserves_the_token_chain() {
        let counts = [3usize, 5, 2, 8, 1, 4];
        let series = window_token_counts(&counts, 4);
        assert_eq!(series, vec![8.0f32, 10.0, 1.0, 4.0]);
        let total: f32 = series.iter().sum();
        assert_eq!(total, 23.0);
    }

    #[test]
    fn default_windows_is_the_power_of_two_near_the_file_count() {
        assert_eq!(default_windows(33), 32);
        assert_eq!(default_windows(174), 128);
        assert_eq!(default_windows(44), 32);
        assert_eq!(default_windows(4), 4);
        assert_eq!(default_windows(300), 256);
    }

    #[test]
    fn pair_windows_never_exceeds_a_file_count() {
        assert_eq!(pair_windows(32, 174, 33), 32);
        assert_eq!(pair_windows(128, 174, 33), 33);
        assert_eq!(pair_windows(64, 10, 20), 10);
    }
}
