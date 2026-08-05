use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

const SOURCES_PATH: &str = "phi/sources.φ";
const NETLOC_MAP: &str = "/tmp/live_netlocs.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let do_normalize = args.contains(&"--normalize".to_string());

    let content = fs::read_to_string(SOURCES_PATH).expect("read failed");

    let mut blocks: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("source ") {
            if !current.is_empty() {
                blocks.push(std::mem::take(&mut current));
            }
        }
        current.push(line.to_string());
    }
    if !current.is_empty() {
        blocks.push(current);
    }
    for block in &mut blocks {
        block.retain(|l| !l.trim().is_empty());
    }

    let delete_set: HashSet<&str> = DELETE_NAMES.iter().copied().collect();

    let mut deleted = 0usize;
    let mut converted = 0usize;
    let mut stripped = 0usize;

    let mut source_blocks: Vec<(String, u64, Vec<String>)> = Vec::new();

    for mut block in blocks {
        if block.is_empty() {
            continue;
        }

        let head = block[0].clone();
        if !head.starts_with("source ") {
            eprintln!("skipping block without source header: {:?}", block.first());
            continue;
        }

        let name = head.split_whitespace().nth(1).unwrap_or("").to_string();

        if delete_set.contains(name.as_str()) {
            deleted += 1;
            continue;
        }

        for line in &mut block {
            let t = line.trim();
            if t == "86400" {
                *line = "ttl 86400".to_string();
            } else if t == "ssb" {
                *line = "at sun 1.0".to_string();
                converted += 1;
            }
        }
        block.retain(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with("~=3res") && !t.starts_with("~=res") && t != "86400"
        });

        if name == "geosphere_usgs_earthquakes_24h" {
            let before = block.len();
            block.retain(|l| !l.contains("seismic_count_24h"));
            if block.len() < before {
                stripped += 1;
            }
        }

        if name == "geosphere_noaa_swpc_solar_region_magnetic_classes" {
            let has_lat = block.iter().any(|l| l.starts_with("lat_key "));
            let has_lon = block.iter().any(|l| l.starts_with("lon_key "));
            if !has_lat {
                block.push("lat_key latitude".to_string());
            }
            if !has_lon {
                block.push("lon_key longitude".to_string());
            }
        }

        let ttl = extract_ttl(&block).unwrap_or(999999);
        source_blocks.push((name, ttl, block));
    }

    if do_normalize {
        let netlocs = load_netloc_map();
        let mut fixed = 0usize;
        for (_name, _ttl, block) in &mut source_blocks {
            let url_line = block
                .iter()
                .position(|l| l.starts_with("url ") && l.contains("releases/download"));
            if let Some(pos) = url_line {
                let old = block[pos].clone();
                if let Some(new_url) = fix_cdn_tag(&old, &netlocs) {
                    if new_url != old {
                        block[pos] = new_url;
                        fixed += 1;
                    }
                }
            }
        }
        eprintln!("tag-normalize: {} CDN URLs fixed", fixed);
    }

    source_blocks.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

    let source_count_before_new = source_blocks.len();

    let mut new_block = build_source_block(
        "lmsal_solar_flares",
        86400,
        "force em",
        &[
            "at sun 1.0",
            "url https://github.com/omegaflow/sources/releases/download/lmsal.com/astro_solar_hek_flares.json",
            "map result",
            "lat_key hgs_y",
            "lon_key hgs_x",
            "field event_starttime solar_flare_start",
            "field event_peaktime solar_flare_peak",
            "field fl_goescls solar_flare_goes_class",
            "field ar_noaanum solar_flare_active_region",
        ],
    );
    source_blocks.push(("lmsal_solar_flares".to_string(), 86400, new_block));

    let mut new_block2 = build_source_block(
        "satnogs_stations",
        86400,
        "force em",
        &[
            "url https://github.com/omegaflow/sources/releases/download/network.satnogs.org/technosphere_satnogs_stations.json",
            "map .",
            "lat_key lat",
            "lon_key lng",
            "field altitude satnogs_station_alt_m",
            "field name satnogs_station_name",
            "field observations satnogs_station_observations",
            "field status satnogs_station_status",
            "field success_rate satnogs_station_success_rate_pct",
        ],
    );
    source_blocks.push(("satnogs_stations".to_string(), 86400, new_block2));

    source_blocks.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

    let mut output = String::new();
    for (_, _, block) in &source_blocks {
        for line in block {
            output.push_str(line);
            output.push('\n');
        }
        output.push('\n');
    }

    fs::write(SOURCES_PATH, output).expect("write failed");

    eprintln!(
        "done: {} deleted, {} ssb-converted, {} stripped, {} sources + {} new = {} total",
        deleted,
        converted,
        stripped,
        source_count_before_new,
        source_blocks.len() - source_count_before_new,
        source_blocks.len(),
    );
}

fn extract_ttl(block: &[String]) -> Option<u64> {
    for line in block {
        let trimmed = line.trim();
        if trimmed.starts_with("ttl ") {
            return trimmed
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse::<u64>().ok());
        }
    }
    None
}

fn build_source_block(name: &str, ttl: u64, force: &str, fields: &[&str]) -> Vec<String> {
    let mut block: Vec<String> = Vec::new();
    block.push(format!("source {}", name));
    block.push(format!("ttl {}", ttl));
    block.push(force.to_string());
    for f in fields {
        block.push(f.to_string());
    }
    block
}

fn load_netloc_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Ok(data) = fs::read_to_string(NETLOC_MAP) {
        for line in data.lines() {
            let trimmed = line.trim();
            if let Some(space) = trimmed.find(' ') {
                let name = &trimmed[..space];
                let netloc = &trimmed[space + 1..];
                map.insert(name.to_string(), netloc.to_string());
            }
        }
    }
    map
}

fn fix_cdn_tag(url_line: &str, netlocs: &HashMap<String, String>) -> Option<String> {
    let path = &url_line["url ".len()..];
    let prefix = "https://github.com/omegaflow/sources/releases/download/";
    let rest = path.strip_prefix(prefix)?;
    let mut parts = rest.splitn(2, '/');
    let old_tag = parts.next()?;
    let filename = parts.next()?;
    let source_name = filename
        .strip_suffix(".json")
        .or_else(|| filename.strip_suffix(".bin"))?;
    let new_tag = netlocs.get(source_name)?;
    if old_tag != new_tag {
        return Some(format!(
            "url https://github.com/omegaflow/sources/releases/download/{}/{}",
            new_tag, filename
        ));
    }
    None
}

static DELETE_NAMES: &[&str] = &[
    "biosphere_etf_africa_broad",
    "biosphere_etf_brazil",
    "biosphere_etf_china",
    "biosphere_etf_dollar_index",
    "biosphere_etf_dry_bulk_shipping",
    "biosphere_etf_emerging_markets",
    "biosphere_etf_europe",
    "biosphere_etf_frontier_markets",
    "biosphere_etf_germany",
    "biosphere_etf_gold",
    "biosphere_etf_india",
    "biosphere_etf_japan",
    "biosphere_etf_latin_america_broad",
    "biosphere_etf_mexico",
    "biosphere_etf_middle_east",
    "biosphere_etf_oil",
    "biosphere_etf_sector_communication",
    "biosphere_etf_sector_consumer_discretionary",
    "biosphere_etf_sector_consumer_staples",
    "biosphere_etf_sector_energy",
    "biosphere_etf_sector_financials",
    "biosphere_etf_sector_healthcare",
    "biosphere_etf_sector_industrials",
    "biosphere_etf_sector_materials",
    "biosphere_etf_sector_real_estate",
    "biosphere_etf_sector_technology",
    "biosphere_etf_sector_utilities",
    "biosphere_etf_semiconductors",
    "biosphere_etf_south_africa",
    "biosphere_etf_southeast_asia",
    "biosphere_etf_transportation",
    "biosphere_etf_treasury_yield_10y",
    "biosphere_etf_usa",
    "biosphere_etf_volatility",
    "biosphere_etf_world_all_country",
    "biosphere_etf_world_developed",
    "biosphere_yahoo_finance_markets",
    "biosphere_github_public_events",
    "biosphere_inex_traffic",
    "biosphere_irail_belgium",
    "biosphere_ripe_bgp_default_route",
    "biosphere_ripe_bgp_updates",
    "biosphere_transport_bern_departures",
    "biosphere_transport_zurich_departures",
    "biosphere_wiener_linien_realtime",
    "biosphere_energy_charts_renewable",
    "biosphere_inaturalist_observations_observations",
    "biosphere_uk_carbon_intensity",
    "biosphere_inaturalist_observations_observations_count",
    "biosphere_cdc_covid_global",
    "biosphere_exchangerate_api",
    "biosphere_exchangerate_global",
    "biosphere_gbif_occurrences_occurrence_count",
    "biosphere_gbif_occurrences_species_observations_count",
    "biosphere_gdelt_news_volume",
    "biosphere_launch_library",
    "biosphere_tor_relays_running",
    "biosphere_wikipedia_pageviews_total",
    "biosphere_cdc_covid_variants",
    "technosphere_arxiv_astro_total",
    "technosphere_arxiv_cs_total",
    "technosphere_arxiv_hep_total",
    "technosphere_arxiv_math_total",
    "technosphere_arxiv_physics_total",
    "technosphere_arxiv_qbio_total",
    "technosphere_arxiv_qfin_total",
    "technosphere_arxiv_stat_total",
    "biosphere_arxiv_total_papers",
    "biosphere_bci_clinical_trials_recruiting",
    "biosphere_ecdc_monkeypox",
    "biosphere_global_ixp_count",
    "biosphere_submarine_cables",
    "biosphere_unhcr_displacement",
    "biosphere_who_cholera",
    "biosphere_who_gho_tuberculosis",
    "biosphere_who_influenza",
    "technosphere_wikipedia_pages_total",
    "subatomic_physics_cern_alice_pbpb",
    "subatomic_physics_cern_cms_data",
    "subatomic_physics_cern_open_data",
    "subatomic_physics_cern_opendata_records",
    "astro_solar_sunspots",
    "astro_solar_xray_flares",
    "astro_gfz_sunspot_number",
    "geosphere_gdacs_disaster_alerts",
    "geosphere_tsunami_alerts_ntwc",
    "geosphere_tsunami_alerts_ptwc",
    "subatomic_physics_gcn_ligo_circulars",
    "subatomic_physics_ligo_gracedb_events",
    "technosphere_bfs_odl_count",
    "astro_orbital_celestrak_debris",
    "astro_orbital_celestrak_gps",
    "astro_orbital_celestrak_satellites",
    "astro_orbital_celestrak_starlink",
    "astro_orbital_cneos_asteroids_inside_lunar_orbit",
    "astro_orbital_cneos_close_approaches",
    "astro_orbital_dsn_status",
    "astro_orbital_fireballs",
    "astro_neo_close_approaches",
    "astro_asteroids_numbered",
    "astro_tle_catalog",
    "astro_tle_search_iss",
    "exosphere_satnogs_radio_observations",
    "geosphere_copernicus_sentinel2_count",
    "geosphere_effis_fires",
    "geosphere_gdacs_disasters",
    "geosphere_gdacs_disasters_total",
    "geosphere_reliefweb_active_humanitarian_disasters",
    "magnetosphere_aurora_forecast",
    "magnetosphere_aurora_nowcast_north",
    "hydrosphere_dartmouth_flood_observatory_global",
    "astro_eog_viirs_monthly_ntl_metadata",
    "exosphere_swpc_space_weather_alerts",
    "exosphere_donki_legacy_cme",
    "exosphere_donki_legacy_flares",
    "exosphere_donki_legacy_gst",
    "exosphere_donki_legacy_hss",
    "exosphere_donki_legacy_ips",
    "exosphere_donki_legacy_mpc",
    "exosphere_donki_legacy_sep",
    "hydrosphere_emodnet_vessel_density",
    "geosphere_wikipedia_geopages",
    "geosphere_eoapi_collections",
    "geosphere_macrostrat_ages",
    "geosphere_planetary_computer_collections",
    "astro_gcn_circular_latest",
    "astro_solar_noaa_swpc_flare_count",
    "astro_solar_noaa_swpc_flares",
    "biosphere_inpe_fire_foci",
    "astro_orbit_celestrak_active",
    "astro_orbit_celestrak_active_satellites",
    "astro_orbit_celestrak_all_satellites",
    "astro_orbit_celestrak_cubesats",
    "astro_orbit_celestrak_debris",
    "astro_orbit_celestrak_debris_analyst",
    "astro_orbit_celestrak_engineering_sats",
    "astro_orbit_celestrak_glonass",
    "astro_orbit_celestrak_gps",
    "astro_orbit_celestrak_navigation_sats",
    "astro_orbit_celestrak_oneweb",
    "astro_orbit_celestrak_science_sats",
    "astro_orbit_celestrak_space_stations",
    "astro_orbit_celestrak_starlink",
    "astro_orbit_celestrak_weather_sats",
    "magnetosphere_cdaweb_mms1_fgm_magnetic_field",
];
