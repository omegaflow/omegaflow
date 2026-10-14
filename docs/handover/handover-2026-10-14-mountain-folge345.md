<!--
  title: Handover — Mountain-Folge 345 (2026-10-14)
  session: Mountain-Linie — ein Pass + Taucher + Rat + Bridge-Roster: finals-Leser, harvest-Direktive, Doppel-Arme, Venus-Reader-Arm; SABER/Venus/ICGEM durch recherche·roster·rat (Riss auf SABER/Venus)
  class: handover
  date: 2026-10-14
  sha256: e927fbd63adb0d05492e21c78826e165c059daa1fa0195d6ea41b45c9f9d2c25
  status: live
-->
# Handover — Mountain-Folge 345 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-14-mountain-folge344.md` (→ `archiv/`).
Gefaltet (`register_lookup --addressed mountain`): `mycelium-folge346` und
`river-folge208` — die mountain-gerichteten Einträge sind in die Punkte unten
eingegangen (CelesTrak und LOD-`finals` gemessen erledigt, `harvest`-Direktive
gebaut; SABER/Venus/UCAR-fäden bleiben). Der Stehende Pass (Mycelium-Register) wird
zitiert, nie kopiert.

## Burn: open 0.00 · close 0.40 · cap 0.40 Grund: ein Pass + Taucher-Runde + Rat — finals-Leser (Code + Block + 6 Tests), harvest-Direktive, Doppel-Arme, Venus-Reader-Arm (`vnrd.rs` + Dispatch), 6 Taucher (5 parallel + 1 Recherche), Rat + volle Bridge-Roster-Tafel (11 Seats) für SABER/Venus/ICGEM; der Session-Eigenverbrauch ist mid-session nicht isolierbar (`session_burn` trägt nur beendete Sessions — der Gesamtwert fiel im Fenster 1.7797→1.1284), `close` ist die Cap-Grenze, kein gemessener Delta.

## Offen (aufgeschlüsselt)

### SABER — Höhenform (C): Arm + Höhen-Ablage brauchen ein Wort (eigen)
- **Status:** eigen (Rat-Verdikt B2) | **Bindung:** eigen
- **Trigger:** der Arm-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Rat-Verdikt (C-1): `field ktemp.<i> …` (dotted
  Level im Schlüssel), `alt tpgpaltitude km`. **HAPI-`info` ist aus dem Cache gelesen**
  (`/tmp/opencode/saber_info.json`, 7489 B): `ktemp` `size:[400]` K, `tpgpaltitude`
  `size:[400]` km, `fill -999.0`; ein `data`-Row ist `[isotime, <array>, <array>]`,
  dotted via `row[col[param]][i]`. Code-Sites: `Extract::Hapi(Vec<(String,String)>)`
  (`types.rs:287`, **kein** `alt`-Slot), Hapi-Parse (`parse.rs:908-916`), `alt`-Direktive
  (`parse.rs:1841-1868`, füllt nur `Map`/`ProfileMap`), Hapi→`extracted`
  (`extract.rs:6893-6966`), Hapi→`Channel` mit `Position::Source` (`extract.rs:7013-7025`;
  `Position::Source` ist unit — **kein** `alt`-Feld). Der cdaweb-Host ist erneut
  gemessen **unerreichbar** (SSL/000; `spdf.gsfc.nasa.gov` ebenso), kein frisches Daten-Row.
- **Gebaut (mountain-345):** der HAPI-Arm expandiert `param.<i>`-Familien über `parameters[].size`
  zu je einem Skalar-Kanal (`saber_ktemp_0 …`, füllwert `-999.0` = absent), `Position::Source`;
  Blockform `hapi ktemp.<i>=saber_ktemp tpgpaltitude.<i>=saber_tpgpaltitude` + zwei `field`-Zeilen.
- **Blockade:** `cdaweb.gsfc.nasa.gov` ist unerreichbar (Stehender Pass) — das Block-Sample fehlt.
- **Braucht:** nur noch den Block
  `url https://cdaweb.gsfc.nasa.gov/hapi/data?id=TIMED_L2A_SABER&…parameters=ktemp,tpgpaltitude&format=json`,
  `terms saber-rules-of-the-road …`, `at earth` — wartet auf die Host-Rückkehr (kein frisches Sample).

### Venus Radar — Compiler-Achse braucht ein Wort; Reader-Arm steht (eigen)
- **Status:** eigen (Rat-Verdikt D) | **Bindung:** eigen
- **Trigger:** der Compiler-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Bundle `urn:nasa:pds:venus_radar_level2` v1.0,
  DOI `10.17189/1522443`, **CC0-1.0**, 15 Produkte (5 Epochen × `inc`/`ocp`/`scp`), je
  960 000 000 B. **Korrigiert:** die Level-2-Labels sind PDS4 `Array_2D_Map`,
  single-band, `Last Index Fastest` — **kein** `BANDS`/`BAND_STORAGE_TYPE`
  (selbst gemessen: `…/data/venus_1988_north_inc.xml`; `Time_Coordinates`
  1988-06-17). Der `BAND_STORAGE_TYPE=SAMPLE_INTERLEAVED`/`BANDS=2`-Befund
  (mycelium-346-6) gehört zum **rohen** `arcb_nrao-v-rtls_gbt-3-delaydoppler-v1`
  Complex-Backscatter-Bundle (PDS3, `vrm_90xx`), nicht zum reduzierten Asset — **Riss**,
  `pds4_img` braucht **keinen** Interleave-Umbau. `venus_radar_compiler.rs` reduziert
  heute auf `comp = Zeilenindex` (12000 Kanäle) — eine Achse, die das Produkt nicht hat.
- **Blockade:** keine (Rat-Verdikt D entschieden).
- **Gebaut (mountain-345):** Reader-Arm `src/archivar/vnrd.rs` (MAGIC `VNRD`, `parse_bin`,
  strikt `comp ≤ 2` — eine Zeilenindex-Datei wird **verworfen**; `COMP_INC/OCP/SCP` →
  `venus_radar_inc_deg`/`_ocp`/`_scp`), registriert + Dispatch; **Compiler auf Verdikt D
  umgebaut** (`venus_radar_compiler.rs`: `--input`/`--dir` → ein `VNRD`, `t` = Label-Epoche
  (PDS4 `Time_Coordinates`), `v` = Kartenmittel, `comp` = Menge; Roundtrip über den Reader).
- **Braucht:** (1) Block `url …VNRD-CDN-Asset`, `origin` = rohe PDS-Kachel,
  `compiler venus_radar_compiler`, `terms CC0-1.0 …`, Einheit am Feld (`inc`=deg, `ocp`/`scp`=1);
  (2) Split-Route; (3) der native Compiler-Lauf braucht 15×960 MB (CI, nicht operatormaschine).

### ICGEM gfc — „Epoche absent" ist eine neue Konvention und braucht ein Wort (eigen)
- **Status:** eigen (Rat-Verdikt Witness) | **Bindung:** eigen
- **Trigger:** der Witness-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) D2 (Rat + Roster): statisch ohne Epoche,
  kein erfundener MJD. `gfc.rs`-Arm + `declared_fields` stehen (4 Felder `icgem_gfc_c/s`,
  `icgem_gfct_c/s`); der Compiler (`epoch_of` Z.147-150) **verlangt** eine Epoche und
  schreibt sonst nichts. **Neu gemessen:** die Haus-Wirklichkeit — `epoch 0` ist
  **„die Epoche ist Spalte/Key 0"** (`parse.rs:1870`), **kein** Fehlen; die fest-stride
  26×f64 hat keinen absent-Slot; `matrix.rs:644` liest `epoch <= 0.0` als absent und
  **überspringt** den Kanal, `gfc.rs:153` behält 0.0 dagegen. **Kein Präzedenzfall** für
  eine geschriebene Serie mit fehlender Epoche (jede trägt eine echte oder eine deklarierte
  Konstante, z. B. `viking_grav` 1976-01-01). Riss 2: volles Modell ≈ 12,8 GB; Cap D=3211.
- **Blockade:** keine (Rat-Verdikt Witness entschieden).
- **Braucht:** (1) XGM2019e als **nicht-strahlende Gestalt** führen (`witness gestalt`,
  wie `srtm15_plus`), nicht als `format gfc`-Serie; (2) Block
  `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`,
  `no-cadence`, ohne Epoch-Zelle; Cap D=3211 für den Split. Die vier `gfc`-Felder bleiben
  gebaut, strahlen aber nicht (beschlossen).
- **Rest:** `format gfc` war im Reader registriert, aber **nicht** im `main_flow`-Dispatch —
  **behoben** (mountain-345: `"gfc"` in die Dispatch-Liste).

### Epochen-Absenz — zwei Linien, unvereinheitlicht (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom / die nächste epochenlose Quelle
- **Lage:** (gemessen 2026-10-14, Taucher) Linie A liest `epoch <= 0.0` als **absent**
  und überspringt/verwirft (`src/mathematikerin/machines/matrix.rs:644`;
  `src/archivar/drs_fits.rs:38`); Linie B behält `0.0` als **echten Wert**
  (`src/archivar/agrav.rs:49/113`; `src/archivar/hips.rs:793` — Bandindex 0;
  `src/archivar/gfc.rs:153`). **Es gibt keine einheitliche Epochen-Absenz-Konvention.**
  Der Rat (F3) benannte das als tiefsten Befund: vor jeder `gfc`-Manifestation, nicht mit
  einem Sentinel überschreiben.
- **Blockade:** die Frage ist eine Wire-Semantik (fehlt dem 26×f64 ein „absent"-Slot?),
  noch nicht entschieden.
- **Braucht:** die zwei Linien zusammenführen — entweder die Lesesite (`gfc`/`agrav`/`hips`)
  auf die `≤ 0.0`-Absenz ziehen **oder** die Absenz ausdrücklich tragen; dann erst die
  `gfc`-Manifestation.

### UCAR-FPI — Arm verdrahtet, Sample liegt, Block fehlt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-14) Arm + Guard (`2d4c526f4`, `fpi_ucar.rs`, Dispatch
  `extract.rs:159`). Sample `data/registration.hao.ucar.edu/20246300athabasca.dat`,
  7353 Zeilen, 10 leerzeichengetrennte Spalten, Kopf `yyday ut(hr) merid(m/s) merid err dir ut(hr)
  zonal(m/s) zonal err dir skytemp`. Lizenz gemessen: keine benannte (UCAR/DASH-Nutzungssatz),
  terms-URL `https://data.ucar.edu/dataset/fpi-resolute`. **Riss:** kein Athabasca-FPI-DOI.
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN); FPI-eigene Instrumentenkoordinate
  **ungemessen** (2026-10-14, Taucher): die HAO-FPI-Dateiseite, das DASH-Dataset `fpi-resolute`
  (Bounding Box `N/A`) und die HAO-Instrumentenliste tragen nur Palmer/Resolute/Boulder, **kein**
  Athabasca; der einzige Zahlenwert (`54.728 N, 113.328 W, ~500 m`) löst auf die Co-Location
  AUGO auf — genau die Zeile, die **nicht** die Instrumentenkoordinate ist. Riss, ungeglättet.
- **Gebaut/Messung:** die zwei `field`-Zeilen sind code-gemessen — `fpi_mer_wind_ms` (merid,
  `fpi_ucar.rs:10`, `extract.rs:534`), `fpi_zon_wind_ms` (zonal, `fpi_ucar.rs:12`,
  `extract.rs:535`); Dispatch `extract.rs:163`, Guard `main_flow.rs:3224`.
- **Braucht:** die **FPI-eigene** Athabasca-Instrumentenkoordinate an der Primärquelle
  (`qwu@ucar.edu` / DASH-Resource-Support) messen; dann die zwei `field`-Zeilen + die
  Arbeitskopie-Route (`local-only`).

### Fünf fehlende Fäden — terms/field/ttl (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14, Taucher) Mountain-Teil fertig als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ` (176 Zeilen): je Faden die gemessene
  `terms`/`field`/`ttl`-Zeile (Sonnen-Magnetfeld, Stern-Aktivität, 21 cm, Wolken, TGF),
  Haus-Token-Korrekturen belegt (`sdo`→`CC0-1.0`, `vizier-open`→`vizier-scientific-free`,
  `heasarc-free`→`PD`, `NOAA-CDR-open`→`PD`, Zenodo→`CC-BY-4.0`). Zwei Risse offen
  (WSO-Polarfeld ohne Terms; LISIRD ohne Terms), 21 cm EBHIS bereits registriert.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler` je Faden auf den Entwurf; dann `register_sort`.

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide; `docs/specs/force-unit-baseline.txt`
  trägt sie (plus `em mev`) froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund
- **Lage:** (gemessen 2026-10-14, Taucher) ALFALFA `local-only` bestätigt — kein Block in
  `phi`, DOI `10.26093/cds/vizier.18610049` HTTP 200, ReadMe ohne Lizenzstring.
  Ersatz-Bau: `opentopography_raster_compiler.rs` liest SRTM15+ V2.6 und schreibt ein
  Gestalt-Witness (`SRTM`), **ohne** Geoid-/Datum-/Toleranz-Anker; kein `opentopography_raster`-Block
  in `phi` (nur `opentopography_coverage` `:11436-11439`, `srtm15_plus_*`); KAPU-Bbox nirgends
  registriert. `load_gestalt_surface_threads` (`main_flow.rs:580`) — einziger Aufrufer
  `tests.rs:10921`, **kein** Produktionsaufrufer. Eigene Ephemeride: `ilrs_npt_compiler.rs`
  steht (`FORMAT "ilrs_npt"`), aber **kein** Lese-Arm in `src/`; `icrf3_compiler.rs` schreibt
  JSON (Zielpfad unter `phi/`, Datei absent), **kein** SKD1/Golden-File/Round-Trip.
- **Blockade:** keine.
- **Braucht:** (1) Höhenbezug + Toleranz vor der `opentopography_raster`-Registrierung
  festlegen (KAPU-Kontrollpunkte 179,94 m vs. Ortslage 217,90 m), dann Block; (2)
  `load_gestalt_surface_threads` an einen Produktions-Aufrufer binden **oder** `descoped`;
  (3) `ilrs_npt`-Lese-Arm in `extract.rs` + Block; (4) `icrf3` erst Vertrag versionieren
  + Round-Trip (`skydirection.rs`) + Golden-File, dann SKD1. `docs/SOURCE_PORT.md`.

## An mycelium

Origin: mountain-345 (2026-10-14):

- `xfer:mountain-342-7` **CelesTrak — gemessen erledigt** (`sgrep -i celestrak phi/` = 0):
  der Block ist bereits entfernt, keine `descoped`-Zeile mehr zu setzen. Echo
  `xfer:mycelium-346-1`. Kein Schritt.
- **ICGEM-Manifestation wartet:** der `gfc`-Block braucht `url`/`origin`/`compiler`
  (`icgem.gfz.de/gfc.bin`, `https://icgem.gfz.de/getmodel/gfc/…/XGM2019e.gfc`,
  `gfc_compiler.rs`) — erst nach der epochenlosen Wire-Ablage (Punkt ICGEM).
- **Fünf Fäden:** der Mountain-Teil liegt fertig als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ` (terms/field/ttl je Faden, Haus-Token-Korrekturen
  belegt). Bitte `url`/`origin`/`compiler` je Faden daraufsetzen; danach `register_sort`.
- **Venus:** der Reader-Arm steht (`src/archivar/vnrd.rs`); `url`/`origin`/`compiler` erst nach
  dem Achsen-Wort (Punkt Venus).

## An river

Origin: mountain-345 (2026-10-14):

- `xfer:mountain-342-6` **SABER:** die Registerzeile folgt nach dem HAPI-`alt`-Arm;
  `cdaweb.gsfc.nasa.gov` ist laut Stehendem Pass host-weit unerreichbar — das Sample
  bleibt bis zur Host-Rückkehr ungemessen.
- `xfer:mountain-343-1` **Athabasca:** von `river-folge210` gefaltet (Arm + Zeilen stehen).

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344, 345).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Kette gefahren; Verdikt (Rat + Roster): **nT²/Hz primär (Kalibrierung im Arm, linear), V²/Hz als Rohkanal**; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh, Kalibrierung versioniert im Arm) | 2026-10-14 | (mountain 344 — Operator legte das Verdikt der Stimmen-Kette vor).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. In diesem
Atom (mountain-345) committet: `src/archivar/channels.rs` · `src/archivar/main_flow.rs` ·
`src/archivar/parse.rs` · `src/archivar/tests.rs` · `src/archivar/vnrd.rs` ·
`src/archivar/mod.rs` · `src/archivar/extract.rs` ·
`tools/harvest/src/bin/venus_radar_compiler.rs` · `phi/sources.φ` · diese Übergabe ·
der `archiv/`-Move der folge344.

## Kette recherche · roster · rat (2026-10-14) — Riss auf zwei Fragen

**Rat (fünf Stimmen):** 1) SABER-Höhe → **B2** (Höhe als eigener Skalar-Kanal an
`Position::Source`, Präzedenz `viking_grav`; kein `alt`-Direktiv). 2) Venus-Radar → **(D)**
(alle 15 Produkte → ein `VNRD`, `t` = Label-Epoche, `v` = Kartenmittel, `comp` = Menge).
3) ICGEM → **Witness** (nicht-strahlende Gestalt; kein 0.0-Sentinel).

**Roster (Bridge, volle Tafel — Linien-Gruppe + Auth-Route; nicht nur die vier Frontier):**

| Stimme | 1 SABER | 2 Venus | 3 ICGEM |
|---|---|---|---|
| **Rat** | a (B2) | **D** | Witness |
| Duck (GPT-6 Luna) | a | W | W |
| Qwen (Qwen3.8-Max) | b | W | (nicht gelesen) |
| Z.ai (GLM-5.3 Deep Think) | b | W | W |
| Claude (Sonnet 5.5 Extra hoch) | c | W | W |
| DeepSeek (DeepThink) | b | W | W |
| MiniMax M3 | a | D | 0.0 |
| Lumo 2.0 Max | a | W | W |
| Mistral (Vibe) | a | W | W |
| Perplexity | b | W | W |
| Sakana (Namazu) | a | W | W |
| Kimi | — Kontingent überschritten (gemessen) | | |
| Gemini | — kein Chat (Landing Page) | | |

**Q1 SABER:** a=5 (Duck · MiniMax · Lumo · Mistral · Sakana) · b=4 (Qwen · Z.ai · DeepSeek ·
Perplexity) · c=1 (Claude) — **kein Zug**; der Rat (a) liegt knapp vorn, das Roster ist gespalten.

**Riss 2 — Venus (scharf).** **W=10** gegen D=2 (Rat + MiniMax): das Roster will die Karte als
Karte; Mittelwert (D) und Zeilenprofil (B) gelten ihm als Strukturverlust. Das ist genau der
Riss, den der Rat selbst benannte. Der D-Compiler ist gebaut; gewinnt W, wird die strahlende
Serie `descoped`, der Compiler geparkt.

**Konvergenz — ICGEM.** Rat **Witness** + Roster **W=8** (nur MiniMax 0.0): einig.

**Tiefster Befund (F3, eigener Punkt):** zwei Epochen-Linien widersprechen sich —
`matrix.rs:644`/`drs_fits.rs:38` lesen `epoch ≤ 0.0` als absent, `agrav.rs:113`/`hips.rs:793`/
`gfc.rs:153` behalten 0.0 als echten Wert. Keine Sentinel-Überschreibung.

**Offen:** der Operator wiegt die zwei Risse (Rat gegen Roster) — dann steht das Verdikt fest,
und die Verliererform wird `descoped`.
