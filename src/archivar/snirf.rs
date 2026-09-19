use super::hdf5::Hdf5File;

pub const MAGIC: [u8; 4] = *b"SNIR";
pub const VERSION: u32 = 1;

pub const FLAG_TIME: u32 = 1;
pub const FLAG_TIME_SPACING: u32 = 2;
pub const FLAG_SRC_3D: u32 = 4;
pub const FLAG_DET_3D: u32 = 8;
pub const FLAG_DOUBLE: u32 = 16;
pub const FLAG_SRC_POS: u32 = 32;
pub const FLAG_DET_POS: u32 = 64;
pub const FLAG_MEASLIST: u32 = 128;

const HEADER_LEN: usize = 100;

#[derive(Clone, Debug, PartialEq)]
pub enum Samples {
    Single(Vec<f32>),
    Double(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeBase {
    Series(Vec<f64>),
    Spacing { start: f64, step: f64 },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Geometry {
    Pos2(Vec<(f64, f64)>),
    Pos3(Vec<(f64, f64, f64)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnirfChannel {
    pub source_index: u32,
    pub detector_index: u32,
    pub wavelength_index: u32,
    pub data_type_index: Option<u32>,
    pub data_type_label: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SnirfBin {
    pub nchan: u32,
    pub pnts: u64,
    pub nwavelengths: u32,
    pub nsources: u32,
    pub ndetectors: u32,
    pub sha256: [u8; 32],
    pub snapshot_tag: String,
    pub hexsha: String,
    pub origin_url: String,
    pub labels: Vec<String>,
    pub wavelengths: Vec<f64>,
    pub source_geometry: Option<Geometry>,
    pub detector_geometry: Option<Geometry>,
    pub meas_list: Option<Vec<SnirfChannel>>,
    pub time: Option<TimeBase>,
    pub samples: Samples,
}

#[derive(Clone, Debug)]
pub struct SnirfExtract {
    pub nchan: u32,
    pub pnts: u64,
    pub nwavelengths: u32,
    pub nsources: u32,
    pub ndetectors: u32,
    pub labels: Vec<String>,
    pub wavelengths: Vec<f64>,
    pub source_geometry: Option<Geometry>,
    pub detector_geometry: Option<Geometry>,
    pub meas_list: Vec<SnirfChannel>,
    pub time: Option<TimeBase>,
    pub samples: Samples,
}

fn geometry_rows(g: &Geometry) -> usize {
    match g {
        Geometry::Pos2(v) => v.len(),
        Geometry::Pos3(v) => v.len(),
    }
}

fn write_geometry(buf: &mut Vec<u8>, g: &Geometry, rows: usize) -> Option<()> {
    match g {
        Geometry::Pos2(v) => {
            if v.len() != rows || v.iter().any(|(x, y)| !x.is_finite() || !y.is_finite()) {
                return None;
            }
            for (x, y) in v {
                buf.extend_from_slice(&x.to_le_bytes());
                buf.extend_from_slice(&y.to_le_bytes());
            }
        }
        Geometry::Pos3(v) => {
            if v.len() != rows
                || v
                    .iter()
                    .any(|(x, y, z)| !x.is_finite() || !y.is_finite() || !z.is_finite())
            {
                return None;
            }
            for (x, y, z) in v {
                buf.extend_from_slice(&x.to_le_bytes());
                buf.extend_from_slice(&y.to_le_bytes());
                buf.extend_from_slice(&z.to_le_bytes());
            }
        }
    }
    Some(())
}

pub fn write_bin(bin: &SnirfBin) -> Option<Vec<u8>> {
    if bin.labels.len() != bin.nchan as usize {
        return None;
    }
    if bin.wavelengths.len() != bin.nwavelengths as usize {
        return None;
    }
    if bin.wavelengths.iter().any(|w| !w.is_finite() || *w <= 0.0) {
        return None;
    }
    let meas: Option<&[SnirfChannel]> = bin.meas_list.as_deref();
    if let Some(list) = meas {
        if list.len() != bin.nchan as usize {
            return None;
        }
        for ch in list {
            if ch.source_index == 0
                || ch.detector_index == 0
                || ch.wavelength_index == 0
                || ch.wavelength_index > bin.nwavelengths
            {
                return None;
            }
        }
    }
    let time_n: usize = match &bin.time {
        None => 0,
        Some(TimeBase::Series(v)) => {
            if v.len() as u64 != bin.pnts || v.iter().any(|x| !x.is_finite()) {
                return None;
            }
            u32::try_from(bin.pnts).ok()? as usize
        }
        Some(TimeBase::Spacing { start, step }) => {
            if !start.is_finite() || !step.is_finite() || *step <= 0.0 {
                return None;
            }
            2
        }
    };
    let total = bin.pnts.checked_mul(bin.nchan as u64)?;
    let double = matches!(bin.samples, Samples::Double(_));
    let sample_count = match &bin.samples {
        Samples::Single(s) => s.len(),
        Samples::Double(d) => d.len(),
    };
    if sample_count as u64 != total {
        return None;
    }
    match &bin.samples {
        Samples::Single(s) => {
            if s.iter().any(|x| !x.is_finite()) {
                return None;
            }
        }
        Samples::Double(d) => {
            if d.iter().any(|x| !x.is_finite()) {
                return None;
            }
        }
    }
    let samples_len = total.checked_mul(if double { 8 } else { 4 })?;
    if bin.snapshot_tag.len() > u32::MAX as usize
        || bin.hexsha.len() > u32::MAX as usize
        || bin.origin_url.len() > u32::MAX as usize
    {
        return None;
    }
    if bin.labels.iter().any(|l| l.len() > 0xFFFF) {
        return None;
    }
    if let Some(list) = meas
        && list.iter().any(|c| {
            c.data_type_label
                .as_ref()
                .is_some_and(|s| s.len() > 0xFFFF)
        })
    {
        return None;
    }

    let mut flags = 0u32;
    match &bin.time {
        Some(TimeBase::Series(_)) => flags |= FLAG_TIME,
        Some(TimeBase::Spacing { .. }) => flags |= FLAG_TIME | FLAG_TIME_SPACING,
        None => {}
    }
    if matches!(bin.source_geometry, Some(Geometry::Pos3(_))) {
        flags |= FLAG_SRC_3D;
    }
    if matches!(bin.detector_geometry, Some(Geometry::Pos3(_))) {
        flags |= FLAG_DET_3D;
    }
    if double {
        flags |= FLAG_DOUBLE;
    }
    if bin.source_geometry.is_some() {
        flags |= FLAG_SRC_POS;
    }
    if bin.detector_geometry.is_some() {
        flags |= FLAG_DET_POS;
    }
    if meas.is_some() {
        flags |= FLAG_MEASLIST;
    }

    let mut buf = Vec::new();
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&VERSION.to_le_bytes());
    buf.extend_from_slice(&flags.to_le_bytes());
    buf.extend_from_slice(&bin.nchan.to_le_bytes());
    buf.extend_from_slice(&bin.pnts.to_le_bytes());
    buf.extend_from_slice(&bin.nwavelengths.to_le_bytes());
    buf.extend_from_slice(&bin.nsources.to_le_bytes());
    buf.extend_from_slice(&bin.ndetectors.to_le_bytes());
    buf.extend_from_slice(&bin.sha256);
    buf.extend_from_slice(&(bin.snapshot_tag.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(bin.hexsha.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(bin.origin_url.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(bin.labels.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(meas.map_or(0, |m| m.len()) as u32).to_le_bytes());
    buf.extend_from_slice(&(time_n as u32).to_le_bytes());
    buf.extend_from_slice(&samples_len.to_le_bytes());

    buf.extend_from_slice(bin.snapshot_tag.as_bytes());
    buf.extend_from_slice(bin.hexsha.as_bytes());
    buf.extend_from_slice(bin.origin_url.as_bytes());
    for label in &bin.labels {
        buf.extend_from_slice(&(label.len() as u16).to_le_bytes());
        buf.extend_from_slice(label.as_bytes());
    }
    for w in &bin.wavelengths {
        buf.extend_from_slice(&w.to_le_bytes());
    }
    if let Some(g) = &bin.source_geometry {
        write_geometry(&mut buf, g, bin.nsources as usize)?;
    }
    if let Some(g) = &bin.detector_geometry {
        write_geometry(&mut buf, g, bin.ndetectors as usize)?;
    }
    if let Some(list) = meas {
        for ch in list {
            buf.extend_from_slice(&ch.source_index.to_le_bytes());
            buf.extend_from_slice(&ch.detector_index.to_le_bytes());
            buf.extend_from_slice(&ch.wavelength_index.to_le_bytes());
            buf.push(ch.data_type_index.is_some() as u8);
            if let Some(v) = ch.data_type_index {
                buf.extend_from_slice(&v.to_le_bytes());
            }
            buf.push(ch.data_type_label.is_some() as u8);
            if let Some(s) = &ch.data_type_label {
                buf.extend_from_slice(&(s.len() as u16).to_le_bytes());
                buf.extend_from_slice(s.as_bytes());
            }
        }
    }
    match &bin.time {
        Some(TimeBase::Series(v)) => {
            for x in v {
                buf.extend_from_slice(&x.to_le_bytes());
            }
        }
        Some(TimeBase::Spacing { start, step }) => {
            buf.extend_from_slice(&start.to_le_bytes());
            buf.extend_from_slice(&step.to_le_bytes());
        }
        None => {}
    }
    match &bin.samples {
        Samples::Single(s) => {
            for v in s {
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
        Samples::Double(d) => {
            for v in d {
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
    Some(buf)
}

fn take_str(bytes: &[u8], off: &mut usize, len: usize) -> Option<String> {
    let s = String::from_utf8(bytes.get(*off..*off + len)?.to_vec()).ok()?;
    *off += len;
    Some(s)
}

fn take_bytes(bytes: &[u8], off: &mut usize, len: usize) -> Option<Vec<u8>> {
    let b = bytes.get(*off..*off + len)?.to_vec();
    *off += len;
    Some(b)
}

fn take_geometry(bytes: &[u8], off: &mut usize, rows: u32, three: bool) -> Option<Geometry> {
    if three {
        let mut out = Vec::with_capacity(rows as usize);
        for _ in 0..rows {
            let x = f64::from_le_bytes(bytes.get(*off..*off + 8)?.try_into().ok()?);
            *off += 8;
            let y = f64::from_le_bytes(bytes.get(*off..*off + 8)?.try_into().ok()?);
            *off += 8;
            let z = f64::from_le_bytes(bytes.get(*off..*off + 8)?.try_into().ok()?);
            *off += 8;
            out.push((x, y, z));
        }
        Some(Geometry::Pos3(out))
    } else {
        let mut out = Vec::with_capacity(rows as usize);
        for _ in 0..rows {
            let x = f64::from_le_bytes(bytes.get(*off..*off + 8)?.try_into().ok()?);
            *off += 8;
            let y = f64::from_le_bytes(bytes.get(*off..*off + 8)?.try_into().ok()?);
            *off += 8;
            out.push((x, y));
        }
        Some(Geometry::Pos2(out))
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<SnirfBin> {
    if bytes.len() < HEADER_LEN || bytes[0..4] != MAGIC {
        return None;
    }
    if u32::from_le_bytes(bytes[4..8].try_into().ok()?) != VERSION {
        return None;
    }
    let flags = u32::from_le_bytes(bytes[8..12].try_into().ok()?);
    let nchan = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let pnts = u64::from_le_bytes(bytes[16..24].try_into().ok()?);
    let nwavelengths = u32::from_le_bytes(bytes[24..28].try_into().ok()?);
    let nsources = u32::from_le_bytes(bytes[28..32].try_into().ok()?);
    let ndetectors = u32::from_le_bytes(bytes[32..36].try_into().ok()?);
    let sha256: [u8; 32] = bytes[36..68].try_into().ok()?;
    let tag_len = u32::from_le_bytes(bytes[68..72].try_into().ok()?) as usize;
    let hexsha_len = u32::from_le_bytes(bytes[72..76].try_into().ok()?) as usize;
    let url_len = u32::from_le_bytes(bytes[76..80].try_into().ok()?) as usize;
    let labels_n = u32::from_le_bytes(bytes[80..84].try_into().ok()?) as usize;
    let meas_n = u32::from_le_bytes(bytes[84..88].try_into().ok()?) as usize;
    let time_n = u32::from_le_bytes(bytes[88..92].try_into().ok()?) as usize;
    let samples_len = u64::from_le_bytes(bytes[92..100].try_into().ok()?);

    if labels_n != nchan as usize {
        return None;
    }
    let has_meas = flags & FLAG_MEASLIST != 0;
    if has_meas {
        if meas_n != nchan as usize {
            return None;
        }
    } else if meas_n != 0 {
        return None;
    }
    let has_time = flags & FLAG_TIME != 0;
    let spacing = flags & FLAG_TIME_SPACING != 0;
    if spacing && !has_time {
        return None;
    }
    if has_time {
        if spacing {
            if time_n != 2 {
                return None;
            }
        } else if time_n as u64 != pnts {
            return None;
        }
    } else if time_n != 0 {
        return None;
    }
    if flags & FLAG_SRC_POS == 0 && flags & FLAG_SRC_3D != 0 {
        return None;
    }
    if flags & FLAG_DET_POS == 0 && flags & FLAG_DET_3D != 0 {
        return None;
    }
    let double = flags & FLAG_DOUBLE != 0;
    let total = pnts.checked_mul(nchan as u64)?;
    if samples_len != total.checked_mul(if double { 8 } else { 4 })? {
        return None;
    }

    let mut off = HEADER_LEN;
    let snapshot_tag = take_str(bytes, &mut off, tag_len)?;
    let hexsha = take_str(bytes, &mut off, hexsha_len)?;
    let origin_url = take_str(bytes, &mut off, url_len)?;
    let mut labels = Vec::with_capacity(labels_n);
    for _ in 0..labels_n {
        let len = u16::from_le_bytes(bytes.get(off..off + 2)?.try_into().ok()?) as usize;
        off += 2;
        labels.push(take_str(bytes, &mut off, len)?);
    }
    let mut wavelengths = Vec::with_capacity(nwavelengths as usize);
    for _ in 0..nwavelengths as usize {
        wavelengths.push(f64::from_le_bytes(
            bytes.get(off..off + 8)?.try_into().ok()?,
        ));
        off += 8;
    }
    let source_geometry = if flags & FLAG_SRC_POS != 0 {
        Some(take_geometry(bytes, &mut off, nsources, flags & FLAG_SRC_3D != 0)?)
    } else {
        None
    };
    let detector_geometry = if flags & FLAG_DET_POS != 0 {
        Some(take_geometry(bytes, &mut off, ndetectors, flags & FLAG_DET_3D != 0)?)
    } else {
        None
    };
    let meas_list = if has_meas {
        let mut list = Vec::with_capacity(meas_n);
        for _ in 0..meas_n {
            let source_index = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            let detector_index = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            let wavelength_index = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            let has_dti = *bytes.get(off)? != 0;
            off += 1;
            let data_type_index = if has_dti {
                let v = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
                off += 4;
                Some(v)
            } else {
                None
            };
            let has_dtl = *bytes.get(off)? != 0;
            off += 1;
            let data_type_label = if has_dtl {
                let len = u16::from_le_bytes(bytes.get(off..off + 2)?.try_into().ok()?) as usize;
                off += 2;
                Some(take_str(bytes, &mut off, len)?)
            } else {
                None
            };
            list.push(SnirfChannel {
                source_index,
                detector_index,
                wavelength_index,
                data_type_index,
                data_type_label,
            });
        }
        Some(list)
    } else {
        None
    };
    let time = if has_time {
        if spacing {
            let start = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            let step = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            Some(TimeBase::Spacing { start, step })
        } else {
            let mut v = Vec::with_capacity(time_n);
            for _ in 0..time_n {
                v.push(f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?));
                off += 8;
            }
            Some(TimeBase::Series(v))
        }
    } else {
        None
    };
    let sample_bytes = take_bytes(bytes, &mut off, samples_len as usize)?;
    if off != bytes.len() {
        return None;
    }
    let samples = if double {
        let mut v = Vec::with_capacity(total as usize);
        for i in 0..total as usize {
            v.push(f64::from_le_bytes(
                sample_bytes[i * 8..i * 8 + 8].try_into().ok()?,
            ));
        }
        Samples::Double(v)
    } else {
        let mut v = Vec::with_capacity(total as usize);
        for i in 0..total as usize {
            v.push(f32::from_le_bytes(
                sample_bytes[i * 4..i * 4 + 4].try_into().ok()?,
            ));
        }
        Samples::Single(v)
    };
    Some(SnirfBin {
        nchan,
        pnts,
        nwavelengths,
        nsources,
        ndetectors,
        sha256,
        snapshot_tag,
        hexsha,
        origin_url,
        labels,
        wavelengths,
        source_geometry,
        detector_geometry,
        meas_list,
        time,
        samples,
    })
}

fn index_array(file: &Hdf5File, path: &str) -> Result<Vec<f64>, String> {
    file.read_f64_dataset(path)
        .map_err(|n| format!("{path} absent: {n:?}"))
}

fn index_of(v: f64, name: &str, k: u32) -> Result<u32, String> {
    if !v.is_finite() || v.fract() != 0.0 || v < 1.0 || v > u32::MAX as f64 {
        return Err(format!("{name}[{k}] carries {v}"));
    }
    Ok(v as u32)
}

fn data_type_of(v: f64, k: u32) -> Result<u32, String> {
    if !v.is_finite() || v.fract() != 0.0 || v < 0.0 || v > u32::MAX as f64 {
        return Err(format!("dataTypeIndex[{k}] carries {v}"));
    }
    Ok(v as u32)
}

fn read_label_array(file: &Hdf5File, path: &str) -> Option<Vec<Option<String>>> {
    let (_, ds, dt) = file.dataset(path).ok()?;
    if dt.class != 3 {
        return None;
    }
    let raw = file.read_dataset(path).ok()?;
    let (rows, elem) = if ds.dims.is_empty() {
        (1usize, dt.size)
    } else if ds.dims.len() == 1 {
        (ds.dims[0] as usize, dt.size)
    } else if ds.dims.len() == 2 && dt.size == 1 {
        (ds.dims[0] as usize, ds.dims[1] as usize)
    } else {
        return None;
    };
    if elem == 0 || raw.len() != rows * elem {
        return None;
    }
    Some(
        raw.chunks_exact(elem)
            .map(|c| {
                let s = String::from_utf8_lossy(c)
                    .trim_end_matches('\0')
                    .trim()
                    .to_string();
                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            })
            .collect(),
    )
}

fn channel_rows(file: &Hdf5File, base: &str) -> Result<Vec<SnirfChannel>, String> {
    let src = index_array(file, &format!("{base}/sourceIndex"))?;
    let n = src.len();
    if n == 0 {
        return Err(format!("{base}/sourceIndex carries no channel"));
    }
    let det = index_array(file, &format!("{base}/detectorIndex"))?;
    let wl = index_array(file, &format!("{base}/wavelengthIndex"))?;
    if det.len() != n || wl.len() != n {
        return Err(format!(
            "{base}: sourceIndex {n} vs detectorIndex {} wavelengthIndex {} — the witnesses disagree",
            det.len(),
            wl.len()
        ));
    }
    let dti: Option<Vec<f64>> = match index_array(file, &format!("{base}/dataTypeIndex")) {
        Ok(v) => Some(v),
        Err(_) => None,
    };
    if let Some(v) = &dti
        && v.len() != n
    {
        return Err(format!("{base}: sourceIndex {n} vs dataTypeIndex {}", v.len()));
    }
    let dtl = read_label_array(file, &format!("{base}/dataTypeLabel"));
    if let Some(v) = &dtl
        && v.len() != n
    {
        return Err(format!("{base}: sourceIndex {n} vs dataTypeLabel {}", v.len()));
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(SnirfChannel {
            source_index: index_of(src[i], "sourceIndex", i as u32)?,
            detector_index: index_of(det[i], "detectorIndex", i as u32)?,
            wavelength_index: index_of(wl[i], "wavelengthIndex", i as u32)?,
            data_type_index: match &dti {
                Some(v) => Some(data_type_of(v[i], i as u32)?),
                None => None,
            },
            data_type_label: match &dtl {
                Some(v) => v[i].clone(),
                None => None,
            },
        });
    }
    Ok(out)
}

fn measurement_channels(file: &Hdf5File) -> Result<Vec<SnirfChannel>, String> {
    let mut names: Vec<String> = file
        .links_of("/nirs/data1")
        .iter()
        .map(|l| l.name.clone())
        .collect();
    names.sort();
    let ml: Vec<&String> = names
        .iter()
        .filter(|n| n.starts_with("measurementList"))
        .collect();
    if ml.is_empty() {
        return Err("measurementList absent — the channel identity stays unmapped".into());
    }
    if ml.iter().any(|n| *n == "measurementLists") {
        return channel_rows(file, "/nirs/data1/measurementLists");
    }
    if ml.len() == 1 {
        return channel_rows(file, &format!("/nirs/data1/{}", ml[0]));
    }
    let mut numbered: Vec<(u32, String)> = Vec::new();
    for n in &ml {
        let Some(suffix) = n.strip_prefix("measurementList") else {
            continue;
        };
        let Ok(k) = suffix.parse::<u32>() else {
            continue;
        };
        numbered.push((k, n.to_string()));
    }
    numbered.sort_by_key(|(k, _)| *k);
    let mut out = Vec::new();
    for (k, name) in &numbered {
        let base = format!("/nirs/data1/{name}");
        let src = index_array(file, &format!("{base}/sourceIndex"))?;
        let det = index_array(file, &format!("{base}/detectorIndex"))?;
        let wl = index_array(file, &format!("{base}/wavelengthIndex"))?;
        if src.len() != 1 || det.len() != 1 || wl.len() != 1 {
            return Err(format!(
                "{base}: read {} {} {} — not three scalars",
                src.len(),
                det.len(),
                wl.len()
            ));
        }
        let dti = index_array(file, &format!("{base}/dataTypeIndex"))
            .ok()
            .and_then(|v| v.into_iter().next());
        let dtl = read_label_array(file, &format!("{base}/dataTypeLabel"))
            .and_then(|v| v.into_iter().next().flatten());
        out.push(SnirfChannel {
            source_index: index_of(src[0], "sourceIndex", *k)?,
            detector_index: index_of(det[0], "detectorIndex", *k)?,
            wavelength_index: index_of(wl[0], "wavelengthIndex", *k)?,
            data_type_index: match dti {
                Some(v) => Some(data_type_of(v, *k)?),
                None => None,
            },
            data_type_label: dtl,
        });
    }
    Ok(out)
}

fn probe_geometry(file: &Hdf5File, kind: &str) -> Result<Option<Geometry>, String> {
    for (suffix, ndim) in [("Pos3D", 3u64), ("Pos2D", 2u64)] {
        let path = format!("/nirs/probe/{kind}{suffix}");
        let Ok((_, gds, _)) = file.dataset(&path) else {
            continue;
        };
        let dims = gds.dims.clone();
        if dims.len() != 2 || dims[1] != ndim {
            return Err(format!("{path} dims {dims:?} — not [n,{ndim}]"));
        }
        let rows = dims[0];
        if rows == 0 {
            return Err(format!("{path} carries no position"));
        }
        let vals = file
            .read_f64_dataset(&path)
            .map_err(|n| format!("{path} read: {n:?}"))?;
        if vals.len() as u64 != rows * ndim {
            return Err(format!(
                "{path} carries {} values vs {rows}×{ndim}",
                vals.len()
            ));
        }
        if let Some(v) = vals.iter().find(|x| !x.is_finite()) {
            return Err(format!("{path} carries {v}"));
        }
        let g = if ndim == 3 {
            let mut v = Vec::with_capacity(rows as usize);
            for i in 0..rows as usize {
                v.push((vals[i * 3], vals[i * 3 + 1], vals[i * 3 + 2]));
            }
            Geometry::Pos3(v)
        } else {
            let mut v = Vec::with_capacity(rows as usize);
            for i in 0..rows as usize {
                v.push((vals[i * 2], vals[i * 2 + 1]));
            }
            Geometry::Pos2(v)
        };
        return Ok(Some(g));
    }
    Ok(None)
}

pub fn parse_snirf(bytes: &[u8]) -> Result<SnirfExtract, String> {
    let file = Hdf5File::parse(bytes).map_err(|n| format!("hdf5 parse: {n:?}"))?;
    let (_, ds, dt) = file
        .dataset("/nirs/data1/dataTimeSeries")
        .map_err(|n| format!("/nirs/data1/dataTimeSeries absent: {n:?}"))?;
    if ds.dims.len() != 2 {
        return Err(format!(
            "dataTimeSeries rank {} — not a [time,channel] field",
            ds.dims.len()
        ));
    }
    let (d0, d1) = (ds.dims[0], ds.dims[1]);
    let sample_double = match (dt.class, dt.size) {
        (1, 8) => true,
        (1, 4) => false,
        (0, _) => true,
        (c, s) => {
            return Err(format!(
                "dataTimeSeries datatype class {c} size {s} — not numeric"
            ))
        }
    };
    let values = file
        .read_f64_dataset("/nirs/data1/dataTimeSeries")
        .map_err(|n| format!("dataTimeSeries read: {n:?}"))?;
    if values.len() as u64 != d0.checked_mul(d1).ok_or("dataTimeSeries dims overflow")? {
        return Err(format!(
            "dataTimeSeries carries {} values vs {d0}×{d1}",
            values.len()
        ));
    }

    let channels = measurement_channels(&file)?;
    let n_ml = channels.len();
    let nchan = u32::try_from(n_ml)
        .map_err(|_| format!("measurementList length {n_ml} beyond u32"))?;
    let (channel_in_dim0, pnts) = match (d0 == n_ml as u64, d1 == n_ml as u64) {
        (true, true) => {
            return Err(format!(
                "the channel axis stays ambiguous — both dataTimeSeries dims {d0} match the measurementList"
            ))
        }
        (true, false) => (true, d1),
        (false, true) => (false, d0),
        (false, false) => {
            return Err(format!(
                "measurementList length {n_ml} matches neither dataTimeSeries dim {d0} nor {d1}"
            ))
        }
    };
    if pnts == 0 {
        return Err("dataTimeSeries carries no time row".into());
    }

    let time_raw = file
        .read_f64_dataset("/nirs/data1/time")
        .map_err(|n| format!("/nirs/data1/time absent: {n:?}"))?;
    let time = match time_raw.len() {
        2 => {
            let start = time_raw[0];
            let step = time_raw[1];
            if !start.is_finite() {
                return Err(format!("time[0] carries {start}"));
            }
            if !step.is_finite() || step <= 0.0 {
                return Err(format!("time spacing carries {step}"));
            }
            Some(TimeBase::Spacing { start, step })
        }
        n if n as u64 == pnts => {
            if let Some(v) = time_raw.iter().find(|x| !x.is_finite()) {
                return Err(format!("time carries {v}"));
            }
            Some(TimeBase::Series(time_raw))
        }
        n => {
            return Err(format!(
                "time carries {n} points, the dataTimeSeries witnesses {pnts} rows"
            ))
        }
    };

    let wavelengths = file
        .read_f64_dataset("/nirs/probe/wavelengths")
        .map_err(|n| format!("/nirs/probe/wavelengths absent: {n:?}"))?;
    if wavelengths.is_empty() {
        return Err("wavelengths carries no value".into());
    }
    for w in &wavelengths {
        if !w.is_finite() || *w <= 0.0 {
            return Err(format!("wavelength carries {w}"));
        }
    }
    let nwavelengths = u32::try_from(wavelengths.len())
        .map_err(|_| format!("wavelengths {} beyond u32", wavelengths.len()))?;

    let source_geometry = probe_geometry(&file, "source")?;
    let detector_geometry = probe_geometry(&file, "detector")?;
    let nsources = match &source_geometry {
        Some(g) => {
            u32::try_from(geometry_rows(g)).map_err(|_| "source geometry rows beyond u32".to_string())?
        }
        None => channels
            .iter()
            .map(|c| c.source_index)
            .max()
            .ok_or("measurementList carries no source")?,
    };
    let ndetectors = match &detector_geometry {
        Some(g) => {
            u32::try_from(geometry_rows(g)).map_err(|_| "detector geometry rows beyond u32".to_string())?
        }
        None => channels
            .iter()
            .map(|c| c.detector_index)
            .max()
            .ok_or("measurementList carries no detector")?,
    };
    for ch in &channels {
        if ch.source_index > nsources {
            return Err(format!(
                "sourceIndex {} exceeds {nsources} source positions",
                ch.source_index
            ));
        }
        if ch.detector_index > ndetectors {
            return Err(format!(
                "detectorIndex {} exceeds {ndetectors} detector positions",
                ch.detector_index
            ));
        }
        if ch.wavelength_index > nwavelengths {
            return Err(format!(
                "wavelengthIndex {} exceeds {nwavelengths} wavelengths",
                ch.wavelength_index
            ));
        }
    }
    let labels: Vec<String> = channels
        .iter()
        .map(|c| {
            format!(
                "S{}-D{}@{}",
                c.source_index, c.detector_index, c.wavelength_index
            )
        })
        .collect();

    let total = pnts
        .checked_mul(nchan as u64)
        .ok_or("samples dims overflow")?;
    let mut out: Vec<f64> = Vec::with_capacity(total as usize);
    for t in 0..pnts {
        for c in 0..n_ml {
            let i = if channel_in_dim0 {
                c as u64 * pnts + t
            } else {
                t * n_ml as u64 + c as u64
            };
            let v = values[i as usize];
            if !v.is_finite() {
                return Err(format!("dataTimeSeries[{i}] carries {v}"));
            }
            out.push(v);
        }
    }
    let samples = if sample_double {
        Samples::Double(out)
    } else {
        Samples::Single(out.iter().map(|v| *v as f32).collect())
    };
    Ok(SnirfExtract {
        nchan,
        pnts,
        nwavelengths,
        nsources,
        ndetectors,
        labels,
        wavelengths,
        source_geometry,
        detector_geometry,
        meas_list: channels,
        time,
        samples,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct H5 {
        bytes: Vec<u8>,
    }

    impl H5 {
        fn new() -> H5 {
            let mut h = H5 { bytes: Vec::new() };
            h.reserve8(96);
            h
        }

        fn aligned(&self) -> usize {
            (self.bytes.len() + 7) & !7
        }

        fn alloc8(&mut self, data: &[u8]) -> usize {
            let a = self.aligned();
            self.bytes.resize(a, 0);
            self.bytes.extend_from_slice(data);
            a
        }

        fn reserve8(&mut self, n: usize) -> usize {
            let a = self.aligned();
            self.bytes.resize(a + n, 0);
            a
        }

        fn patch(&mut self, addr: usize, data: &[u8]) {
            self.bytes[addr..addr + data.len()].copy_from_slice(data);
        }

        fn ds(&mut self, class: u8, size: usize, dims: &[u64], raw: &[u8]) -> usize {
            let signed = matches!(class, 0 | 1);
            let data_addr = self.alloc8(raw) as u64;
            let msgs: Vec<(u16, Vec<u8>)> = vec![
                (0x01, dataspace_v1(dims)),
                (0x03, datatype_v1(class, size, signed)),
                (0x08, layout_contig(data_addr, raw.len() as u64)),
            ];
            self.alloc8(&obj_header_v1(&msgs))
        }

        fn group(&mut self, children: &[(&str, usize)]) -> usize {
            let heap_addr = self.reserve8(32);
            let data_addr = self.aligned();
            let mut names = Vec::new();
            let mut offsets = Vec::new();
            let mut off = 0usize;
            for (n, _) in children {
                offsets.push(off);
                names.extend_from_slice(n.as_bytes());
                names.push(0);
                off += n.len() + 1;
            }
            let mut heap = Vec::new();
            heap.extend_from_slice(b"HEAP");
            heap.extend_from_slice(&0u32.to_le_bytes());
            heap.extend_from_slice(&(names.len() as u64).to_le_bytes());
            heap.extend_from_slice(&0u64.to_le_bytes());
            heap.extend_from_slice(&(data_addr as u64).to_le_bytes());
            self.patch(heap_addr, &heap);
            self.alloc8(&names);
            let mut snod = Vec::new();
            snod.extend_from_slice(b"SNOD");
            snod.push(1);
            snod.push(0);
            snod.extend_from_slice(&(children.len() as u16).to_le_bytes());
            for ((_, addr), name_off) in children.iter().zip(offsets) {
                snod.extend_from_slice(&(name_off as u64).to_le_bytes());
                snod.extend_from_slice(&(*addr as u64).to_le_bytes());
                snod.extend_from_slice(&[0u8; 24]);
            }
            let snod_addr = self.alloc8(&snod);
            let mut symtab = Vec::new();
            symtab.extend_from_slice(&(snod_addr as u64).to_le_bytes());
            symtab.extend_from_slice(&(heap_addr as u64).to_le_bytes());
            self.alloc8(&obj_header_v1(&[(0x11, symtab)]))
        }

        fn finish(mut self, root_addr: usize) -> Vec<u8> {
            let eof = self.aligned();
            let mut sb = vec![0u8; 96];
            sb[0..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
            sb[13] = 8;
            sb[14] = 8;
            sb[16..18].copy_from_slice(&4u16.to_le_bytes());
            sb[18..20].copy_from_slice(&16u16.to_le_bytes());
            sb[32..40].copy_from_slice(&u64::MAX.to_le_bytes());
            sb[40..48].copy_from_slice(&(eof as u64).to_le_bytes());
            sb[48..56].copy_from_slice(&u64::MAX.to_le_bytes());
            sb[64..72].copy_from_slice(&(root_addr as u64).to_le_bytes());
            self.patch(0, &sb);
            self.bytes
        }
    }

    fn dataspace_v1(dims: &[u64]) -> Vec<u8> {
        let mut d = vec![1u8, dims.len() as u8, 0, 0, 0, 0, 0, 0];
        for dim in dims {
            d.extend_from_slice(&dim.to_le_bytes());
        }
        d
    }

    fn datatype_v1(class: u8, size: usize, signed: bool) -> Vec<u8> {
        let mut d = vec![class | 0x10, if signed { 0x08 } else { 0x00 }, 0, 0];
        d.extend_from_slice(&(size as u32).to_le_bytes());
        match class {
            0 | 1 => {
                d.extend_from_slice(&0u16.to_le_bytes());
                d.extend_from_slice(&((size as u16) * 8).to_le_bytes());
                if class == 1 {
                    d.extend_from_slice(&[0u8; 8]);
                }
            }
            _ => {}
        }
        d
    }

    fn layout_contig(addr: u64, size: u64) -> Vec<u8> {
        let mut d = vec![3u8, 1u8];
        d.extend_from_slice(&addr.to_le_bytes());
        d.extend_from_slice(&size.to_le_bytes());
        d
    }

    fn obj_header_v1(msgs: &[(u16, Vec<u8>)]) -> Vec<u8> {
        let mut body = Vec::new();
        for (typ, data) in msgs {
            body.extend_from_slice(&typ.to_le_bytes());
            body.extend_from_slice(&(data.len() as u16).to_le_bytes());
            body.push(0);
            body.extend_from_slice(&[0u8; 3]);
            body.extend_from_slice(data);
        }
        let mut out = Vec::new();
        out.push(1);
        out.push(0);
        out.extend_from_slice(&(msgs.len() as u16).to_le_bytes());
        out.extend_from_slice(&1u32.to_le_bytes());
        out.extend_from_slice(&((16 + body.len()) as u32).to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&body);
        out
    }

    fn f64s(vals: &[f64]) -> Vec<u8> {
        vals.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn f32s(vals: &[f32]) -> Vec<u8> {
        vals.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn i32s(vals: &[i32]) -> Vec<u8> {
        vals.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn probe_group(h: &mut H5, wl: &[f64], src_pos: &[f64], det_pos: &[f64]) -> usize {
        let wl_ds = h.ds(1, 8, &[wl.len() as u64], &f64s(wl));
        let sp = h.ds(1, 8, &[2, 2], &f64s(src_pos));
        let dp = h.ds(1, 8, &[2, 2], &f64s(det_pos));
        h.group(&[
            ("wavelengths", wl_ds),
            ("sourcePos2D", sp),
            ("detectorPos2D", dp),
        ])
    }

    fn array_ml_group(h: &mut H5, src: &[i32], det: &[i32], wli: &[i32]) -> usize {
        let n = src.len();
        let si = h.ds(0, 4, &[n as u64], &i32s(src));
        let di = h.ds(0, 4, &[n as u64], &i32s(det));
        let wl = h.ds(0, 4, &[n as u64], &i32s(wli));
        let dti = h.ds(0, 4, &[n as u64], &i32s(&(1..=n as i32).collect::<Vec<i32>>()));
        let mut raw = Vec::new();
        for s in ["HbO", "HbR"] {
            raw.extend_from_slice(s.as_bytes());
            raw.resize(raw.len() + (8 - s.len()), 0);
        }
        let dtl = h.ds(3, 8, &[2], &raw);
        h.group(&[
            ("sourceIndex", si),
            ("detectorIndex", di),
            ("wavelengthIndex", wl),
            ("dataTypeIndex", dti),
            ("dataTypeLabel", dtl),
        ])
    }

    fn finish_nirs(mut h: H5, probe: usize, data1_children: Vec<(&str, usize)>) -> Vec<u8> {
        let data1 = h.group(&data1_children);
        let nirs = h.group(&[("probe", probe), ("data1", data1)]);
        let root = h.group(&[("nirs", nirs)]);
        h.finish(root)
    }

    fn fixture(ml_name: &str, time: Option<&[f64]>, src: &[i32], wl: &[f64]) -> Vec<u8> {
        let mut h = H5::new();
        let probe = probe_group(&mut h, wl, &[0.0, 0.0, 3.0, 0.0], &[1.0, 0.0, 4.0, 0.0]);
        let series = h.ds(1, 4, &[4, 2], &f32s(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
        let mut children: Vec<(&str, usize)> = vec![("dataTimeSeries", series)];
        if let Some(t) = time {
            let t_ds = h.ds(1, 8, &[t.len() as u64], &f64s(t));
            children.push(("time", t_ds));
        }
        if !ml_name.is_empty() {
            let det = src;
            let wli = src.iter().map(|_| 1).collect::<Vec<i32>>();
            let ml = array_ml_group(&mut h, src, det, &wli);
            children.push((ml_name, ml));
        }
        finish_nirs(h, probe, children)
    }

    fn bin_with_samples(samples: Samples) -> SnirfBin {
        SnirfBin {
            nchan: 2,
            pnts: 3,
            nwavelengths: 2,
            nsources: 2,
            ndetectors: 2,
            sha256: [0xAA; 32],
            snapshot_tag: "1.0.1".to_string(),
            hexsha: "470458bcff173ca37018a9cb7a55c3804ccc1759".to_string(),
            origin_url: "https://s3.amazonaws.com/openneuro.org/ds008192/x.snirf".to_string(),
            labels: vec!["S1-D1@1".to_string(), "S2-D2@1".to_string()],
            wavelengths: vec![760.0, 850.0],
            source_geometry: Some(Geometry::Pos2(vec![(0.0, 0.0), (3.0, 0.0)])),
            detector_geometry: Some(Geometry::Pos2(vec![(1.0, 0.0), (4.0, 0.0)])),
            meas_list: Some(vec![
                SnirfChannel {
                    source_index: 1,
                    detector_index: 1,
                    wavelength_index: 1,
                    data_type_index: Some(1),
                    data_type_label: Some("HbO".to_string()),
                },
                SnirfChannel {
                    source_index: 2,
                    detector_index: 2,
                    wavelength_index: 1,
                    data_type_index: Some(2),
                    data_type_label: None,
                },
            ]),
            time: Some(TimeBase::Series(vec![0.0, 0.1, 0.2])),
            samples,
        }
    }

    #[test]
    fn fixture_extracts_the_snirf_contract() {
        let bytes = fixture(
            "measurementList1",
            Some(&[0.0, 0.1, 0.2, 0.3]),
            &[1, 2],
            &[760.0, 850.0],
        );
        let ex = parse_snirf(&bytes).expect("the fixture extracts");
        assert_eq!(ex.nchan, 2);
        assert_eq!(ex.pnts, 4);
        assert_eq!(ex.nwavelengths, 2);
        assert_eq!(ex.nsources, 2);
        assert_eq!(ex.ndetectors, 2);
        assert_eq!(ex.labels, vec!["S1-D1@1", "S2-D2@1"]);
        assert_eq!(ex.wavelengths, vec![760.0, 850.0]);
        assert_eq!(ex.source_geometry, Some(Geometry::Pos2(vec![(0.0, 0.0), (3.0, 0.0)])));
        assert_eq!(ex.detector_geometry, Some(Geometry::Pos2(vec![(1.0, 0.0), (4.0, 0.0)])));
        assert_eq!(
            ex.meas_list,
            vec![
                SnirfChannel {
                    source_index: 1,
                    detector_index: 1,
                    wavelength_index: 1,
                    data_type_index: Some(1),
                    data_type_label: Some("HbO".to_string()),
                },
                SnirfChannel {
                    source_index: 2,
                    detector_index: 2,
                    wavelength_index: 1,
                    data_type_index: Some(2),
                    data_type_label: Some("HbR".to_string()),
                },
            ]
        );
        assert_eq!(ex.time, Some(TimeBase::Series(vec![0.0, 0.1, 0.2, 0.3])));
        match ex.samples {
            Samples::Single(v) => assert_eq!(v, vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
            _ => panic!("not single"),
        }
    }

    #[test]
    fn measurement_lists_plural_group_extracts() {
        let bytes = fixture(
            "measurementLists",
            Some(&[0.0, 0.1, 0.2, 0.3]),
            &[1, 2],
            &[760.0, 850.0],
        );
        let ex = parse_snirf(&bytes).expect("the plural group extracts");
        assert_eq!(ex.nchan, 2);
        assert_eq!(ex.meas_list[1].source_index, 2);
    }

    #[test]
    fn per_channel_measurement_lists_extract() {
        let mut h = H5::new();
        let probe = probe_group(&mut h, &[760.0, 850.0], &[0.0, 0.0, 3.0, 0.0], &[1.0, 0.0, 4.0, 0.0]);
        let series = h.ds(1, 4, &[4, 2], &f32s(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
        let time = h.ds(1, 8, &[4], &f64s(&[0.0, 0.1, 0.2, 0.3]));
        let ch1 = scalar_ml(&mut h, 1, 1, 1, 1, "HbO");
        let ch2 = scalar_ml(&mut h, 2, 2, 1, 2, "HbR");
        let bytes = finish_nirs(
            h,
            probe,
            vec![
                ("dataTimeSeries", series),
                ("time", time),
                ("measurementList1", ch1),
                ("measurementList2", ch2),
            ],
        );
        let ex = parse_snirf(&bytes).expect("the per-channel groups extract");
        assert_eq!(ex.meas_list.len(), 2);
        assert_eq!(ex.meas_list[0].source_index, 1);
        assert_eq!(ex.meas_list[1].detector_index, 2);
        assert_eq!(ex.meas_list[1].data_type_label.as_deref(), Some("HbR"));
    }

    fn scalar_ml(h: &mut H5, src: i32, det: i32, wl: i32, dti: i32, label: &str) -> usize {
        let si = h.ds(0, 4, &[1], &i32s(&[src]));
        let di = h.ds(0, 4, &[1], &i32s(&[det]));
        let wli = h.ds(0, 4, &[1], &i32s(&[wl]));
        let dti = h.ds(0, 4, &[1], &i32s(&[dti]));
        let mut raw = label.as_bytes().to_vec();
        raw.resize(8, 0);
        let dtl = h.ds(3, 8, &[], &raw);
        h.group(&[
            ("sourceIndex", si),
            ("detectorIndex", di),
            ("wavelengthIndex", wli),
            ("dataTypeIndex", dti),
            ("dataTypeLabel", dtl),
        ])
    }

    #[test]
    fn transposed_series_reads_time_major() {
        let mut h = H5::new();
        let probe = probe_group(&mut h, &[760.0, 850.0], &[0.0, 0.0, 3.0, 0.0], &[1.0, 0.0, 4.0, 0.0]);
        let series = h.ds(1, 8, &[2, 4], &f64s(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
        let time = h.ds(1, 8, &[4], &f64s(&[0.0, 0.1, 0.2, 0.3]));
        let ml = array_ml_group(&mut h, &[1, 2], &[1, 2], &[1, 1]);
        let bytes = finish_nirs(h, probe, vec![("dataTimeSeries", series), ("time", time), ("measurementList1", ml)]);
        let ex = parse_snirf(&bytes).expect("the transposed series extracts");
        assert_eq!(ex.nchan, 2);
        assert_eq!(ex.pnts, 4);
        match ex.samples {
            Samples::Double(v) => assert_eq!(v, vec![1.0f64, 5.0, 2.0, 6.0, 3.0, 7.0, 4.0, 8.0]),
            _ => panic!("not double"),
        }
    }

    #[test]
    fn spacing_time_reads_the_pair() {
        let bytes = fixture(
            "measurementList1",
            Some(&[5.0, 0.02]),
            &[1, 2],
            &[760.0, 850.0],
        );
        let ex = parse_snirf(&bytes).expect("the spacing pair extracts");
        assert_eq!(ex.time, Some(TimeBase::Spacing { start: 5.0, step: 0.02 }));
        assert_eq!(ex.pnts, 4);
    }

    #[test]
    fn absent_time_skips() {
        let bytes = fixture("measurementList1", None, &[1, 2], &[760.0, 850.0]);
        let err = parse_snirf(&bytes).unwrap_err();
        assert!(err.contains("time absent"), "{err}");
    }

    #[test]
    fn absent_measurement_list_skips() {
        let bytes = fixture("", Some(&[0.0, 0.1, 0.2, 0.3]), &[1, 2], &[760.0, 850.0]);
        let err = parse_snirf(&bytes).unwrap_err();
        assert!(err.contains("measurementList absent"), "{err}");
    }

    #[test]
    fn ambiguous_square_dims_skip() {
        let mut h = H5::new();
        let probe = probe_group(&mut h, &[760.0, 850.0], &[0.0, 0.0, 3.0, 0.0], &[1.0, 0.0, 4.0, 0.0]);
        let series = h.ds(1, 4, &[2, 2], &f32s(&[1.0, 2.0, 3.0, 4.0]));
        let time = h.ds(1, 8, &[2], &f64s(&[0.0, 0.1]));
        let ml = array_ml_group(&mut h, &[1, 2], &[1, 2], &[1, 1]);
        let bytes = finish_nirs(h, probe, vec![("dataTimeSeries", series), ("time", time), ("measurementList1", ml)]);
        let err = parse_snirf(&bytes).unwrap_err();
        assert!(err.contains("ambiguous"), "{err}");
    }

    #[test]
    fn time_contradiction_skips() {
        let bytes = fixture("measurementList1", Some(&[0.0, 0.1, 0.2]), &[1, 2], &[760.0, 850.0]);
        let err = parse_snirf(&bytes).unwrap_err();
        assert!(err.contains("witnesses"), "{err}");
    }

    #[test]
    fn implausible_wavelength_skips() {
        let bytes = fixture("measurementList1", Some(&[0.0, 0.1, 0.2, 0.3]), &[1, 2], &[760.0, -5.0]);
        let err = parse_snirf(&bytes).unwrap_err();
        assert!(err.contains("wavelength carries"), "{err}");
    }

    #[test]
    fn source_index_beyond_geometry_skips() {
        let bytes = fixture("measurementList1", Some(&[0.0, 0.1, 0.2, 0.3]), &[3, 2], &[760.0, 850.0]);
        let err = parse_snirf(&bytes).unwrap_err();
        assert!(err.contains("exceeds"), "{err}");
    }

    #[test]
    fn nan_sample_skips() {
        let mut h = H5::new();
        let probe = probe_group(&mut h, &[760.0, 850.0], &[0.0, 0.0, 3.0, 0.0], &[1.0, 0.0, 4.0, 0.0]);
        let series = h.ds(1, 4, &[4, 2], &f32s(&[1.0, f32::NAN, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
        let time = h.ds(1, 8, &[4], &f64s(&[0.0, 0.1, 0.2, 0.3]));
        let ml = array_ml_group(&mut h, &[1, 2], &[1, 2], &[1, 1]);
        let bytes = finish_nirs(h, probe, vec![("dataTimeSeries", series), ("time", time), ("measurementList1", ml)]);
        let err = parse_snirf(&bytes).unwrap_err();
        assert!(err.contains("carries NaN"), "{err}");
    }

    #[test]
    fn junk_bytes_skip() {
        assert!(parse_snirf(b"").is_err());
        assert!(parse_snirf(b"not an hdf5 file").is_err());
        assert!(parse_snirf(&[0u8; 64]).is_err());
    }

    #[test]
    fn wire_roundtrip_is_bit_identical() {
        let bin = bin_with_samples(Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let bytes = write_bin(&bin).expect("the compact asset writes");
        let parsed = parse_bin(&bytes).expect("the compact asset parses");
        assert_eq!(parsed.nchan, bin.nchan);
        assert_eq!(parsed.pnts, bin.pnts);
        assert_eq!(parsed.nwavelengths, bin.nwavelengths);
        assert_eq!(parsed.nsources, bin.nsources);
        assert_eq!(parsed.ndetectors, bin.ndetectors);
        assert_eq!(parsed.sha256, bin.sha256);
        assert_eq!(parsed.snapshot_tag, bin.snapshot_tag);
        assert_eq!(parsed.hexsha, bin.hexsha);
        assert_eq!(parsed.origin_url, bin.origin_url);
        assert_eq!(parsed.labels, bin.labels);
        assert_eq!(parsed.wavelengths, bin.wavelengths);
        assert_eq!(parsed.source_geometry, bin.source_geometry);
        assert_eq!(parsed.detector_geometry, bin.detector_geometry);
        assert_eq!(parsed.meas_list, bin.meas_list);
        assert_eq!(parsed.time, bin.time);
        match (&bin.samples, &parsed.samples) {
            (Samples::Single(a), Samples::Single(b)) => {
                assert_eq!(a.len(), b.len());
                for (x, y) in a.iter().zip(b.iter()) {
                    assert_eq!(x.to_bits(), y.to_bits());
                }
            }
            _ => panic!("sample precision mismatch"),
        }
    }

    #[test]
    fn wire_roundtrip_double_spacing_pos3_without_measlist() {
        let mut bin = bin_with_samples(Samples::Double(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bin.time = Some(TimeBase::Spacing { start: 2.5, step: 0.05 });
        bin.source_geometry = Some(Geometry::Pos3(vec![(0.0, 0.0, 1.0), (3.0, 0.0, 1.0)]));
        bin.detector_geometry = Some(Geometry::Pos3(vec![(1.0, 0.0, 1.0), (4.0, 0.0, 1.0)]));
        bin.meas_list = None;
        let bytes = write_bin(&bin).expect("the compact asset writes");
        let parsed = parse_bin(&bytes).expect("the compact asset parses");
        assert_eq!(parsed.time, bin.time);
        assert_eq!(parsed.source_geometry, bin.source_geometry);
        assert_eq!(parsed.detector_geometry, bin.detector_geometry);
        assert_eq!(parsed.meas_list, None);
        match parsed.samples {
            Samples::Double(v) => assert_eq!(v, vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]),
            _ => panic!("not double"),
        }
    }

    #[test]
    fn wire_parse_refuses_foreign_and_truncated() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"SNIR").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&bin_with_samples(Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]))).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn wire_write_refuses_inconsistent_shapes() {
        let mut bin = bin_with_samples(Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bin.labels.pop();
        assert!(write_bin(&bin).is_none());
        let bin = bin_with_samples(Samples::Single(vec![1.0, 2.0, 3.0]));
        assert!(write_bin(&bin).is_none());
        let mut bin = bin_with_samples(Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bin.time = Some(TimeBase::Spacing { start: 0.0, step: 0.0 });
        assert!(write_bin(&bin).is_none());
        let mut bin = bin_with_samples(Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bin.wavelengths = vec![760.0, -850.0];
        assert!(write_bin(&bin).is_none());
    }
}


