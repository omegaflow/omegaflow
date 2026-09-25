use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::ionocal::{PackedIonocalFile, parse_bin, parse_records, write_bin};

const BASE: &str =
    "https://pds-rings.seti.org/pds4/bundles/gll.rss/gll.rss.raw/data_trk223_ionocal/";

struct Source {
    name: &'static str,
    sha256: &'static str,
}

const SOURCES: &[Source] = &[
    Source {
        name: "gll_rss_1989292t0321_dssmm_ion.txt",
        sha256: "dfcda8443cb92bb80e522c55b4e74c532fc779d41b6a2253dc1fa07bc1761fcd",
    },
    Source {
        name: "gll_rss_1994001t0430_dssmm_ion.txt",
        sha256: "57769634d230f711c4e2aab4c8b12c93b706c9c39f91e7d9e955528a8989893c",
    },
    Source {
        name: "gll_rss_1998001t1009_dssmm_ion.txt",
        sha256: "ec51f48c536b9e2348b2efb35e8bf23df4f2737dce3a3437289fa25e9638bf93",
    },
    Source {
        name: "gll_rss_2003001t0341_dssmm_ion.txt",
        sha256: "d0c383b48686f12df8de93533fcf52acbe0561e855591977f09662c236de056c",
    },
    Source {
        name: "gll_rss_2003032t0121_dssmm_ion.txt",
        sha256: "0b145553c66f0f894d64e737c0b962e8b77e5e4e6ad5a099de279edc189fa7b4",
    },
    Source {
        name: "gll_rss_2003060t0719_dssmm_ion.txt",
        sha256: "dff1e4b3146d5fb7ae974a715e2bbe56a4e3ebcca5497db4fd624ab832b90375",
    },
    Source {
        name: "gll_rss_2003091t0511_dssmm_ion.txt",
        sha256: "9ad4c0c5566f37a6c443a513b9698a16409622f25a8589c5ae885cc0c8fcf6f9",
    },
    Source {
        name: "gll_rss_2003121t0316_dssmm_ion.txt",
        sha256: "1bc5570a1ad86efe3c03920d2f79440b1b80a8fafa132a053467a8de8b7587a7",
    },
    Source {
        name: "gll_rss_2003152t0126_dssmm_ion.txt",
        sha256: "0d002b6ec78d1fe0a66e26a8ec4e643b73113fa58fbcb817b46365c1102e1fbd",
    },
    Source {
        name: "gll_rss_2003182t0810_dssmm_ion.txt",
        sha256: "433773e66025caf78721c1b6de40aadc0fb2689ab56e48c2f971e14e8fefbf87",
    },
    Source {
        name: "gll_rss_2003213t0641_dssmm_ion.txt",
        sha256: "45fb256353453abdb20f7c5422678ae3719a77b84d726f910f2ddfc162c43e52",
    },
    Source {
        name: "gll_rss_2003244t0513_dssmm_ion.txt",
        sha256: "f0663119bfbd97d5a8fe19a34ff7cd69426d2a5a046a844e76e71bbbfa092393",
    },
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

fn hex32(hex: &str) -> Option<[u8; 32]> {
    let b = hex.as_bytes();
    if b.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = (hex_nibble(b[i * 2])? << 4) | hex_nibble(b[i * 2 + 1])?;
    }
    Some(out)
}

fn hex_string(b: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for v in b {
        s.push_str(&format!("{v:02x}"));
    }
    s
}

fn gate(bytes: &[u8], src: &Source) -> Result<PackedIonocalFile, String> {
    let hex = sha256_hex(bytes);
    if hex != src.sha256 {
        return Err(format!("{}: sha256 {hex} — provenance mismatch", src.name));
    }
    let parsed = parse_records(bytes);
    if parsed.records.is_empty() {
        return Err(format!("{}: no IONCAL records carried", src.name));
    }
    let Some(sha) = hex32(src.sha256) else {
        return Err(format!("{}: provenance hex void", src.name));
    };
    Ok(PackedIonocalFile {
        name: src.name.to_string(),
        sha256: sha,
        record_count: parsed.records.len() as u32,
        bytes: bytes.to_vec(),
    })
}

fn roundtrip_holds(bin: &[u8]) -> bool {
    let Some(parsed) = parse_bin(bin) else {
        return false;
    };
    parsed.iter().all(|f| {
        let p = parse_records(&f.bytes);
        f.record_count as usize == p.records.len() && sha256_hex(&f.bytes) == hex_string(&f.sha256)
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let probe = args.iter().any(|a| a == "--probe");
    let out = match arg_value(&args, "--out") {
        Some(path) => path,
        None => "data/pds-rings.seti.org/galileo_ionocal.bin".to_string(),
    };
    let dir = arg_value(&args, "--dir");

    let mut files: Vec<PackedIonocalFile> = Vec::new();
    for src in SOURCES {
        let bytes = match &dir {
            Some(d) => std::fs::read(format!("{d}/{}", src.name)).ok(),
            None => fetch_raw_bytes(&format!("{BASE}{}", src.name)),
        };
        let Some(bytes) = bytes else {
            eprintln!("{}: read void", src.name);
            std::process::exit(1);
        };
        let f = match gate(&bytes, src) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        };
        let parsed = parse_records(&f.bytes);
        if probe {
            let sub_minute = parsed
                .records
                .iter()
                .filter(|r| (r.from_unix % 60.0).abs() > 1e-6)
                .count();
            let stations = {
                let mut v: Vec<i64> = parsed.records.iter().map(|r| r.station).collect();
                v.sort_unstable();
                v.dedup();
                v
            };
            eprintln!(
                "{}: {} records ({} broken), stations {:?}, {} with sub-minute seconds",
                f.name,
                parsed.records.len(),
                parsed.broken,
                stations,
                sub_minute
            );
            if let Some(lsk) = omegaflow::archivar::membrane::embedded_lsk() {
                let rows = omegaflow::ionocal::series(&parsed.records, &lsk);
                let (mut tmin, mut tmax) = (f64::INFINITY, f64::NEG_INFINITY);
                let (mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY);
                for (t, v, _) in &rows {
                    tmin = tmin.min(*t);
                    tmax = tmax.max(*t);
                    vmin = vmin.min(*v);
                    vmax = vmax.max(*v);
                }
                eprintln!(
                    "{}: {} series rows, tdb {tmin:.3}..{tmax:.3} s, delay {vmin:.3}..{vmax:.3} m",
                    f.name,
                    rows.len()
                );
            }
        }
        eprintln!(
            "{}: {} records, {} bytes, provenance holds",
            f.name,
            f.record_count,
            f.bytes.len()
        );
        files.push(f);
    }
    let bin = write_bin(&files);
    if !roundtrip_holds(&bin) {
        eprintln!("{out}: roundtrip void — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if let Some(p) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(p);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let total_bytes: usize = files.iter().map(|f| f.bytes.len()).sum();
    eprintln!(
        "{out}: {} IONCAL files packaged ({} bytes), roundtrip holds",
        files.len(),
        total_bytes
    );
    if ci_mode && !upload_release("pds-rings.seti.org", &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &[u8] = b"ADJUST(DOPRNG)BY NRMPOW(   0.7048,   0.0938,   0.8502,  -0.2993,   1.0217,      \n   0.0260)                                                         MODEL(CHPART)\nFROM(03/02/01,01:21)TO(03/02/01,15:01)DSN(C10)SCID(77).    #S20 ADJ 030203 11:19\n";

    fn sample_file(name: &str, nrec: usize) -> PackedIonocalFile {
        let mut bytes = Vec::new();
        for _ in 0..nrec {
            bytes.extend_from_slice(FIXTURE);
        }
        let sha256 = hex32(&sha256_hex(&bytes)).unwrap();
        PackedIonocalFile {
            name: name.to_string(),
            sha256,
            record_count: nrec as u32,
            bytes,
        }
    }

    #[test]
    fn hex32_roundtrips_via_hex_string() {
        let h = sha256_hex(b"galileo ionocal provenance");
        let raw = hex32(&h).unwrap();
        assert_eq!(hex_string(&raw), h);
        assert!(hex32("zz").is_none());
    }

    #[test]
    fn gate_holds_when_provenance_matches() {
        let bytes = sample_file("x.txt", 2).bytes;
        let sha = Box::leak(sha256_hex(&bytes).into_boxed_str());
        let src = Source {
            name: "x.txt",
            sha256: sha,
        };
        let f = gate(&bytes, &src).unwrap();
        assert_eq!(f.record_count, 2);
        assert_eq!(f.bytes.len(), 2 * FIXTURE.len());
    }

    #[test]
    fn gate_rejects_provenance_mismatch() {
        let bytes = sample_file("x.txt", 1).bytes;
        let src = Source {
            name: "x.txt",
            sha256: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        };
        assert!(gate(&bytes, &src).is_err());
    }

    #[test]
    fn gate_rejects_recordless_bytes() {
        let src = Source {
            name: "x.txt",
            sha256: "5c62d5a25698d42f7d6b9560184c071ba3d68aeddd9e289d87408361fae50467",
        };
        assert!(gate(b"# nothing carried\n", &src).is_err());
    }

    #[test]
    fn ionocal_bin_roundtrips() {
        let a = sample_file("gll_rss_2003032t0121_dssmm_ion.txt", 3);
        let b = sample_file("gll_rss_2003060t0719_dssmm_ion.txt", 2);
        let bin = write_bin(&[a, b]);
        let parsed = parse_bin(&bin).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].record_count, 3);
        assert_eq!(sha256_hex(&parsed[0].bytes), hex_string(&parsed[0].sha256));
        assert_eq!(parsed[1].bytes.len(), 2 * FIXTURE.len());
        assert!(parse_bin(b"X").is_none());
    }

    #[test]
    fn roundtrip_holds_for_valid_bin() {
        let a = sample_file("gll_rss_2003032t0121_dssmm_ion.txt", 3);
        let bin = write_bin(&[a]);
        assert!(roundtrip_holds(&bin));
    }

    #[test]
    fn roundtrip_voids_on_corruption() {
        let a = sample_file("gll_rss_2003032t0121_dssmm_ion.txt", 3);
        let mut bin = write_bin(&[a]);
        let last = bin.len() - 1;
        bin[last] ^= 0xff;
        assert!(!roundtrip_holds(&bin));
    }
}
