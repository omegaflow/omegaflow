use std::env;
use std::path::Path;
use std::process::exit;

use omegaflow_measure::idl_save;

fn run(input: &str, out_elev: &str, out_azim: &str) -> Result<(), String> {
    let skymap = idl_save::load_skymap(Path::new(input))?;
    idl_save::emit(input)?;
    idl_save::write_values(out_elev, &skymap.full_elevation)?;
    idl_save::write_values(out_azim, &skymap.full_azimuth)?;
    println!(
        "FULL_ELEVATION -> {} ({} values)",
        out_elev,
        skymap.full_elevation.len()
    );
    println!(
        "FULL_AZIMUTH -> {} ({} values)",
        out_azim,
        skymap.full_azimuth.len()
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        Some(s) => s.clone(),
        None => "state/river/themis/kapu_20240909_skymap.sav".to_string(),
    };
    let out_elev = match args.get(2) {
        Some(s) => s.clone(),
        None => "state/river/themis/kapu_20240909_full_elevation.txt".to_string(),
    };
    let out_azim = match args.get(3) {
        Some(s) => s.clone(),
        None => "state/river/themis/kapu_20240909_full_azimuth.txt".to_string(),
    };
    if let Err(detail) = run(&input, &out_elev, &out_azim) {
        eprintln!("idl_sav_dump: {}", detail);
        exit(2);
    }
}
