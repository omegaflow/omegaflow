# Stimmen — Manifest-Loader-Architektur (Rat + Roster, 2026-10-11)

Frage und Vorbereitung: `state/mycelium/rat-vorbereitung-2026-10-11-manifest-loader.md`
(gegründet auf `archive_search --all`, 982 + 1427 Zeilen, voll gelesen).

## Rat (fünf Stimmen) — Einstimmig

**Option 1 — Spatial-Index-Manifest mit pro-Shard GEMESSENER bbox.** Kein Key-Fallback.
- **Mountain:** die bbox ist eine gemessene Eigenschaft des Shards (LAS-Header); `country_of`
  ist ein Key-Faktum, keine Räumlichkeit.
- **River:** der Manifest-Pre-Filter ist eine neue Schranke *vor* der Diode und darf sie nicht
  umordnen; Shard-Test = nächster bbox-Punkt → `record_in_enclosure` (`fetch.rs:522`), die
  **eine** `enclosure_rho` (`membrane.rs:430`) — keine vierte Dilation.
- **Mycelium:** die Header-range-GETs sind die Manifestations-Kosten (CI); unplatzierbare Shards
  `pending`, nie ein Key-Fallback.
- **Sensory/Future:** bbox als gemessene Wahrnehmung; das durabler CDN-Asset ist das Erbe.
- **Form:** `.manifest`-Zeilen `shard <url> <xmin..zmax>` + `node <prefix> <union-bbox>`;
  `phi/sources.φ` trägt `format coverage_manifest`; **Dispatch über `format`, nie über
  `Frame::Manifest`** (der bleibt der frameless-Parse-Default, `parse.rs:140`).
- **Riss (stärkster):** ICRS-bbox existiert nur, wo der Shard eine CRS-VLR trägt; sonst
  `unplaceable` → `pending`. Die Coverage ist nur so vollständig wie die CRS-Metadaten.
- **Nächster begrenzter Bauschritt:** `parse_coverage_manifest(text) -> Vec<ShardDescriptor>`
  + `shard_intersects_hull(bbox, presences, field, env)` (pure std, Fixture-Test, `cargo check`).

## UI-Roster (`mycelium-ui`) — Bestätigung

- **DeepSeek Chat — bestätigt + geschärft:** „Verdikt bestätigt, aber die pending-Menge braucht
  einen eigenen, konservativen Pfad — sonst reißt genau dort die Korrektheit." Konkret:
  Manifest-Schema führt `bbox:null, crs:null, status:pending`; der Loader behandelt
  pending-Shards konservativ als „kann schneiden" (sonst Falsch-Negativ); die `node`-Union darf
  pending-Kinder **nicht** einklammern, sonst lügt die Union.
- **Qwen3.7-Plus — bestätigt:** Extraktion der bbox aus dem LAS-Header statt aus Keys ist der
  einzig valide Weg (kein Grid-Aliasing); Trennung Dispatch(`format coverage_manifest`) vs. Frame
  wahrt die Abstraktion; der Riss braucht einen Fallback (separater curl-Request zur
  CRS-Auflösung), bevor der Shard den Hull schneiden darf.
- **Gemini 3.1 Pro — `pending` (unmeasured):** der AI-Studio-Composer wurde in dieser Runde nicht
  erreicht (Landing-Page statt Prompt-Feld; Run-Button nicht gefunden) — gemessener Nicht-Antwort-
  Grund, keine stille Null.

**Verdikt (Rat durch UI bestätigt):** Option 1, Dispatch über `format coverage_manifest`,
Shard-Test über die eine `enclosure_rho`, `pending`-Menge mit konservativem Pfad.
