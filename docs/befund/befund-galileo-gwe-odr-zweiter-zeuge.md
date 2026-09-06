<!--
  title: Befund — GWE-ODR als zweiter Zeuge der station-gebundenen Floor-Lautheit: kein überlappender (Tag, Station) mit den laut/ruhig-Tagen — Ded-31-Test n-leer (0 geehrt), Resid→ODR-Transfer auf identischen Kontakten nicht prüfbar
  class: befund
  date: 2026-09-06
  sha256: 17f178b786982472debd8c3eb8a4f3113fb91dd2a2e3bf2b4e6e85892495e844
  status: draft
  antwortet-auf: docs/befund/befund-galileo-gwe-odr-banden-check.md docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-floor-stufen-te.md
  see-also: docs/befund/befund-galileo-gwe-bestand.md docs/befund/befund-galileo-receiver-je-pass-floor.md docs/befund/befund-galileo-dreiweg-sendempfang-floor.md
-->

# Befund: GWE-ODR als zweiter Zeuge der Floor-Lautheit — kein (Tag, Station)-Überlapp (n = 0, 0 geehrt)

## Frage & Bindung

Zwei offene Fragen, beide schon benannt, beide von Agent 3 (diesem Blatt) auf
das GWE-ODR (`GO-X-RSS-1-ODR-V1.0`, PDS3 `GORS_9110`, PPI/UCLA) gerichtet:

1. **Ded-31-Stil (dritter Zeuge upstream):** Ist die tages-scharfe
   laut/ruhig-Floor-Lautheit der Galileo-Gipfel ein real empfangenes
   Signal-/Ketten-Phänomen, das auch im unabhängigen open-loop-Record auftritt —
   oder ein Artefakt der geschlossenen Schleifen-Reduktion? Das ODR ist
   unabhängig von der Closed-Loop-Doppler-Reduktion: open-loop Amplituden-/
   Momentanfrequenz-Aufzeichnung der drei 70-m-Stationen, kein Residuum.
2. **Resid→ODR-Transfer** (die offene Frage des Banden-Check-Blattes,
   injiziert-kalibriert, nicht blind): Prägt die Lautheit/Struktur der lauten
   Tage aus den Residuen auf das ODR desselben Kontakts?

Bindung des Tests: ein Zweiter-Zeuge-Verdikt braucht überlappende Tage — ein
(UTC-Tag, Station), an dem ODR und Closed-Loop-Bestand denselben
Galileo-Kontakt tragen. Der Test ist: misst das ODR auf den laut-Tagen des
Bestands ein anderes (lauteres) Verhalten als auf den ruhigen — ja → die
Lautheit ist upstream/real; nein → Reduktions-Artefakt. Zuerst wird die
Abdeckung gemessen, dann die Überlappung, dann der Transfer; eine fehlende
Überlappung wird als solche benannt (0 geehrt), nicht gefüllt.

Probe `tools/measure/src/bin/galileo_odr_overlap_witness.rs` (neu, einzige
Repo-Änderung außer diesem Blatt; `cargo check` 0/0, RUSTFLAGS `-D warnings`),
Report `/tmp/opencode/galileo_odr_overlap_witness_report.txt`. Zell- und
Klassen-Konvention unverändert zur Referenz-Linie (Boden = Stärke exakt −2560,
laut = Zell-RMS ≥ 1 Hz, robust n ≥ 30, Lock |resid| > 1000 Hz vor dem Rauschen
getrennt, Stärke 0 nie klassiert, TDB-Tag-Zelle mit Mittags-Konvention).
ODR-Abdeckung aus `INDEX/INDEX.TAB` (27 612 B, HTTP 206) frisch gezogen und
lokal geparst; die Stichprobe ist begrenzt (2 × 500 Records, nicht die
4,5 GB).

## n zuerst (0 geehrt)

| Maß | n | Wert |
|---|---|---|
| INDEX-Segmente (auswertbar, Start < Stopp) | 244 | von 251 INDEX-Zeilen; 7 Anomalie-Zeilen (Start > Stopp) ausgelassen |
| ODR-Abdeckung (UTC) | 79 Tage | 1994-04-28T14:16:19Z … 1995-06-28T19:04:59Z |
| ODR-Tage ab Floor-Ära-Beginn 1995-11-23 | 0 | 0 geehrt |
| ODR-Stichprobe (Dateien) | 2 | `41181416.ODR`, `51780610.ODR`, je 500 Records à 566 B, HTTP 206 |
| Trio-Floor-Zellen robust (n ≥ 30), Stations 14/43/63 | 400 | über die ganze Boden-Ära |
| davon laut (Register) | 207 | M1 32+35+34 · M2 17+16+22 · M3 25+16+10 |
| überlappende laut-Tage (ODR-Fenster ∩ Register) | 0 | 0 geehrt |
| Floor-Zellen im ODR-Fenster überhaupt | 1 | M1 st24 1994-12-18, ruhig, Nicht-ODR-Station |
| Kalender-Lücke letzter ODR-Tag → frühester Trio-Floor-Tag | — | 148 Tage |

## Messung 1 — Annex-Reachability und ODR-Abdeckung

Annex-Reachability (frisch gemessen, 2026-09-06): `AAREADME.TXT`,
`INDEX/INDEX.TAB`, `CATALOG/DATASET.CAT`, `DOCUMENT/RSC11_11.TXT` antworten
HTTP 206 (Byte-Range bedient); das Volumen-Wurzel-Listing und das
`ODR/`-Unterverzeichnis antworten HTTP 200 (die im Banden-Check gemessene
500-Antwort der Annex-Wurzel ist ein gelöstes Server-Listing-Problem). Es gibt
kein `DATA/`-Verzeichnis (404) — die `.ODR`-Dateien liegen direkt unter `ODR/`.
Die zwei geholten Stichproben-Dateien (eine am Volumen-Anfang laut INDEX,
eine unter den letzten Dateien des `ODR/`-Listings) liefern je HTTP 206 und
283 000 B = 500 Records à exakt 566 B (mod 566 = 0). n der Stichprobe =
1000 Records.

Abdeckung aus INDEX.TAB: 244 auswertbare Segmente, erster Start
1994-04-28T14:16:19Z, letzter Stopp 1995-06-28T19:04:59Z, 79 verschiedene
UTC-Tage. Das Fenster deckt GWE1 (Frühjahr 1994) und GWE2 (Frühjahr 1995) ab
und **endet 5 Monate vor dem Beginn der Floor-Ära**: 0 ODR-Tage liegen auf
oder nach 1995-11-23. Die Stations-Verteilung der Dateien (DSS 14: 57, DSS 43:
111, DSS 63: 73, LBL-Census 241/241) ist im Bestands-Befund gemessen und gilt
hier unverändert; der INDEX trägt keine Station je Segment.

## Messung 2 — Überlappung mit den laut/ruhig-Tagen (Ded-31-Abdeckung)

Die Floor-Register-Zählung wird auf demselben Resid-Bestand reproduziert
(Konventionen der Referenz-Linie): 400 robuste Trio-Floor-Zellen
(Stationen 14/43/63), davon **207 laut** (M1 101, M2 55, M3 51) — der
Register-Stand „207 laut-Tage" ist exakt reproduziert. Der früheste
Trio-Floor-Tag ist **1995-11-23**, der letzte ODR-Tag ist **1995-06-28**;
die Kalender-Lücke misst **148 Tage**.

Floor-Zellen mit Tag im ODR-Fenster [1994-04-28 … 1995-06-28]: **genau 1** —
M1, Station 24 (Goldstone 34-m), 1994-12-18, n 12 391, Zell-RMS 0,0324 Hz
(ruhig). Station 24 ist **keine** der drei GWE-ODR-Stationen; die Zelle ist
die im Ops-Ära-Blatt bereits benannte einzelne frühe Nicht-70-m-Zelle. Unter
den ODR-Stationen 14/43/63 existiert im gesamten GWE-Fenster **keine einzige
Floor-Zelle** — nicht weil der Closed-Loop-Kanal fehlte (im Fenster liegen
391 294 klassierte Proben an den ODR-Stationen: st14 82 816 · st43 120 379 ·
st63 188 099), sondern weil der AGC-Klemm-Zustand (Stärke −2560) an den
70-m-Stationen in der GWE-Ära nicht auftritt. Der 0-Überlapp ist eine gemessene
Klassen-/Zeit-Abwesenheit, kein Datenloch.

**Ded-31-Verdikt der Abdeckung: n = 0 überlappende (Tag, Station).** Der
Zweiter-Zeuge-Test — reproduziert das ODR die laut/ruhig-Trennung auf
denselben Tagen? — ist auf dem GWE-ODR nicht ziehbar: es gibt keinen einzigen
lauten oder ruhigen Floor-Tag des Bestands, den das ODR aufgezeichnet hat. Die
Frage „upstream/real oder Reduktions-Artefakt" bleibt damit offen — sie ist
durch dieses Volumen nicht entscheidbar, weil das Volumen die Entscheidungs-
Tage nicht trägt. Das ist ein gemessenes Nein zur Zeugen-Eignung (Abdeckung),
kein Ja/Nein zum Phänomen.

## Messung 3 — Resid→ODR-Transfer

Der Transfer (prägt die Lautheit der lauten Tage aus den Residuen auf das ODR
desselben Kontakts?) verlangt denselben Kontakt in beiden Aufzeichnungen. Da
kein laut-Tag des Bestands im ODR liegt, ist der Transfer auf identischen
Kontakten nicht prüfbar: **n = 0, 0 geehrt, Verdikt nicht tragen/nicht
tragen — unentschieden durch Abwesenheit der Kontakte, nicht durch einen
Messwert.** Die einzige im ODR-Fenster liegende Floor-Zelle (st24,
1994-12-18, ruhig 0,0324 Hz) hat keinen ODR-Kontakt derselben Station
(GWE-Stationen sind 14/43/63) — auch sie trägt den Transfer nicht.

## Grenzen

- Die ODR-Abdeckung ist aus INDEX.TAB gemessen (244 auswertbare Segmente,
  79 Tage); die physische Voll-Datei-Prüfung (241 vorhandene `.ODR`, ~4,2 GB)
  ist im Bestands-Befund gemessen und hier nicht wiederholt. Die Stichprobe
  (2 Dateien, 1000 Records) verifiziert Datei-/Record-Reachability und
  das 566-Byte-Format; sie misst keine Spektral- oder Hüllkurven-Größe (der
  Banden-Check hat drei Vollpässe auf Trägerfrequenz und Amplitude gemessen).
- Der Tag-Schlüssel der Floor-Zellen ist der TDB-Tag (Mittags-Konvention der
  Vorlagen); die ODR-Zeiten sind UTC. Die 148-Tage-Lücke ist um Größenordnungen
  größer als jeder Konventions-Unterschied.
- Die Zählung „79 vs 77 Tracking-Tage" (Bestands-Befund) folgt anderen
  Zähl-Konventionen (hier: jeder UTC-Tag, den ein Segment berührt, dort:
  Tracking-Tage); beide sind gemessen, keine widerspricht der anderen.
- Der Register-Stand 207 laut/105 Flips bezieht sich auf die drei 70-m-
  Stationen; die st24-Zelle 1994-12-18 gehört nicht dazu.
- Ob das ODR auf den laut-Tagen ein anderes Rauschen getragen hätte, ist durch
  dieses Volumen unbeantwortet (keine Aufzeichnung nach 1995-06-28); eine
  spätere open-loop-Aufzeichnung (falls vorhanden) wäre der nötige Zeuge.

## Register-Satz

*Das GWE-ODR endet gemessen am 1995-06-28T19:04:59Z; die 207 laut-Tage der
robusten Floor-Register (400 Zellen, M1 101 + M2 55 + M3 51, exakt
reproduziert) beginnen alle erst am 1995-11-23 — der Ded-31-Zweiter-Zeuge-Test
hat n = 0 überlappende (Tag, Station): das ODR kann die laut/ruhig-Trennung
weder bestätigen noch widerlegen (0 geehrt), und der Resid→ODR-Transfer ist
auf identischen Kontakten nicht prüfbar (0 geehrt). Die einzige Floor-Zelle im
ODR-Fenster (M1 st24, 1994-12-18, 0,0324 Hz ruhig) liegt an einer
Nicht-ODR-Station; der Closed-Loop-Kanal lief im Fenster an allen drei
ODR-Stationen (391 294 Proben) — die 0 ist eine Klassen-Abwesenheit, kein
Datenloch. Ob die station-gebundene Floor-Lautheit upstream-real oder
Closed-Loop-Artefakt ist, bleibt offen und ist zeitlich nach der
GWE-open-loop-Aufzeichnung verortet.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat). Probe
`galileo_odr_overlap_witness` (`cargo check` 0/0, RUSTFLAGS `-D warnings`),
Report `/tmp/opencode/galileo_odr_overlap_witness_report.txt`. Abdeckung,
Überlappung und Transfer gemessen; die 0-Überlappung ist benannt, nicht
gefüllt.
