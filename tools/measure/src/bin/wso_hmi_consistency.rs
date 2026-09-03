use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::hmi_polar::parse_bin as parse_hmi;
use omegaflow::spectral::civil_from_days;
use omegaflow::wso_polar::parse_bin as parse_wso;

const WSO_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/wso_polar.bin";
const HMI_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/hmi_polar.bin";
const STRIDE_DAYS: i64 = 30;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load_wso(args: &[String]) -> Option<Vec<(i64, f64, f64, f64)>> {
    match arg_value(args, "--wso-bin") {
        Some(p) => parse_wso(&std::fs::read(p).ok()?),
        None => parse_wso(&fetch_raw_bytes(WSO_CDN, 3600)?),
    }
}

fn load_hmi(args: &[String]) -> Option<Vec<(i64, f64, f64, f64)>> {
    match arg_value(args, "--hmi-bin") {
        Some(p) => parse_hmi(&std::fs::read(p).ok()?),
        None => parse_hmi(&std::fs::read("hmi_polar.bin").ok()?)
            .or_else(|| parse_hmi(&fetch_raw_bytes(HMI_CDN, 3600)?)),
    }
}

fn span(days: i64) -> String {
    match civil_from_days(days) {
        Some((y, m, d)) => format!("{y}-{m:02}-{d:02}"),
        None => format!("day {days}"),
    }
}

struct Fit {
    r: f64,
    slope: f64,
    intercept: f64,
    slope0: f64,
}

fn fit(xs: &[f64], ys: &[f64]) -> Option<Fit> {
    let n = xs.len();
    if n < 8 || n != ys.len() {
        return None;
    }
    let mx = xs.iter().sum::<f64>() / n as f64;
    let my = ys.iter().sum::<f64>() / n as f64;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    let mut sxy = 0.0;
    let mut sxx0 = 0.0;
    let mut sxy0 = 0.0;
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        sxx += (x - mx) * (x - mx);
        syy += (y - my) * (y - my);
        sxy += (x - mx) * (y - my);
        sxx0 += x * x;
        sxy0 += x * y;
    }
    if sxx <= 0.0 || syy <= 0.0 || sxx0 <= 0.0 {
        return None;
    }
    Some(Fit {
        r: sxy / (sxx * syy).sqrt(),
        slope: sxy / sxx,
        intercept: my - (sxy / sxx) * mx,
        slope0: sxy0 / sxx0,
    })
}

fn report(label: &str, xs: &[f64], ys: &[f64]) {
    match fit(xs, ys) {
        Some(f) => {
            println!(
                "{label}: n={} r={:.4} OLS HMI={:+.4}·WSO{:+0.4} Faktor0={:.4}",
                xs.len(),
                f.r,
                f.slope,
                f.intercept,
                f.slope0
            );
        }
        None => println!("{label}: too little coverage — no statement (0 honored)"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(wso) = load_wso(&args) else {
        eprintln!("wso_polar.bin absent or carries no WSP1 contract");
        return;
    };
    let Some(hmi) = load_hmi(&args) else {
        eprintln!("hmi_polar.bin absent or carries no HMP1 contract");
        return;
    };

    let d0 = wso
        .first()
        .map(|&(d, ..)| d)
        .unwrap()
        .min(hmi.first().map(|&(d, ..)| d).unwrap());
    let d1 = wso
        .last()
        .map(|&(d, ..)| d)
        .unwrap()
        .max(hmi.last().map(|&(d, ..)| d).unwrap());
    let bins = ((d1 - d0) / STRIDE_DAYS + 1) as usize;

    let mut w_bin: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(d, _, _, a) in &wso {
        let idx = ((d - d0) / STRIDE_DAYS) as usize;
        if idx < bins {
            w_bin[idx].push(a);
        }
    }
    let mut w_n: Vec<Vec<f64>> = vec![Vec::new(); bins];
    let mut w_s: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(d, n, s, _) in &wso {
        let idx = ((d - d0) / STRIDE_DAYS) as usize;
        if idx < bins {
            w_n[idx].push(n);
            w_s[idx].push(s);
        }
    }
    let mut h_n: Vec<Vec<f64>> = vec![Vec::new(); bins];
    let mut h_s: Vec<Vec<f64>> = vec![Vec::new(); bins];
    let mut h_a: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(d, n, s, a) in &hmi {
        let idx = ((d - d0) / STRIDE_DAYS) as usize;
        if idx < bins {
            h_n[idx].push(n);
            h_s[idx].push(s);
            h_a[idx].push(a);
        }
    }

    let mut wn: Vec<f64> = Vec::new();
    let mut ws: Vec<f64> = Vec::new();
    let mut wa: Vec<f64> = Vec::new();
    let mut hn: Vec<f64> = Vec::new();
    let mut hs: Vec<f64> = Vec::new();
    let mut ha: Vec<f64> = Vec::new();
    for i in 0..bins {
        let mean = |v: &Vec<f64>| {
            if v.is_empty() {
                None
            } else {
                Some(v.iter().sum::<f64>() / v.len() as f64)
            }
        };
        if let (Some(wm), Some(hm)) = (mean(&w_bin[i]), mean(&h_a[i])) {
            wa.push(wm);
            ha.push(hm);
        }
        if let (Some(wm), Some(hm)) = (mean(&w_n[i]), mean(&h_n[i])) {
            wn.push(wm);
            hn.push(hm);
        }
        if let (Some(wm), Some(hm)) = (mean(&w_s[i]), mean(&h_s[i])) {
            ws.push(wm);
            hs.push(hm);
        }
    }

    println!(
        "WSO {} Records {}..{}, HMI {} Records {}..{}",
        wso.len(),
        span(wso.first().unwrap().0),
        span(wso.last().unwrap().0),
        hmi.len(),
        span(hmi.first().unwrap().0),
        span(hmi.last().unwrap().0)
    );
    println!(
        "Overlap bins (30 d): North {}, South {}, Avg {}",
        wn.len(),
        ws.len(),
        wa.len()
    );
    println!();
    println!("Pearson r + factor WSO(x) ↔ HMI(y), monthly:");
    report("North", &wn, &hn);
    report("South", &ws, &hs);
    report("Avg ", &wa, &ha);
}
