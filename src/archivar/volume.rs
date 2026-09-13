const MAGIC: [u8; 4] = [0xCF, 0x86, 0x0D, 0x01];
const HEADER_LEN: usize = 0x40;

fn align16(p: usize) -> usize {
    (p + 15) & !15
}

fn le_u32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn le_f64(b: &[u8], off: usize) -> f64 {
    f64::from_le_bytes([
        b[off],
        b[off + 1],
        b[off + 2],
        b[off + 3],
        b[off + 4],
        b[off + 5],
        b[off + 6],
        b[off + 7],
    ])
}

fn le_f32(b: &[u8], off: usize) -> f32 {
    f32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AxisKind {
    Uniform,
    Explicit,
}

impl AxisKind {
    fn from_u8(k: u8) -> Option<AxisKind> {
        match k {
            0 => Some(AxisKind::Uniform),
            1 => Some(AxisKind::Explicit),
            _ => None,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            AxisKind::Uniform => 0,
            AxisKind::Explicit => 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Axis {
    pub kind: AxisKind,
    pub values: Vec<f64>,
}

impl Axis {
    fn value_count(&self, n: usize) -> usize {
        match self.kind {
            AxisKind::Uniform => 2,
            AxisKind::Explicit => n,
        }
    }

    fn byte_len(&self, n: usize) -> usize {
        self.value_count(n) * 8
    }

    fn is_strictly_monotonic(&self) -> bool {
        if self.values.len() <= 1 {
            return true;
        }
        let asc = self.values.windows(2).all(|w| w[0] < w[1]);
        let desc = self.values.windows(2).all(|w| w[0] > w[1]);
        asc || desc
    }

    fn value_at(&self, i: usize) -> f64 {
        match self.kind {
            AxisKind::Uniform => self.values[0] + i as f64 * self.values[1],
            AxisKind::Explicit => self.values[i],
        }
    }

    fn frac_index(&self, coord: f64, n: usize) -> Option<f64> {
        if n == 0 {
            return None;
        }
        let first = self.value_at(0);
        let last = self.value_at(n - 1);
        let (lo, hi) = if first <= last {
            (first, last)
        } else {
            (last, first)
        };
        if coord < lo || coord > hi {
            return None;
        }
        match self.kind {
            AxisKind::Uniform => Some((coord - first) / self.values[1]),
            AxisKind::Explicit => {
                let ascending = first <= last;
                let mut lo_i = 0usize;
                let mut hi_i = n - 1;
                while hi_i - lo_i > 1 {
                    let mid = (lo_i + hi_i) / 2;
                    let before = if ascending {
                        self.values[mid] <= coord
                    } else {
                        self.values[mid] >= coord
                    };
                    if before {
                        lo_i = mid;
                    } else {
                        hi_i = mid;
                    }
                }
                let v0 = self.values[lo_i];
                let v1 = self.values[hi_i];
                if (v1 - v0).abs() < f64::EPSILON {
                    return Some(lo_i as f64);
                }
                Some(lo_i as f64 + (coord - v0) / (v1 - v0))
            }
        }
    }
}

fn bracket(f: f64, n: usize) -> (usize, usize, f64) {
    if n <= 1 {
        return (0, 0, 0.0);
    }
    let f = f.max(0.0).min((n - 1) as f64);
    let i = f.floor() as usize;
    if i >= n - 1 {
        (n - 1, n - 1, 0.0)
    } else {
        (i, i + 1, f - i as f64)
    }
}

fn kind_byte_len(kind: AxisKind, n: usize) -> usize {
    match kind {
        AxisKind::Uniform => 16,
        AxisKind::Explicit => n * 8,
    }
}

#[derive(Clone, Debug)]
pub struct Volume {
    pub dims: [u32; 3],
    pub axes: [Axis; 3],
    pub data: Vec<f32>,
}

impl Volume {
    fn cells(&self) -> usize {
        self.dims[0] as usize * self.dims[1] as usize * self.dims[2] as usize
    }

    fn cell(&self, d: usize, la: usize, lo: usize) -> f64 {
        let idx = (d * self.dims[1] as usize + la) * self.dims[2] as usize + lo;
        self.data[idx] as f64
    }

    pub fn sample_at(&self, coord: [f64; 3]) -> Option<f64> {
        let f0 = self.axes[0].frac_index(coord[0], self.dims[0] as usize)?;
        let f1 = self.axes[1].frac_index(coord[1], self.dims[1] as usize)?;
        let f2 = self.axes[2].frac_index(coord[2], self.dims[2] as usize)?;
        let (i0, i1, t0) = bracket(f0, self.dims[0] as usize);
        let (j0, j1, t1) = bracket(f1, self.dims[1] as usize);
        let (k0, k1, t2) = bracket(f2, self.dims[2] as usize);
        let c000 = self.cell(i0, j0, k0);
        let c001 = self.cell(i0, j0, k1);
        let c010 = self.cell(i0, j1, k0);
        let c011 = self.cell(i0, j1, k1);
        let c100 = self.cell(i1, j0, k0);
        let c101 = self.cell(i1, j0, k1);
        let c110 = self.cell(i1, j1, k0);
        let c111 = self.cell(i1, j1, k1);
        let c00 = c000 + (c100 - c000) * t0;
        let c01 = c001 + (c101 - c001) * t0;
        let c10 = c010 + (c110 - c010) * t0;
        let c11 = c011 + (c111 - c011) * t0;
        let c0 = c00 + (c10 - c00) * t1;
        let c1 = c01 + (c11 - c01) * t1;
        Some(c0 + (c1 - c0) * t2)
    }

    pub fn write_bin(&self) -> Vec<u8> {
        let nd = self.dims[0] as usize;
        let nla = self.dims[1] as usize;
        let nlo = self.dims[2] as usize;
        let axis_len =
            self.axes[0].byte_len(nd) + self.axes[1].byte_len(nla) + self.axes[2].byte_len(nlo);
        let data_offset = align16(HEADER_LEN + axis_len);
        let mut buf = vec![0u8; data_offset + self.cells() * 4];
        buf[0..4].copy_from_slice(&MAGIC);
        buf[6] = self.axes[0].kind.to_u8();
        buf[7] = self.axes[1].kind.to_u8();
        buf[8] = self.axes[2].kind.to_u8();
        buf[9] = 0x00;
        buf[12..16].copy_from_slice(&(nd as u32).to_le_bytes());
        buf[16..20].copy_from_slice(&(nla as u32).to_le_bytes());
        buf[20..24].copy_from_slice(&(nlo as u32).to_le_bytes());
        let mut p = HEADER_LEN;
        for axis in &self.axes {
            match axis.kind {
                AxisKind::Uniform => {
                    buf[p..p + 8].copy_from_slice(&axis.values[0].to_le_bytes());
                    buf[p + 8..p + 16].copy_from_slice(&axis.values[1].to_le_bytes());
                    p += 16;
                }
                AxisKind::Explicit => {
                    for &v in &axis.values {
                        buf[p..p + 8].copy_from_slice(&v.to_le_bytes());
                        p += 8;
                    }
                }
            }
        }
        p = data_offset;
        for &v in &self.data {
            buf[p..p + 4].copy_from_slice(&v.to_le_bytes());
            p += 4;
        }
        let digest = crate::archivar::sha256::sha256_raw(&buf[HEADER_LEN..]);
        buf[0x18..0x38].copy_from_slice(&digest);
        buf
    }

    pub fn read_bin(bytes: &[u8]) -> Option<Volume> {
        if bytes.len() < HEADER_LEN {
            return None;
        }
        if bytes[0..4] != MAGIC {
            return None;
        }
        if bytes[4] != 0 || bytes[5] != 0 {
            return None;
        }
        let kinds = [
            AxisKind::from_u8(bytes[6])?,
            AxisKind::from_u8(bytes[7])?,
            AxisKind::from_u8(bytes[8])?,
        ];
        if bytes[9] != 0x00 {
            return None;
        }
        if bytes[10] != 0 || bytes[11] != 0 {
            return None;
        }
        let nd = le_u32(bytes, 12) as usize;
        let nla = le_u32(bytes, 16) as usize;
        let nlo = le_u32(bytes, 20) as usize;
        if nd == 0 || nla == 0 || nlo == 0 {
            return None;
        }
        if bytes[0x38..0x40].iter().any(|&b| b != 0) {
            return None;
        }
        let dims = [nd, nla, nlo];
        let axis_len = kind_byte_len(kinds[0], nd)
            .checked_add(kind_byte_len(kinds[1], nla))?
            .checked_add(kind_byte_len(kinds[2], nlo))?;
        let data_offset = align16(HEADER_LEN + axis_len);
        let cells = (nd as u64)
            .checked_mul(nla as u64)?
            .checked_mul(nlo as u64)?;
        let expected = (data_offset as u64).checked_add(cells.checked_mul(4)?)?;
        if bytes.len() as u64 != expected {
            return None;
        }
        let stored = &bytes[0x18..0x38];
        let computed = crate::archivar::sha256::sha256_raw(&bytes[HEADER_LEN..]);
        if computed.as_slice() != stored {
            return None;
        }

        let mut axes: [Option<Axis>; 3] = [None, None, None];
        let mut p = HEADER_LEN;
        for i in 0..3 {
            let axis = match kinds[i] {
                AxisKind::Uniform => {
                    let origin = le_f64(bytes, p);
                    let step = le_f64(bytes, p + 8);
                    p += 16;
                    if !origin.is_finite() || !step.is_finite() || step == 0.0 {
                        return None;
                    }
                    Axis {
                        kind: AxisKind::Uniform,
                        values: vec![origin, step],
                    }
                }
                AxisKind::Explicit => {
                    let mut values = Vec::with_capacity(dims[i]);
                    for _ in 0..dims[i] {
                        let v = le_f64(bytes, p);
                        p += 8;
                        if !v.is_finite() {
                            return None;
                        }
                        values.push(v);
                    }
                    let axis = Axis {
                        kind: AxisKind::Explicit,
                        values,
                    };
                    if !axis.is_strictly_monotonic() {
                        return None;
                    }
                    axis
                }
            };
            axes[i] = Some(axis);
        }

        let mut data = Vec::with_capacity(cells as usize);
        let mut q = data_offset;
        for _ in 0..cells {
            let v = le_f32(bytes, q);
            q += 4;
            if !v.is_finite() {
                return None;
            }
            data.push(v);
        }

        Some(Volume {
            dims: [nd as u32, nla as u32, nlo as u32],
            axes: [
                axes[0].take().unwrap(),
                axes[1].take().unwrap(),
                axes[2].take().unwrap(),
            ],
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_volume() -> Volume {
        let axes = [
            Axis {
                kind: AxisKind::Explicit,
                values: vec![0.0, 100.0, 200.0],
            },
            Axis {
                kind: AxisKind::Uniform,
                values: vec![0.0, 1.0],
            },
            Axis {
                kind: AxisKind::Uniform,
                values: vec![10.0, 1.0],
            },
        ];
        let mut data = Vec::new();
        for d in 0..3 {
            for la in 0..2 {
                for lo in 0..2 {
                    data.push((d * 100 + la * 10 + lo) as f32);
                }
            }
        }
        Volume {
            dims: [3, 2, 2],
            axes,
            data,
        }
    }

    #[test]
    fn round_trips() {
        let v = test_volume();
        let bin = v.write_bin();
        let back = Volume::read_bin(&bin).unwrap();
        assert_eq!(back.dims, [3, 2, 2]);
        assert_eq!(back.data.len(), 12);
        for i in 0..12 {
            assert_eq!(back.data[i], v.data[i]);
        }
    }

    #[test]
    fn sample_at_node_equals_cell() {
        let v = test_volume();
        assert_eq!(v.sample_at([100.0, 1.0, 10.0]).unwrap(), v.cell(1, 1, 0));
    }

    #[test]
    fn sample_outside_domain_is_absent() {
        let v = test_volume();
        assert!(v.sample_at([-1.0, 0.0, 10.0]).is_none());
        assert!(v.sample_at([0.0, 5.0, 10.0]).is_none());
        assert!(v.sample_at([0.0, 0.0, 9.0]).is_none());
    }

    #[test]
    fn trilinear_midpoint_is_corner_mean() {
        let v = test_volume();
        let corners = [
            v.cell(0, 0, 0),
            v.cell(0, 0, 1),
            v.cell(0, 1, 0),
            v.cell(0, 1, 1),
            v.cell(1, 0, 0),
            v.cell(1, 0, 1),
            v.cell(1, 1, 0),
            v.cell(1, 1, 1),
        ];
        let expected: f64 = corners.iter().sum::<f64>() / 8.0;
        assert_eq!(v.sample_at([50.0, 0.5, 10.5]).unwrap(), expected);
    }

    #[test]
    fn refuses_bad_magic() {
        let mut bin = test_volume().write_bin();
        bin[0] = 0x00;
        assert!(Volume::read_bin(&bin).is_none());
    }

    #[test]
    fn refuses_digest_mismatch() {
        let mut bin = test_volume().write_bin();
        let last = bin.len() - 1;
        bin[last] ^= 0xFF;
        assert!(Volume::read_bin(&bin).is_none());
    }

    #[test]
    fn refuses_size_mismatch() {
        let mut bin = test_volume().write_bin();
        bin.push(0);
        assert!(Volume::read_bin(&bin).is_none());
    }
}
