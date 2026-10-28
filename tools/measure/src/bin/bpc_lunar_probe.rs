use std::collections::HashMap;

use omegaflow::archivar::bpc::BpcFile;
use omegaflow::archivar::ephemeris::{full_orientation, rotation_matrix_from_angles};
use omegaflow::archivar::fk::FkFile;
use omegaflow::archivar::pck;
use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, embedded_lsk, parse_ephemeris_binary,
};

const HORIZONS: [(f64, &str, f64, f64, bool); 4] = [
    (
        946_728_000.0,
        "2000-01-01T12:00Z",
        5.025824,
        -6.720739,
        true,
    ),
    (
        1_104_537_600.0,
        "2005-01-01T00:00Z",
        353.860514,
        -4.562844,
        false,
    ),
    (
        1_262_304_000.0,
        "2010-01-01T00:00Z",
        359.065600,
        -0.064924,
        false,
    ),
    (
        1_577_836_800.0,
        "2020-01-01T00:00Z",
        0.145555,
        6.591897,
        false,
    ),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load(path: &str) -> Option<BodyEphemeris> {
    parse_ephemeris_binary(&std::fs::read(path).ok()?)
}

fn ang(d: f64) -> f64 {
    let mut x = d % 360.0;
    if x > 180.0 {
        x -= 360.0;
    }
    if x < -180.0 {
        x += 360.0;
    }
    x
}

fn body_fixed_lonlat(rot_body_to_icrs: [f64; 9], d: [f64; 3]) -> (f64, f64) {
    let bf = [
        rot_body_to_icrs[0] * d[0] + rot_body_to_icrs[3] * d[1] + rot_body_to_icrs[6] * d[2],
        rot_body_to_icrs[1] * d[0] + rot_body_to_icrs[4] * d[1] + rot_body_to_icrs[7] * d[2],
        rot_body_to_icrs[2] * d[0] + rot_body_to_icrs[5] * d[1] + rot_body_to_icrs[8] * d[2],
    ];
    let n = (bf[0] * bf[0] + bf[1] * bf[1] + bf[2] * bf[2]).sqrt();
    (
        bf[1].atan2(bf[0]).to_degrees(),
        (bf[2] / n).asin().to_degrees(),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(bpc_path), Some(fk_path), Some(pck_path), Some(body), Some(origin)) = (
        arg_value(&args, "--bpc"),
        arg_value(&args, "--fk"),
        arg_value(&args, "--pck"),
        arg_value(&args, "--body"),
        arg_value(&args, "--origin"),
    ) else {
        println!(
            "usage: bpc_lunar_probe --bpc <satellite.bpc> --fk <frames.tf> --pck <body.tpc> \
             --body <name> --origin <name> [--body-bin <bin> --origin-bin <bin>]"
        );
        return;
    };
    let table = omegaflow::ephemeris::body_table();
    let Some(body_naif) = table
        .iter()
        .find(|(_, b)| b.name == body)
        .map(|(&id, _)| id)
    else {
        println!("the body table carries no {body}");
        return;
    };
    let Some(origin_naif) = table
        .iter()
        .find(|(_, b)| b.name == origin)
        .map(|(&id, _)| id)
    else {
        println!("the body table carries no {origin}");
        return;
    };
    let body_pck_id = omegaflow::ephemeris::pck_id_of(body_naif);
    let bpc = match BpcFile::open(&bpc_path) {
        Ok(b) => b,
        Err(e) => {
            println!("the bpc does not open: {e}");
            return;
        }
    };
    let fk = match FkFile::open(&fk_path) {
        Ok(f) => f,
        Err(e) => {
            println!("the fk does not open: {e}");
            return;
        }
    };
    let pck_text = match std::fs::read_to_string(&pck_path) {
        Ok(t) => t,
        Err(e) => {
            println!("the pck does not open: {e}");
            return;
        }
    };
    let bodies = pck::parse(None, Some(&pck_text));
    let Some(iau_body) = bodies.get(&body_pck_id) else {
        println!("the pck carries no {body} ({body_pck_id}) — the iau axis stays absent");
        return;
    };
    let pa_frames = fk
        .frames
        .iter()
        .filter(|f| f.class == Some(2) && f.center == Some(body_pck_id))
        .count();
    println!(
        "fk: {} frames, {pa_frames} pck-class frame(s) at center {body_pck_id} — the bpc path needs both a bpc and that frame",
        fk.frames.len()
    );
    println!("{body} ({body_naif}) seen from {origin} ({origin_naif})");

    let bpc_files = vec![bpc];
    let mut bins: HashMap<String, BodyEphemeris> = HashMap::new();
    if let (Some(body_bin), Some(origin_bin)) = (
        arg_value(&args, "--body-bin"),
        arg_value(&args, "--origin-bin"),
    ) && let (Some(b), Some(o)) = (load(&body_bin), load(&origin_bin))
    {
        bins.insert(body.clone(), b);
        bins.insert(origin.clone(), o);
    }
    let lsk = embedded_lsk();

    for (unix, label, h_lon, h_lat, verified) in HORIZONS {
        let Some(tdb) = lsk.as_ref().and_then(|l| l.unix_to_tdb(unix)) else {
            println!("{label}: the epoch reads void on the leap table");
            continue;
        };
        let jd = tdb / 86400.0 + 2451545.0;
        let direction = match (
            body_barycenter_position(&body, tdb, &bins),
            body_barycenter_position(&origin, tdb, &bins),
        ) {
            (Some(m), Some(e)) => Some([e[0] - m[0], e[1] - m[1], e[2] - m[2]]),
            _ => None,
        };
        let bpc_path_angles = full_orientation(iau_body, &bpc_files, &fk, body_pck_id, jd);
        let iau = match (
            iau_body.pole_ra_at((jd - 2451545.0) / 36525.0),
            iau_body.pole_dec_at((jd - 2451545.0) / 36525.0),
            iau_body.pm_at(jd - 2451545.0),
        ) {
            (Some(a), Some(b), Some(c)) => Some((a, b, c)),
            _ => None,
        };
        println!(
            "{label}  jd {jd:.5}  (horizons ref{})",
            if verified { "" } else { "*, epoch unverified" }
        );
        for (source, angles) in [("bpc", bpc_path_angles), ("iau-tpc", iau)] {
            let Some((ra, dec, w)) = angles else {
                println!("  {source:10} absent — not zero");
                continue;
            };
            let (dlon, dlat) = match direction {
                Some(d) => {
                    let (lo, la) = body_fixed_lonlat(rotation_matrix_from_angles(ra, dec, w), d);
                    (ang(lo - h_lon), la - h_lat)
                }
                None => (f64::NAN, f64::NAN),
            };
            println!(
                "  {source:10} ra {ra:9.4} dec {dec:8.4} w {w:10.4}  dlon {dlon:8.4} dlat {dlat:8.4}"
            );
        }
        if let (Some((br, bd, bw)), Some((er, ed, ew))) = (bpc_path_angles, iau) {
            println!(
                "  bpc - iau  dra {:8.4} ddec {:8.4} dw {:8.4}  (kernel: amplitude 0.0051 deg)",
                ang(br - er),
                bd - ed,
                ang(bw - ew)
            );
        }
    }
    if lsk.is_none() {
        println!("the embedded lsk carries no leap table — only jd arithmetic ran");
    }
    if bins.is_empty() {
        println!("no bins given — the sub-Earth point stays unmeasured");
    }
}
