use crate::archivar::fits::{FitsHeader, FitsTable};
use crate::archivar::odf::TnfPhaseRow;
use std::f64::consts::{PI, TAU};

pub const STATION_ALMA: &str = "AA";
pub const STATION_APEX: &str = "AP";
pub const COMP_EHT_AA: u32 = 1;
pub const COMP_EHT_AP: u32 = 2;

const JD_UNIX_EPOCH: f64 = 2440587.5;

#[derive(Clone, Debug)]
pub struct UvAntenna {
    pub name: String,
    pub xyz_m: [f64; 3],
}

#[derive(Clone, Copy, Debug)]
pub struct UvRow {
    pub t_tdb: f64,
    pub u_s: f64,
    pub v_s: f64,
    pub w_s: f64,
    pub baseline: i64,
    pub inttim_s: f64,
    pub amp: Option<f64>,
    pub phase_rad: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct UvFits {
    pub ref_freq_hz: f64,
    pub chan_bw_hz: f64,
    pub antennas: Vec<UvAntenna>,
    pub rows: Vec<UvRow>,
}

fn hdu_extname(h: &FitsHeader) -> Option<String> {
    h.str_unescaped("EXTNAME").map(|s| s.trim().to_string())
}

fn jd_to_tdb(jd: f64, lsk: &crate::archivar::lsk::LeapSeconds) -> Option<f64> {
    if !jd.is_finite() {
        return None;
    }
    let unix = (jd - JD_UNIX_EPOCH) * 86400.0;
    lsk.unix_to_tdb(unix)
}

fn mean_complex(flux: &[f64]) -> Option<(f64, f64)> {
    let mut re = 0.0;
    let mut im = 0.0;
    let mut n = 0usize;
    for pair in flux.chunks(2) {
        let (Some(&r), Some(&i)) = (pair.first(), pair.get(1)) else {
            continue;
        };
        if !r.is_finite() || !i.is_finite() {
            continue;
        }
        re += r;
        im += i;
        n += 1;
    }
    if n == 0 {
        return None;
    }
    Some((re / n as f64, im / n as f64))
}

pub fn parse_uvfits(bytes: &[u8]) -> Option<UvFits> {
    let lsk = crate::archivar::embedded_lsk()?;
    let mut off = 0usize;
    let mut antennas: Vec<UvAntenna> = Vec::new();
    let mut ref_freq_hz = 0.0;
    let mut chan_bw_hz = 0.0;
    let mut rows: Vec<UvRow> = Vec::new();
    while let Some((h, header_next)) = FitsHeader::parse(bytes, off) {
        let extname = hdu_extname(&h);
        let is_table = h.value("XTENSION") == Some("'BINTABLE'");
        match extname.as_deref() {
            Some("ARRAY_GEOMETRY") => {
                let (t, next) = FitsTable::parse(bytes, off)?;
                let (Some(name_col), Some(xyz_col)) = (t.column("ANNAME"), t.column("STABXYZ"))
                else {
                    return None;
                };
                antennas.clear();
                for r in 0..t.n_rows {
                    let name = t
                        .cell_str(bytes, r, name_col)
                        .map(|s| s.trim_end_matches([' ', '\0']).to_string())?;
                    let xyz = t.cell_array_f64(bytes, r, xyz_col)?;
                    let xyz_m = match xyz.as_slice() {
                        [x, y, z] => [*x, *y, *z],
                        _ => return None,
                    };
                    antennas.push(UvAntenna { name, xyz_m });
                }
                off = next;
            }
            Some("UV_DATA") => {
                let (t, _) = FitsTable::parse_lenient(bytes, off)?;
                ref_freq_hz = h.f64("REF_FREQ").unwrap_or(ref_freq_hz);
                chan_bw_hz = h.f64("CHAN_BW").unwrap_or(chan_bw_hz);
                let cols = (
                    t.column("UU---SIN"),
                    t.column("VV---SIN"),
                    t.column("WW---SIN"),
                    t.column("DATE"),
                    t.column("TIME"),
                    t.column("BASELINE"),
                    t.column("INTTIM"),
                    t.column("FLUX"),
                );
                let (
                    Some(uu),
                    Some(vv),
                    Some(ww),
                    Some(date),
                    Some(time),
                    Some(bl),
                    Some(inttim),
                    Some(flux),
                ) = cols
                else {
                    return None;
                };
                for r in 0..t.n_rows {
                    let (Some(d), Some(tm), Some(b), Some(it)) = (
                        t.cell_f64(bytes, r, date),
                        t.cell_f64(bytes, r, time),
                        t.cell_i64(bytes, r, bl),
                        t.cell_f64(bytes, r, inttim),
                    ) else {
                        break;
                    };
                    if !d.is_finite() || !tm.is_finite() || !it.is_finite() {
                        continue;
                    }
                    let Some(t_tdb) = jd_to_tdb(d + tm, &lsk) else {
                        continue;
                    };
                    let (Some(u_s), Some(v_s), Some(w_s)) = (
                        t.cell_f64(bytes, r, uu),
                        t.cell_f64(bytes, r, vv),
                        t.cell_f64(bytes, r, ww),
                    ) else {
                        continue;
                    };
                    let (amp, phase_rad) = match t.cell_array_f64(bytes, r, flux) {
                        Some(raw) => match mean_complex(&raw) {
                            Some((re, im)) => {
                                let mag = re.hypot(im);
                                if mag > 0.0 {
                                    (Some(mag), Some(im.atan2(re)))
                                } else {
                                    (None, None)
                                }
                            }
                            None => (None, None),
                        },
                        None => (None, None),
                    };
                    rows.push(UvRow {
                        t_tdb,
                        u_s,
                        v_s,
                        w_s,
                        baseline: b,
                        inttim_s: it,
                        amp,
                        phase_rad,
                    });
                }
                break;
            }
            _ => {
                if is_table {
                    let (_, next) = FitsTable::parse(bytes, off)?;
                    off = next;
                } else {
                    off = header_next;
                }
            }
        }
    }
    Some(UvFits {
        ref_freq_hz,
        chan_bw_hz,
        antennas,
        rows,
    })
}

pub fn baseline_rows(f: &UvFits, a: &str, b: &str) -> Vec<UvRow> {
    let mut out = Vec::new();
    for r in &f.rows {
        let a1 = r.baseline / 256;
        let a2 = r.baseline % 256;
        if a1 < 1 || a2 < 1 {
            continue;
        }
        let n1 = f.antennas.get(a1 as usize - 1).map(|x| x.name.as_str());
        let n2 = f.antennas.get(a2 as usize - 1).map(|x| x.name.as_str());
        match (n1, n2) {
            (Some(x), Some(y)) if (x == a && y == b) || (x == b && y == a) => out.push(*r),
            _ => {}
        }
    }
    out
}

fn wrap_pi(v: f64) -> f64 {
    let r = v.rem_euclid(TAU);
    if r > PI { r - TAU } else { r }
}

pub fn fringe_rate_hz(rows: &[UvRow]) -> Option<f64> {
    let mut acc = 0.0;
    let mut n = 0usize;
    for pair in rows.windows(2) {
        let (Some(p0), Some(p1)) = (pair[0].phase_rad, pair[1].phase_rad) else {
            continue;
        };
        let dt = pair[1].t_tdb - pair[0].t_tdb;
        if !(dt > 0.0) || !dt.is_finite() {
            continue;
        }
        acc += wrap_pi(p1 - p0) / dt / TAU;
        n += 1;
    }
    if n == 0 {
        return None;
    }
    Some(acc / n as f64)
}

pub fn beat_open(df_hz: f64, dt_s: f64) -> bool {
    df_hz > 0.0 && dt_s > 0.0 && df_hz * dt_s < 0.5
}

pub fn beat_rows(bytes: &[u8]) -> Option<Vec<TnfPhaseRow>> {
    let f = parse_uvfits(bytes)?;
    let pair = baseline_rows(&f, STATION_ALMA, STATION_APEX);
    if pair.is_empty() {
        return None;
    }
    let df = fringe_rate_hz(&pair)?;
    let dt_s = {
        let mut acc = 0.0;
        for r in &pair {
            acc += r.inttim_s;
        }
        acc / pair.len() as f64
    };
    if !beat_open(df, dt_s) {
        return None;
    }
    let t0 = (pair.first()?.t_tdb + pair.last()?.t_tdb) / 2.0;
    let amps: Vec<f64> = pair.iter().filter_map(|r| r.amp).collect();
    let amp = if amps.is_empty() {
        return None;
    } else {
        amps.iter().sum::<f64>() / amps.len() as f64
    };
    if !(amp > 0.0) || !amp.is_finite() {
        return None;
    }
    let phase_mean = {
        let mut re = 0.0;
        let mut im = 0.0;
        let mut n = 0usize;
        for r in &pair {
            let Some(p) = r.phase_rad else {
                continue;
            };
            re += p.cos();
            im += p.sin();
            n += 1;
        }
        if n == 0 {
            return None;
        }
        im.atan2(re)
    };
    let root = amp.sqrt();
    Some(vec![
        TnfPhaseRow {
            t: t0,
            value: root,
            phase: Some(0.0),
            freq: f.ref_freq_hz,
            bin_width: f.chan_bw_hz,
            comp: COMP_EHT_AA,
        },
        TnfPhaseRow {
            t: t0,
            value: root,
            phase: Some(-phase_mean),
            freq: f.ref_freq_hz + df,
            bin_width: f.chan_bw_hz,
            comp: COMP_EHT_AP,
        },
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_card(out: &mut Vec<u8>, card: &str) {
        let mut buf = [b' '; 80];
        let bytes = card.as_bytes();
        let n = bytes.len().min(80);
        buf[..n].copy_from_slice(&bytes[..n]);
        out.extend_from_slice(&buf);
    }

    fn push_end(out: &mut Vec<u8>) {
        push_card(out, "END");
        while out.len() % 2880 != 0 {
            out.push(b' ');
        }
    }

    fn bintable_header(
        out: &mut Vec<u8>,
        extname: &str,
        naxis1: usize,
        naxis2: usize,
        cards: &[&str],
    ) {
        push_card(out, "XTENSION= 'BINTABLE'");
        push_card(out, "BITPIX  =                    8");
        push_card(out, "NAXIS   =                    2");
        push_card(out, &format!("NAXIS1  = {naxis1:>20}"));
        push_card(out, &format!("NAXIS2  = {naxis2:>20}"));
        push_card(out, "PCOUNT  =                    0");
        push_card(out, "GCOUNT  =                    1");
        push_card(out, &format!("EXTNAME = '{extname}'"));
        for c in cards {
            push_card(out, c);
        }
        push_end(out);
    }

    fn primary_hdu(out: &mut Vec<u8>) {
        push_card(out, "SIMPLE  =                    T");
        push_card(out, "BITPIX  =                    8");
        push_card(out, "NAXIS   =                    0");
        push_card(out, "EXTEND  =                    T");
        push_end(out);
    }

    fn fixture_uvfits_with(second: &str) -> Vec<u8> {
        let mut out = Vec::new();
        primary_hdu(&mut out);
        bintable_header(
            &mut out,
            "ARRAY_GEOMETRY",
            8 + 24,
            2,
            &[
                "TFIELDS =                    2",
                "TTYPE1  = 'ANNAME  '",
                "TFORM1  = '8A      '",
                "TTYPE2  = 'STABXYZ '",
                "TFORM2  = '3D      '",
                "TBCOL1  =                    1",
                "TBCOL2  =                    9",
            ],
        );
        let mut ant_row = Vec::new();
        ant_row.extend_from_slice(b"AA\0\0\0\0\0\0");
        ant_row.extend_from_slice(&2.225e6f64.to_be_bytes());
        ant_row.extend_from_slice(&(-5.440e6f64).to_be_bytes());
        ant_row.extend_from_slice(&(-2.481e6f64).to_be_bytes());
        out.extend_from_slice(&ant_row);
        let mut ant_row = Vec::new();
        let mut name = [b'\0'; 8];
        let bytes = second.as_bytes();
        name[..bytes.len()].copy_from_slice(bytes);
        ant_row.extend_from_slice(&name);
        ant_row.extend_from_slice(&2.225e6f64.to_be_bytes());
        ant_row.extend_from_slice(&(-5.441e6f64).to_be_bytes());
        ant_row.extend_from_slice(&(-2.479e6f64).to_be_bytes());
        out.extend_from_slice(&ant_row);
        bintable_header(
            &mut out,
            "UV_DATA",
            4 + 4 + 4 + 8 + 8 + 4 + 4 + 16,
            2,
            &[
                "TFIELDS =                    8",
                "TTYPE1  = 'UU---SIN'",
                "TFORM1  = '1E      '",
                "TTYPE2  = 'VV---SIN'",
                "TFORM2  = '1E      '",
                "TTYPE3  = 'WW---SIN'",
                "TFORM3  = '1E      '",
                "TTYPE4  = 'DATE    '",
                "TFORM4  = '1D      '",
                "TTYPE5  = 'TIME    '",
                "TFORM5  = '1D      '",
                "TTYPE6  = 'BASELINE'",
                "TFORM6  = '1J      '",
                "TTYPE7  = 'INTTIM  '",
                "TFORM7  = '1E      '",
                "TTYPE8  = 'FLUX    '",
                "TFORM8  = '4E      '",
                "TBCOL1  =                    1",
                "TBCOL2  =                    5",
                "TBCOL3  =                    9",
                "TBCOL4  =                   13",
                "TBCOL5  =                   21",
                "TBCOL6  =                   29",
                "TBCOL7  =                   33",
                "TBCOL8  =                   37",
                "REF_FREQ=   2.28162796875000000E+11",
                "CHAN_BW =   5.00000000000000000E+05",
            ],
        );
        let mut row = Vec::new();
        row.extend_from_slice(&0.01f32.to_be_bytes());
        row.extend_from_slice(&(-0.02f32).to_be_bytes());
        row.extend_from_slice(&0.002f32.to_be_bytes());
        row.extend_from_slice(&2457853.5f64.to_be_bytes());
        row.extend_from_slice(&0.6243f64.to_be_bytes());
        row.extend_from_slice(&258i32.to_be_bytes());
        row.extend_from_slice(&0.4f32.to_be_bytes());
        row.extend_from_slice(&1.0f32.to_be_bytes());
        row.extend_from_slice(&0.0f32.to_be_bytes());
        row.extend_from_slice(&1.0f32.to_be_bytes());
        row.extend_from_slice(&0.0f32.to_be_bytes());
        out.extend_from_slice(&row);
        let mut row = Vec::new();
        row.extend_from_slice(&0.0101f32.to_be_bytes());
        row.extend_from_slice(&(-0.0201f32).to_be_bytes());
        row.extend_from_slice(&0.0021f32.to_be_bytes());
        row.extend_from_slice(&2457853.5f64.to_be_bytes());
        row.extend_from_slice(&(0.6243f64 + 0.4 / 86400.0).to_be_bytes());
        row.extend_from_slice(&258i32.to_be_bytes());
        row.extend_from_slice(&0.4f32.to_be_bytes());
        row.extend_from_slice(&1.0f32.to_be_bytes());
        row.extend_from_slice(&0.001f32.to_be_bytes());
        row.extend_from_slice(&1.0f32.to_be_bytes());
        row.extend_from_slice(&0.001f32.to_be_bytes());
        out.extend_from_slice(&row);
        out
    }

    #[test]
    fn parse_reads_antenna_names_and_baseline_rows() {
        let bytes = fixture_uvfits_with("AP");
        let f = parse_uvfits(&bytes).unwrap();
        assert_eq!(f.antennas.len(), 2);
        assert_eq!(f.antennas[0].name, "AA");
        assert_eq!(f.antennas[1].name, "AP");
        assert_eq!(f.ref_freq_hz, 2.28162796875e11);
        assert_eq!(f.chan_bw_hz, 5.0e5);
        assert_eq!(f.rows.len(), 2);
        assert_eq!(f.rows[0].baseline, 258);
        assert_eq!(f.rows[0].inttim_s, 0.4);
        let pair = baseline_rows(&f, STATION_ALMA, STATION_APEX);
        assert_eq!(pair.len(), 2);
    }

    #[test]
    fn phase_comes_from_the_complex_mean() {
        let bytes = fixture_uvfits_with("AP");
        let f = parse_uvfits(&bytes).unwrap();
        let p0 = f.rows[0].phase_rad.unwrap();
        assert!(p0.abs() < 1e-9);
        let p1 = f.rows[1].phase_rad.unwrap();
        assert!((p1 - 1e-3).abs() < 1e-6);
        assert!(f.rows[0].amp.unwrap() > 0.0);
    }

    #[test]
    fn fringe_rate_is_the_wrapped_phase_slope_over_2pi() {
        let bytes = fixture_uvfits_with("AP");
        let f = parse_uvfits(&bytes).unwrap();
        let pair = baseline_rows(&f, STATION_ALMA, STATION_APEX);
        let df = fringe_rate_hz(&pair).unwrap();
        let expect = 1e-3 / 0.4 / TAU;
        assert!((df - expect).abs() < expect * 1e-3);
    }

    #[test]
    fn beat_open_mirrors_the_wgsl_gate() {
        assert!(beat_open(0.001, 0.4));
        assert!(!beat_open(0.0, 0.4));
        assert!(!beat_open(0.001, 0.0));
        assert!(!beat_open(10.0, 0.4));
    }

    #[test]
    fn beat_rows_carry_the_two_station_tones_with_the_measured_df() {
        let bytes = fixture_uvfits_with("AP");
        let rows = beat_rows(&bytes).unwrap();
        assert_eq!(rows.len(), 2);
        let df = (rows[1].freq - rows[0].freq).abs();
        assert!(df > 0.0);
        assert!(df * 0.4 < 0.5);
        assert_eq!(rows[0].phase, Some(0.0));
        assert!(rows[0].bin_width == 5.0e5);
        assert_eq!(rows[0].comp, COMP_EHT_AA);
        assert_eq!(rows[1].comp, COMP_EHT_AP);
    }

    #[test]
    fn beat_rows_without_the_pair_stay_absent() {
        let solo = fixture_uvfits_with("JC");
        let f = parse_uvfits(&solo).unwrap();
        assert!(baseline_rows(&f, STATION_ALMA, STATION_APEX).is_empty());
        assert!(beat_rows(&solo).is_none());
    }
}
