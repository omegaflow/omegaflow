<!--
  title: Handover — Mountain-Folge 172 (Stand 2026-09-26)
  session: Mountain-Folge 172
  class: handover
  date: 2026-09-26
  sha256: 84949360e0f4efff38d7c7aa72759907a06570610a582b7823e081e9f5236a96
  status: live
-->
# Handover — Mountain-Folge 172 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Jeder Punkt aufgeschlüsselt: Trigger / Lage / Blockade /
Braucht. Dieser Atom hat gearbeitet, nicht gemessen: fünf CI-Diagnosen gefixt
(EHT/NOHRSC/gll/CI-Tests/RX100), `--brave` aus den Agenten-Prompts entfernt,
`mail_digest`-cwd-Riss und `gion_bin_roundtrips` gefixt.

## Stehender Pass

- **Postfach:** (gemessen 2026-09-26 via `sread`) `state/mail/mail_ledger.φ`
  existiert (301 Zeilen); `mail_digest` meldete „ledger absent" (cwd-relativer
  Fallback) — in diesem Atom gefixt (`tools/service/src/bin/mail_digest.rs`).
  Kein neuer Mountain-Eingang.
- **CI-Status am HEAD:** (gemessen 2026-09-26 via `ci_manage`) HEAD
  `06ccbe25c`; `ci-check` `36269192190` **pending**; jüngste Mountain-Rotläufe
  an Vorfahren, Fixes in diesem Atom.
- **Artefakt-Frische (`DUE`):** nach dem Push neu messen; `gh workflow run` für
  die betroffenen `*-cdn` + `tools-build` erst nach Commit+Push.

## Offen (aufgeschlüsselt)

### P2 EHT uvfits — Modul-URL gefixt, erste Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Commit+Push, dann `eht-uvfits-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-26 via `ci_manage log 36265202491` + `curl -sSIL`)
  der Lauf scheiterte vor jedem Compiler-Schritt an HTTP 404 der Modul-URL; das
  ALMA-Listing führt den vollen Namen
  `group.uid___A001_X11b3_X30.ec_eht.e17a10-7-hi-na-1921-293-fits.tgz`
  (**HTTP 200, 2 304 234 106 B**); Fix in
  `.github/workflows/eht-uvfits-cdn.yml` gesetzt (nur die Download-URL).
- **Blockade:** keine (Push).
- **Braucht:** nach `/commit`+Push `gh workflow run eht-uvfits-cdn.yml`; danach
  Asset-sha256 und die `Aa`–`Ap`-df aus dem Vollauf in `phi/sources.φ` nachtragen.

### P3 NOHRSC-Asset — Datensatznamen gefixt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Commit+Push, dann `nohrsc_snowfall-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-26 via `ci_manage log 36262592887`) der Compiler
  fragte `latitude`/`longitude`/`data` ab — die NOHRSC-netCDF-v1.2 trägt
  `lat`/`lon`/`Data` (lat/lon f64, Data f32); Fix in
  `tools/harvest/src/bin/nohrsc_snowfall_compiler.rs` (Per-Achsen-Elementgröße
  aus HDF5-Metadaten); lokaler Lauf 506 Cells, Roundtrip parst.
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run nohrsc_snowfall-cdn.yml`; bei success
  `phi/harvest.φ:145` schließen.

### P4 gll.rss ODR — Compiler gestreamt, Shard-Manifest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Commit+Push (auto-dispatch bei Änderung unter `tools/harvest/`), dann Lauf.
- **Lage:** (gemessen 2026-09-26 via `ci_manage log 36260734404`) der Lauf hielt
  12,59 GiB im RAM (Runner 16 GB; `shard_groups` klonte auf ~25 GiB) → Runner-
  Shutdown; zusätzlich erkannte `cdn_reconcile::shard_base` die Namen
  `gll_rss_odr_s<ord>.bin` nicht. Fix in
  `tools/harvest/src/bin/gll_rss_odr_compiler.rs` + `.github/workflows/gll-rss-odr-cdn.yml`:
  streamen (~1 GiB), Shards `gll_rss_odr.bin.NNN` + `gll_rss_odr.manifest`.
- **Blockade:** keine.
- **Braucht:** nach Push den Lauf lesen; Manifest/sha in `phi/sources.φ` (Block
  um `:8438`) nachtragen — kein Einzel-Asset bei 12,59 GiB (2-GiB-CDN-Cap).

### P5 CI-Verify + GIO2-Neumanifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am neuen HEAD.
- **Lage:** (gemessen 2026-09-26 via `ci_manage log 36263271666`) Mountain-Tests
  rot: grib2 `complex_packing_two_groups` (Gruppenparameter spaltenweise), ionocal
  `full_year` (Pivot 69) + `gion_bin_roundtrips` (Name 34 B > 32 B), parquet
  lz4/gzip (Hadoop-Frame). Fixes gesetzt: `src/archivar/grib2.rs`,
  `ionocal.rs` (Pivot, GIO2-Namenfeld 64 B + Legacy-`GION`), `parquet.rs`;
  EHT-Clippy in `src/archivar/uvfits.rs`. Fremde Lints bleiben: mycelium
  (`extract.rs`, `uws.rs`, `tests.rs`, `hdf4.rs`), sensory (`te.rs:4306`,
  `actuators.rs`, `tests.rs`), river (`main_flow.rs`, `spatial.rs`).
- **Blockade:** keine (Push).
- **Braucht:** nach Push `ci_manage view`/`log` am neuen HEAD; `galileo_ionocal.bin`
  im GIO2-Format neu manifestieren (Alt-Asset am CDN `omegaflow/sources`
  überschreiben/entfernen → `gh workflow run galileo-ionocal-cdn`; CDN-Schreibakt
  = Register-/Consent-Duty).

### P6 RX100-Luminanz — lokale Gerätequelle, CI-absent-tolerant
- **Status:** wartend | **Bindung:** eigen (Substanz river)
- **Trigger:** K-Messung gegen kalibriertes Luminanzmeter und Kamera im Smart-Remote-LAN (`handover-2026-09-26-river-folge38.md:109`).
- **Lage:** (gemessen 2026-09-26 via `ci_manage log 36260851738`) der rote
  `harvest`-Lauf war der `rx100_luminance`-Arm: SSDP void, keine Kamera im CI
  (`phi/harvest.φ:233` `asset fehlt`, `:236` arm); Note präzisiert
  (`phi/harvest.φ:240`); `tools/harvest/src/bin/rx100_compiler.rs` gibt bei
  fehlender Kamera jetzt `absent` (exit 0) statt Hard-Abort — der CI-Rot entfällt,
  `asset fehlt` bleibt wahr (kein CDN-Asset).
- **Blockade:** physische Kamera + K-Kalibrierung (River).
- **Braucht:** River (`handover-2026-09-26-river-folge38.md:109-114`: K=12.5
  ungemessen, `freq`/`bin_width` nicht verdrahtet).

### P7 DevTools-MCP — 60-s-Wall gemessen, kein Config-Hebel
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** keiner (gemessen).
- **Lage:** (gemessen 2026-09-26 nach Operator-Neustart via
  `chrome-devtools_evaluate_script`) ein 90-s-Busy-Wait liefert weiterhin
  `-32001 Request timed out`; `mcp.chrome-devtools.timeout=180000` ist gültig
  (Schema-Default 5000 ms), greift aber nicht — der Wall ist server-seitig
  (chrome-devtools-mcp 1.9.0 SDK, `index.js:23985 DEFAULT_REQUEST_TIMEOUT_MSEC
  = 60000`); kein CLI-Flag in 1.9.0. Befund: einzelne MCP-Evals unter 60 s
  halten, längere über `playwright_*`.
- **Blockade:** keine.
- **Braucht:** kein Bau/Config.

### UI-Chat-Stimmen zum `epochrange`-Befund (vier von fünf)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Anmeldung in den vier Tabs).
- **Lage:** (gemessen 2026-09-26 via `chrome-devtools`) `claude.ai`,
  `chat.deepseek.com`, `kimi.ai`, `arena.ai` stehen an Login-/Consent-Wänden;
  `chat.z.ai` (GLM-5.3-Flash) lief und trägt `descoped`
  (`state/stimmen/2026-09-26_zai_ui_epochrange-p6.json`).
- **Blockade:** Login/Consent (nicht umgangen).
- **Wort:** externe Stimmen über den Voice-Runner fragen (nicht chatgpt.com) | 2026-09-26 | Operator (Session).
- **Braucht:** die vier Sessions im `chrome-devtools`-Browser anmelden, dann
  dieselbe prompt-Datei senden.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
