#!/usr/bin/env python3
"""Repair broken Gaia CDN files by re-running TAP queries against ESA Gaia archive.
Uploads regenerated files to gea.esac.esa.int release.
Usage: OMEGAFLOW_TOKEN=ghp_xxx python3 scripts/repair_gaia.py [--dry-run]
"""
import json
import os
import re
import sys
import time
import urllib.request
import urllib.error
import urllib.parse

TOKEN = os.environ.get("OMEGAFLOW_TOKEN", "")
if not TOKEN:
    TOKEN = open(".secrets.local").readlines()
    TOKEN = [l.split("=",1)[1].strip() for l in TOKEN if l.startswith("OMEGAFLOW_TOKEN=")][0]

DRY_RUN = "--dry-run" in sys.argv
ESA_TAP = "https://gea.esac.esa.int/tap-server/tap/sync"
UA = "omegaflow-gaia-repair/1.0"
RELEASE_DOMAIN = "gea.esac.esa.int"
RELEASE_REPO = "omegaflow/sources"
DEFAULT_LIMIT = 200000

# Source definition -> TAP query
# Format: (table, columns, where clause)
GAIA_QUERIES = {
    "astro_gaia_astrometric_binaries": {
        "table": "gaiadr3.binary_masses",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "parallax > 0",
        "limit": 10000,
    },
    "astro_gaia_blue_stragglers": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp",
        "where": "bp_rp < 0.3 AND phot_g_mean_mag < 18 AND parallax > 0",
    },
    "astro_gaia_bright_giants": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp",
        "where": "phot_g_mean_mag < 6 AND parallax > 0 AND bp_rp > 1.0",
    },
    "astro_gaia_bulge": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "CONTAINS(POINT('ICRS',ra,dec),CIRCLE('ICRS',266.4,-29.0,5.0))=1 AND parallax > 0",
    },
    "astro_gaia_cool_dwarfs": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp,teff_gspphot",
        "where": "teff_gspphot < 4000 AND phot_g_mean_mag < 18 AND parallax > 0",
    },
    "astro_gaia_deep_mag": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "phot_g_mean_mag > 20 AND parallax > 0",
    },
    "astro_gaia_extreme_blue": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp",
        "where": "bp_rp < 0.0 AND phot_g_mean_mag < 18 AND parallax > 0",
    },
    "astro_gaia_extreme_red": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp",
        "where": "bp_rp > 3.0 AND phot_g_mean_mag < 18 AND parallax > 0",
    },
    "astro_gaia_galactic_plane": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "ABS(dec) < 5 AND parallax > 0",
    },
    "astro_gaia_giant_branch": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp",
        "where": "bp_rp > 1.0 AND phot_g_mean_mag < 12 AND parallax > 0",
    },
    "astro_gaia_high_velocity": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,pmra,pmdec,phot_g_mean_mag,radial_velocity",
        "where": "SQRT(POWER(pmra,2)+POWER(pmdec,2)) > 500 AND parallax > 0",
    },
    "astro_gaia_hot_subdwarfs": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp,teff_gspphot",
        "where": "teff_gspphot > 30000 AND phot_g_mean_mag < 18 AND parallax > 0",
    },
    "astro_gaia_lmc": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "CONTAINS(POINT('ICRS',ra,dec),CIRCLE('ICRS',80.9,-69.8,5.0))=1 AND parallax > 0",
    },
    "astro_gaia_main_sequence": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp",
        "where": "bp_rp BETWEEN 0.3 AND 1.0 AND phot_g_mean_mag < 15 AND parallax > 0",
    },
    "astro_gaia_metallicity": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,mh_gspphot",
        "where": "mh_gspphot IS NOT NULL AND parallax > 0",
    },
    "astro_gaia_nearby_1000": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,pmra,pmdec,radial_velocity,phot_g_mean_mag,bp_rp",
        "where": "parallax > 10",
    },
    "astro_gaia_nearby_cmap": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,pmra,pmdec,radial_velocity,phot_g_mean_mag,bp_rp,teff_gspphot",
        "where": "parallax > 5",
    },
    "astro_gaia_nearby_plx10": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,pmra,pmdec,phot_g_mean_mag,radial_velocity",
        "where": "parallax > 10",
    },
    "astro_gaia_red_clump": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,bp_rp",
        "where": "bp_rp BETWEEN 1.0 AND 1.5 AND phot_g_mean_mag < 15 AND parallax > 0",
    },
    "astro_gaia_smc": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "CONTAINS(POINT('ICRS',ra,dec),CIRCLE('ICRS',13.2,-72.8,3.0))=1 AND parallax > 0",
    },
    "astro_gaia_binaries": {
        "table": "gaiadr3.nss_two_body_orbit",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,period",
        "where": "parallax > 0",
    },
    "astro_gaia_metal_poor": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag,mh_gspphot",
        "where": "mh_gspphot < -2.0 AND parallax > 0",
    },
    "exosphere_gaia_nearby_stars": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "parallax > 100",
    },
    "exosphere_gaia_stars": {
        "table": "gaiadr3.gaia_source",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "parallax > 5 AND phot_g_mean_mag < 15",
    },
    "exosphere_gaia_variable_stars": {
        "table": "gaiadr3.vari_summary",
        "columns": "source_id,ra,dec,parallax,phot_g_mean_mag",
        "where": "parallax > 0",
        "limit": 50000,
    },
}

def query_esa_tap(table, columns, where_clause, limit=DEFAULT_LIMIT):
    query = f"SELECT TOP {limit} {columns} FROM {table}"
    if where_clause:
        query += f" WHERE {where_clause}"
    
    url = f"{ESA_TAP}?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY={urllib.parse.quote_plus(query)}"
    print(f"  Querying: {table}...", end=" ", flush=True)
    
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    try:
        with urllib.request.urlopen(req, timeout=180) as r:
            data = json.loads(r.read())
        rows = data.get("data", [])
        columns_meta = [c["name"] for c in data.get("metadata", [])]
        
        result = []
        for row in rows:
            obj = {}
            for i, col in enumerate(columns_meta):
                if i < len(row):
                    obj[col] = row[i]
            result.append(obj)
        
        print(f"{len(result)} rows")
        return result
    except Exception as e:
        print(f"ERROR: {e}")
        return None

def api_call(method, path, data=None):
    url = f"https://api.github.com{path}"
    headers = {"Authorization": f"Bearer {TOKEN}", "User-Agent": UA,
               "Accept": "application/vnd.github+json"}
    if data:
        headers["Content-Type"] = "application/json"
        req = urllib.request.Request(url, data=data.encode(), headers=headers, method=method)
    else:
        req = urllib.request.Request(url, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=30) as r:
            return r.status, json.loads(r.read()) if r.status != 204 else None
    except urllib.error.HTTPError as e:
        return e.code, json.loads(e.read()) if e.fp else None

def upload_to_release(filename, data_bytes):
    if DRY_RUN:
        print(f"    [DRY] upload {filename} ({len(data_bytes)} bytes)")
        return True
    
    status, release = api_call("GET", f"/repos/{RELEASE_REPO}/releases/tags/{RELEASE_DOMAIN}")
    if status != 200 or release is None:
        payload = json.dumps({"tag_name": RELEASE_DOMAIN, "name": RELEASE_DOMAIN, "body": ""})
        status, release = api_call("POST", f"/repos/{RELEASE_REPO}/releases", payload)
        if status not in (200, 201):
            print(f"    Release error: {status}")
            return False
    
    if not release:
        print(f"    Release error: no data")
        return False
    
    upload_url = release["upload_url"].split("{")[0]
    url = f"{upload_url}?name={urllib.parse.quote(filename)}"
    req = urllib.request.Request(url, data=data_bytes,
        headers={"Authorization": f"Bearer {TOKEN}", "User-Agent": UA,
                 "Content-Type": "application/octet-stream"}, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            ok = r.status == 201
            if ok: print(f"    uploaded ✅")
            return ok
    except Exception as e:
        print(f"    upload error: {e}")
        return False

def main():
    to_repair = list(GAIA_QUERIES.keys())
    print(f"Repairing {len(to_repair)} Gaia sources\n")
    
    repaired = 0
    failed = 0
    
    for source_name in to_repair:
        q = GAIA_QUERIES[source_name]
        print(f"\n--- {source_name} ---")
        
        rows = query_esa_tap(q["table"], q["columns"], q["where"], 
                            q.get("limit", DEFAULT_LIMIT))
        
        if rows is None:
            failed += 1
            continue
        if len(rows) == 0:
            print(f"    0 rows — skipping")
            continue
        
        flat = json.dumps({"data": rows}).encode()
        asset_name = f"{source_name}.json"
        
        if upload_to_release(asset_name, flat):
            repaired += 1
        else:
            failed += 1
        
        time.sleep(2)  # Rate limit
    
    print(f"\n=== Done: {repaired} repaired, {failed} failed ===")

if __name__ == "__main__":
    main()
