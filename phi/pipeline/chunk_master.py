#!/usr/bin/env python3
import json, subprocess, os, time

os.chdir("/home/johannes/projects/omegaflow")
R = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync"
CHUNK_DIR = "phi/port"
CAP = 2_000_000

def fetch_band(cmd, out, lo):
    rows = []
    rc = 0
    for attempt in range(3):
        r = subprocess.run(cmd, capture_output=True, text=True)
        if os.path.exists(out):
            with open(out) as f:
                try:
                    rows = json.load(f)
                except Exception:
                    rows = []
                    os.remove(out)
        if r.returncode == 0 and rows:
            return rows
        rc = r.returncode
        print(f"  band {lo} attempt {attempt+1}: rc={rc} rows={len(rows)} {r.stderr[-200:]}", flush=True)
        if attempt < 2:
            time.sleep(60)
    return []

def chunk_catalog(name, table, columns, skip_null, ra_col, bands):
    done = []
    failed = 0
    for (lo, hi) in bands:
        out = f"{CHUNK_DIR}/{name}_c{lo}.json"
        if os.path.exists(out):
            with open(out) as f:
                try:
                    rows = json.load(f)
                except Exception:
                    rows = []
            if rows:
                print(f"resume {name} RA {lo}: {len(rows)} rows", flush=True)
                done.append(rows)
                continue
            os.remove(out)
        w = f't."{ra_col}" >= {lo} AND t."{ra_col}" < {hi}'
        cmd = ["./target/debug/tap_compiler", "--root", R, "--table", table,
               "--columns", columns, "--skip-null", skip_null,
               "--crossmatch", "I/355/gaiadr3:RA_ICRS:DE_ICRS:Dist",
               "--crossmatch-pm", "pmRA:pmDE:Plx:RV:Teff:BPmag:RPmag:Gmag",
               "--where", w, "--out", out]
        rows = fetch_band(cmd, out, lo)
        print(f"{name} RA {lo}-{hi}: {len(rows)} rows, dist={sum(1 for x in rows if x.get('dist_pc') is not None)}", flush=True)
        if not rows:
            failed += 1
        done.append(rows)
    if failed > 0:
        print(f"{name}: {failed} bands void — kein Merge, Restart holt die fehlenden", flush=True)
        return
    merged = [r for rows in done for r in rows][:CAP]
    with open(f"{CHUNK_DIR}/{name}.json", "w") as f:
        json.dump(merged, f)
    dist = sum(1 for x in merged if x.get('dist_pc') is not None)
    print(f"{name}: TOTAL {len(merged)} rows, {dist} dist_pc", flush=True)
    for (lo, _) in bands:
        p = f"{CHUNK_DIR}/{name}_c{lo}.json"
        if os.path.exists(p):
            os.remove(p)

if __name__ == "__main__":
    chunk_catalog("pastel", "B/pastel/pastel",
                  "ra:RAdeg;dec:DEdeg;teff:Teff;logg:logg;mag:Vmag", "teff", "RAdeg",
                  [(lo, lo+45) for lo in range(0, 360, 45)])
    chunk_catalog("wds", "B/wds/wds",
                  "ra:RAJ2000;dec:DEJ2000;mag1:mag1;mag2:mag2;sep:sep1", "mag1", "RAJ2000",
                  [(lo, lo+45) for lo in range(0, 360, 45)])
    chunk_catalog("mktypes", "B/mk/mktypes",
                  "ra:RAJ2000;dec:DEJ2000;mag:Mag", "mag", "RAJ2000",
                  [(lo, lo+4.5) for lo in [x*4.5 for x in range(80)]])
    chunk_catalog("denis", "B/denis/denis",
                  "ra:RAJ2000;dec:DEJ2000;imag:Imag;jmag:Jmag;kmag:Kmag", "jmag", "RAJ2000",
                  [(lo, lo+22.5) for lo in [x*22.5 for x in range(16)]])
    print("ALL DONE", flush=True)
