#!/usr/bin/env python3
import json, re, ssl, sys, socket, concurrent.futures
from pathlib import Path
from urllib.parse import urlparse
sys.path.insert(0, str(Path(__file__).parent))
from restore_all_live import build_live_url_map

ROOT = Path(__file__).parent.parent
SRC = ROOT / "phi" / "sources.φ"
CACHE = ROOT / "phi" / "cache"
CACHE.mkdir(exist_ok=True)
UA = "omegaflow-verify/1.0"
TIMEOUT = 15
WORKERS = 24

_dns = concurrent.futures.ThreadPoolExecutor(max_workers=64)
socket.setdefaulttimeout(TIMEOUT)

def _resolve(host, port, timeout=6):
    f = _dns.submit(socket.getaddrinfo, host, port, 0, socket.SOCK_STREAM)
    try:
        return f.result(timeout=timeout)
    except concurrent.futures.TimeoutError:
        return None

def fetch(url, timeout=TIMEOUT):
    try:
        u = urlparse(url)
        host = u.hostname
        port = u.port or (443 if u.scheme == "https" else 80)
        addrs = _resolve(host, port)
        if not addrs:
            return 0, b"dns timeout"
        sock = socket.create_connection((host, port), timeout=timeout)
        if u.scheme == "https":
            ctx = ssl.create_default_context()
            ctx.check_hostname = False
            ctx.verify_mode = ssl.CERT_NONE
            sock = ctx.wrap_socket(sock, server_hostname=host)
        sock.settimeout(timeout)
        path = u.path or "/"
        if u.query:
            path += "?" + u.query
        req = f"GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: {UA}\r\nConnection: close\r\nAccept-Encoding: identity\r\n\r\n"
        sock.sendall(req.encode())
        chunks = []
        while True:
            try:
                d = sock.recv(65536)
            except socket.timeout:
                break
            if not d:
                break
            chunks.append(d)
        sock.close()
        raw = b"".join(chunks)
        if b"\r\n\r\n" in raw:
            head, body = raw.split(b"\r\n\r\n", 1)
            status = 200
            for ln in head.split(b"\r\n"):
                if ln.lower().startswith(b"http/"):
                    status = int(ln.split()[1])
            return status, body
        return 0, b"malformed"
    except Exception as e:
        return 0, str(e).encode()

def parse(r):
    t = r.decode("utf-8", errors="replace")
    try:
        return json.loads(t), True
    except:
        return t, False

# ---- Rust-faithful path semantics (mirrors main.rs jpath_val / jnum / jlast) ----
def scalar_of(v):
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return v
    if isinstance(v, str):
        try:
            return float(v)
        except:
            return None
    if v is True: return 1.0
    if v is False: return 0.0
    return None

def jpath_val(d, path):
    if d is None: return None
    if path == "" or path == ".": return d
    cur = d
    for part in path.split("."):
        if isinstance(cur, dict):
            if part in cur: cur = cur[part]
            else: return None
        elif isinstance(cur, list):
            try: idx = int(part)
            except: return None
            if idx < 0: idx = len(cur) + idx
            if idx < 0 or idx >= len(cur): return None
            cur = cur[idx]
        else:
            return None
    return cur

def jnum(d, key):
    if key in (".", ""): return scalar_of(d)
    v = jpath_val(d, key)
    if v is not None: return scalar_of(v)
    if isinstance(d, dict): return scalar_of(d.get(key))
    return None

def jlast(d, key):
    if "." in key:
        tp, fk = key.rsplit(".", 1)
        parent = d if tp in ("", ".") else jpath_val(d, tp)
        if isinstance(parent, list) and parent:
            v = parent[-1]
            if isinstance(v, dict): return scalar_of(v.get(fk))
            return scalar_of(v)
        return None
    if isinstance(d, list) and d:
        v = d[-1]
        if isinstance(v, dict): return scalar_of(v.get(key))
        return scalar_of(v)
    if isinstance(d, dict):
        v = d.get(key)
        if isinstance(v, list) and v:
            return scalar_of(v[-1])
    return None

def check_extractor(d, etype, spath):
    """Returns True if extractor would yield a value per Rust semantics."""
    if etype == "field": return jnum(d, spath) is not None
    if etype == "last": return jlast(d, spath) is not None
    if etype == "path": return jpath_val(d, spath) is not None
    if etype == "count":
        if spath in (".", ""): return isinstance(d, list)
        v = jpath_val(d, spath)
        return isinstance(v, list)
    return None

text = SRC.read_text(encoding="utf-8")
blocks, cur = [], []
for line in text.split("\n"):
    s = line.strip()
    if s == "":
        if cur: blocks.append(cur); cur = []
        continue
    if s.startswith("source "):
        if cur: blocks.append(cur)
        cur = [s]
    elif cur is not None:
        cur.append(s)
if cur: blocks.append(cur)

url_map = build_live_url_map()
print(f"{len(blocks)} blocks, {len(url_map)} recovered live URLs", flush=True)

def block_meta(blk):
    name = blk[0][7:].strip()
    url = ""
    for l in blk:
        if l.startswith("url "): url = l[4:].strip(); break
    return name, url

def is_cdn(url): return "releases/download" in url

# group blocks by url for shared fetch cache
url_to_blocks = {}
for b in blocks:
    name, url = block_meta(b)
    key = name
    # placeholders: same URL template shared; cache by template
    url_to_blocks.setdefault(url, []).append((name, b))

def cache_path(name):
    import hashlib
    return CACHE / (hashlib.md5(name.encode()).hexdigest()[:16] + ".raw")

def check_block(item):
    url, name_blocks = item
    name0 = name_blocks[0][0]
    if is_cdn(url):
        pass  # server fetches the CDN JSON itself; verify against it
    url = url.replace("{lat}","52.5").replace("{lon}","13.4").replace("{year}","2026").replace("{month}","1")
    cp = cache_path(url)
    if cp.exists():
        status, raw = 200, cp.read_bytes()
    else:
        status, raw = fetch(url)
        if status == 200:
            cp.write_bytes(raw)
    if status == 0:
        return {"n": name0, "err": f"fetch: {raw[:60].decode('utf-8','replace')}", "results": [], "dead": []}
    if status >= 400:
        return {"n": name0, "err": f"HTTP {status}", "results": [], "dead": []}
    data, is_json = parse(raw)
    out = []
    dead = []
    for name, blk in name_blocks:
        exts = []
        map_arr = None
        for l in blk:
            p = l.split(None, 1)
            if not p: continue
            k, v = p[0], p[1] if len(p) > 1 else ""
            if k in ("field","last","count","path"):
                vs = v.split()
                if len(vs) >= 2: exts.append((k, vs[0], vs[1]))
            elif k == "field_in":
                vs = v.split()
                if len(vs) >= 2: exts.append(("field_in", vs[0], vs[1]))
            elif k in ("map","cmap"):
                vs = v.split()
                if vs: map_arr = vs[0]
        if not exts and map_arr is None:
            dead.append((name, "no live extractors"))
            continue
        base = data
        if map_arr is not None and is_json:
            arr = jpath_val(data, map_arr)
            if isinstance(arr, list) and arr:
                base = arr[0]
        for etype, spath, target in exts:
            native = spath.rsplit(".", 1)[-1] if "." in spath else spath
            if etype == "field_in":
                # resolved relative to map element
                if is_json and jpath_val(base, spath) is not None:
                    if target != native: out.append((name, etype, spath, target, native, "dev"))
                    else: out.append((name, etype, spath, target, native, "ok"))
                else:
                    out.append((name, etype, spath, target, native, "bad" if is_json else "text"))
            elif not is_json:
                out.append((name, etype, spath, target, native, "text"))
            else:
                ok = check_extractor(base, "last" if etype=="last" else ("count" if etype=="count" else etype), spath)
                if ok:
                    if target != native: out.append((name, etype, spath, target, native, "dev"))
                    else: out.append((name, etype, spath, target, native, "ok"))
                else:
                    out.append((name, etype, spath, target, native, "bad"))
    return {"n": name0, "err": "", "results": out, "dead": dead}

all_tasks = list(url_to_blocks.items())
done = 0
with open(ROOT / "phi" / "api_check2.jsonl", "w") as ol:
    with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as pool:
        fut = {pool.submit(check_block, t): t for t in all_tasks}
        for f in concurrent.futures.as_completed(fut):
            done += 1
            try:
                r = f.result()
            except Exception as ex:
                r = {"n": "?", "err": f"crash: {ex}", "results": [], "dead": []}
            ol.write(json.dumps(r) + "\n"); ol.flush()
            bad = [x for x in r["results"] if x[5] == "bad"]
            if r.get("err") or bad:
                tag = r["err"] if r["err"] else f"{len(bad)} bad"
                print(f"[{done}/{len(all_tasks)}] {r['n']}: {tag}", flush=True)

# summary
ok = dev = bad = text = errs = 0
dead_lines = 0
rows = []
for line in open(ROOT / "phi" / "api_check2.jsonl"):
    r = json.loads(line)
    if r.get("err"): errs += 1
    dead_lines += len(r.get("dead", []))
    for n, e, sp, tg, nat, st in r["results"]:
        rows.append((n, e, sp, tg, nat, st))
        if st == "ok": ok += 1
        elif st == "dev": dev += 1
        elif st == "bad": bad += 1
        elif st == "text": text += 1
print(f"\n=== SUMMARY (server-faithful) ===", flush=True)
print(f"urls: {len(all_tasks)} fetch-errors: {errs}", flush=True)
print(f"extractors: ok={ok} dev={dev} bad={bad} text={text}", flush=True)
print(f"blocks with NO live extractors (dead lines): {dead_lines}", flush=True)

json.dump({"ok": ok, "dev": dev, "bad": bad, "text": text, "errs": errs,
           "bad": [{"s":n,"e":e,"p":sp,"t":tg} for n,e,sp,tg,nat,st in rows if st=="bad"],
           "dev": [{"s":n,"e":e,"p":sp,"t":tg,"nat":nat} for n,e,sp,tg,nat,st in rows if st=="dev"]},
          open(ROOT / "phi" / "api_check2.json","w"), indent=1)
print("\nphi/api_check2.json written", flush=True)
