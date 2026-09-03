use crate::lsk::days_from_civil;

pub const MAGIC: [u8; 4] = *b"PTLM";
pub const CHAN_MASK: u32 = 0xFF;
pub const FILE_SHIFT: u32 = 8;

pub struct FileDef {
    pub id: u32,
    pub stem: &'static str,
    pub cols: Option<&'static [&'static str]>,
}

pub const FILES: [FileDef; 14] = [
    FileDef {
        id: 0,
        stem: "23AGC",
        cols: None,
    },
    FileDef {
        id: 1,
        stem: "23AGCcomm",
        cols: None,
    },
    FileDef {
        id: 2,
        stem: "23comm",
        cols: Some(&["Ttwt", "Utwt", "Ptwt", "Prec"]),
    },
    FileDef {
        id: 3,
        stem: "23elec",
        cols: Some(&[
            "PRTG",
            "Pcable",
            "Ubus",
            "Pbus",
            "Pshunt",
            "Pshrad",
            "Pbat",
            "C108  124  128 316 421",
            "|",
            "C105",
            "C106",
            "C107",
            "C108",
            "C109",
            "C110",
            "C113",
            "C114",
            "C122",
            "C123",
            "C124",
            "C125",
            "C126",
            "C127",
            "C128",
            "C129",
            "C131",
            "C316",
            "C421",
        ]),
    },
    FileDef {
        id: 4,
        stem: "23misc1",
        cols: Some(&["C115", "C206", "C223", "C303", "C317"]),
    },
    FileDef {
        id: 5,
        stem: "23misc2",
        cols: Some(&[
            "C207", "C211", "C212", "C216", "C221", "C222", "C224", "C227", "C229", "C230", "C313",
            "C314", "C315",
        ]),
    },
    FileDef {
        id: 6,
        stem: "23misc3",
        cols: Some(&["C225", "C226", "C403", "C417"]),
    },
    FileDef {
        id: 7,
        stem: "23misc4",
        cols: Some(&["C421", "C424", "C425", "C427"]),
    },
    FileDef {
        id: 8,
        stem: "23misc5",
        cols: Some(&[
            "E101", "E102", "E109", "E110", "E117", "E118", "E125", "E128", "E201", "E209", "E213",
            "E221",
        ]),
    },
    FileDef {
        id: 9,
        stem: "23prop",
        cols: Some(&["Tn2", "Tprop", "Pprop", "Tspin"]),
    },
    FileDef {
        id: 10,
        stem: "23pulse",
        cols: Some(&["", "1A", "1B", "2A", "2B"]),
    },
    FileDef {
        id: 11,
        stem: "23spin",
        cols: Some(&["Spin (RPM)", "Source"]),
    },
    FileDef {
        id: 12,
        stem: "23temp",
        cols: Some(&[
            "Trtg1", "Trtg2", "Trtg3", "Trtg4", "Tplt1", "Tplt2", "Tplt3", "Tplt4", "Tplt5",
            "Tplt6",
        ]),
    },
    FileDef {
        id: 13,
        stem: "23ttemp",
        cols: Some(&["T1(1A)", "T2(1B)", "T3(2B)", "T4(2A)", "Tvelc1", "Tvelc2"]),
    },
];

pub fn file_def(id: u32) -> Option<&'static FileDef> {
    FILES.iter().find(|f| f.id == id)
}

pub fn file_name(id: u32) -> Option<&'static str> {
    file_def(id).map(|f| f.stem)
}

pub fn channel_name(chan: u32) -> Option<String> {
    let file = file_def(chan >> FILE_SHIFT)?;
    let col = (chan & CHAN_MASK) as usize;
    match (file.cols, col.checked_sub(1)) {
        (Some(names), Some(k)) => match names.get(k) {
            Some(n) if !n.is_empty() => Some(format!("{}_{}", file.stem, n)),
            _ => Some(format!("{}_c{col}", file.stem)),
        },
        _ => Some(format!("{}_c{col}", file.stem)),
    }
}

pub fn channel_unit(chan: u32) -> Option<&'static str> {
    let file = chan >> FILE_SHIFT;
    let col = chan & CHAN_MASK;
    match (file, col) {
        (0 | 1, _) => Some("dB (AGC)"),
        (2, 1) => Some("°F"),
        (2, 2) => Some("mA"),
        (2, 3 | 4) => Some("dBm"),
        (3, 1..=2) => Some("W"),
        (3, 3) => Some("V"),
        (3, 4..=7) => Some("W"),
        (9, 1 | 2 | 4) => Some("°F"),
        (9, 3) => Some("PSIA"),
        (11, 1) => Some("RPM"),
        (12, 1..=4) => Some("°F (RTG-Fin-Root)"),
        (12, 5..=10) => Some("°F (Platform)"),
        (13, 1..=4) => Some("°F (VPT)"),
        (13, 5..=6) => Some("°F (Cluster)"),
        _ => None,
    }
}

fn month_of(s: &str) -> Option<i64> {
    Some(match s {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => return None,
    })
}

pub fn epoch_unix_of(ts: &str) -> Option<f64> {
    let mut toks = ts.split_whitespace();
    toks.next();
    let mon = month_of(toks.next()?)?;
    let day: i64 = toks.next()?.parse().ok()?;
    let clock = toks.next()?;
    let parts: Vec<&str> = clock.split(':').collect();
    let (h, mi, se, year): (i64, i64, i64, i64) = if parts.len() == 4 {
        (
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
            parts[2].parse().ok()?,
            parts[3].parse().ok()?,
        )
    } else if parts.len() == 3 {
        (
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
            parts[2].parse().ok()?,
            toks.next()?.parse().ok()?,
        )
    } else {
        return None;
    };
    let days = days_from_civil(year, mon, day)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + se as f64)
}

pub fn parse_series(text: &str, file_id: u32) -> Vec<(f64, u32, f64)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.is_empty() || fields[0].trim().is_empty() {
            continue;
        }
        if fields[0].starts_with("DOW") {
            continue;
        }
        let Some(t) = epoch_unix_of(fields[0]) else {
            continue;
        };
        for (k, f) in fields.iter().enumerate().skip(1) {
            if k as u32 >= CHAN_MASK {
                break;
            }
            if let Ok(v) = f.trim().parse::<f64>() {
                if v.is_finite() {
                    out.push((t, file_id << FILE_SHIFT | k as u32, v));
                }
            }
        }
    }
    out
}

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, chan) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&chan.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let chan = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        out.push((t, val, chan));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_variants_parse() {
        let d0 = days_from_civil(1972, 3, 3).unwrap() as f64 * 86400.0;
        assert!((epoch_unix_of("Fri Mar  3 00:00:00 1972").unwrap() - d0).abs() < 1e-9);
        assert!((epoch_unix_of("Fri Mar 03 00:00:00 1972").unwrap() - d0).abs() < 1e-9);
        assert!((epoch_unix_of("Fri Mar  3 00:00:00:1972").unwrap() - d0).abs() < 1e-9);
        assert!(
            (epoch_unix_of("Fri Mar  3 04:00:00 1972").unwrap() - (d0 + 4.0 * 3600.0)).abs() < 1e-9
        );
        assert!(epoch_unix_of("DOW Mon Dy hh:mm:ss Year").is_none());
    }

    #[test]
    fn pulse_fragment_skips_header_and_second_timestamp() {
        let text = "DOW Mon Dy hh:mm:ss yyyy\tDOW Mon Dy hh:mm:ss yyyy\t1A\t1B\t2A\t2B\n\nWed Mar  1 00:00:00 1972\tFri Mar  3 00:00:00 1972\t3\t3\t3\t3\nFri Mar  3 04:00:00 1972\tFri Mar  3 05:00:00 1972\t34\t0\t7\t0\n";
        let rows = parse_series(text, 10);
        assert_eq!(rows.len(), 8);
        let d0 = days_from_civil(1972, 3, 1).unwrap() as f64 * 86400.0;
        assert!((rows[0].0 - d0).abs() < 1e-9);
        assert_eq!(rows[0].1, 10 << FILE_SHIFT | 2);
        assert_eq!(rows[0].2, 3.0);
        assert_eq!(rows[7].1, 10 << FILE_SHIFT | 5);
        assert_eq!(rows[7].2, 0.0);
        assert!(
            rows.iter().all(|r| (r.1 & CHAN_MASK) >= 2),
            "column 0/1 (timestamps) stay uncarried"
        );
    }

    #[test]
    fn elec_fragment_skips_pipe_and_blanks() {
        let text = "DOW Mon Dy hh:mm:ss Year\tPRTG\tPcable\tUbus\tPbus\nFri Mar  3 00:00:00 1972\t160.504\t7.931\t.239\t\nFri Mar  3 01:00:00 1972\t\t7.931\t27.891\t100.574\n";
        let rows = parse_series(text, 3);
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0].2, 160.504);
        assert_eq!(rows[2].2, 0.239, "leading point parses");
        assert_eq!(
            rows.iter().filter(|r| (r.1 & CHAN_MASK) == 1).count(),
            1,
            "row 1 carries column 1, row 2 has an empty cell there"
        );
        assert_eq!(
            rows.iter().filter(|r| (r.1 & CHAN_MASK) == 4).count(),
            1,
            "row 2 carries column 4, row 1 has an empty cell there"
        );
    }

    #[test]
    fn headerless_file_reads_data_first_line() {
        let text = "Fri Mar  3 00:00:00 1972\t95.155033\nFri Mar  3 01:00:00 1972\t95.053941\n";
        let rows = parse_series(text, 1);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].2, 95.155033);
        assert_eq!(channel_name(rows[0].1).unwrap(), "23AGCcomm_c1");
    }

    #[test]
    fn channel_names_from_headers() {
        assert_eq!(channel_name(12 << FILE_SHIFT | 1).unwrap(), "23temp_Trtg1");
        assert_eq!(channel_name(9 << FILE_SHIFT | 3).unwrap(), "23prop_Pprop");
        assert_eq!(channel_name(10 << FILE_SHIFT | 2).unwrap(), "23pulse_1A");
        assert_eq!(channel_name(10 << FILE_SHIFT | 1).unwrap(), "23pulse_c1");
        assert_eq!(channel_name(3 << FILE_SHIFT | 9).unwrap(), "23elec_|");
        assert_eq!(
            channel_unit(12 << FILE_SHIFT | 1),
            Some("°F (RTG-Fin-Root)")
        );
        assert_eq!(channel_unit(3 << FILE_SHIFT | 1), Some("W"));
        assert_eq!(channel_unit(3 << FILE_SHIFT | 5), Some("W"));
        assert_eq!(channel_unit(9 << FILE_SHIFT | 3), Some("PSIA"));
        assert_eq!(channel_unit(2 << FILE_SHIFT | 4), Some("dBm"));
        assert_eq!(channel_unit(0 << FILE_SHIFT | 1), Some("dB (AGC)"));
        assert_eq!(
            channel_unit(3 << FILE_SHIFT | 9),
            None,
            "raw 6-bit word stays undecoded"
        );
        assert!(channel_name(99 << FILE_SHIFT | 0).is_none());
    }

    #[test]
    fn bin_roundtrip() {
        let records = vec![
            (-220_000_000.0, 205.022, 12 << FILE_SHIFT | 0),
            (10.0, 160.504, 3 << FILE_SHIFT | 0),
            (10.0, -27.148, 8 << FILE_SHIFT | 4),
        ];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"PTLMabc").is_none());
    }
}
