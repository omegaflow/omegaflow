use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::te::{pcmci_links, TeEstimator, TeNull};

const LAIC_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/laic.bin";
const BIN_MAGIC: u32 = 0x4C41_4943;
const WINDOW_S: f64 = 259200.0;
const CELL_S: f64 = 3600.0;
const N_CELLS: usize = 72;

struct Cursor<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Cursor<'a> {
    fn u8(&mut self) -> Option<u8> {
        let v = *self.b.get(self.p)?;
        self.p += 1;
        Some(v)
    }
    fn u32(&mut self) -> Option<u32> {
        let s: [u8; 4] = self.b.get(self.p..self.p + 4)?.try_into().ok()?;
        self.p += 4;
        Some(u32::from_le_bytes(s))
    }
    fn f64(&mut self) -> Option<f64> {
        let s: [u8; 8] = self.b.get(self.p..self.p + 8)?.try_into().ok()?;
        self.p += 8;
        Some(f64::from_le_bytes(s))
    }
}

struct Window {
    group: u8,
    t0: f64,
    f: Vec<(f64, f64)>,
    bz: Vec<(f64, f64)>,
    region_t: Vec<f64>,
}

fn read_window(c: &mut Cursor) -> Option<Window> {
    let group = c.u8()?;
    let t0 = c.f64()?;
    let _mag = c.f64()?;
    let _lat = c.f64()?;
    let _lon = c.f64()?;
    let n = c.u32()? as usize;
    let mut f = Vec::with_capacity(n);
    for _ in 0..n {
        let t = c.f64()?;
        let x = c.f64()?;
        f.push((t, x));
    }
    let n = c.u32()? as usize;
    let mut bz = Vec::with_capacity(n);
    for _ in 0..n {
        let t = c.f64()?;
        let x = c.f64()?;
        bz.push((t, x));
    }
    let n = c.u32()? as usize;
    let mut region_t = Vec::with_capacity(n);
    for _ in 0..n {
        let t = c.f64()?;
        let _a = c.f64()?;
        let _b = c.f64()?;
        let _x = c.f64()?;
        region_t.push(t);
    }
    let n = c.u32()? as usize;
    for _ in 0..n {
        c.f64()?;
        c.f64()?;
    }
    let n = c.u32()? as usize;
    for _ in 0..n {
        c.f64()?;
        c.f64()?;
        c.f64()?;
        c.f64()?;
    }
    Some(Window {
        group,
        t0,
        f,
        bz,
        region_t,
    })
}

fn bin_count(epochs: &[f64], t0: f64, cell: f64, n: usize) -> Vec<f32> {
    let mut counts = vec![0f32; n];
    for &t in epochs {
        let idx = ((t - t0) / cell).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        counts[idx as usize] += 1.0;
    }
    counts
}

fn bin_mean(series: &[(f64, f64)], t0: f64, cell: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut cnt = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / cell).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        let i = idx as usize;
        sums[i] += v;
        cnt[i] += 1;
    }
    (0..n)
        .map(|i| {
            if cnt[i] > 0 {
                Some((sums[i] / cnt[i] as f64) as f32)
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    println!("=== Nobel-DAG LAIC probe — the common-cause control ===");
    let Some(bytes) = fetch_raw_bytes(LAIC_CDN, 86400) else {
        eprintln!("laic.bin carries no asset — the run stays unmeasured");
        return;
    };
    let mut c = Cursor { b: &bytes, p: 0 };
    let Some(magic) = c.u32() else {
        eprintln!("laic.bin header parses void");
        return;
    };
    if magic != BIN_MAGIC {
        eprintln!("laic.bin carries no LAIC contract");
        return;
    }
    let version = c.u32();
    let _has_env = version == Some(2);
    let Some(n_windows) = c.u32() else {
        eprintln!("laic.bin count parses void");
        return;
    };
    let mut windows = Vec::new();
    for _ in 0..n_windows {
        let Some(mut w) = read_window(&mut c) else {
            eprintln!("laic.bin window parses void — the tail stays unread");
            break;
        };
        if _has_env {
            let Some(n) = c.u32() else {
                eprintln!("laic.bin env count parses void — the tail stays unread");
                break;
            };
            for _ in 0..n as usize {
                c.f64();
                c.f64();
            }
        }
        w.f.sort_by(|a, b| a.0.total_cmp(&b.0));
        w.bz.sort_by(|a, b| a.0.total_cmp(&b.0));
        windows.push(w);
    }
    println!(
        "windows: {} ({} events)",
        windows.len(),
        windows.iter().filter(|w| w.group == 0).count()
    );

    let mut counts = [0usize; 6];
    let mut sum_excess = [0.0f64; 6];
    let mut runs = 0usize;
    for w in windows.iter().filter(|w| w.group == 0) {
        let t_start = w.t0 - WINDOW_S;
        let litho = bin_count(&w.region_t, t_start, CELL_S, N_CELLS);
        let f = bin_mean(&w.f, t_start, CELL_S, N_CELLS);
        let bz = bin_mean(&w.bz, t_start, CELL_S, N_CELLS);
        let mut s0 = Vec::new();
        let mut s1 = Vec::new();
        let mut s2 = Vec::new();
        for i in 0..N_CELLS {
            let (Some(fv), Some(bv)) = (f[i], bz[i]) else {
                continue;
            };
            s0.push(litho[i]);
            s1.push(fv);
            s2.push(bv);
        }
        if s0.len() < 30 {
            continue;
        }
        let series: [&[f32]; 3] = [&s0, &s1, &s2];
        let Some(links) = pcmci_links(
            &series,
            2,
            4,
            3,
            0x9E37_79B9_7F4A_7C15,
            100,
            TeNull::Residual,
            0,
            TeEstimator::Binned,
            4,
            2,
            0.05,
        ) else {
            continue;
        };
        runs += 1;
        let dirs = [(0usize, 1usize), (1, 0), (2, 1), (2, 0), (0, 2), (1, 2)];
        for (k, &(d, t)) in dirs.iter().enumerate() {
            if let Some(l) = links
                .iter()
                .filter(|l| l.driver == d && l.target == t)
                .max_by(|a, b| (a.te - a.threshold).total_cmp(&(b.te - b.threshold)))
            {
                if l.te > l.threshold {
                    counts[k] += 1;
                }
                sum_excess[k] += l.te - l.threshold;
            }
        }
    }

    let names = [
        "litho -> F",
        "F -> litho",
        "Bz -> F",
        "Bz -> litho",
        "litho -> Bz",
        "F -> Bz",
    ];
    println!();
    println!(
        "directed edges across {} event windows (arrow = TE > mean+2σ):",
        runs
    );
    for k in 0..6 {
        let mean = if runs > 0 {
            sum_excess[k] / runs as f64
        } else {
            0.0
        };
        println!(
            "  {:>14} | arrows {}/{} | mean excess {:+.3e}",
            names[k], counts[k], runs, mean
        );
    }

    println!();
    if runs == 0 {
        println!("no event window carried 30 common cells — no verdict");
        return;
    }
    println!("=== Verdict (LAIC common-cause control) ===");
    let l2f = counts[0];
    let f2l = counts[1];
    let bz2f = counts[2];
    let neg_everywhere = (0..6).all(|k| sum_excess[k] < 0.0);
    let l2f_r = l2f as f64 / runs as f64;
    let f2l_r = f2l as f64 / runs as f64;
    let bz2f_r = bz2f as f64 / runs as f64;
    let floor = l2f_r.max(f2l_r);
    let bz_elevated = bz2f_r >= floor + 0.05;
    if neg_everywhere {
        println!(
            "The mean excess is negative for every direction — the surrogate-floor bias, not a measured flow. The LAIC silence holds under common-cause control (consistent with laic-arrow-direction)."
        );
    }
    if l2f_r >= f2l_r + 0.05 {
        println!(
            "litho -> F is clearly elevated over the reverse ({:.3} vs {:.3}) — a directed lithosphere->ionosphere coupling, named.",
            l2f_r, f2l_r
        );
    } else if f2l_r >= l2f_r + 0.05 {
        println!(
            "F -> litho is clearly elevated over the reverse ({:.3} vs {:.3}) — a directed earthward coupling, named.",
            f2l_r, l2f_r
        );
    } else {
        println!(
            "litho -> F ({:.3}) and F -> litho ({:.3}) sit at the same floor — no dominant lithosphere<->ionosphere direction.",
            l2f_r, f2l_r
        );
    }
    if bz_elevated {
        println!(
            "The solar control Bz -> F ({:.3}) is elevated over the floor ({:.3}) — a weak real solar driver, the conditioning target.",
            bz2f_r, floor
        );
    } else {
        println!(
            "The solar control Bz -> F ({:.3}) stays at the floor — the confound carries no measured arrow.",
            bz2f_r
        );
    }
}
