use crate::archivar::s2event::{self, S2EventRecord};

pub const MAGIC: [u8; 4] = s2event::MAGIC;
pub const COMP_EVENTS: u32 = 0;

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<S2EventRecord>> {
    let n = s2event::parse_header(bytes)? as usize;
    if bytes.len() != s2event::HEADER_LEN.checked_add(n.checked_mul(s2event::REC_BYTES)?)? {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = s2event::HEADER_LEN;
    for _ in 0..n {
        let rec = s2event::decode_rec(bytes.get(off..off + s2event::REC_BYTES)?)?;
        out.push(rec);
        off += s2event::REC_BYTES;
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let events = parse_bin(bytes)?;
    let mut out = Vec::new();
    for event in events {
        let Some(epoch) = event.epoch_tdb else {
            continue;
        };
        out.push((epoch, 1.0, COMP_EVENTS));
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_EVENTS => Some("amanda_ii_upgoing_events"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(ra_deg: f32, dec_deg: f32, epoch_tdb: f64) -> S2EventRecord {
        S2EventRecord {
            ra_deg,
            dec_deg,
            sigma_arcsec: Some(8545.5),
            epoch_tdb: Some(epoch_tdb),
            energy: None,
            signalness: None,
            far: None,
            particle_root: s2event::ROOT_NEUTRINO,
        }
    }

    fn write_bin(events: &[S2EventRecord]) -> Vec<u8> {
        let mut out = Vec::new();
        s2event::write_header(&mut out, events.len() as u64);
        for event in events {
            let mut rec = [0u8; s2event::REC_BYTES];
            s2event::encode_rec(&mut rec, event);
            out.extend_from_slice(&rec);
        }
        out
    }

    #[test]
    fn roundtrip_keeps_direction_and_epoch_as_event_counts() {
        let events = [
            event(247.581, 2.2453, 1_000.0),
            event(14.75, 65.364, 2_000.0),
        ];
        let bytes = write_bin(&events);
        let parsed = parse_bin(&bytes).expect("events parse");
        assert_eq!(parsed.len(), 2);
        assert!((parsed[0].ra_deg - 247.581).abs() < 1e-3);
        assert_eq!(parsed[0].epoch_tdb, Some(1_000.0));

        let rows = parse_series(&bytes).expect("series parses");
        assert_eq!(
            rows,
            vec![(1_000.0, 1.0, COMP_EVENTS), (2_000.0, 1.0, COMP_EVENTS)]
        );
        assert_eq!(
            component_name(COMP_EVENTS),
            Some("amanda_ii_upgoing_events")
        );
        assert!(component_name(1).is_none());
    }

    #[test]
    fn truncated_and_unplaced_bins_stay_unread() {
        let bytes = write_bin(&[event(10.0, 1.0, 100.0)]);
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());

        let mut lost_epoch = event(10.0, 1.0, 100.0);
        lost_epoch.epoch_tdb = None;
        assert!(parse_series(&write_bin(&[lost_epoch])).is_none());
    }
}
