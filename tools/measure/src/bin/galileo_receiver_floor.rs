use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const STRONG_MIN: i64 = -1750;
const LOUD_HZ: f64 = 1.0;

const RX_REF_IDX: usize = 8;
const RX_NUM_IDX: usize = 9;
const AMP_NUM_IDX: usize = 10;
const AMP_TYPE_IDX: usize = 11;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rx {
    rx_ref: i64,
    rx_num: i64,
    amp_num: i64,
    amp_type: i64,
}

fn unix_day(tdb: f64) -> i64 {
    (tdb / DAY_S + 10957.5).round() as i64
}

fn date_of(tdb: f64) -> (i64, i64, i64) {
    match civil_from_days(unix_day(tdb)) {
        Some((y, m, d)) => (y as i64, m as i64, d as i64),
        None => (0, 0, 0),
    }
}

fn fmt_date(tdb: f64) -> String {
    let (y, m, d) = date_of(tdb);
    format!("{y:04}-{m:02}-{d:02}")
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn parse_receiver_bin(bytes: &[u8]) -> Option<Vec<[f64; 12]>> {
    if bytes.len() < 8 || &bytes[0..4] != b"GARX" {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + count * 96 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 96;
        let mut r = [0.0f64; 12];
        for k in 0..12 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&bytes[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

#[derive(Clone, Copy)]
struct Cell {
    mode: i64,
    day: i64,
    st: i64,
    floor: bool,
    rx: Rx,
    n: usize,
    rms: f64,
    t0: f64,
}

fn fmt_rx(rx: &Rx) -> String {
    format!(
        "ref {} rcv {} amp {} atype {}",
        rx.rx_ref, rx.rx_num, rx.amp_num, rx.amp_type
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let first_arg = args.iter().find(|a| !a.starts_with('-')).cloned();
    let path = match first_arg {
        Some(p) => p,
        None => "data/galileo_receiver.bin".to_string(),
    };
    let pos = args.iter().position(|a| a == "--report");
    let report = match pos.and_then(|i| args.get(i + 1)).cloned() {
        Some(r) => r,
        None => "/tmp/opencode/galileo_receiver_floor_report.txt".to_string(),
    };
    let anchor_only = args.iter().any(|a| a == "--anchor-only");

    let bytes = fs::read(&path).expect("receiver bin read");
    let recs = match parse_receiver_bin(&bytes) {
        Some(r) => r,
        None => {
            eprintln!("{path}: not a GARX receiver bin");
            std::process::exit(1);
        }
    };

    let mut cell: BTreeMap<(i64, i64, i64, bool, Rx), (f64, f64, usize, f64)> = BTreeMap::new();
    let mut rx_census: BTreeMap<(i64, i64, Rx), usize> = BTreeMap::new();
    let mut n_sk = 0usize;
    let mut n_lock = 0usize;
    let mut n_uncl = 0usize;
    let mut n_zero = 0usize;
    let mut t0_all = f64::INFINITY;
    let mut t1_all = f64::NEG_INFINITY;

    for r in &recs {
        let tdb = r[0];
        t0_all = t0_all.min(tdb);
        t1_all = t1_all.max(tdb);
        let resid = r[1];
        if !resid.is_finite() {
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let mode = r[3] as i64;
        if mode != 1 && mode != 2 {
            n_sk += 1;
            continue;
        }
        let st = r[2] as i64;
        if st == 0 {
            n_zero += 1;
            continue;
        }
        let s = r[7] as i64;
        let floor = if s == FLOOR {
            true
        } else if s >= STRONG_MIN {
            false
        } else {
            n_uncl += 1;
            continue;
        };
        let rx = Rx {
            rx_ref: r[RX_REF_IDX] as i64,
            rx_num: r[RX_NUM_IDX] as i64,
            amp_num: r[AMP_NUM_IDX] as i64,
            amp_type: r[AMP_TYPE_IDX] as i64,
        };
        *rx_census.entry((st, mode, rx)).or_insert(0) += 1;
        let day = (tdb / DAY_S).floor() as i64;
        let e = cell
            .entry((mode, day, st, floor, rx))
            .or_insert_with(|| (0.0, 0.0, 0, tdb));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
        if tdb < e.3 {
            e.3 = tdb;
        }
    }
    drop(recs);
    drop(bytes);

    let mut rows: Vec<Cell> = Vec::new();
    for (&(mode, day, st, floor, rx), &(sum, sum2, n, t0)) in &cell {
        if n == 0 {
            continue;
        }
        let m = sum / n as f64;
        let v = (sum2 / n as f64 - m * m).max(0.0);
        rows.push(Cell {
            mode,
            day,
            st,
            floor,
            rx,
            n,
            rms: v.sqrt(),
            t0,
        });
    }
    rows.sort_by_key(|c| (c.mode, c.day, c.st, c.rx.rx_ref, c.rx.rx_num, c.rx.amp_num));

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push(format!("galileo receiver-floor causation probe — {path}"));
    let classed = cell.values().map(|c| c.2).sum::<usize>();
    push(format!(
        "span {} .. {}, {classed} classed samples",
        fmt_date(t0_all),
        fmt_date(t1_all)
    ));
    push(format!(
        "excluded before the class split: non-mode samples {n_sk}, lock (|resid|>{LOCK_HZ:.0} Hz) {n_lock}, strength==0 {n_zero}, strength neither floor nor strong {n_uncl}"
    ));
    push(format!(
        "classes: floor = strength == {FLOOR} (AGC clamp); strong = strength >= {STRONG_MIN}; loud = cell RMS >= {LOUD_HZ} Hz; cell = (ground_mode, day, station, class, receiver identity)"
    ));
    push(format!(
        "receiver fields: slot {RX_REF_IDX}=DOPPLER_RCVR_REF, slot {RX_NUM_IDX}=RCVR_NUMBER, slot {AMP_NUM_IDX}=AMP_NUMBER, slot {AMP_TYPE_IDX}=AMP_TYPE"
    ));

    if anchor_only {
        for anchor in ["1995-11-24", "1996-06-26"] {
            push(format!("== anchor {anchor} =="));
            for c in rows.iter().filter(|c| c.floor && fmt_date(c.t0) == anchor) {
                push(format!(
                    "  {} mode {} st{} {} n {:<7} RMS {:10.4} Hz {}",
                    fmt_date(c.t0),
                    c.mode,
                    c.st,
                    fmt_rx(&c.rx),
                    c.n,
                    c.rms,
                    if c.rms >= LOUD_HZ { "LOUD" } else { "quiet" }
                ));
            }
        }
    } else {
        push(String::new());
        push(
            "== receiver identity census over classed samples (station, mode, receiver) =="
                .to_string(),
        );
        for ((st, mode, rx), count) in &rx_census {
            push(format!("  st{st} mode{mode} {}: {count}", fmt_rx(rx)));
        }

        push(String::new());
        push("== floor cells (chronological) ==".to_string());
        for c in rows.iter().filter(|c| c.floor) {
            push(format!(
                "  {} mode {} st{} {} n {:<7} RMS {:10.4} Hz {}",
                fmt_date(c.t0),
                c.mode,
                c.st,
                fmt_rx(&c.rx),
                c.n,
                c.rms,
                if c.rms >= LOUD_HZ { "LOUD" } else { "quiet" }
            ));
        }

        push(String::new());
        push("== strong cells (chronological) ==".to_string());
        for c in rows.iter().filter(|c| !c.floor) {
            push(format!(
                "  {} mode {} st{} {} n {:<7} RMS {:10.4} Hz {}",
                fmt_date(c.t0),
                c.mode,
                c.st,
                fmt_rx(&c.rx),
                c.n,
                c.rms,
                if c.rms >= LOUD_HZ { "LOUD" } else { "quiet" }
            ));
        }

        push(String::new());
        push("== causation table: per (class, station, full receiver identity) ==".to_string());
        push("  the ref/rcvr/amp split within a fixed station is the receiver-vs-epoch test; a receiver identity that is both loud and quiet at one station means the coded receiver state alone does not set the loudness".to_string());
        for floor in [true, false] {
            for mode in [1i64, 2] {
                let sub: Vec<&Cell> = rows
                    .iter()
                    .filter(|c| c.floor == floor && c.mode == mode)
                    .collect();
                let name = if floor { "floor" } else { "strong" };
                let mut keys: BTreeSet<(i64, Rx)> = BTreeSet::new();
                for c in &sub {
                    keys.insert((c.st, c.rx));
                }
                push(format!(
                    "  --- {name} mode {mode}: {} cells, {} (station,receiver) keys ---",
                    sub.len(),
                    keys.len()
                ));
                for &(st, rx) in &keys {
                    let cells: Vec<&&Cell> =
                        sub.iter().filter(|c| c.st == st && c.rx == rx).collect();
                    if cells.is_empty() {
                        continue;
                    }
                    let rms_v: Vec<f64> = cells.iter().map(|c| c.rms).collect();
                    let loud = cells.iter().filter(|c| c.rms >= LOUD_HZ).count();
                    let days: BTreeSet<i64> = cells.iter().map(|c| c.day).collect();
                    let med_s = match median(&rms_v) {
                        Some(m) => format!("{m:.4}"),
                        None => "-".to_string(),
                    };
                    push(format!(
                        "  st{st} {}: {} cells / {} days / med RMS {med_s} Hz / loud {loud}",
                        fmt_rx(&rx),
                        cells.len(),
                        days.len()
                    ));
                }
            }
        }
    }

    let _ = fs::write(&report, out.join("\n") + "\n");
    eprintln!("galileo: receiver floor report written to {report}");
}
