#!/usr/bin/env python3
"""Fetch live sources with TTL >= 300 and migrate them to CDN releases.
Usage: OMEGAFLOW_TOKEN=ghp_xxx python3 scripts/migrate_live_to_cdn.py [--dry-run] [--workers N]
"""
import json
import os
import re
import sys
import time
import urllib.request
import urllib.error
import concurrent.futures
from pathlib import Path

TOKEN = os.environ.get("OMEGAFLOW_TOKEN", "")
if not TOKEN:
    print("Missing OMEGAFLOW_TOKEN", file=sys.stderr)
    sys.exit(1)

DRY_RUN = "--dry-run" in sys.argv
WORKERS = 4
for i, arg in enumerate(sys.argv):
    if arg == "--workers" and i + 1 < len(sys.argv):
        WORKERS = int(sys.argv[i + 1])

API_BASE = "https://api.github.com"
UA = "omegaflow-migration/1.0"
SOURCES_PHI = Path(__file__).parent.parent / "phi" / "sources.φ"

def api_get(path):
    req = urllib.request.Request(f"{API_BASE}{path}",
        headers={"Authorization": f"Bearer {TOKEN}", "User-Agent": UA,
                 "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.loads(r.read())

def fetch_url(url, timeout=30):
    try:
        req = urllib.request.Request(url, headers={"User-Agent": UA})
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return r.status, r.read()
    except Exception as e:
        return 0, str(e).encode()

def create_release(repo, tag, name):
    if DRY_RUN:
        print(f"  [dry] create release {repo} {tag}")
        return True
    data = json.dumps({"tag_name": tag, "name": name, "body": ""}).encode()
    req = urllib.request.Request(f"{API_BASE}/repos/{repo}/releases", data=data,
        headers={"Authorization": f"Bearer {TOKEN}", "User-Agent": UA,
                 "Accept": "application/vnd.github+json",
                 "Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=30) as r:
            return r.status in (200, 201)
    except urllib.error.HTTPError as e:
        if e.code == 422:  # already exists
            return True
        print(f"  release create error: {e}", file=sys.stderr)
        return False

def upload_asset(repo, release_id, filename, data):
    if DRY_RUN:
        print(f"  [dry] upload {filename} ({len(data)} bytes)")
        return True
    url = f"https://uploads.github.com/repos/{repo}/releases/{release_id}/assets?name={filename}"
    req = urllib.request.Request(url, data=data,
        headers={"Authorization": f"Bearer {TOKEN}", "User-Agent": UA,
                 "Accept": "application/vnd.github+json",
                 "Content-Type": "application/octet-stream"})
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            return r.status == 201
    except Exception as e:
        print(f"  upload error: {e}", file=sys.stderr)
        return False

def get_release_assets(repo, tag):
    try:
        release = api_get(f"/repos/{repo}/releases/tags/{tag}")
        return release.get("id"), [a["name"] for a in release.get("assets", [])]
    except:
        return None, []

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
        if any('releases/download' in l for l in lines):
            continue  # already CDN
        
        source = {"name": name, "raw": block, "lines": lines}
        for l in lines:
            if l.startswith('ttl '): source["ttl"] = int(l.split()[1])
            if l.startswith('url '): source["url"] = l.split(None, 1)[1]
            if l.startswith('force '): source["force"] = l.split()[1]
            if l.startswith('format '): source["format"] = l.split()[1]
        
        if source.get("ttl", 0) < 300:
            continue
        
        # Skip position-dependent sources
        url = source.get("url", "")
        if re.search(r'\{lat\}|\{lon\}|\{ra\}|\{dec\}|\{radius\}|\{alt\}', url):
            continue
        
        sources.append(source)
    
    return sources, content

def get_domain_tag(url):
    """Extract domain-based release tag from URL."""
    from urllib.parse import urlparse
    parsed = urlparse(url)
    return parsed.netloc

def get_cdn_url(source_name, domain_tag):
    """Build CDN release download URL."""
    return (f"https://github.com/omegaflow/sources/releases/download/"
            f"{domain_tag}/{source_name}.json")

def migrate_source(source):
    name = source["name"]
    url = source["url"]
    domain = get_domain_tag(url)
    
    # Fetch the data
    status, data = fetch_url(url)
    if status == 0:
        print(f"  {name}: FETCH FAILED")
        return None
    if status not in (200, 201, 202, 203, 204):
        print(f"  {name}: HTTP {status}")
        return None
    
    # Create release
    if not create_release("omegaflow/sources", domain, domain):
        print(f"  {name}: RELEASE FAILED")
        return None
    
    # Get release ID
    release_id, existing = get_release_assets("omegaflow/sources", domain)
    if release_id is None:
        print(f"  {name}: RELEASE NOT FOUND")
        return None
    
    # Upload asset
    asset_name = f"{name}.json"
    if not upload_asset("omegaflow/sources", release_id, asset_name, data):
        print(f"  {name}: UPLOAD FAILED")
        return None
    
    # Build new CDN URL
    cdn_url = get_cdn_url(name, domain)
    print(f"  {name}: OK ({len(data)} bytes)")
    
    return {
        "name": name,
        "old_url": url,
        "new_url": cdn_url,
        "domain": domain,
        "source": source,
    }

def main():
    sources, content = parse_sources()
    print(f"Sources to migrate: {len(sources)}")
    
    if len(sources) == 0:
        print("Nothing to do.")
        return
    
    migrated = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as executor:
        futures = {executor.submit(migrate_source, s): s for s in sources}
        for future in concurrent.futures.as_completed(futures):
            result = future.result()
            if result:
                migrated.append(result)
    
    print(f"\nMigrated: {len(migrated)}/{len(sources)}")
    
    if not DRY_RUN and migrated:
        # Update sources.φ
        for m in migrated:
            src = m["source"]
            old_block = src["raw"]
            new_lines = []
            for line in old_block.split('\n'):
                if line.startswith('url ') and line.split(None, 1)[1].strip() == m["old_url"]:
                    new_lines.append(f"url {m['new_url']}")
                else:
                    new_lines.append(line)
            new_block = '\n'.join(new_lines)
            content = content.replace(old_block, new_block)
        
        with open(SOURCES_PHI, 'w') as f:
            f.write(content)
        print(f"Updated {SOURCES_PHI}")

if __name__ == "__main__":
    main()
