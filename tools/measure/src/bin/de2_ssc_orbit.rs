use std::env;
use std::process::exit;

fn tag_values<'a>(xml: &'a str, tag: &str) -> Vec<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(i) = rest.find(&open) {
        let after = &rest[i + open.len()..];
        let Some(j) = after.find(&close) else {
            break;
        };
        out.push(after[..j].trim());
        rest = &after[j + close.len()..];
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (Some(input), Some(output)) = (args.first(), args.get(1)) else {
        eprintln!("usage: de2_ssc_orbit <sscweb-locations.xml> <out.csv>");
        exit(2);
    };
    let xml = match std::fs::read_to_string(input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{input}: {e}");
            exit(2);
        }
    };
    let times = tag_values(&xml, "Time");
    let radii = tag_values(&xml, "RadialLength");
    let locals = tag_values(&xml, "LocalTime");
    let lats = tag_values(&xml, "Latitude");
    let lons = tag_values(&xml, "Longitude");
    let n = times
        .len()
        .min(radii.len())
        .min(locals.len())
        .min(lats.len())
        .min(lons.len());
    if n == 0 {
        eprintln!("{input}: no Time/RadialLength/LocalTime/Latitude/Longitude records");
        exit(2);
    }
    let mut body = String::new();
    for i in 0..n {
        let Ok(r) = radii[i].parse::<f64>() else {
            continue;
        };
        let Ok(lt) = locals[i].parse::<f64>() else {
            continue;
        };
        let Ok(lat) = lats[i].parse::<f64>() else {
            continue;
        };
        let Ok(lon) = lons[i].parse::<f64>() else {
            continue;
        };
        if !(r.is_finite() && lt.is_finite() && lat.is_finite() && lon.is_finite()) {
            continue;
        }
        body.push_str(&format!(
            "{},{:.6},{:.6},{:.6},{:.6}\n",
            times[i], r, lt, lat, lon
        ));
    }
    match std::fs::write(output, &body) {
        Ok(()) => println!("de2_ssc_orbit: {n} records -> {output}"),
        Err(e) => {
            eprintln!("{output}: {e}");
            exit(2);
        }
    }
}
