use crate::core::{build_buffer, sense_buffer, Buffer, Motion, Sample};

static mut CACHE: Option<Buffer> = None;

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_load_buffer(ptr: *const u8, len: usize) -> u32 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    if slice.len() < 72 {
        return 0;
    }
    let record_count = slice.len() / 72;
    let mut samples: Vec<Sample> = Vec::with_capacity(record_count);
    for i in 0..record_count {
        let off = i * 72;
        let x = f64::from_le_bytes(slice[off..off + 8].try_into().unwrap_or([0; 8]));
        let y = f64::from_le_bytes(slice[off + 8..off + 16].try_into().unwrap_or([0; 8]));
        let z = f64::from_le_bytes(slice[off + 16..off + 24].try_into().unwrap_or([0; 8]));
        let val = f64::from_le_bytes(slice[off + 24..off + 32].try_into().unwrap_or([0; 8]));
        let extent = f64::from_le_bytes(slice[off + 32..off + 40].try_into().unwrap_or([0; 8]));
        let t = f64::from_le_bytes(slice[off + 40..off + 48].try_into().unwrap_or([0; 8]));
        let ttl = f64::from_le_bytes(slice[off + 48..off + 56].try_into().unwrap_or([0; 8]));
        let tau = f64::from_le_bytes(slice[off + 56..off + 64].try_into().unwrap_or([0; 8]));
        let force_type = f64::from_le_bytes(slice[off + 64..off + 72].try_into().unwrap_or([0; 8]));
        let vmax = if tau > 0.0 {
            extent / tau
        } else {
            extent * 0.01
        };
        let origin_hash: u32 = ((x * 100.0) as i64).wrapping_mul(0x9E3779B9) as u32
            ^ (((y * 100.0) as i64).wrapping_mul(0x9E3779B9) as u32 >> 16);
        samples.push(Sample {
            origin: (origin_hash, (x * 1000.0) as i32, (y * 1000.0) as i32),
            epoch: t,
            ttl,
            extent,
            tau,
            force_type,
            vmax,
            amax: vmax * 0.1,
            p0f: [x, y, z],
            motion: Motion::Linear {
                p: [x, y, z],
                v: [0.0, 0.0, 0.0],
            },
            fields: vec![("val".to_string(), val)],
        });
    }
    let buffer = build_buffer(samples, 1.0);
    unsafe {
        CACHE = Some(buffer);
    }
    record_count as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn omegaflow_query(
    px: f64,
    py: f64,
    pz: f64,
    pt: f64,
    extent: f64,
    out_ptr: *mut f64,
    max_records: u32,
) -> u32 {
    let buffer = unsafe { &*std::ptr::addr_of!(CACHE) }.as_ref();
    let Some(buffer) = buffer else { return 0 };
    let mut records: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
    sense_buffer(buffer, [px, py, pz], pt, extent, &mut records, None);
    let n = records.len().min(max_records as usize);
    let out = unsafe { std::slice::from_raw_parts_mut(out_ptr, n * 9) };
    for (i, (x, y, z, val, ext, t, ttl, tau, ft)) in records.iter().take(n).enumerate() {
        let o = i * 9;
        out[o] = *x;
        out[o + 1] = *y;
        out[o + 2] = *z;
        out[o + 3] = *val;
        out[o + 4] = *ext;
        out[o + 5] = *t;
        out[o + 6] = *ttl;
        out[o + 7] = *tau;
        out[o + 8] = *ft;
    }
    n as u32
}
