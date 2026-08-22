use super::*;

pub const Φ: f64 = 1.618033988749895;

pub const C: f64 = 299792458.0;

pub const GRID_INIT: f64 = 2147483648.0;

pub const JUMP_GRID: f64 = 268435456.0;

pub const SSAA_MAX: f32 = 8.0;

pub const BUDGET_RELAX: f64 = 0.1;

pub const PERM_GROUND: f32 = f32::EPSILON;

pub const FORCE_NAME: [&str; 9] = [
    "em",
    "gravity",
    "acoustic",
    "seismic-body",
    "seismic-surface",
    "thermal",
    "diffusion",
    "advective",
    "electric",
];

pub const FORCE_SI_UNIT: [&str; 9] = ["W/m2", "m/s2", "Pa", "m", "m", "K", "kg/m3", "m/s", "V/m"];

pub const FIELD_BACKING_SCALE: f64 = 1.0;

pub const EMA_FACTOR: f64 = 0.05;

pub const EXPOSE_OFFSET_BASE: f32 = 4.0;

pub const OFFSET_RELAX: f32 = 0.03125;

pub const REF_RELAX: f32 = 0.0625;

pub const THRUST_STEP: f64 = 64.0;

pub const JUMP_BODIES: [&str; 9] = [
    "sun", "mercury", "venus", "earth", "mars", "jupiter", "saturn", "uranus", "neptune",
];

pub fn q_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ]
}

pub fn q_norm(q: [f64; 4]) -> [f64; 4] {
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3])
        .sqrt()
        .max(1e-12);
    [q[0] / n, q[1] / n, q[2] / n, q[3] / n]
}

pub fn q_rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let p = [0.0, v[0], v[1], v[2]];
    let c = [q[0], -q[1], -q[2], -q[3]];
    let r = q_mul(q_mul(q, p), c);
    [r[1], r[2], r[3]]
}

pub fn q_axis_angle(axis: [f64; 3], angle: f64) -> [f64; 4] {
    let s = (angle / 2.0).sin();
    [(angle / 2.0).cos(), axis[0] * s, axis[1] * s, axis[2] * s]
}

pub const WINDOW_STATE_PATH: &str = "/tmp/omegaflow_window_state.φ";

pub fn window_state_load(path: &str) -> (f64, [f64; 3], [f64; 4]) {
    let mut grid = GRID_INIT;
    let mut p = [0.0f64; 3];
    let mut q = [1.0, 0.0, 0.0, 0.0];
    let Ok(text) = std::fs::read_to_string(path) else {
        return (grid, p, q);
    };
    for line in text.lines() {
        let mut toks = line.split_whitespace();
        match toks.next() {
            Some("grid_step") => {
                if let Some(v) = toks.next().and_then(|s| s.parse::<f64>().ok()) {
                    if v.is_finite() && v > 0.0 {
                        grid = v;
                    }
                }
            }
            Some("p") => {
                let vals: Option<Vec<f64>> = toks.map(|s| s.parse::<f64>().ok()).collect();
                if let Some(vals) = vals {
                    if vals.len() == 3 && vals.iter().all(|v| v.is_finite()) {
                        p = [vals[0], vals[1], vals[2]];
                    }
                }
            }
            Some("q") => {
                let vals: Option<Vec<f64>> = toks.map(|s| s.parse::<f64>().ok()).collect();
                if let Some(vals) = vals {
                    if vals.len() == 4
                        && vals.iter().all(|v| v.is_finite())
                        && vals.iter().any(|v| *v != 0.0)
                    {
                        q = q_norm([vals[0], vals[1], vals[2], vals[3]]);
                    }
                }
            }
            _ => {}
        }
    }
    (grid, p, q)
}

pub fn window_state_save(path: &str, grid_step: f64, p: [f64; 3], q: [f64; 4]) {
    let mut text = String::new();
    text.push_str(&format!("grid_step {:.17e}\n", grid_step));
    text.push_str(&format!("p {:.17e} {:.17e} {:.17e}\n", p[0], p[1], p[2]));
    text.push_str(&format!(
        "q {:.17e} {:.17e} {:.17e} {:.17e}\n",
        q[0], q[1], q[2], q[3]
    ));
    let _ = std::fs::write(path, text);
}

pub fn storage_entry(
    read_only: bool,
    visibility: wgpu::ShaderStages,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub const CAPTURE_RING_SIZE: usize = 32;

pub struct NativeOsc {
    pub ring: [f64; CAPTURE_RING_SIZE],
    pub idx: usize,
    pub filled: usize,
    pub median: f64,
    pub last_sent: f64,
    pub tau: f64,
}

pub struct NativeSensors {
    pub oscs: HashMap<String, NativeOsc>,
    pub tx: mpsc::Sender<Vec<(String, f64, f64)>>,
    pub frame_interval: f64,
}

impl NativeSensors {
    pub fn record_sample(&mut self, name: &str, value: f64) {
        if !value.is_finite() {
            return;
        }
        let osc = self.oscs.entry(name.to_string()).or_insert(NativeOsc {
            ring: [0.0; CAPTURE_RING_SIZE],
            idx: 0,
            filled: 0,
            median: value,
            last_sent: value,
            tau: self.frame_interval,
        });
        osc.tau = self.frame_interval;
        osc.ring[osc.idx] = value;
        osc.idx = (osc.idx + 1) % CAPTURE_RING_SIZE;
        if osc.filled < CAPTURE_RING_SIZE {
            osc.filled += 1;
        }
        osc.median = osc.median * (1.0 - EMA_FACTOR) + value * EMA_FACTOR;
    }

    pub fn flush(&mut self) {
        let mut list = Vec::new();
        for (name, osc) in self.oscs.iter_mut() {
            if !osc.median.is_finite() {
                continue;
            }
            if (osc.median - osc.last_sent).abs() > f64::EPSILON {
                osc.last_sent = osc.median;
                list.push((name.clone(), osc.median, osc.tau));
            }
        }
        if !list.is_empty() {
            if self.tx.send(list).is_err() {
                eprintln!("sensor channel closed — samples dropped");
            }
        }
    }
}

pub struct NativeApp {
    pub rx: mpsc::Receiver<Arc<Buffer>>,
    pub req_tx: mpsc::SyncSender<SenseReq>,
    pub res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
    pub presence_tx: mpsc::Sender<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    pub body_names: Arc<Vec<String>>,
    pub time: Arc<Mutex<Option<LeapSeconds>>>,
    pub shutdown: Arc<AtomicBool>,
    pub consent: Arc<AtomicBool>,
    pub acoustic_tx: mpsc::Sender<PresenceFrame>,
    pub seismic_tx: mpsc::Sender<PresenceFrame>,
    pub silent: bool,

    pub window: Option<Arc<Window>>,
    pub surface: Option<wgpu::Surface<'static>>,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub render_pipe: Option<wgpu::RenderPipeline>,
    pub probe_pipe: Option<wgpu::ComputePipeline>,
    pub probe_layout: Option<wgpu::BindGroupLayout>,
    pub render_layout: Option<wgpu::BindGroupLayout>,
    pub render_binds: [Option<wgpu::BindGroup>; 2],
    pub probe_binds: [Option<wgpu::BindGroup>; 2],
    pub field_bufs: [Option<wgpu::Buffer>; 2],
    pub meta_bufs: [Option<wgpu::Buffer>; 2],
    pub vp_buf: Option<wgpu::Buffer>,
    pub probe_buf: Option<wgpu::Buffer>,
    pub probe_read: Option<wgpu::Buffer>,
    pub prep_param_buf: Option<wgpu::Buffer>,
    pub te_pipe: Option<wgpu::ComputePipeline>,
    pub te_bind: Option<wgpu::BindGroup>,
    pub te_series_buf: Option<wgpu::Buffer>,
    pub te_param_buf: Option<wgpu::Buffer>,
    pub te_out_buf: Option<wgpu::Buffer>,
    pub te_read_buf: Option<wgpu::Buffer>,
    pub te_map: Option<Arc<AtomicBool>>,
    pub te_named: String,
    pub color_lut_view: Option<wgpu::TextureView>,
    pub color_lut_sampler: Option<wgpu::Sampler>,
    pub field_cap: u32,
    pub buf_sel: usize,
    pub packed_gen: u64,
    pub uploaded_gen: u64,
    pub backing: (u32, u32),

    pub latest_field: Option<Arc<Buffer>>,
    pub packed_field: Vec<f32>,
    pub packed_meta: Vec<f32>,
    pub packed_count: u32,
    pub last_response_epoch: f64,

    pub p: [f64; 3],
    pub v: [f64; 3],
    pub t0: f64,
    pub t_presence: f64,
    pub q: [f64; 4],
    pub grid_step: f64,
    pub ssaa: f32,
    pub expose_offset: f32,
    pub field_dark: bool,
    pub hud_dark: bool,
    pub force_ref: [f32; 9],
    pub probe_omega: [f32; 9],
    pub probe_flow: [f32; 3],
    pub probe_ring: [[f32; 12]; 256],
    pub ring_head: usize,
    pub ring_filled: usize,
    pub ring_gen: u64,
    pub pe_ring: Vec<f64>,
    pub solar: SolarMachine,
    pub enso: EnsoMachine,
    pub hud_topology: Option<(usize, usize, Option<f64>, Option<f64>)>,
    pub frame_ms_ema: f64,
    pub field_permeability: f32,
    pub prev_omega_sum: f32,
    pub prev_delta: f32,
    pub prev_in_te: f64,
    pub direction: i32,
    pub ticks_since_turn: u64,
    pub natural_latency_ticks: u64,
    pub t_thrust: f64,
    pub t_thrust_target: f64,
    pub t_frozen: bool,
    pub keys: HashSet<KeyCode>,
    pub shift: bool,
    pub focus_req: u32,
    pub focused: bool,
    pub keys_seen: u64,
    pub cursor: Option<(f64, f64)>,
    pub drag_button: Option<MouseButton>,
    pub touches: HashMap<u64, (f64, f64)>,
    pub tap: Option<(std::time::Instant, (f64, f64))>,
    pub press: Option<(std::time::Instant, u64, (f64, f64))>,
    #[cfg(feature = "gamepad")]
    pub gilrs: Option<gilrs::Gilrs>,
    pub last_tick: Option<std::time::Instant>,
    pub stable_tick: f64,
    pub size: (u32, u32),
    pub scale_factor: f64,
    pub last_sent: (f64, f64, f64, f64, f64, [f64; 3], f64),
    pub last_saved_state: (f64, [f64; 3], [f64; 4]),
    pub sensors: NativeSensors,
    pub frame_count: u64,
    pub frame_ms_max: f64,
    pub last_hud: Option<std::time::Instant>,
    pub hud_tex: Option<wgpu::Texture>,
    pub hud_view: Option<wgpu::TextureView>,
    pub hud_sampler: Option<wgpu::Sampler>,
    pub hud_pipe: Option<wgpu::RenderPipeline>,
    pub hud_layout: Option<wgpu::BindGroupLayout>,
    pub hud_bind: Option<wgpu::BindGroup>,
    pub hud_bitmap: Vec<u8>,
    pub hud_w: u32,
    pub hud_dirty: bool,
}

impl NativeApp {
    pub fn new(
        rx: mpsc::Receiver<Arc<Buffer>>,
        req_tx: mpsc::SyncSender<SenseReq>,
        res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
        presence_tx: mpsc::Sender<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
        sensor_tx: mpsc::Sender<Vec<(String, f64, f64)>>,
        body_names: Arc<Vec<String>>,
        time: Arc<Mutex<Option<LeapSeconds>>>,
        shutdown: Arc<AtomicBool>,
        consent: Arc<AtomicBool>,
        acoustic_tx: mpsc::Sender<PresenceFrame>,
        seismic_tx: mpsc::Sender<PresenceFrame>,
        solar_rx: mpsc::Receiver<SolarCell>,
        enso_rx: mpsc::Receiver<EnsoCell>,
    ) -> Self {
        let (grid0, p0, q0) = window_state_load(WINDOW_STATE_PATH);
        Self {
            rx,
            req_tx,
            res_rx,
            presence_tx,
            body_names,
            time,
            shutdown,
            consent,
            acoustic_tx,
            seismic_tx,
            silent: std::env::var("OMEGAFLOW_HIDDEN").is_ok(),
            window: None,
            surface: None,
            config: None,
            device: None,
            queue: None,
            render_pipe: None,
            probe_pipe: None,
            probe_layout: None,
            render_layout: None,
            render_binds: [None, None],
            probe_binds: [None, None],
            field_bufs: [None, None],
            meta_bufs: [None, None],
            vp_buf: None,
            probe_buf: None,
            probe_read: None,
            prep_param_buf: None,
            te_pipe: None,
            te_bind: None,
            te_series_buf: None,
            te_param_buf: None,
            te_out_buf: None,
            te_read_buf: None,
            te_map: None,
            te_named: String::new(),
            field_cap: 0,
            buf_sel: 0,
            packed_gen: 0,
            uploaded_gen: 0,
            backing: (1, 1),
            latest_field: None,
            packed_field: Vec::new(),
            packed_meta: Vec::new(),
            packed_count: 0,
            last_response_epoch: 0.0,
            p: p0,
            v: [0.0, 0.0, 0.0],
            t0: 0.0,
            t_presence: 0.0,
            q: q0,
            grid_step: grid0,
            ssaa: 1.0,
            expose_offset: EXPOSE_OFFSET_BASE,
            field_dark: false,
            hud_dark: false,
            force_ref: [0.0; 9],
            probe_omega: [0.0; 9],
            probe_flow: [0.0; 3],
            probe_ring: [[0.0; 12]; 256],
            ring_head: 0,
            ring_filled: 0,
            ring_gen: 0,
            pe_ring: Vec::with_capacity(16),
            solar: SolarMachine::new(solar_rx),
            enso: EnsoMachine::new(enso_rx),
            hud_topology: None,
            frame_ms_ema: 0.0,
            field_permeability: 0.0,
            prev_omega_sum: 0.0,
            prev_delta: 0.0,
            prev_in_te: 0.0,
            direction: 1,
            ticks_since_turn: 0,
            natural_latency_ticks: 1,
            t_thrust: 0.0,
            t_thrust_target: 0.0,
            t_frozen: false,
            keys: HashSet::new(),
            shift: false,
            focus_req: 0,
            focused: false,
            keys_seen: 0,
            cursor: None,
            drag_button: None,
            touches: HashMap::new(),
            tap: None,
            press: None,
            #[cfg(feature = "gamepad")]
            gilrs: gilrs::GilrsBuilder::new().build().ok(),
            last_tick: None,
            stable_tick: 0.0,
            size: (1280, 800),
            scale_factor: 1.0,
            last_sent: (0.0, 0.0, 0.0, 0.0, 0.0, [0.0; 3], 0.0),
            last_saved_state: (grid0, p0, q0),
            sensors: NativeSensors {
                oscs: HashMap::new(),
                tx: sensor_tx,
                frame_interval: 0.016,
            },
            frame_count: 0,
            frame_ms_max: 0.0,
            last_hud: None,
            hud_tex: None,
            hud_view: None,
            hud_sampler: None,
            color_lut_view: None,
            color_lut_sampler: None,
            hud_pipe: None,
            hud_layout: None,
            hud_bind: None,
            hud_bitmap: Vec::new(),
            hud_w: 0,
            hud_dirty: false,
        }
    }

    pub fn pos(&self) -> [f64; 3] {
        let dt = self.t_presence - self.t0;
        [
            self.p[0] + self.v[0] * dt,
            self.p[1] + self.v[1] * dt,
            self.p[2] + self.v[2] * dt,
        ]
    }

    pub fn frame(&self) -> ([f64; 3], [f64; 3], [f64; 3]) {
        (
            q_rotate(self.q, [1.0, 0.0, 0.0]),
            q_rotate(self.q, [0.0, 1.0, 0.0]),
            q_rotate(self.q, [0.0, 0.0, 1.0]),
        )
    }

    pub fn fold(&mut self) {
        self.p = self.pos();
        self.t0 = self.t_presence;
    }

    pub fn pe_gate(&mut self, pe: Option<f64>) -> bool {
        let Some(pe) = pe else {
            return true;
        };
        if self.pe_ring.len() == 16 {
            self.pe_ring.remove(0);
        }
        self.pe_ring.push(pe);
        if self.pe_ring.len() < 8 {
            return true;
        }
        let n = self.pe_ring.len() as f64;
        let mean = self.pe_ring.iter().sum::<f64>() / n;
        let var = self
            .pe_ring
            .iter()
            .map(|&p| {
                let d = p - mean;
                d * d
            })
            .sum::<f64>()
            / n;
        (pe - mean).abs() <= 2.0 * var.sqrt()
    }

    pub fn te_say(&mut self, word: &str) {
        if self.te_named != word {
            eprintln!("te {}", word);
            self.te_named = word.to_string();
        }
    }

    pub fn te_probe(
        &mut self,
        xs: &[f32],
        ys: &[f32],
        m: usize,
    ) -> Option<crate::te::TopologicalVerdict> {
        let device = self.device.clone()?;
        let mut carry: Option<crate::te::TopologicalVerdict> = None;
        if let Some(prev) = self.te_map.take() {
            if prev.load(Ordering::SeqCst) {
                let read_buf = self.te_read_buf.as_ref()?;
                let verdict = te_read_verdict(read_buf);
                carry = crate::te::topological_verdict_from_gpu(&verdict);
            } else {
                self.te_map = Some(prev);
                self.te_say("readback pending");
                return None;
            }
        }
        let queue = self.queue.clone()?;
        let pipe = self.te_pipe.as_ref()?;
        let bind = self.te_bind.as_ref()?;
        let series_buf = self.te_series_buf.as_ref()?;
        let param_buf = self.te_param_buf.as_ref()?;
        let out_buf = self.te_out_buf.as_ref()?;
        let read_buf = self.te_read_buf.as_ref()?;
        let mut data = vec![0f32; 12 * TE_SERIES_STRIDE];
        data[0..m].copy_from_slice(xs);
        data[TE_SERIES_STRIDE..TE_SERIES_STRIDE + m].copy_from_slice(ys);
        let mut rng = self.ring_gen.wrapping_add(0x9e3779b97f4a7c15);
        for s in 0..10 {
            let surr = crate::te::phase_randomized_surrogate(ys, &mut rng);
            let off = (2 + s) * TE_SERIES_STRIDE;
            data[off..off + m].copy_from_slice(&surr);
        }
        queue.write_buffer(series_buf, 0, &le_bytes_f32(&data));
        let max_lag = (m as f64 / Φ) as u32;
        let param = [m as u32, max_lag, 1.0f32.to_bits(), 0];
        let mut pb = [0u8; 16];
        for (i, x) in param.iter().enumerate() {
            pb[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        queue.write_buffer(param_buf, 0, &pb);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(pipe);
            pass.set_bind_group(0, bind, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        enc.copy_buffer_to_buffer(out_buf, 0, read_buf, 0, 288);
        queue.submit(std::iter::once(enc.finish()));
        let mapped = Arc::new(AtomicBool::new(false));
        let m2 = mapped.clone();
        let slice = read_buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, move |r| {
            m2.store(r.is_ok(), Ordering::SeqCst);
        });
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(100);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            device.poll(wgpu::Maintain::Poll);
        }
        if !mapped.load(Ordering::SeqCst) {
            self.te_map = Some(mapped);
            if carry.is_some() {
                self.te_say("verdict present");
            } else {
                self.te_say("readback pending");
            }
            return carry;
        }
        let verdict = te_read_verdict(read_buf);
        let v = crate::te::topological_verdict_from_gpu(&verdict);
        let final_v = v.or(carry);
        if final_v.is_some() {
            self.te_say("verdict present");
        } else {
            self.te_say(te_absence_word(&verdict));
        }
        final_v
    }

    pub fn hud_blit(bmp: &mut [u8], stride: usize, w: u32, x: i32, y: i32, ch: char, rgb: [u8; 3]) {
        let c = ch as u32;
        if !(32..=126).contains(&c) {
            return;
        }
        let glyph = &HUD_GLYPH[(c - 32) as usize];
        for (col, bits) in glyph.iter().enumerate() {
            let px = x + col as i32;
            if px < 0 || px >= w as i32 {
                continue;
            }
            for row in 0..7u32 {
                if bits & (1 << row) == 0 {
                    continue;
                }
                let py = y + row as i32;
                if py < 0 || py >= HUD_H as i32 {
                    continue;
                }
                let o = py as usize * stride + px as usize * 4;
                bmp[o] = rgb[0];
                bmp[o + 1] = rgb[1];
                bmp[o + 2] = rgb[2];
                bmp[o + 3] = 255;
            }
        }
    }

    pub fn hud_text(
        bmp: &mut [u8],
        stride: usize,
        w: u32,
        mut x: i32,
        y: i32,
        s: &str,
        rgb: [u8; 3],
    ) {
        for ch in s.chars() {
            Self::hud_blit(bmp, stride, w, x, y, ch, rgb);
            x += HUD_CHAR_W;
        }
    }

    pub fn hud_raster(&mut self, force_tokens: &str, te: Option<(f64, f64)>, te_word: &str) {
        if self.hud_bitmap.is_empty() {
            return;
        }
        let w = self.hud_w;
        let stride = (w as usize * 4 + 255) / 256 * 256;
        for b in self.hud_bitmap.iter_mut() {
            *b = 0;
        }
        let green = [120u8, 255, 140];
        let [x, y, z] = self.pos();
        let line1 = format!(
            "t {:.2}  x {:.3e}  y {:.3e}  z {:.3e}",
            self.t_presence, x, y, z
        );
        let flow = self.probe_flow;
        let mut l3 = match te {
            Some((in_te, thr)) => format!("TE {:.3} thr {:.3}  ", in_te, thr),
            None if te_word.is_empty() => String::new(),
            None => format!("TE {}  ", te_word),
        };
        if let Some((tx, ty, px, py)) = self.hud_topology {
            l3.push_str(&format!("tau {}:{} ", tx, ty));
            match (px, py) {
                (Some(a), Some(b)) => l3.push_str(&format!("PE {:.2}:{:.2} ", a, b)),
                _ => l3.push_str("PE - "),
            }
        }
        l3.push_str(&format!(
            "perm {:.2} flow {:+.2} {:+.2} {:+.2} gen {}",
            self.field_permeability, flow[0], flow[1], flow[2], self.ring_gen
        ));
        let x0 = 4i32;
        let line = HUD_LINE_H;
        let bmp = &mut self.hud_bitmap;
        let scale_line = format!(
            "1 px = {} | grid 2^{} | H hud  P feld | {} | keys {}",
            Self::scale_label(self.grid_step),
            self.grid_step.log2().round() as i64,
            if self.focused { "focus" } else { "kein focus" },
            self.keys_seen
        );
        Self::hud_text(bmp, stride, w, x0, 1, &line1, green);
        Self::hud_text(bmp, stride, w, x0, 1 + line, force_tokens, green);
        Self::hud_text(bmp, stride, w, x0, 1 + 2 * line, &l3, green);
        Self::hud_text(bmp, stride, w, x0, 1 + 3 * line, &scale_line, green);
        self.hud_dirty = true;
    }

    pub fn scale_label(m_per_px: f64) -> String {
        if m_per_px >= 1.0e14 {
            format!("{:.2} AU", m_per_px / 1.495978707e11)
        } else if m_per_px >= 1.0e8 {
            format!("{:.3} Mkm", m_per_px / 1.0e9)
        } else if m_per_px >= 1.0e3 {
            format!("{:.1} km", m_per_px / 1.0e3)
        } else {
            format!("{:.1} m", m_per_px)
        }
    }

    pub fn ensure_hud_texture(&mut self) {
        let Some(device) = self.device.clone() else {
            return;
        };
        let w = self.backing.0.max(1);
        if self.hud_w == w && self.hud_tex.is_some() && self.hud_bind.is_some() {
            return;
        }
        self.hud_w = w;
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: w,
                height: HUD_H,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let stride = (w as usize * 4 + 255) / 256 * 256;
        self.hud_bitmap = vec![0u8; stride * HUD_H as usize];
        if let (Some(sampler), Some(layout), Some(vp_buf)) = (
            self.hud_sampler.clone(),
            self.hud_layout.clone(),
            self.vp_buf.clone(),
        ) {
            self.hud_bind = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: vp_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 10,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 11,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            }));
        }
        self.hud_tex = Some(tex);
        self.hud_view = Some(view);
        self.hud_dirty = true;
    }

    pub fn sense(&mut self) {
        let Some(field) = self.latest_field.clone() else {
            return;
        };
        let center = self.pos();
        let t = self.t_presence;
        let hx = self.size.0 as f64 * self.scale_factor * self.grid_step * 0.5;
        let hy = self.size.1 as f64 * self.scale_factor * self.grid_step * 0.5;
        let pad = 2.0 * (hx * hx + hy * hy).sqrt();
        let cache_interval = (self.grid_step / 30000.0).clamp(Φ, Φ * 10.0);
        let (_, _, forward) = self.frame();
        let _ = self.req_tx.try_send(SenseReq {
            field,
            center,
            t,
            pad,
            cache_interval,
            forward,
            expose_offset: self.expose_offset,
            force_ref: self.force_ref,
            softening: self.grid_step,
        });
    }

    pub fn consider_resend(&mut self) {
        let [x, y, z] = self.pos();
        let (lt, lx, ly, lz, ls, lv, ltt) = self.last_sent;
        let moved = self.t_presence != lt
            || (x - lx).abs() >= self.grid_step
            || (y - ly).abs() >= self.grid_step
            || (z - lz).abs() >= self.grid_step
            || self.grid_step > ls * Φ
            || ls > self.grid_step * Φ
            || self.v != lv
            || self.t_thrust != ltt;
        if !moved {
            return;
        }
        self.last_sent = (
            self.t_presence,
            x,
            y,
            z,
            self.grid_step,
            self.v,
            self.t_thrust,
        );
        let range = self.size.0.max(self.size.1) as f64 * self.scale_factor * self.grid_step * 2.0;
        let _ = self.presence_tx.send((
            "native".to_string(),
            self.t_presence,
            x,
            y,
            z,
            range,
            self.v[0],
            self.v[1],
            self.v[2],
            self.t_thrust,
            self.grid_step,
        ));
        self.sense();
    }

    pub fn consider_state_save(&mut self) {
        let state = (self.grid_step, self.p, self.q);
        if state.0 == self.last_saved_state.0
            && state.1 == self.last_saved_state.1
            && state.2 == self.last_saved_state.2
        {
            return;
        }
        self.last_saved_state = state;
        window_state_save(WINDOW_STATE_PATH, state.0, state.1, state.2);
    }

    pub fn key_action(&mut self, code: KeyCode) {
        match code {
            KeyCode::KeyS => {
                self.fold();
                self.v = [0.0, 0.0, 0.0];
                self.consider_resend();
            }
            KeyCode::KeyY => {
                self.consent.store(true, Ordering::SeqCst);
            }
            KeyCode::KeyN => {
                self.consent.store(false, Ordering::SeqCst);
            }
            KeyCode::Home | KeyCode::Digit0 => {
                self.p = [0.0, 0.0, 0.0];
                self.v = [0.0, 0.0, 0.0];
                self.t0 = self.t_presence;
                self.consider_resend();
            }
            KeyCode::Space => {
                self.t_frozen = !self.t_frozen;
            }
            KeyCode::KeyB => {
                self.q = [1.0, 0.0, 0.0, 0.0];
            }
            KeyCode::Minus => {
                self.grid_step *= 4.0;
                self.consider_resend();
            }
            KeyCode::Equal => {
                self.grid_step /= 4.0;
                self.consider_resend();
            }
            KeyCode::KeyQ => {
                self.ssaa = if self.shift {
                    (self.ssaa / Φ as f32).max(1.0)
                } else {
                    (self.ssaa * Φ as f32).min(SSAA_MAX)
                };
                self.reconfigure();
            }
            KeyCode::KeyE => {
                self.expose_offset = if self.shift {
                    self.expose_offset / Φ as f32
                } else {
                    self.expose_offset * 2.0
                };
            }
            KeyCode::KeyP => {
                self.field_dark = !self.field_dark;
            }
            KeyCode::KeyH => {
                self.hud_dark = !self.hud_dark;
            }
            KeyCode::Digit1 => self.jump(0),
            KeyCode::Digit2 => self.jump(1),
            KeyCode::Digit3 => self.jump(2),
            KeyCode::Digit4 => self.jump(3),
            KeyCode::Digit5 => self.jump(4),
            KeyCode::Digit6 => self.jump(5),
            KeyCode::Digit7 => self.jump(6),
            KeyCode::Digit8 => self.jump(7),
            KeyCode::Digit9 => self.jump(8),
            _ => {}
        }
    }

    pub fn jump(&mut self, idx: usize) {
        let Some(target) = JUMP_BODIES.get(idx) else {
            return;
        };
        let Some(field) = self.latest_field.clone() else {
            return;
        };
        let Some(name) = self.body_names.iter().find(|n| n.as_str() == *target) else {
            return;
        };
        let eph = field.eph.clone();
        let Some(pos) = body_barycenter_position(name, self.t_presence, &eph) else {
            return;
        };
        self.p = pos;
        self.v = [0.0, 0.0, 0.0];
        self.t0 = self.t_presence;
        self.grid_step = JUMP_GRID;
        self.consider_resend();
    }

    pub fn reconfigure(&mut self) {
        let Some(device) = &self.device else {
            return;
        };
        let Some(surface) = &self.surface else {
            return;
        };
        let mut config = match &self.config {
            Some(c) => c.clone(),
            None => return,
        };
        let w = (self.size.0 as f64
            * self.scale_factor
            * self.ssaa as f64
            * FIELD_BACKING_SCALE as f64)
            .round() as u32;
        let h = (self.size.1 as f64
            * self.scale_factor
            * self.ssaa as f64
            * FIELD_BACKING_SCALE as f64)
            .round() as u32;
        config.width = w.max(1);
        config.height = h.max(1);
        surface.configure(device, &config);
        self.config = Some(config);
        self.backing = (w.max(1), h.max(1));
    }

    pub fn vp_data(&self) -> [f32; 36] {
        let (fr, fu, ff) = self.frame();
        let [x, y, z] = self.pos();
        [
            self.backing.0 as f32,
            self.backing.1 as f32,
            self.packed_count as f32,
            self.grid_step as f32,
            fr[0] as f32,
            fr[1] as f32,
            fr[2] as f32,
            (self.v[0] / C) as f32,
            fu[0] as f32,
            fu[1] as f32,
            fu[2] as f32,
            (self.v[1] / C) as f32,
            ff[0] as f32,
            ff[1] as f32,
            ff[2] as f32,
            (self.v[2] / C) as f32,
            self.expose_offset,
            self.last_response_epoch as f32,
            0.0,
            0.0,
            x as f32,
            y as f32,
            z as f32,
            self.t_presence as f32,
            self.force_ref[0],
            self.force_ref[1],
            self.force_ref[2],
            self.force_ref[3],
            self.force_ref[4],
            self.force_ref[5],
            self.force_ref[6],
            self.force_ref[7],
            self.force_ref[8],
            0.0,
            0.0,
            0.0,
        ]
    }

    pub fn relax_force_refs(&mut self) {
        let meds = force_ref_medians(&self.packed_field, &self.packed_meta);
        for (ft, m) in meds.iter().enumerate() {
            let Some(median) = m else {
                continue;
            };
            if self.force_ref[ft] == 0.0 {
                self.force_ref[ft] = *median;
                continue;
            }
            self.force_ref[ft] += (median - self.force_ref[ft]) * REF_RELAX;
        }
    }

    pub fn rebuild_binds(&mut self) {
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(probe_layout) = self.probe_layout.clone() else {
            return;
        };
        let Some(render_layout) = self.render_layout.clone() else {
            return;
        };
        let Some(vp_buf) = self.vp_buf.clone() else {
            return;
        };
        let Some(probe_buf) = self.probe_buf.clone() else {
            return;
        };
        let Some(prep_param_buf) = self.prep_param_buf.clone() else {
            return;
        };
        let Some(color_lut_view) = self.color_lut_view.clone() else {
            return;
        };
        let Some(color_lut_sampler) = self.color_lut_sampler.clone() else {
            return;
        };
        for sel in 0..2 {
            let Some(field_buf) = self.field_bufs[sel].clone() else {
                continue;
            };
            let Some(meta_buf) = self.meta_bufs[sel].clone() else {
                continue;
            };
            self.probe_binds[sel] = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &probe_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: field_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: meta_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: vp_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: probe_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: prep_param_buf.as_entire_binding(),
                    },
                ],
            }));
            self.render_binds[sel] = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &render_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: field_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: meta_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: vp_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: prep_param_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 9,
                        resource: wgpu::BindingResource::TextureView(&color_lut_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 12,
                        resource: wgpu::BindingResource::Sampler(&color_lut_sampler),
                    },
                ],
            }));
        }
    }

    pub fn ensure_capacity(&mut self) {
        let Some(device) = self.device.clone() else {
            return;
        };
        let n = self.packed_count;
        if self.field_cap >= n {
            return;
        }
        let mut c = if self.field_cap > 0 {
            self.field_cap
        } else {
            256
        };
        while c < n {
            c <<= 1;
        }
        let max_buf = device.limits().max_buffer_size;
        while c as u64 * 96 > max_buf {
            c >>= 1;
        }
        if c < 256 {
            c = 256;
        }
        self.field_cap = c;
        for sel in 0..2 {
            self.field_bufs[sel] = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: c as u64 * 48,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
            self.meta_bufs[sel] = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: c as u64 * 64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        let prep_param_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: c as u64 * 32,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.prep_param_buf = Some(prep_param_buf);
        self.rebuild_binds();
    }

    pub fn render(&mut self) {
        let t0 = std::time::Instant::now();
        self.frame_count += 1;
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(queue) = self.queue.clone() else {
            return;
        };
        if self.packed_gen != self.uploaded_gen {
            self.ensure_capacity();
            let sel = self.buf_sel ^ 1;
            if let Some(fb) = &self.field_bufs[sel] {
                queue.write_buffer(fb, 0, &le_bytes_f32(&self.packed_field));
            }
            if let Some(mb) = &self.meta_bufs[sel] {
                queue.write_buffer(mb, 0, &le_bytes_f32(&self.packed_meta));
            }
            self.buf_sel = sel;
            self.uploaded_gen = self.packed_gen;
        }
        let vp = self.vp_data();
        let mut bytes = [0u8; 144];
        for (i, x) in vp.iter().enumerate() {
            bytes[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        let Some(vp_buf) = self.vp_buf.as_ref() else {
            return;
        };
        queue.write_buffer(vp_buf, 0, &bytes);
        if self.hud_dirty {
            if let Some(tex) = self.hud_tex.as_ref() {
                let stride = (self.hud_w * 4 + 255) / 256 * 256;
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: tex,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &self.hud_bitmap,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(stride),
                        rows_per_image: Some(HUD_H),
                    },
                    wgpu::Extent3d {
                        width: self.hud_w,
                        height: HUD_H,
                        depth_or_array_layers: 1,
                    },
                );
            }
            self.hud_dirty = false;
        }
        let mut probe_enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = probe_enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            let sel = self.buf_sel;
            if let (Some(pipe), Some(bind)) =
                (self.probe_pipe.as_ref(), self.probe_binds[sel].as_ref())
            {
                pass.set_pipeline(pipe);
                pass.set_bind_group(0, bind, &[]);
                pass.dispatch_workgroups(1, 1, 1);
            }
        }
        queue.submit(std::iter::once(probe_enc.finish()));
        if self.silent {
            return;
        }
        let Some(surface) = self.surface.as_ref() else {
            return;
        };
        let frame = match surface.get_current_texture() {
            Ok(f) => f,
            Err(_) => return,
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            if !self.field_dark {
                if let (Some(pipe), Some(bind)) = (
                    self.render_pipe.as_ref(),
                    self.render_binds[self.buf_sel].as_ref(),
                ) {
                    pass.set_pipeline(pipe);
                    pass.set_bind_group(0, bind, &[]);
                    pass.draw(0..6, 0..1);
                }
            }
            if !self.hud_dark {
                if let (Some(pipe), Some(bind)) = (self.hud_pipe.as_ref(), self.hud_bind.as_ref()) {
                    pass.set_pipeline(pipe);
                    pass.set_bind_group(0, bind, &[]);
                    pass.draw(0..6, 0..1);
                }
            }
        }
        queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        if ms > self.frame_ms_max {
            self.frame_ms_max = ms;
        }
    }

    pub fn init_gpu(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let mut attrs = WindowAttributes::default()
            .with_title("omegaflow φ")
            .with_active(true)
            .with_fullscreen(Some(Fullscreen::Borderless(None)));
        if std::env::var("OMEGAFLOW_HIDDEN").is_ok() {
            attrs = attrs.with_visible(false);
        }
        let window = match event_loop.create_window(attrs) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("window creation returned: {}", e);
                return;
            }
        };
        if std::env::var("OMEGAFLOW_HIDDEN").is_err() {
            let _ = window.focus_window();
        }
        let (sw, sh) = match window.current_monitor() {
            Some(m) => {
                let ms = m.size();
                (ms.width.max(1), ms.height.max(1))
            }
            None => {
                let inner = window.inner_size();
                (inner.width.max(1), inner.height.max(1))
            }
        };
        self.size = (sw, sh);
        self.scale_factor = 1.0;
        let window = Arc::new(window);
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = match instance.create_surface(window.clone()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("surface creation returned: {}", e);
                return;
            }
        };
        let adapter =
            match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })) {
                Some(a) => a,
                None => {
                    eprintln!("adapter request returned void");
                    return;
                }
            };
        let info = adapter.get_info();
        eprintln!(
            "adapter: {} | {:?} | {:?} | {}",
            info.name, info.backend, info.device_type, info.driver_info
        );
        let (device, queue) = match pollster::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
        ) {
            Ok(dq) => dq,
            Err(e) => {
                eprintln!("device request returned: {}", e);
                return;
            }
        };
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: 1,
            height: 1,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(FIELD_WGSL.into()),
        });
        let probe_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::COMPUTE);
                    e.binding = 0;
                    e
                },
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::COMPUTE);
                    e.binding = 1;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::COMPUTE);
                    e.binding = 3;
                    e
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::COMPUTE);
                    e.binding = 4;
                    e
                },
            ],
        });
        let render_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::VERTEX_FRAGMENT);
                    e.binding = 0;
                    e
                },
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::VERTEX_FRAGMENT);
                    e.binding = 1;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::FRAGMENT);
                    e.binding = 4;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 9,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 12,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });
        let probe_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&probe_layout],
            push_constant_ranges: &[],
        });
        let render_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&render_layout],
            push_constant_ranges: &[],
        });
        let render_pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipe_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        let probe_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&probe_pipe_layout),
            module: &module,
            entry_point: Some("presence_probe"),
            compilation_options: Default::default(),
            cache: None,
        });
        let hud_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let color_lut_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: crate::spectral::COLOR_LUT_LEN as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let mut lut_bytes = Vec::with_capacity(crate::spectral::COLOR_LUT_LEN * 16);
        for e in crate::spectral::color_lut_rgba() {
            for v in e {
                lut_bytes.extend_from_slice(&v.to_le_bytes());
            }
        }
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &color_lut_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &lut_bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some((crate::spectral::COLOR_LUT_LEN * 16) as u32),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: crate::spectral::COLOR_LUT_LEN as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        self.color_lut_view =
            Some(color_lut_tex.create_view(&wgpu::TextureViewDescriptor::default()));
        self.color_lut_sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        }));
        let hud_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 10,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 11,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let hud_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&hud_layout],
            push_constant_ranges: &[],
        });
        let hud_pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&hud_pipe_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("hud_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("hud_fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        self.hud_sampler = Some(hud_sampler);
        self.hud_layout = Some(hud_layout);
        self.hud_pipe = Some(hud_pipe);
        let vp_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 144,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let probe_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 48,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let probe_read = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 48,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(TE_WGSL.into()),
        });
        let te_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::COMPUTE);
                    e.binding = 0;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                {
                    let mut e = storage_entry(false, wgpu::ShaderStages::COMPUTE);
                    e.binding = 2;
                    e
                },
            ],
        });
        let te_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&te_layout],
            push_constant_ranges: &[],
        });
        let te_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&te_pipe_layout),
            module: &te_module,
            entry_point: Some("te_compute"),
            compilation_options: Default::default(),
            cache: None,
        });
        let te_series_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: TE_SERIES_BYTES,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_param_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_out_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let te_read_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &te_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: te_series_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: te_param_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: te_out_buf.as_entire_binding(),
                },
            ],
        });
        self.te_pipe = Some(te_pipe.clone());
        self.te_bind = Some(te_bind);
        self.te_series_buf = Some(te_series_buf);
        self.te_param_buf = Some(te_param_buf);
        self.te_out_buf = Some(te_out_buf);
        self.te_read_buf = Some(te_read_buf);
        self.solar
            .bind_gpu(&device, &queue, te_pipe.clone(), &te_layout);
        self.enso.bind_gpu(&device, &queue, te_pipe, &te_layout);
        self.window = Some(window);
        self.surface = Some(surface);
        self.config = Some(config);
        self.device = Some(device);
        self.queue = Some(queue);
        self.render_pipe = Some(render_pipe);
        self.probe_pipe = Some(probe_pipe);
        self.probe_layout = Some(probe_layout);
        self.render_layout = Some(render_layout);
        self.vp_buf = Some(vp_buf);
        self.probe_buf = Some(probe_buf);
        self.probe_read = Some(probe_read);
        self.reconfigure();
        self.ensure_capacity();
        self.ensure_hud_texture();
    }
}

impl ApplicationHandler for NativeApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.init_gpu(event_loop);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.shutdown.load(Ordering::SeqCst) {
            event_loop.exit();
            return;
        }
        if self.window.is_none() {
            return;
        }
        if !self.focused {
            self.focus_req += 1;
            if self.focus_req % 30 == 0 {
                if let Some(w) = self.window.as_ref() {
                    let _ = w.focus_window();
                }
            }
        }
        let now_i = std::time::Instant::now();
        let raw = match self.last_tick {
            Some(prev) => now_i.duration_since(prev).as_secs_f64() * 1000.0,
            None => Φ,
        };
        self.last_tick = Some(now_i);
        self.stable_tick = self.stable_tick * (1.0 - EMA_FACTOR) + raw * EMA_FACTOR;
        self.expose_offset += (EXPOSE_OFFSET_BASE - self.expose_offset) * OFFSET_RELAX;
        self.consider_state_save();
        if let Some(t) = system_now(&self.time) {
            if self.t_presence == 0.0 {
                self.t_presence = t;
                self.t0 = t;
            }
        }
        if self.keys.contains(&KeyCode::Comma) {
            self.t_thrust_target = -THRUST_STEP;
        } else if self.keys.contains(&KeyCode::Period) {
            self.t_thrust_target = THRUST_STEP;
        } else {
            self.t_thrust_target = 0.0;
        }
        self.t_thrust += (self.t_thrust_target - self.t_thrust)
            * (1.0 - (-raw / self.stable_tick.max(raw)).exp());
        if !self.t_frozen {
            self.t_presence += (1.0 + self.t_thrust) * raw / 1000.0;
        }
        let (fr, fu, ff) = self.frame();
        let thrust_speed = self.grid_step * raw / 1000.0;
        let pan_speed = self.grid_step * if self.shift { 4.0 } else { 1.0 } * raw / 1000.0;
        let mut thrust = [0.0f64; 3];
        let mut pan = [0.0f64; 3];
        if self.keys.contains(&KeyCode::ArrowRight) {
            if self.shift {
                for i in 0..3 {
                    pan[i] += fr[i];
                }
            } else {
                for i in 0..3 {
                    thrust[i] += fr[i];
                }
            }
        }
        if self.keys.contains(&KeyCode::ArrowLeft) {
            if self.shift {
                for i in 0..3 {
                    pan[i] -= fr[i];
                }
            } else {
                for i in 0..3 {
                    thrust[i] -= fr[i];
                }
            }
        }
        if self.keys.contains(&KeyCode::ArrowUp) {
            if self.shift {
                for i in 0..3 {
                    pan[i] += ff[i];
                }
            } else {
                for i in 0..3 {
                    thrust[i] += ff[i];
                }
            }
        }
        if self.keys.contains(&KeyCode::ArrowDown) {
            if self.shift {
                for i in 0..3 {
                    pan[i] -= ff[i];
                }
            } else {
                for i in 0..3 {
                    thrust[i] -= ff[i];
                }
            }
        }
        if self.keys.contains(&KeyCode::PageUp) {
            for i in 0..3 {
                pan[i] += fu[i];
            }
        }
        if self.keys.contains(&KeyCode::PageDown) {
            for i in 0..3 {
                pan[i] -= fu[i];
            }
        }
        if thrust != [0.0, 0.0, 0.0] {
            self.fold();
            for i in 0..3 {
                self.v[i] += thrust[i] * thrust_speed;
            }
            self.consider_resend();
        }
        if pan != [0.0, 0.0, 0.0] {
            for i in 0..3 {
                self.p[i] += pan[i] * pan_speed;
            }
            self.consider_resend();
        }
        if self.v != [0.0, 0.0, 0.0] {
            self.consider_resend();
        }
        #[cfg(feature = "gamepad")]
        {
            let mut gilrs_state = self.gilrs.take();
            if let Some(gilrs) = gilrs_state.as_mut() {
                while let Some(ev) = gilrs.next_event() {
                    if let gilrs::EventType::ButtonPressed(button, _) = ev.event {
                        match button {
                            gilrs::Button::South => {
                                self.fold();
                                self.v = [0.0, 0.0, 0.0];
                            }
                            gilrs::Button::East => self.jump(0),
                            gilrs::Button::North => {
                                self.p = [0.0, 0.0, 0.0];
                                self.v = [0.0, 0.0, 0.0];
                                self.t0 = self.t_presence;
                            }
                            gilrs::Button::West => self.t_frozen = !self.t_frozen,
                            gilrs::Button::Start => {
                                self.shutdown.store(true, Ordering::SeqCst);
                            }
                            _ => {}
                        }
                    }
                }
                if let Some((_, gp)) = gilrs.gamepads().next() {
                    let rx = gp.value(gilrs::Axis::LeftStickX);
                    let ry = gp.value(gilrs::Axis::LeftStickY);
                    let lx = gp.value(gilrs::Axis::RightStickX);
                    let ly = gp.value(gilrs::Axis::RightStickY);
                    let l2 = gp.value(gilrs::Axis::LeftZ);
                    let r2 = gp.value(gilrs::Axis::RightZ);
                    let roll = if gp.is_pressed(gilrs::Button::LeftTrigger) {
                        -1.0
                    } else if gp.is_pressed(gilrs::Button::RightTrigger) {
                        1.0
                    } else {
                        0.0
                    };
                    let (fr, fu, ff) = self.frame();
                    let rot = Φ * raw / 1000.0;
                    if rx.abs() > 0.15 {
                        self.q = q_norm(q_mul(q_axis_angle(fu, rx as f64 * rot), self.q));
                    }
                    if ry.abs() > 0.15 {
                        self.q = q_norm(q_mul(q_axis_angle(fr, ry as f64 * rot), self.q));
                    }
                    if roll != 0.0 {
                        self.q = q_norm(q_mul(q_axis_angle(ff, roll * rot), self.q));
                    }
                    let pan_sp = self.grid_step * raw / 1000.0;
                    if lx.abs() > 0.15 || ly.abs() > 0.15 {
                        self.p[0] += fr[0] * lx as f64 * pan_sp + fu[0] * ly as f64 * pan_sp;
                        self.p[1] += fr[1] * lx as f64 * pan_sp + fu[1] * ly as f64 * pan_sp;
                        self.p[2] += fr[2] * lx as f64 * pan_sp + fu[2] * ly as f64 * pan_sp;
                        self.consider_resend();
                    }
                    if l2 > 0.2 {
                        self.grid_step *= 1.0 + l2 as f64 * raw / 1000.0;
                        self.consider_resend();
                    }
                    if r2 > 0.2 {
                        self.grid_step /= 1.0 + r2 as f64 * raw / 1000.0;
                        self.consider_resend();
                    }
                    let thrust_sp = self.grid_step * raw / 1000.0;
                    let mut thrust = [0.0f64; 3];
                    if gp.is_pressed(gilrs::Button::DPadUp) {
                        thrust[0] += ff[0];
                        thrust[1] += ff[1];
                        thrust[2] += ff[2];
                    }
                    if gp.is_pressed(gilrs::Button::DPadDown) {
                        thrust[0] -= ff[0];
                        thrust[1] -= ff[1];
                        thrust[2] -= ff[2];
                    }
                    if gp.is_pressed(gilrs::Button::DPadRight) {
                        thrust[0] += fr[0];
                        thrust[1] += fr[1];
                        thrust[2] += fr[2];
                    }
                    if gp.is_pressed(gilrs::Button::DPadLeft) {
                        thrust[0] -= fr[0];
                        thrust[1] -= fr[1];
                        thrust[2] -= fr[2];
                    }
                    if thrust != [0.0, 0.0, 0.0] {
                        self.fold();
                        self.v[0] += thrust[0] * thrust_sp;
                        self.v[1] += thrust[1] * thrust_sp;
                        self.v[2] += thrust[2] * thrust_sp;
                        self.consider_resend();
                    }
                }
            }
            self.gilrs = gilrs_state;
        }
        if let Ok(field) = self.rx.try_recv() {
            self.latest_field = Some(field);
            self.sense();
        }
        while let Ok((packed, t, generation)) = self.res_rx.try_recv() {
            self.packed_count = packed.count;
            self.packed_field = packed.field;
            self.packed_meta = packed.meta;
            self.packed_gen = generation;
            self.last_response_epoch = t;
            self.relax_force_refs();
        }
        self.consider_resend();
        self.sensors.frame_interval = (raw / 1000.0).max(0.001);
        if self
            .last_hud
            .map_or(true, |i| i.elapsed().as_secs_f64() >= 1.0)
        {
            self.last_hud = Some(now_i);
            self.sensors.flush();
            if let (Some(device), Some(queue), Some(probe_buf), Some(probe_read)) = (
                self.device.clone(),
                self.queue.clone(),
                self.probe_buf.clone(),
                self.probe_read.clone(),
            ) {
                let mut enc =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                enc.copy_buffer_to_buffer(&probe_buf, 0, &probe_read, 0, 48);
                queue.submit(std::iter::once(enc.finish()));
                let mapped = Arc::new(AtomicBool::new(false));
                let m2 = mapped.clone();
                let slice = probe_read.slice(..);
                slice.map_async(wgpu::MapMode::Read, move |r| {
                    m2.store(r.is_ok(), Ordering::SeqCst);
                });
                let deadline = std::time::Instant::now() + std::time::Duration::from_millis(5);
                while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
                    device.poll(wgpu::Maintain::Poll);
                }
                if mapped.load(Ordering::SeqCst) {
                    let data = slice.get_mapped_range();
                    let mut v = [0f32; 12];
                    for k in 0..12 {
                        let mut b = [0u8; 4];
                        b.copy_from_slice(&data[k * 4..k * 4 + 4]);
                        v[k] = f32::from_le_bytes(b);
                    }
                    drop(data);
                    self.probe_omega.copy_from_slice(&v[0..9]);
                    self.probe_flow.copy_from_slice(&v[9..12]);
                    self.probe_ring[self.ring_head] = v;
                    self.ring_head = (self.ring_head + 1) % 256;
                    if self.ring_filled < 256 {
                        self.ring_filled += 1;
                    }
                    self.ring_gen += 1;
                }
                probe_read.unmap();
            }
            let mut te_opt: Option<(f64, f64)> = None;
            if self.ring_filled >= 32 {
                let m = self.ring_filled;
                let mut xs = vec![0f32; m];
                let mut ys = vec![0f32; m];
                for i in 0..m {
                    let idx = (self.ring_head + 256 - m + i) % 256;
                    let v = self.probe_ring[idx];
                    xs[i] = v[0] + v[1] + v[2] + v[3] + v[4] + v[5] + v[6] + v[7] + v[8];
                    ys[i] = (v[9] * v[9] + v[10] * v[10] + v[11] * v[11]).sqrt();
                }
                if let Some(v) = self.te_probe(&xs, &ys, m) {
                    self.hud_topology = Some((v.tau_x, v.tau_y, v.pe_x, v.pe_y));
                    if self.pe_gate(v.pe_y) {
                        te_opt = Some((v.te, v.threshold));
                    }
                }
            }
            self.solar.tick(self.ring_gen);
            self.enso.tick(self.ring_gen);
            if let Some((in_te, threshold)) = te_opt {
                let delta_te = in_te - self.prev_in_te;
                self.prev_in_te = in_te;
                self.ticks_since_turn += 1;
                if self.direction > 0 && delta_te < -threshold {
                    self.natural_latency_ticks = self.ticks_since_turn.max(1);
                    self.ticks_since_turn = 0;
                    self.direction = -1;
                }
                if self.direction < 0
                    && (delta_te > threshold || self.field_permeability <= PERM_GROUND)
                {
                    self.natural_latency_ticks = self.ticks_since_turn.max(1);
                    self.ticks_since_turn = 0;
                    self.direction = 1;
                }
                let target =
                    (in_te.max(0.0) / (in_te.max(0.0) + threshold + PERM_GROUND as f64)) as f32;
                let alpha = 1.0 - (-1.0 / self.natural_latency_ticks.max(1) as f32).exp();
                self.field_permeability += (target - self.field_permeability) * alpha;
                self.field_permeability = self.field_permeability.clamp(PERM_GROUND, 1.0);
            } else {
                let omega_sum: f32 = self.probe_omega.iter().sum();
                let delta = omega_sum - self.prev_omega_sum;
                if self.prev_delta != 0.0 && delta * self.prev_delta < 0.0 {
                    self.natural_latency_ticks = self.ticks_since_turn.max(1);
                    self.ticks_since_turn = 0;
                }
                self.ticks_since_turn += 1;
                self.prev_delta = delta;
                self.prev_omega_sum = omega_sum;
                let g = omega_sum.abs();
                let v_c = delta.abs();
                let target = (v_c / (g + PERM_GROUND)).tanh();
                let alpha = 1.0 - (-1.0 / self.natural_latency_ticks.max(1) as f32).exp();
                self.field_permeability += (target - self.field_permeability) * alpha;
                self.field_permeability = self.field_permeability.clamp(PERM_GROUND, 1.0);
            }
            let frame = PresenceFrame {
                omega: self.probe_omega,
            };
            if !self.silent {
                let _ = self.acoustic_tx.send(frame);
                let _ = self.seismic_tx.send(frame);
            }
            let [x, y, z] = self.pos();
            let fps = self.frame_count as f64;
            self.frame_count = 0;
            let ms_max = self.frame_ms_max;
            self.frame_ms_max = 0.0;
            let avg_ms = 1000.0 / fps.max(1.0);
            self.frame_ms_ema = if self.frame_ms_ema == 0.0 {
                avg_ms
            } else {
                self.frame_ms_ema * (1.0 - BUDGET_RELAX) + avg_ms * BUDGET_RELAX
            };
            let rec = if self.consent.load(Ordering::SeqCst) {
                "on"
            } else {
                "silent"
            };
            let mut force_tokens = String::new();
            for k in 0..9 {
                if k > 0 {
                    force_tokens.push(' ');
                }
                let v = self.probe_omega[k];
                if v != 0.0 && v.abs() < 0.01 {
                    force_tokens.push_str(&format!(
                        "{}[{}]:{:+.1e}",
                        FORCE_NAME[k], FORCE_SI_UNIT[k], v
                    ));
                } else {
                    force_tokens.push_str(&format!(
                        "{}[{}]:{:+.2}",
                        FORCE_NAME[k], FORCE_SI_UNIT[k], v
                    ));
                }
            }
            let te_word = if self.ring_filled >= 32 {
                self.te_named.clone()
            } else {
                "ring below gate".to_string()
            };
            if !self.hud_dark {
                self.hud_raster(&force_tokens, te_opt, &te_word);
            }
            let (te_s, thr_s) = match te_opt {
                Some((t, h)) => (format!("{:.3}", t), format!("{:.3}", h)),
                None => ("-".to_string(), "-".to_string()),
            };
            let (tau_s, pe_s) = match self.hud_topology {
                Some((tx, ty, px, py)) => {
                    let pe = match (px, py) {
                        (Some(a), Some(b)) => format!("{:.2}:{:.2}", a, b),
                        _ => "-".to_string(),
                    };
                    (format!("{}:{}", tx, ty), pe)
                }
                None => ("-".to_string(), "-".to_string()),
            };
            eprintln!(
                "φ window: t {:.2} | rec {} | gen {} | flow {:+.2} {:+.2} {:+.2} | {} | fps {:.0} | ssaa {:.2} | grid 2^{} | x {:.3e} y {:.3e} z {:.3e} | {} recs | b {}x{} | maxms {:.0} | ema {:.1} | perm {:.2} | off {:.2} | field {} | refs {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} | te {} thr {} | tau {} | pe {} | state {} | focus {} | keys {}",
                self.t_presence,
                rec,
                self.ring_gen,
                self.probe_flow[0],
                self.probe_flow[1],
                self.probe_flow[2],
                force_tokens,
                fps,
                self.ssaa,
                self.grid_step.log2().round() as i64,
                x,
                y,
                z,
                self.packed_count,
                self.backing.0,
                self.backing.1,
                ms_max,
                self.frame_ms_ema,
                self.field_permeability,
                self.expose_offset,
                self.field_dark,
                self.force_ref[0],
                self.force_ref[1],
                self.force_ref[2],
                self.force_ref[3],
                self.force_ref[4],
                self.force_ref[5],
                self.force_ref[6],
                self.force_ref[7],
                self.force_ref[8],
                te_s,
                thr_s,
                tau_s,
                pe_s,
                te_word,
                self.focused,
                self.keys_seen,
            );
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(16),
        ));
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.shutdown.store(true, Ordering::SeqCst);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.size = (size.width.max(1), size.height.max(1));
                self.reconfigure();
                self.ensure_hud_texture();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(m) = self.window.as_ref().and_then(|w| w.current_monitor()) {
                    let ms = m.size();
                    self.size = (ms.width.max(1), ms.height.max(1));
                }
                self.scale_factor = 1.0;
                self.reconfigure();
                self.ensure_hud_texture();
            }
            WindowEvent::ModifiersChanged(m) => {
                self.shift = m.state().shift_key();
            }
            WindowEvent::Focused(f) => {
                self.focused = f;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.logical_key == Key::Named(NamedKey::Escape)
                    || event.physical_key == PhysicalKey::Code(KeyCode::Escape)
                {
                    if event.state == ElementState::Pressed {
                        self.shutdown.store(true, Ordering::SeqCst);
                        event_loop.exit();
                    }
                    return;
                }
                if let PhysicalKey::Code(code) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => {
                            self.keys_seen += 1;
                            if !self.keys.contains(&code) {
                                self.keys.insert(code);
                                self.key_action(code);
                                self.sensors
                                    .record_sample(&format!("event.key.{:?}", code), 1.0);
                            }
                        }
                        ElementState::Released => {
                            self.keys.remove(&code);
                            self.sensors
                                .record_sample(&format!("event.key.{:?}", code), 0.0);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos = position.to_logical::<f64>(self.scale_factor);
                let (px, py) = (pos.x, pos.y);
                if let Some((lx, ly)) = self.cursor {
                    let dx = px - lx;
                    let dy = py - ly;
                    let (fr, fu, ff) = self.frame();
                    let gaze_px = 2.0 / self.backing.0.max(1) as f64;
                    match self.drag_button {
                        Some(MouseButton::Left) => {
                            if dx != 0.0 {
                                self.q = q_norm(q_mul(q_axis_angle(fu, dx * gaze_px), self.q));
                            }
                            if dy != 0.0 {
                                self.q = q_norm(q_mul(q_axis_angle(fr, dy * gaze_px), self.q));
                            }
                        }
                        Some(MouseButton::Middle) => {
                            if dx != 0.0 {
                                self.q = q_norm(q_mul(q_axis_angle(ff, dx * gaze_px), self.q));
                            }
                        }
                        Some(MouseButton::Right) => {
                            self.p[0] -= fr[0] * dx * self.grid_step - fu[0] * dy * self.grid_step;
                            self.p[1] -= fr[1] * dx * self.grid_step - fu[1] * dy * self.grid_step;
                            self.p[2] -= fr[2] * dx * self.grid_step - fu[2] * dy * self.grid_step;
                            self.consider_resend();
                        }
                        _ => {}
                    }
                }
                self.cursor = Some((px, py));
                self.sensors.record_sample("event.mousemove.x", px);
                self.sensors.record_sample("event.mousemove.y", py);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.drag_button = match state {
                    ElementState::Pressed => Some(button),
                    ElementState::Released => None,
                };
                if let ElementState::Pressed = state {
                    if let Some((px, py)) = self.cursor {
                        self.sensors.record_sample("event.click.x", px);
                        self.sensors.record_sample("event.click.y", py);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y as f64 * 32.0,
                    MouseScrollDelta::PixelDelta(p) => p.y,
                };
                self.sensors.record_sample("event.wheel.deltaY", dy);
                if dy != 0.0 {
                    self.grid_step /= 2f64.powf(dy / 128.0);
                    self.consider_resend();
                }
            }
            WindowEvent::PinchGesture { delta, .. } => {
                if delta != 0.0 {
                    self.grid_step /= 2f64.powf(delta / 512.0);
                    self.consider_resend();
                }
            }
            WindowEvent::Touch(touch) => {
                let pos = touch.location.to_logical::<f64>(self.scale_factor);
                let p = (pos.x, pos.y);
                let id = touch.id;
                match touch.phase {
                    TouchPhase::Started => {
                        if self.touches.is_empty() {
                            self.press = Some((std::time::Instant::now(), id, p));
                        }
                        self.touches.insert(id, p);
                    }
                    TouchPhase::Moved => {
                        if let Some(prev) = self.touches.insert(id, p) {
                            let dx = p.0 - prev.0;
                            let dy = p.1 - prev.1;
                            let (fr, fu, _ff) = self.frame();
                            if self.touches.len() == 1 {
                                self.p[0] -=
                                    fr[0] * dx * self.grid_step - fu[0] * dy * self.grid_step;
                                self.p[1] -=
                                    fr[1] * dx * self.grid_step - fu[1] * dy * self.grid_step;
                                self.p[2] -=
                                    fr[2] * dx * self.grid_step - fu[2] * dy * self.grid_step;
                                self.consider_resend();
                            } else if self.touches.len() == 2 {
                                if let Some(o) = self
                                    .touches
                                    .iter()
                                    .find(|(k, _)| **k != id)
                                    .map(|(_, v)| *v)
                                {
                                    let (cx0, cy0) = ((prev.0 + o.0) / 2.0, (prev.1 + o.1) / 2.0);
                                    let (cx1, cy1) = ((p.0 + o.0) / 2.0, (p.1 + o.1) / 2.0);
                                    let d0 = ((prev.0 - o.0).powi(2) + (prev.1 - o.1).powi(2))
                                        .sqrt()
                                        .max(1.0);
                                    let d1 =
                                        ((p.0 - o.0).powi(2) + (p.1 - o.1).powi(2)).sqrt().max(1.0);
                                    let mdx = cx1 - cx0;
                                    let mdy = cy1 - cy0;
                                    self.p[0] -=
                                        fr[0] * mdx * self.grid_step - fu[0] * mdy * self.grid_step;
                                    self.p[1] -=
                                        fr[1] * mdx * self.grid_step - fu[1] * mdy * self.grid_step;
                                    self.p[2] -=
                                        fr[2] * mdx * self.grid_step - fu[2] * mdy * self.grid_step;
                                    self.grid_step /= d1 / d0;
                                    self.consider_resend();
                                }
                            }
                            if let Some((t0, pid, sp)) = self.press {
                                if pid == id && (p.0 - sp.0).abs() + (p.1 - sp.1).abs() > 12.0 {
                                    self.press = None;
                                } else if t0.elapsed() > std::time::Duration::from_millis(600) {
                                    self.t_frozen = !self.t_frozen;
                                    self.press = None;
                                }
                            }
                        }
                    }
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        self.touches.remove(&id);
                        if let Some((t0, pid, sp)) = self.press {
                            if pid == id {
                                let elapsed = t0.elapsed();
                                let still = (p.0 - sp.0).abs() + (p.1 - sp.1).abs() < 12.0;
                                if still && elapsed < std::time::Duration::from_millis(300) {
                                    let now = std::time::Instant::now();
                                    let double = self
                                        .tap
                                        .map(|(last, lp)| {
                                            now.duration_since(last)
                                                < std::time::Duration::from_millis(300)
                                                && (p.0 - lp.0).abs() + (p.1 - lp.1).abs() < 24.0
                                        })
                                        .unwrap_or(false);
                                    if double {
                                        self.p = [0.0, 0.0, 0.0];
                                        self.v = [0.0, 0.0, 0.0];
                                        self.t0 = self.t_presence;
                                        self.consider_resend();
                                        self.tap = None;
                                    } else {
                                        self.tap = Some((now, p));
                                    }
                                } else if still && elapsed >= std::time::Duration::from_millis(600)
                                {
                                    self.t_frozen = !self.t_frozen;
                                }
                            }
                            self.press = None;
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => self.render(),
            _ => {}
        }
    }
}

pub fn run_window(
    rx: mpsc::Receiver<Arc<Buffer>>,
    presence_tx: mpsc::Sender<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    sensor_tx: mpsc::Sender<Vec<(String, f64, f64)>>,
    req_tx: mpsc::SyncSender<SenseReq>,
    res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
    body_names: Arc<Vec<String>>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
    shutdown: Arc<AtomicBool>,
    consent: Arc<AtomicBool>,
    acoustic_tx: mpsc::Sender<PresenceFrame>,
    seismic_tx: mpsc::Sender<PresenceFrame>,
    solar_rx: mpsc::Receiver<SolarCell>,
    enso_rx: mpsc::Receiver<EnsoCell>,
) {
    let mut builder = EventLoopBuilder::<()>::default();
    #[cfg(target_os = "linux")]
    EventLoopBuilderExtX11::with_any_thread(&mut builder, true);
    #[cfg(target_os = "linux")]
    EventLoopBuilderExtWayland::with_any_thread(&mut builder, true);
    let event_loop = match builder.build() {
        Ok(el) => el,
        Err(_) => return,
    };
    event_loop.set_control_flow(ControlFlow::Poll);
    let presence_init: Option<[f64; 4]> = std::env::args().skip(1).find_map(|a| {
        let rest = a.strip_prefix("#x,")?;
        let parts: Vec<&str> = rest.split(',').collect();
        if parts.len() < 4 {
            return None;
        }
        Some([
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
            parts[2].parse().ok()?,
            parts[3].parse().ok()?,
        ])
    });
    let mut app = NativeApp::new(
        rx,
        req_tx,
        res_rx,
        presence_tx,
        sensor_tx,
        body_names,
        time,
        shutdown,
        consent,
        acoustic_tx,
        seismic_tx,
        solar_rx,
        enso_rx,
    );
    if let Some([x, y, z, t]) = presence_init {
        app.p = [x, y, z];
        app.v = [0.0, 0.0, 0.0];
        app.t0 = t;
        app.t_presence = t;
    }
    let _ = event_loop.run_app(&mut app);
}
