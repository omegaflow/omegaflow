pub(crate) const TE_SERIES_STRIDE: usize = 1024;

pub(crate) const TE_SERIES_BYTES: u64 = (12 * TE_SERIES_STRIDE * 4) as u64;

pub(crate) fn le_bytes_f32(v: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len() * 4);
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
    out
}

pub(crate) fn te_read_verdict(read_buf: &wgpu::Buffer) -> [f32; 72] {
    let data = read_buf.slice(..).get_mapped_range();
    let mut verdict = [0f32; 72];
    for k in 0..72 {
        let mut b = [0u8; 4];
        b.copy_from_slice(&data[k * 4..k * 4 + 4]);
        verdict[k] = f32::from_le_bytes(b);
    }
    drop(data);
    read_buf.unmap();
    verdict
}

pub(crate) fn te_absence_word(verdict: &[f32; 72]) -> &'static str {
    if verdict[10] != 1.0 {
        "real series invalid"
    } else {
        "fewer than two surrogates"
    }
}
