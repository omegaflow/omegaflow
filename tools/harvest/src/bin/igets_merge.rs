use omegaflow::archivar::geo::{verify_bin, GeoRec, MAGIC_IGETS, REC_BYTES};
use omegaflow::cdn::upload_release;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const NETLOC: &str = "igetsftp.gfz.de";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

#[derive(PartialEq)]
struct OrdF64(f64);

impl Eq for OrdF64 {}

impl PartialOrd for OrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

struct Shard {
    reader: BufReader<File>,
    remaining: usize,
    prev_t: Option<f64>,
}

impl Shard {
    fn open(path: &Path) -> Option<(Shard, usize)> {
        let file = File::open(path).ok()?;
        let mut reader = BufReader::new(file);
        let mut header = [0u8; 8];
        reader.read_exact(&mut header).ok()?;
        if header[0..4] != MAGIC_IGETS {
            return None;
        }
        let count = u32::from_le_bytes(header[4..8].try_into().ok()?) as usize;
        Some((
            Shard {
                reader,
                remaining: count,
                prev_t: None,
            },
            count,
        ))
    }

    fn next(&mut self) -> Option<GeoRec> {
        if self.remaining == 0 {
            return None;
        }
        let mut buf = [0u8; REC_BYTES];
        self.reader.read_exact(&mut buf).ok()?;
        let rec = rec_from_bytes(&buf)?;
        if !rec.t.is_finite() || !rec.val.is_finite() {
            return None;
        }
        if let Some(prev) = self.prev_t {
            if rec.t < prev {
                return None;
            }
        }
        self.prev_t = Some(rec.t);
        self.remaining -= 1;
        Some(rec)
    }
}

fn rec_from_bytes(b: &[u8]) -> Option<GeoRec> {
    if b.len() < REC_BYTES {
        return None;
    }
    let f64_of = |off: usize| {
        b.get(off..off + 8)
            .and_then(|x| x.try_into().ok())
            .map(f64::from_le_bytes)
    };
    Some(GeoRec {
        t: f64_of(0)?,
        lat: f64_of(8)?,
        lon: f64_of(16)?,
        alt: f64_of(24)?,
        freq: f64_of(32)?,
        bin_width: f64_of(40)?,
        val: f64_of(48)?,
        comp: u32::from_le_bytes(b.get(56..60)?.try_into().ok()?),
        station: 0,
    })
}

fn write_rec(out: &mut impl Write, r: &GeoRec) -> std::io::Result<()> {
    out.write_all(&r.t.to_le_bytes())?;
    out.write_all(&r.lat.to_le_bytes())?;
    out.write_all(&r.lon.to_le_bytes())?;
    out.write_all(&r.alt.to_le_bytes())?;
    out.write_all(&r.freq.to_le_bytes())?;
    out.write_all(&r.bin_width.to_le_bytes())?;
    out.write_all(&r.val.to_le_bytes())?;
    out.write_all(&r.comp.to_le_bytes())
}

fn shard_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(shard_files(&path));
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with("igets-") && name.ends_with(".bin") {
                out.push(path);
            }
        }
    }
    out
}

fn station_of(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_str()?;
    let stem = name.strip_prefix("igets-")?.strip_suffix(".bin")?;
    if stem.is_empty() {
        None
    } else {
        Some(stem.to_string())
    }
}

fn expected_stations() -> Vec<String> {
    let Ok(raw) = std::env::var("IGETS_STATIONS") else {
        return Vec::new();
    };
    raw.trim_matches(|c: char| c == '[' || c == ']' || c == '"' || c.is_whitespace())
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(in_dir) = arg_value(&args, "--in") else {
        eprintln!("--in <dir> required");
        std::process::exit(1);
    };
    let Some(stations) = arg_value(&args, "--stations").and_then(|v| v.parse::<usize>().ok()) else {
        eprintln!("--stations <n> required");
        std::process::exit(1);
    };
    let Some(out_bin) = arg_value(&args, "--out-bin") else {
        eprintln!("--out-bin <path> required");
        std::process::exit(1);
    };

    let mut files = shard_files(Path::new(&in_dir));
    files.sort();
    if files.len() != stations {
        eprintln!(
            "{} shards flow, {stations} stations named — the merge stays unwritten",
            files.len()
        );
        let present: Vec<String> = files.iter().filter_map(|p| station_of(p)).collect();
        let expected = expected_stations();
        if expected.is_empty() {
            eprintln!("shards present: {}", present.join(" "));
        } else {
            let missing: Vec<&str> = expected
                .iter()
                .filter(|s| !present.contains(s))
                .map(|s| s.as_str())
                .collect();
            eprintln!("missing stations: {}", missing.join(" "));
        }
        std::process::exit(1);
    }

    let mut shards = Vec::new();
    let mut total = 0usize;
    for path in &files {
        let Some((shard, count)) = Shard::open(path) else {
            eprintln!("{}: header void — the merge stays unwritten", path.display());
            std::process::exit(1);
        };
        total += count;
        shards.push(shard);
    }
    if total > u32::MAX as usize {
        eprintln!("{total} records exceed the u32 count field — the merge stays unwritten");
        std::process::exit(1);
    }

    let mut current: Vec<Option<GeoRec>> = Vec::with_capacity(shards.len());
    let mut heap: BinaryHeap<(Reverse<OrdF64>, usize)> = BinaryHeap::new();
    for (i, shard) in shards.iter_mut().enumerate() {
        match shard.next() {
            Some(rec) => {
                heap.push((Reverse(OrdF64(rec.t)), i));
                current.push(Some(rec));
            }
            None => {
                if shard.remaining > 0 {
                    eprintln!("shard {i}: a record stays void — the merge stays unwritten");
                    std::process::exit(1);
                }
                current.push(None);
            }
        }
    }

    let Ok(file) = File::create(&out_bin) else {
        eprintln!("write {out_bin} returned void");
        std::process::exit(1);
    };
    let mut out = BufWriter::new(file);
    if out.write_all(&MAGIC_IGETS).is_err() || out.write_all(&(total as u32).to_le_bytes()).is_err() {
        eprintln!("write {out_bin} returned void");
        std::process::exit(1);
    }
    while let Some((_, i)) = heap.pop() {
        let Some(rec) = current[i].take() else {
            eprintln!("shard {i}: a record stays void — the merge stops");
            std::process::exit(1);
        };
        if write_rec(&mut out, &rec).is_err() {
            eprintln!("write {out_bin} returned void");
            std::process::exit(1);
        }
        match shards[i].next() {
            Some(next) => {
                heap.push((Reverse(OrdF64(next.t)), i));
                current[i] = Some(next);
            }
            None => {
                if shards[i].remaining > 0 {
                    eprintln!("shard {i}: a record stays void — the merge stops");
                    std::process::exit(1);
                }
            }
        }
    }
    if out.flush().is_err() {
        eprintln!("write {out_bin} returned void");
        std::process::exit(1);
    }
    drop(out);

    let Ok(bytes) = std::fs::read(&out_bin) else {
        eprintln!("{out_bin}: unreadable — the bin stays unverified");
        std::process::exit(1);
    };
    match verify_bin(MAGIC_IGETS, &bytes) {
        Some(n) if n == total => eprintln!(
            "{out_bin}: {n} geo records ({} shards), streamed verify holds",
            files.len()
        ),
        Some(n) => {
            eprintln!("{out_bin}: {n} of {total} records verify — the bin stays unverified");
            std::process::exit(1);
        }
        None => {
            eprintln!("{out_bin}: streamed verify void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}
