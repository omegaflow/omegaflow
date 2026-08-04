#!/usr/bin/env python3
"""Restore live API URLs for short-TTL sources (TTL < 300s).
Finds original live URLs from git CDN migration history and writes them back.
Usage: python3 scripts/restore_live_urls.py [--dry-run]
"""
import re, subprocess, sys

SOURCE_PHI = "phi/sources.φ"
DRY_RUN = "--dry-run" in sys.argv
MAX_TTL = 300


def load_live_urls_from_git():
    """Extract all original live URLs from CDN migration commit diffs."""
    r = subprocess.run(
        ["git", "log", "--all", "--oneline", "--", SOURCE_PHI],
        capture_output=True, text=True, timeout=30
    )
    hashes = [line.split()[0] for line in r.stdout.split("\n") if "CDN: migrate" in line]
    
    live_urls = {}
    for h in hashes:
        r = subprocess.run(
            ["git", "show", h, "--", SOURCE_PHI],
            capture_output=True, text=True, timeout=30
        )
        lines = r.stdout.split("\n")
        cur_src = None
        for i, line in enumerate(lines):
            # Hunk header gives source context
            if line.startswith("@@ "):
                m = re.search(r"@@\s+.*@@\s+(.*)", line)
                if m:
                    func = m.group(1).strip()
                    if func and func != "source":
                        cur_src = func if " " not in func else func.split()[0]
                else:
                    cur_src = None
                continue
            # Context line with source name
            if line.startswith(" source ") and len(line.strip().split()) >= 2:
                cur_src = line.strip().split()[1]
            # Removed source line
            elif line.startswith("-source "):
                cur_src = line.strip().split()[1]
            # Removed URL line
            elif line.startswith("-url "):
                url = line[5:].strip()
                if "releases/download/" not in url and cur_src:
                    live_urls[cur_src] = url
    
    return live_urls


def build_rename_map():
    """Build {current_name: old_name} from Provider→Sphere rename (e93c1a6)."""
    renames = {}
    r = subprocess.run(
        ["git", "show", "e93c1a6", "--", SOURCE_PHI],
        capture_output=True, text=True, timeout=30
    )
    for line in r.stdout.split("\n"):
        if line.startswith("-source "):
            old = line.strip().split()[1]
            renames[old] = old  # track old names
        elif line.startswith("+source "):
            new = line.strip().split()[1]
            # Find matching old name: previous -source line
            for old_name in list(renames.keys()):
                if renames.get(old_name) == old_name and old_name != new:
                    renames[old_name] = new
    
    # Build reverse: current -> [old_candidates]
    reverse = {}
    for old, new in renames.items():
        if new not in reverse:
            reverse[new] = []
        reverse[new].append(old)
    return reverse


def find_live_url(source_name, live_urls, rename_map):
    """Find the original live URL for a source."""
    candidates = [source_name]
    
    # Try rename reverse
    if source_name in rename_map:
        candidates.extend(rename_map[source_name])
    
    # Try stripping sphere prefix
    spheres = ["astro", "geosphere", "hydrosphere", "atmosphere", "magnetosphere",
               "biosphere", "exosphere", "subatomic", "technosphere", "cryosphere"]
    parts = source_name.split("_")
    if len(parts) >= 2 and parts[0] in spheres:
        stripped = "_".join(parts[1:])
        candidates.append(stripped)
    
    for c in candidates:
        if c in live_urls:
            return live_urls[c]
    
    return None


def main():
    live_urls = load_live_urls_from_git()
    rename_map = build_rename_map()
    print(f"Live URLs from git: {len(live_urls)}", file=sys.stderr)
    print(f"Rename pairs: {len(rename_map)}", file=sys.stderr)
    
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
        
        if cur_ttl is None or cur_ttl >= MAX_TTL:
            continue
        
        # This source has short TTL but is on CDN — fix it
        live_url = find_live_url(cur_source, live_urls, rename_map)
        
        if live_url:
            if DRY_RUN:
                print(f"  {cur_source}: CDN → {live_url[:80]}")
            else:
                lines[i] = f"url {live_url}\n"
            fix_count += 1
        else:
            print(f"  MISSING: {cur_source} (ttl={cur_ttl})", file=sys.stderr)
            missing_count += 1
    
    if not DRY_RUN and fix_count > 0:
        open(SOURCE_PHI, "w").writelines(lines)
    
    print(f"\n{'Would fix' if DRY_RUN else 'Fixed'}: {fix_count}, Missing: {missing_count}", file=sys.stderr)


if __name__ == "__main__":
    main()
