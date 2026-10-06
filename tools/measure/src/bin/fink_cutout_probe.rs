use omegaflow::archivar::fits::{FitsHeader, FitsImage};
use std::env;
use std::process::Command;

const CUTOUT_ENDPOINT: &str = "https://api.lsst.fink-portal.org/api/v1/cutouts";
const SAMPLE_KINDS: [&str; 3] = ["Science", "Template", "Difference"];
const APERTURE_RADIUS_PX: f64 = 5.0;
const SUPPORTED_BITPIX: [i32; 6] = [8, 16, 32, 64, -32, -64];

struct Args {
    dia_source_id: Option<String>,
    kind: String,
    file: Option<String>,
}

fn parse_args(argv: &[String]) -> Option<Args> {
    let mut dia_source_id = None;
    let mut kind = String::from("Science");
    let mut file = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--dia-source-id" => {
                dia_source_id = argv.get(i + 1).cloned();
                i += 1;
            }
            "--kind" => {
                let value = argv.get(i + 1)?;
                if !SAMPLE_KINDS.contains(&value.as_str()) {
                    return None;
                }
                kind = value.clone();
                i += 1;
            }
            "--file" => {
                file = argv.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    if file.is_none() && dia_source_id.is_none() {
        return None;
    }
    Some(Args {
        dia_source_id,
        kind,
        file,
    })
}

fn fetch_cutout(id: &str, kind: &str) -> Option<(u16, Vec<u8>)> {
    let body = format!(
        "{{\"diaSourceId\":\"{}\",\"kind\":\"{}\",\"output-format\":\"FITS\"}}",
        id, kind
    );
    let out = Command::new("curl")
        .args([
            "-sS",
            "-m",
            "60",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "--data-binary",
            &body,
            "-w",
            "\n%{http_code}",
            CUTOUT_ENDPOINT,
        ])
        .output()
        .ok()?;
    let stdout = out.stdout;
    let nl = stdout.iter().rposition(|&b| b == b'\n')?;
    let code: u16 = std::str::from_utf8(&stdout[nl + 1..])
        .ok()?
        .trim()
        .parse()
        .ok()?;
    Some((code, stdout[..nl].to_vec()))
}

fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    let n = values.len();
    if n % 2 == 1 {
        Some(values[n / 2])
    } else {
        Some((values[n / 2 - 1] + values[n / 2]) / 2.0)
    }
}

fn flat_pixels(buf: &[u8], img: &FitsImage) -> Vec<Option<f64>> {
    let mut flat = Vec::with_capacity(img.dims[0] * img.dims[1] * img.dims[2]);
    for z in 0..img.dims[2] {
        for y in 0..img.dims[1] {
            for x in 0..img.dims[0] {
                flat.push(img.value_f64(buf, [x, y, z]));
            }
        }
    }
    flat
}

fn card_value(v: Option<i64>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => String::from("absent"),
    }
}

struct Aperture {
    count: usize,
    sum: f64,
}

fn aperture_sum(flat: &[Option<f64>], dims: [usize; 3], radius: f64) -> Aperture {
    let cx = dims[0] as f64 / 2.0;
    let cy = dims[1] as f64 / 2.0;
    let nx = dims[0];
    let ny = dims[1];
    let mut count = 0usize;
    let mut sum = 0.0f64;
    for z in 0..dims[2] {
        for y in 0..ny {
            for x in 0..nx {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                if dx * dx + dy * dy <= radius * radius {
                    let idx = (z * ny + y) * nx + x;
                    if let Some(Some(v)) = flat.get(idx).copied() {
                        if v.is_finite() {
                            count += 1;
                            sum += v;
                        }
                    }
                }
            }
        }
    }
    Aperture { count, sum }
}

fn report(buf: &[u8]) {
    let Some((header, _)) = FitsHeader::parse(buf, 0) else {
        println!("fits: header absent");
        return;
    };
    let naxis1 = header.int("NAXIS1");
    let naxis2 = header.int("NAXIS2");
    let bitpix = header.int("BITPIX");
    println!(
        "image: NAXIS1={} NAXIS2={} BITPIX={}",
        card_value(naxis1),
        card_value(naxis2),
        card_value(bitpix)
    );
    let Some(bp) = bitpix else {
        println!("pixels: absent (BITPIX card carries no value)");
        return;
    };
    if !SUPPORTED_BITPIX.contains(&(bp as i32)) {
        println!("pixels: absent (BITPIX {bp} has no integer/float arm)");
        return;
    }
    let Some((img, _)) = FitsImage::parse(buf, 0) else {
        println!("pixels: absent (image data does not resolve against the buffer)");
        return;
    };
    let flat = flat_pixels(buf, &img);
    let mut finite: Vec<f64> = flat
        .iter()
        .filter_map(|v| v.filter(|p| p.is_finite()))
        .collect();
    let background = median(&mut finite);
    match background {
        Some(med) => println!("background: median={med:.6} over {} pixels", finite.len()),
        None => println!("background: absent (image carries no finite pixels)"),
    }
    let ap = aperture_sum(&flat, img.dims, APERTURE_RADIUS_PX);
    let cx = img.dims[0] as f64 / 2.0;
    let cy = img.dims[1] as f64 / 2.0;
    println!(
        "aperture: center=({cx:.1},{cy:.1}) radius={APERTURE_RADIUS_PX} pixels={} sum={:.6}",
        ap.count, ap.sum
    );
    if let Some(med) = background {
        let local = med * ap.count as f64;
        println!(
            "aperture_signal: sum_minus_local_background={:.6} (local_background={:.6})",
            ap.sum - local,
            local
        );
    }
}

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    let Some(args) = parse_args(&argv) else {
        println!(
            "fink_cutout_probe: usage: --dia-source-id <id> [--kind Science|Template|Difference] [--file <path>]"
        );
        std::process::exit(2);
    };

    let buf = if let Some(path) = &args.file {
        match std::fs::read(path) {
            Ok(b) => {
                println!("source: local file {path} ({} bytes)", b.len());
                b
            }
            Err(_) => {
                println!("source: local file absent: {path}");
                std::process::exit(2);
            }
        }
    } else {
        let Some(id) = args.dia_source_id.as_deref() else {
            println!("source: dia-source-id absent and no --file given");
            std::process::exit(2);
        };
        println!(
            "source: {CUTOUT_ENDPOINT} diaSourceId={id} kind={}",
            args.kind
        );
        let Some((code, body)) = fetch_cutout(id, &args.kind) else {
            println!("source: curl returned no response");
            std::process::exit(2);
        };
        println!("http: {code} bytes={}", body.len());
        if code != 200 {
            println!("source: http {code} carries no FITS image");
            std::process::exit(2);
        }
        body
    };

    report(&buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(key: &str, value: &str) -> [u8; 80] {
        let mut c = [b' '; 80];
        let k = key.as_bytes();
        let kl = k.len().min(8);
        c[..kl].copy_from_slice(&k[..kl]);
        c[8] = b'=';
        let v = value.as_bytes();
        let vl = v.len().min(20);
        c[10..10 + vl].copy_from_slice(&v[..vl]);
        c
    }

    fn synthetic_fits() -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&card("SIMPLE", "T"));
        header.extend_from_slice(&card("BITPIX", "-32"));
        header.extend_from_slice(&card("NAXIS", "2"));
        header.extend_from_slice(&card("NAXIS1", "8"));
        header.extend_from_slice(&card("NAXIS2", "4"));
        header.extend_from_slice(&card("END", ""));
        while !header.len().is_multiple_of(2880) {
            header.extend_from_slice(&[b' '; 80]);
        }
        let mut buf = header;
        for i in 0..32 {
            buf.extend_from_slice(&(i as f32).to_be_bytes());
        }
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn synthetic_fixture_header_dims_parse() {
        let buf = synthetic_fits();
        let (h, _) = FitsHeader::parse(&buf, 0).unwrap();
        assert_eq!(h.int("NAXIS1"), Some(8));
        assert_eq!(h.int("NAXIS2"), Some(4));
        let (img, _) = FitsImage::parse(&buf, 0).unwrap();
        assert_eq!(img.dims[0], 8);
        assert_eq!(img.dims[1], 4);
        let flat = flat_pixels(&buf, &img);
        let ap = aperture_sum(&flat, img.dims, APERTURE_RADIUS_PX);
        assert!(ap.count > 0);
        assert!(ap.count <= 32);
    }

    #[test]
    fn sample_cutout_header_dims_parse_if_present() {
        let Ok(buf) = std::fs::read("/tmp/opencode/fink_cut.fits") else {
            return;
        };
        let (h, _) = FitsHeader::parse(&buf, 0).unwrap();
        let n1 = h.int("NAXIS1").unwrap();
        let n2 = h.int("NAXIS2").unwrap();
        assert!(n1 > 0);
        assert!(n2 > 0);
    }
}
