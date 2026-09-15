<!--
  title: Handover — Forschung: ODF-Ernte + Voyager Saturn (Stand 2026-09-15)
  session: Forschung (ODF-Ernte + Voyager Saturn)
  class: handover
  date: 2026-09-15
  sha256: bf970739c308dca392e54acfc326a28fded5cc5209a9b2678b537fab114e6894
  status: live
  see-also: docs/auftrag/auftrag-sonden-rohdaten-anfragen.md, docs/paper/twenty-second-band-ground-chain.md
-->
# Handover — Forschung: ODF-Ernte + Voyager Saturn (2026-09-15)

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

## Voyager Saturn (UNIVAC-1108) — gelesen, die Uhr offen

- **Der Saturn-Encounter ist dekodiert** (V1 `PSPA-00049`, V2 `PSPA-00123`):
  `src/archivar/voyager_saturn.rs` (2-B-Record-Header BE, 28 Sub-Records × 288 B,
  36-Bit-Wörter, 6 Tests grün) + `tools/harvest/src/bin/voyager_saturn_compiler.rs`
  (std-only TAR-Extraktion, V1+V2). Sample `DD059517_F1.DAT`: 1578 Records →
  44 169 Tracking-Records (Doppler 29 527 / Range 6 900 / Sync 7 742).
  **Offen: die Uhr.** w3/w8 sind zeitartig (monoton, wrap), Epoche + Einheit sind
  aus dem Sample nicht entscheidbar; **Station (DSS) nicht identifiziert**.
  (Schritt: NSSDC-Format-Doc `NSSD1260` oder die Voyager-Saturn-RSS-SIS laden und
  w3/w8 gegen einen bekannten Zeitstempel pinnen, die DSS-Bits in w2/w4/w17 suchen.)
- Register/CDN: `voyager_saturn.bin` ist noch keine `url`-Zeile in `phi/sources.φ`
  und nicht auf dem CDN. (Schritt: nach dem Uhr-Decode registrieren.)

## Sonden-ODF-Ernte — sieben Compiler gebaut, zwei offen

- Gebaut (TRK-2-34, `parse_odf` wiederverwendet, Muster `juno_odf_compiler`):
  Magellan (grün: 14 665 009 Samples, 1 055 880 656 B), MGS, MRO, Mars Odyssey,
  MESSENGER, Mars Express (MaRS), Rosetta (RSI) —
  `tools/harvest/src/bin/{magellan,mgs,mro,odyssey,messenger,mars_express,rosetta}_odf_compiler.rs`.
  **Register-Eintrag nicht committet**: `phi/blocked_sources.φ` ist fremd-staged
  (Migration) — die neun Blöcke (je `pending`, URL + SCID + data_type + gemessene
  Record-Zahl) sind aus den Compiler-Konstanten neu erzeugbar. (Schritt: bei
  ruhigem Baum die Blöcke in `phi/blocked_sources.φ` + `phi/sources.φ` schreiben.)
- **Dawn** — kein anonymer ODF-Pfad: PPI `/data/` 404, PDS-Geosciences `/dawn/`
  404, SBN `pds4/dawn/` trägt GRaND/gravity/mission ohne RSS. (Schritt:
  PDS-Katalog nach dem Dawn-RSS-ODF-Bundle durchsuchen.)
- **Venus Express (VeRa)** — PSA `VEX-V-VRA-1-2-3-*` trägt unter
  `DATA/LEVEL1A/CLOSED_LOOP/` nur `IFMS/`, kein DSN/ODF; `parse_odf` findet keine
  36-B-Orbit-Records. (Schritt: PSA-Katalog `VEX-V-VRA` auf eine DSN/ODF-Route prüfen.)

## Bande-Split — Papier gelandet, Reste offen

- Papier `docs/paper/twenty-second-band-ground-chain.md` **v5** (Council-Landing:
  Bandgrenze 44–58-mHz-Raster, struck drift, der station-lokale Atmosphären-Zweig
  benannt, Split-Serie + Zensus im §4) und der Sibling
  `docs/paper/probe-front-dark-matter.md` **v8** (Deduktion 27 auf den gemessenen
  Split reframed, struck drift).
- Offen: die **1988-Wertdivergenz** (Split-Peaks rx14 46,58 / rx43 44,12 /
  rx63 50,92 mHz vs. kanonischer Zensus 57,11 / 44,40 / 51,99 mHz) — Ursache
  offen (Datenversion oder Methode); der **160-Hz-Amplituden-Zensus** (`pending`);
  die **NOCC-Reduktionsmaschine** unbenannt. (Schritt: Amplitude-Zensus auf der
  kanonischen Serie fahren; die NOCC-Reduktionsdoku suchen.)

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

- **Nadeln Ⅷ Dunkler Fluss · Ⅸ FRB · Ⅹ Kugelblitz · Ⅻ Urknall** — Dunkler Fluss
  lebt in `docs/paper/dark-flow-sheet-8.md`, FRB in `frb_blatt_probe.rs`, Urknall
  in `bigbang_echo_probe.rs`; Kugelblitz trägt kein Probe-Bin im Baum (nur
  Archiv-Handover + `kybernetische-astrophysik.md`). (Schritt: Status je Nadel
  messen; Kugelblitz anlegen oder als `pending` tragen.)
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
