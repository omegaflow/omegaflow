use omegaflow::archivar::inflate::gunzip;
use omegaflow::archivar::itrf_sinex::{self, Station};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "itrf.ign.fr";
const COMPILER: &str = "tools/harvest/src/bin/itrf_sinex_compiler.rs";
const FORMAT: &str = "sinex";

const SAMPLE: &str = r#"%=SNX 2.02 TEST 22:001:00000 TEST 79:329:00000 21:003:00000 C 01224 2 X V  
*-------------------------------------------------------------------------------
+SITE/ID
*CODE PT __DOMES__ T _STATION DESCRIPTION__ APPROX_LON_ APPROX_LAT_ _APP_H_
 7203  A 14209S001   EFLSBERG Effelsberg, G   6 53 01.0  50 31 29.4   416.8
-SITE/ID
*-------------------------------------------------------------------------------
+SOLUTION/ESTIMATE
*INDEX TYPE__ CODE PT SOLN _REF_EPOCH__ UNIT S __ESTIMATED VALUE____ _STD_DEV___
     1 STAX   7203  A    1 15:001:00000 m    2 0.403394728676361E+07 0.92450E-02
     2 STAY   7203  A    1 15:001:00000 m    2 0.486990823348219E+06 0.38376E-02
     3 STAZ   7203  A    1 15:001:00000 m    2 0.490043108394323E+07 0.10455E-01
     4 VELX   7203  A    1 15:001:00000 m/y  2 -.139698991002166E-01 0.33955E-03
     5 VELY   7203  A    1 15:001:00000 m/y  2 0.169886132211163E-01 0.15577E-03
     6 VELZ   7203  A    1 15:001:00000 m/y  2 0.107017114287353E-01 0.39626E-03
-SOLUTION/ESTIMATE
%ENDSNX
"#;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_xyz(station: &Station) -> bool {
    match (station.x_m, station.y_m, station.z_m) {
        (Some(x), Some(y), Some(z)) => x.is_finite() && y.is_finite() && z.is_finite(),
        _ => false,
    }
}

fn measured(stations: Vec<Station>) -> (Vec<Station>, usize) {
    let total = stations.len();
    let named: Vec<Station> = stations.into_iter().filter(has_xyz).collect();
    let held = total - named.len();
    (named, held)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let input = arg_value(args, "--input")
        .ok_or_else(|| "--input <sinex.snx|.ssc|.gz> absent — the file stays unread".to_string())?;
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{FORMAT}.bin"),
    };

    let raw = std::fs::read(&input).map_err(|e| format!("{input}: read void ({e})"))?;
    let bytes = if input.ends_with(".gz") {
        gunzip(&raw).ok_or_else(|| format!("{input}: the gzip stream carries no inflate"))?
    } else {
        raw
    };
    let stations = itrf_sinex::parse_sinex(&bytes)
        .ok_or_else(|| format!("{input}: carries no %=SNX marker — refused"))?;
    let (mut named, held) = measured(stations);
    if held > 0 {
        eprintln!("{input}: {held} stations carry no finite XYZ — held out, named");
    }
    if named.is_empty() {
        return Err(format!(
            "{input}: no station carries a finite XYZ — the bin stays unwritten (0 honored)"
        ));
    }
    named.sort_by(|a, b| a.code.cmp(&b.code).then_with(|| a.point.cmp(&b.point)));

    let bin = itrf_sinex::write_bin(&named)
        .ok_or_else(|| "a held value is not finite — the bin stays unwritten".to_string())?;
    match itrf_sinex::parse_bin(&bin) {
        Some(parsed) if parsed == named => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;

    let with_velocity = named
        .iter()
        .filter(|s| s.vx_m_y.is_some() && s.vy_m_y.is_some() && s.vz_m_y.is_some())
        .count();

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {input}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} stations, {with_velocity} with a full velocity, {held} held out, {} B, roundtrip parses",
        named.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let Some(stations) = itrf_sinex::parse_sinex(SAMPLE.as_bytes()) else {
        eprintln!("selftest: the sample SINEX does not parse");
        std::process::exit(1);
    };
    if stations.len() != 1 {
        eprintln!("selftest: the sample does not carry exactly one station");
        std::process::exit(1);
    }
    let Some(bin) = itrf_sinex::write_bin(&stations) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    if bin.len() != itrf_sinex::HEADER_BYTES + itrf_sinex::RECORD_BYTES {
        eprintln!("selftest: the bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if itrf_sinex::parse_bin(&bin) != Some(stations.clone()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if itrf_sinex::parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    eprintln!(
        "itrf_sinex_compiler: selftest passes (SINEX SITE/ID + SOLUTION/ESTIMATE → Station bin)"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: itrf_sinex_compiler --input <sinex.snx|.ssc|.gz> [--out <file.bin>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("itrf_sinex_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_roundtrips_the_measured_stride() {
        let stations = itrf_sinex::parse_sinex(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = itrf_sinex::write_bin(&stations).expect("the station encodes");
        assert_eq!(
            bin.len(),
            itrf_sinex::HEADER_BYTES + itrf_sinex::RECORD_BYTES
        );
        assert_eq!(itrf_sinex::parse_bin(&bin), Some(stations));
    }

    #[test]
    fn parse_sinex_refuses_foreign_bytes() {
        assert_eq!(itrf_sinex::parse_sinex(b"not a sinex file"), None);
    }
}
