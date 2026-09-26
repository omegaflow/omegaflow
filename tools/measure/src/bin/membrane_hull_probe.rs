use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use omegaflow::archivar::{
    C_LIGHT, anchor_uses, body_barycenter_position, body_in_enclosure, body_record_epoch,
    build_asteroid_samples, build_star_samples, cache_fresh_cdn, catalog_sample_in_enclosure,
    content_cache, embedded_lsk, enclosure_presences, parse_ephemeris_binary, parse_sources,
};
use omegaflow::mathematikerin::PresenceState;

const DEFAULT_DASTCOM_TTL: u64 = 86400;

struct PathStats {
    candidates: usize,
    admit: usize,
    refuse: usize,
    cone_out_of_admit: usize,
    cone_in_of_refuse: usize,
    margins: Vec<f64>,
}

impl PathStats {
    fn new() -> Self {
        PathStats {
            candidates: 0,
            admit: 0,
            refuse: 0,
            cone_out_of_admit: 0,
            cone_in_of_refuse: 0,
            margins: Vec::new(),
        }
    }

    fn record(&mut self, admitted: bool, pos: [f64; 3], center: [f64; 3], rho_star: f64) {
        self.candidates += 1;
        if admitted {
            self.admit += 1;
        } else {
            self.refuse += 1;
        }
        let dx = pos[0] - center[0];
        let dy = pos[1] - center[1];
        let dz = pos[2] - center[2];
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        let inside = dist <= rho_star;
        if admitted && !inside {
            self.cone_out_of_admit += 1;
        }
        if !admitted && inside {
            self.cone_in_of_refuse += 1;
        }
        if admitted != inside {
            self.margins.push((dist - rho_star).abs());
        }
    }

    fn f_excl(&self) -> f64 {
        if self.admit > 0 {
            self.cone_out_of_admit as f64 / self.admit as f64
        } else {
            0.0
        }
    }

    fn f_inc(&self) -> f64 {
        if self.candidates > 0 {
            self.cone_in_of_refuse as f64 / self.candidates as f64
        } else {
            0.0
        }
    }

    fn interdecile(&self) -> (f64, usize) {
        if self.margins.is_empty() {
            return (1.0, 0);
        }
        let mut sorted = self.margins.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let lo = ecdf_at(&sorted, 0.10);
        let hi = ecdf_at(&sorted, 0.90);
        let ratio = if lo > 0.0 { hi / lo } else { f64::INFINITY };
        (ratio, sorted.len())
    }
}

fn ecdf_at(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[idx]
}

fn diverges(stats: &PathStats) -> bool {
    let (rid, _) = stats.interdecile();
    stats.f_excl() > 0.5 || stats.f_inc() > 0.0 || rid > 10.0
}

fn verdict_word(stats: &PathStats) -> String {
    let f_excl = stats.f_excl();
    let f_inc = stats.f_inc();
    let (rid, _) = stats.interdecile();
    if diverges(stats) {
        format!(
            "REFUTED — f_excl {f_excl:.6} f_inc {f_inc:.6} interdecile {rid:.6}: the path's enclosure gate and the resting cone diverge beyond the star_dmax_probe tolerances"
        )
    } else {
        format!(
            "holds — f_excl {f_excl:.6} f_inc {f_inc:.6} interdecile {rid:.6} stays within the star_dmax_probe tolerances"
        )
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn arg_f64(args: &[String], name: &str) -> Option<f64> {
    arg_value(args, name).and_then(|w| w.parse::<f64>().ok())
}

fn usage() {
    println!(
        "usage: membrane_hull_probe [--eph-dir <dir>] [--stars <bin>] [--dastcom <bin>] [--sources <file>] [--pad <m>] [--now <tdb>] [--help]"
    );
    println!(
        "  the three ingress paths are driven against one resting presence hull — rho_star = C_LIGHT*(now - star_epoch_min) + pad, the center read from the resting presence slot (enclosure_presences of an empty presence map)"
    );
    println!(
        "  defaults: eph-dir data/ssd.jpl.nasa.gov, stars data/ssd.jpl.nasa.gov/dr3_stars.bin, dastcom data/ssd.jpl.nasa.gov/dastcom_asteroids.bin, sources phi/sources.φ, pad 1.0, now = the embedded-LSK system TDB (a missing input leaves that line absent — never fabricated)"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if flag(&args, "--help") || flag(&args, "-h") {
        usage();
        return;
    }
    let eph_dir = match arg_value(&args, "--eph-dir") {
        Some(d) => d,
        None => "data/ssd.jpl.nasa.gov".to_string(),
    };
    let stars_path = match arg_value(&args, "--stars") {
        Some(p) => p,
        None => "data/ssd.jpl.nasa.gov/dr3_stars.bin".to_string(),
    };
    let dastcom_path = match arg_value(&args, "--dastcom") {
        Some(p) => p,
        None => "data/ssd.jpl.nasa.gov/dastcom_asteroids.bin".to_string(),
    };
    let sources_path = match arg_value(&args, "--sources") {
        Some(p) => p,
        None => "phi/sources.φ".to_string(),
    };
    let pad = match arg_f64(&args, "--pad") {
        Some(p) if p.is_finite() => p,
        Some(_) => {
            eprintln!("membrane_hull_probe: --pad carries no finite value");
            std::process::exit(2);
        }
        None => 1.0,
    };
    let now = match arg_f64(&args, "--now") {
        Some(t) if t.is_finite() => t,
        Some(_) => {
            eprintln!("membrane_hull_probe: --now carries no finite value");
            std::process::exit(2);
        }
        None => match embedded_lsk().and_then(|l| l.system_now_tdb()) {
            Some(t) => t,
            None => {
                eprintln!(
                    "membrane_hull_probe: the embedded LSK yields no TDB now — pass --now explicitly"
                );
                std::process::exit(2);
            }
        },
    };

    let sources = match std::fs::read_to_string(&sources_path) {
        Ok(c) => parse_sources(&c),
        Err(_) => {
            eprintln!(
                "membrane_hull_probe: {} read void — the bootstrap and per-tick classifications stay absent",
                sources_path
            );
            Vec::new()
        }
    };
    let eph_sources: Vec<_> = sources
        .iter()
        .filter(|s| (s.format == "ephemeris_binary" || s.format == "orbit_bin") && s.body.is_some())
        .collect();

    let mut eph_map: HashMap<String, omegaflow::archivar::BodyEphemeris> = HashMap::new();
    let mut corpus_absent: Vec<String> = Vec::new();
    for s in &eph_sources {
        let body = s.body.as_deref().unwrap_or("");
        if eph_map.contains_key(body) {
            continue;
        }
        let path = format!("{eph_dir}/ephemeris_{body}.bin");
        match std::fs::read(&path)
            .ok()
            .and_then(|b| parse_ephemeris_binary(&b))
        {
            Some(e) => {
                eph_map.insert(body.to_string(), e);
            }
            None => corpus_absent.push(body.to_string()),
        }
    }

    let slot = Arc::new(RwLock::new(PresenceState::rest()));
    let presences = enclosure_presences(&HashMap::new(), &slot, now);
    let (center, presence_range, presence_grid) = match presences.first() {
        Some(&(_, cx, cy, cz, range, _, _, _, _, grid_step)) => ([cx, cy, cz], range, grid_step),
        None => {
            eprintln!(
                "membrane_hull_probe: the resting presence slot yields no hull — no measurement"
            );
            std::process::exit(2);
        }
    };

    let star_bytes = match std::fs::read(&stars_path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!(
                "membrane_hull_probe: {} read void — the cone radius stays unmeasured",
                stars_path
            );
            std::process::exit(2);
        }
    };
    let star_samples = build_star_samples(&star_bytes);
    if star_samples.is_empty() {
        eprintln!(
            "membrane_hull_probe: {} yields no star samples — the measurement is absent, not zero",
            stars_path
        );
        std::process::exit(2);
    }
    let star_epoch_min = star_samples
        .iter()
        .map(|s| s.epoch)
        .fold(f64::MAX, f64::min);
    let rho_star = C_LIGHT * (now - star_epoch_min).abs() + pad;

    let dastcom_ttl = match sources.iter().find(|s| s.format == "catalog_dastcom") {
        Some(s) => s.ttl,
        None => DEFAULT_DASTCOM_TTL,
    };
    let mut dastcom = PathStats::new();
    let mut dastcom_loaded = 0usize;
    match std::fs::read(&dastcom_path) {
        Ok(bytes) => {
            let samples = build_asteroid_samples(&bytes, dastcom_ttl);
            dastcom_loaded = samples.len();
            for s in &samples {
                let admitted = catalog_sample_in_enclosure(&presences, s, now);
                dastcom.record(admitted, s.anchor_p0, center, rho_star);
            }
        }
        Err(_) => {
            eprintln!(
                "membrane_hull_probe: {} read void — the dastcom line stays absent",
                dastcom_path
            );
        }
    }

    let mut tycho = PathStats::new();
    for s in &star_samples {
        let admitted = catalog_sample_in_enclosure(&presences, s, now);
        tycho.record(admitted, s.anchor_p0, center, rho_star);
    }

    let anchor_uses = anchor_uses(&sources);
    let mut bootstrap = PathStats::new();
    let mut boot_fresh = 0usize;
    let mut boot_unloaded = 0usize;
    for s in &eph_sources {
        let body = s.body.as_deref().unwrap_or("");
        let tmp_path = content_cache(&format!("omegaflow_eph_{body}.bin"));
        if cache_fresh_cdn(&tmp_path, s.ttl, &s.url) {
            boot_fresh += 1;
        } else if let Some(e) = eph_map.get(body)
            && let (Some(t_r), Some(props)) = (body_record_epoch(e), e.props.as_ref())
            && let Some(pos) = body_barycenter_position(body, t_r, &eph_map)
        {
            let admitted = body_in_enclosure(&presences, props, pos, t_r, now);
            bootstrap.record(admitted, pos, center, rho_star);
        } else {
            boot_unloaded += 1;
        }
    }

    let mut tick = PathStats::new();
    let mut tick_absent = 0usize;
    for s in &eph_sources {
        let body = s.body.as_deref().unwrap_or("");
        match eph_map.get(body) {
            None => tick_absent += 1,
            Some(e) => match (body_record_epoch(e), e.props.as_ref()) {
                (Some(t_r), Some(props)) => match body_barycenter_position(body, t_r, &eph_map) {
                    Some(pos) => {
                        let admitted = body_in_enclosure(&presences, props, pos, t_r, now);
                        tick.record(admitted, pos, center, rho_star);
                    }
                    None => tick_absent += 1,
                },
                _ => tick_absent += 1,
            },
        }
    }

    let unique_bodies: std::collections::HashSet<&str> = eph_sources
        .iter()
        .filter_map(|s| s.body.as_deref())
        .collect();
    println!(
        "=== the membrane ingress hull measurement — the three bypass paths against one resting presence hull ==="
    );
    println!(
        "NOW {now:.6e} | STAR_EPOCH_MIN {star_epoch_min:.6e} | AGE_STAR {:.6e} | PAD {pad}",
        (now - star_epoch_min).abs()
    );
    println!(
        "HULL center ({:.6e}, {:.6e}, {:.6e}) from the resting presence slot | rho_star {rho_star:.6e} m (C_LIGHT*(now-star_epoch_min)+pad) | presence range {presence_range:.6e} grid_step {presence_grid:.6e}",
        center[0], center[1], center[2]
    );
    println!(
        "SOURCES {} parsed from {} | ephemeris sources {} ({} bodies) | anchor bodies {}",
        sources.len(),
        sources_path,
        eph_sources.len(),
        unique_bodies.len(),
        anchor_uses.len()
    );
    println!(
        "CORPUS {} ephemeris bin(s) opened in {} | absent: {}",
        eph_map.len(),
        eph_dir,
        if corpus_absent.is_empty() {
            "none".to_string()
        } else {
            corpus_absent.join(",")
        }
    );
    println!(
        "STARS {} loaded from {} | DASTCOM {} loaded from {} (ttl {dastcom_ttl})",
        star_samples.len(),
        stars_path,
        dastcom_loaded,
        dastcom_path
    );

    print_path(
        "bootstrap",
        &bootstrap,
        &format!("fresh {boot_fresh} unloaded {boot_unloaded} declared 0"),
    );
    println!(
        "BOOTSTRAP anchor admission (measured): the anchor class passes the same enclosure gate as the rest class — no source may pass by default; declared_body 0 in the probe (a main-flow state, not carried by the probe corpus); fresh {boot_fresh} by the cache classification"
    );

    print_path("per-tick", &tick, &format!("absent_refused {tick_absent}"));
    println!(
        "PER-TICK absent refusal (measured): in_hull refuses {tick_absent} of {} sources whose corpus ephemeris, properties or position is absent — absence is not a value, no fabricated default",
        eph_sources.len()
    );

    print_path("catalog_dastcom", &dastcom, &format!("ttl {dastcom_ttl}"));
    print_path("catalog_tycho", &tycho, "");

    let refuted = [&bootstrap, &tick, &dastcom, &tycho]
        .iter()
        .filter(|s| diverges(s))
        .count();
    println!(
        "membrane-hull tally: {refuted} of 4 path(s) diverge from the resting cone beyond the star_dmax_probe tolerances"
    );
}

fn print_path(name: &str, stats: &PathStats, extra: &str) {
    let (rid, n) = stats.interdecile();
    println!(
        "PATH {name} | candidates {} | admit {} refuse {} | f_excl {:.6} f_inc {:.6} | interdecile {rid:.6} over {n} disagreement margins | {extra}",
        stats.candidates,
        stats.admit,
        stats.refuse,
        stats.f_excl(),
        stats.f_inc()
    );
    println!("VERDICT {name} | {}", verdict_word(stats));
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agreement_pushes_no_margins_and_zero_rates() {
        let mut st = PathStats::new();
        st.record(true, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0e10);
        st.record(false, [2.0e10, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0e10);
        assert_eq!(st.f_excl(), 0.0);
        assert_eq!(st.f_inc(), 0.0);
        assert_eq!(st.interdecile(), (1.0, 0));
    }

    #[test]
    fn overshoot_and_shortfall_count_as_disagreement() {
        let mut st = PathStats::new();
        st.record(true, [3.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0);
        st.record(false, [0.5, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0);
        assert_eq!(st.f_excl(), 1.0);
        assert_eq!(st.f_inc(), 0.5);
        let (rid, n) = st.interdecile();
        assert_eq!(n, 2);
        assert!(rid > 1.0);
    }

    #[test]
    fn the_cone_boundary_is_inclusive() {
        let mut st = PathStats::new();
        st.record(true, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0);
        assert_eq!(st.f_excl(), 0.0);
        assert_eq!(st.f_inc(), 0.0);
        assert!(st.interdecile().0 == 1.0);
    }
}
