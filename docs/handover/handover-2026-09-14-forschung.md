<!--
  title: Handover — Forschung (Stand 2026-09-14)
  session: Forschung
  class: handover
  date: 2026-09-14
  sha256: 7eccc1965c72ac63bfab09a7ba8d1cad24ee3a26bae9c1b8439a4b122443ae21
  status: live
  see-also: docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md
-->
# Handover — Forschung (2026-09-14)

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

## Bande-Split (20-s-Bande)

- **Bande-Split (20-s-Bande) — Split gemessen: Empfänger-Wanderung.** Die
  1-s-Klasse trägt alle drei ground_modes (mode 1 = 30 973, mode 2 = 11 631 [nur
  st63], mode 3 = 154 150; 78,3 % three-way) — nicht three-way-only; das Band
  sitzt in mode 3. Der Split (PASF-1-s-mode-3 gegen PNAV-`trans`, 103 261
  gematcht): die Frequenz ist **rx-fix** → die Linie folgt der **Empfangskette**.
  Beide Wrinkles aufgelöst: (a) die Amplitude hängt **real** vom (rx,tx)-Paar ab —
  innerhalb einer Epoche (1988) bei vergleichbarem n bleibt der Faktor (rx14 13,9×,
  rx43 ≈3124×, rx63 ≈59×); die LS-Amplitude ist n-unabhängig → ein Zweiweg-Effekt
  (der Uplink setzt die Stärke), kein Epochen-/n-Artefakt. (b) rx63 = 55,9 mHz ist
  der **1993-Epochen-Peak** (52 985 der 92 130 rx63-Samples), kein Gitter-Artefakt;
  das Papier (st63 → 47,35 mHz 1988) ist **nicht reproduziert** — auch die
  1988-Werte differieren (rx63 50,92, rx14 46,58, rx43 44,12); Ursache offen
  (Datenversion: aktueller Bestand 1 071 540 Records / 196 754 1-s vs. Papier
  73 249 sub-10-s — oder Methode). Instrument:
  `tools/measure/src/bin/pioneer10_txrx_split_probe.rs`.
- **Papier-Nichtreproduktion (volle Kette gemessen).** A/B/C: die Dateizahl (B)
  und die Methode (C) sind ausgeschlossen; die Papier-Werte (45,75 / 51,55 /
  47,35 mHz) reproduzieren nicht. **Volle Kette** (der frühere Taucher übersprang
  die Media-Stufe): die Lücke war die DATEN — OMNI2 regeneriert (`omni2_compiler`
  → `cache/omni2_serie.bin`; Plasma auf 146 417/162 548 Samples), common-mode
  **genuin leer** (min. Stationsabstand 110 s > 60 s Toleranz; DSN sequenziell),
  TEC **datiert leer** (GIM ab 1998). `resid_e` trägt exakt die Papier-Zahlen
  (strict-1,0-s **70 602**, sub-10-s **73 249**). Ergebnis: **kein Papier-Wert ist
  irgendwo dominant**; 47,35 dominant nirgends; **50,73 reproduziert nicht**
  (gesäubert global 49,16 / 44,65; ungesäubert 44,4 mHz, nicht detrend-robust; die
  Papier-Zählungen 70 602/73 249 sind die post-gate-Zählungen). **Surrogat-Null: 2/4 Selektion** (st14 45,75, st63 1992
  46,95; Null mean+2σ = 0,65, P≈2,5·10⁻³) — st43 51,55 und st63 1988 47,35
  überleben nicht. Die qualitative Station-Fixität steht (beide Serien).
  1989–1991: die Markwardt-ATDF (.DAT/.TDR/.TDF) ist eine echte Archiv-Lücke
  (deckt 1987/1988/1994; Markwardt selbst dokumentiert Juni 1990–Juni 1991 als
  unlesbares Band, arXiv gr-qc/0208046); die Doppler-Werte sind NICHT absent — sie
  sind geerntet: TRK-2-25-ODF `86334o97343_sc23.odf` → `pioneer10_odf.bin`
  (59 486 Samples, 1973–1998; 1989–1991: 790/572/1548) + `.asc.gz` → NAVIO
  (1973–2002). (Schritt:
  Papier **v4** — Zensus statt Pick, die alten Werte mit gemessenen Rängen,
  Drift-Satz streichen, §2 bleibt; Council-Landing.) Instrumente:
  `pioneer10_txrx_split_probe`, `pioneer10_paper_chain_retrace`,
  `pioneer10_cell_census_probe`, `pioneer10_surrogate_subpeak_null`,
  `pioneer10_paper_harvest_probe`, `pioneer10_method_sensitivity_probe`.
- **Registerzeilen — geschlossen (Prüfliste vor der Toth-Mail)** — die Zahlen
  stehen, gemessen:
  - **f\*** — der globale Peak 50,73 mHz reproduziert nicht (gesäubert global
    49,16 / 44,65 mHz, ungesäubert 44,4 mHz, nicht detrend-robust); die
    Stationswerte 45,75 / 51,55 / 47,35 mHz sind eine Sub-Peak-Auswahl — auf der
    kanonischen Serie gemessene Ränge 4 / 2 / 17 (51,50 ein 0,05-mHz-Gitterschritt
    unter 51,55) —, keine soliden Linien.
  - **1-s-Zählung** — 501 876 (volles ATDF-Set) ⊃ 73 249 (sub-10-s) ⊃ 70 602
    (strikte 1-s); 162 548 = 70 602 + 20 752 + 71 194 (Summe der drei
    Sampler-Klassen), kein Doppelzähler.
  - **Amplitude** — auf der kanonischen Serie re-ankert die Linienamplitude auf
    ~5 Hz (Station 14, 1988); der 160-Hz-Wert (160/153/161 Hz, Station 14) ist
    `pending` — der eigene Zensus steht aus.
  - **Epochen-Persistenz** — 3 ATDF-Epochen (1987-12→1993-04); die 1-s-Klasse in
    zwei Dumps (1988, 1992); der Drift-Satz (47,35 → 46,95 mHz, 0,40 mHz, ~4 J.)
    ist gestrichen — sein 1988-Anker 47,35 erscheint nicht, die Drift wird nicht
    getragen.

## Zwei externe Anfragen stehen (kein anonymer Pfad, request-only)

Drei Taucherrunden (grind-flash + grind-pro, `archive_search --all`, rollierende Proton-VPNs,
Brave, Playwright) haben die Warteliste gemessen; zwei Punkte halten.

- **Voyager Roh-Doppler closed-loop (ODF/TRK-2-34)** — Voyager closed-loop wurde
  als ATDF/ODF aufgezeichnet, aber nie an PDS freigegeben. Offene Teilroute
  (gemessen 200): SPDF Saturn-Encounter-Daten, UNIVAC-1108-Binär, closed-loop
  Doppler+Range — V1 `PSPA-00049` (14 TARs, Okt–Nov 1980), V2 `PSPA-00123`
  (6 TARs, Aug–Sep 1981),
  `spdf.gsfc.nasa.gov/pub/data/voyager/{1,2}/radio_science_rss/saturn_encounter_data/`.
  Grenze: Saturn-Ära + UNIVAC-Format (kein ATDF/ODF/TRK-2-34-Parser); Cruise-/
  Post-Saturn-Fenster + natives Format bleiben request-only (NSSDC `PSNO-00007`,
  SDDPT). (Schritt: UNIVAC-1108-Parser für die Saturn-TARs; die JPL/DSN-Anfrage
  fürs Cruise-Fenster hält weiter.)
- **NSE/Haug Rohdaten** — die NSE-Zwischenstreufunktion I(q,t) von unterdotiertem
  YBCO (Haug et al., *New J. Phys.* 12, 105006 (2010), TRISP/MLZ). Kein Deposit:
  arXiv `e-print/1008.4298` = nur TeX + 8 Figuren; IOP-Suppdata hinter
  Radware-Bot-Manager; iMPULSE `record/2120` nur Metadaten + toter Volltext-Link.
  Die Teilroute „Paper-Fig. 5b digitalisieren" ist gemessen descoped — Fig 5b
  trägt Γ(T) (quasielastische HWHM vs Temperatur), nicht I(q,t); das Paper
  publiziert I(q,t) nirgends. Die Keimer/MPI-FKF-Anfrage ist der einzige Pfad.
  (Schritt: Antwort von B. Keimer.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
