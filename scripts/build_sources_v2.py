#!/usr/bin/env python3
"""Build sources_v2.phi from parser dump + API specs + sources.φ metadata."""
import re, os, csv
from collections import defaultdict

DUMP_PATH = "phi/recovery/migration/dump_fields.csv"
SOURCES_PATH = "phi/sources.φ"
OUTPUT_PATH = "phi/recovery/migration/sources_v2.phi"

# ===================================================================
# SPEC MAPS — parameter_name -> "force unit"
# Expand per API domain. Each entry maps the API's parameter name
# (as it appears in the URL query string or JSON response) to a
# force+unit pair.
# ===================================================================

SPEC = {}

# Open-Meteo (from https://open-meteo.com/en/docs)
SPEC.update({k: v for k, v in {
    'temperature_2m':'thermal C','relative_humidity_2m':'diffusion %','dew_point_2m':'thermal C',
    'apparent_temperature':'thermal C','pressure_msl':'advective hPa','surface_pressure':'advective hPa',
    'wind_speed_10m':'advective km/h','wind_direction_10m':'advective deg','wind_gusts_10m':'advective km/h',
    'shortwave_radiation':'em W/m2','direct_radiation':'em W/m2','direct_normal_irradiance':'em W/m2',
    'diffuse_radiation':'em W/m2','global_tilted_irradiance':'em W/m2',
    'vapour_pressure_deficit':'diffusion kPa','evapotranspiration':'diffusion mm',
    'et0_fao_evapotranspiration':'diffusion mm','precipitation':'acoustic mm',
    'snowfall':'acoustic cm','rain':'acoustic mm','showers':'acoustic mm',
    'snow_depth':'acoustic m','freezing_level_height':'gravity m','visibility':'diffusion m',
    'soil_temperature_0cm':'thermal C','soil_temperature_6cm':'thermal C',
    'soil_temperature_18cm':'thermal C','soil_temperature_54cm':'thermal C',
    'soil_moisture_0_to_1cm':'diffusion %','soil_moisture_1_to_3cm':'diffusion %',
    'soil_moisture_3_to_9cm':'diffusion %','soil_moisture_9_to_27cm':'diffusion %',
    'soil_moisture_27_to_81cm':'diffusion %','temperature_2m_max':'thermal C',
    'temperature_2m_min':'thermal C','precipitation_sum':'acoustic mm','rain_sum':'acoustic mm',
    'shortwave_radiation_sum':'em W/m2','et0_fao_evapotranspiration_sum':'diffusion mm',
    'snowfall_sum':'acoustic cm','wind_speed_10m_max':'advective km/h',
    'wind_gusts_10m_max':'advective km/h','sunshine_duration':'acoustic s',
    'cloud_cover':'diffusion %','cloud_cover_low':'diffusion %','cloud_cover_mid':'diffusion %',
    'cloud_cover_high':'diffusion %','boundary_layer_height':'gravity m',
    'temperature_80m':'thermal C','temperature_120m':'thermal C',
    'temperature_180m':'thermal C','wind_direction_80m':'advective deg',
    'wind_speed_80m':'advective km/h','wind_speed_120m':'advective km/h',
    'wind_speed_180m':'advective km/h','wind_direction_120m':'advective deg',
    'wind_direction_180m':'advective deg','wet_bulb_temperature_2m':'thermal C',
    'snowfall_water_equivalent_sum':'acoustic mm',
    'cape':'advective J/kg','lightning_potential':'em J/kg',
    'convective_inhibition':'advective J/kg',
    'soil_temperature_54cm':'thermal C','mean_temperature_2m':'thermal C',
    'mean_sea_level_pressure':'advective hPa','mean_surface_pressure':'advective hPa',
    'mean_wind_speed_10m':'advective km/h','mean_visibility':'diffusion m',
    'max_temperature_2m':'thermal C','min_temperature_2m':'thermal C',
    'max_sea_level_pressure':'advective hPa','min_sea_level_pressure':'advective hPa',
    'max_surface_pressure':'advective hPa','min_surface_pressure':'advective hPa',
    'max_visibility':'diffusion m','min_visibility':'diffusion m',
    'max_wind_speed_10m':'advective km/h','max_wind_gusts_10m':'advective km/h',
    'max_dewpoint_2m':'thermal C','min_dewpoint_2m':'thermal C',
    'max_relative_humidity_2m':'diffusion %','min_relative_humidity_2m':'diffusion %',
    'max_vapour_pressure_deficit':'diffusion kPa',
    'mean_relative_humidity_2m':'diffusion %','mean_dewpoint_2m':'thermal C',
    'mean_wet_bulb_temperature_2m':'thermal C','mean_apparent_temperature':'thermal C',
    'max_apparent_temperature':'thermal C','min_apparent_temperature':'thermal C',
    'mean_cloud_cover':'diffusion %','max_cape':'advective J/kg','min_cape':'advective J/kg',
    'mean_wind_gusts_10m':'advective km/h','max_wind_gusts_10m':'advective km/h',
    'mean_sea_level_pressure':'advective hPa','mean_surface_pressure':'advective hPa',
    'growing_degree_days_base_0_limit_50':'thermal K·d',
    'leaf_wetness_probability':'diffusion %','precipitation_probability':'diffusion %',
    'precipitation_probability_max':'diffusion %',
    'reference_evapotranspiration':'diffusion mm','reference_evapotranspiration_sum':'diffusion mm',
}.items()})

# SWPC/GOES (from product documentation)
SPEC.update({k: v for k, v in {
    'proton_speed':'advective km/s','proton_density':'diffusion p/cm3',
    'proton_temperature':'thermal K','bt':'em nT','bx_gse':'em nT',
    'by_gse':'em nT','bz_gse':'em nT','bx_gsm':'em nT','by_gsm':'em nT',
    'bz_gsm':'em nT','hp':'em nT','he':'em nT','flux':'em W/m2',
    'radio_flux':'em sfu','f10.7':'em sfu',
}.items()})

# NDBC (from https://www.ndbc.noaa.gov/measdes.shtml)
SPEC.update({k: v for k, v in {
    'WDIR':'advective deg','WSPD':'advective m/s','GST':'advective m/s',
    'WVHT':'acoustic m','DPD':'acoustic s','APD':'acoustic s','MWD':'acoustic deg',
    'PRES':'advective hPa','ATMP':'thermal C','WTMP':'thermal C','DEWP':'thermal C',
    'VIS':'diffusion nmi','PTDY':'advective hPa','TIDE':'acoustic ft',
    'SwH':'acoustic m','SwP':'acoustic s','WWH':'acoustic m','WWP':'acoustic s',
    'SwD':'acoustic deg','WWD':'acoustic deg','CHILL':'thermal C','HEAT':'thermal C',
    'WSPD10':'advective m/s','WSPD20':'advective m/s','BAR':'advective hPa',
    'GDR':'advective deg','GTIME':'acoustic s','HEIGHT':'acoustic m',
    'temperature':'thermal C','salinity':'diffusion PSU',
    'conductivity':'electric μS/cm','pressure':'advective hPa',
    'current_speed':'advective m/s','current_direction':'advective deg',
    'current_u':'advective m/s','current_v':'advective m/s',
    'air_temperature':'thermal C','water_temperature':'thermal C',
    'sea_water_temperature':'thermal C','sea_water_salinity':'diffusion PSU',
    'sea_water_electrical_conductivity':'electric μS/cm',
    'significant_wave_height':'acoustic m','dominant_wave_period':'acoustic s',
    'wave_direction':'acoustic deg','air_pressure':'advective hPa',
    'dew_point_temperature':'thermal C','visibility_in_air':'diffusion nmi',
    'wind_speed':'advective m/s','wind_direction':'advective deg',
    'wind_gust':'advective m/s','relative_humidity':'diffusion %',
    'precipitation':'acoustic mm','snow_depth':'acoustic m',
    'sea_floor_depth_below_sea_surface':'seismic-body m',
    'depth':'seismic-body km','elevation':'gravity m',
}.items()})

# NCEI/NOAA Daily Summaries (from live API response)
SPEC.update({k: v for k, v in {
    'TMAX':'thermal C','TMIN':'thermal C','TAVG':'thermal C','TEMP':'thermal C',
    'MAX':'thermal C','MIN':'thermal C','PRCP':'acoustic mm','SNOW':'acoustic mm',
    'SNWD':'acoustic mm','WDSP':'advective m/s','SLP':'advective hPa',
    'PRES':'advective hPa','VISIB':'diffusion m','DEWP':'thermal C',
    'AWND':'advective m/s','WDF2':'advective deg','WDF5':'advective deg',
    'WSF2':'advective m/s','WSF5':'advective m/s','PGTM':'advective hPa',
    'PSUN':'acoustic s','TSUN':'acoustic s',
}.items()})

# FIRMS fire data (from NASA FIRMS API docs)
SPEC.update({k: v for k, v in {
    'bright_ti4':'thermal K','bright_ti5':'thermal K','bright_t31':'thermal K',
    'frp':'em MW','FRP':'em MW','scan':'advective m','track':'advective m',
    'confidence':'diffusion %',
}.items()})

# USGS Water Data (from waterservices live data)
SPEC.update({k: v for k, v in {
    'discharge':'advective m3/s','gage_height':'acoustic m',
    'temperature_water':'thermal C','specific_conductance':'electric μS/cm',
    'dissolved_oxygen':'diffusion mg/L','turbidity':'diffusion NTU',
    'ph':'diffusion pH','nitrate':'diffusion mg/L',
    'chlorophyll':'diffusion μg/L','precipitation':'acoustic mm',
}.items()})

# NOAA Tides & Currents (product-based)
SPEC.update({k: v for k, v in {
    'water_level':'acoustic m','air_pressure':'advective hPa',
    'water_temperature':'thermal C','air_temperature':'thermal C',
    'wind':'advective m/s','conductivity':'electric μS/cm',
    'salinity':'diffusion PSU','predictions':'acoustic m','currents':'advective m/s',
    'visibility':'diffusion nmi','humidity':'diffusion %',
    'water_level_6min':'acoustic m','hourly_height':'acoustic m',
    'high_low':'acoustic m','monthly_mean':'acoustic m',
}.items()})

# ===================================================================
# Parser field name -> force, unit (suffix + keyword matching)
# ===================================================================

SUFFIX_MAP = [
    ('_km_h','km/h'),('_kmh','km/h'),('_km_s','km/s'),('_m_s','m/s'),('_cm_s','cm/s'),
    ('_mph','mph'),('_knot','knot'),('_kts','knot'),
    ('_c','C'),('_f','F'),('_k','K'),
    ('_hpa','hPa'),('_pa','Pa'),('_mb','mb'),('_mbar','mbar'),('_atm','atm'),
    ('_nt','nT'),('_ut','uT'),('_t','T'),('_gauss','G'),
    ('_wm2','W/m2'),('_w_m2','W/m2'),('_sfu','sfu'),('_jy','Jy'),('_mjy','mJy'),
    ('_ppm','ppm'),('_ppb','ppb'),('_ppt','ppt'),
    ('_mg_m3','mg/m3'),('_ug_m3','μg/m3'),('_mgm3','mg/m3'),('_ugm3','μg/m3'),
    ('_mg_l','mg/L'),('_ug_l','μg/L'),('_mgl','mg/L'),
    ('_p_cm3','p/cm3'),('_cm3','p/cm3'),('_m_3','p/m3'),
    ('_m3_s','m3/s'),('_cfs','ft3/s'),
    ('_deg','deg'),('_degt','deg'),('_degrees','deg'),
    ('_sec','s'),('_seconds','s'),('_s','s'),('_min','min'),('_h','h'),('_d','d'),('_day','d'),
    ('_m','m'),('_meters','m'),('_km','km'),('_cm','cm'),('_mm','mm'),
    ('_ft','ft'),('_feet','ft'),('_nmi','nmi'),('_mi','mi'),
    ('_hz','Hz'),('_khz','kHz'),('_mhz','MHz'),('_ghz','GHz'),
    ('_m_s2','m/s2'),('_ms2','m/s2'),('_gal','gal'),('_mgal','mGal'),
    ('_v_m','V/m'),('_vm','V/m'),('_mv','mV'),('_uv','μV'),('_kv','kV'),
    ('_us_cm','μS/cm'),('_uscm','μS/cm'),('_s_m','S/m'),
    ('_db','dB'),('_dba','dBA'),('_dbz','dBZ'),
    ('_psu','PSU'),('_ntu','NTU'),('_fnu','FNU'),
    ('_uatm','μatm'),('_matm','matm'),
    ('_kg_m3','kg/m3'),('_g_m3','g/m3'),('_g_kg','g/kg'),
    ('_kgm3','kg/m3'),('_gm3','g/m3'),('_gkg','g/kg'),
    ('_pct','%'),('_percent','%'),
    ('_w','W'),('_kw','kW'),('_mw','MW'),('_gw','GW'),
    ('_kj','kJ'),('_mj','MJ'),('_gj','GJ'),
    ('_ev','eV'),('_kev','keV'),('_mev','MeV'),('_gev','GeV'),
    ('_ka','kA'),('_ma','mA'),('_a','A'),
    ('_kn','kN'),('_n','N'),
    ('_sv','Sv'),('_gy','Gy'),('_bq','Bq'),
    ('_mas','mas'),('_arcsec','arcsec'),('_pc','pc'),('_ly','ly'),('_au','AU'),
    ('_du','DU'),('_ha','ha'),
    ('_cps','counts/s'),('_erg_s','erg/s'),('_erg_cm2_s','erg/cm2/s'),
    ('_bft','Beaufort'),
    ('_scalar','scalar'),('_unitless','scalar'),
]

FORCE_BY_UNIT = {
    'K':'thermal','C':'thermal','F':'thermal','K·d':'thermal',
    'm/s':'advective','km/s':'advective','km/h':'advective','cm/s':'advective',
    'mm/s':'advective','mph':'advective','knot':'advective','Beaufort':'advective',
    'hPa':'advective','Pa':'advective','kPa':'advective','mb':'advective','mbar':'advective','atm':'advective',
    'nT':'em','uT':'em','T':'em','G':'em',
    'W/m2':'em','sfu':'em','Jy':'em','mJy':'em','W':'em','kW':'em','MW':'em','GW':'em',
    'counts/s':'em','erg/cm2/s':'em','erg/s':'em',
    'm':'acoustic','cm':'acoustic','mm':'acoustic','ft':'acoustic','mi':'acoustic',
    'm/s2':'gravity','gal':'gravity','mGal':'gravity',
    'ppm':'diffusion','ppb':'diffusion','ppt':'diffusion','mg/m3':'diffusion',
    'μg/m3':'diffusion','mg/L':'diffusion','μg/L':'diffusion',
    'PSU':'diffusion','NTU':'diffusion','FNU':'diffusion','μS/cm':'diffusion',
    'S/m':'diffusion','%':'diffusion','g/m3':'diffusion','kg/m3':'diffusion',
    'g/kg':'diffusion','μatm':'diffusion','matm':'diffusion','p/cm3':'diffusion',
    'm3/s':'advective','ft3/s':'advective',
    'V/m':'electric','V':'electric','mV':'electric','μV':'electric','kV':'electric',
    'Hz':'acoustic','kHz':'acoustic','MHz':'acoustic','GHz':'acoustic',
    'dB':'acoustic','dBA':'acoustic','dBZ':'acoustic',
    's':'acoustic','ms':'acoustic','min':'acoustic','h':'acoustic','d':'acoustic',
    'A':'electric','mA':'electric','kA':'electric',
    'J':'acoustic','kJ':'acoustic','MJ':'acoustic','GJ':'acoustic','J/kg':'advective',
    'eV':'acoustic','keV':'acoustic','MeV':'acoustic','GeV':'acoustic',
    'N':'electric','kN':'electric',
    'Sv':'em','Gy':'em','Bq':'em',
    'mas':'em','arcsec':'em','pc':'em','ly':'em','AU':'em',
    'DU':'diffusion','ha':'acoustic','scalar':'diffusion',
}

EXCLUDE_WORDS = set("""
    magnitud occurr count total num stationid id id code
    rank category status type flag quality version
    scale sig signific uncertainty error stddev rms gap dmin
    time date year month day hour min second epoch
    lat lon long latitude longitude coord geometry
    name addr city country region description comment note
    extent area scan image pixel picture scene tile
    population individual record sample observ
    index scalerank anomaly value kenn mag magnitude
    kp ssn kp_index kpindex ap ae aa a_index a_running
    uv_index uvi weather_code weathercode class
""".split())

def assign_parser_field(name):
    """Assign force+unit from parser field name (human-readable, unit-hinted)."""
    n = name.lower().replace('__','_')
    if not n or len(n) < 2: return None
    
    # Suffix matching (strongest signal)
    for sfx, u in SUFFIX_MAP:
        if n.endswith(sfx):
            f = FORCE_BY_UNIT.get(u)
            if f: return f, u
    
    nl = n.replace('_','')
    
    # Exclude non-physical keys
    for w in EXCLUDE_WORDS:
        if w in nl: return None
    
    # Keyword matching
    if 'dew' in nl and ('point' in nl or 'temp' in nl): return 'thermal','C'
    if 'bright' in nl and 'ti' in nl: return 'thermal','K'
    if 'frp' in nl: return 'em','MW'
    if 'inclination' in nl: return 'em','deg'
    if 'declination' in nl: return 'em','deg'
    if nl in ('wspd','windspeed','wind_speed'): return 'advective','m/s'
    if nl in ('wdir','winddir','wind_direction'): return 'advective','deg'
    if nl in ('gst','gust','windgust','wind_gust'): return 'advective','m/s'
    if nl == 'gs': return 'advective','m/s'
    if nl == 'track': return 'advective','deg'
    if nl in ('atmp','airtemp','wtmp','watertemp','sst','ssta','dewp','dewpt'): return 'thermal','C'
    if 'temp' in nl: return 'thermal','K'
    if 'wind_speed' in n or 'windspeed' in nl: return 'advective','m/s'
    if 'wind_dir' in n or 'winddir' in nl: return 'advective','deg'
    if 'wave_height' in n or 'wvht' in nl or 'swh' in nl: return 'acoustic','m'
    if 'wave_period' in n or 'dpd' in nl or 'apd' in nl: return 'acoustic','s'
    if 'wave_dir' in n or 'mwd' in nl or 'wwd' in nl: return 'acoustic','deg'
    if 'pressure' in nl or 'baro' in nl: return 'advective','hPa'
    if 'precip' in nl or 'rain' in nl or 'snow' in nl: return 'acoustic','m'
    if 'tide' in nl or 'water_level' in nl or 'sea_level' in nl: return 'acoustic','m'
    if 'depth' in nl: return 'seismic-body','km'
    if 'bathymetry' in nl: return 'seismic-body','m'
    if 'flux' in nl or 'irradiance' in nl or 'radiation' in nl or 'radiance' in nl: return 'em','W/m2'
    if 'magnetic' in nl and 'field' in n: return 'em','nT'
    if 'mag_field' in nl or 'magfield' in nl: return 'em','nT'
    if any(n.startswith(p) for p in ('bt','bx','by','bz')) and ('_nt' in n or '_t' in n): return 'em','nT'
    if 'rho_cos_phi' in nl or 'rho_sin_phi' in nl: return 'em','nT'
    if 'humidity' in nl or 'rh' in nl or 'moisture' in nl: return 'diffusion','%'
    if 'salinity' in nl: return 'diffusion','PSU'
    if 'conductivity' in nl: return 'electric','μS/cm'
    if 'visibility' in nl: return 'diffusion','nmi'
    if 'velocity' in nl: return 'advective','m/s'
    if 'current_speed' in n or 'currentspeed' in nl: return 'advective','m/s'
    if 'current_dir' in n or 'currentdir' in nl: return 'advective','deg'
    if 'height' in nl and 'wave' not in nl: return 'gravity','m'
    if 'altitude' in nl or 'elev' in nl: return 'gravity','m'
    if 'energy' in nl and 'detected' in nl: return 'acoustic','J'
    if 'duration' in nl and 'detected' in nl: return 'acoustic','s'
    if 'speed' in nl: return 'advective','m/s'
    if 'distance' in nl: return 'acoustic','m'
    if 'density' in nl: return 'diffusion','kg/m3'
    if 'mass' in nl: return 'diffusion','kg'
    if 'power' in nl: return 'em','W'
    if 'frequency' in nl: return 'acoustic','Hz'
    if 'period' in nl: return 'acoustic','s'
    if 'acceleration' in nl: return 'gravity','m/s2'
    if 'gravity' in nl: return 'gravity','m/s2'
    if 'voltage' in nl: return 'electric','V'
    if 'dissolved_oxygen' in nl: return 'diffusion','mg/L'
    if 'chlorophyll' in nl: return 'diffusion','μg/L'
    if 'turbidity' in nl: return 'diffusion','NTU'
    if 'co2' in nl or 'ch4' in nl or 'so2' in nl or 'no2' in nl or 'o3' in nl or 'nox' in nl: return 'diffusion','ppm'
    if nl == 'gs': return 'advective','m/s'
    if nl == 'track': return 'advective','deg'
    if nl == 'altim': return 'advective','hPa'
    if 'detected_duration' in nl: return 'acoustic','s'
    if 'pl_orbper' in nl: return 'gravity','d'
    if 'pl_eqt' in nl: return 'thermal','K'
    if 'pl_rade' in nl: return 'gravity','m'
    # ArcGIS / EMODnet compound names with unit in suffix after __
    if 'depth__m_' in n: return 'seismic-body','m'
    if 'sea_surface_temperature' in n: return 'thermal','C'
    if 'sea_water_speed' in n and 'cm_s' in n: return 'advective','cm/s'
    if 'flow_cfs' in nl: return 'advective','ft3/s'
    if 'pctsnowice' in nl or 'pctwater' in nl: return 'diffusion','%'
    if 'sumcarbon' in nl: return 'diffusion','kg'
    if 'so2_kilotons' in nl: return 'diffusion','kt'
    if 'nidheight' in nl or 'damheight' in nl: return 'gravity','m'
    if 'stormnum' in nl: return None
    return None

# ===================================================================
# Metadata from sources.φ
# ===================================================================

def parse_sources_meta(path):
    src = open(path).read()
    blocks = re.split(r'\n(?=url )', src)
    meta = {}
    declared = {}
    for b in blocks:
        b = b.strip()
        if not b: continue
        url = ttl = frame = fbody = body_name = mp = cp = None
        lk = ok = ak = rk = dk = pk = None
        pl = pol = poa = None
        from_pos = False
        dkeys = set()
        for li in b.split('\n'):
            li = li.strip()
            if not li: continue
            p = li.split()
            if not p: continue
            k = p[0]
            if k == 'url': url = p[1] if len(p) > 1 else None
            elif k == 'ttl': ttl = p[1] if len(p) > 1 else None
            elif k == 'at': frame = 'at'; fbody = p[1] if len(p) > 1 else 'sun'
            elif k == 'on':
                frame = 'on'
                if len(p) >= 4: fbody = p[1]; pl = p[2]; pol = p[3]; poa = p[4] if len(p) > 4 else None
            elif k == 'body': frame = 'body'; body_name = p[1] if len(p) > 1 else 'earth'
            elif k == 'pos':
                from_pos = True
                if len(p) >= 3: pl = p[1]; pol = p[2]
                if len(p) >= 4: poa = p[3]
            elif k == 'map': mp = p[1] if len(p) > 1 else '.'
            elif k == 'cmap': cp = p[1] if len(p) > 1 else '.'
            elif k == 'lat_key' and len(p) >= 2: lk = p[1]
            elif k == 'lon_key' and len(p) >= 2: ok = p[1]
            elif k == 'alt_key' and len(p) >= 2: ak = p[1]
            elif k == 'ra_key' and len(p) >= 2: rk = p[1]
            elif k == 'dec_key' and len(p) >= 2: dk = p[1]
            elif k == 'plx_key' and len(p) >= 2: pk = p[1]
            elif k in ('field','last','first','path','regex','last_row','last_obj','obj_last','last_line','deep') and len(p) >= 2:
                dkeys.add(p[-1])  # last token is the human-readable name
            elif k == 'field_in':
                dkeys.add(p[-1])  # last token is the human-readable name
        if url:
            meta[url] = (ttl, frame, fbody, body_name, mp, cp, lk, ok, ak, rk, dk, pk, pl, pol, poa, from_pos)
            declared[url] = dkeys
    return meta, declared

def is_key_name(s):
    return bool(s and any(c.isalpha() for c in str(s)))

# ===================================================================
# Main
# ===================================================================

def build(meta, declared, dump_path, output_path):
    url_fields = defaultdict(set)
    
    # 1. Parser dump fields
    if os.path.exists(dump_path):
        with open(dump_path) as f:
            for row in csv.reader(f):
                if len(row) >= 2:
                    url, name = row[0], row[1]
                    r = assign_parser_field(name)
                    if r: url_fields[url].add((r[0], r[1], name.lower()))
    
    # 2. Declared fields from sources.φ
    for url, dkeys in declared.items():
        merged = url_fields.get(url, set())
        for dk in dkeys:
            short = dk.split('.')[-1]
            kc = dk.replace('_','').lower()
            if kc in EXCLUDE_WORDS or len(kc) < 2: continue
            r = assign_parser_field(dk) or assign_parser_field(short)
            if r: merged.add((r[0], r[1], dk.lower()))
        if merged: url_fields[url] = merged
    
    # 2. Spec-based fields (from URL query parameters)
    for url, m in meta.items():
        domain = url.replace('https://','').replace('www.','').split('/')[0]
        merged = url_fields.get(url, set())
        
        # Open-Meteo: parse hourly/daily/current params
        if 'open-meteo' in url:
            for key in ['hourly', 'daily', 'current']:
                mq = re.search(fr'{key}=([^&]+)', url)
                if mq:
                    for p in mq.group(1).split(','):
                        p = p.strip()
                        if p in SPEC:
                            f, u = SPEC[p].split()
                            merged.add((f, u, p))
        
        # SWPC: match known field names in URL path
        if 'swpc' in domain:
            for field, fu in SPEC.items():
                if field in url and any(c.isalpha() for c in field):
                    f, u = fu.split()
                    merged.add((f, u, field))
        
        # NOAA Tides: product-based
        if 'tidesandcurrents' in domain:
            mq = re.search(r'product=([^&]+)', url)
            if mq:
                prod = mq.group(1)
                if prod in SPEC:
                    f, u = SPEC[prod].split()
                    merged.add((f, u, prod))
        
        if merged: url_fields[url] = merged
    
    # 3. Build canonical blocks
    results = []
    for url, flds in url_fields.items():
        if url not in meta: continue
        vals = meta[url]
        ttl, frame, fbody, body_name, mp, cp, lk, ok, ak, rk, dk, pk, pl, pol, poa, from_pos = vals
        if not ttl: ttl = '300'
        
        lines = [f"url {url}", f"ttl {ttl}"]
        if frame == 'at': lines.append(f"at {fbody or 'sun'}")
        elif frame == 'on' and pl and pol:
            a = f"on {fbody or 'earth'} {pl} {pol}"
            if poa and str(poa) not in ('{alt}','0','','0.0'): a += f" {poa}"
            lines.append(a)
        elif frame == 'body': lines.append(f"body {body_name or 'earth'}")
        
        if mp: lines.append(f"map {mp}")
        if cp: lines.append(f"cmap {cp}")
        if lk: lines.append(f"lat {lk} deg")
        if ok: lines.append(f"lon {ok} deg")
        if ak: lines.append(f"alt {ak} km")
        if rk: lines.append(f"ra {rk} deg")
        if dk: lines.append(f"dec {dk} deg")
        if pk: lines.append(f"plx {pk} mas")
        if from_pos and pl and pol and not (mp or cp):
            if is_key_name(pl): lines.append(f"lat {pl} deg")
            if is_key_name(pol): lines.append(f"lon {pol} deg")
            if poa and is_key_name(poa): lines.append(f"alt {poa} km")
        
        for force, unit, name in sorted(flds):
            lines.append(f"field {name} {force} {unit}")
        
        fc = sum(1 for l in lines if l.startswith('field '))
        if fc == 0: continue
        lines.append('')
        results.append('\n'.join(lines))
    
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, 'w') as f:
        f.write('\n\n'.join(results) + '\n')
    
    # Stats
    forces = {}
    for r in results:
        for l in r.split('\n'):
            if l.startswith('field '):
                p = l.split()
                if len(p) >= 4: forces[p[2]] = forces.get(p[2], 0) + 1
    du = sum(1 for r in results for l in r.split('\n') if l.startswith('url '))
    print(f"  Blocks: {du} | Fields: {sum(forces.values())}")
    print(f"  Forces: {dict(sorted(forces.items(), key=lambda x:-x[1]))}")
    return results


if __name__ == '__main__':
    meta, declared = parse_sources_meta(SOURCES_PATH)
    print(f"Parsed {len(meta)} blocks from {SOURCES_PATH}")
    print(f"Spec map entries: {len(SPEC)}")
    results = build(meta, declared, DUMP_PATH, OUTPUT_PATH)
