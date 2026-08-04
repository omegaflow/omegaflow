#!/usr/bin/env python3
"""Migrate: download from old releases (catalogs/catalogs-v2/v1.0), upload to new domain releases.
Handles pre-rename name mapping (Provider->Sphere rename from commit e93c1a6).
Usage: python3 scripts/fill_releases.py [--dry-run] [--workers N] [--limit N]
"""
import json, os, re, sys, time, urllib.error, urllib.parse, urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed

TOKEN = os.environ.get("CATALOGS_TOKEN", "")
HEADERS = {"Authorization": f"Bearer {TOKEN}", "Accept": "application/vnd.github+json",
           "User-Agent": "omegaflow-bot/1.0"}
CATALOGS_API = "https://api.github.com/repos/omegaflow/catalogs"
UPLOAD_URL = "https://uploads.github.com/repos/omegaflow/catalogs/releases/{release_id}/assets"
SOURCE_PHI = "phi/sources.φ"
DRY_RUN = "--dry-run" in sys.argv
WORKERS = int(sys.argv[sys.argv.index("--workers") + 1]) if "--workers" in sys.argv else 8
LIMIT = int(sys.argv[sys.argv.index("--limit") + 1]) if "--limit" in sys.argv else 0

# Old releases that HAVE assets
OLD_RELEASES = [
    ("catalogs-v2", 364587472),
    ("catalogs", 364488765),
    ("v1.0", 363018488),
]

# Sphere prefixes from commit e93c1a6
SPHERES = ["astro", "geosphere", "hydrosphere", "atmosphere", "magnetosphere",
           "biosphere", "exosphere", "subatomic", "technosphere"]


def log(msg):
    print(msg, file=sys.stderr, flush=True)


def get_current_mapping():
    """Return {source_name: {domain_tag, release_url}} for all CDN sources in sources.φ."""
    result = {}
    lines = open(SOURCE_PHI).readlines()
    cur = None
    for line in lines:
        if line.startswith("source "):
            cur = line.strip().split()[1]
        if cur and line.startswith("url ") and "releases/download/" in line:
            m = re.search(r"catalogs-([^/]+)/(.+)", line)
            if m:
                domain_tag = m.group(1)
                filename = m.group(2).rstrip()
                result[cur] = {
                    "domain_tag": domain_tag,
                    "filename": filename,
                    "url": line.strip()[4:]
                }
            cur = None
    return result


def list_old_assets():
    """Return {filename: (release_id, asset_id, url)} for all old release assets."""
    assets = {}
    for tag, rid in OLD_RELEASES:
        page = 1
        while True:
            url = f"{CATALOGS_API}/releases/{rid}/assets?per_page=100&page={page}"
            req = urllib.request.Request(url, headers=HEADERS)
            try:
                with urllib.request.urlopen(req, timeout=30) as r:
                    data = json.load(r)
                if not data:
                    break
                for a in data:
                    name = a["name"]
                    if name not in assets:  # prefer first found (v2 > catalogs > v1.0)
                        assets[name] = {
                            "release_id": rid,
                            "asset_id": a["id"],
                            "download_url": a["url"],
                            "from_tag": tag,
                        }
                page += 1
                time.sleep(0.2)
            except Exception as e:
                log(f"  Error listing {tag} page {page}: {e}")
                break
    return assets


def find_old_filename(current_name):
    """Try to find an old asset filename matching the current source name.
    Pre-rename (e93c1a6): strip sphere prefix. E.g. geosphere_arcgis_* -> arcgis_*"""
    # Direct match
    yield f"{current_name}.json"
    
    # Strip sphere prefix
    for s in SPHERES:
        if current_name.startswith(f"{s}_"):
            old = current_name[len(s) + 1:]
            yield f"{old}.json"
    
    # Other known transformations
    # Some sources had 'universe_' prefix renamed to 'astro_'
    if current_name.startswith("astro_"):
        old = current_name[5:]  # strip 'astro_'
        yield f"universe_{old}.json"


def get_release_id_for_tag(domain_tag):
    """Get the release ID for a domain tag. Returns None if not found."""
    tag_name = f"catalogs-{domain_tag}"
    url = f"{CATALOGS_API}/releases/tags/{urllib.parse.quote(tag_name, safe='')}"
    req = urllib.request.Request(url, headers=HEADERS)
    try:
        with urllib.request.urlopen(req, timeout=10) as r:
            d = json.load(r)
            return d["id"]
    except Exception:
        return None


def migrate_one(source_name, info, old_assets, release_id_cache):
    """Migrate one asset from old release to new domain release."""
    current_filename = info["filename"]
    domain_tag = info["domain_tag"]
    
    if DRY_RUN:
        # Check if old asset exists
        for old_fn in find_old_filename(source_name):
            if old_fn in old_assets:
                return (source_name, f"dry-run: {old_assets[old_fn]['from_tag']} -> {domain_tag}")
        return (source_name, "MISSING")
    
    # Find old asset
    old_entry = None
    for old_fn in find_old_filename(source_name):
        if old_fn in old_assets:
            old_entry = old_assets[old_fn]
            break
    
    if not old_entry:
        return (source_name, "MISSING")
    
    # Get target release ID
    if domain_tag not in release_id_cache:
        rid = get_release_id_for_tag(domain_tag)
        if not rid:
            return (source_name, f"NO_RELEASE {domain_tag}")
        release_id_cache[domain_tag] = rid
    target_rid = release_id_cache[domain_tag]
    
    # Download from old release
    try:
        req = urllib.request.Request(old_entry["download_url"], headers=HEADERS)
        with urllib.request.urlopen(req, timeout=120) as r:
            data = r.read()
    except Exception as e:
        return (source_name, f"DOWNLOAD_FAIL {str(e)[:50]}")
    
    # Upload to new release
    upload_req = urllib.request.Request(
        f"{UPLOAD_URL.format(release_id=target_rid)}?name={current_filename}",
        data=data, method="POST",
        headers={**HEADERS, "Content-Type": "application/octet-stream"})
    try:
        with urllib.request.urlopen(upload_req, timeout=120) as r:
            return (source_name, "OK")
    except urllib.error.HTTPError as e:
        if e.code == 422:  # already exists
            return (source_name, "EXISTS")
        err = e.read().decode(errors="replace")[:80]
        return (source_name, f"UPLOAD_{e.code} {err}")


def main():
    log("=== RELEASE ASSET MIGRATION ===")
    
    # Get current source -> domain mapping
    log("Step 1: Current mapping...")
    current = get_current_mapping()
    log(f"  {len(current)} CDN sources, {len(set(i['domain_tag'] for i in current.values()))} domain tags")
    
    # List old assets
    log("Step 2: Listing old assets...")
    old_assets = list_old_assets()
    log(f"  {len(old_assets)} old assets across {len(OLD_RELEASES)} releases")
    
    # Cache release IDs for domain tags
    release_id_cache = {}
    
    # Migrate
    log(f"Step 3: Migrating {len(current)} assets ({WORKERS} workers)...")
    if LIMIT:
        current = dict(list(current.items())[:LIMIT])
        log(f"  Limited to {LIMIT}")
    
    tasks = list(current.items())
    done, ok, missing, failed = 0, 0, 0, 0
    
    with ThreadPoolExecutor(max_workers=WORKERS) as pool:
        futures = {pool.submit(migrate_one, name, info, old_assets, release_id_cache): name
                   for name, info in tasks}
        for future in as_completed(futures):
            name, status = future.result()
            done += 1
            if status == "OK" or status == "EXISTS" or status.startswith("dry-run"):
                ok += 1
            elif status == "MISSING":
                missing += 1
            else:
                failed += 1
            if done % 100 == 0:
                log(f"  {done}/{len(tasks)} (ok={ok}, missing={missing}, fail={failed})")
    
    log(f"\nDone: {done} total, {ok} ok, {missing} missing, {failed} failed")
    
    if missing > 0:
        log("WARNING: Some sources had no old asset — these may be live-only or never-uploaded")
    if failed > 0:
        log("WARNING: Some uploads failed — re-run to retry")


if __name__ == "__main__":
    main()
