<!--
  title: Handover — Mycelium-Folge 143 (RAWACF-Beweis re-ankert, NRS-Verdikt + CDN-Riss korrigiert, GLM-Disposition geklärt) (Stand 2026-09-23)
  session: Mycelium-Folge 143
  class: handover
  date: 2026-09-23
  sha256: db2e655fa6ddabbed0694993ba00895651c9f763accf94b0204295d41b750068
  status: live
-->
# Handover — Mycelium-Folge 143 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Lage** / **Blockade** / **Braucht**; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session ist der **Ausführungs-Pass** der Mycelium-Linie. Sie hat
`handover-2026-09-23-mycelium-folge142.md` konsumiert und drei Taucher
parallel dispatcht: RAWACF-Beweis-Pfad (grind-flash), NRS-Register-Lücke
(grind-pro), NOAA-S3-GLM-Disposition (grind-flash).

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** beim Start `32de9f3d0`; Arbeitsbaum == HEAD; `origin/main == HEAD`.
- **Postfach:** `post.md` trägt nur die Template-Regelzeile; jüngster
  `mail_ledger`-Eingang `1790116573` (Mandrill-Digest, keine Aktion); davor
  PINE64-Ox64-Versand + GitHub-Support-Ticket 4761801 (PII-Objekte gepurged,
  Pfad 404) — kein neuer mycelium-Eingang, keine Aktion.
- **CI** (`ci_manage list`): `ps1-cdn 35805462755` pending, `35802481393`
  in_progress; `babamul-cdn 35805467066`, `superdarn-rawacf-cdn 35805464999`,
  `noaa-nrs-psd-cdn 35805469550` success; `ci-check 35805438445` in_progress,
  ältere `ci-check` cancelled (superseded).
- **`register_lookup --open`:** 560 offen; pipeline: ledger 2, index 11,
  sources 2, witnesses 4, footprints 2, harvest 0, nrs 0, probes 0, 1 candidate.
- **`open_points_check` folge142:** 17 Pfad-Refs, 2 absent — beide
  `docs/references/general/rawacf.md` (der stale Punkt selbst).

## Diese Session geschlossen (git trägt es)

- **RAWACF-Beweis-Pfad re-ankert.** Phantom-Pfad `docs/references/general/rawacf.md`
  existiert nicht (Baum `docs/references/` fehlt vollständig). Der Anker sitzt im
  Register: `phi/sources.φ:8040` trägt jetzt die gemessene RST-Quell-URL
  (`raw.githubusercontent.com/SuperDARN/rst/…/rprm.c`; `archive_search --sniff`
  HTTP 200, 15940 B, sha256 `116a8c11…`; frang/lagfr als DATASHORT ohne Sentinel,
  Z. 283–284) und die frang=0-null-echt-Begründung (lagfr·c/2 = Abstand erstes
  Range-Gate). Ein Code-Kommentar in `superdarn_rawacf_compiler.rs:497` wurde vom
  `commit_check`-Gate blockiert („comments are dead") und nicht gesetzt. Lebende
  Dokumente mit Phantom-Pfad: keine (nur archivierte folge141 = Historie).
- **NRS-Verdikt gefällt und umgesetzt.** Ein declined Spektral-Asset braucht
  keine `sources.φ`-`url`-Zeile (Oszillator-Gate schlägt fehl; Präzedenz
  ONC-Hydrophon `declined_sources.φ:3546`, Council `c3f33f6`). `declined_sources.φ:3576`
  Note auf gemessenen Stand gesetzt: CDN HTTP 200, 214433228 B, sha256
  `fbb28983…`. Damit die CDN-Manifest-Duty sauber benannt.
- **NRS-CDN-Riss korrigiert.** `nrs_stations.φ:17` trug die stale Teil-Messung
  `79749568B sha256(part)781ea891`; autoritativ gemessen via Release-Asset-Digest:
  214433228 B / sha256 `fbb2898362048eab6a23f1a2455e481913539613751a8bf7c2fe09060653c148`
  (updated 2026-09-07). `archive_search --sniff` bricht bei ~96 MB ab
  („download incomplete") — kein verwertbarer Voll-hash; der Release-Digest ist
  die Messung.
- **NOAA-S3-GLM-Disposition geklärt.** `sources.φ:8050` war Scanner-False-Positive
  (Substring „offen" in „offener Bucket"); Quelle vollständig registriert
  (`glm_l1b.bin` :8042, `glm_l2.bin` :8052) und beide Assets manifestiert
  (`--sniff` HTTP 200; l1b sha256 == :8046). Note `:8050` präzisiert
  (Token-Wort entfernt, `:8044` → `:8046`); kein `goes16`-L2-Gap (eine
  Format-Zeile genügt, Präzedenz `:1088–1090`). `register_lookup --open` taggt
  die Zeile nicht mehr.

## Offen (aufgeschlüsselt)

### PS1-Footprint final-combine
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Asset absent (HTTP 404); Läufe `35805462755` pending, `35802481393`
  in_progress; Legacy-Fix `ps1-cdn.yml:147–153` greift im nächsten vollständigen Lauf.
- **Blockade:** Ernte-Durchsatz (Stunden-Schedule).
- **Braucht:** nächster vollständiger Lauf; dann sha256 + `phi/footprints.φ:19`.

### Katalog-Wald (Rest)
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** Merge 0/72 ohne Fabrikation; Kandidatenzeilen nur `url` + Probe;
  `ttl`, Frame, `field`(force·unit·τ) fehlen; Kandidat
  `phi/pipeline/catalog/archeology_gaps_index.φ` (→ mycelium).
- **Blockade:** fehlende Force/Unit/τ — dieselbe Größe wie pre-cdn.
- **Braucht:** pre-cdn-Konverter-/Metadata-Pfad; Duplikat-Prüfung.

### pre-cdn Join
- **Status:** blockiert | **Bindung:** eigen (Teile operator-gebunden → future)
- **Lage:** `src/archivar/port.rs:380` port_mode; `field` 3-Token → τ-Gate
  `src/archivar/parse.rs:866`; `port_field_synth` (`port.rs:11`) fabriziert Einheit
  „1" + τ=ttl/10; 21 `on`-Blöcke ohne alt; 709 `source`-Direktiven ohne Parser-Arm;
  fehlende Felder per Live-HAPI (`hapi_meta_params`, `port.rs:1070`) messbar.
- **Blockade:** (a) Riss-Politik imag-data (15 URLs) braucht Operator-Segen;
  (b) JSON-Rest ~400 Blöcke `pending review`; (c) 13. Korpus (5206 Blöcke,
  `index.φ:35`) — Aufnahme ins Register?
- **Braucht:** Operator-/Ratswort (a)/(c) → dann Konverter-Pre-Pass grind-max;
  Operator-Teil als `post.md` an future.

### DataONE-Lizenz
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `/terms`+`/data-policy` (www+old, http+https) 401; Doku nur Software
  Apache-2.0; `korpora_heim.φ:28` + `index.φ:107–108` nachgezogen; bleibt `pending`.
- **Blockade:** Data Policy serverseitig 401.
- **Braucht:** Data Policy per Auth messen oder per-Record-Access-Policy.

### Free-Model-Bench
- **Status:** operator-gebunden | **Bindung:** eigen (→ future)
- **Lage:** kein Defekt — der CI-Watchdog kappt Full-Sweeps (3069 s > 2× Median
  1414 s); Artefakt existiert (ID 10721898235, 5612 B); Rate-Limit 31/105 die Messung.
- **Blockade:** Scope-Widerspruch — Default (~105 Modelle/Job) unter dem
  Watchdog-Fenster strukturell unvollendbar.
- **Braucht:** Scope-Entscheidung (Sharding / begrenzter Default / Watchdog-
  Ausnahme) — Operator-/Ratswort.

### DEMETER ISL
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Order 18387 `RUNNING`, `availableFilesCount: 0`; Parser-Gate
  `demeter.rs:61`.
- **Blockade:** CNES-Order läuft.
- **Braucht:** Re-Messung bei `availableFilesCount > 0`.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Globus-Transfer `af68c4f1` ACTIVE (6561 Dateien/21,93 GB) →
  `data/superdarn/map/`; `blocked_sources.φ:16`; FITACF `sources.φ:9465`; MAP-
  Gruppen-Einladungen quittiert (`mail_ledger` 1790020962/1790021001).
- **Blockade:** Transfer läuft.
- **Braucht:** bei Abschluss MAP-Compiler bauen + in `sources.φ` registrieren.

### src.pas TAP
- **Status:** termin | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:10`; `/tap` 200, `/tap/tables` 500 (PostgreSQL).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei `/tap/tables` 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:14`; Portal + CAS ok, „Permission Denied"; PI quittiert
  (`mail_ledger` 1789567528), wartet auf neue Portal-Anleitung.
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

### witnesses.φ absent (4)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `witnesses.φ:7` (ANTARES-REST), `:13` (Fink-LSST cone), `:37`
  (IceCat-1 IceCube), `:91` (ANTARES-2007-2017) — anonyme Kataloge, em-
  Photometrie/S²-Richtung `absent`; nicht in folge142 gelistet.
- **Blockade:** keine.
- **Braucht:** je Zeile prüfen, ob ein Kraft-/S²-Kanal messbar ist, sonst als
  gemessenes `absent` belassen.

## Benchmark

- **Routine-Klasse geschlossen** (flash-Sieger, 2026-09-16) — zitiert. Dispatches
  dieser Session: 2× grind-flash (RAWACF, GLM) + 1× grind-pro (NRS-Verdikt) —
  flash-first; grind-pro nur für das Register-/Verdikt-Urteil. Kein pro/max.
- **Befund (an mountain via `post.md`):** `sfetch` verändert den Body einer
  Text-Ressource — RAWACF-RST `rprm.c`: `curl`/`--sniff` 15940 B, `sfetch`
  15659 B (281 B Differenz, Zeilen-Offset verschoben).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `phi/sources.φ`, `phi/declined_sources.φ`,
  `phi/nrs_stations.φ`, `docs/handover/post.md`, neues Handover
  `docs/handover/handover-2026-09-23-mycelium-folge143.md`.
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge142.md` → `archiv/`.
- **Fremd:** keine Änderung angetastet.

## Sicherheits-Befund (gemeldet, ungeprüft)

Ein grind-flash-Taucher meldete einen eingeschleusten Instruktionsblock (Pfad
`.agents/…` (relativ zu `$HOME/projects`), `<system_warning>` mit Aufforderung, ein
„session token" auszugeben) und verweigerte ihn. Herkunft ungemessen. Für
Operator/Council benannt — nicht geglättet.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
