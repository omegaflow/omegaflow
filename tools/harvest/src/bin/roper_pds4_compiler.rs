use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::gras_2c;
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::pds4::{XElem, parse_xml};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const DEFAULT_NETLOC: &str = "zenodo.org";
const ASSET_PREFIX: &str = "gras_2c_roper_";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_or_read(spec: &str) -> Option<Vec<u8>> {
    if spec.starts_with("http://") || spec.starts_with("https://") {
        fetch_raw_bytes(spec)
    } else {
        std::fs::read(spec).ok()
    }
}

fn local_name(name: &str) -> &str {
    name.rsplit_once(':').map_or(name, |(_, l)| l)
}

fn child<'a>(elem: &'a XElem, name: &str) -> Option<&'a XElem> {
    elem.children.iter().find(|c| local_name(&c.name) == name)
}

fn child_text(elem: &XElem, name: &str) -> Option<String> {
    child(elem, name)
        .map(|c| c.text.trim().to_string())
        .filter(|t| !t.is_empty())
}

struct GroupField {
    location: usize,
    bytes: usize,
}

struct Group {
    repetitions: usize,
    location: usize,
    length: usize,
    fields: Vec<GroupField>,
}

fn parse_group(record: &XElem) -> Option<Group> {
    let g = child(record, "Group_Field_Binary")?;
    let repetitions = child_text(g, "repetitions")?.parse().ok()?;
    let location = child_text(g, "group_location")?.parse().ok()?;
    let length = child_text(g, "group_length")?.parse().ok()?;
    let mut fields = Vec::new();
    for f in g
        .children
        .iter()
        .filter(|c| local_name(&c.name) == "Field_Binary")
    {
        let location = child_text(f, "field_location")?.parse().ok()?;
        let bytes = child_text(f, "field_length")?.parse().ok()?;
        fields.push(GroupField { location, bytes });
    }
    if fields.len() < 2 {
        return None;
    }
    Some(Group {
        repetitions,
        location,
        length,
        fields,
    })
}

fn iso_unix(s: &str) -> Option<f64> {
    let (date, rest) = s.split_once('T')?;
    let rest = rest.trim_end_matches('Z');
    let mut d = date.split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    let days = days_from_civil(y, mo, da)?;
    let mut hms = rest.split(':');
    let h: f64 = hms.next()?.parse().ok()?;
    let mi: f64 = hms.next()?.parse().ok()?;
    let se: f64 = hms.next()?.parse().ok()?;
    Some(days as f64 * 86400.0 + h * 3600.0 + mi * 60.0 + se)
}

fn decode_f32(bytes: &[u8], at: usize) -> Option<f64> {
    let v = f32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?) as f64;
    v.is_finite().then_some(v)
}

fn asset_name(dat_spec: &str) -> String {
    let no_query = dat_spec.split_once('?').map(|(h, _)| h).unwrap_or(dat_spec);
    let segs: Vec<&str> = no_query.rsplit('/').collect();
    let last = segs.first().copied().unwrap_or(dat_spec);
    let base = if last == "content" {
        segs.get(1).copied().unwrap_or(last)
    } else {
        last
    };
    let stem = base.split('.').next().unwrap_or(base);
    format!("{ASSET_PREFIX}{}.bin", stem.to_ascii_lowercase())
}

fn compile(
    label_spec: &str,
    dat_spec: Option<&str>,
    out_dir: Option<&str>,
    netloc: &str,
    ci_mode: bool,
) -> Option<String> {
    let label_bytes = fetch_or_read(label_spec)?;
    let label_text = std::str::from_utf8(&label_bytes).ok()?;
    let doc = parse_xml(label_text)?;
    let fao = child(&doc, "File_Area_Observational")?;
    let table = child(fao, "Table_Binary")?;
    let record = child(table, "Record_Binary")?;
    let record_length: usize = child_text(record, "record_length")?.parse().ok()?;
    let group = parse_group(record)?;
    if group.length == 0 || group.repetitions == 0 || group.length % group.repetitions != 0 {
        eprintln!("group layout void — the radargram stays unwritten (0 honored)");
        return None;
    }
    let substride = group.length / group.repetitions;
    let file = child(fao, "File")?;
    let file_name = child_text(file, "file_name")?;
    let records: usize = child_text(file, "records")
        .or_else(|| child_text(table, "records"))?
        .parse()
        .ok()?;

    let dat = match dat_spec {
        Some(d) => d.to_string(),
        None => match label_spec.rsplit_once('/') {
            Some((dir, _)) => format!("{dir}/{file_name}"),
            None => file_name.clone(),
        },
    };
    let dat_bytes = fetch_or_read(&dat)?;
    if dat_bytes.len() < records * record_length {
        eprintln!(
            "{dat}: {} byte(s) carry {} record(s) of {record_length} — the radargram stays unwritten (0 honored)",
            dat_bytes.len(),
            records
        );
        return None;
    }

    let (start, stop) =
        match child(&doc, "Observation_Area").and_then(|o| child(o, "Time_Coordinates")) {
            Some(tc) => (
                child_text(tc, "start_date_time"),
                child_text(tc, "stop_date_time"),
            ),
            None => (None, None),
        };
    let start_unix = start.as_deref().and_then(iso_unix);
    let stop_unix = stop.as_deref().and_then(iso_unix);
    let cadence = match (start_unix, stop_unix) {
        (Some(a), Some(b)) if records > 1 && b > a => Some((b - a) / (records as f64 - 1.0)),
        _ => None,
    };

    let r_loc = group.fields[0].location.saturating_sub(1);
    let i_loc = group.fields[1].location.saturating_sub(1);
    if r_loc + group.fields[0].bytes > substride || i_loc + group.fields[1].bytes > substride {
        eprintln!(
            "group sub-fields overflow one repetition — the radargram stays unwritten (0 honored)"
        );
        return None;
    }
    let group_start = group.location.saturating_sub(1);
    let mut series: Vec<(f64, f64, u32)> = Vec::with_capacity(records * group.repetitions);
    for i in 0..records {
        let base = i * record_length;
        let t = match (start_unix, cadence) {
            (Some(s), Some(c)) => s + i as f64 * c,
            _ => i as f64,
        };
        for k in 0..group.repetitions {
            let at = base + group_start + k * substride;
            let Some(r) = decode_f32(&dat_bytes, at + r_loc) else {
                continue;
            };
            let Some(im) = decode_f32(&dat_bytes, at + i_loc) else {
                continue;
            };
            let amp = (r * r + im * im).sqrt();
            if amp.is_finite() {
                series.push((t, amp, k as u32));
            }
        }
    }
    if series.is_empty() {
        eprintln!("{dat}: no radargram bin survived — the series stays unwritten (0 honored)");
        return None;
    }
    let bin = gras_2c::write_bin(&series);
    let Some(parsed) = gras_2c::parse_series(&bin) else {
        eprintln!("{dat}: packed read void — the series stays unverified (0 honored)");
        return None;
    };
    if parsed != series {
        eprintln!("{dat}: roundtrip void — the series stays unverified (0 honored)");
        return None;
    }
    let asset = asset_name(&dat);
    let out_path = match out_dir {
        Some(dir) => format!("{}/{asset}", dir.trim_end_matches('/')),
        None => format!("data/{netloc}/gras_2c/{asset}"),
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("write {out_path} returned void");
        return None;
    }
    let basis = match (start_unix, cadence) {
        (Some(_), Some(c)) => format!("label window, cadence {c:.3} s/trace"),
        _ => "row index".to_string(),
    };
    eprintln!(
        "{out_path}: {} bin(s) packed, {} byte(s), sha256 {}, roundtrip holds; {records} trace(s) × {} bin(s); epoch basis {basis}",
        series.len(),
        bin.len(),
        sha256_hex(&bin),
        group.repetitions,
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{netloc}/{asset}");
    println!("format gras_2c");
    println!("ttl 604800");
    println!();
    if ci_mode && !upload_release(netloc, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
        return None;
    }
    Some(asset)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let label = match arg_value(&args, "--label") {
        Some(v) => v,
        None => {
            eprintln!("roper_pds4_compiler: --label <url|path> is mandatory (0 honored)");
            std::process::exit(2);
        }
    };
    let dat = arg_value(&args, "--dat");
    let out = arg_value(&args, "--out");
    let netloc = match arg_value(&args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => DEFAULT_NETLOC.to_string(),
    };
    match compile(&label, dat.as_deref(), out.as_deref(), &netloc, ci_mode) {
        Some(asset) => eprintln!("{asset}: compiled"),
        None => std::process::exit(1),
    }
}
