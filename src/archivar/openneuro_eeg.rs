pub const MAGIC: [u8; 4] = *b"EEGB";
pub const VERSION: u32 = 1;

pub const FLAG_DOUBLE: u32 = 1;
pub const FLAG_EVENTS: u32 = 2;
pub const FLAG_EVENTS_VERBATIM: u32 = 4;

const HEADER_LEN: usize = 100;

#[derive(Clone)]
pub enum Samples {
    Single(Vec<f32>),
    Double(Vec<f64>),
}

#[derive(Clone)]
pub struct EegEvent {
    pub type_: Option<String>,
    pub latency: Option<f64>,
    pub duration: Option<f64>,
    pub urevent: Option<f64>,
}

#[derive(Clone)]
pub enum Events {
    Verbatim(Vec<u8>),
    Normalized(Vec<EegEvent>),
}

#[derive(Clone)]
pub struct OpenNeuroEeg {
    pub nbchan: u32,
    pub pnts: u64,
    pub trials: u32,
    pub srate: Option<f64>,
    pub sha256: [u8; 32],
    pub snapshot_tag: String,
    pub hexsha: String,
    pub origin_url: String,
    pub labels: Vec<String>,
    pub events: Option<Events>,
    pub samples: Samples,
}

fn write_events(events: &[EegEvent]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&(events.len() as u32).to_le_bytes());
    for ev in events {
        buf.push(ev.type_.is_some() as u8);
        if let Some(t) = &ev.type_ {
            buf.extend_from_slice(&(t.len() as u16).to_le_bytes());
            buf.extend_from_slice(t.as_bytes());
        }
        buf.push(ev.latency.is_some() as u8);
        if let Some(v) = ev.latency {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        buf.push(ev.duration.is_some() as u8);
        if let Some(v) = ev.duration {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        buf.push(ev.urevent.is_some() as u8);
        if let Some(v) = ev.urevent {
            buf.extend_from_slice(&v.to_le_bytes());
        }
    }
    buf
}

fn parse_events(blob: &[u8]) -> Option<Vec<EegEvent>> {
    let n = u32::from_le_bytes(blob.get(0..4)?.try_into().ok()?) as usize;
    let mut off = 4usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let has_type = *blob.get(off)? != 0;
        off += 1;
        let type_ = if has_type {
            let len = u16::from_le_bytes(blob.get(off..off + 2)?.try_into().ok()?) as usize;
            off += 2;
            let s = String::from_utf8(blob.get(off..off + len)?.to_vec()).ok()?;
            off += len;
            Some(s)
        } else {
            None
        };
        let has_latency = *blob.get(off)? != 0;
        off += 1;
        let latency = if has_latency {
            let v = f64::from_le_bytes(blob.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            Some(v)
        } else {
            None
        };
        let has_duration = *blob.get(off)? != 0;
        off += 1;
        let duration = if has_duration {
            let v = f64::from_le_bytes(blob.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            Some(v)
        } else {
            None
        };
        let has_urevent = *blob.get(off)? != 0;
        off += 1;
        let urevent = if has_urevent {
            let v = f64::from_le_bytes(blob.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            Some(v)
        } else {
            None
        };
        out.push(EegEvent {
            type_,
            latency,
            duration,
            urevent,
        });
    }
    if off != blob.len() {
        return None;
    }
    Some(out)
}

fn take_str(bytes: &[u8], off: &mut usize, len: usize) -> Option<String> {
    let s = String::from_utf8(bytes.get(*off..*off + len)?.to_vec()).ok()?;
    *off += len;
    Some(s)
}

fn take_bytes(bytes: &[u8], off: &mut usize, len: usize) -> Option<Vec<u8>> {
    let b = bytes.get(*off..*off + len)?.to_vec();
    *off += len;
    Some(b)
}

pub fn write_bin(eeg: &OpenNeuroEeg) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&VERSION.to_le_bytes());
    let mut flags = 0u32;
    if matches!(eeg.samples, Samples::Double(_)) {
        flags |= FLAG_DOUBLE;
    }
    let events_blob: Option<Vec<u8>> = match &eeg.events {
        Some(Events::Verbatim(b)) => {
            flags |= FLAG_EVENTS | FLAG_EVENTS_VERBATIM;
            Some(b.clone())
        }
        Some(Events::Normalized(evs)) => {
            flags |= FLAG_EVENTS;
            Some(write_events(evs))
        }
        None => None,
    };
    buf.extend_from_slice(&flags.to_le_bytes());
    buf.extend_from_slice(&eeg.nbchan.to_le_bytes());
    buf.extend_from_slice(&eeg.pnts.to_le_bytes());
    buf.extend_from_slice(&eeg.trials.to_le_bytes());
    buf.push(eeg.srate.is_some() as u8);
    buf.extend_from_slice(&[0u8; 3]);
    match eeg.srate {
        Some(s) => buf.extend_from_slice(&s.to_le_bytes()),
        None => buf.extend_from_slice(&[0u8; 8]),
    }
    buf.extend_from_slice(&eeg.sha256);
    buf.extend_from_slice(&(eeg.snapshot_tag.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(eeg.hexsha.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(eeg.origin_url.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(eeg.labels.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(events_blob.as_ref().map_or(0, |b| b.len() as u32)).to_le_bytes());
    let samples_len = match &eeg.samples {
        Samples::Single(s) => s.len() as u64 * 4,
        Samples::Double(s) => s.len() as u64 * 8,
    };
    buf.extend_from_slice(&samples_len.to_le_bytes());
    buf.extend_from_slice(eeg.snapshot_tag.as_bytes());
    buf.extend_from_slice(eeg.hexsha.as_bytes());
    buf.extend_from_slice(eeg.origin_url.as_bytes());
    for label in &eeg.labels {
        buf.extend_from_slice(&(label.len() as u16).to_le_bytes());
        buf.extend_from_slice(label.as_bytes());
    }
    if let Some(blob) = &events_blob {
        buf.extend_from_slice(blob);
    }
    match &eeg.samples {
        Samples::Single(s) => {
            for v in s {
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
        Samples::Double(s) => {
            for v in s {
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<OpenNeuroEeg> {
    if bytes.len() < HEADER_LEN || bytes[0..4] != MAGIC {
        return None;
    }
    if u32::from_le_bytes(bytes[4..8].try_into().ok()?) != VERSION {
        return None;
    }
    let flags = u32::from_le_bytes(bytes[8..12].try_into().ok()?);
    let nbchan = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let pnts = u64::from_le_bytes(bytes[16..24].try_into().ok()?);
    let trials = u32::from_le_bytes(bytes[24..28].try_into().ok()?);
    let srate_present = bytes[28] != 0;
    let srate = f64::from_le_bytes(bytes[32..40].try_into().ok()?);
    let sha256: [u8; 32] = bytes[40..72].try_into().ok()?;
    let tag_len = u32::from_le_bytes(bytes[72..76].try_into().ok()?) as usize;
    let hexsha_len = u32::from_le_bytes(bytes[76..80].try_into().ok()?) as usize;
    let url_len = u32::from_le_bytes(bytes[80..84].try_into().ok()?) as usize;
    let labels_n = u32::from_le_bytes(bytes[84..88].try_into().ok()?) as usize;
    let events_len = u32::from_le_bytes(bytes[88..92].try_into().ok()?) as usize;
    let samples_len = u64::from_le_bytes(bytes[92..100].try_into().ok()?) as usize;

    if labels_n != nbchan as usize {
        return None;
    }
    let double = flags & FLAG_DOUBLE != 0;
    let elem = if double { 8usize } else { 4usize };
    let total = (nbchan as u64)
        .checked_mul(pnts)?
        .checked_mul(trials as u64)?;
    if !samples_len.is_multiple_of(elem) || (samples_len / elem) as u64 != total {
        return None;
    }

    let mut off = HEADER_LEN;
    let snapshot_tag = take_str(bytes, &mut off, tag_len)?;
    let hexsha = take_str(bytes, &mut off, hexsha_len)?;
    let origin_url = take_str(bytes, &mut off, url_len)?;
    let mut labels = Vec::with_capacity(labels_n);
    for _ in 0..labels_n {
        let len = u16::from_le_bytes(bytes.get(off..off + 2)?.try_into().ok()?) as usize;
        off += 2;
        labels.push(take_str(bytes, &mut off, len)?);
    }
    let events = if flags & FLAG_EVENTS != 0 {
        let blob = take_bytes(bytes, &mut off, events_len)?;
        if flags & FLAG_EVENTS_VERBATIM != 0 {
            Some(Events::Verbatim(blob))
        } else {
            Some(Events::Normalized(parse_events(&blob)?))
        }
    } else {
        None
    };
    let sample_bytes = take_bytes(bytes, &mut off, samples_len)?;
    if off != bytes.len() {
        return None;
    }
    let samples = if double {
        let mut v = Vec::with_capacity(total as usize);
        for i in 0..total as usize {
            v.push(f64::from_le_bytes(
                sample_bytes[i * 8..i * 8 + 8].try_into().ok()?,
            ));
        }
        Samples::Double(v)
    } else {
        let mut v = Vec::with_capacity(total as usize);
        for i in 0..total as usize {
            v.push(f32::from_le_bytes(
                sample_bytes[i * 4..i * 4 + 4].try_into().ok()?,
            ));
        }
        Samples::Single(v)
    };
    Some(OpenNeuroEeg {
        nbchan,
        pnts,
        trials,
        srate: if srate_present { Some(srate) } else { None },
        sha256,
        snapshot_tag,
        hexsha,
        origin_url,
        labels,
        events,
        samples,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bin(samples: Samples, events: Option<Events>) -> OpenNeuroEeg {
        OpenNeuroEeg {
            nbchan: 2,
            pnts: 3,
            trials: 1,
            srate: Some(100.0),
            sha256: [0xAA; 32],
            snapshot_tag: "1.0.1".to_string(),
            hexsha: "470458bcff173ca37018a9cb7a55c3804ccc1759".to_string(),
            origin_url: "https://s3.amazonaws.com/openneuro.org/ds005034/x.set".to_string(),
            labels: vec!["Fp1".to_string(), "Fp2".to_string()],
            events,
            samples,
        }
    }

    #[test]
    fn single_samples_roundtrip_bit_identical() {
        let eeg = bin(Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]), None);
        let bytes = write_bin(&eeg);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.nbchan, 2);
        assert_eq!(parsed.pnts, 3);
        assert_eq!(parsed.trials, 1);
        assert_eq!(parsed.srate, Some(100.0));
        assert_eq!(parsed.sha256, [0xAA; 32]);
        assert_eq!(parsed.snapshot_tag, "1.0.1");
        assert_eq!(parsed.labels, vec!["Fp1".to_string(), "Fp2".to_string()]);
        assert!(parsed.events.is_none());
        match parsed.samples {
            Samples::Single(v) => assert_eq!(v, vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0]),
            _ => panic!("not single"),
        }
    }

    #[test]
    fn double_samples_and_absent_srate_roundtrip() {
        let mut eeg = bin(Samples::Double(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]), None);
        eeg.srate = None;
        let bytes = write_bin(&eeg);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.srate, None);
        match parsed.samples {
            Samples::Double(v) => assert_eq!(v, vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]),
            _ => panic!("not double"),
        }
    }

    #[test]
    fn normalized_events_roundtrip() {
        let events = Events::Normalized(vec![
            EegEvent {
                type_: Some("boundary".to_string()),
                latency: Some(1.5),
                duration: None,
                urevent: Some(7.0),
            },
            EegEvent {
                type_: None,
                latency: Some(2.5),
                duration: Some(0.25),
                urevent: None,
            },
        ]);
        let eeg = bin(
            Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            Some(events),
        );
        let bytes = write_bin(&eeg);
        let parsed = parse_bin(&bytes).unwrap();
        let Events::Normalized(evs) = parsed.events.unwrap() else {
            panic!("not normalized events");
        };
        assert_eq!(evs.len(), 2);
        assert_eq!(evs[0].type_.as_deref(), Some("boundary"));
        assert_eq!(evs[0].latency, Some(1.5));
        assert_eq!(evs[0].duration, None);
        assert_eq!(evs[0].urevent, Some(7.0));
        assert_eq!(evs[1].type_, None);
        assert_eq!(evs[1].latency, Some(2.5));
    }

    #[test]
    fn verbatim_events_carry_their_bytes() {
        let blob = vec![14u8, 0, 0, 0, 8, 0, 0, 0];
        let eeg = bin(
            Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            Some(Events::Verbatim(blob.clone())),
        );
        let bytes = write_bin(&eeg);
        let parsed = parse_bin(&bytes).unwrap();
        let Events::Verbatim(b) = parsed.events.unwrap() else {
            panic!("not verbatim events");
        };
        assert_eq!(b, blob);
    }

    #[test]
    fn foreign_bytes_and_bad_lengths_read_absent() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"EEGB").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let eeg = bin(Samples::Single(vec![1.0]), None);
        let mut bytes = write_bin(&eeg);
        bytes.pop();
        assert!(parse_bin(&bytes).is_none());
    }
}
