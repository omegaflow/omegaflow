use omegaflow::archivar::asset_path::compiled;
use omegaflow::archivar::embedded_lsk;
use omegaflow::inflate::inflate_stream;
use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

const NETLOC: &str = "lasp.colorado.edu";
const DEFAULT_OUT: &str = "sorce_sim.bin";
const MAGIC: [u8; 4] = *b"SIMA";
const TARGETS: [f64; 3] = [240.0, 300.0, 380.0];
const EOCD: &[u8; 4] = b"PK\x05\x06";
const CENTRAL: &[u8; 4] = b"PK\x01\x02";
const LOCAL: &[u8; 4] = b"PK\x03\x04";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn positional_source(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if matches!(a.as_str(), "--source" | "--out") {
            i += 2;
            continue;
        }
        if a.starts_with('-') {
            i += 1;
            continue;
        }
        return Some(a.clone());
    }
    None
}

fn le16(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(off..off + 2)?.try_into().ok()?))
}

fn le32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

struct Source {
    url: Option<String>,
    path: Option<PathBuf>,
    len: u64,
}

impl Source {
    fn open(spec: &str) -> Result<Source, String> {
        if spec.starts_with("http://") || spec.starts_with("https://") {
            let out = std::process::Command::new("curl")
                .args(["-sSIL", "--retry", "3", "--max-time", "120", spec])
                .output()
                .map_err(|e| format!("{spec}: head void ({e})"))?;
            if !out.status.success() {
                return Err(format!(
                    "{spec}: head void ({})",
                    String::from_utf8_lossy(&out.stderr).trim()
                ));
            }
            let text = String::from_utf8_lossy(&out.stdout);
            let len = text
                .lines()
                .filter_map(|l| {
                    let (k, v) = l.split_once(':')?;
                    if k.trim().eq_ignore_ascii_case("content-length") {
                        v.trim().parse::<u64>().ok()
                    } else {
                        None
                    }
                })
                .last()
                .ok_or_else(|| format!("{spec}: no content-length"))?;
            Ok(Source {
                url: Some(spec.to_string()),
                path: None,
                len,
            })
        } else {
            let path = PathBuf::from(spec);
            let len = std::fs::metadata(&path)
                .map_err(|e| format!("{spec}: metadata void ({e})"))?
                .len();
            Ok(Source {
                url: None,
                path: Some(path),
                len,
            })
        }
    }

    fn read_at(&self, off: u64, len: usize) -> Result<Vec<u8>, String> {
        if len == 0 {
            return Ok(Vec::new());
        }
        let end = off + len as u64 - 1;
        if end >= self.len {
            return Err(format!(
                "range {off}-{end} exceeds the {}-byte source",
                self.len
            ));
        }
        if let Some(url) = &self.url {
            let out = std::process::Command::new("curl")
                .args(["-sS", "-L", "--retry", "3", "--max-time", "300", "-H"])
                .arg(format!("Range: bytes={off}-{end}"))
                .arg(url)
                .output()
                .map_err(|e| format!("{url}: range void ({e})"))?;
            if !out.status.success() {
                return Err(format!(
                    "{url}: range {off}-{end} void ({})",
                    String::from_utf8_lossy(&out.stderr).trim()
                ));
            }
            if out.stdout.len() != len {
                return Err(format!(
                    "{url}: range {off}-{end} returned {} bytes, not {len}",
                    out.stdout.len()
                ));
            }
            Ok(out.stdout)
        } else {
            let path = self.path.as_ref().ok_or("source has no path")?;
            let mut f = File::open(path).map_err(|e| format!("{}: open void ({e})", path.display()))?;
            f.seek(SeekFrom::Start(off))
                .map_err(|e| format!("{}: seek void ({e})", path.display()))?;
            let mut buf = vec![0u8; len];
            f.read_exact(&mut buf)
                .map_err(|e| format!("{}: read void ({e})", path.display()))?;
            Ok(buf)
        }
    }

    fn reader(&self, off: u64, len: u64) -> RangedReader<'_> {
        RangedReader {
            source: self,
            pos: off,
            end: off + len,
            buf: Vec::new(),
            buf_pos: 0,
            chunk: 4 << 20,
        }
    }
}

struct RangedReader<'a> {
    source: &'a Source,
    pos: u64,
    end: u64,
    buf: Vec<u8>,
    buf_pos: usize,
    chunk: u64,
}

impl Read for RangedReader<'_> {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        if self.buf_pos >= self.buf.len() && self.pos < self.end {
            let want = (self.end - self.pos).min(self.chunk) as usize;
            match self.source.read_at(self.pos, want) {
                Ok(buf) => {
                    self.pos += buf.len() as u64;
                    self.buf = buf;
                    self.buf_pos = 0;
                }
                Err(e) => return Err(std::io::Error::other(e)),
            }
        }
        let avail = self.buf.len() - self.buf_pos;
        let n = avail.min(out.len());
        out[..n].copy_from_slice(&self.buf[self.buf_pos..self.buf_pos + n]);
        self.buf_pos += n;
        Ok(n)
    }
}

struct Entry {
    name: String,
    method: u16,
    comp_size: u64,
    local_off: u64,
}

fn eocd_at(bytes: &[u8]) -> Option<(usize, u64, u64)> {
    let mut i = bytes.len().checked_sub(22)?;
    loop {
        if bytes.get(i..i + 4) == Some(EOCD) {
            let entries = le16(bytes, i + 10)? as u64;
            let size = le32(bytes, i + 12)? as u64;
            let off = le32(bytes, i + 16)? as u64;
            return Some((entries as usize, off, size));
        }
        if i == 0 {
            return None;
        }
        i -= 1;
    }
}

fn central_entries(cd: &[u8], count: usize) -> Option<Vec<Entry>> {
    let mut entries = Vec::new();
    let mut off = 0usize;
    while entries.len() < count {
        if cd.get(off..off + 4)? != CENTRAL {
            return None;
        }
        let method = le16(cd, off + 10)?;
        let comp_size = le32(cd, off + 20)? as u64;
        let name_len = le16(cd, off + 28)? as usize;
        let extra_len = le16(cd, off + 30)? as usize;
        let comment_len = le16(cd, off + 32)? as usize;
        let local_off = le32(cd, off + 42)? as u64;
        let name_end = off + 46 + name_len;
        let name = std::str::from_utf8(cd.get(off + 46..name_end)?).ok()?;
        entries.push(Entry {
            name: name.to_string(),
            method,
            comp_size,
            local_off,
        });
        off = name_end + extra_len + comment_len;
    }
    Some(entries)
}

fn member_entry(source: &Source) -> Result<Entry, String> {
    let tail_len = source.len.min(22 + 65_535) as usize;
    let tail_off = source.len - tail_len as u64;
    let tail = source.read_at(tail_off, tail_len)?;
    let (count, cd_off, cd_size) = eocd_at(&tail)
        .ok_or_else(|| "zip carries no end-of-central-directory".to_string())?;
    if cd_off + cd_size > source.len {
        return Err("central directory points beyond the source".to_string());
    }
    let cd = source.read_at(cd_off, cd_size as usize)?;
    let entries = central_entries(&cd, count)
        .ok_or_else(|| "central directory does not parse".to_string())?;
    entries
        .into_iter()
        .find(|e| e.name.to_ascii_lowercase().ends_with(".txt"))
        .ok_or_else(|| "zip carries no .txt member".to_string())
}

fn data_offset(source: &Source, entry: &Entry) -> Result<u64, String> {
    let head = source.read_at(entry.local_off, 30)?;
    if head.get(0..4) != Some(LOCAL) {
        return Err("local header carries no PK\\x03\\x04 magic".to_string());
    }
    let name_len = le16(&head, 26).ok_or("local name length absent")? as u64;
    let extra_len = le16(&head, 28).ok_or("local extra length absent")? as u64;
    Ok(entry.local_off + 30 + name_len + extra_len)
}

struct SimParser {
    line: Vec<u8>,
    day: Option<f64>,
    best: [(f64, f64); 3],
    records: Vec<(f64, f64, u32)>,
    rows: usize,
}

impl SimParser {
    fn new() -> SimParser {
        SimParser {
            line: Vec::new(),
            day: None,
            best: [(f64::INFINITY, 0.0); 3],
            records: Vec::new(),
            rows: 0,
        }
    }

    fn push(&mut self, chunk: &[u8], lsk: &omegaflow::archivar::LeapSeconds) {
        for &b in chunk {
            if b == b'\n' {
                let line = std::mem::take(&mut self.line);
                if let Ok(text) = std::str::from_utf8(&line) {
                    self.on_line(text, lsk);
                }
            } else if b != b'\r' && self.line.len() < 4096 {
                self.line.push(b);
            }
        }
    }

    fn on_line(&mut self, line: &str, lsk: &omegaflow::archivar::LeapSeconds) {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            return;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 9 {
            return;
        }
        let Ok(jdn) = cols[1].parse::<f64>() else {
            return;
        };
        let (Ok(min_wl), Ok(max_wl)) = (cols[2].parse::<f64>(), cols[3].parse::<f64>()) else {
            return;
        };
        let Ok(irradiance) = cols[6].parse::<f64>() else {
            return;
        };
        if self.day != Some(jdn) {
            self.flush(lsk);
            self.day = Some(jdn);
            self.best = [(f64::INFINITY, 0.0); 3];
        }
        let center = 0.5 * (min_wl + max_wl);
        for (idx, target) in TARGETS.iter().enumerate() {
            let d = (center - target).abs();
            if d < self.best[idx].0 {
                self.best[idx] = (d, irradiance);
            }
        }
        self.rows += 1;
    }

    fn flush(&mut self, lsk: &omegaflow::archivar::LeapSeconds) {
        let Some(jdn) = self.day else { return };
        let unix = (jdn - 2_440_587.5) * 86_400.0;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            return;
        };
        for (idx, (_, value)) in self.best.iter().enumerate() {
            if value.is_finite() && *value > 0.0 {
                self.records.push((tdb, *value, idx as u32));
            }
        }
    }

    fn finish(mut self, lsk: &omegaflow::archivar::LeapSeconds) -> Vec<(f64, f64, u32)> {
        self.flush(lsk);
        self.records
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let source_spec = arg_value(args, "--source")
        .or_else(|| positional_source(args))
        .ok_or_else(|| "no source — pass the SORCE-SIM .txt.zip path or URL".to_string())?;
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => {
            let path = compiled(NETLOC, DEFAULT_OUT);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let source = Source::open(&source_spec)?;
    let entry = member_entry(&source)?;
    if entry.method != 8 && entry.method != 0 {
        return Err(format!("{}: compression method {} unread", entry.name, entry.method));
    }
    let start = data_offset(&source, &entry)?;
    let lsk = embedded_lsk().ok_or_else(|| "the embedded leap seconds stay unread".to_string())?;
    let mut parser = SimParser::new();
    let reader = source.reader(start, entry.comp_size);
    if entry.method == 0 {
        let mut reader = reader;
        let mut buf = [0u8; 65536];
        loop {
            let n = reader
                .read(&mut buf)
                .map_err(|e| format!("{}: read void ({e})", entry.name))?;
            if n == 0 {
                break;
            }
            parser.push(&buf[..n], &lsk);
        }
    } else {
        inflate_stream(reader, |chunk| parser.push(chunk, &lsk))
            .map_err(|e| format!("{}: inflate void ({e})", entry.name))?;
    }
    let rows = parser.rows;
    let mut records = parser.finish(&lsk);
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    records.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
    if records.is_empty() {
        return Err(format!("{}: no records — the bin stays unwritten", entry.name));
    }
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, idx) in &records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&idx.to_le_bytes());
    }
    std::fs::write(&out, &buf).map_err(|e| format!("{out}: write void ({e})"))?;
    eprintln!(
        "{out}: {} records ({} B) from {} rows of {}",
        records.len(),
        buf.len(),
        rows,
        entry.name
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("usage: sorce_sim_compiler <sorce_sim_...txt.zip> [--out <bin>] [--source <path|url>]");
        return;
    }
    if let Err(e) = run(&args) {
        eprintln!("sorce_sim: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::embedded_lsk;

    fn lsk() -> omegaflow::archivar::LeapSeconds {
        embedded_lsk().expect("leap seconds")
    }

    #[test]
    fn parses_a_daily_spectrum_excerpt_and_keeps_the_nearest_bin() {
        let text = "\
; SORCE Spectral Irradiance
; ***DATA RECORDS***, number = 12
20030414.5 2452744.0  240.02  240.02 43 27 4.133813e-02 3.9931e-04    64.0
20030414.5 2452744.0  245.00  245.00 43 27 1.000000e-01 1.0000e-03    64.0
20030414.5 2452744.0  300.00  300.00 43 27 5.000000e-01 1.0000e-03    64.0
20030414.5 2452744.0  380.00  380.00 43 27 6.000000e-01 1.0000e-03    64.0
20030415.5 2452745.0  240.50  240.50 43 27 0.000000e+00 0.0000e+00    64.0
20030415.5 2452745.0  380.00  380.00 43 27 6.100000e-01 1.0000e-03    64.0
";
        let mut p = SimParser::new();
        p.push(text.as_bytes(), &lsk());
        let recs = p.finish(&lsk());
        assert_eq!(recs.len(), 4);
        assert_eq!(recs[0].2, 0);
        assert!((recs[0].1 - 0.04133813).abs() < 1e-12);
        assert!((recs[1].1 - 0.5).abs() < 1e-12);
        assert!((recs[2].1 - 0.6).abs() < 1e-12);
        assert_eq!(recs[3].2, 2);
        assert!(recs[3].1 > 0.6);
        assert!(recs[3].0 > recs[0].0);
    }

    #[test]
    fn eocd_and_central_directory_offsets_are_read() {
        let mut eocd = vec![0u8; 22];
        eocd[..4].copy_from_slice(EOCD);
        eocd[10..12].copy_from_slice(&1u16.to_le_bytes());
        eocd[12..16].copy_from_slice(&46u32.to_le_bytes());
        eocd[16..20].copy_from_slice(&7u32.to_le_bytes());
        let (count, off, size) = eocd_at(&eocd).expect("eocd");
        assert_eq!((count, off, size), (1, 7, 46));

        let mut cd = vec![0u8; 46 + 5];
        cd[..4].copy_from_slice(CENTRAL);
        cd[10..12].copy_from_slice(&8u16.to_le_bytes());
        cd[20..24].copy_from_slice(&123u32.to_le_bytes());
        cd[28..30].copy_from_slice(&5u16.to_le_bytes());
        cd[42..46].copy_from_slice(&99u32.to_le_bytes());
        cd[46..51].copy_from_slice(b"a.txt");
        let entries = central_entries(&cd, 1).expect("central");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "a.txt");
        assert_eq!(entries[0].comp_size, 123);
        assert_eq!(entries[0].local_off, 99);
    }
}
