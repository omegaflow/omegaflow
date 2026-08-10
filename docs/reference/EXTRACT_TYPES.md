# Extract Types

`src/main.rs:1900-1988` (`Extract` enum)

## Scalar extracts (outside map/cmap/rows)

| Variant | Fields |
|---------|--------|
| Field | key: String, force_unit: String |
| First | key: String, force_unit: String |
| Last | key: String, force_unit: String |
| Count | key: String, force_unit: String |
| LastRow | key: String, force_unit: String |
| LastObj | arr_path, match_field, value_field, force_unit: String |
| LastLine | force_unit: String |
| ObjLast | arr_path, force_unit: String |
| Path | key: String, force_unit: String |
| Deep | key: String, force_unit: String |
| Regex | pattern: String, force_unit: String |

## GeoJSON events

| Variant | Fields |
|---------|--------|
| GeojsonEvents | mag_key, min_mag: f64, outputs: Vec\<String\> |

## Array iteration extracts

| Variant | Fields |
|---------|--------|
| Map | arr_path, lat_key, lon_key, alt_key, epoch_key, val_key, alt_sign, vel_key, trk_key, vr_key, fields, lon_sign, lat_sign |
| CelestialMap | arr_path, ra_key, dec_key, dist_key, dist_scale, plx_key, z_key, pmra_key, pmdec_key, rv_key, rv_scale, epoch_key, fields |
| Rows | last_line: bool, fields |
| Flatten | arr_path, geom_path, epoch_key, fields |
| CmrPolygon | arr_path, fields, epoch_key, alt_key, val_key |
| CelestialPolygon | arr_path, radius, fields, epoch_key, val_key |
| KeplerMap | arr_path, a_key, e_key, i_key, om_key, w_key, ma_key, epoch_key, fields |

## Direct API extracts

| Variant | Fields |
|---------|--------|
| Hapi | parameters: Vec\<(String, String)\> |
| XmlCount | tag: String, force_unit: String |
| Ephemeris | target: String |
| Vectors | target: String |

## Source Block Parser

See `docs/concepts/SOURCES_V2_SPEC.md` for the directive-to-extract mapping.
The SOURCES_V2_SPEC is the controlling specification for source block syntax.
