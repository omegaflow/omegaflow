use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::nexrad::{
    NexradMoment, NexradSite, NexradVolume, nexrad_site, parse_nexrad,
};
use omegaflow::cdn::upload_release;
use std::io::{BufWriter, Write};

const NETLOC: &str = "unidata-nexrad-level2.s3.amazonaws.com";
const MAGIC: [u8; 4] = *b"NXR1";
const REC_BYTES: usize = 44;
const SITE_BYTES: usize = 1 + 4 + 8 + 8 + 8;
const HEADER_BYTES: usize = 8 + SITE_BYTES;

const KIND_REF: u32 = 1;
const KIND_VEL: u32 = 2;
const KIND_SW: u32 = 3;

struct NexradSample {
    t: f64,
    az_deg: f64,
    el_deg: f64,
    range_km: f64,
    value: f64,
    kind: u32,
}

struct NexradAsset {
    site: Option<NexradSite>,
    samples: Vec<NexradSample>,
}

fn volume_unix(date: u32, time_ms: u32) -> f64 {
    (date as f64 - 1.0) * 86400.0 + time_ms as f64 / 1000.0
}

fn kind_of(name: &[u8; 3]) -> Option<u32> {
    match name {
        b"REF" => Some(KIND_REF),
        b"VEL" => Some(KIND_VEL),
        b"SW " => Some(KIND_SW),
        b"SW\0" => Some(KIND_SW),
        _ => None,
    }
}

fn kind_name(kind: u32) -> &'static str {
    match kind {
        KIND_REF => "REF",
        KIND_VEL => "VEL",
        KIND_SW => "SW",
        _ => "unknown",
    }
}

fn physical_value(raw: u16, moment: &NexradMoment) -> Option<f64> {
    if raw == 0 || raw == 1 {
        return None;
    }
    let v = (raw as f64 - moment.offset as f64) / moment.scale as f64;
    if v.is_finite() { Some(v) } else { None }
}

fn collect(volume: &NexradVolume) -> Result<Vec<NexradSample>, String> {
    let (date, time_ms) = match (volume.date, volume.time_ms) {
        (Some(d), Some(t)) => (d, t),
        _ => {
            return Err(
                "the volume carries no clock (date/time absent) — the bin stays unwritten (0 honored)"
                    .to_string(),
            );
        }
    };
    let t = volume_unix(date, time_ms);
    let mut samples = Vec::new();
    let mut skipped_sentinel = 0usize;
    let mut skipped_moment = 0usize;
    for radial in &volume.radials {
        for moment in &radial.moments {
            let Some(kind) = kind_of(&moment.name) else {
                skipped_moment += moment.num_gates as usize;
                continue;
            };
            for (g, raw) in moment.values.iter().enumerate() {
                let Some(value) = physical_value(*raw, moment) else {
                    skipped_sentinel += 1;
                    continue;
                };
                let range_km = moment.first_gate_km as f64 + g as f64 * moment.gate_width_km as f64;
                samples.push(NexradSample {
                    t,
                    az_deg: radial.az_angle_deg as f64,
                    el_deg: radial.el_angle_deg as f64,
                    range_km,
                    value,
                    kind,
                });
            }
        }
    }
    if samples.is_empty() {
        return Err(
            "no measured gate — every gate is absent, range-folded, or a foreign moment (0 honored)"
                .to_string(),
        );
    }
    eprintln!(
        "nexrad: {} samples, {} absent/range-folded gates skipped, {} foreign-moment gates skipped",
        samples.len(),
        skipped_sentinel,
        skipped_moment
    );
    Ok(samples)
}

fn report_value_ranges(samples: &[NexradSample]) {
    for kind in [KIND_REF, KIND_VEL, KIND_SW] {
        let vals: Vec<f64> = samples
            .iter()
            .filter(|s| s.kind == kind)
            .map(|s| s.value)
            .collect();
        if vals.is_empty() {
            continue;
        }
        let min = vals.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = vals.iter().sum::<f64>() / vals.len() as f64;
        eprintln!(
            "{}: {} samples, value min {:.3} max {:.3} mean {:.3}",
            kind_name(kind),
            vals.len(),
            min,
            max,
            mean
        );
    }
}

fn report_geometry(volume: &NexradVolume) {
    let stid = match volume
        .stid
        .map(|s| String::from_utf8_lossy(&s).into_owned())
    {
        Some(v) => v,
        None => "absent".to_string(),
    };
    eprintln!("site {stid}");

    let mut seen_moments: Vec<&NexradMoment> = Vec::new();
    for radial in &volume.radials {
        for moment in &radial.moments {
            if !seen_moments.iter().any(|m| m.name == moment.name) {
                seen_moments.push(moment);
            }
        }
    }
    for moment in seen_moments {
        eprintln!(
            "moment {}: {} gates, first gate {:.3} km, gate width {:.3} m",
            String::from_utf8_lossy(&moment.name),
            moment.num_gates,
            moment.first_gate_km,
            moment.gate_width_km * 1000.0
        );
    }

    let mut deltas: Vec<f64> = Vec::new();
    for w in volume.radials.windows(2) {
        if w[0].el_num == w[1].el_num {
            let mut d = w[1].az_angle_deg as f64 - w[0].az_angle_deg as f64;
            if d < 0.0 {
                d += 360.0;
            }
            if d > 0.0 && d < 180.0 {
                deltas.push(d);
            }
        }
    }
    if deltas.is_empty() {
        eprintln!("azimuth resolution: absent (fewer than two radials per scan)");
    } else {
        deltas.sort_by(|a, b| a.total_cmp(b));
        let mid = deltas[deltas.len() / 2];
        let min = deltas[0];
        let max = deltas[deltas.len() - 1];
        eprintln!(
            "azimuth step: min {:.4} deg, median {:.4} deg, max {:.4} deg ({} deltas)",
            min,
            mid,
            max,
            deltas.len()
        );
    }

    let mut el_nums: Vec<u8> = volume.radials.iter().map(|r| r.el_num).collect();
    el_nums.sort_unstable();
    el_nums.dedup();
    eprintln!("elevation scans: {} ({:?})", el_nums.len(), el_nums);
}

fn write_asset(asset: &NexradAsset, out_path: &str) -> Result<usize, String> {
    let file = std::fs::File::create(out_path).map_err(|e| format!("create {out_path}: {e}"))?;
    let mut out = BufWriter::new(file);
    out.write_all(&MAGIC)
        .map_err(|e| format!("write {out_path}: {e}"))?;
    out.write_all(&(asset.samples.len() as u32).to_le_bytes())
        .map_err(|e| format!("write {out_path}: {e}"))?;
    let mut site = [0u8; SITE_BYTES];
    if let Some(s) = &asset.site {
        site[0] = 1;
        site[1..5].copy_from_slice(&s.stid);
        site[5..13].copy_from_slice(&s.lat_deg.to_le_bytes());
        site[13..21].copy_from_slice(&s.lon_deg.to_le_bytes());
        site[21..29].copy_from_slice(&s.alt_m.to_le_bytes());
    }
    out.write_all(&site)
        .map_err(|e| format!("write {out_path}: {e}"))?;
    let mut rec = [0u8; REC_BYTES];
    for s in &asset.samples {
        rec[0..8].copy_from_slice(&s.t.to_le_bytes());
        rec[8..16].copy_from_slice(&s.az_deg.to_le_bytes());
        rec[16..24].copy_from_slice(&s.el_deg.to_le_bytes());
        rec[24..32].copy_from_slice(&s.range_km.to_le_bytes());
        rec[32..40].copy_from_slice(&s.value.to_le_bytes());
        rec[40..44].copy_from_slice(&s.kind.to_le_bytes());
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path}: {e}"))?;
    }
    out.flush().map_err(|e| format!("flush {out_path}: {e}"))?;
    let expect = HEADER_BYTES + asset.samples.len() * REC_BYTES;
    let actual = std::fs::metadata(out_path)
        .map_err(|e| format!("stat {out_path}: {e}"))?
        .len() as usize;
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }
    Ok(expect)
}

fn parse_asset(bytes: &[u8]) -> Option<NexradAsset> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * REC_BYTES {
        return None;
    }
    let site = match bytes[8] {
        0 => None,
        1 => {
            let stid = bytes[9..13].try_into().ok()?;
            let lat_deg = f64::from_le_bytes(bytes[13..21].try_into().ok()?);
            let lon_deg = f64::from_le_bytes(bytes[21..29].try_into().ok()?);
            let alt_m = f64::from_le_bytes(bytes[29..37].try_into().ok()?);
            if !lat_deg.is_finite() || !lon_deg.is_finite() || !alt_m.is_finite() {
                return None;
            }
            Some(NexradSite {
                stid,
                lat_deg,
                lon_deg,
                alt_m,
            })
        }
        _ => return None,
    };
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    let f64_of = |b: &[u8], r: std::ops::Range<usize>| {
        b.get(r)
            .and_then(|x| x.try_into().ok())
            .map(f64::from_le_bytes)
    };
    for _ in 0..n {
        let s = bytes.get(off..off + REC_BYTES)?;
        let t = f64_of(s, 0..8)?;
        let az_deg = f64_of(s, 8..16)?;
        let el_deg = f64_of(s, 16..24)?;
        let range_km = f64_of(s, 24..32)?;
        let value = f64_of(s, 32..40)?;
        let kind = u32::from_le_bytes(s[40..44].try_into().ok()?);
        if !t.is_finite()
            || !az_deg.is_finite()
            || !el_deg.is_finite()
            || !range_km.is_finite()
            || !value.is_finite()
        {
            return None;
        }
        out.push(NexradSample {
            t,
            az_deg,
            el_deg,
            range_km,
            value,
            kind,
        });
        off += REC_BYTES;
    }
    Some(NexradAsset { site, samples: out })
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: nexrad_level2_compiler --input <volume> | --url <s3-object-url> --out <path> [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };

    let (bytes, name) = match arg_value(&args, "--input") {
        Some(path) => {
            let leaf = path.rsplit('/').next().unwrap_or(&path).to_string();
            match std::fs::read(&path) {
                Ok(b) => (b, leaf),
                Err(e) => {
                    eprintln!("{path}: read returned void ({e})");
                    std::process::exit(1);
                }
            }
        }
        None => match arg_value(&args, "--url") {
            Some(url) => {
                let leaf = url.rsplit('/').next().unwrap_or(&url).to_string();
                match fetch_raw_bytes(&url, 600) {
                    Some(b) => (b, leaf),
                    None => {
                        eprintln!("{url}: fetch returned void");
                        std::process::exit(1);
                    }
                }
            }
            None => {
                eprintln!("{usage}");
                std::process::exit(1);
            }
        },
    };

    let volume = match parse_nexrad(&bytes) {
        Some(v) => v,
        None => {
            eprintln!("{name}: not an AR2V NEXRAD Level-II volume — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    report_geometry(&volume);
    let samples = match collect(&volume) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("nexrad_level2_compiler: {e}");
            std::process::exit(1);
        }
    };
    report_value_ranges(&samples);
    let site = volume.stid.and_then(|stid| nexrad_site(&stid));
    let asset = NexradAsset { site, samples };

    if let Err(e) = write_asset(&asset, &out_path) {
        eprintln!("nexrad_level2_compiler: {e}");
        std::process::exit(1);
    }
    let written = match std::fs::read(&out_path) {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "nexrad_level2_compiler: {out_path} read returned void — the roundtrip stays unverified"
            );
            std::process::exit(1);
        }
    };
    match parse_asset(&written) {
        Some(parsed) if parsed.samples.len() == asset.samples.len() => {
            let last = parsed.samples.last().unwrap();
            let site_line = match &parsed.site {
                Some(s) => format!(
                    "{} lat {:.5} lon {:.5} alt {:.1} m",
                    String::from_utf8_lossy(&s.stid),
                    s.lat_deg,
                    s.lon_deg,
                    s.alt_m
                ),
                None => "absent".to_string(),
            };
            eprintln!(
                "nexrad: {} samples, {} B -> {out_path}, site {site_line}, roundtrip parses; last {} az {:.4} deg el {:.4} deg range {:.3} km value {:.4}",
                parsed.samples.len(),
                written.len(),
                kind_name(last.kind),
                last.az_deg,
                last.el_deg,
                last.range_km,
                last.value
            );
        }
        _ => {
            eprintln!("{out_path}: roundtrip parse returned void — the bin stays unverified");
            std::process::exit(1);
        }
    }

    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("upload: {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_clock_is_j2000_epoch_days_plus_millis() {
        assert_eq!(volume_unix(1, 0), 0.0);
        assert_eq!(volume_unix(19724, 4932), 1_704_067_204.932);
    }

    #[test]
    fn kind_mapping_covers_ref_vel_sw() {
        assert_eq!(kind_of(b"REF"), Some(KIND_REF));
        assert_eq!(kind_of(b"VEL"), Some(KIND_VEL));
        assert_eq!(kind_of(b"SW "), Some(KIND_SW));
        assert_eq!(kind_of(b"ZDR"), None);
        assert_eq!(kind_of(b"PHI"), None);
    }

    #[test]
    fn sentinel_gates_stay_absent() {
        let moment = NexradMoment {
            name: *b"REF",
            num_gates: 4,
            first_gate_km: 2.125,
            gate_width_km: 0.25,
            scale: 2.0,
            offset: 66.0,
            values: vec![0, 1, 2, 128],
        };
        assert_eq!(physical_value(0, &moment), None);
        assert_eq!(physical_value(1, &moment), None);
        assert_eq!(physical_value(2, &moment), Some(-32.0));
        assert_eq!(physical_value(128, &moment), Some(31.0));
    }

    #[test]
    fn asset_roundtrip_and_rejections() {
        let site = NexradSite {
            stid: *b"KTLX",
            lat_deg: 35.33306,
            lon_deg: -97.2775,
            alt_m: 1213.0 * 0.3048,
        };
        let samples = vec![
            NexradSample {
                t: 1_704_067_204.932,
                az_deg: 90.0,
                el_deg: 0.5,
                range_km: 2.125,
                value: -32.0,
                kind: KIND_REF,
            },
            NexradSample {
                t: 1_704_067_204.932,
                az_deg: 90.5,
                el_deg: 0.5,
                range_km: 2.375,
                value: 10.5,
                kind: KIND_VEL,
            },
        ];
        let asset = NexradAsset {
            site: Some(site),
            samples,
        };
        let out = "/tmp/opencode/nexrad_test_asset.bin";
        let bytes = write_asset(&asset, out).unwrap();
        assert_eq!(bytes, HEADER_BYTES + asset.samples.len() * REC_BYTES);
        let read = std::fs::read(out).unwrap();
        let parsed = parse_asset(&read).unwrap();
        assert_eq!(parsed.samples.len(), asset.samples.len());
        let parsed_site = parsed.site.expect("the site roundtrips");
        assert_eq!(parsed_site.stid, *b"KTLX");
        assert!((parsed_site.lat_deg - 35.33306).abs() < 1e-9);
        assert!((parsed_site.lon_deg - -97.2775).abs() < 1e-9);
        assert!((parsed_site.alt_m - 1213.0 * 0.3048).abs() < 1e-9);
        assert_eq!(parsed.samples[0].value, -32.0);
        assert_eq!(parsed.samples[1].kind, KIND_VEL);
        assert_eq!(parsed.samples[1].range_km, 2.375);

        assert!(parse_asset(b"X").is_none());
        assert!(parse_asset(b"NXR1abc").is_none());
        assert!(parse_asset(&read[..read.len() - 1]).is_none());
        let mut nan = read.clone();
        nan[HEADER_BYTES..HEADER_BYTES + 8].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_asset(&nan).is_none());
    }

    #[test]
    fn absent_site_roundtrips_as_none_not_origin() {
        let asset = NexradAsset {
            site: None,
            samples: vec![NexradSample {
                t: 1.0,
                az_deg: 90.0,
                el_deg: 0.5,
                range_km: 2.125,
                value: -32.0,
                kind: KIND_REF,
            }],
        };
        let out = "/tmp/opencode/nexrad_test_no_site.bin";
        let bytes = write_asset(&asset, out).unwrap();
        assert_eq!(bytes, HEADER_BYTES + REC_BYTES);
        let read = std::fs::read(out).unwrap();
        let parsed = parse_asset(&read).unwrap();
        assert!(parsed.site.is_none());
        assert_eq!(parsed.samples.len(), 1);
        assert_eq!(parsed.samples[0].range_km, 2.125);
    }
}
