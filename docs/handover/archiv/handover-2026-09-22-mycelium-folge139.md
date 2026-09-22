<!--
  title: Handover — Mycelium-Folge 139 (Ausführungs-Pass: RAWACF-Compiler + Workflow, Katalog-Wald, pre-cdn-Join, Lizenz-Auflösung, NOAA-L1B) (Stand 2026-09-22)
  session: Mycelium-Folge 139
  class: handover
  date: 2026-09-22
  sha256: 91d06908efa6523a8841c3bf7aefd94e3462a5ca27944e2802c89c833d931de1
  status: live
-->
# Handover — Mycelium-Folge 139 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt **aufgeschlüsselt**: **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session ist der **Ausführungs-Pass** der Mycelium-Linie. Sie hat die
Vorgängerin `handover-2026-09-22-mycelium-folge138.md` konsumiert und die zehn
parallel abarbeitbaren Punkte dispatcht (grind-flash: RAWACF, Katalog-Wald,
pre-cdn-Join; grind-pro: solar-key, Lizenz-pending, NOAA-L1B; build: Babamul-CDN,
Free-Model-Bench, Gaia-Alerts, CI-format).

## Stehender Pass (gemessen 2026-09-22)

- **HEAD** beim Start `d16f2db0`; während der Session auf `3f7ff390` vorgerückt
  (mountain folge134, sensory folge147). Der Arbeitsbaum trägt weiter nur die
  eigenen Hunks.
- **Postfach** `post.md`: **leer** — mountain foldete die mountain-Zeile
  (`5518e3de0`), sensory die vC-Zeile (`3f7ff390`). Keine mycelium-Zeile.
- **CI** (`/tmp/opencode/ci_status.md`, 16:49): `ci-check 35736814999`,
  `hyperscanning-te 35736695583`, `te-gate 35734557660`, `health-check 35718663622`
  in_progress; ältere `ci-check` rot. Kein mycelium-Job rot.
- **`register_lookup --dropped`**: seit sensory folge147 wieder nutzbar (Baseline
  2665 → 2517) — der Folge138-Punkt `--dropped`-Drift ist damit erledigt.
- **`open_points_check` folge138**: 2 absent — historische `folge137.md`-Nennungen
  im erledigten dirty-tree-Punkt.
- **`cargo check --workspace`**: 0 Fehler, 0 Warnungen.

## Offen (aufgeschlüsselt)

### RAWACF CDN-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Compiler `tools/harvest/src/bin/superdarn_rawacf_compiler.rs` +
  Workflow `.github/workflows/superdarn-rawacf-cdn.yml` gebaut; Quelle `sources.φ:8032`
  (FRDR-DMap, `superdarn_rawacf_lag0_power_db`); CDN `superdarn_rawacf.bin` 404.
  FRDR-Sample 2026-09-22: 200, 604460 B, sha256 `8cb129b5…`.
- **Blockade:** Lauf-Ergebnis offen (Compiler in CI noch nicht end-to-end gelaufen).
- **Braucht:** nach Push `gh workflow run superdarn-rawacf-cdn.yml`, dann sha256 in
  `sources.φ:8032` registrieren.

### solar-system-open-data Compiler
- **Status:** pending | **Bindung:** eigen
- **Lage:** Key gültig — 200 mit `Bearer`, 403 mit rohem GUID (`blocked_sources.φ:47`
  von `blocked key` auf `pending`). Endpunkt ist ein statischer Körper-Katalog
  (gravity/Masse/Radius), keine url-Zeile — er mappt auf `BodyProperties`.
- **Blockade:** keine.
- **Braucht:** Compiler/Format für den Körper-Katalog bauen (BodyProperties-Referenz,
  Bearer über quoted header `"Bearer {SOLAR_SYSTEM_OPEN_DATA_KEY}"`).

### Katalog-Wald (Rest)
- **Status:** pending | **Bindung:** eigen
- **Lage:** 31 `index.φ`-Zeilen abgearbeitet — 21 released, 10 mit Schritt. Offen:
  `grind_arcgis` 17 + `grind_vires` 1 Block in `sources.φ` mergen; 5 Kandidat-Inventare
  proben (`stage/*_candidates.φ`); `queue/master.φ` absent.
- **Blockade:** Merge ist Review, nicht mechanisch.
- **Braucht:** Merge der `stage/grind_*_converted.φ`-Blöcke; Probe der Kandidaten-URLs.

### Katalog-Lizenz — dataone + gaia_swpc-Trennung
- **Status:** pending | **Bindung:** eigen
- **Lage:** 6 der 7 pending aufgelöst (bcodmo + erddap_bcodmo CC BY 4.0;
  bodc/esa_eogateway/gfz_igets decline; gaia_swpc gemischt ESA+PD). Offen:
  `dataone` `/terms` 401 Apache Basic Auth; `gaia_swpc` ESA/PD trennen.
- **Blockade:** dataone-Doku nicht messbar; gaia_swpc mechanische Datei-Teilung.
- **Braucht:** DataONE-Doku/GitHub; `gaia_swpc_vokabular.φ` in zwei Dateien teilen.

### pre-cdn Join (Rest)
- **Status:** pending | **Bindung:** eigen
- **Lage:** Join gemessen — richest 824 / params 45 URLs ⊂ Lost-Blocks (5701);
  729 neu registrierbar; 15 `source`-Risse (HAPI-Drift). Outputs in
  `phi/pipeline/stage/` (gitignored).
- **Blockade:** stale `target/release/omegaflow` (Port-Output `pending`
  Re-Verifikation nach Build).
- **Braucht:** 4877 Lost-Blocks ohne Pool-Eintrag extrahieren; 729 Kandidaten
  registrieren; Port-Output nach CI-Build re-verifizieren.

### Babamul CDN
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Compiler gebaut; `gh workflow run babamul-cdn.yml` dispatcht
  (`35745291297`).
- **Blockade:** Lauf-Ergebnis offen.
- **Braucht:** sha256 nach Lauf in `sources.φ:14016` registrieren.

### Free-Model-Bench
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `gh workflow run free-model-bench.yml -f model=gemini-2.5-flash`
  dispatcht (`35745295808`).
- **Blockade:** Lauf-Ergebnis offen.
- **Braucht:** Ergebnis `free-model-bench.tsv` lesen.

### EPA RadNet AGOL
- **Status:** wartend | **Bindung:** eigen
- **Lage:** CI `35649300152` success (157180 records, agol 140 bad 0);
  `blocked_sources.φ:56`, `sources.φ:1164`.
- **Blockade:** RadNet-Features tragen keine Koordinaten; FRS-Weg geschlossen.
- **Braucht:** Koordinaten-Route (Site/County-Join) oder descope mit Messung.

### DEMETER ISL Download
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Parser-Gate gebaut (`demeter.rs:61`); Order 18387 (34,71 GB),
  `availableFilesCount=0`; `blocked_sources.φ:52`.
- **Blockade:** CDPP stellt die Dateien nicht bereit.
- **Braucht:** Download bei `availableFilesCount>0`.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Globus-Gruppen gewährt; Transfer `af68c4f1` ACTIVE (6.561 Dateien,
  21,93 GB) → `data/superdarn/map/`; `blocked_sources.φ:16`. FITACF `sources.φ:9465`.
- **Blockade:** Transfer läuft im Hintergrund.
- **Braucht:** bei Abschluss MAP-Compiler bauen + in `sources.φ` registrieren.

### PS1-Footprint
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `footprints.φ:19` — Release assets `[]`; Lauf `35670499832` success
  ohne final-combine/upload.
- **Blockade:** final-combine nicht erreicht (Bänder 651–2643 offen).
- **Braucht:** final-combine-Lauf auf dem Slab-Tag.

### NRS SHAPE
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `nrs_stations.φ:17` — NRS02-10,12,13 ohne Spektren-Verankerung.
- **Blockade:** SHAPE-Format-Anker.
- **Braucht:** SHAPE-Spektren parsen/ankern.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:10-12`; `/tap/tables` 500.
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei `/tap/tables` 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:14-16`; Portal 200, CAS-Login funktioniert, „Permission
  Denied" für omegaflow.
- **Blockade:** PI-Portal.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Lage:** `blocked_sources.φ:3` — api 502 über Proton-Exits, Frontend 200; Token
  vorhanden.
- **Blockade:** Broker-Backend.
- **Braucht:** Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Lage:** `blocked_sources.φ:21` — `bc_mpo_more` release_date 2099-01-01,
  `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### EMODNET HFRADAR NADR
- **Status:** wartend | **Bindung:** termin 2026-10-19
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

## Benchmark

- Kein Doppel-Lauf: die Routine-Klasse ist geschlossen (flash-Sieger, 2026-09-16) —
  zitiert. Diese Session dispatchte 6 grind-Agenten (4 flash, 3 pro) und arbeitete
  4 Punkte selbst; kein neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `phi/blocked_sources.φ`, `phi/sources.φ`,
  `phi/pipeline/index.φ`, `phi/pipeline/catalog/korpora_heim.φ`,
  `src/archivar/geo.rs`, `src/archivar/extract.rs`, `src/archivar/main_flow.rs`,
  `tools/harvest/src/bin/superdarn_rawacf_compiler.rs`,
  `.github/workflows/superdarn-rawacf-cdn.yml`, neues Handover
  `docs/handover/handover-2026-09-22-mycelium-folge139.md`.
- **Move mit dem Commit:** `handover-2026-09-22-mycelium-folge138.md` → `archiv/`.
- **Fremd (nicht angetastet):** `src/mathematikerin/te.rs`, `src/archivar/hdf5.rs`,
  diverse `archiv/`-Handover, `AGENTS.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
