# SESSION HANDOFF 2026-08-08 (Fresh Start)

## Ausgangslage

Commit `aceb98e`: **1924 Quellen**, alle `force`+`frame`, `cargo check` sauber.

Die nachfolgende Session (88 Commits, `f248d38` bis `babf936`) hat eine
Migration auf 4-Token-Syntax versucht. Die Migration ist fehlgeschlagen:
- Feldnamen wurden **erfunden** statt aus API-Antworten abgefragt
  (`seismic_magnitude_mw`, `seismic_depth_m` → existieren weder in USGS noch
  in der Matrix)
- Force-Unit-Zuordnungen waren physikalisch falsch (`Kmag em km` statt
  `Kmag em mag`, `Field_Magnitude em mag` statt `em nT`)
- Frame-Logik war inkonsistent (11 Template-Blöcke hatten hartcodierte
  Zürich-Koordinaten, Mars-Wetter verschwand, ISS `at sun` statt `on earth`)
- Kein einziger Audit-Agent wurde gegen die echten API-Antworten ausgeführt

## Was erhalten bleibt (nützliche Artefakte)

- `phi/recovery/unit_cache.json` — 2164 API-Antworten (552KB), als Ground
  Truth für Feldnamen und Messwerte
- `docs/concepts/PARSER_EVALUATION_MATRIX.md` — bindende SI-Matrix (vom Rat)
- `phi/sources.φ` auf `aceb98e` — der saubere Ausgangszustand

## Was verworfen wird

- Alle 88 Commits ab `f248d38` (die Migration)
- `phi/sources_new_format.φ` — die fehlerhafte Migrations-Ausgabe
- `scripts/migrate_format.py` — das Migrations-Skript (flickt falsch)
- `scripts/fetch_units.py` — API-Cache-Builder (war langsam, viele Fehler)

## Auftrag für die frische Session

**Ziel:** `phi/sources.φ` in das kanonische Format überführen.

**Grundsätze:**
- JEDE Zeile quadrat gegen die ECHTE API-Antwort abfragen (unit_cache.json
  ist Ground Truth). Kein Feldname, der nicht in der API existiert.
- Die Matrix aus `PARSER_EVALUATION_MATRIX.md` ist bindend. `C` nicht `degC`.
  Druck (hPa) ist `advective`, nicht `acoustic`. Wellenhöhe (m) ist
  `acoustic`. Windrichtung (deg) ist `seismic-surface`.
- Sitzung ist das Atom. Die Migration muss in EINER Sitzung fertig werden,
  gegen alle APIs validiert sein.
- `cargo check` zero errors, zero warnings.
- Kein `???`. Unbekannt = weglassen, nicht erraten.

**Reihenfolge:**
1. Ausgangs-Datei `phi/sources.φ` (HEAD `aceb98e`) lesen und verstehen.
2. Die Matrix in `PARSER_EVALUATION_MATRIX.md` verstehen.
3. Für JEDE Quelle: API-Antwort aus dem Cache abfragen → Feldnamen und
   Einheiten aus der echten Antwort bestimmen.
4. Syntax: Skalar-Extraktion (`field`, `last`, `path`) = exakt 4 Tokens
   `<keyword> <key> <force> <unit>`. Position (`lat`, `lon`, `alt`,
   `ra`, `dec`, `plx`) = exakt 3 Tokens `<keyword> <key> <unit>`.
5. Frame: `on <body> <lat> <lon>` (Oberfläche) oder `at <body>` (baryzentral).
   Keine `1.0`-Skala. Kein `_key`-Suffix.
6. Keine Erfindungen. Kein `geojson events` mit ausgedachten Namen.
   Keine `domain_physical_lines`-Tabellen mit geratenen Feldern.

**Cache-Abfrage:** Der Cache liegt unter `phi/recovery/unit_cache.json`.
Die Migration MUSS gegen diesen Cache validieren. Felder ohne Eintrag im
Cache → API neu abfragen oder weglassen.

**Validierung:** Vor dem Commit: `cargo check`, Token-Zählung, Matrix-
Konformität, und eine Stichprobe gegen live APIs bestätigen.<｜end▁of▁thinking｜>

<｜｜DSML｜｜tool_calls>
<｜｜DSML｜｜invoke name="write">
<｜｜DSML｜｜parameter name="content" string="true"># Session Handoff — Fresh Start

## State

Clean base: `aceb98e` — **1924 sources**, all `force`+`frame`, `cargo check` clean.

The subsequent session (88 commits, `f248d38`–`babf936`) attempted a 4-token
migration. It failed systematically:
- **Fabricated field names** — `seismic_magnitude_mw`, `seismic_depth_m` etc.
  do not exist in any API response. Invented in `domain_physical_lines`.
- **Physically wrong force–unit assignments** — `Kmag em km` (should be
  `em mag`), `Field_Magnitude em mag` (should be `em nT`), `pressure
  acoustic hPa` (should be `advective hPa` per council).
- **Frame errors** — 11 template blocks had hardcoded Zürich coordinates,
  Mars weather disappeared, ISS `at sun` (should be `on earth`).
- **No API audit** — not a single block was validated against the real
  API response.

## Useful artifacts

| Artifact | What |
|---|---|
| `aceb98e:phi/sources.φ` | Clean base, 1924 blocks, all force+frame |
| `phi/recovery/unit_cache.json` | 2164 real API responses, 552 KB — **ground truth** for field names, values, units |
| `docs/concepts/PARSER_EVALUATION_MATRIX.md` | Binding SI matrix (council verdict) — use `C` not `degC`, `km/h` not `kmh`, pressure → `advective`, wave height → `acoustic`, wind direction → `seismic-surface` |

## Discard

- All 88 migration commits (`f248d38`–`babf936`)
- `phi/sources_new_format.φ` — broken migration output
- `scripts/migrate_format.py` — broken migration script
- `scripts/fetch_units.py` — slow, error-prone cache builder

## Task

Migrate `phi/sources.φ` to canonical format. **One session, complete, validated.**

### Principles

- **Every line validated against the real API response** (`unit_cache.json` is
  ground truth). No field name that does not exist in the API.
- **The matrix is binding.** `C` not `degC`. Pressure (hPa) → `advective`.
  Wave height (m) → `acoustic`. Wind direction (deg) → `seismic-surface`.
- **Session is the atom.** All planning and implementation in one context window.
- `cargo check` zero errors, zero warnings.
- **No fabrication. No `???`.** Unknown = omit.

### Canonical syntax

Scalar extraction (`field`, `last`, `path`, `first`): exactly 4 tokens:
```
keyword key force unit
```

Position (`lat`, `lon`, `alt`, `ra`, `dec`, `plx`, `pmra`, `pmdec`,
`tau`, `z`, `dist`, `epoch`, `radvel`): exactly 3 tokens:
```
keyword key unit
```

Frame: `on <body> <lat> <lon>` (surface) or `at <body>` (barycentric).
No `1.0` scale. No `_key` suffix.

### Steps

1. Read and understand the base file (`aceb98e:phi/sources.φ`).
2. Read and understand `PARSER_EVALUATION_MATRIX.md`.
3. For **every** source: query `unit_cache.json` → determine real field names,
   units, and values from the actual API response.
4. Build the migration: scalar extraction = 4 tokens, position = 3 tokens.
5. Validate: `cargo check`, token-count audit, matrix compliance check.
6. **Spot-check against live APIs** before committing.

### Cache

`phi/recovery/unit_cache.json` — 2164 URL entries. Each entry maps field
names to their native units and sample values (from VOTable metadata, JSON
responses, NDBC header lines). The migration MUST use this as ground truth.
Fields whose unit is not in the cache → re-query the API or omit.
