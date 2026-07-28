"""Generate Saturn ring particles as ICRS ra/dec/distance JSON for cmap."""
import math, json, random

AU_KM = 149597870.7

SAT_RA_DEG = 251.75
SAT_DEC_DEG = -21.0
SAT_DIST_AU = 9.04

POLE_RA = math.radians(40.58)
POLE_DEC = math.radians(83.54)

RINGS = [
    ("D", 66900, 74658),
    ("C", 74658, 92000),
    ("B", 92000, 117580),
    ("A", 122170, 136780),
    ("F", 140180, 140300),
]

cp, sp = math.cos(POLE_RA), math.sin(POLE_RA)
cd, sd = math.cos(POLE_DEC), math.sin(POLE_DEC)
zp = [cd * cp, cd * sp, sd]

def cross(a, b):
    return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]

def norm(v):
    m = math.sqrt(v[0]*v[0]+v[1]*v[1]+v[2]*v[2])
    return [v[0]/m, v[1]/m, v[2]/m] if m > 0 else v

en = [0, 0, 1]
xp = norm(cross(en, zp))
yp = cross(zp, xp)

sr, sdr = math.radians(SAT_RA_DEG), math.radians(SAT_DEC_DEG)
sat_pos = [SAT_DIST_AU * math.cos(sdr) * math.cos(sr),
           SAT_DIST_AU * math.cos(sdr) * math.sin(sr),
           SAT_DIST_AU * math.sin(sdr)]

N_PER_ZONE = 2000
particles = []
for zone_name, r_inner_km, r_outer_km in RINGS:
    for _ in range(N_PER_ZONE):
        r_km = r_inner_km + (r_outer_km - r_inner_km) * math.sqrt(random.random())
        theta = 2 * math.pi * random.random()
        r_au = r_km / AU_KM
        p_icrs = [sat_pos[0] + r_au*math.cos(theta)*xp[0] + r_au*math.sin(theta)*yp[0],
                  sat_pos[1] + r_au*math.cos(theta)*xp[1] + r_au*math.sin(theta)*yp[1],
                  sat_pos[2] + r_au*math.cos(theta)*xp[2] + r_au*math.sin(theta)*yp[2]]
        dist = math.sqrt(p_icrs[0]**2 + p_icrs[1]**2 + p_icrs[2]**2)
        ra = math.degrees(math.atan2(p_icrs[1], p_icrs[0])) % 360
        dec = math.degrees(math.asin(p_icrs[2] / dist))
        particles.append({"ra": round(ra, 6), "dec": round(dec, 6),
                          "dist_au": round(dist, 6), "zone": zone_name})

with open('static/data/saturn_rings_10k.json', 'w') as f:
    json.dump(particles, f, separators=(',', ':'))

print(f"Generated {len(particles)} ring particles -> static/data/saturn_rings_10k.json")