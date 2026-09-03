use super::*;

pub type Record = SampleRecord;

pub struct PackedWindow {
    pub field: Vec<f32>,
    pub meta: Vec<f32>,
    pub count: u32,
}

#[derive(Clone, Copy)]
pub struct PresenceFrame {
    pub omega: [f32; 9],
}

pub trait KineticRadiator: Send + 'static {
    fn vibrate(&mut self, frame: &PresenceFrame);
}

pub struct AcousticOscillator {
    pub _thread: Option<thread::JoinHandle<()>>,
}

impl AcousticOscillator {
    pub fn new(rx: mpsc::Receiver<PresenceFrame>) -> Self {
        let emit = !std::io::stdout().is_terminal();
        let handle = thread::spawn(move || {
            let mut out = std::io::stdout();
            while let Ok(frame) = rx.recv() {
                if emit {
                    let intensity: f32 = frame.omega.iter().sum();
                    if std::io::Write::write_all(&mut out, &intensity.to_le_bytes()).is_err()
                        || std::io::Write::flush(&mut out).is_err()
                    {
                        break;
                    }
                }
            }
        });
        Self {
            _thread: Some(handle),
        }
    }
}

pub struct SeismicOscillator {
    pub port: Option<Box<dyn serialport::SerialPort>>,
}

impl SeismicOscillator {
    pub fn new(path: &str) -> Self {
        let port = serialport::new(path, 115_200)
            .timeout(std::time::Duration::from_millis(50))
            .open()
            .ok();
        if port.is_none() {
            eprintln!(
                "seismic oscillator: {} unreachable — the oscillator stays silent",
                path
            );
        }
        Self { port }
    }
}

impl KineticRadiator for SeismicOscillator {
    fn vibrate(&mut self, frame: &PresenceFrame) {
        let Some(port) = self.port.as_mut() else {
            return;
        };
        let intensity: f32 = frame.omega.iter().sum();
        if std::io::Write::write_all(port, &intensity.to_le_bytes()).is_err() {
            self.port = None;
        }
    }
}

pub fn pack_window(records: &[Record], presence: [f64; 3]) -> PackedWindow {
    let n = records.len();
    let mut field = vec![0.0f32; n * 12];
    let mut meta = vec![0.0f32; n * 16];
    for (j, r) in records.iter().enumerate() {
        let f = j * 12;
        let m = j * 16;
        field[f] = (r.0 - presence[0]) as f32;
        field[f + 1] = (r.1 - presence[1]) as f32;
        field[f + 2] = (r.2 - presence[2]) as f32;
        field[f + 3] = r.3 as f32;
        field[f + 4] = r.4 as f32;
        field[f + 5] = r.5 as f32;
        field[f + 6] = r.9 as f32;
        field[f + 7] = r.10 as f32;
        field[f + 8] = r.11 as f32;
        field[f + 9] = r.12 as f32;
        field[f + 10] = r.13 as f32;
        field[f + 11] = r.14 as f32;
        meta[m] = r.7 as f32;
        meta[m + 1] = r.6 as f32;
        meta[m + 2] = r.8 as f32;
        meta[m + 3] = if r.9 == 0.0 { r.15 as f32 } else { 0.0 };
        meta[m + 4] = r.15 as f32;
        meta[m + 5] = r.16 as f32;
        meta[m + 6] = r.17 as f32;
        meta[m + 7] = r.18 as f32;
        meta[m + 8] = r.19 as f32;
        meta[m + 9] = r.20 as f32;
        meta[m + 10] = r.21 as f32;
        meta[m + 11] = r.22 as f32;
        meta[m + 12] = r.23 as f32;
        meta[m + 13] = r.24 as f32;
        meta[m + 14] = r.25 as f32;
        meta[m + 15] = 0.0;
    }
    PackedWindow {
        field,
        meta,
        count: n as u32,
    }
}

pub fn force_ref_medians(field: &[f32], meta: &[f32]) -> [Option<f32>; 9] {
    let mut hist: [[u32; 256]; 9] = [[0; 256]; 9];
    let mut sum: [[f32; 256]; 9] = [[0.0; 256]; 9];
    let mut n: [u32; 9] = [0; 9];
    for (j, f) in field.chunks_exact(12).enumerate() {
        let ft = f[6] as i64;
        if !(0..=8).contains(&ft) {
            continue;
        }
        let v = f[3];
        if !v.is_finite() || v == 0.0 {
            continue;
        }
        if v.abs() == meta[j * 16] {
            continue;
        }
        let l = v.abs().log2();
        let b = log2_bin_of(l);
        hist[ft as usize][b] += 1;
        sum[ft as usize][b] += l;
        n[ft as usize] += 1;
    }
    let mut meds = [None; 9];
    for ft in 0..9 {
        if n[ft] == 0 {
            continue;
        }
        let target = (n[ft] + 1) / 2;
        let mut cum = 0u32;
        let mut bin = 0usize;
        while cum < target && bin < 256 {
            cum += hist[ft][bin];
            if cum < target {
                bin += 1;
            }
        }
        meds[ft] = Some((sum[ft][bin] / hist[ft][bin] as f32).exp2());
    }
    meds
}

pub fn log2_bin_of(l: f32) -> usize {
    ((l + 126.0) as i32).clamp(0, 255) as usize
}

pub fn emit_curves(
    cset: &CurveSet,
    center: [f64; 3],
    t: f64,
    pad: f64,
    records: &mut Vec<SampleRecord>,
) {
    let Some(kernel) = kernel_id_for_force(0) else {
        return;
    };
    for star in &cset.stars {
        if !star.plx_mas.is_finite() || star.plx_mas <= 0.0 || star.samples.len() < 2 {
            continue;
        }
        let d_m = (1000.0 / star.plx_mas) * PARSEC_M;
        let ra = star.ra_deg.to_radians();
        let dec = star.dec_deg.to_radians();
        let (sa, ca) = ra.sin_cos();
        let (sd, cd) = dec.sin_cos();
        let p = [cd * ca * d_m, cd * sa * d_m, sd * d_m];
        let rel = [p[0] - center[0], p[1] - center[1], p[2] - center[2]];
        let dist = (rel[0] * rel[0] + rel[1] * rel[1] + rel[2] * rel[2]).sqrt();
        if dist > pad {
            continue;
        }
        let idx = star.samples.partition_point(|s| s.0 <= t);
        if idx == 0 || idx >= star.samples.len() {
            continue;
        }
        let (t_a, f_a) = star.samples[idx - 1];
        let (t_b, f_b) = star.samples[idx];
        let (t_s, f_s) = if (t - t_a).abs() <= (t_b - t).abs() {
            (t_a, f_a)
        } else {
            (t_b, f_b)
        };
        records.push((
            p[0],
            p[1],
            p[2],
            f_s as f64,
            t_s,
            star.cadence,
            star.cadence,
            0.0,
            kernel as f64,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            star.freq,
            star.bin_width,
            0.0,
            0.0,
        ));
    }
}

pub struct SenseReq {
    pub field: Arc<Buffer>,
    pub center: [f64; 3],
    pub t: f64,
    pub pad: f64,
    pub cache_interval: f64,
    pub forward: [f64; 3],
    pub expose_offset: f32,
    pub force_ref: [f32; 9],
    pub softening: f64,
}
