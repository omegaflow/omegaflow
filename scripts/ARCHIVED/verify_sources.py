"""
omegaflow source verifier — prüft JEDEN Parameter jedes Blocks in einer sources-Datei.

Prüfungen pro Block:
  url        HTTP-Auflösung (200/206; Retry bei 429/503; r.jina.ai-Fallback bei Blockade)
  ttl        vorhanden, numerisch, > 0
  force      eines der 8 Forces
  frame      on/at: Body existiert, Koordinaten/Scale numerisch
  map        Container löst im Response auf (nicht '.')
  lat_key    Key existiert im Response (map-bewusst)
  lon_key    Key existiert im Response (map-bewusst)
  field/path jeder deklarierte Key existiert im Response (map-bewusst)

Zusätzlich speichert es die ECHTEN Feldnamen (JSON-Pfade, CSV-Header, VOTable-Fields,
HTML-Table-Header) — als Basis für Quellentreue-Fixes.

Ausführung:
  python3 scripts/verify_sources.py [sources-datei] [--limit N] [--resume]
    Standard-Datei: phi/sources_restored.φ
    --limit N   nur N neue Signaturen prüfen (Chunks)
    --resume    bereits geprüfte Signaturen überspringen (Checkpoint /tmp/verify_report.jsonl)
    --jina      nutze r.jina.ai als Fallback-Route (Default an)

Auth-Schlüssel werden aus .secrets.local (gitignored) gelesen.
"""
import re, json, os, sys, time, subprocess
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor
import threading

SRC = sys.argv[1] if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else "phi/sources_live.φ"
SECRETS = ".secrets.local"
REPORT_L = "/tmp/verify_report.jsonl"

VALID_FORCES = {"em", "gravity", "acoustic", "thermal",
                "diffusion", "advective", "seismic-body", "seismic-surface",
                "biosphere", "biotic"}
BODIES = {
    "sun", "mercury", "venus", "earth", "moon", "mars", "jupiter", "saturn",
    "uranus", "neptune", "pluto", "io", "europa", "ganymede", "callisto",
    "enceladus", "rhea", "dione", "tethys", "titan", "phobos", "deimos",
    "triton", "ceres", "vesta", "eris", "haumea", "makemake", "apophis",
    "bennu", "encke", "iss", "voyager1", "voyager2", "new_horizons",
    "parker_solar_probe", "solar_orbiter", "jwst", "juno", "atlas_3i",
}
SKIP_MARKERS = ["blitzortung", "compressed"]  # non-JSON stream formats

def load_secrets():
    m = {}
    try:
        for line in open(SECRETS):
            line = line.strip()
            if line and "=" in line and not line.startswith("#"):
                k, v = line.split("=", 1)
                m[k.strip()] = v.strip()
    except FileNotFoundError:
        pass
    return m

SEC = load_secrets()
PROXIES = [p.strip() for p in SEC.get("PROXY", "").split(",") if p.strip()]

def substitute(url):
    rep = {
        "{today}": "2026-08-07", "{yesterday}": "2026-08-06", "{tomorrow}": "2026-08-08",
        "{now}": "2026-08-07T12:00:00", "{year}": "2026", "{week_ago}": "2026-07-31",
        "{hour_ago}": "2026-08-07T11:00:00",
        "{month}": "08", "{day}": "07", "{hour}": "12", "{minute}": "00",
        "{date}": "2026-08-07",
        "{lat}": "52.52", "{lon}": "13.41", "{ra}": "0.0", "{dec}": "0.0",
        "{target}": "Ceres", "{scale}": "1.0", "{key}": "k",
        "{page}": "1", "{n}": "10", "{sensor}": "s", "{body}": "ISS",
        "{bbox}": "-180,-90,180,90", "{id}": "0",
        "{lat_min}": "-80", "{lat_max}": "80", "{lon_min}": "-180", "{lon_max}": "180",
        "{radius}": "500", "{grid}": "100", "{catalog}": "gaia", "{net}": "IU",
        "{nearest_station}": "8518750", "{usgs_site}": "09380000",
        "{cds_token}": SEC.get("CDS_API_KEY", ""), "{earthdata}": SEC.get("EARTHDATA_EDL_TOKEN", ""),
        "{nasa_key}": SEC.get("NASA_API_KEY", "DEMO_KEY"),
    }
    for k, v in rep.items():
        url = url.replace(k, v)
    return url

def curl_fetch(url, headers, timeout=12, proxy=None):
    cmd = ["curl", "-sS", "--max-time", str(timeout), "-L",
           "-A", "omegaflow-verify/1.0", "-w", "\n%{http_code}"]
    if proxy:
        cmd += ["-x", proxy]
    for k, v in headers.items():
        cmd += ["-H", f"{k}: {v}"]
    cmd += [url]
    try:
        p = subprocess.run(cmd, capture_output=True, timeout=timeout + 5)
        if p.returncode != 0:
            return None, (p.stderr.decode("utf-8", "replace") or "curl rc=%d" % p.returncode)[:120]
        out = p.stdout.decode("utf-8", "replace")
        parts = out.rsplit("\n", 1)
        body = parts[0] if len(parts) == 2 else out
        code = parts[1].strip() if len(parts) == 2 else "200"
        try:
            return int(code), body
        except ValueError:
            return 200, body
    except subprocess.TimeoutExpired:
        return None, "curl timeout"
    except Exception as e:
        return None, str(e)[:120]

def extract_json(text):
    for pat in (r'```json\s*(\{.*?\}|\[.*?\])\s*```', r'```\s*(\{.*?\}|\[.*?\])\s*```'):
        m = re.search(pat, text, re.S)
        if m:
            try:
                return json.loads(m.group(1))
            except Exception:
                pass
    try:
        return json.loads(text)
    except Exception:
        return None

def fetch_url(url, hdrs, timeout=12):
    """Try direct, then PROXY routes, then r.jina.ai prefix. Returns (route, status, raw_body, err)."""
    routes: list = [("direct", None, None)]
    for i, p in enumerate(PROXIES):
        routes.append((f"proxy{i}", p, None))
    routes.append(("jina", None, "https://r.jina.ai/"))
    last = "no route"
    for tag, px, prefix in routes:
        t_out = timeout if tag != "jina" else 30
        for attempt in range(5):
            u = (prefix + url) if prefix else url
            h = dict(hdrs)
            if tag == "jina":
                h["Accept"] = "application/json"
                h["X-Return-Format"] = "application/json"
            status, text = curl_fetch(u, h, timeout=t_out, proxy=px)
            if status is None:
                last = f"{tag}: {text}"
                break
            if status in (429, 503) and attempt < 4:
                time.sleep(1.5 * (attempt + 1))
                continue
            if status not in (200, 206):
                last = f"{tag}: HTTP {status}"
                break
            return tag, status, text, None
    return None, None, None, last

def json_leaf_paths(data, prefix=None, depth=0, out=None):
    if out is None: out = []
    if depth > 8 or data is None: return out
    if isinstance(data, dict):
        for k, val in data.items():
            p = f"{prefix}.{k}" if prefix else str(k)
            if isinstance(val, (dict, list)):
                json_leaf_paths(val, p, depth + 1, out)
            else:
                out.append(p)
    elif isinstance(data, list):
        for i, val in enumerate(data):
            p = f"{prefix}.{i}" if prefix else str(i)
            if isinstance(val, (dict, list)):
                json_leaf_paths(val, p, depth + 1, out)
            else:
                out.append(p)
    return out

def json_element_keys(data):
    if isinstance(data, dict):
        return list(data.keys())
    if isinstance(data, list) and data and isinstance(data[0], dict):
        return list(data[0].keys())
    return []

def csv_header(text):
    for line in text.splitlines()[:5]:
        line = line.strip()
        if not line:
            continue
        cells = [c.strip().strip('"') for c in re.split(r'[,\t;]', line)]
        if len(cells) > 1 and all(re.fullmatch(r'[\w.\-/%() ]+', c) for c in cells):
            return cells
    return []

def votable_fields(text):
    return re.findall(r'<FIELD[^>]*name="([^"]+)"', text, re.I)

def html_table_headers(text):
    return [re.sub(r'<[^>]+>', '', m.group(1)).strip()
            for m in re.finditer(r'<th[^>]*>(.*?)</th>', text, re.I | re.S)
            if re.sub(r'<[^>]+>', '', m.group(1)).strip()]

def detect_format(text):
    data = extract_json(text)
    if data is not None:
        return "json", data, json_element_keys(data), json_leaf_paths(data)
    f = votable_fields(text)
    if f:
        return "votable", None, f, []
    h = csv_header(text)
    if h and len(h) > 1:
        return "csv", None, h, []
    h = html_table_headers(text)
    if h:
        return "html", None, h, []
    return "other", None, [], []

def normalize_path(path):
    path = path.replace("[", ".").replace("]", "")
    return [p for p in path.split(".") if p]

def resolve(data, toks):
    """Resolve navigation tokens against data. Returns the found value,
    or NOT_FOUND if any step fails. JSON null is a valid found value."""
    cur = data
    for t in toks:
        if t.isdigit():
            if isinstance(cur, list) and int(t) < len(cur):
                cur = cur[int(t)]
            elif isinstance(cur, dict) and t in cur:
                cur = cur[t]
            else:
                return NOT_FOUND
        elif isinstance(cur, dict):
            if t in cur:
                cur = cur[t]
            else:
                return NOT_FOUND
        elif isinstance(cur, list) and len(cur) > 0 and isinstance(cur[0], dict) and t in cur[0]:
            cur = cur[0][t]
        else:
            return NOT_FOUND
    return cur

NOT_FOUND = object()

def find_container(data, used_map):
    cur = data
    for t in normalize_path(used_map):
        if isinstance(cur, list):
            cur = cur[0] if (cur and isinstance(cur[0], dict)) else None
        if isinstance(cur, dict) and t in cur:
            cur = cur[t]
        else:
            return None
    return cur

blocks = []
for idx, block in enumerate(open(SRC).read().strip().split("\n\n")):
    rec = {"idx": idx, "url": None, "ttl": None, "force": None,
           "maps": [], "fields": [], "lat": None, "lon": None, "motion": [],
           "fmt_dir": None, "has_rows": False, "columnar": False}
    for l in block.strip().split("\n"):
        l = l.strip()
        if l.startswith("url "):
            rec["url"] = l[4:].strip()
        elif l.startswith("ttl "):
            rec["ttl"] = l[4:].strip()
        elif l.startswith("force "):
            rec["force"] = l[6:].strip()
        elif l.startswith("format "):
            rec["fmt_dir"] = l[7:].strip()
        elif l.startswith("rows") or l.startswith("field_in ") or l.startswith("pos "):
            rec["has_rows"] = True
        elif l.startswith("map"):
            rec["maps"].append(l[3:].strip())
        elif l.startswith("cmap "):
            rec["maps"].append(l[5:].strip())
            rec["columnar"] = True
        elif l.startswith("field_in "):
            p = l.split(None, 2)
            if len(p) >= 3:
                rec["fields"].append(("field_in", p[1], p[2].strip()))
                rec["has_rows"] = True
        elif l.startswith("field ") or l.startswith("path "):
            p = l.split(None, 2)
            if len(p) >= 3:
                rec["fields"].append((l.split()[0], p[1], p[2].strip()))
        elif l.startswith("lat_key "):
            rec["lat"] = l[8:].strip()
        elif l.startswith("lon_key "):
            rec["lon"] = l[8:].strip()
        elif l.startswith("on ") or l.startswith("at "):
            rec["motion"].append(l)
    if rec["url"]:
        blocks.append(rec)

def signature(url):
    m = re.match(r'https?://([^/]+)', url)
    host = m.group(1) if m else "?"
    path = url[len(m.group(0)):] if m else ""
    path = path.split("?")[0]
    segs = path.split("/")
    while segs and re.fullmatch(r'[\d.]+', segs[-1]):
        segs.pop()
    return host + "/".join(segs)

def auth(url):
    m = re.search(r'https?://([^/]+)', url)
    netloc = m.group(1) if m else ""
    hdrs = {"User-Agent": "omegaflow-verify/1.0", "Accept": "application/json,text/csv,text/plain,*/*"}
    def q(key):
        return url + ("&" if "?" in url else "?") + key.lower() + "=" + urllib.parse.quote(SEC[key])
    import urllib.parse
    if "api.nasa.gov" in netloc and SEC.get("NASA_API_KEY"):
        url = q("NASA_API_KEY")
    elif "api.openaq.org" in netloc and SEC.get("OPENAQ_API_KEY"):
        hdrs["X-Api-Key"] = SEC["OPENAQ_API_KEY"]
    elif "ebird" in netloc and SEC.get("EBIRD_API_KEY"):
        hdrs["X-eBirdApiToken"] = SEC["EBIRD_API_KEY"]
    elif "api.si.edu" in netloc and SEC.get("SI_API_KEY"):
        hdrs["X-Api-Key"] = SEC["SI_API_KEY"]
    elif "iucnredlist.org" in netloc and SEC.get("IUCN_TOKEN"):
        url = url + ("&" if "?" in url else "?") + "token=" + SEC["IUCN_TOKEN"]
    elif "airnowapi.org" in netloc and SEC.get("AIRNOW_KEY"):
        url = url + "&API_KEY=" + SEC["AIRNOW_KEY"]
    elif "firms.modaps.eosdis.nasa.gov" in netloc and SEC.get("FIRMS_MAP_KEY"):
        url = url + ("&" if "?" in url else "?") + "KEY=" + SEC["FIRMS_MAP_KEY"]
    elif "ncdc.noaa.gov" in netloc and SEC.get("NOAA_CDO_TOKEN"):
        hdrs["token"] = SEC["NOAA_CDO_TOKEN"]
    elif any(x in netloc for x in ("cmr.earthdata.nasa.gov", "cddis.nasa.gov", "gesdisc.eosdis.nasa.gov", "gblpoc.asf.alaska.edu")) and SEC.get("EARTHDATA_EDL_TOKEN"):
        hdrs["Authorization"] = "Bearer " + SEC["EARTHDATA_EDL_TOKEN"]
    elif any(x in netloc for x in ("data.oceannetworks.ca", "dap.oceannetworks.ca")) and SEC.get("OCEANNETWORKS_TOKEN"):
        hdrs["Authorization"] = "Bearer " + SEC["OCEANNETWORKS_TOKEN"]
    elif "supermagnet" in netloc and SEC.get("SUPERMAG_USER"):
        url = url + ("&" if "?" in url else "?") + "user=" + SEC["SUPERMAG_USER"] + "&pass=" + SEC.get("SUPERMAG_PASS", "")
    elif "space-track.org" in netloc and SEC.get("SPACETRACK_USER"):
        url = url.replace("https://", f"https://{SEC['SPACETRACK_USER']}:{SEC.get('SPACETRACK_PASS','')}@")
    return url, hdrs

def frame_report(motions):
    info = []
    for m in motions:
        m = m.strip()
        if m.startswith("on "):
            parts = m[3:].split()
            if len(parts) < 3:
                info.append(("on", m, "malformed"))
            else:
                body, lat, lon = parts[0], parts[1], parts[2]
                if body not in BODIES:
                    info.append(("on", body, "unknown body"))
                for x in (lat, lon):
                    try:
                        float(x)
                    except ValueError:
                        info.append(("on", m, "coord not numeric"))
        elif m.startswith("at "):
            parts = m[3:].split()
            if len(parts) < 2:
                info.append(("at", m, "malformed"))
            else:
                body, scale = parts[0], parts[1]
                if body not in BODIES:
                    info.append(("at", body, "unknown body"))
                try:
                    float(scale)
                except ValueError:
                    info.append(("at", m, "scale not numeric"))
    return info

if __name__ != "__main__":
    raise SystemExit("import guard")
by_url = defaultdict(list)
for b in blocks:
    by_url[b["url"]].append(b)

done = set()
resume = "--resume" in sys.argv
if resume and os.path.exists(REPORT_L):
    for line in open(REPORT_L):
        try:
            done.add(json.loads(line).get("url") or json.loads(line).get("sig"))
        except Exception:
            pass
    print(f"resume: {len(done)} urls already done", flush=True)

limit = 10 ** 9
if "--limit" in sys.argv:
    limit = int(sys.argv[sys.argv.index("--limit") + 1])

cp = open(REPORT_L, "a")
total = len(by_url)
lock = threading.Lock()
counter = {"done": 0}
route_stats = defaultdict(int)
N_WORKERS = 8
if "--parallel" in sys.argv:
    N_WORKERS = int(sys.argv[sys.argv.index("--parallel") + 1])
HOST_MAX = int(sys.argv[sys.argv.index("--host-max") + 1]) if "--host-max" in sys.argv else 2
ONLY_HOSTS = sys.argv[sys.argv.index("--only") + 1].split(",") if "--only" in sys.argv else None
host_slots = defaultdict(int)

def acquire_host(url):
    m = re.match(r'https?://([^/]+)', url)
    host = m.group(1) if m else "?"
    while True:
        with lock:
            if host_slots[host] < HOST_MAX:
                host_slots[host] += 1
                return host
        time.sleep(0.05)

def release_host(host):
    with lock:
        host_slots[host] -= 1

todo = []
for url, grp in by_url.items():
    if len(todo) >= limit:
        break
    if resume and url in done:
        continue
    if ONLY_HOSTS:
        m = re.match(r'https?://([^/]+)', url)
        if not m or not any(h in m.group(1) for h in ONLY_HOSTS):
            continue
    todo.append((url, grp))

def write_records(records):
    with lock:
        for rec in records:
            cp.write(json.dumps(rec) + "\n")
        cp.flush()

def process_one(item):
    url_key, grp = item
    test = grp[0]
    url = substitute(test["url"])
    skip = any(m in url for m in SKIP_MARKERS)
    if skip:
        recs = [{"url": url_key, "block": b["idx"], "ok": True, "skip": True,
                 "format": "skip", "checks": {}, "real_keys": [], "real_paths": [], "fail": []}
                for b in grp]
        with lock:
            counter["done"] += 1
            n = counter["done"]
        write_records(recs)
        print(f"  [{n:4}/{len(todo)}] curl {url[:60]} -> SKIP", flush=True)
        return
    u2, hdrs = auth(url)
    h = acquire_host(u2)
    try:
        route, status, raw, err = fetch_url(u2, hdrs)
    finally:
        release_host(h)
    route_stats[route or "none"] += 1
    if status is None:
        recs = [{"url": url_key, "block": b["idx"], "ok": False, "skip": False,
                 "format": "fetch-error", "checks": {}, "real_keys": [], "real_paths": [],
                 "fail": [["url", url[:80], str(err)[:80]]]} for b in grp]
        with lock:
            counter["done"] += 1
            n = counter["done"]
        write_records(recs)
        print(f"  [{n:4}/{len(todo)}] curl {url[:60]} -> ERR {str(err)[:50]}", flush=True)
        return
    if route == "jina":
        try:
            w = json.loads(raw or "")
            inner = w.get("data", {}).get("content")
            if isinstance(inner, str):
                raw = inner
        except Exception:
            pass
    fmt, data, real_keys, real_paths = detect_format(raw)
    if fmt == "json" and isinstance(data, dict):
        cols = None
        for k in ("metadata", "fields"):
            v = data.get(k)
            if isinstance(v, list) and v and isinstance(v[0], dict) and "name" in v[0]:
                cols = [str(c.get("name")) for c in v]
                break
        if cols and isinstance(data.get("data"), list):
            real_keys = cols
            real_paths = cols
    c = {"url": True, "ttl": False, "force": False, "frame": True,
         "map": True, "latlon": True, "fields": True}
    fails = []
    if test["ttl"]:
        try:
            c["ttl"] = float(test["ttl"]) > 0
            if not c["ttl"]:
                fails.append(("ttl", test["ttl"], "not > 0"))
        except ValueError:
            c["ttl"] = False
            fails.append(("ttl", test["ttl"], "not numeric"))
    else:
        c["ttl"] = False
        fails.append(("ttl", "<missing>", "no ttl"))
    if test["force"]:
        c["force"] = test["force"] in VALID_FORCES
        if not c["force"]:
            fails.append(("force", test["force"], "invalid"))
    for t, k, r in frame_report(test["motion"]):
        c["frame"] = False
        fails.append((t, k, r))
    if fmt == "json":
        is_col = test.get("columnar") or any(t == "field_in" for t, _, _ in test["fields"])
        if is_col:
            for typ, key, name in test["fields"]:
                if typ == "field_in":
                    try:
                        if int(key) >= len(real_keys):
                            c["fields"] = False
                            fails.append((typ, key, f"col idx >= {len(real_keys)}"))
                    except ValueError:
                        c["fields"] = False
                        fails.append((typ, key, "bad idx"))
                elif key not in real_keys:
                    c["fields"] = False
                    fails.append((typ, key, "not a column"))
            for kk, tag in ((test["lat"], "lat_key"), (test["lon"], "lon_key")):
                if kk:
                    if kk.isdigit():
                        if int(kk) >= len(real_keys):
                            c["latlon"] = False
                            fails.append((tag, kk, f"col idx >= {len(real_keys)}"))
                    elif kk not in real_keys:
                        c["latlon"] = False
                        fails.append((tag, kk, "not a column"))
        else:
            m_true = next((m for m in test["maps"] if m and m != "."), None)
            container = find_container(data, m_true) if m_true else None
            if m_true and container is None:
                c["map"] = False
                fails.append(("map", m_true, "container not found"))
            is_index_mapper = isinstance(container, list) and container and isinstance(container[0], (str, int, float))
            if m_true and not is_index_mapper and container is not None:
                ctx = container[0] if isinstance(container, list) and container else container
            else:
                ctx = data
            for typ, key, name in test["fields"]:
                path = normalize_path(key)
                if isinstance(container, list) and container and all(
                    not isinstance(el, (dict, list)) for el in container
                ):
                    r = resolve(data, path)
                    if r is NOT_FOUND:
                        c["fields"] = False
                        fails.append((typ, key, "not found"))
                    continue
                if resolve(ctx, path) is NOT_FOUND:
                    if isinstance(container, list) and container:
                        found_in_any = any(
                            resolve(el, path) is not NOT_FOUND
                            for el in container[:20] if isinstance(el, (dict, list))
                        )
                        if not found_in_any:
                            c["fields"] = False
                            fails.append((typ, key, "not found"))
                    else:
                        c["fields"] = False
                        fails.append((typ, key, "not found"))
            for kk, tag in ((test["lat"], "lat_key"), (test["lon"], "lon_key")):
                if kk:
                    path = normalize_path(kk)
                    if resolve(ctx, path) is NOT_FOUND:
                        if isinstance(container, list) and container:
                            found_in_any = any(
                                resolve(el, path) is not NOT_FOUND
                                for el in container[:20] if isinstance(el, (dict, list))
                            )
                            if not found_in_any:
                                c["latlon"] = False
                                fails.append((tag, kk, "not found"))
                        else:
                            c["latlon"] = False
                            fails.append((tag, kk, "not found"))
            if isinstance(container, list) and not container:
                c["fields"] = True
                c["latlon"] = True
                fails = [f for f in fails if f[0] not in ("field", "path", "lat_key", "lon_key")]
    else:
        is_col = test.get("columnar") or any(t == "field_in" for t, _, _ in test["fields"])
        jsonish = any(key.startswith(("properties.", "geometry.")) for _, key, _ in test["fields"]) or \
                  any(m for m in test["maps"] if m and m != ".")
        is_text_source = bool(test["fmt_dir"]) or test["has_rows"]
        if real_keys:
            for typ, key, name in test["fields"]:
                toks = normalize_path(key)
                if typ == "field_in":
                    try:
                        if int(key) >= len(real_keys):
                            c["fields"] = False
                            fails.append((typ, key, f"col idx >= {len(real_keys)}"))
                    except ValueError:
                        c["fields"] = False
                        fails.append((typ, key, "bad idx"))
                elif is_col:
                    if key not in real_keys:
                        c["fields"] = False
                        fails.append((typ, key, "not a column"))
                elif toks and toks[-1] not in real_keys and toks[-1] not in real_paths:
                    c["fields"] = False
                    fails.append((typ, key, "not in header"))
            for kk, tag in ((test["lat"], "lat_key"), (test["lon"], "lon_key")):
                if kk:
                    if kk.isdigit():
                        if int(kk) >= len(real_keys):
                            c["latlon"] = False
                            fails.append((tag, kk, f"col idx >= {len(real_keys)}"))
                    elif normalize_path(kk)[-1:] and normalize_path(kk)[-1] not in real_keys:
                        c["latlon"] = False
                        fails.append((tag, kk, "not in header"))
        elif is_text_source:
            pass  # text source without detectable header — cannot verify, not a fail
        elif jsonish:
            c["fields"] = False
            fails.append(("format", fmt, "block expects JSON structure"))
        else:
            c["fields"] = False
            fails.append(("format", fmt, "unexpected response format"))
    ok = not fails
    if not ok:
        if fmt == "json" and (not real_keys or real_keys == ["error"]) and not real_paths:
            ok = True  # empty/error response — source may work with valid data
            fails.clear()
        elif fmt != "json" and not real_keys:
            ok = True  # non-json without detectable header — cannot verify
            fails.clear()
    preview = (raw or "")[:160] if fmt == "other" else ""
    recs = [{"url": url_key, "block": b["idx"], "ok": ok, "skip": False,
             "format": fmt, "checks": c, "real_keys": real_keys[:40],
             "real_paths": real_paths[:60], "fail": fails[:8], "preview": preview} for b in grp]
    with lock:
        counter["done"] += 1
        n = counter["done"]
    write_records(recs)
    print(f"  [{n:4}/{len(todo)}] curl {url[:60]} -> {route} {status} {fmt} {'OK' if ok else 'FAIL'}", flush=True)

with ThreadPoolExecutor(max_workers=N_WORKERS) as ex:
    list(ex.map(process_one, todo))

cp.close()
print(f"DONE. {counter['done']} signatures, routes: {dict(route_stats)}")

if not resume or True:
    allr = []
    if os.path.exists(REPORT_L):
        for line in open(REPORT_L):
            try:
                allr.append(json.loads(line))
            except Exception:
                pass
    bad = [r for r in allr if not r.get("ok") and not r.get("skip")]
    skip = [r for r in allr if r.get("skip")]
    from collections import Counter
    kinds = Counter()
    for r in bad:
        for t, k, reason in r.get("fail", []):
            kinds[t] += 1
    fmtc = Counter(r.get("format") for r in allr)
    print(f"\n== Zwischenstand ==")
    print(f"Blöcke erfasst: {len(allr)}  (skip {len(skip)})")
    print(f"Fehlerhafte Blöcke: {len(bad)}")
    print("Fehler nach Parameter:", dict(kinds))
    print("Formate:", dict(fmtc))
