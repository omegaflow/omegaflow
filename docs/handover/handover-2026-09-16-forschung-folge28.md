<!--
  title: Handover — Forschung-Folge 28 (2026-09-16)
  session: Forschung-Folge 28
  class: handover
  date: 2026-09-16
  sha256: ab8ab0f850c3913309947f77a11defb9ef3893ed6e486fafe098ba6077be1446
  status: live
-->
# Handover — Forschung-Folge 28 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## TE / Statistik — HEAD rot, alle TE-CI-Läufe gated

- **HEAD (20c5f4d9) kompiliert die TE-Bins nicht.** `corona-conditional-probe`
  (run 35075736443) bricht mit E0061/E0432 ab: `te.rs:1083`
  `conditional_te_stats_lagged_n` trägt die Params-Struct-Signatur, der Aufruf in
  `corona_conditional_probe.rs:720` die alte Positionsform. Der Fix liegt im
  **fremden, uncommitteten 142-Datei-WIP** (Params-Struct-Refactor über
  `src/` + `tools/`, staged; Arbeitsbaum grün) — dieselbe Grenze, die
  Bau-Handover 41 benennt. Solange die WIP nicht committet/verworfen ist, ist
  kein TE-CI-Lauf sinnvoll und kein pfad-begrenzter src/-Commit möglich.
  (Schritt: Urheber-Linie/Operator entscheidet die WIP; danach
  `gh workflow run corona-conditional-probe.yml`; §4.6/Limitations in
  `docs/paper/corona-heating-ladder.md` füllen.)
- **broken-null-control — Probe-Bug gemessen und im Baum behoben (uncommittet).**
  `te-null-limits` (run 35076275812) kompilierte und lief, paniked aber
  `capacity overflow` in `bin_mean`: `cached_body` rief
  `cache_path_for(netloc, "<name>.json")` und las `<name>.json.json` — alle vier
  SWPC-Kanäle `absent`, `lo=+∞`/`hi=-∞` ergab `n_cells=usize::MAX`. Fix im
  Arbeitsbaum: Namen ohne `.json` + `live_grid = lo.is_finite() &&
  hi.is_finite() && lo < hi`; `cargo check -p omegaflow-measure --bin
  te_null_limits_probe` grün (0 Warnungen). Die Datei ist eine WIP-Datei — der
  Commit ist durch dieselbe Grenze gated.
  (Schritt: nach WIP-Klärung den Fix committen, dann
  `gh workflow run te-null-limits.yml`; Artefakt → `docs/paper/broken-null-control.md`.)
- **Solar 211A→193A** — conditional stage-2. (Schritt:
  `corona_conditional_probe`-Lauf, `docs/paper/solar-seconds-matrix.md`.)
- **Der Grat** — AIA-2014-per-Zelle-Schwellen. (Schritt: per-Zelle-Schwellen aus
  Lauf ins Blatt — `docs/blatt/blatt-der-grat.md`.)
- **GIC causal driver** — PCMCI auf dem Minuten-Sturm-Ensemble, KDE-h,
  Rückkanal-Härtung. (Schritt: `docs/paper/gic-causal-driver.md`.)
- **Blatt 2/3 — Rest offen.** fam/max-T-Bound (Blatt 2), retro OMNI2-PCMCI-Zeile
  und KDE-h/`laic_probe` 0–72-h brauchen einen lokalen laic-Harvest (CI-Skala).
  (Schritt: `docs/concepts/blatt-papier-resultat.md`, `multi_force_te_probe`.)

## Bande-Split / Sonden-ODF

- **Dawn, Voyager-Saturn, celestrak-eop — Assets manifestiert (gemessen).**
  `sbnarchive.psi.edu/dawn_odf.bin`, `spdf.gsfc.nasa.gov/voyager_saturn.bin`,
  `celestrak.org/celestrak_eop.bin` liegen auf dem `omegaflow/sources`-Release
  (2026-09-16). Drei Punkte geschlossen.
- **7 planetare ODF — Lauf offen.** `planetary-odf-cdn` (run 35076268649) läuft;
  `pds-geosciences.wustl.edu/magellan_odf.bin` liegt, die übrigen sechs
  Missionen (mgs/mro/odyssey/messenger/mars_express/rosetta) offen.
  (Schritt: Lauf-Ende prüfen, je Asset Byte-Existenz.)
- **160-Hz-Amplitudenzensus — der Lauf misst ihn nicht.** `pioneer-link-correction`
  (run 35076271979) lieferte die Link-Korrekturkette (Slipped-cycle-Maske,
  Ramp k=1.4318e-8, Dynamik a_P 4.0e-6, residual-RMS 3.191e3 Hz), aber keine
  Band-Amplitude bei 44–58 mHz. Der Census braucht eine eigene Probe auf dem
  korrigierten Residuum.
  (Schritt: Band-Amplituden-Probe benennen/bauen → §1
  `docs/paper/twenty-second-band-ground-chain.md`.)
- **NOCC-Reduktionsvorschrift** — Moyer 2000 + dsn_redr + 810-005-202E liegen;
  offen der Ketten-Vergleich im retrace gegen Moyer §10/§13.
  (Schritt: `docs/paper/twenty-second-band-ground-chain.md`; fremde
  retrace-Session.)

## Positionslinien / Ephemeriden

- **Die Weberin — Schritt 2 gemessen und eingetragen.** `station-convergence`
  (run 35076279503): Station ABK 53710.2 nT (INTERMAGNET-Boden) gegen 44908.5 nT
  (SWARM-Überflug, 1.98° Versatz, 01:28:43Z) — Abweichung 8801.7 nT über
  Toleranz 836.2 nT, ein Riss. In `docs/concepts/die-weberin.md` §8/§9
  eingetragen. Offen: Schritte 3–9.
- **Zweite unabhängige Linie je Klasse** (Planeten/Monde, Sonden-Doppler, TNO,
  Kometen, encke, juno-Namensschuld).
  (Schritt: `docs/surveys/survey-2026-09-07-weberin-sonnensystem-kette.md`.)
- **CDN-Planetenbins ~116 km SSB-Offset** — neu aus vollem `de441.bsp`.
  (Schritt: `docs/surveys/survey-geometric-ground-truth.md`.)

## Paper / Präregistrierung

- **depth-phase echo — Fleet-Voll-Lauf bereit.** Der Pilot (1 Event) liegt:
  2 agree / 1 oppose / 9 pending, weighted joint depth 234 km gegen Katalog
  231 km. `cmt-ndk-fleet.yml` um einen `max_events`-Input parametrisiert
  (uncommittet).
  (Schritt: nach Push `gh workflow run cmt-ndk-fleet.yml -f max_events=16`, dann
  `docs/paper/depth-phase-echo-fleet.md`.)
- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE 28./29.09.,
  Clipper 03.12. (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)
- **Kausalpfeil Trishuli** — Abfluss-Serie öffnen → `pfeil --lag-sweep`.
  (Schritt: `docs/paper/sturzflut-tibet-pfeil.md`.)
- **JWST disequilibrium** — O2/O3, vegetation red-edge, saisonale Kanäle.
  (Schritt: `docs/paper/jwst-disequilibrium-survey.md`.)

## Extern gebunden (kein Datum)

- NSE/Haug — Antwort von B. Keimer offen.
- Voyager Cruise / JPL-DSN — die Anfrage hält (request-only).
- Fünf Sonden-Anfragen (Voyager closed-loop, Mariner 10, Viking 1/2, Cassini
  closed-loop, Juno Earth-Flyby) — Operator reicht ein
  (`docs/auftrag/auftrag-sonden-rohdaten-anfragen.md`).
- Toth/Turyshev/Markwardt-Mails — die Prüfliste steht; senden in der
  Entscheid-Linie geführt.

## Benchmark

- Diese Session dispatchte keinen Sub-Agenten: der WIP-Blocker und die
  CI-Artefakt-Auswertung (gh ist im Hauptprofil, nicht in den Sub-Profilen)
  liefen im Hauptkontext. Offen: die CI-Artefakt-Extraktion als
  benchmark-Klasse messen, wenn `gh` in einem flash-Profil verfügbar ist.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
