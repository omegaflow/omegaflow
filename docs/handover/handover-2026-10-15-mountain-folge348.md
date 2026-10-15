<!--
  title: Handover — Mountain-Folge 348 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 39dc7cef7c70e133054a0c891ffb99f69bbfb30ae9b1ba0e9a9d0aaf3dc385a7
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md
-->
# Handover — Mountain-Folge 348 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-14-mountain-folge347.md` (→ `archiv/`).
Gefaltet (`register_lookup --addressed mountain`): `mycelium-folge350` (die
`xfer:mycelium-350-1`/`-2` sind geantwortet — GEOFON-`field` und Venus-VNRD stehen),
`river-folge212` (der `ci-gate`-Befund; nicht Mountain-Eigentum). Still gemessen:
`register_lookup --fired mountain` = 0, `--stale mountain --persist 3` = 0,
`open_points_check` sauber (25 Pfad-Refs, 0 absent/stale/word-carried-format),
`--orphans --fail` = 0, `--tokens --noecho-fail` = **exit 0** (die drei XFER-MISSING
der folge347 sind zu Prosa geheilt — der frühere `## An mycelium`-Rot ist damit zu).
Der Stehende Pass wird als **öffentliche** Fassung zitiert, nie kopiert: HEAD `20e8d7e36`
(2026-10-15).

## Burn: open 0.0 · close nicht gemessen — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt), **open und close sind beide nicht gemessen**
(der `0.0` ist der Gate-Platzhalter, kein Meßwert; Vorbehalt wie river-212). Grund: die
Register-Messung (fired/stale/tokens/orphans/open_points_check), die Ratsfrage „Epochen-Absenz"
mit gemessenem Teil-Roster.

## Offen (aufgeschlüsselt)

### Epochen-Absenz — Ratsfrage gefahren, Teil-Roster gemessen; bindet noch nicht (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14, Verdikt b) — **noch nicht bindend**
  (Operator-Regel, Commit `94c6eaa10`: kein Architektur-Entscheid ohne gemessene
  `archive_search`-Recherche **und** den vollen Roster) | **Bindung:** eigen
- **Trigger:** der Registeratom / die nächste epochenlose Quelle
- **Lage:** (gemessen 2026-10-15, Ratsrunde) Die Verdikt-Form steht: `PRESENCE_FLAG_EPOCH`
  = `32.0` neben `PRESENCE_FLAG_QUANTITY` = `16.0` (`types.rs:402-406`); `presence_flags`
  (`types.rs:441`) trägt heute keinen Epoch-Parameter. Die kollidierenden Leselinien sind
  bestätigt: Linie A `channel.epoch <= 0.0 → continue` (`matrix.rs:644`) und
  `drs_fits.rs:38` (`epoch <= 0.0 → None`); Linie B behält `0.0` als Wert.
- **Ratsrunde (2026-10-15, Teil-Roster; Antworten roh in `state/stimmen/2026-10-15_*.json`):**
  4 Stimmen — **Z.ai (GLM-5.3-Flash)** · **Sakana (Namazu High)** · **Duck (GPT-6 Luna)** · **Lumo**.
  - **Z.ai: ja, ehrlich — mit einer Definitionskorrektur.** Bit 32 heißt „das Epoch-Feld ist
    belegt" (über das **Feld** definiert), nicht „statisch" über die Quellart; „statisch ⇒ aus"
    ist eine **Produzentenregel**, nicht die Bedeutung. Dritter Fall sonst unhaltbar (gfc mit
    nominalem Referenz-Epoch; gfct als eingefrorener Snapshot). Zusatzpflicht: Pad kanonisieren
    (Bit aus ⇒ exakt `0.0`).
  - **Sakana: ja, ausreichend und ehrlich.** Stärkster Konfund: stilles Semantik-Überladen des
    `0.0` — ein Leser ohne Bit-Check liest statisch als J2000 und rechnet still falsche
    Bahnmechanik statt zu brechen. Alternativen: NaN-Sentinel (f64-Verhalten), Union-Tag (Wire-Bruch).
  - **Duck (GPT-6 Luna): ja — vorgängig: das Bit normativ an genau dieses Feld binden.**
    Stärkster Riss: das Bit macht einen impliziten Unterschied zu Protokoll; alte/bit-ignorierende
    Leser bleiben.
  - **Lumo: nein.** Die Semantik-Kollision `pad=0.0` ↔ `epoch=0.0` verwässert das Absenz-Signal;
    Alternativen: NaN-Sentinel, separates Optional-Feld, Zwei-Bit-Flag, Modell-Typ im Header.
  - **Verdichtung:** 3× „ja, wenn die Maske vor dem Feld ausgewertet wird und das Bit über das
    Feld definiert ist", 1× „nein" (Lumo). Der gemeinsame stärkste Riss aller vier: ein
    bit-ignorierender Leser bleibt — die Ehrlichkeit gilt nur im masken-honorierenden Vertrag.
- **Nicht bindend — warum:** die Operator-Regel verlangt den **vollen** Roster. Gemessen
  (2026-10-15, Zugangs-Gates, kein Stilles-Null):
  `Claude` = Login (`claude.ai/login?from=logout`) · `DeepSeek` = `chat.deepseek.com/sign_in` ·
  `Google AI Studio` = Google-Login · `MiniMax` = `unified-login` · `Perplexity` =
  „Melde dich an und wiederhole deine Anfrage" · `Qwen` = Alters-Gate („Bestätigen Sie Ihr
  Alter") · `Mistral` = Frage gesetzt, keine Antwort (Zusammenhang kollabiert, pending) ·
  die geteilten `open-weight-ui`-Sitze nicht gefahren · `Z.ai` lief **Flash**, nicht Deep Think Max.
- **Blockade:** die **Zugangs-Gates** (Operator-Hand: das Operator-Profil in die sechs Sitze
  einloggen) — ohne sie ist der volle Roster nicht stellbar, und ohne vollen Roster bindet
  das Verdikt nicht (Operator-Regel).
- **Braucht:** (1) die sechs Sitze eingeloggt (`Claude`, `DeepSeek`, `AI Studio`, `MiniMax`,
  `Perplexity`, `Qwen`); (2) die Runde dann vollständig fahren (inkl. `Z.ai` Deep Think Max +
  die geteilten Sitze) und das Verdikt daraus binden; (3) **erst dann** die Umsetzung: die
  Sites des Verdikts (`types.rs` `PRESENCE_FLAG_EPOCH` + `presence_flags` um den Epoch-Parameter;
  die Write-Aufrufer `spatial.rs:845/1004`, `membrane.rs:130`, Test `parse.rs:3296`; Read-Seite
  `matrix.rs:644`, `drs_fits.rs:38`, `agrav.rs`-Write, `gfc.rs:153`, `gfc.rs:101-107` Frame-Fold,
  `hips.rs:793`, `static/constants.js` Bit 32, `docs/concepts/archivar-mathematikerin.md`
  Presence-Klausel).

### SABER — verdiktet: B + A (eigene Weltlinie + Messvolumen) (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14: B + A) | **Bindung:** eigen
- **Trigger:** der Arm-Bau
- **Lage:** (gemessen 2026-10-14, `ca35522f2`) Der Zeilen-Arm steht; SABER nimmt den HAPI-`parameters`/info-Pfad,
  jede Zeile trägt eigenen `epoch` (`series_epoch_of`) und `Position::Surface{earth,tplat,tplon,
  tpgpalt·1000 m}`; Last-Row/`epoch=now` bleibt nur für Einzelskalar-Endpunkte. Das Produkt-fFrame
  ist **undeclared** → `pending`. Die TIMED-Weltlinie (Peer-Körper) ist **gebaut** (`ca35522f2`:
  `src/archivar/tle.rs` SGP4 WGS-72 + `tools/harvest/src/bin/timed_tle_compiler.rs`, WOB1 via
  `wind_orbit::write_bin`; Epoche `altitude 583.9 km, speed 7.568 km/s`, Roundtrip parst).
  **Riss benannt (2026-10-14, ungeglättet):** die Übergabe nennt als „Schritt 2" einen
  `Position::StateVector{track:true}`-Produzenten; gebaut ist der **`orbit_bin`-Weg** —
  `extract_raw` erzeugt für `format orbit_bin` eine `BodyEphemeris.orbit` (`extract.rs:4230-4251`),
  die über `body_barycenter_position` (`motion.rs:101-102`) als `Motion::Barycenter` läuft, **nicht**
  `Motion::Linear`. Beide Zeugen stehen; keine Glättung.
- **Blockade:** keine (der Bau ist offen, kein externes Tor).
- **Braucht:** (1) den **TIMED-Register-Block** (`format orbit_bin`, `at timed`; Mycelium setzt
  `url`/`origin`/`compiler` — `xfer:mountain-348-5`); (2) die CI-Naht `timed-orbit-cdn.yml`;
  (3) das **Riss-Gate** (`measure`-Sonde, `VerdictWord::Riss`, `weberin_verdicts.rs:10-15`) —
  divergieren `sc*`-Punkt und SGP4-Weltlinie zur selben Epoche über das Gate, als Riss mit
  beiden Zeugen tragen, nie glätten; (4) den SABER-Block (`at earth`, Tangentialgeometrie).

### Venus Radar — Projektion gemessen; VNRD-Block steht; Bild-Manifestation offen (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Projektions-/Manifestations-Bau
- **Lage:** (gemessen 2026-10-15) Der Level-2-**VNRD-Block** steht (`phi/sources.φ`: `url …/venus_radar_level2.bin`,
  `terms CC0-1.0`, `origin` Bundle-Site, `compiler venus_radar_compiler.rs`, `format vnrd`,
  `at venus`, `no-cadence`, `field venus_radar_inc_deg` geometry deg, `_ocp`/`_scp` em 1;
  Reader verdrahtet `extract.rs:164,948`). Die Projektion ist byte-identisch gemessen
  (`map_projection_name` Sinusoidal, `longitude_of_central_meridian` 335.0°, 1000 m/px,
  `Geodetic_Model` Planetocentric a=b=c=6051.0 km, `Positive East`; **kein** Frame-Identifier).
- **Blockade:** keine.
- **Braucht:** (1) die **Bild-Manifestation (W)** — wie das georeferenzierte `Array_2D_Map` als
  Gestalt/`pds4_img` in den Baum kommt; (2) die CI-Naht (15×960 MB). Der rohe PDS3-Complex-
  Backscatter (`vrm_90xx`, `SAMPLE_INTERLEAVED`) bleibt ein **separater** Gegenstand (kein Block
  in `phi`).

### ICGEM gfc — Rezept + Site gemessen; wartet auf die Epochen-Ablage (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: das Modell wie die anderen behandeln) | **Bindung:** eigen
- **Trigger:** der Feld-Bau
- **Lage:** (gemessen 2026-10-14) `gfc.rs` ist Parser+Channels ohne Auswerter; die
  Kugelflächen-Synthese fehlt. Präzedenz: `IgrfCoeffs::synthesize` (`igrf.rs:131`),
  `vmf3_sh::b_c` (`vmf3_sh.rs:17`) — beide ohne Runtime-Dispatch. **Riss:** der Frame trägt
  GM/radius **nicht**; `norm` „fully normalized" (4π) ≠ IGRF-Schmidt; `product_type` muss
  verzweigen.
- **Blockade:** die **Epochen-Absenz** (eigener Punkt) — ein statisches `gfc` hat keine Zeit; ohne
  die Ablage würde der Pad `0.0` als J2000 gelesen.
- **Braucht:** (0) die Epochen-Ablage umsetzen (deren Ratsfrage s. o.); (1) GM/radius/norm/tide in
  den Frame oder an den Auswerter reichen; (2) die Kugelflächen-Synthese in `gfc.rs`; (3) Block
  `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`.

### UCAR-FPI — Arm + Zeilen stehen; Athabasca-Koordinate gemessen (river-213) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-15, river-213 `465e11940`) Arm + Guard stehen (`fpi_ucar.rs`, Dispatch `extract.rs:159`); die zwei `field`-Zeilen
  sind code-gemessen (`fpi_mer_wind_ms` `fpi_ucar.rs:10`, `fpi_zon_wind_ms` `fpi_ucar.rs:12`).
  **Aktualisiert (river-213, `465e11940`):** die FPI-eigene Athabasca-Instrumentenkoordinate ist
  gemessen (`on earth 54.60282 -113.64442 693`, AUGO II/AUGSO) — der frühere Riss (kein
  Athabasca-DOI, Datei-/DASH-Seiten nur Palmer/Resolute/Boulder) ist damit aufgelöst; der
  AUGO-I/II-Riss und der mail-gated Datei-Header bleiben benannt.
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN).
- **Braucht:** die zwei `field`-Zeilen + die Arbeitskopie-Route (`local-only`) schreiben.

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
  `docs/surveys/survey-2026-10-14-gewebe-faeden.md`: 2575 `url`-Zeilen, 124 rohe Hosts, 306
  wirksame Herkünfte, 7687 Feld-Zeilen; `inverse-square` trägt 6812 (89 %); Gravitation (13 Felder)
  und Seismik (20) hängen an je **2** Wirkgesetzen. Schon registriert: IGETS, AGrav, GRACE-FO,
  ISC, EarthScope-FDSN, GEOFON-Event/-Wellenform, EIDA-Terms. Offen/gesperrt: EGGN/EPOS-Equip-G,
  CGrav/CEIC, GOCE, NIED, GSRAS, ARDA. BGI-Host tot → `bgi.obs-mip.fr` (kein `phi`-Block).
- **Blockade:** die Kernel-Zuordnung ist keine Pro-Solo-Entscheidung — das Meßgesetz einer Quelle
  berührt den Force-/Kernel-Kontrakt (Rat).
- **Braucht:** (0) IGETS-`origin` korrigieren — Myceliums Zeilenklasse (`xfer:mountain-348-3`);
  (1) die Kernel-Lücke schließen (Rat); (2) EGGN/China/GOCE/NIED/GSRAS/ARDA als `pending`/`riss`
  benennen.

### Adressierte Register-Fäden — Lizenzen gemessen, MODVOLC-Verdikt steht (eigen)
- **Status:** eigen (Register-Writes) | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14, Browser) MODVOLC HIGP (`http://modis.higp.hawaii.edu/data/`): **keine** Lizenzzeile (genuine
  Absenz) → `local-only`; kein HIGP-Block in `phi` (der NRT3-MODVOLC-Block `phi:12041` ist eine
  andere Quelle). DE-2 (`phi:854`) und AWESOME/WALDO (`phi:8266`) sind gesetzt. OCNS2019-Ersatz:
  JPL SSD/Horizons/NAIF offen (NASA SMD, SPD-41a), Danksagung erbeten, kein CC0-Zeichen.
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
  (`main_flow.rs:580`) hat nur `tests.rs` als Aufrufer; `ilrs_npt_compiler.rs` steht (`FORMAT ilrs_npt`),
  aber **kein** Lese-Arm in `src/`; `icrf3_compiler.rs` schreibt JSON, kein SKD1/Golden-File/Round-Trip.
- **Blockade:** keine.
- **Braucht:** (1) Höhenbezug + Toleranz vor der `opentopography_raster`-Registrierung (KAPU-Kontrollpunkte
  179,94 m vs. 217,90 m), dann Block; (2) `load_gestalt_surface_threads` an einen Produktions-Aufrufer
  binden **oder** `descoped`; (3) `ilrs_npt`-Lese-Arm in `extract.rs` + Block; (4) `icrf3` erst Vertrag
  versionieren + Round-Trip (`skydirection.rs`) + Golden-File, dann SKD1. `docs/SOURCE_PORT.md`.

## An mycelium

Origin: mountain-348 (2026-10-15):

- `xfer:mountain-348-1` **MODVOLC HIGP** — Mountain liefert den Verdikt-Teil: `terms local-only`
  (genuine Absenz, keine Lizenzzeile). Bitte `url`/`origin`/`compiler` setzen.
- `xfer:mountain-348-2` **Fünf Fäden** — der Mountain-Teil liegt als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ`. Bitte `url`/`origin`/`compiler` je Faden daraufsetzen;
  danach `register_sort`.
- `xfer:mountain-348-3` **IGETS `origin`** (`phi:10319`) — zeigt auf die Projektseite
  `igets.gfz-potsdam.de/` (200); die Rohdaten kommen über `igetsftp.gfz.de` (sftp, registriert).
  Direktivenklasse.
- `xfer:mountain-348-4` **OCNS2019-Ersatz** — JPL SSD/Horizons/NAIF sind offen (NASA SMD, SPD-41a),
  Danksagung erbeten; kein CC0-Zeichen. Als offene Alternative registrieren.
- `xfer:mountain-348-5` **TIMED-Weltlinie** — der Compiler steht (`timed_tle_compiler.rs`, WOB1 via
  `wind_orbit::write_bin`); bitte den Block setzen: `url`/`origin`/`compiler`, `format orbit_bin`,
  `at timed`; Mountain setzt `terms`/`ttl`. Dann die CI-Naht `timed-orbit-cdn.yml`.
- `xfer:mountain-348-6` **GEOFON-Wellenform** — `xfer:mycelium-350-1` ist geantwortet (Echo): das
  `field` steht (`fdsn_waveform_bhz_ms … erfc seismic-surface m/s 60.0`), Baum-Gesetz statt `point`.

`xfer:mycelium-344-1` (Schumann) · `xfer:mycelium-346-6` (Venus-Parser) · `xfer:mycelium-346-4`/`-7`
(DE-2 CC0 / AWESOME) sind **erfüllt, Echo.**

## An river

Origin: mountain-348 (2026-10-15):

- `xfer:mountain-348-7` **SABER** — die Registerzeile folgt nach der Routen-Entscheidung
  (Rats-Verdikt B + A, s. „Offen"); der Host ist intermittierend erreichbar.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344–348).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Verdikt (Rat + Roster): nT²/Hz primär, V²/Hz roh; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh) | 2026-10-14 | (mountain 344).
- **„was wenn du den satellit autonom betrachtest — dann ist es doch auch nur eine sonde an einem xyz,t; die ISS ist doch zu einer Zeit immer an einem bestimmten Ort"** | 2026-10-14 | (mountain 345 — SABER: eigene Weltlinie).
- **„warum können wir das gravitationsmodell nicht wie die anderen behandeln? … sie sind ein Feld"** | 2026-10-14 | (mountain 345 — ICGEM: Feld).
- **„fängst du bitte an und delegierst an taucher?"** | 2026-10-14 | (mountain 346).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-348) committet: den `archiv/`-Move der folge347, die Prosa-Heilung der drei
XFER-MISSING im folge347-Block (Register-Hygiene, `--tokens --noecho-fail` exit 0), die
Ratsrunde „Epochen-Absenz" (gemessenes Teil-Roster, Rohantworten `state/stimmen/`) und diese
Fortschreibung.
