use omegaflow::archivar::json::{jstr, parse_json, JsonVal};
use omegaflow::archivar::skydirection::{parse_bin, write_bin, SkyDirection};
use omegaflow::archivar::{C_LIGHT, HUBBLE_H0, PARSEC_M};
use omegaflow::cdn::upload_asset;
use omegaflow_measure::weberin::deredden::ang_sep_arcsec;
use std::process::Command;

const NED_TAP: &str = "https://ned.ipac.caltech.edu/tap/sync";
const UA: &str = "omegaflow-direction-z-join/1.0";

struct Transient {
    id: Option<String>,
    ra_deg: f64,
    dec_deg: f64,
}

struct ZSource {
    ra_deg: f64,
    dec_deg: f64,
    z: Option<f64>,
    name: Option<String>,
}

fn cell_f64(cells: &[JsonVal], i: usize) -> Option<f64> {
    match cells.get(i)? {
        JsonVal::Num(n) => Some(*n),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}

fn cell_str(cells: &[JsonVal], i: usize) -> Option<String> {
    match cells.get(i)? {
        JsonVal::Str(s) => Some(s.clone()),
        _ => None,
    }
}

fn parse_rows(text: &str) -> Option<Vec<ZSource>> {
    let JsonVal::Obj(root) = parse_json(text)? else {
        return None;
    };
    let JsonVal::Arr(meta) = root.get("metadata")? else {
        return None;
    };
    let col = |want: &str| -> Option<usize> {
        for (i, c) in meta.iter().enumerate() {
            if jstr(c, "name").as_deref() == Some(want) {
                return Some(i);
            }
        }
        None
    };
    let (Some(i_ra), Some(i_dec), Some(i_z)) = (col("ra"), col("dec"), col("z")) else {
        return None;
    };
    let i_name = col("prefname");
    let JsonVal::Arr(data) = root.get("data")? else {
        return None;
    };
    let mut out = Vec::with_capacity(data.len());
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        let (Some(ra), Some(dec)) = (cell_f64(cells, i_ra), cell_f64(cells, i_dec)) else {
            continue;
        };
        if !(ra.is_finite()
            && dec.is_finite()
            && (0.0..=360.0).contains(&ra)
            && (-90.0..=90.0).contains(&dec))
        {
            continue;
        }
        let z = cell_f64(cells, i_z).filter(|v| v.is_finite());
        let name = i_name.and_then(|i| cell_str(cells, i));
        out.push(ZSource { ra_deg: ra, dec_deg: dec, z, name });
    }
    Some(out)
}

enum ConeResult {
    Rows(Vec<ZSource>),
    Unanswered(String),
}

fn ned_query(ra: f64, dec: f64, radius_as: f64) -> Option<(String, Vec<u8>)> {
    let r_deg = radius_as / 3600.0;
    let query = format!(
        "SELECT ra,dec,z,prefname FROM NEDTAP.objdir WHERE CONTAINS(POINT('ICRS',ra,dec), CIRCLE('ICRS',{ra},{dec},{r_deg}))=1"
    );
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-A")
        .arg(UA)
        .arg("-G")
        .arg(NED_TAP)
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=json")
        .arg("--data-urlencode")
        .arg(format!("QUERY={query}"))
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn ned_cone(ra: f64, dec: f64, radius_as: f64) -> ConeResult {
    match ned_query(ra, dec, radius_as) {
        None => ConeResult::Unanswered("a stalled cone query".to_string()),
        Some((code, body)) => {
            if code != "200" {
                return ConeResult::Unanswered(format!("HTTP {code}"));
            }
            match std::str::from_utf8(&body) {
                Ok(text) => match parse_rows(text) {
                    Some(rows) => ConeResult::Rows(rows),
                    None => ConeResult::Unanswered("a body that is not the measured TAP JSON".to_string()),
                },
                Err(_) => ConeResult::Unanswered("a body that is not UTF-8".to_string()),
            }
        }
    }
}

#[derive(Debug)]
enum Verdict {
    Placed {
        idx: usize,
        sep: f64,
        within: usize,
    },
    Absent {
        within: usize,
    },
    DirectionOnly,
}

fn resolve(rows: &[ZSource], ra: f64, dec: f64, radius_as: f64) -> Verdict {
    let mut present: Vec<(usize, f64)> = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        let sep = ang_sep_arcsec(ra, dec, r.ra_deg, r.dec_deg);
        if sep <= radius_as {
            present.push((i, sep));
        }
    }
    present.sort_by(|a, b| a.1.total_cmp(&b.1));
    let within = present.len();
    if within == 0 {
        return Verdict::DirectionOnly;
    }
    for (i, sep) in present {
        match rows[i].z {
            Some(z) if z.is_finite() && z > 0.0 => {
                return Verdict::Placed {
                    idx: i,
                    sep,
                    within,
                };
            }
            _ => {}
        }
    }
    Verdict::Absent { within }
}

fn subject(id: Option<&str>, ra: f64, dec: f64) -> String {
    match id {
        Some(n) => format!("{n} at ra {ra:.5} dec {dec:.5}"),
        None => format!("ra {ra:.5} dec {dec:.5}"),
    }
}

fn hubble_mpc(z: f64) -> f64 {
    let d_m = z * C_LIGHT / HUBBLE_H0;
    d_m / (PARSEC_M * 1.0e6)
}

struct Tally {
    n: usize,
    placed: usize,
    absent: usize,
    direction_only: usize,
    pending: usize,
}

fn report(rows: &[ZSource], t: &Transient, radius_as: f64, tally: &mut Tally) -> Option<f64> {
    let subj = subject(t.id.as_deref(), t.ra_deg, t.dec_deg);
    match resolve(rows, t.ra_deg, t.dec_deg, radius_as) {
        Verdict::Placed { idx, sep, within } => {
            tally.placed += 1;
            let r = &rows[idx];
            if within > 1 {
                println!(
                    "direction_z_join: {subj} | {within} NED objdir record(s) in the {radius_as:.0} arcsec cone; the nearest redshift-bearing record (separation {sep:.3} arcsec) carries the measurement"
                );
            }
            let z = match r.z {
                Some(z) => z,
                None => {
                    println!(
                        "direction_z_join: {subj} | the nearest record lost its measured redshift between resolve and report — the redshift stays absent (0 honored)"
                    );
                    return None;
                }
            };
            let id_s = match &r.name {
                Some(n) => n.clone(),
                None => "the nearest NED objdir record".to_string(),
            };
            println!(
                "direction_z_join: {subj} | {id_s} at ra {:.5} dec {:.5} | separation {sep:.3} arcsec | measured redshift z {z:.6} -> Hubble distance {:.2} Mpc | placed",
                r.ra_deg, r.dec_deg, hubble_mpc(z)
            );
            Some(z)
        }
        Verdict::Absent { within } => {
            tally.absent += 1;
            println!(
                "direction_z_join: {subj} | {within} NED objdir record(s) within the {radius_as:.0} arcsec cone, none carries a measured positive redshift — the redshift stays absent (0 honored)"
            );
            None
        }
        Verdict::DirectionOnly => {
            tally.direction_only += 1;
            println!(
                "direction_z_join: {subj} | no NED objdir record within the {radius_as:.0} arcsec cone — the redshift stays absent (0 honored, the empty cone is measured)"
            );
            None
        }
    }
}

fn run_probe(t: &Transient, radius_as: f64, tally: &mut Tally) -> Option<f64> {
    let subj = subject(t.id.as_deref(), t.ra_deg, t.dec_deg);
    match ned_cone(t.ra_deg, t.dec_deg, radius_as) {
        ConeResult::Rows(rows) => report(&rows, t, radius_as, tally),
        ConeResult::Unanswered(reason) => {
            tally.pending += 1;
            println!(
                "direction_z_join: {subj} | the NED objdir cone query did not answer ({reason}) — the redshift stays pending, never invented"
            );
            None
        }
    }
}

fn join_directions(dirs: &[SkyDirection], radius_as: f64, tally: &mut Tally) -> Vec<SkyDirection> {
    let mut next = Vec::with_capacity(dirs.len());
    for d in dirs {
        tally.n += 1;
        let t = Transient {
            id: Some(d.name.clone()),
            ra_deg: d.ra_deg,
            dec_deg: d.dec_deg,
        };
        match run_probe(&t, radius_as, tally) {
            Some(z) => {
                let mut placed = d.clone();
                placed.redshift = Some(z);
                next.push(placed);
            }
            None => next.push(d.clone()),
        }
    }
    next
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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
        "usage: direction_z_join --radius <arcsec> (--object <ra> <dec> [--name <id>] | --directions <skd1> [--out <skd1>]) [--ci-mode]"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
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

    let mut tally = Tally {
        n: 0,
        placed: 0,
        absent: 0,
        direction_only: 0,
        pending: 0,
    };

    match (object_arg(&args), arg_value(&args, "--directions")) {
        (Some((ra, dec)), _) => {
            tally.n += 1;
            let name = arg_value(&args, "--name");
            let t = Transient {
                id: name,
                ra_deg: ra,
                dec_deg: dec,
            };
            let _ = run_probe(&t, radius_as, &mut tally);
        }
        (None, Some(directions_path)) => {
            let bytes = match std::fs::read(&directions_path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!(
                        "read {directions_path} returned void: {e} — the asset stays unread"
                    );
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
            let next = join_directions(&dirs, radius_as, &mut tally);
            if let Some(out) = out_path {
                match write_bin(&next) {
                    Some(bytes) => {
                        if std::fs::write(&out, &bytes).is_err() {
                            eprintln!(
                                "write {out} returned void — the redshift-bearing directions stay in memory"
                            );
                        } else {
                            let placed = next.iter().filter(|d| d.redshift.is_some()).count();
                            println!(
                                "Direction-redshift join: {out} written with {placed} direction(s) carrying a measured redshift; the redshift-less stay redshift-less (0 honored)"
                            );
                            if ci_mode && !upload_asset(&out) {
                                eprintln!(
                                    "direction_z_join: {out} did not reach the CDN release ssd.jpl.nasa.gov — the joined asset stands local, the manifest is pending"
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
        (None, None) => {
            eprintln!("no direction position given — the join stays void (0 honored)");
            return;
        }
    }

    println!(
        "verdict tally: {} direction(s) | placed {} (measured redshift via the NED objdir identity) | redshift-less present {} (record(s) in the cone without a measured z, 0 honored) | no counterpart {} (empty cone, 0 honored) | pending {} (the cone query stayed unanswered)",
        tally.n, tally.placed, tally.absent, tally.direction_only, tally.pending
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::skydirection::{SkyBandSeries, SkySample};

    fn dir(name: &str, ra: f64, dec: f64) -> SkyDirection {
        SkyDirection {
            name: name.to_string(),
            ra_deg: ra,
            dec_deg: dec,
            sigma_arcsec: None,
            bands: Vec::new(),
            distance: None,
            redshift: None,
        }
    }

    fn src(ra: f64, dec: f64, z: Option<f64>, name: &str) -> ZSource {
        ZSource {
            ra_deg: ra,
            dec_deg: dec,
            z,
            name: Some(name.to_string()),
        }
    }

    #[test]
    fn a_measured_redshift_in_the_cone_places_the_direction_and_distance_m_is_finite() {
        let rows = vec![src(
            246.2986,
            -16.1643,
            Some(0.0157),
            "WISEA J162511.60-160951.4",
        )];
        let mut d = dir("ZTF19aamxqli", 246.29813, -16.16443);
        match resolve(&rows, d.ra_deg, d.dec_deg, 5.0) {
            Verdict::Placed { idx, sep, within } => {
                assert_eq!(idx, 0);
                assert_eq!(within, 1);
                assert!(sep < 5.0, "the record lies inside the cone: {sep}");
                d.redshift = Some(rows[idx].z.unwrap());
                let dm = d.distance_m().unwrap();
                let expect = 0.0157 * C_LIGHT / HUBBLE_H0;
                assert!(dm.is_finite());
                assert!(
                    (dm - expect).abs() < expect * 1e-9,
                    "the measured redshift reaches the Hubble distance: {dm} vs {expect}"
                );
            }
            other => panic!("a z-bearing record in the cone reads {other:?}"),
        }
    }

    #[test]
    fn a_cone_without_a_measured_redshift_leaves_the_direction_direction_only_absent() {
        let d = dir("absent_z", 100.0, 30.0);
        match resolve(&[], d.ra_deg, d.dec_deg, 5.0) {
            Verdict::DirectionOnly => {}
            other => panic!("an empty cone reads {other:?}"),
        }
        let rows = vec![src(100.0, 30.0, None, "no-redshift-host")];
        match resolve(&rows, d.ra_deg, d.dec_deg, 5.0) {
            Verdict::Absent { within } => assert_eq!(within, 1),
            other => panic!("a redshift-less present record reads {other:?}"),
        }
        let mut d = dir("absent_z", 100.0, 30.0);
        d.redshift = None;
        assert_eq!(d.distance_m(), None);
        let mut d = d.clone();
        d.redshift = Some(0.0);
        assert_eq!(d.distance_m(), None);
        let mut d = dir("absent_z", 100.0, 30.0);
        d.redshift = Some(-0.1);
        assert_eq!(d.distance_m(), None);
    }

    #[test]
    fn the_nearest_record_with_a_measured_redshift_is_chosen() {
        let rows = vec![
            src(10.0, 20.0, None, "near-without-z"),
            src(10.0001, 20.0, Some(0.3), "far-with-z"),
            src(10.00005, 20.00005, Some(0.05), "near-with-z"),
        ];
        match resolve(&rows, 10.0, 20.0, 60.0) {
            Verdict::Placed { idx, sep, .. } => {
                assert_eq!(rows[idx].name.as_deref(), Some("near-with-z"));
                assert_eq!(rows[idx].z, Some(0.05));
                assert!(sep < 0.3, "the nearest z-bearing record separation: {sep}");
            }
            other => panic!("the nearest z-bearing record reads {other:?}"),
        }
    }

    #[test]
    fn parse_rows_reads_the_measured_ned_tap_json_with_absent_z_as_none() {
        let text = r#"{"metadata":[{"name":"ra","datatype":"DOUBLE"},{"name":"dec","datatype":"DOUBLE"},{"name":"z","datatype":"DOUBLE"},{"name":"prefname","arraysize":"30"}],"data":[[246.2983487,-16.1642787,null,"WISEA J162511.60-160951.4"],[148.8746037,2.5208939,0.963542,"WISEA J095529.90+023115.2"]]}"#;
        let rows = parse_rows(text).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name.as_deref(), Some("WISEA J162511.60-160951.4"));
        assert_eq!(rows[0].z, None);
        assert_eq!(rows[1].name.as_deref(), Some("WISEA J095529.90+023115.2"));
        assert_eq!(rows[1].z, Some(0.963542));
    }

    #[test]
    fn parse_rows_refuses_a_body_that_is_not_the_measured_tap_shape() {
        assert!(parse_rows("{\"metadata\":[{\"name\":\"ra\"}],\"data\":[]}").is_none());
        assert!(parse_rows("not json").is_none());
    }

    #[test]
    fn a_measured_redshift_roundtrips_through_the_skd1_wire() {
        let mut d = dir("redshift_wire", 148.8746, 2.5208);
        d.redshift = Some(0.963542);
        let bytes = write_bin(&[d.clone()]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].redshift, Some(0.963542));
        let dm = parsed[0].distance_m().unwrap();
        let expect = 0.963542 * C_LIGHT / HUBBLE_H0;
        assert!((dm - expect).abs() < expect * 1e-9);
        assert!(parsed[0].spatial_position().is_some());
    }

    #[test]
    fn redshift_absent_stays_absent_through_the_skd1_wire() {
        let d = dir("redshift_absent", 148.8746, 2.5208);
        assert_eq!(d.distance_m(), None);
        let bytes = write_bin(&[d]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].redshift, None);
        assert_eq!(parsed[0].distance_m(), None);
    }

    #[test]
    fn the_held_direction_band_series_survive_a_redshift_placement() {
        let mut d = SkyDirection {
            name: "ZTF21abxxjrh".to_string(),
            ra_deg: 37.284397,
            dec_deg: 9.258595,
            sigma_arcsec: None,
            bands: vec![SkyBandSeries {
                band: Some("g".to_string()),
                samples: vec![SkySample {
                    tdb: 8.2e8,
                    mag: 19.45389747619629,
                }],
            }],
            distance: None,
            redshift: None,
        };
        d.redshift = Some(0.084);
        let bytes = write_bin(&[d]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].bands[0].samples[0].mag, 19.45389747619629);
        let dm = parsed[0].distance_m().unwrap();
        let expect = 0.084 * C_LIGHT / HUBBLE_H0;
        assert!((dm - expect).abs() < expect * 1e-9);
    }
}
