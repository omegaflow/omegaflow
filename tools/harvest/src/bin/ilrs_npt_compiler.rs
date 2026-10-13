use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::collections::BTreeSet;

const NETLOC: &str = "edc.dgfi.tum.de";
const COMPILER: &str = "tools/harvest/src/bin/ilrs_npt_compiler.rs";
const FORMAT: &str = "ilrs_npt";

const MAGIC: [u8; 4] = *b"ILNP";
const HEADER_BYTES: usize = 8;
const RECORD_BYTES: usize = 108;

const P_EPOCH: u32 = 1 << 0;
const P_SEC: u32 = 1 << 1;
const P_TOF: u32 = 1 << 2;
const P_WINDOW: u32 = 1 << 3;
const P_RMS: u32 = 1 << 4;
const P_SKEW: u32 = 1 << 5;
const P_KURT: u32 = 1 << 6;
const P_MEAN: u32 = 1 << 7;
const P_RATE: u32 = 1 << 8;
const P_NUM_RANGES: u32 = 1 << 9;
const P_EPOCH_EVENT: u32 = 1 << 10;
const P_DETECTOR: u32 = 1 << 11;

const FLAG_NO_SESSION: u32 = 1 << 0;
const FLAG_NO_TIME: u32 = 1 << 1;
const FLAG_TOF_RANGE: u32 = 1 << 2;
const FLAG_TIME_RANGE: u32 = 1 << 3;
const FLAG_TIME_ORDER: u32 = 1 << 4;
const FLAG_NO_STATION: u32 = 1 << 5;
const FLAG_NO_TARGET: u32 = 1 << 6;
const FLAG_RANGE_TYPE_UNKNOWN: u32 = 1 << 7;

const TOF_HI_S: f64 = 10.0;
const EPOCH_LO_UTC: f64 = -31_536_000.0;
const EPOCH_HI_UTC: f64 = 4_133_980_800.0;
const RANGE_TYPE_TWO_WAY: u32 = 2;
const RANGE_TYPE_MIXED: u32 = 4;

const SAMPLE: &str = "h1 CRD  1 2026  1 22  4\n\
h2 ZELL       1889 99 01  4\n\
h3 beaconc     6503201 0317     1328 0 1\n\
h4  1 2026  1 21 14 34 58 2026  1 21 14 44  4  0 0 0 0 1 0 2 0\n\
11 52508.739753250004 0.015013270763 std 2 15.0 279 249.8 0.212 -0.475 -1.0 -1.0 0\n\
h8\n\
H9\n";

#[derive(Clone, Copy, Debug, PartialEq)]
struct Npt {
    epoch_utc: Option<f64>,
    sec_of_day: Option<f64>,
    round_trip_s: Option<f64>,
    np_window_s: Option<f64>,
    bin_rms_ps: Option<f64>,
    bin_skew: Option<f64>,
    bin_kurtosis: Option<f64>,
    bin_mean_ps: Option<f64>,
    return_rate_pct: Option<f64>,
    num_ranges: Option<u32>,
    epoch_event: Option<i32>,
    detector_channel: Option<i32>,
    config_id: [u8; 4],
    station_cdp: u32,
    target_ilrs_id: u32,
    range_type: u32,
    flags: u32,
}

fn meas_f64(tok: &str) -> Option<f64> {
    if tok.eq_ignore_ascii_case("na") || tok.eq_ignore_ascii_case("nan") {
        return None;
    }
    let v: f64 = tok.parse().ok()?;
    v.is_finite().then_some(v)
}

fn signed_f64(tok: &str) -> Option<f64> {
    meas_f64(tok)
}

fn nonneg_f64(tok: &str) -> Option<f64> {
    let v = meas_f64(tok)?;
    (v >= 0.0).then_some(v)
}

fn positive_f64(tok: &str) -> Option<f64> {
    let v = meas_f64(tok)?;
    (v > 0.0 && v < TOF_HI_S).then_some(v)
}

fn meas_u32(tok: &str) -> Option<u32> {
    if tok.eq_ignore_ascii_case("na") {
        return None;
    }
    tok.parse().ok()
}

fn meas_i32(tok: &str) -> Option<i32> {
    if tok.eq_ignore_ascii_case("na") {
        return None;
    }
    tok.parse().ok()
}

fn sec_of_day_f64(tok: &str) -> Option<f64> {
    let v = meas_f64(tok)?;
    (0.0..86400.0).contains(&v).then_some(v)
}

fn config_bytes(tok: &str) -> [u8; 4] {
    let mut out = [0u8; 4];
    for (i, b) in tok.bytes().take(4).enumerate() {
        out[i] = b;
    }
    out
}

fn toks_i64(toks: &[&str], i: usize) -> Option<i64> {
    toks.get(i).and_then(|t| t.parse::<i64>().ok())
}

fn civil_date(toks: &[&str], base: usize) -> Option<(i64, i64, i64)> {
    match (
        toks_i64(toks, base),
        toks_i64(toks, base + 1),
        toks_i64(toks, base + 2),
    ) {
        (Some(y), Some(m), Some(d)) => Some((y, m, d)),
        _ => None,
    }
}

fn start_sec_of_day(toks: &[&str]) -> Option<f64> {
    let h = meas_f64(toks.get(5).copied().unwrap_or(""))?;
    let m = meas_f64(toks.get(6).copied().unwrap_or(""))?;
    let s = meas_f64(toks.get(7).copied().unwrap_or(""))?;
    Some(h * 3600.0 + m * 60.0 + s)
}

fn parse_crd(bytes: &[u8]) -> Option<Vec<Npt>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out: Vec<Npt> = Vec::new();
    let mut saw_header = false;
    let mut station_cdp = 0u32;
    let mut target_ilrs_id = 0u32;
    let mut session: Option<(i64, i64, i64)> = None;
    let mut session_end: Option<(i64, i64, i64)> = None;
    let mut start_sod: Option<f64> = None;
    let mut range_type = 0u32;
    let mut last_epoch: Option<f64> = None;

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        match toks[0].to_ascii_lowercase().as_str() {
            "h1" => {
                if !toks
                    .get(1)
                    .map(|t| t.eq_ignore_ascii_case("crd"))
                    .unwrap_or(false)
                {
                    return None;
                }
                match toks.get(2).and_then(|t| t.parse::<i32>().ok()) {
                    Some(1) | Some(2) => saw_header = true,
                    _ => return None,
                }
            }
            "h2" => {
                if let Some(id) = toks.get(2).and_then(|t| t.parse::<u32>().ok()) {
                    station_cdp = id;
                }
            }
            "h3" => {
                if let Some(id) = toks.get(2).and_then(|t| t.parse::<u32>().ok()) {
                    target_ilrs_id = id;
                }
            }
            "h4" => {
                session = civil_date(&toks, 2);
                session_end = civil_date(&toks, 8);
                start_sod = start_sec_of_day(&toks);
                if let Some(rt) = toks.get(20).and_then(|t| t.parse::<u32>().ok()) {
                    range_type = rt;
                }
            }
            "11" => {
                let sec_of_day = toks.get(1).and_then(|t| sec_of_day_f64(t));
                let round_trip_s = toks.get(2).and_then(|t| positive_f64(t));
                let config_id = config_bytes(toks.get(3).copied().unwrap_or(""));
                let epoch_event = toks.get(4).and_then(|t| meas_i32(t));
                let np_window_s = toks.get(5).and_then(|t| nonneg_f64(t));
                let num_ranges = toks.get(6).and_then(|t| meas_u32(t));
                let bin_rms_ps = toks.get(7).and_then(|t| nonneg_f64(t));
                let bin_skew = toks.get(8).and_then(|t| signed_f64(t));
                let bin_kurtosis = toks.get(9).and_then(|t| signed_f64(t));
                let bin_mean_ps = toks.get(10).and_then(|t| signed_f64(t));
                let return_rate_pct = toks.get(11).and_then(|t| nonneg_f64(t));
                let detector_channel = toks.get(12).and_then(|t| meas_i32(t));

                let mut flags = 0u32;
                if session.is_none() {
                    flags |= FLAG_NO_SESSION;
                }
                if station_cdp == 0 {
                    flags |= FLAG_NO_STATION;
                }
                if target_ilrs_id == 0 {
                    flags |= FLAG_NO_TARGET;
                }
                if range_type != RANGE_TYPE_TWO_WAY && range_type != RANGE_TYPE_MIXED {
                    flags |= FLAG_RANGE_TYPE_UNKNOWN;
                }

                let epoch_utc = match (session, session_end, start_sod, sec_of_day) {
                    (Some(s), end, Some(sod0), Some(sod)) => {
                        let day = if sod < sod0 {
                            end.unwrap_or(s)
                        } else {
                            s
                        };
                        days_from_civil(day.0, day.1, day.2)
                            .map(|days| days as f64 * 86400.0 + sod)
                    }
                    _ => None,
                };
                if sec_of_day.is_none() {
                    flags |= FLAG_NO_TIME;
                }
                if let Some(e) = epoch_utc {
                    if !(EPOCH_LO_UTC..EPOCH_HI_UTC).contains(&e) {
                        flags |= FLAG_TIME_RANGE;
                    }
                    if let Some(prev) = last_epoch
                        && e < prev
                    {
                        flags |= FLAG_TIME_ORDER;
                    }
                    last_epoch = Some(match last_epoch {
                        Some(p) if p > e => p,
                        _ => e,
                    });
                }
                if toks.get(2).and_then(|t| meas_f64(t)).is_some() && round_trip_s.is_none() {
                    flags |= FLAG_TOF_RANGE;
                }

                out.push(Npt {
                    epoch_utc,
                    sec_of_day,
                    round_trip_s,
                    np_window_s,
                    bin_rms_ps,
                    bin_skew,
                    bin_kurtosis,
                    bin_mean_ps,
                    return_rate_pct,
                    num_ranges,
                    epoch_event,
                    detector_channel,
                    config_id,
                    station_cdp,
                    target_ilrs_id,
                    range_type,
                    flags,
                });
            }
            _ => {}
        }
    }
    if !saw_header || out.is_empty() {
        return None;
    }
    Some(out)
}

fn put_f64(out: &mut Vec<u8>, v: Option<f64>, bit: u32, present: &mut u32) -> bool {
    match v {
        Some(x) => {
            if !x.is_finite() {
                return false;
            }
            *present |= bit;
            out.extend_from_slice(&x.to_le_bytes());
        }
        None => out.extend_from_slice(&0.0f64.to_le_bytes()),
    }
    true
}

fn put_u32(out: &mut Vec<u8>, v: Option<u32>, bit: u32, present: &mut u32) {
    match v {
        Some(x) => {
            *present |= bit;
            out.extend_from_slice(&x.to_le_bytes());
        }
        None => out.extend_from_slice(&0u32.to_le_bytes()),
    }
}

fn put_i32(out: &mut Vec<u8>, v: Option<i32>, bit: u32, present: &mut u32) {
    match v {
        Some(x) => {
            *present |= bit;
            out.extend_from_slice(&x.to_le_bytes());
        }
        None => out.extend_from_slice(&0i32.to_le_bytes()),
    }
}

fn write_bin(records: &[Npt]) -> Option<Vec<u8>> {
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        let mut present = 0u32;
        if !put_f64(&mut out, r.epoch_utc, P_EPOCH, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.sec_of_day, P_SEC, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.round_trip_s, P_TOF, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.np_window_s, P_WINDOW, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.bin_rms_ps, P_RMS, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.bin_skew, P_SKEW, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.bin_kurtosis, P_KURT, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.bin_mean_ps, P_MEAN, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.return_rate_pct, P_RATE, &mut present) {
            return None;
        }
        put_u32(&mut out, r.num_ranges, P_NUM_RANGES, &mut present);
        put_i32(&mut out, r.epoch_event, P_EPOCH_EVENT, &mut present);
        put_i32(&mut out, r.detector_channel, P_DETECTOR, &mut present);
        out.extend_from_slice(&r.station_cdp.to_le_bytes());
        out.extend_from_slice(&r.target_ilrs_id.to_le_bytes());
        out.extend_from_slice(&r.range_type.to_le_bytes());
        out.extend_from_slice(&r.flags.to_le_bytes());
        out.extend_from_slice(&present.to_le_bytes());
        out.extend_from_slice(&r.config_id);
    }
    Some(out)
}

struct Reader<'a> {
    bytes: &'a [u8],
    off: usize,
}

impl Reader<'_> {
    fn f64(&mut self) -> Option<f64> {
        let v = f64::from_le_bytes(self.bytes.get(self.off..self.off + 8)?.try_into().ok()?);
        self.off += 8;
        Some(v)
    }

    fn u32(&mut self) -> Option<u32> {
        let v = u32::from_le_bytes(self.bytes.get(self.off..self.off + 4)?.try_into().ok()?);
        self.off += 4;
        Some(v)
    }

    fn i32(&mut self) -> Option<i32> {
        let v = i32::from_le_bytes(self.bytes.get(self.off..self.off + 4)?.try_into().ok()?);
        self.off += 4;
        Some(v)
    }

    fn four(&mut self) -> Option<[u8; 4]> {
        let v: [u8; 4] = self.bytes.get(self.off..self.off + 4)?.try_into().ok()?;
        self.off += 4;
        Some(v)
    }
}

fn present_f64(raw: f64, present: u32, bit: u32) -> Option<Option<f64>> {
    if present & bit != 0 {
        if raw.is_finite() {
            Some(Some(raw))
        } else {
            None
        }
    } else if raw == 0.0 {
        Some(None)
    } else {
        None
    }
}

fn present_u32(raw: u32, present: u32, bit: u32) -> Option<Option<u32>> {
    if present & bit != 0 {
        Some(Some(raw))
    } else if raw == 0 {
        Some(None)
    } else {
        None
    }
}

fn present_i32(raw: i32, present: u32, bit: u32) -> Option<Option<i32>> {
    if present & bit != 0 {
        Some(Some(raw))
    } else if raw == 0 {
        Some(None)
    } else {
        None
    }
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<Npt>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(RECORD_BYTES)? {
        return None;
    }
    let mut rd = Reader {
        bytes,
        off: HEADER_BYTES,
    };
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let epoch_raw = rd.f64()?;
        let sec_raw = rd.f64()?;
        let tof_raw = rd.f64()?;
        let window_raw = rd.f64()?;
        let rms_raw = rd.f64()?;
        let skew_raw = rd.f64()?;
        let kurt_raw = rd.f64()?;
        let mean_raw = rd.f64()?;
        let rate_raw = rd.f64()?;
        let num_ranges_raw = rd.u32()?;
        let epoch_event_raw = rd.i32()?;
        let detector_raw = rd.i32()?;
        let station_cdp = rd.u32()?;
        let target_ilrs_id = rd.u32()?;
        let range_type = rd.u32()?;
        let flags = rd.u32()?;
        let present = rd.u32()?;
        let config_id = rd.four()?;

        out.push(Npt {
            epoch_utc: present_f64(epoch_raw, present, P_EPOCH)?,
            sec_of_day: present_f64(sec_raw, present, P_SEC)?,
            round_trip_s: present_f64(tof_raw, present, P_TOF)?,
            np_window_s: present_f64(window_raw, present, P_WINDOW)?,
            bin_rms_ps: present_f64(rms_raw, present, P_RMS)?,
            bin_skew: present_f64(skew_raw, present, P_SKEW)?,
            bin_kurtosis: present_f64(kurt_raw, present, P_KURT)?,
            bin_mean_ps: present_f64(mean_raw, present, P_MEAN)?,
            return_rate_pct: present_f64(rate_raw, present, P_RATE)?,
            num_ranges: present_u32(num_ranges_raw, present, P_NUM_RANGES)?,
            epoch_event: present_i32(epoch_event_raw, present, P_EPOCH_EVENT)?,
            detector_channel: present_i32(detector_raw, present, P_DETECTOR)?,
            config_id,
            station_cdp,
            target_ilrs_id,
            range_type,
            flags,
        });
    }
    Some(out)
}

fn measured(records: Vec<Npt>) -> (Vec<Npt>, usize) {
    let total = records.len();
    let named: Vec<Npt> = records
        .into_iter()
        .filter(|r| r.epoch_utc.is_some() && r.round_trip_s.is_some())
        .collect();
    let held = total - named.len();
    (named, held)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let input = arg_value(args, "--input")
        .ok_or_else(|| "--input <crd.txt|.npt> absent — the file stays unread".to_string())?;
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{FORMAT}.bin"),
    };

    let bytes = std::fs::read(&input).map_err(|e| format!("{input}: read void ({e})"))?;
    let records = parse_crd(&bytes)
        .ok_or_else(|| format!("{input}: carries no CRD normal point — refused"))?;
    let (mut named, held) = measured(records);
    if held > 0 {
        eprintln!(
            "{input}: {held} normal points carry no finite epoch/round-trip time — held out, named"
        );
    }
    if named.is_empty() {
        return Err(format!(
            "{input}: no normal point carries a finite epoch and round-trip time — the bin stays unwritten (0 honored)"
        ));
    }
    named.sort_by(|a, b| match (a.epoch_utc, b.epoch_utc) {
        (Some(x), Some(y)) => x.total_cmp(&y),
        _ => std::cmp::Ordering::Equal,
    });

    let bin = write_bin(&named)
        .ok_or_else(|| "a held value is not finite — the bin stays unwritten".to_string())?;
    match parse_bin(&bin) {
        Some(parsed) if parsed == named => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;

    let stations: BTreeSet<u32> = named.iter().map(|r| r.station_cdp).collect();
    let targets: BTreeSet<u32> = named.iter().map(|r| r.target_ilrs_id).collect();
    let marked = named.iter().filter(|r| r.flags != 0).count();

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {input}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} normal points, stations {stations:?}, targets {targets:?}, {marked} marked, {} B, roundtrip parses",
        named.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let Some(records) = parse_crd(SAMPLE.as_bytes()) else {
        eprintln!("selftest: the sample CRD does not parse");
        std::process::exit(1);
    };
    if records.len() != 1 {
        eprintln!("selftest: the sample does not carry exactly one normal point");
        std::process::exit(1);
    }
    let r = records[0];
    if r.station_cdp != 1889 || r.target_ilrs_id != 6503201 {
        eprintln!("selftest: the sample's station/target do not match the measurement");
        std::process::exit(1);
    }
    if r.round_trip_s != Some(0.015013270763) || r.num_ranges != Some(279) {
        eprintln!("selftest: the sample's round-trip time / range count do not match");
        std::process::exit(1);
    }
    let Some(bin) = write_bin(&records) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    if bin.len() != HEADER_BYTES + RECORD_BYTES {
        eprintln!("selftest: the bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if parse_bin(&bin) != Some(records.clone()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    eprintln!(
        "ilrs_npt_compiler: selftest passes (CRD normal points -> epoch/round-trip series)"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: ilrs_npt_compiler --input <crd.npt> [--out <file.bin>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("ilrs_npt_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_roundtrips_the_measured_stride() {
        let records = parse_crd(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(bin.len(), HEADER_BYTES + RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(records));
    }

    #[test]
    fn parse_crd_refuses_foreign_bytes() {
        assert_eq!(parse_crd(b"not a crd file"), None);
        assert_eq!(parse_crd(b""), None);
        let wrong_literal = SAMPLE.replace("h1 CRD", "h1 XYZ");
        assert_eq!(parse_crd(wrong_literal.as_bytes()), None);
    }

    #[test]
    fn absent_nonnegative_fields_stay_absent() {
        let records = parse_crd(SAMPLE.as_bytes()).expect("the sample parses");
        assert_eq!(records[0].return_rate_pct, None);
        assert_eq!(records[0].bin_mean_ps, Some(-1.0));
        assert_eq!(records[0].epoch_event, Some(2));
        assert_eq!(records[0].detector_channel, Some(0));
    }
}
