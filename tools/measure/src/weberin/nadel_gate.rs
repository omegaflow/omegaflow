use crate::weberin::borrowed_sense::{
    class_reads_natural, fink_object_witness, row_verdict, simbad_otype_known, BrokerVerdict,
    FINK_LSST_CLASS_ABSENT, FINK_LSST_SIMBAD_ABSENT,
};
use std::process::Command;

pub const UA: &str = "omegaflow-nadel-v-lsst-scan/1.0";

pub const IRSA_TAP: &str = "https://irsa.ipac.caltech.edu/TAP/sync";
pub const ALLWISE_TABLE: &str = "allsky_4band_p3as_psd";
pub const WISE_RADIUS_ARCSEC: f64 = 6.0;
pub const AGN_WEDGE_W1_W2: f64 = 0.8;
pub const WISE_AGN_CITE: &str = "Stern et al. 2012, ApJ 753, 30";
pub const ALLWISE_MAG_CODE_MIN: f64 = 90.0;

pub fn natural_excluded(class: i64, simbad: &str) -> bool {
    !(class == -1 && simbad == "Fail")
}

pub struct WiseMatch {
    pub designation: Option<String>,
    pub sep_arcsec: f64,
    pub w1: Option<f64>,
    pub w2: Option<f64>,
    pub w3: Option<f64>,
    pub w4: Option<f64>,
    pub w1_sig: Option<f64>,
    pub w3_snr: Option<f64>,
    pub w4_snr: Option<f64>,
}

pub fn fmt_mag(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.3}"),
        None => "absent".to_string(),
    }
}

pub fn csv_num(f: &[&str], k: usize) -> Option<f64> {
    let cell = f.get(k)?.trim();
    if cell.is_empty() {
        return None;
    }
    let v: f64 = cell.parse().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

pub fn mag_num(f: &[&str], k: usize) -> Option<f64> {
    let v = csv_num(f, k)?;
    if v < ALLWISE_MAG_CODE_MIN {
        Some(v)
    } else {
        None
    }
}

pub fn sep_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let r1 = ra1.to_radians();
    let d1 = dec1.to_radians();
    let r2 = ra2.to_radians();
    let d2 = dec2.to_radians();
    let a = ((d2 - d1) / 2.0).sin().powi(2) + d1.cos() * d2.cos() * ((r2 - r1) / 2.0).sin().powi(2);
    2.0 * a.sqrt().asin().to_degrees() * 3600.0
}

pub fn parse_wise_csv(body: &[u8], ra: f64, dec: f64) -> Vec<WiseMatch> {
    let Ok(text) = std::str::from_utf8(body) else {
        return Vec::new();
    };
    let mut out: Vec<WiseMatch> = Vec::new();
    let mut lines = text.lines();
    let header = match lines.next() {
        Some(h) => h,
        None => return out,
    };
    let cols: Vec<&str> = header.split(',').map(|c| c.trim()).collect();
    let index_of = |name: &str| cols.iter().position(|c| *c == name);
    let (Some(ides), Some(ira), Some(idec), Some(iw1), Some(iw2), Some(iw1s)) = (
        index_of("designation"),
        index_of("ra"),
        index_of("dec"),
        index_of("w1mpro"),
        index_of("w2mpro"),
        index_of("w1sigmpro"),
    ) else {
        return out;
    };
    let iw3 = index_of("w3mpro");
    let iw4 = index_of("w4mpro");
    let iw3s = index_of("w3snr");
    let iw4s = index_of("w4snr");
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        let (Some(sra), Some(sdec)) = (csv_num(&f, ira), csv_num(&f, idec)) else {
            continue;
        };
        let des = match f.get(ides).map(|c| c.trim()) {
            Some(d) if !d.is_empty() => Some(d.to_string()),
            _ => None,
        };
        out.push(WiseMatch {
            designation: des,
            sep_arcsec: sep_arcsec(ra, dec, sra, sdec),
            w1: mag_num(&f, iw1),
            w2: mag_num(&f, iw2),
            w3: iw3.and_then(|k| mag_num(&f, k)),
            w4: iw4.and_then(|k| mag_num(&f, k)),
            w1_sig: mag_num(&f, iw1s),
            w3_snr: iw3s.and_then(|k| csv_num(&f, k)),
            w4_snr: iw4s.and_then(|k| csv_num(&f, k)),
        });
    }
    out
}

fn irsa_tap_sync(adql: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-A")
        .arg(UA)
        .arg("-G")
        .arg(IRSA_TAP)
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg("MAXREC=10")
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

pub fn allwise_cone(ra: f64, dec: f64) -> Option<Vec<WiseMatch>> {
    let r_deg = WISE_RADIUS_ARCSEC / 3600.0;
    let adql = format!(
        "SELECT designation, ra, dec, w1mpro, w2mpro, w3mpro, w4mpro, w1sigmpro, w3snr, w4snr FROM {ALLWISE_TABLE} WHERE CONTAINS(POINT('ICRS', ra, dec), CIRCLE('ICRS', {ra:.6}, {dec:.6}, {r_deg})) = 1"
    );
    let Some((code, body)) = irsa_tap_sync(&adql) else {
        println!(
            "Nadel V (AllWISE round): ra {ra:.4} dec {dec:.4} — the IRSA TAP query did not answer (measured stall), the mid-IR witness stays pending"
        );
        return None;
    };
    if code != "200" {
        println!(
            "Nadel V (AllWISE round): ra {ra:.4} dec {dec:.4} — IRSA TAP answered HTTP {code}, the mid-IR witness stays pending"
        );
        return None;
    }
    let mut matches = parse_wise_csv(&body, ra, dec);
    matches.sort_by(|a, b| a.sep_arcsec.total_cmp(&b.sep_arcsec));
    Some(matches)
}

pub enum WiseOutcome {
    Agn,
    Field,
    Pending,
}

pub fn agn_wedge(m: &WiseMatch) -> Option<(f64, f64, f64, f64)> {
    match (m.w1, m.w2, m.w1_sig) {
        (Some(w1), Some(w2), Some(sig)) if sig > 0.0 => Some((w1, w2, sig, w1 - w2)),
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum WiseRead {
    Agn,
    FieldSource,
    NoSource,
    Pending,
}

pub fn allwise_read(dia: &str, ra: f64, dec: f64) -> WiseRead {
    let Some(matches) = allwise_cone(ra, dec) else {
        return WiseRead::Pending;
    };
    let Some(m) = matches.first() else {
        println!(
            "Nadel V (AllWISE round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — no AllWISE source within {WISE_RADIUS_ARCSEC} arcsec (0 honored) — the candidate remains pending the natural-class crossmatch"
        );
        return WiseRead::NoSource;
    };
    let des = match m.designation.as_deref() {
        Some(d) => d,
        None => "AllWISE",
    };
    let sep = m.sep_arcsec;
    if matches.len() > 1 {
        println!(
            "Nadel V (AllWISE round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — {} further AllWISE source(s) within the {WISE_RADIUS_ARCSEC} arcsec cone; the nearest (separation {sep:.1} arcsec) is the identity",
            matches.len() - 1
        );
    }
    let Some((w1, w2, sig, color)) = agn_wedge(m) else {
        let w1 = fmt_mag(m.w1);
        let w2 = fmt_mag(m.w2);
        println!(
            "Nadel V (AllWISE round): diaObject {dia} at ra {ra:.4} dec {dec:.4} matches AllWISE {des} {sep:.1} arcsec | W1 {w1} W2 {w2} — no two-band W1/W2 detection, no wedge color (0 honored) — the candidate remains"
        );
        return WiseRead::NoSource;
    };
    let w3 = fmt_mag(m.w3);
    let w4 = fmt_mag(m.w4);
    if color >= AGN_WEDGE_W1_W2 {
        println!(
            "Nadel V (AllWISE round): diaObject {dia} at ra {ra:.4} dec {dec:.4} matches AllWISE {des} {sep:.1} arcsec | W1 {w1:.3} W2 {w2:.3} (W1 sig {sig:.3}) W3 {w3} W4 {w4} | W1-W2 {color:.3} >= {AGN_WEDGE_W1_W2} — the mid-IR AGN wedge ({WISE_AGN_CITE}) — a natural AGN, excluded"
        );
        WiseRead::Agn
    } else {
        println!(
            "Nadel V (AllWISE round): diaObject {dia} at ra {ra:.4} dec {dec:.4} matches AllWISE {des} {sep:.1} arcsec | W1 {w1:.3} W2 {w2:.3} (W1 sig {sig:.3}) W3 {w3} W4 {w4} | W1-W2 {color:.3} below the {AGN_WEDGE_W1_W2} wedge — the mid-IR reads a field source, the candidate remains"
        );
        WiseRead::FieldSource
    }
}

pub fn allwise_witness(dia: &str, ra: f64, dec: f64) -> WiseOutcome {
    match allwise_read(dia, ra, dec) {
        WiseRead::Agn => WiseOutcome::Agn,
        WiseRead::FieldSource | WiseRead::NoSource => WiseOutcome::Field,
        WiseRead::Pending => WiseOutcome::Pending,
    }
}

pub const IRAS_FSC_TABLE: &str = "irasfsc";
pub const MSX_TABLE: &str = "msxc6";
pub const AKARI_IRC_TABLE: &str = "akari_irc";

pub const W3_W4_EXCESS_MAG: Option<f64> = None;
pub const F25_F12_EXCESS: Option<f64> = None;
pub const MSX_E_C_EXCESS: Option<f64> = None;
pub const AKARI_F18_F09_EXCESS: Option<f64> = None;

#[derive(Clone, Copy, PartialEq)]
pub enum FarIrRead {
    Excess,
    NoExcess,
    NoSource,
    Pending,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PairVerdict {
    Excess,
    Below,
    Unmeasured,
}

fn ratio_pair(short: Option<f64>, long: Option<f64>, threshold: Option<f64>) -> PairVerdict {
    let (Some(s), Some(l), Some(t)) = (short, long, threshold) else {
        return PairVerdict::Unmeasured;
    };
    if s.is_finite() && l.is_finite() && t.is_finite() && s > 0.0 && l > 0.0 && t > 0.0 {
        if l / s >= t {
            PairVerdict::Excess
        } else {
            PairVerdict::Below
        }
    } else {
        PairVerdict::Unmeasured
    }
}

fn mag_pair(short: Option<f64>, long: Option<f64>, threshold: Option<f64>) -> PairVerdict {
    let (Some(a), Some(b), Some(t)) = (short, long, threshold) else {
        return PairVerdict::Unmeasured;
    };
    if a.is_finite() && b.is_finite() && t.is_finite() {
        if a - b >= t {
            PairVerdict::Excess
        } else {
            PairVerdict::Below
        }
    } else {
        PairVerdict::Unmeasured
    }
}

fn far_ir_axis(pairs: [PairVerdict; 4], stall: bool, found: bool) -> FarIrRead {
    let mut excess = false;
    let mut below = false;
    for p in pairs {
        match p {
            PairVerdict::Excess => excess = true,
            PairVerdict::Below => below = true,
            PairVerdict::Unmeasured => {}
        }
    }
    if excess {
        FarIrRead::Excess
    } else if below {
        FarIrRead::NoExcess
    } else if stall || found {
        FarIrRead::Pending
    } else {
        FarIrRead::NoSource
    }
}

fn far_ir_sync(adql: &str) -> Option<Vec<u8>> {
    let (code, body) = irsa_tap_sync(adql)?;
    if code == "200" {
        Some(body)
    } else {
        None
    }
}

fn far_ir_rows(
    body: &[u8],
    ra: f64,
    dec: f64,
    want: &[&str],
) -> Option<Vec<(f64, Vec<Option<f64>>)>> {
    let text = std::str::from_utf8(body).ok()?;
    let mut lines = text.lines();
    let header = lines.next()?;
    let cols: Vec<&str> = header.split(',').map(|c| c.trim()).collect();
    let mut idx = Vec::with_capacity(want.len() + 2);
    for name in want.iter().chain(["ra", "dec"].iter()) {
        idx.push(cols.iter().position(|c| *c == *name)?);
    }
    let n = want.len();
    let mut rows = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        let (Some(sra), Some(sdec)) = (csv_num(&f, idx[n]), csv_num(&f, idx[n + 1])) else {
            continue;
        };
        let vals: Vec<Option<f64>> = idx[..n].iter().map(|&k| csv_num(&f, k)).collect();
        rows.push((sep_arcsec(ra, dec, sra, sdec), vals));
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    Some(rows)
}

fn far_ir_cone(ra: f64, dec: f64, table: &str, cols: &str) -> Option<Vec<u8>> {
    let r_deg = WISE_RADIUS_ARCSEC / 3600.0;
    let adql = format!(
        "SELECT {cols} FROM {table} WHERE CONTAINS(POINT('ICRS', ra, dec), CIRCLE('ICRS', {ra:.6}, {dec:.6}, {r_deg})) = 1"
    );
    far_ir_sync(&adql)
}

fn far_ir_wise_pair(m: &WiseMatch) -> PairVerdict {
    let w3 = match (m.w3, m.w3_snr) {
        (Some(v), Some(s)) if s > 0.0 => Some(v),
        _ => None,
    };
    let w4 = match (m.w4, m.w4_snr) {
        (Some(v), Some(s)) if s > 0.0 => Some(v),
        _ => None,
    };
    mag_pair(w3, w4, W3_W4_EXCESS_MAG)
}

fn iras_pair(ra: f64, dec: f64) -> Option<(bool, Option<f64>, Option<f64>)> {
    let body = far_ir_cone(
        ra,
        dec,
        IRAS_FSC_TABLE,
        "ra, dec, fnu_12, fnu_25, fqual_12, fqual_25",
    )?;
    let rows = far_ir_rows(
        &body,
        ra,
        dec,
        &["fnu_12", "fnu_25", "fqual_12", "fqual_25"],
    )?;
    if rows.is_empty() {
        return Some((false, None, None));
    }
    let v = &rows[0].1;
    let det = |flux: Option<f64>, qual: Option<f64>| -> Option<f64> {
        match (flux, qual) {
            (Some(f), Some(q)) if q >= 2.0 && f.is_finite() && f > 0.0 => Some(f),
            _ => None,
        }
    };
    Some((true, det(v[0], v[2]), det(v[1], v[3])))
}

fn msx_pair(ra: f64, dec: f64) -> Option<(bool, Option<f64>, Option<f64>)> {
    let body = far_ir_cone(ra, dec, MSX_TABLE, "ra, dec, c, e, q_c, q_e")?;
    let rows = far_ir_rows(&body, ra, dec, &["c", "e", "q_c", "q_e"])?;
    if rows.is_empty() {
        return Some((false, None, None));
    }
    let v = &rows[0].1;
    let det = |flux: Option<f64>, qual: Option<f64>| -> Option<f64> {
        match (flux, qual) {
            (Some(f), Some(q)) if q >= 1.0 && f.is_finite() && f > 0.0 => Some(f),
            _ => None,
        }
    };
    Some((true, det(v[0], v[2]), det(v[1], v[3])))
}

fn akari_pair(ra: f64, dec: f64) -> Option<(bool, Option<f64>, Option<f64>)> {
    let body = far_ir_cone(ra, dec, AKARI_IRC_TABLE, "ra, dec, flux09, flux18")?;
    let rows = far_ir_rows(&body, ra, dec, &["flux09", "flux18"])?;
    if rows.is_empty() {
        return Some((false, None, None));
    }
    let v = &rows[0].1;
    let det = |flux: Option<f64>| -> Option<f64> {
        match flux {
            Some(f) if f.is_finite() && f > 0.0 => Some(f),
            _ => None,
        }
    };
    Some((true, det(v[0]), det(v[1])))
}

pub fn far_ir_read(dia: &str, ra: f64, dec: f64) -> FarIrRead {
    let mut stall = false;
    let mut found = false;

    let wise = match allwise_cone(ra, dec) {
        None => {
            stall = true;
            PairVerdict::Unmeasured
        }
        Some(matches) if matches.is_empty() => PairVerdict::Unmeasured,
        Some(matches) => {
            found = true;
            far_ir_wise_pair(&matches[0])
        }
    };
    let iras = match iras_pair(ra, dec) {
        None => {
            stall = true;
            PairVerdict::Unmeasured
        }
        Some((f, s, l)) => {
            if f {
                found = true;
            }
            ratio_pair(s, l, F25_F12_EXCESS)
        }
    };
    let msx = match msx_pair(ra, dec) {
        None => {
            stall = true;
            PairVerdict::Unmeasured
        }
        Some((f, s, l)) => {
            if f {
                found = true;
            }
            ratio_pair(s, l, MSX_E_C_EXCESS)
        }
    };
    let akari = match akari_pair(ra, dec) {
        None => {
            stall = true;
            PairVerdict::Unmeasured
        }
        Some((f, s, l)) => {
            if f {
                found = true;
            }
            ratio_pair(s, l, AKARI_F18_F09_EXCESS)
        }
    };

    let read = far_ir_axis([wise, iras, msx, akari], stall, found);
    match read {
        FarIrRead::Excess => println!(
            "Nadel V (far-IR round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — the 10–60 μm axis reads excess (a measured pair above its threshold) — a natural dimmer, excluded"
        ),
        FarIrRead::NoExcess => println!(
            "Nadel V (far-IR round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — every measured far-IR pair reads below its threshold — no far-IR excess, the candidate remains"
        ),
        FarIrRead::NoSource => println!(
            "Nadel V (far-IR round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — no far-IR source within {WISE_RADIUS_ARCSEC} arcsec in any of the four instruments (0 honored) — the candidate remains"
        ),
        FarIrRead::Pending => println!(
            "Nadel V (far-IR round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — no measurable far-IR pair (bands upper-limit/absent, or the thresholds stay pending) — the far-IR color stays pending"
        ),
    }
    read
}

pub enum GateWord {
    Zwirn,
    Riss,
    PendingBorrowed,
    PendingConfirmation,
    BrokerSilent,
}

pub struct BorrowedGateVerdict {
    pub excluded: bool,
    pub word: GateWord,
    pub wise_excluded: bool,
    pub far_ir_excluded: bool,
}

pub fn borrowed_gate(
    simbad_known: bool,
    wise: Option<WiseRead>,
    broker: Option<i64>,
) -> (bool, GateWord, bool) {
    let (excluded, word, wise_excluded, _far) =
        borrowed_gate_full(simbad_known, wise, None, broker);
    (excluded, word, wise_excluded)
}

pub fn borrowed_gate_full(
    simbad_known: bool,
    wise: Option<WiseRead>,
    far_ir: Option<FarIrRead>,
    broker: Option<i64>,
) -> (bool, GateWord, bool, bool) {
    let window_natural =
        simbad_known || wise == Some(WiseRead::Agn) || far_ir == Some(FarIrRead::Excess);
    let wise_excluded = !simbad_known && wise == Some(WiseRead::Agn);
    let far_ir_excluded = !simbad_known && far_ir == Some(FarIrRead::Excess);
    match broker {
        None => (
            window_natural,
            GateWord::PendingBorrowed,
            wise_excluded,
            far_ir_excluded,
        ),
        Some(c) if c == FINK_LSST_CLASS_ABSENT => (
            window_natural,
            GateWord::BrokerSilent,
            wise_excluded,
            far_ir_excluded,
        ),
        Some(_) => {
            if window_natural {
                (true, GateWord::Zwirn, wise_excluded, far_ir_excluded)
            } else if wise == Some(WiseRead::FieldSource) {
                (false, GateWord::Riss, false, false)
            } else {
                (false, GateWord::PendingConfirmation, false, false)
            }
        }
    }
}

pub fn natural_gate_with_borrowed_sense(
    dia: &str,
    ra: f64,
    dec: f64,
    simbad: Option<&str>,
    row_class: i64,
    ask_wise: bool,
) -> BorrowedGateVerdict {
    let simbad_known = simbad_otype_known(simbad);
    let witness: Option<BrokerVerdict> = if class_reads_natural(row_class) {
        fink_object_witness(dia)
    } else {
        Some(row_verdict(dia, ra, dec, row_class))
    };
    let wise: Option<WiseRead> = if simbad_known {
        None
    } else if ask_wise {
        Some(allwise_read(dia, ra, dec))
    } else {
        None
    };
    let far_ir: Option<FarIrRead> = if simbad_known {
        None
    } else if ask_wise {
        Some(far_ir_read(dia, ra, dec))
    } else {
        None
    };
    let (excluded, word, wise_excluded, far_ir_excluded) = borrowed_gate_full(
        simbad_known,
        wise,
        far_ir,
        witness.as_ref().map(|w| w.class),
    );
    let class_text = match witness.as_ref() {
        Some(w) => w.class.to_string(),
        None => row_class.to_string(),
    };
    let simbad_word = match simbad {
        Some(t) if !FINK_LSST_SIMBAD_ABSENT.contains(&t) => format!("SIMBAD {t}"),
        _ => "SIMBAD silent".to_string(),
    };
    match &word {
        GateWord::Zwirn => {
            let window_word = if simbad_known {
                simbad_word.clone()
            } else {
                let mut parts: Vec<String> = Vec::new();
                if wise_excluded {
                    parts.push(format!("the AllWISE mid-IR AGN wedge ({WISE_AGN_CITE})"));
                }
                if far_ir_excluded {
                    parts.push("the 10–60 μm far-IR axis".to_string());
                }
                parts.join(" + ")
            };
            println!(
                "Nadel V (natural-class round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — zwirn (agreement): the broker classifier registers class {class_text} and the independent window {window_word} reads the source natural — excluded as a natural dimmer, both witnesses named"
            );
        }
        GateWord::Riss => {
            println!(
                "Nadel V (natural-class round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — riss (contradiction): the AllWISE mid-IR window reads a field source (W1-W2 below the {AGN_WEDGE_W1_W2} wedge, {WISE_AGN_CITE}) while the broker classifier registers class {class_text} — two independent voices refuse to converge, the incompatibility stays visible (never smoothed); the candidate is not excluded on the borrowed sense"
            );
        }
        GateWord::PendingBorrowed => {
            println!(
                "Nadel V (natural-class round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — pending: the borrowed sense did not answer (measured above); the natural-class decision rests on the independent windows ({simbad_word}{wise_note}{far_note})",
                wise_note = match wise {
                    Some(WiseRead::Agn) => " + AllWISE AGN wedge".to_string(),
                    Some(WiseRead::FieldSource) => " + AllWISE field source".to_string(),
                    Some(WiseRead::NoSource) | Some(WiseRead::Pending) | None => String::new(),
                },
                far_note = match far_ir {
                    Some(FarIrRead::Excess) => " + 10–60 μm far-IR excess".to_string(),
                    Some(FarIrRead::NoExcess) => " + 10–60 μm no excess".to_string(),
                    Some(FarIrRead::NoSource) | Some(FarIrRead::Pending) | None => String::new(),
                }
            );
            if excluded {
                println!(
                    "Nadel V (natural-class round): diaObject {dia} — excluded by the independent window(s), the borrowed sense absent"
                );
            }
        }
        GateWord::PendingConfirmation => {
            println!(
                "Nadel V (natural-class round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — pending: the broker classifier registers class {class_text}, but no independent window reads the source — the borrowed sense alone does not exclude (die Weberin §4, never the only witness); the candidate stays visible pending a second independent line"
            );
        }
        GateWord::BrokerSilent => {
            if excluded {
                let window_word = if simbad_known {
                    simbad_word.clone()
                } else {
                    let mut parts: Vec<String> = Vec::new();
                    if wise_excluded {
                        parts.push(format!("the AllWISE mid-IR AGN wedge ({WISE_AGN_CITE})"));
                    }
                    if far_ir_excluded {
                        parts.push("the 10–60 μm far-IR axis".to_string());
                    }
                    parts.join(" + ")
                };
                println!(
                    "Nadel V (natural-class round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — the broker classifier holds no class (absent); the independent window ({window_word}) excludes the natural dimmer"
                );
            } else {
                let tail = match wise {
                    Some(WiseRead::Agn) => {
                        "the AllWISE AGN wedge excludes the natural dimmer".to_string()
                    }
                    Some(WiseRead::FieldSource) => {
                        "the mid-IR window reads a field source".to_string()
                    }
                    Some(WiseRead::NoSource) | Some(WiseRead::Pending) | None => {
                        "unclassified — the candidate stays pending the natural-class crossmatch"
                            .to_string()
                    }
                };
                println!(
                    "Nadel V (natural-class round): diaObject {dia} at ra {ra:.4} dec {dec:.4} — the broker classifier holds no class (absent); {tail}"
                );
            }
        }
    }
    BorrowedGateVerdict {
        excluded,
        word,
        wise_excluded,
        far_ir_excluded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zwirn_when_the_independent_window_and_the_black_box_agree() {
        let (excluded, word, _) = borrowed_gate(true, None, Some(11));
        assert!(excluded);
        assert!(matches!(word, GateWord::Zwirn));
        let (excluded, word, wise_excluded) = borrowed_gate(false, Some(WiseRead::Agn), Some(21));
        assert!(excluded);
        assert!(wise_excluded);
        assert!(matches!(word, GateWord::Zwirn));
    }

    #[test]
    fn riss_when_the_black_box_contradicts_a_field_source_window() {
        let (excluded, word, _) = borrowed_gate(false, Some(WiseRead::FieldSource), Some(11));
        assert!(!excluded, "the borrowed sense alone never excludes");
        assert!(matches!(word, GateWord::Riss));
    }

    #[test]
    fn no_source_window_leaves_the_borrowed_sense_alone_pending() {
        let (excluded, word, _) = borrowed_gate(false, Some(WiseRead::NoSource), Some(11));
        assert!(!excluded);
        assert!(matches!(word, GateWord::PendingConfirmation));
        let (excluded, word, _) = borrowed_gate(false, None, Some(21));
        assert!(!excluded);
        assert!(matches!(word, GateWord::PendingConfirmation));
    }

    #[test]
    fn unreachable_borrowed_sense_is_pending_not_a_verdict() {
        let (excluded, word, _) = borrowed_gate(false, Some(WiseRead::Agn), None);
        assert!(excluded, "the independent window carries the exclusion");
        assert!(matches!(word, GateWord::PendingBorrowed));
        let (excluded, word, _) = borrowed_gate(false, None, None);
        assert!(!excluded);
        assert!(matches!(word, GateWord::PendingBorrowed));
    }

    #[test]
    fn a_black_box_without_a_class_is_absent_not_a_contradiction() {
        let (excluded, word, _) = borrowed_gate(true, None, Some(FINK_LSST_CLASS_ABSENT));
        assert!(excluded, "the SIMBAD window carries the exclusion");
        assert!(matches!(word, GateWord::BrokerSilent));
        let (excluded, word, _) = borrowed_gate(
            false,
            Some(WiseRead::FieldSource),
            Some(FINK_LSST_CLASS_ABSENT),
        );
        assert!(!excluded);
        assert!(matches!(word, GateWord::BrokerSilent));
    }

    #[test]
    fn far_ir_ratio_pair_reads_excess_above_the_threshold() {
        assert!(matches!(
            ratio_pair(Some(1.0), Some(1.5), Some(1.2)),
            PairVerdict::Excess
        ));
        assert!(matches!(
            ratio_pair(Some(1.0), Some(1.1), Some(1.2)),
            PairVerdict::Below
        ));
        assert!(matches!(
            ratio_pair(None, Some(1.5), Some(1.2)),
            PairVerdict::Unmeasured
        ));
        assert!(matches!(
            ratio_pair(Some(1.0), Some(1.5), None),
            PairVerdict::Unmeasured
        ));
    }

    #[test]
    fn far_ir_mag_pair_reads_the_red_excess() {
        assert!(matches!(
            mag_pair(Some(8.0), Some(7.0), Some(0.5)),
            PairVerdict::Excess
        ));
        assert!(matches!(
            mag_pair(Some(8.0), Some(8.2), Some(0.5)),
            PairVerdict::Below
        ));
        assert!(matches!(
            mag_pair(None, Some(7.0), Some(0.5)),
            PairVerdict::Unmeasured
        ));
    }

    #[test]
    fn far_ir_axis_is_a_disjunction_over_the_four_pairs() {
        use PairVerdict::{Below, Excess, Unmeasured};
        assert!(matches!(
            far_ir_axis([Excess, Below, Unmeasured, Unmeasured], false, true),
            FarIrRead::Excess
        ));
        assert!(matches!(
            far_ir_axis([Below, Below, Unmeasured, Unmeasured], false, true),
            FarIrRead::NoExcess
        ));
        assert!(matches!(
            far_ir_axis([Unmeasured; 4], false, true),
            FarIrRead::Pending
        ));
        assert!(matches!(
            far_ir_axis([Unmeasured; 4], true, false),
            FarIrRead::Pending
        ));
        assert!(matches!(
            far_ir_axis([Unmeasured; 4], false, false),
            FarIrRead::NoSource
        ));
    }

    #[test]
    fn far_ir_excess_excludes_as_a_second_independent_window() {
        let (excluded, word, wise_excluded, far_ir_excluded) = borrowed_gate_full(
            false,
            Some(WiseRead::FieldSource),
            Some(FarIrRead::Excess),
            Some(21),
        );
        assert!(
            excluded,
            "the far-IR axis excludes even when W1-W2 reads field"
        );
        assert!(far_ir_excluded);
        assert!(!wise_excluded);
        assert!(matches!(word, GateWord::Zwirn));
    }
}
