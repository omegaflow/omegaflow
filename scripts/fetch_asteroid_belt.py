"""Fetch orbital elements for 10,000 main-belt asteroids from JPL SBDB.
Saves as JSON for kepler_map source."""
import json, urllib.request, time, math

API = "https://ssd-api.jpl.nasa.gov/sbdb_query.api"
QUERY = ("select full_name,a,e,i,om,w,ma,epoch,moid,class "
         "from sbdb where a > 2.0 and a < 3.5 and orbit_class = 'MBA' "
         "order by abs(a - 2.5) limit 10000")

url = f"{API}?query={urllib.request.quote(QUERY)}&format=json"
print(f"Fetching: {url[:120]}...")
resp = urllib.request.urlopen(url, timeout=60)
data = json.loads(resp.read().decode())

rows = data.get('data', [])
fields = [c['name'] for c in data.get('fields', [])]

print(f"Fields: {fields}")
print(f"Rows returned: {len(rows)}")

particles = []
for row in rows:
    d = dict(zip(fields, row))
    a = d.get('a')
    e = d.get('e')
    i = d.get('i')
    om = d.get('om')
    w = d.get('w')
    ma = d.get('ma')
    epoch = d.get('epoch')
    name = d.get('full_name', '')
    try:
        entry = {
            "a": round(float(a), 6) if a else 0,
            "e": round(float(e), 6) if e else 0,
            "i": round(float(i), 4) if i else 0,
            "om": round(float(om), 4) if om else 0,
            "w": round(float(w), 4) if w else 0,
            "ma": round(float(ma), 4) if ma else 0,
            "epoch": round(float(epoch), 2) if epoch else 2460000.5,
            "name": name if name else '',
        }
        if entry['a'] > 0 and entry['epoch'] > 0:
            particles.append(entry)
    except (ValueError, TypeError):
        continue

print(f"Valid particles: {len(particles)}")

with open('static/data/asteroid_belt_10k.json', 'w') as f:
    json.dump(particles, f, separators=(',', ':'))

print(f"Saved: static/data/asteroid_belt_10k.json ({len(particles)} asteroids)")
PYEOF