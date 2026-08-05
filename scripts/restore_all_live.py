#!/usr/bin/env python3
"""Restore all short-TTL live URLs from the pre-CDN state (commit de4b243)."""
import re, subprocess, sys

SOURCE_PHI = "phi/sources.φ"
DRY_RUN = "--dry-run" in sys.argv


def load_live_urls():
    """Return {source_name: live_url} from pre-CDN state de4b243."""
    r = subprocess.run(
        ["git", "show", "de4b243:phi/sources.φ"],
        capture_output=True, text=True, timeout=30
    )
    blocks = re.split(r"\n(?=source )", r.stdout)
    
    live_urls = {}
    spheres = ["astro", "geosphere", "hydrosphere", "atmosphere", "magnetosphere",
               "biosphere", "exosphere", "subatomic", "technosphere", "cryosphere"]
    
    for b in blocks:
        m = re.match(r"source (\S+)", b)
        if not m:
            continue
        old_name = m.group(1)
        urls = re.findall(r"url\s+(https?://[^\s]+)", b)
        if not urls:
            continue
        url = urls[0]
        if "releases/download/" in url:
            continue
        
        candidates = [old_name]
        for s in spheres:
            candidates.append(f"{s}_{old_name}")
        
        for c in candidates:
            live_urls[c] = url
    
    return live_urls


def main():
    live_urls = load_live_urls()
    print(f"Live URLs from pre-CDN state: {len(live_urls)}", file=sys.stderr)
    
    lines = open(SOURCE_PHI).readlines()
    cur_source = None
    cur_ttl = None
    fix_count = 0
    missing_count = 0
    
    for i in range(len(lines)):
        line = lines[i]
        
        if line.startswith("source "):
            cur_source = line.strip().split()[1]
            cur_ttl = None
            continue
        
        if line.startswith("ttl "):
            try:
                cur_ttl = int(line.strip().split()[1])
            except ValueError:
                pass
            continue
        
        if not line.startswith("url "):
            continue
        
        if "releases/download/" not in line:
            continue
        
        if cur_ttl is None or cur_ttl >= 300:
            continue
        
        if cur_source not in live_urls:
            print(f"  MISSING: {cur_source} (ttl={cur_ttl})", file=sys.stderr)
            missing_count += 1
            continue
        
        live_url = live_urls[cur_source]
        
        if DRY_RUN:
            print(f"  {cur_source}: → {live_url[:90]}")
        else:
            lines[i] = f"url {live_url}\n"
        fix_count += 1
    
    if not DRY_RUN and fix_count > 0:
        open(SOURCE_PHI, "w").writelines(lines)
    
    print(f"\n{'Would fix' if DRY_RUN else 'Fixed'}: {fix_count}, Missing: {missing_count}", file=sys.stderr)


if __name__ == "__main__":
    main()
