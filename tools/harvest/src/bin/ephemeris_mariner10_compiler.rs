use omegaflow::archivar::astrometry::fk4_b1950_to_fk5_j2000;
use omegaflow::archivar::bsp_reader::daf::{DafFile, Summary};
use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::ephemeris::{
    CHEBYSHEV_DEGREE, GRANULE_DAYS, J2000_EPOCH, N_SAMPLES, chebyshev_fit, chebyshev_nodes,
    write_binary,
};
use omegaflow::lsk::days_from_civil;
use omegaflow::pck::PckBody;

const MARINER10_NAIF_ID: i32 = -76;
const MARINER10_NAME: &str = "mariner10";
const AU_KM: f64 = 149_597_870.7;
const DISCRETE_STATE_SLOT: usize = 16;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn swap_u32_at(data: &mut [u8], off: usize) {
    data[off..off + 4].reverse();
}

fn swap_words(data: &mut [u8], start: usize) {
    let mut i = start;
    while i + 8 <= data.len() {
        data[i..i + 8].reverse();
        i += 8;
    }
}

fn normalize_legacy(path: &str) -> Result<String, String> {
    let mut data = std::fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    if data.len() < 4096 || !data.len().is_multiple_of(1024) {
        return Err(format!(
            "{path}: {} bytes — no DAF record grid (1024 B), the normalize stays absent",
            data.len()
        ));
    }
    data[0..8].copy_from_slice(b"DAF/SPK ");
    swap_u32_at(&mut data, 8);
    swap_u32_at(&mut data, 12);
    swap_u32_at(&mut data, 76);
    swap_u32_at(&mut data, 80);
    swap_u32_at(&mut data, 84);
    data[88..96].copy_from_slice(b"LTL-IEEE");
    swap_words(&mut data, 3072);
    let shim = format!("{path}.dafnorm.bsp");
    std::fs::write(&shim, &data).map_err(|e| format!("write {shim} returned void: {e}"))?;
    Ok(shim)
}

fn open_daf(path: &str) -> Result<(DafFile, Option<String>), String> {
    match SpkFile::open(path) {
        Ok(_) => DafFile::open(path)
            .map(|d| (d, None))
            .map_err(|e| format!("daf open {path}: {e:?}")),
        Err(_) => {
            eprintln!(
                "{path}: legacy DAF — normalizing a temp copy (big-endian words to little-endian, record 1 stamps rewritten, comment records 2-3 untouched); the segment bytes stay untouched in content"
            );
            let shim = normalize_legacy(path)?;
            let daf = DafFile::open(&shim).map_err(|e| format!("daf open {shim}: {e:?}"))?;
            Ok((daf, Some(shim)))
        }
    }
}

#[derive(Clone)]
struct ChebSeg {
    target: i32,
    center: i32,
    frame: i32,
    start_et: f64,
    end_et: f64,
    init: f64,
    intlen: f64,
    rsize: usize,
    n_records: usize,
    n_coef: usize,
    start_addr: u32,
}

fn cheb_seg(daf: &DafFile, s: &Summary) -> Option<ChebSeg> {
    if s.doubles.len() < 2 || s.integers.len() < 6 {
        return None;
    }
    let addr_a = s.integers[4] as u32;
    let addr_b = s.integers[5] as u32;
    let (start_addr, end_addr) = if addr_a <= addr_b {
        (addr_a, addr_b)
    } else {
        (addr_b, addr_a)
    };
    let trailer = daf.read_doubles(end_addr - 3, end_addr).ok()?;
    let init = trailer[0];
    let intlen = trailer[1];
    let rsize = trailer[2] as usize;
    let n_records = trailer[3] as usize;
    if rsize < 5 || !(rsize - 2).is_multiple_of(3) || n_records == 0 || intlen <= 0.0 {
        return None;
    }
    Some(ChebSeg {
        target: s.integers[0],
        center: s.integers[1],
        frame: s.integers[2],
        start_et: s.doubles[0],
        end_et: s.doubles[1],
        init,
        intlen,
        rsize,
        n_records,
        n_coef: (rsize - 2) / 3,
        start_addr,
    })
}

fn cheby_eval(n: usize, c: &[f64], s: f64) -> (f64, f64) {
    let mut t0 = 1.0f64;
    let mut t1 = s;
    let mut d0 = 0.0f64;
    let mut d1 = 1.0f64;
    let mut val = c[0] + c[1] * s;
    let mut slope = c[1];
    for k in 2..n {
        let t2 = 2.0 * s * t1 - t0;
        let d2 = 2.0 * t1 + 2.0 * s * d1 - d0;
        val += c[k] * t2;
        slope += c[k] * d2;
        t0 = t1;
        t1 = t2;
        d0 = d1;
        d1 = d2;
    }
    (val, slope)
}

fn cheb_seg_state(daf: &DafFile, seg: &ChebSeg, et: f64) -> Option<[f64; 6]> {
    let raw_idx = ((et - seg.init) / seg.intlen).floor() as isize;
    let idx = raw_idx.clamp(0, seg.n_records as isize - 1) as usize;
    let rec_start = seg.start_addr + (idx * seg.rsize) as u32;
    let rec = daf
        .read_doubles(rec_start, rec_start + seg.rsize as u32 - 1)
        .ok()?;
    let mid = rec[0];
    let radius = rec[1];
    if radius == 0.0 {
        return None;
    }
    let s = (et - mid) / radius;
    let n = seg.n_coef;
    let xc = &rec[2..2 + n];
    let yc = &rec[2 + n..2 + 2 * n];
    let zc = &rec[2 + 2 * n..2 + 3 * n];
    let (px, vx) = cheby_eval(n, xc, s);
    let (py, vy) = cheby_eval(n, yc, s);
    let (pz, vz) = cheby_eval(n, zc, s);
    let inv_r = 1.0 / radius;
    Some([px, vx * inv_r, py, vy * inv_r, pz, vz * inv_r])
}

fn discrete_states(daf: &DafFile, seg: &Summary) -> Option<(i32, i32, i32, Vec<(f64, [f64; 6])>)> {
    if seg.doubles.len() < 2 || seg.integers.len() < 6 {
        return None;
    }
    let addr_a = seg.integers[4] as u32;
    let addr_b = seg.integers[5] as u32;
    let (start_addr, end_addr) = if addr_a <= addr_b {
        (addr_a, addr_b)
    } else {
        (addr_b, addr_a)
    };
    let data = daf.read_doubles(start_addr, end_addr).ok()?;
    let (lo, hi) = (seg.doubles[0] - 1e6, seg.doubles[1] + 1e6);
    let mut out: Vec<(f64, [f64; 6])> = Vec::new();
    let mut prev = f64::NEG_INFINITY;
    for i in 0..data.len().saturating_sub(DISCRETE_STATE_SLOT + 6) {
        let epoch = data[i];
        if !epoch.is_finite() || epoch < lo || epoch > hi || epoch <= prev {
            continue;
        }
        let st: [f64; 6] = data[i + DISCRETE_STATE_SLOT..i + DISCRETE_STATE_SLOT + 6]
            .try_into()
            .ok()?;
        let pos = (st[0] * st[0] + st[2] * st[2] + st[4] * st[4]).sqrt();
        let vel = (st[1] * st[1] + st[3] * st[3] + st[5] * st[5]).sqrt();
        if !st.iter().all(|v| v.is_finite()) || pos > 2.0e8 || vel > 80.0 {
            continue;
        }
        prev = epoch;
        out.push((epoch, st));
    }
    if out.len() < 2 {
        eprintln!(
            "  discrete {}..{}: {} doubles, scan found {} (epoch, state) pairs",
            start_addr,
            end_addr,
            data.len(),
            out.len()
        );
        return None;
    }
    Some((seg.integers[0], seg.integers[1], seg.integers[2], out))
}

fn rot_apply(m: &[f64; 9], v: [f64; 6]) -> [f64; 6] {
    let p = [v[0], v[2], v[4]];
    let s = [v[1], v[3], v[5]];
    let mut rp = [0.0f64; 3];
    let mut rs = [0.0f64; 3];
    for k in 0..3 {
        rp[k] = m[k * 3] * p[0] + m[k * 3 + 1] * p[1] + m[k * 3 + 2] * p[2];
        rs[k] = m[k * 3] * s[0] + m[k * 3 + 1] * s[1] + m[k * 3 + 2] * s[2];
    }
    [rp[0], rs[0], rp[1], rs[1], rp[2], rs[2]]
}

fn neg(v: [f64; 6]) -> [f64; 6] {
    let mut out = [0.0f64; 6];
    for k in 0..6 {
        out[k] = -v[k];
    }
    out
}

fn add(a: [f64; 6], b: [f64; 6]) -> [f64; 6] {
    let mut out = [0.0f64; 6];
    for k in 0..6 {
        out[k] = a[k] + b[k];
    }
    out
}

fn interp(states: &[(f64, [f64; 6])], et: f64) -> Option<[f64; 6]> {
    if et < states[0].0 || et > states[states.len() - 1].0 {
        return None;
    }
    let mut lo = 0usize;
    let mut hi = states.len() - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if states[mid].0 <= et {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    if states[hi].0 == states[lo].0 {
        return Some(states[lo].1);
    }
    let f = (et - states[lo].0) / (states[hi].0 - states[lo].0);
    let mut out = [0.0f64; 6];
    for k in 0..6 {
        out[k] = states[lo].1[k] + f * (states[hi].1[k] - states[lo].1[k]);
    }
    Some(out)
}

fn state_over(
    segs: &[ChebSeg],
    target: i32,
    center: i32,
    et: f64,
    daf: &DafFile,
) -> Option<[f64; 6]> {
    for seg in segs
        .iter()
        .filter(|s| s.target == target && s.center == center)
    {
        if et >= seg.start_et && et <= seg.end_et {
            return cheb_seg_state(daf, seg, et);
        }
    }
    None
}

fn run(args: &[String]) -> Result<(), String> {
    let Some(kernel) = arg_value(args, "--kernel") else {
        return Err(
            "usage: ephemeris_mariner10_compiler --kernel <M10_archive_1.bsp> \
             [--with-kernel <planets.bsp>] [--out <ephemeris_mariner10.bin>] \
             [--ci-mode] [--list] [--summaries] [--peek <path>] — refused"
                .into(),
        );
    };
    let (daf, shim) = open_daf(&kernel)?;
    if let Some(shim_path) = &shim {
        eprintln!("normalized copy: {shim_path}");
    }
    let sums = daf.summaries().map_err(|e| format!("summaries: {e:?}"))?;
    let mut chebs: Vec<ChebSeg> = Vec::new();
    let mut discretes: Vec<(i32, i32, i32, Vec<(f64, [f64; 6])>)> = Vec::new();
    for s in &sums {
        if let Some(seg) = cheb_seg(&daf, s) {
            chebs.push(seg);
        } else if let Some(d) = discrete_states(&daf, s) {
            discretes.push(d);
        } else {
            return Err(format!(
                "summary unread: doubles {:?} ints {:?} — the anchor stays unwritten",
                s.doubles, s.integers
            ));
        }
    }
    for seg in &chebs {
        eprintln!(
            "  cheb target {} center {} frame {} N {} records {} et {:.3}..{:.3} JD",
            seg.target,
            seg.center,
            seg.frame,
            seg.n_coef,
            seg.n_records,
            seg.start_et / 86400.0 + J2000_EPOCH,
            seg.end_et / 86400.0 + J2000_EPOCH
        );
    }
    for (target, center, frame, states) in &discretes {
        eprintln!(
            "  discrete target {} center {} frame {} states {} et {:.3}..{:.3} JD",
            target,
            center,
            frame,
            states.len(),
            states[0].0 / 86400.0 + J2000_EPOCH,
            states[states.len() - 1].0 / 86400.0 + J2000_EPOCH
        );
    }

    let ssb_wrt: Vec<&ChebSeg> = chebs.iter().filter(|s| s.target == 0).collect();
    let body_m10: Vec<&(i32, i32, i32, Vec<(f64, [f64; 6])>)> = discretes
        .iter()
        .filter(|(_, c, _, _)| *c == MARINER10_NAIF_ID)
        .collect();
    if ssb_wrt.is_empty() || body_m10.is_empty() {
        return Err(format!(
            "{kernel}: no SSB-wrt-body or no body-wrt-M10 segment — the anchor stays unwritten"
        ));
    }
    for s in ssb_wrt.iter() {
        if s.frame != 2 {
            return Err(format!(
                "SSB-wrt segment target {} center {} carries frame {} — the anchor reads B1950 (2)",
                s.target, s.center, s.frame
            ));
        }
    }
    for (_, _, frame, _) in body_m10.iter() {
        if *frame != 1 {
            return Err(format!(
                "body-wrt-M10 segment carries frame {} — the anchor reads J2000 (1)",
                frame
            ));
        }
    }

    let d_min = body_m10
        .iter()
        .flat_map(|(_, _, _, s)| s.iter())
        .map(|(e, _)| *e)
        .fold(f64::MAX, f64::min);
    let d_max = body_m10
        .iter()
        .flat_map(|(_, _, _, s)| s.iter())
        .map(|(e, _)| *e)
        .fold(f64::MIN, f64::max);
    let c_min = ssb_wrt.iter().map(|s| s.start_et).fold(f64::MAX, f64::min);
    let c_max = ssb_wrt.iter().map(|s| s.end_et).fold(f64::MIN, f64::max);
    let min_et = d_min.max(c_min);
    let max_et = d_max.min(c_max);
    if max_et <= min_et {
        return Err(
            "discrete-state and barycenter coverage do not intersect — the anchor stays unwritten"
                .into(),
        );
    }
    eprintln!(
        "coverage: {:.3}..{:.3} JD ({} days)",
        min_et / 86400.0 + J2000_EPOCH,
        max_et / 86400.0 + J2000_EPOCH,
        (max_et - min_et) / 86400.0
    );

    let rot = fk4_b1950_to_fk5_j2000();
    let body_state_ssb = |et: f64| -> Option<[f64; 6]> {
        for (body_id, _, _, states) in body_m10.iter() {
            let body_wrt_m10 = match interp(states, et) {
                Some(st) => st,
                None => continue,
            };
            let body_wrt_ssb_b1950 = state_over(&chebs, 0, *body_id, et, &daf)?;
            let body_ssb = rot_apply(&rot, body_wrt_ssb_b1950);
            return Some(add(neg(body_wrt_m10), body_ssb));
        }
        None
    };

    if args.iter().any(|a| a == "--list") {
        return Ok(());
    }

    let mut probe_times: Vec<(String, f64)> = Vec::new();
    for (label, y, mo, d) in [
        ("1974-03-24", 1974, 3, 24),
        ("1974-03-26", 1974, 3, 26),
        ("1974-03-28", 1974, 3, 28),
        ("1974-03-31", 1974, 3, 31),
    ] {
        let jd = days_from_civil(y, mo, d).ok_or("civil date reads void")? as f64 + 2440587.5;
        probe_times.push((label.to_string(), (jd - J2000_EPOCH) * 86400.0));
    }
    for k in 0..5 {
        probe_times.push((
            format!("coverage {}/5", k + 1),
            min_et + (max_et - min_et) * (k as f64 + 0.5) / 5.0,
        ));
    }
    for (label, et) in &probe_times {
        match body_state_ssb(*et) {
            Some(st) => {
                let r = (st[0] * st[0] + st[2] * st[2] + st[4] * st[4]).sqrt();
                let v = (st[1] * st[1] + st[3] * st[3] + st[5] * st[5]).sqrt();
                eprintln!(
                    "  {label} (JD {:.2}): r = {:.5} AU, v = {:.2} km/s, st = [{:.0}, {:.3}, {:.0}, {:.3}, {:.0}, {:.3}]",
                    *et / 86400.0 + J2000_EPOCH,
                    r / AU_KM,
                    v,
                    st[0],
                    st[1],
                    st[2],
                    st[3],
                    st[4],
                    st[5]
                );
                if !(0.2..=0.8).contains(&(r / AU_KM)) || !(30.0..=60.0).contains(&v) {
                    return Err(format!(
                        "{label}: r/v outside the trajectory band — the anchor stays unwritten"
                    ));
                }
            }
            None => {
                eprintln!(
                    "  probe {label}: et {et}, ssb_wrt_sun {:?} ssb_wrt_mercury {:?}",
                    state_over(&chebs, 0, 10, *et, &daf).map(|_| "some"),
                    state_over(&chebs, 0, 1, *et, &daf).map(|_| "some")
                );
                return Err(format!(
                    "{label}: state reads void — the anchor stays unwritten"
                ));
            }
        }
    }

    if let Some(planets_path) = arg_value(args, "--with-kernel") {
        let planets = SpkFile::open(&planets_path)
            .map_err(|e| format!("open planet kernel {planets_path}: {e:?}"))?;
        let mut worst = 0.0f64;
        let mut n = 0usize;
        for k in 0..40 {
            let et = min_et + (max_et - min_et) * (k as f64 + 0.5) / 40.0;
            let Some(kernel_sun) = state_over(&chebs, 0, 10, et, &daf) else {
                continue;
            };
            let Some(eph_sun) = planets.state(10, 0, et).ok() else {
                continue;
            };
            let sun_from_kernel = rot_apply(&rot, kernel_sun);
            let delta: f64 = [
                (sun_from_kernel[0] - eph_sun[0]).powi(2),
                (sun_from_kernel[2] - eph_sun[1]).powi(2),
                (sun_from_kernel[4] - eph_sun[2]).powi(2),
            ]
            .iter()
            .sum::<f64>()
            .sqrt();
            if delta > worst {
                worst = delta;
            }
            n += 1;
        }
        eprintln!(
            "sun-from-M10-kernel vs {planets_path}: {n} probes, worst |delta| {:.0} km",
            worst
        );
    }

    let mid_et = (min_et + max_et) / 2.0;
    let half_et = (max_et - min_et) / 2.0;
    let mut samples: Vec<(f64, f64, f64)> = Vec::with_capacity(N_SAMPLES);
    for tau in &chebyshev_nodes(N_SAMPLES) {
        let et = mid_et + tau * half_et;
        match body_state_ssb(et) {
            Some(st) => samples.push((st[0] * 1000.0, st[2] * 1000.0, st[4] * 1000.0)),
            None => {
                return Err(format!(
                    "granule node JD {:.4} reads void — the anchor stays unwritten",
                    et / 86400.0 + J2000_EPOCH
                ));
            }
        }
    }
    let (cx, cy, cz) = chebyshev_fit(&samples, CHEBYSHEV_DEGREE)
        .ok_or("the Chebyshev fit refused — the anchor stays unwritten")?;
    let granules = vec![(
        mid_et / 86400.0 + J2000_EPOCH,
        half_et / 86400.0,
        cx,
        cy,
        cz,
    )];
    eprintln!("granules: 1 ({} days wide)", 2.0 * half_et / 86400.0);
    let _ = GRANULE_DAYS;

    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("ephemeris_{MARINER10_NAME}.bin"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let absent_pck = PckBody::absent();
    if !write_binary(&out, MARINER10_NAME, &granules, &[], &[], &absent_pck, None) {
        return Err(format!("{out}: write returned void"));
    }
    let bytes = std::fs::read(&out).map_err(|e| format!("read {out}: {e}"))?;
    let Some(eph) = omegaflow::archivar::motion::parse_ephemeris_binary(&bytes) else {
        return Err(format!(
            "{out}: parse_ephemeris_binary reads void — the asset stays unwritten"
        ));
    };
    let mut map = std::collections::HashMap::new();
    map.insert(MARINER10_NAME.to_string(), eph);
    let mut max_delta_m = 0.0f64;
    let mut probe_n = 0usize;
    for k in 0..12 {
        let et = min_et + (max_et - min_et) * (k as f64 + 0.5) / 12.0;
        let Some(st) = body_state_ssb(et) else {
            continue;
        };
        let Some(p) =
            omegaflow::archivar::motion::body_barycenter_position(MARINER10_NAME, et, &map)
        else {
            continue;
        };
        let d = ((p[0] - st[0] * 1000.0).powi(2)
            + (p[1] - st[2] * 1000.0).powi(2)
            + (p[2] - st[4] * 1000.0).powi(2))
        .sqrt();
        if d > max_delta_m {
            max_delta_m = d;
        }
        probe_n += 1;
    }
    eprintln!(
        "{out}: {} B, sha256 {} — roundtrip verified on {probe_n} probes, max granule-vs-source delta {:.0} m",
        bytes.len(),
        sha256_hex(&bytes),
        max_delta_m
    );
    eprintln!("CDN asset: {}", body_url(MARINER10_NAME));
    if ci_mode && !upload_release("ssd.jpl.nasa.gov-ephemeris", &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn dump(path: &str, n: usize) -> Result<(), String> {
    let data = std::fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let take = n.min(data.len());
    for (i, row) in data[..take].chunks(16).enumerate() {
        let hex: Vec<String> = row.iter().map(|b| format!("{b:02x}")).collect();
        let ascii: String = row
            .iter()
            .map(|&b| {
                if (0x20..0x7f).contains(&b) {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();
        eprintln!("{:04x}  {:<48}  {}", i * 16, hex.join(" "), ascii);
    }
    Ok(())
}

fn summaries(path: &str) -> Result<(), String> {
    let (daf, _) = open_daf(path)?;
    let sums = daf.summaries().map_err(|e| format!("summaries: {e:?}"))?;
    eprintln!("{} summary record(s)", sums.len());
    for s in sums {
        eprintln!(
            "  doubles {:?} ints {:?} name {:?}",
            s.doubles, s.integers, s.name
        );
    }
    Ok(())
}

fn peek(path: &str, start: u32, end: u32) -> Result<(), String> {
    let (daf, _) = open_daf(path)?;
    let data = daf
        .read_doubles(start, end)
        .map_err(|e| format!("read_doubles({start},{end}): {e:?}"))?;
    eprintln!("doubles {start}..{end}: {} values", data.len());
    for (i, v) in data.iter().enumerate() {
        eprintln!("  [{:>4}] {:.6}", start as usize + i, v);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = arg_value(&args, "--dump") {
        let n = match arg_value(&args, "--bytes").and_then(|v| v.parse().ok()) {
            Some(v) => v,
            None => 128,
        };
        if let Err(msg) = dump(&path, n) {
            eprintln!("ephemeris_mariner10_compiler: {msg}");
            std::process::exit(1);
        }
        return;
    }
    if let Some(path) = arg_value(&args, "--summaries") {
        if let Err(msg) = summaries(&path) {
            eprintln!("ephemeris_mariner10_compiler: {msg}");
            std::process::exit(1);
        }
        return;
    }
    if let Some(path) = arg_value(&args, "--peek") {
        let start = match arg_value(&args, "--start").and_then(|v| v.parse::<u32>().ok()) {
            Some(v) => v,
            None => 0,
        };
        let end = match arg_value(&args, "--end").and_then(|v| v.parse::<u32>().ok()) {
            Some(v) => v,
            None => start,
        };
        if let Err(msg) = peek(&path, start, end) {
            eprintln!("ephemeris_mariner10_compiler: {msg}");
            std::process::exit(1);
        }
        return;
    }
    if let Err(msg) = run(&args) {
        eprintln!("ephemeris_mariner10_compiler: {msg}");
        std::process::exit(1);
    }
}
