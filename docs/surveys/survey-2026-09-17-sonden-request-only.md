<!--
  title: Survey — Sonden request-only: die vier (Stand 2026-09-17)
  class: survey
  date: 2026-09-17
  sha256: cab5028c57c3769f1db69598c9350190172c8b2424e2279cbc6886be03c26b7b
  status: done
  see-also: docs/surveys/survey-2026-09-16-sonden-flotte.md docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md phi/blocked_sources.φ
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
  `GRV_OCRU_2013284_1527XMMMC005V01.ODF` (22 982 400 B). Abgleich geschlossen
  (gemessen 2026-09-17): der OCRU-Bestand ist 21 `.ODF` (2013-284…2016-137) und
  der eigene Ernte-Arm `--volume jnogrv_0001 --out …/juno_ocru_odf.bin`
  (`planetary-odf-cdn.yml`); `juno_odf.bin` trägt dagegen die disjunkte
  JUGR-Serie `jnogrv_1001` (2016-185…2017-244). Die Manifestation von
  `juno_ocru_odf.bin` + `sources.φ`-Block liegt bei der ernte-Linie
  (`handover-2026-09-17-ernte-folge66.md:99`).
- **Pioneer 10 — ATDF (Kontrast, nicht eines der vier):**
  `spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/pioneer10_doppler_tracking_SC_23.asc.gz`
  (19 820 702 B, ASCII-Kopf `ORBIT DATA DUMP`, DTYPE 12 = two-way / 13 = three-way)
  anonym offen — das DTYPE 12/13 ist der ASC/NAVIO-Code-Raum
  (`pioneer_doppler_compiler.rs`), nicht der ATDF-Wert (TRK-2-25 `DATA_TYPE` = 1|2).
  Belegt, dass die SPDF-ATDF-Route als solche existiert.

## Postfach (gemessen)

Die fünf formellen Anfragen (NSSDC Voyager/Mariner/Viking, JPL-NAV Cassini/Juno)
wurden 2026-09-16 gesendet (`state/mail/mail_ledger.φ:207–211`); **keine Antwort**
(jüngste Zeile 214 = NSE/SAMPLE_AUTHOR, 2026-09-16). CSES-Limadou: Sotgiu-Antwort
2026-09-16 („wait a few weeks", CSES-02-Umstellung; `:205,212`).

## Parser-Lücke (geschlossen 2026-09-17)

`src/archivar/odf.rs` dekodiert jetzt alle **18** TRK-2-34-Codes (`odf.rs:1639`
match; zuvor nur `format_code 0`/DT0); der native Serien-Arm ist gebaut
(`tools/harvest/src/bin/tnf_compiler.rs:71` → `write_podf_bin_bands`; `src/archivar/odf.rs:104` PODF). Die TRK-2-18-Spec (Text-Sidecar
`docs/reference/dsn_trk-2-18.1988-01-15.txt`) definiert das gesuchte Produkt: ODF,
Format ID 1, **Data Type 12 = Two-way Doppler**, Observable in Hz + Time Tag.
Referenz-Parser `NASA-PDS/PyTrk234` (`.tmp-trk234-components.txt`).

## Register-Disposition (Council 2026-09-17)

- Vier `pending`-Einträge **mit Ort** in `phi/blocked_sources.φ` — kein gemessenes
  Zugangs-Gate (kein 401/Key/Account), nicht `dead` („Ready for Offline
  Distribution"), nicht `declined` (Physik echt).
- Ein `blocked parser-def odf`-Eintrag (die 17-Code-Lücke) — seit 2026-09-17
  geschlossen (`odf.rs:1639`).
- Mariner `PSPA-00316` als akzeptierte Ernte-Duty (Geschwister-Format
  `voyager_saturn`, PSPA-00049, UNIVAC-1108); die 17-Code-Lücke berührt ihn nicht.
  Nachzug 2026-09-17: Arm + Workflow gebaut (`src/archivar/mariner_occlt.rs`,
  `.github/workflows/mariner-occlt-cdn.yml`). CDN-Dispatch geschlossen (gemessen
  2026-09-27 via `archive_search --sniff`): `spdf.gsfc.nasa.gov/mariner_occlt.bin`
  200, sha256 `4aa487cb…` = `phi/sources.φ:9271`.
- Pioneer-ATDF dtype-12/13 — kein Parser-Gap (gemessen 2026-09-17, folge63): der
  ATDF-Wert ist TRK-2-25 `DATA_TYPE` = 1|2; „DTYPE 12/13" ist der ASC/NAVIO-Raum.
  Der Juno-OCRU-Abgleich ist Ernte-/Bau-Arbeit, kein Gate.

## Vision-Befund (gemessen)

Der Vision-Leser konnte die zwei lokalen PDF-Specs
(`docs/reference/dsn_trk-2-18.1988-10-15.pdf`, `dsn_redr.2021-07-31.pdf`)
**nicht lesen** (`read`: Modell unterstützt keine PDF-Eingabe). Er las den
Text-Sidecar `dsn_trk-2-18.1988-01-15.txt` und extrahierte die ODF-Record-Struktur.
Für künftige Scan-/PDF-Atome: PDF ist kein Vision-Input; OCR braucht eine
Bild-Konversion.

### Nachtrag 2026-09-28 — die Bild-Konversion ist gebaut

Gegenprobe (gemessen 2026-09-28): `archive_search --pdf-image <file|url>`
(`docs/concepts/tool-forms.md`) liftet eingebettete JPEG/PNG/JP2 aus einem PDF
und schreibt sie als Dateien — an `docs/reference/gosat_l1b_format.pdf`
lieferte es 19 PNG. Der Weg für Scan-/PDF-Atome ist damit `--pdf-image` → `vision`
(das Bild-Input hat); `--pdf-text` bleibt für den Text-Layer. Die am 2026-09-17
gemessene Lücke „OCR braucht eine Bild-Konversion" ist gebaut
(Commits `ab604b962`, `c1a273ffc`).

## Benchmark

Die Route-Messung Mariner 10 lief über `research-max` und `grind-flash`: identischer
Befund (SPDF-`PSPA-00316`-Teilroute offen, Gesamtkorpus request-only; `grind-flash`
lieferte zusätzlich den Tar-sha256). Sieger `grind-flash` — flash-first bestätigt.

## Fazit

Kein Wert re-deriviert; jede Abwesenheit ist `request-only`/`pending` **mit Ort**.
Die vier bleiben offen, die fünf Anfragen laufen. Die Cassini-Präzedenz trägt für
Mariner 10 **nicht** (kein RSS-Volume am PDS-Rings-Node).

## Nachtrag 2026-10-20 — Zwilling-Suche mit Signatur, nicht mit Namen

Nachgemessen mit Produkt-IDs, ODF-Headern und Datei-Signaturen. **Kein Zwilling
öffnet eines der vier Ziele**, aber drei Belege im Text oben sind zu korrigieren:

| Ziel | Befund | Konsequenz |
|---|---|---|
| Voyager | `PSNO-00007` ist **nicht** closed-loop-RSS, sondern „SDDPT Voyager Project Data (PDS)", Voyager 2 **Imaging Science (ISS)**, „Ready for Offline Distribution" | der Beleg oben trägt die Doppler-Aussage **nicht**; echter Teil-Zwilling bleibt nur der Saturn-Encounter (`PSPA-00049`/`PSPA-00123`, 20 TARs, Univac-1108) — Cruise/Post-Saturn bleibt request-only |
| Viking | zwei offene JPL-SSD-Dateien: VL1 **Differenced Range** (916 920 B), VL1/VL2 **Range** (69 458 B) | Lander, prozessiert — **kein** Orbiter-RSS; `PSPG-00011` ist bei NSSDCA „Data Identified but not Received" |
| Juno | `JNOGRV_0001` beginnt **2013-284 (11.10.2013)**, `INDEX.TAB` bestätigt | die Pre-EFB-Lücke ist exakt **09.–10.10.2013**, nicht der ganze Cruise; kein Pre-EFB-ODF in PDS/Zenodo/DataCite/JPL-SSD |
| Mariner 10 | `ser.im-ldi.com/M10/MVM/mvm_0001.tgz` (144 349 050 B) ist eine **Namenskollision**: Imaging-EDR-Tapes `MVM_00xx`; `mvm_1001.tgz` = **404** | nicht als Radio-Science-Zwilling verwenden; offen bleibt nur `PSPA-00316` (Venus-Okkultation) |
| CSES | offener Zwilling nur **CSES-01 MAG** (DTU, Tages-CDFs); Portale registrierungsgated | für CSES-02 (Sotgiu) **kein** Ersatz |

**Lizenz:** für keinen der Kandidaten ist ein admitierter Token belegt — durchgehend
`absent`. Der Kontrastfall Pioneer 10/11 (Zenodo, CC-BY-4.0) zeigt, dass es für
diese Datenklasse möglich wäre.

**Postfach:** 5 von 5 Anfragen (Voyager, Mariner, Viking, Cassini, Juno; gesendet
2026-09-16) sind nach 34 Tagen **ohne Antwort**. Kein Zwilling macht eine davon
überflüssig. CSES/Sotgiu ist keine dieser fünf (Antwort: „wait a few weeks").
