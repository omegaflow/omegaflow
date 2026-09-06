use omegaflow::json::{parse_json, JsonVal};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

const ROOT: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const CHUNK_DIR: &str = "phi/pipeline";
const CAP: usize = 2_000_000;
const ATTEMPTS: u32 = 3;
const RETRY_SECS: u64 = 60;

struct Band {
    lo: String,
    hi: String,
}

struct Catalog {
    name: &'static str,
    table: &'static str,
    columns: &'static str,
    skip_null: &'static str,
    ra_col: &'static str,
    bands: Vec<Band>,
}

fn py_fmt(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{:.1}", v)
    } else {
        format!("{}", v)
    }
}

fn band_list(step: f64, count: usize, int_labels: bool) -> Vec<Band> {
    (0..count)
        .map(|k| {
            let lo_v = step * k as f64;
            let hi_v = lo_v + step;
            let lo = if int_labels {
                format!("{}", lo_v as i64)
            } else {
                py_fmt(lo_v)
            };
            let hi = if int_labels {
                format!("{}", hi_v as i64)
            } else {
                py_fmt(hi_v)
            };
            Band { lo, hi }
        })
        .collect()
}

enum LoadState {
    Rows(Vec<bool>),
    Empty,
    Unparsable,
}

fn row_has_dist(row: &JsonVal) -> bool {
    match row {
        JsonVal::Obj(m) => match m.get("dist_pc") {
            Some(v) => !matches!(v, JsonVal::Null),
            None => false,
        },
        _ => false,
    }
}

fn load_flags(path: &Path) -> LoadState {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return LoadState::Unparsable,
    };
    match parse_json(&text) {
        Some(JsonVal::Arr(rows)) if rows.is_empty() => LoadState::Empty,
        Some(JsonVal::Arr(rows)) => LoadState::Rows(rows.iter().map(row_has_dist).collect()),
        _ => LoadState::Unparsable,
    }
}

fn where_clause(cat: &Catalog, band: &Band) -> String {
    format!(
        "t.\"{}\" >= {} AND t.\"{}\" < {}",
        cat.ra_col, band.lo, cat.ra_col, band.hi
    )
}

fn scan_value(bytes: &[u8], start: usize) -> Option<usize> {
    let n = bytes.len();
    let mut i = start;
    if i >= n {
        return None;
    }
    match bytes[i] {
        b'"' => {
            let mut esc = false;
            i += 1;
            while i < n {
                if esc {
                    esc = false;
                } else if bytes[i] == b'\\' {
                    esc = true;
                } else if bytes[i] == b'"' {
                    return Some(i + 1);
                }
                i += 1;
            }
            None
        }
        b'{' | b'[' => {
            let mut depth = 0usize;
            let mut in_str = false;
            let mut esc = false;
            while i < n {
                let b = bytes[i];
                if in_str {
                    if esc {
                        esc = false;
                    } else if b == b'\\' {
                        esc = true;
                    } else if b == b'"' {
                        in_str = false;
                    }
                } else {
                    match b {
                        b'"' => in_str = true,
                        b'{' | b'[' => depth += 1,
                        b'}' | b']' => {
                            depth -= 1;
                            if depth == 0 {
                                return Some(i + 1);
                            }
                        }
                        _ => {}
                    }
                }
                i += 1;
            }
            None
        }
        _ => {
            while i < n {
                let b = bytes[i];
                if b == b',' || b == b']' || b == b'}' || (b as char).is_whitespace() {
                    return Some(i);
                }
                i += 1;
            }
            Some(i)
        }
    }
}

fn split_rows(bytes: &[u8]) -> Option<Vec<(usize, usize)>> {
    let n = bytes.len();
    let mut i = 0usize;
    while i < n && bytes[i] != b'[' {
        i += 1;
    }
    if i >= n {
        return None;
    }
    i += 1;
    let mut ranges = Vec::new();
    loop {
        while i < n && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= n {
            return None;
        }
        if bytes[i] == b']' {
            break;
        }
        let start = i;
        let end = scan_value(bytes, i)?;
        if end > n {
            return None;
        }
        ranges.push((start, end));
        i = end;
        while i < n && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= n {
            return None;
        }
        if bytes[i] == b',' {
            i += 1;
        } else if bytes[i] == b']' {
            break;
        } else {
            return None;
        }
    }
    Some(ranges)
}

fn fetch_band(tap: &Path, cat: &Catalog, band: &Band, out: &Path) -> Option<Vec<bool>> {
    for attempt in 0..ATTEMPTS {
        let run = Command::new(tap)
            .arg("--root")
            .arg(ROOT)
            .arg("--table")
            .arg(cat.table)
            .arg("--columns")
            .arg(cat.columns)
            .arg("--skip-null")
            .arg(cat.skip_null)
            .arg("--crossmatch")
            .arg("I/355/gaiadr3:RA_ICRS:DE_ICRS:Dist")
            .arg("--crossmatch-pm")
            .arg("pmRA:pmDE:Plx:RV:Teff:BPmag:RPmag:Gmag")
            .arg("--where")
            .arg(where_clause(cat, band))
            .arg("--out")
            .arg(out)
            .output();
        let (rc, stderr) = match run {
            Ok(o) => {
                let code = match o.status.code() {
                    Some(c) => c,
                    None => -1,
                };
                (code, String::from_utf8_lossy(&o.stderr).into_owned())
            }
            Err(_) => {
                eprintln!("tap_compiler absent: {}", tap.display());
                return None;
            }
        };
        let mut rows: Option<Vec<bool>> = None;
        if out.exists() {
            match load_flags(out) {
                LoadState::Rows(flags) => rows = Some(flags),
                LoadState::Unparsable => {
                    let _ = fs::remove_file(out);
                }
                LoadState::Empty => {}
            }
        }
        if rc == 0 {
            if let Some(flags) = rows {
                return Some(flags);
            }
        }
        let tail: String = {
            let chars: Vec<char> = stderr.chars().collect();
            let len = chars.len();
            if len > 200 {
                chars[len - 200..].iter().collect()
            } else {
                chars.into_iter().collect()
            }
        };
        let shown = match &rows {
            Some(flags) => flags.len(),
            None => 0,
        };
        println!(
            "  band {} attempt {}: rc={} rows={} {}",
            band.lo,
            attempt + 1,
            rc,
            shown,
            tail
        );
        if attempt + 1 < ATTEMPTS {
            thread::sleep(Duration::from_secs(RETRY_SECS));
        }
    }
    None
}

fn push_merge(file: &mut fs::File, merged: &Path, bytes: &[u8]) -> Result<(), String> {
    file.write_all(bytes)
        .map_err(|e| format!("write {}: {}", merged.display(), e))
}

fn merge_catalog(
    cat: &Catalog,
    chunk_dir: &Path,
    saved: &[(PathBuf, Vec<bool>)],
) -> Result<(), String> {
    let merged = chunk_dir.join(format!("{}.json", cat.name));
    let mut file =
        fs::File::create(&merged).map_err(|e| format!("create {}: {}", merged.display(), e))?;
    push_merge(&mut file, &merged, b"[")?;
    let mut total = 0usize;
    let mut dist = 0usize;
    let mut first = true;
    for (path, flags) in saved {
        if total >= CAP {
            break;
        }
        let bytes = fs::read(path).map_err(|e| format!("read {}: {}", path.display(), e))?;
        let ranges = match split_rows(&bytes) {
            Some(r) => r,
            None => return Err(format!("split {}: no array boundaries", path.display())),
        };
        if ranges.len() != flags.len() {
            return Err(format!(
                "{}: {} row values, {} parsed rows",
                path.display(),
                ranges.len(),
                flags.len()
            ));
        }
        let take = (CAP - total).min(flags.len());
        for i in 0..take {
            if !first {
                push_merge(&mut file, &merged, b",")?;
            }
            first = false;
            let (s, e) = ranges[i];
            push_merge(&mut file, &merged, &bytes[s..e])?;
            if flags[i] {
                dist += 1;
            }
        }
        total += take;
    }
    push_merge(&mut file, &merged, b"]")?;
    println!("{}: TOTAL {} rows, {} dist_pc", cat.name, total, dist);
    for band in &cat.bands {
        let part = chunk_dir.join(format!("{}_c{}.json", cat.name, band.lo));
        let _ = fs::remove_file(&part);
    }
    Ok(())
}

fn compile_catalog(cat: &Catalog, chunk_dir: &Path, tap: &Path) {
    let mut saved: Vec<(PathBuf, Vec<bool>)> = Vec::new();
    let mut failed = 0usize;
    for band in &cat.bands {
        let out = chunk_dir.join(format!("{}_c{}.json", cat.name, band.lo));
        let mut resumed = false;
        let flags = if out.exists() {
            match load_flags(&out) {
                LoadState::Rows(f) => {
                    resumed = true;
                    f
                }
                _ => {
                    let _ = fs::remove_file(&out);
                    match fetch_band(tap, cat, band, &out) {
                        Some(f) => f,
                        None => Vec::new(),
                    }
                }
            }
        } else {
            match fetch_band(tap, cat, band, &out) {
                Some(f) => f,
                None => Vec::new(),
            }
        };
        if resumed {
            println!("resume {} RA {}: {} rows", cat.name, band.lo, flags.len());
            saved.push((out, flags));
        } else {
            let dist = flags.iter().filter(|f| **f).count();
            println!(
                "{} RA {}-{}: {} rows, dist={}",
                cat.name,
                band.lo,
                band.hi,
                flags.len(),
                dist
            );
            if flags.is_empty() {
                failed += 1;
            } else {
                saved.push((out, flags));
            }
        }
    }
    if failed > 0 {
        println!(
            "{}: {} bands void — no merge, a restart fetches the missing bands",
            cat.name, failed
        );
        return;
    }
    if let Err(msg) = merge_catalog(cat, chunk_dir, &saved) {
        eprintln!("{}", msg);
        let _ = fs::remove_file(chunk_dir.join(format!("{}.json", cat.name)));
        std::process::exit(1);
    }
}

fn tap_compiler_path(chunk_root: &Path) -> Option<PathBuf> {
    let sibling = env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.join("tap_compiler")))?;
    if sibling.exists() {
        return Some(sibling);
    }
    let debug = chunk_root.join("target/debug/tap_compiler");
    if debug.exists() {
        Some(debug)
    } else {
        None
    }
}

fn main() {
    let chunk_root = match Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
    {
        Some(r) => r.to_path_buf(),
        None => {
            eprintln!(
                "workspace root not resolvable from {}",
                env!("CARGO_MANIFEST_DIR")
            );
            std::process::exit(1);
        }
    };
    let chunk_dir = chunk_root.join(CHUNK_DIR);
    let tap = match tap_compiler_path(&chunk_root) {
        Some(t) => t,
        None => {
            eprintln!(
                "tap_compiler absent: build omegaflow-harvest first (bin sibling of chunk_master)"
            );
            std::process::exit(1);
        }
    };
    let pastel = Catalog {
        name: "pastel",
        table: "B/pastel/pastel",
        columns: "ra:RAdeg;dec:DEdeg;teff:Teff;logg:logg;mag:Vmag",
        skip_null: "teff",
        ra_col: "RAdeg",
        bands: band_list(45.0, 8, true),
    };
    let wds = Catalog {
        name: "wds",
        table: "B/wds/wds",
        columns: "ra:RAJ2000;dec:DEJ2000;mag1:mag1;mag2:mag2;sep:sep1",
        skip_null: "mag1",
        ra_col: "RAJ2000",
        bands: band_list(45.0, 8, true),
    };
    let mktypes = Catalog {
        name: "mktypes",
        table: "B/mk/mktypes",
        columns: "ra:RAJ2000;dec:DEJ2000;mag:Mag",
        skip_null: "mag",
        ra_col: "RAJ2000",
        bands: band_list(4.5, 80, false),
    };
    let denis = Catalog {
        name: "denis",
        table: "B/denis/denis",
        columns: "ra:RAJ2000;dec:DEJ2000;imag:Imag;jmag:Jmag;kmag:Kmag",
        skip_null: "jmag",
        ra_col: "RAJ2000",
        bands: band_list(22.5, 16, false),
    };
    for cat in [pastel, wds, mktypes, denis] {
        compile_catalog(&cat, &chunk_dir, &tap);
    }
    println!("all done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_bands_label_as_ints() {
        let bands = band_list(45.0, 8, true);
        assert_eq!(bands.len(), 8);
        assert_eq!(bands[0].lo, "0");
        assert_eq!(bands[0].hi, "45");
        assert_eq!(bands[7].lo, "315");
        assert_eq!(bands[7].hi, "360");
    }

    #[test]
    fn fractional_bands_label_as_python_floats() {
        let bands = band_list(4.5, 80, false);
        assert_eq!(bands.len(), 80);
        assert_eq!(bands[0].lo, "0.0");
        assert_eq!(bands[0].hi, "4.5");
        assert_eq!(bands[2].lo, "9.0");
        assert_eq!(bands[79].lo, "355.5");
        assert_eq!(bands[79].hi, "360.0");
        let denis = band_list(22.5, 16, false);
        assert_eq!(denis[2].lo, "45.0");
        assert_eq!(denis[15].hi, "360.0");
    }

    #[test]
    fn split_rows_recovers_elements() {
        let src = "[{\"ra\":1,\"s\":\"x]y\"},{\"ra\":2},{\"ra\":[1,2]}]\n";
        let ranges = match split_rows(src.as_bytes()) {
            Some(r) => r,
            None => panic!("no ranges"),
        };
        assert_eq!(ranges.len(), 3);
        assert_eq!(
            &src.as_bytes()[ranges[0].0..ranges[0].1],
            b"{\"ra\":1,\"s\":\"x]y\"}"
        );
        assert_eq!(&src.as_bytes()[ranges[1].0..ranges[1].1], b"{\"ra\":2}");
        assert_eq!(&src.as_bytes()[ranges[2].0..ranges[2].1], b"{\"ra\":[1,2]}");
    }

    #[test]
    fn split_rows_empty_array() {
        let ranges = match split_rows(b"[]\n") {
            Some(r) => r,
            None => panic!("no ranges"),
        };
        assert!(ranges.is_empty());
    }

    #[test]
    fn dist_count_marks_present_non_null_only() {
        let text = "[{\"ra\":1,\"dist_pc\":2.0},{\"ra\":2,\"dist_pc\":null},{\"ra\":3}]";
        let arr = match parse_json(text) {
            Some(JsonVal::Arr(r)) => r,
            _ => panic!("no array"),
        };
        let flags: Vec<bool> = arr.iter().map(row_has_dist).collect();
        assert_eq!(flags, vec![true, false, false]);
    }
}
