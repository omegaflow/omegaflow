use std::fs;

const REC: usize = 2666;
const HDR: usize = 166;
const MAX_SAMPLE: usize = 3000;
const TRIO: [u8; 7] = [14, 42, 43, 61, 63, 65, 85];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 || (args.len() - 1) % 2 != 0 {
        eprintln!("galileo ODR header census probe: <report path> then <odr> <station> [...] (2 tokens per file)");
        return;
    }
    let report_path = &args[0];
    let mut out: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        let path = &args[i];
        let Ok(station) = args[i + 1].parse::<u8>() else {
            out.push(format!("{path}: station token not numeric"));
            i += 2;
            continue;
        };
        census(&mut out, path, station);
        i += 2;
    }
    if let Err(e) = fs::write(report_path, out.join("\n") + "\n") {
        eprintln!("report write void: {e}");
    }
}

fn census(out: &mut Vec<String>, path: &str, station: u8) {
    let Ok(bytes) = fs::read(path) else {
        out.push(format!("{path}: read void"));
        return;
    };
    let nrec = bytes.len() / REC;
    if nrec == 0 {
        out.push(format!("{path}: no records"));
        return;
    }
    let mut dist: Vec<[u8; 32]> = vec![[0u8; 32]; HDR];
    let step = if nrec > MAX_SAMPLE { nrec / MAX_SAMPLE } else { 1 };
    let mut sampled = 0usize;
    let mut k = 0usize;
    while k < nrec {
        let r = &bytes[k * REC..(k + 1) * REC];
        for (o, d) in dist.iter_mut().enumerate() {
            let v = r[o];
            d[(v >> 3) as usize] |= 1u8 << (v & 7);
        }
        sampled += 1;
        k += step;
    }

    let rec0 = &bytes[0..REC];
    let recn = &bytes[(nrec - 1) * REC..nrec * REC];
    out.push(format!(
        "{path}: station {station}, {nrec} records, {sampled} sampled (stride {step})"
    ));
    let ms0 = u32::from_be_bytes([rec0[12], rec0[13], rec0[14], rec0[15]]) & 0x07FF_FFFF;
    let msn = u32::from_be_bytes([recn[12], recn[13], recn[14], recn[15]]) & 0x07FF_FFFF;
    out.push(format!(
        "  time words 7-8: first {:.3} h, last {:.3} h",
        ms0 as f64 / 3.6e6,
        msn as f64 / 3.6e6
    ));
    out.push(format!(
        "  word 4 bytes 6-7 (prime/secondary FEA): first {} / {}, last {} / {}",
        rec0[6], rec0[7], recn[6], recn[7]
    ));
    out.push(format!(
        "  word 5 bytes 8-9 (spacecraft/code): {} / {}",
        rec0[8], rec0[9]
    ));
    out.push(format!(
        "  word 26 bytes 50-51 (antenna RF config): {:#04x} / {:#04x}",
        rec0[50], rec0[51]
    ));
    let sr = (rec0[158] as u16) << 8 | rec0[159] as u16;
    out.push(format!("  word 80 bytes 158-159 (A-D sample rate): {sr} sps"));

    let mut consts: Vec<String> = Vec::new();
    let mut varying_trio: Vec<String> = Vec::new();
    let mut mode_like: Vec<String> = Vec::new();
    for (o, d) in dist.iter().enumerate() {
        let vals: Vec<u8> = (0u16..256)
            .filter(|v| d[(v >> 3) as usize] & (1u8 << (v & 7)) != 0)
            .map(|v| v as u8)
            .collect();
        if vals.is_empty() {
            continue;
        }
        if vals.len() == 1 {
            consts.push(format!("const byte {o} = {}", vals[0]));
            continue;
        }
        if vals.iter().all(|v| (1..=3).contains(v)) {
            mode_like.push(format!("byte {o}: values {vals:?} (subset of {{1,2,3}})"));
        }
        let trio_vals: Vec<u8> = vals.iter().cloned().filter(|v| TRIO.contains(v)).collect();
        if !trio_vals.is_empty() {
            varying_trio.push(format!(
                "byte {o}: {vals:?} (trio-looking values {trio_vals:?} among varying bytes)"
            ));
        }
    }

    let recv_const: Vec<&str> = consts
        .iter()
        .filter(|s| s.ends_with(&format!("= {station}")))
        .map(|s| s.as_str())
        .collect();
    let second_const: Vec<&str> = consts
        .iter()
        .filter(|s| {
            if s.ends_with(&format!("= {station}")) {
                return false;
            }
            TRIO.iter().any(|t| s.ends_with(&format!("= {t}")))
        })
        .map(|s| s.as_str())
        .collect();
    out.push(format!(
        "  constant (single-valued) header bytes: {} of {HDR}",
        consts.len()
    ));
    if recv_const.is_empty() {
        out.push("  receiver-station constant byte: none".to_string());
    } else {
        out.push(format!("  receiver-station constant bytes: {recv_const:?}"));
    }
    if second_const.is_empty() {
        out.push(
            "  constant station-like byte distinct from the receiver: none (no second/transmitting station field)".to_string(),
        );
    } else {
        out.push(format!("  constant station-like byte distinct from the receiver: {second_const:?}"));
    }
    if mode_like.is_empty() {
        out.push("  no varying byte is restricted to {1,2,3}".to_string());
    } else {
        out.push(format!("  varying bytes restricted to {{1,2,3}}: {mode_like:?}"));
    }
    if varying_trio.is_empty() {
        out.push("  no varying byte carries trio-looking values".to_string());
    } else {
        out.push("  varying bytes carrying trio-looking values (spurious coincidences expected):".to_string());
        for s in &varying_trio {
            out.push(format!("    {s}"));
        }
    }
}
