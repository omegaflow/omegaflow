use omegaflow::archivar::footprint::{
    decode_rec, footprint_gate, parse_header, FootprintBand, FootprintRecord, FootprintVerdict,
    HEADER_LEN, REC_BYTES,
};
use omegaflow::cdn::{CDN_BASE, CDN_RELEASE};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const DOWNLOAD_TTL: u64 = 7200;

#[derive(Debug)]
struct FootprintBinding {
    survey: &'static str,
    asset: &'static str,
    source_host: &'static str,
    tables: &'static [&'static str],
}

const DES_DR2: FootprintBinding = FootprintBinding {
    survey: "des-dr2",
    asset: "des_dr2_coverage.fp01",
    source_host: "datalab.noirlab.edu",
    tables: &["II/357/des_dr1", "des_dr2"],
};

const FOOTPRINTS: [FootprintBinding; 1] = [DES_DR2];

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
        "w3" => Some(FootprintBand::W3),
        "w4" => Some(FootprintBand::W4),
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
        FootprintBand::W3 => "w3",
        FootprintBand::W4 => "w4",
    }
}

fn footprint_binding(name: &str) -> Option<&'static FootprintBinding> {
    FOOTPRINTS
        .iter()
        .find(|f| f.survey == name || f.tables.contains(&name))
}

fn survey_label(binding: &FootprintBinding, given: &str) -> String {
    if given == binding.survey {
        format!("survey {}", binding.survey)
    } else {
        format!("survey {} table {}", binding.survey, given)
    }
}

fn resolve_asset(args: &[String]) -> Result<(String, String), String> {
    let survey_arg = arg_value(args, "--survey");
    let asset_arg = arg_value(args, "--asset");
    if survey_arg.is_some() && asset_arg.is_some() {
        return Err("--survey and --asset: one resolution — refused".into());
    }
    if let Some(name) = survey_arg {
        let binding = footprint_binding(&name)
            .ok_or_else(|| format!("survey {name}: no registered footprint — refused"))?;
        let local = format!("data/{}/{}", binding.source_host, binding.asset);
        if std::path::Path::new(&local).exists() {
            Ok((survey_label(binding, &name), local))
        } else {
            let url = format!("{CDN_BASE}/{CDN_RELEASE}/{}", binding.asset);
            let bytes = omegaflow::archivar::fetch_raw_bytes(&url, DOWNLOAD_TTL)
                .ok_or_else(|| format!("survey {name}: {url} stays unread — refused"))?;
            if let Some(parent) = std::path::Path::new(&local).parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("mkdir {} returned void: {e}", parent.display()))?;
            }
            std::fs::write(&local, &bytes)
                .map_err(|e| format!("write {local} returned void: {e}"))?;
            eprintln!("survey {name}: {url} -> {local} ({} bytes)", bytes.len());
            Ok((survey_label(binding, &name), local))
        }
    } else if let Some(path) = asset_arg {
        Ok((format!("asset {path}"), path))
    } else {
        Err("usage: footprint_gate_probe (--survey des-dr2 | --asset <file.fp01>) --ra <deg> --dec <deg> --band <g|r|i|z|y|u|j|h|ks|w1|w2|w3|w4> — refused".into())
    }
}

fn verdict_word(verdict: FootprintVerdict) -> &'static str {
    match verdict {
        FootprintVerdict::Observed => "Observed",
        FootprintVerdict::NeverObserved => "NeverObserved",
        FootprintVerdict::BandUncovered => "BandUncovered",
        FootprintVerdict::Pending => "Pending",
    }
}

fn coverage_text(records: &[FootprintRecord]) -> String {
    if records.is_empty() {
        return "none".to_string();
    }
    records
        .iter()
        .map(|r| format!("{}={:.6}", band_name(r.band), r.frac))
        .collect::<Vec<String>>()
        .join(" ")
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
    let (label, asset) = resolve_asset(args)?;
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
        None => return Err("--band: one of g r i z y u j h ks w1 w2 w3 w4 — refused".into()),
    };

    let mut file = File::open(&asset).map_err(|e| format!("open {asset} returned void: {e}"))?;
    let mut head = [0u8; HEADER_LEN];
    file.read_exact(&mut head)
        .map_err(|e| format!("read {asset} header returned void: {e}"))?;
    let n_rows = parse_header(&head).ok_or_else(|| format!("{asset}: the header stays unread"))?;

    let Some((order, ipix)) = FootprintRecord::pixel_of(ra, dec) else {
        println!("{label} | ra {ra} dec {dec}: Pending — the direction does not place on S²");
        return Ok(());
    };

    let records = records_for_pixel(&mut file, n_rows, ipix)?;
    let verdict = footprint_gate(Some(&records), band);
    let coverage = coverage_text(&records);
    println!(
        "{label} | ra {ra} dec {dec} | order {order} ipix {ipix} | band {} -> {} | coverage {coverage}",
        band_name(band),
        verdict_word(verdict)
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
            ("w3", FootprintBand::W3),
            ("w4", FootprintBand::W4),
        ] {
            assert_eq!(band_from_letter(letter), Some(band));
            assert_eq!(band_name(band), letter);
        }
        assert_eq!(band_from_letter("x"), None);
        assert_eq!(band_from_letter(""), None);
    }

    #[test]
    fn footprint_binding_resolves_the_survey_and_its_bound_tables() {
        for name in ["des-dr2", "II/357/des_dr1", "des_dr2"] {
            let binding = footprint_binding(name).unwrap();
            assert_eq!(binding.survey, "des-dr2");
            assert_eq!(binding.asset, "des_dr2_coverage.fp01");
        }
        assert!(footprint_binding("ps1").is_none());
        assert!(footprint_binding("2mass").is_none());
    }

    #[test]
    fn coverage_text_names_what_the_pixel_holds() {
        assert_eq!(coverage_text(&[]), "none");
        let recs = [
            FootprintRecord {
                order: 12,
                band: FootprintBand::G,
                ipix: 7,
                frac: 0.75,
            },
            FootprintRecord {
                order: 12,
                band: FootprintBand::I,
                ipix: 7,
                frac: 0.5,
            },
        ];
        assert_eq!(coverage_text(&recs), "g=0.750000 i=0.500000");
    }

    #[test]
    fn verdict_words_match_the_gate_register() {
        assert_eq!(verdict_word(FootprintVerdict::Observed), "Observed");
        assert_eq!(
            verdict_word(FootprintVerdict::NeverObserved),
            "NeverObserved"
        );
        assert_eq!(
            verdict_word(FootprintVerdict::BandUncovered),
            "BandUncovered"
        );
        assert_eq!(verdict_word(FootprintVerdict::Pending), "Pending");
    }
}
