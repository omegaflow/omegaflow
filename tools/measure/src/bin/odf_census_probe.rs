use omegaflow::odf::{self, OdOrbit};

const UNIX_1950_OFFSET: f64 = 631152000.0;

fn civil(secs_1950: f64) -> String {
    let unix = secs_1950 - UNIX_1950_OFFSET;
    let day = (unix / 86400.0).floor() as i64;
    let rem = unix - day as f64 * 86400.0;
    let hh = (rem / 3600.0).floor() as i64;
    let mm = ((rem - hh as f64 * 3600.0) / 60.0).floor() as i64;
    let ss = (rem - hh as f64 * 3600.0 - mm as f64 * 60.0).floor() as i64;
    format!("{day}d+{hh:02}:{mm:02}:{ss:02}")
}

fn main() {
    let arg = std::env::args().skip(1).find(|a| !a.starts_with('-'));
    let path = arg.unwrap_or_else(|| "src/archivar/kernels/odf07155.dat".to_string());
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("odf-census: read void ({path})");
        return;
    };
    let Some(recs) = odf::parse_odf(&bytes) else {
        eprintln!("odf-census: parse void — {path} is not a TRK-2-34 ODF");
        return;
    };
    let total = recs.len();
    let valid: Vec<&OdOrbit> = recs.iter().filter(|r| r.valid).collect();
    let n_valid = valid.len();
    let mut scids: Vec<i64> = valid.iter().map(|r| r.scid).collect();
    scids.sort_unstable();
    scids.dedup();
    let mut dts: Vec<i64> = valid.iter().map(|r| r.data_type).collect();
    dts.sort_unstable();
    dts.dedup();
    let mut rxs: Vec<i64> = valid.iter().map(|r| r.dss_rx).collect();
    rxs.sort_unstable();
    rxs.dedup();
    let mut txs: Vec<i64> = valid.iter().map(|r| r.dss_tx).collect();
    txs.sort_unstable();
    txs.dedup();
    let mut omin = f64::INFINITY;
    let mut omax = f64::NEG_INFINITY;
    let tmin = valid.iter().map(|r| r.t_since_1950).fold(f64::INFINITY, f64::min);
    let tmax = valid.iter().map(|r| r.t_since_1950).fold(f64::NEG_INFINITY, f64::max);
    for r in &valid {
        if r.observable_hz < omin {
            omin = r.observable_hz;
        }
        if r.observable_hz > omax {
            omax = r.observable_hz;
        }
    }
    if valid.is_empty() {
        println!("odf-census verdict: pending — no valid orbit record (0 honored)");
        return;
    }
    eprintln!(
        "odf-census {}: {} records, {} valid; scid {:?}; data_type {:?}; dss_rx {:?}; dss_tx {:?}; span {} .. {}",
        path,
        total,
        n_valid,
        scids,
        dts,
        rxs,
        txs,
        civil(tmin),
        civil(tmax)
    );
    let mname = |dt: i64| match dt {
        11 => "one-way",
        12 => "two-way",
        13 => "three-way",
        14 => "three-way-coh",
        37 => "other",
        _ => "other",
    };
    let dline: Vec<String> = dts.iter().map(|d| format!("{d}({})", mname(*d))).collect();
    println!(
        "odf-census: {} valid TRK-2-34 orbit records, scid {:?}, data_type {:?}, dss_rx {:?}, dss_tx {:?}",
        n_valid, scids, dline, rxs, txs
    );
    println!(
        "odf-census: raw observable_hz span {omin:.1} .. {omax:.1} Hz — total received-frequency offset (mixing/ramp bias unmodeled), not a range-rate residual"
    );
    println!(
        "odf-census verdict: pending — no signed mm/s range-rate residual derivable from this ODF without a per-record doppler/ramp model + matching trajectory (absent here)"
    );
}
