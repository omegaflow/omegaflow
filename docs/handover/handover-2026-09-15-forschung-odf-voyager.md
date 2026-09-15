<!--
  title: Handover — Forschung: Sonden-ODF + Voyager Saturn (Stand 2026-09-15)
  session: Forschung (Sonden-ODF + Voyager Saturn)
  class: handover
  date: 2026-09-15
  sha256: 265a79135181cd7aad9e945e7134f3a6115313903d38d9c011519440b1905f06
  status: live
  see-also: docs/auftrag/auftrag-sonden-rohdaten-anfragen.md, docs/paper/twenty-second-band-ground-chain.md
-->
# Handover — Forschung: Sonden-ODF + Voyager Saturn (2026-09-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## Voyager Saturn — Uhr dekodiert, Reader/Konsument offen

- Der Reader `src/archivar/voyager_saturn.rs` dekodiert die IDRSPS-Uhr (Table 4:
  word2 = YY/DOY/HH, word3 = MM/SS, word4 = SCID/network/station/band; Record-Type
  90/91, Ground-Mode 1-4 = Doppler); 7 Tests grün. Gemessen: 1980-295 .. 1981-261,
  Stationen 12,14,42,43,44,61,62,63 (DSN), 822 782 Tracking-Records (125 062 872 B).
  Die Register-Duty steht (`phi/blocked_sources.φ` pending + `phi/sources.φ`-Block).
  **Offen: kein `voyager_saturn`-Reader** — die bin-Slots sind generisch
  (`counter_word`/`value_word`/`secondary_word` tragen je nach kind Doppler, Range
  oder Winkel), Magic fehlt in `zeuge.rs`. (Schritt: Reader mit kind→Observable
  bauen, dann `voyager_saturn.bin`-CDN-Dispatch.)
- **Offen: die Winkel-Slots** (`status_a`/`status_b`) haben keine em-Einheit
  (`rad` steht nicht im em-Registry `units.rs`); die sources.φ-Zeile führt nur
  Doppler/range. (Schritt: Kraft/Einheit für Winkel entscheiden.)

## Sonden-ODF-Ernte — Blöcke stehen, CDN offen

- Die neun Blöcke (7 ODF-Compiler + Voyager + Dawn) stehen in
  `phi/blocked_sources.φ`, `voyager_saturn.bin` als `url`-Zeile in `phi/sources.φ`.
  Magellan gemessen 14 665 009 Samples (1 055 880 656 B); die übrigen ODF-Compiler
  tragen keine SCID/data_type/Record-Zahl-Konstanten (A = A: absent).
- **Dawn** — Route gefunden: PDS4-SBN
  `https://sbnarchive.psi.edu/pds4/dawn/gravity/dawn-rss-raw-{ceres,vesta}/data-odf/`
  (der frühere Fehlschlag prüfte `pds4/dawn/rss/`; `pds-geosciences/dawn/` bleibt 404).
  (Schritt: ODF-Compiler nach `*_odf_compiler`-Muster + `parse_odf`-Probe an einer .dat.)
- **Venus Express VeRa** — gemessen: PSA `VEX-V-VRA-*` trägt unter
  `DATA/LEVEL1A/CLOSED_LOOP/` nur `IFMS/`, kein `DSN/`/`ODF/`.
  (Schritt: PSA-Katalog `VEX-V-RSS-*` auf eine DSN/ODF-Route prüfen.)
- **CDN-Dispatch** der registrierten Quellen (celestrak-eop, voyager, die 7 ODF,
  Dawn nach Compiler). (Schritt: `gh workflow run` nach Push + Consent.)

## Bande-Split — Divergenz methodisch, Zensus offen

- **1988-Wertdivergenz** — gemessen: Split-Probe und Zensus-Probe lesen dieselbe
  Datei (`pioneer10_skyfreq.bin`); nur retrace/surrogate-null lesen `_6file.bin`.
  Die Divergenz ist damit methodisch (Split-`topk`-Selektion vs Zensus-`peak_of_cell`),
  nicht Datenversion — die Methoden-Ursache bleibt unverifiziert.
  (Schritt: beide Proben auf derselben 1988-Zelle diffen.)
- **160-Hz-Amplitudenzensus** — `pending`. (Schritt:
  `cargo run -p omegaflow-measure --bin pioneer_link_correction_probe` in CI dispatchen.)
- **NOCC-Reduktionsmaschine** — gemessen: nur Format-Specs im Baum
  (`dsn_trk-2-18`, `trk-2-25-atdf`, `810-202b`), die Reduktions-Vorschrift fehlt.
  (Schritt: Reduktions-SIS / 810-005-Reduktionsmodul beschaffen.)

## TE / Statistik

- **broken-null-control** — max-T-Korrektur über die 20-Paar-Matrix, Lag-Sweep
  über τ∈{0,60,120} hinaus, KDE-Bandbreite h ungemessen.
  (Schritt: `docs/paper/broken-null-control.md` + `src/mathematikerin/te.rs`.)
- **Blatt-Papier-Beweis/-Resultat** — multiple-comparison-Korrektur, Lag-Sweep,
  KDE-h; Blatt 2/3 trägt alle TE/lag/n/threshold `pending`; 36 Rotor-Paare;
  multi-force TE. (Schritt: Blatt 2/3 messen.)
- **Korona-Leiter** — bandbreiten-robuster Pfeil + zweites Fenster;
  per-Lag-Schwellen; multi-force TE ist implementiert, nicht berichtet.
  (Schritt: `docs/paper/corona-heating-ladder.md`, `corona_ladder_probe`.)
- **Solar 211A→193A** — conditional probe (stage-2).
  (Schritt: `docs/paper/solar-seconds-matrix.md`.)
- **Der Grat** — AIA-2014-per-cell-Schwellen; ENSO-Pfeil.
  (Schritt: `docs/concepts/blatt-der-grat.md`.)
- **depth-phase echo** — CMT-Strahlungsterm, 6 Kalibrier-Azimute, Δ≈30°.
  (Schritt: `docs/paper/depth-phase-echo-fleet.md`.)
- **GIC causal driver** — PCMCI, Minuten-Sturm-Ensemble, Tages-Lauf, KDE-h,
  Rückkanal. (Schritt: `docs/paper/gic-causal-driver.md`.)

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
- Toth/Turyshev/Markwardt-Mails — der Bande-Split (Split + Registerzeilen +
  Papier) ist geschlossen, die Prüfliste steht; die Mails sind entblockt.
  (Schritt: senden — in der Entscheid-Linie geführt.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
