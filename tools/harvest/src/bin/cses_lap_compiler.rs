use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    COMP_CSES_DENS, COMP_CSES_TEMP, COMP_CSES_UF, COMP_CSES_UP, GeoRec, MAGIC_CSES_LAP, verify_bin,
    write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File};
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::SPECTRAL_NO_BAND;

const NETLOC: &str = "scidb.cn";
const FILETREE_URL: &str = "https://www.scidb.cn/api/sdb-filetree-service/getAllUrl?dataSetId=30660a0fa4f04312b49689c3365fb474&type=personal&version=V1&global=en";
const DEFAULT_FILE_ID: &str = "6398427cbae2f1393c118b50";
const DOWNLOAD: &str = "https://download.scidb.cn/download?fileId=";
const DAY_S: f64 = 86400.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn download_url(file_id: &str) -> String {
    format!("{DOWNLOAD}{file_id}")
}

fn utc_field_to_unix(v: i64) -> Option<f64> {
    if v <= 0 {
        return None;
    }
    let secs = v / 1000;
    let millis = (v % 1000) as f64;
    if secs < 10_000_000_000_000 {
        return None;
    }
    let yyyy = secs / 10_000_000_000;
    let mm = (secs / 100_000_000) % 100;
    let dd = (secs / 1_000_000) % 100;
    let hh = (secs / 10_000) % 100;
    let mi = (secs / 100) % 100;
    let ss = secs % 100;
    if !(1..=12).contains(&mm)
        || !(1..=31).contains(&dd)
        || !(0..=23).contains(&hh)
        || !(0..=59).contains(&mi)
        || !(0..=60).contains(&ss)
    {
        return None;
    }
    let days = days_from_civil(yyyy, mm, dd)?;
    Some(days as f64 * DAY_S + (hh * 3600 + mi * 60 + ss) as f64 + millis / 1000.0)
}

fn read_i64_dataset(file: &Hdf5File, name: &str) -> Result<Vec<i64>, String> {
    let (_, ds, dt) = file
        .dataset(name)
        .map_err(|n| format!("{name} absent: {n:?}"))?;
    if dt.class != 0 || dt.size != 8 {
        return Err(format!(
            "{name} reads class {} size {} — an 8-byte integer is required",
            dt.class, dt.size
        ));
    }
    let raw = file
        .read_dataset(name)
        .map_err(|n| format!("{name} unread: {n:?}"))?;
    let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let off = i * 8;
        let arr: [u8; 8] = raw
            .get(off..off + 8)
            .and_then(|s| s.try_into().ok())
            .ok_or_else(|| format!("{name} ends at element {i}"))?;
        out.push(match dt.endian {
            Endian::Le => i64::from_le_bytes(arr),
            Endian::Be => i64::from_be_bytes(arr),
        });
    }
    Ok(out)
}

fn cadence_seconds(times: &[f64]) -> f64 {
    let mut deltas: Vec<f64> = times
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|d| d.is_finite() && *d > 0.0)
        .collect();
    if deltas.is_empty() {
        return SPECTRAL_NO_BAND;
    }
    deltas.sort_by(f64::total_cmp);
    deltas[deltas.len() / 2]
}

fn assemble(
    utc: &[i64],
    lat: &[f32],
    lon: &[f32],
    alt_km: &[f32],
    dens: &[f64],
    temp: &[f64],
    up: &[f64],
    uf: &[f64],
) -> Vec<GeoRec> {
    let count = [
        utc.len(),
        lat.len(),
        lon.len(),
        alt_km.len(),
        dens.len(),
        temp.len(),
        up.len(),
        uf.len(),
    ]
    .into_iter()
    .min();
    let Some(count) = count else {
        eprintln!("cses_lap: a dataset is absent — the bin stays unwritten (0 honored)");
        return Vec::new();
    };
    let times: Vec<Option<f64>> = (0..count).map(|i| utc_field_to_unix(utc[i])).collect();
    let unix: Vec<f64> = times.iter().flatten().copied().collect();
    let cadence = cadence_seconds(&unix);
    let mut out = Vec::new();
    for i in 0..count {
        let Some(t) = times[i] else { continue };
        let la = lat[i] as f64;
        let lo = lon[i] as f64;
        let al = alt_km[i] as f64;
        if !la.is_finite() || !(-90.0..=90.0).contains(&la) {
            continue;
        }
        if !lo.is_finite() {
            continue;
        }
        let mut lon_norm = lo;
        while lon_norm > 180.0 {
            lon_norm -= 360.0;
        }
        while lon_norm < -180.0 {
            lon_norm += 360.0;
        }
        if !al.is_finite() || al <= 0.0 {
            continue;
        }
        let alt_m = al * 1000.0;
        let mut push = |comp: u32, val: f64| {
            out.push(GeoRec {
                t,
                lat: la,
                lon: lon_norm,
                alt: alt_m,
                freq: SPECTRAL_NO_BAND,
                bin_width: cadence,
                val,
                comp,
                station: 0,
            });
        };
        if dens[i].is_finite() && dens[i] > 0.0 {
            push(COMP_CSES_DENS, dens[i] * 1.0e-6);
        }
        if temp[i].is_finite() && temp[i] > 0.0 {
            push(COMP_CSES_TEMP, temp[i]);
        }
        if up[i].is_finite() {
            push(COMP_CSES_UP, up[i]);
        }
        if uf[i].is_finite() {
            push(COMP_CSES_UF, uf[i]);
        }
    }
    out
}

fn parse_lap(bytes: &[u8]) -> Result<Vec<GeoRec>, String> {
    let file = Hdf5File::parse(bytes).map_err(|n| format!("HDF5 parse: {n:?}"))?;
    let utc = read_i64_dataset(&file, "UTC_TIME")?;
    let lat = file
        .read_f32_dataset("GEO_LAT")
        .map_err(|n| format!("GEO_LAT unread: {n:?}"))?;
    let lon = file
        .read_f32_dataset("GEO_LON")
        .map_err(|n| format!("GEO_LON unread: {n:?}"))?;
    let alt = file
        .read_f32_dataset("ALTITUDE")
        .map_err(|n| format!("ALTITUDE unread: {n:?}"))?;
    let dens = file
        .read_f64_dataset("A311")
        .map_err(|n| format!("A311 unread: {n:?}"))?;
    let temp = file
        .read_f64_dataset("A321")
        .map_err(|n| format!("A321 unread: {n:?}"))?;
    let up = file
        .read_f64_dataset("UP")
        .map_err(|n| format!("UP unread: {n:?}"))?;
    let uf = file
        .read_f64_dataset("UF")
        .map_err(|n| format!("UF unread: {n:?}"))?;
    Ok(assemble(&utc, &lat, &lon, &alt, &dens, &temp, &up, &uf))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("cses_lap_compiler: --out <path> absent — the output path is never silent");
            std::process::exit(1);
        }
    };

    let bytes = if let Some(path) = arg_value(&args, "--input") {
        match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("cses_lap_compiler: read {path}: {e} — the file stays unread");
                std::process::exit(1);
            }
        }
    } else {
        let url = match arg_value(&args, "--url") {
            Some(u) => u,
            None => {
                let file_id = match arg_value(&args, "--file-id") {
                    Some(v) => v,
                    None => DEFAULT_FILE_ID.to_string(),
                };
                download_url(&file_id)
            }
        };
        match fetch_raw_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("cses_lap_compiler: fetch void ({url}); filetree {FILETREE_URL}");
                std::process::exit(1);
            }
        }
    };

    let mut records = match parse_lap(&bytes) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("cses_lap_compiler: {msg} — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    if records.is_empty() {
        eprintln!(
            "cses_lap_compiler: {} B carry no placeable LAP sample — the bin stays unwritten (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }

    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bin = write_bin(MAGIC_CSES_LAP, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("cses_lap_compiler: write {out} void");
        std::process::exit(1);
    }
    match verify_bin(MAGIC_CSES_LAP, &bin) {
        Some(n) => eprintln!(
            "cses_lap: {} records, {} B -> {out} (verified {})",
            records.len(),
            bin.len(),
            n
        ),
        None => {
            eprintln!("cses_lap_compiler: {out}: verify void — the asset stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("cses_lap_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_field_reads_millisecond_stamp() {
        let t = utc_field_to_unix(20180811214033825).expect("stamp parses");
        let expect = days_from_civil(2018, 8, 11).expect("civil day") as f64 * DAY_S
            + 21.0 * 3600.0
            + 40.0 * 60.0
            + 33.0
            + 0.825;
        assert!((t - expect).abs() < 1e-6);
    }

    #[test]
    fn utc_field_rejects_short_and_absent_stamps() {
        assert_eq!(utc_field_to_unix(0), None);
        assert_eq!(utc_field_to_unix(-99999), None);
        assert_eq!(utc_field_to_unix(123), None);
        assert_eq!(utc_field_to_unix(20181811214033825), None);
    }

    #[test]
    fn assemble_gates_density_temperature_and_keeps_zero_potential() {
        let utc = [20180811214033825i64, 20180811214036825];
        let lat = [-69.5f32, -69.4];
        let lon = [85.8f32, 85.6];
        let alt = [519.3f32, 519.2];
        let dens = [-99999.0f64, 5.5e8];
        let temp = [1257.0f64, -99999.0];
        let up = [0.0f64, 1.8];
        let uf = [1.3f64, f64::NAN];
        let recs = assemble(&utc, &lat, &lon, &alt, &dens, &temp, &up, &uf);
        assert_eq!(recs.len(), 5);
        assert_eq!(recs[0].comp, COMP_CSES_TEMP);
        assert_eq!(recs[0].val, 1257.0);
        assert_eq!(recs[1].comp, COMP_CSES_UP);
        assert_eq!(recs[1].val, 0.0);
        assert_eq!(recs[2].comp, COMP_CSES_UF);
        assert_eq!(recs[3].comp, COMP_CSES_DENS);
        assert!((recs[3].val - 550.0).abs() < 1e-9);
        assert_eq!(recs[4].comp, COMP_CSES_UP);
        assert!((recs[0].alt - 519300.0).abs() < 0.1);
        assert!((recs[0].bin_width - 3.0).abs() < 1e-6);
    }

    #[test]
    fn assemble_skips_out_of_range_latitude() {
        let utc = [20180811214033825i64, 20180811214036825];
        let lat = [95.0f32, -69.4];
        let lon = [85.8f32, 85.6];
        let alt = [519.3f32, 519.2];
        let dens = [5.5e8f64, 5.5e8];
        let temp = [1257.0f64, 1257.0];
        let up = [1.7f64, 1.7];
        let uf = [1.3f64, 1.3];
        let recs = assemble(&utc, &lat, &lon, &alt, &dens, &temp, &up, &uf);
        assert_eq!(recs.len(), 4);
        let want = -69.4f32 as f64;
        assert!(recs.iter().all(|r| (r.lat - want).abs() < 1e-9));
    }

    #[test]
    fn cses_bin_roundtrips_through_write_verify_parse() {
        let recs = vec![GeoRec {
            t: 1_533_999_633.825,
            lat: -69.5,
            lon: 85.8,
            alt: 519_300.0,
            freq: SPECTRAL_NO_BAND,
            bin_width: 3.0,
            val: 5.5e8,
            comp: COMP_CSES_DENS,
            station: 0,
        }];
        let bin = write_bin(MAGIC_CSES_LAP, &recs);
        assert_eq!(verify_bin(MAGIC_CSES_LAP, &bin), Some(1));
        let parsed = omegaflow::archivar::geo::parse_bin(MAGIC_CSES_LAP, &bin).expect("parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].val, 5.5e8);
        assert_eq!(parsed[0].comp, COMP_CSES_DENS);
        assert!(verify_bin(omegaflow::archivar::geo::MAGIC_ASCAT, &bin).is_none());
    }
}
