use omegaflow::bayestar::{
    decode_rec, decode_row, encode_rec, parse_header, table_of, write_header, Be19Table, MapHeader,
    REC_BYTES,
};
use omegaflow::inflate::gunzip_stream;
use std::collections::BTreeMap;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_table_header(path: &str) -> Result<Be19Table, String> {
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let mut head = vec![0u8; 65536];
    let n = f
        .read(&mut head)
        .map_err(|e| format!("read {path} header returned void: {e}"))?;
    head.truncate(n);
    match table_of(&head) {
        Some(t) => Ok(t),
        None => {
            let (h1, off1) = match omegaflow::fits::FitsHeader::parse(&head, 0) {
                Some(x) => x,
                None => {
                    return Err(format!("{path}: the primary header stayed unread"));
                }
            };
            let h1_n = h1.int("NAXIS");
            let h2 = match omegaflow::fits::FitsHeader::parse(&head, off1) {
                Some((h2, _)) => h2,
                None => {
                    return Err(format!(
                        "{path}: the table header stayed unread (off {off1})"
                    ));
                }
            };
            let tfields = h2.int("TFIELDS");
            let mut names = String::new();
            if let Some(tf) = tfields {
                for i in 1..=tf {
                    names.push_str(&format!(
                        " col{i}={:?}/{:?}",
                        h2.str_unescaped(&format!("TTYPE{i}")),
                        h2.str_unescaped(&format!("TFORM{i}"))
                    ));
                }
            }
            Err(format!(
                "{path}: the Bayestar19 BINTABLE header stayed unread (naxis {h1_n:?}, xtension {:?}, tfields {:?}{names})",
                h2.value("XTENSION"), tfields,
            ))
        }
    }
}

fn decode_to(path_gz: &str, out_path: &str) -> Result<u64, String> {
    let src =
        std::fs::File::open(path_gz).map_err(|e| format!("open {path_gz} returned void: {e}"))?;
    let dst = std::fs::File::create(out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut w = BufWriter::with_capacity(1 << 20, dst);
    let mut io_err: Option<String> = None;
    let total = gunzip_stream(src, |chunk| {
        if io_err.is_none() {
            if let Err(e) = w.write_all(chunk) {
                io_err = Some(format!("write {out_path} returned void: {e}"));
            }
        }
    })
    .map_err(|e| format!("{path_gz}: {e}"))?;
    if let Some(e) = io_err {
        return Err(e);
    }
    let _ = w.flush();
    let _ = w.into_inner();
    Ok(total)
}

struct Census {
    decoded: u64,
    invalid: u64,
    not_converged: u64,
    conv_byte: BTreeMap<u8, u64>,
    orders: BTreeMap<u8, u64>,
    dm_nan: u64,
    bf_nonfinite: u64,
    bf_negative: u64,
    bf_decreasing: u64,
    dup_pixel: u64,
    seen: std::collections::HashSet<u64>,
    last_total_sum: f64,
    last_total_n: u64,
}

fn row_total(r: &omegaflow::bayestar::Be19Row) -> f64 {
    r.best_fit[119] as f64
}

fn run(args: &[String]) -> Result<(), String> {
    let Some(input) = arg_value(args, "--input") else {
        return Err(
            "usage: bayestar_compiler --input <bayestar2019.fits[.gz]> --out <map.be19> \
             [--decompressed <uncompressed-fits>] — refused"
                .into(),
        );
    };
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("--out <map.be19>: the asset path is never silent — refused".into());
    };

    let is_gz = input.ends_with(".gz");
    let fits_path = if is_gz {
        let named = input.trim_end_matches(".gz");
        match arg_value(args, "--decompressed") {
            Some(d) if std::path::Path::new(&d).exists() => d,
            Some(d) => return Err(format!("--decompressed {d} does not exist — refused")),
            None => {
                if std::path::Path::new(named).exists() {
                    named.to_string()
                } else {
                    eprintln!(
                        "{input}: no decompressed sibling — the compiler decompresses to {named}"
                    );
                    let total = decode_to(&input, named)
                        .map_err(|e| format!("{e} — the asset stays unwritten"))?;
                    eprintln!("{input}: decompressed {total} bytes to {named}");
                    named.to_string()
                }
            }
        }
    } else {
        input.clone()
    };

    let tbl = read_table_header(&fits_path)?;
    eprintln!(
        "table: {} rows of {} bytes at data offset {}",
        tbl.n_rows, tbl.row_bytes, tbl.data_start
    );

    let mut f = std::fs::File::open(&fits_path)
        .map_err(|e| format!("open {fits_path} returned void: {e}"))?;
    f.seek(SeekFrom::Start(tbl.data_start))
        .map_err(|e| format!("seek {fits_path} returned void: {e}"))?;
    let mut rdr = BufReader::with_capacity(1 << 20, f);

    let header = MapHeader {
        n_rows: tbl.n_rows,
        mu0: omegaflow::bayestar::BE19_MU0,
        dmu: omegaflow::bayestar::BE19_DMU,
        bins: omegaflow::bayestar::BE19_BINS as u16,
    };
    let mut out = BufWriter::with_capacity(
        1 << 20,
        std::fs::File::create(&out_path)
            .map_err(|e| format!("create {out_path} returned void: {e}"))?,
    );
    let mut hbuf = Vec::with_capacity(omegaflow::bayestar::ASSET_HEADER_LEN);
    write_header(&mut hbuf, &header);
    out.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;

    let mut census = Census {
        decoded: 0,
        invalid: 0,
        not_converged: 0,
        conv_byte: BTreeMap::new(),
        orders: BTreeMap::new(),
        dm_nan: 0,
        bf_nonfinite: 0,
        bf_negative: 0,
        bf_decreasing: 0,
        dup_pixel: 0,
        seen: std::collections::HashSet::with_capacity(1 << 22),
        last_total_sum: 0.0,
        last_total_n: 0,
    };
    let mut row = vec![0u8; tbl.row_bytes];
    let mut rec = [0u8; REC_BYTES];
    while census.decoded + census.invalid < tbl.n_rows {
        match rdr.read_exact(&mut row) {
            Ok(()) => {}
            Err(e) => return Err(format!("{fits_path}: {e} — the asset stays unwritten")),
        }
        match decode_row(&row, &tbl) {
            Some(r) => {
                let order = r.nside.trailing_zeros() as u8;
                *census.orders.entry(order).or_insert(0) += 1;
                *census.conv_byte.entry(row[tbl.converged.off]).or_insert(0) += 1;
                if !r.converged {
                    census.not_converged += 1;
                }
                let key = ((order as u64) << 32) | (r.ipix as u64);
                if !census.seen.insert(key) {
                    census.dup_pixel += 1;
                }
                if !(r.dm_min.is_finite() && r.dm_max.is_finite()) {
                    census.dm_nan += 1;
                }
                let mut finite = true;
                let mut nonneg = true;
                let mut prev = 0.0f32;
                let mut decreasing = 0u64;
                for (j, &b) in r.best_fit.iter().enumerate() {
                    if !b.is_finite() {
                        finite = false;
                    }
                    if b < -1e-6 {
                        nonneg = false;
                    }
                    if j > 0 && b < prev - 1e-4 {
                        decreasing += 1;
                    }
                    prev = b;
                }
                if !finite {
                    census.bf_nonfinite += 1;
                }
                if !nonneg {
                    census.bf_negative += 1;
                }
                if decreasing > 0 {
                    census.bf_decreasing += 1;
                }
                encode_rec(&mut rec, &r);
                out.write_all(&rec)
                    .map_err(|e| format!("write {out_path} returned void: {e}"))?;
                census.decoded += 1;
                if r.converged {
                    let t = row_total(&r);
                    if t.is_finite() && t >= 0.0 {
                        census.last_total_sum += t;
                        census.last_total_n += 1;
                    }
                }
            }
            None => census.invalid += 1,
        }
    }
    let _ = out.flush();
    let _ = out.into_inner();

    if census.decoded == 0 {
        return Err("no decoded row — the asset stays unwritten (0 honored)".into());
    }

    eprintln!(
        "rows decoded {}, invalid {}, not converged {}, duplicate pixels {}",
        census.decoded, census.invalid, census.not_converged, census.dup_pixel
    );
    let mut oline = String::from("nside histogram:");
    for (o, n) in &census.orders {
        oline.push_str(&format!(" nside={} n={}", 1u64 << o, n));
    }
    eprintln!("{oline}");
    let mut cline = String::from("converged column byte histogram:");
    for (b, n) in &census.conv_byte {
        cline.push_str(&format!(" byte={} n={}", b, n));
    }
    eprintln!("{cline}");
    eprintln!(
        "rows with non-finite DM range {}, non-finite best-fit {}, negative best-fit {}, non-monotone best-fit {}",
        census.dm_nan, census.bf_nonfinite, census.bf_negative, census.bf_decreasing
    );
    let mean_tot = if census.last_total_n > 0 {
        census.last_total_sum / census.last_total_n as f64
    } else {
        0.0
    };
    eprintln!(
        "converged rows {} carry mean total-column E(B-V) {:.4} (SFD-like)",
        census.last_total_n, mean_tot
    );

    let expect_bytes =
        omegaflow::bayestar::ASSET_HEADER_LEN as u64 + census.decoded as u64 * REC_BYTES as u64;
    let actual = std::fs::metadata(&out_path)
        .map_err(|e| format!("stat {out_path} returned void: {e}"))?
        .len();
    if actual != expect_bytes {
        return Err(format!(
            "{out_path}: {} bytes written, {} expected — the asset stays unwritten",
            actual, expect_bytes
        ));
    }
    let mut vf = std::fs::File::open(&out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut head = [0u8; omegaflow::bayestar::ASSET_HEADER_LEN];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {out_path} header returned void: {e}"))?;
    let h = parse_header(&head).ok_or_else(|| format!("{out_path}: the header stays unread"))?;
    eprintln!(
        "{out_path}: {} rows, mu0 {} dmu {} bins {} — roundtrip header verified",
        h.n_rows, h.mu0, h.dmu, h.bins
    );
    let last_off = omegaflow::bayestar::ASSET_HEADER_LEN as u64
        + (census.decoded as u64 - 1) * REC_BYTES as u64;
    vf.seek(SeekFrom::Start(last_off))
        .map_err(|e| format!("seek {out_path} returned void: {e}"))?;
    let mut tail = vec![0u8; REC_BYTES];
    vf.read_exact(&mut tail)
        .map_err(|e| format!("read {out_path} tail returned void: {e}"))?;
    decode_rec(&tail).ok_or_else(|| format!("{out_path}: the last record stays unread"))?;
    eprintln!("{out_path}: last record reads back");
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("bayestar_compiler: {msg}");
        std::process::exit(1);
    }
}
