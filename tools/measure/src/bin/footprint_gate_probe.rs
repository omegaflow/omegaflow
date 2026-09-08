use omegaflow::archivar::footprint::{
    decode_rec, footprint_gate, parse_header, FootprintBand, FootprintRecord, FootprintVerdict,
    HEADER_LEN, REC_BYTES,
};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn band_from_letter(s: &str) -> Option<FootprintBand> {
    match s.trim().to_ascii_lowercase().as_str() {
        "u" => Some(FootprintBand::U),
        "g" => Some(FootprintBand::G),
        "r" => Some(FootprintBand::R),
        "i" => Some(FootprintBand::I),
        "z" => Some(FootprintBand::Z),
        "y" => Some(FootprintBand::Y),
        "j" => Some(FootprintBand::J),
        "h" => Some(FootprintBand::H),
        "ks" => Some(FootprintBand::Ks),
        "w1" => Some(FootprintBand::W1),
        "w2" => Some(FootprintBand::W2),
        _ => None,
    }
}

fn band_name(band: FootprintBand) -> &'static str {
    match band {
        FootprintBand::U => "u",
        FootprintBand::G => "g",
        FootprintBand::R => "r",
        FootprintBand::I => "i",
        FootprintBand::Z => "z",
        FootprintBand::Y => "y",
        FootprintBand::J => "j",
        FootprintBand::H => "h",
        FootprintBand::Ks => "ks",
        FootprintBand::W1 => "w1",
        FootprintBand::W2 => "w2",
    }
}

fn read_ipix(file: &mut File, index: u64) -> Option<u32> {
    let off = HEADER_LEN as u64 + index * REC_BYTES as u64 + 4;
    file.seek(SeekFrom::Start(off)).ok()?;
    let mut b = [0u8; 4];
    file.read_exact(&mut b).ok()?;
    Some(u32::from_le_bytes(b))
}

fn records_for_pixel(
    file: &mut File,
    n_rows: u64,
    ipix: u32,
) -> Result<Vec<FootprintRecord>, String> {
    let mut l = 0u64;
    let mut r = n_rows;
    while l < r {
        let m = l + (r - l) / 2;
        let v = read_ipix(file, m).ok_or("read ipix returned void")?;
        if v < ipix {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let mut out = Vec::new();
    let mut i = l;
    while i < n_rows {
        let off = HEADER_LEN as u64 + i * REC_BYTES as u64;
        file.seek(SeekFrom::Start(off))
            .map_err(|e| format!("seek {off} returned void: {e}"))?;
        let mut b = [0u8; REC_BYTES];
        file.read_exact(&mut b)
            .map_err(|e| format!("read record {i} returned void: {e}"))?;
        let rec = decode_rec(&b).ok_or(format!("record {i} stays unread"))?;
        if rec.ipix != ipix {
            break;
        }
        out.push(rec);
        i += 1;
    }
    Ok(out)
}

fn run(args: &[String]) -> Result<(), String> {
    let asset = match arg_value(args, "--asset") {
        Some(v) => v,
        None => {
            return Err("usage: footprint_gate_probe --asset <footprint.fp01> --ra <deg> --dec <deg> --band <g|r|i|z|y|u|j|h|ks|w1|w2> — refused".into())
        }
    };
    let ra = match arg_value(args, "--ra").and_then(|s| s.parse::<f64>().ok()) {
        Some(v) if v.is_finite() => v,
        _ => return Err("--ra <deg>: a finite degree value — refused".into()),
    };
    let dec = match arg_value(args, "--dec").and_then(|s| s.parse::<f64>().ok()) {
        Some(v) if v.is_finite() => v,
        _ => return Err("--dec <deg>: a finite degree value — refused".into()),
    };
    let band = match arg_value(args, "--band").and_then(|s| band_from_letter(&s)) {
        Some(b) => b,
        None => return Err("--band: one of g r i z y u j h ks w1 w2 — refused".into()),
    };

    let mut file = File::open(&asset).map_err(|e| format!("open {asset} returned void: {e}"))?;
    let mut head = [0u8; HEADER_LEN];
    file.read_exact(&mut head)
        .map_err(|e| format!("read {asset} header returned void: {e}"))?;
    let n_rows = parse_header(&head).ok_or_else(|| format!("{asset}: the header stays unread"))?;

    let Some((order, ipix)) = FootprintRecord::pixel_of(ra, dec) else {
        println!("ra {ra} dec {dec}: pending — the direction does not place on S²");
        return Ok(());
    };

    let records = records_for_pixel(&mut file, n_rows, ipix)?;
    let verdict = footprint_gate(Some(&records), band);

    let bands_held: Vec<String> = records
        .iter()
        .map(|r| format!("{}={:.6}", band_name(r.band), r.frac))
        .collect();
    let held = if bands_held.is_empty() {
        "none".to_string()
    } else {
        bands_held.join(" ")
    };
    let word = match verdict {
        FootprintVerdict::Observed => "observed",
        FootprintVerdict::NeverObserved => "never-observed",
        FootprintVerdict::BandUncovered => "band-uncovered",
        FootprintVerdict::Pending => "pending",
    };
    println!(
        "ra {ra} dec {dec} -> order {order} ipix {ipix} | band {} -> {word} | held: {held}",
        band_name(band)
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("footprint_gate_probe: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_letters_map_every_registry_band() {
        for (letter, band) in [
            ("u", FootprintBand::U),
            ("g", FootprintBand::G),
            ("r", FootprintBand::R),
            ("i", FootprintBand::I),
            ("z", FootprintBand::Z),
            ("y", FootprintBand::Y),
            ("j", FootprintBand::J),
            ("h", FootprintBand::H),
            ("ks", FootprintBand::Ks),
            ("w1", FootprintBand::W1),
            ("w2", FootprintBand::W2),
        ] {
            assert_eq!(band_from_letter(letter), Some(band));
            assert_eq!(band_name(band), letter);
        }
        assert_eq!(band_from_letter("x"), None);
        assert_eq!(band_from_letter(""), None);
    }
}
