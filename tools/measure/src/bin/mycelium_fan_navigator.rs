use omegaflow::archivar::spectral::civil_from_days;
use omegaflow::json::{parse_json, JsonVal};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const FINK_CONE: &str = "https://api.lsst.fink-portal.org/api/v1/conesearch";
const FINK_RA: &str = "r:ra";
const FINK_DEC: &str = "r:dec";
const FINK_NDIA: &str = "r:nDiaSources";
const FINK_CLASS: &str = "f:main_label_classifier";
const FINK_SIMBAD: &str = "f:xm_simbad_otype";
const IRSA_TAP: &str = "https://irsa.ipac.caltech.edu/TAP/sync";
const ALLWISE_TABLE: &str = "allsky_4band_p3as_psd";
const UA: &str = "omegaflow-mycelium-fan-navigator/1.0";

const GOLDEN_ANGLE_DEG: f64 = 137.50776405003785;
const FLOOR_DEFAULT: usize = 24;
const STEPS_DEFAULT: usize = 2;
const FRONTIER_POOL_DEFAULT: usize = 24;
const HTTP_RETRY: usize = 3;
const RATE_LIMIT_BACKOFF_MS: u64 = 3000;
const CONE_PAUSE_MS: u64 = 1000;
const OBJECT_PAUSE_MS: u64 = 250;
const WISE_RADIUS_ARCSEC: f64 = 6.0;
const AGN_WEDGE_W1_W2: f64 = 0.8;
const ALLWISE_MAG_CODE_MIN: f64 = 90.0;
const RA_NGP_DEG: f64 = 192.85948;
const DEC_NGP_DEG: f64 = 27.12825;
const OBSERVABILITY_LAT_SCALE_DEG: f64 = 10.0;
const FEEDER_MIN_FOOD: usize = 1;
const REINFORCE_BRANCHES: usize = 4;
const REINFORCE_RADIUS_DIV: f64 = 2.0;
const MIN_FINE_RADIUS_ARCSEC: f64 = 60.0;
const GENERATION_ATTEMPT_LIMIT: usize = 600;
const RIVER_WINDOW_MULT: f64 = 3.0;
const MYCELIUM_WINDOW_MULT: f64 = 2.0;
const FOOD_SATURATION: f64 = 1.0;
const SEPARATION_EPS_ARCSEC: f64 = 1.0;
const DEFAULT_REGISTER: &str = "phi/reports/scan_coverage.φ";

#[derive(Clone)]
struct ConeVisit {
    ra_deg: f64,
    dec_deg: f64,
    radius_arcsec: f64,
    food: Option<usize>,
}

#[derive(Clone)]
struct ConeCandidate {
    ra_deg: f64,
    dec_deg: f64,
    radius_arcsec: f64,
    label: String,
}

struct ConeObjectRow {
    ra_deg: f64,
    dec_deg: f64,
    n_sources: usize,
    class: i64,
    simbad: String,
}

struct ConeMeasure {
    answered: bool,
    rows_total: usize,
    above_floor: usize,
    natural_excluded: usize,
    agn_excluded: usize,
    wise_pending: usize,
    food: usize,
}

struct WiseMatch {
    sep_arcsec: f64,
    w1: Option<f64>,
    w2: Option<f64>,
    w1_sig: Option<f64>,
}

enum WiseTag {
    Agn,
    Field,
    Pending,
}

struct VoiceScores {
    mountain: Option<f64>,
    river: Option<f64>,
    mycelium: Option<f64>,
    sensory: Option<f64>,
    future: Option<f64>,
    synthesis: f64,
}

struct StepChoice {
    chosen_index: usize,
    scores: Vec<(usize, VoiceScores)>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == name {
            if i + 1 < args.len() {
                return Some(args[i + 1].clone());
            }
            return None;
        }
        i += 1;
    }
    None
}

fn has_arg(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn arg_f64(args: &[String], name: &str) -> Option<f64> {
    let s = arg_value(args, name)?;
    let v: f64 = s.trim().parse().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    let s = arg_value(args, name)?;
    s.trim().parse().ok()
}

fn usage() {
    eprintln!(
        "mycelium_fan_navigator — the autonomous scan navigator (five voices, golden-angle fan)\n\
         coverage state: phi/reports/scan_coverage.φ cone lines (center ra/dec, radius, outcome)\n\
         \x20 mycelium_fan_navigator [--register <path>] [--steps N=2] [--floor N=24]\n\
         \x20   [--ra deg --dec deg --radius arcsec]  (fresh kernel; otherwise the register's first measured cone anchors the fan)\n\
         \x20   [--frontier-pool N=24]   (frontier candidate cap per step)\n\
         \x20   [--no-wise]   (skip the AllWISE mid-IR AGN witness)\n\
         \x20   [--dry]       (plan the first step: score the frontier, choose the cone, measure nothing, write nothing)\n\
         each step scores successor cones with the five voices (mountain, river, mycelium, sensory, future),\n\
         chooses the geometric-mean synthesis of the present scores, measures the cone on the Fink/IRSA route,\n\
         and appends the cone with its five scores to the register"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if has_arg(&args, "--help") {
        usage();
        return;
    }
    let register_path = match arg_value(&args, "--register") {
        Some(p) => p,
        None => DEFAULT_REGISTER.to_string(),
    };
    let steps = match arg_usize(&args, "--steps") {
        Some(s) => s,
        None => STEPS_DEFAULT,
    };
    let floor = match arg_usize(&args, "--floor") {
        Some(f) => f,
        None => FLOOR_DEFAULT,
    };
    let pool_cap = match arg_usize(&args, "--frontier-pool") {
        Some(c) => c,
        None => FRONTIER_POOL_DEFAULT,
    };
    let dry = has_arg(&args, "--dry");
    let wise = !has_arg(&args, "--no-wise");
    let ra_opt = arg_f64(&args, "--ra");
    let dec_opt = arg_f64(&args, "--dec");
    let radius_opt = arg_f64(&args, "--radius");

    let body = read_file(&register_path);
    let visits = parse_register(&body);
    let start = match first_measured(&visits) {
        Some(v) => (v.ra_deg, v.dec_deg, v.radius_arcsec),
        None => match (ra_opt, dec_opt, radius_opt) {
            (Some(ra), Some(dec), Some(radius)) => (ra, dec, radius),
            _ => {
                println!(
                    "mycelium: the register carries no measured cone and no kernel coordinate was given (--ra --dec --radius) — the campaign stays pending"
                );
                return;
            }
        },
    };
    println!(
        "mycelium campaign: register {register_path}, {steps} step(s), floor {floor}, AllWISE {}; kernel cone ra {:.4} dec {:.4} radius {:.0} arcsec",
        if wise { "on" } else { "off" },
        start.0,
        start.1,
        start.2
    );
    if dry {
        println!(
            "mycelium dry planning: the first step is scored against the registered coverage; the live route and the register stay untouched (pending)"
        );
        plan_step(&start, &visits, pool_cap);
        return;
    }
    let note = format!(
        "note mycelium campaign date {} kernel ra {:.4} dec {:.4} radius {:.0} arcsec floor {} steps {steps} AllWISE {}",
        register_date(),
        start.0,
        start.1,
        start.2,
        floor,
        if wise { "on" } else { "off" }
    );
    append_register(&register_path, &note);
    let mut active = visits;
    if first_measured(&active).is_none() {
        println!(
            "mycelium step 1 of {steps}: the fresh kernel cone itself is measured first (it anchors the coverage state)"
        );
        let kernel = ConeCandidate {
            ra_deg: start.0,
            dec_deg: start.1,
            radius_arcsec: start.2,
            label: "kernel".to_string(),
        };
        measure_and_record(&kernel, floor, wise, &register_path, &mut active);
        let mut run = 1usize;
        while run < steps {
            if !campaign_step(
                &start,
                &mut active,
                floor,
                pool_cap,
                wise,
                &register_path,
                run + 1,
                steps,
            ) {
                break;
            }
            run += 1;
        }
        return;
    }
    let mut run = 0usize;
    while run < steps {
        if !campaign_step(
            &start,
            &mut active,
            floor,
            pool_cap,
            wise,
            &register_path,
            run + 1,
            steps,
        ) {
            break;
        }
        run += 1;
    }
}

fn plan_step(start: &(f64, f64, f64), active: &[ConeVisit], pool_cap: usize) {
    let frontier = build_frontier(start, active, pool_cap);
    if frontier.is_empty() {
        println!(
            "mycelium dry plan: the frontier is empty — every fan position lies inside a registered cone (0 honored)"
        );
        return;
    }
    match choose_from_frontier(&frontier, active, start.2) {
        Some(choice) => {
            print_choice(&frontier, &choice, "dry plan");
            let c = &frontier[choice.chosen_index];
            println!(
                "mycelium dry plan: the chosen cone ra {:.4} dec {:.4} radius {:.0} arcsec ({label}) would be measured on the live route — measurement pending",
                c.ra_deg,
                c.dec_deg,
                c.radius_arcsec,
                label = c.label
            );
        }
        None => {
            println!(
                "mycelium dry plan: no frontier cone carries a measurable synthesis — the campaign rests"
            );
        }
    }
}

fn campaign_step(
    start: &(f64, f64, f64),
    active: &mut Vec<ConeVisit>,
    floor: usize,
    pool_cap: usize,
    wise: bool,
    register_path: &str,
    step_no: usize,
    steps: usize,
) -> bool {
    let heading = format!("step {step_no} of {steps}");
    let frontier = build_frontier(start, active, pool_cap);
    if frontier.is_empty() {
        println!(
            "mycelium {heading}: the frontier is empty — every fan position lies inside a registered cone (0 honored); the campaign rests"
        );
        return false;
    }
    match choose_from_frontier(&frontier, active, start.2) {
        Some(choice) => {
            print_choice(&frontier, &choice, &heading);
            let chosen = frontier[choice.chosen_index].clone();
            let scores = &choice.scores[choice.chosen_index].1;
            println!(
                "mycelium {heading} decision: ra {:.4} dec {:.4} radius {:.0} arcsec ({label}) — mountain {m} river {r} mycelium {my} sensory {se} future {fu} synthesis {sy:.4}",
                chosen.ra_deg,
                chosen.dec_deg,
                chosen.radius_arcsec,
                label = chosen.label,
                m = fmt_score(scores.mountain),
                r = fmt_score(scores.river),
                my = fmt_score(scores.mycelium),
                se = fmt_score(scores.sensory),
                fu = fmt_score(scores.future),
                sy = scores.synthesis
            );
            measure_and_record(&chosen, floor, wise, register_path, active);
            true
        }
        None => {
            println!(
                "mycelium {heading}: no frontier cone carries a measurable synthesis — the campaign rests"
            );
            false
        }
    }
}

fn build_frontier(
    start: &(f64, f64, f64),
    active: &[ConeVisit],
    pool_cap: usize,
) -> Vec<ConeCandidate> {
    let mut frontier = golden_fan_candidates(start.0, start.1, start.2, active, pool_cap);
    let reinforce = reinforce_candidates(active, pool_cap);
    frontier.extend(reinforce);
    frontier
}

fn choose_from_frontier(
    frontier: &[ConeCandidate],
    active: &[ConeVisit],
    base_radius_as: f64,
) -> Option<StepChoice> {
    let measured: Vec<&ConeVisit> = active.iter().filter(|v| v.food.is_some()).collect();
    if measured.is_empty() {
        return None;
    }
    let base_radius_deg = base_radius_as / 3600.0;
    let scale_deg = 2.0 * base_radius_deg;
    let river_window_deg = RIVER_WINDOW_MULT * base_radius_deg;
    let mycelium_window_deg = MYCELIUM_WINDOW_MULT * base_radius_deg;
    let mut best: Option<(usize, f64)> = None;
    let mut scores: Vec<(usize, VoiceScores)> = Vec::new();
    for (i, c) in frontier.iter().enumerate() {
        let s = voice_scores(
            c.ra_deg,
            c.dec_deg,
            &measured,
            scale_deg,
            river_window_deg,
            mycelium_window_deg,
        );
        match synthesis_of(&s) {
            Some(v) => {
                let improve = match best {
                    Some((_, bv)) => v > bv,
                    None => true,
                };
                if improve {
                    best = Some((i, v));
                }
            }
            None => {}
        }
        scores.push((i, s));
    }
    match best {
        Some((chosen_index, _)) => Some(StepChoice {
            chosen_index,
            scores,
        }),
        None => None,
    }
}

fn print_choice(frontier: &[ConeCandidate], choice: &StepChoice, heading: &str) {
    println!(
        "mycelium {heading}: {} frontier cone(s) scored; synthesis = geometric mean of the present voice scores (SYNTHESIS_GEOMETRIC_MEAN)",
        choice.scores.len()
    );
    for (i, scores) in &choice.scores {
        let c = &frontier[*i];
        let mark = if *i == choice.chosen_index {
            "  <- chosen"
        } else {
            ""
        };
        println!(
            "  candidate cone ra {ra:.4} dec {dec:.4} radius {radius:.0} arcsec ({label}) — mountain {m} river {r} mycelium {my} sensory {se} future {fu} synthesis {sy}{mark}",
            ra = c.ra_deg,
            dec = c.dec_deg,
            radius = c.radius_arcsec,
            label = c.label,
            m = fmt_score(scores.mountain),
            r = fmt_score(scores.river),
            my = fmt_score(scores.mycelium),
            se = fmt_score(scores.sensory),
            fu = fmt_score(scores.future),
            sy = fmt_score(Some(scores.synthesis))
        );
    }
}

fn voice_scores(
    ra_deg: f64,
    dec_deg: f64,
    measured: &[&ConeVisit],
    scale_deg: f64,
    river_window_deg: f64,
    mycelium_window_deg: f64,
) -> VoiceScores {
    let nearest = nearest_scanned(ra_deg, dec_deg, measured);
    let mountain = match nearest {
        Some((_, d)) => Some(voice_mountain(d, scale_deg)),
        None => None,
    };
    let future = mountain.map(voice_future);
    let river = match nearest {
        Some((n, _)) => voice_river(ra_deg, dec_deg, n, measured, river_window_deg),
        None => None,
    };
    let mycelium = voice_mycelium(ra_deg, dec_deg, measured, mycelium_window_deg);
    let sensory = Some(voice_sensory(ra_deg, dec_deg));
    let mut present: Vec<f64> = Vec::new();
    for s in [mountain, river, mycelium, sensory, future] {
        if let Some(v) = s {
            present.push(v);
        }
    }
    let synthesis = match synthesis_geometric_mean(&present) {
        Some(s) => s,
        None => f64::NAN,
    };
    VoiceScores {
        mountain,
        river,
        mycelium,
        sensory,
        future,
        synthesis,
    }
}

fn synthesis_of(scores: &VoiceScores) -> Option<f64> {
    let s = scores.synthesis;
    if s.is_finite() {
        Some(s)
    } else {
        None
    }
}

fn voice_mountain(nearest_dist_deg: f64, scale_deg: f64) -> f64 {
    (-(nearest_dist_deg / scale_deg)).exp()
}

fn voice_future(mountain: f64) -> f64 {
    1.0 - mountain
}

fn voice_sensory(ra_deg: f64, dec_deg: f64) -> f64 {
    let lat_deg = galactic_lat_deg(ra_deg, dec_deg).abs();
    (lat_deg / OBSERVABILITY_LAT_SCALE_DEG).min(1.0)
}

fn voice_river(
    c_ra: f64,
    c_dec: f64,
    nearest_idx: usize,
    measured: &[&ConeVisit],
    window_deg: f64,
) -> Option<f64> {
    let n = measured[nearest_idx];
    let toward = local_unit(n.ra_deg, n.dec_deg, c_ra, c_dec)?;
    let mut sum = 0.0;
    let mut total = 0usize;
    for (i, m) in measured.iter().enumerate() {
        if i == nearest_idx {
            continue;
        }
        if sep_deg(n.ra_deg, n.dec_deg, m.ra_deg, m.dec_deg) > window_deg {
            continue;
        }
        total += 1;
        if let Some(e) = local_unit(n.ra_deg, n.dec_deg, m.ra_deg, m.dec_deg) {
            let cos = toward.0 * e.0 + toward.1 * e.1;
            if cos > 0.0 {
                sum += cos;
            }
        }
    }
    if total == 0 {
        None
    } else {
        Some(sum / total as f64)
    }
}

fn voice_mycelium(
    ra_deg: f64,
    dec_deg: f64,
    measured: &[&ConeVisit],
    window_deg: f64,
) -> Option<f64> {
    let mut food_sum = 0.0;
    let mut food_measured = false;
    for m in measured {
        if sep_deg(ra_deg, dec_deg, m.ra_deg, m.dec_deg) > window_deg {
            continue;
        }
        match m.food {
            Some(f) => {
                food_measured = true;
                food_sum += f as f64;
            }
            None => {}
        }
    }
    if food_measured {
        Some(food_sum / (food_sum + FOOD_SATURATION))
    } else {
        None
    }
}

fn synthesis_geometric_mean(scores: &[f64]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    let mut product = 1.0;
    for s in scores {
        product *= *s;
    }
    let n = scores.len() as f64;
    let g = product.powf(1.0 / n);
    if g.is_finite() {
        Some(g)
    } else {
        None
    }
}

fn galactic_lat_deg(ra_deg: f64, dec_deg: f64) -> f64 {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let ra_n = RA_NGP_DEG.to_radians();
    let dec_n = DEC_NGP_DEG.to_radians();
    let s = dec.sin() * dec_n.sin() + dec.cos() * dec_n.cos() * (ra - ra_n).cos();
    s.clamp(-1.0, 1.0).asin().to_degrees()
}

fn fmt_score(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.3}"),
        None => "absent".to_string(),
    }
}

fn measure_and_record(
    chosen: &ConeCandidate,
    floor: usize,
    wise: bool,
    register_path: &str,
    active: &mut Vec<ConeVisit>,
) {
    println!(
        "mycelium measuring cone ra {:.4} dec {:.4} radius {:.0} arcsec ({label})",
        chosen.ra_deg,
        chosen.dec_deg,
        chosen.radius_arcsec,
        label = chosen.label
    );
    let m = measure_cone(
        chosen.ra_deg,
        chosen.dec_deg,
        chosen.radius_arcsec,
        floor,
        wise,
    );
    if !m.answered {
        let line = format!(
            "cone ra {:.4} dec {:.4} radius {:.0} instrument fink-lsst date {} outcome pending floor {} via {} (the cone measurement did not answer; registered, not scanned)",
            chosen.ra_deg,
            chosen.dec_deg,
            chosen.radius_arcsec,
            register_date(),
            floor,
            chosen.label
        );
        append_register(register_path, &line);
        active.push(ConeVisit {
            ra_deg: chosen.ra_deg,
            dec_deg: chosen.dec_deg,
            radius_arcsec: chosen.radius_arcsec,
            food: None,
        });
        return;
    }
    let verdict = if m.rows_total == 0 {
        "void; the cone lists no Fink object row"
    } else if m.above_floor == 0 {
        "below-floor; object rows stay under the nDiaSources cut"
    } else if m.food == 0 {
        "no-survivor; every above-floor object carries a natural class or the mid-IR AGN wedge"
    } else {
        "survivors; the unclassified surface the deeper dip scan would dig"
    };
    let line = format!(
        "cone ra {:.4} dec {:.4} radius {:.0} instrument fink-lsst date {} outcome measured floor {} rows-total {} above-floor {} natural-excluded {} agn-excluded {} wise-pending {} food {} via {} ({})",
        chosen.ra_deg,
        chosen.dec_deg,
        chosen.radius_arcsec,
        register_date(),
        floor,
        m.rows_total,
        m.above_floor,
        m.natural_excluded,
        m.agn_excluded,
        m.wise_pending,
        m.food,
        chosen.label,
        verdict
    );
    append_register(register_path, &line);
    active.push(ConeVisit {
        ra_deg: chosen.ra_deg,
        dec_deg: chosen.dec_deg,
        radius_arcsec: chosen.radius_arcsec,
        food: Some(m.food),
    });
    sleep(Duration::from_millis(CONE_PAUSE_MS));
}

fn measure_cone(ra: f64, dec: f64, radius_arcsec: f64, floor: usize, wise: bool) -> ConeMeasure {
    let rows = match cone_rows(ra, dec, radius_arcsec) {
        Some(r) => r,
        None => {
            return ConeMeasure {
                answered: false,
                rows_total: 0,
                above_floor: 0,
                natural_excluded: 0,
                agn_excluded: 0,
                wise_pending: 0,
                food: 0,
            };
        }
    };
    let mut above_floor = 0usize;
    let mut natural_excluded_count = 0usize;
    let mut agn_excluded = 0usize;
    let mut wise_pending = 0usize;
    let mut food = 0usize;
    for r in &rows {
        if r.n_sources < floor {
            continue;
        }
        above_floor += 1;
        if natural_excluded(r.class, &r.simbad) {
            natural_excluded_count += 1;
            continue;
        }
        if wise {
            match allwise_witness(r.ra_deg, r.dec_deg) {
                WiseTag::Agn => agn_excluded += 1,
                WiseTag::Field => food += 1,
                WiseTag::Pending => {
                    food += 1;
                    wise_pending += 1;
                }
            }
            sleep(Duration::from_millis(OBJECT_PAUSE_MS));
        } else {
            food += 1;
        }
    }
    println!(
        "mycelium cone verdict (ra {ra:.4} dec {dec:.4} radius {radius_arcsec:.0} arcsec): {} object row(s), {above_floor} above the floor {floor}, {natural_excluded_count} excluded as natural dimmers, {agn_excluded} by the AllWISE mid-IR AGN wedge ({wise_pending} witness pending), {food} survivor(s) — the food count",
        rows.len()
    );
    ConeMeasure {
        answered: true,
        rows_total: rows.len(),
        above_floor,
        natural_excluded: natural_excluded_count,
        agn_excluded,
        wise_pending,
        food,
    }
}

fn natural_excluded(class: i64, simbad: &str) -> bool {
    !(class == -1 && simbad == "Fail")
}

fn cone_rows(ra: f64, dec: f64, radius_arcsec: f64) -> Option<Vec<ConeObjectRow>> {
    let payload = format!(
        "{{\"ra\": {ra}, \"dec\": {dec}, \"radius\": {radius_arcsec}, \"columns\": \"r:diaObjectId,r:ra,r:dec,r:nDiaSources,f:main_label_classifier,f:xm_simbad_otype\"}}"
    );
    let (code, body) = match curl_post_retry(FINK_CONE, &payload) {
        Some(r) => r,
        None => {
            println!(
                "mycelium cone (ra {ra}, dec {dec}, {radius_arcsec} arcsec): the Fink cone query did not answer (measured stall) — pending"
            );
            return None;
        }
    };
    if code != "200" {
        println!(
            "mycelium cone (ra {ra}, dec {dec}, {radius_arcsec} arcsec): the Fink cone answered HTTP {code} — pending"
        );
        return None;
    }
    let Ok(text) = std::str::from_utf8(&body) else {
        println!(
            "mycelium cone (ra {ra}, dec {dec}, {radius_arcsec} arcsec): the cone body is not UTF-8 — the parser stays pending"
        );
        return None;
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        println!(
            "mycelium cone (ra {ra}, dec {dec}, {radius_arcsec} arcsec): the cone body is not a JSON array — the parser stays pending"
        );
        return None;
    };
    let ids = extract_dia_ids(&body);
    if ids.len() != rows.len() {
        println!(
            "mycelium cone (ra {ra}, dec {dec}, {radius_arcsec} arcsec): {} row(s) but {} exact diaObjectId token(s) — the id alignment stays pending",
            rows.len(),
            ids.len()
        );
        return None;
    }
    let mut objs: Vec<ConeObjectRow> = Vec::new();
    for r in &rows {
        let JsonVal::Obj(m) = r else { continue };
        let (Some(ra_v), Some(dec_v), Some(n_v)) = (
            obj_f64(m, FINK_RA),
            obj_f64(m, FINK_DEC),
            obj_f64(m, FINK_NDIA),
        ) else {
            continue;
        };
        let class = match obj_f64(m, FINK_CLASS) {
            Some(c) if c.is_finite() => c as i64,
            _ => -1,
        };
        let simbad = match obj_str(m, FINK_SIMBAD) {
            Some(s) => s.to_string(),
            None => "Fail".to_string(),
        };
        objs.push(ConeObjectRow {
            ra_deg: ra_v,
            dec_deg: dec_v,
            n_sources: n_v as usize,
            class,
            simbad,
        });
    }
    objs.sort_by(|a, b| b.n_sources.cmp(&a.n_sources));
    println!(
        "mycelium cone (ra {ra}, dec {dec}, {radius_arcsec} arcsec): HTTP {code}, {} object row(s)",
        objs.len()
    );
    Some(objs)
}

fn obj_str<'a>(m: &'a HashMap<String, JsonVal>, key: &str) -> Option<&'a str> {
    match m.get(key) {
        Some(JsonVal::Str(s)) => Some(s),
        _ => None,
    }
}

fn obj_f64(m: &HashMap<String, JsonVal>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(JsonVal::Num(v)) if v.is_finite() => Some(*v),
        _ => None,
    }
}

fn extract_dia_ids(body: &[u8]) -> Vec<String> {
    let Ok(text) = std::str::from_utf8(body) else {
        return Vec::new();
    };
    let needle = "\"r:diaObjectId\":";
    let mut out: Vec<String> = Vec::new();
    let mut pos = 0;
    while let Some(rel) = text[pos..].find(needle) {
        let s = pos + rel + needle.len();
        let mut digits = String::new();
        for c in text[s..].chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else {
                break;
            }
        }
        if !digits.is_empty() {
            let consumed = digits.len();
            out.push(digits);
            pos = s + consumed;
        } else {
            pos = s + 1;
        }
    }
    out
}

fn allwise_witness(ra: f64, dec: f64) -> WiseTag {
    let matches = match allwise_cone(ra, dec) {
        Some(m) => m,
        None => {
            println!(
                "mycelium AllWISE witness ra {ra:.4} dec {dec:.4}: the IRSA TAP query did not answer or answered non-200 — the mid-IR witness stays pending"
            );
            return WiseTag::Pending;
        }
    };
    let m = match matches.first() {
        Some(x) => x,
        None => {
            println!(
                "mycelium AllWISE witness ra {ra:.4} dec {dec:.4}: no AllWISE source within {WISE_RADIUS_ARCSEC} arcsec (0 honored)"
            );
            return WiseTag::Field;
        }
    };
    let color = match (m.w1, m.w2, m.w1_sig) {
        (Some(w1), Some(w2), Some(sig)) if sig > 0.0 => Some(w1 - w2),
        _ => None,
    };
    match color {
        Some(c) if c >= AGN_WEDGE_W1_W2 => {
            println!(
                "mycelium AllWISE witness ra {ra:.4} dec {dec:.4}: nearest match {:.1} arcsec W1-W2 {c:.3} >= {AGN_WEDGE_W1_W2} — the mid-IR AGN wedge excludes the object",
                m.sep_arcsec
            );
            WiseTag::Agn
        }
        Some(c) => {
            println!(
                "mycelium AllWISE witness ra {ra:.4} dec {dec:.4}: nearest match {:.1} arcsec W1-W2 {c:.3} below the {AGN_WEDGE_W1_W2} wedge — the object remains",
                m.sep_arcsec
            );
            WiseTag::Field
        }
        None => {
            println!(
                "mycelium AllWISE witness ra {ra:.4} dec {dec:.4}: nearest match {:.1} arcsec carries no two-band W1/W2 detection — no wedge color (0 honored), the object remains",
                m.sep_arcsec
            );
            WiseTag::Field
        }
    }
}

fn allwise_cone(ra: f64, dec: f64) -> Option<Vec<WiseMatch>> {
    let r_deg = WISE_RADIUS_ARCSEC / 3600.0;
    let adql = format!(
        "SELECT designation, ra, dec, w1mpro, w2mpro, w3mpro, w4mpro, w1sigmpro FROM {ALLWISE_TABLE} WHERE CONTAINS(POINT('ICRS', ra, dec), CIRCLE('ICRS', {ra:.6}, {dec:.6}, {r_deg})) = 1"
    );
    let (code, body) = match irsa_tap_sync(&adql) {
        Some(r) => r,
        None => {
            println!(
                "mycelium AllWISE cone ra {ra:.4} dec {dec:.4}: the IRSA TAP query did not answer (measured stall) — the witness stays pending"
            );
            return None;
        }
    };
    if code != "200" {
        println!(
            "mycelium AllWISE cone ra {ra:.4} dec {dec:.4}: IRSA TAP answered HTTP {code} — the witness stays pending"
        );
        return None;
    }
    let mut matches = parse_wise_csv(&body, ra, dec);
    matches.sort_by(|a, b| a.sep_arcsec.total_cmp(&b.sep_arcsec));
    Some(matches)
}

fn irsa_tap_sync(adql: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-A")
        .arg(UA)
        .arg("-G")
        .arg(IRSA_TAP)
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg("MAXREC=10")
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

fn parse_wise_csv(body: &[u8], ra: f64, dec: f64) -> Vec<WiseMatch> {
    let Ok(text) = std::str::from_utf8(body) else {
        return Vec::new();
    };
    let mut out: Vec<WiseMatch> = Vec::new();
    let mut lines = text.lines();
    let header = match lines.next() {
        Some(h) => h,
        None => return out,
    };
    let cols: Vec<&str> = header.split(',').map(|c| c.trim()).collect();
    let index_of = |name: &str| cols.iter().position(|c| *c == name);
    let (Some(ira), Some(idec), Some(iw1), Some(iw2), Some(iw1s)) = (
        index_of("ra"),
        index_of("dec"),
        index_of("w1mpro"),
        index_of("w2mpro"),
        index_of("w1sigmpro"),
    ) else {
        return out;
    };
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        let (Some(sra), Some(sdec)) = (csv_num(&f, ira), csv_num(&f, idec)) else {
            continue;
        };
        out.push(WiseMatch {
            sep_arcsec: sep_arcsec(ra, dec, sra, sdec),
            w1: mag_num(&f, iw1),
            w2: mag_num(&f, iw2),
            w1_sig: mag_num(&f, iw1s),
        });
    }
    out
}

fn csv_num(f: &[&str], k: usize) -> Option<f64> {
    let cell = f.get(k)?.trim();
    if cell.is_empty() {
        return None;
    }
    let v: f64 = cell.parse().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn mag_num(f: &[&str], k: usize) -> Option<f64> {
    let v = csv_num(f, k)?;
    if v < ALLWISE_MAG_CODE_MIN {
        Some(v)
    } else {
        None
    }
}

fn curl_post_retry(url: &str, json_body: &str) -> Option<(String, Vec<u8>)> {
    for attempt in 0..HTTP_RETRY {
        let resp = match curl_post_bytes(url, json_body) {
            Some(r) => r,
            None => return None,
        };
        if resp.0 == "429" {
            let backoff = RATE_LIMIT_BACKOFF_MS * (attempt as u64 + 1);
            println!(
                "mycelium: HTTP 429 — the endpoint asks for a slower pace; {backoff} ms before the next try (try {})",
                attempt + 1
            );
            sleep(Duration::from_millis(backoff));
            continue;
        }
        return Some(resp);
    }
    println!(
        "mycelium: HTTP 429 held across {HTTP_RETRY} backed-off tries — the rate limit stands, the query stays pending"
    );
    None
}

fn curl_post_bytes(url: &str, json_body: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-A")
        .arg(UA)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(json_body)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn read_file(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => String::new(),
    }
}

fn first_measured(visits: &[ConeVisit]) -> Option<&ConeVisit> {
    visits.iter().find(|v| v.food.is_some())
}

fn parse_register(body: &str) -> Vec<ConeVisit> {
    let mut out: Vec<ConeVisit> = Vec::new();
    for line in body.lines() {
        let words: Vec<String> = line.split_whitespace().map(|w| w.to_string()).collect();
        if words.first().map(String::as_str) != Some("cone") {
            continue;
        }
        let mut i = 1usize;
        let mut ra: Option<f64> = None;
        let mut dec: Option<f64> = None;
        let mut radius: Option<f64> = None;
        let mut outcome: Vec<String> = Vec::new();
        while i < words.len() {
            match words[i].as_str() {
                "ra" => {
                    if i + 1 < words.len() {
                        if let Ok(v) = words[i + 1].parse() {
                            ra = Some(v);
                        }
                    }
                    i += 2;
                }
                "dec" => {
                    if i + 1 < words.len() {
                        if let Ok(v) = words[i + 1].parse() {
                            dec = Some(v);
                        }
                    }
                    i += 2;
                }
                "radius" => {
                    if i + 1 < words.len() {
                        if let Ok(v) = words[i + 1].parse() {
                            radius = Some(v);
                        }
                    }
                    i += 2;
                }
                "outcome" => {
                    if i + 1 < words.len() {
                        outcome = words[i + 1..].to_vec();
                    }
                    break;
                }
                _ => i += 1,
            }
        }
        let (Some(ra), Some(dec), Some(radius)) = (ra, dec, radius) else {
            continue;
        };
        let rest = outcome.join(" ");
        let food = parse_food(&outcome, &rest);
        out.push(ConeVisit {
            ra_deg: ra,
            dec_deg: dec,
            radius_arcsec: radius,
            food,
        });
    }
    out
}

fn parse_food(outcome: &[String], rest: &str) -> Option<usize> {
    if let Some(f) = token_usize_after(outcome, "food") {
        return Some(f);
    }
    if rest.starts_with("void") {
        return Some(0);
    }
    None
}

fn token_usize_after(words: &[String], key: &str) -> Option<usize> {
    let mut i = 0;
    while i < words.len() {
        if words[i] == key {
            if i + 1 < words.len() {
                if let Ok(v) = words[i + 1].parse() {
                    return Some(v);
                }
            }
            return None;
        }
        i += 1;
    }
    None
}

fn register_date() -> String {
    let secs = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => return "1970-01-01".to_string(),
    };
    match civil_from_days((secs / 86400) as i64) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => "1970-01-01".to_string(),
    }
}

fn append_register(path: &str, line: &str) {
    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut f) => {
            if let Err(e) = writeln!(f, "{line}") {
                println!("mycelium: the register line was not written ({path}): {e}");
            }
        }
        Err(e) => println!("mycelium: the register was not opened for append ({path}): {e}"),
    }
}

fn golden_fan_candidates(
    start_ra: f64,
    start_dec: f64,
    base_radius_as: f64,
    visits: &[ConeVisit],
    cap: usize,
) -> Vec<ConeCandidate> {
    let radius_deg = base_radius_as / 3600.0;
    let step_scale_deg = 2.0 * radius_deg;
    let mut out: Vec<ConeCandidate> = Vec::new();
    let mut n = 0usize;
    let mut attempts = 0usize;
    while out.len() < cap && attempts < GENERATION_ATTEMPT_LIMIT {
        n += 1;
        attempts += 1;
        let r = step_scale_deg * (n as f64).sqrt();
        let th = (n as f64) * GOLDEN_ANGLE_DEG.to_radians();
        let dec_rad = start_dec.to_radians();
        let dra = r * th.cos() / dec_rad.cos();
        let ddec = r * th.sin();
        let ra = wrap_ra(start_ra + dra);
        let dec = start_dec + ddec;
        if blocked(ra, dec, base_radius_as, visits, &out) {
            continue;
        }
        out.push(ConeCandidate {
            ra_deg: ra,
            dec_deg: dec,
            radius_arcsec: base_radius_as,
            label: format!("golden-fan-n{n}"),
        });
    }
    out
}

fn reinforce_candidates(visits: &[ConeVisit], cap: usize) -> Vec<ConeCandidate> {
    let mut out: Vec<ConeCandidate> = Vec::new();
    for v in visits {
        let Some(food) = v.food else { continue };
        if food < FEEDER_MIN_FOOD {
            continue;
        }
        let fine_radius_as = v.radius_arcsec / REINFORCE_RADIUS_DIV;
        if fine_radius_as < MIN_FINE_RADIUS_ARCSEC {
            continue;
        }
        let sep_deg = v.radius_arcsec / 3600.0;
        let dec_rad = v.dec_deg.to_radians();
        for j in 0..REINFORCE_BRANCHES {
            if out.len() >= cap {
                break;
            }
            let ang = 360.0 * (j as f64) / (REINFORCE_BRANCHES as f64);
            let rad = ang.to_radians();
            let dra = sep_deg * rad.cos() / dec_rad.cos();
            let ddec = sep_deg * rad.sin();
            let ra = wrap_ra(v.ra_deg + dra);
            let dec = v.dec_deg + ddec;
            if blocked(ra, dec, fine_radius_as, visits, &out) {
                continue;
            }
            out.push(ConeCandidate {
                ra_deg: ra,
                dec_deg: dec,
                radius_arcsec: fine_radius_as,
                label: format!("reinforce-ra{:.2}-dec{:.2}-j{j}", v.ra_deg, v.dec_deg),
            });
        }
    }
    out
}

fn blocked(
    ra: f64,
    dec: f64,
    cand_radius_as: f64,
    visits: &[ConeVisit],
    peers: &[ConeCandidate],
) -> bool {
    for v in visits {
        let d_as = sep_arcsec(ra, dec, v.ra_deg, v.dec_deg);
        if d_as < v.radius_arcsec - SEPARATION_EPS_ARCSEC {
            return true;
        }
        if d_as < cand_radius_as - SEPARATION_EPS_ARCSEC {
            return true;
        }
        if d_as + cand_radius_as <= v.radius_arcsec - SEPARATION_EPS_ARCSEC {
            return true;
        }
    }
    for p in peers {
        let d_as = sep_arcsec(ra, dec, p.ra_deg, p.dec_deg);
        if d_as < p.radius_arcsec - SEPARATION_EPS_ARCSEC {
            return true;
        }
        if d_as < cand_radius_as - SEPARATION_EPS_ARCSEC {
            return true;
        }
    }
    false
}

fn wrap_ra(ra: f64) -> f64 {
    ra.rem_euclid(360.0)
}

fn nearest_scanned(ra_deg: f64, dec_deg: f64, measured: &[&ConeVisit]) -> Option<(usize, f64)> {
    let mut best: Option<(usize, f64)> = None;
    for (i, m) in measured.iter().enumerate() {
        let d = sep_deg(ra_deg, dec_deg, m.ra_deg, m.dec_deg);
        match best {
            Some((_, bd)) => {
                if d < bd {
                    best = Some((i, d));
                }
            }
            None => best = Some((i, d)),
        }
    }
    best
}

fn local_unit(from_ra: f64, from_dec: f64, to_ra: f64, to_dec: f64) -> Option<(f64, f64)> {
    let cos_dec = from_dec.to_radians().cos();
    let x = delta_ra(from_ra, to_ra) * cos_dec;
    let y = to_dec - from_dec;
    let len = (x * x + y * y).sqrt();
    if len > 0.0 {
        Some((x / len, y / len))
    } else {
        None
    }
}

fn delta_ra(a_deg: f64, b_deg: f64) -> f64 {
    let d = (b_deg - a_deg).rem_euclid(360.0);
    if d > 180.0 {
        d - 360.0
    } else {
        d
    }
}

fn sep_deg(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let r1 = ra1.to_radians();
    let d1 = dec1.to_radians();
    let r2 = ra2.to_radians();
    let d2 = dec2.to_radians();
    let a = ((d2 - d1) / 2.0).sin().powi(2) + d1.cos() * d2.cos() * ((r2 - r1) / 2.0).sin().powi(2);
    2.0 * a.sqrt().clamp(0.0, 1.0).asin().to_degrees()
}

fn sep_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    sep_deg(ra1, dec1, ra2, dec2) * 3600.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn register_void_cone_parses_as_measured_zero() {
        let body = "cone ra 266.4168 dec -29.0078 radius 900 instrument fink-lsst date 2026-09-05 outcome void (0 object rows nDiaSources>=24)";
        let visits = parse_register(body);
        assert_eq!(visits.len(), 1);
        assert_eq!(visits[0].ra_deg, 266.4168);
        assert_eq!(visits[0].dec_deg, -29.0078);
        assert_eq!(visits[0].food, Some(0));
    }

    #[test]
    fn register_measured_line_parses_food() {
        let body = "cone ra 267.0 dec -29.5 radius 900 instrument fink-lsst date 2026-09-05 outcome measured floor 24 rows-total 3 above-floor 2 natural-excluded 1 agn-excluded 0 food 1 via golden-fan-n1";
        let visits = parse_register(body);
        assert_eq!(visits.len(), 1);
        assert_eq!(visits[0].radius_arcsec, 900.0);
        assert_eq!(visits[0].food, Some(1));
    }

    #[test]
    fn register_pending_line_carries_no_food() {
        let body = "cone ra 267.0 dec -29.5 radius 900 instrument fink-lsst date 2026-09-05 outcome pending floor 24 via golden-fan-n1";
        let visits = parse_register(body);
        assert_eq!(visits.len(), 1);
        assert_eq!(visits[0].food, None);
    }

    #[test]
    fn mountain_and_future_complement() {
        let m = voice_mountain(0.3, 0.5);
        let f = voice_future(m);
        assert!(approx(m + f, 1.0));
        assert!(m > 0.0 && m < 1.0);
    }

    #[test]
    fn synthesis_geometric_mean_uses_present_scores_only() {
        assert_eq!(synthesis_geometric_mean(&[]), None);
        let g = synthesis_geometric_mean(&[0.5, 1.0, 0.25]);
        assert!(approx(g.unwrap(), 0.5));
    }

    #[test]
    fn sensory_reads_galactic_extinction_latitude() {
        let center = voice_sensory(266.4168, -29.0078);
        assert!(center < 0.1);
        let pole = voice_sensory(192.86, 27.13);
        assert!(pole > 0.99);
        let cap = voice_sensory(0.0, 90.0);
        assert!(approx(cap, 1.0));
    }

    #[test]
    fn golden_fan_keeps_cone_centers_outside_each_others_disk() {
        let radius_as = 900.0;
        let radius_deg = radius_as / 3600.0;
        let visits: Vec<ConeVisit> = vec![ConeVisit {
            ra_deg: 266.4168,
            dec_deg: -29.0078,
            radius_arcsec: radius_as,
            food: Some(0),
        }];
        let fan = golden_fan_candidates(266.4168, -29.0078, radius_as, &visits, 16);
        assert!(!fan.is_empty());
        for a in &fan {
            for v in &visits {
                assert!(sep_deg(a.ra_deg, a.dec_deg, v.ra_deg, v.dec_deg) >= radius_deg);
            }
            for b in &fan {
                if a.ra_deg == b.ra_deg && a.dec_deg == b.dec_deg {
                    continue;
                }
                assert!(sep_deg(a.ra_deg, a.dec_deg, b.ra_deg, b.dec_deg) >= radius_deg);
            }
        }
    }

    #[test]
    fn reinforce_only_around_feeders() {
        let visits: Vec<ConeVisit> = vec![
            ConeVisit {
                ra_deg: 266.0,
                dec_deg: -28.0,
                radius_arcsec: 900.0,
                food: Some(3),
            },
            ConeVisit {
                ra_deg: 266.5,
                dec_deg: -29.0,
                radius_arcsec: 900.0,
                food: Some(0),
            },
        ];
        let fine = reinforce_candidates(&visits, 8);
        assert_eq!(fine.len(), REINFORCE_BRANCHES);
        for c in &fine {
            assert_eq!(c.radius_arcsec, 450.0);
        }
    }
}
