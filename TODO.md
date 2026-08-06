# TODO — Quellentreue & CDN-Restructure

## Ausgangszustand

- 2066 Quellen, 0 refused, 0 Warnungen. `cargo run` lädt alles sauber.
- `phi/sources.φ`: url-first Blöcke, Leerzeilen-Trenner, 1:1 `phi/live_url_map.json` (2066 Einträge).
- Parser: `url` als Anker (`flush!()` bei nächstem `url`), keyword-basiert, `body`-Pflicht.
- CDN-Releases: Assets haben NOCH alte Namen (`hydrosphere_ndbc_buoy_46084.json`).
- sources.φ URLs haben `{latest}`-Suffix — aber der Archivar löst `{latest}` nicht auf und die CDN-Assets heißen anders.
- 40 Ephemeriden-Binaries in `ssd.jpl.nasa.gov` Release (via `generate-ephemerides.yml`).

---

## 1. `{latest}`-URLs reparieren (blockiert alles)

**Problem:** `sources.φ` URLs haben `{name}_{latest}.json`, aber die CDN-Assets heißen noch `{old_name}.json` (mit Sphere-Prefix). Der Archivar hat keinen `{latest}`-Resolver.

**Aktion:**
- Revertiere `{latest}` in `sources.φ` zurück auf alte Asset-Namen. Commit `395be55` hat `_{latest}` eingeführt — nur die URL-Änderung rückgängig machen, nicht die anderen Änderungen aus diesem Commit.
- `cargo run` → 2066 loaded, 0 refused.
- Commit: `Revert {latest} URLs — CDN assets not yet renamed`

**Verifikation:** `grep -c '{latest}' phi/sources.φ` → 0.

---

## 2. CI-Scripts auf url-first-Parsing umstellen

**Problem:** `tap_to_cdn.py`, `refresh_catalogs.py`, `restore_all_live.py` parsen sources.φ mit `source <name>`-Zeilen als Block-Anker. Die aktuelle Datei ist url-first.

**Aktion — `scripts/tap_to_cdn.py` (Zeile 52-56):**
```python
# ALT:
blocks = re.split(r"\n(?=source )", content)
m = re.match(r"source (\S+)", b)
```
```python
# NEU:
blocks = content.strip().split('\n\n')
m = re.match(r"url .*releases/download/.*/(\S+)\.(json|bin)", b)
```

**Aktion — `scripts/refresh_catalogs.py`:**
- Gleiche Logik: Block-Trennung statt `source`-Zeilen.
- Name aus URL-Pfad ableiten.

**Aktion — `scripts/restore_all_live.py`:**
- `_current_names_from_urls()`: strips `{latest}` bereits (Zeile ~85). Nach Schritt 1 ist das nicht mehr nötig.
- `_load_cdn_migration_urls()`: Iteriert Diff-Zeilen. Prüfen ob `+url`-Zeilen (Restore-Commits) auch erfasst werden.

**Verifikation:**
- `python3 scripts/tap_to_cdn.py --dry-run` → findet Quellen, nicht 0.
- `python3 scripts/migrate_live_to_cdn.py --mode mirror-all --dry-run --limit 10` → funktioniert.

---

## 3. CDN-Assets quellentreu umbenennen (in CI, nicht lokal!)

**Problem:** Assets heißen `hydrosphere_ndbc_buoy_46084.json`, sollen `46083_20260806T120000Z.json` heißen (1:1 was die Quelle liefert + ISO-8601 Zeitstempel).

**Regel:**
- Release-Tag = netloc (z.B. `ndbc.noaa.gov`)
- Asset = `{api_delivered_filename}_{iso8601_utc}.json`
- Immutable: jeder CI-Lauf erzeugt ein neues Asset mit neuem Zeitstempel. Kein `--clobber`.

**Aktion — CI-Workflow `rename-cdn-assets.yml`:**
1. Läuft auf GitHub-Worker (nicht lokal!).
2. Liest `phi/live_url_map.json` → für jede CDN-Quelle: Original-API-URL.
3. Original-Dateiname = letztes Pfadsegment der API-URL (z.B. `46083.txt` → `46083`).
4. `gh release upload` mit neuem Namen `{delivered_filename}_{iso_utc}.json`.
5. `gh release delete-asset` für das alte Prefix-Asset.
6. Schreibt `sources.φ` URLs auf `{delivered_filename}.json` um.

**Aktion — `scripts/migrate_live_to_cdn.py` (Zeile 185):**
```python
# ALT:
asset_name = f"{name}.json"
# NEU:
asset_name = f"{delivered_filename}_{iso_utc}.json"
```
`delivered_filename` aus `live_url_map.json` → letztes Pfadsegment.

**Verifikation:**
- Workflow manuell triggern (`gh workflow run`).
- `gh release view ndbc.noaa.gov --repo omegaflow/sources` → Assets haben neue Namen.
- `cargo run` → lädt korrekt (sobald `{latest}`-Resolver existiert).

---

## 4. `{latest}`-Resolver im Archivar implementieren

**Problem:** Ohne Resolver zeigt sources.φ auf `{name}_{latest}.json` → 404. Oder ohne `{latest}` zeigt es auf `{name}.json` → nur das neueste Asset (CI überschreibt oder legt neues an).

**Aktion (Rust, `src/main.rs`):**
Wenn beide Schritte erledigt sind (Asset-Rename + `{latest}`):
- Der Archivar ersetzt `{latest}` im URL durch den aktuellsten Timestamp aus dem CDN-Release.
- GitHub API: `GET /repos/omegaflow/sources/releases/tags/{netloc}` → Assets nach Name filtern → neuestes wählen.
- Oder: ohne `{latest}` — URL zeigt auf `{delivered_filename}.json` → CI legt neues Asset an → Archivar fetched das jeweils aktuellste.

**Empfehlung:** Ohne `{latest}` weitermachen bis Asset-Rename abgeschlossen ist. `{latest}` erst implementieren wenn alle Assets Zeitstempel-Namen haben.

---

## 5. Systematischen Daten-Audit in CI durchführen

**Problem:** Nur ~100 von 2066 Quellen wurden gegen echte CDN-Daten verifiziert. Die restlichen Parameter (lat_key, lon_key, ra_key, dec_key) basieren auf Key-Klassifikation, nicht auf Daten.

**Aktion:**
- CI-Workflow `audit-source-keys.yml` existiert bereits. `scripts/api_audit.py` läuft auf GitHub-Worker.
- Der Audit fetched Range-Requests (200KB) von CDN-Assets und vergleicht deklarierte Keys mit realen Daten-Keys.
- Output: `phi/api_audit.jsonl` → Download als Workflow-Artifact.

**Aktion nach Audit:**
- `key-missing` Quellen: `lat_key`/`lon_key`/`ra_key`/`dec_key` an tatsächliche CDN-Daten-Keys anpassen.
- `not-json` Quellen: CDN-Pipeline prüfen (flatten_cdn.py Bug).
- `fetch-fail` Quellen: CDN-Asset existiert nicht → Pipeline-Defekt.

---

## 6. Nicht quellentreu (optional, niedrigere Prio)

**135 Live-TAP-Katalogquellen mit ttl=86400:** Sind statische Kataloge. Sollten CDN-gemirrort werden, nicht als Live-APIs laufen.

**`em gravity`-Fix (69 Gaia-Quellen):** Wurde zu `em` geändert per Rat-Entscheidung. Nicht gegen Daten verifiziert. Die Gaia-Daten sind Lichtbeobachtungen → `em` ist korrekt. Gravity war falsch.

**395 map-Frame-Fixes:** Basieren auf Key-Klassifikation. Nicht gegen Daten verifiziert. CDN-Audit (Schritt 5) wird zeigen ob die Keys korrekt sind.

**Station-Koordinaten:** 2 NDBC-Bojen um ~260 km verschoben (41001, 42055). Rest nicht geprüft.

---

## Dateien im Repo (Stand letzter Commit `b877be1`)

| Datei | Beschreibung |
|---|---|
| `phi/sources.φ` | Quell-Datei (2066 Blöcke, url-first) |
| `phi/sources_restored.φ` | Version mit Live-APIs statt CDN-URLs (Referenz) |
| `phi/live_url_map.json` | 1:1 Map: Quell-Name → Original-API-URL |
| `phi/sources.φ.spec` | Spezifikation des Dateiformats |
| `phi/recovery/*.φ` | Historische sources.φ-Versionen (für Recovery) |
| `scripts/restore_all_live.py` | Original-API-Recovery aus Git-Historie |
| `scripts/migrate_live_to_cdn.py` | CDN-Pipeline (Asset-Erzeugung + Upload) |
| `scripts/tap_to_cdn.py` | TAP-Katalog-Konvertierung |
| `scripts/api_audit.py` | CDN-Daten-Audit (läuft in CI) |
| `.github/workflows/audit-source-keys.yml` | CI-Workflow für Daten-Audit |
| `.github/workflows/generate-ephemerides.yml` | Ephemeriden-Generierung (täglich) |
| `.github/workflows/refresh-protected-data.yml` | CDN-Refresh (alle 5 Min) |
