#!/usr/bin/env python3
"""Create missing domain releases and migrate assets from old sphere releases.
Usage: python3 scripts/migrate_assets.py [--dry-run] [--create-only] [--migrate-only]
"""
import json, os, re, sys, time, urllib.error, urllib.parse, urllib.request
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed

TOKEN = os.environ.get("CATALOGS_TOKEN", "")
HEADERS = {"Authorization": f"Bearer {TOKEN}", "Accept": "application/vnd.github+json",
           "User-Agent": "omegaflow-bot/1.0"}
RELEASES_URL = "https://api.github.com/repos/omegaflow/sources/releases"
UPLOAD_URL = "https://uploads.github.com/repos/omegaflow/sources/releases/{release_id}/assets"
SOURCE_PHI = "phi/sources.φ"
DRY_RUN = "--dry-run" in sys.argv
CREATE_ONLY = "--create-only" in sys.argv
MIGRATE_ONLY = "--migrate-only" in sys.argv
WORKERS = 8

# Old sphere releases (assets are still there)
OLD_SPHERE_TAGS = [
    "catalogs-astro", "catalogs-geosphere", "catalogs-subatomic",
    "catalogs-atmosphere", "catalogs-magnetosphere", "catalogs-biosphere",
    "catalogs-exosphere", "catalogs-technosphere", "catalogs-hydrosphere",
    "catalogs-kp.gfz.de",  # mis-mapped USGS
    "catalogs-github.com-FreeCodeCamp-ProjectReferenceData",  # mis-mapped NASA
]


def log(msg):
    print(msg, file=sys.stderr, flush=True)


def extract_domain_tag(url_line):
    m = re.search(r"catalogs-([^/]+)/", url_line)
    return m.group(1) if m else None


def get_current_releases():
    """Return {tag: source_names} from current sources.φ."""
    result = defaultdict(list)
    lines = open(SOURCE_PHI).readlines()
    cur = None
    for line in lines:
        if line.startswith("source "):
            cur = line.strip().split()[1]
        if cur and line.startswith("url ") and "releases/download/" in line:
            tag = extract_domain_tag(line)
            if tag:
                result[tag].append(cur)
            cur = None
    return dict(result)


def get_old_asset_urls():
    """Return {source_name: old_cdn_url} for all former-sphere sources."""
    content = open(SOURCE_PHI).read()
    all_sources = set()
    for line in content.split("\n"):
        if line.startswith("source "):
            all_sources.add(line.strip().split()[1])

    old_urls = {}
    for tag in OLD_SPHERE_TAGS:
        for name in all_sources:
            old_urls[name] = f"https://github.com/omegaflow/sources/releases/download/{tag}/{name}.json"
    
    # Also check older releases: catalogs, catalogs-v2, v1.0
    old_format_tags = ["catalogs-v2", "catalogs"]
    for tag in old_format_tags:
        for name in all_sources:
            # pre-rename names might differ
            old_urls[name] = f"https://github.com/omegaflow/sources/releases/download/{tag}/{name}.json"
    
    return old_urls


def get_or_create_release(domain):
    """Get existing release ID or create a new one. Returns release_id.
    domain = the human-readable label (e.g. 'ndbc.noaa.gov').
    The actual GitHub tag is 'catalogs-{domain}'."""
    full_tag = f"catalogs-{domain}"
    check_url = f"{RELEASES_URL}/tags/{urllib.parse.quote(full_tag, safe='')}"
    req = urllib.request.Request(check_url, headers=HEADERS)
    try:
        with urllib.request.urlopen(req) as r:
            d = json.load(r)
            if d.get('id'):
                return d['id']
    except urllib.error.HTTPError as e:
        if e.code != 404:
            log(f"  ERROR checking {full_tag}: {e.code}")
            return None
    except Exception as e:
        log(f"  ERROR checking {full_tag}: {e}")
        return None
    
    if DRY_RUN:
        return 0
    
    body = json.dumps({
        "tag_name": full_tag,
        "name": full_tag,
        "body": f"CDN release for {domain}",
        "draft": False
    }).encode()
    req = urllib.request.Request(RELEASES_URL, data=body,
                                 headers={**HEADERS, "Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req) as r:
            d = json.load(r)
            log(f"  CREATED {full_tag} (id={d['id']})")
            return d['id']
    except urllib.error.HTTPError as e:
        err_body = e.read().decode(errors="replace")[:200]
        log(f"  FAIL create {full_tag}: {e.code} {err_body}")
        return None


def migrate_one(source_name, old_url, new_release_id):
    """Download from old release, upload to new release. Returns (name, success)."""
    if DRY_RUN:
        return (source_name, True)
    
    data = None
    
    # Try old sphere tag URL first
    for old_tag in OLD_SPHERE_TAGS:
        url = f"https://github.com/omegaflow/sources/releases/download/{old_tag}/{source_name}.json"
        try:
            req = urllib.request.Request(url, headers=HEADERS)
            with urllib.request.urlopen(req, timeout=120) as r:
                data = r.read()
            break
        except Exception:
            data = None
            continue
    
    # Try older formats
    if data is None:
        for old_tag in ["catalogs-v2", "catalogs"]:
            url = f"https://github.com/omegaflow/sources/releases/download/{old_tag}/{source_name}.json"
            try:
                req = urllib.request.Request(url, headers=HEADERS)
                with urllib.request.urlopen(req, timeout=120) as r:
                    data = r.read()
                break
            except Exception:
                continue
    
    if data is None:
        log(f"  MISS {source_name}: not found in any old release")
        return (source_name, False)
    
    # Upload to new release
    upload_req = urllib.request.Request(
        f"{UPLOAD_URL.format(release_id=new_release_id)}?name={source_name}.json",
        data=data, method="POST",
        headers={**HEADERS, "Content-Type": "application/octet-stream"})
    try:
        with urllib.request.urlopen(upload_req, timeout=120) as r:
            return (source_name, True)
    except urllib.error.HTTPError as e:
        body = e.read().decode(errors="replace")[:100]
        if e.code == 422:  # already exists
            return (source_name, True)
        log(f"  UPLOAD FAIL {source_name}: {e.code} {body}")
        return (source_name, False)


def main():
    log("=== ASSET MIGRATION ===")
    
    # Get current source -> tag mapping
    log("Step 1: Scanning sources.φ...")
    releases = get_current_releases()
    log(f"  {len(releases)} unique domain tags, {sum(len(v) for v in releases.values())} assets")
    
    # Create missing releases
    if not MIGRATE_ONLY:
        log(f"Step 2: Creating missing releases (existing: {len(releases)} tags)...")
        release_ids = {}
        created = 0
        for tag in sorted(releases.keys()):
            rid = get_or_create_release(tag)
            if rid:
                release_ids[tag] = rid
                if rid > 0:
                    created += 1
            time.sleep(0.5)
        log(f"  {created} new releases, {len(release_ids)} total ready")
        
        if CREATE_ONLY:
            return
    
    # Migrate assets
    if not CREATE_ONLY:
        log(f"Step 3: Migrating assets ({sum(len(v) for v in releases.values())} files)...")
        # Build tag -> release_id from existing releases
        release_ids = {}
        for tag in releases.keys():
            rid = get_or_create_release(tag)
            if rid:
                release_ids[tag] = rid
            time.sleep(0.3)
        
        if DRY_RUN:
            log("  Dry run — would migrate all assets")
            return
        
        # Use ThreadPool for parallel migration
        total = sum(len(sources) for sources in releases.values())
        done = 0
        failed = 0
        
        # Flatten tasks: (source_name, target_release_id)
        tasks = []
        for tag, sources in releases.items():
            rid = release_ids.get(tag)
            if not rid:
                log(f"  SKIP {tag}: no release ID")
                continue
            for name in sources:
                tasks.append((name, rid))
        
        with ThreadPoolExecutor(max_workers=WORKERS) as pool:
            futures = {pool.submit(migrate_one, name, None, rid): name for name, rid in tasks}
            for future in as_completed(futures):
                name, ok = future.result()
                done += 1
                if not ok:
                    failed += 1
                if done % 50 == 0:
                    log(f"  Progress: {done}/{len(tasks)} ({failed} failed)")
        
        log(f"\nDone. Migrated: {done - failed}/{done} ({failed} failed)")


if __name__ == "__main__":
    main()
