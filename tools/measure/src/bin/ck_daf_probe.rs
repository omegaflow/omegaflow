use omegaflow::bsp_reader::daf::{DafError, DafFile, Summary, DOUBLE_BYTES, RECORD_BYTES};
use std::collections::HashMap;

const EGA_LO: f64 = -286_200_000.0;
const EGA_HI: f64 = -285_854_400.0;
const JD_J2000: f64 = 2451545.0;
const JD_1970: f64 = 2440587.5;

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn et_to_date(et: f64) -> String {
    let jd = JD_J2000 + et / 86_400.0;
    let dn = jd - JD_1970;
    let days = dn.floor() as i64;
    let frac = dn - days as f64;
    let (y, m, d) = civil_from_days(days);
    let secs = (frac * 86_400.0).round() as i64;
    let hh = secs / 3600;
    let mm = (secs % 3600) / 60;
    let ss = secs % 60;
    format!("{y:04}-{m:02}-{d:02} ~{hh:02}:{mm:02}:{ss:02}")
}

fn load_sclk_breaks(path: &str) -> Option<Vec<(f64, f64)>> {
    let text = std::fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = text.lines().collect();
    let mut start = None;
    for (i, l) in lines.iter().enumerate() {
        if l.contains("SCLK01_COEFFICIENTS_77") {
            start = Some(i);
            break;
        }
    }
    let s = start?;
    let mut buf: Vec<f64> = Vec::new();
    for l in &lines[s + 1..] {
        let t = l.trim();
        if t == ")" {
            break;
        }
        for tok in l.split_whitespace() {
            if let Ok(v) = tok.parse::<f64>() {
                buf.push(v);
            }
        }
    }
    let mut brk: Vec<(f64, f64)> = Vec::new();
    for c in buf.chunks_exact(3) {
        brk.push((c[0], c[1]));
    }
    brk.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    if brk.len() >= 2 {
        Some(brk)
    } else {
        None
    }
}

fn tick_to_et(t: f64, brk: &[(f64, f64)]) -> f64 {
    if t <= brk[0].0 {
        return brk[0].1;
    }
    let last = brk[brk.len() - 1];
    if t >= last.0 {
        return last.1;
    }
    let mut lo = 0usize;
    let mut hi = brk.len() - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if brk[mid].0 <= t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let (x0, y0) = brk[lo];
    let (x1, y1) = brk[hi];
    y0 + (y1 - y0) * (t - x0) / (x1 - x0)
}

fn be_f64(b: &[u8]) -> f64 {
    let mut a = [0u8; 8];
    a.copy_from_slice(b);
    f64::from_be_bytes(a)
}

fn be_i32(b: &[u8]) -> i32 {
    let mut a = [0u8; 4];
    a.copy_from_slice(b);
    i32::from_be_bytes(a)
}

// BIG-IEEE DAF reader: the daily GLL CK products are big-endian (idword
// "NAIF/DAF"), while the crate DafFile reads LTL-IEEE only.  The record and
// summary layout mirrors daf.rs with the file's native byte order swapped.
struct BigDaf {
    data: Vec<u8>,
    nd: u32,
    ni: u32,
    fward: u32,
}

impl BigDaf {
    fn from_data(data: Vec<u8>) -> Result<Self, DafError> {
        if data.len() < RECORD_BYTES {
            return Err(DafError::TooSmall(data.len()));
        }
        if &data[88..96] != b"BIG-IEEE" {
            return Err(DafError::UnsupportedFormat([
                data[88], data[89], data[90], data[91], data[92], data[93], data[94], data[95],
            ]));
        }
        let nd = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
        let ni = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);
        let fward = u32::from_be_bytes([data[76], data[77], data[78], data[79]]);
        Ok(BigDaf { data, nd, ni, fward })
    }

    fn idword(&self) -> [u8; 8] {
        let mut a = [0u8; 8];
        a.copy_from_slice(&self.data[0..8]);
        a
    }

    fn nd(&self) -> u32 {
        self.nd
    }

    fn ni(&self) -> u32 {
        self.ni
    }

    fn summary_size_doubles(&self) -> usize {
        self.nd as usize + (self.ni as usize).div_ceil(2)
    }

    fn record_bytes(&self, rec: u32) -> Result<&[u8], DafError> {
        let start = (rec as usize - 1) * RECORD_BYTES;
        let end = start + RECORD_BYTES;
        if end > self.data.len() {
            return Err(DafError::BadSummary {
                record: rec,
                reason: "record extends past end of file",
            });
        }
        Ok(&self.data[start..end])
    }

    fn next_record(&self, rec: u32) -> Result<u32, DafError> {
        let b = self.record_bytes(rec)?;
        Ok(be_f64(&b[0..8]) as u32)
    }

    fn read_summary_record(&self, rec: u32, out: &mut Vec<Summary>) -> Result<(), DafError> {
        let sbytes = self.record_bytes(rec)?;
        let nbytes = self.record_bytes(rec + 1)?;
        let nsum = be_f64(&sbytes[16..24]) as usize;
        let ss = self.summary_size_doubles();
        let nd = self.nd as usize;
        let ni = self.ni as usize;
        let name_chars = ss * DOUBLE_BYTES;
        for i in 0..nsum {
            let soff = 24 + i * ss * DOUBLE_BYTES;
            if soff + ss * DOUBLE_BYTES > sbytes.len() {
                return Err(DafError::BadSummary {
                    record: rec,
                    reason: "summary past end of record",
                });
            }
            let sslice = &sbytes[soff..soff + ss * DOUBLE_BYTES];
            let mut doubles = Vec::with_capacity(nd);
            for k in 0..nd {
                doubles.push(be_f64(&sslice[k * DOUBLE_BYTES..(k + 1) * DOUBLE_BYTES]));
            }
            let mut integers = Vec::with_capacity(ni);
            let int_start = nd * DOUBLE_BYTES;
            for k in 0..ni {
                integers.push(be_i32(&sslice[int_start + k * 4..int_start + (k + 1) * 4]));
            }
            let noff = i * name_chars;
            let name_slice = &nbytes[noff..noff + name_chars];
            let name = std::str::from_utf8(name_slice)
                .unwrap_or("")
                .trim_end_matches('\0')
                .trim_end()
                .to_string();
            out.push(Summary {
                doubles,
                integers,
                name,
            });
        }
        Ok(())
    }

    fn summaries(&self) -> Result<Vec<Summary>, DafError> {
        let mut out = Vec::new();
        let mut rec = self.fward;
        while rec != 0 {
            self.read_summary_record(rec, &mut out)?;
            rec = self.next_record(rec)?;
        }
        Ok(out)
    }

    fn read_doubles(&self, start_addr: u32, end_addr: u32) -> Result<Vec<f64>, DafError> {
        if start_addr == 0 || end_addr < start_addr {
            return Err(DafError::AddressOutOfBounds {
                start: start_addr,
                end: end_addr,
                file_doubles: (self.data.len() / DOUBLE_BYTES) as u64,
            });
        }
        let byte_start = (start_addr as usize - 1) * DOUBLE_BYTES;
        let byte_end = end_addr as usize * DOUBLE_BYTES;
        if byte_end > self.data.len() {
            return Err(DafError::AddressOutOfBounds {
                start: start_addr,
                end: end_addr,
                file_doubles: (self.data.len() / DOUBLE_BYTES) as u64,
            });
        }
        let n = (end_addr - start_addr + 1) as usize;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let off = byte_start + i * DOUBLE_BYTES;
            out.push(be_f64(&self.data[off..off + 8]));
        }
        Ok(out)
    }
}

fn dump(idx: usize, s: &Summary, brk: &[(f64, f64)]) {
    let dc: Vec<String> = s.doubles.iter().map(|v| format!("{v:.6e}")).collect();
    let ic: Vec<String> = s.integers.iter().map(|v| v.to_string()).collect();
    let date = if s.doubles.len() >= 1 {
        et_to_date(tick_to_et(s.doubles[0], brk))
    } else {
        String::from("-")
    };
    println!(
        "[{idx}] DC=[{}] IC=[{}] name='{}' dc0~{date}",
        dc.join(", "),
        ic.join(", "),
        s.name
    );
}

fn histo(summaries: &[Summary], slot: usize) {
    let mut m: HashMap<i32, usize> = HashMap::new();
    for s in summaries {
        if let Some(&v) = s.integers.get(slot) {
            *m.entry(v).or_insert(0) += 1;
        }
    }
    let mut v: Vec<(i32, usize)> = m.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    let top: Vec<String> = v.iter().take(6).map(|(k, c)| format!("{k}:{c}")).collect();
    println!("IC[{slot}] histogram: {}", top.join("  "));
}

struct RotorSeg {
    dtype: i32,
    reclen: usize,
    np: usize,
    theta: f64,
    dt: f64,
    acc: usize,
    nint: usize,
    bad_norm: usize,
    median_dt: f64,
    seg_dt: f64,
}

fn decode_rotor_segment(daf: &BigDaf, idx: usize, s: &Summary, brk: &[(f64, f64)]) -> Option<RotorSeg> {
    if s.integers.len() < 6 {
        println!("  seg [{idx}] descriptor carries {} integers, needs 6", s.integers.len());
        return None;
    }
    let frame = s.integers[0];
    let dtype = s.integers[2];
    if frame != -77000 {
        return None;
    }
    let a0 = s.integers[4] as u32;
    let a1 = s.integers[5] as u32;
    let dbls = match daf.read_doubles(a0, a1) {
        Ok(v) => v,
        Err(e) => {
            println!("  seg [{idx}] payload read: {e}");
            return None;
        }
    };
    let total = dbls.len();
    if total < 8 {
        println!("  seg [{idx}] payload too short ({total} doubles)");
        return None;
    }
    let last = dbls[total - 1];
    let np = last.round() as i64;
    if np < 2 || (last - np as f64).abs() > 1e-6 {
        println!("  seg [{idx}] no layout: trailing value {last:.6e} is not a pointing-instance count");
        return None;
    }
    let npu = np as usize;
    let mut reclen = 0usize;
    if dtype == 1 {
        let dir = (npu - 1) / 100;
        for r in [4usize, 7] {
            if npu * r + npu + dir + 1 == total {
                reclen = r;
                break;
            }
        }
    } else if dtype == 3 {
        let s2 = dbls[total - 2];
        let numint = s2.round() as i64;
        if numint >= 1 && (s2 - numint as f64).abs() < 1e-6 {
            let niu = numint as usize;
            let dir = (npu - 1) / 100;
            let sdir = (niu - 1) / 100;
            for r in [4usize, 7] {
                if npu * r + npu + dir + niu + sdir + 2 == total {
                    reclen = r;
                    break;
                }
            }
        }
    }
    if reclen == 0 {
        let head: Vec<String> = dbls.iter().take(6).map(|x| format!("{x:.4e}")).collect();
        let tail: Vec<String> = dbls.iter().rev().take(4).map(|x| format!("{x:.4e}")).collect();
        println!(
            "  seg [{idx}] dtype {dtype} geometry unresolved: total {total} np {npu}; head [{}] tail [{}]",
            head.join(", "),
            tail.join(", ")
        );
        return None;
    }
    if brk.is_empty() {
        println!("  seg [{idx}] sclk breakpoints absent: spin decode needs the tsc kernel");
        return None;
    }
    let base = npu * reclen;
    let mut et: Vec<f64> = Vec::with_capacity(npu);
    let mut q: Vec<[f64; 4]> = Vec::with_capacity(npu);
    let mut bad_norm = 0usize;
    for i in 0..npu {
        let off = i * reclen;
        let qq = [dbls[off], dbls[off + 1], dbls[off + 2], dbls[off + 3]];
        let nrm = (qq[0] * qq[0] + qq[1] * qq[1] + qq[2] * qq[2] + qq[3] * qq[3]).sqrt();
        if (nrm - 1.0).abs() > 1e-6 {
            bad_norm += 1;
        }
        q.push(qq);
        et.push(tick_to_et(dbls[base + i], brk));
    }
    let nint = npu - 1;
    let mut dts: Vec<f64> = Vec::with_capacity(nint);
    for i in 0..nint {
        let d = et[i + 1] - et[i];
        if d > 0.0 {
            dts.push(d);
        }
    }
    if dts.is_empty() {
        println!("  seg [{idx}] no increasing sclk times in the pointing instances");
        return None;
    }
    dts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median_dt = dts[dts.len() / 2];
    if !median_dt.is_finite() || median_dt <= 0.0 {
        println!("  seg [{idx}] sclk times not decodable to increasing seconds");
        return None;
    }
    let cap = (median_dt * 3.0).min(9.0);
    let mut theta_sum = 0.0f64;
    let mut dt_sum = 0.0f64;
    let mut acc = 0usize;
    for i in 0..nint {
        let dt = et[i + 1] - et[i];
        if dt <= 0.0 || dt > cap {
            continue;
        }
        let dot = (q[i][0] * q[i + 1][0]
            + q[i][1] * q[i + 1][1]
            + q[i][2] * q[i + 1][2]
            + q[i][3] * q[i + 1][3])
        .abs();
        let th = 2.0 * dot.min(1.0).acos();
        if th < 1e-9 || th > std::f64::consts::PI - 0.05 {
            continue;
        }
        theta_sum += th;
        dt_sum += dt;
        acc += 1;
    }
    if dt_sum <= 0.0 || theta_sum <= 0.0 {
        println!("  seg [{idx}] no usable intervals (median_dt {median_dt:.4}s, cap {cap:.3}s)");
        return None;
    }
    Some(RotorSeg {
        dtype,
        reclen,
        np: npu,
        theta: theta_sum,
        dt: dt_sum,
        acc,
        nint,
        bad_norm,
        median_dt,
        seg_dt: et[npu - 1] - et[0],
    })
}

fn rotor_main(cks: &[String], tsc: Option<&str>) {
    let brk = tsc.and_then(load_sclk_breaks);
    match &brk {
        Some(b) => println!(
            "sclk breakpoints loaded: {} (tsc={})",
            b.len(),
            tsc.as_deref().unwrap_or("")
        ),
        None => println!("sclk breakpoints: none (spin decode in seconds requires the tsc kernel)"),
    }
    let mut w_segs = 0usize;
    let mut w_np = 0usize;
    let mut w_theta = 0.0f64;
    let mut w_dt = 0.0f64;
    for path in cks {
        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                println!("{path}: read: {e}");
                continue;
            }
        };
        let daf = match BigDaf::from_data(data.clone()) {
            Ok(d) => d,
            Err(e) => {
                println!("{path}: big-endian DAF header: {e}");
                continue;
            }
        };
        println!("\n=== rotor ck {path} ({}) ===", data.len());
        println!(
            "idword ascii='{}' nd={} ni={} summary_size_doubles={}",
            String::from_utf8_lossy(&daf.idword()).trim_end(),
            daf.nd(),
            daf.ni(),
            daf.summary_size_doubles()
        );
        let summaries = match daf.summaries() {
            Ok(s) => s,
            Err(e) => {
                println!("{path}: summary walk: {e}");
                continue;
            }
        };
        println!("total summaries: {}", summaries.len());
        if !summaries.is_empty() {
            histo(&summaries, 0);
            histo(&summaries, 2);
        }
        if let Some(b) = &brk {
            let mut min_et = f64::INFINITY;
            let mut max_et = f64::NEG_INFINITY;
            for s in &summaries {
                if let Some(&v) = s.doubles.first() {
                    let e = tick_to_et(v, b);
                    if e < min_et {
                        min_et = e;
                    }
                }
                if let Some(&v) = s.doubles.get(1) {
                    let e = tick_to_et(v, b);
                    if e > max_et {
                        max_et = e;
                    }
                }
            }
            if min_et.is_finite() && max_et.is_finite() {
                println!("coverage {} .. {}", et_to_date(min_et), et_to_date(max_et));
            }
        }
        let rotor_total = summaries.iter().filter(|s| s.integers.first() == Some(&-77000)).count();
        println!("ROTOR frame -77000 carried by {} of {} segments", rotor_total, summaries.len());
        let mut f_segs = 0usize;
        let mut f_np = 0usize;
        let mut f_theta = 0.0f64;
        let mut f_dt = 0.0f64;
        for (i, s) in summaries.iter().enumerate() {
            let Some(st) = decode_rotor_segment(&daf, i, s, brk.as_deref().unwrap_or(&[])) else {
                continue;
            };
            f_segs += 1;
            f_np += st.np;
            f_theta += st.theta;
            f_dt += st.dt;
            let rate = st.theta / st.dt;
            let revs = st.theta / std::f64::consts::TAU;
            println!(
                "  seg[{i}] type={} reclen={} np={} intervals {}/{} badnorm={} median_dt={:.4}s cover={:.1}s acc_dt={:.1}s rate={rate:.9}rad/s revs={revs:.1}",
                st.dtype,
                st.reclen,
                st.np,
                st.acc,
                st.nint,
                st.bad_norm,
                st.median_dt,
                st.seg_dt,
                st.dt
            );
        }
        if f_dt > 0.0 && f_theta > 0.0 {
            let rate = f_theta / f_dt;
            let period = std::f64::consts::TAU / rate;
            println!(
                "FILE_SPIN segs={f_segs} np={f_np} sumdt={:.3}s sumtheta={:.3}rad revs={:.1} rate={rate:.9}rad/s period={period:.6}s rpm={:.6} mhz={:.6}",
                f_dt,
                f_theta,
                f_theta / std::f64::consts::TAU,
                rate * 60.0 / std::f64::consts::TAU,
                rate * 1000.0 / std::f64::consts::TAU
            );
            w_segs += f_segs;
            w_np += f_np;
            w_theta += f_theta;
            w_dt += f_dt;
        } else {
            println!("FILE_SPIN no decodable -77000 type-1/3 segment in this file");
        }
    }
    println!("\n=== window across {} daily rotor CKs ===", cks.len());
    if w_dt > 0.0 && w_theta > 0.0 {
        let rate = w_theta / w_dt;
        let period = std::f64::consts::TAU / rate;
        let rpm = rate * 60.0 / std::f64::consts::TAU;
        let mhz = rate * 1000.0 / std::f64::consts::TAU;
        println!(
            "WINDOW_SPIN segs={w_segs} np={w_np} sumdt={w_dt:.3}s sumtheta={w_theta:.3}rad revs={:.1}",
            w_theta / std::f64::consts::TAU
        );
        println!("WINDOW_SPIN rate={rate:.9} rad/s  period={period:.6} s  rpm={rpm:.6}  mhz={mhz:.6}");
        println!(
            "WINDOW_SPIN vs tone 19.10 s: ratio {:.6}, delta {:.4} s",
            period / 19.10,
            period - 19.10
        );
        println!(
            "WINDOW_SPIN vs tone 52.39 mHz: ratio {:.6}, delta {:.4} mHz",
            mhz / 52.39,
            mhz - 52.39
        );
        println!(
            "WINDOW_SPIN vs nominal 3.15 rpm: ratio {:.6}",
            rpm / 3.15
        );
    } else {
        println!("WINDOW_SPIN none");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cks: Vec<String> = Vec::new();
    let mut tsc_path: Option<String> = None;
    for a in args {
        if a.ends_with(".tsc") && tsc_path.is_none() {
            tsc_path = Some(a);
        } else {
            cks.push(a);
        }
    }
    let Some(path) = cks.first().cloned() else {
        println!("usage: ck_daf_probe <ck.bc ...> [gll.tsc]");
        return;
    };

    let is_big = std::fs::read(&path)
        .map(|b| b.len() >= 96 && &b[88..96] == b"BIG-IEEE")
        .unwrap_or(false);
    if is_big {
        rotor_main(&cks, tsc_path.as_deref());
        return;
    }

    let daf = match DafFile::open(&path) {
        Ok(d) => d,
        Err(e) => {
            println!("DafFile::open error: {e}");
            return;
        }
    };

    println!("path: {path}");
    println!(
        "idword: {:?}  ascii='{}'",
        daf.idword(),
        String::from_utf8_lossy(&daf.idword()).trim_end()
    );
    println!(
        "nd={} ni={} summary_size_doubles={}",
        daf.nd(),
        daf.ni(),
        daf.summary_size_doubles()
    );

    let brk = tsc_path.as_deref().and_then(load_sclk_breaks);
    match &brk {
        Some(b) => println!("sclk breakpoints loaded: {} (tsc={})", b.len(), tsc_path.as_deref().unwrap_or("")),
        None => println!("sclk breakpoints: none (coverage dates not decodable)"),
    }
    if brk.is_none() {
        println!("usage for date decode: ck_daf_probe <ck.bc> <gll.tsc>");
    }

    let summaries = match daf.summaries() {
        Ok(s) => s,
        Err(e) => {
            println!("summaries() error: {e}");
            return;
        }
    };
    let total = summaries.len();
    println!("total summaries: {total}");

    if total > 0 {
        for slot in 0..4 {
            histo(&summaries, slot);
        }
    }

    let brk = match &brk {
        Some(b) => b,
        None => {
            for (i, s) in summaries.iter().enumerate() {
                dump(i, s, &[]);
            }
            return;
        }
    };

    let mut min_et = f64::INFINITY;
    let mut max_et = f64::NEG_INFINITY;
    for s in &summaries {
        if let Some(&v) = s.doubles.first() {
            let e = tick_to_et(v, brk);
            if e < min_et {
                min_et = e;
            }
        }
        if let Some(&v) = s.doubles.get(1) {
            let e = tick_to_et(v, brk);
            if e > max_et {
                max_et = e;
            }
        }
    }
    if min_et.is_finite() && max_et.is_finite() {
        println!("coverage first summary start ~{}", et_to_date(min_et));
        println!("coverage last summary end   ~{}", et_to_date(max_et));
    }

    if total <= 300 {
        println!("-- full dump ({total}) --");
        for (i, s) in summaries.iter().enumerate() {
            dump(i, s, brk);
        }
    } else {
        println!("-- first 3 --");
        for i in 0..3 {
            dump(i, &summaries[i], brk);
        }
        println!("-- last 2 --");
        for i in (total - 2)..total {
            dump(i, &summaries[i], brk);
        }
    }

    let mut ega: Vec<usize> = Vec::new();
    for (i, s) in summaries.iter().enumerate() {
        if s.doubles.len() < 2 {
            continue;
        }
        let e0 = tick_to_et(s.doubles[0], brk);
        let e1 = tick_to_et(s.doubles[1], brk);
        if e1 >= EGA_LO && e0 <= EGA_HI {
            ega.push(i);
        }
    }

    if ega.is_empty() {
        println!("verdict: NO summary overlaps EGA-1 window 1990-12-07 00:00 .. 1990-12-11 00:00 (et [{EGA_LO:.0},{EGA_HI:.0}])");
    } else {
        println!("verdict: {} summary/summaries overlap EGA-1 window:", ega.len());
        for &i in &ega {
            let s = &summaries[i];
            let e0 = tick_to_et(s.doubles[0], brk);
            let e1 = tick_to_et(s.doubles[1], brk);
            println!(
                "  segment [{i}] {} .. {}  (ticks {:.0} .. {:.0})  frame={} IC={:?}",
                et_to_date(e0),
                et_to_date(e1),
                s.doubles[0],
                s.doubles[1],
                s.integers[0],
                &s.integers[..4]
            );
            let a0 = s.integers[4] as u32;
            let a1 = s.integers[5] as u32;
            let n = (a1 - a0 + 1) as usize;
            let peek = daf.read_doubles(a0, a0 + (n.min(24) as u32) - 1);
            match peek {
                Ok(v) => {
                    let vals: Vec<String> = v.iter().map(|x| format!("{x:.6e}")).collect();
                    println!("     payload addr {a0}..{a1} ({n} doubles), first {}: [{}]", v.len(), vals.join(", "));
                }
                Err(e) => println!("     payload read error: {e}"),
            }
        }
    }
}
