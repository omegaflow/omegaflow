#!/usr/bin/env python3
"""Classifies every unique token from main.rs as BODY_SPECIFIC or BODY_AGNOSTIC."""

import re
import json

FREQ_FILE = "analysis/token_frequencies.txt"
OUT = "analysis/token_classification.txt"

BODY_NAMES = {
    'earth', 'sun', 'moon', 'mars', 'venus', 'jupiter', 'saturn',
    'mercury', 'uranus', 'neptune', 'pluto', 'ssb', 'luna', 'ceres',
    'comet', 'asteroid', 'planet',
}

EARTH_CONSTANTS = {
    6378137.0, 6378136.6, 6371000.0, 6378000.0, 6356752.0,
    111319.0, 0.409092804, 280.46061837, 360.98564736629, 0.000387933,
    84381.448, 0.0167, 1.0/298.257223563,
}

EARTH_KEYWORDS = {
    'geodetic', 'geocentric', 'geoid', 'wgs84', 'gmst', 'obliquity',
    'ecef', 'eci', 'earth_radius', 'earthradius', 'era', 'eop',
    'lat', 'lon', 'alt', 'latitude', 'longitude', 'altitude',
    'nasa_key', 'demo_key', 'nasa', 'demo',
}

BODY_NEUTRAL_CONSTANTS = {
    'C', 'C_LIGHT', 'PHI', 'Φ', 'AU', 'PARSEC_M', 'HUBBLE_H0',
    'UNIX_J2000_OFFSET', 'UNIX_EPOCH', 'J2000_EPOCH',
    'MAS_YR_TO_RAD_S', 'V_SOUND_288', 'V_P_GRANITE', 'V_S_GRANITE',
    'D_AIR', 'ALPHA_AIR', 'DISPLAY_GAMUT_SATURATION',
    'STD_F64', 'F64', 'INFINITY', 'NAN',
}

PHYSICS_CONSTANTS = {
    299792458.0, 1.495978707e11, 3.085677581e16,
    946728000.0, 2451545.0, 1.618033988749895,
    4.84813681109536e-9, 31557600.0, 343.0,
    5950.0, 3630.0, 2.0e-5, 2.18e-5, 109775.0,
}

def is_body_specific(token):
    t = token.strip()

    if t.startswith('"') and t.endswith('"'):
        inner = t[1:-1]
        inner_lower = inner.lower()
        inner_words = set(inner_lower.split())
        for bn in BODY_NAMES:
            if bn in inner_words or inner_lower == bn:
                return True
        for ek in EARTH_KEYWORDS:
            if ek in inner_words:
                return True
        # Template variables containing body coordinate vars specifically
        body_templates = {'{lat}', '{lon}', '{alt}', '{x}', '{y}', '{z}',
                          '{grid}', '{grid_lat}', '{grid_lon}',
                          '{lat_int}', '{lon_int}', '{lat_min}', '{lat_max}',
                          '{lon_min}', '{lon_max}', '{nasa_key}'}
        template_vars = set()
        for part in re.findall(r'\{[^}]+\}', inner):
            template_vars.add('{' + part.split('}')[0].split('{')[-1] + '}')
        if template_vars & body_templates:
            return True
        return False

    # Numeric literals
    try:
        val = float(t)
        for ec in EARTH_CONSTANTS:
            if abs(val - ec) < abs(ec) * 1e-12:
                return True
        for pc in PHYSICS_CONSTANTS:
            if abs(val - pc) < abs(pc) * 1e-12:
                return False
        # Any other number is body-specific?
        # Small numbers (0, 1, 2, etc.) are body-agnostic
        # Large specific numbers need context
        if val == 0.0 or val == 1.0 or val == 2.0 or val == 3.0:
            return False
        if val.is_integer() and abs(val) < 1e6:
            return False
        # Number with context: check for earth-typical magnitudes
        if 6000000.0 < val < 7000000.0:  # Earth radius range
            return True
        if 100000.0 < val < 120000.0:    # meters per degree range
            return True
        return False
    except (ValueError, OverflowError):
        pass

    # Identifiers
    lower = t.lower()
    if lower in BODY_NAMES:
        return True
    for ek in EARTH_KEYWORDS:
        words = re.split(r'[^a-z0-9]', lower)
        if ek in words:
            return True
    for nc in BODY_NEUTRAL_CONSTANTS:
        if nc.lower() == lower:
            return False

    # Common body-agnostic identifiers
    if any(t.startswith(p) for p in ['fn ', 'let ', 'mut ', 'const ', 'if ', 'else',
        'for ', 'while ', 'loop ', 'match ', 'return ', 'struct ', 'enum ', 'impl ',
        'use ', 'mod ', 'pub ', 'type ', 'trait ', 'unsafe ', 'async ', 'dyn ',
        'self', 'Self', 'super', 'crate', 'extern', 'ref', 'move', 'where', 'as',
        'true', 'false', 'Some', 'None', 'Ok', 'Err', 'Result', 'Option']):
        return False

    # Punctuation and operators
    if len(t) <= 3 and all(c in '{}[]()<>.,;:=+-*/%^&|!?@#_~' for c in t):
        return False

    # Standard library
    if any(t.startswith(p) for p in ['std::', 'core::', 'Vec<', 'HashMap<', 'HashSet<',
        'String', 'Arc<', 'Mutex<', 'RwLock<', 'Condvar', 'BufReader', 'BufRead',
        'TcpStream', 'TcpListener', 'Command', 'Instant', 'SystemTime', 'Duration']):
        return False

    return False  # default: body-agnostic


def main():
    tokens = []
    with open(FREQ_FILE, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            idx = 0
            if not line.startswith('"'):
                continue
            idx = 1
            token_chars = []
            while idx < len(line):
                c = line[idx]
                if c == '\\' and idx + 1 < len(line):
                    token_chars.append(line[idx + 1])
                    idx += 2
                elif c == '"':
                    idx += 1
                    break
                else:
                    token_chars.append(c)
                    idx += 1
            else:
                continue
            token = ''.join(token_chars)
            rest = line[idx:].strip()
            m = re.match(r'^(\d+)\s+\[(.*)\]$', rest)
            if not m:
                continue
            count = int(m.group(1))
            lines = m.group(2)
            tokens.append((token, count, lines))

    body_specific = []
    body_agnostic = []

    for token, count, lines in tokens:
        if is_body_specific(token):
            body_specific.append((token, count, lines))
        else:
            body_agnostic.append((token, count, lines))

    # Resort alphabetically
    body_specific.sort(key=lambda x: x[0].upper())
    body_agnostic.sort(key=lambda x: x[0].upper())

    with open(OUT, 'w') as f:
        f.write("=== BODY-SPECIFIC TOKENS ===\n")
        f.write(f"Count: {len(body_specific)}\n\n")
        for token, count, lines in body_specific:
            f.write(f'"{token}" {count} [{lines}]\n')
        f.write(f"\n=== BODY-AGNOSTIC TOKENS ===\n")
        f.write(f"Count: {len(body_agnostic)}\n\n")
        for token, count, lines in body_agnostic:
            f.write(f'"{token}" {count} [{lines}]\n')

    print(f"Classification written to {OUT}")
    print(f"  BODY_SPECIFIC: {len(body_specific)} tokens")
    print(f"  BODY_AGNOSTIC: {len(body_agnostic)} tokens")
    print(f"  TOTAL:         {len(tokens)} tokens")


if __name__ == '__main__':
    main()
