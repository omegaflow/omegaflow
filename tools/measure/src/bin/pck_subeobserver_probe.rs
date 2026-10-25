use std::collections::HashMap;

use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, body_fixed_vector_to_icrs, embedded_lsk,
    parse_ephemeris_binary,
};

const HORIZONS_LON_DEG: f64 = 5.025824;
const HORIZONS_LAT_DEG: f64 = -6.720739;
const EPOCH_UNIX: f64 = 946728000.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load(path: &str) -> Option<BodyEphemeris> {
    parse_ephemeris_binary(&std::fs::read(path).ok()?)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(body), Some(origin_body), Some(body_path), Some(origin_path)) = (
        arg_value(&args, "--body"),
        arg_value(&args, "--origin"),
        arg_value(&args, "--body-bin"),
        arg_value(&args, "--origin-bin"),
    ) else {
        println!("usage: pck_subeobserver_probe --body <name> --body-bin <bin> --origin <name> --origin-bin <bin>");
        return;
    };
    let (Some(body_eph), Some(origin_eph)) = (load(&body_path), load(&origin_path)) else {
        println!("a bin reads unparsed — the sub-observer point stays unmeasured");
        return;
    };
    let Some(lsk) = embedded_lsk() else {
        println!("the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
        return;
    };
    let Some(tdb) = lsk.unix_to_tdb(EPOCH_UNIX) else {
        println!("the epoch reads void on the leap table");
        return;
    };
    let mut map: HashMap<String, BodyEphemeris> = HashMap::new();
    map.insert(origin_body.clone(), origin_eph);
    map.insert(body.clone(), body_eph);
    let (Some(origin_bc), Some(body_bc)) = (
        body_barycenter_position(&origin_body, tdb, &map),
        body_barycenter_position(&body, tdb, &map),
    ) else {
        println!("a barycenter is absent at the epoch");
        return;
    };
    let Some(origin) = body_fixed_vector_to_icrs(&body, [0.0, 0.0, 0.0], tdb, &map) else {
        println!("the body-fixed frame is absent at the epoch");
        return;
    };
    let scale = 1.0e6;
    let mut rot = [[0.0_f64; 3]; 3];
    for i in 0..3 {
        let mut e = [0.0_f64; 3];
        e[i] = scale;
        let Some(f) = body_fixed_vector_to_icrs(&body, e, tdb, &map) else {
            println!("the body-fixed frame is absent along axis {i}");
            return;
        };
        for row in 0..3 {
            rot[row][i] = (f[row] - origin[row]) / scale;
        }
    }
    let to_origin = [
        origin_bc[0] - body_bc[0],
        origin_bc[1] - body_bc[1],
        origin_bc[2] - body_bc[2],
    ];
    let body_fixed = [
        rot[0][0] * to_origin[0] + rot[1][0] * to_origin[1] + rot[2][0] * to_origin[2],
        rot[0][1] * to_origin[0] + rot[1][1] * to_origin[1] + rot[2][1] * to_origin[2],
        rot[0][2] * to_origin[0] + rot[1][2] * to_origin[1] + rot[2][2] * to_origin[2],
    ];
    let n = (body_fixed[0] * body_fixed[0]
        + body_fixed[1] * body_fixed[1]
        + body_fixed[2] * body_fixed[2])
        .sqrt();
    if !n.is_finite() || n <= 0.0 {
        println!("the body-fixed direction is not a vector");
        return;
    }
    let lon = (body_fixed[1].atan2(body_fixed[0])).to_degrees();
    let lat = (body_fixed[2] / n).asin().to_degrees();
    println!("distance {origin_body}-{body} {:.1} km", n / 1000.0);
    if let Some(p) = map.get(&body).and_then(|e| e.props.as_ref()) {
        println!(
            "props {body}: a0 {:.4} d a0 {:.6} d0 {:.4} w0 {:.4} dw {:.7}",
            p.α0_deg, p.dα0_dt_deg_per_century, p.δ0_deg, p.w0_deg, p.dw_dt_deg_per_day
        );
    }
    println!(
        "sub-{origin_body} point at 2000-01-01T12:00Z: lon {lon:.6} lat {lat:.6} (ours)"
    );
    println!(
        "horizons: lon {HORIZONS_LON_DEG:.6} lat {HORIZONS_LAT_DEG:.6} → dlon {:.6} dlat {:.6} deg",
        lon - HORIZONS_LON_DEG,
        lat - HORIZONS_LAT_DEG
    );
}
