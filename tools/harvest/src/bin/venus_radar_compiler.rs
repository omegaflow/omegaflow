use omegaflow::archivar::pds4::{XElem, parse_xml};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{LeapSeconds, embedded_lsk, parse_iso_tdb, pds4_img, vnrd};
use omegaflow::cdn::{
    CDN_PART_MAX_BYTES, GITHUB_RELEASE_ASSET_MAX_BYTES, format_parts_manifest, part_name,
    parts_manifest_name, upload_release,
};
use std::path::{Path, PathBuf};

const IMG_BYTES: usize = pds4_img::LINE_COUNT * pds4_img::SAMPLE_COUNT * pds4_img::ELEMENT_BYTES;

const NETLOC: &str = "pds-geosciences.wustl.edu";

struct Product {
    img: PathBuf,
    label: PathBuf,
    comp: u32,
}

fn cell_value(cell: &[u8]) -> Option<f64> {
    let v = f32::from_le_bytes([cell[0], cell[1], cell[2], cell[3]]);
    v.is_finite().then_some(v as f64)
}

fn map_mean(bytes: &[u8], lines: usize, samples: usize) -> Option<f64> {
    let count = lines.checked_mul(samples)?;
    let expected = count.checked_mul(pds4_img::ELEMENT_BYTES)?;
    if lines == 0 || samples == 0 || bytes.len() != expected {
        return None;
    }
    let mut sum = 0.0f64;
    let mut present = 0usize;
    for cell in bytes.chunks_exact(pds4_img::ELEMENT_BYTES) {
        if let Some(v) = cell_value(cell) {
            sum += v;
            present += 1;
        }
    }
    (present > 0).then(|| sum / present as f64)
}

fn local_name(name: &str) -> &str {
    name.rsplit_once(":").map_or(name, |(_, local)| local)
}

fn find_local_text(elem: &XElem, name: &str) -> Option<String> {
    if local_name(&elem.name) == name {
        return Some(elem.text.trim().to_string());
    }
    elem.children
        .iter()
        .find_map(|child| find_local_text(child, name))
}

fn label_epoch_tdb(xml: &str, lsk: &LeapSeconds) -> Option<f64> {
    let doc = parse_xml(xml)?;
    let start = find_local_text(&doc, "start_date_time")?;
    parse_iso_tdb(&start, lsk)
}

fn comp_of_quantity(quantity: &str) -> Option<u32> {
    match quantity {
        "inc" => Some(vnrd::COMP_INC),
        "ocp" => Some(vnrd::COMP_OCP),
        "scp" => Some(vnrd::COMP_SCP),
        _ => None,
    }
}

fn comp_of_img(img: &Path) -> Option<u32> {
    let stem = img.file_stem()?.to_str()?;
    let quantity = stem.rsplit("_").next()?;
    comp_of_quantity(quantity)
}

fn product_of(img: PathBuf) -> Result<Product, String> {
    let Some(comp) = comp_of_img(&img) else {
        return Err(format!(
            "{} does not end in _inc, _ocp or _scp",
            img.display()
        ));
    };
    let label = img.with_extension("xml");
    Ok(Product { img, label, comp })
}

fn products_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("img") {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !name.starts_with("venus_") || !name.contains("_north_") {
            continue;
        }
        if comp_of_img(&path).is_none() {
            continue;
        }
        out.push(path);
    }
    out.sort();
    out
}

fn record_of(epoch: f64, mean: Option<f64>, comp: u32) -> Option<(f64, f64, u32)> {
    mean.map(|value| (epoch, value, comp))
}

fn write_series(rows: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vnrd::HEADER_BYTES + rows.len() * vnrd::REC_BYTES);
    out.extend_from_slice(&vnrd::MAGIC);
    out.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for &(t, v, comp) in rows {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
        out.extend_from_slice(&comp.to_le_bytes());
    }
    out
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == name {
            if let Some(value) = args.get(i + 1) {
                out.push(value.clone());
                i += 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    arg_values(args, name).pop()
}

fn usage() {
    eprintln!(
        "usage: venus_radar_compiler [--input <product.img>]... [--dir <directory>]... --output <series.bin> [--ci-mode]\n\
         one invocation bundles the level-2 Venus radar products into one VNRD asset:\n\
         t = the label epoch (TDB), v = the whole-map mean, comp = inc (0) | ocp (1) | scp (2)"
    );
}

fn upload_vnrd(tag: &str, path: &str, bytes: &[u8]) -> Result<(), String> {
    let asset = std::path::Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{path}: name is not utf-8 — the upload stays unraised"))?;
    if bytes.len() as u64 <= GITHUB_RELEASE_ASSET_MAX_BYTES {
        if !upload_release(tag, path) {
            return Err(format!("{tag}/{asset}: the CDN upload returned void"));
        }
        return Ok(());
    }
    let parent = match std::path::Path::new(path).parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir,
        _ => {
            return Err(format!(
                "{path}: carries no directory — the part upload stays unraised"
            ));
        }
    };
    let total = sha256_hex(bytes);
    let part_max = CDN_PART_MAX_BYTES as usize;
    let mut parts: Vec<(String, String, u64)> = Vec::new();
    for (idx, chunk) in bytes.chunks(part_max).enumerate() {
        let pname = part_name(asset, idx);
        let ppath = parent.join(&pname);
        std::fs::write(&ppath, chunk)
            .map_err(|e| format!("{}: write void ({e})", ppath.display()))?;
        let pstr = ppath.to_str().ok_or_else(|| {
            format!(
                "{}: name is not utf-8 — the part upload stays unraised",
                ppath.display()
            )
        })?;
        if !upload_release(tag, pstr) {
            return Err(format!("{tag}/{pname}: the CDN part upload returned void"));
        }
        parts.push((pname, sha256_hex(chunk), chunk.len() as u64));
    }
    let manifest = format_parts_manifest(&total, &parts);
    let mpath = parent.join(parts_manifest_name(asset));
    std::fs::write(&mpath, manifest)
        .map_err(|e| format!("{}: write void ({e})", mpath.display()))?;
    let mstr = mpath.to_str().ok_or_else(|| {
        format!(
            "{}: name is not utf-8 — the manifest upload stays unraised",
            mpath.display()
        )
    })?;
    if !upload_release(tag, mstr) {
        return Err(format!(
            "{tag}/{asset}: the CDN manifest upload returned void"
        ));
    }
    eprintln!(
        "{tag}/{asset}: {} B over the GitHub asset limit — split into {} part(s) + manifest",
        bytes.len(),
        parts.len()
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        std::process::exit(2);
    }
    let Some(output) = arg_value(&args, "--output") else {
        eprintln!("venus_radar_compiler: --output <series.bin> is required");
        usage();
        std::process::exit(2);
    };
    let mut imgs: Vec<PathBuf> = arg_values(&args, "--input")
        .into_iter()
        .map(PathBuf::from)
        .collect();
    for dir in arg_values(&args, "--dir") {
        imgs.extend(products_in_dir(Path::new(&dir)));
    }
    if imgs.is_empty() {
        eprintln!("venus_radar_compiler: no product given; pass --input or --dir");
        usage();
        std::process::exit(2);
    }
    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "venus_radar_compiler: the embedded naif0012 table stays unread — the epochs stay unconverted"
        );
        std::process::exit(2);
    };
    let mut rows: Vec<(f64, f64, u32)> = Vec::new();
    for img in &imgs {
        let product = match product_of(img.clone()) {
            Ok(product) => product,
            Err(message) => {
                eprintln!("venus_radar_compiler: {message} — the asset stays unwritten");
                std::process::exit(2);
            }
        };
        let xml = match std::fs::read_to_string(&product.label) {
            Ok(text) => text,
            Err(e) => {
                eprintln!(
                    "venus_radar_compiler: read {}: {e} — the label stays unread",
                    product.label.display()
                );
                std::process::exit(2);
            }
        };
        let Some(epoch) = label_epoch_tdb(&xml, &lsk) else {
            eprintln!(
                "venus_radar_compiler: {} carries no Time_Coordinates/start_date_time — the record stays absent",
                product.label.display()
            );
            std::process::exit(2);
        };
        let bytes = match std::fs::read(&product.img) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!(
                    "venus_radar_compiler: read {}: {e} — the map stays unread",
                    product.img.display()
                );
                std::process::exit(2);
            }
        };
        if bytes.len() != IMG_BYTES {
            eprintln!(
                "venus_radar_compiler: {} carries {} byte(s), the declared Array_2D_Map needs {IMG_BYTES} — the map stays unread",
                product.img.display(),
                bytes.len()
            );
            std::process::exit(2);
        }
        let mean = map_mean(&bytes, pds4_img::LINE_COUNT, pds4_img::SAMPLE_COUNT);
        match record_of(epoch, mean, product.comp) {
            Some(record) => rows.push(record),
            None => eprintln!(
                "venus_radar_compiler: {} carries no present sample — its record stays absent (0 honored)",
                product.img.display()
            ),
        }
    }
    rows.sort_by(|a, b| a.2.cmp(&b.2).then(a.0.total_cmp(&b.0)));
    if rows.is_empty() {
        eprintln!(
            "venus_radar_compiler: no product carried a present sample — the asset stays unwritten (0 honored)"
        );
        std::process::exit(2);
    }
    let bin = write_series(&rows);
    let Some(back) = vnrd::parse_bin(&bin) else {
        eprintln!(
            "venus_radar_compiler: the packed series does not read back — the asset stays unverified"
        );
        std::process::exit(1);
    };
    if back != rows {
        eprintln!(
            "venus_radar_compiler: the packed series differs on the roundtrip — the asset stays unverified"
        );
        std::process::exit(1);
    }
    if let Some(parent) = Path::new(&output).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if std::fs::write(&output, &bin).is_err() {
        eprintln!("venus_radar_compiler: write {output} void");
        std::process::exit(1);
    }
    if ci_mode && let Err(message) = upload_vnrd(NETLOC, &output, &bin) {
        eprintln!("venus_radar_compiler: {message}");
        std::process::exit(1);
    }
    for &(t, v, comp) in &rows {
        if let Some(name) = vnrd::component_name(comp) {
            println!("{name}  t={t:.6}  v={v:.6e}");
        }
    }
    println!(
        "{output}: {} record(s), {} byte(s), roundtrip {} point(s)",
        rows.len(),
        bin.len(),
        back.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const LABEL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Product_Observational xmlns="http://pds.nasa.gov/pds4/pds/v1">
  <Identification_Area>
    <logical_identifier>urn:nasa:pds:venus_radar_level2:data:venus_1988_north_inc</logical_identifier>
  </Identification_Area>
  <Observation_Area>
    <Time_Coordinates>
      <start_date_time>1988-06-17T14:58:20Z</start_date_time>
      <stop_date_time>1988-06-17T15:02:50Z</stop_date_time>
    </Time_Coordinates>
  </Observation_Area>
</Product_Observational>"#;

    const PREFIXED_LABEL: &str = r#"<pds:Product_Observational xmlns:pds="http://pds.nasa.gov/pds4/pds/v1">
  <pds:Observation_Area>
    <pds:Time_Coordinates>
      <pds:start_date_time>2012-05-10T00:00:00Z</pds:start_date_time>
    </pds:Time_Coordinates>
  </pds:Observation_Area>
</pds:Product_Observational>"#;

    fn frame(values: &[f32]) -> Vec<u8> {
        let mut out = Vec::with_capacity(values.len() * pds4_img::ELEMENT_BYTES);
        for value in values {
            out.extend_from_slice(&value.to_le_bytes());
        }
        out
    }

    #[test]
    fn label_epoch_reads_the_start_date_time() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let got = label_epoch_tdb(LABEL, &lsk).expect("the label carries a start_date_time");
        let want = parse_iso_tdb("1988-06-17T14:58:20Z", &lsk).expect("the epoch parses");
        assert_eq!(got, want);
    }

    #[test]
    fn label_epoch_reads_namespace_prefixed_elements() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let got =
            label_epoch_tdb(PREFIXED_LABEL, &lsk).expect("the label carries a start_date_time");
        let want = parse_iso_tdb("2012-05-10T00:00:00Z", &lsk).expect("the epoch parses");
        assert_eq!(got, want);
    }

    #[test]
    fn a_label_without_time_coordinates_leaves_the_epoch_absent() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        assert!(label_epoch_tdb("<Product_Observational/>", &lsk).is_none());
    }

    #[test]
    fn quantity_names_map_to_the_three_components() {
        assert_eq!(comp_of_quantity("inc"), Some(vnrd::COMP_INC));
        assert_eq!(comp_of_quantity("ocp"), Some(vnrd::COMP_OCP));
        assert_eq!(comp_of_quantity("scp"), Some(vnrd::COMP_SCP));
        assert_eq!(comp_of_quantity("mag"), None);
    }

    #[test]
    fn the_product_name_carries_the_quantity() {
        assert_eq!(
            comp_of_img(Path::new("venus_1988_north_inc.img")),
            Some(vnrd::COMP_INC)
        );
        assert_eq!(
            comp_of_img(Path::new("venus_2020_north_scp.img")),
            Some(vnrd::COMP_SCP)
        );
        assert_eq!(comp_of_img(Path::new("venus_2020_north_mag.img")), None);
    }

    #[test]
    fn the_whole_map_mean_averages_the_present_samples() {
        let bytes = frame(&[1.0, 2.0, 3.0, f32::NAN]);
        assert_eq!(map_mean(&bytes, 2, 2), Some(2.0));
    }

    #[test]
    fn an_absent_map_leaves_no_record() {
        let absent = frame(&[f32::NAN, f32::INFINITY]);
        assert_eq!(map_mean(&absent, 1, 2), None);
        assert_eq!(record_of(1.0, None, vnrd::COMP_OCP), None);
        assert_eq!(
            record_of(1.0, Some(2.5), vnrd::COMP_OCP),
            Some((1.0, 2.5, vnrd::COMP_OCP))
        );
    }

    #[test]
    fn records_order_by_component_then_epoch() {
        let mut rows = vec![(3.0, 1.0, 2), (1.0, 1.0, 0), (2.0, 1.0, 0), (5.0, 1.0, 1)];
        rows.sort_by(|a, b| a.2.cmp(&b.2).then(a.0.total_cmp(&b.0)));
        assert_eq!(
            rows,
            vec![(1.0, 1.0, 0), (2.0, 1.0, 0), (5.0, 1.0, 1), (3.0, 1.0, 2)]
        );
    }

    #[test]
    fn the_series_roundtrips_through_the_registered_reader() {
        let rows = vec![(1.0, 2.0, vnrd::COMP_INC), (3.0, 4.0, vnrd::COMP_OCP)];
        let bin = write_series(&rows);
        assert_eq!(vnrd::parse_bin(&bin), Some(rows));
    }
}
