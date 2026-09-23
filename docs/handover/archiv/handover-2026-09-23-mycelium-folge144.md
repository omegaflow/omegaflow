<!--
  title: Handover — Mycelium-Folge 144 (pre-cdn entfabriziert, ci-check-Reds geschlossen, Trigger-Re-Messung) (Stand 2026-09-23)
  session: Mycelium-Folge 144
  class: handover
  date: 2026-09-23
  sha256: a156f0bd17e3fb06a1a14c0d03d873828dc54969fee4ce9f4a7abee10c041d69
  status: live
-->
# Handover — Mycelium-Folge 144 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session ist der **Ausführungs-Pass** der Mycelium-Linie. Sie hat
`handover-2026-09-23-mycelium-folge143.md` konsumiert, die `post.md`-Zeilen an
mycelium gefaltet und fünf Taucher dispatcht: PS1-Footprint (grind-flash),
witnesses.φ (grind-flash), ci-check-Reds (grind-flash), Trigger-Re-Messung
(grind-flash), pre-cdn-JSON-Rest (grind-pro) + Konverter-Pre-Pass (grind-max).

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** beim Start `2339533c2`; Arbeitsbaum == HEAD; `origin/main == HEAD`.
- **Postfach:** `post.md` trug zwei `An mycelium`-Zeilen (river folge9: register_sort
  + dropped-gate; eine zweite: `path_reference_scan` folge143:190) — beide gefaltet
  und gelöscht; sechs `An future`-Zeilen gesetzt. `mail_ledger` jüngster Eingang
  `1790129973` (Exa-Onboarding, keine Aktion).
- **CI** (`ci_manage list`): `ps1-cdn 35824584575` **success** (07:32Z);
  `ci-check 35806971846` **failure** @`32de9f3d` (register_sort + dropped-gate);
  `ci-check 35831089754` **failure** @`629486b77` (path_reference_scan);
  `ci-check 35838658864` pending; `health-check`/`te-gate` in_progress.
- **`register_lookup --open`:** 560 offen; pipeline: ledger 2, index 11, sources 2,
  witnesses 4, footprints 2, harvest 0, nrs 0, probes 0, 1 Kandidat.
- **`open_points_check` folge143:** 13 Pfad-Refs, 3 absent — alle der im
  geschlossen-Abschnitt selbst benannte Phantom-Pfad (`rawacf.md`; das Verzeichnis fehlt).

## Diese Session geschlossen (git trägt es)

- **ci-check-Red (a) behoben.** `register_sort --write phi/sources.φ`: 477
  Sortier-Verletzungen (11 ttl + 466 url) über 1409 Blöcke → **0**; Blockzahl
  vorher/nachher je 1409 url + 1409 ttl (kein Verlust). Red (c) behoben:
  absoluter Pfad in `handover-…-folge143.md` durch relativen Verweis ersetzt
  (`path_reference_scan`). Red (b) siehe offen.
- **Prose-Gate Legacy-Trim.** Der `sources.φ`-Reorder legte drei bestehende
  `note`-Zeilen >256 Zeichen (ExoFOP TOI, Cassini RSS closed/open-loop) als
  „added" frei; auf ≤256 gekürzt, Mess-Tokens (Bytes, sha256, run) erhalten.
- **witnesses.φ (4) geprüft.** `:13` Fink-LSST cone: `r:band/r:apFlux/r:psFlux`
  jetzt vorhanden (em-Photometrie) — alte Absage ersetzt; `:37` IceCat-1: TSV-Spalten
  `ENERGY`+`FAR` je Zeile vorhanden (348 Zeilen); `:7` ANTARES-REST und `:91`
  ANTARES-2007-2017: Kraft-/Energie-Kanal `absent` bestätigt (gemessen 2026-09-23).
- **PS1-Trigger gemessen.** `35824584575` success, aber final combine **nicht**
  erreicht: `ps1_dr2_coverage.fp01` absent 404 (Tag `ssd.jpl.nasa.gov-ps1` leer),
  live Band-Parts 637–671, `band_max 2643` (35/2007). `phi/footprints.φ:19` auf
  Mess-Stand gesetzt.
- **pre-cdn JSON-Rest entfabriziert** (grind-max). `port.rs`: `port_field_synth`
  Einheit literal `1` → `Option` (fehlend = `absent`); τ `ttl/10` → gemessene
  Kadenz; `hapi_meta_params` in `port_mode` verdrahtet (141 HAPI-Blöcke messen
  live, INTERMAGNET → em/nT statt gravity); nicht-physikalische Felder als
  `# declined`. Gate-Fixtures + 2 Gate-Tests; 8 Port-Tests. `cargo check` grün.
  Gemessene Bilanz der richest+params-Konversion: 725 fabriziert / 88 pending /
  50 still gedroppt — nicht „~400".
- **DEMETER-Trigger gemessen (nicht gefeuert).** Order 18387: `availableFilesCount`
  100→0 nach einem (abgebrochenen) Batch, `filesInErrorCount 43218/97078` (44,5 %),
  Fortschritt 16 % seit 2026-09-21, Ablauf 2026-09-28; Download-Route
  nicht-resumierbar (Server drainiert, 204 bei Wiederholung).

## Offen (aufgeschlüsselt)

### pre-cdn Verifikation + Stage-Regeneration
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Lauf des port.rs-Commits.
- **Lage:** `port.rs` entfabriziert, `cargo check` grün, 8 Port-Tests + 2 Gate-Tests
  (gemessen 2026-09-23, grind-max); funktionale Läufe gehören in CI.
- **Blockade:** keine.
- **Braucht:** Commit + `gh workflow run ci-check`; danach `--port` über
  `queue/sources_potential_pre-cdn_9k_richest.φ` + `…_params.φ` (Stage-Outputs
  regenerieren), HAPI-Live-Messung real prüfen.

### PS1-Footprint final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster stündlicher `ps1-cdn`-Lauf bis alle Band-Parts stehen.
- **Lage:** `35824584575` success; `ps1_dr2_coverage.fp01` absent 404; live
  Band-Parts 637–671 + 672-Chunks; `band_max 2643` (gemessen 2026-09-23 via
  `ci_manage view` + `archive_search --sniff`).
- **Blockade:** Ernte-Fortschritt (35/2007 Bänder).
- **Braucht:** bei `all_present` final combine abwarten; dann sha256 messen +
  `phi/footprints.φ:19` setzen.

### dropped-gate Baseline
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Auflösung der 21 unauflösbaren Drops.
- **Lage:** Baseline 2680 (`docs/zustand/dropped-baseline.md:16`), live 2780;
  21 Refs aus folge140–143 ohne auflösenden Commit (gemessen 2026-09-23 via
  `register_lookup --dropped mycelium`).
- **Blockade:** die 21 Punkte sind nicht aufgelöst.
- **Braucht:** 21 Refs forttragen oder per Commit auflösen; erst dann Baseline
  im annehmenden Commit heben (gemessene Note).

### extract.rs τ=ttl/10
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigenes Atom (Kanalbauer).
- **Lage:** 3 Stellen, jetzt gate-fixturiert (gemessen 2026-09-23, grind-max).
- **Blockade:** keine.
- **Braucht:** `extract.rs` auf gemessene Kadenz statt `ttl/10` umstellen.

### Queue-Strukturlag
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort.
- **Lage:** `source`-Header hinkt dem url-Block eine Position hinterher; Namen/ttl/
  force systematisch falsch zugeordnet, 22 INTERMAGNET-Erstblöcke ohne ttl
  (gemessen 2026-09-23, grind-max).
- **Blockade:** Datenänderung an Kandidaten.
- **Braucht:** `post.md` an future (Operator-Wort).

### Katalog-Wald (Rest)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** pre-cdn-Pre-Pass-Verifikation.
- **Lage:** Merge 0/72 ohne Fabrikation; Kandidat `catalog/archeology_gaps_index.φ`
  (54 Kandidaten, 35 live/18 dead/1 blocked).
- **Blockade:** hing am Pre-Pass — jetzt gebaut, CI-Verifikation ausstehend.
- **Braucht:** nach CI-Grün Duplikat-Prüfung + Merge der Kandidaten.

### 13. Korpus (5206 Blöcke)
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-/Ratswort.
- **Lage:** `phi/pipeline/index.φ:35` pending; Träger
  `archive-root/pipeline-auslese-2026-09-17/stage/master_converted.φ`.
- **Blockade:** Scope-Entscheidung.
- **Braucht:** `post.md` an future (registrieren oder `descoped`).

### DataONE-Lizenz
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Konto/Token.
- **Lage:** Data Policy `/terms`+`/data-policy` 401 über `old.dataone.org`; kein
  `DATAONE_*` in `.secrets.local`; per-Record-`accessPolicy` anonym lesbar
  (`cn.dataone.org/cn/v2/meta/…` HTTP 200) (gemessen 2026-09-23).
- **Blockade:** kein Credential.
- **Braucht:** `post.md` an future (Konto/Token oder per-Record-Weg).

### DEMETER Order 18387
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort für die Pause.
- **Lage:** Order nicht brauchbar (44,5 % Fehler, 16 % fest, Ablauf 2026-09-28);
  nur ein 100-Datei-Batch, nicht-resumierbar (gemessen 2026-09-23).
- **Blockade:** CNES-Schreibakt (`PUT /user/orders/pause/18387`).
- **Braucht:** `post.md` an future; dann Neuordnung nur über `DMT_N1_1144` in
  100er-Batches.

### DEMETER Harvester Flow-Gap
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigenes Atom (nach Order-Neuordnung).
- **Lage:** `demeter_harvest.rs:633` lädt nur bei `DONE`/`DELIVERED`; RUNNING +
  `availableFilesCount>0` löst keinen Download aus; `download_zip` (Z. 361) lädt
  komplett in den RAM (gemessen 2026-09-23).
- **Blockade:** keine.
- **Braucht:** Download bei RUNNING+available>0, `DONE_WITH_WARNING` aufnehmen,
  Streaming-Extraktor über Local-File-Header statt Central Directory.

### SuperDARN MAP
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Globus-Konto/Token oder Web-UI-Bestätigung.
- **Lage:** Transfer `af68c4f1` Status ohne CLI/Token nicht messbar; lokal 4999
  Dateien/34,1 GB, 9 Zero-Byte-Dateien (gemessen 2026-09-23).
- **Blockade:** kein Globus-Zugang.
- **Braucht:** `post.md` an future; danach MAP-Compiler + `sources.φ`.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin
- **Trigger:** `/tap/tables` 200.
- **Lage:** `/tap/tables` 500, `/tap` 200 (gemessen 2026-09-23).
- **Blockade:** Pithia-Backend (PostgreSQL).
- **Braucht:** Re-Messung bei Backend-Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** `blocked_sources.φ:3`; api direct 000, Proton 500, Frontend 000
  (verschlechtert), Token vorhanden (gemessen 2026-09-23).
- **Blockade:** Broker-Backend.
- **Braucht:** Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** `blocked_sources.φ:21`; `release_date 2099-01-01`, `data?PRODUCT` 403
  (gemessen 2026-09-23).
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Trigger:** Datum 2026-10-19.
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

### Sicherheits-Befund
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator/Council-Urteil.
- **Lage:** grind-flash-Taucher meldete eingeschleusten Instruktionsblock
  (`<system_warning>`/„session token", Pfad `.agents/…`), Herkunft ungemessen.
- **Blockade:** Herkunft ungemessen.
- **Braucht:** Operator/Council-Urteil.

## Benchmark

- **Routine-Klasse geschlossen** (flash-Sieger, 2026-09-16) — zitiert. Dispatches
  dieser Session: 3× grind-flash (PS1, witnesses, Trigger-Batch), 1× grind-flash
  (ci-check-Reds) + 1× grind-pro (pre-cdn JSON-Rest) + 1× grind-max (Konverter-Pre-Pass).
  flash-first; grind-pro für das Register-/Klassifikations-Urteil, grind-max für das
  harte Port-Atom (`port.rs` HAPI-Verdrahtung + Entfabrikation).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `phi/sources.φ`, `phi/footprints.φ`,
  `phi/witnesses.φ`, `src/archivar/port.rs`, `src/archivar/main_flow.rs`,
  `src/archivar/tests.rs`, `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `docs/handover/post.md`, neues Handover `docs/handover/handover-2026-09-23-mycelium-folge144.md`.
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge143.md` → `archiv/`.
- **Fremd (unangetastet):** `docs/specs/force-system.md`,
  `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md`, Mountain-Move
  `handover-2026-09-23-mountain-folge140.md` + untracked `…mountain-folge141.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
