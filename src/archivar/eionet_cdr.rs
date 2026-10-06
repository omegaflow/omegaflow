use crate::archivar::geo::GeoRec;

pub const COMP_AIR: u32 = 1;
pub const COMP_WATER: u32 = 2;
pub const COMP_SOIL: u32 = 3;

pub const POLLUTANTS: &[(u32, &str, &str)] = &[
    (1, "ALACHLOR", "eionet_cdr_alachlor"),
    (2, "ALDRIN", "eionet_cdr_aldrin"),
    (3, "ANTHRACENE", "eionet_cdr_anthracene"),
    (4, "AS AND COMPOUNDS", "eionet_cdr_as_and_compounds"),
    (5, "ASBESTOS", "eionet_cdr_asbestos"),
    (6, "ATRAZINE", "eionet_cdr_atrazine"),
    (7, "BENZENE", "eionet_cdr_benzene"),
    (8, "BENZO(G,H,I)PERYLENE", "eionet_cdr_benzo_g_h_i_perylene"),
    (
        9,
        "BROMINATED DIPHENYLETHER",
        "eionet_cdr_brominated_diphenylether",
    ),
    (10, "CD AND COMPOUNDS", "eionet_cdr_cd_and_compounds"),
    (11, "CFCS", "eionet_cdr_cfcs"),
    (12, "CH4", "eionet_cdr_ch4"),
    (13, "CHLORDECONE", "eionet_cdr_chlordecone"),
    (14, "CHLORFENVINPHOS", "eionet_cdr_chlorfenvinphos"),
    (15, "CHLORIDES", "eionet_cdr_chlorides"),
    (
        16,
        "CHLORINE AND INORGANIC COMPOUNDS",
        "eionet_cdr_chlorine_and_inorganic_compounds",
    ),
    (
        17,
        "CHLORO-ALKANES (C10-13)",
        "eionet_cdr_chloro_alkanes_c10_13",
    ),
    (18, "CHLORPYRIFOS", "eionet_cdr_chlorpyrifos"),
    (19, "CLORDANE", "eionet_cdr_clordane"),
    (20, "CO", "eionet_cdr_co"),
    (21, "CO2", "eionet_cdr_co2"),
    (22, "CO2 EXCL BIOMASS", "eionet_cdr_co2_excl_biomass"),
    (23, "CR AND COMPOUNDS", "eionet_cdr_cr_and_compounds"),
    (24, "CU AND COMPOUNDS", "eionet_cdr_cu_and_compounds"),
    (25, "CYANIDES", "eionet_cdr_cyanides"),
    (26, "DDT", "eionet_cdr_ddt"),
    (27, "DEHP", "eionet_cdr_dehp"),
    (
        28,
        "DICHLOROETHANE-1,2 (DCE)",
        "eionet_cdr_dichloroethane_1_2_dce",
    ),
    (
        29,
        "DICHLOROMETHANE (DCM)",
        "eionet_cdr_dichloromethane_dcm",
    ),
    (30, "DIELDRIN", "eionet_cdr_dieldrin"),
    (31, "DIURON", "eionet_cdr_diuron"),
    (32, "ENDOSULPHAN", "eionet_cdr_endosulphan"),
    (33, "ENDRIN", "eionet_cdr_endrin"),
    (34, "ETHYLBENZENE", "eionet_cdr_ethylbenzene"),
    (35, "ETHYLENE OXIDE", "eionet_cdr_ethylene_oxide"),
    (36, "FLUORANTHENE", "eionet_cdr_fluoranthene"),
    (37, "FLUORIDES", "eionet_cdr_fluorides"),
    (
        38,
        "FLUORINE AND INORGANIC COMPOUNDS",
        "eionet_cdr_fluorine_and_inorganic_compounds",
    ),
    (
        39,
        "HALOGENATED ORGANIC COMPOUNDS",
        "eionet_cdr_halogenated_organic_compounds",
    ),
    (40, "HALONS", "eionet_cdr_halons"),
    (41, "HCFCS", "eionet_cdr_hcfcs"),
    (42, "HCN", "eionet_cdr_hcn"),
    (43, "HEPTACHLOR", "eionet_cdr_heptachlor"),
    (44, "HEXABROMOBIPHENYL", "eionet_cdr_hexabromobiphenyl"),
    (
        45,
        "HEXACHLOROBENZENE (HCB)",
        "eionet_cdr_hexachlorobenzene_hcb",
    ),
    (
        46,
        "HEXACHLOROBUTADIENE (HCBD)",
        "eionet_cdr_hexachlorobutadiene_hcbd",
    ),
    (
        47,
        "HEXACHLOROCYCLOHEXANE(HCH)",
        "eionet_cdr_hexachlorocyclohexane_hch",
    ),
    (48, "HFCS", "eionet_cdr_hfcs"),
    (49, "HG AND COMPOUNDS", "eionet_cdr_hg_and_compounds"),
    (50, "ISODRIN", "eionet_cdr_isodrin"),
    (51, "ISOPROTURON", "eionet_cdr_isoproturon"),
    (52, "LINDANE", "eionet_cdr_lindane"),
    (53, "MIREX", "eionet_cdr_mirex"),
    (54, "N2O", "eionet_cdr_n2o"),
    (55, "NAPHTHALENE", "eionet_cdr_naphthalene"),
    (56, "NH3", "eionet_cdr_nh3"),
    (57, "NI AND COMPOUNDS", "eionet_cdr_ni_and_compounds"),
    (58, "NMVOC", "eionet_cdr_nmvoc"),
    (59, "NOX", "eionet_cdr_nox"),
    (60, "NP/NPES", "eionet_cdr_np_npes"),
    (
        61,
        "OCTYLPHENOLS AND OCTYLPHENOL ETHOXYLATES",
        "eionet_cdr_octylphenols_and_octylphenol_ethoxylates",
    ),
    (
        62,
        "ORGANOTIN - COMPOUNDS",
        "eionet_cdr_organotin_compounds",
    ),
    (63, "PB AND COMPOUNDS", "eionet_cdr_pb_and_compounds"),
    (
        64,
        "PCDD+PCDF (DIOXINS+FURANS)",
        "eionet_cdr_pcdd_pcdf_dioxins_furans",
    ),
    (65, "PENTACHLOROBENZENE", "eionet_cdr_pentachlorobenzene"),
    (
        66,
        "PENTACHLOROPHENOL (PCP)",
        "eionet_cdr_pentachlorophenol_pcp",
    ),
    (67, "PFCS", "eionet_cdr_pfcs"),
    (68, "PHENOLS", "eionet_cdr_phenols"),
    (69, "PM10", "eionet_cdr_pm10"),
    (
        70,
        "POLYCHLORINATED BIPHENYLS (PCBS)",
        "eionet_cdr_polychlorinated_biphenyls_pcbs",
    ),
    (
        71,
        "POLYCYCLIC AROMATIC HYDROCARBONS",
        "eionet_cdr_polycyclic_aromatic_hydrocarbons",
    ),
    (72, "SF6", "eionet_cdr_sf6"),
    (73, "SIMAZINE", "eionet_cdr_simazine"),
    (74, "SOX", "eionet_cdr_sox"),
    (
        75,
        "TETRACHLOROETHANE-1,1,2,2",
        "eionet_cdr_tetrachloroethane_1_1_2_2",
    ),
    (
        76,
        "TETRACHLOROETHYLENE (PER)",
        "eionet_cdr_tetrachloroethylene_per",
    ),
    (
        77,
        "TETRACHLOROMETHANE (TCM)",
        "eionet_cdr_tetrachloromethane_tcm",
    ),
    (78, "TOLUENE", "eionet_cdr_toluene"),
    (79, "TOTAL - NITROGEN", "eionet_cdr_total_nitrogen"),
    (
        80,
        "TOTAL ORGANIC CARBON (TOC)",
        "eionet_cdr_total_organic_carbon_toc",
    ),
    (81, "TOTAL - PHOSPHORUS", "eionet_cdr_total_phosphorus"),
    (82, "TOXAPHENE", "eionet_cdr_toxaphene"),
    (
        83,
        "TRIBUTYLTIN AND COMPOUNDS",
        "eionet_cdr_tributyltin_and_compounds",
    ),
    (
        84,
        "TRICHLOROBENZENES (TCB)",
        "eionet_cdr_trichlorobenzenes_tcb",
    ),
    (
        85,
        "TRICHLOROETHANE-1,1,1 (TCE)",
        "eionet_cdr_trichloroethane_1_1_1_tce",
    ),
    (
        86,
        "TRICHLOROETHYLENE (TRI)",
        "eionet_cdr_trichloroethylene_tri",
    ),
    (87, "TRICHLOROMETHANE", "eionet_cdr_trichloromethane"),
    (88, "TRIFLURALIN", "eionet_cdr_trifluralin"),
    (
        89,
        "TRIPHENYLTIN AND COMPOUNDS",
        "eionet_cdr_triphenyltin_and_compounds",
    ),
    (90, "VINYL CHLORIDE", "eionet_cdr_vinyl_chloride"),
    (91, "XYLENES", "eionet_cdr_xylenes"),
    (92, "ZN AND COMPOUNDS", "eionet_cdr_zn_and_compounds"),
];

pub const COMP_MAX: u32 = (92 << 2) | 3;

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

pub fn series_name(comp: u32) -> Option<String> {
    let base = component_name(comp)?;
    let medium = medium_of(comp)?;
    Some(format!("{base}_{medium}"))
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

pub fn parse_report(bytes: &[u8]) -> Result<Vec<GeoRec>, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("report is not UTF-8: {e}"))?;
    let text = text.trim_start_matches('\u{feff}');
    if find_open(text, "PollutantReleaseAndTransferReport").is_none() {
        return Err("carries no E-PRTR/LCP report root".to_string());
    }
    let year = tag_body(text, "ReportingYear")
        .and_then(|b| b.trim().parse::<i64>().ok())
        .ok_or_else(|| "ReportingYear absent".to_string())?;
    let days = crate::lsk::days_from_civil(year, 1, 1)
        .ok_or_else(|| format!("ReportingYear {year} is not a civil date"))?;
    let lsk = crate::archivar::embedded_lsk()
        .ok_or_else(|| "embedded leap-second table absent".to_string())?;
    let t = lsk
        .unix_to_tdb(days as f64 * 86400.0)
        .ok_or_else(|| format!("ReportingYear {year} does not map to TDB"))?;
    let mut out = Vec::new();
    for facility in segments(text, "FacilityReport") {
        let Some((lon, lat)) = coord(facility) else {
            continue;
        };
        for release in segments(facility, "PollutantRelease") {
            let raw_medium = tag_body(release, "MediumCode").map(str::trim);
            let medium = raw_medium.and_then(medium_id).ok_or_else(|| {
                format!(
                    "MediumCode '{}' is not in the E-PRTR medium set",
                    raw_medium.unwrap_or("<absent>")
                )
            })?;
            let code = tag_body(release, "PollutantCode")
                .map(str::trim)
                .ok_or_else(|| "PollutantRelease carries no PollutantCode".to_string())?;
            let id = pollutant_id(code).ok_or_else(|| {
                format!("PollutantCode '{code}' is not in the E-PRTR pollutant table")
            })?;
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
    if out.is_empty() {
        return Err(format!(
            "the {year} report carries no E-PRTR release with coordinate and quantity"
        ));
    }
    Ok(out)
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

    fn comp(code: &str, medium: u32) -> u32 {
        (pollutant_id(code).expect("code in table") << 2) | medium
    }

    #[test]
    fn reads_the_measured_shape_and_skips_absent_fields() {
        let records = parse_report(FIXTURE.as_bytes()).expect("the report parses");
        assert_eq!(records.len(), 2);
        let co2 = records
            .iter()
            .find(|r| pollutant_of(r.comp) == Some("CO2"))
            .expect("CO2 present");
        assert_eq!(co2.comp, comp("CO2", COMP_AIR));
        assert_eq!(co2.val, 355000000.0);
        assert_eq!(co2.lat, 47.283943);
        assert_eq!(co2.lon, 12.788444);
        assert_eq!(medium_of(co2.comp), Some("air"));
        let n = records
            .iter()
            .find(|r| pollutant_of(r.comp) == Some("TOTAL - NITROGEN"))
            .expect("TOTAL - NITROGEN present");
        assert_eq!(n.comp, comp("TOTAL - NITROGEN", COMP_WATER));
        assert_eq!(medium_of(n.comp), Some("water"));
    }

    #[test]
    fn a_foreign_body_is_void() {
        assert!(parse_report(b"not xml").is_err());
        assert!(parse_report(b"<ReportData></ReportData>").is_err());
    }

    #[test]
    fn the_table_is_the_authoritative_codelist() {
        assert_eq!(POLLUTANTS.len(), 92);
        for (i, (id, code, slug)) in POLLUTANTS.iter().enumerate() {
            assert_eq!(*id as usize, i + 1, "{code}: id must be the table position");
            assert!(code.chars().all(|c| !c.is_ascii_lowercase()), "{code}");
            assert!(slug.starts_with("eionet_cdr_"), "{code}");
        }
        for code in [
            "CR AND COMPOUNDS",
            "CO2 EXCL BIOMASS",
            "SF6",
            "POLYCHLORINATED BIPHENYLS (PCBS)",
            "VINYL CHLORIDE",
        ] {
            assert!(pollutant_id(code).is_some(), "{code} must be in the table");
        }
    }

    #[test]
    fn an_unmapped_pollutant_code_is_named() {
        let report = FIXTURE.replace(
            "<PollutantCode>CO2</PollutantCode>",
            "<PollutantCode>KRYPTONITE</PollutantCode>",
        );
        let msg = parse_report(report.as_bytes()).expect_err("unmapped code must abort");
        assert!(msg.contains("KRYPTONITE"), "{msg}");
    }

    #[test]
    fn component_names_come_from_the_pollutant_table() {
        assert_eq!(
            component_name(comp("CO2", COMP_AIR)),
            Some("eionet_cdr_co2")
        );
        assert_eq!(
            component_name(comp("NOX", COMP_AIR)),
            Some("eionet_cdr_nox")
        );
        assert_eq!(component_name(999), None);
        assert_eq!(pollutant_id("PCDD+PCDF (DIOXINS+FURANS)"), Some(64));
        assert_eq!(pollutant_id("CR AND COMPOUNDS"), Some(23));
        assert_eq!(medium_id("AIR"), Some(COMP_AIR));
        assert_eq!(medium_id("MAGMA"), None);
    }

    #[test]
    fn component_key_space_covers_every_pollutant_and_medium() {
        let max_id = POLLUTANTS
            .iter()
            .map(|(id, _, _)| *id)
            .max()
            .expect("the pollutant table is not empty");
        assert_eq!(max_id, 92);
        for medium in [COMP_AIR, COMP_WATER, COMP_SOIL] {
            assert!((max_id << 2) | medium <= COMP_MAX);
        }
    }

    #[test]
    fn series_names_carry_the_medium() {
        assert_eq!(
            series_name(comp("CO2", COMP_AIR)).as_deref(),
            Some("eionet_cdr_co2_air")
        );
        assert_eq!(
            series_name(comp("TOTAL - NITROGEN", COMP_WATER)).as_deref(),
            Some("eionet_cdr_total_nitrogen_water")
        );
        assert_eq!(
            series_name(comp("NOX", COMP_SOIL)).as_deref(),
            Some("eionet_cdr_nox_soil")
        );
        assert_eq!(series_name(comp("CO2", COMP_AIR) & !0x3), None);
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
