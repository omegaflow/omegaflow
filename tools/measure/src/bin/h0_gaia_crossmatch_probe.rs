use std::io::Write;
use std::process::{Command, Stdio};

use omegaflow_measure::h0::{normalize_name, parse_cepheids};

const ARXIV_EPRINT: &str = "https://arxiv.org/e-print/2012.08534";
const SIMBAD_TAP: &str = "https://simbad.cds.unistra.fr/simbad/sim-tap/sync";
const GAIA_TAP: &str = "https://gea.esac.esa.int/tap-server/tap/sync";
const UA: &str = "omegaflow-h0-gaia-crossmatch/1.0";
const CONE_RADIUS_DEG: f64 = 3.0 / 3600.0;
const IDENTITY_MAX_SEP_ARCSEC: f64 = 2.0;
const TRANSCRIPTION_RIFT_UAS: f64 = 100.0;

struct SimbadRow {
    main_id: String,
    ra: f64,
    dec: f64,
    otype: String,
}

enum Resolve {
    Hit(SimbadRow),
    Miss,
    NoAnswer,
}

struct GaiaRow {
    source_id: i64,
    ra: f64,
    dec: f64,
    parallax: Option<f64>,
    parallax_error: Option<f64>,
}

fn unmeasured(msg: &str) -> ! {
    eprintln!("h0_gaia_crossmatch: {msg} — the crossmatch stays unmeasured");
    std::process::exit(1);
}

fn or_unmeasured<T>(opt: Option<T>, msg: &str) -> T {
    match opt {
        Some(v) => v,
        None => unmeasured(msg),
    }
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("600")
        .arg("-A")
        .arg(UA)
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(out.stdout)
}

fn sha256_stdin(bytes: &[u8]) -> Option<String> {
    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    child.stdin.take()?.write_all(bytes).ok()?;
    let out = child.wait_with_output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    text.split_whitespace().next().map(|s| s.to_string())
}

fn gzip_tar_member(tarball: &[u8], member: &str) -> Option<String> {
    let dir = std::env::temp_dir().join(format!("omegaflow_h0_crossmatch_{}", std::process::id()));
    std::fs::create_dir_all(&dir).ok()?;
    let tar_path = dir.join("source.tar.gz");
    std::fs::write(&tar_path, tarball).ok()?;
    let out = Command::new("tar")
        .arg("-xzOf")
        .arg(&tar_path)
        .arg(member)
        .output()
        .ok()?;
    let _ = std::fs::remove_dir_all(&dir);
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

fn tap_csv(tap: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("180")
        .arg("-A")
        .arg(UA)
        .arg("-G")
        .arg(tap)
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
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
        } else {
            cur.push(c);
        }
    }
    out.push(cur);
    out
}

fn sql_literal(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

fn sep_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let d_ra = (ra1 - ra2).to_radians() * dec1.to_radians().cos();
    let d_dec = (dec1 - dec2).to_radians();
    (d_ra * d_ra + d_dec * d_dec).sqrt().to_degrees() * 3600.0
}

fn simbad_resolve_one(name: &str) -> Resolve {
    let adql = format!(
        "SELECT b.main_id, b.ra, b.dec, b.otype_txt FROM ident i JOIN basic b ON i.oidref=b.oid WHERE i.id = {}",
        sql_literal(name)
    );
    let Some(csv) = tap_csv(SIMBAD_TAP, &adql) else {
        return Resolve::NoAnswer;
    };
    let mut lines = csv.lines();
    let Some(header) = lines.next() else {
        return Resolve::NoAnswer;
    };
    if !header.starts_with("main_id") {
        return Resolve::NoAnswer;
    }
    for line in lines {
        let f = csv_fields(line);
        if f.len() < 4 {
            continue;
        }
        let Ok(ra) = f[1].parse::<f64>() else {
            continue;
        };
        let Ok(dec) = f[2].parse::<f64>() else {
            continue;
        };
        if !ra.is_finite() || !dec.is_finite() {
            continue;
        }
        return Resolve::Hit(SimbadRow {
            main_id: f[0].clone(),
            ra,
            dec,
            otype: f[3].clone(),
        });
    }
    Resolve::Miss
}

fn simbad_resolve_all(names: &[String]) -> Vec<Resolve> {
    let workers = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
        .min(8);
    let chunk_size = names.len().div_ceil(workers);
    std::thread::scope(|s| {
        let handles: Vec<_> = names
            .chunks(chunk_size)
            .map(|chunk| {
                let n = chunk.len();
                let h = s.spawn(move || {
                    chunk
                        .iter()
                        .map(|x| simbad_resolve_one(x))
                        .collect::<Vec<_>>()
                });
                (n, h)
            })
            .collect();
        let mut out = Vec::with_capacity(names.len());
        for (n, h) in handles {
            match h.join() {
                Ok(v) => out.extend(v),
                Err(_) => out.extend((0..n).map(|_| Resolve::NoAnswer)),
            }
        }
        out
    })
}

fn gaia_cone(positions: &[(f64, f64)]) -> Option<Vec<GaiaRow>> {
    if positions.is_empty() {
        return Some(Vec::new());
    }
    let conds = positions
        .iter()
        .map(|(ra, dec)| {
            format!("1=CONTAINS(POINT('ICRS',ra,dec),CIRCLE('ICRS',{ra},{dec},{CONE_RADIUS_DEG}))")
        })
        .collect::<Vec<_>>()
        .join(" OR ");
    let adql = format!(
        "SELECT source_id,ra,dec,parallax,parallax_error FROM gaiadr3.gaia_source WHERE {conds}"
    );
    let csv = tap_csv(GAIA_TAP, &adql)?;
    let mut lines = csv.lines();
    let header = lines.next()?;
    if !header.starts_with("source_id") {
        return None;
    }
    let mut rows = Vec::new();
    for line in lines {
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 5 {
            continue;
        }
        let source_id = match f[0].trim().parse::<i64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let ra = match f[1].trim().parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let dec = match f[2].trim().parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !ra.is_finite() || !dec.is_finite() {
            continue;
        }
        let parallax = f[3].trim().parse::<f64>().ok().filter(|v| v.is_finite());
        let parallax_error = f[4].trim().parse::<f64>().ok().filter(|v| v.is_finite());
        rows.push(GaiaRow {
            source_id,
            ra,
            dec,
            parallax,
            parallax_error,
        });
    }
    Some(rows)
}

fn vari_cepheid(source_ids: &[i64]) -> Option<std::collections::HashMap<i64, String>> {
    let mut map = std::collections::HashMap::new();
    if source_ids.is_empty() {
        return Some(map);
    }
    let id_list = source_ids
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let adql = format!(
        "SELECT source_id,type_best_classification FROM gaiadr3.vari_cepheid WHERE source_id IN ({id_list})"
    );
    let csv = tap_csv(GAIA_TAP, &adql)?;
    let mut lines = csv.lines();
    let header = lines.next()?;
    if !header.starts_with("source_id") {
        return None;
    }
    for line in lines {
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 2 {
            continue;
        }
        if let Ok(sid) = f[0].trim().parse::<i64>() {
            map.insert(sid, f[1].trim().to_string());
        }
    }
    Some(map)
}

fn main() {
    let tarball = or_unmeasured(
        curl_bytes(ARXIV_EPRINT),
        "the arXiv e-print returned no bytes",
    );
    let tarball_sha = or_unmeasured(sha256_stdin(&tarball), "the arXiv tarball hash is absent");
    let tex = or_unmeasured(
        gzip_tar_member(&tarball, "bigtable_redux3.tex"),
        "bigtable_redux3.tex is absent from the arXiv package",
    );
    let (stars, absent) = parse_cepheids(&tex);
    if stars.is_empty() {
        unmeasured("the Cepheid table carries no parsed rows");
    }

    let names: Vec<String> = stars.iter().map(|s| normalize_name(&s.name)).collect();
    let resolves = simbad_resolve_all(&names);

    let mut resolved_pos: Vec<(f64, f64)> = Vec::new();
    for r in &resolves {
        if let Resolve::Hit(row) = r {
            resolved_pos.push((row.ra, row.dec));
        }
    }

    let mut gaia_rows: Vec<GaiaRow> = Vec::new();
    for chunk in resolved_pos.chunks(15) {
        let part = or_unmeasured(gaia_cone(chunk), "the Gaia DR3 cone query did not answer");
        gaia_rows.extend(part);
    }

    let matched_ids: Vec<i64> = gaia_rows.iter().map(|g| g.source_id).collect();
    let cepheid_class = or_unmeasured(
        vari_cepheid(&matched_ids),
        "the gaiadr3.vari_cepheid classification did not answer",
    );

    println!("h0_gaia_crossmatch: arXiv 2012.08534 sha256={tarball_sha} | SIMBAD sim-tap + Gaia DR3 tap live");
    println!(
        "h0_gaia_crossmatch: Cepheid table N={} rows, {} π_EDR3 absent, {} fitted",
        stars.len(),
        absent,
        stars.iter().filter(|s| s.pi_edr3.is_some()).count()
    );
    let resolved_count = resolves
        .iter()
        .filter(|r| matches!(r, Resolve::Hit(_)))
        .count();
    println!(
        "h0_gaia_crossmatch: {} of {} names resolve via SIMBAD ident→basic; {} Gaia sources returned by the cones",
        resolved_count,
        stars.len(),
        gaia_rows.len()
    );

    let mut unresolved = 0usize;
    let mut absent_source = 0usize;
    let mut wide = 0usize;
    let mut matched = 0usize;
    let mut deltas: Vec<f64> = Vec::new();
    let mut used_sources: std::collections::HashSet<i64> = std::collections::HashSet::new();
    let mut rifts: Vec<String> = Vec::new();

    println!("h0_gaia_crossmatch: per-star verdict");
    for (i, resolve) in resolves.iter().enumerate() {
        let s = &stars[i];
        let r = match resolve {
            Resolve::Hit(row) => row,
            Resolve::Miss => {
                unresolved += 1;
                println!("  {} -> no SIMBAD ident (0 honored)", s.name);
                continue;
            }
            Resolve::NoAnswer => {
                unresolved += 1;
                println!(
                    "  {} -> the SIMBAD resolver did not answer (pending)",
                    s.name
                );
                continue;
            }
        };
        let mut nearest: Option<(&GaiaRow, f64)> = None;
        for g in &gaia_rows {
            let d = sep_arcsec(r.ra, r.dec, g.ra, g.dec);
            if nearest.map_or(true, |(_, bd)| d < bd) {
                nearest = Some((g, d));
            }
        }
        match nearest {
            None => {
                absent_source += 1;
                println!(
                    "  {} -> {} | no Gaia DR3 source within {:.0} arcsec (0 honored)",
                    s.name,
                    r.main_id,
                    CONE_RADIUS_DEG * 3600.0
                );
            }
            Some((g, d)) => {
                if d > IDENTITY_MAX_SEP_ARCSEC {
                    wide += 1;
                    println!(
                        "  {} -> {} | nearest Gaia source {:.2} arcsec (wide — identity not certified)",
                        s.name, r.main_id, d
                    );
                    continue;
                }
                matched += 1;
                if !used_sources.insert(g.source_id) {
                    rifts.push(format!(
                        "{} source_id {} already matched by another star",
                        s.name, g.source_id
                    ));
                }
                let class = match cepheid_class.get(&g.source_id) {
                    Some(c) => c.clone(),
                    None => "absent".to_string(),
                };
                let gaia_word = match g.parallax {
                    Some(p) => match g.parallax_error {
                        Some(e) => format!("{p:.3} ± {e:.3} mas"),
                        None => format!("{p:.3} mas"),
                    },
                    None => "parallax absent".to_string(),
                };
                match (s.pi_edr3, g.parallax) {
                    (Some(pi), Some(gp)) => {
                        let delta_uas = (pi - gp) * 1000.0;
                        deltas.push(delta_uas);
                        let flag = if delta_uas.abs() > TRANSCRIPTION_RIFT_UAS {
                            " RIFT"
                        } else {
                            ""
                        };
                        println!(
                            "  {} -> {} | sep {:.2} arcsec | π_EDR3 {:.3} vs Gaia {} (Δ {:.0} μas) | otype {} | vari_cepheid {}{}",
                            s.name, r.main_id, d, pi, gaia_word, delta_uas, r.otype, class, flag
                        );
                    }
                    (Some(pi), None) => {
                        println!(
                            "  {} -> {} | sep {:.2} arcsec | π_EDR3 {:.3} vs Gaia parallax absent | otype {} | vari_cepheid {}",
                            s.name, r.main_id, d, pi, r.otype, class
                        );
                    }
                    (None, _) => {
                        println!(
                            "  {} -> {} | sep {:.2} arcsec | π_EDR3 absent | Gaia {} | otype {} | vari_cepheid {}",
                            s.name, r.main_id, d, gaia_word, r.otype, class
                        );
                    }
                }
            }
        }
    }

    println!(
        "h0_gaia_crossmatch: tally — matched {matched} | unresolved {unresolved} | absent-source {absent_source} | wide {wide}"
    );
    if !deltas.is_empty() {
        let mut sorted = deltas.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let median = sorted[sorted.len() / 2];
        let min = sorted[0];
        let max = sorted[sorted.len() - 1];
        println!(
            "h0_gaia_crossmatch: parallax offset π_EDR3 − Gaia_DR3 over {n} matched+fitted stars — median {median:.0} μas, range [{min:.0}, {max:.0}] μas (the table carries the L20b zero-point, not the −14 μas residual zp — table note d)",
            n = deltas.len()
        );
    }

    let gate = unresolved == 0 && absent_source == 0 && wide == 0 && rifts.is_empty();
    if gate {
        println!(
            "h0_gaia_crossmatch: identity gate PASS — every table name resolves to a unique Gaia DR3 source within {IDENTITY_MAX_SEP_ARCSEC} arcsec"
        );
    } else {
        println!("h0_gaia_crossmatch: identity gate RIFT — the deviations above are the finding, registered, not hidden");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_fields_splits_quoted_and_unquoted() {
        let line = "\"V* AA Gem\",91.64560871194,26.32922281127,\"cC*\"";
        let f = csv_fields(line);
        assert_eq!(f.len(), 4);
        assert_eq!(f[0], "V* AA Gem");
        assert_eq!(f[1], "91.64560871194");
        assert_eq!(f[3], "cC*");
    }

    #[test]
    fn csv_fields_handles_embedded_quote() {
        let f = csv_fields("\"a\"\"b\",1");
        assert_eq!(f.len(), 2);
        assert_eq!(f[0], "a\"b");
    }

    #[test]
    fn sql_literal_escapes_quotes() {
        assert_eq!(sql_literal("S Vul"), "'S Vul'");
        assert_eq!(sql_literal("O'Brien"), "'O''Brien'");
    }

    #[test]
    fn separation_is_small_for_nearby_points() {
        let d = sep_arcsec(100.0, 15.0, 100.0, 15.0 + 1.0 / 3600.0);
        assert!((d - 1.0).abs() < 1e-6);
    }
}
