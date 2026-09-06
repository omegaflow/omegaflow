<!--
  title: Befund — Beschaffung 2 (Ded-31-Zweiter-Zeuge): GO-J-RSS-1-ODR-V1.0 + GO-JS-RSS-1-ODR-V1.0 sind der zeitgleiche unabhängige open-loop Receiver-Record der Floor-Ära (10 ODR-Dateien, 7 UTC-Tage, sha256 belegt) — 3 ODR-Windows decken eine wirklich laute Closed-Loop-Phase (st14 1997-02-26 Window-RMS 84 Hz), der unabhängige Trägerlinien-Ton ist dort schmal und stark (segsnr 113.9; laut-Phasen-Fenster 70.4–113.9 gegen ruhige 25.5–128.3, vollständig überlappend): ein erstes gemessenes Nein zur Open-Loop-Lautheits-Signatur, das GWE-n=0 des Vorgängers war ein Volumen-Auswahlfehler
  class: befund
  date: 2026-09-06
  sha256: 4a86a15dd6909993830301e70b0ffa8aca448b57bd6f91d912b1b94c6c979aec
  status: draft
  antwortet-auf: docs/befund/befund-galileo-gwe-odr-zweiter-zeuge.md
  see-also: docs/befund/befund-galileo-gwe-bestand.md docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-receiver-je-pass-floor.md docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-doppler-odf-beschaffung3.md
-->
# Befund: Beschaffung 2 — der zeitgleiche Open-Loop-Record der Floor-Ära existiert und ist beschafft (GO-J-RSS-1-ODR-V1.0 / GO-JS-RSS-1-ODR-V1.0, 10 ODR-Dateien, sha256 belegt): 16 ODR-Window-überlappende Register-Zellen (11 mit in-window-Samples, 5 laut / 6 ruhig), 3 ODR-Windows decken eine wirklich laute Phase (Window-RMS 84 Hz an st14 1997-02-26) — der unabhängige Trägerlinien-Ton ist an laut und ruhig schmal und stark (laut-Phasen-Fenster segsnr 70.4–113.9 gegen ruhige 25.5–128.3, vollständig überlappend; Breite 0.05–0.20 Hz; laute Phase st14 113.9 gegen ruhige Phase desselben Tages 128.3): ein erstes gemessenes Nein zur Open-Loop-Lautheits-Signatur (n = 10 Dateien / 3 laut-Phasen-Fenster / 1 Zwei-Phasen-Split, ehrlich benannt)

## Auftrag & Bindung

Der Ded-31-Zweiter-Zeuge-Test fragt, ob die station-gebundene laut/ruhig-Floor-Lautheit der Galileo-Gipfel (Register: 207 laut / 105 Flips über 400 robuste Zellen, Anker 1995-11-24 st14 laut / 1996-06-26 st63 laut) upstream-real ist — ein unabhängiger (open-loop / höher aufgelöster) Receiver-Record misst auf den laut-Tagen anderes/lauteres Verhalten — oder ein Reduktions-Artefakt der geschlossenen Schleife. Der Vorgänger-Lauf (`befund-galileo-gwe-odr-zweiter-zeuge.md`) besorgte nur das GWE-ODR (`GO-X-RSS-1-ODR-V1.0`), dessen Fenster 1995-06-28 endet: n = 0 Überlappung, 148 Tage vor dem ersten Floor-Tag. Dieser Lauf (Beschaffung 2 von 3) besorgt den **zeitgleichen** unabhängigen Record der Floor-Ära 1995-11-23..1997-02-28.

Der Irrtum des Vorgängers ist gemessen: er prüfte nur das GWE-Volumen. Die Annex-Wurzel des PDS-PPI (UCLA) führt **zwei Orbital-Ära-ODR-Volumina**, die der Bestands-Befund nicht als Floor-Ära-Datenbestand erfasst hatte: `GO-J-RSS-1-ODR-V1.0` (GORS_9120, AAREADME „Jupiter Occultation Measurement Experiment", Fenster 1995-12-08..1997-11-16) und `GO-JS-RSS-1-ODR-V1.0` (GORS_1301-1304, Satelliten-Okkultation). Beide tragen echte open-loop ODR des Radio-Science-Systems: 2666-Byte-Records = 166-B-Kopf + 625 × (AD1..AD4) 8-bit, Abtastraten-Feld 1250 sps/Kanal (gemessen, Kopf-Byte 159-160), Station je Datei im LBL (14/43/63). Die `.ODR`-Dateien im Floor-Fenster an den Trio-Stationen sind der gesuchte zeitgleiche unabhängige Receiver-Record (open-loop, 1250 sps gegen den TDF der geschlossenen Schleife).

## n zuerst (0 geehrt) — der beschaffte Bestand

| Maß | n | Beleg |
|---|---|---|
| Annex-Wurzel `https://pds-ppi.igpp.ucla.edu/annex/` | HTTP 200 | Listing (2026-09-06) |
| `GO-J-RSS-1-ODR-V1.0/` + `GO-JS-RSS-1-ODR-V1.0/` | je HTTP 200 | Band-Wurzeln, AAREADME/INDEX |
| heruntergeladene `.ODR` | **10** | `data/galileo_goj_odr/`, sha256 + Größe je Datei |
| abgedeckte UTC-Tage (Floor-Ära) | 7 | 1996-11-08, 12-19, 12-21, 12-22, 1997-02-25, 02-26, 02-27 |
| ODR-Window-überlappende Register-Zellen | 16 | Overlap-Probe |
| davon mit in-window-Samples | 11 (5 laut / 6 ruhig) | dito |
| ODR-Windows, die eine wirklich laute Phase decken (in-window-RMS ≥ 1 Hz) | 3 | 1996-11-08 st43 m2 (n 25, dünn), 1996-12-21 st14 m1 (n 378), 1997-02-26 st14 m1 GOJS (n 3576, 84.4 Hz) |

Dateien (je HTTP 200, sha256 in `data/galileo_goj_odr/sha256.txt`): GOJ 63131033 (st43 1996-11-08, 1 122 386 B), 63131742 (st63 1996-11-08, 12 791 468 B), 63561707 (st14 1996-12-21, 40 952 426 B), 63570045 (st43 1996-12-22, 41 594 932 B), 70571807 (st14 1997-02-26, 49 270 346 B), 70571825 (st43 1997-02-26, 55 988 666 B), 70580900 (st63 1997-02-27, 36 962 304 B); GOJS 63540659 (st43 1996-12-19, 13 439 306 B), 70561433 (st14 1997-02-25, 30 395 066 B), 70571407 (st14 1997-02-26, 29 755 226 B). — Größen sind die vollständigen Server-Antworten (Content-Length gemessen).
sha256-Manifest (Provenienz, auch als `data/galileo_goj_odr/sha256.txt`):

| Datei | Station / Tag | Größe (B) | sha256 |
|---|---|---|---|
| 63131033.ODR | st43 1996-11-08 | 1 122 386 | 4f976d45f5184a3394e7ec863efdf48b1f0747538a8bdec663aa6723318e7aa1 |
| 63131742.ODR | st63 1996-11-08 | 12 791 468 | 1edb63f1d5b39b7a5a80982742d382bfbcd6413d8e549eebd9ab4f7e458ad975 |
| 63561707.ODR | st14 1996-12-21 | 40 952 426 | 72904a159e53265d7c1db7e8c196ceb5066ce89262c8aea0f4508843ca7f91eb |
| 63570045.ODR | st43 1996-12-22 | 41 594 932 | 7aad817520a1f83416925b3ae4c7712b02d7bec2148602da43d7a3952a793857 |
| 70571807.ODR | st14 1997-02-26 | 49 270 346 | 8b91928291df989c2ab7d48cd202d43d955710c5c1911b40ff6ca2e526e34433 |
| 70571825.ODR | st43 1997-02-26 | 55 988 666 | fd319f30898dcffe0d2e61270cf298bb9adba6f4b695bf1928ea66703a64255c |
| 70580900.ODR | st63 1997-02-27 | 36 962 304 | 362d3d00d614f5c5e748d870f5a69846ed03b3e32df046976d5d8f8f08150dcd |
| JS_63540659.ODR | st43 1996-12-19 | 13 439 306 | aed589e47d1078296ea4f36373fb577608320722516917aa56713c513ef330c7 |
| JS_70561433.ODR | st14 1997-02-25 | 30 395 066 | a16fa7b38d060f4b0dc53327bbf1824ab2a4cef198f09e4f57dc78d5b21772a9 |
| JS_70571407.ODR | st14 1997-02-26 | 29 755 226 | d35f50efeada76741335c784d0f9ab417be23e91f95a9bd5959f3ccbdab531be |


## Messung 1 — Abdeckung: die ODR-Windows liegen auf Floor-Zellen, teils in der lauten Phase

Die Probe `galileo_odr_floor_overlap_probe` (committet, `cargo check` 0/0) kreuzt jedes ODR-Window gegen die Floor-Zellen aus `data/galileo_resid.bin` (kanonischer Rund-Tag-Schlüssel, Modi 1–3, Trio-Stationen, Boden = Stärke −2560, Lock |resid| > 1000 Hz ausgeschlossen) und misst je Zelle die Floor-Samples **innerhalb** des ODR-Windows samt deren RMS (vollständige Tabelle im Report `/tmp/opencode/galileo_odr_floor_overlap_report.txt`):

| ODR-Window | Register-Zelle (Tag st Mode) | Tages-RMS/n | Klasse | Window-n | Window-RMS |
|---|---|---|---|---|---|
| GOJ 1996-11-08 st43 10:33-10:37 | m2 | 11.860 / 33 058 | laut | 25 | **326.8 Hz** |
| GOJ 1996-11-08 st63 17:42-18:22 | m1 | 0.062 / 26 209 | ruhig | 2400 | 0.020 Hz |
| GOJS 1996-12-19 st43 06:59-07:41 | m1 | 4.806 / 10 114 | laut | 540 | 0.098 Hz |
| GOJS 1996-12-19 st43 06:59-07:41 | m2 | 0.136 / 30 721 | ruhig | 236 | 1.276 Hz |
| GOJ 1996-12-21 st14 17:07-19:15 | m1 | 4.097 / 23 454 | laut | 378 | **4.258 Hz** |
| GOJ 1996-12-22 st43 00:45-02:55 | m1 | 0.028 / 3 942 | ruhig | 3942 | 0.028 Hz |
| GOJS 1997-02-25 st14 14:33-16:08 | m1 | 0.040 / 7 177 | ruhig | 5141 | 0.026 Hz |
| GOJ 1997-02-26 st14 18:07-20:41 | m1 | 29.145 / 35 536 | laut | 10 665 | 0.022 Hz |
| GOJS 1997-02-26 st14 14:07-15:40 | m1 | 29.145 / 35 536 | laut | 3576 | **84.416 Hz** |
| GOJ 1997-02-26 st43 18:25-21:20 | m1 | 0.022 / 43 293 | ruhig | 10 823 | 0.024 Hz |
| GOJ 1997-02-27 st63 09:00-12:20 | m1 | 38.483 / 7 913 | laut | 1208 | 0.161 Hz |

Drei ODR-Windows decken eine **wirklich laute Phase** ab (in-window-RMS ≥ 1 Hz): GOJ st43 1996-11-08 m2 (nur 25 Samples, dünn), GOJ st14 1996-12-21 m1 (n 378, 4.26 Hz), GOJS st14 1997-02-26 m1 (n 3576, 84.4 Hz — die lauteste überlappte Phase). Die laut-Tage st14 1997-02-26 im GOJ-Window 18-20h und st63 1997-02-27 liegen mit ihren ODR-Windows in **ruhigen Phasen** der laut-Tage (Window-RMS 0.022/0.161 Hz): 0 geehrt — diese Windows sagen über die laute Phase nichts.

## Messung 2 — der unabhängige Open-Loop-Linien-Ton (segsnr)

Die Probe `galileo_odr_openloop_probe` (committet, `cargo check` 0/0) dekodiert die ODR-Records, bildet 8192-Sample-Segmente (6.55 s) und misst je Segment das Verhältnis der stärksten Spektrallinie > 2 Hz zum Median (segsnr, verstärkungs-robust); angegeben der Median über die Segmente im ODR-Window:

| Datei | Zelle / Klasse | segsnr med | p10–p90 |
|---|---|---|---|
| GOJ 63131033 | st43 1996-11-08 m2 laut (Window-RMS 327) | 93.7 | 13.7–111.8 |
| GOJ 63131742 | st63 1996-11-08 ruhig | 67.5 | 12.2–97.8 |
| GOJS 63540659 | st43 1996-12-19 laut (Window-RMS 0.10) | 98.4 | 14.6–127.0 |
| GOJ 63561707 | st14 1996-12-21 laut (Window-RMS 4.26) | 70.4 | 11.9–104.4 |
| GOJ 63570045 | st43 1996-12-22 ruhig | 63.3 | 13.2–88.3 |
| GOJS 70561433 | st14 1997-02-25 ruhig | 125.0 | 14.0–161.6 |
| GOJS 70571407 | st14 1997-02-26 laut (Window-RMS 84.4) | 113.9 | 14.3–146.3 |
| GOJ 70571807 | st14 1997-02-26 laut-Tag, ruhige Phase | 128.3 | 12.4–165.8 |
| GOJ 70571825 | st43 1997-02-26 ruhig | 80.5 | 12.9–130.0 |
| GOJ 70580900 | st63 1997-02-27 laut-Tag, ruhige Phase | 25.5 | 12.1–105.2 |

Die drei laut-Phasen-Windows (in-window Closed-Loop-RMS ≥ 1 Hz) messen segsnr 93.7 (st43 1996-11-08, 327 Hz, dünn n = 25), 70.4 (st14 1996-12-21, 4.26 Hz) und **113.9** (st14 1997-02-26 GOJS, 84 Hz) — sie liegen vollständig innerhalb der Spanne der ruhigen Fenster (25.5–128.3). Zwei-Phasen-Split derselben Station und desselben Tages (st14, 1997-02-26): laute Phase (GOJS 14-16h, Window-RMS 84 Hz) segsnr 113.9 gegen ruhige Phase desselben laut-Tages (GOJ 18-20h, Window-RMS 0.022 Hz) segsnr 128.3 — kein Einbruch in der lauten Phase. Linienbreite (Halbwertsbreite in 20-s-Fenstern): st14 laut-Phase 0.10 Hz, st14 ruhig 0.05–0.10 Hz, st14 ruhiger Tag 0.20 Hz — schmal in beiden Zuständen. Der reine Stations-Vergleich im selben Uhrzeit-Fenster (18:25–20:41, je 16 272 Records) liest st14 (laut-Tag, aber ruhige Phase) 127.2 gegen st43 (ruhiger Tag) 93.6: zwei ruhige Fenster, kein laut/ruhig-Test.

## Verdikt (erste Messung, ehrlich benannt)

**Der zeitgleiche unabhängige Open-Loop-Record der Floor-Ära existiert und ist beschafft** (Hauptlieferung). Auf den überlappenden (Tag, Station) misst er auf den laut-Tagen **kein** anderes/lauteres Verhalten als auf den ruhigen: wo ein ODR-Window wirklich in eine laute Closed-Loop-Phase fällt (st14 1997-02-26 GOJS, Window-RMS 84 Hz; st14 1996-12-21, 4.26 Hz), trägt der Open-Loop-Linien-Ton eine schmale starke Linie (segsnr 113.9 bzw. 70.4, Breite 0.10 Hz) — die laut-Phasen-Fenster messen 70.4–113.9, die ruhigen Fenster 25.5–128.3: vollständig überlappend, Breite 0.05–0.20 Hz in beiden Zuständen. Die laut-Phasen-Fenster (n = 3) liegen mit segsnr 70.4–113.9 vollständig innerhalb der Spanne der ruhigen Fenster (25.5–128.3); die lauteste überlappte Closed-Loop-Phase (st14 1997-02-26, 84 Hz) trägt segsnr 113.9 gegen 128.3 in der ruhigen Phase desselben Tages. Der reine Stations-Vergleich im selben Uhrzeit-Fenster (18:25–20:41) liest st14 (laut-Tag, aber ruhige Phase) 127.2 gegen st43 (ruhiger Tag) 93.6 — zwei ruhige Fenster, kein laut/ruhig-Zeuge; als solcher benannt, nicht als Zeuge der lauten Phase gezählt. **Die Floor-Lautheit erscheint im unabhängigen, höher aufgelösten Receiver-Record nicht als empfangene Spektral-Störung** — ein erstes gemessenes Nein zur Open-Loop-Lautheits-Signatur, konsistent mit einem Sitz der Lautheit in der geschlossenen Schleife / im per-Pass-Empfangs-Zustand, nicht im empfangenen Träger.

**n ehrlich:** 10 ODR-Dateien über 7 UTC-Tage; 11 Register-Zellen mit in-window-Samples (5 laut / 6 ruhig); nur 3 Windows decken eine wirklich laute Phase (1 dünn n = 25, 2 robust n = 378 / n = 3576); der Zwei-Phasen-Split derselben Station und desselben Tages ist n = 1 Tag (st14 1997-02-26), die laut-Phasen-Fenster sind n = 3 (1 dünn n = 25). Ein endgültiges Ded-31-Verdikt braucht mehr Same-Day-Splits oder die volle Spektral-Reduktion über die langen ODR-Serien; die gemessene Richtung ist als erste Messung benannt, nicht überzogen. 0 geehrt für die laut-Tage, deren ODR-Windows nur ruhige Phasen decken (st14 1997-02-26 GOJ 18-20h, st63 1997-02-27): deren Windows sagen über die laute Phase nichts — benannt, nicht als Negativ gezählt.

## Grenzen

- Der segsnr ist eine erste unabhängige Größe (Linien-Stärke über Rauschen); eine volle Spektral-Reduktion (Linienform, Phasenrauschen, Seitenbänder über die Zeit) ist nicht gezogen. Alle 4 AD-Kanäle tragen dieselbe Linie (gleiche segsnr je Datei, gemessen) — die Kanäle sind parallele Abbilder desselben Empfangs; eine Kanal-Zuordnung je Experiment (welche AD den Träger trägt) ist aus dem LBL nicht je Datei gezogen.
- Auswahl: alle GO-J/GO-JS-INDEX-Zeilen im Floor-Fenster, deren (Tag, Station) eine Floor-Zelle mit ODR-Window-Überlapp trägt; das GO-J-Volumen reicht bis 1997-11-16, aber die Floor-Ära endet 1997-02-28 — die Ära ist über die 7 geholten Tage abgedeckt. `GO-SUN-RSS-1-ODR-V1.0` (Solarwind, 566-B-Records) ist ein drittes zeitgleiches ODR-Volumen (HTTP 200, hier nicht geholt, eigener Fokus).
- Datei 70580900.ODR: geholte Datei = 13 864 volle Records + 880 B (Content-Length 36 962 304); INDEX/LBL nennen 24 002 Records bis 12:20, die Datei endet bei ~10:55 (Archiv-interne Label-Diskrepanz, gemessen, benannt, nicht geglättet). Ihr Window deckt ohnehin nur die ruhige Phase des laut-Tages.
- Daten liegen maschinenlokal unter `data/galileo_goj_odr/` (gitignored); Provenienz + sha256 dort (`PROVENANCE.txt`, `sha256.txt`) und in diesem Blatt. Dauerhaftes Zuhause ist der PDS-Annex (URLs oben, HTTP 200 gemessen).

## Registrierung (Session-Duty)

Keine `phi/sources.φ`-Registrierung: beschafft sind Ausschnitte **bestehender öffentlicher PDS3-Volumina** (`GO-J-RSS-1-ODR-V1.0`, `GO-JS-RSS-1-ODR-V1.0` am PDS-PPI-Annex), kein neuer Feed/kein neuer Endpoint; die CI-Manifestation der Galileo-Daten läuft über die bestehenden Dedicated-CD-Workflows (`galileo-*-cdn.yml`), deren Zuständigkeit für die ODR-Volumina die Haupt-Session benennt (Register-Pflicht hier vermerkt, `.github/workflows/` nicht angefasst). Die 10 ODR-Dateien sind ein mess-begrenzter Ausschnitt für den Ded-31-Witness-Test (Provenienz + sha256 in diesem Blatt und in `data/galileo_goj_odr/`).

## Register-Satz

*Die zeitgleiche unabhängige (open-loop, 1250 sps) Aufzeichnung der Floor-Ära 1995-11-23..1997-02-28 existiert und ist beschafft: der GWE-zentrierte Vorgänger (n = 0) prüfte nur GO-X; die Annex-Wurzel des PDS-PPI führt die Orbital-Ära-ODR-Volumina GO-J-RSS-1-ODR-V1.0 (GORS_9120, 1995-12-08..1997-11-16) und GO-JS-RSS-1-ODR-V1.0 (GORS_1301-1304) — 10 .ODR-Dateien (je HTTP 200, sha256 + Größe belegt, data/galileo_goj_odr/) decken 7 UTC-Tage mit 16 ODR-Window-überlappenden Register-Zellen (11 mit in-window-Samples: 5 laut / 6 ruhig), davon decken 3 ODR-Windows eine wirklich laute Phase (st14 1997-02-26 Window-RMS 84 Hz n 3576, st14 1996-12-21 4.26 Hz n 378, st43 1996-11-08 327 Hz n 25 dünn); der unabhängige Trägerlinien-Ton (segsnr-Median, Breite) misst auf den laut-Phasen schmale starke Linien (segsnr 70.4–113.9, Breite 0.10 Hz) — die ruhigen Fenster messen 25.5–128.3: vollständig überlappend, 0.05–0.20 Hz in beiden Zuständen, der die laut-Phasen-Fenster messen segsnr 70.4–113.9 (Spanne der ruhigen Fenster 25.5–128.3) — ein erstes gemessenes Nein zur Open-Loop-Lautheits-Signatur (n = 10 Dateien / 1 Same-Day-Split / 1 Zwei-Phasen-Split, ehrlich benannt; 0 geehrt wo das Window nur ruhige Phasen deckt); ein endgültiges Ded-31-Verdikt braucht mehr Same-Day-Splits oder die volle Spektral-Reduktion; keine sources.φ-Registrierung (bestehende PDS-Volumina, CI über bestehende galileo-*-cdn-Workflows, ODR-Zuständigkeit an die Haupt-Session benannt).*
