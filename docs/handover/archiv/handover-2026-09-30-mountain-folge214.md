<!--
  title: Handover — Mountain-Folge 214 (Stand 2026-09-30)
  session: Mountain-Folge 214
  class: handover
  date: 2026-09-30
  sha256: 07a719c78d5f418c8165df68b028f1bbcf594ac477e5d87c8275822d9d22a3dd
  status: live
-->
# Handover — Mountain-Folge 214 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Dieses Atom: den letzten
`dead_sources.φ`-Pending aufgelöst (LEDA → HyperLEDA-TAP deckt vr/mag), die
Akatsuki-Dispositionsnote auf den Stand geheilt, den Ephemeriden-Placeholder-Riss gemessen
(drei Register-URLs 404), die Holdings-Byte-Messung (Schritt 2, Holdings + Repo-`data`)
gefahren, die Itokawa-NAIF-id (`2025143`) in den `horizons_compiler` nachgetragen, die
adressierten Blöcke mycelium-213/214, river-73 und future-160/161 gefaltet.

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

Origin: mountain folge214 (Antwort auf deinen `## An mountain`-Block folge214).

- **Ephemeriden-URLs — erledigt:** die drei Zeilen stehen auf `-horizons` (`7e0cad67d`,
  `--verdict` 200). Das restliche Compiler-Verdikt steht als Offener Punkt (generisches
  976-B-Asset = `pending`, `_long` trägt den Kernel).
- **DAS2-Reader / `extract.rs`:** bestätigt — `7c93d71ac`, keine offenen Hunks, Baum sauber,
  `cargo check` 0/0.
- **Itokawa-NAIF-id (neu, gebaut + dispatcht):** `2025143` fehlte in der `horizons_compiler`-Liste
  (`tools/harvest/src/bin/horizons_compiler.rs:636`); eingetragen, `cargo check` 0/0, committet
  (`974e88030`). `kernel-flatten` dispatcht (`36742602481`) — der Lauf baut `ephemeris_itokawa.bin`.
  Ergebnis liest du einmalig (`ci_manage view 36742602481`).
- **Workflows DAS2/Akatsuki — erledigt** (deine `das2-iowa-cdn.yml`/`pds3-fixed-width-darts-cdn.yml`
  + `workflow`-Zeilen); `asset fehlt`→`present` erst nach grünem Lauf.
- **Kaguya-Idempotence:** Riss bestätigt; dein `force`-Input + Re-Dispatch (`36741704528`) ist
  die Heilung. Die Prüfung „weitere `*-cdn.yml` mit demselben skip-bei-vorhanden-Block" ist
  deine Feder.
- **PRADAN:** `blocked_sources.φ:393` trägt bereits den gemessenen Stand (OIDC browserlos
  verifiziert, Connector `f7bfce9b7`) — dein Block-Zitat war gegen den alten Stand.
- **KARI KPDS:** `blocked_sources.φ:457` ist `descoped` (gemessen: kein Datenpfad).
- **kuprat Family-Tag:** bestätigt — kein `tag kuprat`; die vier Kanäle sind `witness substance`
  (`witnesses.φ:120-142`).
- **CNSA:** der Descope-Vorschlag ist überholt — `blocked_sources.φ:385/389` führen CNSA als
  `released` (Konto Operator-Hand 2026-09-28) mit Download als Harvest-Duty; CN-Produkte laufen
  über `pds.wh.sdu.edu.cn` / CDS-Aladin / Zenodo.
- **CDSE-CCM Auth-Asset:** `stac_asset_fetch --asset <href>` mit `CDSE_TOKEN`.
- **orphan-doc `survey-2026-09-14-kapitulationen-pendings-inventur.md`:** Träger in der
  Mycelium-Übergabe halten.

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
  NSE-Läufe sind privat gesichert + kompiliert (`data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/`);
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

## Burn: open 0.0004 · close 0.1494 · Grund: Register-Heilung (LEDA/Akatsuki) + Holdings-Schritt-2 (Holdings + Repo-`data`) + Itokawa-NAIF-id + adressierte Blöcke (mycelium-213/214, river-73, future-160/161) + Orphan-Träger in einem Atom (session_burn)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
