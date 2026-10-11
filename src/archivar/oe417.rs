pub const MAGIC: [u8; 3] = [0xCF, 0x86, 0x09];
pub const FRAME_HEADER: usize = 19;
pub const SLOTS: usize = 26;
pub const RECORD_BYTES: usize = SLOTS * 8;

const SLOT_VAL: usize = 3;
const SLOT_EPOCH: usize = 4;
const SLOT_CHANNEL: usize = 9;

pub const COMP_DEMAND_LOSS_MW: u32 = 1;
pub const COMP_CUSTOMERS_AFFECTED: u32 = 2;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_DEMAND_LOSS_MW => Some("oe417_demand_loss_mw"),
        COMP_CUSTOMERS_AFFECTED => Some("oe417_customers_affected"),
        _ => None,
    }
}

fn slot(bytes: &[u8], off: usize, k: usize) -> Option<f64> {
    let p = off + k * 8;
    Some(f64::from_le_bytes(bytes.get(p..p + 8)?.try_into().ok()?))
}

fn component_of(channel: f64) -> Option<u32> {
    if !channel.is_finite() || channel < 1.0 || channel > 2.0 {
        return None;
    }
    let comp = channel as u32;
    if comp as f64 != channel {
        return None;
    }
    Some(comp)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < FRAME_HEADER || bytes[0..3] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(15..19)?.try_into().ok()?) as usize;
    if bytes.len() != FRAME_HEADER + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = FRAME_HEADER;
    for _ in 0..n {
        let val = slot(bytes, off, SLOT_VAL)?;
        let epoch = slot(bytes, off, SLOT_EPOCH)?;
        let channel = slot(bytes, off, SLOT_CHANNEL)?;
        off += RECORD_BYTES;
        let Some(comp) = component_of(channel) else {
            continue;
        };
        if !epoch.is_finite() || !val.is_finite() {
            continue;
        }
        out.push((epoch, val, comp));
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(epoch: f64, val: f64, channel: f64) -> [f64; 26] {
        let mut r = [0.0f64; 26];
        r[SLOT_VAL] = val;
        r[SLOT_EPOCH] = epoch;
        r[SLOT_CHANNEL] = channel;
        r
    }

    fn frame(records: &[[f64; 26]]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&0.0f64.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for r in records {
            for v in r {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    #[test]
    fn a_two_channel_frame_roundtrips() {
        let records = [
            record(1.673_500_800e9, 41000.0, 2.0),
            record(1.673_500_800e9, 0.0, 1.0),
        ];
        let bytes = frame(&records);
        assert_eq!(bytes.len(), FRAME_HEADER + 2 * RECORD_BYTES);
        let series = parse_series(&bytes).expect("frame parses");
        assert_eq!(
            series,
            vec![
                (1.673_500_800e9, 41000.0, COMP_CUSTOMERS_AFFECTED),
                (1.673_500_800e9, 0.0, COMP_DEMAND_LOSS_MW)
            ]
        );
    }

    #[test]
    fn a_foreign_or_truncated_frame_is_void() {
        assert!(parse_series(b"XX").is_none());
        let good = frame(&[record(1.0, 1.0, 1.0)]);
        assert!(parse_series(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn component_names_carry_the_read_site() {
        assert_eq!(
            component_name(COMP_DEMAND_LOSS_MW),
            Some("oe417_demand_loss_mw")
        );
        assert_eq!(
            component_name(COMP_CUSTOMERS_AFFECTED),
            Some("oe417_customers_affected")
        );
        assert_eq!(component_name(3), None);
    }
}
