<!--
  title: Survey — Sonden request-only: die vier (Stand 2026-09-17)
  class: survey
  date: 2026-09-17
  sha256: df570ea53ffe2886932bd7a54a1a1d8fd744aeddff646ad6efcc4c737ac1d69d
  status: live
  see-also: docs/surveys/survey-2026-09-16-sonden-flotte.md docs/auftrag/auftrag-sonden-rohdaten-anfrage.md phi/blocked_sources.φ
-->
# Survey — Sonden request-only: die vier (Stand 2026-09-17)

Anlass: die vier verbleibend **request-only** Roh-Tracking-Daten (Voyager 1/2,
Mariner 10, Viking 1/2, Juno pre-EFB) wurden mit der vollen Taucher-Riege
(`grind-flash`, `research-max`, `vision`; harte Bandagen Playwright + `curl -k` +
`--cacert` + Proton-Vorschlag) neu gemessen — **Cassini ausgenommen** (läuft in
der Ernte: `phi/sources.φ` TRK-2-34-Bundles, Post an ernte). Methode = Herkunft;
kein Wert abgeleitet, keine Zahl re-deriviert.

## Ergebnis je Ziel

| Ziel | Produkt | gemessene Route(n) | Verdikt |
|---|---|---|---|
| Voyager 1/2 | closed-loop two-way Doppler (TRK-2-34/ODF/ATDF) | PDS-PPI nur open-loop `.ODR` (ROCC-Volumes, PDS4 `voyager*_rss_jupiter_raw`); SPDF nur Okkultations-Produkte; PDS-ATDF-Bundles existieren für Galileo/Magellan/Pathfinder/Ulysses/LRO, **nicht** Voyager; NSSDC `PSNO-00007` | **request-only bestätigt** |
| Viking 1/2 | Doppler + Range | NSSDC `PSPG-00011` („Data Identified but not Received"), `PSPG-00457`/`00277`/`00406` („Ready for Offline Distribution"); NSSDC-FTP anonym nicht bedienbar (Redirect → SPDF 404); PPI 0 Viking; SPDF nur `sweden/` + `lander/` | **request-only bestätigt** |
| Juno | pre-EFB merged ODF (2013-10-09) | PDS3 `JNOGRV_0001` (OCRU) trägt nur **post-EFB** (START 2013-284 = 2013-10-11); `juno_merged_odf_2013_preefb.dat` = 404; kein Alternativ-Volume | **request-only (pre-EFB) bestätigt** |
| Mariner 10 | 762 7-Track-Tapes (SDDPT 73-085A-0008, `MVM_1001–1007`) | `PSCM-00009` request-only; `download.action` = NSSDCA-Fehlerseite; PPI/DITDOS 0; NMSU „None were archived in standard PDS format"; PDS-Rings kein Mariner 10 | **request-only (Gesamtkorpus) bestätigt** |

## Die offenen Teilrouten (gemessen, nicht die Vollbestände)

- **Mariner 10 — Venus-Okkultation (reduziert):**
  `spdf.gsfc.nasa.gov/pub/data/mariner/mariner10/celestial_mechanics_and_radio_science/red_tele_signal_data_venus_occlt/`
  — 9 `.tar` à ~19,5 MB anonym; `PSPA-00316_DD029604_05-FEB-74.tar` = 19 619 840 B,
  sha256 `065cd743e6b1c4dd0c729dba21b732909f1e4bbcef08a92526ffec81ccae9a04`;
  `attrib`: `PRIMARY_COLLECTION_ID PSPA-00316`, Original Univac 1108, 7-Bit-Stream.
  Reduced Venus-Daten, **nicht** die 762 Tapes.
- **Juno — post-EFB OCRU:** `atmos.nmsu.edu/PDS/data/jnogrv_0001/` (PDS3
  `JNOGRV_0001`) — merged ODFs `2013_postefb`/`2014`/`2015`/`2016`; früheste
  `GRV_OCRU_2013284_1527XMMMC005V01.ODF` (22 982 400 B). Der Bestand steht als
  `juno_odf.bin`; Abgleich Ernte↔Bestand offen.
- **Pioneer 10 — ATDF (Kontrast, nicht eines der vier):**
  `spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/Data/ATDF_Data-Files_CMarkwardt_Readable/`
  — `pioneer10_doppler_tracking_SC_23.asc` (63 MB, ASCII-Kopf `ORBIT DATA DUMP`,
  DTYPE 12 = two-way / 13 = three-way) anonym offen. Belegt, dass die SPDF-ATDF-
  Route als solche existiert.

## Postfach (gemessen)

Die fünf formellen Anfragen (NSSDC Voyager/Mariner/Viking, JPL-NAV Cassini/Juno)
wurden 2026-09-16 gesendet (`state/mail/mail_ledger.φ:207–211`); **keine Antwort**
(jüngste Zeile 214 = NSE/Haug, 2026-09-16). CSES-Limadou: Sotgiu-Antwort
2026-09-16 („wait a few weeks", CSES-02-Umstellung; `:205,212`).

## Parser-Lücke (unverändert)

`src/archivar/odf.rs` dekodiert nur `format_code 0` (DT0); die übrigen
17 TRK-2-34-Codes + ein nativer Serien-Arm fehlen. Die TRK-2-18-Spec (Text-Sidecar
`docs/reference/dsn_trk-2-18.1988-01-15.txt`) definiert das gesuchte Produkt: ODF,
Format ID 1, **Data Type 12 = Two-way Doppler**, Observable in Hz + Time Tag.
Referenz-Parser `NASA-PDS/PyTrk234` (`.tmp-trk234-components.txt`).

## Register-Disposition (Council 2026-09-17)

- Vier `pending`-Einträge **mit Ort** in `phi/blocked_sources.φ` — kein gemessenes
  Zugangs-Gate (kein 401/Key/Account), nicht `dead` („Ready for Offline
  Distribution"), nicht `declined` (Physik echt).
- Ein `blocked parser-def odf`-Eintrag (die 17-Code-Lücke).
- Mariner `PSPA-00316` als akzeptierte Ernte-Duty (Geschwister-Format
  `voyager_saturn`, PSPA-00049, UNIVAC-1108); die 17-Code-Lücke berührt ihn nicht.
- Pioneer-ATDF dtype-12-Arm (`atdf.rs:503/636` hält nur dtype 1|2) und der
  Juno-OCRU-Abgleich sind Ernte-/Bau-Arbeit, kein Gate.

## Vision-Befund (gemessen)

Der Vision-Leser konnte die zwei lokalen PDF-Specs
(`docs/reference/dsn_trk-2-18.1988-10-15.pdf`, `dsn_redr.2021-07-31.pdf`)
**nicht lesen** (`read`: Modell unterstützt keine PDF-Eingabe). Er las den
Text-Sidecar `dsn_trk-2-18.1988-01-15.txt` und extrahierte die ODF-Record-Struktur.
Für künftige Scan-/PDF-Atome: PDF ist kein Vision-Input; OCR braucht eine
Bild-Konversion.

## Benchmark

Die Route-Messung Mariner 10 lief über `research-max` und `grind-flash`: identischer
Befund (SPDF-`PSPA-00316`-Teilroute offen, Gesamtkorpus request-only; `grind-flash`
lieferte zusätzlich den Tar-sha256). Sieger `grind-flash` — flash-first bestätigt.

## Fazit

Kein Wert re-deriviert; jede Abwesenheit ist `request-only`/`pending` **mit Ort**.
Die vier bleiben offen, die fünf Anfragen laufen. Die Cassini-Präzedenz trägt für
Mariner 10 **nicht** (kein RSS-Volume am PDS-Rings-Node).
