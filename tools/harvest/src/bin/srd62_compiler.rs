use omegaflow::cdn::upload_asset;
use omegaflow::suprastrom::{SuprastromBin, SuprastromPoint, SuprastromSeries};
use std::collections::BTreeMap;
use std::process::Command;

const ANGSTROM_M: f64 = 1.0e-10;
const LAMBDA_MIN_ANGSTROM: f64 = 50.0;
const LAMBDA_MAX_ANGSTROM: f64 = 500000.0;

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "curl {} http {}: {}",
            url,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn strip_tags(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&sigma;", "sigma")
        .replace("&amp;", "&")
        .replace("&#39;", "'")
        .replace("&delta;", "d")
        .trim()
        .to_string()
}

fn search_citations(html: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let needle = "href=\"/CeramicDataPortal/Hts/";
    let mut rest = html;
    while let Some(i) = rest.find(needle) {
        let after = &rest[i + needle.len()..];
        let id: String = after.chars().take_while(|c| *c != '"').collect();
        let id_len = id.len();
        if !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric()) && !ids.contains(&id) {
            ids.push(id);
        }
        rest = &after[id_len..];
    }
    ids
}

fn table_rows(table: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut rest = table;
    while let Some(ts) = rest.find("<tr") {
        let Some(te_rel) = rest[ts..].find("</tr>") else {
            break;
        };
        let te = ts + te_rel + 5;
        let row = &rest[ts..te];
        let mut cells = Vec::new();
        let mut cre = row;
        loop {
            let td = cre.find("<td");
            let th = cre.find("<th");
            let (tag, close) = match (td, th) {
                (Some(a), Some(b)) if a < b => ("<td", "</td>"),
                (Some(_), None) => ("<td", "</td>"),
                (None, Some(_)) => ("<th", "</th>"),
                _ => break,
            };
            let Some(cs) = cre.find(tag) else { break };
            let Some(ce_rel) = cre[cs..].find(close) else {
                break;
            };
            let ce = cs + ce_rel + close.len();
            cells.push(strip_tags(&cre[cs..ce]));
            cre = &cre[ce..];
        }
        if !cells.is_empty() {
            rows.push(cells);
        }
        rest = &rest[te..];
    }
    rows
}

fn parse_penetration(html: &str, source_id: &str) -> Vec<SuprastromSeries> {
    let mut series_map: BTreeMap<String, Vec<SuprastromPoint>> = BTreeMap::new();
    let mut rest = html;
    while let Some(ts) = rest.find("<table") {
        let Some(te_rel) = rest[ts..].find("</table>") else {
            break;
        };
        let te = ts + te_rel + 8;
        let table = &rest[ts..te];
        if !table.to_lowercase().contains("penetration depth") {
            rest = &rest[te..];
            continue;
        }
        let rows = table_rows(table);
        let header = rows.first().map(|r| r.clone()).unwrap_or_default();
        if header.is_empty() {
            rest = &rest[te..];
            continue;
        }
        let lower_hdr: Vec<String> = header.iter().map(|c| c.to_lowercase()).collect();
        let Some(pen_idx) = lower_hdr.iter().position(|c| c.contains("penetration")) else {
            rest = &rest[te..];
            continue;
        };
        let temp_idx = lower_hdr.iter().position(|c| c.contains("temperature"));
        if temp_idx.is_none() {
            eprintln!(
                "  {}: penetration table carries no temperature axis — not a rho_s(T) series (0 honored)",
                source_id
            );
            rest = &rest[te..];
            continue;
        }
        let Some(ti) = temp_idx else {
            rest = &rest[te..];
            continue;
        };
        let condition_idx: Vec<usize> = (0..header.len())
            .filter(|i| *i != pen_idx && *i != ti)
            .collect();
        for row in rows.iter().skip(1) {
            let lambda_cell = match row.get(pen_idx) {
                Some(c) => c,
                None => continue,
            };
            let Ok(lambda_a) = lambda_cell.trim().parse::<f64>() else {
                continue;
            };
            if !lambda_a.is_finite()
                || lambda_a < LAMBDA_MIN_ANGSTROM
                || lambda_a > LAMBDA_MAX_ANGSTROM
            {
                continue;
            }
            let temp_cell = match row.get(ti) {
                Some(c) => c,
                None => continue,
            };
            let Ok(t_k) = temp_cell.trim().parse::<f64>() else {
                continue;
            };
            if !t_k.is_finite() {
                continue;
            }
            let label = condition_idx
                .iter()
                .filter_map(|i| row.get(*i))
                .map(|c| c.trim().to_string())
                .filter(|c| !c.is_empty() && c != "---" && c != "--")
                .collect::<Vec<_>>()
                .join(" | ");
            series_map.entry(label).or_default().push(SuprastromPoint {
                t_k,
                lambda_m: lambda_a * ANGSTROM_M,
            });
        }
        rest = &rest[te..];
    }
    let mut series: Vec<SuprastromSeries> = series_map
        .into_iter()
        .map(|(label, mut points)| {
            points.sort_by(|a, b| {
                a.t_k
                    .partial_cmp(&b.t_k)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            points.dedup_by(|a, b| (a.t_k - b.t_k).abs() < 1e-6);
            SuprastromSeries {
                id: source_id.to_string(),
                label,
                points,
            }
        })
        .collect();
    series.retain(|s| !s.points.is_empty());
    series
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut ci_mode = false;
    let mut out_dir = String::from(".");
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--ci-mode" => ci_mode = true,
            "--out" => {
                if let Some(d) = args.get(i + 1) {
                    out_dir = d.clone();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    let out_path = format!("{}/srd62_suprastrom.bin", out_dir);

    let search_url = "https://srdata.nist.gov/CeramicDataPortal/Hts/DoSearch?Properties=44";
    let Some(search_html) = fetch(search_url) else {
        eprintln!("srd62_compiler: search carries void");
        std::process::exit(1);
    };
    let ids = search_citations(&search_html);
    eprintln!(
        "srd62_compiler: {} citations with Penetration Depth",
        ids.len()
    );

    let mut all_series: Vec<SuprastromSeries> = Vec::new();
    for id in &ids {
        let url = format!("https://srdata.nist.gov/CeramicDataPortal/Hts/{}", id);
        let Some(html) = fetch(&url) else {
            continue;
        };
        let series = parse_penetration(&html, id);
        for s in &series {
            eprintln!("  {} [{}] -> {} points", id, s.label, s.points.len());
        }
        all_series.extend(series);
    }
    let n_points: usize = all_series.iter().map(|s| s.points.len()).sum();
    eprintln!(
        "srd62_compiler: {} penetration-depth points across {} series",
        n_points,
        all_series.len()
    );

    let out = omegaflow::suprastrom::encode_suprastrom_bin(&SuprastromBin { series: all_series });
    if let Err(e) = std::fs::write(&out_path, &out) {
        eprintln!("srd62_compiler: write {} returned {}", out_path, e);
        std::process::exit(1);
    }
    eprintln!("srd62_compiler: wrote {} ({} points)", out_path, n_points);
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("srd62_compiler: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
