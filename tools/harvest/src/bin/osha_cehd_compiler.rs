use omegaflow::archivar::quaoar_occlt::{ZipEntry, zip_entries, zip_extract};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::zcta;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const NETLOC: &str = "obis.osha.gov";
const ZIP_URL: &str = "https://www.osha.gov/sites/default/files/healthsamples.zip";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120 Safari/537.36";
const FORMAT: &str = "osha_cehd_si_axis_value_text";
const MEMBER: &str = "healthsamples/sample_data_2019.csv";

const DATE_FIELD: &str = "DATE_SAMPLED";
const RESULT_FIELD: &str = "SAMPLE_RESULT";
const UNIT_FIELD: &str = "UNIT_OF_MEASUREMENT";
const QUALIFIER_FIELD: &str = "QUALIFIER";
const SUBSTANCE_FIELD: &str = "SUBSTANCE";
const SIC_FIELD: &str = "SIC_CODE";
const NAICS_FIELD: &str = "NAICS_CODE";
const ZIP_FIELD: &str = "ZIP_CODE";
const GAZETTEER_ASSET: &str = "zcta_gazetteer.bin";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("900")
        .arg("--user-agent")
        .arg(USER_AGENT)
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: reads no bytes — {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    if out.stdout.is_empty() {
        return Err(format!("{url}: carries no bytes"));
    }
    Ok(out.stdout)
}

fn read_bytes(source: &str) -> Result<Vec<u8>, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        fetch(source)
    } else {
        std::fs::read(source).map_err(|e| format!("read {source} returned void: {e}"))
    }
}

fn split_csv(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for c in line.chars() {
        match c {
            '"' if in_quotes => in_quotes = false,
            '"' => in_quotes = true,
            ',' if !in_quotes => {
                fields.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    fields.push(cur.trim().to_string());
    fields
}

fn header_fields(header: &str) -> Vec<String> {
    let line = match header.strip_prefix('\u{feff}') {
        Some(s) => s,
        None => header,
    };
    split_csv(line)
}

fn field_index(fields: &[String], name: &str) -> Option<usize> {
    fields.iter().position(|f| f == name)
}

fn require_field(fields: &[String], name: &str) -> Result<(), String> {
    match field_index(fields, name) {
        Some(_) => Ok(()),
        None => Err(format!("the CEHD table carries no {name} column")),
    }
}

fn month_number(name: &str) -> Option<i64> {
    match name.to_ascii_uppercase().as_str() {
        "JAN" => Some(1),
        "FEB" => Some(2),
        "MAR" => Some(3),
        "APR" => Some(4),
        "MAY" => Some(5),
        "JUN" => Some(6),
        "JUL" => Some(7),
        "AUG" => Some(8),
        "SEP" => Some(9),
        "OCT" => Some(10),
        "NOV" => Some(11),
        "DEC" => Some(12),
        _ => None,
    }
}

fn epoch_of_date(s: &str) -> Option<f64> {
    let mut parts = s.trim().split('-');
    let year: i64 = parts.next()?.parse().ok()?;
    let month = month_number(parts.next()?)?;
    let day: i64 = parts.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0)
}

fn finite_result(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    if v.is_finite() && v >= 0.0 {
        Some(v)
    } else {
        None
    }
}

fn censored(qualifier: &str) -> bool {
    let q = qualifier.trim().to_ascii_uppercase();
    q == "ND" || q == "BLK"
}

#[derive(Clone, Copy)]
enum SiQuantity {
    MassConcentration,
    Mass,
    AmountFraction,
    NumberDensity,
    Fraction,
}

impl SiQuantity {
    fn label(self) -> &'static str {
        match self {
            SiQuantity::MassConcentration => "kg/m3",
            SiQuantity::Mass => "kg",
            SiQuantity::AmountFraction => "1",
            SiQuantity::NumberDensity => "1/m3",
            SiQuantity::Fraction => "1",
        }
    }
}

#[derive(Clone, Copy)]
struct SiUnit {
    quantity: SiQuantity,
    factor: f64,
}

fn si_unit(s: &str) -> Option<SiUnit> {
    match s.trim().to_ascii_uppercase().as_str() {
        "M" => Some(SiUnit {
            quantity: SiQuantity::MassConcentration,
            factor: 1e-6,
        }),
        "MCG/M3" => Some(SiUnit {
            quantity: SiQuantity::MassConcentration,
            factor: 1e-9,
        }),
        "X" => Some(SiUnit {
            quantity: SiQuantity::Mass,
            factor: 1e-9,
        }),
        "Y" => Some(SiUnit {
            quantity: SiQuantity::Mass,
            factor: 1e-6,
        }),
        "P" => Some(SiUnit {
            quantity: SiQuantity::AmountFraction,
            factor: 1e-6,
        }),
        "F" => Some(SiUnit {
            quantity: SiQuantity::NumberDensity,
            factor: 1e6,
        }),
        "%" => Some(SiUnit {
            quantity: SiQuantity::Fraction,
            factor: 1e-2,
        }),
        _ => None,
    }
}

fn to_si(value: f64, unit: &SiUnit) -> f64 {
    value * unit.factor
}

struct Sample {
    unix: f64,
    value: f64,
    quantity: SiQuantity,
    pos: Option<(f64, f64)>,
}

struct ParseCounts {
    rows: usize,
    carried: usize,
    date_absent: usize,
    result_absent: usize,
    censored_absent: usize,
    unit_absent: usize,
    geocoded: usize,
    zip_absent: usize,
}

fn parse_cehd(
    text: &str,
    stride: usize,
    limit: usize,
    gaz: &[zcta::Zcta],
) -> Result<(Vec<Sample>, ParseCounts), String> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = lines
        .next()
        .ok_or_else(|| "the CEHD table carries no header".to_string())?;
    let fields = header_fields(header);
    let date_idx = field_index(&fields, DATE_FIELD)
        .ok_or_else(|| format!("the CEHD table carries no {DATE_FIELD} column"))?;
    let result_idx = field_index(&fields, RESULT_FIELD)
        .ok_or_else(|| format!("the CEHD table carries no {RESULT_FIELD} column"))?;
    let unit_idx = field_index(&fields, UNIT_FIELD)
        .ok_or_else(|| format!("the CEHD table carries no {UNIT_FIELD} column"))?;
    let qualifier_idx = field_index(&fields, QUALIFIER_FIELD)
        .ok_or_else(|| format!("the CEHD table carries no {QUALIFIER_FIELD} column"))?;
    require_field(&fields, SUBSTANCE_FIELD)?;
    require_field(&fields, SIC_FIELD)?;
    require_field(&fields, NAICS_FIELD)?;
    let zip_idx = field_index(&fields, ZIP_FIELD);

    let mut samples: Vec<Sample> = Vec::new();
    let mut counts = ParseCounts {
        rows: 0,
        carried: 0,
        date_absent: 0,
        result_absent: 0,
        censored_absent: 0,
        unit_absent: 0,
        geocoded: 0,
        zip_absent: 0,
    };
    for (row, line) in lines.enumerate() {
        counts.rows += 1;
        if row % stride != 0 {
            continue;
        }
        let cells = split_csv(line);
        let Some(unix) = cells.get(date_idx).and_then(|s| epoch_of_date(s)) else {
            counts.date_absent += 1;
            continue;
        };
        let qualifier = match cells.get(qualifier_idx) {
            Some(q) => q.as_str(),
            None => "",
        };
        if censored(qualifier) {
            counts.censored_absent += 1;
            continue;
        }
        let Some(unit) = cells.get(unit_idx).and_then(|s| si_unit(s)) else {
            counts.unit_absent += 1;
            continue;
        };
        let Some(result) = cells.get(result_idx).and_then(|s| finite_result(s)) else {
            counts.result_absent += 1;
            continue;
        };
        let pos = zip_idx
            .and_then(|i| cells.get(i))
            .and_then(|z| zcta::lookup(gaz, z));
        if pos.is_some() {
            counts.geocoded += 1;
        } else {
            counts.zip_absent += 1;
        }
        samples.push(Sample {
            unix,
            value: to_si(result, &unit),
            quantity: unit.quantity,
            pos,
        });
        counts.carried += 1;
        if samples.len() >= limit {
            break;
        }
    }
    Ok((samples, counts))
}

fn leaf_text<'a>(s: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = s.find(&open)? + open.len();
    let end = s[start..].find(&close)? + start;
    Some(s[start..end].trim())
}

fn is_open(rest: &str) -> bool {
    matches!(
        rest.as_bytes().first(),
        Some(b'>') | Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')
    )
}

fn decode_entity(entity: &str) -> Option<char> {
    if let Some(num) = entity.strip_prefix('#') {
        let code = match num.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok()?,
            None => num.parse::<u32>().ok()?,
        };
        char::from_u32(code)
    } else {
        match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => None,
        }
    }
}

fn decode_entities(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        let semi = rest.find(';');
        match semi {
            Some(semi) => match decode_entity(&rest[1..semi]) {
                Some(c) => {
                    out.push(c);
                    rest = &rest[semi + 1..];
                }
                None => {
                    out.push('&');
                    rest = &rest[1..];
                }
            },
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn xml_field(record: &str, tag: &str) -> Option<String> {
    leaf_text(record, tag).map(decode_entities)
}

fn parse_cehd_xml(
    text: &str,
    stride: usize,
    limit: usize,
    gaz: &[zcta::Zcta],
) -> Result<(Vec<Sample>, ParseCounts), String> {
    let mut samples: Vec<Sample> = Vec::new();
    let mut counts = ParseCounts {
        rows: 0,
        carried: 0,
        date_absent: 0,
        result_absent: 0,
        censored_absent: 0,
        unit_absent: 0,
        geocoded: 0,
        zip_absent: 0,
    };
    for record in text.split("<DATA_RECORD").skip(1) {
        if !is_open(record) {
            continue;
        }
        let body = match record.find("</DATA_RECORD>") {
            Some(end) => &record[..end],
            None => continue,
        };
        counts.rows += 1;
        if (counts.rows - 1) % stride != 0 {
            continue;
        }
        let Some(unix) = xml_field(body, "date_sampled").and_then(|s| epoch_of_date(&s)) else {
            counts.date_absent += 1;
            continue;
        };
        let qualifier = match xml_field(body, "qualifier") {
            Some(q) => q,
            None => String::new(),
        };
        if censored(&qualifier) {
            counts.censored_absent += 1;
            continue;
        }
        let Some(unit) = xml_field(body, "unit_of_measurement").and_then(|s| si_unit(&s)) else {
            counts.unit_absent += 1;
            continue;
        };
        let Some(result) = xml_field(body, "sample_result").and_then(|s| finite_result(&s)) else {
            counts.result_absent += 1;
            continue;
        };
        let pos = xml_field(body, "ZIP_CODE").and_then(|z| zcta::lookup(gaz, &z));
        if pos.is_some() {
            counts.geocoded += 1;
        } else {
            counts.zip_absent += 1;
        }
        samples.push(Sample {
            unix,
            value: to_si(result, &unit),
            quantity: unit.quantity,
            pos,
        });
        counts.carried += 1;
        if samples.len() >= limit {
            break;
        }
    }
    Ok((samples, counts))
}

fn find_member<'a>(entries: &'a [ZipEntry], member: &str) -> Option<&'a ZipEntry> {
    entries.iter().find(|e| e.name == member).or_else(|| {
        entries
            .iter()
            .find(|e| e.name.ends_with("/sample_data_2019.csv"))
    })
}

fn parse_step(args: &[String], name: &str, default: usize) -> Result<usize, String> {
    match arg_value(args, name) {
        Some(v) => {
            let n = v
                .parse::<usize>()
                .map_err(|_| format!("{name} {v} carries no step"))?;
            if n == 0 {
                return Err(format!("{name} carries no positive step"));
            }
            Ok(n)
        }
        None => Ok(default),
    }
}

fn ensure_parent(out: &str) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    Ok(())
}

fn last_seg(path: &str) -> &str {
    match path.rsplit('/').next() {
        Some(s) => s,
        None => path,
    }
}

fn zip_source(args: &[String]) -> String {
    match arg_value(args, "--input") {
        Some(p) => p,
        None => match arg_value(args, "--url") {
            Some(u) => u,
            None => ZIP_URL.to_string(),
        },
    }
}

fn load_gazetteer(args: &[String]) -> Vec<zcta::Zcta> {
    let path = match arg_value(args, "--gazetteer") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{}/{GAZETTEER_ASSET}", zcta::NETLOC),
    };
    match std::fs::read(&path) {
        Ok(bytes) => match zcta::parse_bin(&bytes) {
            Some(rows) => rows,
            None => {
                eprintln!(
                    "osha_cehd_compiler: {path} parses void — every position stays pending (no geocode)"
                );
                Vec::new()
            }
        },
        Err(_) => {
            eprintln!(
                "osha_cehd_compiler: {path} stays unread — every position stays pending (no geocode)"
            );
            Vec::new()
        }
    }
}

fn inspect(source: &str, bytes: &[u8]) -> Result<(), String> {
    let entries = zip_entries(bytes)
        .ok_or_else(|| format!("{source}: central directory void — the zip stays unread"))?;
    println!("{source}: {} members", entries.len());
    for e in &entries {
        println!("{} {} {} {}", e.comp_size, e.method, e.local_offset, e.name);
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let source = zip_source(args);
    let bytes = read_bytes(&source)?;
    if args.iter().any(|a| a == "--inspect") {
        return inspect(&source, &bytes);
    }

    let stride = parse_step(args, "--stride", 1)?;
    let limit = parse_step(args, "--limit", usize::MAX)?;
    let format = match arg_value(args, "--format") {
        Some(f) if !f.is_empty() => f,
        _ => FORMAT.to_string(),
    };
    let member_name = match arg_value(args, "--member") {
        Some(m) if !m.is_empty() => m,
        _ => MEMBER.to_string(),
    };
    let entries = zip_entries(&bytes)
        .ok_or_else(|| format!("{source}: central directory void — the zip stays unread"))?;
    let member = find_member(&entries, &member_name)
        .ok_or_else(|| format!("{source}: no {member_name} member — the table stays unread"))?;
    let raw = zip_extract(&bytes, member)
        .ok_or_else(|| format!("{}: entry extract void", member.name))?;
    let table = String::from_utf8_lossy(&raw);
    let gaz = load_gazetteer(args);
    let (samples, counts) = if member.name.ends_with(".xml") {
        parse_cehd_xml(&table, stride, limit, &gaz)?
    } else {
        parse_cehd(&table, stride, limit, &gaz)?
    };
    if samples.is_empty() {
        return Err(format!(
            "{source}: no row with a measured date, uncensored {RESULT_FIELD} and a measured {UNIT_FIELD} — the artifact stays unwritten (0 honored)"
        ));
    }

    let mut text = String::new();
    text.push_str(&format!(
        "# OSHA CEHD sample value | SI unit per line | source {source} | member {}\n",
        member.name
    ));
    text.push_str(
        "# unit table: M = mg/m3 -> kg/m3; mcg/m3 = ug/m3 -> kg/m3; X = ug -> kg; Y = mg -> kg;\n",
    );
    text.push_str("#   P = ppm (vol) -> amount fraction (1); F = fibers/cc -> number density (1/m3); % -> fraction (1);\n");
    text.push_str("# unit codes N, BM/S, AAAAA and E carry no measured definition in the source -> absent (pending unit->SI);\n");
    text.push_str("#   position = 2020 ZCTA centroid resolved from ZIP_CODE via the Census gazetteer (INTPTLAT/INTPTLONG);\n");
    text.push_str("#   an unresolved ZIP carries `-`, never 0.0\n");
    for s in &samples {
        match s.pos {
            Some((lat, lon)) => text.push_str(&format!(
                "{} {} {} {lat} {lon}\n",
                s.unix,
                s.value,
                s.quantity.label()
            )),
            None => text.push_str(&format!(
                "{} {} {} - -\n",
                s.unix,
                s.value,
                s.quantity.label()
            )),
        }
    }

    let netloc = match arg_value(args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => NETLOC.to_string(),
    };
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{netloc}/osha_cehd_si.txt"),
    };
    ensure_parent(&out)?;
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} void: {e}"))?;

    println!(
        "url https://github.com/omegaflow/sources/releases/download/{netloc}/{}",
        last_seg(&out)
    );
    println!("origin {source}");
    println!("compiler tools/harvest/src/bin/osha_cehd_compiler.rs");
    println!("format {format}");
    println!("sha256 {}", sha256_hex(text.as_bytes()));
    eprintln!(
        "{out}: {} carried of {} rows ({} date-absent, {} result-absent, {} censored, {} unit-absent, {} geocoded, {} zip-absent), {} B",
        counts.carried,
        counts.rows,
        counts.date_absent,
        counts.result_absent,
        counts.censored_absent,
        counts.unit_absent,
        counts.geocoded,
        counts.zip_absent,
        text.len()
    );
    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {netloc} release"
        ));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: osha_cehd_compiler [--input <zip-path>] [--url <zip-url>] [--inspect] [--member <name>] [--netloc <netloc>] [--stride N] [--limit N] [--gazetteer <bin>] [--out <path>] [--ci-mode]"
        );
        eprintln!(
            "  reads the US OSHA CEHD health-samples zip; the CSV arm reads sample_data_2014.csv..2019.csv, the XML arm sample_data_1984.xml..2013.xml"
        );
        eprintln!("  --inspect lists the zip central directory and stops");
        eprintln!("  emits one text record per sample: <unix_seconds> <value> <si_unit>");
        eprintln!(
            "  unit table: M = mg/m3 -> kg/m3; mcg/m3 = ug/m3 -> kg/m3; X = ug -> kg; Y = mg -> kg;"
        );
        eprintln!(
            "    P = ppm (vol) -> amount fraction (1); F = fibers/cc -> number density (1/m3); % -> fraction (1)"
        );
        eprintln!(
            "    N/BM/S/AAAAA/E carry no measured definition in the source -> absent (pending unit->SI)"
        );
        eprintln!(
            "  position = 2020 ZCTA centroid resolved from ZIP_CODE via --gazetteer; an unresolved ZIP carries `-`, never 0.0"
        );
        eprintln!("  --limit bounds the emitted samples; --stride samples every Nth data row");
        eprintln!("  --ci-mode uploads the verified asset to the <netloc> CDN release");
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("osha_cehd_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_csv_keeps_quoted_commas() {
        let fields = split_csv("a,\"b,c\",d");
        assert_eq!(fields, vec!["a", "b,c", "d"]);
    }

    #[test]
    fn epoch_of_date_reads_the_named_month() {
        let d0 = epoch_of_date("2019-FEB-01").unwrap();
        let d1 = epoch_of_date("2019-FEB-02").unwrap();
        assert!((d1 - d0 - 86400.0).abs() < 1.0);
        assert!(epoch_of_date("").is_none());
        assert!(epoch_of_date("2019-XXX-01").is_none());
    }

    #[test]
    fn si_unit_maps_measured_units_to_their_si_scale() {
        let m = si_unit("M").unwrap();
        assert!(matches!(m.quantity, SiQuantity::MassConcentration));
        assert!((m.factor - 1e-6).abs() < 1e-18);
        let ug = si_unit(" mcg/m3 ").unwrap();
        assert!(matches!(ug.quantity, SiQuantity::MassConcentration));
        assert!((ug.factor - 1e-9).abs() < 1e-18);
        let x = si_unit("X").unwrap();
        assert!(matches!(x.quantity, SiQuantity::Mass));
        assert!((x.factor - 1e-9).abs() < 1e-18);
        let y = si_unit("Y").unwrap();
        assert!(matches!(y.quantity, SiQuantity::Mass));
        assert!((y.factor - 1e-6).abs() < 1e-18);
        let p = si_unit("P").unwrap();
        assert!(matches!(p.quantity, SiQuantity::AmountFraction));
        assert!((p.factor - 1e-6).abs() < 1e-18);
        let f = si_unit("F").unwrap();
        assert!(matches!(f.quantity, SiQuantity::NumberDensity));
        assert!((f.factor - 1e6).abs() < 1e-6);
        let pct = si_unit("%").unwrap();
        assert!(matches!(pct.quantity, SiQuantity::Fraction));
        assert!((pct.factor - 1e-2).abs() < 1e-18);
    }

    #[test]
    fn si_unit_leaves_unmeasured_codes_absent() {
        for code in ["N", "BM/S", "AAAAA", "E", "unknown"] {
            assert!(si_unit(code).is_none(), "code {code} is unmeasured");
        }
    }

    #[test]
    fn to_si_applies_the_unit_scale() {
        assert!((to_si(1.0, &si_unit("M").unwrap()) - 1e-6).abs() < 1e-15);
        assert!((to_si(1.0, &si_unit("X").unwrap()) - 1e-9).abs() < 1e-18);
        assert!((to_si(2.0, &si_unit("F").unwrap()) - 2e6).abs() < 1e-6);
    }

    #[test]
    fn finite_result_refuses_void_and_negative() {
        assert_eq!(finite_result("0.5179"), Some(0.5179));
        assert_eq!(finite_result("0"), Some(0.0));
        assert_eq!(finite_result(""), None);
        assert_eq!(finite_result("-1"), None);
    }

    #[test]
    fn censored_results_stay_absent() {
        assert!(censored("ND"));
        assert!(censored("blk"));
        assert!(!censored(""));
        assert!(!censored("@"));
    }

    fn fixture() -> &'static str {
        "SIC_CODE,NAICS_CODE,DATE_SAMPLED,SUBSTANCE,SAMPLE_RESULT,UNIT_OF_MEASUREMENT,QUALIFIER\n\
,\"123,456\",2019-FEB-01,\"Arsenic, Inorganic\",0.5,M,\n\
562910,562910,2019-FEB-01,Lead,0.5179,mcg/m3,\n\
562910,562910,2019-FEB-01,Cadmium,0,M,ND\n\
562910,562910,2019-FEB-01,Quartz,0.2,X,\n\
562910,562910,2019-FEB-01,Noise,80,F,\n"
    }

    #[test]
    fn parse_cehd_carries_measured_units_with_their_si_scale() {
        let (samples, counts) = parse_cehd(fixture(), 1, usize::MAX, &[]).unwrap();
        assert_eq!(counts.rows, 5);
        assert_eq!(samples.len(), 4);
        assert_eq!(counts.censored_absent, 1);
        assert_eq!(counts.unit_absent, 0);
        assert!((samples[0].value - 0.5e-6).abs() < 1e-15);
        assert!((samples[1].value - 0.5179e-9).abs() < 1e-18);
        assert!((samples[2].value - 0.2e-9).abs() < 1e-18);
        assert!((samples[3].value - 80e6).abs() < 1e-6);
    }

    #[test]
    fn parse_cehd_does_not_fabricate_a_position() {
        let (samples, _) = parse_cehd(fixture(), 1, usize::MAX, &[]).unwrap();
        assert!(
            samples
                .iter()
                .all(|s| s.unix.is_finite() && s.value.is_finite() && s.pos.is_none())
        );
    }

    fn xml_fixture() -> &'static str {
        "<main xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">\n\
<main>\n\
<DATA_RECORD>\n\
<inspection_number>111</inspection_number>\n\
<ESTABLISHMENT_NAME>ACME &#40;A&#41;</ESTABLISHMENT_NAME>\n\
<ZIP_CODE>21093</ZIP_CODE>\n\
<inspection_number>111</inspection_number>\n\
<date_sampled>2013-JAN-25</date_sampled>\n\
<sample_result>0.5179</sample_result>\n\
<unit_of_measurement>mcg/m3</unit_of_measurement>\n\
<qualifier></qualifier>\n\
<substance>Lead</substance>\n\
</DATA_RECORD>\n\
<DATA_RECORD>\n\
<inspection_number>222</inspection_number>\n\
<ZIP_CODE>99999</ZIP_CODE>\n\
<date_sampled>2013-JAN-26</date_sampled>\n\
<sample_result>0.2</sample_result>\n\
<unit_of_measurement>M</unit_of_measurement>\n\
<qualifier>ND</qualifier>\n\
<substance>Cadmium</substance>\n\
</DATA_RECORD>\n\
<DATA_RECORD>\n\
<inspection_number>333</inspection_number>\n\
<ZIP_CODE>99999</ZIP_CODE>\n\
<date_sampled>2013-JAN-27</date_sampled>\n\
<sample_result>80</sample_result>\n\
<unit_of_measurement>&#37;</unit_of_measurement>\n\
<qualifier></qualifier>\n\
<substance>Quartz</substance>\n\
</DATA_RECORD>\n\
</main>\n\
</main>\n"
    }

    #[test]
    fn decode_entities_reads_numeric_and_named_references() {
        assert_eq!(decode_entities("ACME &#40;A&#41;"), "ACME (A)");
        assert_eq!(decode_entities("100&#37;"), "100%");
        assert_eq!(decode_entities("&amp;&lt;&gt;"), "&<>");
        assert_eq!(decode_entities("no entity"), "no entity");
    }

    #[test]
    fn parse_cehd_xml_carries_measured_units_with_their_si_scale() {
        let gaz = vec![zcta::Zcta {
            geoid: 21093,
            lat: 39.4,
            lon: -76.7,
        }];
        let (samples, counts) = parse_cehd_xml(xml_fixture(), 1, usize::MAX, &gaz).unwrap();
        assert_eq!(counts.rows, 3);
        assert_eq!(samples.len(), 2);
        assert_eq!(counts.censored_absent, 1);
        assert_eq!(counts.geocoded, 1);
        assert!((samples[0].value - 0.5179e-9).abs() < 1e-18);
        assert_eq!(samples[0].pos, Some((39.4, -76.7)));
        assert!((samples[1].value - 80e-2).abs() < 1e-18);
    }

    #[test]
    fn parse_cehd_xml_honours_stride_and_limit() {
        let (strided, _) = parse_cehd_xml(xml_fixture(), 2, usize::MAX, &[]).unwrap();
        assert_eq!(strided.len(), 2);
        let (limited, _) = parse_cehd_xml(xml_fixture(), 1, 1, &[]).unwrap();
        assert_eq!(limited.len(), 1);
    }

    fn zip_fixture() -> &'static str {
        "ZIP_CODE,DATE_SAMPLED,SAMPLE_RESULT,UNIT_OF_MEASUREMENT,QUALIFIER,SUBSTANCE,SIC_CODE,NAICS_CODE\n\
21093,2019-FEB-01,0.5,M,,Lead,562910,562910\n\
99999,2019-FEB-01,0.5,M,,Lead,562910,562910\n"
    }

    #[test]
    fn a_zip_resolves_to_its_zcta_centroid_and_an_unmapped_zip_stays_absent() {
        let gaz = vec![zcta::Zcta {
            geoid: 21093,
            lat: 39.4,
            lon: -76.7,
        }];
        let (samples, counts) = parse_cehd(zip_fixture(), 1, usize::MAX, &gaz).unwrap();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].pos, Some((39.4, -76.7)));
        assert_eq!(samples[1].pos, None);
        assert_eq!(counts.geocoded, 1);
        assert_eq!(counts.zip_absent, 1);
    }

    #[test]
    fn parse_cehd_honours_stride_and_limit() {
        let (strided, _) = parse_cehd(fixture(), 2, usize::MAX, &[]).unwrap();
        assert!(strided.len() <= 2);
        let (limited, _) = parse_cehd(fixture(), 1, 1, &[]).unwrap();
        assert_eq!(limited.len(), 1);
    }

    #[test]
    fn find_member_prefers_the_exact_name() {
        let entries = vec![
            ZipEntry {
                name: "healthsamples/sample_data_2018.csv".to_string(),
                method: 8,
                comp_size: 10,
                local_offset: 0,
            },
            ZipEntry {
                name: "healthsamples/sample_data_2019.csv".to_string(),
                method: 8,
                comp_size: 20,
                local_offset: 100,
            },
        ];
        let member = find_member(&entries, MEMBER).unwrap();
        assert_eq!(member.comp_size, 20);
    }
}
