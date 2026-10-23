use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

const HEADER_END: u64 = 26;

struct Reader {
    file: File,
    pos: u64,
    len: u64,
}

impl Reader {
    fn open(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("open {path}: {e}"))?;
        let len = file
            .metadata()
            .map_err(|e| format!("stat {path}: {e}"))?
            .len();
        Ok(Self { file, pos: 0, len })
    }
    fn seek(&mut self, pos: u64) -> Result<(), String> {
        if pos > self.len {
            return Err(format!("seek {pos} beyond file length {}", self.len));
        }
        self.file
            .seek(SeekFrom::Start(pos))
            .map_err(|e| format!("seek {pos}: {e}"))?;
        self.pos = pos;
        Ok(())
    }
    fn skip(&mut self, n: u64) -> Result<(), String> {
        let p = self.pos.checked_add(n).ok_or("offset overflow")?;
        if p > self.len {
            return Err(format!("skip to {p} beyond file length {}", self.len));
        }
        self.pos = p;
        Ok(())
    }
    fn u32(&mut self) -> Result<u32, String> {
        let mut b = [0u8; 4];
        self.seek(self.pos)?;
        self.file
            .read_exact(&mut b)
            .map_err(|e| format!("read u32 at {}: {e}", self.pos))?;
        self.pos += 4;
        Ok(u32::from_le_bytes(b))
    }
    fn i32(&mut self) -> Result<i32, String> {
        Ok(self.u32()? as i32)
    }
    fn f32(&mut self) -> Result<f32, String> {
        Ok(f32::from_bits(self.u32()?))
    }
    fn u64(&mut self) -> Result<u64, String> {
        let lo = self.u32()? as u64;
        let hi = self.u32()? as u64;
        Ok(lo | (hi << 32))
    }
    fn string(&mut self) -> Result<String, String> {
        let n = self.u32()? as u64;
        if n > self.len {
            return Err(format!("string length {n} exceeds file"));
        }
        let mut v = vec![0u8; n as usize];
        self.seek(self.pos)?;
        self.file
            .read_exact(&mut v)
            .map_err(|e| format!("read string at {}: {e}", self.pos))?;
        self.pos += n;
        Ok(String::from_utf8_lossy(&v).into_owned())
    }
}

#[derive(Clone, Copy, Debug)]
struct Axis {
    n_bins: usize,
    min: f32,
    max: f32,
}

impl Axis {
    fn index(&self, value: f32) -> usize {
        if self.n_bins == 0 || !self.max.is_finite() || self.max <= self.min {
            return 0;
        }
        let step = (self.max - self.min) / self.n_bins as f32;
        let i = ((value - self.min) / step).floor();
        if i.is_nan() || i < 0.0 {
            0
        } else if i >= self.n_bins as f32 {
            self.n_bins - 1
        } else {
            i as usize
        }
    }
    fn center(&self, i: usize) -> f32 {
        self.min + (i as f32 + 0.5) * (self.max - self.min) / self.n_bins as f32
    }
}

struct Meta {
    n_bins: [usize; 4],
    axes: [Axis; 4],
    md_image: u64,
    s_start: u64,
    e_start: u64,
    npix_start: u64,
    min_max_start: u64,
    n_data: u64,
    pix_start: u64,
}

fn parse(path: &str) -> Result<Meta, String> {
    let mut r = Reader::open(path)?;
    r.seek(HEADER_END)?;

    let _main_name = r.string()?;
    let _main_path = r.string()?;
    let _main_title = r.string()?;
    let n_files = r.u32()?;

    for _ in 0..n_files {
        let _name = r.string()?;
        let _path = r.string()?;
        r.skip(4 * (7 + 3 * 4))?;
        let n_en = r.u32()? as u64;
        r.skip(4 * n_en)?;
        r.skip(4 * (4 + 4 * 4 + 4))?;
        let n_rows = r.u32()? as u64;
        let n_cols = r.u32()? as u64;
        r.skip(n_rows.checked_mul(n_cols).ok_or("label size overflow")?)?;
    }

    let _det_name = r.string()?;
    let _det_path = r.string()?;
    let n_det = r.u32()? as u64;
    r.skip(n_det.checked_mul(6 * 4).ok_or("detector size overflow")?)?;

    let _data_name = r.string()?;
    let _data_path = r.string()?;
    let _data_title = r.string()?;

    r.skip(4 * (3 + 3 + 4 + 16 + 4))?;

    let n_labels = r.u32()? as u64;
    let labels_len = r.u32()? as u64;
    r.skip(
        n_labels
            .checked_mul(labels_len)
            .ok_or("label block overflow")?,
    )?;

    let npax = r.u32()? as usize;
    if npax != 4 {
        return Err(format!("npax={npax}; this tool expects a 4D sqw image"));
    }
    let niax = 4usize.saturating_sub(npax);
    r.skip(3 * niax as u64 * 4)?;
    let mut pax = Vec::with_capacity(npax);
    for _ in 0..npax {
        pax.push(r.i32()?);
    }
    let _pax = pax;
    let mut axes = [Axis {
        n_bins: 0,
        min: 0.0,
        max: 0.0,
    }; 4];
    for axis in axes.iter_mut() {
        let n_axis_points = r.u32()? as usize;
        if n_axis_points < 2 {
            return Err("axis has fewer than 2 points".into());
        }
        let mut min = 0.0f32;
        let mut max = 0.0f32;
        for j in 0..n_axis_points {
            let v = r.f32()?;
            if j == 0 {
                min = v;
            }
            max = v;
        }
        *axis = Axis {
            n_bins: n_axis_points - 1,
            min,
            max,
        };
    }
    let _dax: Vec<i32> = (0..npax).map(|_| r.i32()).collect::<Result<_, _>>()?;

    let md_image = axes
        .iter()
        .try_fold(1u64, |acc, a| acc.checked_mul(a.n_bins as u64))
        .ok_or("image size overflow")?;
    let n_bins = [
        axes[0].n_bins,
        axes[1].n_bins,
        axes[2].n_bins,
        axes[3].n_bins,
    ];

    let s_start = r.pos;
    let e_start = s_start
        .checked_add(md_image.checked_mul(4).ok_or("image size overflow")?)
        .ok_or("offset overflow")?;
    let npix_start = e_start
        .checked_add(md_image.checked_mul(4).ok_or("image size overflow")?)
        .ok_or("offset overflow")?;
    let min_max_start = npix_start
        .checked_add(md_image.checked_mul(8).ok_or("image size overflow")?)
        .ok_or("offset overflow")?;
    let (n_data, pix_start) = if min_max_start + 32 + 4 + 8 <= r.len {
        r.seek(min_max_start + 32 + 4)?;
        let n = r.u64()?;
        (n, min_max_start + 32 + 4 + 8)
    } else {
        (0, 0)
    };

    Ok(Meta {
        n_bins,
        axes,
        md_image,
        s_start,
        e_start,
        npix_start,
        min_max_start,
        n_data,
        pix_start,
    })
}

fn read_s(path: &str, meta: &Meta) -> Result<Vec<f32>, String> {
    let mut file = File::open(path).map_err(|e| format!("open {path}: {e}"))?;
    let bytes = meta.md_image * 4;
    let mut buf = vec![0u8; bytes as usize];
    file.seek(SeekFrom::Start(meta.s_start))
        .map_err(|e| format!("seek s: {e}"))?;
    file.read_exact(&mut buf)
        .map_err(|e| format!("read s ({} bytes): {e}", bytes))?;
    Ok(buf
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| f32::from_le_bytes(*c))
        .collect())
}

fn cut(path: &str, q: [f32; 3], window: i32, out: Option<&str>) -> Result<(), String> {
    let meta = parse(path)?;
    let s = read_s(path, &meta)?;
    let n = meta.n_bins;
    let idx = [
        meta.axes[0].index(q[0]),
        meta.axes[1].index(q[1]),
        meta.axes[2].index(q[2]),
    ];
    let flat = |i0: usize, i1: usize, i2: usize, i3: usize| -> usize {
        i0 + n[0] * (i1 + n[1] * (i2 + n[2] * i3))
    };
    let mut csv = String::from("E_meV,S_mean,nbins\n");
    for ie in 0..n[3] {
        let mut sum = 0.0f64;
        let mut count = 0u32;
        for d0 in -window..=window {
            for d1 in -window..=window {
                for d2 in -window..=window {
                    let i0 = idx[0] as i32 + d0;
                    let i1 = idx[1] as i32 + d1;
                    let i2 = idx[2] as i32 + d2;
                    if i0 < 0
                        || i1 < 0
                        || i2 < 0
                        || i0 >= n[0] as i32
                        || i1 >= n[1] as i32
                        || i2 >= n[2] as i32
                    {
                        continue;
                    }
                    sum += s[flat(i0 as usize, i1 as usize, i2 as usize, ie)] as f64;
                    count += 1;
                }
            }
        }
        let mean = if count > 0 { sum / count as f64 } else { 0.0 };
        csv.push_str(&format!(
            "{:.5},{:.6e},{}\n",
            meta.axes[3].center(ie),
            mean,
            count
        ));
    }
    match out {
        Some(p) => std::fs::write(p, csv).map_err(|e| format!("write {p}: {e}"))?,
        None => {
            let mut stdout = std::io::stdout();
            stdout
                .write_all(csv.as_bytes())
                .map_err(|e| format!("stdout: {e}"))?;
        }
    }
    eprintln!(
        "q=({:.3},{:.3},{:.3}) idx={:?} bins={:?} s_bytes={}",
        q[0],
        q[1],
        q[2],
        idx,
        n,
        meta.md_image * 4
    );
    Ok(())
}

fn meta_json(path: &str) -> Result<(), String> {
    let m = parse(path)?;
    let names = ["H", "K", "L", "E"];
    let axes: Vec<String> = m
        .axes
        .iter()
        .enumerate()
        .map(|(i, a)| {
            format!(
                "{{\"name\":\"{}\",\"min\":{:.9},\"max\":{:.9},\"n_bins\":{},\"bin\":{:.9}}}",
                names[i],
                a.min,
                a.max,
                a.n_bins,
                (a.max - a.min) / a.n_bins as f32
            )
        })
        .collect();
    println!(
        "{{\"file\":\"{}\",\"n_bins\":[{},{},{},{}],\"axes\":[{}],\"md_image\":{},\"offsets\":{{\"s\":{},\"e\":{},\"npix\":{},\"min_max\":{},\"pix\":{}}},\"n_data\":{}}}",
        path,
        m.n_bins[0],
        m.n_bins[1],
        m.n_bins[2],
        m.n_bins[3],
        axes.join(","),
        m.md_image,
        m.s_start,
        m.e_start,
        m.npix_start,
        m.min_max_start,
        m.pix_start,
        m.n_data
    );
    Ok(())
}

fn parse_q(s: &str) -> Result<[f32; 3], String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err("--q needs H,K,L".into());
    }
    let mut q = [0.0f32; 3];
    for (i, p) in parts.iter().enumerate() {
        q[i] = p
            .trim()
            .parse()
            .map_err(|_| format!("bad q component {p}"))?;
    }
    Ok(q)
}

fn f32_at(rec: &[u8; 36], i: usize) -> f32 {
    let o = i * 4;
    f32::from_le_bytes([rec[o], rec[o + 1], rec[o + 2], rec[o + 3]])
}

struct PixelOpts<'a> {
    q: [f32; 3],
    dq: f32,
    emin: Option<f32>,
    emax: Option<f32>,
    ebins: usize,
    out: Option<&'a str>,
    limit: Option<u64>,
}

fn pixels(path: &str, o: &PixelOpts) -> Result<(), String> {
    let q = o.q;
    let dq = o.dq;
    let emin = o.emin;
    let emax = o.emax;
    let ebins = o.ebins;
    let out = o.out;
    let limit = o.limit;
    let meta = parse(path)?;
    if meta.n_data == 0 || meta.pix_start == 0 {
        return Err("pixel block not present in this file (header chunk only?)".into());
    }
    let mut file = File::open(path).map_err(|e| format!("open {path}: {e}"))?;
    file.seek(SeekFrom::Start(meta.pix_start))
        .map_err(|e| format!("seek pixels: {e}"))?;

    let e0 = emin.unwrap_or(meta.axes[3].min);
    let e1 = emax.unwrap_or(meta.axes[3].max);
    if e1 <= e0 || ebins == 0 {
        return Err("bad energy range or bin count".into());
    }
    let total = limit.unwrap_or(meta.n_data).min(meta.n_data);
    let chunk_events = 1_000_000usize;
    let mut buf = vec![0u8; chunk_events * 36];
    let mut hist_s = vec![0.0f64; ebins];
    let mut hist_e = vec![0.0f64; ebins];
    let mut hist_n = vec![0u64; ebins];
    let mut done = 0u64;
    let mut kept = 0u64;
    while done < total {
        let n = ((total - done) as usize).min(chunk_events);
        let bytes = n * 36;
        file.read_exact(&mut buf[..bytes])
            .map_err(|e| format!("read pixels at {done}: {e}"))?;
        for rec in buf[..bytes].as_chunks::<36>().0 {
            let u1 = f32_at(rec, 0);
            let u2 = f32_at(rec, 1);
            let u3 = f32_at(rec, 2);
            let e = f32_at(rec, 3);
            if (u1 - q[0]).abs() > dq || (u2 - q[1]).abs() > dq || (u3 - q[2]).abs() > dq {
                continue;
            }
            if e < e0 || e > e1 || !e.is_finite() {
                continue;
            }
            let b = (((e - e0) / (e1 - e0)) * ebins as f32) as usize;
            let b = b.min(ebins - 1);
            hist_s[b] += f32_at(rec, 7) as f64;
            hist_e[b] += f32_at(rec, 8) as f64;
            hist_n[b] += 1;
            kept += 1;
        }
        done += n as u64;
        if done.is_multiple_of(5_000_000) || done == total {
            eprintln!("pixels: {done}/{} kept={kept}", total);
        }
    }
    let mut csv = String::from("E_meV,S,err,npix\n");
    for i in 0..ebins {
        let e = e0 + (i as f32 + 0.5) * (e1 - e0) / ebins as f32;
        csv.push_str(&format!(
            "{:.5},{:.6e},{:.6e},{}\n",
            e,
            hist_s[i],
            hist_e[i].max(0.0).sqrt(),
            hist_n[i]
        ));
    }
    match out {
        Some(p) => std::fs::write(p, csv).map_err(|e| format!("write {p}: {e}"))?,
        None => std::io::stdout()
            .write_all(csv.as_bytes())
            .map_err(|e| format!("stdout: {e}"))?,
    }
    eprintln!(
        "pixels: q=({:.3},{:.3},{:.3}) dq={:.3} E=[{:.3},{:.3}] bins={} kept={}",
        q[0], q[1], q[2], dq, e0, e1, ebins, kept
    );
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    match args.first().map(String::as_str) {
        Some("meta") if args.len() == 2 => meta_json(&args[1]),
        Some("cut") if args.len() >= 2 => {
            let mut q = [1.5f32, 1.5, 2.0];
            let mut window = 3i32;
            let mut out: Option<&str> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--q" if i + 1 < args.len() => {
                        q = parse_q(&args[i + 1])?;
                        i += 2;
                    }
                    "--window" if i + 1 < args.len() => {
                        window = args[i + 1].parse().map_err(|_| "bad --window".to_string())?;
                        i += 2;
                    }
                    "--out" if i + 1 < args.len() => {
                        out = Some(args[i + 1].as_str());
                        i += 2;
                    }
                    other => return Err(format!("unknown argument: {other}")),
                }
            }
            cut(&args[1], q, window, out)
        }
        Some("pixels") if args.len() >= 2 => {
            let mut q = [1.5f32, 1.5, 2.0];
            let mut dq = 0.3f32;
            let mut emin = None;
            let mut emax = None;
            let mut ebins = 1200usize;
            let mut out: Option<&str> = None;
            let mut limit = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--q" if i + 1 < args.len() => {
                        q = parse_q(&args[i + 1])?;
                        i += 2;
                    }
                    "--dq" if i + 1 < args.len() => {
                        dq = args[i + 1].parse().map_err(|_| "bad --dq".to_string())?;
                        i += 2;
                    }
                    "--emin" if i + 1 < args.len() => {
                        emin = Some(args[i + 1].parse().map_err(|_| "bad --emin".to_string())?);
                        i += 2;
                    }
                    "--emax" if i + 1 < args.len() => {
                        emax = Some(args[i + 1].parse().map_err(|_| "bad --emax".to_string())?);
                        i += 2;
                    }
                    "--ebins" if i + 1 < args.len() => {
                        ebins = args[i + 1].parse().map_err(|_| "bad --ebins".to_string())?;
                        i += 2;
                    }
                    "--out" if i + 1 < args.len() => {
                        out = Some(args[i + 1].as_str());
                        i += 2;
                    }
                    "--limit" if i + 1 < args.len() => {
                        limit = Some(args[i + 1].parse().map_err(|_| "bad --limit".to_string())?);
                        i += 2;
                    }
                    other => return Err(format!("unknown argument: {other}")),
                }
            }
            pixels(
                &args[1],
                &PixelOpts {
                    q,
                    dq,
                    emin,
                    emax,
                    ebins,
                    out,
                    limit,
                },
            )
        }
        _ => Err("usage:\n  sqw_image meta <file.sqw>\n  sqw_image cut <file.sqw> [--q H,K,L] [--window N] [--out CSV]\n  sqw_image pixels <file.sqw> [--q H,K,L] [--dq D] [--emin E] [--emax E] [--ebins N] [--limit N] [--out CSV]".into()),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&args) {
        eprintln!("sqw_image: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_index_clamps() {
        let a = Axis {
            n_bins: 50,
            min: -1.0,
            max: 1.0,
        };
        assert_eq!(a.index(-2.0), 0);
        assert_eq!(a.index(0.0), 25);
        assert_eq!(a.index(2.0), 49);
    }

    #[test]
    fn axis_center_roundtrip() {
        let a = Axis {
            n_bins: 50,
            min: -1.0,
            max: 1.0,
        };
        assert!((a.center(0) - -0.98).abs() < 1e-6);
        assert!((a.center(49) - 0.98).abs() < 1e-6);
    }
}
