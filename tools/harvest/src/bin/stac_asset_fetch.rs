use omegaflow::archivar::fetch_raw_bytes_headers;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::stac::{parse_collection_ids, parse_items, select_asset};

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn auth_headers(token_env: &str) -> Vec<(String, String)> {
    match std::env::var(token_env) {
        Ok(t) if !t.is_empty() => vec![("Authorization".to_string(), format!("Bearer {t}"))],
        _ => Vec::new(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let token_env = match arg_value(&args, "--token-env") {
        Some(v) => v,
        None => "CDSE_TOKEN".to_string(),
    };
    let headers = auth_headers(&token_env);

    if let Some(url) = arg_value(&args, "--collections") {
        let Some(bytes) = fetch_raw_bytes_headers(&url, &headers) else {
            eprintln!("collections fetch returned void ({url})");
            std::process::exit(1);
        };
        let Some(ids) = parse_collection_ids(&bytes) else {
            eprintln!("no collection id in the response ({url})");
            std::process::exit(1);
        };
        for id in ids {
            println!("{id}");
        }
        return;
    }

    if let Some(url) = arg_value(&args, "--items") {
        let Some(bytes) = fetch_raw_bytes_headers(&url, &headers) else {
            eprintln!("items fetch returned void ({url})");
            std::process::exit(1);
        };
        let Some(items) = parse_items(&bytes) else {
            eprintln!("no STAC item in the response ({url})");
            std::process::exit(1);
        };
        let prefer_spec = match arg_value(&args, "--prefer") {
            Some(v) => v,
            None => "geotiff,netcdf".to_string(),
        };
        let prefer: Vec<&str> = prefer_spec.split(',').collect();
        for it in &items {
            match select_asset(it, &prefer) {
                Some(a) => println!(
                    "{} {} {} {}",
                    it.id,
                    a.key,
                    a.media_type.as_deref().unwrap_or("-"),
                    a.href
                ),
                None => println!("{} - - (no asset)", it.id),
            }
        }
        return;
    }

    if let Some(url) = arg_value(&args, "--asset") {
        let Some(bytes) = fetch_raw_bytes_headers(&url, &headers) else {
            eprintln!("asset fetch returned void ({url})");
            std::process::exit(1);
        };
        if bytes.is_empty() {
            eprintln!("asset returned empty (0 honored) ({url})");
            std::process::exit(1);
        }
        println!("{} {} {}", bytes.len(), sha256_hex(&bytes), url);
        return;
    }

    eprintln!(
        "stac_asset_fetch: --collections <url> | --items <url> [--prefer geotiff,netcdf] | --asset <url> [--token-env CDSE_TOKEN]"
    );
    std::process::exit(2);
}
