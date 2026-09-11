<!--
  title: Recherche-Befund (extern) — Galileo-S-Band-Residualsprung 1995-11-30/12-01: Borduhr-Sprung (A) oder Bodenseite (B)?
  class: concept
  date: 2026-09-06
  sha256: a43dbc44b2660d8bc60220e39a0e528c18d0509920a75391c48cd5f89cfed9b5
  status: live
  see-also: tools/measure/src/bin/galileo_floor_sustained_lock_state.rs tools/measure/src/bin/galileo_floor_basis_ruck.rs tools/measure/src/bin/galileo_ruck_zeugen.rs tools/measure/src/bin/galileo_ruck_nachmessen_f3.rs
-->
# Recherche-Befund: Galileo-S-Band-Residualsprung 1995-11-30/12-01 — Borduhr-Sprung (A) oder Bodenseite (B)?

Messung gegen öffentliche Quellen (Web/NTRS/JPL-TDA-TMO-Archiv/PDS-Kontext).
Zugriffstag für alle Abrufe: 2026-09-06. Jede Aussage trägt Quelle + HTTP-Status.
Reine Recherche; keine Code-Änderung.

## Vorbemerkung (gemessene Umrechnung, keine Spekulation)

S-Band-Downlink f = 2295 MHz; Dopplerskala dv/df = c/f = 0,1307 m·s⁻¹·Hz⁻¹.
0,80–0,82 Hz ≈ 10,5–10,7 cm/s. Fraktioneller Schritt 0,8/2,295e9 ≈ 3,5e-10.
Mode-1-only (Einweg): Nur der Einweg-Residual hängt von der angenommenen/echten
Sonden-Referenzfrequenz (USO bzw. nicht-kohärenter Sender) ab; Zwei-/Dreiweg
(kohärent) bildet den Downlink als festes Vielfaches des Uplinks ab und ist
gegen einen Borduhr-Versatz unempfindlich — dies erklärt die Mode-1-only-Signatur
für BEIDE Kandidaten und trennt sie nicht.

---

## 1. Gefundene unabhängige Analysen über das Dez-1995-Fenster (Einweg)

| # | Analyse | Autor/Jahr | Quelle (URL, Status) | Überdeckt Fenster? |
|---|---------|-----------|----------------------|--------------------|
| 1 | USO-Flugleistung Galileo Orbiter (82 Passes, Okt 1989–Nov 1991) | Morabito/Krisher/Asmar, 1993 | NTRS 19930020414 (TDA PR 42-114, 1993-05-15) + 19940009925 (42-115, 1993-08-15); https://ntrs.nasa.gov/citations/19930020414 — HTTP 200 | **Nein** (endet Nov 1991) |
| 2 | „Characteristic Trends of Ultrastable Oscillators for Radio Science Experiments" | Asmar, 1997 | TDA PR 42-129, 129F.pdf, https://ipnpr.jpl.nasa.gov/progress_report/42-129/129F.pdf — HTTP 200 | Nein (Technologie-Übersicht, keine Pass-Serie 1995/96) |
| 3 | „Jupiter's ionosphere: Results from the First Galileo Radio Occultation Experiment" | Hinson, Flasar, Kliore, Schinder, Twicken, Herrera, 1997 | GRL, DOI 10.1029/97gl01608; Abstract via OpenAlex https://api.openalex.org/works/doi:10.1029/97gl01608 — HTTP 200 (AGU-Volltext 403) | **Ja** (Okkultationsdaten 1995-12-08, eine Woche nach dem Sprung) |
| 4 | „Analysis of Galileo Doppler Measurements during the Solar Occultations in 1994 and 1995" | Wohlmuth, Plettemeier, Edenhofer, Bird, Asmar, 1997 | The Three Galileos (ASSL 220), S. 421–428, DOI 10.1007/978-94-015-8790-7_41; Abstract https://link.springer.com/chapter/10.1007/978-94-015-8790-7_41 — HTTP 200 (Volltext Paywall) | **Ja** (S-Band-Doppler, ~30-Tage-Läufe an der 1995er-Konjunktion = Jupiter-Arrival-Ära) |
| 5 | „The Galileo Mission to Jupiter: Interplanetary Cruise Post-Earth-2 Encounter Through Jupiter Orbit Insertion" | Beyer, Mudgway, Andrews, 1996 | TDA PR 42-125, https://ipnpr.jpl.nasa.gov/progress_report/42-125/125B.pdf — HTTP 200 | **Ja** (Ereignis-Narrativ Nov/Dez 1995) |
| 6 | Navigation: „Navigating Galileo at Jupiter Arrival" | Haw, Antreasian, McElrath, Graat, 1997 | JSR 34(4); DOI 10.2514/2.3240; Crossref-Metadaten HTTP 200; Volltext Paywall (nicht gelesen) | teilweise (Ansage, OD), kein Frequenz-Schritt-Statement |
| 7 | Navigation: „Galileo Navigation: Launch to Jupiter Orbit" | D'Amario, 1997 | NTRS 20060036761 (Abstract-only), https://ntrs.nasa.gov/citations/20060036761 — HTTP 200 | teilweise, kein Frequenz-Schritt-Statement |

---

## 2. Befund je Quelle

### (1) Morabito/Krisher/Asmar, USO-Flugleistung — Fenster NICHT überdeckt
Abstract (NTRS 19930020414, HTTP 200, wörtlich):
> „Estimates for the USO-referenced, spacecraft-transmitted frequency and frequency
> stability were made for 82 data acquisition passes conducted between launch
> (Oct. 1989) and Nov. 1991. … The Galileo USO appears to be healthy and functioning
> normally in a reasonable manner."

Zeigt die ~0,8-Hz-Diskontinuität um Nov/Dez 1995? **Nicht anwendbar** — die einzige
öffentliche Absolutfrequenz-Zeitreihe der Sonden-Sendefrequenz endet Nov 1991.
Fortsetzung in die Jupiter-Orbital-Phase (1995–97): **nicht gefunden** (gemessene Lücke):
- NTRS-Suche „Galileo USO" → nur die 1993er Datensätze (19930020414, 19940009925,
  20060038990, 20210004469) — HTTP 200.
- NTRS-Suche „Galileo oscillator frequency Jupiter" → total 0 Treffer — HTTP 200.
- NTRS-Suche Autor „Morabito Galileo" → kein USO/Frequenz-Artikel nach 1993 — HTTP 200.

### (2) Asmar 42-129 USO-Übersicht — keine Pass-Serie über 1995/96
Survey über USO-Klassen; Galileo-Orbiter-USO = Voyager-Klasse (Quarz, SC-Schnitt).
Referenz [2] dort verweist auf die 1993er Flugleistungs-Analyse (Morabito, Krisher,
Asmar, IEEE Frequency Control Symposium, Juni 1993). Enthält keine
Frequenz-über-die-Zeit-Serie der Galileo-Orbitalphase und keinen Schritt-Befund
(TDA PR 42-129/129F.pdf, HTTP 200).

### (3) Hinson et al. 1997, erste Jupiter-Ionosphären-Okkultation — überdeckt das Fenster, zeigt den Schritt nicht
Abstract (OpenAlex, HTTP 200, wörtlich):
> „The Galileo spacecraft passed behind Jupiter on December 8, 1995, allowing the
> first radio occultation measurements of its ionospheric structure in 16 years."

Daten = eine Woche NACH dem Sprung. Während der Okkultation liegt die Sonde hinter
dem Planeten; der Downlink ist dort zwingend Einweg/nicht-kohärent (physikalische
Folgerung aus der Geometrie; die Sonde empfängt in dieser Phase keinen Uplink).
Diese Einweg-Reduktion berichtet **keine** anhaltende ~0,8-Hz-/10-cm/s-Diskontinuität
der Sendefrequenz. Sie kann sie aber auch nicht zeigen: Eine Okkultations-Inversion
leitet Elektronendichte aus der FrequenzABLEITUNG/Basislinie ab; ein konstanter
Sendefrequenz-Versatz wird als konstante Frequenzresidual-Basislinie entfernt und
ist der Reduktion unsichtbar. ⇒ Weder Beleg für (A) noch Beleg gegen (A);
konsistent mit (B), aber kein Trenn-Test.

### (4) Wohlmuth/Plettemeier/Edenhofer/Bird/Asmar 1997, 1994+1995er Konjunktion — Fenster überdeckt, Schritt nicht adressiert
Abstract (Springer, HTTP 200, wörtlich):
> „Measurements of S-band downlink frequency (Doppler) shift were collected for
> intervals of about 30 days during the 1994 and 1995 solar conjunctions of the
> Galileo spacecraft. … Spectral analysis was carried out with the S-band Doppler
> scintillations …"

Die 1995er Konjunktion = Jupiter-Arrival-Ära (Beyer et al., siehe (5): SEP < 7°
ab 1995-12-09, Konjunktionsperiode); das ~30-Tage-Fenster liegt damit um
Nov/Dez 1995 und spannt den Sprung. Die unabhängige Bonner/JPL-Gruppe reduzierte
S-Band-Doppler über genau dieses Fenster — für koronale Szintillations-Spektren und
Zwei-Stationen-Kreuzkorrelation, nicht für Absolutfrequenz. Ein konstanter
Frequenzversatz ist in dieser Reduktion nicht sichtbar/adressiert. ⇒ überdeckt,
zeigt keinen Borduhr-Sprung, kann ihn aber methodisch nicht ausschließen.

### (5) Beyer/Mudgway/Andrews 42-125, Missions-Narrativ um den Sprung — kein Borduhr-Ereignis dokumentiert
Volltext (TDA PR 42-125/125B.pdf, HTTP 200). Die im Fenster 1995-11-30/12-01
dokumentierten Ereignisse (wörtlich/nah am Text):
- „To permit the Probe radio oscillators to temperature stabilize, on November 27
  they were turned on" — **Probe**-Oszillatoren, nicht Orbiter-Referenz.
- „On November 30, the first phase of the Jupiter zero orbit encounter (JOE-A)
  prime sequence memory load … was uplinked … The J0E-B prime sequence memory load,
  which included the critical relay/JOI time period, was uplinked to the spacecraft
  on December 1." — Sequenzladungen (Bodenkommandos), keine Frequenzänderung.
- „The spacecraft downlink transitioned from the suppressed-carrier mode … to the
  residual-carrier mode … on December 5" — Modulationsindex-Wechsel, keine Frequenz.
- „The Galileo spacecraft arrived at Jupiter on December 7, 1995" (relay, JOI-Burn).
- „With a solar separation angle of less than 7 deg on December 9, the spacecraft
  entered the solar conjunction period."
- Radio-Science-Kontext: „the Solar Wind Scintillation Experiment using the
  spacecraft radio frequency equipment resumed on November 24 and was completed on
  December 4" (DSS 43) — d. h. über die Sprung-Grenze lief aktive Einweg-RS-Doppler-
  Erfassung.
Der einzige Hz-große Doppler-Sprung, den der Artikel nennt, ist −0,6 Hz bei der
Probe-Trennung am 1995-07-12 (nicht im Fenster): „the first indication of release
being a change in the spacecraft Doppler data of −0.6 Hz."
⇒ Im öffentlichen Missions-Narrativ ist **kein** Orbiter-Referenz-/Sendefrequenz-
Ereignis um 1995-11-30/12-01 dokumentiert.

### (6) Haw et al. 1997 (Navigation, Jupiter-Arrival) — kein Frequenz-Schritt-Statement
Volltext nicht zugänglich (JSR-Paywall; nicht gelesen). Metadaten/Abstract (Crossref,
HTTP 200) enthalten kein Statement zu einer Sende-/Borduhr-Diskontinuität. Navigation
stützte sich auf Zweiweg-Doppler; ein Mode-1-Versatz wäre in OD-Frequenz-Biases
absorbiert und nicht berichtspflichtig. ⇒ kein Beleg, kein Gegenbeleg (ehrlich
`pending` für diese Quelle).

### (7) D'Amario 1997 (Navigation, Launch→Jupiter Orbit) — kein Frequenz-Schritt-Statement
NTRS 20060036761, Abstract-only (HTTP 200): behandelt Flugbahn/OD-Probe/JOI;
kein Oszillator-/Frequenz-Schritt erwähnt.

### Gravitationswellen-Experiment (Kandidatenfamilie 3)
Einzig gefundenes Galileo-Doppler-GW-Experiment: „System Performance of the Joint
Galileo/Mars Observer/Ulysses 1993 Gravitational Wave Experiment" (TDA PR 42-114,
1993; NTRS 20060038891, Abstract, HTTP 200) — **vor** dem Fenster. Keine
1995/96-Folge mit Einweg über den Sprung in NTRS/Crossref gefunden (gemessene Lücke).

### TDA/TMO-Progress-Report-Serie 42-124…42-133 (Kandidatenfamilie 5)
Inhaltsverzeichnisse 42-124…42-133 einzeln abgerufen (je HTTP 200,
https://ipnpr.jpl.nasa.gov/progress_report/42-XXX/title.htm). Ergebnis (gemessen):
- Galileo/USO/Frequenz-Stabilitäts-/Kalibrier-Artikel: **keine**.
- Enthaltene Galileo-Treffer: 42-125 Beyer et al. (Cruise→JOI, siehe (5));
  42-127 „Twice-a-Day Frame Loss in Galileo's Telemetry" (Kommunikation);
  42-133 „DSN Support for the Galileo Mission to Jupiter: Jupiter Orbital Operations
  …" (nach dem Fenster, 1996–97).
- 42-129 Asmar-USO-Übersicht (siehe (2)).

---

## 3. Verdikt

**Das öffentliche Material ist unzureichend, um (A) gegen (B) zu trennen — Befund: `pending`.**

Begründung (je eine Messung):
1. Die einzige öffentliche Absolutfrequenz-Zeitreihe der Galileo-Sendefrequenz
   (USO-Flugleistung, Morabito et al. 1993) endet **Nov 1991**; eine Fortsetzung in
   die Orbitalphase ist in NTRS/JPL-PR-Reihe nicht auffindbar. Damit existiert kein
   öffentlicher Absolutfrequenz-Beleg für ODER gegen einen anhaltenden ~0,8-Hz-Sprung
   der Borduhr um Dez 1995.
2. Die unabhängigen Einweg-Reduktionen, die das Fenster überdecken (Jupiter-Okkultation
   1995-12-08, Hinson 1997; 1995er-Konjunktions-S-Band-Doppler, Wohlmuth 1997;
   SWS-Lauf 1995-11-24…12-04, Beyer 1996), reduzieren **relative** Frequenz und
   berichten keinen Borduhr-Sprung — sie sind methodisch blind gegen einen konstanten
   Sendefrequenz-Versatz und trennen daher nicht.
3. Kein öffentliches Dokument nennt ein Orbiter-Referenz-/Oszillator-Ereignis um
   1995-11-30/12-01; das Missions-Narrativ (Beyer 1996) nennt für das Fenster nur
   Sequenzladungen und den Modulationswechsel vom 5. Dez.
4. Für Kandidat (B) spricht nur indirekt, dass unabhängige wissenschaftliche
   Einweg-Reduktionen über die Ära ohne einen Borduhr-Sprung auskamen — ein schwaches,
   kein trennendes Signal. Ein konkretes bodenseitiges Predict-/Modell-Update am
   1995-11-30/12-01 ist öffentlich ebenfalls nicht dokumentiert.

**Was die Lücke schließen würde (benannt, nicht erraten):** eine öffentliche
Reduktion, die die Sonden-Sendefrequenz (USO-referenziert, S-Band, Einweg) als
Absolutwert über die Grenze 1995-11-30/12-01 hinweg führt — z. B. eine
Fortsetzung der Morabito-42-113/114-Linie, die Pass-Schätzung der RSS-Datenköpfe
der PDS-Sätze (GO-J-RSS-*), oder ein Einweg-Doppler-Residual-Datensatz, der beide
Seiten der Grenze mit derselben Reduktion enthält. Bis dahin bleibt die Trenn-Frage
`pending`.

## Quellenindex (Status je Abruf 2026-09-06)
- NTRS 19930020414 & 19940009925 (USO-Flugleistung) — HTTP 200.
- NTRS 20060036761 (D'Amario, Navigation) — HTTP 200; 20060038891 (GW 1993) — HTTP 200.
- NTRS-API-Suchen „Galileo USO", „Morabito Galileo", „Galileo oscillator frequency
  Jupiter" — je HTTP 200 (eine parallele Doppelabfrage anfangs HTTP 429).
- TDA PR 42-125 (125B.pdf) — HTTP 200. TDA PR 42-129 (129F.pdf) — HTTP 200.
- IPNPR-Inhaltsverzeichnisse 42-124…42-133 — je HTTP 200 (Root-Pfad /progress_report/
  selbst: HTTP 403, Einzelhefte erreichbar).
- GRL 1997 (10.1029/97gl01608) Abstract über OpenAlex — HTTP 200; AGU-Volltextseite — HTTP 403.
- Wohlmuth 1997 Abstract über link.springer.com — HTTP 200; Volltext Paywall.
- Haw 1997 (10.2514/2.3240) Crossref-Metadaten — HTTP 200; JSR-Volltext Paywall.
- Crossref-API-Abfragen — HTTP 200 (eine Semantic-Scholar-Abfrage HTTP 429).
