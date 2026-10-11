use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::hapi_csv::parse_iso_seconds;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::body_url;
use omegaflow::cdn::upload_release;
use omegaflow::force::{force_id_of, kernel_id_for_force};
use omegaflow::hdf5::Hdf5File;
use omegaflow::json::{JsonVal, parse_json};
use std::collections::HashMap;
use std::f64::consts::FRAC_PI_2;
use std::f64::consts::FRAC_PI_4;

const NETLOC: &str = "stac.itslive.cloud";
const FORMAT: &str = "its_live_velocity";
const COMPILER: &str = "tools/harvest/src/bin/its_live_compiler.rs";
const DEFAULT_ITEMS: &str =
    "https://stac.itslive.cloud/collections/itslive-granules/items?limit=10";
const FORCE_NAME: &str = "advective";
const UNIT: &str = "m/yr";

const FRAME_MAGIC: [u8; 3] = [0xCF, 0x86, 0x09];
const REC_BYTES: usize = 26 * 8;
const FRAME_HEADER: usize = 3 + 8 + 4 + 4;
const TTL_S: f64 = 31_536_000.0;
const TAU_S: f64 = 604_800.0;

const REFERENCE_SURFACE_ALT: f64 = 0.0;

fn usage() -> &'static str {
    "usage: its_live_compiler [--items <url|file>] [--item <url|file>] [--out <file.bin>] \
     [--stride N] [--limit N] [--ephemeris <path|url>] [--probe] [--ci-mode]"
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn obj(v: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match v {
        JsonVal::Obj(m) => Some(m),
        _ => None,
    }
}

fn str_of(v: Option<&JsonVal>) -> Option<String> {
    match v {
        Some(JsonVal::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

fn num_of(v: Option<&JsonVal>) -> Option<f64> {
    match v {
        Some(JsonVal::Num(n)) if n.is_finite() => Some(*n),
        _ => None,
    }
}

fn asset_href(v: &JsonVal) -> Option<String> {
    obj(v).and_then(|o| str_of(o.get("href")))
}

fn asset_is_netcdf(v: &JsonVal) -> bool {
    let Some(o) = obj(v) else {
        return false;
    };
    let media = str_of(o.get("type")).is_some_and(|m| m.contains("netcdf"));
    let href = str_of(o.get("href")).is_some_and(|h| h.ends_with(".nc") || h.contains(".nc?"));
    media || href
}

fn collect_points(v: &JsonVal, out: &mut Vec<(f64, f64)>) {
    if let JsonVal::Arr(a) = v {
        if a.len() >= 2 && a.iter().all(|x| matches!(x, JsonVal::Num(_))) {
            if let (Some(lon), Some(lat)) = (num_of(a.first()), num_of(a.get(1))) {
                out.push((lon, lat));
            }
        } else {
            for e in a {
                collect_points(e, out);
            }
        }
    }
}

fn geometry_centroid(geometry: &JsonVal) -> Option<(f64, f64)> {
    let coords = obj(geometry).and_then(|g| g.get("coordinates"))?;
    let mut pts = Vec::new();
    collect_points(coords, &mut pts);
    if pts.is_empty() {
        return None;
    }
    let n = pts.len() as f64;
    let lon = pts.iter().map(|p| p.0).sum::<f64>() / n;
    let lat = pts.iter().map(|p| p.1).sum::<f64>() / n;
    if lon.is_finite() && lat.is_finite() {
        Some((lon, lat))
    } else {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Item {
    id: String,
    t_epoch: Option<f64>,
    lon: Option<f64>,
    lat: Option<f64>,
    data_href: Option<String>,
}

fn parse_item(v: &JsonVal) -> Option<Item> {
    let m = obj(v)?;
    let id = str_of(m.get("id"))?;
    let props = m.get("properties").and_then(obj);
    let t_epoch = props
        .and_then(|p| str_of(p.get("mid_datetime")).or_else(|| str_of(p.get("datetime"))))
        .and_then(|s| parse_iso_seconds(&s));
    let (mut lon, mut lat) = (
        props.and_then(|p| num_of(p.get("longitude"))),
        props.and_then(|p| num_of(p.get("latitude"))),
    );
    if lon.is_none() || lat.is_none() {
        if let Some((lo, la)) = m.get("geometry").and_then(geometry_centroid) {
            lon = Some(lo);
            lat = Some(la);
        }
    }
    let data_href = m.get("assets").and_then(obj).and_then(|assets| {
        assets.get("data").and_then(asset_href).or_else(|| {
            assets.iter().find_map(|(k, av)| {
                if k == "data" || asset_is_netcdf(av) {
                    asset_href(av)
                } else {
                    None
                }
            })
        })
    });
    Some(Item {
        id,
        t_epoch,
        lon,
        lat,
        data_href,
    })
}

fn entries_of(json: &JsonVal) -> Vec<JsonVal> {
    match obj(json).and_then(|m| m.get("features")) {
        Some(JsonVal::Arr(a)) => a.clone(),
        _ => vec![json.clone()],
    }
}

fn source_bytes(src: &str) -> Result<Vec<u8>, String> {
    if src.starts_with("http://") || src.starts_with("https://") {
        fetch_raw_bytes(src).ok_or_else(|| format!("{src}: fetch returned void"))
    } else {
        std::fs::read(src).map_err(|e| format!("read {src} returned void: {e}"))
    }
}

fn read_var(file: &Hdf5File, name: &str) -> Option<Vec<f64>> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_f64_dataset(name)))
        .ok()?
        .ok()
}

fn attr_scalar(a: &omegaflow::hdf5::Hdf5Attribute) -> Option<f64> {
    let b = &a.data;
    match a.datatype.class {
        0 => match a.datatype.size {
            1 => b.first().map(|&x| x as i8 as f64),
            2 => b.get(0..2).map(|x| i16::from_le_bytes([x[0], x[1]]) as f64),
            4 => b
                .get(0..4)
                .map(|x| i32::from_le_bytes([x[0], x[1], x[2], x[3]]) as f64),
            8 => b
                .get(0..8)
                .and_then(|x| x.try_into().ok())
                .map(|x| i64::from_le_bytes(x) as f64),
            _ => None,
        },
        1 => match a.datatype.size {
            4 => omegaflow::hdf5::decode_f32(b, 0, a.datatype.endian).map(f64::from),
            8 => omegaflow::hdf5::decode_f64(b, 0, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
}

fn fill_of(file: &Hdf5File, name: &str) -> Option<f64> {
    file.attribute(name, "_FillValue").and_then(attr_scalar)
}

fn keep_velocity(v: f64, fill: Option<f64>) -> bool {
    if !v.is_finite() {
        return false;
    }
    if let Some(f) = fill {
        if f.is_finite() && v == f {
            return false;
        }
    }
    v >= 0.0
}

fn t_of(phi: f64, e: f64, north: bool) -> f64 {
    let angle = if north {
        FRAC_PI_4 - phi / 2.0
    } else {
        FRAC_PI_4 + phi / 2.0
    };
    let es = e * phi.sin();
    angle.tan() * ((1.0 + es) / (1.0 - es)).powf(e / 2.0)
}

fn m_of(phi: f64, e: f64) -> f64 {
    let s = phi.sin();
    phi.cos() / (1.0 - e * e * s * s).sqrt()
}

#[derive(Clone, Copy)]
struct PolarStereo {
    a: f64,
    e: f64,
    lon0_deg: f64,
    lat_ts_deg: f64,
    north: bool,
}

impl PolarStereo {
    fn from_file(file: &Hdf5File) -> Option<PolarStereo> {
        let a = file.attr_f64("mapping", "semi_major_axis")?;
        let inv_f = file.attr_f64("mapping", "inverse_flattening")?;
        let lon0_deg = file.attr_f64("mapping", "straight_vertical_longitude_from_pole")?;
        let lat_ts_deg = file.attr_f64("mapping", "latitude_of_origin")?;
        if !(a > 0.0) || !(inv_f > 0.0) || !lon0_deg.is_finite() || !lat_ts_deg.is_finite() {
            return None;
        }
        let f = 1.0 / inv_f;
        let e = (f * (2.0 - f)).sqrt();
        if !e.is_finite() {
            return None;
        }
        Some(PolarStereo {
            a,
            e,
            lon0_deg,
            lat_ts_deg,
            north: lat_ts_deg >= 0.0,
        })
    }

    fn inverse(&self, x: f64, y: f64) -> (f64, f64) {
        let lat_ts = self.lat_ts_deg.to_radians();
        let t_c = t_of(lat_ts, self.e, self.north);
        let m_c = m_of(lat_ts, self.e);
        let rho = (x * x + y * y).sqrt();
        if rho < 1e-9 {
            let lat = if self.north { 90.0 } else { -90.0 };
            return (self.lon0_deg, lat);
        }
        let t = rho * t_c / (self.a * m_c);
        let mut phi = if self.north {
            FRAC_PI_2 - 2.0 * t.atan()
        } else {
            -(FRAC_PI_2 - 2.0 * t.atan())
        };
        for _ in 0..16 {
            let es = self.e * phi.sin();
            let angle = t * ((1.0 - es) / (1.0 + es)).powf(self.e / 2.0);
            let next = if self.north {
                FRAC_PI_2 - 2.0 * angle.atan()
            } else {
                -(FRAC_PI_2 - 2.0 * angle.atan())
            };
            if (next - phi).abs() < 1e-13 {
                phi = next;
                break;
            }
            phi = next;
        }
        let theta = if self.north { x.atan2(-y) } else { x.atan2(y) };
        let lon = self.lon0_deg + theta.to_degrees();
        let lon = ((lon + 540.0) % 360.0) - 180.0;
        (lon, phi.to_degrees())
    }
}

fn record(pos: [f64; 3], val: f64, tdb: f64, kernel: f64, force: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = 0.0;
    r[8] = kernel;
    r[9] = force;
    r[25] = 0.0;
    r
}

fn write_bin(records: &[[f64; 26]], response_epoch: f64, id: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(FRAME_HEADER + records.len() * REC_BYTES);
    out.extend_from_slice(&FRAME_MAGIC);
    out.extend_from_slice(&response_epoch.to_le_bytes());
    out.extend_from_slice(&id.to_le_bytes());
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn verify_bin(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < FRAME_HEADER || bytes[0..3] != FRAME_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[15..19].try_into().ok()?) as usize;
    if bytes.len() != FRAME_HEADER + count * REC_BYTES {
        return None;
    }
    Some(count)
}

struct Compiled {
    records: Vec<[f64; 26]>,
    cells: u64,
    fill_skips: u64,
    negative_skips: u64,
    projection_void: u64,
    frame_void: u64,
}

fn compile_item(
    item: &Item,
    file: &Hdf5File,
    tdb: f64,
    eph: &HashMap<String, BodyEphemeris>,
    receiver: &str,
    stride: usize,
    limit: usize,
) -> Result<Compiled, String> {
    let Some(v) = read_var(file, "v") else {
        return Err(format!("{}: carries no 'v' velocity dataset", item.id));
    };
    let fill = fill_of(file, "v");
    let xs = read_var(file, "x").ok_or_else(|| format!("{}: carries no 'x' axis", item.id))?;
    let ys = read_var(file, "y").ok_or_else(|| format!("{}: carries no 'y' axis", item.id))?;
    let stereo = PolarStereo::from_file(file)
        .ok_or_else(|| format!("{}: the 'mapping' group carries no projection", item.id))?;
    let dims = file
        .dims("v")
        .ok_or_else(|| format!("{}: the 'v' dataset carries no dims", item.id))?;
    if dims.len() < 2 {
        return Err(format!("{}: 'v' is not a 2D grid", item.id));
    }
    let ny = dims[dims.len() - 2] as usize;
    let nx = dims[dims.len() - 1] as usize;
    if xs.len() < nx || ys.len() < ny {
        return Err(format!(
            "{}: the {nx}x{ny} grid does not meet the x/y axes ({}x{})",
            item.id,
            xs.len(),
            ys.len()
        ));
    }
    let (kernel, force) = ids_of(FORCE_NAME)?;
    let mut records = Vec::new();
    let mut cells = 0u64;
    let mut fill_skips = 0u64;
    let mut negative_skips = 0u64;
    let mut projection_void = 0u64;
    let mut frame_void = 0u64;
    let mut row = 0usize;
    while row < ny {
        let mut col = 0usize;
        while col < nx {
            let idx = row * nx + col;
            let val = match v.get(idx) {
                Some(val) => *val,
                None => {
                    fill_skips += 1;
                    col += stride;
                    continue;
                }
            };
            if !keep_velocity(val, fill) {
                if val.is_finite() && val < 0.0 {
                    negative_skips += 1;
                } else {
                    fill_skips += 1;
                }
                col += stride;
                continue;
            }
            let (lon, lat) = stereo.inverse(xs[col], ys[row]);
            if !lon.is_finite() || !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
                projection_void += 1;
                col += stride;
                continue;
            }
            match body_fixed_to_icrs(receiver, lat, lon, REFERENCE_SURFACE_ALT, tdb, eph) {
                Some(pos) => {
                    records.push(record(pos, val, tdb, f64::from(kernel), f64::from(force)));
                    cells += 1;
                    if records.len() >= limit {
                        return Ok(Compiled {
                            records,
                            cells,
                            fill_skips,
                            negative_skips,
                            projection_void,
                            frame_void,
                        });
                    }
                }
                None => frame_void += 1,
            }
            col += stride;
        }
        row += stride;
    }
    Ok(Compiled {
        records,
        cells,
        fill_skips,
        negative_skips,
        projection_void,
        frame_void,
    })
}

fn ids_of(force_name: &str) -> Result<(u8, u8), String> {
    let force =
        force_id_of(force_name).ok_or_else(|| format!("force '{force_name}' is not registered"))?;
    let kernel = kernel_id_for_force(force)
        .ok_or_else(|| format!("force '{force_name}' carries no kernel"))?;
    Ok((kernel, force))
}

fn field_line(force_name: &str) -> Result<String, String> {
    let kernel_name = kernel_name_for(force_name)?;
    Ok(format!(
        "field {} {} {} {} {} {} 0.0 0.0",
        "its_live_velocity", "its_live_velocity", kernel_name, force_name, UNIT, TAU_S as u64
    ))
}

fn kernel_name_for(force_name: &str) -> Result<&'static str, String> {
    omegaflow::force::default_kernel_for(force_name)
        .map(|(k, _)| k)
        .ok_or_else(|| format!("force '{force_name}' carries no default kernel"))
}

fn probe(file: &Hdf5File, item: &Item) {
    eprintln!("probe item {}", item.id);
    for name in ["v", "vx", "vy", "x", "y", "time", "mapping"] {
        match file.dims(name) {
            Some(dims) => eprintln!("  {name}: dims {dims:?}"),
            None => eprintln!("  {name}: absent"),
        }
    }
    if let Some(v) = read_var(file, "v") {
        let fill = fill_of(file, "v");
        let mut n = 0u64;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        let mut sum = 0.0;
        for x in &v {
            if keep_velocity(*x, fill) {
                n += 1;
                min = min.min(*x);
                max = max.max(*x);
                sum += *x;
            }
        }
        let mean = if n > 0 { sum / n as f64 } else { 0.0 };
        eprintln!("  v: fill {fill:?}, kept {n}, min {min}, max {max}, mean {mean}");
    }
    if let (Some(xs), Some(ys)) = (read_var(file, "x"), read_var(file, "y")) {
        if let (Some(x0), Some(x1)) = (xs.first(), xs.last()) {
            if let (Some(y0), Some(y1)) = (ys.first(), ys.last()) {
                eprintln!("  x: {x0} .. {x1} ({} values)", xs.len());
                eprintln!("  y: {y0} .. {y1} ({} values)", ys.len());
                if let Some(st) = PolarStereo::from_file(file) {
                    let xc = (x0 + x1) / 2.0;
                    let yc = (y0 + y1) / 2.0;
                    let (lon, lat) = st.inverse(xc, yc);
                    eprintln!(
                        "  grid centre ({xc}, {yc}) -> lat {lat}, lon {lon} (item centroid {:?}, {:?})",
                        item.lat, item.lon
                    );
                }
            }
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let items_src = match arg_value(args, "--item").or_else(|| arg_value(args, "--items")) {
        Some(v) => v,
        None => DEFAULT_ITEMS.to_string(),
    };
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => "data/its_live_velocity.bin".to_string(),
    };
    let stride = parse_step(args, "--stride", 1)?;
    let limit = parse_step(args, "--limit", usize::MAX)?;
    let probe_mode = args.iter().any(|a| a == "--probe");
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let receiver = arg_value(args, "--body").ok_or_else(|| {
        "--body <name> is required — the receiver body is declared, never defaulted".to_string()
    })?;

    let body = source_bytes(&items_src)?;
    let json = parse_json(
        std::str::from_utf8(&body).map_err(|_| format!("{items_src}: the body is not UTF-8"))?,
    )
    .ok_or_else(|| format!("{items_src}: carries no json"))?;
    let entries = entries_of(&json);
    if entries.is_empty() {
        return Err(format!("{items_src}: carries no STAC item"));
    }

    let lsk: LeapSeconds = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no TDB clock".to_string())?;
    let eph_bytes = match arg_value(args, "--ephemeris") {
        Some(src) if src.starts_with("http") => {
            fetch_raw_bytes(&src).ok_or_else(|| format!("earth ephemeris fetch void ({src})"))?
        }
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("earth ephemeris read {path}: {e}"))?
        }
        None => fetch_raw_bytes(&body_url(&receiver))
            .ok_or_else(|| "receiver ephemeris fetch void (CDN)".to_string())?,
    };
    let earth = parse_ephemeris_binary(&eph_bytes)
        .ok_or_else(|| "the receiver ephemeris binary stays unread — no ICRS frame".to_string())?;
    let eph = HashMap::from([(receiver.clone(), earth)]);

    if probe_mode {
        for entry in &entries {
            let Some(item) = parse_item(entry) else {
                continue;
            };
            let Some(href) = &item.data_href else {
                eprintln!("probe item {}: carries no netcdf asset", item.id);
                continue;
            };
            let bytes = source_bytes(href)?;
            let file = Hdf5File::parse(&bytes).map_err(|e| format!("{href}: {e:?}"))?;
            probe(&file, &item);
        }
        return Ok(());
    }

    let mut records: Vec<[f64; 26]> = Vec::new();
    let mut response_epoch = 0.0f64;
    let mut skipped = 0usize;
    for entry in &entries {
        let Some(item) = parse_item(entry) else {
            skipped += 1;
            continue;
        };
        let (Some(href), Some(t_epoch)) = (item.data_href.clone(), item.t_epoch) else {
            skipped += 1;
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(t_epoch) else {
            skipped += 1;
            continue;
        };
        let bytes = match source_bytes(&href) {
            Ok(b) => b,
            Err(msg) => {
                eprintln!("its_live: {msg}");
                skipped += 1;
                continue;
            }
        };
        let file = match Hdf5File::parse(&bytes) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("its_live: {href}: {e:?}");
                skipped += 1;
                continue;
            }
        };
        let compiled = match compile_item(&item, &file, tdb, &eph, &receiver, stride, limit) {
            Ok(c) => c,
            Err(msg) => {
                eprintln!("its_live: {msg}");
                skipped += 1;
                continue;
            }
        };
        eprintln!(
            "its_live: {} -> {} cells (fill {}, negative {}, projection_void {}, frame_void {})",
            item.id,
            compiled.cells,
            compiled.fill_skips,
            compiled.negative_skips,
            compiled.projection_void,
            compiled.frame_void
        );
        if response_epoch == 0.0 && !compiled.records.is_empty() {
            response_epoch = tdb;
        }
        records.extend(compiled.records);
        if records.len() >= limit {
            break;
        }
    }

    if records.is_empty() {
        return Err(format!(
            "{items_src}: no measured velocity cell left the harvest — the bin stays unwritten (0 honored; {skipped} items skipped)"
        ));
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    let bin = write_bin(&records, response_epoch, 0);
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    let count = verify_bin(&bin).ok_or_else(|| {
        format!("{out}: roundtrip parse returned void — the bin stays unverified")
    })?;
    if count != records.len() {
        return Err(format!(
            "{out}: {count} parsed against {} written — the bin stays unverified",
            records.len()
        ));
    }

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {items_src}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("at {receiver}");
    println!("ttl {}", TTL_S as u64);
    println!("{}", field_line(FORCE_NAME)?);
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} velocity cells, {} B, roundtrip parses",
        records.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("{}", usage());
        eprintln!("  reads ITS_LIVE glacier velocity STAC items and their netCDF-4 granules");
        eprintln!("  default items {DEFAULT_ITEMS}");
        eprintln!("  --probe prints the measured netCDF grid and stops");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("its_live_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ITEM: &str = r#"{
      "type":"Feature",
      "id":"S1C_IW_SLC__1SSV_20261005T050325_20261005T050336_009745_013689_DD37_X_S1D_IW_SLC__1SSV_20261011T050337_20261011T050348_004963_0095CF_3B06_G0120V02_P084",
      "bbox":[18.435,68.9192,24.0756,70.5904],
      "properties":{
        "datetime":"2026-10-08T05:03:33.062588Z",
        "mid_datetime":"2026-10-08T05:03:33.062588Z",
        "longitude":21.2797,
        "latitude":69.761,
        "percent_valid_pixels":84,
        "proj:code":"EPSG:3413"
      },
      "geometry":{"type":"Polygon","coordinates":[[[18.435,68.9192],[24.0756,68.9192],[24.0756,70.5904],[18.435,70.5904],[18.435,68.9192]]]},
      "assets":{
        "data":{"href":"https://its-live-data.s3.amazonaws.com/velocity_image_pair/sentinel1/v02/N60E020/x_G0120V02_P084.nc","type":"application/netcdf","roles":["data"]},
        "overview":{"href":"https://its-live-data.s3.amazonaws.com/x.png","type":"image/png","roles":["overview"]}
      }
    }"#;

    #[test]
    fn stac_item_json_reads_the_record_fields() {
        let json = parse_json(ITEM).expect("item parses");
        let record = parse_item(&json).expect("record fields extract");
        assert_eq!(
            record.id,
            "S1C_IW_SLC__1SSV_20261005T050325_20261005T050336_009745_013689_DD37_X_S1D_IW_SLC__1SSV_20261011T050337_20261011T050348_004963_0095CF_3B06_G0120V02_P084"
        );
        assert_eq!(record.lon, Some(21.2797));
        assert_eq!(record.lat, Some(69.761));
        let t = record.t_epoch.expect("mid_datetime reads");
        assert!((t - 1_791_435_813.062588).abs() < 1e-3, "t {t}");
        assert_eq!(
            record.data_href.as_deref(),
            Some(
                "https://its-live-data.s3.amazonaws.com/velocity_image_pair/sentinel1/v02/N60E020/x_G0120V02_P084.nc"
            )
        );
    }

    #[test]
    fn absent_fields_stay_none_not_fabricated_zero() {
        let json = parse_json(r#"{"id":"x","properties":{},"assets":{}}"#).expect("json parses");
        let record = parse_item(&json).expect("id carries the item");
        assert_eq!(record.t_epoch, None);
        assert_eq!(record.lon, None);
        assert_eq!(record.lat, None);
        assert_eq!(record.data_href, None);
    }

    #[test]
    fn geometry_centroid_is_the_mean_vertex() {
        let json = parse_json(ITEM).expect("item parses");
        let geometry = obj(&json)
            .and_then(|m| m.get("geometry"))
            .expect("geometry present");
        let (lon, lat) = geometry_centroid(geometry).expect("centroid");
        assert!((lon - 20.69124).abs() < 1e-6, "lon {lon}");
        assert!((lat - 69.58768).abs() < 1e-6, "lat {lat}");
    }

    #[test]
    fn zero_velocity_is_honored_and_fill_is_absent() {
        assert!(keep_velocity(0.0, Some(-32767.0)));
        assert!(keep_velocity(123.4, Some(-32767.0)));
        assert!(!keep_velocity(-32767.0, Some(-32767.0)));
        assert!(!keep_velocity(-1.0, None));
        assert!(!keep_velocity(f64::NAN, None));
        assert!(!keep_velocity(f64::INFINITY, None));
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let kernel = ids_of(FORCE_NAME).expect("advective ids").0;
        let force = ids_of(FORCE_NAME).expect("advective ids").1;
        let r = record(
            [1.0, 2.0, 3.0],
            0.0,
            8.0e8,
            f64::from(kernel),
            f64::from(force),
        );
        assert_eq!(r[0], 1.0);
        assert_eq!(r[3], 0.0);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], TAU_S);
        assert_eq!(r[8], f64::from(kernel));
        assert_eq!(r[9], f64::from(force));
        assert_eq!(r[25], 0.0);
        for slot in 10..25 {
            assert_eq!(r[slot], 0.0);
        }
    }

    #[test]
    fn frame_roundtrips_the_measured_stride() {
        let records = vec![
            record([1.0, 2.0, 3.0], 0.0, 8.0e8, 0.0, 7.0),
            record([4.0, 5.0, 6.0], 12.5, 8.0e8 + 1.0, 0.0, 7.0),
        ];
        let bytes = write_bin(&records, 8.0e8, 0);
        assert!(bytes.starts_with(&[0xCF, 0x86, 0x09]));
        assert_eq!(bytes.len(), FRAME_HEADER + 2 * REC_BYTES);
        assert_eq!(verify_bin(&bytes), Some(2));
        assert!(verify_bin(b"X").is_none());
        assert!(verify_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn polar_stereographic_inverse_hits_the_standard_parallel() {
        let st = PolarStereo {
            a: 6_378_137.0,
            e: (2.0 / 298.257223563 - (1.0 / 298.257223563_f64).powi(2)).sqrt(),
            lon0_deg: -45.0,
            lat_ts_deg: 70.0,
            north: true,
        };
        let (lon, lat) = st.inverse(0.0, 0.0);
        assert_eq!(lon, -45.0);
        assert!((lat - 90.0).abs() < 1e-9, "pole lat {lat}");
        let t_c = t_of(70.0f64.to_radians(), st.e, true);
        let m_c = m_of(70.0f64.to_radians(), st.e);
        let rho = st.a * m_c * t_c / t_c;
        let (_, lat) = st.inverse(rho, 0.0);
        assert!((lat - 70.0).abs() < 1e-6, "standard parallel lat {lat}");
    }

    #[test]
    fn kernel_name_reads_the_registered_default() {
        assert_eq!(kernel_name_for("advective").unwrap(), "patch-levy");
        assert!(kernel_name_for("bogus").is_err());
    }
}
