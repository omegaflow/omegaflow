use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    COMP_CSES_EFD_ELF_EX, COMP_CSES_EFD_ELF_EY, COMP_CSES_EFD_ELF_EZ, COMP_CSES_EFD_ULF_EX,
    COMP_CSES_EFD_ULF_EY, COMP_CSES_EFD_ULF_EZ, COMP_CSES_EFD_VLF_EX, COMP_CSES_EFD_VLF_EY,
    COMP_CSES_EFD_VLF_EZ, GeoRec, MAGIC_CSES_EFD, verify_bin, write_bin,
};
use omegaflow::archivar::rinex::{ecef_to_geodetic, receiver_ellipsoid_for_compiler};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File};
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::SPECTRAL_NO_BAND;

const NETLOC: &str = "scidb.cn";
const FILETREE_URL: &str = "https://www.scidb.cn/api/sdb-filetree-service/getAllUrl?dataSetId=30660a0fa4f04312b49689c3365fb474&type=personal&version=V1&global=en";
const DEFAULT_FILE_ID: &str = "6398427cbae2f1393c118b54";
const DOWNLOAD: &str = "https://download.scidb.cn/download?fileId=";
const DAY_S: f64 = 86400.0;
const BIN: &str = "cses_efd_compiler";

struct EfdBand {
    tag: &'static str,
    ds: [&'static str; 3],
    comps: [u32; 3],
    fs: f64,
    packet: usize,
}

static EFD_BANDS: [EfdBand; 3] = [
    EfdBand {
        tag: "ulf",
        ds: ["A111_W", "A112_W", "A113_W"],
        comps: [
            COMP_CSES_EFD_ULF_EX,
            COMP_CSES_EFD_ULF_EY,
            COMP_CSES_EFD_ULF_EZ,
        ],
        fs: 125.0,
        packet: 256,
    },
    EfdBand {
        tag: "elf",
        ds: ["A121_W", "A122_W", "A123_W"],
        comps: [
            COMP_CSES_EFD_ELF_EX,
            COMP_CSES_EFD_ELF_EY,
            COMP_CSES_EFD_ELF_EZ,
        ],
        fs: 5000.0,
        packet: 2048,
    },
    EfdBand {
        tag: "vlf",
        ds: ["A131_W", "A132_W", "A133_W"],
        comps: [
            COMP_CSES_EFD_VLF_EX,
            COMP_CSES_EFD_VLF_EY,
            COMP_CSES_EFD_VLF_EZ,
        ],
        fs: 50000.0,
        packet: 2048,
    },
];

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

fn read_units(file: &Hdf5File, name: &str) -> Result<f64, String> {
    let attr = file.attribute(name, "units").ok_or_else(|| {
        format!("{name}: units attribute absent — the V/m claim stays unmeasured")
    })?;
    let text = String::from_utf8_lossy(&attr.data);
    let text = text.trim_matches(|c: char| c == '\0' || c.is_whitespace());
    match text {
        "mV/m" => Ok(1.0e-3),
        "V/m" => Ok(1.0),
        other => Err(format!(
            "{name}: units \"{other}\" carries no V/m arm — the bin stays unwritten"
        )),
    }
}

fn place(lat: f64, lon: f64, alt_km: f64) -> Option<(f64, f64, f64)> {
    if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return None;
    }
    if !lon.is_finite() {
        return None;
    }
    let mut lon_norm = lon;
    while lon_norm > 180.0 {
        lon_norm -= 360.0;
    }
    while lon_norm < -180.0 {
        lon_norm += 360.0;
    }
    if !alt_km.is_finite() || alt_km <= 0.0 {
        return None;
    }
    Some((lat, lon_norm, alt_km * 1000.0))
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

fn assemble(
    comps: [u32; 3],
    fs: f64,
    packet: usize,
    verse_ms: &[f64],
    lat: &[f64],
    lon: &[f64],
    alt_km: &[f64],
    e: [&[f64]; 3],
    to_v_m: f64,
) -> Result<Vec<GeoRec>, String> {
    if !fs.is_finite() || fs <= 0.0 {
        return Err(format!("sample rate {fs} is no time base"));
    }
    if packet == 0 {
        return Err("packet size 0 carries no sample".to_string());
    }
    let np = verse_ms.len();
    let total = np
        .checked_mul(packet)
        .ok_or_else(|| "packet count overflows".to_string())?;
    if total == 0 {
        return Ok(Vec::new());
    }
    for (idx, ds) in ["Ex", "Ey", "Ez"].iter().enumerate() {
        if e[idx].len() < total {
            return Err(format!(
                "{ds} carries {} samples, {np} packets x {packet} needed",
                e[idx].len()
            ));
        }
    }
    let pos_per_packet = lat.len() == np && lon.len() == np && alt_km.len() == np;
    let pos_per_sample = lat.len() >= total && lon.len() >= total && alt_km.len() >= total;
    if !pos_per_packet && !pos_per_sample {
        return Err(format!(
            "position length lat {} lon {} alt {} matches neither {np} packets nor {total} samples",
            lat.len(),
            lon.len(),
            alt_km.len()
        ));
    }
    let epoch = days_from_civil(2009, 1, 1)
        .ok_or_else(|| "2009-01-01 unrepresentable".to_string())? as f64
        * DAY_S;
    let dt = 1.0 / fs;
    let mut out = Vec::with_capacity(total);
    for p in 0..np {
        let stamp = verse_ms[p];
        if !stamp.is_finite() || stamp < 0.0 {
            continue;
        }
        let t_pkt = epoch + stamp / 1000.0;
        for s in 0..packet {
            let k = p * packet + s;
            let (la, lo, al) = if pos_per_packet {
                match place(lat[p], lon[p], alt_km[p]) {
                    Some(v) => v,
                    None => continue,
                }
            } else {
                match place(lat[k], lon[k], alt_km[k]) {
                    Some(v) => v,
                    None => continue,
                }
            };
            let t = t_pkt + s as f64 * dt;
            for c in 0..3 {
                let v = e[c][k];
                if v.is_finite() {
                    push_rec(&mut out, t, la, lo, al, dt, comps[c], v * to_v_m);
                }
            }
        }
    }
    Ok(out)
}

fn assemble_legacy(
    comps: [u32; 3],
    ns: usize,
    utc: &[Option<f64>],
    xyz: &[f64],
    e: [&[f64]; 3],
    to_v_m: f64,
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
        if e[idx].len() < total {
            return Err(format!(
                "{ds} carries {} samples, {np} packets x {ns} needed",
                e[idx].len()
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
                let v = e[c][k];
                if v.is_finite() {
                    push_rec(&mut out, t, la, lo, al, sample_dt, comps[c], v * to_v_m);
                }
            }
        }
    }
    Ok(out)
}

fn parse_modern(file: &Hdf5File, band_tag: Option<&str>) -> Result<Vec<GeoRec>, String> {
    let band = match band_tag {
        Some(tag) => EFD_BANDS
            .iter()
            .find(|b| b.tag == tag)
            .ok_or_else(|| format!("band {tag} absent — ulf|elf|vlf"))?,
        None => EFD_BANDS
            .iter()
            .find(|b| file.dataset(b.ds[0]).is_ok())
            .ok_or_else(|| "no EFD waveform dataset present (A111_W/A121_W/A131_W)".to_string())?,
    };
    let to_v_m = read_units(&file, band.ds[0])?;
    let mut e: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for c in 0..3 {
        e[c] = file
            .read_f64_dataset(band.ds[c])
            .map_err(|n| format!("{} unread: {n:?}", band.ds[c]))?;
    }
    let verse = file
        .read_f64_dataset("VERSE_TIME")
        .map_err(|n| format!("VERSE_TIME unread: {n:?}"))?;
    let lat = file
        .read_f64_dataset("GEO_LAT")
        .map_err(|n| format!("GEO_LAT unread: {n:?}"))?;
    let lon = file
        .read_f64_dataset("GEO_LON")
        .map_err(|n| format!("GEO_LON unread: {n:?}"))?;
    let alt = file
        .read_f64_dataset("ALTITUDE")
        .map_err(|n| format!("ALTITUDE unread: {n:?}"))?;
    assemble(
        band.comps,
        band.fs,
        band.packet,
        &verse,
        &lat,
        &lon,
        &alt,
        [&e[0], &e[1], &e[2]],
        to_v_m,
    )
}

fn parse_legacy(
    file: &Hdf5File,
    band_tag: Option<&str>,
    ellipsoid: Option<(f64, f64)>,
) -> Result<Vec<GeoRec>, String> {
    let band = match band_tag {
        Some(tag) => EFD_BANDS
            .iter()
            .find(|b| b.tag == tag)
            .ok_or_else(|| format!("band {tag} absent — ulf|elf|vlf"))?,
        None => &EFD_BANDS[0],
    };
    let mut e: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for (c, ds) in ["X_WAVE", "Y_WAVE", "Z_WAVE"].iter().enumerate() {
        e[c] = file
            .read_f64_dataset(ds)
            .map_err(|n| format!("{ds} unread: {n:?}"))?;
    }
    let ns = match file.dims("X_WAVE") {
        Some(d) if d.len() == 2 => d[1] as usize,
        _ => return Err("X_WAVE carries no 2-D packet shape".to_string()),
    };
    let utc_raw = read_i64_dataset(file, "UTCTime")?;
    let utc: Vec<Option<f64>> = utc_raw.iter().map(|v| utc_field_to_unix(*v)).collect();
    let xyz = file
        .read_f64_dataset("SAT_POS")
        .map_err(|n| format!("SAT_POS unread: {n:?}"))?;
    let to_v_m = match read_units(file, "X_WAVE") {
        Ok(f) => f,
        Err(_) => 1.0e-3,
    };
    assemble_legacy(
        band.comps,
        ns,
        &utc,
        &xyz,
        [&e[0], &e[1], &e[2]],
        to_v_m,
        ellipsoid,
    )
}

fn parse_efd(
    bytes: &[u8],
    band_tag: Option<&str>,
    ellipsoid: Option<(f64, f64)>,
) -> Result<Vec<GeoRec>, String> {
    let file = Hdf5File::parse(bytes).map_err(|n| format!("HDF5 parse: {n:?}"))?;
    if EFD_BANDS.iter().any(|b| file.dataset(b.ds[0]).is_ok()) {
        return parse_modern(&file, band_tag);
    }
    if file.dataset("X_WAVE").is_ok() {
        return parse_legacy(&file, band_tag, ellipsoid);
    }
    Err(
        "no EFD waveform dataset present (A111_W/A121_W/A131_W or X_WAVE/Y_WAVE/Z_WAVE)"
            .to_string(),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("cses_efd_compiler: --out <path> absent — the output path is never silent");
            std::process::exit(1);
        }
    };
    let band = arg_value(&args, "--band");

    let bytes = if let Some(path) = arg_value(&args, "--input") {
        match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("cses_efd_compiler: read {path}: {e} — the file stays unread");
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
                eprintln!("cses_efd_compiler: fetch void ({url}); filetree {FILETREE_URL}");
                std::process::exit(1);
            }
        }
    };

    let mut records = match parse_efd(
        &bytes,
        band.as_deref(),
        receiver_ellipsoid_for_compiler(BIN),
    ) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("cses_efd_compiler: {msg} — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    if records.is_empty() {
        eprintln!(
            "cses_efd_compiler: {} B carry no placeable EFD sample — the bin stays unwritten (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }

    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bin = write_bin(MAGIC_CSES_EFD, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("cses_efd_compiler: write {out} void");
        std::process::exit(1);
    }
    match verify_bin(MAGIC_CSES_EFD, &bin) {
        Some(n) => eprintln!(
            "cses_efd: {} records, {} B -> {out} (verified {})",
            records.len(),
            bin.len(),
            n
        ),
        None => {
            eprintln!("cses_efd_compiler: {out}: verify void — the asset stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("cses_efd_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (
        [u32; 3],
        f64,
        usize,
        Vec<f64>,
        Vec<f64>,
        Vec<f64>,
        Vec<f64>,
        [Vec<f64>; 3],
        f64,
    ) {
        let verse = vec![0.0f64, 1000.0];
        let lat = vec![10.0f64, 20.0];
        let lon = vec![30.0f64, 40.0];
        let alt = vec![500.0f64, 501.0];
        let ex = vec![1.0f64, 2.0, 3.0, -4.0];
        let ey = vec![5.0f64, 6.0, 7.0, 8.0];
        let ez = vec![9.0f64, 10.0, 11.0, 12.0];
        (
            [
                COMP_CSES_EFD_ULF_EX,
                COMP_CSES_EFD_ULF_EY,
                COMP_CSES_EFD_ULF_EZ,
            ],
            125.0,
            2,
            verse,
            lat,
            lon,
            alt,
            [ex, ey, ez],
            1.0e-3,
        )
    }

    #[test]
    fn assemble_writes_three_components_per_sample() {
        let (comps, fs, packet, verse, lat, lon, alt, e, to_v_m) = fixture();
        let recs = assemble(
            comps,
            fs,
            packet,
            &verse,
            &lat,
            &lon,
            &alt,
            [&e[0], &e[1], &e[2]],
            to_v_m,
        )
        .expect("fixture assembles");
        assert_eq!(recs.len(), 12);
        assert_eq!(recs[0].comp, COMP_CSES_EFD_ULF_EX);
        assert!((recs[0].val - 0.001).abs() < 1e-12);
        assert_eq!(recs[1].comp, COMP_CSES_EFD_ULF_EY);
        assert!((recs[1].val - 0.005).abs() < 1e-12);
        assert_eq!(recs[2].comp, COMP_CSES_EFD_ULF_EZ);
        assert!((recs[2].val - 0.009).abs() < 1e-12);
        assert!((recs[2].alt - 500_000.0).abs() < 1e-9);
        assert!((recs[0].bin_width - 1.0 / 125.0).abs() < 1e-12);
        let epoch = days_from_civil(2009, 1, 1).expect("civil day") as f64 * DAY_S;
        assert!((recs[0].t - epoch).abs() < 1e-6);
        assert!((recs[3].t - (epoch + 1.0 / 125.0)).abs() < 1e-9);
    }

    #[test]
    fn assemble_skips_non_finite_components() {
        let (comps, fs, packet, verse, lat, lon, alt, mut e, to_v_m) = fixture();
        e[0][1] = f64::NAN;
        e[2][2] = f64::INFINITY;
        let recs = assemble(
            comps,
            fs,
            packet,
            &verse,
            &lat,
            &lon,
            &alt,
            [&e[0], &e[1], &e[2]],
            to_v_m,
        )
        .expect("fixture assembles");
        assert_eq!(recs.len(), 10);
        assert!(recs.iter().all(|r| r.val.is_finite()));
    }

    #[test]
    fn assemble_skips_out_of_range_position_packet() {
        let (comps, fs, packet, verse, mut lat, lon, alt, e, to_v_m) = fixture();
        lat[0] = 95.0;
        let recs = assemble(
            comps,
            fs,
            packet,
            &verse,
            &lat,
            &lon,
            &alt,
            [&e[0], &e[1], &e[2]],
            to_v_m,
        )
        .expect("fixture assembles");
        assert_eq!(recs.len(), 6);
        assert!(recs.iter().all(|r| (r.lat - 20.0).abs() < 1e-9));
    }

    #[test]
    fn assemble_refuses_sample_count_mismatch() {
        let (comps, fs, packet, verse, lat, lon, alt, e, to_v_m) = fixture();
        let short = [&e[0][..3], &e[1][..], &e[2][..]];
        assert!(assemble(comps, fs, packet, &verse, &lat, &lon, &alt, short, to_v_m).is_err());
    }

    #[test]
    fn utc_field_reads_millisecond_stamp() {
        let t = utc_field_to_unix(20180811214030825).expect("stamp parses");
        let expect = days_from_civil(2018, 8, 11).expect("civil day") as f64 * DAY_S
            + 21.0 * 3600.0
            + 40.0 * 60.0
            + 30.0
            + 0.825;
        assert!((t - expect).abs() < 1e-6);
    }

    #[test]
    fn assemble_legacy_packs_ecef_position_and_derives_dt() {
        let epoch = days_from_civil(2018, 8, 11).expect("civil day") as f64 * DAY_S + 21.0 * 3600.0;
        let utc = [Some(epoch), Some(epoch + 1.0)];
        let xyz = [
            164443.0, 2390484.0, -6447972.0, 167864.0, 2396892.0, -6445496.0,
        ];
        let ex = [1.0f64, 2.0, 3.0, -4.0];
        let ey = [5.0f64, 6.0, 7.0, 8.0];
        let ez = [9.0f64, 10.0, 11.0, 12.0];
        let comps = [
            COMP_CSES_EFD_ULF_EX,
            COMP_CSES_EFD_ULF_EY,
            COMP_CSES_EFD_ULF_EZ,
        ];
        let recs = assemble_legacy(
            comps,
            2,
            &utc,
            &xyz,
            [&ex, &ey, &ez],
            1.0e-3,
            Some((6_378_137.0, 6.694_379_990_14e-3)),
        )
        .expect("assembles");
        assert_eq!(recs.len(), 12);
        assert!((recs[0].t - epoch).abs() < 1e-6);
        assert!((recs[3].t - (epoch + 0.5)).abs() < 1e-9);
        assert!((recs[0].bin_width - 0.5).abs() < 1e-12);
        assert!((-90.0..=-60.0).contains(&recs[0].lat));
        assert!((recs[0].alt - 519_000.0).abs() < 20_000.0);
        assert!((recs[0].val - 0.001).abs() < 1e-12);
    }

    #[test]
    fn assemble_legacy_refuses_missing_position() {
        let utc = [Some(0.0)];
        let xyz = [1.0, 2.0];
        let ex = [1.0f64, 2.0];
        let ey = [1.0f64, 2.0];
        let ez = [1.0f64, 2.0];
        let comps = [
            COMP_CSES_EFD_ULF_EX,
            COMP_CSES_EFD_ULF_EY,
            COMP_CSES_EFD_ULF_EZ,
        ];
        assert!(
            assemble_legacy(
                comps,
                2,
                &utc,
                &xyz,
                [&ex, &ey, &ez],
                1.0e-3,
                Some((6_378_137.0, 6.694_379_990_14e-3)),
            )
            .is_err()
        );
    }

    #[test]
    fn efd_bin_roundtrips_through_write_verify_parse() {
        let recs = vec![GeoRec {
            t: 1_230_768_001.0,
            lat: -69.5,
            lon: 85.8,
            alt: 519_300.0,
            freq: SPECTRAL_NO_BAND,
            bin_width: 1.0 / 5000.0,
            val: 0.0025,
            comp: COMP_CSES_EFD_ELF_EX,
            station: 0,
        }];
        let bin = write_bin(MAGIC_CSES_EFD, &recs);
        assert_eq!(verify_bin(MAGIC_CSES_EFD, &bin), Some(1));
        let parsed = omegaflow::archivar::geo::parse_bin(MAGIC_CSES_EFD, &bin).expect("parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].val, 0.0025);
        assert_eq!(parsed[0].comp, COMP_CSES_EFD_ELF_EX);
        assert!(verify_bin(omegaflow::archivar::geo::MAGIC_CSES_LAP, &bin).is_none());
    }
}
