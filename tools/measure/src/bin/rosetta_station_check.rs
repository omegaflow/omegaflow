use omegaflow::archivar::station::{
    cartesian_from_cylindrical, cylindrical_from_itrf, enu_basis, project_enu, propagate,
};

struct Cylindrical {
    name: &'static str,
    longitude_deg: f64,
    spin_axis_km: f64,
    equator_km: f64,
    site_rate_cm_per_year: [f64; 3],
}

struct Itrf {
    name: &'static str,
    x_km: f64,
    y_km: f64,
    z_km: f64,
    rate_cm_per_year: [f64; 3],
}

const TABLE_1997: [Cylindrical; 5] = [
    Cylindrical {
        name: "DSS-15",
        longitude_deg: 243.112806431,
        spin_axis_km: 5204.2343009,
        equator_km: 3676.6699607,
        site_rate_cm_per_year: [-1.61, 0.59, -0.51],
    },
    Cylindrical {
        name: "DSS-24",
        longitude_deg: 243.125207105,
        spin_axis_km: 5209.4825187,
        equator_km: 3669.2423067,
        site_rate_cm_per_year: [-1.61, 0.59, -0.51],
    },
    Cylindrical {
        name: "DSS-25",
        longitude_deg: 243.124638324,
        spin_axis_km: 5209.6355450,
        equator_km: 3669.0405526,
        site_rate_cm_per_year: [-1.61, 0.59, -0.51],
    },
    Cylindrical {
        name: "DSS-54",
        longitude_deg: 355.745901454,
        spin_axis_km: 4862.8322727,
        equator_km: 4114.6186914,
        site_rate_cm_per_year: [-0.76, 1.96, 1.29],
    },
    Cylindrical {
        name: "DSS-55",
        longitude_deg: 355.747364983,
        spin_axis_km: 4862.9140537,
        equator_km: 4114.4949403,
        site_rate_cm_per_year: [-0.76, 1.96, 1.29],
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

struct Naif {
    name: &'static str,
    x_km: f64,
    y_km: f64,
    z_km: f64,
}

const JPL_2005: [Naif; 5] = [
    Naif {
        name: "DSS-15",
        x_km: -2353.5389600,
        y_km: -4641.6494082,
        z_km: 3676.6699412,
    },
    Naif {
        name: "DSS-24",
        x_km: -2354.9067148,
        y_km: -4646.8400771,
        z_km: 3669.2422852,
    },
    Naif {
        name: "DSS-25",
        x_km: -2355.0220181,
        y_km: -4646.9531865,
        z_km: 3669.0405265,
    },
    Naif {
        name: "DSS-54",
        x_km: 4849.4344781,
        y_km: -360.7238954,
        z_km: 4114.6188518,
    },
    Naif {
        name: "DSS-55",
        x_km: 4849.5252463,
        y_km: -360.6060885,
        z_km: 4114.4951006,
    },
];

const NAIF_EPOCH: f64 = 2020.81;

const NAIF_2020: [Naif; 5] = [
    Naif {
        name: "DSS-15",
        x_km: -2353.5392785750,
        y_km: -4641.6493132370,
        z_km: 3676.6699163230,
    },
    Naif {
        name: "DSS-24",
        x_km: -2354.9070315750,
        y_km: -4646.8399792370,
        z_km: 3669.2422573230,
    },
    Naif {
        name: "DSS-25",
        x_km: -2355.0223345750,
        y_km: -4646.9530882370,
        z_km: 3669.0404993230,
    },
    Naif {
        name: "DSS-54",
        x_km: 4849.4343099030,
        y_km: -360.7234689048,
        z_km: 4114.6191128320,
    },
    Naif {
        name: "DSS-55",
        x_km: 4849.5250779030,
        y_km: -360.6056622048,
        z_km: 4114.4953618320,
    },
];

const NN1_1997: [f64; 3] = [-2414.066824, 4907.869366, -3270.605535];
const NN1_2015: [f64; 3] = [-2414.0676590, 4907.8695120, -3270.6046270];
const NN1_RATE_CM_PER_YEAR: [f64; 3] = [-4.75, 0.94, 4.98];

fn geodetic_latitude_deg(x: f64, y: f64, z: f64) -> Option<f64> {
    let a = 6378.137;
    let f = 1.0 / 298.257223563;
    let e2 = 2.0 * f - f * f;
    let p = (x * x + y * y).sqrt();
    if !p.is_finite() || p <= 0.0 {
        return None;
    }
    let mut lat = z.atan2(p * (1.0 - e2));
    let mut h;
    for _ in 0..6 {
        let sl = lat.sin();
        let n = a / (1.0 - e2 * sl * sl).sqrt();
        h = p / lat.cos() - n;
        lat = z.atan2(p * (1.0 - e2 * n / (n + h)));
    }
    Some(lat.to_degrees())
}

fn enu_residual(target: [f64; 3], moved: [f64; 3]) -> Option<([f64; 3], f64)> {
    let latitude = geodetic_latitude_deg(moved[0], moved[1], moved[2])?;
    let longitude = moved[1].atan2(moved[0]).to_degrees();
    let basis = enu_basis(latitude, longitude)?;
    let displacement = [
        (target[0] - moved[0]) * 1.0e6,
        (target[1] - moved[1]) * 1.0e6,
        (target[2] - moved[2]) * 1.0e6,
    ];
    let enu = project_enu(basis, displacement);
    let magnitude = (enu[0] * enu[0] + enu[1] * enu[1] + enu[2] * enu[2]).sqrt();
    Some((enu, magnitude))
}

fn solve3(a: [[f64; 3]; 3], b: [f64; 3]) -> Option<[f64; 3]> {
    let det = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
    if !det.is_finite() || det.abs() < 1e-18 {
        return None;
    }
    let mut out = [0.0; 3];
    for column in 0..3 {
        let mut m = a;
        for row in 0..3 {
            m[row][column] = b[row];
        }
        let d = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
        out[column] = d / det;
    }
    Some(out)
}

fn rotation_fit(pairs: &[([f64; 3], [f64; 3])]) -> Option<([f64; 3], f64, f64)> {
    let mut a = [[0.0f64; 3]; 3];
    let mut b = [0.0f64; 3];
    for (r, d) in pairs {
        let r2 = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
        for i in 0..3 {
            for j in 0..3 {
                a[i][j] += if i == j { r2 } else { 0.0 } - r[i] * r[j];
            }
        }
        let cross = [
            r[1] * d[2] - r[2] * d[1],
            r[2] * d[0] - r[0] * d[2],
            r[0] * d[1] - r[1] * d[0],
        ];
        for i in 0..3 {
            b[i] += cross[i];
        }
    }
    let omega = solve3(a, b)?;
    let mut before = 0.0;
    let mut after = 0.0;
    for (r, d) in pairs {
        before += d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
        let predicted = [
            omega[1] * r[2] - omega[2] * r[1],
            omega[2] * r[0] - omega[0] * r[2],
            omega[0] * r[1] - omega[1] * r[0],
        ];
        let e = [
            d[0] - predicted[0],
            d[1] - predicted[1],
            d[2] - predicted[2],
        ];
        after += e[0] * e[0] + e[1] * e[1] + e[2] * e[2];
    }
    Some((omega, before.sqrt(), after.sqrt()))
}

fn solve_n(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for column in 0..n {
        let mut pivot = column;
        for row in column + 1..n {
            if a[row][column].abs() > a[pivot][column].abs() {
                pivot = row;
            }
        }
        if a[pivot][column].abs() < 1e-18 {
            return None;
        }
        a.swap(column, pivot);
        b.swap(column, pivot);
        for row in column + 1..n {
            let factor = a[row][column] / a[column][column];
            for k in column..n {
                a[row][k] -= factor * a[column][k];
            }
            b[row] -= factor * b[column];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let mut sum = b[row];
        for k in row + 1..n {
            sum -= a[row][k] * x[k];
        }
        x[row] = sum / a[row][row];
    }
    Some(x)
}

fn rigid_fit(pairs: &[([f64; 3], [f64; 3])]) -> Option<([f64; 3], [f64; 3], f64, f64)> {
    let mut normal = vec![vec![0.0f64; 6]; 6];
    let mut rhs = vec![0.0f64; 6];
    for (r, d) in pairs {
        let rows = [
            [1.0, 0.0, 0.0, 0.0, r[2], -r[1]],
            [0.0, 1.0, 0.0, -r[2], 0.0, r[0]],
            [0.0, 0.0, 1.0, r[1], -r[0], 0.0],
        ];
        for (row, target) in rows.iter().zip(d.iter()) {
            for i in 0..6 {
                for j in 0..6 {
                    normal[i][j] += row[i] * row[j];
                }
                rhs[i] += row[i] * target;
            }
        }
    }
    let x = solve_n(normal, rhs)?;
    let translation = [x[0], x[1], x[2]];
    let omega = [x[3], x[4], x[5]];
    let mut before = 0.0;
    let mut after = 0.0;
    for (r, d) in pairs {
        before += d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
        let predicted = [
            translation[0] + omega[1] * r[2] - omega[2] * r[1],
            translation[1] + omega[2] * r[0] - omega[0] * r[2],
            translation[2] + omega[0] * r[1] - omega[1] * r[0],
        ];
        let e = [
            d[0] - predicted[0],
            d[1] - predicted[1],
            d[2] - predicted[2],
        ];
        after += e[0] * e[0] + e[1] * e[1] + e[2] * e[2];
    }
    Some((translation, omega, before.sqrt(), after.sqrt()))
}

fn residual_after_rigid(translation: [f64; 3], omega: [f64; 3], r: [f64; 3], d: [f64; 3]) -> f64 {
    let predicted = [
        translation[0] + omega[1] * r[2] - omega[2] * r[1],
        translation[1] + omega[2] * r[0] - omega[0] * r[2],
        translation[2] + omega[0] * r[1] - omega[1] * r[0],
    ];
    let e = [
        d[0] - predicted[0],
        d[1] - predicted[1],
        d[2] - predicted[2],
    ];
    (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt() * 1.0e6
}

fn velocity_offset(pairs: &[([f64; 3], [f64; 3])], years: f64) -> Option<([f64; 3], f64)> {
    if pairs.is_empty() || !years.is_finite() || years <= 0.0 {
        return None;
    }
    let mut mean = [0.0f64; 3];
    for (_, d) in pairs {
        for axis in 0..3 {
            mean[axis] += d[axis];
        }
    }
    for axis in 0..3 {
        mean[axis] /= pairs.len() as f64;
    }
    let rate = [
        mean[0] / years * 1.0e6,
        mean[1] / years * 1.0e6,
        mean[2] / years * 1.0e6,
    ];
    let mut worst = 0.0f64;
    for (_, d) in pairs {
        let e = [d[0] - mean[0], d[1] - mean[1], d[2] - mean[2]];
        worst = worst.max((e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt() * 1.0e6);
    }
    Some((rate, worst))
}

fn itrf_frame(
    x_m: [f64; 3],
    t_mm: [f64; 3],
    t_rate_mm: [f64; 3],
    d_ppb: f64,
    d_rate_ppb: f64,
    at: f64,
) -> [f64; 3] {
    let dt = at - 2015.0;
    let scale = 1.0 + (d_ppb + d_rate_ppb * dt) * 1.0e-9;
    let t = [
        (t_mm[0] + t_rate_mm[0] * dt) / 1000.0,
        (t_mm[1] + t_rate_mm[1] * dt) / 1000.0,
        (t_mm[2] + t_rate_mm[2] * dt) / 1000.0,
    ];
    [
        x_m[0] * scale + t[0],
        x_m[1] * scale + t[1],
        x_m[2] * scale + t[2],
    ]
}

fn itrf_frame_inverse(
    x_m: [f64; 3],
    t_mm: [f64; 3],
    t_rate_mm: [f64; 3],
    d_ppb: f64,
    d_rate_ppb: f64,
    at: f64,
) -> [f64; 3] {
    let dt = at - 2015.0;
    let scale = 1.0 + (d_ppb + d_rate_ppb * dt) * 1.0e-9;
    let t = [
        (t_mm[0] + t_rate_mm[0] * dt) / 1000.0,
        (t_mm[1] + t_rate_mm[1] * dt) / 1000.0,
        (t_mm[2] + t_rate_mm[2] * dt) / 1000.0,
    ];
    [
        (x_m[0] - t[0]) / scale,
        (x_m[1] - t[1]) / scale,
        (x_m[2] - t[2]) / scale,
    ]
}

fn frame_2000_from_2008(x_m: [f64; 3], at: f64) -> [f64; 3] {
    let x2020 = itrf_frame_inverse(x_m, [0.2, 1.0, 3.3], [0.0, -0.1, 0.1], -0.29, 0.03, at);
    itrf_frame(x2020, [-0.2, 0.8, -34.2], [0.1, 0.0, -1.7], 2.25, 0.11, at)
}

fn main() {
    let mut worst_document = 0.0f64;
    let mut worst_station = 0.0f64;
    let mut station_pairs: Vec<([f64; 3], [f64; 3])> = Vec::new();
    for earlier in &TABLE_1997 {
        let Some(later) = TABLE_2005.iter().find(|s| s.name == earlier.name) else {
            println!(
                "station-check: {} carries no epoch-2005 counterpart in the document",
                earlier.name
            );
            continue;
        };
        let Some(cartesian) = cartesian_from_cylindrical(
            earlier.longitude_deg,
            earlier.spin_axis_km,
            earlier.equator_km,
        ) else {
            println!("station-check: {} gives no Cartesian point", earlier.name);
            continue;
        };
        let document_rate = [
            earlier.site_rate_cm_per_year[0] / 1.0e5,
            earlier.site_rate_cm_per_year[1] / 1.0e5,
            earlier.site_rate_cm_per_year[2] / 1.0e5,
        ];
        let station_rate = [
            later.rate_cm_per_year[0] / 1.0e5,
            later.rate_cm_per_year[1] / 1.0e5,
            later.rate_cm_per_year[2] / 1.0e5,
        ];
        let target = [later.x_km, later.y_km, later.z_km];

        let with_document = propagate(cartesian[0], cartesian[1], cartesian[2], document_rate, 8.0);
        let with_station = propagate(cartesian[0], cartesian[1], cartesian[2], station_rate, 8.0);
        match (
            enu_residual(target, with_document),
            enu_residual(target, with_station),
        ) {
            (Some((enu_d, mag_d)), Some((enu_s, mag_s))) => {
                worst_document = worst_document.max(mag_d);
                worst_station = worst_station.max(mag_s);
                station_pairs.push((
                    with_station,
                    [
                        target[0] - with_station[0],
                        target[1] - with_station[1],
                        target[2] - with_station[2],
                    ],
                ));
                println!(
                    "station-check: {} 1997.0 → 2005.0 — document site rate: east {:+6.1} north {:+6.1} up {:+6.1} mm (|d| {:.1}); station rate: east {:+6.1} north {:+6.1} up {:+6.1} mm (|d| {:.1})",
                    earlier.name,
                    enu_d[0],
                    enu_d[1],
                    enu_d[2],
                    mag_d,
                    enu_s[0],
                    enu_s[1],
                    enu_s[2],
                    mag_s
                );
            }
            _ => println!("station-check: {} gives no ENU residual", earlier.name),
        }
    }
    println!(
        "station-check: worst over the three Goldstone antennas — {worst_document:.1} mm with the document site rate, {worst_station:.1} mm with the station rate"
    );
    for earlier in &TABLE_1997 {
        let Some(later) = TABLE_2005.iter().find(|s| s.name == earlier.name) else {
            continue;
        };
        let Some(cartesian) = cartesian_from_cylindrical(
            earlier.longitude_deg,
            earlier.spin_axis_km,
            earlier.equator_km,
        ) else {
            continue;
        };
        let document_rate = [
            earlier.site_rate_cm_per_year[0] / 1.0e5,
            earlier.site_rate_cm_per_year[1] / 1.0e5,
            earlier.site_rate_cm_per_year[2] / 1.0e5,
        ];
        let station_rate = [
            later.rate_cm_per_year[0] / 1.0e5,
            later.rate_cm_per_year[1] / 1.0e5,
            later.rate_cm_per_year[2] / 1.0e5,
        ];
        let with_document = propagate(cartesian[0], cartesian[1], cartesian[2], document_rate, 8.0);
        let with_station = propagate(cartesian[0], cartesian[1], cartesian[2], station_rate, 8.0);
        let spread = [
            (with_station[0] - with_document[0]) * 1.0e6,
            (with_station[1] - with_document[1]) * 1.0e6,
            (with_station[2] - with_document[2]) * 1.0e6,
        ];
        let spread_mm =
            (spread[0] * spread[0] + spread[1] * spread[1] + spread[2] * spread[2]).sqrt();
        println!(
            "station-check: {} — the document prints two velocity rows for Goldstone; over eight years they part by {:.1} mm (dx {:+.1}, dy {:+.1}, dz {:+.1})",
            earlier.name, spread_mm, spread[0], spread[1], spread[2]
        );
    }

    match velocity_offset(&station_pairs, 8.0) {
        Some((rate, spread)) => println!(
            "station-check: the 1997→2005 residual reads as one velocity offset of the Goldstone site — ({:+.2}, {:+.2}, {:+.2}) mm/year, leaving {:.1} mm spread across the three antennas",
            rate[0], rate[1], rate[2], spread
        ),
        None => println!("station-check: the residual carries no velocity offset"),
    }

    match rotation_fit(&station_pairs) {
        Some((omega, before, after)) => {
            let magnitude_mas = (omega[0] * omega[0] + omega[1] * omega[1] + omega[2] * omega[2])
                .sqrt()
                * 206_264_806.0;
            println!(
                "station-check: the 1997→2005 residual is a rotation of {magnitude_mas:.3} mas about ({:+.3e}, {:+.3e}, {:+.3e}) rad — {:.1} mm before the fit, {:.1} mm after",
                omega[0],
                omega[1],
                omega[2],
                before * 1.0e6,
                after * 1.0e6
            );
        }
        None => println!("station-check: the residual carries no rotation"),
    }

    let mut naif_pairs: Vec<([f64; 3], [f64; 3])> = Vec::new();
    for naif in &NAIF_2020 {
        let source = match TABLE_2005.iter().find(|s| s.name == naif.name) {
            Some(later) => (
                [later.x_km, later.y_km, later.z_km],
                [
                    later.rate_cm_per_year[0] / 1.0e5,
                    later.rate_cm_per_year[1] / 1.0e5,
                    later.rate_cm_per_year[2] / 1.0e5,
                ],
                2005.0,
            ),
            None => match TABLE_1997.iter().find(|s| s.name == naif.name) {
                Some(earlier) => match cartesian_from_cylindrical(
                    earlier.longitude_deg,
                    earlier.spin_axis_km,
                    earlier.equator_km,
                ) {
                    Some(cartesian) => (
                        cartesian,
                        [
                            earlier.site_rate_cm_per_year[0] / 1.0e5,
                            earlier.site_rate_cm_per_year[1] / 1.0e5,
                            earlier.site_rate_cm_per_year[2] / 1.0e5,
                        ],
                        1997.0,
                    ),
                    None => {
                        println!(
                            "station-check: {} gives no Cartesian point for the NAIF comparison",
                            naif.name
                        );
                        continue;
                    }
                },
                None => {
                    println!(
                        "station-check: {} carries no counterpart for the NAIF comparison",
                        naif.name
                    );
                    continue;
                }
            },
        };
        let (start, rate, epoch) = source;
        let years = NAIF_EPOCH - epoch;
        let moved = propagate(start[0], start[1], start[2], rate, years);
        naif_pairs.push((
            moved,
            [
                naif.x_km - moved[0],
                naif.y_km - moved[1],
                naif.z_km - moved[2],
            ],
        ));
        match enu_residual([naif.x_km, naif.y_km, naif.z_km], moved) {
            Some((enu, magnitude)) => println!(
                "station-check: {} against NAIF at {NAIF_EPOCH:.2} — east {:+6.1} north {:+6.1} up {:+6.1} mm (|d| {magnitude:.1}) over {years:.1} years",
                naif.name, enu[0], enu[1], enu[2]
            ),
            None => println!("station-check: {} gives no NAIF residual", naif.name),
        }
    }

    match rotation_fit(&naif_pairs) {
        Some((omega, before, after)) => {
            let magnitude_mas = (omega[0] * omega[0] + omega[1] * omega[1] + omega[2] * omega[2])
                .sqrt()
                * 206_264_806.0;
            println!(
                "station-check: the NAIF residual is a rotation of {magnitude_mas:.3} mas about ({:+.3e}, {:+.3e}, {:+.3e}) rad — {:.1} mm before the fit, {:.1} mm after",
                omega[0],
                omega[1],
                omega[2],
                before * 1.0e6,
                after * 1.0e6
            );
        }
        None => println!("station-check: the NAIF residual carries no rotation"),
    }

    match rigid_fit(&naif_pairs) {
        Some((translation, omega, before, after)) => {
            let omega_mas = (omega[0] * omega[0] + omega[1] * omega[1] + omega[2] * omega[2])
                .sqrt()
                * 206_264_806.0;
            println!(
                "station-check: the NAIF residual under one rigid translation and rotation — T ({:+.1}, {:+.1}, {:+.1}) mm, |omega| {omega_mas:.3} mas, {:.1} mm before the fit, {:.1} mm after",
                translation[0] * 1.0e6,
                translation[1] * 1.0e6,
                translation[2] * 1.0e6,
                before * 1.0e6,
                after * 1.0e6
            );
            for (index, naif) in NAIF_2020.iter().enumerate() {
                if let Some((r, d)) = naif_pairs.get(index) {
                    let rest = residual_after_rigid(translation, omega, *r, *d);
                    println!(
                        "station-check: {} keeps {rest:.1} mm after the rigid fit",
                        naif.name
                    );
                }
            }
        }
        None => println!("station-check: the NAIF residual carries no rigid fit"),
    }

    for jpl in &JPL_2005 {
        let mut delta: Option<[f64; 3]> = None;
        if let Some(esa) = TABLE_2005.iter().find(|s| s.name == jpl.name) {
            delta = Some([
                (esa.x_km - jpl.x_km) * 1.0e6,
                (esa.y_km - jpl.y_km) * 1.0e6,
                (esa.z_km - jpl.z_km) * 1.0e6,
            ]);
        } else if let Some(earlier) = TABLE_1997.iter().find(|s| s.name == jpl.name) {
            if let Some(cartesian) = cartesian_from_cylindrical(
                earlier.longitude_deg,
                earlier.spin_axis_km,
                earlier.equator_km,
            ) {
                let rate = [
                    earlier.site_rate_cm_per_year[0] / 1.0e5,
                    earlier.site_rate_cm_per_year[1] / 1.0e5,
                    earlier.site_rate_cm_per_year[2] / 1.0e5,
                ];
                let moved = propagate(cartesian[0], cartesian[1], cartesian[2], rate, 8.0);
                delta = Some([
                    (moved[0] - jpl.x_km) * 1.0e6,
                    (moved[1] - jpl.y_km) * 1.0e6,
                    (moved[2] - jpl.z_km) * 1.0e6,
                ]);
            }
        }
        match delta {
            Some(d) => {
                let magnitude = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                let enu = match enu_residual(
                    [jpl.x_km, jpl.y_km, jpl.z_km],
                    [
                        jpl.x_km + d[0] / 1.0e6,
                        jpl.y_km + d[1] / 1.0e6,
                        jpl.z_km + d[2] / 1.0e6,
                    ],
                ) {
                    Some((enu, _)) => enu,
                    None => [f64::NAN; 3],
                };
                println!(
                    "station-check: {} against the JPL file of 2013 (ITRF2008, epoch 2005.0) — dx {:+8.1} dy {:+8.1} dz {:+8.1} mm, |d| {magnitude:.1} mm; east {:+.1} north {:+.1} up {:+.1}",
                    jpl.name, d[0], d[1], d[2], enu[0], enu[1], enu[2]
                );
            }
            None => println!(
                "station-check: {} carries no ESA counterpart for the JPL file",
                jpl.name
            ),
        }
    }

    for name in ["DSS-54", "DSS-55"] {
        let Some(earlier) = TABLE_1997.iter().find(|s| s.name == name) else {
            continue;
        };
        let Some(jpl) = JPL_2005.iter().find(|s| s.name == name) else {
            continue;
        };
        let Some(cartesian) = cartesian_from_cylindrical(
            earlier.longitude_deg,
            earlier.spin_axis_km,
            earlier.equator_km,
        ) else {
            continue;
        };
        let rate_esa = [
            earlier.site_rate_cm_per_year[0] / 1.0e5,
            earlier.site_rate_cm_per_year[1] / 1.0e5,
            earlier.site_rate_cm_per_year[2] / 1.0e5,
        ];
        let rate_jpl = [-7.4e-6, 1.92e-5, 1.49e-5];
        let mut best_esa = (f64::INFINITY, 0.0, [0.0; 3]);
        let mut best_jpl = (f64::INFINITY, 0.0, [0.0; 3]);
        let mut offset = -10.0;
        while offset <= 10.0 {
            let years = 8.0 + offset;
            let moved = propagate(cartesian[0], cartesian[1], cartesian[2], rate_esa, years);
            let moved_jpl = propagate(jpl.x_km, jpl.y_km, jpl.z_km, rate_jpl, offset);
            let basis = match geodetic_latitude_deg(moved[0], moved[1], moved[2]) {
                Some(latitude) => {
                    let longitude = moved[1].atan2(moved[0]).to_degrees();
                    enu_basis(latitude, longitude)
                }
                None => None,
            };
            if let Some(basis) = basis {
                let d = [
                    (moved[0] - jpl.x_km) * 1.0e6,
                    (moved[1] - jpl.y_km) * 1.0e6,
                    (moved[2] - jpl.z_km) * 1.0e6,
                ];
                let enu = project_enu(basis, d);
                let magnitude = (enu[0] * enu[0] + enu[1] * enu[1] + enu[2] * enu[2]).sqrt();
                if magnitude < best_esa.0 {
                    best_esa = (magnitude, offset, enu);
                }
                let d_jpl = [
                    (moved_jpl[0] - moved[0]) * 1.0e6,
                    (moved_jpl[1] - moved[1]) * 1.0e6,
                    (moved_jpl[2] - moved[2]) * 1.0e6,
                ];
                let enu_jpl = project_enu(basis, d_jpl);
                let magnitude_jpl =
                    (enu_jpl[0] * enu_jpl[0] + enu_jpl[1] * enu_jpl[1] + enu_jpl[2] * enu_jpl[2])
                        .sqrt();
                if magnitude_jpl < best_jpl.0 {
                    best_jpl = (magnitude_jpl, offset, enu_jpl);
                }
            }
            offset += 0.01;
        }
        println!(
            "station-check: {name} epoch scan with the ESA rate — the residual is smallest at {:+.2} years offset, {:.1} mm left (east {:+.1}, north {:+.1}, up {:+.1})",
            best_esa.1, best_esa.0, best_esa.2[0], best_esa.2[1], best_esa.2[2]
        );
        println!(
            "station-check: {name} epoch scan with the JPL rate — smallest at {:+.2} years, {:.1} mm left (east {:+.1}, north {:+.1}, up {:+.1})",
            best_jpl.1, best_jpl.0, best_jpl.2[0], best_jpl.2[1], best_jpl.2[2]
        );
    }

    for name in ["DSS-54", "DSS-55"] {
        let Some(earlier) = TABLE_1997.iter().find(|s| s.name == name) else {
            continue;
        };
        let Some(jpl) = JPL_2005.iter().find(|s| s.name == name) else {
            continue;
        };
        let Some(cartesian) = cartesian_from_cylindrical(
            earlier.longitude_deg,
            earlier.spin_axis_km,
            earlier.equator_km,
        ) else {
            continue;
        };
        let rate = [
            earlier.site_rate_cm_per_year[0] / 1.0e5,
            earlier.site_rate_cm_per_year[1] / 1.0e5,
            earlier.site_rate_cm_per_year[2] / 1.0e5,
        ];
        let esa = propagate(cartesian[0], cartesian[1], cartesian[2], rate, 8.0);
        let jpl_in_2000 = frame_2000_from_2008(
            [jpl.x_km * 1000.0, jpl.y_km * 1000.0, jpl.z_km * 1000.0],
            2005.0,
        );
        let jpl_2000_km = [
            jpl_in_2000[0] / 1000.0,
            jpl_in_2000[1] / 1000.0,
            jpl_in_2000[2] / 1000.0,
        ];
        match enu_residual(jpl_2000_km, esa) {
            Some((enu, magnitude)) => println!(
                "station-check: {name} with the official ITRF2000 frame — the JPL position transformed into ITRF2000 at 2005.0 sits {:.1} mm from the ESA value (east {:+.1}, north {:+.1}, up {:+.1}); it was 78.6 mm in ITRF2008",
                magnitude, enu[0], enu[1], enu[2]
            ),
            None => println!("station-check: {name} carries no frame-corrected residual"),
        }
    }

    match velocity_offset(&naif_pairs[0..3], NAIF_EPOCH - 2005.0) {
        Some((rate, spread)) => println!(
            "station-check: the Goldstone residual against NAIF reads as one velocity offset — ({:+.2}, {:+.2}, {:+.2}) mm/year, leaving {:.1} mm spread",
            rate[0], rate[1], rate[2], spread
        ),
        None => println!("station-check: the Goldstone residual carries no velocity offset"),
    }
    match velocity_offset(&naif_pairs[3..5], NAIF_EPOCH - 1997.0) {
        Some((rate, spread)) => println!(
            "station-check: the Madrid residual against NAIF reads as one velocity offset — ({:+.2}, {:+.2}, {:+.2}) mm/year, leaving {:.1} mm spread",
            rate[0], rate[1], rate[2], spread
        ),
        None => println!("station-check: the Madrid residual carries no velocity offset"),
    }

    let frozen = [
        NN1_2015[0] - NN1_1997[0],
        NN1_2015[1] - NN1_1997[1],
        NN1_2015[2] - NN1_1997[2],
    ];
    let frozen_mm =
        (frozen[0] * frozen[0] + frozen[1] * frozen[1] + frozen[2] * frozen[2]).sqrt() * 1.0e6;
    let rate = [
        NN1_RATE_CM_PER_YEAR[0] / 1.0e5,
        NN1_RATE_CM_PER_YEAR[1] / 1.0e5,
        NN1_RATE_CM_PER_YEAR[2] / 1.0e5,
    ];
    let carried = propagate(NN1_1997[0], NN1_1997[1], NN1_1997[2], rate, 18.0);
    let carried_mm = ((carried[0] - NN1_2015[0]).powi(2)
        + (carried[1] - NN1_2015[1]).powi(2)
        + (carried[2] - NN1_2015[2]).powi(2))
    .sqrt()
        * 1.0e6;
    println!(
        "station-check: NN1 1997.0 against 2015.0 — the two printed coordinates stand {frozen_mm:.1} mm apart, and eighteen years at the printed rate would move the 1997 point {carried_mm:.1} mm away from the 2015 one"
    );

    match cylindrical_from_itrf(NN1_2015[0], NN1_2015[1], NN1_2015[2]) {
        Some((longitude, spin_axis, equator)) => println!(
            "station-check: NN1 at 2015.0 — longitude {longitude:.9} deg, spin axis {spin_axis:.7} km, equator {equator:.7} km"
        ),
        None => println!("station-check: NN1 gives no cylinder"),
    }
}
