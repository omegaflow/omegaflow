use omegaflow::archivar::{JsonVal, fetch_raw_bytes, jnum, jstr, parse_json};
use omegaflow::cdn::upload_release;
use std::io::Write;

const NETLOC: &str = "prop.kc2g.com";
const URL: &str = "https://prop.kc2g.com/api/stations.json";

#[derive(Clone, Debug)]
struct Station {
    code: String,
    lat_deg: f64,
    lon_deg: f64,
    mufd_mhz: Option<f64>,
    fof2_mhz: Option<f64>,
    tec_tecu: Option<f64>,
    cs: Option<f64>,
    time_unix: Option<f64>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn finite(v: Option<f64>) -> Option<f64> {
    match v {
        Some(x) if x.is_finite() => Some(x),
        _ => None,
    }
}

fn parse_stations(text: &str) -> Option<Vec<Station>> {
    let json = parse_json(text)?;
    let rows = match &json {
        JsonVal::Arr(a) => a.as_slice(),
        _ => return None,
    };
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let (Some(lat), Some(lon), Some(code)) = (
            finite(jnum(row, "station.lat")),
            finite(jnum(row, "station.lon")),
            jstr(row, "station.code"),
        ) else {
            continue;
        };
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            continue;
        }
        out.push(Station {
            code,
            lat_deg: lat,
            lon_deg: lon,
            mufd_mhz: finite(jnum(row, "mufd")),
            fof2_mhz: finite(jnum(row, "fof2")),
            tec_tecu: finite(jnum(row, "tec")),
            cs: finite(jnum(row, "cs")),
            time_unix: finite(jnum(row, "time")),
        });
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

fn cell(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x}"),
        None => String::new(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => URL.to_string(),
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "kc2g_stations.csv".to_string(),
    };

    let body = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => {
                eprintln!("read {} returned void: {}", path, e);
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(&url) {
            Some(b) => String::from_utf8_lossy(&b).into_owned(),
            None => {
                eprintln!("fetch {} returned void", url);
                std::process::exit(1);
            }
        },
    };

    let stations = match parse_stations(&body) {
        Some(s) => s,
        None => {
            eprintln!(
                "kc2g: {} carried no station with a measured position — the asset stays unwritten (0 honored)",
                url
            );
            std::process::exit(1);
        }
    };

    let mut f = match std::fs::File::create(&out) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create {} returned void: {}", out, e);
            std::process::exit(1);
        }
    };
    if f.write_all(b"code,lat_deg,lon_deg,mufd_mhz,fof2_mhz,tec_tecu,cs,time_unix\n")
        .is_err()
    {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    let mut records: u64 = 0;
    for s in &stations {
        let line = format!(
            "{},{},{},{},{},{},{},{}\n",
            s.code,
            s.lat_deg,
            s.lon_deg,
            cell(s.mufd_mhz),
            cell(s.fof2_mhz),
            cell(s.tec_tecu),
            cell(s.cs),
            cell(s.time_unix),
        );
        if f.write_all(line.as_bytes()).is_err() {
            eprintln!("write {} returned void", out);
            std::process::exit(1);
        }
        records += 1;
    }
    if f.flush().is_err() {
        eprintln!("flush {} returned void", out);
        std::process::exit(1);
    }
    eprintln!("kc2g harvested {} ionospheric stations -> {}", records, out);
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("upload_release for {} returned void", out);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"[
        {"mufd":14.2,"fof2":5.1,"tec":12.5,"cs":0.83,"station":{"lat":40.7,"lon":-74.0,"code":"K2ABC"},"time":1759800000.0},
        {"mufd":null,"fof2":3.3,"tec":0.0,"cs":0.5,"station":{"lat":-33.9,"lon":151.2,"code":"VK2XYZ"},"time":1759800000.0},
        {"mufd":9.9,"fof2":4.4,"tec":7.7,"cs":0.6,"station":{"lat":999.0,"lon":0.0,"code":"BAD"}}
    ]"#;

    #[test]
    fn parses_present_and_absent_fields() {
        let rows = parse_stations(FIXTURE).expect("fixture parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].code, "K2ABC");
        assert_eq!(rows[0].lat_deg, 40.7);
        assert_eq!(rows[0].lon_deg, -74.0);
        assert_eq!(rows[0].mufd_mhz, Some(14.2));
        assert_eq!(rows[0].fof2_mhz, Some(5.1));
        assert_eq!(rows[0].tec_tecu, Some(12.5));
        assert_eq!(rows[0].cs, Some(0.83));
        assert_eq!(rows[0].time_unix, Some(1759800000.0));
        assert_eq!(rows[1].code, "VK2XYZ");
        assert_eq!(rows[1].mufd_mhz, None);
        assert_eq!(rows[1].tec_tecu, Some(0.0));
    }

    #[test]
    fn skips_rows_without_position_or_code() {
        let text = r#"[{"mufd":1.0,"station":{"lat":10.0,"lon":20.0}}]"#;
        assert!(parse_stations(text).is_none());
    }
}
