#[derive(Clone, Debug)]
pub struct UvReading {
    pub id: String,
    pub name: Option<String>,
    pub index: f64,
    pub utcdatetime: Option<String>,
    pub status: Option<String>,
}

fn xml_elem_text(block: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let start = block.find(&open)? + open.len();
    let close = format!("</{}>", tag);
    let end = block[start..].find(&close)? + start;
    Some(block[start..end].trim().to_string())
}

fn is_open(rest: &str) -> bool {
    matches!(
        rest.as_bytes().first(),
        Some(b'>') | Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')
    )
}

fn attr_value(tag: &str, name: &str) -> Option<String> {
    let gt = tag.find('>')?;
    let open = &tag[..gt];
    let key = format!("{}=\"", name);
    let start = open.find(&key)? + key.len();
    let end = open[start..].find('"')? + start;
    Some(open[start..end].to_string())
}

pub fn parse_uv_xml(body: &str) -> Vec<UvReading> {
    let mut readings = Vec::new();
    for loc in body.split("<location").skip(1) {
        if !is_open(loc) {
            continue;
        }
        let id = match attr_value(loc, "id") {
            Some(v) => v,
            None => continue,
        };
        let Some(gt) = loc.find('>') else {
            continue;
        };
        let block = &loc[gt + 1..];
        let Some(index) = xml_elem_text(block, "index")
            .and_then(|t| t.parse::<f64>().ok())
            .filter(|v| v.is_finite())
        else {
            continue;
        };
        readings.push(UvReading {
            id,
            name: xml_elem_text(block, "name"),
            index,
            utcdatetime: xml_elem_text(block, "utcdatetime"),
            status: xml_elem_text(block, "status"),
        });
    }
    readings
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<stations>
  <location id="Adelaide">
    <name>adl</name>
    <index>1.3</index>
    <time>4:05 PM</time>
    <date>18/09/2026</date>
    <fulldate>Friday, 18 September 2026</fulldate>
    <utcdatetime>2026/09/18 06:35</utcdatetime>
    <status>ok</status>
  </location>
  <location id="Alice Springs">
    <name>asp</name>
    <index>7.8</index>
    <time>3:35 PM</time>
    <date>18/09/2026</date>
    <fulldate>Friday, 18 September 2026</fulldate>
    <utcdatetime>2026/09/18 06:05</utcdatetime>
    <status>ok</status>
  </location>
</stations>"#;

    #[test]
    fn parses_two_locations_with_measured_fields() {
        let r = parse_uv_xml(MEASURED);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].id, "Adelaide");
        assert_eq!(r[0].name.as_deref(), Some("adl"));
        assert_eq!(r[0].index, 1.3);
        assert_eq!(r[0].utcdatetime.as_deref(), Some("2026/09/18 06:35"));
        assert_eq!(r[0].status.as_deref(), Some("ok"));
        assert_eq!(r[1].id, "Alice Springs");
        assert_eq!(r[1].name.as_deref(), Some("asp"));
        assert_eq!(r[1].index, 7.8);
        assert_eq!(r[1].utcdatetime.as_deref(), Some("2026/09/18 06:05"));
    }

    #[test]
    fn missing_index_is_skipped_not_zero() {
        let body = r#"<stations>
  <location id="NoIndex"><name>nix</name><utcdatetime>2026/09/18 06:35</utcdatetime><status>ok</status></location>
  <location id="Good"><name>gud</name><index>2.0</index><status>ok</status></location>
</stations>"#;
        let r = parse_uv_xml(body);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "Good");
        assert_eq!(r[0].index, 2.0);
    }

    #[test]
    fn measured_zero_index_is_kept() {
        let body = r#"<stations><location id="Night"><name>ngt</name><index>0.0</index><status>ok</status></location></stations>"#;
        let r = parse_uv_xml(body);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "Night");
        assert_eq!(r[0].index, 0.0);
    }
}
