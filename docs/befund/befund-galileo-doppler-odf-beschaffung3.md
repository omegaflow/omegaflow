<!--
  title: Befund — Beschaffung 3 von 3 (Deduktion 27, ODF/TRK-2-18-Pfad): Doppler-ODF der Floor-Ära über die GO-RSS-Familie und die PDS-Landschaft gesucht und gemessen leer — kein weiteres ODF-Volumen trägt Doppler über 1995-11-23..1997-02-28, die 5 TDF-Bände führen 0 ODF-Dateien, das einzige ODF (GO-J-RSS-1-ODF-V1.0, GORS_9400) ist VLBI-only (136 Orbit-Daten-Wörter, data_type 1/3/5/6, Empfang 14, Sende-Wort 0, 0 Doppler, 0 Dreiweg) und fensterfremd (Schnitt 24 ODF-Pass-Tage ∩ 61 Floor-Tage = 0), die Kandidaten weiterer ODF-Bände sind je HTTP 404, andere PDS-Knoten/NSSDCA führen kein Galileo-Doppler-ODF — die Floor-Serie selbst ist Roh-TDF (TRK-2-25, keine Sende-Station); der (Empfänger×Sender)-Split der 51 lauten / 43 ruhigen Dreiweg-Tage bleibt nicht ziehbar, Deduktion 27 bleibt pending, kein neuer Datensatz beschafft, nichts registriert (0 geehrt)
  class: befund
  date: 2026-09-06
  sha256: 34a9e04e33ff9bef6f66270ed43c406f58dc68fd8a818b05c02eee4773b5c445
  status: draft
  see-also: docs/befund/befund-galileo-odf-sender-uplink.md docs/befund/befund-galileo-dsn-passplan-uplink.md docs/befund/befund-galileo-dsn-passplan-uplink-beschaffung1.md docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-gwe-bestand.md
-->
# Befund: Beschaffung 3 von 3 — Doppler-ODF der Floor-Ära über die GO-RSS-Familie gesucht und gemessen leer: kein weiteres ODF-Volumen trägt Doppler über 1995-11-23..1997-02-28, die TDF-Bände führen 0 ODF-Dateien, das einzige ODF (GO-J-RSS-1-ODF-V1.0) ist VLBI-only und fensterfremd zu den Floor-Tagen — die Sende-Station je (Tag, Dreiweg-Zelle) bleibt unbeschaffbar, Deduktion 27 bleibt pending, nichts registriert

## Auftrag & Bindung

Deduktion 27 fragt, ob die Lautheit der Mode-3-Floor-Zellen dem Uplink (Sender)
oder dem Downlink (Empfänger) folgt. Der ODF/TRK-2-18-Record trägt die
Sende-Station als Feld (item 7, Bits 139–145; Empfang item 6, Bits 132–138;
verankert über Spacecraft-ID item 12 = 77). Der Schwester-Befund
`befund-galileo-odf-sender-uplink.md` hat gemessen, dass das einzige ODF-Volumen
der GO-RSS-Familie (GO-J-RSS-1-ODF-V1.0) dieses Feld nirgends besetzt (alle
Records VLBI, tx = 0) und fensterfremd zu allen Floor-Tagen liegt. Dieser Lauf
(Beschaffung 3 von 3) ist der Beschaffungs-Sweep: existiert irgendwo ein
**Doppler-tragendes** ODF/TRK-2-18 oder ein gleichwertiger Orbit-Daten-Record,
der die Floor-Ära 1995-11-23..1997-02-28 abdeckt? Falls ja: herunterladen,
überschneidende Dateien nach `data/`, (Empfänger×Sender)-Split ziehen. Falls nur
eine Leere verifiziert wird: nichts registrieren (0 geehrt).

## n zuerst (0 geehrt) — der Sweep über die PDS-Landschaft

| Maß | n | Beleg |
|---|---|---|
| Annex-Wurzel `https://pds-ppi.igpp.ucla.edu/annex/` | HTTP 200 | Listing |
| GO-*-RSS-*-V1.0-Bände am Annex | 10 | Annex-Wurzel |
| davon mit ODF-Datenverzeichnis | 1 (`GO-J-RSS-1-ODF-V1.0`) | Band-Wurzeln, je HTTP 200 |
| `.ODF`-Dateien in den 5 TDF-Band-Datenverzeichnissen | 0 je Band | GO-J/-JG/-JS/-SS/-SUN `…/TDF/`, Listing je HTTP 200 |
| `.ODF`-Dateien im einzigen ODF-Datenverzeichnis | 23 (je 1 `.LBL`) | `…/ODF/`, HTTP 200 |
| Kandidaten weiterer ODF-Bände (GO-SUN/-JG/-JS/-SS/-X-RSS-1-ODF, RSS-2-ODF, RSS-1-ATDF) | 0 erreichbar | 7 URL-Sonden, je HTTP 404 |
| ODF-Pass-Tage ∩ Mode-3-Floor-Tage | 0 | Tag-Schnitt (Reproduktion) |
| Mode-3-Floor-Tage (resid, Ära) | 61, 1995-11-23..1997-02-25 | `data/galileo_resid.bin` |
| robuste Floor-Zellen / laut | 94 / 51 | dito |
| weitere Galileo-ODF-Volumina anderer PDS-Knoten/NSSDCA | 0 | PDS-Geo (nur Radiosci-Doku), SBN, Atmosphären-Knoten, NSSDCA-Katalog |

## Messung 1 — die GO-RSS-Familie trägt genau ein ODF-Verzeichnis

Die Annex-Wurzel listet 10 `GO-*-RSS-*-V1.0`-Bände: `GO-J-RSS-1-ODF`,
`GO-J-RSS-1-ODR`, `GO-J-RSS-1-TDF`, `GO-JG-RSS-1-TDF`, `GO-JS-RSS-1-ODR`,
`GO-JS-RSS-1-TDF`, `GO-SS-RSS-1-TDF`, `GO-SUN-RSS-1-ODR`, `GO-SUN-RSS-1-TDF`,
`GO-X-RSS-1-ODR` (je Band-Wurzel HTTP 200). Nur `GO-J-RSS-1-ODF-V1.0` führt ein
`ODF/`-Datenverzeichnis; alle TDF-Bände führen ausschließlich `TDF/`
(0 `.ODF`-Dateien je Listing), die ODR-Bände ausschließlich `ODR/`. Ein zweites
ODF-Volumen existiert nicht: die Kandidaten `GO-SUN-RSS-1-ODF`, `GO-JG-RSS-1-ODF`,
`GO-JS-RSS-1-ODF`, `GO-SS-RSS-1-ODF`, `GO-X-RSS-1-ODF`, `GO-J-RSS-2-ODF` und
`GO-SUN-RSS-1-ATDF` antworten je HTTP 404. Auch das RSS-SIS
(`GLLRSSIS.TXT`, HTTP 200) benennt als Datenverzeichnis je Band TDF/ODF/ODR/VLB
ohne weiteren ODF-Datensatz.

## Messung 2 — der Inhalt des einzigen ODF ist VLBI-only und fensterfremd

Das ODF-Volumen GO-J-RSS-1-ODF-V1.0 (`AAREADME.TXT`: `GORS_9400`, „Jovian
System Ephemeris Improvement Experiment", 1996-07-17..1997-12-10) trägt 23
`.ODF`-Dateien. Die Wiedergabe des Census an den im Floor-Fenster gelegenen
Dateien (Reproduktion über die bereits vollständig geholten 23 Dateien) bestätigt
die Referenz: 136 Orbit-Daten-Wörter über das ganze Volumen, alle data_type
1/3/5/6 (VLBI), Empfang station 14 durchgehend, Sende-Wort 0 durchgehend;
0 Doppler-Records (data_type 11–14), 0 Dreiweg, 0 Doppler-Runs. Der
ODF-Fenster-Tag-Schnitt mit den Mode-3-Floor-Tagen ist 0: die Floor-Ära ist
gemessen (61 Tage, 1995-11-23..1997-02-25; 94 robuste Zellen, 51 laut: st14
40/25, st43 33/16, st63 21/10), aber kein ODF-Pass-Tag fällt auf einen Floor-Tag.

## Messung 3 — die Floor-Serie selbst ist Roh-TDF, nicht ODF

`data/galileo_resid.bin` (900 MB, 1995-11-23..1997-02-25) ist aus den rohen
TRK-2-25-TDF-Bänden reduziert (`galileo_atdf_compiler`, 5 TDF-Bände); der
TRK-2-25-Dreiweg-Record trägt die Sende-Station nicht (Tor-1-Verdikt,
`befund-galileo-dreiweg-sendempfang-floor.md`). Ein Doppler-ODF, das dieselbe
Ära mit item 7 trüge, existiert in keiner erreichten Quelle.

## Verdikt

**Beschaffung 3 von 3: Leere verifiziert (0 geehrt), nichts beschafft, nichts
registriert.** Ein Doppler-tragendes Galileo-ODF/TRK-2-18 (oder gleichwertiger
Orbit-Daten-Record), das die Floor-Ära 1995-11-23..1997-02-28 abdeckt, existiert
in keiner erreichten Quelle: die GO-RSS-Familie am PDS-PPI-Annex trägt genau ein
ODF-Verzeichnis (VLBI-only, tx = 0, fensterfremd), die TDF-Bände führen
0 ODF-Dateien, die Kandidaten weiterer ODF-Bände sind HTTP 404, und andere
PDS-Knoten/NSSDCA führen kein Galileo-Doppler-ODF. Die Floor-Serie selbst liegt
als Roh-TDF (TRK-2-25) vor und trägt die Sende-Station nicht. Der
(Empfänger×Sender)-Split der 51 lauten / 43 ruhigen Dreiweg-Floor-Tage ist
damit weiterhin nicht ziehbar; die Frage „Uplink oder Downlink" bleibt `pending`.
Die Sende-Station wird nicht ersetzt und nicht geraten.

## Grenzen

- Der Sweep deckt die erreichbaren öffentlichen PDS-Bestände und den
  NSSDCA-Katalog ab; ein nicht-öffentlicher DSN-/JPL-Navigationsbestand ist
  damit weder bestätigt noch widerlegt (benannte Lücke, kein Ersatz).
- Die ODF-Inhaltszählung stützt sich auf den vollständigen Download des
  Volumens (23/23 Dateien, bereits im Schwester-Befund geholt) und wurde hier an
  den Fenster-Dateien reproduziert; die 136-Zahl ist die Volumen-Zensur.
- Die Floor-Zell-Serie ist lokal vorhanden und reproduziert die Referenz;
  der Roh-Ursprung (TRK-2-25) ist aus dem Compiler-Pfad benannt, nicht geraten.

## Register-Satz

*Beschaffung 3 von 3 (Deduktion 27, ODF/TRK-2-18-Pfad) gemessen leer: über den
PDS-PPI-Annex (10 GO-*-RSS-*-V1.0-Bände, je HTTP 200) trägt nur
GO-J-RSS-1-ODF-V1.0 ein ODF-Datenverzeichnis (23 .ODF, GORS_9400, VLBI-only:
136 Orbit-Daten-Wörter, data_type 1/3/5/6, Empfang 14, Sende-Wort 0, 0 Doppler,
0 Dreiweg; Fenster 1996-07-17..1997-12-10, Schnitt mit 61 Floor-Tagen = 0),
die 5 TDF-Bände führen 0 ODF-Dateien, die Kandidaten GO-SUN/-JG/-JS/-SS/-X-
RSS-1-ODF, GO-J-RSS-2-ODF und GO-SUN-RSS-1-ATDF sind je HTTP 404, andere
PDS-Knoten und der NSSDCA-Katalog führen kein Galileo-Doppler-ODF — ein
Doppler-tragendes ODF/TRK-2-18 der Floor-Ära 1995-11-23..1997-02-28 existiert
in keiner erreichten Quelle; die Floor-Serie selbst ist Roh-TDF (TRK-2-25,
keine Sende-Station, 94 robuste Zellen / 51 laut); der (Empfänger×Sender)-Split
ist nicht ziehbar, Deduktion 27 bleibt pending; kein neuer Datensatz beschafft,
nichts registriert.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat). Es wurde kein neuer Rust-Bin
gebaut: die Messung ist ein Quellen-Sweep (URL + HTTP-Status je Kandidat) gegen
den bereits committeten Bestand (Sonde `galileo_odf_sender_uplink`, Referenz
`befund-galileo-odf-sender-uplink.md`); nur dieses Blatt wird committet.
