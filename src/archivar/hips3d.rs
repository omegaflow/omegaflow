use super::fits::{FitsHeader, FitsTable};

pub const MAGIC: [u8; 4] = *b"H3D1";
pub const VERSION: u8 = 1;
pub const HEADER_BYTES: usize = 46;
pub const REC_BYTES: usize = 20;

pub const FMOC_MAX_ORDER: u32 = 51;
pub const SPACE_MAX_ORDER: u32 = 29;
pub const MAX_MANIFEST_TILES: usize = 4_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hips3DTile {
    pub order: u32,
    pub npix: u32,
    pub dir: u32,
    pub order_axis2: u32,
    pub npix_axis2: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hips3DManifest {
    pub order: u32,
    pub order_axis2: u32,
    pub tile_width: u32,
    pub tile_depth: u32,
    pub em_min: f64,
    pub em_max: f64,
    pub base: String,
    pub tiles: Vec<Hips3DTile>,
}

pub const DIR_STRIDE: u32 = 10_000;
pub const AXIS2_DIR_STRIDE: u32 = 10;

pub fn dir_of(npix: u32) -> u32 {
    (npix / DIR_STRIDE) * DIR_STRIDE
}

pub fn axis2_dir_of(npix_axis2: u32) -> u32 {
    (npix_axis2 / AXIS2_DIR_STRIDE) * AXIS2_DIR_STRIDE
}

pub fn spatial_npix_bound(order: u32) -> Option<u32> {
    4u32.checked_pow(order)?.checked_mul(12)
}

pub fn axis2_npix_bound(order: u32) -> Option<u64> {
    1u64.checked_shl(order.checked_add(1)?)
}

pub fn tile_path(
    order: u32,
    npix: u32,
    order_axis2: u32,
    npix_axis2: u32,
    ext: &str,
) -> Option<String> {
    if npix >= spatial_npix_bound(order)? {
        return None;
    }
    if (npix_axis2 as u64) >= axis2_npix_bound(order_axis2)? {
        return None;
    }
    Some(format!(
        "Norder{order}_{order_axis2}/Dir{}_{}/Npix{npix}_{npix_axis2}.{ext}",
        dir_of(npix),
        axis2_dir_of(npix_axis2)
    ))
}

pub fn tile_url(
    base: &str,
    order: u32,
    npix: u32,
    order_axis2: u32,
    npix_axis2: u32,
    ext: &str,
) -> Option<String> {
    Some(format!(
        "{}/{}",
        base.trim_end_matches('/'),
        tile_path(order, npix, order_axis2, npix_axis2, ext)?
    ))
}

fn degrade_range(start: u64, end: u64, shift: u32, bound: u64) -> Option<(u64, u64)> {
    if end <= start {
        return None;
    }
    let lo = start >> shift;
    let hi = (end - 1) >> shift;
    if lo >= bound {
        return None;
    }
    Some((lo, hi.min(bound - 1)))
}

pub fn decode_sfmoc_ranges(
    values: &[u64],
    freq_order: u32,
    space_order: u32,
) -> Option<Vec<Hips3DTile>> {
    if freq_order > FMOC_MAX_ORDER || space_order > SPACE_MAX_ORDER {
        return None;
    }
    let freq_bound = axis2_npix_bound(freq_order)?;
    let space_bound = spatial_npix_bound(space_order)? as u64;
    let freq_shift = FMOC_MAX_ORDER - freq_order;
    let space_shift = (SPACE_MAX_ORDER - space_order) * 2;

    let mut tiles = Vec::new();
    let mut i = 0usize;
    let mut saw_block = false;
    while i < values.len() {
        let mut freq = Vec::new();
        while i < values.len() && (values[i] & (1u64 << 63)) != 0 {
            freq.push(values[i] & !(1u64 << 63));
            i += 1;
        }
        let mut space = Vec::new();
        while i < values.len() && (values[i] & (1u64 << 63)) == 0 {
            space.push(values[i]);
            i += 1;
        }
        if freq.is_empty() {
            return None;
        }
        saw_block = true;
        if !freq.len().is_multiple_of(2) || !space.len().is_multiple_of(2) {
            return None;
        }
        for f in freq.chunks_exact(2) {
            let (f0, f1) = degrade_range(f[0], f[1], freq_shift, freq_bound)?;
            for s in space.chunks_exact(2) {
                let (s0, s1) = degrade_range(s[0], s[1], space_shift, space_bound)?;
                for npix in s0..=s1 {
                    for npix_axis2 in f0..=f1 {
                        if tiles.len() >= MAX_MANIFEST_TILES {
                            return None;
                        }
                        tiles.push(Hips3DTile {
                            order: space_order,
                            npix: npix as u32,
                            dir: dir_of(npix as u32),
                            order_axis2: freq_order,
                            npix_axis2: npix_axis2 as u32,
                        });
                    }
                }
            }
        }
    }
    if !saw_block {
        return None;
    }
    tiles.sort_unstable_by_key(|t| (t.npix, t.npix_axis2));
    tiles.dedup_by_key(|t| (t.npix, t.npix_axis2));
    Some(tiles)
}

pub fn parse_sfmoc_tiles(
    moc_bytes: &[u8],
    freq_order: u32,
    space_order: u32,
) -> Option<Vec<Hips3DTile>> {
    let (_, next) = FitsHeader::parse(moc_bytes, 0)?;
    let (table, _) = FitsTable::parse(moc_bytes, next)?;
    let column = table.columns.first()?;
    let mut values = Vec::with_capacity(table.n_rows);
    for row in 0..table.n_rows {
        values.push(table.cell_i64(moc_bytes, row, column)? as u64);
    }
    decode_sfmoc_ranges(&values, freq_order, space_order)
}

fn tile_plausible(tile: &Hips3DTile, order: u32, order_axis2: u32) -> bool {
    tile.order == order
        && tile.order_axis2 == order_axis2
        && spatial_npix_bound(tile.order).is_some_and(|b| tile.npix < b)
        && axis2_npix_bound(tile.order_axis2).is_some_and(|b| (tile.npix_axis2 as u64) < b)
        && tile.dir == dir_of(tile.npix)
}

pub fn write_bin(manifest: &Hips3DManifest) -> Option<Vec<u8>> {
    if manifest.base.is_empty()
        || manifest.base.len() > u16::MAX as usize
        || manifest.tiles.len() > u32::MAX as usize
        || !manifest.em_min.is_finite()
        || !manifest.em_max.is_finite()
        || manifest.em_min > manifest.em_max
        || manifest.tile_width == 0
        || manifest.tile_depth == 0
    {
        return None;
    }
    for tile in &manifest.tiles {
        if !tile_plausible(tile, manifest.order, manifest.order_axis2) {
            return None;
        }
    }
    let mut out =
        Vec::with_capacity(HEADER_BYTES + manifest.base.len() + manifest.tiles.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.push(0);
    out.extend_from_slice(&[0u8; 2]);
    out.extend_from_slice(&manifest.order.to_le_bytes());
    out.extend_from_slice(&manifest.order_axis2.to_le_bytes());
    out.extend_from_slice(&manifest.tile_width.to_le_bytes());
    out.extend_from_slice(&manifest.tile_depth.to_le_bytes());
    out.extend_from_slice(&manifest.em_min.to_le_bytes());
    out.extend_from_slice(&manifest.em_max.to_le_bytes());
    out.extend_from_slice(&(manifest.tiles.len() as u32).to_le_bytes());
    out.extend_from_slice(&(manifest.base.len() as u16).to_le_bytes());
    out.extend_from_slice(manifest.base.as_bytes());
    for tile in &manifest.tiles {
        out.extend_from_slice(&tile.order.to_le_bytes());
        out.extend_from_slice(&tile.npix.to_le_bytes());
        out.extend_from_slice(&tile.dir.to_le_bytes());
        out.extend_from_slice(&tile.order_axis2.to_le_bytes());
        out.extend_from_slice(&tile.npix_axis2.to_le_bytes());
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Hips3DManifest> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC || bytes[4] != VERSION || bytes[5] != 0 {
        return None;
    }
    let order = u32::from_le_bytes(bytes[8..12].try_into().ok()?);
    let order_axis2 = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let tile_width = u32::from_le_bytes(bytes[16..20].try_into().ok()?);
    let tile_depth = u32::from_le_bytes(bytes[20..24].try_into().ok()?);
    let em_min = f64::from_le_bytes(bytes[24..32].try_into().ok()?);
    let em_max = f64::from_le_bytes(bytes[32..40].try_into().ok()?);
    let count = u32::from_le_bytes(bytes[40..44].try_into().ok()?) as usize;
    let base_len = u16::from_le_bytes(bytes[44..46].try_into().ok()?) as usize;
    let base_start = 46usize;
    let base_end = base_start.checked_add(base_len)?;
    let base = String::from_utf8(bytes.get(base_start..base_end)?.to_vec()).ok()?;
    let records_end = base_end.checked_add(count.checked_mul(REC_BYTES)?)?;
    if base.is_empty() || records_end != bytes.len() {
        return None;
    }
    if tile_width == 0
        || tile_depth == 0
        || !em_min.is_finite()
        || !em_max.is_finite()
        || em_min > em_max
    {
        return None;
    }
    let mut tiles = Vec::with_capacity(count);
    let mut off = base_end;
    for _ in 0..count {
        let order = u32::from_le_bytes(bytes[off..off + 4].try_into().ok()?);
        let npix = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().ok()?);
        let dir = u32::from_le_bytes(bytes[off + 8..off + 12].try_into().ok()?);
        let order_axis2 = u32::from_le_bytes(bytes[off + 12..off + 16].try_into().ok()?);
        let npix_axis2 = u32::from_le_bytes(bytes[off + 16..off + 20].try_into().ok()?);
        off += REC_BYTES;
        let tile = Hips3DTile {
            order,
            npix,
            dir,
            order_axis2,
            npix_axis2,
        };
        if !tile_plausible(&tile, order, order_axis2) {
            return None;
        }
        tiles.push(tile);
    }
    if tiles
        .iter()
        .any(|t| t.order != order || t.order_axis2 != order_axis2)
    {
        return None;
    }
    Some(Hips3DManifest {
        order,
        order_axis2,
        tile_width,
        tile_depth,
        em_min,
        em_max,
        base,
        tiles,
    })
}

pub const COMP_TILE_NPIX: u32 = 0;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_TILE_NPIX => Some("hips3d_tile_npix"),
        _ => None,
    }
}

pub fn parse_asset(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let manifest = parse_bin(bytes)?;
    Some(
        manifest
            .tiles
            .iter()
            .map(|t| (t.npix_axis2 as f64, t.npix as f64, COMP_TILE_NPIX))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> Hips3DManifest {
        Hips3DManifest {
            order: 7,
            order_axis2: 18,
            tile_width: 256,
            tile_depth: 16,
            em_min: 0.20825830009069737,
            em_max: 0.23143256178622565,
            base: "https://alasky.cds.unistra.fr/HIPS3D/ASKAP/CDS_C_ASKAP_WALLABY-Pilot-DR2"
                .to_string(),
            tiles: vec![
                Hips3DTile {
                    order: 7,
                    npix: 99701,
                    dir: 90000,
                    order_axis2: 18,
                    npix_axis2: 253834,
                },
                Hips3DTile {
                    order: 7,
                    npix: 99701,
                    dir: 90000,
                    order_axis2: 18,
                    npix_axis2: 253835,
                },
            ],
        }
    }

    #[test]
    fn addresses_follow_the_hips3d_path() {
        let url = tile_url(
            "https://alasky.cds.unistra.fr/HIPS3D/ASKAP/CDS_C_ASKAP_WALLABY-Pilot-DR2",
            7,
            99701,
            18,
            253834,
            "fits",
        )
        .unwrap();
        assert_eq!(
            url,
            "https://alasky.cds.unistra.fr/HIPS3D/ASKAP/CDS_C_ASKAP_WALLABY-Pilot-DR2/Norder7_18/Dir90000_253830/Npix99701_253834.fits"
        );
        assert_eq!(dir_of(99701), 90000);
        assert_eq!(axis2_dir_of(253834), 253830);
        assert!(spatial_npix_bound(7) == Some(196_608));
        assert!(axis2_npix_bound(18) == Some(1 << 19));
        assert!(tile_url("x", 7, 196_608, 18, 0, "fits").is_none());
        assert!(tile_url("x", 7, 0, 18, 1 << 19, "fits").is_none());
        assert!(spatial_npix_bound(40).is_none());
    }

    #[test]
    fn sfmoc_ranges_degrade_to_leaf_tiles() {
        let f0 = (253_834u64 << 33) | (1u64 << 63);
        let f1 = (253_836u64 << 33) | (1u64 << 63);
        let s0 = 99_701u64 << 44;
        let s1 = 99_703u64 << 44;
        let tiles = decode_sfmoc_ranges(&[f0, f1, s0, s1], 18, 7).unwrap();
        assert_eq!(tiles.len(), 4);
        assert_eq!(
            tiles
                .iter()
                .map(|t| (t.npix, t.npix_axis2))
                .collect::<Vec<_>>(),
            vec![
                (99_701, 253_834),
                (99_701, 253_835),
                (99_702, 253_834),
                (99_702, 253_835)
            ]
        );
    }

    #[test]
    fn manifest_roundtrips() {
        let m = manifest();
        let bytes = write_bin(&m).unwrap();
        assert_eq!(parse_bin(&bytes), Some(m.clone()));
        assert_eq!(parse_bin(&bytes[..bytes.len() - 1]), None);
        assert_eq!(parse_asset(&bytes).unwrap().len(), 2);
        assert_eq!(component_name(COMP_TILE_NPIX), Some("hips3d_tile_npix"));
        assert_eq!(component_name(1), None);
    }

    #[test]
    fn manifest_refuses_foreign_or_inconsistent_records() {
        assert!(parse_bin(b"H3D1").is_none());
        let mut bytes = write_bin(&manifest()).unwrap();
        bytes[4] = 2;
        assert!(parse_bin(&bytes).is_none());
        let mut m = manifest();
        m.tiles[0].dir = 0;
        assert!(write_bin(&m).is_none());
        let mut m = manifest();
        m.tiles[0].npix = 196_608;
        assert!(write_bin(&m).is_none());
        let mut m = manifest();
        m.order_axis2 = 63;
        assert!(write_bin(&m).is_none());
    }
}
