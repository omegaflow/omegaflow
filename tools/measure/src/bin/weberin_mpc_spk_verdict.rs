use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use omegaflow::archivar::mpcorb::{self, MpcorbRec};
use omegaflow::archivar::{
    BodyEphemeris, J2000_EPOCH, LeapSeconds, body_barycenter_position, embedded_lsk,
    parse_ephemeris_binary, system_now,
};
use omegaflow::weberin::{BODY_NUMBER, BodyOutcome, WEBERIN_TOL_M, Weberin, WeberinFeed};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() {
    println!(
        "usage: weberin_mpc_spk_verdict [--eph-dir <data-root>] [--mpcorb <mpcorb.bin>] [--mpcorb-distant <mpcorb_distant.bin>] [--epoch <jd>] [--tol <m>]"
    );
}

fn load_mpcorb(path: &str) -> Vec<MpcorbRec> {
    match std::fs::read(path) {
        Ok(b) => b
            .chunks_exact(mpcorb::RECORD_STRIDE)
            .filter_map(mpcorb::parse_record)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn read_eph(dir: &str, name: &str) -> Option<BodyEphemeris> {
    let path = format!("{dir}/ssd.jpl.nasa.gov/ephemeris_{name}.bin");
    let bytes = std::fs::read(&path).ok()?;
    parse_ephemeris_binary(&bytes)
}

fn mpc_line(name: &str, number: Option<u32>, outcome: &BodyOutcome) -> String {
    let num = number.map_or_else(|| "pending".to_string(), |n| n.to_string());
    match outcome {
        BodyOutcome::Placed { sep_m } => {
            format!("weberin-mpc {name} ({num}) state placed sep {sep_m:e}")
        }
        BodyOutcome::Absent { line } => format!(
            "weberin-mpc {name} ({num}) state absent sep absent missing {}",
            line.word()
        ),
        BodyOutcome::Riss { sep_m, knot } => format!(
            "weberin-mpc {name} ({num}) state riss sep {sep_m:e} knot {}+{}",
            knot[0].word(),
            knot[1].word()
        ),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let eph_dir = match arg_value(&args, "--eph-dir") {
        Some(d) => d,
        None => "data".to_string(),
    };
    let mpcorb_path = match arg_value(&args, "--mpcorb") {
        Some(d) => d,
        None => "data/ssd.jpl.nasa.gov/mpcorb.bin".to_string(),
    };
    let mpcorb_distant_path = match arg_value(&args, "--mpcorb-distant") {
        Some(d) => d,
        None => "data/ssd.jpl.nasa.gov/mpcorb_distant.bin".to_string(),
    };
    let tol_m = match arg_value(&args, "--tol").and_then(|w| w.parse::<f64>().ok()) {
        Some(t) if t.is_finite() && t > 0.0 => t,
        Some(_) => {
            eprintln!("weberin-mpc: --tol not a finite positive value — the weave stays closed");
            return;
        }
        None => WEBERIN_TOL_M,
    };
    let jd = match arg_value(&args, "--epoch").and_then(|w| w.parse::<f64>().ok()) {
        Some(j) if j.is_finite() && j > 0.0 => j,
        Some(_) => {
            eprintln!("weberin-mpc: --epoch not a finite JD — the weave stays closed");
            return;
        }
        None => {
            let time: Arc<Mutex<Option<LeapSeconds>>> = Arc::new(Mutex::new(embedded_lsk()));
            match system_now(&time) {
                Some(tdb) => tdb / 86400.0 + J2000_EPOCH,
                None => {
                    eprintln!(
                        "weberin-mpc: the system TDB epoch reads void (naif0012 leap table) — give --epoch <jd>"
                    );
                    return;
                }
            }
        }
    };
    let tdb = (jd - J2000_EPOCH) * 86400.0;

    let mut mpc_recs = load_mpcorb(&mpcorb_path);
    let distant_recs = load_mpcorb(&mpcorb_distant_path);
    let used_path = if !mpc_recs.is_empty() {
        mpcorb_path.clone()
    } else if !distant_recs.is_empty() {
        mpc_recs = distant_recs;
        mpcorb_distant_path.clone()
    } else {
        String::new()
    };
    if mpc_recs.is_empty() {
        println!(
            "weberin-mpc: {mpcorb_path} and {mpcorb_distant_path} are both void — the MPC line stays unread (run mpcorb_compiler on the registered mpcorb_extended.json.gz)"
        );
        return;
    }

    let Some(sun_eph) = read_eph(&eph_dir, "sun") else {
        println!(
            "weberin-mpc: the sun ephemeris bin is void — the heliocentric MPC line stays without a barycentric fold"
        );
        return;
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut sun_map: HashMap<String, BodyEphemeris> = HashMap::new();
    sun_map.insert("sun".to_string(), sun_eph);
    let mut spk_opened = 0usize;
    for (name, _) in BODY_NUMBER {
        if let Some(body_eph) = read_eph(&eph_dir, name) {
            eph.insert((*name).to_string(), body_eph);
            spk_opened += 1;
        }
    }
    let sun_at_tdb = body_barycenter_position("sun", tdb, &sun_map);
    if sun_at_tdb.is_none() {
        println!(
            "weberin-mpc: the sun reference reads void at tdb {tdb:.3} — the weave stays closed"
        );
        return;
    }

    println!(
        "=== Weberin step 1 — the second body line (MPC Keplerian elements) against the SPK ephemeris points ==="
    );
    println!(
        "mpc line {used_path}: {} record(s) read | weave epoch jd {jd:.5} (tdb {tdb:.3} s past J2000) | tolerance {tol_m:.3e} m | {} of {} small-body SPK ephemeris bin(s) opened",
        mpc_recs.len(),
        spk_opened,
        BODY_NUMBER.len()
    );

    let mut w = Weberin::new();
    w.feed(WeberinFeed {
        eph: Arc::new(eph),
        sun: Arc::new(sun_map),
        eph_inpop: Arc::new(HashMap::new()),
        eph_epm: Arc::new(HashMap::new()),
        recs: Vec::new(),
        comets: Vec::new(),
        mpc_recs,
    });
    w.weave(tdb, tol_m);
    if !w.woven {
        eprintln!("weberin-mpc: the weave did not run — no body line is judged");
        return;
    }

    let mut placed = 0usize;
    let mut absent = 0usize;
    let mut riss = 0usize;
    let mut judged = 0usize;
    for v in &w.mpc_verdicts {
        judged += 1;
        let number = omegaflow::weberin::body_number(&v.name);
        match &v.outcome {
            BodyOutcome::Placed { .. } => placed += 1,
            BodyOutcome::Absent { .. } => absent += 1,
            BodyOutcome::Riss { .. } => riss += 1,
        }
        println!("{}", mpc_line(&v.name, number, &v.outcome));
    }
    println!(
        "weberin-mpc tally: {judged} small-body line(s) judged | placed {placed} | absent {absent} | riss {riss}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::weberin::BodyLine;

    #[test]
    fn placed_line_carries_the_measured_separation_and_number() {
        let o = BodyOutcome::Placed { sep_m: 2.5e4 };
        assert_eq!(
            mpc_line("ceres", Some(1), &o),
            "weberin-mpc ceres (1) state placed sep 2.5e4"
        );
    }

    #[test]
    fn absent_line_names_the_missing_mpc_line() {
        let o = BodyOutcome::Absent {
            line: BodyLine::Mpc,
        };
        assert_eq!(
            mpc_line("vesta", Some(4), &o),
            "weberin-mpc vesta (4) state absent sep absent missing mpc-keplerian"
        );
        let o = BodyOutcome::Absent {
            line: BodyLine::Spk,
        };
        assert_eq!(
            mpc_line("pallas", Some(2), &o),
            "weberin-mpc pallas (2) state absent sep absent missing spk-ephemeris"
        );
    }

    #[test]
    fn riss_line_names_both_refusing_threads() {
        let o = BodyOutcome::Riss {
            sep_m: 3.1e6,
            knot: [BodyLine::Spk, BodyLine::Mpc],
        };
        assert_eq!(
            mpc_line("pluto", Some(134340), &o),
            "weberin-mpc pluto (134340) state riss sep 3.1e6 knot spk-ephemeris+mpc-keplerian"
        );
    }
}
