use omegaflow::spectral::{parse_xp_spectra_bin, write_xp_spectra_bin, XpStar};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn merge_bins(parts: &[Vec<u8>]) -> Option<(f64, Vec<XpStar>)> {
    let mut stars: Vec<XpStar> = Vec::new();
    let mut epoch: Option<f64> = None;
    for bytes in parts {
        let (e, mut s) = parse_xp_spectra_bin(bytes)?;
        match epoch {
            None => epoch = Some(e),
            Some(prev) if prev == e => {}
            Some(_) => return None,
        }
        stars.append(&mut s);
    }
    Some((epoch?, stars))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!("--out absent — the merged catalog path is undeclared");
            std::process::exit(1);
        }
    };
    let mut parts: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--out" {
            i += 2;
            continue;
        }
        if args[i].starts_with("--") {
            i += 1;
            continue;
        }
        match std::fs::read(&args[i]) {
            Ok(bytes) => parts.push(bytes),
            Err(_) => {
                eprintln!("read {} returned void", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if parts.is_empty() {
        eprintln!("no part paths given — the merge stays unwritten");
        std::process::exit(1);
    }
    let (epoch, stars) = match merge_bins(&parts) {
        Some(v) => v,
        None => {
            eprintln!("a part returned void or an epoch differs — the merge stays unwritten");
            std::process::exit(1);
        }
    };
    if stars.is_empty() {
        eprintln!("no stars across the parts — the merge stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_xp_spectra_bin(epoch, &stars);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_xp_spectra_bin(&bytes) {
        Some((e, parsed))
            if e == epoch
                && parsed.len() == stars.len()
                && write_xp_spectra_bin(e, &parsed) == bytes =>
        {
            eprintln!(
                "{}: {} stars from {} parts, epoch_tdb {} — the merge parses byte-identical ({} B)",
                out,
                parsed.len(),
                parts.len(),
                e,
                bytes.len()
            );
        }
        _ => {
            eprintln!("{}: roundtrip parse void — the merge stays unverified", out);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn star(id: u64) -> XpStar {
        XpStar {
            source_id: id,
            ra: 1.0,
            dec: 2.0,
            plx_mas: 10.0,
            bins: vec![(400.0, 10.0, 1.0), (410.0, 10.0, 2.0)],
        }
    }

    #[test]
    fn merge_bins_concatenates_across_parts() {
        let a = write_xp_spectra_bin(42.0, &[star(1), star(2)]);
        let b = write_xp_spectra_bin(42.0, &[star(3)]);
        let (epoch, stars) = merge_bins(&[a, b]).expect("merge holds");
        assert_eq!(epoch, 42.0);
        assert_eq!(stars.len(), 3);
        assert_eq!(stars[0].source_id, 1);
        assert_eq!(stars[2].source_id, 3);
    }

    #[test]
    fn merge_bins_refuses_a_mixed_epoch() {
        let a = write_xp_spectra_bin(42.0, &[star(1)]);
        let b = write_xp_spectra_bin(43.0, &[star(2)]);
        assert!(merge_bins(&[a, b]).is_none());
    }

    #[test]
    fn merge_bins_roundtrips() {
        let a = write_xp_spectra_bin(7.0, &[star(5)]);
        let b = write_xp_spectra_bin(7.0, &[star(6)]);
        let (epoch, stars) = merge_bins(&[a, b]).expect("merge holds");
        let bytes = write_xp_spectra_bin(epoch, &stars);
        let (e, parsed) = parse_xp_spectra_bin(&bytes).expect("roundtrip holds");
        assert_eq!(e, 7.0);
        assert_eq!(parsed.len(), 2);
    }
}
