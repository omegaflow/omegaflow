use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::s2event::{
    encode_rec, write_header, S2EventRecord, REC_BYTES, ROOT_NEUTRINO,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::LeapSeconds;
use std::path::PathBuf;

const TAG: &str = "dataverse.harvard.edu";
const MJD_UNIX_EPOCH: f64 = 40587.0;

const FILE_IDS: [u64; 14] = [
    13597946,
    13597981,
    13597928,
    13597936,
    13597919,
    13597943,
    13597923,
    13597957,
    13597970,
    13597929,
    13597916,
    13597930,
    13597971,
    13597931,
];

#[derive(Clone, Copy)]
struct Cols {
    ra: usize,
    dec: usize,
    ang: usize,
    mjd: Option<usize>,
    log_e: Option<usize>,
}

fn header_cols(line: &str) -> Option<Cols> {
    let t = line.trim().trim_start_matches('#');
    let names: Vec<&str> = t.split_whitespace().collect();
    let find = |p: &str| names.iter().position(|n| n.starts_with(p));
    Some(Cols {
        ra: find("RA")?,
        dec: find("Dec")?,
        ang: find("AngErr")?,
        mjd: find("MJD"),
        log_e: find("log10"),
    })
}

fn parse_text(text: &str, lsk: &LeapSeconds, out: &mut Vec<S2EventRecord>) -> usize {
    let mut cols: Option<Cols> = None;
    let mut taken = 0usize;
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with('#') {
            if cols.is_none() && t.contains("RA") && t.contains("Dec") && t.contains("AngErr") {
                cols = header_cols(t);
            }
            continue;
        }
        let Some(c) = cols else { continue };
        let cells: Vec<&str> = t.trim_matches('"').split_whitespace().collect();
        let num = |i: usize| {
            cells
                .get(i)
                .and_then(|s| s.trim_matches('"').parse::<f64>().ok())
        };
        let (Some(ra), Some(dec), Some(ang)) = (num(c.ra), num(c.dec), num(c.ang)) else {
            continue;
        };
        if !(ra.is_finite() && dec.is_finite() && ang.is_finite()) {
            continue;
        }
        if !(0.0..360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) || !(ang > 0.0) {
            continue;
        }
        let epoch_tdb = c
            .mjd
            .and_then(num)
            .and_then(|mjd| lsk.unix_to_tdb((mjd - MJD_UNIX_EPOCH) * 86400.0));
        let energy = c
            .log_e
            .and_then(num)
            .map(|v| 10f64.powf(v))
            .filter(|v| v.is_finite() && *v > 0.0);
        out.push(S2EventRecord {
            ra_deg: ra as f32,
            dec_deg: dec as f32,
            sigma_arcsec: Some(ang * 3600.0),
            epoch_tdb,
            energy,
            signalness: None,
            far: None,
            particle_root: ROOT_NEUTRINO,
        });
        taken += 1;
    }
    taken
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(id: u64) -> Option<String> {
    let url = format!("https://dataverse.harvard.edu/api/access/datafile/{id}");
    let out = std::process::Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("900")
        .arg(&url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        None
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let out_path = arg_value(args, "--out")
        .ok_or("usage: icecube_compiler --out <events.s2e1> [--in <dir|file>] [--ci-mode] — refused")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let lsk = embedded_lsk()
        .ok_or("the embedded leap-second table stays unread — MJD epochs stay unassigned")?;

    let mut records: Vec<S2EventRecord> = Vec::new();
    let mut rows = 0usize;
    match arg_value(args, "--in") {
        Some(p) => {
            let meta = std::fs::metadata(&p).map_err(|e| format!("stat {p} returned void: {e}"))?;
            let files: Vec<PathBuf> = if meta.is_dir() {
                let mut v: Vec<PathBuf> = std::fs::read_dir(&p)
                    .map_err(|e| format!("read_dir {p} returned void: {e}"))?
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.extension().is_some_and(|x| x == "tab"))
                    .collect();
                v.sort();
                v
            } else {
                vec![PathBuf::from(&p)]
            };
            if files.is_empty() {
                return Err(format!("{p}: no .tab event table — the asset stays unwritten"));
            }
            for f in files {
                let text = std::fs::read_to_string(&f)
                    .map_err(|e| format!("read {} returned void: {e}", f.display()))?;
                rows += parse_text(&text, &lsk, &mut records);
            }
        }
        None => {
            for id in FILE_IDS {
                let text = fetch(id).ok_or(format!(
                    "https://dataverse.harvard.edu/api/access/datafile/{id} carries void"
                ))?;
                rows += parse_text(&text, &lsk, &mut records);
            }
        }
    }
    if records.is_empty() {
        return Err("no placeable event — the thread stays unwritten (0 honored)".into());
    }

    let mut bytes = Vec::with_capacity(13 + records.len() * REC_BYTES);
    write_header(&mut bytes, records.len() as u64);
    let mut rec = [0u8; REC_BYTES];
    for r in &records {
        encode_rec(&mut rec, r);
        bytes.extend_from_slice(&rec);
    }
    let expect = 13 + records.len() * REC_BYTES;
    if bytes.len() != expect {
        return Err(format!(
            "{} bytes laid out, {expect} expected — the asset stays unwritten",
            bytes.len()
        ));
    }
    std::fs::write(&out_path, &bytes)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    eprintln!(
        "icecube_compiler: {} events from {rows} rows -> {out_path} ({} bytes, S2E1)",
        records.len(),
        bytes.len()
    );
    if ci_mode && !upload_release(TAG, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("icecube_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str =
        "#           run         event      subevent     MJD[days]  log10(E/GeV)   AngErr[deg]       RA[deg]      Dec[deg]  Azimuth[deg]   Zenith[deg]";

    #[test]
    fn header_maps_the_measured_columns() {
        let c = header_cols(HEADER).unwrap();
        assert_eq!(c.ra, 6);
        assert_eq!(c.dec, 7);
        assert_eq!(c.ang, 5);
        assert_eq!(c.mjd, Some(3));
        assert_eq!(c.log_e, Some(4));
    }

    #[test]
    fn row_becomes_a_neutrino_thread_with_arcsec_sigma() {
        let lsk = embedded_lsk().unwrap();
        let text = format!(
            "{HEADER}\n\"     118175      807013     0   55694.41990176          4.15          0.20       339.389       -13.584       132.444        76.471\"\n"
        );
        let mut out = Vec::new();
        assert_eq!(parse_text(&text, &lsk, &mut out), 1);
        let r = out[0];
        assert_eq!(r.particle_root, ROOT_NEUTRINO);
        assert!((r.ra_deg - 339.389).abs() < 1e-3);
        assert!((r.dec_deg + 13.584).abs() < 1e-3);
        assert!((r.sigma_arcsec.unwrap() - 720.0).abs() < 1e-9);
        assert!(r.epoch_tdb.is_some());
        assert!((r.energy.unwrap() - 10f64.powf(4.15)).abs() < 1.0);
    }

    #[test]
    fn unplaceable_rows_are_dropped_not_faked() {
        let lsk = embedded_lsk().unwrap();
        let bad = format!(
            "{HEADER}\n\" 1 2 3 55694.4 4.0 0.2 400.0 -13.0 1 1\"\n\" 1 2 3 55694.4 4.0 0.0 10.0 13.0 1 1\"\n"
        );
        let mut out = Vec::new();
        assert_eq!(parse_text(&bad, &lsk, &mut out), 0);
    }
}
