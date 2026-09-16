use super::*;
use crate::lsk::days_from_civil;

pub struct TecGrid {
    pub epoch_unix: f64,
    pub lat_first: f64,
    pub lat_step: f64,
    pub nlat: usize,
    pub lon_first: f64,
    pub lon_step: f64,
    pub nlon: usize,
    pub cells: Vec<f64>,
}

fn header_exponent(body: &str) -> Option<f64> {
    for line in body.lines() {
        if line.ends_with("EXPONENT") {
            return line
                .split_whitespace()
                .next()
                .and_then(|t| t.parse::<f64>().ok());
        }
        if line.contains("START OF TEC MAP") {
            break;
        }
    }
    None
}

fn epoch_unix_of(epoch_line: &str) -> Option<f64> {
    let f = |s: &str| s.trim().parse::<i64>().ok();
    let (Some(y), Some(mo), Some(d), Some(h), Some(mi), Some(se)) = (
        f(&epoch_line[2..8]),
        f(&epoch_line[8..14]),
        f(&epoch_line[14..20]),
        f(&epoch_line[20..26]),
        f(&epoch_line[26..32]),
        f(&epoch_line[32..38]),
    ) else {
        return None;
    };
    let days = days_from_civil(y, mo, d)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + se as f64)
}

pub struct TecMap {
    pub grid: TecGrid,
    pub alt_km: f64,
}

pub fn parse_gim_maps(body: &str, default_exponent: f64) -> Vec<TecMap> {
    let exponent = header_exponent(body).unwrap_or(default_exponent);
    let scale = 10f64.powf(exponent);
    let lines: Vec<&str> = body.lines().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        if !lines[i].contains("START OF TEC MAP") {
            i += 1;
            continue;
        }
        i += 1;
        if i >= lines.len() || lines[i].len() < 38 {
            continue;
        }
        let Some(epoch_unix) = epoch_unix_of(lines[i]) else {
            continue;
        };
        i += 1;
        let mut lat_first = f64::NAN;
        let mut lat_step = f64::NAN;
        let mut nlat = 0usize;
        let mut lon_first = f64::NAN;
        let mut lon_step = f64::NAN;
        let mut nlon = 0usize;
        let mut alt_km = f64::NAN;
        let mut cells: Vec<f64> = Vec::new();
        loop {
            if i >= lines.len() {
                break;
            }
            let cur = lines[i];
            if cur.contains("START OF TEC MAP")
                || cur.contains("END OF TEC MAP")
                || cur.contains("EPOCH")
            {
                break;
            }
            if cur.len() < 8 {
                break;
            }
            let Ok(lat) = cur[2..8].trim().parse::<f64>() else {
                break;
            };
            let Some(lon1) = cur.get(8..14).and_then(|s| s.trim().parse::<f64>().ok()) else {
                break;
            };
            let Some(lon2) = cur.get(14..20).and_then(|s| s.trim().parse::<f64>().ok()) else {
                break;
            };
            let Some(dlon) = cur.get(20..26).and_then(|s| s.trim().parse::<f64>().ok()) else {
                break;
            };
            let nlon_cur = ((lon2 - lon1) / dlon).round() as i64 + 1;
            if !(1..=1000).contains(&nlon_cur) {
                break;
            }
            let nlon_cur = nlon_cur as usize;
            if alt_km.is_nan()
                && let Some(h) = cur.get(26..32).and_then(|s| s.trim().parse::<f64>().ok())
            {
                alt_km = h;
            }
            i += 1;
            let mut row: Vec<f64> = Vec::new();
            let mut remaining = nlon_cur;
            let mut complete = true;
            while remaining > 0 {
                if i >= lines.len() {
                    complete = false;
                    break;
                }
                let take = remaining.min(16);
                if lines[i].len() < 5 * take {
                    complete = false;
                    break;
                }
                let vline = lines[i];
                for k in 0..take {
                    match vline
                        .get(5 * k..5 * k + 5)
                        .and_then(|s| s.trim().parse::<f64>().ok())
                    {
                        Some(v) => row.push(v * scale),
                        None => {
                            complete = false;
                            break;
                        }
                    }
                }
                if !complete {
                    break;
                }
                remaining -= take;
                i += 1;
            }
            if !complete {
                break;
            }
            if nlat == 0 {
                lat_first = lat;
                lon_first = lon1;
                lon_step = dlon;
                nlon = nlon_cur;
            } else if lat_step.is_nan() {
                lat_step = lat - lat_first;
            }
            nlat += 1;
            cells.extend_from_slice(&row);
            if lat < -87.0 {
                break;
            }
        }
        if nlat >= 2 && nlon >= 2 && cells.len() == nlat * nlon {
            out.push(TecMap {
                grid: TecGrid {
                    epoch_unix,
                    lat_first,
                    lat_step,
                    nlat,
                    lon_first,
                    lon_step,
                    nlon,
                    cells,
                },
                alt_km,
            });
        }
    }
    out
}

pub fn parse_gim(body: &str, default_exponent: f64) -> Vec<TecGrid> {
    parse_gim_maps(body, default_exponent)
        .into_iter()
        .map(|m| m.grid)
        .collect()
}

pub fn build_channels(
    src: &SourceConfig,
    text: &str,
    now: f64,
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let Some(fc) = src.extracts.iter().find_map(|e| match e {
        Extract::Field(fc) if fc.key == "tec" => Some(fc),
        _ => None,
    }) else {
        return Vec::new();
    };
    let Some(exponent) = header_exponent(text) else {
        return Vec::new();
    };
    let no_value = 9999.0 * 10f64.powf(exponent);
    let maps = parse_gim_maps(text, exponent);
    let mut best: Option<(&TecMap, f64)> = None;
    for m in &maps {
        if !m.alt_km.is_finite() {
            continue;
        }
        let Some(epoch) = lsk.unix_to_tdb(m.grid.epoch_unix) else {
            continue;
        };
        if epoch > now {
            continue;
        }
        if best.is_none_or(|(b, _)| m.grid.epoch_unix > b.grid.epoch_unix) {
            best = Some((m, epoch));
        }
    }
    let Some((map, epoch)) = best else {
        return Vec::new();
    };
    let body = frame_body_name(&src.frame);
    let alt = map.alt_km * 1000.0;
    let mut channels = Vec::with_capacity(map.grid.cells.len());
    for (idx, tec) in map.grid.cells.iter().enumerate() {
        if !tec.is_finite() || *tec < 0.0 || *tec == no_value {
            continue;
        }
        let row = idx / map.grid.nlon;
        let col = idx % map.grid.nlon;
        let lat = map.grid.lat_first + row as f64 * map.grid.lat_step;
        let lon = map.grid.lon_first + col as f64 * map.grid.lon_step;
        channels.push((
            Channel {
                z: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                epoch,
                position: Position::Surface {
                    body_name: body.clone(),
                    lat,
                    lon,
                    alt,
                },
                name: fc.name.clone(),
                value: *tec,
            },
            fc.clone(),
        ));
    }
    channels
}

pub fn tec_at(g: &TecGrid, lat: f64, lon: f64) -> Option<f64> {
    if g.cells.len() != g.nlat * g.nlon || g.nlat < 2 || g.nlon < 2 {
        return None;
    }
    if !lat.is_finite() || !lon.is_finite() {
        return None;
    }
    if g.lat_step.abs() < 1e-12 || g.lon_step.abs() < 1e-12 {
        return None;
    }
    let flat = ((lat - g.lat_first) / g.lat_step).clamp(0.0, (g.nlat - 1) as f64);
    let flon = ((lon - g.lon_first) / g.lon_step).clamp(0.0, (g.nlon - 1) as f64);
    let b0 = flat.floor() as usize;
    let b1 = (b0 + 1).min(g.nlat - 1);
    let c0 = flon.floor() as usize;
    let c1 = (c0 + 1).min(g.nlon - 1);
    let fb = flat - b0 as f64;
    let fc = flon - c0 as f64;
    let v00 = g.cells[b0 * g.nlon + c0];
    let v01 = g.cells[b0 * g.nlon + c1];
    let v10 = g.cells[b1 * g.nlon + c0];
    let v11 = g.cells[b1 * g.nlon + c1];
    Some(
        v00 * (1.0 - fb) * (1.0 - fc)
            + v01 * (1.0 - fb) * fc
            + v10 * fb * (1.0 - fc)
            + v11 * fb * fc,
    )
}

pub const MAGIC_GIM: [u8; 4] = *b"GIM1";

const MAP_HEADER_BYTES: usize = 8 + 8 + 8 + 4 + 8 + 8 + 4;

pub fn write_gim_bin(maps: &[TecGrid]) -> Vec<u8> {
    let mut cells: usize = 0;
    for g in maps {
        cells = cells.saturating_add(g.cells.len());
    }
    let mut out = Vec::with_capacity(8 + maps.len() * MAP_HEADER_BYTES + cells * 8);
    out.extend_from_slice(&MAGIC_GIM);
    out.extend_from_slice(&(maps.len() as u32).to_le_bytes());
    for g in maps {
        out.extend_from_slice(&g.epoch_unix.to_le_bytes());
        out.extend_from_slice(&g.lat_first.to_le_bytes());
        out.extend_from_slice(&g.lat_step.to_le_bytes());
        out.extend_from_slice(&(g.nlat as u32).to_le_bytes());
        out.extend_from_slice(&g.lon_first.to_le_bytes());
        out.extend_from_slice(&g.lon_step.to_le_bytes());
        out.extend_from_slice(&(g.nlon as u32).to_le_bytes());
        for c in &g.cells {
            out.extend_from_slice(&c.to_le_bytes());
        }
    }
    out
}

pub fn parse_gim_bin(data: &[u8]) -> Option<Vec<TecGrid>> {
    if data.len() < 8 || data[0..4] != MAGIC_GIM {
        return None;
    }
    let n = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    let f64_of = |off: usize| {
        data.get(off..off + 8)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
    };
    let u32_of = |off: usize| {
        data.get(off..off + 4)
            .and_then(|b| b.try_into().ok())
            .map(u32::from_le_bytes)
    };
    let mut off = 8usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let epoch_unix = f64_of(off)?;
        off += 8;
        let lat_first = f64_of(off)?;
        off += 8;
        let lat_step = f64_of(off)?;
        off += 8;
        let nlat = u32_of(off)? as usize;
        off += 4;
        let lon_first = f64_of(off)?;
        off += 8;
        let lon_step = f64_of(off)?;
        off += 8;
        let nlon = u32_of(off)? as usize;
        off += 4;
        if !epoch_unix.is_finite()
            || !lat_first.is_finite()
            || !lat_step.is_finite()
            || !lon_first.is_finite()
            || !lon_step.is_finite()
        {
            return None;
        }
        if nlat == 0 || nlon == 0 || nlat > 100_000 || nlon > 100_000 {
            return None;
        }
        let cell_count = nlat.checked_mul(nlon)?;
        let mut cells = Vec::with_capacity(cell_count);
        for _ in 0..cell_count {
            let c = f64_of(off)?;
            off += 8;
            if !c.is_finite() {
                return None;
            }
            cells.push(c);
        }
        out.push(TecGrid {
            epoch_unix,
            lat_first,
            lat_step,
            nlat,
            lon_first,
            lon_step,
            nlon,
            cells,
        });
    }
    if off != data.len() {
        return None;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_gim(maps: usize) -> String {
        let mut body = String::new();
        for map in 1..=maps {
            body.push_str(&format!(
                "     {map}                                                      START OF TEC MAP\n  2024     1     1     {map}     0     0                        EPOCH OF CURRENT MAP\n"
            ));
            for b in 0..71 {
                let lat = 87.5 - 2.5 * b as f64;
                body.push_str(&format!(
                    "  {lat:>6.1}-180.0 180.0   5.0 450.0                            LAT/LON1/LON2/DLON/H\n"
                ));
                let v = 10 * map;
                for line_in_band in 0..5 {
                    let n = if line_in_band < 4 { 16 } else { 9 };
                    for _ in 0..n {
                        body.push_str(&format!("{v:>5}"));
                    }
                    body.push('\n');
                }
            }
            body.push_str(
                "     1                                                      END OF TEC MAP\n",
            );
        }
        body
    }

    #[test]
    fn parses_synthetic_gim_with_fallback_exponent() {
        let grids = parse_gim(&synthetic_gim(2), -1.0);
        assert_eq!(grids.len(), 2);
        assert_eq!(grids[0].nlat, 71);
        assert_eq!(grids[0].nlon, 73);
        assert_eq!(grids[0].lat_first, 87.5);
        assert_eq!(grids[0].lat_step, -2.5);
        assert_eq!(grids[0].lon_first, -180.0);
        assert_eq!(grids[0].lon_step, 5.0);
        let v0 = tec_at(&grids[0], 0.0, 0.0).unwrap();
        assert!((v0 - 1.0).abs() < 1e-9, "10·10^-1 TECU = 1.0, got {v0}");
        let v1 = tec_at(&grids[1], 0.0, 0.0).unwrap();
        assert!((v1 - 2.0).abs() < 1e-9);
        let day0 = days_from_civil(2024, 1, 1).unwrap() as f64 * 86400.0;
        assert!((grids[0].epoch_unix - (day0 + 3600.0)).abs() < 1e-9);
        assert!((grids[1].epoch_unix - (day0 + 7200.0)).abs() < 1e-9);
    }

    #[test]
    fn parse_gim_maps_carries_the_shell_altitude() {
        let maps = parse_gim_maps(&synthetic_gim(1), -1.0);
        assert_eq!(maps.len(), 1);
        assert_eq!(maps[0].alt_km, 450.0);
        assert_eq!(maps[0].grid.nlat, 71);
    }

    #[test]
    fn header_exponent_overrides_the_fallback() {
        let mut body = String::from("     0                                          EXPONENT\n");
        body.push_str(&synthetic_gim(1));
        let grids = parse_gim(&body, -1.0);
        assert_eq!(grids.len(), 1);
        let v = tec_at(&grids[0], 0.0, 0.0).unwrap();
        assert!((v - 10.0).abs() < 1e-9, "Exponent 0 → 10 TECU, got {v}");
    }

    #[test]
    fn bilinear_interpolation_midpoint_is_the_mean() {
        let mut cells = vec![0.0f64; 4];
        cells[0] = 1.0;
        cells[1] = 3.0;
        cells[2] = 5.0;
        cells[3] = 7.0;
        let g = TecGrid {
            epoch_unix: 0.0,
            lat_first: 1.0,
            lat_step: -2.0,
            nlat: 2,
            lon_first: -1.0,
            lon_step: 2.0,
            nlon: 2,
            cells,
        };
        let v = tec_at(&g, 0.0, 0.0).unwrap();
        assert!((v - 4.0).abs() < 1e-12, "midpoint mean, got {v}");
        let v00 = tec_at(&g, 1.0, -1.0).unwrap();
        assert!((v00 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn incomplete_band_ends_the_map() {
        let mut body = String::new();
        body.push_str(
            "     1                                                      START OF TEC MAP\n",
        );
        body.push_str(
            "  2024     1     1     1     0     0                        EPOCH OF CURRENT MAP\n",
        );
        body.push_str(
            "  87.5-180.0 180.0   5.0 450.0                            LAT/LON1/LON2/DLON/H\n",
        );
        for _ in 0..10 {
            body.push_str("    1");
        }
        body.push('\n');
        let grids = parse_gim(&body, -1.0);
        assert!(grids.is_empty(), "the truncated map stays uncarried");
    }

    #[test]
    fn gim_bin_roundtrip_preserves_every_map() {
        let grids = parse_gim(&synthetic_gim(2), -1.0);
        assert_eq!(grids.len(), 2);
        let bin = write_gim_bin(&grids);
        let parsed = parse_gim_bin(&bin).unwrap();
        assert_eq!(parsed.len(), 2);
        for (a, b) in parsed.iter().zip(grids.iter()) {
            assert_eq!(a.epoch_unix, b.epoch_unix);
            assert_eq!(a.lat_first, b.lat_first);
            assert_eq!(a.lat_step, b.lat_step);
            assert_eq!(a.nlat, b.nlat);
            assert_eq!(a.lon_first, b.lon_first);
            assert_eq!(a.lon_step, b.lon_step);
            assert_eq!(a.nlon, b.nlon);
            assert_eq!(a.cells, b.cells);
        }
        let v = tec_at(&parsed[0], 0.0, 0.0).unwrap();
        assert!((v - 1.0).abs() < 1e-9);
    }

    #[test]
    fn gim_bin_rejects_foreign_and_truncated_bytes() {
        assert!(parse_gim_bin(b"X").is_none());
        assert!(parse_gim_bin(b"GIM1abc").is_none());
        let grids = parse_gim(&synthetic_gim(1), -1.0);
        let bin = write_gim_bin(&grids);
        assert!(parse_gim_bin(&bin[..bin.len() - 1]).is_none());
    }
}
