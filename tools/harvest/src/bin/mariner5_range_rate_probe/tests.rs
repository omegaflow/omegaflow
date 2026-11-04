use super::*;

const TAPE: [(f64, f64); 12] = [
    (-69498421.9, 258815.7),
    (-69494245.9, 268958.5),
    (-69493845.9, 270317.4),
    (-69493262.0, 272460.8),
    (-69492597.9, 275213.1),
    (-69492142.0, 277229.1),
    (-69491685.9, 279402.7),
    (-69491365.9, 280945.0),
    (-69491357.9, 280993.1),
    (-69485582.0, 183157.9),
    (-69485502.0, 182641.9),
    (-69484165.9, 177530.5),
];

fn spin() -> BodySpin {
    BodySpin {
        radius_m: 6_378_137.0,
        flattening: 1.0 / 298.257_223_563,
        rotation_rad_s: 360.985_623_5 * DEG / 86400.0,
        pole: [0.0, 0.0, 1.0],
        node: [0.0, 1.0, 0.0],
        cross: [-1.0, 0.0, 0.0],
        w0_deg: 190.147,
        dw_dt_deg_per_day: 360.985_623_5,
    }
}

fn goldstone() -> Station {
    Station {
        lat_deg: 35.25,
        lon_deg: 243.1,
    }
}

fn rms_residual(station: Station) -> f64 {
    let spin = spin();
    let orbit = mariner5_encounter();
    let mut squares = 0.0;
    for (t, observed) in TAPE {
        let model =
            range_rate_cps(t, orbit, station, spin, &Paths::analytic()).expect("body fixture");
        let residual = observed - model;
        squares += residual * residual;
    }
    (squares / TAPE.len() as f64).sqrt()
}

#[test]
fn the_computed_range_rate_tracks_the_tape() {
    let spin = spin();
    let orbit = mariner5_encounter();
    for (t, observed) in TAPE {
        let model =
            range_rate_cps(t, orbit, goldstone(), spin, &Paths::analytic()).expect("body fixture");
        assert!(
            (observed - model).abs() < 700.0,
            "at {t} the tape carries {observed} and the orbit gives {model}"
        );
    }
}

#[test]
fn a_thirteen_percent_scale_slip_is_visible() {
    let spin = spin();
    let orbit = mariner5_encounter();
    let (t, observed) = TAPE[0];
    let model =
        range_rate_cps(t, orbit, goldstone(), spin, &Paths::analytic()).expect("body fixture");
    let slipped = model * 1.13;
    assert!(
        (observed - slipped).abs() > 20_000.0,
        "a slip of thirteen percent reaches {slipped}, the tape carries {observed}"
    );
}

#[test]
fn goldstone_explains_the_tape_better_than_the_other_stations() {
    let goldstone = rms_residual(goldstone());
    let canberra = rms_residual(Station {
        lat_deg: -35.22,
        lon_deg: 148.98,
    });
    let madrid = rms_residual(Station {
        lat_deg: 40.43,
        lon_deg: 355.75,
    });
    assert!(
        goldstone < canberra && goldstone < madrid,
        "goldstone {goldstone:.0} canberra {canberra:.0} madrid {madrid:.0}"
    );
}

#[test]
fn the_periapsis_matches_the_reported_distance() {
    let orbit = mariner5_encounter();
    let periapsis = orbit.a * (1.0 - orbit.e);
    assert!(
        (periapsis - 10_151.0).abs() < 2.0,
        "periapsis {periapsis:.1} km against the reported 10151"
    );
}

#[test]
fn the_station_rides_the_body_rotation() {
    let spin = spin();
    let jd = -69498421.9_f64 / 86400.0 + UNIX_JD;
    let (r, v) = station_state(jd, goldstone(), spin);
    let radius = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
    let speed = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let along = (r[0] * v[0] + r[1] * v[1] + r[2] * v[2]) / radius;
    assert!(
        (radius - 6_378_137.0 / 1_000.0).abs() < 1.0,
        "radius {radius:.3} km"
    );
    assert!(
        along.abs() < 1e-9,
        "the station velocity leaves the sphere by {along}"
    );
    assert!(
        (speed - spin.rotation_rad_s * radius * goldstone().lat_deg.to_radians().cos()).abs()
            < 1e-6,
        "the station speed is {speed:.6} km/s"
    );
}
