<!--
  title: Handover — Forschung-Folge 23 (2026-09-16)
  session: Forschung-Folge 23
  class: handover
  date: 2026-09-16
  sha256: 0b671b6cb710543785a655759cbb20c9a37687fc0ce87b38ee08a0e786706322
  status: live
-->
# Handover — Forschung-Folge 23 (2026-09-16)

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

- **Korona-Leiter — der multi-force/conditional TE-Lauf fehlt.** Der 9-Kraft-TE ist
  in der Probe-Familie gebaut, aber im Korona-Paper nur als Limit benannt
  (`corona-heating-ladder.md:485-489`), nicht als Ergebnis berichtet; ein zweites
  90-Tage-Fenster ist ungemessen (`:479-481`).
  (Schritt: `multi_force_te_probe` auf der Korona-Ladder laufen lassen, §4.6/§6 mit
  den gemessenen Zahlen schließen — `docs/paper/corona-heating-ladder.md`.)
- **Solar 211A→193A** — conditional stage-2.
  (Schritt: `corona_conditional_probe`-Lauf, `docs/paper/solar-seconds-matrix.md`.)
- **Der Grat** — AIA-2014-per-Zelle-Schwellen.
  (Schritt: per-Zelle-Schwellen aus Lauf ins Blatt — `docs/blatt/blatt-der-grat.md`.)
- **depth-phase echo** — CMT-Strahlungsterm + 6 Kalibrier-Azimute.
  (Schritt: CMT/NDK fetchen, Azimute registrieren — `docs/paper/depth-phase-echo-fleet.md`.)
- **GIC causal driver** — PCMCI auf dem Minuten-Sturm-Ensemble, KDE-h, Rückkanal-Härtung.
  (Schritt: `docs/paper/gic-causal-driver.md`.)
- **Blatt 2/3 — Rest offen.** fam/max-T-Bound (Blatt 2), retro OMNI2-PCMCI-Zeile und
  KDE-h/`laic_probe` 0–72-h brauchen einen lokalen laic-Harvest (CI-Skala).
  (Schritt: `docs/concepts/blatt-papier-resultat.md`, `multi_force_te_probe`.)
- **broken-null-control — Rest offen.** Window-Drift ist geschlossen (naive
  Fisher–Yates + phase-randomisiert auf identischer Datenbasis, `te_null_limits_probe`
  Abschnitt M0: phase-Thr > naive-Thr auf jedem Paar, die Hénon-Rückrichtung ist ein
  naiver FP, den die Phase-Null stillt); offen bleibt das volle 60-s-Gitter (CI).
  (Schritt: `te_null_limits_probe` 60-s-Lauf in CI — `docs/paper/broken-null-control.md`.)

## Bande-Split / Sonden-ODF

- **1988-Wertdivergenz — gemessen.** `topk`/`peak_of_cell` lesen dieselben Werte
  (46,58/44,12/50,92 mHz); die Divergenz ist die verarbeitete Reihe (`resid_e`) gegen
  das rohe Residuum. Offen: welcher Schritt der Subtraktionskette rx14 um +10,1 mHz
  verschiebt. Eine fremde Session bearbeitet `pioneer10_paper_chain_retrace.rs` live
  (A1-Chain-Ablation) — nicht anfassen.
  (Schritt: Ablation auswerten, sobald die fremde Session committet.)
- **160-Hz-Amplitudenzensus** — pending.
  (Schritt: `cargo run -p omegaflow-measure --bin pioneer_link_correction_probe` in CI dispatchen.)
- **NOCC-Reduktionsvorschrift — Dokumente geholt.** Moyer 2000 (`docs/reference/moyer.2000.pdf`)
  + dsn_redr-Familie (`dsn_redr.2021-07-31.pdf`, `redr_unpack.pdf`) + 810-005-202E liegen
  in `docs/reference/`. Offen: der Ketten-Vergleich.
  (Schritt: Reduktionskette im retrace gegen Moyer §10 (Media/Antenna) + §13 (Observables)
  prüfen — `docs/paper/twenty-second-band-ground-chain.md`.)
- **Dawn — Compiler gebaut + registriert.** `tools/harvest/src/bin/dawn_odf_compiler.rs`
  baut `data/sbnarchive.psi.edu/dawn_odf.bin` (674 `.dat`, 892 244 Doppler-Samples,
  data_type 11/12/13, 14 DSS-Stationen); Block in `phi/sources.φ` (`format dawn_odf`).
  Offen nur der CDN-Dispatch.
  (Schritt: `gh workflow run` nach Push + Consent.)
- **Venus Express VeRa — PSA trägt keine DSN/ODF-Route (gemessen).** Rohroute NASA/PDS
  `VEX-V-RSS-1-ENT-V1.0` (NMSU), Identifier `USA_NASA_SUE_VXRS_11XX`.
  (Schritt: exaktes Verzeichnis + Landing-Page fetchen, dann Reader.)
- **Voyager-Saturn CDN-Dispatch** — Reader steht, nur der `voyager_saturn.bin`-Dispatch.
  (Schritt: `gh workflow run` nach Push + Consent.)
- **CDN-Dispatch** der registrierten Quellen (celestrak-eop, voyager, die 7 ODF, Dawn).
  (Schritt: `gh workflow run` nach Push + Consent.)

## Positionslinien / Ephemeriden

- **Zweite unabhängige Linie je Klasse** (Planeten/Monde, Sonden-Doppler, TNO,
  Kometen, encke, juno-Namensschuld).
  (Schritt: `docs/surveys/survey-2026-09-07-weberin-sonnensystem-kette.md`.)
- **CDN-Planetenbins ~116 km SSB-Offset** — neu aus vollem `de441.bsp`.
  (Schritt: `docs/surveys/survey-geometric-ground-truth.md`.)
- **Die Weberin / Zeugnis: 9 Schritte.**
  (Schritt: Schritt 1 (MPC gegen SPK) bauen — `docs/concepts/die-weberin.md`.)

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

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
