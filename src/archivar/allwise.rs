use super::*;

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
}
