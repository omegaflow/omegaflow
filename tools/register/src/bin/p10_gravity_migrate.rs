use std::env;
use std::fs;

const DEFAULT_REGISTER: &str = "phi/sources.φ";

const GEOMETRY: [&str; 26] = [
    "planet_radius",
    "venera15_16_surface_radius_km",
    "gracefo_kbr_inter_satellite_distance_m",
    "gracefo_kbr_range_rate_m_s",
    "gracefo_kbr_range_accl_m_s2",
    "gage_height_ft",
    "hydrosphere_river_stage_m",
    "sig_wave_height_m",
    "hydrosphere_ndbc_buoy_tide",
    "hydrosphere_tide_water_level_m",
    "tide_ft",
    "ndbc_dart_21414_height_m",
    "d20_thermocline_depth_m",
    "hydrosphere_sealevel_m",
    "uhslc_sea_level_mm",
    "swot_l2_lr_ssh_ssha_m",
    "gaia_rrl_parallax_mas",
    "gaia_dr2_opencluster_plx_mas",
    "decaps_dr2_parallax_arcsec",
    "charm2_ud_mas",
    "charm2_ld_mas",
    "jsdc_ldd_mas",
    "eop_iers_ut1_utc_s",
    "eop_iers_polar_motion_x_arcsec",
    "eop_iers_polar_motion_y_arcsec",
    "iers_eop_lod_s",
];

const SOURCE_PARAM: [&str; 5] = [
    "cb_primary_mass",
    "cb_secondary_mass",
    "lmxb_primary_mass",
    "lmxb_secondary_mass",
    "planet_mass",
];

const KEEP: [&str; 10] = [
    "agrav_gravity_ms2",
    "copernicus_geopotential_height",
    "igets_gravity_nm_s2",
    "viking_grav_acceleration_mm_s2",
    "lpf_drs_dg_x_ms2",
    "lpf_drs_dg_y_ms2",
    "lpf_drs_dg_z_ms2",
    "corot_logg",
    "polarbase_logg",
    "pastel_logg",
];

fn usage() {
    eprintln!("usage: p10_gravity_migrate [path] [--write]");
    eprintln!("  report only, or --write to apply the gravity→quantity line migration");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = String::from(DEFAULT_REGISTER);
    let mut write = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => write = true,
            "-h" | "--help" => {
                usage();
                return;
            }
            other if other.starts_with('-') => {
                eprintln!("p10_gravity_migrate: unknown flag '{}'", other);
                usage();
                std::process::exit(2);
            }
            other => {
                if path == DEFAULT_REGISTER {
                    path = other.to_string();
                } else {
                    eprintln!("p10_gravity_migrate: multiple paths");
                    std::process::exit(2);
                }
            }
        }
        i += 1;
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("p10_gravity_migrate: read {}: {}", path, e);
            std::process::exit(2);
        }
    };

    let mut geometry = 0usize;
    let mut source_param = 0usize;
    let mut keep = 0usize;
    let mut unclassified: Vec<String> = Vec::new();
    let mut histogram: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    let mut out = String::with_capacity(content.len());
    for line in content.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() >= 5 && tokens[0] == "field" && tokens[4] == "gravity" {
            let name = tokens[2];
            *histogram.entry(name.to_string()).or_default() += 1;
            if GEOMETRY.contains(&name) {
                let mut t = tokens.clone();
                t[0] = "quantity";
                t[4] = "geometry";
                out.push_str(&t.join(" "));
                out.push('\n');
                geometry += 1;
                continue;
            }
            if SOURCE_PARAM.contains(&name) {
                let mut t = tokens.clone();
                t[0] = "quantity";
                t[4] = "source-parameter";
                out.push_str(&t.join(" "));
                out.push('\n');
                source_param += 1;
                continue;
            }
            if KEEP.contains(&name) {
                keep += 1;
            } else {
                unclassified.push(line.trim().to_string());
            }
        }
        out.push_str(line);
        out.push('\n');
    }

    println!(
        "p10_gravity_migrate {}: {} geometry · {} source-parameter · {} gravity-kept · {} unclassified",
        path,
        geometry,
        source_param,
        keep,
        unclassified.len()
    );
    for line in &unclassified {
        println!("  unclassified: {}", line);
    }
    for (name, count) in &histogram {
        println!("  {} ×{}", name, count);
    }

    if write {
        if let Err(e) = fs::write(&path, &out) {
            eprintln!("p10_gravity_migrate: write {}: {}", path, e);
            std::process::exit(2);
        }
        println!("p10_gravity_migrate: {path} rewritten");
    }
}
