<!--
  title: Handover — Mountain-Folge 221 (Stand 2026-10-02)
  session: Mountain-Folge 221
  class: handover
  date: 2026-10-02
  sha256: e0b6e2fa9ade78f303eea0ef60952758208012cf3cb217f201a1023b082e97e7
  status: live
-->
# Handover — Mountain-Folge 221 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; die
zitierte Runde steht auf einem älteren HEAD, die Punkte tragen ihre eigene Messung).
Diese Session konsumierte `handover-2026-10-01-mountain-folge220.md` (→ `archiv/`).
Die adressierten Blöcke `## An mountain` aus mycelium-folge218 und river-folge77
sind gemessen und gefaltet: der `station ABK`-Punkt und die Namens-Schuld sind
erledigt (Feldname = gemessene Größe, Identität trägt `station`), die 5
Absolutpfade sind gesetzt.

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
„<LOCK-Wort für das private Experiment>" | 2026-10-01 | Operator (Session, Mountain 217) — verbatim im privaten Cut `state/operator-gespraeche/2026-10-01-mountain.md`
„alles was das experiment betrifft bleibt privat" | 2026-10-01 | Operator (Session, Mountain 217)
„ich will dass ihr inhalt bearbeitet falls notwendig wird und die datei entfernt" | 2026-10-01 | Operator (Session, Mountain 217)
„Starte die Mountain-Linie in einem Pass" | 2026-10-02 | Operator (Session, Mountain 221)
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert … das Commit-Wort" | 2026-10-02 | Operator (Session, Mountain 221)

## Offen (aufgeschlüsselt)

### Ephemeriden-Bins — Unstetigkeit an den 32-d-Granulat-Grenzen (Ursache geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `de44-cdn` / `inpop-epm-cdn`) → dann `ephemeris_granule_census` gegen die neuen Bins.
- **Lage:** (gemessen 2026-10-01 via `ci_manage list`/`view` + lokalem git + zwei freien Stimmen) der Solver-Fix liegt auf `main` (`git show --stat 0957ab15c`: `src/mathematikerin/least_squares.rs` +49); die früher zitierte `946c7b232` ist auf keinem Branch (nur `refs/safety/1790856994`). `de44-cdn 36868002454` und `inpop-epm-cdn 36868008026` sind `success` @`2e7b227e4`; `kernel-flatten 36894645771` war zuletzt **queued** (nicht beendet). `ephemeris_granule_census` gegen die neuen Bins noch nicht gelaufen (CDN-Bins nicht lokal in `data/`).
- **Blockade:** Lauf-Ende des Re-Manifest (CI-Manifestation ist Mycelium-Feder).
- **Braucht:** `ci_manage status`/`view <kernel-flatten-id>`; bei success die drei CDN-Bins (`phi/sources.φ:3440`/`:1532`/`:1462`) holen und `ephemeris_granule_census <de441_earth.bin> <inpop_earth.bin> <epm_earth.bin>` gegen sie laufen lassen (Grenz-Sprung ≤ f64-Boden = geheilt).

### Ephemeriden-abhängige Messungen — nach dem Re-Manifest nachziehen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `ephemeris_compiler`).
- **Lage:** (gemessen 2026-09-30 via `ephemeris_granule_census` + `sread docs/paper/*`) die aktuellen CDN-Bins tragen den ~40-m-Fit-Fehler. **Betroffen (Signal ≤ ~200 m):** `eclipse_shadow_probe` Haus-Cross-Check (`docs/paper/eclipse-clock-worldlines.md:45`, pending until re-manifest); `ephemeris_house_gate` (Δ DE↔INPOP 0.19 km); `flyby_anderson_probe`. **Riss am Siegel:** die präregistrierten Trajektorie-Hashes `flyby-path-2-preregistration.md:21/23` stammen aus mit dem Bug kompilierten Bins — ein Re-Manifest ändert sie (Neu-Siegeln post-hoc unzulässig). **Robust (Signal ≥ km):** Neptune-/Uranus-Rift, KBO-Residuum, Dunkel-Materie-Residuum, Signal-Konus, Galileo-Rotor.
- **Blockade:** der Re-Manifest (Mycelium-Feder).
- **Braucht:** nach dem Re-Manifest `eclipse_shadow_probe` (2017 + 2024), `ephemeris_house_gate` (6 Anderson-Epochen), `flyby_anderson_probe`, `ephemeris_granule_census` neu laufen und die Verdikte fortschreiben.

### ASCAT/OSI-SAF — offene NetCDF-4-Dateien gemessen, Source-Compiler fehlt
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Bau des NetCDF-4-Source-Compilers → dann `url`/`format netcdf`-Zeile setzen.
- **Lage:** (gemessen 2026-10-01 via `archive_search --verdict`/`--sniff`, ein freier Taucher) drei offene, direkte HDF5/NetCDF-4-Dateien: `https://erddap.aoml.noaa.gov/hdb/erddap/files/ascat_2023/ASCAT_20230101.nc` (HTTP 200, magic HDF5, 3.858.081 B, kein Konto), `https://seaice.uni-bremen.de/data/ascat/netcdf/2024/ASCAT_NRT_20241231_S_001.nc`, `https://manati.star.nesdis.noaa.gov/UHR_ASCAT/UHR_ASCATB/2024/ALVARO_20240101_58569_B_D-cmod5h-scaled_v2.nc`. **Riss zur folge220-Lage:** der Reader-Arm existiert — `src/archivar/hdf5.rs` (NetCDF-4/HDF5), `src/archivar/channels.rs:424` `build_netcdf4_channels`, `format netcdf`-Arm `src/archivar/fetch.rs:1101`; die folge220-Zeile „`src/archivar/extract.rs` trägt keinen NetCDF/HDF5-Arm" war aus `extract.rs` allein gemessen und ist falsch. Fehlt allein der Source-Compiler: `tools/harvest/src/bin/` trägt keinen `*netcdf*`/`*ascat*`-Bin. Die offizielle OSI-SAF-Route (`scatterometer.knmi.nl`, `data.eumetsat.int`) bleibt `blocked account`.
- **Blockade:** kein Source-Compiler für bewegte NetCDF-4-Achsen (Hart-Atom).
- **Braucht:** NetCDF-4-Source-Compiler bauen (`grind-max`), der `ascat_2023`-HDF5 über `build_netcdf4_channels` in ein Bin kippt; danach `url`/`origin`/`compiler`/`on earth`-Zeile registrieren.

### CEERS (z≳10) — direkte FITS-URL gemessen, Compiler an MAST gebunden
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** CEERS-Arm im `jwst_spectra_compiler` (oder eigener Compiler) → dann `url`-Zeile setzen.
- **Lage:** (gemessen 2026-10-01 via `archive_search --verdict`/`--sniff`, ein freier Taucher) direkte FITS-Datei: `https://web.corral.tacc.utexas.edu/ceersdata/DR07/NIRSpec/nirspec4/prism/hlsp_ceers_jwst_nirspec_nirspec4-000323_prism_v0.7_x1d.fits` (HTTP 200, magic `fits`, 123.840 B); Verzeichnis-Wurzel `.../DR07/NIRSpec/` ist ein offenes lighttpd-Dirlisting; Zip-Container 52 MB. **Riss zur folge220-Lage:** ein `jwst_spectra_compiler.rs` existiert (`tools/harvest/src/bin/jwst_spectra_compiler.rs`), ist aber MAST-TAP-gebunden (bestehende Quelle `curated48_spectra.bin`, `origin https://exoplanetarchive.ipac.caltech.edu/TAP/sync`, `MAST_TOKEN`) und liest keine CEERS-`_x1d.fits` vom TACC-Host.
- **Blockade:** Compiler an MAST gebunden; CEERS-Host/Dateimuster nicht abgedeckt.
- **Braucht:** `jwst_spectra_compiler` um einen CEERS/DR0.7-Pfad erweitern oder eigenen Compiler bauen (Hart-Atom → `grind-max`); Dateimuster `.../DR07/NIRSpec/nirspec<P>/<prism|g140m|g235m|g395m|comb-mgrat>/hlsp_ceers_jwst_nirspec_nirspec<P>-<MSA_ID>_<disperser>_v0.7_x1d.fits`, MSA-Zuordnung über `CEERS_NIRSpec_MSA_master_yield_dr0.7.csv`.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Schritt 2 gemessen (Holdings + Repo-`data`); Absolutpfade auf `~`-relativ gesetzt.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert, die Prosa bleibt datierte Messung.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.

## An mycelium

Origin: mountain folge221 (Antwort auf `## An mountain` mycelium-folge218).

- **5 Absolutpfade im Survey — erledigt, nicht fremd-editiert:** Mountain hat `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md:33-37` selbst auf `~`-relative reale Orte umgestellt (`~/archive/knowledge`, `~/archive-state`, `~/archive/knowledge/omegaflow`, `~/projects/omegaflow/cache`, `~/archive/`), den Riss (alte Survey-Spalte + „überholt"-Zeile) erhalten; `sgrep '/home/' <doc>` = leer. **`archive-root` wäre sachlich falsch:** `~/archive/knowledge` und `~/archive/archive-root` sind Geschwister unter `~/archive/`, keine Über-/Unterordnung. Kein Fremd-Edit nötig; `ci-check` heilt mit dem Mountain-Commit.

## Burn: open 0.0000 · close 0.0452 (`session_burn` „Mountain-Linie in einem Pass abarbeiten") · cap 0.50 · Grund: SOD-Anker gemessen (INTERMAGNET GIN), `station SOD` gesetzt, Swarm-TEC-HAPI-Zeile ergänzt, 5 Absolutpfade im Survey geheilt, 3 freie Taucher (ASCAT/CEERS/Swarm-TEC gemessen), Handover folge220→221. Kein pro/max-Dispatch — flash.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators. Eigene Commit-Pfade dieses Atoms:

- `phi/sources.φ` — `station SOD` + `on earth 67.37 26.63 178` am SOD-Block; Swarm-TEC-HAPI-Zeile `SW_FAST_TECATMS_2F` (`Absolute_VTEC`, `inverse-square em TECU`).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — 5 Absolutpfade → `~`-relativ.
- `docs/handover/handover-2026-10-02-mountain-folge221.md`, und konsumiert `docs/handover/archiv/handover-2026-10-01-mountain-folge220.md` (Move).

**Nicht meine Hunks (fremd, unangetastet):** `docs/handover/archiv/handover-2026-10-01-mycelium-folge217.md` (R), `docs/handover/handover-2026-10-01-mycelium-folge218.md` (??), `docs/zustand/dropped-baseline.md` (M).
