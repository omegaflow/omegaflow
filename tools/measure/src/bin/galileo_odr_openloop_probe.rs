use std::fs;

const REC: usize = 2666;
const HDR: usize = 166;
const FS: f64 = 1250.0;
const SEG: usize = 8192;

struct SegStat {
    n_seg: usize,
    med_snr: f64,
    p10: f64,
    p90: f64,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.len() % 4 != 0 {
        eprintln!("galileo ODR open-loop probe: <odr> <label> <h0> <h1> [...] (4 tokens per file)");
        return;
    }
    let mut i = 0;
    while i < args.len() {
        let path = &args[i];
        let label = &args[i + 1];
        let Ok(h0) = args[i + 2].parse::<f64>() else {
            eprintln!("{path}: window start not numeric");
            return;
        };
        let Ok(h1) = args[i + 3].parse::<f64>() else {
            eprintln!("{path}: window stop not numeric");
            return;
        };
        measure(path, label, h0, h1);
        i += 4;
    }
}

fn measure(path: &str, label: &str, h0: f64, h1: f64) {
    let Ok(bytes) = fs::read(path) else {
        println!("{label}: read void");
        return;
    };
    let nrec = bytes.len() / REC;
    if nrec == 0 {
        println!("{label}: no records");
        return;
    }
    let t0 = (h0 * 3.6e6) as u32;
    let t1 = (h1 * 3.6e6) as u32;
    let mut chans: [Vec<f64>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut n_in = 0usize;
    for k in 0..nrec {
        let r = &bytes[k * REC..(k + 1) * REC];
        let ms = u32::from_be_bytes([r[12], r[13], r[14], r[15]]) & 0x07FF_FFFF;
        if ms < t0 {
            continue;
        }
        if ms >= t1 {
            break;
        }
        n_in += 1;
        let s = &r[HDR..REC];
        for ch in 0..4 {
            let stream = &mut chans[ch];
            for g in 0..625 {
                stream.push(s[g * 4 + ch] as f64);
            }
        }
    }
    if n_in == 0 {
        println!("{label}: no records in [{h0:.1},{h1:.1}]h");
        return;
    }
    println!("{label}: {n_in} records");
    for ch in 0..4 {
        let x = &chans[ch];
        let n = x.len();
        if n < SEG {
            println!("  AD{}: only {n} samples", ch + 1);
            continue;
        }
        let mean = x.iter().sum::<f64>() / n as f64;
        let std = x
            .iter()
            .map(|v| {
                let d = v - mean;
                d * d
            })
            .sum::<f64>()
            / n as f64;
        let st = segment_snr(x);
        println!(
            "  AD{}: n {n} std {std:.1} segsnr med {:.1} p10 {:.1} p90 {:.1} ({} segs)",
            ch + 1,
            st.med_snr,
            st.p10,
            st.p90,
            st.n_seg
        );
    }
}

fn segment_snr(x: &[f64]) -> SegStat {
    let mut snrs: Vec<f64> = Vec::new();
    let mut i = 0;
    while i + SEG <= x.len() {
        let m = seg_spec(&x[i..i + SEG]);
        let nf = median(&m);
        let pk = m.iter().cloned().fold(f64::MIN, f64::max);
        if nf > 0.0 {
            snrs.push(pk / nf);
        }
        i += SEG;
    }
    if snrs.is_empty() {
        return SegStat { n_seg: 0, med_snr: 0.0, p10: 0.0, p90: 0.0 };
    }
    snrs.sort_by(|a, b| a.total_cmp(b));
    let med = snrs[snrs.len() / 2];
    let p10 = snrs[(snrs.len() as f64 * 0.10) as usize];
    let p90 = snrs[((snrs.len() as f64 * 0.90) as usize).min(snrs.len() - 1)];
    SegStat { n_seg: snrs.len(), med_snr: med, p10, p90 }
}

fn seg_spec(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let mut re = vec![0.0f64; n];
    let mut im = vec![0.0f64; n];
    for (j, v) in x.iter().enumerate() {
        let w = 0.5 - 0.5 * ((std::f64::consts::TAU * j as f64) / (n - 1) as f64).cos();
        re[j] = v * w;
    }
    fft(&mut re, &mut im, false);
    let mut out = Vec::new();
    for b in 1..=n / 2 {
        let hz = b as f64 * FS / n as f64;
        if hz > 2.0 {
            out.push(re[b] * re[b] + im[b] * im[b]);
        }
    }
    out
}

fn median(v: &[f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    s[s.len() / 2]
}

fn fft(re: &mut [f64], im: &mut [f64], inverse: bool) {
    let n = re.len();
    if n <= 1 {
        return;
    }
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2usize;
    while len <= n {
        let ang = if inverse {
            std::f64::consts::TAU / len as f64
        } else {
            -std::f64::consts::TAU / len as f64
        };
        let wr = ang.cos();
        let wi = ang.sin();
        let mut i = 0usize;
        while i < n {
            let mut cr = 1.0f64;
            let mut ci = 0.0f64;
            for k in 0..len / 2 {
                let ur = re[i + k];
                let ui = im[i + k];
                let vr = re[i + k + len / 2] * cr - im[i + k + len / 2] * ci;
                let vi = re[i + k + len / 2] * ci + im[i + k + len / 2] * cr;
                re[i + k] = ur + vr;
                im[i + k] = ui + vi;
                re[i + k + len / 2] = ur - vr;
                im[i + k + len / 2] = ui - vi;
                let ncr = cr * wr - ci * wi;
                ci = cr * wi + ci * wr;
                cr = ncr;
            }
            i += len;
        }
        len <<= 1;
    }
    if inverse {
        for v in re.iter_mut() {
            *v /= n as f64;
        }
        for v in im.iter_mut() {
            *v /= n as f64;
        }
    }
}
