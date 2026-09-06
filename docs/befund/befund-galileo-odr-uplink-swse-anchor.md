<!--
  title: Befund — Galileo-ODR-Uplink-Feldprüfung (Ded-27, Richtung J) + SWSE4-Zweiter-Zeuge der frühen Anker (Teil b): der Galileo-ODR-Record (RSC-11-11/13) trägt weder TRACKING_MODE noch Uplink-DSS — Wort-Zensus über 10 ODR-Dateien zeigt als einziges Stations-Wort byte 6 (Wort-4-Prime-FEA = Empfangsstation), byte 7 (Secondary FEA) = 0 in allen Records, kein Sender-Feld (das Uplink-DSS-Feld der Grail-RSR-SFDU ist eine spätere Recorder-Generation); und die SWS4-ODR-Dateiabdeckung beginnt im GO-SUN-Archiv erst 1995-12-08T16:15 (GORS_1032/1033, INDEX 723 Zeilen) — die Anker 1995-11-24 und 1995-12-04/05/06 tragen n=0 GO-SUN-ODR-Dateien, die SWS4-Kampagnen-Spanne (95-328..96-013) ist nicht die Dateiabdeckung — Ded-27 bleibt aus dem ODR strukturell nicht direkt schließbar, die frühen lauten Anker bleiben ohne open-loop-Zeugen
  class: befund
  date: 2026-09-06
  sha256: fa582d473f1791069b09477cb0bbb1268a45f7c9b61e47cc03f6943faf3e7eeb
  status: draft
  see-also: docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-goj-odr-ded31-sameday.md docs/befund/befund-galileo-odr-goj-beschaffung2.md docs/befund/befund-galileo-receiver-je-pass-floor.md docs/befund/befund-galileo-floor-stufen-te.md
-->
# Befund: Galileo-ODR-Uplink-Feldprüfung und SWSE4-Zweiter-Zeuge — das Galileo-ODR führt die sendende Station nicht (strukturelles Verdikt aus Kopf-Semantik und Byte-Zensus), die SWS4-ODR decken die frühen Anker nicht als Dateien

## Auftrag & Bindung

Richtung J der Deduktion-27-Arbeit stellt zwei Mess-Fragen an den
Open-Loop-Record:

**(a)** Trägt die Galileo-ODR-SFDU bei TRACKING_MODE=3 die Uplink-DSS
(sendende Station) — so wie es der Grail-RSS-SIS (`dpsis.htm`) für die
moderne RSR/EDR-SFDU beschreibt ("DSS Identifier for the uplink antenna
when TRACKING_MODE=3")? Der externe Recherche-Lead benennt dieses Feld als
**unverifiziert** für Galileo. Falls das Galileo-ODR es führt und bei
Dreiweg besetzt, wäre Ded-27 (Uplink/Downlink-Sitz der laut/ruhig-Floor-
Lautheit) direkt aus den Daten zu ziehen: (Empfänger, Sender) je Dreiweg-
Record, und die Lautheit dem Sender oder dem Empfänger zugeordnet.

**(b)** Der externe Recherche-Lead nennt `GO-SUN-RSS-1-ODR-V1.0` (SWSE4,
Kampagnen-Spanne "Solar Wind Scintillation 4, 1995-328 to 1996-013") als
open-loop-Zweiten-Zeugen für die frühen lauten Anker 1995-11-24 (st14,
Mode-1-Floor 25,8 Hz) und 1995-12-04/05/06 (st14/st43 Dreiweg-Zellen,
st14 1995-12-05 52,9 Hz), die GO-J/GO-JS-ODR (Beginn 1995-12-08) nicht
decken. Zu verifizieren: existieren SWSE4-Open-Loop-Dateien auf genau
diesen Anker-Tagen an den Trio-Stationen, und misst der open-loop
Trägerlinien-Ton auf dem lauten Tag anderes als auf dem ruhigen (Same-Day-
Split)?

Bindung wie die Vorlagen: die laut/ruhig-Floor-Zellen sind die gemessenen
Register-Zellen der Faden-Serie (resid, Boden −2560, laut = RMS ≥ 1 Hz);
ODR-Record = 2666 B = 166-B-Kopf (Wörter 1..83, 2 B/Wort) + 625×4 Kanäle
8-bit (1250 sps/Kanal, gemessen Wort 80). Neue Messung (a): Kopf-
Wort-Semantik aus dem formatgebenden Dokument des Volumens
(`DOCUMENT/RSC11_11.TXT`, das SIS des ODR-Datentyps) + empirischer
Byte-Zensus über die 166 Kopf-Bytes aller vorhandenen ODR-Dateien. Neue
Messung (b): Datei-Inventar des GO-SUN-ODR-Archivs (Annex-Aggregat
GORS_9100, INDEX 723 Zeilen) gegen die vier Anker-Tage; Sub-Volumen-
AAREADMEs (GORS_1032/1033 am PDS-Datenknoten) für die Volumen-Zeitspannen.

Proben (neu, `cargo check` je 0 Warnungen, RUSTFLAGS `-D warnings`):
`tools/measure/src/bin/galileo_odr_header_census_probe.rs` (a),
`tools/measure/src/bin/galileo_gosun_swse_coverage_probe.rs` (b). Reporte
`/tmp/opencode/galileo_odr_header_census_report.txt`,
`/tmp/opencode/galileo_gosun_swse_coverage_report.txt`.

## n zuerst (0 geehrt)

| Maß | n | Beleg |
|---|---|---|
| lokal vorhandene GO-J/GO-JS-ODR-Dateien, Kopf-Zensus | 10 | `data/pds-ppi.igpp.ucla.edu/galileo_goj_odr/` (st14/st43/st63, 421..21 001 Records je Datei, ≤ 3000 gestride-sampled) |
| davon mit einem zweiten Stations-Wert im 166-B-Kopf | 0 | Zensus: 0 geehrt (kein Byte trägt einen Trio-Wert ungleich dem Empfänger) |
| davon mit dokumentiertem TRACKING_MODE-Wort | 0 | Feldsemantik RSC-11-11, Wörter 1..83; 0 geehrt |
| GO-SUN-RSS-1-ODR-V1.0 (Annex-Aggregat GORS_9100) INDEX-Zeilen | 723 | `INDEX/INDEX.TAB`, Annex HTTP 200 (2026-09-06) |
| GO-SUN-ODR-Dateien auf 1995-11-24 (DOY 328) | 0 | Zensus |
| GO-SUN-ODR-Dateien auf 1995-12-04 (DOY 338) | 0 | Zensus |
| GO-SUN-ODR-Dateien auf 1995-12-05 (DOY 339) | 0 | Zensus |
| GO-SUN-ODR-Dateien auf 1995-12-06 (DOY 340) | 0 | Zensus |
| erste SWS4-Ära-ODR-Datei im Archiv | 1 | `53421615.ODR`, 1995-12-08T16:15:00Z |
| heruntergeladene SWSE4-Dateien | 0 | nichts vorhanden für die Anker-Tage, 0 geehrt |

## Messung Teil (a) — Kopf-Semantik und Byte-Zensus: kein Uplink-Feld, kein TRACKING_MODE

### 1. Feldsemantik des ODR-Kopfs (formatgebendes Dokument RSC-11-11)

Das Galileo-ODR ist der DSN-Radio-Science-Record der Ära (RSC-11-11B
Echtzeit-Blöcke / RSC-11-13 ODR-Tape; `DOCUMENT/RSC11_11.TXT` im Volumen,
"AAREADME: SIS for ODR data type"). Der Record-Kopf (166 B, Wörter 1..83)
ist dort lückenlos belegt. Die einzigen Identitäts-/Konfigurations-Wörter:

| Wort | Bytes | Feld (RSC-11-11) |
|---|---|---|
| 4 | 6/7 | Prime Front End Area (FEA) Number (z.B. 14, 43) / Secondary FEA Number |
| 5 | 8/9 | Spacecraft Number / Signal Processing Center (SPC) Designator (10, 40, 60, 21) |
| 6 | 10/11 | Jahr (Bits 1..7) / Day of Year (Bits 8..16) |
| 7/8 | 12..15 | Zeit des ersten Samples, ms past 0 h UTC |
| 26 | 52/53 | Antennen-RF-Konfiguration (Prime/CROSS/FAROT) + POCA-Rate |
| 80 | 158/159 | A-D-Converter-Sample-Rate |
| 83 | 164/165 | Sync-Daten / Signal-Select |

Das Tape-/Sitzungs-Label (Fig. 1) führt SCN/PASS/SPC und `Pxx Sxx`
(primäre/sekundäre Antenne) — ebenfalls Empfangs-Antennen. **Kein Wort der
Wörter 1..83 trägt einen TRACKING_MODE (1/2/3) und kein Wort eine
Uplink-/Sende-Stations-Identität.** Das ist ein struktureller Befund aus
dem formatgebenden Dokument selbst, kein Ersatz und keine Inferenz.

### 2. Byte-Zensus über 10 ODR-Dateien (gemessen)

Die Probe liest je Datei bis zu 3000 Records im Stride und zählt je
Kopf-Byte 0..165 die auftretenden Werte. Ergebnis über alle 10 Dateien
(Detail je Datei im Report):

- **byte 6 ist das einzige konstante Stations-Wort** und trägt in jeder
  Datei die Empfangs-Station (Wort-4-Prime-FEA): st43-Dateien 43, st63 63,
  st14 14 — deckungsgleich mit der LBL-Station je Datei.
- **byte 7 (Secondary FEA) = 0 in jedem Record** (keine zweite
  Empfangs-Antenne in diesen Pässen).
- Wort-5 byte 9 = SPC-Designator (10 für st14-Dateien, 40 für st43, 60 für
  st63) — folgt dem Empfangs-Zentrum, keine zweite Station.
- **kein konstantes Byte trägt einen Trio-Stationswert ungleich dem
  Empfänger.** Die einzigen konstanten Trio-Werte neben byte 6 sind
  byte 165 = 85 (0x55) in allen 10 Dateien (festes Bit-Muster des
  Wort-83-Trailers) und vereinzelte je-Datei-eingefrorene Bytes
  (byte 100 = 61/42/14, byte 107 = 14, byte 16/17 = 65/85 in je einer
  Datei) — je-Datei-Zufallskonstanten in Mess-/Statusfeldern, kein
  systematisches Sender-Feld.
- Byte-Subset-{1,2,3}-Koinzidenzen (z.B. byte 12/34/46/68/98/154 mit
  Werten {1,2,3} oder {2,3}) fallen auf die High-Bytes des ms-of-day-Zeit-
  Worts und anderer Zähler (Wort 6/7-Dekodierung bestätigt: DOY und Zeit
  in den gemessenen Dateien korrekt) — keine dokumentierten
  TRACKING_MODE-Felder.

### 3. Der Grail-RSR-Lead gilt für eine spätere Recorder-Generation

Die Grail-RSS-SIS (`dpsis.htm`, RSS-EDR-SFDU) definiert tatsächlich die
beiden Spalten TRACKING MODE (Werte 1/2/3) und "UPLINK DSS ID FOR 3-WAY
TRACKING": "DSS Identifier for the uplink antenna when TRACKING_MODE=3;
otherwise undefined … 14, 15, 25, 43, 45, 54, 63" — gemessen am Dokument.
Diese RSR/EDR-SFDU ist die Recorder-Generation nach dem Galileo-ODR
(Galileo-Floor-Ära 1995..1997 lief auf dem RSC-11-11/13-ODR); das
Galileo-ODR hat das Feld nicht, sein eigenes SIS definiert es nicht.

**Strukturelles Verdikt (a):** Die Galileo-ODR-SFDU trägt die Uplink-DSS
bei TRACKING_MODE=3 nicht — sie trägt weder TRACKING_MODE noch ein
zweites Stations-Feld; die einzige Stations-Identität im 166-B-Kopf ist
Wort 4 = die empfangende FEA/Station, und secondary FEA ist in allen
Records 0. Ein (Empfänger×Sender)-Split ist aus dem Galileo-ODR **per
Record nicht ziehbar**; Ded-27 bleibt aus dem ODR strukturell nicht direkt
schließbar — dasselbe Klassen-Verdikt wie für den TRK-2-25-Roh-Record
(Dreiweg-Befund), jetzt von der ODR-Kopf-Seite gemessen (n = 10 Dateien;
zweites Stations-Wort n = 0, TRACKING_MODE-Wort n = 0, 0 geehrt). Kein
Ersatz, keine erfundene Uplink-Zuordnung.

## Messung Teil (b) — SWSE4-Dateiabdeckung der frühen Anker: n = 0

### 1. Die SWS4-Kampagnen-Spanne ist nicht die Dateiabdeckung

Der Annex des PDS-PPI führt das GO-SUN-ODR als Aggregat GORS_9100
(`GO-SUN-RSS-1-ODR-V1.0/INDEX/INDEX.TAB`, 723 Zeilen, HTTP 200). Die
AAREADME-Titelzeile nennt die Kampagnen: "Solar Wind Scintillation 4
(1995-328 to 1996-013)" — das ist die **Kampagnen-Spanne**, nicht die
archivierte Datei-Spanne. Die Sub-Volumen-Tabelle der SWS4-Volumina
(GORS_1032/1033-AAREADME, pds.nasa.gov) gibt die tatsächlichen
Datei-Zeitspannen: GORS_1032 = 1995-342T22:43 .. 1995-347, GORS_1033 =
1995-348 .. 1995-358; im Annex-Aggregat beginnt die SWS4-Ära-ODR-Reihe mit
`53421615.ODR`, Start 1995-12-08T16:15:00Z (DOY 342).

### 2. Zensus gegen die vier Anker-Tage (Probe b)

Die Coverage-Probe liest den Archiv-INDEX (723 Zeilen) und zählt je
UTC-Tag die ODR-Dateien im Fenster 1995-11-20..1996-01-20:

- **1995-11-24 (DOY 328): n = 0**
- **1995-12-04 (DOY 338): n = 0**
- **1995-12-05 (DOY 339): n = 0**
- **1995-12-06 (DOY 340): n = 0**

Erste Datei der Ära im Archiv ist `53421615.ODR` (1995-12-08T16:15:00Z);
davor liegt im GO-SUN-ODR-Archiv keine Datei. Die geschlossene-Schleife-
TDFs der Anker-Tage (5327328A.TDF 11-23/24, 5337339A.TDF 12-04/05,
5340341A.TDF 12-07, GO-SUN-RSS-1-TDF-V1.0) existieren und tragen die
Lautheit — aber sie sind closed-loop und ohne Sender-Feld (Dreiweg-
Befund). GO-J/GO-JS-ODR beginnen ebenfalls erst 1995-12-08.

**Verdikt (b):** SWSE4-Open-Loop-Dateien für 1995-11-24 und
1995-12-04/05/06 existieren im GO-SUN-ODR-Archiv **nicht** (n = 0 je Tag,
gemessen am 723-Zeilen-INDEX und an den SWS4-Volumen-Tabellen). Der
externe Lead hat die SWS4-Kampagnen-Spanne (95-328..96-013) als
Dateiabdeckung gelesen; gemessen ist die Dateiabdeckung ab 1995-12-08. Ein
Same-Day-laut/ruhig-Open-Loop-Split auf den Anker-Tagen ist damit **nicht
stellbar** (0 geehrt — kein Same-Day-Split, keine open-loop-Messung der
frühen lauten Anker). Die frühen Anker 1995-11-24 (st14, 25,8 Hz) und
1995-12-04/05/06 (st14/st43 Dreiweg, st14 52,9 Hz am 12-05) bleiben ohne
zeitgleichen open-loop-Zeugen; die Ded-27-/Ded-31-Fragen auf diesen Tagen
bleiben `pending` — mit der gemessenen Ursache benannt (kein ODR im
Archiv), nicht ersetzt und nicht geglättet. Die erste archivierte
SWS4-Ära-ODR (1995-12-08 16:15) und ihr Fenster sind benannt (für eine
spätere Anschluss-Messung ab DOY 342), nicht als Anker-Zeuge gezählt.

## Grenzen

- Teil (a): Der Kopf-Zensus ist über die 10 lokal vorhandenen GO-J/GO-JS-
  ODR-Dateien gemessen (Floor-Ära-Ausschnitt, 1996-11-08..1997-02-27,
  Stationen 14/43/63). Die Feldsemantik stammt aus dem formatgebenden
  Dokument des Volumens (RSC-11-11); die empirischen Dekodierungen (Wort
  4 = FEA, Wort 6 = Jahr/DOY, Wort 7-8 = Zeit, Wort 80 = 1250 sps) stützen
  die Wort-Zuordnung an den echten Daten. Die GO-SUN-ODR derselben Ära
  nutzen dasselbe RSC-11-11-Format (dasselbe DOCUMENT im GO-SUN-Volumen);
  ihr Kopf wurde nicht separat gezählt (keine Datei auf den Anker-Tagen —
  0 geehrt).
- Teil (b): Die Aussage "keine Datei auf den Anker-Tagen" ist über den
  Archiv-INDEX (723 Zeilen des Annex-Aggregats) und die Sub-Volumen-
  AAREADME-Tabellen (GORS_1032/1033) gemessen; die Sub-Volumen-Zuordnung
  einzelner Dateien des Aggregats zu Original-Volumina ist dafür nicht
  nötig — der Archiv-INDEX ist der Datei-Manifest. Original-Bänder der
  SWS4-Kampagne vor DOY 342 (falls je bespielt) sind im PDS-Archiv nicht
  als ODR-Dateien manifestiert — benannt, nicht behauptet.
- Die Verifikations-Belege liegen maschinenlokal unter
  `data/pds-ppi.igpp.ucla.edu/galileo_gosun_swse/` (INDEX.TAB, INDEX.LBL,
  GORS_1033-AAREADME, sha256 + Provenienz dort); der ODR-Datenbestand der
  GO-J/GO-JS liegt unverändert unter `galileo_goj_odr/`.

## Registrierung (Session-Duty)

Keine `phi/sources.φ`-Registrierung: Es wurde **kein neuer Datensatz
beschafft** — die vier Anker-Tage tragen im GO-SUN-ODR-Archiv keine Datei
(n = 0, 0 geehrt), der Mess-Befund ist ein Feld-/Leere-Prüfbefund über ein
bereits registriertes öffentliches PDS3-Volumen; die Kopie des
Archiv-INDEX und der GORS_1033-AAREADME als Beleg ist ein Ausschnitt des
bestehenden Volumens (gitignored, sha256 in `galileo_gosun_swse/`). Nach
der Resid-Praezedenz des Repos (abgeleitete Galileo-Assets leben nicht in
`phi/sources.φ`; Registrierung = Workflow + CDN-Release) gilt auch für
jede spätere ODR-Ernte die bestehende Zuständigkeitslinie der
`galileo-*-cdn.yml`-Workflows. `.github/workflows/`, `phi/` und `docs/`
fremde Dateien wurden nicht angefasst.

## Register-Satz

*Die Galileo-ODR-SFDU trägt die Uplink-DSS nicht (Richtung J, Teil a):
über den formatgebenden Wort-Plan des ODR-Datentyps (RSC-11-11,
Wörter 1..83 des 166-B-Kopfs: Wort 4 Prime/Secondary FEA = Empfangs-
Antenne, Wort 5 SPC-Designator, keine TRACKING_MODE-/Uplink-Spalte) und
den Byte-Zensus über 10 lokale GO-J/GO-JS-ODR-Dateien (421..21 001
Records, ≤ 3000 gestride) ist byte 6 das einzige Stations-Wort (= FEA des
Empfängers, st14/43/63 je Datei deckungsgleich mit der LBL), byte 7
Secondary FEA = 0 in allen Records, kein konstantes oder systematisches
Byte trägt einen zweiten Stationswert (byte 165 = 0x55 über alle Dateien
ist ein festes Trailer-Muster; vereinzelte eingefrorene Bytes je Datei
sind Zufallskonstanten), kein dokumentiertes TRACKING_MODE-Wort — das
Uplink-DSS-Feld der Grail-RSS-SFDU ("UPLINK DSS ID FOR 3-WAY TRACKING …
when TRACKING_MODE=3") ist eine spätere Recorder-Generation; Ded-27 ist
aus dem Galileo-ODR per Record nicht schließbar, strukturelles Verdikt wie
für TRK-2-25, kein Ersatz. Die SWS4-Open-Loop-Zeugen der frühen lauten
Anker existieren als Dateien nicht (Teil b): GO-SUN-RSS-1-ODR-V1.0
(Annex-Aggregat GORS_9100, INDEX 723 Zeilen) hat auf 1995-11-24,
1995-12-04/05/06 je n = 0 ODR-Dateien (0 geehrt), die SWS4-Kampagnen-
Spanne "1995-328..1996-013" der AAREADME ist nicht die Dateiabdeckung
(GORS_1032 = 342..347, GORS_1033 = 348..358; erste archivierte ODR =
53421615.ODR, 1995-12-08T16:15:00Z) — kein Same-Day-laut/ruhig-Split auf
den Anker-Tagen stellbar, die frühen Anker bleiben ohne open-loop-Zeugen,
die Ded-27-/Ded-31-Frage auf diesen Tagen bleibt mit gemessener Ursache
pending; keine phi/sources.φ-Registrierung (kein neuer Datensatz, nur
Feld-/Leere-Prüfung; Beleg-INDEX gitignored unter
data/…/galileo_gosun_swse/ mit sha256).*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; eine Registerzeile ergänzt
die Haupt-Session). Proben committet:
`tools/measure/src/bin/galileo_odr_header_census_probe.rs` und
`tools/measure/src/bin/galileo_gosun_swse_coverage_probe.rs`
(`cargo check -p omegaflow-measure --bin …`, RUSTFLAGS `-D warnings`,
0/0). Reporte `/tmp/opencode/galileo_odr_header_census_report.txt` und
`/tmp/opencode/galileo_gosun_swse_coverage_report.txt`.
