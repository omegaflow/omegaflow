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
    class: u8,
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
        class: dt.class,
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

fn decode_num_at(var: &VarLoad, idx: usize, unsigned: bool) -> Option<f64> {
    let off = idx.checked_mul(var.size)?;
    match var.class {
        0 => decode_int_at(&var.raw, off, var.size, var.endian, var.signed && !unsigned)
            .map(|v| v as f64),
        1 => decode_float_at(&var.raw, off, var.size, var.endian),
        _ => None,
    }
}

struct NumGate {
    unsigned: bool,
    fill: Option<i64>,
    valid: Option<(i64, i64)>,
    scale: Option<f64>,
    offset: Option<f64>,
}

fn gated_value(var: &VarLoad, gate: &NumGate, idx: usize) -> Option<f64> {
    let raw = decode_num_at(var, idx, gate.unsigned)?;
    if var.class == 0 {
        if !fill_valid_int_ok(raw as i64, gate.fill, gate.valid) {
            return None;
        }
    } else if !raw.is_finite() {
        return None;
    }
    scaled(raw, gate.scale, gate.offset)
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
        || attr_string(attrs, "_Unsigned").is_some_and(|s| s.eq_ignore_ascii_case("true"))
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

struct GateSkips {
    qf: u64,
    degraded: u64,
    energy: u64,
    time: u64,
    lat: u64,
    lon: u64,
    tdb: u64,
}

impl GateSkips {
    fn zero() -> GateSkips {
        GateSkips {
            qf: 0,
            degraded: 0,
            energy: 0,
            time: 0,
            lat: 0,
            lon: 0,
            tdb: 0,
        }
    }
}

fn flash_records(file: &Hdf5File, lsk: &LeapSeconds, src: &str) -> (Vec<GeoRec>, GateSkips) {
    let Some(lat_v) = dataset_load(file, "flash_lat") else {
        eprintln!("glm_l2: {} carries no flash_lat", src);
        return (Vec::new(), GateSkips::zero());
    };
    let Some(lon_v) = dataset_load(file, "flash_lon") else {
        eprintln!("glm_l2: {} carries no flash_lon", src);
        return (Vec::new(), GateSkips::zero());
    };
    let Some(energy_v) = dataset_load(file, "flash_energy") else {
        eprintln!("glm_l2: {} carries no flash_energy", src);
        return (Vec::new(), GateSkips::zero());
    };
    let Some(time_v) = dataset_load(file, "flash_time_offset_of_first_event") else {
        eprintln!(
            "glm_l2: {} carries no flash_time_offset_of_first_event",
            src
        );
        return (Vec::new(), GateSkips::zero());
    };
    let Some(qf_v) = dataset_load(file, "flash_quality_flag") else {
        eprintln!("glm_l2: {} carries no flash_quality_flag", src);
        return (Vec::new(), GateSkips::zero());
    };
    let Some(time_attrs) = dataset_attrs(file, "flash_time_offset_of_first_event") else {
        eprintln!(
            "glm_l2: {} carries no attributes on flash_time_offset_of_first_event",
            src
        );
        return (Vec::new(), GateSkips::zero());
    };
    let time_unsigned = attr_unsigned(time_attrs);
    let Some(units) = attr_string(time_attrs, "units") else {
        eprintln!(
            "glm_l2: {} carries no units string on flash_time_offset_of_first_event — no epoch, no records",
            src
        );
        return (Vec::new(), GateSkips::zero());
    };
    let Some(epoch_unix) = units_epoch_unix(&units) else {
        eprintln!(
            "glm_l2: {} units '{units}' parses void — no epoch, no records",
            src
        );
        return (Vec::new(), GateSkips::zero());
    };
    let time_gate = NumGate {
        unsigned: time_unsigned,
        fill: attr_int_unsigned(time_attrs, "_FillValue", time_unsigned),
        valid: attr_pair_int_unsigned(time_attrs, "valid_range", time_unsigned),
        scale: attr_number(time_attrs, "scale_factor"),
        offset: attr_number(time_attrs, "add_offset"),
    };
    let energy_attrs = dataset_attrs(file, "flash_energy");
    let energy_unsigned = energy_attrs.is_some_and(attr_unsigned);
    let energy_gate = NumGate {
        unsigned: energy_unsigned,
        fill: energy_attrs.and_then(|a| attr_int_unsigned(a, "_FillValue", energy_unsigned)),
        valid: energy_attrs.and_then(|a| attr_pair_int_unsigned(a, "valid_range", energy_unsigned)),
        scale: energy_attrs.and_then(|a| attr_number(a, "scale_factor")),
        offset: energy_attrs.and_then(|a| attr_number(a, "add_offset")),
    };
    let lat_attrs = dataset_attrs(file, "flash_lat");
    let lat_unsigned = lat_attrs.is_some_and(attr_unsigned);
    let lat_gate = NumGate {
        unsigned: lat_unsigned,
        fill: lat_attrs.and_then(|a| attr_int_unsigned(a, "_FillValue", lat_unsigned)),
        valid: lat_attrs.and_then(|a| attr_pair_int_unsigned(a, "valid_range", lat_unsigned)),
        scale: lat_attrs.and_then(|a| attr_number(a, "scale_factor")),
        offset: lat_attrs.and_then(|a| attr_number(a, "add_offset")),
    };
    let lon_attrs = dataset_attrs(file, "flash_lon");
    let lon_unsigned = lon_attrs.is_some_and(attr_unsigned);
    let lon_gate = NumGate {
        unsigned: lon_unsigned,
        fill: lon_attrs.and_then(|a| attr_int_unsigned(a, "_FillValue", lon_unsigned)),
        valid: lon_attrs.and_then(|a| attr_pair_int_unsigned(a, "valid_range", lon_unsigned)),
        scale: lon_attrs.and_then(|a| attr_number(a, "scale_factor")),
        offset: lon_attrs.and_then(|a| attr_number(a, "add_offset")),
    };
    let qf_attrs = dataset_attrs(file, "flash_quality_flag");
    let qf_fill = qf_attrs.and_then(|a| attr_int(a, "_FillValue"));
    for (name, var) in [
        ("flash_lat", &lat_v),
        ("flash_lon", &lon_v),
        ("flash_energy", &energy_v),
        ("flash_time_offset_of_first_event", &time_v),
        ("flash_quality_flag", &qf_v),
    ] {
        if var.raw.len() < var.n.saturating_mul(var.size) {
            eprintln!(
                "glm_l2: {} {name} reads {} B < {} × {} B — the dataset reads short",
                src,
                var.raw.len(),
                var.n,
                var.size
            );
            return (Vec::new(), GateSkips::zero());
        }
    }
    let n = lat_v
        .n
        .min(lon_v.n)
        .min(energy_v.n)
        .min(time_v.n)
        .min(qf_v.n);
    let mut out = Vec::new();
    let mut skips = GateSkips::zero();
    for j in 0..n {
        let Some(qf) = decode_num_at(&qf_v, j, false) else {
            skips.qf += 1;
            continue;
        };
        let qf_is_fill = if qf_v.class == 0 {
            qf_fill.is_some_and(|f| qf as i64 == f)
        } else {
            !qf.is_finite()
        };
        if qf_is_fill {
            skips.qf += 1;
            continue;
        }
        if qf != 0.0 {
            skips.degraded += 1;
            continue;
        }
        let Some(energy) = gated_value(&energy_v, &energy_gate, j) else {
            skips.energy += 1;
            continue;
        };
        if !(energy > 0.0) {
            skips.energy += 1;
            continue;
        }
        let Some(tsec) = gated_value(&time_v, &time_gate, j) else {
            skips.time += 1;
            continue;
        };
        if !tsec.is_finite() {
            skips.time += 1;
            continue;
        }
        let Some(lat) = gated_value(&lat_v, &lat_gate, j) else {
            skips.lat += 1;
            continue;
        };
        if !lat.is_finite() || lat.abs() > 90.0 {
            skips.lat += 1;
            continue;
        }
        let Some(lon) = gated_value(&lon_v, &lon_gate, j) else {
            skips.lon += 1;
            continue;
        };
        if !lon.is_finite() || lon.abs() > 180.0 {
            skips.lon += 1;
            continue;
        }
        let Some(tdb) = lsk.unix_to_tdb(epoch_unix + tsec) else {
            skips.tdb += 1;
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
    (out, skips)
}

fn run(out_path: &str, granules: &[String], lsk: &LeapSeconds, ci: bool) -> Result<(), String> {
    let mut records: Vec<GeoRec> = Vec::new();
    for src in granules {
        let bytes = granule_bytes(src).ok_or(format!("{src} stayed unreadable"))?;
        let file = Hdf5File::parse(&bytes).map_err(|e| format!("{src} parses void ({e:?})"))?;
        let (recs, skips) = flash_records(&file, lsk, src);
        eprintln!(
            "glm_l2: {} → {} flashes ({} degraded; skipped qf {} energy {} time {} lat {} lon {} tdb {})",
            src,
            recs.len(),
            skips.degraded,
            skips.qf,
            skips.energy,
            skips.time,
            skips.lat,
            skips.lon,
            skips.tdb
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
            eprintln!("glm_l2_compiler: --out <file.bin> absent — the output path is never silent");
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
    use omegaflow::hdf5::{Hdf5Dataspace, Hdf5Datatype};

    fn string_attr(name: &str, value: &str) -> Hdf5Attribute {
        Hdf5Attribute {
            name: name.to_string(),
            datatype: Hdf5Datatype {
                class: 3,
                size: value.len(),
                endian: Endian::Le,
                signed: false,
                bit_offset: 0,
                precision: 0,
                string_pad: 0,
                string_charset: 0,
                members: Vec::new(),
                array_dims: Vec::new(),
                base: None,
                reference_type: 0,
                vlen_is_string: false,
            },
            dataspace: Hdf5Dataspace { dims: Vec::new() },
            data: value.as_bytes().to_vec(),
        }
    }

    #[test]
    fn unsigned_attr_reads_the_string_true_form() {
        assert!(attr_unsigned(&[string_attr("_Unsigned", "true")]));
        assert!(!attr_unsigned(&[string_attr("_Unsigned", "false")]));
        assert!(!attr_unsigned(&[]));
    }

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

    fn num_gate(scale: Option<f64>, offset: Option<f64>) -> NumGate {
        NumGate {
            unsigned: false,
            fill: None,
            valid: None,
            scale,
            offset,
        }
    }

    #[test]
    fn float32_energy_decodes_as_float_not_int_bits() {
        let raw = 1.5f32.to_le_bytes().to_vec();
        let float_var = VarLoad {
            raw: raw.clone(),
            size: 4,
            endian: Endian::Le,
            signed: true,
            class: 1,
            n: 1,
        };
        assert_eq!(gated_value(&float_var, &num_gate(None, None), 0), Some(1.5));
        let int_var = VarLoad {
            raw,
            size: 4,
            endian: Endian::Le,
            signed: false,
            class: 0,
            n: 1,
        };
        assert_eq!(
            gated_value(&int_var, &num_gate(None, None), 0),
            Some(0x3FC0_0000u32 as f64)
        );
    }

    #[test]
    fn float64_time_offset_decodes_as_seconds() {
        let var = VarLoad {
            raw: 820_497_600.0f64.to_le_bytes().to_vec(),
            size: 8,
            endian: Endian::Le,
            signed: true,
            class: 1,
            n: 1,
        };
        assert_eq!(
            gated_value(&var, &num_gate(None, None), 0),
            Some(820_497_600.0)
        );
    }

    #[test]
    fn non_finite_float_is_gated_not_compared() {
        let var = VarLoad {
            raw: f32::NAN.to_le_bytes().to_vec(),
            size: 4,
            endian: Endian::Le,
            signed: true,
            class: 1,
            n: 1,
        };
        assert_eq!(gated_value(&var, &num_gate(None, None), 0), None);
    }

    #[test]
    fn int16_latlon_scales_through_the_gate() {
        let var = VarLoad {
            raw: 3380i16.to_le_bytes().to_vec(),
            size: 2,
            endian: Endian::Le,
            signed: true,
            class: 0,
            n: 1,
        };
        assert_eq!(
            gated_value(&var, &num_gate(Some(0.01), None), 0),
            Some(33.8)
        );
    }
}
