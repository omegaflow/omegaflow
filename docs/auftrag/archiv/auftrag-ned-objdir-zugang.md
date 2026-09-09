<!--
  title: Auftrag (extern) — NED: Dienst-Erholung, Bulk-z-Zugang, stabile Harvest-Route
  class: auftrag
  date: 2026-09-09
  sha256: 607a691a761204321863b1ea1b95595b577f1333c1630ee71e334a3487d7dade
  status: archived
  see-also: docs/SOURCE_PORT.md docs/TODO.md phi/sources.φ
-->

# Rechercheauftrag (extern): NED — Dienst-Erholung, Bulk-z-Zugang, stabile Harvest-Route

Ausgangslage (gemessen 2026-09-08/09, nicht neu suchen): der `ned-cdn`-Harvest
(`NEDTAP.objdir`, ~1,1 Mrd Cross-ID-Objekte; gebrauchter Feldwert z, 11–19 Mio
mit gemessenem z) scheiterte reproduzierbar an NEDs TAP-Dienst:
- Sync-Endpoint (`ned.ipac.caltech.edu/tap/sync`) hat ein **dokumentiertes**
  `executionDuration default 60` (VOSI gemessen) — große/sustained Abfragen
  brechen ab ("limited to 60 seconds", "stopped unexpectedly"), unter Dauerlast
  `void`.
- Async-Endpoint (`/tap/async`) lebt (303 + UWS-Job), aber die Queue staut sich
  unter Last (TOP 10 PENDING 10+ min); `retentionPeriod 0`, `outputLimit`
  1.000.000 Zeilen.
- Die Adresse ist **korrekt** (VOSI `capabilities` 200, base
  `ned.ipac.caltech.edu:443/tap`); die Config stimmt. Kein Config-Fehler.
- Kein öffentlicher Bulk-Download des objdir-z existiert (NEDL kein Format;
  einziges Dateiprodukt = NED-LVS, kuratierter 2-Mio-Sample). IRSA trägt keinen
  NED-Bestand; HyperLEDA heute unerreichbar.
- Stabile Alternative bereits gebaut: 2MRS (`J/ApJS/199/26/table3`) via
  `tapvizier.cds.unistra.fr` → `twomrs.bin` (44.599 Galaxien, cz). Die NED-Ernte
  ist pausiert, bis NED sich erholt.

Dieser Auftrag öffnet den externen Weg, den die anonyme Maschinen-Suche nicht
betreten kann: menschliche Rückfrage beim NED-Betreiber (NASA/IPAC) über
Dienst-Zustand, Bulk-z-Zugang und eine empfohlene, schonende Harvest-Route. Er
ist selbsttragend; wer ihn ausführt, liest vorab `docs/SOURCE_PORT.md` für die
Prüf-Kaskade. Es wird KEINE Datei geschrieben, kein Register editiert, kein
Oszillator-Gate entschieden — es wird nur benannt, was die Messung IST.

## Status-Vokabular (bindend)

- `live` — 200, offen, maschinenlesbar, Tabellen-Form (ra/dec + z oder cz).
- `blocked` — lebt, Zugang gesperrt (Konto/Antrag/Hilfepunkt).
- `declined` — lebt, aber keine freie Messung am Punkt (nur Figur/Metadaten).
- `pending` — Route bekannt, Antwort ausstehend.
- `not-published` — kein offener Weg nach abgeschlossener Suche.
- Für die Dienst-Frage zusätzlich: `erholt` / `degradiert` (NED-TAP-Zustand zum
  Messzeitpunkt, mit Datum).

Spekulationswörter sind verboten; ein ungemessener Befund heißt `pending`. Jeder
Befund trägt das Datum der Messung.

## Kanäle (in dieser Reihenfolge)

1. **NED-Betreiber (IPAC)** — die Rückfrage an den NED-Helpdesk (Kontakt über
   `ned.ipac.caltech.edu`, der Support-Link in der TAP-Doku; benenne die
   genaue Adresse erst nach Messung der Kontakt-Seite): (a) ist der TAP-Dienst
   bekannt degradiert, gibt es eine Wartung/Wiederherstellung (Zeitplan)?
   (b) gibt es einen **Bulk-z-Export** des objdir (FTP, Datensatz-Download,
   eine Datei mit ra/dec/z für die z-tragenden Objekte) — über die TAP hinaus?
   (c) welche Harvest-Route empfehlen sie (Sync-Seitengröße, MAXREC, Async,
   Raten-Limit), damit eine große aber schonende z-Ernte den Dienst nicht
   überlastet? Antwort = Messung.
2. **NED-Dokumentation/Status-Seite** — ist ein öffentlicher Service-Status
   (Uptime, Known Issues) vorhanden? Miss die NED-Homepage und die
   TAP-Dokumentation auf eine Status-/Wartungs-Meldung.
3. **Bulk-z-Route als Datei** — nur falls der Betreiber einen benannten
   Datensatz nennt: dessen Format + Größe + ra/dec/z-Spalten messen. Ohne eine
   solche benannte Datei bleibt der Weg `not-published`.

## Gesucht

Eine maschinenlesbare Tabellen-Form mit ra/dec + z (oder cz), offen oder mit
benanntem Antrag, die eine **schonende, stabile** Ernte trägt (das Himmelsgitter
aus z>0-Objekten), ODER eine benannte Dienst-Wiederherstellung des NED-TAP mit
einer empfohlenen Raten-Grenze. Ohne beides bleibt der Eintrag `not-published`
mit der Note, dass die stabile z-Ernte bereits über 2MRS-via-CDS
(`twomrs.bin`) läuft und NED-objdir ein tieferer, aber derzeit
dienst-begrenzter Zusatz ist (0 honored — nie eine erfundene Route).

## Rückgabe

Ein gemessener Verdikt je Kanal (Kontakt, HTTP, Format, Spalten, Antwort-Datum)
und ein End-Verdikt `live`/`blocked`/`declined`/`pending`/`not-published` (bzw.
`erholt`/`degradiert`) mit Datum. Keine Datei, kein Register-Edit.
