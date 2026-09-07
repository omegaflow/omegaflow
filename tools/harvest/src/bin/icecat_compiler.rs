use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::cdn::upload_asset;
use omegaflow::zeuge::{magic_identity, FeldIdentitaet, ZeugeArt};
use omegaflow::lsk::LeapSeconds;
use omegaflow::s2event::{
    decode_rec as s2e_decode, encode_rec as s2e_encode, parse_header as s2e_parse_header,
    write_header as s2e_write_header, S2EventRecord, REC_BYTES as S2E_REC, ROOT_NEUTRINO,
};
use omegaflow::skymap::{
    decode_rec as sky_decode, encode_rec as sky_encode, parse_header as sky_parse_header,
    write_header as sky_write_header, SkymapRecord, KIND_NEUTRINO, REC_BYTES as SKY_REC,
};
use std::collections::BTreeMap;
use std::io::{Read, Seek, SeekFrom};

const HDR: usize = 13;
const MJD_UNIX_EPOCH: f64 = 40587.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn unquoted(cell: &str) -> &str {
    let t = cell.trim();
    if let Some(rest) = t.strip_prefix('"') {
        if let Some(inner) = rest.strip_suffix('"') {
            inner
        } else {
            t
        }
    } else {
        t
    }
}

fn parse_f64(cell: &str) -> Option<f64> {
    let v: f64 = unquoted(cell).parse().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn opt_f64(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.3}"),
        None => "absent".to_string(),
    }
}

struct RowCols {
    name: Option<usize>,
    mjd: Option<usize>,
    ra: usize,
    dec: usize,
    ra_err_plus: Option<usize>,
    ra_err_minus: Option<usize>,
    dec_err_plus: Option<usize>,
    dec_err_minus: Option<usize>,
    energy: Option<usize>,
    signalness: Option<usize>,
    far: Option<usize>,
}

fn build_cols(names: &[&str]) -> Option<RowCols> {
    let idx = |name: &str| names.iter().position(|n| *n == name);
    Some(RowCols {
        name: idx("NAME"),
        mjd: idx("EVENTMJD"),
        ra: idx("RA")?,
        dec: idx("DEC")?,
        ra_err_plus: idx("RA_ERR_PLUS"),
        ra_err_minus: idx("RA_ERR_MINUS"),
        dec_err_plus: idx("DEC_ERR_PLUS"),
        dec_err_minus: idx("DEC_ERR_MINUS"),
        energy: idx("ENERGY"),
        signalness: idx("SIGNAL"),
        far: idx("FAR"),
    })
}

fn mjd_tdb_since_j2000(mjd: f64, lsk: &LeapSeconds) -> Option<f64> {
    lsk.unix_to_tdb((mjd - MJD_UNIX_EPOCH) * 86400.0)
}

fn circular_sigma_arcsec(
    dec_deg: f64,
    ra_err_plus: f64,
    ra_err_minus: f64,
    dec_err_plus: f64,
    dec_err_minus: f64,
) -> Option<f64> {
    if !(ra_err_plus >= 0.0 && ra_err_minus >= 0.0 && dec_err_plus >= 0.0 && dec_err_minus >= 0.0) {
        return None;
    }
    let cos_dec = dec_deg.to_radians().cos();
    let ra_err = (ra_err_plus + ra_err_minus) * 0.5 * cos_dec;
    let dec_err = (dec_err_plus + dec_err_minus) * 0.5;
    let sigma_deg = ((ra_err * ra_err + dec_err * dec_err) * 0.5).sqrt();
    if sigma_deg > 0.0 {
        Some(sigma_deg * 3600.0)
    } else {
        None
    }
}

struct ParsedEvent {
    name: String,
    record: S2EventRecord,
}

fn parse_row(cells: &[&str], cols: &RowCols, lsk: &LeapSeconds) -> Option<ParsedEvent> {
    let ra = cells.get(cols.ra).and_then(|c| parse_f64(c))?;
    let dec = cells.get(cols.dec).and_then(|c| parse_f64(c))?;
    if !(0.0..360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
        return None;
    }
    let name = match cols.name {
        Some(i) => match cells.get(i) {
            Some(c) => unquoted(c).to_string(),
            None => String::new(),
        },
        None => String::new(),
    };
    let epoch_tdb = match cols.mjd {
        Some(i) => match cells.get(i).and_then(|c| parse_f64(c)) {
            Some(mjd) => mjd_tdb_since_j2000(mjd, lsk),
            None => None,
        },
        None => None,
    };
    let err = |i: Option<usize>| i.and_then(|k| cells.get(k).and_then(|c| parse_f64(c)));
    let sigma_arcsec = match (
        err(cols.ra_err_plus),
        err(cols.ra_err_minus),
        err(cols.dec_err_plus),
        err(cols.dec_err_minus),
    ) {
        (Some(ra_p), Some(ra_m), Some(de_p), Some(de_m)) => {
            circular_sigma_arcsec(dec, ra_p, ra_m, de_p, de_m)
        }
        _ => None,
    };
    let energy = err(cols.energy).filter(|v| *v > 0.0);
    let signalness = err(cols.signalness).filter(|v| *v >= 0.0);
    let far = err(cols.far).filter(|v| *v > 0.0);
    let record = S2EventRecord {
        ra_deg: ra as f32,
        dec_deg: dec as f32,
        sigma_arcsec,
        epoch_tdb,
        energy,
        signalness,
        far,
        particle_root: ROOT_NEUTRINO,
    };
    Some(ParsedEvent { name, record })
}

fn verify_asset(
    path: &str,
    n_rows: usize,
    rec_bytes: usize,
    parse_header: fn(&[u8]) -> Option<u64>,
) -> Result<Vec<u8>, String> {
    let expect = HDR as u64 + n_rows as u64 * rec_bytes as u64;
    let actual = std::fs::metadata(path)
        .map_err(|e| format!("stat {path} returned void: {e}"))?
        .len();
    if actual != expect {
        return Err(format!(
            "{path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let mut head = [0u8; HDR];
    f.read_exact(&mut head)
        .map_err(|e| format!("read {path} header returned void: {e}"))?;
    let n = parse_header(&head).ok_or_else(|| format!("{path}: the header stays unread"))?;
    if n as usize != n_rows {
        return Err(format!(
            "{path}: header {n} rows, {n_rows} written — the asset stays unwritten"
        ));
    }
    let last_off = HDR as u64 + (n_rows as u64 - 1) * rec_bytes as u64;
    f.seek(SeekFrom::Start(last_off))
        .map_err(|e| format!("seek {path} returned void: {e}"))?;
    let mut tail = vec![0u8; rec_bytes];
    f.read_exact(&mut tail)
        .map_err(|e| format!("read {path} tail returned void: {e}"))?;
    Ok(tail)
}

fn witness_s2_direction_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Zeuge(ZeugeArt::S2Richtung)) => {
            eprintln!(
                "{} reads as an s2-direction witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not s2-direction — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    witness_s2_direction_identity(omegaflow::s2event::MAGIC)?;
    witness_s2_direction_identity(omegaflow::skymap::MAGIC)?;
    let input = match arg_value(args, "--input") {
        Some(v) => v,
        None => {
            return Err(
                "usage: icecat_compiler --input <tsv.csv> --out <threads.s2e1> --out-map <projection.sky1> [--ci-mode] — refused"
                    .into(),
            )
        }
    };
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => {
            return Err("--out <threads.s2e1>: the asset path is never silent — refused".into())
        }
    };
    let out_map = match arg_value(args, "--out-map") {
        Some(v) => v,
        None => {
            return Err(
                "--out-map <projection.sky1>: the asset path is never silent — refused".into(),
            )
        }
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let lsk = embedded_lsk()
        .ok_or("the embedded leap-second table stays unread — MJD epochs stay unassigned")?;
    let text =
        std::fs::read_to_string(&input).map_err(|e| format!("read {input} returned void: {e}"))?;
    let mut lines = text.lines();
    let header_line = match lines.next() {
        Some(h) => h,
        None => return Err(format!("{input}: the header stays unread")),
    };
    let header = header_line.trim().trim_start_matches('\u{feff}');
    let delimiter = if header.contains('\t') { '\t' } else { ',' };
    let names: Vec<&str> = header.split(delimiter).map(|c| c.trim()).collect();
    let cols = build_cols(&names).ok_or_else(|| {
        format!("{input}: a mandatory column (RA or DEC) is absent from the header — refused")
    })?;

    let mut events: Vec<S2EventRecord> = Vec::new();
    let mut projections: Vec<SkymapRecord> = Vec::new();
    let mut name_count: BTreeMap<String, u64> = BTreeMap::new();
    let mut last_name = String::new();
    let mut row_count = 0u64;
    let mut skipped_direction = 0u64;
    let mut unprojectable = 0u64;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        row_count += 1;
        let cells: Vec<&str> = line.split(delimiter).collect();
        let Some(parsed) = parse_row(&cells, &cols, &lsk) else {
            skipped_direction += 1;
            continue;
        };
        last_name = parsed.name.clone();
        *name_count.entry(parsed.name).or_insert(0) += 1;
        match S2EventRecord::pixel_of(parsed.record.ra_deg as f64, parsed.record.dec_deg as f64) {
            Some((order, ipix)) => projections.push(SkymapRecord {
                order,
                kind: KIND_NEUTRINO,
                ipix,
                ra_deg: parsed.record.ra_deg,
                dec_deg: parsed.record.dec_deg,
                value: 1.0,
            }),
            None => unprojectable += 1,
        }
        events.push(parsed.record);
    }

    if events.is_empty() {
        return Err("no decoded event thread — both assets stay unwritten (0 honored)".into());
    }
    let mut evb = Vec::with_capacity(HDR + events.len() * S2E_REC);
    s2e_write_header(&mut evb, events.len() as u64);
    let mut rec = [0u8; S2E_REC];
    for ev in &events {
        s2e_encode(&mut rec, ev);
        evb.extend_from_slice(&rec);
    }
    std::fs::write(&out_path, &evb).map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let ev_tail = verify_asset(&out_path, events.len(), S2E_REC, s2e_parse_header)?;

    if projections.is_empty() {
        return Err(
            "no placeable direction — the density projection stays unwritten (0 honored)".into(),
        );
    }
    let mut mvb = Vec::with_capacity(HDR + projections.len() * SKY_REC);
    sky_write_header(&mut mvb, projections.len() as u64);
    let mut mrec = [0u8; SKY_REC];
    for p in &projections {
        sky_encode(&mut mrec, p);
        mvb.extend_from_slice(&mrec);
    }
    std::fs::write(&out_map, &mvb).map_err(|e| format!("write {out_map} returned void: {e}"))?;
    let sky_tail = verify_asset(&out_map, projections.len(), SKY_REC, sky_parse_header)?;

    let last =
        s2e_decode(&ev_tail).ok_or_else(|| format!("{out_path}: the last thread stays unread"))?;
    let last_proj = sky_decode(&sky_tail)
        .ok_or_else(|| format!("{out_map}: the last projection stays unread"))?;

    eprintln!(
        "{out_path}: {} neutrino event threads from {row_count} rows of {input} — roundtrip verified",
        events.len()
    );
    eprintln!(
        "last thread {last_name}: ra {:.3} dec {:.3} sigma {} arcsec epoch {} s energy {} TeV signalness {} FAR {} yr^-1",
        last.ra_deg,
        last.dec_deg,
        opt_f64(last.sigma_arcsec),
        opt_f64(last.epoch_tdb),
        opt_f64(last.energy),
        opt_f64(last.signalness),
        opt_f64(last.far)
    );
    let uniq = name_count.len();
    if uniq != events.len() {
        eprintln!(
            "event names seen {uniq}, threads {} — duplicate names held",
            events.len()
        );
    }
    eprintln!(
        "{out_map}: {} density projections (value 1.0 per thread, kind neutrino) — roundtrip verified",
        projections.len()
    );
    eprintln!(
        "last projection: ra {:.3} dec {:.3} value {:.1} kind {}",
        last_proj.ra_deg, last_proj.dec_deg, last_proj.value, last_proj.kind
    );
    eprintln!(
        "skipped: {skipped_direction} row(s) without a measured direction, {unprojectable} unprojectable"
    );
    if ci_mode {
        if !upload_asset(&out_path) {
            return Err(format!("{out_path}: CDN upload returned void"));
        }
        if !upload_asset(&out_map) {
            return Err(format!("{out_map}: CDN upload returned void"));
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("icecat_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::lsk::parse as parse_lsk;

    const FIXTURE_LSK: &str = "KPL/LSK\n\
[2]       DELTA_AT  =  TAI - UTC\n\
[3]       DELTA_ET  =  ET - (TAI - DELTA_AT)\n\
\\begindata\n\n\
DELTET/DELTA_T_A       =   32.184\n\
DELTET/K               =    1.657D-3\n\
DELTET/M               = (  6.239996D0   1.99096871D-7 )\n\n\
DELTET/DELTA_AT        = ( 10,   @1972-JAN-1,\n 37,   @2017-JAN-1 )\n";

    const HEADER: &str = "NAME\tRUNID\tEVENTID\tSTART\tEVENTMJD\tI3TYPE\tRA\tDEC\tRA_ERR_PLUS\tRA_ERR_MINUS\tDEC_ERR_PLUS\tDEC_ERR_MINUS\tENERGY\tFAR\tSIGNAL\tCASCADE_SCR\tSKIMMING_SCR\tSTART_SCR\tSTOP_SCR\tTHRGOING_SCR\tCR_VETO\tOTHER_I3TYPES";

    #[test]
    fn unquote_strips_quotes_and_parse_refuses_blank_or_word() {
        assert_eq!(unquoted("\"IC110514A\""), "IC110514A");
        assert_eq!(parse_f64(" 187.0 "), Some(187.0));
        assert_eq!(parse_f64(""), None);
        assert_eq!(parse_f64("n/a"), None);
        assert_eq!(parse_f64("\"FALSE\""), None);
    }

    #[test]
    fn build_cols_maps_the_measured_header() {
        let names: Vec<&str> = HEADER.split('\t').collect();
        let cols = build_cols(&names).unwrap();
        assert_eq!(cols.ra, 6);
        assert_eq!(cols.dec, 7);
        assert_eq!(cols.mjd, Some(4));
        assert_eq!(cols.energy, Some(12));
        assert_eq!(cols.far, Some(13));
        assert_eq!(cols.signalness, Some(14));
        assert_eq!(cols.name, Some(0));
        assert_eq!(cols.ra_err_plus, Some(8));
        assert_eq!(cols.dec_err_minus, Some(11));
    }

    #[test]
    fn circular_sigma_is_the_circular_equivalent_1sigma_in_arcsec() {
        let s = circular_sigma_arcsec(60.0, 1.0, 1.0, 1.0, 1.0).unwrap();
        let expect = (0.625f64).sqrt() * 3600.0;
        assert!((s - expect).abs() < 1e-6);
        assert_eq!(circular_sigma_arcsec(0.0, 1.0, 1.0, -1.0, 1.0), None);
        assert_eq!(circular_sigma_arcsec(0.0, 0.0, 0.0, 0.0, 0.0), None);
    }

    #[test]
    fn mjd_tdb_since_j2000_carries_the_leap_table() {
        let lsk = parse_lsk(FIXTURE_LSK).unwrap();
        let tdb = mjd_tdb_since_j2000(61246.72222222222, &lsk).unwrap();
        assert!((tdb - 838272069.184).abs() < 1e-2);
    }

    #[test]
    fn parse_row_carries_a_measured_event_thread() {
        let lsk = parse_lsk(FIXTURE_LSK).unwrap();
        let names: Vec<&str> = HEADER.split('\t').collect();
        let cols = build_cols(&names).unwrap();
        let row = "\"IC110514A\"\t118178\t17334444\t2011-05-14 01:32:22.654109\t55695.06415108922\t\"gfu-gold\"\t138.47\t-1.94\t6.68\t3.78\t0.97\t1.12\t187.0\t1.3\t0.508\t3.46e-07\t4.48e-07\t0.0811\t4.65e-05\t0.919\t\"FALSE\"\t\"None\"";
        let cells: Vec<&str> = row.split('\t').collect();
        let parsed = parse_row(&cells, &cols, &lsk).unwrap();
        assert_eq!(parsed.name, "IC110514A");
        assert_eq!(parsed.record.ra_deg, 138.47);
        assert_eq!(parsed.record.dec_deg, -1.94);
        assert_eq!(parsed.record.energy, Some(187.0));
        assert_eq!(parsed.record.signalness, Some(0.508));
        assert_eq!(parsed.record.far, Some(1.3));
        assert_eq!(parsed.record.particle_root, ROOT_NEUTRINO);
        let sigma = parsed.record.sigma_arcsec.unwrap();
        assert!((10000.0..20000.0).contains(&sigma));
        let epoch = parsed.record.epoch_tdb.unwrap();
        assert!((epoch - 358608784.84).abs() < 1.0);
    }
}
