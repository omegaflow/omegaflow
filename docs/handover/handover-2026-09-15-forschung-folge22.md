<!--
  title: Handover — Forschung-Folge 22 (2026-09-15)
  session: Forschung-Folge 22
  class: handover
  date: 2026-09-15
  sha256: 7a8039afd2ee50ccaaf284ebd215fc1bb6333c553ddf9b2625988beb2c11789b
  status: live
  see-also: docs/paper/broken-null-control.md
-->
# Handover — Forschung-Folge 22 (2026-09-15)

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

- **Korona-Leiter** — bandbreiten-robuster Pfeil + zweites Fenster; per-Lag-Schwellen;
  multi-force TE ist implementiert, nicht berichtet.
  (Schritt: `docs/paper/corona-heating-ladder.md`, `corona_ladder_probe`.)
- **Solar 211A→193A** — conditional probe (stage-2).
  (Schritt: `docs/paper/solar-seconds-matrix.md`.)
- **Der Grat** — AIA-2014-per-cell-Schwellen; ENSO-Pfeil.
  (Schritt: `docs/blatt/blatt-der-grat.md`.)
- **depth-phase echo** — CMT-Strahlungsterm, 6 Kalibrier-Azimute, Δ≈30°.
  (Schritt: `docs/paper/depth-phase-echo-fleet.md`.)
- **GIC causal driver** — PCMCI, Minuten-Sturm-Ensemble, Tages-Lauf, KDE-h, Rückkanal.
  (Schritt: `docs/paper/gic-causal-driver.md`.)
- **Blatt 2/3 — Rest offen.** Blatt 2/3 sind gemessen gefüllt; offen bleiben der
  fam/max-T-Bound (Blatt 2) und die retro OMNI2-PCMCI-Zeile (beide CI-Skala) sowie
  KDE-h und der `laic_probe` 0–72-h-Lauf, die einen lokal fehlenden laic-Harvest
  brauchen. Der multi-force-TE-Lauf (216 Zellen) trägt 1 FDR-Pass und findet die
  3 gepflanzten Pfeile bei n=512 nicht (benannt).
  (Schritt: `docs/concepts/blatt-papier-resultat.md`, `multi_force_te_probe`.)
- **broken-null-control — Rest offen.** Bandbreite h, Lag-Sweep und max-T-Korrektur
  sind gemessen geschlossen (h-Fragilität, τ-Optimum, fam tötet die per-Zelle-FPs
  ohne die wahren Kopplungen zu töten). Offen: Window-Drift (naive/phase auf
  identischer Datenbasis) und das volle 60-s-Gitter (CI); der n=300-FN-Bias ist benannt.
  (Schritt: `docs/paper/broken-null-control.md`, `te_null_limits_probe`.)

## Bande-Split / Sonden-ODF

- **1988-Wertdivergenz — gemessen.** Die Divergenz ist die verarbeitete Reihe
  (`resid_e`, Subtraktionskette + Sample-Maskierung) gegen das rohe Residuum, nicht
  die Methode: `topk` (Split) und `peak_of_cell` (Zensus) lesen auf derselben Datei
  dieselben Werte (46.58/44.12/50.92 mHz); §1's 57.11/44.40/51.99 stammen nur aus
  `resid_e`. Offen: welcher Schritt der Subtraktionskette rx14 um +10.1 mHz verschiebt.
  (Schritt: `pioneer10_paper_chain_retrace.rs`.)
- **160-Hz-Amplitudenzensus** — `pending`.
  (Schritt: `cargo run -p omegaflow-measure --bin pioneer_link_correction_probe` in CI dispatchen.)
- **NOCC-Reduktionsvorschrift — offen erhältlich (gemessen).** Moyer, *Formulation
  for Observed and Computed Values of DSN Data Types* (Descanso Monograph 2, JPL 00-7)
  und die `dsn_redr`-Familie (WUSTL PDS radiosci docs, `dsn_redr.2021-07-31.pdf`,
  `redr_unpack.pdf`); 810-005 trägt kein Reduktionsmodul (202E = Doppler-Link).
  (Schritt: die Dokumente in `docs/reference/` holen + die Reduktionskette im retrace
  gegen die Moyer-Formulierung prüfen.)
- **Dawn — Route bestätigt.** PDS4-SBN
  `https://sbnarchive.psi.edu/pds4/dawn/gravity/dawn-rss-raw-{ceres,vesta}/data-odf/`
  ist real (HTTP 200), jahres-partitioniert, `.dat`+`.xml`-Paare, offene Listings.
  (Schritt: ODF-Compiler nach `*_odf_compiler`-Muster + `parse_odf`-Probe an einer .dat.)
- **Venus Express VeRa — PSA trägt keine DSN/ODF-Route (gemessen).** `VEX-V-VRA-*`
  unter `DATA/LEVEL1A/CLOSED_LOOP/` nur `IFMS/`; keine `VEX-V-RSS-*`-Familie in der
  PSA (live/legacy/pds3_extra). Die Rohroute ist NASA/PDS `VEX-V-RSS-1-ENT-V1.0`
  (NMSU, `USA_NASA_SUE_VXRS_11XX`).
  (Schritt: exaktes Verzeichnis per Fetch bestätigen, dann Reader.)
- **Voyager-Saturn CDN-Dispatch** — der Reader steht (kind→Observable in
  `voyager_saturn::parse_series`, `VSAT`-Magic in `zeuge.rs`, `em rad` für die
  Winkel-Slots); offen nur der `voyager_saturn.bin`-CDN-Dispatch.
  (Schritt: `gh workflow run` nach Push + Consent.)
- **CDN-Dispatch** der registrierten Quellen (celestrak-eop, voyager, die 7 ODF,
  Dawn nach Compiler). (Schritt: `gh workflow run` nach Push + Consent.)

## Positionslinien / Ephemeriden

- **Zweite unabhängige Linie je Klasse** (Planeten/Monde, Sonden-Doppler, TNO,
  Kometen, encke, juno-Namensschuld).
  (Schritt: `docs/surveys/survey-2026-09-07-weberin-sonnensystem-kette.md`.)
- **body_fixed_to_icrs ~117°-Bug.** (Schritt: `docs/concepts/positive-maske.md`.)
- **CDN-Planetenbins ~116 km SSB-Offset** — neu aus de441.bsp.
  (Schritt: `docs/surveys/survey-geometric-ground-truth.md`.)
- **Die Weberin / Zeugnis: 9 Schritte.** (Schritt: `docs/concepts/die-weberin.md`.)
- **eclipse-clock** — exakte Subpunkt-Sweep-Rate; Gate-Test LINES[0]=de441.
  (Schritt: `docs/paper/eclipse-clock-worldlines.md`.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE
  28/29.09., Clipper 03.12.
  (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)
- **Kausalpfeil Trishuli** — Abfluss-Serie öffnen → `pfeil --lag-sweep`.
  (Schritt: `docs/paper/sturzflut-tibet-pfeil.md`.)
- **JWST disequilibrium** — O2/O3, vegetation red-edge, saisonale Kanäle.
  (Schritt: `docs/paper/jwst-disequilibrium-survey.md`.)

## Verschwunden (Repo-Historie, kein Commit) — wieder eingetragen

- **Nadel Ⅺ Placebo-Coregistration** — `.elc`-Reader steht, `rigid_coregister`
  fehlt. (Schritt: in `tools/measure/src/fiducial.rs` bauen.)
- **front-c-epsilon-2d** — `tools/measure/src/bin/pioneer_navio_epsilon_2d.rs`.
- **te-series-periodicity (K≈185 d)** — `tools/measure/src/bin/te_series_periodicity_probe.rs`.
- **klassen-benchmark-pcmci** — `tools/measure/src/bin/pcmci_class_benchmark.rs`.
- **iapetus Front B** — keine Datei im Baum. (Schritt: erste Messung — Konzept/Probe suchen.)
- **v1298-tau-b-ocs** — `tools/measure/src/bin/v1298_tau_b_sulfur_quench_probe.rs`
  (MAST 10.17909/kjg5-8t66).

## Extern gebunden (kein Datum)

- NSE/Haug — Antwort von B. Keimer offen.
- Voyager Cruise / JPL-DSN — die Anfrage hält (request-only).
- Fünf Sonden-Anfragen (Voyager closed-loop, Mariner 10, Viking 1/2, Cassini
  closed-loop, Juno Earth-Flyby) — Operator reicht ein
  (`docs/auftrag/auftrag-sonden-rohdaten-anfragen.md`).
- Toth/Turyshev/Markwardt-Mails — die Prüfliste steht; die Mails sind entblockt.
  (Schritt: senden — in der Entscheid-Linie geführt.)

## Benchmark — offene Recherche-Aufgaben (general/flash vs research-max/pro-max)

Dieselbe Frage, identischer Wortlaut, gleiche Lese-Grenze (≤20 Tool-Calls, ≤15 Zeilen),
nur `./target/release/archive_search`. Kosten aus `opencode.db`.

- **Venus-Express-VeRa-PSA-Route** · flash: kein PSA-DSN/ODF-Pfad, `IFMS/`-only direkt
  gefetcht (ODF/ 404, DSN_DOC-PDF 200), Rohroute = NASA/PDS `VEX-V-RSS-1-ENT-V1.0` $0.00894 ·
  max: kein PSA-DSN/ODF, keine RSS-Familie in allen drei PSA-Bäumen (170+21 VRA-Dirs),
  Dataset-Interieur nicht re-fetcht $0.02885 · **Sieger: flash**.
- **NOCC-Reduktionsvorschrift** · flash: Moyer ODP-Formulierung (Descanso 2, 200) +
  Morabito/Asmar TDA 42-120 (200) + 810-005 202E; 810-005 ohne Reduktionsmodul $0.01230 ·
  max: `dsn_redr`-Familie (2021-07-31, `redr_unpack.pdf`) + 810-005 202E/203E/209G;
  Verzeichnis gefetcht, PDFs nicht einzeln $0.02991 · **Sieger: flash**.
- **Dawn-PDS4-SBN-ODF-Route** · flash: beide Routen real (200), `.dat`/`.xml`-Paare,
  Sample `.dat` gefetcht $0.00253 · max: identisch, andere Samples $0.00598 ·
  **Sieger: flash**.

Regel: gleichwertige Antwort → Aufgabe bleibt bei flash; nur eine falsche oder
unvollständige flash-Antwort rechtfertigt den max-Lauf.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
