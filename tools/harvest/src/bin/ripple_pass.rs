use std::collections::BTreeMap;
use std::env;
use std::fs;

const USAGE: &str = "usage: ripple_pass --descriptor <file.te>\n       ripple_pass --bin <seconds> --series <name=path> [--series ...] [--separation <bins>] [--edge <bins>] [--keep <n>] [--min-bins <n>] [--epoch <iso>]";

#[derive(Clone, Debug)]
pub struct Series {
    pub name: String,
    pub bins: BTreeMap<i64, f64>,
    pub source: String,
}

#[derive(Clone, Debug)]
pub struct Spec {
    pub bin_seconds: i64,
    pub series: Vec<(String, String)>,
    pub channels: Vec<String>,
    pub separation: usize,
    pub edge: usize,
    pub keep: usize,
    pub min_bins: usize,
    pub epoch: Option<f64>,
}

impl Default for Spec {
    fn default() -> Self {
        Spec {
            bin_seconds: 3600,
            series: Vec::new(),
            channels: Vec::new(),
            separation: 1,
            edge: 2,
            keep: 3,
            min_bins: 10,
            epoch: None,
        }
    }
}

pub fn parse_descriptor(text: &str) -> Result<Spec, String> {
    let mut spec = Spec::default();
    let mut seen_mode = false;
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let at = index + 1;
        match parts[0] {
            "ripple" => {
                if parts.len() != 3 || parts[1] != "pass" {
                    return Err(format!(
                        "descriptor:{at}: ripple expects 'ripple pass <seconds>'"
                    ));
                }
                spec.bin_seconds = parts[2]
                    .parse()
                    .map_err(|_| format!("descriptor:{at}: '{}' carries no seconds", parts[2]))?;
                seen_mode = true;
            }
            "series" => {
                if parts.len() != 3 {
                    return Err(format!(
                        "descriptor:{at}: series expects 'series <name> <path>'"
                    ));
                }
                spec.series
                    .push((parts[1].to_string(), parts[2].to_string()));
            }
            "channels" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: channels carries no list"))?;
                for name in token.split(',') {
                    let name = name.trim();
                    if !name.is_empty() {
                        spec.channels.push(name.to_string());
                    }
                }
            }
            "separation" => {
                spec.separation = parse_usize(parts.get(1), at, "separation")?;
            }
            "edge" => {
                spec.edge = parse_usize(parts.get(1), at, "edge")?;
            }
            "keep" => {
                spec.keep = parse_usize(parts.get(1), at, "keep")?;
            }
            "min-bins" => {
                spec.min_bins = parse_usize(parts.get(1), at, "min-bins")?;
            }
            "epoch" => {
                let token = parts
                    .get(1)
                    .ok_or_else(|| format!("descriptor:{at}: epoch carries no time"))?;
                spec.epoch = omegaflow::archivar::parse_iso_unix(token);
                if spec.epoch.is_none() {
                    return Err(format!("descriptor:{at}: epoch '{token}' does not parse"));
                }
            }
            other => return Err(format!("descriptor:{at}: '{other}' names no directive")),
        }
    }
    if !seen_mode {
        return Err("descriptor carries no 'ripple pass <seconds>' line".to_string());
    }
    if spec.series.is_empty() {
        return Err("descriptor carries no series".to_string());
    }
    if spec.bin_seconds <= 0 {
        return Err("bin must be positive".to_string());
    }
    Ok(spec)
}

fn parse_usize(value: Option<&&str>, at: usize, name: &str) -> Result<usize, String> {
    let token = value.ok_or_else(|| format!("descriptor:{at}: {name} carries no number"))?;
    token
        .parse()
        .map_err(|_| format!("descriptor:{at}: {name} '{token}' carries no number"))
}

pub fn load_series(name: &str, path: &str, bin_seconds: i64, epoch: f64) -> Result<Series, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let mut buckets: BTreeMap<i64, (f64, u32)> = BTreeMap::new();
    let mut unparsed = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split_whitespace().collect();
        if cells.len() < 2 {
            unparsed += 1;
            continue;
        }
        let (Some(time), Ok(value)) = (
            omegaflow::archivar::parse_iso_unix(cells[0]),
            cells[1].parse::<f64>(),
        ) else {
            unparsed += 1;
            continue;
        };
        if !value.is_finite() {
            continue;
        }
        let bin = ((time - epoch) / bin_seconds as f64).floor() as i64;
        let entry = buckets.entry(bin).or_insert((0.0, 0));
        entry.0 += value;
        entry.1 += 1;
    }
    if buckets.is_empty() {
        return Err(format!("{path} carries no row with a time and a value"));
    }
    let bins = buckets
        .into_iter()
        .map(|(k, (sum, n))| (k, sum / n as f64))
        .collect();
    if unparsed > 0 {
        eprintln!("ripple_pass: {name} skipped {unparsed} row(s) without a time and a value");
    }
    Ok(Series {
        name: name.to_string(),
        bins,
        source: path.to_string(),
    })
}

#[derive(Clone, Copy)]
struct Fit {
    slope: f64,
    n: usize,
}

struct Moments {
    n: Vec<f64>,
    x: Vec<f64>,
    y: Vec<f64>,
    xx: Vec<f64>,
    xy: Vec<f64>,
}

impl Moments {
    fn build(xs: &[f64], ys: &[f64]) -> Self {
        let len = xs.len();
        let mut m = Moments {
            n: vec![0.0; len + 1],
            x: vec![0.0; len + 1],
            y: vec![0.0; len + 1],
            xx: vec![0.0; len + 1],
            xy: vec![0.0; len + 1],
        };
        for i in 0..len {
            m.n[i + 1] = m.n[i] + 1.0;
            m.x[i + 1] = m.x[i] + xs[i];
            m.y[i + 1] = m.y[i] + ys[i];
            m.xx[i + 1] = m.xx[i] + xs[i] * xs[i];
            m.xy[i + 1] = m.xy[i] + xs[i] * ys[i];
        }
        m
    }

    fn fit(&self, lo: usize, hi: usize) -> Option<Fit> {
        let n = self.n[hi] - self.n[lo];
        if n < 4.0 {
            return None;
        }
        let sx = self.x[hi] - self.x[lo];
        let sy = self.y[hi] - self.y[lo];
        let sxx = self.xx[hi] - self.xx[lo];
        let sxy = self.xy[hi] - self.xy[lo];
        let den = n * sxx - sx * sx;
        let scale = (n * sxx).abs() + (sx * sx).abs();
        if scale == 0.0 || den.abs() <= 1e-12 * scale {
            return None;
        }
        Some(Fit {
            slope: (n * sxy - sx * sy) / den,
            n: n as usize,
        })
    }
}

#[derive(Clone, Debug)]
struct Step {
    bin: i64,
    ratio: f64,
    score: f64,
}

fn changepoints(
    xs: &[f64],
    ys: &[f64],
    bins: &[i64],
    separation: usize,
    edge: usize,
    keep: usize,
) -> Vec<Step> {
    let moments = Moments::build(xs, ys);
    let mut found: Vec<Step> = Vec::new();
    for split in edge..bins.len().saturating_sub(edge) {
        let (Some(before), Some(after)) = (moments.fit(0, split), moments.fit(split, bins.len()))
        else {
            continue;
        };
        if before.slope <= 0.0 || after.slope <= 0.0 {
            continue;
        }
        let ratio = after.slope / before.slope;
        let weight = (before.n.min(after.n) as f64).sqrt();
        found.push(Step {
            bin: bins[split],
            ratio,
            score: ratio.ln().abs() * weight,
        });
    }
    found.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut kept: Vec<Step> = Vec::new();
    for step in found {
        if kept
            .iter()
            .any(|k| (k.bin - step.bin).unsigned_abs() as usize <= separation)
        {
            continue;
        }
        kept.push(step);
        if kept.len() == keep {
            break;
        }
    }
    kept
}

pub fn format_time(unix: f64, bin_seconds: i64, epoch: f64) -> String {
    let seconds = epoch + unix as f64 * bin_seconds as f64;
    let days = (seconds / 86_400.0).floor() as i64;
    let rest = seconds - days as f64 * 86_400.0;
    match omegaflow::spectral::civil_from_days(days) {
        Some((year, month, day)) => {
            let hour = (rest / 3600.0).floor() as i64;
            let minute = ((rest - hour as f64 * 3600.0) / 60.0).floor() as i64;
            let second = (rest - hour as f64 * 3600.0 - minute as f64 * 60.0).floor() as i64;
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
        }
        None => format!("bin {unix}"),
    }
}

fn run(spec: &Spec) -> Result<(), String> {
    let mut loaded: Vec<Series> = Vec::new();
    let mut readable: Vec<(&String, &String, String)> = Vec::new();
    for (name, path) in &spec.series {
        match fs::read_to_string(path) {
            Ok(text) => readable.push((name, path, text)),
            Err(reason) => {
                println!("source {name:<20} pending: read {path} returned void: {reason}")
            }
        }
    }
    for channel in &spec.channels {
        if !spec.series.iter().any(|(name, _)| name == channel) {
            println!("source {channel:<20} pending: no series declared");
        }
    }
    if readable.is_empty() {
        println!("ripple: pending — no declared series carries a file");
        return Ok(());
    }
    let epoch = match spec.epoch {
        Some(value) => value,
        None => {
            let mut first: Option<f64> = None;
            for (_, _, text) in &readable {
                for line in text.lines() {
                    let cells: Vec<&str> = line.split_whitespace().collect();
                    if cells.len() < 2 {
                        continue;
                    }
                    if let Some(time) = omegaflow::archivar::parse_iso_unix(cells[0]) {
                        first = Some(first.map_or(time, |f: f64| f.min(time)));
                        break;
                    }
                }
            }
            match first {
                Some(value) => value,
                None => {
                    println!("ripple: pending — no readable series carries a parseable time");
                    return Ok(());
                }
            }
        }
    };
    for (name, path, _) in &readable {
        match load_series(name, path, spec.bin_seconds, epoch) {
            Ok(series) => loaded.push(series),
            Err(reason) => println!("source {name:<20} pending: {reason}"),
        }
    }
    if loaded.len() < 2 {
        println!(
            "ripple: pending — {} of {} declared series carry data, a pair needs two",
            loaded.len(),
            spec.channels.len().max(spec.series.len())
        );
        return Ok(());
    }
    println!(
        "ripple_pass: {} series, bin {} s, separation {} bins, edge {} bins, keep {}, min-bins {}",
        loaded.len(),
        spec.bin_seconds,
        spec.separation,
        spec.edge,
        spec.keep,
        spec.min_bins
    );
    for series in &loaded {
        let span = match (series.bins.keys().next(), series.bins.keys().next_back()) {
            (Some(a), Some(b)) => format!(
                "{} .. {}",
                format_time(*a as f64, spec.bin_seconds, epoch),
                format_time(*b as f64, spec.bin_seconds, epoch)
            ),
            _ => "absent".to_string(),
        };
        println!(
            "source {:<20} {:>5} bins  {}  ({})",
            series.name,
            series.bins.len(),
            span,
            series.source
        );
    }

    let mut all_steps: Vec<(String, Step)> = Vec::new();
    for a in 0..loaded.len() {
        for b in (a + 1)..loaded.len() {
            let common: Vec<i64> = loaded[a]
                .bins
                .keys()
                .filter(|k| loaded[b].bins.contains_key(k))
                .copied()
                .collect();
            if common.len() < spec.min_bins {
                println!(
                    "pair {:<20} ~ {:<20} common bins {:>5}  below {}",
                    loaded[a].name,
                    loaded[b].name,
                    common.len(),
                    spec.min_bins
                );
                continue;
            }
            let xs: Vec<f64> = common.iter().map(|k| loaded[b].bins[k]).collect();
            let ys: Vec<f64> = common.iter().map(|k| loaded[a].bins[k]).collect();
            let steps = changepoints(&xs, &ys, &common, spec.separation, spec.edge, spec.keep);
            if steps.is_empty() {
                println!(
                    "pair {:<20} ~ {:<20} common bins {:>5}  absent: no split carries a positive slope on both sides",
                    loaded[a].name,
                    loaded[b].name,
                    common.len()
                );
                continue;
            }
            let rendered: Vec<String> = steps
                .iter()
                .map(|s| {
                    format!(
                        "{} {:+.1}%",
                        format_time(s.bin as f64, spec.bin_seconds, epoch),
                        100.0 * (s.ratio - 1.0)
                    )
                })
                .collect();
            println!(
                "pair {:<20} ~ {:<20} common bins {:>5}  top {}",
                loaded[a].name,
                loaded[b].name,
                common.len(),
                rendered.join("  ")
            );
            for step in steps {
                all_steps.push((format!("{}~{}", loaded[a].name, loaded[b].name), step));
            }
        }
    }

    if all_steps.is_empty() {
        println!("ripple: no pair carries a split — nothing moves together");
        return Ok(());
    }
    let mut ordered = all_steps.clone();
    ordered.sort_by(|a, b| a.1.bin.cmp(&b.1.bin).then(a.0.cmp(&b.0)));
    let mut clusters: Vec<(i64, Vec<(String, Step)>)> = Vec::new();
    for (name, step) in ordered {
        match clusters.last_mut() {
            Some((anchor, members))
                if (step.bin - *anchor).unsigned_abs() as usize <= spec.separation =>
            {
                members.push((name, step));
            }
            _ => clusters.push((step.bin, vec![(name, step)])),
        }
    }
    println!(
        "ripple: bins where more than one pair steps within {} bin(s)",
        spec.separation
    );
    let mut any = false;
    for (anchor, members) in &clusters {
        let mut names: Vec<&str> = Vec::new();
        for (name, _) in members {
            if !names.contains(&name.as_str()) {
                names.push(name.as_str());
            }
        }
        if names.len() < 2 {
            continue;
        }
        any = true;
        println!(
            "  {}  pairs {}  steps {}",
            format_time(*anchor as f64, spec.bin_seconds, epoch),
            names.len(),
            members.len()
        );
        for (name, step) in members {
            println!("    {:<24} {:+.1}%", name, 100.0 * (step.ratio - 1.0));
        }
    }
    if !any {
        println!("  absent: steps exist, no bin carries more than one pair");
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut spec = Spec::default();
    let mut descriptor: Option<String> = None;
    let mut bin_given = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--descriptor" => {
                descriptor = Some(
                    args.get(index + 1)
                        .ok_or_else(|| USAGE.to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--bin" => {
                spec.bin_seconds = args
                    .get(index + 1)
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| "bin carries no number".to_string())?;
                bin_given = true;
                index += 2;
            }
            "--series" => {
                let token = args.get(index + 1).ok_or_else(|| USAGE.to_string())?;
                let (name, path) = token
                    .split_once('=')
                    .ok_or_else(|| format!("--series {token} expects name=path"))?;
                spec.series.push((name.to_string(), path.to_string()));
                index += 2;
            }
            "--separation" => {
                spec.separation = args
                    .get(index + 1)
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| "separation carries no number".to_string())?;
                index += 2;
            }
            "--edge" => {
                spec.edge = args
                    .get(index + 1)
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| "edge carries no number".to_string())?;
                index += 2;
            }
            "--keep" => {
                spec.keep = args
                    .get(index + 1)
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| "keep carries no number".to_string())?;
                index += 2;
            }
            "--min-bins" => {
                spec.min_bins = args
                    .get(index + 1)
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| "min-bins carries no number".to_string())?;
                index += 2;
            }
            "--epoch" => {
                let token = args.get(index + 1).ok_or_else(|| USAGE.to_string())?;
                spec.epoch = omegaflow::archivar::parse_iso_unix(token);
                if spec.epoch.is_none() {
                    return Err(format!("epoch '{token}' does not parse"));
                }
                index += 2;
            }
            other => return Err(format!("{other} names no option; {USAGE}")),
        }
    }
    if let Some(path) = descriptor {
        let text = fs::read_to_string(&path)
            .map_err(|e| format!("descriptor {path} returned void: {e}"))?;
        let from_file = parse_descriptor(&text)?;
        if bin_given {
            return Err("descriptor and --bin name one run — one form".to_string());
        }
        return run(&from_file);
    }
    if spec.series.is_empty() {
        return Err(USAGE.to_string());
    }
    run(&spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series(name: &str, pairs: &[(i64, f64)]) -> Series {
        Series {
            name: name.to_string(),
            bins: pairs.iter().copied().collect(),
            source: "test".to_string(),
        }
    }

    #[test]
    fn a_descriptor_carries_its_directives() {
        let text = "# a comment\nripple pass 3600\nseries a /tmp/a.txt\nseries b /tmp/b.txt\nseparation 2\nedge 3\nkeep 4\nmin-bins 12\n";
        let spec = parse_descriptor(text).expect("a spec");
        assert_eq!(spec.bin_seconds, 3600);
        assert_eq!(spec.series.len(), 2);
        assert_eq!(spec.separation, 2);
        assert_eq!(spec.edge, 3);
        assert_eq!(spec.keep, 4);
        assert_eq!(spec.min_bins, 12);
    }

    #[test]
    fn a_descriptor_without_mode_is_absent() {
        assert!(parse_descriptor("series a /tmp/a.txt\n").is_err());
    }

    #[test]
    fn a_descriptor_without_series_is_absent() {
        assert!(parse_descriptor("ripple pass 60\n").is_err());
    }

    #[test]
    fn a_joint_step_is_found_in_the_same_bin() {
        let a: Vec<(i64, f64)> = (0..24).map(|i| (i, 1.0 + i as f64)).collect();
        let b: Vec<(i64, f64)> = (0..24)
            .map(|i| {
                (
                    i,
                    if i < 12 {
                        1.0 + i as f64
                    } else {
                        1.0 + 3.0 * (i as f64 - 11.0)
                    },
                )
            })
            .collect();
        let xs: Vec<f64> = (0..24).map(|i| b[i as usize].1).collect();
        let ys: Vec<f64> = (0..24).map(|i| a[i as usize].1).collect();
        let bins: Vec<i64> = (0..24).collect();
        let steps = changepoints(&xs, &ys, &bins, 2, 2, 3);
        assert!(!steps.is_empty());
        assert!((steps[0].bin - 12).abs() <= 2);
    }

    #[test]
    fn a_flat_pair_carries_no_step() {
        let xs: Vec<f64> = (0..24).map(|i| 5.0 + i as f64).collect();
        let ys: Vec<f64> = xs.clone();
        let bins: Vec<i64> = (0..24).collect();
        let steps = changepoints(&xs, &ys, &bins, 2, 2, 3);
        assert!(steps.is_empty());
    }

    #[test]
    fn the_time_formatter_reads_a_known_epoch() {
        let epoch = omegaflow::archivar::parse_iso_unix("2005-03-04T00:00:00").expect("epoch");
        let text = format_time(3.0, 3600, epoch);
        assert_eq!(text, "2005-03-04T03:00:00Z");
    }

    #[test]
    fn the_series_keeps_the_given_name() {
        let s = series("dss24", &[(0, 1.0), (1, 2.0)]);
        assert_eq!(s.name, "dss24");
        assert_eq!(s.bins.len(), 2);
    }
}
