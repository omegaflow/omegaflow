#!/usr/bin/env python3
"""Apply final domain corrections to sources.φ (363 sources).
Usage: python3 scripts/final_domain_fix.py [--dry-run]
"""
import re, sys

SOURCE_PHI = "phi/sources.φ"
MAP_FILE = "/tmp/opencode/final_domain_map.tsv"
DRY_RUN = "--dry-run" in sys.argv


def load_domain_map():
    """Return {source_name: correct_domain}."""
    result = {}
    with open(MAP_FILE) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split("\t")
            if len(parts) >= 2:
                result[parts[0]] = parts[1]
    return result


def domain_to_tag(domain):
    """Convert a domain string to a catalogs-{tag} suffix.
    github.com/owner/repo -> github.com-owner-repo"""
    return domain.replace("/", "-")


def apply_fixes(domain_map):
    """Rewrite sources.φ line-by-line. Safe — no block splitting."""
    lines = open(SOURCE_PHI).readlines()
    count = 0
    current_source = None
    updated = set()

    for i, line in enumerate(lines):
        if line.startswith("source "):
            current_source = line.strip().split()[1]
            continue

        if current_source is None:
            continue

        if current_source not in domain_map:
            continue

        if not line.startswith("url "):
            continue

        if "releases/download/" not in line:
            continue

        correct_domain = domain_map[current_source]
        # Don't double-fix
        if current_source in updated:
            continue

        old_domain = re.search(r"catalogs-([^/]+)/", line)
        if not old_domain:
            continue
        old_domain_tag = old_domain.group(1)

        new_tag = domain_to_tag(correct_domain)
        if old_domain_tag == new_tag:
            continue

        new_line = line.replace(f"catalogs-{old_domain_tag}", f"catalogs-{new_tag}")
        if new_line == line:
            continue

        if DRY_RUN:
            print(f"  {current_source}: catalogs-{old_domain_tag} -> catalogs-{new_tag}")
            count += 1
            updated.add(current_source)
            continue

        lines[i] = new_line
        count += 1
        updated.add(current_source)
        current_source = None

    if not DRY_RUN and count > 0:
        open(SOURCE_PHI, "w").writelines(lines)

    return count


def main():
    domain_map = load_domain_map()
    print(f"Loaded {len(domain_map)} corrections", file=sys.stderr)

    count = apply_fixes(domain_map)
    print(f"{'Would fix' if DRY_RUN else 'Fixed'} {count} URLs", file=sys.stderr)

    # Verify no sphere tags remain
    content = open(SOURCE_PHI).read()
    sphere_tags = re.findall(r"catalogs-(astro|geosphere|subatomic|atmosphere|magnetosphere|biosphere|exosphere|technosphere|hydrosphere)", content)
    if sphere_tags:
        print(f"WARNING: {len(sphere_tags)} sphere tags remain", file=sys.stderr)
    else:
        print("All sphere tags resolved", file=sys.stderr)

    kp_count = len(re.findall(r"catalogs-kp\.gfz\.de", content))
    if kp_count:
        print(f"WARNING: {kp_count} kp.gfz.de tags remain", file=sys.stderr)

    fcc_count = len(re.findall(r"catalogs-github\.com-FreeCodeCamp", content))
    if fcc_count:
        print(f"WARNING: {fcc_count} FreeCodeCamp tags remain", file=sys.stderr)


if __name__ == "__main__":
    main()
