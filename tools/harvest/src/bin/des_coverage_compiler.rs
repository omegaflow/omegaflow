use omegaflow::archivar::footprint::{
    band_code, decode_rec, encode_rec, parse_header, write_header, FootprintBand, FootprintRecord,
    HEADER_LEN, REC_BYTES,
};
use omegaflow::cdn::upload_asset;
use omegaflow::zeuge::{magic_identity, FeldIdentitaet};
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::Command;

const TAP_ROOT: &str = "https://datalab.noirlab.edu/tap/sync";
const NSIDE: i64 = 4096;
const NPIX: i64 = 12 * NSIDE * NSIDE;
const RANGE_W: i64 = 1 << 20;

const BANDS: [(&str, FootprintBand); 5] = [
    ("frac_det_g", FootprintBand::G),
    ("frac_det_i", FootprintBand::I),
    ("frac_det_r", FootprintBand::R),
    ("frac_det_y", FootprintBand::Y),
    ("frac_det_z", FootprintBand::Z),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn footprint_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Footprint) => {
            eprintln!(
                "{} reads as a survey-footprint asset (sibling of the witnesses, not a witness)",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not a footprint — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn tap_query_csv(adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("600")
        .arg("--compressed")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg(TAP_ROOT)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap_query http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn column_index(header: &str, name: &str) -> Option<usize> {
    header.split(',').map(|c| c.trim()).position(|c| c == name)
}

fn parse_frac(cell: &str) -> Option<f32> {
    let t = cell.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("null") || t.eq_ignore_ascii_case("nan") {
        return None;
    }
    let v: f64 = t.parse().ok()?;
    if v.is_nan() || !v.is_finite() || !(0.0..=1.0).contains(&v) {
        return None;
    }
    Some(v as f32)
}

struct Census {
    rows: u64,
    bad_hpix: u64,
    absent_band: u64,
    invalid_frac: u64,
    dup_exact: u64,
    dup_divergent: u64,
}

fn harvest_range(lo: i64, hi: i64, census: &mut Census) -> Vec<FootprintRecord> {
    let adql = format!(
        "SELECT frac_det_g,frac_det_i,frac_det_r,frac_det_y,frac_det_z,hpix_4096 \
         FROM des_dr2.coverage WHERE hpix_4096 >= {lo} AND hpix_4096 < {hi}"
    );
    let Some(body) = tap_query_csv(&adql) else {
        return Vec::new();
    };
    let mut lines = body.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let Some(idx_hpix) = column_index(header, "hpix_4096") else {
        eprintln!("des_dr2.coverage: the hpix_4096 column is absent from the header {header}");
        return Vec::new();
    };
    let band_idx: Vec<Option<usize>> = BANDS
        .iter()
        .map(|(name, _)| column_index(header, name))
        .collect();

    let mut records: Vec<FootprintRecord> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split(',').collect();
        let Some(hpix_cell) = cells.get(idx_hpix) else {
            census.bad_hpix += 1;
            continue;
        };
        let Ok(hpix) = hpix_cell.trim().parse::<i64>() else {
            census.bad_hpix += 1;
            continue;
        };
        if !(0..NPIX).contains(&hpix) {
            census.bad_hpix += 1;
            continue;
        }
        census.rows += 1;
        let order = NSIDE.trailing_zeros() as u8;
        for (band, idx) in BANDS.iter().zip(band_idx.iter()) {
            let Some(idx) = idx else {
                continue;
            };
            let Some(cell) = cells.get(*idx) else {
                census.absent_band += 1;
                continue;
            };
            match parse_frac(cell) {
                Some(frac) => records.push(FootprintRecord {
                    order,
                    band: band.1,
                    ipix: hpix as u32,
                    frac,
                }),
                None => {
                    if cell.trim().is_empty() {
                        census.absent_band += 1;
                    } else {
                        census.invalid_frac += 1;
                    }
                }
            }
        }
    }
    collapse_max(records, census)
}

fn collapse_max(mut records: Vec<FootprintRecord>, census: &mut Census) -> Vec<FootprintRecord> {
    records.sort_by(|a, b| {
        a.ipix
            .cmp(&b.ipix)
            .then(band_code(a.band).cmp(&band_code(b.band)))
            .then(b.frac.to_bits().cmp(&a.frac.to_bits()))
    });
    let mut deduped: Vec<FootprintRecord> = Vec::with_capacity(records.len());
    for r in records {
        if let Some(last) = deduped.last() {
            if last.ipix == r.ipix && last.band == r.band {
                if last.frac == r.frac {
                    census.dup_exact += 1;
                } else {
                    census.dup_divergent += 1;
                }
                continue;
            }
        }
        deduped.push(r);
    }
    deduped
}

fn run(args: &[String]) -> Result<(), String> {
    footprint_identity(omegaflow::archivar::footprint::MAGIC)?;
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("usage: des_coverage_compiler --out <des_dr2_coverage.fp01> [--limit-ranges N] [--ci-mode] — refused".into());
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let limit_ranges: Option<i64> = arg_value(args, "--limit-ranges").and_then(|s| s.parse().ok());

    let n_ranges = (NPIX + RANGE_W - 1) / RANGE_W;
    let max_ranges = limit_ranges.map(|n| n.min(n_ranges)).unwrap_or(n_ranges);

    let mut file = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, 0);
    file.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} header returned void: {e}"))?;

    let mut census = Census {
        rows: 0,
        bad_hpix: 0,
        absent_band: 0,
        invalid_frac: 0,
        dup_exact: 0,
        dup_divergent: 0,
    };
    let mut total: u64 = 0;
    for ri in 0..max_ranges {
        let lo = ri * RANGE_W;
        let hi = (lo + RANGE_W).min(NPIX);
        let records = harvest_range(lo, hi, &mut census);
        let mut buf = Vec::with_capacity(records.len() * REC_BYTES);
        let mut rec = [0u8; REC_BYTES];
        for r in &records {
            encode_rec(&mut rec, r);
            buf.extend_from_slice(&rec);
        }
        file.write_all(&buf)
            .map_err(|e| format!("write {out_path} range {ri} returned void: {e}"))?;
        total += records.len() as u64;
        eprintln!(
            "range {ri}/{n_ranges} hpix [{lo}, {hi}): {} records",
            records.len()
        );
    }

    if total == 0 {
        return Err("no coverage record harvested — the asset stays unwritten (0 honored)".into());
    }

    file.seek(SeekFrom::Start(5))
        .map_err(|e| format!("seek {out_path} header returned void: {e}"))?;
    file.write_all(&total.to_le_bytes())
        .map_err(|e| format!("patch {out_path} header returned void: {e}"))?;
    file.flush()
        .map_err(|e| format!("flush {out_path} returned void: {e}"))?;

    let expect = HEADER_LEN as u64 + total * REC_BYTES as u64;
    let actual = std::fs::metadata(&out_path)
        .map_err(|e| format!("stat {out_path} returned void: {e}"))?
        .len();
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }

    let mut vf = std::fs::File::open(&out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut head = [0u8; HEADER_LEN];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {out_path} header returned void: {e}"))?;
    let n_rows =
        parse_header(&head).ok_or_else(|| format!("{out_path}: the header stays unread"))?;
    if n_rows != total {
        return Err(format!(
            "{out_path}: header {n_rows} rows, {total} written — the asset stays unwritten"
        ));
    }
    let last_off = HEADER_LEN as u64 + (total - 1) * REC_BYTES as u64;
    vf.seek(SeekFrom::Start(last_off))
        .map_err(|e| format!("seek {out_path} tail returned void: {e}"))?;
    let mut tail = vec![0u8; REC_BYTES];
    vf.read_exact(&mut tail)
        .map_err(|e| format!("read {out_path} tail returned void: {e}"))?;
    let last =
        decode_rec(&tail).ok_or_else(|| format!("{out_path}: the last record stays unread"))?;

    eprintln!(
        "{out_path}: {} records over {} ranges, header {} — roundtrip verified",
        total, max_ranges, n_rows
    );
    eprintln!(
        "last record: order {} band {:?} ipix {} frac {:.6}",
        last.order, last.band, last.ipix, last.frac
    );
    eprintln!(
        "census: {} source rows, {} bad-hpix, {} absent-band, {} invalid-frac, {} exact-duplicate, {} divergent-duplicate",
        census.rows,
        census.bad_hpix,
        census.absent_band,
        census.invalid_frac,
        census.dup_exact,
        census.dup_divergent
    );
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("des_coverage_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_frac_reads_zero_one_and_absent() {
        assert_eq!(parse_frac("0"), Some(0.0));
        assert_eq!(parse_frac("1"), Some(1.0));
        assert_eq!(parse_frac("0.5625"), Some(0.5625));
        assert_eq!(parse_frac(""), None);
        assert_eq!(parse_frac("null"), None);
        assert_eq!(parse_frac("nan"), None);
        assert_eq!(parse_frac("1.5"), None);
        assert_eq!(parse_frac("-0.1"), None);
    }

    #[test]
    fn column_index_finds_the_measured_header() {
        let header = "frac_det_g,frac_det_i,frac_det_r,frac_det_y,frac_det_z,hpix_4096";
        assert_eq!(column_index(header, "hpix_4096"), Some(5));
        assert_eq!(column_index(header, "frac_det_g"), Some(0));
        assert_eq!(column_index(header, "frac_det_z"), Some(4));
        assert_eq!(column_index(header, "frac_det_nope"), None);
    }

    fn census_zero() -> Census {
        Census {
            rows: 0,
            bad_hpix: 0,
            absent_band: 0,
            invalid_frac: 0,
            dup_exact: 0,
            dup_divergent: 0,
        }
    }

    fn rec(ipix: u32, band: FootprintBand, frac: f32) -> FootprintRecord {
        FootprintRecord {
            order: 12,
            band,
            ipix,
            frac,
        }
    }

    #[test]
    fn collapse_keeps_the_deepest_fraction_per_pixel_band() {
        let mut c = census_zero();
        let records = vec![
            rec(67115728, FootprintBand::G, 0.0),
            rec(67115728, FootprintBand::G, 0.0),
            rec(67115728, FootprintBand::G, 0.5625),
            rec(67115728, FootprintBand::G, 0.5625),
            rec(67115728, FootprintBand::G, 0.0),
            rec(67115728, FootprintBand::G, 0.0),
            rec(67115728, FootprintBand::G, 0.0),
            rec(67115728, FootprintBand::G, 0.0),
            rec(67115728, FootprintBand::G, 0.0),
            rec(67115728, FootprintBand::I, 1.0),
            rec(67115728, FootprintBand::I, 0.6875),
            rec(135705536, FootprintBand::Z, 1.0),
            rec(135705536, FootprintBand::Z, 0.09375),
        ];
        let out = collapse_max(records, &mut c);
        assert_eq!(out.len(), 3);
        let g = out
            .iter()
            .find(|r| r.ipix == 67115728 && r.band == FootprintBand::G)
            .unwrap();
        assert_eq!(g.frac, 0.5625);
        let i = out
            .iter()
            .find(|r| r.ipix == 67115728 && r.band == FootprintBand::I)
            .unwrap();
        assert_eq!(i.frac, 1.0);
        let z = out
            .iter()
            .find(|r| r.ipix == 135705536 && r.band == FootprintBand::Z)
            .unwrap();
        assert_eq!(z.frac, 1.0);
    }

    #[test]
    fn collapse_keeps_a_real_zero_when_every_row_is_zero() {
        let mut c = census_zero();
        let records = vec![
            rec(135704016, FootprintBand::Z, 0.0),
            rec(135704016, FootprintBand::Z, 0.0),
            rec(135704016, FootprintBand::Z, 0.0),
        ];
        let out = collapse_max(records, &mut c);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].frac, 0.0);
        assert_eq!(c.dup_exact, 2);
    }
}
