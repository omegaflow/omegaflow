use omegaflow::archivar::geo::{
    parse_bin, write_bin, GeoRec, COMP_BGR_AZIM, COMP_BGR_FREQ, COMP_BGR_RMS, COMP_BGR_VAPP,
    MAGIC_BGR,
};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File, Hdf5Layout};
use omegaflow::inflate::inflate;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

const NETLOC: &str = "download.bgr.de";

const DEFAULT_ZIP: &str = "https://download.bgr.de/bgr/geophysik/Infrasound_hf_product/netCDF/BGR_infrasound_hf_product.zip";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    arg_value(args, name).and_then(|v| v.parse::<usize>().ok())
}

fn curl_range(url: &str, from: u64, to: u64) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("240")
        .arg("-r")
        .arg(format!("{}-{}", from, to))
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn http_size(url: &str) -> Option<u64> {
    let out = Command::new("curl")
        .arg("-sSI")
        .arg("-L")
        .arg("-m")
        .arg("60")
        .arg(url)
        .output()
        .ok()?;
    let head = String::from_utf8_lossy(&out.stdout);
    for l in head.lines() {
        let l = l.to_ascii_lowercase();
        if l.starts_with("content-length:") {
            if let Some(v) = l
                .split(':')
                .nth(1)
                .and_then(|s| s.trim().parse::<u64>().ok())
            {
                return Some(v);
            }
        }
    }
    None
}

fn le16(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}
fn le32(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

struct ZipEntry {
    name: String,
    comp: u64,
    method: u16,
    lho: u64,
}

fn zip_entries(url: &str) -> Option<Vec<ZipEntry>> {
    let size = http_size(url)?;
    let tail_n = 200000u64;
    let tail = curl_range(url, size - tail_n, size - 1)?;
    let mut eocd: Option<u64> = None;
    for i in (0..tail.len() - 4).rev() {
        if tail[i..i + 4] == [0x50, 0x4b, 0x05, 0x06] {
            eocd = Some((size - tail_n) + i as u64);
            break;
        }
    }
    let eo = (eocd? - (size - tail_n)) as usize;
    let cd_size = le32(&tail[eo + 12..]) as u64;
    let cd_off = le32(&tail[eo + 16..]) as u64;
    let cd = curl_range(url, cd_off, cd_off + cd_size.saturating_sub(1))?;
    let mut entries = Vec::new();
    let mut p = 0usize;
    while p + 46 <= cd.len() {
        if cd[p..p + 4] != [0x50, 0x4b, 0x01, 0x02] {
            p += 1;
            continue;
        }
        let method = le16(&cd[p + 10..]);
        let comp = le32(&cd[p + 20..]) as u64;
        let nlen = le16(&cd[p + 28..]) as usize;
        let xlen = le16(&cd[p + 30..]) as usize;
        let clen = le16(&cd[p + 32..]) as usize;
        let lho = le32(&cd[p + 42..]) as u64;
        let name = String::from_utf8_lossy(&cd[p + 46..p + 46 + nlen]).into_owned();
        entries.push(ZipEntry {
            name,
            comp,
            method,
            lho,
        });
        p += 46 + nlen + xlen + clen;
    }
    Some(entries)
}

fn fetch_entry_bytes(url: &str, e: &ZipEntry) -> Option<Vec<u8>> {
    let lh = curl_range(url, e.lho, e.lho + 30)?;
    let nlen = le16(&lh[26..]) as u64;
    let xlen = le16(&lh[28..]) as u64;
    let start = e.lho + 30 + nlen + xlen;
    let raw = curl_range(url, start, start + e.comp.saturating_sub(1))?;
    if e.method == 0 {
        Some(raw)
    } else {
        inflate(&raw)
    }
}

fn elem_f64(raw: &[u8], idx: usize, class: u8, size: usize, endian: Endian) -> Option<f64> {
    let off = idx.checked_mul(size)?;
    let b = raw.get(off..off + size)?;
    match (class, size) {
        (0, 1) => Some(b[0] as i8 as f64),
        (0, 2) => {
            let v = if endian == Endian::Le {
                i16::from_le_bytes([b[0], b[1]])
            } else {
                i16::from_be_bytes([b[0], b[1]])
            };
            Some(v as f64)
        }
        (0, 4) => {
            let arr: [u8; 4] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                i32::from_le_bytes(arr)
            } else {
                i32::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (0, 8) => {
            let arr: [u8; 8] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                i64::from_le_bytes(arr)
            } else {
                i64::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (1, 4) => {
            let arr: [u8; 4] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                f32::from_le_bytes(arr)
            } else {
                f32::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (1, 8) => {
            let arr: [u8; 8] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                f64::from_le_bytes(arr)
            } else {
                f64::from_be_bytes(arr)
            };
            Some(v)
        }
        _ => None,
    }
}

fn cell(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => {
            if x.fract() == 0.0 && x.abs() < 1.0e15 {
                format!("{}", x as i64)
            } else {
                format!("{}", x)
            }
        }
        _ => String::new(),
    }
}

struct VarLoad {
    raw: Vec<u8>,
    class: u8,
    size: usize,
    endian: Endian,
    n: usize,
}

fn load_detection_var(file: &Hdf5File, name: &str, n_det: usize) -> Option<VarLoad> {
    let (obj, ds, dt) = file.dataset(name).ok()?;
    if obj.is_group {
        return None;
    }
    let n: usize = ds
        .dims
        .iter()
        .fold(1usize, |a, d| a.saturating_mul(*d as usize));
    if n == 0 || n % n_det != 0 {
        return None;
    }
    let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_dataset(name)))
        .ok()?
        .ok()?;
    Some(VarLoad {
        raw,
        class: dt.class,
        size: dt.size,
        endian: dt.endian,
        n,
    })
}

fn string_cell(raw: &[u8], idx: usize, n_det: usize, lead: usize) -> String {
    let mut out = Vec::new();
    for r in 0..lead {
        if let Some(b) = raw.get(r * n_det + idx) {
            out.push(*b);
        }
    }
    let s = String::from_utf8_lossy(&out).into_owned();
    s.trim_end_matches(['\0', ' ']).to_string()
}

fn dt_to_unix(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 15 {
        return None;
    }
    let n = |i: usize, j: usize| s.get(i..j)?.parse::<i64>().ok();
    let (y, mo, d) = (n(0, 4)?, n(4, 6)?, n(6, 8)?);
    let (h, mi, se) = (n(9, 11)?, n(11, 13)?, n(13, 15)?);
    let days = days_from_civil(y, mo, d)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + se as f64)
}

struct TimeAxis {
    raw: Vec<u8>,
    n: usize,
    lead: usize,
    tp_size: usize,
}

fn time_axis(file: &Hdf5File) -> Option<TimeAxis> {
    let (obj, ds, dt) = file.dataset("time_p").ok()?;
    if obj.is_group {
        return None;
    }
    let n = *ds.dims.last().unwrap_or(&0) as usize;
    if n == 0 {
        return None;
    }
    let lead = if ds.dims.len() <= 1 {
        1
    } else {
        ds.dims[..ds.dims.len() - 1]
            .iter()
            .map(|d| *d as usize)
            .product::<usize>()
    };
    if lead == 0 {
        return None;
    }
    let raw =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_dataset("time_p")))
            .ok()?
            .ok()?;
    Some(TimeAxis {
        raw,
        n,
        lead,
        tp_size: dt.size,
    })
}

fn detection_time(ta: &TimeAxis, j: usize) -> Option<f64> {
    let s = if ta.tp_size > 1 {
        let off = (j * ta.tp_size).min(ta.raw.len().saturating_sub(ta.tp_size));
        let raw = ta.raw.get(off..off + ta.tp_size)?;
        String::from_utf8_lossy(raw).into_owned()
    } else {
        string_cell(&ta.raw, j, ta.n, ta.lead)
    };
    dt_to_unix(s.trim_end_matches(['\0', ' ', 'T']))
}

fn station_scalar(file: &Hdf5File, name: &str) -> Option<f64> {
    let (obj, _, dt) = file.dataset(name).ok()?;
    if obj.is_group {
        return None;
    }
    let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_dataset(name)))
        .ok()?
        .ok()?;
    elem_f64(&raw, 0, dt.class, dt.size, dt.endian)
}

fn detection_axis(file: &Hdf5File, name: &str) -> Option<VarLoad> {
    let (obj, ds, dt) = file.dataset(name).ok()?;
    if obj.is_group {
        return None;
    }
    let n: usize = ds
        .dims
        .iter()
        .fold(1usize, |a, d| a.saturating_mul(*d as usize));
    if n == 0 {
        return None;
    }
    let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_dataset(name)))
        .ok()?
        .ok()?;
    Some(VarLoad {
        raw,
        class: dt.class,
        size: dt.size,
        endian: dt.endian,
        n,
    })
}

fn var_mean(v: &VarLoad, j: usize) -> Option<f64> {
    if j >= v.n {
        return None;
    }
    elem_f64(&v.raw, j, v.class, v.size, v.endian)
}

fn flag_by_step(file: &Hdf5File) -> Option<HashMap<i64, f64>> {
    let (obj_t, ds_t, dt_t) = file.dataset("time").ok()?;
    if obj_t.is_group {
        return None;
    }
    let n: usize = *ds_t.dims.last().unwrap_or(&0) as usize;
    if n == 0 {
        return None;
    }
    let lead: usize = if ds_t.dims.len() <= 1 {
        1
    } else {
        ds_t.dims[..ds_t.dims.len() - 1]
            .iter()
            .map(|d| *d as usize)
            .product::<usize>()
    };
    if lead == 0 {
        return None;
    }
    let t_raw = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        file.read_dataset("time")
    })) {
        Ok(Ok(b)) => b,
        _ => return None,
    };
    let (obj_f, ds_f, dt_f) = file.dataset("flag").ok()?;
    if obj_f.is_group {
        return None;
    }
    let f_count: usize = ds_f
        .dims
        .iter()
        .fold(1usize, |a, d| a.saturating_mul(*d as usize));
    let f_raw = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        file.read_dataset("flag")
    })) {
        Ok(Ok(b)) => b,
        _ => return None,
    };
    if f_count < n {
        return None;
    }
    let mut map = HashMap::new();
    for j in 0..n {
        let s = if dt_t.size > 1 {
            let off = (j * dt_t.size).min(t_raw.len().saturating_sub(dt_t.size));
            let raw = t_raw.get(off..off + dt_t.size)?;
            String::from_utf8_lossy(raw).into_owned()
        } else {
            string_cell(&t_raw, j, n, lead)
        };
        let Some(epoch) = dt_to_unix(s.trim_end_matches(['\0', ' ', 'T'])) else {
            continue;
        };
        let Some(fl) = elem_f64(&f_raw, j, dt_f.class, dt_f.size, dt_f.endian) else {
            continue;
        };
        map.insert(epoch as i64, fl);
    }
    Some(map)
}

fn detection_records(file: &Hdf5File, lsk: &omegaflow::lsk::LeapSeconds) -> Vec<GeoRec> {
    let Some(ta) = time_axis(file) else {
        return Vec::new();
    };
    let Some(lat) = station_scalar(file, "lat") else {
        return Vec::new();
    };
    let Some(lon) = station_scalar(file, "lon") else {
        return Vec::new();
    };
    let Some(elev) = station_scalar(file, "elev") else {
        eprintln!(
            "bgr infrasound: station lat {lat} lon {lon} carries no elevation — held pending (geodata lat/lon-to-elevation lookup), no fabricated 0.0"
        );
        return Vec::new();
    };
    let azim = detection_axis(file, "azim");
    let vapp = detection_axis(file, "vapp");
    let a_rms = detection_axis(file, "a_rms");
    let freq = detection_axis(file, "freq");
    if azim.is_none() && vapp.is_none() && a_rms.is_none() && freq.is_none() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for j in 0..ta.n {
        let Some(t) = detection_time(&ta, j) else {
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(t) else {
            continue;
        };
        let push = |out: &mut Vec<GeoRec>, comp: u32, val: f64| {
            if val.is_finite() {
                out.push(GeoRec {
                    t: tdb,
                    lat,
                    lon,
                    alt: elev,
                    freq: 0.0,
                    bin_width: 0.0,
                    val,
                    comp,
                });
            }
        };
        if let Some(v) = azim.as_ref().and_then(|v| var_mean(v, j)) {
            push(&mut out, COMP_BGR_AZIM, v);
        }
        if let Some(v) = vapp.as_ref().and_then(|v| var_mean(v, j)) {
            if v > 0.0 {
                push(&mut out, COMP_BGR_VAPP, v);
            }
        }
        if let Some(v) = a_rms.as_ref().and_then(|v| var_mean(v, j)) {
            if v > 0.0 {
                push(&mut out, COMP_BGR_RMS, v);
            }
        }
        if let Some(v) = freq.as_ref().and_then(|v| var_mean(v, j)) {
            if v > 0.0 {
                push(&mut out, COMP_BGR_FREQ, v);
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let zip_url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => DEFAULT_ZIP.to_string(),
    };
    let station = match arg_value(&args, "--station") {
        Some(v) => v,
        None => "IS52".to_string(),
    };
    let year = match arg_value(&args, "--year") {
        Some(v) => v,
        None => "2024".to_string(),
    };
    let out = arg_value(&args, "--out");
    let out_bin = arg_value(&args, "--out-bin");
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let lsk_text = arg_value(&args, "--lsk").and_then(|p| fs::read_to_string(p).ok());
    let limit = arg_usize(&args, "--limit").unwrap_or(10);

    let entries = match zip_entries(&zip_url) {
        Some(e) => e,
        None => {
            eprintln!("bgr: {} stayed unreadable — pending", zip_url);
            std::process::exit(1);
        }
    };
    let n_nc = entries.iter().filter(|e| e.name.ends_with(".nc")).count();
    let mut cand: Vec<&ZipEntry> = entries
        .iter()
        .filter(|e| {
            e.name.ends_with(".nc")
                && !e.name.contains("archived")
                && e.name.contains(&station)
                && e.name.contains(&year)
        })
        .collect();
    cand.sort_by_key(|e| e.comp);
    eprintln!(
        "bgr: {} netCDF entries in the archive, {} match {}_{}",
        n_nc,
        cand.len(),
        station,
        year
    );
    if cand.is_empty() {
        eprintln!(
            "bgr: {}_{} matched no entry in {} — pending",
            station, year, zip_url
        );
        std::process::exit(1);
    }
    if let Some(bin_path) = out_bin {
        let Some(lsk) = lsk_text.as_deref().and_then(parse_lsk) else {
            eprintln!("bgr: --out-bin needs --lsk <naif0012.tls> — the TDB clock stays unread");
            std::process::exit(1);
        };
        let mut records: Vec<GeoRec> = Vec::new();
        let cap = arg_value(&args, "--limit").and_then(|v| v.parse::<usize>().ok());
        for entry in &cand {
            if cap.is_some_and(|c| records.len() >= c) {
                break;
            }
            let bytes = match fetch_entry_bytes(&zip_url, entry) {
                Some(b) => b,
                None => {
                    eprintln!("bgr: entry {} stayed unreadable — pending", entry.name);
                    continue;
                }
            };
            let Ok(file) = Hdf5File::parse(&bytes) else {
                eprintln!("bgr: {} parses void", entry.name);
                continue;
            };
            let recs = detection_records(&file, &lsk);
            eprintln!(
                "bgr: {} → {} detection parameter rows",
                entry.name,
                recs.len()
            );
            records.extend(recs);
        }
        if records.is_empty() {
            eprintln!(
                "bgr: no detection rows from {}_{} in {} — the bin stays unwritten (0 honored)",
                station, year, zip_url
            );
            std::process::exit(1);
        }
        records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
        let bytes = write_bin(MAGIC_BGR, &records);
        if fs::write(&bin_path, &bytes).is_err() {
            eprintln!("write {} returned void", bin_path);
            std::process::exit(1);
        }
        match parse_bin(MAGIC_BGR, &bytes) {
            Some(parsed) => eprintln!(
                "{}: {} geo records, {} B, roundtrip parses",
                bin_path,
                parsed.len(),
                bytes.len()
            ),
            None => {
                eprintln!("{}: roundtrip parse void", bin_path);
                std::process::exit(1);
            }
        }
        if ci_mode && !upload_release(NETLOC, &bin_path) {
            std::process::exit(1);
        }
        return;
    }

    let chosen = match cand.first() {
        Some(c) => *c,
        None => {
            eprintln!(
                "bgr: {}_{} matched no entry in {} — pending",
                station, year, zip_url
            );
            std::process::exit(1);
        }
    };
    eprintln!("bgr: entry {} ({} B compressed)", chosen.name, chosen.comp);
    let bytes = match fetch_entry_bytes(&zip_url, chosen) {
        Some(b) => b,
        None => {
            eprintln!("bgr: entry {} stayed unreadable — pending", chosen.name);
            std::process::exit(1);
        }
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("bgr: {} parses void ({:?})", chosen.name, note);
            std::process::exit(1);
        }
    };

    let (obj_tp, ds_tp, dt_tp) = match file.dataset("time_p") {
        Ok(d) => d,
        Err(_) => {
            eprintln!("bgr: {} carries no time_p detection axis", chosen.name);
            std::process::exit(1);
        }
    };
    if matches!(obj_tp.layout, Some(Hdf5Layout::Contiguous { .. })) {
        eprintln!(
            "bgr: {} time_p is a contiguous-layout dataset (address gap) — pending",
            chosen.name
        );
        std::process::exit(1);
    }
    let tpl = ds_tp.dims.clone();
    let n_det = *tpl.last().unwrap_or(&0) as usize;
    if n_det == 0 {
        eprintln!("bgr: {} time_p axis length is zero — pending", chosen.name);
        std::process::exit(1);
    }
    let lead_tp = if tpl.len() <= 1 {
        1
    } else {
        tpl[..tpl.len() - 1]
            .iter()
            .map(|d| *d as usize)
            .product::<usize>()
    };
    if lead_tp == 0 {
        eprintln!(
            "bgr: {} time_p carries a zero-width string axis — pending",
            chosen.name
        );
        std::process::exit(1);
    }
    let tp_size = dt_tp.size;
    let time_raw = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        file.read_dataset("time_p")
    })) {
        Ok(Ok(b)) => b,
        _ => {
            eprintln!("bgr: {} time_p stays unreadable — pending", chosen.name);
            std::process::exit(1);
        }
    };

    let mut names: Vec<String> = file
        .links_of("/")
        .iter()
        .filter(|l| l.name != "time_p")
        .map(|l| l.name.clone())
        .collect();
    names.sort();

    let mut loads: Vec<(String, VarLoad, usize)> = Vec::new();
    for name in &names {
        if name == "N_avail" {
            continue;
        }
        if let Some(l) = load_detection_var(&file, name, n_det) {
            let dims = match file.dataset(name) {
                Ok((_, ds, _)) => ds.dims.clone(),
                Err(_) => Vec::new(),
            };
            let lead = if dims.len() <= 1 {
                1
            } else {
                dims[..dims.len() - 1]
                    .iter()
                    .map(|d| *d as usize)
                    .product::<usize>()
            };
            loads.push((name.clone(), l, lead));
        }
    }
    if loads.is_empty() {
        eprintln!(
            "bgr: {} carried no readable detection datasets — pending",
            chosen.name
        );
        std::process::exit(1);
    }
    let flag_by_step = flag_by_step(&file);

    let mut header = String::from("#idx|time_utc");
    if flag_by_step.is_some() {
        header.push_str("|flag");
    }
    for (name, _, lead) in &loads {
        if *lead <= 1 {
            header.push_str(&format!("|{}", name));
        } else {
            for r in 0..*lead {
                header.push_str(&format!("|{}.{}", name, r));
            }
        }
    }
    let mut buf = String::new();
    if flag_by_step.is_some() {
        buf.push_str("#flag=1 all sensors|2 fewer but at least three|3 less than three, no PMCC\n");
    }
    buf.push_str(&header);
    buf.push('\n');
    for j in 0..n_det {
        let mut row = vec![j.to_string()];
        let time_s = if tp_size > 1 {
            let off = (j * tp_size).min(time_raw.len().saturating_sub(tp_size));
            let s = String::from_utf8_lossy(&time_raw[off..off + tp_size]).into_owned();
            s.trim_end_matches(['\0', ' ']).to_string()
        } else {
            string_cell(&time_raw, j, n_det, lead_tp)
        };
        row.push(time_s.clone());
        if let Some(map) = &flag_by_step {
            let step = dt_to_unix(time_s.trim_end_matches(['\0', ' ', 'T']))
                .map(|t| t as i64)
                .and_then(|t| map.get(&t).copied());
            row.push(cell(step));
        }
        for (_, l, lead) in &loads {
            for r in 0..*lead {
                let idx = r * n_det + j;
                if idx < l.n {
                    row.push(cell(elem_f64(&l.raw, idx, l.class, l.size, l.endian)));
                } else {
                    row.push(String::new());
                }
            }
        }
        buf.push_str(&row.join("|"));
        buf.push('\n');
    }

    if let Some(p) = out {
        if fs::write(&p, &buf).is_err() {
            eprintln!("write {} returned void", p);
            std::process::exit(1);
        }
    }
    eprintln!(
        "bgr: {} detection rows read from {} ({} detection variables)",
        n_det,
        chosen.name,
        loads.len()
    );
    println!("{}", header);
    for line in buf.lines().skip(1).take(limit) {
        println!("{}", line);
    }
}
