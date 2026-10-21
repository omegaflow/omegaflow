use omegaflow::archivar::asset_path::compiled;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    COMP_KPVT_MAP_VALUE, GeoRec, MAGIC_KPVT_MAP, parse_bin as parse_geo_bin,
    write_bin as write_geo_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage};
use omegaflow::lsk::{LeapSeconds, days_from_civil};

const NETLOC: &str = "nispdata.nso.edu";
const DEFAULT_OUT: &str = "kpvt_synoptic_grid.bin";
const BASE: &str = "https://nispdata.nso.edu/ftp/kpvt/synoptic/mag/";
const MAP_COLS: usize = 360;
const MAP_ROWS: usize = 180;
const SECONDS_PER_DAY: f64 = 86400.0;

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

fn parse_epoch(text: &str) -> Option<(i64, f64)> {
    let text = text.trim().trim_matches('\'').trim();
    let text = text.split(',').next()?.trim();
    let (date, clock) = match text.split_once(['T', ' ', '_']) {
        Some((d, t)) => (d, Some(t.trim())),
        None => (text, None),
    };
    let sep = if date.contains('-') {
        '-'
    } else if date.contains('.') {
        '.'
    } else if date.contains('/') {
        '/'
    } else {
        return None;
    };
    let parts: Vec<&str> = date.split(sep).collect();
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
            parts[1].parse::<i64>().ok()?,
            parts[0].parse::<i64>().ok()?,
        )
    } else {
        let y2 = parts[2].parse::<i64>().ok()?;
        let year = if y2 >= 50 { 1900 + y2 } else { 2000 + y2 };
        (
            year,
            parts[1].parse::<i64>().ok()?,
            parts[0].parse::<i64>().ok()?,
        )
    };
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
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

fn cr_from_path(path: &str) -> Option<i64> {
    let name = path.rsplit('/').next()?;
    let rest = name.strip_prefix('m')?;
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

fn map_day(header: &FitsHeader, epoch: Option<(i64, f64)>) -> Option<(i64, f64)> {
    if epoch.is_some() {
        return epoch;
    }
    for key in ["T_OBS", "DATE-OBS", "DATE"] {
        if let Some(s) = header.str_unescaped(key)
            && let Some(parsed) = parse_epoch(&s)
        {
            return Some(parsed);
        }
    }
    None
}

fn pixel_lat_lon(x: usize, y: usize, cols: usize, rows: usize) -> (f64, f64) {
    let sin_lat = -1.0 + 2.0 * (y as f64 + 0.5) / rows as f64;
    let lat = sin_lat.clamp(-1.0, 1.0).asin().to_degrees();
    let lon = (x as f64 + 0.5) * 360.0 / cols as f64;
    (lat, lon)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            let path = compiled(NETLOC, DEFAULT_OUT);
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

    let input = arg_value(&args, "--input");
    let url = arg_value(&args, "--url");
    let cr_arg = arg_value(&args, "--cr");
    let (bytes, source) = if let Some(path) = input {
        match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        }
    } else if let Some(url) = url {
        match fetch_raw_bytes(&url) {
            Some(b) => (b, url),
            None => {
                eprintln!("{url}: fetch void — no map, nothing written");
                std::process::exit(1);
            }
        }
    } else if let Some(cr) = cr_arg {
        let url = format!("{BASE}m{cr}f.fits");
        match fetch_raw_bytes(&url) {
            Some(b) => (b, url),
            None => {
                eprintln!("{url}: fetch void — no map, nothing written");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("--input <m<CR>f.fits> | --url <url> | --cr <carrington> required");
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
    if cols != MAP_COLS || rows != MAP_ROWS {
        eprintln!("{source}: {cols}x{rows}, expected the KPVT synoptic {MAP_COLS}x{MAP_ROWS}");
        std::process::exit(1);
    }
    if img.bitpix != 16 {
        eprintln!(
            "{source}: BITPIX {}, expected 16 (BSCALE 0.1 integer map)",
            img.bitpix
        );
        std::process::exit(1);
    }
    let bscale = header.f64("BSCALE").unwrap_or(1.0);
    let bzero = match header.f64("BZERO") {
        Some(v) => v,
        None => return None,
    };
    let blank = header
        .int("BLANK")
        .map(|b| b as f64 * bscale + bzero)
        .filter(|b| b.is_finite());
    let bunit = header.str_unescaped("BUNIT");
    match &bunit {
        Some(u) => eprintln!("{source}: BUNIT {u:?} (measured header card)"),
        None => eprintln!(
            "{source}: BUNIT absent in header — value kept in unit 1; no Gauss label is invented"
        ),
    }
    eprintln!(
        "{source}: BITPIX 16, {cols}x{rows}, BSCALE {bscale}, BZERO {bzero}, BLANK {}",
        match blank {
            Some(b) => format!("{b}"),
            None => "absent".to_string(),
        }
    );

    let cr = cr_from_path(&source);
    let Some((day, day_seconds)) = map_day(&header, epoch) else {
        eprintln!(
            "{source}: header carries neither DATE nor T_OBS — pass --epoch YYYY-MM-DD; the CR->TDB map is not asserted here"
        );
        std::process::exit(1);
    };
    let unix = day as f64 * SECONDS_PER_DAY + day_seconds;
    let Some(tdb) = lsk.unix_to_tdb(unix) else {
        eprintln!("{source}: leap-second kernel has no entry for unix {unix} — nothing written");
        std::process::exit(1);
    };
    match cr {
        Some(cr) => {
            eprintln!("{source}: CR {cr}, anchor day {day} + {day_seconds:.0} s UT -> TDB {tdb:.3}")
        }
        None => eprintln!(
            "{source}: no m<CR>f stem — anchor day {day} + {day_seconds:.0} s UT -> TDB {tdb:.3}"
        ),
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut records: Vec<GeoRec> = Vec::with_capacity(cols * rows);
    let mut blanked = 0usize;
    let mut non_finite = 0usize;
    for y in 0..rows {
        for x in 0..cols {
            let Some(v) = img.value_f64(&bytes, [x, y, 0]) else {
                non_finite += 1;
                continue;
            };
            if !v.is_finite() {
                non_finite += 1;
                continue;
            }
            if let Some(b) = blank
                && v == b
            {
                blanked += 1;
                continue;
            }
            let (lat, lon) = pixel_lat_lon(x, y, cols, rows);
            records.push(GeoRec {
                t: tdb,
                lat,
                lon,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: v,
                comp: COMP_KPVT_MAP_VALUE,
                station: 0,
            });
        }
    }
    if records.is_empty() {
        eprintln!("{source}: no finite pixel — nothing written");
        std::process::exit(1);
    }
    eprintln!(
        "{source}: {} of {} pixels kept, {} BLANK, {} unreadable (absent)",
        records.len(),
        cols * rows,
        blanked,
        non_finite
    );

    let bytes = write_geo_bin(MAGIC_KPVT_MAP, &records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_geo_bin(MAGIC_KPVT_MAP, &bytes) {
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

    fn fits_fixture(values: &[i16], w: usize, h: usize) -> Vec<u8> {
        let mut out = String::new();
        out.push_str(&card("SIMPLE", "T"));
        out.push_str(&card("BITPIX", "16"));
        out.push_str(&card("NAXIS", "2"));
        out.push_str(&card("NAXIS1", &w.to_string()));
        out.push_str(&card("NAXIS2", &h.to_string()));
        out.push_str(&card("BSCALE", "0.1"));
        out.push_str(&card("BZERO", "0.0"));
        out.push_str(&card("DATE", "'22-02-96'"));
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
    fn epoch_reads_the_kpvt_date_card() {
        assert_eq!(
            parse_epoch("22-02-96"),
            Some((days_from_civil(1996, 2, 22).unwrap(), 0.0))
        );
        assert_eq!(
            parse_epoch("1996-02-22"),
            Some((days_from_civil(1996, 2, 22).unwrap(), 0.0))
        );
        assert_eq!(
            parse_epoch("1996.02.22_12:00:00"),
            Some((days_from_civil(1996, 2, 22).unwrap(), 43200.0))
        );
        assert_eq!(
            parse_epoch("22-02-1996"),
            Some((days_from_civil(1996, 2, 22).unwrap(), 0.0))
        );
        assert_eq!(parse_epoch("nonsense"), None);
    }

    #[test]
    fn cr_stem_is_only_a_label() {
        assert_eq!(cr_from_path("/tmp/m1625f.fits"), Some(1625));
        assert_eq!(cr_from_path("https://x/m2007f.fits"), Some(2007));
        assert_eq!(cr_from_path("h1625f.fits"), None);
    }

    #[test]
    fn the_first_row_is_the_south_pole_and_the_first_column_is_lon_zero() {
        let (lat0, lon0) = pixel_lat_lon(0, 0, 360, 180);
        let (lat1, _) = pixel_lat_lon(0, 179, 360, 180);
        assert!(lat0 < -83.0 && lat0 > -84.0, "lat0 {lat0}");
        assert!(lat1 > 83.0 && lat1 < 84.0, "lat1 {lat1}");
        assert!((lon0 - 0.5).abs() < 1e-12, "lon0 {lon0}");
        let (_, lon_mid) = pixel_lat_lon(180, 90, 360, 180);
        assert!((lon_mid - 180.5).abs() < 1e-12, "lon_mid {lon_mid}");
    }

    #[test]
    fn the_integer_map_is_scaled_and_keeps_the_row_order() {
        let values = [10i16, 20, -10, -20];
        let bytes = fits_fixture(&values, 2, 2);
        let (header, _) = FitsHeader::parse(&bytes, 0).unwrap();
        let (img, _) = FitsImage::parse(&bytes, 0).unwrap();
        assert_eq!(img.bitpix, 16);
        assert_eq!(img.dims[0], 2);
        assert_eq!(img.dims[1], 2);
        assert_eq!(header.f64("BSCALE"), Some(0.1));
        assert_eq!(img.value_f64(&bytes, [0, 0, 0]), Some(1.0));
        assert_eq!(img.value_f64(&bytes, [1, 1, 0]), Some(-2.0));
        assert_eq!(header.str_unescaped("BUNIT"), None);
    }
}
