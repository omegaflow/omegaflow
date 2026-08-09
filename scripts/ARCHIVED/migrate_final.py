#!/usr/bin/env python3
"""Fetch all sources.φ URLs in parallel, save raw responses, then extract fields for canonical output."""
import subprocess, json, re, os, sys, time
from concurrent.futures import ThreadPoolExecutor, as_completed
from urllib.parse import quote

SRC = "phi/sources.φ"
OUT = "phi/recovery/migration/sources_v2.φ"
TASKS = "phi/recovery/migration/tasks_01.json"
SECRETS = {}
for line in open(".secrets.local"):
    line = line.strip()
    if line and '=' in line and not line.startswith('#'):
        k, v = line.split('=', 1)
        SECRETS[k.strip()] = v.strip().strip('"')

def sub(u):
    u = u.replace("{today}", "2026-08-07").replace("{yesterday}", "2026-08-06")
    u = u.replace("{hour_ago}", "2026-08-07T11:00:00").replace("{week_ago}", "2026-07-31")
    u = u.replace("{year}", "2026").replace("{month}", "08").replace("{day}", "07")
    u = u.replace("{lat}", "52.52").replace("{lon}", "13.41")
    u = u.replace("{ra}", "0.0").replace("{dec}", "0.0").replace("{bbox}", "-180,-90,180,90")
    for i in range(5,101,5):
        u = u.replace("{{lon_min_{}}}".format(i), "0").replace("{{lon_max_{}}}".format(i), str(i))
        u = u.replace("{{lat_min_{}}}".format(i), "0").replace("{{lat_max_{}}}".format(i), str(i))
    for m in re.findall(r'\{buf_\d+[km]+\}', u): u = u.replace(m, "0.00001")
    for k, v in SECRETS.items(): u = u.replace("{{%s}}" % k, v)
    return u

def curl(url):
    try:
        p = subprocess.run(
            ["curl", "-sS", "--connect-timeout", "5", "--max-time", "10", "-L",
             "-A", "omegaflow/2.0", "-H", "Accept: application/json,text/csv,text/plain", url],
            capture_output=True, timeout=15)
        if p.returncode != 0: return None
        return p.stdout.decode("utf-8", "replace")
    except: return None

def parse_ndbc_header(text):
    lines = text.strip().split('\n')
    if len(lines) < 2: return None
    h1 = lines[0].lstrip('#').strip().split()
    h2 = lines[1].lstrip('#').strip().split()
    if len(h1) != len(h2): return None
    return dict(zip(h1, h2))

U = {'degc':'C','degf':'F','degk':'K','m/s':'m/s','km/h':'km/h','knots':'knot',
    'kt':'knot','mph':'mph','hpa':'hPa','mb':'mb','pa':'Pa','nt':'nT','ut':'uT',
    't':'T','g':'G','w/m2':'W/m2','sfu':'sfu','jy':'Jy','ppm':'ppm','ppb':'ppb',
    'mg/m3':'mg/m3','ug/m3':'μg/m3','mg/l':'mg/L','cm3':'p/cm3','m-3':'p/m3',
    'm3/s':'m3/s','deg':'deg','degt':'deg','s':'s','sec':'s','m':'m','km':'km',
    'ft':'ft','cm':'cm','mm':'mm','nmi':'nmi','hz':'Hz','khz':'kHz','mhz':'MHz',
    'm/s2':'m/s2','gal':'gal','mgal':'mGal','v/m':'V/m','v':'V','mv':'mV',
    'uv':'μV','us/cm':'μS/cm','db':'dB','psu':'PSU','ntu':'NTU','uatm':'μatm',
    '%':'%','w':'W','kw':'kW','mw':'MW','kj':'kJ','ev':'eV','kev':'keV',
    'mev':'MeV','a':'A','ma':'mA','ka':'kA','mas':'mas','du':'DU','erg/s':'erg/s',
    'counts/s':'counts/s','g/m3':'g/m3','g/kg':'g/kg','kg/m3':'kg/m3','m3/m3':'m3/m3',
}
FO = {'K':'thermal','C':'thermal','F':'thermal','m/s':'advective','km/s':'advective',
    'km/h':'advective','mph':'advective','knot':'advective','hPa':'advective',
    'Pa':'advective','kPa':'advective','mb':'advective','atm':'advective',
    'nT':'em','uT':'em','T':'em','G':'em','W/m2':'em','sfu':'em','Jy':'em',
    'mJy':'em','W':'em','kW':'em','MW':'em','counts/s':'em','erg/cm2/s':'em',
    'm':'acoustic','cm':'acoustic','mm':'acoustic','ft':'acoustic',
    'm/s2':'gravity','gal':'gravity','mGal':'gravity','ppm':'diffusion',
    'ppb':'diffusion','ppt':'diffusion','mg/m3':'diffusion','μg/m3':'diffusion',
    'mg/L':'diffusion','PSU':'diffusion','NTU':'diffusion','μS/cm':'diffusion',
    'μatm':'diffusion','%':'diffusion','g/m3':'diffusion','kg/m3':'diffusion',
    'g/kg':'diffusion','μmol/L':'diffusion','DU':'diffusion','m3/m3':'diffusion',
    'm3/s':'advective','ft3/s':'advective','V/m':'electric','V':'electric',
    'mV':'electric','μV':'electric','Hz':'acoustic','kHz':'acoustic',
    'MHz':'acoustic','dB':'acoustic','dBA':'acoustic','s':'acoustic',
    'ms':'acoustic','A':'electric','mA':'electric','kA':'electric',
    'J':'acoustic','kJ':'acoustic','eV':'acoustic','keV':'acoustic',
    'MeV':'acoustic','erg':'acoustic',
}

SK = set("id name hex flight callsign icao24 evid publicid net auth source station series stationid stn stid wmo wban usaf icao nwsli platform sensor satellite spacecraft target_name designation catalog obsid dataset dr version object otype main_type alias title doi url link reference ref country state county fips timezone location place locality flynn_region region city description detail note comment magnitudetype magtype evtype category status quality flag flags units unit instrument squawk messages seen rssi dmin rms gap nst sil gva sda nic rc nac_p nac_v records number station_count event_count sample_size multiplicity individualcount population occurrences npred nexposure ssn smoothed_ssn kp kp_index estimated_kp a_running a_index aa ap ae noaa_scale storm_level flare_index classtype coded_type weather_code uv_index uvi scale intensity severity level class grade rank spectral_index photon_index pl_index lp_index sigma significance detection_significance confidence standard_error uncertainty frac_variability variability_index time time_tag timestamp datetime date generated created modified updated lastupdate begintime peaktime endtime origintime starttime stoptime yy mm dd hh mn ss year month day hour minute second doy week daynum epoch mjd jd acq_date acq_time local_date_time soldate active iscancel isfinal issea istraining manual_mode alert spi confirmed auto automatic reviewed tisb mlat domestictsunami alt_geom ias tas mach mag_heading true_heading nav_qnh nav_modes roll track_rate semi_major_axis_68 semi_minor_axis_68 position_angle_68 semi_major_axis_95 semi_minor_axis_95 position_angle_95 pivot_energy spectrum_type overall_quality max_convergence_flag max_data_flag max_error_count_flag max_processing_flag max_range_flag max_sample_count_flag max_telemetry_flag data_release pid release solution_id random_index pl_flux_density_error lp_flux_density_error plec_flux_density_error pl_index_error lp_index_error plec_index_error flux_peak_error energy_flux_error lp_epeak_error plec_epeak_error frac_variability_error flux_peak lp_epeak plec_epeak wd ws oat tat total sceneid gcmdactivity instruments linkedevents submissiontime tz felt cdi mmi ids sources types title".split())

def skip(k):
    return k.replace('_','').replace('-','').replace('.','').lower() in SK

def nu(u):
    return U.get(u.replace('°','deg').replace('µ','u').lower(), u.lower()) if u else None

def process_task(task):
    url = task['url']
    ttl = task['ttl']
    frame = task.get('frame', '')
    body = task.get('body', '')
    mp = task.get('map', '')
    cp = task.get('cmap', '')
    lk = task.get('lat_key', '')
    ok = task.get('lon_key', '')
    ak = task.get('alt_key', '')
    rk = task.get('ra_key', '')
    dk = task.get('dec_key', '')
    pk = task.get('plx_key', '')
    pos = task.get('pos', {})
    
    ru = sub(url)
    text = curl(ru)
    fields = {}
    
    if text:
        hdr = parse_ndbc_header(text)
        if hdr:
            for col, unit in hdr.items():
                if not skip(col):
                    u = nu(unit)
                    if u and u in FO: fields[col.upper()] = (FO[u], u)
        else:
            try: data = json.loads(text)
            except: data = None
            if data:
                def walk(obj, prefix=''):
                    if isinstance(obj, dict):
                        for k, v in obj.items():
                            if skip(k): continue
                            if isinstance(v, (int, float)) and not isinstance(v, bool):
                                kk = k.lower()
                                for sfx, un in [('_km_h','km/h'),('_kmh','km/h'),('_km_s','km/s'),('_m_s','m/s'),
                                    ('_mph','mph'),('_knot','knot'),('_kn','knot'),('_c','C'),('_f','F'),('_k','K'),
                                    ('_hpa','hPa'),('_pa','Pa'),('_mb','mb'),('_nt','nT'),('_ut','uT'),('_t','T'),
                                    ('_wm2','W/m2'),('_sfu','sfu'),('_jy','Jy'),('_mjy','mJy'),
                                    ('_ppm','ppm'),('_ppb','ppb'),('_mgm3','mg/m3'),('_ugm3','μg/m3'),
                                    ('_mgl','mg/L'),('_cm3','p/cm3'),('_m3','p/m3'),('_m3_s','m3/s'),
                                    ('_deg','deg'),('_s','s'),('_m','m'),('_km','km'),('_cm','cm'),('_mm','mm'),
                                    ('_ft','ft'),('_hz','Hz'),('_khz','kHz'),('_ms2','m/s2'),('_gal','gal'),
                                    ('_mgal','mGal'),('_vm','V/m'),('_mv','mV'),('_uv','μV'),('_uscm','μS/cm'),
                                    ('_db','dB'),('_psu','PSU'),('_ntu','NTU'),('_uatm','μatm'),('_pct','%'),
                                    ('_kw','kW'),('_mw','MW'),('_kj','kJ'),('_kev','keV'),('_mev','MeV'),
                                    ('_mas','mas'),('_du','DU'),('_ergs','erg/s'),('_cps','counts/s'),
                                    ('_gm3','g/m3'),('_gkg','g/kg'),('_kgm3','kg/m3'),('_m3m3','m3/m3')]:
                                    if kk.endswith(sfx):
                                        fields[k] = (FO.get(un, 'em'), un)
                                        break
                            elif isinstance(v, dict) and 'value' in v:
                                pass
                            elif isinstance(v, list) and v and isinstance(v[0], dict) and prefix=='':
                                walk(v[0], k)
                    elif isinstance(obj, list) and obj and isinstance(obj[0], dict):
                        walk(obj[0], prefix)
                walk(data)
    
    if not fields: return None
    
    out = ["url {}".format(url), "ttl {}".format(ttl)]
    if frame: out.append(frame)
    if body and not frame: out.append("body {}".format(body))
    if mp: out.append("map {}".format(mp))
    if cp: out.append("cmap {}".format(cp))
    if lk: out.append("lat {} deg".format(lk))
    if ok: out.append("lon {} deg".format(ok))
    if ak: out.append("alt {} km".format(ak))
    if rk: out.append("ra {} deg".format(rk))
    if dk: out.append("dec {} deg".format(dk))
    if pk: out.append("plx {} mas".format(pk))
    if pos and not (mp or cp):
        if pos.get('lat'): out.append("lat {} deg".format(pos['lat']))
        if pos.get('lon'): out.append("lon {} deg".format(pos['lon']))
        if pos.get('alt'): out.append("alt {} km".format(pos['alt']))
    
    for k in sorted(fields):
        f, u = fields[k]
        out.append("field {} {} {}".format(k.lower(), f, u))
    out.append("")
    return '\n'.join(out)

# Load all tasks
all_tasks = []
for i in range(1, 13):
    path = "phi/recovery/migration/tasks_{:02d}.json".format(i)
    if os.path.exists(path):
        for line in open(path):
            line = line.strip()
            if line:
                all_tasks.append(json.loads(line))

print("Processing {} blocks...".format(len(all_tasks)))
results = []
t0 = time.time()
for i, task in enumerate(all_tasks):
    r = process_task(task)
    if r:
        results.append(r)
    if (i+1) % 100 == 0:
        e = time.time() - t0
        print("  {}/{} kept={} [{:.0f}s]".format(i+1, len(all_tasks), len(results), e))

e = time.time() - t0
print("Done: {} blocks ({:.0f}s)".format(len(results), e))
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'w') as f:
    f.write('\n'.join(results) + '\n')
print("Output: {}".format(OUT))
