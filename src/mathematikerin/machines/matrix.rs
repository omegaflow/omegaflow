use super::*;

pub const MATRIX_GRID: u32 = 21600;

pub const MATRIX_RING_MAX: usize = 1024;

pub const MATRIX_PROBE_MAX: usize = 256;

pub const MATRIX_N_GATE: usize = 30;

pub const MATRIX_SCALES: [f32; 3] = [1.0, 0.5, 2.0];

pub const MATRIX_SCALE_NAMES: [&str; 3] = ["h", "h/2", "2h"];

pub const MATRIX_SHIFT_STEP_BINS: i64 = 4;

pub const MATRIX_SHIFT_MAX_DAYS: i64 = 30;

pub const MATRIX_SHIFT_COUNT: usize = (MATRIX_SHIFT_MAX_DAYS * 2 + 1) as usize;

pub const MATRIX_CELLS_PER_SCALE: usize = MATRIX_SHIFT_COUNT * 2;

pub const MATRIX_CELLS_PER_ROUND: usize = MATRIX_CELLS_PER_SCALE * MATRIX_SCALES.len();

pub const MATRIX_PAD_M: f64 = 2000.0;

pub const MATRIX_STATE_FILE: &str = "omegaflow_matrix_state.bin";

pub fn matrix_state_path() -> String {
    let base = if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        std::path::PathBuf::from(dir)
    } else if let Ok(home) = std::env::var("HOME") {
        std::path::PathBuf::from(home)
            .join(".local")
            .join("state")
            .join("omegaflow")
    } else {
        std::path::PathBuf::from(".")
    };
    base.join(MATRIX_STATE_FILE).to_string_lossy().into_owned()
}

fn rd_u16(b: &[u8], p: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
    *p += 2;
    Some(v)
}

fn rd_u32(b: &[u8], p: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v)
}

fn rd_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn rd_f64(b: &[u8], p: &mut usize) -> Option<f64> {
    let v = f64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn rd_f32(b: &[u8], p: &mut usize) -> Option<f32> {
    let v = f32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v)
}

fn rd_name(b: &[u8], p: &mut usize) -> Option<String> {
    let len = rd_u16(b, p)? as usize;
    let s = std::str::from_utf8(b.get(*p..*p + len)?).ok()?;
    *p += len;
    Some(s.to_string())
}

fn wr_name(buf: &mut Vec<u8>, name: &str) {
    let nb = name.as_bytes();
    buf.extend_from_slice(&(nb.len() as u16).to_le_bytes());
    buf.extend_from_slice(nb);
}

#[derive(Clone, Copy)]
pub struct MatrixCellVerdict {
    pub dir: u8,
    pub shift: i64,
    pub n: usize,
    pub te: f64,
    pub thr: f64,
}

pub struct MatrixAccum {
    pub m: usize,
    pub fam: f64,
    pub fam_any: bool,
    pub surr_over: usize,
    pub surr_total: usize,
    pub cells1: Vec<MatrixCellVerdict>,
    pub small: Vec<MatrixCellVerdict>,
}

impl MatrixAccum {
    pub fn fresh() -> MatrixAccum {
        MatrixAccum {
            m: 0,
            fam: 0.0,
            fam_any: false,
            surr_over: 0,
            surr_total: 0,
            cells1: Vec::new(),
            small: Vec::new(),
        }
    }
}

pub struct PairResult {
    pub cells: usize,
    pub accum: MatrixAccum,
    pub done: bool,
}

impl PairResult {
    pub fn fresh() -> PairResult {
        PairResult {
            cells: 0,
            accum: MatrixAccum::fresh(),
            done: false,
        }
    }
}

pub struct MatrixLine {
    pub pairs: usize,
    pub arrows: usize,
    pub family: usize,
    pub h_bound: usize,
    pub silent: usize,
    pub absent: usize,
    pub expected: f64,
}

impl MatrixLine {
    pub fn fresh() -> MatrixLine {
        MatrixLine {
            pairs: 0,
            arrows: 0,
            family: 0,
            h_bound: 0,
            silent: 0,
            absent: 0,
            expected: 0.0,
        }
    }
}

pub enum MetaAnchor {
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
    },
    Barycenter {
        body_name: String,
        scale: f64,
    },
}

pub struct NameMeta {
    pub anchor: MetaAnchor,
    pub force: u8,
    pub kernel: u8,
    pub tau: f64,
}

pub struct MatrixMachine {
    pub rings: HashMap<String, Vec<(f64, f32)>>,
    pub metas: HashMap<String, NameMeta>,
    pub rx: Option<
        mpsc::Receiver<(
            crate::archivar::Frame,
            Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
        )>,
    >,
    pub present: Vec<String>,
    pub results: HashMap<String, PairResult>,
    pub line: MatrixLine,
    pub last_presence: Option<[f64; 3]>,
    pub last_field: Option<Arc<crate::archivar::Buffer>>,
    pub last_rebuild: Option<std::time::Instant>,
    pub last_state_save: Option<std::time::Instant>,
    pub state_path: String,
    pub due: bool,
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub te_pipe: Option<wgpu::ComputePipeline>,
    pub te_bind: Option<wgpu::BindGroup>,
    pub te_series_buf: Option<wgpu::Buffer>,
    pub te_param_buf: Option<wgpu::Buffer>,
    pub te_out_buf: Option<wgpu::Buffer>,
    pub te_read_buf: Option<wgpu::Buffer>,
    pub te_map: Option<Arc<AtomicBool>>,
    pub pending: Option<(String, String, usize)>,
    pub pending_n: usize,
    pub pending_since: Option<std::time::Instant>,
    pub named: String,
    pub rng: u64,
}

impl MatrixMachine {
    pub fn new(
        rx: mpsc::Receiver<(
            crate::archivar::Frame,
            Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
        )>,
    ) -> MatrixMachine {
        MatrixMachine {
            rings: HashMap::new(),
            metas: HashMap::new(),
            rx: Some(rx),
            present: Vec::new(),
            results: HashMap::new(),
            line: MatrixLine::fresh(),
            last_presence: None,
            last_field: None,
            last_rebuild: None,
            last_state_save: None,
            state_path: matrix_state_path(),
            due: false,
            device: None,
            queue: None,
            te_pipe: None,
            te_bind: None,
            te_series_buf: None,
            te_param_buf: None,
            te_out_buf: None,
            te_read_buf: None,
            te_map: None,
            pending: None,
            pending_n: 0,
            pending_since: None,
            named: String::new(),
            rng: 0,
        }
    }

    pub fn save_state(&self) {
        self.save_state_to(&self.state_path);
    }

    pub fn load_state(&mut self) {
        let Some(s) = MatrixMachine::load_state_from(&self.state_path) else {
            return;
        };
        self.rings = s.rings;
        self.metas = s.metas;
        self.present = s.present;
        self.results = s.results;
        self.line = s.line;
        self.due = s.due;
    }

    pub fn save_state_to(&self, path: &str) {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(b"OMX2");
        buf.extend_from_slice(&(self.rings.len() as u32).to_le_bytes());
        for (name, ring) in &self.rings {
            wr_name(&mut buf, name);
            buf.extend_from_slice(&(ring.len() as u32).to_le_bytes());
            for &(t, v) in ring {
                buf.extend_from_slice(&t.to_le_bytes());
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
        buf.extend_from_slice(&(self.metas.len() as u32).to_le_bytes());
        for (name, meta) in &self.metas {
            wr_name(&mut buf, name);
            buf.push(meta.force);
            buf.push(meta.kernel);
            buf.extend_from_slice(&meta.tau.to_le_bytes());
            match &meta.anchor {
                MetaAnchor::Surface {
                    body_name,
                    lat,
                    lon,
                    alt,
                } => {
                    buf.push(0u8);
                    wr_name(&mut buf, body_name);
                    buf.extend_from_slice(&lat.to_le_bytes());
                    buf.extend_from_slice(&lon.to_le_bytes());
                    buf.extend_from_slice(&alt.to_le_bytes());
                }
                MetaAnchor::Barycenter { body_name, scale } => {
                    buf.push(1u8);
                    wr_name(&mut buf, body_name);
                    buf.extend_from_slice(&scale.to_le_bytes());
                }
            }
        }
        buf.extend_from_slice(&(self.present.len() as u32).to_le_bytes());
        for name in &self.present {
            wr_name(&mut buf, name);
        }
        buf.push(if self.due { 1 } else { 0 });
        buf.extend_from_slice(&(self.results.len() as u32).to_le_bytes());
        for (key, r) in &self.results {
            wr_name(&mut buf, key);
            buf.extend_from_slice(&(r.cells as u32).to_le_bytes());
            buf.push(if r.done { 1 } else { 0 });
            buf.extend_from_slice(&(r.accum.m as u64).to_le_bytes());
            buf.extend_from_slice(&r.accum.fam.to_le_bytes());
            buf.push(if r.accum.fam_any { 1 } else { 0 });
            buf.extend_from_slice(&(r.accum.surr_over as u64).to_le_bytes());
            buf.extend_from_slice(&(r.accum.surr_total as u64).to_le_bytes());
            for cells in [&r.accum.cells1, &r.accum.small] {
                buf.extend_from_slice(&(cells.len() as u32).to_le_bytes());
                for c in cells.iter() {
                    buf.push(c.dir);
                    buf.extend_from_slice(&c.shift.to_le_bytes());
                    buf.extend_from_slice(&(c.n as u64).to_le_bytes());
                    buf.extend_from_slice(&c.te.to_le_bytes());
                    buf.extend_from_slice(&c.thr.to_le_bytes());
                }
            }
        }
        let l = &self.line;
        for v in [l.pairs, l.arrows, l.family, l.h_bound, l.silent, l.absent] {
            buf.extend_from_slice(&(v as u64).to_le_bytes());
        }
        buf.extend_from_slice(&l.expected.to_le_bytes());
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(path, &buf);
    }

    pub fn load_state_from(path: &str) -> Option<MatrixMachine> {
        let bytes = std::fs::read(path).ok()?;
        let mut p = 0usize;
        let magic = bytes.get(p..p + 4)?;
        if magic != b"OMX1" && magic != b"OMX2" {
            return None;
        }
        p += 4;
        let mut rings: HashMap<String, Vec<(f64, f32)>> = HashMap::new();
        let n = rd_u32(&bytes, &mut p)? as usize;
        for _ in 0..n {
            let name = rd_name(&bytes, &mut p)?;
            let len = rd_u32(&bytes, &mut p)? as usize;
            let mut ring = Vec::with_capacity(len);
            for _ in 0..len {
                let t = rd_f64(&bytes, &mut p)?;
                let v = rd_f32(&bytes, &mut p)?;
                ring.push((t, v));
            }
            rings.insert(name, ring);
        }
        let mut metas: HashMap<String, NameMeta> = HashMap::new();
        let n = rd_u32(&bytes, &mut p)? as usize;
        for _ in 0..n {
            let name = rd_name(&bytes, &mut p)?;
            let force = *bytes.get(p)?;
            p += 1;
            let kernel = *bytes.get(p)?;
            p += 1;
            let tau = rd_f64(&bytes, &mut p)?;
            let anchor = match *bytes.get(p)? {
                0 => {
                    p += 1;
                    let body_name = rd_name(&bytes, &mut p)?;
                    let lat = rd_f64(&bytes, &mut p)?;
                    let lon = rd_f64(&bytes, &mut p)?;
                    let alt = rd_f64(&bytes, &mut p)?;
                    MetaAnchor::Surface {
                        body_name,
                        lat,
                        lon,
                        alt,
                    }
                }
                1 => {
                    p += 1;
                    let body_name = rd_name(&bytes, &mut p)?;
                    let scale = rd_f64(&bytes, &mut p)?;
                    MetaAnchor::Barycenter { body_name, scale }
                }
                _ => {
                    p += 1;
                    continue;
                }
            };
            metas.insert(
                name,
                NameMeta {
                    anchor,
                    force,
                    kernel,
                    tau,
                },
            );
        }
        let mut present: Vec<String> = Vec::new();
        let n = rd_u32(&bytes, &mut p)? as usize;
        for _ in 0..n {
            present.push(rd_name(&bytes, &mut p)?);
        }
        if magic == b"OMX1" {
            return Some(MatrixMachine {
                rings,
                metas,
                rx: None,
                present,
                results: HashMap::new(),
                line: MatrixLine::fresh(),
                last_presence: None,
                last_field: None,
                last_rebuild: None,
                last_state_save: None,
                state_path: matrix_state_path(),
                due: false,
                device: None,
                queue: None,
                te_pipe: None,
                te_bind: None,
                te_series_buf: None,
                te_param_buf: None,
                te_out_buf: None,
                te_read_buf: None,
                te_map: None,
                pending: None,
                pending_n: 0,
                pending_since: None,
                named: String::new(),
                rng: 0,
            });
        }
        let due = *bytes.get(p)? == 1;
        p += 1;
        let mut results: HashMap<String, PairResult> = HashMap::new();
        let n = rd_u32(&bytes, &mut p)? as usize;
        for _ in 0..n {
            let key = rd_name(&bytes, &mut p)?;
            let cells = rd_u32(&bytes, &mut p)? as usize;
            let done = *bytes.get(p)? == 1;
            p += 1;
            let mut accum = MatrixAccum::fresh();
            accum.m = rd_u64(&bytes, &mut p)? as usize;
            accum.fam = rd_f64(&bytes, &mut p)?;
            accum.fam_any = *bytes.get(p)? == 1;
            p += 1;
            accum.surr_over = rd_u64(&bytes, &mut p)? as usize;
            accum.surr_total = rd_u64(&bytes, &mut p)? as usize;
            for cells_v in [&mut accum.cells1, &mut accum.small] {
                let n = rd_u32(&bytes, &mut p)? as usize;
                for _ in 0..n {
                    let dir = *bytes.get(p)?;
                    p += 1;
                    let shift = rd_u64(&bytes, &mut p)? as i64;
                    let cn = rd_u64(&bytes, &mut p)? as usize;
                    let te = rd_f64(&bytes, &mut p)?;
                    let thr = rd_f64(&bytes, &mut p)?;
                    cells_v.push(MatrixCellVerdict {
                        dir,
                        shift,
                        n: cn,
                        te,
                        thr,
                    });
                }
            }
            results.insert(key, PairResult { cells, accum, done });
        }
        let mut line = MatrixLine::fresh();
        line.pairs = rd_u64(&bytes, &mut p)? as usize;
        line.arrows = rd_u64(&bytes, &mut p)? as usize;
        line.family = rd_u64(&bytes, &mut p)? as usize;
        line.h_bound = rd_u64(&bytes, &mut p)? as usize;
        line.silent = rd_u64(&bytes, &mut p)? as usize;
        line.absent = rd_u64(&bytes, &mut p)? as usize;
        line.expected = rd_f64(&bytes, &mut p)?;
        Some(MatrixMachine {
            rings,
            metas,
            rx: None,
            present,
            results,
            line,
            last_presence: None,
            last_field: None,
            last_rebuild: None,
            last_state_save: None,
            state_path: matrix_state_path(),
            due,
            device: None,
            queue: None,
            te_pipe: None,
            te_bind: None,
            te_series_buf: None,
            te_param_buf: None,
            te_out_buf: None,
            te_read_buf: None,
            te_map: None,
            pending: None,
            pending_n: 0,
            pending_since: None,
            named: String::new(),
            rng: 0,
        })
    }

    pub fn bind_gpu(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        te_pipe: wgpu::ComputePipeline,
        te_layout: &wgpu::BindGroupLayout,
    ) {
        let series_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: TE_SERIES_BYTES,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let param_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let out_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 288,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: te_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: series_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: param_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: out_buf.as_entire_binding(),
                },
            ],
        });
        self.device = Some(device.clone());
        self.queue = Some(queue.clone());
        self.te_pipe = Some(te_pipe);
        self.te_bind = Some(bind);
        self.te_series_buf = Some(series_buf);
        self.te_param_buf = Some(param_buf);
        self.te_out_buf = Some(out_buf);
        self.te_read_buf = Some(read_buf);
    }

    pub fn say(&mut self, line: String) {
        if self.named != line {
            eprintln!("{}", line);
            self.named = line;
        }
    }

    pub fn cell_label(&self, a: &str, b: &str, cell_idx: usize) -> String {
        let (scale_idx, dir, shift_days) = matrix_cell_desc(cell_idx);
        let (driver, target) = if dir == 0 { (a, b) } else { (b, a) };
        format!(
            "matrix te {}→{} {} {}d",
            driver, target, MATRIX_SCALE_NAMES[scale_idx as usize], shift_days
        )
    }

    pub fn record(
        &mut self,
        frame: &crate::archivar::Frame,
        channels: Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
    ) {
        let frame_anchor = match frame {
            crate::archivar::Frame::Surface {
                body_name,
                lat,
                lon,
                alt,
            } => Some(MetaAnchor::Surface {
                body_name: body_name.clone(),
                lat: *lat,
                lon: *lon,
                alt: *alt,
            }),
            crate::archivar::Frame::Barycenter { body_name, scale } => {
                Some(MetaAnchor::Barycenter {
                    body_name: body_name.clone(),
                    scale: *scale,
                })
            }
            crate::archivar::Frame::Manifest => None,
        };
        for (channel, sensor) in channels {
            if !(channel.epoch > 0.0) || !channel.epoch.is_finite() {
                continue;
            }
            let anchor = match &channel.position {
                crate::archivar::Position::Surface {
                    body_name,
                    lat,
                    lon,
                    alt,
                } => MetaAnchor::Surface {
                    body_name: body_name.clone(),
                    lat: *lat,
                    lon: *lon,
                    alt: *alt,
                },
                _ => match &frame_anchor {
                    Some(a) => match a {
                        MetaAnchor::Surface {
                            body_name,
                            lat,
                            lon,
                            alt,
                        } => MetaAnchor::Surface {
                            body_name: body_name.clone(),
                            lat: *lat,
                            lon: *lon,
                            alt: *alt,
                        },
                        MetaAnchor::Barycenter { body_name, scale } => MetaAnchor::Barycenter {
                            body_name: body_name.clone(),
                            scale: *scale,
                        },
                    },
                    None => continue,
                },
            };
            let meta = NameMeta {
                anchor,
                force: sensor.force,
                kernel: sensor.kernel,
                tau: sensor.tau,
            };
            self.metas.insert(channel.name.clone(), meta);
            let ring = self.rings.entry(channel.name.clone()).or_default();
            insert_bin(ring, channel.epoch, channel.value as f32);
        }
    }

    pub fn rebuild(&mut self, presence: [f64; 3], t_presence: f64) {
        let Some(field) = self.last_field.clone() else {
            return;
        };
        if !self.metas.is_empty() {
            let any_anchor_body = self.metas.values().any(|m| {
                let body = match &m.anchor {
                    MetaAnchor::Surface { body_name, .. }
                    | MetaAnchor::Barycenter { body_name, .. } => body_name,
                };
                field
                    .eph
                    .get(body.as_str())
                    .and_then(|e| e.props.as_ref())
                    .is_some()
            });
            if !any_anchor_body {
                return;
            }
        }
        let mut present: Vec<String> = Vec::new();
        let mut point_anchor: Option<MetaAnchor> = None;
        for (name, meta) in &self.metas {
            let (motion, body_props): (
                crate::archivar::Motion,
                Option<&crate::archivar::BodyProperties>,
            ) = match &meta.anchor {
                MetaAnchor::Surface {
                    body_name,
                    lat,
                    lon,
                    alt,
                } => (
                    crate::archivar::Motion::Surface {
                        body_name: body_name.clone(),
                        lat: *lat,
                        lon: *lon,
                        alt: *alt,
                    },
                    field
                        .eph
                        .get(body_name.as_str())
                        .and_then(|e| e.props.as_ref()),
                ),
                MetaAnchor::Barycenter { .. } => continue,
            };
            let Some(body_props) = body_props else {
                continue;
            };
            let Some(p) = motion.at(t_presence, t_presence, &field.eph) else {
                continue;
            };
            let d2 = (p[0] - presence[0]).powi(2)
                + (p[1] - presence[1]).powi(2)
                + (p[2] - presence[2]).powi(2);
            let extent =
                crate::archivar::kernel_extent(meta.force, meta.kernel, Some(body_props), meta.tau);
            if d2 > (extent + MATRIX_PAD_M).powi(2) {
                continue;
            }
            if point_anchor.is_none() {
                point_anchor = Some(match &meta.anchor {
                    MetaAnchor::Surface {
                        body_name,
                        lat,
                        lon,
                        alt,
                    } => MetaAnchor::Surface {
                        body_name: body_name.clone(),
                        lat: *lat,
                        lon: *lon,
                        alt: *alt,
                    },
                    MetaAnchor::Barycenter { body_name, scale } => MetaAnchor::Barycenter {
                        body_name: body_name.clone(),
                        scale: *scale,
                    },
                });
            }
            present.push(name.clone());
        }
        let mut epochs: Vec<f64> = present
            .iter()
            .filter_map(|n| self.rings.get(n))
            .flat_map(|r| r.iter().map(|&(t, _)| t))
            .collect();
        epochs.sort_by(|a, b| a.total_cmp(b));
        epochs.dedup();
        if let Some(MetaAnchor::Surface {
            body_name,
            lat,
            lon,
            alt,
        }) = point_anchor
        {
            let motion = crate::archivar::Motion::Surface {
                body_name,
                lat,
                lon,
                alt,
            };
            let (ob_cos, ob_sin) = 23.4392911_f64.to_radians().sin_cos();
            for (body, eph) in field.eph.iter() {
                let mut series: Vec<(f64, f32)> = Vec::new();
                let mut lon_sin: Vec<(f64, f32)> = Vec::new();
                let mut lon_cos: Vec<(f64, f32)> = Vec::new();
                let gm = eph.props.as_ref().and_then(|p| p.gm);
                for &t in &epochs {
                    let Some(pb) = crate::archivar::body_barycenter_position(body, t, &field.eph)
                    else {
                        continue;
                    };
                    let Some(pf) = motion.at(t, t, &field.eph) else {
                        continue;
                    };
                    let rx = pb[0] - pf[0];
                    let ry = pb[1] - pf[1];
                    let rz = pb[2] - pf[2];
                    let r2 = rx * rx + ry * ry + rz * rz;
                    if !(r2 > 0.0) {
                        continue;
                    }
                    if let Some(gm) = gm {
                        if gm > 0.0 {
                            series.push((t, (gm / r2) as f32));
                        }
                    }
                    let y_ecl = ry * ob_cos + rz * ob_sin;
                    let lon = y_ecl.atan2(rx);
                    lon_sin.push((t, lon.sin() as f32));
                    lon_cos.push((t, lon.cos() as f32));
                }
                if let Some(gm) = gm {
                    if gm > 0.0 && series.len() >= MATRIX_N_GATE {
                        present.push(format!("eph_{}", body));
                        self.rings.insert(format!("eph_{}", body), series);
                    }
                }
                if lon_sin.len() >= MATRIX_N_GATE {
                    present.push(format!("eph_{}_lon_sin", body));
                    self.rings.insert(format!("eph_{}_lon_sin", body), lon_sin);
                }
                if lon_cos.len() >= MATRIX_N_GATE {
                    present.push(format!("eph_{}_lon_cos", body));
                    self.rings.insert(format!("eph_{}_lon_cos", body), lon_cos);
                }
            }
        }
        present.sort_by(|a, b| {
            let a_eph = a.starts_with("eph_");
            let b_eph = b.starts_with("eph_");
            (a_eph, a.as_str()).cmp(&(b_eph, b.as_str()))
        });
        let non_eph: Vec<String> = present
            .iter()
            .filter(|n| !n.starts_with("eph_"))
            .cloned()
            .collect();
        let prev_non_eph: Vec<String> = self
            .present
            .iter()
            .filter(|n| !n.starts_with("eph_"))
            .cloned()
            .collect();
        let changed = non_eph != prev_non_eph;
        if changed {
            self.present = present.clone();
            let counts: Vec<String> = present
                .iter()
                .map(|n| format!("{}:{}", n, self.rings[n].len()))
                .collect();
            self.say(format!(
                "matrix rings {} {}",
                present.len(),
                counts.join(" ")
            ));
        }
        self.due = true;
    }

    pub fn fold(&mut self, a: &str, b: &str, cell_idx: usize, n: usize) {
        let Some(read_buf) = self.te_read_buf.as_ref() else {
            return;
        };
        let verdict = te_read_verdict(read_buf);
        let label = self.cell_label(a, b, cell_idx);
        let (scale_idx, dir, shift_days) = matrix_cell_desc(cell_idx);
        let Some(v) = crate::te::topological_verdict_from_gpu(&verdict) else {
            self.say(format!(
                "{} n {} state {}",
                label,
                n,
                te_absence_word(&verdict)
            ));
            return;
        };
        let excess = v.te - v.threshold;
        let key = pair_key(a, b);
        let result = self.results.entry(key).or_insert_with(PairResult::fresh);
        let accum = &mut result.accum;
        if scale_idx == 0 {
            accum.m += 1;
            for s in 2..12 {
                if verdict[s * 6 + 4] == 1.0 {
                    let st = verdict[s * 6 + 1] as f64;
                    if !accum.fam_any || st > accum.fam {
                        accum.fam = st;
                        accum.fam_any = true;
                    }
                    if st > v.threshold {
                        accum.surr_over += 1;
                    }
                    accum.surr_total += 1;
                }
            }
            accum.cells1.push(MatrixCellVerdict {
                dir,
                shift: shift_days,
                n,
                te: v.te,
                thr: v.threshold,
            });
        } else {
            accum.small.push(MatrixCellVerdict {
                dir,
                shift: shift_days,
                n,
                te: v.te,
                thr: v.threshold,
            });
        }
        if excess > 0.0 {
            self.say(format!(
                "{} n {} te {:.3} thr {:.3} state arrow",
                label, n, v.te, v.threshold
            ));
        }
    }

    pub fn collect(&mut self) {
        let Some(prev) = self.te_map.take() else {
            return;
        };
        if !prev.load(Ordering::SeqCst) {
            let stale = self
                .pending_since
                .map_or(false, |i| i.elapsed().as_secs_f64() > 30.0);
            if stale {
                self.pending = None;
                self.pending_since = None;
                self.say("matrix te readback timeout — cell pending".to_string());
                return;
            }
            self.te_map = Some(prev);
            return;
        }
        if let Some((a, b, cell)) = self.pending.take() {
            self.pending_since = None;
            let n = self.pending_n;
            self.fold(&a, &b, cell, n);
        }
    }

    pub fn probe(&mut self, a: &str, b: &str, cell_idx: usize) {
        let label = self.cell_label(a, b, cell_idx);
        let (scale_idx, dir, shift_days) = matrix_cell_desc(cell_idx);
        let shift_s = (shift_days * MATRIX_SHIFT_STEP_BINS) as f64 * MATRIX_GRID as f64;
        let h_scale = MATRIX_SCALES[scale_idx as usize];
        let (driver, target) = if dir == 0 { (a, b) } else { (b, a) };
        let driver_ring = &self.rings[driver];
        let target_ring = &self.rings[target];
        let (mut ys, mut xs) = shift_pair(driver_ring, target_ring, shift_s);
        if xs.len() > MATRIX_PROBE_MAX {
            let drop = xs.len() - MATRIX_PROBE_MAX;
            ys.drain(..drop);
            xs.drain(..drop);
        }
        if xs.len() < MATRIX_N_GATE {
            return;
        }
        let degenerate = |v: &[f32]| -> bool {
            let mut mn = v[0];
            let mut mx = v[0];
            for &x in v {
                mn = mn.min(x);
                mx = mx.max(x);
            }
            let scale = mx.abs().max(mn.abs()).max(1.0);
            (mx - mn) < 1e-9 * scale
        };
        if degenerate(&xs) || degenerate(&ys) {
            return;
        }
        let Some(device) = self.device.clone() else {
            return;
        };
        let Some(queue) = self.queue.clone() else {
            return;
        };
        let Some(pipe) = self.te_pipe.as_ref() else {
            return;
        };
        let Some(bind) = self.te_bind.as_ref() else {
            return;
        };
        let Some(series_buf) = self.te_series_buf.as_ref() else {
            return;
        };
        let Some(param_buf) = self.te_param_buf.as_ref() else {
            return;
        };
        let Some(out_buf) = self.te_out_buf.as_ref() else {
            return;
        };
        let Some(read_buf) = self.te_read_buf.as_ref() else {
            return;
        };
        let m = xs.len();
        let mut data = vec![0f32; 12 * TE_SERIES_STRIDE];
        data[0..m].copy_from_slice(&xs);
        data[TE_SERIES_STRIDE..TE_SERIES_STRIDE + m].copy_from_slice(&ys);
        let mut rng = self.rng.wrapping_add(0x9e3779b97f4a7c15);
        for s in 0..10 {
            let surr = crate::te::phase_randomized_surrogate(&ys, &mut rng);
            let off = (2 + s) * TE_SERIES_STRIDE;
            data[off..off + m].copy_from_slice(&surr);
        }
        queue.write_buffer(series_buf, 0, &le_bytes_f32(&data));
        let max_lag = (m as f64 / Φ) as u32;
        let param = [m as u32, max_lag, h_scale.to_bits(), 0];
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
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(250);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            device.poll(wgpu::Maintain::Poll);
        }
        if !mapped.load(Ordering::SeqCst) {
            self.te_map = Some(mapped);
            self.pending = Some((a.to_string(), b.to_string(), cell_idx));
            self.pending_n = m;
            self.pending_since = Some(std::time::Instant::now());
            self.say(format!("{} state readback pending", label));
            return;
        }
        self.fold(a, b, cell_idx, m);
    }

    pub fn sheet(&mut self, a: &str, b: &str) {
        let key = pair_key(a, b);
        let (state, sheet_parts, p_hat, m_count) = {
            let acc = &self.results[&key].accum;
            let mut best: Option<(u8, i64, usize, f64, f64, f64)> = None;
            for c in &acc.cells1 {
                let e = c.te - c.thr;
                if best.map_or(true, |(_, _, _, be, _, _)| e > be) {
                    best = Some((c.dir, c.shift, c.n, e, c.te, c.thr));
                }
            }
            let p_hat = if acc.surr_total > 0 {
                acc.surr_over as f64 / acc.surr_total as f64
            } else {
                0.0
            };
            if let Some((dir, d_star, n_star, excess, te, thr)) = best {
                let robust = 1 + acc
                    .small
                    .iter()
                    .filter(|c| c.dir == dir && c.shift == d_star && c.te > c.thr)
                    .count();
                let (driver, target) = if dir == 0 { (a, b) } else { (b, a) };
                let state = if excess <= 0.0 {
                    "silent"
                } else if acc.fam_any && te <= acc.fam {
                    "family bound"
                } else if robust < 3 {
                    "h bound"
                } else {
                    "arrow"
                };
                let parts = format!(
                    "matrix sheet {}→{} lag {}d n {} te {:.3} thr {:.3} fam {:.3} p {:.3} M {} h {}/3 state {}",
                    driver, target, d_star, n_star, te, thr, acc.fam, p_hat, acc.m, robust, state
                );
                (state, parts, p_hat, acc.m)
            } else {
                let parts = format!("matrix sheet {}→{} state no statement", a, b);
                ("no statement", parts, p_hat, acc.m)
            }
        };
        self.say(sheet_parts);
        self.line.pairs += 1;
        self.line.expected += p_hat * m_count as f64;
        match state {
            "arrow" => self.line.arrows += 1,
            "family bound" => self.line.family += 1,
            "h bound" => self.line.h_bound += 1,
            "silent" => self.line.silent += 1,
            _ => self.line.absent += 1,
        }
    }

    pub fn matrix_line(&mut self) {
        let l = &self.line;
        self.say(format!(
            "matrix line pairs {} arrows {} family {} hbound {} silent {} absent {} expected {:.2} state measured",
            l.pairs, l.arrows, l.family, l.h_bound, l.silent, l.absent, l.expected
        ));
    }

    pub fn tick(
        &mut self,
        field: Option<Arc<crate::archivar::Buffer>>,
        presence: [f64; 3],
        t_presence: f64,
        ring_gen: u64,
    ) {
        self.rng = ring_gen;
        let batches: Vec<(
            crate::archivar::Frame,
            Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
        )> = match &self.rx {
            Some(rx) => {
                let mut out = Vec::new();
                while let Ok(batch) = rx.try_recv() {
                    out.push(batch);
                }
                out
            }
            None => Vec::new(),
        };
        for (frame, batch) in batches {
            self.record(&frame, batch);
        }
        if self
            .last_state_save
            .map_or(true, |i| i.elapsed().as_secs_f64() > 120.0)
        {
            self.last_state_save = Some(std::time::Instant::now());
            self.save_state();
        }
        if self.te_map.is_some() {
            self.collect();
        }
        let moved = self.last_presence.map_or(true, |p| {
            let d2 = (p[0] - presence[0]).powi(2)
                + (p[1] - presence[1]).powi(2)
                + (p[2] - presence[2]).powi(2);
            d2 > MATRIX_PAD_M * MATRIX_PAD_M
        });
        let fresh = match (&field, &self.last_field) {
            (Some(f), Some(l)) => !Arc::ptr_eq(f, l),
            (Some(_), None) => true,
            _ => false,
        };
        let rebuild_due = (moved || fresh)
            && self
                .last_rebuild
                .map_or(true, |i| i.elapsed().as_secs_f64() > 30.0);
        if rebuild_due {
            self.last_rebuild = Some(std::time::Instant::now());
            self.last_presence = Some(presence);
            if let Some(f) = field.clone() {
                self.last_field = Some(f);
            }
            self.rebuild(presence, t_presence);
        }
        if !self.due || self.te_map.is_some() {
            return;
        }
        if self.present.len() < 2 {
            self.say("matrix line pairs 0 state no pairs".to_string());
            self.due = false;
            return;
        }
        let pairs = all_pairs(&self.present);
        let mut budget = 2048usize;
        while self.due && self.te_map.is_none() && budget > 0 {
            budget -= 1;
            let mut target: Option<(String, String)> = None;
            for (a, b) in &pairs {
                let done = self.results.get(&pair_key(a, b)).map_or(false, |r| r.done);
                if !done {
                    target = Some((a.clone(), b.clone()));
                    break;
                }
            }
            let Some((a, b)) = target else {
                self.matrix_line();
                self.due = false;
                break;
            };
            let key = pair_key(&a, &b);
            let cells = self.results.get(&key).map_or(0, |r| r.cells);
            if cells < MATRIX_CELLS_PER_ROUND {
                self.results
                    .entry(key)
                    .or_insert_with(PairResult::fresh)
                    .cells = cells + 1;
                self.probe(&a, &b, cells);
            } else {
                self.sheet(&a, &b);
                if let Some(r) = self.results.get_mut(&key) {
                    r.done = true;
                }
            }
        }
    }
}

pub fn all_pairs(present: &[String]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for i in 0..present.len() {
        for j in (i + 1)..present.len() {
            out.push((present[i].clone(), present[j].clone()));
        }
    }
    out
}

pub fn pair_key(a: &str, b: &str) -> String {
    format!("{}→{}", a, b)
}

pub fn matrix_cell_desc(cell_idx: usize) -> (u8, u8, i64) {
    let scale_idx = (cell_idx / MATRIX_CELLS_PER_SCALE) as u8;
    let rest = cell_idx % MATRIX_CELLS_PER_SCALE;
    let dir = (rest / MATRIX_SHIFT_COUNT) as u8;
    let shift_days = rest as i64 % MATRIX_SHIFT_COUNT as i64 - MATRIX_SHIFT_MAX_DAYS;
    (scale_idx, dir, shift_days)
}

pub fn shift_pair(
    driver: &[(f64, f32)],
    target: &[(f64, f32)],
    shift_s: f64,
) -> (Vec<f32>, Vec<f32>) {
    let mut ys = Vec::new();
    let mut xs = Vec::new();
    for &(tb, tv) in target {
        let db = tb - shift_s;
        let Ok(i) = driver.binary_search_by(|(t, _)| t.total_cmp(&db)) else {
            continue;
        };
        ys.push(driver[i].1);
        xs.push(tv);
    }
    (ys, xs)
}

pub fn insert_bin(ring: &mut Vec<(f64, f32)>, epoch: f64, val: f32) {
    match ring.binary_search_by(|(t, _)| t.total_cmp(&epoch)) {
        Ok(i) => ring[i] = (epoch, val),
        Err(i) => ring.insert(i, (epoch, val)),
    }
    while ring.len() > MATRIX_RING_MAX {
        ring.remove(0);
    }
}
