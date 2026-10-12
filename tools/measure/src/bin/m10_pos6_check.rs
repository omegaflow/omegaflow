use std::collections::HashMap;
use std::env;

use omegaflow::archivar::{BodyEphemeris, body_barycenter_position, parse_ephemeris_binary};

const EPOCH_1966_JD: f64 = 2439126.5;
const J2000_EPOCH: f64 = 2451545.0;

fn f64_be(b: &[u8]) -> f64 {
    f64::from_be_bytes(b[..8].try_into().expect("8 bytes"))
}

fn f32_be(b: &[u8]) -> f32 {
    f32::from_be_bytes(b[..4].try_into().expect("4 bytes"))
}

fn load(path: &str) -> Option<BodyEphemeris> {
    std::fs::read(path)
        .ok()
        .and_then(|b| parse_ephemeris_binary(&b))
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn angle(a: [f64; 3], b: [f64; 3]) -> f64 {
    let (na, nb) = (norm(a), norm(b));
    if na == 0.0 || nb == 0.0 {
        return f64::NAN;
    }
    ((a[0] * b[0] + a[1] * b[1] + a[2] * b[2]) / (na * nb))
        .clamp(-1.0, 1.0)
        .acos()
        .to_degrees()
}

fn unit(a: [f64; 3]) -> [f64; 3] {
    let n = norm(a);
    [a[0] / n, a[1] / n, a[2] / n]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn triad(a: [f64; 3], b: [f64; 3]) -> [[f64; 3]; 3] {
    let e1 = unit(a);
    let t = [
        b[0] - dot(b, e1) * e1[0],
        b[1] - dot(b, e1) * e1[1],
        b[2] - dot(b, e1) * e1[2],
    ];
    let e2 = unit(t);
    let e3 = cross(e1, e2);
    [e1, e2, e3]
}

fn rot_apply(m: [[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        dot(m[0], v),
        dot(m[1], v),
        dot(m[2], v),
    ]
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!(
            "m10_pos6_check: usage: m10_pos6_check <POS_M1_ENC_42SEC.DAT> <m10_ephemeris.bin> <mercury_ephemeris.bin> — refused"
        );
        std::process::exit(1);
    }
    let path = &args[0];
    let m10_path = &args[1];
    let mer_path = &args[2];
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("m10_pos6_check: read {path}: {e}");
            std::process::exit(1);
        }
    };
    let row = 152usize;
    if !data.len().is_multiple_of(row) {
        eprintln!("m10_pos6_check: {} B not a multiple of 152", data.len());
        std::process::exit(1);
    }
    let n = data.len() / row;
    let col = |rec: usize, k: usize| -> f64 { f32_be(&data[rec * row + 8 + 4 * (k - 2)..]) as f64 };
    let time = |rec: usize| -> f64 { f64_be(&data[rec * row..]) };
    let Some(m10) = load(m10_path) else {
        eprintln!("m10_pos6_check: {m10_path} reads void");
        std::process::exit(1);
    };
    let Some(mer) = load(mer_path) else {
        eprintln!("m10_pos6_check: {mer_path} reads void");
        std::process::exit(1);
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    eph.insert("mariner10".into(), m10);
    eph.insert("mercury".into(), mer);

    let mut matched = 0usize;
    let mut sum_dv = 0.0f64;
    let mut max_dv = 0.0f64;
    let mut max_dv_jd = 0.0f64;
    let mut sum_dr = 0.0f64;
    let mut max_dr = 0.0f64;
    let mut min_ang = f64::MAX;
    let mut max_ang = 0.0f64;
    let mut sum_ang = 0.0f64;
    let mut first_pos: Option<[f64; 3]> = None;
    let mut first_vec: Option<[f64; 3]> = None;
    let mut pos_all: Vec<[f64; 3]> = Vec::new();
    let mut vec_all: Vec<[f64; 3]> = Vec::new();
    let mut sum_pair_pos = 0.0f64;
    let mut sum_pair_vec = 0.0f64;
    let mut pair_n = 0usize;
    let mut worst: Option<(f64, [f64; 3], [f64; 3])> = None;
    for r in 0..n {
        let jd = EPOCH_1966_JD + time(r) / 86400.0;
        let tdb = (jd - J2000_EPOCH) * 86400.0;
        let (Some(sc), Some(me)) = (
            body_barycenter_position("mariner10", tdb, &eph),
            body_barycenter_position("mercury", tdb, &eph),
        ) else {
            continue;
        };
        let vec = [
            (sc[0] - me[0]) * 1e-3,
            (sc[1] - me[1]) * 1e-3,
            (sc[2] - me[2]) * 1e-3,
        ];
        let pos = [col(r, 8), col(r, 9), col(r, 10)];
        let dv = norm([pos[0] - vec[0], pos[1] - vec[1], pos[2] - vec[2]]);
        let dr = norm(pos) - norm(vec);
        let ang = angle(pos, vec);
        matched += 1;
        sum_dv += dv;
        sum_dr += dr.abs();
        sum_ang += ang;
        min_ang = min_ang.min(ang);
        if dv > max_dv {
            max_dv = dv;
            max_dv_jd = jd;
            worst = Some((jd, pos, vec));
        }
        max_dr = max_dr.max(dr.abs());
        max_ang = max_ang.max(ang);
        pos_all.push(pos);
        vec_all.push(vec);
        match (first_pos, first_vec) {
            (Some(p0), Some(v0)) => {
                sum_pair_pos += angle(pos, p0);
                sum_pair_vec += angle(vec, v0);
                pair_n += 1;
            }
            _ => {
                first_pos = Some(pos);
                first_vec = Some(vec);
            }
        }
    }
    println!("POS-6 records: {n}, matched against the ephemeris: {matched}");
    println!(
        "|POS6 - (sc-me)| : mean {:.1} km, max {:.1} km @ JD {:.6}",
        sum_dv / matched as f64,
        max_dv,
        max_dv_jd
    );
    println!(
        "|POS6| - |sc-me| : mean {:.1} km, max {:.1} km",
        sum_dr / matched as f64,
        max_dr
    );
    println!(
        "angle(POS6, sc-me): mean {:.4} deg, min {:.4}, max {:.4}",
        sum_ang / matched as f64,
        min_ang,
        max_ang
    );
    println!(
        "frame test — angle to record 0: POS6-pair mean {:.4} deg vs eph-pair mean {:.4} deg ({pair_n} probes)",
        sum_pair_pos / pair_n as f64,
        sum_pair_vec / pair_n as f64
    );
    if let Some((jd, pos, vec)) = worst {
        println!("worst frame JD {jd:.6}");
        println!(
            "  POS6 [{:.3}, {:.3}, {:.3}] |{:.3}|",
            pos[0],
            pos[1],
            pos[2],
            norm(pos)
        );
        println!(
            "  eph  [{:.3}, {:.3}, {:.3}] |{:.3}|",
            vec[0],
            vec[1],
            vec[2],
            norm(vec)
        );
    }
    if matched >= 3 {
        let (a, b) = (0usize, matched / 3);
        let tv = triad(vec_all[a], vec_all[b]);
        let tp = triad(pos_all[a], pos_all[b]);
        let mut r = [[0.0f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                r[i][j] = tp[0][i] * tv[0][j] + tp[1][i] * tv[1][j] + tp[2][i] * tv[2][j];
            }
        }
        let mut sum = 0.0f64;
        let mut mx = 0.0f64;
        for k in 0..matched {
            let rp = rot_apply(r, vec_all[k]);
            let d = norm([rp[0] - pos_all[k][0], rp[1] - pos_all[k][1], rp[2] - pos_all[k][2]]);
            sum += d;
            mx = mx.max(d);
        }
        let tr = r[0][0] + r[1][1] + r[2][2];
        let theta = ((tr - 1.0) / 2.0).clamp(-1.0, 1.0).acos().to_degrees();
        let axis = unit([r[2][1] - r[1][2], r[0][2] - r[2][0], r[1][0] - r[0][1]]);
        println!(
            "R (POS6 = R*(sc-me)): residual mean {:.1} km, max {:.1} km | rotation {:.3} deg about axis RA {:.3} Dec {:.3}",
            sum / matched as f64,
            mx,
            theta,
            axis[1].atan2(axis[0]).to_degrees().rem_euclid(360.0),
            axis[2].clamp(-1.0, 1.0).asin().to_degrees()
        );
    }
}
