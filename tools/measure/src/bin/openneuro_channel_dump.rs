use std::env;
use std::process::exit;

use omegaflow::matfile::{MatArray, MatData, parse_mat};
use omegaflow_measure::eeglab::{
    EeglabSet, channel_series, open_set, open_set_bin, open_set_mat, resolve_channel,
};

fn load_set(path: &str) -> Option<(EeglabSet, Vec<f32>)> {
    open_set_bin(path)
        .or_else(|| open_set(path))
        .or_else(|| open_set_mat(path))
}

fn scalar_summary(kind: &str, len: usize, value: Option<f64>) -> String {
    match (len, value) {
        (1, Some(v)) => format!("{kind} = {v}"),
        (0, _) => format!("{kind} empty"),
        (n, _) => format!("{kind} [{n} value(s)]"),
    }
}

fn summary(a: &MatArray) -> String {
    match &a.data {
        MatData::Double(d) => scalar_summary("double", d.len(), d.first().copied()),
        MatData::Single(d) => scalar_summary("single", d.len(), d.first().map(|v| *v as f64)),
        MatData::Int32(d) => scalar_summary("int32", d.len(), d.first().map(|v| *v as f64)),
        MatData::Char(c) => {
            let text = String::from_utf8_lossy(c)
                .trim_end_matches('\0')
                .trim()
                .to_string();
            if text.is_empty() {
                format!("char [{} byte(s)]", c.len())
            } else {
                format!("char = \"{text}\"")
            }
        }
        MatData::Struct(fields) => {
            let names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
            format!("struct {{{}}}", names.join(", "))
        }
        MatData::Cell(cells) => format!("cell [{} element(s)]", cells.len()),
        MatData::Empty => "empty".to_string(),
    }
}

fn names_of(arrays: &[MatArray]) -> String {
    arrays
        .iter()
        .map(|a| a.name.as_str())
        .collect::<Vec<&str>>()
        .join(", ")
}

fn print_struct_fields(fields: &[omegaflow::matfile::MatField], indent: usize) {
    let pad = "  ".repeat(indent);
    for field in fields {
        for value in &field.values {
            println!("{pad}{} : {}", field.name, summary(value));
            if let MatData::Struct(inner) = &value.data {
                if matches!(field.name.as_str(), "etc" | "event" | "urevent") {
                    print_struct_fields(inner, indent + 1);
                }
            }
        }
    }
}

fn dump_header(path: &str) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("openneuro_channel_dump: {path} unreadable: {e}");
            exit(2);
        }
    };
    let Some(arrays) = parse_mat(&bytes) else {
        eprintln!("openneuro_channel_dump: {path} is not a MATLAB-5 MAT file");
        exit(2);
    };
    println!("MAT top-level at {path}: {}", names_of(&arrays));
    match arrays.iter().find(|a| a.name == "EEG") {
        Some(eeg) => match &eeg.data {
            MatData::Struct(fields) => {
                println!("EEG struct fields:");
                print_struct_fields(fields, 1);
            }
            _ => println!("EEG is not a struct"),
        },
        None => {
            println!("no EEG struct; top-level summaries:");
            for a in &arrays {
                println!("  {} : {}", a.name, summary(a));
            }
            for name in ["etc", "event"] {
                if let Some(a) = arrays.iter().find(|a| a.name == name) {
                    if let MatData::Struct(fields) = &a.data {
                        println!("{name} struct:");
                        print_struct_fields(fields, 1);
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut positional: Vec<String> = Vec::new();
    let mut channel_sel = "1".to_string();
    let mut header = false;
    let mut labels = false;
    let mut points: Option<usize> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--channel" => match args.get(i + 1) {
                Some(v) => {
                    channel_sel = v.clone();
                    i += 2;
                }
                None => {
                    eprintln!("openneuro_channel_dump: --channel needs a selector (n|label)");
                    exit(2);
                }
            },
            "--points" => match args.get(i + 1).and_then(|v| v.parse::<usize>().ok()) {
                Some(n) if n > 0 => {
                    points = Some(n);
                    i += 2;
                }
                _ => {
                    eprintln!("openneuro_channel_dump: --points needs a positive count");
                    exit(2);
                }
            },
            "--header" => {
                header = true;
                i += 1;
            }
            "--labels" => {
                labels = true;
                i += 1;
            }
            other => {
                positional.push(other.to_string());
                i += 1;
            }
        }
    }

    let Some(in_path) = positional.first() else {
        eprintln!(
            "openneuro_channel_dump <in.bin|in.set> <out.txt> [--channel <n|label>]\n\
             openneuro_channel_dump <in.set> --header | --labels"
        );
        exit(2);
    };

    if header {
        dump_header(in_path);
        return;
    }

    let Some((set, samples)) = load_set(in_path) else {
        eprintln!(
            "openneuro_channel_dump: {in_path} reads absent (no EEGB bin, .set/.fdt, or MAT-v5 EEG struct)"
        );
        exit(2);
    };

    if labels {
        println!(
            "openneuro_channel_dump: {in_path} | nbchan = {} | pnts = {} | trials = {} | srate = {}",
            set.nbchan,
            set.pnts,
            set.trials,
            match set.srate {
                Some(v) => format!("{v}"),
                None => "absent".to_string(),
            }
        );
        for (idx, label) in set.labels.iter().enumerate() {
            let shown = if label.is_empty() { "absent" } else { label };
            println!("{} {}", idx + 1, shown);
        }
        return;
    }

    let Some(out_path) = positional.get(1) else {
        eprintln!("openneuro_channel_dump: an output path is required");
        exit(2);
    };
    let Some(ch) = resolve_channel(&set, &channel_sel) else {
        eprintln!(
            "openneuro_channel_dump: channel [{channel_sel}] absent among {} channels",
            set.nbchan
        );
        exit(2);
    };
    let Some(series) = channel_series(&samples, &set, ch) else {
        eprintln!("openneuro_channel_dump: channel {ch} does not resolve against the sample block");
        exit(2);
    };

    let label: &str = match set.labels.get(ch) {
        Some(l) if !l.is_empty() => l.as_str(),
        _ => "absent",
    };
    let written = match points {
        Some(n) => series.len().min(n),
        None => series.len(),
    };
    let mut out = String::new();
    for v in &series[..written] {
        if !v.is_finite() {
            eprintln!("openneuro_channel_dump: sample [{v}] is not finite — absent, not written");
            exit(2);
        }
        out.push_str(&format!("{v}\n"));
    }
    if let Err(e) = std::fs::write(out_path, &out) {
        eprintln!("openneuro_channel_dump: {out_path} unreadable: {e}");
        exit(2);
    }

    println!(
        "openneuro_channel_dump: {in_path} -> {out_path} | channel {ch} ({label}) | \
         nbchan = {} | pnts = {} | trials = {} | written = {} of {}",
        set.nbchan,
        set.pnts,
        set.trials,
        written,
        series.len()
    );
}
