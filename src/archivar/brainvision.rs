use super::openneuro_eeg::{EegEvent, Events, Samples};

pub struct BrainVisionChannel {
    pub name: String,
    pub resolution: Option<f64>,
    pub unit: Option<String>,
}

pub struct BrainVisionHeader {
    pub data_file: Option<String>,
    pub marker_file: Option<String>,
    pub data_orientation: Option<String>,
    pub sampling_interval_us: Option<u64>,
    pub binary_format: Option<String>,
    pub big_endian: bool,
    pub channels: Vec<BrainVisionChannel>,
}

pub struct BrainVisionMarker {
    pub number: u64,
    pub type_: Option<String>,
    pub description: Option<String>,
    pub position: u64,
    pub size: Option<u64>,
    pub channel: Option<i64>,
}

pub struct BrainVisionEeg {
    pub nbchan: u32,
    pub pnts: u64,
    pub srate: Option<f64>,
    pub labels: Vec<String>,
    pub samples: Samples,
    pub events: Option<Events>,
}

fn strip_bom(bytes: &[u8]) -> &[u8] {
    match bytes {
        [0xEF, 0xBB, 0xBF, rest @ ..] => rest,
        b => b,
    }
}

fn csv_fields(value: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut chars = value.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if !quoted => quoted = true,
            '"' => {
                if chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    quoted = false;
                }
            }
            ',' if !quoted => {
                fields.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    fields.push(cur);
    fields
}

pub fn parse_vhdr(bytes: &[u8]) -> Option<BrainVisionHeader> {
    let text = String::from_utf8_lossy(strip_bom(bytes));
    let mut header = BrainVisionHeader {
        data_file: None,
        marker_file: None,
        data_orientation: None,
        sampling_interval_us: None,
        binary_format: None,
        big_endian: false,
        channels: Vec::new(),
    };
    let mut section = String::new();
    let mut declared_channels: Option<u64> = None;
    let mut saw_channel_infos = false;
    for raw in text.lines() {
        let line = raw.trim_end_matches('\r').trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section.clear();
            section.push_str(line);
            continue;
        }
        let Some(eq) = line.find('=') else {
            continue;
        };
        let key = line[..eq].trim();
        let value = line[eq + 1..].trim();
        match section.as_str() {
            "[Common Infos]" => match key {
                "DataFile" => {
                    if !value.is_empty() {
                        header.data_file = Some(value.to_string());
                    }
                }
                "MarkerFile" => {
                    if !value.is_empty() {
                        header.marker_file = Some(value.to_string());
                    }
                }
                "DataFormat" => {
                    if !value.eq_ignore_ascii_case("BINARY") {
                        return None;
                    }
                }
                "DataOrientation" => {
                    if !(value.eq_ignore_ascii_case("MULTIPLEXED")
                        || value.eq_ignore_ascii_case("VECTORIZED"))
                    {
                        return None;
                    }
                    header.data_orientation = Some(value.to_string());
                }
                "SamplingInterval" => {
                    header.sampling_interval_us = value.parse::<u64>().ok().filter(|v| *v > 0);
                }
                "NumberOfChannels" => {
                    let Some(n) = value.parse::<u64>().ok() else {
                        return None;
                    };
                    declared_channels = Some(n);
                }
                _ => {}
            },
            "[Binary Infos]" => match key {
                "BinaryFormat" => {
                    if !value.is_empty() {
                        header.binary_format = Some(value.to_string());
                    }
                }
                "UseBigEndianOrder" => {
                    header.big_endian = value.eq_ignore_ascii_case("YES");
                }
                _ => {}
            },
            "[Channel Infos]" => {
                saw_channel_infos = true;
                let Some(_) = key
                    .strip_prefix("Ch")
                    .and_then(|n| n.trim().parse::<u64>().ok())
                else {
                    continue;
                };
                let fields = csv_fields(value);
                let name = match fields.first() {
                    Some(f) => f.trim().to_string(),
                    None => continue,
                };
                if name.is_empty() {
                    continue;
                }
                let resolution = fields.get(2).and_then(|f| {
                    let t = f.trim();
                    if t.is_empty() {
                        None
                    } else {
                        t.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0)
                    }
                });
                let unit = fields
                    .get(3)
                    .map(|f| f.trim().to_string())
                    .filter(|u| !u.is_empty());
                header.channels.push(BrainVisionChannel {
                    name,
                    resolution,
                    unit,
                });
            }
            _ => {}
        }
    }
    if !saw_channel_infos || header.channels.is_empty() {
        return None;
    }
    if header.binary_format.is_none() {
        return None;
    }
    if let Some(n) = declared_channels
        && n as usize != header.channels.len()
    {
        return None;
    }
    Some(header)
}

pub fn parse_vmrk(bytes: &[u8]) -> Option<Vec<BrainVisionMarker>> {
    let text = String::from_utf8_lossy(strip_bom(bytes));
    let mut in_markers = false;
    let mut out = Vec::new();
    for raw in text.lines() {
        let line = raw.trim_end_matches('\r').trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_markers = line == "[Marker Infos]";
            continue;
        }
        if !in_markers {
            continue;
        }
        let Some(eq) = line.find('=') else {
            continue;
        };
        let key = line[..eq].trim();
        let Some(number) = key
            .strip_prefix("Mk")
            .and_then(|n| n.trim().parse::<u64>().ok())
        else {
            continue;
        };
        let fields = csv_fields(&line[eq + 1..]);
        let Some(position) = fields.get(2).and_then(|f| f.trim().parse::<u64>().ok()) else {
            continue;
        };
        if position == 0 {
            continue;
        }
        let size = match fields.get(3) {
            Some(f) => {
                let t = f.trim();
                if t.is_empty() {
                    None
                } else {
                    let Some(s) = t.parse::<u64>().ok() else {
                        continue;
                    };
                    Some(s)
                }
            }
            None => None,
        };
        let channel = fields.get(4).and_then(|f| {
            let t = f.trim();
            if t.is_empty() {
                None
            } else {
                t.parse::<i64>().ok()
            }
        });
        let type_ = fields
            .get(0)
            .map(|f| f.trim().to_string())
            .filter(|s| !s.is_empty());
        let description = fields
            .get(1)
            .map(|f| f.trim().to_string())
            .filter(|s| !s.is_empty());
        out.push(BrainVisionMarker {
            number,
            type_,
            description,
            position,
            size,
            channel,
        });
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

fn binary_elem(format: &str) -> Option<usize> {
    match format {
        "INT_16" | "UINT_16" => Some(2),
        "INT_32" | "IEEE_FLOAT_32" | "FLOAT_32" => Some(4),
        _ => None,
    }
}

pub fn decode_samples(header: &BrainVisionHeader, eeg: &[u8]) -> Option<Samples> {
    let nbchan = u32::try_from(header.channels.len()).ok()?;
    if nbchan == 0 || header.big_endian {
        return None;
    }
    let format = header.binary_format.as_deref()?;
    let elem = binary_elem(format)?;
    let stride = (nbchan as usize).checked_mul(elem)?;
    if !eeg.len().is_multiple_of(stride) {
        return None;
    }
    let pnts = eeg.len() / stride;
    let vectorized = header
        .data_orientation
        .as_deref()
        .is_some_and(|o| o.eq_ignore_ascii_case("VECTORIZED"));
    let mut out = Vec::with_capacity(eeg.len() / elem);
    for t in 0..pnts {
        for ch in 0..nbchan as usize {
            let at = if vectorized {
                (ch * pnts + t) * elem
            } else {
                (t * nbchan as usize + ch) * elem
            };
            let raw = eeg.get(at..at + elem)?;
            let value = match format {
                "INT_16" => i16::from_le_bytes(raw.try_into().ok()?) as f32,
                "UINT_16" => u16::from_le_bytes(raw.try_into().ok()?) as f32,
                "INT_32" => i32::from_le_bytes(raw.try_into().ok()?) as f32,
                _ => f32::from_le_bytes(raw.try_into().ok()?),
            };
            let scale = header.channels.get(ch)?.resolution.map_or(1.0f64, |r| r);
            let scaled = value * scale as f32;
            if !scaled.is_finite() {
                return None;
            }
            out.push(scaled);
        }
    }
    Some(Samples::Single(out))
}

fn events_of(markers: &[BrainVisionMarker]) -> Events {
    Events::Normalized(
        markers
            .iter()
            .map(|m| EegEvent {
                type_: m.description.clone().or_else(|| m.type_.clone()),
                latency: Some(m.position as f64),
                duration: m.size.map(|s| s as f64),
                urevent: None,
            })
            .collect(),
    )
}

pub fn extract(vhdr: &[u8], vmrk: Option<&[u8]>, eeg: &[u8]) -> Option<BrainVisionEeg> {
    let header = parse_vhdr(vhdr)?;
    let samples = decode_samples(&header, eeg)?;
    let nbchan = u32::try_from(header.channels.len()).ok()?;
    let n = match &samples {
        Samples::Single(s) => s.len(),
        Samples::Double(_) => return None,
    };
    if n == 0 {
        return None;
    }
    let pnts = (n / nbchan as usize) as u64;
    let labels: Vec<String> = header.channels.iter().map(|c| c.name.clone()).collect();
    if labels.iter().any(|l| l.len() > 0xFFFF) {
        return None;
    }
    let srate = header
        .sampling_interval_us
        .map(|us| 1_000_000.0 / us as f64);
    let events = vmrk.and_then(parse_vmrk).map(|m| events_of(&m));
    Some(BrainVisionEeg {
        nbchan,
        pnts,
        srate,
        labels,
        samples,
        events,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::openneuro_eeg::{OpenNeuroEeg, parse_bin, write_bin};
    use crate::archivar::sha256::sha256_raw;

    fn vhdr() -> Vec<u8> {
        "\u{FEFF}Brain Vision Data Exchange Header File Version 1.0\r\n\
         [Common Infos]\r\n\
         DataFile=sub-01_task-x_eeg.eeg\r\n\
         MarkerFile=sub-01_task-x_eeg.vmrk\r\n\
         DataFormat=BINARY\r\n\
         DataOrientation=MULTIPLEXED\r\n\
         NumberOfChannels=2\r\n\
         SamplingInterval=2000\r\n\
         [Binary Infos]\r\n\
         BinaryFormat=INT_16\r\n\
         [Channel Infos]\r\n\
         Ch1=Fp1,,0.1,µV\r\n\
         Ch2=Fp2,,0.1,µV\r\n"
            .as_bytes()
            .to_vec()
    }

    fn vmrk() -> Vec<u8> {
        "Brain Vision Data Exchange Marker File, Version 1.0\r\n\
         [Marker Infos]\r\n\
         Mk1=New Segment,,1,1,0,20200101000000000000\r\n\
         Mk2=Stimulus,S  1,101,1,0,20200101000100000000\r\n\
         Mk3=Comment,\"S  2, onset\",201,2,0,20200101000200000000\r\n"
            .as_bytes()
            .to_vec()
    }

    fn eeg_multiplexed() -> Vec<u8> {
        let mut b = Vec::new();
        for v in [100i16, 400, 200, 500, 300, 600] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b
    }

    #[test]
    fn vhdr_header_parses() {
        let h = parse_vhdr(&vhdr()).unwrap();
        assert_eq!(h.data_file.as_deref(), Some("sub-01_task-x_eeg.eeg"));
        assert_eq!(h.marker_file.as_deref(), Some("sub-01_task-x_eeg.vmrk"));
        assert_eq!(h.data_orientation.as_deref(), Some("MULTIPLEXED"));
        assert_eq!(h.sampling_interval_us, Some(2000));
        assert_eq!(h.binary_format.as_deref(), Some("INT_16"));
        assert!(!h.big_endian);
        assert_eq!(h.channels.len(), 2);
        assert_eq!(h.channels[0].name, "Fp1");
        assert_eq!(h.channels[0].resolution, Some(0.1));
        assert_eq!(h.channels[0].unit.as_deref(), Some("µV"));
        assert_eq!(h.channels[1].name, "Fp2");
    }

    #[test]
    fn ascii_data_or_foreign_bytes_reads_absent() {
        assert!(parse_vhdr(&[]).is_none());
        assert!(parse_vhdr(b"not a vhdr").is_none());
        let ascii = String::from_utf8(vhdr()).unwrap().replace("BINARY", "ASCII");
        assert!(parse_vhdr(ascii.as_bytes()).is_none());
        let no_bin = String::from_utf8(vhdr())
            .unwrap()
            .replace("BinaryFormat=INT_16\r\n", "");
        assert!(parse_vhdr(no_bin.as_bytes()).is_none());
        let mismatch = String::from_utf8(vhdr())
            .unwrap()
            .replace("NumberOfChannels=2", "NumberOfChannels=3");
        assert!(parse_vhdr(mismatch.as_bytes()).is_none());
    }

    #[test]
    fn vmrk_markers_parse_with_quoted_descriptions() {
        let m = parse_vmrk(&vmrk()).unwrap();
        assert_eq!(m.len(), 3);
        assert_eq!(m[0].number, 1);
        assert_eq!(m[0].type_.as_deref(), Some("New Segment"));
        assert_eq!(m[0].description, None);
        assert_eq!(m[0].position, 1);
        assert_eq!(m[0].size, Some(1));
        assert_eq!(m[0].channel, Some(0));
        assert_eq!(m[2].number, 3);
        assert_eq!(m[2].description.as_deref(), Some("S  2, onset"));
        assert_eq!(m[2].position, 201);
        assert_eq!(m[2].size, Some(2));
    }

    #[test]
    fn multiplexed_int16_decodes_channel_major_and_scaled() {
        let h = parse_vhdr(&vhdr()).unwrap();
        let samples = decode_samples(&h, &eeg_multiplexed()).unwrap();
        let Samples::Single(v) = samples else {
            panic!("not single");
        };
        assert_eq!(v, vec![10.0f32, 20.0, 30.0, 40.0, 50.0, 60.0]);
    }

    #[test]
    fn vectorized_float32_decodes_as_it_is() {
        let text = String::from_utf8(vhdr())
            .unwrap()
            .replace("DataOrientation=MULTIPLEXED", "DataOrientation=VECTORIZED")
            .replace("BinaryFormat=INT_16", "BinaryFormat=IEEE_FLOAT_32")
            .replace("Ch1=Fp1,,0.1,µV", "Ch1=Fp1,,1,µV")
            .replace("Ch2=Fp2,,0.1,µV", "Ch2=Fp2,,1,µV");
        let h = parse_vhdr(text.as_bytes()).unwrap();
        let mut eeg = Vec::new();
        for v in [1.5f32, 2.5, 3.5, 4.5] {
            eeg.extend_from_slice(&v.to_le_bytes());
        }
        let samples = decode_samples(&h, &eeg).unwrap();
        let Samples::Single(v) = samples else {
            panic!("not single");
        };
        assert_eq!(v, vec![1.5f32, 2.5, 3.5, 4.5]);
    }

    #[test]
    fn extract_carries_labels_srate_and_events() {
        let ex = extract(&vhdr(), Some(&vmrk()), &eeg_multiplexed()).unwrap();
        assert_eq!(ex.nbchan, 2);
        assert_eq!(ex.pnts, 3);
        assert_eq!(ex.srate, Some(500.0));
        assert_eq!(ex.labels, vec!["Fp1".to_string(), "Fp2".to_string()]);
        let Some(Events::Normalized(evs)) = ex.events else {
            panic!("no events");
        };
        assert_eq!(evs.len(), 3);
        assert_eq!(evs[0].type_.as_deref(), Some("New Segment"));
        assert_eq!(evs[0].latency, Some(1.0));
        assert_eq!(evs[0].duration, Some(1.0));
        assert_eq!(evs[0].urevent, None);
        assert_eq!(evs[1].type_.as_deref(), Some("S  1"));
        assert_eq!(evs[1].latency, Some(101.0));
        assert_eq!(evs[2].type_.as_deref(), Some("S  2, onset"));
        assert_eq!(evs[2].duration, Some(2.0));
    }

    #[test]
    fn compact_roundtrip_is_bit_identical() {
        let ex = extract(&vhdr(), Some(&vmrk()), &eeg_multiplexed()).unwrap();
        let mut source = Vec::new();
        source.extend_from_slice(&vhdr());
        source.extend_from_slice(&vmrk());
        source.extend_from_slice(&eeg_multiplexed());
        let rec = OpenNeuroEeg {
            nbchan: ex.nbchan,
            pnts: ex.pnts,
            trials: 1,
            srate: ex.srate,
            sha256: sha256_raw(&source),
            snapshot_tag: "1.0.1".to_string(),
            hexsha: "470458bcff173ca37018a9cb7a55c3804ccc1759".to_string(),
            origin_url: "https://s3.amazonaws.com/openneuro.org/ds007471/sub-01_task-x_eeg.vhdr"
                .to_string(),
            labels: ex.labels.clone(),
            events: ex.events.clone(),
            samples: ex.samples.clone(),
        };
        let bytes = write_bin(&rec);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.nbchan, rec.nbchan);
        assert_eq!(parsed.pnts, rec.pnts);
        assert_eq!(parsed.trials, 1);
        assert_eq!(parsed.srate, Some(500.0));
        assert_eq!(parsed.labels, rec.labels);
        assert_eq!(parsed.sha256, sha256_raw(&source));
        match (&rec.samples, &parsed.samples) {
            (Samples::Single(a), Samples::Single(b)) => {
                assert_eq!(a.len(), b.len());
                for (x, y) in a.iter().zip(b.iter()) {
                    assert_eq!(x.to_bits(), y.to_bits());
                }
            }
            _ => panic!("sample precision mismatch"),
        }
        match (&rec.events, &parsed.events) {
            (Some(Events::Normalized(a)), Some(Events::Normalized(b))) => {
                assert_eq!(a.len(), b.len());
                for (x, y) in a.iter().zip(b.iter()) {
                    assert_eq!(x.type_, y.type_);
                    assert_eq!(x.latency, y.latency);
                    assert_eq!(x.duration, y.duration);
                    assert_eq!(x.urevent, y.urevent);
                }
            }
            _ => panic!("event kind mismatch"),
        }
    }

    #[test]
    fn big_endian_or_unknown_binary_format_reads_absent() {
        let be = String::from_utf8(vhdr()).unwrap().replace(
            "[Binary Infos]\r\n",
            "[Binary Infos]\r\nUseBigEndianOrder=YES\r\n",
        );
        let h = parse_vhdr(be.as_bytes()).unwrap();
        assert!(h.big_endian);
        assert!(decode_samples(&h, &eeg_multiplexed()).is_none());
        let odd = String::from_utf8(vhdr())
            .unwrap()
            .replace("BinaryFormat=INT_16", "BinaryFormat=INT_8");
        let h2 = parse_vhdr(odd.as_bytes()).unwrap();
        assert!(decode_samples(&h2, &eeg_multiplexed()).is_none());
    }

    #[test]
    fn eeg_byte_count_not_a_frame_multiple_reads_absent() {
        let h = parse_vhdr(&vhdr()).unwrap();
        let mut eeg = eeg_multiplexed();
        eeg.pop();
        assert!(decode_samples(&h, &eeg).is_none());
    }

    #[test]
    fn marker_without_position_is_skipped() {
        let text = String::from_utf8(vmrk())
            .unwrap()
            .replace("Mk2=Stimulus,S  1,101,1,0", "Mk2=Stimulus,S  1,,1,0");
        let m = parse_vmrk(text.as_bytes()).unwrap();
        assert_eq!(m.len(), 2);
        assert_eq!(m[1].number, 3);
    }
}
