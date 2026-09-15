<!--
  title: Handover — Forschung (Stand 2026-09-14)
  session: Forschung
  class: handover
  date: 2026-09-14
  sha256: 8f0ebd294e655fd214e5fb955d4dcf44d3a52ba4e42b6a1c8e3e2cb72cf0408e
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
  1-s-Klasse trägt mode 1 (alle drei Stationen) + mode 2 (nur st63) + mode 3; das
  Band sitzt in mode 3. Der Split (PASF-1-s-mode-3 gegen PNAV-`trans` gejoint,
  103 261 gematcht): die Frequenz ist **rx-fix** — für festes rx gleich, egal
  welcher Sender sendet (rx14 ≈ 45, rx43 ≈ 44–45, rx63 ≈ 55,6 mHz) → die Linie
  folgt der **Empfangskette**, nicht dem Sender. Offen daneben: (a) die Amplitude
  hängt vom (rx,tx)-Paar ab (rx14: 16 mit tx43 vs 125 mit tx63; rx63: 670 mit tx14
  vs 2,0 mit tx43); (b) rx63 reproduziert 55,9 statt 47,35 mHz (Teilmenge/Epoche
  zu prüfen). Instrument: `tools/measure/src/bin/pioneer10_txrx_split_probe.rs`.
- **Registerzeilen — geschlossen (Prüfliste vor der Toth-Mail)** — die Zahlen
  stehen, gemessen:
  - **f\*** — 50,73 mHz (0,1-mHz-Gitter) = 50,714 mHz (0,05-mHz-Gitter); 50,71 mHz
    ist der Alias-Partner (mit 949,29 mHz), keine Diskrepanz; die soliden Linien
    sind 45,75 / 51,55 / 47,35 mHz.
  - **1-s-Zählung** — 501 876 (volles ATDF-Set) ⊃ 73 249 (sub-10-s) ⊃ 70 602
    (strikte 1-s); 162 548 = 70 602 + 20 752 + 71 194 (Summe der drei
    Sampler-Klassen), kein Doppelzähler.
  - **Amplitude** — ~160 Hz = Station-14-Starksignal (160/153/161 Hz), nicht
    bandweit (Station 43: 104/102/82 Hz, Station 63: 57/50/65 Hz).
  - **Epochen-Persistenz** — 3 ATDF-Epochen (1987-12→1993-04); die 1-s-Klasse in
    zwei Dumps (1988, 1992); Drift Madrid 63: 47,35 → 46,95 mHz (0,40 mHz, ~4 J.).

## Zwei externe Anfragen stehen (kein anonymer Pfad, request-only)

Drei Taucherrunden (grind-flash + grind-pro, Godmode, rollierende Proton-VPNs,
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
