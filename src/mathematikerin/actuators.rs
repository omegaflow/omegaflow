use super::*;

const FRAME_TAG: u8 = 0x02;
const MASK_INTENSITY: u8 = 0x01;
const MASK_PAN: u8 = 0x02;
const MASK_TILT: u8 = 0x04;

pub const PCM_SAMPLE_RATE_HZ: u32 = 48000;
pub const PCM_CHANNELS: usize = 2;
pub const PCM_S16_BOUND: f32 = 32767.0;
pub const PCM_SAMPLES_PER_TICK: u32 = PCM_SAMPLE_RATE_HZ * LOOP_TICK_MS as u32 / 1000;

pub type Record = SampleRecord;

pub struct PackedWindow {
    pub field: Vec<f32>,
    pub meta: Vec<f32>,
    pub count: u32,
}

#[derive(Clone, Copy)]
pub struct PresenceFrame {
    pub omega: [f32; 9],
    pub aperture: f32,
    pub pan_ms: Option<f32>,
    pub tilt_ms: Option<f32>,
    pub tau_ticks: u64,
}

pub trait KineticRadiator: Send + 'static {
    fn vibrate(&mut self, frame: &PresenceFrame);
}

pub fn kinetic_sample(frame: &PresenceFrame) -> f32 {
    frame.omega.iter().sum::<f32>() * frame.aperture
}

pub fn frame_bytes(frame: &PresenceFrame) -> Vec<u8> {
    let pan = frame.pan_ms.filter(|v| v.is_finite());
    let tilt = frame.tilt_ms.filter(|v| v.is_finite());
    let mut mask = MASK_INTENSITY;
    if pan.is_some() {
        mask |= MASK_PAN;
    }
    if tilt.is_some() {
        mask |= MASK_TILT;
    }
    let mut out =
        Vec::with_capacity(2 + 4 + (pan.is_some() as usize) * 4 + (tilt.is_some() as usize) * 4);
    out.push(FRAME_TAG);
    out.push(mask);
    out.extend_from_slice(&kinetic_sample(frame).to_le_bytes());
    if let Some(p) = pan {
        out.extend_from_slice(&p.to_le_bytes());
    }
    if let Some(t) = tilt {
        out.extend_from_slice(&t.to_le_bytes());
    }
    out
}

pub struct AcousticOscillator {
    pub _thread: Option<thread::JoinHandle<()>>,
}

pub fn tone_hz(tau_ticks: u64) -> f32 {
    if tau_ticks == 0 {
        return 0.0;
    }
    1000.0 / (tau_ticks as f32 * LOOP_TICK_MS as f32)
}

pub fn acoustic_amplitude(omega_sum: f32, aperture: f32) -> f32 {
    (omega_sum * aperture).clamp(-PCM_S16_BOUND, PCM_S16_BOUND)
}

pub fn acoustic_pcm(frame: &PresenceFrame, phase: &mut f32) -> Vec<u8> {
    let sum: f32 = frame.omega.iter().sum();
    let amp = acoustic_amplitude(sum, frame.aperture);
    let f_hz = tone_hz(frame.tau_ticks);
    let step = f_hz * std::f32::consts::TAU / PCM_SAMPLE_RATE_HZ as f32;
    let mut pcm = Vec::with_capacity(PCM_SAMPLES_PER_TICK as usize * PCM_CHANNELS * 2);
    for _ in 0..PCM_SAMPLES_PER_TICK {
        let s = if f_hz > 0.0 {
            *phase += step;
            if *phase >= std::f32::consts::TAU {
                *phase -= std::f32::consts::TAU;
            }
            (*phase).sin() * amp
        } else {
            0.0
        };
        let left = if sum < 0.0 { s } else { 0.0 };
        let right = if sum > 0.0 { s } else { 0.0 };
        pcm.extend_from_slice(&(left as i16).to_le_bytes());
        pcm.extend_from_slice(&(right as i16).to_le_bytes());
    }
    pcm
}

impl AcousticOscillator {
    pub fn new(
        rx: mpsc::Receiver<PresenceFrame>,
        writer: Option<Box<dyn std::io::Write + Send>>,
    ) -> Self {
        let handle = thread::spawn(move || {
            let Some(mut out) = writer else {
                return;
            };
            let mut phase: f32 = 0.0;
            while let Ok(frame) = rx.recv() {
                let bytes = acoustic_pcm(&frame, &mut phase);
                if std::io::Write::write_all(&mut out, &bytes).is_err()
                    || std::io::Write::flush(&mut out).is_err()
                {
                    break;
                }
            }
        });
        Self {
            _thread: Some(handle),
        }
    }
}

pub struct SeismicOscillator {
    pub port: Option<Box<dyn std::io::Write + Send>>,
}

impl SeismicOscillator {
    pub fn new(writer: Box<dyn std::io::Write + Send>) -> Self {
        Self { port: Some(writer) }
    }
}

impl KineticRadiator for SeismicOscillator {
    fn vibrate(&mut self, frame: &PresenceFrame) {
        let Some(port) = self.port.as_mut() else {
            return;
        };
        let bytes = frame_bytes(frame);
        if std::io::Write::write_all(port, &bytes).is_err() {
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
    for (j, f) in field.as_chunks::<12>().0.iter().enumerate() {
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
        let target = n[ft].div_ceil(2);
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

pub fn color_emission(field: &[f32], meta: &[f32]) -> [f32; 4] {
    let mut acc = [0.0f32; 4];
    for (j, f) in field.as_chunks::<12>().0.iter().enumerate() {
        let ft = f[6] as i64;
        if ft != 0 {
            continue;
        }
        let v = f[3];
        if !v.is_finite() {
            continue;
        }
        let w = v.abs();
        if w <= 0.0 || !w.is_finite() {
            continue;
        }
        let ci = meta[j * 16 + 10] as f64;
        let c = crate::spectral::color_for_ci(ci);
        acc[0] += c[0] * w;
        acc[1] += c[1] * w;
        acc[2] += c[2] * w;
        acc[3] += w;
    }
    acc
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
    pub band: Option<(f64, f64)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack_one(force_type: f64, val: f64, color_index: f64) -> (Vec<f32>, Vec<f32>) {
        let r: SampleRecord = (
            0.0,
            0.0,
            0.0,
            val,
            0.0,
            1.0,
            1.0,
            0.0,
            0.0,
            force_type,
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
            color_index,
            0.0,
            0.0,
            0.0,
            0.0,
        );
        let p = pack_window(&[r], [0.0, 0.0, 0.0]);
        (p.field, p.meta)
    }

    #[test]
    fn color_emission_white_for_absent_color() {
        let (field, meta) = pack_one(0.0, 2.0, 0.0);
        let e = color_emission(&field, &meta);
        assert!(e[3] > 0.0);
        let w = e[3];
        assert!((e[0] / w - 1.0).abs() < 1e-6);
        assert!((e[1] / w - 1.0).abs() < 1e-6);
        assert!((e[2] / w - 1.0).abs() < 1e-6);
    }

    #[test]
    fn color_emission_red_for_red_color_index() {
        let (field, meta) = pack_one(0.0, 2.0, 2.0);
        let e = color_emission(&field, &meta);
        let w = e[3];
        assert!(w > 0.0);
        let (r, g, b) = (e[0] / w, e[1] / w, e[2] / w);
        assert!(r > b, "red source r {r} vs b {b}");
        assert!(g > b, "red source g {g} vs b {b}");
    }

    #[test]
    fn color_emission_ignores_non_em_forces() {
        let (field, meta) = pack_one(1.0, 2.0, 2.0);
        let e = color_emission(&field, &meta);
        assert_eq!(e[3], 0.0, "gravity carries no color of its own");
    }

    #[test]
    fn the_audio_law_is_linear_without_saturation() {
        let frame = PresenceFrame {
            omega: [1.0, -2.0, 3.0, 4.0, -5.0, 6.0, -7.0, 8.0, -9.0],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 1,
        };
        let base = kinetic_sample(&frame) as f64;
        assert_ne!(base, 0.0);
        for lambda in [0.5f64, 2.0, 1e4] {
            let scaled = frame.omega.map(|o| (o as f64 * lambda) as f32);
            let got = kinetic_sample(&PresenceFrame {
                omega: scaled,
                aperture: 1.0,
                pan_ms: None,
                tilt_ms: None,
                tau_ticks: 1,
            }) as f64;
            let want = base * lambda;
            let rel = (got - want).abs() / want.abs();
            assert!(
                rel < 1e-6,
                "lambda {lambda}: {got} vs {want} — the law shapes or saturates"
            );
        }
    }

    #[test]
    fn the_aperture_attenuates_the_raw_sum() {
        let omega = [1.0, -2.0, 3.0, 4.0, -5.0, 6.0, -7.0, 8.0, -9.0];
        let raw = omega.iter().sum::<f32>();
        assert_ne!(raw, 0.0);
        for aperture in [1.0f32, 0.5, 0.0, f32::EPSILON] {
            let got = kinetic_sample(&PresenceFrame {
                omega,
                aperture,
                pan_ms: None,
                tilt_ms: None,
                tau_ticks: 1,
            });
            assert_eq!(got, raw * aperture, "aperture {aperture}");
        }
    }

    #[test]
    fn an_open_aperture_passes_the_raw_sum_untouched() {
        let omega = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let frame = PresenceFrame {
            omega,
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 1,
        };
        assert_eq!(kinetic_sample(&frame), omega.iter().sum::<f32>());
    }

    struct Sink(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for Sink {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().expect("sink lock").extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn the_seismic_wire_carries_the_raw_sum() {
        let bytes = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut osc = SeismicOscillator::new(Box::new(Sink(bytes.clone())));
        let frame = PresenceFrame {
            omega: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 1,
        };
        osc.vibrate(&frame);
        let got = bytes.lock().expect("sink lock").clone();
        let mut expected = [0u8; 6];
        expected[0] = 0x02;
        expected[1] = 0x01;
        expected[2..6].copy_from_slice(&kinetic_sample(&frame).to_le_bytes());
        assert_eq!(got, expected);
    }

    #[test]
    fn the_tone_maps_inverse_to_natural_latency_ticks() {
        let base = tone_hz(1);
        assert!((base - 62.5).abs() < 1e-3, "τ=1 → 62.5 Hz, got {base}");
        for tau in [2u64, 3, 4, 8, 16, 64] {
            let f = tone_hz(tau);
            assert!(f > 0.0 && f.is_finite());
            assert!(
                (62.5 / tau as f32 - f).abs() < 1e-3,
                "τ={tau} → {} Hz, got {f}",
                62.5 / tau as f32
            );
        }
        assert_eq!(tone_hz(0), 0.0, "τ = 0: no temporal extent, no tone");
    }

    #[test]
    fn the_acoustic_amplitude_clamps_at_the_s16_bound() {
        assert_eq!(acoustic_amplitude(9.0, 1.0), 9.0);
        assert_eq!(acoustic_amplitude(-4.0, 2.0), -8.0);
        assert_eq!(acoustic_amplitude(1.0e9, 1.0), PCM_S16_BOUND);
        assert_eq!(acoustic_amplitude(-1.0e9, 1.0), -PCM_S16_BOUND);
        assert_eq!(acoustic_amplitude(0.0, 1.0), 0.0);
        assert_eq!(acoustic_amplitude(5.0, 0.0), 0.0);
    }

    #[test]
    fn the_acoustic_channel_emits_s16_stereo_pcm_with_signed_steering() {
        let mut phase = 0.0f32;
        let frame = PresenceFrame {
            omega: [100.0; 9],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 1,
        };
        let pcm = acoustic_pcm(&frame, &mut phase);
        assert_eq!(
            pcm.len(),
            PCM_SAMPLES_PER_TICK as usize * PCM_CHANNELS * 2,
            "one tick of interleaved S16 stereo"
        );
        let mut max_abs = 0i32;
        let mut min_r = 0i32;
        let mut max_r = 0i32;
        for pair in pcm.chunks_exact(4) {
            let l = i16::from_le_bytes([pair[0], pair[1]]) as i32;
            let r = i16::from_le_bytes([pair[2], pair[3]]) as i32;
            assert_eq!(l, 0, "a positive Σω routes the tone right");
            max_abs = max_abs.max(r.abs());
            min_r = min_r.min(r);
            max_r = max_r.max(r);
        }
        assert!(min_r < 0 && max_r > 0, "the tone oscillates");
        assert!(max_abs <= PCM_S16_BOUND as i32, "the format is the clamp");
        assert!(max_abs >= 800, "the tone carries the amplitude: {max_abs}");
    }

    #[test]
    fn a_negative_sum_routes_the_tone_left() {
        let mut phase = 0.0f32;
        let frame = PresenceFrame {
            omega: [-100.0; 9],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 2,
        };
        let pcm = acoustic_pcm(&frame, &mut phase);
        let mut min_l = 0i32;
        let mut max_l = 0i32;
        for pair in pcm.chunks_exact(4) {
            let l = i16::from_le_bytes([pair[0], pair[1]]) as i32;
            let r = i16::from_le_bytes([pair[2], pair[3]]) as i32;
            assert_eq!(r, 0, "a negative Σω routes the tone left");
            min_l = min_l.min(l);
            max_l = max_l.max(l);
        }
        assert!(min_l < 0 && max_l > 0, "the tone oscillates");
    }

    #[test]
    fn a_zero_sum_is_silence_on_both_channels() {
        let mut phase = 0.0f32;
        let frame = PresenceFrame {
            omega: [1.0, -1.0, 2.0, -2.0, 3.0, -3.0, 4.0, -4.0, 0.0],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 1,
        };
        let pcm = acoustic_pcm(&frame, &mut phase);
        assert_eq!(pcm.len(), PCM_SAMPLES_PER_TICK as usize * PCM_CHANNELS * 2);
        assert!(
            pcm.iter().all(|&b| b == 0),
            "zero Σω → zero amplitude — silence is the response"
        );
    }

    #[test]
    fn the_acoustic_phase_continues_across_frames() {
        let mut phase = 0.0f32;
        let frame = PresenceFrame {
            omega: [100.0; 9],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 3,
        };
        let _ = acoustic_pcm(&frame, &mut phase);
        let second = acoustic_pcm(&frame, &mut phase);
        let first_of_second = i16::from_le_bytes([second[2], second[3]]) as i32;
        let expected = ((2.0 * std::f64::consts::PI / 3.0).sin() * 900.0) as i32;
        assert!(
            (first_of_second - expected).abs() <= 2,
            "the phase carries over the frame boundary: got {first_of_second}, expected ≈ {expected}"
        );
    }

    #[test]
    fn a_tau_step_changes_pitch_without_an_edge() {
        let mut phase = 0.0f32;
        let fast = PresenceFrame {
            omega: [100.0; 9],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 1,
        };
        let slow = PresenceFrame {
            omega: [100.0; 9],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 2,
        };
        let first = acoustic_pcm(&fast, &mut phase);
        let last_of_first =
            i16::from_le_bytes([first[first.len() - 2], first[first.len() - 1]]) as i32;
        let second = acoustic_pcm(&slow, &mut phase);
        let first_of_second = i16::from_le_bytes([second[2], second[3]]) as i32;
        assert!(
            (first_of_second - last_of_first).abs() <= 100,
            "a τ step bends the tone, it does not cut it: {last_of_first} → {first_of_second}"
        );
        let negatives = |pcm: &[u8]| -> usize {
            pcm.chunks_exact(4)
                .filter(|p| i16::from_le_bytes([p[2], p[3]]) < 0)
                .count()
        };
        assert_ne!(
            negatives(&first),
            negatives(&second),
            "the pitch changed between τ=1 and τ=2"
        );
    }

    #[test]
    fn the_acoustic_oscillator_writes_pcm_into_the_memory_writer() {
        let bytes = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let (tx, rx) = mpsc::channel::<PresenceFrame>();
        let osc = AcousticOscillator::new(rx, Some(Box::new(Sink(bytes.clone()))));
        let frame = PresenceFrame {
            omega: [100.0; 9],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 1,
        };
        tx.send(frame).expect("frame reaches the oscillator");
        drop(tx);
        if let Some(h) = osc._thread {
            let _ = h.join();
        }
        let got = bytes.lock().expect("sink lock").clone();
        assert_eq!(got.len(), PCM_SAMPLES_PER_TICK as usize * PCM_CHANNELS * 2);
        let l = i16::from_le_bytes([got[0], got[1]]);
        let r = i16::from_le_bytes([got[2], got[3]]);
        assert_eq!(l, 0);
        assert!(r > 0, "the first sample rides the right channel: {r}");
    }

    #[test]
    fn a_frame_with_pan_and_tilt_emits_the_full_mask_and_three_payloads() {
        let frame = PresenceFrame {
            omega: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9],
            aperture: 1.0,
            pan_ms: Some(1.5),
            tilt_ms: Some(1.25),
            tau_ticks: 1,
        };
        let bytes = frame_bytes(&frame);
        assert_eq!(bytes[0], 0x02);
        assert_eq!(bytes[1], 0x07);
        assert_eq!(
            bytes.len(),
            14,
            "tag, mask, then Σω, pan, tilt in bit order"
        );
        let intensity = f32::from_le_bytes(bytes[2..6].try_into().expect("intensity"));
        assert_eq!(intensity, kinetic_sample(&frame));
        let pan = f32::from_le_bytes(bytes[6..10].try_into().expect("pan"));
        assert_eq!(pan, 1.5);
        let tilt = f32::from_le_bytes(bytes[10..14].try_into().expect("tilt"));
        assert_eq!(tilt, 1.25);
    }

    #[test]
    fn a_present_but_non_finite_pan_clears_its_bit() {
        let frame = PresenceFrame {
            omega: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
            aperture: 1.0,
            pan_ms: Some(f32::NAN),
            tilt_ms: None,
            tau_ticks: 1,
        };
        let bytes = frame_bytes(&frame);
        assert_eq!(bytes[1], 0x01, "non-finite pan is absence, not a value");
        assert_eq!(bytes.len(), 6, "no pan bytes ride the wire");
    }

    #[test]
    fn a_frame_without_pan_or_tilt_carries_one_payload() {
        let frame = PresenceFrame {
            omega: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
            aperture: 1.0,
            pan_ms: None,
            tilt_ms: None,
            tau_ticks: 1,
        };
        let bytes = frame_bytes(&frame);
        assert_eq!(bytes[1], 0x01);
        assert_eq!(bytes.len(), 6);
    }
}
