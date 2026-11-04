use omegaflow::fits::FitsHeader;
use omegaflow::healpix::pix2ang_nest;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

struct Table {
    nside: i64,
    npix: usize,
    data_start: usize,
    t_width: usize,
    width: usize,
    key: String,
    unit: String,
    bad: Option<f64>,
}

fn read_table(bytes: &[u8], col: usize) -> Option<(Table, String)> {
    let (_, off) = FitsHeader::parse(bytes, 0)?;
    let (h, data_start) = FitsHeader::parse(bytes, off)?;
    let nside = h.int("NSIDE")?;
    let npix = h.int("NAXIS2")? as usize;
    let tfields = h.int("TFIELDS")? as usize;
    if col < 1 || col > tfields {
        eprintln!("--column {col} outside TFIELDS {tfields}: unread");
        return None;
    }
    if 12 * nside * nside != npix as i64 {
        eprintln!(
            "NAXIS2 {} != 12*NSIDE^2 ({}): the table shape stays unread",
            npix, nside
        );
        return None;
    }
    let Some(ordering) = h.str_unescaped("ORDERING") else {
        return None;
    };
    if !ordering.trim().eq_ignore_ascii_case("NESTED") {
        eprintln!(
            "ORDERING '{}': the probe reads NESTED only",
            ordering.trim()
        );
        return None;
    }
    let row = h.int("NAXIS1")? as usize;
    let Some(form) = h.str_unescaped(&format!("TFORM{col}")) else {
        return None;
    };
    let width = if form.contains('D') { 8 } else { 4 };
    let Some(ttype) = h.str_unescaped(&format!("TTYPE{col}")) else {
        return None;
    };
    let key = ttype.trim().to_string();
    let Some(tunit) = h.str_unescaped(&format!("TUNIT{col}")) else {
        return None;
    };
    let unit = tunit.trim().to_string();
    if row < (col - 1) * width + width {
        eprintln!("NAXIS1 {row} < column {col} end: unread");
        return None;
    }
    let bad = h.f64("BAD_DATA");
    if data_start + npix * row > bytes.len() {
        eprintln!("table exceeds the fetched bytes");
        return None;
    }
    Some((
        Table {
            nside,
            npix,
            data_start,
            t_width: width,
            width: row,
            key,
            unit,
            bad,
        },
        ttype,
    ))
}

fn col_value(bytes: &[u8], t: &Table, row: usize, col: usize) -> Option<f64> {
    let off = t.data_start + row * t.width + (col - 1) * t.t_width;
    if t.t_width == 8 {
        let raw = bytes.get(off..off + 8)?;
        Some(f64::from_be_bytes(raw.try_into().ok()?))
    } else {
        let raw = bytes.get(off..off + 4)?;
        Some(f32::from_be_bytes(raw.try_into().ok()?) as f64)
    }
}

fn is_bad(v: f64, t: &Table) -> bool {
    let Some(b) = t.bad else {
        return false;
    };
    if t.t_width == 8 {
        v == b
    } else {
        (v as f32) == (b as f32)
    }
}

fn l_b_of_pix(nside: i64, pix: i64) -> Option<(f64, f64)> {
    let (theta, phi) = pix2ang_nest(nside, pix)?;
    let l = phi.to_degrees();
    let b = 90.0 - theta.to_degrees();
    Some((l, b))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(input) = arg_value(&args, "--fits") else {
        eprintln!("usage: dust_map_audit_probe --fits <fits> --column <n> --nside <n>");
        std::process::exit(1);
    };
    let col: usize = match arg_value(&args, "--column").and_then(|v| v.parse().ok()) {
        Some(c) => c,
        None => {
            eprintln!("--column <n>: the audited column is never silent");
            std::process::exit(1);
        }
    };
    let nside_out: i64 = match arg_value(&args, "--nside").and_then(|v| v.parse().ok()) {
        Some(n) => n,
        None => {
            eprintln!("--nside <n>: the audited resolution is never silent");
            std::process::exit(1);
        }
    };

    let bytes = match std::fs::read(&input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {input} returned void: {e}");
            std::process::exit(1);
        }
    };
    let Some((table, ttype)) = read_table(&bytes, col) else {
        std::process::exit(1);
    };
    if nside_out <= 0 || nside_out > table.nside || table.nside % nside_out != 0 {
        eprintln!(
            "--nside {nside_out} is not a power-of-two divisor of NSIDE {}",
            table.nside
        );
        std::process::exit(1);
    }
    let ratio = table.nside / nside_out;
    let npix_out = (12 * nside_out * nside_out) as usize;
    let mut sum = vec![0.0f64; npix_out];
    let mut count = vec![0u64; npix_out];
    for r in 0..table.npix {
        let Some(v) = col_value(&bytes, &table, r, col) else {
            continue;
        };
        if !v.is_finite() || is_bad(v, &table) {
            continue;
        }
        let c = (r as i64 / (ratio * ratio)) as usize;
        sum[c] += v;
        count[c] += 1;
    }

    let mut n_pix = 0usize;
    let mut n_neg = 0usize;
    let mut n_gt10 = 0usize;
    let mut n_gt100 = 0usize;
    let mut n_gt300 = 0usize;
    let mut tsum = 0.0f64;
    let mut tmin = f64::INFINITY;
    let mut tmax = f64::NEG_INFINITY;
    let mut i_min = None;
    let mut i_max = None;
    let mut n_min = 0usize;
    let mut n_max = 0usize;
    for (c, cell) in (0..npix_out).zip(&sum) {
        if count[c] == 0 {
            continue;
        }
        let v = *cell / count[c] as f64;
        n_pix += 1;
        tsum += v;
        if v < tmin {
            tmin = v;
            i_min = Some(c);
            n_min = count[c] as usize;
        }
        if v > tmax {
            tmax = v;
            i_max = Some(c);
            n_max = count[c] as usize;
        }
        if v < 0.0 {
            n_neg += 1;
        }
        if v > 10.0 {
            n_gt10 += 1;
        }
        if v > 100.0 {
            n_gt100 += 1;
        }
        if v > 300.0 {
            n_gt300 += 1;
        }
    }
    if n_pix == 0 {
        eprintln!("no dust pixels — nothing to report (0 honored)");
        return;
    }
    eprintln!(
        "column {col} = '{}' ({}) {}, NSIDE {} -> {}, {} dust pixels",
        ttype.trim(),
        table.key,
        table.unit,
        table.nside,
        nside_out,
        n_pix
    );
    eprintln!(
        "mean {:.6} {}, min {:.6} {}, max {:.6} {}",
        tsum / n_pix as f64,
        table.unit,
        tmin,
        table.unit,
        tmax,
        table.unit
    );
    let show = |tag: &str, i: Option<usize>, raw: f64, nsub: usize, unit: &str| match i {
        Some(c) => match l_b_of_pix(nside_out, c as i64) {
            Some((l, b)) => eprintln!(
                "{tag} {:.6} {} at (l,b) = ({:.2}, {:.2}) deg, {} subpixels",
                raw, unit, l, b, nsub
            ),
            None => eprintln!(
                "{tag} {:.6} {} at pixel {c} (coordinate unread), {} subpixels",
                raw, unit, nsub
            ),
        },
        None => eprintln!("{tag}: absent"),
    };
    show("min", i_min, tmin, n_min, &table.unit);
    show("max", i_max, tmax, n_max, &table.unit);
    eprintln!(
        "negatives: {} pixels ({} of {} = {:.4}%), high A_V > 10: {} (>100: {}, >300: {})",
        n_neg,
        n_neg,
        n_pix,
        100.0 * n_neg as f64 / n_pix as f64,
        n_gt10,
        n_gt100,
        n_gt300
    );
    if n_neg > 0 {
        let mut most_plane = None;
        for (c, cell) in (0..npix_out).zip(&sum) {
            if count[c] == 0 {
                continue;
            }
            let v = *cell / count[c] as f64;
            if v < 0.0 {
                if let Some((l, b)) = l_b_of_pix(nside_out, c as i64) {
                    let ab = b.abs();
                    match most_plane {
                        Some((ab0, _)) if ab < ab0 => most_plane = Some((ab, l)),
                        None => most_plane = Some((ab, l)),
                        _ => {}
                    }
                }
            }
        }
        match most_plane {
            Some((ab, l)) => eprintln!(
                "most plane-ward negative at |b| = {:.2} deg (l = {:.2})",
                ab, l
            ),
            None => eprintln!("negative pixels carry no readable coordinate"),
        }
    }
}
