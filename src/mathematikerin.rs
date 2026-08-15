use super::{body_barycenter_position, sense_buffer, system_now, Buffer, LeapSeconds, Radiator};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoopBuilder};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::wayland::EventLoopBuilderExtWayland;
use winit::platform::x11::EventLoopBuilderExtX11;
use winit::window::{Window, WindowAttributes, WindowId};

const Φ: f64 = 1.618033988749895;
const C: f64 = 299792458.0;
const GRID_TO_ANGLE: f64 = 4611686018427387904.0;
const GRID_INIT: f64 = 2147483648.0;
const JUMP_GRID: f64 = 268435456.0;
const SSAA_MAX: f32 = 8.0;
const EMA_FACTOR: f64 = 0.05;
const THRUST_STEP: f64 = 64.0;

pub type Record = (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
);

pub struct PackedWindow {
    pub field: Vec<f32>,
    pub meta: Vec<f32>,
    pub count: u32,
}

pub fn pack_window(records: &[Record], presence: [f64; 3]) -> PackedWindow {
    let n = records.len();
    let mut field = vec![0.0f32; n * 12];
    let mut meta = vec![0.0f32; n * 12];
    for (j, r) in records.iter().enumerate() {
        let f = j * 12;
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
        meta[f] = r.7 as f32;
        meta[f + 1] = r.6 as f32;
        meta[f + 2] = r.8 as f32;
        meta[f + 3] = 0.0;
        meta[f + 4] = r.15 as f32;
        meta[f + 5] = r.16 as f32;
        meta[f + 6] = r.17 as f32;
        meta[f + 7] = r.18 as f32;
        meta[f + 8] = r.19 as f32;
        meta[f + 9] = r.20 as f32;
        meta[f + 10] = 0.0;
        meta[f + 11] = 0.0;
    }
    PackedWindow {
        field,
        meta,
        count: n as u32,
    }
}

pub struct MathematikerinRadiator {
    tx: mpsc::SyncSender<Arc<Buffer>>,
    shutdown: Arc<AtomicBool>,
    _thread: Option<thread::JoinHandle<()>>,
}

impl MathematikerinRadiator {
    pub fn new(
        presence_tx: mpsc::Sender<(f64, f64, f64, f64, f64)>,
        body_names: Arc<Vec<String>>,
        time: Arc<Mutex<Option<LeapSeconds>>>,
    ) -> Self {
        let (tx, rx) = mpsc::sync_channel::<Arc<Buffer>>(2);
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();
        let handle = thread::spawn(move || {
            run_window(rx, presence_tx, body_names, time, shutdown_clone);
        });
        Self {
            tx,
            shutdown,
            _thread: Some(handle),
        }
    }
}

impl Radiator for MathematikerinRadiator {
    fn accept(&mut self, field: Arc<Buffer>) {
        let _ = self.tx.try_send(field);
    }
}

impl Drop for MathematikerinRadiator {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

fn q_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ]
}

fn q_norm(q: [f64; 4]) -> [f64; 4] {
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3])
        .sqrt()
        .max(1e-12);
    [q[0] / n, q[1] / n, q[2] / n, q[3] / n]
}

fn q_rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let p = [0.0, v[0], v[1], v[2]];
    let c = [q[0], -q[1], -q[2], -q[3]];
    let r = q_mul(q_mul(q, p), c);
    [r[1], r[2], r[3]]
}

fn q_axis_angle(axis: [f64; 3], angle: f64) -> [f64; 4] {
    let s = (angle / 2.0).sin();
    [(angle / 2.0).cos(), axis[0] * s, axis[1] * s, axis[2] * s]
}

fn le_bytes_f32(v: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len() * 4);
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
    out
}

fn storage_entry(read_only: bool, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
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

struct NativeApp {
    rx: mpsc::Receiver<Arc<Buffer>>,
    presence_tx: mpsc::Sender<(f64, f64, f64, f64, f64)>,
    body_names: Arc<Vec<String>>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
    shutdown: Arc<AtomicBool>,

    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    config: Option<wgpu::SurfaceConfiguration>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    render_pipe: Option<wgpu::RenderPipeline>,
    probe_pipe: Option<wgpu::ComputePipeline>,
    probe_layout: Option<wgpu::BindGroupLayout>,
    render_layout: Option<wgpu::BindGroupLayout>,
    render_bind: Option<wgpu::BindGroup>,
    probe_bind: Option<wgpu::BindGroup>,
    field_buf: Option<wgpu::Buffer>,
    meta_buf: Option<wgpu::Buffer>,
    vp_buf: Option<wgpu::Buffer>,
    probe_buf: Option<wgpu::Buffer>,
    field_cap: u32,
    backing: (u32, u32),

    latest_field: Option<Arc<Buffer>>,
    packed_field: Vec<f32>,
    packed_meta: Vec<f32>,
    packed_count: u32,
    packed_dirty: bool,
    last_response_epoch: f64,

    p: [f64; 3],
    v: [f64; 3],
    t0: f64,
    t_presence: f64,
    q: [f64; 4],
    grid_step: f64,
    ssaa: f32,
    exposure: f32,
    t_thrust: f64,
    t_thrust_target: f64,
    keys: HashSet<KeyCode>,
    shift: bool,
    cursor: Option<(f64, f64)>,
    drag_button: Option<MouseButton>,
    last_tick: Option<std::time::Instant>,
    stable_tick: f64,
    size: (u32, u32),
    scale_factor: f64,
    last_sent: (f64, f64, f64, f64),
    inflight: Arc<AtomicU32>,
}

impl NativeApp {
    fn new(
        rx: mpsc::Receiver<Arc<Buffer>>,
        presence_tx: mpsc::Sender<(f64, f64, f64, f64, f64)>,
        body_names: Arc<Vec<String>>,
        time: Arc<Mutex<Option<LeapSeconds>>>,
        shutdown: Arc<AtomicBool>,
    ) -> Self {
        Self {
            rx,
            presence_tx,
            body_names,
            time,
            shutdown,
            window: None,
            surface: None,
            config: None,
            device: None,
            queue: None,
            render_pipe: None,
            probe_pipe: None,
            probe_layout: None,
            render_layout: None,
            render_bind: None,
            probe_bind: None,
            field_buf: None,
            meta_buf: None,
            vp_buf: None,
            probe_buf: None,
            field_cap: 0,
            backing: (1, 1),
            latest_field: None,
            packed_field: Vec::new(),
            packed_meta: Vec::new(),
            packed_count: 0,
            packed_dirty: false,
            last_response_epoch: 0.0,
            p: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
            t0: 0.0,
            t_presence: 0.0,
            q: [1.0, 0.0, 0.0, 0.0],
            grid_step: GRID_INIT,
            ssaa: 1.0,
            exposure: 0.0,
            t_thrust: 0.0,
            t_thrust_target: 0.0,
            keys: HashSet::new(),
            shift: false,
            cursor: None,
            drag_button: None,
            last_tick: None,
            stable_tick: 0.0,
            size: (1280, 800),
            scale_factor: 1.0,
            last_sent: (0.0, 0.0, 0.0, 0.0),
            inflight: Arc::new(AtomicU32::new(0)),
        }
    }

    fn pos(&self) -> [f64; 3] {
        let dt = self.t_presence - self.t0;
        [
            self.p[0] + self.v[0] * dt,
            self.p[1] + self.v[1] * dt,
            self.p[2] + self.v[2] * dt,
        ]
    }

    fn frame(&self) -> ([f64; 3], [f64; 3], [f64; 3]) {
        (
            q_rotate(self.q, [1.0, 0.0, 0.0]),
            q_rotate(self.q, [0.0, 1.0, 0.0]),
            q_rotate(self.q, [0.0, 0.0, 1.0]),
        )
    }

    fn sense(&mut self) {
        let Some(field) = self.latest_field.clone() else {
            return;
        };
        let center = self.pos();
        let t = self.t_presence;
        let hx = self.size.0 as f64 * self.scale_factor * self.grid_step * 0.5;
        let hy = self.size.1 as f64 * self.scale_factor * self.grid_step * 0.5;
        let pad = 2.0 * (hx * hx + hy * hy).sqrt();
        let cache_interval = (self.grid_step / 30000.0).clamp(Φ, Φ * 10.0);
        let mut records: Vec<Record> = Vec::new();
        let eph = field.eph.clone();
        sense_buffer(&field, center, t, pad, cache_interval, &mut records, &eph);
        let packed = pack_window(&records, center);
        self.packed_count = packed.count;
        self.packed_field = packed.field;
        self.packed_meta = packed.meta;
        self.packed_dirty = true;
        self.last_response_epoch = t;
    }

    fn consider_resend(&mut self) {
        let [x, y, z] = self.pos();
        let (lx, ly, lz, ls) = self.last_sent;
        let moved = (x - lx).abs() >= self.grid_step
            || (y - ly).abs() >= self.grid_step
            || (z - lz).abs() >= self.grid_step
            || self.grid_step > ls * Φ
            || ls > self.grid_step * Φ;
        if !moved {
            return;
        }
        self.last_sent = (x, y, z, self.grid_step);
        let range = self.size.0.max(self.size.1) as f64 * self.scale_factor * self.grid_step * 2.0;
        let _ = self.presence_tx.send((self.t_presence, x, y, z, range));
        self.sense();
    }

    fn key_action(&mut self, code: KeyCode) {
        match code {
            KeyCode::KeyS => {
                self.p = [0.0, 0.0, 0.0];
                self.v = [0.0, 0.0, 0.0];
                self.t0 = self.t_presence;
                self.consider_resend();
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
                self.exposure += if self.shift { -1.0 } else { 1.0 };
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

    fn jump(&mut self, idx: usize) {
        let Some(name) = self.body_names.get(idx) else {
            return;
        };
        let Some(field) = self.latest_field.clone() else {
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

    fn reconfigure(&mut self) {
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
        let w = (self.size.0 as f64 * self.scale_factor * self.ssaa as f64).round() as u32;
        let h = (self.size.1 as f64 * self.scale_factor * self.ssaa as f64).round() as u32;
        config.width = w.max(1);
        config.height = h.max(1);
        surface.configure(device, &config);
        self.config = Some(config);
        self.backing = (w.max(1), h.max(1));
    }

    fn vp_data(&self) -> [f32; 32] {
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
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            self.exposure,
            self.last_response_epoch as f32,
            0.0,
            0.0,
            x as f32,
            y as f32,
            z as f32,
            self.t_presence as f32,
        ]
    }

    fn ensure_capacity(&mut self) {
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
        let field_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: c as u64 * 48,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let meta_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: c as u64 * 48,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let probe_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
            ],
        });
        let render_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
            ],
        });
        self.field_buf = Some(field_buf);
        self.meta_buf = Some(meta_buf);
        self.probe_bind = Some(probe_bind);
        self.render_bind = Some(render_bind);
    }

    fn render(&mut self) {
        if self.inflight.load(Ordering::Relaxed) > 2 {
            return;
        }
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(queue) = self.queue.clone() else {
            return;
        };
        if self.packed_dirty {
            self.ensure_capacity();
            if let Some(fb) = &self.field_buf {
                queue.write_buffer(fb, 0, &le_bytes_f32(&self.packed_field));
            }
            if let Some(mb) = &self.meta_buf {
                queue.write_buffer(mb, 0, &le_bytes_f32(&self.packed_meta));
            }
            self.packed_dirty = false;
        }
        let vp = self.vp_data();
        let mut bytes = [0u8; 128];
        for (i, x) in vp.iter().enumerate() {
            bytes[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        let Some(vp_buf) = self.vp_buf.as_ref() else {
            return;
        };
        queue.write_buffer(vp_buf, 0, &bytes);
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
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            if let (Some(pipe), Some(bind)) = (self.probe_pipe.as_ref(), self.probe_bind.as_ref()) {
                pass.set_pipeline(pipe);
                pass.set_bind_group(0, bind, &[]);
                pass.dispatch_workgroups(1, 1, 1);
            }
        }
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
            if let (Some(pipe), Some(bind)) = (self.render_pipe.as_ref(), self.render_bind.as_ref())
            {
                pass.set_pipeline(pipe);
                pass.set_bind_group(0, bind, &[]);
                pass.draw(0..6, 0..1);
            }
        }
        self.inflight.fetch_add(1, Ordering::Relaxed);
        queue.submit(std::iter::once(encoder.finish()));
        let counter = self.inflight.clone();
        queue.on_submitted_work_done(move || {
            counter.fetch_sub(1, Ordering::Relaxed);
        });
        frame.present();
    }

    fn init_gpu(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = WindowAttributes::default()
            .with_title("omegaflow φ")
            .with_inner_size(LogicalSize::new(1280.0, 800.0));
        let Ok(window) = event_loop.create_window(attrs) else {
            return;
        };
        self.size = (1280, 800);
        self.scale_factor = window.scale_factor();
        let window = Arc::new(window);
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = match instance.create_surface(window.clone()) {
            Ok(s) => s,
            Err(_) => return,
        };
        let adapter =
            match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })) {
                Some(a) => a,
                None => return,
            };
        let (device, queue) = match pollster::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
        ) {
            Ok(dq) => dq,
            Err(_) => return,
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
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/membrane.wgsl").into()),
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
            ],
        });
        let render_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::FRAGMENT);
                    e.binding = 0;
                    e
                },
                {
                    let mut e = storage_entry(true, wgpu::ShaderStages::FRAGMENT);
                    e.binding = 1;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
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
        let vp_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 128,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let probe_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 36,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
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
        self.reconfigure();
        self.ensure_capacity();
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
        let now_i = std::time::Instant::now();
        let raw = match self.last_tick {
            Some(prev) => now_i.duration_since(prev).as_secs_f64() * 1000.0,
            None => Φ,
        };
        self.last_tick = Some(now_i);
        self.stable_tick = self.stable_tick * (1.0 - EMA_FACTOR) + raw * EMA_FACTOR;
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
        self.t_presence += (1.0 + self.t_thrust) * raw / 1000.0;
        let pan_speed = self.grid_step * if self.shift { 4.0 } else { 1.0 } * raw / 1000.0;
        let (fr, fu, ff) = self.frame();
        if self.keys.contains(&KeyCode::ArrowRight) {
            self.p[0] += fr[0] * pan_speed;
            self.p[1] += fr[1] * pan_speed;
            self.p[2] += fr[2] * pan_speed;
        }
        if self.keys.contains(&KeyCode::ArrowLeft) {
            self.p[0] -= fr[0] * pan_speed;
            self.p[1] -= fr[1] * pan_speed;
            self.p[2] -= fr[2] * pan_speed;
        }
        if self.keys.contains(&KeyCode::ArrowUp) {
            self.p[0] += ff[0] * pan_speed;
            self.p[1] += ff[1] * pan_speed;
            self.p[2] += ff[2] * pan_speed;
        }
        if self.keys.contains(&KeyCode::ArrowDown) {
            self.p[0] -= ff[0] * pan_speed;
            self.p[1] -= ff[1] * pan_speed;
            self.p[2] -= ff[2] * pan_speed;
        }
        if self.keys.contains(&KeyCode::PageUp) {
            self.p[0] += fu[0] * pan_speed;
            self.p[1] += fu[1] * pan_speed;
            self.p[2] += fu[2] * pan_speed;
        }
        if self.keys.contains(&KeyCode::PageDown) {
            self.p[0] -= fu[0] * pan_speed;
            self.p[1] -= fu[1] * pan_speed;
            self.p[2] -= fu[2] * pan_speed;
        }
        if let Ok(field) = self.rx.try_recv() {
            self.latest_field = Some(field);
            self.sense();
        }
        self.consider_resend();
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.size = (size.width.max(1), size.height.max(1));
                self.reconfigure();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor;
                self.reconfigure();
            }
            WindowEvent::ModifiersChanged(m) => {
                self.shift = m.state().shift_key();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => {
                            if !self.keys.contains(&code) {
                                self.keys.insert(code);
                                self.key_action(code);
                            }
                        }
                        ElementState::Released => {
                            self.keys.remove(&code);
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
                    let (fr, fu, _) = self.frame();
                    match self.drag_button {
                        Some(MouseButton::Left) => {
                            if dx != 0.0 {
                                self.q = q_norm(q_mul(
                                    q_axis_angle(fu, dx * self.grid_step / GRID_TO_ANGLE),
                                    self.q,
                                ));
                            }
                            if dy != 0.0 {
                                self.q = q_norm(q_mul(
                                    q_axis_angle(fr, dy * self.grid_step / GRID_TO_ANGLE),
                                    self.q,
                                ));
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
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.drag_button = match state {
                    ElementState::Pressed => Some(button),
                    ElementState::Released => None,
                };
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y as f64 * 32.0,
                    MouseScrollDelta::PixelDelta(p) => p.y,
                };
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
            WindowEvent::RedrawRequested => self.render(),
            _ => {}
        }
    }
}

fn run_window(
    rx: mpsc::Receiver<Arc<Buffer>>,
    presence_tx: mpsc::Sender<(f64, f64, f64, f64, f64)>,
    body_names: Arc<Vec<String>>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
    shutdown: Arc<AtomicBool>,
) {
    let mut builder = EventLoopBuilder::<()>::default();
    EventLoopBuilderExtX11::with_any_thread(&mut builder, true);
    EventLoopBuilderExtWayland::with_any_thread(&mut builder, true);
    let event_loop = match builder.build() {
        Ok(el) => el,
        Err(_) => return,
    };
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = NativeApp::new(rx, presence_tx, body_names, time, shutdown);
    let _ = event_loop.run_app(&mut app);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_pack_slots_against_wgsl_access() {
        let presence = [1.0e3, 2.0e3, 3.0e3];
        let r: Record = (
            7001.0, 7002.0, 7003.0, 7004.0, 7005.0, 7006.0, 7007.0, 7008.0, 7009.0, 7010.0, 7011.0,
            7012.0, 7013.0, 7014.0, 7015.0, 7016.0, 7017.0, 7018.0, 7019.0, 7020.0, 7021.0,
        );
        let packed = pack_window(&[r], presence);
        assert_eq!(packed.count, 1);
        let f = &packed.field;
        assert_eq!(f[0], 6001.0);
        assert_eq!(f[1], 5002.0);
        assert_eq!(f[2], 4003.0);
        assert_eq!(f[3], 7004.0);
        assert_eq!(f[4], 7005.0);
        assert_eq!(f[5], 7006.0);
        assert_eq!(f[6], 7010.0);
        assert_eq!(f[7], 7011.0);
        assert_eq!(f[8], 7012.0);
        assert_eq!(f[9], 7013.0);
        assert_eq!(f[10], 7014.0);
        assert_eq!(f[11], 7015.0);
        let m = &packed.meta;
        assert_eq!(m[0], 7008.0);
        assert_eq!(m[1], 7007.0);
        assert_eq!(m[2], 7009.0);
        assert_eq!(m[3], 0.0);
        assert_eq!(m[4], 7016.0);
        assert_eq!(m[5], 7017.0);
        assert_eq!(m[6], 7018.0);
        assert_eq!(m[7], 7019.0);
        assert_eq!(m[8], 7020.0);
        assert_eq!(m[9], 7021.0);
        assert_eq!(m[10], 0.0);
        assert_eq!(m[11], 0.0);
    }
}
