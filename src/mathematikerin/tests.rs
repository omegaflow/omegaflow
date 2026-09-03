use super::*;

#[test]
fn field_wgsl_validates_offline() {
    let module = match naga::front::wgsl::parse_str(FIELD_WGSL) {
        Ok(m) => m,
        Err(e) => panic!("wgsl parse: {}", e.emit_to_string(FIELD_WGSL)),
    };
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    );
    if let Err(e) = validator.validate(&module) {
        panic!("wgsl validate: {}", e.emit_to_string(FIELD_WGSL));
    }
}

#[test]
fn te_wgsl_validates_offline() {
    let module = match naga::front::wgsl::parse_str(TE_WGSL) {
        Ok(m) => m,
        Err(e) => panic!("wgsl parse: {}", e.emit_to_string(TE_WGSL)),
    };
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    );
    if let Err(e) = validator.validate(&module) {
        panic!("wgsl validate: {}", e.emit_to_string(TE_WGSL));
    }
}

#[test]
fn te_gpu_crosscheck_against_cpu_reference() {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
    let adapter = match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::None,
        compatible_surface: None,
        force_fallback_adapter: false,
    })) {
        Some(a) => a,
        None => {
            eprintln!("adapter request returned void — crosscheck skipped");
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
        module: &module,
        entry_point: Some("te_compute"),
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
    let te_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &te_layout,
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
    let n = 200usize;
    let mut x = vec![0f32; n];
    let mut y = vec![0f32; n];
    for t in 0..n {
        y[t] = (t as f32 * 0.5).sin();
    }
    for t in 0..n - 1 {
        x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
    }
    let seed = 42u64;
    let mut data = vec![0f32; 12 * TE_SERIES_STRIDE];
    data[0..n].copy_from_slice(&x);
    data[TE_SERIES_STRIDE..TE_SERIES_STRIDE + n].copy_from_slice(&y);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for s in 0..10 {
        let surr = crate::te::phase_randomized_surrogate(&y, &mut rng);
        let off = (2 + s) * TE_SERIES_STRIDE;
        data[off..off + n].copy_from_slice(&surr);
    }
    queue.write_buffer(&series_buf, 0, &le_bytes_f32(&data));
    let max_lag = (n as f64 / Φ) as u32;
    let param = [n as u32, max_lag, 1.0f32.to_bits(), 0];
    let mut pb = [0u8; 16];
    for (i, p) in param.iter().enumerate() {
        pb[i * 4..i * 4 + 4].copy_from_slice(&p.to_le_bytes());
    }
    queue.write_buffer(&param_buf, 0, &pb);
    let start = std::time::Instant::now();
    let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass.set_pipeline(&te_pipe);
        pass.set_bind_group(0, &te_bind, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    enc.copy_buffer_to_buffer(&out_buf, 0, &read_buf, 0, 288);
    queue.submit(std::iter::once(enc.finish()));
    let mapped = Arc::new(AtomicBool::new(false));
    let m2 = mapped.clone();
    let slice = read_buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, move |r| {
        m2.store(r.is_ok(), Ordering::SeqCst);
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
        device.poll(wgpu::Maintain::Poll);
    }
    assert!(
        mapped.load(Ordering::SeqCst),
        "te gpu readback returned void"
    );
    let elapsed = start.elapsed();
    let mapped_data = slice.get_mapped_range();
    let mut verdict = [0f32; 72];
    for k in 0..72 {
        let mut b = [0u8; 4];
        b.copy_from_slice(&mapped_data[k * 4..k * 4 + 4]);
        verdict[k] = f32::from_le_bytes(b);
    }
    drop(mapped_data);
    read_buf.unmap();
    let gpu = crate::te::topological_verdict_from_gpu(&verdict);
    let cpu = crate::te::topological_te_phase(&x, &y, 3, 3, seed);
    eprintln!("te crosscheck elapsed {:?}", elapsed);
    eprintln!(
        "gpu: {:?}",
        gpu.as_ref().map(|v| (
            v.tau_x,
            v.tau_y,
            v.te,
            v.threshold,
            v.surrogates_used,
            v.pe_x,
            v.pe_y
        ))
    );
    eprintln!(
        "cpu: {:?}",
        cpu.as_ref().map(|v| (
            v.tau_x,
            v.tau_y,
            v.te,
            v.threshold,
            v.surrogates_used,
            v.pe_x,
            v.pe_y
        ))
    );
    let (gpu_v, cpu_v) = match (gpu, cpu) {
        (Some(g), Some(c)) => (g, c),
        (None, None) => return,
        (g, c) => {
            panic!(
                "te crosscheck verdict divergence: gpu valid = {}, cpu valid = {}",
                g.is_some(),
                c.is_some()
            );
        }
    };
    assert_eq!(gpu_v.tau_x, cpu_v.tau_x, "tau_x diverges");
    assert_eq!(gpu_v.tau_y, cpu_v.tau_y, "tau_y diverges");
    assert!(
        gpu_v.surrogates_used >= 2 && cpu_v.surrogates_used >= 2,
        "surrogates_used below two: gpu {} cpu {}",
        gpu_v.surrogates_used,
        cpu_v.surrogates_used
    );
    let te_rel = ((gpu_v.te - cpu_v.te) / cpu_v.te.abs()).abs();
    assert!(
        te_rel < 0.1,
        "te diverges: gpu {} cpu {} rel {}",
        gpu_v.te,
        cpu_v.te,
        te_rel
    );
    match (gpu_v.pe_x, cpu_v.pe_x) {
        (Some(g), Some(c)) => {
            assert!((g - c).abs() < 1e-3, "pe_x diverges: gpu {} cpu {}", g, c)
        }
        (None, None) => {}
        (g, c) => panic!("pe_x presence diverges: gpu {:?} cpu {:?}", g, c),
    }
    match (gpu_v.pe_y, cpu_v.pe_y) {
        (Some(g), Some(c)) => {
            assert!((g - c).abs() < 1e-3, "pe_y diverges: gpu {} cpu {}", g, c)
        }
        (None, None) => {}
        (g, c) => panic!("pe_y presence diverges: gpu {:?} cpu {:?}", g, c),
    }
    for h_scale in [0.5f32, 2.0f32] {
        let param = [n as u32, max_lag, h_scale.to_bits(), 0];
        let mut pb = [0u8; 16];
        for (i, p) in param.iter().enumerate() {
            pb[i * 4..i * 4 + 4].copy_from_slice(&p.to_le_bytes());
        }
        queue.write_buffer(&param_buf, 0, &pb);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(&te_pipe);
            pass.set_bind_group(0, &te_bind, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        enc.copy_buffer_to_buffer(&out_buf, 0, &read_buf, 0, 288);
        queue.submit(std::iter::once(enc.finish()));
        let mapped = Arc::new(AtomicBool::new(false));
        let m2 = mapped.clone();
        let slice = read_buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, move |r| {
            m2.store(r.is_ok(), Ordering::SeqCst);
        });
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            device.poll(wgpu::Maintain::Poll);
        }
        assert!(mapped.load(Ordering::SeqCst), "te scaled readback void");
        let mapped_data = slice.get_mapped_range();
        let mut verdict = [0f32; 72];
        for k in 0..72 {
            let mut b = [0u8; 4];
            b.copy_from_slice(&mapped_data[k * 4..k * 4 + 4]);
            verdict[k] = f32::from_le_bytes(b);
        }
        drop(mapped_data);
        read_buf.unmap();
        let scaled = crate::te::topological_verdict_from_gpu(&verdict)
            .unwrap_or_else(|| panic!("te verdict invalid at bandwidth scale {}", h_scale));
        assert!(
            (scaled.te - gpu_v.te).abs() > 1e-4,
            "te unchanged at bandwidth scale {}: {}",
            h_scale,
            scaled.te
        );
    }
}

#[test]
fn golden_pack_slots_against_wgsl_access() {
    let presence = [1.0e3, 2.0e3, 3.0e3];
    let r: Record = (
        7001.0, 7002.0, 7003.0, 7004.0, 7005.0, 7006.0, 7007.0, 7008.0, 7009.0, 7010.0, 7011.0,
        7012.0, 7013.0, 7014.0, 7015.0, 7016.0, 7017.0, 7018.0, 7019.0, 7020.0, 7021.0, 7022.0,
        7023.0, 7024.0, 7025.0, 7026.0,
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
    assert_eq!(m[10], 7022.0);
    assert_eq!(m[11], 7023.0);
    assert_eq!(m[12], 7024.0);
    assert_eq!(m[13], 7025.0);
    assert_eq!(m[14], 7026.0);
    assert_eq!(m[15], 0.0);
}

#[test]
fn force_ref_medians_routes_forces_and_honors_zero() {
    let mut field = vec![0.0f32; 48];
    field[3] = 4.0;
    field[15] = 4.0;
    field[27] = -2.0;
    field[30] = 2.0;
    field[39] = 0.0;
    field[42] = 8.0;
    let meds = force_ref_medians(&field, &[0.0; 48]);
    assert_eq!(meds[0].unwrap(), 4.0);
    assert_eq!(meds[1], None);
    assert_eq!(meds[2].unwrap(), 2.0);
    for ft in 3..9 {
        assert_eq!(meds[ft], None);
    }
}

#[test]
fn force_ref_medians_holds_reference_on_absence() {
    let mut app = OmegaLoop {
        force_ref: [7.0; 9],
        ..OmegaLoop::new(
            mpsc::channel().1,
            mpsc::sync_channel(1).0,
            mpsc::sync_channel(2).1,
            Arc::new(Mutex::new(None)),
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicBool::new(false)),
            mpsc::channel().0,
            mpsc::channel().0,
            mpsc::channel().1,
            mpsc::channel().1,
            Arc::new(RwLock::new(PresenceState::rest())),
            Arc::new(RwLock::new(DiodeState {
                force_ref: [0.0; 9],
                expose_offset: EXPOSE_OFFSET_BASE,
            })),
        )
    };
    app.packed_field = vec![0.0; 12];
    app.packed_meta = vec![0.0; 12];
    for _ in 0..64 {
        app.relax_force_refs();
    }
    for ft in 0..9 {
        assert_eq!(app.force_ref[ft], 7.0);
    }
}

#[test]
fn force_ref_medians_skips_length_annotations() {
    let mut field = vec![0.0f32; 24];
    let mut meta = vec![0.0f32; 32];
    field[3] = 2.0f32.powi(20);
    field[6] = 1.0;
    meta[0] = 2.0f32.powi(20);
    field[15] = 2.0f32.powi(45);
    field[18] = 1.0;
    let meds = force_ref_medians(&field, &meta);
    assert_eq!(meds[1].unwrap(), 2.0f32.powi(45));
}

#[test]
fn force_ref_snaps_on_first_sight() {
    let mut app = OmegaLoop {
        force_ref: [0.0; 9],
        ..OmegaLoop::new(
            mpsc::channel().1,
            mpsc::sync_channel(1).0,
            mpsc::sync_channel(2).1,
            Arc::new(Mutex::new(None)),
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicBool::new(false)),
            mpsc::channel().0,
            mpsc::channel().0,
            mpsc::channel().1,
            mpsc::channel().1,
            Arc::new(RwLock::new(PresenceState::rest())),
            Arc::new(RwLock::new(DiodeState {
                force_ref: [0.0; 9],
                expose_offset: EXPOSE_OFFSET_BASE,
            })),
        )
    };
    app.packed_field = vec![0.0; 12];
    app.packed_field[3] = 8.0;
    app.packed_meta = vec![0.0; 12];
    app.relax_force_refs();
    assert_eq!(app.force_ref[0], 8.0);
}

#[test]
fn aberration_shifts_toward_apex_and_stays_unit() {
    fn aberr(u: [f64; 3], beta: [f64; 3]) -> [f64; 3] {
        let b2 = beta[0] * beta[0] + beta[1] * beta[1] + beta[2] * beta[2];
        let gamma = 1.0 / (1.0 - b2).sqrt();
        let ud = u[0] * beta[0] + u[1] * beta[1] + u[2] * beta[2];
        let inv = 1.0 / (1.0 + ud);
        let k = gamma / (gamma + 1.0) * ud;
        [
            (u[0] / gamma + beta[0] + k * beta[0]) * inv,
            (u[1] / gamma + beta[1] + k * beta[1]) * inv,
            (u[2] / gamma + beta[2] + k * beta[2]) * inv,
        ]
    }
    let beta = [0.5, 0.0, 0.0];
    let ahead = aberr([1.0, 0.0, 0.0], beta);
    assert!((ahead[0] - 1.0).abs() < 1e-9);
    assert!(ahead[1].abs() < 1e-9 && ahead[2].abs() < 1e-9);
    let side = aberr([0.0, 1.0, 0.0], beta);
    assert!(side[0] > 0.0, "transverse star shifts toward the apex");
    let n = (side[0] * side[0] + side[1] * side[1] + side[2] * side[2]).sqrt();
    assert!((n - 1.0).abs() < 1e-9);
    let rest = aberr([0.3, 0.4, 0.916515139], [0.0, 0.0, 0.0]);
    assert!((rest[0] - 0.3).abs() < 1e-9);
    assert!((rest[1] - 0.4).abs() < 1e-9);
}
