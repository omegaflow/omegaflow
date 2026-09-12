<!--
  title: Handover — Papier-Reihe: sieben Papers, sieben Surveys, Fix-Atom (Stand 2026-09-12)
  session: Papier-Reihe
  class: handover
  date: 2026-09-12
  sha256: c88c5670b643ac1e2463a7e06ac01b16e172fba2d51a992fff90f5d245fac20f
  status: archived
-->
# Handover — Papier-Reihe (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der
Baum ruhig ist.

## Gebaut in diesem Atom

- **Sieben Papers** in `docs/paper/` — alle Export-Gate-grün (Titel ≤ 75,
  Abstract ≤ 200, Zahlen-Match, sha-Match) — und **sieben Axiom-Gate-Surveys**
  in `docs/surveys/axiom-gate-<slug>.md`. Beleg-Kette: 25/25 arXiv-Kennungen
  `resolved`, alle DOIs `resolved`.
- **Fix-Atom (Operator-Wort, 2026-09-12):**
  - **Drei Alt-Papers Gate-grün:** `jwst-disequilibrium-survey` (Abstract
    202 → 195 W), `woo-armstrong-1979-jgr-abstract` (Titel 83 → 59 Zeichen,
    sha256-Feld nachgetragen), `text-as-data-pioneer` (Header-sha auf den
    gemessenen Body-sha gebracht) — und zwei weitere, die der volle
    Gate-Lauf dazu maß: `armstrong-1998-phase-scintillation-abstract`
    (Titel 94 → 62 Zeichen, sha256 nachgetragen, toter see-also auf den
    echten Asmar-Pfad gestellt) und das Asmar-2005-Record (ohne Header
    geblieben — Header + sha256 nachgetragen; der Register-Titel ist die
    gekürzte Form, die Original-Überschrift bleibt im Body).
  - **Kollab-Blatts `status: pending` geschlossen:** die drei konditionierten
    cTE-Werte gemessen (`cross_te_screen`, 134 s, 20 Surrogate: gyirong→rasuwa
    0.1983 > 0.1815; gyirong→kollab 0.1602 > 0.1568; kollab→gyirong 0.1584 >
    0.1538) — Blatt auf `live`, Grat-Zelle nachgezogen, Paper + Survey
    nachgeführt; der marginale rasuwa→kollab-lag-6-Zusatz (0.2099 > 0.2097)
    ist benannt, nicht versteckt.
  - **ak135-Divergenz aufgelöst:** der Code trug die Wahrheit
    (`MAX_DEPTH_KM = 700.0`, Erweiterung seit 2026-09-09 geschlossen,
    `ak135.dat` bis 6371 km); die veraltete Register-/Konzept-Zeile
    korrigiert, Paper + Survey nachgeführt.
  - **ADS-Kennungen belegt (ADS-API, 2026-09-12):** Espenak 2006fmcs.book.....E
    (der Erstansatz war absent — korrigiert); Fienga 2019NSTIM.109.....F;
    Pitjeva & Pitjev auf das verifizierte 2014-Record korrigiert (CeMDA 119,
    237, DOI 10.1007/s10569-014-9569-0 — die 2018-Zeile war eine
    Verschmelzung zweier Records); alle übrigen Bibcodes der vier Papers
    `resolved`.
  - **Matrix-Defekt getragen:** Builder-Fix (2026-09-09) + Re-Verifikation
    (vier Linien Δ 0,0 km) stehen jetzt im Eclipse-Paper + Survey.
  - **Die Glocke:** der Rotor-Spin-Ton trägt sein Paper
    (`galileo-rotor-spin-era-floor.md`) — kein Doppel-Blatt; die Erd-
    Eigenmoden stehen als registrierte Frage (Rat) unten.

## Offene Register-Pflichten (benannt, getragen — jeder Empfänger explizit)

- **H₀:** der eigene Gaia-TAP-Crossmatch der 75 SH0ES-Cepheiden (`pending`)
  und der Schlichter (dritter Faden ≲1–2 %) — getragen in
  `docs/blatt/blatt-h0-linien-register.md`.
- **Finsternis:** die 65-s-Diagnose bleibt Restbefund (fakultativ) —
  getragen im Paper `docs/paper/eclipse-clock-worldlines.md`.
- **de441 mars** trägt weiter Vor-Fix-Matrizen (Δ Anker 6045,3 km); der
  kernel-flatten-Run 34348392827 endete `failure` — getragen im thematischen
  Handover `docs/handover/handover-thematisch-membran-sonde.md`.
- **Echo-Tiefe:** CMT-Quell-Strahlungsterm; Kalibrier-Gate (die sechs
  Stationsazimute des Feldpilots in keinem Register) — getragen in
  `docs/concepts/die-akteure-im-boden-und-wasser.md`.
- **Uranus:** Absolut-Offset/Aberrations-Zerlegung; Neptun-Bau-Linie —
  getragen im Paper `docs/paper/uranus-rift-ephemerides.md`.
- **Solar:** konditionale Prüfung des 211A→193A-Pfeils (`pending`) —
  getragen in `docs/blatt/blatt-solar-seconds-matrix.md`.
- **Tōhoku:** Eikonal-CI re-dispatched — **Run 34718368835** (queued),
  Wächter = grüner Lauf gegen ETOPO1 — getragen in
  `docs/concepts/die-akteure-im-boden-und-wasser.md` (Tōhoku-Kette); der
  M9.1-Picker-Nachfolger (USGS-Mww-Zentroid) läuft in der Forschung-Linie.
- **Erd-Eigenmoden („die Glocke")** — registrierte Frage (Rat): freie
  Eigenmoden nach Großbeben, Meßkette miniSEED/ak135/Flotten-Stationen —
  getragen in `docs/concepts/die-akteure-im-boden-und-wasser.md`; erst
  Frage → Messung → Paper, nie ein Titel ohne Messung.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der
gemessene Abschluss-Check. Der Baum trägt fremde uncommittete Arbeit
(Firmware-Trio einer Parallel-Session) — committet wird nur der eigene
Pfad-Satz; gepusht wird erst, wenn der Baum ruhig ist.
