use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::zip_members;
use omegaflow::rixs::parse_sw_spin;

const NETLOC: &str = "zenodo.org";
const SOURCE: &str = "https://zenodo.org/api/records/7286412/files/UnravellingNatureofSpinAndChargeExcitations-theory%2BexpData.zip/content";
const MEMBER_FILE: &str = "sw_spin.txt";
const SERIES_DIR: &str = "azimuthal_analysis";
const DEFAULT_OUT: &str = "data/zenodo.org";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn sample_of(member: &str) -> Option<String> {
    let parts: Vec<&str> = member.split('/').collect();
    if parts.last().copied() != Some(MEMBER_FILE) {
        return None;
    }
    let at = parts.iter().position(|p| *p == SERIES_DIR)?;
    parts.get(at + 1).map(|s| (*s).to_string())
}

fn series_text(sample: &str, raw: &str) -> String {
    let mut out =
        format!("# rixs spin series | sample {sample} | Eloss weight err | origin {SOURCE}\n");
    out.push_str(raw);
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn series_assets(zip: &[u8]) -> Result<Vec<(String, String)>, String> {
    let mut found: Vec<(String, Vec<u8>)> = Vec::new();
    let members = zip_members(zip, |name, data| {
        if let Some(sample) = sample_of(name) {
            found.push((sample, data.to_vec()));
        }
    });
    if members.is_none() {
        return Err("the zip carries no readable central directory".into());
    }
    let mut assets: Vec<(String, String)> = Vec::new();
    for (sample, bytes) in &found {
        let text = String::from_utf8_lossy(bytes);
        if parse_sw_spin(&text).is_none() {
            continue;
        }
        assets.push((sample.clone(), series_text(sample, &text)));
    }
    if assets.is_empty() {
        return Err("no sw_spin.txt member carries a parseable Eloss/weight series".into());
    }
    assets.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(assets)
}

fn emit(zip: &[u8], out_dir: &str, ci_mode: bool) -> Result<usize, String> {
    let assets = series_assets(zip)?;
    if !out_dir.is_empty() {
        std::fs::create_dir_all(out_dir).map_err(|e| format!("create {out_dir} returned {e}"))?;
    }
    let mut written = 0usize;
    for (sample, text) in &assets {
        let path = format!("{out_dir}/rixs_spin_{}.txt", sanitize(sample));
        std::fs::write(&path, text.as_bytes()).map_err(|e| format!("write {path} returned {e}"))?;
        println!(
            "url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{}",
            path.rsplit('/').next().unwrap_or(&path)
        );
        println!("origin {SOURCE}");
        println!("compiler rixs_series_compiler");
        println!("sha256 {}", sha256_hex(text.as_bytes()));
        println!("format rixs_series_text");
        println!("sample {sample}");
        written += 1;
        if ci_mode && !upload_release(NETLOC, &path) {
            return Err(format!("upload {path} returned void"));
        }
    }
    Ok(written)
}

fn stored_zip(members: &[(&str, &str)]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut central = Vec::new();
    for (name, body) in members {
        let local_off = out.len() as u32;
        let bytes = body.as_bytes();
        let name_b = name.as_bytes();
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name_b.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(name_b);
        out.extend_from_slice(bytes);

        central.extend_from_slice(b"PK\x01\x02");
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u32.to_le_bytes());
        central.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        central.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        central.extend_from_slice(&(name_b.len() as u16).to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u32.to_le_bytes());
        central.extend_from_slice(&local_off.to_le_bytes());
        central.extend_from_slice(name_b);
    }
    let cd_off = out.len() as u32;
    let cd_size = central.len() as u32;
    out.extend_from_slice(&central);
    out.extend_from_slice(b"PK\x05\x06");
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(members.len() as u16).to_le_bytes());
    out.extend_from_slice(&(members.len() as u16).to_le_bytes());
    out.extend_from_slice(&cd_size.to_le_bytes());
    out.extend_from_slice(&cd_off.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

fn selftest() {
    let ud = "# Eloss weight err\n0.10 1.0 0.1\n0.20 2.0 0.2\n";
    let od = "# Eloss weight err\n0.10 3.0 0.3\n0.20 4.0 0.4\n";
    let zip = stored_zip(&[
        ("exp_data/azimuthal_analysis/UD_0p33_0/sw_spin.txt", ud),
        (
            "__MACOSX/exp_data/azimuthal_analysis/UD_0p33_0/._sw_spin.txt",
            "junk",
        ),
        ("exp_data/azimuthal_analysis/OD1_0p33_0/sw_spin.txt", od),
        ("exp_data/exp_figure_data/fig3/ud_033_0_spin.txt", "1 2 3\n"),
    ]);
    let assets = match series_assets(&zip) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("selftest: series_assets void: {e}");
            std::process::exit(1);
        }
    };
    if assets.len() != 2 {
        eprintln!(
            "selftest: {} series found, 2 expected (the __MACOSX twin and the non-sw_spin member stay out)",
            assets.len()
        );
        std::process::exit(1);
    }
    if assets[0].0 != "OD1_0p33_0" || assets[1].0 != "UD_0p33_0" {
        eprintln!(
            "selftest: sample order {:?} is not the sorted (OD1, UD) pair",
            assets.iter().map(|a| &a.0).collect::<Vec<_>>()
        );
        std::process::exit(1);
    }
    let spec = match parse_sw_spin(&assets[1].1) {
        Some(s) => s,
        None => {
            eprintln!("selftest: the emitted UD text parses void");
            std::process::exit(1);
        }
    };
    if spec.eloss_ev != vec![0.10, 0.20]
        || spec.weight != vec![1.0, 2.0]
        || spec.err != vec![0.1, 0.2]
    {
        eprintln!(
            "selftest: the emitted UD series is not the measured (0.10,1.0,0.1)/(0.20,2.0,0.2) pair"
        );
        std::process::exit(1);
    }
    eprintln!("rixs_series_compiler: selftest passes (zip member → Eloss/weight/err text)");
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
    let zip = if let Some(path) = arg_value(&args, "--zip") {
        match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        }
    } else if let Some(url) = arg_value(&args, "--url") {
        match fetch_raw_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("{url}: fetch void — the text series stays unwritten");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "usage: rixs_series_compiler (--zip <path> | --url <url>) [--out <dir>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    };
    match emit(&zip, &out, ci_mode) {
        Ok(n) => eprintln!("rixs_series_compiler: {n} text series written to {out}"),
        Err(e) => {
            eprintln!("rixs_series_compiler: {e}");
            std::process::exit(1);
        }
    }
}
