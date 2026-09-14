<!--
  title: Handover — Warteliste-Suchlauf (Stand 2026-09-14)
  session: Warteliste-Suchlauf
  class: handover
  date: 2026-09-14
  sha256: bf1f3fdad8425a21bcfbabb4c617c3d233b48d58155e54779986a201ba705777
  status: live
  see-also: docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md
-->
# Handover — Warteliste-Suchlauf (2026-09-14)

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

## Zwei externe Anfragen stehen (kein anonymer Pfad, request-only)

Drei Taucherrunden (grind-flash + grind-pro, Godmode, rollierende Proton-VPNs,
Brave, Playwright) haben die Warteliste gemessen; zwei Punkte halten.

- **Voyager Roh-Doppler closed-loop (ODF/TRK-2-34)** — Voyager closed-loop wurde
  als ATDF/ODF aufgezeichnet, aber nie an PDS freigegeben; öffentlich überlebt nur
  die open-loop ODR-Okkultation (PDS-Rings). Einzige nicht-anonyme Ablage:
  NSSDC `PSNO-00007` (SDDPT, „archive, not distribution"). Die JPL/DSN-Anfrage
  hält. (Schritt: Antwort von JPL/DSN; sonst NSSDC-SDDPT-Antrag `PSNO-00007`.)
- **NSE/Haug Rohdaten** — die NSE-Zwischenstreufunktion I(q,t) von unterdotiertem
  YBCO (Haug et al., *New J. Phys.* 12, 105006 (2010), TRISP/MLZ). Kein Deposit:
  arXiv `e-print/1008.4298` = nur TeX + 8 Figuren; IOP-Suppdata hinter
  Radware-Bot-Manager; iMPULSE `record/2120` nur Metadaten + toter Volltext-Link.
  Die Keimer/MPI-FKF-Anfrage hält. (Schritt: Antwort von B. Keimer; Teilroute
  Paper-Fig. 5b digitalisieren.)

## Register-Disposition — Kandidaten ungeprüft

- 28 Weberin-Routen + 9 Auferstehungen liegen als `ausstehend kandidat` in
  `phi/pipeline/ledger.φ` (Runtime-Zustand, gitignored) — **nicht** durch `--probe`
  gezogen. (Schritt: `cargo run -p omegaflow-utils --bin archive_search -- --probe
  <url>` je Kandidat.)
- 6 Stub-URLs tragen `decline reachable-2026-09-14`; auf
  `decline no-measurement-2026-09-14` schärfen. (Schritt: `phi/declined_sources.φ`
  editieren.)

## Warteliste der Entscheid-Linie — hinfällige streichen

Der Survey `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`
trägt die gemessenen Alternativen: 8 hinfällig, 2 teilweise (Voyager ODR,
NSE/Haug), 2 offen, 6 lokal gedeckt (`.secrets.local`: DAHITI, ICIMOD,
SSDC/CSES-Limadou, Lasair, NASA ADS, Zenodo). Die Wartepunkte selbst leben in
`docs/handover/handover-2026-09-14-entscheid-folge6.md` §Warten auf Rückmeldung —
die hinfälligen gehören dort gestrichen. (Schritt: Entscheid-Linie, oder die
Zeilen hierher übernehmen.)

## Carry-over der konsumierten Linie (Register-Suchlauf)

- Himawari-8 AHI Block 6 + Kalibrierung anwenden (`himawari_hsd_compiler.rs`);
  GK2A/GOES-16 GSICS-Kalibrierung (`CALIB_GSICS_PENDING`). (Schritt: Compiler.)
- Tor-1: MACHO (keine Klassen-Spalte), LAMOST DR11 (Konsument PAST II
  `10.3847/1538-3881/ac0f08`), NOIRLab (Wiedervorlage Gaia DR4 2026-12-02); AQS
  keylos; Babamul 401; ONC CI-Lauf. (Schritt: je Register-Notiz in
  `phi/blocked_sources.φ`.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
