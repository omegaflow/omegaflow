<!--
  title: Handover — Mountain-Folge 351 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 57460d2c543f29eb18e21b2bc80488f5a74ccf91f64d1c8c823ab96da15db9a3
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md, docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md
-->
# Handover — Mountain-Folge 351 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-15-mountain-folge350.md` (→ `archiv/`).

**Der benannte Auftrag ist am Baum getragen — keine Doppelarbeit.** Die vom Operator
genannte Übergabe `briefs-2026-10-15-mountain-mycelium.md` (§&nbsp;/mountain) trägt
die sechs Register-Schreibakte (`xfer:mycelium-351-1…-6`), die zwei WDC-B-Zeilenklassen
und die vier Korrekturen. Am Baum gemessen (2026-10-15): alle sechs stehen —
mountain-349, `36b50dfca`: GSRAS `terms free-open` + `field
gsras_teleseismic_catalog_magnitude` · CMONOC ×2 `terms CC-BY-4.0` + die zwei
Velocity-Felder · CEDC `terms CC-BY-4.0` + `field cedc_catalog_magnitude` · NGL-ARDA
`terms free-open` als **GNSS-Zeuge** (`on earth 41.111 42.699 1834.093`) · ISC `field
isc_bulletin_magnitude` · 32 WDC-B-Blöcke `terms CC-BY-NC-4.0` + `field
obn_obninsk_depth`/`obn_obninsk_magnitude`. Die vier Korrekturen stehen (ARDA
umklassifiziert; die UCAR-stale „ungemessen"-Zeile in folge346/347 gestrichen;
„EGGN" in zwei Surveys korrigiert; die `alt`-Deklaration in
`docs/concepts/archivar-mathematikerin.md:16`). Der lebende `sep/`-Baum bleibt
Mycelium-gebunden (`terms wdcb-sep-policy` via `xfer:mountain-349-1`). **Der Auftrag
wird nicht erneut ausgeführt.**

Still gemessen: `register_lookup --fired mountain` = 0, `--stale mountain --persist
3` = 0; `open_points_check` auf dem Auftrag sauber; auf dem Blatt
`handover-2026-10-15-quellen-neu-und-riss-ucar.md` zwei **ABSENT**-Refs (die
`see-also`-Zeilen auf die nach `archiv/` gewanderten folge346/folge212) — geheilt.
Gefaltet (`register_lookup --addressed mountain`): `mycelium-folge351` (die sechs
`xfer:mycelium-351-1…-6` sind geantwortet) und `river-folge213` (der `ci-gate`-Befund;
nicht Mountain-Eigentum) — beide von folge349 bereits gefaltet. Der Stehende Pass wird
als **öffentliche** Fassung zitiert, nie kopiert.

## Burn: open 0.0 · close nicht gemessen — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt); der Platzhalter `0.0` ist **kein** Meßwert,
die Session-Kosten sind nicht gemessen (Vorbehalt wie river-212/mountain-348/349).

## Offen (aufgeschlüsselt)

Die sechs Register-Transporte `xfer:mycelium-351-1…-6` sind **geantwortet** (Echo) und am
Baum getragen (git: `36b50dfca`). Was folgt, ist offen.

### Ketten-Quellen (SWIFF · WINDMI · MAGION · MOSWOC-Kp · OVATION-AACGM) — harter Lauf gemessen (eigen)
- **Status:** eigen · **gemessen 2026-10-15** (fünf Taucher + vier Mountain-`bin/all_wide`-Läufe) | **Bindung:** eigen
- **Trigger:** Operator-Wort 2026-10-15 („taucher mit harten bandagen und archive search all") — abgearbeitet.
- **Lage:** (gemessen 2026-10-15 via fünf Taucher + vier `bin/all_wide`-Läufe + `archive_search --root phi` + `--jina`/`--verdict`) volles Messblatt `docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md` (sha `ff8d9a9b…`); keine der fünf steht in `phi/sources.φ` (`--root phi` = **0/0** für alle fünf). Ergebnis:
  - **SWIFF** — FP7-Projekt/Paper/Modell (CORDIS 263340, DOI `10.1051/swsc/2013027`, Nachfolger BSPM bei PITHIA); **kein Datensatz** (absent auf Zenodo/DataCite/re3data/…, gemessen).
  - **WINDMI** — CCMC-Rechenmodell (Dst/AL), Live-Nowcast + Rolling-`.gif`-Baum 2012–2026, ~1 h Vorlauf bestätigt (DOI `10.1029/2008SW000459`); **kein numerisches Produkt, keine DOI**; **Riss:** „Nutzung auf Anfrage" nicht getragen (Dauerlauf, keine Request-Route, 404).
  - **MAGION** — tschechische Mikro-Subsatelliten-Serie (S2-X/S2-A), Magnetometer **SG-R8/DMA** separat; **kein offenes MAGION-Produkt mit DOI** (nur PI-gated IKI, Host `worsta.iki.rssi.ru` TCP-Timeout); offene DOI'te Magnetometer-Daten sind der Hauptsatellit IMAP-3 (CDPP `10.24400/802406/mqcs-ddjr`, NASA/SPDF CC0).
  - **MOSWOC-Kp** — Zenodo `10.5281/zenodo.5211482` (Bingham 2021), **`access_right: restricted`, `files: []`, 403** — **nicht frei abrufbar**; Lizenz Non-Commercial Government Licence v2.0; Span 2019-03-01…09-30, Takt 3 h, Horizont 3 Tage, Kp+Kuk.
  - **OVATION-AACGM** — Zenodo `10.5281/zenodo.19651247` (Zhao 2026), **CC-BY-4.0, offen**; eine Datei `OVATION_AACGM.zip` (4.9 MB, md5 `1dfcbbe8…`); **eine Station (Mohe)**, nicht „je Station".
- **Risse (unglättet):** „frei abrufbar" (river) vs. `restricted` (MOSWOC) · „je Station" (river) vs. **eine Station** (OVATION) · „ein Satelliten-Magnetometer" (river) vs. **Subsatelliten-Serie + separates Instrument** (MAGION) · „Nutzung auf Anfrage" (river) vs. **Dauerlauf** (WINDMI) · river-214:293 „Zugangswege im Ketten-Punkt" vs. Baum (kein solcher Block).
- **Form-Defekt (gemessen):** der Taucher-Sandbox verweigert `bin/all_wide` (dreimal); Mountain führt den Wrapper ohne Beanstandung aus. Die Profil-Allowlist deckt den Wrapper nicht → die kanonische weite Leiter war den Tauchern verschlossen (Breite lief über `--all` + Einzelmodi).
- **Blockade:** keine.
- **Braucht:** (1) die **Register-/Feld-/Kernel-/Format-Entscheidung** für OVATION-AACGM (Rat — Pro-Solo-Grenze: Feld-/Kernel-Kontrakt); (2) den **MOSWOC-Kp-Zugang** als per-Akt-Operator-Wort (Zenodo-Access-Request), kein Register-Verdikt; (3) SWIFF/WINDMI/MAGION bleiben als **kein Datensatz** benannt (Modell/gated), keine Zeile.

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

### Taucherstaffel 347 — Messungen und nächste Schritte (eigen)
- **Status:** descoped (überführt) | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-15) Die vier Taucher-Messungen (die fünf CDN-Quellen geklärt,
  `opentopography_coverage`-`terms` gesetzt, `catalog_mpcorb`/`cosmic_ro`/`heasarc_w3browse`
  descoped, Venus-Roh-Block, SABER-/TIMED-Bau) sind auf die Punkte „Venus Radar", „SABER" und
  „Fünf fehlende Fäden" übergegangen; kein eigener offener Gegenstand bleibt hier.
- **Blockade:** keine.
- **Braucht:** nichts an dieser Stelle.

### WDC-B/Obninsk — Erbe lebt: 32 → ~205 + lebender sep/-Baum (eigen)
- **Status:** eigen (Rest Mycelium-gebunden) | **Bindung:** eigen
- **Trigger:** der Registeratom / Myceliums Transport
- **Lage:** (gemessen 2026-10-15, weite WDC-B-Runde) Die Erbe-Rettung ist breiter als die 32:
  **205 ladbare Payload-Dateien** (1335 CDX-Zeilen; ~173 neu — `sep/GravConst` 51,
  `stp/geo_min.val` 44, `sep/data/gdata` 31, `sdata` 18, `stp/geomagni.ind/aa` 14,
  `stp/catal_pc` 8, `gps` 7). Die 32 Wayback-Blöcke stehen (`terms CC-BY-NC-4.0`).
  Der **lebende `sep/`-Baum** (`www.wdcb.ru/sep/`, offen, kein Key) trägt **keine Datei-Lizenz**
  — nur die Site-Policy: frei für Forschung/Lehre, nicht-kommerziell; kommerziell nur nach
  Kontakt; **Nennung Pflicht** (`http://www.wdcb.ru/sep/policy.html`). **Kein CC-Ersatz** — die
  Policy wird als eigene Zeile benannt. Schwester-Hosts live: `usd.`/`esdb.`/`rjes.`/`stp.wdcb.ru`.
- **Blockade:** die ~173 Blöcke + der lebende Baum brauchen Myceliums `url`/`origin`/`compiler`.
- **Braucht:** Mycelium registriert die ~173 Wayback-Payloads (`terms CC-BY-NC-4.0`) und den
  lebenden `sep/`-Baum (`terms` = die Site-Policy, nicht CC) + die Schwester-Hosts; die fünf
  wertvollsten Gruppen zuerst (`GravConst`, `geo_min.val`, `gdata`, `sep/strong`, `sep/catalogues`).

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

## An mycelium

Origin: mountain-349 (2026-10-15):

- `xfer:mountain-348-3` **IGETS `origin`** — der Block trägt heute `origin sftp://igetsftp.gfz.de`
  (die Rohdaten-Naht), die Projektseite `igets.gfz-potsdam.de/` ist nicht mehr die `origin`-Zeile.
- `xfer:mountain-348-6` **GEOFON-Wellenform** — `xfer:mycelium-350-1` ist geantwortet (Echo): das
  `field` steht.
- `xfer:mountain-349-1` **WDC-B-Erbe 32 → ~205 + lebender `sep/`-Baum** — Mountain liefert die
  Verdikt-Seite: die Wayback-Gruppen `terms CC-BY-NC-4.0` (die Bulletin-Dateien); der lebende
  `sep/`-Baum `terms wdcb-sep-policy http://www.wdcb.ru/sep/policy.html` (frei für Forschung/Lehre,
  nicht-kommerziell, Nennung; kommerziell nur nach Kontakt) — **kein CC-Ersatz**. Bitte
  `url`/`origin`/`compiler` je Block/Gruppe setzen + die Schwester-Hosts.

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
- **„Der WDC-B-Punkt ändert sich: es sind nicht mehr die 32 Zeilen allein, sondern ~205. Und die Lizenz differiert: Wayback-Erbe (die 32 + die neuen Gruppen) = CC-BY-NC-4.0 (die Bulletin-Dateien); der lebende sep/-Baum: keine Datei-Lizenz — nur die Site-Policy (frei für Forschung/Lehre, nicht-kommerziell, Nennung). Das gehört als terms-Zeile ehrlich benannt, nicht als CC-Ersatz."** | 2026-10-15 | (mountain 349 — die WDC-B-Ergänzung).
- **Der Auftrag `briefs-2026-10-15-mountain-mycelium.md` (§&nbsp;/mountain) — die sechs Register-Schreibakte (`xfer:mycelium-351-1…-6`), die zwei WDC-B-Zeilenklassen und die vier Korrekturen.** | 2026-10-15 | (mountain 350) — am Baum verifiziert: getragen von mountain-349 (`36b50dfca`); **nicht erneut ausgeführt**.
- **„kannst du bitte taucher mit harten bandagen und archive search all!!! auf — SWIFF · WINDMI · MAGION · MOSWOC-Kp · OVATION-AACGM — loslassen"** | 2026-10-15 | (mountain 351 — der harte Ketten-Quellen-Lauf; Messblatt `docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md`).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-351) hat den harten Ketten-Quellen-Lauf gefahren (fünf Taucher + vier
`bin/all_wide`-Läufe), die fünf Befunde am Baum nachgemessen (Register `--root phi` = 0/0)
und das Messblatt `docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md` geschrieben;
die vier Risse gegen river-214 benannt (MOSWOC restricted, OVATION eine Station, MAGION
Satelliten-Serie, WINDMI Dauerlauf) und den `all_wide`-Profil-Defekt gemessen. Kein Code
geändert (`cargo check` unberührt). Diese Fortschreibung (folge350 → `archiv/`).
