use omegaflow::json::{parse_json, JsonVal};
use std::collections::HashMap;
use std::process::Command;

const FINK_CONE: &str = "https://api.lsst.fink-portal.org/api/v1/conesearch";
const FINK_SOURCES: &str = "https://api.lsst.fink-portal.org/api/v1/sources";
const LAS_CONE: &str = "https://api.lasair.lsst.ac.uk/api/cone/";
const ALERCE_OBJECTS: &str = "https://api.alerce.online/objects";
const MATCH_RADIUS_ARCSEC: f64 = 2.0;
const VERDICT_BROKER_MIN: usize = 2;
const LAS_TUNNEL_IFACE: &str = "proton0";
const STALL_WORD: &str = "no response (connection stalled)";
const UA: &str = "omegaflow-broker-difference-probe/1.0";
const STATE_SAMPLE_DIR: &str = "/tmp/opencode";
const ALERCE_RETIRE_NOTE: &str =
    "api.alerce.online is the retired direct-database stub (dead_sources.φ: Direct database \
     access is being retired) — a non-200 read is no negative, excluded from the verdict";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Read {
    Present,
    Absent,
    Excluded,
}

struct BrokerLine {
    broker: &'static str,
    code: String,
    read: Read,
    note: String,
}

fn obj_f64(m: &HashMap<String, JsonVal>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(JsonVal::Num(n)) if n.is_finite() => Some(*n),
        _ => None,
    }
}

#[derive(PartialEq, Debug)]
enum ConeParse {
    Empty,
    Objects(usize),
    Unresolved,
}

fn id_present(v: Option<&JsonVal>) -> bool {
    match v {
        Some(JsonVal::Str(s)) => !s.is_empty(),
        Some(JsonVal::Num(n)) => n.is_finite(),
        _ => false,
    }
}

fn fink_cone_parse(body: &[u8]) -> ConeParse {
    let Ok(text) = std::str::from_utf8(body) else {
        return ConeParse::Unresolved;
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        return ConeParse::Unresolved;
    };
    if rows.is_empty() {
        return ConeParse::Empty;
    }
    let mut n = 0usize;
    for r in &rows {
        match r {
            JsonVal::Obj(m) if id_present(m.get("r:diaObjectId")) => n += 1,
            _ => return ConeParse::Unresolved,
        }
    }
    ConeParse::Objects(n)
}

fn lasair_cone_parse(body: &[u8]) -> ConeParse {
    let Ok(text) = std::str::from_utf8(body) else {
        return ConeParse::Unresolved;
    };
    let Some(JsonVal::Obj(root)) = parse_json(text) else {
        return ConeParse::Unresolved;
    };
    let Some(JsonVal::Arr(rows)) = root.get("objects") else {
        return ConeParse::Unresolved;
    };
    if rows.is_empty() {
        return ConeParse::Empty;
    }
    let mut n = 0usize;
    for r in rows {
        match r {
            JsonVal::Obj(m) if id_present(m.get("object")) => n += 1,
            _ => return ConeParse::Unresolved,
        }
    }
    ConeParse::Objects(n)
}

fn curl_get(url: &str, token: Option<&str>) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("40")
        .arg("-A")
        .arg(UA)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    if let Some(t) = token {
        cmd.arg("-H").arg(format!("Authorization: Token {t}"));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn curl_post(url: &str, json_body: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-A")
        .arg(UA)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(json_body)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn curl_form_post(
    url: &str,
    form: &str,
    token: Option<&str>,
    iface: Option<&str>,
) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-A")
        .arg(UA)
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(form)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    if let Some(t) = token {
        cmd.arg("-H").arg(format!("Authorization: Token {t}"));
    }
    if let Some(i) = iface {
        cmd.arg("--interface").arg(i);
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn cache_state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home)
            .join(".local")
            .join("state")
            .join("omegaflow");
    }
    std::path::PathBuf::from(".")
}

fn token_from(key: &str) -> Option<String> {
    if let Ok(t) = std::env::var(key) {
        if !t.is_empty() {
            return Some(t);
        }
    }
    let body = std::fs::read_to_string(cache_state_dir().join(".secrets.local")).ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn sample_name(sample_dir: &str, ra: f64, dec: f64, broker: &str) -> String {
    format!("{sample_dir}/broker_diff_{ra:.5}_{dec:.5}_{broker}.json")
}

fn save_sample(path: &str, body: &[u8]) {
    if std::fs::write(path, body).is_err() {
        println!("broker difference: the sample was not saved ({path})");
    }
}

fn fink_membership(ra: f64, dec: f64) -> BrokerLine {
    let payload = format!(
        "{{\"ra\": {ra:.8}, \"dec\": {dec:.8}, \"radius\": {MATCH_RADIUS_ARCSEC}, \"columns\": \"r:diaObjectId,r:ra,r:dec\"}}"
    );
    let Some((code, body)) = curl_post(FINK_CONE, &payload) else {
        return BrokerLine {
            broker: "fink",
            code: STALL_WORD.to_string(),
            read: Read::Excluded,
            note:
                "the conesearch query did not answer — excluded from the verdict (never a negative)"
                    .to_string(),
        };
    };
    if code != "200" {
        return BrokerLine {
            broker: "fink",
            code: code.clone(),
            read: Read::Excluded,
            note:
                "a non-200 conesearch read is no negative — excluded from the verdict (0 honored)"
                    .to_string(),
        };
    }
    let path = sample_name(STATE_SAMPLE_DIR, ra, dec, "fink");
    save_sample(&path, &body);
    match fink_cone_parse(&body) {
        ConeParse::Empty => BrokerLine {
            broker: "fink",
            code,
            read: Read::Absent,
            note: format!(
                "the {MATCH_RADIUS_ARCSEC} arcsec cone holds no diaObject row (0 honored); sample {path}"
            ),
        },
        ConeParse::Objects(n) => BrokerLine {
            broker: "fink",
            code,
            read: Read::Present,
            note: format!(
                "{n} diaObject row(s) within the {MATCH_RADIUS_ARCSEC} arcsec cone; sample {path}"
            ),
        },
        ConeParse::Unresolved => BrokerLine {
            broker: "fink",
            code,
            read: Read::Excluded,
            note: format!("the 200 body is not the measured conesearch row array (r:diaObjectId per row) — the membership read stays pending a real schema; excluded from the verdict (never a negative); sample {path}"),
        },
    }
}

fn lasair_token() -> Option<String> {
    token_from("LASAIR_LSST_TOKEN")
}

fn tunnel_iface_present(iface: &str) -> bool {
    let Ok(out) = Command::new("ip")
        .arg("-o")
        .arg("link")
        .arg("show")
        .arg(iface)
        .output()
    else {
        return false;
    };
    out.status.success()
}

fn fetch_code(fetch: &Option<(String, Vec<u8>)>) -> String {
    match fetch {
        Some((code, _)) => code.clone(),
        None => STALL_WORD.to_string(),
    }
}

fn route_down(fetch: &Option<(String, Vec<u8>)>) -> bool {
    match fetch {
        Some((code, _)) => code == "000",
        None => true,
    }
}

fn lasair_cone_line(
    ra: f64,
    dec: f64,
    code: &str,
    body: &[u8],
    token_word: &str,
    route_lead: &str,
) -> BrokerLine {
    let path = sample_name(STATE_SAMPLE_DIR, ra, dec, "lasair");
    save_sample(&path, body);
    match lasair_cone_parse(body) {
        ConeParse::Empty => BrokerLine {
            broker: "lasair",
            code: code.to_string(),
            read: Read::Absent,
            note: format!(
                "{route_lead}the {MATCH_RADIUS_ARCSEC} arcsec cone holds no object row (0 honored); {token_word}; sample {path}"
            ),
        },
        ConeParse::Objects(n) => BrokerLine {
            broker: "lasair",
            code: code.to_string(),
            read: Read::Present,
            note: format!(
                "{route_lead}{n} object row(s) within the {MATCH_RADIUS_ARCSEC} arcsec cone; {token_word}; sample {path}"
            ),
        },
        ConeParse::Unresolved => BrokerLine {
            broker: "lasair",
            code: code.to_string(),
            read: Read::Excluded,
            note: format!("{route_lead}the 200 body is not the measured cone row array (object per row) — the membership read stays pending a real schema; excluded from the verdict (never a negative); sample {path}"),
        },
    }
}

fn lasair_membership_core<F, P>(
    ra: f64,
    dec: f64,
    token: Option<&str>,
    iface_present: F,
    probe: P,
) -> BrokerLine
where
    F: Fn(&str) -> bool,
    P: Fn(Option<&str>) -> Option<(String, Vec<u8>)>,
{
    let token_word = match token {
        Some(_) => "LASAIR_LSST_TOKEN present",
        None => "LASAIR_LSST_TOKEN absent (env or the .secrets.local key)",
    };
    let direct = probe(None);
    let direct_code = fetch_code(&direct);
    if let Some((code, body)) = &direct {
        if code == "200" {
            return lasair_cone_line(ra, dec, code, body, token_word, "");
        }
    }
    if !route_down(&direct) {
        let note = format!(
            "the direct route answered HTTP {direct_code} — a non-200 cone read is no negative; excluded from the verdict (0 honored); {token_word}"
        );
        return BrokerLine {
            broker: "lasair",
            code: direct_code,
            read: Read::Excluded,
            note,
        };
    }
    if !iface_present(LAS_TUNNEL_IFACE) {
        let note = format!(
            "the direct route read {direct_code} and the {LAS_TUNNEL_IFACE} tunnel interface is not present (ip link) — no tunnel retry exists, the direct finding stands; excluded from the verdict (never a negative); {token_word}"
        );
        return BrokerLine {
            broker: "lasair",
            code: direct_code,
            read: Read::Excluded,
            note,
        };
    }
    let tunnel = probe(Some(LAS_TUNNEL_IFACE));
    let tunnel_code = fetch_code(&tunnel);
    if let Some((code, body)) = &tunnel {
        if code == "200" {
            let route_lead = format!(
                "the direct route read {direct_code} — the {LAS_TUNNEL_IFACE} tunnel carried the 200 — "
            );
            return lasair_cone_line(ra, dec, code, body, token_word, &route_lead);
        }
    }
    if !route_down(&tunnel) {
        let note = format!(
            "the direct route read {direct_code}; the {LAS_TUNNEL_IFACE} tunnel answered HTTP {tunnel_code} — a non-200 cone read is no negative; excluded from the verdict (0 honored); {token_word}"
        );
        return BrokerLine {
            broker: "lasair",
            code: tunnel_code,
            read: Read::Excluded,
            note,
        };
    }
    let note = format!(
        "the direct route read {direct_code} and the {LAS_TUNNEL_IFACE} tunnel route read {tunnel_code} — no route carries the cone; excluded from the verdict (never a negative); {token_word}"
    );
    BrokerLine {
        broker: "lasair",
        code: tunnel_code,
        read: Read::Excluded,
        note,
    }
}

fn lasair_membership(ra: f64, dec: f64) -> BrokerLine {
    let token = lasair_token();
    let form = format!("ra={ra:.8}&dec={dec:.8}&radius={MATCH_RADIUS_ARCSEC}&requestType=all");
    lasair_membership_core(ra, dec, token.as_deref(), tunnel_iface_present, |iface| {
        curl_form_post(LAS_CONE, &form, token.as_deref(), iface)
    })
}

fn alerce_membership(ra: f64, dec: f64) -> BrokerLine {
    let Some((code, body)) = curl_get(ALERCE_OBJECTS, None) else {
        return BrokerLine {
            broker: "alerce",
            code: STALL_WORD.to_string(),
            read: Read::Excluded,
            note: ALERCE_RETIRE_NOTE.to_string(),
        };
    };
    if code != "200" {
        return BrokerLine {
            broker: "alerce",
            code: code.clone(),
            read: Read::Excluded,
            note: ALERCE_RETIRE_NOTE.to_string(),
        };
    }
    let path = sample_name(STATE_SAMPLE_DIR, ra, dec, "alerce");
    save_sample(&path, &body);
    let Ok(text) = std::str::from_utf8(&body) else {
        return BrokerLine {
            broker: "alerce",
            code,
            read: Read::Excluded,
            note: format!("the 200 body is not UTF-8 — the membership read stays pending; excluded from the verdict (never a negative); sample {path}"),
        };
    };
    let shape = top_level_shape(text);
    BrokerLine {
        broker: "alerce",
        code,
        read: Read::Excluded,
        note: format!(
            "the 200 body is a new surface, not the dead stub — its cone-membership schema is unmeasured (shape {shape:?}); the read stays pending, excluded from the verdict (never a negative); sample {path}"
        ),
    }
}

fn top_level_shape(text: &str) -> Vec<(String, usize)> {
    let Some(root) = parse_json(text) else {
        return Vec::new();
    };
    match root {
        JsonVal::Obj(map) => {
            let mut out: Vec<(String, usize)> = map
                .iter()
                .map(|(k, v)| {
                    let len = match v {
                        JsonVal::Arr(a) => a.len(),
                        JsonVal::Obj(o) => o.len(),
                        _ => 1,
                    };
                    (k.clone(), len)
                })
                .collect();
            out.sort();
            out
        }
        JsonVal::Arr(a) => vec![("[root array]".to_string(), a.len())],
        _ => Vec::new(),
    }
}

fn sources_coord(body: &[u8]) -> Option<(f64, f64)> {
    let Ok(text) = std::str::from_utf8(body) else {
        return None;
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        return None;
    };
    for r in &rows {
        let JsonVal::Obj(m) = r else {
            continue;
        };
        if let (Some(ra), Some(dec)) = (obj_f64(m, "r:ra"), obj_f64(m, "r:dec")) {
            return Some((ra, dec));
        }
    }
    None
}

fn resolve_object(id: &str) -> Option<(f64, f64, String, Vec<u8>)> {
    let payload = format!("{{\"diaObjectId\": \"{id}\"}}");
    let Some((code, body)) = curl_post(FINK_SOURCES, &payload) else {
        println!(
            "broker difference: diaObject {id} — the sources query did not answer (measured stall) — the object stays unplaced"
        );
        return None;
    };
    println!(
        "broker difference: diaObject {id} placed via Fink /api/v1/sources: HTTP {code}, {} bytes",
        body.len()
    );
    if code != "200" {
        return None;
    }
    let path = format!("{STATE_SAMPLE_DIR}/broker_diff_object_{id}_sources.json");
    save_sample(&path, &body);
    let Some((ra, dec)) = sources_coord(&body) else {
        println!(
            "broker difference: diaObject {id} — no sources row carries r:ra/r:dec — the object stays unplaced (pending); sample {path}"
        );
        return None;
    };
    Some((ra, dec, code, body))
}

struct Position {
    ra: f64,
    dec: f64,
    label: String,
}

fn broker_verdict_word(hits: usize, reachable: usize) -> &'static str {
    if hits >= VERDICT_BROKER_MIN {
        "sky"
    } else if hits == 1 && reachable >= VERDICT_BROKER_MIN {
        "pipeline"
    } else {
        "pending"
    }
}

fn verdict_counts(lines: &[BrokerLine]) -> (usize, usize) {
    let hits = lines.iter().filter(|l| l.read == Read::Present).count();
    let reachable = lines
        .iter()
        .filter(|l| l.read == Read::Present || l.read == Read::Absent)
        .count();
    (hits, reachable)
}

fn print_verdict_line(lines: &[BrokerLine]) -> usize {
    let (hits, reachable) = verdict_counts(lines);
    let word = broker_verdict_word(hits, reachable);
    println!(
        "  verdict: {word} — {} reachable broker(s) with a clean membership read, {hits} of them present",
        reachable
    );
    match word {
        "sky" => println!(
            "  the event survives the broker boundary: {hits} independent broker(s) carry it (>= {VERDICT_BROKER_MIN}) — it belongs to the sky, not to one reduction pipeline"
        ),
        "pipeline" => println!(
            "  the event dies at the broker boundary: exactly 1 of the {reachable} reachable broker(s) carries it while another reachable broker read nothing there — a reduction-artifact candidate, it belongs to the pipeline"
        ),
        _ => {
            if hits == 0 {
                println!(
                    "  no reachable broker read the event present — without a hit there is no broker-difference signal to adjudicate (0 honored); when a candidate event exists, re-run it here"
                );
            } else {
                println!(
                    "  the {VERDICT_BROKER_MIN}-broker floor of the sky/pipeline gate is open: {reachable} reachable broker(s) with {hits} hit(s); fewer than {VERDICT_BROKER_MIN} reachable brokers leaves the verdict pending, not negative"
                );
            }
        }
    }
    hits
}

fn run_position(p: &Position) -> usize {
    println!(
        "position {} at ra {:.6} dec {:.6} (label: {}) — cone radius {MATCH_RADIUS_ARCSEC} arcsec per broker",
        p.label, p.ra, p.dec, p.label
    );
    let lines = vec![
        fink_membership(p.ra, p.dec),
        lasair_membership(p.ra, p.dec),
        alerce_membership(p.ra, p.dec),
    ];
    for l in &lines {
        match l.read {
            Read::Present => {
                println!("  {}: present — HTTP {} — {}", l.broker, l.code, l.note)
            }
            Read::Absent => println!("  {}: absent — HTTP {} — {}", l.broker, l.code, l.note),
            Read::Excluded => {
                println!("  {}: unreachable({}) — {}", l.broker, l.code, l.note)
            }
        }
    }
    print_verdict_line(&lines)
}

fn usage() {
    eprintln!(
        "broker_difference_probe — Funke 3, the broker-difference verdict: an event in several\n\
         independent brokers belongs to the sky; an event in only one belongs to the pipeline.\n\
         Membership is asked per position from each broker independently:\n\
         \x20 Fink /conesearch (anonymous), Lasair /api/cone (token), ALeRCE /objects (anonymous).\n\
         \x20 every broker call is measured for reachability (HTTP code); an unreachable or blocked\n\
         \x20 broker is named absent/pending and EXCLUDED from the verdict — never a 'not present'.\n\
         verdict per position (three-valued): sky (>= 2 reachable brokers present) | pipeline\n\
         \x20 (exactly 1 reachable broker present, another reachable broker read nothing) | pending.\n\
         one or more positions or diaObjectId(s):\n\
         \x20 broker_difference_probe --pos <ra>,<dec> [--pos <ra>,<dec> ...]\n\
         \x20 broker_difference_probe --object <diaObjectId> [--object <diaObjectId> ...]\n\
         \x20   a diaObjectId is placed through Fink /api/v1/sources (ra/dec), then cone-searched.\n\
         Lasair: a direct cone read of 000 (or a stall) is retried once through the proton0\n\
         \x20 tunnel interface (measured 2026-09-06: direct 000, tunnel HTTP 200) and the tunnel\n\
         \x20 is named when it carries the read; ALeRCE api.alerce.online is the retired 404 stub\n\
         \x20 (excluded by design).\n\
         LAS token key for api.lasair.lsst.ac.uk: LASAIR_LSST_TOKEN (env or the .secrets.local\n\
         \x20 key in the omegaflow state dir)."
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut pos_list: Vec<(f64, f64)> = Vec::new();
    let mut object_list: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--pos" => {
                if let Some(spec) = args.get(i + 1) {
                    let p: Vec<&str> = spec.split(',').collect();
                    if p.len() == 2 {
                        if let (Some(ra), Some(dec)) = (
                            p[0].trim().parse::<f64>().ok(),
                            p[1].trim().parse::<f64>().ok(),
                        ) {
                            if ra.is_finite()
                                && dec.is_finite()
                                && (0.0..360.0).contains(&ra)
                                && (-90.0..=90.0).contains(&dec)
                            {
                                pos_list.push((ra, dec));
                                i += 1;
                            }
                        }
                    }
                }
            }
            "--object" => {
                if let Some(id) = args.get(i + 1) {
                    object_list.push(id.clone());
                    i += 1;
                }
            }
            _ => {
                usage();
                return;
            }
        }
        i += 1;
    }
    if pos_list.is_empty() && object_list.is_empty() {
        usage();
        return;
    }
    let mut hits_total = 0usize;
    for (k, (ra, dec)) in pos_list.iter().enumerate() {
        hits_total += run_position(&Position {
            ra: *ra,
            dec: *dec,
            label: format!("p{}", k + 1),
        });
    }
    for id in &object_list {
        match resolve_object(id) {
            Some((ra, dec, code, _)) => {
                println!(
                    "broker difference: diaObject {id} resolved to ra {ra:.6} dec {dec:.6} via Fink sources HTTP {code}"
                );
                hits_total += run_position(&Position {
                    ra,
                    dec,
                    label: format!("diaObject {id}"),
                });
            }
            None => {
                println!(
                    "broker difference: diaObject {id} — no position to cone-search; the membership test stays pending for the unplaced object (0 honored), never a negative"
                );
            }
        }
    }
    println!(
        "broker-difference survey: {hits_total} present read(s) across the probed position(s); every verdict is honest — pending where fewer than {VERDICT_BROKER_MIN} brokers answer, never a fabricated sky or pipeline"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_line(read: Read) -> BrokerLine {
        BrokerLine {
            broker: "broker",
            code: "200".to_string(),
            read,
            note: "test".to_string(),
        }
    }

    #[test]
    fn fink_cone_parser_reads_the_measured_conesearch_schema() {
        let body = br#"[{"r:dec":2.5208155633,"r:diaObjectId":313998569858662581,"r:midpointMjdTai":61205.9837394019,"r:ra":148.8746049917,"v:separation_degree":0.0000354169}]"#;
        assert_eq!(
            fink_cone_parse(body),
            ConeParse::Objects(1),
            "the measured conesearch row carries the diaObjectId as a JSON number"
        );
        let string_id = br#"[{"r:diaObjectId":"313998569858662581","r:ra":148.8745712297,"r:dec":2.5208047616}]"#;
        assert_eq!(fink_cone_parse(string_id), ConeParse::Objects(1));
        assert_eq!(fink_cone_parse(b"[]"), ConeParse::Empty);
        assert_eq!(fink_cone_parse(b"{}"), ConeParse::Unresolved);
        assert_eq!(
            fink_cone_parse(br#"[{"r:ra":1.0}]"#),
            ConeParse::Unresolved,
            "a row without r:diaObjectId is not the measured schema — never read as absent"
        );
        assert_eq!(fink_cone_parse(b"not json"), ConeParse::Unresolved);
    }

    #[test]
    fn lasair_cone_parser_reads_the_measured_response_object() {
        let body = br#"{"objects":[{"object":313998569858662581,"separation":0.12750748541524667}],"count":1,"nearest":{"object":313998569858662581,"separation":0.12750748541524667}}"#;
        assert_eq!(lasair_cone_parse(body), ConeParse::Objects(1));
        let string_id = br#"{"objects":[{"object":"313998569858662581","separation":0.5}],"count":1,"nearest":{}}"#;
        assert_eq!(lasair_cone_parse(string_id), ConeParse::Objects(1));
        assert_eq!(
            lasair_cone_parse(br#"{"objects":[],"count":0,"nearest":{}}"#),
            ConeParse::Empty
        );
        assert_eq!(
            lasair_cone_parse(br#"{"objects":[{"separation":0.5}],"count":1,"nearest":{}}"#),
            ConeParse::Unresolved,
            "a row without object is not the measured schema — never read as absent"
        );
        assert_eq!(lasair_cone_parse(b"{}"), ConeParse::Unresolved);
        assert_eq!(lasair_cone_parse(b"[]"), ConeParse::Unresolved);
    }

    #[test]
    fn sources_parser_recovers_the_coordinate_from_the_measured_row() {
        let body = br#"[{"r:diaObjectId":"313998569858662581","r:ra":148.8745712297,"r:dec":2.5208047616,"r:band":"r"}]"#;
        let (ra, dec) = sources_coord(body).unwrap();
        assert!((ra - 148.8745712297).abs() < 1e-12);
        assert!((dec - 2.5208047616).abs() < 1e-12);
        assert!(sources_coord(b"[]").is_none());
        assert!(sources_coord(b"{}").is_none());
    }

    #[test]
    fn three_present_reads_verdict_sky() {
        let lines = vec![
            read_line(Read::Present),
            read_line(Read::Present),
            read_line(Read::Present),
        ];
        let (hits, reachable) = verdict_counts(&lines);
        assert_eq!(hits, 3);
        assert_eq!(reachable, 3);
        assert_eq!(broker_verdict_word(hits, reachable), "sky");
    }

    #[test]
    fn two_present_brokers_clear_the_sky_gate_even_with_an_absent_broker() {
        let lines = vec![
            read_line(Read::Present),
            read_line(Read::Present),
            read_line(Read::Absent),
        ];
        let (hits, reachable) = verdict_counts(&lines);
        assert_eq!(broker_verdict_word(hits, reachable), "sky");
    }

    #[test]
    fn one_present_one_absent_verdicts_pipeline() {
        let lines = vec![read_line(Read::Present), read_line(Read::Absent)];
        let (hits, reachable) = verdict_counts(&lines);
        assert_eq!(hits, 1);
        assert_eq!(reachable, 2);
        assert_eq!(broker_verdict_word(hits, reachable), "pipeline");
    }

    #[test]
    fn unreachable_brokers_never_count_as_absent() {
        let mut excluded = read_line(Read::Excluded);
        excluded.code = "404".to_string();
        let lines = vec![
            read_line(Read::Present),
            excluded,
            read_line(Read::Excluded),
        ];
        let (hits, reachable) = verdict_counts(&lines);
        assert_eq!(
            reachable, 1,
            "the excluded brokers are not reachable negatives"
        );
        assert_eq!(hits, 1);
        assert_eq!(
            broker_verdict_word(hits, reachable),
            "pending",
            "with one reachable broker the gate stays open — excluded brokers mean pending, not pipeline and not sky"
        );
    }

    #[test]
    fn a_lone_reachable_present_stays_pending() {
        let lines = vec![
            read_line(Read::Present),
            read_line(Read::Excluded),
            read_line(Read::Excluded),
        ];
        let (hits, reachable) = verdict_counts(&lines);
        assert_eq!(reachable, 1);
        assert_eq!(broker_verdict_word(hits, reachable), "pending");
    }

    #[test]
    fn all_reachable_absent_verdicts_pending_not_a_negative() {
        let lines = vec![
            read_line(Read::Absent),
            read_line(Read::Absent),
            read_line(Read::Absent),
        ];
        let (hits, reachable) = verdict_counts(&lines);
        assert_eq!(hits, 0);
        assert_eq!(reachable, 3);
        assert_eq!(
            broker_verdict_word(hits, reachable),
            "pending",
            "a broker-difference detector speaks only where the event appears somewhere — all-absent is no sky/pipeline verdict"
        );
    }

    #[test]
    fn two_present_with_one_unreachable_verdicts_sky() {
        let lines = vec![
            read_line(Read::Present),
            read_line(Read::Present),
            read_line(Read::Excluded),
        ];
        let (hits, reachable) = verdict_counts(&lines);
        assert_eq!(hits, 2);
        assert_eq!(reachable, 2);
        assert_eq!(broker_verdict_word(hits, reachable), "sky");
    }

    const LAS_OBJECTS_BODY: &[u8] =
        br#"{"objects":[{"object":313998569858662581,"separation":0.5}],"count":1,"nearest":{}}"#;

    #[test]
    fn lasair_direct_000_tunnel_200_reads_present_through_the_tunnel() {
        let probe = |iface: Option<&str>| -> Option<(String, Vec<u8>)> {
            match iface {
                None => Some(("000".to_string(), Vec::new())),
                Some(_) => Some(("200".to_string(), LAS_OBJECTS_BODY.to_vec())),
            }
        };
        let line = lasair_membership_core(0.5, 1.5, None, |_| true, probe);
        assert_eq!(line.read, Read::Present);
        assert_eq!(line.code, "200");
        assert!(line.note.contains("direct route read 000"));
        assert!(line.note.contains("proton0 tunnel carried the 200"));
    }

    #[test]
    fn lasair_direct_000_without_the_tunnel_interface_stays_unreachable() {
        let probe = |iface: Option<&str>| -> Option<(String, Vec<u8>)> {
            match iface {
                None => Some(("000".to_string(), Vec::new())),
                Some(_) => Some(("200".to_string(), LAS_OBJECTS_BODY.to_vec())),
            }
        };
        let line = lasair_membership_core(0.5, 1.5, None, |_| false, probe);
        assert_eq!(line.read, Read::Excluded);
        assert_eq!(line.code, "000");
        assert!(line
            .note
            .contains("tunnel interface is not present (ip link)"));
        assert!(line.note.contains("no tunnel retry exists"));
    }

    #[test]
    fn lasair_direct_000_and_tunnel_000_stays_unreachable() {
        let probe = |iface: Option<&str>| -> Option<(String, Vec<u8>)> {
            match iface {
                None => Some(("000".to_string(), Vec::new())),
                Some(_) => Some(("000".to_string(), Vec::new())),
            }
        };
        let line = lasair_membership_core(0.5, 1.5, None, |_| true, probe);
        assert_eq!(line.read, Read::Excluded);
        assert_eq!(line.code, "000");
        assert!(line.note.contains("no route carries the cone"));
    }

    #[test]
    fn lasair_direct_200_reads_without_any_tunnel_call() {
        let probe = |iface: Option<&str>| -> Option<(String, Vec<u8>)> {
            assert!(
                iface.is_none(),
                "the tunnel is not attempted after a direct 200"
            );
            Some(("200".to_string(), LAS_OBJECTS_BODY.to_vec()))
        };
        let line = lasair_membership_core(0.5, 1.5, None, |_| true, probe);
        assert_eq!(line.read, Read::Present);
        assert!(!line.note.contains("tunnel"));
    }

    #[test]
    fn lasair_direct_http_error_is_a_broker_answer_not_a_route_stall() {
        let probe = |iface: Option<&str>| -> Option<(String, Vec<u8>)> {
            assert!(
                iface.is_none(),
                "a real HTTP answer is not retried over the tunnel"
            );
            Some(("500".to_string(), b"{}".to_vec()))
        };
        let line = lasair_membership_core(0.5, 1.5, None, |_| true, probe);
        assert_eq!(line.read, Read::Excluded);
        assert_eq!(line.code, "500");
        assert!(line.note.contains("answered HTTP 500"));
    }

    #[test]
    fn lasair_tunnel_200_with_an_empty_cone_reads_absent() {
        let probe = |iface: Option<&str>| -> Option<(String, Vec<u8>)> {
            match iface {
                None => Some(("000".to_string(), Vec::new())),
                Some(_) => Some((
                    "200".to_string(),
                    br#"{"objects":[],"count":0,"nearest":{}}"#.to_vec(),
                )),
            }
        };
        let line = lasair_membership_core(0.5, 1.5, None, |_| true, probe);
        assert_eq!(line.read, Read::Absent);
        assert!(line.note.contains("proton0 tunnel carried the 200"));
        assert!(line.note.contains("0 honored"));
    }
}
