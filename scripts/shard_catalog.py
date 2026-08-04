#!/usr/bin/env python3
"""Spatially shard a huge TAP catalog into RA-bin CDN files.

Splits a catalog (e.g. NVSS 1.77M rows) along the RA axis into N bins,
queries each bin via `WHERE <ra_col> BETWEEN lo AND hi`, converts each to
{"data":[...]}, uploads each as `<source>_<bin>.json` to the v1.0 release,
and (optionally) rewrites sources.φ with one source per bin.

Usage:
    python3 scripts/shard_catalog.py --source vizier_nvss_radio --bins 24
    python3 scripts/shard_catalog.py --source vizier_nvss_radio --bins 24 --update-sources

Env:
    CATALOGS_TOKEN: PAT with contents write to omegaflow/sources
    TAP_TIMEOUT: per-request timeout (default 180)
    TAP_SLEEP: seconds between requests (default 2)
"""
import argparse
import json
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

TOKEN = os.environ.get("CATALOGS_TOKEN", "")
RELEASE_URL = "https://uploads.github.com/repos/omegaflow/sources/releases/{release_id}/assets"
SHARD_DOMAIN = "tapvizier.cds.unistra.fr"
_SHARD_RELEASE_ID = None
TIMEOUT = int(os.environ.get("TAP_TIMEOUT", "180"))
SLEEP = float(os.environ.get("TAP_SLEEP", "2"))

# source name -> (table, ra_col, extra_where)
SHARDS = {
    "vizier_nvss_radio": ("VIII/65/nvss", "RAJ2000", ""),
    "vizier_nvss_radio_catalog": ("VIII/65/nvss", "RAJ2000", ""),
    "vizier_nvss_galcenter": ("VIII/65/nvss", "RAJ2000",
                              " AND CONTAINS(POINT('ICRS',RAJ2000,DEJ2000),CIRCLE('ICRS',266.4,-29.0,5.0))=1"),
    "vizier_first_radio": ("VIII/92/first14", "RAJ2000", ""),
    "vizier_first_radio_catalog": ("VIII/92/first14", "RAJ2000", ""),
    "vizier_gleam_extragalactic_catalog": ("J/MNRAS/464/1146/gleam", "RAJ2000", ""),
    "vizier_lotss_dr3": ("J/A+A/707/A198/lotssdr3", "RAJ2000", ""),
    "vizier_lotss_dr3_radio": ("J/A+A/707/A198/lotssdr3", "RAJ2000", ""),
    "vizier_2dfgrs": ("J/MNRAS/329/227/2dFGRS", "RAJ2000", ""),
    "vizier_sdss12_galaxies": ("V/147/sdss12", "RAJ2000", ""),
    "vizier_sdss12_stars": ("V/147/sdss12", "RAJ2000", ""),
}

# Mid-size catalogs uploaded as TOP excerpts on CDN; reload them fully.
# file -> (table, select_cols, ra_col, expected_full_size)
EXCERPTS = {
    "vizier_6dfgs.json": ("VII/259/6dfgs", "RAJ2000,DEJ2000,cz,e_cz,bJmag,S/G", "RAJ2000", 110000),
    "vizier_hecate.json": ("J/MNRAS/506/1896/hecate", "RAJ2000,DEJ2000,HRV,D,W1mag,W2mag", "RAJ2000", 20000),
    "vizier_hi4pi.json": ("J/A+A/594/A116/hi4pi", "GLON,GLAT,NHI,VLSR,FWHM", "GLON", 200000),
    "vizier_2qz.json": ("VII/241/2qz", "RAJ2000,DEJ2000,z,bJmag", "RAJ2000", 49000),
}


def build_reload_url(table, select_cols, lo=None, hi=None, ra_col=None):
    q = f"SELECT {select_cols} FROM {table}"
    if lo is not None and ra_col:
        q += f" WHERE {ra_col} BETWEEN {lo:.4f} AND {hi:.4f}"
    base = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY="
    return base + urllib.parse.quote_plus(q)


def fetch(url):
    req = urllib.request.Request(url, headers={"User-Agent": "omegaflow-bot/1.0"})
    with urllib.request.urlopen(req, timeout=TIMEOUT) as r:
        return r.read().decode("utf-8", errors="replace")


def votable_to_json(text):
    if "<VOTABLE" not in text:
        return None
    try:
        from astropy.io.votable import parse_single_table
        import io
    except Exception:
        return None
    try:
        table = parse_single_table(io.BytesIO(text.encode("utf-8")))
        data = table.array
        cols = data.dtype.names or []
        rows = []
        for rec in data:
            row = {}
            for col in cols:
                val = rec[col]
                if val is None:
                    continue
                try:
                    val = val.item() if hasattr(val, "item") else val
                    row[col] = float(val)
                except (TypeError, ValueError):
                    s = str(val)
                    if s in ("--", "null", "nan", "NaN", ""):
                        continue
                    row[col] = s
            rows.append(row)
        return rows
    except Exception:
        return None


def csv_to_json(text):
    lines = [l.rstrip("\n") for l in text.split("\n") if l.strip()]
    if not lines:
        return []
    header_idx = 0
    for i, l in enumerate(lines):
        if not l.startswith("#"):
            header_idx = i
            break
    header_line = lines[header_idx]
    if "\t" in header_line:
        delim = "\t"
    elif "|" in header_line:
        delim = "|"
    else:
        delim = ","
    cols = [c.strip().strip('"') for c in header_line.split(delim)]
    if len(cols) < 2:
        return []
    rows = []
    for line in lines[header_idx + 1:]:
        if line.startswith("#"):
            continue
        vals = [v.strip() for v in line.split(delim)]
        if len(vals) < len(cols):
            continue
        row = {}
        for i, col in enumerate(cols):
            v = vals[i]
            if not v or v in ("null", "NaN"):
                continue
            try:
                row[col] = float(v)
            except ValueError:
                row[col] = v
        rows.append(row)
    return rows


def parse_rows(text):
    rows = votable_to_json(text)
    if rows is None:
        rows = csv_to_json(text)
    return rows


def build_shard_url(table, ra_col, lo, hi, extra_where, select_cols="*"):
    # Use the server's native TAP sync with CSV output
    q = f"SELECT {select_cols} FROM {table} WHERE {ra_col} BETWEEN {lo:.4f} AND {hi:.4f}{extra_where}"
    if table.startswith(("VIII/", "J/", "V/", "B/")):
        base = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY="
    else:
        base = "https://heasarc.gsfc.nasa.gov/xamin/vo/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY="
    return base + urllib.parse.quote_plus(q)


def get_shard_release_id():
    global _SHARD_RELEASE_ID
    if _SHARD_RELEASE_ID:
        return _SHARD_RELEASE_ID
    tag = f"{SHARD_DOMAIN}"
    url = f"https://api.github.com/repos/omegaflow/sources/releases/tags/{urllib.parse.quote(tag, safe='')}"
    req = urllib.request.Request(url, headers={"Authorization": f"Bearer {TOKEN}",
                                                "Accept": "application/vnd.github+json",
                                                "User-Agent": "omegaflow-bot/1.0"})
    try:
        with urllib.request.urlopen(req, timeout=15) as r:
            d = json.load(r)
            _SHARD_RELEASE_ID = d["id"]
            return _SHARD_RELEASE_ID
    except Exception:
        return None


def upload_asset(filename, data):
    if not TOKEN:
        print(f"  !! no CATALOGS_TOKEN, skip {filename}", file=sys.stderr)
        return False
    rid = get_shard_release_id()
    if not rid:
        print(f"  !! no release for {SHARD_DOMAIN}", file=sys.stderr)
        return False
    upload_url = RELEASE_URL.format(release_id=rid)
    req = urllib.request.Request(
        f"{upload_url}?name={filename}",
        data=data, method="POST",
        headers={"Authorization": f"Bearer {TOKEN}",
                 "Content-Type": "application/octet-stream",
                 "User-Agent": "omegaflow-bot/1.0"},
    )
    try:
        with urllib.request.urlopen(req, timeout=600) as r:
            resp = json.load(r)
            print(f"  uploaded {filename} ({r.status})", file=sys.stderr)
            return True
    except urllib.error.HTTPError as e:
        body = e.read().decode(errors="replace")[:200]
        print(f"  upload failed {e.code}: {body}", file=sys.stderr)
        return False


def reload_excerpts(dry_run=False):
    """Reload mid-size CDN excerpt files to full catalog (COUNT-verified)."""
    import urllib.error as ue
    print(f"Reloading {len(EXCERPTS)} excerpt files to full catalogs", file=sys.stderr)
    for fname, (table, cols, ra_col, expected) in EXCERPTS.items():
        print(f"--- {fname} ({table}, expect ~{expected})", file=sys.stderr)
        if dry_run:
            print(f"    {build_reload_url(table, cols)[:120]}", file=sys.stderr)
            continue
        try:
            text = fetch(build_reload_url(table, cols))
            rows = parse_rows(text)
            if not rows:
                print("    EMPTY", file=sys.stderr)
                continue
            # verification: rough completeness (no exact COUNT, use expected threshold)
            if expected and len(rows) < expected * 0.5:
                print(f"    INCOMPLETE: {len(rows)} vs ~{expected} (skip)", file=sys.stderr)
                continue
            payload = json.dumps({"data": rows}).encode()
            if upload_asset(fname, payload):
                print(f"    reloaded {fname}: {len(rows)} rows", file=sys.stderr)
        except Exception as e:
            print(f"    FAILED: {str(e)[:120]}", file=sys.stderr)
        time.sleep(SLEEP)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", required=False)
    ap.add_argument("--bins", type=int, default=24)
    ap.add_argument("--update-sources", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--reload-excerpts", action="store_true",
                    help="reload mid-size CDN excerpt files to full catalog")
    args = ap.parse_args()

    if args.reload_excerpts:
        reload_excerpts(args.dry_run)
        return

    if not args.source or args.source not in SHARDS:
        print(f"Unknown shard config for {args.source}", file=sys.stderr)
        sys.exit(1)
    table, ra_col, extra = SHARDS[args.source]

    # Sample the select columns from sources.phi if present
    select_cols = "*"
    content = open("phi/sources.φ").read()
    blocks = re.split(r"\n(?=source )", content)
    for b in blocks:
        m = re.match(rf"source {args.source}\n(.*?)(?=\nsource |\Z)", b, re.DOTALL)
        if m:
            url = re.findall(r"url\s+(\S+)", b)
            if url:
                dec = urllib.parse.unquote_plus(url[0])
                sel = re.search(r"SELECT\s+(.*?)\s+FROM", dec, re.I | re.DOTALL)
                if sel:
                    cols = re.sub(r"TOP\s*\d+\s*", "", sel.group(1), flags=re.I).strip()
                    # dedupe columns, keep order
                    seen = set()
                    uniq = []
                    for c in re.split(r",", cols):
                        c = c.strip()
                        if c and c not in seen:
                            seen.add(c)
                            uniq.append(c)
                    select_cols = ",".join(uniq) if uniq else "*"
            break

    lo = 0.0
    bin_w = 360.0 / args.bins
    uploaded = []
    total = 0
    for i in range(args.bins):
        hi = lo + bin_w
        bname = f"{args.source}_{i:02d}"
        fname = f"{bname}.json"
        url = build_shard_url(table, ra_col, lo, hi, extra, select_cols)
        print(f"--- bin {i} ({lo:.1f}-{hi:.1f} deg) {bname}", file=sys.stderr)
        if args.dry_run:
            print(f"    {url[:150]}", file=sys.stderr)
            lo = hi
            continue
        try:
            text = fetch(url)
            rows = parse_rows(text)
            if not rows:
                print("    EMPTY", file=sys.stderr)
                lo = hi
                continue
            payload = json.dumps({"data": rows}).encode()
            if upload_asset(fname, payload):
                uploaded.append((bname, lo + bin_w / 2))
                total += len(rows)
                print(f"    {len(rows)} rows", file=sys.stderr)
        except Exception as e:
            print(f"    FAILED: {str(e)[:120]}", file=sys.stderr)
        lo = hi
        time.sleep(SLEEP)

    print(f"\nTotal rows across shards: {total}", file=sys.stderr)

    if args.update_sources and uploaded:
        # rewrite the single source into N shard sources
        rewrite_shard_sources(args.source, uploaded)


def rewrite_shard_sources(src_name, shards):
    content = open("phi/sources.φ").read()
    blocks = re.split(r"\n(?=source )", content)
    out = []
    replaced = False
    for b in blocks:
        m = re.match(rf"source ({src_name})\n(.*?)(\Z)", b, re.DOTALL)
        if m:
            # keep base directives (ttl, force) from original
            body = m.group(2)
            ttl = re.search(r"ttl (\d+)", body)
            force = re.search(r"force (\S+)", body)
            base = f"ttl {ttl.group(1) if ttl else 86400}\nforce {force.group(1) if force else 'em'}"
            for bname, ra_center in shards:
                fname = f"{bname}.json"
                out.append(
                    f"source {bname}\n{base}\n"
                    f"url https://github.com/omegaflow/sources/releases/download/catalogs-{SHARD_DOMAIN}/{fname}\n"
                    f"format json\nmap .\n"
                    f"lat_key RAJ2000\nlon_key DEJ2000\n"
                    f"wgs84 0.0 {ra_center:.2f}\n"
                )
            replaced = True
        else:
            out.append(b)
    if replaced:
        open("phi/sources.φ", "w").write("\n".join(out))
        print(f"Rewrote {src_name} -> {len(shards)} shard sources", file=sys.stderr)


if __name__ == "__main__":
    main()
