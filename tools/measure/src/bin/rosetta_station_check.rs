use omegaflow::archivar::station::{cylindrical_from_itrf, propagate};

struct Itrf {
    name: &'static str,
    x_km: f64,
    y_km: f64,
    z_km: f64,
    rate_cm_per_year: [f64; 3],
}

struct Cylindrical {
    name: &'static str,
    longitude_deg: f64,
    spin_axis_km: f64,
    equator_km: f64,
}

const TABLE_1994: [Cylindrical; 5] = [
    Cylindrical {
        name: "DSS-15",
        longitude_deg: 243.112806431,
        spin_axis_km: 5204.2343009,
        equator_km: 3676.6699607,
    },
    Cylindrical {
        name: "DSS-24",
        longitude_deg: 243.125207105,
        spin_axis_km: 5209.4825187,
        equator_km: 3669.2423067,
    },
    Cylindrical {
        name: "DSS-25",
        longitude_deg: 243.124638324,
        spin_axis_km: 5209.6355450,
        equator_km: 3669.0405526,
    },
    Cylindrical {
        name: "DSS-54",
        longitude_deg: 355.745901454,
        spin_axis_km: 4862.8322727,
        equator_km: 4114.6186914,
    },
    Cylindrical {
        name: "DSS-55",
        longitude_deg: 355.747364983,
        spin_axis_km: 4862.9140537,
        equator_km: 4114.4949403,
    },
];

const TABLE_2005: [Itrf; 3] = [
    Itrf {
        name: "DSS-15",
        x_km: -2353.5389600,
        y_km: -4641.6494082,
        z_km: 3676.6699412,
        rate_cm_per_year: [-1.56, 0.63, -0.30],
    },
    Itrf {
        name: "DSS-24",
        x_km: -2354.9067148,
        y_km: -4646.8400771,
        z_km: 3669.2422852,
        rate_cm_per_year: [-1.56, 0.63, -0.30],
    },
    Itrf {
        name: "DSS-25",
        x_km: -2355.0220180,
        y_km: -4646.9531865,
        z_km: 3669.0405265,
        rate_cm_per_year: [-1.56, 0.63, -0.30],
    },
];

fn main() {
    let mut worst_spin = 0.0f64;
    let mut worst_equator = 0.0f64;
    let mut worst_longitude = 0.0f64;
    for later in &TABLE_2005 {
        let Some(earlier) = TABLE_1994.iter().find(|s| s.name == later.name) else {
            println!(
                "station-check: {} has no epoch-1997 counterpart",
                later.name
            );
            continue;
        };
        let rates_km_per_year = [
            later.rate_cm_per_year[0] / 1.0e5,
            later.rate_cm_per_year[1] / 1.0e5,
            later.rate_cm_per_year[2] / 1.0e5,
        ];
        let moved = propagate(later.x_km, later.y_km, later.z_km, rates_km_per_year, -8.0);
        let Some((longitude, spin_axis, equator)) =
            cylindrical_from_itrf(moved[0], moved[1], moved[2])
        else {
            println!("station-check: {} gives no cylinder", later.name);
            continue;
        };
        let d_longitude = (longitude - earlier.longitude_deg) * 3600.0;
        let d_spin = (spin_axis - earlier.spin_axis_km) * 1.0e6;
        let d_equator = (equator - earlier.equator_km) * 1.0e6;
        worst_spin = worst_spin.max(d_spin.abs());
        worst_equator = worst_equator.max(d_equator.abs());
        worst_longitude = worst_longitude.max(d_longitude.abs());
        println!(
            "station-check: {} propagated 2005.0 → 1997.0 — longitude {longitude:.9} deg (delta {d_longitude:+.1} mas), spin axis {spin_axis:.7} km (delta {d_spin:+.1} mm), equator {equator:.7} km (delta {d_equator:+.1} mm)",
            later.name
        );
    }
    println!(
        "station-check: worst of the three — {worst_longitude:.1} mas in longitude, {worst_spin:.1} mm on the spin axis, {worst_equator:.1} mm on the equator"
    );
}
