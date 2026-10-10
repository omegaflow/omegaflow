use omegaflow::cdn::upload_release;
use std::process::Command;

const MAGIC: [u8; 4] = *b"GRRL";
const HEADER_BYTES: usize = 8;
const RECORD_BYTES: usize = 51;
const GAIA_TAP_SYNC: &str = "https://gea.esac.esa.int/tap-server/tap/sync";
const CDN_RELEASE: &str = "gea.esac.esa.int";
const DEFAULT_OUT: &str = "data/gea.esac.esa.int/gaia_rrl.bin";
const USER_AGENT: &str = "omegaflow-gaia-rrl-compiler/1.0";

pub const RRL_ADQL: &str = "SELECT TOP 500 g.source_id, g.ra, g.dec, g.phot_g_mean_mag, g.parallax, g.bp_rp, v.best_class_name FROM gaiadr3.vari_classifier_result AS v JOIN gaiadr3.gaia_source AS g ON g.source_id = v.source_id WHERE v.best_class_name = 'RR'";

#[derive(Clone, PartialEq, Debug)]
pub struct RrlRecord {
    pub source_id: u64,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub g_mag: Option<f64>,
    pub parallax_mas: Option<f64>,
    pub bp_rp: Option<f64>,
}

pub struct RrlCounts {
    pub header_rows: usize,
    pub rows: usize,
    pub emitted: usize,
    pub refused: usize,
    pub g_absent: usize,
    pub parallax_absent: usize,
    pub bp_rp_absent: usize,
}

fn csv_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_q {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_q = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_q = true;
        } else if c == ',' {
            out.push(std::mem::take(&mut cur));
        } else if c != '\r' {
            cur.push(c);
        }
    }
    out.push(cur);
    out
}

fn column_index(header: &[String], name: &str) -> Option<usize> {
    header.iter().position(|h| h == name)
}

fn opt_f64(field: &str) -> Option<Option<f64>> {
    let t = field.trim();
    if t.is_empty() {
        Some(None)
    } else {
        match t.parse::<f64>() {
            Ok(v) if v.is_finite() => Some(Some(v)),
            _ => None,
        }
    }
}

fn ra_dec_plausible(ra: f64, dec: f64) -> bool {
    ra.is_finite() && dec.is_finite() && (0.0..360.0).contains(&ra) && (-90.0..=90.0).contains(&dec)
}

pub fn parse_rrl_csv(body: &str) -> Option<(Vec<RrlRecord>, RrlCounts)> {
    let mut lines = body.lines();
    let header = csv_fields(lines.next()?);
    let idx_source = column_index(&header, "source_id")?;
    let idx_ra = column_index(&header, "ra")?;
    let idx_dec = column_index(&header, "dec")?;
    let idx_g = column_index(&header, "phot_g_mean_mag")?;
    let idx_par = column_index(&header, "parallax")?;
    let idx_bp = column_index(&header, "bp_rp")?;
    let idx_class = column_index(&header, "best_class_name")?;
    let mut counts = RrlCounts {
        header_rows: header.len(),
        rows: 0,
        emitted: 0,
        refused: 0,
        g_absent: 0,
        parallax_absent: 0,
        bp_rp_absent: 0,
    };
    let mut out = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        counts.rows += 1;
        let fields = csv_fields(line);
        let at = |i: usize| fields.get(i).map(|s| s.as_str());
        let (
            Some(source),
            Some(ra_s),
            Some(dec_s),
            Some(g_s),
            Some(par_s),
            Some(bp_s),
            Some(class),
        ) = (
            at(idx_source),
            at(idx_ra),
            at(idx_dec),
            at(idx_g),
            at(idx_par),
            at(idx_bp),
            at(idx_class),
        )
        else {
            counts.refused += 1;
            continue;
        };
        if class.trim() != "RR" {
            counts.refused += 1;
            continue;
        }
        let (Ok(source_id), Ok(ra), Ok(dec)) = (
            source.trim().parse::<u64>(),
            ra_s.trim().parse::<f64>(),
            dec_s.trim().parse::<f64>(),
        ) else {
            counts.refused += 1;
            continue;
        };
        if !ra_dec_plausible(ra, dec) {
            counts.refused += 1;
            continue;
        }
        let (Some(g_mag), Some(parallax_mas), Some(bp_rp)) =
            (opt_f64(g_s), opt_f64(par_s), opt_f64(bp_s))
        else {
            counts.refused += 1;
            continue;
        };
        if g_mag.is_none() {
            counts.g_absent += 1;
        }
        if parallax_mas.is_none() {
            counts.parallax_absent += 1;
        }
        if bp_rp.is_none() {
            counts.bp_rp_absent += 1;
        }
        counts.emitted += 1;
        out.push(RrlRecord {
            source_id,
            ra_deg: ra,
            dec_deg: dec,
            g_mag,
            parallax_mas,
            bp_rp,
        });
    }
    Some((out, counts))
}

fn push_optional(out: &mut Vec<u8>, value: Option<f64>) -> Option<()> {
    match value {
        Some(v) if v.is_finite() => {
            out.push(1);
            out.extend_from_slice(&v.to_le_bytes());
        }
        Some(_) => return None,
        None => {
            out.push(0);
            out.extend_from_slice(&0.0f64.to_le_bytes());
        }
    }
    Some(())
}

pub fn write_bin(records: &[RrlRecord]) -> Option<Vec<u8>> {
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        if !ra_dec_plausible(r.ra_deg, r.dec_deg) {
            return None;
        }
        out.extend_from_slice(&r.source_id.to_le_bytes());
        out.extend_from_slice(&r.ra_deg.to_le_bytes());
        out.extend_from_slice(&r.dec_deg.to_le_bytes());
        push_optional(&mut out, r.g_mag)?;
        push_optional(&mut out, r.parallax_mas)?;
        push_optional(&mut out, r.bp_rp)?;
    }
    Some(out)
}

fn read_optional(bytes: &[u8], off: &mut usize) -> Option<Option<f64>> {
    let flag = *bytes.get(*off)?;
    *off += 1;
    let raw = f64::from_le_bytes(bytes.get(*off..*off + 8)?.try_into().ok()?);
    *off += 8;
    match flag {
        0 => Some(None),
        1 if raw.is_finite() => Some(Some(raw)),
        _ => None,
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<RrlRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count * RECORD_BYTES {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let source_id = u64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let ra_deg = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let dec_deg = f64::from_le_bytes(bytes.get(off + 16..off + 24)?.try_into().ok()?);
        off += 24;
        let g_mag = read_optional(bytes, &mut off)?;
        let parallax_mas = read_optional(bytes, &mut off)?;
        let bp_rp = read_optional(bytes, &mut off)?;
        if !ra_dec_plausible(ra_deg, dec_deg) {
            return None;
        }
        out.push(RrlRecord {
            source_id,
            ra_deg,
            dec_deg,
            g_mag,
            parallax_mas,
            bp_rp,
        });
    }
    Some(out)
}

fn tap_csv(adql: &str) -> Option<(String, Vec<u8>)> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("300")
        .arg("-A")
        .arg(USER_AGENT)
        .arg("-G")
        .arg(GAIA_TAP_SYNC)
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .output()
        .ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_path = DEFAULT_OUT.to_string();
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = match args.get(i + 1) {
                    Some(p) => p.clone(),
                    None => {
                        eprintln!("gaia_rrl: --out needs a path");
                        std::process::exit(2);
                    }
                };
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--help" | "-h" => {
                println!("usage: gaia_rrl_compiler [--out <gaia_rrl.bin>] [--ci-mode]");
                return;
            }
            other => {
                eprintln!("gaia_rrl: unknown argument {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let Some((code, body)) = tap_csv(RRL_ADQL) else {
        println!(
            "gaia_rrl: the Gaia TAP query did not answer (measured stall) — the harvest stays pending"
        );
        std::process::exit(1);
    };
    if code != "200" {
        println!(
            "gaia_rrl: the Gaia TAP endpoint answered HTTP {code} — the harvest stays pending"
        );
        std::process::exit(1);
    }
    let Ok(text) = std::str::from_utf8(&body) else {
        println!("gaia_rrl: the HTTP 200 body is not UTF-8 — the harvest stays pending");
        std::process::exit(1);
    };
    let Some((records, counts)) = parse_rrl_csv(text) else {
        println!(
            "gaia_rrl: the HTTP 200 CSV did not carry the measured columns — the harvest stays pending"
        );
        std::process::exit(1);
    };
    println!(
        "gaia_rrl: rows {} emitted {} refused {} (g_absent {} parallax_absent {} bp_rp_absent {})",
        counts.rows,
        counts.emitted,
        counts.refused,
        counts.g_absent,
        counts.parallax_absent,
        counts.bp_rp_absent
    );
    if records.is_empty() {
        println!(
            "gaia_rrl: the query answered no RR record — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let Some(bytes) = write_bin(&records) else {
        println!(
            "gaia_rrl: a held record is not finite or not serializable — the asset stays unwritten"
        );
        std::process::exit(1);
    };
    match parse_bin(&bytes) {
        Some(parsed) if parsed == records => {
            println!(
                "gaia_rrl: {} B -> {} ({} records, roundtrip reads back)",
                bytes.len(),
                out_path,
                parsed.len()
            );
        }
        _ => {
            println!("gaia_rrl: the roundtrip does not read back — the asset stays unwritten");
            std::process::exit(1);
        }
    }
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bytes).is_err() {
        println!("gaia_rrl: write {out_path} returned void — the asset stays unwritten");
        std::process::exit(1);
    }
    if ci_mode && !upload_release(CDN_RELEASE, &out_path) {
        println!(
            "gaia_rrl: {out_path} did not reach the CDN — the local asset stands, the manifest is pending"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "source_id,ra,dec,phot_g_mean_mag,parallax,bp_rp,best_class_name\n\
5902329227124762112,227.281350641993,-49.807940181362135,17.71831,0.1135535987233241,0.9884186,RR\n\
6070124184786140032,202.03483641186935,-50.90975049184871,20.76024,,1.869997,RR\n\
123,999.0,-10.0,19.0,0.2,0.5,RR\n";

    #[test]
    fn csv_holds_two_of_three_rows_and_keeps_the_absent_parallax() {
        let (records, counts) = parse_rrl_csv(SAMPLE).expect("the measured header is present");
        assert_eq!(counts.rows, 3);
        assert_eq!(counts.emitted, 2);
        assert_eq!(counts.refused, 1);
        assert_eq!(counts.parallax_absent, 1);
        assert_eq!(records[0].source_id, 5902329227124762112);
        assert_eq!(records[1].parallax_mas, None);
    }

    #[test]
    fn roundtrip_preserves_present_and_absent_columns() {
        let (records, _) = parse_rrl_csv(SAMPLE).expect("the measured header is present");
        let bytes = write_bin(&records).expect("finite records encode");
        let parsed = parse_bin(&bytes).expect("the encoded asset reads back");
        assert_eq!(parsed, records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
    }

    #[test]
    fn parse_bin_refuses_a_truncated_asset() {
        let (records, _) = parse_rrl_csv(SAMPLE).expect("the measured header is present");
        let bytes = write_bin(&records).expect("finite records encode");
        assert_eq!(parse_bin(&bytes[..bytes.len() - 1]), None);
    }

    #[test]
    fn non_rr_class_is_refused() {
        let text = "source_id,ra,dec,phot_g_mean_mag,parallax,bp_rp,best_class_name\n\
1,10.0,-5.0,18.0,0.4,0.7,CEP\n";
        let (records, counts) = parse_rrl_csv(text).expect("the header is present");
        assert!(records.is_empty());
        assert_eq!(counts.refused, 1);
    }
}
