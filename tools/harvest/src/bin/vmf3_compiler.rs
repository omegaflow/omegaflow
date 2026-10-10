use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::vmf3::{self, SiteMf};
use omegaflow::cdn::upload_release;
use std::collections::BTreeSet;

const NETLOC: &str = "vmf.geo.tuwien.ac.at";
const COMPILER: &str = "tools/harvest/src/bin/vmf3_compiler.rs";
const FORMAT: &str = "vmf3";

const SITE_SAMPLE: &str = "AGGO      61042.00  0.00126424  0.00043626  2.2940  0.1199  1005.73  22.79  16.62\n\
AIRA      61042.00  0.00121746  0.00041399  2.2444  0.0500   983.52   1.96   5.46\n";

const GRID_SAMPLE: &str = "! Version:            1.0\n\
! Epoch:              2026 01 01 00 00  0.0\n\
! Scale_factor:       1.e+00\n\
 89.5    0.5  0.00114741  0.00069930  2.2862  0.0158\n\
 89.5    1.5  0.00114740  0.00069526  2.2861  0.0158\n";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn write_out(out: &str, bin: &[u8]) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(out, bin).map_err(|e| format!("write {out} void: {e}"))
}

fn print_sources(input: &str, stem: &str, bin: &[u8]) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{stem}.bin");
    println!("origin {input}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(bin));
}

fn run_site(input: &str, out: &str, bytes: &[u8], ci_mode: bool) -> Result<(), String> {
    let records = vmf3::parse_site(bytes)
        .ok_or_else(|| format!("{input}: carries no VMF3 site row — refused"))?;
    let total = records.len();
    let mut named: Vec<SiteMf> = records
        .into_iter()
        .filter(|r| {
            r.mjd.is_finite() && r.ah.is_finite() && r.aw.is_finite() && r.zhd_m.is_finite()
        })
        .collect();
    let held = total - named.len();
    if held > 0 {
        eprintln!("{input}: {held} site rows carry no finite core value — held out, named");
    }
    named.sort_by(|a, b| a.station.cmp(&b.station).then(a.mjd.total_cmp(&b.mjd)));

    let bin = vmf3::write_bin(&named)
        .ok_or_else(|| "a held site value is not finite — the bin stays unwritten".to_string())?;
    match vmf3::parse_bin(&bin) {
        Some(parsed) if parsed == named => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }
    write_out(out, &bin)?;

    let stations: BTreeSet<&str> = named.iter().map(|r| r.station.as_str()).collect();
    let first = named.first().map(|r| r.mjd);
    let last = named.last().map(|r| r.mjd);
    print_sources(input, "vmf3_site", &bin);
    eprintln!(
        "{out}: {} site rows, {} stations, mjd {first:?}..{last:?}, {} B, roundtrip parses",
        named.len(),
        stations.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn run_grid(input: &str, out: &str, bytes: &[u8], ci_mode: bool) -> Result<(), String> {
    let cells = vmf3::parse_grid(bytes)
        .ok_or_else(|| format!("{input}: carries no VMF3 grid cell — refused"))?;
    let bin = vmf3::write_grid_bin(&cells)
        .ok_or_else(|| "a held grid value is not finite — the bin stays unwritten".to_string())?;
    match vmf3::parse_grid_bin(&bin) {
        Some(parsed) if parsed == cells => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }
    write_out(out, &bin)?;

    print_sources(input, "vmf3_grid", &bin);
    match vmf3::parse_grid_header(bytes) {
        Some(h) => eprintln!(
            "{out}: {} grid cells, epoch {}-{:02}-{:02}T{:02}:{:02}:{:06.3}, scale {}, {} B, roundtrip parses",
            cells.len(),
            h.epoch_year,
            h.epoch_month,
            h.epoch_day,
            h.epoch_hour,
            h.epoch_minute,
            h.epoch_second,
            h.scale_factor,
            bin.len()
        ),
        None => eprintln!(
            "{out}: {} grid cells, epoch unnamed, {} B, roundtrip parses",
            cells.len(),
            bin.len()
        ),
    }

    if ci_mode && !upload_release(NETLOC, out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let grid = args.iter().any(|a| a == "--grid");
    let input = arg_value(args, "--input")
        .ok_or_else(|| "--input <file> absent — the file stays unread".to_string())?;
    let stem = if grid { "vmf3_grid" } else { "vmf3_site" };
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{stem}.bin"),
    };

    let bytes = std::fs::read(&input).map_err(|e| format!("{input}: read void ({e})"))?;
    if grid {
        run_grid(&input, &out, &bytes, ci_mode)
    } else {
        run_site(&input, &out, &bytes, ci_mode)
    }
}

fn selftest() {
    let site = vmf3::parse_site(SITE_SAMPLE.as_bytes());
    let Some(site) = site else {
        eprintln!("selftest: the site sample does not parse");
        std::process::exit(1);
    };
    if site.len() != 2 {
        eprintln!("selftest: the site sample does not carry two rows");
        std::process::exit(1);
    }
    let Some(bin) = vmf3::write_bin(&site) else {
        eprintln!("selftest: site write_bin void");
        std::process::exit(1);
    };
    if bin.len() != vmf3::HEADER_BYTES + 2 * vmf3::RECORD_BYTES {
        eprintln!("selftest: the site bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if vmf3::parse_bin(&bin) != Some(site) {
        eprintln!("selftest: the site roundtrip does not read back");
        std::process::exit(1);
    }

    let grid = vmf3::parse_grid(GRID_SAMPLE.as_bytes());
    let Some(grid) = grid else {
        eprintln!("selftest: the grid sample does not parse");
        std::process::exit(1);
    };
    let Some(gbin) = vmf3::write_grid_bin(&grid) else {
        eprintln!("selftest: grid write_grid_bin void");
        std::process::exit(1);
    };
    if gbin.len() != vmf3::HEADER_BYTES + 2 * vmf3::GRID_RECORD_BYTES {
        eprintln!("selftest: the grid bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if vmf3::parse_grid_bin(&gbin) != Some(grid) {
        eprintln!("selftest: the grid roundtrip does not read back");
        std::process::exit(1);
    }
    if vmf3::parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated site bin reads back");
        std::process::exit(1);
    }
    eprintln!("vmf3_compiler: selftest passes (VMF3 site and grid products → bin round-trip)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: vmf3_compiler --input <file> [--out <file.bin>] [--grid] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("vmf3_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_sample_roundtrips_the_measured_stride() {
        let rows = vmf3::parse_site(SITE_SAMPLE.as_bytes()).expect("the site sample parses");
        let bin = vmf3::write_bin(&rows).expect("finite site rows encode");
        assert_eq!(bin.len(), vmf3::HEADER_BYTES + 2 * vmf3::RECORD_BYTES);
        assert_eq!(vmf3::parse_bin(&bin), Some(rows));
    }

    #[test]
    fn grid_sample_roundtrips_the_measured_stride() {
        let cells = vmf3::parse_grid(GRID_SAMPLE.as_bytes()).expect("the grid sample parses");
        let bin = vmf3::write_grid_bin(&cells).expect("finite grid cells encode");
        assert_eq!(bin.len(), vmf3::HEADER_BYTES + 2 * vmf3::GRID_RECORD_BYTES);
        assert_eq!(vmf3::parse_grid_bin(&bin), Some(cells));
    }
}
