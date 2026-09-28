<!--
  title: Handover — Mycelium-Folge 197 (2026-09-28)
  session: Mycelium-Folge 197
  class: handover
  date: 2026-09-28
  sha256: 7c640c07cdc919c4f163a81b09863d8350e2d4150346fd619037d8ccb106f7d2
  status: live
-->
# Handover — Mycelium-Folge 197 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`;
Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge196.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „naja ich möchte ja erstmal dass du die taucher mit harten bandagen auf die sonden loslässt" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „sofort prinzp warum sid die 2 kerne nicht registriert wir haben doch DE441 DE442 mich wundert was du sagst mir kommt das alles sehr dubios vor hast du wirklich korrekt in cources assetzsd und lokal geschaut?" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „ziehst du die konsquentz strukturell dann musst du sie in opencode/baum verankern" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „Diver-B/C-Befunde einzeln gegen den Baum prüfen?" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „hast du alles bis zur kante geplant?" | Quelle: Mycelium-Session 197.
- Wort | 2026-09-28 | „und hast du den rest auch gemessen?" | Quelle: Mycelium-Session 197.
- Wort | 2026-09-28 | „ja bitte" (die verbleibenden Behauptungen read-only nachmessen) | Quelle: Mycelium-Session 197.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort" | Quelle: Mycelium-Session 197.

## Offen (aufgeschlüsselt)

### Sonden-Ephemeriden — 16 Sonden portieren, 5 Anderson-priorisiert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass (`docs/SOURCE_PORT.md`).
- **Lage:** (gemessen 2026-09-28 via `research-max` + `archive_search --verdict`/`--sniff` + `sgrep`) **16/16** radio-trächtige Sonden ohne Ephemeriden-Asset (Galileo, Ulysses, Cassini, MAVEN, DART, MESSENGER, MGS, MRO, Odyssey, Magellan, Dawn, Mars Express, Rosetta, VEX, LRO, Pathfinder). Für die 5 Anderson-Sonden sind NAIF-Routen **voll** gemessen: Galileo `naif.jpl.nasa.gov/pub/naif/GLL/kernels/spk/s970311a.bsp` (1496064 B, sha256 `61b3f580…`; Interplanetary Cruise 1989-10-19→1995-07-02, deckt beide Earth-Flybys) · Cassini `…/CASSINI/kernels/spk/990807A_SCEPH_EM52_EP13.bsp` (156672 B, sha256 `4316e465…`; COVER `1999-06-27→1999-08-31`, **trägt den Earth-Flyby 1999-08-18**; EM52=Earth−52 d, EP13=Earth+13 d) · Rosetta `…/ROSETTA/kernels/spk/ORER_______________00031.BSP` (467968 B, sha256 `5951829f…`; der ESA-PSA-Spiegelpfad ist 404, NAIF trägt) · MESSENGER `…/messsp_1000/data/spk/msgr_040803_120516_140823_od268sc_0.bsp` (162024448 B, sha256 `373c78e9…`) · NEAR `…/nearsp_1000/data/spk/near_cruise_nav_v1.bsp` (35403776 B, sha256 `7cfa191b…`; NEAR trägt weder ODF noch SPK). Zusatz-Kernel (Galileo Jupiter-Tour, **nicht** die Flybys) `…/GLL/kernels/spk/gll_951120_021126_raj2021.bsp` (44949504 B, sha256 `96a6cb71…`; Coverage 1995-11-21→2002-11-26). Der generische `tools/harvest/src/bin/ephemeris_compiler.rs` ist ein **rekursiver SPK/PCK-Harvester**, kein Einzel-Sonden-Tool.
- **Blockade:** keine.
- **Braucht:** je Sonde die NAIF-Route über `docs/SOURCE_PORT.md` portieren (`spacecraft_table`-Eintrag + Compiler/Workflow nach `mariner10-ephemeris-cdn.yml`), dann `phi/sources.φ`-Block (`format ephemeris_binary`, `origin <NAIF-URL>`, `at <sonde>`, `no-cadence`).

### ODF-Coverage der Flyby-Fenster
- **Status:** wartend | **Bindung:** eigen (mit sensory)
- **Trigger:** die `ephemeris_<sonde>`-Blöcke in `phi/sources.φ` stehen (5 NAIF-Routen registriert).
- **Lage:** (gemessen 2026-09-28 Rat) ungemessen, ob `galileo_odf`/`cassini_odf`/`rosetta_odf`/`messenger_odf` die Flyby-Fenster wirklich tragen (Spanne ≠ Record-Coverage).
- **Blockade:** keine.
- **Braucht:** je ODF-Asset die Record-Coverage gegen das Flyby-Datum messen (Galileo Ⅰ 1990-12-08 / Ⅱ 1992-12-08 · Cassini 1999-08-18 · Rosetta 2005-03-04 · MESSENGER 2005-08-02).

### Register-Kandidaten `index.φ` (8)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass (`docs/SOURCE_PORT.md`).
- **Lage:** (gemessen 2026-09-28 via `sread phi/pipeline/index.φ`) 8 `verifiziert` (owner mycelium): `queue/sources_potential_pre-cdn_9k_richest.φ` `:37`, `…_params.φ` `:39`, `catalog/oai_arxiv.φ` `:75`, `b2find_intermagnet_catalog.φ` `:79`, `terrapulse_catalog.φ` `:89`, `esa_geomagnetic_catalog.φ` `:91`, `archeology_gaps_index.φ` `:93`, `copernicus_catalog.φ` `:97`.
- **Blockade:** keine.
- **Braucht:** die 8 Inventare über `docs/SOURCE_PORT.md` portieren.

### pds3/pds4 — CDN-Erstlauf dispatchen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-fixed-width-cdn.yml` / `pds4-fixed-width-cdn.yml` liegen auf `origin/main` (dieser Commit).
- **Lage:** (gemessen 2026-09-28) zwei Workflows geschrieben (`pds3-fixed-width-cdn.yml`, `pds4-fixed-width-cdn.yml`) + zwei `phi/harvest.φ`-Blöcke (`asset fehlt`); Asset-Namen sind Familien `pds3_fixed_width_<stem>.bin` / `pds4_fixed_width_<stem>.bin`.
- **Blockade:** keine.
- **Braucht:** `gh workflow run pds3-fixed-width-cdn.yml` und `pds4-fixed-width-cdn.yml`, dann `ci_manage jobs <id>`; nach grünem Lauf sha256/size in die `note` und `asset fehlt` → `asset present`.

### DAS2 Iowa + Occultation-DB UTFPR — Workflow nach Admission
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission (`phi/blocked_sources.φ:363-368`).
- **Lage:** (gemessen 2026-09-28) beide Arme gebaut (`b4106e69a`), aber kein `phi/sources.φ`-Block und kein Workflow (`find .github/workflows -iname '*das2*' -o -iname '*utfpr*' -o -iname '*occultation*'` → kein Treffer); UTFPR-Compiler `occultation_compiler.rs`, DAS2 nur generischer `port.rs`-Arm.
- **Blockade:** Admission-Verdikt (Mountain).
- **Braucht:** nach Admission: `format`/`tag`/`pattern` + Workflow `das2-iowa-cdn.yml`/`utfpr-occultation-cdn.yml` nach Muster.

### ci-check — Bestätigungslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des jeweiligen lebenden `ci-check`-Laufs auf dem HEAD (generisch, keine Run-ID pinnen — jeder Push überholt sie).
- **Lage:** (gemessen 2026-09-28) folge196 pinnte `36413456788` (längst überholt); clippy-Rot `src/archivar/odf.rs:209` in `9f8debcc3` geheilt.
- **Blockade:** keine.
- **Braucht:** `ci_manage status` beim nächsten Pass; bei Rot `ci_manage log <id>`.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`/`jobs`) `36344350143` rot, Job-Log `unread`.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143`.

### ci_watchdog — Matcher heilt echte Runner-Shutdowns
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf, belegt in `ci_watchdog.log`.
- **Lage:** (gemessen 2026-09-28 via `bash -n` + `sread bin/ci_watchdog.sh`) transient-first, Assertion-Klasse begrenzt.
- **Blockade:** keine.
- **Braucht:** der nächste Shutdown-Rot trägt „rerun … measured transient cause".

### D5-Orphan-Residuum — Röhren-Asset-CDN-Weg
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:383` §14.4 Punkt 4; nicht `:288` — §10 S²-Kugel).
- **Lage:** (gemessen 2026-09-28 via `sgrep`/`sread`) kein Producer-Bin (`sgrep -i roehre tools` leer), keine Register-Zeile, kein `*-cdn.yml`. Label „D5" ist ein Mountain-Bundle-Kürzel (Orphan-Doc-Reconciliation).
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht (Röhren-Feld aus `zeugnis.md` §14).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict`) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

### Träger — 11 Prosadokumente ohne Namenträger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `register_lookup --orphan-docs` meldet einen ORPHAN_DOC (gefeuert).
- **Lage:** (gemessen 2026-09-28 via `register_lookup --orphan-docs`) **11** trägerlose Dokumente: `arxiv-api` 2 · `exzellenz-konzept` 3 · `pfeiler-der-architektur` 2 · `tools-map` 8 · `survey-…-daten-holdings-inventur` 3 · `survey-…-orphan-verdicts` 7 · `survey-…-tmp-opencode-scan` 3 · `survey-…-kapitulationen-pendings-inventur` 22 · `survey-…-dead-sources-relevanz` 3 · `survey-…-browser-anbindung` 3 · `survey-fortschritt` 1.
- **Blockade:** teils Operator-Wort/Trigger der Owner.
- **Braucht:** je Dokument einen Namenträger in der Owner-Übergabe (mountain/river) — als `## An mountain`/`## An river`-Zeilen getragen; beim nächsten Pass prüfen.

## An mountain

- **DEMETER Dispositions-Klasse** | (gemessen 2026-09-28) `phi/blocked_sources.φ:86-88` trägt `pending` (url `regards.cnes.fr/api/v1/rs-order`, Order 18387). Die sensory-Angabe `:81-83`/Klasse `ip-blocked` ist **falsch** — `:81-83` ist der descoped PDS-ODF-Arm; der Datei-Endpoint ist **UA-gated** (curl ohne Browser-UA 403 / mit UA 202), kein IP-Block. Verdikt-Klasse (Dispositions-Register = Mountains Feder) neu zu setzen (`parser-def`/Route-Gate?). Braucht: Mountains Verdikt.
  Origin: mycelium-folge197
- **DAS2 Iowa + Occultation-DB UTFPR — Admission** | (gemessen 2026-09-28) beide Arme gebaut (`b4106e69a`), `phi/blocked_sources.φ:363-368` führt sie als `pending`; es fehlt die Admission (`format`/`tag`/`pattern`) für einen `phi/sources.φ`-Block. Braucht: Mountains Admission-Verdikt.
  Origin: mycelium-folge197

## An river

- **flyby_anderson_probe — kein Weberin-Arm** | (Rat-Entscheidung 2026-09-28, gemessen) Ein Doppler-Residuum (mm/s) ist keine Bahnquelle; `BodyLine = {Spk, Dastcom, Mpc, Inpop, Epm}` (`src/weberin.rs:32-38`) und die `sep_m`-Faltung bleiben unberührt. Die Position der Anomalie ist ein Probe-Bin `tools/measure/src/bin/flyby_anderson_probe.rs` nach Muster `flyby_ephemeris_gate.rs` (`d310d5888`); Residuum-Typ `FlybyResidual { t_utc, dz_meas_mm_s: Option<f64>, dz_pred_mm_s: Option<f64> }` neben `doppler.rs`, Anderson-Konvention **+ toward Earth**. Reihenfolge: Ephemeriden (Mycelium) → ODF-Coverage → Probe → Verdict (Mountain). Braucht: Rivers Bau; die Ephemeriden-Voraussetzung steht als Mycelium-Punkt.
  Origin: mycelium-folge197

## An future

- **gic-causal-driver — DOIs** | (gemessen 2026-09-28) `docs/paper/gic-causal-driver.md:531` (Repository-DOI) und `:538` (Software-DOI) sind `pending`. DOI-Minting ist ein Schreibakt an einem Dritten (DataCite/Zenodo) → Operator-Hand, nicht Maschinen-Akt. Braucht: Operator-Wort nach Vorbereitung (Metadaten-Draft) — in Futures Operator-Queue.
  Origin: mycelium-folge197

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). **Operator-Wort 2026-09-28 (folge196): Commit erst, wenn alle anderen Sessions committet haben** — der geteilte Baum trägt fremde uncommittete Arbeit (`.github/workflows/ci-check.yml` geändert, `ci-gate.yml` untracked), die nicht berührt wird.
