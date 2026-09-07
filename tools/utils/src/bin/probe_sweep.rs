use omegaflow::archivar::{
    draft_context_mode, draft_url_mode, load_env, probe_mode, url_probe_mode,
};
use omegaflow_utils::discovery::{source_url_candidates_run, CANDIDATES_PATH};

const LIVE_PATH: &str = "phi/pipeline/probe_live.txt";
const DRAFTS_PATH: &str = "phi/pipeline/probe_drafts.φ";
const ENRICHED_PATH: &str = "phi/pipeline/probe_drafts_enriched.φ";
const WAVE_PATH: &str = "phi/pipeline/probe_wave.φ";
const SURVIVORS_PATH: &str = "phi/pipeline/probe_survivors.φ";
const VOID_PATH: &str = "phi/pipeline/probe_void.txt";
const REPORT_DIR: &str = "phi/reports";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut candidates: String = CANDIDATES_PATH.to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--candidates" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("probe_sweep --candidates <file>");
                    std::process::exit(2);
                }
                candidates = args[i].clone();
            }
            _ => {
                eprintln!("probe_sweep [--candidates <file>]");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let env = load_env();

    if std::path::Path::new(&candidates).exists() == false {
        let code = source_url_candidates_run();
        if code != 0 {
            eprintln!("probe_sweep: lens stopped (source_url_candidates)");
            std::process::exit(code);
        }
    }

    let mut code = url_probe_mode(&candidates, &env, false, true);
    if code != 0 {
        eprintln!("probe_sweep: urls stage stopped");
        std::process::exit(code);
    }

    code = draft_url_mode(LIVE_PATH, &env, false);
    if code != 0 {
        eprintln!("probe_sweep: draft stage stopped");
        std::process::exit(code);
    }

    code = draft_context_mode(DRAFTS_PATH);
    if code != 0 {
        eprintln!("probe_sweep: draft-context stage stopped");
        std::process::exit(code);
    }

    if drop_pending_blocks(ENRICHED_PATH, WAVE_PATH) != 0 {
        eprintln!("probe_sweep: dead-filter stopped");
        std::process::exit(1);
    }

    code = probe_mode(WAVE_PATH, false, 0.0, 0.0, &env, false);
    if code != 0 {
        eprintln!("probe_sweep: probe stage stopped");
        std::process::exit(code);
    }

    if copy_report(SURVIVORS_PATH, "probe_sweep_survivors.φ") != 0
        || copy_report(VOID_PATH, "probe_sweep_void.txt") != 0
    {
        std::process::exit(1);
    }
}

fn drop_pending_blocks(input: &str, output: &str) -> i32 {
    let content = match std::fs::read_to_string(input) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("dead-filter: read {}: unreadable", input);
            return 1;
        }
    };
    let mut kept: Vec<String> = Vec::new();
    let mut block: Vec<String> = Vec::new();
    let flush = |block: &mut Vec<String>, kept: &mut Vec<String>| {
        if !block.is_empty() {
            let pending = block.iter().any(|l| l.contains("frame: frame pending"));
            if !pending {
                kept.push(block.join("\n"));
            }
            block.clear();
        }
    };
    for line in content.lines() {
        if line.trim().is_empty() {
            flush(&mut block, &mut kept);
        } else {
            block.push(line.to_string());
        }
    }
    flush(&mut block, &mut kept);
    let mut out = String::new();
    for b in &kept {
        out.push_str(b);
        out.push_str("\n\n");
    }
    match std::fs::write(output, out) {
        Ok(()) => {
            eprintln!("dead-filter: {} blocks → {}", kept.len(), output);
            0
        }
        Err(_) => {
            eprintln!("dead-filter: write {}: unwritable", output);
            1
        }
    }
}

fn copy_report(src: &str, name: &str) -> i32 {
    let dst = format!("{}/{}", REPORT_DIR, name);
    match std::fs::copy(src, &dst) {
        Ok(_) => {
            eprintln!("probe_sweep: report → {}", dst);
            0
        }
        Err(e) => {
            eprintln!("probe_sweep: report copy {}: {}", dst, e);
            1
        }
    }
}
