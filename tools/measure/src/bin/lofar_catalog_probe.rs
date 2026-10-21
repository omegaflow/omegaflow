use std::io::Write;
use std::process::{Command, Stdio};

const UA: &str = "omegaflow-lofar-catalog-probe/1.0";
const TAP: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const SCHEMA: &str = "TAP_SCHEMA.tables";

struct Fetched {
    status: String,
    content_type: String,
    body: Vec<u8>,
}

struct Catalog {
    label: &'static str,
    table: &'static str,
    dat_url: Option<&'static str>,
    readme_url: &'static str,
    count_sql: &'static str,
    stats_sql: Option<&'static str>,
}

const CATALOGS: [Catalog; 3] = [
    Catalog {
        label: "LoTSS-DR1-quasar-sample",
        table: "J/A+A/622/A15/ldr1qsos",
        dat_url: Some("https://cdsarc.cds.unistra.fr/ftp/J/A+A/622/A15/ldr1qsos.dat"),
        readme_url: "https://cdsarc.cds.unistra.fr/ftp/J/A+A/622/A15/ReadMe",
        count_sql: "SELECT COUNT(*) AS n, COUNT(RAdeg) AS n_radio_position, COUNT(L144) AS n_l144 FROM \"J/A+A/622/A15/ldr1qsos\"",
        stats_sql: None,
    },
    Catalog {
        label: "LoLSS-DR1",
        table: "J/A+A/673/A165/lolss1s",
        dat_url: Some("https://cdsarc.cds.unistra.fr/ftp/J/A+A/673/A165/lolss1s.dat.gz"),
        readme_url: "https://cdsarc.cds.unistra.fr/ftp/J/A+A/673/A165/ReadMe",
        count_sql: "SELECT COUNT(*) AS n FROM \"J/A+A/673/A165/lolss1s\"",
        stats_sql: Some("SELECT AVG(e_RAJ2000) AS avg_e_ra, AVG(e_DEJ2000) AS avg_e_de, MAX(e_RAJ2000) AS max_e_ra FROM \"J/A+A/673/A165/lolss1s\""),
    },
    Catalog {
        label: "LoTSS-DR3-ILTJ",
        table: "J/A+A/707/A198/lotssdr3",
        dat_url: None,
        readme_url: "https://cdsarc.cds.unistra.fr/ftp/J/A+A/707/A198/ReadMe",
        count_sql: "SELECT COUNT(*) AS n FROM \"J/A+A/707/A198/lotssdr3\"",
        stats_sql: Some("SELECT AVG(e_RAJ2000) AS avg_e_ra, AVG(e_DEJ2000) AS avg_e_de FROM \"J/A+A/707/A198/lotssdr3\""),
    },
];

fn percent_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 2);
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn curl_get(url: &str) -> Option<Fetched> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-m")
        .arg("120")
        .arg("-A")
        .arg(UA)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}\t%{content_type}")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let meta = String::from_utf8_lossy(&stdout[idx + 1..]).trim().to_string();
    let mut fields = meta.split('\t');
    let status = fields.next().unwrap_or("absent").to_string();
    let content_type = fields.next().unwrap_or("absent").to_string();
    Some(Fetched {
        status,
        content_type,
        body: stdout[..idx].to_vec(),
    })
}

fn tap_url(query: &str) -> String {
    format!(
        "{TAP}?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY={}",
        percent_encode(query)
    )
}

fn tap_csv(query: &str) -> Option<Fetched> {
    curl_get(&tap_url(query))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut child = match Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return "absent".to_string(),
    };
    if let Some(mut stdin) = child.stdin.take() {
        if stdin.write_all(bytes).is_err() {
            return "absent".to_string();
        }
    }
    match child.wait_with_output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
            .split_whitespace()
            .next()
            .unwrap_or("absent")
            .to_string(),
        _ => "absent".to_string(),
    }
}

fn csv_rows(body: &[u8]) -> Vec<Vec<String>> {
    let text = String::from_utf8_lossy(body);
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut cells = Vec::new();
        let mut current = String::new();
        let mut quoted = false;
        for ch in line.chars() {
            match ch {
                '"' => quoted = !quoted,
                ',' if !quoted => {
                    cells.push(current.trim().to_string());
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        cells.push(current.trim().to_string());
        rows.push(cells);
    }
    rows
}

fn row_value(rows: &[Vec<String>], index: usize) -> String {
    match rows
        .get(1)
        .and_then(|row| row.get(index))
        .filter(|value| !value.is_empty())
        .cloned()
    {
        Some(value) => value,
        None => "absent".to_string(),
    }
}

fn readme_evidence(body: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(body);
    let mut out = Vec::new();
    for line in text.lines() {
        let lower = line.to_ascii_lowercase();
        let carries_frequency = lower.contains("mhz");
        let carries_resolution = lower.contains("arcsec") && lower.contains("resolution");
        if (carries_frequency || carries_resolution) && out.len() < 8 {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                out.push(trimmed.to_string());
            }
        }
    }
    out
}

fn probe_catalog(catalog: &Catalog) -> bool {
    println!(
        "catalog={} table={} vizier={}",
        catalog.label,
        catalog.table,
        format!("https://vizier.cds.unistra.fr/viz-bin/VizieR?-source={}", catalog.table)
    );

    let columns_query = format!("SELECT TOP 1 * FROM \"{}\"", catalog.table);
    let columns = tap_csv(&columns_query);
    let mut machine_readable = false;
    let mut column_count = 0usize;
    if let Some(fetched) = &columns {
        println!(
            "catalog={} columns_http={} columns_content_type={} columns_bytes={} columns_sha256={}",
            catalog.label,
            fetched.status,
            fetched.content_type,
            fetched.body.len(),
            sha256_hex(&fetched.body)
        );
        if fetched.status == "200" {
            let rows = csv_rows(&fetched.body);
            if let Some(header) = rows.first() {
                column_count = header.len();
                if column_count > 0 {
                    machine_readable = true;
                }
                println!(
                    "catalog={} columns_n={} columns={}",
                    catalog.label,
                    column_count,
                    header.join(",")
                );
            }
        }
    } else {
        println!("catalog={} columns_http=absent", catalog.label);
    }

    match tap_csv(catalog.count_sql) {
        Some(fetched) if fetched.status == "200" => {
            let rows = csv_rows(&fetched.body);
            let header = match rows.first() {
                Some(row) => row.clone(),
                None => Vec::new(),
            };
            if header.is_empty() {
                println!(
                    "catalog={} count_http=200 count=absent",
                    catalog.label
                );
                machine_readable = false;
            } else {
                for (index, name) in header.iter().enumerate() {
                    println!(
                        "catalog={} {}={}",
                        catalog.label,
                        name,
                        row_value(&rows, index)
                    );
                }
            }
        }
        Some(fetched) => {
            println!(
                "catalog={} count_http={} count=unreadable",
                catalog.label, fetched.status
            );
            machine_readable = false;
        }
        None => {
            println!("catalog={} count_http=absent count=unreadable", catalog.label);
            machine_readable = false;
        }
    }

    if let Some(stats_sql) = catalog.stats_sql {
        match tap_csv(stats_sql) {
            Some(fetched) if fetched.status == "200" => {
                let rows = csv_rows(&fetched.body);
                let header = match rows.first() {
                Some(row) => row.clone(),
                None => Vec::new(),
            };
                for (index, name) in header.iter().enumerate() {
                    println!(
                        "catalog={} {}={}",
                        catalog.label,
                        name,
                        row_value(&rows, index)
                    );
                }
            }
            Some(fetched) => {
                println!(
                    "catalog={} stats_http={} stats=unreadable",
                    catalog.label, fetched.status
                );
            }
            None => {
                println!("catalog={} stats_http=absent stats=unreadable", catalog.label);
            }
        }
    }

    if let Some(dat_url) = catalog.dat_url {
        match curl_get(dat_url) {
            Some(fetched) => {
                println!(
                    "catalog={} dat_url={} dat_http={} dat_content_type={} dat_bytes={} dat_sha256={}",
                    catalog.label,
                    dat_url,
                    fetched.status,
                    fetched.content_type,
                    fetched.body.len(),
                    sha256_hex(&fetched.body)
                );
            }
            None => {
                println!(
                    "catalog={} dat_url={} dat_http=absent",
                    catalog.label, dat_url
                );
            }
        }
    }

    match curl_get(catalog.readme_url) {
        Some(fetched) => {
            println!(
                "catalog={} readme_url={} readme_http={} readme_content_type={} readme_bytes={} readme_sha256={}",
                catalog.label,
                catalog.readme_url,
                fetched.status,
                fetched.content_type,
                fetched.body.len(),
                sha256_hex(&fetched.body)
            );
            for line in readme_evidence(&fetched.body) {
                println!("catalog={} readme-line={}", catalog.label, line);
            }
        }
        None => {
            println!(
                "catalog={} readme_url={} readme_http=absent",
                catalog.label, catalog.readme_url
            );
        }
    }

    println!(
        "catalog={} machine_readable={} column_count={}",
        catalog.label,
        if machine_readable { "yes" } else { "no" },
        column_count
    );
    machine_readable
}

fn probe_named_catalog(query: &str, label: &str) {
    match tap_csv(query) {
        Some(fetched) if fetched.status == "200" => {
            let rows = csv_rows(&fetched.body);
            let header = match rows.first() {
                Some(row) => row.clone(),
                None => Vec::new(),
            };
            if let Some(name) = header.first() {
                println!(
                    "search={} {}={}",
                    label,
                    name,
                    row_value(&rows, 0)
                );
            } else {
                println!("search={} result=unreadable", label);
            }
        }
        Some(fetched) => println!("search={} http={} result=unreadable", label, fetched.status),
        None => println!("search={} http=absent result=unreadable", label),
    }
}

fn main() {
    println!("probe=lofar_catalog_probe");
    println!("tap={TAP}");
    println!("schema={SCHEMA}");

    probe_named_catalog(
        "SELECT COUNT(*) AS n FROM TAP_SCHEMA.tables WHERE description LIKE '%LoVAR%'",
        "LoVAR-description",
    );
    probe_named_catalog(
        "SELECT COUNT(*) AS n FROM TAP_SCHEMA.tables WHERE table_name LIKE '%LoVAR%'",
        "LoVAR-table",
    );
    probe_named_catalog(
        "SELECT COUNT(*) AS n FROM TAP_SCHEMA.tables WHERE description LIKE '%LOFAR%'",
        "LOFAR-tables",
    );

    let mut readable = 0usize;
    for catalog in &CATALOGS {
        if probe_catalog(catalog) {
            readable += 1;
        }
    }
    println!(
        "summary=catalogs={} machine_readable={}",
        CATALOGS.len(),
        readable
    );
    println!("summary=lovar=absent");
}
