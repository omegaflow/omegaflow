<!--
  title: Handover — Forschung-Folge 27 (2026-09-16)
  session: Forschung-Folge 27
  class: handover
  date: 2026-09-16
  sha256: 2db3888511c59727d199526d81022926dced76025bb25b82a376e24ae3b36b87
  status: live
-->
# Handover — Forschung-Folge 27 (2026-09-16)

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

## TE / Statistik

- **Korona-Leiter — Lauf bricht am Compile ab (HEAD rot).** `corona-conditional-probe`
  (run 35075736443) scheitert: das bin `corona_conditional_probe` kompiliert an
  HEAD nicht (E0061/E0432 — `conditional_te_stats_lagged_n`-Signatur gegen den
  Aufruf). Der Fix liegt im **staged** Sweep einer anderen Linie
  (te.rs-Params-Refactor, uncommitted; der Arbeitsbaum kompiliert grün).
  (Schritt: nach dem Commit der anderen Linie `gh workflow run
  corona-conditional-probe.yml` erneut; dann §4.6/Limitations in
  `docs/paper/corona-heating-ladder.md` füllen — die Binning-Sparsität bei 5
  Konfundern entscheidet binned vs. ksg.)
- **Solar 211A→193A** — conditional stage-2.
  (Schritt: `corona_conditional_probe`-Lauf, `docs/paper/solar-seconds-matrix.md`.)
- **Der Grat** — AIA-2014-per-Zelle-Schwellen.
  (Schritt: per-Zelle-Schwellen aus Lauf ins Blatt — `docs/blatt/blatt-der-grat.md`.)
- **depth-phase echo — CMT-Strahlungsterm gemessen.** `cmt-ndk-fleet`
  (run 35075743255) hat den GCMT-NDK-Quellterm geladen (70039 Zentroide) und
  das per-Station-R_P-Vorzeichen gegen das gemessene pP geprüft: 2 agree /
  1 oppose / 9 pending (Pilot, `--max-events 1`); die Kalibrier-Azimute sind
  registriert; `docs/paper/depth-phase-echo-fleet.md` ist damit gefüllt.
  Offen: der volle Fleet-Lauf (>1 Event) für die fleet-weite Bilanz.
  (Schritt: `cmt-ndk-fleet` mit größerem `--max-events` dispatchen.)
- **GIC causal driver** — PCMCI auf dem Minuten-Sturm-Ensemble, KDE-h,
  Rückkanal-Härtung. (Schritt: `docs/paper/gic-causal-driver.md`.)
- **Blatt 2/3 — Rest offen.** fam/max-T-Bound (Blatt 2), retro OMNI2-PCMCI-Zeile
  und KDE-h/`laic_probe` 0–72-h brauchen einen lokalen laic-Harvest (CI-Skala).
  (Schritt: `docs/concepts/blatt-papier-resultat.md`, `multi_force_te_probe`.)
- **broken-null-control — 60-s-Gitter.** Workflow `te-null-limits.yml` gebaut
  (kein CDN-Release; die vier SWPC-JSON werden per curl in den archivar cache
  geholt).
  (Schritt: dispatcht run 35076275812 — Artefakt lesen, dann das Gitter in
  `docs/paper/broken-null-control.md` füllen.)

## Bande-Split / Sonden-ODF

- **Dawn — Workflow gebaut.** `.github/workflows/dawn-cdn.yml` (Compiler
  `dawn_odf_compiler --ci-mode` → `sbnarchive.psi.edu/dawn_odf.bin`).
  (Schritt: dispatcht run 35076261911 — Ergebnis prüfen.)
- **Voyager-Saturn — Workflow gebaut.** `.github/workflows/voyager-saturn-cdn.yml`
  (`voyager_saturn_compiler --ci-mode` → `spdf.gsfc.nasa.gov/voyager_saturn.bin`).
  (Schritt: dispatcht run 35076265454 — Ergebnis prüfen.)
- **7 planetare ODF — Workflow gebaut.** `.github/workflows/planetary-odf-cdn.yml`
  (matrix magellan/mgs/mro/odyssey/messenger/mars_express/rosetta, je `--ci-mode`).
  (Schritt: dispatcht run 35076268649 — Ergebnis prüfen.)
- **CDN-Dispatch celestrak-eop** — dispatcht (run 35075745915).
  (Schritt: Ergebnis prüfen.)
- **160-Hz-Amplitudenzensus.** Workflow `pioneer-link-correction.yml` gebaut
  (CDN: pioneer10_skyfreq, ephemeris_earth, ephemeris_pioneer10_daily,
  omni2_serie).
  (Schritt: dispatcht run 35076271979 — Ergebnis prüfen.)
- **NOCC-Reduktionsvorschrift — Dokumente geholt.** Moyer 2000 + dsn_redr-Familie +
  810-005-202E in `docs/reference/`. Offen: der Ketten-Vergleich.
  (Schritt: Reduktionskette im retrace gegen Moyer §10/§13 prüfen —
  `docs/paper/twenty-second-band-ground-chain.md`; berührt die fremde retrace-Session.)

## Positionslinien / Ephemeriden

- **Die Weberin — Schritt 1 gebaut.** `BodyLine::Mpc` + `weberin_mpc_spk_verdict`
  weben MPCORB gegen die SPK-Punkte. Schritt 2 (Stations-Konvergenz) hat einen
  Workflow `.github/workflows/station-convergence.yml`
  (`station_convergence_probe --live`); offen bleiben Schritte 3–9.
  (Schritt: dispatcht run 35076279503 — Ergebnis lesen, dann
  `docs/concepts/die-weberin.md`.)
- **Zweite unabhängige Linie je Klasse** (Planeten/Monde, Sonden-Doppler, TNO,
  Kometen, encke, juno-Namensschuld).
  (Schritt: `docs/surveys/survey-2026-09-07-weberin-sonnensystem-kette.md`.)
- **CDN-Planetenbins ~116 km SSB-Offset** — neu aus vollem `de441.bsp`.
  (Schritt: `docs/surveys/survey-geometric-ground-truth.md`.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE 28./29.09.,
  Clipper 03.12.
  (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)
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
- Toth/Turyshev/Markwardt-Mails — die Prüfliste steht; die Mails sind entblockt.
  (Schritt: senden — in der Entscheid-Linie geführt.)

## Benchmark

- Der Bau der sechs CI/CDN-Workflows lief an `grind-flash` (flash-first), das
  Bestands-Inventar an `explore` (flash). Kein pro/max-Atom in dieser Session —
  die Routine-Klasse ist geschlossen (`docs/concepts/tools-map.md`: flash
  2.4–11× billiger bei identischem Ergebnis, Sieger `grind-flash`).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
