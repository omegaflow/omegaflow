use crate::archivar::pds3_table::odl_kv;

pub const MAGIC: [u8; 4] = *b"P3IM";
pub const BAND_NAME_BYTES: usize = 32;

#[derive(Clone, Debug, PartialEq)]
pub enum ImgReject {
    MissingField(&'static str),
    UnknownSampleType(String),
    UndeterminedByteOrder(String),
    SampleBitsNotByteAligned(u32),
    UnsupportedEnviType(u32),
    UnknownByteOrder(String),
    UnknownInterleave(String),
}

impl ImgReject {
    pub fn reason(&self) -> String {
        match self {
            ImgReject::MissingField(k) => format!("missing field {k}"),
            ImgReject::UnknownSampleType(t) => format!("unknown sample type {t}"),
            ImgReject::UndeterminedByteOrder(t) => {
                format!("sample type {t} carries no byte order")
            }
            ImgReject::SampleBitsNotByteAligned(b) => {
                format!("sample bits {b} not byte-aligned")
            }
            ImgReject::UnsupportedEnviType(t) => format!("unsupported envi data type {t}"),
            ImgReject::UnknownByteOrder(b) => format!("unknown byte order {b}"),
            ImgReject::UnknownInterleave(i) => format!("unknown interleave {i}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImgMeta {
    pub record_type: Option<String>,
    pub record_bytes: Option<usize>,
    pub lines: Option<usize>,
    pub line_samples: Option<usize>,
    pub sample_type: Option<String>,
    pub sample_bits: Option<u32>,
    pub bands: Option<usize>,
    pub offset: Option<usize>,
    pub line_prefix_bytes: Option<usize>,
    pub missing_constant: Option<f64>,
    pub product_id: Option<String>,
    pub envi_data_type: Option<u32>,
    pub byte_order: Option<u32>,
    pub interleave: Option<String>,
    pub header_offset: Option<usize>,
    pub band_names: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImgRaster {
    pub bands: usize,
    pub lines: usize,
    pub samples: usize,
    pub band_names: Vec<String>,
    pub values: Vec<Option<f64>>,
}

fn normalized_band_names(meta: &ImgMeta, bands: usize) -> Vec<String> {
    let mut names = meta.band_names.clone();
    names.resize(bands, String::new());
    names
}

#[derive(Clone, Copy, Debug)]
enum ByteOrder {
    Big,
    Little,
}

fn parse_number(v: &str) -> Option<f64> {
    v.trim().parse::<f64>().ok().filter(|x| x.is_finite())
}

pub fn parse_label(text: &str) -> Option<ImgMeta> {
    let mut meta = ImgMeta {
        record_type: None,
        record_bytes: None,
        lines: None,
        line_samples: None,
        sample_type: None,
        sample_bits: None,
        bands: None,
        offset: None,
        line_prefix_bytes: None,
        missing_constant: None,
        product_id: None,
        envi_data_type: None,
        byte_order: None,
        interleave: None,
        header_offset: None,
        band_names: Vec::new(),
    };
    for (key, value) in odl_kv(text) {
        match key.as_str() {
            "RECORD_TYPE" => meta.record_type = Some(value),
            "RECORD_BYTES" => meta.record_bytes = value.parse().ok(),
            "LINES" => meta.lines = value.parse().ok(),
            "LINE_SAMPLES" => meta.line_samples = value.parse().ok(),
            "SAMPLE_TYPE" => meta.sample_type = Some(value),
            "SAMPLE_BITS" => meta.sample_bits = value.parse().ok(),
            "BANDS" => meta.bands = value.parse().ok(),
            "OFFSET" => meta.offset = value.parse().ok(),
            "LINE_PREFIX_BYTES" => meta.line_prefix_bytes = value.parse().ok(),
            "MISSING_CONSTANT" => meta.missing_constant = parse_number(&value),
            "PRODUCT_ID" => meta.product_id = Some(value),
            _ => {}
        }
    }
    Some(meta)
}

pub fn parse_envi_hdr(text: &str) -> Option<ImgMeta> {
    let mut meta = ImgMeta {
        record_type: None,
        record_bytes: None,
        lines: None,
        line_samples: None,
        sample_type: None,
        sample_bits: None,
        bands: None,
        offset: None,
        line_prefix_bytes: None,
        missing_constant: None,
        product_id: None,
        envi_data_type: None,
        byte_order: None,
        interleave: None,
        header_offset: None,
        band_names: Vec::new(),
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim().trim_matches('"').trim().to_string();
        if value.is_empty() {
            continue;
        }
        match key.as_str() {
            "samples" => meta.line_samples = value.parse().ok(),
            "lines" => meta.lines = value.parse().ok(),
            "bands" => meta.bands = value.parse().ok(),
            "data type" => meta.envi_data_type = value.parse().ok(),
            "byte order" => meta.byte_order = value.parse().ok(),
            "interleave" => meta.interleave = Some(value),
            "header offset" => meta.header_offset = value.parse().ok(),
            "data ignore value" => meta.missing_constant = parse_number(&value),
            _ => {}
        }
    }
    Some(meta)
}

fn byte_order_of(sample_type: &str) -> Result<ByteOrder, ImgReject> {
    match sample_type.to_ascii_uppercase().as_str() {
        "MSB_INTEGER" | "MSB_UNSIGNED_INTEGER" | "SUN_INTEGER" | "SUN_UNSIGNED_INTEGER" => {
            Ok(ByteOrder::Big)
        }
        "LSB_INTEGER" | "LSB_UNSIGNED_INTEGER" | "INTEL_INTEGER" | "INTEL_UNSIGNED_INTEGER" => {
            Ok(ByteOrder::Little)
        }
        "IEEE_REAL"
        | "PC_REAL"
        | "PC_INTEGER"
        | "PC_UNSIGNED_INTEGER"
        | "VAX_REAL"
        | "VAX_INTEGER"
        | "REAL"
        | "FLOAT"
        | "INTEGER"
        | "UNSIGNED_INTEGER"
        | "SIGNED_INTEGER" => Err(ImgReject::UndeterminedByteOrder(
            sample_type.to_ascii_uppercase(),
        )),
        other => Err(ImgReject::UnknownSampleType(other.to_string())),
    }
}

fn sample_width(sample_type: &str, sample_bits: u32) -> Result<usize, ImgReject> {
    byte_order_of(sample_type)?;
    if sample_bits == 0 || sample_bits % 8 != 0 {
        return Err(ImgReject::SampleBitsNotByteAligned(sample_bits));
    }
    Ok((sample_bits / 8) as usize)
}

fn int_of(raw: &[u8], order: ByteOrder, signed: bool) -> Option<f64> {
    let v = match raw.len() {
        1 => {
            if signed {
                raw[0] as i8 as f64
            } else {
                raw[0] as f64
            }
        }
        2 => {
            let b = [raw[0], raw[1]];
            match (order, signed) {
                (ByteOrder::Big, true) => i16::from_be_bytes(b) as f64,
                (ByteOrder::Big, false) => u16::from_be_bytes(b) as f64,
                (ByteOrder::Little, true) => i16::from_le_bytes(b) as f64,
                (ByteOrder::Little, false) => u16::from_le_bytes(b) as f64,
            }
        }
        4 => {
            let b = [raw[0], raw[1], raw[2], raw[3]];
            match (order, signed) {
                (ByteOrder::Big, true) => i32::from_be_bytes(b) as f64,
                (ByteOrder::Big, false) => u32::from_be_bytes(b) as f64,
                (ByteOrder::Little, true) => i32::from_le_bytes(b) as f64,
                (ByteOrder::Little, false) => u32::from_le_bytes(b) as f64,
            }
        }
        8 => {
            let b = [
                raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            ];
            match (order, signed) {
                (ByteOrder::Big, true) => i64::from_be_bytes(b) as f64,
                (ByteOrder::Big, false) => u64::from_be_bytes(b) as f64,
                (ByteOrder::Little, true) => i64::from_le_bytes(b) as f64,
                (ByteOrder::Little, false) => u64::from_le_bytes(b) as f64,
            }
        }
        _ => return None,
    };
    Some(v)
}

fn decode_sample(
    sample_type: &str,
    sample_bits: u32,
    raw: &[u8],
    missing: Option<f64>,
) -> Result<Option<f64>, ImgReject> {
    let order = byte_order_of(sample_type)?;
    let width = sample_width(sample_type, sample_bits)?;
    if raw.len() != width {
        return Ok(None);
    }
    let signed = !sample_type.to_ascii_uppercase().contains("UNSIGNED");
    let Some(v) = int_of(raw, order, signed) else {
        return Ok(None);
    };
    if !v.is_finite() {
        return Ok(None);
    }
    match missing {
        Some(m) if v == m => Ok(None),
        _ => Ok(Some(v)),
    }
}

pub fn decode_raster(bytes: &[u8], meta: &ImgMeta) -> Result<ImgRaster, ImgReject> {
    let lines = meta.lines.ok_or(ImgReject::MissingField("LINES"))?;
    let samples = meta
        .line_samples
        .ok_or(ImgReject::MissingField("LINE_SAMPLES"))?;
    let bands = meta.bands.ok_or(ImgReject::MissingField("BANDS"))?;
    let sample_type = meta
        .sample_type
        .as_deref()
        .ok_or(ImgReject::MissingField("SAMPLE_TYPE"))?;
    let sample_bits = meta
        .sample_bits
        .ok_or(ImgReject::MissingField("SAMPLE_BITS"))?;
    if lines == 0 || samples == 0 || bands == 0 {
        return Err(ImgReject::MissingField("LINES/LINE_SAMPLES/BANDS"));
    }
    let width = sample_width(sample_type, sample_bits)?;
    let prefix = match meta.line_prefix_bytes {
        Some(p) => p,
        None => 0,
    };
    let offset = match meta.offset {
        Some(o) => o,
        None => 0,
    };
    let data_bytes_per_line = match samples.checked_mul(width) {
        Some(v) => v,
        None => return Err(ImgReject::SampleBitsNotByteAligned(sample_bits)),
    };
    let record_bytes = match meta.record_bytes {
        Some(rb) => rb,
        None => match data_bytes_per_line.checked_add(prefix) {
            Some(v) => v,
            None => return Err(ImgReject::SampleBitsNotByteAligned(sample_bits)),
        },
    };
    if record_bytes < data_bytes_per_line + prefix {
        return Err(ImgReject::MissingField("RECORD_BYTES"));
    }
    let per_band = match lines.checked_mul(samples) {
        Some(v) => v,
        None => return Err(ImgReject::SampleBitsNotByteAligned(sample_bits)),
    };
    let total = match bands.checked_mul(per_band) {
        Some(v) => v,
        None => return Err(ImgReject::SampleBitsNotByteAligned(sample_bits)),
    };
    let mut values = Vec::with_capacity(total);
    for band in 0..bands {
        for line in 0..lines {
            let record_at = offset + (band * lines + line) * record_bytes + prefix;
            for s in 0..samples {
                let at = record_at + s * width;
                let v = match bytes.get(at..at + width) {
                    Some(raw) => {
                        decode_sample(sample_type, sample_bits, raw, meta.missing_constant)?
                    }
                    None => None,
                };
                values.push(v);
            }
        }
    }
    Ok(ImgRaster {
        bands,
        lines,
        samples,
        band_names: normalized_band_names(meta, bands),
        values,
    })
}

fn envi_width(data_type: u32) -> Result<usize, ImgReject> {
    match data_type {
        1 => Ok(1),
        2 | 12 => Ok(2),
        3 | 4 | 13 => Ok(4),
        5 | 14 | 15 => Ok(8),
        other => Err(ImgReject::UnsupportedEnviType(other)),
    }
}

fn envi_value(raw: &[u8], data_type: u32, order: ByteOrder, missing: Option<f64>) -> Option<f64> {
    let v = match (data_type, raw.len()) {
        (1, 1) => raw[0] as f64,
        (2, 2) => {
            let b = [raw[0], raw[1]];
            match order {
                ByteOrder::Big => i16::from_be_bytes(b) as f64,
                ByteOrder::Little => i16::from_le_bytes(b) as f64,
            }
        }
        (3, 4) => {
            let b = [raw[0], raw[1], raw[2], raw[3]];
            match order {
                ByteOrder::Big => i32::from_be_bytes(b) as f64,
                ByteOrder::Little => i32::from_le_bytes(b) as f64,
            }
        }
        (4, 4) => {
            let b = [raw[0], raw[1], raw[2], raw[3]];
            match order {
                ByteOrder::Big => f32::from_be_bytes(b) as f64,
                ByteOrder::Little => f32::from_le_bytes(b) as f64,
            }
        }
        (5, 8) => {
            let b = [
                raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            ];
            match order {
                ByteOrder::Big => f64::from_be_bytes(b),
                ByteOrder::Little => f64::from_le_bytes(b),
            }
        }
        (12, 2) => {
            let b = [raw[0], raw[1]];
            match order {
                ByteOrder::Big => u16::from_be_bytes(b) as f64,
                ByteOrder::Little => u16::from_le_bytes(b) as f64,
            }
        }
        (13, 4) => {
            let b = [raw[0], raw[1], raw[2], raw[3]];
            match order {
                ByteOrder::Big => u32::from_be_bytes(b) as f64,
                ByteOrder::Little => u32::from_le_bytes(b) as f64,
            }
        }
        (14, 8) => {
            let b = [
                raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            ];
            match order {
                ByteOrder::Big => i64::from_be_bytes(b) as f64,
                ByteOrder::Little => i64::from_le_bytes(b) as f64,
            }
        }
        (15, 8) => {
            let b = [
                raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            ];
            match order {
                ByteOrder::Big => u64::from_be_bytes(b) as f64,
                ByteOrder::Little => u64::from_le_bytes(b) as f64,
            }
        }
        _ => return None,
    };
    if !v.is_finite() {
        return None;
    }
    match missing {
        Some(m) if v == m => None,
        _ => Some(v),
    }
}

#[derive(Clone, Copy, Debug)]
enum Interleave {
    Bsq,
    Bil,
    Bip,
}

fn interleave_of(s: &str) -> Result<Interleave, ImgReject> {
    match s.to_ascii_lowercase().as_str() {
        "bsq" => Ok(Interleave::Bsq),
        "bil" => Ok(Interleave::Bil),
        "bip" => Ok(Interleave::Bip),
        other => Err(ImgReject::UnknownInterleave(other.to_string())),
    }
}

pub fn decode_envi(bytes: &[u8], meta: &ImgMeta) -> Result<ImgRaster, ImgReject> {
    let samples = meta
        .line_samples
        .ok_or(ImgReject::MissingField("samples"))?;
    let lines = meta.lines.ok_or(ImgReject::MissingField("lines"))?;
    let bands = meta.bands.ok_or(ImgReject::MissingField("bands"))?;
    let data_type = meta
        .envi_data_type
        .ok_or(ImgReject::MissingField("data type"))?;
    if samples == 0 || lines == 0 || bands == 0 {
        return Err(ImgReject::MissingField("samples/lines/bands"));
    }
    let width = envi_width(data_type)?;
    let order = match meta.byte_order {
        Some(0) => ByteOrder::Little,
        Some(1) => ByteOrder::Big,
        Some(other) => return Err(ImgReject::UnknownByteOrder(other.to_string())),
        None => ByteOrder::Little,
    };
    let interleave = match meta.interleave.as_deref() {
        Some(s) => interleave_of(s)?,
        None => Interleave::Bsq,
    };
    let header_offset = match meta.header_offset {
        Some(h) => h,
        None => 0,
    };
    let per_band = match samples.checked_mul(lines) {
        Some(v) => v,
        None => return Err(ImgReject::SampleBitsNotByteAligned(0)),
    };
    let total = match bands.checked_mul(per_band) {
        Some(v) => v,
        None => return Err(ImgReject::SampleBitsNotByteAligned(0)),
    };
    let mut values = vec![None; total];
    for idx in 0..total {
        let (band, line, sample) = match interleave {
            Interleave::Bsq => (idx / per_band, (idx % per_band) / samples, idx % samples),
            Interleave::Bil => {
                let per_line = bands * samples;
                ((idx % per_line) / samples, idx / per_line, idx % samples)
            }
            Interleave::Bip => (
                idx % bands,
                (idx / bands) / samples,
                (idx / bands) % samples,
            ),
        };
        let at = header_offset + idx * width;
        let v = match bytes.get(at..at + width) {
            Some(raw) => envi_value(raw, data_type, order, meta.missing_constant),
            None => None,
        };
        let canonical = band * per_band + line * samples + sample;
        values[canonical] = v;
    }
    Ok(ImgRaster {
        bands,
        lines,
        samples,
        band_names: normalized_band_names(meta, bands),
        values,
    })
}

pub fn pack(raster: &ImgRaster) -> Vec<u8> {
    let n = raster.bands * raster.lines * raster.samples;
    let words = n.div_ceil(64);
    let mut bin = vec![0u8; 16 + raster.bands * BAND_NAME_BYTES + n * 8 + words * 8];
    bin[0..4].copy_from_slice(&MAGIC);
    bin[4..8].copy_from_slice(&(raster.bands as u32).to_le_bytes());
    bin[8..12].copy_from_slice(&(raster.lines as u32).to_le_bytes());
    bin[12..16].copy_from_slice(&(raster.samples as u32).to_le_bytes());
    for i in 0..raster.bands {
        let base = 16 + i * BAND_NAME_BYTES;
        if let Some(name) = raster.band_names.get(i) {
            let b = name.as_bytes();
            let l = b.len().min(BAND_NAME_BYTES);
            bin[base..base + l].copy_from_slice(&b[..l]);
        }
    }
    let data_base = 16 + raster.bands * BAND_NAME_BYTES;
    let mut pw = vec![0u64; words];
    for (i, v) in raster.values.iter().enumerate() {
        if let Some(x) = v {
            bin[data_base + i * 8..data_base + i * 8 + 8].copy_from_slice(&x.to_le_bytes());
            pw[i / 64] |= 1u64 << (i % 64);
        }
    }
    for (w, word) in pw.iter().enumerate() {
        let wb = data_base + n * 8 + w * 8;
        bin[wb..wb + 8].copy_from_slice(&word.to_le_bytes());
    }
    bin
}

fn str_field(bytes: &[u8], from: usize, to: usize) -> Option<String> {
    let field = bytes.get(from..to)?;
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    String::from_utf8(field[..end].to_vec()).ok()
}

pub fn parse_image(bytes: &[u8]) -> Option<ImgRaster> {
    if bytes.len() < 16 || bytes[0..4] != MAGIC {
        return None;
    }
    let bands = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let lines = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let samples = u32::from_le_bytes(bytes[12..16].try_into().ok()?) as usize;
    if bands == 0 || lines == 0 || samples == 0 {
        return None;
    }
    let n = bands.checked_mul(lines)?.checked_mul(samples)?;
    let words = n.div_ceil(64);
    let data_base = 16 + bands * BAND_NAME_BYTES;
    let expected = data_base + n * 8 + words * 8;
    if bytes.len() != expected {
        return None;
    }
    let mut band_names = Vec::with_capacity(bands);
    for i in 0..bands {
        let base = 16 + i * BAND_NAME_BYTES;
        let name = str_field(bytes, base, base + BAND_NAME_BYTES)?;
        band_names.push(name);
    }
    let mut values = Vec::with_capacity(n);
    for i in 0..n {
        let wb = data_base + n * 8 + (i / 64) * 8;
        let present = u64::from_le_bytes(bytes[wb..wb + 8].try_into().ok()?);
        let v = f64::from_le_bytes(
            bytes[data_base + i * 8..data_base + i * 8 + 8]
                .try_into()
                .ok()?,
        );
        let value = if present & (1u64 << (i % 64)) != 0 && v.is_finite() {
            Some(v)
        } else {
            None
        };
        values.push(value);
    }
    Some(ImgRaster {
        bands,
        lines,
        samples,
        band_names,
        values,
    })
}

pub fn band_means(raster: &ImgRaster) -> Option<Vec<(f64, f64, u32)>> {
    let per_band = raster.lines * raster.samples;
    let mut out = Vec::with_capacity(raster.bands);
    for b in 0..raster.bands {
        let start = b * per_band;
        let mut sum = 0.0f64;
        let mut count = 0usize;
        for i in start..start + per_band {
            if let Some(v) = raster.values[i] {
                sum += v;
                count += 1;
            }
        }
        if count > 0 {
            out.push((b as f64, sum / count as f64, b as u32));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let raster = parse_image(bytes)?;
    band_means(&raster)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIRF_LABEL: &str = "PDS_VERSION_ID = PDS3
RECORD_TYPE = FIXED_LENGTH
RECORD_BYTES = 8
FILE_RECORDS = 6
LINES = 2
LINE_SAMPLES = 2
BANDS = 1
SAMPLE_TYPE = MSB_INTEGER
SAMPLE_BITS = 16
OFFSET = 0
LINE_PREFIX_BYTES = 0
PRODUCT_ID = \"MIRF_CPR_TEST\"
END
";

    fn minirf_raster() -> Vec<u8> {
        let mut out = Vec::new();
        for v in [1i16, 2, 3, 4] {
            out.extend_from_slice(&v.to_be_bytes());
        }
        out
    }

    #[test]
    fn parse_label_reads_the_raster_label() {
        let meta = parse_label(MINIRF_LABEL).expect("label parses");
        assert_eq!(meta.record_type.as_deref(), Some("FIXED_LENGTH"));
        assert_eq!(meta.record_bytes, Some(8));
        assert_eq!(meta.lines, Some(2));
        assert_eq!(meta.line_samples, Some(2));
        assert_eq!(meta.bands, Some(1));
        assert_eq!(meta.sample_type.as_deref(), Some("MSB_INTEGER"));
        assert_eq!(meta.sample_bits, Some(16));
        assert_eq!(meta.offset, Some(0));
        assert_eq!(meta.line_prefix_bytes, Some(0));
    }

    #[test]
    fn decode_reads_the_raster_values() {
        let meta = parse_label(MINIRF_LABEL).expect("label parses");
        let raster = decode_raster(&minirf_raster(), &meta).expect("raster decodes");
        assert_eq!(raster.bands, 1);
        assert_eq!(raster.lines, 2);
        assert_eq!(raster.samples, 2);
        assert_eq!(
            raster.values,
            vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]
        );
    }

    #[test]
    fn decode_honors_line_prefix_bytes_and_offset() {
        let mut meta = parse_label(MINIRF_LABEL).expect("label parses");
        meta.line_prefix_bytes = Some(4);
        meta.record_bytes = Some(8);
        let mut raw = Vec::new();
        raw.extend_from_slice(&[0xAAu8, 0xBB, 0xCC, 0xDD]);
        raw.extend_from_slice(&10i16.to_be_bytes());
        raw.extend_from_slice(&20i16.to_be_bytes());
        raw.extend_from_slice(&[0xAAu8, 0xBB, 0xCC, 0xDD]);
        raw.extend_from_slice(&30i16.to_be_bytes());
        raw.extend_from_slice(&40i16.to_be_bytes());
        let raster = decode_raster(&raw, &meta).expect("raster decodes");
        assert_eq!(
            raster.values,
            vec![Some(10.0), Some(20.0), Some(30.0), Some(40.0)]
        );
    }

    #[test]
    fn decode_rejects_an_unknown_sample_type_by_name() {
        let mut meta = parse_label(MINIRF_LABEL).expect("label parses");
        meta.sample_type = Some("BAUDOT".to_string());
        let err = decode_raster(&minirf_raster(), &meta).expect_err("rejected");
        assert_eq!(err, ImgReject::UnknownSampleType("BAUDOT".to_string()));
        assert!(err.reason().contains("BAUDOT"));
    }

    #[test]
    fn decode_rejects_ieee_real_without_byte_order() {
        let mut meta = parse_label(MINIRF_LABEL).expect("label parses");
        meta.sample_type = Some("IEEE_REAL".to_string());
        meta.sample_bits = Some(32);
        let err = decode_raster(&minirf_raster(), &meta).expect_err("rejected");
        assert!(matches!(err, ImgReject::UndeterminedByteOrder(_)));
    }

    #[test]
    fn decode_maps_missing_constant_to_absence_not_zero() {
        let mut meta = parse_label(MINIRF_LABEL).expect("label parses");
        meta.missing_constant = Some(3.0);
        let raster = decode_raster(&minirf_raster(), &meta).expect("raster decodes");
        assert_eq!(raster.values, vec![Some(1.0), Some(2.0), None, Some(4.0)]);
    }

    const M3_HDR: &str = "ENVI
samples = 2
lines = 2
bands = 2
data type = 4
byte order = 0
interleave = bil
header offset = 0
data ignore value = -99.0
";

    #[test]
    fn parse_envi_hdr_reads_the_cube_layout() {
        let meta = parse_envi_hdr(M3_HDR).expect("hdr parses");
        assert_eq!(meta.line_samples, Some(2));
        assert_eq!(meta.lines, Some(2));
        assert_eq!(meta.bands, Some(2));
        assert_eq!(meta.envi_data_type, Some(4));
        assert_eq!(meta.byte_order, Some(0));
        assert_eq!(meta.interleave.as_deref(), Some("bil"));
        assert_eq!(meta.missing_constant, Some(-99.0));
    }

    #[test]
    fn decode_envi_bil_f32_reads_all_bands() {
        let meta = parse_envi_hdr(M3_HDR).expect("hdr parses");
        let vals = [1.0f32, 2.0, 10.0, 20.0, 3.0, 4.0, 30.0, 40.0];
        let mut raw = Vec::new();
        for v in vals {
            raw.extend_from_slice(&v.to_le_bytes());
        }
        let raster = decode_envi(&raw, &meta).expect("envi decodes");
        assert_eq!(raster.bands, 2);
        assert_eq!(raster.values.len(), 8);
        assert_eq!(
            raster.values,
            vec![
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(10.0),
                Some(20.0),
                Some(30.0),
                Some(40.0),
            ]
        );
    }

    #[test]
    fn decode_envi_rejects_an_unknown_data_type_by_name() {
        let mut meta = parse_envi_hdr(M3_HDR).expect("hdr parses");
        meta.envi_data_type = Some(99);
        let err = decode_envi(&[], &meta).expect_err("rejected");
        assert_eq!(err, ImgReject::UnsupportedEnviType(99));
    }

    #[test]
    fn pack_roundtrip_and_band_means_hold() {
        let meta = parse_label(MINIRF_LABEL).expect("label parses");
        let raster = decode_raster(&minirf_raster(), &meta).expect("raster decodes");
        let bin = pack(&raster);
        let parsed = parse_image(&bin).expect("packed image parses");
        assert_eq!(parsed, raster);
        let means = band_means(&parsed).expect("band means");
        assert_eq!(means, vec![(0.0, 2.5, 0)]);
        let series = parse_series(&bin).expect("series");
        assert_eq!(series, vec![(0.0, 2.5, 0)]);
    }

    #[test]
    fn parse_image_rejects_foreign_bytes() {
        assert!(parse_image(b"").is_none());
        assert!(parse_image(b"XXXX").is_none());
        assert!(parse_image(b"P3BN000000000000").is_none());
    }
}
