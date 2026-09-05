pub const BL_NARROWBAND_MAGIC: [u8; 4] = *b"BLN1";
pub const BL_NARROWBAND_HEADER_BYTES: usize = 8;
pub const BL_NARROWBAND_RECORD_BYTES: usize = 48;

#[derive(Clone, Copy, Debug)]
pub struct BlNarrowbandEvent {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub epoch_tdb: f64,
    pub freq_hz: f64,
    pub bin_width_hz: f64,
    pub val: f64,
}

pub fn write_bin(events: &[BlNarrowbandEvent]) -> Option<Vec<u8>> {
    let mut buf =
        Vec::with_capacity(BL_NARROWBAND_HEADER_BYTES + events.len() * BL_NARROWBAND_RECORD_BYTES);
    buf.extend_from_slice(&BL_NARROWBAND_MAGIC);
    buf.extend_from_slice(&(events.len() as u32).to_le_bytes());
    for e in events {
        if !e.ra_deg.is_finite()
            || !e.dec_deg.is_finite()
            || !e.epoch_tdb.is_finite()
            || !e.freq_hz.is_finite()
            || !e.bin_width_hz.is_finite()
            || !e.val.is_finite()
        {
            return None;
        }
        buf.extend_from_slice(&e.ra_deg.to_le_bytes());
        buf.extend_from_slice(&e.dec_deg.to_le_bytes());
        buf.extend_from_slice(&e.epoch_tdb.to_le_bytes());
        buf.extend_from_slice(&e.freq_hz.to_le_bytes());
        buf.extend_from_slice(&e.bin_width_hz.to_le_bytes());
        buf.extend_from_slice(&e.val.to_le_bytes());
    }
    Some(buf)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<BlNarrowbandEvent>> {
    if bytes.len() < BL_NARROWBAND_HEADER_BYTES || bytes[0..4] != BL_NARROWBAND_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != BL_NARROWBAND_HEADER_BYTES + count * BL_NARROWBAND_RECORD_BYTES {
        return None;
    }
    let mut events = Vec::with_capacity(count);
    for i in 0..count {
        let off = BL_NARROWBAND_HEADER_BYTES + i * BL_NARROWBAND_RECORD_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            let raw: [u8; 8] = bytes.get(o..o + 8)?.try_into().ok()?;
            Some(f64::from_le_bytes(raw))
        };
        let e = BlNarrowbandEvent {
            ra_deg: f64_at(off)?,
            dec_deg: f64_at(off + 8)?,
            epoch_tdb: f64_at(off + 16)?,
            freq_hz: f64_at(off + 24)?,
            bin_width_hz: f64_at(off + 32)?,
            val: f64_at(off + 40)?,
        };
        if !e.ra_deg.is_finite()
            || !e.dec_deg.is_finite()
            || !e.epoch_tdb.is_finite()
            || !e.freq_hz.is_finite()
            || !e.bin_width_hz.is_finite()
            || !e.val.is_finite()
        {
            return None;
        }
        events.push(e);
    }
    Some(events)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_roundtrip() {
        let events = vec![
            BlNarrowbandEvent {
                ra_deg: 344.3679166,
                dec_deg: 20.7689,
                epoch_tdb: 1455000000.0,
                freq_hz: 1116.651519e6,
                bin_width_hz: 0.000811e6,
                val: 107.386932,
            },
            BlNarrowbandEvent {
                ra_deg: 350.0,
                dec_deg: -30.0,
                epoch_tdb: 1455001000.0,
                freq_hz: 1420.405751e6,
                bin_width_hz: 1000.0,
                val: 12.5,
            },
        ];
        let bytes = write_bin(&events).expect("write");
        assert_eq!(bytes.len(), 8 + 2 * 48);
        let parsed = parse_bin(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra_deg, events[0].ra_deg);
        assert_eq!(parsed[0].dec_deg, events[0].dec_deg);
        assert_eq!(parsed[0].epoch_tdb, events[0].epoch_tdb);
        assert_eq!(parsed[0].freq_hz, events[0].freq_hz);
        assert_eq!(parsed[0].bin_width_hz, events[0].bin_width_hz);
        assert_eq!(parsed[0].val, events[0].val);
        assert_eq!(parsed[1].freq_hz, events[1].freq_hz);
    }

    #[test]
    fn parse_refuses_malformed() {
        assert!(parse_bin(b"XXXX00000000").is_none());
        let events = vec![BlNarrowbandEvent {
            ra_deg: 1.0,
            dec_deg: 2.0,
            epoch_tdb: 3.0,
            freq_hz: 4.0,
            bin_width_hz: 5.0,
            val: 6.0,
        }];
        let bytes = write_bin(&events).expect("write");
        let mut short = bytes.clone();
        short.truncate(bytes.len() - 4);
        assert!(parse_bin(&short).is_none());
        let mut nonfinite = write_bin(&events).expect("write");
        let val_at = 8 + 40;
        nonfinite[val_at..val_at + 8].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_bin(&nonfinite).is_none());
    }

    #[test]
    fn write_refuses_nonfinite() {
        let bad = BlNarrowbandEvent {
            ra_deg: 1.0,
            dec_deg: 2.0,
            epoch_tdb: 3.0,
            freq_hz: f64::INFINITY,
            bin_width_hz: 5.0,
            val: 6.0,
        };
        assert!(write_bin(&[bad]).is_none());
    }
}
