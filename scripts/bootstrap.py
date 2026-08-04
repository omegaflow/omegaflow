#!/usr/bin/env python3
"""Post-rename bootstrap: create bare-domain releases and migrate assets.
Run AFTER the repo has been renamed to omegaflow/sources.
Usage: CATALOGS_TOKEN=ghp_xxx python3 scripts/bootstrap.py [--dry-run] [--workers N]
"""
import json, os, re, sys, time, urllib.error, urllib.parse, urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed

TOKEN = os.environ.get("CATALOGS_TOKEN", "")
if not TOKEN:
    print("Missing CATALOGS_TOKEN", file=sys.stderr)
    sys.exit(1)

HEADERS = {"Authorization": f"Bearer {TOKEN}", "Accept": "application/vnd.github+json",
           "User-Agent": "omegaflow-bot/1.0"}
REPO = "omegaflow/sources"
API = f"https://api.github.com/repos/{REPO}"
DRY_RUN = "--dry-run" in sys.argv
WORKERS = int(sys.argv[sys.argv.index("--workers") + 1]) if "--workers" in sys.argv else 6
SOURCE_PHI = "phi/sources.φ"

# Old release tags on the renamed repo (assets are here)
OLD_LEGACY = [
    ("catalogs", 364488765),
    ("catalogs-v2", 364587472),
    ("v1.0", 363018488),
]


def log(msg):
    print(msg, file=sys.stderr, flush=True)


def get_current_domains():
    """Return {(domain_tag, source_name, cdn_filename)} for all CDN sources."""
    results = set()
    lines = open(SOURCE_PHI).readlines()
    cur = None
    for line in lines:
        if line.startswith("source "):
            cur = line.strip().split()[1]
        if cur and line.startswith("url ") and "releases/download/" in line:
            m = re.search(r"releases/download/([^/]+)/(.+)", line)
            if m:
                domain_tag = m.group(1)
                cdn_filename = m.group(2).rstrip()
                results.add((domain_tag, cur, cdn_filename))
            cur = None
    return results


def api_get(url):
    req = urllib.request.Request(url, headers=HEADERS)
    try:
        with urllib.request.urlopen(req, timeout=15) as r:
            return json.load(r)
    except Exception:
        return None


def list_all_releases():
    """Return {tag_name: release_id} for all releases on the repo."""
    releases = {}
    page = 1
    while True:
        data = api_get(f"{API}/releases?per_page=100&page={page}")
        if not data:
            break
        for rel in data:
            releases[rel["tag_name"]] = rel["id"]
        page += 1
        time.sleep(0.3)
    return releases


def get_release_assets(release_id):
    """Return {asset_name: asset_info} for a release."""
    assets = {}
    page = 1
    while True:
        data = api_get(f"{API}/releases/{release_id}/assets?per_page=100&page={page}")
        if not data:
            break
        for a in data:
            assets[a["name"]] = {"id": a["id"], "url": a["url"]}
        page += 1
        time.sleep(0.3)
    return assets


def create_release(domain_tag):
    tag = domain_tag.replace("/", "-")
    body = json.dumps({"tag_name": tag, "name": tag, "body": f"CDN for {domain_tag}", "draft": False}).encode()
    req = urllib.request.Request(f"{API}/releases", data=body,
                                 headers={**HEADERS, "Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=15) as r:
            d = json.load(r)
            log(f"  CREATED {tag} (id={d['id']})")
            return d["id"]
    except urllib.error.HTTPError as e:
        if e.code == 422:  # already exists
            return None
        err = e.read().decode(errors="replace")[:150]
        log(f"  FAIL {tag}: {e.code} {err}")
        return None


def migrate_assets_for_domain(domain, sources, existing_releases, old_assets_cache):
    """For a domain, ensure release exists and upload all its sources' assets."""
    tag = domain.replace("/", "-")
    
    if tag not in existing_releases:
        if DRY_RUN:
            log(f"  Would create {tag}")
            return 0, 0
        rid = create_release(domain)
        if not rid:
            return 0, len(sources)
        existing_releases[tag] = rid
    rid = existing_releases[tag]
    
    upload_url = f"https://uploads.github.com/repos/{REPO}/releases/{rid}/assets"
    ok, fail = 0, 0
    
    for source_name, cdn_filename in sources:
        if DRY_RUN:
            ok += 1
            continue
        
        # Find the asset in any old release
        data = None
        old_fn = f"{source_name}.json"
        
        # First check catalogs-{domain} old release (from pre-rename migration)
        old_cats_tag = f"catalogs-{domain}"
        if old_cats_tag in existing_releases:
            old_rid = existing_releases[old_cats_tag]
            if old_rid not in old_assets_cache:
                old_assets_cache[old_rid] = get_release_assets(old_rid)
            
            if old_fn in old_assets_cache.get(old_rid, {}):
                dl_url = old_assets_cache[old_rid][old_fn]["url"]
                try:
                    req = urllib.request.Request(dl_url, headers=HEADERS)
                    with urllib.request.urlopen(req, timeout=60) as r:
                        data = r.read()
                except Exception:
                    pass
        
        # Fallback to legacy releases
        if data is None:
            for old_tag, old_rid in OLD_LEGACY:
                if old_rid not in old_assets_cache:
                    old_assets_cache[old_rid] = get_release_assets(old_rid)
                
                # Try various old filename patterns (pre-rename)
                sp = source_name.split("_", 2)
                if len(sp) >= 3:
                    old_name = f"{sp[2]}.json"  # strip sphere prefix
                else:
                    old_name = old_fn
                
                candidates = [old_fn, old_name]
                found = False
                for candidate in candidates:
                    if candidate in old_assets_cache.get(old_rid, {}):
                        dl_url = old_assets_cache[old_rid][candidate]["url"]
                        try:
                            req = urllib.request.Request(dl_url, headers=HEADERS)
                            with urllib.request.urlopen(req, timeout=60) as r:
                                data = r.read()
                            found = True
                            break
                        except Exception:
                            pass
                if found:
                    break
        
        if data is None:
            fail += 1
            continue
        
        # Upload
        upload_req = urllib.request.Request(
            f"{upload_url}?name={cdn_filename}",
            data=data, method="POST",
            headers={**HEADERS, "Content-Type": "application/octet-stream"})
        try:
            with urllib.request.urlopen(upload_req, timeout=120) as r:
                ok += 1
        except urllib.error.HTTPError as e:
            if e.code == 422:
                ok += 1  # already exists
            else:
                fail += 1
    
    return ok, fail


def main():
    log("=== BOOTSTRAP: bare-domain releases + asset migration ===")
    
    # Get domain→sources mapping
    log("Step 1: Domain→source mapping...")
    domain_sources = {}
    for domain, source_name, cdn_filename in get_current_domains():
        if domain not in domain_sources:
            domain_sources[domain] = []
        domain_sources[domain].append((source_name, cdn_filename))
    log(f"  {len(domain_sources)} unique domains, {sum(len(v) for v in domain_sources.values())} assets")
    
    # List existing releases
    log("Step 2: Existing releases...")
    existing = list_all_releases()
    log(f"  {len(existing)} releases exist")
    
    # Count catalogs- prefixed releases (to be replaced)
    old_cats = {k: v for k, v in existing.items() if k.startswith("catalogs-")}
    log(f"  {len(old_cats)} old catalogs- releases to migrate from")
    
    # Create missing bare releases + migrate assets
    log(f"Step 3: Bare releases + migration ({len(domain_sources)} domains, {WORKERS} workers)...")
    
    if DRY_RUN:
        missing = [d for d in domain_sources if d.replace("/", "-") not in existing]
        log(f"  Would create {len(missing)} releases, migrate {sum(len(v) for v in domain_sources.values())} assets")
        for d in sorted(missing)[:10]:
            log(f"    {d}")
        return
    
    total_ok, total_fail = 0, 0
    done = 0
    old_assets_cache = {}
    
    # Download old assets cache for legacy releases (one-time)
    log("  Caching legacy release assets...")
    for old_tag, old_rid in OLD_LEGACY:
        if old_rid not in old_assets_cache:
            old_assets_cache[old_rid] = get_release_assets(old_rid)
            log(f"    {old_tag}: {len(old_assets_cache[old_rid])} assets")
    
    # Also cache old catalogs- releases (sample ~10 largest)
    for old_tag in sorted(old_cats.keys())[:50]:
        old_rid = old_cats[old_tag]
        if old_rid not in old_assets_cache:
            try:
                old_assets_cache[old_rid] = get_release_assets(old_rid)
                time.sleep(0.2)
            except Exception:
                pass
    
    with ThreadPoolExecutor(max_workers=WORKERS) as pool:
        futures = {}
        for domain, sources in domain_sources.items():
            f = pool.submit(migrate_assets_for_domain, domain, sources, existing, old_assets_cache)
            futures[f] = domain
        
        for future in as_completed(futures):
            domain = futures[future]
            ok, fail = future.result()
            total_ok += ok
            total_fail += fail
            done += 1
            if done % 20 == 0:
                log(f"  {done}/{len(domain_sources)} domains, {total_ok} ok, {total_fail} fail")
    
    log(f"\nDone: {done} domains, {total_ok} ok, {total_fail} failed")
    
    if total_fail > 0:
        log("Re-run to retry failed assets")


if __name__ == "__main__":
    main()
