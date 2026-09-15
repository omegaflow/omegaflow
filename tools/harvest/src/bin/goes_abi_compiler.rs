use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{
    decode_f32, decode_f64, Endian, GeostationaryProjection, Hdf5Attribute, Hdf5Datatype, Hdf5File,
};
use std::io::{BufWriter, Write};
use std::process::Command;

const CDN_TAG: &str = "noaa-goes19.s3.amazonaws.com";
const GSICS_DEFAULT_URL: &str =
    "https://www.star.nesdis.noaa.gov/GOESCal/images/GSICS/GSICS_Harmonization_release_May2025_current.txt";
const MAGIC: [u8; 4] = *b"GAB1";
const VERSION: u8 = 1;
const HDR_LEN: usize = 12;
const REC_BYTES: usize = 56;
const CALIB_L1B: u8 = 0;
const CALIB_GSICS_PENDING: u8 = 1;
const CALIB_GSICS: u8 = 2;

type GsicsTable = [Option<(f64, f64)>; 16];

#[derive(Clone, Debug)]
struct Granule {
    t: f64,
    band_id: u8,
    calib: u8,
    band_wavelength: f32,
    esun: f32,
    kappa0: f32,
    sub_lon: f32,
    persp_h: f32,
    rad_mean: f32,
    rad_std: f32,
    rad_min: f32,
    rad_max: f32,
    valid: u32,
    total: u32,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn calib_name(calib: u8) -> &'static str {
    match calib {
        CALIB_L1B => "L1b",
        CALIB_GSICS => "gsics",
        CALIB_GSICS_PENDING => "gsics-pending",
        _ => "unknown",
    }
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
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
    (match attr_int(attrs, "_Unsigned") {
        Some(v) => v,
        None => 0,
    }) != 0
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

fn gsics_lookup(table: &GsicsTable, band_id: u8) -> Option<(f64, f64)> {
    if band_id == 0 {
        return None;
    }
    table.get((band_id as usize) - 1).copied().flatten()
}

fn parse_gsics_txt(text: &str) -> GsicsTable {
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
        if channel < 1 || channel > 16 {
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

fn read_gsics_source(src: &str) -> Option<String> {
    if src.starts_with("http://") || src.starts_with("https://") {
        curl_bytes(src).and_then(|b| String::from_utf8(b).ok())
    } else {
        match std::fs::read_to_string(src) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("{src}: read void: {e}");
                None
            }
        }
    }
}

fn stats_of(
    raw: &[u8],
    dt: &Hdf5Datatype,
    total: usize,
    unsigned: bool,
    fill: Option<i64>,
    valid_range: Option<(i64, i64)>,
    scale: f64,
    offset: f64,
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
        if let Some(f) = fill {
            if count == f {
                continue;
            }
        }
        if let Some((lo, hi)) = valid_range {
            if count < lo || count > hi {
                continue;
            }
        }
        let rad = count as f64 * scale + offset;
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

fn parse_granule(bytes: &[u8], gsics: Option<&GsicsTable>) -> Result<Granule, String> {
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
    let offset = match attr_number(&rad_obj.attrs, "add_offset") {
        Some(v) => v,
        None => 0.0,
    };
    let fill = attr_int_unsigned(&rad_obj.attrs, "_FillValue", unsigned);
    let valid_range = attr_int_pair_unsigned(&rad_obj.attrs, "valid_range", unsigned);
    let (sum, sumsq, min, max, valid) =
        stats_of(&raw, dt, total, unsigned, fill, valid_range, scale, offset);
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
            std = slope.abs() * std;
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
    Ok(Granule {
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

fn encode_rec(buf: &mut [u8], g: &Granule) {
    buf[0..8].copy_from_slice(&g.t.to_le_bytes());
    buf[8] = g.band_id;
    buf[9] = g.calib;
    buf[10] = 0;
    buf[11] = 0;
    buf[12..16].copy_from_slice(&g.band_wavelength.to_le_bytes());
    buf[16..20].copy_from_slice(&g.esun.to_le_bytes());
    buf[20..24].copy_from_slice(&g.kappa0.to_le_bytes());
    buf[24..28].copy_from_slice(&g.sub_lon.to_le_bytes());
    buf[28..32].copy_from_slice(&g.persp_h.to_le_bytes());
    buf[32..36].copy_from_slice(&g.rad_mean.to_le_bytes());
    buf[36..40].copy_from_slice(&g.rad_std.to_le_bytes());
    buf[40..44].copy_from_slice(&g.rad_min.to_le_bytes());
    buf[44..48].copy_from_slice(&g.rad_max.to_le_bytes());
    buf[48..52].copy_from_slice(&g.valid.to_le_bytes());
    buf[52..56].copy_from_slice(&g.total.to_le_bytes());
}

fn decode_rec(buf: &[u8]) -> Option<Granule> {
    if buf.len() != REC_BYTES {
        return None;
    }
    let calib = buf[9];
    if calib != CALIB_L1B && calib != CALIB_GSICS_PENDING && calib != CALIB_GSICS {
        return None;
    }
    Some(Granule {
        t: f64::from_le_bytes(buf[0..8].try_into().ok()?),
        band_id: buf[8],
        calib,
        band_wavelength: f32::from_le_bytes(buf[12..16].try_into().ok()?),
        esun: f32::from_le_bytes(buf[16..20].try_into().ok()?),
        kappa0: f32::from_le_bytes(buf[20..24].try_into().ok()?),
        sub_lon: f32::from_le_bytes(buf[24..28].try_into().ok()?),
        persp_h: f32::from_le_bytes(buf[28..32].try_into().ok()?),
        rad_mean: f32::from_le_bytes(buf[32..36].try_into().ok()?),
        rad_std: f32::from_le_bytes(buf[36..40].try_into().ok()?),
        rad_min: f32::from_le_bytes(buf[40..44].try_into().ok()?),
        rad_max: f32::from_le_bytes(buf[44..48].try_into().ok()?),
        valid: u32::from_le_bytes(buf[48..52].try_into().ok()?),
        total: u32::from_le_bytes(buf[52..56].try_into().ok()?),
    })
}

fn write_asset(records: &[Granule], out_path: &str) -> Result<usize, String> {
    let file = std::fs::File::create(out_path).map_err(|e| format!("create {out_path}: {e}"))?;
    let mut out = BufWriter::new(file);
    let mut hdr = Vec::with_capacity(HDR_LEN);
    hdr.extend_from_slice(&MAGIC);
    hdr.push(VERSION);
    hdr.extend_from_slice(&[0u8; 3]);
    hdr.extend_from_slice(&(records.len() as u32).to_le_bytes());
    out.write_all(&hdr)
        .map_err(|e| format!("write {out_path}: {e}"))?;
    let mut rec = [0u8; REC_BYTES];
    for r in records {
        encode_rec(&mut rec, r);
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path}: {e}"))?;
    }
    out.flush().map_err(|e| format!("flush {out_path}: {e}"))?;
    let expect = HDR_LEN + records.len() * REC_BYTES;
    let actual = std::fs::metadata(out_path)
        .map_err(|e| format!("stat {out_path}: {e}"))?
        .len() as usize;
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} B written, {expect} expected — the asset stays unwritten"
        ));
    }
    Ok(expect)
}

fn verify_asset(out_path: &str, records: &[Granule]) -> Result<(), String> {
    let bytes = std::fs::read(out_path).map_err(|e| format!("read {out_path}: {e}"))?;
    if bytes.len() < HDR_LEN || bytes[0..4] != MAGIC {
        return Err(format!("{out_path}: magic absent — the asset stays unread"));
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().map_err(|_| "count unread")?) as usize;
    if n != records.len() {
        return Err(format!(
            "{out_path}: {n} records, {} expected",
            records.len()
        ));
    }
    let last = decode_rec(&bytes[bytes.len() - REC_BYTES..])
        .ok_or_else(|| format!("{out_path}: last record stays unread"))?;
    eprintln!(
        "last granule: t {:.1} band {} wavelength {:.4} um mean {:.4} W m-2 sr-1 um-1",
        last.t, last.band_id, last.band_wavelength, last.rad_mean
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "goes_abi_rad.bin".to_string(),
    };
    let input = arg_value(&args, "--input");
    let url = arg_value(&args, "--url");
    let gsics_src = arg_value(&args, "--gsics");
    let bytes = match (input, url) {
        (Some(path), _) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{path}: read void: {e}");
                std::process::exit(1);
            }
        },
        (None, Some(u)) => match curl_bytes(&u) {
            Some(b) => b,
            None => {
                eprintln!("{u}: fetch void");
                std::process::exit(1);
            }
        },
        (None, None) => {
            eprintln!(
                "usage: goes_abi_compiler (--input <granule.nc> | --url <https url>) --out <goes_abi_rad.bin> [--ci-mode] [--gsics <txt url-or-file>]"
            );
            std::process::exit(1);
        }
    };
    let gsics_text = match gsics_src {
        Some(src) => read_gsics_source(&src),
        None if ci_mode => curl_bytes(GSICS_DEFAULT_URL).and_then(|b| String::from_utf8(b).ok()),
        None => None,
    };
    let gsics_table = gsics_text.as_deref().map(parse_gsics_txt);
    let granule = match parse_granule(&bytes, gsics_table.as_ref()) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("goes_abi_compiler: {e}");
            std::process::exit(1);
        }
    };
    if gsics_text.is_some() && granule.calib == CALIB_GSICS_PENDING {
        eprintln!(
            "gsics: band {} has no GOES-16 coefficient in the txt — calib stays gsics-pending",
            granule.band_id
        );
    }
    eprintln!(
        "granule: band {} wavelength {:.4} um t {:.1} (J2000 s) sub_lon {:.2} deg persp_h {:.0} m calib {}",
        granule.band_id,
        granule.band_wavelength,
        granule.t,
        granule.sub_lon,
        granule.persp_h,
        calib_name(granule.calib)
    );
    eprintln!(
        "radiance: mean {:.4} std {:.4} min {:.4} max {:.4} W m-2 sr-1 um-1, {} valid / {} pixels, esun {:.4} W m-2 um-1, kappa0 {:.6} (W m-2 um-1)-1",
        granule.rad_mean,
        granule.rad_std,
        granule.rad_min,
        granule.rad_max,
        granule.valid,
        granule.total,
        granule.esun,
        granule.kappa0
    );
    let records = vec![granule];
    if let Err(e) = write_asset(&records, &out_path) {
        eprintln!("goes_abi_compiler: {e}");
        std::process::exit(1);
    }
    if let Err(e) = verify_asset(&out_path, &records) {
        eprintln!("goes_abi_compiler: {e}");
        std::process::exit(1);
    }
    eprintln!(
        "{}: 1 record, {} B -> {}",
        out_path,
        HDR_LEN + REC_BYTES,
        out_path
    );
    if ci_mode && !upload_release(CDN_TAG, &out_path) {
        eprintln!("upload {}: did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_roundtrip() {
        let g = Granule {
            t: 758368477.5,
            band_id: 1,
            calib: CALIB_GSICS,
            band_wavelength: 0.4700,
            esun: 2024.0,
            kappa0: 0.000157,
            sub_lon: -75.2,
            persp_h: 35_786_023.0,
            rad_mean: 51.23,
            rad_std: 12.4,
            rad_min: 0.0,
            rad_max: 401.7,
            valid: 14_000_000,
            total: 15_000_000,
        };
        let mut rec = [0u8; REC_BYTES];
        encode_rec(&mut rec, &g);
        let back = decode_rec(&rec).unwrap();
        assert_eq!(back.t, g.t);
        assert_eq!(back.band_id, g.band_id);
        assert_eq!(back.calib, g.calib);
        assert_eq!(back.band_wavelength, g.band_wavelength);
        assert_eq!(back.esun, g.esun);
        assert_eq!(back.kappa0, g.kappa0);
        assert_eq!(back.sub_lon, g.sub_lon);
        assert_eq!(back.persp_h, g.persp_h);
        assert_eq!(back.rad_mean, g.rad_mean);
        assert_eq!(back.rad_std, g.rad_std);
        assert_eq!(back.rad_min, g.rad_min);
        assert_eq!(back.rad_max, g.rad_max);
        assert_eq!(back.valid, g.valid);
        assert_eq!(back.total, g.total);
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
        let (sum, _, min, max, valid) =
            stats_of(&raw, &dt, 4, true, Some(65535), Some((0, 4095)), 1.0, 0.0);
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

    #[test]
    fn real_goes16_abi_granule_compiles() {
        let path = "phi/pipeline/catalog/noaa_goes16/OR_ABI-L1b-RadC-M6C01_G16_s20240010001173_e20240010003546_c20240010004005.nc";
        if !std::path::Path::new(path).exists() {
            eprintln!(
                "skipped (fixture absent): goes16 abi — fetch from noaa-goes19.s3.amazonaws.com/ABI-L1b-RadC/2024/001/00/"
            );
            return;
        }
        let bytes = std::fs::read(path).expect("fixture read");
        let g = parse_granule(&bytes, None).expect("granule parses");
        assert_eq!(g.band_id, 1);
        assert!(g.total == 15_000_000);
        assert!(g.valid > 0 && g.valid <= g.total);
        assert!(g.rad_mean.is_finite());
        assert!(g.esun > 0.0);
        assert!(g.kappa0 > 0.0);
        assert!((g.sub_lon - -75.2).abs() < 1.0);
    }
}
