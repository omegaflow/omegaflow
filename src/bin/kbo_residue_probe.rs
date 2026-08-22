use omegaflow::archivar::{
    body_barycenter_position, download_ephemeris_batch, fetch_raw, load_sources,
    parse_ephemeris_binary, BodyEphemeris, SourceConfig, J2000_EPOCH,
};
use omegaflow::json::{parse_json, JsonVal};
use omegaflow::kbo::{family_name, rec_from_row, state_at, KboRec, FAM_CLASSICAL};
use omegaflow::te::{surrogate_stats_phase, transfer_entropy_lag};
use std::collections::HashMap;

const PLANETS: [&str; 8] = [
    "mercury", "venus", "earth", "mars", "jupiter", "saturn", "uranus", "neptune",
];
const GM_SUN: f64 = omegaflow::kepler::GM_SUN_M3_S2;
const AU_M: f64 = omegaflow::kepler::AU_M;
const DT_DAYS: f64 = 30.0;
const W_YR: f64 = 400.0;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

struct Planet {
    name: String,
    gm: f64,
}

fn coverage(eph: &BodyEphemeris) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in &eph.granules {
        lo = lo.min(g.t0_jd - g.dt_jd);
        hi = hi.max(g.t0_jd + g.dt_jd);
    }
    (lo, hi)
}

fn acc(
    r: [f64; 3],
    tdb: f64,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<[f64; 3]> {
    let r2 = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
    let r3 = r2 * r2.sqrt();
    let mut a = [
        -GM_SUN * r[0] / r3,
        -GM_SUN * r[1] / r3,
        -GM_SUN * r[2] / r3,
    ];
    for p in planets {
        let q = body_barycenter_position(&p.name, tdb, eph)?;
        let dx = q[0] - r[0];
        let dy = q[1] - r[1];
        let dz = q[2] - r[2];
        let d2 = dx * dx + dy * dy + dz * dz;
        let d3 = d2 * d2.sqrt();
        if d3 <= 0.0 {
            continue;
        }
        a[0] += p.gm * dx / d3;
        a[1] += p.gm * dy / d3;
        a[2] += p.gm * dz / d3;
    }
    Some(a)
}

fn elements_from_state(r: [f64; 3], v: [f64; 3]) -> (f64, f64, f64) {
    let h = [
        r[1] * v[2] - r[2] * v[1],
        r[2] * v[0] - r[0] * v[2],
        r[0] * v[1] - r[1] * v[0],
    ];
    let hh = (h[0] * h[0] + h[1] * h[1] + h[2] * h[2]).sqrt();
    if hh <= 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let inc = (h[2] / hh).clamp(-1.0, 1.0).acos();
    let rn = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
    if rn <= 0.0 {
        return (inc, 0.0, 0.0);
    }
    let vh = [
        v[1] * h[2] - v[2] * h[1],
        v[2] * h[0] - v[0] * h[2],
        v[0] * h[1] - v[1] * h[0],
    ];
    let ev = [
        vh[0] / GM_SUN - r[0] / rn,
        vh[1] / GM_SUN - r[1] / rn,
        vh[2] / GM_SUN - r[2] / rn,
    ];
    let e = (ev[0] * ev[0] + ev[1] * ev[1] + ev[2] * ev[2]).sqrt();
    let nx = -h[1];
    let ny = h[0];
    let nn = (nx * nx + ny * ny).sqrt();
    if nn < 1e-12 || e < 1e-12 {
        return (inc, e, 0.0);
    }
    let node = ny.atan2(nx);
    let cos_w = (nx * ev[0] + ny * ev[1]) / (nn * e);
    let w = if ev[2] >= 0.0 {
        cos_w.clamp(-1.0, 1.0).acos()
    } else {
        -cos_w.clamp(-1.0, 1.0).acos()
    };
    (inc, e, node + w)
}

fn series_of(
    rec: &KboRec,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
    dt_d: f64,
    samples: usize,
) -> Option<(Vec<f32>, Vec<f32>, f64)> {
    let t_lo = rec.epoch_jd - W_YR * 365.25;
    let t_hi = rec.epoch_jd + W_YR * 365.25;
    for p in planets {
        let (lo, hi) = coverage(eph.get(&p.name)?);
        if t_lo < lo || t_hi > hi {
            return None;
        }
    }
    let (r0, v0) = state_at(rec, rec.epoch_jd)?;
    let steps = ((2.0 * W_YR * 365.25) / dt_d) as usize;
    if steps < 2 * samples {
        return None;
    }
    let stride = steps / samples;
    let tdb0 = (rec.epoch_jd - J2000_EPOCH) * 86400.0;
    let dt = dt_d * 86400.0;
    let mut r = r0;
    let mut v = v0;
    let a0 = acc(r, tdb0, planets, eph)?;
    v = [
        v[0] + 0.5 * dt * a0[0],
        v[1] + 0.5 * dt * a0[1],
        v[2] + 0.5 * dt * a0[2],
    ];
    let mut rs = Vec::with_capacity(samples);
    let mut ws = Vec::with_capacity(samples);
    let mut prev_w = 0.0;
    for k in 1..=steps {
        let tdb = tdb0 + k as f64 * dt;
        r = [r[0] + dt * v[0], r[1] + dt * v[1], r[2] + dt * v[2]];
        let a = acc(r, tdb, planets, eph)?;
        v = [v[0] + dt * a[0], v[1] + dt * a[1], v[2] + dt * a[2]];
        if k % stride == 0 && rs.len() < samples {
            let t_jd = rec.epoch_jd + k as f64 * dt_d;
            let (kr, _) = state_at(rec, t_jd)?;
            let dx = (r[0] - kr[0]) / AU_M;
            let dy = (r[1] - kr[1]) / AU_M;
            let dz = (r[2] - kr[2]) / AU_M;
            let (_, _, mut w) = elements_from_state(r, v);
            let mut d = w - prev_w;
            while d > std::f64::consts::PI {
                w -= std::f64::consts::TAU;
                d -= std::f64::consts::TAU;
            }
            while d < -std::f64::consts::PI {
                w += std::f64::consts::TAU;
                d += std::f64::consts::TAU;
            }
            prev_w = w;
            rs.push((dx * dx + dy * dy + dz * dz).sqrt() as f32);
            ws.push(w as f32);
        }
    }
    let max_r = rs.iter().copied().fold(0.0f32, f32::max);
    Some((rs, ws, max_r as f64))
}

fn load_kbo_asset(bin: &str) -> Vec<KboRec> {
    let (text, source_name) = if std::path::Path::new(bin).exists() {
        (std::fs::read_to_string(bin).ok(), bin.to_string())
    } else {
        let sources = load_sources();
        let block = sources.iter().find(|s| s.url.contains("kbo_elements.json"));
        match block {
            Some(s) => (
                fetch_raw(&s.url, None, &[], s.ttl),
                format!("sources.φ {}", s.url),
            ),
            None => (None, "block missing".to_string()),
        }
    };
    let Some(text) = text else {
        eprintln!("kbo katalog: fetch void ({source_name})");
        return Vec::new();
    };
    let Some(root) = parse_json(&text) else {
        eprintln!("kbo katalog: json void ({source_name})");
        return Vec::new();
    };
    let JsonVal::Obj(m) = &root else {
        eprintln!("kbo katalog: shape void");
        return Vec::new();
    };
    let Some(JsonVal::Arr(rows)) = m.get("data") else {
        eprintln!("kbo katalog: data void");
        return Vec::new();
    };
    let recs: Vec<KboRec> = rows.iter().filter_map(rec_from_row).collect();
    let n = recs.len();
    eprintln!("kbo katalog: {source_name} — {n} objekte");
    recs
}

fn family_verdict(
    label: &str,
    n: usize,
    rs: &[f32],
    ws: &[f32],
    lag_max: usize,
    fam: f64,
) -> (f64, usize, f64) {
    if n < 30 {
        println!("{label:<22} n {n:<5} keine Aussage (n < 30)");
        return (0.0, 0, fam);
    }
    let mut best_te = 0.0f64;
    let mut best_lag = 0usize;
    for lag in 0..=lag_max {
        if let Some(te) = transfer_entropy_lag(rs, ws, lag) {
            if te > best_te {
                best_te = te;
                best_lag = lag;
            }
        }
    }
    let mut rev_te = 0.0f64;
    let mut rev_lag = 0usize;
    for lag in 0..=lag_max {
        if let Some(te) = transfer_entropy_lag(ws, rs, lag) {
            if te > rev_te {
                rev_te = te;
                rev_lag = lag;
            }
        }
    }
    let (forward, dir) = if best_te >= rev_te {
        (best_te, "R->ϖ")
    } else {
        (rev_te, "ϖ->R")
    };
    let lag = if best_te >= rev_te { best_lag } else { rev_lag };
    let thr = surrogate_stats_phase(rs, ws, lag, SEED)
        .map(|(_, _, t)| t)
        .unwrap_or(0.0);
    let word = if forward > fam {
        "fam-tragend"
    } else if forward > thr {
        "Pfeil ueber eigener Schwelle, unter Familien-Schwelle"
    } else {
        "still"
    };
    println!(
        "{label:<22} n {n:<5} te {forward:.4e} ({dir}, lag {lag}) thr {thr:.4e} fam {fam:.4e} | {word}"
    );
    (forward, lag, thr)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut bin = "kbo_elements.json".to_string();
    let mut limit = usize::MAX;
    let mut family_filter: Option<u8> = None;
    let mut lag_max = 64usize;
    let mut samples = 256usize;
    let mut dt_d = DT_DAYS;
    let mut check_only = false;
    let mut offline = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--bin" => {
                i += 1;
                bin = args.get(i).cloned().unwrap_or(bin);
            }
            "--limit" => {
                i += 1;
                limit = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(limit);
            }
            "--family" => {
                i += 1;
                family_filter = args.get(i).and_then(|s| s.parse().ok());
            }
            "--lag-max" => {
                i += 1;
                lag_max = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(lag_max);
            }
            "--samples" => {
                i += 1;
                samples = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(samples);
            }
            "--dt-d" => {
                i += 1;
                dt_d = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(dt_d);
            }
            "--offline" => offline = true,
            "--check-bins" => check_only = true,
            _ => {
                eprintln!(
                    "usage: kbo_residue_probe [--bin <f>] [--limit N] [--family F] [--lag-max M] [--samples S] [--dt-d D] [--offline] [--check-bins]"
                );
                return;
            }
        }
        i += 1;
    }

    let sources = load_sources();
    let items: Vec<(usize, SourceConfig, String)> = sources
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            s.format == "ephemeris_binary" && PLANETS.contains(&s.body.as_deref().unwrap_or(""))
        })
        .map(|(idx, s)| {
            (
                idx,
                s.clone(),
                format!("ephemeris_{}.bin", s.body.as_deref().unwrap_or("")),
            )
        })
        .collect();
    if offline {
        eprintln!("offline: {} planeten-bins aus dem cache", items.len());
    } else {
        download_ephemeris_batch(&items);
    }
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut planets: Vec<Planet> = Vec::new();
    for (_, s, path) in &items {
        let name = s.body.clone().unwrap_or_default();
        match std::fs::read(path)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
        {
            Some(e) => {
                let gm = e.props.as_ref().and_then(|p| p.gm);
                let (lo, hi) = coverage(&e);
                eprintln!("{name:<9} gm {gm:?} span {lo:.2}..{hi:.2}");
                match gm {
                    Some(gm) if gm.is_finite() && gm > 0.0 => {
                        planets.push(Planet {
                            name: name.clone(),
                            gm,
                        });
                        eph.insert(name, e);
                    }
                    _ => eprintln!("{name}: gm fehlt — der Koerper traegt die Masse nicht"),
                }
            }
            None => eprintln!("{name}: bin parse void"),
        }
    }
    if check_only {
        return;
    }
    if planets.len() < PLANETS.len() {
        eprintln!(
            "modell unvollstaendig: {} von {} planeten",
            planets.len(),
            PLANETS.len()
        );
        if planets.is_empty() {
            return;
        }
    }

    let recs = load_kbo_asset(&bin);
    let sel: Vec<KboRec> = recs
        .into_iter()
        .filter(|r| family_filter.map(|f| r.family == f).unwrap_or(true))
        .take(limit)
        .collect();
    eprintln!("objekte: {} selektiert", sel.len());
    if sel.is_empty() {
        return;
    }

    println!("=== Nadel VI: TE(Residuum -> Bahn) je Familie ===");
    println!(
        "Konstruktion: R(t) = |Kepler(Bahn_geerntet) - N-Koerper(Sun+8, Leapfrog dt {dt_d} d)| in AU; Bahn = ϖ(t) aus dem N-Koerper-Lauf; {samples} Samples je Objekt, Fenster ±{W_YR} yr (Deckungs-Bound der ephemeris-Bins); Mittel-Reihe je Familie."
    );
    println!(
        "Nullkontrollen: I Sun-only-Selbstlauf (R muss ≈ 0), II kalte Klassische (e<0.15, i<5 Grad), III phasenrandomisierte Surrogat-Schwelle (mean+2σ, f64 FFT); fam = max Surrogat-Schwelle der Runde."
    );

    let mut fam_rs: HashMap<u8, Vec<f64>> = HashMap::new();
    let mut fam_ws: HashMap<u8, Vec<f64>> = HashMap::new();
    let mut fam_n: HashMap<u8, usize> = HashMap::new();
    let mut fam_rstat: HashMap<u8, (f64, f64)> = HashMap::new();
    let mut cold_rs: Vec<f64> = vec![0.0; samples];
    let mut cold_ws: Vec<f64> = vec![0.0; samples];
    let mut cold_n = 0usize;
    let mut model_gaps = 0usize;

    for rec in &sel {
        let Some((rs, ws, max_r)) = series_of(rec, &planets, &eph, dt_d, samples) else {
            model_gaps += 1;
            continue;
        };
        let n = *fam_n.entry(rec.family).or_insert(0);
        if n == 0 {
            fam_rs.insert(rec.family, vec![0.0; samples]);
            fam_ws.insert(rec.family, vec![0.0; samples]);
        }
        for k in 0..samples {
            fam_rs.get_mut(&rec.family).unwrap()[k] += rs[k] as f64;
            fam_ws.get_mut(&rec.family).unwrap()[k] += ws[k] as f64;
        }
        fam_n.insert(rec.family, n + 1);
        let st = fam_rstat.entry(rec.family).or_insert((0.0, 0.0));
        st.0 += max_r;
        st.1 = st.1.max(max_r);
        if rec.family == FAM_CLASSICAL && rec.e < 0.15 && rec.incl_deg < 5.0 {
            for k in 0..samples {
                cold_rs[k] += rs[k] as f64;
                cold_ws[k] += ws[k] as f64;
            }
            cold_n += 1;
        }
    }

    let mut fams: Vec<u8> = fam_n.keys().copied().collect();
    fams.sort();
    let mut fam = 0.0f64;
    for f in &fams {
        let n = fam_n[f];
        let f_rs: Vec<f32> = fam_rs[f].iter().map(|&v| (v / n as f64) as f32).collect();
        let f_ws: Vec<f32> = fam_ws[f].iter().map(|&v| (v / n as f64) as f32).collect();
        for lag in 0..=lag_max {
            for (x, y) in [(&f_rs, &f_ws), (&f_ws, &f_rs)] {
                if let Some((_, _, thr)) = surrogate_stats_phase(x, y, lag, SEED) {
                    fam = fam.max(thr);
                }
            }
        }
    }

    for f in &fams {
        let n = fam_n[f];
        if n < 30 {
            family_verdict(&family_name(*f).to_string(), n, &[], &[], lag_max, fam);
            continue;
        }
        let f_rs: Vec<f32> = fam_rs[f].iter().map(|&v| (v / n as f64) as f32).collect();
        let f_ws: Vec<f32> = fam_ws[f].iter().map(|&v| (v / n as f64) as f32).collect();
        let st = fam_rstat[f];
        let mean_r = st.0 / n as f64;
        let (te, lag, _) =
            family_verdict(&family_name(*f).to_string(), n, &f_rs, &f_ws, lag_max, fam);
        println!(
            "  residuum |R| mittel {mean_r:.4e} AU, max je Objekt {:.4e} AU, te {te:.4e} @ lag {lag}",
            st.1
        );
    }

    let (cold_rs_f, cold_ws_f): (Vec<f32>, Vec<f32>) = if cold_n >= 30 {
        (
            cold_rs
                .iter()
                .map(|&v| (v / cold_n as f64) as f32)
                .collect(),
            cold_ws
                .iter()
                .map(|&v| (v / cold_n as f64) as f32)
                .collect(),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    if cold_n >= 30 {
        family_verdict(
            "Nullkontrolle II kalt",
            cold_n,
            &cold_rs_f,
            &cold_ws_f,
            lag_max,
            fam,
        );
    } else {
        println!("Nullkontrolle II kalt n {cold_n} keine Aussage (n < 30)");
    }

    if let Some(first) = sel.first() {
        let empty_planets: Vec<Planet> = Vec::new();
        let (_, _, max0) = match series_of(first, &empty_planets, &eph, dt_d, samples) {
            Some(s) => s,
            None => (Vec::new(), Vec::new(), f64::NAN),
        };
        println!(
            "Nullkontrolle I Sun-only-Selbstlauf ({}): max R {:.3e} AU — der Integrator gegen die Kepler-Referenz",
            family_name(first.family),
            max0
        );
    }
    eprintln!(
        "modell-luecken (serie void): {model_gaps} von {} — Chebyshev-Deckung oder 0-Kanon-Sprung",
        sel.len()
    );
}
