use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::json::{JsonVal, jnum, jstr, parse_json};
use omegaflow::cdn::upload_release;

const CDN_TAG: &str = "clpds.bao.ac.cn";
const HOST: &str = "https://clpds.bao.ac.cn";
const CATALOGUE_PATH: &str = "/moon-admin/client/science/catalogue";
const LIST_PATH: &str = "/moon-admin/client/science/dataInfoList";
const CATALOGUE_PAGE_SIZE: usize = 100;
const PAGE_SIZE: usize = 5000;

struct Page {
    total: usize,
    rows: Vec<JsonVal>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn push_sep(out: &mut String, first: &mut bool) {
    if !*first {
        out.push(',');
    }
    *first = false;
}

fn push_str_field(out: &mut String, key: &str, val: &str, first: &mut bool) {
    push_sep(out, first);
    out.push('"');
    out.push_str(key);
    out.push_str("\":\"");
    out.push_str(&json_escape(val));
    out.push('"');
}

fn push_num_field(out: &mut String, key: &str, val: f64, first: &mut bool) {
    push_sep(out, first);
    out.push('"');
    out.push_str(key);
    out.push_str("\":");
    out.push_str(&format!("{}", val));
}

fn opt_str(out: &mut String, key: &str, row: &JsonVal, field: &str, first: &mut bool) {
    if let Some(v) = jstr(row, field) {
        if !v.is_empty() {
            push_str_field(out, key, &v, first);
        }
    }
}

fn opt_num(out: &mut String, key: &str, row: &JsonVal, field: &str, first: &mut bool) {
    if let Some(v) = jnum(row, field) {
        if v.is_finite() {
            push_num_field(out, key, v, first);
        }
    }
}

fn bbox(row: &JsonVal) -> Option<((f64, f64), (f64, f64))> {
    let lon0 = jnum(row, "longitudeStart")?;
    let lon1 = jnum(row, "longitudeEnd")?;
    let lat0 = jnum(row, "latitudeStart")?;
    let lat1 = jnum(row, "latitudeEnd")?;
    if !(lon0.is_finite() && lon1.is_finite() && lat0.is_finite() && lat1.is_finite()) {
        return None;
    }
    if !(-90.0..=90.0).contains(&lat0) || !(-90.0..=90.0).contains(&lat1) {
        return None;
    }
    if !(-180.0..=360.0).contains(&lon0) || !(-180.0..=360.0).contains(&lon1) {
        return None;
    }
    if lon0 == 0.0 && lon1 == 0.0 && lat0 == 0.0 && lat1 == 0.0 {
        return None;
    }
    Some(((lat0, lat1), (lon0, lon1)))
}

fn annex_of(row: &JsonVal) -> Option<String> {
    let JsonVal::Obj(map) = row else {
        return None;
    };
    let Some(JsonVal::Arr(arr)) = map.get("annexes") else {
        return None;
    };
    for a in arr {
        if let JsonVal::Obj(m) = a {
            if let Some(JsonVal::Str(p)) = m.get("dataAnnex") {
                if !p.is_empty() {
                    return Some(p.clone());
                }
            }
        }
    }
    None
}

fn catalogue_record(row: &JsonVal) -> Option<(f64, String)> {
    let id = jnum(row, "id")?;
    if !id.is_finite() {
        return None;
    }
    let mut out = String::with_capacity(320);
    out.push('{');
    let mut first = true;
    push_num_field(&mut out, "id", id, &mut first);
    opt_str(&mut out, "name", row, "name", &mut first);
    opt_num(&mut out, "task", row, "task", &mut first);
    opt_str(&mut out, "taskName", row, "taskName", &mut first);
    opt_num(&mut out, "load", row, "load", &mut first);
    opt_str(&mut out, "loadName", row, "loadName", &mut first);
    opt_num(&mut out, "level", row, "dataLevel", &mut first);
    opt_str(&mut out, "levelName", row, "dataLevelName", &mut first);
    opt_num(&mut out, "category", row, "category", &mut first);
    opt_str(&mut out, "categoryName", row, "categoryName", &mut first);
    opt_str(&mut out, "platform", row, "platform", &mut first);
    opt_str(&mut out, "size", row, "dataSize", &mut first);
    opt_str(&mut out, "format", row, "dataFormat", &mut first);
    opt_str(&mut out, "version", row, "dataVersion", &mut first);
    opt_str(&mut out, "obtainStart", row, "obtainTime", &mut first);
    opt_str(&mut out, "obtainEnd", row, "obtainTimeEnd", &mut first);
    opt_str(&mut out, "release", row, "releaseTime", &mut first);
    opt_num(&mut out, "releaseTotal", row, "releaseTotal", &mut first);
    opt_str(&mut out, "producer", row, "dataProducer", &mut first);
    opt_str(&mut out, "contact", row, "dataServiceContact", &mut first);
    opt_num(&mut out, "foreignId", row, "foreignId", &mut first);
    opt_str(&mut out, "desc", row, "desc", &mut first);
    out.push('}');
    Some((id, out))
}

fn list_record(row: &JsonVal) -> Option<(String, String)> {
    let id = jstr(row, "dataInfoId")?;
    if id.is_empty() {
        return None;
    }
    let mut out = String::with_capacity(320);
    out.push('{');
    let mut first = true;
    push_str_field(&mut out, "id", &id, &mut first);
    opt_str(&mut out, "name", row, "name", &mut first);
    opt_str(&mut out, "size", row, "dataSize", &mut first);
    opt_num(&mut out, "catalogue", row, "dataCatalogueId", &mut first);
    opt_str(
        &mut out,
        "catalogueName",
        row,
        "dataCatalogueName",
        &mut first,
    );
    opt_num(&mut out, "task", row, "task", &mut first);
    opt_str(&mut out, "taskName", row, "taskName", &mut first);
    opt_num(&mut out, "load", row, "load", &mut first);
    opt_str(&mut out, "loadName", row, "loadName", &mut first);
    opt_num(&mut out, "level", row, "dataLevel", &mut first);
    opt_str(&mut out, "levelName", row, "dataLevelName", &mut first);
    opt_str(&mut out, "station", row, "receivingStation", &mut first);
    opt_str(&mut out, "detection", row, "detectionTask", &mut first);
    opt_str(&mut out, "version", row, "dataVersion", &mut first);
    opt_str(&mut out, "start", row, "startTime", &mut first);
    opt_str(&mut out, "end", row, "endTime", &mut first);
    opt_num(&mut out, "orbit", row, "orbitalNumber", &mut first);
    if let Some(v) = jstr(row, "thirdDataInfoId") {
        if !v.is_empty() {
            push_str_field(&mut out, "thirdId", &v, &mut first);
        }
    }
    if let Some(a) = annex_of(row) {
        push_str_field(&mut out, "annex", &a, &mut first);
    }
    if let Some((lat, lon)) = bbox(row) {
        push_sep(&mut out, &mut first);
        out.push_str("\"lat\":[");
        out.push_str(&format!("{},{}", lat.0, lat.1));
        out.push_str("],\"lon\":[");
        out.push_str(&format!("{},{}", lon.0, lon.1));
        out.push(']');
    }
    out.push('}');
    Some((id, out))
}

fn fetch_page(url: &str) -> Option<Page> {
    let bytes = fetch_raw_bytes(url)?;
    let text = std::str::from_utf8(&bytes).ok()?;
    let JsonVal::Obj(map) = parse_json(text)? else {
        return None;
    };
    let total = match map.get("total") {
        Some(JsonVal::Num(n)) if n.is_finite() && *n >= 0.0 => *n as usize,
        _ => return None,
    };
    let rows = match map.get("rows") {
        Some(JsonVal::Arr(rows)) => rows.clone(),
        _ => return None,
    };
    Some(Page { total, rows })
}

fn fetch_catalogue() -> Vec<(f64, String)> {
    let mut out: Vec<(f64, String)> = Vec::new();
    let mut page = 1usize;
    loop {
        let url = format!(
            "{}{}?pageNum={}&pageSize={}",
            HOST, CATALOGUE_PATH, page, CATALOGUE_PAGE_SIZE
        );
        let Some(p) = fetch_page(&url) else {
            eprintln!(
                "clpds: catalogue page {} returned void — the manifest carries the pages read so far",
                page
            );
            break;
        };
        if p.rows.is_empty() {
            break;
        }
        let n = p.rows.len();
        for row in &p.rows {
            if let Some(rec) = catalogue_record(row) {
                out.push(rec);
            }
        }
        if n < CATALOGUE_PAGE_SIZE {
            break;
        }
        if p.total != 0 && out.len() >= p.total {
            break;
        }
        page += 1;
        if page > 64 {
            break;
        }
    }
    out.sort_by(|a, b| match a.0.partial_cmp(&b.0) {
        Some(o) => o,
        None => std::cmp::Ordering::Equal,
    });
    out
}

fn fetch_files(page_size: usize, max_pages: Option<usize>) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut page = 1usize;
    loop {
        if let Some(mp) = max_pages {
            if page > mp {
                break;
            }
        }
        let url = format!(
            "{}{}?pageNum={}&pageSize={}",
            HOST, LIST_PATH, page, page_size
        );
        let Some(p) = fetch_page(&url) else {
            eprintln!(
                "clpds: file page {} returned void — the manifest carries the pages read so far",
                page
            );
            break;
        };
        if p.rows.is_empty() {
            break;
        }
        let n = p.rows.len();
        for row in &p.rows {
            if let Some(rec) = list_record(row) {
                out.push(rec);
            }
        }
        eprintln!(
            "clpds: file page {} — {} rows ({} measured)",
            page, n, p.total
        );
        if n < page_size {
            break;
        }
        if p.total != 0 && out.len() >= p.total {
            break;
        }
        page += 1;
        if page > 100000 {
            break;
        }
    }
    out.sort_by(|a, b| match (a.0.parse::<u128>(), b.0.parse::<u128>()) {
        (Ok(x), Ok(y)) => x.cmp(&y),
        _ => a.0.cmp(&b.0),
    });
    out.dedup_by(|a, b| a.0 == b.0);
    out
}

fn write_asset(out_dir: &str, name: &str, body: &str) -> Option<String> {
    let path = format!("{}/{}", out_dir.trim_end_matches('/'), name);
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&path, body).is_err() {
        return None;
    }
    Some(path)
}

fn roundtrip_catalogue(path: &str, count: usize) -> bool {
    let Ok(text) = std::fs::read_to_string(path) else {
        return false;
    };
    match parse_json(&text) {
        Some(JsonVal::Arr(arr)) => {
            arr.iter().filter(|v| matches!(v, JsonVal::Obj(_))).count() == count
        }
        _ => false,
    }
}

fn roundtrip_files(path: &str, count: usize) -> bool {
    let Ok(text) = std::fs::read_to_string(path) else {
        return false;
    };
    let mut seen = 0usize;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match parse_json(line) {
            Some(JsonVal::Obj(map)) => match map.get("id") {
                Some(JsonVal::Str(s)) if !s.is_empty() => seen += 1,
                _ => return false,
            },
            _ => return false,
        }
    }
    seen == count
}

const FIXTURE_CATALOGUE: &str = r#"{"total":2,"code":200,"rows":[{"id":673,"name":"2016 HO3_CNSA_OEM","taskName":"Tianwen-2","dataSize":"48 KB","dataFormat":"ephemeris"},{"id":485,"name":"MARS ION","dataSize":"463.8GB"}]}"#;
const FIXTURE_LIST: &str = r#"{"total":3,"code":200,"rows":[{"dataInfoId":"250530123647582066","name":"a.dat","dataSize":"49152.0","orbitalNumber":1.0,"longitudeStart":0.0,"longitudeEnd":0.0,"latitudeStart":0.0,"latitudeEnd":0.0},{"dataInfoId":"250530123647582078","name":"b.dat","longitudeStart":10.0,"longitudeEnd":12.0,"latitudeStart":-5.0,"latitudeEnd":-4.0},{"name":"no id"}]}"#;

fn rows_of(fixture: &str) -> Vec<JsonVal> {
    match parse_json(fixture) {
        Some(JsonVal::Obj(map)) => match map.get("rows") {
            Some(JsonVal::Arr(rows)) => rows.clone(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

fn selftest() {
    let cat: Vec<(f64, String)> = rows_of(FIXTURE_CATALOGUE)
        .iter()
        .filter_map(catalogue_record)
        .collect();
    if cat.len() != 2 {
        eprintln!("selftest: {} catalogue rows folded, 2 measured", cat.len());
        std::process::exit(1);
    }
    let files: Vec<(String, String)> = rows_of(FIXTURE_LIST)
        .iter()
        .filter_map(list_record)
        .collect();
    if files.len() != 2 {
        eprintln!(
            "selftest: {} file rows survived, 2 measured (a row without id stays absent)",
            files.len()
        );
        std::process::exit(1);
    }
    if files[0].1.contains("\"lat\"") {
        eprintln!("selftest: the all-zero position box was written, absent measured");
        std::process::exit(1);
    }
    if !files[1].1.contains("\"lat\":[-5,-4]") {
        eprintln!("selftest: the measured position box does not carry the measured value");
        std::process::exit(1);
    }
    if !rows_of("not json").is_empty() {
        eprintln!("selftest: a non-JSON body is read as rows");
        std::process::exit(1);
    }
    eprintln!("clpds_compiler: selftest passes (catalogue + file fold, plausibility gate)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_dir = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .to_string_lossy()
            .into_owned(),
    };
    let page_size = match arg_value(&args, "--page-size") {
        Some(v) => match v.parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => PAGE_SIZE,
        },
        None => PAGE_SIZE,
    };
    let max_pages = arg_value(&args, "--max-pages").and_then(|v| v.parse::<usize>().ok());

    let cat = fetch_catalogue();
    if cat.is_empty() {
        eprintln!("clpds: the catalogue carried no dataset — nothing fabricated");
        std::process::exit(1);
    }
    let cat_body = format!(
        "[{}]\n",
        cat.iter()
            .map(|(_, r)| r.as_str())
            .collect::<Vec<&str>>()
            .join(",")
    );
    let Some(cat_path) = write_asset(&out_dir, "clpds_catalogue.json", &cat_body) else {
        eprintln!("clpds: catalogue write returned void");
        std::process::exit(1);
    };
    if !roundtrip_catalogue(&cat_path, cat.len()) {
        eprintln!("clpds: catalogue roundtrip lost rows — the asset stays unverified");
        std::process::exit(1);
    }
    eprintln!("{}: {} datasets, {} B", cat_path, cat.len(), cat_body.len());

    let files = fetch_files(page_size, max_pages);
    if files.is_empty() {
        eprintln!("clpds: no file row survived the fold — nothing fabricated");
        std::process::exit(1);
    }
    let mut body = String::with_capacity(files.len() * 256);
    for (_, line) in &files {
        body.push_str(line);
        body.push('\n');
    }
    let Some(files_path) = write_asset(&out_dir, "clpds_files.jsonl", &body) else {
        eprintln!("clpds: file manifest write returned void");
        std::process::exit(1);
    };
    if !roundtrip_files(&files_path, files.len()) {
        eprintln!("clpds: file manifest roundtrip lost records — the asset stays unverified");
        std::process::exit(1);
    }
    eprintln!("{}: {} files, {} B", files_path, files.len(), body.len());

    if ci_mode {
        if !upload_release(CDN_TAG, &cat_path) {
            eprintln!("clpds_catalogue.json: CDN upload returned void");
            std::process::exit(1);
        }
        if !upload_release(CDN_TAG, &files_path) {
            eprintln!("clpds_files.jsonl: CDN upload returned void");
            std::process::exit(1);
        }
    }
}
