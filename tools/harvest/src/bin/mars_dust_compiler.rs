use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::lsk::LeapSeconds;
use omegaflow::archivar::lsk::parse as parse_lsk;
use omegaflow::archivar::membrane::NAIF_LSK_EMBEDDED;
use omegaflow::archivar::motion::{
    BodyEphemeris, body_fixed_vector_to_icrs, parse_ephemeris_binary,
};
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage};
use omegaflow::json::{JsonVal, parse_json};
use std::collections::HashMap;
use std::process::Command;

const CDN_TAG: &str = "vo.lmd.jussieu.fr";
const TAP_DEFAULT: &str = "http://vo.lmd.jussieu.fr/tap/sync";
const EPH_DEFAULT: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_mars.bin";
const MAGIC: [u8; 4] = *b"MDST";
const REC_BYTES: usize = 26 * 8;
const FORCE_EM: f64 = 0.0;
const KERNEL_INVERSE_SQUARE: f64 = 0.0;
const TTL_S: f64 = 604800.0;
const TAU_S: f64 = 86400.0;
const JD_UNIX_EPOCH: f64 = 2440587.5;
const SECS_PER_DAY: f64 = 86400.0;
const MARS: &str = "mars";
const EPN_TABLE: &str = "mars_dust.epn_core";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn tap_query(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("300")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=json")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "mars_dust tap http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn as_arr(j: &JsonVal) -> Option<&Vec<JsonVal>> {
    match j {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn as_obj(j: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match j {
        JsonVal::Obj(o) => Some(o),
        _ => None,
    }
}

fn cell_str(c: &JsonVal) -> String {
    match c {
        JsonVal::Str(s) => s.clone(),
        JsonVal::Num(v) => format!("{}", v),
        _ => String::new(),
    }
}

fn tap_rows(root: &str, adql: &str) -> Option<(Option<Vec<String>>, Vec<Vec<String>>)> {
    let body = tap_query(root, adql)?;
    let parsed = parse_json(&body)?;
    let JsonVal::Obj(m) = &parsed else {
        eprintln!("mars_dust tap body is not an object: {} bytes", body.len());
        return None;
    };
    let fields: Option<Vec<String>> = m.get("metadata").and_then(as_arr).map(|meta| {
        meta.iter()
            .filter_map(|md| {
                as_obj(md).and_then(|o| match o.get("name") {
                    Some(JsonVal::Str(s)) => Some(s.clone()),
                    _ => None,
                })
            })
            .collect()
    });
    let data = m.get("data").and_then(as_arr)?;
    let mut rows = Vec::new();
    for row in data {
        let Some(r) = as_arr(row) else { continue };
        rows.push(r.iter().map(cell_str).collect());
    }
    Some((fields, rows))
}

fn col_pos(fields: &Option<Vec<String>>, name: &str, fallback: usize) -> Option<usize> {
    match fields {
        Some(fs) => fs.iter().position(|f| f.eq_ignore_ascii_case(name)),
        None => Some(fallback),
    }
}

fn cell_f64(cells: &[String], pos: usize) -> Option<f64> {
    cells
        .get(pos)?
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
}

struct CubeRow {
    url: String,
    t_min: f64,
    t_max: f64,
}

fn row_from_cells(fields: &Option<Vec<String>>, cells: &[String]) -> Option<CubeRow> {
    let u = col_pos(fields, "access_url", 0)?;
    let mn = col_pos(fields, "time_min", 1)?;
    let mx = col_pos(fields, "time_max", 2)?;
    let url = cells.get(u)?.clone();
    if url.is_empty() {
        return None;
    }
    let t_min = cell_f64(cells, mn)?;
    let t_max = cell_f64(cells, mx)?;
    if !(t_max > t_min) {
        return None;
    }
    Some(CubeRow { url, t_min, t_max })
}

fn adql_all() -> String {
    format!(
        "SELECT access_url,time_min,time_max FROM {} ORDER BY access_url",
        EPN_TABLE
    )
}

fn adql_url(url: &str) -> String {
    format!(
        "SELECT access_url,time_min,time_max FROM {} WHERE access_url = '{}'",
        EPN_TABLE, url
    )
}

fn adql_like(basename: &str) -> String {
    format!(
        "SELECT access_url,time_min,time_max FROM {} WHERE access_url LIKE '%{}%'",
        EPN_TABLE, basename
    )
}

fn list_cubes(root: &str) -> Result<(), String> {
    let (fields, rows) = tap_rows(root, &adql_all())
        .ok_or_else(|| "the epn_core inventory stays unread".to_string())?;
    let mut printed = 0usize;
    for cells in &rows {
        let Some(r) = row_from_cells(&fields, cells) else {
            continue;
        };
        println!("{} {} {}", r.url, r.t_min, r.t_max);
        printed += 1;
    }
    if printed == 0 {
        return Err("no epn_core row carries measured JD anchors".into());
    }
    eprintln!(
        "mars_dust tap: {} rows, {} with measured JD anchors",
        rows.len(),
        printed
    );
    Ok(())
}

struct SolAxis {
    crval3: f64,
    cdelt3: f64,
    crpix3: f64,
    jd_first: f64,
    jd_last: f64,
    n_slices: usize,
}

impl SolAxis {
    fn sol(&self, k: usize) -> f64 {
        self.crval3 + ((k + 1) as f64 - self.crpix3) * self.cdelt3
    }

    fn jd(&self, k: usize) -> Option<f64> {
        let s0 = self.sol(0);
        let s1 = self.sol(self.n_slices - 1);
        if s1 == s0 {
            return None;
        }
        let f = (self.sol(k) - s0) / (s1 - s0);
        let jd = self.jd_first + f * (self.jd_last - self.jd_first);
        if jd.is_finite() { Some(jd) } else { None }
    }
}

fn record(pos: [f64; 3], val: f64, epoch_jd: f64, extent: f64, presence: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = epoch_jd;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = extent;
    r[8] = KERNEL_INVERSE_SQUARE;
    r[9] = FORCE_EM;
    r[25] = presence;
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

fn read_bin(data: &[u8]) -> Option<(usize, usize)> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * REC_BYTES {
        return None;
    }
    let mut present = 0usize;
    for i in 0..count {
        let base = 8 + i * REC_BYTES + 25 * 8;
        let presence = f64::from_le_bytes(data[base..base + 8].try_into().ok()?);
        if presence == 1.0 {
            present += 1;
        }
    }
    Some((count, present))
}

fn compile_cube(
    fits: &[u8],
    row: &CubeRow,
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
) -> Result<Vec<[f64; 26]>, String> {
    let header = FitsHeader::parse(fits, 0)
        .ok_or_else(|| "the primary header stays unread".to_string())?
        .0;
    let image = FitsImage::parse(fits, 0)
        .ok_or_else(|| {
            "the primary cube stays unread (BITPIX/NAXIS outside the 3D image gate)".to_string()
        })?
        .0;
    let dims = image.dims;
    if dims[2] < 2 {
        return Err(format!(
            "the time axis carries {} slices — the sol→JD mapping needs two anchors",
            dims[2]
        ));
    }
    let a_radius = match header.f64("A_RADIUS") {
        Some(r) if r.is_finite() && r > 0.0 => r,
        _ => return Err("A_RADIUS absent — the grid sphere stays unmeasured".into()),
    };
    let pixel_deg = match (header.f64("CD1_1"), header.f64("CD2_2")) {
        (Some(a), Some(b)) => a.abs().max(b.abs()),
        _ => match (header.f64("CDELT1"), header.f64("CDELT2")) {
            (Some(a), Some(b)) => a.abs().max(b.abs()),
            _ => return Err("no CD/CDELT pixel scale — the pixel extent stays unmeasured".into()),
        },
    };
    if !(pixel_deg.is_finite() && pixel_deg > 0.0) {
        return Err(format!(
            "pixel scale {pixel_deg} deg is no positive measurement"
        ));
    }
    let extent = a_radius * pixel_deg.to_radians();
    let crval3 = match header.f64("CRVAL3") {
        Some(v) if v.is_finite() => v,
        _ => return Err("CRVAL3 absent — the sol axis stays unread".into()),
    };
    let cdelt3 = match header.f64("CDELT3").or(header.f64("CD3_3")) {
        Some(v) if v.is_finite() && v != 0.0 => v,
        _ => return Err("CDELT3/CD3_3 absent — the sol step stays unread".into()),
    };
    let crpix3 = match header.f64("CRPIX3") {
        Some(v) if v.is_finite() => v,
        _ => 1.0,
    };
    let axis = SolAxis {
        crval3,
        cdelt3,
        crpix3,
        jd_first: row.t_min,
        jd_last: row.t_max,
        n_slices: dims[2],
    };
    let (jd0, jd1) = match (axis.jd(0), axis.jd(dims[2] - 1)) {
        (Some(a), Some(b)) => (a, b),
        _ => return Err("the sol span carries no JD anchors".into()),
    };
    let mut bf_grid: Vec<Option<[f64; 3]>> = Vec::with_capacity(dims[0] * dims[1]);
    let mut bf_placed = 0usize;
    for j in 0..dims[1] {
        for i in 0..dims[0] {
            let Some((lon_deg, lat_deg)) = image.world((i + 1) as f64, (j + 1) as f64) else {
                bf_grid.push(None);
                continue;
            };
            if !(lat_deg.is_finite() && (-90.0..=90.0).contains(&lat_deg) && lon_deg.is_finite()) {
                bf_grid.push(None);
                continue;
            }
            let lat = lat_deg.to_radians();
            let lon = lon_deg.to_radians();
            let (sl, cl) = lat.sin_cos();
            let (so, co) = lon.sin_cos();
            bf_grid.push(Some([
                a_radius * cl * co,
                a_radius * cl * so,
                a_radius * sl,
            ]));
            bf_placed += 1;
        }
    }
    if bf_placed == 0 {
        return Err("no pixel lands in the ±90° lat grid — the cube stays unplaced".into());
    }
    let mut records: Vec<[f64; 26]> = Vec::new();
    let mut absent = 0usize;
    let mut skipped_slices = 0usize;
    let mut skipped_pos = 0usize;
    for k in 0..dims[2] {
        let Some(jd) = axis.jd(k) else {
            continue;
        };
        let unix = (jd - JD_UNIX_EPOCH) * SECS_PER_DAY;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            skipped_slices += 1;
            continue;
        };
        for j in 0..dims[1] {
            for i in 0..dims[0] {
                let Some(bf) = bf_grid[j * dims[0] + i] else {
                    continue;
                };
                let Some(pos) = body_fixed_vector_to_icrs(MARS, bf, tdb, eph) else {
                    skipped_pos += 1;
                    continue;
                };
                match image.value_f64(fits, [i, j, k]) {
                    Some(v) if v.is_finite() => {
                        records.push(record(pos, v, jd, extent, 1.0));
                    }
                    _ => {
                        records.push(record(pos, 0.0, jd, extent, 0.0));
                        absent += 1;
                    }
                }
            }
        }
    }
    if records.is_empty() {
        return Err(format!(
            "no record left the cube — {absent} absent, {skipped_pos} unplaced, {skipped_slices} slices outside the leap table"
        ));
    }
    eprintln!(
        "mars_dust cube {} ({}x{}x{}): {} records, {} absent CDOD, {} unplaced, {} slices outside the leap table; extent {:.0} m; sol step {:.6} sol; JD span {:.6}..{:.6}",
        row.url,
        dims[0],
        dims[1],
        dims[2],
        records.len(),
        absent,
        skipped_pos,
        skipped_slices,
        extent,
        axis.sol(1) - axis.sol(0),
        jd0,
        jd1,
    );
    Ok(records)
}

fn run(args: &[String]) -> Result<(), String> {
    let usage = "usage: mars_dust_compiler --out <bin> [--row <k> | --url <fits-url> | --input <fits-file>] [--root <tap>] [--ephemeris <path|url>] [--list] [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let root = match arg_value(args, "--root") {
        Some(v) => v,
        None => TAP_DEFAULT.to_string(),
    };
    if args.iter().any(|a| a == "--list") {
        return list_cubes(&root);
    }
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => return Err(usage.to_string()),
    };
    let lsk = parse_lsk(NAIF_LSK_EMBEDDED).ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the JD→TDB step is unavailable".to_string()
    })?;
    let eph_bytes = match arg_value(args, "--ephemeris") {
        Some(src) if src.starts_with("http") => {
            fetch_raw_bytes(&src).ok_or_else(|| format!("ephemeris fetch void ({src})"))?
        }
        Some(path) => std::fs::read(&path).map_err(|e| format!("ephemeris read {path}: {e}"))?,
        None => fetch_raw_bytes(EPH_DEFAULT)
            .ok_or_else(|| format!("ephemeris fetch void ({EPH_DEFAULT})"))?,
    };
    let eph_body = parse_ephemeris_binary(&eph_bytes)
        .ok_or_else(|| "the Mars ephemeris binary stays unread — no ICRS frame".to_string())?;
    let mut eph = HashMap::new();
    eph.insert(MARS.to_string(), eph_body);
    let (fits, row) = match (
        arg_value(args, "--row"),
        arg_value(args, "--url"),
        arg_value(args, "--input"),
    ) {
        (Some(k_str), _, _) => {
            let k: usize = k_str
                .parse()
                .map_err(|_| format!("--row {k_str} is not a row index"))?;
            let (fields, rows) = tap_rows(&root, &adql_all())
                .ok_or_else(|| "the epn_core inventory stays unread".to_string())?;
            let cells = rows
                .get(k)
                .ok_or_else(|| format!("--row {k} is beyond {} rows", rows.len()))?;
            let row = row_from_cells(&fields, cells)
                .ok_or_else(|| format!("row {k} carries no measured JD anchors"))?;
            let fits = fetch_raw_bytes(&row.url)
                .ok_or_else(|| format!("cube fetch void ({})", row.url))?;
            (fits, row)
        }
        (None, Some(url), _) => {
            let (fields, rows) = tap_rows(&root, &adql_url(&url))
                .ok_or_else(|| "the epn_core row for the cube URL stays unread".to_string())?;
            let cells = rows
                .first()
                .ok_or_else(|| format!("epn_core carries no row for {url}"))?;
            let row = row_from_cells(&fields, cells)
                .ok_or_else(|| format!("the row for {url} carries no measured JD anchors"))?;
            let fits = fetch_raw_bytes(&url).ok_or_else(|| format!("cube fetch void ({url})"))?;
            (fits, row)
        }
        (None, None, Some(path)) => {
            let name = match std::path::Path::new(&path).file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => return Err(format!("--input {path} carries no filename")),
            };
            let (fields, rows) = tap_rows(&root, &adql_like(&name))
                .ok_or_else(|| "the epn_core row for the cube file stays unread".to_string())?;
            let cells = rows
                .first()
                .ok_or_else(|| format!("epn_core carries no row like {name}"))?;
            let row = row_from_cells(&fields, cells)
                .ok_or_else(|| format!("the row for {name} carries no measured JD anchors"))?;
            let fits = std::fs::read(&path).map_err(|e| format!("cube read {path}: {e}"))?;
            (fits, row)
        }
        (None, None, None) => {
            return Err(
                "--row <k> | --url <fits-url> | --input <fits-file> absent — refused".into(),
            );
        }
    };
    let records = compile_cube(&fits, &row, &lsk, &eph)?;
    let bin = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out}: {e}"))?;
    let (count, present) = read_bin(&bin)
        .ok_or_else(|| format!("{out}: roundtrip parse void — the asset stays unverified"))?;
    eprintln!(
        "mars_dust: {} records, {} finite CDOD, {} B -> {out} (roundtrip: {count} records, {present} present)",
        records.len(),
        present,
        bin.len(),
    );
    if ci_mode && !upload_release(CDN_TAG, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("mars_dust_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_carries_the_wire_slots() {
        let r = record([1.0, 2.0, 3.0], 0.5, 2451545.0, 354952.0, 1.0);
        assert_eq!(r[0], 1.0);
        assert_eq!(r[1], 2.0);
        assert_eq!(r[2], 3.0);
        assert_eq!(r[3], 0.5);
        assert_eq!(r[4], 2451545.0);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], TAU_S);
        assert_eq!(r[7], 354952.0);
        assert_eq!(r[8], 0.0);
        assert_eq!(r[9], 0.0);
        assert_eq!(r[10], 0.0);
        assert_eq!(r[25], 1.0);
    }

    #[test]
    fn absent_record_pads_val_and_drops_presence() {
        let r = record([1.0, 2.0, 3.0], 0.0, 2451545.0, 100.0, 0.0);
        assert_eq!(r[3], 0.0);
        assert_eq!(r[25], 0.0);
    }

    #[test]
    fn bin_roundtrip_counts_presence() {
        let records = vec![
            record([1.0, 2.0, 3.0], 0.4, 2451545.0, 100.0, 1.0),
            record([4.0, 5.0, 6.0], 0.0, 2451545.1, 100.0, 0.0),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        let (count, present) = read_bin(&bytes).expect("parse");
        assert_eq!(count, 2);
        assert_eq!(present, 1);
    }

    #[test]
    fn read_bin_rejects_bad_magic_and_truncation() {
        assert!(read_bin(b"X").is_none());
        let bytes = write_bin(&[record([1.0, 2.0, 3.0], 0.4, 2451545.0, 100.0, 1.0)]);
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert!(read_bin(&bad).is_none());
    }

    #[test]
    fn sol_axis_maps_linear_jd_anchors() {
        let axis = SolAxis {
            crval3: 334.0,
            cdelt3: 0.5,
            crpix3: 1.0,
            jd_first: 2452500.0,
            jd_last: 2452501.0,
            n_slices: 5,
        };
        assert_eq!(axis.sol(0), 334.0);
        assert_eq!(axis.sol(4), 336.0);
        assert_eq!(axis.jd(0), Some(2452500.0));
        assert_eq!(axis.jd(4), Some(2452501.0));
        assert_eq!(axis.jd(2), Some(2452500.5));
    }

    #[test]
    fn sol_axis_refuses_a_degenerate_span() {
        let axis = SolAxis {
            crval3: 334.0,
            cdelt3: 0.0,
            crpix3: 1.0,
            jd_first: 2452500.0,
            jd_last: 2452501.0,
            n_slices: 5,
        };
        assert_eq!(axis.jd(0), None);
    }

    #[test]
    fn col_pos_falls_back_to_query_order() {
        assert_eq!(col_pos(&None, "access_url", 0), Some(0));
        let fields = Some(vec![
            "access_url".into(),
            "time_min".into(),
            "time_max".into(),
        ]);
        assert_eq!(col_pos(&fields, "TIME_MIN", 0), Some(1));
    }
}
