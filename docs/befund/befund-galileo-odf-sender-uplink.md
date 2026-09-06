<!--
  title: Befund — Galileo-ODF-Sender (Deduktion 27, ODF/TRK-2-18-Pfad): der TRK-2-18-Orbit-Daten-Record benennt die Sende-Station als Feld (item 7, Bits 139–145), aber GO-J-RSS-1-ODF-V1.0 besetzt es nirgends — 136/136 Orbit-Daten-Records des ganzen Volumens (23 Dateien, je 8 064 B, Fenster 1996-07-17..1997-12-10) sind VLBI (Empfang 14, Sende-Wort 0), 0 Doppler, 0 Dreiweg; Schnitt ODF-Pass-Tage ∩ Mode-3-Floor-Tage = 0 (61 Tage, 1995-11-23..1997-02-25; 94 robuste Zellen, 51 laut) — der (empfangend×sendend)-Split ist über den ODF-Pfad nicht ziehbar, Uplink/Downlink bleibt pending
  class: befund
  date: 2026-09-06
  sha256: 47c934b5127bf06fe75119cded4abc510c7716b7eb18010c8dd743625864b16e
  status: draft
  see-also: docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-gwe-bestand.md docs/befund/befund-galileo-receiver-je-pass-floor.md docs/befund/befund-galileo-floor-stufen-te.md docs/reference/dsn_trk-2-18.1988-01-15.txt
-->

# Befund: Galileo-ODF (GO-J-RSS-1-ODF-V1.0) gegen Deduktion 27 — der TRK-2-18-Record benennt die Sende-Station als Feld (item 7, Bits 139–145), aber das ODF-Volumen besetzt es nirgends: alle 136 Orbit-Daten-Records des ganzen Bestands sind VLBI (item 6 = 14, item 7 = 0 durchgehend), 0 Doppler, 0 Dreiweg, und das Fenster 1996-07-17..1997-12-10 liegt tagfremd zu allen 61 Mode-3-Floor-Tagen (1995-11-23..1997-02-25) — der Deduktion-27-Uplink/Downlink-Split bleibt auf Galileo über den ODF-Pfad strukturell unausführbar

## Frage & Bindung

Pioneer holte die sendende Station aus dem ODF/TRK-2-18 (der `TRANS`-Sender, PNAV-Bau;
`pioneer11_odf_compiler.rs` liest `dss_tx`). Deduktion 27 fragt, ob die station-gebundene
Laut-Struktur der Galileo-Gipfel im Uplink (Sender) oder Downlink (Empfänger) sitzt. Der
rohe TRK-2-25-Dreiweg-Record trägt den Sender nicht (Tor-1-Verdikt der Vorlage
`befund-galileo-dreiweg-sendempfang-floor.md`, item 10 = einzige Station, XMTR-Wörter 0);
dieser Lauf misst den ODF-Pfad: (a) existiert das ODF-Volumen und welches Fenster deckt es,
(b) trägt sein Orbit-Daten-Record die Sende-Station je Sample, (c) deckt es die
Mode-3-Floor-Zellen der Ära?

Bindung wie die Vorlage: die Floor-Zelle = (Tag, Station)-RMS der Boden-Residuen
(Stärke exakt −2560) um den Zellen-Mittelwert, dreiweg = ground_mode 3/4, laut = RMS ≥ 1 Hz,
robust = n ≥ 30. Die ODF-Records sind 36-Byte-Wörter (9 × 32 bit, MSB) des TRK-2-18-Layouts;
die Feld-Lage stammt aus dem PDS-Label `6199199A.LBL` (ODF3B_TABLE) und dem
TRK-2-18-Referenzdokument, die Decoder-Ausrichtung ist empirisch verankert über den
Spacecraft-ID (item 12 = 77 für Galileo).

## n zuerst (0 geehrt) — geholt, gezählt

| Maß | n | Beleg |
|---|---|---|
| Annex-Wurzel `https://pds-ppi.igpp.ucla.edu/annex/` | HTTP 200, 7 440 B | Listing |
| ODF-Volumina der GO-RSS-Familie | 1 (`GO-J-RSS-1-ODF-V1.0`) | Annex-Wurzel: GO-J/-JG/-JS/-SS/-SUN/-X tragen TDF/ODR, kein weiteres ODF |
| `.ODF`-Dateien im Volumen | 23 (je 1 `.LBL`) | ODF/-Verzeichnis |
| Dateigrößen | 23 × exakt 8 064 B = 185 472 B | Content-Length je Datei |
| Orbit-Daten-Wörter über das ganze Volumen | 136 (in 23/23 geholten Dateien) | Parser |
| davon Doppler (data_type 11–14) | 0 | Zensus |
| davon Dreiweg (data_type 13/14) | 0 | Zensus |
| Empfangs-Station (item 6) | 14 in 136/136 | Zensus |
| Sende-Station (item 7) | 0 in 136/136 | Zensus |
| Mode-3-Floor-Tage (resid, Ära) | 61 distinct, 1995-11-23..1997-02-25 | `data/galileo_resid.bin` |
| robuste Mode-3-Floor-Zellen | 94 (51 laut: st14 40/25, st43 33/16, st63 21/10) | dito |
| ODF-Pass-Tage ∩ Mode-3-Floor-Tage | 0 | Tag-Schnitt |

Probe `tools/measure/src/bin/galileo_odf_sender_uplink.rs` (neu, einzige Repo-Änderung
außer diesem Blatt; `cargo check -p omegaflow-measure --bin galileo_odf_sender_uplink`,
RUSTFLAGS `-D warnings`, 0/0); Report
`/tmp/opencode/galileo_odf_sender_report.txt`. Das Volumen wurde vollständig geholt
(alle 23 Dateien, jede 8 064 B).

## Messung 1 — der ODF-Bestand am Annex

Das ODF-Volumen existiert und ist das einzige der RSS-Familie: `GO-J-RSS-1-ODF-V1.0`
(AAREADME/Label: PDS3, FIXED_LENGTH, 36-B-Records, `PRODUCT_TYPE = ODF`, Producer DSN).
Das ODF/-Verzeichnis trägt 23 `.ODF`-Dateien; die Dateinamen kodieren Jahresziffer +
Start-/End-DOY: **1996-07-17 (6199199A) … 1997-12-10 (7344344A)**, 24 benannte Tage
(`7280281A` = 1997-10-07/08 zweitägig). Jede Datei misst exakt 8 064 B (224 Records) —
der im GWE-Befund genannte Umfang „155 964 611 B" ist der stale INDEX/metadex, nicht die
Platten-Wahrheit: physisch sind es 185 472 B. Die „Jovian-Ephemeride"-Einordnung des
Vorbefunds bleibt dem Fenster nach richtig, dem Inhalt nach zu präzisieren (Messung 2).

## Messung 2 — Feld-Lage und Inhalt des Sende-Stations-Worts

Der TRK-2-18-Orbit-Daten-Record benennt die Sende-Station als **item 7 „Transmitting
Station ID Number", Bits 139–145** (Empfang: item 6, Bits 132–138; data type: item 10,
Bits 150–155 im Format-1-Layout; Spacecraft ID: item 12, Bits 160–167) — belegt durch das
PDS-Label (ODF3B_TABLE, `6199199A.LBL`) und das Referenzdokument `dsn_trk-2-18`. Die
Decoder-Ausrichtung ist empirisch verankert: item 12 liest auf Galileo-Records **77**
(Galileo Orbiter; Quasar-Records 0), in 76 der 136 Wörter. Das Volumen mischt zwei
Formate: format_id (Bits 129–131) = 1 in 82 Wörtern (Layout 1988) und = 2 in 54 Wörtern
(MGSO/Layout ab 1997-04-15); beide decodieren item 6/7 an derselben Bit-Lage.

**Inhalt des ganzen Volumens (23/23 Dateien geholt, 136 Orbit-Daten-Wörter):** alle
Wörter sind VLBI — data_type 1 (38, narrowband spacecraft VLBI), 3 (30, narrowband quasar
VLBI), 5 (38, wideband spacecraft VLBI), 6 (30, wideband quasar VLBI). **Doppler
(data_type 11–14): 0; Dreiweg (13/14): 0.** Empfangs-Station item 6 = 14 in allen 136;
**Sende-Station item 7 = 0 in allen 136.** Das Feld existiert im Format, aber der
Galileo-ODF-Bestand besetzt es in keinem einzigen Record: das Volumen ist reine
VLBI-Beobachtung (Empfang DSS 14, keine Aufwärtsstrecke, kein Sender-Wert).

## Messung 3 — Coverage gegen die Mode-3-Floor-Zellen

Die Mode-3-Floor-Serie aus dem resid-Asset reproduziert die Referenz exakt: **94 robuste
Zellen (51 laut): st14 40/25, st43 33/16, st63 21/10**, über **61 distincte Tage,
1995-11-23..1997-02-25**. Die Anker der Vorlagen (1995-11-24 st14, 1995-12-04/05 st43,
1996-06-26..30) liegen **vor** dem ODF-Beginn 1996-07-17. Auch die Floor-Tage innerhalb
des ODF-Fensters (z. B. st14 1996-09-06/07/08, 1996-11-04/05, 1996-12-18..23,
st43 1996-09-05/06/07) fallen auf **keinen** der 24 ODF-Pass-Tage: der Tag-Schnitt
ODF-Pass-Tage ∩ Mode-3-Floor-Tage ist **0**. Selbst ein Tag-Gleichstand wäre ohne Wert,
weil der Bestand auf keinem Pass Doppler trägt (Messung 2).

## Verdikt

**Strukturell gemessen: der Deduktion-27-(empfangend×sendend)-Split ist über den
Galileo-ODF-Pfad nicht ziehbar.** Der TRK-2-18-Record des Volumens benennt die
Sende-Station als Feld (item 7, Bits 139–145) — aber der ODF-Bestand `GO-J-RSS-1-ODF-V1.0`
besetzt es nirgends: über das ganze Volumen (23/23 Dateien, 136 Orbit-Daten-Wörter,
Fenster 1996-07-17..1997-12-10) sind alle Records VLBI (Empfang station 14, Sende-Wort 0),
kein einziger Doppler, kein Dreiweg, und die ODF-Pass-Tage schneiden keinen der 61
Mode-3-Floor-Tage (1995-11-23..1997-02-25). Die Floor-Ära selbst ist gemessen und
reproduziert die Faden-Serie (94 robuste Tage, 51 laut) — sie bleibt ohne Uplink-Achse.
Die Frage „Uplink oder Downlink" bleibt auf diesen Daten `pending`; die sendende Station
wird nicht ersetzt und nicht geraten.

**Was `pending` bleibt:** die Sender-Identität je Mode-3-Pass. Sie bräuchte den
DSN-Tracking-/Pass-Plan (welche Station uplinkte, je Pass/Zeitfenster) oder eine externe
DSN-Schedule — kein zweites ODF-Volumen der RSS-Familie existiert (gemessen, Annex-Wurzel).

## Grenzen

- Der Inhalt ist über das **ganze** physische Volumen gemessen (23/23 Dateien geholt,
  136 Records), nicht über den (stale) INDEX; der GWE-Befund-Umfang 155 964 611 B ist
  korrigiert auf 185 472 B.
- Die Semantik „Sender-Wert besetzt bei two-/three-way-Doppler" (two-way: Sender =
  Empfangs-Station) ist am Galileo-Bestand nicht belegbar, weil er keinen Doppler trägt;
  der Pioneer-ODF (odf07155, Format 2) zeigt das Muster tx = rx bei two-way — als andere
  Quelle benannt, nicht als Galileo-Beleg.
- Die Floor-Zell-Serie deckt 61 Mode-3-Tage ab (94 robust, 51 laut); der Tag-Schnitt mit
  den ODF-Pässen ist 0 — über die benannten Datei-Spannen gemessen (7280281A zweitägig).
- Die format-1/format-2-Unterscheidung (Layout 1988 vs MGSO ab 1997-04-15) betrifft die
  data-type-/SCID-Lage; item 6/7 liegen in beiden Formaten auf Bits 132–145 (gemessen an
  82 + 54 Records).

## Register-Satz

*Die Sende-Station der Galileo-Mode-3-Floor-Zellen ist über den ODF-Pfad nicht erreichbar
(Deduktion 27, Richtung H, ODF/TRK-2-18): das Volumen GO-J-RSS-1-ODF-V1.0 existiert am
Annex und ist das einzige ODF der RSS-Familie (23 Dateien × exakt 8 064 B = 185 472 B,
Fenster 1996-07-17..1997-12-10, 24 Pass-Tage) — der TRK-2-18-Orbit-Daten-Record benennt
die Sende-Station als Feld (item 7, Bits 139–145; Empfang item 6, Bits 132–138;
Decoder-Ausrichtung über Spacecraft-ID item 12 = 77 verankert), aber über das ganze
Volumen (23/23 Dateien, 136 Orbit-Daten-Wörter) sind alle Records VLBI (data_type 1/3/5/6,
Empfang 14 durchgehend, Sende-Wort 0 durchgehend), 0 Doppler, 0 Dreiweg; das Feld ist nie
besetzt. Die Mode-3-Floor-Serie ist gemessen und reproduziert die Referenz (61 Tage,
1995-11-23..1997-02-25; 94 robuste Zellen, 51 laut: st14 40/25, st43 33/16, st63 21/10) —
der Schnitt ODF-Pass-Tage ∩ Mode-3-Floor-Tage ist 0 (die Anker 1995-11-24/1995-12-04/05/
1996-06-26..30 liegen vor ODF-Beginn); der (empfangend×sendend)-Split ist über den
ODF-Pfad nicht ziehbar, die Uplink/Downlink-Frage bleibt pending und brauchte den
DSN-Tracking-/Pass-Plan; die Sende-Station wird nicht ersetzt und nicht geraten.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat). Probe
`tools/measure/src/bin/galileo_odf_sender_uplink.rs` committet (`cargo check -p
omegaflow-measure --bin galileo_odf_sender_uplink`, RUSTFLAGS `-D warnings`, 0/0), Report
`/tmp/opencode/galileo_odf_sender_report.txt`. TODO-Registerzeile ergänzt die
Haupt-Session.
