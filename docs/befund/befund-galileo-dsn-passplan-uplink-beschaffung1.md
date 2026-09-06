<!--
  title: Befund — DSN-Tracking-/Master-Schedule Galileo (Beschaffung 1): die Appendix-A-Figures der RSS-Handbücher sind beschafft und OCR-gelesen — sie tragen den einzigen per-Pass-Stations-Plan des PDS-RSS-Korpus, aber keiner der Pläne erreicht die Floor-Ära (letzter Plan-Tag DOY 179/1995, erster Floor-Tag 1995-11-23); das per-Pass-Uplink-Bild der Floor-Ära bleibt aus öffentlichen Quellen unbeschaffbar, der Deduktion-27-Split bleibt pending; die TDA-Progress-Report-Artikel sind live erreichbar (tda.jpl.nasa.gov HTTP 200, 42-127-Inhalt gemessen: kein Stations-Plan) — Korrektur zum 403-Ledger
  class: befund
  date: 2026-09-06
  sha256: b5a55ccbc3f615ec9ffb0fc128e523363cf04edcad6e33d1b93e8e368aa05779
  status: draft
  see-also: docs/befund/befund-galileo-dsn-passplan-uplink.md docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-receiver-je-pass-floor.md docs/reference/trk-2-25-atdf.txt
-->
# Befund: DSN-Tracking-/Master-Schedule der Floor-Ära — die RSS-Handbuch-Appendix-A beschafft und gemessen, Uplink-Station je Floor-Tag bleibt unbeschaffbar

## Auftrag & Bindung

Dieser Lauf (Beschaffung 1 von 3) besorgt den DSN-Tracking-/Master-Schedule, der
je Galileo-Pass der Floor-Ära (1995-11-23 .. 1997-02-28) die Uplink-Sende-
Station nennt. Ziel ist der (Empfänger x Sender)-Split der 51 lauten / 43
ruhigen Dreiweg-Floor-Tage (Deduktion 27). Der Vorgänger-Lauf
`befund-galileo-dsn-passplan-uplink.md` hat die Lücke benannt: der PDS-
DOCUMENT-Bestand führt nur SIS/Handbücher, die TDA/IPNPR-Report-Archive
antworteten HTTP 403, und die HANDBK5-Appendix-A-Figures (die gescannten
"exact schedule of tracking times"-Seiten) lagen unausgewertet im Bestand.
Dieser Lauf holt genau diese Figures (URL + HTTP-Status belegt), liest sie per
OCR (gemessener Transfer, tesseract 5.3.4) und misst, welche Fenster sie
abdecken; dazu ein breiterer DSN-Schedule-Sweep mit belegtem Status je Quelle.

## Beschafftes Gut (URL + HTTP-Status + sha256)

Basis: PDS-PPI-Annex, Volumen `GO-SUN-RSS-1-TDF-V1.0`, Ordner
`DOCUMENT/HANDBOOK5/`. Alle Abrufe HTTP 200 (gemessen 2026-09-06).

- `https://pds-ppi.igpp.ucla.edu/annex/GO-SUN-RSS-1-TDF-V1.0/DOCUMENT/HANDBOOK5/HANDBK5.ASC` — 200, 192 533 B. Das ASCII trägt Appendix A nur als leere Überschrift (Z. 3373-3390, reproduziert).
- `HANDBK5.LBL` — 200; `^TIF_DOCUMENT` listet `A_01.TIF`..`A_20.TIF` als die Appendix-A-Seiten ("A_xx.TIF ... make up appendices in the Handbook ... scanned from hard copy").
- `A_01.TIF`..`A_20.TIF` — je 200, je 1 892 989 B (uncompressed TIFF, 1203 x 1573, 8-bit grayscale, PhotometricInterpretation BlackIsZero). sha256 je Datei im Manifest
  `data/galileo_dsn_sched/sha256_appendixA_tif.txt` (Kopie liegt in diesem Lauf unter `/tmp/opencode/beschaffung1/`, transient); die lokalen OCR-Transkripte je Seite liegen unter `data/galileo_dsn_sched/appendixA_ocr/A_*.txt` (maschinenlokal, `data/` ist gitignored; dauerhaftes Zuhause ist der PDS-Annex mit obiger URL + sha256).

## Was die Figures tragen (gemessen per OCR)

OCR-Methode: tesseract 5.3.4 / leptonica 1.82.0; jede Seite in vier Rotationen
(0/90/180/270) gelesen, je Seite die Lesart mit lesbarem Kopf/Inhalt gewählt.
Die Seiten sind Scan-Faksimiles; Zahlen- und Datumsfelder sind in den
OCR-Transkripten teils unscharf (Schriftart), die Kopf-/Struktur-Lesart ist
eindeutig. Wo ein Feld im Scan unlesbar bleibt, ist es `pending`, nicht ergänzt.

Per-Seite-Inhalt (Kopf der OCR-Lesart gemessen):

| Seite | Titel / Inhalt (OCR-Kopf) | Abdeckung |
|---|---|---|
| A_01..A_07 | GALILEO USO ACTIVITIES, Log-Tabelle (PAGE 1..7) | Einträge Dez-1994 .. Jan-1997 |
| A_08, A_09 | Galileo Solar Wind Scintillation Experiment '94, Balken-Abdeckungs-Chart (DOY-Achse, "DARK AREAS INDICATE CONTINUED SUPPORT", Signatur 10/10/94) | DOY 337-362 (1994) |
| A_10..A_13 | Galileo Solar Wind Scintillation Experiment '94, Stations-Zuteilungs-Tabellen (Spalten Doy / DSS / BOT / EOT / Config / Comments), Seiten A-SWSE94-1..-4 | DOY 309-362 = 1994-11-05..12-28 |
| A_14 | ULYSSES SOLAR CORONA EXPERIMENT 95, Stations-Zuteilungs-Chart | 1995 |
| A_15, A_16 | ULYSSES SCE 95, Tabellen Seite 1/2 | DOY 54-74 = 1995-02-23..03-15 |
| A_17 | GALILEO GRAVITATIONAL WAVE EXPERIMENT 95, Stations-Zuteilungs-Chart (DOY-Achse) | DOY 140-159 |
| A_18..A_20 | GLL Gravitational Wave Experiment '95, Stations-Zuteilungs-Tabellen (Spalten Doy / DoW / DSS / AOS / EOT / CFG), Seiten A-GWESS/GWEOS | DOY 155-179 = 1995-06-04..06-28 |

Die Stations-Zuteilungs-Tabellen (A_10..A_13, A_18..A_20) sind der einzige
per-Pass-Stations-Plan des ganzen PDS-RSS-Korpus: je DOY eine oder mehrere
DSS-Zeilen (DSS 14/43/63, teils 34-m-Stationen) mit AOS/EOT-Zeit und
Config-Code (A_10..A_13 führen GLL 0481 neben ULY 6486; A_18..A_20 führen
6481). Gemessene Grenze: die Tabellen nennen die **Empfangs-Zuteilung** je Pass
(Fenster je DSS), nicht ein separates Sende-Wort — für die dreiweg-Frage
müsste der Modus (zweiweg = gleiche Station sendet/empfängt gegen dreiweg =
getrennte Stationen) hinzutreten, den die Tabellen nicht führen. Die USO-Log-
Seiten (A_01..A_07) nennen je Eintrag eine DSS (14/43/63/42/61 gemessen) und
Kommentare wie "DSS 14 Transmitter off to acquire 1-way data" — die Einträge
sind USO-Test-Passes (~1-2 h), keine Dreiweg-Vollpasses.

**Abdeckungs-Verdikt: keiner der per-Pass-Pläne erreicht die Floor-Ära.**
Der letzte geplante Pass-Tag der Zuteilungs-Tabellen ist DOY 179/1995
(1995-06-28, GWE'95); der erste Floor-Tag der Serie ist 1995-11-23 (DOY 327).
Die Lücke zwischen letztem Plan-Tag und Floor-Beginn misst 148 Tage (0 geehrt:
die Floor-Ära ist in keiner beschafften Figur geführt). Die USO-Log-Seiten
reichen zwar bis Jan-1997 in die Ära hinein, aber ihre Eintrags-Tage fallen
nicht auf die Floor-Dreiweg-Tage: die Anker-Tage 1995-11-24/25, 1995-12-04/05
und 1996-06-26..30 tragen in der Log keinen Eintrag (DOY-Reihen gemessen;
Log-Einträge jener Monate: DOY 345/346/350 im Dezember 1995 (DSS 42/63/61)
und DOY 164 im Juni 1996 (DSS 63) — nicht die Floor-Tage). Damit ist die Uplink-Sende-
Station je (Tag, Dreiweg-Zelle) der Floor-Ära aus den beschafften Handbuch-
Figures nicht gewinnbar.

## Breiterer Sweep (URL + HTTP-Status, gemessen 2026-09-06)

- `https://ipnpr.jpl.nasa.gov/progress_report/` und `.../42-124/42-124A.pdf` — HTTP 403 (live bestätigt, wie im Vorgänger-Ledger).
- `https://tda.jpl.nasa.gov/progress_report/42-124/124A.pdf` — **HTTP 200**, PDF (6 Seiten, 303 544 B) heruntergeladen; `title.htm` der Hefte 42-124 und 42-127 — HTTP 200. **Korrektur zum Vorgänger-Ledger:** die Artikel-Ebene des TDA-Progress-Report-Archivs ist live erreichbar (nur der Index-Root antwortet 403). Inhalt 42-127 (Juli-September 1996, mitten in der Floor-Ära) gemessen aus `title.htm`: Artikel u. a. Keihm/Marsh (troposphärische Kalibration), Belongie et al. (Galileo-Telemetrie-Frame-Loss), Simon et al., Conroy/Hoppe — **kein per-Pass-Stations-Plan** in der Inhaltsliste.
- Wayback: `archive.org/wayback/available?url=tda.jpl.nasa.gov/progress_report/` — Snapshot 2009-07-12 (HTTP 200); `.../ipnpr.../42-124/` — kein Snapshot. CDX `tda.jpl.nasa.gov/progress_report/42-1*` — 200 (PDF-Bestand der Hefte 42-100 ff. im 2006er-Crawl belegt).
- `https://nssdc.gsfc.nasa.gov/nmc/spacecraft/display.action?id=1989-084B` — HTTP 200 (Missions-Überblick, kein Schedule, wie im Vorgänger-Ledger).
- `https://deepspace.jpl.nasa.gov/` — HTTP 200 (heutiges DSN-Portal; es führt keine historischen Schichten der Ära).
- `https://ntrs.nasa.gov/api/citations/search?q=galileo%20tracking%20schedule` — HTTP 200; Treffer tragen keinen Schedule (`Galileo Satellite Tour: Orbit Determination`, `A new code for Galileo`); Suchbegriff-Kombination "DSN tracking schedule" 0 Treffer.
- PDS-Volumina GO-JG/GO-J/GO-JS/GO-SS/GO-SUN-ODR/GO-J-ODR `DOCUMENT/` — HTTP 200, Ordner gelistet: die Handbücher Bd. 3/4/5/6 und OS_/FIGA_/RSDIR_-Figuren. `HANDBK6` (GO-JG, Publikation 1995-11-22) Appendix-Struktur gemessen: Appendix A = Open-Loop-System-Text, B = Abkürzungen, C = RSDIR_1..5 — **kein Schedule-Anhang**; die OS_01..15-Figuren sind Operations-Skripte (Abschnitt 5.6.2, JOI-Phase, gelesen), keine Stations-Zuteilung. `HANDBK4` (GO-SS, Earth-1/2-Masse, 1992) Anhang F/G = Schedule der Earth-Flybys 1990-1992, vor der Ära.

## Verdikt / Deduktion-27

**Der einzige per-Pass-DSN-Stations-Plan des PDS-RSS-Korpus ist beschafft und
gelesen — er deckt die Floor-Ära nicht ab.** Die HANDBK5-Appendix-A-Figures
(GO-SUN-TDF, alle 20 Seiten heruntergeladen, HTTP 200, sha256 belegt, OCR-
Transkripte lokal in `data/galileo_dsn_sched/appendixA_ocr/`) enden mit dem
GWE'95-Plan am 1995-06-28; die Floor-Ära beginnt am 1995-11-23. Die
USO-Activity-Log-Einträge der Ära fallen auf Test-Tage, nicht auf die
Floor-Dreiweg-Tage. Die TDA-Progress-Report-Hefte der Ära sind auf
Artikel-Ebene live erreichbar (Korrektur zum 403-Ledger), tragen aber keinen
Stations-Plan (42-127-Inhalt gemessen). Damit bleibt die Uplink-Sende-Station
je (Tag, Dreiweg-Zelle) der Floor-Ära **unbeschaffbar** aus öffentlichen
Quellen; der (Empfänger x Sender)-Split der 51 lauten / 43 ruhigen Dreiweg-
Tage ist **nicht ziehbar — Deduktion 27 bleibt pending**, jetzt mit gemessenem
Negativ über die tatsächlich gelesenen Appendix-Seiten statt über eine
Inventar-Lücke.

**Was die Lücke schliessen würde (benannt):** ein Floor-Ära-Gegenstück der
beschafften Zuteilungs-Tabellen — d. h. ein per-Pass-Plan der Ära, der je
(Tag, Pass) Sende- und Empfangs-Station nennt. HANDBK6 §5.1.3/5.1.4 benennt
die Betriebs-Artefakte, die diese Angabe tragen (SOE/Keyword-Datei,
7-Day-Schedule des DSN-Network-Operations), als Flugbetriebs-Produkte, die
nicht öffentlich archiviert sind. Ein öffentliches Archiv eben dieser
Artefakte (oder einer DSN-Schicht der Ära) würde die Achse öffnen.

## Registrierung (Session-Duty)

Keine `phi/sources.φ`-Registrierung: beschafft ist eine Text-/OCR-Gewinnung
aus bestehenden PDS-Dokument-Figuren (kein neuer physikalischer Datensatz, kein
neues Mess-Asset). Benannt statt registriert: die Appendix-A-Figures sind
Dokumente des bestehenden GO-SUN-RSS-1-TDF-Bestands (deren Herkunft oben mit
URL + sha256 belegt ist); die OCR-Transkripte sind abgeleiteter Text,
maschinenlokal unter `data/galileo_dsn_sched/`.

## Grenzen

- Die OCR ist ein gemessener Transfer gescannter Seiten; einzelne Zahlen-/
Datumsfelder sind unscharf. Die Abdeckungs- und Kopf-Aussagen stützen sich auf
die eindeutig lesbaren Kopf-/Strukturzeilen und die DOY-Reihen der Tabellen;
nicht lesbare Zellen bleiben `pending`.
- Die Zuteilungs-Tabellen nennen die Empfangs-Zuteilung je Pass; ob ein Pass
zweiweg oder dreiweg gefahren wurde und welche Station sendete, steht dort
nicht — auch ein die Ära deckender Plan gleicher Bauart würde die Uplink-
Frage nur mit zusätzlicher Modus-/Sende-Angabe beantworten.
- Die Sweep-Status sind an dem Tag gemessen (2026-09-06); die 403-Antworten von
`ipnpr.jpl.nasa.gov` sind Zugriffs-Sperren, keine Inhalts-Aussage.

## Register-Satz

*Der DSN-Tracking-/Master-Schedule der Floor-Ära ist beschafft und die Quellen sind ausgewertet
(Beschaffung 1, Deduktion-27-Split): die HANDBK5-Appendix-A-Figures des
GO-SUN-RSS-1-TDF-Bands (20 TIF, je 200, je 1 892 989 B, sha256-Manifest in
data/galileo_dsn_sched/, OCR-Transkripte je Seite in data/galileo_dsn_sched/
appendixA_ocr/) sind heruntergeladen und per tesseract 5.3.4 gelesen — die
Seiten tragen den einzigen per-Pass-Stations-Plan des PDS-RSS-Korpus: SWSE'94-
Zuteilung (DOY 309-362, 1994), Ulysses-SCE'95 (DOY 54-74, 1995), GLL-GWE'95
(DOY 155-179, 1995) und die Galileo-USO-Activity-Log (Dez-1994..Jan-1997);
kein Plan erreicht die Floor-Ära (letzter Plan-Tag 1995-06-28, erster Floor-
Tag 1995-11-23, Lücke 148 Tage gemessen), und die USO-Log-Einträge der Ära
fallen auf Test-Tage, nicht auf die Floor-Dreiweg-Tage (Anker 1995-11-24/25,
1995-12-04/05, 1996-06-26..30 ohne Log-Eintrag) — die Uplink-Sende-Station je
(Tag, Dreiweg-Zelle) bleibt unbeschaffbar, der (Empfänger x Sender)-Split der
51 lauten / 43 ruhigen Dreiweg-Tage ist nicht ziehbar, Deduktion 27 bleibt
pending; Korrektur zum 403-Ledger: die TDA-Progress-Report-Artikel sind live
erreichbar (tda.jpl.nasa.gov/progress_report/42-124/124A.pdf 200, 6 Seiten),
aber die Hefte der Ära tragen keinen Stations-Plan (42-127-Inhalt gemessen:
Kalibrations-/Telemetrie-/Kommunikations-Artikel), ipnpr.jpl.nasa.gov bleibt
403; HANDBK6 (GO-JG, 1995-11-22) führt keinen Schedule-Anhang (Appendix A =
Open-Loop-Text, OS_01..15 = Operations-Skripte), HANDBK4 (GO-SS) nur die
Earth-Flyby-Ära 1990-1992; NSSDC 200 ohne Schedule, deepspace.jpl.nasa.gov
200 ohne historische Schichten, NTRS-Suche ohne Schedule-Treffer; keine
phi/sources.φ-Registrierung (OCR-Gewinnung aus bestehenden Dokument-Figuren,
kein neuer Datensatz), die Lücke braucht ein öffentliches Archiv der
Flugbetriebs-Artefakte (SOE/7-Day-Schedule), die HANDBK6 als Träger der
Sende-Angabe benennt.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; die Haupt-Session ergänzt die
TODO-Registerzeile). Beschafft und committet ist nur dieses Blatt
(pfad-beschränkt, eigene Arbeit); die Appendix-Figures und OCR-Transkripte
liegen maschinenlokal unter `data/galileo_dsn_sched/` (gitignored) mit dauerhaftem Zuhause auf dem PDS-Annex (URL + sha256 belegt). Keine Rust-Änderung,
kein `cargo check`-Lauf nötig.
