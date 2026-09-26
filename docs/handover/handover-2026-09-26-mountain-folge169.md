<!--
  title: Handover — Mountain-Folge 169 (2026-09-26)
  session: Mountain-Folge 169
  class: handover
  date: 2026-09-26
  sha256: bd0bb1c4871919e3e129a71d0d2898e6a693f04d74be28eb5dab8174484cffa0
  status: live
-->
# Handover — Mountain-Folge 169 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann in keiner Rangfolge. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### gll_rss_rsr — Workflow + Asset manifestiert, sha offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Volldownload des Assets (`archive_search --sniff …/gll_rss_rsr.bin`).
- **Lage:** (gemessen 2026-09-26) `.github/workflows/gll-rss-rsr-cdn.yml` gebaut
  (Muster `galileo-atdf-cdn.yml`), committet `0eacaec2b`, gepusht, dispatcht
  `36240229845` → **success** (head `d88c8fd5f`). Asset `gll_rss_rsr.bin` auf der
  CDN: `--sniff` HTTP **200**, **85 956 014 B**, kein Shard — **sha256 nur
  partial** (`618b5b6c…`, Download unvollständig) → **ungemessen**. YAML nicht
  maschinell validiert (kein `actionlint`/`js-yaml`) — strukturell Zeile für
  Zeile `galileo-atdf-cdn.yml`; die Annahmen (`timeout: 180`, Shard-Guard,
  Release-Tag vorausgesetzt) bleiben benannt.
- **Blockade:** keine.
- **Braucht:** vollständigen `gll_rss_rsr.bin`-Download + sha256, dann in den
  `gll_rss_rsr`-Block `phi/sources.φ` nachtragen.

#### TAP-Quellen — 4 Query-Fixes gebaut, CI-Verify offen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via grind-flash) `phi/sources.φ` 4 Blöcke
  korrigiert (`gavo.aip.de:7458` RAdeg/DEdeg/HRV/Teff_K, `padc-tap-rcsed:7468`
  JOIN ra/dec/z + corrfibmag_u/g/r, `voparis:7480` ra/dec/mag/vr, `skvo:9528`
  JOIN ogle.objects_all + obs_time); `phi/blocked_sources.φ` 4 `blocked
  parser-def json` → `descoped` (HTTP 200: 254541 / 454247 / 201877 / 442297 B).
  **Der Register-Anteil ist committet** — Mycelium folge172 griff die
  Arbeitsbaum-Änderung auf (`f889b29c3`, `sources.φ` + `blocked_sources.φ`) und
  schrieb sie unter eigenem Commit; die Planck-Zeile dieses Atoms (+5) ist noch
  offen. **Unabhängig nachgemessen (2026-09-26):** alle vier HTTP **200**; die
  Byte-Zahlen **streuen live** (`SELECT TOP 5000` ohne `ORDER BY`) — 252486 /
  458965 / 201877 / 416508 B, nur TAP3 deckt sich mit dem Taucher-Wert. Stabile
  Größe ist allein der 200-Status (die notes in `phi/blocked_sources.φ` tragen
  eine zeitgestempelte Momentmessung, nicht reproduzierbar).
- **Blockade:** keine.
- **Braucht:** `gh workflow run ci-check.yml` (Register-Parse + Format; prüft
  auch die neue `epoch obs_time mjd`-Direktive im ogle-Block). **Test-Grünheit
  entscheidet der CI-Lauf** — lokale Testausführung ist strukturell verweigert;
  lokal gemessen ist nur, dass der Test-Code kompiliert (`cargo check --tests`
  grün).

#### Parquet-Codec — Arme gebaut, CI-Tests offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ci-check am HEAD `0eacaec2b`.
- **Lage:** (gemessen 2026-09-26 via grind-max) `src/archivar/parquet.rs`:
  gzip (codec 2 via `inflate::gunzip`), LZ4 (codec 5, Hadoop-Framing), **LZ4_RAW
  (codec 7** — `PageHeaderInfo.uncompressed_page_size` neu gelesen),
  DELTA_BINARY_PACKED (5), DELTA_LENGTH_BYTE_ARRAY (6), DELTA_BYTE_ARRAY (7),
  BYTE_STREAM_SPLIT (9); 18 neue `#[cfg(test)]`-Fixtures (35 `#[test]` in der
  Datei inkl. vor-bestehender, gemessen via `sgrep -c`); `cargo check -p
  omegaflow` und `--tests` **0/0**. **Pending (benannt):** delta für
  INT96/FIXED_LEN_BYTE_ARRAY; BSS für Nicht-FLOAT/DOUBLE.
- **Blockade:** keine.
- **Braucht:** committet `0eacaec2b` + gepusht; **Test-Grünheit = CI** am Push
  (lokale Ausführung strukturell verweigert); lokal gemessen ist nur die
  Kompilation.

#### GRIB-2-Codec — 5.0/5.2/5.3/CCITT-G4 gebaut, CI-Tests offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ci-check am HEAD `0eacaec2b`.
- **Lage:** (gemessen 2026-09-26) `src/archivar/grib2.rs`: Template 5.0
  (`decode_simple_packing`, `MsbBitReader`) + complex 5.2 (`decode_complex_packing`,
  Test `complex_packing_two_groups`) + complex+spatial 5.3
  (`decode_complex_spatial_packing`, `read_sign_magnitude`, Test
  `complex_spatial_packing_order_one`) + CCITT-G4 (`CCITT_WHITE`/`CCITT_BLACK`
  aus libtiff `t4.h`, Transkription zeichenweise verifiziert — 104+104 Einträge,
  `diff` gegen die Quelle leer; Source-Cross-check RFC 804, 0 Abweichungen;
  `decode_ccitt_g4` 1D,
  Test `ccitt_g4_one_line`, 2D → `None`); 16 `#[test]` in `grib2.rs` (4 neu,
  gemessen via `sgrep -c`); `cargo check -p omegaflow` und `--tests` **0/0**.
  Benchmark (gemessen): **der enge Zuschnitt entschied, nicht das Profil** — 5.2
  landete auf `grind-max` (eng), 5.3/CCITT-G4 auf `grind-flash` (eng, Tabelle
  vorab aus der Quelle); der breite `grind-max`-Lauf blieb zweimal leer. Ein
  Profil-Anteil (flash vs. max) ist damit **nicht isoliert** — die frühere
  „flash-first"-Zuschreibung war konfundiert.
- **Blockade:** keine. (2D-Fax-Modus bleibt benannt offen — `None`, kein Silent.)
- **Braucht:** committet `0eacaec2b` + gepusht; **Test-Grünheit = CI** am Push
  (lokal verweigert); lokal gemessen ist nur die Kompilation.

#### gll.rss + Klasse-5 — Register-Nachtrag
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Lauf-Ende `gll-rss-atdf-cdn` / `tools-build`.
- **Lage:** (gemessen 2026-09-26) `gll-rss-atdf-cdn 36233487657` + `tools-build
  36233489263` (head `cc991b751`) in_progress; `messenger_tnf.bin`-sha pending
  (Teildownload).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36233487657` / `ci_manage view 36233489263`; dann
  `messenger_tnf.bin`-sha + Shards `gll-rss-odr`/`atdf` in `phi/sources.φ`.

#### Atom D — Beat-Paar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** externe two-station-Aufnahme — Anfrage an die Träger-Linie offen (`docs/handover/handover-2026-09-26-sensory-folge176.md`).
- **Lage:** (gemessen 2026-09-26) Atom D gebaut (`odf.rs::tnf_phase_series`,
  WGSL `beat_pair` hinter presence-/ν-Gates); das Paar selbst absent.
- **Blockade:** keine Messdaten für ein kohärentes Paar.
- **Braucht:** dual-comb-/two-station-Kandidat bei der Träger-Linie anfragen
  (Aufenthalt = Eigentum: per Direkt-Edit in die Übergabe des Owners).

### Operator handelt

#### `epochrange` Wire-Slot (MJD-Breite) — Akt
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26 via grind-pro, Vorbereitung fertig) Entwurf:
  Slot 20 (`r_eq`) als Kopf des Mess-Blocks; Einheit TDB-Sekunden (MJD-Tage ×
  86400, Delta ohne J2000-Offset); `0.0` = Punktmessung (null-echt, Spiegel
  `bin_width`); im WGSL **ungelesen** → null GPU-Wirkung. Änderungsfläche:
  `types.rs:41-62` + `:183-199`, `parse.rs:1053-1058`, `extract.rs:4406`,
  `spatial.rs:495` + `:613`, `constants.js:112/143`; Record bleibt 26×f64.
- **Blockade:** Architektur-Akt (Wire-Version) → Operator-/Rat-Wort.
- **Braucht:** Operator-Wort; danach Implementierung nach dem Entwurf.

#### Chrome-DevTools-MCP — Akt
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26) MCP antwortet `-32001 timeout`; die Vorbereitung
  steht bereits in `docs/concepts/tools-map.md:294-297` (Pfad i, noch nicht
  angebunden; Flags `--no-usage-statistics --no-performance-crux` als Bedingung).
- **Blockade:** Operator-Wort (Debugger-Rechte am live Chrome/Neustart).
- **Braucht:** Operator-Wort; danach Chrome mit den Flags starten und MCP
  anbinden.

#### S3-Scheme (Token) — Akt
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (bzw. Token).
- **Lage:** (gemessen 2026-09-26) SigV4+Handshake gebaut; `blocked key`
  (alter 401 = Register-Umbuchung).
- **Blockade:** Operator-Wort (Token).
- **Braucht:** Token bzw. Wort; danach die Route freischalten.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
