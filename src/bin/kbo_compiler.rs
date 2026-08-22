use omegaflow::json::{parse_json, JsonVal};
use omegaflow::kbo::{
    family_name, family_of, name_of, packed_epoch_to_jd, write_bin, KboRec, MPC_ABSENT, MPC_AGREE,
    MPC_DISAGREE, NAME_BYTES,
};
use std::collections::HashMap;
use std::process::Command;

const SBDB_BASE_URL: &str = "https://ssd-api.jpl.nasa.gov/sbdb_query.api?fields=full_name,a,e,i,om,w,ma,epoch,H,class&sb-cdata=%7B%22AND%22%3A%5B%22a%7CGT%7C30%22%2C%22a%7CLT%7C200%22%2C%22e%7CLT%7C0.9%22%5D%7D&sb-kind=a&limit=1000&sb-class=TNO&full-prec=true&limit-from=";
const SBDB_ETNO_URL: &str = "https://ssd-api.jpl.nasa.gov/sbdb_query.api?fields=full_name,a,e,i,om,w,ma,epoch,H,class&sb-cdata=%7B%22AND%22%3A%5B%22a%7CGT%7C200%22%2C%22e%7CLT%7C0.99%22%5D%7D&sb-kind=a&limit=1000&sb-class=TNO&full-prec=true&limit-from=";
const MPC_DISTANT_URL: &str = "https://www.minorplanetcenter.net/iau/MPCORB/Distant.txt";
const PAGE: usize = 1000;
const DA_AU: f64 = 5e-3;
const DE: f64 = 5e-3;
const DI_DEG: f64 = 0.05;
const EPOCH_GATE_JD: f64 = 30.0;

#[derive(Clone, Copy)]
struct MpcRec {
    a_au: f64,
    e: f64,
    i_deg: f64,
    epoch_jd: f64,
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--max-time")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

fn field_index_map(fields: &[JsonVal]) -> HashMap<String, usize> {
    fields
        .iter()
        .enumerate()
        .filter_map(|(k, f)| match f {
            JsonVal::Str(s) => Some((s.clone(), k)),
            _ => None,
        })
        .collect()
}

fn cell_str(row: &[JsonVal], idx: usize) -> Option<&str> {
    match row.get(idx) {
        Some(JsonVal::Str(s)) => Some(s),
        _ => None,
    }
}

fn cell_f64(row: &[JsonVal], idx: usize) -> Option<f64> {
    cell_str(row, idx)?.trim().parse::<f64>().ok()
}

fn parse_sbdb_rows(body: &str) -> Option<(Vec<Vec<JsonVal>>, i64, HashMap<String, usize>)> {
    let root = parse_json(body)?;
    let (fields, data, count) = match &root {
        JsonVal::Obj(m) => (
            m.get("fields")?,
            m.get("data")?,
            m.get("count").and_then(|c| match c {
                JsonVal::Num(n) => Some(*n as i64),
                _ => None,
            })?,
        ),
        _ => return None,
    };
    let JsonVal::Arr(fields) = fields else {
        return None;
    };
    let idx = field_index_map(fields);
    let JsonVal::Arr(rows) = data else {
        return None;
    };
    let rows: Vec<Vec<JsonVal>> = rows
        .iter()
        .filter_map(|r| match r {
            JsonVal::Arr(cells) => Some(cells.clone()),
            _ => None,
        })
        .collect();
    Some((rows, count, idx))
}

fn prov_of(text: &str) -> Option<String> {
    let t = text.trim();
    let inner = if let (Some(a), Some(b)) = (t.find('('), t.find(')')) {
        if b > a + 1 {
            t[a + 1..b].trim()
        } else {
            t
        }
    } else {
        t
    };
    prov_pattern(inner)
}

fn prov_pattern(s: &str) -> Option<String> {
    let b = s.as_bytes();
    if b.len() < 6 || !b[0..4].iter().all(|c| c.is_ascii_digit()) || b[4] != b' ' {
        return None;
    }
    if !(b[5] as char).is_ascii_uppercase() {
        return None;
    }
    let mut end = 6;
    if end < b.len() && (b[end] as char).is_ascii_alphabetic() {
        end += 1;
    }
    while end < b.len() && b[end].is_ascii_digit() {
        end += 1;
    }
    if end < b.len() {
        return None;
    }
    Some(s.to_string())
}

fn leading_number(text: &str) -> Option<i64> {
    let t = text.trim();
    if let (Some(a), Some(b)) = (t.find('('), t.find(')')) {
        if b > a + 1 && t[a + 1..b].bytes().all(|c| c.is_ascii_digit()) {
            return t[a + 1..b].parse::<i64>().ok();
        }
    }
    let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse::<i64>().ok()
}

fn packed_number(text: &str) -> Option<i64> {
    let t = text.trim();
    let b = t.as_bytes();
    if b.is_empty() {
        return None;
    }
    if (b[0] as char).is_ascii_alphabetic() && b.len() >= 5 {
        let tens = pack_digit(b[0])?;
        let rest: String = t[1..5].chars().filter(|c| c.is_ascii_digit()).collect();
        if rest.len() != 4 {
            return None;
        }
        return Some(tens * 10000 + rest.parse::<i64>().ok()?);
    }
    if b.iter().all(|c| c.is_ascii_digit()) {
        return t.parse::<i64>().ok();
    }
    None
}

fn pack_digit(c: u8) -> Option<i64> {
    match c {
        b'0'..=b'9' => Some((c - b'0') as i64),
        b'A'..=b'Z' => Some((c - b'A' + 10) as i64),
        b'a'..=b'z' => Some((c - b'a' + 36) as i64),
        _ => None,
    }
}

fn slice(line: &str, lo: usize, hi: usize) -> Option<&str> {
    line.get(lo..hi)
}

fn parse_mpc_line(line: &str) -> Option<(Option<i64>, Option<String>, MpcRec)> {
    let packed = slice(line, 0, 7)?.trim();
    if packed.is_empty() {
        return None;
    }
    let epoch = packed_epoch_to_jd(slice(line, 20, 25)?.trim())?;
    let a_au: f64 = slice(line, 92, 103)?.trim().parse().ok()?;
    let e: f64 = slice(line, 70, 79)?.trim().parse().ok()?;
    let i_deg: f64 = slice(line, 59, 68)?.trim().parse().ok()?;
    let readable = slice(line, 166, 194)
        .or_else(|| line.get(166..))
        .unwrap_or("")
        .trim();
    let num = packed_number(packed);
    let prov = if num.is_some() {
        None
    } else {
        prov_of(readable)
    };
    Some((
        num,
        prov,
        MpcRec {
            a_au,
            e,
            i_deg,
            epoch_jd: epoch,
        },
    ))
}

fn sbdb_rec(row: &[JsonVal], idx: &HashMap<String, usize>) -> Option<KboRec> {
    let i_name = *idx.get("full_name")?;
    let name = cell_str(row, i_name)?.trim();
    if name.is_empty() {
        return None;
    }
    let a = cell_f64(row, *idx.get("a")?)?;
    let e = cell_f64(row, *idx.get("e")?)?;
    let incl = cell_f64(row, *idx.get("i")?)?;
    let node = cell_f64(row, *idx.get("om")?)?;
    let peri = cell_f64(row, *idx.get("w")?)?;
    let ma = cell_f64(row, *idx.get("ma")?)?;
    let epoch = cell_f64(row, *idx.get("epoch")?)?;
    let h = cell_f64(row, *idx.get("H")?)?;
    if !a.is_finite() || a <= 0.0 || !e.is_finite() || e < 0.0 || e >= 1.0 || !epoch.is_finite() {
        return None;
    }
    let mut nm = [0u8; NAME_BYTES];
    let bytes = name.as_bytes();
    if bytes.len() > NAME_BYTES {
        return None;
    }
    nm[..bytes.len()].copy_from_slice(bytes);
    Some(KboRec {
        name: nm,
        a_au: a,
        e,
        incl_deg: incl,
        node_deg: node,
        peri_deg: peri,
        ma_deg: ma,
        epoch_jd: epoch,
        h_mag: h,
        family: family_of(a, e),
        mpc_flag: MPC_ABSENT,
    })
}

fn cross_check(
    recs: &mut [KboRec],
    num_map: &HashMap<i64, MpcRec>,
    prov_map: &HashMap<String, MpcRec>,
    mpc_all: &[MpcRec],
) -> (usize, usize, usize, usize, usize, usize, Vec<String>) {
    let mut agree = 0usize;
    let mut disagree = 0usize;
    let mut absent = 0usize;
    let mut elem_join = 0usize;
    let mut ambiguous = 0usize;
    let mut epoch_distant = 0usize;
    let mut samples: Vec<String> = Vec::new();
    for rec in recs.iter_mut() {
        let name = name_of(rec).to_string();
        let mpc = num_map
            .get(&leading_number(&name).unwrap_or(-1))
            .or_else(|| prov_map.get(&prov_of(&name).unwrap_or_default()));
        match mpc {
            Some(m) => {
                if (rec.epoch_jd - m.epoch_jd).abs() > EPOCH_GATE_JD {
                    epoch_distant += 1;
                    continue;
                }
                if (rec.a_au - m.a_au).abs() < DA_AU
                    && (rec.e - m.e).abs() < DE
                    && (rec.incl_deg - m.i_deg).abs() < DI_DEG
                {
                    rec.mpc_flag = MPC_AGREE;
                    agree += 1;
                } else {
                    rec.mpc_flag = MPC_DISAGREE;
                    disagree += 1;
                    if samples.len() < 10 {
                        samples.push(format!(
                            "{name}: sbdb a {} e {} i {} | mpc a {} e {} i {} | de {}",
                            rec.a_au,
                            rec.e,
                            rec.incl_deg,
                            m.a_au,
                            m.e,
                            m.i_deg,
                            rec.epoch_jd - m.epoch_jd
                        ));
                    }
                }
            }
            None => {
                let hits: Vec<&MpcRec> = mpc_all
                    .iter()
                    .filter(|m| {
                        (rec.a_au - m.a_au).abs() < DA_AU
                            && (rec.e - m.e).abs() < DE
                            && (rec.incl_deg - m.i_deg).abs() < DI_DEG
                    })
                    .collect();
                match hits.len() {
                    1 => {
                        rec.mpc_flag = MPC_AGREE;
                        elem_join += 1;
                    }
                    0 => absent += 1,
                    _ => ambiguous += 1,
                }
            }
        }
    }
    (
        agree,
        disagree,
        absent,
        elem_join,
        ambiguous,
        epoch_distant,
        samples,
    )
}

fn fetch_pages(
    url_base: &str,
    offline_dir: Option<&str>,
    tag: &str,
) -> (Vec<Vec<JsonVal>>, Option<HashMap<String, usize>>, i64) {
    let mut rows = Vec::new();
    let mut idx = None;
    let mut first_count = 0i64;
    let mut from = 0usize;
    let mut short_pages = 0usize;
    loop {
        let body = match offline_dir {
            Some(dir) => std::fs::read_to_string(format!("{dir}/sbdb_{tag}_{from}.json")).ok(),
            None => curl(&format!("{url_base}{from}")),
        };
        let Some(body) = body else {
            eprintln!("{tag} page {from}: fetch void");
            break;
        };
        let Some((page_rows, count, page_idx)) = parse_sbdb_rows(&body) else {
            eprintln!("{tag} page {from}: json shape void");
            break;
        };
        if page_rows.is_empty() {
            break;
        }
        if from == 0 {
            first_count = count;
        }
        let len = page_rows.len();
        if idx.is_none() {
            idx = Some(page_idx);
        }
        rows.extend(page_rows);
        if len < PAGE {
            if short_pages >= 1 {
                break;
            }
            short_pages += 1;
        }
        from += PAGE;
    }
    (rows, idx, first_count)
}

fn dedupe(rows: Vec<Vec<JsonVal>>, idx: &HashMap<String, usize>) -> (Vec<Vec<JsonVal>>, usize) {
    let Some(&i_name) = idx.get("full_name") else {
        return (rows, 0);
    };
    let mut seen = std::collections::HashSet::new();
    let mut dup = 0usize;
    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let key = match r.get(i_name) {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => String::new(),
        };
        if key.is_empty() || !seen.insert(key) {
            dup += 1;
        } else {
            out.push(r);
        }
    }
    (out, dup)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out = String::new();
    let mut ci_mode = false;
    let mut etno = false;
    let mut offline: Option<String> = None;
    let mut probe: Option<(String, f64)> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or_default();
            }
            "--ci-mode" => ci_mode = true,
            "--etno" => etno = true,
            "--offline" => {
                i += 1;
                offline = args.get(i).cloned();
            }
            "--probe" => {
                i += 1;
                let n = args.get(i).cloned().unwrap_or_default();
                i += 1;
                let jd = args.get(i).and_then(|s| s.parse::<f64>().ok());
                if let Some(jd) = jd {
                    probe = Some((n, jd));
                }
            }
            _ => {
                eprintln!(
                    "usage: kbo_compiler --out <kbo_elements.bin> [--ci-mode] [--etno] [--offline <dir>] [--probe <name> <jd>]"
                );
                return;
            }
        }
        i += 1;
    }
    if out.is_empty() {
        eprintln!("usage: kbo_compiler --out <kbo_elements.bin> [--ci-mode] [--etno] [--offline <dir>] [--probe <name> <jd>]");
        return;
    }

    let (base_rows, idx, base_count) = fetch_pages(SBDB_BASE_URL, offline.as_deref(), "base");
    let mut all_rows = base_rows;
    let mut etno_count = 0i64;
    if etno {
        let (etno_rows, _, c) = fetch_pages(SBDB_ETNO_URL, offline.as_deref(), "etno");
        etno_count = c;
        all_rows.extend(etno_rows);
    }
    if all_rows.is_empty() {
        eprintln!("sbdb harvest void");
        return;
    }
    let idx = idx.unwrap_or_default();
    let (all_rows, dup) = dedupe(all_rows, &idx);
    eprintln!(
        "sbdb: base count {} + etno count {}, zeilen {}, doppelte {}",
        base_count,
        etno_count,
        all_rows.len(),
        dup
    );

    let body = match offline.as_deref() {
        Some(dir) => std::fs::read_to_string(format!("{dir}/distant.txt")).ok(),
        None => curl(MPC_DISTANT_URL),
    };
    let mut num_map: HashMap<i64, MpcRec> = HashMap::new();
    let mut prov_map: HashMap<String, MpcRec> = HashMap::new();
    let mut mpc_all: Vec<MpcRec> = Vec::new();
    let mut short_lines = 0usize;
    if let Some(text) = body {
        for line in text.lines() {
            match parse_mpc_line(line) {
                Some((num, prov, m)) => {
                    mpc_all.push(m);
                    if let Some(n) = num {
                        num_map.entry(n).or_insert(m);
                    }
                    if let Some(p) = prov {
                        prov_map.entry(p).or_insert(m);
                    }
                }
                None => short_lines += 1,
            }
        }
    } else {
        eprintln!("mpc distant fetch void — Kreuzcheck entfaellt");
    }

    let mut recs: Vec<KboRec> = Vec::new();
    let mut skipped = 0usize;
    for row in &all_rows {
        match sbdb_rec(row, &idx) {
            Some(r) => recs.push(r),
            None => skipped += 1,
        }
    }
    let (agree, disagree, absent, elem_join, ambiguous, epoch_distant, samples) =
        cross_check(&mut recs, &num_map, &prov_map, &mpc_all);

    let mut fam_counts: HashMap<u8, usize> = HashMap::new();
    for r in &recs {
        *fam_counts.entry(r.family).or_insert(0) += 1;
    }
    let mut fams: Vec<u8> = fam_counts.keys().copied().collect();
    fams.sort();

    let mut individual_epochs = 0usize;
    for r in &recs {
        if (r.epoch_jd - 2461200.5).abs() > 1e-6 {
            individual_epochs += 1;
        }
    }

    let bytes = write_bin(&recs);
    std::fs::write(&out, &bytes).expect("write bin");
    eprintln!("out {out}: {} records ({} bytes)", recs.len(), bytes.len());
    eprintln!(
        "ernte: {} zeilen sbdb, {} uebersprungen (0-Kanon), mpc zeilen {}, kurz {}",
        all_rows.len(),
        skipped,
        mpc_all.len(),
        short_lines
    );
    for f in fams {
        eprintln!("familie {:<9} n {}", family_name(f), fam_counts[&f]);
    }
    eprintln!(
        "mpc: agree {}, disagree {}, absent {}, element-join {}, ambiguous {}, epoch-distant {}",
        agree, disagree, absent, elem_join, ambiguous, epoch_distant
    );
    for s in samples {
        eprintln!("disagree: {s}");
    }
    eprintln!("epochen: individual {}", individual_epochs);

    if let Some((n, jd)) = probe {
        let mut found: Option<KboRec> = None;
        for r in &recs {
            if name_of(r).contains(&n) {
                found = Some(*r);
                break;
            }
        }
        match found {
            Some(r) => {
                let state = omegaflow::kbo::state_at(&r, jd);
                eprintln!(
                    "probe {n} @ jd {jd}: a {} e {} i {} fam {} mpc {} state {:?}",
                    r.a_au,
                    r.e,
                    r.incl_deg,
                    family_name(r.family),
                    r.mpc_flag,
                    state.map(|(p, _)| p)
                );
            }
            None => eprintln!("probe {n}: not in harvest"),
        }
    }

    if ci_mode && !omegaflow::cdn::upload_asset(&out) {
        eprintln!("ci upload void (GH_TOKEN absent)");
    }
}
