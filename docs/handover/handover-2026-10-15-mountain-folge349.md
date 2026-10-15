<!--
  title: Handover — Mountain-Folge 349 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: b69c4026902e28499a2ee7b1e61dbcb7505701940fe9d07c9b246566368d5d62
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md
-->
# Handover — Mountain-Folge 349 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-15-mountain-folge348.md` (→ `archiv/`).
Gefaltet (`register_lookup --addressed mountain`): `mycelium-folge351` (die sechs
`xfer:mycelium-351-1…-6` sind geantwortet — die Kraft-/Lizenz-Zeilen stehen),
`river-folge213` (der `ci-gate`-Befund; nicht Mountain-Eigentum). Still gemessen:
`register_lookup --fired mountain` = 0, `--stale mountain --persist 3` = 0,
`open_points_check` sauber (14 Pfad-Refs, 0 absent/stale/word-carried-format).
Der Stehende Pass wird als **öffentliche** Fassung zitiert, nie kopiert.

## Burn: open 0.0 · close nicht gemessen — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt), **open und close sind beide nicht
gemessen** (der `0.0` ist der Gate-Platzhalter, kein Meßwert; Vorbehalt wie
river-212/mountain-348). Grund: die Register-Schreibakte (sechs Quellen + 32
WDC-B-Blöcke), die `alt`-Deklaration, die drei Korrekturen.

## Offen (aufgeschlüsselt)

Die sechs Register-Transporte `xfer:mycelium-351-1`, `xfer:mycelium-351-2`,
`xfer:mycelium-351-3`, `xfer:mycelium-351-4`, `xfer:mycelium-351-5`,
`xfer:mycelium-351-6` sind **geantwortet** (Echo) — die Kraft-/Lizenz-Zeilen stehen
(s. „Register-Akte dieses Runds").

### Epochen-Absenz — Ratsrunde vollständig; Operator-Wort: direkt der Unterscheider (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14, Verdikt b) · voller Roster 2026-10-15
  gefahren · **Operator-Wort 2026-10-15: „nein direkt der unterscheider mit archive search und
  rat" — die Bit-Brücke ist verworfen**, der Unterscheider ist ein eigener Punkt (s. u.) | **Bindung:** eigen
- **Trigger:** der Registeratom / die nächste epochenlose Quelle
- **Lage:** (gemessen 2026-10-15, Ratsrunde) Die Verdikt-Form steht: `PRESENCE_FLAG_EPOCH`
  = `32.0` neben `PRESENCE_FLAG_QUANTITY` = `16.0` (`types.rs:402-406`); `presence_flags`
  (`types.rs:441`) trägt heute keinen Epoch-Parameter. Die kollidierenden Leselinien sind
  bestätigt: Linie A `channel.epoch <= 0.0 → continue` (`matrix.rs:644`) und
  `drs_fits.rs:38` (`epoch <= 0.0 → None`); Linie B behält `0.0` als Wert.
- **Riss in meinem eigenen Mitschnitt (ungeglättet):** meine Sakana-Aufzeichnung trug
  „Ja, ausreichend und ehrlich", die Operator-Runde trug „**Nein**" — zwei Läufe, zwei Verdikte.
  **Baum-Verifikation:** der „u32/u64"-Einwand der Stimmen ist ein Namens-Artefakt — die Maske ist
  ein f64-Bitfeld aus Potenz-Werten (`1.0 … 16.0`), ein neuer Flag ist der Wert `32.0`.
- **Verdikt (gebunden):** `PRESENCE_FLAG_EPOCH = 32.0` als nicht-brechende Brücke ist
  **verworfen** (Operator-Wort) — die Entscheidung fällt auf den Modellart-Unterscheider.
  Der Riss der Leselinien (`matrix.rs:644`, `drs_fits.rs:38` vs. `agrav.rs`/`gfc.rs`/`hips.rs`)
  bleibt als **Zeuge** stehen, bis der Unterscheider ihn an der Wurzel löst.
- **Blockade:** keine.
- **Braucht:** nichts mehr an dieser Stelle — die Umsetzung trägt der eigene Punkt
  „Modellart-Unterscheider".

### Modellart-Unterscheider — Recherche + voller Rat; Verdikt gebunden (eigen)
- **Status:** eigen · **Operator-Wort 2026-10-15: direkt der Unterscheider** (statt der Bit-Brücke)
  · **Rat-Runde vollständig — Verdikt gebunden** | **Bindung:** eigen
- **Trigger:** die ICGEM-Manifestation
- **Lage:** (gemessen 2026-10-15) Der Wire bleibt fix 26 × f64. **Recherche-Haus-Präzedenz:** der
  Baum trägt den Unterscheider am Rand **schon als `Option`** — `SourceConfig.catalog_epoch:
  Option<f64>` (`types.rs:526`) und `itrf_sinex.rs:40` `ref_epoch: Option<RefEpoch>`; ICGEM taggt
  selbst `gfc`/`gfct`. **Verdikt (korrigiert am Baum):** **Achse 1** — `model_kind` source-level
  (`static`|`time_variable`) ist die Wahrheit; der Draht behält den 0.0-Pad; **NaN nie**
  (Presence-Bit per Operator-Wort verworfen).
- **Gebaut:** `ModelKind` + `source_model_kind(format)` + `SourceConfig.model_kind` (`types.rs`),
  abgeleitet im Parser (`parse.rs`); die zwei Leselinien löschen `0.0`/negative Epochen nicht mehr.
  Die `phi`-Direktive `model_kind static|time_variable` steht (`parse_model_kind`); sie
  **überschreibt** die Format-Ableitung. Die Maschinen-Naht trägt `Option<ModelKind>` mit
  (`main_flow.rs:949/:1377` → `LoopCtx.machine_rx` → `MatrixMachine`); `record` weist einen
  `Static`-Batch vor dem Wert ab. `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** nichts an dieser Stelle — die `model_kind`-Direktive landet mit der
  ICGEM-Manifestation (kein Block trägt sie heute; `gfc` leitet `static` schon aus dem Format ab).

### SABER — verdiktet: B + A (eigene Weltlinie + Messvolumen) (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14: B + A) | **Bindung:** eigen
- **Trigger:** der Arm-Bau
- **Lage:** (gemessen 2026-10-14, `ca35522f2`) Der Zeilen-Arm steht; SABER nimmt den HAPI-`parameters`/info-Pfad,
  jede Zeile trägt eigenen `epoch` (`series_epoch_of`) und `Position::Surface{earth,…}`. Die TIMED-Weltlinie
  (Peer-Körper) ist **gebaut** (`timed_tle_compiler.rs`, WOB1; Roundtrip parst). **Riss benannt:** die
  Übergabe nennt „Schritt 2" einen `Position::StateVector{track:true}`-Produzenten; gebaut ist der
  `orbit_bin`-Weg als `Motion::Barycenter`. Beide Zeugen stehen, keine Glättung.
- **Blockade:** keine (der Bau ist offen, kein externes Tor).
- **Braucht:** (1) den **TIMED-Register-Block** (`format orbit_bin`, `at timed`; Mycelium setzt
  `url`/`origin`/`compiler` — `xfer:mountain-348-5`); (2) die CI-Naht `timed-orbit-cdn.yml`;
  (3) das **Riss-Gate** (`measure`-Sonde, `VerdictWord::Riss`); (4) den SABER-Block (`at earth`,
  Tangentialgeometrie, Höhe als geschlossene Level-Achse).

### Venus Radar — Projektion gemessen; VNRD-Block steht; Bild-Manifestation offen (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Projektions-/Manifestations-Bau
- **Lage:** (gemessen 2026-10-15) Der Level-2-**VNRD-Block** steht (`phi/sources.φ`: `url …/venus_radar_level2.bin`,
  `terms CC0-1.0`, `origin` Bundle-Site, `compiler venus_radar_compiler.rs`, `format vnrd`,
  `at venus`, `no-cadence`, `field venus_radar_inc_deg`). Die Projektion ist byte-identisch gemessen.
- **Blockade:** keine.
- **Braucht:** (1) die **Bild-Manifestation (W)**; (2) die CI-Naht (15×960 MB). Der rohe
  PDS3-Complex-Backscatter bleibt ein separater Gegenstand (kein Block in `phi`).

### ICGEM gfc — Rezept + Site gemessen; wartet auf die Epochen-Ablage (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: das Modell wie die anderen behandeln) | **Bindung:** eigen
- **Trigger:** der Feld-Bau
- **Lage:** (gemessen 2026-10-14) `gfc.rs` ist Parser+Channels ohne Auswerter; die
  Kugelflächen-Synthese fehlt. Präzedenz: `IgrfCoeffs::synthesize`, `vmf3_sh::b_c`. **Riss:** der Frame
  trägt GM/radius **nicht**; `norm` „fully normalized" (4π) ≠ IGRF-Schmidt; `product_type` muss verzweigen.
- **Blockade:** die **Epochen-Absenz** (eigener Punkt).
- **Braucht:** (0) die Epochen-Ablage (Modellart-Unterscheider); (1) GM/radius/norm/tide in
  den Frame oder an den Auswerter reichen; (2) die Kugelflächen-Synthese in `gfc.rs`; (3) Block
  `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`.

### UCAR-FPI — Arm + Zeilen stehen; Athabasca-Koordinate gemessen (river-213) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-15, river-213 `465e11940`) Arm + Guard stehen (`fpi_ucar.rs`, Dispatch `extract.rs:159`); die zwei `field`-Zeilen
  sind code-gemessen (`fpi_mer_wind_ms`, `fpi_zon_wind_ms`). Die Athabasca-Instrumentenkoordinate ist
  gemessen (`on earth 54.60282 -113.64442 693`, AUGO II/AUGSO, orthometrisch). Die stale Zeile
  „Instrumentenkoordinate ungemessen" ist in `folge346`/`folge347` **gestrichen**.
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN).
- **Braucht:** den UCAR-FPI-Block (Mycelium: `url`/`origin`/`compiler`) + die zwei `field`-Zeilen +
  die Arbeitskopie-Route (`local-only`).

### Fünf fehlende Fäden — Entwurf steht, Mycelium setzt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) Mountain-Teil fertig als Entwurf `state/mountain/faden-term-field-ttl-draft.φ`
  (Sonnen-Magnetfeld, Stern-Aktivität, 21 cm, Wolken, TGF; Haus-Token-Korrekturen belegt).
  Zwei Risse offen (WSO-Polarfeld ohne Terms; LISIRD ohne Terms); 21 cm EBHIS bereits registriert.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler` je Faden auf den Entwurf; dann `register_sort`.

### Quellen-Gewebe — Kraft × Kernel gemessen; die dünnen Fäden (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) Die zwei Messungen liegen in
  `docs/surveys/survey-2026-10-14-gewebe-faeden.md`. Schon registriert: IGETS, AGrav, GRACE-FO,
  ISC, EarthScope-FDSN, GEOFON-Event/-Wellform, EIDA-Terms, **GSRAS** (1993-Catalog) und
  **NGL-ARDA** (als **GNSS-Zeuge** umklassifiziert) — die Kraft-/Lizenz-Zeilen stehen
  (mountain-349). Offen/gesperrt: EGGN/EPOS-Equip-G, CGrav/CEIC, GOCE, NIED.
- **Blockade:** die Kernel-Zuordnung ist keine Pro-Solo-Entscheidung — das Meßgesetz einer Quelle
  berührt den Force-/Kernel-Kontrakt (Rat).
- **Braucht:** (0) IGETS-`origin` — bereits `sftp://igetsftp.gfz.de` (Myceliums Zeilenklasse);
  (1) die Kernel-Lücke schließen (Rat); (2) EGGN/China/GOCE/NIED als `pending`/`riss` benennen.

### Adressierte Register-Fäden — Lizenzen gemessen, MODVOLC-Verdikt steht (eigen)
- **Status:** eigen (Register-Writes) | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14, Browser) MODVOLC HIGP (`http://modis.higp.hawaii.edu/data/`): **keine** Lizenzzeile (genuine
  Absenz) → `local-only`; kein HIGP-Block in `phi`. DE-2 (`phi:854`) und AWESOME/WALDO (`phi:8266`) sind gesetzt.
  OCNS2019-Ersatz: JPL SSD/Horizons/NAIF offen (NASA SMD, SPD-41a), Danksagung erbeten, kein CC0-Zeichen.
- **Blockade:** MODVOLC braucht einen **neuen** Block — Mycelium setzt `url`/`origin`/`compiler`
  (`xfer:mountain-348-1`).
- **Braucht:** MODVOLC-Block + `terms local-only`; OCNS2019-Ersatz als offene Alternative
  registrieren (`xfer:mountain-348-4`).

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide; `docs/specs/force-unit-baseline.txt` trägt sie (plus `em mev`)
  froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund
- **Lage:** (gemessen 2026-10-14) ALFALFA `local-only` bestätigt (kein Block in `phi`); `opentopography_raster_compiler.rs`
  schreibt ein SRTM-Gestalt-Witness ohne Geoid-/Datum-/Toleranz-Anker; `load_gestalt_surface_threads`
  (`main_flow.rs:580`) hat nur `tests.rs` als Aufrufer; `ilrs_npt_compiler.rs` steht, aber **kein**
  Lese-Arm in `src/`; `icrf3_compiler.rs` schreibt JSON, kein SKD1/Golden-File/Round-Trip.
- **Blockade:** keine.
- **Braucht:** (1) Höhenbezug + Toleranz vor der `opentopography_raster`-Registrierung, dann Block;
  (2) `load_gestalt_surface_threads` an einen Produktions-Aufrufer binden **oder** `descoped`;
  (3) `ilrs_npt`-Lese-Arm in `extract.rs` + Block; (4) `icrf3` erst Vertrag versionieren + Round-Trip
  + Golden-File, dann SKD1. `docs/SOURCE_PORT.md`.

## Register-Akte dieses Runds (mountain-349)

Die sechs `xfer:mycelium-351-1…-6` sind gesetzt (Myceliums `url`/`origin`/`compiler` standen):

- **GSRAS-Teleseismic** (`gsras_teleseismic_catalog_1993.txt`): `terms free-open http://www.gsras.ru/`
  (die Indexseiten nennen keine formale Lizenz, nur Copyright; die Daten sind offen) ·
  `field gsras_teleseismic_catalog_magnitude … erfc seismic-surface Mw 31536000 0.0 0.0`.
- **CMONOC-GNSS (Zenodo 7798617)** — zwei Blöcke: `terms CC-BY-4.0 https://zenodo.org/records/7798617`
  (Zenodo-API `license.id = cc-by-4.0`, gemessen) · je ein Velocity-Feld
  (`cmonoc_continuous_velocity` / `cmonoc_campaign_velocity`, `gaussian-inverse-square seismic-body
  mm/yr 31536000 0.0 0.0` — Haus-Präzedenz `port.rs:2600` slip-rate → seismic-body mm/yr).
- **CEDC (Zenodo 8084659)**: `terms CC-BY-4.0 https://zenodo.org/records/8084659` (gemessen) ·
  `field cedc_catalog_magnitude … erfc seismic-surface Mw 31536000 0.0 0.0`.
- **NGL-ARDA**: `terms free-open https://nbmg.unr.edu/Copyright.html` (die NBMG/UNR-Policy-Seite,
  Nennung erbeten) — **GNSS-Zeuge**, keine Schwere/Seismik.
- **ISC** (`phi`-Block `isc_bulletin.iscb`): die fehlende Kraft-Zeile
  `field isc_bulletin_magnitude … erfc seismic-surface Mw 604800 0.0 0.0`.
- **WDC-B/Obninsk — Erbe-Rettung** (32 `raw_mirror`-Blöcke): `terms CC-BY-NC-4.0
  http://creativecommons.org/licenses/by-nc/4.0/deed.ru` (Seiten-Fuß) · je
  `field obn_obninsk_depth … erfc seismic-surface km …` und
  `field obn_obninsk_magnitude … erfc seismic-surface Mw …` (fixed-column Text, lat/lon/Tiefe/M).
- **IGETS/GFZ**: kein Auftrag — `terms CC-BY-4.0` und `field igets_gravity_nm_s2` standen; Mycelium
  setzte nur die fehlende `compiler`-Zeile (`igets_compiler.rs`). **BGI** = `agrav`-Block,
  **CB** = Netzwerk-Eintrag → keine eigene Zeile.

Gemessen: `register_sort` canonical (2 614 Blöcke) · `source_physics_lint` parst 7 760 Felder,
kein Physik-Mismatch; die neuen Felder in `seismic-surface × erfc` und `seismic-body ×
gaussian-inverse-square`.

## Korrekturen (beide Enden genannt, nie geglättet)

1. **ARDA** — die Gravimetrie-/Seismik-Zuordnung war eine **Kategorienverwechslung**; `ARDA` ist der
   **GNSS-Stationscode** der NGL/MAGNET-Station (Ardahan, Türkei). Umklassifiziert: **GNSS-Zeuge**
   (`ngl_arda.tenv3`, `on earth 41.111 42.699 1834.093`), nicht Schwere/Seismik.
2. **UCAR-FPI Athabasca** — die stale Zeile „die FPI-eigene Athabasca-Instrumentenkoordinate ist
   ungemessen" (`archiv/handover-2026-10-14-mountain-folge346.md`,
   `-folge347.md`) ist **gestrichen**; gemessen steht `on earth 54.60282 -113.64442 693`
   (AUGO II / AUGSO, orthometrisch MSL/EGM96; SRTM30m am Punkt 696.0 m).
3. **„EGGN"** — **Fehlname** (die realen Akronyme: ECGN/UEGN/EGGP); die Schwere-Daten liegen bei
   IGETS/BGI. Die Zeile zeigt auf **EQUIP-G** (`pending`; Daten erst nach 2029, Zugang per
   E-Mail-Registrierung `https://equip-g.eu`) + die realen Schwestern **IGETS/GFZ** (`CC-BY-4.0`)
   und **BGI** (frei ohne Einschränkung). Kein `phi`-Block (kein EGGN/EQUIP-Eintrag).
4. **`alt`-Deklaration (Mycelium-316 geschlossen)** — `on earth <alt>` trägt **eine** Haus-Skala:
   **orthometrisch (MSL/EGM96)**. Steht einmal im Datenkontrakt
   (`docs/concepts/archivar-mathematikerin.md`, Motion-Laws). Kein neues Feld, kein Umbau.

## An mycelium

Origin: mountain-349 (2026-10-15):

- `xfer:mountain-348-3` **IGETS `origin`** — der Block trägt heute `origin sftp://igetsftp.gfz.de`
  (die Rohdaten-Naht), die Projektseite `igets.gfz-potsdam.de/` ist nicht mehr die `origin`-Zeile.
- `xfer:mountain-348-6` **GEOFON-Wellenform** — `xfer:mycelium-350-1` ist geantwortet (Echo): das
  `field` steht.

## An river

Origin: mountain-349 (2026-10-15):

- `xfer:mountain-348-7` **SABER** — die Registerzeile folgt nach der Routen-Entscheidung
  (Rats-Verdikt B + A, s. „Offen"); der Host ist intermittierend erreichbar.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344–348).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Verdikt (Rat + Roster): nT²/Hz primär, V²/Hz roh; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh) | 2026-10-14 | (mountain 344).
- **„was wenn du den satellit autonom betrachtest … die ISS ist doch zu einer Zeit immer an einem bestimmten Ort"** | 2026-10-14 | (mountain 345 — SABER: eigene Weltlinie).
- **„warum können wir das gravitationsmodell nicht wie die anderen behandeln? … sie sind ein Feld"** | 2026-10-14 | (mountain 345 — ICGEM: Feld).
- **„fängst du bitte an und delegierst an taucher?"** | 2026-10-14 | (mountain 346).
- **„bitte fahre die frage über trying open"** | 2026-10-15 | (mountain 348 — die offenen Gewichte in die Epochen-Absenz-Ratsrunde).
- **„nein direkt der unterscheider mit archive search und rat"** | 2026-10-15 | (mountain 348 — die Bit-Brücke verworfen; der Modellart-Unterscheider als eigener Punkt).
- **„Starte die Mountain-Linie in einem Pass … Die Kraft-/Lizenz-Zeilen aus Myceliums sechs Aufträgen setzen (terms/field/ttl je Quelle), ARDA formal umklassifizieren, die UCAR-stale-Zeile streichen, „EGGN" korrigieren, die `alt`-Deklaration in den Datenkontrakt."** | 2026-10-15 | (mountain 349) — die Register-Schreibakte dieses Runds.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-349) committet: die sechs Quellen-Zeilen in `phi/sources.φ` (GSRAS, CMONOC ×2,
CEDC, ARDA-`terms`, ISC-`field`, 32 WDC-B-Blöcke mit `terms` + `field`), die `alt`-Deklaration
in `docs/concepts/archivar-mathematikerin.md`, die Streichung der UCAR-stale-Zeile in den
archivierten folge346/347 und diese Fortschreibung.
