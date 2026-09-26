use super::*;

pub const MAGIC_ALWS: [u8; 4] = *b"ALWS";
pub const HEADER_BYTES: usize = 8;
pub const REC_BYTES: usize = 65;

const MASK_W1: u8 = 1 << 0;
const MASK_W2: u8 = 1 << 1;
const MASK_W3: u8 = 1 << 2;
const MASK_W4: u8 = 1 << 3;
const MASK_W3SNR: u8 = 1 << 4;
const MASK_W4SNR: u8 = 1 << 5;

pub const COMP_W1: u32 = 1;
pub const COMP_W2: u32 = 2;
pub const COMP_W3: u32 = 3;
pub const COMP_W4: u32 = 4;
pub const COMP_W3SNR: u32 = 5;
pub const COMP_W4SNR: u32 = 6;
pub const COMP_MAX: u32 = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AllwisePsd {
    pub ra: f64,
    pub dec: f64,
    pub w1mpro: Option<f64>,
    pub w2mpro: Option<f64>,
    pub w3mpro: Option<f64>,
    pub w4mpro: Option<f64>,
    pub w3snr: Option<f64>,
    pub w4snr: Option<f64>,
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_W1 => Some("allwise_w1_mag"),
        COMP_W2 => Some("allwise_w2_mag"),
        COMP_W3 => Some("allwise_w3_mag"),
        COMP_W4 => Some("allwise_w4_mag"),
        COMP_W3SNR => Some("allwise_w3_snr"),
        COMP_W4SNR => Some("allwise_w4_snr"),
        _ => None,
    }
}

pub fn component_value(src: &AllwisePsd, comp: u32) -> Option<f64> {
    match comp {
        COMP_W1 => src.w1mpro,
        COMP_W2 => src.w2mpro,
        COMP_W3 => src.w3mpro,
        COMP_W4 => src.w4mpro,
        COMP_W3SNR => src.w3snr,
        COMP_W4SNR => src.w4snr,
        _ => None,
    }
}

fn num_cell(v: Option<&JsonVal>) -> Option<f64> {
    match v {
        Some(JsonVal::Num(n)) if n.is_finite() => Some(*n),
        _ => None,
    }
}

pub fn parse_rows(rows: &JsonVal) -> Option<Vec<AllwisePsd>> {
    let JsonVal::Arr(arr) = rows else {
        return None;
    };
    let mut out = Vec::new();
    for row in arr {
        let JsonVal::Obj(map) = row else {
            continue;
        };
        let Some(ra) = num_cell(map.get("ra")) else {
            continue;
        };
        let Some(dec) = num_cell(map.get("dec")) else {
            continue;
        };
        if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
            continue;
        }
        let w1mpro = num_cell(map.get("w1mpro"));
        let w2mpro = num_cell(map.get("w2mpro"));
        let w3mpro = num_cell(map.get("w3mpro"));
        let w4mpro = num_cell(map.get("w4mpro"));
        if w1mpro.is_none() && w2mpro.is_none() && w3mpro.is_none() && w4mpro.is_none() {
            continue;
        }
        out.push(AllwisePsd {
            ra,
            dec,
            w1mpro,
            w2mpro,
            w3mpro,
            w4mpro,
            w3snr: num_cell(map.get("w3snr")),
            w4snr: num_cell(map.get("w4snr")),
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_votable(body: &str) -> Option<Vec<AllwisePsd>> {
    let rows = votable_to_json(body)?;
    parse_rows(&rows)
}

fn f64_at(data: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(data.get(off..off + 8)?.try_into().ok()?))
}

fn encode_rec(r: &AllwisePsd) -> [u8; REC_BYTES] {
    let mut rec = [0u8; REC_BYTES];
    rec[0..8].copy_from_slice(&r.ra.to_le_bytes());
    rec[8..16].copy_from_slice(&r.dec.to_le_bytes());
    let mut mask = 0u8;
    let cells: [(Option<f64>, u8, usize); 6] = [
        (r.w1mpro, MASK_W1, 16),
        (r.w2mpro, MASK_W2, 24),
        (r.w3mpro, MASK_W3, 32),
        (r.w4mpro, MASK_W4, 40),
        (r.w3snr, MASK_W3SNR, 48),
        (r.w4snr, MASK_W4SNR, 56),
    ];
    for (value, bit, off) in cells {
        match value {
            Some(v) => {
                mask |= bit;
                rec[off..off + 8].copy_from_slice(&v.to_le_bytes());
            }
            None => {
                rec[off..off + 8].copy_from_slice(&0.0f64.to_le_bytes());
            }
        }
    }
    rec[64] = mask;
    rec
}

fn decode_rec(data: &[u8]) -> Option<AllwisePsd> {
    if data.len() < REC_BYTES {
        return None;
    }
    let ra = f64_at(data, 0)?;
    let dec = f64_at(data, 8)?;
    if !ra.is_finite() || !dec.is_finite() {
        return None;
    }
    let mask = data[64];
    let cell = |off: usize, bit: u8| -> Option<f64> {
        if mask & bit == 0 {
            return None;
        }
        f64_at(data, off).filter(|v| v.is_finite())
    };
    Some(AllwisePsd {
        ra,
        dec,
        w1mpro: cell(16, MASK_W1),
        w2mpro: cell(24, MASK_W2),
        w3mpro: cell(32, MASK_W3),
        w4mpro: cell(40, MASK_W4),
        w3snr: cell(48, MASK_W3SNR),
        w4snr: cell(56, MASK_W4SNR),
    })
}

pub fn write_bin(records: &[AllwisePsd]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC_ALWS);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        out.extend_from_slice(&encode_rec(r));
    }
    out
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<AllwisePsd>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC_ALWS {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        out.push(decode_rec(data.get(base..base + REC_BYTES)?)?);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_VOTABLE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<VOTABLE version="1.3" xmlns="http://www.ivoa.net/xml/VOTable/v1.3">
  <RESOURCE type="results">
    <INFO name="QUERY_STATUS" value="OK"/>
    <TABLE>
      <FIELD name="ra" datatype="double" ID="col_0" unit="deg"/>
      <FIELD name="dec" datatype="double" ID="col_1" unit="deg"/>
      <FIELD name="w1mpro" datatype="float" ID="col_2" unit="mag"/>
      <FIELD name="w2mpro" datatype="float" ID="col_3" unit="mag"/>
      <FIELD name="w3mpro" datatype="float" ID="col_4" unit="mag"/>
      <FIELD name="w4mpro" datatype="float" ID="col_5" unit="mag"/>
      <FIELD name="w3snr" datatype="float" ID="col_6"/>
      <FIELD name="w4snr" datatype="float" ID="col_7"/>
      <DATA>
        <TABLEDATA>
          <TR>
            <TD>189.5907715</TD>
            <TD>-50.3575314</TD>
            <TD>13.615</TD>
            <TD>13.666</TD>
            <TD>12.826</TD>
            <TD>9.617</TD>
            <TD>0.7</TD>
            <TD>0.0</TD>
          </TR>
        </TABLEDATA>
      </DATA>
    </TABLE>
  </RESOURCE>
</VOTABLE>"#;

    #[test]
    fn parse_votable_carries_the_measured_row() {
        let rows = parse_votable(MEASURED_VOTABLE).expect("the measured VOTable 1.3 parses");
        assert_eq!(rows.len(), 1);
        let r = rows[0];
        assert_eq!(r.ra, 189.5907715);
        assert_eq!(r.dec, -50.3575314);
        assert_eq!(r.w1mpro, Some(13.615));
        assert_eq!(r.w4mpro, Some(9.617));
        assert_eq!(r.w3snr, Some(0.7));
        assert_eq!(r.w4snr, Some(0.0));
    }

    #[test]
    fn parse_votable_keeps_absent_cells_as_none() {
        let body = MEASURED_VOTABLE.replace("<TD>9.617</TD>", "<TD></TD>");
        let rows = parse_votable(&body).expect("the row stays with three mags");
        assert_eq!(rows[0].w4mpro, None);
        assert_eq!(rows[0].w1mpro, Some(13.615));
    }

    #[test]
    fn parse_votable_rejects_void_and_header_only() {
        assert!(parse_votable("").is_none());
        assert!(parse_votable("<VOTABLE version=\"1.3\"><RESOURCE/></VOTABLE>").is_none());
    }

    #[test]
    fn parse_rows_skips_out_of_range_and_magless_rows() {
        let rows = JsonVal::Arr(vec![
            JsonVal::Obj(HashMap::from([
                ("ra".to_string(), JsonVal::Num(400.0)),
                ("dec".to_string(), JsonVal::Num(0.0)),
                ("w1mpro".to_string(), JsonVal::Num(12.0)),
            ])),
            JsonVal::Obj(HashMap::from([
                ("ra".to_string(), JsonVal::Num(10.0)),
                ("dec".to_string(), JsonVal::Num(0.0)),
                ("w3snr".to_string(), JsonVal::Num(1.1)),
            ])),
            JsonVal::Obj(HashMap::from([
                ("ra".to_string(), JsonVal::Num(20.0)),
                ("dec".to_string(), JsonVal::Num(-45.0)),
                ("w1mpro".to_string(), JsonVal::Num(14.5)),
                ("w4snr".to_string(), JsonVal::Null),
            ])),
        ]);
        let out = parse_rows(&rows).expect("one placeable row stays");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].ra, 20.0);
        assert_eq!(out[0].w4snr, None);
    }

    fn fixture() -> Vec<AllwisePsd> {
        vec![
            AllwisePsd {
                ra: 189.5907715,
                dec: -50.3575314,
                w1mpro: Some(13.615),
                w2mpro: Some(13.666),
                w3mpro: Some(12.826),
                w4mpro: Some(9.617),
                w3snr: Some(0.7),
                w4snr: Some(0.0),
            },
            AllwisePsd {
                ra: 1.5,
                dec: 2.5,
                w1mpro: Some(16.2),
                w2mpro: None,
                w3mpro: None,
                w4mpro: None,
                w3snr: None,
                w4snr: None,
            },
        ]
    }

    #[test]
    fn bin_roundtrip_carries_all_rows_and_absent_masks() {
        let recs = fixture();
        let bytes = write_bin(&recs);
        let parsed = parse_bin(&bytes).expect("roundtrip parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra, recs[0].ra);
        assert_eq!(parsed[0].w4snr, Some(0.0));
        assert_eq!(parsed[1].w2mpro, None);
        assert_eq!(parsed[1].w1mpro, Some(16.2));
    }

    #[test]
    fn bin_rejects_bad_magic_and_truncation() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&fixture());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert!(parse_bin(&bad).is_none());
    }

    #[test]
    fn bin_rejects_nonfinite_cell_behind_a_set_mask() {
        let bytes = write_bin(&fixture());
        let mut bad = bytes;
        let off = HEADER_BYTES + 24;
        bad[off..off + 8].copy_from_slice(&f64::INFINITY.to_le_bytes());
        assert!(parse_bin(&bad).is_none());
    }

    #[test]
    fn component_names_and_values_bind_the_six_channels() {
        let src = fixture()[0];
        assert_eq!(component_name(COMP_W1), Some("allwise_w1_mag"));
        assert_eq!(component_name(COMP_W4SNR), Some("allwise_w4_snr"));
        assert_eq!(component_name(COMP_MAX + 1), None);
        assert_eq!(component_value(&src, COMP_W1), Some(13.615));
        assert_eq!(component_value(&src, COMP_W4SNR), Some(0.0));
        assert_eq!(component_value(&fixture()[1], COMP_W3), None);
    }
}
