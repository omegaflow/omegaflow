fn main() {
    if let Ok(path) = std::env::var("OMEGAFLOW_FIT_SAMPLE") {
        fit_sample_dump(&path);
    }
    omegaflow::archivar::main_flow()
}

fn fit_sample_dump(path: &str) -> ! {
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(read) => {
            println!("fit: {path} not readable: {read}");
            std::process::exit(2);
        }
    };
    match omegaflow::archivar::fit::parse_fit(&bytes) {
        None => {
            println!("fit: {path} refused by the header/CRC gate");
            std::process::exit(2);
        }
        Some(batch) => {
            let mut nn_count = 0usize;
            let mut nn_min: Option<f64> = None;
            let mut nn_max: Option<f64> = None;
            let mut all_finite = true;
            for (key, value, _) in &batch {
                if !value.is_finite() {
                    all_finite = false;
                }
                if key == "nn" {
                    nn_count += 1;
                    nn_min = Some(match nn_min {
                        Some(current) => current.min(*value),
                        None => *value,
                    });
                    nn_max = Some(match nn_max {
                        Some(current) => current.max(*value),
                        None => *value,
                    });
                }
            }
            println!("fit: {path} parsed");
            println!("records: {}", batch.len());
            println!("nn: {nn_count}");
            match (nn_min, nn_max) {
                (Some(min), Some(max)) => {
                    println!("nn min: {min}");
                    println!("nn max: {max}");
                }
                _ => println!("nn min/max: absent"),
            }
            println!("all finite: {all_finite}");
            std::process::exit(0);
        }
    }
}
