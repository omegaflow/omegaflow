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

pub fn parse_gim(body: &str, default_exponent: f64) -> Vec<TecGrid> {
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
    }
    out
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
}
