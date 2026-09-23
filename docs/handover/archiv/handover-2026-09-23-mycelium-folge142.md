<!--
  title: Handover — Mycelium-Folge 142 (CDN-Manifestationen geschlossen: RAWACF + Babamul sha256 registriert, NRS/PS1 gemessen; RAWACF-Beweis-Pfad stale) (Stand 2026-09-23)
  session: Mycelium-Folge 142
  class: handover
  date: 2026-09-23
  sha256: acf2b4d432193be9195f318ea081b5ae939c322d1de3b0b005ec25339703b41d
  status: live
-->
# Handover — Mycelium-Folge 142 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Lage** / **Blockade** / **Braucht**; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session ist der **Ausführungs-Pass** der Mycelium-Linie. Sie hat
`handover-2026-09-23-mycelium-folge141.md` konsumiert und drei grind-flash-Taucher
parallel dispatcht (CDN-Verifikation+Registrierung): RAWACF+Babamul (sources.φ),
NRS (nrs_stations.φ), PS1 (footprints.φ).

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** beim Start `7e9ae7af1`; Arbeitsbaum == HEAD; `origin/main == HEAD`
  (`git_safety --snapshot`: nothing to record).
- **Postfach:** `post.md` leer (nur Template-Regelzeile); jüngster `mail_ledger`-Eingang
  `1790116573` (Mandrill-Digest, Maschinendigest, keine Aktion) — kein neuer
  mycelium-Eingang, keine Aktion.
- **CI** (`ci_manage list`): die vier folge141-Dispatches — `superdarn-rawacf-cdn
  35805464999` success, `babamul-cdn 35805467066` success, `noaa-nrs-psd-cdn
  35805469550` success, `ps1-cdn 35805462755` pending; `ci-check 35805438445`
  in_progress; alter `ci-check 35796139491` failure @`576dddf98` (superseded).
- **`register_lookup --open`**: 557 offen; pipeline: ledger 3, index 11, sources 3,
  witnesses 4, footprints 2, 1 candidate.
- **`open_points_check` folge141**: 24 Pfad-Refs, **1 absent** —
  `docs/references/general/rawacf.md` (Pfad fehlt im Baum).

## Diese Session geschlossen (git trägt es)

- **RAWACF CDN** `superdarn_rawacf.bin`: HTTP 200, 215288 B, sha256
  `6f60033e…d8a0ba` → `phi/sources.φ:8036` + Manifest-Note.
- **Babamul CDN** `babamul_alerts.bin`: HTTP 200, 2000013 B, sha256
  `934858db…3403d7` → `phi/sources.φ:14022` + Note; `phi/pipeline/ledger.φ`
  Babamul `ausstehend` → `disponiert` (Port kompiliert+manifestiert).
- **NRS**: Lauf `35805469550` idempotent („already present"); Asset HTTP 200,
  79749568 B, sha256(part) `781ea891` → `phi/nrs_stations.φ:17` nachgezogen.
- **PS1**: Asset `ps1_dr2_coverage.fp01` absent (HTTP 404, Tags
  `ssd.jpl.nasa.gov` + `-ps1`); final-combine nicht erreicht → `phi/footprints.φ:19`
  auf gemessenen Stand gesetzt (Band-Fortschritt nicht gemessen).

## Offen (aufgeschlüsselt)

### PS1-Footprint final-combine
- **Status:** wartend | **Bindung:** eigen
- **Lage:** final-combine korrekt gegatet, nie erreicht — Asset absent (HTTP 404);
  Läufe `35805462755` pending, `35802481393` in_progress, Log 404. Der Legacy-Fix
  (`ps1-cdn.yml:147–153`: Löschung auf gekapptem `ssd.jpl.nasa.gov`, Upload Final
  auf `ssd.jpl.nasa.gov-ps1`) steht und greift im nächsten vollständigen Lauf.
- **Blockade:** Ernte-Durchsatz (Stunden-Schedule).
- **Braucht:** nächster vollständiger Lauf; dann sha256 + `phi/footprints.φ:19`.

### RAWACF-Beweis-Pfad stale
- **Status:** wartend | **Bindung:** eigen
- **Lage:** die RAWACF-`frang=0`-null-echt-Begründung (folge141) zitiert
  `docs/references/general/rawacf.md:61` — dieser Pfad existiert nicht; kein
  RAWACF-Dokument und kein RST-Quelltext (`radar.1.22/src/rprm.c`) lokal
  (`sgrep`/`glob` leer). Tragende Evidenz ist extern (RST `rprm.c:281`,
  DATASHORT ohne Sentinel) + lokal `tools/harvest/src/bin/superdarn_rawacf_compiler.rs:497`.
- **Blockade:** keine.
- **Braucht:** Beweis-Linie re-ankern (RST-Quellpfad + Compiler-Zeile) oder ein
  reales Referenzdokument messen/einordnen.

### NRS-Register-Lücke
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `noaa_nrs_psd.bin` manifestiert (HTTP 200, 79749568 B), aber nur in
  `phi/declined_sources.φ:3576` als gestrichener Feldblock geführt — keine
  `sources.φ`-`url`-Zeile. `nrs_audio_series.bin` (`sources.φ:9356`) ist ein
  anderes Asset.
- **Blockade:** Verdikt-/Register-Frage (declined Asset vs. CDN-Manifest-Duty).
- **Braucht:** prüfen, ob ein declined Spektral-Asset eine `sources.φ`-Zeile
  braucht, oder das declined belassen (Rat/Register-Disziplin).

### Katalog-Wald (Rest)
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** Merge gemessen — 0/72 ohne Fabrikation möglich; Kandidatenzeilen
  tragen nur `url` + Probe-Verdikt; `ttl`, Frame, `field`(force·unit·τ) fehlen;
  Großteil steht bereits in `declined_sources.φ`/`sources.φ`/`witnesses.φ`.
- **Blockade:** fehlende Force/Unit/τ — dieselbe Größe wie pre-cdn.
- **Braucht:** pre-cdn-Konverter-/Metadata-Pfad; Duplikat-Prüfung.

### pre-cdn Join
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** research-max-Blueprint. Korruption klein (2–3 Blöcke, imag-data),
  kein Pre-Merge-Original am Datenträger. Queue-Grammatik register-untauglich:
  `field` 3-Token → τ-Gate (`src/archivar/parse.rs:866`); `port_field_synth`
  (`port.rs:11`) fabriziert Einheit „1" + τ=ttl/10; 21 `on`-Blöcke ohne alt;
  709 `source`-Direktiven ohne Parser-Arm. Fehlende Felder per Live-HAPI
  (`hapi_meta_params`, `port.rs:1070`) messbar.
- **Blockade:** (a) Riss-Politik imag-data (15 URLs) braucht Operator-Segen;
  (b) JSON-Rest ~400 Blöcke ohne Metadatenquelle = `pending review`; (c) 13.
  Korpus (5206 Blöcke, `index.φ:35`) — Aufnahme ins Register?
- **Braucht:** Operator-/Ratswort zu (a)/(c); dann Konverter-Pre-Pass an
  `src/archivar/port.rs:380 port_mode` — grind-max.

### DataONE-Lizenz
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `/terms`+`/data-policy` (www+old, http+https) 401; Doku nur Software
  Apache-2.0; MN-Partner-Guidelines 2017 ohne öffentliche Metadaten-Lizenz.
  `korpora_heim.φ:28` + `index.φ:107–108` nachgezogen; bleibt `pending`.
- **Blockade:** Data Policy serverseitig 401.
- **Braucht:** Data Policy per Auth messen oder per-Record-Access-Policy.

### Free-Model-Bench
- **Status:** operator-gebunden | **Bindung:** eigen
- **Lage:** kein Defekt — der CI-Watchdog kappt Full-Sweeps (3069 s > 2× Median
  1414 s); Artefakt existiert (ID 10721898235, 5612 B); Free-Model-Rate-Limit ist
  die Messung (31/105 bis Cancel). „kein Artefakt" widerlegt.
- **Blockade:** Scope-Widerspruch — Default (alle ~105 Modelle in einem Job) ist
  unter dem Watchdog-Fenster strukturell unvollendbar; Median aus gefilterten Läufen.
- **Braucht:** Scope-Entscheidung (Sharding / begrenzter Default / Watchdog-
  Ausnahme) — Operator-/Ratswort.

### DEMETER ISL
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Order 18387 `RUNNING`, `availableFilesCount: 0`; Parser-Gate gebaut
  (`demeter.rs:61`).
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
- **Lage:** `ledger.φ:14`; Portal + CAS ok, „Permission Denied"; PI quittiert.
- **Blockade:** PI-Portal (Umbau).
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
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

## Benchmark

- **Routine-Klasse geschlossen** (flash-Sieger, 2026-09-16) — zitiert, kein
  Doppel-Lauf. Dispatches dieser Session: 3× grind-flash (RAWACF+Babamul, NRS,
  PS1) — alle flash-first; kein pro/max nötig, kein Sieger zu ermitteln.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `phi/sources.φ`, `phi/footprints.φ`,
  `phi/nrs_stations.φ`, `phi/pipeline/ledger.φ`, neues Handover
  `docs/handover/handover-2026-09-23-mycelium-folge142.md`.
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge141.md` → `archiv/`.
- **Fremd:** keine Änderung angetastet.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
