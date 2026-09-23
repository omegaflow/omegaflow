<!--
  title: Handover — Mycelium-Folge 141 (Ausführungs-Pass: RAWACF-frang null-echt, Babamul-Fenster, PS1 final-combine, DataONE-Lizenz, Katalog-Wald-Befund, pre-cdn-Blueprint) (Stand 2026-09-23)
  session: Mycelium-Folge 141
  class: handover
  date: 2026-09-23
  sha256: 2a3305c307b87a09a728176fb2a510be9e585b7b033342a006441cae5bf12f87
  status: live
-->
# Handover — Mycelium-Folge 141 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Lage** / **Blockade** / **Braucht**; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session ist der **Ausführungs-Pass** der Mycelium-Linie. Sie hat
`handover-2026-09-22-mycelium-folge140.md` konsumiert und acht Punkte parallel
dispatcht (grind-flash: Katalog-Wald, DataONE-Lizenz, Free-Model-Bench;
grind-pro: RAWACF, Babamul, PS1; research-max: pre-cdn; plus die GOES/DEMETER-
Marken-Messung im Vorfeld).

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** beim Start `576dddf98`; Arbeitsbaum == HEAD; `origin/main == HEAD`
  (`git_safety --snapshot`: nothing to record).
- Während der Session erschienen **fremde** uncommittete Änderungen
  (`AGENTS.md`, `docs/concepts/tools-map.md`) — eine parallele Linie; **nicht
  angetastet, nicht committet**.
- **Postfach** `post.md`: zwei Nachrichten an mycelium eingefaltet (Such-API-
  stale — Registerzeile `external-state.md:35` nachgezogen; free-model-bench)
  → `post.md` leer. `docs/zustand/external-state.md` ist gitignored (Disk-State).
- **Register-Marken (neu gemessen):** GOES GLM `sources.φ:8048` — CDN-Asset
  `glm_l1b.bin` HTTP 200, 1050368 B, sha256 == `sources.φ:8044` → manifestiert,
  Marker nachgezogen. DEMETER `blocked_sources.φ:47` — Order 18387 `RUNNING`,
  `availableFilesCount: 0`. Babamul `sources.φ:14024` — Asset 404 (Fix s.u.).
- **CI** (`ci_manage list`): jüngste `ci-check` rot auf älteren SHAs; `ps1-cdn`,
  `swpc-mirror-cdn`, `tools-build` success. Watchdog-Snapshot 2026-09-22T23:13.
- **`register_lookup --open`**: 559 offen; pipeline: ledger 3, index 10,
  sources 3, witnesses 4, footprints 2, 1 candidate.
- **`open_points_check` folge140**: 23 Pfad-Refs, 0 absent.
- **`cargo check -p omegaflow-harvest`**: 0 Fehler, 0 Warnungen; `cargo fmt`
  (eigene Pfade) still.

## Offen (aufgeschlüsselt)

### SuperDARN RAWACF CDN-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `frang=0` als **null-echt** gemessen (RST `radar.1.22/src/rprm.c:281`
  liest DATASHORT ohne Sentinel-Pfad; `docs/references/general/rawacf.md:61`
  „Distance to the first range gate"; `frang = lagfr·c/2 = 0 km`). Compiler-
  Schwelle `frang <= 0.0` → `frang < 0.0`
  (`tools/harvest/src/bin/superdarn_rawacf_compiler.rs:497`), Fixture
  `zero_frang_block` + Test `gather_keeps_a_zero_frang_record`; `cargo check` 0/0.
- **Blockade:** keine (Code steht; wirksam erst nach Commit).
- **Braucht:** nach `/commit` `gh workflow run superdarn-rawacf-cdn.yml`; dann
  sha256 in `phi/sources.φ:8032` registrieren.

### Babamul CDN
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Ursache gemessen — das Default-Fenster öffnete **in die Zukunft**
  (`jd_now()..jd_now()+0.999`), Alerts sind vergangene Ereignisse → 0 Treffer
  (Lauf `35745291297`: „zero candidate rows in 53 B"). Gegenprobe: Vergangen-
  heitsfenster liefert 29,7 MB. Fix `jd_now() - JD_WINDOW`
  (`tools/harvest/src/bin/babamul_compiler.rs:82`); `cargo check` 0/0. Workflow
  unverändert (Compile/Manifest-Schritt vollständig).
- **Blockade:** keine (wirksam nach Commit).
- **Braucht:** nach `/commit` `gh workflow run babamul-cdn.yml`; dann sha256 in
  `phi/sources.φ:14016`.

### PS1-Footprint final-combine
- **Status:** wartend | **Bindung:** eigen
- **Lage:** final-combine wird korrekt gegatet (`all_present`), aber nie
  erreicht — Ernte bei Band 665 von 2643 (Durchsatz). Echter Defekt gefunden:
  die Legacy-Löschung zielte auf den Familien-Tag `ssd.jpl.nasa.gov-ps1`
  (0 Assets) statt den gekappten Release `ssd.jpl.nasa.gov` (187
  `ps1_part_*`). Fix `.github/workflows/ps1-cdn.yml:147–153`.
- **Blockade:** Ernte-Durchsatz (Stunden-Schedule).
- **Braucht:** `/commit` (Fix greift im nächsten Lauf, sobald alle Bänder
  stehen); `phi/footprints.φ:19` erst nach gemessenem Upload.

### Katalog-Wald (Rest)
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** Merge gemessen durchgeführt — **0/72** ohne Fabrikation möglich. Die
  Kandidatenzeilen tragen nur `url` + Probe-Verdikt; `ttl`, Frame und
  `field`(force·unit·τ) fehlen; der Großteil steht bereits in den Registern
  (`declined_sources.φ`/`sources.φ`/`witnesses.φ`). `phi/sources.φ` unverändert.
- **Blockade:** fehlende Force/Unit/τ — dieselbe Größe wie pre-cdn.
- **Braucht:** den pre-cdn-Konverter-/Metadata-Pfad (s.u.) für die Rest-
  Kandidaten; Duplikat-Prüfung bleibt.

### pre-cdn Join
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** research-max-Blueprint. Die Korruption ist klein (2–3 Blöcke,
  imag-data-Bereich), aber kein Pre-Merge-Original auf dem Datenträger. Die
  Queue-Grammatik ist register-untauglich: `field` 3-Token → τ-Gate aktiv
  (`src/archivar/parse.rs:866`); `port_field_synth` (`port.rs:11`) fabriziert
  Einheit „1" + τ=ttl/10; 21 `on`-Blöcke ohne alt; 709 `source`-Direktiven ohne
  Parser-Arm. Fehlende Felder sind per Live-HAPI (`hapi_meta_params`,
  `port.rs:1070`) messbar.
- **Blockade:** (a) Riss-Politik imag-data (15 URLs) braucht Operator-Segen;
  (b) JSON-Rest ~400 Blöcke ohne Metadatenquelle = `pending review`; (c) 13.
  Korpus (5206 Blöcke, `index.φ:35`) — Aufnahme ins Register?
- **Braucht:** Operator-/Ratswort zu (a)/(c); dann Konverter-Pre-Pass an
  `src/archivar/port.rs:380 port_mode` — grind-max.

### NRS SHAPE
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Parser repariert; Bucket `nrs/products` trägt SHAPE nur NRS01/11
  (`nrs_stations.φ:17`); NRS02-10,12,13 absent.
- **Blockade:** CI-Lauf noch nicht gefahren.
- **Braucht:** nach `/commit` `gh workflow run noaa-nrs-psd-cdn.yml` (NRS01/11
  ankern; übrige bleiben absent).

### DataONE-Lizenz
- **Status:** wartend | **Bindung:** eigen
- **Lage:** erweitert gemessen — `/terms`+`/data-policy` (www+old, http+https)
  401 (kein Wayback, Sitemap 2026-09-23 ohne Policy-Seite); Doku
  `dataone_documentation`/`Operations_Documentation`/`DataONE_Operations` +
  `purl.dataone.org/architecture/license_and_copyright_policy.html` nur Software
  Apache-2.0; MN-Partner-Guidelines 2017 (Wayback 20200212005757) ohne
  öffentliche Metadaten-Lizenz. `korpora_heim.φ:28` + `index.φ:107–108`
  nachgezogen; bleibt `pending`.
- **Blockade:** aktuelle Data Policy serverseitig 401.
- **Braucht:** Data Policy per Auth messen oder per-Record-Access-Policy.

### Free-Model-Bench
- **Status:** operator-gebunden | **Bindung:** eigen
- **Lage:** kein Defekt — gemessen: der CI-Watchdog kappt Full-Sweeps
  (`/tmp/opencode/ci_watchdog.log:46`: 3069 s > 2× Median 1414 s); das Artefakt
  existiert (ID 10721898235, 5612 B); die Rate-Limitierung der Free-Modelle ist
  die Messung (31/105 bis Cancel, `timeout 6`, `pass 0`). „kein Artefakt" war
  widerlegt.
- **Blockade:** Scope-Widerspruch — der Default (alle ~105 Modelle in einem Job)
  ist unter dem Watchdog-Fenster strukturell unvollendbar; der Median stammt aus
  gefilterten Läufen (Henne-Ei).
- **Braucht:** Scope-Entscheidung (Sharding / begrenzter Default / Watchdog-
  Ausnahme für Messläufe) — Operator-/Ratswort.

### DEMETER ISL
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Order 18387 `RUNNING`, `availableFilesCount: 0` (2026-09-23);
  Parser-Gate gebaut (`demeter.rs:61`).
- **Blockade:** CNES-Order läuft.
- **Braucht:** Re-Messung bei `availableFilesCount > 0`.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Globus-Transfer `af68c4f1` ACTIVE (6561 Dateien/21,93 GB) →
  `data/superdarn/map/`; `blocked_sources.φ:16`; FITACF `sources.φ:9465`.
- **Blockade:** Transfer läuft.
- **Braucht:** bei Abschluss MAP-Compiler bauen + in `sources.φ` registrieren.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:10`; `/tap/tables` 500 (PostgreSQL).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei `/tap/tables` 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:14`; Portal + CAS ok, „Permission Denied"; PI-Antwort
  2026-09-16, quittiert (`mail_ledger:80`).
- **Blockade:** PI-Portal (Umbau).
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Lage:** `blocked_sources.φ:3` — api 502 über Proton-Exits, Frontend 200;
  Token vorhanden.
- **Blockade:** Broker-Backend.
- **Braucht:** Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Lage:** `blocked_sources.φ:21` — `bc_mpo_more` release_date 2099-01-01,
  `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

## Benchmark

- **Routine-Klasse geschlossen** (flash-Sieger, 2026-09-16) — zitiert, kein
  Doppel-Lauf. Dispatches dieser Session: 3× grind-flash (Katalog-Wald, DataONE,
  Bench), 3× grind-pro (RAWACF, Babamul, PS1), 1× research-max (pre-cdn).
- Das **harte Atom** (pre-cdn, Parser-/τ-Grammatik) lief als research-max-Erstlauf
  und lieferte den Blueprint (Konverter-Pre-Pass `port_mode`); ein flash-Mirror
  wäre strukturell unvollständig — kein Sieger nötig, der nächste Schritt ist der
  Konverter-Bau.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `.github/workflows/ps1-cdn.yml`,
  `docs/handover/post.md`, `phi/pipeline/catalog/korpora_heim.φ`,
  `phi/pipeline/index.φ`, `phi/sources.φ`,
  `tools/harvest/src/bin/babamul_compiler.rs`,
  `tools/harvest/src/bin/superdarn_rawacf_compiler.rs`, neues Handover
  `docs/handover/handover-2026-09-23-mycelium-folge141.md`.
- **Move mit dem Commit:** `handover-2026-09-22-mycelium-folge140.md` → `archiv/`.
- **Fremd (nicht angetastet, nicht committet):** `AGENTS.md`,
  `docs/concepts/tools-map.md` (parallele Linie). `docs/zustand/external-state.md`
  ist gitignored (Disk-State, kein Commit).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
