

#[derive(Clone, Debug)]
pub struct Sensitivity {
    pub value: f64,
    pub frequency_hz: f64,
    pub input_units: String,
    pub output_units: String,
}

#[derive(Clone, Debug)]
pub struct Channel {
    pub code: String,
    pub location: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation_m: Option<f64>,
    pub sample_rate_hz: f64,
    pub sensitivity: Option<Sensitivity>,
    pub stages: usize,
}

#[derive(Clone, Debug)]
pub struct Station {
    pub code: String,
    pub description: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation_m: Option<f64>,
    pub start: Option<String>,
    pub channels: Vec<Channel>,
}

#[derive(Clone, Debug)]
pub struct Network {
    pub code: String,
    pub description: Option<String>,
    pub stations: Vec<Station>,
}

fn element<'a>(s: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}>");
    let start = s.find(&open)? + open.len();
    let close = format!("</{tag}>");
    let end = s[start..].find(&close)? + start;
    Some(&s[start..end])
}

fn text(s: &str, tag: &str) -> Option<String> {
    element(s, tag).map(|v| v.trim().to_string())
}

fn num(s: &str, tag: &str) -> Option<f64> {
    text(s, tag)?.parse::<f64>().ok()
}

fn attr(tag_line: &str, key: &str) -> Option<String> {
    let needle = format!("{key}=\"");
    let p = tag_line.find(&needle)? + needle.len();
    let rest = &tag_line[p..];
    let q = rest.find('"')?;
    Some(rest[..q].to_string())
}

fn blocks<'a>(s: &'a str, tag: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let open = format!("<{tag} ");
    let close = format!("</{tag}>");
    let mut rest = s;
    while let Some(p) = rest.find(&open) {
        let after = &rest[p..];
        let Some(e) = after.find(&close) else {
            break;
        };
        out.push(&after[..e]);
        rest = &after[e + close.len()..];
    }
    out
}

fn sensitivity_of(channel: &str) -> Option<Sensitivity> {
    let resp = element(channel, "Response")?;
    let sens = element(resp, "InstrumentSensitivity")?;
    let value = num(sens, "Value")?;
    let frequency_hz = num(sens, "Frequency")?;
    let input_units = element(sens, "InputUnits").and_then(|u| text(u, "Name"))?;
    let output_units = element(sens, "OutputUnits").and_then(|u| text(u, "Name"))?;
    Some(Sensitivity {
        value,
        frequency_hz,
        input_units,
        output_units,
    })
}

pub fn parse(xml: &str) -> Option<Network> {
    let net = blocks(xml, "Network").into_iter().next()?;
    let code = attr(net, "code")?;
    let description = text(net, "Description");
    let mut stations = Vec::new();
    for st in blocks(net, "Station") {
        let Some(code) = attr(st, "code") else {
            continue;
        };
        let Some(latitude) = num(st, "Latitude") else {
            continue;
        };
        let Some(longitude) = num(st, "Longitude") else {
            continue;
        };
        let elevation_m = num(st, "Elevation");
        let start = if st.starts_with("<Station ") {
            attr(st, "startDate")
        } else {
            None
        };
        let mut channels = Vec::new();
        for ch in blocks(st, "Channel") {
            let Some(code) = attr(ch, "code") else {
                continue;
            };
            let mut location = String::new();
            if let Some(v) = attr(ch, "locationCode") {
                location = v;
            }
            let Some(sample_rate_hz) = num(ch, "SampleRate") else {
                continue;
            };
            let stages = ch.matches("<Stage ").count();
            channels.push(Channel {
                code,
                location,
                latitude: num(ch, "Latitude"),
                longitude: num(ch, "Longitude"),
                elevation_m: num(ch, "Elevation"),
                sample_rate_hz,
                sensitivity: sensitivity_of(ch),
                stages,
            });
        }
        stations.push(Station {
            code,
            description: text(st, "Description"),
            latitude,
            longitude,
            elevation_m,
            start,
            channels,
        });
    }
    if stations.is_empty() {
        return None;
    }
    Some(Network {
        code,
        description,
        stations,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    const ANMO: &str = include_str!("kernels/stationxml/IU_ANMO_BHZ.xml");

    #[test]
    fn the_iris_document_parses_to_network_station_channel_and_response() {
        let net = parse(ANMO).expect("station xml");
        assert_eq!(net.code, "IU");
        assert!(!net.stations.is_empty());
        let st = net
            .stations
            .iter()
            .find(|s| s.code == "ANMO")
            .expect("ANMO");
        assert!((st.latitude - 34.9459).abs() < 1e-6, "{}", st.latitude);
        assert!((st.longitude + 106.4572).abs() < 1e-6, "{}", st.longitude);
        let ch = st
            .channels
            .iter()
            .find(|c| c.code == "BHZ" && c.location == "00")
            .expect("BHZ.00");
        assert!((ch.sample_rate_hz - 20.0).abs() < 1e-9, "{}", ch.sample_rate_hz);
        assert!(ch.stages >= 1, "stages {}", ch.stages);
        let sens = ch.sensitivity.as_ref().expect("sensitivity");
        assert!((sens.value - 8.64679e8).abs() < 1.0, "{}", sens.value);
        assert!((sens.frequency_hz - 0.02).abs() < 1e-9, "{}", sens.frequency_hz);
        assert_eq!(sens.input_units, "m/s");
        assert_eq!(sens.output_units, "counts");
    }

    #[test]
    fn a_document_without_a_network_is_absent() {
        assert!(parse("<root></root>").is_none());
        assert!(parse("").is_none());
    }

    const GEOFON: &str = include_str!("kernels/stationxml/GE_APE_BHZ.xml");
    const RESIF: &str = include_str!("kernels/stationxml/FR_ZELS_BHZ.xml");

    #[test]
    fn three_operators_supply_the_same_fields() {
        let iris = parse(ANMO).expect("IRIS");
        let geofon = parse(GEOFON).expect("GEOFON");
        let resif = parse(RESIF).expect("RESIF");
        assert_eq!(iris.code, "IU");
        assert_eq!(geofon.code, "GE");
        assert_eq!(resif.code, "FR");
        let ape = geofon
            .stations
            .iter()
            .find(|s| s.code == "APE")
            .expect("GE.APE");
        assert!((ape.latitude - 37.07274).abs() < 1e-6, "{}", ape.latitude);
        assert!((ape.longitude - 25.52301).abs() < 1e-6, "{}", ape.longitude);
        let ape_bhz = ape.channels.iter().find(|c| c.code == "BHZ").expect("BHZ");
        assert!((ape_bhz.sample_rate_hz - 20.0).abs() < 1e-9);
        let ape_sens = ape_bhz.sensitivity.as_ref().expect("sensitivity");
        assert!((ape_sens.value - 5.88e8).abs() < 1.0, "{}", ape_sens.value);
        let zels = resif
            .stations
            .iter()
            .find(|s| s.code == "ZELS")
            .expect("FR.ZELS");
        assert!((zels.latitude - 48.302306).abs() < 1e-6, "{}", zels.latitude);
        let zels_bhz = zels.channels.iter().find(|c| c.code == "BHZ").expect("BHZ");
        assert!((zels_bhz.sample_rate_hz - 20.0).abs() < 1e-9);
        let zels_sens = zels_bhz.sensitivity.as_ref().expect("sensitivity");
        assert!(
            (zels_sens.value - 5.04364575e8).abs() < 1.0,
            "{}",
            zels_sens.value
        );
    }
}
