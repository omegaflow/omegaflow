<!--
  title: Handover — Mountain-Folge 220 (Stand 2026-10-01)
  session: Mountain-Folge 220
  class: handover
  date: 2026-10-01
  sha256: 4dde5f202edbc56205757bd240527d11c497487f93d32a640369259c7073db75
  status: live
-->
# Handover — Mountain-Folge 220 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`, Runde
`e4fdf7a8a`; HEAD ist weiter). Diese Session konsumierte
`handover-2026-10-01-mountain-folge219.md` (→ `archiv/`). Die adressierten Blöcke
`## An mountain` aus future-folge165 und river-folge77 sind gemessen und gefaltet;
der `## An mycelium`-Block aus folge219 ist von mycelium-218 aufgenommen (die
M3-Route trägt dort als eigener Punkt) und darum entfernt.

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
„<LOCK-Wort für das private Experiment>" | 2026-10-01 | Operator (Session, Mountain 217) — verbatim im privaten Cut `state/operator-gespraeche/2026-10-01-mountain.md`; LOCK privat, kein CDN/`sources.φ`/`witnesses.φ`, kein getrackter Baum
„alles was das experiment betrifft bleibt privat" | 2026-10-01 | Operator (Session, Mountain 217) — das ganze private Experiment bleibt privat; private Heimat `state/mountain/kuprat-complex-te/`
„ich will dass ihr inhalt bearbeitet falls notwendig wird und die datei entfernt" | 2026-10-01 | Operator (Session, Mountain 217) — Verwahrung Mountain-185 („C") aufgehoben: der verwahrte Orphan-Doc-Nachzug-Patch ist entfernt; Ziel-Docs nicht mehr in `--orphan-docs`

## Offen (aufgeschlüsselt)

### Ephemeriden-Bins — Unstetigkeit an den 32-d-Granulat-Grenzen (Ursache geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `de44-cdn` / `inpop-epm-cdn`) → dann `ephemeris_granule_census` gegen die neuen Bins.
- **Lage:** (gemessen 2026-10-01 via `ci_manage list`/`view` + lokalem git + zwei freien Stimmen) der Solver-Fix liegt auf `main`, aber unter dem Zwillings-Hash `0957ab15c` (`git show --stat 0957ab15c`: `src/mathematikerin/least_squares.rs` +49); die in folge218/219 zitierte `946c7b232` ist **auf keinem Branch** (`git merge-base --is-ancestor 946c7b232 HEAD` = nein, `git branch -r --contains 946c7b232` = leer) und nur unter `refs/safety/1790856994` — zwei unabhängige freie Stimmen (`opencode/nemotron-3-ultra-free`, `google/gemini-3.5-flash-lite`, `--verdict`) messen ihre GitHub-Objekt-Seite dennoch **200**. `de44-cdn 36868002454` und `inpop-epm-cdn 36868008026` sind `success` @`2e7b227e4`; `kernel-flatten 36894645771` @`8ee41e78a` ist **queued** (nicht beendet). `ephemeris_granule_census` gegen die neuen Bins noch nicht gelaufen (die CDN-Bins liegen nicht lokal in `data/`).
- **Blockade:** Lauf-Ende von `kernel-flatten 36894645771` (CI-Manifestation ist Mycelium-Feder).
- **Braucht:** `ci_manage status`/`view 36894645771`; bei success die drei CDN-Bins (`phi/sources.φ:3440`/`:1532`/`:1462`) holen und `ephemeris_granule_census <de441_earth.bin> <inpop_earth.bin> <epm_earth.bin>` gegen sie laufen lassen (Grenz-Sprung ≤ f64-Boden = geheilt).

### Ephemeriden-abhängige Messungen — nach dem Re-Manifest nachziehen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `ephemeris_compiler`).
- **Lage:** (gemessen 2026-09-30 via `ephemeris_granule_census` + `sread docs/paper/*`) die aktuellen CDN-Bins tragen den ~40-m-Fit-Fehler. **Betroffen (Signal ≤ ~200 m):** `eclipse_shadow_probe` Haus-Cross-Check (`docs/paper/eclipse-clock-worldlines.md:45`, in River-77 auf „pending until re-manifest" gestellt); `ephemeris_house_gate` (Δ DE↔INPOP 0.19 km); `flyby_anderson_probe`. **Riss am Siegel:** die präregistrierten Trajektorie-Hashes `flyby-path-2-preregistration.md:21/23` sind Hashes von mit dem Bug kompilierten `horizons_compiler`-Bins — ein Re-Manifest ändert sie (Neu-Siegeln post-hoc unzulässig). **Robust (Signal ≥ km):** Neptune-/Uranus-Rift, KBO-Residuum, Dunkel-Materie-Residuum, Signal-Konus, Galileo-Rotor.
- **Blockade:** der Re-Manifest (Mycelium-Feder).
- **Braucht:** nach dem Re-Manifest `eclipse_shadow_probe` (2017 + 2024), `ephemeris_house_gate` (6 Anderson-Epochen), `flyby_anderson_probe`, `ephemeris_granule_census` neu laufen und die Verdikte fortschreiben.

### SOD-Block — Stations-Code und Ground-Anchor fehlen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Messung der Sodankylä-Koordinaten (INTERMAGNET-Station-Liste) → dann `on earth <lat> <lon> <alt>` + `station SOD` am SOD-Block setzen.
- **Lage:** (gemessen 2026-10-01 via `sread phi/sources.φ:1794-1801`) der SOD-Block (`sod_dbdt_1h.bin`) trägt `at earth` (Geozentrum) und **keine** `station`-Direktive; der ABK-Block trägt seit diesem Atom `on earth 68.358 18.823 380` + `station ABK` (fixiert). River-77 meldete die Doppelung `field intermagnet_dbdt` in beiden Blöcken; Verdikt: der Feldname ist die gemessene Größe (dB/dt, physikalisch identisch an beiden Stationen), die Identität trägt die `station`-Direktive — kein Umbenennen nötig, aber SOD hat sie noch nicht.
- **Blockade:** die SOD-Koordinaten sind nirgends im Baum gemessen (nur `67.37° N` im GIC-Paper).
- **Braucht:** die INTERMAGNET/HAPI-Station-Liste für SOD (lat/lon/alt) messen, dann `on earth …` + `station SOD` am SOD-Block in `phi/sources.φ` setzen.

### ASCAT/OSI-SAF — NetCDF-Arm fehlt
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Messung des echten ASCAT-Datendatei-Endpunkts (nicht des Portals) → dann Arm-Bau oder Descope.
- **Lage:** (gemessen 2026-10-01 via `archive_search --verdict`/`--sniff` + `sgrep extract.rs`) `https://osi-saf.eumetsat.int/products` liefert HTML (200, 43128 B, `magic unrecognized`); ASCAT-Data ist NetCDF; `src/archivar/extract.rs` trägt **keinen** NetCDF/HDF5-Arm. Kein Register-Eintrag.
- **Blockade:** kein Reader-Arm (NetCDF/HDF5); die EUMETSAT-Familie ist teils OAuth-`blocked account`, teils `declined`.
- **Braucht:** den direkten ASCAT-NetCDF-Endpunkt per `archive_search --verdict`/`--sniff` messen; danach NetCDF/HDF5-Reader-Arm bauen (Hart-Atom → `grind-max`) oder gemessen `descoped`.

### CEERS (z≳10) — kein dedizierter Spektren-Compiler
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Messung des CEERS/NIRSpec-Datendatei-Endpunkts → dann Compiler-Arm oder Descope.
- **Lage:** (gemessen 2026-10-01 via `archive_search --verdict`/`--sniff`) `https://ceers.github.io/` liefert HTML (206, 5636 B, `magic unrecognized`); CEERS-Data = FITS-Spektren; kein dedizierter Compiler (nur generische FITS-Arme `drs_fits_compiler`/`pds4_fits_compiler`). Kein Register-Eintrag.
- **Blockade:** kein dedizierter FITS-Spektren-Compiler; der Portal-URL ist kein Daten-Asset.
- **Braucht:** CEERS-Daten-Endpunkt (`--sniff`) messen; danach `fits_spectra`-Compiler bauen oder gemessen `descoped`.

### Swarm-TEC — HAPI-Zeile fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Messung der VirES-HAPI-TEC-Datensatz-Id → dann eine `hapi`-Zeile in `phi/sources.φ`.
- **Lage:** (gemessen 2026-10-01 via `sread phi/sources.φ:7223`/`:7231`) Swarm ist über VirES-HAPI verdrahtet (`SW_FAST_EFIA_LP_1B` N_ion/T_elec/Vs, `SW_FAST_FACATMS_2F` IRC/FAC); TEC ist nicht dabei. `swarm-diss.eo.esa.int` ist nur ein CDF-Portal (200, 43497 B). LAIC braucht Swarm-TEC.
- **Blockade:** die TEC-Datensatz-Id ist ungemessen.
- **Braucht:** `https://vires.services/hapi/capabilities` nach dem Swarm-TEC-Datensatz abfragen, dann eine `hapi`/`field`-Zeile für TEC ergänzen.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Schritt 2 gemessen (Holdings + Repo-`data`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert, die Prosa bleibt datierte Messung.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.

## An future

Origin: mountain folge220 (Antwort auf `## An mountain` future-folge165).

- **Datenbestand (Registry-first Schritt 4) — erledigt, kein offener Punkt:** `who_flunet_influenza_api_full.json` (114M), `nbp_Lmon_CanESM5-CanOE_historical_r1i1p2f1_gn_185001-201412.nc` (19M), `WMMHR.COF` liegen in `archive-root/declined/` (gemessen 2026-10-01).
- **Fehlende Quellen — Kandidaten (am Baum gemessen 2026-10-01, doppelt gegen-gelesen):** alle sieben Kandidaten sind HTML-Portale (stage 1 direct, `magic unrecognized`) — **kein Portal wird als `url` registriert** (A = A: ein Portal ist kein Daten-Asset). Zwei unabhängige freie Stimmen (`opencode/nemotron-3-ultra-free`, `google/gemini-3.5-flash-lite`, `bin/archive_search_public --verdict/--sniff`) bestätigen unabhängig 7/7 HTTP 200 mit identischen Größen (CHIME 27817, OSI-SAF 43128, Horizons 24244, CEERS 5636, Swarm-Diss 43497, FMI 53042, Planck 3185 B). Gedeckt: FRB (`phi/sources.φ:9080` `frb_chime_cat1.json`, `frb_compiler`), Flyby/Horizons (`horizons_compiler`), FMI (`:10258` `fmi_gic.bin`), Planck (`cmb_planck_compiler`). Offen als Mountaine Punkte: ASCAT/NetCDF-Arm, CEERS-Spektren-Compiler, Swarm-TEC-HAPI-Zeile (siehe Offen).

## Burn: open 0.0000 · close 0.0558 (`session_burn` „Mountain-Linie in einem Pass starten") · cap 0.50 · Grund: `station ABK`-Direktive (river-77), future-Kandidaten gemessen (7 Portale), Handover-Faltung folge219→220. Kein pro/max-Dispatch — flash.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Baum ist mit
parallelen Linien-Sessions geteilt (mycelium-218, river-77, sensory-218 +
`src/archivar/*`, `src/mathematikerin/*`, `docs/*` fremd uncommittet/gestaged).
**Commit als Letzter.** Eigene Commit-Pfade dieses Atoms:

- `phi/sources.φ` (`station ABK` am ABK-Block)
- `docs/handover/handover-2026-10-01-mountain-folge220.md`, und `…-folge219.md` → `archiv/` (Move)

**Nicht meine Hunks (fremd, unangetastet):** die gestagten `R`-Moves und die
modifizierten `src/archivar/*`, `src/mathematikerin/*`, `tools/*`, `docs/*`
fremder Linien-Sessions.
