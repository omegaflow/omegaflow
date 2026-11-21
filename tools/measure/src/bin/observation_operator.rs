use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use omegaflow::archivar::{lsk, sddpt};

const HEADER: &str = "target,epoch_tdb,observable,value,sigma,station,source,method,version\n";

#[derive(Clone, Debug)]
struct Row {
    target: String,
    epoch_tdb: String,
    observable: String,
    value: f64,
    sigma: Option<f64>,
    station: String,
    source: String,
    method: String,
}

#[derive(Clone, Debug)]
struct CensusRow {
    dataset: String,
    source: String,
    records: usize,
    first: f64,
    last: f64,
    note: String,
}

impl Row {
    fn line(&self) -> String {
        let sigma = match self.sigma {
            Some(value) => format!("{value:.9}"),
            None => "nan".to_string(),
        };
        format!(
            "{},{},{},{:.9},{},{},{},{},1\n",
            self.target,
            self.epoch_tdb,
            self.observable,
            self.value,
            sigma,
            self.station,
            self.source,
            self.method,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dataset {
    Mariner5,
    Mariner10,
    Viking,
    All,
}

impl Dataset {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "mariner5" | "pscm-00001" => Some(Self::Mariner5),
            "mariner10" | "pscm-00009" => Some(Self::Mariner10),
            "viking" | "pspg-00457" => Some(Self::Viking),
            "all" => Some(Self::All),
            _ => None,
        }
    }
    fn includes(self, other: Self) -> bool {
        self == Self::All || self == other
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: observation_operator [--root <sources/nssdc>] [--out <observations.tsv>] \
             [--dataset mariner5|mariner10|viking|all] [--census-out <observations_census.tsv>] \
             [--raw] [--raw-out <observations_raw.tsv>] \
             [--kat] [--kat-reference <tape_f00002_full.csv>]"
        );
        return;
    }
    if let Err(message) = run(&args) {
        eprintln!("observation_operator: {message}");
        std::process::exit(1);
    }
}

fn arg_value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn run(args: &[String]) -> Result<(), String> {
    let root = PathBuf::from(arg_value(args, "--root").unwrap_or("sources/nssdc"));
    let out = PathBuf::from(arg_value(args, "--out").unwrap_or("observations.tsv"));
    let census_out =
        PathBuf::from(arg_value(args, "--census-out").unwrap_or("observations_census.tsv"));
    let raw_out = PathBuf::from(arg_value(args, "--raw-out").unwrap_or("observations_raw.tsv"));
    let raw = args.iter().any(|a| a == "--raw");
    let dataset = match arg_value(args, "--dataset") {
        Some(value) => Dataset::parse(value).ok_or_else(|| format!("unknown --dataset {value}"))?,
        None => Dataset::All,
    };

    let mut calibrated: Vec<Row> = Vec::new();
    let mut raw_rows: Vec<Row> = Vec::new();
    let mut census: Vec<CensusRow> = Vec::new();

    if dataset.includes(Dataset::Mariner5) {
        let points = mariner5_points(&root)?;
        eprintln!(
            "mariner5: {} points from {} files",
            points.len(),
            mariner5_files(&root)?.len()
        );
        calibrated.extend(points.into_iter().map(|(source, point)| Row {
            target: "Venus".to_string(),
            epoch_tdb: iso8601(point.epoch_unix),
            observable: "doppler_cps".to_string(),
            value: point.doppler_cps,
            sigma: None,
            station: match point.station {
                Some(station) => format!("DSS-{station}"),
                None => "unknown".to_string(),
            },
            source,
            method: "sddpt-mariner5-base2-f36".to_string(),
        }));
    }

    if dataset.includes(Dataset::Mariner10) {
        let (rows, files, file_census) = mariner10_rows(&root, raw)?;
        let count: usize = file_census.iter().map(|row| row.records).sum();
        eprintln!("mariner10: {count} records from {files} files");
        raw_rows.extend(rows);
        census.extend(file_census);
    }
    if dataset.includes(Dataset::Viking) {
        let (rows, files, file_census) = viking_rows(&root, raw)?;
        let count: usize = file_census.iter().map(|row| row.records).sum();
        eprintln!("viking: {count} records from {files} files");
        raw_rows.extend(rows);
        census.extend(file_census);
    }

    calibrated.sort_by(|a, b| a.epoch_tdb.cmp(&b.epoch_tdb));
    write_rows(&out, &calibrated)?;
    eprintln!("{}: {} calibrated rows", out.display(), calibrated.len());

    if !census.is_empty() {
        write_census(&census_out, &census)?;
        eprintln!(
            "{}: {} file rows (one per tape file; raw fields have no epoch yet)",
            census_out.display(),
            census.len()
        );
    }

    if raw && !raw_rows.is_empty() {
        let target = if dataset == Dataset::All {
            &raw_out
        } else {
            &out
        };
        write_rows(target, &raw_rows)?;
        eprintln!(
            "{}: {} raw record rows (empty epoch, method prefix sddpt-)",
            target.display(),
            raw_rows.len()
        );
    }

    if args.iter().any(|a| a == "--kat") {
        let reference = arg_value(args, "--kat-reference")
            .map(PathBuf::from)
            .or_else(|| {
                dataset_dir(&root, "PSCM-00001")
                    .ok()
                    .map(|dir| dir.join("tape_f00002_full.csv"))
            })
            .filter(|path| path.exists());
        kat(&root, reference.as_deref())?;
    }
    Ok(())
}

fn write_rows(path: &Path, rows: &[Row]) -> Result<(), String> {
    let mut text = String::with_capacity(HEADER.len() + rows.len() * 120);
    text.push_str(HEADER);
    for row in rows {
        text.push_str(&row.line());
    }
    fs::write(path, text).map_err(|e| format!("write {}: {e}", path.display()))
}

fn write_census(path: &Path, rows: &[CensusRow]) -> Result<(), String> {
    let mut text = String::with_capacity(64 + rows.len() * 120);
    text.push_str("dataset,source,records,first,last,note\n");
    for row in rows {
        text.push_str(&format!(
            "{},{},{},{:.6},{:.6},{}\n",
            row.dataset, row.source, row.records, row.first, row.last, row.note
        ));
    }
    fs::write(path, text).map_err(|e| format!("write {}: {e}", path.display()))
}

fn dataset_dir(root: &Path, name: &str) -> Result<PathBuf, String> {
    let candidates = [
        root.join(name),
        root.join("nssdc").join(name),
        root.join("nssdc.gsfc.nasa.gov").join(name),
    ];
    for candidate in candidates {
        if candidate.is_dir() {
            return Ok(candidate);
        }
    }
    Err(format!("{name} not found under {}", root.display()))
}

fn mariner5_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let dir = dataset_dir(root, "PSCM-00001")?;
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| format!("read {}: {e}", dir.display()))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("dr004487_f") && name.ends_with(".phys.1"))
        })
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(format!(
            "no Mariner 5 .phys.1 files under {}",
            dir.display()
        ));
    }
    Ok(files)
}

fn mariner5_points(root: &Path) -> Result<Vec<(String, sddpt::Mariner5Point)>, String> {
    let mut out = Vec::new();
    for path in mariner5_files(root)? {
        let bytes = fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let source = format!(
            "PSCM-00001/{}",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("?")
        );
        for point in sddpt::mariner5_points(&bytes) {
            out.push((source.clone(), point));
        }
    }
    out.sort_by(|a, b| a.1.epoch_unix.total_cmp(&b.1.epoch_unix));
    Ok(out)
}

fn mariner10_rows(root: &Path, collect: bool) -> Result<(Vec<Row>, usize, Vec<CensusRow>), String> {
    let dir = dataset_dir(root, "PSCM-00009")?;
    let files = data_files(&dir, "CMR_")?;
    let mut rows = Vec::new();
    let mut census = Vec::new();
    for path in &files {
        let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let source = dataset_source("PSCM-00009", &dir, path);
        let records = sddpt::cmr_records(&bytes);
        let mut first = 0.0;
        let mut last = 0.0;
        let mut seen = 0usize;
        for record in &records {
            let Some(count) = record.time_count else {
                continue;
            };
            if seen == 0 {
                first = count as f64;
            }
            last = count as f64;
            seen += 1;
            if collect {
                rows.push(Row {
                    target: "Venus".to_string(),
                    epoch_tdb: String::new(),
                    observable: "cmr_word2_count".to_string(),
                    value: count as f64,
                    sigma: None,
                    station: "unknown".to_string(),
                    source: source.clone(),
                    method: "sddpt-cmr-word2".to_string(),
                });
            }
        }
        census.push(CensusRow {
            dataset: "PSCM-00009".to_string(),
            source,
            records: seen,
            first,
            last,
            note: "cmr word2 monotone counter; unit not established".to_string(),
        });
    }
    Ok((rows, files.len(), census))
}

fn viking_rows(root: &Path, collect: bool) -> Result<(Vec<Row>, usize, Vec<CensusRow>), String> {
    let dir = dataset_dir(root, "PSPG-00457")?;
    let files = data_files(&dir, "PT_")?;
    let mut rows = Vec::new();
    let mut census = Vec::new();
    for path in &files {
        let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let source = dataset_source("PSPG-00457", &dir, path);
        let records = sddpt::pt_records(&bytes);
        let mut first = 0.0;
        let mut last = 0.0;
        let mut seen = 0usize;
        for record in &records {
            let (observable, value) = match record.range_km {
                Some(range) => ("pt_range_km", range),
                None => ("pt_word3", f64::from(record.words[3])),
            };
            if seen == 0 {
                first = value;
            }
            last = value;
            seen += 1;
            if collect {
                rows.push(Row {
                    target: "Mars".to_string(),
                    epoch_tdb: String::new(),
                    observable: observable.to_string(),
                    value,
                    sigma: None,
                    station: "unknown".to_string(),
                    source: source.clone(),
                    method: "sddpt-pt-unsnaked".to_string(),
                });
            }
        }
        let note = match sddpt::pt_label_epoch(&bytes) {
            Some(epoch) => format!(
                "label epoch {}; per-record epoch not established; pt word3 candidate range (km)",
                label_epoch_text(epoch)
            ),
            None => "label epoch absent; pt word3 candidate range (km)".to_string(),
        };
        census.push(CensusRow {
            dataset: "PSPG-00457".to_string(),
            source,
            records: seen,
            first,
            last,
            note,
        });
    }
    Ok((rows, files.len(), census))
}

fn data_files(root: &Path, prefix: &str) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    walk(root, &mut out).map_err(|e| format!("walk {}: {e}", root.display()))?;
    out.retain(|path| {
        path.parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            == Some("DATA")
            && path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(prefix))
    });
    out.sort();
    Ok(out)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

fn dataset_source(dataset: &str, dir: &Path, path: &Path) -> String {
    format!(
        "{dataset}/{}",
        path.strip_prefix(dir).unwrap_or(path).to_string_lossy()
    )
}

fn label_epoch_text(epoch: sddpt::PtLabelEpoch) -> String {
    let Some(start) = lsk::days_from_civil(i64::from(epoch.year), 1, 1) else {
        return format!(
            "{:04}-{:03}T{:02}:{:02}:{:02}Z",
            epoch.year, epoch.day_of_year, epoch.hour, epoch.minute, epoch.second
        );
    };
    let days = start + i64::from(epoch.day_of_year) - 1;
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        epoch.hour, epoch.minute, epoch.second
    )
}

fn iso8601(unix: f64) -> String {
    let days = (unix / 86_400.0).floor() as i64;
    let seconds = unix - days as f64 * 86_400.0;
    let mut whole = seconds.floor() as i64;
    let mut millis = ((seconds - whole as f64) * 1000.0).round() as i64;
    if millis == 1000 {
        whole += 1;
        millis = 0;
    }
    let (year, month, day) = civil_from_days(days + whole / 86_400);
    let of_day = whole % 86_400;
    let (hour, minute, second) = (of_day / 3600, (of_day / 60) % 60, of_day % 60);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")
}

fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 {
        z / 146_097
    } else {
        (z - 146_096) / 146_097
    };
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month as u32, day as u32)
}

fn kat(root: &Path, reference: Option<&Path>) -> Result<(), String> {
    let files = mariner5_files(root)?;
    let mut per_file = Vec::new();
    for path in &files {
        let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        per_file.push((path.clone(), sddpt::mariner5_points(&bytes)));
    }
    if per_file.len() != 2 {
        return Err(format!(
            "KAT needs exactly two Mariner 5 files, found {}",
            per_file.len()
        ));
    }
    let expected = [1_500usize, 7_325usize];
    for ((path, points), want) in per_file.iter().zip(expected) {
        if points.len() != want {
            return Err(format!(
                "KAT count {}: {} points, expected {want}",
                path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                points.len()
            ));
        }
    }

    let f1 = &per_file[0].1;
    let f2 = &per_file[1].1;
    let checks = [
        ("f00001 first cps", f1[0].doppler_cps, 99_599.883_789_062_5),
        (
            "f00001 min cps",
            f1.iter().map(|p| p.doppler_cps).fold(f64::MAX, f64::min),
            40_702.900_878_906_25,
        ),
        (
            "f00001 max cps",
            f1.iter().map(|p| p.doppler_cps).fold(f64::MIN, f64::max),
            99_599.883_789_062_5,
        ),
        ("f00002 first cps", f2[0].doppler_cps, 45_580.632_324_218_75),
        (
            "f00002 last cps",
            f2[f2.len() - 1].doppler_cps,
            250_117.013_671_875,
        ),
        (
            "f00002 max cps",
            f2.iter().map(|p| p.doppler_cps).fold(f64::MIN, f64::max),
            280_993.058_593_75,
        ),
    ];
    for (name, got, want) in checks {
        if (got - want).abs() > 1.0e-3 {
            return Err(format!("KAT {name}: {got} != {want}"));
        }
    }
    let time_checks = [
        ("f00001 first epoch", f1[0].epoch_unix, -80_499_341.95),
        ("f00002 first epoch", f2[0].epoch_unix, -80_009_453.95),
        (
            "f00002 last epoch",
            f2[f2.len() - 1].epoch_unix,
            -66_629_341.95,
        ),
    ];
    for (name, got, want) in time_checks {
        if (got - want).abs() > 0.11 {
            return Err(format!("KAT {name}: {got} != {want}"));
        }
    }

    let mut matched = Vec::new();
    for a in f1 {
        if let Some(b) = f2.iter().min_by(|x, y| {
            (x.epoch_unix - a.epoch_unix)
                .abs()
                .total_cmp(&(y.epoch_unix - a.epoch_unix).abs())
        }) && (b.epoch_unix - a.epoch_unix).abs() < 1.0
        {
            matched.push((a.doppler_cps - b.doppler_cps).abs());
        }
    }
    if matched.len() < 10 {
        return Err(format!(
            "KAT overlap: only {} matched points",
            matched.len()
        ));
    }
    matched.sort_by(f64::total_cmp);
    let median = matched[matched.len() / 2];
    if median > 1.0e-3 {
        return Err(format!("KAT overlap median {median} cps"));
    }

    if let Some(path) = reference {
        let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let mut lines = text.lines();
        let header = lines.next().unwrap_or("");
        if !header.contains("unix") || !header.contains("obs_cps") {
            return Err(format!(
                "KAT reference {} has no unix/obs_cps header",
                path.display()
            ));
        }
        let mut rows = 0usize;
        for (line, point) in lines.zip(f2) {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 6 {
                continue;
            }
            let unix: f64 = fields[1]
                .parse()
                .map_err(|_| "KAT reference unix".to_string())?;
            let obs: f64 = fields[5]
                .parse()
                .map_err(|_| "KAT reference obs".to_string())?;
            if (unix - point.epoch_unix).abs() > 0.11 {
                return Err(format!(
                    "KAT reference epoch row {rows}: {unix} != {}",
                    point.epoch_unix
                ));
            }
            if (obs - point.doppler_cps).abs() > 1.0e-3 {
                return Err(format!(
                    "KAT reference cps row {rows}: {obs} != {}",
                    point.doppler_cps
                ));
            }
            rows += 1;
        }
        if rows != f2.len() {
            return Err(format!(
                "KAT reference rows {rows} != decoded points {}",
                f2.len()
            ));
        }
        eprintln!("KAT reference: {rows} rows match {path:?}");
    }

    let mut summary = String::new();
    let _ = writeln!(
        summary,
        "KAT ok: f00001 {} points, f00002 {} points, overlap {} matched (median {median:.3e} cps)",
        f1.len(),
        f2.len(),
        matched.len()
    );
    eprint!("{summary}");
    Ok(())
}
