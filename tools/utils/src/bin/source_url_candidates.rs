use omegaflow_utils::discovery::source_url_candidates_run;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        eprintln!("source_url_candidates: no arguments");
        std::process::exit(2);
    }
    std::process::exit(source_url_candidates_run());
}
