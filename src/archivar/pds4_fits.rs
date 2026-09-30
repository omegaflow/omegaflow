use crate::archivar::fits::{FitsColumn, FitsHeader, FitsImage, FitsTable};

#[derive(Clone, Debug, PartialEq)]
pub struct FitsRaster {
    pub bands: usize,
    pub lines: usize,
    pub samples: usize,
    pub band_names: Vec<String>,
    pub values: Vec<Option<f64>>,
}

fn band_names_of(header: &FitsHeader, bands: usize) -> Vec<String> {
    let bunit = header.str_unescaped("BUNIT").map(|s| s.trim().to_string());
    if bands == 1 {
        match bunit {
            Some(name) if !name.is_empty() => vec![name],
            _ => vec!["value".to_string()],
        }
    } else {
        (0..bands).map(|b| format!("plane_{b}")).collect()
    }
}

fn raster_from_image(bytes: &[u8], img: &FitsImage, header: &FitsHeader) -> Option<FitsRaster> {
    let bands = img.dims[2];
    let lines = img.dims[1];
    let samples = img.dims[0];
    if bands == 0 || lines == 0 || samples == 0 {
        return None;
    }
    let band_names = band_names_of(header, bands);
    let total = bands.checked_mul(lines)?.checked_mul(samples)?;
    let mut values = Vec::with_capacity(total);
    for b in 0..bands {
        for y in 0..lines {
            for x in 0..samples {
                let v = img.value_f64(bytes, [x, y, b]).filter(|v| v.is_finite());
                values.push(v);
            }
        }
    }
    Some(FitsRaster {
        bands,
        lines,
        samples,
        band_names,
        values,
    })
}

fn next_hdu(buf: &[u8], off: usize, header: &FitsHeader) -> Option<usize> {
    let next = match header.value("XTENSION") {
        Some("'BINTABLE'") => FitsTable::parse(buf, off)?.1,
        _ => FitsImage::parse(buf, off)?.1,
    };
    if next <= off || next >= buf.len() {
        return None;
    }
    Some(next)
}

pub fn parse_image(bytes: &[u8]) -> Option<FitsRaster> {
    let mut off = 0usize;
    for _ in 0..8 {
        let (header, _) = FitsHeader::parse(bytes, off)?;
        match header.value("XTENSION") {
            None | Some("'IMAGE'") => {
                let (img, next) = FitsImage::parse(bytes, off)?;
                if img.dims.iter().any(|d| *d > 0) {
                    return raster_from_image(bytes, &img, &header);
                }
                if next <= off || next >= bytes.len() {
                    return None;
                }
                off = next;
            }
            Some("'BINTABLE'") => {
                off = next_hdu(bytes, off, &header)?;
            }
            _ => return None,
        }
    }
    None
}

pub fn band_means(raster: &FitsRaster) -> Option<Vec<(f64, f64, u32)>> {
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

#[derive(Clone, Debug, PartialEq)]
pub struct FitsTableSeries {
    pub names: Vec<String>,
    pub units: Vec<Option<String>>,
    pub rows: Vec<(f64, f64, u32)>,
}

fn channel_ordinal(name: &str) -> Option<usize> {
    let trimmed = name.trim();
    let rest = trimmed
        .strip_prefix('t')
        .or_else(|| trimmed.strip_prefix('T'))?;
    if rest.is_empty() || !rest.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    rest.parse::<usize>().ok()
}

fn bintable_series_of(
    buf: &[u8],
    table: &FitsTable,
    header: &FitsHeader,
) -> Option<FitsTableSeries> {
    let axis = table
        .columns
        .iter()
        .find(|c| c.name.trim().eq_ignore_ascii_case("et"))?;
    let mut channels: Vec<&FitsColumn> = table
        .columns
        .iter()
        .filter(|c| channel_ordinal(&c.name).is_some())
        .collect();
    channels.sort_by_key(|c| channel_ordinal(&c.name));
    if channels.is_empty() {
        return None;
    }
    let bunit = header
        .str_unescaped("BUNIT")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let names: Vec<String> = channels.iter().map(|c| c.name.trim().to_string()).collect();
    let units: Vec<Option<String>> = channels
        .iter()
        .map(|c| c.unit.clone().or_else(|| bunit.clone()))
        .collect();
    let mut rows = Vec::new();
    for r in 0..table.n_rows {
        let epoch = match table.cell_f64(buf, r, axis) {
            Some(t) if t.is_finite() => t,
            _ => continue,
        };
        for (comp, c) in channels.iter().enumerate() {
            let v = match table.cell_f64(buf, r, c) {
                Some(x) if x.is_finite() => x,
                _ => continue,
            };
            rows.push((epoch, v, comp as u32));
        }
    }
    if rows.is_empty() {
        return None;
    }
    Some(FitsTableSeries { names, units, rows })
}

pub fn parse_bintable_series(bytes: &[u8]) -> Option<FitsTableSeries> {
    let mut off = 0usize;
    for _ in 0..8 {
        let (header, _) = FitsHeader::parse(bytes, off)?;
        match header.value("XTENSION") {
            Some("'BINTABLE'") => {
                let (table, next) = FitsTable::parse(bytes, off)?;
                if let Some(series) = bintable_series_of(bytes, &table, &header) {
                    return Some(series);
                }
                if next <= off || next >= bytes.len() {
                    return None;
                }
                off = next;
            }
            None | Some("'IMAGE'") => {
                let (_, next) = FitsImage::parse(bytes, off)?;
                if next <= off || next >= bytes.len() {
                    return None;
                }
                off = next;
            }
            _ => return None,
        }
    }
    None
}

type NamedSeries = (Vec<String>, Vec<(f64, f64, u32)>);

pub fn parse_named_series(bytes: &[u8]) -> Option<NamedSeries> {
    if let Some(raster) = parse_image(bytes) {
        let names = raster.band_names.clone();
        let rows = band_means(&raster)?;
        return Some((names, rows));
    }
    let series = parse_bintable_series(bytes)?;
    Some((series.names, series.rows))
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    parse_named_series(bytes).map(|(_, rows)| rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pad_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        card
    }

    fn image(cards: &[(&str, &str)], raw: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        for (k, v) in cards {
            header.extend_from_slice(&pad_card(k, v));
        }
        header.extend_from_slice(&pad_card("END", ""));
        while !header.len().is_multiple_of(2880) {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);
        buf.extend_from_slice(raw);
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }
        buf
    }

    fn f32_pixels(vals: &[f32]) -> Vec<u8> {
        let mut raw = Vec::with_capacity(vals.len() * 4);
        for v in vals {
            raw.extend_from_slice(&v.to_be_bytes());
        }
        raw
    }

    fn bintable(cards: &[(&str, &str)], raw: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let mut primary: Vec<u8> = Vec::new();
        primary.extend_from_slice(&pad_card("SIMPLE", "T"));
        primary.extend_from_slice(&pad_card("BITPIX", "8"));
        primary.extend_from_slice(&pad_card("NAXIS", "0"));
        primary.extend_from_slice(&pad_card("EXTEND", "T"));
        primary.extend_from_slice(&pad_card("END", ""));
        while !primary.len().is_multiple_of(2880) {
            primary.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&primary);

        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("XTENSION", "'BINTABLE'"));
        for (k, v) in cards {
            header.extend_from_slice(&pad_card(k, v));
        }
        header.extend_from_slice(&pad_card("END", ""));
        while !header.len().is_multiple_of(2880) {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);
        buf.extend_from_slice(raw);
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }
        buf
    }

    fn mrm_table() -> Vec<u8> {
        let cards = [
            ("BITPIX", "8"),
            ("NAXIS", "2"),
            ("NAXIS1", "20"),
            ("NAXIS2", "2"),
            ("PCOUNT", "0"),
            ("GCOUNT", "1"),
            ("TFIELDS", "5"),
            ("TTYPE1", "'orbit'"),
            ("TFORM1", "I"),
            ("TZERO1", "32768"),
            ("TTYPE2", "'utc'"),
            ("TFORM2", "2A"),
            ("TTYPE3", "'et'"),
            ("TFORM3", "D"),
            ("TUNIT3", "'s'"),
            ("TTYPE4", "'t1'"),
            ("TFORM4", "E"),
            ("TUNIT4", "'K'"),
            ("TTYPE5", "'t2'"),
            ("TFORM5", "E"),
        ];
        let mut raw = Vec::new();
        raw.extend_from_slice(&1i16.to_be_bytes());
        raw.extend_from_slice(b"ab");
        raw.extend_from_slice(&1.5f64.to_be_bytes());
        raw.extend_from_slice(&10.0f32.to_be_bytes());
        raw.extend_from_slice(&20.0f32.to_be_bytes());
        raw.extend_from_slice(&2i16.to_be_bytes());
        raw.extend_from_slice(b"cd");
        raw.extend_from_slice(&2.5f64.to_be_bytes());
        raw.extend_from_slice(&30.0f32.to_be_bytes());
        raw.extend_from_slice(&40.0f32.to_be_bytes());
        bintable(&cards, &raw)
    }

    #[test]
    fn single_band_2d_map_reads_band_mean() {
        let buf = image(
            &[
                ("BITPIX", "-32"),
                ("NAXIS", "2"),
                ("NAXIS1", "2"),
                ("NAXIS2", "2"),
                ("BUNIT", "'K'"),
            ],
            &f32_pixels(&[1.0, 2.0, 3.0, 4.0]),
        );
        let raster = parse_image(&buf).expect("raster parses");
        assert_eq!(raster.bands, 1);
        assert_eq!(raster.lines, 2);
        assert_eq!(raster.samples, 2);
        assert_eq!(raster.band_names, vec!["K"]);
        assert_eq!(
            raster.values,
            vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]
        );
        let series = band_means(&raster).expect("band means");
        assert_eq!(series, vec![(0.0, 2.5, 0)]);
    }

    #[test]
    fn three_band_cube_reads_per_plane_means() {
        let buf = image(
            &[
                ("BITPIX", "-32"),
                ("NAXIS", "3"),
                ("NAXIS1", "2"),
                ("NAXIS2", "2"),
                ("NAXIS3", "2"),
            ],
            &f32_pixels(&[1.0, 2.0, 3.0, 4.0, 10.0, 20.0, 30.0, 40.0]),
        );
        let raster = parse_image(&buf).expect("raster parses");
        assert_eq!(raster.bands, 2);
        assert_eq!(raster.band_names, vec!["plane_0", "plane_1"]);
        let series = band_means(&raster).expect("band means");
        assert_eq!(series, vec![(0.0, 2.5, 0), (1.0, 25.0, 1)]);
    }

    #[test]
    fn nan_pixels_are_absent_not_zero() {
        let buf = image(
            &[
                ("BITPIX", "-32"),
                ("NAXIS", "2"),
                ("NAXIS1", "2"),
                ("NAXIS2", "1"),
            ],
            &f32_pixels(&[1.0, f32::NAN]),
        );
        let raster = parse_image(&buf).expect("raster parses");
        assert_eq!(raster.values, vec![Some(1.0), None]);
        let series = band_means(&raster).expect("band means");
        assert_eq!(series, vec![(0.0, 1.0, 0)]);
    }

    #[test]
    fn int16_with_bscale_bzero_reads_scaled_mean() {
        let mut raw = Vec::new();
        for v in [0i16, 2i16] {
            raw.extend_from_slice(&v.to_be_bytes());
        }
        let buf = image(
            &[
                ("BITPIX", "16"),
                ("NAXIS", "2"),
                ("NAXIS1", "2"),
                ("NAXIS2", "1"),
                ("BSCALE", "0.5"),
                ("BZERO", "100"),
            ],
            &raw,
        );
        let raster = parse_image(&buf).expect("raster parses");
        let series = band_means(&raster).expect("band means");
        assert_eq!(series, vec![(0.0, 100.5, 0)]);
    }

    #[test]
    fn skips_empty_primary_and_reads_image_extension() {
        let mut buf: Vec<u8> = Vec::new();
        let mut primary: Vec<u8> = Vec::new();
        primary.extend_from_slice(&pad_card("SIMPLE", "T"));
        primary.extend_from_slice(&pad_card("BITPIX", "8"));
        primary.extend_from_slice(&pad_card("NAXIS", "0"));
        primary.extend_from_slice(&pad_card("END", ""));
        while !primary.len().is_multiple_of(2880) {
            primary.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&primary);

        let mut ext: Vec<u8> = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'IMAGE'"));
        ext.extend_from_slice(&pad_card("BITPIX", "-32"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "2"));
        ext.extend_from_slice(&pad_card("NAXIS2", "1"));
        ext.extend_from_slice(&pad_card("END", ""));
        while !ext.len().is_multiple_of(2880) {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);
        buf.extend_from_slice(&f32_pixels(&[5.0, 7.0]));
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }

        let raster = parse_image(&buf).expect("raster parses");
        let series = band_means(&raster).expect("band means");
        assert_eq!(series, vec![(0.0, 6.0, 0)]);
    }

    #[test]
    fn bintable_reads_et_axis_and_t_channels() {
        let buf = mrm_table();
        let series = parse_series(&buf).expect("bintable series parses");
        assert_eq!(
            series,
            vec![
                (1.5, 10.0, 0),
                (1.5, 20.0, 1),
                (2.5, 30.0, 0),
                (2.5, 40.0, 1),
            ]
        );
    }

    #[test]
    fn bintable_named_series_carries_field_names_and_units() {
        let buf = mrm_table();
        let named = parse_bintable_series(&buf).expect("bintable series parses");
        assert_eq!(named.names, vec!["t1", "t2"]);
        assert_eq!(named.units, vec![Some("K".to_string()), None]);
        assert_eq!(named.rows.len(), 4);
    }

    #[test]
    fn bintable_without_et_axis_is_absent() {
        let cards = [
            ("BITPIX", "8"),
            ("NAXIS", "2"),
            ("NAXIS1", "12"),
            ("NAXIS2", "1"),
            ("PCOUNT", "0"),
            ("GCOUNT", "1"),
            ("TFIELDS", "2"),
            ("TTYPE1", "'time'"),
            ("TFORM1", "D"),
            ("TTYPE2", "'t1'"),
            ("TFORM2", "E"),
        ];
        let mut raw = Vec::new();
        raw.extend_from_slice(&1.0f64.to_be_bytes());
        raw.extend_from_slice(&2.0f32.to_be_bytes());
        let buf = bintable(&cards, &raw);
        assert!(parse_bintable_series(&buf).is_none());
        assert!(parse_series(&buf).is_none());
    }

    #[test]
    fn parse_series_rejects_foreign_bytes() {
        assert!(parse_series(b"").is_none());
        assert!(parse_series(b"XXXX").is_none());
        assert!(parse_image(b"P3IM000000000000").is_none());
    }
}
