src/main.rs

=== Add after line 24 ===
const V_SOUND_288: f64 = 343.0;
const V_P_GRANITE: f64 = 5900.0;
const V_S_GRANITE: f64 = 2930.0;
const D_AIR: f64 = 2.0e-5;
const ALPHA_AIR: f64 = 2.18e-5;
const V_WIND_REF: f64 = 5.0;

=== Replace lines 168-179 ===
old:
#[derive(Clone)]
struct Sample {
    origin: Origin,
    epoch: f64,
    ttl: f64,
    r: f64,
    vmax: f64,
    amax: f64,
    p0f: [f64; 3],
    motion: Motion,
    fields: Vec<(String, f64)>,
}
new:
#[derive(Clone)]
struct Sample {
    origin: Origin,
    epoch: f64,
    ttl: f64,
    extent: f64,
    tau: f64,
    force_type: f64,
    vmax: f64,
    amax: f64,
    p0f: [f64; 3],
    motion: Motion,
    fields: Vec<(String, f64)>,
}

=== Delete lines 197-199 ===

=== Replace lines 231-272 ===
old:
fn build_family(samples: Vec<Sample>, cadence: f64) -> Family {
    let mut vmax = 0.0f64;
    let mut amax = 0.0f64;
    let mut rmax = 0.0f64;
    let mut epoch_min = f64::MAX;
    for s in &samples {
        vmax = vmax.max(s.vmax);
        amax = amax.max(s.amax);
        rmax = rmax.max(s.r);
        epoch_min = epoch_min.min(s.epoch);
    }
    let rho_cad = rmax + vmax * cadence + 0.5 * amax * cadence * cadence;
    let shift = (2.0 * rho_cad).log2().ceil().clamp(0.0, 63.0) as i32;
    let cell_size = 2f64.powi(shift);
    let mut cells: HashMap<CellKey, Vec<Sample>> = HashMap::new();
    let mut cell_lo = (i64::MAX, i64::MAX, i64::MAX);
    let mut cell_hi = (i64::MIN, i64::MIN, i64::MIN);
    for s in samples {
        let c = cell_of(s.p0f, cell_size);
        cell_lo.0 = cell_lo.0.min(c.0);
        cell_lo.1 = cell_lo.1.min(c.1);
        cell_lo.2 = cell_lo.2.min(c.2);
        cell_hi.0 = cell_hi.0.max(c.0);
        cell_hi.1 = cell_hi.1.max(c.1);
        cell_hi.2 = cell_hi.2.max(c.2);
        cells.entry(c).or_default().push(s);
    }
    Family {
        cell_size,
        vmax,
        amax,
        rmax,
        epoch_min: if epoch_min == f64::MAX {
            0.0
        } else {
            epoch_min
        },
        cell_lo,
        cell_hi,
        cells,
    }
}
new:
fn build_family(samples: Vec<Sample>, cadence: f64) -> Family {
    let mut vmax = 0.0f64;
    let mut amax = 0.0f64;
    let mut rmax = 0.0f64;
    let mut epoch_min = f64::MAX;
    let mut min_extent = f64::MAX;
    for s in &samples {
        vmax = vmax.max(s.vmax);
        amax = amax.max(s.amax);
        rmax = rmax.max(s.extent);
        epoch_min = epoch_min.min(s.epoch);
        min_extent = min_extent.min(s.extent);
    }
    let shift = (min_extent.max(1.0)).log2().floor().clamp(0.0, 63.0) as i32;
    let cell_size = 2f64.powi(shift);
    let mut cells: HashMap<CellKey, Vec<Sample>> = HashMap::new();
    let mut cell_lo = (i64::MAX, i64::MAX, i64::MAX);
    let mut cell_hi = (i64::MIN, i64::MIN, i64::MIN);
    for s in samples {
        let c = cell_of(s.p0f, cell_size);
        cell_lo.0 = cell_lo.0.min(c.0);
        cell_lo.1 = cell_lo.1.min(c.1);
        cell_lo.2 = cell_lo.2.min(c.2);
        cell_hi.0 = cell_hi.0.max(c.0);
        cell_hi.1 = cell_hi.1.max(c.1);
        cell_hi.2 = cell_hi.2.max(c.2);
        cells.entry(c).or_default().push(s);
    }
    Family {
        cell_size,
        vmax,
        amax,
        rmax,
        epoch_min: if epoch_min == f64::MAX { 0.0 } else { epoch_min },
        cell_lo,
        cell_hi,
        cells,
    }
}

=== Replace entire enclose_family function (lines 283-343) ===
old:
fn enclose_family(
    fam: &Family,
    anchor: [f64; 3],
    q: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64)>,
) {
    if fam.cells.is_empty() { return; }
    let qf = [q[0] - anchor[0], q[1] - anchor[1], q[2] - anchor[2]];
    let dt = (t2 - fam.epoch_min).abs();
    let rho = fam.rmax + fam.vmax * dt + 0.5 * fam.amax * dt * dt + pad;
    let s = fam.cell_size;
    let qlo = cell_of([qf[0] - rho, qf[1] - rho, qf[2] - rho], s);
    let qhi = cell_of([qf[0] + rho, qf[1] + rho, qf[2] + rho], s);
    let lo = (
        qlo.0.max(fam.cell_lo.0),
        qlo.1.max(fam.cell_lo.1),
        qlo.2.max(fam.cell_lo.2),
    );
    let hi = (
        qhi.0.min(fam.cell_hi.0),
        qhi.1.min(fam.cell_hi.1),
        qhi.2.min(fam.cell_hi.2),
    );
    if lo.0 > hi.0 || lo.1 > hi.1 || lo.2 > hi.2 { return; }
    for cx in lo.0..=hi.0 {
        for cy in lo.1..=hi.1 {
            for cz in lo.2..=hi.2 {
                let Some(samples) = fam.cells.get(&(cx, cy, cz)) else { continue; };
                for smp in samples {
                    let age = (t2 - smp.epoch).abs();
                    let reach = smp.r + smp.vmax * age + 0.5 * smp.amax * age * age + pad;
                    let dx = smp.p0f[0] - qf[0];
                    let dy = smp.p0f[1] - qf[1];
                    let dz = smp.p0f[2] - qf[2];
                    if dx * dx + dy * dy + dz * dz > reach * reach { continue; }
                    let p = smp.motion.at(t2, smp.epoch);
                    let ddx = p[0] - q[0];
                    let ddy = p[1] - q[1];
                    let ddz = p[2] - q[2];
                    let exact = smp.r + pad;
                    if ddx * ddx + ddy * ddy + ddz * ddz > exact * exact { continue; }
                    for (_, val) in &smp.fields {
                        records.push((*val, p[0], p[1], p[2], smp.r, smp.epoch, smp.ttl));
                    }
                }
            }
        }
    }
}
new:
fn enclose_family(
    fam: &Family,
    anchor: [f64; 3],
    q: [f64; 3],
    t2: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
) {
    if fam.cells.is_empty() { return; }
    let qf = [q[0] - anchor[0], q[1] - anchor[1], q[2] - anchor[2]];
    let dt = (t2 - fam.epoch_min).abs();
    let rho = fam.rmax + fam.vmax * dt + 0.5 * fam.amax * dt * dt;
    let s = fam.cell_size;
    let qlo = cell_of([qf[0] - rho, qf[1] - rho, qf[2] - rho], s);
    let qhi = cell_of([qf[0] + rho, qf[1] + rho, qf[2] + rho], s);
    let lo = (
        qlo.0.max(fam.cell_lo.0),
        qlo.1.max(fam.cell_lo.1),
        qlo.2.max(fam.cell_lo.2),
    );
    let hi = (
        qhi.0.min(fam.cell_hi.0),
        qhi.1.min(fam.cell_hi.1),
        qhi.2.min(fam.cell_hi.2),
    );
    if lo.0 > hi.0 || lo.1 > hi.1 || lo.2 > hi.2 { return; }
    for cx in lo.0..=hi.0 {
        for cy in lo.1..=hi.1 {
            for cz in lo.2..=hi.2 {
                let Some(samples) = fam.cells.get(&(cx, cy, cz)) else { continue; };
                for smp in samples {
                    let age = (t2 - smp.epoch).abs();
                    let reach = smp.extent + smp.vmax * age + 0.5 * smp.amax * age * age;
                    let dx = smp.p0f[0] - qf[0];
                    let dy = smp.p0f[1] - qf[1];
                    let dz = smp.p0f[2] - qf[2];
                    if dx * dx + dy * dy + dz * dz > reach * reach { continue; }
                    let p = smp.motion.at(t2, smp.epoch);
                    let ddx = p[0] - q[0];
                    let ddy = p[1] - q[1];
                    let ddz = p[2] - q[2];
                    let exact = smp.extent;
                    if ddx * ddx + ddy * ddy + ddz * ddz > exact * exact { continue; }
                    for (_, val) in &smp.fields {
                        records.push((p[0], p[1], p[2], *val, smp.extent, smp.epoch, smp.ttl, smp.tau, smp.force_type));
                    }
                }
            }
        }
    }
}

=== Replace lines 345-355 ===
old:
fn sense_buffer(
    buf: &Buffer,
    q: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64)>,
) {
    let (ex, ey, ez) = earth_position_icrs(t2);
    enclose_family(&buf.terra, [ex, ey, ez], q, t2, pad, records);
    enclose_family(&buf.inertial, [0.0, 0.0, 0.0], q, t2, pad, records);
}
new:
fn sense_buffer(
    buf: &Buffer,
    q: [f64; 3],
    t2: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
) {
    let (ex, ey, ez) = earth_position_icrs(t2);
    enclose_family(&buf.terra, [ex, ey, ez], q, t2, records);
    enclose_family(&buf.inertial, [0.0, 0.0, 0.0], q, t2, records);
}

=== Replace lines 1676-1682 ===
old:
                let window_radius = extent * extent;
                let mut records: Vec<(f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
                let q = [x0, y0, z0];
                sense_buffer(&field, q, t0, extent, &mut records);
                sense_buffer(&station_buf, q, t0, extent, &mut records);
new:
                let window_radius = extent * extent;
                let mut records: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
                let q = [x0, y0, z0];
                sense_buffer(&field, q, t0, &mut records);
                sense_buffer(&station_buf, q, t0, &mut records);
