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
RELEASE_URL = "https://uploads.github.com/repos/omegaflow/catalogs/releases/363018488/assets"
MAX_ROWS = int(os.environ.get("TAP_MAX_ROWS", "0"))
TIMEOUT = int(os.environ.get("TAP_TIMEOUT", "120"))
SLEEP = float(os.environ.get("TAP_SLEEP", "2"))
MAX_SOURCES = int(os.environ.get("TAP_MAX_SOURCES", "0"))

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


def upload_asset(filename, data):
    if not TOKEN:
        print("  !! no CATALOGS_TOKEN, skip upload", file=sys.stderr)
        return False
    req = urllib.request.Request(
        f"{RELEASE_URL}?name={filename}",
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


def update_source_block(block, cdn_file):
    """Update a source block in sources.φ to point at the CDN JSON."""
    new_block = block
    new_block = re.sub(
        r"url\s+https?://[^\s]+",
        f"url https://github.com/omegaflow/catalogs/releases/download/v1.0/{cdn_file}",
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
    """Rewrite sources.φ so uploaded sources point at their CDN file."""
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
        new_b = update_source_block(b, cdn_file)
        if new_b != b:
            blocks[i] = new_b
            changed += 1
            print(f"  updated {name} -> {cdn_file}", file=sys.stderr)
    if changed:
        open(path, "w").write("".join(blocks))
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
    uploaded = set()
    for s in sources:
        name = s["source"]
        fname = cdn_filename(name)
        print(f"--- {name}", file=sys.stderr)
        if args.dry_run:
            print(f"    {s['url'][:120]}", file=sys.stderr)
            continue
        try:
            url = strip_top(s["url"])
            text = fetch(url)
            rows = votable_to_json(text)
            if rows is None:
                rows = csv_to_json(text)
            if not rows:
                print("    EMPTY", file=sys.stderr)
                skipped += 1
                continue
            payload = json.dumps({"data": rows}).encode()
            if upload_asset(fname, payload):
                done += 1
                uploaded.add(fname)
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
