use omegaflow::archivar::double::{crossmatch, ConeCatalog};
use omegaflow::archivar::exclude::parse_bin as parse_excl;
use omegaflow::archivar::ir::parse_bin as parse_ir;
use omegaflow::archivar::radio::parse_bin as parse_radio;
use omegaflow::archivar::spatial::parse_star_record;
use omegaflow::archivar::spatial::STAR_RECORD_BYTES;
use omegaflow::archivar::tns::parse_bin as parse_tns;
use std::path::Path;

const MATCH_RADIUS_DEG: f64 = 0.01;
const IR_EXCESS_THRESHOLD_MAG: f64 = -0.5;

fn name_of(n: &[u8; 32]) -> String {
    let end = n.iter().position(|&x| x == 0).unwrap_or(32);
    String::from_utf8_lossy(&n[..end]).to_string()
}

fn run(
    ir_path: &str,
    stars_path: &str,
    radio_path: &str,
    tns_path: &str,
    excl_path: &str,
    out_path: &str,
    only_excess: bool,
) -> Result<(), String> {
    let ir_bytes = std::fs::read(ir_path).map_err(|e| format!("ir {ir_path}: {e}"))?;
    let ir = parse_ir(&ir_bytes).ok_or("ir.bin: no IR1X contract")?;

    let mut gaia_ra = Vec::new();
    let mut gaia_dec = Vec::new();
    let mut gaia_color = Vec::new();
    let sb = std::fs::read(stars_path).map_err(|e| format!("stars {stars_path}: {e}"))?;
    for chunk in sb.chunks_exact(STAR_RECORD_BYTES) {
        if let Some(rec) = parse_star_record(chunk) {
            gaia_ra.push(rec.ra_deg);
            gaia_dec.push(rec.dec_deg);
            gaia_color.push(rec.color_index);
        }
    }
    let gaia = ConeCatalog::with_values(gaia_ra, gaia_dec, gaia_color);
    eprintln!("gaia stars loaded: {}", gaia.len());

    let mut radio_ra = Vec::new();
    let mut radio_dec = Vec::new();
    let mut radio_flux = Vec::new();
    if Path::new(radio_path).exists() {
        let rb = std::fs::read(radio_path).map_err(|e| format!("radio {radio_path}: {e}"))?;
        for s in parse_radio(&rb).unwrap_or_default() {
            radio_ra.push(s.ra_deg);
            radio_dec.push(s.dec_deg);
            radio_flux.push(s.flux);
        }
    }
    let radio = ConeCatalog::with_values(radio_ra, radio_dec, radio_flux);
    eprintln!("radio sources loaded: {}", radio.len());

    let mut tns_ra = Vec::new();
    let mut tns_dec = Vec::new();
    let mut tns_z = Vec::new();
    if Path::new(tns_path).exists() {
        let tb = std::fs::read(tns_path).map_err(|e| format!("tns {tns_path}: {e}"))?;
        for o in parse_tns(&tb).unwrap_or_default() {
            tns_ra.push(o.ra_deg);
            tns_dec.push(o.dec_deg);
            tns_z.push(o.z);
        }
    }
    let tns = ConeCatalog::with_values(tns_ra, tns_dec, tns_z);
    eprintln!("tns objects loaded: {}", tns.len());

    let mut excl_ra = Vec::new();
    let mut excl_dec = Vec::new();
    let mut excl_name = Vec::new();
    if Path::new(excl_path).exists() {
        let eb = std::fs::read(excl_path).map_err(|e| format!("excl {excl_path}: {e}"))?;
        for r in parse_excl(&eb).unwrap_or_default() {
            excl_ra.push(r.ra_deg);
            excl_dec.push(r.dec_deg);
            excl_name.push(r.name);
        }
    }
    let excl = ConeCatalog::with_names(excl_ra, excl_dec, excl_name);
    eprintln!("exclusion rows loaded: {}", excl.len());

    let rows = crossmatch(
        &ir.iter().map(|s| s.ra_deg).collect::<Vec<_>>(),
        &ir.iter().map(|s| s.dec_deg).collect::<Vec<_>>(),
        &ir.iter().map(|s| s.excess).collect::<Vec<_>>(),
        &gaia,
        &radio,
        &tns,
        &excl,
        MATCH_RADIUS_DEG,
        only_excess,
    );

    let excluded = rows.iter().filter(|r| r.excluded).count();
    let remaining = rows.len() - excluded;
    let mut out = String::new();
    out.push_str("Doppel-Anomalie catalog — Verdict\n");
    out.push_str("OA = (Variability_observed − Variability_expected)/Chromativity\n");
    out.push_str(&format!(
        "IR-excess threshold: W3−W4 < {:.1} mag | Match radius: {:.2}°\n",
        IR_EXCESS_THRESHOLD_MAG, MATCH_RADIUS_DEG
    ));
    out.push_str(&format!(
        "Inputs: ir={} gaia_stars={} radio={} tns={} excl={}\n",
        ir.len(),
        gaia.len(),
        radio.len(),
        tns.len(),
        excl.len()
    ));
    out.push_str(&format!(
        "Mode: {}\n",
        if only_excess {
            "IR-excess candidates only"
        } else {
            "full sky-sweep (all ir positions)"
        }
    ));
    out.push_str(&format!(
        "Rows: {} | Hephaistos-excluded: {} | remaining: {}\n",
        rows.len(),
        excluded,
        remaining
    ));
    for r in &rows {
        out.push_str(&format!(
            "  ra {:.4}  dec {:.4}  W3−W4 {:.3}  gaia_color {:.3}  radio {:.3e}  tns_z {:.4}  {}\n",
            r.ra_deg,
            r.dec_deg,
            r.excess_mag,
            r.gaia_color,
            r.radio_flux,
            r.tns_z,
            if r.excluded {
                format!("EXCLUDED (VSX/GCVS/Exoplanet: {})", name_of(&r.ex_name))
            } else {
                "candidate".to_string()
            }
        ));
    }
    if rows.is_empty() {
        out.push_str("  (no rows — the negative result is a quantitative limit, 0 honored)\n");
    }
    std::fs::write(out_path, &out).map_err(|e| format!("{out_path}: {e}"))?;
    println!("{out}");
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut ir = "/tmp/opencode/ir.bin".to_string();
    let mut stars = "/tmp/opencode/dr3_stars.bin".to_string();
    let mut radio = "/tmp/opencode/radio.bin".to_string();
    let mut tns = "/tmp/opencode/tns.bin".to_string();
    let mut excl = "/tmp/opencode/exclude.bin".to_string();
    let mut out = "/tmp/opencode/double_anomaly.txt".to_string();
    let mut only_excess = true;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--ir" => {
                i += 1;
                ir = args.get(i).cloned().unwrap_or(ir);
            }
            "--stars" => {
                i += 1;
                stars = args.get(i).cloned().unwrap_or(stars);
            }
            "--radio" => {
                i += 1;
                radio = args.get(i).cloned().unwrap_or(radio);
            }
            "--tns" => {
                i += 1;
                tns = args.get(i).cloned().unwrap_or(tns);
            }
            "--excl" => {
                i += 1;
                excl = args.get(i).cloned().unwrap_or(excl);
            }
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(out);
            }
            "--all" => only_excess = false,
            other => {
                eprintln!("infrared_anomaly_compiler: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if let Err(msg) = run(&ir, &stars, &radio, &tns, &excl, &out, only_excess) {
        eprintln!("infrared_anomaly_compiler: {msg}");
        std::process::exit(1);
    }
}
