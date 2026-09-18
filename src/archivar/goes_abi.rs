use super::hdf5::{
    Endian, GeostationaryProjection, Hdf5Attribute, Hdf5Datatype, Hdf5File, decode_f32, decode_f64,
};

pub mod s3;

pub const MAGIC: [u8; 4] = *b"GAB1";
pub const VERSION: u8 = 1;
pub const HEADER_BYTES: usize = 12;
pub const REC_BYTES: usize = 56;

pub const CALIB_L1B: u8 = 0;
pub const CALIB_GSICS_PENDING: u8 = 1;
pub const CALIB_GSICS: u8 = 2;

pub const COMP_RADIANCE: u32 = 0;

#[derive(Clone, Debug)]
pub struct AbiGranule {
    pub t: f64,
    pub band_id: u8,
    pub calib: u8,
    pub band_wavelength: f32,
    pub esun: f32,
    pub kappa0: f32,
    pub sub_lon: f32,
    pub persp_h: f32,
    pub rad_mean: f32,
    pub rad_std: f32,
    pub rad_min: f32,
    pub rad_max: f32,
    pub valid: u32,
    pub total: u32,
}

pub fn calib_name(calib: u8) -> &'static str {
    match calib {
        CALIB_L1B => "L1b",
        CALIB_GSICS => "gsics",
        CALIB_GSICS_PENDING => "gsics-pending",
        _ => "unknown",
    }
}

fn calib_known(calib: u8) -> bool {
    matches!(calib, CALIB_L1B | CALIB_GSICS_PENDING | CALIB_GSICS)
}

pub fn write_bin(records: &[AbiGranule]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&[0u8; 3]);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for g in records {
        if !g.t.is_finite()
            || !g.band_wavelength.is_finite()
            || !g.esun.is_finite()
            || !g.kappa0.is_finite()
            || !g.sub_lon.is_finite()
            || !g.persp_h.is_finite()
            || !g.rad_mean.is_finite()
            || g.rad_mean <= 0.0
            || !g.rad_std.is_finite()
            || !g.rad_min.is_finite()
            || !g.rad_max.is_finite()
            || !calib_known(g.calib)
        {
            return None;
        }
        let mut rec = [0u8; REC_BYTES];
        rec[0..8].copy_from_slice(&g.t.to_le_bytes());
        rec[8] = g.band_id;
        rec[9] = g.calib;
        rec[12..16].copy_from_slice(&g.band_wavelength.to_le_bytes());
        rec[16..20].copy_from_slice(&g.esun.to_le_bytes());
        rec[20..24].copy_from_slice(&g.kappa0.to_le_bytes());
        rec[24..28].copy_from_slice(&g.sub_lon.to_le_bytes());
        rec[28..32].copy_from_slice(&g.persp_h.to_le_bytes());
        rec[32..36].copy_from_slice(&g.rad_mean.to_le_bytes());
        rec[36..40].copy_from_slice(&g.rad_std.to_le_bytes());
        rec[40..44].copy_from_slice(&g.rad_min.to_le_bytes());
        rec[44..48].copy_from_slice(&g.rad_max.to_le_bytes());
        rec[48..52].copy_from_slice(&g.valid.to_le_bytes());
        rec[52..56].copy_from_slice(&g.total.to_le_bytes());
        out.extend_from_slice(&rec);
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<AbiGranule>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    let n = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let rec = bytes.get(off..off + REC_BYTES)?;
        off += REC_BYTES;
        let f32_at = |r: std::ops::Range<usize>| {
            rec.get(r)
                .and_then(|x| x.try_into().ok())
                .map(f32::from_le_bytes)
        };
        let g = AbiGranule {
            t: f64::from_le_bytes(rec[0..8].try_into().ok()?),
            band_id: rec[8],
            calib: rec[9],
            band_wavelength: f32_at(12..16)?,
            esun: f32_at(16..20)?,
            kappa0: f32_at(20..24)?,
            sub_lon: f32_at(24..28)?,
            persp_h: f32_at(28..32)?,
            rad_mean: f32_at(32..36)?,
            rad_std: f32_at(36..40)?,
            rad_min: f32_at(40..44)?,
            rad_max: f32_at(44..48)?,
            valid: u32::from_le_bytes(rec[48..52].try_into().ok()?),
            total: u32::from_le_bytes(rec[52..56].try_into().ok()?),
        };
        if !calib_known(g.calib)
            || !g.t.is_finite()
            || !g.band_wavelength.is_finite()
            || !g.rad_mean.is_finite()
            || g.rad_mean <= 0.0
        {
            return None;
        }
        out.push(g);
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_RADIANCE => Some("goes_abi_radiance"),
        _ => None,
    }
}

pub type GsicsTable = [Option<(f64, f64)>; 16];

pub fn gsics_lookup(table: &GsicsTable, band_id: u8) -> Option<(f64, f64)> {
    if band_id == 0 {
        return None;
    }
    table.get((band_id as usize) - 1).copied().flatten()
}

pub fn parse_gsics_txt(text: &str) -> GsicsTable {
    let mut table: GsicsTable = [None; 16];
    let mut goes16_first = false;
    for line in text.replace('\r', "\n").lines() {
        if !goes16_first {
            if let (Some(i16), Some(i18)) = (line.find("GOES-16"), line.find("GOES-18")) {
                if i16 < i18 {
                    goes16_first = true;
                } else {
                    return table;
                }
            }
            continue;
        }
        let cols: Vec<&str> = line.split('\t').map(|c| c.trim()).collect();
        if cols.len() < 3 {
            continue;
        }
        let Ok(channel) = cols[0].parse::<usize>() else {
            continue;
        };
        if !(1..=16).contains(&channel) {
            continue;
        }
        let Some(offset) = cols[1].parse::<f64>().ok().filter(|v| v.is_finite()) else {
            continue;
        };
        let Some(slope) = cols[2].parse::<f64>().ok().filter(|v| v.is_finite()) else {
            continue;
        };
        table[channel - 1] = Some((slope, offset));
    }
    table
}

fn attr_find<'a>(attrs: &'a [Hdf5Attribute], name: &str) -> Option<&'a Hdf5Attribute> {
    attrs.iter().find(|a| a.name == name)
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

fn attr_number(attrs: &[Hdf5Attribute], name: &str) -> Option<f64> {
    let a = attr_find(attrs, name)?;
    match a.datatype.class {
        0 => attr_int(attrs, name).map(|v| v as f64),
        1 => match a.datatype.size {
            4 => decode_f32(&a.data, 0, a.datatype.endian).map(|v| v as f64),
            8 => decode_f64(&a.data, 0, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
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

fn attr_int_pair_unsigned(
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

fn attr_unsigned(attrs: &[Hdf5Attribute]) -> bool {
    attr_int(attrs, "_Unsigned").is_some_and(|v| v != 0)
}

fn decode_count(raw: &[u8], i: usize, dt: &Hdf5Datatype, unsigned: bool) -> Option<i64> {
    decode_int_at(raw, i * dt.size, dt.size, dt.endian, dt.signed && !unsigned)
}

fn scalar_f64(file: &Hdf5File, name: &str) -> Option<f64> {
    let (obj, _ds, _dt) = file.dataset(name).ok()?;
    let fill = attr_number(&obj.attrs, "_FillValue");
    let v = file.read_f64_dataset(name).ok()?.first().copied()?;
    if !v.is_finite() {
        return None;
    }
    match fill {
        Some(f) if f.is_finite() && (v - f).abs() < 1e-9 => None,
        _ => Some(v),
    }
}

fn scalar_u8(file: &Hdf5File, name: &str) -> Option<u8> {
    file.read_dataset(name).ok()?.first().copied()
}

fn harmonization_coeff(file: &Hdf5File, name: &str) -> Option<f64> {
    let (obj, _ds, _dt) = file.dataset(name).ok()?;
    let fill = attr_number(&obj.attrs, "_FillValue");
    let v = file.read_f64_dataset(name).ok()?.first().copied()?;
    if !v.is_finite() {
        return None;
    }
    match fill {
        Some(f) if f.is_finite() && (v - f).abs() < 1e-6 => None,
        _ => Some(v),
    }
}

fn gsics_slope_offset(file: &Hdf5File) -> Option<(f64, f64)> {
    let offset = harmonization_coeff(file, "a_h_NRTH")?;
    let slope = harmonization_coeff(file, "b_h_NRTH")?;
    Some((slope, offset))
}

struct Calib {
    scale: f64,
    offset: f64,
}

const NO_ADD_OFFSET: f64 = 0.0;

fn stats_of(
    raw: &[u8],
    dt: &Hdf5Datatype,
    total: usize,
    unsigned: bool,
    fill: Option<i64>,
    valid_range: Option<(i64, i64)>,
    calib: Calib,
) -> (f64, f64, f64, f64, u32) {
    let mut sum = 0.0f64;
    let mut sumsq = 0.0f64;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut valid = 0u32;
    for i in 0..total {
        let Some(count) = decode_count(raw, i, dt, unsigned) else {
            continue;
        };
        if let Some(f) = fill
            && count == f
        {
            continue;
        }
        if let Some((lo, hi)) = valid_range
            && (count < lo || count > hi)
        {
            continue;
        }
        let rad = count as f64 * calib.scale + calib.offset;
        if !rad.is_finite() {
            continue;
        }
        sum += rad;
        sumsq += rad * rad;
        if rad < min {
            min = rad;
        }
        if rad > max {
            max = rad;
        }
        valid += 1;
    }
    (sum, sumsq, min, max, valid)
}

pub fn parse_granule(bytes: &[u8], gsics: Option<&GsicsTable>) -> Result<AbiGranule, String> {
    let file = Hdf5File::parse(bytes).map_err(|n| format!("hdf5 parse: {n:?}"))?;
    let (rad_obj, ds, dt) = file
        .dataset("Rad")
        .map_err(|n| format!("Rad absent: {n:?}"))?;
    if ds.dims.len() != 2 {
        return Err(format!(
            "Rad rank {} — not a [y,x] radiance field",
            ds.dims.len()
        ));
    }
    let total: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let raw = file
        .read_dataset("Rad")
        .map_err(|n| format!("Rad read: {n:?}"))?;
    if raw.len() != total * dt.size {
        return Err(format!(
            "Rad {} B read, {} expected — the field stays unread",
            raw.len(),
            total * dt.size
        ));
    }
    let unsigned = attr_unsigned(&rad_obj.attrs);
    let scale = attr_number(&rad_obj.attrs, "scale_factor")
        .ok_or_else(|| "Rad scale_factor absent".to_string())?;
    let offset = attr_number(&rad_obj.attrs, "add_offset").unwrap_or(NO_ADD_OFFSET);
    let fill = attr_int_unsigned(&rad_obj.attrs, "_FillValue", unsigned);
    let valid_range = attr_int_pair_unsigned(&rad_obj.attrs, "valid_range", unsigned);
    let (sum, sumsq, min, max, valid) = stats_of(
        &raw,
        dt,
        total,
        unsigned,
        fill,
        valid_range,
        Calib { scale, offset },
    );
    if valid == 0 {
        return Err(
            "Rad: no valid radiance pixel — the record stays unwritten (0 honored)".to_string(),
        );
    }
    let n = valid as f64;
    let mut mean = sum / n;
    let variance = (sumsq - sum * sum / n) / n;
    let mut std = if variance > 0.0 { variance.sqrt() } else { 0.0 };
    let mut min = min;
    let mut max = max;
    let band_id = scalar_u8(&file, "band_id").ok_or_else(|| "band_id absent".to_string())?;
    let external = gsics.and_then(|t| gsics_lookup(t, band_id));
    let calib = match external.or_else(|| gsics_slope_offset(&file)) {
        Some((slope, offset)) => {
            mean = slope * mean + offset;
            std *= slope.abs();
            let lo = slope * min + offset;
            let hi = slope * max + offset;
            let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
            min = lo;
            max = hi;
            CALIB_GSICS
        }
        None => CALIB_GSICS_PENDING,
    };
    let t = scalar_f64(&file, "t").ok_or_else(|| "t absent or void".to_string())?;
    let band_wavelength = scalar_f64(&file, "band_wavelength")
        .ok_or_else(|| "band_wavelength absent or void".to_string())?
        as f32;
    let esun = scalar_f64(&file, "esun").ok_or_else(|| "esun absent or void".to_string())? as f32;
    let kappa0 =
        scalar_f64(&file, "kappa0").ok_or_else(|| "kappa0 absent or void".to_string())? as f32;
    let proj: GeostationaryProjection = file
        .geostationary_projection()
        .map_err(|n| format!("projection absent: {n:?}"))?;
    Ok(AbiGranule {
        t,
        band_id,
        calib,
        band_wavelength,
        esun,
        kappa0,
        sub_lon: proj.sub_longitude_deg as f32,
        persp_h: proj.perspective_height_m as f32,
        rad_mean: mean as f32,
        rad_std: std as f32,
        rad_min: min as f32,
        rad_max: max as f32,
        valid,
        total: total as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn granule() -> AbiGranule {
        AbiGranule {
            t: 729_777_632.227_242_1,
            band_id: 1,
            calib: CALIB_GSICS,
            band_wavelength: 0.47,
            esun: 0.0,
            kappa0: 0.0,
            sub_lon: -75.0,
            persp_h: 35_786_000.0,
            rad_mean: 5.56,
            rad_std: 0.5,
            rad_min: 0.1,
            rad_max: 12.0,
            valid: 30_000_000,
            total: 30_250_000,
        }
    }

    #[test]
    fn roundtrip() {
        let g = granule();
        let bytes = write_bin(std::slice::from_ref(&g)).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        let b = &parsed[0];
        assert_eq!(b.t, g.t);
        assert_eq!(b.band_id, g.band_id);
        assert_eq!(b.calib, g.calib);
        assert_eq!(b.band_wavelength, g.band_wavelength);
        assert_eq!(b.sub_lon, g.sub_lon);
        assert_eq!(b.persp_h, g.persp_h);
        assert_eq!(b.rad_mean, g.rad_mean);
        assert_eq!(b.valid, g.valid);
        assert_eq!(b.total, g.total);
    }

    #[test]
    fn refuses_foreign_magic_and_truncated() {
        assert!(parse_bin(b"GKA1").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&[granule()]).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn refuses_unknown_calibration_and_nonpositive_radiance() {
        let mut g = granule();
        g.calib = 9;
        assert!(write_bin(&[g]).is_none());
        let mut g = granule();
        g.rad_mean = 0.0;
        assert!(write_bin(&[g]).is_none());
        let mut g = granule();
        g.rad_mean = f32::NAN;
        assert!(write_bin(&[g]).is_none());
    }

    #[test]
    fn component_name_maps_radiance() {
        assert_eq!(component_name(COMP_RADIANCE), Some("goes_abi_radiance"));
        assert_eq!(component_name(7), None);
    }

    #[test]
    fn decode_count_reinterprets_unsigned() {
        let mut dt = Hdf5Datatype {
            class: 0,
            size: 2,
            endian: Endian::Le,
            signed: true,
            bit_offset: 0,
            precision: 0,
            string_pad: 0,
            string_charset: 0,
            members: Vec::new(),
            array_dims: Vec::new(),
            base: None,
            reference_type: 0,
            vlen_is_string: false,
        };
        let raw = [0x00u8, 0x80]; // 0x8000 = 32768 as u16, -32768 as i16
        assert_eq!(decode_count(&raw, 0, &dt, true), Some(32768));
        assert_eq!(decode_count(&raw, 0, &dt, false), Some(-32768));
        dt.signed = false;
        assert_eq!(decode_count(&raw, 0, &dt, false), Some(32768));
    }

    #[test]
    fn stats_skips_fill_and_range() {
        let dt = Hdf5Datatype {
            class: 0,
            size: 2,
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
        };
        let mut raw = Vec::new();
        raw.extend_from_slice(&10u16.to_le_bytes());
        raw.extend_from_slice(&20u16.to_le_bytes());
        raw.extend_from_slice(&65535u16.to_le_bytes()); // fill
        raw.extend_from_slice(&5000u16.to_le_bytes()); // out of range
        let (sum, _, min, max, valid) = stats_of(
            &raw,
            &dt,
            4,
            true,
            Some(65535),
            Some((0, 4095)),
            Calib { scale: 1.0, offset: 0.0 },
        );
        assert_eq!(valid, 2);
        assert_eq!(sum, 30.0);
        assert_eq!(min, 10.0);
        assert_eq!(max, 20.0);
    }

    #[test]
    fn parse_gsics_txt_reads_goes16_slope_offset() {
        let text = "GOES-16\t\tGOES-18\t\tGOES-19\r\
Channel\tA\t B \tA\t B \tA\t B\r\
1\t0.0000\t0.9078\t0.0000\t0.9765\t0.0000\t1.0230\r\
16\t-0.2504\t1.0000\t0.2135\t1.0000\t-0.9346\t1.0000\r";
        let table = parse_gsics_txt(text);
        assert_eq!(gsics_lookup(&table, 1), Some((0.9078, 0.0)));
        assert_eq!(gsics_lookup(&table, 16), Some((1.0000, -0.2504)));
        assert_eq!(gsics_lookup(&table, 17), None);
        assert_eq!(gsics_lookup(&table, 0), None);
    }
}
