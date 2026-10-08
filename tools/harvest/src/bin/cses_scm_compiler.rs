use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    COMP_CSES_SCM_ULF_X, COMP_CSES_SCM_ULF_Y, COMP_CSES_SCM_ULF_Z, GeoRec, MAGIC_CSES_SCM,
    verify_bin, write_bin,
};
use omegaflow::archivar::hdf5::{Endian, Hdf5File};
use omegaflow::archivar::rinex::{ecef_to_geodetic, receiver_ellipsoid_for_compiler};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::SPECTRAL_NO_BAND;

const NETLOC: &str = "scidb.cn";
const FILETREE_URL: &str = "https://www.scidb.cn/api/sdb-filetree-service/getAllUrl?dataSetId=30660a0fa4f04312b49689c3365fb474&type=personal&version=V1&global=en";
const DEFAULT_FILE_ID: &str = "6398427cbae2f1393c118b55";
const DOWNLOAD: &str = "https://download.scidb.cn/download?fileId=";
const DAY_S: f64 = 86400.0;
const BIN: &str = "cses_scm_compiler";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn download_url(file_id: &str) -> String {
    format!("{DOWNLOAD}{file_id}")
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

fn median_positive(mut deltas: Vec<f64>) -> f64 {
    if deltas.is_empty() {
        return SPECTRAL_NO_BAND;
    }
    deltas.sort_by(f64::total_cmp);
    deltas[deltas.len() / 2]
}

fn push_rec(
    out: &mut Vec<GeoRec>,
    t: f64,
    lat: f64,
    lon: f64,
    alt_m: f64,
    cadence: f64,
    comp: u32,
    val: f64,
) {
    out.push(GeoRec {
        t,
        lat,
        lon,
        alt: alt_m,
        freq: SPECTRAL_NO_BAND,
        bin_width: cadence,
        val,
        comp,
        station: 0,
    });
}

fn assemble_legacy(
    comps: [u32; 3],
    ns: usize,
    utc: &[Option<f64>],
    xyz: &[f64],
    b: [&[f64]; 3],
    to_nt: f64,
    ellipsoid: Option<(f64, f64)>,
) -> Result<Vec<GeoRec>, String> {
    if ns == 0 {
        return Err("packet size 0 carries no sample".to_string());
    }
    let np = utc.len();
    let total = np
        .checked_mul(ns)
        .ok_or_else(|| "packet count overflows".to_string())?;
    if total == 0 {
        return Ok(Vec::new());
    }
    for (idx, ds) in ["X_WAVE", "Y_WAVE", "Z_WAVE"].iter().enumerate() {
        if b[idx].len() < total {
            return Err(format!(
                "{ds} carries {} samples, {np} packets x {ns} needed",
                b[idx].len()
            ));
        }
    }
    if xyz.len() < np * 3 {
        return Err(format!(
            "SAT_POS carries {} values, {np} x 3 needed",
            xyz.len()
        ));
    }
    let deltas: Vec<f64> = utc
        .windows(2)
        .filter_map(|w| match (w[0], w[1]) {
            (Some(a), Some(b)) => {
                let d = b - a;
                if d.is_finite() && d > 0.0 {
                    Some(d)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();
    let sample_dt = median_positive(deltas) / ns as f64;
    let mut out = Vec::with_capacity(total);
    for p in 0..np {
        let Some(t_pkt) = utc[p] else { continue };
        let Some((a, e2)) = ellipsoid else { continue };
        let (la, lo, al) = match ecef_to_geodetic(xyz[p * 3], xyz[p * 3 + 1], xyz[p * 3 + 2], a, e2)
        {
            Some((la, lo, al))
                if la.is_finite()
                    && (-90.0..=90.0).contains(&la)
                    && lo.is_finite()
                    && al.is_finite()
                    && al > 0.0 =>
            {
                (la, lo, al)
            }
            _ => continue,
        };
        for s in 0..ns {
            let k = p * ns + s;
            let t = t_pkt + s as f64 * sample_dt;
            for c in 0..3 {
                let v = b[c][k];
                if v.is_finite() {
                    push_rec(&mut out, t, la, lo, al, sample_dt, comps[c], v * to_nt);
                }
            }
        }
    }
    Ok(out)
}

fn parse_scm(bytes: &[u8], ellipsoid: Option<(f64, f64)>) -> Result<Vec<GeoRec>, String> {
    let file = Hdf5File::parse(bytes).map_err(|n| format!("HDF5 parse: {n:?}"))?;
    let mut b: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for (c, ds) in ["X_WAVE", "Y_WAVE", "Z_WAVE"].iter().enumerate() {
        b[c] = file
            .read_f64_dataset(ds)
            .map_err(|n| format!("{ds} unread: {n:?}"))?;
    }
    let ns = match file.dims("X_WAVE") {
        Some(d) if d.len() == 2 => d[1] as usize,
        _ => return Err("X_WAVE carries no 2-D packet shape".to_string()),
    };
    let utc_raw = read_i64_dataset(&file, "UTCTime")?;
    let utc: Vec<Option<f64>> = utc_raw.iter().map(|v| utc_field_to_unix(*v)).collect();
    let xyz = file
        .read_f64_dataset("SAT_POS")
        .map_err(|n| format!("SAT_POS unread: {n:?}"))?;
    let comps = [
        COMP_CSES_SCM_ULF_X,
        COMP_CSES_SCM_ULF_Y,
        COMP_CSES_SCM_ULF_Z,
    ];
    assemble_legacy(comps, ns, &utc, &xyz, [&b[0], &b[1], &b[2]], 1.0, ellipsoid)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("cses_scm_compiler: --out <path> absent — the output path is never silent");
            std::process::exit(1);
        }
    };

    let bytes = if let Some(path) = arg_value(&args, "--input") {
        match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("cses_scm_compiler: read {path}: {e} — the file stays unread");
                std::process::exit(1);
            }
        }
    } else {
        let url = match arg_value(&args, "--url") {
            Some(u) => u,
            None => match arg_value(&args, "--file-id") {
                Some(id) => download_url(&id),
                None => download_url(DEFAULT_FILE_ID),
            },
        };
        match fetch_raw_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("cses_scm_compiler: fetch void ({url}); filetree {FILETREE_URL}");
                std::process::exit(1);
            }
        }
    };

    let mut records = match parse_scm(&bytes, receiver_ellipsoid_for_compiler(BIN)) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("cses_scm_compiler: {msg} — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    if records.is_empty() {
        eprintln!(
            "cses_scm_compiler: {} B carry no placeable SCM sample — the bin stays unwritten (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }

    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bin = write_bin(MAGIC_CSES_SCM, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("cses_scm_compiler: write {out} void");
        std::process::exit(1);
    }
    match verify_bin(MAGIC_CSES_SCM, &bin) {
        Some(n) => eprintln!(
            "cses_scm: {} records, {} B -> {out} (verified {})",
            records.len(),
            bin.len(),
            n
        ),
        None => {
            eprintln!("cses_scm_compiler: {out}: verify void — the asset stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("cses_scm_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_field_reads_millisecond_stamp() {
        let t = utc_field_to_unix(20180811214154006).expect("stamp parses");
        let expect = days_from_civil(2018, 8, 11).expect("civil day") as f64 * DAY_S
            + 21.0 * 3600.0
            + 41.0 * 60.0
            + 54.0
            + 0.006;
        assert!((t - expect).abs() < 1e-6);
    }

    #[test]
    fn assemble_legacy_packs_ecef_position_and_derives_dt() {
        let epoch = days_from_civil(2018, 8, 11).expect("civil day") as f64 * DAY_S + 21.0 * 3600.0;
        let utc = [Some(epoch), Some(epoch + 1.0)];
        let xyz = [
            164443.0, 2390484.0, -6447972.0, 167864.0, 2396892.0, -6445496.0,
        ];
        let x = [1.0f64, 2.0, 3.0, -4.0];
        let y = [5.0f64, 6.0, 7.0, 8.0];
        let z = [9.0f64, 10.0, 11.0, 12.0];
        let comps = [
            COMP_CSES_SCM_ULF_X,
            COMP_CSES_SCM_ULF_Y,
            COMP_CSES_SCM_ULF_Z,
        ];
        let recs = assemble_legacy(
            comps,
            2,
            &utc,
            &xyz,
            [&x, &y, &z],
            1.0,
            Some((6_378_137.0, 6.694_379_990_14e-3)),
        )
        .expect("assembles");
        assert_eq!(recs.len(), 12);
        assert_eq!(recs[0].comp, COMP_CSES_SCM_ULF_X);
        assert!((recs[0].val - 1.0).abs() < 1e-12);
        assert!((recs[0].bin_width - 0.5).abs() < 1e-12);
        assert!((recs[0].alt - 519_000.0).abs() < 20_000.0);
    }

    #[test]
    fn scm_bin_roundtrips_through_write_verify_parse() {
        let recs = vec![GeoRec {
            t: 1_533_974_930.0,
            lat: -64.74,
            lon: 81.15,
            alt: 517_500.0,
            freq: SPECTRAL_NO_BAND,
            bin_width: 1.0 / 200.0,
            val: 9.5,
            comp: COMP_CSES_SCM_ULF_Y,
            station: 0,
        }];
        let bin = write_bin(MAGIC_CSES_SCM, &recs);
        assert_eq!(verify_bin(MAGIC_CSES_SCM, &bin), Some(1));
        let parsed = omegaflow::archivar::geo::parse_bin(MAGIC_CSES_SCM, &bin).expect("parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].val, 9.5);
        assert_eq!(parsed[0].comp, COMP_CSES_SCM_ULF_Y);
        assert!(verify_bin(omegaflow::archivar::geo::MAGIC_CSES_HPM, &bin).is_none());
    }
}
