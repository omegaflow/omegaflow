use omegaflow::archivar::build_frame_registry;

const DEFAULT_OUT: &str = "phi/pipeline/frame_registry.φ";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_path = DEFAULT_OUT.to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("frame_registry [--out <path>]");
                    std::process::exit(2);
                }
                out_path = args[i].clone();
            }
            other => {
                eprintln!("frame_registry: unknown argument '{}'", other);
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let registry = build_frame_registry();
    let mut reg = String::from(
        "# frame-registry — route (host/path, query stripped) → frame, self-learning from sources.φ + dead_sources.φ + blocked_sources.φ + frame_learned.φ\n",
    );
    let mut reg_keys: Vec<(&String, &String)> = registry.iter().collect();
    reg_keys.sort();
    for (nl, f) in reg_keys {
        reg.push_str(&format!("{} | {}\n", nl, f));
    }

    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("frame_registry: create {}: {}", parent.display(), e);
                std::process::exit(1);
            }
        }
    }
    if let Err(e) = std::fs::write(&out_path, reg) {
        eprintln!("frame_registry: write {}: {}", out_path, e);
        std::process::exit(1);
    }
    println!("frame_registry: {} routes → {}", registry.len(), out_path);
}
