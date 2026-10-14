<!--
  title: Handover — Mountain-Folge 346 (2026-10-14)
  session: Mountain-Linie — ein Pass: pds3_img BAND_STORAGE_TYPE (BSQ/BIL/BIP) + adressierte Blöcke (mycelium-347, river-209) gefaltet
  class: handover
  date: 2026-10-14
  sha256: 18a627113c72581851bfa68114f80165181742655ecfcbb4383291711e95253e
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md
-->
# Handover — Mountain-Folge 346 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-14-mountain-folge345.md` (→ `archiv/`).
Gefaltet (`register_lookup --addressed mountain`): `mycelium-folge347` und
`river-folge209`. Still gemessen: `register_lookup --fired mountain` = 0,
`--stale mountain --persist 3` = 0, `open_points_check` sauber (23 Pfad-Refs,
0 absent/stale/word-carried). Der Stehende Pass wird als **öffentliche** Fassung
zitiert, nie kopiert: HEAD `9b2517557` (2026-10-14).

## Burn: open 0.00 · close 0.40 · cap 0.40 Grund: ein Pass — der `pds3_img`-Interleave-Arm (`BAND_STORAGE_TYPE` → `Bsq`/`Bil`/`Bip` in `parse_label` + `decode_raster`, 4 neue Unit-Tests) und die Ablösung des Byte-Reshuffles im `pds3_bip_radar_compiler` (der Strict-Extent-Wächter bleibt); `cargo check` 0/0, Bin gebaut, Selftest grün; die zwei adressierten Blöcke gelesen und in die Punkte unten gefaltet; kein Rat/Roster, kein Seat.

## Offen (aufgeschlüsselt)

### SABER — eigene Sonde an xyz,t; Arm-Umbau + Block warten (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: die Sonde an xyz,t) | **Bindung:** eigen
- **Trigger:** der Arm-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) HAPI-`info` aus dem Cache
  (`/tmp/opencode/saber_info.json`): `ktemp` `size:[400]` K, `tpgpaltitude` `size:[400]`
  km, `fill -999.0`; ein `data`-Row ist `[isotime, <array>, <array>]`. Der Arm
  (mountain-345) expandiert `param.<i>` über `parameters[].size` zu Skalar-Kanälen
  (`saber_ktemp_0 …`), heute mit `Position::Source` und `Motion::Linear`-Kanal-Orten.
  Das Operator-Wort verlangt die **eigene Weltlinie** des Satelliten:
  `Position::StateVector { p, v }` + `Motion::Linear`, die Höhe als Wert.
  `Position::Source` ist unit — **kein** `alt`-Feld (`extract.rs:7013-7025`).
  `cdaweb.gsfc.nasa.gov` ist host-weit unerreichbar (Stehender Pass), kein frisches Row.
- **Blockade:** cdaweb-Host unerreichbar — das Sample/der Block fehlt.
- **Braucht:** (0) **Bezugssystem + Zeitbasis** festlegen (ICRS der Haus-Konvention vs.
  ECI/ECEF; Zeitskala) — Gegenprobe GLM/Claude/DeepSeek; (1) den Arm auf
  `Position::StateVector { p, v }` + `Motion::Linear` ziehen (Kanäle an der
  Satelliten-Weltlinie); (2) den Block
  `url https://cdaweb.gsfc.nasa.gov/hapi/data?id=TIMED_L2A_SABER&…parameters=ktemp,tpgpaltitude&format=json`,
  `terms saber-rules-of-the-road …` — wartet auf die Host-Rückkehr.

### Venus Radar — Reader/Compiler geparkt (Operator-Wort W); pds3_img-Interleave gebaut (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Achsen-/Projektions-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Bundle `urn:nasa:pds:venus_radar_level2`
  v1.0, DOI `10.17189/1522443`, **CC0-1.0**, 15 Produkte. Die Level-2-Labels sind
  PDS4 `Array_2D_Map`, single-band, `Last Index Fastest` — kein
  `BANDS`/`BAND_STORAGE_TYPE`; `pds4_img` braucht keinen Interleave-Umbau. Der
  `SAMPLE_INTERLEAVED`/`BANDS=2`-Befund (mycelium-346-6) gehört zum **rohen** PDS3
  Complex-Backscatter-Bundle (`vrm_90xx`). Das Operator-Wort hält das reduzierte Bild
  als Bild (W): der D-Compiler (`venus_radar_compiler.rs`) und der VNRD-Reader
  (`vnrd.rs`) werden **geparkt**, die strahlende Serie entfällt.
- **Gebaut (mountain-346):** `src/archivar/pds3_img.rs` liest `BAND_STORAGE_TYPE` →
  `Bsq`/`Bil`/`Bip` (`BAND_SEQUENTIAL`/`LINE_INTERLEAVED`/`SAMPLE_INTERLEAVED`) in
  `parse_label`; `decode_raster` indiziert nach Interleave (BIP: `values_per_record =
  bands·samples`); ein unbekannter Token wird verworfen (`UnknownInterleave`), **kein**
  stiller BSQ-Fallback. Vier neue Unit-Tests (BIP/BIL-Token, BIP/BIL-Dekodierung,
  unbekannter Token). `pds3_bip_radar_compiler.rs` delegiert an den Kern-Decoder — das
  Byte-Reshuffle entfällt, der Strict-Extent-Wächter (`sample_interleaved_extent`) bleibt,
  Selftest grün.
- **Blockade:** keine für den Parser.
- **Braucht:** (0) **Projektion + Ortsbezug des Bildes** aus dem PDS4-Label benennen
  (Gegenprobe Qwen/DeepSeek: ohne sie ist ein Ganzbild kein Ortsfeld); (1) den
  Bild-Block (`url`/`origin`/`compiler`/`terms CC0-1.0`, Einheit am Feld);
  (2) Split-Route erst nachgelagert; (3) der native Lauf braucht 15×960 MB (CI).

### ICGEM gfc — Modell als Feld; Auswerter fehlt (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: das Modell wie die anderen behandeln) | **Bindung:** eigen
- **Trigger:** der Feld-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) D2 (Rat + Roster): statisch ohne Epoche,
  kein erfundener MJD. `gfc.rs`-Arm + `declared_fields` stehen (4 Felder
  `icgem_gfc_c/s`, `icgem_gfct_c/s`); der Compiler (`epoch_of` Z.147-150) **verlangt**
  eine Epoche und schreibt sonst nichts. Haus-Wirklichkeit: `epoch 0` ist „Spalte/Key 0"
  (`parse.rs:1870`), **kein** Fehlen; die fest-stride 26×f64 hat keinen absent-Slot;
  `matrix.rs:644` liest `epoch <= 0.0` als absent und **überspringt**, `gfc.rs:153`
  behält 0.0. Kein Präzedenzfall für eine geschriebene Serie ohne Epoche.
- **Operator-Wort (2026-10-14):** das Gravitationsmodell wie das Erdmagnetfeld- und das
  Atmosphären-Modell **an einem Ort auswerten** und dort einen **echten Feldwert**
  liefern. Präzedenz: **VMF3** (`vmf3_sh` am Ort), **IGRF**
  (`synthesize(date, radius, colat, lon) -> (Br,Bθ,Bφ)`). Ein Modell ist ein **Feld**,
  keine Gestalt. `format gfc` ist im `main_flow`-Dispatch (mountain-345).
- **Blockade:** der Koeffizienten-Arm (`gfc.rs`) führt die Modell-Interna als Kanäle;
  der Auswerter (Koeffizienten an einem Ort → Feldwert) fehlt.
- **Braucht:** (0) die drei Gegenprobe-Randbedingungen tragen — **(0a) Bahn↔Feld-Kopplung**
  (Gravitation erzeugt die Bahn selbst; GLM), **(0b) Höhen-Bezugsfläche**
  (Ellipsoid/Geoid, selbst eine Gestalt; Claude), **(0c) Feldwert vektoriell/tensoriell**
  (Richtung/Gradient, nicht nur Betrag; DeepSeek); (1) Auswerter wie `vmf3_sh`/`igrf`;
  (2) Block `url …/icgem.gfz.de/gfc.bin`,
  `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`, Feld am Ort; Cap D=3211.

### Epochen-Absenz — zwei Linien, unvereinheitlicht (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom / die nächste epochenlose Quelle
- **Lage:** (gemessen 2026-10-14, Taucher) Linie A liest `epoch <= 0.0` als **absent**
  und überspringt/verwirft (`src/mathematikerin/machines/matrix.rs:644`;
  `src/archivar/drs_fits.rs:38`); Linie B behält `0.0` als **echten Wert**
  (`src/archivar/agrav.rs:49/113`; `src/archivar/hips.rs:793`; `src/archivar/gfc.rs:153`).
  **Es gibt keine einheitliche Epochen-Absenz-Konvention.** Der Rat (F3) benannte das als
  tiefsten Befund: vor jeder `gfc`-Manifestation, nicht mit einem Sentinel überschreiben.
- **Blockade:** die Frage ist eine Wire-Semantik (fehlt dem 26×f64 ein „absent"-Slot?),
  noch nicht entschieden.
- **Braucht:** die zwei Linien zusammenführen — entweder die Lesesite
  (`gfc`/`agrav`/`hips`) auf die `≤ 0.0`-Absenz ziehen **oder** die Absenz ausdrücklich
  tragen; dann erst die `gfc`-Manifestation.

### UCAR-FPI — Arm + Zeilen stehen, Primärkoordinate offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-14) Arm + Guard (`fpi_ucar.rs`, Dispatch `extract.rs:159`).
  Sample `data/registration.hao.ucar.edu/20246300athabasca.dat`, 7353 Zeilen, 10
  leerzeichengetrennte Spalten. Lizenz gemessen: keine benannte (UCAR/DASH-Nutzungssatz),
  terms-URL `https://data.ucar.edu/dataset/fpi-resolute`. **Riss:** kein Athabasca-FPI-DOI.
  Die zwei `field`-Zeilen sind code-gemessen (`fpi_mer_wind_ms` `fpi_ucar.rs:10`,
  `fpi_zon_wind_ms` `fpi_ucar.rs:12`; `extract.rs:534/535`).
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN); die FPI-eigene
  Athabasca-Instrumentenkoordinate ist ungemessen (die File-/DASH-Seiten tragen nur
  Palmer/Resolute/Boulder; der einzige Zahlenwert löst auf AUGO auf — Riss, ungeglättet).
- **Braucht:** die **FPI-eigene** Athabasca-Instrumentenkoordinate an der Primärquelle
  (`qwu@ucar.edu` / DASH-Resource-Support) messen; dann die zwei `field`-Zeilen + die
  Arbeitskopie-Route (`local-only`).

### Fünf fehlende Fäden — Entwurf steht, Mycelium setzt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14, Taucher) Mountain-Teil fertig als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ` (176 Zeilen): je Faden die gemessene
  `terms`/`field`/`ttl`-Zeile (Sonnen-Magnetfeld, Stern-Aktivität, 21 cm, Wolken, TGF),
  Haus-Token-Korrekturen belegt (`sdo`→`CC0-1.0`, `vizier-open`→`vizier-scientific-free`,
  `heasarc-free`→`PD`, `NOAA-CDR-open`→`PD`, Zenodo→`CC-BY-4.0`). Zwei Risse offen
  (WSO-Polarfeld ohne Terms; LISIRD ohne Terms), 21 cm EBHIS bereits registriert.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler` je Faden auf den Entwurf; dann
  `register_sort`.

### Adressierte Register-Fäden aus mycelium-347 (eigen)
- **Status:** eigen (Register-Writes; die Messungen sind Mycelium-Claims, am Baum
  gegenzumessen) | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14 via `register_lookup --addressed mountain`, `mycelium-folge347`)
  - `xfer:mycelium-346-3` **MODVOLC** — HIGP-Bulk-Tree `http://modis.higp.hawaii.edu/data/`
    (200, kein Login, `.061`) erreichbar, **ohne Lizenzzeile** → `terms local-only` (kein CDN).
  - `xfer:mycelium-346-4` **DE-2 NACS/WATS** — SPASE-**CC0**-DOIs
    `10.48322/z50y-hd65`/`j8kp-5y58`; der `de2`-Block (`phi:851-871`) trägt heute
    `terms PD https://cdaweb.gsfc.nasa.gov/`. Terms-Korrektur auf `CC0` nach Messung.
  - `xfer:mycelium-346-7` **AWESOME-VLF/LF** — `doi.org/10.48322/fwte-dv13` (+ `fzcf-f191`),
    **CC0** gemessen; als offene Quelle registrieren. `fzcf-f191` sitzt bereits an
    `waldo` (`phi:8266`); das `vlf_awesome`-Feld trägt heute `terms PD`
    (`phi:1744`) — Abgleich fällig.
  - `xfer:mycelium-346-5` **OCNS2019-Ersatz** — JPL SSD/Horizons (NASA CC0) als offene
    Alternative zu `local-only`.
- **Blockade:** jede Zeile ist ein Verdikt → erst die DOI/den Lizenztext **messen**
  (`archive_search --verdict`), dann setzen (kein Verdikt ohne Messung).
- **Braucht:** die vier Terms-Verdikte am Register setzen und `register_sort`.

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide;
  `docs/specs/force-unit-baseline.txt` trägt sie (plus `em mev`) froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund
- **Lage:** (gemessen 2026-10-14, Taucher) ALFALFA `local-only` bestätigt — kein Block in
  `phi`, DOI `10.26093/cds/vizier.18610049` HTTP 200, ReadMe ohne Lizenzstring.
  Ersatz-Bau: `opentopography_raster_compiler.rs` liest SRTM15+ V2.6 und schreibt ein
  Gestalt-Witness (`SRTM`), **ohne** Geoid-/Datum-/Toleranz-Anker; kein
  `opentopography_raster`-Block in `phi` (nur `opentopography_coverage`, `srtm15_plus_*`).
  `load_gestalt_surface_threads` (`main_flow.rs:580`) — einziger Aufrufer `tests.rs`,
  **kein** Produktionsaufrufer. Eigene Ephemeride: `ilrs_npt_compiler.rs` steht
  (`FORMAT "ilrs_npt"`), aber **kein** Lese-Arm in `src/`; `icrf3_compiler.rs` schreibt
  JSON (Zielpfad absent), **kein** SKD1/Golden-File/Round-Trip.
- **Blockade:** keine.
- **Braucht:** (1) Höhenbezug + Toleranz vor der `opentopography_raster`-Registrierung
  festlegen (KAPU-Kontrollpunkte 179,94 m vs. 217,90 m), dann Block; (2)
  `load_gestalt_surface_threads` an einen Produktions-Aufrufer binden **oder** `descoped`;
  (3) `ilrs_npt`-Lese-Arm in `extract.rs` + Block; (4) `icrf3` erst Vertrag versionieren
  + Round-Trip (`skydirection.rs`) + Golden-File, dann SKD1. `docs/SOURCE_PORT.md`.

## An mycelium

Origin: mountain-346 (2026-10-14):

- `xfer:mycelium-346-6` **Venus-Parser — erledigt.** `pds3_img` liest `BAND_STORAGE_TYPE`
  (BSQ/BIL/BIP), `decode_raster` indiziert danach; `pds3_bip_radar_compiler` delegiert an
  den Kern-Decoder. Echo.
- `xfer:mycelium-346-2` **Venus-Radar** — der Parser-Blocker ist gelöst. Die reduzierte
  Bild-Manifestation (W) braucht Projektion/Georeferenz; `url`/`origin`/`compiler` erst
  nach dem Projektions-Wort.
- **Fünf Fäden:** der Mountain-Teil liegt fertig als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ`. Bitte `url`/`origin`/`compiler` je Faden
  daraufsetzen; danach `register_sort`.
- `xfer:mycelium-346-3`/`-4`/`-5`/`-7` — als Register-Fäden oben geführt; Mountain misst
  die Terms und setzt die Verdikte.

## An river

Origin: mountain-346 (2026-10-14):

- `xfer:mountain-342-6` **SABER:** die Registerzeile folgt nach dem HAPI-Weltlinien-Arm;
  `cdaweb.gsfc.nasa.gov` ist host-weit unerreichbar — das Sample bleibt bis zur
  Host-Rückkehr ungemessen.
- **UCAR-FPI:** die zwei `field`-Zeilen stehen code-gemessen; offen bleibt allein die
  FPI-eigene Athabasca-Instrumentenkoordinate.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344, 345, 346).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Kette gefahren; Verdikt (Rat + Roster): **nT²/Hz primär (Kalibrierung im Arm, linear), V²/Hz als Rohkanal**; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh, Kalibrierung versioniert im Arm) | 2026-10-14 | (mountain 344 — Operator legte das Verdikt der Stimmen-Kette vor).
- **„was wenn du den satellit autonom betrachtest — dann ist es doch auch nur eine sonde an einem xyz,t; die ISS ist doch zu einer Zeit immer an einem bestimmten Ort"** | 2026-10-14 | (mountain 345 — SABER: eigene Weltlinie, supersediert B2).
- **„warum können wir das gravitationsmodell nicht wie die anderen behandeln? das Erdmagnetfeld-Modell und das Atmosphären-Modell werden an einem Ort ausgewertet und geben dort einen echten Feldwert — sie sind ein Feld"** | 2026-10-14 | (mountain 345 — ICGEM: Feld, supersediert Witness).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. In diesem
Atom (mountain-346) committet: `src/archivar/pds3_img.rs` ·
`tools/harvest/src/bin/pds3_bip_radar_compiler.rs` · diese Übergabe ·
der `archiv/`-Move der folge345.
