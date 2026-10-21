use omegaflow::archivar::asset_path::{compiled, raw};
use omegaflow::archivar::euvi::{comp_of_wavelength, parse_bin, write_bin};
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage};
use omegaflow::lsk::{LeapSeconds, days_from_civil, parse as parse_lsk};

const NETLOC: &str = "stereo-ssc.nascom.nasa.gov";
const DEFAULT_OUT: &str = "stereo_euvi.bin";
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
            parse_lsk(&text)
        }
        None => embedded_lsk(),
    }
}

fn is_fits_name(name: &str) -> bool {
    let low = name.to_ascii_lowercase();
    low.ends_with(".fts") || low.ends_with(".fits")
}

fn parse_date_obs(text: &str) -> Option<f64> {
    let text = text.trim().trim_matches('\'').trim();
    let text = text.strip_suffix('Z').unwrap_or(text);
    let (date, clock) = match text.split_once(['T', ' ']) {
        Some((d, t)) => (d, t.trim()),
        None => (text, "00:00:00"),
    };
    let mut d = date.split(['-', '/']);
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let mut t = clock.split(':');
    let hour: f64 = t.next()?.parse().ok()?;
    let minute: f64 = t.next().unwrap_or("0").parse().ok()?;
    let second: f64 = t.next().unwrap_or("0").parse().ok()?;
    if !(0.0..24.0).contains(&hour)
        || !(0.0..60.0).contains(&minute)
        || !(0.0..=61.0).contains(&second)
    {
        return None;
    }
    Some(days as f64 * SECONDS_PER_DAY + hour * 3600.0 + minute * 60.0 + second)
}

fn header_unit(header: &FitsHeader) -> Option<String> {
    header
        .str_unescaped("BUNIT")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn image_mean(bytes: &[u8], img: &FitsImage, header: &FitsHeader) -> Option<(f64, usize)> {
    let (w, h) = (img.dims[0], img.dims[1]);
    if w == 0 || h == 0 {
        return None;
    }
    let blank = header.int("BLANK").and_then(|b| img.pixel_value(b));
    let mut sum = 0.0f64;
    let mut n = 0usize;
    for y in 0..h {
        for x in 0..w {
            let Some(v) = img.value_f64(bytes, [x, y, 0]) else {
                continue;
            };
            if !v.is_finite() {
                continue;
            }
            if let Some(b) = blank {
                if v == b {
                    continue;
                }
            }
            sum += v;
            n += 1;
        }
    }
    if n == 0 {
        None
    } else {
        Some((sum / n as f64, n))
    }
}

struct Frame {
    source: String,
    unix: f64,
    mean: f64,
    pixels: usize,
    wavelnth: u32,
    comp: u32,
    unit: Option<String>,
}

fn read_frame(source: &str, bytes: &[u8]) -> Result<Frame, String> {
    let (header, _) =
        FitsHeader::parse(bytes, 0).ok_or_else(|| format!("{source}: FITS header parses void"))?;
    let (img, _) = FitsImage::parse(bytes, 0).ok_or_else(|| {
        format!("{source}: FITS image parses void (truncated or unsupported BITPIX)")
    })?;
    let raw_wavelnth = header
        .f64("WAVELNTH")
        .ok_or_else(|| format!("{source}: WAVELNTH absent — the passband stays unread"))?;
    let wavelnth = raw_wavelnth.round();
    if (raw_wavelnth - wavelnth).abs() > 1e-6 || wavelnth <= 0.0 {
        return Err(format!(
            "{source}: WAVELNTH {raw_wavelnth} is not a passband value"
        ));
    }
    let comp = comp_of_wavelength(wavelnth as u32).ok_or_else(|| {
        format!("{source}: WAVELNTH {wavelnth} is not one of the EUVI 171/195/284/304 bands")
    })?;
    let date = header
        .str_unescaped("DATE-OBS")
        .or_else(|| header.str_unescaped("DATE_OBS"))
        .ok_or_else(|| format!("{source}: DATE-OBS absent — the epoch stays unread"))?;
    let unix = parse_date_obs(&date)
        .ok_or_else(|| format!("{source}: DATE-OBS {date:?} does not parse"))?;
    let (mean, pixels) = image_mean(bytes, &img, &header)
        .ok_or_else(|| format!("{source}: no finite pixel — the frame stays unread"))?;
    Ok(Frame {
        source: source.to_string(),
        unix,
        mean,
        pixels,
        wavelnth: wavelnth as u32,
        comp,
        unit: header_unit(&header),
    })
}

fn hrefs(text: &str) -> Vec<String> {
    let low = text.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(p) = low[from..].find("href=\"") {
        let start = from + p + 6;
        match low[start..].find('"') {
            Some(end) => {
                out.push(text[start..start + end].to_string());
                from = start + end;
            }
            None => break,
        }
    }
    out
}

fn join_url(base: &str, link: &str) -> String {
    let link = link.split(['?', '#']).next().unwrap_or(link);
    if link.starts_with("http://") || link.starts_with("https://") {
        return link.to_string();
    }
    if link.starts_with('/') {
        let scheme_end = match base.find("://") {
            Some(i) => i + 3,
            None => return link.to_string(),
        };
        let host_end = match base[scheme_end..].find('/') {
            Some(i) => scheme_end + i,
            None => base.len(),
        };
        return format!("{}{}", &base[..host_end], link);
    }
    let dir = base.rsplit_once('/').map(|(d, _)| d).unwrap_or(base);
    format!("{dir}/{link}")
}

fn resolve_inputs(spec: &str, cache_dir: &str) -> Result<Vec<String>, String> {
    let looks_like_fits = is_fits_name(spec.split(['?', '#']).next().unwrap_or(spec));
    let is_remote = spec.starts_with("http://") || spec.starts_with("https://");
    if looks_like_fits {
        return Ok(vec![spec.to_string()]);
    }
    let html = if is_remote {
        let got = fetch_raw_bytes(spec).ok_or_else(|| format!("{spec}: fetch returned void"))?;
        String::from_utf8_lossy(&got).into_owned()
    } else {
        let path = std::path::Path::new(spec);
        if path.is_dir() {
            let mut names: Vec<String> = std::fs::read_dir(path)
                .map_err(|e| format!("read {spec} returned void: {e}"))?
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| is_fits_name(n))
                .collect();
            names.sort();
            return Ok(names
                .into_iter()
                .map(|n| format!("{}/{}", spec.trim_end_matches('/'), n))
                .collect());
        }
        return Err(format!("{spec}: neither a FITS file nor a directory"));
    };
    let mut out: Vec<String> = Vec::new();
    for link in hrefs(&html) {
        let name = link.split(['?', '#']).next().unwrap_or(&link);
        if !is_fits_name(name) {
            continue;
        }
        let url = if is_remote {
            join_url(spec, &link)
        } else {
            link
        };
        if !out.contains(&url) {
            out.push(url);
        }
    }
    if out.is_empty() {
        return Err(format!("{spec}: index carries no .fts/.fits link"));
    }
    let _ = cache_dir;
    Ok(out)
}

fn fetch_frame(spec: &str, cache_dir: &str) -> Option<Vec<u8>> {
    if spec.starts_with("http://") || spec.starts_with("https://") {
        let name = spec.rsplit('/').next().unwrap_or("frame.fts");
        let cache_path = format!("{cache_dir}/{name}");
        if let Ok(b) = std::fs::read(&cache_path) {
            return Some(b);
        }
        let got = fetch_raw_bytes(spec);
        if let Some(b) = &got {
            let _ = std::fs::create_dir_all(cache_dir);
            let _ = std::fs::write(&cache_path, b);
        }
        got
    } else {
        std::fs::read(spec).ok()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || has_flag(&args, "--help") || has_flag(&args, "-h") {
        eprintln!("usage: euvi_compiler (--in <file|dir> | --url <fits|index> | --index <index>)");
        eprintln!("       [--out-bin <path>] [--cache-dir <dir>] [--lsk <naif0012.tls>]");
        eprintln!("       [--limit N] [--probe] [--ci-mode]");
        eprintln!("  record: TDB seconds (J2000) + mean of finite pixels + WAVELNTH band");
        eprintln!("  unit:   FITS BUNIT (measured; no substitute is invented)");
        std::process::exit(1);
    }
    let ci_mode = has_flag(&args, "--ci-mode");
    let probe = has_flag(&args, "--probe");
    let out = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => {
            let path = compiled(NETLOC, DEFAULT_OUT);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let cache_dir = match arg_value(&args, "--cache-dir") {
        Some(v) => v,
        None => raw(NETLOC, "cache/euvi").to_string_lossy().into_owned(),
    };
    let _ = std::fs::create_dir_all(&cache_dir);

    let spec = match (
        arg_value(&args, "--in"),
        arg_value(&args, "--url"),
        arg_value(&args, "--index"),
    ) {
        (Some(v), _, _) | (_, Some(v), _) | (_, _, Some(v)) => v,
        _ => {
            eprintln!("euvi_compiler: --in, --url or --index absent — nothing read");
            std::process::exit(1);
        }
    };
    let Some(lsk) = load_lsk(arg_value(&args, "--lsk").as_deref()) else {
        eprintln!("euvi_compiler: leap-second kernel stays unread — no fabricated epoch");
        std::process::exit(1);
    };
    let limit = arg_value(&args, "--limit").and_then(|v| v.parse::<usize>().ok());
    let mut sources = match resolve_inputs(&spec, &cache_dir) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("euvi_compiler: {e}");
            std::process::exit(1);
        }
    };
    if let Some(n) = limit {
        sources.truncate(n);
    }
    eprintln!("EUVI: {} FITS source(s) from {spec}", sources.len());

    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    let mut unit: Option<Option<String>> = None;
    let mut skipped = 0usize;
    for source in &sources {
        let Some(bytes) = fetch_frame(source, &cache_dir) else {
            eprintln!("{source}: fetch/read returned void");
            skipped += 1;
            continue;
        };
        let frame = match read_frame(source, &bytes) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{e}");
                skipped += 1;
                continue;
            }
        };
        if unit.is_none() {
            unit = Some(frame.unit.clone());
        } else if unit.as_ref() != Some(&frame.unit) {
            eprintln!(
                "{}: BUNIT {:?} differs from {:?} — the frame stays out of this asset",
                frame.source,
                frame.unit,
                unit.as_ref().and_then(|u| u.clone())
            );
            skipped += 1;
            continue;
        }
        let Some(tdb) = lsk.unix_to_tdb(frame.unix) else {
            eprintln!(
                "{}: leap-second kernel has no entry for unix {}",
                frame.source, frame.unix
            );
            skipped += 1;
            continue;
        };
        eprintln!(
            "{}: WAVELNTH {} BUNIT {} DATE-OBS unix {:.3} -> TDB {:.3}, {} px mean {:.6}",
            frame.source,
            frame.wavelnth,
            frame.unit.as_deref().unwrap_or("absent"),
            frame.unix,
            tdb,
            frame.pixels,
            frame.mean
        );
        records.push((tdb, frame.mean, frame.comp));
    }

    match unit.as_ref().and_then(|u| u.clone()) {
        Some(u) => eprintln!("EUVI: BUNIT {u:?} (measured in every accepted frame)"),
        None => eprintln!("EUVI: BUNIT absent in every accepted frame — unit stays 1"),
    }
    eprintln!("EUVI: {} records, {} skipped", records.len(), skipped);
    if records.is_empty() {
        eprintln!("euvi_compiler: no frame measured — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    let bytes = write_bin(&records);
    if probe {
        eprintln!(
            "--probe: {} B would be written, nothing written",
            bytes.len()
        );
        return;
    }
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("euvi_compiler: write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!(
            "{out}: {} records, roundtrip parses ({} B)",
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("{out}: CDN upload returned void");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(key: &str, value: &str) -> String {
        let mut s = format!("{key:<8}= {value}");
        assert!(s.len() <= 80, "card {key} overflows 80 bytes: {s}");
        while s.len() < 80 {
            s.push(' ');
        }
        s
    }

    fn fits_fixture(values: &[i16], w: usize, h: usize, wavelnth: &str) -> Vec<u8> {
        let mut out = String::new();
        out.push_str(&card("SIMPLE", "T"));
        out.push_str(&card("BITPIX", "16"));
        out.push_str(&card("NAXIS", "2"));
        out.push_str(&card("NAXIS1", &w.to_string()));
        out.push_str(&card("NAXIS2", &h.to_string()));
        out.push_str(&card("DATE-OBS", "'2011-04-17T00:15:30.006'"));
        out.push_str(&card("WAVELNTH", wavelnth));
        out.push_str(&card("BUNIT", "'DN'"));
        out.push_str(&card("BSCALE", "1.0"));
        out.push_str(&card("BZERO", "32768"));
        out.push_str(&card("BLANK", "-32768"));
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
    fn date_obs_carries_the_fractional_second() {
        let unix = parse_date_obs("2011-04-17T00:15:30.006").unwrap();
        let day = days_from_civil(2011, 4, 17).unwrap() as f64 * 86400.0;
        assert!((unix - (day + 15.0 * 60.0 + 30.006)).abs() < 1e-6);
    }

    #[test]
    fn frame_reads_header_band_unit_and_mean_skipping_blank() {
        let values = [-31768i16, -30768, -32768, -29768, -28768, -32768];
        let bytes = fits_fixture(&values, 3, 2, "195");
        let frame = read_frame("fixture.fts", &bytes).unwrap();
        assert_eq!(frame.wavelnth, 195);
        assert_eq!(frame.comp, 1);
        assert_eq!(frame.unit.as_deref(), Some("DN"));
        assert_eq!(frame.pixels, 4);
        assert!((frame.mean - 2500.0).abs() < 1e-9, "mean {}", frame.mean);
    }

    #[test]
    fn frame_refuses_an_unknown_wavelnth_instead_of_guessing() {
        let unknown = fits_fixture(&[1i16; 4], 2, 2, "193");
        assert!(read_frame("x.fts", &unknown).is_err());
        let band_284 = fits_fixture(&[1i16; 4], 2, 2, "284");
        assert_eq!(read_frame("x.fts", &band_284).unwrap().comp, 2);
    }

    #[test]
    fn index_keeps_fits_links_and_resolves_them_against_the_base() {
        let html = r#"
            <a href="20110417_000530_n4euA.fts">FITS</a>
            <a href="20110417_001530_n7euA.fts">FITS</a>
            <a href="thumbnail.png">PNG</a>
            <a href="20110417_000530_n4euA.fts">dup</a>
        "#;
        let mut links: Vec<String> = Vec::new();
        for link in hrefs(html) {
            let name = link.split(['?', '#']).next().unwrap_or(&link);
            if !is_fits_name(name) {
                continue;
            }
            let url = join_url(
                "https://stereo-ssc.nascom.nasa.gov/pub/ins_data/secchi/L0_YMD/a/img/euvi/2011/04/17/",
                &link,
            );
            if !links.contains(&url) {
                links.push(url);
            }
        }
        assert_eq!(links.len(), 2);
        assert!(links[0].ends_with("/2011/04/17/20110417_000530_n4euA.fts"));
    }
}
