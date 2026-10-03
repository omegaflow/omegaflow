use omegaflow::archivar::json::{JsonVal, parse_json};
use std::env;
use std::fs;
use std::process::Command;

const API: &str = "https://www.kaggle.com/api/v1";

fn usage() -> ! {
    eprintln!(
        "kaggle_kernel — push a Kaggle kernel and read its status/output (std + curl)\n\
         usage:\n\
         \x20 kaggle_kernel push <dir>            # dir with kernel-metadata.json + code_file\n\
         \x20 kaggle_kernel status <owner/slug>\n\
         \x20 kaggle_kernel output <owner/slug>\n\
         token: $KAGGLE_CONFIG_DIR/kaggle.json or ~/.kaggle/kaggle.json (the operator's hand)"
    );
    std::process::exit(2);
}

fn absent(what: &str) -> ! {
    eprintln!("kaggle_kernel: {what} absent — the token is the operator's hand");
    std::process::exit(2);
}

fn credentials() -> (String, String) {
    let home = match env::var("HOME") {
        Ok(h) => h,
        Err(_) => absent("$HOME"),
    };
    let dir = match env::var("KAGGLE_CONFIG_DIR") {
        Ok(d) => d,
        Err(_) => format!("{home}/.kaggle"),
    };
    let path = format!("{dir}/kaggle.json");
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => absent(&path),
    };
    let v = match parse_json(&text) {
        Some(v) => v,
        None => {
            eprintln!("kaggle_kernel: {path} carries no JSON");
            std::process::exit(2);
        }
    };
    let JsonVal::Obj(map) = v else {
        eprintln!("kaggle_kernel: {path} is not an object");
        std::process::exit(2);
    };
    let field = |k: &str| match map.get(k) {
        Some(JsonVal::Str(s)) => s.clone(),
        _ => {
            eprintln!("kaggle_kernel: {path}: field {k} absent");
            std::process::exit(2);
        }
    };
    (field("username"), field("key"))
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

fn run_curl(args: &[String], user: &str, key: &str) -> String {
    let out = match Command::new("sh")
        .arg("-c")
        .arg(r#"curl -sS --max-time 300 -u "$KAGGLE_USER:$KAGGLE_KEY" "$@""#)
        .arg("kaggle_kernel")
        .args(args)
        .env("KAGGLE_USER", user)
        .env("KAGGLE_KEY", key)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("kaggle_kernel: curl did not run: {e}");
            std::process::exit(2);
        }
    };
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    if !out.status.success() {
        s.push_str(&String::from_utf8_lossy(&out.stderr));
    }
    s
}

fn read_meta(dir: &str) -> std::collections::HashMap<String, JsonVal> {
    let path = format!("{dir}/kernel-metadata.json");
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("kaggle_kernel: {path}: {e}");
            std::process::exit(2);
        }
    };
    let v = match parse_json(&text) {
        Some(v) => v,
        None => {
            eprintln!("kaggle_kernel: {path} carries no JSON");
            std::process::exit(2);
        }
    };
    match v {
        JsonVal::Obj(m) => m,
        _ => {
            eprintln!("kaggle_kernel: {path} is not an object");
            std::process::exit(2);
        }
    }
}

fn str_field(m: &std::collections::HashMap<String, JsonVal>, k: &str) -> String {
    match m.get(k) {
        Some(JsonVal::Str(s)) => s.clone(),
        _ => {
            eprintln!("kaggle_kernel: metadata field {k} absent");
            std::process::exit(2);
        }
    }
}

fn bool_field(m: &std::collections::HashMap<String, JsonVal>, k: &str, default: bool) -> bool {
    match m.get(k) {
        Some(JsonVal::Bool(b)) => *b,
        _ => default,
    }
}

fn push(dir: &str, user: &str, key: &str) -> String {
    let m = read_meta(dir);
    let slug = str_field(&m, "id");
    let title = str_field(&m, "title");
    let language = str_field(&m, "language");
    let kernel_type = str_field(&m, "kernel_type");
    let code_file = str_field(&m, "code_file");
    let code_path = format!("{dir}/{code_file}");
    let code = match fs::read_to_string(&code_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("kaggle_kernel: {code_path}: {e}");
            std::process::exit(2);
        }
    };
    let body = format!(
        r#"{{"slug":"{}","newTitle":"{}","language":"{}","kernelType":"{}","text":"{}","isPrivate":{},"enableGpu":{},"enableTpu":{},"enableInternet":{},"datasetDataSources":[],"competitionDataSources":[],"kernelDataSources":[],"categoryIds":[]}}"#,
        json_escape(&slug),
        json_escape(&title),
        json_escape(&language),
        json_escape(&kernel_type),
        json_escape(&code),
        bool_field(&m, "is_private", true),
        bool_field(&m, "enable_gpu", false),
        bool_field(&m, "enable_tpu", false),
        bool_field(&m, "enable_internet", true),
    );
    let tmp = format!("/tmp/opencode/kaggle_push_{}.json", std::process::id());
    if let Err(e) = fs::write(&tmp, &body) {
        eprintln!("kaggle_kernel: write {tmp}: {e}");
        std::process::exit(2);
    }
    let args = vec![
        "-H".to_string(),
        "Content-Type: application/json".to_string(),
        "--data-binary".to_string(),
        format!("@{tmp}"),
        format!("{API}/kernels/push"),
    ];
    let resp = run_curl(&args, user, key);
    let _ = fs::remove_file(&tmp);
    resp
}

fn read_endpoint(endpoint: &str, target: &str, user: &str, key: &str) -> String {
    let (who, slug) = match target.split_once('/') {
        Some(x) => x,
        None => {
            eprintln!("kaggle_kernel: target must be <owner>/<slug>");
            std::process::exit(2);
        }
    };
    let url = format!("{API}/kernels/{endpoint}?userName={who}&kernelSlug={slug}");
    run_curl(&[url], user, key)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(cmd) = args.get(1) else { usage() };
    let (user, key) = credentials();
    match cmd.as_str() {
        "push" => {
            let Some(dir) = args.get(2) else { usage() };
            println!("{}", push(dir, &user, &key));
        }
        "status" => {
            let Some(target) = args.get(2) else { usage() };
            println!("{}", read_endpoint("status", target, &user, &key));
        }
        "output" => {
            let Some(target) = args.get(2) else { usage() };
            println!("{}", read_endpoint("output", target, &user, &key));
        }
        _ => usage(),
    }
}
