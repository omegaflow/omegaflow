use std::collections::BTreeMap;

use omegaflow::bsp_reader::spk::SpkFile;

const AU_KM: f64 = 149_597_870.7;
const J2000_EPOCH: f64 = 2_451_545.0;
const SECONDS_PER_DAY: f64 = 86_400.0;

struct TestPoint {
    jed: f64,
    target: i32,
    center: i32,
    coord: usize,
    value: f64,
}

fn kc_to_naif(kc: i32) -> Option<i32> {
    match kc {
        1 => Some(1),    // mercury barycenter
        2 => Some(2),    // venus barycenter
        3 => Some(3),    // earth-moon barycenter
        4 => Some(4),    // mars barycenter
        5 => Some(5),    // jupiter barycenter
        6 => Some(6),    // saturn barycenter
        7 => Some(7),    // uranus barycenter
        8 => Some(8),    // neptune barycenter
        9 => Some(9),    // pluto barycenter
        10 => Some(301), // moon
        11 => Some(10),  // sun
        12 => Some(0),   // solar-system barycenter
        13 => Some(399), // earth geocenter
        _ => None,
    }
}

fn kc_name(kc: i32) -> &'static str {
    match kc {
        1 => "mercury",
        2 => "venus",
        3 => "emb",
        4 => "mars",
        5 => "jupiter",
        6 => "saturn",
        7 => "uranus",
        8 => "neptune",
        9 => "pluto",
        10 => "moon",
        11 => "sun",
        12 => "ssb",
        13 => "earth",
        _ => "unknown",
    }
}

fn parse_testpo(text: &str) -> Vec<TestPoint> {
    let mut points = Vec::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() != 7 {
            continue;
        }
        let jed = match fields[2].parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let (Ok(target), Ok(center), Ok(coord)) = (
            fields[3].parse::<i32>(),
            fields[4].parse::<i32>(),
            fields[5].parse::<i32>(),
        ) else {
            continue;
        };
        if !(1..=6).contains(&coord) {
            continue;
        }
        let Ok(value) = fields[6].parse::<f64>() else {
            continue;
        };
        points.push(TestPoint {
            jed,
            target,
            center,
            coord: coord as usize,
            value,
        });
    }
    points
}

fn usage() {
    eprintln!("usage: inpop_testpo_probe --bsp <inpop.spk> --testpo <testpo.INPOP19A_TDB>");
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

struct BodyStat {
    count: usize,
    skipped: usize,
    max_pos_diff_m: f64,
    max_vel_diff_mm_s: f64,
    max_pos_jed: f64,
    max_pos_kc_target: i32,
    max_pos_kc_center: i32,
    max_vel_jed: f64,
    max_vel_kc_target: i32,
    max_vel_kc_center: i32,
}

impl BodyStat {
    fn new() -> BodyStat {
        BodyStat {
            count: 0,
            skipped: 0,
            max_pos_diff_m: 0.0,
            max_vel_diff_mm_s: 0.0,
            max_pos_jed: 0.0,
            max_pos_kc_target: 0,
            max_pos_kc_center: 0,
            max_vel_jed: 0.0,
            max_vel_kc_target: 0,
            max_vel_kc_center: 0,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let Some(bsp_path) = arg_value(&args, "--bsp") else {
        eprintln!("inpop-testpo: --bsp <inpop.spk> absent");
        return;
    };
    let Some(testpo_path) = arg_value(&args, "--testpo") else {
        eprintln!("inpop-testpo: --testpo <testpo.INPOP19A_TDB> absent");
        return;
    };

    let spk = match SpkFile::open(&bsp_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("inpop-testpo: {bsp_path} does not open as SPK: {e}");
            return;
        }
    };

    println!("inpop-testpo: SPK {bsp_path}");
    println!("inpop-testpo: {} segment(s):", spk.segments().len());
    for seg in spk.segments() {
        println!(
            "  target {} center {} frame {} type {} et [{:.1}, {:.1}] name {:?}",
            seg.target, seg.center, seg.frame, seg.data_type, seg.start_et, seg.end_et, seg.name
        );
    }

    let testpo_text = match std::fs::read_to_string(&testpo_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("inpop-testpo: {testpo_path} read: {e}");
            return;
        }
    };
    let points = parse_testpo(&testpo_text);
    if points.is_empty() {
        eprintln!("inpop-testpo: {testpo_path} carries no test point (7-field) line");
        return;
    }
    let jed_lo = points.iter().map(|p| p.jed).fold(f64::INFINITY, f64::min);
    let jed_hi = points
        .iter()
        .map(|p| p.jed)
        .fold(f64::NEG_INFINITY, f64::max);
    println!(
        "inpop-testpo: {testpo_path} reads {} test point(s) | jed {:.1} .. {:.1}",
        points.len(),
        jed_lo,
        jed_hi
    );

    let mut stats: BTreeMap<i32, BodyStat> = BTreeMap::new();
    let mut unmapped = 0usize;
    for p in &points {
        let (Some(naif_target), Some(naif_center)) = (kc_to_naif(p.target), kc_to_naif(p.center))
        else {
            unmapped += 1;
            continue;
        };
        let et = (p.jed - J2000_EPOCH) * SECONDS_PER_DAY;
        let state = match spk.state(naif_target, naif_center, et) {
            Ok(s) => s,
            Err(_) => {
                stats.entry(p.target).or_insert_with(BodyStat::new).skipped += 1;
                continue;
            }
        };
        let computed = if p.coord <= 3 {
            state[p.coord - 1] / AU_KM
        } else {
            state[p.coord - 1] * SECONDS_PER_DAY / AU_KM
        };
        let diff = computed - p.value;
        let st = stats.entry(p.target).or_insert_with(BodyStat::new);
        st.count += 1;
        if p.coord <= 3 {
            let diff_m = diff * AU_KM * 1000.0;
            if diff_m.abs() > st.max_pos_diff_m {
                st.max_pos_diff_m = diff_m.abs();
                st.max_pos_jed = p.jed;
                st.max_pos_kc_target = p.target;
                st.max_pos_kc_center = p.center;
            }
        } else {
            let diff_mm_s = diff * AU_KM * 1000.0 / SECONDS_PER_DAY;
            if diff_mm_s.abs() > st.max_vel_diff_mm_s {
                st.max_vel_diff_mm_s = diff_mm_s.abs();
                st.max_vel_jed = p.jed;
                st.max_vel_kc_target = p.target;
                st.max_vel_kc_center = p.center;
            }
        }
    }

    let mut global_max_pos = 0.0f64;
    let mut global_max_vel = 0.0f64;
    for st in stats.values() {
        if st.max_pos_diff_m > global_max_pos {
            global_max_pos = st.max_pos_diff_m;
        }
        if st.max_vel_diff_mm_s > global_max_vel {
            global_max_vel = st.max_vel_diff_mm_s;
        }
    }

    println!();
    println!("inpop-testpo: per-body measured divergence (INPOP SPK vs INPOP testpo reference):");
    for (kc, st) in &stats {
        println!(
            "  {:<7} points {:>5} skipped {:>4} | max |pos| {:.3e} m (jed {:.1}, {}-wrt-{}) | max |vel| {:.3e} mm/s (jed {:.1}, {}-wrt-{})",
            kc_name(*kc),
            st.count,
            st.skipped,
            st.max_pos_diff_m,
            st.max_pos_jed,
            kc_name(st.max_pos_kc_target),
            kc_name(st.max_pos_kc_center),
            st.max_vel_diff_mm_s,
            st.max_vel_jed,
            kc_name(st.max_vel_kc_target),
            kc_name(st.max_vel_kc_center),
        );
    }
    println!();
    println!(
        "inpop-testpo: {} test point(s) judged across {} body/bodies | {} unmapped kc field(s)",
        points.len() - unmapped,
        stats.len(),
        unmapped
    );
    println!(
        "inpop-testpo verdict: max |pos| {:.3e} m | max |vel| {:.3e} mm/s — the INPOP SPK reproduces the IMCCE testpo reference {}",
        global_max_pos,
        global_max_vel,
        if global_max_pos < 1.0 {
            "to sub-metre"
        } else if global_max_pos < 1000.0 {
            "to sub-kilometre"
        } else {
            "with a kilometre-scale rift"
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_skips_header_and_reads_data_lines() {
        let text = "INPOP19A_TDB\nKSIZE= 1876\n\nINPOP#\t -- date -- -- jed -- t# c# x# -- coordinate ---\nEOT\n100  1899.01.26 2414680.5  2  1  5       -0.00663610218463795733\n";
        let points = parse_testpo(text);
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].jed, 2414680.5);
        assert_eq!(points[0].target, 2);
        assert_eq!(points[0].center, 1);
        assert_eq!(points[0].coord, 5);
    }

    #[test]
    fn kc_mapping_covers_the_thirteen_components() {
        assert_eq!(kc_to_naif(1), Some(1));
        assert_eq!(kc_to_naif(3), Some(3));
        assert_eq!(kc_to_naif(10), Some(301));
        assert_eq!(kc_to_naif(11), Some(10));
        assert_eq!(kc_to_naif(12), Some(0));
        assert_eq!(kc_to_naif(13), Some(399));
        assert_eq!(kc_to_naif(99), None);
    }
}
