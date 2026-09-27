use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{COMP_ESACCI_SST, GeoRec, MAGIC_ESACCI_SST, parse_bin, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5Attribute, Hdf5Datatype, Hdf5File, decode_f32, decode_f64};
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const NETLOC: &str = "data.ceda.ac.uk-eocis-sst";
const BASE: &str = "https://dap.ceda.ac.uk/neodc/eocis/data/global_and_regional/sea_surface_temperature/CDR_v3/Analysis/L4/v3.0.1";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("600")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn product_of(year: i64) -> &'static str {
    if year <= 2021 { "CDR3.0" } else { "ICDR3.0" }
}

fn day_url(year: i64, month: i64, day: i64) -> String {
    let product = product_of(year);
    format!(
        "{BASE}/{year:04}/{month:02}/{day:02}/{year:04}{month:02}{day:02}120000-ESACCI-L4_GHRSST-SSTdepth-OSTIA-GLOB_{product}-v02.0-fv01.0.nc"
    )
}

fn parse_date(date: &str) -> Option<(i64, i64, i64)> {
    let year: i64 = date.get(0..4)?.parse().ok()?;
    let month: i64 = date.get(5..7)?.parse().ok()?;
    let day: i64 = date.get(8..10)?.parse().ok()?;
    days_from_civil(year, month, day)?;
    Some((year, month, day))
}

fn attr_find<'a>(attrs: &'a [Hdf5Attribute], name: &str) -> Option<&'a Hdf5Attribute> {
    attrs.iter().find(|a| a.name == name)
}

fn decode_int_at(
    data: &[u8],
    off: usize,
    size: usize,
    endian: Endian,
    signed: bool,
) -> Option<i64> {
    let be = endian == Endian::Be;
    match size {
        1 => data
            .get(off)
            .map(|&b| if signed { b as i8 as i64 } else { b as i64 }),
        2 => {
            let b: [u8; 2] = data.get(off..off + 2)?.try_into().ok()?;
            let v = if be {
                u16::from_be_bytes(b)
            } else {
                u16::from_le_bytes(b)
            };
            Some(if signed { v as i16 as i64 } else { v as i64 })
        }
        4 => {
            let b: [u8; 4] = data.get(off..off + 4)?.try_into().ok()?;
            let v = if be {
                u32::from_be_bytes(b)
            } else {
                u32::from_le_bytes(b)
            };
            Some(if signed { v as i32 as i64 } else { v as i64 })
        }
        8 => {
            let b: [u8; 8] = data.get(off..off + 8)?.try_into().ok()?;
            let v = if be {
                u64::from_be_bytes(b)
            } else {
                u64::from_le_bytes(b)
            };
            Some(v as i64)
        }
        _ => None,
    }
}

fn attr_number(attrs: &[Hdf5Attribute], name: &str) -> Option<f64> {
    let a = attr_find(attrs, name)?;
    match a.datatype.class {
        0 => decode_int_at(
            &a.data,
            0,
            a.datatype.size,
            a.datatype.endian,
            a.datatype.signed,
        )
        .map(|v| v as f64),
        1 => match a.datatype.size {
            4 => decode_f32(&a.data, 0, a.datatype.endian).map(|v| v as f64),
            8 => decode_f64(&a.data, 0, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
}

struct Var {
    raw: Vec<u8>,
    dt: Hdf5Datatype,
    dims: Vec<u64>,
}

fn read_var(file: &Hdf5File, name: &str) -> Result<Var, String> {
    let (_, ds, dt) = file
        .dataset(name)
        .map_err(|e| format!("{name}: dataset reads void ({e:?})"))?;
    if dt.size == 0 {
        return Err(format!("{name}: datatype size reads void"));
    }
    let count = ds
        .dims
        .iter()
        .try_fold(1usize, |a, d| a.checked_mul(*d as usize))
        .ok_or_else(|| format!("{name}: dataspace overflows"))?;
    let raw = file
        .read_dataset(name)
        .map_err(|e| format!("{name}: read void ({e:?})"))?;
    if raw.len() != count * dt.size {
        return Err(format!(
            "{name}: {} B read against {count} cells x {} B",
            raw.len(),
            dt.size
        ));
    }
    Ok(Var {
        raw,
        dt: dt.clone(),
        dims: ds.dims.clone(),
    })
}

fn decode_num(raw: &[u8], off: usize, dt: &Hdf5Datatype) -> Option<f64> {
    match dt.class {
        1 => match dt.size {
            4 => decode_f32(raw, off, dt.endian).map(|v| v as f64),
            8 => decode_f64(raw, off, dt.endian),
            _ => None,
        },
        0 => decode_int_at(raw, off, dt.size, dt.endian, dt.signed).map(|v| v as f64),
        _ => None,
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let date =
        arg_value(args, "--date").ok_or_else(|| "--date (YYYY-MM-DD) required".to_string())?;
    let (year, month, day) = parse_date(&date)
        .ok_or_else(|| format!("--date {date} carries no civil day (YYYY-MM-DD)"))?;
    let stride = arg_value(args, "--stride")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(32);
    if stride == 0 {
        return Err("--stride carries no positive sampling step".to_string());
    }
    let out = arg_value(args, "--out").ok_or_else(|| "--out (path) required".to_string())?;
    let url = match arg_value(args, "--url") {
        Some(u) => u,
        None => day_url(year, month, day),
    };

    let bytes = fetch(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let file = Hdf5File::parse(&bytes).map_err(|e| format!("{url}: hdf5 parses void ({e:?})"))?;

    let lat = read_var(&file, "lat")?;
    let lon = read_var(&file, "lon")?;
    let sst = read_var(&file, "analysed_sst")?;
    let n_lat = lat.dims.iter().product::<u64>() as usize;
    let n_lon = lon.dims.iter().product::<u64>() as usize;
    let n_sst = sst.dims.iter().product::<u64>() as usize;
    let n_grid = n_lat
        .checked_mul(n_lon)
        .ok_or_else(|| "the grid shape overflows — the bin stays unwritten".to_string())?;
    if n_sst != n_grid {
        return Err(format!(
            "the grid shape reads void — {n_lat} lat, {n_lon} lon, {n_sst} analysed_sst cells; the bin stays unwritten"
        ));
    }

    let (sst_obj, _, _) = file
        .dataset("analysed_sst")
        .map_err(|e| format!("analysed_sst: dataset reads void ({e:?})"))?;
    let fill = attr_number(&sst_obj.attrs, "_FillValue");
    let scale =
        match attr_number(&sst_obj.attrs, "scale_factor") {
            Some(v) if v.is_finite() && v != 0.0 => v,
            _ => return Err(
                "analysed_sst carries no finite non-zero scale_factor — the values stay unmeasured"
                    .to_string(),
            ),
        };
    let offset = match attr_number(&sst_obj.attrs, "add_offset") {
        Some(v) if v.is_finite() => v,
        _ => {
            return Err(
                "analysed_sst carries no finite add_offset — the values stay unmeasured"
                    .to_string(),
            );
        }
    };
    eprintln!(
        "{url}: analysed_sst dims {:?} dt class {} size {} scale {scale} offset {offset} fill {fill:?}",
        sst.dims, sst.dt.class, sst.dt.size
    );

    let tdb = days_from_civil(year, month, day)
        .and_then(|d| lsk.unix_to_tdb(d as f64 * 86400.0 + 12.0 * 3600.0))
        .ok_or_else(|| "the daily anchor reads void".to_string())?;

    let mut records = Vec::new();
    let mut sampled = 0usize;
    let mut kept = 0usize;
    let mut i = 0usize;
    while i < n_lat {
        let Some(lat_v) = decode_num(&lat.raw, i * lat.dt.size, &lat.dt) else {
            break;
        };
        let mut j = 0usize;
        while j < n_lon {
            let Some(lon_v) = decode_num(&lon.raw, j * lon.dt.size, &lon.dt) else {
                break;
            };
            let Some(raw_v) = decode_num(&sst.raw, (i * n_lon + j) * sst.dt.size, &sst.dt) else {
                break;
            };
            sampled += 1;
            let filled = matches!(fill, Some(f) if raw_v == f);
            let val = raw_v * scale + offset;
            if !filled && val.is_finite() && val > 0.0 {
                records.push(GeoRec {
                    t: tdb,
                    lat: lat_v,
                    lon: lon_v,
                    alt: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    val,
                    comp: COMP_ESACCI_SST,
                    station: 0,
                });
                kept += 1;
            }
            j += stride;
        }
        i += stride;
    }

    if records.is_empty() {
        return Err(format!(
            "{url}: no measured cell left the harvest ({sampled} sampled, {kept} kept) — the bin stays unwritten (0 honored)"
        ));
    }
    let bytes = write_bin(MAGIC_ESACCI_SST, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_bin(MAGIC_ESACCI_SST, &bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} SST cells ({} sampled, {} kept, grid {n_lat}x{n_lon} stride {stride}), roundtrip parses",
                parsed.len(),
                sampled,
                kept
            );
            Ok(())
        }
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }?;
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("esacci_sst_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::hdf5::Hdf5Dataspace;

    #[test]
    fn product_follows_the_cdr_icdr_boundary() {
        assert_eq!(product_of(2000), "CDR3.0");
        assert_eq!(product_of(2021), "CDR3.0");
        assert_eq!(product_of(2022), "ICDR3.0");
        assert_eq!(product_of(2024), "ICDR3.0");
    }

    #[test]
    fn day_url_names_the_measured_daily_file() {
        assert_eq!(
            day_url(2000, 1, 1),
            "https://dap.ceda.ac.uk/neodc/eocis/data/global_and_regional/sea_surface_temperature/CDR_v3/Analysis/L4/v3.0.1/2000/01/01/20000101120000-ESACCI-L4_GHRSST-SSTdepth-OSTIA-GLOB_CDR3.0-v02.0-fv01.0.nc"
        );
        assert!(day_url(2024, 6, 22).ends_with(
            "2024/06/22/20240622120000-ESACCI-L4_GHRSST-SSTdepth-OSTIA-GLOB_ICDR3.0-v02.0-fv01.0.nc"
        ));
    }

    #[test]
    fn parse_date_rejects_a_non_civil_day() {
        assert_eq!(parse_date("2000-01-01"), Some((2000, 1, 1)));
        assert_eq!(parse_date("2000-13-01"), None);
        assert_eq!(parse_date("2000-01-00"), None);
    }

    #[test]
    fn attr_number_reads_float_and_int() {
        let f32_attr = Hdf5Attribute {
            name: "scale_factor".to_string(),
            datatype: Hdf5Datatype {
                class: 1,
                size: 4,
                endian: Endian::Le,
                signed: true,
                ..Hdf5Datatype::flat_f64()
            },
            dataspace: Hdf5Dataspace { dims: Vec::new() },
            data: 0.01f32.to_le_bytes().to_vec(),
        };
        let i16_attr = Hdf5Attribute {
            name: "_FillValue".to_string(),
            datatype: Hdf5Datatype {
                class: 0,
                size: 2,
                endian: Endian::Be,
                signed: true,
                ..Hdf5Datatype::flat_f64()
            },
            dataspace: Hdf5Dataspace { dims: Vec::new() },
            data: (-32768i16).to_be_bytes().to_vec(),
        };
        assert_eq!(attr_number(&[f32_attr], "scale_factor"), Some(0.01f64));
        assert_eq!(attr_number(&[i16_attr], "_FillValue"), Some(-32768.0));
    }

    #[test]
    fn bin_roundtrip_carries_the_sst_record() {
        let records = vec![
            GeoRec {
                t: 946_728_000.0,
                lat: -30.5,
                lon: 10.25,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: 291.4,
                comp: COMP_ESACCI_SST,
                station: 0,
            },
            GeoRec {
                t: 946_728_000.0,
                lat: 45.0,
                lon: -120.0,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: 283.1,
                comp: COMP_ESACCI_SST,
                station: 0,
            },
        ];
        let bytes = write_bin(MAGIC_ESACCI_SST, &records);
        let parsed = parse_bin(MAGIC_ESACCI_SST, &bytes).expect("the packed bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].val, 291.4);
        assert_eq!(parsed[1].lat, 45.0);
        assert_eq!(parsed[0].comp, COMP_ESACCI_SST);
    }
}
