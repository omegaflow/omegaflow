use crate::archivar::geo::GeoRec;

pub const COMP_AIR: u32 = 1;
pub const COMP_WATER: u32 = 2;
pub const COMP_SOIL: u32 = 3;

pub const POLLUTANTS: &[(u32, &str, &str)] = &[
    (1, "ANTHRACENE", "eionet_cdr_anthracene"),
    (2, "AS AND COMPOUNDS", "eionet_cdr_as_and_compounds"),
    (3, "BENZENE", "eionet_cdr_benzene"),
    (4, "BENZO(G,H,I)PERYLENE", "eionet_cdr_benzo_g_h_i_perylene"),
    (5, "CD AND COMPOUNDS", "eionet_cdr_cd_and_compounds"),
    (6, "CH4", "eionet_cdr_ch4"),
    (7, "CHLORIDES", "eionet_cdr_chlorides"),
    (
        8,
        "CHLORINE AND INORGANIC COMPOUNDS",
        "eionet_cdr_chlorine_and_inorganic_compounds",
    ),
    (
        9,
        "CHLORO-ALKANES (C10-13)",
        "eionet_cdr_chloro_alkanes_c10_13",
    ),
    (10, "CO", "eionet_cdr_co"),
    (11, "CO2", "eionet_cdr_co2"),
    (12, "CU AND COMPOUNDS", "eionet_cdr_cu_and_compounds"),
    (13, "CYANIDES", "eionet_cdr_cyanides"),
    (14, "DEHP", "eionet_cdr_dehp"),
    (
        15,
        "DICHLOROETHANE-1,2 (DCE)",
        "eionet_cdr_dichloroethane_1_2_dce",
    ),
    (
        16,
        "DICHLOROMETHANE (DCM)",
        "eionet_cdr_dichloromethane_dcm",
    ),
    (17, "DIURON", "eionet_cdr_diuron"),
    (18, "ETHYLBENZENE", "eionet_cdr_ethylbenzene"),
    (19, "FLUORANTHENE", "eionet_cdr_fluoranthene"),
    (20, "FLUORIDES", "eionet_cdr_fluorides"),
    (
        21,
        "HALOGENATED ORGANIC COMPOUNDS",
        "eionet_cdr_halogenated_organic_compounds",
    ),
    (22, "HCN", "eionet_cdr_hcn"),
    (23, "HFCS", "eionet_cdr_hfcs"),
    (24, "HG AND COMPOUNDS", "eionet_cdr_hg_and_compounds"),
    (25, "ISOPROTURON", "eionet_cdr_isoproturon"),
    (26, "N2O", "eionet_cdr_n2o"),
    (27, "NAPHTHALENE", "eionet_cdr_naphthalene"),
    (28, "NH3", "eionet_cdr_nh3"),
    (29, "NI AND COMPOUNDS", "eionet_cdr_ni_and_compounds"),
    (30, "NMVOC", "eionet_cdr_nmvoc"),
    (31, "NOX", "eionet_cdr_nox"),
    (32, "NP/NPES", "eionet_cdr_np_npes"),
    (33, "PB AND COMPOUNDS", "eionet_cdr_pb_and_compounds"),
    (
        34,
        "PCDD+PCDF (DIOXINS+FURANS)",
        "eionet_cdr_pcdd_pcdf_dioxins_furans",
    ),
    (
        35,
        "PENTACHLOROPHENOL (PCP)",
        "eionet_cdr_pentachlorophenol_pcp",
    ),
    (36, "PHENOLS", "eionet_cdr_phenols"),
    (37, "PM10", "eionet_cdr_pm10"),
    (38, "SOX", "eionet_cdr_sox"),
    (39, "TOLUENE", "eionet_cdr_toluene"),
    (40, "TOTAL - NITROGEN", "eionet_cdr_total_nitrogen"),
    (
        41,
        "TOTAL ORGANIC CARBON (TOC)",
        "eionet_cdr_total_organic_carbon_toc",
    ),
    (42, "TOTAL - PHOSPHORUS", "eionet_cdr_total_phosphorus"),
    (43, "TRICHLOROMETHANE", "eionet_cdr_trichloromethane"),
    (44, "XYLENES", "eionet_cdr_xylenes"),
    (45, "ZN AND COMPOUNDS", "eionet_cdr_zn_and_compounds"),
];

pub const COMP_MAX: u32 = (45 << 2) | 3;

pub fn medium_id(name: &str) -> Option<u32> {
    match name.trim() {
        "AIR" => Some(COMP_AIR),
        "WATER" => Some(COMP_WATER),
        "SOIL" => Some(COMP_SOIL),
        _ => None,
    }
}

pub fn medium_of(comp: u32) -> Option<&'static str> {
    match comp & 0x3 {
        COMP_AIR => Some("air"),
        COMP_WATER => Some("water"),
        COMP_SOIL => Some("soil"),
        _ => None,
    }
}

pub fn pollutant_id(code: &str) -> Option<u32> {
    let code = code.trim();
    POLLUTANTS
        .iter()
        .find(|(_, c, _)| *c == code)
        .map(|(id, _, _)| *id)
}

pub fn pollutant_of(comp: u32) -> Option<&'static str> {
    let id = comp >> 2;
    POLLUTANTS
        .iter()
        .find(|(i, _, _)| *i == id)
        .map(|(_, c, _)| *c)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    let id = comp >> 2;
    POLLUTANTS
        .iter()
        .find(|(i, _, _)| *i == id)
        .map(|(_, _, n)| *n)
}

fn find_open(text: &str, tag: &str) -> Option<usize> {
    let needle = format!("<{tag}");
    let mut from = 0usize;
    while let Some(rel) = text[from..].find(&needle) {
        let i = from + rel;
        match text.as_bytes().get(i + 1 + tag.len()).copied() {
            Some(b'>') | Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') => return Some(i),
            _ => from = i + 1,
        }
    }
    None
}

fn tag_body<'a>(text: &'a str, tag: &str) -> Option<&'a str> {
    let open = find_open(text, tag)?;
    let after = text.get(open + 1 + tag.len()..)?;
    let gt = after.find('>')?;
    let body = after.get(gt + 1..)?;
    let close = format!("</{tag}>");
    let end = body.find(&close)?;
    body.get(..end)
}

fn segments<'a>(text: &'a str, tag: &str) -> Vec<&'a str> {
    let close = format!("</{tag}>");
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(open) = find_open(rest, tag) {
        let Some(after) = rest.get(open + 1 + tag.len()..) else {
            break;
        };
        let Some(gt) = after.find('>') else {
            break;
        };
        let body = &after[gt + 1..];
        let Some(end) = body.find(&close) else {
            break;
        };
        out.push(&body[..end]);
        rest = &body[end + close.len()..];
    }
    out
}

fn coord(body: &str) -> Option<(f64, f64)> {
    let lon = tag_body(body, "LongitudeMeasure")?
        .trim()
        .parse::<f64>()
        .ok()?;
    let lat = tag_body(body, "LatitudeMeasure")?
        .trim()
        .parse::<f64>()
        .ok()?;
    if !lon.is_finite() || !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return None;
    }
    if !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    Some((lon, lat))
}

pub fn parse_report(bytes: &[u8]) -> Option<Vec<GeoRec>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let text = text.trim_start_matches('\u{feff}');
    find_open(text, "PollutantReleaseAndTransferReport")?;
    let year = tag_body(text, "ReportingYear")?
        .trim()
        .parse::<i64>()
        .ok()?;
    let days = crate::lsk::days_from_civil(year, 1, 1)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let t = lsk.unix_to_tdb(days as f64 * 86400.0)?;
    let mut out = Vec::new();
    for facility in segments(text, "FacilityReport") {
        let Some((lon, lat)) = coord(facility) else {
            continue;
        };
        for release in segments(facility, "PollutantRelease") {
            let Some(medium) = tag_body(release, "MediumCode").and_then(medium_id) else {
                continue;
            };
            let Some(id) = tag_body(release, "PollutantCode").and_then(pollutant_id) else {
                continue;
            };
            let Some(val) =
                tag_body(release, "TotalQuantity").and_then(|b| b.trim().parse::<f64>().ok())
            else {
                continue;
            };
            if !val.is_finite() || val < 0.0 {
                continue;
            }
            out.push(GeoRec {
                t,
                lat,
                lon,
                alt: 0.0,
                freq: crate::spectral::SPECTRAL_NO_BAND,
                bin_width: crate::spectral::SPECTRAL_NO_BAND,
                val,
                comp: (id << 2) | medium,
                station: 0,
            });
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::geo::{parse_bin, write_bin};

    const FIXTURE: &str = "\u{feff}<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>
<PollutantReleaseAndTransferReport xmlns=\"urn:eu:com:env:prtr:data:standard:2\">
  <ReportingYear>2017</ReportingYear>
  <CountryID>AT</CountryID>
  <CoordinateSystemID>EPSG:4326</CoordinateSystemID>
  <FacilityReport>
    <NationalID>20000.00989</NationalID>
    <FacilityName>Biogas ZEMKA GmbH</FacilityName>
    <GeographicalCoordinate>
      <LongitudeMeasure>12.788444</LongitudeMeasure>
      <LatitudeMeasure>47.283943</LatitudeMeasure>
    </GeographicalCoordinate>
    <PollutantRelease>
      <MediumCode>AIR</MediumCode>
      <PollutantCode>CO2</PollutantCode>
      <TotalQuantity unitCode=\"KGM\">355000000</TotalQuantity>
      <AccidentalQuantity unitCode=\"KGM\">0.00</AccidentalQuantity>
    </PollutantRelease>
    <PollutantRelease>
      <MediumCode>WATER</MediumCode>
      <PollutantCode>TOTAL - NITROGEN</PollutantCode>
      <TotalQuantity unitCode=\"KGM\">1200</TotalQuantity>
    </PollutantRelease>
  </FacilityReport>
  <FacilityReport>
    <NationalID>20000.00001</NationalID>
    <FacilityName>No coordinate facility</FacilityName>
    <PollutantRelease>
      <MediumCode>AIR</MediumCode>
      <PollutantCode>CO</PollutantCode>
      <TotalQuantity unitCode=\"KGM\">1</TotalQuantity>
    </PollutantRelease>
  </FacilityReport>
  <FacilityReport>
    <NationalID>20000.00002</NationalID>
    <FacilityName>No quantity facility</FacilityName>
    <GeographicalCoordinate>
      <LongitudeMeasure>11.875</LongitudeMeasure>
      <LatitudeMeasure>47.43639</LatitudeMeasure>
    </GeographicalCoordinate>
    <PollutantRelease>
      <MediumCode>AIR</MediumCode>
      <PollutantCode>CO</PollutantCode>
    </PollutantRelease>
  </FacilityReport>
</PollutantReleaseAndTransferReport>";

    #[test]
    fn reads_the_measured_shape_and_skips_absent_fields() {
        let records = parse_report(FIXTURE.as_bytes()).expect("the report parses");
        assert_eq!(records.len(), 2);
        let co2 = records
            .iter()
            .find(|r| pollutant_of(r.comp) == Some("CO2"))
            .expect("CO2 present");
        assert_eq!(co2.comp, (11 << 2) | COMP_AIR);
        assert_eq!(co2.val, 355000000.0);
        assert_eq!(co2.lat, 47.283943);
        assert_eq!(co2.lon, 12.788444);
        assert_eq!(medium_of(co2.comp), Some("air"));
        let n = records
            .iter()
            .find(|r| pollutant_of(r.comp) == Some("TOTAL - NITROGEN"))
            .expect("TOTAL - NITROGEN present");
        assert_eq!(n.comp, (40 << 2) | COMP_WATER);
        assert_eq!(medium_of(n.comp), Some("water"));
    }

    #[test]
    fn a_foreign_body_is_void() {
        assert!(parse_report(b"not xml").is_none());
        assert!(parse_report(b"<ReportData></ReportData>").is_none());
    }

    #[test]
    fn component_names_come_from_the_pollutant_table() {
        assert_eq!(component_name((11 << 2) | COMP_AIR), Some("eionet_cdr_co2"));
        assert_eq!(component_name((31 << 2) | COMP_AIR), Some("eionet_cdr_nox"));
        assert_eq!(component_name(999), None);
        assert_eq!(pollutant_id("PCDD+PCDF (DIOXINS+FURANS)"), Some(34));
        assert_eq!(medium_id("AIR"), Some(COMP_AIR));
        assert_eq!(medium_id("MAGMA"), None);
        assert!(COMP_MAX >= (45 << 2) | 3);
    }

    #[test]
    fn bin_roundtrip_carries_every_geo_slot() {
        let records = parse_report(FIXTURE.as_bytes()).expect("the report parses");
        let magic = crate::archivar::geo::magic_of("eionet_cdr").expect("registered format");
        let bytes = write_bin(magic, &records);
        let parsed = parse_bin(magic, &bytes).expect("roundtrip parses");
        assert_eq!(parsed.len(), records.len());
        for (a, b) in parsed.iter().zip(records.iter()) {
            assert_eq!(a.t, b.t);
            assert_eq!(a.lat, b.lat);
            assert_eq!(a.lon, b.lon);
            assert_eq!(a.val, b.val);
            assert_eq!(a.comp, b.comp);
        }
        assert!(parse_bin(crate::archivar::geo::MAGIC_BGR, &bytes).is_none());
    }
}
