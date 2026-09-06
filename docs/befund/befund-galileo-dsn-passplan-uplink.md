<!--
  title: Befund — Galileo-DSN-Pass-Plan der Floor-Ära (Richtung H, Deduktion-27-Split): die Uplink-Sende-Station je Pass ist in keinem erreichbaren Pass-Plan geführt — die PDS-RSS-Volumina tragen keinen Tracking-/Stations-Plan (DOCUMENT/INDEX/CATALOG/LBL/GEOMETRY gemessen durchsucht), die externen DSN-Report-Archive sind gesperrt (HTTP 403), der (Empfänger×Sender)-Split der 51 lauten / 43 ruhigen Dreiweg-Tage bleibt pending
  class: befund
  date: 2026-09-06
  sha256: 77c61fc1be8f849b253b6317524ffd97f3d1453a1d758bedfb5a4af59d999940
  status: draft
  see-also: docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-receiver-je-pass-floor.md docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-gwe-bestand.md docs/befund/befund-galileo-floor-stufen-te.md docs/reference/trk-2-25-atdf.txt
-->
# Befund: Der Galileo-DSN-Pass-Plan der Floor-Ära — Such-Ledger gemessen leer, Deduktion 27 bleibt pending

## Frage & Bindung

Die Pioneer-Deduktion 27 (der two-/three-way-Split der 20-s-Bande) fragt, ob die
station-gebundene Laut-Struktur der Galileo-Floor-Gipfel im **Uplink**
(sendende Station) oder im **Downlink** (empfangende Station) sitzt. Der
Schwester-Befund `befund-galileo-dreiweg-sendempfang-floor.md` hat das
strukturelle Tor gemessen: der rohe TRK-2-25-Dreiweg-Record trägt die
Sende-Station weder pro Sample noch pro Pass (item 10 ist das einzige
Stations-Wort = die aufzeichnende/Empfangs-Station; XMTR-Felder unbesetzt).
Damit braucht der Split die Sende-Station aus einer **zweiten Quelle**: dem
DSN-Tracking-/Pass-Plan (welche Bodenstation an welchem Pass/Pass-Tag
uplinkte). Dieser Lauf (Agent 1 von 3) sucht genau diesen Plan für die
Floor-Ära (1995-11-23 .. 1997-02-28) und belegt jede Suche mit
URL/HTTP-Status/Provenienz. Ersatz und Raten sind ausgeschlossen: wenn kein
Pass-Plan erreichbar ist, ist das das gemessene Ergebnis, und der Split
bleibt `pending`.

## Die Ziel-Zellen, die der Plan hätte zerlegen sollen (aus der Referenz)

Der (Empfänger×Sender)-Split hätte über die 94 robusten Dreiweg-Floor-Tage
(n ≥ 30) der Floor-Ära laufen müssen — 51 laut, 43 ruhig, Empfänger (item 10)
je Zelle bekannt: st14 40 Tage (25 laut / 15 ruhig), st43 33 (16 / 17), st63 21
(10 / 11). Der Sender je (Tag, Zelle) ist die gesuchte Größe; die
laut/ruhig-Flips laufen bei konstanter aufgezeichneter Station (z. B. st14
dreiweg 52,9 Hz am 1995-12-05 zwischen ruhigen Nachbartagen; st63 dreiweg
34,7 Hz am 1995-11-23 → ruhig am 1995-11-25 → 186,5 Hz am 1995-11-27).
Register und Zell-RMS: `befund-galileo-dreiweg-sendempfang-floor.md` und der
Report `/tmp/opencode/galileo_threeway_txrx_split_report.txt`.

## Such-Ledger (gemessen, 2026-09-06)

### 1. PDS-PPI-Annex, die RSS-Volumina selbst

Wurzel `https://pds-ppi.igpp.ucla.edu/annex/` — HTTP 200; dort liegen die 10
`GO-*-RSS-*-V1.0`-Volumina. Durchsucht (Verzeichnis-Listings, HTTP 200):

- **`DOCUMENT/` aller RSS-Volumina** (GO-SUN/GO-J/GO-JG/GO-JS/GO-SS-RSS-1-TDF,
  GO-J-RSS-1-ODF, GO-SUN/GO-JS/GO-J-RSS-1-ODR): Dateibestand je Volumen ist
  nur SIS-Text (`TRK_2_25.TXT`, `TRK_2_18.TXT`, `RSC11_11.TXT`,
  `GLLRSSIS.TXT`), Handbuch-Bilder (`HANDBOOK*`, TIF-Scans), `DESCRIPT.TXT`/
  `DOCINFO.TXT`, und im ODR-Volumen die `POSTOPS93/`/`POSTOPS94/`-Bildordner
  (grafische Post-Operations-Reporte der GWE-Ära 1993/94, vor der Floor-Ära).
  **Kein Dateibestand trägt einen Tracking-/Pass-/Stations-Plan.**
- **`INDEX.TAB`** (GO-SUN-TDF gelesen, HTTP 200, 6 Spalten): `VOLUME_ID`,
  `FILE_SPECIFICATION_NAME`, `PRODUCT_ID`, `START_TIME`, `STOP_TIME`,
  `PRODUCT_CREATION_TIME` — keine Stations-Spalte, kein Rollen-Wort; die
  `INDEX.LBL`-Spaltendefinition gelesen. GO-JG-TDF gleicher Spaltenbau.
- **Per-Datei-`LBL` der Floor-Ära-TDF** (4 Dateien, HTTP 200): `5327328A.LBL`,
  `5337339A.LBL`, `5340341A.LBL` (GO-SUN-TDF) und `6177179A.LBL` (GO-JG-TDF).
  Jede trägt `DSN_STATION_NUMBER = {14,43,63}` (die Menge der Stationen, deren
  Records in der Datei liegen — Empfangs-Seite), `PRODUCER_ID = DSN`,
  `MISSION_PHASE_NAME`, Start/Stopp. Kein Uplink-/Sender-Wort.
- **`GEOMETRY/`** (GO-SUN-TDF: `S970311A.XSP` = 1997-03-11, nach der
  Floor-Ära; GO-JG-TDF: `S980326B.XSP` = 1998): NAIF-SPK/pointing-Geometrie,
  kein Stations-Plan; `GEOMINFO.TXT` gelesen.
- **`GLLRSSIS.TXT`** (RSS-SIS, GO-SUN- und GO-JG-TDF, HTTP 200): beschreibt
  die RDA-Struktur (ATDF/ODF/ODR) und die Records, keinen Pass-Plan.
- **`HANDBK5.ASC`** (Radio-Science-Handbook Bd. 5, GO-SUN-TDF, HTTP 200):
  beschreibt DSN-Geräte und generische Betriebs-Konfiguration; der
  „Radio Science Master Schedule" existiert nur als gescannte Figuren
  (Appendix A ist im ASCII leer, Z. 3373-3390) und ist kein Pass-Plan.

### 2. Lokaler Referenz-Bestand

`docs/reference/` hält die SIS-Texte `trk-2-25-atdf.txt`,
`dsn_trk-2-18.*` — die vollständige TRK-2-25-Feldtabelle (item 10 = einziges
Stations-Wort), keinen Schedule. Ein früherer Auftrag (`auftrag-flyby-doppler-rohdaten.md`)
referenziert „tracking plan" nur als Such-Begriff, nicht als Bestand.

### 3. Externe DSN-/Missions-Archive

- `https://ipnpr.jpl.nasa.gov/progress_report/` — HTTP **403** (auch mit
  Browser-User-Agent); das Archiv der DSN-/TDA-Progress-Reporte (der
  historische Veröffentlichungs-Ort von „DSN support of Galileo"-Übersichten)
  ist nicht erreichbar.
- `https://tda.jpl.nasa.gov/progress_report/` — HTTP **403** (zweimal
  gemessen). Ein Wayback-Schnappschuss (1997) existiert (HTTP 200), wurde aber
  nicht als Pass-Plan-Quelle geführt: die Progress-Reporte tragen
  Missions-Übersichten, keinen per-Pass-Uplink-Plan.
- `https://nssdc.gsfc.nasa.gov/nmc/experiment/display.action?id=1989-084B-08`
  (Galileo-Experiment-Seite) — HTTP 200; Missions-/Experiment-Überblick, kein
  Pass-Plan.
- PDS4-Registry-Pfade (`urn:nasa:pds:galileo.rss`, `pds.nasa.gov/datasets/`,
  `pds.nasa.gov/search/`) — 0 Treffer bzw. HTTP 404, wie der
  GWE-Bestands-Befund bereits gemessen hat; es existiert kein PDS4-RSS-Bündel,
  das einen Schedule tragen könnte.

## Gemessenes Ergebnis

**Der DSN-Tracking-/Pass-Plan der Floor-Ära ist aus keiner erreichten Quelle
beziehbar.** Die PDS-RSS-Volumina (der einzige primäre Bestand) führen keinen
Plan: DOCUMENT-, INDEX-, CATALOG-, LBL- und GEOMETRY-Inhalte über die
10 `GO-*-RSS-*-V1.0`-Volumina sind gemessen und tragen keinen per-Pass-
Stations-/Uplink-Plan — die einzige Stations-Identität des ganzen Bestands
ist der Empfangs-Record (item 10, `DSN_STATION_NUMBER = {14,43,63}` als
Datei-Menge). Die externen DSN-Report-Archive sind gesperrt (HTTP 403), der
NSSDC-Überblick trägt keinen Plan, und ein PDS4-Bündel existiert nicht. Damit
ist die Sende-Station je (Tag, Dreiweg-Zelle) weiterhin **unmessbar** — das
ist das gemessene Ergebnis dieses Laufs, kein Ersatz.

**Deduktion-27-Verdikt: `pending`.** Der (Empfänger×Sender)-Split der 51
lauten / 43 ruhigen Dreiweg-Tage ist nicht ziehbar, weil der Empfänger je
Zelle (item 10) gemessen, der Sender je Zelle aber in keiner erreichbaren
Quelle geführt ist. Ob die Lautheit dem Uplink (Sender) oder dem Downlink
(Empfänger) folgt, bleibt offen.

**Was die Lücke schließen würde (benannt, nicht ersetzt):** ein per-Pass-
Plan der Floor-Ära, der je (Tag, Pass) die Uplink-Station nennt — als
öffentliches Archiv des DSN-Tracking-Schedules bzw. der NOCC-Tages-
Zuweisungen oder der Sequence-of-Events der Ära (Flugbetriebs-Artefakt, nicht
in PDS archiviert), als ODF/TRK-2-18- oder Record-Metadatum mit Sender-Angabe
(im Bestand absent, gemessen), oder als offener Zugang zu den
TDA/IPNPR-Progress-Reporten (derzeit HTTP 403). Die
Zeitüberlappungs-Lesart aus dem TRK-2-25-Korpus (dreiweg-Zelle überlappt
zweiweg-Block einer anderen Station) bleibt Inferenz und wird nicht zum
Verdikt erhoben.

## Grenzen

- Die Suche deckt den primären Bestand vollständig ab (alle RSS-Volumina des
  PDS-PPI-Annex, DOCUMENT/INDEX/LBL/GEOMETRY gemessen); sie ist aber eine
  Suche in *erreichbaren* Quellen. Ein Pass-Plan in nicht öffentlich
  erreichbaren Flugbetriebs-Archiven (JPL-intern, DSN-Betrieb) ist damit
  weder bestätigt noch widerlegt — benannt als Lücke, nicht als Absenz der
  Sache.
- Die HTTP-403-Antworten der JPL-Report-Archive sind Zugriffs-Sperren,
  keine Inhalts-Aussage.
- Der Appendix A des Radio-Science-Handbuchs („Schedule of Activities")
  liegt nur als gescannte Figuren vor; sein Inhalt wurde nicht gelesen und
  wird nicht gedeutet (nicht maschinenlesbar archiviert).

## Register-Satz

*Der DSN-Tracking-/Pass-Plan der Floor-Ära ist aus keiner erreichten Quelle
beziehbar (Agent 1, Richtung H, Deduktion-27-Split): die PDS-RSS-Volumina
tragen keinen per-Pass-Plan — über die 10 GO-*-RSS-*-V1.0-Volumina des
PPI-Annex sind DOCUMENT- (nur SIS/Handbuch/Post-Ops-Bilder der GWE-Ära),
INDEX- (6 Spalten, keine Stations-Spalte), CATALOG-, per-Datei-LBL-
(DSN_STATION_NUMBER = {14,43,63} als Empfangs-Menge, kein Sender-Wort) und
GEOMETRY-Inhalte (SPK-Geometrie) gemessen durchsucht; die externen DSN-
Report-Archive tda.jpl.nasa.gov und ipnpr.jpl.nasa.gov antworten HTTP 403,
der NSSDC-Galileo-Überblick (HTTP 200) trägt keinen Plan, ein PDS4-RSS-Bündel
existiert nicht — die Uplink-Sende-Station je (Tag, Dreiweg-Zelle) bleibt
unmessbar; der (Empfänger×Sender)-Split der 51 lauten / 43 ruhigen
Dreiweg-Floor-Tage (st14 25/15, st43 16/17, st63 10/11) ist nicht ziehbar,
Deduktion 27 auf Galileo bleibt pending; die Lücke bräuchte einen öffentlichen
DSN-Tracking-Schedule/SOE der Ära oder ein Sender-Metadatum oder offenen
Progress-Report-Zugang; die Zeitüberlappungs-Lesart bleibt Inferenz, kein
Ersatz.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session). Keine Probe angelegt: der Lauf ist eine Quellen-Suche, deren
gemessenes Ergebnis die Abwesenheit der zweiten Achse ist — ohne Pass-Plan
gibt es keine zweite Achse zu kreuzen. Committet ist nur dieses Blatt
(pfad-beschränkt).
