use omegaflow::archivar::gfc::{FUNCTIONALS, Functional, GfcModel};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn point_of(s: &str) -> Option<(f64, f64, f64)> {
    let mut it = s.split(',');
    let x: f64 = it.next()?.trim().parse().ok()?;
    let y: f64 = it.next()?.trim().parse().ok()?;
    let z: f64 = it.next()?.trim().parse().ok()?;
    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return None;
    }
    Some((x, y, z))
}

fn mean_anomaly_degrees(mean_deg: f64) -> f64 {
    let m = mean_deg % 360.0;
    if m < 0.0 { m + 360.0 } else { m }
}

fn run(args: &[String]) -> Option<()> {
    let path = arg_value(args, "--input")?;
    let point = point_of(&arg_value(args, "--point")?)?;
    let bytes = std::fs::read(&path).ok()?;
    let model = GfcModel::from_bytes(&bytes)?;
    let (x, y, z) = point;
    let r = (x * x + y * y + z * z).sqrt();
    let colat = (z / r).clamp(-1.0, 1.0).acos().to_degrees();
    let lon = mean_anomaly_degrees(y.atan2(x).to_degrees());
    model.evaluate(r, colat, lon, Functional::Potential)?;
    println!("input {path}");
    println!("degree {}", model.nmax);
    println!("r {r:.6} m");
    println!("colat {colat:.10} deg");
    println!("lon {lon:.10} deg");
    for functional in FUNCTIONALS {
        match model.evaluate(r, colat, lon, *functional) {
            Some(v) => println!("{} {:.12E} {}", functional.name(), v, functional.unit()),
            None => println!("{} pending {}", functional.name(), functional.unit()),
        }
    }
    Some(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("usage: gfc_probe --input <x.gfc> --point <x,y,z>");
        return;
    }
    if run(&args).is_none() {
        eprintln!("gfc_probe: no value — the input or the point stays unread");
        std::process::exit(1);
    }
}
