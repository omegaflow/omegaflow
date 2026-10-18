use std::collections::BTreeMap;
use std::env;
use std::fs;

const DEFAULT_REGISTER: &str = "phi/sources.φ";

fn quantity_kind(force: &str, name: &str) -> Option<&'static str> {
    match force {
        "gravity" => match name {
            "saber_tpgpaltitude"
            | "copernicus_geopotential_height"
            | "srtm15_plus_opentopography_elevation_m"
            | "srtm15_plus_elevation_m" => Some("geometry"),
            "corot_logg" | "polarbase_logg" | "pastel_logg" => Some("source-parameter"),
            _ => None,
        },
        "geometry" => Some("geometry"),
        "acoustic" => match name {
            "noaa_dcdb_depth_m"
            | "kcdc_kascade_zenith_deg"
            | "kcdc_kascade_azimuth_deg"
            | "kcdc_grande_zenith_deg"
            | "kcdc_grande_azimuth_deg"
            | "bgr_infrasound_back_azimuth_deg" => Some("geometry"),
            _ => None,
        },
        "diffusion" => None,
        "advective" => match name {
            "soho_lasco_cme_pa_deg" | "soho_lasco_cme_width_deg" => Some("geometry"),
            _ => None,
        },
        "thermal" => match name {
            "rave_dr4_effective_temp_k"
            | "galah_dr3_effective_temp_k"
            | "rave_teff_k"
            | "exo_stellarhost_teff_k"
            | "corot_teff_k"
            | "polarbase_teff_k"
            | "pastel_teff_k"
            | "wd_teff_k"
            | "apogee_dr17_teff_rv"
            | "rave_dr6_teffs_sparv"
            | "rave_dr6_teff_irfm" => Some("source-parameter"),
            _ => None,
        },
        "seismic-body" | "seismic-surface" => {
            if name.ends_with("_depth")
                || name.ends_with("_depth_km")
                || name.contains("station_elevation")
            {
                Some("geometry")
            } else if name.contains("magnitude")
                || name.contains("_mag")
                || name.contains("scalar_moment")
                || name.contains("shear_velocity")
            {
                Some("source-parameter")
            } else {
                None
            }
        }
        _ => None,
    }
}

fn usage() {
    eprintln!("usage: p10_quantity_migrate [path] [--write]");
    eprintln!(
        "  report only, or --write to retag the P10 tranches (gravity/geometry/seismic, acoustic, diffusion, advective, thermal) field→quantity"
    );
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
                eprintln!("p10_quantity_migrate: unknown flag '{}'", other);
                usage();
                std::process::exit(2);
            }
            other => {
                if path == DEFAULT_REGISTER {
                    path = other.to_string();
                } else {
                    eprintln!("p10_quantity_migrate: multiple paths");
                    std::process::exit(2);
                }
            }
        }
        i += 1;
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("p10_quantity_migrate: read {}: {}", path, e);
            std::process::exit(2);
        }
    };

    let mut migrated: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut kept: BTreeMap<String, usize> = BTreeMap::new();
    let mut out = String::with_capacity(content.len());
    for line in content.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() >= 7
            && tokens[0] == "field"
            && matches!(
                tokens[4],
                "gravity"
                    | "geometry"
                    | "seismic-body"
                    | "seismic-surface"
                    | "acoustic"
                    | "diffusion"
                    | "advective"
                    | "thermal"
            )
        {
            let force = tokens[4];
            let name = tokens[2];
            match quantity_kind(force, name) {
                Some(kind) => {
                    let mut t = tokens.clone();
                    t[0] = "quantity";
                    t[4] = kind;
                    out.push_str(&t.join(" "));
                    out.push('\n');
                    *migrated
                        .entry((force.to_string(), kind.to_string()))
                        .or_default() += 1;
                    continue;
                }
                None => {
                    *kept.entry(force.to_string()).or_default() += 1;
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }

    println!("p10_quantity_migrate {path}:");
    for ((force, kind), count) in &migrated {
        println!("  migrated {force} × {kind}: {count}");
    }
    for (force, count) in &kept {
        println!("  kept {force}: {count}");
    }

    if write {
        if let Err(e) = fs::write(&path, &out) {
            eprintln!("p10_quantity_migrate: write {}: {}", path, e);
            std::process::exit(2);
        }
        println!("p10_quantity_migrate: {path} rewritten");
    }
}
