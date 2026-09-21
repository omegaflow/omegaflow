use omegaflow::archivar::geo::{
    COMP_GLML2_FLASH_ENERGY, GeoRec, MAGIC_GLML2, parse_bin, write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5Attribute, Hdf5File};
use omegaflow::lsk::{LeapSeconds, days_from_civil, parse as parse_lsk};
use std::process::Command;

const NETLOC: &str = "noaa-goes18";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSLf")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("240");
    let out = cmd.arg(url).output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch {}: {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn granule_bytes(path_or_url: &str) -> Option<Vec<u8>> {
    if let Ok(b) = std::fs::read(path_or_url) {
        return Some(b);
    }
    fetch(path_or_url)
}

struct VarLoad {
    raw: Vec<u8>,
    size: usize,
    endian: Endian,
    signed: bool,
    n: usize,
}

fn dataset_load(file: &Hdf5File, name: &str) -> Option<VarLoad> {
    let (obj, ds, dt) = file.dataset(name).ok()?;
    if obj.is_group {
        return None;
    }
    let n: usize = ds
        .dims
        .iter()
        .fold(1usize, |a, d| a.saturating_mul(*d as usize));
    if n == 0 {
        return None;
    }
    let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_dataset(name)))
        .ok()?
        .ok()?;
    Some(VarLoad {
        raw,
        size: dt.size,
        endian: dt.endian,
        signed: dt.signed,
        n,
    })
}

fn dataset_attrs<'a>(file: &'a Hdf5File, name: &str) -> Option<&'a [Hdf5Attribute]> {
    let (obj, _, _) = file.dataset(name).ok()?;
    Some(&obj.attrs)
}

fn decode_int_at(
    data: &[u8],
    off: usize,
    size: usize,
    endian: Endian,
    signed: bool,
) -> Option<i64> {
    let be = endian == Endian::Be;
    match size {
        1 => data
            .get(off)
            .map(|&b| if signed { b as i8 as i64 } else { b as i64 }),
        2 => {
            let b: [u8; 2] = data.get(off..off + 2)?.try_into().ok()?;
            let v = if be {
                u16::from_be_bytes(b)
            } else {
                u16::from_le_bytes(b)
            };
            Some(if signed { v as i16 as i64 } else { v as i64 })
        }
        4 => {
            let b: [u8; 4] = data.get(off..off + 4)?.try_into().ok()?;
            let v = if be {
                u32::from_be_bytes(b)
            } else {
                u32::from_le_bytes(b)
            };
            Some(if signed { v as i32 as i64 } else { v as i64 })
        }
        8 => {
            let b: [u8; 8] = data.get(off..off + 8)?.try_into().ok()?;
            let v = if be {
                u64::from_be_bytes(b)
            } else {
                u64::from_le_bytes(b)
            };
            Some(v as i64)
        }
        _ => None,
    }
}

fn decode_float_at(data: &[u8], off: usize, size: usize, endian: Endian) -> Option<f64> {
    match size {
        4 => {
            let b: [u8; 4] = data.get(off..off + 4)?.try_into().ok()?;
            let v = if endian == Endian::Be {
                f32::from_be_bytes(b)
            } else {
                f32::from_le_bytes(b)
            };
            Some(v as f64)
        }
        8 => {
            let b: [u8; 8] = data.get(off..off + 8)?.try_into().ok()?;
            Some(if endian == Endian::Be {
                f64::from_be_bytes(b)
            } else {
                f64::from_le_bytes(b)
            })
        }
        _ => None,
    }
}

fn attr_find<'a>(attrs: &'a [Hdf5Attribute], name: &str) -> Option<&'a Hdf5Attribute> {
    attrs.iter().find(|a| a.name == name)
}

fn attr_int(attrs: &[Hdf5Attribute], name: &str) -> Option<i64> {
    let a = attr_find(attrs, name)?;
    if a.datatype.class != 0 {
        return None;
    }
    decode_int_at(
        &a.data,
        0,
        a.datatype.size,
        a.datatype.endian,
        a.datatype.signed,
    )
}

fn attr_int_unsigned(attrs: &[Hdf5Attribute], name: &str, unsigned: bool) -> Option<i64> {
    let a = attr_find(attrs, name)?;
    if a.datatype.class != 0 {
        return None;
    }
    decode_int_at(
        &a.data,
        0,
        a.datatype.size,
        a.datatype.endian,
        a.datatype.signed && !unsigned,
    )
}

fn attr_pair_int_unsigned(
    attrs: &[Hdf5Attribute],
    name: &str,
    unsigned: bool,
) -> Option<(i64, i64)> {
    let a = attr_find(attrs, name)?;
    if a.datatype.class != 0 || a.data.len() < 2 * a.datatype.size {
        return None;
    }
    let signed = a.datatype.signed && !unsigned;
    let lo = decode_int_at(&a.data, 0, a.datatype.size, a.datatype.endian, signed)?;
    let hi = decode_int_at(
        &a.data,
        a.datatype.size,
        a.datatype.size,
        a.datatype.endian,
        signed,
    )?;
    Some((lo, hi))
}

fn attr_number(attrs: &[Hdf5Attribute], name: &str) -> Option<f64> {
    let a = attr_find(attrs, name)?;
    match a.datatype.class {
        0 => attr_int(attrs, name).map(|v| v as f64),
        1 => match a.datatype.size {
            4 => decode_float_at(&a.data, 0, 4, a.datatype.endian),
            8 => decode_float_at(&a.data, 0, 8, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
}

fn attr_unsigned(attrs: &[Hdf5Attribute]) -> bool {
    attr_int(attrs, "_Unsigned").is_some_and(|v| v != 0)
}

fn attr_string(attrs: &[Hdf5Attribute], name: &str) -> Option<String> {
    let a = attr_find(attrs, name)?;
    let is_str = a.datatype.class == 3 || (a.datatype.class == 9 && a.datatype.vlen_is_string);
    if !is_str {
        return None;
    }
    let mut bytes = a.data.clone();
    while bytes.last().is_some_and(|&b| b == 0 || b == b' ') {
        bytes.pop();
    }
    String::from_utf8(bytes).ok()
}

fn units_epoch_unix(units: &str) -> Option<f64> {
    let rest = units.strip_prefix("seconds since ")?;
    let rb = rest.as_bytes();
    if rb.len() < 19 {
        return None;
    }
    if rb.get(4) != Some(&b'-') || rb.get(7) != Some(&b'-') {
        return None;
    }
    let year: i64 = rest.get(0..4)?.parse().ok()?;
    let month: i64 = rest.get(5..7)?.parse().ok()?;
    let day: i64 = rest.get(8..10)?.parse().ok()?;
    if rb.get(10) != Some(&b' ') || rb.get(13) != Some(&b':') || rb.get(16) != Some(&b':') {
        return None;
    }
    let hour: i64 = rest.get(11..13)?.parse().ok()?;
    let minute: i64 = rest.get(14..16)?.parse().ok()?;
    let sec: f64 = rest.get(17..)?.trim_end().parse().ok()?;
    if !(0..24).contains(&hour) || !(0..60).contains(&minute) || !(0.0..61.0).contains(&sec) {
        return None;
    }
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + sec)
}

fn fill_valid_int_ok(raw: i64, fill: Option<i64>, valid: Option<(i64, i64)>) -> bool {
    if fill.is_some_and(|f| raw == f) {
        return false;
    }
    if let Some((lo, hi)) = valid
        && (raw < lo || raw > hi)
    {
        return false;
    }
    true
}

fn scaled(raw: f64, scale: Option<f64>, offset: Option<f64>) -> Option<f64> {
    let mut v = raw;
    if let Some(s) = scale {
        v *= s;
    }
    if let Some(o) = offset {
        v += o;
    }
    if v.is_finite() { Some(v) } else { None }
}

fn flash_records(file: &Hdf5File, lsk: &LeapSeconds, src: &str) -> (Vec<GeoRec>, u64) {
    let Some(lat_v) = dataset_load(file, "flash_lat") else {
        eprintln!("glm_l2: {} carries no flash_lat", src);
        return (Vec::new(), 0);
    };
    let Some(lon_v) = dataset_load(file, "flash_lon") else {
        eprintln!("glm_l2: {} carries no flash_lon", src);
        return (Vec::new(), 0);
    };
    let Some(energy_v) = dataset_load(file, "flash_energy") else {
        eprintln!("glm_l2: {} carries no flash_energy", src);
        return (Vec::new(), 0);
    };
    let Some(time_v) = dataset_load(file, "flash_time_offset_of_first_event") else {
        eprintln!("glm_l2: {} carries no flash_time_offset_of_first_event", src);
        return (Vec::new(), 0);
    };
    let Some(qf_v) = dataset_load(file, "flash_quality_flag") else {
        eprintln!("glm_l2: {} carries no flash_quality_flag", src);
        return (Vec::new(), 0);
    };
    let Some(time_attrs) = dataset_attrs(file, "flash_time_offset_of_first_event") else {
        eprintln!(
            "glm_l2: {} carries no attributes on flash_time_offset_of_first_event",
            src
        );
        return (Vec::new(), 0);
    };
    let time_unsigned = attr_unsigned(time_attrs);
    let Some(units) = attr_string(time_attrs, "units") else {
        eprintln!(
            "glm_l2: {} carries no units string on flash_time_offset_of_first_event — no epoch, no records",
            src
        );
        return (Vec::new(), 0);
    };
    let Some(epoch_unix) = units_epoch_unix(&units) else {
        eprintln!(
            "glm_l2: {} units '{units}' parses void — no epoch, no records",
            src
        );
        return (Vec::new(), 0);
    };
    let time_scale = attr_number(time_attrs, "scale_factor");
    let time_offset = attr_number(time_attrs, "add_offset");
    let time_fill = attr_int_unsigned(time_attrs, "_FillValue", time_unsigned);
    let time_valid = attr_pair_int_unsigned(time_attrs, "valid_range", time_unsigned);
    let energy_attrs = dataset_attrs(file, "flash_energy");
    let energy_unsigned = energy_attrs.is_some_and(attr_unsigned);
    let energy_scale = energy_attrs.and_then(|a| attr_number(a, "scale_factor"));
    let energy_offset = energy_attrs.and_then(|a| attr_number(a, "add_offset"));
    let energy_fill =
        energy_attrs.and_then(|a| attr_int_unsigned(a, "_FillValue", energy_unsigned));
    let energy_valid =
        energy_attrs.and_then(|a| attr_pair_int_unsigned(a, "valid_range", energy_unsigned));
    let qf_attrs = dataset_attrs(file, "flash_quality_flag");
    let qf_fill = qf_attrs.and_then(|a| attr_int(a, "_FillValue"));
    let n = lat_v.n.min(lon_v.n).min(energy_v.n).min(time_v.n).min(qf_v.n);
    let mut out = Vec::new();
    let mut degraded = 0u64;
    for j in 0..n {
        let Some(qf) = decode_int_at(&qf_v.raw, j * qf_v.size, qf_v.size, qf_v.endian, qf_v.signed)
        else {
            continue;
        };
        if qf_fill.is_some_and(|f| qf == f) {
            continue;
        }
        if qf != 0 {
            degraded += 1;
            continue;
        }
        let Some(raw_energy) = decode_int_at(
            &energy_v.raw,
            j * energy_v.size,
            energy_v.size,
            energy_v.endian,
            energy_v.signed && !energy_unsigned,
        ) else {
            continue;
        };
        if !fill_valid_int_ok(raw_energy, energy_fill, energy_valid) {
            continue;
        }
        let Some(raw_time) = decode_int_at(
            &time_v.raw,
            j * time_v.size,
            time_v.size,
            time_v.endian,
            time_v.signed && !time_unsigned,
        ) else {
            continue;
        };
        if !fill_valid_int_ok(raw_time, time_fill, time_valid) {
            continue;
        }
        let Some(lat) = decode_float_at(&lat_v.raw, j * lat_v.size, lat_v.size, lat_v.endian)
        else {
            continue;
        };
        if !lat.is_finite() || lat.abs() > 90.0 {
            continue;
        }
        let Some(lon) = decode_float_at(&lon_v.raw, j * lon_v.size, lon_v.size, lon_v.endian)
        else {
            continue;
        };
        if !lon.is_finite() || lon.abs() > 180.0 {
            continue;
        }
        let Some(energy) = scaled(raw_energy as f64, energy_scale, energy_offset) else {
            continue;
        };
        if !energy.is_finite() || energy <= 0.0 {
            continue;
        }
        let Some(tsec) = scaled(raw_time as f64, time_scale, time_offset) else {
            continue;
        };
        if !tsec.is_finite() {
            continue;
        }
        let Some(tdb) = lsk.unix_to_tdb(epoch_unix + tsec) else {
            continue;
        };
        out.push(GeoRec {
            t: tdb,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: energy,
            comp: COMP_GLML2_FLASH_ENERGY,
            station: 0,
        });
    }
    (out, degraded)
}

fn run(out_path: &str, granules: &[String], lsk: &LeapSeconds, ci: bool) -> Result<(), String> {
    let mut records: Vec<GeoRec> = Vec::new();
    for src in granules {
        let bytes = granule_bytes(src).ok_or(format!("{src} stayed unreadable"))?;
        let file = Hdf5File::parse(&bytes).map_err(|e| format!("{src} parses void ({e:?})"))?;
        let (recs, degraded) = flash_records(&file, lsk, src);
        eprintln!(
            "glm_l2: {} → {} flashes ({} quality-degraded skipped)",
            src,
            recs.len(),
            degraded
        );
        records.extend(recs);
    }
    if records.is_empty() {
        return Err("no flashes harvested — the bin stays unwritten (0 honored)".into());
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bytes = write_bin(MAGIC_GLML2, &records);
    std::fs::write(out_path, &bytes).map_err(|e| format!("{out_path}: {e}"))?;
    match parse_bin(MAGIC_GLML2, &bytes) {
        Some(parsed) => eprintln!(
            "{}: {} flashes, {} B, roundtrip parses",
            out_path,
            parsed.len(),
            bytes.len()
        ),
        None => {
            return Err(format!(
                "{out_path}: roundtrip parse void — the bin stays unverified"
            ));
        }
    }
    if ci && !upload_release(NETLOC, out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!(
                "glm_l2_compiler: --out <file.bin> absent — the output path is never silent"
            );
            std::process::exit(1);
        }
    };
    let ci = args.iter().any(|a| a == "--ci-mode");
    let granules: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == "--granule")
        .filter_map(|(i, _)| args.get(i + 1))
        .cloned()
        .collect();
    if granules.is_empty() {
        eprintln!("glm_l2_compiler: --granule <url|path> absent — refused");
        std::process::exit(1);
    }
    let lsk_path = match arg_value(&args, "--lsk") {
        Some(p) => p,
        None => {
            eprintln!("glm_l2_compiler: --lsk <naif0012.tls> absent — the TDB clock stays unread");
            std::process::exit(1);
        }
    };
    let lsk_text = match std::fs::read_to_string(&lsk_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "glm_l2_compiler: {} unreadable: {e} — the TDB clock stays unread",
                lsk_path
            );
            std::process::exit(1);
        }
    };
    let Some(lsk) = parse_lsk(&lsk_text) else {
        eprintln!(
            "glm_l2_compiler: {} parses void — the leap table stays unread",
            lsk_path
        );
        std::process::exit(1);
    };
    if let Err(msg) = run(&out, &granules, &lsk, ci) {
        eprintln!("glm_l2_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_epoch_parses_valid_and_rejects_malformed() {
        assert_eq!(
            units_epoch_unix("seconds since 2026-01-01 00:00:00.000"),
            Some(1767225600.0)
        );
        assert_eq!(
            units_epoch_unix("seconds since 2026-01-01 00:00:00.500"),
            Some(1767225600.5)
        );
        assert!(units_epoch_unix("seconds since not-a-date").is_none());
        assert!(units_epoch_unix("hours since 2026-01-01 00:00:00").is_none());
        assert!(units_epoch_unix("seconds since 2026-01-01T00:00:00").is_none());
        assert!(units_epoch_unix("").is_none());
    }

    #[test]
    fn unsigned_int16_decodes_raw_high_bit() {
        let raw = [0x00, 0x80];
        assert_eq!(decode_int_at(&raw, 0, 2, Endian::Le, false), Some(32768));
        assert_eq!(decode_int_at(&raw, 0, 2, Endian::Le, true), Some(-32768));
    }

    #[test]
    fn fill_and_valid_range_gate_skips() {
        assert!(!fill_valid_int_ok(65535, Some(65535), Some((0, 65530))));
        assert!(!fill_valid_int_ok(65531, Some(65535), Some((0, 65530))));
        assert!(fill_valid_int_ok(65530, Some(65535), Some((0, 65530))));
        assert!(fill_valid_int_ok(0, Some(65535), Some((0, 65530))));
        assert!(fill_valid_int_ok(12345, None, None));
    }
}
