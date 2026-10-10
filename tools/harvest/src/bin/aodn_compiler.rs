use omegaflow::archivar::cf_time_unix_seconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::extract::kernel_id_of;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::opendap::{
    AsciiVar, DapAttr, DapAttrs, DapSchema, DapType, parse_ascii, parse_das, parse_dds,
};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::mathematikerin::force::force_id_of;
use std::collections::HashMap;

const NETLOC: &str = "thredds.aodn.org.au";
const DODS_BASE: &str = "https://thredds.aodn.org.au/thredds/dodsC/";
const COMPILER: &str = "tools/harvest/src/bin/aodn_compiler.rs";
const MAGIC: [u8; 4] = *b"AODN";
const REC_BYTES: usize = 26 * 8;
const DEFAULT_COUNT: usize = 256;

struct FieldMap {
    source: &'static str,
    key: &'static str,
    kernel: &'static str,
    force: &'static str,
    unit: &'static str,
    celsius_to_kelvin: bool,
}

const FIELDS: [FieldMap; 5] = [
    FieldMap {
        source: "TEMP",
        key: "aodn_temperature",
        kernel: "exponential-decay",
        force: "thermal",
        unit: "K",
        celsius_to_kelvin: true,
    },
    FieldMap {
        source: "PSAL",
        key: "aodn_salinity",
        kernel: "gaussian-inverse-square",
        force: "diffusion",
        unit: "PSU",
        celsius_to_kelvin: false,
    },
    FieldMap {
        source: "UCUR",
        key: "aodn_velocity_u",
        kernel: "patch-levy",
        force: "advective",
        unit: "m/s",
        celsius_to_kelvin: false,
    },
    FieldMap {
        source: "VCUR",
        key: "aodn_velocity_v",
        kernel: "patch-levy",
        force: "advective",
        unit: "m/s",
        celsius_to_kelvin: false,
    },
    FieldMap {
        source: "WSSH",
        key: "aodn_wave_height",
        kernel: "gaussian-inverse-square",
        force: "acoustic",
        unit: "m",
        celsius_to_kelvin: false,
    },
];

struct VarAttrs {
    fill: f64,
    valid_min: Option<f64>,
    valid_max: Option<f64>,
}

struct Obs {
    unix: f64,
    val: f64,
    lat: f64,
    lon: f64,
    alt: f64,
}

fn usage() -> &'static str {
    "usage: aodn_compiler --var <TEMP|PSAL|UCUR|VCUR|WSSH> --body <name> (--url <opendap.nc> | --catalog <catalog.xml> [--match <substr>]) [--start <i>] [--count <n>] [--out <dir>] [--ephemeris <src>] [--ci-mode]"
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn field_of(source: &str) -> Option<&'static FieldMap> {
    FIELDS.iter().find(|f| f.source == source)
}

fn fetch_text(url: &str) -> Result<String, String> {
    match fetch_raw_bytes(url) {
        Some(b) => Ok(String::from_utf8_lossy(&b).into_owned()),
        None => Err(format!("{url}: fetch returned void (0 honored)")),
    }
}

fn strip_format_suffix(url: &str) -> String {
    for suf in [".ascii", ".dds", ".das", ".dods", ".html"] {
        if let Some(base) = url.strip_suffix(suf) {
            return base.to_string();
        }
    }
    url.to_string()
}

fn first_url_path(xml: &str, needle: Option<&str>) -> Option<String> {
    for line in xml.lines() {
        let key = "urlPath=\"";
        let Some(i) = line.find(key) else { continue };
        let rest = &line[i + key.len()..];
        let Some(j) = rest.find('"') else { continue };
        let path = &rest[..j];
        match needle {
            Some(n) if !path.contains(n) => continue,
            _ => return Some(path.to_string()),
        }
    }
    None
}

fn base_url(args: &[String]) -> Result<String, String> {
    if let Some(u) = arg_value(args, "--url") {
        return Ok(strip_format_suffix(&u));
    }
    if let Some(c) = arg_value(args, "--catalog") {
        let xml = fetch_text(&c)?;
        let needle = arg_value(args, "--match");
        let path = first_url_path(&xml, needle.as_deref())
            .ok_or_else(|| format!("{c}: no dataset urlPath carries the match"))?;
        return Ok(format!("{DODS_BASE}{path}"));
    }
    Err(usage().to_string())
}

fn var_len(schema: &DapSchema, name: &str) -> Option<usize> {
    let v = schema.vars.iter().find(|v| v.name == name)?;
    if v.dims.len() != 1 {
        return None;
    }
    let dn = &v.dims[0];
    schema
        .dims
        .iter()
        .find(|d| &d.name == dn)
        .map(|d| d.len as usize)
}

fn is_1d(schema: &DapSchema, name: &str) -> bool {
    match schema.vars.iter().find(|v| v.name == name) {
        Some(v) => v.dims.len() == 1,
        None => false,
    }
}

fn attr_list<'a>(das: &'a DapAttrs, var: &str) -> Option<&'a Vec<DapAttr>> {
    for (k, v) in &das.per_var {
        if k == var {
            return Some(v);
        }
        if let Some(i) = k.rfind('.') {
            if &k[i + 1..] == var {
                return Some(v);
            }
        }
    }
    None
}

fn num_attr(list: &[DapAttr], key: &str) -> Option<f64> {
    let a = list.iter().find(|a| a.name == key)?;
    match a.dap_type {
        DapType::Str | DapType::Url => None,
        _ => {
            let b = a.raw.get(0..8)?;
            Some(f64::from_bits(u64::from_be_bytes([
                b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
            ])))
        }
    }
}

fn attr_text(das: &DapAttrs, var: &str, key: &str) -> Option<String> {
    let list = attr_list(das, var)?;
    let a = list.iter().find(|a| a.name == key)?;
    match a.dap_type {
        DapType::Str | DapType::Url => Some(String::from_utf8_lossy(&a.raw).into_owned()),
        _ => None,
    }
}

fn var_attrs(das: &DapAttrs, var: &str) -> Result<VarAttrs, String> {
    let list = attr_list(das, var)
        .ok_or_else(|| format!("the DAS carries no attribute block for '{var}' — no fill gate"))?;
    let fill = match num_attr(list, "_FillValue") {
        Some(f) => Some(f),
        None => num_attr(list, "missing_value"),
    };
    let fill = fill.ok_or_else(|| {
        format!("'{var}' carries neither _FillValue nor missing_value — the value stays ungated")
    })?;
    Ok(VarAttrs {
        fill,
        valid_min: num_attr(list, "valid_min"),
        valid_max: num_attr(list, "valid_max"),
    })
}

fn ascii_var<'a>(vars: &'a [AsciiVar], name: &str) -> Option<&'a AsciiVar> {
    vars.iter()
        .find(|v| v.name == name || v.name.rsplit('.').next() == Some(name))
}

fn gate(raw: f64, a: &VarAttrs, c2k: bool) -> Option<f64> {
    if !raw.is_finite() || raw == a.fill {
        return None;
    }
    if let Some(min) = a.valid_min {
        if raw < min {
            return None;
        }
    }
    if let Some(max) = a.valid_max {
        if raw > max {
            return None;
        }
    }
    if c2k { Some(raw + 273.15) } else { Some(raw) }
}

fn observations(
    ascii: &str,
    source: &str,
    depth_name: Option<&str>,
    qc_name: Option<&str>,
    time_units: &str,
    attrs: &VarAttrs,
    c2k: bool,
) -> Result<Vec<Obs>, String> {
    let vars = parse_ascii(ascii).map_err(|e| format!("DAP2-ASCII parse {e:?}"))?;
    let target =
        ascii_var(&vars, source).ok_or_else(|| format!("no '{source}' in the DAP2-ASCII body"))?;
    let time =
        ascii_var(&vars, "TIME").ok_or_else(|| "no 'TIME' in the DAP2-ASCII body".to_string())?;
    let inst = ascii_var(&vars, "instrument_index")
        .ok_or_else(|| "no 'instrument_index' in the DAP2-ASCII body".to_string())?;
    let lat = ascii_var(&vars, "LATITUDE")
        .ok_or_else(|| "no 'LATITUDE' in the DAP2-ASCII body".to_string())?;
    let lon = ascii_var(&vars, "LONGITUDE")
        .ok_or_else(|| "no 'LONGITUDE' in the DAP2-ASCII body".to_string())?;
    let depth = match depth_name {
        Some(d) => {
            Some(ascii_var(&vars, d).ok_or_else(|| format!("no '{d}' in the DAP2-ASCII body"))?)
        }
        None => None,
    };
    let qc = match qc_name {
        Some(q) => {
            Some(ascii_var(&vars, q).ok_or_else(|| format!("no '{q}' in the DAP2-ASCII body"))?)
        }
        None => None,
    };
    let n = target
        .values
        .len()
        .min(time.values.len())
        .min(inst.values.len());
    let mut out = Vec::new();
    for i in 0..n {
        let Some(val) = gate(target.values[i], attrs, c2k) else {
            continue;
        };
        if let Some(q) = qc {
            if let Some(&f) = q.values.get(i) {
                if f == 3.0 || f == 4.0 || f == 9.0 {
                    continue;
                }
            }
        }
        let Some(unix) = cf_time_unix_seconds(time_units, time.values[i]) else {
            continue;
        };
        let alt = match depth {
            Some(d) => {
                let Some(&dv) = d.values.get(i) else {
                    continue;
                };
                if !dv.is_finite() || dv == attrs.fill {
                    continue;
                }
                -dv
            }
            None => 0.0,
        };
        let ii = inst.values[i];
        if !ii.is_finite() || ii < 0.0 {
            continue;
        }
        let idx = ii as usize;
        let Some(&la) = lat.values.get(idx) else {
            continue;
        };
        let Some(&lo) = lon.values.get(idx) else {
            continue;
        };
        if !la.is_finite()
            || !lo.is_finite()
            || !(-90.0..=90.0).contains(&la)
            || !(-180.0..=180.0).contains(&lo)
        {
            continue;
        }
        out.push(Obs {
            unix,
            val,
            lat: la,
            lon: lo,
            alt,
        });
    }
    Ok(out)
}

fn cadence_seconds(obs: &[Obs]) -> Option<f64> {
    if obs.len() < 2 {
        return None;
    }
    let mut t: Vec<f64> = obs.iter().map(|o| o.unix).collect();
    t.sort_by(|a, b| a.total_cmp(b));
    let mut d: Vec<f64> = Vec::new();
    for w in t.windows(2) {
        let dt = w[1] - w[0];
        if dt > 0.0 && dt.is_finite() {
            d.push(dt);
        }
    }
    if d.is_empty() {
        return None;
    }
    d.sort_by(|a, b| a.total_cmp(b));
    let m = d.len() / 2;
    if d.len() % 2 == 1 {
        Some(d[m])
    } else {
        Some((d[m - 1] + d[m]) / 2.0)
    }
}

fn record(
    pos: [f64; 3],
    val: f64,
    tdb: f64,
    ttl: f64,
    tau: f64,
    kernel: f64,
    force: f64,
) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = ttl;
    r[6] = tau;
    r[7] = 0.0;
    r[8] = kernel;
    r[9] = force;
    r[25] = 1.0;
    r
}

fn write_bin(records: &[[f64; 26]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn read_bin(data: &[u8]) -> Option<usize> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * REC_BYTES {
        return None;
    }
    Some(count)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let source = arg_value(args, "--var")
        .ok_or_else(|| format!("--var <source> is required\n{}", usage()))?;
    let fm = field_of(&source).ok_or_else(|| {
        let known: Vec<&str> = FIELDS.iter().map(|f| f.source).collect();
        format!("'{source}' carries no field mapping — {}", known.join("/"))
    })?;
    let body = arg_value(args, "--body").ok_or_else(|| {
        "--body <name> is required — the receiver body is declared, never defaulted".to_string()
    })?;
    let out_dir = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}"),
    };
    let start = match arg_value(args, "--start") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--start '{v}' is not an index"))?,
        None => 0,
    };
    let want = match arg_value(args, "--count") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--count '{v}' is not a count"))?,
        None => DEFAULT_COUNT,
    };
    if want == 0 {
        return Err("--count 0 carries no observation".to_string());
    }

    let base = base_url(args)?;

    let dds_url = format!("{base}.dds");
    let dds_text = fetch_text(&dds_url)?;
    let schema = parse_dds(&dds_text).map_err(|e| format!("{dds_url}: DDS parse {e:?}"))?;
    let obs_len = var_len(&schema, &source).ok_or_else(|| {
        format!("'{source}' is absent or not a one-dimensional OBSERVATION array in the DDS — the gridded path stays pending")
    })?;
    if start >= obs_len {
        return Err(format!(
            "--start {start} lies beyond the {obs_len} observations"
        ));
    }
    let count = want.min(obs_len - start);
    let last = start + count - 1;

    let depth_name = if is_1d(&schema, "DEPTH") {
        Some("DEPTH")
    } else if is_1d(&schema, "PRES") {
        Some("PRES")
    } else {
        None
    };
    let qc_candidate = format!("{source}_quality_control");
    let qc_name = if is_1d(&schema, &qc_candidate) {
        Some(qc_candidate)
    } else {
        None
    };

    if !is_1d(&schema, "LATITUDE")
        || !is_1d(&schema, "LONGITUDE")
        || !is_1d(&schema, "instrument_index")
    {
        return Err(format!(
            "{base}: the discrete-sampling instrument arrays (LATITUDE/LONGITUDE/instrument_index) are absent — the gridded path stays pending"
        ));
    }
    let ninst = var_len(&schema, "LATITUDE")
        .ok_or_else(|| "LATITUDE carries no INSTRUMENT extent".to_string())?;
    if ninst == 0 {
        return Err(format!("{base}: the INSTRUMENT extent reads 0"));
    }

    let das_url = format!("{base}.das");
    let das_text = fetch_text(&das_url)?;
    let das = parse_das(&das_text).map_err(|e| format!("{das_url}: DAS parse {e:?}"))?;
    let attrs = var_attrs(&das, &source)?;
    let time_units = attr_text(&das, "TIME", "units")
        .ok_or_else(|| format!("{base}: TIME carries no units — the clock stays unread"))?;

    let slice = |name: &str| format!("{name}%5B{start}:1:{last}%5D");
    let mut proj = vec![slice(&source), slice("TIME"), slice("instrument_index")];
    if let Some(d) = depth_name {
        proj.push(slice(d));
    }
    if let Some(q) = qc_name.as_deref() {
        proj.push(slice(q));
    }
    proj.push(format!("LATITUDE%5B0:1:{}%5D", ninst - 1));
    proj.push(format!("LONGITUDE%5B0:1:{}%5D", ninst - 1));
    let ascii_url = format!("{base}.ascii?{}", proj.join(","));
    let ascii_text = fetch_text(&ascii_url)?;

    let obs = observations(
        &ascii_text,
        &source,
        depth_name,
        qc_name.as_deref(),
        &time_units,
        &attrs,
        fm.celsius_to_kelvin,
    )?;
    if obs.is_empty() {
        return Err(format!(
            "{base}: no measured observation left the harvest ({count} read) — the asset stays unwritten (0 honored)"
        ));
    }
    let cadence = cadence_seconds(&obs).ok_or_else(|| {
        format!(
            "{base}: the {}-observation window carries no measurable time step — the cadence stays unread",
            obs.len()
        )
    })?;

    let kernel_id = kernel_id_of(fm.kernel)
        .ok_or_else(|| format!("kernel '{}' carries no id", fm.kernel))? as f64;
    let force_id =
        force_id_of(fm.force).ok_or_else(|| format!("force '{}' carries no id", fm.force))? as f64;

    let lsk = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no date→TDB step".to_string())?;
    let eph_bytes = match arg_value(args, "--ephemeris") {
        Some(src) if src.starts_with("http") => {
            fetch_raw_bytes(&src).ok_or_else(|| format!("body ephemeris fetch void ({src})"))?
        }
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("body ephemeris read {path}: {e}"))?
        }
        None => fetch_raw_bytes(&body_url(&body))
            .ok_or_else(|| "body ephemeris fetch void (CDN) — no ICRS frame".to_string())?,
    };
    let body_eph = parse_ephemeris_binary(&eph_bytes)
        .ok_or_else(|| "the body ephemeris binary stays unread — no ICRS frame".to_string())?;
    let eph: HashMap<String, BodyEphemeris> = HashMap::from([(body.clone(), body_eph)]);

    let mut records: Vec<[f64; 26]> = Vec::with_capacity(obs.len());
    let mut clock_void = 0usize;
    let mut frame_void = 0usize;
    for o in &obs {
        let Some(tdb) = lsk.unix_to_tdb(o.unix) else {
            clock_void += 1;
            continue;
        };
        let Some(pos) = body_fixed_to_icrs(&body, o.lat, o.lon, o.alt, tdb, &eph) else {
            frame_void += 1;
            continue;
        };
        records.push(record(
            pos, o.val, tdb, cadence, cadence, kernel_id, force_id,
        ));
    }
    if records.is_empty() {
        return Err(format!(
            "{base}: no observation reached the ICRS frame — clock_void {clock_void}, frame_void {frame_void}"
        ));
    }

    std::fs::create_dir_all(&out_dir)
        .map_err(|e| format!("create {out_dir} returned void: {e}"))?;
    let asset = format!(
        "aodn_{}_{}_{}.bin",
        source.to_ascii_lowercase(),
        start,
        count
    );
    let path = format!("{out_dir}/{asset}");
    let bin = write_bin(&records);
    std::fs::write(&path, &bin).map_err(|e| format!("write {path} returned void: {e}"))?;
    let roundtrip = read_bin(&bin)
        .ok_or_else(|| format!("{path}: roundtrip parse void — the asset stays unverified"))?;

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format aodn");
    println!("origin {base}");
    println!("compiler {COMPILER}");
    println!("at {body}");
    println!("ttl {}", cadence.round() as u64);
    println!(
        "field {} {} {} {} {} {} 0.0 0.0",
        fm.key,
        fm.key,
        fm.kernel,
        fm.force,
        fm.unit,
        cadence.round() as u64
    );
    println!("sha256 {}", sha256_hex(&bin));

    eprintln!(
        "aodn_compiler: {source} {} observations -> {path} (roundtrip {roundtrip}); cadence {cadence:.1} s; clock_void {clock_void}, frame_void {frame_void}",
        records.len()
    );
    if ci_mode && !upload_release(NETLOC, &path) {
        return Err(format!("{path}: CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let attrs = VarAttrs {
        fill: 99999.0,
        valid_min: Some(-2.5),
        valid_max: Some(40.0),
    };
    let ascii = "Dataset {\n    Float32 TEMP[OBSERVATION = 4];\n} demo.nc;\n\
        ---------------------------------------------\n\
        TEMP[4]\n14.0, 16.0, 99999.0, 18.0\n\n\
        TEMP_quality_control[4]\n1, 4, 1, 1\n\n\
        DEPTH[4]\n85.0, 85.0, 85.0, 85.0\n\n\
        TIME[4]\n21285.0, 21285.5, 21286.0, 21286.5\n\n\
        instrument_index[4]\n0, 0, 0, 0\n\n\
        LATITUDE[1]\n-42.6\n\n\
        LONGITUDE[1]\n148.2\n\n";
    let obs = match observations(
        ascii,
        "TEMP",
        Some("DEPTH"),
        Some("TEMP_quality_control"),
        "days since 1950-01-01 00:00:00 UTC",
        &attrs,
        true,
    ) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("selftest: observations void: {e}");
            std::process::exit(1);
        }
    };
    if obs.len() != 2 {
        eprintln!(
            "selftest: {} observations left the gate, not the measured 2 (fill and qc=4 removed)",
            obs.len()
        );
        std::process::exit(1);
    }
    if (obs[0].val - 287.15).abs() > 1e-9 || (obs[1].val - 291.15).abs() > 1e-9 {
        eprintln!("selftest: the Kelvin conversion is not the measured offset");
        std::process::exit(1);
    }
    if obs[0].lat != -42.6 || obs[0].lon != 148.2 || obs[0].alt != -85.0 {
        eprintln!("selftest: the instrument geometry is not the measured point");
        std::process::exit(1);
    }
    let cadence = match cadence_seconds(&obs) {
        Some(c) => c,
        None => {
            eprintln!("selftest: cadence void");
            std::process::exit(1);
        }
    };
    if (cadence - 129600.0).abs() > 1e-6 {
        eprintln!("selftest: cadence {cadence} is not the measured 1.5-day step");
        std::process::exit(1);
    }
    if gate(99999.0, &attrs, true).is_some() || gate(41.0, &attrs, true).is_some() {
        eprintln!("selftest: the fill and the out-of-range value did not close the gate");
        std::process::exit(1);
    }
    eprintln!(
        "aodn_compiler: selftest passes (DAP2-ASCII ragged cell → gated Kelvin observations + cadence)"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if let Err(msg) = run(&args) {
        eprintln!("aodn_compiler: {msg}");
        std::process::exit(2);
    }
}
