use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GbcoRec, MAGIC_OCS, parse_ocs, write_ocs};
use omegaflow::archivar::gpkg::{SqliteDb, SqliteValue};
use omegaflow::archivar::json::{JsonVal, jpath_val, jstr, parse_json};
use omegaflow::archivar::tiff::parse_tiff;
use omegaflow::cdn::upload_release;
use omegaflow::zeuge::{FeldIdentitaet, ZeugeArt, magic_identity};

const NETLOC: &str = "noaa-ocs-hydrodata-pds.s3.amazonaws.com";
const BASE: &str = "https://noaa-ocs-hydrodata-pds.s3.amazonaws.com";

const GRS80_A: f64 = 6378137.0;
const GRS80_INV_F: f64 = 298.257222101;
const UTM18_CENTRAL_MERIDIAN_DEG: f64 = -75.0;
const UTM_K0: f64 = 0.9996;
const UTM_FALSE_EASTING: f64 = 500000.0;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn witness_gestalt_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt)) => {
            eprintln!(
                "{} reads as a gestalt witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not gestalt — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn resolve(base: &str, href: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        return href.to_string();
    }
    let mut h = href;
    while let Some(rest) = h.strip_prefix("./") {
        h = rest;
    }
    let dir_end = match base.rfind('/').map(|i| i + 1) {
        Some(v) => v,
        None => 0,
    };
    format!("{}{}", &base[..dir_end], h)
}

fn child_qualified_href(catalog: &JsonVal) -> Option<String> {
    let links = match jpath_val(catalog, "links") {
        Some(JsonVal::Arr(arr)) => arr,
        _ => return None,
    };
    for l in links {
        if jstr(l, "rel").as_deref() == Some("child")
            && jstr(l, "title").as_deref() == Some("Qualified")
        {
            return jstr(l, "href");
        }
    }
    None
}

fn item_hrefs(collection: &JsonVal) -> Vec<String> {
    let links = match jpath_val(collection, "links") {
        Some(JsonVal::Arr(arr)) => arr,
        _ => return Vec::new(),
    };
    let mut out = Vec::new();
    for l in links {
        if jstr(l, "rel").as_deref() == Some("item") {
            if let Some(h) = jstr(l, "href") {
                out.push(h);
            }
        }
    }
    out
}

fn gpkg_asset_href(item: &JsonVal) -> Option<String> {
    let assets = match jpath_val(item, "assets") {
        Some(JsonVal::Obj(map)) => map,
        _ => return None,
    };
    for (name, asset) in assets {
        if name.ends_with(".gpkg") {
            return jstr(asset, "href");
        }
    }
    None
}

fn tif_asset_href(item: &JsonVal) -> Option<String> {
    let assets = match jpath_val(item, "assets") {
        Some(JsonVal::Obj(map)) => map,
        _ => return None,
    };
    for (name, asset) in assets {
        if name.ends_with(".tif") {
            return jstr(asset, "href");
        }
    }
    None
}

fn first_ident(segment: &str) -> Option<String> {
    let t = segment.trim_start();
    for quote in ['"', '`'] {
        if let Some(rest) = t.strip_prefix(quote) {
            let end = rest.find(quote)?;
            return Some(rest[..end].to_string());
        }
    }
    let word: String = t
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if word.is_empty() { None } else { Some(word) }
}

fn columns_from_create_sql(sql: &str) -> Option<Vec<String>> {
    let open = sql.find('(')?;
    let close = sql.rfind(')')?;
    let body = &sql[open + 1..close];
    let mut cols = Vec::new();
    let mut start = 0usize;
    let mut depth = 0i32;
    let mut in_quote = false;
    for (i, &b) in body.as_bytes().iter().enumerate() {
        let c = b as char;
        if c == '"' || c == '\'' || c == '`' {
            in_quote = !in_quote;
            continue;
        }
        if in_quote {
            continue;
        }
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                cols.push(first_ident(&body[start..i])?);
                start = i + 1;
            }
            _ => {}
        }
    }
    cols.push(first_ident(&body[start..])?);
    Some(cols)
}

fn sounding_columns(db: &SqliteDb) -> Option<(usize, usize, usize)> {
    let sql = db.table_schema("soundings")?;
    let cols = columns_from_create_sql(&sql)?;
    let x = cols.iter().position(|c| c.eq_ignore_ascii_case("X"))?;
    let y = cols.iter().position(|c| c.eq_ignore_ascii_case("Y"))?;
    let elev = cols
        .iter()
        .position(|c| c.eq_ignore_ascii_case("Elevation"))?;
    Some((x, y, elev))
}

fn real_at(row: &[SqliteValue], i: usize) -> Option<f64> {
    match row.get(i) {
        Some(SqliteValue::Real(v)) => Some(*v),
        Some(SqliteValue::Int(v)) => Some(*v as f64),
        _ => None,
    }
}

fn utm18n_to_latlon(easting: f64, northing: f64) -> Option<(f64, f64)> {
    let a = GRS80_A;
    let f = 1.0 / GRS80_INV_F;
    let e2 = f * (2.0 - f);
    let ep2 = e2 / (1.0 - e2);
    let x = easting - UTM_FALSE_EASTING;
    let m = northing / UTM_K0;
    let mu = m / (a * (1.0 - e2 / 4.0 - 3.0 * e2 * e2 / 64.0 - 5.0 * e2 * e2 * e2 / 256.0));
    let e1 = (1.0 - (1.0 - e2).sqrt()) / (1.0 + (1.0 - e2).sqrt());
    let phi1 = mu
        + (3.0 * e1 / 2.0 - 27.0 * e1.powi(3) / 32.0) * (2.0 * mu).sin()
        + (21.0 * e1.powi(2) / 16.0 - 55.0 * e1.powi(4) / 32.0) * (4.0 * mu).sin()
        + (151.0 * e1.powi(3) / 96.0) * (6.0 * mu).sin()
        + (1097.0 * e1.powi(4) / 512.0) * (8.0 * mu).sin();
    let sin_phi1 = phi1.sin();
    let cos_phi1 = phi1.cos();
    let tan_phi1 = phi1.tan();
    let n1 = a / (1.0 - e2 * sin_phi1 * sin_phi1).sqrt();
    let r1 = a * (1.0 - e2) / (1.0 - e2 * sin_phi1 * sin_phi1).powf(1.5);
    let t1 = tan_phi1 * tan_phi1;
    let c1 = ep2 * cos_phi1 * cos_phi1;
    let d = x / (n1 * UTM_K0);
    let phi = phi1
        - (n1 * tan_phi1 / r1)
            * (d * d / 2.0
                - (5.0 + 3.0 * t1 + 10.0 * c1 - 4.0 * c1 * c1 - 9.0 * ep2) * d.powi(4) / 24.0
                + (61.0 + 90.0 * t1 + 298.0 * c1 + 45.0 * t1 * t1 - 252.0 * ep2 - 3.0 * c1 * c1)
                    * d.powi(6)
                    / 720.0);
    let lam = UTM18_CENTRAL_MERIDIAN_DEG.to_radians()
        + (d - (1.0 + 2.0 * t1 + c1) * d.powi(3) / 6.0
            + (5.0 - 2.0 * c1 + 28.0 * t1 - 3.0 * c1 * c1 + 8.0 * ep2 + 24.0 * t1 * t1)
                * d.powi(5)
                / 120.0)
            / cos_phi1;
    let lat = phi.to_degrees();
    let lon = lam.to_degrees();
    if lat.is_finite() && lon.is_finite() {
        Some((lat, lon))
    } else {
        None
    }
}

fn soundings_from_gpkg(bytes: &[u8]) -> Option<Vec<GbcoRec>> {
    let db = SqliteDb::from_bytes(bytes.to_vec())?;
    let (xi, yi, ei) = sounding_columns(&db)?;
    let rows = db.read_table("soundings")?;
    let mut recs = Vec::with_capacity(rows.len());
    for row in &rows {
        let Some(x) = real_at(row, xi) else { continue };
        let Some(y) = real_at(row, yi) else { continue };
        let Some(elev) = real_at(row, ei) else {
            continue;
        };
        if !x.is_finite() || !y.is_finite() || !elev.is_finite() {
            continue;
        }
        let Some((lat, lon)) = utm18n_to_latlon(x, y) else {
            continue;
        };
        recs.push(GbcoRec { lat, lon, elev });
    }
    Some(recs)
}

fn soundings_from_tif(bytes: &[u8]) -> Option<Vec<GbcoRec>> {
    let img = parse_tiff(bytes)?;
    if img.pixels.is_empty() {
        return None;
    }
    if *img.bits_per_sample.first()? != 32 {
        return None;
    }
    if img.samples_per_pixel == 0 {
        return None;
    }
    let geo = img.geo.as_ref()?;
    let mut bytes_per_pixel = 0usize;
    for &b in &img.bits_per_sample {
        if b % 8 != 0 {
            return None;
        }
        bytes_per_pixel += (b / 8) as usize;
    }
    if bytes_per_pixel == 0 {
        return None;
    }
    let width = img.width as usize;
    let height = img.height as usize;
    let row_bytes = width.checked_mul(bytes_per_pixel)?;
    let expected = row_bytes.checked_mul(height)?;
    if img.pixels.len() < expected {
        return None;
    }
    let mut recs = Vec::new();
    for row in 0..height {
        for col in 0..width {
            let lon = geo.x0 + (col as f64 + 0.5) * geo.dx;
            let lat = geo.y0 + (row as f64 + 0.5) * geo.dy;
            if !lat.is_finite()
                || !lon.is_finite()
                || lat < -90.0
                || lat > 90.0
                || lon < -180.0
                || lon > 180.0
            {
                continue;
            }
            let off = (row * width + col) * bytes_per_pixel;
            let v = img.pixels.get(off..off + 4)?;
            let elev = f32::from_le_bytes(v.try_into().ok()?);
            if !elev.is_finite() || elev == f32::MAX {
                continue;
            }
            recs.push(GbcoRec {
                lat,
                lon,
                elev: elev as f64,
            });
        }
    }
    Some(recs)
}

fn harvest_survey(item_url: &str) -> Option<Vec<GbcoRec>> {
    let item_body = fetch_raw(item_url, None, &[], 3600)?;
    let item = parse_json(&item_body)?;
    let mut recs = Vec::new();
    let mut asset_seen = false;
    if let Some(href) = gpkg_asset_href(&item) {
        asset_seen = true;
        let url = resolve(item_url, &href);
        if let Some(bytes) = fetch_raw_bytes(&url, 3600) {
            if let Some(r) = soundings_from_gpkg(&bytes) {
                recs.extend(r);
            }
        }
    }
    if let Some(href) = tif_asset_href(&item) {
        asset_seen = true;
        let url = resolve(item_url, &href);
        if let Some(bytes) = fetch_raw_bytes(&url, 3600) {
            if let Some(r) = soundings_from_tif(&bytes) {
                recs.extend(r);
            }
        }
    }
    if asset_seen { Some(recs) } else { None }
}

fn survey_id(href: &str) -> Option<&str> {
    let trimmed = href.trim_start_matches("./");
    let mut parts = trimmed.split('/');
    let first = parts.next()?;
    match parts.next() {
        Some(_) => Some(first),
        None => None,
    }
}

fn write_records(out: &str, records: &[GbcoRec]) -> Result<usize, String> {
    if let Some(parent) = std::path::Path::new(out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let bytes = write_ocs(records);
    std::fs::write(out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_ocs(&bytes) {
        Some(parsed) if parsed.len() == records.len() => Ok(parsed.len()),
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    witness_gestalt_identity(MAGIC_OCS)?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/ocs_hydro_depth.bin"),
    };
    let max_surveys = arg_value(args, "--max-surveys").and_then(|v| v.parse::<usize>().ok());
    let all = args.iter().any(|a| a == "--all");
    if all && (arg_value(args, "--survey").is_some() || max_surveys.is_some()) {
        return Err(
            "--all declares every survey; --survey/--max-surveys contradict it".to_string(),
        );
    }

    let mut records: Vec<GbcoRec> = Vec::new();

    if let Some(path) = arg_value(args, "--gpkg") {
        let bytes = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        let recs = soundings_from_gpkg(&bytes)
            .ok_or_else(|| format!("{path}: the soundings table reads void"))?;
        eprintln!("{path}: {} sounding records", recs.len());
        records.extend(recs);
    } else {
        let catalog_url = format!("{BASE}/catalog.json");
        let catalog_body = fetch_raw(&catalog_url, None, &[], 3600)
            .ok_or_else(|| format!("{catalog_url}: fetch void"))?;
        let catalog =
            parse_json(&catalog_body).ok_or_else(|| format!("{catalog_url}: json void"))?;
        let collection_href = child_qualified_href(&catalog)
            .ok_or_else(|| "catalog carries no Qualified child collection".to_string())?;
        let collection_url = resolve(&catalog_url, &collection_href);
        let collection_body = fetch_raw(&collection_url, None, &[], 3600)
            .ok_or_else(|| format!("{collection_url}: fetch void"))?;
        let collection =
            parse_json(&collection_body).ok_or_else(|| format!("{collection_url}: json void"))?;

        let mut item_hrefs = item_hrefs(&collection);
        if let Some(survey) = arg_value(args, "--survey") {
            item_hrefs.retain(|h| h.contains(&format!("/{survey}/")));
        }
        item_hrefs.sort();
        item_hrefs.dedup();

        if all {
            let out_dir = match arg_value(args, "--out-dir") {
                Some(v) => v,
                None => format!("data/{NETLOC}/ocs_all"),
            };
            let mut written = 0usize;
            for href in &item_hrefs {
                let item_url = resolve(&collection_url, href);
                let Some(id) = survey_id(href) else {
                    eprintln!(
                        "{item_url}: no survey id in the href — the survey stays unharvested"
                    );
                    continue;
                };
                match harvest_survey(&item_url) {
                    Some(mut recs) if !recs.is_empty() => {
                        recs.sort_by(|a, b| {
                            a.lat
                                .total_cmp(&b.lat)
                                .then(a.lon.total_cmp(&b.lon))
                                .then(a.elev.total_cmp(&b.elev))
                        });
                        let path = format!("{out_dir}/{id}.bin");
                        match write_records(&path, &recs) {
                            Ok(n) => {
                                eprintln!("{item_url}: {n} depth records written to {path}");
                                if ci_mode && !upload_release(NETLOC, &path) {
                                    return Err(format!("{path}: CDN upload returned void"));
                                }
                                written += 1;
                            }
                            Err(msg) => eprintln!("{item_url}: {msg}"),
                        }
                    }
                    Some(_) => eprintln!(
                        "{item_url}: soundings read empty — the survey stays unharvested (0 honored)"
                    ),
                    None => {
                        eprintln!("{item_url}: soundings void — the survey stays unharvested")
                    }
                }
            }
            if written == 0 {
                return Err(format!(
                    "{collection_url}: no survey carried measured soundings — the asset stays unwritten (0 honored)"
                ));
            }
            return Ok(());
        }

        let mut surveys = 0usize;
        for href in &item_hrefs {
            if let Some(m) = max_surveys {
                if surveys >= m {
                    break;
                }
            }
            let item_url = resolve(&collection_url, href);
            match harvest_survey(&item_url) {
                Some(recs) => {
                    eprintln!("{item_url}: {} sounding records", recs.len());
                    records.extend(recs);
                    surveys += 1;
                }
                None => {
                    eprintln!("{item_url}: soundings void — the survey stays unharvested");
                }
            }
        }
        if surveys == 0 {
            return Err(format!(
                "{collection_url}: no survey carried measured soundings — the asset stays unwritten (0 honored)"
            ));
        }
    }

    if records.is_empty() {
        return Err("no measured soundings — the asset stays unwritten (0 honored)".to_string());
    }
    records.sort_by(|a, b| {
        a.lat
            .total_cmp(&b.lat)
            .then(a.lon.total_cmp(&b.lon))
            .then(a.elev.total_cmp(&b.elev))
    });
    let bytes = write_ocs(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_ocs(&bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!("{out}: {} depth records, roundtrip parses", parsed.len());
            Ok(())
        }
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }?;
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("noaa_ocs_hydrodata_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utm18n_central_meridian_inverse() {
        let (lat, lon) = utm18n_to_latlon(500000.0, 4649776.224884).unwrap();
        assert!((lat - 42.0).abs() < 1e-6);
        assert!((lon - (-75.0)).abs() < 1e-6);
    }

    #[test]
    fn utm18n_dd10045_extent_corner() {
        let (lat, lon) = utm18n_to_latlon(368952.482, 4340727.52).unwrap();
        assert!((lat - 39.205974).abs() < 1e-4);
        assert!((lon - (-76.517841)).abs() < 1e-4);
    }

    #[test]
    fn columns_parse_soundings_schema() {
        let sql = r#"CREATE TABLE IF NOT EXISTS "soundings" ( "fid" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "geom" POINT, "X" REAL, "Y" REAL, "Elevation" REAL, "Uncertainty" REAL)"#;
        assert_eq!(
            columns_from_create_sql(sql).unwrap(),
            vec!["fid", "geom", "X", "Y", "Elevation", "Uncertainty"]
        );
    }

    #[test]
    fn resolve_relative_and_absolute_hrefs() {
        assert_eq!(
            resolve(
                "https://noaa-ocs-hydrodata-pds.s3.amazonaws.com/catalog.json",
                "./Qualified/collection.json"
            ),
            "https://noaa-ocs-hydrodata-pds.s3.amazonaws.com/Qualified/collection.json"
        );
        assert_eq!(
            resolve(
                "https://noaa-ocs-hydrodata-pds.s3.amazonaws.com/Qualified/collection.json",
                "https://noaa-ocs-hydrodata-pds.s3.amazonaws.com/03-Staging/Qualified/DD10045/DD10045_VB_MLLW_1of1.gpkg"
            ),
            "https://noaa-ocs-hydrodata-pds.s3.amazonaws.com/03-Staging/Qualified/DD10045/DD10045_VB_MLLW_1of1.gpkg"
        );
    }

    #[test]
    fn ocs_magic_roundtrips_as_gestalt() {
        let recs = vec![GbcoRec {
            lat: 39.2059,
            lon: -76.5178,
            elev: -10.363,
        }];
        let bytes = write_ocs(&recs);
        assert_eq!(parse_ocs(&bytes).unwrap().len(), 1);
        assert_eq!(
            magic_identity(MAGIC_OCS),
            Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt))
        );
    }

    #[test]
    fn tif_asset_href_reads_tif_asset() {
        let item =
            parse_json(r#"{"assets":{"a.gpkg":{"href":"g"},"b.tif":{"href":"https://x/b.tif"}}}"#)
                .unwrap();
        assert_eq!(tif_asset_href(&item).as_deref(), Some("https://x/b.tif"));
        let gpkg_only = parse_json(r#"{"assets":{"a.gpkg":{"href":"g"}}}"#).unwrap();
        assert_eq!(tif_asset_href(&gpkg_only), None);
    }

    fn ifd_entry(out: &mut Vec<u8>, tag: u16, field_type: u16, count: u32, value: u32) {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&field_type.to_le_bytes());
        out.extend_from_slice(&count.to_le_bytes());
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn build_float32_geotiff(
        width: u32,
        height: u32,
        pixels: &[u8],
        compression: u16,
        scale: [f64; 3],
        tie: [f64; 6],
    ) -> Vec<u8> {
        let entry_count = 11u16;
        let pixel_offset = (8 + 2 + entry_count as usize * 12 + 4) as u32;
        let scale_offset = pixel_offset + pixels.len() as u32;
        let tie_offset = scale_offset + 24;
        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&entry_count.to_le_bytes());
        ifd_entry(&mut out, 256, 4, 1, width);
        ifd_entry(&mut out, 257, 4, 1, height);
        ifd_entry(&mut out, 258, 3, 2, 32 | (32 << 16));
        ifd_entry(&mut out, 259, 3, 1, compression as u32);
        ifd_entry(&mut out, 262, 3, 1, 1);
        ifd_entry(&mut out, 273, 4, 1, pixel_offset);
        ifd_entry(&mut out, 277, 3, 1, 2);
        ifd_entry(&mut out, 278, 4, 1, height);
        ifd_entry(&mut out, 279, 4, 1, pixels.len() as u32);
        ifd_entry(&mut out, 33550, 12, 3, scale_offset);
        ifd_entry(&mut out, 33922, 12, 6, tie_offset);
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(pixels);
        for v in scale {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in tie {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    fn lzw_encode(input: &[u8]) -> Vec<u8> {
        use std::collections::HashMap;
        let mut dict: HashMap<Vec<u8>, u16> = HashMap::new();
        for i in 0..256u16 {
            dict.insert(vec![i as u8], i);
        }
        let mut free = 258u16;
        let mut width = 9u32;
        let mut codes: Vec<(u16, u32)> = vec![(256, 9)];
        let mut w: Vec<u8> = Vec::new();
        for &k in input {
            let mut wk = w.clone();
            wk.push(k);
            if dict.contains_key(&wk) {
                w = wk;
            } else {
                codes.push((dict[&w], width));
                if (free as usize) < 4096 {
                    dict.insert(wk, free);
                    free += 1;
                    if free == (1u16 << width) - 1 && width < 12 {
                        width += 1;
                    }
                }
                w = vec![k];
            }
        }
        codes.push((dict[&w], width));
        codes.push((257, width));
        let mut out = Vec::new();
        let mut acc = 0u32;
        let mut nbits = 0u32;
        for (code, w) in codes {
            acc = (acc << w) | (code as u32);
            nbits += w;
            while nbits >= 8 {
                nbits -= 8;
                out.push((acc >> nbits) as u8);
                acc &= (1 << nbits) - 1;
            }
        }
        if nbits > 0 {
            out.push((acc << (8 - nbits)) as u8);
        }
        out
    }

    #[test]
    fn soundings_from_tif_decodes_float32_elevation() {
        let floats = [-10.0f32, 0.5, f32::MAX, 0.0, -20.0, 1.0, -30.0, 2.0];
        let pixels: Vec<u8> = floats.iter().flat_map(|f| f.to_le_bytes()).collect();
        let data = build_float32_geotiff(
            2,
            2,
            &pixels,
            1,
            [0.5, 0.5, 0.0],
            [0.0, 0.0, 0.0, 10.0, 20.0, 0.0],
        );
        let recs = soundings_from_tif(&data).unwrap();
        assert_eq!(recs.len(), 3);
        assert_eq!(recs[0].lat, 19.75);
        assert_eq!(recs[0].lon, 10.25);
        assert_eq!(recs[0].elev, -10.0);
        assert_eq!(recs[1].lat, 19.25);
        assert_eq!(recs[1].lon, 10.25);
        assert_eq!(recs[1].elev, -20.0);
        assert_eq!(recs[2].lat, 19.25);
        assert_eq!(recs[2].lon, 10.75);
        assert_eq!(recs[2].elev, -30.0);
    }

    #[test]
    fn soundings_from_tif_decodes_lzw() {
        let floats = [-12.5f32, 0.25, -13.5, 0.5];
        let raw: Vec<u8> = floats.iter().flat_map(|f| f.to_le_bytes()).collect();
        let lzw = lzw_encode(&raw);
        let data = build_float32_geotiff(
            2,
            1,
            &lzw,
            5,
            [1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, -75.0, 40.0, 0.0],
        );
        let recs = soundings_from_tif(&data).unwrap();
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].lat, 39.5);
        assert_eq!(recs[0].lon, -74.5);
        assert_eq!(recs[0].elev, -12.5);
        assert_eq!(recs[1].lon, -73.5);
        assert_eq!(recs[1].elev, -13.5);
    }
}
