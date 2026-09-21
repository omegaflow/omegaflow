use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ia2Source {
    pub ra: f64,
    pub dec: f64,
    pub psfmag_r: Option<f64>,
    pub psfmag_g: Option<f64>,
}

fn cell_f64(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("nan") {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn parse_sources(body: &str) -> Option<Vec<Ia2Source>> {
    let mut lines = body.lines().filter(|l| !l.trim().is_empty());
    let header_line = lines.find(|l| l.contains(',') || l.contains('\0'))?;
    let headers = split_csv_line(header_line);
    let col = |name: &str| headers.iter().position(|h| h == name);
    let (Some(ira), Some(idec), Some(ir), Some(ig)) =
        (col("ra"), col("dec_"), col("psfMag_r"), col("psfMag_g"))
    else {
        return None;
    };
    let mut out = Vec::new();
    for line in lines {
        if !line.contains(',') && !line.contains('\0') {
            continue;
        }
        let fields = split_csv_line(line);
        let at = |i: usize| fields.get(i).and_then(|s| cell_f64(s));
        let (Some(ra), Some(dec)) = (at(ira), at(idec)) else { continue };
        if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
            continue;
        }
        let psfmag_r = at(ir);
        let psfmag_g = at(ig);
        if psfmag_r.is_none() && psfmag_g.is_none() {
            continue;
        }
        out.push(Ia2Source {
            ra,
            dec,
            psfmag_r,
            psfmag_g,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn to_skymap(sources: &[Ia2Source]) -> Vec<crate::skymap::SkymapRecord> {
    let mut out = Vec::new();
    for s in sources {
        let Some(val) = s.psfmag_r.or(s.psfmag_g) else { continue };
        let Some((order, ipix)) = crate::skymap::SkymapRecord::pixel_of(s.ra, s.dec) else {
            continue;
        };
        out.push(crate::skymap::SkymapRecord {
            order,
            kind: crate::skymap::KIND_GENERIC,
            ipix,
            ra_deg: s.ra as f32,
            dec_deg: s.dec as f32,
            value: val as f32,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_NUL_CSV: &str = "ra\0dec_\0psfMag_r\0psfMag_g\n0.00460846\01.17697726\020.99679946899414\020.968534469604492\n0.00784693\0-9.78113858\019.324390411376953\019.5224609375\n";

    #[test]
    fn parse_sources_carries_measured_nul_csv_rows() {
        let rows = parse_sources(MEASURED_NUL_CSV).expect("the measured NUL-CSV parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].ra, 0.00460846);
        assert_eq!(rows[0].dec, 1.17697726);
        assert_eq!(rows[0].psfmag_r, Some(20.99679946899414));
        assert_eq!(rows[1].psfmag_g, Some(19.5224609375));
    }

    #[test]
    fn parse_sources_rejects_header_only_and_void() {
        assert!(parse_sources("ra\0dec_\0psfMag_r\0psfMag_g\n").is_none());
        assert!(parse_sources("").is_none());
        assert!(parse_sources("ra\0dec_\0psfMag_r\n0.0\00.0\015.0\n").is_none());
        assert!(parse_sources("ra\0dec_\0psfMag_r\0psfMag_g\n0.0\0-95.0\015.0\0NaN\n").is_none());
    }

    #[test]
    fn to_skymap_prefers_r_band_then_g_band() {
        let rows = parse_sources(MEASURED_NUL_CSV).unwrap();
        let skymap = to_skymap(&rows);
        assert_eq!(skymap.len(), 2);
        assert_eq!(skymap[0].value, 20.99679946899414);
        assert_eq!(skymap[0].kind, crate::skymap::KIND_GENERIC);
        assert_eq!(skymap[0].dec_deg, 1.17697726);
    }
}
