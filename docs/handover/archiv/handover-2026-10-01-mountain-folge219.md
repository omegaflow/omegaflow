<!--
  title: Handover — Mountain-Folge 219 (Stand 2026-10-01)
  session: Mountain-Folge 219
  class: handover
  date: 2026-10-01
  sha256: 28acbb403b80178c914bb4f4d283f7e98ac2ffa31078faffe426736a1f58939b
  status: live
-->
# Handover — Mountain-Folge 219 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-01-mountain-folge218.md` (→ `archiv/`).
Die beiden zuletzt offenen Manifestationen sind gemessen und im Register
geschlossen: `phi/harvest.φ` `format pds3_img` → `asset present`
(WUSTL/Mini-RF `pds3_img_fsb_00720_1cd_xhu_84n209_v1.bin`, 29896248 B, sha256
`6aa0eb1f…`); `format quake_ptevent` → `asset present` (3 Assets, sha256
`ff7e2f67…`/`28e08148…`/`cde2a662…`). Der M3-ENVI-Cube-Rest
(`pds-imaging.jpl.nasa.gov`) ist ein gemessener CI-Datacenter-IP-Block und als
`blocked ip-blocked` in `phi/blocked_sources.φ` registriert. `phi/sources.φ` (12
url-order) und `phi/declined_sources.φ` (2) sind via `register_sort --write`
kanonisch.

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
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-09-30 | Operator (Session, Mountain 216)
„<LOCK-Wort für das private Experiment>" | 2026-10-01 | Operator (Session, Mountain 217) — verbatim im privaten Cut `state/operator-gespraeche/2026-10-01-mountain.md`; LOCK privat, kein CDN/`sources.φ`/`witnesses.φ`, kein getrackter Baum; von future-164 gefaltet (LOCK-Abschnitt)
„alles was das experiment betrifft bleibt privat" | 2026-10-01 | Operator (Session, Mountain 217) — stehend: das ganze private Experiment (Daten, Ableitungen, experiment-spezifischer Code) bleibt privat; private Heimat `state/mountain/kuprat-complex-te/`; von future-164 gefaltet (LOCK-Abschnitt)
„ich will dass ihr inhalt bearbeitet falls notwendig wird und die datei entfernt" | 2026-10-01 | Operator (Session, Mountain 217) — Verwahrung Mountain-185 („C") aufgehoben: der verwahrte Orphan-Doc-Nachzug-Patch (Mountain-185) ist entfernt; `git apply --check` scheitert (Docs weitergelaufen → stale), Ziel-Docs nicht mehr in `--orphan-docs` → Inhalt überholt, kein Nachzug nötig

## Offen (aufgeschlüsselt)

### Ephemeriden-Bins — Unstetigkeit an den 32-d-Granulat-Grenzen (Ursache geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `de44-cdn` / `inpop-epm-cdn`) → dann `ephemeris_granule_census` gegen die neuen Bins.
- **Lage:** (gemessen 2026-10-01) der Solver-Fix `946c7b232` liegt auf `main`; die CDN-Bins tragen den ~40-m-Fit-Fehler noch. Mycelium hat dispatcht (`de44-cdn`/`inpop-epm-cdn` @`2e7b227e4`, `kernel-flatten 36894645771`) — `ci_manage status` zeigt `36894645771` pending.
- **Blockade:** Lauf-Ende (CI-Manifestation ist Mycelium-Feder).
- **Braucht:** `ci_manage status`/`log` der Läufe; danach `ephemeris_granule_census` gegen die neuen Bins.

### Ephemeriden-abhängige Messungen — nach dem Re-Manifest nachziehen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `ephemeris_compiler`).
- **Lage:** (gemessen 2026-09-30 via `ephemeris_granule_census` + `sread docs/paper/*`) die aktuellen CDN-Bins tragen den ~40-m-Fit-Fehler. **Betroffen (Signal ≤ ~200 m):** `eclipse_shadow_probe` Haus-Cross-Check (`docs/paper/eclipse-clock-worldlines.md:45`, in River-77 auf „pending until re-manifest" gestellt); `ephemeris_house_gate` (Δ DE↔INPOP 0.19 km); `flyby_anderson_probe`. **Riss am Siegel:** die präregistrierten Trajektorie-Hashes `flyby-path-2-preregistration.md:21/23` sind Hashes von mit dem Bug kompilierten `horizons_compiler`-Bins — ein Re-Manifest ändert sie (Neu-Siegeln post-hoc unzulässig). **Robust (Signal ≥ km):** Neptune-/Uranus-Rift, KBO-Residuum, Dunkel-Materie-Residuum, Signal-Konus, Galileo-Rotor.
- **Blockade:** der Re-Manifest (Mycelium-Feder).
- **Braucht:** nach dem Re-Manifest `eclipse_shadow_probe` (2017 + 2024), `ephemeris_house_gate` (6 Anderson-Epochen), `flyby_anderson_probe`, `ephemeris_granule_census` neu laufen und die Verdikte fortschreiben.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Schritt 2 gemessen (Holdings + Repo-`data`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert, die Prosa bleibt datierte Messung.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.

## Register-Träger (eigene)

- `phi/blocked_sources.φ` `blocked ip-blocked` M3 (`https://pds-imaging.jpl.nasa.gov/data/m3/CH1M3_0003/DATA/`) — Owner mycelium (via `register_lookup --open`); Aufenthalt beim Owner, dieser Atom reicht ihn als `## An mycelium` weiter (bereits geflaggt als `ORPHAN_UNCOMMITTED [mycelium]`).
- `phi/blocked_sources.φ` gap-Token `pds3-img` auf die Messung gefaltet: Arm + Compiler-Bin stehen (`src/archivar/pds3_img.rs`, `tools/harvest/src/bin/pds3_img_compiler.rs`), WUSTL-Asset present 2026-10-01, M3 CI-403.

## An mycelium

Origin: mountain folge219 (M3-Route + Ephemeriden-Messung).

- **M3-ENVI-Cube-Route (`phi/blocked_sources.φ` `blocked ip-blocked`, `pds-imaging.jpl.nasa.gov`):** WUSTL/Mini-RF ist present (`pds3-img-cdn 36887145554` success), der M3-Schwesterzweig ist CI-Datacenter-IP-403 (`36737530030`, `ci_manage log`: `.HDR` HTTP 403), lokal 206, kein Wayback-Snapshot. Braucht eine andere CI-Route/Proxy oder einen gemessenen M3-Descope-Befund. Bitte in deine Übergabe falten (der Eintrag steht als `ORPHAN_UNCOMMITTED [mycelium]`).
- **Ephemeriden-Re-Manifest:** `kernel-flatten 36894645771` pending, `de44-cdn`/`inpop-epm-cdn` @`2e7b227e4` — nach Lauf-Ende misst Mountain `ephemeris_granule_census` gegen die neuen Bins (mein offener Punkt).

## Burn: open 0.0000 · close 0.0828 (`session_burn` „Mountain-Linie Übergabe in einem Pass abarbeiten…") · cap 0.50 · Grund: Register-Schluss (pds3_img/quake_ptevent present), M3-`blocked ip-blocked`, `register_sort --write` (sources.φ 12 / declined_sources.φ 2), Handover-Faltung. Kein pro/max-Dispatch — flash.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Baum ist mit
parallelen Linien-Sessions geteilt (River/Sensory Handover + `src/mathematikerin/*`,
`docs/*` fremd uncommittet) — **Commit als Letzter**. Eigene Commit-Pfade dieses
Atoms:

- `phi/harvest.φ` (pds3_img + quake_ptevent → `asset present`)
- `phi/blocked_sources.φ` (M3 `blocked ip-blocked`; gap-Token `pds3-img` gefaltet)
- `phi/sources.φ` (`register_sort --write`, 12 url-order)
- `phi/declined_sources.φ` (`register_sort --write`, 2 url-order)
- `docs/handover/handover-2026-10-01-mountain-folge219.md`, und `…-folge218.md` → `archiv/` (Move)

**Nicht meine Hunks (fremd, unangetastet):** `src/mathematikerin/omega.rs`,
`src/mathematikerin/s2.rs`, `src/mathematikerin/tests.rs`,
`tools/harvest/src/bin/tap_compiler.rs`, `tools/measure/src/bin/bz_retro_probe.rs`,
`tools/measure/src/bin/enso_blatt_probe.rs`, `docs/paper/eclipse-clock-worldlines.md`,
`docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`, `.github/workflows/enso-probe.yml`,
`docs/blatt/fruehwarnsystem-praeregistrierung.md`, `docs/blatt/sonne-erde-blatt.md`,
`docs/handover/handover-2026-10-01-river-folge77.md`, `docs/handover/handover-2026-10-01-sensory-folge217.md`.
