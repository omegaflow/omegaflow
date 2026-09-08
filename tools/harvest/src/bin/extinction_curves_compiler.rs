use std::env;
use std::process::Command;

const URL: &str =
    "https://vizier.cds.unistra.fr/viz-bin/asu-tsv?-source=J/ApJ/663/320/stars&-out.all";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl_body(url: &str) -> Vec<u8> {
    let tmp = env::temp_dir().join(format!("extc_{}.tsv", std::process::id()));
    let _ = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-g")
        .arg("-m")
        .arg("120")
        .arg("--connect-timeout")
        .arg("20")
        .arg("-o")
        .arg(&tmp)
        .arg(url)
        .output();
    let body = match std::fs::read(&tmp) {
        Ok(b) => b,
        Err(_) => Vec::new(),
    };
    let _ = std::fs::remove_file(&tmp);
    body
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("usage: extinction_curves_compiler --out <name>.json [--ci-mode]");
            return;
        }
    };
    let ci = args.iter().any(|a| a == "--ci-mode");

    let body = curl_body(URL);
    let text = String::from_utf8_lossy(&body);
    let mut header: Vec<&str> = Vec::new();
    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if header.is_empty() {
            header = line.split('\t').map(|c| c.trim()).collect();
            continue;
        }
        if line.trim_start().starts_with('-') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() >= header.len() {
            rows.push(cols);
        }
    }
    let idx = |name: &str| header.iter().position(|h| *h == name);
    let Some(i_name) = idx("Name") else {
        eprintln!("header carries no Name column");
        return;
    };
    let Some(i_ebv) = idx("E(B-V)") else {
        eprintln!("header carries no E(B-V) column");
        return;
    };
    let Some(i_e_ebv) = idx("e_E(B-V)") else {
        eprintln!("header carries no e_E(B-V) column");
        return;
    };
    let Some(i_rv) = idx("R(V)") else {
        eprintln!("header carries no R(V) column");
        return;
    };
    let Some(i_e_rv) = idx("e_R(V)") else {
        eprintln!("header carries no e_R(V) column");
        return;
    };
    let opt = |name: &str| idx(name);
    let i_x0 = opt("x0");
    let i_gamma = opt("gamma");
    let i_c1 = opt("c1");
    let i_c2 = opt("c2");
    let i_c3 = opt("c3");
    let i_c4 = opt("c4");
    let i_c5 = opt("c5");
    let i_ra = opt("_RA");
    let i_dec = opt("_DE");

    let num = |cols: &[&str], i: Option<usize>| -> String {
        match i.and_then(|j| cols.get(j)) {
            Some(s) => match s.trim().parse::<f64>() {
                Ok(v) if v.is_finite() => v.to_string(),
                _ => "null".to_string(),
            },
            None => "null".to_string(),
        }
    };
    let esc = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");

    let mut out_lines: Vec<String> = Vec::with_capacity(rows.len());
    for cols in rows {
        let name = cols.get(i_name).map(|s| s.trim()).unwrap_or("");
        if name.is_empty() {
            continue;
        }
        let mut obj = String::from("{\"name\":\"");
        obj.push_str(&esc(name));
        obj.push_str("\",\"ra\":");
        obj.push_str(&num(&cols, i_ra));
        obj.push_str(",\"dec\":");
        obj.push_str(&num(&cols, i_dec));
        obj.push_str(",\"ebv\":");
        obj.push_str(&num(&cols, Some(i_ebv)));
        obj.push_str(",\"e_ebv\":");
        obj.push_str(&num(&cols, Some(i_e_ebv)));
        obj.push_str(",\"rv\":");
        obj.push_str(&num(&cols, Some(i_rv)));
        obj.push_str(",\"e_rv\":");
        obj.push_str(&num(&cols, Some(i_e_rv)));
        obj.push_str(",\"x0\":");
        obj.push_str(&num(&cols, i_x0));
        obj.push_str(",\"gamma\":");
        obj.push_str(&num(&cols, i_gamma));
        for key in [
            ("c1", i_c1),
            ("c2", i_c2),
            ("c3", i_c3),
            ("c4", i_c4),
            ("c5", i_c5),
        ] {
            obj.push_str(",\"");
            obj.push_str(key.0);
            obj.push_str("\":");
            obj.push_str(&num(&cols, key.1));
        }
        obj.push('}');
        out_lines.push(obj);
    }
    let mut json = String::from("[");
    json.push_str(&out_lines.join(","));
    json.push_str("]\n");
    if std::fs::write(&out, &json).is_err() {
        eprintln!("{} unwritable", out);
        return;
    }
    eprintln!(
        "extinction_curves: {} sightlines written to {}",
        out_lines.len(),
        out
    );
    if ci {
        let ok = omegaflow::cdn::upload_asset(&out);
        eprintln!("upload {}: {}", out, ok);
    }
}
