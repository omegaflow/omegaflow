#!/usr/bin/env python3
"""Set all target names in sources.phi to native API leaf keys.
field <path> <target> -> field <path> <leaf(path)>
last <key> <target> -> last <key> <key>
last_row <col> <target> -> last_row <col> <col>
All references (pos, lat_key, lon_key, etc.) updated to match.
Regex targets are synthetic and kept as-is.
"""
import sys
from pathlib import Path

ROOT = Path(__file__).parent.parent
SRC = ROOT / "phi" / "sources.φ"

def leaf_key(path):
    return path.rsplit(".", 1)[-1]

EXTRACTORS = {"field", "last", "last_row", "first", "path", "field_in"}
REF_KEYS = {"pos", "lat_key", "lon_key", "alt_key", "vel_key", "trk_key", "vr_key",
            "ra_key", "dec_key", "plx_key", "pmra_key", "pmdec_key",
            "dist_key", "z_key", "radvel_key", "tau_key"}

text = SRC.read_text(encoding="utf-8")
blocks = text.split("\n\n")
new_blocks = []

for block in blocks:
    if not block.strip():
        new_blocks.append(block)
        continue
    lines = block.strip().split("\n")
    mapping = {}  # old_target -> new_target
    new_lines = []
    for line in lines:
        parts = line.split(None, 1)
        if not parts:
            new_lines.append(line)
            continue
        key = parts[0]
        value = parts[1] if len(parts) > 1 else ""
        if key in EXTRACTORS:
            vals = value.split()
            if len(vals) >= 2:
                src_path = vals[0]
                old_target = vals[1]
                new_target = leaf_key(src_path) if key != "regex" else old_target
                mapping[old_target] = new_target
                vals[1] = new_target
                new_lines.append(f"{key} {' '.join(vals)}")
                continue
        elif key in REF_KEYS:
            vals = value.split()
            new_vals = [mapping.get(v, v) for v in vals]
            new_lines.append(f"{key} {' '.join(new_vals)}")
            continue
        new_lines.append(line)
    new_blocks.append("\n".join(new_lines))

result = "\n\n".join(new_blocks)
if result != text:
    changes = len(text) - len(result)
    SRC.write_text(result, encoding="utf-8")
    print(f"Written. {changes:+d} bytes delta.")
else:
    print("No changes.")
