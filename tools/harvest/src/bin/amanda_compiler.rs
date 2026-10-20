
use omegaflow::archivar::asset_path::compiled;
use omegaflow::archivar::inflate::zip_members;
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::s2event::{
    REC_BYTES, ROOT_NEUTRINO, S2EventRecord, encode_rec, write_header,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::LeapSeconds;

const TAG: &str = "icecube.wisc.edu";
const DEFAULT_OUT: &str = "amanda_7yr.s2e1";
const MJD_UNIX_EPOCH: f64 = 40587.0;
const DEFAULT_URL: &str = "https://icecube.wisc.edu/data-releases/20080911_AMANDA_7_Year_Data.zip";
const MEMBER: &str = "AMANDA_7_Year_Data.txt";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn is_header(line: &str) -> bool {
    let t = line.trim();
    t.contains("Dec.(deg)") && t.contains("RA(h)") && t.contains("MJD")
}

pub fn parse_text(text: &str, lsk: &LeapSeconds) -> Vec<S2EventRecord> {
    let mut out = Vec::new();
    let mut seen_header = false;
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if is_header(t) {
            seen_header = true;
            continue;
        }
        if !seen_header {
            continue;
        }
        let cells: Vec<&str> = t.split_whitespace().collect();
        if cells.len() < 8 {
            continue;
        }
        let num = |i: usize| cells.get(i).and_then(|s| s.parse::<f64>().ok());
        let (Some(dec), Some(ra_h), Some(nch), Some(resol), Some(mjd)) =
            (num(0), num(1), num(2), num(3), num(7))
        else {
            continue;
        };
        if !(dec.is_finite()
            && ra_h.is_finite()
            && nch.is_finite()
            && resol.is_finite()
            && mjd.is_finite())
        {
            continue;
        }
        if !(-90.0..=90.0).contains(&dec)
            || !(0.0..24.0).contains(&ra_h)
            || nch <= 0.0
            || resol <= 0.0
            || mjd < 40_000.0
        {
            continue;
        }
        let Some(epoch_tdb) = lsk.unix_to_tdb((mjd - MJD_UNIX_EPOCH) * 86400.0) else {
            continue;
        };
        out.push(S2EventRecord {
            ra_deg: (ra_h * 15.0) as f32,
            dec_deg: dec as f32,
            sigma_arcsec: Some(resol * 3600.0),
            epoch_tdb: Some(epoch_tdb),
            energy: None,
            signalness: None,
            far: None,
            particle_root: ROOT_NEUTRINO,
        });
    }
    out
}

fn parse_zip(bytes: &[u8], lsk: &LeapSeconds) -> Option<Vec<S2EventRecord>> {
    let mut member: Option<Vec<u8>> = None;
    zip_members(bytes, |name, data| {
        if member.is_none() && name.ends_with(MEMBER) && !name.contains("__MACOSX/") {
            member = Some(data.to_vec());
        }
    })?;
    let text = String::from_utf8(member?).ok()?;
    let records = parse_text(&text, lsk);
    if records.is_empty() {
        None
    } else {
        Some(records)
    }
}

fn selftest() {
    let lsk = embedded_lsk().expect("embedded leap-second table");
    let text = "\n Dec.(deg)   RA(h)       Nch         Resol.(deg) Year        GPS Day     Second      MJD            Atm Subset\n ---------   --------    ---         ----------- ----        -------     ------      ------------   ----------\n 2.2453      16.5054     24          2.37375     2000        47          51553       51590.596678   X\n 65.364      0.983633    57          1.12197     2000        48          33373       51591.386262\n";
    let records = parse_text(text, &lsk);
    if records.len() != 2 {
        eprintln!("selftest: the fixture does not yield two events");
        std::process::exit(1);
    }
    if records[0].particle_root != ROOT_NEUTRINO
        || (records[0].ra_deg - 247.581).abs() > 1e-3
        || records[0].epoch_tdb.is_none()
    {
        eprintln!("selftest: the first event does not carry direction/epoch");
        std::process::exit(1);
    }
    eprintln!("amanda_compiler: selftest passes (header + direction + MJD)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            let path = compiled(TAG, DEFAULT_OUT);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("the embedded leap-second table stays unread — MJD epochs stay unassigned");
        std::process::exit(1);
    };
    let bytes = match arg_value(&args, "--file") {
        Some(path) => match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(err) => {
                eprintln!("read {path} returned void: {err}");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--url") {
                Some(v) => v,
                None => DEFAULT_URL.to_string(),
            };
            match omegaflow::archivar::fetch_raw_bytes(&url) {
                Some(bytes) => bytes,
                None => {
                    eprintln!("{url}: the source stays unfetched");
                    std::process::exit(1);
                }
            }
        }
    };
    let Some(records) = parse_zip(&bytes, &lsk) else {
        eprintln!("{MEMBER}: no placeable event — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    };

    let mut out = Vec::with_capacity(13 + records.len() * REC_BYTES);
    write_header(&mut out, records.len() as u64);
    let mut rec = [0u8; REC_BYTES];
    for record in &records {
        encode_rec(&mut rec, record);
        out.extend_from_slice(&rec);
    }
    let expect = 13 + records.len() * REC_BYTES;
    if out.len() != expect {
        eprintln!(
            "{} bytes laid out, {expect} expected — the asset stays unwritten",
            out.len()
        );
        std::process::exit(1);
    }
    if std::fs::write(&out_path, &out).is_err() {
        eprintln!("write {out_path} returned void");
        std::process::exit(1);
    }
    eprintln!(
        "amanda_compiler: {} events -> {out_path} ({} bytes, S2E1)",
        records.len(),
        out.len()
    );
    if ci_mode && !upload_release(TAG, &out_path) {
        eprintln!("{out_path}: CDN upload returned void");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "\n Dec.(deg)   RA(h)       Nch         Resol.(deg) Year        GPS Day     Second      MJD            Atm Subset\n ---------   --------    ---         ----------- ----        -------     ------      ------------   ----------\n";

    #[test]
    fn header_and_rows_map_direction_time_and_resolution() {
        let lsk = embedded_lsk().unwrap();
        let text = format!(
            "{HEADER} 2.2453      16.5054     24          2.37375     2000        47          51553       51590.596678   X\n 65.364      0.983633    57          1.12197     2000        48          33373       51591.386262\n"
        );
        let records = parse_text(&text, &lsk);
        assert_eq!(records.len(), 2);
        let first = records[0];
        assert_eq!(first.particle_root, ROOT_NEUTRINO);
        assert!((first.ra_deg - 247.581).abs() < 1e-3);
        assert!((first.dec_deg - 2.2453).abs() < 1e-6);
        assert!((first.sigma_arcsec.unwrap() - 8545.5).abs() < 1e-6);
        assert!(first.epoch_tdb.is_some());
        assert!(first.energy.is_none());
        assert_eq!(records[1].sigma_arcsec, Some(1.12197 * 3600.0));
    }

    #[test]
    fn unplaceable_rows_are_dropped_not_faked() {
        let lsk = embedded_lsk().unwrap();
        let text = format!(
            "{HEADER} 95.0       16.5054     24          2.37375     2000        47          51553       51590.596678   X\n 2.0        25.0        24          2.37375     2000        47          51553       51590.596678   X\n 2.0        1.0          0          2.37375     2000        47          51553       51590.596678   X\n"
        );
        assert!(parse_text(&text, &lsk).is_empty());
    }

    #[test]
    fn rows_before_the_header_are_not_events() {
        let lsk = embedded_lsk().unwrap();
        let text = " 2.2453 16.5054 24 2.37375 2000 47 51553 51590.596678 X\n";
        assert!(parse_text(text, &lsk).is_empty());
    }
}
