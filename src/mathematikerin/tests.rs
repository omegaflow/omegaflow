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
fn scalar_te_wgsl_validates_offline() {
    let module = match naga::front::wgsl::parse_str(SCALAR_TE_WGSL) {
        Ok(m) => m,
        Err(e) => panic!("wgsl parse: {}", e.emit_to_string(SCALAR_TE_WGSL)),
    };
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    );
    if let Err(e) = validator.validate(&module) {
        panic!("wgsl validate: {}", e.emit_to_string(SCALAR_TE_WGSL));
    }
}

#[test]
fn cond_bin_te_wgsl_validates_offline() {
    let module = match naga::front::wgsl::parse_str(COND_BIN_TE_WGSL) {
        Ok(m) => m,
        Err(e) => panic!("wgsl parse: {}", e.emit_to_string(COND_BIN_TE_WGSL)),
    };
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    );
    if let Err(e) = validator.validate(&module) {
        panic!("wgsl validate: {}", e.emit_to_string(COND_BIN_TE_WGSL));
    }
}

#[test]
fn cond_bin_te_gpu_crosscheck_against_cpu() {
    let mut gpu = match CondBinTeGpu::new() {
        Some(g) => g,
        None => {
            eprintln!("compute-only device request returned void — crosscheck skipped");
            return;
        }
    };
    let n = 512;
    let c: Vec<f32> = (0..n).map(|t| (t as f32 * 0.2).sin()).collect();
    let mut rng = 0x9E37_79B9_7F4A_7C15u64;
    let noise = |rng: &mut u64| -> f32 {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
    };
    let x: Vec<f32> = c.iter().map(|&z| z + 0.4 * noise(&mut rng)).collect();
    let mut y = vec![0f32; n];
    for t in 0..n {
        y[t] = if t == 0 {
            0.3 * noise(&mut rng)
        } else {
            0.9 * y[t - 1] + (1.0 - 0.9) * c[t - 1] + 0.3 * noise(&mut rng)
        };
    }
    let cpu = crate::te::transfer_entropy_conditional_binned_n(&y, &x, &[&c], 1, 4)
        .expect("cpu resolves");
    let gpu_te = gpu.run(&y, &x, &c, 1).expect("gpu resolves");
    assert!(
        (gpu_te - cpu).abs() < 1e-3,
        "cond binning TE parity: gpu {} vs cpu {}",
        gpu_te,
        cpu
    );
}

#[test]
fn s2_wgsl_validates_offline() {
    let module = match naga::front::wgsl::parse_str(S2_WGSL) {
        Ok(m) => m,
        Err(e) => panic!("wgsl parse: {}", e.emit_to_string(S2_WGSL)),
    };
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    );
    if let Err(e) = validator.validate(&module) {
        panic!("wgsl validate: {}", e.emit_to_string(S2_WGSL));
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
        let scaled = match crate::te::topological_verdict_from_gpu(&verdict) {
            Some(v) => v,
            None => panic!("te verdict invalid at bandwidth scale {}", h_scale),
        };
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
                em_color: [0.0; 4],
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
                em_color: [0.0; 4],
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

#[test]
fn s2_gpu_matches_the_cpu_spherical_harmonic_reference() {
    use crate::archivar::skydirection::{SkyBandSeries, SkyDirection, SkySample};
    let dir = |name: &str, ra: f64, dec: f64| SkyDirection {
        name: name.to_string(),
        ra_deg: ra,
        dec_deg: dec,
        sigma_arcsec: None,
        bands: vec![SkyBandSeries {
            band: Some("g".to_string()),
            samples: vec![SkySample {
                tdb: 8.4e8,
                mag: 18.0,
            }],
        }],
        distance: None,
        redshift: None,
    };
    let dirs = vec![
        dir("a", 10.0, 20.0),
        dir("b", 40.0, -15.0),
        dir("c", 200.0, 55.0),
    ];
    let oscs = crate::s2::osc_window(&dirs, 8.4e8, crate::s2::S2_TAU_DEFAULT_S);
    let expect = crate::s2::shell_field(&oscs, crate::s2::S2_LMAX);
    let pack = crate::s2::pack_oscs(&oscs, crate::s2::S2_OSC_CAP);
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
    let adapter = match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::None,
        compatible_surface: None,
        force_fallback_adapter: false,
    })) {
        Some(a) => a,
        None => {
            eprintln!("adapter request returned void — s2 gpu crosscheck skipped");
            return;
        }
    };
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
        source: wgpu::ShaderSource::Wgsl(S2_WGSL.into()),
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
                let mut e = storage_entry(true, wgpu::ShaderStages::COMPUTE);
                e.binding = 2;
                e
            },
            {
                let mut e = storage_entry(false, wgpu::ShaderStages::COMPUTE);
                e.binding = 3;
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
        entry_point: Some("s2_field"),
        compilation_options: Default::default(),
        cache: None,
    });
    let cap = crate::s2::S2_OSC_CAP as usize;
    let osc_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (cap * 32) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let probe_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (cap * 16) as u64,
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
        size: (cap * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (cap * 4) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: osc_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: param_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: probe_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: out_buf.as_entire_binding(),
            },
        ],
    });
    queue.write_buffer(&osc_buf, 0, &le_bytes_f32(&pack.osc));
    queue.write_buffer(&probe_buf, 0, &le_bytes_f32(&pack.probes));
    let param = [pack.count, crate::s2::S2_LMAX, pack.probe_count, 0];
    let mut pb = [0u8; 16];
    for (i, p) in param.iter().enumerate() {
        pb[i * 4..i * 4 + 4].copy_from_slice(&p.to_le_bytes());
    }
    queue.write_buffer(&param_buf, 0, &pb);
    let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass.set_pipeline(&pipe);
        pass.set_bind_group(0, &bind, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    let copy_bytes = (pack.probe_count as usize * 4) as u64;
    enc.copy_buffer_to_buffer(&out_buf, 0, &read_buf, 0, copy_bytes);
    queue.submit(std::iter::once(enc.finish()));
    let mapped = Arc::new(AtomicBool::new(false));
    let m2 = mapped.clone();
    let slice = read_buf.slice(..copy_bytes);
    slice.map_async(wgpu::MapMode::Read, move |r| {
        m2.store(r.is_ok(), Ordering::SeqCst);
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
        device.poll(wgpu::Maintain::Poll);
    }
    assert!(
        mapped.load(Ordering::SeqCst),
        "s2 gpu readback returned void"
    );
    let data = slice.get_mapped_range();
    let mut gpu = vec![0f32; pack.probe_count as usize];
    for k in 0..pack.probe_count as usize {
        let mut b = [0u8; 4];
        b.copy_from_slice(&data[k * 4..k * 4 + 4]);
        gpu[k] = f32::from_le_bytes(b);
    }
    drop(data);
    read_buf.unmap();
    for k in 0..gpu.len() {
        let e = expect[k];
        let g = gpu[k] as f64;
        let rel = if e.abs() > 0.0 {
            ((g - e) / e).abs()
        } else {
            g.abs()
        };
        assert!(
            rel < 2e-2,
            "s2 field diverges at probe {k}: gpu {g} cpu {e} rel {rel}"
        );
    }
    let n2 = 130usize;
    let mut dirs2: Vec<SkyDirection> = Vec::with_capacity(n2);
    for i in 0..n2 {
        let z = 1.0 - 2.0 * (i as f64) / (n2 as f64 - 1.0);
        let theta = (i as f64) * 2.399963229728653;
        let ra = (theta.to_degrees() % 360.0 + 360.0) % 360.0;
        let dec = (z.asin()).to_degrees();
        dirs2.push(dir(&format!("s{i}"), ra, dec));
    }
    let oscs2 = crate::s2::osc_window(&dirs2, 8.4e8, crate::s2::S2_TAU_DEFAULT_S);
    let expect2 = crate::s2::shell_field(&oscs2, crate::s2::S2_LMAX);
    let pack2 = crate::s2::pack_oscs(&oscs2, crate::s2::S2_OSC_CAP);
    assert_eq!(pack2.count, n2 as u32);
    assert_eq!(pack2.probe_count, n2 as u32);
    queue.write_buffer(&osc_buf, 0, &le_bytes_f32(&pack2.osc));
    queue.write_buffer(&probe_buf, 0, &le_bytes_f32(&pack2.probes));
    let param2 = [pack2.count, crate::s2::S2_LMAX, pack2.probe_count, 0];
    let mut pb2 = [0u8; 16];
    for (i, p) in param2.iter().enumerate() {
        pb2[i * 4..i * 4 + 4].copy_from_slice(&p.to_le_bytes());
    }
    queue.write_buffer(&param_buf, 0, &pb2);
    let groups = (pack2.probe_count + 63) / 64;
    assert!(groups >= 3, "a 130-probe window must span workgroups");
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = enc2.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass.set_pipeline(&pipe);
        pass.set_bind_group(0, &bind, &[]);
        pass.dispatch_workgroups(groups, 1, 1);
    }
    let copy_bytes2 = (pack2.probe_count as usize * 4) as u64;
    enc2.copy_buffer_to_buffer(&out_buf, 0, &read_buf, 0, copy_bytes2);
    queue.submit(std::iter::once(enc2.finish()));
    let mapped2 = Arc::new(AtomicBool::new(false));
    let m22 = mapped2.clone();
    let slice2 = read_buf.slice(..copy_bytes2);
    slice2.map_async(wgpu::MapMode::Read, move |r| {
        m22.store(r.is_ok(), Ordering::SeqCst);
    });
    let deadline2 = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !mapped2.load(Ordering::SeqCst) && std::time::Instant::now() < deadline2 {
        device.poll(wgpu::Maintain::Poll);
    }
    assert!(
        mapped2.load(Ordering::SeqCst),
        "s2 gpu readback returned void for the wide window"
    );
    let data2 = slice2.get_mapped_range();
    let mut gpu2 = vec![0f32; pack2.probe_count as usize];
    for k in 0..gpu2.len() {
        let mut b = [0u8; 4];
        b.copy_from_slice(&data2[k * 4..k * 4 + 4]);
        gpu2[k] = f32::from_le_bytes(b);
    }
    drop(data2);
    read_buf.unmap();
    for k in 0..gpu2.len() {
        let e = expect2[k];
        let g = gpu2[k] as f64;
        let rel = if e.abs() > 0.0 {
            ((g - e) / e).abs()
        } else {
            g.abs()
        };
        assert!(
            rel < 2e-2,
            "s2 field diverges at wide-window probe {k}: gpu {g} cpu {e} rel {rel}"
        );
    }
}

#[test]
fn sky_tick_projects_event_threads_and_keeps_the_epochless_gate_closed() {
    use crate::archivar::s2event::{S2EventRecord, ROOT_NEUTRINO};
    let evt = |ra: f64, dec: f64, epoch: Option<f64>, energy: Option<f64>| S2EventRecord {
        ra_deg: ra as f32,
        dec_deg: dec as f32,
        sigma_arcsec: None,
        epoch_tdb: epoch,
        energy,
        signalness: None,
        far: None,
        particle_root: ROOT_NEUTRINO,
    };
    let mut app = OmegaLoop {
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
                em_color: [0.0; 4],
            })),
        )
    };
    app.t_presence = 8.4e8;
    app.sky.events = vec![
        evt(30.0, 60.0, Some(8.4e8), Some(187.0)),
        evt(40.0, 50.0, None, Some(9.0)),
    ];
    app.sky_reload();
    app.sky.directions.clear();
    app.sky_tick();
    assert_eq!(app.sky.oscs.len(), 2);
    let with_epoch = &app.sky.oscs[0];
    assert!((with_epoch.weight - 187.0).abs() < 1e-6);
    let expected = evt(30.0, 60.0, Some(8.4e8), Some(187.0));
    let p = expected.unit_direction();
    for k in 0..3 {
        assert!((with_epoch.p_hat[k] - p[k]).abs() < 1e-12);
    }
    let epochless = &app.sky.oscs[1];
    assert_eq!(epochless.weight, 0.0);
    assert_eq!(app.sky.report().live_count, 1);
    assert_eq!(app.sky.report().osc_count, 2);
}

const SCALAR_PARITY_TOL: f64 = 1e-3;

fn sg_gate_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn sg_gate_ar1(n: usize, phi: f64, rng: &mut u64) -> Vec<f32> {
    let mut v = Vec::with_capacity(n);
    let mut x = 0.0f64;
    for _ in 0..n {
        x = phi * x + sg_gate_rng(rng) * 2.0 - 1.0;
        v.push(x as f32);
    }
    v
}

fn sg_fr(gpu: &mut ScalarTeGpu, x: &[f32], y: &[f32], lag: usize) -> Option<(f64, f64)> {
    let grid = gpu.run(x, y, &[]);
    if grid.is_empty() {
        return None;
    }
    let fv = grid[((0 * 11 + 0) * 13 + lag) * 2 + 1];
    let rv = grid[((1 * 11 + 0) * 13 + lag) * 2 + 1];
    if fv == 0.0 || rv == 0.0 {
        return None;
    }
    Some((
        grid[((0 * 11 + 0) * 13 + lag) * 2] as f64,
        grid[((1 * 11 + 0) * 13 + lag) * 2] as f64,
    ))
}

fn sg_parity_agree(
    gpu: &mut ScalarTeGpu,
    x: &[f32],
    y: &[f32],
    lag: usize,
    seed: u64,
) -> Option<(bool, bool, f64, f64)> {
    let cpu_te = crate::te::transfer_entropy_lag(x, y, lag)?;
    let (_, _, thr) = crate::te::surrogate_stats_phase(x, y, lag, seed)?;
    let (gf, _) = sg_fr(gpu, x, y, lag)?;
    Some((cpu_te > thr, gf > thr, cpu_te, gf))
}

#[test]
fn scalar_gpu_parity_fp_decision() {
    let Some(mut gpu) = ScalarTeGpu::new(1) else {
        eprintln!("scalar gpu fp gate skipped: no adapter");
        return;
    };
    let mut rng = 0x9E37_79B9_7F4A_7C15u64;
    let mut gpu_fp = 0usize;
    let mut gpu_meas = 0usize;
    let mut agree = 0usize;
    let mut agree_tot = 0usize;
    let mut floor_viol = 0usize;
    for t in 0..30 {
        let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let x = sg_gate_ar1(300, 0.7, &mut rng);
        let y = sg_gate_ar1(300, 0.7, &mut rng);
        let Some((cpu_ab, gpu_ab, cpu_te, gpu_te)) = sg_parity_agree(&mut gpu, &x, &y, 0, seed)
        else {
            continue;
        };
        gpu_meas += 1;
        if gpu_ab {
            gpu_fp += 1;
        }
        agree_tot += 1;
        if cpu_ab == gpu_ab {
            agree += 1;
        }
        if (gpu_te - cpu_te).abs() > SCALAR_PARITY_TOL * (cpu_te.abs() + 1e-3) {
            floor_viol += 1;
        }
    }
    assert!(
        gpu_meas >= 20,
        "scalar gpu FP gate: {} of 30 measurable — the machine stays silent too often",
        gpu_meas
    );
    assert!(
        gpu_fp <= 8,
        "scalar gpu FP gate: {} of {} above the threshold — the null does not hold on the GPU",
        gpu_fp,
        gpu_meas
    );
    assert!(
        agree * 100 >= agree_tot * 95,
        "scalar gpu FP gate: CPU/GPU classification agrees in {}/{}",
        agree,
        agree_tot
    );
    assert_eq!(
        floor_viol, 0,
        "scalar gpu FP gate: {} numeric-floor violations",
        floor_viol
    );
}

#[test]
fn scalar_gpu_parity_fn_decision() {
    let Some(mut gpu) = ScalarTeGpu::new(1) else {
        eprintln!("scalar gpu fn gate skipped: no adapter");
        return;
    };
    let mut rng = 0x517C_C1B7_2722_0A95u64;
    let mut found = 0usize;
    let mut meas = 0usize;
    let mut agree = 0usize;
    let mut floor_viol = 0usize;
    for t in 0..20 {
        let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let a: Vec<f32> = (0..300)
            .map(|_| (sg_gate_rng(&mut rng) * 2.0 - 1.0) as f32)
            .collect();
        let b: Vec<f32> = (0..a.len())
            .map(|i| {
                if i == 0 {
                    sg_gate_rng(&mut rng) as f32
                } else {
                    (0.9 * a[i - 1] as f64 + (sg_gate_rng(&mut rng) * 0.2 - 0.1)) as f32
                }
            })
            .collect();
        let Some((cpu_ab, gpu_ab, cpu_te, gpu_te)) = sg_parity_agree(&mut gpu, &b, &a, 0, seed)
        else {
            continue;
        };
        meas += 1;
        if cpu_ab && gpu_ab {
            found += 1;
        }
        if cpu_ab == gpu_ab {
            agree += 1;
        }
        if (gpu_te - cpu_te).abs() > SCALAR_PARITY_TOL * (cpu_te.abs() + 1e-3) {
            floor_viol += 1;
        }
    }
    if meas == 0 {
        panic!("scalar gpu FN gate: no coupling measurement succeeded in 20 trials");
    }
    assert!(
        found as f64 / meas as f64 > 0.5,
        "scalar gpu FN gate: {} of {} true couplings found on the GPU",
        found,
        meas
    );
    assert!(
        agree * 100 >= meas * 95,
        "scalar gpu FN gate: CPU/GPU classification agrees in {}/{}",
        agree,
        meas
    );
    assert_eq!(
        floor_viol, 0,
        "scalar gpu FN gate: {} numeric-floor violations",
        floor_viol
    );
}

#[test]
fn scalar_gpu_parity_symmetry() {
    let Some(mut gpu) = ScalarTeGpu::new(1) else {
        eprintln!("scalar gpu symmetry gate skipped: no adapter");
        return;
    };
    let mut rng = 0x2722_0A95_517C_C1B7u64;
    let a = sg_gate_ar1(300, 0.7, &mut rng);
    let Some((f, r)) = sg_fr(&mut gpu, &a, &a, 0) else {
        return;
    };
    assert!(
        (f - r).abs() < 1e-3,
        "scalar gpu symmetry: a=a measures unequal, {} vs {}",
        f,
        r
    );
}

#[test]
fn scalar_gpu_parity_n_floor() {
    let Some(mut gpu) = ScalarTeGpu::new(13) else {
        eprintln!("scalar gpu n-floor gate skipped: no adapter");
        return;
    };
    let x = sg_gate_ar1(300, 0.7, &mut 0x9E37_79B9_7F4A_7C15u64);
    let constant = vec![1.0f32; 300];
    let gpu_te = crate::te::transfer_entropy_lag(&x, &constant, 0);
    assert!(
        gpu_te.is_none(),
        "scalar CPU constant series must stay absent"
    );
    let grid = gpu.run(&x, &constant, &[]);
    if !grid.is_empty() {
        let valid = grid[0 + 1];
        assert_eq!(valid, 0.0, "scalar GPU constant series must stay absent");
    }
}

#[test]
fn scalar_gpu_parity_surrogate_slots_match_cpu() {
    let Some(mut gpu) = ScalarTeGpu::new(13) else {
        eprintln!("scalar gpu surrogate-slot gate skipped: no adapter");
        return;
    };
    let mut rng = 0x0FEB_11D1_B2D1_9C93u64;
    let x = sg_gate_ar1(300, 0.7, &mut rng);
    let y = sg_gate_ar1(300, 0.7, &mut rng);
    let mut surrs: Vec<Vec<f32>> = Vec::new();
    for _ in 0..10 {
        surrs.push(crate::te::phase_randomized_surrogate(&y, &mut rng));
    }
    let grid = gpu.run(&x, &y, &surrs);
    if grid.is_empty() {
        return;
    }
    let series_at = |k: usize| -> &[f32] {
        if k == 0 {
            &y
        } else {
            &surrs[k - 1]
        }
    };
    let mut viol = 0usize;
    let mut meas = 0usize;
    for dir in 0..2 {
        for k in 0..11 {
            for lag in 0..13 {
                let idx = ((dir * 11 + k) * 13 + lag) * 2;
                let gpu_te = grid[idx];
                let gpu_valid = grid[idx + 1];
                let cpu_te = if dir == 0 {
                    crate::te::transfer_entropy_lag(&x, series_at(k), lag)
                } else {
                    crate::te::transfer_entropy_lag(series_at(k), &x, lag)
                };
                match (gpu_valid > 0.0, cpu_te) {
                    (true, Some(c)) => {
                        meas += 1;
                        if (gpu_te as f64 - c).abs() > SCALAR_PARITY_TOL * (c.abs() + 1e-3) {
                            viol += 1;
                            eprintln!(
                                "surrogate slot: dir {} k {} lag {} gpu {} cpu {}",
                                dir, k, lag, gpu_te, c
                            );
                        }
                    }
                    (false, None) => {}
                    (true, None) => {
                        viol += 1;
                        eprintln!(
                            "surrogate slot: dir {} k {} lag {} gpu valid, cpu absent",
                            dir, k, lag
                        );
                    }
                    (false, Some(c)) => {
                        viol += 1;
                        eprintln!(
                            "surrogate slot: dir {} k {} lag {} gpu absent, cpu {}",
                            dir, k, lag, c
                        );
                    }
                }
            }
        }
    }
    assert!(
        meas >= 200,
        "scalar gpu surrogate-slot gate: only {} of 286 grid slots measurable",
        meas
    );
    assert_eq!(
        viol, 0,
        "scalar gpu surrogate-slot gate: {} grid mismatches",
        viol
    );
}
