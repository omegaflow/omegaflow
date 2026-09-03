use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::odf;

const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer11/radio/Turyshev20170327_Pioneer-11/DOPPLER";
const FILES: &[&str] = &[
    "74108o360_bj_archive_sc24.odf",
    "77305o79259_bj_archive_sc24.odf",
    "86002o90274_sc24.odf",
];
const UNIX_1950_OFFSET: f64 = 631152000.0;

fn ls_peak(ts: &[f64], vs: &[f64], flo: f64, fhi: f64, step: f64) -> (f64, f64) {
    let m = ts.len() as f64;
    let vsum = vs.iter().sum::<f64>() / m;
    let mut best = (f64::NAN, 0.0);
    let mut f = flo;
    while f <= fhi {
        let mut s = 0.0;
        let mut c = 0.0;
        for &t in ts {
            let ph = std::f64::consts::TAU * f * t;
            s += ph.sin();
            c += ph.cos();
        }
        s /= m;
        c /= m;
        let mut ss = 0.0;
        let mut cc = 0.0;
        let mut sc = 0.0;
        let mut sy = 0.0;
        let mut cy = 0.0;
        for (i, &t) in ts.iter().enumerate() {
            let ph = std::f64::consts::TAU * f * t;
            let ds = ph.sin() - s;
            let dc = ph.cos() - c;
            let dv = vs[i] - vsum;
            ss += ds * ds;
            cc += dc * dc;
            sc += ds * dc;
            sy += ds * dv;
            cy += dc * dv;
        }
        let det = ss * cc - sc * sc;
        if det.abs() > 1e-300 {
            let a = (sy * cc - cy * sc) / det;
            let b = (cy * ss - sy * sc) / det;
            let p = (a * a + b * b) * m / 2.0;
            if p > best.1 {
                best = (f, p);
            }
        }
        f += step;
    }
    best
}

fn band_scan(label: &str, ts: &[f64], vs: &[f64]) {
    if ts.len() < 500 {
        eprintln!(
            "p11-odf   {label}: {} samples — too short (0 honored)",
            ts.len()
        );
        return;
    }

    let mut dts: Vec<f64> = Vec::new();
    let mut dvs: Vec<f64> = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= 120.0 {
            hi += 1;
        }
        if hi - lo >= 4 {
            let xt: Vec<f64> = ts[lo..hi].to_vec();
            let yv: Vec<f64> = vs[lo..hi].to_vec();
            let mx = xt.iter().sum::<f64>() / xt.len() as f64;
            let my = yv.iter().sum::<f64>() / yv.len() as f64;
            let mut num = 0.0;
            let mut den = 0.0;
            for k in 0..xt.len() {
                num += (xt[k] - mx) * (yv[k] - my);
                den += (xt[k] - mx) * (xt[k] - mx);
            }
            let slope = if den.abs() > 1e-300 { num / den } else { 0.0 };
            for k in 0..xt.len() {
                dts.push(xt[k]);
                dvs.push(yv[k] - (slope * (xt[k] - mx) + my));
            }
        }
        lo = hi;
    }
    let (fp, pp) = ls_peak(&dts, &dvs, 0.0004, 0.0012, 0.00002);
    let mut pows: Vec<f64> = Vec::new();
    let mut f = 0.0004;
    while f <= 0.0012 {
        let (_, p) = ls_peak(&dts, &dvs, f, f, 1e-12);
        pows.push(p);
        f += 0.00002;
    }
    pows.sort_by(f64::total_cmp);
    let floor = pows[pows.len() / 2];
    eprintln!(
        "p11-odf   {label}: n={}, peak {:.5} Hz ({:.1}× floor), A = {:.2e} Hz",
        dts.len(),
        fp,
        pp / floor,
        (2.0 * pp / dts.len() as f64).sqrt()
    );
}

fn main() {
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 9]> = Vec::new();
    for rel in FILES {
        let url = format!("{BASE}/{rel}");
        let Some(bytes) = fetch_raw_bytes(&url, 31536000) else {
            eprintln!("{rel}: fetch void ({url})");
            continue;
        };
        let Some(recs) = odf::parse_odf(&bytes) else {
            eprintln!("{rel}: parse void — {} B", bytes.len());
            continue;
        };
        let mut kept = 0usize;
        let mut skipped = 0usize;
        for r in &recs {
            if !r.valid || r.scid != 24 || !(11..=14).contains(&r.data_type) {
                skipped += 1;
                continue;
            }
            let unix = r.t_since_1950 - UNIX_1950_OFFSET;
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                skipped += 1;
                continue;
            };
            merged.push([
                tdb,
                r.observable_hz,
                r.ref_hz,
                r.dss_rx as f64,
                r.dss_tx as f64,
                r.data_type as f64,
                r.downlink_band as f64,
                r.scid as f64,
                r.compression_s,
            ]);
            kept += 1;
        }
        eprintln!(
            "{rel}: {} orbit records, {kept} kept ({skipped} discarded)",
            recs.len()
        );
    }
    if merged.is_empty() {
        eprintln!("no P11-ODF samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/pioneer11_odf.bin";
    let bin = odf::write_podf_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            eprintln!(
                "{out}: {} Samples, {}..{} (tdb), {:.0} B — roundtrip parses",
                parsed.len(),
                d0[0],
                d1[0],
                bin.len()
            );
        }
        None => eprintln!("{out}: roundtrip parse void — the series stays unverified"),
    }

    let mut per: std::collections::BTreeMap<(i64, i64), (Vec<f64>, Vec<f64>)> =
        std::collections::BTreeMap::new();
    for r in &merged {
        if r[8] < 2.0 {
            continue;
        }
        per.entry((r[3] as i64, r[5] as i64))
            .or_default()
            .0
            .push(r[0]);
        per.entry((r[3] as i64, r[5] as i64))
            .or_default()
            .1
            .push(r[1]);
    }
    let mut keys: Vec<(i64, i64)> = per.keys().copied().collect();
    keys.sort();
    for (st, dt) in keys {
        let (ts, vs) = &per[&(st, dt)];
        let mname = match dt {
            11 => "Einweg",
            12 => "Zweiweg",
            13 => "Dreiweg",
            14 => "Dreiweg-koh",
            _ => "sonst",
        };
        band_scan(&format!("Station {st} Mode {dt} ({mname})"), ts, vs);
    }
    let mut tx: Vec<(i64, i64, usize)> = Vec::new();
    let mut per2: std::collections::BTreeMap<(i64, i64), usize> = std::collections::BTreeMap::new();
    for r in &merged {
        if r[8] < 2.0 {
            continue;
        }
        *per2.entry((r[3] as i64, r[4] as i64)).or_default() += 1;
    }
    for ((rx, txx), c) in per2 {
        tx.push((rx, txx, c));
    }
    tx.sort();
    for (rx, txx, c) in tx {
        if c >= 500 {
            eprintln!("p11-odf   Empfang {rx} × Sende {txx}: {c} Samples");
        }
    }
}
