use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::atdf::{
    IDFORM, LOGICAL_RECORD, S_BAND_REF_HI, S_BAND_REF_LO, XPFORM, extract, field_of, full_year,
    parse_bin, strip_markers, tracking_record, write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{LeapSeconds, days_from_civil};

const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/Data";
const FILES: &[&str] = &[
    "ATDF_Data-Files_CMarkwardt_Readable/SC23-87361172500-88078042912.TDF",
    "ATDF_Data-Files_CMarkwardt_Readable/SC23-88063171500-88169011500.TDF",
    "ATDF_Data-Files_CMarkwardt_Readable/SC23-88168163000-88263042950.TDF",
    "ATDF_Data-Files_CMarkwardt_Readable/SC23-88334163000-88337043000.TDF",
    "ATDF_Data-Files_CMarkwardt_Readable/pioneer10.fl1",
    "ATDF_Data-Files_CMarkwardt_Readable/pioneer10.fl2",
];
const FSKY_MED_HALF_WIDTH: f64 = 0.6e6;
const GAP_DAY: f64 = 0.1;
const RATE_OFFSET: f64 = 1e6;
const S_BAND_RATIO: f64 = 96.0 * 240.0 / 221.0;

fn tdb_of(tr: &omegaflow::atdf::Tracking, lsk: &LeapSeconds) -> Option<f64> {
    if tr.day <= 0 || tr.day > 366 {
        return None;
    }
    let year = full_year(tr.year);
    let days = days_from_civil(year, 1, 1)? + tr.day - 1;
    let unix = days as f64 * 86400.0
        + tr.hour as f64 * 3600.0
        + tr.minute as f64 * 60.0
        + tr.second as f64;
    lsk.unix_to_tdb(unix)
}

fn header_of(stripped: &[u8]) -> (i64, f64, f64) {
    let rec0 = &stripped[0..LOGICAL_RECORD];
    let rec1 = &stripped[LOGICAL_RECORD..2 * LOGICAL_RECORD];
    let year = extract(rec0, field_of(IDFORM, 3).unwrap());
    let day = extract(rec0, field_of(IDFORM, 4).unwrap());
    let sc = extract(rec1, field_of(XPFORM, 9).unwrap());
    let xpon_hp = extract(rec1, field_of(XPFORM, 17).unwrap());
    let xpon_lp = extract(rec1, field_of(XPFORM, 18).unwrap());
    let xpon = xpon_hp as f64 * 1e4 + xpon_lp as f64 / 1e3;
    (sc, full_year(year) as f64 + (day - 1) as f64 / 366.0, xpon)
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.is_empty() {
        return f64::NAN;
    }
    v[v.len() / 2]
}

fn reduce(name: &str, file_id: f64, bytes: &[u8], lsk: &LeapSeconds) -> Option<Vec<[f64; 14]>> {
    let stripped = strip_markers(bytes)?;
    let nlog = stripped.len() / LOGICAL_RECORD;
    if nlog < 3 {
        eprintln!("{name}: {nlog} logical records — too short");
        return None;
    }
    let (sc, file_year, xpon) = header_of(&stripped);
    let mut recs: Vec<omegaflow::atdf::Tracking> = Vec::with_capacity(nlog - 2);
    let mut skipped_zero = 0usize;
    for i in 2..nlog {
        let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
        let tr = tracking_record(rec);
        if tr.day == 0 {
            skipped_zero += 1;
            continue;
        }
        recs.push(tr);
    }
    if recs.len() < 2 {
        eprintln!("{name}: {} tracking records — too short", recs.len());
        return None;
    }
    let mut n = recs.len();
    let mut t = vec![0.0f64; n];
    let mut dcnt = vec![0.0f64; n];
    let mut ref_hz = vec![0.0f64; n];
    let mut bias = vec![0i64; n];
    let mut sampler = vec![0.0f64; n];
    let mut dtype = vec![0i64; n];
    let mut mode = vec![0i64; n];
    let mut station = vec![0i64; n];
    let mut resid = vec![0.0f64; n];
    let mut slipped = vec![0i64; n];
    let mut strength = vec![0i64; n];
    let mut ramp = vec![0i64; n];
    let mut kept = 0usize;
    for tr in recs.iter() {
        let Some(tdb) = tdb_of(tr, lsk) else {
            continue;
        };
        let r = tr.doppler_ref as f64 / 10.0;
        let s = tr.sampler_time as f64 / 100.0;
        t[kept] = tdb;
        dcnt[kept] = tr.doppler_cnt_hp as f64 * 1e4 + tr.doppler_cnt_lp as f64 / 1e3;
        ref_hz[kept] = r;
        bias[kept] = tr.doppler_bias;
        sampler[kept] = s;
        dtype[kept] = tr.data_type;
        mode[kept] = tr.ground_mode;
        station[kept] = tr.station;
        resid[kept] = tr.doppler_resid as f64 / 1000.0;
        slipped[kept] = tr.slipped_cycle;
        strength[kept] = tr.signal_strength;
        ramp[kept] = tr.ramp_rate;
        kept += 1;
    }
    n = kept;
    t.truncate(n);
    dcnt.truncate(n);
    ref_hz.truncate(n);
    bias.truncate(n);
    sampler.truncate(n);
    dtype.truncate(n);
    mode.truncate(n);
    station.truncate(n);
    resid.truncate(n);
    slipped.truncate(n);
    strength.truncate(n);
    ramp.truncate(n);

    let mut fsky = vec![0.0f64; n.saturating_sub(1)];
    let mut good = vec![false; n.saturating_sub(1)];
    for i in 0..n - 1 {
        if dtype[i] == 6 || sampler[i] <= 0.0 {
            continue;
        }
        let doff = if dcnt[i + 1] < dcnt[i] {
            2f64.powi(32)
        } else {
            0.0
        };
        let drate = (dcnt[i + 1] + doff - dcnt[i]) / sampler[i];
        let sdoppler = if bias[i] >= 0 { 1.0 } else { -1.0 };
        fsky[i] = S_BAND_RATIO * ref_hz[i] - sdoppler * (drate - RATE_OFFSET);
        good[i] = true;
    }
    let mut fsky_finite: Vec<f64> = fsky.iter().copied().filter(|x| x.is_finite()).collect();
    let fmed = median(&mut fsky_finite);
    let mut out: Vec<[f64; 14]> = Vec::new();
    let mut ramp_records = 0usize;
    let mut bias_rejected = 0usize;
    let mut ref_rejected = 0usize;
    let mut gap_rejected = 0usize;
    let mut wrap_rejected = 0usize;
    let mut med_rejected = 0usize;
    for i in 0..n - 1 {
        if !good[i] {
            if dtype[i] == 6 {
                ramp_records += 1;
            }
            continue;
        }
        if bias[i].abs() > 1 {
            bias_rejected += 1;
            continue;
        }
        if !(S_BAND_REF_LO..=S_BAND_REF_HI).contains(&ref_hz[i]) {
            ref_rejected += 1;
            continue;
        }
        let gap_days = (t[i + 1] - t[i]) / 86400.0;
        if gap_days >= GAP_DAY {
            gap_rejected += 1;
            continue;
        }
        if dcnt[i + 1] <= dcnt[i] {
            wrap_rejected += 1;
            continue;
        }
        if (fsky[i] - fmed).abs() >= FSKY_MED_HALF_WIDTH {
            med_rejected += 1;
            continue;
        }
        out.push([
            t[i],
            fsky[i],
            ref_hz[i],
            sampler[i],
            bias[i] as f64,
            dtype[i] as f64,
            station[i] as f64,
            dcnt[i],
            resid[i],
            slipped[i] as f64,
            strength[i] as f64,
            ramp[i] as f64,
            file_id,
            mode[i] as f64,
        ]);
    }
    let n_out = out.len();
    let n_slipped = out.iter().filter(|r| r[9] != 0.0).count();
    let mut stations: Vec<i64> = out.iter().map(|r| r[6] as i64).collect();
    stations.sort_unstable();
    stations.dedup();
    eprintln!(
        "{name}: SC {sc}, file year {file_year:.1}, Xponder {xpon:.3e} Hz, {n} tracking records ({skipped_zero} null records), {n_out} fsky samples (median {fmed:.6e} Hz), {n_slipped} with slipped cycle, stations {stations:?} — separated: {ramp_records} ramp, {bias_rejected} bias, {ref_rejected} ref, {gap_rejected} gap, {wrap_rejected} wrap, {med_rejected} median"
    );
    if out.is_empty() { None } else { Some(out) }
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / 86400.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0} s"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 14]> = Vec::new();
    for (fid, rel) in FILES.iter().enumerate() {
        let url = format!("{BASE}/{rel}");
        let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
            eprintln!("{rel}: fetch void ({url})");
            continue;
        };
        if let Some(samples) = reduce(rel, fid as f64, &bytes, &lsk) {
            merged.extend(samples);
        }
    }
    if merged.is_empty() {
        eprintln!("no fsky samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/pioneer10_skyfreq.bin";
    std::fs::create_dir_all("data").ok();
    let bin = write_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match parse_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            let mut f: Vec<f64> = parsed.iter().map(|r| r[1]).collect();
            let fmed = median(&mut f);
            eprintln!(
                "{out}: {} Samples ({}..{}), fsky {:.3e}..{:.3e} Hz (Median {:.6e} Hz), {} B — roundtrip parses",
                parsed.len(),
                jd_date(d0[0]),
                jd_date(d1[0]),
                d1[1],
                d0[1],
                fmed,
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
        }
    }
    if ci_mode && !upload_release("spdf.gsfc.nasa.gov", out) {
        std::process::exit(1);
    }
}
