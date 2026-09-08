use super::*;

pub const SCALAR_GRID_TOTAL: usize = 286;

pub const SCALAR_OUT_FLOATS: usize = SCALAR_GRID_TOTAL * 2;

pub const SCALAR_OUT_BYTES: u64 = (SCALAR_OUT_FLOATS * 4) as u64;

pub struct ScalarTeGpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    series_buf: wgpu::Buffer,
    param_buf: wgpu::Buffer,
    out_buf: wgpu::Buffer,
    read_buf: wgpu::Buffer,
    bind: wgpu::BindGroup,
    pipe: wgpu::ComputePipeline,
    n_lags: u32,
}

impl ScalarTeGpu {
    pub fn new(n_lags: usize) -> Option<ScalarTeGpu> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))?;
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .ok()?;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(SCALAR_TE_WGSL.into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipe_layout),
            module: &module,
            entry_point: Some("scalar_te_compute"),
            compilation_options: Default::default(),
            cache: None,
        });
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
            size: SCALAR_OUT_BYTES,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: SCALAR_OUT_BYTES,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
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
        Some(ScalarTeGpu {
            device,
            queue,
            series_buf,
            param_buf,
            out_buf,
            read_buf,
            bind,
            pipe,
            n_lags: n_lags as u32,
        })
    }

    pub fn n_lags(&self) -> usize {
        self.n_lags as usize
    }

    pub fn run(&mut self, xs: &[f32], ys: &[f32], surrogates: &[Vec<f32>]) -> Vec<f32> {
        let m = xs.len().min(ys.len()).min(TE_SERIES_STRIDE);
        let mut data = vec![0f32; 12 * TE_SERIES_STRIDE];
        data[0..m].copy_from_slice(&xs[..m]);
        data[TE_SERIES_STRIDE..TE_SERIES_STRIDE + m].copy_from_slice(&ys[..m]);
        for (s, surr) in surrogates.iter().enumerate() {
            let off = (2 + s) * TE_SERIES_STRIDE;
            data[off..off + m].copy_from_slice(&surr[..m]);
        }
        self.queue
            .write_buffer(&self.series_buf, 0, &le_bytes_f32(&data));
        let param = [m as u32, self.n_lags, 0, 0];
        let mut pb = [0u8; 16];
        for (i, x) in param.iter().enumerate() {
            pb[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        self.queue.write_buffer(&self.param_buf, 0, &pb);
        let mut enc = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(&self.pipe);
            pass.set_bind_group(0, &self.bind, &[]);
            pass.dispatch_workgroups(SCALAR_GRID_TOTAL.div_ceil(64) as u32, 1, 1);
        }
        enc.copy_buffer_to_buffer(&self.out_buf, 0, &self.read_buf, 0, SCALAR_OUT_BYTES);
        self.queue.submit(std::iter::once(enc.finish()));
        let mapped = Arc::new(AtomicBool::new(false));
        let m2 = mapped.clone();
        let slice = self.read_buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, move |r| {
            m2.store(r.is_ok(), Ordering::SeqCst);
        });
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            self.device.poll(wgpu::Maintain::Poll);
        }
        if !mapped.load(Ordering::SeqCst) {
            return Vec::new();
        }
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity(SCALAR_OUT_FLOATS);
        for k in 0..SCALAR_OUT_FLOATS {
            let mut b = [0u8; 4];
            b.copy_from_slice(&data[k * 4..k * 4 + 4]);
            out.push(f32::from_le_bytes(b));
        }
        drop(data);
        self.read_buf.unmap();
        out
    }
}
