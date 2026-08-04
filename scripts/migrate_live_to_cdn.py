#!/usr/bin/env python3
"""Mirror live sources to CDN for health checking.
Modes:
  --mode migrate    Migrate non-positional TTL>=300 sources to CDN (update sources.φ)
  --mode mirror-live Mirror ALL non-CDN sources to CDN (keep original URLs, health check)
  --mode mirror-all  Mirror ALL non-CDN sources AND update sources.φ to CDN
  --dry-run          Skip actual uploads

Usage: OMEGAFLOW_TOKEN=ghp_xxx python3 scripts/migrate_live_to_cdn.py [--mode MODE] [--dry-run] [--limit N]
"""
import json
import os
import re
import sys
import time
import urllib.request
import urllib.error
import urllib.parse
import concurrent.futures
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent))
from flatten_cdn import flatten_to_universal

TOKEN = os.environ.get("OMEGAFLOW_TOKEN", "")
if not TOKEN:
    print("Missing OMEGAFLOW_TOKEN", file=sys.stderr)
    sys.exit(1)

MODE = "migrate"
for i, arg in enumerate(sys.argv):
    if arg == "--mode" and i + 1 < len(sys.argv):
        MODE = sys.argv[i + 1]

DRY_RUN = "--dry-run" in sys.argv
WORKERS = 4
LIMIT = None
MAX_TTL = None
for i, arg in enumerate(sys.argv):
    if arg == "--workers" and i + 1 < len(sys.argv):
        WORKERS = int(sys.argv[i + 1])
    if arg == "--limit" and i + 1 < len(sys.argv):
        LIMIT = int(sys.argv[i + 1])
    if arg == "--max-ttl" and i + 1 < len(sys.argv):
        MAX_TTL = int(sys.argv[i + 1])

API_BASE = "https://api.github.com"
UA = "omegaflow-mirror/1.0"
SOURCES_PHI = Path(__file__).parent.parent / "phi" / "sources.φ"
HEALTH_FILE = Path(__file__).parent.parent / "phi" / "health.json"

def fetch_url(url, timeout=30):
    try:
        req = urllib.request.Request(url, headers={"User-Agent": UA})
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return r.status, r.read()
    except Exception as e:
        return 0, str(e).encode()

def api_call(method, path, data=None):
    url = f"{API_BASE}{path}"
    headers = {"Authorization": f"Bearer {TOKEN}", "User-Agent": UA,
               "Accept": "application/vnd.github+json"}
    if data:
        headers["Content-Type"] = "application/json"
        req = urllib.request.Request(url, data=data.encode(), headers=headers, method=method)
    else:
        req = urllib.request.Request(url, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=30) as r:
            return r.status, json.loads(r.read()) if r.status != 204 else None
    except urllib.error.HTTPError as e:
        return e.code, json.loads(e.read()) if e.fp else {"message": str(e)}

def create_release(repo, tag):
    if DRY_RUN:
        return True
    status, _ = api_call("GET", f"/repos/{repo}/releases/tags/{tag}")
    if status == 200:
        return True
    payload = {"tag_name": tag, "name": tag, "body": ""}
    status, _ = api_call("POST", f"/repos/{repo}/releases", json.dumps(payload))
    return status in (200, 201)

def upload_asset(repo, tag, filename, data):
    if DRY_RUN:
        return True
    status, release = api_call("GET", f"/repos/{repo}/releases/tags/{tag}")
    if status != 200 or release is None:
        return False
    upload_url = release["upload_url"].split("{")[0]
    url = f"{upload_url}?name={urllib.parse.quote(filename)}"
    req = urllib.request.Request(url, data=data,
        headers={"Authorization": f"Bearer {TOKEN}", "User-Agent": UA,
                 "Content-Type": "application/octet-stream"}, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            return r.status == 201
    except Exception as e:
        return False

def parse_sources():
    with open(SOURCES_PHI) as f:
        content = f.read()
    blocks = content.strip().split('\n\n')
    sources = []
    for block in blocks:
        lines = block.strip().split('\n')
        if not lines or not lines[0].startswith('source '):
            continue
        name = lines[0].split()[1]
        is_cdn = any('releases/download' in l for l in lines)
        ttl = None
        url = None
        for l in lines:
            if l.startswith('ttl '): ttl = int(l.split()[1])
            if l.startswith('url '): url = l.split(None, 1)[1].strip()
        if not url:
            continue
        sources.append({
            "name": name, "ttl": ttl, "url": url, "is_cdn": is_cdn,
            "raw": block, "lines": lines
        })
    return sources, content

def get_domain_tag(url):
    from urllib.parse import urlparse
    parsed = urlparse(url)
    domain = parsed.netloc
    if domain.startswith('www.'):
        domain = domain[4:]
    return domain

def mirror_source(src, update_phi=False):
    name = src["name"]
    url = src["url"]
    domain = get_domain_tag(url)
    
    status, data = fetch_url(url)
    if status == 0 or status >= 400:
        return {"name": name, "ok": False, "error": f"HTTP {status}", "size": 0}
    
    if len(data) < 16:
        return {"name": name, "ok": False, "error": f"Too small ({len(data)} bytes)", "size": len(data)}
    

    try:
        flat_data, fmt_hint = flatten_to_universal(data, name)
    except Exception as e:
        return {"name": name, "ok": False, "error": f"Flatten failed: {e}", "size": len(data)}
    
    if not create_release("omegaflow/sources", domain):
        return {"name": name, "ok": False, "error": "Release create failed", "size": len(data)}
    
    asset_name = f"{name}.json"
    if not upload_asset("omegaflow/sources", domain, asset_name, flat_data):
        return {"name": name, "ok": False, "error": "Upload failed", "size": len(flat_data)}
    
    result = {"name": name, "ok": True, "size": len(data), "domain": domain}
    
    if update_phi:
        cdn_url = f"https://github.com/omegaflow/sources/releases/download/{domain}/{name}.json"
        result["cdn_url"] = cdn_url
    
    return result

def main():
    sources, content = parse_sources()
    

    if MODE == "migrate":

        candidates = [s for s in sources if not s["is_cdn"] and s.get("ttl", 0) >= 300
                     and not re.search(r'\{lat\}|\{lon\}|\{ra\}|\{dec\}|\{radius\}|\{alt\}', s["url"])]
    elif MODE == "mirror-live":

        candidates = [s for s in sources if not s["is_cdn"]]
        if MAX_TTL is not None:
            candidates = [s for s in candidates if s.get("ttl", 999999) <= MAX_TTL]
    elif MODE == "mirror-all":

        candidates = [s for s in sources if not s["is_cdn"]]
        update_phi = False
    else:
        print(f"Unknown mode: {MODE}", file=sys.stderr)
        sys.exit(1)
    
    if LIMIT:
        candidates = candidates[:LIMIT]
    
    print(f"Mode: {MODE}, sources: {len(candidates)}")
    
    if len(candidates) == 0:
        print("Nothing to do.")
        return
    
    health = {"timestamp": time.time(), "mode": MODE, "results": []}
    migrated = 0
    failed = 0
    
    with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as executor:
        futures = {}
        for s in candidates:
            should_update = MODE in ("migrate", "mirror-all") and s.get("ttl", 0) >= 300 \
                and not re.search(r'\{lat\}|\{lon\}|\{ra\}|\{dec\}|\{radius\}|\{alt\}', s["url"])
            futures[executor.submit(mirror_source, s, should_update)] = s
        for future in concurrent.futures.as_completed(futures):
            result = future.result()
            if result["ok"]:
                migrated += 1
                if migrated % 20 == 0:
                    print(f"  migrated: {migrated}/{len(candidates)}")
            else:
                failed += 1
                print(f"  FAIL: {result['name']} — {result['error']}", file=sys.stderr)
            health["results"].append(result)
    
    print(f"Done: {migrated} ok, {failed} failed")
    

    with open(HEALTH_FILE, 'w') as f:
        json.dump(health, f)
    

    phi_updated = 0
    for result in health["results"]:
        if not result["ok"] or "cdn_url" not in result:
            continue
        for src in candidates:
            if src["name"] == result["name"]:
                should_update = MODE in ("migrate", "mirror-all") and src.get("ttl", 0) >= 300 \
                    and not re.search(r'\{lat\}|\{lon\}|\{ra\}|\{dec\}|\{radius\}|\{alt\}', src["url"])
                if not should_update:
                    continue
                old_block = src["raw"]
                new_lines = []
                for line in old_block.split('\n'):
                    if line.startswith('url ') and line.split(None, 1)[1].strip() == src["url"]:
                        new_lines.append(f"url {result['cdn_url']}")
                    else:
                        new_lines.append(line)
                new_block = '\n'.join(new_lines)
                content = content.replace(old_block, new_block)
                phi_updated += 1
                break
    if phi_updated > 0 and not DRY_RUN:
        with open(SOURCES_PHI, 'w') as f:
            f.write(content)
        print(f"Updated {SOURCES_PHI} ({phi_updated} sources)")
    
    if failed > 0:
        sys.exit(1)

if __name__ == "__main__":
    main()
