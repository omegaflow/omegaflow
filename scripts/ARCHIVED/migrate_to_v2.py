#!/usr/bin/env python3
"""sources.φ → sources_v2.φ. Trusts field declarations. Cache + key-name for units. No live fetch."""
import json, re, os, sys, time

CACHE_PATH = "phi/recovery/unit_cache.json"
SOURCES = "phi/sources.φ"
OUTPUT = "phi/recovery/migration/sources_v2.φ"

UNIT_MAP = {
    'degc':'C','°c':'C','degf':'F','°f':'F','degk':'K','kelvin':'K','celsius':'C','fahrenheit':'F',
    'm/s':'m/s','mps':'m/s','meters/sec':'m/s','km/h':'km/h','kmh':'km/h','knots':'knot','kt':'knot','mph':'mph',
    'hpa':'hPa','mb':'mb','mbar':'mbar','atm':'atm','mmhg':'mmHg','pa':'Pa','kpa':'kPa','mpa':'MPa',
    'nt':'nT','nanotesla':'nT','ut':'uT','microtesla':'uT','t':'T','tesla':'T','g':'G','gauss':'G',
    'w/m2':'W/m2','w/m²':'W/m2','w m-2':'W/m2','wm-2':'W/m2','sfu':'sfu','jy':'Jy','mjy':'mJy',
    'ppm':'ppm','ppb':'ppb','ppt':'ppt','mg/m3':'mg/m3','μg/m3':'μg/m3','ug/m3':'μg/m3',
    'mg/l':'mg/L','μg/l':'μg/L','μmol/l':'μmol/L','umol/l':'μmol/L',
    'p/cm3':'p/cm3','cm-3':'p/cm3','1/cm3':'p/cm3','p/m3':'p/m3','m-3':'p/m3',
    'm3/s':'m3/s','cfs':'ft3/s','ft3/s':'ft3/s','l/s':'L/s',
    'deg':'deg','degt':'deg','degrees':'deg','°':'deg','s':'s','sec':'s','seconds':'s',
    'ms':'ms','min':'min','h':'h','hr':'h','d':'d','day':'d',
    'm':'m','meter':'m','meters':'m','km':'km','kilometers':'km',
    'ft':'ft','feet':'ft','foot':'ft','cm':'cm','mm':'mm','μm':'μm','nm':'nm',
    'nmi':'nmi','nautical miles':'nmi','hz':'Hz','khz':'kHz','mhz':'MHz','ghz':'GHz',
    'm/s2':'m/s2','m s-2':'m/s2','gal':'gal','mgal':'mGal','μgal':'μGal',
    'v/m':'V/m','mv/m':'mV/m','kv/m':'kV/m','v':'V','mv':'mV','μv':'μV','kv':'kV',
    'μs/cm':'μS/cm','us/cm':'μS/cm','s/m':'S/m','ms/cm':'mS/cm',
    'db':'dB','dba':'dBA','dbz':'dBZ','psu':'PSU','ntu':'NTU','fnu':'FNU',
    'μatm':'μatm','uatm':'μatm','matm':'matm','du':'DU','dobson':'DU',
    'kg/m3':'kg/m3','g/m3':'g/m3','g/kg':'g/kg','kg/kg':'kg/kg','m3/m3':'m3/m3',
    '%':'%','pct':'%','percent':'%','w':'W','kw':'kW','mw':'MW','gw':'GW',
    'j':'J','kj':'kJ','mj':'MJ','ev':'eV','kev':'keV','mev':'MeV','gev':'GeV',
    'a':'A','ma':'mA','ka':'kA','kn':'kN','n':'N','sv':'Sv','gy':'Gy','bq':'Bq',
    'erg':'erg','erg/s':'erg/s','erg/cm2/s':'erg/cm2/s','counts/s':'counts/s','cps':'counts/s',
    'mas':'mas','arcsec':'arcsec','pc':'pc','ly':'ly','au':'AU','torr':'Torr','l':'L',
    'mi':'mi','mph':'mph','dbz':'dBZ','ha':'ha',
}

FORCE = {
    'temperature':'thermal','temp':'thermal','t':'thermal','sst':'thermal','tmax':'thermal',
    'tmin':'thermal','tavg':'thermal','tobs':'thermal','dew':'thermal','dewpt':'thermal',
    'dew_point':'thermal','wet_bulb':'thermal','heat_index':'thermal','apparent':'thermal',
    'wind_chill':'thermal','brightness':'thermal','bright_ti4':'thermal','bright_ti5':'thermal',
    'bright_t31':'thermal','water_temp':'thermal','air_temp':'thermal','soil_temp':'thermal',
    'wind_speed':'advective','wind_gust':'advective','wspd':'advective','gst':'advective',
    'wind_dir':'advective','wdir':'advective','wind_direction':'advective','gs':'advective',
    'gust':'advective','speed':'advective','velocity':'advective','current_speed':'advective',
    'discharge':'advective','streamflow':'advective','flow':'advective','mwd':'advective',
    'pressure':'advective','pres':'advective','baro':'advective','alt_baro':'advective',
    'baro_rate':'advective','slp':'advective','msl':'advective','ptdy':'advective','press':'advective',
    'wave_height':'acoustic','wvht':'acoustic','swh':'acoustic','wvh':'acoustic',
    'wave_period':'acoustic','dpd':'acoustic','apd':'acoustic','swp':'acoustic',
    'wwp':'acoustic','mwp':'acoustic','period':'acoustic',
    'precipitation':'acoustic','prcp':'acoustic','rain':'acoustic','snow':'acoustic',
    'snowd':'acoustic','snom':'acoustic','precip':'acoustic','pcp':'acoustic','hail':'acoustic',
    'water_level':'acoustic','tide':'acoustic','height':'acoustic',
    'stage':'acoustic','gage_height':'acoustic','sea_level':'acoustic',
    'magnetic':'em','mag_field':'em','bt':'em','bx':'em','by':'em','bz':'em',
    'b_total':'em','imf':'em','hp':'em','he':'em','dst':'em',
    'flux':'em','irradiance':'em','radiation':'em','radiance':'em','radio':'em','xray':'em',
    'gamma':'em','count_rate':'em','f10.7':'em','f107':'em','sfu':'em','solar_flux':'em',
    'frp':'em','fire_radiative':'em','shortwave':'em','longwave':'em','solar_radiation':'em',
    'ghli':'em','dni':'em','ghi':'em','lightning':'em','aod':'em',
    'acceleration':'gravity','gravity':'gravity','elevation':'gravity',
    'altitude':'gravity','geo_height':'gravity','geoid':'gravity',
    'co2':'diffusion','ch4':'diffusion','n2o':'diffusion','sf6':'diffusion',
    'co':'diffusion','so2':'diffusion','no2':'diffusion','o3':'diffusion','pm1':'diffusion',
    'pm25':'diffusion','pm10':'diffusion','particulate':'diffusion','aerosol':'diffusion',
    'dust':'diffusion','humidity':'diffusion','rh':'diffusion','relative_humidity':'diffusion',
    'vapor':'diffusion','moisture':'diffusion','salinity':'diffusion','conductivity':'diffusion',
    'turbidity':'diffusion','dissolved_oxygen':'diffusion','do':'diffusion','oxygen':'diffusion',
    'nitrate':'diffusion','phosphate':'diffusion','silicate':'diffusion','pco2':'diffusion',
    'ph':'diffusion','alkalinity':'diffusion','chlorophyll':'diffusion','visibility':'diffusion',
    'concentration':'diffusion','density':'diffusion','mixing_ratio':'diffusion','column':'diffusion',
    'depth':'seismic-body','magnitude':'seismic-body','mag':'seismic-body',
    'mmi':'seismic-surface','pga':'seismic-body','pgv':'seismic-body','seismic':'seismic-body',
    'voltage':'electric','potential':'electric','e_field':'electric','electric':'electric',
    'conductance':'electric','bioelectric':'electric','telluric':'electric',
}

SKIP = {
    'id','name','hex','flight','callsign','icao24','evid','publicid','net','auth','source',
    'station','stationid','stid','stn','wmo','wban','usaf','icao','coop','nwsli','platform',
    'sensor','satellite','spacecraft','target','target_name','designation','catalog','obsid',
    'version','release','object','otype','main_type','primary_id','alt_id','alias','title',
    'doi','url','link','reference','ref','country','state','county','fips','timezone',
    'location','place','locality','flynn_region','region','address','city','description',
    'detail','note','comment','magnitudetype','magtype','evtype','type','category','status',
    'quality','flag','flags','units','unit','parameter','param','instrument','squawk',
    'messages','seen','rssi','dmin','rms','gap','nst','n','count','total','records',
    'number','station_count','event_count','sample_size','multiplicity','individualcount',
    'population','occurrences','ssn','smoothed_ssn','kp','kp_index','estimated_kp',
    'a_running','a_index','aa','ap','ae','noaa_scale','storm_level','flare_index',
    'classtype','coded_type','weather_code','uv_index','uvi','scale','intensity','severity',
    'level','class','grade','rank','spectral_index','photon_index','pl_index','lp_index',
    'sigma','significance','detection_significance','confidence','standard_error','uncertainty',
    'time','time_tag','timestamp','datetime','date','generated','created','modified','updated',
    'lastupdate','begintime','peaktime','endtime','origintime','starttime','stoptime',
    'yy','mm','dd','hh','mn','ss','year','month','day','hour','minute','second','doy','week',
    'daynum','epoch','mjd','jd','acq_date','acq_time','local_date_time','soldate',
    'active','iscancel','isfinal','domestictsunami','issea','istraining','manual_mode','alert',
    'spi','confirmed','auto','automatic','reviewed','tisb','mlat','nic','rc','sil','gva','sda',
    'alt_geom','ias','tas','mach','mag_heading','true_heading','nav_qnh','nav_modes',
    'roll','track_rate','roll_rate','wd','ws','oat','tat',
    'npred','nexposure','frac_variability','variability_index',
    'plec_curve_significance','lp_curve_significance',
    'pl_flux_density_error','lp_flux_density_error','plec_flux_density_error',
    'pl_index_error','lp_index_error','plec_index_error','lp_epeak_error','plec_epeak_error',
    'flux_peak_error','energy_flux_error','semi_major_axis_68','semi_minor_axis_68',
    'position_angle_68','semi_major_axis_95','semi_minor_axis_95','position_angle_95',
    'max_convergence_flag','max_data_flag','max_error_count_flag','max_processing_flag',
    'max_range_flag','max_sample_count_flag','max_telemetry_flag','overall_quality',
    'pivot_energy','spectrum_type','data_release','luminosity','mass',
}

def is_skip(k):
    kl = k.lower().replace('_','').replace('-','').replace('.','').replace(' ','')
    return len(kl)<2 or kl in SKIP

def norm_unit(u):
    if not u: return None
    ul = u.lower().replace('°','deg').replace('µ','u')
    return UNIT_MAP.get(ul, ul)

def assign_force(k):
    kl = k.lower().replace('_',' ').replace('-',' ')
    for p, f in sorted(FORCE.items(), key=lambda x:-len(x[0])):
        if p in kl: return f
    return None

def force_by_unit(unit):
    if unit in ('K','C','F','degC','degF','degK'): return 'thermal'
    if unit in ('m/s','km/s','km/h','mph','knot'): return 'advective'
    if unit in ('hPa','Pa','kPa','MPa','mb','mbar','atm','bar','mmHg','psi'): return 'advective'
    if unit in ('W/m2','nT','uT','T','G','sfu','Jy','mJy','W','kW','MW','GW','counts/s','erg/cm2/s'): return 'em'
    if unit in ('m','cm','mm','ft','mi'): return 'acoustic'
    if unit in ('m/s2','gal','mGal','μGal'): return 'gravity'
    if unit in ('ppm','ppb','ppt','mg/m3','μg/m3','mg/L','PSU','NTU','μS/cm','%','g/m3','kg/m3','μmol/L','μatm','DU','g/kg'): return 'diffusion'
    if unit in ('m3/s','ft3/s','L/s'): return 'advective'
    if unit in ('V/m','V','mV','μV','kV','μS/cm','mS/cm','S/m'): return 'electric'
    if unit in ('Hz','kHz','MHz','GHz','dB','dBA','dBZ','s','ms'): return 'acoustic'
    if unit in ('A','mA','kA'): return 'electric'
    if unit in ('J','kJ','MJ','eV','keV','MeV','GeV','erg'): return 'acoustic'
    return 'em'

def infer_unit(key, cache_entry=None):
    """Infer unit from key name or cache header_units."""
    # First try cache header_units
    if isinstance(cache_entry, dict) and 'header_units' in cache_entry:
        for col, unit in cache_entry['header_units'].items():
            if col.upper() == key.upper():
                return norm_unit(str(unit))
    # Key-name patterns
    kl = key.lower().replace(' ','_')
    pats = [
        ('_km_h','km/h'),('_kmh','km/h'),('_km_s','km/s'),('_m_s','m/s'),('_ms','m/s'),
        ('_mph','mph'),('_knot','knot'),('_kts','knot'),('_kn','knot'),
        ('_c','C'),('_degc','C'),('_f','F'),('_degf','F'),('_k','K'),('_degk','K'),
        ('_hpa','hPa'),('_pa','Pa'),('_mb','mb'),('_mbar','mbar'),('_atm','atm'),
        ('_nt','nT'),('_ut','uT'),('_t','T'),('_gauss','G'),
        ('_wm2','W/m2'),('_w_m2','W/m2'),('_sfu','sfu'),('_jy','Jy'),('_mjy','mJy'),
        ('_ppm','ppm'),('_ppb','ppb'),('_ppt','ppt'),
        ('_mgm3','mg/m3'),('_ugm3','μg/m3'),('_mgl','mg/L'),
        ('_cm3','p/cm3'),('_m3','p/m3'),
        ('_m3_s','m3/s'),('_cfs','ft3/s'),
        ('_deg','deg'),
        ('_s','s'),('_min_','min'),('_h','h'),('_hr','h'),('_d','d'),
        ('_m','m'),('_km','km'),('_cm','cm'),('_mm','mm'),('_ft','ft'),('_nmi','nmi'),
        ('_hz','Hz'),('_khz','kHz'),('_mhz','MHz'),
        ('_ms2','m/s2'),('_gal','gal'),('_mgal','mGal'),
        ('_vm','V/m'),('_mvm','mV/m'),('_kvm','kV/m'),('_mv','mV'),('_uv','μV'),
        ('_uscm','μS/cm'),('_sm','S/m'),
        ('_db','dB'),('_dba','dBA'),('_dbz','dBZ'),
        ('_psu','PSU'),('_ntu','NTU'),
        ('_uatm','μatm'),('_matm','matm'),
        ('_kgm3','kg/m3'),('_gm3','g/m3'),('_gkg','g/kg'),('_m3m3','m3/m3'),
        ('_pct','%'),('_kw','kW'),('_mw','MW'),('_gw','GW'),
        ('_kj','kJ'),('_kev','keV'),('_mev','MeV'),('_gev','GeV'),
        ('_ka','kA'),('_kn','kN'),
        ('_sv','Sv'),('_gy','Gy'),
        ('_ergs','erg/s'),('_ergcm2s','erg/cm2/s'),
        ('_cps','counts/s'),
        ('_mas','mas'),('_arcsec','arcsec'),
        ('_du','DU'),('_ha','ha'),('_mi','mi'),
    ]
    for sfx, u in pats:
        if kl.endswith(sfx): return u
    # Try unit from cache value
    if isinstance(cache_entry, dict) and 'header_units' not in cache_entry:
        for k, v in cache_entry.items():
            if k.upper() == key.upper() and isinstance(v, str):
                return norm_unit(v)
    return None

def process(block_text):
    lines = block_text.strip().split('\n')
    url, ttl, frame, fbody, fmt = None, None, None, None, 'json'
    mp, cp, rp = None, None, None
    lk, ok, ak, rk, dk, pk = None, None, None, None, None, None
    pmk, pmdk, rvk, zk, dsk, epk, tauk, vk, trk, vrk, vak = None,None,None,None,None,None,None,None,None,None,None
    pl, pol, poa = None, None, None
    fields = set()

    for line in lines:
        line = line.strip()
        if not line or line[0]=='#': continue
        p = line.split()
        if not p: continue
        k = p[0]

        if k=='url': url = p[1] if len(p)>1 else None
        elif k=='ttl': ttl = p[1] if len(p)>1 else None
        elif k=='force': pass
        elif k=='at': frame='at'; fbody=p[1] if len(p)>1 else 'sun'
        elif k=='on':
            frame='on'
            if len(p)>=4: fbody=p[1]; pl=p[2]; pol=p[3]; poa=p[4] if len(p)>4 else None
        elif k=='body': frame='body'; fbody=p[1] if len(p)>1 else 'earth'
        elif k=='pos':
            if len(p)>=3: pl=p[1]; pol=p[2]
            if len(p)>=4: poa=p[3]
        elif k=='map': mp = p[1] if len(p)>1 else '.'
        elif k=='cmap': cp = p[1] if len(p)>1 else '.'
        elif k=='rows': rp = p[1] if len(p)>1 else 'rows'; fmt='csv'
        elif k=='format': fmt = p[1] if len(p)>1 else 'json'
        elif k=='lat_key' and len(p)>=2: lk=p[1]
        elif k=='lon_key' and len(p)>=2: ok=p[1]
        elif k=='alt_key' and len(p)>=2: ak=p[1]
        elif k=='ra_key' and len(p)>=2: rk=p[1]
        elif k=='dec_key' and len(p)>=2: dk=p[1]
        elif k=='plx_key' and len(p)>=2: pk=p[1]
        elif k=='pmra_key' and len(p)>=2: pmk=p[1]
        elif k=='pmdec_key' and len(p)>=2: pmdk=p[1]
        elif k=='radvel_key' and len(p)>=2: rvk=p[1]
        elif k=='z_key' and len(p)>=2: zk=p[1]
        elif k=='dist_key' and len(p)>=2: dsk=p[1]
        elif k=='epoch_key' and len(p)>=2: epk=p[1]
        elif k=='tau_key' and len(p)>=2: tauk=p[1]
        elif k=='vel_key' and len(p)>=2: vk=p[1]
        elif k=='trk_key' and len(p)>=2: trk=p[1]
        elif k=='vr_key' and len(p)>=2: vrk=p[1]
        elif k in ('field','last','first','path','regex','last_row','last_obj','obj_last','last_line') and len(p)>=2:
            fields.add(p[1])
        elif k=='field_in' and len(p)>=2:
            fields.add(p[1])
        elif k=='count' and len(p)>=2:
            pass  # counts dropped

    if not url or not ttl: return None

    # Match cache entry
    ce = CACHE.get(url)
    if not ce:
        base = url.split('?')[0]
        for ck in CACHE:
            if ck.split('?')[0] == base: ce = CACHE[ck]; break

    out = [f"url {url}", f"ttl {ttl}"]
    if frame=='at': out.append(f"at {fbody}")
    elif frame=='on' and pl and pol:
        a = f"on {fbody} {pl} {pol}"
        if poa and str(poa) not in ('{alt}','0',''): a += f" {poa}"
        out.append(a)
    elif frame=='body': out.append(f"body {fbody}")

    if mp: out.append(f"map {mp}")
    if cp: out.append(f"cmap {cp}")
    if lk: out.append(f"lat {lk} deg")
    if ok: out.append(f"lon {ok} deg")
    if ak: out.append(f"alt {ak} km")
    if rk: out.append(f"ra {rk} deg")
    if dk: out.append(f"dec {dk} deg")
    if pk: out.append(f"plx {pk} mas")
    if pmk: out.append(f"pmra {pmk} mas/yr")
    if pmdk: out.append(f"pmdec {pmdk} mas/yr")
    if rvk: out.append(f"radvel {rvk} km/s")
    if zk: out.append(f"z {zk}")
    if dsk: out.append(f"dist {dsk} pc")
    if epk: out.append(f"epoch {epk} s")
    if tauk: out.append(f"tau {tauk} s")
    if vk: out.append(f"vel {vk} m/s")
    if trk: out.append(f"trk {trk} deg")
    if vrk: out.append(f"vr {vrk} m/s")
    if pl and pol and not (mp or cp):
        out.append(f"lat {pl} deg"); out.append(f"lon {pol} deg")
        if poa: out.append(f"alt {poa} km")

    fc = 0; seen = set()
    for fk in sorted(fields):
        if is_skip(fk): continue
        clean = fk.split('.')[-1].lower().replace('_','')
        if is_skip(clean): continue

        f = assign_force(fk) or assign_force(clean)
        u = infer_unit(fk, ce) or infer_unit(clean, ce)
        if not u: continue
        if not f: f = force_by_unit(u)

        ok = fk.lower()
        if ok in seen: continue
        seen.add(ok)
        out.append(f"field {ok} {f} {u}")
        fc += 1

    if fc == 0: return None
    out.append(''); return '\n'.join(out)

# MAIN
if not os.path.exists(CACHE_PATH): sys.exit("No cache")
CACHE = json.load(open(CACHE_PATH))
text = open(SOURCES).read()
blocks = re.split(r'\n(?=url )', text)
print(f"Processing {len(blocks)} blocks from {SOURCES}...")
results = []; nf = 0; t0 = time.time()
for i,b in enumerate(blocks):
    b = b.strip()
    if not b: continue
    r = process(b)
    if r: results.append(r)
    else: nf += 1
    if (i+1)%500 == 0:
        e = time.time()-t0
        print(f"  {i+1}/{len(blocks)} kept={len(results)} dropped={nf} [{e:.0f}s]")
e = time.time()-t0
print(f"Done: {len(results)} blocks ({nf} no-measurement) [{e:.0f}s]")
os.makedirs(os.path.dirname(OUTPUT), exist_ok=True)
with open(OUTPUT,'w') as f: f.write('\n'.join(results)+'\n')
print(f"Output: {OUTPUT}")
