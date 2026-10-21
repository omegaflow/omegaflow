use omegaflow::archivar::asset_path::compiled;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    COMP_MWO_DP_VLOS, COMP_MWO_MP_BLOS, GeoRec, MAGIC_MWO_MAP, parse_bin as parse_geo_bin,
    write_bin as write_geo_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage};
use omegaflow::lsk::{LeapSeconds, days_from_civil};

const NETLOC: &str = "sha.stanford.edu";
const SECONDS_PER_DAY: f64 = 86400.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Mp,
    Dp,
}

impl Kind {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "mp" => Some(Kind::Mp),
            "dp" => Some(Kind::Dp),
            _ => None,
        }
    }

    fn default_bunit(self) -> &'static str {
        match self {
            Kind::Mp => "GAUSS",
            Kind::Dp => "KM/SEC",
        }
    }

    fn value_scale(self) -> f64 {
        match self {
            Kind::Mp => 1e-4,
            Kind::Dp => 1e3,
        }
    }

    fn comp(self) -> u32 {
        match self {
            Kind::Mp => COMP_MWO_MP_BLOS,
            Kind::Dp => COMP_MWO_DP_VLOS,
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

fn load_lsk(path: Option<&str>) -> Option<LeapSeconds> {
    match path {
        Some(p) => {
            let text = std::fs::read_to_string(p).ok()?;
            omegaflow::lsk::parse(&text)
        }
        None => omegaflow::archivar::embedded_lsk(),
    }
}

fn bunit_scale(kind: Kind, header: &FitsHeader) -> Option<f64> {
    let Some(raw) = header.str_unescaped("BUNIT") else {
        eprintln!(
            "BUNIT absent in header — falling back to the {kind:?} canonical {:?} (measured gap)",
            kind.default_bunit()
        );
        return Some(kind.value_scale());
    };
    let unit: String = raw
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_ascii_uppercase();
    let matches = match kind {
        Kind::Mp => unit == "GAUSS" || unit == "G",
        Kind::Dp => unit == "KM/SEC" || unit == "KM/S" || unit == "KM.S-1",
    };
    if !matches {
        eprintln!(
            "BUNIT {raw:?} does not match --kind {:?} (expected {:?})",
            match kind {
                Kind::Mp => "mp",
                Kind::Dp => "dp",
            },
            kind.default_bunit()
        );
        return None;
    }
    Some(kind.value_scale())
}

fn parse_date_obs(text: &str) -> Option<(i64, i64, i64)> {
    let date = text.split(',').next()?.trim().trim_matches('\'');
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let (year, month, day) = if parts[0].len() == 4 {
        (
            parts[0].parse::<i64>().ok()?,
            parts[1].parse::<i64>().ok()?,
            parts[2].parse::<i64>().ok()?,
        )
    } else if parts[2].len() == 4 {
        (
            parts[2].parse::<i64>().ok()?,
            parts[0].parse::<i64>().ok()?,
            parts[1].parse::<i64>().ok()?,
        )
    } else {
        let y2 = parts[2].parse::<i64>().ok()?;
        let year = if y2 >= 50 { 1900 + y2 } else { 2000 + y2 };
        (
            year,
            parts[0].parse::<i64>().ok()?,
            parts[1].parse::<i64>().ok()?,
        )
    };
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some((year, month, day))
}

fn parse_epoch(text: &str) -> Option<(i64, f64)> {
    let text = text.trim();
    let (date, clock) = match text.split_once(['T', ' ']) {
        Some((d, t)) => (d, Some(t.trim())),
        None => (text, None),
    };
    let (year, month, day) = if date.contains('-') {
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        if parts[0].len() == 4 {
            (
                parts[0].parse().ok()?,
                parts[1].parse().ok()?,
                parts[2].parse().ok()?,
            )
        } else {
            parse_date_obs(date)?
        }
    } else if date.contains('.') {
        let parts: Vec<&str> = date.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        if parts[0].len() == 4 {
            (
                parts[0].parse().ok()?,
                parts[1].parse().ok()?,
                parts[2].parse().ok()?,
            )
        } else {
            (
                parts[2].parse().ok()?,
                parts[0].parse().ok()?,
                parts[1].parse().ok()?,
            )
        }
    } else {
        return None;
    };
    let days = days_from_civil(year, month, day)?;
    let seconds = match clock {
        Some(t) => {
            let mut it = t.split(':');
            let hh: f64 = it.next()?.parse().ok()?;
            let mm: f64 = it.next().unwrap_or("0").parse().ok()?;
            let ss: f64 = it.next().unwrap_or("0").parse().ok()?;
            if !(0.0..24.0).contains(&hh) || !(0.0..60.0).contains(&mm) {
                return None;
            }
            if !(0.0..=60.0).contains(&ss) {
                return None;
            }
            hh * 3600.0 + mm * 60.0 + ss
        }
        None => return None,
    };
    Some((days, seconds))
}

fn map_time(header: &FitsHeader, epoch: Option<(i64, f64)>) -> Option<(i64, f64)> {
    if epoch.is_some() {
        return epoch;
    }
    let (year, month, day) = parse_date_obs(&header.str_unescaped("DATE-OBS")?)?;
    let days = days_from_civil(year, month, day)?;
    let hours = header.f64("TAVG-UT")?;
    if !(0.0..24.0).contains(&hours) {
        return None;
    }
    Some((days, hours * 3600.0))
}

fn pixel_lat_lon(
    col: usize,
    row: usize,
    x0: f64,
    y0: f64,
    r0: f64,
    b0: f64,
    cenlon: f64,
) -> Option<(f64, f64)> {
    let x = (col as f64 + 1.0 - x0) / r0;
    let y = -((row as f64 + 1.0 - y0) / r0);
    let rho2 = x * x + y * y;
    if rho2 > 1.0 {
        return None;
    }
    let z = (1.0 - rho2).sqrt();
    let (sin_b0, cos_b0) = b0.sin_cos();
    let lat = (y * cos_b0 + z * sin_b0)
        .clamp(-1.0, 1.0)
        .asin()
        .to_degrees();
    let off = x.atan2(z * cos_b0 - y * sin_b0).to_degrees();
    let lon = (cenlon + off).rem_euclid(360.0);
    Some((lat, lon))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let Some(kind) = arg_value(&args, "--kind").as_deref().and_then(Kind::parse) else {
        eprintln!("--kind mp|dp required (mp = magnetogram GAUSS, dp = dopplergram KM/SEC)");
        std::process::exit(2);
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            let name = match kind {
                Kind::Mp => "mwo_mag_mp.bin",
                Kind::Dp => "mwo_mag_dp.bin",
            };
            let path = compiled(NETLOC, name);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let epoch = match arg_value(&args, "--epoch") {
        Some(v) => match parse_epoch(&v) {
            Some(e) => Some(e),
            None => {
                eprintln!("--epoch {v}: not a date (YYYY-MM-DD[THH:MM:SS])");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let Some(lsk) = load_lsk(arg_value(&args, "--lsk").as_deref()) else {
        eprintln!(
            "leap-second kernel unread — pass --lsk naif0012.tls or let the embedded copy parse"
        );
        std::process::exit(1);
    };

    let (bytes, source) = if let Some(path) = arg_value(&args, "--input") {
        match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        }
    } else if let Some(url) = arg_value(&args, "--url") {
        match fetch_raw_bytes(&url) {
            Some(b) => (b, url),
            None => {
                eprintln!("{url}: fetch void — no frame, nothing written");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("--input <MP/DP .fits> | --url <url> required");
        std::process::exit(2);
    };

    let Some((header, _)) = FitsHeader::parse(&bytes, 0) else {
        eprintln!("{source}: FITS header parses void");
        std::process::exit(1);
    };
    let Some((img, _)) = FitsImage::parse(&bytes, 0) else {
        eprintln!("{source}: FITS image parses void (truncated or unsupported BITPIX)");
        std::process::exit(1);
    };
    let (cols, rows) = (img.dims[0], img.dims[1]);
    if cols != rows {
        eprintln!("{source}: {cols}x{rows} is not a square solar disk");
        std::process::exit(1);
    }
    if img.bitpix != 16 {
        eprintln!("{source}: BITPIX {}, expected 16", img.bitpix);
        std::process::exit(1);
    }
    let Some(scale) = bunit_scale(kind, &header) else {
        std::process::exit(1);
    };
    let bscale = match header.f64("BSCALE") {
        Some(v) => v,
        None => 1.0,
    };
    let bzero = match header.f64("BZERO") {
        Some(v) => v,
        None => return None,
    };
    let blank = header
        .int("BLANK")
        .map(|b| b as f64 * bscale + bzero)
        .filter(|b| b.is_finite());
    eprintln!(
        "{source}: {cols}x{rows}, BITPIX 16, BUNIT {:?}, BSCALE {bscale}, BZERO {bzero}, BLANK {}",
        match header.str_unescaped("BUNIT") {
            Some(v) => v,
            None => "absent".to_string(),
        },
        match blank {
            Some(b) => format!("{b}"),
            None => "absent".to_string(),
        }
    );
    for key in ["TELESCOP", "LAMBDA", "DXB_IMG", "CARROT", "CENLON", "B0"] {
        if let Some(v) = header.str_unescaped(key) {
            eprintln!("{source}: {key} = {v}");
        }
    }

    let x0 = match header.f64("X0") {
        Some(v) => v,
        None => (cols as f64 + 1.0) / 2.0,
    };
    let y0 = match header.f64("Y0") {
        Some(v) => v,
        None => (rows as f64 + 1.0) / 2.0,
    };
    let r0 = match header.f64("R0") {
        Some(v) => v,
        None => cols.min(rows) as f64 / 2.0,
    }
    .max(1.0);
    let b0 = match header.f64("B0") {
        Some(v) => v,
        None => return None,
    };
    let Some(cenlon) = header
        .f64("CENLON")
        .or_else(|| arg_value(&args, "--cenlon").and_then(|v| v.parse::<f64>().ok()))
    else {
        eprintln!(
            "{source}: CENLON absent — pass --cenlon <degrees>; the longitude frame stays undefined"
        );
        std::process::exit(1);
    };
    if header.f64("CENLON").is_none() {
        eprintln!("{source}: CENLON absent in header, --cenlon {cenlon} used");
    }
    if header.f64("B0").is_none() {
        eprintln!("{source}: B0 absent in header, 0.0 rad used (measured gap)");
    }
    if header.f64("X0").is_none() || header.f64("Y0").is_none() || header.f64("R0").is_none() {
        eprintln!("{source}: X0/Y0/R0 partially absent, disk centre defaults used (measured gap)");
    }

    let Some((day, day_seconds)) = map_time(&header, epoch) else {
        eprintln!(
            "{source}: DATE-OBS/TAVG-UT absent or unreadable — pass --epoch YYYY-MM-DD[THH:MM:SS]"
        );
        std::process::exit(1);
    };
    let unix = day as f64 * SECONDS_PER_DAY + day_seconds;
    let Some(tdb) = lsk.unix_to_tdb(unix) else {
        eprintln!("{source}: leap-second kernel has no entry for unix {unix} — nothing written");
        std::process::exit(1);
    };
    eprintln!("{source}: anchor day {day} + {day_seconds:.1} s UT -> TDB {tdb:.3}");

    let mut records: Vec<GeoRec> = Vec::with_capacity(cols * rows);
    let mut blanked = 0usize;
    let mut outside = 0usize;
    let mut unreadable = 0usize;
    for row in 0..rows {
        for col in 0..cols {
            let Some((lat, lon)) = pixel_lat_lon(col, row, x0, y0, r0, b0, cenlon) else {
                outside += 1;
                continue;
            };
            let Some(v) = img.value_f64(&bytes, [col, row, 0]) else {
                unreadable += 1;
                continue;
            };
            if !v.is_finite() {
                unreadable += 1;
                continue;
            }
            if let Some(b) = blank
                && v == b
            {
                blanked += 1;
                continue;
            }
            records.push(GeoRec {
                t: tdb,
                lat,
                lon,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: v * scale,
                comp: kind.comp(),
                station: 0,
            });
        }
    }
    if records.is_empty() {
        eprintln!("{source}: no measured disk pixel — nothing written");
        std::process::exit(1);
    }
    eprintln!(
        "{source}: {} disk pixels kept, {blanked} BLANK, {outside} off-disk, {unreadable} unreadable (absent)",
        records.len()
    );

    let bytes = write_geo_bin(MAGIC_MWO_MAP, &records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_geo_bin(MAGIC_MWO_MAP, &bytes) {
        Some(parsed) if parsed.len() == records.len() => eprintln!(
            "{out}: {} geo records, roundtrip parses ({} B)",
            parsed.len(),
            bytes.len()
        ),
        _ => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(key: &str, value: &str) -> String {
        let mut s = format!("{key:<8}= {value:>20}");
        while s.len() < 80 {
            s.push(' ');
        }
        s
    }

    fn fits_fixture(values: &[i16], w: usize, h: usize, bunit: Option<&str>) -> Vec<u8> {
        let mut out = String::new();
        out.push_str(&card("SIMPLE", "T"));
        out.push_str(&card("BITPIX", "16"));
        out.push_str(&card("NAXIS", "2"));
        out.push_str(&card("NAXIS1", &w.to_string()));
        out.push_str(&card("NAXIS2", &h.to_string()));
        out.push_str(&card("BLANK", "-32768"));
        out.push_str(&card("BSCALE", "0.1"));
        out.push_str(&card("BZERO", "0.0"));
        if let Some(u) = bunit {
            out.push_str(&card("BUNIT", &format!("'{u}'")));
        }
        out.push_str(&card("DATE-OBS", "'01-01-13,   2'"));
        out.push_str(&card("TAVG-UT", "22.8830"));
        out.push_str(&card("X0", "2.5"));
        out.push_str(&card("Y0", "2.5"));
        out.push_str(&card("R0", "1.2"));
        out.push_str(&card("B0", "-0.054857"));
        out.push_str(&card("CENLON", "314.4551"));
        out.push_str(&card("CARROT", "2132.1265"));
        out.push_str(&card("END", ""));
        while out.len() % 2880 != 0 {
            out.push(' ');
        }
        let mut bytes = out.into_bytes();
        for v in values {
            bytes.extend_from_slice(&v.to_be_bytes());
        }
        bytes
    }

    #[test]
    fn date_obs_and_time_read_the_mwo_cards() {
        assert_eq!(parse_date_obs("01-01-13,   2"), Some((2013, 1, 1)));
        assert_eq!(parse_date_obs("11-14-95"), Some((1995, 11, 14)));
        assert_eq!(parse_date_obs("nonsense"), None);
        assert_eq!(
            parse_epoch("2013-01-01T22:53:00"),
            Some((days_from_civil(2013, 1, 1).unwrap(), 82380.0))
        );
        assert_eq!(
            parse_epoch("01-01-13"),
            Some((days_from_civil(2013, 1, 1).unwrap(), 0.0))
        );
    }

    #[test]
    fn bunit_is_the_header_card() {
        let mp = fits_fixture(&[0i16; 4], 2, 2, Some("GAUSS"));
        let (h, _) = FitsHeader::parse(&mp, 0).unwrap();
        assert_eq!(bunit_scale(Kind::Mp, &h), Some(1e-4));
        let dp = fits_fixture(&[0i16; 4], 2, 2, Some("KM/SEC"));
        let (h, _) = FitsHeader::parse(&dp, 0).unwrap();
        assert_eq!(bunit_scale(Kind::Dp, &h), Some(1e3));
        let absent = fits_fixture(&[0i16; 4], 2, 2, None);
        let (h, _) = FitsHeader::parse(&absent, 0).unwrap();
        assert_eq!(bunit_scale(Kind::Mp, &h), Some(1e-4));
    }

    #[test]
    fn the_disk_centre_sits_at_b0_and_the_central_meridian() {
        let (lat, lon) = pixel_lat_lon(9, 9, 10.0, 10.0, 10.0, 0.1, 314.4551).unwrap();
        assert!((lat - 0.1f64.to_degrees()).abs() < 1e-9, "lat {lat}");
        assert!((lon - 314.4551).abs() < 1e-9, "lon {lon}");
    }

    #[test]
    fn the_north_edge_is_the_north_pole_and_the_east_edge_has_positive_longitude() {
        let (lat_n, _) = pixel_lat_lon(10, 0, 10.5, 10.5, 10.0, 0.0, 0.0).unwrap();
        assert!(lat_n > 70.0, "north edge lat {lat_n}");
        let (_, lon_e) = pixel_lat_lon(19, 10, 10.5, 10.5, 10.0, 0.0, 0.0).unwrap();
        assert!(lon_e > 0.0 && lon_e < 90.0, "east lon {lon_e}");
        assert!(pixel_lat_lon(0, 0, 10.5, 10.5, 10.0, 0.0, 0.0).is_none());
    }

    #[test]
    fn blank_is_absent_and_values_are_scaled() {
        let mut values = [10i16; 16];
        values[5] = -32768;
        let bytes = fits_fixture(&values, 4, 4, Some("GAUSS"));
        let (header, _) = FitsHeader::parse(&bytes, 0).unwrap();
        let (img, _) = FitsImage::parse(&bytes, 0).unwrap();
        let bscale = header.f64("BSCALE").unwrap();
        let bzero = header.f64("BZERO").unwrap();
        let blank = header.int("BLANK").map(|b| b as f64 * bscale + bzero);
        assert_eq!(blank, Some(-3276.8));
        let kept = (0..16)
            .filter(|i| {
                let v = img.value_f64(&bytes, [i % 4, i / 4, 0]).unwrap();
                Some(v) != blank
            })
            .count();
        assert_eq!(kept, 15);
        assert_eq!(img.value_f64(&bytes, [0, 0, 0]), Some(1.0));
    }
}
