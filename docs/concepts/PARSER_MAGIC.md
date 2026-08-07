Alright, here's the complete list of parser magic I discussed:

**1. Auto-Frame aus `lat_key`/`lon_key`** — Sources wie `opensky_aircraft_positions`, `aeic_alaska_earthquakes`, `atmosphere_blitzortung_strikes` haben `lat_key`/`lon_key` und `map` aber kein `lat`/`lon`. Der Parser sagt refused weil `has_data_position` nur für `Map`, `GeojsonEvents`, `Ephemeris`, `Vectors`, `CelestialMap`, `Flatten`, `Rows`, `KeplerMap` Extract-Types auf `true` setzt — aber `Field` Extracts haben `Frame::Data` nicht. Der Fix: wenn `lat_key` oder `lon_key` gesetzt ist, Frame=Data oder Ground setzen.

**2. `extent` pro Force-Type verbessern** — Aktuell: `force_constants` gibt `v_or_d = C_LIGHT` für EM/Gravity, `343` für acoustic, `5900` für seismic-body, etc. → `extent = v_or_d * tau`. Für EM ist `C_LIGHT * tau` gigantisch (3e8 m bei τ=1s). Der Punkt aber: ein EM-Sample sollte nicht `c * τ` als Ausdehnung haben, sondern die tatsächliche Messausdehnung oder eine sinnvolle physikalische Größe. Gravity-Bodies (Planeten) bräuchten `extent = body_radius`, nicht `c * τ`.

**3. `kepler_map` Parsing** — MPC/JPL Asteroiden-Daten: EPOCH, EC (Exzentrizität), QR (Periheldistanz), TP (Perihelzeit), OM (Ω), W (ω), IN (i), H (Magnitude), G. Aktuell gibt's den Extract-Type aber die Kepler-Element-Parser fehlen.

**4. `vectors` / Horizons Text Parser**. Der Horizons-Text-Parser (`Extract::Vectors`) ist implementiert aber die Fetches geben 0B zurück weil die URL-Templates falsche Timestamps bekommen. Der Fetch müsste die Epoche der Daten als Zeitparameter setzen, nicht `now`.

**5. `cmap` Celestial Map Parsing** — ICRS-Sternkataloge mit RA/Dec (deg), Parallaxe (mas → Distanz), Proper Motion (mas/yr → 6D Statevector), Radialgeschwindigkeit. `ra_key`/`dec_key`/`plx_key`/`pmra_key`/`pmdec_key`/`radvel_key` existieren im SourceConfig aber werden im Parser noch nicht gefüllt.

**6. `window` / Temporal Bounding** — Sources die nur innerhalb eines Zeitfensters gültige Daten haben (z.B. `starttime=2020-01-01`). Aktuell wird `{today}` im URL-Template gerendert, aber es gibt kein `from`/`until` direkt im Config.

**7. Constant `lat_key`/`lon_key Detection** — `lat_key 48.1` sollte als *Konstante* erkannt werden (lat=48.1 per Default, nicht als Spaltenname). Aktuell wird der String `48.1` als Column-Key verwendet und schlägt dann fehl weil lat/lon fehlen.

**8. `map` als Frame-Indikator** — Der `map`-Befehl definiert den JSON-Pfad zur Daten-Liste. Wenn `map` gesetzt ist UND `lat_key`/`lon_key` existieren, sollte Frame=Data automatisch folgen.

**9. `field_in` Nested Support** — `field_in <index> <name>` extrahiert array-basierte Felder nach Index. Aktuell nur für flache Arrays. Für nested arrays (`states.state[0].name`) wird's nicht richtig aufgelöst.

**10. `Flatten` Extract-Type** — Es gibt `Extract::Flatten` (line 780 `flatten_geojson_coords`) aber der Type ist kaum genutzt. Könnte generische Array-Flattener für CSV/JSON/Geojson machen.

**11. Unbekannte Force → Fallback** — Wenn `force_constants` `None` zurückgibt, wird der Source refused. Sanfter Fallback auf `em` + Warning.

**12. Kategorie/Group Inheritance** — Die φ Datei hat hierarchische Namensräume (`aeic_alaska_earthquakes`), aber keine Vererbung von defaults von übergeordneten Gruppen.

**13. Extent Zero → Default** — Samples mit `extent=0` werden vom enclosure-Filter nicht gefunden (weil `exact = extent + pad = pad`). Sollten einen minimalen Default-Extent kriegen (z.B. `scale * Φ`).
