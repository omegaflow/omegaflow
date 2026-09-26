use omegaflow::cdn::upload_release;
use omegaflow::inflate::{gunzip, gunzip_stream, gunzip_tar_members};
use omegaflow::sha256::sha256_hex;
use std::process::exit;

const NETLOC: &str = "almascience.org";
const USAGE: &str = "eht_uvfits_compiler: modes: --inspect <file.tgz> | --cat <file.tgz> <member> | --verify <file.tgz> | --run <file.tgz> [--pair <a> <b>] [--out <bin>] [--ci-mode] | --runfits <file.FITS> [--pair <a> <b>]";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn pair_arg(args: &[String]) -> (String, String) {
    let Some(i) = args.iter().position(|a| a == "--pair") else {
        return (
            omegaflow::uvfits::STATION_ALMA.to_string(),
            omegaflow::uvfits::STATION_APEX.to_string(),
        );
    };
    let (Some(a), Some(b)) = (args.get(i + 1), args.get(i + 2)) else {
        eprintln!("eht_uvfits_compiler: --pair <a> <b>");
        exit(2);
    };
    (a.clone(), b.clone())
}

fn pair_beat_open(f: &omegaflow::uvfits::UvFits, pair: (&str, &str)) -> bool {
    let pair_rows = omegaflow::uvfits::baseline_rows(f, pair.0, pair.1);
    if pair_rows.is_empty() {
        return false;
    }
    match omegaflow::uvfits::fringe_rate_hz(&pair_rows) {
        Some(df) => {
            let dt_s = pair_rows.iter().map(|r| r.inttim_s).sum::<f64>() / pair_rows.len() as f64;
            omegaflow::uvfits::beat_open(df, dt_s)
        }
        None => false,
    }
}

fn pair_readable(bytes: &[u8], pair: (&str, &str)) -> bool {
    if pair.0 == omegaflow::uvfits::STATION_ALMA && pair.1 == omegaflow::uvfits::STATION_APEX {
        return omegaflow::uvfits::beat_rows(bytes).is_some();
    }
    match omegaflow::uvfits::parse_uvfits(bytes) {
        Some(f) => pair_beat_open(&f, pair),
        None => false,
    }
}

fn read_file(path: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("eht_uvfits_compiler: {path}: read returned void: {e}");
            exit(2);
        }
    }
}

fn tar_octal(field: &[u8]) -> Option<usize> {
    let mut value = 0usize;
    let mut any = false;
    for &b in field {
        if b == 0 || b == b' ' {
            break;
        }
        if !(b'0'..=b'7').contains(&b) {
            return None;
        }
        value = value * 8 + (b - b'0') as usize;
        any = true;
    }
    if any { Some(value) } else { None }
}

fn tar_text(field: &[u8]) -> String {
    let end = field.iter().position(|&b| b == 0).unwrap_or(field.len());
    String::from_utf8_lossy(&field[..end]).to_string()
}

fn tar_partial(tar: &[u8]) -> Option<Vec<(String, usize, usize)>> {
    let mut out = Vec::new();
    let mut off = 0usize;
    loop {
        let header = tar.get(off..off.checked_add(512)?)?;
        if header.iter().all(|&b| b == 0) {
            return Some(out);
        }
        let size = tar_octal(&header[124..136])?;
        let typeflag = header[156];
        let data_off = off + 512;
        let full_end = data_off.checked_add(size)?;
        let avail_end = full_end.min(tar.len());
        if matches!(typeflag, b'0' | 0) {
            out.push((tar_text(&header[..100]), data_off, avail_end));
        }
        if full_end > tar.len() {
            return Some(out);
        }
        off = full_end.checked_add((512 - size % 512) % 512)?;
    }
}

fn fits_cards(bytes: &[u8], n: usize) {
    let mut off = 0usize;
    for _ in 0..n {
        let Some(card) = bytes.get(off..off + 80) else {
            return;
        };
        let text = String::from_utf8_lossy(card);
        let clean = text.trim_end_matches([' ', '\0']);
        if !clean.is_empty() {
            println!("{clean}");
        }
        off += 80;
    }
}

fn gunzip_tolerant(bytes: &[u8]) -> Vec<u8> {
    match gunzip(bytes) {
        Some(t) => t,
        None => {
            let mut partial = Vec::new();
            let sink = |chunk: &[u8]| {
                partial.extend_from_slice(chunk);
            };
            let _ = gunzip_stream(bytes, sink);
            partial
        }
    }
}

fn inspect(path: &str) {
    let bytes = read_file(path);
    let tar = gunzip_tolerant(&bytes);
    match tar_partial(&tar) {
        Some(members) => {
            for (name, start, end) in &members {
                println!("{} {}", end - start, name);
            }
            for (name, start, end) in &members {
                if name.ends_with(".FITS") {
                    println!("--- {name} (first 12 FITS cards) ---");
                    fits_cards(&tar[*start..*end], 12);
                    break;
                }
            }
        }
        None => eprintln!("eht_uvfits_compiler: {path}: tar walk returned void"),
    }
}

fn cat_member(path: &str, want_name: &str) {
    let bytes = read_file(path);
    let tar = gunzip_tolerant(&bytes);
    let members = match tar_partial(&tar) {
        Some(m) => m,
        None => {
            eprintln!("eht_uvfits_compiler: {path}: tar member walk returned void");
            exit(1);
        }
    };
    match members.into_iter().find(|(n, _, _)| n == want_name) {
        Some((_, start, end)) => {
            use std::io::Write;
            let out = std::io::stdout();
            let mut handle = out.lock();
            let _ = handle.write_all(&tar[start..end]);
            let _ = handle.flush();
        }
        None => {
            eprintln!("eht_uvfits_compiler: {path}: member {want_name} absent");
            exit(1);
        }
    }
}

fn verify(path: &str) {
    let bytes = read_file(path);
    let tar = gunzip_tolerant(&bytes);
    let members = match tar_partial(&tar) {
        Some(m) => m,
        None => {
            eprintln!("eht_uvfits_compiler: {path}: tar member walk returned void");
            exit(1);
        }
    };
    let Some((_, sums_start, sums_end)) =
        members.iter().find(|(n, _, _)| n.ends_with(".sha256sums"))
    else {
        eprintln!("eht_uvfits_compiler: {path}: no .sha256sums member");
        exit(1);
    };
    let sums_text = String::from_utf8_lossy(&tar[*sums_start..*sums_end]);
    let mut checked = 0usize;
    let mut ok = 0usize;
    let mut missing = 0usize;
    for line in sums_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let (Some(hash), Some(name)) = (parts.next(), parts.next()) else {
            eprintln!("eht_uvfits_compiler: .sha256sums line carries no hash-name pair: {line}");
            exit(1);
        };
        match members.iter().find(|(n, _, _)| n == name) {
            Some((_, start, end)) => {
                checked += 1;
                let computed = sha256_hex(&tar[*start..*end]);
                if computed == hash {
                    ok += 1;
                } else {
                    eprintln!("eht_uvfits_compiler: {name}: {computed} != {hash}");
                }
            }
            None => {
                missing += 1;
                eprintln!("eht_uvfits_compiler: {name}: member absent");
            }
        }
    }
    println!("{path}: {ok}/{checked} members verified, {missing} absent");
    if missing > 0 || ok != checked {
        exit(1);
    }
}

fn report_member(name: &str, bytes: &[u8], pair: (&str, &str)) {
    let Some(f) = omegaflow::uvfits::parse_uvfits(bytes) else {
        eprintln!("{name}: uvfits parse void");
        return;
    };
    let ant_names: Vec<&str> = f.antennas.iter().map(|a| a.name.as_str()).collect();
    println!(
        "{name}: antennas=[{}] rows={} ref_freq={:.3e} chan_bw={:.3e}",
        ant_names.join(","),
        f.rows.len(),
        f.ref_freq_hz,
        f.chan_bw_hz
    );
    let pair_rows = omegaflow::uvfits::baseline_rows(&f, pair.0, pair.1);
    if pair_rows.is_empty() {
        println!("{name}: no {}-{} rows", pair.0, pair.1);
        return;
    }
    let Some(df) = omegaflow::uvfits::fringe_rate_hz(&pair_rows) else {
        eprintln!(
            "{name}: {}-{} pair present, fringe rate void",
            pair.0, pair.1
        );
        return;
    };
    let dt_s: f64 = pair_rows.iter().map(|r| r.inttim_s).sum::<f64>() / pair_rows.len() as f64;
    let open = omegaflow::uvfits::beat_open(df, dt_s);
    println!(
        "{name}: {}-{} rows={} df={:.6e} Hz dt={:.6} s gate={} (df*dt={:.6})",
        pair.0,
        pair.1,
        pair_rows.len(),
        df,
        dt_s,
        if open { "open" } else { "closed" },
        df * dt_s
    );
    if !open {
        return;
    }
    match omegaflow::uvfits::beat_rows(bytes) {
        Some(rows) => {
            for r in rows {
                println!(
                    "row t={:.6} value={:.6e} freq={:.6e} bin_width={:.3e} phase={:?} comp={}",
                    r.t, r.value, r.freq, r.bin_width, r.phase, r.comp
                );
            }
        }
        None => eprintln!("{name}: beat rows void"),
    }
}

fn run(path: &str, pair: (&str, &str), out: Option<&str>, ci_mode: bool) {
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("eht_uvfits_compiler: {path}: open returned void: {e}");
            exit(2);
        }
    };
    let mut members_seen = 0usize;
    let mut written = false;
    let result = gunzip_tar_members(
        file,
        |name| name.ends_with(".FITS"),
        |name, bytes| {
            members_seen += 1;
            match out {
                Some(out_path) => {
                    if written {
                        return;
                    }
                    if !pair_readable(bytes, pair) {
                        return;
                    }
                    match std::fs::write(out_path, bytes) {
                        Ok(()) => {
                            written = true;
                            eprintln!(
                                "{name}: {}-{} beat open, {} B -> {out_path}",
                                pair.0,
                                pair.1,
                                bytes.len()
                            );
                        }
                        Err(e) => eprintln!("{out_path}: write returned void: {e}"),
                    }
                }
                None => report_member(name, bytes, pair),
            }
        },
    );
    if members_seen == 0 {
        match result {
            Ok(_) => eprintln!("eht_uvfits_compiler: {path}: no .FITS member"),
            Err(e) => eprintln!("eht_uvfits_compiler: {path}: gzip walk void: {e}"),
        }
        exit(1);
    }
    let Some(out_path) = out else {
        println!("{path}: {members_seen} .FITS members walked");
        return;
    };
    if !written {
        eprintln!(
            "eht_uvfits_compiler: {path}: no {}-{} beat member — {out_path} stays unwritten",
            pair.0, pair.1
        );
        exit(1);
    }
    let bytes = read_file(out_path);
    if !pair_readable(&bytes, pair) {
        eprintln!("eht_uvfits_compiler: {out_path}: roundtrip pair gate void");
        exit(1);
    }
    println!(
        "{out_path}: {} B, {}-{} beat open — roundtrip parses",
        bytes.len(),
        pair.0,
        pair.1
    );
    if ci_mode && !upload_release(NETLOC, out_path) {
        exit(1);
    }
}

fn run_fits(path: &str, pair: (&str, &str)) {
    let bytes = read_file(path);
    report_member(path, &bytes, pair);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("--inspect") => match arg_value(&args, "--inspect") {
            Some(p) => inspect(&p),
            None => {
                eprintln!("eht_uvfits_compiler: {USAGE}");
                exit(2);
            }
        },
        Some("--verify") => match arg_value(&args, "--verify") {
            Some(p) => verify(&p),
            None => {
                eprintln!("eht_uvfits_compiler: {USAGE}");
                exit(2);
            }
        },
        Some("--cat") => match (arg_value(&args, "--cat"), args.get(3)) {
            (Some(p), Some(name)) => cat_member(&p, name),
            _ => {
                eprintln!("eht_uvfits_compiler: --cat <file.tgz> <member>");
                exit(2);
            }
        },
        Some("--run") => match arg_value(&args, "--run") {
            Some(p) => {
                let pair = pair_arg(&args);
                let out = arg_value(&args, "--out");
                let ci_mode = args.iter().any(|a| a == "--ci-mode");
                run(
                    &p,
                    (pair.0.as_str(), pair.1.as_str()),
                    out.as_deref(),
                    ci_mode,
                );
            }
            None => {
                eprintln!("eht_uvfits_compiler: {USAGE}");
                exit(2);
            }
        },
        Some("--runfits") => match arg_value(&args, "--runfits") {
            Some(p) => {
                let pair = pair_arg(&args);
                run_fits(&p, (pair.0.as_str(), pair.1.as_str()));
            }
            None => {
                eprintln!("eht_uvfits_compiler: {USAGE}");
                exit(2);
            }
        },
        _ => {
            eprintln!("eht_uvfits_compiler: {USAGE}");
            exit(2);
        }
    }
}
