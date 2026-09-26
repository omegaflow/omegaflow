pub const MAGIC: [u8; 4] = *b"G3D1";
pub const VERSION: u8 = 1;
pub const NLAT: usize = 181;
pub const NLON: usize = 361;
pub const NLAYERS: usize = 59;
pub const NODES: usize = NLAT * NLON;
pub const LAT0: f64 = -90.0;
pub const LAT_STEP: f64 = 1.0;
pub const LON0: f64 = -180.0;
pub const LON_STEP: f64 = 1.0;
pub const RECORD_BYTES: usize = 24;
pub const LAYER_TABLE_BYTES: usize = NLAYERS * 8;
pub const GEOMETRY_BYTES: usize = NODES * 8;
pub const HEADER_BYTES: usize = 48;
pub const RECORDS_OFFSET: usize = HEADER_BYTES + LAYER_TABLE_BYTES + GEOMETRY_BYTES;
pub const COORD_FILE: &str = "LLNL_G3D_JPS.Interpolated.Coordinates.txt";
pub const SURFACE_PREFIX: &str = "LLNL_G3D_JPS.Interpolated.Surface.";

pub const ZIP_URL: &str =
    "https://gs.llnl.gov/sites/gs/files/2021-09/llnl_g3d_jps.interpolated.zip";
pub const ZIP_SHA256: &str = "3bb043779efd1c4e4cb7e99547ad7c2fa12f7477922bb3cb22dd9ea67450edc8";

pub struct G3dModel {
    pub mean_depth: [f64; NLAYERS],
    pub geometry: Vec<[f32; 2]>,
    pub layers: Vec<Vec<[f32; 6]>>,
}

fn gate_value(v: f64, lo: f64, hi: f64) -> Option<f64> {
    if v.is_finite() && v > lo && v < hi {
        Some(v)
    } else {
        None
    }
}

fn gate_bounded(v: f64, lo: f64, hi: f64) -> Option<f64> {
    if v.is_finite() && v >= lo && v <= hi {
        Some(v)
    } else {
        None
    }
}

pub fn parse_coordinates(text: &str) -> Option<Vec<[f32; 2]>> {
    let mut out = Vec::with_capacity(NODES);
    for (row, line) in text.lines().enumerate() {
        if row >= NODES {
            return None;
        }
        let mut it = line.split_whitespace();
        let lat = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let lon = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let geolat = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let sealevel = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let want_lat = LAT0 + (row / NLON) as f64 * LAT_STEP;
        let want_lon = LON0 + (row % NLON) as f64 * LON_STEP;
        if (lat - want_lat).abs() > 1e-6 || (lon - want_lon).abs() > 1e-6 {
            return None;
        }
        let geolat = gate_bounded(geolat, -90.0, 90.0)?;
        let sealevel = gate_value(sealevel, 6000.0, 6500.0)?;
        out.push([geolat as f32, sealevel as f32]);
    }
    if out.len() != NODES {
        return None;
    }
    Some(out)
}

pub fn parse_surface(text: &str) -> Option<Vec<[f32; 6]>> {
    let mut out = Vec::with_capacity(NODES);
    for (row, line) in text.lines().enumerate() {
        if row >= NODES {
            return None;
        }
        let mut it = line.split_whitespace();
        let radius = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let depth = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let vp = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let dvp = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let vs = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let dvs = it.next().and_then(|t| t.parse::<f64>().ok())?;
        let radius = gate_value(radius, 0.0, 7000.0)?;
        let depth = gate_value(depth, -100.0, 6500.0)?;
        let vp = gate_value(vp, 0.0, 20.0)?;
        let dvp = gate_bounded(dvp, -1000.0, 1000.0)?;
        let vs = gate_bounded(vs, 0.0, 20.0)?;
        let dvs = gate_bounded(dvs, -1000.0, 1000.0)?;
        out.push([
            radius as f32,
            depth as f32,
            vp as f32,
            dvp as f32,
            vs as f32,
            dvs as f32,
        ]);
    }
    if out.len() != NODES {
        return None;
    }
    Some(out)
}

pub fn surface_index(name: &str) -> Option<usize> {
    let stem = name.strip_prefix(SURFACE_PREFIX)?;
    let num = stem.split('.').next()?;
    let idx = num.parse::<usize>().ok()?;
    if !(1..=NLAYERS).contains(&idx) {
        return None;
    }
    Some(idx)
}

fn le16(b: &[u8], off: usize) -> u16 {
    b[off] as u16 | ((b[off + 1] as u16) << 8)
}

fn le32(b: &[u8], off: usize) -> u32 {
    b[off] as u32
        | ((b[off + 1] as u32) << 8)
        | ((b[off + 2] as u32) << 16)
        | ((b[off + 3] as u32) << 24)
}

pub fn zip_entries(data: &[u8]) -> Vec<(String, u16, u32, u32)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 46 <= data.len() {
        if &data[i..i + 4] == b"PK\x01\x02" {
            let method = le16(data, i + 10);
            let comp_size = le32(data, i + 20);
            let name_len = le16(data, i + 28) as usize;
            let extra_len = le16(data, i + 30) as usize;
            let comment_len = le16(data, i + 32) as usize;
            let local_off = le32(data, i + 42);
            if i + 46 + name_len <= data.len() {
                let name = String::from_utf8_lossy(&data[i + 46..i + 46 + name_len]).into_owned();
                out.push((name, method, comp_size, local_off));
            }
            i += 46 + name_len + extra_len + comment_len;
            continue;
        }
        i += 1;
    }
    out
}

pub fn zip_entry_bytes(data: &[u8], name: &str) -> Option<Vec<u8>> {
    for (entry, method, comp_size, local_off) in zip_entries(data) {
        if entry != name {
            continue;
        }
        let local_off = local_off as usize;
        if local_off + 30 > data.len() {
            continue;
        }
        let name_len = le16(data, local_off + 26) as usize;
        let extra_len = le16(data, local_off + 28) as usize;
        let start = local_off + 30 + name_len + extra_len;
        let comp_size = comp_size as usize;
        if start + comp_size > data.len() {
            continue;
        }
        let body = &data[start..start + comp_size];
        return match method {
            0 => Some(body.to_vec()),
            8 => crate::inflate::inflate(body),
            _ => None,
        };
    }
    None
}

pub fn compile_zip(bytes: &[u8]) -> Option<G3dModel> {
    let entries = zip_entries(bytes);
    let mut by_index: Vec<Option<&str>> = vec![None; NLAYERS + 1];
    for (name, _, _, _) in &entries {
        if let Some(idx) = surface_index(name) {
            if by_index[idx].is_some() {
                return None;
            }
            by_index[idx] = Some(name);
        }
    }
    let coord_bytes = zip_entry_bytes(bytes, COORD_FILE)?;
    let coord_text = std::str::from_utf8(&coord_bytes).ok()?;
    let geometry = parse_coordinates(coord_text)?;
    let mut layers = Vec::with_capacity(NLAYERS);
    let mut mean_depth = [0.0f64; NLAYERS];
    for idx in 1..=NLAYERS {
        let name = by_index[idx]?;
        let raw = zip_entry_bytes(bytes, name)?;
        let text = std::str::from_utf8(&raw).ok()?;
        let rows = parse_surface(text)?;
        let mut sum = 0.0f64;
        for r in &rows {
            sum += r[1] as f64;
        }
        mean_depth[idx - 1] = sum / rows.len() as f64;
        layers.push(rows);
    }
    Some(G3dModel {
        mean_depth,
        geometry,
        layers,
    })
}

pub fn write_bin(model: &G3dModel) -> Vec<u8> {
    let mut buf = vec![0u8; RECORDS_OFFSET + NLAYERS * NODES * RECORD_BYTES];
    buf[0..4].copy_from_slice(&MAGIC);
    buf[4] = VERSION;
    buf[6..8].copy_from_slice(&(NLAT as u16).to_le_bytes());
    buf[8..10].copy_from_slice(&(NLON as u16).to_le_bytes());
    buf[10..12].copy_from_slice(&(NLAYERS as u16).to_le_bytes());
    buf[16..24].copy_from_slice(&LAT0.to_le_bytes());
    buf[24..32].copy_from_slice(&LAT_STEP.to_le_bytes());
    buf[32..40].copy_from_slice(&LON0.to_le_bytes());
    buf[40..48].copy_from_slice(&LON_STEP.to_le_bytes());
    let mut p = HEADER_BYTES;
    for &d in &model.mean_depth {
        buf[p..p + 8].copy_from_slice(&d.to_le_bytes());
        p += 8;
    }
    for g in &model.geometry {
        buf[p..p + 4].copy_from_slice(&g[0].to_le_bytes());
        buf[p + 4..p + 8].copy_from_slice(&g[1].to_le_bytes());
        p += 8;
    }
    p = RECORDS_OFFSET;
    for layer in &model.layers {
        for rec in layer {
            for (k, v) in rec.iter().enumerate() {
                buf[p + k * 4..p + k * 4 + 4].copy_from_slice(&v.to_le_bytes());
            }
            p += RECORD_BYTES;
        }
    }
    buf
}

fn le_f32(b: &[u8], off: usize) -> f32 {
    f32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn le_f64(b: &[u8], off: usize) -> f64 {
    f64::from_le_bytes([
        b[off],
        b[off + 1],
        b[off + 2],
        b[off + 3],
        b[off + 4],
        b[off + 5],
        b[off + 6],
        b[off + 7],
    ])
}

fn le_u16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

pub fn read_bin(bytes: &[u8]) -> Option<G3dModel> {
    if bytes.len() != RECORDS_OFFSET + NLAYERS * NODES * RECORD_BYTES {
        return None;
    }
    if bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    if le_u16(bytes, 6) as usize != NLAT
        || le_u16(bytes, 8) as usize != NLON
        || le_u16(bytes, 10) as usize != NLAYERS
    {
        return None;
    }
    let lat0 = le_f64(bytes, 16);
    let lat_step = le_f64(bytes, 24);
    let lon0 = le_f64(bytes, 32);
    let lon_step = le_f64(bytes, 40);
    if !lat0.is_finite()
        || !lat_step.is_finite()
        || !lon0.is_finite()
        || !lon_step.is_finite()
        || lat_step <= 0.0
        || lon_step <= 0.0
    {
        return None;
    }
    if (lat0 + (NLAT as f64 - 1.0) * lat_step - 90.0).abs() > 1e-9
        || (lon0 + (NLON as f64 - 1.0) * lon_step - 180.0).abs() > 1e-9
    {
        return None;
    }
    let mut p = HEADER_BYTES;
    let mut mean_depth = [0.0f64; NLAYERS];
    for d in mean_depth.iter_mut() {
        let v = le_f64(bytes, p);
        p += 8;
        if !v.is_finite() || v <= -100.0 || v >= 6500.0 {
            return None;
        }
        *d = v;
    }
    let mut geometry = Vec::with_capacity(NODES);
    for _ in 0..NODES {
        let geolat = le_f32(bytes, p);
        let sealevel = le_f32(bytes, p + 4);
        p += 8;
        if !geolat.is_finite()
            || !sealevel.is_finite()
            || !(-90.0..=90.0).contains(&geolat)
            || !(6000.0..6500.0).contains(&sealevel)
        {
            return None;
        }
        geometry.push([geolat, sealevel]);
    }
    let mut layers = Vec::with_capacity(NLAYERS);
    for _ in 0..NLAYERS {
        let mut rows = Vec::with_capacity(NODES);
        for _ in 0..NODES {
            let mut rec = [0.0f32; 6];
            for (k, slot) in rec.iter_mut().enumerate() {
                let v = le_f32(bytes, p + k * 4);
                *slot = v;
            }
            p += RECORD_BYTES;
            let [radius, depth, vp, dvp, vs, dvs] = rec;
            if !radius.is_finite()
                || !depth.is_finite()
                || !vp.is_finite()
                || !dvp.is_finite()
                || !vs.is_finite()
                || !dvs.is_finite()
                || radius <= 0.0
                || radius >= 7000.0
                || depth <= -100.0
                || depth >= 6500.0
                || vp <= 0.0
                || vp >= 20.0
                || vs < 0.0
                || vs >= 20.0
                || dvp.abs() > 1000.0
                || dvs.abs() > 1000.0
            {
                return None;
            }
            rows.push(rec);
        }
        layers.push(rows);
    }
    Some(G3dModel {
        mean_depth,
        geometry,
        layers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn surface_text() -> String {
        let mut out = String::new();
        for i in 0..NODES {
            let depth = 68.2 - (i % 100) as f64 * 0.001;
            let line = format!(
                "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4}\n",
                6300.0 - (i % 100) as f64 * 0.001,
                depth,
                8.1556,
                0.8595,
                4.5880,
                1.5098
            );
            out.push_str(&line);
        }
        out
    }

    fn coordinates_text() -> String {
        let mut out = String::new();
        for i in 0..NODES {
            let lat = LAT0 + (i / NLON) as f64;
            let lon = LON0 + (i % NLON) as f64;
            let line = format!(
                "{lat:.4} {lon:.4} {:.4} {:.4}\n",
                lat - lat.abs() * 0.002,
                6356.72 + (i % 100) as f64 * 0.0001
            );
            out.push_str(&line);
        }
        out
    }

    fn test_model() -> G3dModel {
        let geometry = parse_coordinates(&coordinates_text()).unwrap();
        let layers: Vec<Vec<[f32; 6]>> = (0..NLAYERS)
            .map(|_| parse_surface(&surface_text()).unwrap())
            .collect();
        let mut mean_depth = [0.0f64; NLAYERS];
        for (k, layer) in layers.iter().enumerate() {
            let sum: f64 = layer.iter().map(|r| r[1] as f64).sum();
            mean_depth[k] = sum / layer.len() as f64;
        }
        G3dModel {
            mean_depth,
            geometry,
            layers,
        }
    }

    #[test]
    fn coordinates_parse_the_regular_grid() {
        let coords = parse_coordinates(&coordinates_text()).unwrap();
        assert_eq!(coords.len(), NODES);
        assert_eq!(coords[0][1], 6356.72);
        assert_eq!(coords[NODES - 1][0], (90.0 - 90.0 * 0.002) as f32);
    }

    #[test]
    fn coordinates_refuse_an_irregular_row() {
        let mut text = coordinates_text();
        text.replace_range(0..3, "91.");
        assert!(parse_coordinates(&text).is_none());
    }

    #[test]
    fn coordinates_refuse_a_short_file() {
        let mut lines: Vec<&str> = coordinates_text().lines().collect();
        lines.pop();
        let text = lines.join("\n");
        assert!(parse_coordinates(&text).is_none());
    }

    #[test]
    fn surface_parses_all_six_columns() {
        let rows = parse_surface(&surface_text()).unwrap();
        assert_eq!(rows.len(), NODES);
        assert_eq!(rows[0][2], 8.1556f32);
        assert_eq!(rows[0][4], 4.5880f32);
    }

    #[test]
    fn surface_refuses_an_unphysical_value() {
        let mut text = surface_text();
        text.replace_range(0..7, "99999.0");
        assert!(parse_surface(&text).is_none());
    }

    #[test]
    fn surface_refuses_a_short_row() {
        let mut text = surface_text();
        text.replace_range(0..16, "6300.0 68.2\n");
        assert!(parse_surface(&text).is_none());
    }

    #[test]
    fn surface_index_reads_the_layer_number() {
        assert_eq!(
            surface_index("LLNL_G3D_JPS.Interpolated.Surface.01.Crust_1_Top_of_water.txt"),
            Some(1)
        );
        assert_eq!(
            surface_index(
                "LLNL_G3D_JPS.Interpolated.Surface.59.Lower_Mantle_59_2891.0km_-_CMB.txt"
            ),
            Some(59)
        );
        assert_eq!(
            surface_index("LLNL_G3D_JPS.Interpolated.Coordinates.txt"),
            None
        );
        assert_eq!(
            surface_index("LLNL_G3D_JPS.Interpolated.Surface.60.x.txt"),
            None
        );
    }

    #[test]
    fn binary_roundtrips() {
        let model = test_model();
        let bin = write_bin(&model);
        assert_eq!(bin.len(), RECORDS_OFFSET + NLAYERS * NODES * RECORD_BYTES);
        let back = read_bin(&bin).unwrap();
        assert_eq!(back.mean_depth, model.mean_depth);
        assert_eq!(back.geometry, model.geometry);
        assert_eq!(back.layers, model.layers);
    }

    #[test]
    fn binary_refuses_a_truncated_asset() {
        let bin = write_bin(&test_model());
        assert!(read_bin(&bin[..bin.len() - 4]).is_none());
    }

    #[test]
    fn binary_refuses_a_foreign_magic() {
        let mut bin = write_bin(&test_model());
        bin[0] = b'X';
        assert!(read_bin(&bin).is_none());
    }

    #[test]
    fn binary_refuses_a_corrupted_grid_axis() {
        let mut bin = write_bin(&test_model());
        bin[16..24].copy_from_slice(&(-89.0f64).to_le_bytes());
        assert!(read_bin(&bin).is_none());
    }
}
