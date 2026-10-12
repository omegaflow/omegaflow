use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://service.iris.edu/fdsnws/station/1/query";

struct Station {
    network: String,
    code: String,
    latitude: String,
    longitude: String,
}

pub fn fdsn_lines(query: &str, max: usize) -> Vec<String> {
    let needle = query.trim();
    if needle.is_empty() {
        return vec![format!("absent — fdsn carries no station: {query}")];
    }
    let url = format!(
        "{}?level=station&station={}*&nodata=404&format=text",
        ENDPOINT,
        urlencode(needle)
    );
    let fetched = match get(&url, &[], "60") {
        Some(f) => f,
        None => return vec!["pending — no network".to_string()],
    };
    match fetched.status {
        Some(200) => {}
        Some(404) => return vec![format!("absent — fdsn carries no station: {query}")],
        Some(status) => return vec![format!("pending — fdsn HTTP {}", status)],
        None => return vec!["pending — no network".to_string()],
    }
    let stations = parse_stations(&fetched.body);
    if stations.is_empty() {
        return vec![format!("absent — fdsn carries no station: {query}")];
    }
    stations
        .iter()
        .take(max)
        .map(|station| station_line(station, &url))
        .collect()
}

fn parse_stations(body: &str) -> Vec<Station> {
    let mut stations = Vec::new();
    for line in body.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 4 {
            continue;
        }
        stations.push(Station {
            network: fields[0].trim().to_string(),
            code: fields[1].trim().to_string(),
            latitude: fields[2].trim().to_string(),
            longitude: fields[3].trim().to_string(),
        });
    }
    stations
}

fn station_line(station: &Station, url: &str) -> String {
    let mut line = format!("url {}", url);
    for (key, value) in [
        ("network", station.network.as_str()),
        ("station", station.code.as_str()),
        ("lat", station.latitude.as_str()),
        ("lon", station.longitude.as_str()),
    ] {
        if !value.is_empty() {
            line.push_str(&format!("\t{}: {}", key, value));
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANMO: &str = "#Network | Station | Latitude | Longitude | Elevation | SiteName | StartTime | EndTime \n\
IU|ANMO|34.9459|-106.4572|1850.0|Albuquerque, New Mexico, USA|1989-08-29T00:00:00.0000|2000-10-19T16:00:00.0000\n\
IU|ANMO|34.9502|-106.4602|1839.0|Albuquerque, New Mexico, USA|2000-10-19T16:00:00.0000|2002-11-19T21:07:00.0000\n\
IU|ANMO|34.94591|-106.4572|1820.0|Albuquerque, New Mexico, USA|2002-11-19T21:07:00.0000|\n\
SR|ANMO|34.946201|-106.456703|1740.0|Albuquerque, New Mexico, USA|1974-08-28T00:00:00.0000|1989-08-30T00:00:00.0000\n\
SY|ANMO|34.9459|-106.457199|1750.0|ANMO synthetic|1900-01-01T00:00:00.0000|\n";

    #[test]
    fn parses_the_captured_station_rows_and_skips_the_header() {
        let stations = parse_stations(ANMO);
        assert_eq!(stations.len(), 5);
        assert_eq!(stations[0].network, "IU");
        assert_eq!(stations[0].code, "ANMO");
        assert_eq!(stations[0].latitude, "34.9459");
        assert_eq!(stations[0].longitude, "-106.4572");
        assert_eq!(stations[4].network, "SY");
    }

    #[test]
    fn an_empty_body_carries_no_stations() {
        assert!(parse_stations("").is_empty());
        assert!(parse_stations("#Network | Station | Latitude | Longitude \n").is_empty());
    }

    #[test]
    fn station_line_carries_the_url_and_the_measured_fields() {
        let stations = parse_stations(ANMO);
        let url = "https://service.iris.edu/fdsnws/station/1/query?level=station&station=ANMO*&nodata=404&format=text";
        assert_eq!(
            station_line(&stations[0], url),
            "url https://service.iris.edu/fdsnws/station/1/query?level=station&station=ANMO*&nodata=404&format=text\tnetwork: IU\tstation: ANMO\tlat: 34.9459\tlon: -106.4572"
                .to_string()
        );
    }
}
