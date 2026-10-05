use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::opendap::{AsciiVar, parse_ascii};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "data.ceda.ac.uk-gebco";
const DEFAULT_OUT: &str = "data/data.ceda.ac.uk-gebco";
const DEFAULT_VAR: &str = "elevation";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == name {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
                i += 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn var_leaf(name: &str) -> &str {
    match name.rfind('.') {
        Some(i) => &name[i + 1..],
        None => name,
    }
}

fn epoch_of(url: &str) -> Option<String> {
    for seg in url.split(|c: char| c == '/' || c == '?') {
        let low = seg.to_ascii_lowercase();
        if low.starts_with("gebco_") {
            let tail: String = seg
                .chars()
                .skip(6)
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if !tail.is_empty() {
                return Some(format!("gebco_{tail}"));
            }
        }
    }
    None
}

fn select_var<'a>(vars: &'a [AsciiVar], want: &str) -> Option<&'a AsciiVar> {
    vars.iter()
        .find(|v| var_leaf(&v.name) == want && v.shape.len() >= 2)
        .or_else(|| {
            vars.iter()
                .find(|v| v.name == want || v.name.ends_with(&format!(".{want}")))
        })
        .or_else(|| vars.iter().find(|v| v.shape.len() >= 2))
}

fn series_text(epoch: &str, var: &AsciiVar, url: &str) -> String {
    let shape: Vec<String> = var.shape.iter().map(|n| n.to_string()).collect();
    let mut out = format!(
        "# gbco {} series | epoch {epoch} | shape {} | axis row-major cell index | value {} | origin {url}\n",
        var.name,
        shape.join("x"),
        var.name,
    );
    for (i, v) in var.values.iter().enumerate() {
        out.push_str(&format!("{i} {v}\n"));
    }
    out
}

fn series_from(
    ascii_text: &str,
    epoch: &str,
    want: &str,
    url: &str,
) -> Result<(String, String, Vec<usize>), String> {
    let vars = parse_ascii(ascii_text).map_err(|e| format!("DAP2-ASCII parse {e:?}"))?;
    let var = select_var(&vars, want)
        .ok_or_else(|| format!("no variable '{want}' (or any 2D grid) in the DAP2-ASCII body"))?;
    if var.values.is_empty() {
        return Err(format!("variable '{}' carries no values", var.name));
    }
    let asset = format!(
        "gbco_{}_{}.txt",
        sanitize(var_leaf(&var.name)),
        sanitize(epoch)
    );
    Ok((asset, series_text(epoch, var, url), var.shape.clone()))
}

fn emit(urls: &[String], out_dir: &str, want: &str, ci_mode: bool) -> Result<usize, String> {
    let mut built: Vec<(String, String, Vec<usize>, String, String)> = Vec::new();
    for url in urls {
        let Some(bytes) = fetch_raw_bytes(url) else {
            return Err(format!("{url}: fetch void — the series stays unwritten"));
        };
        let text = String::from_utf8_lossy(&bytes);
        let epoch = epoch_of(url).ok_or_else(|| {
            format!("{url}: no gebco_<year> token — the epoch stays unnamed, refused")
        })?;
        let (asset, series, shape) = series_from(&text, &epoch, want, url)?;
        built.push((asset, series, shape, epoch, url.clone()));
    }
    let Some((_, _, first_shape, _, _)) = built.first() else {
        return Err("no URL given — nothing to emit".to_string());
    };
    let first_shape = first_shape.clone();
    if built
        .iter()
        .any(|(_, _, shape, _, _)| *shape != first_shape)
    {
        return Err(format!(
            "epoch grids differ in shape ({first_shape:?}) — the shared index axis would misalign, refused"
        ));
    }
    if !out_dir.is_empty() {
        std::fs::create_dir_all(out_dir).map_err(|e| format!("create {out_dir} returned {e}"))?;
    }
    let mut written = 0usize;
    for (asset, series, _, epoch, origin) in &built {
        let path = format!("{out_dir}/{asset}");
        std::fs::write(&path, series.as_bytes())
            .map_err(|e| format!("write {path} returned {e}"))?;
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
        println!("origin {origin}");
        println!("compiler gbco_series_compiler");
        println!("sha256 {}", sha256_hex(series.as_bytes()));
        println!("format gbco_axis_value_text");
        println!("epoch {epoch}");
        written += 1;
        if ci_mode && !upload_release(NETLOC, &path) {
            return Err(format!("upload {path} returned void"));
        }
    }
    Ok(written)
}

fn selftest() {
    let ascii = "Dataset {\n    Grid {\n     ARRAY:\n        Int16 elevation[lat = 2][lon = 2];\n     MAPS:\n        Float64 lat[lat = 2];\n        Float64 lon[lon = 2];\n    } elevation;\n} bodc/gebco/global/gebco_2023/ice_surface_elevation/netcdf/GEBCO_2023_CF.nc;\n---------------------------------------------\nelevation.elevation[2][2]\n[0], 2829, 2829\n[1], 2830, 2830\n\nelevation.lat[2]\n-89.99791666666667, -89.99375\n\nelevation.lon[2]\n-179.99791666666667, -179.99375\n\n";
    let (asset, text) = match series_from(
        ascii,
        "gebco_2023",
        DEFAULT_VAR,
        "https://example.invalid/GEBCO_2023_CF.nc.ascii?elevation[0:1][0:1]",
    ) {
        Ok((asset, text, _shape)) => (asset, text),
        Err(e) => {
            eprintln!("selftest: series_from void: {e}");
            std::process::exit(1);
        }
    };
    if asset != "gbco_elevation_gebco_2023.txt" {
        eprintln!("selftest: asset {asset} is not the measured name");
        std::process::exit(1);
    }
    let lines: Vec<&str> = text.lines().collect();
    if !lines.first().is_some_and(|l| l.starts_with('#')) {
        eprintln!("selftest: the series carries no leading '#' comment");
        std::process::exit(1);
    }
    let pairs: Vec<(f64, f64)> = lines
        .iter()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let mut n = l.split_whitespace().filter_map(|t| t.parse::<f64>().ok());
            Some((n.next()?, n.next()?))
        })
        .collect();
    if pairs != vec![(0.0, 2829.0), (1.0, 2829.0), (2.0, 2830.0), (3.0, 2830.0)] {
        eprintln!("selftest: the emitted axis/value pairs are not the measured grid");
        std::process::exit(1);
    }
    eprintln!("gbco_series_compiler: selftest passes (DAP2-ASCII grid → axis/value text)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let want = match arg_value(&args, "--var") {
        Some(v) => v,
        None => DEFAULT_VAR.to_string(),
    };
    let urls = arg_values(&args, "--url");
    if urls.is_empty() {
        eprintln!(
            "usage: gbco_series_compiler --url <dap2-ascii-url> [--url <second-epoch-url> ...] [--var {DEFAULT_VAR}] [--out <dir>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    match emit(&urls, &out, &want, ci_mode) {
        Ok(n) => eprintln!("gbco_series_compiler: {n} axis/value series written to {out}"),
        Err(e) => {
            eprintln!("gbco_series_compiler: {e}");
            std::process::exit(1);
        }
    }
}
