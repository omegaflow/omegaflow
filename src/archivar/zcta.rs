pub const FORMAT: &str = "zcta_gazetteer";
pub const NETLOC: &str = "www2.census.gov";
pub const MAGIC: [u8; 4] = *b"ZCTA";
pub const REC_BYTES: usize = 4 + 8 + 8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zcta {
    pub geoid: u32,
    pub lat: f64,
    pub lon: f64,
}

fn numeric(cell: &str) -> Option<f64> {
    let t = cell.trim();
    if t.is_empty() {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn parse_row(line: &str) -> Option<Zcta> {
    let cols: Vec<&str> = line.split('\t').collect();
    if cols.len() < 7 {
        return None;
    }
    let geoid: u32 = cols[0].trim().parse().ok()?;
    let lat = numeric(cols[5])?;
    let lon = numeric(cols[6])?;
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    Some(Zcta { geoid, lat, lon })
}

pub fn parse_txt(text: &str) -> Vec<Zcta> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim_end();
        if t.trim().is_empty() {
            continue;
        }
        if t.starts_with("GEOID") {
            continue;
        }
        if let Some(z) = parse_row(t) {
            out.push(z);
        }
    }
    out.sort_by_key(|z| z.geoid);
    out
}

pub fn write_bin(rows: &[Zcta]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + rows.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for z in rows {
        out.extend_from_slice(&z.geoid.to_le_bytes());
        out.extend_from_slice(&z.lat.to_le_bytes());
        out.extend_from_slice(&z.lon.to_le_bytes());
    }
    out
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<Zcta>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let base = 8 + i * REC_BYTES;
        let geoid = u32::from_le_bytes(bytes[base..base + 4].try_into().ok()?);
        let lat = f64::from_le_bytes(bytes[base + 4..base + 12].try_into().ok()?);
        let lon = f64::from_le_bytes(bytes[base + 12..base + 20].try_into().ok()?);
        out.push(Zcta { geoid, lat, lon });
    }
    Some(out)
}

pub fn lookup(rows: &[Zcta], zip: &str) -> Option<(f64, f64)> {
    let geoid: u32 = zip.trim().parse().ok()?;
    rows.binary_search_by_key(&geoid, |z| z.geoid)
        .ok()
        .map(|i| (rows[i].lat, rows[i].lon))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "GEOID\tALAND\tAWATER\tALAND_SQMI\tAWATER_SQMI\tINTPTLAT\tINTPTLONG\n\
00601\t166659744\t799292\t64.348\t0.309\t18.180555\t-66.749961\n\
21093\t100\t10\t1.0\t0.1\t39.4\t-76.7\n\
\t\t\t\t\t\t\n";

    #[test]
    fn parse_txt_skips_the_header_and_reads_the_centroid() {
        let rows = parse_txt(FIXTURE);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].geoid, 601);
        assert!((rows[0].lat - 18.180555).abs() < 1e-9);
        assert!((rows[0].lon + 66.749961).abs() < 1e-9);
    }

    #[test]
    fn a_row_without_a_geoid_or_centroid_stays_absent() {
        let rows = parse_txt("GEOID\tA\tW\tX\tY\tINTPTLAT\tINTPTLONG\n\t \t\t\t\t\t\n");
        assert!(rows.is_empty());
    }

    #[test]
    fn a_missing_lat_or_lon_stays_absent() {
        assert!(parse_row("21093\t1\t1\t1\t1\t\t-76.7").is_none());
        assert!(parse_row("21093\t1\t1\t1\t1\t99.0\t-76.7").is_none());
    }

    #[test]
    fn bin_roundtrip_keeps_every_centroid() {
        let rows = parse_txt(FIXTURE);
        let bytes = write_bin(&rows);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(parse_bin(&bytes).unwrap(), rows);
    }

    #[test]
    fn a_foreign_or_truncated_bin_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&parse_txt(FIXTURE));
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn lookup_resolves_a_leading_zero_zip() {
        let rows = parse_txt(FIXTURE);
        assert_eq!(lookup(&rows, "00601"), Some((18.180555, -66.749961)));
        assert_eq!(lookup(&rows, "21093"), Some((39.4, -76.7)));
        assert_eq!(lookup(&rows, "99999"), None);
    }
}
