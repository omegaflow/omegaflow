use super::*;
use crate::archivar::fetch::PresenceSample;
use crate::archivar::hdf5::{Endian, Hdf5Attribute, Hdf5File};

pub const COMP_RADIANCE: u32 = 0;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_RADIANCE => Some("mtg_li_flash_radiance_mw_m2_sr"),
        _ => None,
    }
}

const DS_LAT: &str = "latitude";
const DS_LON: &str = "longitude";
const DS_TIME: &str = "flash_time";

pub struct MtgFlash {
    pub t_unix: f64,
    pub lat: f64,
    pub lon: f64,
    pub value: f64,
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

fn decode_int(data: &[u8], off: usize, size: usize, endian: Endian, signed: bool) -> Option<i64> {
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

fn decode_float(data: &[u8], off: usize, size: usize, endian: Endian) -> Option<f64> {
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
        0 => decode_int(&var.raw, off, var.size, var.endian, var.signed && !unsigned)
            .map(|v| v as f64),
        1 => decode_float(&var.raw, off, var.size, var.endian),
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
        if let Some(f) = gate.fill
            && raw as i64 == f
        {
            return None;
        }
        if let Some((lo, hi)) = gate.valid
            && (raw < lo as f64 || raw > hi as f64)
        {
            return None;
        }
    } else if !raw.is_finite() {
        return None;
    }
    let mut v = raw;
    if let Some(s) = gate.scale {
        v *= s;
    }
    if let Some(o) = gate.offset {
        v += o;
    }
    if v.is_finite() { Some(v) } else { None }
}

fn attr_find<'a>(attrs: &'a [Hdf5Attribute], name: &str) -> Option<&'a Hdf5Attribute> {
    attrs.iter().find(|a| a.name == name)
}

fn attr_int(attrs: &[Hdf5Attribute], name: &str) -> Option<i64> {
    let a = attr_find(attrs, name)?;
    if a.datatype.class != 0 {
        return None;
    }
    decode_int(
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
    decode_int(
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
    let lo = decode_int(&a.data, 0, a.datatype.size, a.datatype.endian, signed)?;
    let hi = decode_int(
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
            4 => decode_float(&a.data, 0, 4, a.datatype.endian),
            8 => decode_float(&a.data, 0, 8, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
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

fn attr_unsigned(attrs: &[Hdf5Attribute]) -> bool {
    attr_int(attrs, "_Unsigned").is_some_and(|v| v != 0)
        || attr_string(attrs, "_Unsigned").is_some_and(|s| s.eq_ignore_ascii_case("true"))
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
    let month: u32 = rest.get(5..7)?.parse().ok()?;
    let day: u32 = rest.get(8..10)?.parse().ok()?;
    if rb.get(10) != Some(&b' ') || rb.get(13) != Some(&b':') || rb.get(16) != Some(&b':') {
        return None;
    }
    let hour: i64 = rest.get(11..13)?.parse().ok()?;
    let minute: i64 = rest.get(14..16)?.parse().ok()?;
    let sec: f64 = rest.get(17..)?.trim_end().parse().ok()?;
    if !(0..24).contains(&hour) || !(0..60).contains(&minute) || !(0.0..61.0).contains(&sec) {
        return None;
    }
    let days = crate::archivar::ymd_to_days(year, month, day)?;
    Some(days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + sec)
}

fn gate_of(attrs: Option<&[Hdf5Attribute]>) -> NumGate {
    let unsigned = attrs.is_some_and(attr_unsigned);
    NumGate {
        unsigned,
        fill: attrs.and_then(|a| attr_int_unsigned(a, "_FillValue", unsigned)),
        valid: attrs.and_then(|a| attr_pair_int_unsigned(a, "valid_range", unsigned)),
        scale: attrs.and_then(|a| attr_number(a, "scale_factor")),
        offset: attrs.and_then(|a| attr_number(a, "add_offset")),
    }
}

pub fn parse_flashes(bytes: &[u8], value_key: &str) -> Option<Vec<MtgFlash>> {
    let bytes = if bytes.starts_with(&[0x1f, 0x8b]) {
        gunzip(bytes)?
    } else {
        bytes.to_vec()
    };
    let file = Hdf5File::parse(&bytes).ok()?;
    let lat_v = dataset_load(&file, DS_LAT)?;
    let lon_v = dataset_load(&file, DS_LON)?;
    let time_v = dataset_load(&file, DS_TIME)?;
    let value_v = dataset_load(&file, value_key)?;
    let time_attrs = dataset_attrs(&file, DS_TIME)?;
    let units = attr_string(time_attrs, "units")?;
    let epoch_unix = units_epoch_unix(&units)?;
    let lat_gate = gate_of(dataset_attrs(&file, DS_LAT));
    let lon_gate = gate_of(dataset_attrs(&file, DS_LON));
    let value_gate = gate_of(dataset_attrs(&file, value_key));
    let n = lat_v.n.min(lon_v.n).min(time_v.n).min(value_v.n);
    let mut out = Vec::new();
    for j in 0..n {
        let Some(tsec) = decode_num_at(&time_v, j, false) else {
            continue;
        };
        if !tsec.is_finite() {
            continue;
        }
        let Some(lat) = gated_value(&lat_v, &lat_gate, j) else {
            continue;
        };
        if lat.abs() > 90.0 {
            continue;
        }
        let Some(lon) = gated_value(&lon_v, &lon_gate, j) else {
            continue;
        };
        if lon.abs() > 180.0 {
            continue;
        }
        let Some(value) = gated_value(&value_v, &value_gate, j) else {
            continue;
        };
        if value <= 0.0 {
            continue;
        }
        out.push(MtgFlash {
            t_unix: epoch_unix + tsec,
            lat,
            lon,
            value,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn build_channels(
    src: &SourceConfig,
    bytes: &[u8],
    lsk: &LeapSeconds,
    now: f64,
    presences: &[PresenceSample],
    body_radius: Option<f64>,
    eph: &HashMap<String, BodyEphemeris>,
) -> Vec<(Channel, FieldConfig)> {
    let body_name = frame_body_name(&src.frame);
    let body_props = eph.get(body_name.as_str()).and_then(|e| e.props.as_ref());
    let body_medium = eph.get(body_name.as_str()).and_then(|e| e.medium.as_ref());
    let fields: Vec<&FieldConfig> = src
        .extracts
        .iter()
        .filter_map(|e| match e {
            Extract::Field(fc) => Some(fc),
            _ => None,
        })
        .collect();
    let mut channels = Vec::new();
    for fc in fields {
        let Some(flashes) = parse_flashes(bytes, &fc.key) else {
            continue;
        };
        for f in flashes {
            let Some(epoch) = lsk.unix_to_tdb(f.t_unix) else {
                continue;
            };
            let alt = 0.0;
            let motion = Motion::Surface {
                body_name: body_name.clone(),
                lat: f.lat,
                lon: f.lon,
                alt,
            };
            let mut keep = true;
            if let Some((anchor_vmax, anchor_amax, _)) = law_bounds(&motion, epoch, 0.0, eph) {
                keep = record_in_enclosure(
                    presences,
                    body_fixed_to_icrs(&body_name, f.lat, f.lon, alt, epoch, eph),
                    epoch,
                    now,
                    EnclosureField {
                        config: fc,
                        body_props,
                        medium: body_medium,
                        body_radius,
                    },
                    AnchorEnvelope {
                        vmax: anchor_vmax,
                        amax: anchor_amax,
                        pad: 0.0,
                        ttl: src.ttl as f64,
                    },
                );
            }
            if !keep {
                continue;
            }
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch,
                    position: Position::Surface {
                        body_name: body_name.clone(),
                        lat: f.lat,
                        lon: f.lon,
                        alt,
                    },
                    station_code: src.station_code.clone(),
                    name: fc.name.clone(),
                    value: f.value,
                },
                fc.clone(),
            ));
        }
    }
    channels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_epoch_reads_the_measured_mtg_time_base() {
        assert_eq!(
            units_epoch_unix("seconds since 2000-01-01 00:00:00.0"),
            Some(946_684_800.0)
        );
    }

    #[test]
    fn units_epoch_refuses_a_foreign_base() {
        assert!(units_epoch_unix("days since 2000-01-01").is_none());
        assert!(units_epoch_unix("seconds since").is_none());
    }

    #[test]
    fn component_name_maps_the_flash_radiance() {
        assert_eq!(
            component_name(COMP_RADIANCE),
            Some("mtg_li_flash_radiance_mw_m2_sr")
        );
        assert_eq!(component_name(7), None);
    }
}
