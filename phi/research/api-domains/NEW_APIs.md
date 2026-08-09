source blitzortung_lightning_local
ttl 60
url https://api.blitzortung.org/v1/strikes/latest?lat={lat}&lon={lon}&radius=100
count . atmosphere_lightning_strikes_count_100km

source usgs_geomag_boulder_realtime
ttl 60
url https://geomag.usgs.gov/ws/data/?id=BOU&type=adjusted&starttime={today}T00:00:00&endtime={today}T23:59:59&format=json&sampling_period=60
field BOU.0.0.0 magnetometer_x_bou_nt
field BOU.0.0.1 magnetometer_y_bou_nt
field BOU.0.0.2 magnetometer_z_bou_nt

source waterquality_nitrate_local
ttl 3600
url https://www.waterqualitydata.us/data/Result/search?siteType=Stream&characteristicName=Nitrate&lat={lat}&long={lon}&radius=25&mimeType=json
count value hydrosphere_nitrate_measurements_count_25km

source aeronet_aod_mauna_loa
ttl 86400
url https://aeronet.gsfc.nasa.gov/cgi-bin/print_web_data_v3?site=Mauna_Loa&year={year}&month={month}&day={day}&year2={year}&month2={month}&day2={day}&AOD15=1&AVG=10&if_no_html=1
format csv
last_row AOD_500nm atmosphere_aerosol_optical_depth_mauna_loa

source noaa_chlorophyll_satellite_local
ttl 86400
url https://coastwatch.pfeg.noaa.gov/erddap/griddap/erdMH1chla1day.json?chlorophyll[(last)][(0.0)][({lat})][({lon})]
path table.rows.0.4 biosphere_chlorophyll_satellite_mgm3

source noaa_wind_station_8724580
ttl 300
lat 24.5557
lon -81.8079
url https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station=8724580&product=wind&time_zone=gmt&units=metric&format=json&date=latest
first data.0.s atmosphere_wind_speed_keywest_ms
first data.0.dr atmosphere_wind_dir_keywest_deg

source argovis_local_profiles
ttl 86400
url https://argovis-api.colorado.edu/argo?center={lon},{lat}&radius=100&startDate={yesterday}T00:00:00Z&endDate={today}T23:59:59Z
count . hydrosphere_argo_profiles_count_100km

source cosmic_rays_oulu
ttl 3600
res 0
url http://cosmicrays.oulu.fi/phi/Pulu_1min.txt
format csv
last_row 5 cosmic_neutron_flux_oulu_cpm

source ndbc_buoy_51001
ttl 300
url https://www.ndbc.noaa.gov/data/realtime2/51001.txt
format csv
last_row WTMP hydrosphere_buoy_51001_water_temp
last_row WVHT hydrosphere_buoy_51001_wave_height
last_row DPD hydrosphere_buoy_51001_dominant_period

source iris_seismic_stations_local
ttl 2592000
url https://service.iris.edu/fdsnws/station/1/query?format=text&latitude={lat}&longitude={lon}&maxradius=5
count lines geosphere_seismic_stations_5deg

source noaa_currents_puget
ttl 300
lat 47.24
lon -122.43
url https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station=PUG1511&product=currents&time_zone=gmt&units=metric&format=json&date=latest
first data.0.speed hydrosphere_current_speed_puget_ms
first data.0.direction hydrosphere_current_dir_puget_deg

source intermagnet_frn
ttl 300
url https://imag-data.bgs.ac.uk/GIN_V1/GINServices?Request=GetData&format=JSON&observatoryIagaCode=FRN&samplesPerDay=1440&dataStartDate={today}&dataDuration=1
field X magnetosphere_x_frn_nt
field Y magnetosphere_y_frn_nt
field Z magnetosphere_z_frn_nt

source xenocanto_local_birds
ttl 86400
url https://www.xeno-canto.org/api/2/recordings?query=lat:{lat},lon:{lon}
field numRecordings biosphere_bird_sounds_local_count

source nasa_gcn_circulars
ttl 300
url https://gcn.nasa.gov/circulars.json?limit=10
count circulars cosmic_gcn_alerts_count

source caida_internet_outages
ttl 300
url https://api.ioda.caida.org/v2/outages/asn/?from=-3600&to=now
count data technosphere_global_outage_events

source noaa_gml_cfc11_global
ttl 2592000
url https://gml.noaa.gov/aftp/data/hats/cfcs/cfc11/combined/HATS_global_F11.txt
format csv
last_row global_mean atmosphere_cfc11_global_ppt

source noaa_gml_cfc12_global
ttl 2592000
url https://gml.noaa.gov/aftp/data/hats/cfcs/cfc12/combined/HATS_global_F12.txt
format csv
last_row global_mean atmosphere_cfc12_global_ppt

source usgs_streamflow_local
ttl 300
url https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=00060&siteStatus=active
path value.timeSeries.0.values.0.value.0.value hydrosphere_local_river_flow_cfs

source sensor_community_local_pm
ttl 300
url https://data.sensor.community/airrohr/v1/filter/area={lat},{lon},10
count . atmosphere_local_pm_sensors_count

source satnogs_observations_global
ttl 300
url https://network.satnogs.org/api/observations/?format=json&limit=1
field count technosphere_satnogs_total_observations

source usgs_water_temp_local
ttl 300
url https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=00010&siteStatus=active
path value.timeSeries.0.values.0.value.0.value hydrosphere_local_water_temp_c

source nmdb_cosmic_rays_newk
ttl 3600
res 0
url http://www.nmdb.eu/nest/dynamics.php?stations=NEWK&output=json&resolution=60&history=300
path data.NEWK.1h.0.1 cosmic_neutron_flux_newk

source ndbc_buoy_44065
ttl 300
url https://www.ndbc.noaa.gov/data/realtime2/44065.txt
format csv
last_row WTMP hydrosphere_buoy_44065_water_temp
last_row WVHT hydrosphere_buoy_44065_wave_height
last_row DPD hydrosphere_buoy_44065_dominant_period

source glodap_ocean_ph_measurements
ttl 2592000
url https://erddap.emodnet-physics.eu/erddap/tabledap/GLODAPv2_2023.json?&salinity,pH&distinct()
count table.rows hydrosphere_glodap_global_ph_samples_count

source intermagnet_hon
ttl 300
url https://imag-data.bgs.ac.uk/GIN_V1/GINServices?Request=GetData&format=JSON&observatoryIagaCode=HON&samplesPerDay=1440&dataStartDate={today}&dataDuration=1
field X magnetosphere_x_hon_nt
field Y magnetosphere_y_hon_nt
field Z magnetosphere_z_hon_nt

source opensky_local_aircraft
ttl 30
url https://opensky-network.org/api/states/all?lamin={lat_min}&lomin={lon_min}&lamax={lat_max}&lomax={lon_max}
count states technosphere_local_aircraft_count

source nws_asos_jfk
ttl 300
url https://api.weather.gov/stations/KJFK/observations/latest
path properties.temperature.value atmosphere_temp_jfk_c
path properties.barometricPressure.value atmosphere_pressure_jfk_pa
path properties.windSpeed.value atmosphere_wind_speed_jfk_ms

source iem_nldn_lightning_us
ttl 3600
url https://mesonet.agron.iastate.edu/api/1/nldn.geojson?sts={yesterday}T00:00:00Z&ets={today}T00:00:00Z
count features atmosphere_lightning_strikes_us_24h

source met_norway_oslo_temp
ttl 3600
header User-Agent "omegaflow"
url https://api.met.no/weatherapi/observationdata/2.0/?source=SN18700&referencetime=latest&elements=air_temperature
path data.observations.0.value atmosphere_temp_oslo_c

source digitraffic_ais_finland
ttl 60
url https://meri.digitraffic.fi/api/ais/v1/locations
count . technosphere_ships_tracking_finland

source nasa_firms_active_fires
ttl 10800
url https://firms.modaps.eosdis.nasa.gov/data/active_fire/suomi-npp-viirs-c2/csv/SUOMI_VIIRS_C2_Global_24h.csv
format csv
count lines geosphere_global_active_fires_24h

source usgs_water_conductivity_local
ttl 300
url https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=00095&siteStatus=active
path value.timeSeries.0.values.0.value.0.value hydrosphere_local_water_conductivity_uscm

source usgs_volcanoes_elevated
ttl 3600
url https://volcanoes.usgs.gov/hans-public/api/volcano/getElevatedVolcanoes
count . geosphere_elevated_volcanoes_us

source raspberry_shake_local
ttl 2592000
url https://fdsnws.raspberryshakedata.com/fdsnws/station/1/query?format=text&latitude={lat}&longitude={lon}&maxradius=5
count lines geosphere_raspberry_shake_stations_5deg

source usgs_groundwater_local
ttl 3600
url https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=72019&siteStatus=active
path value.timeSeries.0.values.0.value.0.value hydrosphere_local_groundwater_depth_ft

source usgs_ph_local
ttl 3600
url https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=00400&siteStatus=active
path value.timeSeries.0.values.0.value.0.value hydrosphere_local_river_ph

source usgs_turbidity_local
ttl 3600
url https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=63680&siteStatus=active
path value.timeSeries.0.values.0.value.0.value hydrosphere_local_turbidity_fnu

source usgs_dissolved_oxygen_local
ttl 3600
url https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=00300&siteStatus=active
path value.timeSeries.0.values.0.value.0.value hydrosphere_local_dissolved_oxygen_mgl

source ndbc_spectral_waves_41001
ttl 300
url https://www.ndbc.noaa.gov/data/realtime2/41001.spec
format csv
last_row SwH hydrosphere_swell_height_41001_m
last_row SwP hydrosphere_swell_period_41001_s

source cosmic_rays_south_pole
ttl 3600
res 0
url http://www.nmdb.eu/nest/dynamics.php?stations=SPO&output=json&resolution=60&history=300
path data.SPO.1h.0.1 cosmic_neutron_flux_south_pole

source met_norway_oslo_precip
ttl 3600
header User-Agent "omegaflow"
url https://api.met.no/weatherapi/observationdata/2.0/?source=SN18700&referencetime=latest&elements=precipitation_amount
path data.observations.0.value atmosphere_precipitation_oslo_mm

source noaa_air_temp_8454000
ttl 300
lat 41.8067
lon -71.4006
url https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station=8454000&product=air_temperature&date=latest&time_zone=gmt&units=metric&format=json
first v atmosphere_air_temp_providence_c

source noaa_gml_co2_daily
ttl 86400
res 0
url https://gml.noaa.gov/webdata/ccgg/trends/co2/co2_daily_mlo.txt
format csv
last_row value atmosphere_co2_daily_mlo_ppm

source noaa_gml_co2_monthly
ttl 2592000
res 0
url https://gml.noaa.gov/webdata/ccgg/trends/co2/co2_mm_mlo.txt
format csv
last_row average atmosphere_co2_monthly_mlo_ppm

source noaa_gml_ch4
ttl 2592000
res 0
url https://gml.noaa.gov/webdata/ccgg/trends/ch4/ch4_mm_gl.txt
format csv
last_row average atmosphere_ch4_monthly_global_ppb

source noaa_gml_n2o
ttl 2592000
res 0
url https://gml.noaa.gov/webdata/ccgg/trends/n2o/n2o_mm_gl.txt
format csv
last_row average atmosphere_n2o_monthly_global_ppb

source noaa_gml_sf6
ttl 2592000
res 0
url https://gml.noaa.gov/webdata/ccgg/trends/sf6/sf6_mm_gl.txt
format csv
last_row average atmosphere_sf6_monthly_global_ppt

source noaa_gml_co
ttl 2592000
res 0
url https://gml.noaa.gov/webdata/ccgg/trends/co/co_mm_gl.txt
format csv
last_row average atmosphere_co_monthly_global_ppb

source gbif_local_records
ttl 86400
url https://api.gbif.org/v1/occurrence/search?decimalLatitude={lat}&decimalLongitude={lon}&radius=10&limit=0
field count biosphere_gbif_records_10km

source gbif_recent_30d
ttl 3600
url https://api.gbif.org/v1/occurrence/search?decimalLatitude={lat}&decimalLongitude={lon}&radius=50&hasCoordinate=true&eventDate=LAST_30_DAYS&limit=0
field count biosphere_gbif_recent_50km

source inaturalist_local_research
ttl 3600
url https://api.inaturalist.org/v1/observations?lat={lat}&lng={lon}&radius=10&quality_grade=research&per_page=0
field total_results biosphere_inaturalist_research_10km

source obis_ocean_species
ttl 86400
url https://api.obis.org/occurrence?lat={lat}&lon={lon}&radius=50&size=0
field count biosphere_obis_species_50km

source openmeteo_marine
ttl 600
url https://marine-api.open-meteo.com/v1/marine?latitude={lat}&longitude={lon}&current=wave_height,wave_direction,wave_period,wind_wave_height,swell_wave_height
field current.wave_height hydrosphere_wave_height_m
field current.wave_direction hydrosphere_wave_direction_deg
field current.wave_period hydrosphere_wave_period_s
field current.wind_wave_height hydrosphere_wind_wave_height_m
field current.swell_wave_height hydrosphere_swell_wave_height_m

source openmeteo_soil
ttl 3600
url https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&hourly=soil_temperature_0_to_7cm,soil_moisture_0_to_7cm&forecast_hours=1&timezone=auto
first hourly.soil_temperature_0_to_7cm geosphere_soil_temp_7cm_c
first hourly.soil_moisture_0_to_7cm geosphere_soil_moisture_7cm_m3m3

source openmeteo_convection
ttl 3600
url https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&hourly=cape,lifted_index,convective_inhibition&forecast_hours=1&timezone=auto
first hourly.cape atmosphere_cape_jkg
first hourly.lifted_index atmosphere_lifted_index
first hourly.convective_inhibition atmosphere_cin_jkg

source openmeteo_elevation
ttl 2592000
url https://api.open-meteo.com/v1/elevation?latitude={lat}&longitude={lon}
field elevation geosphere_elevation_m

source noaa_sst_satellite
ttl 86400
url https://coastwatch.pfeg.noaa.gov/erddap/griddap/jplMURSST41.json?analysed_sst[(last)][(0.0)][({lat})][({lon})]
path table.rows.0.4 hydrosphere_sst_satellite_c

source noaa_coral_dhw
ttl 86400
url https://coastwatch.pfeg.noaa.gov/erddap/griddap/NOAA_DHW.json?degree_heating_week[(last)][({lat})][({lon})]
path table.rows.0.4 biosphere_coral_degree_heating_weeks

source soilgrids_ph
ttl 2592000
url https://rest.isric.org/soilgrids/v2.0/properties/query?lon={lon}&lat={lat}&property=phh2o&depth=0-5cm&value=mean
path properties.phh2o.depths.0.values.mean geosphere_soil_ph_0_5cm

source soilgrids_clay
ttl 2592000
url https://rest.isric.org/soilgrids/v2.0/properties/query?lon={lon}&lat={lat}&property=clay&depth=0-5cm&value=mean
path properties.clay.depths.0.values.mean geosphere_soil_clay_0_5cm_gkg

source soilgrids_carbon
ttl 2592000
url https://rest.isric.org/soilgrids/v2.0/properties/query?lon={lon}&lat={lat}&property=soc&depth=0-5cm&value=mean
path properties.soc.depths.0.values.mean geosphere_soil_organic_carbon_0_5cm

source nasa_fireballs
ttl 86400
res 0
url https://ssd-api.jpl.nasa.gov/fireball.api?limit=10
count data cosmic_fireball_count

source nasa_eonet_open
ttl 3600
url https://eonet.gsfc.nasa.gov/api/v3/events?status=open&limit=100
count events geosphere_eonet_open_events

source ndbc_buoy_62001
ttl 300
lat 59.86
lon -19.55
url https://www.ndbc.noaa.gov/data/realtime2/62001.txt
format csv
last_row WTMP hydrosphere_buoy_62001_water_temp
last_row WVHT hydrosphere_buoy_62001_wave_height
last_row PRES hydrosphere_buoy_62001_pressure

source ndbc_buoy_46001
ttl 300
lat 56.30
lon -148.02
url https://www.ndbc.noaa.gov/data/realtime2/46001.txt
format csv
last_row WTMP hydrosphere_buoy_46001_water_temp
last_row WVHT hydrosphere_buoy_46001_wave_height
last_row PRES hydrosphere_buoy_46001_pressure

source ndbc_buoy_51004
ttl 300
lat 0.00
lon -170.46
url https://www.ndbc.noaa.gov/data/realtime2/51004.txt
format csv
last_row WTMP hydrosphere_buoy_51004_water_temp
last_row WVHT hydrosphere_buoy_51004_wave_height

source ndbc_buoy_23001
ttl 300
lat 0.00
lon 81.00
url https://www.ndbc.noaa.gov/data/realtime2/23001.txt
format csv
last_row WTMP hydrosphere_buoy_23001_water_temp
last_row WVHT hydrosphere_buoy_23001_wave_height

source ndbc_buoy_42001
ttl 300
lat 25.92
lon -89.64
url https://www.ndbc.noaa.gov/data/realtime2/42001.txt
format csv
last_row WTMP hydrosphere_buoy_42001_water_temp
last_row WVHT hydrosphere_buoy_42001_wave_height
last_row PRES hydrosphere_buoy_42001_pressure

# ==========================================
# IONOSPHÄRE — Funkwellenausbreitung
# ==========================================

source noaa_ionosphere_fof2_global
ttl 300
res 0
url https://services.swpc.noaa.gov/json/ionospheric_foF2.json
last fof2 ionosphere_fof2_global_mhz

source noaa_tec_global
ttl 300
res 0
url https://services.swpc.noaa.gov/json/rtsw/rtsw_tec_global.json
last value ionosphere_tec_global_tecu

source giro_fof2_vienna_24h
ttl 900
res 0
url https://lgdc.uml.edu/common/DIDBGetValues?ursiCode=AT138&charName=foF2&startDate={yesterday}T00:00:00Z&endDate={today}T23:59:59Z&format=json
# Die Antwort ist ein Array von Zeit-Wert-Paaren — letzter Wert:
# Anmerkung: Das JSON-Format von LGDC ist verschachtelt, 
# du brauchst vermutlich einen speziellen Extractor dafür

source giro_hmf2_vienna_24h
ttl 900
res 0
url https://lgdc.uml.edu/common/DIDBGetValues?ursiCode=AT138&charName=hmF2&startDate={yesterday}T00:00:00Z&endDate={today}T23:59:59Z&format=json
