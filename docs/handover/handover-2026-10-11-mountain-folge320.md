<!--
  title: Handover — Mountain-Folge 320 (2026-10-11)
  session: Mountain-Linie in einem Pass — zwei Register-Form-Risse durch Rat + Research + UI-Roster entschieden (McPherron-Onset-Liste → witness point-event, Riss gegen River gemessen: die Liste trägt SML-Werte/Bay-Properties, nicht nur Onsets; THEMIS-ASI-Stationstabelle → Compiler-Eingabe, Riss asilib-23 vs Berkeley-20) + 7 Quellen-Terms/Verdikte gemessen (OE-417/THEMIS-ASI/PCN/CSES/BGS-GIC/NERC-GMD/McPherron) + McPherron-Schema exakt gemessen + `MCON`-Punkt-Event-Form gebaut + Future-236s Rätsel/Mariner gefaltet (Mariner-Rohbänder gemessen: GNU-tar/IBM-7094-Framing, M10-imaging-EDR 968-B + DSC-73-M6/7-Layout via 2 Vision-Reads, `NSSDC_USER`/`NSSDC_PASS` bestätigt; Mariner-10-Imaging-EDR-Format `mariner_m10_edr` + Compiler gebaut, `cargo check` 0/0; M6/7-Packing gemessen (6-bit-Zeichen, 6 B/Wort, Zeit→1969) + `mariner_rst`-Parser/Compiler gebaut, lokal auf M6/M7 verifiziert; DSC 0119 (M4-Feldtabelle) aus future-238 gefaltet)
  class: handover
  date: 2026-10-11
  sha256: 5f1e6c488890916321fb38f4a06ddba32f78712b177d5c74d096a5e6db25236b
  status: live
-->
# Handover — Mountain-Folge 320 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; der dort
genannte HEAD war `537c5dc09`, aktueller HEAD dieser Session `7517e6016`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge319.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.24 · cap 0.50 · reason: Linien-Session + 1 general-Taucher (Quellen-Terms) + 1 grind-flash-Taucher (McPherron-Schema) + 1 council (2 Form-Risse) + 2 UI-Seats (Claude Sonnet 5.5 Extra hoch · GLM-5.3 Deep Think Max) + `archive_search --all`/Fach-Modi als Research-Schicht — gemessen via `session_burn`, deepseek-flash, kein pro/max.

## Erledigt in diesem Atom (gelöscht aus dem Register; git trägt es)

- **Register-Form Q1 entschieden — McPherron-Onset-Liste als `witness point-event`.** Durch Rat (5 Stimmen) + Research-Schicht + UI-Roster konvergent: **kein neues Grammatik-Arm**. Der Oszillator ist das Ereignis an seinem Epoche (τ=0, extent=0, 0 honored); Träger = `witness point-event`, `record mcon`, `force em`, `no-cadence`. Der Compiler bleibt `mcpherron_compiler.rs`; `MCON` tritt in den Point-Event-Arm (`src/archivar/witness.rs:20-22`) + Gate-Test im selben Atom.
- **Riss gegen River-187 gemessen und benannt (McPherron-Inhalt).** Die Behauptung „reiner Onset-Zeit-Katalog, keine Feldwerte" ist **widerlegt**: die drei Dateien tragen SML-Werte und Bay-Properties (s. u.). Damit entfällt die council-seitige Bedingung „per-Bin-Zählung via `.te`-Deskriptor" — **jedes Ereignis trägt seinen gemessenen SML-Skalar**, kein Bin nötig.
- **Register-Form Q2 entschieden — THEMIS-ASI-Stationstabelle ist Compiler-Eingabe**, kein `record`/`witness` (nackte Koordinate ohne Wert bleibt verworfen, `die-weberin.md:127-129`). Compiler-Defekt benannt: `themis_asi_keogram_compiler.rs:112` muss `on earth <lat> <lon> <alt>` + `station <CODE>` emittieren (MIRACLE-Form, `phi/sources.φ:20287`). Riss a (asilib `asi_locations.csv`, 23) vs b (Berkeley `thm_asi_stations.pro`, 20, abweichende Menge, lon 0–360): **a ist autoritativ** (gleicher `<code>`-Raum wie die UCalgary-Labels), b als zweite Zeugenlinie getragen; fehlendes `alt` → Station bleibt `pending`, nie `0.0`.
- **7 Quellen-Terms/Verdikte gemessen (general-Taucher, `--verdict`/`curl`/Zenodo-API):**
  - **OE-417** (`doe417.pnnl.gov`) — **not-admit**: die 27-Box-Cause-Taxonomie trägt **keine** geomagnetische Kategorie; Rohdatensätze hinter Microsoft-Entra (OIDC). Nicht als GMD-Quelle führen.
  - **THEMIS-ASI** (aurorax + UCalgary) — **admit**: data.phys.ucalgary.ca keyless lesend, offen mit Zitatpflicht (kein SPDX-String); `ttl 604800`. Bestehender Berkeley-Block `phi/sources.φ:20889-20895` (`terms free-open`).
  - **PCN** (DTU WDC) — **admit**: CC BY 4.0 (im Datei-Header `ftp.space.dtu.dk/WDC/indices/pcn/PCN_definitive/`), `ttl 31536000` (definitive, 1975–2024) / `86400` (quicklook).
  - **CSES** Zenodo `13329652`/`8082201` — CC BY 4.0, `ttl 31536000`.
  - **BGS-GIC** — **not-admit**: nur der B_GIC-Index als Bild + modelliertes UK-Feld (PNG), **keine** maschinenlesbare Messreihe.
  - **NERC-GMD** — **not-admit / blocked-account**: `eroportal.nerc.net/gmd-event-list` liefert die ERO-Portal-„System Error"-Seite, Konto-Gate + BotDetect; keine offenen Records.
  - **McPherron** — cc-by-4.0, `ttl 31536000` (statisches Archiv), **admit als Daten**.
- **McPherron-Schema exakt gemessen (grind-flash-Taucher, `unzip`/`tr`/`awk`):** `CorrOnsFiles.zip` (md5 `52951d852f4d5bf7ac092a7d304f2b26` bestätigt) → `ftims.txt` (11 Spalten: 6 Zeit + 5 Relativzeiten in min), `fvals.txt` (11 Spalten: 6 Zeit + **SML-Werte** an den 5 Zeitpunkten), `fadd.txt` (10 Spalten: 6 Zeit + **Amplitude nT, Bay-Depth nT, Bay-Duration min, Bay-Width min**); **314 560 Ereignisse**, 1966-01-01→2024-05-29, Zeit = `[year month day hour minute second=00]` (Kalender-Vektor, **kein** doy/Epoche), `NaN`-Sentinel für unbestimmte Werte → `Option`, nie `0.0`. Kein MLT/MLAT/GLON/GLAT, kein Onset-Typ-Flag.
- **`MCON`-Punkt-Event-Form gebaut (Mountain, dieser Atom).** Neues Modul `src/archivar/mcon_event.rs` (MAGIC `MCON`, Header 13 B, Record 32 B, `present`-Bits, Komponenten `mcon_sml_nt`/`mcon_amplitude_nt`/`mcon_bay_decrease_nt`/`mcon_bay_duration_min`/`mcon_bay_width_min`), `extract.rs`-Arm `"mcon_event"`, `MCON` im Punkt-Event-Arm `witness.rs` (+ Test `transient_event_records_are_point_events`), Compiler `mcpherron_compiler.rs` auf ftims/fvals/fadd-Umbau (Join per Zeile, `NaN`→absent). `cargo check` **0/0**; Compiler-Lauf gegen das Archiv: **314 555 Onsets** (10 065 773 B), sha256 `441df5f8c59ad57962d81cc82814a0d36c7770f827b4d1893b31425f035cc3f1`. 5 Zeilen ohne gemessenen Skalar bleiben aus (BareCoordinate, 0 honored).
- **Mariner-Rohbänder gemessen + per Vision entschlüsselt (future-236 `## An mountain` gefaltet).** (1) **Framing (grind-flash):** die SPDF-Tars sind **GNU-tar** mit genau zwei Membern `<name>.dir/<name>_F1.DAT`; kanonischer Stream = **`u16 BE Recordlänge + Payload`**; `attrib/<name>_att` (PVL, `NSSDC_ATTRIBUTE_OBJECT` v3) trägt **keine Feldtabelle**, nur `FORMAT_IDENTIFIER` NSSD1311 (M6) / NSSD1310 (M7) / NSSD1312 (M4), `MACHINE_REPRESENTATION = IBM 7044/7090/7094`, `MAXIMUM_RECORD_LENGTH_BYTES` 1208/1112/908; Tar-sha256 je Datei gemessen. (2) **Mariner-10-Imaging-EDR (ASU-Spiegel):** Vision-Read `pict_app_a.pdf` (PICT Appendix A, JPL-Pub 85-50) + `aareadme.txt` ergeben exakt: logischer Record **968 B = 832 B Pixellinie + 136 B Engineering**; **22 Blöcke × 31 944 B = 702 768 B/Datei**; Block 1 = VICAR-Label (360 B, **EBCDIC**) + Filler (608 B); 700 Zeilen, 832×700, 8 bit; 231 Dateien max. (3) **Mariner 6/7-Tracking:** Vision-Read `dsc_0073.pdf` (NSSDC DSC-73) — **IBM-7094-36-bit-Wörter**; Header-Records (FILE LABEL BCD · MISSION NUMBER BI · 15 Stationen × Datentyp-Zähler BI · Startzeiten DPFP), Datensatz-Records mit **CODES-WORD** (Bits 1–15 = anwesende Datentypen) + DPFP-Felder; die NSSD1312-Spec (M4) bleibt `pending` (kein DSC im digitalisierten Satz). (4) **Secrets:** `NSSDC_USER`/`NSSDC_PASS` existieren in `.secrets.local` (gemessen via `bin/secrets_keys`, Namen only).
- **Mariner-10-Imaging-EDR: Format + Parser + Compiler gebaut (Mountain).** (a) **`.tgz` gemessen (grind-flash):** `mvm_0001.tgz` = gzip-tar, 144 349 050 B, **1367 Member** (11 Verzeichnisse + 1356 Dateien); **677 Bilddateien** `MVE_00N/IMAGES/MVE_00N.NNN` à **702 768 B** = 726 × 968 B; Records sind **nackte 968-B-Einheiten** ohne Längenpräfix; Record 0 = 360-B-EBCDIC-VICAR-Label, Records 1–700 = Bildzeilen (B[968·(r+1)+c]), Records 701–725 = Null-Pad. (b) **Gebaut:** `src/archivar/mariner_m10_edr.rs` (MAGIC `M10E`, Spaltenmittel-Bin, `parse_bin`/`write_bin` + Tests), `extract.rs`-Arm `"mariner_m10_edr"`, Compiler `mariner_m10_edr_compiler.rs` (gunzip → `tar_members` → je Bild EBCDIC-Label-`ERT YR/DAY/GMT`-Epoche + 832 Spaltenmittel über 700 Zeilen). `cargo check` **0/0**, Bin baut clean. Der volle 144-MB-Lauf ist **CI** (schwere Rechnung gehört nicht auf die Operator-Maschine).
- **DSC 0119 gefaltet (future-238 `## An mountain`) — die M4-Blockade ist widerlegt.** Der SPDF-DSC **0119** (`spdf.gsfc.nasa.gov/…/renamed_dataset_catalogs/mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf`, HTTP 200, 1 380 249 B, sha256 `ec3c41ebbb082287437fe6ff9b9ba91a7fd9474311b274d8cbd975ddd5ce55b0`) existiert und **trägt die Formatbeschreibung** (via `--pdf-image` + `vision`, 16 S.): `SECTION II` ODG-Data-File (Master-Label 9 Wörter BCD + Check-Sum, Station-Summary 201 Wörter, Datentypen 1–15: Range km · Range-Rate · EL/AZ/DEC/HA-RA ° · Doppler C1/C2/C3 CPS · TR µs · RU · PRU ns) + `APPENDIX I` ODP-Logical-Record (Wort 0 #Wörter · 1–2 doppeltgenaue Zeit s seit 1950.0 · 4 Frequenzinkrement · 5 Code Word Bit↔Datentyp · 7 Weight Code Word) + `Fig. 2`; Reorder 65-205. Dataset `64-077A-09A` (Celestial Mechanics, IBM 7094, 556 BPI ODD 7-Track, 2 Tapes `D-01528`/`D-01529`). Der Code „NSSD1312" steht auf **keiner** der 16 Seiten (interner Identifier); `TX OTD900-1`/`TR32-1106` referenziert, **nicht online**.
- **Mariner-6/7-Packing gemessen + `mariner_rst` gebaut (Mountain, dieser Atom).** **Messung (grind-flash):** die Länge ist **in Bytes** (`2 + L`-Walk konsumiert jedes `*_F1.DAT` exakt), der Payload ist **6-bit-Zeichen, eins pro Byte** (kein Byte > 0x3F), ein 36-bit-Wort = **6 Zeichen BE = 6 B/Wort**; Wort 0 Bits 3–17 = Wörter−1 (100 %); Header-Größen exakt (60/1206/582 B = 10/201/97 Wörter); **Zeit** = Wörter 2–3 als 72-bit-Float (Vorzeichen 1 · Exp 8 über-128 Basis 2 · Fraktion 63) = Sekunden seit 1950 → **1969** (M6 1969-03-01…08-29, M7 1969-04-08…09-03, deckt die Attribut-Spannen). **Gebaut:** `src/archivar/mariner_rst.rs` (MAGIC `MRST`, normalisiertes Serien-Bin) + `mariner_rst_compiler.rs` (Walk → 6-bit-Wörter → Zeit + CODES-Word → Observablen als IBM-Hex-Single) + `extract.rs`-Arm; `cargo check` **0/0**. **Lokal verifiziert:** M6 `DD005408` → **55 900 Observablen** 1969-03-01…08-29; M7 `DD005410` → **38 572** 1969-04-08…09-03.

## Offen (aufgeschlüsselt)

### Mariner-6/7-Tracking (`mariner_rst`) — Zusatz-Tapes + Observable-Skala (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via grind-flash) der Parser ist gebaut und auf den zwei Datentapes verifiziert (s. Erledigt). **Offen:** `DD005407`/`DD005409` sind strukturell andere (Summary-)Tapes (durchweg 1206-B-Records ohne Observablen-Zeitfeld) — ein eigener Tape-Typ, `pending`; die **Observable-Skalierung** einzelner Datentypen ist am Original gegenzuprüfen (die Frequenz-Skala war uneindeutig: roh 1.08e12, /2⁹ ≈ 2114 MHz).
- **Blockade:** die Zusatz-Tapes brauchen eine eigene Messung; die Observable-Skala ist nicht durchgehend belegt.
- **Braucht:** `url`-Zeilen `spdf.gsfc.nasa.gov/.../PSCM-0001{7,4}_DD0054{08,10}_*.tar` + `…/renamed_dataset_catalogs/mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf` (Mycelium) + `terms NASA-PD` (Mountain); die Summary-Tapes messen oder `descoped`.

### Mariner 10 RSS PSCM-00009 — Auth-Kante (NSSDCA)
- **Status:** wartend | **Bindung:** eigen (Vorbereitung) / Operator (Auth-Route)
- **Trigger:** NSSDCA-Antwort (Anfrage 2026-09-16, keine Antwort) oder Operator-Wort für die Auth-Route
- **Lage:** (gemessen 2026-10-11) 762 7-Track-Tapes, `request-only` („Ready for Offline Distribution", kein Download); `NSSDC_USER`/`NSSDC_PASS` liegen vor; es gibt **keinen** `archive_search`-Arm für NSSDC. Operator-Wort 2026-10-11: Auth ok, wenn eligible + redistributable.
- **Blockade:** fehlender NSSDC-Arm (Build) — die Auth-Route kann nicht gefahren werden.
- **Braucht:** einen `archive_search`-NSSDC-Arm bauen (`tools/utils`, `NSSDC_USER`/`NSSDC_PASS`) oder Operator-Hand; bis dahin `wartend` (Wahrheit `state/zustand/wartend.φ`).

### Rätsel-Datenlücken (future-235/236) — url/origin (Mycelium) + Parse
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via future-235/236, `--verdict`) 5 Datenlücken, 4 offen erreichbar: Ⅶ Wurmloch (NAIF Voyager SPK, US-PD) · Ⅷ Dunkler Fluss (Zenodo UNCOVER `14281664`, JADES `7460116`, CC-BY-4.0) · Ⅻ Urknall (BK18 `bicepkeck.org`, CC-BY-4.0) · GIC-Mäntsälä (FMI NUR `space.fmi.fi/image`, CC-BY-4.0; nächste Station ~27 km) · Ⅺ Placebo: **kein** offenes Einzeldataset trägt Gabe×HRV×Blutmarker (nur Zwei-Kanal-Depots) — Weg nur selbst erheben oder eigenes Blatt.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** je Lücke die `url`-Zeile (Mycelium) → `terms`/`format` (Mountain) → Parser; Ⅺ als descoped-Befund oder eigenes Blatt.

### NIST NCNR NSE — offene Kohärenz-Route (future-235, ersetzt `trisp-keller`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via future-235/236) `https://ncnr.nist.gov/pub/ncnrdata/nse/` US-PD, HTTP 200, monatliche Sets; `trisp-keller` descoped (generischer Anspruch).
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** `url`-Route registrieren (Mycelium) + `terms`/`format` (Mountain).



### THEMIS-ASI-Keogramm — Compiler-Fix blockiert am aktiv gehaltenen Dateiset
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Freigabe von `tools/harvest/src/bin/themis_asi_keogram_compiler.rs` durch die haltende Session
- **Lage:** (gemessen 2026-10-11 via `git status`/`stat`) die Datei ist **uncommittet modifiziert** (mtime 2026-10-11 20:5x, nach HEAD); Form-Verdikt steht (`on earth …` + `station …`, a=asilib autoritativ).
- **Blockade:** die Datei hält eine parallele Session (uncommitteter Hunk).
- **Braucht:** nach Freigabe `emit` in `themis_asi_keogram_compiler.rs:112` auf `on earth <lat> <lon> <alt>` + `station <CODE>` umstellen (Alt aus gemessener Spalte oder `pending`); `cargo check` 0/0.

### Quellen-Terms schreiben — `phi/sources.φ` aktiv gehalten
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Freigabe von `phi/sources.φ` durch die haltende Session
- **Lage:** (gemessen 2026-10-11 via `git status`/`git diff --stat`) `phi/sources.φ` trägt **70 uncommittete Zeilen** (url/origin/compiler-Manifest-Bindungen, Mycelium-Anteil) plus `src/mathematikerin/{mod,receiver,tropo}.rs`, `cdn_reconcile.rs`, `themis_asi_keogram_compiler.rs` — ein aktiver Fremd-Halter. Die 7 Terms/Verdikte (s. Erledigt) sind gemessen, aber noch nicht als Verdikt-Zeilen geschrieben.
- **Blockade:** ein pfad-begrenzter Commit über `phi/sources.φ` würde die fremden Hünke mitnehmen; „Commit als Letzter" verbietet den Schreibakt in eine aktiv gehaltene geteilte Datei.
- **Braucht:** nach Freigabe je Block die `terms`/`ttl`-Zeile setzen (THEMIS-ASI/PCN/CSES admissibel; OE-417/BGS-GIC/NERC-GMD als not-admit-Verdikt; McPherron als point-event-Block nach `MCON`-Bau), `terms` = Mountain, `url`/`origin`/`compiler` = Mycelium.

### Band-Paar-Atom — CI-Idempotenz-Sperre (unverändert aus 319)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `spdf.gsfc.nasa.gov`-Asset-Refresh + `witness-odf-residuum`-Lauf
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert) `pioneer-odf-cdn.yml:21-31` überspringt Compile+Upload, wenn das Release-Asset existiert (zuletzt 2026-09-18) → Witness `model: 0 samples modeled`.
- **Blockade:** Manifestations-Recht (Mycelium).
- **Braucht:** ODF-Workflows mit erzwungenem Re-Manifest (`--force`/clobber) neu dispatchten; danach Witness-Lauf lesen.

### ci-gate — rot am HEAD; format/register fremd
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf
- **Lage:** (gemessen 2026-10-11 via `ci_manage list`) `ci-gate` Läufe `38164577203`/`38164341503`/`38164276418` = **failure** (18:32–18:37Z); clippy in vmf3 geheilt (Vorgänger-Atom). `data.fingrid.fi` erscheint jetzt als Block `phi/sources.φ:11133-11136` (uncommittet).
- **Blockade:** `src/mathematikerin/*`-Format-Hünke hält eine andere Session; register/netloc-Bindung Mycelium.
- **Braucht:** die haltende Session formatiert `src/mathematikerin/*`; danach einen `ci-gate`-Lauf lesen (`ci_manage log <id>`).

### Eigene Ephemeride (PEP/Tudat) — Step 4 blockiert am aktiven Dateiset (unverändert)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Freigabe von `src/mathematikerin/{mod,receiver,tropo}.rs`
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert) Rat-Votum C; Research + UI konvergent.
- **Blockade:** fremd-aktiv gehaltenes Dateiset.
- **Braucht:** nach Freigabe die drei Rat-Schritte 1+2 als begrenzten Bau.

### Teil C — A5 offen (Galileo TDF), A6 descoped
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert) A5 `GO-SS-RSS-1-TDF-V1.0` (PDS PPI); A6 descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** A5 die TDF-Route als `url`-Zeile (Mycelium) + `terms`/`format` (Mountain), oder `descoped` mit Befund.

### opentopography_coverage — per-Dataset-Lizenz (Rat-Entscheid steht)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Coverage-Manifest auf dem CDN + Block registrieren
- **Lage:** (gemessen 2026-10-11 via 319-Register) per-Dataset-Lizenz; Portal-Manifest ohne `terms` (0 honored); `shard`-Emit nicht registriert.
- **Blockade:** Manifest-Asset (Mycelium).
- **Braucht:** Manifest (Mycelium) + Block (`url /origin /compiler /format coverage_manifest`, keine `terms`).

### SWPC/RTSW-Kurzreihen — nicht alignbar (river-187, gefaltet)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via river-187, Lauf `38119733631` Job `matrix-vlies`) 126/210 Matrix-Zellen `alignment pending` — SWPC-Livereihen decken ~1 Tag, OMNI endet 2026-10-05, keine Überlappung.
- **Blockade:** fehlende statische überlappende Archivreihe.
- **Braucht:** Verdikt + `ttl` für eine statische SWPC/RTSW-Archivreihe (URL) **oder** die Kurzreihen als nicht-alignbar führen (descoped mit Befund).

### `api.sensor.community` — eigener Register-Punkt (future-235, gefaltet)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via future-235) **kein** Future-Wort; Ursprung `mycelium-158…162` (403) + `mountain-306`.
- **Blockade:** Arm/Zielzeile selbst zu benennen.
- **Braucht:** Arm/Zielzeile benennen oder `descoped` mit Befund.

### Orphan-Prosa-Träger — `survey-2026-10-10-worldwide-lidar-quellen.md` (Mountain-Träger)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Verdikt-Pass
- **Lage:** (gemessen 2026-10-11 via `register_lookup --orphan-docs`) das Dokument trägt **51 offene Marker** und keinen Live-Handover-Träger; die 8 future-235/237-Dokumente sind inzwischen getragen (`--orphan-docs` nennt nur noch dieses).
- **Blockade:** die Worldwide-LiDAR-Verdikte sind Mountain-Recht (letzter Befund über den Baum).
- **Braucht:** die 51 Marker sichten, je Klasse eine Verdikt-/Träger-Zeile, `phi/sources.φ`-Einträge setzen; dann ist das Dokument getragen.

### Sechs Klima-Quellen / series-manifest · copernicus B1–B6 · NOAA S3 B7/B8 · SPT-3G D1 (unverändert aus 319)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je der erste CDN-/Manifest-Lauf bzw. gewählte Eingaben
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert) ecad/aodn brauchen gewählte Station/URL; die übrigen Läufe laufen/fehlen.
- **Blockade:** kein Lauf / wissenschaftliche Wahl ohne Default.
- **Braucht:** die Läufe lesen (`ci_manage log <id>`) + sha256 je Familie.

### Frame/Epoche · particle-cern · Exposom-Matrix · iEEG · Flyby-Kette · FMHY (unverändert aus 319)
- **Status:** eigen/wartend/termin/blockiert | **Bindung:** eigen/termin:2026-11-01
- **Trigger:** der 319-Register-Trigger je Punkt (Frame/Epoche: CRS-VLR-Messung; particle-cern: `subset`; Exposom: extern gedeckter Repräsentativpunkt; iEEG: neues Operator-Wort über 2026-10-06 — **nicht erneut vorlegen**; Flyby: ESOC-Recon-Release; FMHY: Riss 4/5)
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert)
- **Blockade:** s. je Punkt im 319-Register.
- **Braucht:** s. je Punkt; sonst der benannte Riss/Descope-Befund.

## An mycelium

Origin: mountain-folge320 (2026-10-11).

- **`phi/sources.φ` aktiv gehalten:** 70 uncommittete url/origin/compiler-Zeilen im Baum (plus `cdn_reconcile.rs`); bitte committen, damit Mountains Verdikt-Zeilen (7 Terms oben) ohne Hunk-Kollision geschrieben werden können.
- **`themis_asi_keogram_compiler.rs` aktiv gehalten:** uncommittet modifiziert; bitte committen (Fix `at earth`→`on earth` wartet).
- **THEMIS-ASI/PCN/CSES admissibel** — bitte `url`/`origin`/`compiler` je Quelle; `terms`/`ttl` schreibt Mountain nach Freigabe.
- **McPherron — `MCON`-Form ist gebaut (Mountain), nur noch Manifestation (Mycelium):** Register-Block braucht `url https://github.com/omegaflow/sources/releases/download/zenodo.org/mcpherron_onsets.bin` · `origin https://zenodo.org/api/records/22711679/files/CorrOnsFiles.zip/content` · `compiler tools/harvest/src/bin/mcpherron_compiler.rs` · `format mcon_event` · `witness point-event` · `record mcon` · `force em` · `no-cadence` — **kein** `terms`-Direktiv, aber `terms CC-BY-4.0 https://zenodo.org/records/22711679` als Zeile? (Zenodo-Witness-Block: `terms` gehört in den Block, Mycelium) + CDN-Workflow `mcpherron-cdn.yml`; Compiler-sha256 `441df5f8c59ad57962d81cc82814a0d36c7770f827b4d1893b31425f035cc3f1`. Die fünf `field`-Zeilen (mcon_sml_nt/amplitude_nt/bay_decrease_nt/bay_duration_min/bay_width_min) müssen den Komponenten folgen.
- **MLA-Register:** Repoint + `field RANGE mla_range` stehen (319); nur noch CDN-Manifestation + sha256.
- **Re-Admission/GIC-Terms stehen** (319 + 320); es fehlen die `url`/`origin`-Zeilen.
- **Mariner-Rohband-CDN (future-236 gefaltet):** sobald Mountain die Parser-Arme `mariner_rst` + `mariner_m10_edr` gebaut hat, gehören die Assets in CI → CDN; `url`-Zeilen für `spdf.gsfc.nasa.gov/pub/data/mariner/{mariner4,6,7}/…` (NASA-PD) und `ser.im-ldi.com/M10/MVM/mvm_0001.tgz`. Der NSSDCA-Auth-Weg (`NSSDC_USER`/`NSSDC_PASS`, **kein** `archive_search`-Arm) ist die Kante für `PSCM-00009` (762 7-Track-Tapes).
- **Rätsel-/NSE-Routen (future-235/236):** `url`/`origin` für NAIF Voyager SPK, Zenodo UNCOVER `14281664`/JADES `7460116`, BK18 `bicepkeck.org`, FMI NUR `space.fmi.fi/image`, NIST NCNR NSE `ncnr.nist.gov/pub/ncnrdata/nse/` — je `url` durch Mycelium, dann `terms`/`format` durch Mountain.

## Operator-Wort-Register

- **„b"** | 2026-10-11 | Operator-Session (mountain) — Form **(b)** für das Band-Paar-Atom: Companion-Band-Ebene (Record bleibt 9 Felder), nicht das Feld-6-Packing.
- **„ja bitte mro-Schritt nachziehen?"** | 2026-10-11 | Operator-Session (mountain) — auch `mro_odf_compiler` (streaming/shard) auf das Band-Paar nachziehen.
- **„bitte befrage auch research und roster wenn du den rat befragst"** | 2026-10-11 | Operator-Session (mountain) — jede Ratsfrage durch Research-Schicht + Roster.
- **„nein bitte hole die stimmen erst ein hier geht es um etwas sehr existenzielles"** | 2026-10-11 | Operator-Session (mountain) — Stimmen vor dem Bau.
- **„hab glm abgeholt"** | 2026-10-11 | Operator-Session (mountain) — GLM 5.3-Antwort vom Operator nachgereicht.
- **„bitte bei ratfragen auch archive search all und roster (mind. claude und glm 5.3 max) befragen"** | 2026-10-11 | Operator-Session (mountain) — jede Ratsfrage zusätzlich über `archive_search --all` (Research-Schicht) und den UI-Roster, mindestens Claude und GLM 5.3 max. In diesem Atom angewandt (Q1/Q2): `archive_search --all` + Fach-Modi, Claude Sonnet 5.5 Extra hoch, GLM-5.3 Deep Think Max.
- **„hast du denn alles was future zu den rätseln und mariner gefunden hat gefaltet … vision und vielleicht auch secrets"** | 2026-10-11 | Operator-Session (mountain) — Future-236s Mariner-/Rätsel-Befunde falten; für Mariner Vision + Secrets einsetzen. Angewandt: 2 Vision-Reads (`pict_app_a.pdf` M10-Layout, `dsc_0073.pdf` M6/7-Layout), `NSSDC_USER`/`NSSDC_PASS` gemessen (`bin/secrets_keys`).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom: die eigenen
Dateien committen — `docs/handover/*`, `src/archivar/mcon_event.rs`,
`src/archivar/{mariner_m10_edr,mariner_rst}.rs`, `src/archivar/{mod,witness,extract}.rs`,
`tools/harvest/src/bin/{mcpherron_compiler,mariner_m10_edr_compiler,mariner_rst_compiler}.rs`; `phi/sources.φ`,
`src/mathematikerin/*`, `tools/harvest/src/bin/{ea_lidar_index,themis_asi_keogram_compiler}.rs`
und `tools/measure/src/bin/themis_asi_timed_keogram.rs` sind fremd-aktiv gehalten
und werden nicht angetastet („Commit als Letzter").

**Gemessener Zwischenfall (2026-10-11, 21:16):** der `git mv` der konsumierten
Übergabe (`folge319` → `archiv/`) wurde vom **parallel laufenden Mycelium-Commit
`9141bb97d`** („mycelium 311: fold the binding outcome…") mitgeschrieben — ein
geteilter Index zwischen zwei Linien-Sessions; die Umbenennung liegt jetzt unter
fremder Commit-Message. Bestätigt die „Commit als Letzter"-Regel: `git mv`/`git add`
in den geteilten Index während einer parallelen Session committen lässt. Der eigene
`folge320`-Commit bleibt davon getrennt.
