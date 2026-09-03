use omegaflow::archivar::membrane::MAX_SAMPLES;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::twomass::{
    MAGIC, RECORD_BYTES, Selection, file_list, parse_psc_row, read_bin, row_record,
};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};

const NETLOC: &str = "irsa.ipac.caltech.edu";
const PROGRESS_EVERY: u64 = 1_000_000;

enum Mode {
    Stdin,
    File(String),
    Finalize,
    FileList,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("twomass_compiler: {msg}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let mut mode: Option<Mode> = None;
    let mut selection: Option<Selection> = None;
    let mut rows: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--stdin" => mode = Some(Mode::Stdin),
            "--finalize" => mode = Some(Mode::Finalize),
            "--file-list" => mode = Some(Mode::FileList),
            "--file" => {
                i += 1;
                let v = args.get(i).ok_or("--file without path — refused")?;
                mode = Some(Mode::File(v.clone()));
            }
            "--rows" => {
                i += 1;
                rows = Some(args.get(i).ok_or("--rows without path — refused")?.clone());
            }
            "--out" => {
                i += 1;
                out = Some(args.get(i).ok_or("--out without path — refused")?.clone());
            }
            "--ci-mode" => ci = true,
            "--selection" => {
                i += 1;
                let kind = args
                    .get(i)
                    .ok_or("--selection without a selection — refused")?;
                selection = Some(parse_selection(kind, args, &mut i)?);
            }
            other => return Err(format!("unknown argument {other} — refused")),
        }
        i += 1;
    }
    match mode {
        None => {
            Err("no mode (--stdin | --file <path> | --finalize | --file-list) — refused".into())
        }
        Some(Mode::FileList) => {
            for name in file_list() {
                println!("{name}");
            }
            Ok(())
        }
        Some(Mode::Finalize) => finalize(
            &rows.ok_or("--finalize without --rows — refused")?,
            &out.ok_or("--finalize without --out — refused")?,
            ci,
        ),
        Some(Mode::Stdin) => {
            let sel = selection
                .as_ref()
                .ok_or("--stdin without --selection — the selection is never silent — refused")?;
            let rows_path = rows.ok_or("--stdin without --rows — refused")?;
            let stdin = std::io::stdin();
            let mut reader = BufReader::new(stdin.lock());
            harvest(&mut reader, sel, &rows_path)
        }
        Some(Mode::File(path)) => {
            let sel = selection
                .as_ref()
                .ok_or("--file without --selection — the selection is never silent — refused")?;
            let rows_path = rows.ok_or("--file without --rows — refused")?;
            if path.ends_with(".gz") {
                let bytes = std::fs::read(&path).map_err(|_| format!("{path}: read void"))?;
                let plain = gunzip(&bytes).ok_or(format!("{path}: gunzip void"))?;
                let mut reader = BufReader::new(plain.as_slice());
                harvest(&mut reader, sel, &rows_path)
            } else {
                let file = std::fs::File::open(&path).map_err(|_| format!("{path}: open void"))?;
                let mut reader = BufReader::new(file);
                harvest(&mut reader, sel, &rows_path)
            }
        }
    }
}

fn parse_selection(kind: &str, args: &[String], i: &mut usize) -> Result<Selection, String> {
    match kind {
        "jmag" => {
            *i += 1;
            if args.get(*i).map(String::as_str) != Some("--jmag-limit") {
                return Err("--selection jmag without --jmag-limit — refused".into());
            }
            *i += 1;
            let v: f64 = args
                .get(*i)
                .ok_or("--jmag-limit without a value — refused")?
                .parse()
                .map_err(|_| "--jmag-limit carries no number — refused".to_string())?;
            if !v.is_finite() || v <= 0.0 {
                return Err("--jmag-limit carries no plausible value — refused".into());
            }
            Ok(Selection::Jmag { limit: v })
        }
        "decimation" => {
            *i += 1;
            if args.get(*i).map(String::as_str) != Some("--decimation-factor") {
                return Err("--selection decimation without --decimation-factor — refused".into());
            }
            *i += 1;
            let v: u64 = args
                .get(*i)
                .ok_or("--decimation-factor without a value — refused")?
                .parse()
                .map_err(|_| "--decimation-factor carries no number — refused".to_string())?;
            if v == 0 {
                return Err("--decimation-factor 0 — refused".into());
            }
            Ok(Selection::Decimation { factor: v })
        }
        "declination" => {
            *i += 1;
            if args.get(*i).map(String::as_str) != Some("--dec-lo") {
                return Err("--selection declination without --dec-lo — refused".into());
            }
            *i += 1;
            let lo: f64 = args
                .get(*i)
                .ok_or("--dec-lo without a value — refused")?
                .parse()
                .map_err(|_| "--dec-lo carries no number — refused".to_string())?;
            *i += 1;
            if args.get(*i).map(String::as_str) != Some("--dec-hi") {
                return Err("--selection declination without --dec-hi — refused".into());
            }
            *i += 1;
            let hi: f64 = args
                .get(*i)
                .ok_or("--dec-hi without a value — refused")?
                .parse()
                .map_err(|_| "--dec-hi carries no number — refused".to_string())?;
            if !lo.is_finite() || !hi.is_finite() || lo < -90.0 || hi > 90.0 || lo >= hi {
                return Err("--dec-lo/--dec-hi carry no plausible band — refused".into());
            }
            Ok(Selection::Declination { lo, hi })
        }
        other => Err(format!("--selection {other}: unknown selection — refused")),
    }
}

fn seen_path(rows_path: &str) -> String {
    format!("{rows_path}.seen")
}

fn load_seen(rows_path: &str) -> u64 {
    std::fs::read(seen_path(rows_path))
        .ok()
        .and_then(|b| b.try_into().ok().map(u64::from_le_bytes))
        .unwrap_or(0)
}

fn save_seen(rows_path: &str, seen: u64) -> Result<(), String> {
    std::fs::write(seen_path(rows_path), seen.to_le_bytes())
        .map_err(|_| format!("seen {}: write void", seen_path(rows_path)))
}

fn harvest(reader: &mut dyn BufRead, selection: &Selection, rows_path: &str) -> Result<(), String> {
    let mut rows_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(rows_path)
        .map_err(|_| format!("rows {rows_path}: open void"))?;
    let mut w = BufWriter::new(&mut rows_file);
    let mut seen = load_seen(rows_path);
    let mut read = 0u64;
    let mut void = 0u64;
    let mut selected = 0u64;
    let mut line: Vec<u8> = Vec::new();
    loop {
        line.clear();
        match reader.read_until(b'\n', &mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => return Err("row stream read void".into()),
        }
        read += 1;
        let Ok(text) = std::str::from_utf8(&line) else {
            void += 1;
            continue;
        };
        let Some(row) = parse_psc_row(text) else {
            void += 1;
            continue;
        };
        if selection.keep(&row, &mut seen) {
            for v in row_record(&row) {
                w.write_all(&v.to_le_bytes())
                    .map_err(|_| format!("rows {rows_path}: write void"))?;
            }
            selected += 1;
        }
        if read % PROGRESS_EVERY == 0 {
            eprintln!("\r\x1b[K{read} rows, {void} rows void, {selected} selected (seen {seen})");
        }
    }
    w.flush()
        .map_err(|_| format!("rows {rows_path}: flush void"))?;
    if seen > 0 {
        save_seen(rows_path, seen)?;
    }
    eprintln!(
        "{read} rows, {void} rows void, {selected} selected (seen {seen}) — rows {rows_path}"
    );
    Ok(())
}

fn finalize(rows_path: &str, out: &str, ci: bool) -> Result<(), String> {
    let bytes = std::fs::read(rows_path).map_err(|_| format!("rows {rows_path}: read void"))?;
    if bytes.is_empty() || bytes.len() % RECORD_BYTES != 0 {
        return Err(format!(
            "rows {rows_path}: {} bytes — no {}-byte records — the asset stays unwritten",
            bytes.len(),
            RECORD_BYTES
        ));
    }
    let count = bytes.len() / RECORD_BYTES;
    if count > MAX_SAMPLES {
        return Err(format!(
            "selection carries {count} sources over MAX_SAMPLES {MAX_SAMPLES} — the asset stays unwritten"
        ));
    }
    let mut bin = Vec::with_capacity(8 + bytes.len());
    bin.extend_from_slice(&MAGIC);
    bin.extend_from_slice(&(count as u32).to_le_bytes());
    bin.extend_from_slice(&bytes);
    let Some(parsed) = read_bin(&bin) else {
        return Err("roundtrip parse void — the asset stays unwritten".into());
    };
    let mut jmin = f64::INFINITY;
    let mut jmax = 0.0f64;
    for r in &parsed {
        if r[2] > 0.0 {
            jmin = jmin.min(r[2]);
            jmax = jmax.max(r[2]);
        }
    }
    std::fs::write(out, &bin).map_err(|_| format!("write {out} void"))?;
    eprintln!(
        "{out}: {count} sources, jmag {jmin:.3}..{jmax:.3}, {} B — roundtrip parses",
        bin.len()
    );
    if ci && !upload_release(NETLOC, out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}
