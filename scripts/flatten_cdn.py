#!/usr/bin/env python3
"""Flatten API responses to universal CDN format.
Produces {"data": [{"lat": ..., "lon": ..., "val": ...}]} for terrestrial,
or {"data": [{"ra": ..., "dec": ..., ...}]} for celestial.
"""
import json
import re

def flatten_to_universal(raw_bytes, source_name=""):
    """Convert any API response to flat universal JSON format.
    Returns (bytes, format_hint) where format_hint is 'celestial' or 'terrestrial'.
    """
    text = raw_bytes.decode(errors='replace').strip()
    if not text:
        return json.dumps({"data": []}).encode(), "terrestrial"
    
    # Try JSON first
    try:
        data = json.loads(text)
        return _flatten_json(data, source_name)
    except (json.JSONDecodeError, ValueError):
        pass
    
    # Try text/CSV
    return _flatten_text(text, source_name)

def _flatten_json(data, source_name):
    """Flatten JSON data to universal format."""
    # Already in universal format?
    if isinstance(data, dict) and "data" in data and isinstance(data["data"], list):
        return json.dumps(data).encode(), _detect_format(data["data"])
    
    # Array of objects
    if isinstance(data, list) and len(data) > 0 and isinstance(data[0], dict):
        flat = [_normalize_keys(item) for item in data]
        return json.dumps({"data": flat}).encode(), _detect_format(flat)
    
    # Object with known array keys
    if isinstance(data, dict):
        for key in ("data", "results", "features", "rows", "items", "records", "stations"):
            if key in data and isinstance(data[key], list):
                arr = data[key]
                if len(arr) > 0 and isinstance(arr[0], dict):
                    # Normalize known field names
                    flat = []
                    for item in arr:
                        flat.append(_normalize_keys(item))
                    return json.dumps({"data": flat}).encode(), _detect_format(flat)
        
        # Single object — wrap in array
        flat_obj = _normalize_keys(data)
        return json.dumps({"data": [flat_obj]}).encode(), _detect_format([flat_obj])
    
    # Array of primitives — can't extract fields
    if isinstance(data, list):
        return json.dumps({"data": [{"val": v} for v in data if isinstance(v, (int, float))]}).encode(), "terrestrial"
    
    # Fallback: wrap raw value
    return json.dumps({"data": [{"val": str(data)}]}).encode(), "terrestrial"

def _flatten_text(text, source_name):
    """Parse text/CSV to flat JSON."""
    lines = [l.strip() for l in text.split('\n') if l.strip() and not l.strip().startswith('#')]
    if not lines:
        return json.dumps({"data": []}).encode(), "terrestrial"
    
    # Try to find a header row
    header = None
    data_start = 0
    for i, line in enumerate(lines):
        # Look for comma/whitespace-separated values
        parts = _split_line(line)
        if len(parts) >= 2:
            # Check if this looks like a header (contains non-numeric values)
            numeric = sum(1 for p in parts if _is_numeric(p))
            if numeric < len(parts) * 0.5:  # Less than half numeric → probably header
                header = [p.strip('"\' ') for p in parts]
                data_start = i + 1
                break
    
    if header is None:
        # No header — use col0, col1, ...
        first_parts = _split_line(lines[0])
        header = [f"col{i}" for i in range(len(first_parts))]
        data_start = 0
    
    # Parse data rows
    rows = []
    for line in lines[data_start:]:
        parts = _split_line(line)
        if len(parts) < 2:
            # Single value line
            if _is_numeric(parts[0] if parts else ""):
                rows.append({"val": float(parts[0])})
            continue
        row = {}
        for i, val in enumerate(parts):
            if i >= len(header):
                continue
            key = header[i]
            if _is_numeric(val):
                row[key] = float(val)
            else:
                row[key] = val.strip('"\' ')
        if row:
            rows.append(row)
    
    if not rows:
        return json.dumps({"data": []}).encode(), "terrestrial"
    
    return json.dumps({"data": rows}).encode(), _detect_format(rows)

def _split_line(line):
    """Split a data line by common delimiters."""
    if '\t' in line and line.count('\t') >= 2:
        return line.split('\t')
    if '|' in line and line.count('|') >= 2:
        return [p.strip() for p in line.split('|') if p.strip()]
    if ',' in line:
        return [p.strip() for p in line.split(',')]
    return line.split()

def _is_numeric(s):
    """Check if string is numeric."""
    try:
        float(s.strip().strip('"\''))
        return True
    except (ValueError, TypeError):
        return False

def _normalize_keys(obj):
    """Normalize common key names and flatten nested coordinates to universal format."""
    if not isinstance(obj, dict):
        return obj
    
    result = {}
    
    # Known nested patterns to extract
    nested_patterns = [
        # GeoJSON geometry
        ("geometry", lambda v: _extract_geojson_coords(v, result)),
        # Common centroid patterns
        ("centroid_coordinates", lambda v: _extract_centroid(v, result)),
        ("centroid", lambda v: _extract_centroid(v, result)),
        ("coordinates", lambda v: _extract_coords(v, result)),
        ("location", lambda v: _extract_location(v, result)),
        ("position", lambda v: _extract_coords(v, result)),
        ("center", lambda v: _extract_centroid(v, result)),
        ("geolocation", lambda v: _extract_geolocation(v, result)),
    ]
    
    for k, v in obj.items():
        handled = False
        for pattern, extractor in nested_patterns:
            if k == pattern and isinstance(v, dict):
                extractor(v)
                handled = True
                break
        if not handled:
            mapped = KEY_MAP.get(k, k)
            # Recursively normalize nested dicts
            if isinstance(v, dict):
                result[mapped] = _normalize_keys(v)
            else:
                result[mapped] = v
    
    return result

def _extract_geojson_coords(geom, result):
    if isinstance(geom, dict) and "coordinates" in geom:
        coords = geom["coordinates"]
        if isinstance(coords, list) and len(coords) >= 2:
            result["lon"] = coords[0]
            result["lat"] = coords[1]
            if len(coords) >= 3:
                result["alt"] = coords[2]

def _extract_centroid(obj, result):
    for key in ("lat", "latitude"):
        if key in obj:
            result["lat"] = obj[key]
            break
    for key in ("lon", "longitude", "lng"):
        if key in obj:
            result["lon"] = obj[key]
            break

def _extract_coords(obj, result):
    if "x" in obj: result["lon"] = obj["x"]
    if "y" in obj: result["lat"] = obj["y"]
    if "z" in obj: result["alt"] = obj["z"]
    for key in ("lat", "latitude"):
        if key in obj: result["lat"] = obj[key]
    for key in ("lon", "longitude", "lng"):
        if key in obj: result["lon"] = obj[key]
    for key in ("alt", "altitude", "elevation"):
        if key in obj: result["alt"] = obj[key]

def _extract_location(obj, result):
    _extract_coords(obj, result)
    for key in ("lat", "latitude", "lon", "longitude"):
        if key in obj: result[key] = obj[key]

def _extract_geolocation(obj, result):
    _extract_coords(obj, result)

# Known key mappings
KEY_MAP = {
    'latitude': 'lat', 'LAT': 'lat', 'LATITUDE': 'lat', 'site_lat': 'lat',
    'Site_Latitude(Degrees)': 'lat', 'location_lat': 'lat', 'lat_deg': 'lat',
    'longitude': 'lon', 'LON': 'lon', 'LONGITUDE': 'lon', 'site_lon': 'lon',
    'Site_Longitude(Degrees)': 'lon', 'location_lon': 'lon', 'lon_deg': 'lon', 'long': 'lon',
    'RA': 'ra', 'right_ascension': 'ra',
    'DEC': 'dec', 'declination': 'dec',
    'altitude': 'alt', 'elevation': 'alt', 'elev': 'alt', 'depth': 'alt',
    'time': 't', 'timestamp': 't', 'date': 't', 'epoch': 't',
    'value': 'val', 'magnitude': 'val', 'mag': 'val', 'count': 'val',
    'intensity': 'val', 'flux': 'val', 'temperature': 'val', 'temp': 'val',
}

def _detect_format(rows):
    """Detect if data is celestial (ra/dec) or terrestrial (lat/lon)."""
    if not rows or not isinstance(rows[0], dict):
        return "terrestrial"
    first = rows[0]
    if "ra" in first and "dec" in first:
        return "celestial"
    return "terrestrial"
