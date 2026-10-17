use std::env;
use std::process::exit;

use omegaflow::cdf::CdfFile;

const WANTED: [&str; 7] = [
    "Epoch",
    "Geo_Lat",
    "Geo_Lon",
    "X_GEI_Norm_Orb",
    "Y_GEI_Norm_Orb",
    "Z_GEI_Norm_Orb",
    "Greenwich_Time_Rad",
];

fn read_var(bytes: &[u8], file: &CdfFile, name: &str) -> Result<Vec<f64>, String> {
    let var = file
        .vars
        .iter()
        .find(|v| v.name == name)
        .ok_or_else(|| format!("{} absent", name))?;
    let meta = format!(
        "sparse={} record_vary={} max_rec={}",
        var.sparse, var.record_vary, var.max_rec
    );
    let records = file
        .var_records(bytes, var)
        .map_err(|e| format!("{}: {:?}", meta, e))?;
    Ok(records
        .into_iter()
        .filter_map(|(_, values)| values.into_iter().next())
        .collect())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (Some(input), Some(output)) = (args.first(), args.get(1)) else {
        eprintln!("usage: de2_orbit_probe <cdf> <out.txt>");
        exit(2);
    };
    let bytes = match std::fs::read(input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}: {}", input, e);
            exit(2);
        }
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {:?}", input, e);
            exit(2);
        }
    };
    let mut columns: Vec<Vec<f64>> = Vec::new();
    for name in WANTED {
        match read_var(&bytes, &file, name) {
            Ok(v) => columns.push(v),
            Err(why) => {
                eprintln!("variable not readable: {}", why);
                exit(2);
            }
        }
    }
    let Some(records) = columns.iter().map(Vec::len).min() else {
        eprintln!("no columns read");
        exit(2);
    };
    let mut body = String::new();
    for i in 0..records {
        let row: Vec<String> = columns.iter().map(|c| format!("{:.6}", c[i])).collect();
        body.push_str(&row.join(" "));
        body.push('\n');
    }
    match std::fs::write(output, &body) {
        Ok(()) => println!(
            "de2_orbit_probe: {} records -> {} (columns: {:?})",
            records, output, WANTED
        ),
        Err(e) => {
            eprintln!("{}: {}", output, e);
            exit(2);
        }
    }
}
