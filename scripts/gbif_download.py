#!/usr/bin/env python3
"""Download a GBIF occurrence dataset (DWCA) and convert occurrence.txt to JSON.

Usage:
    GBIF_USER=... GBIF_PASS=... python3 scripts/gbif_download.py <taxonKey> <outfile.json>
"""
import json
import os
import sys
import time
import urllib.request
import urllib.parse

USER = os.environ.get("GBIF_USER", "")
PASS = os.environ.get("GBIF_PASS", "")
BASE = "https://api.gbif.org/v1/occurrence/download"


def http_req(url, method="GET", data=None, ua=None):
    headers = {"User-Agent": ua or f"{USER}/1.0", "Accept": "application/json"}
    body = None
    if data is not None:
        body = json.dumps(data).encode()
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(url, data=body, headers=headers, method=method)
    auth = f"{USER}:{PASS}".encode()
    import base64
    req.add_header("Authorization", "Basic " + base64.b64encode(auth).decode())
    with urllib.request.urlopen(req, timeout=300) as resp:
        return resp.read()


def main():
    if len(sys.argv) < 3:
        print("usage: gbif_download.py <taxonKey> <outfile.json>", file=sys.stderr)
        return 1
    taxon_key = sys.argv[1]
    outfile = sys.argv[2]
    if not USER or not PASS:
        print("GBIF_USER/GBIF_PASS not set", file=sys.stderr)
        return 1
    ua = f"{USER}/1.0"

    # 1) request download
    payload = {
        "predicate": {"type": "equals", "key": "TAXON_KEY", "value": int(taxon_key)},
        "format": "DWCA",
    }
    resp = http_req(BASE + "/request", "POST", payload, ua)
    dlkey = resp.decode().strip().strip('"')
    print(f"download key: {dlkey}", file=sys.stderr)
    if not dlkey or "error" in dlkey.lower():
        print(f"request failed: {resp.decode()[:200]}", file=sys.stderr)
        return 1

    # 2) poll
    status = ""
    for i in range(60):
        st = http_req(f"{BASE}/{dlkey}", "GET", None, ua)
        try:
            status = json.loads(st).get("status", "UNKNOWN")
        except Exception:
            status = "UNKNOWN"
        print(f"status: {status} ({i + 1})", file=sys.stderr)
        if status in ("SUCCEEDED", "FAILED", "KILLED"):
            break
        time.sleep(10)
    if status != "SUCCEEDED":
        print(f"download {status}", file=sys.stderr)
        return 1

    # 3) fetch zip
    zipdata = http_req(f"{BASE}/{dlkey}.zip", "GET", None, ua)
    with open("/tmp/gbif_download.zip", "wb") as f:
        f.write(zipdata)

    # 4) extract occurrence.txt
    import zipfile
    with zipfile.ZipFile("/tmp/gbif_download.zip") as z:
        names = [n for n in z.namelist() if n.endswith("occurrence.txt")]
        if not names:
            print("occurrence.txt not in archive", file=sys.stderr)
            return 1
        with z.open(names[0]) as src:
            import csv
            import io
            text = src.read().decode("utf-8", errors="replace")
            reader = csv.DictReader(io.StringIO(text), delimiter="\t")
            out = []
            for row in reader:
                try:
                    lat = float(row.get("decimalLatitude"))
                    lon = float(row.get("decimalLongitude"))
                except (TypeError, ValueError):
                    continue
                out.append({
                    "lat": lat,
                    "lon": lon,
                    "gbifID": row.get("gbifID"),
                    "taxonKey": row.get("taxonKey"),
                    "kingdom": row.get("kingdom"),
                    "species": row.get("species"),
                    "eventDate": row.get("eventDate"),
                    "year": row.get("year"),
                })
    with open(outfile, "w") as f:
        json.dump(out, f)
    print(f"rows: {len(out)} -> {outfile}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
