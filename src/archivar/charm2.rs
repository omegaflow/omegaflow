use super::*;

pub const MAGIC_CHM2: [u8; 4] = *b"CHM2";
pub const HEADER_BYTES: usize = 8;
pub const REC_BYTES: usize = 41;

const MASK_UD: u8 = 1 << 0;
const MASK_LD: u8 = 1 << 1;
const MASK_PLX: u8 = 1 << 2;

pub const COMP_UD: u32 = 1;
pub const COMP_LD: u32 = 2;
pub const COMP_MAX: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Charm2Gaia {
    pub ra: f64,
    pub dec: f64,
    pub ud_mas: Option<f64>,
    pub ld_mas: Option<f64>,
    pub plx_mas: Option<f64>,
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_UD => Some("charm2_ud_mas"),
        COMP_LD => Some("charm2_ld_mas"),
        _ => None,
    }
}

pub fn component_value(src: &Charm2Gaia, comp: u32) -> Option<f64> {
    match comp {
        COMP_UD => src.ud_mas,
        COMP_LD => src.ld_mas,
        _ => None,
    }
}

pub fn parse_crossmatch_tsv(body: &str) -> Option<Vec<Charm2Gaia>> {
    let mut lines = body.lines();
    let header = lines.next()?;
    let cols: Vec<&str> = header.split('\t').collect();
    let idx = |name: &str| -> Option<usize> { cols.iter().position(|c| c.trim() == name) };
    let i_recno = idx("crecno")?;
    let i_cra = idx("cra")?;
    let i_cdec = idx("cdec")?;
    let i_cud = idx("cud")?;
    let i_cld = idx("cld")?;
    let i_gra = idx("gra")?;
    let i_gdec = idx("gdec")?;
    let i_gplx = idx("gplx")?;
    let i_gmag = idx("gmag")?;

    let cell = |row: &[&str], i: usize| -> Option<f64> {
        row.get(i)
            .and_then(|s| s.trim().parse::<f64>().ok())
            .filter(|v| v.is_finite())
    };

    struct Cand {
        cra: f64,
        cdec: f64,
        cud: Option<f64>,
        cld: Option<f64>,
        gra: f64,
        gdec: f64,
        gplx: Option<f64>,
        gmag: Option<f64>,
    }

    let mut by_recno: HashMap<u64, Vec<Cand>> = HashMap::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let row: Vec<&str> = line.split('\t').collect();
        let Some(recno) = row.get(i_recno).and_then(|s| s.trim().parse::<u64>().ok()) else {
            continue;
        };
        let (Some(cra), Some(cdec), Some(gra), Some(gdec)) = (
            cell(&row, i_cra),
            cell(&row, i_cdec),
            cell(&row, i_gra),
            cell(&row, i_gdec),
        ) else {
            continue;
        };
        if !(0.0..=360.0).contains(&cra) || !(-90.0..=90.0).contains(&cdec) {
            continue;
        }
        by_recno.entry(recno).or_default().push(Cand {
            cra,
            cdec,
            cud: cell(&row, i_cud).filter(|v| *v > 0.0),
            cld: cell(&row, i_cld).filter(|v| *v > 0.0),
            gra,
            gdec,
            gplx: cell(&row, i_gplx).filter(|v| *v > 0.0),
            gmag: cell(&row, i_gmag),
        });
    }
    if by_recno.is_empty() {
        return None;
    }

    let mut out = Vec::new();
    for cands in by_recno.into_values() {
        let mut best: Option<(f64, f64, Cand)> = None;
        for cand in cands {
            if cand.gplx.is_none() {
                continue;
            }
            let dist = chord2(cand.cra, cand.cdec, cand.gra, cand.gdec);
            let mag = cand.gmag.unwrap_or(f64::INFINITY);
            let better = match best {
                None => true,
                Some((bd, bm, _)) => dist < bd || (dist == bd && mag < bm),
            };
            if better {
                best = Some((dist, mag, cand));
            }
        }
        if let Some((_, _, cand)) = best {
            out.push(Charm2Gaia {
                ra: cand.cra,
                dec: cand.cdec,
                ud_mas: cand.cud,
                ld_mas: cand.cld,
                plx_mas: cand.gplx,
            });
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn chord2(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let (sa1, ca1) = ra1.to_radians().sin_cos();
    let (sd1, cd1) = dec1.to_radians().sin_cos();
    let (sa2, ca2) = ra2.to_radians().sin_cos();
    let (sd2, cd2) = dec2.to_radians().sin_cos();
    let p1 = [cd1 * ca1, cd1 * sa1, sd1];
    let p2 = [cd2 * ca2, cd2 * sa2, sd2];
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    let dz = p1[2] - p2[2];
    dx * dx + dy * dy + dz * dz
}

fn f64_at(data: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(data.get(off..off + 8)?.try_into().ok()?))
}

fn encode_rec(r: &Charm2Gaia) -> [u8; REC_BYTES] {
    let mut rec = [0u8; REC_BYTES];
    rec[0..8].copy_from_slice(&r.ra.to_le_bytes());
    rec[8..16].copy_from_slice(&r.dec.to_le_bytes());
    let mut mask = 0u8;
    let cells: [(Option<f64>, u8, usize); 3] = [
        (r.ud_mas, MASK_UD, 16),
        (r.ld_mas, MASK_LD, 24),
        (r.plx_mas, MASK_PLX, 32),
    ];
    for (value, bit, off) in cells {
        match value {
            Some(v) => {
                mask |= bit;
                rec[off..off + 8].copy_from_slice(&v.to_le_bytes());
            }
            None => rec[off..off + 8].copy_from_slice(&0.0f64.to_le_bytes()),
        }
    }
    rec[40] = mask;
    rec
}

fn decode_rec(data: &[u8]) -> Option<Charm2Gaia> {
    if data.len() < REC_BYTES {
        return None;
    }
    let ra = f64_at(data, 0)?;
    let dec = f64_at(data, 8)?;
    if !ra.is_finite() || !dec.is_finite() {
        return None;
    }
    let mask = data[40];
    let cell = |off: usize, bit: u8| -> Option<f64> {
        if mask & bit == 0 {
            return None;
        }
        f64_at(data, off).filter(|v| v.is_finite())
    };
    Some(Charm2Gaia {
        ra,
        dec,
        ud_mas: cell(16, MASK_UD),
        ld_mas: cell(24, MASK_LD),
        plx_mas: cell(32, MASK_PLX),
    })
}

pub fn write_bin(records: &[Charm2Gaia]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC_CHM2);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        out.extend_from_slice(&encode_rec(r));
    }
    out
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<Charm2Gaia>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC_CHM2 {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        out.push(decode_rec(data.get(base..base + REC_BYTES)?)?);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_TSV: &str = "crecno\tcra\tcdec\tcud\tcld\tgra\tgdec\tgplx\tgmag\n\
2431\t47.53249999999999\t13.453333333333331\t3.0\t3.11\t47.53279023311\t13.45368452696\t1.3905\t7.264919\n\
1071\t44.21731666666666\t14.60959722222222\t3.64\t\t44.21732707808\t14.60959257859\t3.0241\t6.486686\n\
435\t43.952074999999994\t18.331638888888886\t\t9.5\t43.95203909304\t18.33157003634\t\t3.719293\n";

    #[test]
    fn parse_tsv_keeps_positive_parallax_matches_and_drops_absent() {
        let rows = parse_crossmatch_tsv(MEASURED_TSV).expect("the measured cross-match parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].ra, 47.53249999999999);
        assert_eq!(rows[0].ud_mas, Some(3.0));
        assert_eq!(rows[0].ld_mas, Some(3.11));
        assert_eq!(rows[0].plx_mas, Some(1.3905));
        assert_eq!(rows[1].ud_mas, Some(3.64));
        assert_eq!(rows[1].ld_mas, None);
    }

    #[test]
    fn parse_tsv_picks_nearest_and_magnitude_prior() {
        let body = "crecno\tcra\tcdec\tcud\tcld\tgra\tgdec\tgplx\tgmag\n\
7\t10.0\t5.0\t2.0\t\t10.001\t5.0\t5.0\t9.0\n\
7\t10.0\t5.0\t2.0\t\t10.0\t5.001\t5.5\t6.0\n";
        let rows = parse_crossmatch_tsv(body).expect("one recno with two candidates");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].plx_mas, Some(5.5));
        assert_eq!(rows[0].ra, 10.0);
    }

    #[test]
    fn parse_tsv_rejects_header_only_and_void() {
        assert!(parse_crossmatch_tsv("").is_none());
        assert!(
            parse_crossmatch_tsv("crecno\tcra\tcdec\tcud\tcld\tgra\tgdec\tgplx\tgmag\n").is_none()
        );
    }

    fn fixture() -> Vec<Charm2Gaia> {
        vec![
            Charm2Gaia {
                ra: 47.53249999999999,
                dec: 13.453333333333331,
                ud_mas: Some(3.0),
                ld_mas: Some(3.11),
                plx_mas: Some(1.3905),
            },
            Charm2Gaia {
                ra: 44.21731666666666,
                dec: 14.60959722222222,
                ud_mas: Some(3.64),
                ld_mas: None,
                plx_mas: Some(3.0241),
            },
        ]
    }

    #[test]
    fn bin_roundtrip_carries_rows_and_absent_masks() {
        let recs = fixture();
        let bytes = write_bin(&recs);
        let parsed = parse_bin(&bytes).expect("roundtrip parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra, recs[0].ra);
        assert_eq!(parsed[0].ud_mas, Some(3.0));
        assert_eq!(parsed[1].ld_mas, None);
        assert_eq!(parsed[1].plx_mas, Some(3.0241));
    }

    #[test]
    fn bin_rejects_bad_magic_and_truncation() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&fixture());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert!(parse_bin(&bad).is_none());
    }

    #[test]
    fn bin_rejects_nonfinite_cell_behind_a_set_mask() {
        let bytes = write_bin(&fixture());
        let mut bad = bytes;
        let off = HEADER_BYTES + 16;
        bad[off..off + 8].copy_from_slice(&f64::INFINITY.to_le_bytes());
        assert!(parse_bin(&bad).is_none());
    }

    #[test]
    fn component_names_and_values_bind_the_two_channels() {
        let src = fixture()[0];
        assert_eq!(component_name(COMP_UD), Some("charm2_ud_mas"));
        assert_eq!(component_name(COMP_LD), Some("charm2_ld_mas"));
        assert_eq!(component_name(COMP_MAX + 1), None);
        assert_eq!(component_value(&src, COMP_UD), Some(3.0));
        assert_eq!(component_value(&fixture()[1], COMP_LD), None);
    }
}
