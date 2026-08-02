"""Generate Saturn ring particles as ICRS StateVectors for cmap.

Each particle carries:
  ra, dec        — ICRS direction (deg)
  _dist_m        — distance from SSB (m), for dist_key 1.0
  pmra, pmdec    — proper motion (mas/yr) from Kepler orbital velocity
  optical_depth  — zone optical depth (field value)
  zone           — ring name
"""
import math, json, random

AU_M = 1.495978707e11
GM_SATURN = 3.7931207e16  # G * M_saturn (m^3/s^2)

SAT_RA_DEG = 251.75
SAT_DEC_DEG = -21.0
SAT_DIST_AU = 9.04

POLE_RA = math.radians(40.58)
POLE_DEC = math.radians(83.54)

RINGS = [
    ("D", 66900, 74658, 0.01),
    ("C", 74658, 92000, 0.05),
    ("B", 92000, 117580, 1.0),
    ("A", 122170, 136780, 0.40),
    ("F", 140180, 140300, 0.10),
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
sat_pos = [SAT_DIST_AU * math.cos(sdr) * math.cos(sr) * AU_M,
           SAT_DIST_AU * math.cos(sdr) * math.sin(sr) * AU_M,
           SAT_DIST_AU * math.sin(sdr) * AU_M]

def radec_of(v):
    d = math.sqrt(v[0]**2 + v[1]**2 + v[2]**2)
    ra = math.degrees(math.atan2(v[1], v[0])) % 360
    dec = math.degrees(math.asin(v[2] / d))
    return ra, dec, d

MAS_PER_YR = (math.pi / 180 / 3600 / 1000) / (365.25 * 86400)  # rad/s per mas/yr

N_PER_ZONE = 2048
particles = []
for zone_name, r_inner_km, r_outer_km, tau_zone in RINGS:
    for _ in range(N_PER_ZONE):
        r_km = r_inner_km + (r_outer_km - r_inner_km) * math.sqrt(random.random())
        r_m = r_km * 1000.0
        theta = 2 * math.pi * random.random()
        ct, st = math.cos(theta), math.sin(theta)
        # position in ring plane around Saturn
        p_icrs = [sat_pos[0] + r_m*ct*xp[0] + r_m*st*yp[0],
                  sat_pos[1] + r_m*ct*xp[1] + r_m*st*yp[1],
                  sat_pos[2] + r_m*ct*xp[2] + r_m*st*yp[2]]
        ra, dec, dist_m = radec_of(p_icrs)
        # Keplerian orbital speed, tangential direction in ring plane
        v_t = math.sqrt(GM_SATURN / r_m)
        tang = [-st * xp[0] + ct * yp[0],
                -st * xp[1] + ct * yp[1],
                -st * xp[2] + ct * yp[2]]
        v_vec = [v_t * tang[0], v_t * tang[1], v_t * tang[2]]
        # project velocity onto RA/Dec unit vectors -> proper motion
        ra_r, dec_r = math.radians(ra), math.radians(dec)
        a_hat = [-math.sin(ra_r), math.cos(ra_r), 0.0]
        d_hat = [-math.sin(dec_r)*math.cos(ra_r), -math.sin(dec_r)*math.sin(ra_r), math.cos(dec_r)]
        mu_a_rad = (v_vec[0]*a_hat[0] + v_vec[1]*a_hat[1] + v_vec[2]*a_hat[2]) / (dist_m * math.cos(dec_r))
        mu_d_rad = (v_vec[0]*d_hat[0] + v_vec[1]*d_hat[1] + v_vec[2]*d_hat[2]) / dist_m
        particles.append({
            "ra": round(ra, 7),
            "dec": round(dec, 7),
            "_dist_m": round(dist_m, 3),
            "pmra": round(mu_a_rad / MAS_PER_YR, 2),
            "pmdec": round(mu_d_rad / MAS_PER_YR, 2),
            "optical_depth": tau_zone,
            "zone": zone_name,
        })

with open('static/data/saturn_rings_10k.json', 'w') as f:
    json.dump({"data": particles}, f, separators=(',', ':'))

print(f"Generated {len(particles)} ring particles (power-of-2 per zone) -> static/data/saturn_rings_10k.json")
if particles:
    p = particles[0]
    print(f"Sample: ra={p['ra']} dec={p['dec']} dist={p['_dist_m']:.3e}m pmra={p['pmra']} pmdec={p['pmdec']} tau={p['optical_depth']}")
