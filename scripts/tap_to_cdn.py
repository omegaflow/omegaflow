#!/usr/bin/env python3
"""Convert live TAP-STATIC catalog sources to CDN JSON on the omegaflow/catalogs release.

Reads phi/sources.φ, finds sources pointing at TAP endpoints (no template vars),
downloads the FULL table (removing TOP limits), converts to {"data":[...]},
and uploads to the v1.0 release via the CATALOGS_TOKEN.

Uses astropy.io.votable for BINARY/BINARY2/TABLEDATA decoding (cloud-side);
falls back to a plain CSV parser when astropy is unavailable.

Usage:
    python3 scripts/tap_to_cdn.py                  # process all, small first
    python3 scripts/tap_to_cdn.py --source <name>   # process one source
    python3 scripts/tap_to_cdn.py --dry-run         # just list what would run
    python3 scripts/tap_to_cdn.py --limit 5000      # cap rows (debug)

Env:
    CATALOGS_TOKEN: PAT with contents write to omegaflow/catalogs
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
MAX_ROWS = int(os.environ.get("TAP_MAX_ROWS", "0"))
CATALOGS_REPO = "omegaflow/catalogs"
TIMEOUT = int(os.environ.get("TAP_TIMEOUT", "120"))
SLEEP = float(os.environ.get("TAP_SLEEP", "2"))
MAX_SOURCES = int(os.environ.get("TAP_MAX_SOURCES", "0"))

# Catalogs too large for a single flat JSON (would exceed release/server limits).
# These need spatial sharding; skip them so they don't fail verification.
HUGE_CATALOGS = {
    "vizier_nvss_radio", "vizier_nvss_radio_catalog", "vizier_nvss_galcenter",
    "vizier_first_radio", "vizier_first_radio_catalog", "vizier_gleam_extragalactic_catalog",
    "vizier_panstarrs_dr2_catalog", "vizier_2dfgrs", "vizier_catwise_agn_candidates",
    "astro_vizier_panstarrs", "astro_vizier_2mass",
}

TAP_MARKERS = [
    "/tap/sync", "tapvizier", "heasarc.gsfc.nasa.gov/xamin",
    "irsa.ipac.caltech.edu/TAP", "mast.stsci.edu/vo-tap",
    "skyserver.sdss.org", "ned.ipac.caltech.edu/tap",
    "gea.esac.esa.int/tap-server", "gaia.ari.uni-heidelberg.de/tap",
]


def find_tap_sources(path="phi/sources.φ"):
    content = open(path).read()
    blocks = re.split(r"\n(?=source )", content)
    out = []
    for b in blocks:
        m = re.match(r"source (\S+)", b)
        if not m:
            continue
        name = m.group(1)
        urls = re.findall(r"url\s+(\S+)", b)
        if not urls:
            continue
        url = urls[0]
        if "releases/download/" in url:
            continue
        if not any(t in url for t in TAP_MARKERS):
            continue
        if "{" in url:
            continue
        out.append({"source": name, "url": url})
    return out


def strip_top(url):
    """Remove TOP n (incl. +-encoded) from a TAP URL to fetch the full catalog.
    Operates on the raw query to preserve existing encoding (%22 etc)."""
    return re.sub(
        r"(\bSELECT\b[+\s%]*)\bTOP\b[+\s%]*\d+[+\s%]*",
        r"\1",
        url,
        flags=re.IGNORECASE,
        count=1,
    )


def extract_table(url):
    """Extract the FROM table name from a TAP URL (decoded)."""
    decoded = urllib.parse.unquote_plus(url)
    m = re.search(r"\bFROM\b\s+([A-Za-z0-9_\.]+)", decoded, re.IGNORECASE)
    if not m:
        m = re.search(r'FROM\s+"([^"]+)"', decoded, re.IGNORECASE)
    return m.group(1) if m else None


def tap_schema_columns(url, table):
    """Query TAP_SCHEMA.columns for the actual column names.
    Returns list of column names, or None if the query fails."""
    if not table or "heasarc" not in url:
        return None
    base = url.split("QUERY=")[0]
    q = f"SELECT column_name FROM TAP_SCHEMA.columns WHERE table_name='{table}'"
    try:
        text = fetch(base + "QUERY=" + urllib.parse.quote_plus(q))
    except Exception:
        return None
    rows = votable_to_json(text)
    if rows is None:
        rows = csv_to_json(text)
    if not rows:
        return None
    return [list(r.values())[0] for r in rows]


def repair_query_for_schema(url):
    """Check SELECT columns against TAP_SCHEMA. If they don't match,
    return a repaired URL that uses SELECT * (or available columns).
    Returns (url, repaired, missing_cols)."""
    decoded = urllib.parse.unquote_plus(url)
    sel = re.search(r"SELECT\s+(.*?)\s+FROM", decoded, re.I | re.DOTALL)
    if not sel:
        return url, False, []
    sel_cols = [c.strip().strip('"') for c in
                re.sub(r"TOP\s+\d+\s*", "", sel.group(1), flags=re.I).split(",")]
    
    table = extract_table(decoded)
    actual = tap_schema_columns(decoded, table)
    if actual is None:
        return url, False, []
    missing = [c for c in sel_cols if c not in actual]
    if not missing:
        return url, False, []
    # Repair: remove invalid columns from SELECT, keep valid ones
    valid = [c for c in sel_cols if c not in missing]
    if not valid:
        return url, False, missing  # no valid columns left -> can't repair
    repaired_select = "SELECT " + ", ".join(valid) + " FROM"
    repaired = re.sub(r"SELECT\s+.*?\s+FROM", repaired_select, decoded, count=1, flags=re.I | re.DOTALL)
    repaired = re.sub(r"TOP\s+\d+\s*", "", repaired, count=1, flags=re.I)
    # Remove ORDER BY referencing missing columns
    repaired = re.sub(r"\s+ORDER\s+BY\s+.*$", "", repaired, count=1, flags=re.I)
    # Re-encode query part
    if "QUERY=" in url:
        base, _, _ = url.partition("QUERY=")
        q = repaired.split("QUERY=", 1)[1] if "QUERY=" in repaired else repaired
        return base + "QUERY=" + urllib.parse.quote_plus(q), True, missing
    return repaired, True, missing


def detect_ra_col(url):
    """Detect the RA column name from the SELECT list."""
    decoded = urllib.parse.unquote_plus(url)
    sel = re.search(r"SELECT\s+(.*?)\s+FROM", decoded, re.I | re.DOTALL)
    if not sel:
        return None
    candidates = ["RAJ2000", "_RA", "ra", "RA", "RA_ICRS", "_RA_icrs", "RA_ICRS"]
    cols = [c.strip().strip('"') for c in re.sub(r"TOP\s+\d+\s*","",sel.group(1),flags=re.I).split(",")]
    for cand in candidates:
        if cand in cols:
            return cand
    # fuzzy: any column with 'ra' in the name (case-insensitive)
    for c in cols:
        if 'ra' in c.lower():
            return c
    return None


def try_fetch_rows(url):
    """Try a single fetch, return (rows, truncated)."""
    text = fetch(url)
    rows = votable_to_json(text)
    if rows is None:
        rows = csv_to_json(text)
    truncated = False
    if rows:
        n = len(rows)
        # Heuristic: common server limits indicate truncation
        if n in (5000, 2000, 10000, 50000, 3000, 1000):
            truncated = True
        # Also check for OVERFLOW status in VOTable XML
        if 'value="OVERFLOW"' in text:
            truncated = True
    return rows, truncated


def spatial_fetch(url_template, lo, hi, ra_col, limit):
    """Recursive spatial fetch: query RA bin, split if saturated."""
    # Inject WHERE clause into the query, keeping encoding intact
    decoded = urllib.parse.unquote_plus(url_template)
    where_clause = f"{ra_col} BETWEEN {lo:.6f} AND {hi:.6f}"
    if "WHERE" in decoded:
        decoded = re.sub(r"WHERE\s+.*$", f"WHERE {where_clause}", decoded, count=1, flags=re.I)
    else:
        decoded = decoded + f" WHERE {where_clause}"
    # Re-encode the query part only
    if "QUERY=" in url_template:
        base, _, _ = url_template.partition("QUERY=")
        q = decoded.split("QUERY=", 1)[1] if "QUERY=" in decoded else ""
        req_url = base + "QUERY=" + urllib.parse.quote_plus(q)
    else:
        req_url = urllib.parse.quote(decoded, safe=":/?=&%")

    text = fetch(req_url)
    rows = votable_to_json(text)
    if rows is None:
        rows = csv_to_json(text)

    n = len(rows)
    if n < limit:
        return rows
    if hi - lo < 0.5:
        return rows

    mid = (lo + hi) / 2.0
    time.sleep(SLEEP * 0.5)
    left = spatial_fetch(url_template, lo, mid, ra_col, limit)
    time.sleep(SLEEP * 0.5)
    right = spatial_fetch(url_template, mid, hi, ra_col, limit)
    return left + right


def fetch_full_spatial(url, ra_col):
    """Fetch a complete catalog:
    1. Try full fetch (no WHERE). If complete, return rows.
    2. If truncated, recursively spatial-fetch RA bins.
    Returns (rows, truncation_flag, shard_bin_count)."""
    rows, truncated = try_fetch_rows(url)
    if not rows:
        return [], False, 0
    if not truncated:
        return rows, False, 0
    limit = len(rows)  # server limit detected from saturation
    print(f"    spatial sharding (limit={limit})", file=sys.stderr)
    ra_col = ra_col or "RAJ2000"
    # Recurse RA 0-360
    rows = spatial_fetch(url, 0.0, 360.0, ra_col, limit)
    print(f"    spatial complete: {len(rows)} rows", file=sys.stderr)
    return rows, True, 0


def fetch(url):
    """Fetch TAP response. Retries without FORMAT=csv if the server rejects it."""
    attempts = [url]
    if re.search(r"FORMAT=csv", url, re.IGNORECASE):
        attempts.append(re.sub(r"&?FORMAT=csv", "", url, flags=re.IGNORECASE))
    last_text = ""
    for u in attempts:
        req = urllib.request.Request(u, headers={"User-Agent": "omegaflow-bot/1.0"})
        try:
            with urllib.request.urlopen(req, timeout=TIMEOUT) as r:
                text = r.read().decode("utf-8", errors="replace")
            last_text = text
            if "Unsupported format" in text or 'value="ERROR"' in text:
                continue
            return text
        except urllib.error.HTTPError as e:
            if e.code in (400, 422, 503) and len(attempts) > 1:
                continue
            raise
    return last_text


def csv_to_json(text):
    """Plain CSV -> [ {col: val}, ... ]. Returns [] if not CSV-shaped."""
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
        if MAX_ROWS and len(rows) >= MAX_ROWS:
            break
    return rows


def votable_to_json(text):
    """Parse VOTable (any encoding) via astropy. Falls back to None if astropy absent."""
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
            if MAX_ROWS and len(rows) >= MAX_ROWS:
                break
        return rows
    except Exception as e:
        print(f"    astropy parse failed: {str(e)[:80]}", file=sys.stderr)
        return None


def get_release_for_domain(domain):
    """Get (release_id, tag_name) for a domain. Creates release if missing."""
    tag = f"catalogs-{domain}"
    api_url = f"https://api.github.com/repos/{CATALOGS_REPO}/releases/tags/{urllib.parse.quote(tag, safe='')}"
    req = urllib.request.Request(api_url, headers={"Authorization": f"Bearer {TOKEN}",
                                                     "Accept": "application/vnd.github+json",
                                                     "User-Agent": "omegaflow-bot/1.0"})
    try:
        with urllib.request.urlopen(req, timeout=15) as r:
            d = json.load(r)
            return d["id"], tag
    except urllib.error.HTTPError as e:
        if e.code != 404:
            return None, None
    
    body = json.dumps({"tag_name": tag, "name": tag, "body": f"CDN for {domain}", "draft": False}).encode()
    create_url = f"https://api.github.com/repos/{CATALOGS_REPO}/releases"
    req = urllib.request.Request(create_url, data=body,
                                 headers={"Authorization": f"Bearer {TOKEN}",
                                          "Content-Type": "application/json",
                                          "Accept": "application/vnd.github+json",
                                          "User-Agent": "omegaflow-bot/1.0"})
    try:
        with urllib.request.urlopen(req, timeout=15) as r:
            d = json.load(r)
            return d["id"], tag
    except Exception:
        return None, None


_RELEASE_CACHE = {}


def get_upload_url(domain):
    """Return (upload_url, release_tag) for a domain."""
    if domain not in _RELEASE_CACHE:
        rid, tag = get_release_for_domain(domain)
        if rid:
            _RELEASE_CACHE[domain] = (f"https://uploads.github.com/repos/{CATALOGS_REPO}/releases/{rid}/assets", tag)
        else:
            return None, None
    return _RELEASE_CACHE[domain]


def extract_domain_from_url(url):
    """Extract clean domain from a TAP URL."""
    host = urllib.parse.urlparse(url).hostname or ""
    # Map to canonical domain for release tagging
    domain_map = {
        "tapvizier.cds.unistra.fr": "tapvizier.cds.unistra.fr",
        "cds.unistra.fr": "tapvizier.cds.unistra.fr",
        "heasarc.gsfc.nasa.gov": "heasarc.gsfc.nasa.gov",
        "irsa.ipac.caltech.edu": "irsa.ipac.caltech.edu",
        "mast.stsci.edu": "mast.stsci.edu",
        "skyserver.sdss.org": "skyserver.sdss.org",
        "gea.esac.esa.int": "gea.esac.esa.int",
        "gaia.ari.uni-heidelberg.de": "gaia.ari.uni-heidelberg.de",
        "ned.ipac.caltech.edu": "ned.ipac.caltech.edu",
    }
    if host in domain_map:
        return domain_map[host]
    return host


def upload_asset(filename, data, domain):
    if not TOKEN:
        print("  !! no CATALOGS_TOKEN, skip upload", file=sys.stderr)
        return False
    upload_url, tag = get_upload_url(domain)
    if not upload_url:
        print(f"  !! no release for {domain}", file=sys.stderr)
        return False
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


def cdn_filename(source):
    return f"{source}.json"


def update_source_block(block, cdn_file, domain):
    """Update a source block in sources.φ to point at the CDN JSON."""
    cdn_url = f"https://github.com/omegaflow/catalogs/releases/download/catalogs-{domain}/{cdn_file}"
    new_block = block
    new_block = re.sub(
        r"url\s+https?://[^\s]+",
        f"url {cdn_url}",
        new_block, count=1)
    # 2. rows+format text -> map . or cmap . + format json (CDN data is named-object JSON)
    if "rows" in new_block:
        new_block = new_block.replace("format text", "format json", 1)
        # sources needing distance/redshift keys -> cmap . (keeps ra_key/dec_key/z_key)
        if any(k in new_block for k in ("z_key ", "plx_key ", "dist_key ",
                                         "pmra_key ", "pmdec_key ", "rv_key ")):
            new_block = re.sub(r"\nrows\b", "\ncmap .", new_block, count=1)
        else:
            new_block = re.sub(r"\nrows\b", "\nmap .", new_block, count=1)
            new_block = re.sub(r"^ra_key ", "lat_key ", new_block, flags=re.M)
            new_block = re.sub(r"^dec_key ", "lon_key ", new_block, flags=re.M)
    # 3. cmap/map with no format -> add format json
    elif ("cmap " in new_block or "map ." in new_block) and "format " not in new_block:
        new_block = re.sub(
            r"(url\s+\S+\n)",
            r"\1format json\n",
            new_block, count=1)
    return new_block


def update_sources_file(uploaded, path="phi/sources.φ"):
    """Rewrite sources.φ so uploaded sources point at their CDN file.
    uploaded = {cdn_file: domain_tag}"""
    content = open(path).read()
    blocks = re.split(r"\n(?=source )", content)
    changed = 0
    for i, b in enumerate(blocks):
        m = re.match(r"source (\S+)", b)
        if not m:
            continue
        name = m.group(1)
        cdn_file = f"{name}.json"
        if cdn_file not in uploaded:
            continue
        domain = uploaded[cdn_file]
        new_b = update_source_block(b, cdn_file, domain)
        if new_b != b:
            blocks[i] = new_b
            changed += 1
            print(f"  updated {name} -> {cdn_file}", file=sys.stderr)
    if changed:
        open(path, "w").write("\n".join(blocks))
    return changed


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", help="process only this source")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--limit", type=int, default=0, help="max rows (debug)")
    ap.add_argument("--update-sources", action="store_true",
                    help="rewrite sources.φ so uploaded sources point at their CDN file")
    args = ap.parse_args()
    global MAX_ROWS
    if args.limit:
        MAX_ROWS = args.limit

    sources = find_tap_sources()
    if args.source:
        sources = [s for s in sources if s["source"] == args.source]
    if MAX_SOURCES:
        sources = sources[:MAX_SOURCES]
    print(f"TAP-STATIC sources: {len(sources)}", file=sys.stderr)

    done, skipped, failed = 0, 0, 0
    uploaded = {}
    for s in sources:
        name = s["source"]
        fname = cdn_filename(name)
        domain = extract_domain_from_url(s["url"])
        print(f"--- {name}", file=sys.stderr)
        if args.dry_run:
            print(f"    {s['url'][:120]}", file=sys.stderr)
            continue
        if name in HUGE_CATALOGS:
            print("    HUGE catalog (needs sharding) - skip", file=sys.stderr)
            skipped += 1
            continue
        try:
            url = strip_top(s["url"])
            url, repaired, missing_cols = repair_query_for_schema(url)
            if missing_cols:
                print(f"    SCHEMA MISMATCH: {missing_cols} (repaired={'yes' if repaired else 'no'})",
                      file=sys.stderr)
                if not repaired:
                    skipped += 1
                    continue
            ra_col = detect_ra_col(url)
            rows, truncated, n_shards = fetch_full_spatial(url, ra_col)
            if not rows:
                print("    EMPTY", file=sys.stderr)
                skipped += 1
                continue
            top_m = re.search(r'TOP\+?(\d+)', s["url"], re.I)
            if top_m:
                orig_top = int(top_m.group(1))
                if len(rows) <= orig_top:
                    print(f"    INCOMPLETE: got {len(rows)} rows, original TOP was {orig_top} "
                          f"(skipping, keeping old CDN file)", file=sys.stderr)
                    skipped += 1
                    continue
            print(f"    {len(rows)} rows{' (spatial)' if truncated else ''}", file=sys.stderr)
            payload = json.dumps({"data": rows}).encode()
            if upload_asset(fname, payload, domain):
                done += 1
                uploaded[fname] = domain
            else:
                failed += 1
        except Exception as e:
            print(f"    FAILED: {str(e)[:120]}", file=sys.stderr)
            failed += 1
        time.sleep(SLEEP)

    if args.update_sources and uploaded:
        print(f"\nUpdating sources.φ ({len(uploaded)} files)...", file=sys.stderr)
        update_sources_file(uploaded)

    print(f"\nDone: {done}  Skipped: {skipped}  Failed: {failed}", file=sys.stderr)


if __name__ == "__main__":
    main()
