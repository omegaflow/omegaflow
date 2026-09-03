<!--
  title: COMPLETE EXECUTION PLAN
  class: concept
  sha256: 00e9db0d526f69226eef4fcf032e0536f630ce09a24bbf7a964cc704fe5d1ba7
-->
# COMPLETE EXECUTION PLAN

No "see above", no `...`, no `else if` without a body, no `Option` without a match.

---

## WP0 — Baseline

```bash
cargo build 2>&1 | tail -3
cargo test 2>&1 | tail -20
```

---

## WP1 — ephemeris pipeline: 7× empty map + enclose_family

**E1.** `enclose_family` signature (L457) change from:
```rust
fn enclose_family(
    fam: &Family,
    anchor: [f64; 3],
    center: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    _frustum: Option<([f64; 3], [f64; 3], [f64; 3], f64, f64)>,
) {
```
to:
```rust
fn enclose_family(
    fam: &Family,
    anchor: [f64; 3],
    center: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    _frustum: Option<([f64; 3], [f64; 3], [f64; 3], f64, f64)>,
    body_props: Option<&BodyProperties>,
) {
```

L549 change from:
```rust
            let p = smp.motion.at(t2, smp.epoch, &HashMap::new());
```
to:
```rust
            let p = smp.motion.at(t2, smp.epoch, &HashMap::new());
```
(stays unchanged — `motion.at` needs `eph`, but `enclose_family` gets `body_props` instead of `eph`. See E41 for the `motion.at` correction.)

**E2.** `sense_buffer` (L575–597) replace completely:
```rust
fn sense_buffer(
    buf: &Buffer,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    _frustum: Option<([f64; 3], [f64; 3], [f64; 3], f64, f64)>,
    eph: &HashMap<String, BodyEphemeris>,
) {
    for (body_name, fam) in &buf.bodies {
        let anchor =
            body_barycenter_position(body_name, t2, eph).unwrap_or([0.0, 0.0, 0.0]);
        let body_props = eph.get(body_name).and_then(|e| e.props.as_ref());
        enclose_family(
            fam, anchor, center, t2, pad, records, _frustum, body_props,
        );
    }
    enclose_family(
        &buf.inertial,
        [0.0, 0.0, 0.0],
        center,
        t2,
        pad,
        records,
        _frustum,
        None,
    );
}
```

**E3.** L1816 after `let center = [x0, y0, z0];` insert:
```rust
                let eph_map = archive.body_ephemerides.read().unwrap();
```
L1824 change from:
```rust
                sense_buffer(
                    &field,
                    center,
                    t0,
                    extent,
                    &mut records,
                    None,
                    &HashMap::new(),
                );
```
to:
```rust
                sense_buffer(
                    &field,
                    center,
                    t0,
                    extent,
                    &mut records,
                    None,
                    &eph_map,
                );
```
L1833 change from:
```rust
                sense_buffer(
                    &station_buf,
                    center,
                    t0,
                    extent,
                    &mut records,
                    None,
                    &HashMap::new(),
                );
```
to:
```rust
                sense_buffer(
                    &station_buf,
                    center,
                    t0,
                    extent,
                    &mut records,
                    None,
                    &eph_map,
                );
```

**E4.** `/station` endpoint: replace L1515–1527 with:
```rust
                        let eph_map = archive.body_ephemerides.read().unwrap();
                        let now = tdb_now();
                        let (p, v) = archive
                            .station
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .sample
                            .as_ref()
                            .map(|smp| {
                                let p0 = smp.motion.at(now, smp.epoch, &eph_map);
                                let p1 = smp.motion.at(now + 1.0, smp.epoch, &eph_map);
                                (
                                    p0,
                                    [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]],
                                )
                            })
                            .unwrap_or(([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]));
```

**E5.** `materialize` (L3321) change from:
```rust
                let pred = pm.at(pend.epoch, entry.prev_epoch, &HashMap::new());
```
to:
```rust
                let pred = pm.at(pend.epoch, entry.prev_epoch, eph);
```
L3336 change from:
```rust
    let (mut vmax, amax, p0f) = law_bounds(&motion, pend.epoch, resid_ema, &HashMap::new());
```
to:
```rust
    let (mut vmax, amax, p0f) = law_bounds(&motion, pend.epoch, resid_ema, eph);
```

---

## WP2 — body_name pass-through + frame_motion + guard

**E6.** `PendingPosition` (L716–736) delete and replace with:
```rust
enum PendingPosition {
    Source,
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
    },
    SurfaceFlow {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
        speed: f64,
        track: f64,
        vrate: f64,
    },
    StateVector {
        p: [f64; 3],
        v: [f64; 3],
        track: bool,
    },
}
```

**E7.** `PendingSample` (L738–742) delete and replace with:
```rust
struct PendingSample {
    epoch: f64,
    position: PendingPosition,
    fields: Vec<(String, f64)>,
    extent: Option<f64>,
    ttl: Option<f64>,
    tau: Option<f64>,
}
```

L2958 `pending.push(PendingSample {` change from:
```rust
                                pending.push(PendingSample {
                                    epoch,
                                    position,
                                    fields: ev_fields,
                                });
```
to:
```rust
                                pending.push(PendingSample {
                                    epoch,
                                    position,
                                    fields: ev_fields,
                                    extent: None,
                                    ttl: None,
                                    tau: None,
                                });
```

L3010 `pending.push(PendingSample {` change from:
```rust
                            pending.push(PendingSample {
                                epoch: now,
                                position: PendingPosition::Geodetic { lat, lon, alt },
                                fields: ev_fields,
                            });
```
to:
```rust
                            pending.push(PendingSample {
                                epoch: now,
                                position: PendingPosition::Surface {
                                    body_name: body_name.clone(),
                                    lat,
                                    lon,
                                    alt,
                                },
                                fields: ev_fields,
                                extent: None,
                                ttl: None,
                                tau: None,
                            });
```

L3036 `pending.push(PendingSample {` change from:
```rust
                            pending.push(PendingSample {
                                epoch: now,
                                position: PendingPosition::Geodetic { lat, lon, alt },
                                fields: ev_fields,
                            });
```
to:
```rust
                            pending.push(PendingSample {
                                epoch: now,
                                position: PendingPosition::Surface {
                                    body_name: body_name.clone(),
                                    lat,
                                    lon,
                                    alt,
                                },
                                fields: ev_fields,
                                extent: None,
                                ttl: None,
                                tau: None,
                            });
```

L3223 `pending.push(PendingSample {` change from:
```rust
                                                pending.push(PendingSample {
                                                    epoch: now,
                                                    position: PendingPosition::Geodetic {
                                                        lat: ela,
                                                        lon: elo,
                                                        alt: -ed * 1000.0,
                                                    },
                                                    fields: ev_fields,
                                                });
```
to:
```rust
                                                pending.push(PendingSample {
                                                    epoch: now,
                                                    position: PendingPosition::Surface {
                                                        body_name: frame_body_name(
                                                            &src.frame,
                                                        ),
                                                        lat: ela,
                                                        lon: elo,
                                                        alt: -ed * 1000.0,
                                                    },
                                                    fields: ev_fields,
                                                    extent: None,
                                                    ttl: None,
                                                    tau: None,
                                                });
```

**E8.** `flow_motion` (L599–647) delete and replace with:
```rust
fn surface_motion(
    body_name: &str,
    lat: f64,
    lon: f64,
    alt: f64,
    speed: f64,
    track: f64,
    vrate: f64,
    t: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Motion> {
    let p0 = body_fixed_to_icrs(body_name, lat, lon, alt, t, eph)?;
    let p1 = body_fixed_to_icrs(body_name, lat, lon, alt, t + 1.0, eph)?;
    let v_frame = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let latr = lat.to_radians();
    let lonr = lon.to_radians();
    let trk = track.to_radians();
    let v_e = speed * trk.sin();
    let v_n = speed * trk.cos();
    let v_body = [
        -v_e * lonr.sin() - v_n * latr.sin() * lonr.cos()
            + vrate * latr.cos() * lonr.cos(),
        v_e * lonr.cos() - v_n * latr.sin() * lonr.sin()
            + vrate * latr.cos() * lonr.sin(),
        v_n * latr.cos() + vrate * latr.sin(),
    ];
    let r = eph.get(body_name)?.props.as_ref()?.radius_m;
    let cl = latr.cos();
    let dt = 0.01;
    let pp = body_fixed_to_icrs(
        body_name,
        lat + v_body[1] * dt / r,
        lon + v_body[0] * dt / (r * cl),
        alt + v_body[2] * dt,
        t,
        eph,
    )?;
    let v_rot = [
        (pp[0] - p0[0]) / dt,
        (pp[1] - p0[1]) / dt,
        (pp[2] - p0[2]) / dt,
    ];
    Some(Motion::Linear {
        p: p0,
        v: [
            v_frame[0] + v_rot[0],
            v_frame[1] + v_rot[1],
            v_frame[2] + v_rot[2],
        ],
    })
}
```

**E9.** After L302 (after `impl Motion { ... }`) insert:
```rust
fn frame_motion(
    frame: &Frame,
    spd: Option<f64>,
    hdg: Option<f64>,
    t: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Motion> {
    match frame {
        Frame::Surface {
            body_name,
            lat,
            lon,
            alt,
        } => match (spd, hdg) {
            (Some(s), Some(h)) if s > 0.0 => surface_motion(
                body_name, *lat, *lon, *alt, s, h, 0.0, t, eph,
            ),
            _ => Some(Motion::Surface {
                body_name: body_name.clone(),
                lat: *lat,
                lon: *lon,
                alt: *alt,
            }),
        },
        Frame::Barycenter {
            body_name, scale, ..
        } => Some(Motion::Barycenter {
            body_name: body_name.clone(),
            scale: *scale,
        }),
    }
}
```

**E10.** Nach L1271 `enum Frame { ... }` insert:
```rust
fn frame_body_name(frame: &Frame) -> String {
    match frame {
        Frame::Surface { body_name, .. } | Frame::Barycenter { body_name, .. } => {
            body_name.clone()
        }
    }
}
```

**E11.** `materialize` Motion-Match (L3277–3305) delete and replace with:
```rust
        let motion = match &pend.position {
            PendingPosition::StateVector { p, v, .. } => {
                Motion::Linear { p: *p, v: *v }
            }
            PendingPosition::Surface {
                body_name,
                lat,
                lon,
                alt,
            } => Motion::Surface {
                body_name: body_name.clone(),
                lat: *lat,
                lon: *lon,
                alt: *alt,
            },
            PendingPosition::SurfaceFlow {
                body_name,
                lat,
                lon,
                alt,
                speed,
                track,
                vrate,
            } => {
                match frame_motion(
                    &Frame::Surface {
                        body_name: body_name.clone(),
                        lat: *lat,
                        lon: *lon,
                        alt: *alt,
                    },
                    Some(*speed),
                    Some(*track),
                    pend.epoch,
                    eph,
                ) {
                    Some(m) => m,
                    None => return vec![],
                }
            }
            PendingPosition::Source => {
                match frame_motion(&src.frame, None, None, pend.epoch, eph) {
                    Some(m) => m,
                    None => return vec![],
                }
            }
        };
```

**E12.** After the motion match, before L3306 `let abs =` insert:
```rust
        let eph_ok = match &motion {
            Motion::Surface { body_name, .. } => eph
                .get(body_name)
                .map(|e| e.props.is_some())
                .unwrap_or(false),
            Motion::Barycenter { body_name, .. } => eph
                .get(body_name)
                .map(|e| !e.granules.is_empty())
                .unwrap_or(false),
            Motion::Linear { .. } => true,
        };
        if !eph_ok {
            return vec![];
        }
```

**E13.** `materialize` L3360 tau-Berechnung delete and replace with:
```rust
            let tau = pend
                .tau
                .or(src.tau)
                .or_else(|| {
                    src.tau_key.as_ref().and_then(|k| {
                        clean_fields
                            .iter()
                            .find(|(n, _)| n == k)
                            .map(|(_, v)| *v / v_or_d)
                    })
                })
                .unwrap_or(tau_default);
```

L3374–3380 extent-Berechnung delete and replace with:
```rust
            let effective_ttl = src.reach_ttl.unwrap_or(src.ttl) as f64;
            let reach_time = effective_ttl + tau;
            let extent = pend.extent.unwrap_or_else(|| {
                if is_diff {
                    (2.0 * v_or_d * reach_time).sqrt()
                } else {
                    v_or_d * reach_time
                }
            });
```

L3387 change from:
```rust
                ttl: src.ttl.max(1) as f64,
```
to:
```rust
                ttl: pend.ttl.unwrap_or(src.ttl.max(1) as f64),
```

**E14.** L2932–2933 `PendingPosition::GeodeticFlow`-Konstruktion delete and replace with:
```rust
                                let position =
                                    if let (Some(sp), Some(tr)) = (speed, track) {
                                        PendingPosition::SurfaceFlow {
                                            body_name: frame_body_name(
                                                &src.frame,
                                            ),
                                            lat: la,
                                            lon: lon_val,
                                            alt: al,
                                            speed: sp,
                                            track: tr,
                                            vrate: vrate.unwrap_or(0.0),
                                        }
                                    } else {
                                        PendingPosition::Surface {
                                            body_name: frame_body_name(
                                                &src.frame,
                                            ),
                                            lat: la,
                                            lon: lon_val,
                                            alt: al,
                                        }
                                    };
```

**E15.** L2969 `if let Frame::Surface { lat, lon, alt, .. }` delete and replace with:
```rust
                if let Frame::Surface {
                    body_name,
                    lat,
                    lon,
                    alt,
                } = &src.frame
                {
```

L3013 change from:
```rust
                                    position: PendingPosition::Geodetic {
                                        lat,
                                        lon,
                                        alt,
                                    },
```
to:
```rust
                                    position: PendingPosition::Surface {
                                        body_name: body_name.clone(),
                                        lat,
                                        lon,
                                        alt,
                                    },
```

L3039 change from:
```rust
                                position: PendingPosition::Geodetic {
                                    lat,
                                    lon,
                                    alt,
                                },
```
to:
```rust
                                position: PendingPosition::Surface {
                                    body_name: body_name.clone(),
                                    lat,
                                    lon,
                                    alt,
                                },
```

**E16.** L3216–3223 `position: PendingPosition::Geodetic`-Block delete and replace with:
```rust
                                                    position: PendingPosition::Surface {
                                                        body_name: frame_body_name(
                                                            &src.frame,
                                                        ),
                                                        lat: ela,
                                                        lon: elo,
                                                        alt: -ed * 1000.0,
                                                    },
```

---

## WP3 — icrs_to_body_surface + render_url + constant deleted

**E17.** L12 delete:
```rust
const EARTH_RADIUS: f64 = 6378137.0;
```

**E18.** L227–253 `icrs_to_geodetic` delete and replace with:
```rust
fn icrs_to_body_surface(
    x: f64,
    y: f64,
    z: f64,
    tdb_secs: f64,
    body_name: &str,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<(f64, f64)> {
    let e = eph.get(body_name)?;
    let props = e.props.as_ref()?;
    let b = body_barycenter_position(body_name, tdb_secs, eph)?;
    let jd = tdb_secs / 86400.0 + J2000_EPOCH;
    let tc = (jd - J2000_EPOCH) / 36525.0;
    let a =
        (props.α0_deg + props.dα0_dt_deg_per_century * tc).to_radians();
    let d =
        (props.δ0_deg + props.dδ0_dt_deg_per_century * tc).to_radians();
    let w = ((props.w0_deg
        + props.dw_dt_deg_per_day * (jd - J2000_EPOCH))
        - (props.α0_deg + props.dα0_dt_deg_per_century * tc))
        .to_radians();
    let (sa, ca) = a.sin_cos();
    let (sd, cd) = d.sin_cos();
    let xb = x - b[0];
    let yb = y - b[1];
    let zb = z - b[2];
    let xt = cd * ca * xb + cd * sa * yb + sd * zb;
    let yt = -sa * xb + ca * yb;
    let zt = -sd * ca * xb - sd * sa * yb + cd * zb;
    let xr = xt * w.cos() - yt * w.sin();
    let yr = xt * w.sin() + yt * w.cos();
    Some((
        zt.atan2((xr * xr + yr * yr).sqrt()).to_degrees(),
        yr.atan2(xr).to_degrees(),
    ))
}
```

**E19.** `render_url` (L2382–2533) signature and body replace completely with:
```rust
fn render_url(
    template: &str,
    x: f64,
    y: f64,
    z: f64,
    tdb_secs: f64,
    extent: f64,
    body_name: &str,
    eph: &HashMap<String, BodyEphemeris>,
) -> String {
    let unix = tdb_secs + UNIX_J2000_OFFSET;
    let secs = unix as u64;
    let days = secs / 86400;
    let (ty, tm, td) = days_to_ymd(days);
    let yday = {
        let cum = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let leap = (ty % 4 == 0 && ty % 100 != 0) || ty % 400 == 0;
        let base = if tm > 0 {
            cum[(tm - 1) as usize]
        } else {
            0
        };
        base + td + if leap && tm > 2 { 1 } else { 0 }
    };
    let year2 = ty % 100;
    let today = format!("{}-{:02}-{:02}", ty, tm, td);
    let (yy, ym, yd) = days_to_ymd(days - 1);
    let yesterday = format!("{}-{:02}-{:02}", yy, ym, yd);
    let (tmy, tmm, tmd) = days_to_ymd(days + 1);
    let tomorrow = format!("{}-{:02}-{:02}", tmy, tmm, tmd);
    let today_yyyymmdd = format!("{}_{:02}_{:02}", ty, tm, td);
    let today_nodashes = format!("{}{:02}{:02}", ty, tm, td);
    let yesterday_nodashes = format!("{}{:02}{:02}", yy, ym, yd);
    let tomorrow_nodashes = format!("{}{:02}{:02}", tmy, tmm, tmd);
    let hour_ago = {
        let dt = secs.saturating_sub(3600);
        let (h_y, h_m, h_d) = days_to_ymd(dt / 86400);
        let h_h = (dt % 86400) / 3600;
        let h_min = (dt % 3600) / 60;
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:00",
            h_y, h_m, h_d, h_h, h_min
        )
    };
    let now_iso = {
        let n_h = (secs % 86400) / 3600;
        let n_min = (secs % 3600) / 60;
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:00",
            ty, tm, td, n_h, n_min
        )
    };
    let now_minus_1 = {
        let dt = secs.saturating_sub(60);
        let (n1_y, n1_m, n1_d) = days_to_ymd(dt / 86400);
        let n1_h = (dt % 86400) / 3600;
        let n1_min = (dt % 3600) / 60;
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:00",
            n1_y, n1_m, n1_d, n1_h, n1_min
        )
    };
    let now_minus_2 = {
        let dt = secs.saturating_sub(120);
        let (n2_y, n2_m, n2_d) = days_to_ymd(dt / 86400);
        let n2_h = (dt % 86400) / 3600;
        let n2_min = (dt % 3600) / 60;
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:00",
            n2_y, n2_m, n2_d, n2_h, n2_min
        )
    };
    let week_ago = {
        let dt = secs.saturating_sub(604800);
        let (w_y, w_m, w_d) = days_to_ymd(dt / 86400);
        format!("{}-{:02}-{:02}", w_y, w_m, w_d)
    };
    let week_ago_nodashes = {
        let dt = secs.saturating_sub(604800);
        let (w_y, w_m, w_d) = days_to_ymd(dt / 86400);
        format!("{}{:02}{:02}", w_y, w_m, w_d)
    };
    let q_hour = (secs % 86400) / 3600;
    let q_minute = (secs % 3600) / 60;
    let unix_now = secs.to_string();
    let unix_now_plus_3600 = (secs + 3600).to_string();

    let res_usize = 6usize;
    let mut url = template
        .replace("{x}", &format!("{}", x))
        .replace("{y}", &format!("{}", y))
        .replace("{z}", &format!("{}", z))
        .replace("{today}", &today)
        .replace("{yesterday}", &yesterday)
        .replace("{tomorrow}", &tomorrow)
        .replace("{today_yyyymmdd}", &today_yyyymmdd)
        .replace("{today_ymd}", &today_yyyymmdd)
        .replace("{today_nodashes}", &today_nodashes)
        .replace("{yesterday_nodashes}", &yesterday_nodashes)
        .replace("{tomorrow_nodashes}", &tomorrow_nodashes)
        .replace("{t_start}", &yesterday)
        .replace("{t_end}", &today)
        .replace("{now}", &now_iso)
        .replace("{now_minus_1}", &now_minus_1)
        .replace("{now_minus_2}", &now_minus_2)
        .replace("{week_ago}", &week_ago)
        .replace("{week_ago_nodashes}", &week_ago_nodashes)
        .replace(
            "{today_plus_365}",
            &format!("{}-{:02}-{:02}", ty + 1, tm, td),
        )
        .replace("{hour_ago}", &hour_ago)
        .replace("{year}", &ty.to_string())
        .replace("{year2}", &format!("{:02}", year2))
        .replace("{month}", &tm.to_string())
        .replace("{day}", &td.to_string())
        .replace("{yday}", &format!("{:03}", yday))
        .replace("{hour}", &format!("{:02}", q_hour))
        .replace("{minute}", &format!("{:02}", q_minute))
        .replace("{unix_now}", &unix_now)
        .replace("{unix_now_plus_3600}", &unix_now_plus_3600);

    if let Some((lat, lon)) =
        icrs_to_body_surface(x, y, z, tdb_secs, body_name, eph)
    {
        if let Some(m_per_deg) = eph
            .get(body_name)
            .and_then(|e| e.props.as_ref())
            .map(|p| p.radius_m * std::f64::consts::PI / 180.0)
        {
            let half_deg = extent / m_per_deg;
            let lat_str = format!("{:.6}", lat);
            let lon_str = format!("{:.6}", lon);
            let lat_min_str =
                format!("{:.*}", res_usize, lat - half_deg);
            let lat_max_str =
                format!("{:.*}", res_usize, lat + half_deg);
            let lon_min_str =
                format!("{:.*}", res_usize, lon - half_deg);
            let lon_max_str =
                format!("{:.*}", res_usize, lon + half_deg);
            let (grid_str, grid_lat_str, grid_lon_str) = {
                let step = half_deg * 0.5;
                let mut g = Vec::with_capacity(16);
                let mut gla = Vec::with_capacity(4);
                let mut glo = Vec::with_capacity(4);
                for i in 0..4 {
                    for j in 0..4 {
                        g.push(format!(
                            "{:.*},{:.*}",
                            res_usize,
                            lat + (i as f64 - 1.5) * step,
                            res_usize,
                            lon + (j as f64 - 1.5) * step
                        ));
                    }
                    gla.push(format!(
                        "{:.*}",
                        res_usize,
                        lat + (i as f64 - 1.5) * step
                    ));
                    glo.push(format!(
                        "{:.*}",
                        res_usize,
                        lon + (i as f64 - 1.5) * step
                    ));
                }
                (g.join("|"), gla.join(","), glo.join(","))
            };
            url = url
                .replace("{grid_lat}", &grid_lat_str)
                .replace("{grid_lon}", &grid_lon_str)
                .replace("{grid}", &grid_str)
                .replace("{lat}", &lat_str)
                .replace("{lon}", &lon_str)
                .replace("{lat_min}", &lat_min_str)
                .replace("{lat_max}", &lat_max_str)
                .replace("{lon_min}", &lon_min_str)
                .replace("{lon_max}", &lon_max_str)
                .replace("{lat_int}", &format!("{}", lat as i32))
                .replace("{lon_int}", &format!("{}", lon as i32));
        }
    }

    let nasa_key = std::env::var("NASA_KEY").unwrap_or_default();
    url.replace("{nasa_key}", &nasa_key)
}
```

**E20.** `render_url` callers change:

L2545 change from:
```rust
    let mut url = render_url(&src.url, x, y, z, tdb, r, eph);
```
to:
```rust
    let mut url = render_url(
        &src.url,
        x,
        y,
        z,
        tdb,
        r,
        &frame_body_name(&src.frame),
        eph,
    );
```

L2649 change from:
```rust
    let mut body = render_url(tmpl, x, y, z, tdb, r, eph);
```
to:
```rust
    let mut body = render_url(
        tmpl,
        x,
        y,
        z,
        tdb,
        r,
        &frame_body_name(&src.frame),
        eph,
    );
```

L3412 change from:
```rust
        .map(|(k, v)| {
            (k.clone(), render_url(v, x, y, z, now, extent, eph))
        })
```
to:
```rust
        .map(|(k, v)| {
            (
                k.clone(),
                render_url(
                    v,
                    x,
                    y,
                    z,
                    now,
                    extent,
                    &frame_body_name(&src.frame),
                    eph,
                ),
            )
        })
```

**E21.** L2622–2631 `icrs_to_geodetic`-Block in `render_source_url` delete and replace with:
```rust
                if let Some((lat, lon)) = icrs_to_body_surface(
                    x,
                    y,
                    z,
                    tdb,
                    &frame_body_name(&src.frame),
                    eph,
                ) {
                    let mut best = 0usize;
                    let mut best_d = f64::MAX;
                    for (i, st) in stations.iter().enumerate() {
                        let d2 = (st.lat - lat).powi(2)
                            + (st.lon - lon).powi(2);
                        if d2 < best_d {
                            best_d = d2;
                            best = i;
                        }
                    }
                    url = url
                        .replace("{nearest_station}", &stations[best].id);
                }
```

---

## WP4 — region_quantize + warm_cache

**E22.** L335 `region_quantize` delete and replace with:
```rust
fn region_quantize(deg: f64, extent: f64, m_per_deg: f64) -> i32 {
    (deg * m_per_deg / extent).round() as i32
}
```

**E23.** `warm_cache` Surface-Zweig:

L3561 Pattern change from:
```rust
                    Frame::Surface {
                        lat, lon, alt, ..
                    } => {
```
to:
```rust
                    Frame::Surface {
                        body_name,
                        lat,
                        lon,
                        alt,
                        ..
                    } => {
```

L3574 `icrs_to_geodetic`-Zeile delete and replace with:
```rust
                                let (tlat, tlon) =
                                    match icrs_to_body_surface(
                                        px, py, pz, now, body_name,
                                        &eph_map,
                                    ) {
                                        Some(v) => v,
                                        None => continue,
                                    };
```

L3575–3576 `let origin`-Block delete and replace with:
```rust
                                let m_per_deg = match eph_map
                                    .get(body_name)
                                    .and_then(|e| e.props.as_ref())
                                {
                                    Some(p) => {
                                        p.radius_m
                                            * std::f64::consts::PI
                                            / 180.0
                                    }
                                    None => continue,
                                };
                                let origin = (
                                    i as u32,
                                    region_quantize(
                                        tlat, r, m_per_deg,
                                    ),
                                    region_quantize(
                                        tlon, r, m_per_deg,
                                    ),
                                );
```

L3628–3629 `body_fixed_to_icrs("earth"`-Block delete and replace with:
```rust
                        let pa = match body_fixed_to_icrs(
                            body_name,
                            *lat,
                            *lon,
                            *alt,
                            now,
                            &eph_map,
                        ) {
                            Some(pa) => pa,
                            None => continue,
                        };
                        let pos = (pa[0], pa[1], pa[2]);
```

---

## WP5 — load_sources: 4 Earth fallbacks eliminated

**E24.** L1948–1978 `let frame = if let (Some(lat), ...`-Block delete and replace with:
```rust
                    let frame =
                        if let (Some(lat), Some(lon)) =
                            (cur_lat, cur_lon)
                        {
                            match cur_body
                                .clone()
                                .filter(|b| !b.is_empty())
                            {
                                Some(body_name) => {
                                    Some(Frame::Surface {
                                        body_name,
                                        lat,
                                        lon,
                                        alt: cur_alt,
                                    })
                                }
                                None => {
                                    eprintln!(
                                        "source refused (surface frame without body): {}",
                                        cur_url
                                    );
                                    None
                                }
                            }
                        } else if let Some(scale) = cur_scale {
                            match cur_body
                                .clone()
                                .filter(|b| !b.is_empty())
                            {
                                Some(body_name) => {
                                    Some(Frame::Barycenter {
                                        body_name,
                                        scale,
                                    })
                                }
                                None => {
                                    eprintln!(
                                        "source refused (barycenter frame without body): {}",
                                        cur_url
                                    );
                                    None
                                }
                            }
                        } else if has_data_position {
                            match cur_body
                                .clone()
                                .filter(|b| !b.is_empty())
                            {
                                Some(body_name) => {
                                    Some(Frame::Surface {
                                        body_name,
                                        lat: 0.0,
                                        lon: 0.0,
                                        alt: 0.0,
                                    })
                                }
                                None => {
                                    eprintln!(
                                        "source refused (data-carried position without body): {}",
                                        cur_url
                                    );
                                    None
                                }
                            }
                        } else if cur_url.contains("{lat}")
                            || cur_url.contains("{lon}")
                            || cur_url.contains("{x}")
                            || cur_url.contains("{y}")
                            || cur_url.contains("{z}")
                            || cur_url.contains("{grid")
                        {
                            match cur_body
                                .clone()
                                .filter(|b| !b.is_empty())
                            {
                                Some(body_name) => {
                                    Some(Frame::Surface {
                                        body_name,
                                        lat: 0.0,
                                        lon: 0.0,
                                        alt: 0.0,
                                    })
                                }
                                None => {
                                    eprintln!(
                                        "source refused (url template frame without body): {}",
                                        cur_url
                                    );
                                    None
                                }
                            }
                        } else {
                            eprintln!(
                                "source refused (no reference frame): {}",
                                cur_url
                            );
                            None
                        };
```

---

## WP6 — station: source through materialize

**E25.** L1695–1769 delete and replace with:
```rust
            let (mut st_lat, mut st_lon, mut st_alt, mut st_acc, mut st_spd, mut st_hdg) =
                (None, None, None, None, None, None);
            let mut st_body: Option<String> = None;
            for (name, value) in &source_oscillators {
                if let Some(body) = name.strip_prefix("body:") {
                    st_body = Some(body.to_string());
                    continue;
                }
                match name.as_str() {
                    "lat" => st_lat = Some(*value),
                    "lon" => st_lon = Some(*value),
                    "alt" => st_alt = Some(*value),
                    "acc" => st_acc = Some(*value),
                    "spd" => st_spd = Some(*value),
                    "hdg" => st_hdg = Some(*value),
                    _ => {}
                }
                station_fields.push((name.clone(), *value));
            }
            let station_buf = {
                let mut station = archive
                    .station
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                if let (Some(lat), Some(lon), Some(acc)) =
                    (st_lat, st_lon, st_acc)
                {
                    if acc > 0.0 {
                        let dt = if station.last_seen > 0.0 {
                            (now - station.last_seen).abs()
                        } else {
                            0.0
                        };
                        if dt > 0.0 {
                            if station.ema_interval <= 0.0 {
                                station.ema_interval = dt;
                            } else {
                                let tau = station.ema_interval;
                                station.ema_interval +=
                                    (dt - tau)
                                        * (1.0 - (-dt / tau).exp());
                            }
                        }
                        station.last_seen = now;
                        let eph_map = archive
                            .body_ephemerides
                            .read()
                            .unwrap();
                        let maybe_samples: Option<Vec<Sample>> =
                            (|| {
                                let body = st_body.as_deref()?;
                                let props = eph_map
                                    .get(body)?
                                    .props
                                    .as_ref()?;
                                let m_per_deg = props.radius_m
                                    * std::f64::consts::PI
                                    / 180.0;
                                let acc_extent = 2f64.powf(
                                    (acc / m_per_deg).log2().ceil(),
                                );
                                let alt = st_alt.unwrap_or(0.0);
                                let station_src = SourceConfig {
                                    name: String::new(),
                                    ttl: 0,
                                    url: String::new(),
                                    frame: Frame::Surface {
                                        body_name: body
                                            .to_string(),
                                        lat,
                                        lon,
                                        alt,
                                    },
                                    force: "em".to_string(),
                                    tau: None,
                                    tau_key: None,
                                    format: String::new(),
                                    extracts: vec![],
                                    headers: vec![],
                                    pos_fields: None,
                                    target: None,
                                    catalog: None,
                                    max_freq: None,
                                    min_freq: None,
                                    body: None,
                                    stations_url: None,
                                    stations_path: String::new(),
                                    stations_lat: String::new(),
                                    stations_lon: String::new(),
                                    stations_id: String::new(),
                                    flux_from_mag: None,
                                    abs_mag_from: None,
                                    reach_ttl: None,
                                    catalog_epoch: None,
                                    repeat_ra_bins: 0,
                                };
                                let position =
                                    match (st_spd, st_hdg) {
                                        (Some(spd), Some(hdg))
                                            if spd > 0.0 =>
                                        {
                                            PendingPosition::SurfaceFlow {
                                                body_name: body
                                                    .to_string(),
                                                lat,
                                                lon,
                                                alt,
                                                speed: spd,
                                                track: hdg,
                                                vrate: 0.0,
                                            }
                                        }
                                        _ => {
                                            PendingPosition::Surface {
                                                body_name: body
                                                    .to_string(),
                                                lat,
                                                lon,
                                                alt,
                                            }
                                        }
                                    };
                                let pend = PendingSample {
                                    epoch: now,
                                    position,
                                    fields: station_fields,
                                    extent: Some(acc_extent),
                                    ttl: Some(
                                        Φ * Φ
                                            * station
                                                .ema_interval,
                                    ),
                                    tau: Some(
                                        station.ema_interval,
                                    ),
                                };
                                let samples = materialize(
                                    &station_src,
                                    (u32::MAX, 0, 0),
                                    (u32::MAX, 0, 0),
                                    &pend,
                                    &mut HashMap::new(),
                                    &eph_map,
                                );
                                if samples.is_empty() {
                                    None
                                } else {
                                    Some(samples)
                                }
                            })();
                        match maybe_samples {
                            Some(samples) => {
                                station.sample =
                                    samples.first().cloned();
                                station.buffer = Arc::new(
                                    build_buffer(
                                        samples,
                                        station.ema_interval,
                                    ),
                                );
                            }
                            None => {
                                station.sample = None;
                                station.buffer = Arc::new(
                                    build_buffer(
                                        Vec::new(),
                                        1.0,
                                    ),
                                );
                            }
                        }
                    }
                }
                Arc::clone(&station.buffer)
            };
```

**E26.** `static/index.html` L201–205, in the `if (!indexMap.has('lat'))` block, after `registerOscillator('alt', c.altitude || 0);` insert:
```js
                        registerOscillator('body:earth', 1);
```

---

## WP7 — fix the tests

**E27.** L4076 change from:
```rust
        let url = render_source_url(
            &src, 0.0, 0.0, 0.0, 0.0, 1000.0, None,
        );
```
to:
```rust
        let url = render_source_url(
            &src,
            0.0,
            0.0,
            0.0,
            0.0,
            1000.0,
            None,
            &std::collections::HashMap::new(),
        );
```

**E28.** L4113 change from:
```rust
        let body = render_source_body(
            &src, 0.0, 0.0, 0.0, 0.0, 100000.0,
        );
```
to:
```rust
        let body = render_source_body(
            &src,
            0.0,
            0.0,
            0.0,
            0.0,
            100000.0,
            &std::collections::HashMap::new(),
        );
```

**E29.** `test_post_body_rendering` (L4083–4119):

L4089 change from:
```rust
            frame: super::Frame::Surface {
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
                ..
            },
```
to:
```rust
            frame: super::Frame::Surface {
                body_name: "testbody".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
                ..
            },
```

Vor L4113 `let body = render_source_body(...)` insert:
```rust
        let now = super::tdb_now();
        let granule = super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH + now / 86400.0 - 0.5,
            dt_jd: 1.0,
            cx: [0.0; 18],
            cy: [0.0; 18],
            cz: [0.0; 18],
        };
        let eph_map = std::collections::HashMap::from([(
            "testbody".to_string(),
            super::BodyEphemeris {
                granules: vec![granule],
                props: Some(super::BodyProperties {
                    α0_deg: 0.0,
                    dα0_dt_deg_per_century: 0.0,
                    δ0_deg: 90.0,
                    dδ0_dt_deg_per_century: 0.0,
                    w0_deg: 0.0,
                    dw_dt_deg_per_day: 360.0,
                    radius_m: 1.0e7,
                    flattening: 0.0,
                    v_sound: None,
                    v_seismic_p: None,
                    v_seismic_s: None,
                    alpha_thermal: None,
                    d_diffusion: None,
                    v_advective: None,
                }),
            },
        )]);
```

L4113 replace with:
```rust
        let body = render_source_body(
            &src, 0.0, 0.0, 0.0, now, 100000.0, &eph_map,
        );
```

**E30.** L4186 change from:
```rust
            super::PendingPosition::Geodetic {
                lat, lon, alt,
            } => {
```
to:
```rust
            super::PendingPosition::Surface {
                body_name,
                lat,
                lon,
                alt,
            } => {
```
and after `assert!((lat - ...)` insert:
```rust
                assert_eq!(body_name, "");
```

L4191 change from:
```rust
                _ => panic!("expected Geodetic position"),
```
to:
```rust
                _ => panic!("expected Surface position"),
```

L4196 change from:
```rust
            super::PendingPosition::Geodetic { alt, .. } => {
```
to:
```rust
            super::PendingPosition::Surface { alt, .. } => {
```

L4199 change from:
```rust
                _ => panic!("expected Geodetic position"),
```
to:
```rust
                _ => panic!("expected Surface position"),
```

---

## WP8 — delete the media constants

**E31.** L20–L24 delete:
```rust
const V_SOUND_288: f64 = 343.0;
```
```rust
const V_P_GRANITE: f64 = 5950.0;
```
```rust
const V_S_GRANITE: f64 = 3630.0;
```
```rust
const D_AIR: f64 = 2.0e-5;
```
```rust
const ALPHA_AIR: f64 = 2.18e-5;
```

Remove all five lines from the file.

---

## WP9 — BodyProperties extended + force constants removed

**E32.** `BodyProperties` (L50–59) delete and replace with:
```rust
struct BodyProperties {
    α0_deg: f64,
    dα0_dt_deg_per_century: f64,
    δ0_deg: f64,
    dδ0_dt_deg_per_century: f64,
    w0_deg: f64,
    dw_dt_deg_per_day: f64,
    radius_m: f64,
    flattening: f64,
    v_sound: Option<f64>,
    v_seismic_p: Option<f64>,
    v_seismic_s: Option<f64>,
    alpha_thermal: Option<f64>,
    d_diffusion: Option<f64>,
    v_advective: Option<f64>,
}
```

L114–123 `stype==1`-Konstruktion delete and replace with:
```rust
            props = Some(BodyProperties {
                α0_deg: f(0),
                dα0_dt_deg_per_century: f(1),
                δ0_deg: f(2),
                dδ0_dt_deg_per_century: f(3),
                w0_deg: f(4),
                dw_dt_deg_per_day: f(5),
                radius_m: f(6),
                flattening: f(7),
                v_sound: None,
                v_seismic_p: None,
                v_seismic_s: None,
                alpha_thermal: None,
                d_diffusion: None,
                v_advective: None,
            });
```

**E33.** `parse_ephemeris_binary` stype==2-Parser insert:

Before the `for` loop, after `let mut props = None;`:
```rust
    let mut media = [0.0f64; 6];
    let mut has_media = false;
```

Inside the `for` loop, after the stype==1 block (`pos += 64; continue;` at L125), before the `if stype != 0 {`:
```rust
        if stype == 2 {
            if pos + 48 > data.len() {
                break;
            }
            let f = |i: usize| -> f64 {
                f64::from_le_bytes(
                    data[pos + i * 8..pos + (i + 1) * 8]
                        .try_into()
                        .unwrap_or([0; 8]),
                )
            };
            media = [
                f(0), f(1), f(2), f(3), f(4), f(5),
            ];
            has_media = true;
            pos += 48;
            continue;
        }
```

After the `for` loop, before `if granules.is_empty()`:
```rust
    if has_media {
        if let Some(ref mut p) = props {
            if media[0] > 0.0 {
                p.v_sound = Some(media[0]);
            }
            if media[1] > 0.0 {
                p.v_seismic_p = Some(media[1]);
            }
            if media[2] > 0.0 {
                p.v_seismic_s = Some(media[2]);
            }
            if media[3] > 0.0 {
                p.alpha_thermal = Some(media[3]);
            }
            if media[4] > 0.0 {
                p.d_diffusion = Some(media[4]);
            }
            if media[5] > 0.0 {
                p.v_advective = Some(media[5]);
            }
        }
    }
```

**E34.** `force_constants` (L1228–1240, estimated — the function with `match force { "em" => ...`) delete.

**E35.** `force_constants_by_id` (L1242–1254, estimated) delete.

**E36.** New `force_id_of` function after L1254 insert:
```rust
fn force_id_of(force: &str) -> Option<u8> {
    match force {
        "em" => Some(0),
        "gravity" => Some(1),
        "acoustic" => Some(2),
        "seismic-body" => Some(3),
        "seismic-surface" => Some(4),
        "thermal" => Some(5),
        "diffusion" => Some(6),
        "advective" => Some(7),
        _ => None,
    }
}
```

**E37.** `force_type_of` (L1256–1258) delete and replace with:
```rust
fn force_type_val(force: &str) -> f64 {
    force_id_of(force).map(|id| id as f64).unwrap_or(0.0)
}
```

**E38.** `load_sources` L1927 change from:
```rust
                } else if force_constants(&cur_force).is_none() {
```
to:
```rust
                } else if force_id_of(&cur_force).is_none() {
```

**E39.** `materialize` L3343–3347 change from:
```rust
    let forces: Vec<(f64, f64, bool, u8)> = src
        .force
        .split_whitespace()
        .filter_map(|f| force_constants(f))
        .collect();
```
to:
```rust
    let body_props = motion
        .anchor_body()
        .and_then(|b| eph.get(b))
        .and_then(|e| e.props.as_ref());
    let forces: Vec<(f64, f64, bool, u8)> = src
        .force
        .split_whitespace()
        .filter_map(|f| {
            let id = force_id_of(f)?;
            let (v, tau_def, is_diff) = match id {
                0 | 1 => (C_LIGHT, 1.0, false),
                2 => (
                    body_props.and_then(|p| p.v_sound)?,
                    1.0,
                    false,
                ),
                3 => (
                    body_props.and_then(|p| p.v_seismic_p)?,
                    1.0,
                    false,
                ),
                4 => (
                    body_props.and_then(|p| p.v_seismic_s)?,
                    1.0,
                    false,
                ),
                5 => (
                    body_props.and_then(|p| p.alpha_thermal)?,
                    1.0,
                    true,
                ),
                6 => (
                    body_props.and_then(|p| p.d_diffusion)?,
                    1.0,
                    true,
                ),
                7 => (
                    body_props.and_then(|p| p.v_advective)?,
                    0.1,
                    false,
                ),
                _ => return None,
            };
            Some((v, tau_def, is_diff, id))
        })
        .collect();
```

**E40.** `enclose_family` L524–533 (der `force_constants_by_id`-Block) delete and replace with:
```rust
            let (v_or_d, is_diff) = match body_props {
                Some(props) => match smp.force_type as u8 {
                    0 | 1 => (C_LIGHT, false),
                    2 => match props.v_sound {
                        Some(v) => (v, false),
                        None => continue,
                    },
                    3 => match props.v_seismic_p {
                        Some(v) => (v, false),
                        None => continue,
                    },
                    4 => match props.v_seismic_s {
                        Some(v) => (v, false),
                        None => continue,
                    },
                    5 => match props.alpha_thermal {
                        Some(v) => (v, true),
                        None => continue,
                    },
                    6 => match props.d_diffusion {
                        Some(v) => (v, true),
                        None => continue,
                    },
                    7 => match props.v_advective {
                        Some(v) => (v, false),
                        None => continue,
                    },
                    _ => continue,
                },
                None => match smp.force_type as u8 {
                    0 | 1 => (C_LIGHT, false),
                    _ => continue,
                },
            };
```

---

## WP10 — Protokoll: Absorption im Record

**E41.** L1841–1852 `out.extend_from_slice`-Block delete and replace with:
```rust
            out.extend_from_slice(
                &(records.len() as u32).to_le_bytes(),
            );
            let absorbs: Vec<f64> = records
                .iter()
                .map(|&(_, _, _, _, _, _, _, _, force_type)| {
                    let id = force_type as u8;
                    match id {
                        0 | 1 => 0.0,
                        2 => 0.1,
                        3 => 0.05,
                        4 => 0.08,
                        5 => 0.01,
                        6 => 0.02,
                        7 => 0.15,
                        _ => 0.0,
                    }
                })
                .collect();
            for (i, &(x, y, z, val, extent, epoch, ttl, tau, force_type)) in
                records.iter().enumerate()
            {
                out.extend_from_slice(&x.to_le_bytes());
                out.extend_from_slice(&y.to_le_bytes());
                out.extend_from_slice(&z.to_le_bytes());
                out.extend_from_slice(&val.to_le_bytes());
                out.extend_from_slice(&extent.to_le_bytes());
                out.extend_from_slice(&epoch.to_le_bytes());
                out.extend_from_slice(&ttl.to_le_bytes());
                out.extend_from_slice(&tau.to_le_bytes());
                out.extend_from_slice(&force_type.to_le_bytes());
                out.extend_from_slice(&absorbs[i].to_le_bytes());
            }
```

**E42.** `static/constants.js` L70 change from:
```js
        if (o + 72 > bytes.length) break;
```
to:
```js
        if (o + 80 > bytes.length) break;
```

L80 after `const force_type = dvRes.getFloat64(o, true); o += 8;` insert:
```js
        const absorption = dvRes.getFloat64(o, true); o += 8;
```

L93 change from:
```js
        meta[mOff + 3] = 0;
```
to:
```js
        meta[mOff + 3] = absorption;
```

---

## WP11 — client WGSL: delete the media constants

**E43.** `static/index.html` L236–239 delete:
```rust
const ALPHA_AIR: f32 = 2.18e-5;
const V_P_GRANITE: f32 = 5950.0;
const V_S_GRANITE: f32 = 3630.0;
const D_AIR: f32 = 2.0e-5;
```

**E44.** `absorption_coeff` (L253–264) delete and replace with:
```rust
fn absorption_coeff(abs_coeff: f32) -> f32 {
    return abs_coeff;
}
```

**E45.** L249 `struct VOut` extend — after `@location(2) @interpolate(flat) force_type: u32` insert:
```rust
@location(3) @interpolate(flat) absorption: f32,
```

**E46.** Vertex-Shader: L287 `out.force_type = 0u;` → darunter:
```rust
    out.absorption = 0.0;
    out.force_type = 0u;
```
L340 `out.force_type = f_type;` → darunter:
```rust
    out.absorption = mt.w;
    out.force_type = f_type;
```

**E47.** L415 `fs`-Aufruf change from:
```rust
    let absorption = absorption_coeff(in.force_type);
```
to:
```rust
    let absorption = absorption_coeff(in.absorption);
```

---

## WP12 — Ephemeriden-Generator

**E48.** `scripts/generate_ephemerides.py`: after the stype==1 block, append the stype==2 section (type=2, count=1, degree=5, 6 f64):

```python
    f.write(struct.pack('<III', 2, 1, 5))
    f.write(struct.pack('<I', 0))
    if body_name == 'earth':
        f.write(struct.pack(
            '<dddddd',
            343.0,      # v_sound
            5950.0,     # v_seismic_p
            3630.0,     # v_seismic_s
            2.18e-5,    # alpha_thermal
            2.0e-5,     # d_diffusion
            10.0,       # v_advective
        ))
    else:
        f.write(struct.pack('<dddddd', 0.0, 0.0, 0.0, 0.0, 0.0, 0.0))
```

---

## WP13 — Verifikation

```bash
cargo build && cargo test
```

All tests must be green.

```bash
grep -n 'EARTH_RADIUS\|6378137\.0\|6378136\.6\|111319\.0\|0\.40909\|280\.460\|360\.985\|DEMO_KEY\|Geodetic\|V_SOUND_288\|V_P_GRANITE\|V_S_GRANITE\|D_AIR\|ALPHA_AIR\|force_constants' src/main.rs
```
**Ziel: 0 Treffer.**

```bash
grep -n '"earth"' src/main.rs
```
**Ziel: 0 Treffer** (ausser L4088 Provider-URL in Test).

```bash
grep -n 'v_sound\|v_seismic_p\|v_seismic_s\|alpha_thermal\|d_diffusion\|v_advective\|surface_motion\|icrs_to_body_surface\|frame_motion\|body_props' src/main.rs | wc -l
```
**Ziel: >0** (die neuen generischen Namen sind im Code).
