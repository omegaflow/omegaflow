#!/usr/bin/env python3
"""Canonicalize source naming: every source must have {sphere}_{provider}_{detail}.
Types: A (token2=provider, needs detail) → add _main suffix
       B (token2=data_type, needs provider) → insert provider from domain
       C (INTERMAGNET stations) → insert intermagnet provider
Usage: python3 scripts/canonicalize_names.py [--dry-run]
"""
import re, sys

SOURCE_PHI = "phi/sources.φ"
DRY_RUN = "--dry-run" in sys.argv
DOMAIN_PROVIDER = {
    "gml.noaa.gov": "gml",
    "open-meteo.com": "openmeteo",
    "weather.gov": "nws",
    "earthquake.usgs.gov": "usgs",
    "en.wikipedia.org": "wikipedia",
    "services.swpc.noaa.gov": "swpc",
    "seismicportal.eu": "emsc",
    "gwis.jrc.ec.europa.eu": "gwis",
    "network.satnogs.org": "satnogs",
    "github.com-scikit-hep-particle": "pdg",
}
TYPE_A_DETAIL = {
    "astro_heasarc": "main",
    "astro_irsa": "main",
    "astro_simbad": "main",
    "astro_tevcat": "main",
    "astro_vizier": "main",
    "astro_ned": "main",
    "astro_cadc": "main",
    "astro_esasky": "main",
    "biosphere_gbif": "occurrences",
    "biosphere_inaturalist": "observations",
    "magnetosphere_goes": "particles",
    "exosphere_magnetometer": "stations",
    "geosphere_geopages": "list",
}

INTERMAGNET_PREFIX = "magnetosphere"
INTERMAGNET_CODES = {
    "abg", "abk", "aia", "ars", "asp", "bdv", "bel", "bfo", "bou", "brw",
    "bsl", "cki", "clf", "cmo", "cnb", "cpL", "csy", "cta", "cyg", "ded",
    "dlt", "dou", "dur", "ebr", "eyr", "frd", "frn", "fur", "gan", "gck",
    "gdh", "gng", "gua", "gui", "hbk", "her", "hlp", "hon", "hrb", "hrn",
    "hua", "hyb", "ipm", "irt", "izn", "jai", "kak", "kdu", "kir", "kmh",
    "kny", "kou", "lvv", "lyc", "mab", "maw", "mcqg", "mmb", "nag", "nck",
    "new", "ngk", "nur", "nvs", "orc", "pag", "peg", "phu", "pil", "ppt",
    "reu", "sba", "sfs", "she", "shu", "sit", "sjg", "sod", "spt", "sua",
    "thL", "thy", "tirm", "tlon", "tsu", "ttb", "tuc", "ups", "valL",
    "vna", "vos", "vss", "wic", "wng",
}
def build_rename_map():
    """Parse sources.φ and build {old_name: new_name} for all 2-token sources."""
    renames = {}
    lines = open(SOURCE_PHI).readlines()
    cur_source = None
    cur_domain = None
    
    for line in lines:
        if line.startswith("source "):
            cur_source = line.strip().split()[1]
            cur_domain = None
            continue
        
        if not cur_source:
            continue
        

        if line.startswith("url ") and "releases/download/" in line:
            m = re.search(r"releases/download/([^/]+)/(.+)", line)
            if m:
                cur_domain = m.group(1)
        

        if line.startswith("url ") and "releases/download/" not in line and "://" in line:
            m = re.search(r"://([^/\s]+)", line)
            if m:
                cur_domain = m.group(1)
                if cur_domain.startswith("www."):
                    cur_domain = cur_domain[4:]
        
        if cur_domain is None:
            continue
        
        tokens = cur_source.split("_")
        if len(tokens) != 2:
            cur_source = None
            continue
        
        sphere, t2 = tokens[0], tokens[1]
        new_name = None
        

        if sphere == INTERMAGNET_PREFIX and (t2.lower() in {c.lower() for c in INTERMAGNET_CODES} or t2 in INTERMAGNET_CODES):
            new_name = f"{sphere}_intermagnet_{t2}"

        elif cur_domain in DOMAIN_PROVIDER:
            provider = DOMAIN_PROVIDER[cur_domain]
            new_name = f"{sphere}_{provider}_{t2}"

        elif cur_source in TYPE_A_DETAIL:
            detail = TYPE_A_DETAIL[cur_source]
            new_name = f"{sphere}_{t2}_{detail}"
        else:
            domain_prov = cur_domain.split(".")[0] if "." in cur_domain else cur_domain
            new_name = f"{sphere}_{domain_prov}_{t2}"
        
        if new_name and new_name != cur_source:
            renames[cur_source] = new_name
        
        cur_source = None
    
    return renames
def apply_renames(rename_map, dry=False):
    """Rewrite sources.φ line-by-line, renaming source blocks and CDN filenames."""
    lines = open(SOURCE_PHI).readlines()
    count = 0
    errors = 0
    
    for i in range(len(lines)):
        line = lines[i]
        
        for old, new in rename_map.items():
            if old in line:
                new_line = line.replace(old, new)
                if new_line != line:
                    if dry:
                        print(f"  {old} → {new}")
                        count += 1
                    else:
                        lines[i] = new_line
                        count += 1
                    break
    
    if not dry and count > 0:
        open(SOURCE_PHI, "w").writelines(lines)
    
    return count, errors
def main():
    rename_map = build_rename_map()
    print(f"Rename map: {len(rename_map)} sources", file=sys.stderr)
    

    intermagnet = {k: v for k, v in rename_map.items() if k.startswith("magnetosphere_") and "intermagnet" in v}
    type_b = {k: v for k, v in rename_map.items() if k not in intermagnet and k not in TYPE_A_DETAIL}
    type_a = {k: v for k, v in rename_map.items() if k in TYPE_A_DETAIL}
    
    print(f"  Type C (INTERMAGNET): {len(intermagnet)}", file=sys.stderr)
    for k in sorted(intermagnet.keys())[:5]:
        print(f"    {k} → {intermagnet[k]}", file=sys.stderr)
    
    print(f"  Type A (provider→detail): {len(type_a)}", file=sys.stderr)
    for k in sorted(type_a.keys()):
        print(f"    {k} → {type_a[k]}", file=sys.stderr)
    
    print(f"  Type B (insert provider): {len(type_b)}", file=sys.stderr)
    for k in sorted(type_b.keys()):
        print(f"    {k} → {type_b[k]}", file=sys.stderr)
    
    count, _ = apply_renames(rename_map, dry=DRY_RUN)
    action = "Would rename" if DRY_RUN else "Renamed"
    print(f"\n{action} {count} occurrences", file=sys.stderr)
    
    if not DRY_RUN and count > 0:

        remaining = 0
        for line in open(SOURCE_PHI):
            if line.startswith("source "):
                name = line.strip().split()[1]
                if len(name.split("_")) == 2:
                    remaining += 1
                    print(f"  REMAINING: {name}", file=sys.stderr)
        print(f"Remaining 2-token sources: {remaining}", file=sys.stderr)
if __name__ == "__main__":
    main()
