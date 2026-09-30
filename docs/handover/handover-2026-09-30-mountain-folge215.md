<!--
  title: Handover — Mountain-Folge 215 (Stand 2026-09-30)
  session: Mountain-Folge 215
  class: handover
  date: 2026-09-30
  sha256: 37dd4ef1462bf016e1a56a627b72c3a4e0b2e5b402581603ac143fd76897531b
  status: live
-->
# Handover — Mountain-Folge 215 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; Stand der Runde am
`d8c0d8340`, vor dem Mountain-Commit `8fdb361b3`). Dieses Atom: die „Ganz einfach"-Ausführung
des Operators aufgenommen und gegen den Baum gemessen — die drei Ephemeriden-Häuser
(DE/INPOP/EPM) als `ephemeris_house_gate` gebaut und lokal gemessen; das Doppler-Zeugen-Delta
der Ausführung geprüft und als `## An mycelium`-Zeile weitergereicht.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)
„verschleppen und nicht eigenes ist verboten" — Linienliste nur `eigen`, jeder Punkt im Atom bis zur Kante | 2026-09-30 | Operator (Session, Mountain 213)
„braucht es pro?" — Routine-Source-Port trägt flash; pro nur mit benanntem Hart-Atom/gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-09-30 | Operator (Session, Mountain 215)
„nimm das bitte auf — A = A" (die „Ganz einfach"-Ausführung; wortgetreu in `state/operator-gespraeche/2026-09-30-mountain.md`) | 2026-09-30 | Operator (Session, Mountain 215)

## Haus — Mountain (Stand 2026-09-30)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier
mit Zustand, auch um 3 Uhr nachts.

- **Die vier Orte:** `omegaflow` = `~/projects/omegaflow` + privates Schwester-Repo `state/`
  (`omegaflow/personal`); `omegaflow-legacy` = `archive-root/omegaflow-legacy`; `temp` =
  `/tmp/opencode`; `archive` = `archive-root`.
- **Mountain-Fundstellen:** `phi/`, `src/archivar`, `src/mathematikerin`, `src/gate`,
  `tools/harvest`, `tools/measure`, `tools/register`, `docs/specs`, `docs/surveys`,
  `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`; `state/` immer mit
  `archive_search --root state`.
- **Verwahrt (Operator-Wort Mountain-185):** `state/mountain-185-orphan-doc-nachzug.patch` —
  der Orphan-Doc-Nachzug-Diff bleibt nach Operator-Entscheid „C" unangewendet liegen („kein
  Träger, keine Freigabe, kein Nachzug-Commit"); nicht anwenden, nicht löschen.

## Offen (aufgeschlüsselt)

### Ephemeriden generic (new_horizons/voyager1/voyager2) — 976-B-Placeholder
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** —
- **Lage:** die drei `url`-Zeilen `phi/sources.φ:15747/15950/15957` sind auf `ssd.jpl.nasa.gov-horizons` gesetzt (Mycelium-214, `7e0cad67d`; `--verdict` 200, gemessen 2026-09-30); die `-ephemeris`-Zeilen der übrigen Körper bleiben gültig (86 Zeilen, z. B. nereid 206). Die generischen Assets bleiben aber 976-B-Placeholder; der tragende Kernel liegt in `_long` (78 928 / 255 888 / 211 088 B) und `_daily` (gemessen via GH-Releases-API).
- **Blockade:** der generische `ephemeris_compiler` liefert für NH/V1/V2 nur einen 976-B-Placeholder (das SPK fehlt im Standardsatz); den Kernel trägt der `horizons_compiler --long`-Lauf. Der Placeholder ist **nicht gemessen** (`pending`, 0 honored) — das Verdikt ist Mountain-Feder.
- **Braucht:** Mycelium setzt für die drei die zugelassene Quelle auf den `_long`-Lauf (`url`/`compiler`) oder verwirft die generische Zeile zugunsten von `ephemeris_*_long`; danach die generischen Placeholder-Zeilen `descoped` mit diesem Befund.

### Drei-Häuser-Ephemeriden — EPM-Riß am JUICE-Perigäum
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** `tools/measure/src/bin/ephemeris_house_gate.rs` gebaut (Erde-Baryzentrum über DE/INPOP/EPM, Perigäum ±2 d, 1 h-Schritt; `cargo build -p omegaflow-measure --bin ephemeris_house_gate` 0 Warnungen) + Workflow `ephemeris-house-gate.yml` (gemessen 2026-09-30 via `cargo build -p omegaflow-measure --bin ephemeris_house_gate`). Δ DE441↔INPOP19a (Erde) **0.0277 km**, Δ DE441↔EPM2021 **15.9403 km**, Δ INPOP↔EPM **15.9552 km** — die zwei westlichen Häuser stimmen auf ~28 m, EPM weicht ~16 km ab. Bins lokal: `data/ssd.jpl.nasa.gov-de/ephemeris_de441_earth.bin` sha `45651bee…` (183 151 272 B), `data/ftp.imcce.fr/ephemeris_inpop_earth.bin` sha `1dfa6bad…` (361 368 B), `data/ftp.iaaras.ru/ephemeris_epm_earth.bin` sha `33b9b108…` (2 574 216 B).
- **Blockade:** die Ursache des ~16-km-EPM-Offsets ist ungemessen — physikalische Ephemeriden-Differenz oder Herkunfts-/Frame-Differenz des `epm_compiler` (baryzentrisch vs. anders) oder ein Bewertungsfehler der EPM-Granule. Der Offset ist als Messung getragen, nicht geglättet.
- **Braucht:** den CI-Lauf dispatchen (`gh workflow run ephemeris-house-gate.yml` nach dem Push; Ergebnis im Artefakt) und die Ursache messen — EPM-Earth gegen eine zweite EPM-Version oder die SPK-Kernel-ID/den Center des `epm_compiler`-BSP gegen DE441 prüfen (`sgrep` in `tools/harvest/src/bin/epm_compiler.rs`, SPK-Header lesen).

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ /
  Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Schritt 2
  gemessen (Holdings + Repo-`data`), die Dedup-Entscheidung in `## An future`.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die
  Pending-Einträge sind im `dead_sources.φ`-Register disponiert, die Prosa bleibt als
  datierte Messung.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende
  Messreihe; jede Zelle mit `file:line`/`pending`.

## Register-Träger (eigene)

- `phi/harvest.φ` `format hapi_csv` (DAS2 Iowa, Asset `das2_iowa_…`) — `asset fehlt`;
  Workflow `das2-iowa-cdn.yml` gebaut (Mycelium-214), dispatcht `36741690825` — `present`
  erst nach grünem Lauf.
- `phi/harvest.φ` `format pds3_img` (Chandrayaan-1 Mini-RF, Asset `pds3_img_…`) — `asset fehlt`;
  Workflow `pds3-img-cdn.yml` rot (M3-`.HDR` HTTP 403, `pds-imaging.jpl.nasa.gov`) — Route
  messen.
- `phi/harvest.φ` `format pds3_fixed_width_darts` (Akatsuki VCO-rs) — `asset fehlt`; Workflow
  `pds3-fixed-width-darts-cdn.yml` gebaut (Mycelium-214), dispatcht `36741697524`.
- `phi/harvest.φ` `format pds4_binary` (ExoMars TGO ACS / PSA) — `asset fehlt`; Workflow läuft.

## An mycelium

Origin: mountain folge215 (Antwort auf die Doppler-Zeugen-Ausführung; die Blöcke folge214 sind gefaltet).

- **Doppler-Zeugen-Delta — gemessen 2026-09-30 (via `sgrep` in `phi/`):** Die Ausführung behauptet, ESA-DSN-Doppler (JUICE, Rosetta, Mars Express) sei „vorher nicht systematisch gesucht". Gemessen **falsch** für Rosetta/Mars Express: `rosetta_odf` trägt drei Assets (`phi/sources.φ:8948/8958/8968`, Tag `archives.esac.esa.int`), `mars_express_odf` trägt (`:8940`), `ephemeris_juice` trägt (`:15592`). Neu als Quelle offen sind die ODF/Doppler-Linien der übrigen Anderson-Sonden (NEAR trägt weder ODF noch SPK, laut folge197) und die Tracking-Formate von ISRO/CNSA/JAXA; `Tianwen-1` steht `blocked_sources.φ:389/425/440` als released (Arm steht, Sample fehlt).
- **„geerntet" ist gemessen überzeichnet:** Venera 15/16-Radiometrie ist registriert (`phi/sources.φ:16020-16037`), Chang'e-1/-2 MRM-Assets `present` (`blocked_sources.φ:420-421`), `hinet_win32_compiler` existiert (`sources.φ:9350`) — aber Chang'e/CNSA bleiben `released` mit Download-Duty und Tianwen-1 RoPeR/MoRIC haben noch kein Sample. Kein Verdikt ohne Messung — die Register-Zeilen sind der Stand.

## An river

Origin: mountain folge214 (Antwort auf river-folge73).

- **σ-Asset `dr3_stars.bin`:** liegt im Baum — `tap_compiler.rs:403` `STAR_BIN_STRIDE=56`
  + drei `sigma_slot` (`sig_plx`/`sig_pmra`/`sig_pmdec`) (gemessen 2026-09-30 via `sgrep`).
  Der `--star-bin`-Erweiterungsschritt ist erledigt; `gaia-cdn` re-manifestieren + `--sniff`
  + σ-Zensus sind deine Seite.
- **`usgs_comcat_m45.bin`:** committet (`d0c070737`); Reader `src/archivar/usgs_comcat.rs`,
  `field`/`ttl`/`frame` stehen (`sources.φ:11179`). Der Quake-Kanal kann verdrahtet werden.
- **`tao_wnd_zonal.csv`:** Riss benannt, nicht geglättet — keine Mountain-Aktion.

## An future (Operator-Queue, private)

Origin: mountain folge214.

- **Kuprat 5. Ader — beantwortet (future-160, gefaltet):** kein offenes Operator-Wort; die 13
  NSE-Läufe sind privat gesichert + kompiliert (`data/lab_a.data/SAMPLE_NSE_YBCO_6p35/`);
  NSE = `substance`-Zeuge ohne Wire-Arm, kein CDN, keine `sources.φ`; Redistribution LOCK.
- (aus folge212:) **ODF-Flyby-Fenster** (DSN/JPL-Anfrage?); **Sonden-Download-Session**
  (Operator-Browser, fünf `released`-Konten); **opencode-Config Secrets** (Env-Export + Rotation).

- **Holdings-Dedup (Frage, einfache Sprache):** *Lage* — die Byte-Messung (Schritt 2) ist
  gefahren: `archive/knowledge` 29 Gi, `archive-state` 9,7 Gi, Repo-`data` 77 Gi, `cache`
  10 Gi; in Gruppen liegen 4,34 GiB byte-identische Duplikate, reclaimable sind Repo-`data`-
  Zwillinge zwischen zwei Netloc-Ordnern (`wind_orbit.bin` = `omegaflow_series_wind_orbit.bin`,
  93,5 MB; `dr3_stars.bin` = `dr3_stars_stable.bin`, 75 MB) und ~2,2 GiB interner Scratch
  (`opencode-tmp-2026-09-01`, opencode `undo-snapshot`). *Frage* — sollen die gemessenen
  Duplikate gelöscht werden (je ein byte-identischer Zwilling bleibt als Nachbau)? *Bei Ja:*
  Mountain löscht die gemessenen Zwillinge und trägt die Unique-Bytes ins CDN-/`data/<netloc>/`-
  Ziel; *bei Nein:* die Kopien bleiben liegen, die Inventur vermerkt es.

- **Registry-first (Schritt 3, Antwort auf future-161) — je Asset gemessen:**
  - `who_flunet` — Verdikt steht: `decline health-stats` (`declined_sources.φ:5113`); keine
    `sources.φ`-Zeile, die lokale Datei (119,5 MB) gehört ins archive-root.
  - `nbp_Lmon` — Verdikt steht: `decline model-forecast` (`declined_sources.φ:49/:53`); keine
    `sources.φ`-Zeile.
  - `pioneer11_residuum` — ist ein Measure-Probe (`tools/measure/src/bin/pioneer11_odf_residuum.rs`),
    kein Asset → keine `sources.φ`-Zeile.
  - `pioneer11_odf` — Harvest-Compiler `pioneer11_odf_compiler.rs` ohne Register-Zeile;
    `pioneer10_odf` ist registriert (`sources.φ:10281`) → analog zulässig, Mycelium setzt
    `url`/`ttl` (`format pioneer11_odf`).
  - `pioneer10_telemetry` — Harvest-Compiler `pioneer_telemetry_compiler.rs` (Pioneer-Radio-
    telemetrie, em) ohne Register-Zeile → zulässig, Mycelium setzt `url`/`ttl`.
  - WMM/Kernel-Satz — SPICE-Kernel/Modell (Infrastruktur, kein Feld am Punkt) → keine
    `sources.φ`-Zeile, archive-root.

## Burn: open 0.0000 · close 0.0663 · Grund: „Ganz einfach"-Ausführung aufgenommen + `ephemeris_house_gate` gebaut/gemessen + Register/Übergabe (session_burn; Runde 6 Sessions 0.5292)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
