#!/usr/bin/env python3
"""Recover original live API URLs for all sources from git history.
Used by migrate_live_to_cdn.py --mode regen-cdn to regenerate CDN assets.
"""
import re
import subprocess

SPHERES = ["astro", "geosphere", "hydrosphere", "atmosphere", "magnetosphere",
           "biosphere", "exosphere", "subatomic", "technosphere", "cryosphere"]


def _git(cmd):
    r = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
    return r.stdout


def _load_cdn_migration_urls():
    """Extract original live URLs from all CDN migration commit diffs."""
    live = {}
    log = _git(["git", "log", "--all", "--oneline", "--", "phi/sources.φ"])
    hashes = [l.split()[0] for l in log.split("\n") if l.strip()]
    for h in hashes:
        try:
            diff = _git(["git", "show", h, "--", "phi/sources.φ"])
        except Exception:
            continue
        cur = None
        for line in diff.split("\n"):
            if line.startswith(" source ") and len(line.strip().split()) >= 2:
                cur = line.strip().split()[1]
            elif line.startswith("-source "):
                cur = line.strip().split()[1]
            elif line.startswith("-url ") and cur:
                url = line[5:].strip()
                if "releases/download/" not in url:
                    live[cur] = url
    return live


def _load_precdn_urls():
    """Extract live URLs from pre-CDN states (de4b243 + 28c0b07~1 + 3a6a1a9~1)."""
    live = {}
    for ref in ["de4b243", "28c0b07~1", "3a6a1a9~1"]:
        try:
            content = _git(["git", "show", f"{ref}:phi/sources.φ"])
        except Exception:
            continue
        for b in re.split(r"\n(?=source )", content):
            m = re.match(r"source (\S+)", b)
            if not m:
                continue
            urls = re.findall(r"url\s+(\S+)", b)
            if urls and "releases/download/" not in urls[0]:
                live.setdefault(m.group(1), urls[0])
    return live


def _load_rename_map():
    """Build {new_name: old_name} from Provider->Sphere rename (e93c1a6)."""
    rename = {}
    diff = _git(["git", "show", "e93c1a6", "--", "phi/sources.φ"])
    prev = None
    for line in diff.split("\n"):
        if line.startswith("-source "):
            prev = line.strip().split()[1]
        elif line.startswith("+source "):
            new = line.strip().split()[1]
            if prev:
                rename[new] = prev
    return rename


def build_live_url_map():
    """Return {current_source_name: original_live_api_url} for all sources
    that can be recovered from git history."""
    live = {}
    live.update(_load_cdn_migration_urls())
    live.update(_load_precdn_urls())

    rename = _load_rename_map()

    # Map current names (with sphere prefixes + canonicalization) to recovered URLs
    final = {}
    all_names = set(live.keys()) | set(rename.keys())

    # Read current source names from sources.φ
    current_names = set()
    try:
        for line in open("phi/sources.φ"):
            if line.startswith("source "):
                current_names.add(line.strip().split()[1])
    except Exception:
        pass

    for name in current_names:
        # Direct
        if name in live:
            final[name] = live[name]
            continue
        # Via rename: current -> old
        if name in rename and rename[name] in live:
            final[name] = live[rename[name]]
            continue
        # Strip sphere prefix
        parts = name.split("_")
        found = False
        for sp in SPHERES:
            if parts[0] == sp and len(parts) >= 2:
                stripped = "_".join(parts[1:])
                if stripped in live:
                    final[name] = live[stripped]
                    found = True
                    break
                # Try rename of stripped
                if stripped in rename and rename[stripped] in live:
                    final[name] = live[rename[stripped]]
                    found = True
                    break
        if found:
            continue
        # Reverse canonical: intermagnet / main suffixes
        if len(parts) >= 3:
            if parts[1] == "intermagnet":
                stripped2 = f"{parts[0]}_{parts[2]}"
                if stripped2 in live:
                    final[name] = live[stripped2]
                    continue
            if parts[2] == "main":
                stripped2 = f"{parts[0]}_{parts[1]}"
                if stripped2 in live:
                    final[name] = live[stripped2]
                    continue
            if parts[1] in ("gml", "openmeteo", "nws", "usgs", "wikipedia", "swpc", "emsc", "gwis", "satnogs", "pdg"):
                stripped2 = f"{parts[0]}_{parts[2]}"
                if stripped2 in live:
                    final[name] = live[stripped2]

    return final


if __name__ == "__main__":
    m = build_live_url_map()
    print(f"Recovered {len(m)} original API URLs")
    for k in sorted(m.keys())[:10]:
        print(f"  {k}: {m[k][:80]}")
