#!/usr/bin/env python3
"""Final parallel migration — all 1924 blocks. Cache-first, live-fallback with parallel curl."""
import subprocess, json, re, os, sys, time
from concurrent.futures import ThreadPoolExecutor, as_completed

SEC = {}
for line in open('.secrets.local'):
    line = line.strip()
    if line and '=' in line and not line.startswith('#'):
        k, v = line.split('=', 1)
        SEC[k.strip()] = v.strip()
CACHE = json.load(open('phi/recovery/unit_cache.json'))
PROXIES = [p.strip() for p in SEC.get('PROXY', '').split(',') if p.strip()]

def sub(u):
    rep = {'{today}':'2026-08-07','{yesterday}':'2026-08-06','{tomorrow}':'2026-08-08',
        '{now}':'2026-08-07T12:00:00','{year}':'2026','{week_ago}':'2026-07-31',
        '{hour_ago}':'2026-08-07T11:00:00','{month}':'08','{day}':'07',
        '{hour}':'12','{minute}':'00','{date}':'2026-08-07',
        '{lat}':'52.52','{lon}':'13.41','{ra}':'0.0','{dec}':'0.0',
        '{target}':'Ceres','{scale}':'1.0','{key}':'k','{page}':'1','{n}':'10','{sensor}':'s','{body}':'ISS',
        '{bbox}':'-180,-90,180,90','{id}':'0',
        '{lat_min}':'-80','{lat_max}':'80','{lon_min}':'-180','{lon_max}':'180',
        '{radius}':'500','{grid}':'100','{catalog}':'gaia','{net}':'IU',
        '{nearest_station}':'8518750','{usgs_site}':'09380000',
        '{cds_token}':SEC.get('CDS_API_KEY',''),
        '{earthdata}':SEC.get('EARTHDATA_EDL_TOKEN',''),
        '{nasa_key}':SEC.get('NASA_API_KEY','DEMO_KEY')}
    for k, v in rep.items(): u = u.replace(k, v)
    return u

def curl(url, headers, timeout=12, proxy=None):
    cmd = ['curl','-sS','--max-time',str(timeout),'-L','-A','omegaflow/2.0','-w','\n%{http_code}']
    if proxy: cmd += ['-x', proxy]
    for k, v in headers.items(): cmd += ['-H', f'{k}: {v}']
    cmd += [url]
    try:
        p = subprocess.run(cmd, capture_output=True, timeout=timeout+5)
        if p.returncode != 0: return None
        out = p.stdout.decode('utf-8','replace')
        parts = out.rsplit('\n', 1)
        body = parts[0] if len(parts)==2 else out
        code = parts[1].strip() if len(parts)==2 else '200'
        if int(code) in (200, 206): return body
        return None
    except: return None

def fetch(url, hdrs=None):
    if hdrs is None: hdrs = {'Accept':'application/json,text/csv,text/plain'}
    for tag, px, prefix in [('direct',None,None)] + [(f'p{i}',p,None) for i,p in enumerate(PROXIES)] + [('jina',None,'https://r.jina.ai/')]:
        to = 12 if tag != 'jina' else 30
        for attempt in range(3):
            u = (prefix+url) if prefix else url
            h = dict(hdrs)
            if tag == 'jina': h['Accept']='application/json'; h['X-Return-Format']='application/json'
            text = curl(u, h, timeout=to, proxy=px)
            if text: return text
            if attempt < 2: time.sleep(1)
    return None

# Fetch many URLs in parallel
def fetch_many(urls, max_workers=20):
    results = {}
    with ThreadPoolExecutor(max_workers=max_workers) as ex:
        fut = {ex.submit(fetch, u): u for u in urls}
        for f in as_completed(fut):
            results[fut[f]] = f.result()
    return results

# -- Units & Forces --
U = {
    'degc':'C','degf':'F','degk':'K','kelvin':'K','m/s':'m/s','km/h':'km/h',
    'knots':'knot','kt':'knot','mph':'mph','hpa':'hPa','mb':'mb','pa':'Pa','kpa':'kPa',
    'nt':'nT','ut':'uT','t':'T','g':'G','gauss':'G','w/m2':'W/m2','sfu':'sfu',
    'jy':'Jy','mjy':'mJy','ppm':'ppm','ppb':'ppb','ppt':'ppt',
    'mg/m3':'mg/m3','ug/m3':'μg/m3','mg/l':'mg/L','umol/l':'μmol/L',
    'p/cm3':'p/cm3','m-3':'p/m3','m3/s':'m3/s','cfs':'ft3/s',
    'deg':'deg','degt':'deg','s':'s','sec':'s','m':'m','km':'km','ft':'ft',
    'cm':'cm','mm':'mm','nmi':'nmi','hz':'Hz','khz':'kHz','mhz':'MHz',
    'm/s2':'m/s2','gal':'gal','mgal':'mGal','v/m':'V/m','v':'V','mv':'mV','uv':'μV',
    'us/cm':'μS/cm','db':'dB','psu':'PSU','ntu':'NTU','uatm':'μatm',
    'kg/m3':'kg/m3','g/m3':'g/m3','g/kg':'g/kg','m3/m3':'m3/m3',
    '%':'%','w':'W','kw':'kW','mw':'MW','j':'J','kj':'kJ',
    'ev':'eV','kev':'keV','mev':'MeV','a':'A','ma':'mA','ka':'kA',
    'mas':'mas','arcsec':'arcsec','du':'DU','erg':'erg','erg/s':'erg/s',
    'erg/cm2/s':'erg/cm2/s','counts/s':'counts/s',
}
FO = {
    'K':'thermal','C':'thermal','F':'thermal',
    'm/s':'advective','km/s':'advective','km/h':'advective','mph':'advective','knot':'advective',
    'hPa':'advective','Pa':'advective','kPa':'advective','mb':'advective','atm':'advective',
    'nT':'em','uT':'em','T':'em','G':'em','W/m2':'em','sfu':'em','Jy':'em','mJy':'em',
    'W':'em','kW':'em','MW':'em','counts/s':'em','erg/cm2/s':'em',
    'm':'acoustic','cm':'acoustic','mm':'acoustic','ft':'acoustic',
    'm/s2':'gravity','gal':'gravity','mGal':'gravity',
    'ppm':'diffusion','ppb':'diffusion','ppt':'diffusion','mg/m3':'diffusion',
    'ug/m3':'diffusion','mg/L':'diffusion','PSU':'diffusion','NTU':'diffusion',
    'μS/cm':'diffusion','μatm':'diffusion','%':'diffusion','g/m3':'diffusion',
    'kg/m3':'diffusion','g/kg':'diffusion','μmol/L':'diffusion','DU':'diffusion',
    'm3/s':'advective','ft3/s':'advective',
    'V/m':'electric','V':'electric','mV':'electric','μV':'electric',
    'Hz':'acoustic','kHz':'acoustic','MHz':'acoustic','dB':'acoustic','dBA':'acoustic',
    's':'acoustic','ms':'acoustic','A':'electric','mA':'electric','kA':'electric',
    'J':'acoustic','kJ':'acoustic','eV':'acoustic','keV':'acoustic','MeV':'acoustic',
    'erg':'acoustic','m3/m3':'diffusion',
}
SKIP = set("id name hex flight callsign icao24 evid publicid net auth source station stationid stn stid wmo wban usaf icao coo nwsli platform sensor satellite spacecraft target_name designation catalog obsid dataset dr version object otype main_type alias title doi url link reference ref country state county fips timezone location place locality flynn_region region city description detail note comment magnitudetype magtype evtype category status quality flag flags units unit instrument squawk messages seen rssi dmin rms gap nst sil gva sda nic rc nac_p nac_v records number station_count event_count sample_size multiplicity individualcount population occurrences npred nexposure ssn smoothed_ssn kp kp_index estimated_kp a_running a_index aa ap ae noaa_scale storm_level flare_index classtype coded_type weather_code uv_index uvi scale intensity severity level class grade rank spectral_index photon_index pl_index lp_index sigma significance detection_significance confidence standard_error uncertainty frac_variability variability_index time time_tag timestamp datetime date generated created modified updated lastupdate begintime peaktime endtime origintime starttime stoptime yy mm dd hh mn ss year month day hour minute second doy week daynum epoch mjd jd acq_date acq_time local_date_time soldate active iscancel isfinal issea istraining manual_mode alert spi confirmed auto automatic reviewed tisb mlat domestictsunami alt_geom ias tas mach mag_heading true_heading nav_qnh nav_modes roll track_rate semi_major_axis_68 semi_minor_axis_68 position_angle_68 semi_major_axis_95 semi_minor_axis_95 position_angle_95 pivot_energy spectrum_type overall_quality max_convergence_flag max_data_flag max_error_count_flag max_processing_flag max_range_flag max_sample_count_flag max_telemetry_flag data_release pid release solution_id random_index pl_flux_density_error lp_flux_density_error plec_flux_density_error pl_index_error lp_index_error plec_index_error flux_peak_error energy_flux_error lp_epeak_error plec_epeak_error frac_variability_error flux_peak lp_epeak plec_epeak wd ws oat tat total sceneid gcmdactivity instruments linkedevents submissiontime tz felt cdi mmi ids sources types title".split())

def skip(k): return k.replace('_','').replace('-','').replace('.','').lower() in SKIP

def nu(u):
    if not u: return None
    return U.get(u.replace('°','deg').replace('µ','u').lower(), u.lower())

def fu(key):
    k = key.lower()
    for sfx, u in [
        ('_km_h','km/h'),('_kmh','km/h'),('_km_s','km/s'),('_m_s','m/s'),
        ('_mph','mph'),('_knot','knot'),('_kn','knot'),('_kts','knot'),
        ('_c','C'),('_degc','C'),('_f','F'),('_k','K'),
        ('_hpa','hPa'),('_pa','Pa'),('_mb','mb'),('_mbar','mbar'),('_atm','atm'),
        ('_nt','nT'),('_ut','uT'),('_t','T'),('_gauss','G'),
        ('_wm2','W/m2'),('_w_m2','W/m2'),('_sfu','sfu'),('_jy','Jy'),('_mjy','mJy'),
        ('_ppm','ppm'),('_ppb','ppb'),('_ppt','ppt'),('_mgm3','mg/m3'),('_ugm3','μg/m3'),
        ('_mgl','mg/L'),('_cm3','p/cm3'),('_m3','p/m3'),('_m3_s','m3/s'),('_cfs','ft3/s'),
        ('_deg','deg'),('_s','s'),('_min','min'),('_h','h'),('_d','d'),
        ('_m','m'),('_km','km'),('_cm','cm'),('_mm','mm'),('_ft','ft'),('_nmi','nmi'),
        ('_hz','Hz'),('_khz','kHz'),('_mhz','MHz'),
        ('_ms2','m/s2'),('_gal','gal'),('_mgal','mGal'),
        ('_vm','V/m'),('_mvm','mV/m'),('_kvm','kV/m'),('_mv','mV'),('_uv','μV'),
        ('_uscm','μS/cm'),('_sm','S/m'),('_db','dB'),('_dba','dBA'),('_dbz','dBZ'),
        ('_psu','PSU'),('_ntu','NTU'),('_uatm','μatm'),('_matm','matm'),
        ('_kgm3','kg/m3'),('_gm3','g/m3'),('_gkg','g/kg'),('_m3m3','m3/m3'),
        ('_pct','%'),('_kw','kW'),('_mw','MW'),('_kj','kJ'),
        ('_kev','keV'),('_mev','MeV'),('_gev','GeV'),('_ka','kA'),('_kn','kN'),
        ('_ergs','erg/s'),('_ergcm2s','erg/cm2/s'),('_cps','counts/s'),
        ('_mas','mas'),('_arcsec','arcsec'),('_du','DU'),
    ]:
        if k.endswith(sfx): return FO.get(u, 'em'), u
    return None, None

KF = {
    'rtsw_wind': dict(proton_speed='advective km/s',proton_density='diffusion p/cm3',proton_temperature='thermal K'),
    'rtsw_mag': dict(bt='em nT',bx_gse='em nT',by_gse='em nT',bz_gse='em nT',bx_gsm='em nT',by_gsm='em nT',bz_gsm='em nT'),
    'xrays': dict(flux='em W/m2'),
    'integral': dict(flux='em W/m2'),
    'differential': dict(flux='em W/m2'),
    'magnetometers': dict(hp='em nT',he='em nT',total='em nT'),
    'solar-radio': dict(flux='em sfu'),
    'solar-cycle': dict(f10='em sfu'),
}

def cache_fields(url, ru):
    r = {}
    ce = CACHE.get(ru) or CACHE.get(url)
    if not ce:
        b = url.split('?')[0]
        for ck in CACHE:
            if ck.split('?')[0] == b: ce = CACHE[ck]; break
    if not isinstance(ce, dict): return r

    if 'header_units' in ce:
        for col, unit in ce['header_units'].items():
            if skip(col): continue
            u = nu(str(unit))
            if u and u in FO: r[col.upper()] = (FO[u], u)
        return r

    if 'error' not in str(ce).lower():
        for k, v in ce.items():
            if skip(k): continue
            if isinstance(v, dict) and 'value' in v:
                f, u = fu(k)
                if f: r[k] = (f, u)
            elif isinstance(v, (int, float)) and not isinstance(v, bool):
                f, u = fu(k)
                if f: r[k] = (f, u)
        if ce.get('type') == 'FeatureCollection' or 'features' in ce:
            r['depth'] = ('seismic-body', 'km')
        if r: return r

    if 'error' in str(ce).lower():
        for pat, kf in KF.items():
            if pat in ru:
                for k, fv in kf.items():
                    f, v = fv.split()
                    r[k] = (f, v)
                return r
        if 'earthquake' in ru or 'seismic' in ru or 'geonet' in ru or 'ingv' in ru:
            r['depth'] = ('seismic-body', 'km')
            return r
    return r

def live_fields(text):
    r = {}
    if not text: return r
    ls = text.strip().split('\n')
    if len(ls) >= 2 and ls[0].startswith('#'):
        h1 = ls[0].lstrip('#').strip().split()
        h2 = ls[1].lstrip('#').strip().split()
        if len(h1) == len(h2):
            for c, u_ in zip(h1, h2):
                if skip(c): continue
                u = nu(str(u_))
                if u and u in FO: r[c.upper()] = (FO[u], u)
            return r
    try: data = json.loads(text)
    except: return r

    def walk(obj, pre=''):
        if isinstance(obj, dict):
            for k, v in obj.items():
                if skip(k): continue
                if isinstance(v, (int, float)) and not isinstance(v, bool):
                    f, u = fu(k)
                    if f: r[k] = (f, u)
                elif isinstance(v, dict) and 'value' in v:
                    f, u = fu(k)
                    if f: r[k] = (f, u)
                elif isinstance(v, list) and v and isinstance(v[0], dict) and pre=='':
                    walk(v[0], k)
        elif isinstance(obj, list) and obj and isinstance(obj[0], dict):
            walk(obj[0], pre)
    walk(data)
    return r


print("Phase 1: Parsing sources.φ and collecting cache fields...")
SRC = open('phi/sources.φ').read()
blocks_raw = [b.strip() for b in re.split(r'\n(?=url )', SRC) if b.strip()]

# Parse all blocks
blocks = []
live_urls = []
for i, b in enumerate(blocks_raw):
    ls = b.split('\n')
    url = ttl = frame = fbody = mp = cp = None
    lk = ok = ak = rk = dk = pk = None
    pl = pol = poa = None
    for li in ls:
        li = li.strip()
        if not li or li[0]=='#': continue
        p = li.split()
        if not p: continue
        k = p[0]
        if k=='url': url = p[1] if len(p)>1 else None
        elif k=='ttl': ttl = p[1] if len(p)>1 else None
        elif k in ('force','format','method','header','post_body','extent','reach_ttl',
                   'target','catalog','catalog_epoch','max_freq','min_freq',
                   'repeat','stations','stations_path','stations_lat','stations_lon',
                   'stations_id','flux_from_mag','abs_mag_from','tail'): pass
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
        elif k=='lat_key' and len(p)>=2: lk=p[1]
        elif k=='lon_key' and len(p)>=2: ok=p[1]
        elif k=='alt_key' and len(p)>=2: ak=p[1]
        elif k=='ra_key' and len(p)>=2: rk=p[1]
        elif k=='dec_key' and len(p)>=2: dk=p[1]
        elif k=='plx_key' and len(p)>=2: pk=p[1]
    if url and ttl:
        ru = sub(url)
        fds = cache_fields(url, ru)
        info = dict(url=url, ttl=ttl, frame=frame, fbody=fbody, mp=mp, cp=cp,
                    lk=lk, ok=ok, ak=ak, rk=rk, dk=dk, pk=pk,
                    pl=pl, pol=pol, poa=poa, fields=fds, ru=ru)
        if not fds: live_urls.append(ru)
        blocks.append(info)

print(f"  Parsed {len(blocks)} blocks, {len([b for b in blocks if b['fields']])} have cache fields, {len(live_urls)} need live fetch")

# Phase 2: Parallel live fetch
if live_urls:
    print(f"Phase 2: Fetching {len(live_urls)} URLs in parallel (20 workers)...")
    t0 = time.time()
    results_map = fetch_many(live_urls, max_workers=20)
    print(f"  Fetched {len(results_map)}/{len(live_urls)} in {time.time()-t0:.0f}s")

    for b in blocks:
        if not b['fields'] and b['ru'] in results_map:
            b['fields'] = live_fields(results_map[b['ru']])

# Phase 3: Write output
print(f"Phase 3: Writing canonical output...")
results = []
nf = 0
for b in blocks:
    if not b['fields']:
        nf += 1; continue
    out = [f"url {b['url']}", f"ttl {b['ttl']}"]
    if b['frame'] == 'at': out.append(f"at {b['fbody']}")
    elif b['frame'] == 'on' and b['pl'] and b['pol']:
        a = f"on {b['fbody']} {b['pl']} {b['pol']}"
        if b['poa'] and str(b['poa']) not in ('{alt}','0',''): a += f" {b['poa']}"
        out.append(a)
    elif b['frame'] == 'body': out.append(f"body {b['fbody']}")

    if b['mp']: out.append(f"map {b['mp']}")
    if b['cp']: out.append(f"cmap {b['cp']}")
    if b['lk']: out.append(f"lat {b['lk']} deg")
    if b['ok']: out.append(f"lon {b['ok']} deg")
    if b['ak']: out.append(f"alt {b['ak']} km")
    if b['rk']: out.append(f"ra {b['rk']} deg")
    if b['dk']: out.append(f"dec {b['dk']} deg")
    if b['pk']: out.append(f"plx {b['pk']} mas")
    if b['pl'] and b['pol'] and not (b['mp'] or b['cp']):
        out.append(f"lat {b['pl']} deg"); out.append(f"lon {b['pol']} deg")
        if b['poa']: out.append(f"alt {b['poa']} km")

    fc = 0; seen = set()
    for k in sorted(b['fields']):
        if k in seen: continue; seen.add(k)
        f, u = b['fields'][k]
        out.append(f"field {k.lower()} {f} {u}")
        fc += 1
    if fc == 0: nf += 1; continue
    out.append('')
    results.append('\n'.join(out))

os.makedirs('phi/recovery/migration', exist_ok=True)
with open('phi/recovery/migration/sources_v2.φ', 'w') as f:
    f.write('\n'.join(results) + '\n')
print(f"\nDone: {len(results)} blocks ({nf} no-measurement)")
print(f"Output: phi/recovery/migration/sources_v2.φ")
print(f"Total lines: {sum(1 for _ in open('phi/recovery/migration/sources_v2.φ'))}")
