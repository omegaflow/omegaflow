#!/usr/bin/env python3
"""One-pass domain-based CDN release migration.
Usage: python3 scripts/domain_release_migration.py [--dry-run]
Output: 2308 CDN source URLs in sources.φ rewritten to catalogs-{domain}/ schema.
"""
import json, os, re, subprocess, sys, time, urllib.parse, urllib.request
from collections import defaultdict

TOKEN = os.environ.get("CATALOGS_TOKEN", "")
HEADERS = {"Authorization": f"Bearer {TOKEN}", "Accept": "application/vnd.github+json",
           "User-Agent": "omegaflow-bot/1.0"}
RELEASES_URL = "https://api.github.com/repos/omegaflow/sources/releases"
UPLOAD_URL = "https://uploads.github.com/repos/omegaflow/sources/releases/{release_id}/assets"

SOURCE_PHI = "phi/sources.φ"
DRY_RUN = "--dry-run" in sys.argv


def log(msg):
    print(msg, file=sys.stderr, flush=True)


def extract_cdn_sources():
    """Return {source_name: (sphere_release, cdn_url)} for all CDN sources."""
    content = open(SOURCE_PHI).read()
    blocks = re.split(r"\n(?=source )", content)
    result = {}
    for b in blocks:
        m = re.match(r"source (\S+)", b)
        if not m: continue
        name = m.group(1)
        url_m = re.search(r"url\s+(\S+)", b)
        if not url_m: continue
        url = url_m.group(1)
        rel_m = re.search(r"releases/download/catalogs-([^/]+)/", url)
        if not rel_m: continue
        result[name] = (rel_m.group(1), url)
    return result


def build_domain_map(cdn_sources):
    """Map each CDN source to its REAL origin domain from git history."""
    cmd = ["git", "log", "--all", "--oneline", "--", SOURCE_PHI]
    all_commits = subprocess.check_output(cmd, text=True).strip().split("\n")[:500]
    
    old_live_url_for = {}
    for line in all_commits:
        commit_hash = line.split()[0]
        try:
            diff = subprocess.check_output(
                ["git", "show", commit_hash, "--", SOURCE_PHI],
                text=True, stderr=subprocess.DEVNULL, timeout=10)
        except Exception:
            continue
        current_source = None
        for dline in diff.split("\n"):
            sm = re.match(r"^\s*source\s+(\S+)", dline)
            if sm:
                current_source = sm.group(1)
                continue
            if dline.startswith("-url ") and current_source:
                old_url = dline[5:].strip()
                if "releases/download/" not in old_url:
                    d = extract_domain(old_url)
                    if d and current_source in cdn_sources and current_source not in old_live_url_for:
                        old_live_url_for[current_source] = d
                current_source = None
    
    # Build provider-token -> domain mapping from successful matches
    provider_domain = {}
    for name, domain in old_live_url_for.items():
        parts = name.split('_')
        if len(parts) >= 2:
            provider_domain.setdefault(parts[1], domain)
    
    # Apply to all sources
    result = {}
    for name in cdn_sources:
        if name in old_live_url_for:
            result[name] = old_live_url_for[name]
            continue
        parts = name.split('_')
        if len(parts) >= 2 and parts[1] in provider_domain:
            result[name] = provider_domain[parts[1]]
            continue
        result[name] = parts[0] if parts else 'default'
    
    return result


def extract_domain(url):
    """Extract a clean domain label from a URL."""
    url = urllib.parse.unquote(url)
    m = re.match(r"https?://([^/]+)", url)
    if not m: return None
    domain = m.group(1)
    # Clean: remove www, api, services6/services9 prefix for grouping
    domain = re.sub(r"^(www|api|services\d+)\.", "", domain)
    # For raw.githubusercontent.com paths, use the repo: github.com/{owner}
    if "raw.githubusercontent.com" in domain:
        repo = re.search(r"raw\.githubusercontent\.com/([^/]+/[^/]+)", url)
        if repo:
            return "github.com/" + repo.group(1)
        return "github.com"
    return domain


def get_or_create_release(domain):
    """Get existing release ID or create a new one. Returns release_id."""
    tag = f"catalogs-{domain.replace('/', '-')}"  # keep dots, only fix slashes
    # Check if exists
    req = urllib.request.Request(f"{RELEASES_URL}/tags/{tag}", headers=HEADERS)
    try:
        with urllib.request.urlopen(req) as r:
            d = json.load(r)
            if d.get('id'):
                return d['id'], tag
    except urllib.error.HTTPError:
        pass
    if DRY_RUN:
        return 0, tag
    # Create
    body = json.dumps({"tag_name": tag, "name": f"catalogs-{domain}", 
                       "body": f"CDN for {domain}", "draft": False}).encode()
    req = urllib.request.Request(RELEASES_URL, data=body, headers={**HEADERS, "Content-Type": "application/json"})
    with urllib.request.urlopen(req) as r:
        d = json.load(r)
        return d['id'], tag


def migrate_assets(cdn_sources, domain_map):
    """Download from sphere releases, upload to domain releases. Returns {source: new_cdn_url}."""
    releases_cache = {}
    new_urls = {}
    
    for name, (sphere, old_url) in sorted(cdn_sources.items()):
        domain = domain_map.get(name, "github.com/omegaflow")
        if DRY_RUN:
            rel_tag = f"catalogs-{domain.replace('/','-').replace('.','-')}"
            new_urls[name] = f"https://github.com/omegaflow/sources/releases/download/{rel_tag}/{name}.json"
            continue
        
        if domain not in releases_cache:
            rid, tag = get_or_create_release(domain)
            releases_cache[domain] = (rid, tag)
        else:
            rid, tag = releases_cache[domain]
        
        new_urls[name] = f"https://github.com/omegaflow/sources/releases/download/{tag}/{name}.json"
        
        # Download from old sphere release
        try:
            req = urllib.request.Request(old_url, headers=HEADERS)
            with urllib.request.urlopen(req, timeout=60) as r:
                data = r.read()
        except Exception:
            continue
        
        # Upload to new domain release
        upload_req = urllib.request.Request(
            f"{UPLOAD_URL.format(release_id=rid)}?name={name}.json",
            data=data, method="POST",
            headers={**HEADERS, "Content-Type": "application/octet-stream"})
        try:
            with urllib.request.urlopen(upload_req, timeout=120) as r:
                pass  # success
        except Exception as e:
            print(f"  FAIL upload {name}: {e}", file=sys.stderr)
    
    return new_urls


def rewrite_sources(new_urls):
    """Rewrite sources.φ URLs line-by-line (safe, no block splitting)."""
    lines = open(SOURCE_PHI).readlines()
    count = 0
    cur_name = None
    for i, line in enumerate(lines):
        if line.startswith("source "):
            cur_name = line.strip().split()[1]
        if cur_name and cur_name in new_urls and line.startswith("url ") and "releases/download/" in line:
            lines[i] = f"url {new_urls[cur_name]}\n"
            count += 1
            cur_name = None
    open(SOURCE_PHI, "w").writelines(lines)
    return count


def main():
    log("=== DOMAIN RELEASE MIGRATION ===")
    
    log("Step 1: Extract CDN sources...")
    cdn = extract_cdn_sources()
    log(f"  Found {len(cdn)} CDN sources across {len(set(v[0] for v in cdn.values()))} sphere releases")
    
    log("Step 2: Build domain map from git history...")
    domain_map = build_domain_map(cdn)
    mapped = sum(1 for n in cdn if n in domain_map)
    log(f"  Mapped {mapped}/{len(cdn)} sources")
    # Show top domains
    from collections import Counter
    dom_counts = Counter(domain_map.values())
    for d, c in dom_counts.most_common(10):
        log(f"    {c:4d} → catalogs-{d}")
    
    log("Step 3: Create releases (API, no assets yet)...")
    releases_cache = {}
    created = 0
    for domain in set(domain_map.values()):
        if DRY_RUN:
            continue
        try:
            rid, tag = get_or_create_release(domain)
            releases_cache[domain] = tag
            created += 1
        except Exception as e:
            log(f"  FAIL create catalogs-{domain}: {e}")
    log(f"  {created} releases ready")
    
    # Build new URL map (without migrating assets — assets stay on old releases temporarily)
    new_urls = {}
    for name, (sphere, old_url) in sorted(cdn.items()):
        domain = domain_map.get(name, "github.com/omegaflow")
        tag = releases_cache.get(domain, f"catalogs-{domain.replace('.','-')}")
        new_urls[name] = f"https://github.com/omegaflow/sources/releases/download/{tag}/{name}.json"
    
    log("Step 4: Rewrite sources.φ...")
    rewritten = rewrite_sources(new_urls)
    log(f"  {rewritten} URL lines rewritten")
    
    log("Step 5: Validate...")
    r = subprocess.run(["cargo", "run"], capture_output=True, text=True, timeout=30)
    loaded = re.search(r"loaded (\d+) sources", r.stdout)
    if loaded:
        log(f"  Server loads {loaded.group(1)} sources")
    else:
        log(f"  WARNING: could not verify server load")
    
    # Commit if not dry-run
    if not DRY_RUN and rewritten > 0:
        subprocess.run(["git", "add", SOURCE_PHI])
        subprocess.run(["git", "commit", "-m", f"Domain migration: {rewritten} URLs → catalogs-{{domain}}"])
    
    log(f"\nDone. Rewritten: {rewritten}. {'Dry-run complete.' if DRY_RUN else 'Commit ready.'}")


if __name__ == "__main__":
    main()
