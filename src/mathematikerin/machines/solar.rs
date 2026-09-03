use super::*;

pub const SOLAR_FAST_GRID: u32 = 60;

pub const SOLAR_COARSE_GRID: u32 = 43200;

pub const SOLAR_GOES_SYNC_S: f64 = 149_597_870_700.0 / 299_792_458.0;

pub const SOLAR_L1_SUN_M: f64 = 1.481e11;

pub const SOLAR_RING_MAX: usize = 256;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolarChannel {
    Xray,
    Euv304,
    Euv284,
    BzGsm,
    Density,
    F107,
}

impl SolarChannel {
    pub fn name(self) -> &'static str {
        match self {
            SolarChannel::Xray => "xray",
            SolarChannel::Euv304 => "euv304",
            SolarChannel::Euv284 => "euv284",
            SolarChannel::BzGsm => "bz",
            SolarChannel::Density => "density",
            SolarChannel::F107 => "f107",
        }
    }

    pub fn idx(self) -> usize {
        match self {
            SolarChannel::Xray => 0,
            SolarChannel::Euv304 => 1,
            SolarChannel::Euv284 => 2,
            SolarChannel::BzGsm => 3,
            SolarChannel::Density => 4,
            SolarChannel::F107 => 5,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SolarCell {
    pub grid: u32,
    pub channel: SolarChannel,
    pub bin: u64,
    pub value: f32,
}

pub fn solar_find_block<'a>(
    sources: &'a [SourceConfig],
    field_name: &str,
) -> Option<&'a SourceConfig> {
    sources.iter().find(|s| {
        s.extracts.iter().any(|e| match e {
            Extract::Field(fc)
            | Extract::First(fc, _)
            | Extract::Last(fc, _)
            | Extract::Path(fc) => fc.name == field_name,
            _ => false,
        })
    })
}

pub fn solar_series(
    block: &SourceConfig,
    body: &str,
    field_name: &str,
    lsk: &LeapSeconds,
) -> Vec<(f64, f64)> {
    let mut series_src = block.clone();
    series_src.extracts.retain(|e| {
        matches!(
            e,
            Extract::First(fc, _) | Extract::Last(fc, _) | Extract::Path(fc)
                if fc.name == field_name
        )
    });
    extract_series(&series_src, body, lsk)
}

pub fn solar_l1_sync(
    series: &[(f64, f64)],
    wind: &[(f64, f64)],
    tolerance_s: f64,
) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for &(t, v) in series {
        let mut best: Option<(f64, f64)> = None;
        for &(tw, vw) in wind {
            let dt = (tw - t).abs();
            if dt <= tolerance_s && best.map_or(true, |(b, _)| dt < b) {
                best = Some((dt, vw));
            }
        }
        let Some((_, v_ms)) = best else {
            continue;
        };
        if v_ms <= 0.0 {
            continue;
        }
        out.push((t - SOLAR_L1_SUN_M / v_ms, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

pub fn solar_send_bins(
    series: &[(f64, f64)],
    grid: u32,
    channel: SolarChannel,
    last_sent: &mut std::collections::HashMap<(u32, SolarChannel), u64>,
    tx: &mpsc::Sender<SolarCell>,
) {
    let dt = grid as f64;
    let mut sorted: Vec<&(f64, f64)> = series
        .iter()
        .filter(|&&(t, _)| t.is_finite() && t > 0.0)
        .collect();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let gate = last_sent.get(&(grid, channel)).copied().unwrap_or(0);
    let mut cells: Vec<SolarCell> = Vec::new();
    let mut cur_bin: i64 = i64::MIN;
    let mut sum: f64 = 0.0;
    let mut cnt: u32 = 0;
    for &&(t, v) in &sorted {
        let bin = (t / dt).floor() as i64;
        if bin != cur_bin {
            if cur_bin != i64::MIN && cnt > 0 {
                let b = cur_bin as u64;
                if b > gate {
                    cells.push(SolarCell {
                        grid,
                        channel,
                        bin: b,
                        value: (sum / cnt as f64) as f32,
                    });
                }
            }
            cur_bin = bin;
            sum = 0.0;
            cnt = 0;
        }
        sum += v;
        cnt += 1;
    }
    if cur_bin != i64::MIN && cnt > 0 {
        let b = cur_bin as u64;
        if b > gate {
            cells.push(SolarCell {
                grid,
                channel,
                bin: b,
                value: (sum / cnt as f64) as f32,
            });
        }
    }
    if gate == 0 && cells.len() > SOLAR_RING_MAX {
        let drop = cells.len() - SOLAR_RING_MAX;
        cells.drain(..drop);
    }
    for cell in &cells {
        let _ = tx.send(*cell);
    }
    if let Some(last) = cells.last() {
        last_sent.insert((grid, channel), last.bin);
    }
}

pub fn solar_harvest(
    tx: mpsc::Sender<SolarCell>,
    sources: Vec<SourceConfig>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
) {
    let mut last_sent: std::collections::HashMap<(u32, SolarChannel), u64> =
        std::collections::HashMap::new();
    loop {
        let lock = match time.lock() {
            Ok(l) => l,
            Err(_) => break,
        };
        let Some(lsk) = lock.as_ref() else {
            drop(lock);
            thread::sleep(std::time::Duration::from_secs(SOLAR_FAST_GRID as u64));
            continue;
        };
        let goes_shift = |series: Vec<(f64, f64)>| -> Vec<(f64, f64)> {
            series
                .into_iter()
                .map(|(t, v)| (t - SOLAR_GOES_SYNC_S, v))
                .collect()
        };
        if let Some(block) = solar_find_block(&sources, "noaa_goes_xray_flux_w_m2") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let raw = solar_series(block, &body, "noaa_goes_xray_flux_w_m2", lsk);
                let shifted = goes_shift(raw);
                solar_send_bins(
                    &shifted,
                    SOLAR_FAST_GRID,
                    SolarChannel::Xray,
                    &mut last_sent,
                    &tx,
                );
                solar_send_bins(
                    &shifted,
                    SOLAR_COARSE_GRID,
                    SolarChannel::Xray,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        if let Some(block) = solar_find_block(&sources, "solar_euv_flux_304_wm2") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                for (field, channel) in [
                    ("solar_euv_flux_304_wm2", SolarChannel::Euv304),
                    ("solar_euv_flux_284_wm2", SolarChannel::Euv284),
                ] {
                    let raw = solar_series(block, &body, field, lsk);
                    let shifted = goes_shift(raw);
                    solar_send_bins(&shifted, SOLAR_FAST_GRID, channel, &mut last_sent, &tx);
                    solar_send_bins(&shifted, SOLAR_COARSE_GRID, channel, &mut last_sent, &tx);
                }
            }
        }
        if let Some(block) = solar_find_block(&sources, "solar_f107_flux_sfu") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let raw = solar_series(block, &body, "solar_f107_flux_sfu", lsk);
                let shifted = goes_shift(raw);
                solar_send_bins(
                    &shifted,
                    SOLAR_COARSE_GRID,
                    SolarChannel::F107,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        let mut wind: Vec<(f64, f64)> = Vec::new();
        if let Some(block) = solar_find_block(&sources, "solar_wind_speed_km_s") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                wind = solar_series(block, &body, "solar_wind_speed_km_s", lsk);
                let dens = solar_series(block, &body, "solar_wind_density_cm3", lsk);
                let synced = solar_l1_sync(&dens, &wind, 60.0);
                solar_send_bins(
                    &synced,
                    SOLAR_FAST_GRID,
                    SolarChannel::Density,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        if let Some(block) = solar_find_block(&sources, "magnetosphere_imf_bz_nt") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let bz = solar_series(block, &body, "magnetosphere_imf_bz_nt", lsk);
                let synced = solar_l1_sync(&bz, &wind, 60.0);
                solar_send_bins(
                    &synced,
                    SOLAR_FAST_GRID,
                    SolarChannel::BzGsm,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        drop(lock);
        thread::sleep(std::time::Duration::from_secs(SOLAR_FAST_GRID as u64));
    }
}

pub const SOLAR_N_GATE: usize = 30;

pub const SOLAR_FAST_PAIRS: [(SolarChannel, SolarChannel); 20] = [
    (SolarChannel::Xray, SolarChannel::Euv304),
    (SolarChannel::Xray, SolarChannel::Euv284),
    (SolarChannel::Xray, SolarChannel::BzGsm),
    (SolarChannel::Xray, SolarChannel::Density),
    (SolarChannel::Euv304, SolarChannel::Xray),
    (SolarChannel::Euv304, SolarChannel::Euv284),
    (SolarChannel::Euv304, SolarChannel::BzGsm),
    (SolarChannel::Euv304, SolarChannel::Density),
    (SolarChannel::Euv284, SolarChannel::Xray),
    (SolarChannel::Euv284, SolarChannel::Euv304),
    (SolarChannel::Euv284, SolarChannel::BzGsm),
    (SolarChannel::Euv284, SolarChannel::Density),
    (SolarChannel::BzGsm, SolarChannel::Xray),
    (SolarChannel::BzGsm, SolarChannel::Euv304),
    (SolarChannel::BzGsm, SolarChannel::Euv284),
    (SolarChannel::BzGsm, SolarChannel::Density),
    (SolarChannel::Density, SolarChannel::Xray),
    (SolarChannel::Density, SolarChannel::Euv304),
    (SolarChannel::Density, SolarChannel::Euv284),
    (SolarChannel::Density, SolarChannel::BzGsm),
];

pub const SOLAR_COARSE_PAIRS: [(SolarChannel, SolarChannel); 6] = [
    (SolarChannel::F107, SolarChannel::Xray),
    (SolarChannel::F107, SolarChannel::Euv304),
    (SolarChannel::F107, SolarChannel::Euv284),
    (SolarChannel::Xray, SolarChannel::F107),
    (SolarChannel::Euv304, SolarChannel::F107),
    (SolarChannel::Euv284, SolarChannel::F107),
];

pub struct SolarMachine {
    pub rx: mpsc::Receiver<SolarCell>,
    pub rings: [[Vec<(u64, f32)>; 6]; 2],
    pub fast_rotor: usize,
    pub coarse_rotor: usize,
    pub fast_last_bin: u64,
    pub coarse_last_bin: u64,
    pub fast_due: bool,
    pub coarse_due: bool,
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub te_pipe: Option<wgpu::ComputePipeline>,
    pub te_bind: Option<wgpu::BindGroup>,
    pub te_series_buf: Option<wgpu::Buffer>,
    pub te_param_buf: Option<wgpu::Buffer>,
    pub te_out_buf: Option<wgpu::Buffer>,
    pub te_read_buf: Option<wgpu::Buffer>,
    pub te_map: Option<Arc<AtomicBool>>,
    pub pending: Option<(String, usize)>,
    pub named: String,
    pub rng: u64,
}

impl SolarMachine {
    pub fn new(rx: mpsc::Receiver<SolarCell>) -> Self {
        SolarMachine {
            rx,
            rings: std::array::from_fn(|_| std::array::from_fn(|_| Vec::new())),
            fast_rotor: 0,
            coarse_rotor: 0,
            fast_last_bin: 0,
            coarse_last_bin: 0,
            fast_due: false,
            coarse_due: false,
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
            named: String::new(),
            rng: 0,
        }
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

    pub fn say(
        &mut self,
        label: &str,
        n: usize,
        verdict: Option<&crate::te::TopologicalVerdict>,
        state: &str,
    ) {
        let line = match verdict {
            Some(v) => format!(
                "solar te {} n {} te {:.3} thr {:.3} tau {}:{} pe {}:{} state {}",
                label,
                n,
                v.te,
                v.threshold,
                v.tau_x,
                v.tau_y,
                v.pe_x
                    .map(|p| format!("{:.2}", p))
                    .unwrap_or_else(|| "-".to_string()),
                v.pe_y
                    .map(|p| format!("{:.2}", p))
                    .unwrap_or_else(|| "-".to_string()),
                state
            ),
            None => format!("solar te {} n {} state {}", label, n, state),
        };
        if self.named != line {
            eprintln!("{}", line);
            self.named = line;
        }
    }

    pub fn pair(&self, grid: usize, from: SolarChannel, to: SolarChannel) -> (Vec<f32>, Vec<f32>) {
        let ra = &self.rings[grid][from.idx()];
        let rb = &self.rings[grid][to.idx()];
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        let (mut i, mut j) = (0usize, 0usize);
        while i < ra.len() && j < rb.len() {
            match ra[i].0.cmp(&rb[j].0) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Equal => {
                    ys.push(ra[i].1);
                    xs.push(rb[j].1);
                    i += 1;
                    j += 1;
                }
            }
        }
        if xs.len() > SOLAR_RING_MAX {
            let drop = xs.len() - SOLAR_RING_MAX;
            xs.drain(..drop);
            ys.drain(..drop);
        }
        (xs, ys)
    }

    pub fn collect(&mut self) {
        let Some(prev) = self.te_map.take() else {
            return;
        };
        if !prev.load(Ordering::SeqCst) {
            self.te_map = Some(prev);
            return;
        }
        let Some(read_buf) = self.te_read_buf.as_ref() else {
            return;
        };
        let verdict = te_read_verdict(read_buf);
        let v = crate::te::topological_verdict_from_gpu(&verdict);
        if let Some((l, n)) = self.pending.take() {
            match v {
                Some(vv) => {
                    let state = if vv.te > vv.threshold {
                        "arrow"
                    } else {
                        "silent"
                    };
                    self.say(&l, n, Some(&vv), state);
                }
                None => self.say(&l, n, None, te_absence_word(&verdict)),
            }
        }
    }

    pub fn te_probe(&mut self, label: &str, xs: &[f32], ys: &[f32], m: usize) {
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
        let mut data = vec![0f32; 12 * TE_SERIES_STRIDE];
        data[0..m].copy_from_slice(xs);
        data[TE_SERIES_STRIDE..TE_SERIES_STRIDE + m].copy_from_slice(ys);
        let mut rng = self.rng.wrapping_add(0x9e3779b97f4a7c15);
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
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(250);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            device.poll(wgpu::Maintain::Poll);
        }
        if !mapped.load(Ordering::SeqCst) {
            self.te_map = Some(mapped);
            self.pending = Some((label.to_string(), m));
            self.say(label, m, None, "readback pending");
            return;
        }
        let verdict = te_read_verdict(read_buf);
        match crate::te::topological_verdict_from_gpu(&verdict) {
            Some(v) => {
                let state = if v.te > v.threshold {
                    "arrow"
                } else {
                    "silent"
                };
                self.say(label, m, Some(&v), state);
            }
            None => self.say(label, m, None, te_absence_word(&verdict)),
        }
    }

    pub fn dispatch(&mut self, grid: usize, from: SolarChannel, to: SolarChannel) {
        let label = format!("{}→{}", from.name(), to.name());
        let (xs, ys) = self.pair(grid, from, to);
        if xs.len() < SOLAR_N_GATE {
            self.say(&label, xs.len(), None, "no statement");
            return;
        }
        self.te_probe(&label, &xs, &ys, xs.len());
    }

    pub fn tick(&mut self, ring_gen: u64) {
        self.rng = ring_gen;
        let mut max_fast_bin = self.fast_last_bin;
        let mut max_coarse_bin = self.coarse_last_bin;
        while let Ok(cell) = self.rx.try_recv() {
            let grid = match cell.grid {
                SOLAR_FAST_GRID => 0,
                SOLAR_COARSE_GRID => 1,
                _ => continue,
            };
            let ring = &mut self.rings[grid][cell.channel.idx()];
            match ring.binary_search_by_key(&cell.bin, |&(b, _)| b) {
                Ok(i) => ring[i] = (cell.bin, cell.value),
                Err(i) => ring.insert(i, (cell.bin, cell.value)),
            }
            while ring.len() > SOLAR_RING_MAX {
                ring.remove(0);
            }
            if grid == 0 {
                max_fast_bin = max_fast_bin.max(cell.bin);
            } else {
                max_coarse_bin = max_coarse_bin.max(cell.bin);
            }
        }
        if max_fast_bin > self.fast_last_bin {
            self.fast_last_bin = max_fast_bin;
            self.fast_due = true;
        }
        if max_coarse_bin > self.coarse_last_bin {
            self.coarse_last_bin = max_coarse_bin;
            self.coarse_due = true;
        }
        if self.te_map.is_some() {
            self.collect();
        }
        if self.fast_due && self.te_map.is_none() {
            self.fast_due = false;
            let (from, to) = SOLAR_FAST_PAIRS[self.fast_rotor % SOLAR_FAST_PAIRS.len()];
            self.fast_rotor += 1;
            self.dispatch(0, from, to);
        }
        if self.coarse_due && self.te_map.is_none() {
            self.coarse_due = false;
            let (from, to) = SOLAR_COARSE_PAIRS[self.coarse_rotor % SOLAR_COARSE_PAIRS.len()];
            self.coarse_rotor += 1;
            self.dispatch(1, from, to);
        }
    }
}
