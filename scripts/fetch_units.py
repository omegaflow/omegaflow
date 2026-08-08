#!/usr/bin/env python3
"""Fetch every source API once, extract column units from the actual response.
Writes phi/recovery/unit_cache.json: url -> {field: {unit,value,dtype}} or
{header_units: {...}} for text formats. Run to completion - the cache is truth."""
import re
import json
import sys
import time
import socket
import threading
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET

UA = "omegaflow/1.0"
CACHE_PATH = "phi/recovery/unit_cache.json"

def resolve(url):
    for a, b in [("{today}", "2026-08-07"), ("{yesterday}", "2026-08-06"),
                 ("{year}", "2026"), ("{month}", "08"), ("{day}", "07"),
                 ("{lat}", "52.52"), ("{lon}", "13.40"),
                 ("{ra}", "266.4"), ("{dec}", "-29.0"), ("{radius}", "5.0"),
                 ("{date}", "2026-08-07"), ("{now}", "2026-08-07T12:00Z"),
                 ("{net}", "IU"), ("{id}", "25544"), ("{epoch}", "2026")]:
        url = url.replace(a, b)
    return url


def get(url, timeout=5):
    """Fetch with a hard thread-join timeout - cannot hang."""
    result = {}

    def worker():
        try:
            socket.setdefaulttimeout(timeout)
            req = urllib.request.Request(url, headers={"User-Agent": UA})
            with urllib.request.urlopen(req, timeout=timeout) as r:
                ctype = r.headers.get("Content-Type", "")
                raw = r.read().decode("utf-8", errors="replace")[:500000]
                result["ctype"] = ctype
                result["raw"] = raw
        except Exception as e:
            result["error"] = str(e)

    t = threading.Thread(target=worker, daemon=True)
    t.start()
    t.join(timeout + 3)
    if "raw" in result:
        return result.get("ctype", ""), result["raw"]
    return None, None


def is_tap(url):
    u = url.lower()
    return ("tap" in u and "sync" in u) or "tapvizier" in u


def force_json(url):
    u = resolve(url)
    u = re.sub(r"(?i)format=(csv|votable|text|xml)", "format=json", u)
    if "format=json" not in u.lower():
        u += "&FORMAT=json" if "?" in u else "?FORMAT=json"
    return u


def parse_votable_units(raw, out):
    """Extract FIELD name/unit/datatype from VOTable XML."""
    try:
        root = ET.fromstring(raw)
    except Exception:
        return False
    ns = {"v": "http://www.ivoa.net/xml/VOTable/v1.3",
          "v2": "http://www.ivoa.net/xml/VOTable/v1.2"}
    fields = root.findall(".//FIELD") or root.findall(".//{*}FIELD")
    if not fields:
        return False
    for f in fields:
        name = f.get("name") or ""
        unit = f.get("unit") or ""
        dtype = f.get("datatype") or ""
        if name:
            e = out.setdefault(name, {})
            if unit:
                e["unit"] = unit
            if dtype:
                e["dtype"] = dtype
    # also try to capture a data sample row
    trs = root.findall(".//TR") or root.findall(".//{*}TR")
    if trs and fields:
        td = trs[0].findall("TD") or trs[0].findall("{*}TD")
        for i, f in enumerate(fields):
            name = f.get("name", "")
            if i < len(td) and td[i].text:
                try:
                    val = float(str(td[i].text))
                    out.setdefault(name, {}).setdefault("value", val)
                except ValueError:
                    pass
    return True


def parse_tap_metadata(raw, out):
    try:
        data = json.loads(raw)
    except Exception:
        return
    if isinstance(data, dict) and "metadata" in data:
        metas = data.get("metadata", [])
        for i, m in enumerate(metas):
            if not isinstance(m, dict):
                continue
            name = m.get("name", "") or str(i)
            e = out.setdefault(name, {})
            if m.get("unit"):
                e["unit"] = m["unit"]
            if m.get("datatype"):
                e["dtype"] = str(m["datatype"])[:20]
        # sample values from data rows
        rows = data.get("data", [])
        cols = [m.get("name", "") for m in metas if isinstance(m, dict)]
        if rows and cols:
            for i, c in enumerate(cols):
                for row in rows[:3]:
                    if i < len(row) and isinstance(row[i], (int, float)):
                        out.setdefault(c, {}).setdefault("value", row[i])
                        break


def parse_list_json(raw, out):
    try:
        data = json.loads(raw)
    except Exception:
        return
    items = data if isinstance(data, list) else [data]
    for it in items:
        if isinstance(it, dict):
            for k, v in it.items():
                e = out.setdefault(k, {})
                if isinstance(v, (int, float)) and "value" not in e:
                    e["value"] = v
                elif isinstance(v, str) and "string" not in e:
                    e["string"] = v[:60]


def parse_ndbc_units(raw, out):
    """NDBC realtime2 text files have a unit line as the second row."""
    lines = raw.strip().split("\n")
    if len(lines) < 2:
        return
    header = lines[0].lstrip("#").split()
    units = lines[1].lstrip("#").split()
    if len(header) == len(units) and units and not units[0].isdigit():
        hdr = {}
        for h, u in zip(header, units):
            hdr[h] = u
        out["header_units"] = hdr


def parse_csv_units(raw, out):
    lines = raw.strip().split("\n")
    if not lines:
        return
    header = lines[0].lstrip("#").strip().split()
    if len(header) < 2 or header[0].startswith(("1", "2", "2026")):
        return
    hdr = {}
    for i, h in enumerate(header):
        hdr[h] = ""
    out["csv_columns"] = hdr


def main():
    args = [a for a in sys.argv[1:]]
    chunk = 0
    if "--chunk" in args:
        i = args.index("--chunk")
        chunk = int(args[i + 1])
    cache = {}
    try:
        cache = json.load(open(CACHE_PATH))
    except Exception:
        pass

    blocks = open("phi/sources.φ").read().strip().split("\n\n")
    seen = set()
    urls = []
    for b in blocks:
        if not b.strip():
            continue
        for l in b.split("\n"):
            ls = l.strip()
            if ls.startswith("url "):
                u = ls.replace("url ", "")
                if u and u not in seen:
                    seen.add(u)
                    urls.append(u)
                break
    # re-fetch only TAP endpoints and invalid-json entries (unit-bearing)
    def worth_retry(u, v):
        if not isinstance(v, dict) or not v.get("error"):
            return False
        err = str(v["error"])
        if "invalid-json" in err:
            return True
        if is_tap(u) or "heasarc" in u or "irsa" in u or "gea." in u or "tapvizier" in u:
            return True
        return False

    urls = [u for u in urls if worth_retry(u, cache.get(u))]

    total = len(urls)
    for i, u in enumerate(urls):
        if chunk and i >= chunk:
            break
        if i % 5 == 0:
            sys.stderr.write(f"fetching[{i}]: {u[:80]}\n")
        prev = cache.get(u)
        if prev is not None and not (isinstance(prev, dict) and prev.get("error")):
            continue
        ctype, raw = None, None
        if is_tap(u):
            to = 20
            for attempt in range(2):
                ctype, raw = get(force_json(u), timeout=to)
                if raw is not None:
                    break
                time.sleep(1)
        else:
            ctype, raw = get(resolve(u))

        if raw is None:
            cache[u] = {"error": "no-response"}
        else:
            out = {}
            content_type = (ctype or "").lower()
            first_line = raw.lstrip().split("\n")[0][:200]
            if "json" in content_type or first_line.startswith(("{", "[")):
                try:
                    json.loads(raw)
                    parse_tap_metadata(raw, out)
                    if not out:
                        parse_list_json(raw, out)
                except Exception:
                    out = {"error": "invalid-json"}
            elif raw.lstrip().lower().startswith(("<?xml", "<votable")):
                if parse_votable_units(raw, out):
                    pass  # fields extracted
                else:
                    out = {"error": "votable-unparsed", "preview": first_line[:150]}
            elif "ndbc.noaa.gov" in u and (".txt" in u or ".spec" in u or ".dart" in u):
                parse_ndbc_units(raw, out)
            else:
                # text: try header-line unit extraction
                parse_ndbc_units(raw, out)
                if not out:
                    out = {"error": "text", "preview": first_line[:150]}
            cache[u] = out

        if i % 20 == 0:
            json.dump(cache, open(CACHE_PATH, "w"), indent=0)
            sys.stderr.write(f"[{i}/{total}] ok={sum(1 for v in cache.values() if not isinstance(v, dict) or not v.get('error'))}\n")
        time.sleep(0.25)

    json.dump(cache, open(CACHE_PATH, "w"), indent=0)
    ok = sum(1 for v in cache.values() if isinstance(v, dict) and not v.get("error") and ("header_units" in v or "csv_columns" in v or any(k for k in v if k not in ("error",))))
    print(f"Done. {len(cache)} URLs cached, {ok} with unit info.")


if __name__ == "__main__":
    main()
