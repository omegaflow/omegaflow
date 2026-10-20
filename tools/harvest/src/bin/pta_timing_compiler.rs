
use omegaflow::archivar::inflate::gunzip_tar_members;
use omegaflow::cdn::upload_release;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::process::{Child, ChildStdout, Command, Stdio};

const COMPILER: &str = "tools/harvest/src/bin/pta_timing_compiler.rs";
const DEFAULT_OUT: &str = "pta_timing.csv";
const DEFAULT_NETLOC: &str = "pta-timing";

#[derive(Clone, Debug, Default, PartialEq)]
struct Params {
    psr: String,
    f0: Option<f64>,
    f1: Option<f64>,
    pepoch: Option<f64>,
    dm: Option<f64>,
    tres_us: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
struct Row {
    psr: String,
    mjd: f64,
    freq_mhz: f64,
    residual_us: Option<f64>,
    uncertainty_us: Option<f64>,
    tres_us: Option<f64>,
    observatory: String,
    pta: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Resid {
    mjd: f64,
    freq_mhz: f64,
    residual_us: f64,
    uncertainty_us: Option<f64>,
    flag: String,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn finite(s: &str) -> Option<f64> {
    s.trim()
        .trim_matches('"')
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
}

fn flag_value<'a>(toks: &'a [&'a str], key: &str) -> Option<String> {
    toks.windows(2)
        .find(|w| w[0] == key)
        .map(|w| w[1].to_string())
}


fn parse_par(text: &str) -> Option<Params> {
    let mut p = Params::default();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t == "C" || t.starts_with("C ") {
            continue;
        }
        let mut it = t.split_whitespace();
        let Some(key) = it.next() else { continue };
        let val = it.next();
        match key {
            "PSRJ" | "PSR" => {
                if let Some(v) = val {
                    let v = v.trim_matches('"');
                    if !v.is_empty() {
                        p.psr = match canonical_psr(v) {
                            Some(name) => name,
                            None => v.to_string(),
                        };
                    }
                }
            }
            "F0" => p.f0 = val.and_then(finite),
            "F1" => p.f1 = val.and_then(finite),
            "PEPOCH" => p.pepoch = val.and_then(finite),
            "DM" => p.dm = val.and_then(finite),
            "TRES" => p.tres_us = val.and_then(finite),
            _ => {}
        }
    }
    if p.psr.is_empty() { None } else { Some(p) }
}


fn is_psr_token(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 8 || (b[0] != b'J' && b[0] != b'B') {
        return false;
    }
    let mut digits = 0usize;
    let mut sign = false;
    for &c in &b[1..] {
        if c.is_ascii_digit() {
            digits += 1;
        } else if c == b'+' || c == b'-' {
            sign = true;
        } else if !c.is_ascii_alphabetic() {
            return false;
        }
    }
    digits >= 4 && sign
}

fn canonical_psr(s: &str) -> Option<String> {
    if !is_psr_token(s) {
        return None;
    }
    let mut end = s.len();
    while end > 1 && s.as_bytes()[end - 1].is_ascii_lowercase() {
        end -= 1;
    }
    let trimmed = &s[..end];
    Some(if is_psr_token(trimmed) { trimmed } else { s }.to_string())
}

fn psr_from_member(name: &str) -> Option<String> {
    for comp in name.split('/').rev() {
        if comp.is_empty() || comp.starts_with('.') {
            continue;
        }
        let stem = comp.split('.').next().unwrap_or(comp);
        for cand in stem.split('_') {
            if let Some(psr) = canonical_psr(cand) {
                return Some(psr);
            }
        }
    }
    None
}

fn normalize_tag(tag: &str, fallback: &str) -> String {
    let t = tag.trim();
    if t.eq_ignore_ascii_case("nanograv") {
        "NANOGrav15yr".to_string()
    } else if t.eq_ignore_ascii_case("ppta") {
        "PPTA DR3".to_string()
    } else if t.eq_ignore_ascii_case("inpta") {
        "InPTA DR2".to_string()
    } else if t.eq_ignore_ascii_case("mpta") {
        "MPTA 4.5yr".to_string()
    } else if t.is_empty() {
        fallback.to_string()
    } else {
        t.to_string()
    }
}

fn parse_tim(text: &str, pta_fallback: &str) -> Vec<Row> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with('#') || t.starts_with("C ") || t == "C" {
            continue;
        }
        let upper = t.to_ascii_uppercase();
        if upper.starts_with("FORMAT")
            || upper.starts_with("MODE")
            || upper.starts_with("INCLUDE")
            || upper.starts_with("INFO")
            || upper.starts_with("TIMEOFFSET")
            || upper.starts_with("JUMP")
        {
            continue;
        }
        let toks: Vec<&str> = t.split_whitespace().collect();
        if toks.len() < 5 {
            continue;
        }
        let Some(freq_mhz) = finite(toks[1]) else {
            continue;
        };
        let Some(mjd) = finite(toks[2]) else { continue };
        let Some(uncertainty_us) = finite(toks[3]) else {
            continue;
        };
        if !(30_000.0..=70_000.0).contains(&mjd) || freq_mhz <= 0.0 || uncertainty_us < 0.0 {
            continue;
        }
        let tag = match flag_value(&toks[5..], "-pta")
            .map(|v| normalize_tag(&v, pta_fallback))
        {
            Some(v) => v,
            None => pta_fallback.to_string(),
        };
        out.push(Row {
            psr: String::new(),
            mjd,
            freq_mhz,
            residual_us: None,
            uncertainty_us: Some(uncertainty_us),
            tres_us: None,
            observatory: toks[4].to_string(),
            pta: tag,
        });
    }
    out
}


fn parse_residual(text: &str, _pta: &str) -> Vec<Resid> {
    let mut residual_col = 2usize;
    let mut uncertainty_col = 3usize;
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with('#') {
            let lower = t.to_ascii_lowercase();
            if lower.contains("mjd") && lower.contains("residual") {
                let cols: Vec<String> = t
                    .trim_start_matches('#')
                    .split_whitespace()
                    .map(|s| s.to_ascii_lowercase())
                    .collect();
                if let Some(i) = cols
                    .iter()
                    .position(|c| c.contains("residual") && !c.contains("white"))
                {
                    residual_col = i;
                }
                if let Some(i) = cols.iter().position(|c| c.contains("uncertainty")) {
                    uncertainty_col = i;
                }
            }
            continue;
        }
        let toks: Vec<&str> = t.split_whitespace().collect();
        let need = residual_col.max(uncertainty_col).max(2);
        if toks.len() <= need {
            continue;
        }
        let Some(mjd) = finite(toks[0]) else { continue };
        let Some(freq_mhz) = finite(toks[1]) else {
            continue;
        };
        let Some(residual_us) = finite(toks[residual_col]) else {
            continue;
        };
        if !(30_000.0..=70_000.0).contains(&mjd) || freq_mhz <= 0.0 {
            continue;
        }
        let uncertainty_us = finite(toks[uncertainty_col]).filter(|v| *v >= 0.0);
        let flag = match toks
            .last()
            .filter(|s| finite(s).is_none())
            .map(|s| s.to_string())
        {
            Some(s) => s,
            None => String::new(),
        };
        out.push(Resid {
            mjd,
            freq_mhz,
            residual_us,
            uncertainty_us,
            flag,
        });
    }
    out
}


fn qmjd(mjd: f64) -> i64 {
    (mjd * 1e6).round() as i64
}

fn qfreq(freq: f64) -> i64 {
    (freq * 1e3).round() as i64
}

fn finalize(
    psr: &str,
    params: Option<&Params>,
    tims: &[Row],
    ress: &[Resid],
    pta: &str,
) -> Vec<Row> {
    let mut out = Vec::new();
    let tres_us = params.and_then(|p| p.tres_us);

    if !tims.is_empty() {
        let mut by_key: HashMap<(i64, i64), Vec<&Resid>> = HashMap::new();
        let mut by_mjd: HashMap<i64, Vec<&Resid>> = HashMap::new();
        for r in ress {
            by_key
                .entry((qmjd(r.mjd), qfreq(r.freq_mhz)))
                .or_default()
                .push(r);
            by_mjd.entry(qmjd(r.mjd)).or_default().push(r);
        }
        let mut matched: HashSet<(i64, i64)> = HashSet::new();
        for t in tims {
            let mut row = t.clone();
            row.psr = psr.to_string();
            row.tres_us = tres_us;
            let key = (qmjd(t.mjd), qfreq(t.freq_mhz));
            let hit = by_key
                .get(&key)
                .and_then(|v| v.first())
                .copied()
                .or_else(|| {
                    by_mjd.get(&key.0).and_then(|v| {
                        v.iter()
                            .min_by(|a, b| {
                                (a.freq_mhz - t.freq_mhz)
                                    .abs()
                                    .total_cmp(&(b.freq_mhz - t.freq_mhz).abs())
                            })
                            .copied()
                    })
                });
            if let Some(r) = hit {
                matched.insert((qmjd(r.mjd), qfreq(r.freq_mhz)));
                row.residual_us = Some(r.residual_us);
                if r.uncertainty_us.is_some() {
                    row.uncertainty_us = r.uncertainty_us;
                }
                if row.observatory.is_empty() {
                    row.observatory = r.flag.clone();
                }
            }
            if row.pta.is_empty() {
                row.pta = pta.to_string();
            }
            out.push(row);
        }
        for r in ress {
            if matched.contains(&(qmjd(r.mjd), qfreq(r.freq_mhz))) {
                continue;
            }
            out.push(Row {
                psr: psr.to_string(),
                mjd: r.mjd,
                freq_mhz: r.freq_mhz,
                residual_us: Some(r.residual_us),
                uncertainty_us: r.uncertainty_us,
                tres_us,
                observatory: r.flag.clone(),
                pta: pta.to_string(),
            });
        }
    } else {
        for r in ress {
            out.push(Row {
                psr: psr.to_string(),
                mjd: r.mjd,
                freq_mhz: r.freq_mhz,
                residual_us: Some(r.residual_us),
                uncertainty_us: r.uncertainty_us,
                tres_us,
                observatory: r.flag.clone(),
                pta: pta.to_string(),
            });
        }
    }

    if tres_us.is_none() {
        let mut num = 0.0f64;
        let mut den = 0.0f64;
        for r in &out {
            if let Some(res) = r.residual_us {
                let w = match r.uncertainty_us {
                    Some(s) if s > 0.0 => 1.0 / (s * s),
                    _ => 1.0,
                };
                num += res * res * w;
                den += w;
            }
        }
        if den > 0.0 {
            let rms = (num / den).sqrt();
            for r in &mut out {
                r.tres_us = Some(rms);
            }
        }
    }

    out.sort_by(|a, b| {
        a.mjd
            .total_cmp(&b.mjd)
            .then_with(|| a.freq_mhz.total_cmp(&b.freq_mhz))
    });
    let mut seen = HashSet::new();
    out.retain(|r| {
        seen.insert((
            qmjd(r.mjd),
            qfreq(r.freq_mhz),
            r.residual_us.map(|v| (v * 1e6).round() as i64),
        ))
    });
    out
}


fn detect_pta(name: &str) -> Option<&'static str> {
    let n = name.to_ascii_lowercase();
    if n.contains("nanograv") {
        Some("NANOGrav15yr")
    } else if n.contains("ppta") {
        Some("PPTA DR3")
    } else if n.contains("inpta") {
        Some("InPTA DR2")
    } else if n.contains("mpta") || n.contains("partim/") {
        Some("MPTA 4.5yr")
    } else {
        None
    }
}

struct CurlReader {
    _child: Child,
    out: ChildStdout,
}

impl Read for CurlReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.out.read(buf)
    }
}

fn open_input(args: &[String]) -> Result<(String, Box<dyn Read>), String> {
    if let Some(path) = arg_value(args, "--file").or_else(|| arg_value(args, "--tarball")) {
        let f = File::open(&path).map_err(|e| format!("{path}: the file stays unread: {e}"))?;
        return Ok((path, Box::new(f)));
    }
    if let Some(url) = arg_value(args, "--url") {
        let mut child = Command::new("curl")
            .args(["-sSfL", "--max-time", "3600"])
            .arg(&url)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("{url}: curl spawn returned void: {e}"))?;
        let out = child
            .stdout
            .take()
            .ok_or_else(|| format!("{url}: curl stdout stays absent"))?;
        return Ok((url, Box::new(CurlReader { _child: child, out })));
    }
    Err("usage: pta_timing_compiler (--file <release.tar.gz> | --url <url>) [--out <csv>] [--split-dir <dir>] [--pta <label>]".into())
}


fn csv_cell(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn fmt_num(v: Option<f64>, prec: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.prec$}"),
        _ => String::new(),
    }
}

fn header_comment(pta: &str) -> String {
    format!(
        "# pta_timing_compiler v1\n# compiler {COMPILER}\n# pta {pta}\n# at sun\n# no-cadence\n# frame SSB\n# tres par TRES, else weighted residual RMS\n"
    )
}

fn write_combined(out: &str, rows: &[Row], pta: &str) -> Result<(), String> {
    let mut text = header_comment(pta);
    text.push_str("pta,psr,observatory,mjd,freq_mhz,residual_us,uncertainty_us,tres_us\n");
    for r in rows {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            csv_cell(&r.pta),
            csv_cell(&r.psr),
            csv_cell(&r.observatory),
            fmt_num(Some(r.mjd), 12),
            fmt_num(Some(r.freq_mhz), 6),
            fmt_num(r.residual_us, 6),
            fmt_num(r.uncertainty_us, 6),
            fmt_num(r.tres_us, 6),
        ));
    }
    std::fs::write(out, text).map_err(|e| format!("write {out} returned void: {e}"))
}

fn sanitize_file(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn write_split(dir: &str, rows: &[Row], pta: &str) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{dir}: mkdir returned void: {e}"))?;
    let mut by_psr: BTreeMap<String, Vec<&Row>> = BTreeMap::new();
    for r in rows {
        by_psr.entry(r.psr.clone()).or_default().push(r);
    }
    for (psr, set) in by_psr {
        let path = format!("{dir}/{}_{}.csv", sanitize_file(pta), sanitize_file(&psr));
        let mut text = header_comment(pta);
        text.push_str("pta,psr,mjd,freq_mhz,residual_us,uncertainty_us,tres_us\n");
        for r in set {
            text.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                csv_cell(&r.pta),
                csv_cell(&r.psr),
                fmt_num(Some(r.mjd), 12),
                fmt_num(Some(r.freq_mhz), 6),
                fmt_num(r.residual_us, 6),
                fmt_num(r.uncertainty_us, 6),
                fmt_num(r.tres_us, 6),
            ));
        }
        std::fs::write(&path, text).map_err(|e| format!("write {path} returned void: {e}"))?;
    }
    Ok(())
}


fn run(args: &[String]) -> Result<(), String> {
    let (src, reader) = open_input(args)?;
    let pta_override = arg_value(args, "--pta");

    let mut pars: BTreeMap<String, Params> = BTreeMap::new();
    let mut tims: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    let mut ress: BTreeMap<String, Vec<Resid>> = BTreeMap::new();
    let mut detected: Option<String> = None;
    let mut members = 0usize;

    let total = gunzip_tar_members(
        reader,
        |name| {
            let n = name.to_ascii_lowercase();
            !n.contains("__macosx")
                && (n.ends_with(".par") || n.ends_with(".tim") || n.ends_with(".res"))
        },
        |name, data| {
            members += 1;
            let Ok(text) = std::str::from_utf8(data) else {
                return;
            };
            let fallback = match pta_override
                .clone()
                .or_else(|| detect_pta(name).map(|s| s.to_string()))
                .or_else(|| detected.clone())
            {
                Some(v) => v,
                None => "unknown".to_string(),
            };
            if detected.is_none() {
                detected = Some(fallback.clone());
            }
            let lower = name.to_ascii_lowercase();
            if lower.ends_with(".par") {
                if let Some(mut p) = parse_par(text) {
                    if p.psr.is_empty() {
                        if let Some(psr) = psr_from_member(name) {
                            p.psr = psr;
                        }
                    }
                    let slot = pars.entry(p.psr.clone()).or_insert_with(|| p.clone());
                    if slot.tres_us.is_none() && p.tres_us.is_some() {
                        *slot = p;
                    }
                }
            } else if lower.ends_with(".tim") {
                let Some(psr) = psr_from_member(name) else {
                    return;
                };
                let rows = parse_tim(text, &fallback);
                if !rows.is_empty() {
                    tims.entry(psr).or_default().extend(rows);
                }
            } else if lower.ends_with(".res") {
                let Some(psr) = psr_from_member(name) else {
                    return;
                };
                let rows = parse_residual(text, &fallback);
                if !rows.is_empty() {
                    ress.entry(psr).or_default().extend(rows);
                }
            }
        },
    )
    .map_err(|e| format!("{src}: {e}"))?;

    let pta_label = match pta_override.clone().or(detected) {
        Some(v) => v,
        None => "unknown".to_string(),
    };

    let mut psrs: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    let mut keys: Vec<String> = pars
        .keys()
        .chain(tims.keys())
        .chain(ress.keys())
        .cloned()
        .collect();
    keys.sort();
    keys.dedup();
    for psr in keys {
        let rows = finalize(
            &psr,
            pars.get(&psr),
            tims.get(&psr).map(|v| v.as_slice()).unwrap_or(&[]),
            ress.get(&psr).map(|v| v.as_slice()).unwrap_or(&[]),
            &pta_label,
        );
        if !rows.is_empty() {
            psrs.insert(psr, rows);
        }
    }

    let mut rows: Vec<Row> = psrs.into_values().flatten().collect();
    rows.sort_by(|a, b| {
        a.pta
            .cmp(&b.pta)
            .then_with(|| a.psr.cmp(&b.psr))
            .then_with(|| a.mjd.total_cmp(&b.mjd))
            .then_with(|| a.freq_mhz.total_cmp(&b.freq_mhz))
    });

    if rows.is_empty() {
        return Err(format!(
            "{src}: {members} wanted member(s), no TOA/residual row — the table stays unwritten"
        ));
    }

    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    if has_flag(args, "--stat") {
        let np = rows
            .iter()
            .map(|r| r.psr.as_str())
            .collect::<HashSet<_>>()
            .len();
        let nr = rows.iter().filter(|r| r.residual_us.is_some()).count();
        eprintln!(
            "{src}: {np} pulsar(s), {} row(s), {nr} with residual, {members} member(s) read, {total} decompressed tar bytes",
            rows.len()
        );
        return Ok(());
    }

    write_combined(&out, &rows, &pta_label)?;
    if let Some(dir) = arg_value(args, "--split-dir") {
        write_split(&dir, &rows, &pta_label)?;
    }
    eprintln!(
        "pta_timing_compiler: {} row(s) from {members} member(s) -> {out} (PTA {pta_label}, at sun, no-cadence)",
        rows.len()
    );

    if has_flag(args, "--ci-mode") {
        let netloc = match arg_value(args, "--netloc") {
            Some(v) => v,
            None => DEFAULT_NETLOC.to_string(),
        };
        if !upload_release(&netloc, &out) {
            return Err(format!("{out}: CDN upload returned void"));
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("pta_timing_compiler: {msg}");
        std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const PAR: &str = "\
PSRJ           J0437-4715
RAJ             04:37:15.9284079
DECJ           -47:15:09.30372
F0             173.68794566494279101
F1             -1.7283596545597335406e-15
PEPOCH         55486
DM             3.0747561115068535438
TRES           0.387
";

    const TIM: &str = "\
FORMAT 1
MODE 1
INCLUDE tims/other.tim
 r070716_195854.rf.pcm 2968.05500000 54297.85253478867169363 0.08200 pks -fe 1050CM -be PDFB2 -pta PPTA
 r070724_195852.rf.pcm 1337.21800000 54305.85481753976480235 0.08100 pks -fe MULTI -pta PPTA
";

    const RES: &str = "\
# J0636+5128_NG15yr_nb.full.res
# Narrowband post-fit residuals (no red noise)
# MJD frequency(MHz) residual(us) uncertainty(us) chi2 flag
58011.862736222654\t1165.636\t-5.092544071271619191\t5.895675156573922\t0.926\tRcvr1_2_GUPPI
58011.86273624338\t1185.159\t25.06768764916468003\t8.80233903687875\t1.08\tRcvr1_2_GUPPI
";

    const RES_AVG_RN: &str = "\
# avg_MJD avg_frequency(MHz) avg_residual(us) avg_white_residual(us) avg_uncertainty(us)
53267.35935334882\t1400.1028910559035\t-3.3147824404753686532\t-2.9942250301861294758\t1.5493322441385498
";

    #[test]
    fn par_reads_the_timing_model() {
        let p = parse_par(PAR).expect("the par parses");
        assert_eq!(p.psr, "J0437-4715");
        assert!((p.f0.unwrap() - 173.68794566494279101).abs() < 1e-9);
        assert!((p.f1.unwrap() + 1.7283596545597335e-15).abs() < 1e-25);
        assert_eq!(p.pepoch, Some(55486.0));
        assert!((p.dm.unwrap() - 3.0747561115068535).abs() < 1e-9);
        assert!((p.tres_us.unwrap() - 0.387).abs() < 1e-12);
    }

    #[test]
    fn tim_reads_toas_and_tolerates_include() {
        let rows = parse_tim(TIM, "PPTA DR3");
        assert_eq!(rows.len(), 2, "INCLUDE/FORMAT/MODE must not become rows");
        assert!((rows[0].mjd - 54297.85253478867169363).abs() < 1e-9);
        assert!((rows[0].freq_mhz - 2968.055).abs() < 1e-9);
        assert_eq!(rows[0].uncertainty_us, Some(0.082));
        assert_eq!(rows[0].observatory, "pks");
        assert_eq!(rows[0].pta, "PPTA DR3");
        assert_eq!(rows[0].residual_us, None);
    }

    #[test]
    fn residual_header_maps_mjd_frequency_residual_uncertainty() {
        let rows = parse_residual(RES, "NANOGrav15yr");
        assert_eq!(rows.len(), 2);
        assert!((rows[0].mjd - 58011.862736222654).abs() < 1e-9);
        assert!((rows[0].freq_mhz - 1165.636).abs() < 1e-9);
        assert!((rows[0].residual_us + 5.092544071271619).abs() < 1e-9);
        assert!((rows[0].uncertainty_us.unwrap() - 5.895675156573922).abs() < 1e-9);
        assert_eq!(rows[0].flag, "Rcvr1_2_GUPPI");
    }

    #[test]
    fn residual_rn_header_skips_the_white_residual_column() {
        let rows = parse_residual(RES_AVG_RN, "NANOGrav15yr");
        assert_eq!(rows.len(), 1);
        assert!((rows[0].residual_us + 3.3147824404753687).abs() < 1e-9);
        assert!((rows[0].uncertainty_us.unwrap() - 1.5493322441385498).abs() < 1e-9);
    }

    #[test]
    fn finalize_joins_residual_to_the_matching_toa() {
        let p = parse_par(PAR).unwrap();
        let tims = parse_tim(
            "FORMAT 1\nMODE 1\ntoa.fits 1165.636 58011.862736222654 5.0 pks -pta NANOGrav\n",
            "NANOGrav15yr",
        );
        let ress = parse_residual(RES, "NANOGrav15yr");
        let rows = finalize("J0636+5128", Some(&p), &tims, &ress, "NANOGrav15yr");
        assert_eq!(
            rows.len(),
            2,
            "the matched TOA plus the unmatched epoch row"
        );
        let joined = rows
            .iter()
            .find(|r| (r.freq_mhz - 1165.636).abs() < 1e-9)
            .expect("the TOA row is present");
        assert!((joined.residual_us.unwrap() + 5.092544071271619).abs() < 1e-9);
        assert!((joined.tres_us.unwrap() - 0.387).abs() < 1e-12);
        assert_eq!(joined.psr, "J0636+5128");
    }

    #[test]
    fn missing_tres_falls_back_to_the_weighted_residual_rms() {
        let p = parse_par("PSR J0000+0000\nF0 1.0\n").unwrap();
        let tims = parse_tim(
            "FORMAT 1\nMODE 1\ntoa 1400.0 58000.0 1.0 pks\n",
            "NANOGrav15yr",
        );
        let ress = parse_residual(
            "# MJD frequency(MHz) residual(us) uncertainty(us) chi2 flag\n\
             58000.0 1400.0 3.0 1.0 1.0 x\n\
             58000.0 1401.0 4.0 1.0 1.0 x\n",
            "NANOGrav15yr",
        );
        let rows = finalize("J0000+0000", Some(&p), &tims, &ress, "NANOGrav15yr");
        let want = (12.5f64).sqrt();
        assert!(
            rows.iter()
                .all(|r| (r.tres_us.unwrap() - want).abs() < 1e-12)
        );
    }

    #[test]
    fn canonical_psr_strips_a_receiver_suffix_but_keeps_a_name_letter() {
        assert_eq!(
            canonical_psr("J1713+0747gbt").as_deref(),
            Some("J1713+0747")
        );
        assert_eq!(
            canonical_psr("J1909-3744gbt").as_deref(),
            Some("J1909-3744")
        );
        assert_eq!(canonical_psr("J1824-2452A").as_deref(), Some("J1824-2452A"));
        assert_eq!(canonical_psr("J0437-4715").as_deref(), Some("J0437-4715"));
    }

    #[test]
    fn member_name_carries_the_pulsar() {
        assert_eq!(
            psr_from_member(
                "NANOGrav15yr_PulsarTiming_v2.1.0/narrowband/tim/J0437-4715_PINT_20220301.nb.tim"
            )
            .as_deref(),
            Some("J0437-4715")
        );
        assert_eq!(
            psr_from_member("InPTA.DR2-e2806fc/J0437-4715/tims/GM_GWB_1460_100.0_b1_pre36.tim")
                .as_deref(),
            Some("J0437-4715")
        );
        assert_eq!(
            psr_from_member("partim/J0437-4715.par").as_deref(),
            Some("J0437-4715")
        );
        assert_eq!(
            psr_from_member(
                "NANOGrav15yr_PulsarTiming_v2.1.0/narrowband/tim/J1713+0747gbt_PINT_20220302.nb.tim"
            )
            .as_deref(),
            Some("J1713+0747")
        );
        assert_eq!(
            psr_from_member("PPTA-DR3/analysis_codes/data/all/J1824-2452A.tim").as_deref(),
            Some("J1824-2452A")
        );
    }

    #[test]
    fn csv_carries_the_pta_label_and_tres() {
        let p = parse_par(PAR).unwrap();
        let rows = finalize(
            "J0437-4715",
            Some(&p),
            &parse_tim(TIM, "PPTA DR3"),
            &[],
            "PPTA DR3",
        );
        let mut text = header_comment("PPTA DR3");
        text.push_str("pta,psr,observatory,mjd,freq_mhz,residual_us,uncertainty_us,tres_us\n");
        for r in &rows {
            text.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                r.pta,
                r.psr,
                r.observatory,
                fmt_num(Some(r.mjd), 12),
                fmt_num(Some(r.freq_mhz), 6),
                fmt_num(r.residual_us, 6),
                fmt_num(r.uncertainty_us, 6),
                fmt_num(r.tres_us, 6),
            ));
        }
        assert!(text.contains("# at sun"));
        assert!(text.contains("# no-cadence"));
        assert!(text.contains("PPTA DR3,J0437-4715,pks,"));
        assert!(text.ends_with("0.387000\n"));
    }
}
