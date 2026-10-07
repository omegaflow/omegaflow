use omegaflow::archivar::eionet_cdr::{
    COMP_AIR, COMP_SOIL, COMP_WATER, POLLUTANTS, parse_report, series_name,
};
use omegaflow::archivar::geo::{magic_of, parse_bin, write_bin};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::process::Command;

const FORMAT: &str = "eionet_cdr";
const CDN_TAG: &str = "cdr.eionet.europa.eu";
const TTL_SECONDS: u64 = 31_536_000;
const MEDIA: [u32; 3] = [COMP_AIR, COMP_WATER, COMP_SOIL];

fn emit_field_names() {
    for (id, _, _) in POLLUTANTS {
        for medium in MEDIA {
            let comp = (*id << 2) | medium;
            if let Some(name) = series_name(comp) {
                println!("field {name} {name} point diffusion kg {TTL_SECONDS} 0.0 0.0");
            }
        }
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn positional(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if matches!(a.as_str(), "--in" | "--out") {
            i += 2;
            continue;
        }
        if !a.starts_with("--") {
            return Some(a.clone());
        }
        i += 1;
    }
    None
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSLf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("600")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "eionet_cdr: fetch {url} returned ({}) — the report stays pending",
            out.status
        );
        None
    }
}

fn read_source(source: &str) -> Result<Vec<u8>, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        curl_bytes(source).ok_or_else(|| format!("{source}: read void — the report stays pending"))
    } else {
        std::fs::read(source).map_err(|e| format!("{source}: read void ({e})"))
    }
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--emit-field-names") {
        emit_field_names();
        return Ok(());
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let source = arg_value(args, "--in")
        .or_else(|| positional(args))
        .ok_or_else(|| {
            "usage: eionet_cdr_compiler <xml|url> [--in <xml|url>] --out <bin> [--ci-mode]"
                .to_string()
        })?;
    let out = arg_value(args, "--out").ok_or_else(|| {
        "usage: eionet_cdr_compiler <xml|url> [--in <xml|url>] --out <bin> [--ci-mode]".to_string()
    })?;

    let bytes = read_source(&source)?;
    let mut records = parse_report(&bytes).map_err(|e| {
        format!(
            "{source}: {e} ({} B) — the bin stays unwritten (0 honored)",
            bytes.len()
        )
    })?;
    if records.is_empty() {
        return Err(format!(
            "{source}: no measured facility release left the harvest — the bin stays unwritten (0 honored)"
        ));
    }
    records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.lat.total_cmp(&b.lat))
            .then(a.lon.total_cmp(&b.lon))
            .then(a.comp.cmp(&b.comp))
    });

    let magic =
        magic_of(FORMAT).ok_or_else(|| format!("format {FORMAT} carries no geo::magic_of arm"))?;
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let bin = write_bin(magic, &records);
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;
    let parsed = parse_bin(magic, &bin)
        .ok_or_else(|| format!("{out}: roundtrip parse void — the asset stays unverified"))?;
    if parsed.len() != records.len() {
        return Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        ));
    }
    println!("origin {source}");
    println!("compiler tools/harvest/src/bin/eionet_cdr_compiler.rs");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} records from the E-PRTR/LCP report, {} B, roundtrip parses",
        parsed.len(),
        bin.len()
    );

    if ci_mode && !upload_release(CDN_TAG, &out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {CDN_TAG} release"
        ));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("eionet_cdr_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "\u{feff}<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>
<PollutantReleaseAndTransferReport xmlns=\"urn:eu:com:env:prtr:data:standard:2\">
  <ReportingYear>2017</ReportingYear>
  <FacilityReport>
    <GeographicalCoordinate>
      <LongitudeMeasure>12.788444</LongitudeMeasure>
      <LatitudeMeasure>47.283943</LatitudeMeasure>
    </GeographicalCoordinate>
    <PollutantRelease>
      <MediumCode>AIR</MediumCode>
      <PollutantCode>NOX</PollutantCode>
      <TotalQuantity unitCode=\"KGM\">372000</TotalQuantity>
    </PollutantRelease>
  </FacilityReport>
</PollutantReleaseAndTransferReport>";

    #[test]
    fn compiler_roundtrips_a_measured_fixture() {
        let records = parse_report(FIXTURE.as_bytes()).expect("fixture parses");
        assert_eq!(records.len(), 1);
        let magic = magic_of(FORMAT).expect("format registered");
        let bin = write_bin(magic, &records);
        let parsed = parse_bin(magic, &bin).expect("roundtrip");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].val, 372000.0);
        assert_eq!(parsed[0].lat, 47.283943);
        assert_eq!(parsed[0].lon, 12.788444);
        assert_eq!(
            omegaflow::archivar::eionet_cdr::pollutant_of(parsed[0].comp),
            Some("NOX")
        );
    }

    #[test]
    fn a_body_without_the_report_root_is_void() {
        assert!(parse_report(b"<html></html>").is_err());
    }
}
