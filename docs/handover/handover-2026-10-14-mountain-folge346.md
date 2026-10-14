<!--
  title: Handover — Mountain-Folge 346 (2026-10-14)
  session: Mountain-Linie — ein Pass: pds3_img BAND_STORAGE_TYPE (BSQ/BIL/BIP) + vier Taucher + Rat (SABER, Venus-Projektion, ICGEM, Lizenzen, Epochen-Absenz)
  class: handover
  date: 2026-10-14
  sha256: 59db97e8d9ea3be832551588c013d391f3f3695a014402c874b71902d7a203b4
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

## Burn: open 0.00 · close 0.40 · cap 0.40 Grund: ein Pass — der `pds3_img`-Interleave-Arm (`BAND_STORAGE_TYPE` → `Bsq`/`Bil`/`Bip` in `parse_label` + `decode_raster`, 4 neue Unit-Tests) und die Ablösung des Byte-Reshuffles im `pds3_bip_radar_compiler` (der Strict-Extent-Wächter bleibt), `cargo check` 0/0, Selftest grün; danach vier Taucher (Lizenzen, Venus-Projektion, ICGEM-gfc, SABER-Weltlinie) plus ein Rat (Epochen-Absenz); kein Seat.

## Offen (aufgeschlüsselt)

### SABER — eigene Sonde; die Route ist der offene Punkt (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: die Sonde an xyz,t) | **Bindung:** eigen
- **Trigger:** der Arm-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Das HAPI-`info` (live HTTP 200, 7489 B —
  die Host-Erreichbarkeit ist **intermittierend**, der Cache `/tmp/opencode/saber_info.json`
  ist aktuell absent) nennt neben `ktemp`/`tpgpaltitude` **eine Scan-Mittel-Position**:
  `sclatitudeAVG`/`sclongitudeAVG` (deg) und `scaltitudeAVG` (**km**), dazu
  `tpl…`-Tangentialpunkt-Geometrie. Das Produkt trägt **kein** kartesisches xyz und
  **keine** Geschwindigkeit; Zeit ist `Time` (isotime, UTC). Der Arm heute
  (`extract.rs:6994-7006` family, `:7083-7095` scalar) liest **nur die letzte Zeile**
  (`data.last()`) und stempelt `epoch: now` — die `Time`-Spalte wird verworfen,
  `Position::Source` (unit-Variante, `types.rs:67`) gesetzt. Kein SABER-Block in `phi`.
  Korrigiert: die folge345-Zitation `extract.rs:7013-7025` für `Position::Source` ist
  **stale** (dort steht heute der `param`-Komponentenparse).
- **Präzedenz (gemessen):** eine eigene Weltlinie läuft im Haus **nicht** über
  `Position::StateVector { p, v }`, sondern über eine **Ephemeride**: `at <body>` →
  `Frame::Barycenter` → `Motion::Barycenter` → `body_barycenter_position` (Chebyshev,
  ICRF/J2000, **Meter**, TDB-Sekunden seit J2000), geübt von **Voyager**
  (`phi:27461-27466` Ephemeride + `phi:11603-11613` Datenblock, beide `at voyager1`).
  ISS hat nur die Ephemeriden-Seite, keine Kanäle. `Position::StateVector { p,v }` +
  `Motion::Linear` ist verdrahtet (`channels.rs:1408-1409`, `motion.rs:436-439`), hat aber
  **keinen** Produktionsproduzenten und nie `track: true`.
- **Route B ist NICHT quellen-blockiert — korrigiert (2026-10-14, `archive_search --all`):**
  TIMED ist **NORAD 26998** (2001-055B; Bahn ~615 km, i 74.1°), und **TLEs existieren**
  (CelesTrak/Space-Track; Tracker `in-the-sky.org/spacecraft.php?id=26998`,
  `orbitalradar.com/satellite/26998`, `n2yo.com/satellite/?s=26998`,
  `satellitemap.space/sat/26998`). Eine **TLE/SGP4-eigene Bahn** ist damit baubar
  (Kilometer-Klasse) und legt TIMED als **Peer-Körper** an. Zusätzlich gefunden:
  NTRS `20040082147` „Orbit Determination of the TIMED Mission Using TDRSS Differenced
  One-way Doppler Tracking Data"; `timedsds.jhuapl.edu`; `helio.data.nasa.gov/mission/TIMED`.
  **Was zuerst gemessen wurde (unvollständig):** `--verdict …/naif/TIMED/kernels/spk/` →
  **HTTP 404**; NAIF-Wurzel (200) → kein `TIMED/`; Horizons `COMMAND='TIMED'` → `No matches`.
  Das prüfte nur die **SPK-Güte** (NAIF/Horizons) und übersah die TLE-Stufe — der erste
  „blockiert"-Schluss war eine **unvollständige Messung, nicht die Wahrheit** (A=A).
  **TLE verifiziert (Browser, Wayback 2026-02-22, Status 200; CelesTrak live Timeout):**
  `TIMED` · `1 26998U 01055B 26052.51864582 .00001123 …` ·
  `2 26998 74.0671 107.8683 0002724 294.3335 65.7565 14.94968599313531`
  (i 74.07°, ~14.95 Umläufe/Tag) — die eigene Bahn ist damit belegt.
- **Exaktheit (Operator-Kriterium):** A ist exakt am **vermessenen Ort** (Limb-Sounder: der
  Tangentialpunkt `tpl…`/`tpgpaltitude` ist die Messstelle, nicht der Satellit) — aber
  scan-gemittelt, ohne Datum, ohne Sub-Scan-Zeit. **B (TLE/SGP4)** gäbe eine kontinuierliche
  Bahn mit `v` über die ganze Mission — **Kilometer-Klasse**, datum-/ICRS-frei — und wäre in
  **Zeit/Dynamik** exakter als A.
  **Ephemeriden-Nutzen:** B legt einen Peer-Körper in `body_ephemerides` (heute 92) an;
  A legt keinen an.
- **Roster-Runde (mountain-ui, 2026-10-14) — zweiter Anlauf, korrigierte Prämisse:**
  Frage A/B mit der verifizierten TLE neu gestellt. Geantwortet **B** (Claude, Mistral, Qwen,
  Perplexity, Z.ai/GLM-Deep-Think-Max). Claude: „die Sonde braucht die Satellitenposition,
  die liefert nur die SGP4-Bahn; der Produktpunkt ist der Tangentenpunkt und bleibt als
  Messort-Attribut je Zeile" — **Vorbehalt:** für 2002–2026 braucht es die **historische
  TLE-Reihe** (eine Einzel-TLE trägt Tage–Wochen). **A** sagte nur Lumo (die Messpunkte seien
  exakter als die ~km-SGP4-Rekonstruktion). Ohne Antwort: DeepSeek, Duck (Frage erneut nicht
  gepostet). Der **erste Anlauf** (falsche Prämisse „keine Bahn") war **A** — 4×.
- **Rat-Futter:** Recherche (primär: TLE verifiziert; NAIF/Horizons kein TIMED) + dieser
  Roster liegen vor. Mit den **acht** mountain-ui-Sitzen gespeist; es fehlen weiter Gemini,
  MiniMax, Kimi, Sakana + open-weight — **nicht die volle Liste**.
- **Braucht:** (0) ggf. die restlichen Sitze + den Rat; (1) die **historische TLE-Reihe** für
  2002–2026 (CelesTrak/Space-Track) prüfen; (2) den Arm **je Zeile** lesen lassen (heute nur
  die letzte) und `Time` je Zeile nach TDB (`parse_iso_tdb`); (3) Block-Query über einen
  Zeitraum; (4) Terms `saber-rules-of-the-road`.

### Venus Radar — Projektion gemessen; Bild-Manifestation offen (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Projektions-/Manifestations-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Bundle `urn:nasa:pds:venus_radar_level2` v1.0,
  DOI `10.17189/1522443`, **CC0-1.0**, 15 Produkte (5 Epochen × `inc`/`ocp`/`scp`).
  Die Level-2-Labels sind PDS4 `Array_2D_Map`, single-band; `pds4_img` braucht keinen
  Interleave-Umbau. Der `SAMPLE_INTERLEAVED`/`BANDS=2`-Befund (mycelium-346-6) gehört zum
  **rohen** PDS3 Complex-Backscatter-Bundle (`vrm_90xx`).
  **Projektion gemessen** (byte-identisch über alle Epochen, aus
  `.../data/venus_1988_north_inc.xml`): `map_projection_name` = **Sinusoidal**,
  `longitude_of_central_meridian` = **335.0 deg**; `pixel_resolution_x/y` = 1000 m/px;
  `upperleft_corner_x/y` = `-10000000.0` / `9359753.174046` m; Bounding-Box
  W 254.256 / E 37.213 / N 80.845 / S -25.0 deg; `Geodetic_Model`: `Planetocentric`,
  a=b=c=`6051.0` km, `Positive East`. **Kein** expliziter Frame-Identifier (absent) —
  der Frame ist nur über Körper-fix + `longitude_direction` konveyiert.
- **Gebaut (mountain-346):** `pds3_img` liest `BAND_STORAGE_TYPE` (BSQ/BIL/BIP);
  `decode_raster` indiziert danach; unbekannter Token → `UnknownInterleave` (kein
  stiller BSQ-Fallback); `pds3_bip_radar_compiler` delegiert an den Kern (Reshuffle weg).
- **Blockade:** keine.
- **Braucht:** (1) die **Bild-Manifestation (W)** wählen — wie das georeferenzierte
  `Array_2D_Map` als Gestalt/`pds4_img` in den Baum kommt; (2) den Block
  (`url`/`origin`/`compiler`/`terms CC0-1.0`, Einheit am Feld); (3) Split-Route erst
  nachgelagert; (4) der native Lauf braucht 15×960 MB (CI). Der D-Compiler
  (`venus_radar_compiler.rs`) und der VNRD-Reader (`vnrd.rs`) bleiben **geparkt**.

### ICGEM gfc — Rezept + Site gemessen; Auswerter wartet auf die Epochen-Ablage (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: das Modell wie die anderen behandeln) | **Bindung:** eigen
- **Trigger:** der Feld-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Das `gfc`-Format (ICGEM-Format-2023,
  `https://icgem.gfz.de/home` → Spec-PDF) trägt Header `product_type`, `modelname`,
  `earth_gravity_constant` (GM, m³/s²), `radius` (m), `max_degree`, `errors`,
  optional `norm` (fully normalized) / `tide_system`; Datensätze `gfc` (statische
  C_lm/S_lm) und `gfct` (zeitvariabel). Die **Synthese** ist die Standard-Kugelflächen-
  Entwicklung `V(r,θ,λ) = (GM/r)·Σ_l (a/r)^l ·Σ_m P̄_lm(cosθ)(C_lm cos mλ + S_lm sin mλ)`;
  das Feld = `∇V` (m/s²), Potenzial m²/s². **Präzedenz:** `IgrfCoeffs::synthesize`
  (`igrf.rs:131`), `vmf3_sh::b_c` (`vmf3_sh.rs:17`) + `vmf3.rs`-Produkte — beide aber
  **ohne** Runtime-Dispatch (nur Test bzw. `tools/measure`-Zeuge). `gfc.rs` ist heute
  Parser+Channels ohne Auswerter; die Header-Konstanten (GM/radius) liest der Compiler
  und **verwirft** sie (nicht im Frame). Empfohlene Site: eine `pub fn` in `src/archivar/gfc.rs`
  (Geschwister von `synthesize`/`b_c`), Eingang Koeffizienten (C/S,n,m) + GM/radius/norm/
  tide_system, plus Dispatch-Erweiterung in `extract.rs`/`main_flow.rs:3111-3226`.
- **Riss:** der Frame trägt GM/radius **nicht** — der Auswerter kann ohne sie nicht
  rechnen (Frame-Header oder Begleitgröße nötig); `norm` „fully normalized" (4π) ≠
  IGRF-Schmidt; `product_type` muss verzweigen (Gravitation vs. Ozeangezeiten); der
  Feldwert ist vektoriell (0c), die Höhenbezugsfläche (0b) und die Bahn↔Feld-Kopplung
  (0a) bleiben Bedingungen.
- **Blockade:** die **Epochen-Absenz** (eigener Punkt) — ein statisches `gfc` hat keine
  Zeit; ohne die Ablage würde der Pad `0.0` als J2000 gelesen.
- **Braucht:** (0) die Epochen-Ablage umsetzen; (1) GM/radius/norm/tide in den Frame
  oder an den Auswerter reichen; (2) die Kugelflächen-Synthese in `gfc.rs`; (3) Block
  `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`.

### Epochen-Absenz — Rats-Verdikt (b); Umsetzung offen (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14, Verdikt b) · **nach der Rat-Regel erneut
  zu befragen** (Recherche + voller Roster fehlten) | **Bindung:** eigen
- **Trigger:** der Registeratom / die nächste epochenlose Quelle
- **Lage:** (gemessen 2026-10-14, Taucher + Rat) Zwei Leselinien widersprechen sich an
  **demselben Wert 0.0**: Linie A liest `epoch <= 0.0` als **absent** und überspringt
  (`src/mathematikerin/machines/matrix.rs:644`; `src/archivar/drs_fits.rs:38`); Linie B
  behält `0.0` als Wert (`agrav.rs:49`; `hips.rs:793` Bandindex 0; `gfc.rs:153`).
  Entscheidend: `epoch` ist TDB-Sekunden seit J2000 — **`0.0` ist J2000.0**, ein echter
  Wert; `< 0.0` ist Prä-J2000 (bereits als Wert behandelt, `tests.rs:1115`). Linie A
  ist damit eine **Fabrikation** (sie löscht den Ursprung und die Vorgeschichte).
- **Rats-Verdikt (b):** Absenz **ausdrücklich tragen**, über das schon vorhandene
  Presence-Bit-Mask — neuer `PRESENCE_FLAG_EPOCH` (32.0), **kein** Wire-Umbau (26×f64,
  208 B bleiben). Static `gfc` → Bit aus (Pad 0.0 als *absent* gelesen, Manifestation zur
  Query-Zeit); `gfct` → Bit an (0.0 = J2000 legitim).
- **Offen (Operator-Regel, Commit `94c6eaa10`):** dieses Verdikt fiel **vor** der Regel
  „Rat MUSS gefüttert werden" — nur der Übergabe-Stand, **keine** `archive_search`-Recherche,
  **kein** Roster (`work/ui-seats.md`). Nach der Regel erneut befragen, bevor es bindet.
- **Blockade:** die Umsetzung berührt viele Sites (Write- und Read-Seite) — kein
  Teilumbau möglich ohne die Gegen-Fabrikation (ein Reader-only-Schnitt würde absent als
  J2000 lesen).
- **Braucht:** die Sites des Verdikts: `types.rs` `PRESENCE_FLAG_EPOCH` + `presence_flags`
  (um den Epoch-Parameter erweitern; die 4 Write-Aufrufer `spatial.rs:845/1004`,
  `membrane.rs:130`, Test in `parse.rs:3296`), Read-Seite `matrix.rs:644` (Bit statt
  `<= 0.0`), `drs_fits.rs:38` (nur `!is_finite`), `agrav.rs`-Write (Bit setzen, 0.0
  legal), `gfc.rs:153` (Bit lesen) **und** `gfc.rs:101-107` Frame-Fold (der statische Pad
  darf nicht als Frame-Epoch erscheinen), `hips.rs:793` (Bandindex → Bit an),
  `static/constants.js` (Bit 32 sichtbar machen), `docs/concepts/archivar-mathematikerin.md`
  (Presence-Klausel). **Erst dann** die `gfc`-Manifestation.

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
  `state/mountain/faden-term-field-ttl-draft.φ`: je Faden die gemessene
  `terms`/`field`/`ttl`-Zeile (Sonnen-Magnetfeld, Stern-Aktivität, 21 cm, Wolken, TGF),
  Haus-Token-Korrekturen belegt (`sdo`→`CC0-1.0`, `vizier-open`→`vizier-scientific-free`,
  `heasarc-free`→`PD`, `NOAA-CDR-open`→`PD`, Zenodo→`CC-BY-4.0`). Zwei Risse offen
  (WSO-Polarfeld ohne Terms; LISIRD ohne Terms), 21 cm EBHIS bereits registriert.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler` je Faden auf den Entwurf; dann
  `register_sort`.

### Adressierte Register-Fäden — Lizenzen gemessen, Verdikte offen (eigen)
- **Status:** eigen (Register-Writes; Messungen jetzt am Baum gegengeprüft) | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14 via `register_lookup --addressed mountain`, Taucher,
  und **direkt im Browser** auf den Seiten — JS-gerenderte Felder, die `archive_search`
  nicht las)
  - **MODVOLC HIGP** (`http://modis.higp.hawaii.edu/data/`): **keine** Lizenzzeile
    (genuine Absenz); nur `© University of Hawaii at Manoa, 2016` + NASA-MODIS-
    Förderhinweis. Kein HIGP-Block in `phi` (der NRT3-MODVOLC-Block `phi:12041` ist eine
    andere Quelle, `terms PD`). → `local-only`.
  - **DE-2 NACS/WATS — gesetzt (mountain-346).** SPASE-`RightsList` (Browser, `spase-metadata.org`
    …/DE2/WATS/PT2S): `RightsIdentifierScheme SPDX`, `RightsIdentifier CC0-1.0`,
    Note „… applicable to all publicly available NASA Heliophysics data products". Der
    `de2`-Block trägt jetzt `terms CC0-1.0` (`phi:854`, vormals `PD`).
  - **AWESOME-VLF/LF — gesetzt (mountain-346).** Browser auf
    `spase-metadata.org/ISWI/NumericalData/AWESOME/VLF/PT0.00001S.html`: JSON-LD
    `"license": "https://cdla.io/permissive-1-0/"`, DATA USAGE POLICY „WALDO data are
    released without restriction, and can be freely analyzed or published" (Danksagung
    erbeten). Exaktes Token **CDLA-Permissive-1.0** (nicht CC0). `phi:8266` (waldo) trägt
    jetzt `terms CDLA-Permissive-1.0 https://doi.org/10.48322/fzcf-f191` (vormals `CC0-1.0`).
  - **OCNS2019-Ersatz (JPL SSD / Horizons / NAIF) — gemessen (Browser).** Die JPL-SSD-Seite
    trägt **keine** Datum-Lizenzzeile (Footer: `URS Clearance: CL#21-4165`, Privacy, Image
    Policy); die Caltech/JPL-Notiz deckt **Dokumente/Bilder** („JPL-authored documents …
    may be protected … Permission to reproduce may be required"), **nicht** die Daten. Die
    **Daten** fallen unter die NASA-SMD-Politik **SPD-41a**
    (`science.nasa.gov/researchers/science-information-policy/`): „Mission data are released
    as soon as possible", offen, kein Embargo. **NAIF** (`naif.jpl.nasa.gov/naif/credit.html`,
    „Rules/Giving Credit") bittet um Nennung, ohne Lizenz-Beschränkung.
    → **offen, Danksagung erbeten** (kein CC0-Zeichen auf der Seite).
- **Blockade:** MODVOLC braucht einen **neuen** Block (kein HIGP-Block in `phi`) — Mycelium
  setzt `url`/`origin`/`compiler`, Mountain das Terms-Verdikt.
- **Braucht:** MODVOLC-Block + `terms local-only`. DE-2 und WALDO sind gesetzt.
- **Für die eigenen Ephemeriden (Operator-Frage):** unsere Bahnen ruhen auf **JPL/NAIF** —
  die DE-Kerne (DE440/441/442 registriert), `horizons_compiler` (Horizons-API) und
  `spacecraft_ephemeris_compiler` (NAIF-SPK). Diese Daten sind **offen** (NASA SMD, SPD-41a),
  die Nennung erbeten — also für Bau und Weitergabe der eigenen Ephemeriden frei; die drei
  Punktquellen (MODVOLC/WALDO/DE-2) tragen dazu nichts bei.

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
  (BSQ/BIL/BIP); der BIP-Arm delegiert an den Kern-Decoder. Echo.
- `xfer:mycelium-346-2` **Venus-Radar** — Parser-Blocker gelöst; die Projektion ist
  gemessen (Sinusoidal, CM 335°E). Die Bild-Manifestation (W) und der Block folgen.
- **Fünf Fäden:** der Mountain-Teil liegt als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ`. Bitte `url`/`origin`/`compiler` je Faden
  daraufsetzen; danach `register_sort`.
- `xfer:mycelium-346-3`/`-4`/`-5`/`-7` — Terms gemessen (s. Punkt „Adressierte
  Register-Fäden"); Mountain setzt die Verdikte nach eigener Messung.

## An river

Origin: mountain-346 (2026-10-14):

- `xfer:mountain-342-6` **SABER:** die Registerzeile folgt nach der Routen-Entscheidung;
  der Host ist intermittierend erreichbar (heute HTTP 200), das Sample bleibt bis zur
  Route ungemessen.
- **UCAR-FPI:** die zwei `field`-Zeilen stehen code-gemessen; offen bleibt allein die
  FPI-eigene Athabasca-Instrumentenkoordinate.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344, 345, 346).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Kette gefahren; Verdikt (Rat + Roster): **nT²/Hz primär (Kalibrierung im Arm, linear), V²/Hz als Rohkanal**; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh, Kalibrierung versioniert im Arm) | 2026-10-14 | (mountain 344 — Operator legte das Verdikt der Stimmen-Kette vor).
- **„was wenn du den satellit autonom betrachtest — dann ist es doch auch nur eine sonde an einem xyz,t; die ISS ist doch zu einer Zeit immer an einem bestimmten Ort"** | 2026-10-14 | (mountain 345 — SABER: eigene Weltlinie, supersediert B2).
- **„warum können wir das gravitationsmodell nicht wie die anderen behandeln? das Erdmagnetfeld-Modell und das Atmosphären-Modell werden an einem Ort ausgewertet und geben dort einen echten Feldwert — sie sind ein Feld"** | 2026-10-14 | (mountain 345 — ICGEM: Feld, supersediert Witness).
- **„fängst du bitte an und delegierst an taucher?"** (die offenen Punkte aufnehmen, Taucher dispatchen) | 2026-10-14 | (mountain 346).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. In diesem
Atom (mountain-346) committet: `src/archivar/pds3_img.rs` ·
`tools/harvest/src/bin/pds3_bip_radar_compiler.rs` · diese Übergabe · der
`archiv/`-Move der folge345 (erste Handlung); diese Fortschreibung der Übergabe
(zweite Handlung).
