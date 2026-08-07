#!/usr/bin/env python3
"""Audit source keys from CDN cache files (not live APIs).
Streams results line-by-line so partial progress survives. CDN sources use
Range requests (fast, no full download). Live sources get short timeouts."""
import json, ssl, sys, urllib.request, concurrent.futures
from pathlib import Path

SOURCES = Path("phi/sources.φ")
OUT = Path("phi/api_audit.jsonl")

ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE

MAX_BYTES = 200_000  # 200 KB range for CDN files

def fetch(url, timeout=10):
    try:
        headers = {"User-Agent": "omegaflow-audit/1.0"}
        if "releases/download" in url:
            headers["Range"] = f"bytes=0-{MAX_BYTES}"
        req = urllib.request.Request(url, headers=headers)
        with urllib.request.urlopen(req, timeout=timeout, context=ctx) as r:
            return r.status, r.read()
    except Exception:
        return 0, b""

def parse_blocks():
    with open(SOURCES) as f:
        text = f.read()
    blocks = text.split("\n\n")
    out = []
    for block in blocks:
        lines = [l.strip() for l in block.split("\n") if l.strip()]
        if not lines or not lines[0].startswith("url "):
            continue
        url = lines[0][4:].strip()
        declared = []
        for l in lines:
            parts = l.split()
            if parts and parts[0] in ("field", "path", "last", "count", "field_in", "last_row", "last_line", "first", "last_obj", "obj_last", "regex", "hapi"):
                if len(parts) >= 2:
                    declared.append((parts[0], parts[1]))
        out.append({"url": url, "lines": lines, "declared": declared, "is_cdn": "releases/download" in url})
    return out

def extract_keys(data):
    try:
        obj = json.loads(data.decode(errors="replace"))
    except Exception:
        return None, None
    def find_records(o, depth=0):
        if depth > 3: return None
        if isinstance(o, dict):
            for key in ("data", "features", "result", "results", "items", "rows", "value", "values"):
                if key in o and isinstance(o[key], list) and o[key]:
                    return o[key]
            for v in o.values():
                r = find_records(v, depth+1)
                if r is not None:
                    return r
        elif isinstance(o, list) and o:
            return o
        return None
    records = find_records(obj)
    sample = records[0] if records else obj
    keys = set()
    def collect(o):
        if isinstance(o, dict):
            for k, v in o.items():
                keys.add(k)
                if isinstance(v, (dict, list)):
                    collect(v)
        elif isinstance(o, list) and o:
            collect(o[0])
    collect(sample)
    return sorted(keys), sample

def audit(src):
    url = src["url"]
    t = url.replace("{today}", "2026-08-06").replace("{year}", "2026").replace("{month}", "08").replace("{day}", "06")
    if "{lat}" in t: t = t.replace("{lat}", "35.0")
    if "{lon}" in t: t = t.replace("{lon}", "139.0")
    if "{grid}" in t: t = t.replace("{grid}", "35.0,139.0")
    if "{latest}" in t: t = t.replace("{latest}", "")

    status, data = fetch(t)
    real_keys, sample = (None, None)
    err = None
    # HTTP 206 = partial content from Range request — treat as success
    if status in (200, 206) and len(data) > 16:
        real_keys, sample = extract_keys(data)
        if real_keys is None:
            err = "not-json"
            real_keys = []
    else:
        err = f"HTTP {status}" if status else "timeout/error"
        real_keys = []

    return {
        "url": url,
        "is_cdn": src["is_cdn"],
        "status": status,
        "size": len(data) if status == 200 else 0,
        "error": err,
        "real_keys": real_keys,
        "declared": src["declared"],
        "sample_keys": list(sample.keys()) if isinstance(sample, dict) else None,
    }

def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--cdn-only", action="store_true", help="audit only CDN sources")
    parser.add_argument("--live-only", action="store_true", help="audit only live sources")
    args = parser.parse_args()

    blocks = parse_blocks()
    if args.cdn_only:
        blocks = [b for b in blocks if b["is_cdn"]]
    elif args.live_only:
        blocks = [b for b in blocks if not b["is_cdn"]]
    else:
        blocks.sort(key=lambda b: not b["is_cdn"])
    print(f"Auditing {len(blocks)} sources ({'CDN' if args.cdn_only else 'LIVE' if args.live_only else 'all'} first)...", file=sys.stderr)
    with open(OUT, "w") as f:
        with concurrent.futures.ThreadPoolExecutor(max_workers=32) as ex:
            futures = {ex.submit(audit, b): b for b in blocks}
            done = 0
            for fut in concurrent.futures.as_completed(futures):
                done += 1
                if done % 200 == 0:
                    print(f"  {done}/{len(blocks)}", file=sys.stderr)
                try:
                    f.write(json.dumps(fut.result()) + "\n")
                except Exception:
                    pass
                f.flush()

    cdn_ok = 0
    cdn_total = 0
    live_ok = 0
    live_total = 0
    ok = 0
    statuses = {}
    for line in open(OUT):
        r = json.loads(line)
        if r["is_cdn"]:
            cdn_total += 1
            if r["status"] in (200, 206) and not r["error"]: cdn_ok += 1
        else:
            live_total += 1
            if r["status"] in (200, 206) and not r["error"]: live_ok += 1
        if r["status"] in (200, 206) and not r["error"]: ok += 1
        statuses[r["status"]] = statuses.get(r["status"], 0) + 1
    print(f"\n=== SUMMARY ===", file=sys.stderr)
    print(f"Total: {len(blocks)}", file=sys.stderr)
    print(f"CDN: {cdn_ok}/{cdn_total} ok", file=sys.stderr)
    print(f"Live: {live_ok}/{live_total} ok", file=sys.stderr)
    print(f"HTTP 200+JSON: {ok}", file=sys.stderr)
    print(f"Statuses: {dict(sorted(statuses.items()))}", file=sys.stderr)
    print(f"Saved to {OUT}", file=sys.stderr)

if __name__ == "__main__":
    main()
