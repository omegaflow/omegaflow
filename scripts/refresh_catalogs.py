#!/usr/bin/env python3
"""Mycelium CDN refresh with integrity checks. Domain-based releases."""
import hashlib, json, os, re, sys, time, urllib.error, urllib.request

CATALOGS_REPO = os.environ.get("CATALOGS_REPO", "omegaflow/catalogs")

MIN_SIZES = {
    "eia_grid_demand.json": 2000,
    "firms_viirs_nrt_world.csv": 5000,
    "usgs_water_daily.json": 1000,
    "ebird_recent.json": 1000,
    "openaq_pm25.json": 1000,
    "openaq_pm10.json": 1000,
    "oceannetworks_sites.json": 1000,
}
DEFAULT_MIN_JSON = 200
DEFAULT_MIN_CSV = 1000
TTL_MAP = {}
HEALTH = {"updated": "", "hashes": {}, "corrupt": [], "unchanged": []}
LAST_HASHES = {}
FILENAME_DOMAIN = {}


def load_filename_domain_map():
    """Build {cdn_filename: domain_tag} from sources.φ."""
    result = {}
    with open("phi/sources.φ") as f:
        lines = f.readlines()
    cur = None
    for line in lines:
        if line.startswith("source "):
            cur = line.strip().split()[1]
        if cur and line.startswith("url ") and "releases/download/" in line:
            m = re.search(r"catalogs-([^/]+)/(.+)", line)
            if m:
                domain_tag = m.group(1)
                cdn_name = m.group(2).rstrip()
                result[cdn_name] = domain_tag
            cur = None
    return result


def cdn_url_for(filename):
    domain = FILENAME_DOMAIN.get(filename, "catalogs")
    return f"https://github.com/{CATALOGS_REPO}/releases/download/catalogs-{domain}/{filename}"


def load_last_hashes():
    try:
        url = f"https://github.com/{CATALOGS_REPO}/releases/download/catalogs/SYSTEM_HEALTH.json"
        req = urllib.request.Request(url, headers={"User-Agent": "omegaflow-bot/1.0"})
        with urllib.request.urlopen(req, timeout=30) as r:
            d = json.load(r)
        return d.get("hashes", {})
    except Exception:
        return {}


def load_ttl_from_sources():
    ttl = {}
    if not os.path.exists("phi/sources.φ"):
        return ttl
    with open("phi/sources.φ") as f:
        lines = f.readlines()
    cur = None
    cur_file = None
    for line in lines:
        if line.startswith("source "):
            cur = line.strip().split()[1]
        if line.startswith("url ") and "catalogs" in line and cur:
            m = re.search(r"download/catalogs-[^/]+/(.+)", line)
            if m:
                cur_file = m.group(1).rstrip()
        if cur and cur_file and line.startswith("ttl ") and cur_file.endswith((".json", ".csv")):
            try:
                ttl[cur_file] = int(line.split()[1])
            except ValueError:
                pass
    return ttl


def should_skip(fname):
    ttl = TTL_MAP.get(fname, 300)
    try:
        url = cdn_url_for(fname)
        req = urllib.request.Request(url, method="HEAD", headers={"User-Agent": "omegaflow-bot/1.0"})
        with urllib.request.urlopen(req, timeout=15) as r:
            lm = r.headers.get("Last-Modified", "")
            if lm:
                from email.utils import parsedate_to_datetime
                age = time.time() - parsedate_to_datetime(lm).timestamp()
                return age < ttl / 1.618
    except Exception:
        pass
    return False


def validate_asset(fname, data):
    min_sz = MIN_SIZES.get(fname, DEFAULT_MIN_JSON if fname.endswith(".json") else DEFAULT_MIN_CSV)
    if len(data) < min_sz:
        print(f"  CORRUPT: {len(data)}B < {min_sz}B min", file=sys.stderr)
        HEALTH["corrupt"].append(fname)
        return False
    if fname.endswith(".json"):
        try:
            json.loads(data)
        except Exception:
            print(f"  STRUCTURE FAIL: invalid JSON", file=sys.stderr)
            HEALTH["corrupt"].append(fname)
            return False
    elif fname.endswith(".csv"):
        text = data.decode(errors="replace")
        if not text.strip() or ("<html" in text[:200].lower() and "error" in text[:500].lower()):
            print(f"  STRUCTURE FAIL: HTML/error response", file=sys.stderr)
            HEALTH["corrupt"].append(fname)
            return False
    return True


def upload_asset(fname, path):
    import subprocess
    domain_tag = FILENAME_DOMAIN.get(fname, "catalogs")
    release = f"catalogs-{domain_tag}"
    cmd = ["gh", "release", "upload", release, path, "--repo", CATALOGS_REPO, "--clobber"]
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode == 0:
        print(f"  uploaded {fname} -> {release}", file=sys.stderr)
        return True
    print(f"  upload failed: {r.stderr[:200]}", file=sys.stderr)
    return False


def main():
    global TTL_MAP, LAST_HASHES, FILENAME_DOMAIN
    FILENAME_DOMAIN = load_filename_domain_map()
    TTL_MAP = load_ttl_from_sources()
    LAST_HASHES = load_last_hashes()

    HEALTH["updated"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    checked, skipped, uploaded = 0, 0, 0

    data_dir = "data"
    if not os.path.isdir(data_dir):
        print("No data/ directory", file=sys.stderr)
        return

    for fname in sorted(os.listdir(data_dir)):
        fpath = os.path.join(data_dir, fname)
        if not fname.endswith((".json", ".csv", ".geojson")):
            continue
        checked += 1

        if should_skip(fname):
            print(f"  SKIP {fname} (fresh, TTL={TTL_MAP.get(fname,300)}s)", file=sys.stderr)
            skipped += 1
            continue

        with open(fpath, "rb") as f:
            data = f.read()

        if not validate_asset(fname, data):
            continue

        h = hashlib.sha256(data).hexdigest()
        prev = LAST_HASHES.get(fname, "")
        if prev == h:
            HEALTH["unchanged"].append(fname)
        LAST_HASHES[fname] = h

        if upload_asset(fname, fpath):
            uploaded += 1

    HEALTH["hashes"] = LAST_HASHES
    with open(os.path.join(data_dir, "SYSTEM_HEALTH.json"), "w") as f:
        json.dump(HEALTH, f)
    # SYSTEM_HEALTH always goes to the catalogs release (legacy)
    domain_tag = FILENAME_DOMAIN.get("SYSTEM_HEALTH.json", "catalogs")
    FILENAME_DOMAIN["SYSTEM_HEALTH.json"] = "catalogs"
    upload_asset("SYSTEM_HEALTH.json", os.path.join(data_dir, "SYSTEM_HEALTH.json"))

    print(f"\nChecked: {checked}  Skipped: {skipped}  Uploaded: {uploaded}  Corrupt: {len(HEALTH['corrupt'])}",
          file=sys.stderr)


if __name__ == "__main__":
    main()
