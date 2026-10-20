use omegaflow::archivar::asset_path::compiled;
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Hdf5Datatype, Hdf5Layout, LazyHdf5, decode_f64};
use omegaflow::spectral::{parse_spectral_bin, write_spectral_bin};
use std::cell::Cell;
use std::fs::File;
#[cfg(unix)]
use std::os::unix::fs::FileExt;
use std::process::Command;

const NETLOC: &str = "galileo.sese.asu.edu";
const DEFAULT_OUT: &str = "edges_raw_spectrum.bin";
const DEFAULT_CHUNK: usize = 16;
const BASE_BYTES: u64 = 1_048_576;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Selection {
    Integration(usize),
    Range(usize, usize),
    Limit(usize),
    Mean,
}

struct Options {
    load: usize,
    mean_loads: bool,
    selection: Selection,
    chunk: usize,
    epoch_override: Option<f64>,
}

struct Source {
    kind: SourceKind,
    fetches: Cell<usize>,
    bytes: Cell<u64>,
}

enum SourceKind {
    #[cfg_attr(not(test), allow(dead_code))]
    Memory(Vec<u8>),
    Local {
        file: File,
        len: u64,
    },
    Http {
        url: String,
    },
}

impl Source {
    fn open(input: &str) -> Result<Self, String> {
        if input.starts_with("http://") || input.starts_with("https://") {
            return Ok(Source {
                kind: SourceKind::Http {
                    url: input.to_string(),
                },
                fetches: Cell::new(0),
                bytes: Cell::new(0),
            });
        }
        let file = File::open(input).map_err(|e| format!("open {input}: {e}"))?;
        let len = file
            .metadata()
            .map_err(|e| format!("stat {input}: {e}"))?
            .len();
        Ok(Source {
            kind: SourceKind::Local { file, len },
            fetches: Cell::new(0),
            bytes: Cell::new(0),
        })
    }

    #[cfg(test)]
    fn memory(bytes: Vec<u8>) -> Self {
        Source {
            kind: SourceKind::Memory(bytes),
            fetches: Cell::new(0),
            bytes: Cell::new(0),
        }
    }

    fn len(&self) -> u64 {
        match &self.kind {
            SourceKind::Memory(b) => b.len() as u64,
            SourceKind::Local { len, .. } => *len,
            SourceKind::Http { .. } => u64::MAX,
        }
    }

    fn read_at(&self, off: u64, len: u64) -> Result<Vec<u8>, String> {
        if len == 0 {
            return Ok(Vec::new());
        }
        self.fetches.set(self.fetches.get() + 1);
        self.bytes.set(self.bytes.get() + len);
        match &self.kind {
            SourceKind::Memory(bytes) => {
                let start = off as usize;
                let end = start
                    .checked_add(len as usize)
                    .ok_or("memory range overflow")?;
                if end > bytes.len() {
                    return Err(format!(
                        "memory range {start}..{end} outside {} B",
                        bytes.len()
                    ));
                }
                Ok(bytes[start..end].to_vec())
            }
            SourceKind::Local { file, .. } => {
                let mut buf = vec![0u8; len as usize];
                let mut done = 0usize;
                while done < buf.len() {
                    let n = file
                        .read_at(&mut buf[done..], off + done as u64)
                        .map_err(|e| format!("read_at {off}+{done}: {e}"))?;
                    if n == 0 {
                        return Err(format!("short read at {}+{}", off, done));
                    }
                    done += n;
                }
                Ok(buf)
            }
            SourceKind::Http { url } => http_read(url, off, len),
        }
    }
}

fn http_read(url: &str, start: u64, len: u64) -> Result<Vec<u8>, String> {
    let end = start
        .checked_add(len)
        .and_then(|v| v.checked_sub(1))
        .ok_or("http range overflow")?;
    let path = omegaflow::temp_path(&format!(
        "edges_gsh5_range_{}_{}_{}.bin",
        std::process::id(),
        start,
        len
    ));
    let status = Command::new("curl")
        .args([
            "-sS",
            "--retry",
            "5",
            "--retry-delay",
            "2",
            "--retry-all-errors",
            "--max-time",
            "600",
            "--speed-time",
            "120",
            "--speed-limit",
            "1024",
            "-r",
            &format!("{start}-{end}"),
            url,
            "-o",
            &path,
        ])
        .status()
        .map_err(|e| format!("curl {url}: {e}"))?;
    if !status.success() {
        let _ = std::fs::remove_file(&path);
        return Err(format!(
            "curl range {start}-{end} on {url} returned {status}"
        ));
    }
    let bytes = std::fs::read(&path).map_err(|e| format!("read {path}: {e}"))?;
    let _ = std::fs::remove_file(&path);
    if bytes.len() as u64 != len {
        return Err(format!(
            "http range {start}-{end} returned {} B, wanted {len}",
            bytes.len()
        ));
    }
    Ok(bytes)
}

fn dataset_info<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    file: &mut LazyHdf5<F>,
    name: &str,
) -> Result<(Hdf5Layout, Vec<u64>, Hdf5Datatype), String> {
    let (obj, ds, dt) = file
        .dataset(name)
        .map_err(|note| format!("{name}: dataset absent: {note:?}"))?;
    let layout = obj
        .layout
        .clone()
        .ok_or_else(|| format!("{name}: layout absent"))?;
    Ok((layout, ds.dims.clone(), dt.clone()))
}

fn read_contiguous(
    source: &Source,
    layout: &Hdf5Layout,
    dims: &[u64],
    dt: &Hdf5Datatype,
    start_elem: u64,
    count: u64,
) -> Result<Vec<f64>, String> {
    if dt.class != 1 || dt.size != 8 {
        return Err(format!(
            "datatype class {} size {} — only float64 is read",
            dt.class, dt.size
        ));
    }
    let total = dims.iter().fold(1u64, |a, d| a.saturating_mul(*d));
    let end_elem = start_elem
        .checked_add(count)
        .ok_or("element range overflow")?;
    if end_elem > total {
        return Err(format!(
            "element range {start_elem}..{end_elem} outside {total} elements"
        ));
    }
    let bytes = match layout {
        Hdf5Layout::Contiguous { addr, size } => {
            let expected = total.saturating_mul(8);
            if *size != 0 && *size != expected {
                return Err(format!(
                    "contiguous size {size} contradicts {total} x 8 = {expected}"
                ));
            }
            source.read_at(
                addr.saturating_add(start_elem.saturating_mul(8)),
                count.saturating_mul(8),
            )?
        }
        Hdf5Layout::Compact { data } => {
            let start = (start_elem as usize)
                .checked_mul(8)
                .ok_or("compact offset overflow")?;
            let end = start
                .checked_add(count as usize * 8)
                .ok_or("compact range overflow")?;
            if end > data.len() {
                return Err(format!(
                    "compact range {start}..{end} outside {} B",
                    data.len()
                ));
            }
            data[start..end].to_vec()
        }
        Hdf5Layout::Chunked { .. } => {
            return Err(
                "chunked layout is not read by this arm (the EDGES gsh5 files are contiguous)"
                    .into(),
            );
        }
    };
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        out.push(
            decode_f64(&bytes, i * 8, dt.endian)
                .ok_or_else(|| format!("float64 decode void at element {i}"))?,
        );
    }
    Ok(out)
}

fn read_rows(
    source: &Source,
    layout: &Hdf5Layout,
    dims: &[u64],
    dt: &Hdf5Datatype,
    load: usize,
    pol: usize,
    start: usize,
    count: usize,
) -> Result<Vec<f64>, String> {
    if dims.len() != 4 {
        return Err(format!(
            "data/data rank {} — expected 4 (load, pol, time, freq)",
            dims.len()
        ));
    }
    let (nload, npol, ntime, nfreq) = (
        dims[0] as usize,
        dims[1] as usize,
        dims[2] as usize,
        dims[3] as usize,
    );
    if load >= nload || pol >= npol {
        return Err(format!("load {load} / pol {pol} outside {nload} x {npol}"));
    }
    let end = start.checked_add(count).ok_or("time range overflow")?;
    if end > ntime {
        return Err(format!("time range {start}..{end} outside {ntime}"));
    }
    let flat = (((load * npol + pol) * ntime + start) as u64)
        .checked_mul(nfreq as u64)
        .ok_or("flat offset overflow")?;
    read_contiguous(
        source,
        layout,
        dims,
        dt,
        flat,
        (count as u64).saturating_mul(nfreq as u64),
    )
}

fn read_gsh5(source: &Source, opts: &Options) -> Result<(f64, Vec<(f64, f64, f64)>), String> {
    let base_len = BASE_BYTES.min(source.len());
    let base = source.read_at(0, base_len)?;
    let mut file = LazyHdf5::open(&base, |off, len| source.read_at(off, len).ok())
        .map_err(|note| format!("HDF5 parse void: {note:?}"))?;

    let (fl, fdims, fdt) = dataset_info(&mut file, "metadata/freqs")?;
    if fdims.len() != 1 {
        return Err(format!("metadata/freqs rank {} — expected 1", fdims.len()));
    }
    let nfreq = fdims[0] as usize;
    if nfreq == 0 {
        return Err("metadata/freqs is empty".into());
    }
    let freqs = read_contiguous(source, &fl, &fdims, &fdt, 0, nfreq as u64)?;

    let (tl, tdims, tdt) = dataset_info(&mut file, "metadata/times")?;
    let times = read_contiguous(
        source,
        &tl,
        &tdims,
        &tdt,
        0,
        tdims.iter().fold(1u64, |a, d| a.saturating_mul(*d)),
    )?;

    let (dl, ddims, ddt) = dataset_info(&mut file, "data/data")?;
    if ddims.len() != 4 {
        return Err(format!("data/data rank {} — expected 4", ddims.len()));
    }
    let (nload, npol, ntime, data_nfreq) = (
        ddims[0] as usize,
        ddims[1] as usize,
        ddims[2] as usize,
        ddims[3] as usize,
    );
    if data_nfreq != nfreq {
        return Err(format!(
            "data/data has {data_nfreq} channels, metadata/freqs has {nfreq}"
        ));
    }
    if nload == 0 || npol == 0 || ntime == 0 {
        return Err(format!("data/data shape {ddims:?} is void"));
    }

    let (start, end) = match opts.selection {
        Selection::Integration(i) => (i, i + 1),
        Selection::Range(s, e) => (s, e),
        Selection::Limit(n) => (0, n),
        Selection::Mean => (0, ntime),
    };
    if end > ntime {
        return Err(format!(
            "selection {start}..{end} outside {ntime} integrations"
        ));
    }
    if start >= end {
        return Err(format!("selection {start}..{end} is empty"));
    }

    let loads: Vec<usize> = if opts.mean_loads {
        (0..nload).collect()
    } else {
        if opts.load >= nload {
            return Err(format!("--load {} outside {nload} loads", opts.load));
        }
        vec![opts.load]
    };

    if opts.chunk == 0 {
        return Err("--chunk 0 carries no integrations".to_string());
    }
    let chunk = opts.chunk;
    let mut sums = vec![0.0f64; nfreq];
    let mut blocks = 0usize;
    for &load in &loads {
        let mut t = start;
        while t < end {
            let take = (end - t).min(chunk);
            let vals = read_rows(source, &dl, &ddims, &ddt, load, 0, t, take)?;
            for r in 0..take {
                let row = &vals[r * nfreq..(r + 1) * nfreq];
                for (sum, v) in sums.iter_mut().zip(row.iter()) {
                    *sum += *v;
                }
            }
            blocks += take;
            t += take;
        }
    }
    if blocks == 0 {
        return Err("no integration survived — the bin stays unwritten".into());
    }
    let values: Vec<f64> = sums.iter().map(|s| s / blocks as f64).collect();

    let max_freq = freqs.iter().cloned().fold(f64::MIN, f64::max);
    let unit_scale = if max_freq > 1e6 { 1.0 } else { 1e6 };
    let mut diffs: Vec<f64> = freqs
        .windows(2)
        .map(|w| (w[1] - w[0]).abs() * unit_scale)
        .filter(|d| d.is_finite() && *d > 0.0)
        .collect();
    diffs.sort_by(|a, b| a.total_cmp(b));
    let bin_width = match diffs.get(diffs.len() / 2).copied() {
        Some(v) => v,
        None => return Err("frequency grid carries no finite width".to_string()),
    };
    let bins: Vec<(f64, f64, f64)> = freqs
        .iter()
        .zip(values.iter())
        .map(|(f, v)| (f * unit_scale, bin_width, *v))
        .collect();

    let epoch = match opts.epoch_override {
        Some(e) => e,
        None => {
            let cols = if tdims.len() == 1 {
                1
            } else {
                tdims[1] as usize
            };
            let col = if opts.mean_loads {
                0
            } else {
                opts.load.min(cols.saturating_sub(1))
            };
            let idx = start * cols + col;
            let jd = *times
                .get(idx)
                .ok_or_else(|| format!("metadata/times has no index {idx}"))?;
            (jd - 2_440_587.5) * 86_400.0
        }
    };
    if !epoch.is_finite() {
        return Err("epoch is not finite".into());
    }
    Ok((epoch, bins))
}

fn run(source: &Source, opts: &Options, out: &str) -> Result<usize, String> {
    let (epoch, bins) = read_gsh5(source, opts)?;
    let bytes = write_spectral_bin(epoch, &bins);
    std::fs::write(out, &bytes).map_err(|e| format!("write {out}: {e}"))?;
    match parse_spectral_bin(&bytes) {
        Some((parsed_epoch, parsed)) => {
            eprintln!(
                "{out}: {} bins, epoch {parsed_epoch} — roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => return Err(format!("{out}: roundtrip parse void")),
    }
    eprintln!(
        "range reads {} · bytes {}",
        source.fetches.get(),
        source.bytes.get()
    );
    Ok(bins.len())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci = args.iter().any(|a| a == "--ci-mode");
    let input = match arg_value(&args, "--input") {
        Some(v) => v,
        None => {
            eprintln!("--input absent — the .gsh5 path or URL is undeclared");
            std::process::exit(1);
        }
    };
    let selection = if let Some(v) = arg_value(&args, "--integration") {
        match v.parse::<usize>() {
            Ok(i) => Selection::Integration(i),
            Err(_) => {
                eprintln!("--integration {v} is not an index");
                std::process::exit(1);
            }
        }
    } else if let Some(v) = arg_value(&args, "--range") {
        let Some((s, e)) = v.split_once(':') else {
            eprintln!("--range {v} is not start:end");
            std::process::exit(1);
        };
        match (s.parse::<usize>(), e.parse::<usize>()) {
            (Ok(s), Ok(e)) => Selection::Range(s, e),
            _ => {
                eprintln!("--range {v} is not start:end");
                std::process::exit(1);
            }
        }
    } else if let Some(v) = arg_value(&args, "--limit") {
        match v.parse::<usize>() {
            Ok(n) => Selection::Limit(n),
            Err(_) => {
                eprintln!("--limit {v} is not a count");
                std::process::exit(1);
            }
        }
    } else {
        Selection::Mean
    };
    let selection = match arg_value(&args, "--limit") {
        Some(v) if !matches!(selection, Selection::Limit(_)) => match v.parse::<usize>() {
            Ok(n) if n > 0 => match selection {
                Selection::Integration(i) => Selection::Range(i, i + n),
                Selection::Range(s, _) => Selection::Range(s, s + n),
                Selection::Mean => Selection::Limit(n),
                other => other,
            },
            _ => {
                eprintln!("--limit {v} is not a positive count");
                std::process::exit(1);
            }
        },
        _ => selection,
    };
    let load_arg = arg_value(&args, "--load");
    let load = match load_arg.as_deref() {
        Some(v) => match v.parse::<usize>() {
            Ok(l) => l,
            Err(_) => {
                eprintln!("--load {v} is not an index");
                std::process::exit(1);
            }
        },
        None => usize::MIN,
    };
    let mean_loads = args.iter().any(|a| a == "--mean-loads") || load_arg.is_none();
    let chunk = match arg_value(&args, "--chunk") {
        Some(v) => match v.parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => {
                eprintln!("--chunk {v} is not a positive count");
                std::process::exit(1);
            }
        },
        None => DEFAULT_CHUNK,
    };
    let epoch_override = match arg_value(&args, "--epoch") {
        Some(v) => match v.parse::<f64>() {
            Ok(e) if e.is_finite() => Some(e),
            _ => {
                eprintln!("--epoch {v} is not a finite unix second");
                std::process::exit(1);
            }
        },
        None => None,
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            let path = compiled(NETLOC, DEFAULT_OUT);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let source = match Source::open(&input) {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    };
    let opts = Options {
        load,
        mean_loads,
        selection,
        chunk,
        epoch_override,
    };
    if let Err(msg) = run(&source, &opts, &out) {
        eprintln!("{msg}");
        std::process::exit(1);
    }
    if ci && !upload_release(NETLOC, &out) {
        eprintln!("{out}: CDN upload returned void");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &[u8] = include_bytes!("../../testdata/edges_gsh5_table.h5");

    fn opts(load: usize, mean_loads: bool, selection: Selection) -> Options {
        Options {
            load,
            mean_loads,
            selection,
            chunk: 4,
            epoch_override: None,
        }
    }

    fn bins(source: &Source, o: &Options) -> (f64, Vec<(f64, f64, f64)>) {
        read_gsh5(source, o).expect("fixture reads")
    }

    #[test]
    fn gsh5_memory_integration_zero_load_zero() {
        let source = Source::memory(FIXTURE.to_vec());
        let (epoch, b) = bins(&source, &opts(0, false, Selection::Integration(0)));
        assert_eq!(b.len(), 4);
        assert!((b[0].0 - 50_000_000.0).abs() < 1e-6);
        assert!((b[3].0 - 50_300_000.0).abs() < 1e-6);
        assert!((b[0].1 - 100_000.0).abs() < 1e-6);
        let vals: Vec<f64> = b.iter().map(|x| x.2).collect();
        assert_eq!(vals, vec![1.0, 2.0, 3.0, 4.0]);
        assert!((epoch - 1_473_120_000.0).abs() < 1e-6);
    }

    #[test]
    fn gsh5_range_one_selects_the_second_integration() {
        let source = Source::memory(FIXTURE.to_vec());
        let (_, b) = bins(&source, &opts(0, false, Selection::Range(1, 2)));
        let vals: Vec<f64> = b.iter().map(|x| x.2).collect();
        assert_eq!(vals, vec![5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn gsh5_mean_over_two_integrations_load_one() {
        let source = Source::memory(FIXTURE.to_vec());
        let (_, b) = bins(&source, &opts(1, false, Selection::Limit(2)));
        let vals: Vec<f64> = b.iter().map(|x| x.2).collect();
        assert_eq!(vals, vec![30.0, 40.0, 50.0, 60.0]);
    }

    #[test]
    fn gsh5_mean_loads_averages_the_three_states() {
        let source = Source::memory(FIXTURE.to_vec());
        let (_, b) = bins(&source, &opts(0, true, Selection::Integration(0)));
        let vals: Vec<f64> = b.iter().map(|x| x.2).collect();
        assert_eq!(vals, vec![37.0, 74.0, 111.0, 148.0]);
    }

    #[test]
    fn gsh5_mean_default_averages_all_times_and_states() {
        let source = Source::memory(FIXTURE.to_vec());
        let (_, b) = bins(&source, &opts(0, true, Selection::Mean));
        let vals: Vec<f64> = b.iter().map(|x| x.2).collect();
        assert_eq!(vals, vec![111.0, 148.0, 185.0, 222.0]);
    }

    #[test]
    fn gsh5_local_path_reads_the_same_bytes() {
        let path =
            std::env::temp_dir().join(format!("edges_gsh5_fixture_{}.h5", std::process::id()));
        std::fs::write(&path, FIXTURE).expect("write fixture");
        let source = Source::open(path.to_str().unwrap()).expect("open local");
        let (_, b) = bins(&source, &opts(0, false, Selection::Integration(0)));
        let vals: Vec<f64> = b.iter().map(|x| x.2).collect();
        assert_eq!(vals, vec![1.0, 2.0, 3.0, 4.0]);
        assert!(source.fetches.get() >= 3, "base + freqs + times + data");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn gsh5_empty_selection_is_refused() {
        let source = Source::memory(FIXTURE.to_vec());
        let err = read_gsh5(&source, &opts(0, false, Selection::Range(2, 2)))
            .expect_err("empty selection");
        assert!(err.contains("is empty"));
    }
}
