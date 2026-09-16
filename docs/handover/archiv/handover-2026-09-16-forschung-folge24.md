<!--
  title: Handover — Forschung-Folge 24 (2026-09-16)
  session: Forschung-Folge 24
  class: handover
  date: 2026-09-16
  sha256: a5823bd078bd4cdc6af17fa58badaacc504cad9dfc0bd4042b48f6068eead12b
  status: live
-->
# Handover — Forschung-Folge 24 (2026-09-16)

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

- **Korona-Leiter — voller N-Konfunder-Lauf fehlt.** §4.6 berichtet die
  1–2-Konfunder-Konditionierung (GOES/335/94) mit Zahlen; der Bullet
  `corona-heating-ladder.md:485-490` nennt nur den vollen Multi-Force-Fall
  (jede Rung auf alle anderen konditioniert) — der ist ungemessen, ebenso ein
  zweites 90-Tage-Fenster (`:479-481`). `multi_force_te_probe` ist ein
  synthetischer 9-Kraft-Benchmark, kein Korona-Konsument.
  (Schritt: `corona_conditional_probe` auf N Konfunder erweitern
  (`transfer_entropy_conditional_binned_n`/`conditional_te_stats_lagged_n`) und
  in CI über `data/jsoc.stanford.edu/aia2014_fullyear.bin` +
  `data/ncei.noaa.gov/goes15/` laufen lassen — `docs/paper/corona-heating-ladder.md`.)
- **Solar 211A→193A** — conditional stage-2.
  (Schritt: `corona_conditional_probe`-Lauf, `docs/paper/solar-seconds-matrix.md`.)
- **Der Grat** — AIA-2014-per-Zelle-Schwellen.
  (Schritt: per-Zelle-Schwellen aus Lauf ins Blatt — `docs/blatt/blatt-der-grat.md`.)
- **depth-phase echo — CMT-Strahlungsterm.** Die Kalibrier-Azimute sind
  registriert (sechs Pilot-Stationen, gemessen 2026-09-16,
  `depth_phase_azimuth_probe`); offen bleibt der volle CMT-Radiationsterm.
  (Schritt: CMT/NDK-Fetch, per-Station-Vorzeichen gegen die Azimute —
  `docs/paper/depth-phase-echo-fleet.md`, `cmt-ndk-fleet.yml`.)
- **GIC causal driver** — PCMCI auf dem Minuten-Sturm-Ensemble, KDE-h, Rückkanal-Härtung.
  (Schritt: `docs/paper/gic-causal-driver.md`.)
- **Blatt 2/3 — Rest offen.** fam/max-T-Bound (Blatt 2), retro OMNI2-PCMCI-Zeile und
  KDE-h/`laic_probe` 0–72-h brauchen einen lokalen laic-Harvest (CI-Skala).
  (Schritt: `docs/concepts/blatt-papier-resultat.md`, `multi_force_te_probe`.)
- **broken-null-control — Rest offen.** Window-Drift ist geschlossen; offen bleibt
  das volle 60-s-Gitter (CI).
  (Schritt: `te_null_limits_probe` 60-s-Lauf in CI — `docs/paper/broken-null-control.md`.)

## Bande-Split / Sonden-ODF

- **1988-Wertdivergenz — die Ablations-Stufe gemessen.** Die A1-Chain-Ablation
  (`pioneer10_paper_chain_retrace.rs`, committet) fährt die rx14-1988-Zelle
  (Station 14, strict-1.0-s, 44–58) Stufe für Stufe: resid0/resid_c/resid_d
  halten 45,75 mHz, **resid_e springt auf 57,11 mHz (+11,36 mHz)** — der Sprung
  entsteht in Deduction 7 (daily-curve Segment-Slope), nicht in Media/Plasma/Ramp.
  Offen: ob die Segment-Slope-Subtraktion den Paper-Wert verfälscht oder die
  Kette korrekt ist.
  (Schritt: Deduction 7 gegen die Paper-Methode prüfen —
  `docs/paper/ground-sources-20s-band.md`.)
- **160-Hz-Amplitudenzensus** — pending.
  (Schritt: `cargo run -p omegaflow-measure --bin pioneer_link_correction_probe` in CI dispatchen.)
- **NOCC-Reduktionsvorschrift — Dokumente geholt.** Moyer 2000 + dsn_redr-Familie +
  810-005-202E in `docs/reference/`. Offen: der Ketten-Vergleich.
  (Schritt: Reduktionskette im retrace gegen Moyer §10/§13 prüfen —
  `docs/paper/twenty-second-band-ground-chain.md`; berührt die fremde retrace-Session.)
- **Venus Express VeRa — Reader gebaut + Voll-Harvest gemessen.** `vex_odf_compiler.rs`
  las alle vier Volumes (`VXRS_1101..1104`): 140 ODF, 1 052 174 Samples, data_type
  11/12, Stationen 34/43/45, 75 756 536 B, Roundtrip parst
  (`data/atmos.nmsu.edu/vex_odf.bin`). Offen: Register + CDN.
  (Schritt: `phi/sources.φ`-Block + `vex-cdn.yml` + `gh workflow run` nach Push + Consent.)
- **Dawn — Compiler gebaut + registriert.** Offen nur der CDN-Dispatch.
  (Schritt: `gh workflow run` nach Push + Consent.)
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
- **Die Weberin — Schritt 1 gebaut.** `BodyLine::Mpc` + `weberin_mpc_spk_verdict`
  weben MPCORB gegen die SPK-Punkte (am MPC-Epoch: ceres 58,5 km, vesta 24,0 km
  placed; die distant objects rissen auch am Epoch; 99 Tage später alle riss).
  Offen: Schritte 2–9.
  (Schritt: Schritt 2 (Stations-Konvergenz) — `docs/concepts/die-weberin.md`.)

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
