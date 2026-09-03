use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::rixs::{SpinBin, parse_spin_bin};

const RIXS_SPIN_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/rixs_spin.bin";
const MIN_N: usize = 30;
const CHANNELS: [(&str, &str); 3] = [
    ("Spin", "em"),
    ("Lattice", "acoustic"),
    ("Supercurrent", "electric"),
];

fn doping_name(d: u8) -> &'static str {
    match d {
        0 => "UD",
        1 => "OD1",
        2 => "OD2",
        _ => "?",
    }
}

fn load_spin(path: Option<String>) -> Option<SpinBin> {
    if let Some(p) = path {
        match std::fs::read(&p) {
            Ok(bytes) => match parse_spin_bin(&bytes) {
                Some(bin) => return Some(bin),
                None => {
                    eprintln!(
                        "rixs_cuprate_probe: {} parses void — the spin channel stays unmeasured",
                        p
                    );
                    return None;
                }
            },
            Err(_) => {
                eprintln!(
                    "rixs_cuprate_probe: {} reads void — the spin channel stays unmeasured",
                    p
                );
                return None;
            }
        }
    }
    match fetch_raw_bytes(RIXS_SPIN_CDN, 3600) {
        Some(bytes) => match parse_spin_bin(&bytes) {
            Some(bin) => Some(bin),
            None => {
                eprintln!("rixs_cuprate_probe: the CDN spin bin parses void");
                None
            }
        },
        None => {
            eprintln!(
                "rixs_cuprate_probe: {} carries no asset (0 honored)",
                RIXS_SPIN_CDN
            );
            None
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args
        .iter()
        .position(|a| a == "--bin")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let spin = load_spin(path);

    let spin_spectra = spin.as_ref().map(|s| s.spectra.len()).unwrap_or(0);
    let spin_osc: usize = spin
        .as_ref()
        .map(|s| s.spectra.iter().map(|sp| sp.oscillators.len()).sum())
        .unwrap_or(0);
    let doping_classes: std::collections::BTreeSet<u8> = spin
        .as_ref()
        .map(|s| s.spectra.iter().map(|sp| sp.doping).collect())
        .unwrap_or_default();
    let lab = spin.as_ref().and_then(|s| s.lab);

    println!("rixs_cuprate_probe — the Kuprat Blatt (doping axis)");
    println!(
        "  Spin channel: {} spectra, {} oscillators, {} dopings ({})",
        spin_spectra,
        spin_osc,
        doping_classes.len(),
        doping_classes
            .iter()
            .map(|&d| doping_name(d))
            .collect::<Vec<_>>()
            .join(","),
    );
    println!(
        "  anchor: {}",
        match lab {
            Some((lat, lon, alt)) => format!("lab {:.3} {:.3} {:.0}", lat, lon, alt),
            None => "pending".to_string(),
        }
    );
    println!("  channels:");
    for (name, force) in CHANNELS {
        let present = name == "Spin" && spin_spectra > 0;
        println!(
            "    {} ({}) — {}",
            name,
            force,
            if present {
                "harvested"
            } else {
                "unharvested — no statement"
            }
        );
    }
    println!(
        "  series axis: doping (UD/OD1/OD2), {} classes — MIN_N = {}",
        doping_classes.len(),
        MIN_N
    );

    println!("  TE matrix (driver → follower):");
    for (i, (dn, _)) in CHANNELS.iter().enumerate() {
        let mut row = String::new();
        for j in 0..CHANNELS.len() {
            let cell = if i == j { "itself" } else { "no statement" };
            if j > 0 {
                row.push_str(" | ");
            }
            row.push_str(&format!("{:12}", cell));
        }
        println!("    {} -> {}", dn, row);
    }

    let verdict = if spin_spectra == 0 {
        "no statement — the Spin channel carries no harvest"
    } else if doping_classes.len() < MIN_N {
        "no statement — the doping axis carries fewer series than MIN_N, and lattice/supercurrent are unharvested; silence is the finding"
    } else {
        "no statement — lattice and supercurrent are unharvested"
    };
    println!("  verdict: {}", verdict);
}
