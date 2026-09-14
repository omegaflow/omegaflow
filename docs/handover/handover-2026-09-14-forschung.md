<!--
  title: Handover — Forschung (Stand 2026-09-14)
  session: Forschung
  class: handover
  date: 2026-09-14
  sha256: c90f480c52c241d4b2b5e84a0b03a19c3ad64068e26bab1f35fad4eae3758b5d
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

- **Bande-Split (20-s-Bande)** — Segment-Check gemessen: three-way existiert; die
  1-s-Klasse, die die starken Linien trägt, ist three-way-only (two-way 0/0/25 an
  Stationen 14/43/63, Deduktion 27). Der Split ist der Parser, nicht die Mail.
  (Schritt: CDN-Asset `https://github.com/omegaflow/sources/releases/download/spdf.gsfc.nasa.gov/pioneer10_skyfreq.bin`
  → `data/spdf.gsfc.nasa.gov/pioneer10_skyfreq.bin` holen + Ramp-Record-Join für
  den Uplink-Sender — der ATDF-Parser extrahiert bisher nur die Empfangsstation
  TKFORM[9], nicht den Sender.)

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
