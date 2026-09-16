use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{
    Endian, Hdf5Attribute, Hdf5Datatype, Hdf5File, Hdf5Object, decode_f32, decode_f64,
    geostationary_lat_lon,
};
use std::io::{BufWriter, Write};
use std::process::Command;

const CDN_TAG: &str = "noaa-gk2a-pds.s3.amazonaws.com";
const MAGIC: [u8; 4] = *b"GKA1";
const VERSION: u8 = 1;
const HDR_LEN: usize = 12;
const REC_BYTES: usize = 56;
const CALIB_L1B: u8 = 0;
const CALIB_GSICS_PENDING: u8 = 1;
const CALIB_GSICS: u8 = 2;

#[derive(Clone, Debug)]
struct Granule {
    t: f64,
    band_id: u8,
    calib: u8,
    band_wavelength: f32,
    esun: f32,
    kappa0: f32,
    center_lat: f32,
    center_lon: f32,
    rad_mean: f32,
    rad_std: f32,
    rad_min: f32,
    rad_max: f32,
    valid: u32,
    total: u32,
}

enum Calibration {
    Gsics {
        gain: f64,
        offset: f64,
        slope: f64,
        intercept: f64,
        quadratic: f64,
        lo: Option<f64>,
        hi: Option<f64>,
    },
    Linear {
        gain: f64,
        offset: f64,
    },
    Pending,
}

impl Calibration {
    fn flag(&self) -> u8 {
        match self {
            Calibration::Gsics { .. } => CALIB_GSICS,
            Calibration::Linear { .. } => CALIB_L1B,
            Calibration::Pending => CALIB_GSICS_PENDING,
        }
    }

    fn radiance(&self, count: f64) -> Option<f64> {
        match self {
            Calibration::Gsics {
                gain,
                offset,
                slope,
                intercept,
                quadratic,
                lo,
                hi,
            } => {
                let op = gain * count + offset;
                if let (Some(l), Some(h)) = (lo, hi) {
                    if *l < *h && (op < *l || op > *h) {
                        return None;
                    }
                }
                let r = intercept + slope * op + quadratic * op * op;
                if r.is_finite() && r > 0.0 {
                    Some(r)
                } else {
                    None
                }
            }
            Calibration::Linear { gain, offset } => {
                let r = gain * count + offset;
                if r.is_finite() && r > 0.0 {
                    Some(r)
                } else {
                    None
                }
            }
            Calibration::Pending => None,
        }
    }
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

fn attr_text(obj: &Hdf5Object, name: &str) -> Option<String> {
    let a = obj.attrs.iter().find(|a| a.name == name)?;
    let s = String::from_utf8_lossy(&a.data);
    let t = s.trim_end_matches('\0').trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn root_attr_number(file: &Hdf5File, name: &str) -> Option<f64> {
    let root = file.resolve("").ok()?;
    attr_number(&root.attrs, name)
}

fn scalar_f64(file: &Hdf5File, name: &str) -> Option<f64> {
    let v = file.read_f64_dataset(name).ok()?.first().copied()?;
    if v.is_finite() { Some(v) } else { None }
}

fn band_wavelength_um(file: &Hdf5File) -> Option<f64> {
    if let Some(v) = root_attr_number(file, "channel_center_wavelength") {
        return Some(v);
    }
    let root = file.resolve("").ok()?;
    attr_text(root, "channel_center_wavelength")?
        .trim()
        .parse()
        .ok()
}

fn wavenumber_mw_to_wavelength_w_um(rad_mw_cm: f64, wavelength_um: f64) -> f64 {
    rad_mw_cm * 10.0 / (wavelength_um * wavelength_um)
}

fn band_id_from_channel(name: &str) -> Option<u8> {
    let digits: String = name
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

fn scale_offset(obj: &Hdf5Object) -> Option<Calibration> {
    let scale = attr_number(&obj.attrs, "scale_factor")?;
    let offset = match attr_number(&obj.attrs, "add_offset") {
        Some(v) => v,
        None => 0.0,
    };
    Some(Calibration::Linear {
        gain: scale,
        offset,
    })
}

fn dn_to_radiance(file: &Hdf5File) -> Option<(f64, f64)> {
    let gain = root_attr_number(file, "DN_to_Radiance_Gain")?;
    let offset = root_attr_number(file, "DN_to_Radiance_Offset")?;
    Some((gain, offset))
}

fn gsics_coeffs(file: &Hdf5File) -> Option<(f64, f64, f64, Option<f64>, Option<f64>)> {
    let slope = scalar_f64(file, "gsics_coeff_slope")?;
    let intercept = scalar_f64(file, "gsics_coeff_intercept")?;
    let quadratic = scalar_f64(file, "gsics_coeff_quadratic")?;
    let lo = scalar_f64(file, "gsics_coeff_valid_range_lower_limit");
    let hi = scalar_f64(file, "gsics_coeff_valid_range_upper_limit");
    Some((slope, intercept, quadratic, lo, hi))
}

fn select_calibration(file: &Hdf5File, obj: &Hdf5Object) -> Calibration {
    if let Some(c) = scale_offset(obj) {
        return c;
    }
    let linear = dn_to_radiance(file);
    let gsics = gsics_coeffs(file);
    match (linear, gsics) {
        (Some((gain, offset)), Some((slope, intercept, quadratic, lo, hi))) => Calibration::Gsics {
            gain,
            offset,
            slope,
            intercept,
            quadratic,
            lo,
            hi,
        },
        (Some((gain, offset)), None) => Calibration::Linear { gain, offset },
        _ => Calibration::Pending,
    }
}

fn radiance_stats(
    raw: &[u8],
    dt: &Hdf5Datatype,
    total: usize,
    valid_bits: u8,
    flag_bits: Option<u8>,
    calib: &Calibration,
) -> (f64, f64, f64, f64, u32) {
    let count_mask: u32 = if valid_bits >= 16 {
        u32::MAX
    } else {
        (1u32 << valid_bits) - 1
    };
    let mut sum = 0.0f64;
    let mut sumsq = 0.0f64;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut valid = 0u32;
    for i in 0..total {
        let off = i * dt.size;
        let Some(b) = raw.get(off..off + dt.size) else {
            continue;
        };
        let dn: u32 = match dt.size {
            1 => b[0] as u32,
            2 => {
                let arr: [u8; 2] = match b.try_into() {
                    Ok(a) => a,
                    Err(_) => continue,
                };
                match dt.endian {
                    Endian::Le => u16::from_le_bytes(arr) as u32,
                    Endian::Be => u16::from_be_bytes(arr) as u32,
                }
            }
            _ => continue,
        };
        let total_bits: u32 = (dt.size * 8) as u32;
        if let Some(fb) = flag_bits {
            let fb = fb as u32;
            if fb > 0 && fb < total_bits {
                let flag = (dn >> (total_bits - fb)) & ((1u32 << fb) - 1);
                if flag >= 2 {
                    continue;
                }
            }
        }
        let count = (dn & count_mask) as f64;
        let Some(rad) = calib.radiance(count) else {
            continue;
        };
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

fn parse_granule(bytes: &[u8]) -> Result<Granule, String> {
    let file = Hdf5File::parse(bytes).map_err(|n| format!("hdf5 parse: {n:?}"))?;
    let (rad_obj, ds, dt) = file
        .dataset("image_pixel_values")
        .map_err(|n| format!("image_pixel_values absent: {n:?}"))?;
    if ds.dims.len() != 2 {
        return Err(format!(
            "image_pixel_values rank {} — not a [y,x] count field",
            ds.dims.len()
        ));
    }
    let total: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let raw = file
        .read_dataset("image_pixel_values")
        .map_err(|n| format!("image_pixel_values read: {n:?}"))?;
    if raw.len() != total * dt.size {
        return Err(format!(
            "image_pixel_values {} B read, {} expected — the field stays unread",
            raw.len(),
            total * dt.size
        ));
    }
    let valid_bits: u8 =
        match attr_int_unsigned(&rad_obj.attrs, "number_of_valid_bits_per_pixel", true) {
            Some(v) if v > 0 && v <= 16 => v as u8,
            _ => 16,
        };
    let flag_bits = attr_int_unsigned(
        &rad_obj.attrs,
        "number_of_data_quality_flag_bits_per_pixel",
        true,
    )
    .filter(|v| *v > 0 && *v <= 4)
    .map(|v| v as u8);
    let calib = select_calibration(&file, rad_obj);
    let (sum, sumsq, min, max, valid) =
        radiance_stats(&raw, dt, total, valid_bits, flag_bits, &calib);
    if valid == 0 {
        return Err(
            "image_pixel_values: no valid radiance pixel — the record stays unwritten (0 honored)"
                .to_string(),
        );
    }
    let n = valid as f64;
    let mean_mw = sum / n;
    let variance_mw = (sumsq - sum * sum / n) / n;
    let std_mw = if variance_mw > 0.0 {
        variance_mw.sqrt()
    } else {
        0.0
    };
    let wavelength_um = band_wavelength_um(&file)
        .ok_or_else(|| "channel_center_wavelength absent or void".to_string())?;
    let mean = wavenumber_mw_to_wavelength_w_um(mean_mw, wavelength_um);
    let std = wavenumber_mw_to_wavelength_w_um(std_mw, wavelength_um);
    let min = wavenumber_mw_to_wavelength_w_um(min, wavelength_um);
    let max = wavenumber_mw_to_wavelength_w_um(max, wavelength_um);
    let t = root_attr_number(&file, "observation_start_time")
        .ok_or_else(|| "observation_start_time absent or void".to_string())?;
    let band_wavelength = wavelength_um as f32;
    let channel =
        attr_text(rad_obj, "channel_name").ok_or_else(|| "channel_name absent".to_string())?;
    let band_id = band_id_from_channel(&channel)
        .ok_or_else(|| format!("channel_name {channel} carries no band id"))?;
    let esun = match scalar_f64(&file, "esun") {
        Some(v) => v as f32,
        None => 0.0,
    };
    let kappa0 = match scalar_f64(&file, "kappa0") {
        Some(v) => v as f32,
        None => 0.0,
    };
    let proj = file
        .geostationary_projection()
        .map_err(|n| format!("projection absent: {n:?}"))?;
    let (center_lat, center_lon) =
        geostationary_lat_lon(0.0, 0.0, proj.sub_longitude_deg, proj.perspective_height_m);
    Ok(Granule {
        t,
        band_id,
        calib: calib.flag(),
        band_wavelength,
        esun,
        kappa0,
        center_lat: center_lat as f32,
        center_lon: center_lon as f32,
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
    buf[24..28].copy_from_slice(&g.center_lat.to_le_bytes());
    buf[28..32].copy_from_slice(&g.center_lon.to_le_bytes());
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
        center_lat: f32::from_le_bytes(buf[24..28].try_into().ok()?),
        center_lon: f32::from_le_bytes(buf[28..32].try_into().ok()?),
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
        "last granule: t {:.1} band {} wavelength {:.4} um center ({:.3}, {:.3}) deg mean {:.4} W m-2 sr-1 um-1",
        last.t, last.band_id, last.band_wavelength, last.center_lat, last.center_lon, last.rad_mean
    );
    Ok(())
}

fn probe(file: &Hdf5File) {
    if let Ok((obj, _, _)) = file.dataset("image_pixel_values") {
        println!(
            "image_pixel_values channel_name = {:?}",
            attr_text(obj, "channel_name")
        );
        println!(
            "image_pixel_values number_of_valid_bits_per_pixel = {:?}",
            attr_int_unsigned(&obj.attrs, "number_of_valid_bits_per_pixel", true)
        );
        println!(
            "image_pixel_values number_of_data_quality_flag_bits_per_pixel = {:?}",
            attr_int_unsigned(
                &obj.attrs,
                "number_of_data_quality_flag_bits_per_pixel",
                true
            )
        );
        println!(
            "image_pixel_values min_pixel_value = {:?}",
            attr_int_unsigned(&obj.attrs, "min_pixel_value", true)
        );
        println!(
            "image_pixel_values max_pixel_value = {:?}",
            attr_int_unsigned(&obj.attrs, "max_pixel_value", true)
        );
        println!(
            "image_pixel_values number_of_error_pixels = {:?}",
            attr_int(&obj.attrs, "number_of_error_pixels")
        );
        println!(
            "image_pixel_values average_pixel_value = {:?}",
            attr_number(&obj.attrs, "average_pixel_value")
        );
    }
    println!(
        "root observation_start_time = {:?}",
        root_attr_number(file, "observation_start_time")
    );
    println!(
        "root channel_center_wavelength (numeric) = {:?}",
        root_attr_number(file, "channel_center_wavelength")
    );
    println!(
        "root channel_center_wavelength (text) = {:?}",
        file.resolve("")
            .ok()
            .and_then(|r| attr_text(r, "channel_center_wavelength"))
    );
    println!(
        "root DN_to_Radiance_Gain = {:?}",
        root_attr_number(file, "DN_to_Radiance_Gain")
    );
    println!(
        "root DN_to_Radiance_Offset = {:?}",
        root_attr_number(file, "DN_to_Radiance_Offset")
    );
    println!(
        "root sub_longitude = {:?}",
        root_attr_number(file, "sub_longitude")
    );
    println!(
        "root nominal_satellite_height = {:?}",
        root_attr_number(file, "nominal_satellite_height")
    );
    println!(
        "gsics_coeff_slope = {:?}",
        scalar_f64(file, "gsics_coeff_slope")
    );
    println!(
        "gsics_coeff_intercept = {:?}",
        scalar_f64(file, "gsics_coeff_intercept")
    );
    println!(
        "gsics_coeff_quadratic = {:?}",
        scalar_f64(file, "gsics_coeff_quadratic")
    );
    println!(
        "gsics_coeff_valid_range_lower_limit = {:?}",
        scalar_f64(file, "gsics_coeff_valid_range_lower_limit")
    );
    println!(
        "gsics_coeff_valid_range_upper_limit = {:?}",
        scalar_f64(file, "gsics_coeff_valid_range_upper_limit")
    );
    pixel_bit_layout(file);
}

fn pixel_bit_layout(file: &Hdf5File) {
    let Ok((_, ds, dt)) = file.dataset("image_pixel_values") else {
        return;
    };
    let total: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let Ok(raw) = file.read_dataset("image_pixel_values") else {
        return;
    };
    let mut f13 = [0u64; 4];
    let mut f14 = [0u64; 4];
    let mut min13 = u32::MAX;
    let mut max13 = 0u32;
    let mut min14 = u32::MAX;
    let mut max14 = 0u32;
    let mut sum13 = 0u64;
    let mut sum14 = 0u64;
    let mut n = 0u64;
    for i in 0..total {
        let off = i * dt.size;
        let Some(b) = raw.get(off..off + dt.size) else {
            continue;
        };
        let dn: u32 = match dt.size {
            1 => b[0] as u32,
            2 => {
                let arr: [u8; 2] = match b.try_into() {
                    Ok(a) => a,
                    Err(_) => continue,
                };
                match dt.endian {
                    Endian::Le => u16::from_le_bytes(arr) as u32,
                    Endian::Be => u16::from_be_bytes(arr) as u32,
                }
            }
            _ => continue,
        };
        f13[(dn >> 13) as usize % 4] += 1;
        f14[(dn >> 14) as usize % 4] += 1;
        let c13 = dn & 0x1FFF;
        let c14 = dn & 0x3FFF;
        if c13 < min13 {
            min13 = c13;
        }
        if c13 > max13 {
            max13 = c13;
        }
        if c14 < min14 {
            min14 = c14;
        }
        if c14 > max14 {
            max14 = c14;
        }
        sum13 += c13 as u64;
        sum14 += c14 as u64;
        n += 1;
    }
    println!("pixels {n} — flag@13-14: {:?} | flag@14-15: {:?}", f13, f14);
    println!(
        "count&0x1FFF mean {} min {} max {} | count&0x3FFF mean {} min {} max {}",
        if n > 0 { sum13 as f64 / n as f64 } else { 0.0 },
        min13,
        max13,
        if n > 0 { sum14 as f64 / n as f64 } else { 0.0 },
        min14,
        max14
    );
    let mut gsum = 0u64;
    let mut gn = 0u64;
    let mut gmin = u32::MAX;
    let mut gmax = 0u32;
    let mut rmin = f64::INFINITY;
    let mut rmax = f64::NEG_INFINITY;
    let mut rsum = 0.0f64;
    for i in 0..total {
        let off = i * dt.size;
        let Some(b) = raw.get(off..off + dt.size) else {
            continue;
        };
        let dn: u32 = match dt.size {
            2 => {
                let arr: [u8; 2] = match b.try_into() {
                    Ok(a) => a,
                    Err(_) => continue,
                };
                match dt.endian {
                    Endian::Le => u16::from_le_bytes(arr) as u32,
                    Endian::Be => u16::from_be_bytes(arr) as u32,
                }
            }
            _ => continue,
        };
        if (dn >> 14) & 3 != 0 {
            continue;
        }
        let c = dn & 0x1FFF;
        gsum += c as u64;
        gn += 1;
        if c < gmin {
            gmin = c;
        }
        if c > gmax {
            gmax = c;
        }
        let r = -0.0144806550815701 * c as f64 + 118.050903320312;
        if r < rmin {
            rmin = r;
        }
        if r > rmax {
            rmax = r;
        }
        rsum += r;
    }
    println!(
        "good (flag0) {gn} — count mean {} min {} max {} — radiance(linear) mean {} min {} max {}",
        if gn > 0 { gsum as f64 / gn as f64 } else { 0.0 },
        gmin,
        gmax,
        if gn > 0 { rsum / gn as f64 } else { 0.0 },
        rmin,
        rmax
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "gk2a_ami_rad.bin".to_string(),
    };
    let input = arg_value(&args, "--input");
    let url = arg_value(&args, "--url");
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
                "usage: gk2a_ami_compiler (--input <granule.nc> | --url <https url>) --out <gk2a_ami_rad.bin> [--ci-mode] [--probe]"
            );
            std::process::exit(1);
        }
    };
    if has_flag(&args, "--probe") {
        match Hdf5File::parse(&bytes) {
            Ok(file) => probe(&file),
            Err(n) => {
                eprintln!("gk2a_ami_compiler: hdf5 parse: {n:?}");
                std::process::exit(1);
            }
        }
        return;
    }
    let granule = match parse_granule(&bytes) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("gk2a_ami_compiler: {e}");
            std::process::exit(1);
        }
    };
    eprintln!(
        "granule: band {} wavelength {:.4} um t {:.1} (J2000 s) center ({:.3}, {:.3}) deg calib {}",
        granule.band_id,
        granule.band_wavelength,
        granule.t,
        granule.center_lat,
        granule.center_lon,
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
        eprintln!("gk2a_ami_compiler: {e}");
        std::process::exit(1);
    }
    if let Err(e) = verify_asset(&out_path, &records) {
        eprintln!("gk2a_ami_compiler: {e}");
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

    fn int16_dt() -> Hdf5Datatype {
        Hdf5Datatype {
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
        }
    }

    #[test]
    fn record_roundtrip() {
        let g = Granule {
            t: 729_777_632.227_242_1,
            band_id: 87,
            calib: CALIB_GSICS,
            band_wavelength: 8.7,
            esun: 0.0,
            kappa0: 0.0,
            center_lat: 0.0,
            center_lon: 128.2,
            rad_mean: 1.234,
            rad_std: 0.5,
            rad_min: 0.1,
            rad_max: 5.0,
            valid: 30_000_000,
            total: 30_250_000,
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
        assert_eq!(back.center_lat, g.center_lat);
        assert_eq!(back.center_lon, g.center_lon);
        assert_eq!(back.rad_mean, g.rad_mean);
        assert_eq!(back.rad_std, g.rad_std);
        assert_eq!(back.rad_min, g.rad_min);
        assert_eq!(back.rad_max, g.rad_max);
        assert_eq!(back.valid, g.valid);
        assert_eq!(back.total, g.total);
    }

    #[test]
    fn gsics_corrects_operational_radiance() {
        let calib = Calibration::Gsics {
            gain: -0.0144806550815701,
            offset: 118.050903320312,
            slope: 0.9978591203689575,
            intercept: 0.15076667070388794,
            quadratic: 0.0,
            lo: None,
            hi: None,
        };
        let op = -0.0144806550815701 * 7600.0 + 118.050903320312;
        let expected = 0.15076667070388794 + 0.9978591203689575 * op;
        assert_eq!(calib.radiance(7600.0).unwrap(), expected);
        assert_eq!(calib.flag(), CALIB_GSICS);
    }

    #[test]
    fn gsics_rejects_non_positive_radiance() {
        let calib = Calibration::Gsics {
            gain: -0.0144806550815701,
            offset: 118.050903320312,
            slope: 1.0,
            intercept: 0.0,
            quadratic: 0.0,
            lo: None,
            hi: None,
        };
        assert!(calib.radiance(8191.0).is_none());
    }

    #[test]
    fn gsics_valid_range_sentinel_stays_inactive() {
        let calib = Calibration::Gsics {
            gain: 1.0,
            offset: 0.0,
            slope: 1.0,
            intercept: 0.0,
            quadratic: 0.0,
            lo: Some(-999.0),
            hi: Some(-999.0),
        };
        assert_eq!(calib.radiance(50.0), Some(50.0));
        let constrained = Calibration::Gsics {
            gain: 1.0,
            offset: 0.0,
            slope: 1.0,
            intercept: 0.0,
            quadratic: 0.0,
            lo: Some(10.0),
            hi: Some(100.0),
        };
        assert!(constrained.radiance(5.0).is_none());
        assert!(constrained.radiance(200.0).is_none());
        assert_eq!(constrained.radiance(50.0), Some(50.0));
    }

    #[test]
    fn linear_calibration_applies_gain_and_offset() {
        let calib = Calibration::Linear {
            gain: 0.5,
            offset: 2.0,
        };
        assert_eq!(calib.radiance(10.0), Some(7.0));
        assert_eq!(calib.flag(), CALIB_L1B);
        assert_eq!(Calibration::Pending.flag(), CALIB_GSICS_PENDING);
    }

    #[test]
    fn band_id_parses_channel_digits() {
        assert_eq!(band_id_from_channel("IR087"), Some(87));
        assert_eq!(band_id_from_channel("SW038"), Some(38));
        assert_eq!(band_id_from_channel("VIS004"), Some(4));
        assert_eq!(band_id_from_channel("VIS"), None);
    }

    #[test]
    fn flag_masking_skips_out_of_scan_and_error_pixels() {
        let dt = int16_dt();
        let mut raw = Vec::new();
        raw.extend_from_slice(&1000u16.to_le_bytes());
        raw.extend_from_slice(&2000u16.to_le_bytes());
        raw.extend_from_slice(&0x8000u16.to_le_bytes());
        raw.extend_from_slice(&0xC000u16.to_le_bytes());
        let calib = Calibration::Gsics {
            gain: 1.0,
            offset: 0.0,
            slope: 0.01,
            intercept: 1.0,
            quadratic: 0.0,
            lo: None,
            hi: None,
        };
        let (sum, _, min, max, valid) = radiance_stats(&raw, &dt, 4, 13, Some(2), &calib);
        assert_eq!(valid, 2);
        assert!((sum - 32.0).abs() < 1e-12);
        assert!((min - 11.0).abs() < 1e-12);
        assert!((max - 21.0).abs() < 1e-12);
    }

    #[test]
    fn wavenumber_mw_converts_to_wavelength_w_um() {
        let w = wavenumber_mw_to_wavelength_w_um(42.09, 8.7);
        assert!((w - 5.56).abs() < 0.01);
    }
}
