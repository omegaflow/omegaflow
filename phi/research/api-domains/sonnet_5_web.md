name	kind	lat	lon	res	confidence	note	
sensor_community_air	DYNAMIC			3	verified	URL already has {lat}/{lon} - just needs res added, e.g. res=3 (0.001 deg ~100m grid)	
apophis_vectors	DYNAMIC			4	verified	JPL Horizons ephemeris body; sub-point lat/lon computed per query from state vector	
atlas_3i_vectors	DYNAMIC			4	verified	Interstellar object; ephemeris-driven	
bennu_vectors	DYNAMIC			4	verified	Horizons asteroid body	
callisto_vectors	DYNAMIC			4	verified	Horizons moon body	
ceres_vectors	DYNAMIC			4	verified	Horizons dwarf planet body	
enceladus_vectors	DYNAMIC			4	verified	Horizons moon body	
encke_vectors	DYNAMIC			4	verified	Horizons comet body	
eris_vectors	DYNAMIC			4	verified	Horizons dwarf planet body	
europa_vectors	DYNAMIC			4	verified	Horizons moon body	
ganymede_vectors	DYNAMIC			4	verified	Horizons moon body	
halley_vectors	DYNAMIC			4	verified	Horizons comet body	
haumea_vectors	DYNAMIC			4	verified	Horizons dwarf planet body	
io_vectors	DYNAMIC			4	verified	Horizons moon body	
juno_vectors	DYNAMIC			4	verified	Horizons spacecraft	
jupiter_vectors	DYNAMIC			4	verified	Horizons planet body	
jwst_vectors	DYNAMIC			4	verified	Horizons spacecraft (L2 orbit - no sub-Earth-point in usual sense; consider storing heliocentric coords instead of lat/lon)	
makemake_vectors	DYNAMIC			4	verified	Horizons dwarf planet body	
mars_vectors	DYNAMIC			4	verified	Horizons planet body	
mercury_vectors	DYNAMIC			4	verified	Horizons planet body	
moon_vectors	DYNAMIC			4	verified	Horizons Earth's moon	
neptune_vectors	DYNAMIC			4	verified	Horizons planet body	
new_horizons_vectors	DYNAMIC			4	verified	Horizons spacecraft, deep solar system	
parker_solar_probe_vectors	DYNAMIC			4	verified	Horizons spacecraft	
pluto_vectors	DYNAMIC			4	verified	Horizons dwarf planet body	
saturn_vectors	DYNAMIC			4	verified	Horizons planet body	
solar_orbiter_vectors	DYNAMIC			4	verified	Horizons spacecraft	
sun_vectors	DYNAMIC			4	verified	Horizons - the Sun itself; heliographic lat/lon convention applies	
titan_vectors	DYNAMIC			4	verified	Horizons moon body	
triton_vectors	DYNAMIC			4	verified	Horizons moon body	
uranus_vectors	DYNAMIC			4	verified	Horizons planet body	
venus_vectors	DYNAMIC			4	verified	Horizons planet body	
vesta_vectors	DYNAMIC			4	verified	Horizons asteroid body	
voyager1_vectors	DYNAMIC			4	verified	Horizons spacecraft, interstellar space	
voyager2_vectors	DYNAMIC			4	verified	Horizons spacecraft, interstellar space	
celestrak_gps	DYNAMIC			3	verified	TLE catalog, many satellites - each has its own sub-point	
celestrak_satellites	DYNAMIC			3	verified	TLE catalog - active satellites, many objects	
celestrak_debris	DYNAMIC			3	verified	TLE catalog - debris field, many objects	
celestrak_starlink	DYNAMIC			3	verified	TLE catalog - Starlink constellation, many objects	
iss_position	DYNAMIC			4	verified	Already the reference pattern - confirmed correct as-is	
opensky_states	DYNAMIC			3	verified	Live aircraft states - inherently many simultaneous positions, not one point	
satnogs_radio_observations	DYNAMIC			3	estimated	Ground-station network tracking passing satellites; satellite sub-point is dynamic. Alternative: treat as STATIC at SatNOGS coordinating org if you want the network hub instead	
spacex_latest_launch	DYNAMIC			4	estimated	Launch position changes per mission; alternatively STATIC at most recent/primary launch site (Starbase 25.9961,-97.1554 or Canaveral 28.5619,-80.5772) if you want a fixed proxy instead	
seismic	STATIC	39.7477	-105.2088	4	verified	USGS NEIC / Geologic Hazards Science Center, Golden CO (confirmed via search, on Colorado School of Mines campus)	
usgs_earthquakes_24h	STATIC	39.7477	-105.2088	4	verified	Same NEIC Golden CO anchor as seismic	
usgs_deep_earthquakes	STATIC	39.7477	-105.2088	4	verified	Same NEIC Golden CO anchor	
usgs_significant_rms	STATIC	39.7477	-105.2088	4	verified	Same NEIC Golden CO anchor	
usgs_volcano_alerts	STATIC	39.7477	-105.2088	4	estimated	USGS Volcano Hazards Program coordinated via Geologic Hazards Science Center, Golden CO - same campus as NEIC	
usgs_streamflow	STATIC	38.9495	-77.3502	4	estimated	USGS National Water Information System / HQ Reston VA (different program than NEIC)	
swpc_solar_events	STATIC	39.9916	-105.2612	4	verified	NOAA Space Weather Prediction Center, Boulder CO (confirmed via search)	
aurora_forecast	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO	
aurora_nowcast_north	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO	
ionosphere_tec_global	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO	
noaa_integral_protons	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO	
noaa_solar_indices	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO	
noaa_solar_radio_measured	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO	
noaa_sunspots	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO	
radio_flux	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO (duplicate dataset of noaa_solar_radio_measured)	
swpc_geomag_forecast	STATIC	39.9916	-105.2612	4	verified	NOAA SWPC, Boulder CO	
magnetism	STATIC	0.0	-75.0	0	verified	Already correctly set - GOES geostationary, no change needed	
noaa_goes_electrons	STATIC	0.0	-75.0	0	verified	Already correctly set - GOES geostationary	
noaa_goes_magnetosphere	STATIC	0.0	-75.0	0	verified	Already correctly set - GOES geostationary	
noaa_proton_flux	STATIC	0.0	-75.0	0	verified	Already correctly set - GOES geostationary	
plasma	STATIC	0.0	-75.0	0	verified	Already correctly set - GOES geostationary	
protons	STATIC	0.0	-75.0	0	verified	Already correctly set - GOES geostationary	
goes_euv	STATIC	0.0	-75.0	0	verified	Already correctly set - GOES geostationary	
nasa_neows	STATIC	34.2007	-118.1712	4	estimated	JPL Center for NEO Studies, Pasadena CA (JPL campus coords)	
nasa_fireballs	STATIC	34.2007	-118.1712	4	estimated	JPL CNEOS, Pasadena CA	
jpl_atmospheric_fireballs	STATIC	34.2007	-118.1712	4	estimated	JPL CNEOS, Pasadena CA	
cneos_close_approaches	STATIC	34.2007	-118.1712	4	estimated	JPL CNEOS, Pasadena CA	
cneos_asteroids_inside_lunar_orbit	STATIC	34.2007	-118.1712	4	estimated	JPL CNEOS, Pasadena CA	
nasa_sentry	STATIC	34.2007	-118.1712	4	estimated	JPL CNEOS Sentry, Pasadena CA	
nasa_donki_cme	STATIC	38.9930	-76.8483	4	estimated	NASA Goddard Space Flight Center, Greenbelt MD - DONKI host	
nasa_donki_flares	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC DONKI	
nasa_donki_gstorms	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC DONKI	
nasa_donki_hss	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC DONKI	
nasa_donki_ips	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC DONKI	
nasa_donki_mpc	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC DONKI	
nasa_donki_rbe	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC DONKI	
nasa_donki_sep	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC DONKI	
nasa_donki_xflares	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC DONKI	
nasa_eonet	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC EONET	
nasa_eonet_fires	STATIC	38.9930	-76.8483	4	estimated	NASA GSFC EONET wildfires	
noaa_methane	STATIC	40.0350	-105.2500	4	estimated	NOAA Global Monitoring Laboratory, Boulder CO	
noaa_n2o	STATIC	40.0350	-105.2500	4	estimated	NOAA GML, Boulder CO	
noaa_sf6	STATIC	40.0350	-105.2500	4	estimated	NOAA GML, Boulder CO	
noaa_pmel_co2_moorings	STATIC	47.6062	-122.3321	4	estimated	NOAA PMEL, Seattle WA (Pacific Marine Environmental Lab, different from GML)	
noaa_enso_oni	STATIC	38.9897	-76.9378	4	estimated	NOAA Climate Prediction Center, College Park MD	
noaa_paleoclimate_co2	STATIC	35.6114	-77.3776	4	estimated	NOAA NCEI paleoclimatology archive, Asheville NC	
global_temp_anomaly	STATIC	40.8075	-73.9626	4	estimated	NASA GISS, Columbia University, New York NY	
hadcrut5_temp	STATIC	50.7276	-3.4720	4	estimated	UK Met Office HQ, Exeter	
nsidc_sea_ice	STATIC	40.0091	-105.2634	4	estimated	National Snow and Ice Data Center, Boulder CO (CU Boulder campus)	
cryosphere_sea_ice_north	STATIC	40.0091	-105.2634	4	estimated	NSIDC, Boulder CO	
cryosphere_sea_ice_south	STATIC	40.0091	-105.2634	4	estimated	NSIDC/CoastWatch ERDDAP, Boulder CO anchor	
cryosphere_glacier_count	STATIC	46.8182	8.2275	2	needs_review	GLIMS is an international consortium (no single HQ) - placeholder uses Switzerland (glacier research center of gravity); consider a different anchor if you have a preferred GLIMS node	
vostok_icecore	STATIC	-78.4645	106.8339	4	verified	Vostok Station, Antarctica - genuine physical drill site coordinates (well-documented station location)	
dome_fuji_co2	STATIC	-77.3190	39.7030	4	estimated	Dome Fuji Station, Antarctica - drill site (Japanese Antarctic station)	
woudc_total_ozone	STATIC	43.6532	-79.3832	4	estimated	World Ozone and UV Data Centre, Environment Canada, Toronto	
usdm_conus_drought	STATIC	40.8202	-96.7005	4	estimated	National Drought Mitigation Center, University of Nebraska-Lincoln	
who_influenza	STATIC	46.2323	6.1420	4	verified	WHO HQ, Avenue Appia 20, Geneva (confirmed via search)	
who_cholera	STATIC	46.2323	6.1420	4	verified	WHO HQ, Geneva	
who_gho_tuberculosis	STATIC	46.2323	6.1420	4	verified	WHO HQ, Geneva	
cern_open_data	STATIC	46.2338	6.0532	4	verified	CERN Meyrin main site (confirmed via search)	
cern_alice_pbpb	STATIC	46.2338	6.0532	4	verified	CERN Meyrin main site (ALICE detector is on LHC ring, this is the HQ anchor)	
cern_cms_data	STATIC	46.2338	6.0532	4	verified	CERN Meyrin main site (CMS detector is on LHC ring, this is the HQ anchor)	
etf_treasury_yield_10y	STATIC	40.7128	-74.0060	4	estimated	US Treasury market / NYSE proxy, New York NY	
rain_radar	STATIC	52.3676	4.9041	4	needs_review	RainViewer BV registered Amsterdam - verify current HQ	
ripe_bgp_default_route	STATIC	52.3792	4.8994	4	verified	RIPE NCC HQ, Stationsplein 11, Amsterdam - east wing of Centraal Station (confirmed via search)	RIPE NCC, Amsterdam
gdelt_news_volume	STATIC	38.9072	-77.0369	4	needs_review	GDELT Project is a distributed academic project, no fixed HQ - Washington DC used as author's institutional proxy, verify preferred anchor	
gdacs_disasters	STATIC	45.4642	9.1900	4	needs_review	GDACS coordinated via EU JRC Ispra - verify, as GDACS itself is a UN/EU joint initiative w/o single HQ	
anthroposphere_global_conflict_events_today	STATIC	38.9072	-77.0369	4	needs_review	ACLED HQ - verify current location, org has grown multi-site	
arxiv_new_papers	STATIC	42.4534	-76.4735	4	estimated	arXiv, Cornell University, Ithaca NY	
astronauts_in_space	STATIC	38.9930	-76.8483	4	estimated	Open Notify aggregator - anchored to NASA GSFC as astronomy/space data proxy	
open_notify_astros	STATIC	38.9930	-76.8483	4	estimated	Duplicate dataset of astronauts_in_space, same anchor	
cdc_covid_global	STATIC	33.7969	-84.3253	4	estimated	US CDC HQ, Atlanta GA	
cdc_covid_variants	STATIC	33.7969	-84.3253	4	estimated	US CDC HQ, Atlanta GA	
ecdc_monkeypox	STATIC	59.3728	18.0172	4	verified	ECDC HQ, Gustav III:s Boulevard 40, Solna (confirmed via search)	ECDC HQ, Solna/Stockholm - verify exact Solna coordinates
unhcr_displacement	STATIC	46.2140	6.1470	4	verified	UNHCR HQ, 94 Rue de Montbrillant, Geneva (confirmed via search)	UNHCR HQ, Geneva - verify exact address coordinates (Rue de Montbrillant)
copernicus_sentinel2_count	STATIC	52.2298	20.9878	4	verified	Copernicus Data Space Ecosystem hosted by CloudFerro, Fabryczna 5, Warsaw - confirmed via search as actual hosting infrastructure (consortium also includes Hasselt BE node)	ESA/ESOC, Darmstadt Germany - verify, Copernicus Data Space may be hosted elsewhere (CloudFerro/Poland)
sentinel_hub_catalog	STATIC	46.0511	14.5051	4	needs_review	Sinergise (Sentinel Hub), Ljubljana Slovenia	
esa_maap_collections	STATIC	48.8566	2.3522	4	needs_review	ESA Paris office used as proxy - MAAP is joint ESA/NASA, verify better anchor (maybe ESRIN Frascati)	
esa_cci_datasets	STATIC	51.2802	-0.5231	4	needs_review	CEDA/STFC Harwell, UK - hosts ESA-CCI archive	
eoapi_collections	STATIC	38.9930	-76.8483	4	needs_review	Generic STAC catalog aggregator - no clear single owner, NASA GSFC used as EO-data proxy	
planetary_computer_collections	STATIC	47.6396	-122.1281	4	estimated	Microsoft Redmond WA - Planetary Computer team	
crossref_dois	STATIC	42.5326	-71.0467	4	verified	Crossref HQ, 50 Salem St, Lynnfield MA 01940 (confirmed via search)	Crossref registered in Lynnfield MA - verify current HQ, may have moved
openalex_works	STATIC	35.4799	-79.1803	4	verified	OurResearch Inc (OpenAlex) legal HQ, Sanford NC - operational team is remote/Vancouver-based (confirmed via search)	OurResearch/OpenAlex - nonprofit registration state varies, verify
wikipedia_pageviews_total	STATIC	37.7936	-122.4014	4	verified	Wikimedia Foundation HQ, 1 Sansome Street Suite 1895, San Francisco - moved here Oct 2024 (confirmed via search)	Wikimedia Foundation HQ, San Francisco CA
technosphere_tor_relays_running	STATIC	42.4522	-71.1375	4	verified	The Tor Project Inc, registered Winchester MA (confirmed via search - legal HQ, not a physical operations center in usual sense)	The Tor Project HQ, Cambridge/Winchester MA - verify current address
technosphere_global_ixp_count	STATIC	42.3736	-71.1097	4	needs_review	PeeringDB nonprofit - verify registered address, this is a rough proxy	
technosphere_submarine_cables	STATIC	38.9072	-77.0369	4	needs_review	TeleGeography, Washington DC - verify	
exchangerate_api	STATIC	39.7392	-104.9903	4	needs_review	ExchangeRate-API operator - verify, could not confirm exact HQ	
exchangerate_global	STATIC	52.5200	13.4050	4	needs_review	open.er-api.com - verify hosting/operator location, low confidence	
worldbank_co2_emissions	STATIC	38.8991	-77.0430	4	estimated	World Bank HQ, Washington DC	
worldbank_population	STATIC	38.8991	-77.0430	4	estimated	World Bank HQ, Washington DC	
anthroposphere_global_population_density_sqkm	STATIC	38.8991	-77.0430	4	estimated	World Bank HQ, Washington DC	
globalforestwatch_tree_cover_loss	STATIC	38.9072	-77.0369	4	estimated	World Resources Institute, Washington DC	
effis_fires	STATIC	45.8098	8.6312	4	verified	EU Joint Research Centre, Via Enrico Fermi 2749, Ispra Italy (confirmed via search)	EU Joint Research Centre, Ispra Italy - verify exact JRC Ispra coordinates
emodnet_vessel_density	STATIC	51.2317	2.9319	4	verified	VLIZ InnovOcean Campus, Ostend Belgium - EMODnet Secretariat host (confirmed via search)	EMODnet Secretariat, Ostend Belgium (VLIZ) - verify
launch_library	STATIC	51.5072	-0.1276	4	needs_review	The Space Devs - distributed remote team, London used as rough proxy only, low confidence	
nifc_fires	STATIC	43.6187	-116.2146	4	estimated	National Interagency Fire Center, Boise ID	
microbe_census	STATIC	52.0806	-0.1900	4	estimated	EMBL-EBI, Hinxton, Cambridgeshire UK	
moon_phase	STATIC	38.9930	-76.8483	4	needs_review	Farmsense API - no public HQ found, NASA GSFC used as astronomy-data proxy, low confidence	
obis_cetaceans	STATIC	51.2317	2.9319	4	verified	OBIS Secretariat hosted at VLIZ, Ostend Belgium (confirmed via search)	OBIS Secretariat hosted at VLIZ, Ostend Belgium
obis_statistics	STATIC	51.2317	2.9319	4	verified	OBIS Secretariat, VLIZ Ostend (confirmed via search)	OBIS Secretariat, Ostend Belgium
gbif_species_observations_count	STATIC	55.7022	12.5592	4	verified	GBIF Secretariat, Universitetsparken 15, Copenhagen (confirmed via search)	GBIF Secretariat, Copenhagen Denmark
ebird_recent	STATIC	42.4809	-76.4527	4	estimated	Cornell Lab of Ornithology, Ithaca NY (eBird's true home institution)	
ebird_hotspots	STATIC	42.4809	-76.4527	4	estimated	Cornell Lab of Ornithology, Ithaca NY	
gbif_migrations	STATIC	55.7022	12.5592	4	verified	GBIF Secretariat, Copenhagen (confirmed via search)	GBIF Secretariat, Copenhagen
inaturalist_observations_count	STATIC	37.7749	-122.4661	4	estimated	California Academy of Sciences, San Francisco (iNaturalist co-founder org)	
neotoma_paleoecology	STATIC	43.0731	-89.4012	3	needs_review	Neotoma Paleoecology DB consortium - University of Wisconsin-Madison node used as proxy	
pbdb_paleobiology	STATIC	43.0731	-89.4012	3	needs_review	Paleobiology Database consortium - UW-Madison node used as proxy, verify preferred anchor	
macrostrat_ages	STATIC	43.0731	-89.4012	4	estimated	Macrostrat, University of Wisconsin-Madison	
macrostrat_timescale	STATIC	43.0731	-89.4012	4	estimated	Macrostrat, UW-Madison	
crystallography_xrd	STATIC	47.3220	5.0415	3	needs_review	Crystallography Open Database - hosted at multiple mirrors (France/Poland/Lithuania), low confidence single-point anchor	
protein_structures	STATIC	40.5008	-74.4474	4	verified	RCSB PDB HQ, Rutgers University, Piscataway NJ 08854 (confirmed via search - additional sites at UC San Diego and UC San Francisco also exist)	
superk_proton_decay	STATIC	36.4286	137.3103	4	estimated	Super-Kamiokande detector, Kamioka Observatory, Japan - actual underground instrument site	
pdg_alpha_s	STATIC	37.8756	-122.2508	4	estimated	Particle Data Group, Lawrence Berkeley National Laboratory	
pdg_higgs_mass	STATIC	37.8756	-122.2508	4	estimated	Particle Data Group, LBNL	
pdg_proton_mass	STATIC	37.8756	-122.2508	4	estimated	Particle Data Group, LBNL	
pdg_w_boson_mass	STATIC	37.8756	-122.2508	4	estimated	Particle Data Group, LBNL	
pdg_z_boson_mass	STATIC	37.8756	-122.2508	4	estimated	Particle Data Group, LBNL	
esa_gaia_stars	STATIC	40.4438	-3.9529	4	verified	ESAC, Villanueva de la Canada, Madrid (confirmed via search - exact ESA-published coordinates)	ESAC (European Space Astronomy Centre), Villanueva de la Canada, Madrid - Gaia Archive host, verify exact coords
gaia_nearby_stars	STATIC	40.4438	-3.9529	4	verified	ESAC Madrid (confirmed via search)	ESAC Madrid - same Gaia archive anchor
gaia_stellar_ages	STATIC	40.4438	-3.9529	4	verified	ESAC Madrid (confirmed via search)	ESAC Madrid
gaia_total_measured_stars	STATIC	40.4438	-3.9529	4	verified	ESAC Madrid (confirmed via search)	ESAC Madrid
gaia_variable_stars	STATIC	40.4438	-3.9529	4	verified	ESAC Madrid (confirmed via search)	ESAC Madrid
simbad_brown_dwarfs	STATIC	48.5833	7.7667	4	estimated	CDS / Strasbourg Astronomical Observatory - SIMBAD host institution	
simbad_carbon_stars	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_eclipsing_binaries	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_highest_redshift_quasar	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_novae	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_supernovae	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_symbiotic_stars	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_white_dwarfs	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_wolf_rayet	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_young_stellar_objects	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_galaxies	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_galaxy_clusters	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_high_z_galaxies	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_millisecond_pulsars	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_pulsars	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_quasars	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
simbad_total_objects	STATIC	48.5833	7.7667	4	estimated	CDS Strasbourg	
nasa_exoplanets	STATIC	34.2007	-118.1712	4	estimated	NASA Exoplanet Archive, IPAC/Caltech, Pasadena	
nasa_exoplanet_total	STATIC	34.2007	-118.1712	4	estimated	NASA Exoplanet Archive, IPAC/Caltech	
nasa_hot_jupiters	STATIC	34.2007	-118.1712	4	estimated	NASA Exoplanet Archive, IPAC/Caltech	
sdss_cosmic_web_galaxies	STATIC	32.7803	-105.8203	4	estimated	Apache Point Observatory, New Mexico - actual SDSS telescope site (real instrument, not just an office)	
solar_system_earth_data	STATIC	48.8566	2.3522	4	needs_review	Le Systeme Solaire API - independently maintained, no formal HQ; Paris used as low-confidence dev-location proxy	
biosphere_global_vegetation_health_index	STATIC	38.9784	-76.9199	4	needs_review	NOAA/NESDIS STAR, College Park MD - verify	
biosphere_uv_index_global	STATIC	37.0871	-76.3872	4	needs_review	NASA Langley Research Center, Hampton VA (POWER project host) - verify	
worldbank_gdp_growth_ABW	STATIC	12.5246	-70027	4	verified	World Bank GDP growth API for Aruba - anchored at capital city Oranjestad (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_AFG	STATIC	34.5553	69.2075	4	verified	World Bank GDP growth API for Afghanistan - anchored at capital city Kabul (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_AGO	STATIC	-8839	13.2894	4	verified	World Bank GDP growth API for Angola - anchored at capital city Luanda (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ALB	STATIC	41.3275	19.8187	4	verified	World Bank GDP growth API for Albania - anchored at capital city Tirana (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_AND	STATIC	42.5063	1.5218	4	verified	World Bank GDP growth API for Andorra - anchored at capital city Andorra la Vella (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ARE	STATIC	24.4539	54.3773	4	verified	World Bank GDP growth API for United Arab Emirates - anchored at capital city Abu Dhabi (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ARG	STATIC	-34.6037	-58.3816	4	verified	World Bank GDP growth API for Argentina - anchored at capital city Buenos Aires (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ARM	STATIC	40.1792	44.4991	4	verified	World Bank GDP growth API for Armenia - anchored at capital city Yerevan (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ASM	STATIC	-14.2756	-170702	4	verified	World Bank GDP growth API for American Samoa - anchored at capital city Pago Pago (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ATG	STATIC	17.1274	-61.8468	4	verified	World Bank GDP growth API for Antigua and Barbuda - anchored at capital city Saint John's (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_AUS	STATIC	-35.2809	149.13	4	verified	World Bank GDP growth API for Australia - anchored at capital city Canberra (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_AUT	STATIC	48.2082	16.3738	4	verified	World Bank GDP growth API for Austria - anchored at capital city Vienna (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_AZE	STATIC	40.4093	49.8671	4	verified	World Bank GDP growth API for Azerbaijan - anchored at capital city Baku (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BDI	STATIC	-3.4264	29.9306	4	verified	World Bank GDP growth API for Burundi - anchored at capital city Gitega (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BEL	STATIC	50.8503	4.3517	4	verified	World Bank GDP growth API for Belgium - anchored at capital city Brussels (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BEN	STATIC	6.4969	2.6289	4	verified	World Bank GDP growth API for Benin - anchored at capital city Porto-Novo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BFA	STATIC	12.3714	-1.5197	4	verified	World Bank GDP growth API for Burkina Faso - anchored at capital city Ouagadougou (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BGD	STATIC	23.8103	90.4125	4	verified	World Bank GDP growth API for Bangladesh - anchored at capital city Dhaka (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BGR	STATIC	42.6977	23.3219	4	verified	World Bank GDP growth API for Bulgaria - anchored at capital city Sofia (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BHR	STATIC	26.2285	50586	4	verified	World Bank GDP growth API for Bahrain - anchored at capital city Manama (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BHS	STATIC	25.0343	-77.3963	4	verified	World Bank GDP growth API for Bahamas - anchored at capital city Nassau (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BIH	STATIC	43.8563	18.4131	4	verified	World Bank GDP growth API for Bosnia and Herzegovina - anchored at capital city Sarajevo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BLR	STATIC	53.9006	27559	4	verified	World Bank GDP growth API for Belarus - anchored at capital city Minsk (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BLZ	STATIC	17251	-88759	4	verified	World Bank GDP growth API for Belize - anchored at capital city Belmopan (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BMU	STATIC	32.2949	-64.7834	4	verified	World Bank GDP growth API for Bermuda - anchored at capital city Hamilton (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BOL	STATIC	-16.4897	-68.1193	4	verified	World Bank GDP growth API for Bolivia - anchored at capital city La Paz/Sucre (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BRA	STATIC	-15.7801	-47.9292	4	verified	World Bank GDP growth API for Brazil - anchored at capital city Brasilia (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BRB	STATIC	13.1132	-59.5988	4	verified	World Bank GDP growth API for Barbados - anchored at capital city Bridgetown (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BRN	STATIC	4.9031	114.9398	4	verified	World Bank GDP growth API for Brunei - anchored at capital city Bandar Seri Begawan (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BTN	STATIC	27.4712	89.6339	4	verified	World Bank GDP growth API for Bhutan - anchored at capital city Thimphu (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_BWA	STATIC	-24.6282	25.9231	4	verified	World Bank GDP growth API for Botswana - anchored at capital city Gaborone (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CAF	STATIC	4.3947	18.5582	4	verified	World Bank GDP growth API for Central African Republic - anchored at capital city Bangui (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CAN	STATIC	45.4215	-75.6972	4	verified	World Bank GDP growth API for Canada - anchored at capital city Ottawa (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CHE	STATIC	46948	7.4474	4	verified	World Bank GDP growth API for Switzerland - anchored at capital city Bern (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CHI	STATIC	49.1805	-2.1058	4	verified	World Bank GDP growth API for Channel Islands - anchored at capital city Saint Helier (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CHL	STATIC	-33.4489	-70.6693	4	verified	World Bank GDP growth API for Chile - anchored at capital city Santiago (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CHN	STATIC	39.9042	116.4074	4	verified	World Bank GDP growth API for China - anchored at capital city Beijing (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CIV	STATIC	6.8276	-5.2893	4	verified	World Bank GDP growth API for Cote d'Ivoire - anchored at capital city Yamoussoukro (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CMR	STATIC	3848	11.5021	4	verified	World Bank GDP growth API for Cameroon - anchored at capital city Yaounde (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_COD	STATIC	-4.4419	15.2663	4	verified	World Bank GDP growth API for Congo, Dem. Rep. - anchored at capital city Kinshasa (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_COG	STATIC	-4.2634	15.2429	4	verified	World Bank GDP growth API for Congo, Rep. - anchored at capital city Brazzaville (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_COL	STATIC	4711	-74.0721	4	verified	World Bank GDP growth API for Colombia - anchored at capital city Bogota (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_COM	STATIC	-11.7172	43.2473	4	verified	World Bank GDP growth API for Comoros - anchored at capital city Moroni (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CPV	STATIC	14933	-23.5133	4	verified	World Bank GDP growth API for Cabo Verde - anchored at capital city Praia (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CRI	STATIC	9.9281	-84.0907	4	verified	World Bank GDP growth API for Costa Rica - anchored at capital city San Jose (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CUB	STATIC	23.1136	-82.3666	4	verified	World Bank GDP growth API for Cuba - anchored at capital city Havana (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CUW	STATIC	12.1091	-68.9316	4	verified	World Bank GDP growth API for Curacao - anchored at capital city Willemstad (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CYM	STATIC	19.2866	-81.3674	4	verified	World Bank GDP growth API for Cayman Islands - anchored at capital city George Town (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CYP	STATIC	35.1856	33.3823	4	verified	World Bank GDP growth API for Cyprus - anchored at capital city Nicosia (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_CZE	STATIC	50.0755	14.4378	4	verified	World Bank GDP growth API for Czechia - anchored at capital city Prague (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_DEU	STATIC	52.52	13405	4	verified	World Bank GDP growth API for Germany - anchored at capital city Berlin (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_DJI	STATIC	11.5721	43.1456	4	verified	World Bank GDP growth API for Djibouti - anchored at capital city Djibouti (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_DMA	STATIC	15.3092	-61.3794	4	verified	World Bank GDP growth API for Dominica - anchored at capital city Roseau (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_DNK	STATIC	55.6761	12.5683	4	verified	World Bank GDP growth API for Denmark - anchored at capital city Copenhagen (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_DOM	STATIC	18.4861	-69.9312	4	verified	World Bank GDP growth API for Dominican Republic - anchored at capital city Santo Domingo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_DZA	STATIC	36.7538	3.0588	4	verified	World Bank GDP growth API for Algeria - anchored at capital city Algiers (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ECU	STATIC	-0.1807	-78.4678	4	verified	World Bank GDP growth API for Ecuador - anchored at capital city Quito (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_EGY	STATIC	30.0444	31.2357	4	verified	World Bank GDP growth API for Egypt - anchored at capital city Cairo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ERI	STATIC	15.3229	38.9251	4	verified	World Bank GDP growth API for Eritrea - anchored at capital city Asmara (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ESP	STATIC	40.4168	-3.7038	4	verified	World Bank GDP growth API for Spain - anchored at capital city Madrid (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_EST	STATIC	59437	24.7536	4	verified	World Bank GDP growth API for Estonia - anchored at capital city Tallinn (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ETH	STATIC	9.03	38.74	4	verified	World Bank GDP growth API for Ethiopia - anchored at capital city Addis Ababa (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_FIN	STATIC	60.1699	24.9384	4	verified	World Bank GDP growth API for Finland - anchored at capital city Helsinki (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_FJI	STATIC	-18.1416	178.4419	4	verified	World Bank GDP growth API for Fiji - anchored at capital city Suva (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_FRA	STATIC	48.8566	2.3522	4	verified	World Bank GDP growth API for France - anchored at capital city Paris (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_FRO	STATIC	62.0079	-6.7716	4	verified	World Bank GDP growth API for Faroe Islands - anchored at capital city Torshavn (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_FSM	STATIC	6.9147	158161	4	verified	World Bank GDP growth API for Micronesia - anchored at capital city Palikir (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GAB	STATIC	0.4162	9.4673	4	verified	World Bank GDP growth API for Gabon - anchored at capital city Libreville (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GBR	STATIC	51.5072	-0.1276	4	verified	World Bank GDP growth API for United Kingdom - anchored at capital city London (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GEO	STATIC	41.7151	44.8271	4	verified	World Bank GDP growth API for Georgia - anchored at capital city Tbilisi (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GHA	STATIC	5.6037	-187	4	verified	World Bank GDP growth API for Ghana - anchored at capital city Accra (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GIN	STATIC	9.6412	-13.5784	4	verified	World Bank GDP growth API for Guinea - anchored at capital city Conakry (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GMB	STATIC	13.4549	-16579	4	verified	World Bank GDP growth API for Gambia - anchored at capital city Banjul (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GNB	STATIC	11.8636	-15.5977	4	verified	World Bank GDP growth API for Guinea-Bissau - anchored at capital city Bissau (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GNQ	STATIC	3.7523	8.7742	4	verified	World Bank GDP growth API for Equatorial Guinea - anchored at capital city Malabo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GRC	STATIC	37.9838	23.7275	4	verified	World Bank GDP growth API for Greece - anchored at capital city Athens (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GRD	STATIC	12.0561	-61.7488	4	verified	World Bank GDP growth API for Grenada - anchored at capital city Saint George's (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GRL	STATIC	64.1835	-51.7216	4	verified	World Bank GDP growth API for Greenland - anchored at capital city Nuuk (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GTM	STATIC	14.6349	-90.5069	4	verified	World Bank GDP growth API for Guatemala - anchored at capital city Guatemala City (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GUM	STATIC	13.4745	144.7504	4	verified	World Bank GDP growth API for Guam - anchored at capital city Hagatna (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_GUY	STATIC	6.8013	-58.1551	4	verified	World Bank GDP growth API for Guyana - anchored at capital city Georgetown (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_HKG	STATIC	22.3193	114.1694	4	verified	World Bank GDP growth API for Hong Kong SAR, China - anchored at capital city Hong Kong (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_HND	STATIC	14.0723	-87.1921	4	verified	World Bank GDP growth API for Honduras - anchored at capital city Tegucigalpa (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_HRV	STATIC	45815	15.9819	4	verified	World Bank GDP growth API for Croatia - anchored at capital city Zagreb (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_HTI	STATIC	18.5944	-72.3074	4	verified	World Bank GDP growth API for Haiti - anchored at capital city Port-au-Prince (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_HUN	STATIC	47.4979	19.0402	4	verified	World Bank GDP growth API for Hungary - anchored at capital city Budapest (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_IDN	STATIC	-6.2088	106.8456	4	verified	World Bank GDP growth API for Indonesia - anchored at capital city Jakarta (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_IND	STATIC	28.6139	77209	4	verified	World Bank GDP growth API for India - anchored at capital city New Delhi (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_IRL	STATIC	53.3498	-6.2603	4	verified	World Bank GDP growth API for Ireland - anchored at capital city Dublin (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_IRN	STATIC	35.6892	51389	4	verified	World Bank GDP growth API for Iran - anchored at capital city Tehran (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_IRQ	STATIC	33.3152	44.3661	4	verified	World Bank GDP growth API for Iraq - anchored at capital city Baghdad (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ISL	STATIC	64.1466	-21.9426	4	verified	World Bank GDP growth API for Iceland - anchored at capital city Reykjavik (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ISR	STATIC	31.7683	35.2137	4	verified	World Bank GDP growth API for Israel - anchored at capital city Jerusalem (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ITA	STATIC	41.9028	12.4964	4	verified	World Bank GDP growth API for Italy - anchored at capital city Rome (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_JAM	STATIC	17.9712	-76.7936	4	verified	World Bank GDP growth API for Jamaica - anchored at capital city Kingston (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_JOR	STATIC	31.9454	35.9284	4	verified	World Bank GDP growth API for Jordan - anchored at capital city Amman (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_JPN	STATIC	35.6762	139.6503	4	verified	World Bank GDP growth API for Japan - anchored at capital city Tokyo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_KAZ	STATIC	51.1694	71.4491	4	verified	World Bank GDP growth API for Kazakhstan - anchored at capital city Astana (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_KEN	STATIC	-1.2921	36.8219	4	verified	World Bank GDP growth API for Kenya - anchored at capital city Nairobi (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_KGZ	STATIC	42.8746	74.5698	4	verified	World Bank GDP growth API for Kyrgyz Republic - anchored at capital city Bishkek (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_KHM	STATIC	11.5564	104.9282	4	verified	World Bank GDP growth API for Cambodia - anchored at capital city Phnom Penh (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_KIR	STATIC	1.3382	172.9797	4	verified	World Bank GDP growth API for Kiribati - anchored at capital city Tarawa (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_KNA	STATIC	17.3026	-62.7177	4	verified	World Bank GDP growth API for St. Kitts and Nevis - anchored at capital city Basseterre (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_KOR	STATIC	37.5665	126978	4	verified	World Bank GDP growth API for Korea, Rep. - anchored at capital city Seoul (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_KWT	STATIC	29.3759	47.9774	4	verified	World Bank GDP growth API for Kuwait - anchored at capital city Kuwait City (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LAO	STATIC	17.9757	102.6331	4	verified	World Bank GDP growth API for Lao PDR - anchored at capital city Vientiane (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LBN	STATIC	33.8938	35.5018	4	verified	World Bank GDP growth API for Lebanon - anchored at capital city Beirut (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LBR	STATIC	6.2907	-10.7605	4	verified	World Bank GDP growth API for Liberia - anchored at capital city Monrovia (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LBY	STATIC	32.8872	13.1913	4	verified	World Bank GDP growth API for Libya - anchored at capital city Tripoli (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LCA	STATIC	14.0101	-60.9875	4	verified	World Bank GDP growth API for St. Lucia - anchored at capital city Castries (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LIE	STATIC	47141	9.5209	4	verified	World Bank GDP growth API for Liechtenstein - anchored at capital city Vaduz (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LKA	STATIC	6901	79.9776	4	verified	World Bank GDP growth API for Sri Lanka - anchored at capital city Sri Jayawardenepura Kotte (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LSO	STATIC	-29.3151	27.4869	4	verified	World Bank GDP growth API for Lesotho - anchored at capital city Maseru (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LTU	STATIC	54.6872	25.2797	4	verified	World Bank GDP growth API for Lithuania - anchored at capital city Vilnius (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LUX	STATIC	49.6116	6.1319	4	verified	World Bank GDP growth API for Luxembourg - anchored at capital city Luxembourg (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_LVA	STATIC	56.9496	24.1052	4	verified	World Bank GDP growth API for Latvia - anchored at capital city Riga (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MAC	STATIC	22.1987	113.5439	4	verified	World Bank GDP growth API for Macao SAR, China - anchored at capital city Macao (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MAR	STATIC	34.0209	-6.8416	4	verified	World Bank GDP growth API for Morocco - anchored at capital city Rabat (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MCO	STATIC	43.7384	7.4246	4	verified	World Bank GDP growth API for Monaco - anchored at capital city Monaco (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MDA	STATIC	47.0105	28.8638	4	verified	World Bank GDP growth API for Moldova - anchored at capital city Chisinau (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MDG	STATIC	-18.8792	47.5079	4	verified	World Bank GDP growth API for Madagascar - anchored at capital city Antananarivo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MDV	STATIC	4.1755	73.5093	4	verified	World Bank GDP growth API for Maldives - anchored at capital city Male (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MEX	STATIC	19.4326	-99.1332	4	verified	World Bank GDP growth API for Mexico - anchored at capital city Mexico City (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MHL	STATIC	7.1164	171.1858	4	verified	World Bank GDP growth API for Marshall Islands - anchored at capital city Majuro (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MKD	STATIC	41.9981	21.4254	4	verified	World Bank GDP growth API for North Macedonia - anchored at capital city Skopje (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MLI	STATIC	12.6392	-8.0029	4	verified	World Bank GDP growth API for Mali - anchored at capital city Bamako (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MLT	STATIC	35.8989	14.5146	4	verified	World Bank GDP growth API for Malta - anchored at capital city Valletta (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MMR	STATIC	19.7633	96.0785	4	verified	World Bank GDP growth API for Myanmar - anchored at capital city Naypyidaw (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MNE	STATIC	42.4304	19.2594	4	verified	World Bank GDP growth API for Montenegro - anchored at capital city Podgorica (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MNG	STATIC	47.8864	106.9057	4	verified	World Bank GDP growth API for Mongolia - anchored at capital city Ulaanbaatar (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MNP	STATIC	15.19	145748	4	verified	World Bank GDP growth API for Northern Mariana Islands - anchored at capital city Saipan (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MOZ	STATIC	-25.9692	32.5732	4	verified	World Bank GDP growth API for Mozambique - anchored at capital city Maputo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MRT	STATIC	18.0735	-15.9582	4	verified	World Bank GDP growth API for Mauritania - anchored at capital city Nouakchott (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MUS	STATIC	-20.1609	57.5012	4	verified	World Bank GDP growth API for Mauritius - anchored at capital city Port Louis (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MWI	STATIC	-13.9626	33.7741	4	verified	World Bank GDP growth API for Malawi - anchored at capital city Lilongwe (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_MYS	STATIC	3139	101.6869	4	verified	World Bank GDP growth API for Malaysia - anchored at capital city Kuala Lumpur (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NAM	STATIC	-22.5609	17.0658	4	verified	World Bank GDP growth API for Namibia - anchored at capital city Windhoek (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NCL	STATIC	-22.2758	166458	4	verified	World Bank GDP growth API for New Caledonia - anchored at capital city Noumea (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NER	STATIC	13.5127	2.1128	4	verified	World Bank GDP growth API for Niger - anchored at capital city Niamey (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NGA	STATIC	9.0765	7.3986	4	verified	World Bank GDP growth API for Nigeria - anchored at capital city Abuja (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NIC	STATIC	12115	-86.2362	4	verified	World Bank GDP growth API for Nicaragua - anchored at capital city Managua (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NLD	STATIC	52.3676	4.9041	4	verified	World Bank GDP growth API for Netherlands - anchored at capital city Amsterdam (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NOR	STATIC	59.9139	10.7522	4	verified	World Bank GDP growth API for Norway - anchored at capital city Oslo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NPL	STATIC	27.7172	85324	4	verified	World Bank GDP growth API for Nepal - anchored at capital city Kathmandu (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NRU	STATIC	-0.5477	166.9209	4	verified	World Bank GDP growth API for Nauru - anchored at capital city Yaren (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_NZL	STATIC	-41.2865	174.7762	4	verified	World Bank GDP growth API for New Zealand - anchored at capital city Wellington (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_OMN	STATIC	23.5859	58.4059	4	verified	World Bank GDP growth API for Oman - anchored at capital city Muscat (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PAK	STATIC	33.6844	73.0479	4	verified	World Bank GDP growth API for Pakistan - anchored at capital city Islamabad (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PAN	STATIC	8.9824	-79.5199	4	verified	World Bank GDP growth API for Panama - anchored at capital city Panama City (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PER	STATIC	-12.0464	-77.0428	4	verified	World Bank GDP growth API for Peru - anchored at capital city Lima (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PHL	STATIC	14.5995	120.9842	4	verified	World Bank GDP growth API for Philippines - anchored at capital city Manila (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PLW	STATIC	7.5006	134.6242	4	verified	World Bank GDP growth API for Palau - anchored at capital city Ngerulmud (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PNG	STATIC	-9.4438	147.1803	4	verified	World Bank GDP growth API for Papua New Guinea - anchored at capital city Port Moresby (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_POL	STATIC	52.2297	21.0122	4	verified	World Bank GDP growth API for Poland - anchored at capital city Warsaw (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PRI	STATIC	18.4655	-66.1057	4	verified	World Bank GDP growth API for Puerto Rico - anchored at capital city San Juan (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PRK	STATIC	39.0392	125.7625	4	verified	World Bank GDP growth API for Korea, Dem. People's Rep. - anchored at capital city Pyongyang (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PRT	STATIC	38.7223	-9.1393	4	verified	World Bank GDP growth API for Portugal - anchored at capital city Lisbon (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PRY	STATIC	-25.2637	-57.5759	4	verified	World Bank GDP growth API for Paraguay - anchored at capital city Asuncion (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PSE	STATIC	31.8996	35.2042	4	verified	World Bank GDP growth API for West Bank and Gaza - anchored at capital city Ramallah (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_PYF	STATIC	-17.5516	-149.5585	4	verified	World Bank GDP growth API for French Polynesia - anchored at capital city Papeete (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_QAT	STATIC	25.2854	51531	4	verified	World Bank GDP growth API for Qatar - anchored at capital city Doha (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ROU	STATIC	44.4268	26.1025	4	verified	World Bank GDP growth API for Romania - anchored at capital city Bucharest (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_RUS	STATIC	55.7558	37.6173	4	verified	World Bank GDP growth API for Russian Federation - anchored at capital city Moscow (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_RWA	STATIC	-1.9403	29.8739	4	verified	World Bank GDP growth API for Rwanda - anchored at capital city Kigali (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SAU	STATIC	24.7136	46.6753	4	verified	World Bank GDP growth API for Saudi Arabia - anchored at capital city Riyadh (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SDN	STATIC	15.5007	32.5599	4	verified	World Bank GDP growth API for Sudan - anchored at capital city Khartoum (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SEN	STATIC	14.7167	-17.4677	4	verified	World Bank GDP growth API for Senegal - anchored at capital city Dakar (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SGP	STATIC	1.3521	103.8198	4	verified	World Bank GDP growth API for Singapore - anchored at capital city Singapore (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SLB	STATIC	-9428	159.95	4	verified	World Bank GDP growth API for Solomon Islands - anchored at capital city Honiara (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SLE	STATIC	8.4657	-13.2317	4	verified	World Bank GDP growth API for Sierra Leone - anchored at capital city Freetown (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SLV	STATIC	13.6929	-89.2182	4	verified	World Bank GDP growth API for El Salvador - anchored at capital city San Salvador (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SMR	STATIC	43.9424	12.4578	4	verified	World Bank GDP growth API for San Marino - anchored at capital city San Marino (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SOM	STATIC	2.0469	45.3182	4	verified	World Bank GDP growth API for Somalia - anchored at capital city Mogadishu (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SRB	STATIC	44.7866	20.4489	4	verified	World Bank GDP growth API for Serbia - anchored at capital city Belgrade (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SSD	STATIC	4.8517	31.5825	4	verified	World Bank GDP growth API for South Sudan - anchored at capital city Juba (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_STP	STATIC	0.3365	6.6731	4	verified	World Bank GDP growth API for Sao Tome and Principe - anchored at capital city Sao Tome (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SUR	STATIC	5852	-55.2038	4	verified	World Bank GDP growth API for Suriname - anchored at capital city Paramaribo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SVK	STATIC	48.1486	17.1077	4	verified	World Bank GDP growth API for Slovak Republic - anchored at capital city Bratislava (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SVN	STATIC	46.0569	14.5058	4	verified	World Bank GDP growth API for Slovenia - anchored at capital city Ljubljana (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SWE	STATIC	59.3293	18.0686	4	verified	World Bank GDP growth API for Sweden - anchored at capital city Stockholm (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SWZ	STATIC	-26.3054	31.1367	4	verified	World Bank GDP growth API for Eswatini - anchored at capital city Mbabane (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SXM	STATIC	18.0237	-63.0458	4	verified	World Bank GDP growth API for Sint Maarten - anchored at capital city Philipsburg (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SYC	STATIC	-4.6191	55.4513	4	verified	World Bank GDP growth API for Seychelles - anchored at capital city Victoria (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_SYR	STATIC	33.5138	36.2765	4	verified	World Bank GDP growth API for Syrian Arab Republic - anchored at capital city Damascus (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TCD	STATIC	12.1348	15.0557	4	verified	World Bank GDP growth API for Chad - anchored at capital city N'Djamena (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TGO	STATIC	6.1725	1.2314	4	verified	World Bank GDP growth API for Togo - anchored at capital city Lome (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_THA	STATIC	13.7563	100.5018	4	verified	World Bank GDP growth API for Thailand - anchored at capital city Bangkok (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TJK	STATIC	38.5598	68787	4	verified	World Bank GDP growth API for Tajikistan - anchored at capital city Dushanbe (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TKM	STATIC	37.9601	58.3261	4	verified	World Bank GDP growth API for Turkmenistan - anchored at capital city Ashgabat (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TLS	STATIC	-8.5569	125.5603	4	verified	World Bank GDP growth API for Timor-Leste - anchored at capital city Dili (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TON	STATIC	-21.1393	-175.2049	4	verified	World Bank GDP growth API for Tonga - anchored at capital city Nuku'alofa (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TTO	STATIC	10.6549	-61.5019	4	verified	World Bank GDP growth API for Trinidad and Tobago - anchored at capital city Port of Spain (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TUN	STATIC	36.8065	10.1815	4	verified	World Bank GDP growth API for Tunisia - anchored at capital city Tunis (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TUR	STATIC	39.9334	32.8597	4	verified	World Bank GDP growth API for Turkiye - anchored at capital city Ankara (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TUV	STATIC	-8.5211	179.1983	4	verified	World Bank GDP growth API for Tuvalu - anchored at capital city Funafuti (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_TZA	STATIC	-6163	35.7516	4	verified	World Bank GDP growth API for Tanzania - anchored at capital city Dodoma (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_UGA	STATIC	0.3476	32.5825	4	verified	World Bank GDP growth API for Uganda - anchored at capital city Kampala (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_UKR	STATIC	50.4501	30.5234	4	verified	World Bank GDP growth API for Ukraine - anchored at capital city Kyiv (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_URY	STATIC	-34.9011	-56.1645	4	verified	World Bank GDP growth API for Uruguay - anchored at capital city Montevideo (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_USA	STATIC	38.9072	-77.0369	4	verified	World Bank GDP growth API for United States - anchored at capital city Washington, D.C. (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_UZB	STATIC	41.2995	69.2401	4	verified	World Bank GDP growth API for Uzbekistan - anchored at capital city Tashkent (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_VCT	STATIC	13.16	-61.2248	4	verified	World Bank GDP growth API for St. Vincent and the Grenadines - anchored at capital city Kingstown (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_VEN	STATIC	10.4806	-66.9036	4	verified	World Bank GDP growth API for Venezuela, RB - anchored at capital city Caracas (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_VGB	STATIC	18.4283	-64.62	4	verified	World Bank GDP growth API for British Virgin Islands - anchored at capital city Road Town (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_VIR	STATIC	18.3419	-64.9307	4	verified	World Bank GDP growth API for Virgin Islands (U.S.) - anchored at capital city Charlotte Amalie (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_VNM	STATIC	21.0278	105.8342	4	verified	World Bank GDP growth API for Vietnam - anchored at capital city Hanoi (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_VUT	STATIC	-17.7333	168.3273	4	verified	World Bank GDP growth API for Vanuatu - anchored at capital city Port Vila (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_WSM	STATIC	-13.8506	-171.7513	4	verified	World Bank GDP growth API for Samoa - anchored at capital city Apia (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_XKX	STATIC	42.6629	21.1655	4	verified	World Bank GDP growth API for Kosovo - anchored at capital city Pristina (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_YEM	STATIC	15.3694	44191	4	verified	World Bank GDP growth API for Yemen, Rep. - anchored at capital city Sana'a (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ZAF	STATIC	-25.7479	28.2293	4	verified	World Bank GDP growth API for South Africa - anchored at capital city Pretoria (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ZMB	STATIC	-15.3875	28.3228	4	verified	World Bank GDP growth API for Zambia - anchored at capital city Lusaka (standard geographic reference, not an institutional guess)	
worldbank_gdp_growth_ZWE	STATIC	-17.8252	31.0335	4	verified	World Bank GDP growth API for Zimbabwe - anchored at capital city Harare (standard geographic reference, not an institutional guess)	

