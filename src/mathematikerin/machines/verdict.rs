pub(crate) const TE_SERIES_STRIDE: usize = 1024;

pub(crate) const TE_SERIES_BYTES: u64 = (12 * TE_SERIES_STRIDE * 4) as u64;

pub(crate) const TE_VERDICT_SLOTS: usize = 12 * 6;
pub(crate) const TE_KSG_SLOTS: usize = 12 * 2;

pub(crate) const fn te_verdict_bytes(k: u32) -> u64 {
    ((TE_VERDICT_SLOTS + if k > 0 { TE_KSG_SLOTS } else { 0 }) * 4) as u64
}

pub(crate) const TE_KSG_K_PROD: u32 = 0;

pub(crate) fn le_bytes_f32(v: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len() * 4);
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
    out
}

pub(crate) fn te_read_verdict(read_buf: &wgpu::Buffer) -> [f32; TE_VERDICT_SLOTS] {
    let data = read_buf.slice(..).get_mapped_range();
    let mut verdict = [0f32; TE_VERDICT_SLOTS];
    for k in 0..TE_VERDICT_SLOTS {
        let mut b = [0u8; 4];
        b.copy_from_slice(&data[k * 4..k * 4 + 4]);
        verdict[k] = f32::from_le_bytes(b);
    }
    drop(data);
    read_buf.unmap();
    verdict
}

pub(crate) fn te_absence_word(verdict: &[f32; TE_VERDICT_SLOTS]) -> &'static str {
    if verdict[10] != 1.0 {
        "real series invalid"
    } else {
        "fewer than two surrogates"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_te_verdict_bytes_follows_k() {
        assert_eq!(te_verdict_bytes(0), 288, "K=0 carries the base verdict only");
        for k in 1..=8u32 {
            assert!(
                te_verdict_bytes(k) >= 384,
                "K={} must append the KSG columns (readback >= 384 B)",
                k
            );
        }
        assert_eq!(
            te_verdict_bytes(4),
            384,
            "the Kalibrier-Gate K grows the verdict to 384 B"
        );
    }
}
