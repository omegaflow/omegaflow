use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::cdn::upload_asset;
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
use std::process::Command;

const HDR: usize = 13;
const CONE_URL: &str = "https://vo.km3net.de/ant20_01/nu/cone/form";
const CONE_BODY: &str = "hscs_pos=180%200&hscs_sr=10800&_FORMAT=CSV&MAXREC=250000&submit=Go&__nevow_form__=genForm";
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

fn fetch_cone() -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-g")
        .arg("--retry")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg("240")
        .arg("--connect-timeout")
        .arg("32")
        .arg("-X")
        .arg("POST")
        .arg("--data")
        .arg(CONE_BODY)
        .arg(CONE_URL)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "curl {} http {}: {}",
            CONE_URL,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

struct RowCols {
    mjd: Option<usize>,
    beta: Option<usize>,
    ra: usize,
    dec: usize,
    id: Option<usize>,
}

fn build_cols(names: &[&str]) -> Option<RowCols> {
    let idx = |name: &str| names.iter().position(|n| *n == name);
    Some(RowCols {
        mjd: idx("MJD"),
        beta: idx("Beta"),
        ra: idx("RA")?,
        dec: idx("Decl")?,
        id: idx("ID"),
    })
}

fn mjd_tdb_since_j2000(mjd: f64, lsk: &LeapSeconds) -> Option<f64> {
    lsk.unix_to_tdb((mjd - MJD_UNIX_EPOCH) * 86400.0)
}

struct ParsedEvent {
    id: String,
    record: S2EventRecord,
}

fn parse_row(cells: &[&str], cols: &RowCols, lsk: &LeapSeconds) -> Option<ParsedEvent> {
    let ra_raw = cells.get(cols.ra).and_then(|c| parse_f64(c))?;
    let dec = cells.get(cols.dec).and_then(|c| parse_f64(c))?;
    if !(-90.0..=90.0).contains(&dec) {
        return None;
    }
    let ra = ra_raw.rem_euclid(360.0);
    let id = match cols.id {
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
    let sigma_arcsec = match cols.beta {
        Some(i) => match cells.get(i).and_then(|c| parse_f64(c)) {
            Some(beta_deg) => {
                let s = beta_deg * 3600.0;
                if s.is_finite() && s > 0.0 {
                    Some(s)
                } else {
                    None
                }
            }
            None => None,
        },
        None => None,
    };
    let record = S2EventRecord {
        ra_deg: ra as f32,
        dec_deg: dec as f32,
        sigma_arcsec,
        epoch_tdb,
        energy: None,
        signalness: None,
        far: None,
        particle_root: ROOT_NEUTRINO,
    };
    Some(ParsedEvent { id, record })
}

struct Census {
    rows: u64,
    skipped_direction: u64,
    unprojectable: u64,
    missing_epoch: u64,
    missing_sigma: u64,
}

fn parse_csv(
    text: &str,
    lsk: &LeapSeconds,
) -> Result<
    (
        Vec<S2EventRecord>,
        Vec<SkymapRecord>,
        BTreeMap<String, u64>,
        Census,
    ),
    String,
> {
    let mut lines = text.lines();
    let header_line = lines
        .next()
        .ok_or("the cone CSV header stays unread")?;
    let header = header_line.trim().trim_start_matches('\u{feff}');
    let delimiter = if header.contains('\t') { '\t' } else { ',' };
    let names: Vec<&str> = header.split(delimiter).map(|c| c.trim()).collect();
    let cols = build_cols(&names).ok_or(
        "a mandatory column (RA or Decl) is absent from the cone CSV header — refused",
    )?;
    let mut events: Vec<S2EventRecord> = Vec::new();
    let mut projections: Vec<SkymapRecord> = Vec::new();
    let mut name_count: BTreeMap<String, u64> = BTreeMap::new();
    let mut census = Census {
        rows: 0,
        skipped_direction: 0,
        unprojectable: 0,
        missing_epoch: 0,
        missing_sigma: 0,
    };
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        census.rows += 1;
        let cells: Vec<&str> = line.split(delimiter).collect();
        let Some(parsed) = parse_row(&cells, &cols, lsk) else {
            census.skipped_direction += 1;
            continue;
        };
        if parsed.record.epoch_tdb.is_none() {
            census.missing_epoch += 1;
        }
        if parsed.record.sigma_arcsec.is_none() {
            census.missing_sigma += 1;
        }
        *name_count.entry(parsed.id).or_insert(0) += 1;
        match S2EventRecord::pixel_of(
            parsed.record.ra_deg as f64,
            parsed.record.dec_deg as f64,
        ) {
            Some((order, ipix)) => projections.push(SkymapRecord {
                order,
                kind: KIND_NEUTRINO,
                ipix,
                ra_deg: parsed.record.ra_deg,
                dec_deg: parsed.record.dec_deg,
                value: 1.0,
            }),
            None => census.unprojectable += 1,
        }
        events.push(parsed.record);
    }
    Ok((events, projections, name_count, census))
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

fn run(args: &[String]) -> Result<(), String> {
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
        fetch_cone().ok_or_else(|| format!("{CONE_URL}: the cone CSV stays unfetched"))?;
    let (events, projections, name_count, census) = parse_csv(&text, &lsk)?;

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
        "{out_path}: {} neutrino event threads from {} cone rows of {CONE_URL} — roundtrip verified",
        events.len(),
        census.rows
    );
    eprintln!(
        "last thread: ra {:.3} dec {:.3} sigma {} arcsec epoch {} s — energy absent, the source carries none",
        last.ra_deg,
        last.dec_deg,
        opt_f64(last.sigma_arcsec),
        opt_f64(last.epoch_tdb)
    );
    let uniq = name_count.len();
    if uniq != events.len() {
        eprintln!(
            "event ids seen {uniq}, threads {} — duplicate ids held",
            events.len()
        );
    }
    eprintln!(
        "{out_map}: {} neutrino density projections (value 1.0 per thread) — roundtrip verified",
        projections.len()
    );
    eprintln!(
        "last projection: ra {:.3} dec {:.3} value {:.1} kind {}",
        last_proj.ra_deg, last_proj.dec_deg, last_proj.value, last_proj.kind
    );
    eprintln!(
        "skipped: {skipped_direction} row(s) without a measured direction, {unprojectable} unprojectable; rows missing epoch {missing_epoch}, missing sigma {missing_sigma}",
        skipped_direction = census.skipped_direction,
        unprojectable = census.unprojectable,
        missing_epoch = census.missing_epoch,
        missing_sigma = census.missing_sigma
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
        eprintln!("antares_vo_compiler: {msg}");
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

    const HEADER: &str = "_r,MJD,Beta,Nhit,RA,Decl,ID";

    const ROWS: &str = "\
0.3605546206306824,58082.913,0.4,34,180.3,-0.2,ANT8677\n\
133.0,55232.0985,0.7,41,360.0,-47.0,ANT2373\n\
179.41690676958692,55926.0166,0.5,24,359.5,0.3,ANT4536";

    #[test]
    fn build_cols_maps_the_measured_vo_header() {
        let names: Vec<&str> = HEADER.split(',').collect();
        let cols = build_cols(&names).unwrap();
        assert_eq!(cols.ra, 4);
        assert_eq!(cols.dec, 5);
        assert_eq!(cols.mjd, Some(1));
        assert_eq!(cols.beta, Some(2));
        assert_eq!(cols.id, Some(6));
    }

    #[test]
    fn parse_row_carries_a_measured_neutrino_thread() {
        let lsk = parse_lsk(FIXTURE_LSK).unwrap();
        let names: Vec<&str> = HEADER.split(',').collect();
        let cols = build_cols(&names).unwrap();
        let row = "0.3605546206306824,58082.913,0.4,34,180.3,-0.2,ANT8677";
        let cells: Vec<&str> = row.split(',').collect();
        let parsed = parse_row(&cells, &cols, &lsk).unwrap();
        assert_eq!(parsed.id, "ANT8677");
        assert_eq!(parsed.record.ra_deg, 180.3);
        assert_eq!(parsed.record.dec_deg, -0.2);
        assert_eq!(parsed.record.sigma_arcsec, Some(1440.0));
        assert_eq!(parsed.record.energy, None);
        assert_eq!(parsed.record.signalness, None);
        assert_eq!(parsed.record.far, None);
        assert_eq!(parsed.record.particle_root, ROOT_NEUTRINO);
        let epoch = parsed.record.epoch_tdb.unwrap();
        assert!((epoch - 564918952.384).abs() < 1.0);
    }

    #[test]
    fn absent_beta_and_mjd_stay_absent() {
        let lsk = parse_lsk(FIXTURE_LSK).unwrap();
        let names: Vec<&str> = HEADER.split(',').collect();
        let cols = build_cols(&names).unwrap();
        let row = "1.0,58000.0,,20,10.0,20.0,ANTX";
        let cells: Vec<&str> = row.split(',').collect();
        let parsed = parse_row(&cells, &cols, &lsk).unwrap();
        assert_eq!(parsed.record.sigma_arcsec, None);
        let row2 = "1.0,,0.5,20,10.0,20.0,ANTY";
        let cells2: Vec<&str> = row2.split(',').collect();
        let parsed2 = parse_row(&cells2, &cols, &lsk).unwrap();
        assert_eq!(parsed2.record.epoch_tdb, None);
    }

    #[test]
    fn parse_csv_yields_threads_projection_and_wrap_normalized() {
        let lsk = parse_lsk(FIXTURE_LSK).unwrap();
        let text = format!("{HEADER}\n{ROWS}");
        let (events, projections, name_count, census) = parse_csv(&text, &lsk).unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(projections.len(), 3);
        assert_eq!(name_count.len(), 3);
        assert_eq!(census.rows, 3);
        assert_eq!(census.skipped_direction, 0);
        assert_eq!(census.unprojectable, 0);
        assert_eq!(events[1].ra_deg, 0.0);
        assert_eq!(events[1].dec_deg, -47.0);
        assert_eq!(events[2].ra_deg, 359.5);
        assert_eq!(events[2].dec_deg, 0.3);
    }

    #[test]
    fn encoded_threads_roundtrip_with_sigma_and_epoch_present() {
        let lsk = parse_lsk(FIXTURE_LSK).unwrap();
        let text = format!("{HEADER}\n{ROWS}");
        let (events, _, _, _) = parse_csv(&text, &lsk).unwrap();
        let mut rec = [0u8; S2E_REC];
        s2e_encode(&mut rec, &events[0]);
        assert_eq!(rec[0], ROOT_NEUTRINO);
        assert_eq!(rec[1], 0x03);
        let back = s2e_decode(&rec).unwrap();
        assert_eq!(back.ra_deg, events[0].ra_deg);
        assert_eq!(back.dec_deg, events[0].dec_deg);
        assert_eq!(back.sigma_arcsec, events[0].sigma_arcsec);
        assert_eq!(back.epoch_tdb, events[0].epoch_tdb);
        assert_eq!(back.energy, None);
        assert_eq!(back.signalness, None);
        assert_eq!(back.far, None);
    }
}
