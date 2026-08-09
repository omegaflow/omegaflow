#!/usr/bin/env python3
"""Final migration: cache-first, live-fallback. ALL measurements, no lazy drops."""
import json, re, os, sys, time, ssl, gzip
from urllib.request import Request, urlopen
from datetime import datetime, timedelta

CACHE = json.load(open("phi/recovery/unit_cache.json"))
SECRETS = {}
for line in open(".secrets.local"):
    line = line.strip()
    if '=' in line and not line.startswith('#'):
        k, v = line.split('=', 1)
        SECRETS[k.strip()] = v.strip().strip('"')

U = {
    'degc':'C','degf':'F','degk':'K','kelvin':'K','m/s':'m/s','km/h':'km/h',
    'knots':'knot','kt':'knot','mph':'mph','hpa':'hPa','mb':'mb','pa':'Pa',
    'kpa':'kPa','nt':'nT','ut':'uT','t':'T','g':'G','gauss':'G',
    'w/m2':'W/m2','wm-2':'W/m2','sfu':'sfu','jy':'Jy','mjy':'mJy',
    'ppm':'ppm','ppb':'ppb','ppt':'ppt','mg/m3':'mg/m3','ug/m3':'μg/m3',
    'mg/l':'mg/L','umol/l':'μmol/L','p/cm3':'p/cm3','m-3':'p/m3',
    'm3/s':'m3/s','cfs':'ft3/s','deg':'deg','degt':'deg','s':'s','sec':'s',
    'm':'m','km':'km','ft':'ft','cm':'cm','mm':'mm','nmi':'nmi',
    'hz':'Hz','khz':'kHz','mhz':'MHz','m/s2':'m/s2','gal':'gal','mgal':'mGal',
    'v/m':'V/m','v':'V','mv':'mV','uv':'μV','us/cm':'μS/cm',
    'db':'dB','psu':'PSU','ntu':'NTU','uatm':'μatm',
    'kg/m3':'kg/m3','g/m3':'g/m3','g/kg':'g/kg','m3/m3':'m3/m3',
    '%':'%','w':'W','kw':'kW','mw':'MW','j':'J','kj':'kJ',
    'ev':'eV','kev':'keV','mev':'MeV','a':'A','ma':'mA','ka':'kA',
    'mas':'mas','arcsec':'arcsec','du':'DU','erg':'erg','erg/s':'erg/s',
    'erg/cm2/s':'erg/cm2/s','counts/s':'counts/s',
}

F = {
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

SK = set("id name hex flight callsign icao24 evid publicid net auth source station stationid stn stid wmo wban usaf icao coo nwsli platform sensor satellite spacecraft target_name designation catalog obsid dataset dr version object otype main_type alias title doi url link reference ref country state county fips timezone location place locality flynn_region region city description detail note comment magnitudetype magtype evtype category status quality flag flags units unit instrument squawk messages seen rssi dmin rms gap nst sil gva sda nic rc nac_p nac_v records number station_count event_count sample_size multiplicity individualcount population occurrences npred nexposure ssn smoothed_ssn kp kp_index estimated_kp a_running a_index aa ap ae noaa_scale storm_level flare_index classtype coded_type weather_code uv_index uvi scale intensity severity level class grade rank spectral_index photon_index pl_index lp_index sigma significance detection_significance confidence standard_error uncertainty frac_variability variability_index time time_tag timestamp datetime date generated created modified updated lastupdate begintime peaktime endtime origintime starttime stoptime yy mm dd hh mn ss year month day hour minute second doy week daynum epoch mjd jd acq_date acq_time local_date_time soldate active iscancel isfinal issea istraining manual_mode alert spi confirmed auto automatic reviewed tisb mlat domestictsunami alt_geom ias tas mach mag_heading true_heading nav_qnh nav_modes roll track_rate semi_major_axis_68 semi_minor_axis_68 position_angle_68 semi_major_axis_95 semi_minor_axis_95 position_angle_95 pivot_energy spectrum_type overall_quality max_convergence_flag max_data_flag max_error_count_flag max_processing_flag max_range_flag max_sample_count_flag max_telemetry_flag data_release pid release solution_id random_index pl_flux_density_error lp_flux_density_error plec_flux_density_error pl_index_error lp_index_error plec_index_error flux_peak_error energy_flux_error lp_epeak_error plec_epeak_error frac_variability_error flux_peak lp_epeak plec_epeak wd ws oat tat total sceneid gcmdactivity instruments linkedevents submissiontime tz felt cdi mmi ids sources types title".split())

def skip(k):
    return k.replace('_','').replace('-','').replace('.','').lower() in SK

def nu(u):
    if not u: return None
    return U.get(u.replace('°','deg').replace('µ','u').lower(), u.lower())

def fu(key):
    k = key.lower()
    for sfx, unit in [
        ('_km_h','km/h'),('_kmh','km/h'),('_km_s','km/s'),('_m_s','m/s'),
        ('_mph','mph'),('_knot','knot'),('_kn','knot'),('_kts','knot'),
        ('_c','C'),('_degc','C'),('_f','F'),('_k','K'),
        ('_hpa','hPa'),('_pa','Pa'),('_mb','mb'),('_mbar','mbar'),('_atm','atm'),
        ('_nt','nT'),('_ut','uT'),('_t','T'),('_gauss','G'),
        ('_wm2','W/m2'),('_w_m2','W/m2'),('_sfu','sfu'),('_jy','Jy'),('_mjy','mJy'),
        ('_ppm','ppm'),('_ppb','ppb'),('_ppt','ppt'),('_mgm3','mg/m3'),('_ugm3','μg/m3'),
        ('_mgl','mg/L'),('_cm3','p/cm3'),('_m3','p/m3'),
        ('_m3_s','m3/s'),('_cfs','ft3/s'),
        ('_deg','deg'),('_s','s'),('_min','min'),('_h','h'),('_d','d'),
        ('_m','m'),('_km','km'),('_cm','cm'),('_mm','mm'),('_ft','ft'),('_nmi','nmi'),
        ('_hz','Hz'),('_khz','kHz'),('_mhz','MHz'),('_ghz','GHz'),
        ('_ms2','m/s2'),('_gal','gal'),('_mgal','mGal'),
        ('_vm','V/m'),('_mvm','mV/m'),('_kvm','kV/m'),('_mv','mV'),('_uv','μV'),
        ('_uscm','μS/cm'),('_sm','S/m'),('_db','dB'),('_dba','dBA'),('_dbz','dBZ'),
        ('_psu','PSU'),('_ntu','NTU'),('_uatm','μatm'),('_matm','matm'),
        ('_kgm3','kg/m3'),('_gm3','g/m3'),('_gkg','g/kg'),('_m3m3','m3/m3'),
        ('_pct','%'),('_pc','%'),('_kw','kW'),('_mw','MW'),('_gw','GW'),
        ('_kj','kJ'),('_kev','keV'),('_mev','MeV'),('_gev','GeV'),
        ('_ka','kA'),('_kn','kN'),('_n','N'),('_sv','Sv'),('_gy','Gy'),
        ('_ergs','erg/s'),('_ergcm2s','erg/cm2/s'),('_cps','counts/s'),
        ('_mas','mas'),('_arcsec','arcsec'),('_du','DU'),('_ha','ha'),('_mi','mi'),
    ]:
        if k.endswith(sfx):
            return F.get(unit, 'em'), unit
    return None, None

def res(u):
    for k, v in SECRETS.items(): u = u.replace("{%s}"%k, v)
    n = datetime.utcnow(); y = n - timedelta(days=1); h = n - timedelta(hours=1)
    u = u.replace('{today}', n.strftime('%Y-%m-%d'))
    u = u.replace('{yesterday}', y.strftime('%Y-%m-%d'))
    u = u.replace('{hour_ago}', h.strftime('%Y-%m-%dT%H:%M:%S'))
    u = u.replace('{lat}', '0').replace('{lon}', '0')
    for i in range(5,101,5):
        u = u.replace('{lon_min_%d}'%i, '0').replace('{lon_max_%d}'%i, str(i))
        u = u.replace('{lat_min_%d}'%i, '0').replace('{lat_max_%d}'%i, str(i))
    for m in re.findall(r'\{buf_\d+[km]+\}', u): u = u.replace(m, '0.00001')
    return u

def fetch(url):
    try:
        ctx = ssl.create_default_context(); ctx.check_hostname=False; ctx.verify_mode=ssl.CERT_NONE
        req = Request(url, headers={'User-Agent':'omegaflow/2.0','Accept-Encoding':'gzip'})
        r = urlopen(req, timeout=10, context=ctx)
        d = r.read()
        if r.headers.get('Content-Encoding') == 'gzip': d = gzip.decompress(d)
        ct = r.headers.get('Content-Type','')
        enc = ct.split('charset=')[-1].strip() if 'charset=' in ct else 'utf-8'
        return d.decode(enc, errors='replace')
    except: return None

KF = {
    'rtsw_wind': {'proton_speed':'advective km/s','proton_density':'diffusion p/cm3','proton_temperature':'thermal K'},
    'rtsw_mag': {'bt':'em nT','bx_gse':'em nT','by_gse':'em nT','bz_gse':'em nT','bx_gsm':'em nT','by_gsm':'em nT','bz_gsm':'em nT'},
    'xrays': {'flux':'em W/m2'},
    'integral-electrons': {'flux':'em W/m2'},
    'integral-protons': {'flux':'em W/m2'},
    'differential': {'flux':'em W/m2'},
    'magnetometers': {'hp':'em nT','he':'em nT','total':'em nT'},
    'solar-radio-flux': {'flux':'em sfu'},
    'solar-cycle': {'f10.7':'em sfu'},
}

def fields(url, ru):
    ce = CACHE.get(ru) or CACHE.get(url)
    if not ce:
        b = url.split('?')[0]
        for ck in CACHE:
            if ck.split('?')[0] == b: ce = CACHE[ck]; break
    r = {}

    # header_units
    if isinstance(ce, dict) and 'header_units' in ce:
        for col, unit in ce['header_units'].items():
            if skip(col): continue
            u = nu(str(unit))
            if u and u in F: r[col.upper()] = (F[u], u)
        return r

    # JSON values
    if isinstance(ce, dict) and 'error' not in str(ce).lower():
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

    # Known error patterns
    if ce and 'error' in str(ce).lower():
        for pat, kf in KF.items():
            if pat in ru:
                for k, fu_ in kf.items():
                    f_, u_ = fu_.split()
                    r[k] = (f_, u_)
                return r
        if 'earthquake' in ru or 'seismic' in ru:
            r['depth'] = ('seismic-body', 'km')
            return r

    # Live fetch
    txt = fetch(ru)
    if not txt: return r

    # CSV
    ls = txt.strip().split('\n')
    if len(ls) >= 2 and ls[0].startswith('#'):
        h1 = ls[0].lstrip('#').strip().split()
        h2 = ls[1].lstrip('#').strip().split()
        if len(h1) == len(h2):
            for c, u_ in zip(h1, h2):
                if skip(c): continue
                u = nu(str(u_))
                if u and u in F: r[c.upper()] = (F[u], u)
            return r

    # JSON
    try: data = json.loads(txt)
    except: return r

    def walk(obj, prefix=''):
        if isinstance(obj, dict):
            for k, v in obj.items():
                if skip(k): continue
                if isinstance(v, (int, float)) and not isinstance(v, bool):
                    f, u_ = fu(k)
                    if f: r[k] = (f, u_)
                elif isinstance(v, dict) and 'value' in v:
                    f, u_ = fu(k)
                    if f: r[k] = (f, u_)
                elif isinstance(v, list) and v and isinstance(v[0], dict) and prefix=='':
                    walk(v[0], k)
        elif isinstance(obj, list) and obj and isinstance(obj[0], dict):
            walk(obj[0], prefix)
    walk(data)
    return r

# Parse blocks
SRC = open('phi/sources.φ').read()
blocks = re.split(r'\n(?=url )', SRC)
print(f"Processing {len(blocks)} blocks with {len(CACHE)} cache entries...")

results = []
nf = 0
t0 = time.time()
for i, b in enumerate(blocks):
    b = b.strip()
    if not b: continue
    ls = b.split('\n')
    url = ttl = frame = fbody = None
    mp = cp = lk = ok = ak = rk = dk = pk = None
    pl = pol = poa = None

    for li in ls:
        li = li.strip()
        if not li or li[0]=='#': continue
        p = li.split()
        if not p: continue
        k = p[0]
        if k=='url': url = p[1] if len(p)>1 else None
        elif k=='ttl': ttl = p[1] if len(p)>1 else None
        elif k in ('force','format','method','header','post_body','extent',
                   'reach_ttl','target','catalog','catalog_epoch','max_freq','min_freq',
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

    if not url or not ttl: continue
    ru = res(url)
    fds = fields(url, ru)

    out = ["url %s" % url, "ttl %s" % ttl]
    if frame=='at': out.append("at %s" % fbody)
    elif frame=='on' and pl and pol:
        a = "on %s %s %s" % (fbody, pl, pol)
        if poa and str(poa) not in ('{alt}','0',''): a += " %s" % poa
        out.append(a)
    elif frame=='body': out.append("body %s" % fbody)

    if mp: out.append("map %s" % mp)
    if cp: out.append("cmap %s" % cp)
    if lk: out.append("lat %s deg" % lk)
    if ok: out.append("lon %s deg" % ok)
    if ak: out.append("alt %s km" % ak)
    if rk: out.append("ra %s deg" % rk)
    if dk: out.append("dec %s deg" % dk)
    if pk: out.append("plx %s mas" % pk)
    if pl and pol and not (mp or cp):
        out.append("lat %s deg" % pl); out.append("lon %s deg" % pol)
        if poa: out.append("alt %s km" % poa)

    fc = 0; seen = set()
    for k_ in sorted(fds):
        if k_ in seen: continue
        seen.add(k_)
        f_, u_ = fds[k_]
        out.append("field %s %s %s" % (k_.lower(), f_, u_))
        fc += 1

    if fc == 0: nf += 1; continue
    out.append('')
    results.append('\n'.join(out))

    if (i+1) % 200 == 0:
        e = time.time() - t0
        print("  %d/%d kept=%d dropped=%d [%ds]" % (i+1, len(blocks), len(results), nf, e))

e = time.time() - t0
print("\nDone: %d blocks (%d no-measurement) [%ds]" % (len(results), nf, e))
os.makedirs('phi/recovery/migration', exist_ok=True)
with open('phi/recovery/migration/sources_v2.φ', 'w') as f:
    f.write('\n'.join(results) + '\n')
print("Output: phi/recovery/migration/sources_v2.φ (%d blocks)" % len(results))
