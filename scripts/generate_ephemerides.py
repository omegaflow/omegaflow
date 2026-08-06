import struct, subprocess, sys, numpy as np, urllib.request, urllib.parse, json, ssl, math, re

try:
    import spiceypy as spice
except ImportError:
    spice = None

GRANULE_DAYS = 32.0
DEGREE = 17
N_COEFF = DEGREE + 1
N_SAMPLES = 25
J2000 = 2451545.0
MAGIC = b'\xCF\x86\x01\x00'
ECLIPTIC_OBLIQUITY = 0.409092804


def _ecliptic_to_icrs(x, y, z):
    c = math.cos(ECLIPTIC_OBLIQUITY)
    s = math.sin(ECLIPTIC_OBLIQUITY)
    return x, y * c - z * s, y * s + z * c

KERNELS = [
    ('de440.bsp', [
        (1, 'mercury'), (2, 'venus'), (399, 'earth'), (301, 'moon'),
        (4, 'mars'), (5, 'jupiter'), (6, 'saturn'), (7, 'uranus'),
        (8, 'neptune'), (9, 'pluto'),
    ]),
    ('jup365.bsp', [
        (501, 'io'), (502, 'europa'), (503, 'ganymede'), (504, 'callisto'),
    ]),
    ('sat441.bsp', [
        (602, 'enceladus'), (603, 'rhea'), (604, 'dione'), (605, 'tethys'),
        (606, 'titan'),
    ]),
    ('mar099s.bsp', [
        (401, 'phobos'), (402, 'deimos'),
    ]),
    ('nep097.bsp', [
        (801, 'triton'),
    ]),
]

WGCCRE = {
    10: ('sun', 286.13, 0.0, 63.87, 0.0, 84.176, 14.1844, 696000000.0, 0.0),
    199: ('mercury', 281.01, -0.033, 61.45, -0.005, 329.548, 6.1385, 2439700.0, 0.0),
    299: ('venus', 272.76, 0.0, 67.16, 0.0, 160.20, -1.4814, 6051800.0, 0.0),
    399: ('earth', 0.0, 0.0, 90.0, 0.0, 190.147, 360.9856235, 6378136.6, 0.0033527),
    301: ('moon', 269.9949, 0.0031, 66.5392, 0.013, 38.3213, 13.17635815, 1737400.0, 0.0),
    4: ('mars', 317.68143, -0.1061, 52.88650, -0.0609, 176.630, 350.89198226, 3396190.0, 0.00589),
    5: ('jupiter', 268.056595, -0.006499, 64.495303, 0.002413, 284.95, 870.536, 71492000.0, 0.06487),
    6: ('saturn', 40.589, -0.036, 83.537, -0.004, 38.90, 810.7939024, 60268000.0, 0.09796),
    7: ('uranus', 257.311, 0.0, -15.175, 0.0, 203.81, -501.1600928, 25559000.0, 0.02293),
    8: ('neptune', 299.36, 0.70, 43.46, -0.51, 253.18, 536.3128492, 24764000.0, 0.0171),
    9: ('pluto', 132.993, 0.0, -6.163, 0.0, 302.695, 56.3625225, 1188300.0, 0.0),
    501: ('io', 268.05, -0.009, 64.50, 0.003, 200.39, 203.4889538, 1821600.0, 0.0),
    502: ('europa', 268.08, -0.009, 64.51, 0.003, 35.98, 101.3747235, 1560800.0, 0.0),
    503: ('ganymede', 268.20, -0.009, 64.57, 0.003, 44.064, 50.3176081, 2631200.0, 0.0),
    504: ('callisto', 268.72, -0.009, 64.83, 0.003, 259.51, 21.5710715, 2410300.0, 0.0),
    606: ('titan', 36.41, -0.036, 83.94, -0.004, 189.64, 22.5769768, 2575500.0, 0.0),
    801: ('triton', 299.36, 0.70, 43.46, -0.51, 296.53, -61.2572637, 1353400.0, 0.0),
    602: ('enceladus', 40.66, -0.036, 83.52, -0.004, 36.41, 262.7318996, 252100.0, 0.0),
    603: ('rhea', 40.38, -0.036, 83.55, -0.004, 345.65, 79.6900478, 763800.0, 0.0),
    604: ('dione', 40.66, -0.036, 83.52, -0.004, 357.00, 131.5349316, 561400.0, 0.0),
    605: ('tethys', 50.41, -0.036, 83.55, -0.004, 299.11, 190.6979086, 531100.0, 0.0),
    401: ('phobos', 317.68, -0.108, 54.46, -0.061, 165.00, 1128.844759, 11260.0, 0.0),
    402: ('deimos', 317.68, -0.108, 54.46, -0.061, 240.00, 285.161891, 6230.0, 0.0),
}

MEDIA = {
    10:   ('sun',       11557.5,  0.0,    0.0,    0.0,     0.0,     400000.0),
    199:  ('mercury',   515.4,    5200.0, 3000.0, 0.0,     0.0,     0.0),
    299:  ('venus',     433.1,    6200.0, 3600.0, 7.92e-7, 7.27e-7, 1.0),
    399:  ('earth',     340.2,    5950.0, 3630.0, 2.18e-5, 2.00e-5, 15.0),
    301:  ('moon',      355.3,    5000.0, 2900.0, 0.0,     0.0,     0.0),
    4:    ('mars',      231.5,    5400.0, 3200.0, 1.74e-3, 1.60e-3, 30.0),
    5:    ('jupiter',   933.5,    0.0,    0.0,    3.42e-5, 3.13e-5, 150.0),
    6:    ('saturn',    871.2,    0.0,    0.0,    2.59e-5, 2.37e-5, 400.0),
    7:    ('uranus',    580.9,    0.0,    0.0,    9.79e-6, 8.98e-6, 250.0),
    8:    ('neptune',   568.7,    0.0,    0.0,    9.08e-6, 8.33e-6, 580.0),
    9:    ('pluto',     125.7,    0.0,    0.0,    8.18e-2, 7.50e-2, 5.0),
    501:  ('io',        137.8,    5800.0, 3400.0, 0.0,     0.0,     0.0),
    502:  ('europa',    208.3,    3800.0, 1900.0, 0.0,     0.0,     0.0),
    503:  ('ganymede',  218.5,    3800.0, 1900.0, 0.0,     0.0,     0.0),
    504:  ('callisto',  228.2,    3800.0, 1900.0, 0.0,     0.0,     0.0),
    606:  ('titan',     197.7,    3800.0, 1900.0, 2.85e-6, 2.62e-6, 1.0),
    801:  ('triton',    125.7,    2800.0, 1400.0, 7.59e-2, 6.96e-2, 5.0),
    602:  ('enceladus', 0.0,      3800.0, 1900.0, 0.0,     0.0,     0.0),
    603:  ('rhea',      0.0,      3800.0, 1900.0, 0.0,     0.0,     0.0),
    604:  ('dione',     0.0,      3800.0, 1900.0, 0.0,     0.0,     0.0),
    605:  ('tethys',    0.0,      3800.0, 1900.0, 0.0,     0.0,     0.0),
    401:  ('phobos',    0.0,      3200.0, 1800.0, 0.0,     0.0,     0.0),
    402:  ('deimos',    0.0,      3200.0, 1800.0, 0.0,     0.0,     0.0),
}


def fit_granule(kernel_path, body_id, t0_jd, half_jd):
    spampled = np.linspace(-1, 1, N_SAMPLES)
    xs, ys, zs = [], [], []
    for tau in spampled:
        t_jd = t0_jd + tau * half_jd
        t_sec = (t_jd - J2000) * 86400.0
        state, _ = spice.spkezr(str(body_id), t_sec, "J2000", "NONE", "0")
        xs.append(state[0] * 1000.0)
        ys.append(state[1] * 1000.0)
        zs.append(state[2] * 1000.0)
    cx = np.polynomial.chebyshev.chebfit(spampled, xs, DEGREE)
    cy = np.polynomial.chebyshev.chebfit(spampled, ys, DEGREE)
    cz = np.polynomial.chebyshev.chebfit(spampled, zs, DEGREE)
    return cx, cy, cz


def compute_rotation_matrix(body_id, body_name, t_jd):
    t_sec = (t_jd - J2000) * 86400.0
    frame_name = "IAU_" + body_name.upper()
    try:
        px = spice.pxform(frame_name, "J2000", t_sec)
    except Exception:
        return None
    return [
        px[0, 0], px[0, 1], px[0, 2],
        px[1, 0], px[1, 1], px[1, 2],
        px[2, 0], px[2, 1], px[2, 2],
    ]


def generate_rotation_matrices(kernel_path, body_id, body_name, granules):
    spice.furnsh(kernel_path)
    matrices = []
    for t0_jd, _, _, _, _ in granules:
        m = compute_rotation_matrix(body_id, body_name, t0_jd)
        if m is not None:
            matrices.append((t0_jd, m))
    spice.unload(kernel_path)
    return matrices


def generate(kernel_path, body_id, body_name):
    spice.furnsh(kernel_path)
    cover = spice.spkcov(kernel_path, body_id)
    granules = []
    for i in range(cover.card // 2):
        t0_sec = cover[i*2]
        t1_sec = cover[i*2+1]
        dur_sec = t1_sec - t0_sec
        n_granules = int(dur_sec / (GRANULE_DAYS * 86400.0))
        for i in range(n_granules):
            mid_sec = t0_sec + (i + 0.5) * GRANULE_DAYS * 86400.0
            mid_jd = mid_sec / 86400.0 + J2000
            half_jd = GRANULE_DAYS / 2.0
            cx, cy, cz = fit_granule(kernel_path, body_id, mid_jd, half_jd)
            granules.append((mid_jd, half_jd, cx, cy, cz))
    spice.unload(kernel_path)
    if not granules:
        return None
    return granules


def write_binary(granules, body_name, body_id, rotation_matrices=None):
    wgccre_entry = next((v for k, v in WGCCRE.items() if v[0] == body_name), None)
    media_entry = next((v for k, v in MEDIA.items() if v[0] == body_name), None)
    n_sections = 1
    if wgccre_entry is not None:
        n_sections += 1
    if media_entry is not None:
        n_sections += 1
    if rotation_matrices:
        n_sections += 1
    buf = bytearray()
    buf.extend(MAGIC)
    buf.extend(struct.pack('<I', n_sections))
    buf.extend(struct.pack('<I', 0))
    buf.extend(struct.pack('<I', len(granules)))
    buf.extend(struct.pack('<I', DEGREE))
    buf.extend(struct.pack('<I', 0))
    for t0, dt, cx, cy, cz in granules:
        buf.extend(struct.pack('<d', t0))
        buf.extend(struct.pack('<d', dt))
        for c in cx: buf.extend(struct.pack('<d', c))
        for c in cy: buf.extend(struct.pack('<d', c))
        for c in cz: buf.extend(struct.pack('<d', c))
    if wgccre_entry is not None:
        _, a0, da0, d0, dd0, w0, dw, r, f = wgccre_entry
        buf.extend(struct.pack('<I', 1))
        buf.extend(struct.pack('<I', 0))
        buf.extend(struct.pack('<I', 17))
        buf.extend(struct.pack('<I', 0))
        buf.extend(struct.pack('<d', a0))
        buf.extend(struct.pack('<d', da0))
        buf.extend(struct.pack('<d', d0))
        buf.extend(struct.pack('<d', dd0))
        buf.extend(struct.pack('<d', w0))
        buf.extend(struct.pack('<d', dw))
        buf.extend(struct.pack('<d', r))
        buf.extend(struct.pack('<d', f))
    if media_entry is not None:
        _, vs, vp, vsseis, ath, dd, vad = media_entry
        buf.extend(struct.pack('<I', 2))
        buf.extend(struct.pack('<I', 0))
        buf.extend(struct.pack('<I', 17))
        buf.extend(struct.pack('<I', 0))
        buf.extend(struct.pack('<d', vs))
        buf.extend(struct.pack('<d', vp))
        buf.extend(struct.pack('<d', vsseis))
        buf.extend(struct.pack('<d', ath))
        buf.extend(struct.pack('<d', dd))
        buf.extend(struct.pack('<d', vad))
    if rotation_matrices:
        buf.extend(struct.pack('<I', 3))
        buf.extend(struct.pack('<I', len(rotation_matrices)))
        buf.extend(struct.pack('<I', 8))
        buf.extend(struct.pack('<I', 0))
        for t0, m in rotation_matrices:
            buf.extend(struct.pack('<d', t0))
            for v in m:
                buf.extend(struct.pack('<d', v))
    return bytes(buf)


def upload(path):
    subprocess.run(['gh', 'release', 'upload', 'ssd.jpl.nasa.gov', path,
                    '--clobber', '--repo', 'omegaflow/sources'], check=False)


HORIZONS_BASE = "https://ssd.jpl.nasa.gov/api/horizons.api"

HORIZONS_BODIES_STABLE = [
    ("Ceres;", "ceres"),
    ("Vesta", "vesta"),
    ("Eris;", "eris"),
    ("Haumea;", "haumea"),
    ("Makemake;", "makemake"),
    ("Apophis", "apophis"),
    ("Bennu", "bennu"),
    ("90000031", "encke"),
]

HORIZONS_BODIES_DYNAMIC = [
    ("-125544", "iss"),
    ("-31", "voyager1"),
    ("-32", "voyager2"),
    ("-98", "new_horizons"),
    ("-96", "parker_solar_probe"),
    ("-144", "solar_orbiter"),
    ("-170", "jwst"),
    ("-61", "juno"),
    ("2020-047A", "atlas_3i"),
]

HORIZONS_BODIES = HORIZONS_BODIES_STABLE + HORIZONS_BODIES_DYNAMIC

HORIZONS_RETRY = []


def _horizons_request(command, t_start, t_stop, step_days=1):
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    query = (
        f"format=json"
        f"&COMMAND='{urllib.parse.quote(command)}'"
        f"&CENTER='500@0'"
        f"&MAKE_EPHEM='YES'"
        f"&EPHEM_TYPE='VECTORS'"
        f"&START_TIME='JD+{t_start:.2f}'"
        f"&STOP_TIME='JD+{t_stop:.2f}'"
        f"&STEP_SIZE='{step_days}+d'"
    )
    url = f"{HORIZONS_BASE}?{query}"
    req = urllib.request.Request(url, headers={"User-Agent": "omegaflow-ci/1.0"})
    with urllib.request.urlopen(req, timeout=120, context=ctx) as r:
        return json.loads(r.read().decode(errors="replace"))


def _extract_vectors(data):
    vectors = []
    result_str = data.get("result", "")
    in_block = False
    current_jd = None
    for line in result_str.split("\n"):
        line = line.strip()
        if line.startswith("$$SOE"):
            in_block = True
            continue
        if line.startswith("$$EOE"):
            break
        if not in_block or not line:
            continue
        if line.startswith("VX=") or line.startswith("LT="):
            continue
        eq_pos = line.find("=")
        if eq_pos < 0:
            continue
        before_eq = line[:eq_pos].strip()
        try:
            jd = float(before_eq)
            current_jd = jd
        except ValueError:
            pass
        if current_jd is not None and "X =" in line.lstrip():
            m = re.match(r'.*X\s*=\s*([-+]?\S+)\s+Y\s*=\s*([-+]?\S+)\s+Z\s*=\s*([-+]?\S+)', line)
            if m:
                try:
                    x = float(m.group(1)) * 1000.0
                    y = float(m.group(2)) * 1000.0
                    z = float(m.group(3)) * 1000.0
                    x, y, z = _ecliptic_to_icrs(x, y, z)
                    vectors.append((current_jd, x, y, z))
                except ValueError:
                    continue
    return vectors


def _fit_granule_from_samples(samples, t0_jd, half_jd):
    spampled = np.linspace(-1, 1, N_SAMPLES)
    sample_times = np.array([s[0] for s in samples])
    xs = np.array([s[1] for s in samples])
    ys = np.array([s[2] for s in samples])
    zs = np.array([s[3] for s in samples])
    if len(sample_times) < 4:
        return None
    interp_x_vals = np.interp(t0_jd + spampled * half_jd, sample_times, xs)
    interp_y_vals = np.interp(t0_jd + spampled * half_jd, sample_times, ys)
    interp_z_vals = np.interp(t0_jd + spampled * half_jd, sample_times, zs)
    cx = np.polynomial.chebyshev.chebfit(spampled, interp_x_vals, DEGREE)
    cy = np.polynomial.chebyshev.chebfit(spampled, interp_y_vals, DEGREE)
    cz = np.polynomial.chebyshev.chebfit(spampled, interp_z_vals, DEGREE)
    return cx, cy, cz


def generate_from_horizons(command, body_name, months=12, lookback=30):
    jd_now = _current_jd()
    t_start = jd_now - lookback
    t_stop = jd_now + months * 30.44
    data = _horizons_request(command, t_start, t_stop, step_days=1)
    vectors = _extract_vectors(data)
    if len(vectors) < 10:
        print(f"  SKIP {body_name}: only {len(vectors)} vectors", file=sys.stderr)
        return []
    granules = []
    n_granules = int((t_stop - t_start) / GRANULE_DAYS)
    for i in range(n_granules):
        mid_jd = t_start + (i + 0.5) * GRANULE_DAYS
        half_jd = GRANULE_DAYS / 2.0
        fitted = _fit_granule_from_samples(vectors, mid_jd, half_jd)
        if fitted is None:
            continue
        cx, cy, cz = fitted
        granules.append((mid_jd, half_jd, cx, cy, cz))
    return granules


def _current_jd():
    import datetime
    now = datetime.datetime.now(datetime.timezone.utc)
    j2000 = datetime.datetime(2000, 1, 1, 12, 0, 0, tzinfo=datetime.timezone.utc)
    delta = now - j2000
    return 2451545.0 + delta.total_seconds() / 86400.0


def main():
    for kernel_file, bodies in KERNELS:
        for body_id, body_name in bodies:
            if spice is None:
                print(f"  SKIP {body_name}: spiceypy not installed", file=sys.stderr)
                continue
            granules = generate(kernel_file, body_id, body_name)
            if not granules:
                print(f"  SKIP {body_name}: no granules", file=sys.stderr)
                continue
            rot_mats = generate_rotation_matrices(kernel_file, body_id, body_name, granules)
            data = write_binary(granules, body_name, body_id, rotation_matrices=rot_mats)
            path = f'/tmp/ephemeris_{body_name}.bin'
            with open(path, 'wb') as f:
                f.write(data)
            upload(path)
            print(f"  {body_name}: {len(granules)} granules, {len(data)} B", file=sys.stderr)

    for command, body_name in HORIZONS_BODIES_STABLE:
        try:
            granules = generate_from_horizons(command, body_name, months=12)
        except Exception as e:
            print(f"  SKIP {body_name}: Horizons error: {e}", file=sys.stderr)
            continue
        if not granules:
            print(f"  SKIP {body_name}: no granules from Horizons", file=sys.stderr)
            continue
        data = write_binary(granules, body_name, 0)
        path = f'/tmp/ephemeris_{body_name}.bin'
        with open(path, 'wb') as f:
            f.write(data)
        upload(path)
        print(f"  {body_name}: {len(granules)} granules, {len(data)} B", file=sys.stderr)

    for command, body_name in HORIZONS_BODIES_DYNAMIC:
        try:
            months = 0.9 if body_name == "iss" else 1
            granules = generate_from_horizons(command, body_name, months=months, lookback=5)
        except Exception as e:
            print(f"  SKIP {body_name}: Horizons error: {e}", file=sys.stderr)
            continue
        if not granules:
            print(f"  SKIP {body_name}: no granules from Horizons", file=sys.stderr)
            continue
        data = write_binary(granules, body_name, 0)
        path = f'/tmp/ephemeris_{body_name}.bin'
        with open(path, 'wb') as f:
            f.write(data)
        upload(path)
        print(f"  {body_name}: {len(granules)} granules, {len(data)} B", file=sys.stderr)

    for command, body_name in HORIZONS_RETRY:
        try:
            granules = generate_from_horizons(command, body_name, months=1)
            if not granules:
                granules = generate_from_horizons(f"{command};", body_name, months=1)
        except Exception as e:
            print(f"  SKIP {body_name}: Horizons error: {e}", file=sys.stderr)
            continue
        if not granules:
            print(f"  SKIP {body_name}: no granules from Horizons", file=sys.stderr)
            continue
        data = write_binary(granules, body_name, 0)
        path = f'/tmp/ephemeris_{body_name}.bin'
        with open(path, 'wb') as f:
            f.write(data)
        upload(path)
        print(f"  {body_name}: {len(granules)} granules, {len(data)} B", file=sys.stderr)

    sun_data = write_binary([], 'sun', 10)
    path = '/tmp/ephemeris_sun.bin'
    with open(path, 'wb') as f:
        f.write(sun_data)
    upload(path)
    print(f"  sun: 0 granules, {len(sun_data)} B", file=sys.stderr)


if __name__ == '__main__':
    main()
