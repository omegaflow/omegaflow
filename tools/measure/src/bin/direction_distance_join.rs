use omegaflow::archivar::json::{jnum, jstr, parse_json, JsonVal};
use omegaflow::archivar::skydirection::{parse_bin, write_bin, SkyDirection};
use omegaflow::archivar::spatial::{parse_star_record, star_stride, STAR_RECORD_BYTES};
use omegaflow::archivar::PARSEC_M;
use omegaflow_measure::weberin::deredden::{build_star_index, StarIndex};
use omegaflow::cdn::upload_asset;

const DEG2_PER_SR: f64 = 129600.0 / std::f64::consts::PI;

struct Transient {
    id: Option<String>,
    ra_deg: f64,
    dec_deg: f64,
}

fn read_alert(m: &JsonVal) -> Option<Transient> {
    let ra = jnum(m, "meanra").or_else(|| jnum(m, "ra"))?;
    let dec = jnum(m, "meandec").or_else(|| jnum(m, "dec"))?;
    if !(ra.is_finite() && dec.is_finite()) {
        return None;
    }
    if !((0.0..=360.0).contains(&ra) && (-90.0..=90.0).contains(&dec)) {
        return None;
    }
    let id = jstr(m, "oid")
        .or_else(|| jstr(m, "id"))
        .or_else(|| jstr(m, "obj"));
    Some(Transient {
        id,
        ra_deg: ra,
        dec_deg: dec,
    })
}

fn parse_alerts(text: &str) -> Vec<Transient> {
    let Some(root) = parse_json(text) else {
        return Vec::new();
    };
    let list: &Vec<JsonVal> = match &root {
        JsonVal::Arr(a) => a,
        JsonVal::Obj(map) => match map.get("items") {
            Some(JsonVal::Arr(a)) => a,
            _ => return Vec::new(),
        },
        _ => return Vec::new(),
    };
    list.iter().filter_map(read_alert).collect()
}

fn raw_ra(chunk: &[u8]) -> Option<f64> {
    Some(f64::from_le_bytes(chunk.get(0..8)?.try_into().ok()?))
}

fn raw_dec(chunk: &[u8]) -> Option<f64> {
    Some(f64::from_le_bytes(chunk.get(8..16)?.try_into().ok()?))
}

fn raw_plx(chunk: &[u8]) -> Option<f64> {
    Some(f32::from_le_bytes(chunk.get(24..28)?.try_into().ok()?) as f64)
}

fn build_present_standins(bytes: &[u8], refused: &mut usize) -> Vec<u8> {
    let mut out = Vec::new();
    for chunk in bytes.chunks_exact(STAR_RECORD_BYTES) {
        if parse_star_record(chunk).is_some() {
            continue;
        }
        let (Some(ra), Some(dec), Some(plx)) = (raw_ra(chunk), raw_dec(chunk), raw_plx(chunk))
        else {
            *refused += 1;
            continue;
        };
        if ra.is_finite() && dec.is_finite() && !(plx.is_finite() && plx > 0.0) {
            let mut standin = chunk.to_vec();
            standin[24..28].copy_from_slice(&1.0f32.to_le_bytes());
            out.extend_from_slice(&standin);
        } else {
            *refused += 1;
        }
    }
    out
}

#[derive(Debug)]
enum Verdict {
    Placed {
        star: usize,
        sep: f64,
        within: usize,
    },
    Absent {
        sep: f64,
        within: usize,
    },
    DirectionOnly,
}

fn resolve(
    distance: &StarIndex,
    present: &StarIndex,
    ra: f64,
    dec: f64,
    radius_as: f64,
) -> Verdict {
    let r_deg = radius_as / 3600.0;
    let near = distance.within(ra, dec, r_deg);
    let here = present.within(ra, dec, r_deg);
    let within = near.len() + here.len();
    match (near.first(), here.first()) {
        (Some((_, s_near)), Some((_, s_here))) if *s_here < *s_near => Verdict::Absent {
            sep: *s_here,
            within,
        },
        (Some((k, sep)), _) => Verdict::Placed {
            star: *k,
            sep: *sep,
            within,
        },
        (None, Some((_, sep))) => Verdict::Absent { sep: *sep, within },
        (None, None) => Verdict::DirectionOnly,
    }
}

fn subject(id: Option<&str>, ra: f64, dec: f64) -> String {
    match id {
        Some(n) => format!("{n} at ra {ra:.5} dec {dec:.5}"),
        None => format!("ra {ra:.5} dec {dec:.5}"),
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn each_after(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (i, a) in args.iter().enumerate() {
        if a == flag {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
        }
    }
    out
}

fn object_arg(args: &[String]) -> Option<(f64, f64)> {
    let i = args.iter().position(|a| a == "--object")?;
    let mut tokens: Vec<&str> = Vec::new();
    for a in &args[i + 1..] {
        if a.starts_with("--") {
            break;
        }
        tokens.extend(a.split_whitespace());
    }
    if tokens.len() < 2 {
        return None;
    }
    let ra = tokens[0].parse::<f64>().ok()?;
    let dec = tokens[1].parse::<f64>().ok()?;
    if ra.is_finite()
        && dec.is_finite()
        && (0.0..=360.0).contains(&ra)
        && (-90.0..=90.0).contains(&dec)
    {
        Some((ra, dec))
    } else {
        None
    }
}

fn usage() {
    println!(
        "usage: direction_distance_join --stars <dr3_stars.bin> --radius <arcsec> (--object <ra> <dec> [--name <id>] | --transients <alerts.json> [--transients <more.json> ...] | --directions <skd1> [--out <skd1>]) [--ci-mode]"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(stars_path) = arg_value(&args, "--stars") else {
        usage();
        return;
    };
    let Some(radius_w) = arg_value(&args, "--radius") else {
        usage();
        return;
    };
    let Ok(radius_as) = radius_w.parse::<f64>() else {
        eprintln!(
            "--radius {radius_w}: not a finite positive arcsec — the search cone stays closed"
        );
        return;
    };
    if !(radius_as.is_finite() && radius_as > 0.0) {
        eprintln!(
            "--radius {radius_w}: not a finite positive arcsec — the search cone stays closed"
        );
        return;
    }

    let star_bytes = match std::fs::read(&stars_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {stars_path} returned void: {e}");
            return;
        }
    };
    let Some(stride) = star_stride(&star_bytes) else {
        eprintln!(
            "star bin {} bytes: no {}-byte records — the catalog stays unread",
            star_bytes.len(),
            STAR_RECORD_BYTES
        );
        return;
    };
    let n_records = star_bytes.len() / stride;
    let distance = build_star_index(&star_bytes);
    let mut refused = 0usize;
    let standins = build_present_standins(&star_bytes, &mut refused);
    let present = build_star_index(&standins);
    let n_distance = distance.stars.len();
    let n_present = present.stars.len();
    let density = n_distance as f64 / DEG2_PER_SR;
    let background = density * std::f64::consts::PI * (radius_as / 3600.0).powi(2);

    println!(
        "=== direction_distance_join — the distance-through-identity join of direction-only transients to the Gaia DR3 parallax catalog ==="
    );
    println!(
        "catalog {stars_path}: {n_records} records | {n_distance} distance-bearing (parallax finite and > 0, indexed) | {n_present} present without a usable parallax | {refused} reader-refused (position or field not finite)"
    );
    println!(
        "search cone {radius_as:.0} arcsec; the nearest Gaia DR3 record inside the cone is the identity, the separation is the match evidence; random background by the all-sky mean density {density:.1} stars/deg2: {background:.5} catalog star(s) per transient"
    );

    let transients: Vec<Transient> = {
        let paths = each_after(&args, "--transients");
        let mut rows: Vec<Transient> = Vec::new();
        for path in &paths {
            let Ok(text) = std::fs::read_to_string(path) else {
                eprintln!("read {path} returned void — the window stays unmeasured");
                continue;
            };
            let alerts = parse_alerts(&text);
            let kept = alerts.len();
            println!("window {path}: {kept} alert row(s) read");
            rows.extend(alerts);
        }
        rows
    };

    let mut tally = Tally {
        n: 0usize,
        placed: 0usize,
        direction_only: 0usize,
        absent: 0usize,
    };

    match (
        object_arg(&args),
        transients.is_empty(),
        arg_value(&args, "--directions"),
    ) {
        (Some((ra, dec)), _, _) => {
            tally.n += 1;
            let name = arg_value(&args, "--name");
            let t = Transient {
                id: name,
                ra_deg: ra,
                dec_deg: dec,
            };
            let _ = report(&distance, &present, &t, radius_as, &mut tally);
        }
        (None, false, None) => {
            for t in &transients {
                tally.n += 1;
                let _ = report(&distance, &present, t, radius_as, &mut tally);
            }
        }
        (None, _, Some(directions_path)) => {
            let bytes = match std::fs::read(&directions_path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("read {directions_path} returned void: {e} — the asset stays unread");
                    return;
                }
            };
            let Some(dirs) = parse_bin(&bytes) else {
                eprintln!(
                    "read {directions_path}: the SKD1 asset does not read back — the join stays closed"
                );
                return;
            };
            let out_path = arg_value(&args, "--out");
            let next = join_directions(&distance, &present, &dirs, radius_as, &mut tally);
            if let Some(out) = out_path {
                match write_bin(&next) {
                    Some(bytes) => {
                        if std::fs::write(&out, &bytes).is_err() {
                            eprintln!(
                                "write {out} returned void — the placed directions stay in memory"
                            );
                        } else {
                            let placed = next.iter().filter(|d| d.distance.is_some()).count();
                            println!(
                                "Direction-distance join: {out} written with {placed} direction(s) carrying a measured distance; the distance-less stay distance-less (0 honored)"
                            );
                            if ci_mode && !upload_asset(&out) {
                                eprintln!(
                                    "Direction-distance join: {out} did not reach the CDN release ssd.jpl.nasa.gov — the joined asset stands local, the manifest is pending"
                                );
                            }
                        }
                    }
                    None => {
                        eprintln!(
                            "write {out} returned void — a placed direction is not serializable"
                        );
                    }
                }
            }
        }
        (None, true, None) => {
            eprintln!("no transient position given — the join stays void (0 honored)");
            return;
        }
    }

    println!(
        "verdict tally: {} transient(s) | placed {} (measured distance via the Gaia identity) | direction-only {} (no Gaia DR3 record in the cone, 0 honored) | absent {} (a present record without a usable parallax)",
        tally.n, tally.placed, tally.direction_only, tally.absent
    );
}

struct Tally {
    n: usize,
    placed: usize,
    direction_only: usize,
    absent: usize,
}

fn join_directions(
    distance: &StarIndex,
    present: &StarIndex,
    dirs: &[SkyDirection],
    radius_as: f64,
    tally: &mut Tally,
) -> Vec<SkyDirection> {
    let mut next = Vec::with_capacity(dirs.len());
    for d in dirs {
        tally.n += 1;
        let t = Transient {
            id: Some(d.name.clone()),
            ra_deg: d.ra_deg,
            dec_deg: d.dec_deg,
        };
        match report(distance, present, &t, radius_as, tally) {
            Some(dist_m) => {
                let mut placed = d.clone();
                placed.distance = Some(dist_m);
                next.push(placed);
            }
            None => next.push(d.clone()),
        }
    }
    next
}

fn report(
    distance: &StarIndex,
    present: &StarIndex,
    t: &Transient,
    radius_as: f64,
    tally: &mut Tally,
) -> Option<f64> {
    match resolve(distance, present, t.ra_deg, t.dec_deg, radius_as) {
        Verdict::Placed { star, sep, within } => {
            tally.placed += 1;
            let s = &distance.stars[star];
            let d_pc = 1000.0 / s.plx_mas;
            let subj = subject(t.id.as_deref(), t.ra_deg, t.dec_deg);
            if within > 1 {
                println!(
                    "Direction-distance join: {subj} | {within} Gaia DR3 record(s) in the {radius_as:.0} arcsec cone; the nearest (separation {sep:.3} arcsec) is the identity"
                );
            }
            println!(
                "Direction-distance join: {subj} | Gaia DR3 identity record {star} at ra {:.5} dec {:.5} | separation {sep:.3} arcsec | parallax {:.3} mas -> distance {d_pc:.1} pc | placed",
                s.ra_deg, s.dec_deg, s.plx_mas
            );
            Some(d_pc * PARSEC_M)
        }
        Verdict::Absent { sep, within } => {
            tally.absent += 1;
            let subj = subject(t.id.as_deref(), t.ra_deg, t.dec_deg);
            if within > 1 {
                println!(
                    "Direction-distance join: {subj} | {within} Gaia DR3 record(s) in the {radius_as:.0} arcsec cone; the nearest (separation {sep:.3} arcsec) is present but distance-less"
                );
            }
            println!(
                "Direction-distance join: {subj} | the nearest Gaia DR3 record (separation {sep:.3} arcsec) carries no usable parallax — the distance stays absent (0 honored), the transient is present-but-unplaceable"
            );
            None
        }
        Verdict::DirectionOnly => {
            tally.direction_only += 1;
            let subj = subject(t.id.as_deref(), t.ra_deg, t.dec_deg);
            println!(
                "Direction-distance join: {subj} | no Gaia DR3 record within the {radius_as:.0} arcsec cone — the direction stays direction-only (0 honored, the absence of a counterpart is measured)"
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn star_rec(ra: f64, dec: f64, plx: f32) -> Vec<u8> {
        let mut b = Vec::with_capacity(STAR_RECORD_BYTES);
        b.extend_from_slice(&ra.to_le_bytes());
        b.extend_from_slice(&dec.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&plx.to_le_bytes());
        b.extend_from_slice(&11.0f32.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b.extend_from_slice(&0.8f32.to_le_bytes());
        b.extend_from_slice(&0.0f32.to_le_bytes());
        b
    }

    #[test]
    fn known_star_in_the_cone_is_placed_at_one_over_parallax() {
        let bytes = star_rec(246.5, -16.8, 1.512);
        let distance = build_star_index(&bytes);
        let mut refused = 0usize;
        let standins = build_present_standins(&bytes, &mut refused);
        let present = build_star_index(&standins);
        match resolve(&distance, &present, 246.5, -16.8, 5.0) {
            Verdict::Placed { star, sep, within } => {
                assert_eq!(star, 0);
                assert!(
                    sep < 0.1,
                    "the identity separation at the catalog position: {sep}"
                );
                assert_eq!(within, 1);
                let d_pc = 1000.0 / distance.stars[star].plx_mas;
                let expect = 1000.0 / (1.512f32 as f64);
                assert!(
                    (d_pc - expect).abs() < 1e-9,
                    "the f32 catalog parallax round trip sets the distance: {d_pc} vs {expect}"
                );
            }
            other => panic!("known star reads {other:?}"),
        }
    }

    #[test]
    fn empty_sky_stays_direction_only() {
        let bytes = star_rec(246.5, -16.8, 1.512);
        let distance = build_star_index(&bytes);
        let mut refused = 0usize;
        let standins = build_present_standins(&bytes, &mut refused);
        let present = build_star_index(&standins);
        match resolve(&distance, &present, 100.0, 30.0, 5.0) {
            Verdict::DirectionOnly => {}
            other => panic!("empty sky reads {other:?}"),
        }
    }

    #[test]
    fn a_negative_parallax_record_refuses_the_distance() {
        let bytes = star_rec(246.5, -16.8, -0.3);
        let distance = build_star_index(&bytes);
        assert_eq!(
            distance.stars.len(),
            0,
            "the negative-parallax record is not distance-bearing"
        );
        let mut refused = 0usize;
        let standins = build_present_standins(&bytes, &mut refused);
        let present = build_star_index(&standins);
        assert_eq!(
            present.stars.len(),
            1,
            "the negative-parallax record stays positionally present"
        );
        match resolve(&distance, &present, 246.5, -16.8, 5.0) {
            Verdict::Absent { sep, .. } => assert!(sep < 0.1),
            other => panic!("negative-parallax identity reads {other:?}"),
        }
    }

    #[test]
    fn the_nearest_record_is_the_identity_even_when_it_refuses_distance() {
        let mut bytes = star_rec(246.5, -16.8, -0.3);
        bytes.extend(star_rec(246.5003, -16.8, 1.512));
        let distance = build_star_index(&bytes);
        assert_eq!(distance.stars.len(), 1);
        let mut refused = 0usize;
        let standins = build_present_standins(&bytes, &mut refused);
        let present = build_star_index(&standins);
        assert_eq!(present.stars.len(), 1);
        match resolve(&distance, &present, 246.5, -16.8, 5.0) {
            Verdict::Absent { within, .. } => {
                assert_eq!(within, 2, "both records lie inside the cone");
            }
            other => panic!(
                "the nearer distance-less record is the identity — no distance may reach past it: {other:?}"
            ),
        }
    }

    #[test]
    fn a_delivered_distance_moves_the_direction_into_the_block_and_absent_stays_on_the_sphere() {
        let bytes = star_rec(246.5, -16.8, 1.512);
        let distance = build_star_index(&bytes);
        let mut refused = 0usize;
        let standins = build_present_standins(&bytes, &mut refused);
        let present = build_star_index(&standins);
        let mk = |name: &str, ra: f64, dec: f64| SkyDirection {
            name: name.to_string(),
            ra_deg: ra,
            dec_deg: dec,
            sigma_arcsec: None,
            bands: Vec::new(),
            distance: None,
            redshift: None,
        };
        let dirs = vec![mk("known", 246.5, -16.8), mk("void", 100.0, 30.0)];
        let mut tally = Tally {
            n: 0,
            placed: 0,
            direction_only: 0,
            absent: 0,
        };
        let next = join_directions(&distance, &present, &dirs, 5.0, &mut tally);
        assert_eq!(next.len(), 2);
        assert_eq!(tally.placed, 1);
        assert_eq!(tally.direction_only, 1);
        let expect = 1000.0 / (1.512f32 as f64) * PARSEC_M;
        let known_d = next[0].distance.unwrap();
        assert!(
            (known_d - expect).abs() < expect * 1e-9,
            "the Gaia distance lands in Some: {known_d} vs {expect}"
        );
        assert_eq!(next[1].distance, None);
        let pos = next[0].spatial_position().unwrap();
        let p = next[0].unit_direction();
        for k in 0..3 {
            assert!((pos[k] - p[k] * expect).abs() < expect * 1e-9);
        }
    }
}
