use super::*;

pub const Φ: f64 = 1.618033988749895;

pub const C: f64 = 299792458.0;

pub const GRID_INIT: f64 = 2147483648.0;

pub const JUMP_GRID: f64 = 268435456.0;

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

pub const EXPOSE_OFFSET_BASE: f32 = 4.0;

pub const OFFSET_RELAX: f32 = 0.03125;

pub const REF_RELAX: f32 = 0.0625;

pub const LOOP_TICK_MS: u64 = 16;

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

#[derive(Clone, Copy)]
pub struct DiodeState {
    pub force_ref: [f32; 9],
    pub expose_offset: f32,
}

#[derive(Clone, Copy)]
pub struct PresenceState {
    pub p: [f64; 3],
    pub v: [f64; 3],
    pub grid_step: f64,
    pub range: f64,
    pub t_thrust: f64,
}

impl PresenceState {
    pub fn rest() -> Self {
        PresenceState {
            p: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
            grid_step: GRID_INIT,
            range: 1280.0 * GRID_INIT * 2.0,
            t_thrust: 0.0,
        }
    }
}

pub struct OmegaLoop {
    pub rx: mpsc::Receiver<Arc<Buffer>>,
    pub req_tx: mpsc::SyncSender<SenseReq>,
    pub res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
    pub time: Arc<Mutex<Option<LeapSeconds>>>,
    pub shutdown: Arc<AtomicBool>,
    pub consent: Arc<AtomicBool>,
    pub acoustic_tx: mpsc::Sender<PresenceFrame>,
    pub seismic_tx: mpsc::Sender<PresenceFrame>,
    pub silent: bool,
    pub presence: Arc<RwLock<PresenceState>>,
    pub diode: Arc<RwLock<DiodeState>>,

    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub probe_pipe: Option<wgpu::ComputePipeline>,
    pub probe_layout: Option<wgpu::BindGroupLayout>,
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
    pub field_cap: u32,
    pub buf_sel: usize,
    pub packed_gen: u64,
    pub uploaded_gen: u64,

    pub latest_field: Option<Arc<Buffer>>,
    pub packed_field: Vec<f32>,
    pub packed_meta: Vec<f32>,
    pub packed_count: u32,
    pub last_response_epoch: f64,

    pub p: [f64; 3],
    pub v: [f64; 3],
    pub t0: f64,
    pub t_presence: f64,
    pub t_thrust: f64,
    pub last_tick: Option<std::time::Instant>,
    pub q: [f64; 4],
    pub grid_step: f64,
    pub range: f64,
    pub expose_offset: f32,
    pub force_ref: [f32; 9],
    pub probe_omega: [f32; 9],
    pub probe_flow: [f32; 3],
    pub probe_ring: [[f32; 12]; 256],
    pub ring_head: usize,
    pub ring_filled: usize,
    pub ring_gen: u64,
    pub pe_ring: Vec<f64>,
    pub solar: SolarMachine,
    pub matrix: MatrixMachine,
    pub te_topology: Option<(usize, usize, Option<f64>, Option<f64>)>,
    pub field_permeability: f32,
    pub prev_omega_sum: f32,
    pub prev_delta: f32,
    pub prev_in_te: f64,
    pub direction: i32,
    pub ticks_since_turn: u64,
    pub natural_latency_ticks: u64,
    pub last_hud: Option<std::time::Instant>,
}

impl OmegaLoop {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rx: mpsc::Receiver<Arc<Buffer>>,
        req_tx: mpsc::SyncSender<SenseReq>,
        res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
        time: Arc<Mutex<Option<LeapSeconds>>>,
        shutdown: Arc<AtomicBool>,
        consent: Arc<AtomicBool>,
        acoustic_tx: mpsc::Sender<PresenceFrame>,
        seismic_tx: mpsc::Sender<PresenceFrame>,
        solar_rx: mpsc::Receiver<SolarCell>,
        machine_rx: mpsc::Receiver<(
            crate::archivar::Frame,
            Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
        )>,
        presence: Arc<RwLock<PresenceState>>,
        diode: Arc<RwLock<DiodeState>>,
    ) -> Self {
        let rest = PresenceState::rest();
        OmegaLoop {
            rx,
            req_tx,
            res_rx,
            time,
            shutdown,
            consent,
            acoustic_tx,
            seismic_tx,
            silent: std::env::var("OMEGAFLOW_HIDDEN").is_ok(),
            presence,
            diode,
            device: None,
            queue: None,
            probe_pipe: None,
            probe_layout: None,
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
            latest_field: None,
            packed_field: Vec::new(),
            packed_meta: Vec::new(),
            packed_count: 0,
            last_response_epoch: 0.0,
            p: rest.p,
            v: rest.v,
            t0: 0.0,
            t_presence: 0.0,
            t_thrust: 0.0,
            last_tick: None,
            q: [1.0, 0.0, 0.0, 0.0],
            grid_step: rest.grid_step,
            range: rest.range,
            expose_offset: EXPOSE_OFFSET_BASE,
            force_ref: [0.0; 9],
            probe_omega: [0.0; 9],
            probe_flow: [0.0; 3],
            probe_ring: [[0.0; 12]; 256],
            ring_head: 0,
            ring_filled: 0,
            ring_gen: 0,
            pe_ring: Vec::with_capacity(16),
            solar: SolarMachine::new(solar_rx),
            matrix: MatrixMachine::new(machine_rx),
            te_topology: None,
            field_permeability: 0.0,
            prev_omega_sum: 0.0,
            prev_delta: 0.0,
            prev_in_te: 0.0,
            direction: 1,
            ticks_since_turn: 0,
            natural_latency_ticks: 1,
            last_hud: None,
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

    pub fn read_presence(&mut self) {
        if let Ok(pres) = self.presence.read() {
            self.p = pres.p;
            self.v = pres.v;
            self.grid_step = pres.grid_step;
            self.range = pres.range;
            self.t_thrust = pres.t_thrust;
        }
        if self.t_presence == 0.0 {
            if let Some(t) = system_now(&self.time) {
                self.t_presence = t;
                self.t0 = t;
            }
        }
    }

    pub fn publish_diode(&self) {
        if let Ok(mut d) = self.diode.write() {
            d.force_ref = self.force_ref;
            d.expose_offset = self.expose_offset;
        }
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

    pub fn sense(&mut self) {
        let Some(field) = self.latest_field.clone() else {
            return;
        };
        let center = self.pos();
        let t = self.t_presence;
        let pad = self.range;
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

    pub fn vp_data(&self) -> [f32; 24] {
        let (fr, fu, ff) = self.frame();
        let [x, y, z] = self.pos();
        [
            0.0,
            0.0,
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
        ]
    }

    pub fn rebuild_binds(&mut self) {
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(probe_layout) = self.probe_layout.clone() else {
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

    pub fn probe(&mut self) {
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
        let mut bytes = [0u8; 96];
        for (i, x) in vp.iter().enumerate() {
            bytes[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        let Some(vp_buf) = self.vp_buf.as_ref() else {
            return;
        };
        queue.write_buffer(vp_buf, 0, &bytes);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            let sel = self.buf_sel;
            if let (Some(pipe), Some(bind)) =
                (self.probe_pipe.as_ref(), self.probe_binds[sel].as_ref())
            {
                pass.set_pipeline(pipe);
                pass.set_bind_group(0, bind, &[]);
                pass.dispatch_workgroups(1, 1, 1);
            }
        }
        queue.submit(std::iter::once(enc.finish()));
    }

    pub fn probe_readback(&mut self) {
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(queue) = self.queue.clone() else {
            return;
        };
        let Some(probe_buf) = self.probe_buf.clone() else {
            return;
        };
        let Some(probe_read) = self.probe_read.clone() else {
            return;
        };
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
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

    pub fn init_gpu(&mut self) {
        if self.device.is_some() {
            return;
        }
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter =
            match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                compatible_surface: None,
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
        let probe_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&probe_layout],
            push_constant_ranges: &[],
        });
        let probe_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&probe_pipe_layout),
            module: &module,
            entry_point: Some("presence_probe"),
            compilation_options: Default::default(),
            cache: None,
        });
        let vp_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 96,
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
        self.probe_pipe = Some(probe_pipe);
        self.probe_layout = Some(probe_layout);
        self.vp_buf = Some(vp_buf);
        self.probe_buf = Some(probe_buf);
        self.probe_read = Some(probe_read);
        self.te_pipe = Some(te_pipe.clone());
        self.te_bind = Some(te_bind);
        self.te_series_buf = Some(te_series_buf);
        self.te_param_buf = Some(te_param_buf);
        self.te_out_buf = Some(te_out_buf);
        self.te_read_buf = Some(te_read_buf);
        self.solar
            .bind_gpu(&device, &queue, te_pipe.clone(), &te_layout);
        self.matrix.load_state();
        self.matrix
            .bind_gpu(&device, &queue, te_pipe.clone(), &te_layout);
        self.device = Some(device);
        self.queue = Some(queue);
        self.ensure_capacity();
    }

    pub fn tick(&mut self) {
        let now_i = std::time::Instant::now();
        let raw = match self.last_tick {
            Some(prev) => now_i.duration_since(prev).as_secs_f64() * 1000.0,
            None => Φ,
        };
        self.last_tick = Some(now_i);
        self.expose_offset += (EXPOSE_OFFSET_BASE - self.expose_offset) * OFFSET_RELAX;
        self.read_presence();
        self.t_presence += (1.0 + self.t_thrust) * raw / 1000.0;
        if let Ok(field) = self.rx.try_recv() {
            self.latest_field = Some(field);
        }
        while let Ok((packed, t, generation)) = self.res_rx.try_recv() {
            self.packed_count = packed.count;
            self.packed_field = packed.field;
            self.packed_meta = packed.meta;
            self.packed_gen = generation;
            self.last_response_epoch = t;
            self.relax_force_refs();
            self.publish_diode();
        }
        self.sense();
        self.probe();
        if self
            .last_hud
            .map_or(true, |i| i.elapsed().as_secs_f64() >= 1.0)
        {
            self.last_hud = Some(std::time::Instant::now());
            self.probe_readback();
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
                    self.te_topology = Some((v.tau_x, v.tau_y, v.pe_x, v.pe_y));
                    if self.pe_gate(v.pe_y) {
                        te_opt = Some((v.te, v.threshold));
                    }
                }
            }
            self.solar.tick(self.ring_gen);
            self.matrix.tick(
                self.latest_field.clone(),
                self.pos(),
                self.t_presence,
                self.ring_gen,
            );
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
            let (te_s, thr_s) = match te_opt {
                Some((t, h)) => (format!("{:.3}", t), format!("{:.3}", h)),
                None => ("-".to_string(), "-".to_string()),
            };
            let (tau_s, pe_s) = match self.te_topology {
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
                "φ window: t {:.2} | rec {} | gen {} | flow {:+.2} {:+.2} {:+.2} | {} | perm {:.2} | off {:.2} | refs {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} {:.2e} | te {} thr {} | tau {} | pe {} | state {}",
                self.t_presence,
                rec,
                self.ring_gen,
                self.probe_flow[0],
                self.probe_flow[1],
                self.probe_flow[2],
                force_tokens,
                self.field_permeability,
                self.expose_offset,
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
            );
        }
    }
}

pub fn run_loop(
    rx: mpsc::Receiver<Arc<Buffer>>,
    req_tx: mpsc::SyncSender<SenseReq>,
    res_rx: mpsc::Receiver<(PackedWindow, f64, u64)>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
    shutdown: Arc<AtomicBool>,
    consent: Arc<AtomicBool>,
    acoustic_tx: mpsc::Sender<PresenceFrame>,
    seismic_tx: mpsc::Sender<PresenceFrame>,
    solar_rx: mpsc::Receiver<SolarCell>,
    machine_rx: mpsc::Receiver<(
        crate::archivar::Frame,
        Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
    )>,
    presence: Arc<RwLock<PresenceState>>,
    diode: Arc<RwLock<DiodeState>>,
) {
    let mut loop_ = OmegaLoop::new(
        rx,
        req_tx,
        res_rx,
        time,
        shutdown.clone(),
        consent,
        acoustic_tx,
        seismic_tx,
        solar_rx,
        machine_rx,
        presence,
        diode,
    );
    loop_.init_gpu();
    while !shutdown.load(Ordering::SeqCst) {
        let t0 = std::time::Instant::now();
        loop_.tick();
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        if ms < LOOP_TICK_MS as f64 {
            thread::sleep(std::time::Duration::from_millis(LOOP_TICK_MS - ms as u64));
        }
    }
}

pub struct LoopRadiator {
    pub tx: mpsc::SyncSender<Arc<Buffer>>,
    pub shutdown: Arc<AtomicBool>,
    pub _thread: Option<thread::JoinHandle<()>>,
}

impl LoopRadiator {
    pub fn new(
        time: Arc<Mutex<Option<LeapSeconds>>>,
        consent: Arc<AtomicBool>,
        acoustic_tx: mpsc::Sender<PresenceFrame>,
        seismic_tx: mpsc::Sender<PresenceFrame>,
        solar_rx: mpsc::Receiver<SolarCell>,
        machine_rx: mpsc::Receiver<(
            crate::archivar::Frame,
            Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
        )>,
        presence: Arc<RwLock<PresenceState>>,
        diode: Arc<RwLock<DiodeState>>,
    ) -> Self {
        let (tx, rx) = mpsc::sync_channel::<Arc<Buffer>>(2);
        let (req_tx, req_rx) = mpsc::sync_channel::<SenseReq>(1);
        let (res_tx, res_rx) = mpsc::sync_channel::<(PackedWindow, f64, u64)>(2);
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();
        thread::spawn(move || {
            let mut generation: u64 = 0;
            let mut last_bytes: Vec<u8> = Vec::new();
            loop {
                let Ok(mut req) = req_rx.recv() else {
                    break;
                };
                while let Ok(newer) = req_rx.try_recv() {
                    req = newer;
                }
                let SenseReq {
                    field,
                    center,
                    t,
                    pad,
                    cache_interval,
                    forward,
                    expose_offset,
                    force_ref,
                    softening,
                    ..
                } = req;
                let mut records: Vec<Record> = Vec::new();
                let eph = field.eph.clone();
                let scale2 = softening * softening;
                let mut floor = [0.0f64; 9];
                for ft in 0..9 {
                    let r = force_ref[ft] as f64;
                    if r.is_finite() && r > 0.0 && scale2 > 0.0 {
                        floor[ft] = r * (0.5f64).powi(expose_offset as i32) / scale2;
                    }
                }
                sense_membrane(
                    &field,
                    center,
                    t,
                    pad,
                    cache_interval,
                    &floor,
                    softening,
                    forward,
                    &mut records,
                    &eph,
                );
                if let Some(cset) = &field.curves {
                    emit_curves(cset, center, t, pad, &mut records);
                }
                let packed = pack_window(&records, center);
                let mut key = Vec::with_capacity(packed.field.len() * 4 + packed.meta.len() * 4);
                for x in &packed.field {
                    key.extend_from_slice(&x.to_le_bytes());
                }
                for x in &packed.meta {
                    key.extend_from_slice(&x.to_le_bytes());
                }
                if key == last_bytes {
                    continue;
                }
                last_bytes = key;
                generation = generation.wrapping_add(1);
                if res_tx.send((packed, t, generation)).is_err() {
                    break;
                }
            }
        });
        let handle = thread::spawn(move || {
            run_loop(
                rx,
                req_tx,
                res_rx,
                time,
                shutdown_clone,
                consent,
                acoustic_tx,
                seismic_tx,
                solar_rx,
                machine_rx,
                presence,
                diode,
            );
        });
        Self {
            tx,
            shutdown,
            _thread: Some(handle),
        }
    }
}

impl Radiator for LoopRadiator {
    fn accept(&mut self, field: Arc<Buffer>) {
        if let Err(mpsc::TrySendError::Disconnected(_)) = self.tx.try_send(field) {
            eprintln!("omega loop channel closed — field buffer dropped");
        }
    }
}

impl LoopRadiator {
    pub fn shutdown_flag(&self) -> Arc<AtomicBool> {
        self.shutdown.clone()
    }
}

impl Drop for LoopRadiator {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}
